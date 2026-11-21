# Dashboard push: verdicts arrive with events; the analyzer reads less often

Status: proposed (2026-11-21). Design only.

## Problem

Every 15 minutes, `pr-hygiene.yml` re-reads every open PR of the five governed
repositories, about 590 GraphQL calls per run. The board shows a verdict as old
as that run, even though the Python engine computed a newer one on the last
event. The engine already holds that newer verdict, and passing it on costs no
extra GitHub reads.

## Decision

**Python pushes its per-PR verdicts at the end of every run**: events, sweeps,
build scans and `scope: all`. The service overlays the newest verdict for each PR
on the latest snapshot. With verdicts arriving on events, the analyzer can drop
from every 15 minutes to hourly.

**Nothing in the dashboard's business logic changes.** Speed, sign-in, accounts,
stage history from record edits, closed PRs, thread severity, `index.md`,
`/ingest`, the analyzer, Pages and `data` all stay as they are.

### Payload

```json
{ "repo": "dashpay/platform", "read_at": "<stamped before collect() starts>",
  "open": [<every open PR number>],
  "verdicts": [ { "number", "head",
                  "verdict": { ...the object `report --format json` prints... } } ] }
```

- **`verdict`** is the dict `evaluate_snapshots` returns, the same row that
  `report --format json` prints and the analyzer parses into `PolicyState`
  (`src/model.rs`). It carries `state`, `status`, `blockers`, `reviewers`,
  `objectors`, `areas`, `approvals`, `ready_since`, `title` and `author`.
- **`open`** comes from `open_prs()`, which every run already calls.
- **`read_at`** is stamped before `collect()` reads anything. If it were stamped
  at `now`, after the reads, a push whose reads came before a snapshot could
  still win against it.
- Neither field needs a new GitHub read.

### Service: one table, overlaid at read time

- **`verdicts` table**, keyed by `(repo, number)`.
  - Its columns are `head`, `read_at`, `verdict`, `stage` and `stage_seen_at`.
  - A push replaces a row only when its `read_at` is newer than the stored one.
  - `stage_seen_at` is set to the push's `read_at` when the pushed stage differs
    from the stored row's stage. It is NULL on a PR's first push, and otherwise
    it is kept.
- **`open_lists` table**, keyed by `repo`, holding the newest `open` set and its
  `read_at`.
- **Where the overlay runs: in `AppState::view()`, at read time.** This is less
  code than doing it at ingest:
  - The cache key becomes (snapshot version, push counter).
  - Every accepted push bumps the counter, including one that only changes an
    `open` list.
  - `View.version` stays the snapshot id, because `api.rs` passes it on as
    `stage_changes(as_of)`.
  - A push writes one row and nothing else.
  - The stored snapshot view stays untouched, so no base copy has to be kept.
  - The overlay runs only when either version moves. It covers hundreds of rows
    on each change.

**The overlay rule.** For a PR in the snapshot view:

- **When the pushed verdict wins:** its `read_at` is later than the repository's
  `data_as_of`, the time that snapshot was generated.
- **What changes when it wins:** only the verdict-derived fields:
  - `engine_state`, `stage`, `draft`, `blockers`, `next_action`, `asks` and
    `objectors`;
  - that PR's entries in every person's `owes`;
  - its author's `wip`, when the stage moves into or out of draft.
- **Everything else** (title, CI, threads, speed inputs) stays from the snapshot.
- **No duplicated logic:** the overlay calls the existing functions in
  `src/dashboard.rs`: `stage()`, `asks()` and `part()`.
- **`stage()` is called with `draft = false`.** The verdict's own `draft` state
  decides, so a PR marked ready after the snapshot leaves Draft.
- **Lateness.** `lateness()` takes the whole `Inputs` today, which the service
  does not have. Its body is split out into
  `lateness_at(stage, since, basis, [late, very_late], now)`, and `lateness()`
  calls it, so the logic does not change. The overlay passes thresholds from
  `view.stages[].late_hours` and `now` = the time the overlay is built.
  `asks` and `lateness_at` become `pub`.
- **Timing.** The first rule that applies sets `since`, with `since_basis`
  `engine`:
  - If `stage_seen_at` is later than `data_as_of`, a stage change happened after
    the snapshot, so `since = stage_seen_at`. This also covers a round trip such
    as review → mergeable → review.
  - Otherwise, if the pushed stage matches the snapshot's, the snapshot's
    `since` stays.
  - Otherwise, `since` = the push's `read_at`.

  The next snapshot reads the exact time from the record edits.
- **When the snapshot wins:** it is newer, so the overlay drops out by itself.
  Its verdict was exported at or after `data_as_of`.
- **Closed PRs:** a snapshot row whose number is missing from an `open` list newer
  than `data_as_of` is hidden. Its entries are also removed from the reviewers'
  `owes` and from the author's `authored` and `wip`. It reappears in the closed
  list on the next snapshot, as today.
- **Pushed PRs not in the snapshot** (opened since the last snapshot) are
  ignored. They appear at the next snapshot, at most an hour later.
- **Speed stays snapshot-only.** It reads its own tables, which only snapshot
  ingest writes. It dates changes from the record edits, so an hourly cadence
  loses no precision.

### Known edge

The snapshot's verdicts come from the export step, which runs a few minutes
before the analyzer stamps `generated_at`. A push read inside that window loses
to a slightly older snapshot verdict until the next push or snapshot. This is
accepted.

