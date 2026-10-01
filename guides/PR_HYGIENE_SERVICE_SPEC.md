# Spec: PR Hygiene as one service with an API

Status: draft for review, 2026-10-02. Replaces the delivery parts of
`DASHBOARD_V2_SPEC.md` (static site, `data` branch); its product parts —
stages, "your part", humans vs bots, lateness — carry over unchanged.

## 1. Problem

PR Hygiene works, but it is four programs that each rebuild the same picture
of every PR, and keep what they learn in places that are not a database.

| Piece | Runs where | Reads GitHub | Keeps state in |
|---|---|---|---|
| Engine (Python, `pr_review/`) | GitHub Actions in each governed repo, per event + hourly sweep + 3×/h build scan, via a caller workflow pinned to an engine commit | yes, per PR | PR comments (a JSON record per PR, edited in place), labels, PR description checklist, commit statuses |
| Dashboard (Rust, `src/`) | Actions in this repo, every 6 h | yes, all PRs, plus a full engine export per repo | the `data` branch of this repo (snapshots, author cache, report), GitHub Pages |
| Slack digest (`aggregate`, `notifications`) | Actions in this repo, daily | yes, a full engine pass over every repo | nothing |
| `/prs` skill | the developer's machine: clones this repo, needs Python ≥ 3.10 | yes, a full engine pass, with the developer's token | nothing |

What that costs, all of it seen this week:

- **Version skew.** Each repo pins an engine commit but reads the policy live.
  Today's fixes do not reach platform until its caller is re-pinned (one PR
  per repo, every time), and a policy change can mean different things to
  different pinned engines — a `v*-dev` pattern alone would have made
  platform's old engine strip every PR's marks.
- **History lives in PR comments.** The engine's memory (admission, review
  cycle, state) is a JSON blob in a comment. When a PR leaves the policy the
  comment is deleted and its history with it (58 platform PRs lost their
  admission order this way). Reading "since when is this PR in its stage"
  needs scraping every comment's revision history (the whole of PR #68).
- **Data committed to a repo.** Dashboard snapshots and caches are bot
  commits on the `data` branch.
