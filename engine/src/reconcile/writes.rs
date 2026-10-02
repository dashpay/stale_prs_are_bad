//! The writes of `pr_review/github.py`, beside the reads and caches of the
//! same [`GitHub`]: commit statuses, the record comment, comments, the
//! checklist in the description, labels and reviewer requests.

use super::values::s;
use crate::evidence::records::{split_checklist, state_comment_body, validate_state};
use crate::evidence::{GitHub, Method, ReadError, Transport};
use crate::policy::{
    is_engine, CHECKLIST_END, CHECKLIST_START, LABEL_FOR_STATE, RETIRED_LABELS, STATE_LABELS,
};
use crate::pycompat::object::{get, get_or, or, str_method, EMPTY_DICT, NONE};
use crate::pycompat::ops::{py_contains, py_eq};
use crate::pycompat::text::{py_len, py_rstrip, py_slice, py_strip};
use crate::pycompat::urllib::py_quote;
use crate::pycompat::{PyDict, PyInt, PyList, PyValue};

/// The context of the engine's commit status.
pub const STATUS_CONTEXT: &str = "PR Hygiene";
/// The longest description GitHub keeps on a status, in characters.
const STATUS_DESCRIPTION: isize = 140;
/// The longest description GitHub keeps on a pull request, in characters.
pub(crate) const BODY_LIMIT: usize = 65536;

/// `_normalise(text)`: line endings and trailing space set aside, for a
/// comparison only — the web form re-saves whole bodies as CRLF.
fn normalise(text: &str) -> String {
    py_rstrip(&text.replace("\r\n", "\n")).to_owned()
}

/// `(current.get("body") or "") if isinstance(current, dict) else ""`.
fn body_of(current: &PyValue) -> Result<&str, ReadError> {
    match current {
        PyValue::Dict(_) => match or(get(current, "body")?, &PyValue::None) {
            PyValue::None => Ok(""),
            body => Ok(str_method(body, "splitlines")?),
        },
        _ => Ok(""),
    }
}

impl<T: Transport> GitHub<T> {
    fn root(&self) -> String {
        format!("repos/{}", self.repo())
    }

    fn write(
        &mut self,
        method: Method,
        path: String,
        payload: Option<PyValue>,
    ) -> Result<PyValue, ReadError> {
        self.client_mut().request(method, &path, payload)
    }

    /// `GitHub.post_status(head, state, description, target_url)`: the
    /// engine's commit status on `head`, unless the newest one there is
    /// already the engine's and says the same.
    ///
    /// The status posted goes into the head's cache under whoever GitHub
    /// says wrote it, so the cache never claims an identity the engine did
    /// not post under, and the next status for the same head is checked
    /// against it without reading the commit's whole history again.
    pub fn post_status(
        &mut self,
        head: &str,
        state: &str,
        description: &str,
        target_url: Option<&str>,
    ) -> Result<PyValue, ReadError> {
        if !["pending", "success", "failure", "error"].contains(&state) {
            return Err(ReadError::GitHub("Invalid commit status state".into()));
        }
        let mut payload = PyDict::new();
        payload.insert("state".into(), s(state));
        payload.insert("context".into(), s(STATUS_CONTEXT));
        payload.insert(
            "description".into(),
            s(py_slice(description, None, Some(STATUS_DESCRIPTION))),
        );
        if let Some(url) = target_url {
            payload.insert("target_url".into(), s(url));
        }
        let statuses = self.head_statuses_mut(head)?;
        let mut latest = None;
        for item in statuses.iter() {
            if get(item, "context")?.is_some_and(|context| py_eq(context, &s(STATUS_CONTEXT))) {
                latest = Some(item);
                break;
            }
        }
        if let Some(latest) = latest.filter(|latest| latest.truthy()) {
            let creator = or(get(latest, "creator")?, &EMPTY_DICT);
            if is_engine(get(creator, "login")?)? {
                let mut same = true;
                for key in ["state", "description", "target_url"] {
                    let wanted = payload.get(key).unwrap_or(&PyValue::None);
                    if !py_eq(get_or(latest, key, &NONE)?, wanted) {
                        same = false;
                        break;
                    }
                }
                if same {
                    return Ok(latest.clone());
                }
            }
        }
        let path = format!("{}/statuses/{}", self.root(), py_quote(head));
        let written = self.write(Method::Post, path, Some(PyValue::Dict(payload.clone())))?;
        if !matches!(written, PyValue::Dict(_)) {
            return Err(ReadError::GitHub(
                "Commit status was not acknowledged".into(),
            ));
        }
        let mut cached = payload;
        cached.insert(
            "creator".into(),
            or(get(&written, "creator")?, &EMPTY_DICT).clone(),
        );
        cached.insert(
            "created_at".into(),
            get_or(&written, "created_at", &NONE)?.clone(),
        );
        self.head_statuses_mut(head)?
            .insert(0, PyValue::Dict(cached));
        Ok(written)
    }

