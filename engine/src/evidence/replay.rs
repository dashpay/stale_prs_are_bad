//! A transport that answers from a boundary recording: the `calls.jsonl`
//! that `pr_review/conformance.py` writes beside every recorded run.
//!
//! Python reached GitHub by running `gh api` with arguments and a body on
//! stdin, and the recording keeps each call under exactly those. So a call
//! is answered by the recording only if it becomes the very same `gh api`
//! command Python would have run for it — the same arguments, the same body
//! byte for byte, GraphQL documents and JSON separators included — which is
//! what holds the port's requests to Python's. Each recorded answer is
//! served once, in the order recorded; a call the recording does not hold,
//! or asked more often than it was recorded, is refused.
//!
//! A write is answered as the recorder answered it, and is held to the
//! next write the recording holds: the same method and route, and a body
//! that is the same JSON value, every string in it byte for byte. What
//! became of each is kept ([`WriteCheck`]). A write to the recorded route
//! with another body is answered as recorded all the same, so the run goes
//! on and every later write is compared too; a write to another route, or
//! one more than the recording holds, has no answer and is refused, naming
//! where the two differ.
//!
//! Whether a failure is worth one more try is decided here as Python
//! decided it: by looking for a gateway error, a timeout or a truncated
//! answer anywhere in what `gh` said.

use super::error::ReadError;
use super::py::compiled;
use super::transport::{Call, Failure, FailureClass, Method, Reply, Transport, TransportError};
use crate::pycompat::text::py_lstrip;
use crate::pycompat::{py_dumps, py_loads, PyDict, PyValue};
use indexmap::IndexMap;
use regex::Regex;
use std::collections::VecDeque;
use std::sync::LazyLock;

/// What `gh` said when a call failed, that makes it worth asking again.
const TRANSIENT_SIGNS: [&str; 6] = [
    "unexpected end of JSON input",
    "502",
    "503",
    "504",
    "timeout",
    "EOF",
];

/// `_transient`'s text rule: any sign anywhere in stderr followed by stdout.
pub fn transient(stderr: &str, stdout: &str) -> bool {
    let text = format!("{stderr}{stdout}");
    TRANSIENT_SIGNS.iter().any(|sign| text.contains(sign))
}

/// `json.dumps(value)`: Python's default separators, keys in the order the
/// value holds them, everything outside ASCII escaped.
fn python_json(value: &PyValue) -> Result<String, ReadError> {
    py_dumps(value, false, None, None)
        .map_err(|_| ReadError::NotPorted("a float in a request body".into()))
}

/// The arguments after `gh api`, and the text on stdin, that Python's
/// `GitHub.request` and `GitHub.pages` build for `call`.
pub fn gh_arguments(call: &Call) -> Result<(Vec<String>, Option<String>), ReadError> {
    match call {
        Call::Rest {
            method,
            path,
            body,
            paginate,
        } => {
            let mut arguments = vec!["--method".to_owned(), method.to_string(), path.clone()];
            if *paginate {
                arguments.extend(["--paginate".to_owned(), "--slurp".to_owned()]);
            }
            let stdin = match body {
                Some(body) => {
                    arguments.extend(["--input".to_owned(), "-".to_owned()]);
                    Some(python_json(body)?)
                }
                None => None,
            };
            Ok((arguments, stdin))
        }
        Call::Graphql { query, variables } => {
            let mut document = PyDict::new();
            document.insert("query".into(), PyValue::Str(query.clone()));
            document.insert("variables".into(), variables.clone());
            let arguments = ["--method", "POST", "graphql", "--input", "-"]
                .map(str::to_owned)
                .to_vec();
            Ok((arguments, Some(python_json(&PyValue::Dict(document))?)))
        }
    }
}

/// Python's `\w`, one character at a time.
static WORD: LazyLock<Regex> = LazyLock::new(|| compiled(r"\A\w\z"));

fn is_word(c: Option<char>) -> bool {
    c.is_some_and(|c| WORD.is_match(c.encode_utf8(&mut [0; 4])))
}

/// `re.search(r"\b(?:mutation|subscription)\b", query)`.
fn names_a_write(query: &str) -> bool {
    ["mutation", "subscription"].iter().any(|word| {
        query.match_indices(word).any(|(at, found)| {
            !is_word(query[..at].chars().next_back())
                && !is_word(query[at + found.len()..].chars().next())
        })
    })
}

