# Sweep failure isolation (prompt re-evaluation dropped)

> **Note (2026-11-21):** the Rust port of the engine (`engine/`), its conformance corpus and the differential were removed; the parts of this spec about them are abandoned.

**Status:** v3, as built.
- PR 1 (§2a) is built, in both engines.
- PR 2 (§2b, §2c) was **dropped on 2026-10-29 and deferred to the service**,
  which has its own queue (§8). Its sections are kept as the record of the
  design.
- §2d stays deferred.
- The owner's decisions are in §9.
- Draft v1 was revised into v2 after a five-lens spec review, and a second
  round re-checked v2 (Appendix A).
- Line numbers refer to master at c76976f, before PR 1.

**Scope:**
- what a run does when it cannot read GitHub (built, §2a);
- a head the engine itself leaves at "reconciliation required" or an evidence
  error (§1b; dropped 2026-10-29, deferred to the service).

Sibling specs own build completion (B) and status wording (C).

## 1. Problem and evidence

### 1a. One failed read turns a whole run's PRs red

`collect` (`pr_review/main.py:114-184`) reads every candidate's history in one
batched GraphQL query (`load_histories` → `GitHub.histories`,
`github.py:543-615`). If that read raises, the outer handler
(`main.py:178-184`) posts `error: Incomplete policy evidence; reconciliation
required` on **every selected PR**, then re-raises.

| Run | `selected` (everything marked when the read fails) |
|---|---|
| `--pr N` (every PR event) | every governed PR by N's author |
| `--batch-size 6` (hourly sweep, :17) | every governed PR by the authors of the 6 PRs in the batch |
| `--waiting-on-build` (build scan, :02/:32/:47) | **every governed PR** |
| no selector (`scope=all` dispatch) | every governed PR |

Failures inside one PR's own snapshot are already isolated
(`main.py:165-176`): only that PR is marked.

**Live evidence, dashpay/platform:**

- Every burst traces to a run that failed in `load_histories` →
  `histories()`, with HTTP 504, HTTP 502, `stream error … CANCEL`, or the
  60 s subprocess timeout.

  | Burst | Run | Kind | Error |
  |---|---|---|---|
  | 10-12 11:59–12:0x | 38246080742 | build scan | attempt 1 HTTP 504; re-run by hand 33 min later |
  | 10-13 19:2x | 38453490330 | build scan | HTTP 504 |
  | 10-14 17:1x | 38605278785 | build scan | HTTP 504 |
  | 10-16 19:1x | 38953498149 | build scan | HTTP 504 |
  | 10-20 22:5x | 39511005381 | build scan | attempt 1 stream CANCEL; re-run by hand 38 min later |
  | 10-22 06:3x | 39734624364 | build scan | HTTP 504 |
  | 10-23 15:0x | 39973969260 | build scan | 60 s timeout |
  | 10-23 19:2x | 40011155953 | hourly sweep | stream CANCEL |

  A build scan is recognisable because its target checkout was skipped. A
  manual re-run of a build scan repairs nothing, because it reconciles only
  `waiting-build` PRs.
- **Size of the damage:**
  - On the current heads of all 110 open platform PRs, statuses posted since
    10-20 include 201 `Incomplete policy evidence` errors and 7 `success`.
  - Bursts covered 31–62 PRs each.
- **PR events fail the same way.** Six review and `pull_request_target` runs
  between 10-26 18:01 and 18:31 failed in `histories()` with 502, 504 or a
  truncated response. Each marked every PR by one author:
  - QuantumExplorer has 17 open PRs; PastaPastaPasta has 16.
  - #4730, #4899, #4933 and #5569 are still `error` today, about 15 h later
    (as of 10-27 09:40 UTC). Two of them are drafts.
- **Recovery is slow.** It waits for the sweep, which reconciles 6 PRs an hour
  out of 110. A full rotation takes about 18 h. #5242 was red from 10-16
  19:17 until 10-17 07:26, about 12 h. On platform this is the only required
  status, so people bypassed it.

**Why that query fails.** Replaying the scan's query against platform today
returns 2.1 MB in 6.1–7.3 s (3 tries). GitHub ends any request at 10 s with a
502 or 504, and charges extra rate-limit points for it
(<https://docs.github.com/en/graphql/overview/rate-limits-and-query-limits-for-the-graphql-api>,
"Timeouts"). One author's query (17 PRs, about 230 KB) takes about 1.5 s.

**No retry for GraphQL.** `_run`/`_transient` (`github.py:139-148`, `428-433`)
retry once, after 2 s, and only for GET/PUT/PATCH/DELETE. Every GraphQL read
goes through `request("POST", "graphql", …)` (`github.py:571`, `645`, `699`,
`779`), so histories, threads, build state and comment pages are **never
retried**. The Rust client copies that rule (`engine/src/evidence/client.rs:92`;
`transport.rs:33`), and so does the service transport
(`service/src/reader/transport.rs:1313`). §2d defers changing it.

**"Admission depends on all candidates" does not justify the wide mark.**
- `admit()` (`policy.py:183-197`) groups PRs by author.
- `context_fingerprint` and `admission_valid` read only that author's PRs.
- In `evaluate`, `too-many-open-prs` is gated only inside `if human:`
  (`policy.py:1068-1076`). That branch always ends at the pending
  `ready-for-human` gate (`:1103`).
- `success` is returned only at `:1130`, where no human review is needed.
- `publish`'s `admission_valid` re-check (`main.py:647`, `656`) can only turn a
  ready head *pending*.

So admission can never be the reason a head is `success`. Marking the other
PRs keeps none of them from passing wrongly. It only blocks merges that were
correctly gated.

### 1b. "Reconciliation required" is never followed up

