# pr-hygiene

This repository hosts two things:

1. **The shared PR review policy** — `policies/` (who owns and reviews what in
   Platform, rust-dashcore, Tenderdash, GroveDB and Dash Evo Tool), the Python
   evaluator `pr_review/` that those repositories call through
   `.github/workflows/pr-review-reusable.yml`, and the daily Slack digest. See
   [guides/PR_REVIEW_ARCHITECTURE.md](guides/PR_REVIEW_ARCHITECTURE.md) and
   [guides/PR_REVIEW_OPERATIONS.md](guides/PR_REVIEW_OPERATIONS.md).
2. **The PR-hygiene dashboard** described below.


Nightly PR-hygiene dashboard for every repository registered in
[`policies/repositories.json`](policies/repositories.json). Surfaces who has the
most open PRs with unresolved review feedback — CodeRabbit and human reviewers
both — so social pressure replaces 1:1 nagging, and shows the shared review
engine's verdict for each PR next to it.

The deliverable is a GitHub Pages site built from the generated `index.md` on
the `data` branch, regenerated and re-deployed every 6 hours. URL pattern:

```
https://<owner>.github.io/<repo>/
```

For this repo, after the first workflow run that's
`https://dashpay.github.io/stale_prs_are_bad/`.

## What it does

For every open PR in every registered repository:

1. Fetches PR metadata, review state, merge state, and every review thread via
   the GitHub GraphQL API.
2. Classifies each thread as **unresolved** when it is open, not outdated, has
   at least one non-author comment, and the last comment is from a reviewer.
3. Tags each unresolved thread as `coderabbit` / `bot` / `human` and assigns a
   severity (`high` / `medium` / `low`) — CodeRabbit severity comes from the
   `⚠️ / 🛠️ / 🧹 / 🔵` markers in the first comment, human threads default to
   `medium` and escalate to `high` when any reviewer has `CHANGES_REQUESTED`.
   Review bots whose threads block merge in the shared engine (`thepastaclaw`)
   are graded like CodeRabbit; other bots are `low`.
4. Scores each PR:
   `score = high*5 + medium*2 + low*0.5`, then `score *= max(1, ln(oldest+1))`.
5. Routes review duty from the repository's policy in `policies/`: the owners
   and reviewers of every area the changed files fall into (or the repository
   fallback for files no area claims), combined with GitHub's explicit review
   requests. Areas whose ownership the policy still lists as unresolved are
   flagged on the PR.
6. Joins the review engine's exported state (`waiting-bots`, `ready-for-human`,
   …) and its blockers onto each PR. A repository whose export is missing is
   reported as "engine state unavailable", never as clean.
7. Rolls up per author and ranks "top offenders".

Fairness guards (all in [`.pr-hygiene.yml`](.pr-hygiene.yml)):

- Bots and CI authors are excluded by default.
- PRs tagged `wip`, `blocked`, `do-not-merge`, `help-wanted` are skipped.
- Brand-new contributors (first PR < 14 days ago) are skipped — controlled by
  `grace_period_days`. Tenure is tracked locally in `.pr-hygiene/authors.json`.
- `count_nitpicks: false` (default) drops `low`-severity threads.
- `maintainer_only: true` filters out threads with no maintainer or CodeRabbit
  involvement (drive-by review noise).

## Quick start

```bash
cargo run --release -- --token "$GITHUB_TOKEN"
```

Useful flags:

- `--repo dashpay/platform` — analyze only this registered repository
- `--policies-root ./policies` — directory holding `repositories.json` and the policy files
- `--policy-state ./policy-state` — directory of review-engine exports (`<name>.json` per repository)
- `--config ./alt.yml` — alternate config path
- `--out docs/index.md` — output path
- `--dry-run` — skip writing files; print the report to stdout

`GITHUB_TOKEN` can also be supplied via the `--token` flag, or any other env var
your shell sets it from.

### Required token scopes

| Setting          | Scope                                           |
|------------------|-------------------------------------------------|
| Read public repo | `public_repo` (classic) or read PRs (fine-grained) |
| Read private repo| `repo` (classic) or read PRs (fine-grained)     |

The dashboard only reads. The review engine is the single writer per repository.

## GitHub Action + Pages

The included [`.github/workflows/pr-hygiene.yml`](.github/workflows/pr-hygiene.yml)
runs every 6 hours (00:00 / 06:00 / 12:00 / 18:00 UTC) and on `workflow_dispatch`. It has two jobs:

