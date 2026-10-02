//! `evaluate`: the verdict a pull request earns under a policy, as the dict
//! Python returns, key for key and in Python's key order.
//!
//! The verdict is the first requirement not met, in this order: the bots
//! have reported, the author has attested, a slot is free, the build is
//! green, the approvals are in. Every requirement is still worked out past
//! the first unmet one, so the checklist shows the whole road; fields that
//! are set only past a gate are still set only past it.
//!
//! The status is a required check: it passes only when the policy is
//! satisfied. Anything still waiting is `pending`, and only a problem
//! somebody must fix in the configuration or the evidence is `error`.

use super::bots::{schedule, skipped_by, Facts};
use super::patterns::{final_phase, ATTESTATION, HEAD_SHA};
use super::prints::carried_heads;
use super::receipts::{rabbit_receipt, receipt_instant, receipt_print};
use super::validate::{governs, required_bots, validate_policy};
use super::values::{
    dict, earliest, latest, lower, re_text, s, string, strings, strs, time, upper, upper_in,
};
use super::{holders, machine_author, may_object, BOTS, WRITE};
use crate::pycompat::object::{
    get, get_or, getitem, iterate, no_attribute, or, py_str, str_method, EMPTY_DICT, EMPTY_STR,
};
use crate::pycompat::ops::{
    py_compare, py_eq, py_eq_str, py_hashable, py_in_str_set, py_same_element, py_sort_by, Compare,
};
use crate::pycompat::text::{py_lower, py_strip};
use crate::pycompat::{PyDict, PyErr, PyList, PyValue};
use indexmap::IndexMap;
use std::borrow::Cow;
use std::cmp::Ordering;
use std::collections::BTreeSet;

/// Where `evaluate` stops: the state, the status, and the reasons that
/// lead the blockers.
struct Stop {
    state: &'static str,
    status: &'static str,
    reasons: Vec<String>,
}

impl Stop {
    fn pending(state: &'static str, reasons: Vec<String>) -> Self {
        Stop {
            state,
            status: "pending",
            reasons,
        }
    }

    fn error(reason: impl Into<String>) -> Self {
        Stop {
            state: "configuration-error",
            status: "error",
            reasons: vec![reason.into()],
        }
    }
}

/// An area a change touched, read from a policy `validate_policy` accepted.
struct Area<'p> {
    id: &'p str,
    paths: Vec<&'p str>,
    owners: Vec<&'p str>,
    reviewers: Vec<&'p str>,
    unresolved: bool,
}

impl<'p> Area<'p> {
    /// One of `policy['areas']`.
    fn read(area: &'p PyValue) -> Result<Self, PyErr> {
        Ok(Area {
            id: string(getitem(area, "id")?)?,
            paths: strings(getitem(area, "paths")?)?,
            owners: strings(getitem(area, "owners")?)?,
            reviewers: strings(getitem(area, "reviewers")?)?,
            unresolved: get(area, "unresolved")?.truthy(),
        })
    }

    /// `dict(policy['fallback'], id='fallback')`: it covers whatever no
    /// area's prefix does, and has no paths and no unresolved identities.
    fn fallback(fallback: &'p PyValue) -> Result<Self, PyErr> {
        Ok(Area {
            id: "fallback",
            paths: Vec::new(),
            owners: strings(getitem(fallback, "owners")?)?,
            reviewers: strings(getitem(fallback, "reviewers")?)?,
            unresolved: get(fallback, "unresolved")?.truthy(),
        })
    }

    /// `area['owners'] + area['reviewers']`.
    fn people(&self) -> impl Iterator<Item = &'p str> + '_ {
        self.owners.iter().chain(self.reviewers.iter()).copied()
    }
}