/// `conformance.is_read`: a `GET` of a route under a repository with no
/// body, or a GraphQL document that is a query and nothing else. Anything
/// not proven a read is a write.
pub fn is_read(call: &Call) -> bool {
    static REPOSITORY_ROUTE: LazyLock<Regex> =
        LazyLock::new(|| compiled(r"\Arepos/[A-Za-z0-9_.-]+/[A-Za-z0-9_.-]+/"));
    match call {
        Call::Rest {
            method: Method::Get,
            path,
            body: None,
            ..
        } => REPOSITORY_ROUTE.is_match(path) && !path.contains("://") && !path.contains(".."),
        Call::Rest { .. } => false,
        Call::Graphql { query, .. } => {
            py_lstrip(query).starts_with("query") && !names_a_write(query)
        }
    }
}

/// One recorded answer.
#[derive(Debug, Clone)]
enum Recorded {
    /// `gh` ran and exited.
    Exited {
        exit: i32,
        stdout: String,
        stderr: String,
    },
    /// `gh` could not run, or ran out of its sixty seconds.
    Raised,
}

#[derive(Debug, Default)]
struct Answers {
    /// Each answer, with the ordinal of the call it answered.
    recorded: Vec<(usize, Recorded)>,
    served: usize,
}

/// One recorded write: the `gh api` command, and the answer the recorder
/// made for it.
#[derive(Debug)]
struct RecordedWrite {
    ordinal: usize,
    arguments: Vec<String>,
    stdin: Option<String>,
    stdout: String,
}

/// The `gh api` command a call becomes: its arguments and its stdin.
type Command = (Vec<String>, Option<String>);

/// Answers from a recording's reads, and checks its writes.
#[derive(Debug, Default)]
pub struct ReplayTransport {
    answers: IndexMap<Command, Answers>,
    /// Reads refused because the recording does not hold them.
    missing: usize,
    /// Reads refused because they were asked more often than recorded.
    exhausted: usize,
    writes: VecDeque<RecordedWrite>,
    /// How many writes the recording holds.
    recorded_writes: usize,
    /// What became of each write made, in the order made.
    write_checks: Vec<WriteCheck>,
    /// The ordinal of every recorded call served, in the order asked.
    served: Vec<usize>,
}

/// What became of one write the port made, in the order made.
#[derive(Debug, Clone)]
pub enum WriteCheck {
    /// The next recorded write: the same route and the same body.
    Matched,
    /// The next recorded write's route with another body: both bodies, as
    /// the values they carry. Answered as recorded, so the run goes on.
    Body { made: PyValue, recorded: PyValue },
    /// Another route than the next recorded write's. Refused.
    Route,
    /// A write after every recorded one. Refused.
    Unrecorded,
}

/// Why a recording could not be read.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[error("calls.jsonl line {line}: {problem}")]
pub struct RecordingError {
    pub line: usize,
    pub problem: String,
}

fn field<'a>(entry: &'a PyDict, key: &str, line: usize) -> Result<&'a PyValue, RecordingError> {
    entry.get(key).ok_or_else(|| RecordingError {
        line,
        problem: format!("no {key}"),
    })
}

fn string(value: &PyValue, key: &str, line: usize) -> Result<String, RecordingError> {
    match value {
        PyValue::Str(s) => Ok(s.clone()),
        _ => Err(RecordingError {
            line,
            problem: format!("{key} is not a string"),
        }),
    }
}

impl ReplayTransport {
    /// The reads of a recording, from the text of its `calls.jsonl`: one
    /// JSON object per line, in the order the calls were made.
    pub fn from_calls_jsonl(text: &str) -> Result<Self, RecordingError> {
        let mut replay = ReplayTransport::default();
        // The recorder ends every line with `\n` and writes no other line
        // break raw, so this is its line structure even where an answer
        // holds characters `str.splitlines` would also split on.
        for (index, line) in text.split('\n').enumerate() {
            let number = index + 1;
            if line.is_empty() {
                continue;
            }
            let entry = py_loads(line).map_err(|error| RecordingError {
                line: number,
                problem: error.to_string(),
            })?;
            let PyValue::Dict(entry) = entry else {
                return Err(RecordingError {
                    line: number,
                    problem: "not an object".into(),
                });
            };
            replay.add(&entry, number)?;
        }
        Ok(replay)
    }

