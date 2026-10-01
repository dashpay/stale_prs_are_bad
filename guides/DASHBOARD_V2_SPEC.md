# Spec: PR Hygiene dashboard v2 — flow, people, and "my queue"

Status: draft, revision 3 (after feasibility, product, engine-domain and
security reviews, and the owner's answers of 2026-10-01).

## 1. Goal

Make PRs move faster: show **where we are late and whose move it is**, so
people review others' PRs sooner, finish their own sooner, and keep fewer
open. Every view answers "what is late, and whose move is it?" — counts
without time do not.

Three levels: **Team** (all PRs, flow, trends), **People** (every human and
bot: what they owe, what they hold), **Me** (one person's actionable list:
reviews owed *with their areas*, own PRs with next action and timeline).

## 2. Facts this design rests on (verified)

- The review engine (`pr_review/`) is the source of truth for PR state. Its
  JSON export (`report --format json`, `aggregate`) is the full `evaluate`
  result per PR: `state`, `blockers`, `reviewers`, `ready_since`, `areas`,
  `approvals` (`{area, files, approvers, approved_by, owned}` per touched
  area), `objections` (free-text lines), `checklist`. `approvals` and
  `objections` are filled only once the bots and self-review gates pass.
  It covers only open PRs on the policy's `target_branches`; a PR whose
  evidence read fails is dropped with a stderr line.
- `reviewers` (only in `ready-for-human`) = everyone eligible for any area
  lacking a current-head approval, plus objectors. Any one approval clears
  an area. It can be empty when an area is *stranded* (everyone eligible
  attested instead). Spelling follows the policy for roster members and is
  lowercased for objectors outside it.
- Exact engine state history exists from ≈2026-09-14: the engine's record
  comment carries `platform-pr-review-state-v1 {"state":…,"ready_since":…}`
  and every rewrite is kept in `IssueComment.userContentEdits`. State labels
  (one per state only since 2026-09-20, renamed again 09-23) are lossy:
  `waiting-author` shares a label with `waiting-self-review`; draft,
  `waiting-build`, `ready-to-merge`, `configuration-error` and "not
  governed" carry none; humans sometimes edit them; they survive merge.
- 48 of 82 non-draft open platform PRs target `v5.1-dev`/`v6.0-dev`, outside
  `target_branches`: the engine never evaluates them.
- Bot identity: GitHub `Bot` accounts / `[bot]` logins; each policy's
  `bot_authors` (`infraclaw-dash`, `DCG-Claude`, `Claudius-Maginificent`);
  the engine's hard-coded review bots (`thepastaclaw`, `coderabbitai`).
  `thepastaclaw` is a GitHub `User`. The dashboard wrongly merges
  `thepastaclaw` into `PastaPastaPasta` and `Claudius-Maginificent` into
  `lklimek` via `author_aliases`.
- Engine bug (found in review, fixed separately in
  `fix/review-bots-are-machine-authors`): a PR opened by `thepastaclaw` was
  treated as human-authored, waiting for a `/self-reviewed` the engine
  ignores from `BOTS` and spending a slot.
- Live security bug (fixed separately in
  `fix/report-escapes-html-and-liquid`): the Markdown report passed HTML and
  Liquid from PR titles, blockers and thread excerpts to the published page.
- The `data` branch is unprotected. All repos and the Pages site are public;
  `dashpay.github.io` is shared with ~16 other dashpay Pages sites (shared
  origin and `localStorage`).
- GraphQL cost is not the constraint (50 PRs × 50 timeline items = 1 point).
  The engine export's REST calls per governed PR are; unmeasured.

## 3. Design

### 3.1 Stages — whose move it is

| Stage | From | Owner | Late? |
|---|---|---|---|
| Draft | GitHub draft | author | idle > N days flagged; counted in open totals |
| Not governed | base ∉ `target_branches` | author/maintainers | idle flag only |
| Bots | `waiting-bots` | bots | yes |
| Self-review | `waiting-author`, `waiting-self-review`, `waiting-build` with a failed build | author | yes; card shows the next action (post `/self-reviewed`, fix the build, answer @X) |
| CI | `waiting-build`, build running | nobody | only if stuck past threshold |
| Queued | `too-many-open-prs` | author's other PRs | never per PR; WIP warning on the person |
| Review | `ready-for-human` | reviewers | yes |
| Mergeable | `ready-to-merge` | author/maintainer | yes |
| Blocked | `configuration-error` | maintainers | yes; excluded from author metrics |
| Done | merged / closed | — | — |
| Unknown | no evidence (export missing, read failed) | — | shown hatched; never guessed |

Build failed vs running is read from the export's structured checklist
build state, not blocker text. Unknown engine states map to Unknown and a
Rust test enumerates the known ones.

### 3.2 Review responsibility with areas

Implemented in the engine as phase A (dashpay/stale_prs_are_bad#64); the
dashboard reuses the same rule. For person P (all of P's logins) and an
engine row in `ready-for-human`, P's part is:

- each `approvals` entry not `owned`, with empty `approved_by` and P in
  `approvers` (case-insensitive), shown as "`dpp` (you or Carol)" — any one
  listed person's approval clears the area;
- "re-review or resolve your objection" when P is in the exported
  `objectors` (an open objection may be a changes request or an unresolved
  thread, so resolving can be the fix);
- an area nobody may approve is shown as such on the PR ("`drive`: nobody
  may approve"), never as a met ask.

Built from all engine rows, not only PRs the dashboard keeps. Wording now on
every surface, areas in the order the engine first touches them (fixed, not
hash-dependent):

| Surface | Wording |
|---|---|
| PR move comment | "Ready for review — files with no dedicated owner: QE or shumkov · `github`: ktechmidas or shumkov · re-review or resolve: lklimek." |
| PR checklist | per area; an area nobody may approve says so |
| Slack channel / DM | "Needs: dpp: A or B · drive: C" / "Your part: dpp (you or Carol)" per Slack identity |
| `report` / `aggregate report --user` (`/prs` skill) | "needs …; your part: …"; skill text updated in dashpay/platform#5242 |
| Dashboard (phase B) | same rule from `approvals` + `objectors` |

Still to add for the dashboard (phase B, when it has a consumer):
`unreadable: [{number, error}]` for PRs dropped on read failure, and the
engine's bot list / per-row `author_kind`.

### 3.3 People: humans and bots, never merged

`kind`: `human` or `bot` with roles `review-bot` / `author-bot` (both
possible), from the engine's exported bot list ∪ all policies'
`bot_authors` ∪ GitHub `Bot` type. `author_aliases` is removed (config key
rejected with a pointer to this change). Bots get a 🤖 badge, a separate
table, and are excluded from team KPIs by default (toggle to include).
Bots are shown on their own; no "operated by" link to a human (owner's
decision).

### 3.4 Metrics

- **Whose move** (headline): open PRs by stage owner, humans' PRs by default.
- **Time in current stage** for every open PR. Entry time from the newest of:
  last record-comment edit changing state, last engine-actor label event,
  draft/ready event, base change, last approval; else PR age, labelled
  "since opened".
- **Review wait**: per Review interval, from entry to the first *decisive*
  review (approve, changes requested, objection thread) by an eligible
  person — COMMENTED does not end it. Counted per PR/area, not charged to
  every listed approver. Per person: only reviews they actually gave.
  For history before record comments, eligible = requested reviewers at
  entry (stated proxy).
- **Self-review turnaround** per Self-review interval; **cycle time** first non-draft →
  merged; **throughput** and **opened vs closed per week**; **WIP** vs the
  5-per-repo slot limit (person-level).
- Medians always show n. Lateness thresholds are configurable in
  `.pr-hygiene.yml` (defaults: Review 1d/3d, Self-review 3d/7d, Bots 1d/2d,
  Mergeable 1d/3d, CI 6h, idle draft/not-governed 14d). Calendar days.
- Thread data (unresolved comments, CI failing, conflict) stays as card
  badges and People columns; the thread score no longer ranks people.

### 3.5 Views (static app)

Shared filter bar in the URL: repo, area, author, reviewer, human/bot, stage,
late only.

- **Me / Person** (`#/me`, `#/people/<login>`): reviews owed (PR, author,
  your areas + shared-with, waiting, lateness); your PRs with stage, time in
  stage, next action, blockers; your WIP per repo vs 5; kind and areas owned
  per repo (from policies). Phase C adds the per-PR timeline strip and,
  for the signed-in person only, their own speed trends (3.8).
- **Team**: "whose move" tiles; **aging plot** — y = stage, x = time in
  stage, dot per PR, colour = lateness (the "status y, time x" view);
  sortable PR list with the same filters; "close or revive" list (idle
  drafts / not governed / conflicting). Phase C adds flow-over-time
  (stacked area) and opened vs closed. A kanban board is deferred: the aging
  plot shows the same grouping plus time.
- **People**: humans table, then bots; current-state columns (owed count,
  oldest owed, own PRs per stage, WIP, unresolved comments). Default sort:
  name. Lateness colour is on PRs, not people. No speed columns: speed is
  private (3.8).
- **Timeline** (phase C): Gantt of PRs, coloured stage segments, review ticks;
  for filtered sets (one person, one area, late only) — not all 500 at once.

### 3.6 Delivery

- Rust computes everything with meaning (stages, lateness, owed reviews,
  metrics) and writes `dashboard.json` (`schema_version`, `generated_at`).
  Plain `docs/index.html` + ES modules, no build step, render only.
- Observable Plot and d3 are **vendored** in `docs/vendor/` (versions and
  checksums recorded); no CDN.
- Publish job assembles a fresh directory (app + `dashboard.json`) and
  deploys it without Jekyll. The Markdown report is retired when the app
  ships (phase B); until then it gets the escaping fix (phase 0).
- `data` branch holds only history: `states/<repo>.json` and the merged-PR
  timeline cache, with a cache schema version, validation on read and a
  `--rebuild-cache` switch. `dashboard.json` is built in the analyze job and
  handed to publish as an artifact, so runs without history changes add no
  commit. Cadence stays 6h until the engine export's REST budget is
  measured in CI; hourly only if it fits.
- A stale-data banner when `generated_at` is older than 2× cadence.

### 3.7 Security

- Data rendered only via `textContent` / Plot text marks; CI greps `docs/*.js`
  to forbid `innerHTML`, `outerHTML`, `insertAdjacentHTML`, `document.write`.
- Links built from an allowlisted repo + integer PR number; avatars from
  `https://avatars.githubusercontent.com/<login>` with a validated login.
- Meta CSP: `default-src 'none'; script-src 'self'; style-src 'self'
  'unsafe-inline'; img-src 'self' https://avatars.githubusercontent.com;
  connect-src 'self'; base-uri 'none'; object-src 'none'; form-action
  'none'`. No `frame-ancestors` via meta — acceptable because the page has
  no actions.
- "Me" in `localStorage` under a namespaced key; stored and URL logins
  validated against `^[A-Za-z0-9-]{1,39}$` and the people list; no tokens
  ever (shared origin). This is also why sign-in is not done.
- `<meta name="robots" content="noindex">`. Per-person speed metrics are never
  written to `dashboard.json` or the `data` branch (both public); team
  metrics are aggregates only.
- `data` branch ruleset: no force-push, no deletion (admin action).

### 3.8 Identity and private speed

Two kinds of personal data, handled differently:

- **What you owe and hold** (reviews owed with areas, your PRs' stages and
  blockers) is public on GitHub already. Public site, a searchable "Who are
  you?" picker sets `#/me`; every person page is linkable and the Slack
  digest deep-links each person to theirs. No sign-in needed (phase B).
- **How fast you are** (review wait you gave, self-review turnaround, cycle
  time of your PRs) is shown **only to you**, as a monthly trend over the
  last year (owner's requirement). The raw timestamps are public on GitHub,
  so "private" means the dashboard never publishes or compares them — it
  needs real identity, which a static GitHub Pages site cannot provide
  (GitHub's token endpoint has no CORS and needs the client secret even with
  PKCE; a token on the shared `dashpay.github.io` origin would be readable by
  the other dashpay sites). History depth: review response and cycle time
  from GitHub events go back a year or more; self-review turnaround only
  from engine rollout (≈2026-09-14).

Options for the private part (decide before phase C):

| Option | How | Cost |
|---|---|---|
| **P1. Slack DM** | Monthly personal trend in the existing private per-person Slack DM (already identity-verified) | No new infra; text/emoji sparkline, not an interactive chart |
| **P2. Cloudflare Access** | A small Cloudflare-hosted "Me" page behind Access with GitHub as identity provider (free ≤50 users); a Pages Function serves only the caller's metrics file from private storage, written by the workflow | dash.org Cloudflare account + subdomain + secrets; no OAuth code of our own |
| **P3. Own OAuth** | GitHub OAuth App + a small Worker for the code exchange and a session cookie on its own domain; same private storage | Most code to write and secure |

Recommendation: P1 first (cheap, private by default); P2 if an interactive
personal chart is wanted. P3 rejected: P2 gives the same result without
custom auth code.

## 4. Phases (each its own PR, reviewed separately)

0. **Security fix now** — escape HTML and Liquid in the Markdown report
   (regression test first). Ask an admin to protect `data`.
1. **A — Engine** (first: people act on PR comments and Slack more than on
   dashboards): structured export additions (3.2); per-area / re-review /
   stranded wording in move comment, checklist, Slack DM, `aggregate
   report`; Slack deep link to the person's page. Companion `/prs` skill PR
   in `dashpay/platform`. The engine fix for review bots that open PRs
   ships separately (approved).
2. **B — Dashboard foundation**: bot/human classification, aliases removed,
   engine-based queue with areas, stages + time-in-stage for open PRs,
   `dashboard.json`, static app (Me/Person, Team aging plot + list + close-or-
   revive, People current-state), security measures, Markdown retired.
3. **C — History**: record-comment edit history (≈09-14 on) + engine-actor
   label events, base changes, cut at merge/close; merged-PR cache; full 90
   days for cycle time / throughput / first review; team metrics with n;
   Timeline view, flow-over-time, opened vs closed, per-PR strips; cadence
   decision; private personal speed trends via the option chosen in 3.8.

## 5. Alternatives rejected

- **Markdown + Mermaid** — theme doesn't render it; no filters.
- **React/Vite SPA** — npm build in a Rust/Python repo for a few views.
- **Grafana / BI** — needs DB and hosting for a few MB of data.
- **GitHub Projects board** — kanban only, lossy labels, no metrics.
- **States from labels only** — wrong in gaps and edited by hand; record
  comment history is exact.
- **Sample engine state each run as the history source** — coarser than
  record edits and gone for the past.
- **Sign-in for the whole site** — what you owe and hold is public; only
  speed is private (3.8).
- **Keep the heuristic queue beside the engine's** — two disagreeing lists.

## 6. Failure modes

| Case | Behaviour |
|---|---|
| Export missing for a repo | Its PRs → Unknown; banner; KPI deltas marked partial; history records a *gap*, not a state change (clocks don't reset) |
| Engine drops a PR on read failure | Listed in `unreadable` → Unknown, never "nothing owed" |
| Repo fetch fails | Banner; cache for that repo not pruned; current state not reused |
| Base outside target branches | Not governed stage, not Unknown |
| Labels edited by a human / left after merge | Only engine-actor events used; intervals cut at merge/close |
| Closed PR reopened | Only merged timelines are cached as immutable |
| Corrupt or old cache | Schema version + validation; `--rebuild-cache` |
| Rate limit | Fetcher waits for reset; job timeout fails the run; previous site stays |
| Stale site (schedule disabled) | Banner from `generated_at` |
| JSON schema mismatch | Page says so instead of rendering wrong data |
| Hostile text in data | Text-only rendering, CSP, built URLs |

## 7. Verification

- Rust unit tests (pure functions in `src/stages.rs`, fetch in
  `src/timeline.rs`): classification incl. dual-role bots; owed reviews incl.
  shared, re-review, stranded, PR absent from fetched set, v4.3-dev case;
  stage derivation from event sequences (record edits vs labels, human label
  edits ignored, gaps, base change, cut at merge); each metric on hand-built
  timelines with n; lateness thresholds; Queued never late.
- Insta snapshot of `dashboard.json` from extended fixtures.
- Python tests for the export additions and wording (update pinned wording in
  `pr_review/tests/test_checklist.py`).
- Phase 0: test with a `<img onerror>` / `{{ }}` title fails before the fix.
- Browser: serve the assembled site locally with a live dry-run JSON;
  screenshot each view at desktop and phone width; injected-title check;
  CSP violations none in console.

## 8. Decisions

Owner, 2026-10-01: engine fix for review bots approved (merge as admin);
no "operated by" on bot rows; per-person speed visible only to the person
as a month/year trend; start with the security fix, then phase A; the
Author stage is called Self-review.

Open:

1. Private speed delivery: P1 Slack DM, P2 Cloudflare Access, or both
   (3.8) — needed before phase C.
2. Lateness defaults in 3.4 OK? Retire the Markdown report when phase B
   ships?
