# Spec: PR Hygiene as one service with an API

Status: draft revision 8, 2026-10-02 — after simplicity, reliability/migration
and security reviews; the owner's decisions (**Rust**, a **public API**,
**GitHub sign-in open to anyone** that personalises the page); then a
Rust-port review and a public-API/privacy review. Replaces the delivery parts of `DASHBOARD_V2_SPEC.md`
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

One **Rust** service, installed as a **new, private GitHub App** on the five
repositories, owning the loop: GitHub events → the engine → GitHub writes;
every evaluation recorded in **SQLite**; one **public read API** behind the
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

- **Engine ported to Rust, verdict for verdict.** Rust is the team's
  language, and the dashboard's Rust (stage derivation, the data contract,
  history backfill, registry loading) becomes part of the service instead of
  being retired. The engine (`policy.py` evaluate, `main.py` orchestration,
  `github.py` evidence and writes: ≈3 000 lines, ≈410 tests) is the risk,
  handled by a **conformance corpus**:
  - **Recorded at the engine's one GitHub boundary** (`GitHub._run`): every
    request with its response, the clock injected everywhere it is read,
    writes captured instead of sent. The Rust engine replays the reads
    (GraphQL queries ported verbatim) and must produce the same verdict and
    the same **ordered list of writes**; replaying the post-write state must
    produce none. Live PRs and every Python test fixture are recorded.
    Failure paths (label 404, reviewer 422, GET 502 retried once, POST never
    retried) are HTTP-mock tests.
  - **What must match exactly:** everything one engine writes and the other
    reads back — markers, the record, `diff_print`, `receipt_print`, the
    checklist, move text, labels, status state and description. Not the
    `evidence`/`context` hashes (only compared within one run) and not
    blockers carrying a Python exception's text.
  - **Python behaviours reproduced in one `compat` module:** insertion-ordered
    maps (IndexMap only), first-wins ties of `max(key=)`, `splitlines`/
    `strip`/`\s` character sets, lengths in code points, a canonical JSON
    writer matching `json.dumps` (sorted keys, ASCII-escaped). The exporter's
    Python version is pinned. The full list is in §6c.
  - **Shadow** compares in two ways: Rust's engine on Python's recorded
    evidence (exact), and Rust's live evidence reader against Python's
    recorded evidence. Both run in a scheduled differential job, and the
    service's read-only loop is compared with GitHub's state (§6c, 1b-5 and
    1b-6).
  - Until a repo is cut over its Python engine keeps running; engine rule
    changes during the port land in both or wait.
- **Memory unchanged in v1.** The Rust engine reads and writes the same
  record comments and commit statuses, in the same format, so either engine
  can continue the other's state and rollback is a switch. Moving the
  memory into the DB is an optional v2.
- **Store.** Every evaluation is appended (`repo, pr, head, state,
  reviewers, approvals, objectors, blockers, evaluated_at, engine_commit`);
  stage transitions, timelines, speed and admission order are queries. One
  SQLite file on a **local** volume (WAL, `busy_timeout`, writes in
  `BEGIN IMMEDIATE`), daily backup. Losing it loses history, never a
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
  PR index (fork PRs arrive without a PR list); bursts for one PR coalesce
  in the queue. Events caused by the App's own writes are filtered by
  sender. Installation tokens are refreshed before their hour expires;
  per-PR caches live for one evaluation, never the process. One installation
  budget (5 000/h) replaces five job tokens, and GitHub's secondary limits
  (≈80 writes/min) pace any mass reconcile. Per-run limits such as
  `NUDGES_PER_RUN` become per-time-window limits.
- **API** (`/api/v1`, JSON, **public, no sign-in**): `GET /prs` (filters:
  repo, area, stage, author, reviewer, late), `GET /prs/{owner}/{repo}/{n}`
  (with stage history), `GET /people`, `GET /people/{login}` (what one owes
  — with their areas — and holds; `?format=text` is the digest the skill and
  Slack show; `GET /prs/{owner}/{repo}/{n}?format=text` for one PR),
  `GET /metrics` (team speed by repo and area only, never by person, no group
  under 5). Speed per person is the one signed-in resource: `GET /me/speed`.
  - **Public reads never reach GitHub or the worker.** They answer from what
    the store already holds; an unknown login or PR is empty or 404, never a
    lookup; there is no refresh. The front opens SQLite read-only with
    statement timeouts, rejects unknown query parameters, caches with ETags
    and rate-limits per client.
  - **Only what the engine already posts on PRs** (status, checklist, move
    comment, record) and GitHub's own public data is served. Collaborator
    **permission levels**, part of the engine's evidence, are never served,
    and corpus recordings containing them are redacted before they enter this
    public repository.
  - People are keyed by **GitHub user id**, not login (logins get renamed and
    re-registered).
  - Public responses: `Access-Control-Allow-Origin: *`, no cookies,
    cacheable. `/me/*` and `/auth/*`: same-origin, `no-store`, never cached
    by a CDN.
  - During shadow the API serves the verdict that gates the merge (Python's,
    as posted) and marks each repository `shadow` or `enforced`, so the skill
    never contradicts the PR.
