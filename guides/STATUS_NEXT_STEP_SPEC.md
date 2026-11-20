# The `PR Hygiene` status says the next step

## Problem and evidence

GitHub shows the `PR Hygiene` commit status in the merge box. Today its
description is just a state name: `waiting-bots`, `waiting-self-review`,
`ready-for-human`, and so on (`main.status_description`,
`pr_review/main.py:376`). The reason only appears in the move comment and
in the checklist in the PR description. People read the bare state, decide
the engine is stuck, and bypass it.

Verified live with read-only `gh api` on 2026-10-27:

- **tenderdash #1549** (head `b638379`). The status said
  `waiting-self-review` from 2026-10-21 16:54 to 2026-10-22 15:56 UTC.
  - From 23:51 on 10-21 the move comment said "Your move: coderabbitai left
    review threads unresolved; resolve them."
  - lklimek posted `/self-reviewed` at 08:45 on 10-22, but the status did
    not change. A bot finding (`waiting-author`) is shown under the
    self-review name (`main.py:382`, commit 02ab833).
  - The PR merged at 15:56.
- **platform #5494, #5455, #5483**: the status said `waiting-bots`. The
  comment said "Waiting for bot review — coderabbitai not yet ·
  thepastaclaw ✓."
- **platform #5541**: the comment said "coderabbitai skipped after its own
  rate limit · thepastaclaw not yet."
- **The status on the head when the PR merged**, for the last ~60 updated
  closed PRs each of platform and tenderdash:

  | Status | Merges |
  |---|---|
  | `waiting-bots` | 29 |
  | `waiting-self-review` | 17 |
  | `ready-for-human` | 10 |
  | `waiting-build` | 3 |
  | `ready-to-merge` | 24 |

  The earlier audit found that 16 of platform's 36 bypassed merges were at
  `waiting-bots`.

## GitHub constraints

- **Length: 140 characters.** GitHub's REST reference does not document
  it. The API enforces it with the error "description is too long (maximum
  is 140 characters)", reported in zalando/zappr#378 and kirmanak/jumi#145.
  The engine already cuts to 140 code points (`github.py:1089`,
  `engine/src/reconcile/writes.rs:79`).
- **Non-ASCII is unproven.** Every sampled `PR Hygiene` and CodeRabbit
  description on our repos is ASCII. So are about 70 heads sampled across
  17 large public repos. If GitHub rejected a character, every verdict post
  would get a 422, and every PR in all five repos would turn `error` red,
  including green ones (`main.py:1036-1045`). The text is therefore kept
  ASCII (below).
- **Per-SHA cap:** 1000 statuses per SHA and context.
  https://docs.github.com/en/rest/commits/statuses#create-a-commit-status
  The busiest of 110 open platform heads has 71.
- **Secondary limits:** at most 80 content-creating requests a minute and
  500 an hour.
  https://docs.github.com/en/rest/using-the-rest-api/rate-limits-for-the-rest-api#about-secondary-rate-limits
- **Plain text only.** GitHub renders no markdown here, and @-mentions do
  not notify.

## Approach

The status of `waiting-bots`, `waiting-self-review`, `waiting-author` and
`ready-for-human` carries the line the move comment already computes:

```
<token>: <move line, folded to ASCII>
```

The result is cut to 140 characters, as described below.

- **`<token>` is exactly today's description.** For these states that is
  the move name: `waiting-bots`, `waiting-self-review` (also used for
  `waiting-author`), and `ready-for-human`. To get the state back, take the
  part before the first `': '`. No state name contains `': '`.