The engine leaves a head unsettled in these places:

| Posted by | Status |
|---|---|
| `publish` (`main.py:645`, `660`) | pending `Review evidence changed; reconciliation required` |
| `publish` (`:648`) | pending `PR admission context changed; …` |
| `publish` (`:664`) | pending `Policy changed; …` |
| `run` (`:1045`) | error `Policy reconciliation failed; inspect workflow log` |
| `_mark_unreadable` (`:191`) | error `Incomplete policy evidence; reconciliation required` |
| `publish`, after `Evaluating current review policy` (`:737`) | early returns at `:739`, `746`, `756`, `789`, `813`, `826` leave that status if `base_sha` or `draft` changed under the run; a merge into the base raises no PR event |

After any of these, nothing comes back to that head. It waits for its next
event or the rotation. PR 1 does not change this. §2b and §2c were the
answer, and both are now deferred to the service (§8).

Events do not cover the commonest cause, a race between two runs for one
author. Many evidence changes raise no event the callers listen to: human
comments are filtered out, and thread resolution is not subscribed.

| PR | Left at | What happened | Merged by bypass |
|---|---|---|---|
| tenderdash#1539 | 10-19 13:14:05 `Review evidence changed` | The approval's run (39258288242) failed its admission re-check, because the same author's other PR closed during the run (39258398055). That run did not include #1539, since #1539's slot did not change. | 13:21, after 7 min |
| tenderdash#1513 | 10-23 07:16 `PR admission context changed` | — | 07:26, after 9 min |
| platform#5396 | 10-19 10:26 `Review evidence changed` | — | 10:32, after 6 min |
| tenderdash#1565 | 10-22 17:02 `Policy reconciliation failed` | `unexpected end of JSON input` mid-publish (39819133719) | repaired at 17:26 by the next event |

**People bypassed within 6–9 minutes.** Platform's schedules over the last
7 days:
- 594 of the 672 expected scheduled runs fired (88%, in the 7 days to
  10-27 08:50 UTC);
- gaps between consecutive scheduled runs: p50 14 min, p90 32, p99 59,
  max 127.

