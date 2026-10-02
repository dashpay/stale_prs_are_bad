//! The re-checks a publication makes before it writes, and before the
//! status a success needs: `identity_matches`, `admission_valid` and
//! `finish` of `main.publish`.

use super::publish::Publication;
use super::state::{admission_fingerprint, context_fingerprint, same_admissions};
use super::values::{list, lower, number, s, text};
use super::{ClockSite, Reconciler};
use crate::evidence::{ReadError, Transport};
use crate::policy::{admit, evaluate, fingerprint, governs};
use crate::pycompat::object::{get_or, getitem, NONE};
use crate::pycompat::ops::{py_eq, py_eq_str};
use crate::pycompat::PyValue;

/// The fields of a pull request whose change under a run means its
/// verdict no longer applies.
const IDENTITY: [&str; 5] = ["head", "base", "base_sha", "draft", "state"];

impl<T: Transport> Reconciler<'_, T> {
    /// `identity_matches()`: the pull request read again, and its head,
    /// base, draft state and open state as the verdict saw them.
    pub(super) fn identity_matches(&mut self, pr: &PyValue) -> Result<bool, ReadError> {
        let current = self.api.pull(&number(pr)?)?;
        for key in IDENTITY {
            if !py_eq(get_or(&current, key, &NONE)?, get_or(pr, key, &NONE)?) {
                return Ok(false);
            }
        }
        Ok(true)
    }

    /// `admission_valid(expected)`: whether this author's admission is still
    /// what the verdict was decided under — the same pull requests open,
    /// the same admissions recorded, and this one's slot the same.
    pub(super) fn admission_valid(
        &mut self,
        publication: &Publication<'_>,
        expected: &[PyValue],
    ) -> Result<bool, ReadError> {
        let Publication {
            policy, pr, result, ..
        } = *publication;
        let current_prs = self.api.open_prs()?;
        let author = getitem(pr, "author")?;
        if context_fingerprint(&current_prs, author)? != publication.context {
            return Ok(false);
        }
        // Only this author's histories can change this pull request's
        // admission.
        let mut relevant = Vec::new();
        for p in &current_prs {
            if governs(policy, getitem(p, "base")?)?
                && lower(getitem(p, "author")?)? == lower(author)?
            {
                relevant.push(p.clone());
            }
        }
        let histories = self.load_histories(&relevant)?;
        let mut baseline = Vec::new();
        for p in expected {
            if lower(getitem(p, "author")?)? == lower(author)? {
                baseline.push(p);
            }
        }
        if !same_admissions(
            &admission_fingerprint(&histories)?,
            &admission_fingerprint(baseline)?,
        ) {
            return Ok(false);
        }
        let admitted_at = get_or(result, "admitted_at", &NONE)?;
        let now = if admitted_at.truthy() {
            admitted_at.clone()
        } else {
            s(self.utc_now(ClockSite::AdmissionValid))
        };
        let slots = admit(policy, &list(histories), &now)?;
        let slot = slots.get(&number(pr)?).unwrap_or(&PyValue::None);
        Ok(py_eq(slot, admitted_at))
    }

    /// `finish(expected)`: the status a success needs, posted only after
    /// the admission is checked again and the evidence read afresh, with
    /// every cache dropped, and decided the same. Admission history reads
    /// can be slow, so the evidence is read after them, and a dismissed
    /// approval is not reused from before them.
    pub(super) fn finish(
        &mut self,
        publication: &Publication<'_>,
        expected: &[PyValue],
    ) -> Result<(), ReadError> {
        let Publication {
            policy, pr, result, ..
        } = *publication;
        let valid_admission = self.admission_valid(publication, expected)?;
        self.api.forget_cached_access();
        let n = number(pr)?;
        let last = self.api.snapshot(&n, policy, None)?;
        let head = text(getitem(pr, "head")?, "head")?.to_owned();
        if !valid_admission || fingerprint(&last)? != fingerprint(pr)? {
            self.api.post_status(
                &head,
                "pending",
                "Review evidence changed; reconciliation required",
                None,
            )?;
            return Ok(());
        }
        let now = s(self.utc_now(ClockSite::Finish));
        let check = evaluate(
            policy,
            &last,
            get_or(result, "admitted_at", &NONE)?,
            &now,
            &PyValue::None,
        )?;
        let succeeded = || -> Result<bool, crate::pycompat::PyErr> {
            let status = check
                .get("status")
                .ok_or_else(|| crate::pycompat::PyErr::Key("status".into()))?;
            Ok(py_eq_str(status, "success"))
        };
        if py_eq_str(getitem(result, "status")?, "success") && !succeeded()? {
            self.api.post_status(
                &head,
                "pending",
                "Policy changed; reconciliation required",
                None,
            )?;
            return Ok(());
        }
        self.post_verdict(&head, result)
    }

    /// `api.post_status(pr['head'], result['status'], result['state'])`.
    pub(super) fn post_verdict(&mut self, head: &str, result: &PyValue) -> Result<(), ReadError> {
        // `post_status` refuses a state it does not know before it reads
        // the description, as Python's does.
        let PyValue::Str(status) = getitem(result, "status")? else {
            return Err(ReadError::GitHub("Invalid commit status state".into()));
        };
        let state = text(getitem(result, "state")?, "state")?;
        self.api.post_status(head, status, state, None)?;
        Ok(())
    }
}
