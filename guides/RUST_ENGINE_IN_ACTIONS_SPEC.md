# Spec: the Rust engine in Actions, then no Python

Status: draft 2, 2026-11-21. Design only; no code yet. Five independent
reviewers read draft 1, each through one lens:

- feasibility against the code;
- parity of switching writers;
- security and supply chain;
- failure modes and rollback;
- simplicity.

Their must-fixes are folded in. Appendix A lists what changed and why.
Appendix B lists the findings not taken.

**Owner decision (2026-11-21).** Stop running half the system in Python.
Switch production to the Rust engine inside the existing reusable workflow,
one repository at a time, then delete Python, without waiting for the hosted
service.

The five governed repositories call
`dashpay/stale_prs_are_bad/.github/workflows/pr-review-reusable.yml@master`:
platform, rust-dashcore, tenderdash, grovedb and dash-evo-tool. Their callers
must not change: they belong to other teams.

## 1. Problem and goal

Every status, record comment, label, checklist and nudge in the five
repositories is written by Python: `pr_review/`, 5.5k lines plus 1.3k of
conformance tooling.

The Rust engine (`engine/`, 22k lines) matches Python on the whole corpus:

- all 306 cases of `conformance/evaluate`;
- `conformance/functions`;
- the synthetic recordings, ordered writes included;
- `pending.txt` is empty.

It also matched Python in the live differential, ordered writes on live
recordings included, over 20+ green runs. The differential has been off since
2026-10-16, when it overran the `pr-hygiene` App's rate limit.

But it has never written to GitHub. Missing:

- a binary;
- a transport that can write;
- a few pieces Python runs that were never ported.

Meanwhile every engine change lands twice: #121, #122 and #123 all did.

**Goal.** The reusable workflow runs a Rust binary that makes the same calls
and the same writes, as the same author and in the same order, as Python does
today. Each repository moves under a switch that can be reverted. Once all
five have run on Rust for a week, `pr_review/` and everything that exists only
to check Rust against Python are deleted.

The service spec's step 2 moves the engine into the service. It then starts
from one engine, one transport and one write path, already used in
production.

**Non-goals:**

- **No behaviour change during the switch.** Fixes and improvements wait
  until Python is gone (§5.3, §15). The one exception: a failure only Rust
  can produce fails closed (§5.4).
- **No change in the governed repositories.** No caller edits and no new
  variables.
- **No change to the trust model.** The writer stays `github-actions[bot]`
  with each repository's `GITHUB_TOKEN`, and policies stay on this
  repository's protected `master`.

## 2. Decisions at a glance

| Question | Decision | Main alternative rejected |
|---|---|---|
| Binary | `pr-review`, a new `[[bin]]` of the `service` crate beside `differential-live`. It mirrors `python3 -m pr_review.main` and `pr_review.event` | a new `cli/` crate plus moving the transport into a `github/` crate (§4.1) |
| Runtime | The engine stays synchronous. `main` builds a tokio runtime and runs the engine on a blocking thread, as the service does | a second, blocking transport (§5.1) |
| Transport | The service's `HttpTransport`. A new `Writes` layer sits beside `ReadOnly` and pins an allowlist of the engine's own writes | a CLI-only transport (§5.1) |
| Retry and rate-limit behaviour at the switch | Python's rules. The per-request timeout is raised to Python's 60 s | adding fixes during the switch (§5.3) |
| Delivery | A prebuilt static binary (`x86_64-unknown-linux-musl`) per `master` commit, with an artifact attestation. The binary for `job.workflow_sha` is downloaded and its attestation verified against that commit | compiling in the caller's job; a checksum pinned in the same commit; a container image (§7.1) |
| Binary not available | Until Python is deleted, that run uses Python, with a warning. After deletion, a bounded wait, then the event's pull request is marked `pending` | an 8-minute wait that leaves a stale green (§7.2) |
| Per-repository switch | An inline list in the reusable workflow: `contains(fromJSON('[…]'), github.repository)` | `policies/engines.txt`; a key in `repositories.json`; variables in the callers' repositories (§8.1) |
| Rollback | Remove one line, merged by an admin through the ruleset's pull-request bypass, in about 2 minutes. `PR_REVIEW_AUTOMATION_ENABLED=false` stays the emergency brake | (§8.3) |
| Evidence before a switch | The corpus, extended with failed writes; mock-server tests of every write route; one comparison of the dashboard export | a live dry-run shadow before switching (Appendix B) |
| Evidence after a switch | **A reverse check.** After each real Rust run, Python makes a recorded dry run of the same work and must want to write nothing. It runs in a read-only job, within a budget | none, or only a one-off rollback drill (§9.3) |
| Order | grovedb → tenderdash and dash-evo-tool → rust-dashcore → platform | (§8.4) |
| Callers pinned to a commit | Refused: the gate accepts only `@master`. All five callers use `@master` today, and binaries exist only for `master` commits | keeping commit pins (§4.2) |
| Digest (`pr_review.aggregate`) | **Delete** (owner decision D1) | port to Rust (§3) |
| After Python | The corpus becomes frozen goldens. An update mode for the corpus tests is built with the first behaviour change, under rules fixed now. `pycompat` stays | regenerating goldens from Rust's own answers, unreviewed (§10) |

## 3. Inventory: every Python entry point and what replaces it

| Python | Called from | Replacement |
|---|---|---|
| `pr_review.event` (event routing) | `pr-review-reusable.yml` | `pr-review event` (§4.2), ported |
| `pr_review.main sync`, with `--pr`, `--batch-size`, all, `--waiting-on-build`, `--apply` and `--format` | through `event` | `pr-review sync` with the same flags (§4.2), over the ported `reconcile` and the build scan (§6) |
| `pr_review.main report --format json` | the export in `pr-hygiene.yml` (App token) | `pr-review report`, always read-only |
| `pr_review.main validate` (`--repository-root`) | the reusable workflow on sweeps; `pr-review-tests.yml` | `pr-review validate`, with the path checks ported (§6) |
| `pr_review.main codeowners --check` | the reusable workflow on sweeps | `pr-review codeowners [--check]` (§6) |
| The inline Python step "Resolve the engine pin…" | the reusable workflow's first step | `pr-review gate` (§4.2), run after the binary is verified |
| `pr_review.registry` (strict schema) | everything | a Rust port with Python's exact rules (§6) |
| `pr_review.aggregate report`, `notifications.py` and `policies/slack.json` | `pr-review-digest.yml`, nightly | **Deleted** (D1). Slack is not configured (`channel_id` and every user are `null`), so the job only writes a Markdown summary. The service's `/api/v1/people/{login}` serves the same content. Slack delivery belongs to the service (service spec §2) |
| `pr_review.rollout` | `pr-review-tests.yml` (actionlint over its packets); onboarding | the generated caller, committed once as `templates/pr-review-policy.yml` (`@master`), plus `pr-review codeowners` |
| `.github/scripts/repo_checks.py bots` | `pr-review-tests.yml` | about ten lines of bash with `yq` (preinstalled on ubuntu-latest): CodeRabbit's `auto_review` against `required_bots` |
| `.github/scripts/repo_checks.py pins`, and the loop validating policies against every pinned engine | `pr-review-tests.yml` | **Gone.** The gate accepts only `@master`, and a pinned caller fails loudly when it runs. The policy check runs `pr-review validate` from the proposed build and from `master`'s released binary. The second covers runs that resolved the previous commit but read the merged policies |
| `pr_review.conformance` (record, harvest, replay, redact) | the differential; corpus freshness; the reverse check (§9.3) until P7 | deleted in P7; the corpus is frozen (§10) |
| `conformance/synthetic/generate.py`, `conformance/pycompat/generate.py`, `generate_policy.py` | by hand; freshness checks | run once more for the new cases (P1), then deleted. Their outputs stay as frozen goldens |
| `pr_review/tests/*` | `pr-review-tests.yml` | The engine-level tests are mirrored by the corpus and 588 Rust tests. P7 carries a table mapping the remaining six files to their Rust counterparts: `test_event`, `test_bootstrap`, `test_workflow`, `test_repositories`, `test_repo_checks` and `test_rollout`, about 700 lines |
| Inline `python3 -c` registry listings | `pr-hygiene.yml`, `pr-review-tests.yml` | `jq` (GitHub-hosted runners have it) |
| Inline `python3` in `pr-hygiene-post.yml` | the post job | `jq -rn --arg u "$SERVICE_URL" '$u\|@uri'` and `jq -r .value` |
| Inline `python3` in `pr-review-context-test.yml` | its caller, `pr-review-tests.yml` | bash `[[ =~ ]]` checks; the calling job moves to `rust.yml` |
| `engine-differential.yml` (disabled), `engine/src/bin/differential.rs`, `service/src/bin/differential-live.rs`, `engine/src/conformance/live.rs`, `service/src/differential*`, `engine/tests/differential/`, `engine/tests/reconcile/live.rs`, `service/tests/differential_live.rs` | the differential | deleted in P7 |
| `site/fixtures/serve.py` (a development-only mock of the sign-in routes) | by hand | owner decision D7: keep it; it is not part of the engine |
| `actions/setup-python`, and the hash pin of `pyyaml==6.0.3` | workflows | deleted in P7 |

**Rust code that reads Python's source today.** P7 repoints or deletes each:

- **`OwnWords`**, used by `engine/tests/policy/corpus.rs` and
  `engine/tests/reconcile/recordings.rs`, reads `_OWN_BLOCKERS` from
  `conformance.py` and the messages `policy.py` raises. It becomes a
  committed list, written once from today's source.
