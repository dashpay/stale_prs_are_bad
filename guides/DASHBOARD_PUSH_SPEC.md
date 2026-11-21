# Dashboard push: Python reads, the service stores and shows

Status: proposed (2026-11-21). Design only.

## Problem

GitHub is read twice. The Python engine (`pr_review/`) reads every PR it judges.
Every 15 minutes, `pr-hygiene.yml` then reads all five repositories a second time
with the Rust analyzer (`src/`, about 590 GraphQL calls per run), asks
`pr_review.main report` for verdicts, and posts `dashboard.json` to the service.
That leaves two codebases that each have to understand GitHub and the engine's
record comments.

## Decision

**Python is the only reader. The service is the only store and view.** At the end
of each reconcile run, the reusable workflow posts what the run read: the open-PR
listing and the verdicts it computed. The service keeps the newest of each,
timestamps stage changes itself, and derives the board and people from its store.

### Payload: facts and verdict; the service derives the board

```json
{ "repo": "dashpay/platform", "read_at": "<listing read time>",
  "policy": { "slot_limit": 5, "rosters": { "<area>|fallback": ["login"] },
              "bot_authors": [], "review_bots": [] },
  "open": [ { "number", "title", "url", "author", "author_is_bot", "draft", "base",
              "governed", "created_at", "updated_at", "milestone" } ],
  "rows": [ { "number", "read_at": "<snapshot read time>", "verdict": { ... },
              "mergeable", "build", "unresolved_threads" } ] }
```

- **`open`** comes from `open_prs()`, which every run already calls: events,
  sweeps and build scans alike. `updated_at` and `milestone` come from the same
  listing response, and `mergeable` from the `/pulls/{n}` response the snapshot
  already fetches. A `null` counts as "no conflict known".
- **`policy`** carries what the board needs from the policy: the rosters that
  `people()` walks, the bot logins that `kind()` uses, and the slot limit. The
  service therefore needs no policy reader of its own.
- **`rows`** holds only the PRs this run evaluated: one for an event, about six
  for a sweep, every PR for `scope: all`. `verdict` is the same dict that
  `report --format json` prints today.
- **Caution.** The extra listing and snapshot fields stay out of the PR evidence
  dict and go in a side map on `GitHub`. `policy.fingerprint` hashes every key of
  that dict, so `updated_at` inside it would turn every comment into new evidence.

**Who owns what.**
- **Python owns the verdict and the facts:** about 170 new lines (`push.py` plus
  the side map) and 25 lines of YAML.
- **The service owns `stage()`, lateness, `asks`, `part`, people/WIP and `kind`.**
  These are about 450 lines that move from `src/dashboard.rs` with
  `PolicyState`. They now read the rosters from the stored `policy` and
  `areas`/`unresolved` from the row.
- **Why they live in the service.** Lateness and people need every row plus the
  stage history, and only the service has both.
- **Rejected:** Python sending finished `PrOut` rows. The derivation would then
  be split across two languages.
- **Deleted:** `src/stages.rs`, which reconstructs stage entry from comment edit
  history, along with the fetcher, analyzer, scorer, renderer, history and the
  Rust policy reader. That is about 12k lines.

### Service

- **`prs`** stores the newest row per PR. **`listings`** stores the newest `open`
  set and `policy` per repository.
- **Ordering.** Each listing and each row is ordered by its own `read_at`, and an
  older one is ignored. A late re-run therefore cannot revive a closed PR, and a
  slow sweep cannot overwrite a newer event's verdict.
- **The board** is the newest listing joined with each PR's newest verdict. A PR
  with no verdict shows draft, not-governed or `unknown`, as it does today.
- **Stage history.** When a PR's stage or engine state differs from the stored
  one, the service adds a `stage_changes` row with `observed_at = row.read_at`. A
  PR with no verdict records nothing. Without that rule, the switch would log
  every PR as `unknown` and then log it again, which would reset every `since`.
- **`since`** is the start of the current stage in `stage_changes`. While a PR is
  in review it falls back to the engine's `ready_since`.
- **Snapshots.** Each post also adds a `snapshots` row, so
  `stage_changes.snapshot_id` stays valid without a migration.
- **The page schema stays the same.**
  - The `fetch_error`, `stage_times_error` and `commit` fields are null.
  - `engine_state_available` and `tracked` are always true.
- **Staleness.** A repository with no listing for 2 hours is marked stale.

## What is lost or changes

- **Closed and merged PRs.** Callers already trigger on `closed`. That run's
  listing leaves the PR out, so the PR leaves the board within seconds. If an
  event is missed, the next build scan, at most 15 minutes away, removes it.
- **Dropped: personal speed, and with it sign-in, accounts and opt-out.**
  - **Why:** speed needs closed-PR history, reviewer ids on decisive reviews and
    whole-repository snapshots, and Python reads none of these. Sign-in exists
    only to show people their own speed.
  - **What goes:** `speed.rs`, `auth.rs` and `github.rs` (about 3k lines), nine
    store tables, the OAuth app config, and the page's `account`, `speed` and
    `privacy` scripts.
  - If speed is wanted later, it gets its own spec.
- **Dropped: the `closed` list.** Only speed used it.
- **Dropped: the analyzer's thread classification.** `unresolved_threads`
  becomes a plain count.
- **Changed: `ci_failing`, `merge_conflict` and `unresolved_comments` are set only
  on evaluated PRs.** On drafts and ungoverned PRs they show false or 0. Each
  value is as of the PR's last evaluation: any event re-evaluates the PR, and the
  sweep reaches every PR within about a day. `changes_requested` now means the
  verdict has objectors.