- **Identity — only to personalise, open to anyone with a GitHub account.**
  A separate **sign-in App with no repository permissions**. The web page
  offers "Sign in with GitHub": `state` bound to a pre-login cookie, PKCE,
  the GitHub token used once for `/user` and discarded, a session in a
  `__Host-` cookie (Secure, HttpOnly, SameSite=Lax) on the service's own
  subdomain, holding the user id and login only; logout is a POST with an
  Origin check and deletes the session server-side. The sign-in App asks
  for no permissions and no email. Signing in only tells the page who you
  are; it unlocks nothing but your own speed. Scripts and Actions need no identity: the API they
  read is public. The developer's `gh auth token` is never sent anywhere.
- **Privacy (GDPR applies: speed profiles work performance, even when
  derived from public data).** Speed per person is shown only to its owner,
  but it is **not secret**: stage history and public reviews let anyone
  rebuild it. The UI says so: "Only shown to you here. Built from public
  GitHub activity, so not secret; we don't publish or rank it." A privacy
  page states the source (GitHub, not the person), retention (13 months for
  person-linked evaluation rows, 30 days for sessions, short logs without
  full IPs, backups alike), an **opt-out** (hidden from People, no speed
  computed), and that revoking the sign-in App (its `github_app_authorization`
  webhook) or "delete my account" removes all sessions; speed is computed on
  request, not stored. Cookies and tokens are never logged.
- **No rankings.** `/people` and the People page sort by name, show what is
  owed and held but no per-person lateness or "oldest owed", and list only
  people the policies name or who hold governed PRs; outside contributors'
  stages are never coloured late.
- **Web UI.** The page from #67 served from the service's origin, reading
  the API. **Team** and **People** are public. **Me** replaces the "Who are
  you?" picker with "Sign in with GitHub" (saying it asks only for your
  public profile): signed in, it is your People page plus your speed
  (monthly, a year back), with empty states for people with no areas or
  PRs; signed out, a login search goes to `/people/<login>` and stores
  nothing. Slack links to `/people/<login>`. Old `#/…` Pages links are
  redirected client-side. CSP sent as a header with `frame-ancestors 'none'`; Trusted
  Types and `site/check.sh` kept.
- **`/prs` skill.** Reads `GET /people/{login}?format=text` with the login
  from `gh api user` (JSON for `--format json`, the PR text endpoint for
  one PR): no clone, no Python, no token, the same answer as the page and
  Slack. The versioned text carries generation time, each repository's
  freshness and mode (`shadow`/`enforced`), partial or unknown repositories
  named; reviews owed oldest first with URL, author, your areas and who
  shares them, re-review asks and whether the wait is recorded; your PRs
  with stage, whose move (author / bots / access / queue), next action,
  queue position and slots used per repository, with the combined-over-five
  warning. Titles are stripped of control characters. When the service is
  down the skill says so. SKILL.md's "nothing about CI" rule is rewritten.
- **Language and layout.** Rust only, one workspace:
  - `engine`: synchronous, with no runtime. It holds `pycompat`, `policy`,
    `evidence` (the port of `github.py` behind a `Transport`, with replay and
    a stateful fake) and `reconcile`.
  - `service`: front, worker and scheduler, plus the HTTP transport, App
    authentication and the store.

  §6c has the detail. Reused from `src/`: `dashboard.rs`, `stages.rs` (backfill),
  registry loading. Retired (≈3.8k lines): `scorer`, `renderer`,
  `analyzer`, `history`, `config`, and the Python engine. The dashboard's
  GraphQL fetcher does not supply engine evidence (`diff_print` needs REST
  file `sha` and `patch`). Crates: axum + tower, reqwest, rusqlite,
  hmac/sha2 (`verify_slice`), jsonwebtoken (or KMS signing), indexmap,
  httpmock. CI's `validate`, `codeowners` and `rollout` commands are ported
  or dropped.

## 3. Security of the merge path

- The App's `PR Hygiene` status becomes the required source on **all five**
  repos — today it can be posted by any Actions workflow (or by anyone, on
  platform and rust-dashcore). The engine's memory can be forged today. Any
  workflow posts as `github-actions[bot]`, and any collaborator with write
  access can edit the engine's comments.
  - Before cut-over, records edited by anyone but the engine are ignored.
  - After a repository's cut-over instant, a `github-actions[bot]` status or
    record counts only if it predates that instant.
  - The App is recognised by its numeric bot id as well as its login.

  §6c, "Trust model", has the detail.
