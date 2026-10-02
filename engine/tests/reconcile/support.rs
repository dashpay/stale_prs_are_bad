//! What the reconcile tests share: values written as JSON, a clock that
//! logs where it is read, and a transport that counts the calls made.

pub use pr_hygiene_engine::evidence::{
    Call, Client, GitHub, NoSleep, ReadError, Reply, Transport, TransportError,
};
pub use pr_hygiene_engine::pycompat::{py_dumps, py_loads, PyDateTime, PyInt, PyValue};
pub use pr_hygiene_engine::reconcile::{Clock, ClockSite};
pub use serde_json::{json, Value};
use std::cell::{Cell, RefCell};
use std::rc::Rc;

/// A value as compact JSON in its own key order: how `conformance.py`
/// compares a function's answer, and the strictest way to compare two.
pub fn dump(value: &PyValue) -> String {
    py_dumps(value, false, Some((",", ":")), None).expect("no floats in a compared value")
}

/// A field of a dict.
#[track_caller]
pub fn field<'a>(value: &'a PyValue, key: &str) -> &'a PyValue {
    match value {
        PyValue::Dict(entries) => entries
            .get(key)
            .unwrap_or_else(|| panic!("no {key} in {}", dump(value))),
        _ => panic!("not a dict: {}", dump(value)),
    }
}

/// A list's items.
#[track_caller]
pub fn items(value: &PyValue) -> &[PyValue] {
    match value {
        PyValue::List(items) => items,
        _ => panic!("not a list: {}", dump(value)),
    }
}

/// A string's text.
#[track_caller]
pub fn text(value: &PyValue) -> &str {
    match value {
        PyValue::Str(s) => s,
        _ => panic!("not a string: {}", dump(value)),
    }
}

/// How many calls have crossed a transport, shared with whoever needs to
/// know: the clock logs a read against it.
pub type Calls = Rc<Cell<usize>>;

/// Every read of a clock: where, and how many calls had been made by then.
pub type Reads = Rc<RefCell<Vec<(String, usize)>>>;

/// A clock stopped at one instant that logs every read, as Python's
/// recorder does: the site, and the ordinal of the last call before it.
pub struct LoggingClock {
    pub at: PyDateTime,
    pub calls: Calls,
    pub reads: Reads,
}

impl LoggingClock {
    pub fn new(at: &str, calls: Calls) -> Self {
        LoggingClock {
            at: PyDateTime::fromisoformat(&at.replace('Z', "+00:00")).expect("an instant"),
            calls,
            reads: Rc::default(),
        }
    }

    /// Log a read the engine makes outside its own clock: the hourly batch
    /// a caller chooses.
    pub fn log(&self, site: &str) {
        self.reads
            .borrow_mut()
            .push((site.to_owned(), self.calls.get()));
    }
}

impl Clock for LoggingClock {
    fn now(&mut self, site: ClockSite) -> PyDateTime {
        self.log(site.as_str());
        self.at
    }
}

/// `conformance/`, where the corpus lives.
pub fn conformance() -> std::path::PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../conformance")
}

/// A file read as Python's JSON reader reads it.
pub fn read_json(path: &std::path::Path) -> PyValue {
    let text = std::fs::read_to_string(path).unwrap_or_else(|e| panic!("{}: {e}", path.display()));
    py_loads(&text).unwrap_or_else(|e| panic!("{}: {e}", path.display()))
}

/// `conformance/pending.txt`: one case path per line, relative to
/// `conformance/`; blank lines and `#` comments ignored.
pub fn pending() -> std::collections::BTreeSet<String> {
    let path = conformance().join("pending.txt");
    std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("{}: {e}", path.display()))
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty() && !line.starts_with('#'))
        .map(str::to_owned)
        .collect()
}
