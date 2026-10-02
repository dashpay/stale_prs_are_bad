//! The words the engine writes for people: the checklist in the
//! description, the move comment, and what a pull request still needs.
//! Each reads a verdict as `evaluate` returns it.

use super::values::{join, one_of, prefix, shown, text};
use crate::policy::{CHECKLIST_END, CHECKLIST_START, MOVE_MARKER};
use crate::pycompat::object::{get, getitem, iterate, or, EMPTY_LIST};
use crate::pycompat::ops::{py_eq_str, py_hashable};
use crate::pycompat::text::py_lower;
use crate::pycompat::{PyErr, PyValue};
use std::borrow::Cow;

/// `MOVE_STATES`: whose move each state is, as the move comment names it.
/// An objection and a bot's finding are the author's move, as a missing
/// attestation is.
pub const MOVE_STATES: [(&str, &str); 4] = [
    ("waiting-self-review", "waiting-self-review"),
    ("waiting-author", "waiting-self-review"),
    ("ready-for-human", "ready-for-human"),
    ("ready-to-merge", "ready-to-merge"),
];

/// What a record comment says when nobody is told a move: where the
/// checklist is.
pub const POINTER: &str = "PR Hygiene: the checklist is in the description.";

/// `MOVE_STATES.get(state)`.
pub fn move_state(state: &PyValue) -> Result<Option<&'static str>, PyErr> {
    let names: Vec<&str> = MOVE_STATES.iter().map(|(state, _)| *state).collect();
    Ok(one_of(state, &names)?
        .and_then(|found| MOVE_STATES.iter().find(|(state, _)| *state == found))
        .map(|(_, mv)| *mv))
}

/// `{item['item']: item for item in result.get('checklist') or []}`.
struct Items<'r>(Vec<(&'r PyValue, &'r PyValue)>);

impl<'r> Items<'r> {
    fn of(result: &'r PyValue) -> Result<Self, PyErr> {
        let checklist = or(get(result, "checklist")?, &EMPTY_LIST);
        let PyValue::List(entries) = checklist else {
            // A dict's keys and a string's characters cannot be indexed
            // by `'item'`; anything else is not iterable.
            iterate(checklist)?;
            return Err(PyErr::type_error(
                "string indices must be integers, not 'str'",
            ));
        };
        let mut items: Vec<(&PyValue, &PyValue)> = Vec::new();
        for entry in entries.iter() {
            let key = getitem(entry, "item")?;
            py_hashable(key)?;
            match items
                .iter_mut()
                .find(|(known, _)| crate::pycompat::ops::py_eq(known, key))
            {
                Some(slot) => slot.1 = entry,
                None => items.push((key, entry)),
            }
        }
        Ok(Items(items))
    }

    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// `items[name]`.
    fn get(&self, name: &str) -> Result<&'r PyValue, PyErr> {
        self.0
            .iter()
            .find(|(key, _)| py_eq_str(key, name))
            .map(|(_, item)| *item)
            .ok_or_else(|| PyErr::Key(name.to_owned()))
    }
}

fn tick(done: &PyValue) -> &'static str {
    if done.truthy() {
        "[x]"
    } else {
        "[ ]"
    }
}

/// `SAFE_PATH.fullmatch(path)`: `[A-Za-z0-9._/@+-]+`, which a pull
/// request cannot use to break out of a code span or a list item.
fn safe_path(path: &str) -> bool {
    !path.is_empty()
        && path
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || "._/@+-".contains(c))
}

/// `_files(files)`: a few paths, quoted only when safe — a path is the pull
/// request's to choose.
fn files(files: &PyValue) -> Result<String, PyErr> {
    let all = iterate(files)?;
    let mut safe = Vec::new();
    for file in &all {
        if safe_path(text(file, "expected string or bytes-like object")?) {
            safe.push(shown(file));
        }
    }
    if safe.is_empty() {
        return Ok(format!("{} files", all.len()));
    }
    let quoted: Vec<String> = safe.iter().take(3).map(|f| format!("`{f}`")).collect();
    let mut out = quoted.join(", ");
    if all.len() > 3 {
        out.push_str(&format!(" and {} more", all.len() - 3));
    }
    Ok(out)
}

