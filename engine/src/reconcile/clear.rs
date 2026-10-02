//! `clear_marks`: the engine's marks taken off a pull request it no longer
//! governs, and its record set aside rather than deleted.

use super::state::bot_comments;
use super::values::{number, shown, text};
use super::{except_github, Reconciler, WAIVED_LABEL};
use crate::evidence::records::{
    current_checklist, parse_controller_state, state_comment_body, DIFF_MARKER, STATE_MARKER,
};
use crate::evidence::{ReadError, Transport};
use crate::policy::{governs, RETIRED_LABELS, STATE_LABELS};
use crate::pycompat::object::{get, getitem, iterate, or, EMPTY_LIST};
use crate::pycompat::ops::{py_eq, py_eq_str, py_hashable};
use crate::pycompat::{PyDict, PyList, PyValue};

impl<T: Transport> Reconciler<'_, T> {
    /// `clear_marks(api, policy, prs, apply)`: take the engine's marks off
    /// every open pull request it no longer governs.
    ///
    /// A pull request rebased onto a branch outside the policy is never
    /// selected again, so its labels and its checklist stayed exactly as
    /// they were the day it left — which reads as a verdict and is not one.
    /// The record comment stays, saying why nothing is checked, and keeps
    /// what was said about each diff; the review slot and the review clock
    /// it held are given up (see [`Reconciler::set_aside_record`]).
    ///
    /// A name the engine has retired goes too, but only where it left a
    /// mark of its own: `ready-to-merge` is ordinary English, and a pull
    /// request that was never governed may be wearing somebody else's.
    pub fn clear_marks(
        &mut self,
        policy: &PyValue,
        prs: &[PyValue],
        apply: bool,
    ) -> Result<(), ReadError> {
        let mine: Vec<&str> = STATE_LABELS.iter().copied().chain([WAIVED_LABEL]).collect();
        for pr in prs {
            if !py_eq_str(getitem(pr, "state")?, "open") || governs(policy, getitem(pr, "base")?)? {
                continue;
            }
            let worn = or(get(pr, "labels")?, &EMPTY_LIST);
            let mut labels: Vec<String> = Vec::new();
            for label in iterate(worn)? {
                py_hashable(&label)?;
                if let PyValue::Str(label) = label.as_ref() {
                    if !labels.contains(label) {
                        labels.push(label.clone());
                    }
                }
            }
            let block = current_checklist(get(pr, "body")?.unwrap_or(&PyValue::None))?;
            let wears = |names: &[&str]| labels.iter().any(|label| names.contains(&label.as_str()));
            // A retired name on its own proves nothing, so the record
            // comment is what says whether the engine put it there. That
            // read costs a request and is spent only on the few pull
            // requests still wearing one. A pull request with no mark at
            // all is not read at all.
            if !wears(&mine)
                && block.is_none()
                && !(wears(&RETIRED_LABELS) && self.was_ours(pr, apply)?)
            {
                continue;
            }
            let mut stale: Vec<String> = labels
                .iter()
                .filter(|label| {
                    mine.contains(&label.as_str()) || RETIRED_LABELS.contains(&label.as_str())
                })
                .cloned()
                .collect();
            stale.sort();
            let n = number(pr)?;
            let what: Vec<String> = [
                stale.join(", "),
                if block.is_some() {
                    "the checklist".to_owned()
                } else {
                    String::new()
                },
            ]
            .into_iter()
            .filter(|part| !part.is_empty())
            .collect();
            self.say(format!(
                "PR #{n}: no longer governed ({}); clearing {}",
                shown(getitem(pr, "base")?),
                what.join(", ")
            ));
            if !apply {
                continue;
            }
            // The record goes first: if it cannot be written, the marks
            // stay and the next sweep comes back to it.
            if !self.set_aside_record(pr)? {
                continue;
            }
            for label in &stale {
                if except_github(self.api.set_label(&n, label, false, worn))?.is_err() {
                    self.say(format!("PR #{n}: could not remove {label}"));
                }
            }
            if block.is_some() {
                if let Err(error) = except_github(self.api.remove_checklist(&n))? {
                    self.say(format!("PR #{n}: could not remove the checklist: {error}"));
                }
            }
        }
        Ok(())
    }

    /// `_was_ours(api, pr, apply)`: whether the engine's record comment is
    /// still on a pull request it no longer governs.
    fn was_ours(&mut self, pr: &PyValue, apply: bool) -> Result<bool, ReadError> {
        if !apply {
            return Ok(false);
        }
        let comments = match except_github(self.api.comments(&number(pr)?))? {
            Ok(comments) => comments,
            Err(_) => return Ok(false),
        };
        let PyValue::Dict(fields) = pr else {
            return Err(ReadError::NotPorted(
                "a listed pull request that is not a dict".into(),
            ));
        };
        let mut with: PyDict = fields.iter().map(|(k, v)| (k.clone(), v.clone())).collect();
        with.insert("comments".into(), PyValue::List(PyList::from(comments)));
        match except_github(bot_comments(&PyValue::Dict(with), STATE_MARKER))? {
            Ok(found) => Ok(!found.is_empty()),
            Err(_) => Ok(false),
        }
    }

    /// `_set_aside_record(api, pr)`: keep what was said about each diff,
    /// give up the review slot and the review clock. Whether the pull
    /// request may lose its marks now.
    ///
    /// A pull request that leaves the policy and comes back is treated like
    /// one converted to draft and back: it queues for a slot again and its
    /// review wait starts again, instead of displacing whoever was admitted
    /// while it was away or counting the time away as waiting. What the
    /// bots and the author said about each diff is kept byte for byte, so
    /// none of it is asked again. Only the current record — the one written
    /// last — is rewritten: rewriting an older one too could make its stale
    /// state the newest. It is read where each comment's editor is known:
    /// without that a forged record cannot be told from the engine's own.
    pub fn set_aside_record(&mut self, pr: &PyValue) -> Result<bool, ReadError> {
        let n = number(pr)?;
        let read = (|| -> Result<_, ReadError> {
            let comments = self
                .api
                .histories(std::slice::from_ref(&n))?
                .shift_remove(&n)
                .map(|history| history.comments)
                .unwrap_or_default();
            let record = parse_controller_state(&comments)?;
            Ok((comments, record))
        })();
        let (comments, record) = match except_github(read)? {
            Ok(read) => read,
            Err(error) => {
                self.say(format!(
                    "PR #{n}: could not read the record comment: {error}"
                ));
                return Ok(false);
            }
        };
        let Some(record) = record else {
            return Ok(true);
        };
        if !py_eq(
            get(&record.state, "number")?.unwrap_or(&PyValue::None),
            &PyValue::Int(n.clone()),
        ) {
            return Ok(true);
        }
        let mut holder = None;
        for comment in &comments {
            if py_eq(getitem(comment, "id")?, &record.comment_id) {
                holder = Some(comment);
                break;
            }
        }
        let Some(holder) = holder else {
            return Err(ReadError::NotPorted(
                "a record read from a comment that is not among them".into(),
            ));
        };
        let held = text(getitem(holder, "body")?, "body")?;
        let markers = held.split("\n\n").next().unwrap_or("");
        let diff_line = markers
            .split('\n')
            .find(|line| line.starts_with(DIFF_MARKER));
        let base = text(getitem(pr, "base")?, "base")?.replace('`', "");
        let note = format!(
            "PR Hygiene is not checking this pull request: it targets `{base}`, outside the policy. \
             What was said about its diffs is kept for when it returns; it will queue for a review slot again."
        );
        let PyValue::Dict(fields) = &record.state else {
            return Err(ReadError::NotPorted("a record that is not a dict".into()));
        };
        let mut set_aside: PyDict = fields.iter().map(|(k, v)| (k.clone(), v.clone())).collect();
        set_aside.insert("admitted_at".into(), PyValue::None);
        set_aside.insert("ready_since".into(), PyValue::None);
        set_aside.insert("state".into(), PyValue::Str("not-governed".into()));
        let written = state_comment_body(&PyValue::Dict(set_aside), &note, None)?;
        let (marker, display) = written.split_once("\n\n").unwrap_or((written.as_str(), ""));
        let mut body = marker.to_owned();
        if let Some(line) = diff_line.filter(|line| !line.is_empty()) {
            body.push('\n');
            body.push_str(line);
        }
        body.push_str("\n\n");
        body.push_str(display);
        if body == held {
            return Ok(true);
        }
        if let Err(error) = except_github(self.api.edit_comment(&record.comment_id, &body))? {
            self.say(format!(
                "PR #{n}: could not update the record comment: {error}"
            ));
            return Ok(false);
        }
        Ok(true)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn the_marker_block_ends_at_the_first_blank_line() {
        // `body.partition('\n\n')[0]`: the whole body when there is none.
        assert_eq!("a\nb".split("\n\n").next(), Some("a\nb"));
        assert_eq!("a\n\nb\n\nc".split("\n\n").next(), Some("a"));
    }
}