GitHub documents both effects: "The schedule event can be delayed during
periods of high loads … some queued jobs may be dropped"
(<https://docs.github.com/en/actions/reference/workflows-and-actions/events-that-trigger-workflows#schedule>).
A schedule alone cannot beat those bypasses.

## 2. Approach

**Budget.** dashpay is on GitHub's Free plan (`gh api orgs/dashpay` →
`plan.name: free`). GITHUB_TOKEN therefore has 1,000 REST requests and 1,000
GraphQL points per hour per repository, shared by every workflow in that
repository
(<https://docs.github.com/en/rest/using-the-rest-api/rate-limits-for-the-rest-api>).
One `sync --pr N` costs about 45–50 calls: the synthetic
`conformance/synthetic/sync-pr-2/calls.jsonl` has 36 REST and 11 GraphQL.
Every bound below follows from that.

### 2a. A failed shared read marks only what it must (PR 1)

The outer handler in `collect` changes by selection:

| Selection | Today | After |
|---|---|---|
| `--pr N` | every governed PR by N's author | **N only**, if N is in `selected` |
| `--batch-size` (sweep), `--waiting-on-build` (scan) | every selected PR | **nothing**; the job fails |
| no selector (`scope=all`, local full `sync`) | every selected PR | **unchanged** |

`_mark_unreadable` itself does not change. It still writes on a head shared
with another PR, which fails closed and matches `evaluate_snapshots`
(`main.py:898-903`). It must not copy the shared-head and draft guards of
`_mark_configuration_error`.

**Why each row is the smallest safe blast radius:**

- **`--pr N`.** The event says N's evidence changed, so N's old status may be
  stale and must not stay green. The author's other PRs had no event. Their
  slots cannot change their merge gate (§1a), so their last verdicts are as
  good as before the run.
- **Sweep and scan.** No PR is known to have changed. The work just did not
  happen. The sweep's rotation is chosen by the clock (`main.py:91-102`), so a
  batch whose read fails is skipped for a whole rotation (about 18 h on
  platform). Before PR 1 the same batch sat red for the same time, and
  blocked merges as well. The build scan looks again at its next slot.
  Changes that raise no event (a revoked permission, a reopened thread) go
  unchecked for that rotation, as they already do on any PR the rotation has
  not reached yet. A push to the default branch and a plain
  `workflow_dispatch` run the same batch (`event.py:42`). It is still chosen
  by the clock, not by what the push changed, so the same reasoning holds.
- **Full pass.** Its purpose is to replace every head's status with one
  computed now, after a policy or engine change (`event.py:36-40`,
  `main.py:1037-1040`). If its read fails, no pre-existing green can be
  trusted, so it keeps failing closed. It is started by an operator, who sees
  the failed job.

**What a failed shared read leaves on each PR's status:**
- Event run: N shows `error: Incomplete policy evidence; reconciliation
  required` until the next event on N, or until the sweep reaches it. Every
  other PR keeps its status. A closed N, or one on an ungoverned base, is
  not in `selected`, so nothing is written.
- Sweep or scan: every PR keeps its status.
- Full pass: every governed PR is marked, as today.
- In every case, records, slots and labels are untouched.

**Writing nothing, or only some PRs, cannot corrupt admission:**
- A slot is stored in the PR's own record comment, and only `publish` writes
  it, for that PR.
- No author's slots depend on another author's PRs.
- Within one author, publishing a subset already happens today when one
  snapshot fails. Slots come from histories, and `admit` is deterministic over
  the records.
- `admission_conflicts` settles any over-count from racing runs.
- §2a adds no partial-write state: when the shared read fails, nothing is
  published.

**What it gives up:**
- If N's run fails, the author's PRs whose slot should change (because N
  closed and freed one) wait for their next event or the rotation. This
  affects reviewer routing only; no merge gate moves.
- With PR 2 dropped, N itself stays at `error` until its next event or the
  rotation. That is up to about 18 h on platform, for one PR instead of every
  PR by its author.

### 2b. Re-run, inside the job, what the run left unsettled (PR 2: dropped)

**Dropped (2026-10-29): deferred to the service.** Retry plumbing on the
Actions side is moot once the GitHub-App service lands, because the service
has its own queue. Kept as the record of the design.

**The signal (Python only, no requests).** A PR the run touched counts as
*unsettled* until its verdict is posted. `main.py` gets two module-level
sets, `UNSETTLED` and `SETTLED`. `run()` empties both in place with
`.clear()` when it starts, because `event.py` imports the names.

| Edit | Where |
|---|---|
| add the PR's number before `nudge`/`publish`, under `apply` only | the `run` loop (`main.py:1016-1031`) |
| move it from `UNSETTLED` to `SETTLED` right after the verdict post | `publish`, 3 sites: `666`, `734`, `827` |
| add the number on entry, before its re-read | `_mark_unreadable` |

This covers every row of the §1b table, including the early returns after
`Evaluating …`, and also a mark whose own re-read failed. It covers any
future post site too, because the rule tracks the absence of a verdict rather
than any particular phrase. The GitHub boundary does not change.
Recordings never read the set.

**The policy, in `event.main` (`event.py:59-72`):**

1. Run every selection.
   - Catch `GitHubError`, `ValueError`, `KeyError` and `OSError` per selection
     (the set `main.py:1065` catches), and keep the first.
   - Copy `UNSETTLED` and `SETTLED` in a `finally` after each `run()`.
2. Build the retry list:
   - the event's own N, if its run raised `GitHubError`;
   - every unsettled number, the event's N first;
   - a sweep or scan selection that raised `GitHubError` and left nothing
     unsettled (its shared read failed), run once more as a whole. A
     selection that raised because one PR's publish failed (`main.py:1049`)
     is not re-run; only its unsettled numbers are retried;
   - never a full pass, and never a selection that raised anything else (a
     configuration error is deterministic).
3. Only if all of these hold:
   - the run has `--apply`;
   - the list is not empty;
   - `GET rate_limit` shows at least 300 REST and 200 GraphQL remaining.
     Per the REST docs, this endpoint does not count against the limit. The
     check is made **before every run**, not once.

   Then:
   - sleep 30–45 s, with jitter so two racing jobs do not collide again;
   - make **one round**: at most 3 PR numbers (the event's N first), or one
     sweep or scan re-run;
   - carry on past a retry that raises, because `run` also raises for
     failures that repeat every time (`main.py:63`, `1049`), and stopping
     there would starve the rest;
   - stop starting runs when the 3-minute budget runs out or the floor is
     reached.
4. **Last resort**, only when all of these hold:
   - writes are on, the same condition that adds `--apply`
     (`event.py:66-67`);
   - N's last attempt raised `GitHubError`;
   - N is not in that attempt's `SETTLED`.

   The usual case is a listing that failed twice. Then `event.py` calls a
   new `main.mark_unreadable(repo, policies_root, N)`, which loads the
   registered policy and calls `_mark_unreadable`. That costs one GET, one
   statuses read and one POST, with no listing and no GraphQL. Without it, a
   failed event run would leave N's old green standing, and §2c cannot see a
   green.

   The `SETTLED` condition stops a raise caused by another of the author's
   PRs from overwriting the verdict N did get. Configuration errors never
   reach this step (step 2).
5. **Job result.** Fail if anything's last attempt raised. Otherwise succeed,
   even if the first attempt raised. For PR events the reusable job appears as
   a check on the PR, and a red job next to a repaired status misleads
   (decision D2). Log every retried number, and any that is still unsettled,
   to stderr.

`event.py` is the Actions entry point. Neither the conformance corpus nor the
Rust port runs it, and the service replaces it with its own queue (§8).
Keeping retries out of `main.run` means no sleeps in recordings and no Rust
mirror.

**Timing.** In tenderdash#1539, the competing run finished 25 s after the
unsettled post. A 30–45 s retry would have published the real verdict before
the bypass.

The 10-26 failures were interleaved with successes: head 175505b went green at
18:18:26 and 18:20:18, around a failure at 18:18:57. A 30 s retry has a real
chance, but failure clusters last 30–40 min, so §2c has to carry the long tail.

All of these timings assume the job starts promptly. Run 38541303282 sat in
the queue for 2 h.

**Cost:**
- No requests when nothing is unsettled, which is the usual case.
- Otherwise, at about 45–50 calls per `--pr` run, up to about 150 calls for 3
  PRs, or about 270 for a sweep re-run of 6 PRs.
- At most 3 minutes of runner time after the sleep. The floor is checked
  before every run.
- The retry runs inside the event's per-PR concurrency group
  (`pr-review-reusable.yml:18`, `queue: max`), so the next event on that PR
  waits up to about 4 minutes longer.

### 2c. Backstop on the existing schedules (PR 2: dropped)

**Dropped (2026-10-29): deferred to the service**, for the same reason as
§2b. Kept as the record of the design.

This catches what §2b could not: a cancelled job, an outage longer than the
round, the round's cap, and a head left at `Evaluating …` by one of
`publish`'s early returns. It also runs in `event.py`, after the selections
and §2b, on **schedule events only** (both crons). It needs writes on, the
same gate the scan uses (`event.py:24-27`). It adds no new selection, CLI flag
or `collect` branch.

**The read:** `GitHub.hygiene_statuses()`, a new Python-only reader.
- One GraphQL query, paginated over `pullRequests(states: OPEN, first: 100)`.
- It fetches `number`, `baseRefName`, `headRefOid` and
  `commits(last:1){nodes{commit{oid status{context(name:"PR Hygiene"){state
  description createdAt creator{login __typename}}}}}}`.
- A live test on platform cost 1 point and took 2–3 s per 100 PRs, with no
  comment bodies.
- Like the other readers, it refuses an `errors` array and a cursor seen
  before. Unlike them, it de-duplicates by number rather than failing on
  pagination drift, because this read only chooses what to look at.
- `Status.context` and `StatusContext.creator` are nullable in the schema. On
  platform, 18 of 110 heads have no context. A null `status`, `context` or
  `creator` means "not picked" and is never an error. `_graphql_login` is
  applied only when `creator` is an object; it raises on anything else.

**The pick.** All of these must hold:

- the PR is governed: `governs(policy, baseRefName)`, with the policy loaded
  as `run` loads it;
- `commit.oid == headRefOid`;
- the creator, after `_graphql_login`, is `github-actions[bot]`, the Actions
  identity. The service's App identity is excluded on purpose: once the
  service writes, it owns its own retries;
- the description is `Evaluating current review policy` or one of the five
  unsettled phrases. The phrases are kept in one constant shared with the
  writers:

  ```
  Review evidence changed; reconciliation required
  PR admission context changed; reconciliation required
  Policy changed; reconciliation required
  Incomplete policy evidence; reconciliation required
  Policy reconciliation failed; inspect workflow log
  ```
- the head's **unsettled streak** fits an age tier. The streak is the run of
  newest engine `PR Hygiene` statuses on that head whose descriptions are
  phrases or `Evaluating current review policy`. Its age is the age of the
  oldest status in it, taken from one REST statuses read per candidate. That
  is the existing cached `_head_statuses`, and only heads that already passed
  the checks above are read, usually none.

  The latest status's `createdAt` will not do. A publish that fails the same
  way every time posts `Evaluating …` and then the phrase again on every pick,
  so `createdAt` never ages. That head would be re-picked 4 times an hour for
  ever, and it would reach GitHub's limit of 1,000 statuses per SHA and context
  ("Attempts to create more than 1000 statuses will result in a validation
  error", <https://docs.github.com/en/rest/commits/statuses>) in about 5 days.
  After that, not even a correct verdict could be posted on it until a push.
  Streak age bounds a stuck head to about 84 statuses.

  | Age | Picked on |
  |---|---|
  | under 5 min | never: §2b's round owns it |
  | 5 min to 6 h | every scheduled run |
  | 6 h to 24 h | the :17 sweep only |
  | 24 h or more | never: a persistent failure, left to events, the rotation and `scope=all` |

  The tiers start from the streak's beginning, not from its latest status.

**Bounds:**
- At most **4** picks per run, youngest streak first. Fresh failures go
  before stuck ones, and a stuck head ages out through the tiers.
  `periodic_batch` would not rotate here: its start is `(k*4) % len`, always 0
  for 2 or 4 picks.
- Each pick is one `run(options + ['--pr', n])`.
- Picks are not retried in the job; the next scheduled run is their retry.
- The floor is checked before every pick.
- A pick that raises does not stop the others. The backstop stops when its
  4-minute budget runs out or the floor is reached.
- Idle cost: 2 GraphQL points per scheduled run on platform, about 8 an hour.
  REST and GraphQL are separate budgets, and REST is the one that binds. At the
  10-26 peak there were 41–50 policy runs an hour, so the headroom is not
  known. The floor is what protects the budget. A 403 or 429 is never treated
  as something to retry: the floor check comes first.
- Worst case (a mass event, while still above the floor): 16 picks an hour,
  about 800 calls.

**Worst-case time to re-evaluation,** for a head left unsettled or at an
evidence error:

| Case | Time |
|---|---|
| In the job, GitHub answering | 45–90 s |
| Backstop, nominal | eligible after 5 min, then the next of :02, :17, :32, :47, so ≤ 20 min |
| Backstop, measured | the 5-min wait plus the scheduled-run gap: p90 ≈ 37 min, max ≈ 2 h 10 min (platform, last 7 days) |
| Mass event | 16 heads an hour |
| Today | about 18 h for the rotation; observed 12 h (#5242), and 15 h and counting for the four heads still red |

### 2d. Retrying GraphQL reads: deferred

A GraphQL `query` has no side effects, and the engine sends no mutations.
Retrying one 502/504 would let a sweep or a single PR survive a blip. It is
**not part of PR 1**. The change touches a contract both engines share, so it
is listed for a separate decision (D3), which stays deferred (§9).

Findings for whoever takes it on:
- Both engines check for a retry *before* checking for a partial answer
  (`github.py:428` vs `:434`; `client.rs:92` vs `:99-110`). Reorder both, or a
  partial answer whose comment bodies contain `timeout`/`EOF` gets retried.
- `_transient`'s substring signs are too broad: `502` matches PR numbers
  5020–5029 in error text. They are also too narrow: they miss
  `stream error … CANCEL`, and the 60 s timeout raises before `_transient`
  runs. Even with the retry, 3 of the 8 bursts in §1a would not have
  recovered: two stream CANCELs and one timeout.
- The service transport decides "transient" from the status code, not the
  text (`service/src/reader/transport.rs:523-534`). The two engines would
  disagree on a GraphQL 200 that carries a timeout error.
- Artefacts to change:
  - `conformance/README.md:687-693` and `:892-896` (the contract);
  - `engine/src/evidence/{client.rs:67-110,mod.rs:9-10,transport.rs:28-44}`;
  - `service/src/reader/transport.rs:1313,1409`;
  - `engine/src/conformance/live.rs:306-310`;
  - the `transient-retry` and §4's `history-unreadable-*` recordings;
  - the counts in `engine/tests/differential/main.rs:76-96`;
  - tests in `engine/tests/evidence/boundary.rs`.

## 3. Alternatives rejected

The entries that weigh something against §2b or §2c are kept as the record
of the dropped design.

- **Keep marking the author's other PRs on events.** Admission cannot change
  any of their merge gates (§1a). Because of it, the four platform heads had
  been red for 15 h as of 10-27.
- **Read histories one author at a time, so a sweep can go on for the other
  authors.** That changes the batched query text and the request order in
  every recording with more than one author, and needs a Rust mirror. The
  service already reconciles one author at a time. The safety argument for it
  is §2a's "Writing nothing, or only some PRs, cannot corrupt admission".
- **Retry with a follow-up `workflow_dispatch`.** GITHUB_TOKEN can start one,
  but:
  - the callers grant no `actions: write`, and a reusable workflow cannot get
    more than its caller grants;
  - the callers' dispatch input has only `scope`, so it cannot name a PR;
  - it would need a PR in each of the 5 governed repos, plus guards against
    dispatch loops.

  `repository_dispatch` has the same problem: it needs `contents: write` and a
  new `on:` entry in every caller.
- **The backstop alone.** Bypasses came within 6–9 min; scheduled-run gaps
  reach p90 32 min.
- **The in-job round alone.** A cancelled job, or an outage longer than the
  round, leaves the head waiting for the rotation again (§1a).
- **A `--unsettled` CLI selection inside `collect`** (draft v1). It needs a
  `collect` branch, CLI-flag exclusivity, `requested` narrowing and conformance
  argv naming. The `event.py` loop does the same work with none of that, and
  reads histories per author, the size that does not time out.
- **Detecting unsettled heads by reading statuses back, after every event
  run or for every head over REST** (`latest_state_from_status`):
  - It costs requests on every run, or 440 an hour for 110 heads.
  - It misses a mark whose own re-read failed.
  - The in-process rule costs nothing.
- **Retrying inside `main.run`.** Recordings call `run` with the write path
  on, so retries would mean sleeps and repeated runs in the corpus, and a Rust
  mirror.
- **An age threshold on the latest `Evaluating …` status** (draft v1).
  `post_status` skips a post identical to the latest status, so a live run's
  `Evaluating` can carry an old `createdAt`. Streak age (§2c) replaces it.
- **Using `pending` instead of `error`, or rewording the phrases.** C leaves
  the phrases to A. Rewording them would change `OWN_DESCRIPTIONS` and
  `FAILURE_DESCRIPTIONS` (`engine/src/conformance/live.rs:148-219`), and the
  backstop would have to match old and new text. D4 (2026-10-29) keeps
  `error` and today's wording on the event's PR. Without §2b, that error
  lasts until the next event or the sweep, not about a minute.

## 4. Interface and data flow

### PR 1: Python + Rust (§2a)

**Python:**
- `main.collect`: the outer `except GitHubError` branches on `number`,
  `batch_size` and `waiting_on_build`, per the §2a table. The comment at
  `:180-181` becomes the §2a reason.

**Rust:**
- `engine/src/reconcile/collect.rs:172-180` matches on the selection:
  - `All`: mark every selected PR (unchanged);
  - `Pr(n)`: mark n if it is in `selected`;
  - `Batch`: mark nothing.

  Update the doc comment at `:93-98`. The build scan is not ported
  (`run.rs:155` refuses unknown argv).
- Stale comment at `engine/tests/reconcile/checklist.rs:1093-1094`.

**Corpus:** two new synthetic recordings. A recording is one run with one argv.

| Recording | Run | Result |
|---|---|---|
| `history-unreadable-pr` | `sync --pr 2 --format json`; a third PR by `reviewer` on head `'d'*40`, with `reviews[3]=[]` and `history[3]=[]`, set in the scenario function, not in `Scenario.__init__` | only `statuses/cccc…` is written |
| `history-unreadable-batch` | `sync --batch-size 1 --format json` | no write |

- Both use `gh.failing_at[('POST','graphql')] = {1: …HTTP 504…, 2: …HTTP 504…}`,
  so they stay meaningful if §2d ever lands.
- Add both to:
  - the `generate.py` docstring and `SCENARIOS`;
  - `conformance/README.md:228-232`;
  - a guard in `engine/tests/reconcile/recordings.rs:311`.
- Bump the counts in `engine/tests/differential/main.rs:76-96` from 13 to 15
  recordings, plus the **all** row.
- `pending.txt` does not grow.

**Docs:**
- `PR_REVIEW_OPERATIONS.md:55` (sweeps) and `:86`. Say that a read failure
  follows the §2a table, which is *not* the configuration-error rule: a read
  failure keeps marking shared heads and drafts.
- `PR_REVIEW_ARCHITECTURE.md:29`.

**As built, beyond the above:**
- The build scan's own per-head status read (`latest_state_from_status`, for
  a PR with no record yet) is inside the same handler. If it fails, the scan
  also writes nothing.
- `--pr N` marks N only if N is in `selected`, the governed open listing. A
  closed N, or one on an ungoverned base, gets nothing. The Python test and
  a third Rust test pin this
  (`an_unreadable_history_marks_nothing_when_the_pull_request_asked_for_is_closed`).
- The `all` row of the differential counts becomes
  `| **all** (15 recordings) | 18/18 | 18/18 | 15/15 | 29/29 | 15/15 | 69/69 | 15/15 | 4/4 | 15/15 | 15/15 | 0 | 445 | 0 |`.
- No harvested case under `conformance/evaluate` or `conformance/functions`
  changed, and neither did `conformance/pycompat`. The new tests reach no
  harvested function.

### PR 2: Python only (§2b, §2c) — dropped (2026-10-29): deferred to the service

**`main.py`:**
- `UNSETTLED`, `SETTLED` and their three edits;
- `UNSETTLED_DESCRIPTIONS`, the five phrases, used by the writers and the
  backstop;
- `mark_unreadable(repo, policies_root, number)`.

**`github.py`:** `hygiene_statuses()`.

**`event.py`:**
- `main` implements §2b and §2c.
- Constants: `RETRY_AFTER = (30, 45)`, `RETRY_PRS = 3`, `BACKSTOP_PICKS = 4`
  (youngest streak first),
  `RETRY_BUDGET_SECONDS = 180`, `BACKSTOP_BUDGET_SECONDS = 240`,
  `FLOOR_REST = 300`, `FLOOR_GRAPHQL = 200`, and the age tiers.
- `selections()` is unchanged.

**Rust, conformance and the service:** no change.
- `hygiene_statuses` is not in `engine/src/evidence/queries.rs`, and not in
  the service read-only allowlist (`service/src/reader/read_only.rs:225-228`).
  It stays Python-only.
- Re-harvest and commit `conformance/functions` if the new publication tests
  reach harvested functions. The `pr-review-tests.yml:36-47` freshness gate
  will say so.

**Docs:** the Operations guide gains one paragraph on the in-job round and the
backstop.

**Workflows and callers:** none in any of the five governed repos, and none in
`pr-review-reusable.yml`.

## 5. Failure modes

As built (PR 1). The modes of the dropped §2b and §2c (retry rounds, the
backstop, `hygiene_statuses`, the rate floor) no longer apply.

| Mode | Result |
|---|---|
| Event run, shared read fails | Only N is marked `error`. The author's other PRs keep their status. The job fails. N gets its real verdict from its next event or when the sweep reaches it. |
| N is closed, or on an ungoverned base, when the read fails | N is not in `selected`, so nothing is written. The job fails. |
| N is a draft, or shares its head with another open PR | N is marked, as before. A draft cannot merge. Any run that evaluates a shared head marks it `configuration-error`, so this matches; the error shows on the other PR on that head too. |
| N moved between the listing and the mark | `_mark_unreadable` re-reads N and writes on its current head, as before. A push raises its own event. |
| The mark itself fails | It is logged to stderr, as before. N keeps its old status, which may be green, and the job fails. |
| Sweep or scan, shared read fails (history query, or the scan's per-head status read) | Nothing is written. The job fails, which shows only in the Actions tab: GitHub notifies just the user who last edited the cron. The scan looks again at its next slot. A failed sweep batch is skipped for one rotation, about 18 h on platform. |
| Full pass, shared read fails | Every governed head is marked, as before. The operator who dispatched it sees the failed job. |
| A failure that repeats on every run (a malformed or foreign record comment, which raises inside `load_histories`; a query that has grown past GitHub's limit) | Every sweep whose batch includes that author, and every build scan, fails and writes nothing, every time. Before PR 1 this was loud (every selected PR went red); now it shows only as repeated failed scheduled runs, so the PRs in those batches go unswept until someone fixes the cause. The Operations guide says to look into repeated failed scheduled runs. Marking the PR the error belongs to, as per-PR evidence errors already are, is a possible follow-up. |
| Fail-open | Runs leave unmarked only PRs that had no new event: the author's other PRs on an event run, and everything on a sweep or scan (§2a). Every status they leave was posted by a run that read its evidence. A change that raises no event (a revoked permission, a reopened thread) goes unchecked until the rotation comes back, as it already does on any PR the rotation has not reached; before PR 1 a failed sweep marked those PRs red instead. |
| Never green without evaluation | PR 1 only removes error writes. It posts no verdict. |
| Records, slots and labels | Untouched in every case: nothing is published when the shared read fails. |
| Job exit status | Unchanged. `collect` re-raises in every selection, so every such run fails. |

## 6. Rollout

- Every governed repo's caller runs `@master`, so each merge goes live in all
  five at once. Rollback is a revert on master.
- No PR is needed in any governed repo.

Steps:

1. **PR 1, §2a.** This alone ends the bursts.
2. ~~PR 2, §2b and §2c.~~ Dropped (2026-10-29): deferred to the service.
3. Heads that earlier bursts left red are not repaired by PR 1. They wait for
   their next event or the rotation. Where that is too slow, one `scope=all`
   dispatch on the repository replaces every head's status at once. It sends
   the largest form of the history query, the one that failed in 7 of the 8
   bursts. If it fails, it marks every governed head, so check that its run
   succeeded before treating it as a repair.
4. §2d only if D3 says so. D3 stays deferred.

## 7. Tests (red on today's code, green after)

### Python

1. `test_main` (§2a). Built.
   - `test_an_unreadable_history_in_a_sweep_or_build_scan_writes_no_status`.
     Three cases: `batch_size=6`, `waiting_on_build=True`, and the scan's
     per-head status read failing. Before PR 1 each posted 3 errors.
   - `test_an_unreadable_history_marks_only_the_pull_request_the_event_names`.
     Two authors, 3 PRs on distinct heads, `--pr 1`; before PR 1 it marked
     both of alice's. Plus a draft N, which is still marked (red before PR 1
     only because its sibling was marked too), and a closed N,
     which before PR 1 marked the author's other PR.
   - `test_unreadable_history_revokes_every_known_head_only_in_apply` (full
     pass) stays green unchanged.
   - `test_an_unreadable_history_in_a_full_pass_marks_every_governed_head`:
     a guard, green before and after. The existing full-pass test has one
     PR, so it could not tell "every head" from "the first".

Items 2–4 belong to the dropped PR 2 and were not built.

2. `test_main`. Today `UNSETTLED` does not exist, so both fail.
   - Publication, with `UNSETTLED` seeded with the number, since the run loop
     adds it: `test_a_pull_request_stays_unsettled_until_its_verdict_is_posted`.
     - Must stay in `UNSETTLED`:
       - each of the four phrase paths;
       - an identity-mismatch return after `Evaluating`;
       - a raise.
     - Must move to `SETTLED`: each of the three verdict paths.
   - Through `run()`, with `collect` and the API patched:
     `test_the_run_marks_what_it_touched_and_an_unreadable_mark_even_if_its_reread_fails`.
3. `test_event` (`run`, `time.sleep` and the clock patched):
   - `test_a_pull_request_left_unsettled_is_reconciled_once_more_before_the_job_ends`
   - `test_an_event_run_that_raised_is_retried_and_the_job_is_green_once_repaired`
   - `test_a_sweep_that_raised_is_run_once_more_and_fails_if_it_raises_again`.
     Today it is not run again.
   - `test_the_round_is_capped_and_stops_at_a_failure_or_its_time_budget`
   - `test_nothing_is_retried_or_picked_below_the_rate_floor`
   - `test_an_event_pull_request_still_unreadable_after_its_retry_is_marked`
   - `test_the_last_resort_mark_skips_a_pull_request_that_got_its_verdict_and_never_writes_when_writes_are_off`
   - `test_a_raise_does_not_stop_the_round_or_the_backstop`
   - `test_the_backstop_picks_only_actions_unsettled_current_governed_heads_by_age`.
     The cases:

     | Case | Picked |
     |---|---|
     | evidence error, 10 min old, current head | yes |
     | same, posted by another login | no |
     | same, on a stale head | no |
     | 2 min old | no |
     | 8 h old, on the scan cron | no |
     | 8 h old, on the sweep cron | yes |
     | 30 h old | no |
     | latest status 10 min old, but its `Evaluating`/error streak began 30 h ago | no |
     | ungoverned base | no |
     | `waiting-bots` | no |
     | null `context` or `creator` | no, and the read does not fail |
     | `Evaluating`, streak 10 min old | yes |
     | 5 picks: the youngest 4 run | — |
   - `test_the_backstop_runs_only_on_schedules_with_writes_on`
4. `test_github`: `test_hygiene_statuses_reads_every_page_dedupes_and_refuses_errors_or_a_stuck_cursor`.

### Rust

5. Under `engine/tests/reconcile` (`publication.rs`). Built.
   - `an_unreadable_history_in_a_batch_writes_no_status`;
   - `an_unreadable_history_marks_only_the_pull_request_asked_for`;
   - `an_unreadable_history_marks_the_pull_request_asked_for_even_as_a_draft`
     (drafts are still marked; red before PR 1 only because the sibling was
     marked too);
   - `an_unreadable_history_marks_nothing_when_the_pull_request_asked_for_is_closed`.

   `publication.rs:254` (`Selection::All`) and `checklist.rs:1133`
   (`Pick::Pr(1)`, one PR) stay green.
6. Replaying `history-unreadable-pr` before the fix, Rust marks PR 2, then
   asks for `pulls/3`, which the recording does not hold, so it fails. After
   the fix it passes. `history-unreadable-batch` fails the same way on
   `pulls/1`. Observed as described.
7. `engine/tests/differential/main.rs` counts: red until they are updated.
   Observed as described.

A reviewer prototyped draft v1's §2a, which also stopped marking on a full
pass. The only existing tests it turned red were the two full-pass tests
(`test_unreadable_history_revokes_every_known_head_only_in_apply` and
`publication.rs:254`), and v2 keeps both green. No other existing test pins
the paths this spec narrows.

The red-before, green-after transition goes in the implementation commit's
message (TDD rule).

### Live verification

- The next failing scan or sweep should leave no new error statuses.
- The next event run whose history read fails should mark only its own PR.
- Re-run the bypass audit (rule-suites, head status at merge). Expect fewer
  bypasses of heads at `Incomplete policy evidence`.
- The daily `hygiene_statuses` count belonged to PR 2 and is dropped with it.

## 8. Dependencies on sibling specs and the roadmap

With PR 2 dropped, the points below about §2b, §2c and `hygiene_statuses`
are moot. What still holds: under §2a a failing scan, B's head-scoped scan
included, writes nothing. The service's queue now owns re-evaluating what a
run leaves unsettled.

- **B (build completion trigger, `guides/BUILD_COMPLETION_TRIGGER_SPEC.md`).**
  - B's §10 still refers to draft v1's `['--unsettled']` selection. v2 adds no
    selection: §2b and §2c live in `event.main`, after whatever
    `selections()` returns. B's new `workflow_run` branch therefore
    conflicts with nothing.
  - (§2b only, dropped.) B's head-scoped `--waiting-on-build --head SHA` run
    is a scan selection under §2b step 2. If it raises with nothing
    unsettled, it is re-run once.
  - B keeps the `2,32,47` cron, which the §2c backstop needs (at least 4
    scheduled runs an hour).
  - B offers to widen its build-completion `waiting_on` to A's phrases.
    Declined: §2b already re-runs within about a minute, and §2c covers the
    tail. Widening it would couple build completion to a status that has
    nothing to do with the build. With §2b and §2c dropped, the first reason
    is gone, but the coupling argument still holds.
  - B can reuse `hygiene_statuses()` in Python if it needs every head's status
    cheaply.
  - Under §2a a failing scan already writes nothing.
- **C (status next step).**
  - C leaves the five phrases unchanged and assigns them to A, and A does not
    reword them.
  - The backstop matches them exactly from one constant. It does not go
    through `latest_state_from_status`, which C changes to return a token.
  - No phrase contains `": "`, so C's token parse leaves them whole.
  - (§2c only, dropped.) Anyone rewording a phrase later must keep the old
    text matched until heads posted before the change age past 24 h.
- **Service roadmap.**
  - §2b and §2c were Actions-only and would have been retired with the
    Actions engine. They are dropped (2026-10-29), so the service's queue is
    the only planned re-evaluation of unsettled heads.
  - §6e of the service spec already plans re-enqueue with backoff for
    "…reconciliation required" and "Policy reconciliation failed".
  - The owner should add `Incomplete policy evidence` and an `Evaluating`
    early return to §6e. In Rust, the `UNSETTLED` analogue should be a value
    `reconcile::run` returns, not a global.
  - §2a is shared behaviour and lands in both engines, as #105–#111 did.

## 9. Decisions for the user

**Taken by the owner on 2026-10-29:**
- **PR 2 (§2b, §2c) is dropped**, not built. Retry plumbing on the Actions
  side is moot once the GitHub-App service lands, which has its own queue.
- **D1: approved.** A sweep or scan whose shared read fails writes nothing,
  `--pr N` marks only N, and a full pass (`scope=all`, no selector) keeps
  failing closed, unchanged.
- **D2: moot.** It tuned PR 2.
- **D3: deferred.** §2d stays out.
- **D4: approved.** The event's own PR keeps `error` and today's wording.
- **D5: moot.** It tuned PR 2.

The questions as they were put:

- **D1.** A sweep or scan whose shared read fails writes nothing, while a full
  pass keeps failing closed. *Recommended.* Note that GitHub notifies only the
  user who last edited the cron about a failed scheduled run. A failed sweep is
  visible only in the Actions tab. §2b re-runs it once, and the next sweep
  tries again. [Superseded: the next sweep reads another batch; the
  rotation comes back to the skipped one (§2a).]
- **D2.** A job that the in-job round repaired ends green, so no red job sits
  next to a correct status. *Recommended.*
- **D3.** Retrying GraphQL reads (§2d): a separate follow-up in both engines,
  or not at all. *Recommended: a follow-up, after PRs 1–2 have run for a
  week.*
- **D4.** On a failed read, the event's own PR keeps `error` and today's
  wording. *Recommended.*
- **D5.** The tuning:

  | Setting | Value |
  |---|---|
  | in-job round | one, after 30–45 s |
  | PRs per round | 3 |
  | time budget | 3 min in the round, 4 min in the backstop |
  | rate floor | 300 REST / 200 GraphQL |
  | backstop age tiers (by streak age) | 5 min / 6 h / 24 h |
  | backstop picks per run | 4, youngest streak first |

## Appendix A: review findings rejected or changed

The items below about §2b and §2c belong to the dropped design. The review of
the built code is in the PR description.

Five reviewers looked at draft v1: correctness, simplicity, failure modes,
Rust/conformance/roadmap, and GitHub semantics. Most must-fixes were adopted
and are listed in the v2 text: full pass fail-closed, the backstop in
`event.py`, one round, caps, the rate floor, age tiers, two recordings, the
missing tests, the §1 corrections and §2d deferred. Rejected or changed:

- **Read the status back instead of an in-process list** (simplicity M2).
  Rejected. It costs requests on every event run under a 1,000-per-hour Free
  plan budget, and it misses a mark whose re-read failed. The
  "unsettled until verdict" rule takes three edits and no reads.
- **On a failed full pass, fall back to per-author reads and mark only heads
  that are currently `success`** (failure-modes M1 fix). Not adopted. Keeping
  the full pass unchanged is simpler, and an operator starts it.
- **An extra backstop pick for heads shared with another PR** (failure modes,
  M2(b)). Rejected. That gap predates this spec, step 4 covers the event case,
  and the next successful run marks shared heads `configuration-error`.
- **Close the race window in `finish()` by re-snapshotting after `success`**
  (failure-modes nice-to-have 5). Out of scope: the race predates this spec.
- **`::warning::` annotations** (failure-modes nice-to-have 7). `event.py`'s
  stdout is redirected into the step summary, so plain stderr lines are used
  instead.
- **Also pick `Invalid policy configuration`** (failure-modes nice-to-have 13).
  Rejected. A configuration error is deterministic and needs a policy fix.
- **Drop §2d entirely** (simplicity N1). Kept as a deferred section with the
  reviewers' findings, so the facts are not lost.
- **Catch every exception per selection** (correctness M3, failure-modes
  nice-to-have 8). Adopted for four classes. Only `GitHubError` is retried.
- **Bound a stuck head by marking the backstop's own statuses with a
  `target_url`** (GitHub-semantics must-fix 6, option 1). Not adopted. It
  would put a flag through `run` and `publish` for an Actions-only concern.
  Option 2 was adopted instead: streak age, from one cached REST read per
  candidate.
- **Second round.** A reviewer re-checked v2 and found 8 must-fixes, all
  adopted:
  - gate the last-resort mark on writes being on, and fire it only if N got
    no verdict (`SETTLED`);
  - carry on past a retry or pick that raises;
  - order picks youngest streak first, since `periodic_batch` does not
    rotate over 2 or 4 picks;
  - re-run a sweep only for the shared-read case;
  - check the floor before every run;
  - pick a stale `Evaluating` by streak age;
  - fix the §7.2 and §7.6 test text;
  - fix the "push or verdict" wording.
- **Retry the 60 s subprocess timeout** (GitHub-semantics nice-to-have).
  Recorded under §2d for the follow-up. It is not done here, because each
  retry could add another 62 s per call.
