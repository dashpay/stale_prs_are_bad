//! A boundary recording as `pr_review/conformance.py` writes one: the files
//! of one recorded run, read with the crate's own JSON reader.

use crate::pycompat::{py_loads, PyErr, PyValue, ValueError};
use std::fmt;

/// The recording formats this reader accepts. Format 1 is format 2 without
/// `clock_reads`; nothing here reads the clock log, so both compare alike.
pub const READABLE_FORMATS: [i64; 2] = [1, 2];

/// The files of a recording this crate reads.
pub const FILES: [&str; 4] = [
    "recording.json",
    "calls.jsonl",
    "evaluations.jsonl",
    "verdicts.json",
];

/// One recorded run: what it was, every call it made, and what its
/// `evaluate` and `evaluate_snapshots` were given and answered.
#[derive(Clone)]
pub struct Recording {
    /// `recording.json`: the format, the repository, the command, the policy.
    pub meta: PyValue,
    /// The text of `calls.jsonl`, one call per line, as the replay
    /// transport reads it.
    pub calls: String,
    /// Each line of `evaluations.jsonl`: the `pr` Python's `evaluate` saw,
    /// its other arguments, and its result.
    pub evaluations: Vec<PyValue>,
    /// `verdicts.json`: the rows `evaluate_snapshots` returned, in order.
    pub verdicts: Vec<PyValue>,
}

/// Why a recording could not be read. Its text names a file and a
/// position, never what the file holds.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum LoadError {
    #[error("{file}: cannot be read ({problem})")]
    Unreadable { file: &'static str, problem: String },
    #[error("{file} line {line}: not JSON ({problem})")]
    NotJson {
        file: &'static str,
        line: usize,
        problem: String,
    },
    #[error("{file}: {problem}")]
    Shape {
        file: &'static str,
        problem: &'static str,
    },
    #[error("recording.json: a recording format this reader does not know")]
    Format,
}

/// What was wrong with a line of JSON, without what the line holds: a
/// decoding error names what was expected and where; anything else is
/// named by its class alone.
fn json_problem(error: &PyErr) -> String {
    match error {
        PyErr::Value(ValueError::JsonDecode(decode)) => decode.to_string(),
        PyErr::Value(ValueError::LoneSurrogate { pos }) => format!("lone surrogate at char {pos}"),
        PyErr::Recursion(_) => "nested too deeply".into(),
        _ => "unreadable".into(),
    }
}

fn parse(file: &'static str, line: usize, text: &str) -> Result<PyValue, LoadError> {
    py_loads(text).map_err(|error| LoadError::NotJson {
        file,
        line,
        problem: json_problem(&error),
    })
}

fn field<'a>(value: &'a PyValue, key: &str) -> Option<&'a PyValue> {
    match value {
        PyValue::Dict(entries) => entries.get(key),
        _ => None,
    }
}

impl Recording {
    /// Read a recording through `read`, which returns the text of the named
    /// file: the crate does no I/O of its own.
    pub fn load_with<E: fmt::Display>(
        mut read: impl FnMut(&'static str) -> Result<String, E>,
    ) -> Result<Self, LoadError> {
        let mut text = |file: &'static str| {
            read(file).map_err(|error| LoadError::Unreadable {
                file,
                problem: error.to_string(),
            })
        };
        let meta = parse("recording.json", 1, &text("recording.json")?)?;
        if !matches!(meta, PyValue::Dict(_)) {
            return Err(LoadError::Shape {
                file: "recording.json",
                problem: "not an object",
            });
        }
        let format = match field(&meta, "format") {
            Some(PyValue::Int(format)) => format.as_i64(),
            _ => None,
        };
        let Some(format) = format.filter(|f| READABLE_FORMATS.contains(f)) else {
            return Err(LoadError::Format);
        };
        // As `load_recording` refuses: the clock log belongs to format 2
        // and only to it.
        let logged = matches!(field(&meta, "clock_reads"), Some(PyValue::List(_)));
        if (format == 2) != logged {
            return Err(LoadError::Format);
        }
        let calls = text("calls.jsonl")?;
        // One record per `\n`, as the recorder writes them: free text inside
        // a record may hold the other characters `splitlines` breaks at.
        let evaluations = text("evaluations.jsonl")?
            .split('\n')
            .enumerate()
            .filter(|(_, line)| !line.is_empty())
            .map(|(index, line)| parse("evaluations.jsonl", index + 1, line))
            .collect::<Result<Vec<_>, _>>()?;
        if evaluations.iter().any(|e| !matches!(e, PyValue::Dict(_))) {
            return Err(LoadError::Shape {
                file: "evaluations.jsonl",
                problem: "a line that is not an object",
            });
        }
        let verdicts = match parse("verdicts.json", 1, &text("verdicts.json")?)? {
            PyValue::List(rows) => rows.into_vec(),
            _ => {
                return Err(LoadError::Shape {
                    file: "verdicts.json",
                    problem: "not a list",
                })
            }
        };
        Ok(Recording {
            meta,
            calls,
            evaluations,
            verdicts,
        })
    }

