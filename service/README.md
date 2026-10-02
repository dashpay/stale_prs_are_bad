# PR Hygiene service

A read-only web service. The scheduled `pr-hygiene.yml` workflow in this
repository posts the analyzer's `dashboard.json` to it, through the reusable
`pr-hygiene-post.yml`, with a GitHub Actions OIDC token; the service keeps
each repository's last good data in SQLite and serves it, publicly, as JSON
and as a plain-text digest.

It holds **no credential with access to any repository** and calls GitHub
only to fetch GitHub's public token-signing keys from
`https://token.actions.githubusercontent.com/.well-known/jwks` and, when
[sign-in](#sign-in-with-github) is on, to sign people in through a GitHub
App with no permissions.

## What it never does

- Write anything to GitHub, or read anything from it but those public keys
  and, at sign-in, the id and login of the person signing in.
- Accept data from anything but the post workflow's job, called by the one
  scheduled workflow on `master`, on a GitHub-hosted runner, on the first
  attempt of a scheduled or manual run.
- Look anything up for a public request: every answer comes from the
  database; an unknown login or PR is a 404.
- Set a cookie on a public response; keep a GitHub token past the sign-in
  that obtained it.
- Log an `Authorization` header, a cookie, a session id, a sign-in code or
  token, the client secret or a client address.

## Running it

| | |
|---|---|
| Image | `docker build -f service/Dockerfile -t pr-hygiene-service .` (from the repository root) |
| Process | `pr-hygiene-service serve`, as uid 65532 |
| Port | 8080, plain HTTP; terminate TLS in front of it |
| Volume | `/data`, holding `pr-hygiene.sqlite3` and its `-wal`/`-shm` files. Must be a **local** volume: SQLite's WAL mode needs real file locks, so not NFS or SMB. The service refuses to start if WAL cannot be enabled. |
| Outbound | HTTPS to `token.actions.githubusercontent.com`; with sign-in on, also `github.com` and `api.github.com` |
| Inbound | `POST /ingest` from GitHub-hosted runners, so reachable from the internet over HTTPS (runners have no fixed addresses; GitHub publishes their changing ranges under `actions` in `https://api.github.com/meta`, should you allowlist); `GET /api/v1/*` from anyone; with sign-in on, `/auth/*` and `/api/v1/me*` from browsers |

### Environment

| Variable | Default | Meaning |
|---|---|---|
| `PR_HYGIENE_OIDC_AUDIENCE` | — (required) | The audience the post job asks GitHub for: the service's public URL, e.g. `https://pr-hygiene.dash.org`, character for character the repository variable `PR_HYGIENE_SERVICE_URL` (a trailing slash in one and not the other is a mismatch). |
| `PR_HYGIENE_BIND` | `0.0.0.0:8080` | Listen address. |
| `PR_HYGIENE_DB` | `/data/pr-hygiene.sqlite3` | Database file. |
| `PR_HYGIENE_SITE_DIR` | `/srv/site` in the image | The page's files, served at `/` with its security headers (CSP incl. `frame-ancestors 'none'`, `nosniff`, `no-referrer`); the page reads `/dashboard.json`. Unset: API only. |
| `PR_HYGIENE_REPOSITORY_ID` | `1242761300` | Numeric id of the repository allowed to post (`dashpay/stale_prs_are_bad`). |
| `PR_HYGIENE_REPOSITORY_OWNER_ID` | `11511719` | Numeric id of its owner (`dashpay`). |
| `PR_HYGIENE_REF` | `refs/heads/master` | The only branch a posting run may run on. |
| `PR_HYGIENE_WORKFLOW_REF` | `dashpay/stale_prs_are_bad/.github/workflows/pr-hygiene.yml@refs/heads/master` | The only workflow whose runs may post (the token's `workflow_ref`): the scheduled caller. |
| `PR_HYGIENE_JOB_WORKFLOW_REF` | `dashpay/stale_prs_are_bad/.github/workflows/pr-hygiene-post.yml@refs/heads/master` | The only code that may hold a posting token (the token's `job_workflow_ref`): the reusable post workflow. Every other job of the caller — the Pages deploy, which also holds `id-token: write`, included — carries the caller's ref here and is refused. |
| `PR_HYGIENE_BODY_LIMIT` | `1048576` | Largest snapshot accepted, in bytes. Five repositories with 139 open PRs measured 109 KB. |
| `PR_HYGIENE_JOB_TIMEOUT_SECS` | `1800` | How long before the token was minted the snapshot may have been generated: the analyze job's `timeout-minutes` (20) plus the post job's start-up. 60 to 86400. |
| `PR_HYGIENE_SIGNIN_CLIENT_ID` | — | The sign-in App's **client id** (`Iv23li…`, not its numeric App id). See [sign-in](#sign-in-with-github). |
| `PR_HYGIENE_SIGNIN_CLIENT_SECRET_FILE` | — | A file holding the sign-in App's client secret, e.g. a mounted secret. Preferred. |
| `PR_HYGIENE_SIGNIN_CLIENT_SECRET` | — | The client secret itself, instead of the file (not both). |
| `PR_HYGIENE_PUBLIC_ORIGIN` | — | The origin the page is served from, exactly as a browser writes it: `https://hygiene.dash.org` — lower case, no default port, no path, no trailing slash (`http://` only for `localhost`). GitHub returns people to `<origin>/auth/callback`, and every request that changes something must carry exactly this `Origin`. |
| `RUST_LOG` | `info` | Log filter. |

The three sign-in settings (id, one of the secret's two sources, origin) are
set together or not at all: none turns sign-in off, part of them stops the
service at start-up.

### Turning on the post from GitHub Actions

The `post` job of `pr-hygiene.yml` is dormant until the repository variable
**`PR_HYGIENE_SERVICE_URL`** is set (Settings → Secrets and variables →
Actions → Variables) to the service's `https://` base URL — the same value
as `PR_HYGIENE_OIDC_AUDIENCE`. Every run then calls
`.github/workflows/pr-hygiene-post.yml`, which:

- downloads the run's `dashboard-data` artifact (no checkout, no build, no
  third-party action besides `actions/download-artifact`);
- mints an OIDC token with that URL as its audience, masks it, and
- POSTs `dashboard.json` to `<url>/ingest` with `curl --fail-with-body`,
  refusing a URL that is not `https://`.

Runs: every 15 minutes (analyze and post only) and every 6 hours or on
dispatch (also commit to the `data` branch and deploy Pages). A dry run
posts nothing. A post is never retried — a token posts once; the next run,
at most 15 minutes later, carries newer data. On a quarter-hourly run a
failed post is a warning (the service's staleness alert notices a lasting
outage); on a full run it fails the job. The post job sends only the bytes
whose SHA-256 the analyze job recorded: the token pin protects the token,
and the digest protects the data from any later step that could replace
the artifact.
A 403 names the claim that did not match; the job prints its
`job_workflow_ref` and `workflow_ref` before posting.

When a repository cannot be fetched the analyzer still writes the data
and then exits non-zero. The data is posted anyway: the service keeps that
repository's last good data and marks it stale, while the others stay
current. A full run then turns red and does not commit or deploy Pages; a
quarter-hourly run stays green with a warning. Runs that only lack an
engine export are posted too, and those repositories show as stale.

### In front of it

The service limits each request to 30 seconds and each snapshot to the body
limit, and nothing else. The reverse proxy that terminates TLS must also:

- time out slow and idle connections (request headers within about 10 s);
- cap connections, and rate-limit `/api/v1/*` per client;
- pass `POST /ingest` through unbuffered or with a body limit no larger
  than the service's, and never log its `Authorization` header;
- never log `Cookie` or `Set-Cookie` headers, nor the query string of
  `/auth/callback` (it carries GitHub's one-time code);
- rate-limit `/auth/*` and `/api/v1/me*` per client, tighter than the
  public reads: starting a sign-in needs no account and writes to the
  database, and a flood of them would push out real sign-ins under way;
- pass the `Origin` header through unchanged.

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

Daily. **Keep backups no longer than 13 months**, the longest the service
keeps anything personal (see [Retention](#retention)): a backup holds
sessions, speed inputs and opt-outs as they were that day, and an opt-out
or account deletion cannot reach into it. The service never sees its
backups, so pruning them is the operator's (infra's) job. Losing the
database loses history only, never a verdict: the engine's state stays on
GitHub. To restore, stop the service and put the copy in place as
`/data/pr-hygiene.sqlite3` (with no stale `-wal`/`-shm` beside it).

### Retention

| Kept | For |
|---|---|
| Sessions — the GitHub user id and login only (with the SHA-256 of the cookie's id) | 30 days from sign-in |
| Speed inputs | 13 months after the time that dates each row; an ask or turn still open is kept, as it is current |
| Stage history (`stage_changes`; names PRs and stages, no person) | 13 months after the snapshot that first showed each row |
| Raw snapshots | 30 days, then cleared at the next ingest; their time and commit stay |
| Sign-ins under way | 10 minutes |
| Opt-outs — the GitHub user id and when | until further notice, through account deletion |
| Each repository's last good data, and the view | until replaced |
| Ids of tokens that have posted | a day, long after any expires |
| Backups | no longer than 13 months: the operator's job (see [Backup](#backup)) |

Expired sessions, abandoned sign-ins, and stage history and speed inputs
past their 13 months are deleted at start-up and daily after. A PR that
has not changed stage in 13 months loses its only stage row and is
recorded afresh at the next ingest.

**Opting out** (`POST /api/v1/me/opt-out`) deletes your speed inputs at
once and keeps them from being recorded again, at every ingest and import
from then on; the opt-out itself keeps your user id only. The public
queue, a mirror of GitHub, is unchanged, and you stay signed in.
**Deleting your account** (`DELETE /api/v1/me`) deletes every session of
yours, in every browser, and your speed inputs. It keeps an opt-out if you
made one, so that goes on being honoured; without one, recording resumes
from the next ingest, as for anyone in the public data.

## Sign in with GitHub

Optional; off until configured. Signing in only tells the page who you are:
`GET /api/v1/me` then answers with your GitHub id, login and public People
entry. Everything else stays public and needs no sign-in.

### Creating the sign-in App

A **separate, public GitHub App** — not the App that posts statuses, and
public because a private App lets only its own organisation's members sign
in. In the organisation's settings → Developer settings → GitHub Apps → New:

- **Homepage URL**: the service's origin, e.g. `https://hygiene.dash.org`.
- **Callback URL**: `<origin>/auth/callback`, exactly, and no other.
- **Expire user authorization tokens**: on (the service revokes each token
  at once anyway). **Request user authorization (OAuth) during
  installation**: off. **Enable Device Flow**: off.
- **Webhook**: inactive.
- **Permissions**: none at all — repository, organisation and account. The
  authorisation screen then asks for nothing beyond the public profile, and
  no email.
- **Where can this GitHub App be installed?**: Any account. It never needs
  installing: signing in is a user authorisation, not an installation.

Then generate a **client secret**, and give the service the client id, the
secret (best as a file: `PR_HYGIENE_SIGNIN_CLIENT_SECRET_FILE`) and
`PR_HYGIENE_PUBLIC_ORIGIN`. The page must be served from that origin
(`PR_HYGIENE_SITE_DIR`), as the cookies are bound to it.

### How a sign-in goes

1. `GET /auth/login` stores a random `state` and PKCE verifier under a
   random id, sets that id in the `__Host-prh_prelogin` cookie (`Secure`,
   `HttpOnly`, `SameSite=Lax`, `Path=/`, 10 minutes) and redirects to
   `https://github.com/login/oauth/authorize` with `client_id`, the fixed
   `redirect_uri`, `state`, `code_challenge` and
   `code_challenge_method=S256`. No scopes.
2. GitHub sends the browser to `GET /auth/callback?code=…&state=…`. The
   pre-login named by the cookie is deleted at once, so it is good for one
   try; `state` must equal it. The code is exchanged with the client id and
   secret, `redirect_uri` and the PKCE verifier (an `error` in GitHub's 200
   answer is a failure); `GET https://api.github.com/user` is read once for
   the id and login; the token is revoked (`DELETE
   /applications/{client_id}/token`; if that fails it is logged and the
   sign-in goes on — the token reads only public data and expires); the
   session is created, ending the session this browser held before, if
   any.
3. Success redirects to `/#/me` with the `__Host-prh_session` cookie
   (`Secure`, `HttpOnly`, `SameSite=Lax`, `Path=/`, 30 days). Any failure
   redirects to `/#/me?signin=failed`: GitHub's error text is never shown,
   and no request can choose where a sign-in goes or ends.

Calls to GitHub time out after 6 s, follow no redirects and read at most
64 KB. They run as a task of their own, so the token is revoked even if
the browser goes away mid-sign-in.

### Routes

None of these are public API: no CORS headers, `Cache-Control: private,
no-store`, `Vary: Cookie`, no ETag. With sign-in off, each answers 404.

| Route | |
|---|---|
| `GET /auth/login` | Start a sign-in (above). |
| `GET /auth/callback` | Finish it (above). |
| `GET /api/v1/me` | `{"id", "login", "person", "opted_out"}` — `person` is the `/api/v1/people/{login}` entry, or `null` when the data does not name you; 401 signed out. |
| `GET /api/v1/me/speed` | Your own speed (below), computed on request from your speed inputs alone; `{"opted_out": true}` once you opted out; 401 signed out. |
| `POST /auth/logout` | End this browser's session, on the server too. 204. |
| `POST /api/v1/me/opt-out` | Opt out: no speed is computed for you and its inputs are deleted; the public queue, a mirror of GitHub, is unchanged. You stay signed in. 204; 401 signed out. |
| `DELETE /api/v1/me` | Delete your account: every session of yours, in every browser, and your speed inputs. An opt-out is kept, so it is still honoured. 204; 401 signed out. |

The three that change something require an `Origin` header exactly equal to
`PR_HYGIENE_PUBLIC_ORIGIN`; a missing or different one is a 403. The page
calls them with `fetch` (its CSP has `form-action 'none'`).

### Your speed

```json
{"since": "2025-10-14T12:00:00Z",
 "months": [{"month": "2026-09",
   "review_wait": {"asked": 7, "answered": 5, "open": 1, "median_hours": 18.5, "n": 5},
   "your_turn":   {"median_hours": 30.0, "n": 4},
   "cycle_time":  {"median_hours": 96.0, "n": 3}}]}
```

Months run oldest first, from the first with data to the current one, the
last 12 at most. Each median rolls over the month and the two before it,
in hours to one decimal, and is `null` when that window holds fewer than
three values; `n` is the window's count either way. `since` is the
earliest time the listed months rest on (`null` with no data).

- **Review wait**: the asks made of you in the window — a PR waiting on
  review with you among the approvers of an area still unapproved, or
  with your objection standing — and how many you answered and how many
  are still open. Timed from when you were asked to your first decisive
  review, for answered asks only: one covered by a co-approver, or ended by
  the PR leaving review, is counted, never timed.
- **Your turn**: on your PRs, the time per turn in the author's stage
  (self-review, answering an objection, a failed build). A run broken only
  by bots or a build is one turn; only its stretches in the author's stage
  count. By the month a turn ended.
- **Cycle time**: first ready for review to merge, on your merged PRs, by
  the month merged. It includes other people's review time.

An interval is counted but not timed when its start is not known (already
running when its repository was first read, with no recorded start) or
when its repository could not be read while it was open.

### What is stored

| Table | Holds | Kept |
|---|---|---|
| `prelogins` | SHA-256 of the pre-login cookie's id; the `state` and PKCE verifier; when | until used, at most 10 minutes; at most 10 000 at once, oldest dropped first |
| `sessions` | SHA-256 of the session cookie's id (never the id); GitHub user id and login; created and expiry | 30 days; at most 5 per person, oldest dropped first |
| `opt_outs` | GitHub user id; when | until further notice, through account deletion |
| `asks` | PR; the GitHub user id asked (while an ask is open, the login asked instead until an event ties it to an id); when asked and when it ended; answered, covered or left; whether timed | 13 months after it ended |
| `turns` | PR; author id; start, end and time in the author's stage; whether timed | 13 months after it ended |
| `merges` | PR; author id; first ready for review; merged | 13 months after the merge |
| `reviews` | PR; reviewer id; approved, changes requested or dismissed; when | 13 months after the review |
| `pr_authors` | PR; author id; last seen | 13 months after last seen |

No GitHub token, email or name is stored. The speed inputs come from
public GitHub activity in the snapshots, only from repositories a snapshot
read, and are never recorded for anyone who opted out. A login is tied to
an account id only by an event GitHub reports in the same snapshot — that
person's own decisive review on the PR, or a PR they authored — never by
looking a login up, so someone who registers a login another person gave up
inherits nothing. An ask that ends before anything ties its login to an id
is deleted. Opting out or deleting your account deletes every row with your
id and the open asks known only by the login you signed in with.

## Ingest

`POST /ingest`, `Authorization: Bearer <GitHub OIDC token>`, body: the
analyzer's `dashboard.json`. Checked in this order:

1. The token, before the body is read: RS256 only; signature against
   GitHub's keys; `iss` and `aud` exactly; `exp`/`nbf` with 60 s of skew;
   `iat` no older than 10 minutes; `repository_id`, `repository_owner_id`,
   `ref` with `ref_type` = `branch`, `workflow_ref` (the caller) and
   `job_workflow_ref` (the post workflow), `event_name` ∈ {`schedule`,
   `workflow_dispatch`}, `runner_environment` = `github-hosted`,
   `run_attempt` = `1`. Not `sub`: it changes when the repository is
   renamed, and its format can be customised.
2. The body, up to the size limit: the analyzer's schema version, every
   login, repository and area in GitHub's or the policy's format, every text
   bounded, every time between 2000 and the run (`generated_at` up to now,
   each PR's times up to an hour after `generated_at`). Long
   `fetch_error`/`stage_times_error` diagnostics are cut, not refused.
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
generation time of the snapshot that first showed the row, bounds it. Rows
are kept 13 months after their `observed_at`.

## Public API

`/api/v1`, JSON, no sign-in. `Access-Control-Allow-Origin: *`, no cookies,
`Cache-Control: public, max-age=60`, and a strong `ETag` — a hash of the
response's bytes; send `If-None-Match` for a 304. `OPTIONS` answers the
browser's preflight for that header. An unknown query parameter is a 400.
Before the first snapshot, every endpoint answers 503.

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
first, each with author, how long it has waited, what it still needs and
the person's part in the engine's own words (``needs `dpp`: alice or bob ·
re-review or resolve: carol; your part: `dpp` (you or bob)``) and the PR's
URL; their PRs with
stage, whose move it is, the engine's next action and URL; open PRs per
repository against its slot limit. Text from GitHub has control characters
replaced and `&`, `<`, `>` escaped, so it cannot start a line, mention a
channel or forge a link in Slack.

## Trying it locally

`import` stores a snapshot from a file with no token. It needs write access
to the database file, so it opens nothing to an HTTP client; the schema and
bounds are checked as on ingest, the binding to a run is not. A snapshot no
newer than the latest is not stored, but its closed PRs' merges and
reviews are: that is how a year of cycle time gets in, from a one-time
`pr-hygiene --json-out backfill.json --closed-days 365` run, too large to
post.

```sh
cargo run -p pr-hygiene-service -- import --db /tmp/prh.sqlite3 /tmp/b2-dashboard.json
PR_HYGIENE_DB=/tmp/prh.sqlite3 PR_HYGIENE_OIDC_AUDIENCE=http://localhost \
  PR_HYGIENE_BIND=127.0.0.1:8080 cargo run -p pr-hygiene-service -- serve
curl -s localhost:8080/api/v1/repos
curl -s 'localhost:8080/api/v1/people/QuantumExplorer?format=text'
```

`cargo test -p pr-hygiene-service` signs tokens with a key generated for the
test run and serves its own key set in place of GitHub's.