- **`engine/tests/reconcile/scene.rs`** reads
  `pr_review/tests/fixtures/platform.json`. The fixture moves under
  `engine/tests/`.
- **Two tests exist only to match Python**: `src/dashboard.rs` (it reads
  `BOTS` from `policy.py`) and `service/src/digest.rs` (fragments of
  `main.py`). They are deleted.
- **The differential binaries**, `engine/src/conformance/live.rs` and
  `service/src/differential/tests.rs` are deleted with the differential.

## 4. The Rust CLI: `pr-review`

### 4.1 Placement

`pr-review` is a new `[[bin]]` of `service`, as `differential-live` already
is, and like it is not part of the service's image. It uses `service`'s
`reader` module. A new constructor there, `write_transport`, sits beside
`read_only_transport` (§5.2).

*Rejected (draft 1): a new `cli/` crate, with the transport moved into a
shared `github/` crate.* The move is larger than it looks:

- `TokenSource` returns a `BoxFuture` of `AppAuthError`;
- `USER_AGENT` is a constant;
- `InstallationTokens` uses `pub(super)` helpers;
- test helpers are shared across modules.

What the move buys is a smaller dependency tree, which matters for build-time
supply-chain risk. §7.3 handles that risk with `cargo deny` instead. Step 2
wants the transport in `service` anyway. If the static build or an audit
proves the larger tree painful, the move can still be made then.

### 4.2 Subcommands and flags

Only what the workflows call, with Python's rules for each:

```
pr-review sync --repo R --policies-root D [--repository-root D]
               [--pr N | --batch-size K | --waiting-on-build] [--apply] [--format markdown|json]
pr-review report --repo R --policies-root D [--format markdown|json]
pr-review validate --repo R --policies-root D [--repository-root D]
pr-review codeowners --repo R --policies-root D [--check --repository-root D]
pr-review event                       # environment-driven, as pr_review.event
pr-review gate                        # the reusable workflow's bootstrap checks
pr-review --version                   # the commit it was built from
```

- **Rules kept from `main.py`:**
  - `--pr` must be positive.
  - `--batch-size` requires `sync`, a positive size, and no `--pr`.
  - `--waiting-on-build` excludes both.
  - `--apply` is valid only for `sync`, and only when `GITHUB_ACTIONS=true`,
    `GITHUB_REPOSITORY == --repo` and `PR_REVIEW_AUTOMATION_ENABLED=true`.
  - `codeowners --check` requires `--repository-root`.
  - A usage error exits 2.
- **Not ported:**
  - `--user`, which only the digest used;
  - `--policy`;
  - argparse's abbreviations of long flags. Every workflow spells its flags
    out.
- **`--policies-root` is required.** Python's default, the package's own
  `policies/`, does not exist for a static binary.
- **`event`** reads the same environment as `event.py`:
  `GITHUB_EVENT_PATH`, `GITHUB_EVENT_NAME`, `GITHUB_REPOSITORY`,
  `PR_REVIEW_SCOPE`, `PR_REVIEW_AUTOMATION_ENABLED`, `PR_REVIEW_POLICIES_ROOT`
  and `PR_REVIEW_REPOSITORY_ROOT`.

  It routes each trigger the same way:

  | Trigger | Becomes |
  |---|---|
  | The build-scan cron (`2,32,47 * * * *`) | `--waiting-on-build`, only when writes are on |
  | The sweep cron (`17 * * * *`), `push`, `workflow_dispatch` | `--batch-size 6` |
  | `workflow_dispatch` with scope `all` | a full pass |
  | Pull-request events and comments on a pull request | `--pr N` |
  | An unknown cron | a warning, then the sweep |
  | An unsupported event | an error |

  It stops at the first failing selection.

  The cron strings and the constants `NUDGES_PER_RUN`, `SWEEP_SECONDS`,
  `BUILD_SCAN_SECONDS` and `BUILD_SCAN_SIZE` live in `reconcile`. The replay
  uses the same code, so it also checks the CLI's wiring (§6).
- **`gate`** does what the inline Python does today, from the same
  environment. Its checks:
  - The reusable workflow is named **only** at `refs/heads/master`. A
    commit pin is refused (decision D10).
  - The caller is `.github/workflows/pr-review-policy.yml` of the target.
  - The target's default branch, read with `GET repos/{target}`, matches
    today's `[A-Za-z0-9._/-]+` exactly. That pattern is what keeps a newline
    out of `GITHUB_OUTPUT`.
  - `GET repos/dashpay/stale_prs_are_bad/rules/branches/master` still carries
    the required ruleset, `non_fast_forward` included.
  - It writes `branch=` to `GITHUB_OUTPUT`.

  Both reads go through `ReadOnly`, which today allows only routes below
  `repos/{r}/`. It gets explicit allowances for these two routes.

  **The `compare` check is dropped.** The gate no longer reads
  `compare master...SHA`, because the attestation check (§7.2) proves more:
  the bytes about to run were built by this repository's release workflow, on
  `refs/heads/master`, from exactly the running commit. With the
  `non_fast_forward` rule, "built from `master`'s head" means "merged". The
  `compare` check never stopped a malicious workflow file anyway: the YAML at
  an unmerged commit could skip it.

  **Tests.** Six of `test_bootstrap.py`'s eight cases become `gate` tests,
  plus a new case: a default branch containing a newline. The two cases about
  `compare` become tests of the fetch step's verify flags (§7.2).

### 4.3 What the workflows consume

- **Exit status:**

  | Status | When |
  |---|---|
  | `0` | success |
  | `1` | a handled failure: the errors `main.py` catches, or any pull request's reconciliation failing once every one has had its turn. A panic is caught at the top of `main` (`panic = "unwind"`) and also exits `1` |
  | `2` | a usage error |

  On a failure in `event`, Python prints a traceback, because `event.py`
  catches nothing. The CLI prints `PR Hygiene error: …` instead. This is an
  intended difference.
- **stdout:** the report, byte for byte as Python prints it, in Markdown or
  as JSON with `indent=2` through `pycompat`'s writer. `event` prints one
  report per selection, as today. The workflow appends stdout to
  `$GITHUB_STEP_SUMMARY`.
- **The footer.** `event` adds one footer to its Markdown, holding only engine
  constants and numbers:
  - which engine ran, and the commit it was built from;
  - the writes by kind, each with the pull request numbers it touched
    (`status #12 pending→success`, `comment+ #12`, `label- #9`, …);
  - the requests made.

  `sync` and `report` never print a footer on stdout, so their JSON stays
  valid; `sync` writes it to stderr.
- **stderr:** the engine's log lines (`Reconciler::take_log`), then
  `PR Hygiene error: <message>` on failure.

### 4.4 stderr hygiene

The governed repositories are public, and so are their job logs.

- **What a message may quote:** HTTP status codes, GitHub's top-level
  `message`, and a GraphQL error's `type` and `message`, cut to 300
  characters. That is what Python showed through `gh`.
- **What it never quotes:** a response body, a request body, a comment, a
  description, or a token.
- **Panics.** The panic hook prints `internal error at <file>:<line>` and
  never the payload. It ignores `RUST_BACKTRACE`.
