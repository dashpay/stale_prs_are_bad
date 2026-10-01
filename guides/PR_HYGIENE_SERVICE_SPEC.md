# Spec: PR Hygiene as one service with an API

Status: draft revision 2, 2026-10-02 — after simplicity, reliability/migration
and security reviews. Replaces the delivery parts of `DASHBOARD_V2_SPEC.md`
(static site, `data` branch); its product parts — stages, "your part",
humans vs bots, lateness — carry over unchanged.

## 1. Problem

PR Hygiene is four programs that each rebuild the same picture of every PR
and keep what they learn in places that are not a database.

| Piece | Runs where | Keeps state in |
|---|---|---|
| Engine (Python, `pr_review/`) | Actions in each governed repo (per event, hourly sweep, 3×/h build scan), callers pinned to an engine commit | record comments (JSON, edited in place), commit statuses, labels, PR checklist — all keyed on the author `github-actions[bot]` |
| Dashboard (Rust, `src/`) | Actions here, every 6 h | `data` branch, GitHub Pages |
| Slack digest | Actions here, daily | recomputes everything |
| `/prs` skill | developer's machine: clone + Python ≥ 3.10, ~75 s | recomputes everything with the developer's token |

Costs seen this week: fixes reach a repo only after a re-pin PR, and a live
policy can mean different things to differently pinned engines; a PR leaving
the policy loses its record comment and with it its admission order (58
platform PRs); stage history has to be scraped from comment revisions
(PR #68); dashboard data is bot commits; four surfaces give four answers;
the same rules exist in Python and Rust; a static page cannot know who is
looking. And the `PR Hygiene` required check can be posted by anything
running as GitHub Actions in the repo — or by anyone, on the two repos that
accept any source.

## 2. Target

One Python service, installed as a **new, private GitHub App** on the five
repositories, owning the loop: GitHub events → the unchanged engine → GitHub
writes; every evaluation recorded in **SQLite**; one **read API** behind the
web UI, the `/prs` skill, the Slack digest and any other tool.

```
            ┌──────────── front (no App key) ────────────┐
GitHub ───▶ │ webhook receiver ─▶ durable queue (SQLite) │ ◀── read API ◀── web UI / skill /
 events     │                     │                      │                  Slack / Actions
            └─────────────────────┼──────────────────────┘
                                  ▼
            ┌──────────── worker (holds App key) ────────┐
            │ per (repo, author): re-read PR, engine     │ ──▶ GitHub writes (status, labels,
            │ evaluate, write, append evaluation row     │      checklist, move comment)
            │ sweep · startup reconcile · delivery replay│
            └────────────────────────────────────────────┘
```

- **Engine unchanged.** `policy.evaluate()` is already a function of
  evidence. v1 keeps the engine's memory where it is — record comments and
  commit statuses — so the service changes *where* the engine runs, not what
  it decides. Moving the memory into the DB is an optional v2 (≈450 lines of
  `main.py`/`github.py`; `policy.py` reads it through four keys, so an
  adapter goes there).
- **Store.** Every evaluation is appended (`repo, pr, head, state,
  reviewers, approvals, objectors, blockers, evaluated_at, engine_commit`);
  stage transitions, timelines, speed and admission order are queries. One
  SQLite file on a volume, daily backup. Losing it loses history, never a
  verdict: the engine's memory stays on GitHub in v1.
- **Policies stay in git** (this repo), reloaded from the commit the API
  reports for the default branch (not the push payload), with today's
  ruleset check.
- **Concurrency.** The receiver only verifies the signature (HMAC-SHA256,
  constant time), drops repeated `X-GitHub-Delivery` ids, enqueues
  `(repo, pr)` and answers within GitHub's 10 s. One worker drains the
  queue, serialised per (repo, author) — admission is per-author state — and
  re-reads the PR before acting; a payload is a hint, never evidence.
- **Not losing events.** GitHub does not redeliver failures. The worker
  replays failed deliveries from the App's delivery log, runs a full
  reconcile on start-up, and sweeps in rotating batches within the
  installation's rate limit (a full pass is ~1 500 calls). `check_suite`,
  `check_run` and `status` events replace the build scan, via a head-SHA →
  PR index (fork PRs arrive without a PR list). Events caused by the App's
  own writes are filtered by sender.
- **API** (`/api/v1`, JSON): `GET /prs` (filters: repo, area, stage, author,
  reviewer, late), `GET /prs/{owner}/{repo}/{n}` (with stage history),
  `GET /people`, `GET /people/{login}` (what one owes and holds — never
  speed), `GET /me` (the caller's queue **and speed**; `?format=text` is the
  Slack/skill digest).
- **Identity** — a separate **sign-in App with no repository permissions**:
  - web: GitHub sign-in with `state` bound to a pre-login cookie and PKCE;
    session in a `__Host-` cookie (Secure, HttpOnly, SameSite=Lax) on the
    service's own subdomain; the GitHub token is used once for `/user` and
    discarded. If infra already runs an SSO proxy limited to the org, use it
    instead and write no sign-in code.
  - skill and scripts: device flow on the sign-in App; the service checks
    the token was issued to its App and returns its own short-lived token.
    The developer's `gh auth token` (usually full `repo` scope) is never
    accepted.
  - Actions: GitHub OIDC with the service as audience, checked against
    `repository_id` and `job_workflow_ref`.
- **Privacy.** Speed is served only by `/me`. No per-person speed elsewhere;
  no team or area aggregate under 5 people. "Only to you" governs display:
  review timestamps are public on GitHub. Retention for metrics, backups and
  logs is fixed up front; `Authorization` is never logged. Team/people
  views: public or org-only (owner).
- **Web UI.** The page from #67 (Team, People, Me) served from the service's
  origin, reading the API; CSP sent as a header with `frame-ancestors
  'none'`, Trusted Types and `site/check.sh` kept.
- **Language.** Python only. From the Rust analyzer the stage mapping,
  lateness, the people/"your part" contract and unresolved-thread counts are
  ported; the thread-severity score and scoreboard are dropped.

## 3. Security of the merge path

- The App's `PR Hygiene` status becomes the required source on **all five**
  repos — today it can be posted by any Actions workflow (or by anyone, on
  platform and rust-dashcore). After cut-over the engine trusts only the
  App's own `<slug>[bot]` identity.
- The key lives only in the worker, ideally as a non-exportable cloud KMS
  key; the internet-facing front holds none. Key-rotation runbook.
- `pull_requests: write` lets an App approve. Code-owner review stays
  required and the App stays off every bypass list. Act only on events from
  known installation and repository ids.
- Deploy: an image built from protected `master` only, through an
  environment with required reviewers; dependencies hash-pinned, image by
  digest, provenance attested; the running commit shown in the status
  description. A per-repo write switch limits a bad deploy's blast radius.
  Governed repos no longer review engine upgrades — the trust root is this
  repo's protected `master`, as it already is for policies.

## 4. Requirements for infra

One always-on container running two processes (front, worker); a persistent
volume for SQLite with daily backup; HTTPS on a dash.org subdomain (inbound
from GitHub webhooks); outbound HTTPS to GitHub and Slack; secrets: App
key (KMS if available), webhook secret, sign-in App client secret, session
key, Slack token; restart on crash, alerting on delivery failures and queue
age; a deploy that runs on merge to `master` (no ticket per release). The
`PR Hygiene` status is merge-critical: while the worker is down PRs keep
their last status and cannot turn green.

## 5. Roadmap — each step useful on its own, each reversible

0. **Now, no infra.**
   - Callers track `pr-review-reusable.yml@master` (small change to the
     reusable workflow's pin check + five caller lines): ends re-pin PRs and
     version skew; the per-repo `PR_REVIEW_AUTOMATION_ENABLED` switch stays
     as the canary.
   - Merge #65 (target-branch patterns; 58 ungoverned platform PRs).
   - Engine: keep the record comment when a PR leaves the policy (stops the
     lost-admission bug).
   - Engine: accept both `github-actions[bot]` and a configured App identity
     everywhere it reads its own memory (12 places, statuses included), so
     either writer can continue the other's state.
1. **Service, shadow.** Receiver + worker running the engine read-only on
   GitHub; store; API; UI; `/prs` skill and Slack digest switch to the API.
   Backfill history once from record comments and GitHub events (#68's
   logic). Compare per (PR, head) when Actions posts its status, same engine
   commit on both sides, differences sorted into categories defined up
   front (clock-dependent timeouts, telemetry, admission time).
2. **Cut over writes, one repo at a time** (grovedb → dash-evo-tool →
   tenderdash → rust-dashcore → platform): turn Actions writes off with the
   repo variable (instant, every branch — deleting the caller would leave
   `v*-dev` copies running); service writes and reconciles every open PR;
   once the App has posted the check, make it the required source. The
   service keeps writing the record marker in the same format, so
   **rollback** is the variable back on — no PR through a broken check.
3. **Retire** the callers, the reusable workflow, the Rust analyzer, the
   `data` branch and Pages; drop trust in `github-actions[bot]`.
4. **Optional v2:** engine memory in the DB, behind an adapter.

Fallback if infra will not host anything merge-critical: stop after step 1
(engine stays in Actions on `@master`; the service is a read-only API).

## 6. Open PRs

- #65: merge (step 0).
- #66: ship its three live-bug fixes alone (bot accounts named `[bot]`, bots
  not merged into people, retry on a truncated response); close the rest.
- #67, #68: pause; the page, the product rules and the backfill logic move
  into the service.
- platform#5242 (`/prs` skill wording): fine now; the skill moves to the API
  in step 1.

## 7. Decisions for the owner

1. This roadmap, starting with step 0 now?
2. Team/People views public, or dashpay org members only?
3. Hosting (infra team): §4.