- **The line is the move comment's own line.**
  - The body of `move_text` becomes `move_line(result)`. `move_text` wraps
    it in the marker and footer exactly as today, so its output and the
    order of its exceptions do not change. A prototype confirmed that every
    `main.move_text` corpus case still replays.
  - The status applies a fixed character fold, because it is plain text and
    must be ASCII:
    - `` ` `` is removed;
    - `—` becomes `-`;
    - `·` becomes `|`;
    - `✓` becomes `done`.
  - These are the only non-ASCII characters a move line can contain. It
    contains only:
    - logins;
    - bot names;
    - policy area ids, which must match `[a-z0-9][a-z0-9-]*`
      (`policy.py:115`);
    - fixed text.

    It never contains titles, paths or comment text.
- **Truncation.** If the text is over 140 code points:
  1. Take the first 138.
  2. Drop everything from the last space onward.
  3. Append `...`.

  In Python this is `said[:138].rsplit(' ', 1)[0] + '...'`. The result is
  never longer than 140, because the space after the token is always within
  the first 22 characters. The existing `[:140]` cut in `post_status` stays
  as a backstop. What blocks the PR comes first in every line, but a very
  long list of areas or findings can itself be cut.
- **Every other status keeps its current text**:
  - **`draft`, `waiting-build`, `configuration-error`**: they have no move
    line.
  - **`ready-to-merge`**: it is already green, and nobody bypasses a green
    check. Its line, "this can merge", would also claim more than was
    checked.
  - **`too-many-open-prs`**: `Missing human approval` and
    `Human review unresolved` already state the requirement. The rest of
    the line is about slot routing, which 64f5077 deliberately kept out of
    the status.
  - **A bot-authored PR in the self-review move**: `publish` posts no move
    comment there (`main.py:701`), and the checklist says self-review is
    not asked of it. The verdict carries `checklist.self_review.bot_author`,
    so `status_description` can tell this case without the policy.
  - **The fixed non-verdict phrases** (`Evaluating current review policy`,
    `…; reconciliation required`, …).
- **No @-mentions.** `move_text` already drops the one reason that contains
  `@`: "Proceeded without …: skipped by @x" (`main.py:521`). None of 84 live
  lines and no corpus case contains `@`.

### Examples

These were computed by a prototype of the rule over live move lines from
2026-10-27 and over the evaluate corpus. The number before each line is its
length.

```
  5  draft
 13  waiting-build
 19  configuration-error
 14  ready-to-merge
 22  Missing human approval                         (too-many-open-prs; or "Human review unresolved", 23)
 19  waiting-self-review                            (bot-authored PR in waiting-author)
