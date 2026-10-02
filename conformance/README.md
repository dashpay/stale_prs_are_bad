# Conformance corpus

What the Python engine (`pr_review/`) decided, written down so that another
implementation of it can be held to the same answers, verdict for verdict and
write for write. The code is `pr_review/conformance.py`; its tests are
`pr_review/tests/test_conformance.py`.

There are five artifacts.

| Artifact | Where | Committed | What it pins |
|---|---|---|---|
| Boundary recordings | `conformance/live/` | **no** (`.gitignore`) | a whole run against real pull requests: every GitHub call and its answer, the verdicts, the ordered writes, the exact text written |
| Synthetic recordings | `conformance/synthetic/` | yes | the same, recorded offline over the recorder's test fake, shaped to walk read paths a live run may not show |
| Evaluate cases | `conformance/evaluate/` | yes | every `policy.evaluate` call the engine's own test suite makes: inputs and result |
| Function cases | `conformance/functions/<function>/` | yes | every call the test suite makes of the pure functions `main.py` relies on: inputs and output, byte for byte |
| Python behaviour goldens | `conformance/pycompat/` | yes | what Python 3.12 itself does with the text, JSON, patterns and timestamps the engine handles |

Python 3.12 is what these were produced with, and what the engine and its
tests run on in CI (`actions/setup-python`); any Python from 3.10 runs the
engine, but `harvest` refuses any other than 3.12. With
[uv](https://docs.astral.sh/uv/), prefix each command below with
`uv run -q --python 3.12 --no-project --with pyyaml`.

## Boundary recordings

The engine reaches GitHub in exactly one place: `GitHub._run` in
`pr_review/github.py`, which runs `gh api`. A recording stands there, in place
of the `subprocess` module that function calls, so the engine's own handling
of an answer — the single retry, a partial GraphQL answer, invalid JSON — runs
unchanged and is part of what is recorded.

### Recording

```sh
export GH_TOKEN=...      # read-only: see "Handling live recordings"
RAW=$(mktemp -d)         # mode 700, outside every repository and worktree
python -m pr_review.main report --repo dashpay/platform --format json \
    --record "$RAW/platform-report"
python -m pr_review.main sync --repo dashpay/platform \
    --record "$RAW/platform-sync"            # dry: nothing is sent
python -m pr_review.main sync --repo dashpay/platform --pr 1234 \
    --record "$RAW/platform-sync-pr-1234"    # one pull request's author
```

`--record DIR` is accepted on `report` and on `sync` without `--apply`, and
nowhere else; it refuses `--apply`, and refuses a `DIR` that already holds
anything. Every governed repository: `platform`, `rust-dashcore`,
`tenderdash`, `grovedb`, `dash-evo-tool`.

`sync --pr N` is the path an event on one pull request runs, and the one the
service runs per author: it reads that author's pull requests, decides their
admission, takes a snapshot of the one named (and of any of theirs whose slot
it moves), and sweeps nothing else. Record it beside the batch syncs.

What a recording run does differently, and only while recording:

- **Every call is classified at the boundary.** A read is a `GET` of a route
  under `repos/<owner>/<repo>/` with nothing on stdin, or a GraphQL document
  that is a query and nothing else, naming no key twice. Anything else,
  including any argument shape the engine does not build, is a write.
- **Writes are never sent.** The recorder answers each write itself, with what
  GitHub would have answered, made out of the request: a status or comment
  under the engine's own identity (`github-actions[bot]`), at the recorded
  instant, with an id above 10^15 so it cannot be mistaken for a real one. A
  write to a route it has no answer for stops the recording. The read path
  re-checks the classification before it runs `gh`.
- **A dry `sync` walks the write path**, as `--apply` would, so the writes it
  would have made are recorded in order. `report` has no write path. Since
  nothing lands, a read made after a write sees GitHub as it was: where the
  engine reads its own write back in the same run, the run goes on as it
  would if that write had been lost. The one place this shows today is a
  first admission: the pre-success re-check reloads the author's records,
  does not find the record this run wrote, and the recording ends with
  `pending: Review evidence changed; reconciliation required` where a real
  run posts the verdict. The recording is still exact — the same reads must
  give the same writes — but it is not a forecast of the next real run.
- **The clock is read once.** `main.clock()` is the one place the engine reads
  the time (`utc_now` and the batch rotations go through it); the recording
  fixes it at the start of the run and stores it. Every later read returns
  that instant and is logged, in order, as `{site, after}`: `site` is the
  engine function that asked (the first on the stack outside `clock` and
  `utc_now`, by name — never a line number, and never a comprehension or a
  lambda, whose frames come and go between Python versions), `after` the
  ordinal of the last call made before it, `0` for none.
- **The review system's status page** (`telemetry.fetch`) is read for real
  once, if the policy has bot timeouts, and stored.
- **Snapshots run one at a time.** In production four run at once and share
  the run's access and status caches, so which reads a run makes depends on
  which thread gets there first. A recording has to replay read for read.
- `GITHUB_RUN_ID`, the one environment variable the write path reads, is stored.

### Files in a recording

| File | Contents |
|---|---|
| `recording.json` | `format`, `repository`, `argv` (the command, without local paths), `clock` and `clock_reads`, `policy` (as loaded), `telemetry` (the payload or `null`) and `telemetry_reads`, `outcome` (`{"returned": 0}` or the exception), `environment`, `python`, `engine_commit`, `redacted` |
| `calls.jsonl` | one line per `gh api` call, in the order made: `ordinal`, `kind` (`read`/`write`), `args`, `stdin`, `exit`, `stdout`, `stderr`; `raised` when `gh` could not run |
| `verdicts.json` | the verdict rows, in order, exactly as `evaluate_snapshots` returned them |
| `evaluations.jsonl` | every `evaluate` call of the run: `pr` (the evidence Python's reader produced), `admitted_at`, `now`, `telemetry_states`, `result`; `policy` only where it is not the recording's |
| `outputs.json` | per verdict, the text it puts on GitHub — see below |
| `printed.txt` | the report the run printed (with `--format json`, what the dashboard reads) |

Objects are written in the engine's key order, which is how its maps iterate
(a record keeps the last eight receipts by insertion); comparisons treat a
map as its entries and a list as its order.

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
| `comment` | the whole record comment as a new one is written: markers, a blank line, then the move text, or where nobody is told a move the pointer at the description (a record refreshed in place keeps the words already there, and the writes show them) |
| `comment_refused` | why the engine would refuse to write that comment, else `null` |
| `diff_print` | `policy.diff_print` of the evidence |
| `receipt_prints` | `policy.receipt_print` of each comment that has one, by comment id |

`record` and `diff` are cut from `comment`, which `GitHub.state_comment_body`
wrote, so their bytes are the engine's own JSON writer's.

### Redacting

```sh
python -m pr_review.conformance redact "$RAW/platform-report" conformance/live/redacted/platform-report \
    && rm -rf "$RAW/platform-report"
```

A live recording is read with your token, so it holds what only you may see:
collaborator permission levels; a listing of everyone with access to the
repository, including members whose membership of the organisation is
private; and fields describing the asker, such as `author_association` (which
says `MEMBER` for a private member). Redaction:

- collapses every level to the two the engine tells apart: `write` for admin,
  maintain, write and any custom role that can push; `read` for triage, read
  and none. The flags, the legacy `permission` and every `role_name`,
  including the one inside `user`, say the same. A level the engine could not
  read becomes no flags, no permission and no role name;
- drops from the listing everyone not named by the policy and not appearing
  as a login in any other answer the run read — nobody the engine can ask about;
- removes `author_association`, `authorAssociation`, `permissions` (outside
  the two access routes: on a repository they are the asker's own) and
  `requested_teams` (which can name a secret team) from every other answer.
  The engine reads none of them.

`recording.json` says what was done, but not how many people were dropped.

It then replays the redacted copy and refuses to write it unless every verdict,
every result, the outcome and the order of every call are identical to the
original's, the evidence differs in nothing but permission levels, and the
writes differ in nothing but the evidence print. The copy stores what the
engine decides on the redacted answers (its evidence prints change, since they
hash the evidence), and is replayed once more after it is written. The raw
recording is left in place; the command above deletes it once the redacted
copy is written.

Redacted recordings are still not committed: they are large, and dated the
moment they are made.

### Handling live recordings

- **Record with a read-only token**: in a job, its installation token;
  locally, a fine-grained token with read access only. The recorder sends no
  write; the token makes sure none could be sent.
- **A raw recording lives outside every repository and worktree**, in a
  directory only you can open (mode 700, as `mktemp -d` makes one), and is
  deleted as soon as its redacted copy is written.
- **`conformance/live/` holds redacted copies only.**
- **No recording, raw or redacted, goes into a CI artifact, an issue, a pull
  request's text or an agent's prompt.** Report counts only: calls, writes,
  verdicts, differences.

### Replaying

```sh
python -m pr_review.conformance replay conformance/live/redacted conformance/evaluate conformance/functions
```

Takes recordings, evaluate cases, function cases, or directories of any of
them. For a recording
it runs the engine at the recorded clock, with the recorded policy and status
page, every read answered from the recording — each answer once, in the order
given for the same request — and every write answered as when recording.

A request is matched on its arguments exactly and on its body as a JSON
value: the path with its query string as written (`?state=open&per_page=100`,
a login escaped as `quote(login, safe='')`), `--paginate --slurp` meaning
every page as a list of pages, and a GraphQL query's text verbatim; the
separators, escaping and key order Python used to write the body are not part
of it.

This is Python checked against itself, so it is stricter than the contract
for another engine below. It fails when:

- a verdict differs in any field, or the outcome differs (including the
  exception's message);
- the writes differ in number, order, route or body;
- the calls were made in a different order;
- an output differs, or the printed report does;
- an evaluation's result or evidence differs;
- the engine asks a read the recording does not hold, or leaves one unasked;
- the status page is read a different number of times;
- the clock is read at another site, at another point among the calls, or a
  different number of times.

The summary line notes a recording made on another minor version of Python:
what `fromisoformat` accepts changed between them.

A recording is `format` 2. Format 1 is the same without `clock_reads`; it
still replays, every comparison but that one made, and its summary line says
so. Any other format is refused.

### Synthetic recordings

```sh
uv run -q --python 3.12 --no-project --with pyyaml python conformance/synthetic/generate.py
```

Recordings made by the same recorder, offline, with the recorder's own test
fake (`FakeGh` in `pr_review/tests/test_conformance.py`) standing where `gh`
does. Each is shaped to walk a read path a live recording may never show: a
conversation longer than the batched window, a record a person edited, a
GraphQL answer that fails with data, a transient failure and its retry, a
call that ran out of Python's own time. `generate.py` says which recording
covers what. They hold nothing private and are committed; the generator
writes the same bytes on every run, and a test fails when the committed ones
are not what it writes, so a change to what the engine reads carries the
regenerated recordings with it. Python 3.12 only, as the corpus is.

The Rust engine's `tests/evidence` rebuilds, from each recording's reads
alone, every snapshot Python's `evaluate` was given (`evaluations.jsonl`),
key for key and in its order. It asks each read as `gh api` with the exact
arguments and stdin Python sent, so a request written one byte differently
is one the recording does not hold. The same test runs over `conformance/live`
when asked (`cargo test -p pr-hygiene-engine --test evidence -- --ignored`),
and refuses a recording that was not redacted. Its driver is the engine's
`conformance` module, which the differential job (below) runs too.

## Evaluate cases

```sh
python -m pr_review.conformance harvest      # rewrites conformance/evaluate and conformance/functions
```

Runs the engine's test suite with `policy.evaluate` observed, and writes one
file per distinct call:
`{policy, pr, admitted_at, now, telemetry_states, result, tests,
python_exception_text, python}`. `tests` names the tests that made it; `python`
is the minor version that made it (only the minor: CI's patch release need not
be yours). Files are named
`<state>-<first 12 hex of SHA-256 over the case>.json`, so the same call is the
same file on every harvest; the engine clock is fixed at `2026-10-01T00:00:00Z`
during a harvest for the few tests that leave it running. A case is kept only
if it replays from its JSON exactly. Re-harvest whenever `evaluate` or its
tests change, and commit the difference: CI harvests into a scratch directory
and fails on any difference from what is committed.

Three test modules are not run (`NOT_HARVESTED` in `conformance.py`):
`test_conformance`, which replays the corpus, and `test_repositories` and
`test_roster`, which check the live policies under `policies/`. No case
depends on a live policy, so editing one leaves the corpus as it is and lands
without a re-harvest; tests that need a realistic policy read the frozen copy
in `pr_review/tests/fixtures/`.

`evaluate` is replaced before the engine or any test is imported, so every
name it is bound to is the observing one and the engine carries no hook for
it; an independent count of calls agrees (321). Evidence built from mocks
would not survive JSON and is not kept; the harvest reports how many calls
that was (none, today). The one test that evaluates in child processes, to
vary the hash seed, is not seen; other cases cover the renames it uses.

`python_exception_text` is true for a `configuration-error` whose first reason
is neither one `evaluate` stops with nor a message `policy.py` raises — read
from its source, so a new message is known without being listed.

## Function cases

The same harvest observes the pure functions `main.py` relies on besides
`evaluate`, and writes one file per distinct call into
`conformance/functions/<function>/<first 12 hex of SHA-256 over the case>.json`:
`{function, inputs, output, tests, python}`, `inputs` by parameter name.

| Function | What it makes |
|---|---|
| `main.checklist_block` | the description's block |
| `main.move_text` | the move comment's words |
| `github.GitHub.state_comment_body` | the record comment, markers and words |
| `main.state_record` | the record inside the state marker |
| `main.diff_record` | the diff record beside it |
| `policy.admit` | who holds a review slot, and since when |
| `main.admission_conflicts` | authors holding more slots than the policy allows |
| `policy.receipt_print` | what a bot's comment said, apart from how |
| `policy.diff_print` | what a reviewer read |
| `telemetry.head_state` | what the review system last said about a head |

`diff_print` and `receipt_print` are also called from inside `evaluate`, and
those calls are cases too. Nothing in the engine changes for this: each
function is replaced, like `evaluate`, before anything binds it.

`output` is compared as JSON text, so key order and every character count.
Two answers have no JSON of their own: `admit` keys its map by pull request
number, written as JSON writes any key, as a string (`{"12":"2026-…"}`, as
`serde_json` writes an integer key too); `admission_conflicts` answers a set,
written sorted. A case is kept only if it replays from its JSON exactly; today
one call does not (a `state_record` made while a test had replaced
`fingerprint` with a stand-in, which the real one does not reproduce). None raised and none
held anything JSON cannot, today; the harvest prints all of these counts per
function. A `state_record` output carries the evidence and context prints of
its inputs: here, unlike across runs, they are the same bytes from the same
inputs, made by the JSON writer the port has to have anyway.

```sh
python -m pr_review.conformance replay conformance/functions
```

## Python behaviour goldens

```sh
uv run -q --python 3.12 --no-project python conformance/pycompat/generate.py
```

Asks CPython 3.12 (its C `json` and `datetime`, its `re`, its Unicode 15.0
data) about every code point and about curated and seeded random inputs, and
writes the answers: the JSON files here, and the character tables the Rust
engine compiles in (`engine/src/pycompat/tables.rs`). The engine's
`tests/pycompat` replays them. The script refuses any other Python, records
the version it ran on in every file, and writes the same bytes each run; run
it again only when the Python the engine runs on changes.

```sh
uv run -q --python 3.12 --no-project --with pyyaml python conformance/pycompat/generate_policy.py
```

Writes what the policy port needs beyond that, from `pr_review/policy.py`
itself: `policy_regex.json` (every pattern the policy uses, with what `re`
matched), `object.json` (`str()`, float `repr`, `==` and `<` between values,
the characters whose `upper()` is ASCII, what `sorted()` leaves or raises),
`policy_malformed.json` (corpus cases with one field deleted or given another
type, whether `evaluate` raised or what it answered, and `fingerprint`), and
`policy_variants.json` (what the corpus cannot tell apart: which of two
equal instants a verdict keeps, what `strip` takes around an attestation or
a skip, the order `diff_print` sorts files in, and `fingerprint` of every
corpus snapshot). It refuses to run when a pattern it copies no longer
appears in `policy.py`, and records only the minor version of Python, so CI
regenerates the files and fails on any difference, as it does the corpus.

## The Rust engine's gate

`engine/tests/policy` runs every evaluate case and the `policy.admit`,
`policy.receipt_print` and `policy.diff_print` function cases through the
Rust port, comparing each result as the JSON Python's writer makes of it.
`conformance/pending.txt` lists, one path per line relative to this
directory, the cases the port is known not to match yet. It only shrinks: a
listed case that passes fails the test, and so do an unlisted case that
fails and a listed case that does not exist.

### Where the Rust engine departs from Python on purpose

Each refuses input that nothing GitHub answers, or the engine writes, can
hold, and fails closed or fails the run rather than guess:

- **A lone surrogate** (`"\ud800"`) in JSON read from a bot's comment cannot
  be held in a Rust string. That read takes its function's unreadable path,
  the path invalid JSON takes in Python: a CodeRabbit receipt holding one is
  not a receipt.
- **A float in a hashed print** (`diff_print`, `fingerprint`): Python's JSON
  writer writes it; the engine's writes no float, so the run fails
  (`PyErr::Unported`).
- **A carried commit id holding a character outside ASCII**: Python matches
  thepastaclaw's final-phase marker case-insensitively with its own Unicode
  case data, which the port does not have. Such an id matches nothing, so the
  bot reads as not having reported on it. The engine's records name only
  hexadecimal commits.
- **Timeouts in fractional hours**, and **a review slot keyed by a pull request
  number that is not an int**: refused (`Unported`); `validate_policy` and
  GitHub allow neither.
- **Sorting values that cannot be compared**: under 64 items the port asks `<`
  of the same pairs as CPython and raises where it raises; past that CPython
  merges runs and may meet a different incomparable pair first.

## The differential job

`.github/workflows/engine-differential.yml` is the Rust engine's shadow on
live data. It runs every six hours (03:40, 09:40, 15:40 and 21:40 UTC) and
on dispatch, on `master` only, with the read-only App token that
`pr-hygiene.yml`'s export uses.

**What it records.** The governed repositories are taken in turn, starting
from a different one each run. For each, the Python engine records:

- `report --format json`, every open pull request the policy governs;
- `sync --batch-size 6`, dry: the hourly sweep;
- `sync --pr N`, dry: the one-author path the service runs, for one pull
  request of each of three authors. The authors and their pull requests are
  read from the report's verdicts and rotate with the run, so every author
  comes round.

A budget bounds the run: 1 200 requests to GitHub, counted from the
recordings themselves, with one request per page of a paginated read (the
tool's `--requests`). Before each step the job compares what is left with
that step's estimate: eight requests per open pull request for a report, 150
for a sweep, 60 for a one-author sync. A step that would not fit is
skipped, and a repository skipped this run comes first in a later one. A
report reads about six requests per pull request (the pull request, files,
reviews, threads, statuses and checks), so the governed repositories' reports
together cost roughly six times their open pull requests. The job logs the
App's remaining REST and GraphQL limits before and after. When either is
below the budget plus a reserve of 1 000, nothing is recorded.

**What it compares.** Python first replays every recording against itself
(`conformance replay`). A failure there is the recorder's or the Python
engine's, not the port's. Then the engine's `differential` tool compares
each recording:

```sh
cargo run -q -p pr-hygiene-engine --bin differential -- DIR...
```

- **Snapshots**: every `pr` Python's `evaluate` saw, rebuilt from the
  recorded reads alone and compared exactly. The evidence tests run the same
  driver.
- **Evaluations**: the port's `evaluate` on each recorded evaluation's own
  arguments, compared exactly with Python's result. The one exclusion is the
  evaluate-case gate's: a reason that carries a Python exception's text. The
  tool decides which reasons those are from the engine's source, as
  `conformance.python_exception_text` does, and a test holds that rule to
  every evaluate case's mark.
- **Verdicts**: each row of `verdicts.json`, compared with the port's result
  plus what `evaluate_snapshots` adds. That is the repository, and the
  shared-head override where Python's row shows it, since which heads are
  shared comes from the open listing and not from `evaluate`.

Reads the port needs that the recording lacks are counted as missing reads,
and any count above zero fails the run. Writes, canonical outputs, the
printed report and the clock log are not compared yet. A deliberate
divergence listed above has no category of its own yet, so it shows as a
plain difference.

Formats 1 and 2 both load. The tool exits 0 when every recording matched, 1
on any difference or unreadable recording, and 2 when it could not run.

**What it never outputs.** Recordings are written to a mode-700 directory
under the runner's temporary directory. The job's last step deletes them,
pass or fail, and no artifact, cache or upload holds one. The engine's
printed report and its stderr go to `/dev/null`, and Python's replay shows
only its line per recording and its total. The tool prints counts, field
paths and case indices. It never prints a title, body, login, permission
level or error message, any of which a recording can hold. A key that is
data is written `*` (a login under `permissions`, a digest under
`receipts`), and a key the tool does not know as a field is written `?`. The
job summary holds the counts table only.

**Reading a red run.**

- *A dry run wrote no recording*: the recorder refused a call, or the engine
  exited before it ran. This is Python's problem.
- *Python's own replay failed*: the same recording replayed differently in
  Python. This is the recorder or the engine, not the port.
- *The Rust engine differed*: the categories table names the layer, the
  field path (`[]` is any list item), the kind of difference and the cases.
  The kinds are value, type, length, missing, extra, key order, exception
  text on one side only, and own words where Python raised. Cases are
  indices into `evaluations.jsonl` for snapshots and evaluations, and into
  `verdicts.json` for verdicts.
  - A snapshot that differs while its evaluation matches is the reader
    (`evidence`).
  - An evaluation that differs is `policy`.
  - *Read not in the recording* is a request the port wrote differently
    from Python, byte for byte, or one Python never made.

  The recordings of a red run are gone by the time anyone looks. To
  reproduce one, record the same command locally with a read-only token (see
  Recording), redact it into `conformance/live/`, and run `differential` or
  the ignored evidence test on the redacted copy.

**Phase 2, after the reconcile layer.** The same tool replays each sync
recording through the Rust reconcile layer, over a replay transport that
answers writes as the recorder did. It then compares:

- the ordered writes: method, path, and body as a JSON value;
- the canonical outputs (`outputs.json`), the printed report, and the clock
  reads;
- the verdict rows, now from the port's own `evaluate_snapshots`, with no
  override taken from Python's row.

The live half follows. Rust reads the same pull requests over HTTP, and its
snapshots are compared with Python's recorded ones. A pull request whose
evidence print equals Python's last real record must produce no would-be
write. Any difference is run again with Python's clock, status page and
`admitted_at` substituted, and only a difference that then vanishes is
categorised (clock, telemetry or admission).

## What another engine must match exactly

- **The verdicts**: every field of every row, rows in order.
- **The ordered writes**: method, path and body. Bodies are compared as JSON
  values; every string in them byte for byte.
- **The canonical outputs**: markers, record, diff record, `diff_print`,
  `receipt_print`, checklist, move text, labels, status state and description.
  These are what one engine writes and the other reads back.
- **The printed report**, where the port prints one.
- **Whether the run failed**, not the message it failed with.
- **Replaying the state after the writes** must produce no writes. Nothing
  here builds that state; the port's own replay fake does.

## What is excluded

- The `evidence` and `context` prints inside a record. They are compared only
  within one run, never across runs or engines.
- A blocker carrying the text of a Python exception (an evaluate case marks it
  `python_exception_text: true`), wherever it shows: the verdict's blockers and
  the printed report's next action.
- Which reads are made, and in what order. Another engine may read in any
  order and cache more; it must not need a read the recording does not hold.
- The canned ids and anything the engine prints to its log.

## What this corpus cannot tell apart

- **When the clock is read, from the answers alone.** It is fixed for the
  whole run, so an engine that reuses the run's instant where Python reads the
  clock again — the second evaluation before a success, the admission
  re-check — gives the same answers here; in a real run they differ by
  seconds. Only the log of reads (`clock_reads`) tells the two apart, so
  another engine logs a site for each read of its own clock, named as Python's
  are, and the two logs are compared.
- **What happens when a write fails.** Every canned answer is a success. The
  failure paths — a label that 404s, a reviewer request refused with 422 —
  belong to the port's own HTTP-mock tests.
- **A run that reads its own write.** See the dry `sync` under Recording.

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
  snapshot. A status the engine posts goes into its head's cache with the
  `creator` and `created_at` GitHub answered, and that cached status decides
  whether the same status is posted again.
- **Not to be copied: a race between snapshots.** In production four
  snapshots run at once, and `GitHub.access()` marks the listing read before
  it has read it; another thread can then find nobody listed, ask the
  per-person route, and on a failure read the person's access as unknown —
  a configuration error the next run clears. Recordings run snapshots one at
  a time and never show it; the port should read the listing once, then use it.
- **Login spellings**: REST reports an app as `name[bot]`, GraphQL as `name`.
  Comment authors are given the suffix back; review-thread authors are not, so
  a bot such as `github-actions` that opens a thread is looked up as a person
  (a 404, read as unknown access, which lets its thread count as an objection).
  No live verdict depends on this today.