/// `area_name(area, code)`: how an area is named in the words.
pub fn area_name(area: &PyValue, code: bool) -> Cow<'_, str> {
    if py_eq_str(area, "fallback") {
        return Cow::Borrowed("files with no dedicated owner");
    }
    if code {
        Cow::Owned(format!("`{}`", shown(area)))
    } else {
        crate::pycompat::object::py_str(area)
    }
}

/// `checklist_block(result)`: the description's block, every requirement
/// met or not, checked when met — or `None` when the verdict has no
/// checklist.
///
/// One shape always. The state is the first unchecked line, which is what
/// the label and the status say too, so the three surfaces never disagree —
/// and an author with an approval in hand can see which files it did not
/// cover, before anyone has to ask.
pub fn checklist_block(result: &PyValue) -> Result<Option<String>, PyErr> {
    let items = Items::of(result)?;
    if items.is_empty() {
        return Ok(None);
    }
    let mut lines = vec![
        CHECKLIST_START.to_owned(),
        format!("# PR Hygiene · `{}`", prefix(getitem(result, "head")?, 7)?),
    ];
    let bots = items.get("bots")?;
    let mut line = format!(
        "- {} Bots — {}",
        tick(getitem(bots, "done")?),
        join(" · ", getitem(bots, "lines")?)?
    );
    if getitem(bots, "skippable")?.truthy() {
        line.push_str(" — `/skip-bots` proceeds without the ones not yet reported");
    }
    lines.push(line);
    let attest = items.get("self_review")?;
    if getitem(attest, "bot_author")?.truthy() {
        lines.push(format!(
            "- {} Self-review — not asked of a bot author",
            tick(getitem(attest, "done")?)
        ));
    } else if getitem(attest, "address")?.truthy() {
        let done = tick(getitem(attest, "done")?);
        lines.push(format!(
            "- {done} Self-review — address {}, then post `/self-reviewed`",
            join("; ", getitem(attest, "address")?)?
        ));
    } else if getitem(attest, "done")?.truthy() {
        lines.push("- [x] Self-review — posted; again after any push".to_owned());
    } else {
        lines.push("- [ ] Self-review — post `/self-reviewed`".to_owned());
    }
    // Somebody posted it who is not the author. It cannot count, and
    // saying so is the difference between a pull request that moves and one
    // whose author believes it already has. Whoever posted it has read the
    // diff and is simply not holding the pull request yet: assigning
    // themselves makes what they already wrote count.
    if getitem(attest, "on_their_behalf")?.truthy()
        && !getitem(attest, "done")?.truthy()
        && !getitem(attest, "bot_author")?.truthy()
    {
        let mut who = Vec::new();
        for name in iterate(getitem(attest, "on_their_behalf")?)? {
            who.push(format!("@{}", text(&name, "can only concatenate str")?));
        }
        if let Some(last) = lines.last_mut() {
            last.push_str(&format!(
                " — {} posted it. If you have taken this pull request over, \
                 assign it to yourself and what you posted counts",
                who.join(", ")
            ));
        }
    }
    let slot = items.get("slot")?;
    let mut line = format!(
        "- {} Within your {} open PRs",
        tick(getitem(slot, "done")?),
        shown(getitem(slot, "limit")?)
    );
    if !getitem(slot, "done")?.truthy() {
        line.push_str(" — this one is beyond the limit; it waits until one merges");
    }
    lines.push(line);
    let build = items.get("build")?;
    let state = getitem(build, "state")?;
    let fallback = format!("Build {}", shown(state));
    let mut words = match one_of(state, &["green", "failed", "running"])? {
        Some("green") => "Build green".to_owned(),
        Some("failed") => "Build failed".to_owned(),
        Some(_) => "Build running".to_owned(),
        None => fallback,
    };
    if getitem(build, "latched")?.truthy() && !getitem(build, "done")?.truthy() {
        words.push_str(" — review was already requested; it still has to pass to merge");
    }
    lines.push(format!("- {} {words}", tick(getitem(build, "done")?)));
    let approvals = items.get("approvals")?;
    let areas = getitem(approvals, "areas")?;
    let mut all_owned = true;
    for area in iterate(areas)? {
        if !get(&area, "owned")?.is_some_and(PyValue::truthy) {
            all_owned = false;
            break;
        }
    }
    if all_owned && !getitem(approvals, "awaiting")?.truthy() {
        lines.push(format!(
            "- {} Approvals — you own every area touched; none needed",
            tick(getitem(approvals, "done")?)
        ));
    } else {
        // A plain bullet, not a task: a task parent counts in GitHub's
        // N-of-M beside its children and the bar would read double.
        lines.push("- Approvals".to_owned());
        for area in iterate(areas)? {
            let name = area_name(getitem(&area, "area")?, true).into_owned();
            if get(&area, "owned")?.is_some_and(PyValue::truthy) {
                lines.push(format!("  - [x] {name} — you own it"));
            } else if getitem(&area, "approved_by")?.truthy() {
                let paths = files(getitem(&area, "files")?)?;
                lines.push(format!(
                    "  - [x] {name} ({paths}) — approved by {}",
                    join(", ", getitem(&area, "approved_by")?)?
                ));
            } else {
                let mut who = join(" or ", getitem(&area, "approvers")?)?;
                if who.is_empty() {
                    who = "nobody may approve: everyone who could attested to it".to_owned();
                }
                let paths = files(getitem(&area, "files")?)?;
                lines.push(format!("  - [ ] {name} ({paths}) — {who}"));
            }
        }
        for objection in iterate(getitem(approvals, "awaiting")?)? {
            lines.push(format!(
                "  - [ ] {} — waiting for them to re-review or dismiss",
                shown(&objection)
            ));
        }
    }
    lines.push(String::new());
    lines.push(
        "When every box is checked the `PR Hygiene` check passes and this can merge.".to_owned(),
    );
    lines.push(CHECKLIST_END.to_owned());
    Ok(Some(lines.join("\n")))
}