- The key lives only in the worker, ideally as a non-exportable cloud KMS
  key; the internet-facing front holds none. Key-rotation runbook.
- `pull_requests: write` lets an App approve. Code-owner review stays
  required and the App stays off every bypass list. Act only on events from
  known installation and repository ids.
- Deploy: an image built from protected `master` only, through an
  environment with required reviewers; dependencies hash-pinned, image by
  digest, provenance attested; the running commit in the status's
  `target_url` — never its description, which the engine reads back as
  memory. A per-repo write switch limits a bad deploy's blast radius.
  Governed repos no longer review engine upgrades — the trust root is this
  repo's protected `master`, as it already is for policies.

## 4. Requirements for infra

One always-on container running two processes (front, worker); a persistent
**local** volume for SQLite (not NFS) with daily backup; HTTPS on a dash.org subdomain (inbound
from GitHub webhooks); outbound HTTPS to GitHub and Slack; secrets: App
key (KMS if available), webhook secret, sign-in App client secret, session
key, Slack token; restart on crash, alerting on delivery failures and queue
age; a deploy that runs on merge to `master` (no ticket per release). The
`PR Hygiene` status is merge-critical: while the worker is down PRs keep
their last status and cannot turn green.

## 5. Effort (one senior Rust engineer; review estimate)

| Work | Engineer-weeks |
|---|---|
| Python exporter (boundary recording, clock injection, redaction) | 1 |
| Engine port to corpus-green | 4–5 |
| Evidence reader, client, App auth | 2–3 |
| Reconcile (write path) | 2–3 |
| Service: queue, worker, API, sign-in, UI move | 3–4 |
| Shadow (3–4 calendar weeks) | 1–2 |
| Cut-over (≈5 calendar weeks, repo by repo) | 1–2 |
| **Total** | **≈15–20** |

Corpus-green alone is ≈7–9 weeks. Step 0 and a read-only API on the Python
engine's posted verdicts deliver value long before.