- **Changed: stage entry times.** A change raised by an event is timed to within
  seconds. A change with no event is timed when it is next seen: a build within
  15 minutes, anything else within the sweep's rotation. Today these times are
  read back from comment edit history.
- **Backfill: none.** The service keeps its SQLite file, and `stage_changes`
  already holds each PR's current stage and `since`. After the switch the admin
  dispatches `scope: all` once in each repository. Until then, PRs that have not
  been evaluated show `unknown`.
- **Dropped:** `index.md`, the `data` branch with its nightly snapshots, and
  Pages. The digest workflow stays as it is.

## Auth

`/ingest/rows` gets a second identity in `service/src/oidc.rs`. These checks stay
as they are:

- the RS256 signature against GitHub's keys;
- `iss = https://token.actions.githubusercontent.com` and `aud` = the service URL;
- `exp`, `nbf` and the age of `iat`;
- single use of `jti`.

These are added:

- `repository_owner_id` is dashpay's id.
- `repository_id` is one of the five configured ids (id → name), and the
  `repository` claim equals the mapped name.
- `job_workflow_ref` = `dashpay/stale_prs_are_bad/.github/workflows/pr-review-reusable.yml@refs/heads/master`.
- **Binding:** the body's `repo` equals that name. If it does not, the service
  answers 403 and stores nothing, so one repository cannot post for another.

These are not checked:

- **`ref`, `workflow_ref`, `event_name`:** `pull_request_target` runs on each
  PR's base branch, and every event posts.
- **`run_attempt`:** re-runs read fresh data, and `read_at` orders the posts.
- **`runner_environment`:** `DASH_LIGHT_RUNNER` may be a self-hosted runner, and
  that runner already holds a token that can write statuses.
- **`sha`:** it is the governed repository's own commit.

**Why forging rows is hard.** The job checks out only the engine and the policies
from master, never PR code. A writer who edits a caller on a branch still runs
`reusable@master`, which reads real data. Any job that writer adds carries the
caller's `job_workflow_ref`, and the service refuses it. Forks and other
repositories fail the owner and id checks.

**Accepted relaxation.** Every step of the reconcile job can now mint a token.
Today `pr-hygiene-post.yml` keeps the token in a separate job. The actions are
pinned by SHA, and the same job already holds a token that can write statuses.

**Configuration and limits.**
- The service URL is fixed in the reusable workflow, because callers do not own
  a variable that would hold it.
- The body is at most 2 MB, with at most 1,000 `open` entries and 1,000 `rows`.
- Every number in `rows` must appear in `open`.
- `read_at` must fall between `iat − 1h` and `now + skew`.
- Texts and logins are checked the way `snapshot.rs` checks them today.

## Failure behaviour: a push never touches the gate

1. **Python writes the file and never calls the service.** It writes
   `$RUNNER_TEMP/push.json` inside a `try/except Exception` that only warns, so
   the file cannot change the exit code or skip a write. The write comes after
   every PR's status and before the final `reconciliation failed` raise, so a run
   that partly failed still posts the rows that succeeded.
2. **A last step posts the file.** It runs with `if: always()`,
   `continue-on-error: true`, `timeout-minutes: 2` and `curl --max-time 20`. It
   exits 0 when there is no file and reports any failure only as `::warning::`.
3. **The reusable workflow drops its top-level `permissions:` block.** A called
   workflow that asks for `id-token: write` when its caller did not grant it fails
   to start. That would block the gate on every base branch whose caller is stale,
   and old `v*-dev` branches are among them, because `pull_request_target` runs the
   caller from the base branch. Without its own block, the called workflow inherits
   exactly what the caller grants, which today is the same four scopes. A caller
   without `id-token` has no token URL, so its post step only warns. The two
   changes can therefore ship in any order.

The posted verdict is the computed one, so the board can briefly run ahead of a
status that `publish()` is holding `pending`. The board heals on the next event,
scan or sweep.

## Plan (each step ships alone)

| # | PR | Size | Why it is safe alone |
|---|---|---|---|
| 1 | Engine push: `push.py`, the side map, the post step; the reusable workflow loses its `permissions:` block; `rollout.py`'s caller template and its test gain `id-token: write` | +170 py, +120 test, +25 yaml | Without a token, or before step 4 (404), the post only warns |
| 2 | Five caller one-liners adding `id-token: write` on each default branch | 5 × +1 | The service answers 404 until step 4, which only warns |
| 3 | Drop speed and sign-in: service, tables, page, config, deploy secrets | −4k | Pure deletion |
| 4 | The service runs on rows: `/ingest/rows`, the verifier, `prs`/`listings`, and the board logic moved from `src/dashboard.rs`. Delete the snapshot `/ingest`, `snapshot.rs`, `pr-hygiene.yml` and `pr-hygiene-post.yml` | +600 / −1.5k | One source from deploy on; then `scope: all` in each repository |
| 5 | Delete `src/`, the root crate, `tests/`, the analyzer jobs in `rust.yml`, and the Pages assembly | −12k | Nothing reads them after step 4 |

A week after step 5, the admin disables Pages and deletes the `data` branch.

## Verification

- **Step 1 tests:**
  - The payload is built from fixture snapshots.
  - A snapshot's fingerprint is unchanged once the side map exists.
  - When `push.py` raises, `run()` still publishes and returns 0.
- **Step 1, live:** in each repository's log, the post step's warning appears and
  the status is set.
- **Step 4 tests:** each of these is refused or ignored as stated:
  - a token from repository A posting `repo: B` (403);
  - a wrong `job_workflow_ref` (403);
  - an older post arriving after a newer one (ignored);
  - a PR missing from a newer listing (removed);
  - a listed PR with no verdict (no `stage_changes` row).
- **Step 4, live:** the board fills within 15 minutes of deploy.