134  waiting-bots: Waiting for bot review - coderabbitai not yet | thepastaclaw done. Wait for the missing reviews, or a writer can post...
132  waiting-bots: Waiting for bot review - coderabbitai skipped after its own rate limit | thepastaclaw not yet. Wait for the missing...
136  waiting-bots: Waiting for bot review - thepastaclaw not yet. Wait for the missing reviews, or a writer can post /skip-bots to proceed...
 68  waiting-self-review: Bots are done - your move: post /self-reviewed.
 90  waiting-self-review: Your move: coderabbitai left review threads unresolved; resolve them.                (#1549)
125  waiting-self-review: Bots are done - your move: address romchornyi left a review thread unresolved, then post /self-reviewed.
138  waiting-self-review: Your move: thepastaclaw requested changes on this head; dismiss the review or push a fix; thepastaclaw left review...
 98  ready-for-human: Ready for review - github: ktechmidas or shumkov | re-review or resolve: lklimek.
132  ready-for-human: Ready for review - files with no dedicated owner: QuantumExplorer or shumkov | dpp: QuantumExplorer or shumkov |...
```

The longest realistic case is the waiting-bots line at 134–136 characters.
Lines are cut as follows:

| Lines | Live | Corpus |
|---|---|---|
| waiting-bots | 18 of 18, always inside the generic advice | 39 of 39 |
| waiting-self-review | 16 of 43 | none |
| ready-for-human | 9 of 19 | none |

## Objector wording

Added to the same PR after platform #5460 (2026-10-29). A reviewer left a
thread, and the author answered in it without resolving it. The reviewer
then approved, and the author posted `/self-reviewed`. The move comment and
the new status said "Ready for review — re-review or resolve: shumkov". It
read as though the approval had been ignored. Order did not matter: an
unresolved thread is an objection, and an approval does not withdraw it.

The owner's decision is to **keep the rule** and make every surface say
what is actually needed:

| Objector | `asks` (move comment, status, report) | `your_part` | Checklist line |
|---|---|---|---|
| Requested changes | `re-review or resolve: lklimek` (unchanged) | `re-review or resolve your objection` (unchanged) | `lklimek requested changes — waiting for them to re-review or dismiss` (unchanged) |
| Thread only, approved | `resolve every review thread shumkov left open (their approval counts)` | `resolve every review thread you left open (your approval counts)` | `shumkov left a review thread unresolved — resolve every thread they left open (their approval counts)` |
| Thread only, not approved | `resolve every review thread shumkov left open` | `resolve every review thread you left open` | `shumkov left a review thread unresolved — resolve every thread they left open` |

- **"Every review thread X left open"**, not "X's thread": `evaluate`
  counts anyone who spoke in a thread, including a reply in the author's
  own, and writes one line per objector however many threads they hold.
  "Every" stays true for one thread or several, and does not send the
  author looking for a thread X opened.
- **The ask comes first and the approval after it.** A long ready-for-human
  status is cut at 140 characters, and a cut has to keep what is asked.
  The report also joins the asks after "needs" and separates its parts
  with `; `, so an ask must neither read as "needs shumkov approved" nor
  carry a `;` of its own.
- **No new verdict field.** `evaluate` writes one objection line per
  objector: `<login> requested changes`, or else `<login>` +
  `THREAD_LEFT_OPEN` (` left a review thread unresolved`). `policy.py` and
  `engine/src/policy` define that constant once, and both `evaluate` and
  the wording use it. "Approved" means the objector is in some area's
  `approved_by`, the approvals that count on this head. An approval that
  counts for nothing is not mentioned.
- **The fallback is today's wording.** A line the wording does not
  recognise asks for a re-review. It claims no approval and never drops
  the objector. A test pins both phrases `evaluate` writes, so rewording
  either one fails loudly instead of falling back silently.
- **Not changed here:** the dashboard and service (`src/dashboard.rs`,
  `service/src/digest.rs`, `site/js/`). They receive only the objectors'
  names, so they keep "re-review or resolve" for everyone. Making the same
  distinction there needs new fields in `dashboard.json` and the service
  API, which is a follow-up. The service's guard test pins the new
  `main.py` fragment.

## Alternatives rejected

- **Hand-written short phrases per state.** That is a second wording to
  maintain, and it drifts from the comment.
- **`result['blockers'][0]`.**
  - For waiting-bots it says "X has not reported" instead of giving each
    bot's state.
  - It is not the comment's text.
  - Some blockers contain a 40-hex SHA.
- **`waiting-author` as the token.** That would undo 02ab833, which aligned
  the token, the label and the move name. The text after the token now
  says what is owed.
- **Check Runs with `output.summary`.** Branch protection in all five repos
  requires the `PR Hygiene` status context, and statuses serve as engine
  memory. That change belongs to the service cut-over (service spec §3).
- **A link in `target_url`.** The merge box would still show the bare
  state, and the service spec reserves `target_url` for the running
  commit.
- **Keeping Unicode and gating on a test write (V1).** One wrong guess
  about GitHub turns every PR red. The ASCII fold removes the risk for
  four `replace` calls.
- **Cutting at a sentence boundary.** It needs `. ` parsing. A word
  boundary takes one call in each language.

## Every consumer of the description

| Consumer | Reads | After the change |
|---|---|---|
| `GitHub.ready_published` (`github.py:1072`) and Rust `ready_published` (`engine/src/evidence/reader.rs:1158`) | `description == 'ready-for-human'` on the engine's own statuses. This is the build latch (`policy.py:1065`). | **Changes** to `d == 'ready-for-human' or (isinstance(d, str) and d.startswith('ready-for-human: '))`. Older bare statuses cannot be changed and still count. Without this, a ready PR whose build later fails would fall back to waiting-build. |
| `GitHub.latest_state_from_status` (`github.py:1150`) | The newest engine description. `main.py:150` compares it with `'waiting-build'` for PRs that have no record. | **Unchanged**: waiting-build stays bare. It already returns non-state phrases. The Rust copy has no caller, because the build scan is not ported (`engine/src/conformance/run.rs:817`); only `engine/tests/evidence/reads.rs` pins it. |
| Dedup in `post_status` (`github.py:1095`, `writes.rs`) | `(state, description, target_url)` | Code unchanged. See the write-count section. |
| `head_seen_at`, `coderabbit_rate_limited_at`, build reads | `created_at`; CodeRabbit's own context; the build reads skip our context (`github.py:108`) | Not affected. |
| Canonical outputs (`conformance.py:316`, `engine/src/conformance/run.rs:316`) | `status_description(result)[:140]` | Code unchanged. The outputs are regenerated. |
| Differential `own_words` (`engine/src/conformance/live.rs:181`) | Names a would-be status write by its text only when the text is on an allowlist | A description with the move line is not on the list, so the write is named by route only. That is safe: no logins reach the job summary. Matching is by target, so no checking is lost. Naming by token is an optional follow-up. |
| Differential "No write" layer (`live.rs:2481`) | Rust must not write where Python's dry run did not | A PR that Python wrote to is counted as *Unsettled* and is not compared. Every open PR is Unsettled once during the transition. **This layer cannot detect lost deduplication**; see the failure modes. |
| `report --format json` → `policy-state/` → dashboard (`src/`, `site/`) | Evaluation rows | Not affected. `src/fetcher.rs` reads only `statusCheckRollup.state`. |
| `aggregate.py`, the digest, `render_report` | Evaluation rows | Not affected. |
| Branch protection and rulesets | Context and state | Not affected. The state never comes from the description. |
| Sibling A's `hygiene_statuses()` (its §2c backstop) | Would have matched its five fixed phrases exactly | **Not built.** A's §2b and §2c were dropped on 2026-10-29 and deferred to the service. If the service builds such a reader, nothing here is in its way: none of the phrases is a state name, and C leaves them unchanged. |
| Bypass-audit scripts (ad hoc) | The status on the head at merge | Take the part before the first `': '` to get the state; the rest is the reason. |

## Write-count effect

- **Steady state: no extra writes.**
  - When anything needs correcting, `publish` takes the slow path. That
    path always posts `pending: Evaluating current review policy` and then
    the verdict (`main.py:737`, then `:827` or `finish`). The verdict
    always differs from the latest status, so dedup never applies to it.
  - A changed move line makes the move comment incorrect (`move_correct`),
    so it already forces the slow path.
  - With bot-authored self-review moves excluded, there is no fast-path
    case where only the description changes.
  - The text is deterministic: bot names and approvers are sorted, areas
    follow file order, and there are no timestamps or newlines.
- **One-time: +1 status per open non-draft PR in a changed state**, on that
  PR's next run. Live today that is 80 PRs. Events and the hourly sweep (6
  PRs per repo per hour, `event.py:45`) spread them out. That is far below
  the secondary limits. Do not dispatch `scope: all` (`event.py:37`) to
  hurry it.
- **Per SHA:** each run makes as many writes as before, so nothing moves
  closer to the 1000 cap.

## Changes (one PR to `dashpay/stale_prs_are_bad`)

Reader and writer go in **separate commits**, and the PR is merged with a
merge commit, not squashed. This repository uses both styles (#112 and
#115 were merged, #113 and #114 squashed). Keeping the commits lets the
status text be withdrawn without losing the reader: revert every commit
after the reader, newest first. The writer cannot be reverted on its own
once later commits build on it.

- **Python, `main.py`.**
  - Extract `move_line`.
  - `status_description` returns the `too-many-open-prs` text first, which
    avoids recursion. It then maps `waiting-author` to its token.
  - It returns the bare token when there is no move line, when the state is
    `ready-to-merge`, or when the move is the self-review move and
    `self_review.bot_author` is set. Otherwise it returns `clip(token + ': '
    + fold(line))`.
- **Python, `github.py`.** Change `ready_published` as in the consumer
  table.
- **Rust, `engine/src/reconcile/text.rs`.** Mirror the Python change.
  - Measure length with `py_len` (code points, not bytes: the #5494 line is
    140 code points but 147 bytes).
  - Cut with `py_slice(said, None, Some(138))`, then use `rsplit_once(' ')`
    and `str::replace`.
  - Use one shared constant for the limit, `writes.rs::STATUS_DESCRIPTION`,
    and make `conformance/run.rs:318` use it too.
- **Rust, `recheck.rs::post_verdict`.** Compute the description before
  checking `status`. Python evaluates `status_description(result)` as an
  argument, so it runs before `post_status` checks the state.
- **Rust, `engine/src/evidence/reader.rs`.** Mirror `ready_published`.
- **Corpus.** Regenerate in this order, each step prefixed with
  `uv run -q --python 3.12 --no-project --with pyyaml`:
  1. `python -m pr_review.conformance harvest`
  2. `python conformance/pycompat/generate_policy.py`
  3. `python conformance/synthetic/generate.py`

  Rule: outside `functions/main.status_description`, no existing case's
  inputs or output may change. New files are allowed, and so are longer
  `tests` lists, which come from the new tests. Check this with a script
  over `git diff`. `pending.txt` stays empty.
- **Synthetic recordings.**
  - In `rich-evidence`, the newest seeded status (`generate.py:300`)
    becomes the new-format text the engine will compute. That keeps the
    dedup path covered. The older statuses stay as they are.
  - Add a scene, `ready-latched`, whose head has only a new-format
    `ready-for-human: …` engine status, a failed build, and a record that
    does not say ready-for-human. It must stay latched. Add a matching assertion in
    `engine/tests/evidence/recordings.rs`.
- **Docs.**
  - `conformance/README.md`, the `main.status_description` row.
  - `guides/PR_REVIEW_OPERATIONS.md:28` and
    `guides/PR_REVIEW_ARCHITECTURE.md:31`, which describe what the status
    says (64f5077 edited both).
- **No change** to workflows, callers in the governed repos, or policies.
  Every caller is on `@master`, so the change is live in all five repos
  once it merges.

## Failure modes

| Risk | Effect | Guard |
|---|---|---|
| GitHub refuses a description | A 422 on every verdict, so everything turns red | ASCII-only text of 140 characters or fewer. A line that would not be plain printable ASCII after the fold, or that holds `@`, gives the state alone, so this holds by construction. A test asserts it for every description the corpus and the tests produce. |
| GitHub stores the text differently from what was sent | Dedup never matches, so every run writes one more status | Post-rollout checks below. The differential cannot see this. |
| The status text is withdrawn | None. The readers accept both forms. | Revert every commit after the reader, newest first, and keep the reader. A one-line hotfix that makes `status_description` return the token does the same. |
| The whole PR is reverted | Heads whose only `ready-for-human` status is new-format lose the build latch. They show `waiting-build` (pending) until the build is green. This fails closed, and the record comment still latches PRs that stayed ready. | Withdraw the text as above instead. Keep the reader while any open head carries a new-format status. |
| A reader is missed | It silently misreads the status | The consumer table above comes from a `git grep` of `description`, `_head_statuses`, `ready_published`, `latest_state_from_status` and `status_description` across `pr_review/`, `engine/`, `service/`, `src/`, `site/` and the workflows, done independently by two reviewers. |
| A state is shown green or terminal without evaluation | — | The state still comes only from `result['status']`; only the text changes. |

## Tests

The baseline when this was built was 548 Python tests passing (`uv run --no-project
--python 3.12 --with pyyaml python -m unittest discover -s pr_review/tests`)
and the Rust suite (`cargo test -p pr-hygiene-engine`).

**These must fail on today's code and pass after:**

1. `test_checklist.py::test_status_names_the_bot_finding_the_author_owes`
   uses the #1549 shape: a CodeRabbit thread and an attestation. The
   description must equal `waiting-self-review: Your move: coderabbitai
   left review threads unresolved; resolve them.`, and `publish` must post
   it.
2. `test_status_names_which_bots_are_missing` uses the #5494 shape. The
   description must start with `waiting-bots: Waiting for bot review -
   coderabbitai not yet | thepastaclaw done.`
3. `test_status_is_the_move_line_shortened` covers waiting-bots,
   waiting-self-review, waiting-author and a six-area ready-for-human. For
   each:
   - the text after the token, without a trailing `...`, is a prefix of the
     folded move line;
   - it is at most 140 characters, ASCII, and contains no `@`;
   - a cut ends on a word.
4. `test_github.py::test_ready_published_reads_the_state_before_the_next_step`:
   - a new-format status from an engine identity counts;
   - the same text from another identity does not;
   - `ready-for-humans` and `ready-for-human-x: …` do not count;
   - a bare legacy `ready-for-human` still counts.
5. Rust `engine/tests/evidence/reads.rs` mirrors test 4.
6. Rust `tests/reconcile` (function cases and synthetic replays) runs over
   the regenerated corpus. It fails on today's `text.rs`.
7. Rust `tests/evidence` runs over the new latch scene. It fails on
   today's `reader.rs`.

**Guards, which pass before and after:**

- `draft`, `waiting-build`, `configuration-error`, `ready-to-merge` and
  `too-many-open-prs` keep their bare text. This replaces
  `test_nonqueued_status_descriptions_keep_their_existing_meaning`.
- A bot-authored PR in waiting-author gets a bare `waiting-self-review`.
- Dedup is already pinned by `test_should_avoid_republishing_identical_status`
  (`test_github.py:1256`) and by Rust `an_identical_status_is_not_published_again`.
  The new-format case is pinned by the `rich-evidence` recording.

**Existing tests that pin bare text for the four changed states, and must
be updated:**

- **Python, `test_checklist.py`**:
  `test_author_response_check_uses_the_self_review_name`.
- **Python, `test_main.py`**: `test_labels_read_like_the_status` patches
  `move_text` and passes a result with no checklist. It must also patch
  `move_line` or use a real checklist.
- **Python, `test_conformance.py`**:
  `test_the_outputs_are_the_bytes_the_write_path_sends`.
- **Rust**: `engine/tests/reconcile/checklist.rs`
  (`the_second_verdict_sees_the_same_rate_limit_notice`) and
  `properties.rs` (`a_first_admission_completes`,
  `a_reviewer_request_refused_with_422_does_not_stop_the_status`); the
  engine differential's totals, where the `ready-latched` verdict post is
  now deduplicated.
- **Not updated**: the fixtures in `engine/tests/evidence/reads.rs` stay
  bare, because they cover legacy statuses.
  `test_the_status_never_depends_on_the_description_or_the_labels` pins
  `ready-to-merge`, which stays bare.

**After rollout (read-only):**

- The next status on a waiting-bots or waiting-author PR reads as in the
  examples.
- Within about one sweep rotation (around 10 hours on platform), no open
  head shows two adjacent engine `PR Hygiene` statuses with the same state
  and description. That would mean dedup is broken.
- The differential's Unsettled count returns to its pre-rollout level.
- Check in one merge box how much of the text GitHub shows. If only about
  70 characters show, the `Waiting for bot review - ` lead-in pushes the
  bot names to the edge. That would be a follow-up, not a blocker.

## Dependencies on sibling specs

- **A (sweep failure isolation)**:
  - Only A's §2a was built: a failed shared read marks only the PR that
    needs it. Its §2b (re-running what a run left unsettled) and §2c (the
    backstop on the schedules, with its `hygiene_statuses()` reader) were
    dropped on 2026-10-29 and deferred to the service.
  - C does not change the five non-verdict phrases, and none of them is a
    state name, so the two are compatible either way. If a phrase ever
    changes, the differential's `OWN_DESCRIPTIONS` allowlist changes with it.
  - With no backstop, nothing retries a PR whose verdict post failed until
    its next event or the hourly sweep, and a 422 would leave it at
    `error` until then. That is another reason C keeps the text ASCII and
    short.
- **B (build completion trigger)**:
  - C keeps `waiting-build` bare, so B can find waiting-build heads by
    `latest_state_from_status(head) == 'waiting-build'`.
  - Any new reader B adds must use the part before the first `': '`, not
    an exact match, for any state C changes, especially `ready-for-human`.
  - More runs from B do not change C's writes per run.
- **Service roadmap**:
  - The change lives in `engine/src/reconcile/text.rs` and `reader.rs`,
    which the service will run. Both engines keep reading both formats,
    as the service spec requires for statuses used as memory.
  - The service's would-be-write tables (service spec 1b-6) should treat
    the status description like the checklist, that is, hash it, because
    it now carries logins.

## Decisions

Taken by the owner on 2026-10-29, all three as recommended.

1. **Include `ready-for-human`?** Yes. It was the status on
   10 of 59 non-green merges, and it says who must approve.
   - It is the only changed state that the engine reads back, so it is
     what brings in the `ready_published` change.
   - Without it, the change touches the writer alone, and no reader
     changes.
2. **What stays bare.** Confirmed:
   - `ready-to-merge` stays bare.
   - `too-many-open-prs` keeps `Missing human approval`.
   - `waiting-author` keeps the `waiting-self-review` token.
   - Bot-authored self-review moves stay bare.
3. **The ASCII fold.** Confirmed: `—` becomes `-`, `·` becomes `|`, `✓`
   becomes `done`, and backticks are removed; a text over 140 code points
   is cut at a word and ends in `...`. The rejected alternative was to keep
   Unicode and first prove that GitHub accepts it, with one authorized
   scratch-status write.

## Appendix: spec review

Four independent reviewers looked at the spec:

- (a) correctness against the code;
- (b) simplicity and scope;
- (c) failure modes and safety;
- (d) fit with the Rust port, the conformance corpus and the roadmap.

**Must-fixes folded in:**

- Bot-authored self-review moves stay bare (a, c).
- The corpus and synthetic-recording claims are corrected, with a
  regeneration order and an invariant (a, d).
- The full list of tests to update is given (a, d).
- The differential is no longer cited as a guard against lost dedup (c, d).
- The text is ASCII instead of being gated on V1 (c).
- `ready-to-merge` stays bare (b, c).
- One PR with two commits instead of two PRs (b).
- `latest_state_from_status` is left unchanged (b).
- The `own_words` change is dropped (b).
- Also taken: `py_len` and `py_slice` with one shared limit; describing
  the move line correctly (it shows no paths); the `post_verdict` ordering;
  docs; the sibling-A update; not dispatching `scope: all`.

**Rejected:**

- **(b) "Change only waiting-bots and waiting-self-review/waiting-author;
  drop ready-for-human."** Kept as decision 1, with ready-for-human
  included.
  - Ready-for-human was the status on 10 of 59 non-green merges in b's own
    data.
  - Its line names who must approve, which is exactly what someone
    bypassing at that state is missing.
  - The reader change it needs is one condition per language, and if
    reverted it fails closed.
- **(d) A shared public Rust helper for the state token.** Only
  `ready_published` needs it today, and one inline condition is smaller.
  Future readers (B, the service) can add the helper when they exist.
- **(d) Naming verdict writes by token in `own_words`, and updating the
  `CASES` text.** Deferred. Naming is reporting only, and the route-only
  fallback is already safe (b, c).
- **(b) Fold the merge-box look into a pre-merge scratch write.** That
  needs a GitHub write for something that only affects presentation. It
  stays a read-only check after rollout.
- **(c) If Unicode stayed, re-post the bare token after a 422.** Not
  needed, because the text is ASCII.