- **Four recomputations, four answers.** Dashboard (6 h old), Slack (daily),
  skill (live, ~75 s, needs a local Python and the developer's rate limit)
  and the engine itself can disagree about the same PR.
- **Logic in two languages.** `governs`, the bot list, "your part" and stage
  rules exist in Python and Rust, held together by parity tests.
- **No identity.** A static page cannot know who is looking, so "your speed,
  only to you" is impossible.

## 2. Proposal

One service, "PR Hygiene", installed as a **GitHub App** on the governed
repositories. It owns the whole loop:

```
GitHub ──webhooks──▶ ingest ──▶ engine.evaluate() ──▶ GitHub writes (status, labels,
   ▲                    │                              checklist, comments, review requests)
   │   reconcile sweep  ▼
   └──────────────── store (one DB: PRs, evaluations, transitions, people)
                        │
                     read API  ──▶ web UI (team / people / me)
                        │      ──▶ /prs skill (curl + gh token)
                        │      ──▶ Slack digest (scheduled inside the service)
                        │      ──▶ anything else (release tooling, Actions)
```

- **Engine unchanged as logic.** `policy.evaluate()` (≈1 000 lines, 420
  tests) is already a function of evidence. The service calls it; what
  changes is where evidence and memory come from (the DB, not record
  comments) and who runs it (the service, not each repo's Actions).
- **One store.** Every evaluation is appended (`pr, head, state, reviewers,
  approvals, objectors, blockers, at`); a transition is a change of stage.
  "Since when", timelines, speed per person and per area, and the five-slot
  admission order are queries, recorded as they happen — no archaeology.
  SQLite on a persistent volume is enough (hundreds of PRs, tens of
  thousands of rows a year); Postgres if infra prefers a managed one.
- **Policies stay in git** (this repo). Config as code with review and CI is
  right; the service reloads a policy on push to the default branch. One
  engine version, one policy version, everywhere, at once.
- **API** (JSON, versioned `/api/v1`):
  - `GET /prs` (filters: repo, area, stage, author, reviewer, late) — the
    board; `GET /prs/{owner}/{repo}/{n}` — one PR with its stage history.
  - `GET /people`, `GET /people/{login}` — what someone owes and holds.
  - `GET /me` — the caller's own queue and **their speed** (review wait
    given, self-review turnaround, cycle time; monthly, a year back).
  - `GET /digest/{login}` — the text the Slack DM and the skill both show.
  - `POST /webhooks/github` — signed by GitHub.
- **Identity.** The same GitHub App signs users in ("Sign in with GitHub",
  user-to-server flow): the web UI gets an HttpOnly session cookie on the
  service's own domain; the skill and scripts send
  `Authorization: Bearer $(gh auth token)`, which the service checks once
  against GitHub's `/user` and caches by hash — it never stores tokens.
  Speed is served only to its owner. Team and people views: public or
  org-members-only (owner's choice).
- **Web UI.** The page built in #67 (Team, People, Me) served by the service
  from its own origin, reading the API instead of `dashboard.json`.
- **Skill.** `/prs` becomes `curl -sH "Authorization: Bearer $(gh auth token)"
  https://<host>/api/v1/me` — no clone, no Python, no rate-limit cost, same
  answer as the page and Slack.
- **Language: Python** — the engine is the bulk of the domain logic and is
  Python. The Rust analyzer is retired; what it uniquely computes (stage
  mapping, lateness, the dashboard contract) is small and is ported.

## 3. What goes away

Per-repo caller workflows and engine pins (and every re-pin PR), the
reusable workflow, record comments as storage (a human-readable move comment
can stay), the `data` branch, GitHub Pages, the Rust analyzer, the 6-hourly
and daily recomputations, the skill's local Python, the Python/Rust parity
tests.

## 4. Requirements for infra

- One always-on container (HTTP: webhooks, API, UI) with a scheduler (sweep
  every ~15 min, daily digest), outbound HTTPS to GitHub and Slack.
- A persistent volume (SQLite) or a small managed Postgres; daily backup.
- HTTPS on a dash.org subdomain (e.g. `hygiene.dash.org`).
- Secrets: GitHub App private key, webhook secret, App client secret (user
  sign-in), session signing key, Slack bot token.
- Availability: the `PR Hygiene` status is a **required check** — while the
  service is down, PRs keep their last status and cannot turn green. Same
  failure mode as an Actions outage today; the sweep repairs missed events
  on recovery. Target: restart on crash, alert on webhook failures.

## 5. Alternatives

- **A. Keep the engine in Actions; add only a store + API.** Each run POSTs
  its evaluation to the service (authenticated with the Actions OIDC token,
  no secrets); readers use the API. Smaller first step, no webhook ingest,
  merge-critical path stays on GitHub's runners. Keeps pins, version skew,
  per-repo workflows and two runtimes. Reasonable if infra can host a small
  API but not a merge-critical one; a waypoint towards the proposal, not an
  end state.
- **B. Static site + Slack DM for private speed** (the previous plan). No
  infra, but data stays in the repo, four recomputations stay, and "only
  you" stays impossible on the web.
- **C. Cloudflare Worker in front of the static site.** Gives login, not a
  single source of truth; the engine and its storage stay as they are.
- **D. Rewrite the engine in Rust.** Most risk for no user-visible gain.

## 6. Migration — one repo at a time, always reversible

1. **Shadow.** Deploy the service read-only on GitHub: webhooks + sweep,
   evaluate with the engine, store, serve API + UI. Backfill history once
   from the existing record comments and GitHub events (the logic of #68).
   Switch the `/prs` skill and the Slack digest to the API.
2. **Compare.** A daily diff of service verdicts vs the Actions engine's
   statuses per PR; zero unexplained differences for a week.
3. **Cut over per repo** (grovedb → dash-evo-tool → tenderdash →
   rust-dashcore → platform): enable writes for the repo in the service,
   remove its caller workflow (one last PR), and where branch protection
   pins the check to GitHub Actions (tenderdash, grovedb, dash-evo-tool
   require `PR Hygiene` from app 15368) switch it to the App. Platform and
   rust-dashcore accept any source. Rollback = restore the caller workflow.
4. **Retire** the reusable workflow, the Rust analyzer, the `data` branch
   and Pages.

## 7. What happens to the open PRs

- **#65** (target-branch patterns): independent and needed now — 58
  platform PRs are ungoverned. Merge.
- **Re-pin callers**: still needed until cut-over (to get #62, #64, #65).
- **#66/#67/#68** (Rust data, static site, comment archaeology): built for
  the static architecture. Pause. Their product decisions (stages, your
  part, parked work, lateness, bots apart), the page itself, and the
  backfill logic carry into the service. The three live-bug fixes in #66
  (bot accounts named `[bot]`, bots not merged into people, retry on a
  truncated response) can ship alone for the current board.

## 8. Risks

- Merge-critical availability moves from GitHub to us (§4).
- The engine's orchestration (`main.collect` / `publish`) reads its memory
  from record comments; moving that memory to the DB is a real refactor of
  `main.py`/`github.py`, with the engine's own tests as the guard.
- Webhooks are at-least-once and unordered; every handler re-reads the PR
  and evaluates idempotently, as the sweep already does.
- App permissions are broad (statuses, issues, pull requests: write) on five
  repos; the private key is the crown jewel.

## 9. Decisions for the owner

1. Go with the service (§2), or the Actions + API waypoint (§5 A) first?
2. Team/People views public, or dashpay org members only?
3. Hosting (infra team): container + volume or managed Postgres, subdomain.
4. Open PRs per §7.
