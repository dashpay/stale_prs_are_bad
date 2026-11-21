# Spec: PR Hygiene as one service with an API

Status: draft revision 8, 2026-10-02 — after simplicity, reliability/migration
and security reviews; the owner's decisions (**Rust**, a **public API**,
**GitHub sign-in open to anyone** that personalises the page); then a
Rust-port review and a public-API/privacy review. Replaces the delivery parts of `DASHBOARD_V2_SPEC.md`
(static site, `data` branch); its product parts — stages, "your part",
humans vs bots, lateness — carry over unchanged. Revision 9, 2026-11-21:
GitHub events drive the service read-only before the write cut-over (§6f).

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

Two always-on containers from one image, `serve` (front) and `worker`
(§6d, §6f), sharing the database volume; a persistent
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
   difference is a bug in the port. GitHub's events reach the service
   read-only first (§6f): the board follows a PR within seconds, and the
   shadow soaks on the real event path before step 2.
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
- Before cut-over, records follow these rules in both engines, as above:
  - A record is edited if GitHub gives it an edit time or names an editor.
    An edited record is trusted only when the engine is the editor; a null
    editor is not trusted.
  - Records are ordered by when the engine last wrote them, not by
    `updated_at`.
  - When the newest record is untrusted, the PR has no record. An older
    record is never brought back.
- **Accepted residuals:**
  - Deleting the newest record, or editing its marker out, brings back the
    one before it. This needs write access, and only matters when an older
    record holds an earlier admission.
  - A workflow on a pushed branch acts as `github-actions[bot]`. Only the
    App identity closes this.
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

## 6d. Step 1b-6 in detail — the service's read-only loop

(Revised after a feasibility and scope review.)

**What is already proven, and where.** The live differential job (1b-5,
#89–#96) already runs the service's own path against live GitHub. It runs
`Reconciler::run` over `Observed<ReadOnly<HttpTransport>>` with the job's
read-only token, and compares with Python's recording of the same moment:
snapshots, verdicts, would-be writes, and mechanical categories. Its one
gap is sampling: about 3 authors and 2 report PRs per repository every
6 hours.

So 1b-6 splits in two:

**1b-6a — full coverage, in the job, no host needed.**
- Each run adds a dry `Selection::All` pass per repository, read live by the
  Rust engine, next to Python's dry sweep of the same repository. That gives
  every open PR a verdict and a would-be-write check, not a sample.
- Rate: one all-PR pass costs about what a report does (≈600 requests for
  all five repositories). The job's budgets grow to fit, inside the App's
  measured headroom. If needed, the pass rotates across repositories.
- This starts the three-to-four-week verdict clock for step 2 now, rather
  than when hosting arrives.
- Built, then disabled by the owner: it overran the export App's rate
  limit. §6f's soak on events, with its sweep, takes over full coverage and
  restarts the clock.

**1b-6b — the operational soak, hosted.** This tests what the job cannot:
- an installation token minted and refreshed by the service
  (`InstallationTokens`; today only a given token is used live);
- a process that runs for weeks;
- the event path.

It was to be judged on operation, with 1b-6a carrying the verdicts. With
1b-6a disabled, it carries both (§6f).

**1b-6b's design.**
- **Process.**
  - `pr-hygiene-service worker`: a subcommand of the same binary, with the
    same multi-thread runtime, and one `spawn_blocking` per run.
  - It runs in its own container from the same image. The image is distroless
    with a single `USER`, so it can't run two uids in one container.
  - Only this container is given `PR_HYGIENE_READER_*` and mounts the key.
  - `serve` refuses to start if reader settings are in its environment.
- **Store.** The worker has its own SQLite file. It is owned by the worker,
  disposable and not backed up, so `serve` never sees it. It holds one
  table, keyed (repo, pr, head, evidence print) and holding:
  - the outcome;
  - the would-be write kinds;
  - the field paths that differed;
  - the policy commit;
  - the time.

  It holds no bodies, values, permission levels or listings, and rows go
  after 30 days.

  From §6f the worker also writes to the main database: queue leases and
  each PR's public dashboard row (an overlay). Nothing private goes there.
- **What drives it** (§6f has the detail).
  - Events. The reader App's webhook reaches `serve`, which queues the PR in
    the main database. The worker runs `Selection::Pr(N)` per PR, one at a
    time.
  - A sweep at the lowest priority, chunked per author, paced by a request
    budget. Replaces the serial `Selection::All` loop, which would block
    events for minutes on platform.
- **Writes** are caught by the engine's `Observed` layer, as in 1b-5. Each one
  is answered as a GitHub failure, so the run goes on and every PR gets a
  verdict. `ReadOnly` sits below it as the guarantee. A refusal from
  `ReadOnly` would stop the run at the first write.
- **Comparison.** There is one comparator, `conformance::live`, made generic
  over where Python's side comes from: here, the posted record and status in
  place of a recording.
  - Rust's verdict state is compared with the record's.
  - Where the record's state, head, admission and ready time equal what Rust
    decided, Rust must want no write.
  - Anything else is "unsettled" and is looked at again on a later pass.
  - A difference is re-run over what was already read, with the record's
    `admitted_at` and the time the record was written, at no new requests.
  - Telemetry can't be a category here, because the record holds no
    status-page payload.
  - A PR whose read failed (403 or 429 are not retried) is "not compared".
    It is not a difference.
- **Policies** are loaded from this repository's protected master, at the
  commit the API reports and after the same ruleset check Python makes. The
  commit is recorded per row, so a worker never decides on policies older
  than the ones the Python engine reads.
- **Rate metering.** `HttpTransport` reads the `x-ratelimit-*` headers of
  every answer. The loop pauses until the reset time when the reserve is
  reached. Step 2's pacing reuses this.
- **What it says.** After each pass it prints a summary in 1b-5's format
  (`conformance::report`: counts and field paths, never content), and the
  same table is kept in the store. Step 2's entry criterion reads 1b-6a's job
  summaries and this table.

**Failure modes.**
- Reader App not configured: the worker exits with a clear message; `serve`
  is unaffected.
- Rate limit near the reserve: the loop pauses until the reset time.
- A repository's reads fail: it is marked stale in the summary, and the
  others go on.
- A panic in one run: it is contained with the job's `guarded`/`panic_line`
  (location only), and the loop goes on.

**Verification.**
- **1b-6a:** the job's own tests, extended to the all-PR pass (budget,
  no-content, synthetic recordings served as live).
