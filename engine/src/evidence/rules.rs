//! The parts of `pr_review/policy.py` the reader applies as it reads: which
//! accounts are the engine's own, which are review bots, the delimiters of
//! the checklist block, and how each review bot labels a finding.

use crate::pycompat::re::translate;
use crate::pycompat::text::{py_lower, py_strip};
use regex::Regex;
use std::sync::LazyLock;

/// The accounts the engine writes as, lowercase. Its records, statuses and
/// diff prints are its memory, recognised only by who wrote them; only the
/// `name[bot]` spelling counts, since a person can register the bare name.
pub const ENGINE_LOGINS: &[&str] = &["github-actions[bot]"];

/// `is_engine(login)` for a login known to be a string: one of
/// [`ENGINE_LOGINS`], in any case.
pub fn is_engine(login: &str) -> bool {
    ENGINE_LOGINS.contains(&py_lower(login).as_str())
}

/// Accounts whose access is never read: the review bots, and the engine.
pub const BOT_LOGINS: &[&str] = &[
    "coderabbitai[bot]",
    "coderabbitai",
    "thepastaclaw",
    "github-actions[bot]",
];

/// Where the engine's block in a pull request's description starts.
pub const CHECKLIST_START: &str = "<!-- pr-hygiene:start -->";
/// Where it ends.
pub const CHECKLIST_END: &str = "<!-- pr-hygiene:end -->";

/// How each review bot labels a finding it opens a thread for, and whether
/// that label holds the pull request, in the order `policy.py` lists them.
pub const FINDING_BLOCKS: &[(&str, &[(&str, bool)])] = &[
    (
        "thepastaclaw",
        &[
            ("🔴 Blocking", true),
            ("🟡 Suggestion", false),
            ("💬 Nitpick", false),
        ],
    ),
    (
        "coderabbitai",
        &[
            ("🔴 Critical", true),
            ("🟠 Major", true),
            ("🟡 Minor", false),
            ("🔵 Trivial", false),
        ],
    ),
];

/// A pattern written in Python's syntax, compiled with Python's classes.
/// Every pattern here is a constant the tests compile, so neither step can
/// fail at run time.
fn compiled(pattern: &str) -> Regex {
    let translated = translate(pattern).expect("a constant pattern translates");
    Regex::new(&translated).expect("a constant pattern compiles")
}

static HIDDEN_MARKUP: LazyLock<Regex> = LazyLock::new(|| compiled(r"(?s)<!--.*?-->"));
// thepastaclaw: `**🟡 Suggestion: title**`. The label leads with an emoji,
// which keeps a bold "Why:" in the text from reading as a heading.
static PASTA_HEADING: LazyLock<Regex> =
    LazyLock::new(|| compiled(r"(?m)^\*\*(?P<label>[^\x00-\x7f][^*:\n]*):"));
// CodeRabbit: `_🎯 Functional Correctness_ | _🟠 Major_ | _⚡ Quick win_`, a
// line of two or more italic segments and nothing else.
static RABBIT_HEADING: LazyLock<Regex> =
    LazyLock::new(|| compiled(r"(?m)^_[^_\n]+_(?:[ \t]*\|[ \t]*_[^_\n]+_)+[ \t\r]*$"));

/// `_producer(author)`: the account a bot's label table is filed under.
fn producer(author: &str) -> String {
    let lowered = py_lower(author);
    match lowered.strip_suffix("[bot]") {
        Some(bare) => bare.to_owned(),
        None => lowered,
    }
}

/// `finding_severities(author, body)`: the severity of every finding a
/// review bot opened a thread with.
///
/// Every heading is read, since one opening can hold several findings, and
/// only the bot's own heading shape names a label, so one quoted in the
/// text is not taken for one. A blocking label counts wherever it appears,
/// so a heading this cannot parse beside one it can still holds the thread;
/// a heading with no severity that bot uses is kept as written, which holds
/// it too. Anyone else's thread has no labels.
pub fn finding_severities(author: &str, body: &str) -> Vec<String> {
    let who = producer(author);
    let Some((_, table)) = FINDING_BLOCKS.iter().find(|(bot, _)| *bot == who) else {
        return Vec::new();
    };
    let text = HIDDEN_MARKUP.replace_all(body, "");
    let mut labels: Vec<String> = Vec::new();
    if who == "thepastaclaw" {
        for found in PASTA_HEADING.captures_iter(&text) {
            if let Some(label) = found.name("label") {
                labels.push(py_strip(label.as_str()).to_owned());
            }
        }
    } else {
        for line in RABBIT_HEADING.find_iter(&text) {
            let known: Vec<String> = line
                .as_str()
                .split('|')
                .map(|segment| py_strip(py_strip(segment).trim_matches('_')).to_owned())
                .filter(|segment| table.iter().any(|(label, _)| label == segment))
                .collect();
            if known.is_empty() {
                labels.push(py_strip(line.as_str()).to_owned());
            } else {
                labels.extend(known);
            }
        }
    }
    let unread: Vec<String> = table
        .iter()
        .filter(|(label, blocks)| {
            *blocks && body.contains(label) && !labels.iter().any(|l| l == label)
        })
        .map(|(label, _)| (*label).to_owned())
        .collect();
    labels.extend(unread);
    labels
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_pattern_compiles() {
        for pattern in [&HIDDEN_MARKUP, &PASTA_HEADING, &RABBIT_HEADING] {
            LazyLock::force(pattern);
        }
    }

    #[test]
    fn only_the_bot_spelling_is_the_engine_in_any_case() {
        assert!(is_engine("github-actions[bot]"));
        assert!(is_engine("GitHub-Actions[BOT]"));
        assert!(!is_engine("github-actions"));
        assert!(!is_engine(""));
    }
}