/// `asks(result, code)`: what review a pull request still waits for — each
/// area nobody has approved, then the objectors. An area lists everyone who
/// may approve it, any one of them enough, and one nobody may approve says
/// so, so an empty ask never reads as a met one.
pub fn asks(result: &PyValue, code: bool) -> Result<Vec<String>, PyErr> {
    let mut parts = Vec::new();
    for area in iterate(or(get(result, "approvals")?, &EMPTY_LIST))? {
        if !get(&area, "owned")?.is_some_and(PyValue::truthy)
            && !getitem(&area, "approved_by")?.truthy()
        {
            let mut who = join(" or ", getitem(&area, "approvers")?)?;
            if who.is_empty() {
                who = "nobody may approve".to_owned();
            }
            parts.push(format!(
                "{}: {who}",
                area_name(getitem(&area, "area")?, code)
            ));
        }
    }
    if get(result, "objectors")?.is_some_and(PyValue::truthy) {
        parts.push(format!(
            "re-review or resolve: {}",
            join(", ", getitem(result, "objectors")?)?
        ));
    }
    Ok(parts)
}

/// `your_part(result, logins, code)`: the areas one person, under any of
/// their logins, may approve and nobody has, and any re-review they owe.
pub fn your_part(result: &PyValue, logins: &[&str], code: bool) -> Result<String, PyErr> {
    let mine: Vec<String> = logins.iter().map(|login| py_lower(login)).collect();
    let is_mine = |login: &PyValue| -> Result<bool, PyErr> {
        Ok(mine.contains(&py_lower(crate::pycompat::object::str_method(
            login, "lower",
        )?)))
    };
    let mut parts = Vec::new();
    for area in iterate(or(get(result, "approvals")?, &EMPTY_LIST))? {
        if get(&area, "owned")?.is_some_and(PyValue::truthy)
            || getitem(&area, "approved_by")?.truthy()
        {
            continue;
        }
        let approvers = iterate(getitem(&area, "approvers")?)?;
        let mut asked = false;
        for approver in &approvers {
            asked |= is_mine(approver)?;
        }
        if !asked {
            continue;
        }
        let mut others = Vec::new();
        for approver in &approvers {
            if !is_mine(approver)? {
                others.push(text(approver, "sequence item")?.to_owned());
            }
        }
        let mut part = area_name(getitem(&area, "area")?, code).into_owned();
        if !others.is_empty() {
            part.push_str(&format!(" (you or {})", others.join(" or ")));
        }
        parts.push(part);
    }
    let mut objects = false;
    for objector in iterate(or(get(result, "objectors")?, &EMPTY_LIST))? {
        objects |= is_mine(&objector)?;
    }
    if objects {
        parts.push("re-review or resolve your objection".to_owned());
    }
    Ok(parts.join(" · "))
}

