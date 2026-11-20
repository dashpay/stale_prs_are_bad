# Re-evaluate a pull request when its CI finishes

Status: deferred to the service. Reviewed (draft v2, Appendix A) but not built
in Actions: it needs a new trigger in other teams' caller workflows and about
158 extra runner jobs a day, and would have avoided 2 bypasses in 30 days. The
service receives CI events directly, so step 2 of `PR_HYGIENE_SERVICE_SPEC.md`
is where this lands. The evidence and GitHub research below still hold.

Revision: draft v2, revised after a five-lens review (Appendix A). Scope: how
soon `PR Hygiene` re-evaluates a head recorded as `waiting-build` once its
build has finished. The sibling specs cover the rest:

- A (`SWEEP_FAILURE_ISOLATION_SPEC.md`, v2): failed reads and the "reconciliation
  required" backstop.
- C (`STATUS_NEXT_STEP_SPEC.md`): the wording of the status.

## 1. Problem and evidence

No event the controller hears fires when a build finishes. A head at
`waiting-build` moves on only when one of these happens:

- some other event re-evaluates it;
- the build scan finds it (`'2,32,47 * * * *'`, `event.py:15`,
  `main.py:141-152`);
- the hourly sweep's rotation reaches its author.

**The scan is late.** On tenderdash, 2026-10-23, from the Actions runs API:

- The scan cron fired 62 of its 72 times.
- Each run started a median of 10.7 min after its cron minute (p90 21.2 min,
  max 29.4 min).
- Consecutive scan runs were a median of 23.2 min apart (p90 33.7 min,
  max 42.0 min).

GitHub documents both effects: "The `schedule` event can be delayed during
periods of high loads … some queued jobs may be dropped"
(<https://docs.github.com/en/actions/reference/workflows-and-actions/events-that-trigger-workflows#schedule>).

**What it costs.** From 2026-10-13 to 10-27, across the five repos, 18 heads
sat at `waiting-build` and then went green. The time from their last check
finishing to the head leaving `waiting-build` was:

- median 2.7 min, because another event often re-evaluated the head first;
- p90 27.9 min;
- max 28.4 min.

On the human path no reviewer is asked until that re-evaluation
(`policy.py:1077-1083`). This lag is therefore also the delay in asking a
reviewer.

**Merges whose last status was `waiting-build`**, 2026-09-27 to 10-27:

- **platform:** 2 of 354 merges.
  - #5348: the build was the only item left unchecked. The trigger alone
    would have turned it green: `Tests` finished at 21:05:28 and ktechmidas
    merged at 21:13:19.
  - #5231 still needed an approval.
- **dash-evo-tool:** 3 of 80 merges.
  - #1075: the build was the only item left unchecked. The trigger alone
    would have turned it green: `Clippy` finished at 14:23:45 and the PR
    merged at 14:34:36.
  - #1131 and #1066 still needed approvals.
- **tenderdash:** 3 of 31 merges.
  - #1566 (author Claudius-Maginificent): CI finished at 08:33:28. lklimek
    had enabled auto-merge at 08:27:11, disabled it at 08:34:28 and
    bypass-merged at 08:34:35. After green it still
    needed an approval, so the trigger would not have changed the outcome.
  - #1558: CI finished at 11:02:40 and it was bypass-merged at 11:08:33. The
    trigger helps only together with the parallel decision that makes
    lklimek a co-owner of `.github/`. With both, the PR reaches
    `ready-to-merge`, and lklimek's auto-merge (enabled at 10:52:45) merges
    it about 1 min after CI.
  - #1523: its build had failed.
- **rust-dashcore and grovedb:** none of 86 and 21 merges.

#1557 was not a build case. It was `ready-for-human`, and it merged 29 s
after the approving review, before that review's run finished.

**Size of the problem:** two bypasses in 30 days that the trigger alone
would have avoided, plus a third with the co-owner decision. On top of that,
reviewer requests wait up to about 30 min on roughly one in five heads that
wait on a build.

## 2. Research: what GitHub delivers

### `check_suite` / `check_run`

Neither reaches a workflow for these repos. Both "do not trigger workflows
if the check suite was created by GitHub Actions **or if the check suite's
head SHA is associated with GitHub Actions**" (events page, `#check_suite`,
`#check_run`). Every PR head here carries Actions suites, so even codecov's
suite never reaches a workflow.

### `status`

