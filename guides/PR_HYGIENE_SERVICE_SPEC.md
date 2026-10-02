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
    Python version is pinned.
  - **Shadow** compares twice: Rust's engine on Python's evidence (exact), and
    Rust's evidence reader against Python's (separately), per (PR, head).
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
  `engine` (pure; `Clock`, `compat`), `github` (reqwest client, App auth,
  the port of `github.py`, a `trait Api` with a replay fake), `reconcile`
  (collect/publish), `store`, `service` (front, worker, scheduler),
  `conformance`. Reused from `src/`: `dashboard.rs`, `stages.rs` (backfill),
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
  platform and rust-dashcore). After cut-over the engine trusts only the
  App's own `<slug>[bot]` identity.
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
makes the same writes as the Python engine. It must prove this on the corpus
and then live in shadow, before it writes anything. 1b-1 (the recorder,
#82) is merged: `conformance/README.md` is the contract this section relies
on.

**Where it lives.** A new workspace member `engine/` (crate
`pr-hygiene-engine`), a library with four layers. Each layer depends only on
the ones above it:

1. `pycompat`: the Python behaviours the README lists, as small functions
   with golden tests.
   - `splitlines`, and `strip`/`split()` over Python's `isspace` set. That set
     includes `\x1c`–`\x1f`, which Rust's `char::is_whitespace` and regex
     `\s` do not.
   - Code-point slicing.
   - A `PyDateTime` that parses what Python 3.12's `fromisoformat` accepts
     (after `Z` → `+00:00`) and re-renders the way `isoformat()` does, keeping
     the offset and microseconds.
   - One JSON writer, `py_dumps(value, sort_keys, separators)`, covering the
     four shapes the engine hashes or writes: the marker writer, `diff_print`,
     the admission context and the sort key in `fingerprint`. It always uses
     `ensure_ascii`, lowercase `\uXXXX` and surrogate pairs.
   - A regex rule: every pattern is ported with `\s` replaced by Python's
     whitespace class. A non-multiline `$` is replaced by an explicit
     `\n?\z` only where the match span is not used. Each pattern gets a
     table test of inputs, with Python's matches recorded.
   - Production Python is 3.12 (the `ubuntu-latest` runner's `python3`), so
     3.12 is the reference.
2. `policy`: the policy types and `evaluate`, a pure function.
   - Policy types: serde, with `IndexMap` wherever Python iterates or
     serialises a dict.
   - `validate_policy`, `governs`, `evaluate` and its helpers: receipts,
     attestations, bot schedules, fingerprints.
   - No I/O and no clock: `now` is an argument.
3. `evidence`: what `github.py` reads, behind a transport.
   - `Transport::attempt(call) -> Attempt { exit_ok, stdout, stderr }`, where
     `call` is the same argument shape `gh api` receives: method, path,
     `--paginate --slurp`, stdin.
   - Above it, a `Client` keeps `_run`'s rules: one retry for idempotent
     calls on the same failure substrings, a partial GraphQL answer being an
     answer, empty output being `None`, invalid JSON being an error.
   - The snapshot builder, the access/status/build caches and the batch
     history reader sit on top.
   - Production uses an HTTP transport (an App installation token). It
     renders what `gh` would have printed: the slurped page array, and
     `gh`'s error line with `(HTTP <code>)`. One `Client` then serves HTTP,
     replay and the fake.
4. `reconcile`: what `main.py` does.
   - `collect`, `publish`, `clear_marks`, `nudge`, the checklist, the move
     comment, labels and statuses, and the report.
   - The clock is a `Clock` the caller passes in. It is read at the same
     points `utc_now` is read in Python.

The analyzer's `src/policy.rs` (registry, routing) overlaps layer 2. The
engine's types become the one definition, and the analyzer moves onto them
after 1b-2 in its own PR. Until then the two are not merged.

**Increments.** Each is its own PR and green before the next starts.
- **1b-2 `pycompat` + `policy`.**
  - All 274 committed evaluate cases must match, field for field. Blockers
    marked `python_exception_text` are compared by state only.
  - `pycompat` golden files are produced by Python into
    `conformance/pycompat/`, from an input list that includes every edge
    character above.
  - The Python tests of the pure helpers (`test_policy.py`'s non-evaluate
    tests, `test_bot_timeouts.py`) are ported as Rust tests.
- **1b-3 `evidence`.**
  - For every boundary recording, each snapshot the Rust reader builds
    equals the `pr` Python's `evaluate` saw (`evaluations.jsonl`). It is
    served only from recorded reads; a read the recording lacks is a failure.
  - The `test_github.py` tests are ported against a stateful **fake
    transport** in Rust. It answers from a scenario and applies writes to it.
- **1b-4 `reconcile`.**
  - Every recording gives the same verdicts, the same ordered writes (method,
    path, body), the same canonical outputs and the same printed report.
  - The fake is used to show three things the corpus cannot: replaying the
    state after the writes yields no writes, failing writes take the Python
    failure paths, and a first admission completes.
  - `test_main.py` and `test_checklist.py` are ported.
- **1b-5 shadow.**
  - The service's worker runs the Rust engine on GitHub through a transport
    that refuses every write: the recorder's default-deny classifier, ported.
  - It stores, per (repo, PR, head), the verdict and the writes it would
    have made, and compares them with the Python engine's posted status and
    record.
  - Differences fall into categories defined in advance: clock-dependent
    timeouts, telemetry, admission time, and the deliberate divergences
    below. Any other difference is a bug in the port.
  - Step 2's entry criterion: three to four weeks with no uncategorised
    difference, per repo.

**Corpus in CI.**
- **Committed:**
  - the evaluate cases;
  - the `pycompat` goldens.
- **CI keeps the corpus fresh.** The Python test job re-harvests both and
  fails on any difference. A change to `pr_review/` therefore arrives with its
  new corpus, and the Rust tests fail until the port follows.
- **Live recordings stay local.**
  - They are 85 MB, and the collaborator listing in them names collaborators
    and private organisation members.
  - `cargo test -p pr-hygiene-engine --test live -- --ignored` replays
    whatever `conformance/live/` holds and skips when it is absent.
  - A port PR states that it ran against a fresh recording.
- **Rejected:**
  - Committing redacted recordings: size, churn, and the collaborator list.
  - Pseudonymising them: logins are threaded through free text, and
    timestamps re-identify people anyway.
  - A Python fake GitHub to produce synthetic recordings: the fake has to
    live on after Python is retired, so it is written once, in Rust.

**Concurrency.** Python reads four snapshots at once, and its shared caches
race (README, "Not to be copied"). The port reads sequentially in 1b. Any
later concurrency fills each cache once before snapshots read it. The service
reconciles per (repo, author), not as whole sweeps, so sequential reads cost
nothing on its normal path.

**Deliberate divergences** are listed here, and each is a shadow category:
- The access race is not reproduced.
- A record comment containing `NaN`/`Infinity` (only an admin edit can put
  one there) is unreadable to the port; Python reads it.

Everything else is copied, including quirks: review-thread authors without
`[bot]`, and timestamps compared as strings. Fixes come after cut-over, made
once, in Rust.

**Failure modes.**
- The corpus can't tell when the clock is read. The port mirrors Python's
  read points, and a reviewer checks them against `utc_now`'s call sites.
- A read the port needs but Python never made fails replay loudly, so the
  port can't quietly read more than the engine did.
- A Python change that skips re-harvesting fails CI. A Python change that
  alters the boundary is caught by the next local live replay, and in shadow
  by its category.
- An HTTP transport that renders errors differently from `gh` would change
  only the retry decision. The transport's tests pin the rendered line for
  502/503/504, timeout and truncation.

**Verification.**
- `cargo test --workspace` runs:
  - the evaluate corpus;
  - the goldens;
  - the ported tests;
  - the fake's post-write, failed-write and first-admission scenarios.
- Each increment runs the local live replay and reports it in its PR.
- 1b-5 is judged on the shadow diff.

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
