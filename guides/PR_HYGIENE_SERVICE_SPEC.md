# Spec: PR Hygiene as one service with an API

Status: draft revision 4, 2026-10-02 — after simplicity, reliability/migration
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
