//! The writes of `github.py`, on the fake: the tests of statuses and labels
//! from `test_github.py`, and of the description from `test_checklist.py`
//! (`DescriptionWriteTests`, `MarkerParsingTests`).

use crate::fake::*;
use crate::scene::*;
use crate::support::*;
use pr_hygiene_engine::evidence::records::current_checklist;
use pr_hygiene_engine::policy::{CHECKLIST_END, CHECKLIST_START};

fn one() -> PyInt {
    PyInt::from(1)
}

fn labels(names: &[&str]) -> PyValue {
    PyValue::List(names.iter().map(|n| s(n)).collect())
}

/// `set_state_label(1, state, worn)` on a pull request wearing `worn`: the
/// label writes it made.
fn state_label(state: &str, worn: &[&str]) -> Vec<String> {
    let (_, mut fake) = fixture();
    fake.pr(1).labels = worn.iter().map(|n| (*n).to_owned()).collect();
    let mut scene = Scene::new(fake);
    scene
        .with(|engine| {
            engine
                .api()
                .set_state_label(&one(), &s(state), &labels(worn))
        })
        .expect("the labels are written");
    scene
        .fake
        .written
        .iter()
        .map(|w| match &w.body {
            Some(body) => format!("{} {}", w.method, dump(field(body, "labels"))),
            None => format!("{} {}", w.method, w.route.rsplit('/').next().unwrap_or("")),
        })
        .collect()
}

