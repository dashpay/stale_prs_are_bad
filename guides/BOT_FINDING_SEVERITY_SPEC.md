# Spec: only a bot's blocker blocks, and self-review is reset by code only

## Problem

dashpay/platform#5072 sits in `waiting-author` with a `waiting-self-review`
label although its author attested and nothing they must do remains:

- thepastaclaw's only finding on the head is an unresolved
  `🟡 Suggestion` thread. Every unresolved bot thread gates today
  (`policy.py` `bot_threads` → `findings` → `waiting-author`), whatever it says.
- The author posted bare `/self-reviewed` at 10:33 for head `42716ec`;
  thepastaclaw's final review of the same head landed at 12:52. Every bot
  receipt raises the floor an attestation must clear, so the attestation was
  voided by a review that changed no code and asked for nothing mandatory.

The owner's rule: a bot suggestion is not a blocker — only a blocker is. A
self-review says "I reviewed this code"; only a change to the code resets it.

## Evidence (governed repositories, last ~400 PRs, 2026-10-01)

- ~1,300 bot thread openings; every one is labelled, always on a heading line
  (thepastaclaw: first line after its `<!-- thepastaclaw-review … -->`
  marker; CodeRabbit: line 1). thepastaclaw: `🔴 Blocking`, `🟡 Suggestion`,
  `💬 Nitpick`. CodeRabbit: `🟠 Major`, `🟡 Minor`, `🔴 Critical`, as one
  segment of `_🎯 Functional Correctness_ | _🟠 Major_ | _⚡ Quick win_`
  (older: `_⚠️ Potential issue_ | _🟠 Major_`, `_🛠️ Refactor suggestion_ |
  _🟠 Major_`).