    /// `GitHub.upsert_state(number, state, body, comment_id, diff)`: the
    /// record comment, posted anew or written over `comment_id`. The id of
    /// the comment written.
    pub fn upsert_state(
        &mut self,
        number: &PyInt,
        state: &PyValue,
        body: &str,
        comment_id: Option<&PyValue>,
        diff: Option<&PyValue>,
    ) -> Result<PyValue, ReadError> {
        validate_state(state)?;
        let numbered = PyValue::Int(number.clone());
        if !py_eq(get_or(state, "number", &NONE)?, &numbered) {
            return Err(ReadError::GitHub(
                "Controller state belongs to another PR".into(),
            ));
        }
        if let Some(diff) = diff {
            if !py_eq(crate::pycompat::object::getitem(diff, "number")?, &numbered) {
                return Err(ReadError::GitHub(
                    "Controller diff belongs to another PR".into(),
                ));
            }
        }
        let mut payload = PyDict::new();
        payload.insert("body".into(), s(state_comment_body(state, body, diff)?));
        let root = self.root();
        let result = match comment_id {
            None => self.write(
                Method::Post,
                format!("{root}/issues/{number}/comments"),
                Some(PyValue::Dict(payload)),
            )?,
            Some(id) => self.write(
                Method::Patch,
                format!("{root}/issues/comments/{}", super::values::shown(id)),
                Some(PyValue::Dict(payload)),
            )?,
        };
        match get(&result, "id") {
            Ok(Some(id @ PyValue::Int(_))) => Ok(id.clone()),
            _ => Err(ReadError::GitHub(
                "State comment write returned no identity".into(),
            )),
        }
    }

    /// `GitHub.comment(number, body)`: a new comment.
    pub fn comment(&mut self, number: &PyInt, body: &str) -> Result<PyValue, ReadError> {
        let path = format!("{}/issues/{number}/comments", self.root());
        self.write(Method::Post, path, Some(body_payload(body)))
    }

    /// `GitHub.edit_comment(comment_id, body)`.
    pub fn edit_comment(&mut self, comment_id: &PyValue, body: &str) -> Result<PyValue, ReadError> {
        let path = format!(
            "{}/issues/comments/{}",
            self.root(),
            super::values::shown(comment_id)
        );
        self.write(Method::Patch, path, Some(body_payload(body)))
    }

    /// `GitHub.delete_comment(comment_id)`.
    pub fn delete_comment(&mut self, comment_id: &PyValue) -> Result<PyValue, ReadError> {
        let path = format!(
            "{}/issues/comments/{}",
            self.root(),
            super::values::shown(comment_id)
        );
        self.write(Method::Delete, path, None)
    }

    /// `GitHub.remove_checklist(number)`: the engine's block out of the
    /// description, and nothing else. Whether there was one to take out.
    pub fn remove_checklist(&mut self, number: &PyInt) -> Result<bool, ReadError> {
        let path = format!("{}/pulls/{number}", self.root());
        let current = self.write(Method::Get, path.clone(), None)?;
        let body = body_of(&current)?;
        let (head, block, tail) = split_checklist(body);
        if block.is_none() {
            return Ok(false);
        }
        let kept = py_rstrip(&format!("{}{tail}", py_rstrip(head))).to_owned();
        self.write(Method::Patch, path, Some(body_payload(&kept)))?;
        Ok(true)
    }