    /// The recording's format: 1 or 2.
    pub fn format(&self) -> i64 {
        match field(&self.meta, "format") {
            Some(PyValue::Int(format)) => format.as_i64().unwrap_or(0),
            _ => 0,
        }
    }

    /// The repository it was recorded against, `owner/name`.
    pub fn repository(&self) -> Option<&str> {
        match field(&self.meta, "repository") {
            Some(PyValue::Str(repository)) => Some(repository),
            _ => None,
        }
    }

    /// The policy the run loaded, which every evaluation used unless it
    /// carries its own.
    pub fn policy(&self) -> Option<&PyValue> {
        field(&self.meta, "policy")
    }

    /// Whether `pr_review.conformance redact` wrote this copy.
    pub fn redacted(&self) -> bool {
        !matches!(field(&self.meta, "redacted"), None | Some(PyValue::None))
    }

    /// The command that was recorded, from what names the work selected
    /// alone: `report`, `sync --batch-size 6`, `sync --pr 12`. Never the
    /// user a report was filtered for, which is a login.
    pub fn command(&self) -> String {
        let Some(PyValue::List(argv)) = field(&self.meta, "argv") else {
            return "?".into();
        };
        let words: Vec<&str> = argv
            .iter()
            .map(|word| match word {
                PyValue::Str(word) => word.as_str(),
                _ => "",
            })
            .collect();
        let mut said = vec![match words.first() {
            Some(&command) if ["report", "sync"].contains(&command) => command.to_owned(),
            _ => "?".to_owned(),
        }];
        let mut rest = words.iter().skip(1);
        while let Some(&word) = rest.next() {
            match word {
                "--pr" | "--batch-size" => {
                    if let Some(number) = rest.next().filter(|n| {
                        !n.is_empty() && n.len() <= 9 && n.bytes().all(|b| b.is_ascii_digit())
                    }) {
                        said.push(format!("{word} {number}"));
                    }
                }
                "--waiting-on-build" => said.push(word.to_owned()),
                _ => {}
            }
        }
        said.join(" ")
    }