## 6. Roadmap — each step useful on its own, each reversible

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
1. **Port + shadow.** The Python engine gains a corpus export (evidence +
   verdict per PR, fixed clock); the Rust engine is ported until the corpus
   and the ported tests pass. Receiver + worker run it read-only on GitHub;
   store; public API; UI with sign-in; `/prs` skill and Slack digest switch
   to the API. Backfill history once from record comments and GitHub events
   (#68's logic). Live diff per (PR, head) against the Python engine's
   status, differences sorted into categories defined up front
   (clock-dependent timeouts, telemetry, admission time); any other
   difference is a bug in the port.
2. **Cut over writes, one repo at a time** (grovedb → dash-evo-tool →
   tenderdash → rust-dashcore → platform): turn Actions writes off with the
   repo variable (instant, every branch — deleting the caller would leave
   `v*-dev` copies running); service writes and reconciles every open PR;
   once the App has posted the check, make it the required source. The
   service keeps writing the record marker in the same format, so
   **rollback** is the variable back on — no PR through a broken check.
3. **Retire** the callers, the reusable workflow, the Python engine, the
   Markdown board, the `data` branch and Pages; drop trust in
   `github-actions[bot]`.
4. **Optional v2:** engine memory in the DB, behind an adapter.

Fallback if infra will not host anything merge-critical: stop after step 1
(the Python engine stays in Actions on `@master`; the Rust service is the
public API, the page and the history, read-only on GitHub).

## 6a. Step 1a in detail — the read-only service

Goal: the public API, the site, the skill and Slack on one service before
the engine port, with **no GitHub write credential in the service at all**.
(Revised after a security/design review.)

**Data in — push, from its own job.** The scheduled workflow in this
repository (every 15 minutes: one full engine export measured on
2026-10-02 at ≈590 REST calls, 6 GraphQL points and 139 s for five
repositories and 136 PRs — ≈2 400 calls an hour against the read-only App's
5 000; the governed repositories' own engine runs use their own tokens)
keeps its analyze job as it is — engine export, Rust
analyzer, `dashboard.json` handed on as an artifact. The **post step is its
own reusable workflow** (`.github/workflows/pr-hygiene-post.yml`): no
checkout, no build, no third-party code beyond download-artifact; it checks
the downloaded file against the SHA-256 the analyze job recorded (any later
step could replace the artifact), mints the OIDC token (audience = the
service URL), masks it and POSTs once. Skipped on dry runs; on
quarter-hourly runs a failed post is a warning and a partial run stays
green (the service marks the failed repository stale), while full runs
still fail.

**Ingest checks, before the body is read:**
- JWT: RS256 only; exact `iss` and `aud`; `nbf`/`exp` enforced with ≤60 s
  skew and a cap on `iat` age; keys from GitHub's fixed JWKS URL, cached,
  refreshed at most once a minute on an unknown `kid`, last good keys kept,
  failing closed.
- Claims: `repository_id` and `repository_owner_id` (numeric — names can be
  re-registered), `ref` = `refs/heads/master` with `ref_type` = `branch`,
  `workflow_ref` = this repository's `pr-hygiene.yml@refs/heads/master`
  and `job_workflow_ref` = `pr-hygiene-post.yml@refs/heads/master` — the
  second separates the post job from the caller's other jobs (the Pages
  deploy among them) that also hold `id-token: write`, `event_name` ∈ {`schedule`,
  `workflow_dispatch`}, `runner_environment` = `github-hosted`,
  `run_attempt` = 1 (a re-run executes an old commit).
- Body: an explicit size limit (set from a measured snapshot); the
  snapshot's `commit` must equal the token's `sha` and its `generated_at`
  fall between the token's `iat` minus the job timeout and `iat` plus skew
  — so a replay is harmless and no future-dated snapshot can freeze ingest;
  schema version, string lengths and login formats bounded.

**Partial runs.** A snapshot covers every repository; a repository with a
fetch error or no engine export keeps its **last good data**, marked stale,
and `people` is recomputed over the result. A failed repository records no
observations.

**Store (SQLite).** `snapshots` (latest good per repository; raw JSON for
30 days), `stage_changes` (PR, stage, engine state, since — one row only
when something changes; exact entry times when `since_basis` is `engine`),
`sessions` later. Recording starts with the first ingest, so history builds
from day one. A 13-month retention job; a daily `VACUUM INTO` backup under
the same retention.

**API** (as §2, from the latest good data): `/api/v1/prs`,
`/api/v1/prs/{owner}/{repo}/{n}`, `/api/v1/people`,
`/api/v1/people/{login}` (`?format=text`), `/api/v1/repos` (freshness,
`shadow` in step 1a, stale repositories), `/api/v1/stages`. Public,
cacheable, strong ETag (SHA-256 of the body); nothing reaches GitHub. Text outputs escape
Slack mrkdwn (`<!channel>`, `<url|…>`) and strip control characters.

**Operations.** `/healthz`; `/readyz` (DB and JWKS); operational metrics
on a non-public port; a staleness alert (scheduled runs get delayed, and
GitHub disables schedules in a public repository after 60 days without
activity); logs never carry `Authorization`, cookies or full IPs.

**Site.** The page from #67 served from the service's origin, reading the
API; Me keeps its picker until sign-in exists. Logout, when it comes, is a
fetch POST (the page's CSP has `form-action 'none'`).

**Code.** A `service/` crate in this repository's Cargo workspace beside the
analyzer, reusing its `dashboard` types: axum, tokio, rusqlite (bundled),
jsonwebtoken + reqwest for OIDC keys, tower-http. A Dockerfile and a deploy
note for infra.

**Increments (each a PR):**
1. 1a-1 — workspace, store, ingest with every check above, read API,
   health endpoints.
2. 1a-2 — the post job (dormant until the service URL is configured).
3. 1a-3 — the site on the API.
4. 1a-4 — the skill and the Slack digest on the API.
5. 1a-5 — schema v2 (recently closed PRs with `merged_at`/`closed_at`,
   GitHub user ids), then sign-in + Me + speed together.

## 6b. Step 1a-5 in detail — sign-in and your own speed

(Revised after a security/privacy/metrics review.)

**Data (analyzer), additive — `schema_version` stays 1** (every new field
is optional, and the page refuses unknown versions):
- `author_id` on every PR, from the PR's own author node.
- `reviews` on every PR: decisive reviews only — approved, changes
  requested, dismissed — read with `reviews(states: …)` so bot comments
  cannot push them past the page, with reviewer id, login, state and time.
- `closed`: PRs merged or closed in the last 30 days, each with author id,
  `created_at`, first ready-for-review time, `merged_at`/`closed_at` and its
  decisive reviews (an approval followed by a merge within 15 minutes is in
  no open snapshot). A one-time backfill mode reads a year of them.
- Ids on people looked up from roster logins are for display only; speed
  joins only on ids GitHub supplies with the event itself (a PR's author,
  a review's author), so a re-registered login cannot inherit history.

**Speed inputs (service), append-only, keyed by GitHub user id, kept 13
months** (and stated on the privacy page): `pr_authors` (PR → author id),
`asks` (PR, person id, first asked, ended, how it ended: answered, covered
by a co-approver, left review), `reviews` (PR, reviewer id, state, time),
`merges` (PR, first ready, merged). Recorded at ingest from `asks`,
`reviews`, `closed`; a PR that leaves the snapshot is closed from
`closed_at`. Intervals already running when first seen with no recorded
start, and intervals spanning a stale repository's gap, are excluded.

**Speed (computed on request, for the signed-in person only):**
- *Review wait you gave*: from when **you** were first asked on a PR to
  your first decisive review. Shown as asked N · answered M · still open K;
  only answered asks are timed (an ask covered by a co-approver or ended by
  the PR leaving review is counted, not timed), so leaving hard PRs does not
  look fast.
- *Your turn*: on your PRs, the total time per turn spent in the author's
  stages (self-review, answering an objection, a failed build), unbroken
  runs merged across bot bounces.
- *Cycle time*: first ready-for-review to merge on your merged PRs,
  labelled as including other people's review time.
- Rolling 3-month medians by month with n; a month with n < 3 shows n.
- At launch: a year of cycle time from the backfill; review wait and your
  turn from the engine's record revisions as far back as they survive
  (about mid-September 2026), and forward from then.

**Sign-in.** A separate **public** GitHub App with **no permissions**
(a private App only lets its own organisation's members sign in):
- `GET /auth/login`: a `__Host-` pre-login cookie (SameSite=Lax — a Strict
  cookie is not sent on the way back from github.com — 10 minutes, single
  use) holding `state` and a PKCE verifier; redirect to
  `github.com/login/oauth/authorize` with `client_id`, the fixed
  `redirect_uri`, `state`, `code_challenge`, `code_challenge_method=S256`.
- `GET /auth/callback`: `state` must match the cookie; POST the code to
  `/login/oauth/access_token` with `client_id`, `client_secret`,
  `redirect_uri` and `code_verifier`, treating an `error` field in a 200
  body as failure; `GET /user` once; revoke the token
  (`DELETE /applications/{client_id}/token`); create the session. Errors
  redirect to a fixed `/#/me?signin=failed`, never reflecting GitHub's text
  and never honouring a return URL.
- Sessions: random 256-bit id, stored hashed, holding user id, login and
  expiry; cookie `__Host-prh_session` (Secure, HttpOnly, SameSite=Lax, 30
  days); a few per user at most; expired rows purged daily.
- Every state-changing route — `POST /auth/logout`,
  `POST /api/v1/me/opt-out`, `DELETE /api/v1/me` — requires an exact
  `Origin` equal to the service's own (missing is refused). `/auth/*` and
  `/api/v1/me*` live outside the public router: no CORS, `private,
  no-store`, `Vary: Cookie`, no ETag.
- **Opt-out** deletes the person's speed inputs and stops recording them;
  the public queue, a mirror of GitHub, is unchanged. The opt-out itself is
  kept (the user id only) so it is honoured. **Delete** removes sessions and
  speed inputs and keeps the opt-out if there is one.
- No revocation webhook in this step: sessions hold no GitHub token.

**Page.** Me: signed out — "Sign in with GitHub" (reads only your public
profile) and a login search to anyone's public page; signed in — your
People page plus the speed panel ("Only shown to you here. Built from
public GitHub activity, so not secret; we don't publish or rank it."), and
sign out / opt out / delete. A privacy page states the source (GitHub, not
you), what is kept and for how long (sessions 30 days; speed inputs and
stage history 13 months; backups no longer), and how to opt out or delete.
State changes are fetch POSTs (the page's CSP has `form-action 'none'`).

**Increments:** 1a-5a the analyzer's additive fields and backfill mode;
1a-5b sign-in, sessions, the `/me` routes and the speed-input tables;
1a-5c speed; 1a-5d the page's Me and the privacy page.

## 6c. Step 1b in detail — the engine port

Goal: a Rust engine that, given the same reads, reaches the same verdicts and
makes the same writes as the Python engine. This is proven on the corpus and
then on live repositories before the Rust engine writes anything. 1b-1 (the
recorder, #82) is merged. `conformance/README.md` is the contract.
(Revised after feasibility, conformance, scope and security reviews.)

**Layout.** One **synchronous** library crate, `engine/`
(`pr-hygiene-engine`), with no reqwest and no tokio, so the corpus tests
never pull in a runtime. It has four modules; each depends only on the ones
above it.

1. `pycompat`: Python's behaviours, each written alongside its first caller
   rather than as a separate layer built up front.
   - **Values.** `PyValue` is an ordered dynamic value built on `IndexMap`.
     The engine's inputs arrive as `PyValue`, never as serde structs. Python
     works on partial input before it validates: `carried_heads` runs before
     `validate_policy`, and an absent `build` is not a `null` one. The case
     files `configuration-error-74c12d84a513` and `-ae236af27a58` pin both.
     Validation is ported over `PyValue` in Python's order and with Python's
     messages, and typed views are built only after it passes. The error type
     `PyErr::{Value, Type, Key}` maps one to one onto Python's `except`
     clauses.
   - **JSON reader.** A port of Python's scanner: `NaN`/`Infinity`, exact
     integers of any size, `raw_decode`'s end offset, and a duplicate key
     keeping its first position with its last value. This is what
     `IndexMap::insert` does. The reader is fed by serde's streaming visitor
     or is hand-written, never `serde_json::Value`.
   - **JSON writer.** One writer covers all five shapes the engine produces:
     - compact and sorted: the markers and the evidence print;
     - compact and unsorted: `diff_print`;
     - default separators and sorted: the admission context and the sort
       key inside `fingerprint`;
     - `indent=2`: the JSON report.

     It always applies `ensure_ascii` (lowercase `\uXXXX`, surrogate pairs)
     and rejects floats, of which no output contains any.
   - **Character classes and regexes.** Generated by enumerating every code
     point under Python 3.12, which pins Unicode 15 where current Rust is on
     16. The generated sets are `isspace`, `\s`, `\w`, `\d`, the case-fold
     set of each literal letter in a `(?i)` pattern, and `splitlines`'
     breaks: its own set, which excludes `\x1f` and treats `\r\n` as one
     break. The probes found real differences, all of which these sets
     close:
     - `(?i)` maps `İ`/`ı` onto `i` in Python and not in Rust;
     - `[^A-Za-z<]` under `(?i)` diverges the other way;
     - `\w` disagrees on `²`, `½`, combining marks and joiners.

     Every pattern is ported with these classes. `fullmatch` becomes
     `\A(?:…)\z` and `match` becomes `\A(?:…)`. Patterns built at run time
     are cached. Each pattern has a table test of inputs whose matches were
     recorded from Python.
   - **`PyDateTime`.** A port of CPython 3.12's `_pydatetime` `fromisoformat`
     and `isoformat`:
     - naive values, since `age()` turns the `TypeError` of aware minus naive
       into "unknown";
     - offsets with microseconds;
     - fractions cut to six digits;
     - any date–time separator, and week dates;
     - `.replace('Z', '+00:00')` replacing every `Z`, not only the last.

     It is not adapted from chrono, whose `FixedOffset` holds whole seconds.
   - **Ties.** `py_max_by` and `py_min_by` keep the first of equal
     elements; Rust's `max_by_key` keeps the last. `clippy.toml` uses
     `disallowed-methods` to ban `Iterator::{max, min}{,_by,_by_key}` in the
     engine.
   - **Goldens.** Python 3.12 generates them once into
     `conformance/pycompat/` and records its version. They are regenerated
     only when the reference Python changes.
   - **`serde_json` features stay frozen.** `preserve_order` would reorder
     the service's `json!` responses and change their ETags.
     `arbitrary_precision` breaks `#[serde(flatten)]` numbers in
     `src/policy.rs`. A CI check reads `cargo tree -e features -i
     serde_json` and fails on either feature.
2. `policy`: `validate_policy`, `governs` and `evaluate` with its helpers.
   These are pure functions, and `now` is an argument.
3. `evidence`: what `github.py` reads.
   - The seam is `Transport::call(Call) -> Result<Reply, Failure>`.
     - `Call` is one of `Rest { method, path, body, paginate }` and
       `Graphql { query, variables }`.
     - `Reply` holds the answer or its pages.
     - `Failure { transient, status, body }` describes a failed call.
   - A `Client` above it applies `_run`'s rules:
     - one retry of an idempotent call when `transient`, with the 2 s sleep
       injected;
     - a GraphQL answer that carries `data` is an answer;
     - an empty body is `None`;
     - invalid JSON is an error.
   - Two transports set `transient` differently:
     - **Replay** applies Python's substring rule to the recorded output, so
       conformance stays exact.
     - **HTTP** sets it from the status code and the kind of error.
   - Python's own 60 s subprocess timeout is not retried, while a timeout
     reported by `gh` is. Both cases are tested.
   - Requests carry the headers `gh` sends (checked with `GH_DEBUG=api`),
     because REST response shapes depend on the API version.
   - The snapshot builder, the per-evaluation caches and the history reader
     sit on top of the `Client`.
4. `reconcile`: the part of `main.py` the service runs.
   - It covers `publish`, `clear_marks`, the checklist, the move comment,
     labels, statuses, reviewer requests and nudges, over an explicit set of
     PRs or authors.
   - The clock is a `Clock` that the caller passes in.
   - Actions-only scheduling is not ported: `periodic_batch`, the build scan,
     the per-run nudge count and the Markdown report. The test harness
     derives a recording's batch from its `argv` and clock.

The HTTP transport and App authentication live in `service`. It runs the
engine in `spawn_blocking`, and the transport calls the service's existing
async reqwest client through `Handle::block_on`, so there is no second
runtime. The analyzer's `src/policy.rs` moves onto the engine's policy
types in its own PR after 1b-2.

**Python-side preparation** (small PRs, before 1b-3):
- Pin Python 3.12 (`setup-python`) in `pr-review-reusable.yml` and
  `pr-review-tests.yml`. The harvest records the version and asserts it.
- Harvest the pure functions `main.py` calls as committed, byte-exact cases:
  - `checklist_block`, `move_text` and `state_comment_body`;
  - `state_record`, `diff_record` and `admit`;
  - `admission_conflicts`, `receipt_print` and `diff_print`;
  - `telemetry.head_state`.
- Add one `python_exception_text` case, plus a Python test for `nudge`,
  which has none today.
- The recorder logs each `clock()` read with its call site and the ordinal
  of the call before it. The Rust `Clock` logs a site label, and replay
  compares the two. Python's dead read at `main.py:630` is removed.
- Record `sync --pr N`, the one-author path the service runs, alongside the
  batch syncs.
- **Records edited by someone other than the engine are ignored**, the same
  rule the diff record already follows.
  - Today any collaborator with write access can edit a record and change
    `admitted_at`, which moves their place in the queue, or `state`. Setting
    `state` to `ready-for-human` latches "was ready" and skips the green
    build gate.
  - `_set_aside_record` first moves to the GraphQL route that carries the
    editor; REST has none, and ignoring every edited record there would
    re-queue every PR.
  - The rule then lands as a TDD bug fix in Python, before the port copies
    it.

**Increments.** Each is its own PR, or a series of PRs. Each must be green
against `conformance/pending.txt`, which lists the cases known not to match
yet. The list may only shrink: a listed case that passes fails the test, and
so does an unlisted case that fails. This means a Python hotfix can land
with its case listed, and the port catches up afterwards.
- **1b-2 `pycompat` + `policy`.** Gate:
  - the 274 evaluate cases, the pure-function cases and the goldens;
  - the pure-helper tests ported from `test_policy.py` and
    `test_bot_timeouts.py`.

  Blockers marked `python_exception_text` are compared by state; every
  other field is compared exactly.
- **1b-3 `evidence`**, plus the HTTP transport and App authentication in
  `service`. Gate:
  - for every recording, each snapshot equals the `pr` that Python's
    `evaluate` saw;
  - every snapshot is served from recorded reads only.

  `test_github.py` becomes tests built on a response queue.
- **1b-4 `reconcile`.** Gate: the same verdicts, the same ordered writes
  (method, path and body), the same canonical outputs, and the JSON report
  compared as values. In addition, a **stateful fake** shows what the corpus
  cannot:
  - replaying the state after the writes produces no writes;
  - failing writes take Python's failure paths;
  - a first admission completes.

  `test_main.py`, `test_checklist.py`, `test_multi_repo_main.py` and
  `test_configuration_isolation.py` are rewritten as scenarios on the fake.
  The fake is anchored by **sandbox recordings**:
  - The sandbox is a repository outside dashpay with two or three test
    accounts. Creating it is an owner action and a prerequisite for 1b-4's
    exit.
  - There, a sandbox-only recording mode lets Python `sync --apply`. It is
    allowlisted to that one repository name and refuses every other. The
    recording keeps GitHub's real answers to the writes and the reads that
    follow them. It is committed, since it holds nothing private.
  - The fake must reproduce every one of those post-write reads.
  - The same sandbox proves rollback in both directions. After Rust has
    written, a Python dry sync makes no writes, and the reverse holds too.
- **1b-5 differential job**, the engine's shadow. A scheduled job in this
  repository runs every few hours with the read-only App token the export
  already uses. For each repository:
  - (a) Python records a dry `sync`, both the sweep and `--pr N` per
    author. Rust replays it, and it must match exactly, with no categories.
  - (b) Rust then reads the same PRs live through its HTTP transport. Its
    snapshots are compared with Python's recorded ones.
  - (c) A PR whose evidence print equals the one in Python's last real
    record must produce zero would-be writes against GitHub's actual state.
  - (d) Any difference is re-run with Python's `now`, its status-page
    payload and its `admitted_at` substituted. Only a difference that then
    vanishes is categorised (clock, telemetry or admission). Admission
    qualifies only when both sides have it set and the admitted PRs match in
    set and order. Every other difference is a port bug.

  The job outputs counts and categories only. Recordings never leave it:
  no artifacts, no logs of their contents. A red run blocks the next
  increment.
- **1b-6 the service loop, read-only.** The worker runs the Rust engine on
  events, per (repo, author).
  - It runs as a separate process, with no inbound port and its own uid, and
    it is the only process with the key mounted.
  - It uses a **dedicated read-only App**:
    - read access to Metadata, Pull requests, Issues, Commit statuses,
      Checks and Contents;
    - Actions read only if `workflowRun` needs it;
    - no organisation permissions;
    - its own rate budget.

    The write App is created and installed only at step 2, and its
    repository selection is the real per-repo switch.
  - A transport that refuses writes stays as a second layer, below
    pagination. It allows:
    - method plus host `api.github.com`;
    - GraphQL only with the exact text of a ported query.

    It also follows no redirects and accepts pagination links only on the
    same origin. Telemetry goes through a client that carries no token.
  - Would-be writes go into private tables outside the public router. They
    hold no permission levels, and hold checklist hashes rather than
    descriptions. They are compared with GitHub's state once the PR is
    quiet, and deleted, with their backups, when the repository cuts over.
- **Entry to step 2, per repository:**
  - `pending.txt` is empty;
  - three to four weeks of 1b-5 and 1b-6 have passed with no uncategorised
    difference;
  - every state and every write kind has been seen live or covered by a
    sandbox scenario.

  Today's live corpus has no `configuration-error`, no `too-many-open-prs`
  and no nudge.

**Trust model through cut-over** (corrects §3):
- Any collaborator with write access can edit any comment, and any workflow
  posts as `github-actions[bot]`.
- Before cut-over, both engines ignore edited records, as above.
- At cut-over each repository gets a cut-over instant. After it, a
  `github-actions[bot]` status or record is trusted only if it was created
  and last updated at or before that instant.
- The App rewrites every record at cut-over, and is recognised by its
  numeric bot id as well as its login.

**Deliberate divergences.** Each has a rule saying which fields the
comparison ignores, and each is a category in 1b-5 and 1b-6:
- The access race is not reproduced. Reads are sequential, and any later
  concurrency fills each cache once.
- A lone surrogate in JSON read from a bot's comment cannot be held in a
  Rust `String`. That read takes its function's unreadable path, the same
  path invalid JSON takes in Python.
- Over HTTP, `502`/`503`/`504` appearing in the text of a non-5xx failure
  does not trigger a retry. Replay keeps Python's rule.
- A status's `target_url` is the running commit (§3), where Python uses the
  run URL. Comparisons ignore it, and the first Rust run re-posts each
  status once.
- Nudges are limited per time window rather than per run, batches are
  chosen by events and sweeps, and reconciles are per author, with
  periodic sweeps still calling `clear_marks`.

Everything else is copied, including quirks that fail closed: review-thread
authors read without `[bot]`, and timestamps compared as strings. Fixes come
after cut-over, made once, in Rust.

**Live recordings.**
- Recordings are made with a read-only token: the job's installation
  token, or locally a read-only fine-grained token.
- Raw recordings stay outside every repository and worktree, in a mode-700
  directory, and are deleted once they have been redacted.
- `conformance/live/` holds redacted copies only, and the `live` test
  refuses one whose `meta.redacted` is `null`.
- Recordings never go into CI artifacts, issues, PR text or agent prompts;
  a PR reports counts only.
- `conformance/` stays out of `engine/`, which the service's Docker context
  includes.

**Failure modes.**
- **A read the port needs that Python never made** fails replay loudly.
- **A clock read in the wrong place** shows up in the logged sequence.
- **A Python change without re-harvesting** fails CI. A Python change that
  alters the boundary turns 1b-5 red within hours.
- **The fake and the port sharing one misunderstanding** is caught by the
  sandbox recordings, which hold GitHub's real answers.
- **A forged `github-actions[bot]` status** feeds both engines alike in 1b-5,
  which compares engine with engine, not engine with posted status.

**Verification.**
- `cargo test --workspace` runs the evaluate and pure-function cases, the
  goldens, the regex tables, the ported tests, the fake's scenarios, the
  sandbox replays and the `pending.txt` check.
- The scheduled differential job runs on live repositories.
- 1b-6 is judged on its comparison with GitHub's state.

## 7. Open PRs

- #65: merge (step 0).
- #66: ship its three live-bug fixes alone (bot accounts named `[bot]`, bots
  not merged into people, retry on a truncated response); close the rest.
- #67, #68: pause; the page, the product rules, the GitHub client and the
  backfill logic become the service's starting code.
- platform#5242 (`/prs` skill wording): fine now; the skill moves to the API
  in step 1.

## 8. Decisions

Owner, 2026-10-02: Rust; public API; public site; GitHub sign-in for anyone,
used to personalise Me and show one's own speed. Open: this roadmap starting
with step 0; hosting (infra team, §4).