#[test]
fn exactly_one_state_label_and_nothing_else_is_touched() {
    for (state, worn) in [
        ("ready-for-human", &["ready-for-human", "bug"][..]),
        ("draft", &["bug"][..]),
        ("waiting-build", &["bug"][..]),
        ("ready-to-merge", &["bug"][..]),
    ] {
        assert_eq!(state_label(state, worn), Vec::<String>::new(), "{state}");
    }
    assert_eq!(
        state_label(
            "waiting-bots",
            &["ready-for-human", "bug", "bot-review-skipped"]
        ),
        [r#"POST ["waiting-bots"]"#, "DELETE ready-for-human"],
        "the new label first, so a refused removal never leaves it label-less; unrelated labels stay"
    );
    assert_eq!(
        state_label("configuration-error", &["waiting-bots"]),
        ["DELETE waiting-bots"],
        "a state with no label clears the old one"
    );
    // An objection and a missing attestation are one move — the author's.
    assert_eq!(
        state_label("waiting-author", &[]),
        [r#"POST ["waiting-self-review"]"#]
    );
    assert_eq!(
        state_label("too-many-open-prs", &[]),
        [r#"POST ["too-many-open-prs"]"#]
    );
    // Names this engine used to set are cleared wherever still seen.
    let mut cleared = state_label(
        "waiting-bots",
        &["waiting-bots", "waiting-author", "ready-to-merge"],
    );
    cleared.sort();
    assert_eq!(cleared, ["DELETE ready-to-merge", "DELETE waiting-author"]);
}

#[test]
fn an_identical_status_is_not_published_again() {
    let (_, mut fake) = fixture();
    fake.state
        .engine_status(HEAD, "success", "ready-to-merge", "2026-09-11T09:00:00Z");
    let mut scene = Scene::new(fake);
    scene.with(|engine| {
        let api = engine.api();
        api.post_status(HEAD, "success", "ready-to-merge", None)
            .expect("nothing to post");
        api.post_status(HEAD, "pending", "new evidence", None)
            .expect("posted");
        // A status just written is remembered, so the next write on the
        // same head does not read the commit's whole history again.
        api.post_status(HEAD, "pending", "new evidence", None)
            .expect("nothing to post");
    });
    assert_eq!(scene.writes(), [format!("POST statuses/{HEAD}")]);
    let listed = scene
        .fake
        .calls
        .iter()
        .filter(|call| matches!(call, Call::Rest { path, .. } if path.ends_with("/statuses?per_page=100")))
        .count();
    assert_eq!(listed, 1);
}

#[test]
fn a_status_write_that_was_not_acknowledged_is_refused() {
    let (_, mut fake) = fixture();
    fake.refuse(Method::Post, "statuses/", Refusal::Empty);
    let mut scene = Scene::new(fake);
    let refused = scene.with(|engine| {
        engine
            .api()
            .post_status(HEAD, "success", "ready-to-merge", None)
    });
    assert!(
        matches!(&refused, Err(ReadError::GitHub(m)) if m == "Commit status was not acknowledged"),
        "{refused:?}"
    );
}

#[test]
fn a_status_description_is_cut_to_what_github_keeps() {
    let (_, fake) = fixture();
    let mut scene = Scene::new(fake);
    let long = "é".repeat(200);
    scene
        .with(|engine| engine.api().post_status(HEAD, "pending", &long, None))
        .expect("posted");
    let posted = text(scene.fake.written[0].field("description")).to_owned();
    assert_eq!(posted.chars().count(), 140, "characters, not bytes");
}

/// `set_checklist(1, block)` over a description: whether it wrote, and the
/// description afterwards.
fn checklist(body: &str, block: &str) -> (Result<bool, ReadError>, String, Vec<String>) {
    let (_, mut fake) = fixture();
    fake.pr(1).body = body.into();
    let mut scene = Scene::new(fake);
    let wrote = scene.with(|engine| engine.api().set_checklist(&one(), block));
    let body = scene.fake.pr(1).body.clone();
    (wrote, body, scene.writes())
}

fn block(inside: &str) -> String {
    format!("{CHECKLIST_START}\n{inside}\n{CHECKLIST_END}")
}

#[test]
fn only_the_block_is_written_and_the_authors_text_is_kept_byte_for_byte() {
    let author =
        "## Why\r\n\r\nBecause.\r\n\r\n- [ ] I have performed a self-review of my own code\r\n";
    let body = format!(
        "{author}\n{}\n<!-- coderabbit -->\ntrailing",
        block("stale")
    );
    let (wrote, body, _) = checklist(&body, &block("fresh"));
    assert!(wrote.expect("written"));
    assert!(
        body.starts_with(author.trim_end()),
        "the author's text, CRLF and all, untouched"
    );
    assert!(body.contains("fresh") && !body.contains("stale"));
    assert_eq!(body.matches(CHECKLIST_START).count(), 1);
    assert!(
        body.ends_with("\n<!-- coderabbit -->\ntrailing"),
        "what follows the block is someone else's and stays"
    );
}

#[test]
fn an_identical_block_is_not_rewritten_even_when_line_endings_differ() {
    let same = block("same");
    let (wrote, _, writes) = checklist(
        &format!("text\r\n\r\n{}", same.replace('\n', "\r\n")),
        &same,
    );
    assert!(!wrote.expect("read"));
    assert_eq!(writes, Vec::<String>::new());
}

#[test]
fn a_block_that_would_not_fit_is_refused_not_truncated() {
    let (wrote, body, writes) = checklist(&"x".repeat(65_000), &block(&"y".repeat(1000)));
    assert!(
        matches!(&wrote, Err(ReadError::GitHub(m)) if m == "Description too long for the checklist"),
        "{wrote:?}"
    );
    assert_eq!(body.len(), 65_000);
    assert_eq!(writes, Vec::<String>::new());
}

#[test]
fn a_block_holding_its_own_delimiters_is_refused() {
    let (wrote, _, writes) = checklist("text", &block(&format!("x {CHECKLIST_END} y")));
    assert!(matches!(wrote, Err(ReadError::GitHub(_))));
    assert_eq!(writes, Vec::<String>::new());
}

#[test]
fn the_last_marker_pair_is_the_engines() {
    let quoted = format!("```\n{}\n```", block("example"));
    let real = block("real");
    let read = |body: &str| current_checklist(&s(body)).expect("a description");
    assert_eq!(read(&format!("{quoted}\n\n{real}")), Some(real.clone()));
    assert_eq!(read(&format!("{CHECKLIST_START}\nno end")), None);
}

#[test]
fn an_extra_end_marker_after_the_block_is_the_authors_and_stays() {
    let body = format!("author\n\n{}\nKEEP ME\n{CHECKLIST_END}", block("real"));
    let (_, body, _) = checklist(&body, &block("fresh"));
    assert!(body.contains("KEEP ME"));
    assert_eq!(body.matches("fresh").count(), 1);
}

#[test]
fn a_fenced_example_below_the_block_is_not_the_block() {
    let real = block("real");
    let fenced = format!("```\n{}\n```", block("example"));
    let read = |body: &str| current_checklist(&s(body)).expect("a description");
    assert_eq!(read(&format!("{real}\n\n{fenced}")), Some(real.clone()));
    assert_eq!(
        read(&fenced),
        None,
        "a fenced example alone is no block: the first write appends"
    );
    assert_eq!(
        read(&format!("{CHECKLIST_END}\nreversed\n{CHECKLIST_START}")),
        None
    );
}

#[test]
fn removing_the_block_leaves_the_rest_of_the_description() {
    let (_, mut fake) = fixture();
    fake.pr(1).body = format!("Intro.\n\n{}\n\nAfter.", block("old"));
    let mut scene = Scene::new(fake);
    assert!(scene
        .with(|engine| engine.api().remove_checklist(&one()))
        .expect("removed"));
    assert_eq!(scene.fake.pr(1).body, "Intro.\n\nAfter.");
    scene.fake.forget_calls();
    assert!(!scene
        .with(|engine| engine.api().remove_checklist(&one()))
        .expect("nothing to remove"));
    assert_eq!(scene.writes(), Vec::<String>::new());
}
