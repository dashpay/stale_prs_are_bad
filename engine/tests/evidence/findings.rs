//! How a review bot labelled the findings it opened a thread with: the
//! severities the reader keeps in place of the thread's text.
//!
//! Ported from the `finding_severities` assertions of
//! `pr_review/tests/test_policy.py`; what the policy then makes of a label
//! (`finding_blocks`) is the policy's, and is tested there.

use pr_hygiene_engine::evidence::rules::finding_severities;

const PASTA_SUGGESTION: &str = "<!-- thepastaclaw-review v1 finding=1ae5c0d05709 dedupe=823d079e92be8910 -->\n**🟡 Suggestion: Add coverage for the ambiguous unpadded base64/base58 case**\n\nThe regression test uses a padded encoding, so it never reaches the ambiguous case.";
const PASTA_BLOCKING: &str = "<!-- thepastaclaw-review v1\nfinding=86cd4cfc5f6d dedupe=eabaeb4163b84d5f -->\n**🔴 Blocking: Do not require a live Tokio runtime for every later UI frame**\n\n**Why:** `spawn_blocking` panics when no runtime is entered.";
const PASTA_NITPICK: &str = "<!-- thepastaclaw-review v1 finding=1 dedupe=2 -->\n**💬 Nitpick: Document why the probe stays synchronous**\n\nA sentence would do.";
const RABBIT_MINOR: &str = "_🎯 Functional Correctness_ | _🟡 Minor_ | _⚡ Quick win_\n\n**Reject an empty identifier.**\n\n_Note: this mirrors the parser above._\n<!-- cr-comment:v1:1 -->";
const RABBIT_MAJOR: &str = "_🗄️ Data Integrity & Integration_ | _🟠 Major_ | _🏗️ Heavy lift_\n\n**Support `ECDSA_HASH160` in the external signer before selecting it.**";
const RABBIT_REFACTOR_MAJOR: &str =
    "_🛠️ Refactor suggestion_ | _🟠 Major_\n\n**Extract the shared decoder.**";

fn rabbit_minor_then_major() -> String {
    format!(
        "{RABBIT_MINOR}\n\n---\n\n_📐 Maintainability & Code Quality_ | _🟠 Major_ | _🏗️ Heavy lift_\n\n**Move state-transition protocol interpretation to Rust.**\n<!-- cr-comment:v1:2 -->"
    )
}

#[test]
fn every_finding_in_an_opening_is_read() {
    // CodeRabbit can open one thread with two findings, a Minor and then a
    // Major. Reading the first heading alone lets the Major sit behind the
    // Minor.
    assert_eq!(
        finding_severities("coderabbitai", &rabbit_minor_then_major()),
        ["🟡 Minor", "🟠 Major"]
    );
    // The same holds for thepastaclaw should it ever group findings.
    let grouped = format!("{PASTA_SUGGESTION}\n\n{PASTA_BLOCKING}");
    assert_eq!(
        finding_severities("thepastaclaw", &grouped),
        ["🟡 Suggestion", "🔴 Blocking"]
    );
    // A later finding under a label nobody knows is kept as written, which
    // the policy reads as holding the thread.
    let unknown = format!("{PASTA_SUGGESTION}\n\n**🟣 Urgent: Something new**");
    assert_eq!(
        finding_severities("thepastaclaw", &unknown),
        ["🟡 Suggestion", "🟣 Urgent"]
    );
    let unknown = format!("{RABBIT_MINOR}\n\n---\n\n_🎯 Functional Correctness_ | _🟣 Urgent_\n");
    assert_eq!(
        finding_severities("coderabbitai", &unknown),
        ["🟡 Minor", "_🎯 Functional Correctness_ | _🟣 Urgent_"]
    );
}

#[test]
fn each_bot_is_read_in_its_own_heading_shape() {
    for (author, body, expected) in [
        ("thepastaclaw", PASTA_SUGGESTION, vec!["🟡 Suggestion"]),
        ("thepastaclaw", PASTA_NITPICK, vec!["💬 Nitpick"]),
        // A bold "Why:" in the text is not a heading, and a marker spanning
        // lines is not text.
        ("thepastaclaw", PASTA_BLOCKING, vec!["🔴 Blocking"]),
        // An italic line in the text is not a heading either.
        ("coderabbitai", RABBIT_MINOR, vec!["🟡 Minor"]),
        ("coderabbitai[bot]", RABBIT_MAJOR, vec!["🟠 Major"]),
        // "suggestion" in a CodeRabbit category is not a severity.
        ("coderabbitai", RABBIT_REFACTOR_MAJOR, vec!["🟠 Major"]),
    ] {
        assert_eq!(
            finding_severities(author, body),
            expected,
            "{author}: {body}"
        );
    }
}