- **1b-6b:** mock-GitHub tests of the loop:
  - a sweep of two repositories;
  - the budget pause read from headers;
  - a contained panic;
  - a failed read not compared;
  - a would-be write recorded by kind only.

  Plus tests that `serve` refuses reader settings, and that a token is minted
  and refreshed across a simulated hour. Coalescing per author is tested in
  the events stage.

**Owner actions.**
- 1b-6a: none.
- 1b-6b and events: the reader App with its webhook, its installation, key
  and secret (§6f has the exact settings); the host's second container is
  an infra change (§6f).

## 6e. Step 2 in detail — push, not pull

Goal: once the service is hosted and writes, GitHub's events drive the
engine, and everything that reads PR Hygiene's state reads the service, not
GitHub. Request cost then follows what happens on the pull requests, not how
often anyone polls. §2 gives the receiver, queue and worker; this section
covers what changes in cost and state, and what has to hold for push to be
as correct as today's polling. (Revised after a review of the push model's
failure modes.)

§6f brings the event path forward, read-only. It builds the receiver, the
queue and its coalescing, the freshness check, delivery replay and the
safety net there. What follows is what writing adds on top.

**What reads the service instead of GitHub.** The dashboard, the `/prs`
skill, the Slack digest and any Actions job read the public API, which
answers from the store and never reaches GitHub (§2). This removes:
- the dashboard export through Actions (about 590 requests a run);
- each person's `gh` queries from the skill;
- the interim post path (§6a).

**What triggers a reconcile.**

- **Events.** Subscriptions:
  - `pull_request`, including `edited` (base, body), `labeled`/`unlabeled`,
    `assigned`/`unassigned` (assignees are holders) and
    `review_requested`;
  - `pull_request_review`, `pull_request_review_comment` and
    `pull_request_review_thread`;
  - `issue_comment`, including `edited` and `deleted` (records,
    attestations, CodeRabbit's edited notice);
  - `status`, `check_run` and `check_suite`;
  - `push` to a governed base branch.

  §6f starts without `check_run` and `push`. Step 2 adds them if the soak
  shows a failed build or a base change noticed too late.

  An event enqueues `(repo, pr)`, and the worker reconciles that author with
  `Selection::Pr`. This was measured at 15–40 requests, against about 1 200
  for platform's full pass.
- **Coalescing that loses nothing.**
  - The delivery id is recorded and the event enqueued in one transaction.
    A crash can't lose an event and also mark it delivered.
  - An event that arrives while its author's run is in progress marks that
    author dirty, and the run is followed by another.
  - Check events are debounced, and are enqueued only for PRs recorded as
    waiting on a build, as the build scan does today.
- **Cross-author effects.**
  - Every PR on a shared head is enqueued together and serialised per head,
    so two authors' runs never race on one commit status.
  - A base change that takes a PR out of policy runs `clear_marks` for that
    PR on the event, not on the next sweep.
- **The engine's own retries.** These are re-enqueued with backoff, because
  the event the App's own write raises is dropped by sender:
  - a "…reconciliation required" pending status;
  - "Policy reconciliation failed";
  - a nudge that failed;
  - a nudge deferred by the per-window limit.
- **Reads older than their event.** The worker checks the read against the
  event's payload: head SHA, review or comment id, check conclusion. If the
  read predates the event, the PR is re-enqueued with bounded backoff.
  Otherwise a repository with no further events would keep a stale verdict
  until the sweep.
- **Time.** A pure `next_change(policy, pr, now)` gives the next instant a
  verdict can change by time alone. That is the earliest of these, over the
  bots still owed:
  - first seen plus the nudge time;
  - first seen plus the waive time;
  - a CodeRabbit rate limit plus one hour.

  It is computed **after** the writes, from the `created_at` of the status
  just posted. "First seen" is the App's first status on the head, and that
  status's own event is dropped. A timer runs the author's reconcile then.
  The tests hold `next_change` to `evaluate`: no change at t−1 s, a change
  at t.
- **Inputs no event reports.** Each is polled every few minutes, and a change
  enqueues the PRs it affects:
  - the review system's status page, whose heartbeat and data windows move
    nudges;
  - this repository's master, for policy changes (the App isn't installed
    here);
  - collaborator permissions. There is no webhook for base-role changes,
    and `member` would need an organisation permission the reader App
    doesn't have.
- **A safety sweep.** The worker replays failed deliveries from the App's
  delivery log and reconciles everything on start-up. It also sweeps every
  PR in rotation, so no input is staler than one rotation. Alerting watches
  the PR whose last reconcile is oldest, not each repository.