/// The first unmet requirement: its state and reasons.
type Gate = Option<(&'static str, Vec<String>)>;

fn gate(first: &mut Gate, state: &'static str, reasons: Vec<String>) {
    if first.is_none() {
        *first = Some((state, reasons));
    }
}

/// `evaluate(policy, pr, admitted_at, now, telemetry_states)`: the verdict,
/// as Python's dict. `telemetry_states` maps a bot to what its status page
/// said about this head, or is `None`.
///
/// A `ValueError`, `TypeError` or `KeyError` raised on the way is the
/// verdict `configuration-error`, its text the first blocker. Any other
/// exception is returned as the error, as Python's `evaluate` raises it.
pub fn evaluate(
    policy: &PyValue,
    pr: &PyValue,
    admitted_at: &PyValue,
    now: &PyValue,
    telemetry_states: &PyValue,
) -> Result<PyDict, PyErr> {
    let mut result = PyDict::new();
    for key in ["number", "head", "author", "title", "url"] {
        result.insert(key.into(), get(pr, key)?.clone());
    }
    let head = get(pr, "head")?;
    let reviewed_heads = if head.truthy() {
        PyValue::List(PyList::from(vec![head.clone()]))
    } else {
        PyValue::List(PyList::new())
    };
    let reviewed_since = get(pr, "head_seen_at")?.clone();
    let receipts = or(
        get(or(get(pr, "controller_diff")?, &EMPTY_DICT), "receipts")?,
        &EMPTY_DICT,
    )
    .clone();
    let empty = || PyValue::List(PyList::new());
    for (key, value) in [
        ("state", s("configuration-error")),
        ("status", s("error")),
        ("blockers", empty()),
        ("reviewers", empty()),
        ("objectors", empty()),
        ("areas", empty()),
        ("ready_since", PyValue::None),
        ("admitted_at", admitted_at.clone()),
        ("bot_completed_at", PyValue::None),
        ("self_reviewed_at", PyValue::None),
        ("nudge", empty()),
        ("waived", empty()),
        ("approvals", empty()),
        ("objections", empty()),
        ("checklist", empty()),
        ("reviewed_heads", reviewed_heads),
        ("reviewed_since", reviewed_since),
        ("receipts", receipts),
    ] {
        result.insert(key.into(), value);
    }
    // Before anything can gate: a verdict that stops early must still
    // record what carries this diff, or one configuration error cuts the
    // chain and the pull request starts asking for its reviews again.
    match carried_heads(pr) {
        Ok((heads, since)) => {
            result.insert("reviewed_heads".into(), PyValue::List(heads.into()));
            result.insert("reviewed_since".into(), since);
        }
        Err(error) if error.is_value_type_or_key() => {}
        Err(error) => return Err(error),
    }

    // A waiver is reported wherever the pull request ends up, not only
    // where it was granted.
    let mut notes = Vec::new();
    let stop = match decide(
        policy,
        pr,
        admitted_at,
        now,
        telemetry_states,
        &mut result,
        &mut notes,
    ) {
        Ok(stop) => stop,
        Err(error) if error.is_value_type_or_key() => Stop::error(error.to_string()),
        Err(error) => return Err(error),
    };
    let blockers = stop.reasons.into_iter().chain(notes);
    result.insert("state".into(), s(stop.state));
    result.insert("status".into(), s(stop.status));
    result.insert("blockers".into(), strs(blockers));
    Ok(result)
}

/// `result[key].append(value)`, for a list `evaluate` created.
fn append(result: &mut PyDict, key: &str, value: PyValue) {
    if let Some(PyValue::List(items)) = result.get_mut(key) {
        items.push(value);
    }
}

/// The body of `evaluate`'s `try`: everything that can raise into a
/// configuration error. `result` and `notes` keep what was set before an
/// exception, as Python's do.
fn decide(
    policy: &PyValue,
    pr: &PyValue,
    admitted_at: &PyValue,
    now: &PyValue,
    telemetry_states: &PyValue,
    result: &mut PyDict,
    notes: &mut Vec<String>,
) -> Result<Stop, PyErr> {
    validate_policy(policy)?;
    time(now)?;
    const REQUIRED: [&str; 15] = [
        "number",
        "author",
        "head",
        "base",
        "base_sha",
        "created_at",
        "draft",
        "state",
        "files",
        "reviews",
        "comments",
        "threads",
        "permissions",
        "complete",
        "build",
    ];
    // `pr.get` worked above, so it is a dict.
    let has = |key: &str| matches!(pr, PyValue::Dict(entries) if entries.contains_key(key));
    if !REQUIRED.iter().all(|key| has(key)) || !getitem(pr, "complete")?.truthy() {
        return Ok(Stop::error("Incomplete GitHub snapshot"));
    }
    let head = getitem(pr, "head")?;
    if !HEAD_SHA.is_match(re_text(head)?) {
        return Ok(Stop::error("Invalid head SHA"));
    }
    if !py_eq_str(getitem(pr, "state")?, "open") || !governs(policy, getitem(pr, "base")?)? {
        return Ok(Stop::pending(
            "configuration-error",
            vec!["PR is outside the active policy scope".into()],
        ));
    }
    if getitem(pr, "draft")?.truthy() {
        return Ok(Stop::pending(
            "draft",
            vec!["Draft PR does not occupy a review slot".into()],
        ));
    }
    if admitted_at.truthy() {
        time(admitted_at)?;
    }
    let files = getitem(pr, "files")?;
    if !files.truthy() {
        return Ok(Stop::error("No changed-file evidence"));
    }

    let areas = match getitem(policy, "areas")? {
        PyValue::List(items) => items
            .iter()
            .map(Area::read)
            .collect::<Result<Vec<_>, PyErr>>()?,
        other => return Err(no_attribute(other, "get")),
    };
    let fallback = Area::fallback(getitem(policy, "fallback")?)?;
    // Areas in the order files first touch them: every surface lists them
    // so, and an order that changed between runs rewrote what was posted.
    let mut touched: IndexMap<&str, &Area<'_>> = IndexMap::new();
    let mut files_by_area: IndexMap<&str, BTreeSet<String>> = IndexMap::new();
    for file in iterate(files)? {
        let filename = getitem(&file, "filename")?;
        let previous = get_or(&file, "previous_filename", filename)?;
        // `dict.fromkeys((filename, previous))`: each once, in that order.
        py_hashable(filename)?;
        py_hashable(previous)?;
        let mut paths = vec![filename];
        if !py_eq(filename, previous) {
            paths.push(previous);
        }
        for path in paths {
            let path = match path {
                PyValue::Str(path)
                    if !path.starts_with('/')
                        && !path
                            .split('/')
                            .any(|x| x == "." || x == ".." || x.is_empty()) =>
                {
                    path
                }
                _ => return Err(PyErr::value("Invalid changed path")),
            };
            let area = areas
                .iter()
                .find(|area| area.paths.iter().any(|prefix| path.starts_with(prefix)))
                .unwrap_or(&fallback);
            touched.insert(area.id, area);
            files_by_area
                .entry(area.id)
                .or_default()
                .insert(path.clone());
        }
    }
    let mut area_ids: Vec<&str> = touched.keys().copied().collect();
    area_ids.sort();
    result.insert("areas".into(), strs(area_ids));

    let mut permissions = PyDict::new();
    match getitem(pr, "permissions")? {
        PyValue::Dict(entries) => {
            for (login, level) in entries.iter() {
                permissions.insert(py_lower(login), level.clone());
            }
        }
        other => return Err(no_attribute(other, "items")),
    }
    let level = |login: &str| permissions.get(login).unwrap_or(&PyValue::None);
    let mut people: IndexMap<String, &str> = IndexMap::new();
    for area in touched.values() {
        for person in area.people() {
            people.insert(py_lower(person), person);
        }
    }
    for area in touched.values() {
        if area.unresolved {
            return Ok(Stop::error(format!("Unresolved identities in {}", area.id)));
        }
    }
    let mut unverified: Vec<&String> = people
        .keys()
        .filter(|login| matches!(level(login), PyValue::None))
        .collect();
    unverified.sort();
    if !unverified.is_empty() {
        // Not the same as lacking access: the answer never arrived.
        let names: Vec<&str> = unverified
            .iter()
            .map(|login| people.get(*login).copied().unwrap_or(login.as_str()))
            .collect();
        return Ok(Stop::error(format!(
            "Cannot verify write access for {}",
            names.join(", ")
        )));
    }
    for login in people.keys() {
        if !py_in_str_set(level(login), &WRITE)? {
            return Ok(Stop::error(
                "An assigned owner/reviewer lacks verified write access",
            ));
        }
    }

    // From here on every requirement is computed, whether or not an
    // earlier one is met.
    let mut first: Gate = None;
    let required = required_bots(policy)?;
    // A push that leaves this pull request's own diff untouched carries
    // the review of the commit before it; anything that changes the diff
    // starts over.
    let (reviewed, seen_first) = carried_heads(pr)?;
    let heads: BTreeSet<String> = reviewed.iter().map(lower).collect::<Result<_, _>>()?;
    let reviewed_value = PyValue::List(reviewed.clone().into());
    result.insert("reviewed_heads".into(), reviewed_value.clone());
    result.insert("reviewed_since".into(), seen_first.clone());
    let commit_in_heads = |review: &PyValue, via_get: bool| -> Result<bool, PyErr> {
        let commit = if via_get {
            get(review, "commit_id")?
        } else {
            getitem(review, "commit_id")?
        };
        Ok(heads.contains(&lower(or(commit, &EMPTY_STR))?))
    };

    let reviews = iterate(getitem(pr, "reviews")?)?;
    let latest_reviews = latest_reviews(&reviews)?;
    let mut bot_blocks = Vec::new();
    for (user, review) in &latest_reviews {
        if BOTS.contains(&user.as_str())
            && upper_in(getitem(review, "state")?, &["CHANGES_REQUESTED"])?
        {
            bot_blocks.push(*review);
        }
    }
    // Only a thread holding a blocker is something the bot is owed an
    // answer on; it stands for the bot having spoken, whatever commit it
    // was written on. A suggestion holds nothing.
    let threads = iterate(getitem(pr, "threads")?)?;
    let mut bot_threads = Vec::new();
    for thread in &threads {
        if !getitem(thread, "is_resolved")?.truthy()
            && BOTS.contains(&lower(getitem(thread, "author")?)?.as_str())
            && super::finding_blocks(thread)?
        {
            bot_threads.push(thread.as_ref());
        }
    }
    // The marker naming any of the heads, built the first time a review
    // could carry it; a set iterates its heads sorted, as Python sorts them.
    let sorted_heads: Vec<String> = heads.iter().cloned().collect();
    let mut final_marker = None;
    let mut pasta: Vec<PyValue> = Vec::new();
    let mut rabbit: Vec<PyValue> = Vec::new();
    for review in &reviews {
        let user = lower(getitem(review, "user")?)?;
        let state = upper(getitem(review, "state")?)?;
        if !commit_in_heads(review, false)? {
            continue;
        }
        if user == "thepastaclaw" && matches!(state.as_deref(), Some("APPROVED" | "COMMENTED")) {
            let body = re_text(getitem(review, "body")?)?;
            if final_marker.is_none() {
                final_marker = Some(final_phase(&sorted_heads)?);
            }
            if final_marker
                .iter()
                .flatten()
                .any(|marker| marker.is_match(body))
            {
                pasta.push(getitem(review, "submitted_at")?.clone());
            }
        }
        if (user == "coderabbitai" || user == "coderabbitai[bot]")
            && state.as_deref() == Some("APPROVED")
        {
            rabbit.push(getitem(review, "submitted_at")?.clone());
        }
    }
    let comments = iterate(getitem(pr, "comments")?)?;
    let mut seen_said = PyDict::new();
    for comment in &comments {
        if !matches!(
            lower(getitem(comment, "user")?)?.as_str(),
            "coderabbitai" | "coderabbitai[bot]"
        ) {
            continue;
        }
        let mut receipt = false;
        for h in &reviewed {
            if rabbit_receipt(getitem(comment, "body")?, h)? {
                receipt = true;
                break;
            }
        }
        if receipt {
            let instant = receipt_instant(pr, comment)?;
            rabbit.push(instant.clone());
            if let Some(said) = receipt_print(comment)? {
                seen_said.insert(said, instant);
            }
        }
    }
    result.insert("receipts".into(), PyValue::Dict(seen_said));
    // A bot that requested changes on an earlier head has not reported on
    // this one: that is the shape a nudge and a waiver exist for. An
    // objection raised against the current head is a report, and blocks.
    let mut current_blocks = Vec::new();
    for review in bot_blocks {
        if commit_in_heads(review, true)? {
            current_blocks.push(review);
        }
    }
    let bot_blocks = current_blocks;
    let receipts_of = |bot: &str| -> &[PyValue] {
        match bot {
            "thepastaclaw" => &pasta,
            "coderabbitai" => &rabbit,
            _ => &[],
        }
    };
    // A bot that objected to this head, or left a thread open, has
    // reported. It is not missing, so nothing waives it.
    let mut objecting = BTreeSet::new();
    for (user, review) in &latest_reviews {
        if BOTS.contains(&user.as_str())
            && commit_in_heads(review, true)?
            && upper_in(getitem(review, "state")?, &["CHANGES_REQUESTED"])?
        {
            objecting.insert(without_bot_suffix(user));
        }
    }
    let mut heard = objecting.clone();
    for thread in &bot_threads {
        heard.insert(without_bot_suffix(&lower(getitem(thread, "author")?)?));
    }
    let missing: Vec<&str> = required
        .iter()
        .map(String::as_str)
        .filter(|bot| receipts_of(bot).is_empty() && !heard.contains(*bot))
        .collect();
    let mut waived: IndexMap<&str, PyValue> = IndexMap::new();
    let mut why: IndexMap<&str, &'static str> = IndexMap::new();
    let mut reasons: Vec<String> = Vec::new();
    let outstanding = !bot_blocks.is_empty() || !bot_threads.is_empty();
    let skip = skipped_by(getitem(pr, "comments")?, &permissions, &seen_first)?;
    for bot in &missing {
        let telemetry = get(or(telemetry_states, &EMPTY_DICT), bot)?;
        let facts = Facts {
            pr,
            overrides: &[
                ("head_seen_at", &seen_first),
                ("reviewed_heads", &reviewed_value),
            ],
        };
        let plan = schedule(policy, &facts, bot, now, telemetry)?;
        if let Some(skip) = &skip {
            // A human decided the bots are not coming: a waiver with a name
            // on it. One that had already taken effect keeps its earlier
            // instant, or a late skip would read as the bots speaking again.
            let waived_at = plan.waived_at.filter(|at| !at.is_empty()).map(s);
            let instants = std::iter::once(&skip.at).chain(waived_at.as_ref());
            if let Some(instant) = earliest(instants)? {
                waived.insert(bot, instant.clone());
            }
            result.insert("skipped_by".into(), skip.user.clone());
            continue;
        }
        // Asking for a review while the pull request owes the bots an
        // answer anyway would spend someone else's capacity on nothing.
        if plan.nudge && !outstanding {
            append(result, "nudge", s(*bot));
        }
        match plan.waived_at {
            Some(at) if !at.is_empty() => {
                waived.insert(bot, s(at));
                why.insert(bot, plan.waived_reason.unwrap_or_default());
            }
            _ => reasons.push(format!("{bot} has not reported for the current head")),
        }
    }
    let mut waived_bots: Vec<&str> = waived.keys().copied().collect();
    waived_bots.sort();
    result.insert("waived".into(), strs(waived_bots.iter().copied()));
    for bot in &waived_bots {
        notes.push(match &skip {
            Some(skip) => format!(
                "Proceeded without {bot}: skipped by @{}",
                py_str(&skip.user)
            ),
            None if why.get(bot) == Some(&"rate-limit") => {
                format!("Proceeded without {bot}: it reported a rate limit and did not return")
            }
            None => format!("Proceeded without {bot}: no review within the configured window"),
        });
    }
    // Everything about each bot, not the first thing: a bot can have
    // reported and still hold the box open with an objection or a thread.
    let mut threads_by: IndexMap<String, usize> = IndexMap::new();
    for thread in &bot_threads {
        *threads_by
            .entry(without_bot_suffix(&lower(getitem(thread, "author")?)?))
            .or_default() += 1;
    }
    let mut named_bots: BTreeSet<String> = required.clone();
    named_bots.extend(objecting.iter().cloned());
    named_bots.extend(threads_by.keys().cloned());
    let mut bot_lines = Vec::new();
    for bot in &named_bots {
        let mut parts: Vec<String> = Vec::new();
        if waived.contains_key(bot.as_str()) {
            parts.push(match &skip {
                Some(skip) => format!("skipped by {}", py_str(&skip.user)),
                None if why.get(bot.as_str()) == Some(&"rate-limit") => {
                    "skipped after its own rate limit".into()
                }
                None => "skipped after the window".into(),
            });
        } else if !receipts_of(bot).is_empty() {
            parts.push("✓".into());
        } else if !objecting.contains(bot) && !threads_by.contains_key(bot) {
            parts.push("not yet".into());
        }
        if objecting.contains(bot) {
            parts.push("requested changes — dismiss the review or push a fix".into());
        }
        if let Some(&n) = threads_by.get(bot) {
            let (plural, pronoun) = if n > 1 { ("s", "them") } else { ("", "it") };
            parts.push(format!("{n} thread{plural} unresolved — resolve {pronoun}"));
        }
        bot_lines.push(format!("{bot} {}", parts.join(", ")));
    }
    // Waiting for the bots means a bot has not reported. Once it has, what
    // it said is the author's to answer.
    let mut findings = Vec::new();
    for review in &bot_blocks {
        let who = without_bot_suffix(&lower(getitem(review, "user")?)?);
        findings.push(format!(
            "{who} requested changes on this head; dismiss the review or push a fix"
        ));
    }
    let mut threaded = BTreeSet::new();
    for thread in &bot_threads {
        threaded.insert(without_bot_suffix(&lower(getitem(thread, "author")?)?));
    }
    for bot in threaded {
        findings.push(format!(
            "{bot} left review threads unresolved; resolve them"
        ));
    }
    if !reasons.is_empty() {
        gate(
            &mut first,
            "waiting-bots",
            reasons.into_iter().chain(findings).collect(),
        );
    } else if !findings.is_empty() {
        gate(&mut first, "waiting-author", findings);
    }
    let bots_done = first.is_none();
    // An attestation says the author read this code, so only a change to
    // the code takes it back: a bot reporting on the same code afterwards
    // does not.
    let instants: Vec<&PyValue> = pasta.iter().chain(&rabbit).chain(waived.values()).collect();
    let completed = match latest(instants.iter().copied())? {
        Some(completed) => completed.clone(),
        None => getitem(pr, "created_at")?.clone(),
    };
    // When the bots last spoke, whether or not they are done.
    result.insert(
        "bot_completed_at".into(),
        if instants.is_empty() {
            PyValue::None
        } else {
            completed.clone()
        },
    );

    // `/self-reviewed <sha>` names the commit it covers. Bare, it means
    // everything pushed so far, which is safe only once this head has a
    // status: an attestation written before the last push cannot be reused.
    // It is the author saying it in an unedited comment or in the body of
    // a review submitted on this diff.
    let seen = &seen_first;
    struct Written<'a> {
        user: &'a PyValue,
        body: &'a PyValue,
        at: &'a PyValue,
    }
    let mut written = Vec::new();
    for c in &comments {
        if py_eq(getitem(c, "created_at")?, getitem(c, "updated_at")?) {
            written.push(Written {
                user: getitem(c, "user")?,
                body: getitem(c, "body")?,
                at: getitem(c, "created_at")?,
            });
        }
    }
    for r in &reviews {
        if commit_in_heads(r, true)? {
            written.push(Written {
                user: getitem(r, "user")?,
                body: or(getitem(r, "body")?, &EMPTY_STR),
                at: getitem(r, "submitted_at")?,
            });
        }
    }
    // Whoever holds it — the one who opened it, and anyone it was handed
    // to — is who can say they read it. Anyone else posting the phrase is
    // told to take it over, but only where that would make it count.
    let holding = holders(policy, pr)?;
    let mut attestations: Vec<PyValue> = Vec::new();
    let mut attested_by = BTreeSet::new();
    let mut on_their_behalf: Vec<&PyValue> = Vec::new();
    for comment in &written {
        let text = py_strip(str_method(comment.body, "strip")?);
        let Some(said) = ATTESTATION.captures(text) else {
            continue;
        };
        let user = lower(comment.user)?;
        if BOTS.contains(&user.as_str()) {
            continue;
        }
        match said.name("head") {
            Some(named) => {
                if !heads.contains(&named.as_str().to_ascii_lowercase()) {
                    continue;
                }
            }
            None => {
                if !seen.truthy() || time(comment.at)? <= time(seen)? {
                    continue;
                }
            }
        }
        if holding.contains(&user) {
            attestations.push(comment.at.clone());
            attested_by.insert(user);
        } else {
            on_their_behalf.push(comment.user);
        }
    }
    if attestations.is_empty() && machine_author(policy, pr)? {
        // An account with nobody behind it cannot attest; the eligible
        // approval it needs anyway stands in, dated when this diff was
        // first seen, else when the bots last spoke.
        attestations = vec![or(seen, &completed).clone()];
    }
    if attestations.is_empty() {
        gate(
            &mut first,
            "waiting-self-review",
            vec![format!("Author must post /self-reviewed {}", py_str(head))],
        );
    }
    let self_time = latest(attestations.iter())?.cloned();
    if first.is_none() {
        result.insert(
            "self_reviewed_at".into(),
            self_time.clone().unwrap_or(PyValue::None),
        );
    }

    let mut objectors: IndexMap<String, PyValue> = IndexMap::new();
    let mut objection_lines: Vec<String> = Vec::new();
    for (user, review) in &latest_reviews {
        if !BOTS.contains(&user.as_str())
            && may_object(&permissions, user)?
            && upper_in(getitem(review, "state")?, &["CHANGES_REQUESTED"])?
        {
            objectors.insert(user.clone(), getitem(review, "submitted_at")?.clone());
            objection_lines.push(format!(
                "{} requested changes",
                py_str(getitem(review, "user")?)
            ));
        }
    }
    for thread in &threads {
        // A bot's thread is answered, not objected in: the replies under
        // its findings are whoever is finishing the pull request.
        if getitem(thread, "is_resolved")?.truthy()
            || BOTS.contains(&lower(getitem(thread, "author")?)?.as_str())
        {
            continue;
        }
        // Everyone who spoke in the thread, not only whoever opened it.
        let voices_value = get(thread, "voices")?;
        let voices: Vec<Cow<'_, PyValue>> = if voices_value.truthy() {
            iterate(voices_value)?
        } else {
            vec![Cow::Owned(dict([
                ("user", getitem(thread, "author")?.clone()),
                ("created_at", getitem(thread, "created_at")?.clone()),
            ]))]
        };
        for voice in &voices {
            let user = lower(getitem(voice, "user")?)?;
            // The author's own words are not an objection to their own pull
            // request; anybody else's are, whoever it was handed to.
            if BOTS.contains(&user.as_str())
                || user == lower(getitem(pr, "author")?)?
                || !may_object(&permissions, &user)?
            {
                continue;
            }
            if !objectors.contains_key(&user) {
                objection_lines.push(format!(
                    "{} left a review thread unresolved",
                    py_str(getitem(voice, "user")?)
                ));
            }
            let spoke = getitem(voice, "created_at")?;
            let earlier = objectors
                .get(&user)
                .cloned()
                .unwrap_or_else(|| spoke.clone());
            // `max` of two never comes back empty.
            if let Some(later) = latest([&earlier, spoke])? {
                let later = later.clone();
                objectors.insert(user, later);
            }
        }
    }
    if first.is_none() {
        result.insert("objections".into(), strs(objection_lines.iter().cloned()));
    }

    let author = lower(getitem(pr, "author")?)?;
    let mut needed: BTreeSet<String> = BTreeSet::new();
    let mut stranded: Vec<&str> = Vec::new();
    let mut approvals: Vec<PyValue> = Vec::new();
    let display = |login: &str| -> String {
        people
            .get(login)
            .map_or_else(|| login.to_owned(), |name| name.to_string())
    };
    for area in touched.values() {
        let files = strs(files_by_area.get(area.id).into_iter().flatten().cloned());
        // You may merge your own work in your own area without a second
        // person; being handed a pull request is not having written it.
        let owners: BTreeSet<String> = area.owners.iter().map(|x| py_lower(x)).collect();
        if owners.contains(&author) {
            approvals.push(dict([
                ("area", s(area.id)),
                ("files", files),
                ("approvers", strs(Vec::<String>::new())),
                ("approved_by", strs(Vec::<String>::new())),
                ("owned", PyValue::Bool(true)),
            ]));
            continue;
        }
        // Nobody approves what they attested to; whoever is holding it and
        // said nothing may still approve.
        let eligible: BTreeSet<String> = area
            .people()
            .map(py_lower)
            .filter(|login| *login != author && !attested_by.contains(login))
            .collect();
        let mut approved_by = Vec::new();
        for login in &eligible {
            if let Some(review) = latest_reviews.get(login) {
                if upper_in(getitem(review, "state")?, &["APPROVED"])?
                    && commit_in_heads(review, false)?
                {
                    approved_by.push(display(login));
                }
            }
        }
        approved_by.sort();
        let mut approvers: Vec<String> = eligible.iter().map(|login| display(login)).collect();
        approvers.sort();
        let approved = !approved_by.is_empty();
        // Recorded either way: an approval already covering an area is as
        // much of the answer as the one still missing.
        approvals.push(dict([
            ("area", s(area.id)),
            ("files", files),
            ("approvers", strs(approvers)),
            ("approved_by", strs(approved_by)),
            ("owned", PyValue::Bool(false)),
        ]));
        if !approved {
            // Everyone who could approve this area attested to it instead:
            // an empty ask read as a met one would merge it unread.
            if eligible.is_empty() {
                stranded.push(area.id);
            }
            needed.extend(eligible);
        }
    }
    if first.is_none() {
        result.insert(
            "approvals".into(),
            PyValue::List(approvals.iter().cloned().collect()),
        );
    }
    // An objection at or after the attestation is the author's to answer;
    // one before it has been answered and waits on the objector.
    let mut unanswered = BTreeSet::new();
    for (user, at) in &objectors {
        let open = match &self_time {
            None => true,
            Some(self_time) => time(at)? >= time(self_time)?,
        };
        if open {
            unanswered.insert(user.clone());
        }
    }
    if self_time.is_some() && !unanswered.is_empty() {
        gate(
            &mut first,
            "waiting-author",
            vec!["Author response is required after the latest human objection".into()],
        );
    }
    for user in objectors.keys() {
        if *user != author && may_object(&permissions, user)? && !BOTS.contains(&user.as_str()) {
            needed.insert(user.clone());
        }
    }
    let human = !needed.is_empty() || !objectors.is_empty() || !stranded.is_empty();
    let previous = or(get(pr, "controller_state")?, &EMPTY_DICT);
    // Latched on the recorded state, and on ever having been ready for this
    // head, not on still being ready: a pull request that passed through
    // another state would otherwise need a green build again.
    let was_ready = get(pr, "ready_published")?.truthy()
        || (py_eq_str(get(previous, "state")?, "ready-for-human")
            && py_eq(get(previous, "head")?, head));
    let build = getitem(pr, "build")?;
    let green = py_eq_str(build, "green");
    let failed = py_eq_str(build, "failed");
    if human {
        if !admitted_at.truthy() && !machine_author(policy, pr)? {
            // Five at a time is a limit on human attention, so it applies
            // where a human would be asked; a machine author has none to
            // ration.
            gate(
                &mut first,
                "too-many-open-prs",
                vec![format!(
                    "More than {} open pull requests; this one waits until one merges",
                    py_str(getitem(policy, "max_active_prs")?)
                )],
            );
        }
        if !was_ready && !green {
            // Green before a human is asked; red afterwards does not take
            // the request back.
            gate(
                &mut first,
                "waiting-build",
                vec![if failed {
                    "The build must pass before a human is asked".into()
                } else {
                    "Waiting for the build to finish".into()
                }],
            );
        }
        if first.is_none() {
            let mut reviewers: Vec<String> = needed.iter().map(|u| display(u)).collect();
            reviewers.sort();
            result.insert("reviewers".into(), strs(reviewers));
            // Who among them is asked because of their own objection.
            let mut re_review: Vec<String> = objectors
                .keys()
                .filter(|u| needed.contains(*u))
                .map(|u| display(u))
                .collect();
            re_review.sort();
            result.insert("objectors".into(), strs(re_review));
            if was_ready && get(previous, "ready_since")?.truthy() {
                let since = getitem(previous, "ready_since")?;
                time(since)?;
                result.insert("ready_since".into(), since.clone());
            } else if get(previous, "admitted_at")?.truthy() || machine_author(policy, pr)? {
                // A machine author holds no slot, so admission is never
                // recorded for it; its waiting time starts now.
                result.insert("ready_since".into(), now.clone());
            }
        }
        let mut reasons = vec!["Human approval or objection resolution is required".to_owned()];
        if !stranded.is_empty() {
            stranded.sort();
            reasons.push(format!(
                "Nobody may approve {}: everyone who could has attested to it instead",
                stranded.join(", ")
            ));
        }
        gate(&mut first, "ready-for-human", reasons);
    } else if !green {
        // The owner path asks no human, so nothing else looks at the build.
        gate(
            &mut first,
            "waiting-build",
            vec![if failed {
                "The build must pass before this can merge".into()
            } else {
                "Waiting for the build to finish".into()
            }],
        );
    }

    // In the order the verdict weighs them, so the first unchecked line is
    // the state.
    let first_word = |line: &str| py_lower(line.split_once(' ').map_or(line, |(word, _)| word));
    let skippable = missing.iter().any(|bot| !waived.contains_key(bot));
    let self_review_done =
        !attestations.is_empty() && !(self_time.is_some() && !unanswered.is_empty());
    let bot_author = machine_author(policy, pr)?;
    let mut behalf: Vec<String> = Vec::new();
    for user in &on_their_behalf {
        let user = py_str(user).into_owned();
        if !behalf.contains(&user) {
            behalf.push(user);
        }
    }
    behalf.sort();
    let address = objection_lines
        .iter()
        .filter(|line| unanswered.contains(&first_word(line)) || self_time.is_none())
        .cloned();
    let self_review = [
        ("done", PyValue::Bool(self_review_done)),
        ("bot_author", PyValue::Bool(bot_author)),
        ("on_their_behalf", strs(behalf)),
        ("address", strs(address)),
    ];
    let slot_done = !human || admitted_at.truthy() || machine_author(policy, pr)?;
    let awaiting = objection_lines
        .iter()
        .filter(|line| self_time.is_some() && !unanswered.contains(&first_word(line)))
        .cloned();
    let checklist: [(&str, Vec<(&str, PyValue)>); 5] = [
        (
            "bots",
            vec![
                ("done", PyValue::Bool(bots_done)),
                ("lines", strs(bot_lines)),
                ("skippable", PyValue::Bool(skippable)),
            ],
        ),
        ("self_review", self_review.into()),
        (
            "slot",
            vec![
                ("done", PyValue::Bool(slot_done)),
                ("limit", getitem(policy, "max_active_prs")?.clone()),
            ],
        ),
        (
            "build",
            vec![
                ("done", PyValue::Bool(green)),
                ("state", build.clone()),
                ("latched", PyValue::Bool(was_ready && human)),
            ],
        ),
        (
            "approvals",
            vec![
                ("done", PyValue::Bool(!human)),
                ("areas", PyValue::List(approvals.into_iter().collect())),
                ("awaiting", strs(awaiting)),
            ],
        ),
    ];
    let checklist: PyList = checklist
        .into_iter()
        .map(|(item, facts)| {
            PyValue::Dict(
                std::iter::once(("item".to_owned(), s(item)))
                    .chain(facts.into_iter().map(|(k, v)| (k.to_owned(), v)))
                    .collect(),
            )
        })
        .collect();
    result.insert("checklist".into(), PyValue::List(checklist));

    Ok(match first {
        Some((state, reasons)) => Stop::pending(state, reasons),
        None => Stop {
            state: "ready-to-merge",
            status: "success",
            reasons: vec!["All policy requirements are satisfied".into()],
        },
    })
}

