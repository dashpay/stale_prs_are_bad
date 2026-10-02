# Conformance corpus

What the Python engine (`pr_review/`) decided, written down so that another
implementation of it can be held to the same answers, verdict for verdict and
write for write. The code is `pr_review/conformance.py`; its tests are
`pr_review/tests/test_conformance.py`.

There are two artifacts.

| Artifact | Where | Committed | What it pins |
|---|---|---|---|
| Boundary recordings | `conformance/live/` | **no** (`.gitignore`) | a whole run against real pull requests: every GitHub call and its answer, the verdicts, the ordered writes, the exact text written |
| Evaluate cases | `conformance/evaluate/` | yes | every `policy.evaluate` call the engine's own test suite makes: inputs and result |

Python 3.12 is what these were produced with; any Python from 3.10 runs the
engine. With [uv](https://docs.astral.sh/uv/), prefix each command below with
`uv run -q --python 3.12 --no-project --with pyyaml`.

## Boundary recordings

The engine reaches GitHub in exactly one place: `GitHub._run` in
`pr_review/github.py`, which runs `gh api`. A recording stands there, in place
of the `subprocess` module that function calls, so the engine's own handling
of an answer — the single retry, a partial GraphQL answer, invalid JSON — runs
unchanged and is part of what is recorded.

### Recording

```sh
export GH_TOKEN=$(gh auth token)
python -m pr_review.main report --repo dashpay/platform --format json \
    --record conformance/live/raw/platform-report
python -m pr_review.main sync --repo dashpay/platform \
    --record conformance/live/raw/platform-sync          # dry: nothing is sent
```

`--record DIR` is accepted on `report` and on `sync` without `--apply`, and
nowhere else; it refuses `--apply`, and refuses a `DIR` that already holds
anything. Every governed repository: `platform`, `rust-dashcore`,
`tenderdash`, `grovedb`, `dash-evo-tool`.

What a recording run does differently, and only while recording:

- **Every call is classified at the boundary.** A read is a `GET` with nothing
  on stdin, or a GraphQL document that is a query and nothing else. Anything
  else, including any argument shape the engine does not build, is a write.
- **Writes are never sent.** The recorder answers each write itself, with what
  GitHub would have answered, made out of the request: a status or comment
  under the engine's own identity (`github-actions[bot]`), at the recorded
  instant, with an id above 10^15 so it cannot be mistaken for a real one. A
  write to a route it has no answer for stops the recording. The read path
  re-checks the classification before it runs `gh`.
- **A dry `sync` walks the write path**, as `--apply` would, so the writes it
  would have made are recorded in order. `report` has no write path.
- **The clock is read once.** `main.clock()` is the one place the engine reads
  the time (`utc_now` and the batch rotations go through it); the recording
  fixes it at the start of the run and stores it.
- **The review system's status page** (`telemetry.fetch`) is read for real
  once, if the policy has bot timeouts, and stored.
- **Snapshots run one at a time.** In production four run at once and share
  the run's access and status caches, so which reads a run makes depends on
  which thread gets there first. A recording has to replay read for read.
- `GITHUB_RUN_ID`, the one environment variable the write path reads, is stored.

### Files in a recording

| File | Contents |
|---|---|
| `recording.json` | `format`, `repository`, `argv` (the command, without local paths), `clock`, `policy` (as loaded), `telemetry` (the payload or `null`) and `telemetry_reads`, `outcome` (`{"returned": 0}` or the exception), `environment`, `python`, `engine_commit`, `redacted` |
| `calls.jsonl` | one line per `gh api` call, in the order made: `ordinal`, `kind` (`read`/`write`), `args`, `stdin`, `exit`, `stdout`, `stderr`; `raised` when `gh` could not run |
| `verdicts.json` | the verdict rows, in order, exactly as `evaluate_snapshots` returned them |
| `evaluations.jsonl` | every `evaluate` call of the run: `pr` (the evidence Python's reader produced), `admitted_at`, `now`, `telemetry_states`, `result`; `policy` only where it is not the recording's |
| `outputs.json` | per verdict, the text it puts on GitHub — see below |

Objects keep the engine's key order. That order is part of what it decided:
a record keeps the last eight receipts by insertion.

### Canonical outputs

Per evaluated pull request, what `publish` would make the pull request carry,
whether or not it already does (the writes say what was actually sent):

| Key | What it is |
|---|---|
| `status` | `{state, context, description}` of the `PR Hygiene` commit status |
| `labels` | the managed labels it should wear |
| `checklist` | the description's block, `null` for a draft |
| `move` | the move comment's text, `null` where nobody is told a move |
| `markers` | the marker lines of the record comment |
| `record`, `diff` | the record's and the diff record's JSON, as written inside the markers |
| `comment` | the whole record comment as written: markers, a blank line, then the move text or the pointer |
| `comment_refused` | why the engine would refuse to write that comment, else `null` |
| `diff_print` | `policy.diff_print` of the evidence |
| `receipt_prints` | `policy.receipt_print` of each comment that has one, by comment id |

`record` and `diff` are cut from `comment`, which `GitHub.state_comment_body`
wrote, so their bytes are the engine's own JSON writer's.

### Redacting

```sh
python -m pr_review.conformance redact conformance/live/raw/platform-report \
    conformance/live/redacted/platform-report
```

A live recording holds collaborator permission levels, which are not public,
and a listing of everyone with access to the repository, which includes
members whose membership of the organisation is private. Redaction:

- collapses every level to the two the engine tells apart: `write` for admin,
  maintain, write and any custom role that can push; `read` for triage, read
  and none. Flags, the legacy `permission` and `role_name` all say the same. A
  level the engine could not read is left as it was and counted;
- drops from the listing everyone not named by the policy and not appearing
  as a login in any other answer the run read — nobody the engine can ask about.

It then replays the redacted copy and refuses to write it unless every verdict,
every result, the outcome and the order of every call are identical to the
original's, the evidence differs in nothing but permission levels, and the
writes differ in nothing but the evidence print. The copy stores what the
engine decides on the redacted answers (its evidence prints change, since they
hash the evidence), and is replayed once more after it is written. The raw
recording is left in place; delete it once the redacted copy is made.

Redacted recordings are still not committed: they are large, and dated the
moment they are made.

### Replaying

```sh
python -m pr_review.conformance replay conformance/live/redacted conformance/evaluate
```

Takes recordings, evaluate cases, or directories of either. For a recording
it runs the engine at the recorded clock, with the recorded policy and status
page, every read answered from the recording — each answer once, in the order
given for the same request — and every write answered as when recording. It
fails when:

- a verdict differs in any field, or the outcome differs;
- the writes differ in number, order, route or body;
- the calls were made in a different order;
- an output differs;
- an evaluation's result or evidence differs;
- the engine asks a read the recording does not hold, or leaves one unasked;
- the status page is read a different number of times.

## Evaluate cases

```sh
python -m pr_review.conformance harvest      # rewrites conformance/evaluate
```

Runs the engine's test suite (all but `test_conformance`) with `policy.evaluate`
observed, and writes one file per distinct call:
`{policy, pr, admitted_at, now, telemetry_states, result, tests,
python_exception_text}`. `tests` names the tests that made it. Files are named
`<state>-<first 12 hex of SHA-256 over the case>.json`, so the same call is the
same file on every harvest; the engine clock is fixed at `2026-10-01T00:00:00Z`
during a harvest for the few tests that leave it running. A case is kept only
if it replays from its JSON exactly. Re-harvest whenever `evaluate` or its
tests change, and commit the difference.

`evaluate` is replaced before the engine or any test is imported, so every
name it is bound to is the observing one and the engine carries no hook for
it. Evidence built from mocks would not survive JSON and is not kept; the
harvest reports how many calls that was (none, today).

## What must match exactly

- **The verdicts**: every field of every row, in order.
- **The ordered writes**: method, path and body. Bodies are compared as JSON
  values; every string in them byte for byte.
- **The canonical outputs**: markers, record, diff record, `diff_print`,
  `receipt_print`, checklist, move text, labels, status state and description.
  These are what one engine writes and the other reads back.
- **Replaying the state after the writes** must produce no writes. Nothing
  here builds that state; the port's own replay fake does.

## What is excluded

- The `evidence` and `context` prints inside a record. They are compared only
  within one run, never across runs or engines.
- A blocker carrying the text of a Python exception: a `configuration-error`
  whose first reason is not one of `evaluate`'s own (an evaluate case marks it
  `python_exception_text: true`).
- The order of reads. Another engine may read in any order and cache more; it
  must not need a read the recording does not hold.
- The canned ids and anything the engine prints to its log.

## Python behaviours the port must reproduce

- **Insertion-ordered maps** (IndexMap, never a hash map): receipts keep the
  last eight by insertion; areas are listed in the order files first touch
  them (`dict.fromkeys`); permissions, labels and approvals keep read order.
- **First wins on ties** in `max(key=)` and `min(key=)`: the newest record or
  diff by `(updated_at, id)`, `max(created_at, head_seen_at, key=_time)` in
  `rate_limited_at`, the bots' completion and the attestation instants, an
  objector's latest voice. The value kept is the first one's own string, so
  `...Z` and `...+00:00` for the same instant are not interchangeable.
- **Tuple ordering** where a tie is not broken by a key: the latest inactive
  transition is `max((instant, text))`, so equal instants fall to the larger
  string.
- **Timestamps compared as strings** in some places and as instants in others:
  a build check's `startedAt`, a move comment's `created_at` against the bots'
  completion, `min` of a head's status times, comment ordering. Ported
  faithfully, not normalised.
- **`datetime.fromisoformat` after `Z` → `+00:00`**, which accepts more than
  RFC 3339 (Python 3.11+), and `isoformat()` on the way out: a waiver instant
  keeps the offset and any fraction of the instant it was computed from.
  `utc_now` is `isoformat(timespec='seconds')` with `Z`.
- **`str.splitlines`** splits on `\v`, `\f`, `\x1c`–`\x1e`, `\x85`, `U+2028`
  and `U+2029` as well as `\n` and `\r`. The fence scan that finds the
  checklist block counts character offsets through it.
- **`str.strip`, `str.split()`, `\s` and `str.lower`** are Unicode-aware:
  attestations, `/skip-bots`, receipt prints and the description comparison
  all strip or collapse Unicode whitespace.
- **Lengths and slices in code points**: a status description is cut to 140,
  a description may not pass 65 536, `head[:7]`.
- **The canonical JSON writer**: `json.dumps(value, separators=(',', ':'),
  sort_keys=True)` — keys sorted by code point, everything outside ASCII
  escaped as `\uXXXX` (astral characters as surrogate pairs). `diff_print`
  hashes `json.dumps(items, separators=(',', ':'))` without sorting keys; the
  admission context hashes `json.dumps(sorted(values), sort_keys=True)` with
  the default `', '` and `': '` separators.
- **Lenient JSON reading**: `json.loads` and `raw_decode` accept `NaN` and
  `Infinity`, keep the last of duplicate keys, and read integers exactly.
- **`re` semantics**: `$` also matches before a final newline, `\b` and `\s`
  are Unicode, `(?i)` folds Unicode case.
- **Truthiness**: an empty string, list or map is false (`if admitted_at:`,
  `x or default`).
- **`evaluate` turns `ValueError`, `TypeError` and `KeyError` into a
  configuration error** carrying the exception's text (a `KeyError` reads as
  the quoted key) — the excluded case above.
- **The boundary's own rules**: one retry, two seconds later, only for `GET`,
  `PUT`, `PATCH` and `DELETE`, and only when `502`, `503`, `504`, `timeout`,
  `EOF` or `unexpected end of JSON input` appears in what `gh` said. GraphQL
  reads are `POST` and never retried. A failed GraphQL call whose output still
  holds a `data` object is an answer. Empty output is `None`.
- **Per-run caches**: access by lowercased login (an unreadable answer is
  remembered as unknown for the run), statuses and builds by head, the
  collaborator listing read once; all cleared before the final pre-write
  snapshot.
- **Login spellings**: REST reports an app as `name[bot]`, GraphQL as `name`.
  Comment authors are given the suffix back; review-thread authors are not, so
  a bot such as `github-actions` that opens a thread is looked up as a person
  (a 404, read as unknown access, which lets its thread count as an objection).
  No live verdict depends on this today.