**Cached reads.** The worker stores each REST answer's `ETag` and asks again
with `If-None-Match`. An authorized `304 Not Modified` "does not count
against your primary rate limit"
([REST best practices](https://docs.github.com/en/rest/using-the-rest-api/best-practices-for-using-the-rest-api)).
Its limits:
- **ETags depend on the token**
  ([community #75228](https://github.com/orgs/community/discussions/75228),
  not an official statement). An installation token lives an hour, so 304s
  help repeat reads within that hour, the event-driven re-reads. A sweep
  every few hours mostly pays full price. The cache is keyed by token, each
  token is used for its whole hour, and the soak measures the 304 share
  across rotations.
- **REST reads only.** GraphQL has no conditional requests, and it is about
  a third of the engine's requests. Writes are never conditional, and 304s
  still count toward the secondary limits.
- **ETag only, never `Last-Modified`,** which follows `updated_at` rather
  than the body. GitHub's `max-age=60` is ignored: every reuse is a
  conditional request.
- **Pagination.** A 304 on a page gives that page's cached body. The `next`
  link comes from the 304's own `Link` header, never from the cached page.
  Otherwise a new item on a page after an exactly full last page would never
  be read.
- **Conformance.** The transport returns the cached body, so the engine never
  sees a 304. The per-evaluation caches stay as they are, within one
  evaluation.

**Writes.**
- **Secondary limits.** These include about 80 content-generating requests a
  minute and 500 an hour
  ([rate limits](https://docs.github.com/en/rest/using-the-rest-api/rate-limits-for-the-rest-api)).
  Work runs in priority classes: event, then timer, then sweep, then mass
  reconcile. A write reserve is kept for events.
  - A cut-over (every record rewritten, every status posted again, §6c) or a
    policy change is a paced mass reconcile.
  - Each pass pins one policy commit.
- **No duplicate posts.** A write-intent row per (PR, head, kind) stops a
  run seconds after a crash or timeout from posting a nudge or move comment
  again. Without it, the comment read could lag behind the post it just
  made.
- **The worker is merge-critical.** While it is down, PRs keep their last
  status and can't turn green. Alerting covers failed deliveries, queue
  age, and the oldest last reconcile.
- **Times are processing times.** `ready_since` and `admitted_at` are set
  when the worker processes the PR, so delivery lag shifts them by that
  lag, as Actions queueing does today.

**The engine's identity, everywhere it reads its memory.** At cut-over the
App writes the records. `src/stages.rs` hard-codes `github-actions[bot]`
(line 23), so stage history would stop seeing them. It moves onto the
engine's identity list with the rest (§6 step 0), in the same change that
lets the engine trust the App.

**Where the engine's memory lives.**
- **At cut-over (v1).** Record comments and commit statuses on GitHub stay the
  memory, and the store is history and cache. Rollback to the Python engine
  is a switch only while both engines read the same memory.
  - The trust model's deleted-record residual (§6c) closes in v1. Each
    record carries a sequence number the store also holds, and a record
    behind the store's number is not trusted.
  - Step 3 drops trust in `github-actions[bot]`, which closes the
    branch-workflow residual.
- **Optional, after Python is retired (v2).** The store becomes the memory
  for records.
  - **What stays on GitHub:** commit statuses (`head_seen_at`,
    `ready_published`) remain on GitHub as memory.
  - **The mirror:** the service keeps writing v1-format records. These are
    trusted only when the App wrote them and nobody edited them. They are the
    restore source when a backup is older than the last write, and the way
    back to v1.
  - **Migration,** done per repository with that repository's queue paused:
    - include closed and draft PRs' diff records;
    - apply the trust rules on import;
    - require identical verdicts from both memories before the switch;
    - make it resumable after a crash.

  With the trust residuals closed in v1, v2's gain is mostly speed and
  simplicity. It is worth doing only if the soak shows record reads are a
  real cost.

**What is retired, when.**

| What | When |
|---|---|
| The five repositories' hourly sweeps and quarter-hourly build scans in Actions | per repository at its cut-over (`PR_REVIEW_AUTOMATION_ENABLED`) |
| The dashboard export through Actions and its post job | slowed on §6f's evidence; retired once the worker's own full pass replaces it and the owner accepts Rust's verdicts on the board |
| The engine differential job | after the last cut-over |
| The Python engine and the callers | step 3 |

**Verification.**
- **Worker tests on the stateful fake:**
  - a burst coalesced, and an event during a run causing another run;
  - a delivery recorded and enqueued atomically, with a crash between;
  - each self-scheduled retry re-enqueued;
  - a read older than its event re-enqueued;
  - `next_change` against `evaluate` at t−1 s and at t;
  - a 304 serving the cached page with `next` taken from the 304;
  - a shared head serialised;
  - a write-intent row stopping a duplicate post;
  - recovery by delivery replay, and by the sweep.
- **In 1b-6b's soak (shadow, before step 2):**
  - requests per hour;
  - the 304 share across token rotations;
  - delivery lag;
  - queue age;
  - the oldest last reconcile;
  - secondary-limit headroom.

  Step 2 starts only when these have been stable for the soak's duration.

## 6f. Events first, read-only — before the write cut-over

Goal: the dashboard, the API, the `/prs` skill and Slack follow a pull
request within about half a minute of a change on GitHub, not up to 15
minutes later. Nothing is written to GitHub. The Rust engine runs in shadow
on the same events: this is 1b-6b's soak, and its clock restarts here for
step 2. (Owner decision, 2026-11-21: a GitHub App webhook to the service,
brought forward from step 2.)

**In one paragraph.** A new read-only App sends its webhook to `serve`. The
receiver checks the signature, records the delivery and enqueues the pull
request in one transaction, and answers. A second container, `worker`,
holds the App key. It takes each pull request off the queue, reads it once
the way the analyzer does and runs the Rust engine's dry `Selection::Pr`
over it, and writes one **overlay** row, the PR's dashboard row, into the
database `serve` answers from. The 15-minute Actions run keeps posting full
snapshots, which stay the base layer and the safety net. Read times decide
which of the two is newer, so an older snapshot never overwrites a newer
overlay, and an older overlay never outlives a newer snapshot.

Rejected: governed repositories' Actions pushing their per-PR state to the
API. That needs `id-token: write` in other teams' caller workflows. It gives
a partial row, since the engine has verdicts and only the analyzer derives
the dashboard's fields. And step 2 throws it away.

### The App

A **new, dedicated, read-only App**: the reader App 1b-6 already specifies
(§6c), now with a webhook.

Why not a webhook on the existing `pr-hygiene` App (id 4912985):
- **Rate.** The export already spends about 2 400 of its 5 000 requests an
  hour. The engine differential was disabled because it overran that budget.
  Events and the soak need their own budget.
- **Key.** Its key is an Actions secret. A second key of the same App on the
  host would make a host compromise yield the export's identity as well.
- **Permissions.** It has contents, issues, metadata and pull requests (read).
  It lacks the statuses, checks and actions the engine reads
  (`reader::app::PERMISSIONS`). Adding them needs the org owner's approval of
  each new permission ([modifying an App](https://docs.github.com/en/apps/maintaining-github-apps/modifying-a-github-app-registration)),
  the same work as installing a new App.

At the write cut-over, a **separate writer App** is created (§6c). It is
installed repository by repository, and its installation is the per-repo
switch. The reader App keeps the webhook and every read. Rejected: giving
the reader App write permissions then. That makes the long-lived,
webhook-holding key a write key, and loses the per-repo switch. The writer
App's own writes come back as events on the reader's webhook and are
dropped by its bot's numeric id.

**Settings, for the owner to enter** (*Register new GitHub App* under the
dashpay organisation; nothing is created before this spec is agreed):

| Field | Value |
|---|---|
| GitHub App name | `Dash PR Hygiene Reader` (≤ 34 characters, unique across GitHub; any free name will do) |
| Description | "Read-only. Reads pull requests in the governed repositories and receives their events for the PR Hygiene service. Writes nothing." |
| Homepage URL | `https://pr-hygiene.services.dash.org` |
| Callback URL, Setup URL, Device Flow, Request user authorization during installation | empty / off |
| Webhook **Active** | on |
| Webhook URL | `https://pr-hygiene.services.dash.org/github/webhook` |
| Webhook secret | 32 random bytes as hex (`openssl rand -hex 32`), never written anywhere but the App and SSM (below) |
| SSL verification | enabled |
| Repository permissions | Actions **Read-only**, Checks **Read-only**, Commit statuses **Read-only**, Contents **Read-only**, Issues **Read-only**, Metadata **Read-only** (mandatory), Pull requests **Read-only**; everything else No access |
| Organization and account permissions | none |
| Subscribe to events | Check suite, Issue comment, Pull request, Pull request review, Pull request review comment, Pull request review thread, Status |
| Where can this GitHub App be installed? | **Only on this account** |
| Installation | dashpay → **Only select repositories**: platform, rust-dashcore, tenderdash, grovedb, dash-evo-tool |

Then the owner generates one private key, records the App id and the
installation id, and hands both secrets to SSM (§ *Secrets*).

- The permissions are exactly what `InstallationTokens` asks for. It asks
  for each at `read` and refuses a token that grants more.
- Each event needs a permission in this list
  ([events and payloads](https://docs.github.com/en/webhooks/webhook-events-and-payloads)).
  `installation` and `installation_repositories` arrive without a
  subscription.
- **Not subscribed at first:**
  - `check_run`: about 25–30 runs per platform head, two or three deliveries
    each, measured on two heads on 2026-11-21. Suites complete per workflow,
    and suite completion is what the build state needs here.
  - `push`: a base-branch push touches every PR on that base. That is left
    to the full pass.
  - Step 2 may add either. Checks read covers `check_run`, so no new
    permission is needed.

### Receiver, in `serve`

`POST /github/webhook`. In order, cheapest first, nothing parsed before the
signature holds:
1. **Headers.** `X-GitHub-Event` is a subscribed event, `installation` or
   `ping`; `X-GitHub-Delivery` is a GUID; `X-GitHub-Hook-Installation-Target-ID`
   is the App id; `X-Hub-Signature-256` is `sha256=` and 64 hex digits.
   Anything else gets 400 and is not read further. These headers are filters,
   not proof.
2. **Body.** Read to at most 1 MiB, the limit nginx also enforces; larger
   gets 413. GitHub caps payloads at 25 MB, but every subscribed event is
   bounded by two 65 536-character texts plus GitHub's objects.
3. **Signature.** HMAC-SHA256 of the raw bytes with the webhook secret,
   compared with `hmac`'s `verify_slice`, which runs in constant time
   ([validating deliveries](https://docs.github.com/en/webhooks/using-webhooks/validating-webhook-deliveries)).
   During a rotation both the current and the next secret are tried.
   Failure gets 401, and only a counter records it.
4. **Scope.** The payload's `installation.id` is the configured installation.
   The repository is a governed one by numeric id: the worker resolves the
   registry's names to ids at start-up and writes them to the database. An
   `issue_comment` counts only on a pull request (`issue.pull_request`
   present), and `check_suite` only when `completed`. Anything else gets
   **202 and is dropped**, never 4xx: a 4xx marks the delivery failed, and
   the replay below would send it again.
5. **Record and enqueue.** In one `BEGIN IMMEDIATE` transaction:
   `INSERT OR IGNORE` the delivery id into `webhook_deliveries` (kept 7 days,
   beyond GitHub's 3-day redelivery window); only if it is new, upsert the
   queue entry (below). A repeat id gets 202 and changes nothing: a
   redelivery carries the same `X-GitHub-Delivery`
   ([best practices](https://docs.github.com/en/webhooks/using-webhooks/best-practices-for-using-webhooks)).
   Then 202.

GitHub expects a 2xx within 10 seconds (same page). The route has its own
8-second deadline and at most 32 requests in flight, so a flood cannot
starve the API. A busy database or the deadline gets 503, and the replay
recovers the delivery. Logs carry the delivery id, event, action,
repository, PR number and outcome: never a body, a header value or a
signature. Bodies are dropped after the transaction; only the facts below
are kept.

What the queue keeps from a payload is a **hint**, never evidence:
- the pull request number, or the head SHA for a `status` event or a fork's
  check suite, whose payload names no PR;
- the facts the freshness check needs: the head SHA, the review or comment
  id, and the PR's state (closed or draft).

**Infra change to nginx** (infra owns it):

```nginx
limit_req_zone $binary_remote_addr zone=prh_hook:10m rate=20r/s;

location = /github/webhook {
    limit_except POST { deny all; }
    client_max_body_size    1m;
    client_body_buffer_size 1m;   # held in memory, never written to disk
    limit_req zone=prh_hook burst=200 nodelay;
    proxy_read_timeout 10s;
    proxy_pass http://prh_service;
}
```

The existing `prh` access log holds the path, status and size only, with no
headers. No log format may name `$request_body` or `$http_x_hub_signature_256`.

Rejected: allowing only GitHub's `hooks` ranges from `/meta`. A stale list
silently drops real deliveries, the signature already authenticates, and a
refused delivery is recovered by the replay.

### Process and store

Two containers from one image, as infra#92's README already plans:
- **`serve`**: internet-facing. It holds the webhook secret and never the App
  key, and it refuses to start with `PR_HYGIENE_READER_*` set (§6d).
- **`worker`**: no port. It holds the App key and refuses to start with the
  webhook secret set.

Both open the **main database** on `/data`. The worker also keeps 1b-6b's
private shadow database on a volume of its own, which `serve` never mounts
and which is not backed up.

Compared:

| Option | For | Against |
|---|---|---|
| **Shared SQLite, WAL, two containers on one host** (chosen) | The queue's "record and enqueue" and the worker's lease are SQL transactions. There is no internal API to authenticate, version or bound. The same `Store` code is used by both, and the host's daily `sqlite3 VACUUM INTO` already opens this file from a second process. | Two writers. Writes are short `BEGIN IMMEDIATE` transactions with a 5 s busy timeout. Schema changes have one owner: `serve` migrates, and the worker waits until `user_version` equals its own. |
| Worker writes through an internal endpoint of `serve` | One writer; `serve` could validate rows. | A second listener, its authentication and a wire format for rows and leases. It protects nothing: a compromised worker holds the key anyway, and a compromised `serve` can forge queue entries in both designs, because it holds the webhook secret. |
| One process | Simplest to deploy. | Puts the App key in the internet-facing process (§3, §6d). Step 2 would have to split it again, with a write key. |

WAL across containers works because both run on one kernel over a local
ext4 volume, as the same uid (65532). The `-shm` file is shared through the
bind mount. Never on NFS (§2).

**What the worker trusts in the main database.** Only queue rows, as hints,
and the base layer, to merge into. It decides the **policy commit** itself:
- It reads master's head from GitHub, without a token. A row `serve` wrote
  could otherwise point it at any commit in the repository's network.
- It loads `policies/` and `.pr-hygiene.yml` at that commit, cached by
  commit, after the ruleset check §6d makes.

**Secrets.**
- Two SSM Parameter Store `SecureString` parameters:
  `/pr-hygiene/webhook-secret` and `/pr-hygiene/reader-app-key`, plus
  `/pr-hygiene/webhook-secret-next` during a rotation.
- Terraform creates the parameters with a placeholder value and
  `ignore_changes = [value]`. The owner, or infra with the owner, puts the
  real values with `aws ssm put-parameter`, then deletes the downloaded PEM.
  Real values never enter Terraform state or a repository.
- A host unit, run before `docker compose up` and by `pr-hygiene-deploy`,
  reads them with the instance role. The role gets `ssm:GetParameter` on
  exactly these ARNs. The unit writes them to `/run/pr-hygiene/secrets/`
  (tmpfs, mode 0400, uid 65532).
- Each is mounted read-only into one container only:
  `PR_HYGIENE_WEBHOOK_SECRET_FILE` in `serve`,
  `PR_HYGIENE_READER_APP_KEY_FILE` in `worker`.
- Containers cannot reach the instance role (IMDSv2, hop limit 1), so a
  compromised `serve` cannot fetch the key.
- Rejected for now: signing the App JWT with a non-exportable KMS key. The
  container would need AWS credentials, and the key is read-only. It is
  required for the writer App at step 2 (§3).

**Rotation.**
- **Webhook secret:** put `-next`, restart `serve` (it accepts both), change
  the secret in the App, promote `-next`, restart again.
- **App key:** GitHub allows up to 25 keys per App
  ([private keys](https://docs.github.com/en/apps/creating-github-apps/authenticating-with-a-github-app/managing-private-keys-for-github-apps)).
  Generate the new key, put it in SSM, restart the worker, then delete the
  old key on GitHub.

### Queue and coalescing

`refresh_queue` is keyed by (repository, PR), or (repository, head SHA)
until the worker resolves the SHA.
- **Upsert.** It merges the hints, keeps the first enqueue time, and sets
  `due_at`:
  - PR, review and comment events: 5 s after the last event, at most 30 s
    after the first;
  - check suites and statuses: 30 s after the last, at most 120 s after the
    first.
- **The queue is bounded by the open PRs, not by the deliveries.** A burst of
  check events is one entry.
- **One worker loop, serial.** That keeps the engine's per-author
  serialisation trivially (§2). It **leases** the earliest due entry with a
  `leased_until`, so a crash returns the entry when the lease expires.
- **An event during a run.** It bumps the entry's generation. When the run
  ends, the entry is deleted only if its generation is unchanged; otherwise
  it is re-armed (§6e's "followed by another").
- **One PR is refreshed at most once every 20 s,** which bounds what a flood
  or a replayed delivery can cost.
- **An entry whose last event predates its repository's latest base
  `reads_from` is dropped.** The full pass has already seen it, so a backlog
  after an outage drains to what is still new.
- **Head index.** The worker keeps (repository, PR, head) in its own tables
  to resolve SHA entries. It fills the index from refreshes and from a
  **conditional poll of each repository's open-PR list** every 5 minutes:
  `GET /pulls?state=open`, one or two pages, with an ETag, so mostly 304s.
  The same poll enqueues:
  - any PR whose head changed or whose `updated_at` is later than its last
    read;
  - any PR on the board that the list no longer has, and any PR in the list
    the board lacks.
  That catches most missed deliveries within 5 minutes, at almost no cost.
  §6e's pagination rule for 304s applies.

### Per-PR refresh

For one PR, in one blocking task:
1. **The analyzer's read, for one PR.** A new
   `Fetcher::fetch_pr(owner, name, number)`, as one GraphQL query:
   - `repository { defaultBranchRef pullRequest(number:) { … } }` with the
     list query's node selection, shared as one fragment, plus `state` and
     `headRefOid`;
   - `parse_pr_node`;
   - the existing per-node follow-ups for threads and files;
   - `recheck_mergeable`;
   - `fetch_stage_evidence` for this node.

   About 3–5 GraphQL points.
2. **Freshness.** The read must show what the queued events reported: the
   head SHA, the review or comment id, closed or draft. A read that predates
   its event is retried after 2, 5, 15 and 60 s. Then it is dropped and
   counted, and left to the poll and the full pass (§6e).
3. **The engine, dry.** `Reconciler::run` with `Command::Sync`,
   `apply: false`, `Selection::Pr(n)`, over `Observed<ReadOnly<HttpTransport>>`
   (§6d), at the worker's policy commit. About 15–40 REST requests, as
   measured for a one-author sync.
   - Its report row has the shape Python's `report --format json` exports. It
     is read into the analyzer's `PolicyState` exactly as the exported file
     is.
   - Its would-be writes feed the shadow comparison (below). **One run serves
     both.**
4. **Derivation**, using the analyzer's own code:
   - `analyzer::analyze` with the PR alone (exclusions are pure
     configuration);
   - `scorer::score_prs` and `attach_engine_state`;
   - a new `dashboard::pr_part`, shared with `dashboard::build`, which gives
     the `PrOut` and the PR's **person contributions**: the author's
     `authored` key, and each reviewer's `Owed` from `dashboard::part`.
   - The grace period needs the analyzer's author cache, which lives on the
     `data` branch. A PR already on the board keeps its `tracked`. A new PR
     is tracked if its author has a tracked PR on the board, and otherwise
     waits for the full pass.
5. **A closed or merged PR** gives a **tombstone** overlay, and leaves the
   board.

An engine read that fails (403/429 not retried, as in §6d) writes no
overlay. The entry is retried with backoff, then dropped and counted. There
are no half-rows.

**Whose verdict.** Overlays show the **Rust port's** verdict. Full snapshots
show **Python's** (its `report`, re-evaluated in Actions, which is what the
board has always shown, not the posted status). This departs from §2's
"Python's, as posted" during shadow, and needs the owner (report, decision
1). It is limited three ways:
- stage B below gates it on the overturn rate;
- each overlay names its source;
- the kill switch removes every overlay within one full pass.

The alternative refreshes only GitHub's own fields per event and keeps the
engine's fields from the snapshot. Stage, reviews owed and "whose move"
would then still lag by up to 15 minutes: most of what the owner asked for.

### Merging with the full snapshot

- **Read times, not arrival times.**
  - An overlay's `read_from` is the worker's clock just before its first
    read.
  - A snapshot's `reads_from` is its run's start, before the engine export.
    It is a new additive field, set by the workflow's first step and checked
    on ingest like `generated_at`: no later than `generated_at`, no earlier
    than the token's `iat` minus the job timeout.
  - A snapshot without it counts as `generated_at` minus the job timeout,
    the conservative side.
- **For each PR of a repository, the base wins only if
  `reads_from ≥ read_from + 60 s`.** The margin covers the clock skew
  between runner and host. Otherwise the overlay stands. The consequences:
  - an older snapshot never replaces a newer row;
  - its absence of a newer PR deletes nothing;
  - its listing of a PR an overlay closed does not bring it back.

  At ingest, overlays the base has overtaken are deleted, tombstones
  included. A reopened PR returns through its event or the next snapshot.
- **Stale repositories.** The base keeps its last good data, and overlays
  keep it current; `/api/v1/repos` gains `refreshed_at`. Overlays apply only
  to repositories that have a base.
- **People.**
  - A repository's people are its base people parts, minus every entry keyed
    by an overlaid PR (owes and authored are keyed by PR; `service/src/view.rs`), plus the
    overlays' contributions.
  - `wip` is recounted from the final rows with the analyzer's own rule
    (`counts_toward_wip`, extracted from `dashboard::people`).
  - Areas come from the policies, and nothing per PR changes them.
  - `view::merge_people` then does what it does today.
- **Stage history.**
  - A worker write records stage changes from overlays, with `observed_at`
    set to `read_from`.
  - An ingest records none for PRs where an overlay still wins, so history
    never steps back to an older read.
  - `stage_changes.snapshot_id` becomes nullable, with an `overlay_id`
    beside it.
- **Speed inputs and the closed list** come from full snapshots only, as
  today: monthly medians do not need seconds.
- **Version.** `view.version` becomes a sequence that every assembly bumps
  (ingest or overlay), so `serve`'s cached view and the ETags follow. Stage
  changes carry the sequence they were written at.
- **Kill switch.** `PR_HYGIENE_OVERLAYS=off` on the worker: at start it
  deletes every overlay and writes no new ones. The next assembly serves
  full snapshots only.

### Shadow engine on the same events (1b-6b)

- **Events.** Each refresh's dry run is compared as §6d describes: Rust's
  state against the posted record, and would-be writes where the record
  agrees. The result goes to the private shadow table.
  - Python's own `report` arrives every 15 minutes in the full snapshot.
  - Each overlay is also compared with the next snapshot that started after
    it, for the same PR and head with no event in between: the **overturn
    rate**.
- **Sweep.** §6d's sweep runs at the lowest priority. It is chunked as
  `Selection::Batch` of one author's PRs at a time, so a due event waits at
  most one chunk (about 10–20 s).
  - It pauses while any queue entry is due, and while the installation's
    remaining REST is under a reserve of 1 500.
  - It covers every PR about every 2 hours, about 300 requests an hour.
- **Priority:**
  1. event refreshes and their retries;
  2. the delivery replay and the open-PR poll;
  3. the sweep.

### Recovering lost deliveries

GitHub does not redeliver failed deliveries
([handling failed deliveries](https://docs.github.com/en/webhooks/using-webhooks/handling-failed-webhook-deliveries)).
Failures can be redelivered for 3 days
([redelivering](https://docs.github.com/en/webhooks/testing-and-troubleshooting-webhooks/redelivering-webhooks)).

The worker holds the key, so it replays them:
- Every 5 minutes, with the App's JWT, it reads
  `GET /app/hook/deliveries?per_page=100` (cursor-paged, newest first) back
  to its last high-water mark.
- For each `failure` whose GUID is not in `webhook_deliveries`, it sends
  `POST /app/hook/deliveries/{id}/attempts` (202)
  ([App webhook API](https://docs.github.com/en/rest/apps/webhooks)).
- 413s and payloads for ungoverned repositories are not replayed. A delivery
  that fails twice more is left to the safety net.

The **safety net** is the open-PR poll every 5 minutes and the full pass
every 15. The latter is also the only one for inputs no event reports:
time-driven waivers and nudges, permission changes and the status page
(§6e).

### Full reconciliation cadence

- **The 15-minute Actions pass stays** as the base and as Python's verdicts.
- **Hourly** (a cron change in `pr-hygiene.yml`) once stage C has run 7
  days with:
  - drift of at most 1% of rows per pass, each explained. Drift is a full
    pass changing a row with no event since the overlay, excluding
    lateness and idle, which move with time.
  - no failed delivery left unrecovered;
  - the worker's sweep running.
- **Every 6 hours** only once the read-only `next_change` timers (§6e)
  enqueue time-driven transitions, and the sweep covers every PR hourly.
- **Retiring the export** comes when the worker's own full pass replaces
  it. That means the board shows Rust's verdicts everywhere, and is the
  owner's call after the soak (§6e's table).

### Rate budget

**Reader App** (new; 5 000 REST an hour and 5 000 GraphQL points an hour
per installation, separate pools;
[REST](https://docs.github.com/en/rest/using-the-rest-api/rate-limits-for-the-rest-api),
[GraphQL](https://docs.github.com/en/graphql/overview/rate-limits-and-query-limits-for-the-graphql-api)):

| Use | REST / hour | GraphQL points / hour |
|---|---|---|
| Event refreshes, 15–40 an hour (estimate below) × ~25 | 400–1 000 | 50–200 |
| Shadow sweep, all PRs every ~2 h | ~300 | — |
| Open-PR poll, 5 repos × 12, mostly 304 | ≤ 60 | — |
| Policy commit and files (tokenless) | 0 | — |
| **Total** | **≈ 750–1 400** (peak ≈ 2 500) | ≤ 200 |

The delivery replay goes through the App's JWT, outside the installation's
budget: about 12 requests an hour.

The estimate:
- The repositories' public event feeds over the last days give about 22
  PR-related events an hour across the five. That is a lower bound: the feed
  omits statuses, checks and many actions.
- Check suites add about 9–16 per platform head.
- Coalescing turns these into one refresh per PR per burst.
- The soak measures the real figure. A reserve of 1 500 is always held for
  events.

**`pr-hygiene` App** (export): unchanged at about 2 400 an hour while the
cadence is 15 minutes; about 600 hourly, about 100 every six hours. The
headroom that frees can bring back the 1b-5 differential (disabled by the
owner for budget).

### Failure modes

| Case | What happens |
|---|---|
| Delivery lost (host down, 429, 5xx, timeout) | Replayed from the delivery log within 5 minutes, while under 3 days old; otherwise the poll (5 min) or the full pass (15 min) |
| Delivery duplicated | Same GUID: 202, nothing enqueued; refreshes are idempotent anyway |
| Events out of order | Payloads are hints; the worker reads the present; read times order rows |
| Read older than its event | Freshness check, bounded backoff, then poll and full pass (counted) |
| Receiver up, worker down | Deliveries are acknowledged and queued, so none is lost; the board ages to the 15-minute pass; restart drains one refresh per PR; alert at worker heartbeat > 5 min or oldest due entry > 10 min |
| Backlog | Bounded by the open PRs; entries the base has overtaken are dropped |
| Secret or key rotation | Two secrets accepted during a webhook-secret change; overlapping App keys |
| Captured delivery replayed | GUID kept 7 days; later replays only enqueue a refresh of a real PR, at most every 20 s; nothing in a payload reaches the board |
| Flood on the public route | nginx per-client rate and connection limits, 1 MiB body, header timeouts; in the service, cheap header checks first, 32 in flight, 8 s deadline; refused real deliveries are replayed |
| Repository or PR outside the governed set | 202, dropped and counted; ungoverned *branches* of governed repositories are refreshed, as the board shows them |
| App uninstalled, suspended or permissions pending | `installation` events logged; token minting fails; the worker is not ready; the full pass carries on |
| Wrong overlays (a port bug) | Overturn and drift show it; kill switch; the next full pass restores the base |
| Policy change | Overlays record their policy commit; the next pass and refreshes use the new one |
| Clock skew | 60 s margin in the merge rule; both clocks NTP-synced |

### Verification

- **Receiver** (axum tests with signed fixtures):
  - a valid signature passes;
  - a wrong, missing or SHA-1-only signature is 401;
  - an oversized body is 413 before any HMAC;
  - a repeated GUID enqueues nothing;
  - a foreign installation or repository is 202 and dropped;
  - both secrets pass during a rotation;
  - a busy database is 503 within the deadline;
  - a log-capture test finds no body, signature or header value.
- **Queue:**
  - coalescing;
  - an event during a lease causes another run;
  - a lease expires after a simulated crash;
  - record and enqueue are atomic;
  - an entry the base has overtaken is dropped.
- **Worker on mock GitHub** (httpmock, the reader's existing `mock`):
  - a read older than its event is retried, then dropped and counted;
  - closed gives a tombstone, opened a new row;
  - a status resolves through the head index;
  - a fork's check suite;
  - a failed engine read writes nothing;
  - the reserve pauses the sweep, never events;
  - the replay retries only unseen failures;
  - the poll enqueues a missed head change.
- **Merge, as an oracle.** For a fixture board and one changed PR —
  approved, closed, opened, a draft flipped, an author's `wip` changed:
  ```
  apply_overlays(build(before), pr_part(after, PR)) == build(after)
  ```
  The `PrOut` rows and the people must be equal. And `build` stays equal to
  assembling every PR's `pr_part`, which proves the refactor changes
  nothing.
- **Ordering:**
  - an older snapshot keeps a newer overlay and records no stage change for
    it;
  - a newer one prunes it;
  - a tombstone, then a reopen.
- **Configuration:** `serve` refuses reader settings, and the worker refuses
  the webhook secret.

### Rollout

| Stage | What runs | Exit |
|---|---|---|
| A. Record | Receiver live; deliveries recorded and queued; no worker | 2 days: every delivery GitHub lists verifies; no failures in the App's log beyond deliberate tests; p99 answer < 500 ms; largest body < 512 KiB |
| B. Compute | Worker on; refreshes and shadow run; overlays written to the private table only, not served | 2–3 days: ≥ 95% of entries refreshed within 2 min; overturn ≤ 1% on `engine_state`, stage and owes, every case categorised; REST ≤ 50% of the hour; the soak clock for step 2 starts here |
| C. Serve | Overlays applied (dashboard, API, skill, Slack) | 7 days: delivery→view p50 < 30 s, p95 < 2 min; drift ≤ 1% per pass; kill switch not needed |
| D. Hourly | Actions pass hourly | 7 days of drift as in C |
| E. 6-hourly | After read-only `next_change` timers and an hourly sweep | — |

**Owner actions:**
- agree the decisions in the report;
- create the App with the settings above and install it on the five
  repositories;
- generate the key, put it and the webhook secret into SSM, delete the local
  copy;
- give infra the App id and the installation id;
- at D and E, merge the cron change.

**Infra actions** (dashpay/infra):
- **The worker** in the compose file:
  - the same image digest, `command: ["worker"]`, no ports;
  - `read_only`, `cap_drop: [ALL]`, `no-new-privileges`, tmpfs `/tmp`;
  - `/srv/pr-hygiene/data:/data` and `/srv/pr-hygiene/worker:/work` (not
    backed up);
  - `PR_HYGIENE_READER_APP_ID`, `PR_HYGIENE_READER_INSTALLATION_ID`,
    `PR_HYGIENE_READER_APP_KEY_FILE` and `PR_HYGIENE_WORKER_DB`.
- **`serve`** gains `PR_HYGIENE_WEBHOOK_SECRET_FILE`,
  `PR_HYGIENE_WEBHOOK_APP_ID` and `PR_HYGIENE_WEBHOOK_INSTALLATION_ID`.
- **Secrets:** the SSM parameters, the IAM statement and the secrets unit
  above.
- **nginx:** the location above.
- **Alerting:** on `/readyz` and on the new `/healthz/events`. It answers 503
  when:
  - the worker heartbeat is older than 5 min;
  - the oldest due entry is older than 10 min;
  - the replay has not succeeded for 30 min.

  It carries ages only.
- **Unchanged:** the deploy script (both services start from one digest)
  and the backup. The main database gains small queue and delivery tables,
  with no secrets.

**Increments (each a PR, in order):**
1. **Receiver** (`service`): the route and checks above, `webhook_deliveries`
   and `refresh_queue`, the secret files and the rotation pair, `serve`
   refusing reader settings, `/healthz/events`. Dormant without a secret.
2. **One PR in the analyzer** (`src/`, can run in parallel with 1):
   - the shared node fragment with `state` and `headRefOid`;
   - `Fetcher::fetch_pr`;
   - `dashboard::pr_part` and `counts_toward_wip` extracted, with the
     `build`-equality test;
   - `reads_from` in the snapshot, and the workflow's first step.
3. **Overlays** (`service`):
   - the `pr_overlays` table;
   - `view::apply_overlays`;
   - the version sequence;
   - stage changes from overlays;
   - pruning at ingest;
   - the oracle tests.
4. **Worker** (`service`):
   - the subcommand: lease, coalesce, head index and poll, policy commit;
   - the refresh: fetch, the engine's dry `Selection::Pr`, derivation,
     freshness;
   - the delivery replay, rate metering and the kill switch;
   - the mock-GitHub tests.

   Ships at stage B, with overlays private.
5. **Shadow** (`service`): 1b-6b's comparison table in the private database,
   the chunked sweep, overturn and drift counts, the pass summary.
6. **Cadence** (`pr-hygiene.yml`): hourly, then 6-hourly, on the evidence
   above.

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

Owner, 2026-11-21: GitHub events drive the service now, read-only, through a
GitHub App webhook (§6f). The service refreshes one PR per event and keeps a
full reconciliation as the safety net. The Rust engine runs in shadow on the
same events (1b-6b), and its clock restarts. Rejected: governed repositories
pushing per-PR state from their Actions. Open: §6f's decisions (whose
verdict the overlays show, the App, secrets).
