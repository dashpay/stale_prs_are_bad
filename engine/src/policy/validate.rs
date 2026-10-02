//! Reading a policy: `validate_policy`, which refuses a malformed one with
//! Python's messages and in Python's order (the first problem found is the
//! one reported), and `governs`.
//!
//! Python's `validate_policy` can also check each area's directories on
//! disk; that reads the filesystem, and the engine does no I/O, so that
//! check stays with the command that has a checkout to look at.

use super::patterns::{branch_target, AREA_ID, BRANCH_SYNTAX, HANDLE, PREFIX, REPOSITORY};
use super::values::{lower, re_text, strings};
use super::{BOTS, REVIEW_BOTS};
use crate::pycompat::object::{get_or, getitem, iterate, str_method, EMPTY_LIST};
use crate::pycompat::ops::{py_eq, py_hashable};
use crate::pycompat::text::{py_lower, py_strip};
use crate::pycompat::{PyErr, PyValue};
use std::collections::BTreeSet;

/// Handles nobody may name: two people, and the machines.
const EXCLUDED: [&str; 4] = ["strophy", "silvanassss", "copilot", "dependabot"];

fn invalid(message: &str) -> PyErr {
    PyErr::value(message)
}

/// `_fields(value, allowed, required)`: a dict with every required key and
/// no other than the allowed ones.
fn fields(value: &PyValue, allowed: &[&str], required: &[&str]) -> Result<(), PyErr> {
    let fits = match value {
        PyValue::Dict(entries) => {
            entries.keys().all(|key| allowed.contains(&key.as_str()))
                && required.iter().all(|key| entries.contains_key(*key))
        }
        _ => false,
    };
    if fits {
        Ok(())
    } else {
        Err(invalid("Unknown or missing policy fields"))
    }
}

/// `_handles(values, nonempty)`: a list of distinct GitHub handles, none of
/// them a bot or an excluded identity.
fn handles(values: &PyValue, nonempty: bool) -> Result<(), PyErr> {
    let PyValue::List(items) = values else {
        return Err(invalid("Expected handle list"));
    };
    if nonempty && items.is_empty() {
        return Err(invalid("Expected handle list"));
    }
    let mut seen = BTreeSet::new();
    for handle in items.iter() {
        let PyValue::Str(handle) = handle else {
            return Err(invalid("Malformed GitHub handle"));
        };
        if !HANDLE.is_match(handle) {
            return Err(invalid("Malformed GitHub handle"));
        }
        let folded = py_lower(handle);
        if seen.contains(&folded)
            || EXCLUDED.contains(&folded.as_str())
            || BOTS.contains(&folded.as_str())
        {
            return Err(invalid("Duplicate or excluded identity"));
        }
        seen.insert(folded);
    }
    Ok(())
}

/// The lowercased handles of a list `handles` has accepted.
fn handle_set(values: &PyValue) -> Result<BTreeSet<String>, PyErr> {
    iterate(values)?.iter().map(|h| lower(h)).collect()
}

/// Whether every item is a string with something besides whitespace.
fn all_text(items: &[PyValue]) -> bool {
    items
        .iter()
        .all(|x| matches!(x, PyValue::Str(s) if !py_strip(s).is_empty()))
}

/// `type(value) is int and value == expected`: `True` is not an int here.
fn is_int(value: &PyValue, expected: std::ops::RangeInclusive<i64>) -> bool {
    matches!(value, PyValue::Int(i) if i.as_i64().is_some_and(|v| expected.contains(&v)))
}