#[test]
fn a_label_outside_a_heading_is_not_a_severity() {
    // A finding quoting another one, or a heading inside a hidden marker,
    // must not lend its label to the thread.
    let quoted = format!(
        "{RABBIT_MAJOR}\n\n> _🎯 Functional Correctness_ | _🟡 Minor_\n\nThe 🟡 Minor note above is stale."
    );
    assert_eq!(finding_severities("coderabbitai", &quoted), ["🟠 Major"]);
    let hidden = format!("<!--\n_🎯 Functional Correctness_ | _🟡 Minor_\n-->\n{RABBIT_MAJOR}");
    assert_eq!(finding_severities("coderabbitai", &hidden), ["🟠 Major"]);
    let prose =
        "<!-- thepastaclaw-review v1 -->\nThis reads like a 🟡 Suggestion but has no heading.";
    assert!(finding_severities("thepastaclaw", prose).is_empty());
}

#[test]
fn a_blocker_this_cannot_parse_still_holds_the_thread() {
    // One heading parsed is not every heading parsed: a blocking label in a
    // shape the parser does not know is still read, wherever it appears in
    // the opening as written — even where a stray `<!--` in the text hides
    // the heading from the parser. Each answer is Python 3.12's.
    for (second, expected) in [
        (
            " _🎯 Functional Correctness_ | _🟠 Major_",
            vec!["🟡 Minor", "🟠 Major"],
        ),
        (
            "_⚠️ Potential issue_ | **🟠 Major**",
            vec!["🟡 Minor", "🟠 Major"],
        ),
        (
            "> _🎯 Functional Correctness_ | _🟠 Major_",
            vec!["🟡 Minor", "🟠 Major"],
        ),
        (
            "_🟡 Minor_ | _🟠 Major_",
            vec!["🟡 Minor", "🟡 Minor", "🟠 Major"],
        ),
        (
            "`<!--` opens a comment in the sample.\n\n_🎯 Functional Correctness_ | _🟠 Major_",
            vec!["🟡 Minor", "🟠 Major"],
        ),
    ] {
        let body = format!(
            "{RABBIT_MINOR}\n\n---\n\n{second}\n\n**Fix the decoder.**\n<!-- cr-comment:v1:2 -->"
        );
        assert_eq!(
            finding_severities("coderabbitai", &body),
            expected,
            "{second}"
        );
    }
    let fenced = format!("{PASTA_SUGGESTION}\n\n### 🔴 Blocking: the fence hides the heading");
    assert_eq!(
        finding_severities("thepastaclaw", &fenced),
        ["🟡 Suggestion", "🔴 Blocking"]
    );
}

#[test]
fn the_severity_is_read_not_the_kind_of_finding() {
    // 🧹 Nitpick is the kind of finding in CodeRabbit's heading, beside its
    // severity. Taking it for the severity made a Major read as optional.
    for (heading, expected) in [
        ("_🧹 Nitpick_ | _🔵 Trivial_", "🔵 Trivial"),
        ("_🧹 Nitpick_ | _🟠 Major_", "🟠 Major"),
        // No severity CodeRabbit uses: the heading is kept as written.
        (
            "_🧹 Nitpick_ | _🟣 Something new_",
            "_🧹 Nitpick_ | _🟣 Something new_",
        ),
    ] {
        let body = format!("{heading}\n\n**Rename it.**");
        assert_eq!(
            finding_severities("coderabbitai", &body),
            [expected],
            "{heading}"
        );
    }
}

#[test]
fn each_bot_is_held_to_its_own_labels() {
    // A label one bot uses is unknown in the other's heading shape, and an
    // unknown label is kept as written, which holds the thread.
    let pasta = "<!-- thepastaclaw-review v1 -->\n**🟡 Minor: Rename it**";
    assert_eq!(finding_severities("thepastaclaw", pasta), ["🟡 Minor"]);
    let rabbit = "_🎯 Functional Correctness_ | _🟡 Suggestion_\n\n**Rename it.**";
    assert_eq!(
        finding_severities("coderabbitai", rabbit),
        ["_🎯 Functional Correctness_ | _🟡 Suggestion_"]
    );
}

#[test]
fn a_heading_with_windows_line_endings_is_still_a_heading() {
    let crlf = RABBIT_MINOR.replace('\n', "\r\n");
    assert_eq!(finding_severities("coderabbitai", &crlf), ["🟡 Minor"]);
}

#[test]
fn a_heading_without_a_known_severity_is_kept_so_it_blocks() {
    let body = "_🎯 Functional Correctness_ | _⚡ Quick win_\n\n**Fix it.**";
    assert_eq!(
        finding_severities("coderabbitai", body),
        ["_🎯 Functional Correctness_ | _⚡ Quick win_"]
    );
}

#[test]
fn a_person_has_no_severity() {
    assert!(finding_severities("reviewer", PASTA_SUGGESTION).is_empty());
    assert!(finding_severities("reviewer", RABBIT_MAJOR).is_empty());
}