## Auth

`/ingest/verdicts` gets a second identity in `service/src/oidc.rs`. The existing
checks stay as they are:

- the RS256 signature against GitHub's keys;
- `iss = https://token.actions.githubusercontent.com` and `aud` = the service URL;
- `exp`, `nbf` and the age of `iat`;
- single use of `jti`.

The new identity also requires:

- `repository_owner_id` is dashpay's id.
- `repository_id` is one of the five configured ids (id → name), and the
  `repository` claim equals the mapped name. `Claims` gains a `repository` field
  for this.
- `job_workflow_ref` = `dashpay/stale_prs_are_bad/.github/workflows/pr-review-reusable.yml@refs/heads/master`.
- **SHA-pinned callers:** `rollout.py` still allows them, and they would get a
  permanent 403. That fails closed. Today all five callers use `@master`.
- **Binding:** the body's `repo` must equal that name. Otherwise the service
  answers 403 and stores nothing, so one repository cannot post for another.

These are not checked, on purpose:

- **`ref`, `workflow_ref`, `event_name`:** `pull_request_target` runs on each
  PR's base branch, and every event posts.
- **`run_attempt`:** a re-run reads fresh data, and `read_at` orders the posts.
- **`runner_environment`:** `DASH_LIGHT_RUNNER` may be self-hosted, and that
  runner already holds a status-write token.
- **`sha`:** it is the governed repository's own commit.

**Why a forged verdict is unlikely.** The job checks out the engine and the
policies from master, never PR code. A writer who edits a caller on a branch
still runs `reusable@master`. Any job that writer adds carries the caller's
`job_workflow_ref`, which the service refuses.

**One relaxation, accepted:** every step of the reconcile job can now mint a
token. The actions are pinned by SHA, and the same job already holds a token
that can write statuses.

**Limits:**
- The service URL is fixed in the reusable workflow.
- The body is at most 1 MB, with at most 1,000 numbers in `open` and 1,000
  entries in `verdicts`.
- Every verdict's number must be in `open`.
- `read_at` must fall between `iat − 1h` and `now + skew`.
- Texts and logins are checked the way `snapshot.rs` checks them.

## A push never affects the gate

1. **Python writes `$RUNNER_TEMP/verdicts.json` and makes no HTTP call.** The
   write is wrapped in `try/except Exception` and only warns. It happens after
   every PR's status write and before the final `reconciliation failed` raise.
2. **A last step posts the file.** It runs with `if: always()`,
   `continue-on-error: true`, `timeout-minutes: 2` and `curl --max-time 20`. It
   exits 0 when there is no file, and any failure is only `::warning::`.
3. **The reusable workflow drops its top-level `permissions:` block.** A callee
   that asks for `id-token: write` its caller did not grant fails to start.
   `pull_request_target` runs the caller from the base branch, so stale callers
   on old `v*-dev` branches would block the gate. Without its own block the
   callee inherits exactly what the caller grants, which today is the same four
   scopes. A caller without `id-token` has no token URL, and its post only
   warns. The two changes can ship in any order.

If a push fails, the board falls back to the snapshot until the next push.

## Analyzer cadence

After a day of live pushes, `pr-hygiene.yml` drops the quarter-hour cron
`"15,30,45 * * * *"`. It keeps `"0 */6 * * *"` (full run) and
`"0 1-5,7-11,13-17,19-23 * * *"` (the hourly quick runs), so it runs every hour
on the hour. That is 4× fewer runs: about 2,360 → about 590 GraphQL calls per
hour, at about 590 per run. The comments in the workflow that say "every quarter
hour" are updated to match.

## Plan

| # | PR | Size | Safe alone because |
|---|---|---|---|
| 1 | Python push (`pr_review/push.py` + tests); the reusable workflow loses `permissions:` and gains the post step; `rollout.py`'s caller template and its test gain `id-token: write` | ~+100 py, +100 test, +25 yaml | Without a token, or before step 3 (404), the post only warns |
| 2 | Five caller one-liners adding `id-token: write` | 5 × +1 | 404 warnings until step 3 |
| 3 | Service: `/ingest/verdicts`, the second OIDC identity, the `verdicts` and `open_lists` tables, the overlay in `view()` | ~+350 with tests | Without pushes the overlay is empty, so the view is today's |
| 4 | Cadence: drop `"15,30,45 * * * *"`, after a day of live pushes | −1 line | Only the analyzer's frequency changes |

## Verification

- **Step 1:**
  - The payload is built from fixture rows.
  - If `push.py` raises, `run()` still publishes and returns 0.
  - Live: each repository's log shows the warning and the status is set.
- **Step 3 tests:**
  - A token from repository A posting `repo: B` is refused with 403.
  - A wrong `job_workflow_ref` is refused with 403.
  - An older push after a newer one is ignored.
  - A push newer than the snapshot changes that PR's stage, its asks and the
    reviewer's `owes`, and changes nothing else.
  - A draft marked ready after the snapshot leaves Draft.
  - A review → mergeable → review round trip takes its new `since`.
  - A newer snapshot wins over an older push.
  - A PR missing from a newer `open` list is hidden, including from `owes`,
    `authored` and `wip`.
  - A push that changes only the `open` list still refreshes the cache.
- **Step 3, live:** after an approval, the board shows the new stage within a
  minute.