/// `login.removesuffix('[bot]')`.
fn without_bot_suffix(login: &str) -> String {
    login.strip_suffix("[bot]").unwrap_or(login).to_owned()
}

/// `_latest_reviews(reviews)`: each person's latest decisive review —
/// approved, changes requested or dismissed — by `(submitted_at, id)`,
/// keyed by lowercase login in the order each person first appears.
fn latest_reviews<'a>(
    reviews: &'a [Cow<'a, PyValue>],
) -> Result<IndexMap<String, &'a PyValue>, PyErr> {
    let mut keyed = Vec::new();
    for review in reviews {
        let at = time(getitem(review, "submitted_at")?)?;
        let id = getitem(review, "id")?;
        keyed.push((at, id, review.as_ref()));
    }
    // `(instant, id) < (instant, id)`: by the first items that differ.
    py_sort_by(&mut keyed, |a, b| match a.0.cmp(&b.0) {
        Ordering::Equal => Ok(!py_same_element(a.1, b.1) && py_compare(a.1, Compare::Lt, b.1)?),
        other => Ok(other == Ordering::Less),
    })?;
    let mut latest = IndexMap::new();
    for (_, _, review) in keyed {
        if upper_in(
            getitem(review, "state")?,
            &["APPROVED", "CHANGES_REQUESTED", "DISMISSED"],
        )? {
            latest.insert(lower(getitem(review, "user")?)?, review);
        }
    }
    Ok(latest)
}
