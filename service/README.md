# PR Hygiene service

A read-only web service. The scheduled `pr-hygiene.yml` workflow in this
repository posts the analyzer's `dashboard.json` to it with a GitHub Actions
OIDC token; the service keeps each repository's last good data in SQLite and
serves it, publicly, as JSON and as a plain-text digest.

It holds **no GitHub credential** and **never calls GitHub**, except to fetch
GitHub's public token-signing keys from
`https://token.actions.githubusercontent.com/.well-known/jwks`.

## What it never does

- Write anything to GitHub, or read anything from it but those public keys.
- Accept data from anything but the one workflow, on `master`, on a
  GitHub-hosted runner, on the first attempt of a scheduled or manual run.
- Look anything up for a public request: every answer comes from the
  database; an unknown login or PR is a 404.
- Set a cookie, or log an `Authorization` header, a cookie or a client address.

## Running it

| | |
|---|---|
| Image | `docker build -f service/Dockerfile -t pr-hygiene-service .` (from the repository root) |
| Process | `pr-hygiene-service serve`, as uid 65532 |
| Port | 8080, plain HTTP; terminate TLS in front of it |
| Volume | `/data`, holding `pr-hygiene.sqlite3` and its `-wal`/`-shm` files. Must be a **local** volume: SQLite's WAL mode needs real file locks, so not NFS or SMB. The service refuses to start if WAL cannot be enabled. |
| Outbound | HTTPS to `token.actions.githubusercontent.com` only |
| Inbound | `POST /ingest` from GitHub Actions runners; `GET /api/v1/*` from anyone |

### Environment

| Variable | Default | Meaning |
|---|---|---|
| `PR_HYGIENE_OIDC_AUDIENCE` | — (required) | The audience the post job asks GitHub for; use the service's public URL, e.g. `https://pr-hygiene.dash.org`. |
| `PR_HYGIENE_BIND` | `0.0.0.0:8080` | Listen address. |
| `PR_HYGIENE_DB` | `/data/pr-hygiene.sqlite3` | Database file. |
| `PR_HYGIENE_REPOSITORY_ID` | `1242761300` | Numeric id of the repository allowed to post (`dashpay/stale_prs_are_bad`). |
| `PR_HYGIENE_REPOSITORY_OWNER_ID` | `11511719` | Numeric id of its owner (`dashpay`). |
| `PR_HYGIENE_REF` | `refs/heads/master` | The only branch a posting run may run on. |
| `PR_HYGIENE_WORKFLOW_REF` | `dashpay/stale_prs_are_bad/.github/workflows/pr-hygiene.yml@refs/heads/master` | The only workflow (and job workflow) allowed to post. |
| `PR_HYGIENE_BODY_LIMIT` | `1048576` | Largest snapshot accepted, in bytes. Five repositories with 139 open PRs measured 109 KB. |
| `PR_HYGIENE_JOB_TIMEOUT_SECS` | `1800` | How long before the token was minted the snapshot may have been generated: the analyze job's `timeout-minutes` (20) plus the post job's start-up. 60 to 86400. |
| `RUST_LOG` | `info` | Log filter. |

### In front of it

The service limits each request to 30 seconds and each snapshot to the body
limit, and nothing else. The reverse proxy that terminates TLS must also:

- time out slow and idle connections (request headers within about 10 s);
- cap connections, and rate-limit `/api/v1/*` per client;
- pass `POST /ingest` through unbuffered or with a body limit no larger
  than the service's, and never log its `Authorization` header.

### Health

- `GET /healthz` — 200 while the process runs (liveness).
- `GET /readyz` — 200 when the database answers and GitHub's signing keys
  are loaded; 503 otherwise, with `{"ready", "db", "jwks": {"keys",
  "age_secs"}}` (readiness). Keys load at start-up, retry every minute until
  they do, and are refetched hourly. Without keys every ingest is refused
  (503); public reads still work.

There is no staleness alert yet: alert on `/api/v1/repos` when a
repository's `data_as_of` falls more than a few runs behind (scheduled runs
are delayed by GitHub at times, and GitHub disables schedules in a public
repository after 60 days without activity).

### Backup

SQLite's `VACUUM INTO` writes a consistent copy while the service runs. The
image has no shell; run it from a sidecar or job that mounts the same
volume and has the `sqlite3` CLI:

```sh
sqlite3 /data/pr-hygiene.sqlite3 "VACUUM INTO '/backup/pr-hygiene-$(date +%F).sqlite3'"
```

Daily, with the same retention as the data. Losing the database loses
history only, never a verdict: the engine's state stays on GitHub. To
restore, stop the service and put the copy in place as
`/data/pr-hygiene.sqlite3` (with no stale `-wal`/`-shm` beside it).

### Retention

Raw snapshots are kept 30 days, then cleared at the next ingest (their
metadata stays). Each repository's last good data and the view are kept
until replaced. Stage changes are kept; they name PRs and stages, no person.
The ids of tokens that have posted are kept a day, long after any expires.

## Ingest

`POST /ingest`, `Authorization: Bearer <GitHub OIDC token>`, body: the
analyzer's `dashboard.json`. Checked in this order:

1. The token, before the body is read: RS256 only; signature against
   GitHub's keys; `iss` and `aud` exactly; `exp`/`nbf` with 60 s of skew;
   `iat` no older than 10 minutes; `repository_id`, `repository_owner_id`,
   `ref` with `ref_type` = `branch`, `workflow_ref` and `job_workflow_ref`,
   `event_name` ∈ {`schedule`, `workflow_dispatch`},
   `runner_environment` = `github-hosted`, `run_attempt` = `1`.