    /// How many requests to GitHub the recorded reads made: one per call,
    /// except a paginated one, which made one per page it printed, whether
    /// or not a later page then failed. A retried call is two calls. Writes
    /// were never sent. GraphQL's own limit is counted in points, which
    /// this does not see.
    pub fn requests(&self) -> Result<usize, LoadError> {
        let mut total = 0;
        for (index, line) in self.calls.split('\n').enumerate() {
            if line.is_empty() {
                continue;
            }
            let entry = parse("calls.jsonl", index + 1, line)?;
            if !matches!(field(&entry, "kind"), Some(PyValue::Str(kind)) if kind == "read") {
                continue;
            }
            let paginated = matches!(field(&entry, "args"), Some(PyValue::List(args))
                if args.iter().any(|a| matches!(a, PyValue::Str(a) if a == "--paginate")));
            let pages = match field(&entry, "stdout") {
                Some(PyValue::Str(stdout)) if paginated => match py_loads(stdout) {
                    Ok(PyValue::List(pages)) => pages.len(),
                    _ => 1,
                },
                _ => 1,
            };
            // A paginated read that printed no page still asked once.
            total += if pages == 0 { 1 } else { pages };
        }
        Ok(total)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn recording(meta: &str, calls: &str) -> Result<Recording, LoadError> {
        Recording::load_with(|name| -> Result<String, std::io::Error> {
            Ok(match name {
                "recording.json" => meta.to_owned(),
                "calls.jsonl" => calls.to_owned(),
                "evaluations.jsonl" => String::new(),
                _ => "[]".to_owned(),
            })
        })
    }

    #[test]
    fn formats_1_and_2_read_and_no_other_does() {
        let format = |meta: &str| recording(meta, "").map(|r| r.format());
        assert_eq!(format(r#"{"format": 2, "clock_reads": []}"#), Ok(2));
        assert_eq!(format(r#"{"format": 1}"#), Ok(1));
        // The clock log belongs to format 2 and only to it.
        assert_eq!(format(r#"{"format": 2}"#), Err(LoadError::Format));
        assert_eq!(
            format(r#"{"format": 1, "clock_reads": []}"#),
            Err(LoadError::Format)
        );
        assert_eq!(
            format(r#"{"format": 3, "clock_reads": []}"#),
            Err(LoadError::Format)
        );
        assert_eq!(format(r#"{"format": "2"}"#), Err(LoadError::Format));
    }

    #[test]
    fn a_broken_file_is_named_by_position_not_by_what_it_holds() {
        let error = recording(r#"{"format": 1, "title": "secret"#, "")
            .err()
            .unwrap()
            .to_string();
        assert!(
            error.starts_with("recording.json line 1: not JSON"),
            "{error}"
        );
        assert!(!error.contains("secret"), "{error}");
        let missing = Recording::load_with(|_| -> Result<String, &str> { Err("gone") })
            .err()
            .unwrap();
        assert_eq!(missing.to_string(), "recording.json: cannot be read (gone)");
    }

    #[test]
    fn the_command_names_the_work_and_never_the_user() {
        let command = |argv: &str| {
            recording(&format!(r#"{{"format": 1, "argv": {argv}}}"#), "")
                .unwrap()
                .command()
        };
        assert_eq!(
            command(r#"["report", "--repo", "a/b", "--format", "json"]"#),
            "report"
        );
        assert_eq!(
            command(r#"["sync", "--repo", "a/b", "--pr", "12", "--format", "markdown"]"#),
            "sync --pr 12"
        );
        assert_eq!(
            command(r#"["sync", "--repo", "a/b", "--batch-size", "6"]"#),
            "sync --batch-size 6"
        );
        assert_eq!(
            command(r#"["report", "--repo", "a/b", "--user", "alice"]"#),
            "report"
        );
        assert_eq!(command(r#"["sync", "--pr", "alice"]"#), "sync");
        assert_eq!(command(r#"["rm -rf", "--pr", "1"]"#), "? --pr 1");
    }

    #[test]
    fn requests_count_pages_of_a_paginated_read_and_nothing_for_a_write() {
        let calls = [
            r#"{"kind": "read", "args": ["--method", "GET", "repos/a/b/pulls", "--paginate", "--slurp"], "exit": 0, "stdout": "[[1], [2], [3]]"}"#,
            r#"{"kind": "read", "args": ["--method", "GET", "repos/a/b/pulls/1"], "exit": 0, "stdout": "{}"}"#,
            r#"{"kind": "read", "args": ["--method", "GET", "repos/a/b/x", "--paginate", "--slurp"], "exit": 1, "stdout": ""}"#,
            r#"{"kind": "read", "args": ["--method", "GET", "repos/a/b/y", "--paginate", "--slurp"], "exit": 1, "stdout": "[[1], [2]]"}"#,
            r#"{"kind": "read", "args": ["--method", "POST", "graphql"], "raised": "TimeoutExpired"}"#,
            r#"{"kind": "write", "args": ["--method", "POST", "repos/a/b/statuses/c"], "exit": 0, "stdout": "{}"}"#,
        ]
        .join("\n");
        let recorded = recording(r#"{"format": 1}"#, &calls).unwrap();
        // Three pages; one; a failure that printed nothing, still asked;
        // a failure after two pages; a call that ran out of time.
        assert_eq!(recorded.requests(), Ok(3 + 1 + 1 + 2 + 1));
    }
}