1. **`analyze`** — checks out `master` (code) and the `data` branch (generated
   output), builds the analyzer, exports the review engine's state for each
   registered repository with the read-only GitHub App token (skipped when
   `PR_REVIEW_APP_ID` is unset — the board then says "engine state unavailable"),
   runs the analyzer from inside the `data` checkout, and pushes `index.md` +
   `.pr-hygiene/` to `data`.
2. **`publish`** — copies `data/index.md` into `docs/`, builds the `docs/` folder
   with Jekyll, and deploys to GitHub Pages.

`master` holds only code and configuration and never receives bot commits, so it
can be branch-protected. The `data` branch is written only by the workflow; do
not open PRs against it.

### One-time setup

**One-time manual setup: seed the `data` branch.** The workflow checks out
`data` before anything else, so it must exist:

```bash
git checkout --orphan data
git rm -rf --cached .
printf '# PR Hygiene Report\n' > index.md
mkdir -p .pr-hygiene/history && echo '{}' > .pr-hygiene/authors.json
git add index.md .pr-hygiene && git commit -m "chore(pr-hygiene): seed data branch"
git push origin data
```

Pages itself needs no setup: the workflow auto-enables it on the first run via
`actions/configure-pages` with `enablement: true`. After the first successful
run, Settings → Pages will show "Your site is live at
`https://<owner>.github.io/<repo>/`".

Caveats:

- The repo must be public, **or** your account/org plan allows private Pages.
- If your org has Pages administratively disabled, the workflow can't override
  that — an admin needs to allow Pages first.

## Configuration reference

See [`.pr-hygiene.yml`](.pr-hygiene.yml) — every key has an inline comment.
The keys apply to every registered repository; which repositories exist and
who owns what in them is defined in `policies/`. Highlights:

| Key                       | Default          | Effect                                           |
|---------------------------|------------------|--------------------------------------------------|
| `grace_period_days`       | `14`             | Skip authors first seen within this window       |
| `count_nitpicks`          | `false`          | Whether `low`-severity threads count             |
| `maintainer_only`         | `false`          | Filter out drive-by review noise                 |
| `weights.{high,medium,low}` | `5/2/0.5`      | Severity weights for scoring                     |
| `age_multiplier`          | `ln`             | `ln`, `log10`, or `none`                         |
| `history_retention_days`  | `90`             | Daily snapshots older than this are pruned       |

Unknown keys are rejected with an error, so typos surface immediately.

## What gets committed each run

- `index.md` — the report, committed to the `data` branch (only when changed).
  The Jekyll theme config lives in `docs/_config.yml` on `master`.
- `.pr-hygiene/history/YYYY-MM-DD.json` — full snapshot for week-over-week deltas
- `.pr-hygiene/authors.json` — per-author "first seen" cache for grace periods

Old snapshots beyond `history_retention_days` are deleted in the same run. The
Pages publish job then redeploys whatever `docs/` looks like after the commit.

## Local development

```bash
cargo fmt
cargo clippy -- -D warnings
cargo test
# update insta snapshots after intentional renderer changes:
INSTA_UPDATE=always cargo test --test end_to_end
```

The fixture-based end-to-end test in [`tests/end_to_end.rs`](tests/end_to_end.rs)
exercises the full parse → analyze → route → score → render pipeline against
two fixture repositories ([`tests/fixtures/`](tests/fixtures/)), a temporary
policy registry and a review-engine export for only one of them, and snapshots
the output, so any regression in the renderer, routing or scoring shows up as a
diff.

## Project layout

```
src/
  config.rs    — YAML loading + defaults
  fetcher.rs   — GraphQL client, pagination, retry
  analyzer.rs  — thread classification, severity, grace period
  scorer.rs    — per-PR scoring, policy routing, per-author rollup, deltas
  policy.rs    — policy registry, area routing, review-engine state export
  history.rs   — snapshot persistence + pruning
  renderer.rs  — markdown report
  model.rs     — shared types
  main.rs      — CLI wiring
  lib.rs       — re-exports for integration tests
tests/
  end_to_end.rs
  fixtures/sample_prs.json
  fixtures/sample_prs_rust_dashcore.json
docs/
  _config.yml  — Jekyll theme + title for the Pages site
  index.md     — generated each run
.github/workflows/pr-hygiene.yml
.github/workflows/rust.yml
.pr-hygiene.yml
```