/// `validate_policy(policy)`: `ValueError` with Python's message for the
/// first problem Python would find, in the order it looks.
pub fn validate_policy(policy: &PyValue) -> Result<(), PyErr> {
    const KEYS: [&str; 6] = [
        "version",
        "repository",
        "fallback",
        "max_active_prs",
        "target_branches",
        "areas",
    ];
    let allowed: Vec<&str> = KEYS
        .iter()
        .copied()
        .chain(["required_bots", "bot_timeouts", "bot_authors"])
        .collect();
    fields(policy, &allowed, &KEYS)?;
    handles(get_or(policy, "bot_authors", &EMPTY_LIST)?, false)?;

    let timeouts = get_or(policy, "bot_timeouts", &PyValue::None)?;
    if !matches!(timeouts, PyValue::None) {
        let both = ["nudge_after_hours", "waive_after_hours"];
        fields(timeouts, &both, &both)?;
        if let PyValue::Dict(entries) = timeouts {
            if entries.values().any(|hours| !is_int(hours, 1..=168)) {
                return Err(invalid(
                    "bot_timeouts must be whole hours between 1 and 168",
                ));
            }
        }
    }

    if let Some(bots) = entry(policy, "required_bots") {
        let refused = || {
            invalid(&format!(
                "required_bots must be a subset of {}",
                REVIEW_BOTS.join(", ")
            ))
        };
        let PyValue::List(items) = bots else {
            return Err(refused());
        };
        // `set(bots)` hashes every item before anything is compared.
        for item in items.iter() {
            py_hashable(item)?;
        }
        let duplicated = items
            .iter()
            .enumerate()
            .any(|(i, a)| items[..i].iter().any(|b| py_eq(a, b)));
        let unknown = items
            .iter()
            .any(|bot| !matches!(bot, PyValue::Str(s) if REVIEW_BOTS.contains(&s.as_str())));
        if duplicated || unknown {
            return Err(refused());
        }
    }

    if !is_int(getitem(policy, "version")?, 1..=1) {
        return Err(invalid("Unsupported policy version"));
    }
    match getitem(policy, "repository")? {
        PyValue::Str(repository) if REPOSITORY.is_match(repository) => {}
        _ => return Err(invalid("Invalid repository")),
    }
    if !is_int(getitem(policy, "max_active_prs")?, 5..=5) {
        return Err(invalid("Expected five active slots"));
    }

    let branches = match getitem(policy, "target_branches")? {
        PyValue::List(items) if !items.is_empty() && all_text(items) => items,
        _ => return Err(invalid("Invalid target branches")),
    };
    let names: Vec<&str> = branches
        .iter()
        .filter_map(|x| match x {
            PyValue::Str(s) => Some(s.as_str()),
            _ => None,
        })
        .collect();
    if names.iter().collect::<BTreeSet<_>>().len() != names.len() {
        return Err(invalid("Invalid target branches"));
    }
    // `*` is the only pattern `governs` understands; anything else GitHub's
    // rules accept would be read literally and match no branch at all.
    if names.iter().any(|x| BRANCH_SYNTAX.is_match(x)) {
        return Err(invalid("Target branch patterns support only `*`"));
    }

    let fallback = getitem(policy, "fallback")?;
    fields(fallback, &["owners", "reviewers"], &["owners", "reviewers"])?;
    handles(getitem(fallback, "owners")?, true)?;
    handles(getitem(fallback, "reviewers")?, false)?;

    let PyValue::List(areas) = getitem(policy, "areas")? else {
        return Err(invalid("Expected areas list"));
    };
    let mut names = BTreeSet::new();
    let mut prefixes: Vec<&str> = Vec::new();
    for area in areas.iter() {
        fields(
            area,
            &[
                "id",
                "paths",
                "owners",
                "reviewers",
                "unresolved",
                "metadata",
            ],
            &["id", "paths", "owners", "reviewers"],
        )?;
        let id = match getitem(area, "id")? {
            PyValue::Str(id) if AREA_ID.is_match(id) && !names.contains(id) && id != "fallback" => {
                id
            }
            _ => return Err(invalid("Invalid or duplicate area id")),
        };
        names.insert(id.clone());
        let owners = getitem(area, "owners")?;
        handles(owners, false)?;
        if !owners.truthy() && !get_or(area, "unresolved", &PyValue::None)?.truthy() {
            return Err(invalid(
                "Missing owners require an explicit unresolved identity",
            ));
        }
        let reviewers = getitem(area, "reviewers")?;
        handles(reviewers, false)?;
        if !handle_set(owners)?.is_disjoint(&handle_set(reviewers)?) {
            return Err(invalid("Owner and reviewer roles overlap"));
        }
        if let Some(unresolved) = entry(area, "unresolved") {
            if !matches!(unresolved, PyValue::List(items) if all_text(items)) {
                return Err(invalid("Invalid unresolved identities"));
            }
        }
        if let Some(metadata) = entry(area, "metadata") {
            if !matches!(metadata, PyValue::Dict(_)) {
                return Err(invalid("Invalid area metadata"));
            }
        }
        let paths = match getitem(area, "paths")? {
            PyValue::List(items) if !items.is_empty() => items,
            _ => return Err(invalid("Expected literal directory prefixes")),
        };
        for prefix in paths.iter() {
            let prefix = match prefix {
                PyValue::Str(prefix)
                    if (prefix.is_empty() || PREFIX.is_match(prefix))
                        && !prefix.split('/').any(|x| x == "." || x == "..") =>
                {
                    prefix.as_str()
                }
                _ => return Err(invalid("Invalid literal directory prefix")),
            };
            if prefixes
                .iter()
                .any(|other| prefix.starts_with(other) || other.starts_with(prefix))
            {
                return Err(invalid("Overlapping directory prefixes"));
            }
            prefixes.push(prefix);
        }
    }

    // A machine author needs no attestation because the approval it cannot
    // do without stands in for one. Owning an area it would need neither,
    // and the owner exemption would merge its pull requests unread.
    let machines = handle_set(get_or(policy, "bot_authors", &EMPTY_LIST)?)?;
    let mut named = handle_set(getitem(fallback, "owners")?)?;
    named.extend(handle_set(getitem(fallback, "reviewers")?)?);
    for area in areas.iter() {
        named.extend(handle_set(getitem(area, "owners")?)?);
        named.extend(handle_set(getitem(area, "reviewers")?)?);
    }
    let both: Vec<&str> = machines.intersection(&named).map(String::as_str).collect();
    if !both.is_empty() {
        return Err(invalid(&format!(
            "A machine author cannot own or review: {}",
            both.join(", ")
        )));
    }
    Ok(())
}

/// The value under `key`, when the dict has one.
fn entry<'a>(value: &'a PyValue, key: &str) -> Option<&'a PyValue> {
    match value {
        PyValue::Dict(entries) => entries.get(key),
        _ => None,
    }
}

/// `set(policy.get('required_bots', REVIEW_BOTS))`, for a policy
/// `validate_policy` accepted: sorted, as every use of it sorts it.
pub(crate) fn required_bots(policy: &PyValue) -> Result<BTreeSet<String>, PyErr> {
    match entry(policy, "required_bots") {
        Some(bots) => Ok(strings(bots)?.into_iter().map(str::to_owned).collect()),
        None => Ok(REVIEW_BOTS.iter().map(|bot| bot.to_string()).collect()),
    }
}

/// `governs(policy, branch)`: whether pull requests into `branch` are under
/// this policy. A target is a branch name, or a pattern in which `*`
/// matches any run of characters except `/`, as the repository's own branch
/// rules read it.
pub fn governs(policy: &PyValue, branch: &PyValue) -> Result<bool, PyErr> {
    if !branch.truthy() {
        return Ok(false);
    }
    for target in iterate(getitem(policy, "target_branches")?)? {
        let target = str_method(&target, "split")?;
        let text = re_text(branch)?;
        if branch_target(target)?.is_match(text) {
            return Ok(true);
        }
    }
    Ok(false)
}
