# pr-hygiene

Nightly PR-hygiene dashboard for a target GitHub repository (default:
[`dashpay/platform`](https://github.com/dashpay/platform)). Surfaces who has the
most open PRs with unresolved review feedback — CodeRabbit and human reviewers
both — so social pressure replaces 1:1 nagging.

The deliverable is a GitHub Pages site built from [`docs/index.md`](docs/index.md),
regenerated and re-deployed every 6 hours. URL pattern:

```
https://<owner>.github.io/<repo>/
```

For this repo, after the first workflow run that's
`https://dashpay.github.io/stale_prs_are_bad/`.

## What it does

For every open PR in the target repo:

1. Fetches PR metadata, review state, merge state, and every review thread via
   the GitHub GraphQL API.
2. Classifies each thread as **unresolved** when it is open, not outdated, has
   at least one non-author comment, and the last comment is from a reviewer.
3. Tags each unresolved thread as `coderabbit` / `bot` / `human` and assigns a
   severity (`high` / `medium` / `low`) — CodeRabbit severity comes from the
   `⚠️ / 🛠️ / 🧹 / 🔵` markers in the first comment, human threads default to
   `medium` and escalate to `high` when any reviewer has `CHANGES_REQUESTED`.
4. Scores each PR:
   `score = high*5 + medium*2 + low*0.5`, then `score *= max(1, ln(oldest+1))`.
5. Rolls up per author, ranks "top offenders", and (optionally) labels every PR
   that needs author action with `needs-author-action`.

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

- `--repo dashpay/platform` — override `target_repo` from config
- `--config ./alt.yml` — alternate config path
- `--out docs/index.md` — output path
- `--dry-run` — skip writing files and label mutations; print the report to stdout

`GITHUB_TOKEN` can also be supplied via the `--token` flag, or any other env var
your shell sets it from.

### Required token scopes

| Setting          | Scope                                           |
|------------------|-------------------------------------------------|
| Read public repo | `public_repo` (classic) or read PRs (fine-grained) |
| Read private repo| `repo` (classic) or read PRs (fine-grained)     |
| Label mutations  | `repo` (classic) or `pull-requests:write` (fine-grained) — **on the target repo, not on this repo** |

## GitHub Action + Pages

The included [`.github/workflows/pr-hygiene.yml`](.github/workflows/pr-hygiene.yml)
runs every 6 hours (00:00 / 06:00 / 12:00 / 18:00 UTC) and on `workflow_dispatch`. It has two jobs:

1. **`analyze`** — fetches PRs, generates the report, commits `docs/index.md`
   and `.pr-hygiene/` back to `master` with `[skip ci]`, and applies labels.
2. **`publish`** — checks out the fresh commit, builds the `docs/` folder with
   Jekyll, and deploys to GitHub Pages.

### One-time setup

**Zero manual setup needed.** The workflow auto-enables Pages on the first run
via `actions/configure-pages@v5` with `enablement: true`. After the first
successful run, Settings → Pages will show "Your site is live at
`https://<owner>.github.io/<repo>/`".

Caveats:

- The repo must be public, **or** your account/org plan allows private Pages.
- If your org has Pages administratively disabled, the workflow can't override
  that — an admin needs to allow Pages first.

> [!IMPORTANT]
> **Labeling PRs in another repo needs a PAT.** The workflow's default
> `GITHUB_TOKEN` is scoped to *this* repo only. It can read public PRs from
> `dashpay/platform` just fine, but **it cannot apply labels there**. To enable
> `auto_label`, create a PAT (or fine-grained token) with `pull-requests:write`
> on the target repo, and store it as `secrets.PR_HYGIENE_TOKEN`. The workflow
> picks it up automatically when set.

> [!NOTE]
> If your branch protection rules block the default token from pushing to
> `master`, you'll need the same `PR_HYGIENE_TOKEN` (with `contents:write`) or a
> deploy key with push access. The push step uses whichever token is configured.

## Configuration reference

See [`.pr-hygiene.yml`](.pr-hygiene.yml) — every key has an inline comment.
Highlights:

| Key                       | Default          | Effect                                           |
|---------------------------|------------------|--------------------------------------------------|
| `target_repo`             | `dashpay/platform` | Which repo to analyze                          |
| `grace_period_days`       | `14`             | Skip authors first seen within this window       |
| `count_nitpicks`          | `false`          | Whether `low`-severity threads count             |
| `maintainer_only`         | `false`          | Filter out drive-by review noise                 |
| `auto_label`              | `true`           | Apply/remove `needs-author-action`               |
| `weights.{high,medium,low}` | `5/2/0.5`      | Severity weights for scoring                     |
| `age_multiplier`          | `ln`             | `ln`, `log10`, or `none`                         |
| `history_retention_days`  | `90`             | Daily snapshots older than this are pruned       |

Unknown keys are rejected with an error, so typos surface immediately.

## What gets committed each run

- `docs/index.md` — the report (only re-committed when changed). Also lives at
  `docs/_config.yml` (Jekyll theme config, committed once).
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
exercises the full parse → analyze → score → render pipeline against
[`tests/fixtures/sample_prs.json`](tests/fixtures/sample_prs.json) and snapshots
the output, so any regression in the renderer or scoring shows up as a diff.

## Project layout

```
src/
  config.rs    — YAML loading + defaults
  fetcher.rs   — GraphQL client, pagination, retry
  analyzer.rs  — thread classification, severity, grace period
  scorer.rs    — per-PR scoring, per-author rollup, deltas
  history.rs   — snapshot persistence + pruning
  renderer.rs  — markdown report
  labeler.rs   — `needs-author-action` add/remove diff + REST mutations
  model.rs     — shared types
  main.rs      — CLI wiring
  lib.rs       — re-exports for integration tests
tests/
  end_to_end.rs
  fixtures/sample_prs.json
docs/
  _config.yml  — Jekyll theme + title for the Pages site
  index.md     — generated each run
.github/workflows/pr-hygiene.yml
.pr-hygiene.yml
```