- One CodeRabbit opening can hold several findings, each with its own heading,
  separated by `---` (platform#4844: `🟡 Minor` then `🟠 Major`).
- thepastaclaw final reviews listing `[BLOCKING]` findings: 104/106 are
  `CHANGES_REQUESTED`; the other two (dash-evo-tool#624, #859) carry the
  findings as inline `🔴 Blocking` threads.
- CodeRabbit requests changes in rust-dashcore (45 `CHANGES_REQUESTED`), 14 of
  them over `🟡 Minor` threads only. Elsewhere it only comments.
- The receipt-prose blocker that motivated the floor (platform#4653,
  CodeRabbit "Add signer support … before merging") was posted as an inline
  `🟠 Major` thread on the next review.
- `🟠 Major`/`🔴 Critical` findings that live only in a CodeRabbit review body
  (outside-diff sections in two layouts, and "♻️ Duplicate comments"
  re-raising a resolved finding): 22 reviews in 18 of 400 PRs.
- GitHub does not require thread resolution on these branches
  (`required_review_thread_resolution: false`); PR Hygiene is the only gate.

## Design

### 1. Bot thread severity

A bot thread **blocks unless every finding heading in its opening comment is
labelled non-blocking**.

| Producer | Blocks | Does not block |
|---|---|---|
| thepastaclaw | `🔴 Blocking` | `🟡 Suggestion`, `💬 Nitpick` |
| CodeRabbit | `🔴 Critical`, `🟠 Major` | `🟡 Minor`, `🔵 Trivial` |

Each bot is held to its own table: a label the other bot uses is unknown.
CodeRabbit's `🧹 Nitpick` is the kind of finding beside the severity
(`_🧹 Nitpick_ | _🔵 Trivial_`), not a severity, so every known severity
segment on a heading is read.

Headings are found per producer, after stripping `<!-- … -->` markers
(which may span lines), in the producer's own heading shape only:

- thepastaclaw: a line `**<label>: …**` — the label is the text before `:`.
- CodeRabbit: a line made only of `_segment_` items joined by ` | ` — the
  label is the segment that is a known severity.

A heading whose label is unknown blocks. An opening with no heading blocks.
So a format change by either bot fails closed — the status quo, loudly. Only
headings name a suggestion: one quoted in prose or a blockquote is ignored.
A blocking label counts wherever it appears in the opening, so a blocker in a
shape the parser misses (a leading space, bold instead of italic, a heading
swallowed by an unbalanced `<!--`) beside a suggestion it does parse still
blocks. None of 839 real suggestion-only openings mentions a blocking label,
so the backstop costs nothing today. CRLF line endings are accepted.

A pure helper in `policy.py` returns the list of heading labels for an
opening; `github.py` (which already imports from `policy`) stores that list as
`thread['severities']` instead of the body. The evidence fingerprint then
does not flap when CodeRabbit appends "✅ Addressed in commit …" to the
opening. A thread without `severities` (old fixtures) blocks.

### 2. What the bot gate counts

`bot_threads` is filtered to blocking threads where it is built
(`policy.py` ~702), so `findings` (→ `waiting-author`), `heard`,
`outstanding` (nudge suppression), `threads_by` and the "not yet" wording all
follow from one change. `heard` matters most: a suggestion thread left on an
earlier head must not stand in for a report on this one.

Unchanged and deliberate: a bot's `CHANGES_REQUESTED` on a current head
blocks, whatever its threads say (rust-dashcore's CodeRabbit). It is an
explicit objection, and there it is the only gate on findings outside the
diff. Dismiss it or push.

Non-blocking threads are not listed in the checklist: GitHub already shows
unresolved conversations, and the bots box is checked.

### 3. A reply in a bot thread is an answer, not an objection

The objection loop keeps skipping bot-opened threads. The spec first proposed
counting human replies there as objections, so that "please do this" under a
`🟡 Suggestion` would still hold the PR. Real data reversed that: of 1,336
bot threads, 110 carry a reply from someone other than the author, and the
replies are answers from whoever is finishing the PR — "Fixed in b4b73669a8",
"Not applicable to Dash", "Agreed, deferred" — from colleagues
(`shumkov`, `lklimek`, `QuantumExplorer`), agent accounts
(`Claudius-Maginificent`) and automation (`github-actions`, reported by
GraphQL without `[bot]` and so not a known bot). Counting them would hold the
PRs being finished, and the snapshot reads permissions only for thread
authors, so an unknown replier would count too. A reviewer who wants a
suggestion taken requests changes or opens their own thread.

### 4. Self-review is reset by code only

- `/self-reviewed FULL_SHA` counts when the SHA is one of the diff-equivalent
  heads (`carried_heads`); no time floor (the SHA cannot predate its push).
- Bare `/self-reviewed` counts when posted after this controller first
  reported on the current diff (`seen_first`), as today.
- Bot receipts, waivers and `/skip-bots` no longer raise the floor.
- A push that changes the diff resets it; one that does not carries it
  (existing `carried_heads`).
- Human objections unchanged: one at/after the attestation is the author's to
  answer.
- `on_their_behalf` advice no longer depends on the bots being done.
- Gate text drops "after bot completion"; the checklist line becomes
  `post /self-reviewed`, without "once the bots are done".

### 5. Announcements

`bot_completed_at` and the receipt machinery (`receipt_print`,
`receipt_instant`, `controller_diff.receipts`) stay: they keep that instant
stable across cosmetic CodeRabbit rewrites. The freshness rule
(`main.py` ~618) — a bot reporting after the person was told posts a new
comment instead of editing — now applies only when the state is
`waiting-author` (a finding to answer). Otherwise a bot re-reporting
suggestions on a ready PR would repost "Policy satisfied" / "Ready for
review" and notify reviewers.

### 6. Docs and comments

`guides/PR_REVIEW_OPERATIONS.md`: step 2 (only blockers block; the severity
table in one sentence), step 3 (attestation reset by code only; remove the
"any bot report that lands after you attested" paragraph), and the line-28
freshness reason. Code comments whose reason no longer holds are rewritten
(`policy.py` floor/waiver/advice comments, `main.py` freshness comment).

## Alternatives rejected

- **Gate on review state only.** CodeRabbit requests changes only in
  rust-dashcore; elsewhere its findings exist only as threads.
- **Read only the first heading line.** Misses platform#4844's `🟠 Major`.
- **Keep the floor, raised only by reports carrying a blocking finding
  outside a thread.** Keeps the #4653 guard but adds review-body parsing per
  producer and keeps self-review tied to bot timing, which the owner rejects.
- **Gate on review-body findings.** No resolve action exists: a PR would be
  stuck until the bot re-reviews a new head.
- **Reject bot openings edited by someone else.** Anyone who can edit a bot
  comment can resolve its thread, which has the same effect.

## Failure modes / accepted risk

- **Reopens the hole the floor closed (3b04aea).** A `🟠 Major`/`🔴 Critical`
  that exists only in a CodeRabbit review body — outside the diff, or
  re-raised in "Duplicate comments" after its thread was resolved — no longer
  forces a fresh `/self-reviewed`. ~22 reviews in 400 PRs. It reaches
  `ready-to-merge` where the author owns every area, or a human approved
  before it landed. Today it blocks nothing either; it forces a second
  attestation. Owner accepts or picks the mitigation below.
- **Optional mitigation (not in scope unless asked):** a non-gating checklist
  line, "coderabbitai: N Major finding(s) outside threads — read the review",
  counted from the latest current-head CodeRabbit review.
- An attestation posted before a blocking thread still counts once that
  thread is resolved without a code change. Today it would also need a new
  `/self-reviewed`. Deliberate: resolving is the explicit act.
- Bot format drift → threads read as blocking → PRs hold as today.
- A deleted opening: `comments(first:1)` returns the first surviving reply. A
  bot reply has no heading and blocks; a human reply makes it a human thread
  (today's behaviour).

## Verification

Tests first, red against the current engine, then green. Fixtures use real
heading shapes.

1. #5072 shape: author-owned area, green build, CodeRabbit receipt,
   thepastaclaw `COMMENTED` final review with one `🟡 Suggestion` thread, bare
   `/self-reviewed` after `seen_first` but before the thepastaclaw review →
   `ready-to-merge`.
2. `🔴 Blocking` thread → `waiting-author`; unlabelled bot thread →
   `waiting-author`; CodeRabbit `🟠 Major` blocks, `🟡 Minor` does not;
   #4844 shape (Minor + Major in one opening) blocks;
   `_🛠️ Refactor suggestion_ | _🟠 Major_` blocks.
3. A suggestion thread and no receipt on the current head → `waiting-bots`.
4. A reply under a suggestion thread — a colleague, `github-actions`, a
   stranger — is not an objection.
5. CodeRabbit `CHANGES_REQUESTED` with only `🟡 Minor` threads still blocks.
6. Attestation before a bot receipt counts; diff-changing push resets it;
   diff-preserving push carries it.
7. A bot re-reporting on a ready PR does not repost the announcement; a bot
   reporting a blocker after the author was told does.
8. `threads()` stores severities, not the body; a CodeRabbit "✅ Addressed"
   edit leaves the fingerprint unchanged.
9. Existing tests pinning the floor are rewritten to the new rule (receipt
   tests assert `bot_completed_at` instead of the verdict), not deleted. Each
   reverted behaviour is caught by a named test (mutation check).

## Out of scope

The Rust dashboard grades severity differently (`src/analyzer.rs`: any `⚠`
is High, thepastaclaw `🔴 Blocking` is Medium). It ranks, it does not gate;
align it separately.
