//! What the evidence tests share: values written as JSON, answers as `gh`
//! gives them, and a transport that routes each call to its answer the way
//! the Python tests patch `GitHub.request` and `GitHub.pages`.

pub use pr_hygiene_engine::evidence::replay::transient;
pub use pr_hygiene_engine::evidence::{
    Call, Client, Failure, FailureClass, FromFn, GitHub, History, Method, NoSleep, ReadError,
    Reply, Transport, TransportError,
};
pub use pr_hygiene_engine::pycompat::ops::py_eq;
pub use pr_hygiene_engine::pycompat::{py_dumps, py_loads, PyInt, PyValue};
pub use serde_json::{json, Value};

pub type Answer = Result<Reply, TransportError>;

/// A value written as JSON, read as Python reads it.
pub fn py(value: Value) -> PyValue {
    py_loads(&value.to_string()).expect("serde_json writes JSON")
}

pub fn int(n: i64) -> PyInt {
    PyInt::from(n)
}

/// A value as compact JSON with sorted keys, for a message.
pub fn shown(value: &PyValue) -> String {
    py_dumps(value, true, Some((",", ":")), None).expect("no floats in a test value")
}

/// Python's `==` between what was read and what was expected: maps by their
/// entries, lists by their order.
#[track_caller]
pub fn assert_py(actual: &PyValue, expected: Value) {
    assert!(
        py_eq(actual, &py(expected.clone())),
        "\n  read: {}\nwanted: {expected}",
        shown(actual)
    );
}

/// `other` merged over `base`, as `dict(base, **other)`.
pub fn merged(base: &Value, other: Value) -> Value {
    let mut out = base.clone();
    if let (Value::Object(into), Value::Object(from)) = (&mut out, other) {
        into.extend(from);
    }
    out
}

/// A successful answer: `gh` printed this JSON.
pub fn ok(value: Value) -> Answer {
    Ok(Reply::Text(value.to_string()))
}

/// A successful paginated answer: one page holding these items, as
/// `--paginate --slurp` prints it.
pub fn page(items: Value) -> Answer {
    Ok(Reply::Text(json!([items]).to_string()))
}

/// `gh` exited `exit`, having printed `stdout` and said `stderr`; transient
/// by Python's rule.
pub fn failed(exit: i32, stdout: &str, stderr: &str) -> Answer {
    Err(TransportError::Failed(Failure {
        transient: transient(stderr, stdout),
        status: Some(exit),
        body: stdout.to_owned(),
        detail: stderr.to_owned(),
        class: FailureClass::Unclassed,
    }))
}

/// A transport that answers through a routing function and keeps every
/// call it was asked.
pub struct Fake {
    route: Box<dyn FnMut(&Call) -> Answer>,
    pub calls: Vec<Call>,
}

impl Transport for Fake {
    fn call(&mut self, call: &Call) -> Answer {
        self.calls.push(call.clone());
        (self.route)(call)
    }
}

/// `GitHub("dashpay/platform")` over a routing function; a retry does not
/// wait.
pub fn api(route: impl FnMut(&Call) -> Answer + 'static) -> GitHub<Fake> {
    let fake = Fake {
        route: Box::new(route),
        calls: Vec::new(),
    };
    GitHub::new("dashpay/platform", Client::with_sleep(fake, NoSleep)).expect("a valid repository")
}

/// The same reader with its routing function replaced and its calls
/// forgotten, keeping its caches: what `patch.object` does between two
/// reads of one reconciliation.
pub fn reroute(api: &mut GitHub<Fake>, route: impl FnMut(&Call) -> Answer + 'static) {
    let fake = api.client_mut().transport_mut();
    fake.route = Box::new(route);
    fake.calls.clear();
}

pub fn calls(api: &GitHub<Fake>) -> &[Call] {
    &api.client().transport().calls
}

/// A REST call's path, or `graphql`.
pub fn path(call: &Call) -> &str {
    match call {
        Call::Rest { path, .. } => path,
        Call::Graphql { .. } => "graphql",
    }
}

/// A GraphQL call's document, or nothing.
pub fn query(call: &Call) -> &str {
    match call {
        Call::Graphql { query, .. } => query,
        Call::Rest { .. } => "",
    }
}

/// A GraphQL call's variable.
pub fn variable<'a>(call: &'a Call, name: &str) -> Option<&'a PyValue> {
    match call {
        Call::Graphql {
            variables: PyValue::Dict(variables),
            ..
        } => variables.get(name),
        _ => None,
    }
}

/// A variable as text, `None` for `null`.
pub fn variable_text(call: &Call, name: &str) -> Option<String> {
    match variable(call, name) {
        Some(PyValue::Str(s)) => Some(s.clone()),
        _ => None,
    }
}

/// A field of a read value.
#[track_caller]
pub fn field<'a>(value: &'a PyValue, key: &str) -> &'a PyValue {
    match value {
        PyValue::Dict(entries) => entries
            .get(key)
            .unwrap_or_else(|| panic!("no {key} in {}", shown(value))),
        _ => panic!("not a dict: {}", shown(value)),
    }
}

/// A list's items.
#[track_caller]
pub fn items(value: &PyValue) -> &[PyValue] {
    match value {
        PyValue::List(items) => items,
        _ => panic!("not a list: {}", shown(value)),
    }
}

/// A string's text.
#[track_caller]
pub fn text(value: &PyValue) -> &str {
    match value {
        PyValue::Str(s) => s,
        _ => panic!("not a string: {}", shown(value)),
    }
}

/// The message of a `GitHubError`, failing on anything else.
#[track_caller]
pub fn github_error<T: std::fmt::Debug>(result: Result<T, ReadError>) -> String {
    match result {
        Err(ReadError::GitHub(message)) => message,
        other => panic!("expected a GitHubError, got {other:?}"),
    }
}

/// `GitHub.state_comment_body(state, body, diff)`.
pub fn record_body(state: Value, body: &str, diff: Option<Value>) -> String {
    pr_hygiene_engine::evidence::records::state_comment_body(
        &py(state),
        body,
        diff.map(py).as_ref(),
    )
    .expect("a valid record")
}
