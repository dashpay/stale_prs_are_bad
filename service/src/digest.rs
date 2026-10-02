//! One person's digest as plain text, for the `/prs` skill and Slack: what
//! they owe, what they hold, and how fresh the data is.
//!
//! The wording of what a reviewer is asked for is the engine's own
//! (`your_part` and `asks` in `pr_review/main.py`), so the digest, the PR's
//! checklist and the engine's reports say the same thing the same way.
//!
//! Everything taken from GitHub (titles, logins, errors) has its control
//! characters replaced and Slack's mrkdwn control characters escaped: a
//! title cannot start a new line, mention a channel or forge a link.

use crate::view::{RepoView, View};
use chrono::{DateTime, SecondsFormat, Utc};
use pr_hygiene::dashboard::{Owed, Owner, PersonOut, PrOut, SinceBasis};
use std::collections::HashMap;
use std::fmt::Write;

/// Bumped whenever a reader parsing the previous layout would misread it.
pub const VERSION: u32 = 1;

/// Make untrusted text safe on one line of a Slack message or a terminal:
/// control and bidirectional-override characters become spaces, and `&`,
/// `<` and `>` are escaped as Slack requires.
pub fn clean(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            c if c.is_control() || is_bidi_control(c) => out.push(' '),
            c => out.push(c),
        }
    }
    out
}

fn is_bidi_control(c: char) -> bool {
    matches!(c, '\u{200e}' | '\u{200f}' | '\u{202a}'..='\u{202e}' | '\u{2066}'..='\u{2069}')
}

/// The engine's `area_name`: the fallback area by what it is, others as code.
fn area_name(area: &str) -> String {
    if area == "fallback" {
        "files with no dedicated owner".to_string()
    } else {
        format!("`{}`", clean(area))
    }
}

/// The engine's `your_part`: each area this person may approve and nobody
/// has, with who else may instead, then any re-review their own objection
/// waits on.
pub fn your_part(owed: &Owed) -> String {
    let mut parts: Vec<String> = owed
        .areas
        .iter()
        .map(|a| {
            let others: Vec<String> = a.others.iter().map(|o| clean(o)).collect();
            if others.is_empty() {
                area_name(&a.area)
            } else {
                format!("{} (you or {})", area_name(&a.area), others.join(" or "))
            }
        })
        .collect();
    if owed.rereview {
        parts.push("re-review or resolve your objection".to_string());
    }
    parts.join(" · ")
}

/// The engine's `asks`: every area still unapproved with who may approve
/// it, then the objectors; "an owner" when there is nothing to name.
pub fn needs(pr: &PrOut) -> String {
    let mut parts: Vec<String> = pr
        .asks
        .iter()
        .map(|a| {
            let who: Vec<String> = a.approvers.iter().map(|x| clean(x)).collect();
            let who = if who.is_empty() {
                "nobody may approve".to_string()
            } else {
                who.join(" or ")
            };
            format!("{}: {who}", area_name(&a.area))
        })
        .collect();
    if !pr.objectors.is_empty() {
        let objectors: Vec<String> = pr.objectors.iter().map(|o| clean(o)).collect();
        parts.push(format!("re-review or resolve: {}", objectors.join(", ")));
    }
    if parts.is_empty() {
        "an owner".to_string()
    } else {
        parts.join(" · ")
    }
}

/// The engine's `age`: whole hours, as days and hours from a day on.
fn age(since: DateTime<Utc>, now: DateTime<Utc>) -> String {
    let hours = (now - since).num_hours().max(0);
    if hours >= 24 {
        format!("{}d {}h", hours / 24, hours % 24)
    } else {
        format!("{hours}h")
    }
}

/// How long the PR has been in its stage, where that is recorded.
fn waiting(pr: &PrOut, now: DateTime<Utc>) -> String {
    match pr.since {
        Some(since) if pr.since_basis == Some(SinceBasis::Engine) => age(since, now),
        _ => "not recorded".to_string(),
    }
}

fn time(t: DateTime<Utc>) -> String {
    t.to_rfc3339_opts(SecondsFormat::Secs, true)
}

fn url(pr: &PrOut) -> String {
    format!("https://github.com/{}/pull/{}", pr.repo, pr.number)
}