    fn add(&mut self, entry: &PyDict, line: usize) -> Result<(), RecordingError> {
        let kind = string(field(entry, "kind", line)?, "kind", line)?;
        // The recorder numbers its calls from one; a line without a number
        // is numbered by where it stands.
        let ordinal = match entry.get("ordinal") {
            Some(PyValue::Int(n)) => n.as_i64().and_then(|n| usize::try_from(n).ok()),
            _ => None,
        }
        .unwrap_or(line);
        let PyValue::List(arguments) = field(entry, "args", line)? else {
            return Err(RecordingError {
                line,
                problem: "args is not a list".into(),
            });
        };
        let arguments = arguments
            .iter()
            .map(|argument| string(argument, "args", line))
            .collect::<Result<Vec<_>, _>>()?;
        let stdin = match field(entry, "stdin", line)? {
            PyValue::None => None,
            value => Some(string(value, "stdin", line)?),
        };
        if kind != "read" {
            // The recorder answers every write with a success of its own
            // making; a write recorded as failing is one no replay here can
            // answer as GitHub did, and is refused rather than taken as a
            // success.
            if entry.contains_key("raised")
                || !matches!(entry.get("exit"), Some(PyValue::Int(exit)) if exit.is_zero())
            {
                return Err(RecordingError {
                    line,
                    problem: "a write recorded as failing is not replayed".into(),
                });
            }
            self.recorded_writes += 1;
            self.writes.push_back(RecordedWrite {
                ordinal,
                arguments,
                stdin,
                stdout: string(field(entry, "stdout", line)?, "stdout", line)?,
            });
            return Ok(());
        }
        let recorded = if entry.contains_key("raised") {
            Recorded::Raised
        } else {
            let exit = match field(entry, "exit", line)? {
                PyValue::Int(exit) => exit.as_i64().and_then(|e| i32::try_from(e).ok()),
                _ => None,
            };
            let Some(exit) = exit else {
                return Err(RecordingError {
                    line,
                    problem: "exit is not an exit status".into(),
                });
            };
            Recorded::Exited {
                exit,
                stdout: string(field(entry, "stdout", line)?, "stdout", line)?,
                stderr: string(field(entry, "stderr", line)?, "stderr", line)?,
            }
        };
        self.answers
            .entry((arguments, stdin))
            .or_default()
            .recorded
            .push((ordinal, recorded));
        Ok(())
    }

    /// How many recorded reads were never asked.
    pub fn unasked(&self) -> usize {
        self.answers
            .values()
            .map(|answers| answers.recorded.len() - answers.served)
            .sum()
    }

    /// How many reads were refused because the recording does not hold
    /// them at all.
    pub fn missing(&self) -> usize {
        self.missing
    }

    /// How many reads were refused because they were asked more often than
    /// the recording holds them.
    pub fn exhausted(&self) -> usize {
        self.exhausted
    }

    /// How many recorded writes were never made.
    pub fn unwritten(&self) -> usize {
        self.writes.len()
    }

    /// How many writes the recording holds.
    pub fn recorded_writes(&self) -> usize {
        self.recorded_writes
    }

    /// What became of each write made, in the order made.
    pub fn write_checks(&self) -> &[WriteCheck] {
        &self.write_checks
    }

    /// The ordinal of every recorded call served, in the order it was
    /// asked: `1, 2, 3, …` when the calls were made in the order recorded.
    pub fn served(&self) -> &[usize] {
        &self.served
    }

    /// A write, answered only as the next write the recording holds.
    fn write(&mut self, call: &Call) -> Result<Reply, TransportError> {
        let (arguments, stdin) =
            gh_arguments(call).map_err(|error| TransportError::Refused(error.to_string()))?;
        let Some(recorded) = self.writes.front() else {
            self.write_checks.push(WriteCheck::Unrecorded);
            return Err(TransportError::Refused(format!(
                "A write the recording does not hold: {}",
                named(call, &arguments)
            )));
        };
        let differs = |what: &str, was: &str, now: &str| {
            TransportError::Refused(format!(
                "Write #{} differs from the recording in its {what}: recorded {was}, made {now}",
                recorded.ordinal
            ))
        };
        if recorded.arguments != arguments {
            let refused = differs("route", &recorded.arguments.join(" "), &arguments.join(" "));
            self.write_checks.push(WriteCheck::Route);
            return Err(refused);
        }
        let (was, now) = (
            same_value(recorded.stdin.as_deref()),
            same_value(stdin.as_deref()),
        );
        self.write_checks.push(if was == now {
            WriteCheck::Matched
        } else {
            WriteCheck::Body {
                made: value_of(stdin.as_deref()),
                recorded: value_of(recorded.stdin.as_deref()),
            }
        });
        let ordinal = recorded.ordinal;
        let answer = self
            .writes
            .pop_front()
            .map_or_else(String::new, |recorded| recorded.stdout);
        self.served.push(ordinal);
        Ok(Reply::Text(answer))
    }
}

