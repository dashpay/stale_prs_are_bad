//! Whether what a differential tool printed holds anything a recording
//! holds: the one no-content check, shared by the tests of the replay tool
//! (`engine/tests/differential`) and of the live tool (the service's
//! `differential` module), which include this file.
//!
//! A report may say a word a recording also holds only where the word is
//! the tool's own — a heading, a layer, a kind, a field name a path spells
//! out, read from the string literals of the tool's sources — or names the
//! recording's row: its repository, command and directory.

#![allow(dead_code)]

use serde_json::Value;
use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

/// The files of a recording, as the recorder writes them.
pub const FILES: [&str; 6] = [
    "recording.json",
    "calls.jsonl",
    "evaluations.jsonl",
    "verdicts.json",
    "outputs.json",
    "printed.txt",
];

/// Every string a recording holds as a value — titles, bodies, logins,
/// permission levels, commits, instants — in every file it has.
pub fn strings_of(recording: &Path) -> Vec<String> {
    fn strings(value: &Value, out: &mut Vec<String>) {
        match value {
            Value::String(s) => {
                out.push(s.clone());
                // A recorded answer is JSON inside a string: its strings too.
                if let Ok(inner) = serde_json::from_str::<Value>(s) {
                    if inner.is_object() || inner.is_array() {
                        strings(&inner, out);
                    }
                }
            }
            Value::Array(items) => items.iter().for_each(|v| strings(v, out)),
            Value::Object(entries) => entries.values().for_each(|v| strings(v, out)),
            _ => {}
        }
    }
    let mut out = Vec::new();
    for file in ["calls.jsonl", "evaluations.jsonl"] {
        let text = std::fs::read_to_string(recording.join(file)).expect("a recording file");
        for line in text.lines() {
            strings(&serde_json::from_str(line).expect("a JSON line"), &mut out);
        }
    }
    for file in [
        "verdicts.json",
        "outputs.json",
        "printed.txt",
        "recording.json",
    ] {
        let text = std::fs::read_to_string(recording.join(file)).expect("a recording file");
        // The printed report is JSON where the run printed JSON, and is
        // looked through line by line where it printed Markdown.
        match serde_json::from_str::<Value>(&text) {
            Ok(value) => strings(&value, &mut out),
            Err(_) => out.extend(text.lines().map(str::to_owned)),
        }
    }
    out
}

/// The text of every string literal in `code`: what lies between two
/// unescaped double quotes.
fn literals(code: &str) -> Vec<String> {
    let mut found = Vec::new();
    let mut chars = code.chars();
    while let Some(c) = chars.next() {
        if c != '"' {
            continue;
        }
        let mut literal = String::new();
        while let Some(c) = chars.next() {
            match c {
                '"' => break,
                '\\' => {
                    literal.push(c);
                    if let Some(escaped) = chars.next() {
                        literal.push(escaped);
                    }
                }
                _ => literal.push(c),
            }
        }
        found.push(literal);
    }
    found
}

/// What the report may say whatever a recording holds: the words of the
/// tool's own text, and each of its phrases whole, read from the string
/// literals of `sources` outside their tests; and the recording's
/// repository, command and directory,
/// which name its row. A recording may hold any of these as values too:
/// `write` is a permission level, `draft` a state, `the` a word of a title.
pub fn own_words(sources: &[PathBuf], recording: &Path) -> BTreeSet<String> {
    let mut own = BTreeSet::new();
    for path in sources {
        let source = std::fs::read_to_string(path).expect("the tool's source");
        let code = source.split("#[cfg(test)]").next().unwrap_or_default();
        for literal in literals(code) {
            own.extend(words(&literal));
            // A recorded value that is one of the tool's own phrases whole —
            // a status the engine posts — is the tool's text too.
            own.insert(literal.to_lowercase());
        }
    }
    let meta: Value = serde_json::from_str(
        &std::fs::read_to_string(recording.join("recording.json")).expect("recording.json"),
    )
    .expect("JSON");
    own.extend(words(meta["repository"].as_str().unwrap_or_default()));
    for word in meta["argv"].as_array().into_iter().flatten() {
        own.extend(words(word.as_str().unwrap_or_default()));
    }
    own.extend(words(
        &recording.file_name().unwrap_or_default().to_string_lossy(),
    ));
    own
}

/// Every word of `text`: its runs of letters and digits, lowercased, of
/// three characters or more, which are not a number.
pub fn words(text: &str) -> BTreeSet<String> {
    text.split(|c: char| !c.is_alphanumeric())
        .filter(|w| w.chars().count() >= 3 && !w.chars().all(|c| c.is_ascii_digit()))
        .map(str::to_lowercase)
        .collect()
}

/// The strings of a recording long enough to be told apart from the words
/// of the report itself.
fn contents(sources: &[PathBuf], recording: &Path) -> Vec<String> {
    let own = own_words(sources, recording);
    let mut out = strings_of(recording);
    out.retain(|s| {
        s.chars().count() >= 5 && !own.contains(&s.to_lowercase()) && s != "dashpay/platform"
    });
    out.sort();
    out.dedup();
    out
}

/// Whether the occurrence of a recorded value at `at..end` of `said` is
/// only a part of the report's own words: it starts or ends inside a longer
/// word, and every word it touches is one of `own`. `reviewer` inside the
/// field `reviewers` is the report's vocabulary. A value standing alone, or
/// inside a word that is not the report's, is the value.
fn inside_own_words(said: &str, at: usize, end: usize, own: &BTreeSet<String>) -> bool {
    let word = |c: &char| c.is_alphanumeric();
    let before = said[..at].chars().next_back().filter(word);
    let after = said[end..].chars().next().filter(word);
    if before.is_none() && after.is_none() {
        return false;
    }
    let start = at
        - said[..at]
            .chars()
            .rev()
            .take_while(word)
            .map(char::len_utf8)
            .sum::<usize>();
    let stop = end
        + said[end..]
            .chars()
            .take_while(word)
            .map(char::len_utf8)
            .sum::<usize>();
    words(&said[start..stop]).iter().all(|w| own.contains(w))
}

/// Nothing a recording holds appears in what the tool said: no whole value,
/// except as a part of the report's own words, and no word of one that is
/// not also one of the report's own.
#[track_caller]
pub fn assert_no_contents(said: &str, recording: &Path, sources: &[PathBuf]) {
    let seen = contents(sources, recording);
    assert!(seen.len() > 20, "the recording holds text to look for");
    let own = own_words(sources, recording);
    let leaked: Vec<&String> = seen
        .iter()
        .filter(|s| {
            said.match_indices(s.as_str())
                .any(|(at, found)| !inside_own_words(said, at, at + found.len(), &own))
        })
        .collect();
    assert!(
        leaked.is_empty(),
        "printed what the recording holds: {leaked:?}\n{said}"
    );
    let held: BTreeSet<String> = strings_of(recording)
        .iter()
        .flat_map(|s| words(s))
        .collect();
    let shared: Vec<String> = words(said)
        .into_iter()
        .filter(|w| held.contains(w) && !own.contains(w))
        .collect();
    assert!(
        shared.is_empty(),
        "printed a word the recording holds: {shared:?}\n{said}"
    );
}