/// `review_text(row, user)`: the review half of a report's next action —
/// what is needed, and the user's own part of it.
pub fn review_text(row: &PyValue, user: Option<&str>) -> Result<String, PyErr> {
    let asked = asks(row, true)?;
    let mut out = format!(
        "needs {}",
        if asked.is_empty() {
            "an owner".to_owned()
        } else {
            asked.join(" · ")
        }
    );
    if let Some(user) = user.filter(|user| !user.is_empty()) {
        let part = your_part(row, &[user], true)?;
        if !part.is_empty() {
            out.push_str(&format!("; your part: {part}"));
        }
    }
    Ok(out)
}

/// `move_text(result)`: one line for the person whose move it now is, under
/// the marker that says which move and which head — or `None` where it is
/// nobody's move. No mentions: the comment itself notifies.
pub fn move_text(result: &PyValue) -> Result<Option<String>, PyErr> {
    let Some(mv) = move_state(getitem(result, "state")?)? else {
        return Ok(None);
    };
    let items = Items::of(result)?;
    let line = match mv {
        "waiting-self-review" => {
            // What actually blocks it, not what usually does. A bot's own
            // finding lands in this move too, and "bots are done, post
            // /self-reviewed" is wrong three ways there: the bots are not
            // done, the author has already attested, and the thing owed goes
            // unsaid. Which of the two it is, is the bots line of the
            // checklist.
            let mut reasons = Vec::new();
            for blocker in iterate(or(get(result, "blockers")?, &EMPTY_LIST))? {
                let blocker =
                    crate::pycompat::object::str_method(&blocker, "startswith")?.to_owned();
                if !blocker.starts_with("Proceeded without") {
                    reasons.push(blocker);
                }
            }
            let address = getitem(items.get("self_review")?, "address")?;
            if getitem(items.get("bots")?, "done")?.truthy() {
                let what = if address.truthy() {
                    format!(
                        "address {}, then post `/self-reviewed`",
                        join("; ", address)?
                    )
                } else {
                    "post `/self-reviewed`".to_owned()
                };
                format!("Bots are done — your move: {what}.")
            } else {
                let what = if reasons.is_empty() {
                    "answer the review".to_owned()
                } else {
                    reasons.join("; ")
                };
                format!("Your move: {what}.")
            }
        }
        "ready-for-human" => {
            let asked = asks(result, true)?;
            format!(
                "Ready for review — {}.",
                if asked.is_empty() {
                    "needs an owner".to_owned()
                } else {
                    asked.join(" · ")
                }
            )
        }
        _ => "Policy satisfied — this can merge.".to_owned(),
    };
    Ok(Some(format!(
        "{MOVE_MARKER} state={mv} sha={} -->\n{line}\nFull checklist in the description.",
        shown(getitem(result, "head")?)
    )))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_safe_path_is_the_ascii_class_python_matches() {
        assert!(safe_path("packages/drive/a.rs"));
        assert!(safe_path("@scope/x+y-z_1.ts"));
        assert!(!safe_path(""));
        assert!(!safe_path("y/`z`.rs"));
        assert!(!safe_path("résumé.md"));
        assert!(!safe_path("a b"));
    }

    #[test]
    fn every_state_with_a_move_names_it() {
        let state = |s: &str| PyValue::Str(s.into());
        assert_eq!(
            move_state(&state("waiting-author")).unwrap(),
            Some("waiting-self-review")
        );
        assert_eq!(move_state(&state("waiting-bots")).unwrap(), None);
        assert_eq!(move_state(&PyValue::None).unwrap(), None);
        assert!(move_state(&PyValue::List(Vec::new().into())).is_err());
    }
}
