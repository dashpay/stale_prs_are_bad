//! `publish`: a verdict made visible — the commit status, the labels, the
//! checklist in the description, the record comment and the move comment,
//! the reviewer requests — each written only after the pull request and its
//! evidence are read again and found unchanged. And `nudge`: asking a
//! review bot to look at a head.

use super::state::{bot_comments, context_fingerprint, diff_record, same, state_record, visible};
use super::text::{checklist_block, move_state, move_text, POINTER};
use super::values::{list, number, one_of, prefix, s, shown, text};
use super::writes::BODY_LIMIT;
use super::{except_github, Reconciler, WAIVED_LABEL};
use crate::evidence::records::{current_checklist, DIFF_MARKER, STATE_MARKER};
use crate::evidence::{ReadError, Transport};
use crate::policy::{
    fingerprint, machine_author, LABEL_FOR_STATE, MOVE_MARKER, NUDGE_MARKER, RETIRED_LABELS,
    STATE_LABELS,
};
use crate::pycompat::object::{get, get_or, getitem, iterate, or, EMPTY_LIST, NONE};
use crate::pycompat::ops::{py_compare, py_contains, py_eq, py_eq_str, Compare};
use crate::pycompat::text::py_len;
use crate::pycompat::{PyDict, PyValue};

/// The most reviewers GitHub lets a pull request have requested at once.
const REVIEWER_CAP: usize = 15;

/// What one publication works from: the pull request, its verdict and the
/// admission context it was decided in.
pub(super) struct Publication<'p> {
    pub(super) policy: &'p PyValue,
    pub(super) pr: &'p PyValue,
    pub(super) result: &'p PyValue,
    pub(super) context: String,
}