    /// `GitHub.set_checklist(number, block)`: the engine's block at the end
    /// of the description, and nothing else. Whether it was written.
    ///
    /// The description is the author's. Only the block between the markers
    /// is the engine's to write: read the body immediately before the
    /// write, replace the last marker pair or append, and leave everything
    /// else byte for byte — whatever follows the block is someone else's,
    /// CodeRabbit appends its own, and stays where it was.
    pub fn set_checklist(&mut self, number: &PyInt, block: &str) -> Result<bool, ReadError> {
        let start = CHECKLIST_START.len() as isize;
        let end = CHECKLIST_END.len() as isize;
        if py_slice(block, Some(start), None).contains(CHECKLIST_START)
            || py_slice(block, None, Some(-end)).contains(CHECKLIST_END)
        {
            return Err(ReadError::GitHub(
                "Checklist block must not contain its own delimiters".into(),
            ));
        }
        let path = format!("{}/pulls/{number}", self.root());
        let current = self.write(Method::Get, path.clone(), None)?;
        let body = body_of(&current)?.to_owned();
        let (head, _, tail) = split_checklist(&body);
        let wanted = if py_strip(head).is_empty() {
            format!("{block}{tail}")
        } else {
            format!("{}\n\n{block}{tail}", py_rstrip(head))
        };
        if py_len(&wanted) > BODY_LIMIT {
            return Err(ReadError::GitHub(
                "Description too long for the checklist".into(),
            ));
        }
        if normalise(&body) == normalise(&wanted) {
            return Ok(false);
        }
        self.write(Method::Patch, path, Some(body_payload(&wanted)))?;
        Ok(true)
    }

    /// `GitHub.set_label(number, label, enabled, current_labels)`: the label
    /// on or off, asking only where that changes something.
    pub fn set_label(
        &mut self,
        number: &PyInt,
        label: &str,
        enabled: bool,
        current_labels: &PyValue,
    ) -> Result<PyValue, ReadError> {
        let worn = py_contains(current_labels, &s(label))?;
        let root = self.root();
        if enabled && !worn {
            return self.write(
                Method::Post,
                format!("{root}/issues/{number}/labels"),
                Some(labels_payload(label)),
            );
        }
        if !enabled && worn {
            return self.write(
                Method::Delete,
                format!("{root}/issues/{number}/labels/{label}"),
                None,
            );
        }
        Ok(PyValue::None)
    }

    /// `GitHub.set_state_label(number, state, current_labels)`: exactly one
    /// state label, or none for a state that has none, and the names the
    /// engine used to set cleared. The new label goes on first, so a refused
    /// removal never leaves the pull request without one.
    pub fn set_state_label(
        &mut self,
        number: &PyInt,
        state: &PyValue,
        current_labels: &PyValue,
    ) -> Result<(), ReadError> {
        let names: Vec<&str> = LABEL_FOR_STATE.iter().map(|(state, _)| *state).collect();
        let wanted = super::values::one_of(state, &names)?
            .and_then(|found| LABEL_FOR_STATE.iter().find(|(state, _)| *state == found))
            .map(|(_, label)| *label);
        let root = self.root();
        if let Some(wanted) = wanted {
            if !py_contains(current_labels, &s(wanted))? {
                self.write(
                    Method::Post,
                    format!("{root}/issues/{number}/labels"),
                    Some(labels_payload(wanted)),
                )?;
            }
        }
        let mut seen = Vec::new();
        for label in STATE_LABELS.iter().chain(RETIRED_LABELS.iter()) {
            if seen.contains(label) {
                continue;
            }
            seen.push(label);
            if py_contains(current_labels, &s(*label))? && Some(*label) != wanted {
                self.write(
                    Method::Delete,
                    format!("{root}/issues/{number}/labels/{label}"),
                    None,
                )?;
            }
        }
        Ok(())
    }

    /// `GitHub.request_reviewers(number, users)`.
    pub fn request_reviewers(
        &mut self,
        number: &PyInt,
        users: &[PyValue],
    ) -> Result<PyValue, ReadError> {
        if users.is_empty() {
            return Ok(PyValue::None);
        }
        let mut payload = PyDict::new();
        payload.insert(
            "reviewers".into(),
            PyValue::List(PyList::from(users.to_vec())),
        );
        let path = format!("{}/pulls/{number}/requested_reviewers", self.root());
        self.write(Method::Post, path, Some(PyValue::Dict(payload)))
    }
}

/// `{"body": body}`.
fn body_payload(body: &str) -> PyValue {
    let mut payload = PyDict::new();
    payload.insert("body".into(), s(body));
    PyValue::Dict(payload)
}

/// `{"labels": [label]}`.
fn labels_payload(label: &str) -> PyValue {
    let mut payload = PyDict::new();
    payload.insert("labels".into(), PyValue::List(PyList::from(vec![s(label)])));
    PyValue::Dict(payload)
}
