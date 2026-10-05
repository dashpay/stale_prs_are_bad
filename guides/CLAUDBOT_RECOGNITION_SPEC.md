# Recognize the Claudbot review application

## Problem and evidence

The requested account is `claudbot[bot]`. It is absent from both review
engines' `BOTS` constants. On 2026-10-05, the default branch and this worktree
were both at `828ad254694828ba3c64d3714e0dd0752e0a892a`. The repository's only
open PR was #45, an unrelated ownership wording fix; a search of all PR titles
for `claud` returned no matches.

Example: https://github.com/dashpay/dash-evo-tool/pull/1042
REST review #5385114718 identifies `claudbot[bot]`, type `Bot`, state
`COMMENTED`, with two inline findings. GraphQL identifies that same author as
login `claudbot`, typename `Bot`. Both evidence readers already normalize
GraphQL application logins to their REST `[bot]` spelling.

GitHub review states and review payloads are documented at:
https://docs.github.com/en/rest/pulls/reviews#list-reviews-for-a-pull-request

## Chosen approach and alternatives

Add the exact normalized `claudbot[bot]` identity to `BOTS` in
`pr_review/policy.py` and `engine/src/policy/mod.rs`. Keep the Python and Rust
decisions equal. No new identity abstraction or API requests are needed.
Add the same identity to the dashboard's review-bot list in
`src/analyzer.rs`, using its existing review-bot severity grading. Otherwise
the application is classified as a bot but its threads are graded `Low` and
hidden by the default `count_nitpicks: false` configuration.

The request is interpreted as recognizing an additional review bot, rather
than requiring its review on every governed PR. The engines' `REVIEW_BOTS` continues to name
the two producers with supported completion receipts and nudges. Claudbot
findings use the existing conservative default for an unknown severity format:
unresolved threads block until resolved, and a changes-requested review blocks
when it covers the currently reviewed head. Its approval does not substitute
for a person's approval or another required bot's completion receipt.

Alternatives rejected:
- Add both bare and suffixed names: normalization already supplies the suffix;
  treating a bare human login as an application would broaden the trust boundary.
- Add Claudbot to the engines' `REVIEW_BOTS`: requires a completion protocol and nudge
  semantics that the request and example do not establish.
- Recognize every `[bot]` as a review producer: changes unrelated applications'
  treatment and exceeds this request.
- Parse Claudbot MEDIUM/LOW prose as advisory severity: that is a separate
  policy decision and needs a producer format contract.

## Interface, data flow, and failure modes

REST evidence or normalized GraphQL evidence -> normalized login in snapshot
-> existing `BOTS` membership -> bot blockers, human-approval exclusion,
checklist wording, and self-attestation exclusion in each engine.

The readers already skip permission lookups for every `[bot]` account, so
their `BOT_LOGINS` constants need no change. The dashboard already classifies
every `[bot]` login as a bot and its fetcher normalizes Bot-typed GraphQL
actors to that spelling. Its separate review-bot severity list needs the
addition; its generic known-bot list needs no change.

The material behavior change is that an unresolved Claudbot thread is reported
as a bot blocker regardless of repository write permission. Today unknown
permission (`None`) preserves a human objection, while confirmed read or no
permission can drop the thread. The dashboard will use its existing review-bot
grading: ordinary prose defaults to `Medium`, while text containing recognized
nitpick/trivial markers can remain filtered as `Low`. This display convention
differs from the engines' conservative blocking rule for unknown producers;
adding a Claudbot severity parser is outside this change. A `COMMENTED`
review with resolved or absent threads does not create a new required-bot wait.
Unknown severity labels remain blocking. Dismissed reviews and older-head
changes requests retain existing semantics. The bare human login remains human.
Unresolved threads remain blocking across heads; neither `/skip-bots` nor a
missing-report timeout waives them. Replies under a recognized bot's thread
follow the existing bot-thread handling rather than human-objection timing.

## Verification plan

Before the constants change, add matching Python and Rust regression cases:
an unresolved thread from `claudbot[bot]` on a PR otherwise ready to merge,
with application permission `None` and confirmed `read`, must produce the bot
blocker and checklist entry. Observe those tests fail, then add the identity
and observe them pass. Add a dashboard regression proving an unresolved
Claudbot review survives the default nitpick filter, observing red before
adding it to the analyzer list and green after.
Cover resolution, no extra wait after a plain commented review, current-head
changes requested, case folding, skip/timeout non-waiver, and the bare-login
distinction using existing fixtures. Reuse existing coverage for unchanged
older-head/dismissal, normalization, and human-approval semantics rather than
duplicating a full bot lifecycle suite. Verify normalization covers Bot-typed
GraphQL authors. Run the Python policy suite, Rust policy suite and conformance
tests, dashboard analyzer tests, plus Rust formatting. Do not regenerate golden
evidence unrelated to this identity. Review the actual diff independently after
implementation.

## Review and agreement

Three independent reviewers examined correctness/feasibility, security, and
simplicity/test scope. Their required corrections are included: recognize the
dashboard review-bot list too, assert bot-specific reasons instead of merely
blocked state, and test confirmed read access as well as unknown access. Their
caveats are explicit above: shared dashboard severity grading, blockers that
survive pushes/skips/timeouts, and bot-thread reply semantics. No outstanding
design blocker was reported.

The user approved implementation, push, admin merge, and re-pinning any
governed caller that needs it, including admin-merging those updates. Caller
discovery found all seven deployed callers across the five governed
repositories use `master`; they pick up the central merge without pin edits.

## Verification results

The three Python policy regressions, their three Rust counterparts, and the
dashboard filter regression failed against the original lists and passed after
the additions. Python 3.12: all 538 tests pass. Rust workspace: 876 tests pass,
with two existing local-recording tests ignored because no redacted live
recordings are provided. Workspace Clippy, Rust formatting, and dashboard
security checks pass. Fresh harvesting adds 12 evaluation cases and 12
diff-print cases; no existing corpus file changes. The 287 evaluation and 676
function cases match the fresh harvest, and the Rust policy suite passes with
the new corpus. Policy compatibility goldens remain unchanged.

Three independent code reviewers reported no required fixes. The Rust
checklist assertion was narrowed to the checklist itself following review.