GITHUB_TOKEN writes start no workflow
(<https://docs.github.com/en/actions/concepts/security/github_token>). That
rules out super-linter's `--> Linted: *` on tenderdash and
`action-semantic-pull-request` on platform.

What remains are statuses written by apps:

- `CodeRabbit`, in all five repos. Its completion already wakes the caller
  through its comment edit (`RECEIPT_MARKER`).
- `codecov/*`, on grovedb.

### `workflow_run` with `types: [completed]`

It fires "regardless of the conclusion", and also for re-runs. Only
`requested` is suppressed on a re-run.

- **Default branch.** It runs from the default branch's workflow file. Its
  check lands on the default branch head, never on a PR.
- **Name filters.** `workflows:` matches names. Globs `* ** ? + [ ]` and a
  leading `!` are documented for "workflow name filters"
  (<https://docs.github.com/en/actions/reference/workflows-and-actions/workflow-syntax#onworkflow_runworkflows>,
  `#filter-pattern-cheat-sheet`). Parentheses and colons need no escaping.
- **Chaining.** "You can't use `workflow_run` to chain together more than
  three levels of workflows." CI on `pull_request` (level 1) → PR Hygiene
  (level 2) fits.
- **Forks.** For a fork PR `workflow_run.pull_requests` is empty, in the
  payload and in the runs API
  (<https://github.com/orgs/community/discussions/25220>; live:
  `thepastaclaw/platform`, `kwvg/rust-dashcore`).
  - For an open same-repo PR it has exactly one entry (tenderdash #1567,
    dash-evo-tool #1138). It shows empty for #1566 and #1558 only because
    the API recomputes it after merge.
  - Fork PRs among the last 100: platform 4, dash-evo-tool 2, rust-dashcore
    17. Tenderdash had none among its last 45.
  - #8 (`d2978b0`) removed an earlier `workflow_run` relay because of this.
  - `workflow_run.head_sha` is always the PR head. rust-dashcore's own
    `.github/workflows/ready-for-review.yml` matches it against the open-PR
    list. This spec follows that in-org pattern.
- **Latency.** On rust-dashcore's `workflow_run` workflow, runs were created
  a median of 2 s (p90 3–4 s, max 9 s, n = 117) after the triggering run
  completed. The only documented drop is above 500 queued runs per 10 s per
  repo.
- **Job filters.** A job skipped by a job-level `if` takes no runner. Live:
  tenderdash run 39984072189, `runner_name` null. Its run is still listed,
  as skipped. The `github` and `vars` contexts are available in that `if`.

### Which check finishes last

Measured on the 30 most recently updated PR heads per repo:

- An Actions workflow finished last on 26 heads on platform, 27 on
  tenderdash and 26 on dash-evo-tool.
- codecov finished last on 9 of rust-dashcore's heads and 20 of grovedb's,
  between 4 s and 25 min after the last workflow.

### Which workflow finishes last

Measured on `pull_request` CI runs from 2026-10-13 to 10-26, excluding
cancelled and skipped runs. Platform's sample is capped at 1,000 runs, which
covers 10-22 to 10-26.

| Repo | Heads | Finished last |
|---|---|---|
| tenderdash | 93 | `e2e` 63, `Test` 16, `Build` 5, `Markdown Linter` 5, other 4 |
| platform | 391 | `Build and test Kotlin SDK` 208, `Tests` 158, other 25 |
| dash-evo-tool | 154 | `Tests` 131, `Clippy` 14, other 9 |

### Which repos gate on PR Hygiene

- platform, tenderdash, grovedb and dash-evo-tool require `PR Hygiene` in
  their classic protection.
- rust-dashcore requires no status at all on `dev`.

## 3. Approach

Re-evaluate when one of the CI workflows that usually finishes last
completes. Do it only for a PR on that head whose status says
`waiting-build`, and only once that build is no longer running. The planned
service uses the same pick rule (§10). The scan and the sweep stay as they
are and remain the fallback.

### 3a. Caller, in tenderdash, platform and dash-evo-tool (generated by `rollout.py`)

```yaml
on:
  # … existing triggers, unchanged …
  workflow_run:
    # GitHub starts no workflow on check events for a head GitHub Actions
    # builds, so a CI workflow finishing is what says the build may be done.
    # These are the workflows that finish last on this repository's pull
    # requests. A head whose last check is another one is still seen by the
    # build scan above, only later.
    workflows: ['e2e', 'Test']
    types: [completed]
jobs:
  policy:
    if: >-
      (github.event_name != 'workflow_run' ||
      (vars.PR_REVIEW_AUTOMATION_ENABLED == 'true' &&
      vars.PR_REVIEW_BUILD_TRIGGER != 'off' &&
      github.event.workflow_run.event == 'pull_request' &&
      !contains(fromJSON('["cancelled", "skipped", "action_required"]'),
                github.event.workflow_run.conclusion))) &&
      (github.event_name != 'issue_comment' ||
      …the existing comment clause, unchanged…)
```

**Names per repo:**

| Repo | `workflows:` | Heads where one finishes last | Real runs a day |
|---|---|---|---|
| tenderdash | `e2e`, `Test` | 79/93 (85 %) | ≈21 |
| platform | `Tests`, `Build and test Kotlin SDK` | 366/391 (94 %) | ≈117 |
| dash-evo-tool | `Tests`, `Clippy` | 145/154 (94 %) | ≈20 |

**What the filter does.** Nothing it filters out uses a runner. It drops:

- the listed workflows' runs from other events, such as post-merge pushes on
  base branches and dash-evo-tool's `workflow_dispatch` runs on draft
  branches;
- `cancelled` runs, which a newer head superseded on 190 of 194 sampled.
  That newer head's own `synchronize` event re-evaluates the PR;
- `skipped` runs, which finish as they start;
- `action_required` runs, which wait for approval. The run after approval
  completes normally.

**Off switches.**

- `PR_REVIEW_BUILD_TRIGGER=off` is a repo or org variable. Repo admins and
  org owners can set it. It stops trigger jobs before any runner is taken.
- Repos with writes off pay nothing either, through the
  `PR_REVIEW_AUTOMATION_ENABLED` term.

PR Hygiene's own workflow is not listed, so it never triggers itself.

### 3b. Reusable workflow: one line

```yaml
group: pr-review-${{ github.repository }}-${{ github.event.pull_request.number || github.event.issue.number || github.event.workflow_run.pull_requests[0].number || github.event.workflow_run.head_sha || github.run_id }}
```

This replaces `pr-review-reusable.yml:18`. `queue: max` and
`cancel-in-progress: false` stay.

- **Same-repo PR.** Its trigger runs join the group of that PR's own event
  runs. "Runs for one pull request stay serialised" (`:19-25`) then holds for
  them too. Without it, two trigger runs, or a trigger run and an event run,
  publish the same transition in parallel. Each finds no announcement for
  the new state (`main.py:715-720`) and POSTs its own move comment
  (`main.py:759-762`, `github.py:1124`), and every subscriber is notified
  twice. Measured over all of tenderdash's CI workflows, the last two completions on a head were within
  30 s of each other on 11–13 of 84–103 heads.
- **Fork PR.** `pull_requests` is empty, so the group is the head SHA. That
  serialises trigger runs on that head, but not against the PR's own event
  runs. The race left over is in §7.
- **Other events.** The new terms are null, so their group is unchanged.
- **An empty `pull_requests`.** `[0]` on an empty array is expected to give
  null, so `||` falls through to the head SHA. GitHub does not document this.
  The indirect evidence is that a published action (`sorted-pr-checks`)
  keys its concurrency group on the same expression. The first fork PR after
  the platform rollout checks it. If it errors, key on `head_sha` alone and
  give up the serialisation with event runs.

The target clone is still skipped for `workflow_run`, because its gate names
only dispatch, push and the sweep's cron (`:139,147,173`).

### 3c. Engine, `pr_review/event.py`

Add `import re`. Put a new first branch in `selections()`:

```python
if kind == 'workflow_run':
    finished = event.get('workflow_run') or {}
    # Writes off: nothing is recorded as waiting, as for the scan. Another
    # event than a pull request's CI is not a build the policy waits on.
    if (os.environ.get('PR_REVIEW_AUTOMATION_ENABLED') != 'true'
            or finished.get('event') != 'pull_request'):
        return []
    head = finished.get('head_sha')
    if not isinstance(head, str) or not re.fullmatch(r'[0-9a-f]{40}', head):
        raise ValueError('Invalid workflow run head')
    return [['--waiting-on-build', '--head', head]]
```

Only `head_sha` is read from the payload. `pull_requests`, `conclusion` and
the author-controlled names and branches are never read.

The assertion at `test_event.py:62-63`, that `workflow_run` raises, is
replaced by one that the payload's PR list never selects a PR.

### 3d. Engine, `pr_review/main.py`

`--head SHA` is accepted only with `--waiting-on-build`, and only as 40
lowercase hex characters; anything else is a parser error. `run` passes
`head=args.head` to `collect`, which keeps `number=None`, so neither
`reconcile_author` (`:133-138`) nor the not-open raise (`:153-154`) applies.

In `collect`, with a head given, the steps run in this order. Each stops the
run, writing nothing, when it finds nothing.

1. **Status.** `api.latest_state_from_status(head) == 'waiting-build'`.
   - This is one REST read, cached per head (`github.py:1036-1039`) and
     reused later by the snapshot. It comes before the listing.
   - The status, not the record, decides here. A record can belong to an
     earlier head of the same PR (`main.py:147-149` never checks `head`), and
     reading it means the history query.
   - C keeps `waiting-build` bare (§10).
   - A head showing an "unsettled" phrase is A's backstop's job (§10).
2. **PRs on the head.** From the listing, the governed open PRs whose `head`
   equals `head`. `selected` becomes the governed PRs of their authors, as
   `--pr` narrows (`:127-129`). Admission depends only on an author's own
   PRs (`context_fingerprint`).
3. **Build.** `api.build_state(number, head) != 'running'`.
   - One GraphQL query, cached per head (`github.py:774-775`) and reused by
     the snapshot (`:926`).
   - A build still running would re-evaluate to the same `waiting-build`, so
     skipping it loses nothing.
   - `green` and `failed` both continue: a failed build changes the
     checklist line.
4. **Evaluate.** `load_histories` for `selected`. `requested` is the
   candidates on `head`; no `periodic_batch` is applied. Snapshot, evaluate
   and publish then run as for any run.

Also:

- **No `clear_marks`.** The condition at `:1052` becomes
  `args.pr is None and args.head is None`: a run aimed at one head does not
  tidy the repository.
- **Failures** are handled exactly as for the scan, with no rule of B's own:
  - Today a failed shared read marks `selected`, which is the same author's
    PRs that a `--pr` event run marks. A failed snapshot marks that PR. A
    failed publish posts `Policy reconciliation failed` on that PR
    (`:1036-1047`).
  - Once A's PR 1 lands, this is a `--waiting-on-build` selection, so a
    failed shared read writes nothing (A §2a).
  - A failed publish on the transition run leaves a phrase that A's §2c
    backstop picks up.

### 3e. Conformance, the Rust port and the corpus

**Conformance.**

- `conformance._normalised_argv` (`conformance.py:524-536`) writes
  `--head <sha>` after `--waiting-on-build`.
- After the tests are added, run `python -m pr_review.conformance harvest`
  on 3.12 and commit what it writes. Expect only `tests`-list changes in
  `conformance/evaluate/` and `conformance/functions/`.
- `pending.txt` stays empty.
- No synthetic recording is added: Rust would refuse it.

**Rust: no change.**

- Rust refuses `--waiting-on-build` as an argument it does not take
  (`engine/src/conformance/run.rs:155`, `Failure::Command`).
- `synced()` returns `None` for it (`run.rs:499-506`), so nothing live runs
  it.
- `Recording::command()` labels it `sync --waiting-on-build`
  (`recording.rs:193-205`).
- Nothing in `service/` or `src/` reads caller files or event names.

**The invariant the corpus enforces.** Every new line is reachable only with
`--head`. The `--pr`, batch and full paths make the same calls in the same
order as before. Two checks hold this: the synthetic-recording staleness
check (`test_conformance.py:585-591`) and Rust's replay of every synthetic
recording (`engine/tests/reconcile/recordings.rs:101`).

This is the service spec's line (§6c: "Actions-only scheduling is not
ported: … the build scan"). It is not #105–#111's: those changed behaviour
both engines share.

**CI:** `cargo test --workspace --locked`.

### 3f. `pr_review/rollout.py`

- **`BUILD_WORKFLOWS`.** A constant mapping repository → names (§3a). It
  lives in code so that a regenerated caller cannot lose it.
- **`caller_workflow(engine_revision, build_workflows=())`.**
  - With names, it emits §3a.
  - Without names, its output is byte-identical to today's. All five live
    callers equal `caller_workflow('master')`. rust-dashcore and grovedb stay
    on that output.
- **`write_bundle`** passes `BUILD_WORKFLOWS.get(repository, ())`.
- **Name check.** A name is refused, not escaped, when it is empty, contains
  `* ? + [ ] \ '` or a newline, or starts with `!`. None of the listed names
  needs any of these.
- **`REVIEW_POLICY_ROLLOUT.md`** names the listed workflows and the
  `PR_REVIEW_BUILD_TRIGGER` switch.

## 4. Alternatives rejected

| Alternative | Delay | Extra jobs a day | Why not |
|---|---|---|---|
| Today's scan | median 23 min between scans, max 42; 14 % of scans dropped | — | It is the problem. |
| Scan cron at the minimum `*/5` | 5 min nominal; scheduled starts measured a median of 10.7 min late, max 29 | +192 per repo | Also needs a caller change, and GitHub's delay dominates. |
| Engine only: the sweep also runs `--waiting-on-build` | gap between scheduled runs median 13.4 min, p90 30.1, max 38.8 | 0 | No caller PRs, but the tail barely moves. It adds another all-PR history read an hour, the read A shows timing out. Fallback if D1 is declined, after A. |
| `workflow_run` → today's `--waiting-on-build`, no `--head` | as chosen | as chosen | Fine on tenderdash and dash-evo-tool. On platform each run would do the all-PR history read and re-snapshot its 9 stuck `waiting-build` heads, about 400 requests a run. |
| `workflow_run` → `--pr N`, or `--pr N --waiting-on-build`, from `pull_requests` | as chosen | as chosen | Misses forks. A number can outlive its head, so the head check is needed anyway. `--pr` also re-publishes the author's other PRs. |
| `workflows: ['**', '!PR Hygiene policy']` | as chosen, for every Actions-last head | tenderdash ≈94, platform ≈200, dash-evo-tool ≈45 | Documented, identical callers, no name drift. But 3–4× the jobs, and every intermediate completion spends the shared budget. D2. |
| Every `pull_request` workflow by name | as chosen | tenderdash ≈94 | +15 % of heads for 4.5× the jobs on tenderdash. |
| `check_suite`, `check_run` | never fires | — | §2. |
| `status` | codecov and CodeRabbit only | ≈2 per push | CodeRabbit is already heard. codecov matters only on grovedb, which is not proposed. |
| Auto-merge, merge queue | — | — | lklimek already used auto-merge on #1566 and #1558. It waited on the stale pending status. A merge queue needs `merge_group` support in every required check. |
| Wait for the GitHub-App service | event-driven | — | Hosting has been with infra since 10-07 (dashpay/infra#90), with no cut-over date. |
| Poll inside the run until the build ends | seconds | holds a runner for the build (16 min on e2e) | It blocks the PR's serialised queue. |

## 5. Cost

**Runners.** One job each on `dash-x64-light`, about 35 s; measured scan
jobs take 33–36 s.

| Repo | Real jobs a day | Listed-workflow runs that start no job (skipped, cancelled, other events), a day |
|---|---|---|
| platform | ≈117 | ≈25 |
| tenderdash | ≈21 | ≈1 |
| dash-evo-tool | ≈20 | ≈26 |

That is about 158 jobs, or 1.5 runner-hours, a day. The scans already run
about 360 jobs a day across the five repos.

**API.** dashpay is on GitHub's free plan, so `GITHUB_TOKEN` is limited to
1,000 requests an hour per repository, shared by every workflow there
(<https://docs.github.com/en/rest/using-the-rest-api/rate-limits-for-the-rest-api>).
Cost per trigger job:

| Case | Requests | Share of trigger runs |
|---|---|---|
| Head not at `waiting-build` | 4: the resolve step's 3, plus one statuses read | 97.5 % of completions on tenderdash in the sampled week |
| Head waiting, build still running | 7: adds the listing (1–2 pages) and one `build_state` | |
| Build finished | about 100: the transition run (three snapshots, admission re-reads, writes) | |

The transition run is work the scan does today when it finds the head. The
trigger only moves it earlier.

**Peak hour.** The busiest hours had 65 PR CI completions on tenderdash
(10-21 11:00Z) and 42 on platform. Of those, the listed workflows account
for about 12 and 33. That adds at most about 85 and 230 requests an hour.
No baseline for that hour is known (A §2c). The rollout measures it (§9).

**Writes.** An unchanged PR is never written (`main.py:729-734`,
`github.py:1093-1096`). The 1,000-statuses-per-SHA cap and the secondary
write limit are not touched.

## 6. Delay, before and after

From the last check finishing to `PR Hygiene` re-evaluating:

| Case | Today | After |
|---|---|---|
| A listed workflow finishes last: 85 % of heads on tenderdash, 94 % on platform and dash-evo-tool | Next scan: ≤ 30 min nominal, measured up to 42 min. Longer when scans are dropped or more than 12 PRs wait. | Dispatch 2–9 s, plus runner pickup, plus about 35 s: under a minute typically, bounded by the runner queue |
| Another check finishes last, or a `pull_request_target` check (title or lint on `edited`), or the repo is rust-dashcore or grovedb | as today | as today |
| A failed build re-run to green | as today | as the first row: re-runs fire `completed` |
| A fork PR racing its own event run (§7) | — | as today (next scan) |

## 7. Failure modes

**An unevaluated green: impossible.** The trigger only chooses which PRs to
evaluate. `success` is posted only from `finish()`, after a fresh
re-snapshot (`main.py:653-666`). The payload's `conclusion` is never read.

**Writes outside scope: none.** B's selection writes only to PRs whose
listed head is the SHA and whose head status was `waiting-build`.

- A head shared by two PRs gets both evaluated, as the scan already does.
- `clear_marks` is not run.
- A stale SHA matches no open PR.
- A's §2b retries are A's scope (§10).

**Races.**

- **Same-repo PRs:** closed by the group in §3b.
- **Fork PRs:** a trigger run and the PR's own event run can still overlap.
  - Either the trigger run skips a head still marked `waiting-bots`, or a
    late event run publishes `waiting-build` over a fresh green.
  - In the second case it removes the `ready-*` label, but requested
    reviewers stay requested.
  - The scan repairs both, through the status and `ready_published`.
  - Forks are 0–4 % of PRs on the three repos.

**Name drift.** A renamed workflow, or a new one that finishes last, falls
back to the scan: slower, never wrong. The caller comment says what is
listed and why. §9 re-measures what finishes last.

**Version skew and rollback.**

- The engine merges first. A caller that lands earlier makes every trigger
  job fail at `Unsupported review event`, on the default branch head and on
  no PR.
- **Never revert** the `event.py` branch while a caller lists `workflow_run`.
- To roll back, in order of reach:
  1. set `PR_REVIEW_BUILD_TRIGGER=off` at org or repo level: instant, no
     runner;
  2. a forward central change;
  3. revert the caller, which the repo's owners do.
- A SHA-pinned packet must pin at or after the engine commit.

**Failed trigger jobs.**

- They are red on the default branch head, as a failing scan is today.
- They may email the user whose push triggered the CI run
  (<https://docs.github.com/en/actions/concepts/workflows-and-actions/notifications-for-workflow-runs>).
  That user may be a fork contributor. D3.

**Amplification.**

- A fork PR can add workflows named `Tests`, and each one adds a job.
- The same author can already start self-hosted jobs with comments. Nothing
  new is trusted, but the runner use is real.
- §9 watches PR-event queue time.

**Check suites on the default branch head.**

- Trigger jobs add suites there, as scans do. Tenderdash's head `565c85b`
  already had 507.
- GitHub's check-run listing for a ref sees only the 1,000 newest suites. The
  engine never reads that head.

## 8. Tests

Today's suite passes on `2fa56d6`: 545 tests, with
`uv run --no-project --python 3.12 --with pyyaml python -m unittest discover -s pr_review/tests`.

### Red on today's code, green after

**`test_event.py`, in a new `WorkflowRunTests`:**

- `test_a_finished_pull_request_build_looks_again_at_its_head`
- `test_the_payload_pull_request_list_never_selects`: writes are on, the
  event is `pull_request`, the head is valid and `pull_requests` is
  `[{number: 12}]`. The result is exactly
  `[['--waiting-on-build', '--head', sha]]`. This replaces the assertion at
  `:62-63`.
- `test_a_fork_build_is_heard_by_its_head`: `pull_requests: []` still
  selects.
- `test_a_run_of_any_other_event_or_with_writes_off_does_nothing`

**`test_main.py`:** modelled on the helper at `:235-248`, but with a
distinct head per PR.

- `test_a_finished_build_reconciles_only_the_waiting_pull_request_on_its_head`
  - Fixture: #1 (author X, head A, `waiting-build`), #2 (author X, head B,
    `waiting-build`), #3 (author Y, head C).
  - With `--head A`, only #1 is snapshotted, and histories are read only
    for X's PRs.
  - No write of any kind touches #2.
- `test_a_head_not_waiting_on_its_build_costs_one_status_read`: no listing,
  no histories, no snapshot.
- `test_a_build_still_running_takes_no_snapshot`
- `test_a_record_for_an_earlier_head_does_not_hide_a_waiting_head`
- `test_a_run_aimed_at_one_head_does_not_tidy_the_repository`

**`test_conformance.py`** (not harvested):

- `test_a_recording_names_the_head_a_build_scan_was_aimed_at`: record a
  head-scoped dry sync and replay it with no differences.

**`test_rollout.py`:**

- `test_it_hears_the_ci_workflows_that_finish_last`: `yaml.safe_load` gives
  `{'workflows': [...], 'types': ['completed']}` under `workflow_run`.
- `test_a_finished_run_wakes_it_only_for_a_pull_requests_build`: the
  `workflow_run` clause carries all its terms, including both `vars`
  switches. It is ANDed ahead of the comment clause. `CommentTriggerTests`
  runs over the outputs with and without names.
- `test_a_name_needing_escapes_is_refused`, plus a check that every
  `BUILD_WORKFLOWS` name passes.

**`test_workflow.py`:**

- `test_a_finished_build_queues_with_its_pull_requests_own_runs`: the
  concurrency group carries both `workflow_run` terms, before `run_id`.

### Guards (already green)

- A repo without names keeps today's caller, checked byte for byte against
  the output of `caller_workflow('master')` today. The assertion at
  `test_rollout.py:129` keeps holding for those repos.
- `--head` without `--waiting-on-build`, or not a full SHA, is a parser
  error.
- `workflow_run` does not clone the target.
- The two crons stay.
- `cargo test --workspace --locked` passes, and the corpus re-harvest only
  rewrites `tests` lists.

## 9. Rollout and verification

1. **Central PR** (dashpay/stale_prs_are_bad).
   - Contents: §3b–§3f, their tests, the re-harvest, and
     `guides/PR_REVIEW_OPERATIONS.md` (what the trigger is, the switch, how
     to re-measure).
   - It is live everywhere on merge but dormant, since no caller sends
     `workflow_run`.
   - Check that the next scan, sweep and PR-event runs in each repo are
     green and keep their concurrency groups.
2. **Tenderdash caller PR**, the pilot.
   - Generate it with `pr_review.rollout`.
   - `.github/` belongs to ktechmidas, and also to lklimek under the
     parallel decision.
   - Why tenderdash: 6 open PRs, auto-merge on, CI checks in its ruleset,
     owners already engaged.
3. **Verify for 3–5 days**, read-only (method in Appendix B).
   - One `workflow_run` run within seconds of each listed completion, about
     20 real jobs a day.
   - Every head that reached `waiting-build` leaves it within 2 min when a
     listed workflow finished last.
   - No trigger run writes to a PR whose head is not its SHA.
   - No duplicate move comments.
   - No 403 or 429 in any PR Hygiene run.
   - PR-event job queue time no worse than before.
   - Re-measure which workflows finish last.
4. **Platform and dash-evo-tool caller PRs**, each to its owners. Repeat the
   first three checks of step 3. Platform has the most runs, so watch its
   peak-hour 403/429s for a week.
5. **Not proposed:**
   - rust-dashcore: it requires no status, and codecov finishes last on
     about a third of its heads.
   - grovedb: about 3 PR CI runs a week, and codecov finishes last on 2/3 of
     its heads.
6. **Retirement.** Each repo's service cut-over PR removes the
   `workflow_run` block from its caller. Add that to the service spec's
   "what is retired, when" table.

## 10. Dependencies on sibling specs

**A (v2, sweep failure isolation).**

- **No overlap in `selections()`.** A's §2b re-runs and §2c backstop live in
  `event.main`, after `selections()` returns. B adds only a `workflow_run`
  branch.
- **§2a.** A head-scoped run is a `--waiting-on-build` selection, so a
  failed shared read writes nothing. A's row must key on `waiting_on_build`,
  with or without `--head`. Until A's PR 1 lands, it marks as the scan does
  today (§3d).
- **§2b.**
  - A head-scoped run that raised with nothing unsettled is re-run once as a
    whole, with the same scope.
  - Unsettled numbers are retried as `--pr N`, with `--pr`'s scope, which
    also covers the author's PRs whose slots changed. B's scope promise
    covers B's own selection only.
  - With §3b, a same-repo trigger job's retry runs inside that PR's group.
  - During an outage each failed trigger job holds a runner up to about 4 min
    longer. With the names in §3a that is a few jobs an hour.
- **§2c** needs at least 4 scheduled runs an hour. B keeps both crons
  unchanged.
- A declined widening B's pick to its phrases, and B does not widen it.
- B reads one head's status first, not the all-PR history read that A shows
  failing.

**C (status next step).** C keeps `waiting-build` bare (C, "Dependencies"),
so step 1 of §3d holds. B adds no new status reader.

**Service roadmap.**

- **The same pick rule.** Service spec, "What triggers a reconcile": check
  events go through a head-SHA → PR index, and are "enqueued only for PRs
  recorded as waiting on a build". GitHub delivers `check_suite` webhooks to
  Apps for Actions suites too: the webhook docs carry no such exclusion
  (<https://docs.github.com/en/webhooks/webhook-events-and-payloads#check_suite>).
- **Differences that exist only in Actions, and go with it:**
  - the service runs `Selection::Pr`, which also re-publishes the author's
    PRs whose slots changed;
  - it retries failures from its queue;
  - it debounces and serialises per head, closing the fork race.
- **Retirement:** §9 step 6.

## 11. Decisions for the user

**D1. Which repos get caller PRs.**

- *Recommended:* tenderdash as the pilot, then platform and dash-evo-tool.
  Leave out rust-dashcore and grovedb.
- *Alternatives:*
  - all five;
  - none, with the engine-only fallback in §4: median 23 → 13 min, max
    42 → 39 min, and only after A.

**D2. Which workflows trigger.**

- *Recommended:* the ones that finish last, per repo (§3a): about 158 jobs a
  day, covering 85–94 % of heads, and a list to re-measure now and then.
- *Alternative:* `['**', '!PR Hygiene policy']`: identical callers, never
  stale, every Actions-last head. It costs about 340 jobs a day and more of
  the shared 1,000/h at peak.

**D3. A trigger job whose read fails.**

- *Recommended:* fail it, consistent with A §2b step 5. It may email whoever
  pushed.
- *Alternative:* a warning and exit 0. The head keeps a true pending status
  and the scan retries.

## 12. Seen on the way, out of scope

- `build_verdict` counts everything on the head except this engine's own
  checks. That includes `CodeRabbit`'s status, codecov (a failing
  `codecov/patch` makes grovedb's build `failed`) and the label-driven
  `Claude review` workflow. Whether those should gate is a policy question.
- Platform has 9 open heads at `waiting-build`, most with failed builds that
  have sat there for weeks. They take up the scan's 12 slots. The trigger
  does not use those slots.

## Appendix A: review findings

Five reviewers read v1. Their lenses were correctness, simplicity and
scope, failure modes and safety, fit with Rust and the roadmap, and GitHub
semantics.

**Folded in:**

- serialise per PR, or per head for forks (§3b);
- list only the workflows that finish last, as a constant (§3a, §3f);
- read the head's status first and check the build before snapshotting
  (§3d);
- drop B's own failure rule and use A's (§3d, §10);
- the kill switch and the "never revert" rule (§3a, §7);
- exclude `action_required`;
- the full `check_suite` quote and the documented globs (§2);
- the corrected `pull_requests` evidence and #8 (§2);
- peak-hour cost on the free plan (§5);
- re-harvest the corpus, and the correct cargo command (§3e);
- the differences from the service, and retirement (§9, §10);
- a narrower rollout, and rust-dashcore needing no status (§9);
- tests tightened so that they can fail (§8).

**Rejected:**

- *Validate the event and the SHA in one place only* (simplicity). The
  caller filter saves runners, but the caller is another team's file. The
  engine is the trust boundary, as `event.py` already validates PR numbers
  that `run` checks again.
- *A rate floor (`GET rate_limit`) before each trigger run* (failure
  modes). With §3d's order a non-transition run costs 4–7 requests, and the
  transition run is the scan's own work moved earlier. A's floor guards the
  retries. The rollout measures 403/429s instead.
- *Stage the rollout with today's unscoped selection* (simplicity). There is
  one central PR either way, and `--head` is about 15 lines. The unscoped
  selection on platform costs about 400 requests a run.
- *`--pr N --waiting-on-build` from the payload* (simplicity). It misses
  forks, and a stale number needs the head check anyway.
- *Accept `pull_request_target` CI* (GitHub). Those checks (title, lint,
  milestone, conflict) finish in seconds. The one case they miss, a title
  edit after a green build, falls back to the scan.
- *Exit 0 on a failed trigger job* (failure modes). Left to the user as D3,
  with failing as the default.
- *A path filter against workflows a fork adds with listed names* (failure
  modes). The same author can already start jobs with comments, and the job
  that results trusts nothing new.
- *A Rust test case for `--head`* (Rust fit). `run.rs:155` refuses any
  unknown argument, so the case could never fail.

## Appendix B: how the numbers were measured

All of this is read-only `gh api`, run on 2026-10-27.

- **Scan timing:** `actions/workflows/pr-review-policy.yml/runs?event=schedule`
  on tenderdash for 10-23. Each run was classified as scan or sweep by
  whether the job's `Checkout target default branch` step was skipped.
- **Lag:** for each PR updated since 10-13, take the `PR Hygiene` statuses on
  its head. Find the first status that is not `waiting-build` or
  `Evaluating …` after the last non-Hygiene check run completed, or else the
  merge time.
- **Merges at `waiting-build`:** the last `PR Hygiene` status created before
  `merged_at`, then the unchecked lines of the description's checklist.
- **Which workflow finishes last:** `actions/runs?event=pull_request`,
  grouping runs by `head_sha` and taking the greatest `updated_at`, with
  cancelled and skipped runs excluded.
- **`workflow_run` latency:** the creation time of rust-dashcore's
  `ready-for-review.yml` runs, minus the nearest preceding completion of a
  workflow it lists.