- **The no-content test.** The mock server answers every route with marker
  strings. They appear in body fields, in values of the wrong type
  (`serde_json`'s errors quote the value), and in GraphQL data. The test
  asserts that no marker reaches stdout or stderr. It is modelled on
  `engine/tests/support/no_content.rs`.

### 4.5 Runtime at the edge

`main` builds a multi-thread runtime with 2 workers and runs the command in
`spawn_blocking`. The transport calls `Handle::block_on` from that thread,
the arrangement the service already uses (`service/src/reader/transport.rs`).
So the CLI and step 2 run the same code. Reads and writes stay sequential,
one call at a time.

## 5. The transport

### 5.1 One transport

`HttpTransport` is what the 20+ live differential runs used. A blocking
client for the CLI (`ureq`, or `reqwest::blocking`) would mean writing it all
twice: pagination, failure classes, headers, timeouts and body limits. Two
transports that behave differently is the Python/Rust problem again, in
small.

### 5.2 Layers

```
engine Client ─► Writes (pr-review sync/event) ─► HttpTransport ─► api.github.com
engine Client ─► ReadOnly (report, gate, service) ─► HttpTransport
```

**`Writes`** is new, in `service/src/reader/`.

- **Reads** pass through `ReadOnly`'s check unchanged: plain path segments,
  and only the engine's own GraphQL documents.
- **Writes:** it forwards only the routes below, for exactly one repository
  (`GITHUB_REPOSITORY`), with no query string. `{n}` and `{id}` are digits;
  `{sha}` is 40 hex characters.

| Method and route | Constraint on the body |
|---|---|
| `POST repos/{r}/statuses/{sha}` | `context` is exactly `PR Hygiene`; `state` is pending, success, failure or error; `description` is at most 140 characters; `target_url` is absent or `https://github.com/{r}/actions/runs/{digits}` |
| `POST repos/{r}/issues/{n}/comments` | `{"body": string}` only |
| `PATCH repos/{r}/issues/comments/{id}` | `{"body": string}` only |
| `DELETE repos/{r}/issues/comments/{id}` | no body |
| `POST repos/{r}/issues/{n}/labels` | `{"labels": [one of the engine's labels]}` |
| `DELETE repos/{r}/issues/{n}/labels/{name}` | `name` is one of the engine's labels (`STATE_LABELS`, `RETIRED_LABELS`, `bot-review-skipped`) |
| `PATCH repos/{r}/pulls/{n}` | `{"body": string}` only |
| `POST repos/{r}/pulls/{n}/requested_reviewers` | `{"reviewers": [logins]}` only, never `team_reviewers` |

- **Refused before sending:** anything else. That covers an approval, a
  merge, another status context, another repository, and any GraphQL
  mutation.
- **The comment cap.** `Writes` refuses a third comment creation on one issue
  in one run. The engine creates at most one record or move comment per pull
  request per run, plus one nudge per run (`NUDGES_PER_RUN`), so a third can
  only be a loop.
- **What a refusal does.** It aborts the run. On a `--pr` run the event's
  pull request is marked (§5.4). On a sweep, the remaining pull requests wait
  for the next run.

**`HttpTransport` gains one setting:** a 60 s per-request timeout for the
CLI, which is Python's `gh` timeout. The service keeps its 30 s.

- A slow `POST` therefore fails no sooner than it did in Python.
- A request that times out is also past the call's 60 s deadline, and is not
  asked again, as in Python.

**Already in place, no change:**

- REST bodies are written as Python's `json.dumps` wrote them to `gh`
  (`gh_arguments`);
- an empty `204` reads as `None`;
- no redirect is followed;
- pagination stays on the same origin.

### 5.3 Retries, backoff and rate limits

**At the switch, Python's rules.** If the transport did anything differently,
the set of runs that fail would change, and every comparison the switch
relies on would blur.

- **One retry, after 2 s.** Only an idempotent REST call (GET, PUT, PATCH,
  DELETE) is asked again, and only after a transient failure: a 502, 503 or
  504, a timeout `gh` reported, a lost connection, or an answer cut short.
- **Never retried:** any `POST` (status, comment, label, reviewer request),
  and GraphQL, which is a `POST` in Python too.
- **A GraphQL answer that carries both `data` and `errors`** goes to the
  reader. The history query tolerates only `NOT_FOUND`.
- **A 403 or 429**, from a primary or a secondary limit, fails the call. The
  pull request it hit is marked as Python marks it, and the job fails.
- **Differences already accepted in the service spec:** the transport
  retries on status codes, not on substrings, and it retries a complete
  answer that is not JSON.
- **A GraphQL timeout** (GitHub stops the query at 10 s with a 502 or 504)
  also costs extra points for the next hour [R2].

**After Python, the first Rust-only changes** (§15, backlog). Each is a TDD
change with blessed goldens.

- **B1, GraphQL.**
  - Retry a query once on a 502 or 504.
  - When the history query times out, split its aliases in halves, as #123
    did for the analyzer.

  The history query asks for `comments(last:100)` with full bodies for every
  candidate. On a full pass, and on **every build scan, three times an hour**,
  the candidates are every governed open pull request (§11).
- **B2, secondary limits.**
  - Wait as `retry-after` says, or until `x-ratelimit-reset` when the
    remaining budget is 0, or otherwise at least 60 s.
  - Back off exponentially, within a budget per run [R1].
  - Retry any method once: a request refused for a rate limit was not
    processed.
- **B3, failing closed.** Mark the event's pull request on any failure of an
  event run. That includes `GitHubError`s in shared reads, which Python leaves
  unmarked.

### 5.4 Ordered writes, no duplicates, failing closed

- **Order.**
  - The engine makes each write as a blocking call, in Python's order. The
    transport neither pipelines nor reorders.
  - The replay holds the port to every recorded call, made once and in
    order, and to every write's route and body.
- **No duplicates.**
  - Records and move comments are edited in place.
  - An identical latest status is not posted again.
  - A nudge goes once per head and bot, by its marker.
  - A label is added only when it is absent.
  - A `POST` is never retried.
  - Runs for one pull request are serialised by the reusable workflow's
    `concurrency` group with `queue: max`, which is unchanged.

  The cap in `Writes` is the backstop.
- **Partial writes fail closed.**
  - Before changing a pull request's labels, checklist or record, or posting
    a non-converged status, `publish` posts
    `pending: Evaluating current review policy` (`main.py:846`,
    `publish.rs:350`). A run that dies after that leaves pending, never a
    stale success.
  - Two writes come earlier: the nudge comment (`main.py:1139`), and, on a
    converged pull request, `finish`'s re-read and post. Neither leaves a
    wrong verdict, but neither is preceded by a pending status.
- **Failures only Rust can produce fail closed.** Python never produces:
  - a `Refused` from `Writes`;
  - a `NotPorted` or `Exception` read error;
  - a panic.

  On a `--pr` run, the CLI catches these and marks the event's pull request
  through a fresh transport, as `mark_unreadable` does: it reads the pull
  request, and if it is open and governed, posts `error` with Python's own
  words, `Policy reconciliation failed; inspect workflow log`.
  - Since Python never produces these failures, marking them blurs no
    comparison.
  - Without the mark, an old success could survive a crashed run.

### 5.5 Same author, same bytes

- **The author.** The token is the same `GITHUB_TOKEN`, so GitHub attributes
  every write to `github-actions[bot]`. Both engines recognise their own
  comments and statuses by `ENGINE_LOGINS = ["github-actions[bot]"]`
  (`policy.py:22`, `engine/src/policy/mod.rs:66`).
- **The bytes, for writes that succeed.** The corpus proves them equal:
  - Every replayed write is held to the recorded one: its route, and its
    body as a JSON value (`conformance/run.rs`, `replay.rs`). A comment's
    text is a JSON string, so its marker lines are compared character for
    character.
  - Per verdict, `outputs.json` is compared exactly: `comment`, `markers`,
    `record`, `diff`, `checklist`, `move`, `labels`, `status`, `diff_print`
    and `receipt_prints`.
  - The JSON writer is held to Python 3.12's `json.dumps` by
    `pycompat/json_dumps.json`.
- **Writes that fail.** No recording holds one today: the recorder answers
  every write with success. P1 adds failed writes to the synthetic recordings,
  so that Python's failure branches are compared too: in `publish`, `nudge`,
  `clear_marks` and `_set_aside_record` (§6).
- **What is read back from a write:** only the status answer's `creator` and
  `created_at`, and the comment answer's integer `id`. Both are documented
  fields, and P3's mock tests use GitHub's documented shapes.
- **Why the bytes matter.** `diff_print` and `receipt_print` are persisted in
  the record and compared across runs. A difference would make every pull
  request look as if it carried new work, and would ask for its self-review
  again.
- **`target_url`** is set only on a configuration-error status, to the run's
  URL, in both engines.

## 6. Pieces not yet ported

| Piece | Python | Rust today | Size | Held to Python by |
|---|---|---|---|---|
| Build scan, `--waiting-on-build` | `collect(waiting_on_build=True)`: every governed candidate recorded as `waiting-build`, or, with no record, whose own last status says so. Then `periodic_batch(…, 12, cadence=900)`. A shared-read failure marks nothing | `latest_state_from_status` is ported; the selection is not, and the replay refuses the flag | `Selection::WaitingOnBuild`, about 100 lines | P1: synthetic recordings for none waiting, waiting by record, waiting by status, more than 12 waiting (rotation), and the history unreadable. P2: the replay accepts the flag |
| `periodic_batch` and the run constants | `main.py` | the replay's copy hard-codes 3600 s and logs its clock read by hand | moved into `reconcile`, with a cadence parameter and `ClockSite::PeriodicBatch`, about 40 lines | the batch recordings, now replayed through the production code |
| Event routing | `event.selections`, which reads the environment | — | about 120 lines | P1: a generated table, since the function harvest cannot capture environment reads. `conformance/tables/event.json` holds every event name × payload shape × scope × automation flag, plus the inputs of `test_event.py` |
| Markdown report | `render_report`, `age`, `cell` | `review_text` only | about 60 lines | 8 synthetic recordings already print Markdown, and the replay starts comparing them (P2). P1 adds `--user` and the "No matching PRs." path |
| `validate` with a tree | `validate_policy(policy, root)`, `missing_paths`, and the printed lines | `validate_policy` without the tree | about 60 lines | P1: a table over generated trees, and the five real policies |
| `codeowners` | the generated text; `--check` over three locations | — | about 40 lines | P1: a table of each registered policy's exact file, and check cases |
| Registry | `validate_registry`, `entry_for`, `policy_path` (strict keys) | the analyzer's laxer loader | about 60 lines | P1: a table from the inputs of `test_repositories.py` |
| Failed writes | handled in `publish`, `nudge`, `clear_marks` and `_set_aside_record` | ported, never compared | — | P1: the recorder's write answers become scriptable, and `generate.py` records five failures: 404 removing a label, 422 requesting a reviewer, 403 commenting on a locked conversation, 422 editing a description, 502 posting a status. P2: the replay serves recorded failures |
| Configuration error; status page | — | ported | — | the existing cases |
| Snapshot concurrency | four threads | sequential | — | already accepted: the same answers, and no more reads |

P1's tables (`conformance/tables/`) come from one generator, with its own
freshness check in `pr-review-tests.yml`. A separate generator is needed for
two reasons:

- the function harvest records only the JSON arguments of calls the test
  suite makes;
- its freshness check rejects any file it did not write.

## 7. Build and delivery

### 7.1 Options

Measured:

- **Today.** grovedb's run 44874736744, on a `dash-x64-light` self-hosted
  runner, took 41 s: 17 s to set up Python, and 15 s to reconcile.
- **A cold Rust release build.** Engine-differential run 38812154717, on
  ubuntu-latest, took 9 s for the toolchain and 2 min 14 s to build the
  engine's `differential` and the service's `differential-live`.

| Option | Event latency | Cost and risk | Verdict |
|---|---|---|---|
| **A. Compile in the governed job at `job.workflow_sha`, with `Swatinem/rust-cache`** | about 2–2.5 min cold; about 30–45 s warm | Caches belong to the caller's repository and branch [R3][R4]. `pull_request_target` and `issue_comment` runs get read-only cache access [R3], so every event after an engine merge compiles cold until a scheduled run saves a cache. It spends the governed repositories' 10 GB cache quota, platform's included [R3]. Self-hosted runners need a toolchain | rejected |
| **A′. As A, but cache only the binary, keyed by commit** | cold first, then about 2 s | Events compile cold after each merge. Cache entries are not signed [R3] | rejected |
| **B. Prebuilt binary from this repository, with artifact attestation, keyed by `job.workflow_sha`** | about 2–5 s | Needs a release pipeline. Attestations are available for public repositories on every plan [R5], and this one is public. Sigstore becomes a runtime dependency | **chosen** |
| **C. Prebuilt binary with its SHA-256 pinned in the same commit** | about 2 s | Impossible for the commit's own build: it would need a second "bump the hash" PR after every change. `setup-uv` and `install-action` pin *released* tools this way [R7][R8], which fits a versioned tool, not "the engine at this commit" | rejected |
| **D. Container image** | an image pull | Docker on the self-hosted runners, plus C's pinning problem or B's verification | rejected |

### 7.2 Chosen design

**`engine-release.yml`**, in this repository.

- **Triggers, exactly these:**
  - `push` to `master`;
  - `workflow_dispatch`, with no inputs;
  - `schedule` every 30 minutes, building `master`'s head only if its assets
    are missing. This covers a push run that was skipped or failed. A
    dispatch can attest only the head, so a superseded commit can never get
    a binary later.

  It never has `workflow_call`.
- **What it builds:** `github.sha` only.
- **Concurrency:** `engine-release-${{ github.sha }}`, without
  `cancel-in-progress`.
- **A workflow test** holds the triggers, the checkout of `github.sha`, and
  the permissions below.

It has two jobs:

1. **`build`**, on ubuntu-latest, with `contents: read`. It takes about
   3 min:
   - Check out with `persist-credentials: false`.
   - Install `dtolnay/rust-toolchain` (pinned by SHA), the
     `x86_64-unknown-linux-musl` target, and `musl-tools`.
   - Run `cargo deny check advisories bans sources`: crates.io only, no git
     dependencies.
   - Run `cargo build --release --locked --target x86_64-unknown-linux-musl -p pr-hygiene-service --bin pr-review`,
     **with no cache**.
   - Smoke test: `pr-review --version`, then `pr-review validate` for every
     registered policy.
   - Upload the binary as a workflow artifact.
2. **`publish`**, on ubuntu-latest, with `contents: write`,
   `id-token: write` and `attestations: write`. It runs no third-party code
   beyond pinned actions:
   - Download the artifact.
   - Attest it with `actions/attest-build-provenance`.
   - If both assets for this commit already exist, stop: assets are never
     replaced.
   - Upload the bundle first, then the binary, to **one rolling release,
     `engine`**: `pr-review-<sha>-x86_64-unknown-linux-musl.sigstore.json`,
     then `pr-review-<sha>-x86_64-unknown-linux-musl`.
   - Prune the assets of commits superseded more than 30 days ago, always
     keeping the newest 20. A re-run uses its first attempt's commit [R4],
     and a commit can stay `master`'s head for weeks.

`rust.yml` gains a job that builds the musl binary on every pull request, so
a merge cannot break the release build unnoticed.

**The fetch step** in the reusable workflow (Rust path):

```sh
# SHA comes from env (job.workflow_sha), never interpolated into the script.
[[ $SHA =~ ^[0-9a-f]{40}$ ]] || { echo "::error::bad workflow sha"; exit 1; }
[[ $RUNNER_OS == Linux && $RUNNER_ARCH == X64 ]] || exit 1
# gh: a version with the fix for CVE-2025-25204, in a fresh config and cache
ver=$(gh --version | sed -n 's/^gh version \([0-9.]*\).*/\1/p'); version_at_least "$ver" 2.67.0 || exit 1
export GH_CONFIG_DIR=$(mktemp -d) XDG_CACHE_HOME=$(mktemp -d)
asset=pr-review-$SHA-x86_64-unknown-linux-musl
base=https://github.com/dashpay/stale_prs_are_bad/releases/download/engine
curl -fsSL -o "$RUNNER_TEMP/pr-review.sigstore.json" "$base/$asset.sigstore.json" &&
curl -fsSL -o "$RUNNER_TEMP/pr-review" "$base/$asset" &&
gh attestation verify "$RUNNER_TEMP/pr-review" --bundle "$RUNNER_TEMP/pr-review.sigstore.json" \
  --repo dashpay/stale_prs_are_bad \
  --cert-identity https://github.com/dashpay/stale_prs_are_bad/.github/workflows/engine-release.yml@refs/heads/master \
  --source-digest "$SHA" --signer-digest "$SHA" --source-ref refs/heads/master \
  --deny-self-hosted-runners --format json --jq '…'   # non-empty; checks buildSignerURI,
                                                     # sourceRepositoryDigest, runnerEnvironment
sha256sum "$RUNNER_TEMP/pr-review" > "$RUNNER_TEMP/pr-review.sha256"   # checked again before each use
```

- **Why each flag** [R6]:
  - `--cert-identity` pins both the signing workflow and its ref.
  - `--source-digest` and `--signer-digest` bind the bytes to the very
    commit whose reusable workflow is running.
  - `--deny-self-hosted-runners` refuses a build from a runner this
    repository does not control.
- **The version floor and the JSON check.** `gh` 2.49.0 up to 2.66.x accepted
  a bundle holding only an attestation of another predicate type
  ([CVE-2025-25204](https://github.com/cli/cli/security/advisories/GHSA-fgw4-v983-mgp8)).
  The JSON check guards against a verify that exits 0 with nothing in it.
- **Why the bundle file.** Verifying against it costs the caller's API
  budget nothing. It also sidesteps whether one repository's `GITHUB_TOKEN`
  may read another's attestations. P5 tests, on both runner types, that it
  makes no GitHub API call.
- **Self-hosted runner state.** Fresh `GH_CONFIG_DIR` and `XDG_CACHE_HOME`
  keep anything a self-hosted runner kept from an earlier job out of the
  check. The step logs `gh`'s path and version. `sha256sum -c` runs before
  every later call of the binary.

**When the binary is not available** (just merged, a failed build, a Sigstore
outage, a tampered asset):

- **Until P7, while a Python path exists.** The step tries once. On any
  failure, the run uses Python, with a `::warning::` and a summary line
  naming the reason.
  - Python writes the same bytes (§5.5), so nothing waits, and no stale green
    survives the window after a merge.
  - The footer says which engine ran. The exit criteria (§8.4) count only
    Rust runs, and a release pipeline stuck on Python shows.
- **After P7:**
  - The step polls for both assets for up to 8 minutes, every 15 s with
    jitter (the runners share one outbound IP). It treats a 404 or a failed
    verify as retryable.
  - If the binary is still missing, it posts
    `pending: PR Hygiene engine unavailable; reconciliation required` on the
    event's head with `gh api`, and fails. The head comes from the payload;
    for `issue_comment`, from one GET of the pull request.
  - Sweeps and build scans mark nothing, as Python's shared-read rule says.
  - The step has `timeout-minutes: 10`.

**Then the run proceeds:**

1. `pr-review gate`;
2. the existing checkouts: live policies, and the target tree on sweeps;
3. on sweeps, `pr-review validate` and `codeowners --check`;
4. `pr-review event`, with `timeout-minutes: 30`.

The engine's full checkout is no longer needed on this path.

**Latency.** Fetching and verifying takes about 2–5 s, so an event run on
Rust should take about 20–25 s, against 41 s today.

### 7.3 Supply chain

| Threat | Today (Python) | With B |
|---|---|---|
| A malicious commit on `master` | runs with each repository's write token | the same; the trust root is unchanged |
| Write access to this repository is used to replace or forge an asset | — | the verify rejects it: identity, both digests, ref, predicate type and the `gh` floor |
| A workflow on a branch, or one that calls the release workflow, makes an attestation | — | `--cert-identity …@refs/heads/master`, and the release workflow has no `workflow_call` and no inputs. A test holds both |
| A dependency's `build.rs` mints provenance or touches the assets | — | `build` holds no `id-token` and no `contents: write` |
| A compromised dependency crate in a binary holding five repositories' write tokens | Python had no third-party runtime code beyond `gh` | **A new risk.** Mitigated by `cargo deny` (advisories and sources), `--locked`, and review of every `Cargo.lock` change. It is accepted as the cost of the port; it is the dependency set the service already runs |
| Release assets deleted: an availability attack on a required check | — | Until P7, the run falls back to Python; after P7, a dispatch republishes. Separately, `pr-hygiene.yml`'s `data` checkout keeps a `contents: write` token in `.git/config` while `cargo build` runs (`pr-hygiene.yml:55-69`). P5 sets `persist-credentials: false` and hands the token only to the push step |
| A self-hosted runner's persistent state: `gh` on the PATH, its config, its TUF cache, the binary between steps | setup-python's tool cache has the same exposure | fresh `GH_CONFIG_DIR` and `XDG_CACHE_HOME`; `gh`'s path and version logged; the hash checked before each use. The assumption is stated: the runner itself is trusted, as it is today |
| A Sigstore or CDN outage | — | until P7, Python. After P7, a delay, then the head is marked pending. D9 decides the trust root |
| Cache poisoning | — | neither the release build nor the governed jobs use a cache |

## 8. The per-repository switch

### 8.1 Where it lives

The switch is an inline list in the reusable workflow:

```yaml
env:
  # Repositories whose writes the Rust engine makes. Others run Python.
  RUST_ENGINE: ${{ contains(fromJSON('["dashpay/grovedb"]'), github.repository) }}
```

- **Case.** GitHub's expression functions compare strings ignoring case.
- **Which runs it reaches.** The list is read from the workflow at
  `job.workflow_sha`, so it applies to runs created after the change merges.
  Runs already queued keep the list they started with; a per-pull-request
  queue holds at most a few.
- **Why not `policies/repositories.json`.** `validate_registry` refuses
  unknown keys. Every Python run that read the new registry, including runs
  already in flight, would fail with a configuration error. `engine_revision`
  is that scar.
- **Why not `policies/engines.txt` (draft 1).** A file read live from
  `master` also redirects queued runs. But it has to be parsed before
  anything else, needs a validation test, and moves a checkout. Two states
  fit in one expression.
- **Why not variables in the callers' repositories.** They are instant, but
  they are settings in other teams' repositories, unreviewed and outside git
  history.

### 8.2 The reusable workflow during the transition

```
jobs:
  reconcile:                                   # permissions, concurrency, triggers unchanged
    steps:
      ── RUST_ENGINE == 'true' ──
      1 Fetch and verify (§7.2) → steps.fetch.outcome
      2 [fetch ok] pr-review gate; Checkout live policies; [sweep] Checkout target;
        [sweep] pr-review validate / codeowners --check >> summary (continue-on-error, as today);
        pr-review event >> $GITHUB_STEP_SUMMARY                     # id: rust
      ── RUST_ENGINE != 'true' or fetch failed (warning) ──
      3 today's steps unchanged: Set up Python; Resolve (inline gate); Checkout engine;
        Checkout live policies; [sweep] target; validate + codeowners; python3 -m pr_review.event  # id: python
      4 always(): assert exactly one of steps.rust / steps.python ran; summary names it
  parity:                                      # §9.3; only when steps.rust ran
    needs: reconcile
    if: ${{ !cancelled() && needs.reconcile.outputs.engine == 'rust' }}
    permissions: { contents: read, pull-requests: read, issues: read, statuses: read }
```

- **Expressions kept verbatim:** the `PR_REVIEW_REPOSITORY_ROOT` expression,
  and the condition that clones the tree on sweeps.
- **No silent skip.** Step 4 makes a run in which no engine ran fail rather
  than pass green.
- **The workflow tests move to Rust.** `test_workflow.py` and
  `test_bootstrap.py` become `service/tests/workflow.rs`, which reads the YAML
  with `serde_yaml`. They also check every engine against every event, the
  fallback, and the assertion step.

### 8.3 Rollback

- **How.** Remove the repository from the list.
  - `master`'s ruleset 22953412 lets the admin role bypass its pull-request
    rule, and with it the required checks (`RepositoryRole` 5,
    `bypass_mode: pull_request`). An admin merges without waiting for an
    approval: about 2 minutes end to end.
  - The revert's own commit needs no binary, because Python runs.
  - Without an admin, it takes one code owner plus the checks, which took
    1–8 minutes in recent runs.
- **The emergency brake.** `PR_REVIEW_AUTOMATION_ENABLED=false`, set in the
  governed repository, stops all writes by either engine at once.
- **Safe with records Rust wrote.** The bytes and the author are the same
  (§5.5), and the reverse check (§9.3) proves it on every run.
- **Rollback does not repair what Rust got wrong.** Python carries some
  record fields forward rather than recomputing them: `admitted_at`,
  `ready_since`, receipts, `diff_heads` and `diff_seen` (`main.py:355-381`,
  `policy.py:180, 494`). A wrong value Rust wrote survives the rollback.

  The runbook for a divergence:
  1. Roll back.
  2. List the pull requests Rust wrote to, from the footers.
  3. For each, compare its record with what Python would write; Python's
     recorded dry sync of that pull request shows it.
  4. Where a carried field is wrong, correct it by hand, by editing the
     record comment, which the engine reads back.
  5. Dispatch a sweep, or wait for each pull request's next event.

### 8.4 Order and exit criteria

| Stage | Change | Soak | Exit criteria |
|---|---|---|---|
| S0 | The export. One dispatched comparison per repository of Python's and Rust's `report --format json`, after checking the App's remaining budget. Then `pr-hygiene.yml` builds `pr-review` and exports with it | one run | equal after removing `generated_at`, or each difference explained |
| S1 | grovedb → Rust | 3 days | (1) Rust ran in every run, or each fallback has a known cause. (2) Every run is green, or red only for causes Python shares. (3) The footers are plausible: no comment created twice on one pull request across runs, and no label flip-flop. (4) The reverse check (§9.3) finds zero unexplained would-be writes. (5) The **rollback drill** passes |
| S2 | tenderdash and dash-evo-tool → Rust | 3 days | as S1, without the drill |
| S3 | rust-dashcore → Rust | 5–7 days | as S1, plus at least one completed `scope=all` dispatch |
| S4 | platform → Rust | 7 days | as S1; the reverse check runs on sweeps only, within its budget |
| S5 | delete Python (P7) | after a week with all five on Rust | — |

From P1 to S5, engine behaviour changes are frozen except for fixes, and
every fix lands in both engines (owner decision D5). S1 to S5 takes about 4
weeks.

## 9. Verification

### 9.1 What the corpus proves, and what it does not

**It proves:** given Python's reads, Rust reaches the same verdicts. It makes
the same ordered writes, with the same bytes, and reads the clock at the
same points. Until 2026-10-16 the live differential also showed that Rust's
live reads through `HttpTransport` equal Python's.

**It does not prove:**

- **Failed writes.** P1 and P2 close this gap (§6).
- **The write half of `HttpTransport` against real GitHub.** P3's mock tests
  use the documented shapes; the first real write is grovedb's.
- **The CLI:** routing, argv, the environment gates, exit codes and the
  summary.
- **Delivery:** the download, the verify, and `gh` on the self-hosted
  runners.
- **Live behaviour since 2026-10-16.** #121 and #122 have not run live.
- **Reads with `GITHUB_TOKEN`.** The differential read with the App's token.
  With the Actions token, the collaborator listing leaves out org-level
  admins, so `permission()` reads per login (`github.py:988`). That path is
  replayed, but has never been seen live in Rust.

### 9.2 Before the first switch: the minimum honest evidence

1. **The corpus, extended** by P1 and P2: the build scan, failed writes, the
   Markdown report, and tables for the CLI's pieces.
2. **Mock-server tests** of every write route through `Writes` and
   `HttpTransport` (P3). They use GitHub's documented answers:
   - `201` with `creator` and `id`;
   - `204`;
   - `404` on removing a label;
   - `422` on a reviewer request;
   - `403` on a locked conversation.

   They also cover `Writes`'s refusals and cap, the no-content check (§4.4),
   the argv rules, and the event table.
3. **The export comparison (S0).** It exercises `report` and the evidence
   reader, with the same App token as before, on all five repositories.

There is no forward shadow (Appendix B says why). The first real Rust writes
go to grovedb, the smallest repository, with 1–3 open pull requests. Four
things protect them:

- the fallback to Python (§7.2);
- failing closed on Rust-only failures (§5.4);
- the reverse check, from the first run (§9.3);
- a rollback in about 2 minutes.

A non-governed sandbox repository could take the first real writes instead
(owner decision D3). The recommendation is not to: it needs an owner action
and a second test account to cover approvals, and grovedb's exposure is a few
pull requests for a few minutes, failing closed.

### 9.3 During S1–S4: the reverse check

After every run in which the Rust engine wrote, the `parity` job:

1. **Holds a read-only token.** Its permissions are `contents`,
   `pull-requests`, `issues` and `statuses`, all read. That keeps the
   README's rule: "record with a read-only token".
2. **Sets up Python** and checks out this repository at the job's commit.
   It also checks out the live policies from `master`, as the real run does,
   and passes them with `--policies-root`.
3. **Checks the budget** with `gh api rate_limit`, which is not counted
   against the limit [R11]. It runs only when
   `core.remaining − 150 ≥ 0.6 × core.limit × (seconds to reset / 3600)`,
   and the same holds for GraphQL. The reserve shrinks as the hour runs out,
   so real runs are never starved. It never runs on `scope=all` or on build
   scans, and on platform it runs on sweeps only.
4. **Records a dry run in Python** of each selection that `event.selections`
   computes for this event: `--record DIR`, into a mode-700 directory under
   `$RUNNER_TEMP`. Such a dry sync walks the write path and answers every
   write itself; none is sent.
5. **Counts the would-be writes** (`kind: write` lines in `calls.jsonl`). It
   prints only their kind, pull request number and, for a status, the state
   from and to. Then it deletes the directory, in an `always()` step.

**Expected: zero.** Python, reading the state Rust has just written, wants to
change nothing. On every run, not just the drill's day, that shows:

- Python can continue Rust's state, so a rollback is safe;
- Python would accept Rust's live writes.

**Explained would-be writes.** Python starts 30–60 s after Rust ends, so
some are expected:

- evidence that changed with no event: checks finishing, the status page
  payload, a permission, the base commit;
- the nudge allowance, which is one nudge per *run*: a second pull request
  owed one is nudged on the next run;
- a write Rust attempted and GitHub refused, which shows in Rust's footer and
  stderr.

Anything else is a port bug, and the repository rolls back until it is fixed.

**What it does not prove: byte equality.** Python's "already correct?"
checks normalise line endings and the whitespace around the checklist and the
move text. Byte equality comes from the corpus (§5.5); this check shows
acceptance and continuity.

**Its cost** is about the same again as the Rust run, inside the budget
rule. It is deleted with Python.

**The rollback drill (S1, grovedb, once):**

1. After a day on Rust, read each open pull request's state: its latest
   `PR Hygiene` status id, its record's `updated_at`, a hash of its
   description, and its labels. That is about 10 read-only calls.
2. Remove grovedb from the list and dispatch `scope=all`.
3. Read the state again. Python must have written nothing; an identical
   status is not posted again.
4. Put grovedb back.

## 10. After Python

### 10.1 The corpus becomes frozen goldens

Python is the oracle until P7. After that, nothing can say what Python would
answer. The corpus becomes the regression baseline. The Rust scenario tests
on the stateful fake (`engine/tests/reconcile/fake.rs`) become the
specification.

| Corpus part | After Python |
|---|---|
| `conformance/pycompat/*` | **Frozen for good.** These are facts about Python 3.12 that the persisted formats and text semantics rest on |
| Function cases whose output is persisted: `state_comment_body`, `state_record`, `diff_record`, `diff_print`, `receipt_print` | **Frozen for good.** Changing a persisted format is a deliberate new marker version, never a side effect |
| `conformance/evaluate`, other function cases, `conformance/tables` | **Updatable outputs.** The inputs stay fixed |
| `conformance/synthetic/*` | **Updatable writes, frozen reads.** An update may rewrite `verdicts.json`, `outputs.json`, `printed.txt` and the write lines of `calls.jsonl`. It may never change the sequence of reads, because no GitHub is left to answer new ones |
| `pending.txt` and its gate | **Deleted.** There is nothing left to be pending against |

**The update mode is built with the first behaviour change after P7, not
before.** None of B1–B3 changes a golden: they change failure paths, and B1
changes reads. Its rules are fixed now:

1. An environment variable on the existing corpus tests,
   `CONFORMANCE_UPDATE=1`, rewrites the updatable outputs from Rust's current
   answers. It writes them through `pycompat`'s writer, so the diff stays
   minimal.
2. It refuses to touch the frozen parts.
3. It refuses any recording whose read sequence changed. Such a change
   retires the recording, which is listed with its reason in
   `conformance/retired.txt`. What it pinned is restated as a scenario on the
   fake.
4. It never runs in CI, and CI fails on any difference not updated.

The process for a behaviour change:

1. Write a scenario or function test that names the change, and see it fail.
2. Change the engine.
3. Run the update.
4. Review every changed case in the PR's diff.

### 10.2 `pycompat` stays

Record continuity needs it. The marker JSON is compact, with sorted keys and
`ensure_ascii`. `diff_print` and `receipt_print` are persisted and read back.
All of these must stay byte-stable across runs, and across writers until
step 2's cut-over.

The regex classes and string methods are the engine's semantics: `\s` is
Python's `\s`. Replacing `PyValue` with typed structs, or adopting the regex
crate's newer Unicode data, is a large refactor that users would not see. It
is not part of this plan. If it is ever done, the frozen goldens hold it.

### 10.3 What is deleted, and in which PR

**P7**, after S4 plus a week, deletes:

- **Python itself:**
  - `pr_review/`, its code and its tests;
  - `.github/scripts/repo_checks.py`;
  - `pr_review.rollout`, replaced by the caller template and the `yq` check.
- **In the reusable workflow:** the Python path, the fallback, and the
  `parity` job. The Rust path's handling of a missing binary after P7 lands
  here (§7.2).
- **Workflows:** `engine-differential.yml`, and `pr-review-digest.yml` (D1).
- **The differential's code:** `engine/src/bin/differential.rs`,
  `service/src/bin/differential-live.rs`, `engine/src/conformance/live.rs`,
  `service/src/differential*`, `engine/tests/differential/`,
  `engine/tests/reconcile/live.rs` and `service/tests/differential_live.rs`.
- **Conformance tooling:** `conformance/*/generate*.py`, the table generator,
  `pending.txt` and its gate.
- **`pr-review-tests.yml`**, replaced by a `policies` job in `rust.yml`:
  - `pr-review validate` of every policy against a fresh clone, from both the
    proposed build and `master`'s released binary;
  - the `yq` bot check;
  - actionlint on the workflows and the caller template;
  - the `pr-review-context-test.yml` identity job, moved here.
- **The rest of Python:** `setup-python`, the `pyyaml` pin, and every inline
  `python3`.
- **The two tests that match Python's source,** in `src/dashboard.rs` and
  `service/src/digest.rs`.

P7 also:

- freezes `OwnWords` into a committed list;
- moves the `scene.rs` fixture;
- updates the docs: `README.md`, `guides/PR_REVIEW_OPERATIONS.md`,
  `guides/PR_REVIEW_ARCHITECTURE.md`, and `conformance/README.md`, rewritten
  as "frozen goldens and how to update them";
- removes `CODEOWNERS` lines for `pr_review/`.

### 10.4 Effect on the service spec (`guides/PR_HYGIENE_SERVICE_SPEC.md`, branch `shumkov/dashboard`)

This spec does not edit it. At its next revision, the service spec should
change in these places:

- **§6f.** "Rust vs Python on the board" goes away: from S0, the dashboard
  export is the Rust `report`.
- **§6c, scheduling.** "Actions-only scheduling is not ported" no longer
  holds. The build scan, `periodic_batch` and the Markdown report are in
  `reconcile`; the service may ignore them.
- **§6c, writes.** The write layer (`Writes`) is born in
  `service/src/reader/` and used in production by Actions. At step 2 the
  service reuses it with App installation tokens. Only the token source and
  the author change.
- **§6c 1b-5.** The differential retires with Python. Entry to step 2, per
  repository, becomes "two weeks on the Rust engine in Actions, and 1b-6
  agrees with GitHub's state". It was "`pending.txt` empty plus 3–4 weeks of
  1b-5 and 1b-6". Both writers now run the same engine, so 1b-6 measures only
  what differs between events and selections.
- **§2 and §6.** "Engine rule changes land in both or wait" ends at P7. The
  Python engine retires before step 3, so step 3 retires only the callers
  and the reusable workflow.
- **Trust model.** Unchanged until step 2: `github-actions[bot]`, and
  `master` as the trust root. The attestation adds a check of what runs, not
  a new root.

## 11. Failure modes

| Failure | What happens | Bound, or why it is acceptable |
|---|---|---|
| **A Rust panic, or a `Refused`, `NotPorted` or `Exception`, on a `--pr` run** | Caught. The event's pull request is marked `error` (§5.4); exit 1 | Fails closed. These failures are Rust-only, so marking them blurs no comparison |
| **The same, on a sweep or a build scan** | exit 1; the job is red; the remaining pull requests wait | Python's rule for shared reads. A pull request whose writes had begun is already `pending` |
| **Rust fails a read Python would not** (`GitHubError`) | marked as Python marks it; red | Surfaced by the reverse check and the footers. Roll back |
| **Divergence after a switch** | a wrong status, checklist or comment | Roll back in about 2 minutes, then the runbook (§8.3), carried fields included. Fix test-first and switch again |
| **No binary, until P7** (just merged, a failed build, a Sigstore outage, a tampered asset) | that run uses Python, with a warning | No wait and no stale green. The footers and warnings show the frequency |
| **No binary, after P7** | the step polls for up to 8 minutes, then marks the event's head `pending` and fails | A delay, never a stale green on the event's pull request. A failed release is red on `master`, and the scheduled release run rebuilds the head |
| **The release pipeline breaks** | until P7, the fallback; after P7, the row above, in every repository | The musl build runs on every pull request (`rust.yml`) |
| **Rate limit exceeded**: primary, or secondary on a full pass | the call fails, that pull request is marked, the job is red: as Python | The switch is budget-neutral: Rust makes Python's calls, and its sequential reads make no more. The reverse check is budget-gated. B2 comes after P7 |
| **The history query times out**: platform's full pass, or **any build scan**, which reads the history of every governed open pull request three times an hour | a full pass marks every selected pull request `error`; a build scan marks nothing and goes red, as Python does today | A known risk that the switch does not change. B1 splits the query; until then, prefer sweeps to `scope=all` on platform |
| **A hung step** | step timeouts: fetch 10 min, event 30 min. Python's steps keep the 360-minute job default, as today | A Rust step no longer holds a pull request's queue for 6 h |
| **No engine runs** (a wrong `if`) | step 4 fails the job | Not silent |
| **Rollback with records Rust wrote** | Python continues | The same bytes and author (§5.5), proven on every run by the reverse check, and once by the drill |
| **A queued run created before a rollback** | runs Rust | A per-pull-request queue holds at most a few runs. The emergency brake exists for incidents |
| **A re-run of an old run** | uses its first attempt's commit and binary, kept at least 30 days after it was superseded | — |
| **The engine changes during the transition** | frozen, or in lockstep (D5) | `pending.txt` catches a Python change Rust lacks |

## 12. Rate-limit budget

- **The limits [R1].**
  - `GITHUB_TOKEN`: 1,000 REST requests an hour per repository, shared by
    every workflow there, plus 1,000 GraphQL points an hour.
  - Secondary limits: 100 concurrent requests; 900 REST points and 2,000
    GraphQL points a minute; 80 content-creating requests a minute and 500
    an hour.
- **Rust in place of Python: no change.** Rust makes Python's calls in
  Python's order, and its sequential reads make no more. `sync --pr N` takes
  about 45–50 calls in both.
- **The reverse check, during S1–S4:** about the same again, but only within
  its reserve rule (§9.3), never on full passes or build scans, and on
  platform only on sweeps.
- **The App.** S0 is one dispatched comparison, about one extra export
  (~900 requests), run after checking the App's remaining budget. The
  differential stays off and is deleted. Draft 1's 24-hour double export is
  dropped: it would have doubled the export's ~2,400 an hour.

## 13. PR plan

| PR | Content | Depends on | Effort (engineer-days) |
|---|---|---|---|
| **P1** Python harvest | Synthetic recordings: the build scan (5), failed writes (5), Markdown with `--user` and "No matching PRs.". Scriptable write answers in the recorder. `conformance/tables/` (event, validate with a tree, codeowners, registry) with its generator and freshness check | — | 2.5 |
| **P2** Engine | `periodic_batch` and the constants in `reconcile`, with `ClockSite::PeriodicBatch`; `Selection::WaitingOnBuild`; the Markdown report; `validate` with a tree; `codeowners`; the registry. The replay accepts `--waiting-on-build`, compares Markdown, serves failed writes, and calls the production batch code | P1 | 3.5 |
| **P3** CLI and writes | `Writes` (pinned allowlist and cap), `write_transport`, the 60 s timeout, `gate`'s read allowances. The `pr-review` binary: `gate` (master only), `event`, `sync`, `report`, `validate`, `codeowners`. Exit codes, footer, panic hook, marking of Rust-only failures. Mock tests of every write route; the no-content test | P2 | 4.5 |
| **P4** Release | `engine-release.yml`: build and publish jobs, `cargo deny`, attestation, a rolling release that never replaces an asset, the scheduled backfill, pruning. The musl job in `rust.yml`. Workflow tests pinning triggers, permissions and the verify flags | P3 | 1.5 |
| **P5** Workflow | The inline list (empty), fetch and verify, the fallback, the Rust path, the assertion step, the `parity` job, timeouts. Workflow tests in Rust. In `pr-hygiene.yml`: `persist-credentials: false` on `data`, and building `pr-review` for S0 | P4 | 2.5 |
| **S0–S4** | S0: one dispatch, then the export switches. S1–S4: one-line list PRs, plus watching | P5 | 0.25 each, plus monitoring |
| **P7** Delete | §10.3 | S4 plus a week | 3 |
| | **Total** | | **about 17.5 engineer-days** (about 3.5 weeks), plus about 4 weeks of rollout that overlap only at the end: **about 7.5 weeks** end to end |

P1 → P2 → P3 → P4 → P5 is a chain. Two pieces can still start early: P1's
harvest, and P3's transport half (`Writes` and the mock tests), which can be
written in parallel. P3's CLI half waits for P2.

## 14. Decisions for the owner

| # | Decision | Recommendation |
|---|---|---|
| D1 | The digest: `pr-review-digest.yml`, `aggregate.py`, `notifications.py`, `slack.json` | **Delete.** Slack is not configured, so nothing is delivered. The service's people endpoints give the same digest; Slack, when wanted, is a service feature |
| D2 | Where the switch lives | **An inline list in the reusable workflow.** It is rolled back by an admin-bypass merge in about 2 minutes. Alternatives: a live file on `master`, which also redirects queued runs, at the cost of parsing and a test; or variables in the governed repositories, which are instant but are settings in other teams' repositories |
| D3 | The first real writes: grovedb, or a sandbox repository first | **grovedb.** It is protected by the fallback, failing closed, the reverse check, and a rollback in about 2 minutes. A sandbox needs a new repository and a second test account, and adds about a day |
| D4 | Soak per stage | **3 / 3 / 5–7 / 7 days.** tenderdash and dash-evo-tool go together |
| D5 | Engine changes during the transition | **Freeze behaviour changes** from P1 to S5. Fixes only, landed in both engines |
| D6 | B1–B3: GraphQL retry and query split, waiting out secondary limits, failing closed on any event-run failure | **After P7**, as the first Rust-only changes. Failing closed on Rust-only failures is in from the start |
| D7 | `site/fixtures/serve.py` | **Keep it.** It is a development fixture, with no CI and no production role |
| D8 | Where binaries live | **One rolling release, `engine`.** Assets are named by commit and never replaced. They are pruned 30 days after their commit is superseded, always keeping at least 20 |
| D9 | Sigstore's trust root after P7 | **Decide before P7, on evidence.** If S1–S4 see any fetch failure caused by the CDN, commit the trusted root, verify with `--custom-trusted-root`, and add a monthly refresh job. Otherwise stay online |
| D10 | Callers pinned to a commit | **Refuse them: accept only `@master`.** All five use `@master`, binaries exist only for `master` commits, and after P7 nothing checks policies against an old pinned engine |

## 15. Backlog after Python (not part of this plan)

- **B1:** retry a GraphQL query on 502 or 504, and split the history query in
  halves when GitHub times out (#123's lesson). This covers the build scan
  too.
- **B2:** secondary limits: `retry-after`, `x-ratelimit-reset`, exponential
  backoff [R1].
- **B3:** fail closed on any failure of an event run.
- **B4:** parallel snapshot reads. Python's four threads are a latency win on
  platform's sweeps. Writes stay sequential.
- **O1:** skip rebuilding when no build input changed. It must build or
  reproduce the binary: re-attesting commit X's binary as built from Y would
  be false provenance.
- **O2:** an `aarch64` binary, if a governed repository moves to an ARM
  runner.

## 16. Research sources

- **[R1]** GitHub Docs, [Rate limits for the REST API](https://docs.github.com/en/rest/using-the-rest-api/rate-limits-for-the-rest-api):
  - `GITHUB_TOKEN` gets 1,000 requests an hour per repository, or 15,000 on
    Enterprise Cloud.
  - Secondary limits: 100 concurrent requests; 900 REST or 2,000 GraphQL
    points a minute; 90 s of CPU per 60 s; 80 content-generating requests a
    minute and 500 an hour.
  - The `x-ratelimit-*` headers take priority over `GET /rate_limit`.
  - On a secondary limit: honour `retry-after` or `x-ratelimit-reset`,
    otherwise wait at least a minute, then back off exponentially.
- **[R2]** GitHub Docs, [Rate limits and query limits for the GraphQL API](https://docs.github.com/en/graphql/overview/rate-limits-and-query-limits-for-the-graphql-api):
  - `GITHUB_TOKEN` gets 1,000 points an hour.
  - A request that runs over 10 s is stopped and answered with a 502 or 504,
    and extra points are deducted for the next hour. This is the timeout
    behind #123.
- **[R3]** GitHub Docs, [Dependency caching reference](https://docs.github.com/en/actions/reference/workflows-and-actions/dependency-caching):
  - A run restores caches from the current, default or base branch.
  - `pull_request_target`, `issue_comment` and `workflow_run` get read-only
    access. Only `push`, `schedule`, `workflow_dispatch` and a few others may
    write the default branch's caches.
  - `cache-mode` propagates into reusable workflows.
  - The limit is 10 GB per repository, with eviction after 7 days unused.
  - Cache contents are not signed.
- **[R4]** GitHub Docs, [Reusing workflow configurations](https://docs.github.com/en/actions/reference/workflows-and-actions/reusing-workflow-configurations):
  - The `github` context is the caller's.
  - Permissions can only be kept or reduced.
  - Runners and billing are the caller's.
  - A re-run uses its first attempt's commit.

  For `job.workflow_sha` and `job.workflow_ref`, see the
  [contexts reference](https://docs.github.com/en/actions/reference/workflows-and-actions/contexts#job-context).
- **[R5]** GitHub Docs, [Artifact attestations](https://docs.github.com/en/actions/concepts/security/artifact-attestations)
  and [Using artifact attestations](https://docs.github.com/en/actions/how-tos/secure-your-work/use-artifact-attestations/use-artifact-attestations):
  - On the Free, Pro and Team plans, attestations are available for public
    repositories.
  - Public repositories use Sigstore's public-good instance and its
    transparency log.
  - An attestation binds the workflow, the repository, the commit SHA and
    the triggering event.
  - Attesting needs `id-token: write`, `attestations: write` and
    `contents: read`.
- **[R6]** GitHub CLI manual, [`gh attestation verify`](https://cli.github.com/manual/gh_attestation_verify):
  - flags: `--bundle`, `--cert-identity`, `--source-digest`,
    `--signer-digest`, `--source-ref`, `--deny-self-hosted-runners`,
    `--format json`;
  - the signer workflow is not checked unless asked;
  - with `--bundle`, the repository is checked against the certificate's
    names.

  See also [GHSA-fgw4-v983-mgp8 / CVE-2025-25204](https://github.com/cli/cli/security/advisories/GHSA-fgw4-v983-mgp8):
  a mismatched predicate type was accepted before v2.67.0.
- **[R7]** [astral-sh/setup-uv](https://github.com/astral-sh/setup-uv)
  downloads uv's release binaries and checks their SHA-256 against checksums
  shipped in the action, or a `checksum` input.
- **[R8]** [taiki-e/install-action](https://github.com/taiki-e/install-action)
  installs prebuilt binaries from per-tool manifests with SHA-256, and falls
  back to `cargo-binstall`.
- **[R9]** [cargo-dist: GitHub attestations](https://axodotdev.github.io/cargo-dist/book/supplychain-security/attestations/github.html):
  `github-attestations = true` attests release archives, which
  `gh attestation verify` checks.
- **[R10]** [EmbarkStudios/cargo-deny-action](https://github.com/EmbarkStudios/cargo-deny-action)
  ships a Rust tool as a container action. This plan uses `cargo deny`
  itself.
- **[R11]** GitHub Docs, [Rate limit endpoint](https://docs.github.com/en/rest/rate-limit/rate-limit):
  "Accessing this endpoint does not count against your REST API rate limit."
  It returns `core` and `graphql`.

Most comparable tools pin a released version by checksum in the consumer (R7,
R8), which suits a versioned tool. Here the consumer is "the reusable
workflow at commit X", and it needs the binary of commit X. Provenance keyed
by the source digest (R5, R6, R9) fits that.

## Appendix A. Review findings folded into draft 2

Five independent reviewers each had the spec and the code, not the author's
conclusions.

**Feasibility against the code.**

- **The draft-1 shadow.** It would stop at the first refused write. The
  differential instead answers would-be writes and continues
  (`live.rs:546-565`), with excuses for nudges and the reviewer cap.
  → The forward shadow is dropped (Appendix B), and the reverse check uses
  Python's own recorder.
- **The 24-hour double export.** It doubles the App budget, and two exports
  always differ in `generated_at`. → S0 is one comparison, with
  `generated_at` removed.
- **Reads `ReadOnly` refuses.** It refuses `GET repos/{r}`, `rate_limit` and
  reads of the central repository. → `gate` gets explicit read allowances,
  and `rate_limit` is read with `gh` in the `parity` job.
- **Extracting the transport.** It is bigger than draft 1 said. → The binary
  lives in `service` (the simplicity reviewer agreed).
- **The function harvest.** It cannot record functions that read the
  environment or the filesystem. → P1 adds a table generator with its own
  freshness check.
- **What Rust quotes.** Rust's transport quotes GitHub's `message` field, so
  "stricter than Python" was false. → §4.4 lists what may be quoted, and the
  no-content test places markers in body fields and in wrong-typed values.
- **The build scan.** It reads the history of every governed open pull
  request. → Added to §11 and to B1.
- **Tests P7 would orphan.** `engine/tests/differential` and
  `engine/tests/reconcile/live.rs`, and the context-test job. → Added to
  §10.3.
- **`periodic_batch`.** Its cadence, its clock site, and a batch size of 0.
  → P2.
- **`--policies-root` has no default** in a static binary. → It is required.
- **The footer breaks JSON on stdout.** → Only `event` prints it there.
- **Python's `event` path prints a traceback.** → Noted as an intended
  difference.
- **The evaluate corpus has 306 cases, not 269.** → Corrected.
- **`--bundle` making no API call is unconfirmed.** → P5 tests it.

**Parity of switching writers.**

- **What "zero would-be writes after Python" proves.** It does not imply
  equal bytes: Python's checks are loose, and the first refusal ends the
  run. → The forward shadow is dropped, and the reverse check is described
  for what it proves: acceptance and continuity, not bytes.
- **The explained differences were incomplete.** They miss evidence that
  changes without moving `updated_at`, and failed writes Python swallows.
  → §9.3 lists them.
- **No recording holds a failed write or a real write answer.** → P1 adds
  five failed writes, P2's replay serves them, and §5.5 is narrowed to writes
  that succeed.
- **The per-request timeout.** The transport gives up after 30 s; Python
  waits 60 s. → The CLI uses 60 s.
- **S1's exit criteria could not be measured from counts.** → The footer
  lists pull request numbers.
- **The rollback drill is weak, and carried fields survive a rollback.**
  → The reverse check runs on every run, and §8.3 has a runbook for carried
  fields.
- **Constants the caller passes in were not checked by the corpus.** → They
  move into `reconcile`, and the replay calls the production code.

**Security and supply chain.**

- **`gh` before 2.67.0 accepts a mismatched predicate type**
  (CVE-2025-25204). → A version floor, and a check of the JSON result.
- **The workflow SHA was not validated before use,** and
  `--source-digest ""` disables the check. → A regex on the SHA, passed
  through `env`, plus `--signer-digest`.
- **Dropping `compare` rests on the verify flags and the release triggers.**
  → Workflow tests pin both, and `workflow_call` is forbidden. A test covers
  a default branch containing a newline.
- **`--clobber` is not atomic.** → Assets are never replaced, the release
  workflow has a concurrency group, the bundle is uploaded first, and pruning
  goes by when a commit was superseded.
- **A `contents: write` token persists in `pr-hygiene.yml`'s `data`
  checkout** while `cargo build` runs. → `persist-credentials: false`, in P5.
- **The `Writes` allowlist was not pinned.** → Contexts, path segments, label
  names and bodies are pinned, and reads go through `ReadOnly`'s check.
- **The compromised-dependency risk was understated.** → `cargo deny`, and
  the risk is stated as new.
- **Persistent state on self-hosted runners.** → Fresh `gh` directories,
  logging, and a hash check before each use.
- **No-content gaps.** Values of the wrong type, and `RUST_BACKTRACE` in the
  panic hook. → §4.4.
- **O1 would attest false provenance.** → Reworded.

**Failure modes and rollback.**

- **The 8-minute wait without a binary** leaves a stale green, and queued
  runs pile up behind it. A skipped push run never builds. → Until P7, the
  run falls back to Python. After P7: a bounded wait, a pending mark on the
  event's head, and a scheduled backfill build. The bundle is uploaded first.
- **Rust-only failures** (`Refused`, `NotPorted`, `Exception`, a panic) skip
  Python's mark, and the nudge comes before the pending status. → They are
  marked at the switch, and §5.4 is corrected.
- **The shadow's wait, and steps with no timeout,** hold a pull request's
  queue. → No shadow, and step timeouts.
- **A wrong `if` could skip every engine silently.** → The assertion step,
  and a test of every engine against every event.
- **An absolute budget floor can turn platform red.** → A reserve
  proportional to the time left in the hour, never on full passes or build
  scans, and on platform only on sweeps.
- **Rollback does not repair carried fields.** → The runbook, pull request
  numbers in the footer, and an emergency rollback that removes the line.
- **Nothing writes for real before S1.** → Owner decision D3, with a
  recommendation.
- **Pruning by age.** → Pruning by when a commit was superseded, keeping at
  least 20.
- **Sigstore's CDN is a runtime dependency.** → The fallback until P7, then
  D9.
- **Nits:** a 15 s poll with jitter. A refusal by the cap aborts a sweep, as
  stated.

**Simplicity and scope.**

- **Cut the 24-hour export comparison.** → Done.
- **Accept only `@master` in the gate,** and drop `check-pins` and the loop
  over pinned engines. → Done (D10).
- **Cut the forward shadow.** → Done. It is replaced by the cheaper reverse
  check, which the other reviewers' findings pointed to.
- **Replace `engines.txt` with a list in the workflow.** → Done.
- **Put the binary in `service`** rather than extracting a crate. → Done.
- **Defer the update tool and its directories.** → The tool is deferred; its
  rules are fixed now (§10.1).
- **No `xtask`:** a `yq` bot check and a committed caller template. → Done.
- **Port only the flags the workflows use.** → Done.
- **A smaller footer,** without the per-resource rate tally. → Done.
- **Build `pr-review` in the export job.** → Done.
- **Check `runner.os`.** → Done.
- **Narrow the table mapping Python tests.** → Done.

## Appendix B. Review findings not taken

- **A forward live shadow before switching** (draft 1's S0), and its
  repaired form: a transport that answers would-be writes, plus byte checks
  against stored bodies.
  - *Not taken.* Three reviewers showed its signal is weak:
    - Python's checks are loose, and selections shift after Python acts;
    - many differences must be explained by hand;
    - it never exercises the write half;
    - it doubles the budget before delivering any value.

    Repairing it would keep `live.rs`'s observing transport and excuse rules
    alive in production code. The reverse check after a switch needs no new
    Rust code and gives a direct signal (continuity and acceptance), so it
    was chosen instead.
- **Replaying Python's real production run in Rust, in the same job**
  (parity reviewer: a pass-through recorder that also records clock values).
  - *Not taken.* It would change the production writer during the very
    window when the writer must stay still:
    - the recorder would stand at the GitHub boundary of live `--apply` runs;
    - snapshots would run one at a time;
    - the clock would be read differently;
    - private permission data would land on disk in five repositories' jobs.

    What it adds, byte-exact parity on live inputs, is what the corpus
    already proves for recorded inputs. The remaining risks are in the glue,
    which this replay does not exercise.
- **Dropping the per-issue comment cap** (simplicity reviewer: "a failure
  mode Python doesn't have").
  - *Not taken.* A correct engine never trips it: one record or move comment,
    plus one nudge. Without it, a loop bug would keep notifying reviewers on
    public pull requests, which is the hardest harm to undo. It costs about 20
    lines, and a refusal fails closed (§5.4).
- **Committing Sigstore's trusted root now** (failure-modes reviewer).
  - *Deferred to D9.* Until P7, a fetch failure falls back to Python, so a
    CDN outage costs nothing. A committed root fails closed when it rotates,
    and needs a refresh job. It is decided before P7, on evidence from S1–S4.
- **A sandbox repository before grovedb** (failure-modes reviewer).
  - *Left to the owner (D3), recommended against.* It needs a new repository
    and a second account for approvals. grovedb's exposure is small and fails
    closed.
- **A `.failed` marker so that waiting runs stop early** (failure-modes
  reviewer).
  - *Not taken.* Until P7 nothing waits. After P7, the scheduled backfill and
    the 8-minute bound suffice.
- **Running the shadow as a separate read-only job** (security reviewer).
  - *Moot:* there is no forward shadow. The reverse check does run as a
    separate read-only job.
- **The `github/` crate extraction** (draft 1).
  - *Withdrawn.* See §4.1.