/// The reviews someone owes with their PRs, the longest-waiting first. A PR
/// whose wait is not recorded sorts by its age, which only overstates it.
pub fn owed_oldest_first<'a>(view: &'a View, person: &'a PersonOut) -> Vec<(&'a Owed, &'a PrOut)> {
    let prs: HashMap<&str, &PrOut> = view.prs.iter().map(|p| (p.key.as_str(), p)).collect();
    let mut owed: Vec<(&Owed, &PrOut)> = person
        .owes
        .iter()
        .filter_map(|o| prs.get(o.pr.as_str()).map(|p| (o, *p)))
        .collect();
    owed.sort_by(|(_, a), (_, b)| match (a.since, b.since) {
        (Some(x), Some(y)) => x.cmp(&y).then_with(|| a.key.cmp(&b.key)),
        (Some(_), None) => std::cmp::Ordering::Less,
        (None, Some(_)) => std::cmp::Ordering::Greater,
        (None, None) => a.key.cmp(&b.key),
    });
    owed
}

pub fn authored<'a>(view: &'a View, person: &PersonOut) -> Vec<&'a PrOut> {
    let prs: HashMap<&str, &PrOut> = view.prs.iter().map(|p| (p.key.as_str(), p)).collect();
    person
        .authored
        .iter()
        .filter_map(|k| prs.get(k.as_str()).copied())
        .collect()
}

fn whose_move(owner: Owner) -> &'static str {
    match owner {
        Owner::Author => "your move",
        Owner::Reviewers => "reviewers' move",
        Owner::Bots => "the bots' move",
        Owner::Maintainers => "maintainers' move",
        Owner::Nobody => "nobody's move",
    }
}

fn repo_line(r: &RepoView) -> String {
    let mode = serde_json::to_value(r.mode)
        .ok()
        .and_then(|v| v.as_str().map(String::from))
        .unwrap_or_default();
    if !r.stale {
        let since = r.data_as_of.map_or("never".to_string(), time);
        return format!("- {} ({mode}): data from {since}", r.repo);
    }
    let why = match &r.fetch_error {
        Some(e) => format!("could not be fetched: {}", clean(e)),
        None => "the engine's verdicts were not available".to_string(),
    };
    match r.data_as_of {
        Some(at) => format!(
            "- {} ({mode}): STALE, showing data from {} (latest run: {why})",
            r.repo,
            time(at)
        ),
        None => format!(
            "- {} ({mode}): STALE, no data yet (latest run: {why})",
            r.repo
        ),
    }
}