2. The body, up to the size limit: the analyzer's schema version, every
   login, repository and area in GitHub's or the policy's format, every text
   bounded. Long `fetch_error`/`stage_times_error` diagnostics are cut, not
   refused.
3. The binding to the run: the snapshot's `commit` is the token's `sha`, and
   its `generated_at` lies between the token's `iat` minus the job timeout
   and `iat` plus the skew.
4. One post per token: the token's `jti` is spent by its first post that
   gets this far, whatever came of it, so a captured token cannot carry a
   second body. A post job that retries must mint a new token.
5. Newer than the latest stored, or ignored (`200 {"stored": false}`): a
   replay changes nothing.

| Status | Meaning |
|---|---|
| 200 `{"stored": true, "snapshot", "stale_repos", "stage_changes"}` | Stored. |
| 200 `{"stored": false, "reason"}` | Not newer than the latest, or the token already posted; ignored. |
| 400 | The body is missing, malformed, out of bounds or not from this run. |
| 401 | No token, or not a valid GitHub token for this service. |
| 403 | A valid token from a run that may not post; the error names the claim. |
| 413 | Body over the limit. |
| 503 | GitHub's signing keys are not available. |

### Partial runs

A repository whose snapshot entry has a `fetch_error`, or no engine export
(`engine_state_available: false`), is **stale**: the service keeps showing
its last good data, marks it `stale` in `/api/v1/repos` and every list
response's `stale_repos`, and records no stage changes for it.

People are recomputed over the result by merging each repository's part of
each person: the owed reviews and authored PRs whose `owner/name#n` key is in
that repository, and that repository's `wip` and `areas` entries, each taken
from the snapshot that last read the repository. Everything a person carries
is keyed by repository in the analyzer's output, so nothing the snapshot
lacks is needed and none of the engine's rules is re-derived: when every
repository is read, the people served are exactly the analyzer's. Whether
someone is a bot, and how their login is spelled, come from the latest
snapshot.

### Stage changes

One row per PR when it is first seen, when its stage or engine state
changes, or when its recorded entry into the stage changes (it left and
came back between two runs). An entry time that could not be read is not a
change, and while some of a repository's stage records could not be read
(`stage_times_error`) a new entry time alone is not one either: the
analyzer then falls back to other start times. `since` is exact where the
analyzer's `since_basis` is `engine`; otherwise only `observed_at`, the
generation time of the snapshot that first showed the row, bounds it.

## Public API

`/api/v1`, JSON, no sign-in. `Access-Control-Allow-Origin: *`, no cookies,
`Cache-Control: public, max-age=60`, and an `ETag` per snapshot; send
`If-None-Match` for a 304. An unknown query parameter is a 400. Before the
first snapshot, every endpoint answers 503.

| Endpoint | |
|---|---|
| `GET /prs` | Open PRs. Filters, all optional and combined: `repo` (`owner/name`), `area`, `stage` (`draft`, `bots`, `self-review`, `ci`, `queued`, `review`, `mergeable`, `blocked`, `not-governed`, `unknown`), `author`, `reviewer` (someone who owes a review on it), `late` (`true`/`false`). |
| `GET /prs/{owner}/{repo}/{n}` | One PR, with the stage changes recorded for it. |
| `GET /people` | Everyone, sorted by login: what each owes and holds, and their areas. Nothing per person that ranks: no lateness, no "oldest owed". |
| `GET /people/{login}` | One person, any case: owed reviews (oldest first, with the PR) and authored PRs. `?format=text`: the digest below. |
| `GET /repos` | Each repository: `mode` (`shadow`), `stale`, `data_as_of`, `checked_at`, the latest run's errors, `slot_limit`. |
| `GET /stages` | Every stage in display order, whose move it is and when it is late. |

### The digest (`?format=text`)

Plain text for the `/prs` skill and Slack, versioned on its first line
(`PR Hygiene digest v1 for <login>`): generation time and commit; each
repository's freshness and mode, the stale ones named; reviews owed, oldest
first, each with author, how long it has waited, the person's part in the
engine's own words (``files with no dedicated owner (you or bob)``,
``re-review or resolve your objection``) and the PR's URL; their PRs with
stage, whose move it is, the engine's next action and URL; open PRs per
repository against its slot limit. Text from GitHub has control characters
replaced and `&`, `<`, `>` escaped, so it cannot start a line, mention a
channel or forge a link in Slack.

## Trying it locally

`import` stores a snapshot from a file with no token. It needs write access
to the database file, so it opens nothing to an HTTP client; the schema and
bounds are checked as on ingest, the binding to a run is not.

```sh
cargo run -p pr-hygiene-service -- import --db /tmp/prh.sqlite3 /tmp/b2-dashboard.json
PR_HYGIENE_DB=/tmp/prh.sqlite3 PR_HYGIENE_OIDC_AUDIENCE=http://localhost \
  PR_HYGIENE_BIND=127.0.0.1:8080 cargo run -p pr-hygiene-service -- serve
curl -s localhost:8080/api/v1/repos
curl -s 'localhost:8080/api/v1/people/QuantumExplorer?format=text'
```

`cargo test -p pr-hygiene-service` signs tokens with a key generated for the
test run and serves its own key set in place of GitHub's.