impl<T: Transport> Reconciler<'_, T> {
    /// `nudge(api, pr, result, allowance)`: ask each review bot the verdict
    /// names to look at this head, at most `allowance` of them. How many
    /// were asked.
    ///
    /// Asking is best effort: a comment that cannot be posted is reported
    /// and asked again on the next run. It never fails a reconciliation,
    /// because the waiver that eventually unblocks the pull request does
    /// not depend on it.
    pub fn nudge(
        &mut self,
        pr: &PyValue,
        result: &PyValue,
        allowance: usize,
    ) -> Result<usize, ReadError> {
        let mut posted = 0;
        let empty = list(Vec::new());
        for bot in iterate(get_or(result, "nudge", &empty)?)? {
            if posted >= allowance {
                break;
            }
            let n = number(pr)?;
            let current = self.api.pull(&n)?;
            let head = getitem(pr, "head")?;
            if !py_eq_str(getitem(&current, "state")?, "open")
                || !py_eq(getitem(&current, "head")?, head)
            {
                break;
            }
            let (bot, head) = (shown(&bot), shown(head));
            let body = format!(
                "{NUDGE_MARKER} bot={bot} sha={head} -->\n@{bot} review\n\n\
                 No review for `{}` yet, so PR Hygiene is asking once. \
                 If nothing arrives, the requirement is dropped for this commit and the pull \
                 request is labelled `{WAIVED_LABEL}`.",
                prefix(getitem(pr, "head")?, 8)?
            );
            match except_github(self.api.comment(&n, &body))? {
                Ok(_) => posted += 1,
                Err(_) => self.say(format!(
                    "PR #{n}: could not ask {bot} to review; will retry"
                )),
            }
        }
        Ok(posted)
    }

    /// `publish(api, policy, pr, result, context_prs, apply, candidates)`:
    /// the verdict on GitHub, once the pull request and its evidence are
    /// read again and found as they were. The record written, where one
    /// was; `None` where nothing was published or nothing needed writing.
    ///
    /// Only a sync applies anything. A pull request whose evidence already
    /// shows the verdict — description, labels, record and move comment —
    /// gets its status and nothing else, so a run does not churn what it
    /// does not need to.
    pub fn publish(
        &mut self,
        policy: &PyValue,
        pr: &PyValue,
        result: &PyValue,
        context_prs: &[PyValue],
        apply: bool,
        candidates: Option<&[PyValue]>,
    ) -> Result<Option<PyValue>, ReadError> {
        if !apply {
            return Ok(None);
        }
        let candidates = candidates.unwrap_or(context_prs);
        let state = getitem(result, "state")?;
        let actionable = one_of(state, &["ready-for-human", "ready-to-merge"])?.is_some();
        if !self.identity_matches(pr)? {
            return Ok(None);
        }
        let author = getitem(pr, "author")?;
        let publication = Publication {
            policy,
            pr,
            result,
            context: context_fingerprint(context_prs, author)?,
        };
        let head = text(getitem(pr, "head")?, "head")?.to_owned();
        let n = number(pr)?;
        if actionable {
            let again = self.api.snapshot(&n, policy, None)?;
            if fingerprint(&again)? != fingerprint(pr)? {
                self.api.post_status(
                    &head,
                    "pending",
                    "Review evidence changed; reconciliation required",
                    None,
                )?;
                return Ok(None);
            }
            if !self.admission_valid(&publication, candidates)? {
                self.api.post_status(
                    &head,
                    "pending",
                    "PR admission context changed; reconciliation required",
                    None,
                )?;
                return Ok(None);
            }
        }
        let desired = state_record(pr, result, &publication.context)?;

        let ready = py_eq_str(state, "ready-for-human");
        let requested = iterate(get_or(pr, "requested_reviewers", &EMPTY_LIST)?)?
            .into_iter()
            .map(|user| user.into_owned())
            .fold(Vec::new(), |mut set: Vec<PyValue>, user| {
                if !set.iter().any(|known| py_eq(known, &user)) {
                    set.push(user);
                }
                set
            });
        let mut missing = Vec::new();
        if ready {
            for user in iterate(get_or(result, "reviewers", &EMPTY_LIST)?)? {
                if !requested.iter().any(|known| py_eq(known, &user)) {
                    missing.push(user.into_owned());
                }
            }
        }
        let labels = get_or(pr, "labels", &EMPTY_LIST)?;
        let label_states: Vec<&str> = LABEL_FOR_STATE.iter().map(|(state, _)| *state).collect();
        let mut wanted: Vec<&str> = Vec::new();
        if let Some(found) = one_of(state, &label_states)? {
            if let Some((_, label)) = LABEL_FOR_STATE.iter().find(|(state, _)| *state == found) {
                wanted.push(label);
            }
        }
        let waived = get(result, "waived")?.is_some_and(PyValue::truthy);
        if waived && !wanted.contains(&WAIVED_LABEL) {
            wanted.push(WAIVED_LABEL);
        }
        let managed: Vec<&str> = STATE_LABELS
            .iter()
            .chain(RETIRED_LABELS.iter())
            .copied()
            .chain([WAIVED_LABEL])
            .collect();
        let mut worn: Vec<String> = Vec::new();
        for label in iterate(labels)? {
            if let Some(found) = one_of(&label, &managed)? {
                if !worn.iter().any(|known| known == found) {
                    worn.push(found.to_owned());
                }
            }
        }
        let label_correct = worn.len() == wanted.len()
            && wanted
                .iter()
                .all(|label| worn.iter().any(|known| known == label));
        // The description carries the checklist; a draft's does not (its
        // author is still writing it, and nobody reviews a draft). A
        // description with no room for it is left alone, and not fought
        // over on every run.
        let body_now = match or(get(pr, "body")?, &PyValue::None) {
            PyValue::None => s(""),
            body => body.clone(),
        };
        let block = if py_eq_str(state, "draft") {
            None
        } else {
            checklist_block(result)?
        };
        let current = current_checklist(&body_now)?;
        let fits = match &block {
            None => true,
            Some(block) => {
                // `len(body) - len(current) + len(block) + 2 <= limit`, with
                // nothing subtracted: the block now is part of the body.
                let body_length = py_len(text(&body_now, "body")?);
                let current_length = current.as_deref().map_or(0, py_len);
                body_length + py_len(block) + 2 <= BODY_LIMIT + current_length
            }
        };
        let stale_block = block.is_none() && current.is_some();
        let block_correct = !stale_block
            && match &block {
                None => true,
                Some(block) => !fits || same(current.as_deref(), Some(block)),
            };
        // The record lives in the engine's newest comment, whichever kind.
        // It is refreshed there silently whenever what it holds has changed
        // — the state, the head, the admission, the waiting time — never by
        // a new comment: a new comment is a notification, and a record is
        // not news. The holder is the comment the record was read from, the
        // one written last, not the one created last.
        let holders = bot_comments(pr, STATE_MARKER)?;
        let read_from = get_or(pr, "controller_comment_id", &NONE)?;
        let mut holder = None;
        for comment in &holders {
            if py_eq(getitem(comment, "id")?, read_from) {
                holder = Some(*comment);
                break;
            }
        }
        let holder = holder.or_else(|| holders.last().copied());
        let empty = PyValue::Dict(PyDict::new());
        let recorded = or(get(pr, "controller_state")?, &empty);
        let mut record_correct = true;
        if holder.is_some() {
            for key in ["state", "head", "admitted_at", "ready_since"] {
                if !py_eq(get_or(recorded, key, &NONE)?, get_or(&desired, key, &NONE)?) {
                    record_correct = false;
                    break;
                }
            }
        }
        // The diff beside the record is what tells the next run whether a
        // push was new work. A run that leaves it behind teaches the next
        // one nothing.
        if record_correct && holder.is_some() {
            let kept = or(get(pr, "controller_diff")?, &NONE);
            let now = diff_record(pr, result)?.filter(PyValue::truthy);
            record_correct = py_eq(kept, now.as_ref().unwrap_or(&PyValue::None));
        }
        let mv = move_state(state)?;
        // Nobody is told its move where there is nobody to take it.
        let move_body = match mv {
            Some(mv) if !(machine_author(policy, pr)? && mv == "waiting-self-review") => {
                move_text(result)?
            }
            _ => None,
        };
        // An announcement is made when the move passes to somebody, and is
        // kept current in place while it stays with them. Two things take
        // it out of their hands: the move going to someone else and coming
        // back, and a bot reporting after they were told while the pull
        // request waits on them to answer something. Editing through
        // either leaves the person with nothing in their inbox. A state
        // nobody is asked to act on — a build re-run, a permission read that
        // failed — is not a change of hands and must not repost, and neither
        // is a bot reporting again with nothing to answer: that takes no
        // attestation back, and on a ready pull request a repost would
        // notify every reviewer it asks.
        let was = move_state(get_or(recorded, "state", &NONE)?)?;
        let announcement = format!(
            "{MOVE_MARKER} state={} sha={} -->",
            mv.unwrap_or("None"),
            shown(getitem(pr, "head")?)
        );
        let movers = bot_comments(pr, MOVE_MARKER)?;
        let mut announced = Vec::new();
        for comment in &movers {
            if py_contains(getitem(comment, "body")?, &s(announcement.as_str()))? {
                announced.push(*comment);
            }
        }
        let same_hand = was == mv || was.is_none();
        let fresh = match announced.last() {
            None => true,
            Some(last) => {
                let completed = get_or(result, "bot_completed_at", &NONE)?;
                !py_eq_str(state, "waiting-author")
                    || !completed.truthy()
                    || py_compare(getitem(last, "created_at")?, Compare::Ge, completed)?
            }
        };
        let mut target = if same_hand && fresh {
            announced.last().copied()
        } else {
            None
        };
        // A standing comment of the earlier engine that already recorded
        // this move for this head is that announcement: it becomes the move
        // comment in place.
        let mut standing = Vec::new();
        for comment in &holders {
            if !py_contains(getitem(comment, "body")?, &s(MOVE_MARKER))? {
                standing.push(*comment);
            }
        }
        if target.is_none()
            && move_body.as_deref().is_some_and(|body| !body.is_empty())
            && !standing.is_empty()
            && py_eq(get_or(recorded, "state", &NONE)?, state)
            && py_eq(get_or(recorded, "head", &NONE)?, getitem(pr, "head")?)
        {
            target = standing.last().copied();
        }
        let move_correct = match (&move_body, target) {
            (None, _) => true,
            (Some(_), None) => false,
            (Some(body), Some(target)) => same(
                Some(&visible(text(getitem(target, "body")?, "body")?)),
                Some(body),
            ),
        };
        let standing_correct = standing.is_empty()
            || (standing.len() == 1
                && movers.is_empty()
                && same(
                    Some(&visible(text(getitem(standing[0], "body")?, "body")?)),
                    Some(POINTER),
                ));
        if block_correct
            && label_correct
            && missing.is_empty()
            && move_correct
            && record_correct
            && standing_correct
        {
            // Read current evidence on every run, but avoid churning the
            // description, labels and comments.
            if actionable {
                self.finish(&publication, candidates)?;
            } else {
                self.post_verdict(&head, result)?;
            }
            return Ok(None);
        }

        self.api
            .post_status(&head, "pending", "Evaluating current review policy", None)?;
        if !self.identity_matches(pr)? {
            return Ok(None);
        }
        if stale_block {
            if let Err(error) = except_github(self.api.remove_checklist(&n))? {
                self.say(format!("PR #{n}: stale checklist not removed: {error}"));
            }
            if !self.identity_matches(pr)? {
                return Ok(Some(desired));
            }
        }
        if let Some(block) = block.as_deref().filter(|_| !block_correct) {
            // The description is the author's; the check and the labels
            // still carry the state, and one refused write must not mark
            // the pull request an error.
            if let Err(error) = except_github(self.api.set_checklist(&n, block))? {
                self.say(format!("PR #{n}: checklist not written: {error}"));
            }
            if !self.identity_matches(pr)? {
                return Ok(Some(desired));
            }
        }
        let mut written = false;
        let mut kept: Option<PyValue> = None;
        let carried = diff_record(pr, result)?;
        if let (Some(body), false) = (&move_body, move_correct) {
            kept = match target {
                Some(target) => Some(getitem(target, "id")?.clone()),
                None => None,
            };
            self.api
                .upsert_state(&n, &desired, body, kept.as_ref(), carried.as_ref())?;
            written = true;
        } else if let Some(holder) = holder.filter(|_| !record_correct || !standing_correct) {
            // Same words, current record — carried by the announcement whose
            // words match this state when there is one, so the record never
            // sits under the wrong instruction; otherwise by the holder. The
            // earlier engine's standing comment gets the pointer at the
            // description. The move comment is the carrier exactly when
            // there are words for this state and a comment that says them.
            let announces = move_body.is_some() && target.is_some();
            let carrier = match target {
                Some(target) if announces => target,
                _ => holder,
            };
            let carrier_body = text(getitem(carrier, "body")?, "body")?;
            let mut kept_words = visible(carrier_body);
            if kept_words.contains(STATE_MARKER) || kept_words.contains(DIFF_MARKER) {
                // Unreadable words are not worth a wedged pull request.
                kept_words = POINTER.to_owned();
            }
            let words = match &move_body {
                Some(body) if announces => body.clone(),
                _ if carrier_body.contains(MOVE_MARKER) => kept_words,
                _ => POINTER.to_owned(),
            };
            kept = Some(getitem(carrier, "id")?.clone());
            self.api
                .upsert_state(&n, &desired, &words, kept.as_ref(), carried.as_ref())?;
            written = true;
        }
        // Every standing comment that is not the record holder is noise,
        // and goes: once a move comment exists, all of them; before that,
        // all but the one carrying the record.
        if kept.is_none() && !written && movers.is_empty() {
            if let Some(holder) = holder {
                kept = Some(getitem(holder, "id")?.clone());
            }
        }
        let mut superseded = Vec::new();
        for comment in &standing {
            let id = getitem(comment, "id")?;
            if !kept.as_ref().is_some_and(|kept| py_eq(id, kept)) {
                superseded.push(id.clone());
            }
        }
        for old in &superseded {
            if except_github(self.api.delete_comment(old))?.is_err() {
                self.say(format!("PR #{n}: could not remove an old state comment"));
            }
        }
        if (written || !superseded.is_empty()) && !self.identity_matches(pr)? {
            return Ok(Some(desired));
        }
        let labels = get_or(pr, "labels", &EMPTY_LIST)?;
        if except_github(self.api.set_state_label(&n, state, labels))?.is_err() {
            // The labels were read from a snapshot that another run
            // reconciling this author can invalidate, and removing a label
            // that is already gone is a 404. The state is in the status and
            // the description either way.
            self.say(format!(
                "PR #{n}: could not set the state label; the status and description still carry the state"
            ));
        }
        if except_github(self.api.set_label(&n, WAIVED_LABEL, waived, labels))?.is_err() {
            // The repository may not have the label yet. The waiver is
            // already in the status and the description, and one missing
            // label must not abort the remaining pull requests.
            self.say(format!(
                "PR #{n}: could not set {WAIVED_LABEL}; create the label to see waivers in listings"
            ));
        }
        if !missing.is_empty() {
            let room = REVIEWER_CAP.saturating_sub(requested.len());
            if missing.len() > room {
                self.say(format!(
                    "PR #{n}: reviewer request capacity reached; {} request(s) deferred",
                    missing.len() - room
                ));
            }
            if room > 0 {
                if !self.identity_matches(pr)? {
                    return Ok(Some(desired));
                }
                let asked = &missing[..room.min(missing.len())];
                if except_github(self.api.request_reviewers(&n, asked))?.is_err() {
                    // The room left over is read from a snapshot that another
                    // run reconciling this author can invalidate, and GitHub
                    // refuses a request past its own cap. The reviewers are
                    // already named in the description and the status.
                    let names: Vec<String> = asked.iter().map(shown).collect();
                    self.say(format!(
                        "PR #{n}: could not request {}; the description still names them",
                        names.join(", ")
                    ));
                }
            }
        }

        if !actionable {
            if self.identity_matches(pr)? {
                self.post_verdict(&head, result)?;
            }
            return Ok(Some(desired));
        }
        // Review decisions and comments can change without changing the
        // commit, so the admission re-check expects this pull request's own
        // new record.
        let mut expected = Vec::with_capacity(candidates.len());
        for candidate in candidates {
            if py_eq(getitem(candidate, "number")?, getitem(pr, "number")?) {
                let PyValue::Dict(fields) = candidate else {
                    return Err(ReadError::NotPorted(
                        "a candidate that is not a dict".into(),
                    ));
                };
                let mut entry: PyDict =
                    fields.iter().map(|(k, v)| (k.clone(), v.clone())).collect();
                entry.insert("controller_state".into(), desired.clone());
                expected.push(PyValue::Dict(entry));
            } else {
                expected.push(candidate.clone());
            }
        }
        self.finish(&publication, &expected)?;
        Ok(Some(desired))
    }
}