/// The digest for `person`, as of the view's snapshot (never the clock:
/// the same snapshot always gives the same text).
pub fn person(view: &View, person: &PersonOut) -> String {
    let now = view.generated_at;
    let mut out = String::new();
    // `write!` into a String cannot fail.
    let _ = writeln!(
        out,
        "PR Hygiene digest v{VERSION} for {}",
        clean(&person.login)
    );
    let _ = writeln!(
        out,
        "Generated {} from commit {}",
        time(now),
        view.commit.as_deref().map_or("unknown".to_string(), clean)
    );
    let _ = writeln!(out, "Repositories:");
    for r in &view.repos {
        let _ = writeln!(out, "{}", repo_line(r));
    }
    let stale = view.stale_repos();
    let _ = writeln!(
        out,
        "Stale: {}",
        if stale.is_empty() {
            "none".to_string()
        } else {
            stale.join(", ")
        }
    );

    let owed = owed_oldest_first(view, person);
    let _ = writeln!(out);
    if owed.is_empty() {
        let _ = writeln!(out, "Reviews you owe: none");
    } else {
        let _ = writeln!(out, "Reviews you owe: {}, oldest first", owed.len());
    }
    for (i, (o, pr)) in owed.iter().enumerate() {
        let _ = writeln!(out, "{}. {} {}", i + 1, pr.key, clean(&pr.title));
        let author = pr.author.as_deref().map_or("unknown".to_string(), clean);
        let _ = writeln!(out, "   Author: {author}. Waiting: {}.", waiting(pr, now));
        let part = your_part(o);
        if part.is_empty() {
            let _ = writeln!(out, "   Needs: {}", needs(pr));
        } else {
            let _ = writeln!(out, "   Your part: {part}");
        }
        let _ = writeln!(out, "   {}", url(pr));
    }

    let mine = authored(view, person);
    let _ = writeln!(out);
    if mine.is_empty() {
        let _ = writeln!(out, "Your PRs: none");
    } else {
        let _ = writeln!(out, "Your PRs: {}", mine.len());
    }
    for (i, pr) in mine.iter().enumerate() {
        let _ = writeln!(out, "{}. {} {}", i + 1, pr.key, clean(&pr.title));
        let _ = writeln!(
            out,
            "   Stage: {} ({}). In it: {}.",
            pr.stage.key(),
            whose_move(pr.stage.owner()),
            waiting(pr, now)
        );
        let next = pr.next_action.as_deref().map_or("-".to_string(), clean);
        let _ = writeln!(out, "   Next: {next}");
        let _ = writeln!(out, "   {}", url(pr));
    }

    if !person.wip.is_empty() {
        let _ = writeln!(out);
        let _ = writeln!(out, "Open PRs against review slots:");
        for (repo, wip) in &person.wip {
            let limit = view.repo(repo).map_or(5, |r| r.slot_limit);
            let over = if *wip > limit { ", over the limit" } else { "" };
            let _ = writeln!(out, "- {repo}: {wip} of {limit}{over}");
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use pr_hygiene::dashboard::AreaPart;

    fn owed(areas: &[(&str, &[&str])], rereview: bool) -> Owed {
        Owed {
            pr: "dashpay/platform#1".into(),
            areas: areas
                .iter()
                .map(|(area, others)| AreaPart {
                    area: area.to_string(),
                    others: others.iter().map(|s| s.to_string()).collect(),
                })
                .collect(),
            rereview,
        }
    }

    /// The strings `pr_review/tests/test_checklist.py` and `test_main.py`
    /// expect of the engine's `your_part`, for the same asks.
    #[test]
    fn your_part_is_worded_as_the_engine_words_it() {
        assert_eq!(
            your_part(&owed(&[("dpp", &["bob", "carol"])], false)),
            "`dpp` (you or bob or carol)"
        );
        assert_eq!(
            your_part(&owed(
                &[
                    ("fallback", &["QuantumExplorer"]),
                    ("github", &["ktechmidas"])
                ],
                false
            )),
            "files with no dedicated owner (you or QuantumExplorer) · `github` (you or ktechmidas)"
        );
        assert_eq!(your_part(&owed(&[("core", &[])], false)), "`core`");
        assert_eq!(
            your_part(&owed(&[("core", &["carol"])], true)),
            "`core` (you or carol) · re-review or resolve your objection"
        );
        assert_eq!(
            your_part(&owed(&[], true)),
            "re-review or resolve your objection"
        );
    }

    /// The engine is the source of this wording; if it rewords, this
    /// service must follow, and this says so.
    #[test]
    fn the_engine_still_words_it_this_way() {
        let engine =
            std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/../pr_review/main.py"))
                .unwrap();
        for fragment in [
            "return 'files with no dedicated owner'",
            "return f'`{area}`' if code else area",
            r#"(f" (you or {' or '.join(others)})" if others else '')"#,
            "parts.append('re-review or resolve your objection')",
            "return ' · '.join(parts)",
            r#"parts.append(f"{area_name(area['area'], code)}: {' or '.join(area['approvers']) or 'nobody may approve'}")"#,
            "parts.append('re-review or resolve: ' + ', '.join(result['objectors']))",
            "return f'{hours // 24}d {hours % 24}h' if hours >= 24 else f'{hours}h'",
        ] {
            assert!(
                engine.contains(fragment),
                "pr_review/main.py no longer has {fragment}"
            );
        }
    }

    #[test]
    fn needs_is_worded_as_the_engines_asks() {
        let pr: PrOut = serde_json::from_value(serde_json::json!({
            "key": "dashpay/platform#1", "repo": "dashpay/platform", "number": 1, "title": "t",
            "author": "alice", "author_kind": "human", "draft": false, "base": "v3",
            "created_at": null, "updated_at": null, "idle": false, "stage": "review",
            "engine_state": "ready-for-human", "next_action": null, "blockers": [],
            "since": null, "since_basis": null, "lateness": null,
            "asks": [{"area": "dpp", "approvers": ["Alice", "Carol"]},
                     {"area": "drive", "approvers": []}],
            "objectors": ["bob", "dave"], "areas": [], "unresolved_comments": 0,
            "ci_failing": false, "merge_conflict": false, "changes_requested": false,
            "tracked": true
        }))
        .unwrap();
        assert_eq!(
            needs(&pr),
            "`dpp`: Alice or Carol · `drive`: nobody may approve · re-review or resolve: bob, dave"
        );
    }

    #[test]
    fn untrusted_text_cannot_break_a_line_mention_or_link() {
        assert_eq!(
            clean("Fix <!channel> & <https://evil.example|docs>\n- fake line\u{7}"),
            "Fix &lt;!channel&gt; &amp; &lt;https://evil.example|docs&gt; - fake line "
        );
        assert_eq!(clean("a\u{202e}b\r\tc"), "a b  c");
        assert_eq!(clean("ünïcode stays"), "ünïcode stays");
    }

    #[test]
    fn ages_read_as_the_engines() {
        let t = |h: i64| DateTime::<Utc>::UNIX_EPOCH + chrono::TimeDelta::hours(h);
        assert_eq!(age(t(0), t(5)), "5h");
        assert_eq!(age(t(0), t(24)), "1d 0h");
        assert_eq!(age(t(0), t(51)), "2d 3h");
        assert_eq!(age(t(5), t(0)), "0h");
    }
}