/// A request body as the value it carries: its JSON, the text itself when
/// it is not JSON, `None` when there is none.
fn value_of(stdin: Option<&str>) -> PyValue {
    match stdin {
        None => PyValue::None,
        Some(text) => py_loads(text).unwrap_or_else(|_| PyValue::Str(text.to_owned())),
    }
}

/// A request body as the JSON value it carries, keys sorted, so that two
/// bodies holding the same entries compare equal; the text itself when it
/// is not JSON.
fn same_value(stdin: Option<&str>) -> String {
    let Some(stdin) = stdin else {
        return "nothing".into();
    };
    match py_loads(stdin) {
        Ok(value) => {
            py_dumps(&value, true, Some((",", ":")), None).unwrap_or_else(|_| stdin.to_owned())
        }
        Err(_) => stdin.to_owned(),
    }
}

/// How a refusal names a call: the `gh api` command, and for GraphQL,
/// whose arguments are all alike, which document it carried.
fn named(call: &Call, arguments: &[String]) -> String {
    match call {
        Call::Graphql { .. } => format!("gh api {} ({call})", arguments.join(" ")),
        Call::Rest { .. } => format!("gh api {}", arguments.join(" ")),
    }
}

impl Transport for ReplayTransport {
    fn call(&mut self, call: &Call) -> Result<Reply, TransportError> {
        if !is_read(call) {
            return self.write(call);
        }
        let (arguments, stdin) =
            gh_arguments(call).map_err(|error| TransportError::Refused(error.to_string()))?;
        let key = (arguments, stdin);
        let Some(answers) = self.answers.get_mut(&key) else {
            self.missing += 1;
            return Err(TransportError::Refused(format!(
                "A read the recording does not hold: {}",
                named(call, &key.0)
            )));
        };
        let Some((ordinal, answer)) = answers.recorded.get_mut(answers.served) else {
            self.exhausted += 1;
            return Err(TransportError::Refused(format!(
                "A read asked more often than the recording holds it ({} times): {}",
                answers.recorded.len(),
                named(call, &key.0)
            )));
        };
        answers.served += 1;
        self.served.push(*ordinal);
        // Each answer is served once, so it is moved out rather than copied.
        match std::mem::replace(answer, Recorded::Raised) {
            Recorded::Raised => Err(Failure::unavailable().into()),
            Recorded::Exited {
                exit: 0, stdout, ..
            } => Ok(Reply::Text(stdout)),
            Recorded::Exited {
                exit,
                stdout,
                stderr,
            } => Err(Failure {
                transient: transient(&stderr, &stdout),
                status: Some(exit),
                body: stdout,
                detail: stderr,
                class: FailureClass::Unclassed,
            }
            .into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_write_is_never_a_read() {
        let graphql = |query: &str| Call::Graphql {
            query: query.into(),
            variables: PyValue::Dict(PyDict::new()),
        };
        assert!(is_read(&graphql(
            "query($owner:String!) { viewer { login } }"
        )));
        assert!(!is_read(&graphql(
            "mutation { addComment(input: {}) { clientMutationId } }"
        )));
        assert!(!is_read(&graphql("query { a } mutation { b }")));
        // A field merely named like one is still a read.
        assert!(is_read(&graphql("query { mutations_total }")));
        let rest = |method, path: &str, body| Call::Rest {
            method,
            path: path.into(),
            body,
            paginate: false,
        };
        assert!(is_read(&rest(Method::Get, "repos/a/b/pulls/1", None)));
        assert!(!is_read(&rest(
            Method::Get,
            "repos/a/b/pulls/1",
            Some(PyValue::None)
        )));
        assert!(!is_read(&rest(
            Method::Delete,
            "repos/a/b/issues/comments/5",
            None
        )));
        assert!(!is_read(&rest(Method::Get, "user", None)));
        assert!(!is_read(&rest(Method::Get, "repos/a/b/../c/x", None)));
        assert!(!is_read(&rest(
            Method::Get,
            "https://api.github.com/repos/a/b/pulls/1",
            None
        )));
    }

    #[test]
    fn the_transient_rule_is_a_substring_anywhere_in_what_gh_said() {
        assert!(transient("HTTP 502: Bad Gateway", ""));
        assert!(transient("", "unexpected end of JSON input"));
        assert!(transient("dial tcp: i/o timeout", ""));
        // The sign may span the two streams, as in Python's concatenation.
        assert!(transient("50", "2"));
        // Case counts: Go's "Client.Timeout" is not Python's "timeout".
        assert!(!transient("Client.Timeout exceeded", ""));
        assert!(!transient("HTTP 422: Validation Failed", ""));
    }
}
