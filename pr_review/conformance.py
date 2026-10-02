"""Record the engine at its one GitHub boundary, and replay what it recorded.

A recording holds every `gh api` call the engine makes, the instant it ran at,
the review system's status page as it read it, and what it decided: the
verdicts, the ordered writes it would have sent, and the exact text each
pull request's marks would carry. Another implementation of the engine replays
the reads and must arrive at the same verdicts and the same writes.

    python -m pr_review.main report --repo dashpay/platform --record DIR
    python -m pr_review.main sync   --repo dashpay/platform --record DIR
    python -m pr_review.conformance replay DIR [DIR ...]
    python -m pr_review.conformance redact SOURCE DESTINATION
    python -m pr_review.conformance harvest [conformance/evaluate] [--functions conformance/functions]

Nothing here sends a write. Every call is classified at the boundary, a write
is answered by the recorder with a success of its own making, and only a call
proven to be a read is passed to `gh`. See conformance/README.md.
"""

import argparse
import copy
import hashlib
import io
import json
import os
import re
import subprocess
import sys
import tempfile
import threading
from contextlib import ExitStack, redirect_stderr, redirect_stdout
from datetime import datetime, timezone
from pathlib import Path

FORMAT = 2
# Format 1 is format 2 without `clock_reads`. It still replays, with the one
# comparison it cannot make left out.
READABLE_FORMATS = (1, FORMAT)
# The identity every write is answered as: the one the Actions workflow posts
# under. The engine recognises its own statuses and comments by it, so an
# answer under any other name would make it read its own writes as a stranger's.
ENGINE_CREATOR = {'login': 'github-actions[bot]', 'type': 'Bot'}
# Ids for what the recorder pretends to create. Far above any id GitHub has
# issued, so a canned comment can never be mistaken for a real one.
CANNED_ID_BASE = 10 ** 15
# The arguments `GitHub.request` and `GitHub.pages` build, and nothing else. A
# call carrying any other flag is not one this recorder can prove is a read.
_FLAGS_WITH_VALUE = {'--method', '--input'}
_BARE_FLAGS = {'--paginate', '--slurp'}
_GRAPHQL_WRITES = re.compile(r'\b(?:mutation|subscription)\b')
_HASHES = re.compile(r'"(evidence|context)":"[0-9a-f]{64}"')
# Blockers `evaluate` stops with in its own words. With what `policy.py`
# raises, these are the engine's; a configuration error carrying anything
# else carries the text of a Python exception, which another implementation
# cannot be asked to reproduce.
_OWN_BLOCKERS = ('Incomplete GitHub snapshot', 'Invalid head SHA', 'PR is outside the active policy scope',
                 'Draft PR does not occupy a review slot', 'No changed-file evidence', 'Unresolved identities in ',
                 'Cannot verify write access for ', 'An assigned owner/reviewer lacks verified write access',
                 'Another open PR shares this head')


class RecordingError(RuntimeError):
    """The recording cannot be made or trusted. Never an answer from GitHub."""


# ---------------------------------------------------------------- the boundary

def _parse(arguments):
    """(method, path, reads stdin, flags), or None for a shape the engine never builds."""
    method = path = None
    stdin, flags = False, set()
    index = 0
    while index < len(arguments):
        argument = arguments[index]
        if argument in _FLAGS_WITH_VALUE and index + 1 < len(arguments):
            value = arguments[index + 1]
            if argument == '--method':
                if method is not None:
                    return None
                method = value
            elif value == '-':
                stdin = True
            else:
                return None
            index += 2
        elif argument in _BARE_FLAGS:
            flags.add(argument)
            index += 1
        elif not argument.startswith('-') and path is None:
            path = argument
            index += 1
        else:
            return None
    return method, path, stdin, flags


# Every route the engine reads is under the repository it governs.
_REPOSITORY_PATH = re.compile(r'repos/[A-Za-z0-9_.-]+/[A-Za-z0-9_.-]+/')


def _unique_keys(pairs):
    """A JSON object that names a key twice is not one this recorder will judge."""
    keys = [key for key, _ in pairs]
    if len(keys) != len(set(keys)):
        raise ValueError('duplicate key')
    return dict(pairs)


def is_read(arguments, stdin):
    """Whether this call can only read. Anything not proven a read is a write.

    A GET of a route under the repository with nothing on stdin, or a GraphQL
    document that is a query and nothing else. GraphQL is a POST, so the
    method alone does not decide it.
    """
    parsed = _parse(list(arguments))
    if parsed is None:
        return False
    method, path, reads_stdin, flags = parsed
    if not path:
        return False
    if method == 'GET' and not reads_stdin and stdin is None and _REPOSITORY_PATH.match(path) \
            and '://' not in path and '..' not in path:
        return True
    if method == 'POST' and path == 'graphql' and reads_stdin and not flags and isinstance(stdin, str):
        try:
            document = json.loads(stdin, object_pairs_hook=_unique_keys)
        except ValueError:
            return False
        query = document.get('query') if isinstance(document, dict) else None
        return (isinstance(query, str) and set(document) <= {'query', 'variables'}
                and query.lstrip().startswith('query') and not _GRAPHQL_WRITES.search(query))
    return False


def canned_answer(arguments, stdin, now, serial):
    """What GitHub would answer a write with, made here and never asked for.

    Only the routes the engine writes to: an answer it cannot accept would
    change what it does next, and a write to a route nobody planned for is
    a change to the engine this recorder has to learn about, loudly.
    """
    method, path, _, _ = _parse(list(arguments)) or (None, None, None, None)
    payload = json.loads(stdin) if stdin else None
    route = re.fullmatch(r'repos/[A-Za-z0-9_.-]+/[A-Za-z0-9_.-]+/(.+)', path or '')
    tail = route.group(1) if route else ''
    if method == 'POST' and re.fullmatch(r'statuses/[0-9a-f]{40}', tail):
        return {'id': serial, 'state': payload['state'], 'context': payload['context'],
                'description': payload.get('description'), 'target_url': payload.get('target_url'),
                'creator': dict(ENGINE_CREATOR), 'created_at': now, 'updated_at': now}
    if method == 'POST' and re.fullmatch(r'issues/\d+/comments', tail):
        return {'id': serial, 'body': payload['body'], 'user': dict(ENGINE_CREATOR),
                'created_at': now, 'updated_at': now}
    found = re.fullmatch(r'issues/comments/(\d+)', tail)
    if method == 'PATCH' and found:
        return {'id': int(found.group(1)), 'body': payload['body'], 'user': dict(ENGINE_CREATOR), 'updated_at': now}
    if method == 'DELETE' and found:
        return None
    if method == 'POST' and re.fullmatch(r'issues/\d+/labels', tail):
        return [{'name': name} for name in payload['labels']]
    if method == 'DELETE' and re.fullmatch(r'issues/\d+/labels/[^/]+', tail):
        return []
    found = re.fullmatch(r'pulls/(\d+)', tail)
    if method == 'PATCH' and found:
        return {'number': int(found.group(1)), 'body': payload['body']}
    found = re.fullmatch(r'pulls/(\d+)/requested_reviewers', tail)
    if method == 'POST' and found:
        return {'number': int(found.group(1)), 'requested_reviewers': [{'login': u} for u in payload['reviewers']]}
    raise RecordingError(f'A write to a route the recorder has no answer for: {method} {path}')


class _Boundary:
    """Stands in for the `subprocess` module inside `pr_review.github`.

    `GitHub._run` keeps every rule of its own — the single retry, a partial
    GraphQL answer, invalid JSON — and only what `gh` would have said comes
    from here.
    """
    TimeoutExpired = subprocess.TimeoutExpired

    def __init__(self, session):
        self.session = session

    def run(self, command, input=None, **options):
        return self.session.call(command, input, options)


class _NoSleep:
    """`time` for `pr_review.github` during a replay: the retry still happens, the wait does not."""

    @staticmethod
    def sleep(seconds):
        return None


class _InOrder:
    """The engine's thread pool, one task at a time, in the order given.

    Four snapshots at once share the per-run access and status caches, so
    which reads a run makes depends on which thread gets there first. A
    recording has to be replayable read for read, and only an order fixed in
    advance gives that.
    """

    def __init__(self, max_workers=None):
        pass

    def __enter__(self):
        return self

    def __exit__(self, *exc):
        return False

    def map(self, function, *iterables):
        return [function(*items) for items in zip(*iterables)]


# Where the engine lives: a frame from any other file is not the engine asking.
_ENGINE = Path(__file__).resolve().parent


def _clock_site(frame):
    """The engine function that asked for the time: the first on the stack outside `clock` and `utc_now`.

    By name, not line, so an edit elsewhere in the file moves nothing. A
    comprehension, a generator expression or a lambda is not a site of its
    own: whether a comprehension has a frame at all depends on the Python.
    """
    while frame is not None:
        code = frame.f_code
        where = Path(code.co_filename).resolve()
        if (where.parent == _ENGINE and where.name != 'conformance.py'
                and code.co_name not in {'clock', 'utc_now'} and not code.co_name.startswith('<')):
            return code.co_name
        frame = frame.f_back
    return None


def _iso(instant):
    return instant.astimezone(timezone.utc).isoformat(timespec='seconds').replace('+00:00', 'Z')


def _instant(text):
    return datetime.fromisoformat(text.replace('Z', '+00:00'))


def _dump(value):
    """JSON in the engine's own key order. That order is part of what it decided."""
    return json.dumps(value, ensure_ascii=False, separators=(',', ':'))


def _masked(value):
    """`value` with the evidence and context prints blanked wherever a record carries them."""
    if isinstance(value, str):
        return _HASHES.sub(lambda m: f'"{m.group(1)}":"-"', value)
    if isinstance(value, list):
        return [_masked(v) for v in value]
    if isinstance(value, dict):
        return {k: _masked(v) for k, v in value.items()}
    return value


def _value(stdin):
    """A request body as the JSON value it carries, or the text itself when it carries none.

    Requests are matched on what they say, not on how Python spelled the
    JSON: separators, escaping and key order are not part of a request.
    """
    if not stdin:
        return stdin
    try:
        return json.loads(stdin)
    except ValueError:
        return stdin


def _same_form(value):
    """`value` written so that two maps holding the same entries compare equal; lists keep their order."""
    return json.dumps(value, sort_keys=True, ensure_ascii=False, separators=(',', ':'))


def outputs_for(engine, policy, context, pr, result):
    """The exact text this verdict puts on GitHub, as `publish` would write it.

    The desired state, not the delta: a pull request already carrying these
    marks gets no write, and the ordered writes say which were sent.
    """
    from . import github
    from .policy import LABEL_FOR_STATE, diff_print, machine_author, receipt_print
    state = result['state']
    record = engine.state_record(pr, result, engine.context_fingerprint(context, pr['author']))
    carried = engine.diff_record(pr, result)
    move = engine.MOVE_STATES.get(state)
    # As `publish` decides it: nothing is told its move when nobody is there to take it.
    move_body = engine.move_text(result) if move and not (
        machine_author(policy, pr) and move == 'waiting-self-review') else None
    try:
        github._validate_state(record)
        comment = github.GitHub.state_comment_body(record, move_body if move_body is not None else engine.POINTER,
                                                   carried)
        refused = None
    except github.GitHubError as error:
        comment, refused = None, str(error)
    markers = comment.partition('\n\n')[0] if comment else None
    found = github.STATE_PATTERN.search(markers or '')
    diff_found = github.DIFF_PATTERN.search(markers or '')
    labels = ({LABEL_FOR_STATE[state]} if state in LABEL_FOR_STATE else set()) | (
        {engine.WAIVED_LABEL} if result.get('waived') else set())
    receipts = {}
    for item in pr.get('comments') or []:
        said = receipt_print(item)
        if said:
            receipts[str(item['id'])] = said
    return {
        'number': pr['number'], 'head': pr['head'],
        'status': {'state': result['status'], 'context': 'PR Hygiene', 'description': state[:140]},
        'labels': sorted(labels),
        'checklist': None if state == 'draft' else engine.checklist_block(result),
        'move': move_body,
        'record': found.group(1) if found else None,
        'diff': diff_found.group(1) if diff_found else None,
        'markers': markers,
        'comment': comment,
        'comment_refused': refused,
        'diff_print': diff_print(pr),
        'receipt_prints': receipts,
    }


# ---------------------------------------------------------------- sessions

class _Session:
    """Everything a run reads from outside itself, held in one place."""

    def __init__(self, now):
        self.now = now
        self.calls = []
        self.clock_reads = []
        self.verdicts = []
        self.evaluations = []
        self.outputs = []
        self.policy = None
        self.policies_loaded = 0
        self.telemetry_reads = 0
        self._writes = 0
        self._lock = threading.Lock()
        self._boundary = _Boundary(self)
        self._installed = False
        self._fetch = None
        self.printed = ''

    # -- what `main.run` asks of a recording

    def captures_writes(self):
        from . import github
        return self._installed and github.subprocess is self._boundary

    def loaded_policy(self, policy):
        self.policies_loaded += 1
        self.policy = copy.deepcopy(policy)

    # -- the clock

    def clock(self):
        """The recorded instant, logged with where the engine asked for it.

        The site, and the ordinal of the last call made before the read
        (0 for none): a run that reads the time somewhere else, or at
        another point among its calls, is told apart, though every read
        returns the same instant.
        """
        site = _clock_site(sys._getframe(1))
        with self._lock:
            self.clock_reads.append({'site': site, 'after': len(self.calls)})
        return _instant(self.now)

    # -- the boundary

    def call(self, command, stdin, options):
        command = list(command)
        if command[:2] != ['gh', 'api']:
            raise RecordingError('Only `gh api` crosses this boundary')
        arguments = command[2:]
        with self._lock:
            entry = {'ordinal': len(self.calls) + 1}
            if is_read(arguments, stdin):
                entry.update(kind='read', args=arguments, stdin=stdin)
                entry.update(self._read(command, arguments, stdin, options))
            else:
                self._writes += 1
                answer = canned_answer(arguments, stdin, self.now, CANNED_ID_BASE + self._writes)
                entry.update(kind='write', args=arguments, stdin=stdin, exit=0,
                             stdout='' if answer is None else json.dumps(answer), stderr='')
            self.calls.append(entry)
        if entry.get('raised') == 'TimeoutExpired':
            raise subprocess.TimeoutExpired(command, options.get('timeout'))
        if entry.get('raised') == 'OSError':
            raise OSError(entry.get('message') or 'gh unavailable')
        return subprocess.CompletedProcess(command, entry['exit'], entry['stdout'], entry['stderr'])

    def _read(self, command, arguments, stdin, options):
        raise NotImplementedError

    def telemetry(self):
        raise NotImplementedError

    # -- what the engine decided

    def _observe(self, engine):
        original_evaluate, original_snapshots = engine.evaluate, engine.evaluate_snapshots
        session = self

        def evaluate(policy, pr, admitted_at, now, telemetry_states=None):
            seen = copy.deepcopy(pr)
            result = original_evaluate(policy, pr, admitted_at, now, telemetry_states)
            session.evaluations.append({'pr': seen, 'admitted_at': admitted_at, 'now': now,
                                        'telemetry_states': copy.deepcopy(telemetry_states),
                                        'result': copy.deepcopy(result)})
            if policy != session.policy:
                session.evaluations[-1]['policy'] = copy.deepcopy(policy)
            return result

        def evaluate_snapshots(policy, context, candidates, snapshots, now, payload=None):
            rows = original_snapshots(policy, context, candidates, snapshots, now, payload)
            session.verdicts.extend(copy.deepcopy(rows))
            session.outputs.extend(outputs_for(engine, policy, context, pr, row) for pr, row in zip(snapshots, rows))
            return rows

        return evaluate, evaluate_snapshots

    def installed(self, engine):
        """Put this session between `engine` (the running `pr_review.main`) and the world.

        `engine` is passed rather than imported: run as `python -m
        pr_review.main` the engine is `__main__`, and an imported
        `pr_review.main` would be a second copy that never runs.
        """
        from . import github, telemetry
        stack = ExitStack()

        def swap(owner, name, value):
            previous = getattr(owner, name)
            setattr(owner, name, value)
            stack.callback(setattr, owner, name, previous)

        try:
            # The boundary first: nothing below may run before writes are caught.
            swap(github, 'subprocess', self._boundary)
            stack.callback(setattr, self, '_installed', False)
            self._installed = True
            if isinstance(self, ReplaySession):
                swap(github, 'time', _NoSleep)
            swap(engine, 'clock', self.clock)
            self._fetch = telemetry.fetch
            swap(telemetry, 'fetch', lambda *a, **k: self.telemetry())
            swap(engine, 'ThreadPoolExecutor', _InOrder)
            evaluate, evaluate_snapshots = self._observe(engine)
            swap(engine, 'evaluate', evaluate)
            swap(engine, 'evaluate_snapshots', evaluate_snapshots)
        except BaseException:
            stack.close()
            raise
        return stack


class LiveSession(_Session):
    """Reads go to GitHub; writes stop here."""

    def __init__(self, now):
        super().__init__(now)
        self.telemetry_payload = None

    def _read(self, command, arguments, stdin, options):
        # Checked again where the call leaves: the only line in this module
        # that runs `gh`, and it runs nothing that is not a read.
        if not is_read(arguments, stdin):
            raise RecordingError('Refusing to send a call that is not a read')
        try:
            result = subprocess.run(command, input=stdin, **options)
        except subprocess.TimeoutExpired:
            return {'exit': None, 'stdout': '', 'stderr': '', 'raised': 'TimeoutExpired'}
        except OSError as error:
            return {'exit': None, 'stdout': '', 'stderr': '', 'raised': 'OSError', 'message': str(error)}
        return {'exit': result.returncode, 'stdout': result.stdout or '', 'stderr': result.stderr or ''}

    def telemetry(self):
        self.telemetry_reads += 1
        payload = self._fetch()
        self.telemetry_payload = copy.deepcopy(payload)
        return payload


class ReplaySession(_Session):
    """Reads come from a recording, each answer served once, in the order it was given."""

    def __init__(self, meta, calls):
        super().__init__(meta['clock'])
        self.meta = meta
        self.answers = {}
        for entry in calls:
            if entry['kind'] == 'read':
                self.answers.setdefault(self._key(entry['args'], entry['stdin']), []).append(entry)
        self.unrecorded = []

    @staticmethod
    def _key(arguments, stdin):
        return _same_form([list(arguments), _value(stdin)])

    def _read(self, command, arguments, stdin, options):
        queue = self.answers.get(self._key(arguments, stdin))
        if not queue:
            self.unrecorded.append({'args': arguments, 'stdin': stdin})
            raise RecordingError('A read the recording does not hold: ' + ' '.join(arguments))
        entry = queue.pop(0)
        return {k: entry[k] for k in ('exit', 'stdout', 'stderr', 'raised', 'message') if k in entry}

    def telemetry(self):
        self.telemetry_reads += 1
        return copy.deepcopy(self.meta.get('telemetry'))


# ---------------------------------------------------------------- recording

def _normalised_argv(args):
    """The command the engine ran, without anything that names a path on this machine."""
    argv = [args.command, '--repo', args.repo]
    if args.pr is not None:
        argv += ['--pr', str(args.pr)]
    if args.batch_size is not None:
        argv += ['--batch-size', str(args.batch_size)]
    if args.waiting_on_build:
        argv.append('--waiting-on-build')
    if args.user:
        argv += ['--user', args.user]
    argv += ['--format', args.format]
    return argv


def _engine_commit():
    try:
        found = subprocess.run(['git', 'rev-parse', 'HEAD'], cwd=Path(__file__).resolve().parent,
                               capture_output=True, text=True, timeout=10, check=False)
    except (OSError, subprocess.TimeoutExpired):
        return None
    return found.stdout.strip() or None


def _outcome(run):
    try:
        with redirect_stdout(io.StringIO()) as printed:
            code = run()
        return {'returned': code}, printed.getvalue(), None
    except SystemExit as error:
        raise RecordingError(f'The engine exited ({error.code}) before it ran') from error
    except RecordingError:
        raise
    except Exception as error:
        return {'raised': type(error).__name__, 'message': str(error)}, '', error


def record(args, argv, engine):
    """Run `engine.run(argv)` live at the boundary and write what it did into `args.record`."""
    directory = Path(args.record)
    if directory.exists() and (not directory.is_dir() or any(directory.iterdir())):
        raise RecordingError(f'{directory} already holds something; a recording is never written over')
    # The one reading of the real clock; every later one returns this instant.
    session = LiveSession(_iso(engine.clock()))
    with session.installed(engine):
        outcome, printed, error = _outcome(lambda: engine.run(argv, recording=session))
    meta = {
        'format': FORMAT,
        'repository': args.repo,
        'argv': _normalised_argv(args),
        'clock': session.now,
        'clock_reads': session.clock_reads,
        'policy': session.policy,
        'telemetry': session.telemetry_payload,
        'telemetry_reads': session.telemetry_reads,
        'outcome': outcome,
        'environment': {'GITHUB_RUN_ID': os.environ.get('GITHUB_RUN_ID')},
        'python': sys.version.split()[0],
        'engine_commit': _engine_commit(),
        'redacted': None,
    }
    session.printed = printed
    write_recording(directory, meta, session)
    sys.stdout.write(printed)
    writes = sum(1 for c in session.calls if c['kind'] == 'write')
    print(f'Recorded {len(session.calls)} calls ({writes} writes, none sent), '
          f'{len(session.verdicts)} verdicts into {directory}', file=sys.stderr)
    if error is not None:
        raise error
    return outcome['returned']


def write_recording(directory, meta, session):
    directory = Path(directory)
    directory.mkdir(parents=True, exist_ok=True)
    (directory / 'recording.json').write_text(json.dumps(meta, ensure_ascii=False, indent=1) + '\n')
    with open(directory / 'calls.jsonl', 'w') as out:
        for entry in session.calls:
            out.write(_dump(entry) + '\n')
    with open(directory / 'evaluations.jsonl', 'w') as out:
        for entry in session.evaluations:
            out.write(_dump(entry) + '\n')
    (directory / 'verdicts.json').write_text(json.dumps(session.verdicts, ensure_ascii=False, indent=1) + '\n')
    (directory / 'outputs.json').write_text(json.dumps(session.outputs, ensure_ascii=False, indent=1) + '\n')
    # The report as printed: the JSON form is what the dashboard reads.
    (directory / 'printed.txt').write_text(session.printed)


def load_recording(directory):
    directory = Path(directory)
    meta = json.loads((directory / 'recording.json').read_text())
    if meta.get('format') not in READABLE_FORMATS:
        raise RecordingError(f'{directory}: unknown recording format {meta.get("format")!r}')
    if (meta['format'] == FORMAT) != isinstance(meta.get('clock_reads'), list):
        raise RecordingError(f'{directory}: clock reads belong to format {FORMAT} and only to it')

    def lines(name):
        # One record per '\n'. Free text inside a record may hold the other
        # characters `splitlines` breaks at, such as U+2028.
        return [json.loads(line) for line in (directory / name).read_text().split('\n') if line]

    return {'meta': meta, 'calls': lines('calls.jsonl'), 'evaluations': lines('evaluations.jsonl'),
            'verdicts': json.loads((directory / 'verdicts.json').read_text()),
            'outputs': json.loads((directory / 'outputs.json').read_text()),
            'printed': (directory / 'printed.txt').read_text()}


# ---------------------------------------------------------------- replay

def rerun(recording):
    """Run the engine against a recording's reads, at its clock. Returns the session and outcome."""
    from . import main as engine
    meta = recording['meta']
    session = ReplaySession(meta, recording['calls'])
    with tempfile.TemporaryDirectory() as root:
        # The policy as it was when recorded, not as it is in this checkout.
        Path(root, 'policy.json').write_text(json.dumps(meta['policy']))
        Path(root, 'repositories.json').write_text(json.dumps({'version': 1, 'repositories': [
            {'repository': meta['repository'], 'policy': 'policy.json', 'mode': 'preview'}]}))
        argv = list(meta['argv']) + ['--policies-root', root]
        with ExitStack() as stack:
            stack.enter_context(session.installed(engine))
            # The one variable the engine's write path reads: it names the
            # workflow run an error status links to.
            stack.callback(_set_run_id, os.environ.get('GITHUB_RUN_ID'))
            _set_run_id((meta.get('environment') or {}).get('GITHUB_RUN_ID'))
            # What the engine says along the way was said when it was recorded.
            stack.enter_context(redirect_stderr(io.StringIO()))
            try:
                outcome, session.printed, _ = _outcome(lambda: engine.run(argv, recording=session))
            except RecordingError as error:
                outcome = {'raised': 'RecordingError', 'message': str(error)}
    return session, outcome


def _set_run_id(value):
    if value is None:
        os.environ.pop('GITHUB_RUN_ID', None)
    else:
        os.environ['GITHUB_RUN_ID'] = value


def _writes(calls, m):
    return [(c['args'], _same_form(m(_value(c['stdin'])))) for c in calls if c['kind'] == 'write']


def _sequence(calls, m):
    return [(c['kind'], c['args'], _same_form(m(_value(c['stdin'])))) for c in calls]


def _differing(before, after):
    """The keys whose values differ between two maps, over both maps' keys."""
    return [k for k in dict.fromkeys(list(before) + list(after))
            if _same_form(before.get(k)) != _same_form(after.get(k))]


def compare(recording, session, outcome, mask=False):
    """Every way the rerun differs from the recording, as readable lines. Empty means identical.

    This is Python checked against itself, so it is stricter than what
    another engine is held to: every recorded read must be asked again, in
    the same order. Maps compare by their entries, lists by their order.
    `mask` blanks the evidence and context prints, which redaction changes
    and nothing compares across runs.
    """
    m = _masked if mask else (lambda v: v)
    differences = []
    if outcome != recording['meta']['outcome']:
        differences.append(f"outcome: recorded {recording['meta']['outcome']}, replayed {outcome}")
    if session.unrecorded:
        differences.append(f'{len(session.unrecorded)} read(s) not in the recording, first: '
                           + ' '.join(session.unrecorded[0]['args']))
    left = [entry for queue in session.answers.values() for entry in queue]
    if left:
        differences.append(f'{len(left)} recorded read(s) never asked again, first #{left[0]["ordinal"]}: '
                           + ' '.join(left[0]['args']))
    recorded, replayed = recording['verdicts'], session.verdicts
    if _same_form(recorded) != _same_form(replayed):
        if len(recorded) != len(replayed):
            differences.append(f'verdicts: recorded {len(recorded)}, replayed {len(replayed)}')
        for before, after in zip(recorded, replayed):
            keys = _differing(before, after)
            if keys:
                differences.append(f"verdict #{before.get('number')}: {', '.join(keys)} differ "
                                   f"(recorded {before.get('state')}, replayed {after.get('state')})")
    old_writes, new_writes = _writes(recording['calls'], m), _writes(session.calls, m)
    if old_writes != new_writes:
        differences.append(f'writes: recorded {len(old_writes)}, replayed {len(new_writes)}')
        for index, (before, after) in enumerate(zip(old_writes, new_writes), 1):
            if before != after:
                differences.append(f"write {index}: recorded {' '.join(before[0])}, replayed {' '.join(after[0])}"
                                   + ('' if before[0] != after[0] else ' (same route, different body)'))
                break
    if _sequence(recording['calls'], m) != _sequence(session.calls, m) and old_writes == new_writes:
        differences.append('the calls were made in a different order')
    if _same_form(m(recording['outputs'])) != _same_form(m(session.outputs)):
        for before, after in zip(m(recording['outputs']), m(session.outputs)):
            for key in _differing(before, after):
                differences.append(f"output #{before.get('number')}: {key} differs")
        if len(recording['outputs']) != len(session.outputs):
            differences.append(f"outputs: recorded {len(recording['outputs'])}, replayed {len(session.outputs)}")
    old_results = [_same_form(e['result']) for e in recording['evaluations']]
    new_results = [_same_form(e['result']) for e in session.evaluations]
    if old_results != new_results:
        differences.append(f'evaluations: {sum(a != b for a, b in zip(old_results, new_results))} results differ, '
                           f'recorded {len(old_results)}, replayed {len(new_results)}')
    if not mask:
        old_evidence = [_same_form(e['pr']) for e in recording['evaluations']]
        new_evidence = [_same_form(e['pr']) for e in session.evaluations]
        if old_evidence != new_evidence:
            differences.append(f'evidence: {sum(a != b for a, b in zip(old_evidence, new_evidence))} '
                               'pull requests read differently')
    if getattr(session, 'printed', None) != recording['printed']:
        differences.append('the printed report differs')
    if 'clock_reads' in recording['meta']:
        recorded = [(r['site'], r['after']) for r in recording['meta']['clock_reads']]
        replayed = [(r['site'], r['after']) for r in session.clock_reads]
        if recorded != replayed:
            index = next((i for i, (before, after) in enumerate(zip(recorded, replayed)) if before != after),
                         min(len(recorded), len(replayed)))
            said = ['{} after call {}'.format(*reads[index]) if index < len(reads) else 'nothing'
                    for reads in (recorded, replayed)]
            differences.append(f'clock reads: recorded {len(recorded)}, replayed {len(replayed)}; '
                               f'read #{index + 1} recorded {said[0]}, replayed {said[1]}')
    if session.telemetry_reads != recording['meta'].get('telemetry_reads', 0):
        differences.append(f"status page read {session.telemetry_reads} time(s), "
                           f"recorded {recording['meta'].get('telemetry_reads', 0)}")
    return differences


def replay_recording(directory):
    recording = load_recording(directory)
    session, outcome = rerun(recording)
    differences = compare(recording, session, outcome)
    writes = sum(1 for c in recording['calls'] if c['kind'] == 'write')
    summary = (f"{len(recording['calls'])} calls, {writes} writes, {len(recording['verdicts'])} verdicts, "
               f"{len(recording['evaluations'])} evaluations")
    # What `fromisoformat` accepts changed between minor versions, so a
    # recording made on another one may not be this Python's to judge.
    made_on = str(recording['meta'].get('python') or '')
    if made_on.split('.')[:2] != [str(sys.version_info.major), str(sys.version_info.minor)]:
        summary += f' (recorded on Python {made_on or "unknown"})'
    if 'clock_reads' not in recording['meta']:
        summary += f" (format {recording['meta'].get('format')}: clock reads not recorded, not compared)"
    return summary, differences


# ---------------------------------------------------------------- evaluate-level cases

def evaluate_case_result(case):
    from .policy import evaluate
    return evaluate(copy.deepcopy(case['policy']), copy.deepcopy(case['pr']), case['admitted_at'], case['now'],
                    copy.deepcopy(case['telemetry_states']))


def replay_case(path):
    case = json.loads(Path(path).read_text())
    replayed = evaluate_case_result(case)
    keys = _differing(case['result'], replayed)
    return [f'{Path(path).name}: {", ".join(keys)} differ'] if keys else []


def _raised_messages():
    """What `policy.py` raises in its own words: (text, whole) for each, whole False when a value follows.

    Read from the source, so a message added there is known here without
    anybody remembering to list it.
    """
    import ast
    found = []
    tree = ast.parse((Path(__file__).resolve().parent / 'policy.py').read_text())
    for node in ast.walk(tree):
        if not (isinstance(node, ast.Raise) and isinstance(node.exc, ast.Call) and node.exc.args):
            continue
        message = node.exc.args[0]
        while isinstance(message, ast.BinOp):
            message = message.left
        if isinstance(message, ast.Constant) and isinstance(message.value, str):
            found.append((message.value, message is node.exc.args[0]))
        elif isinstance(message, ast.JoinedStr) and message.values and isinstance(message.values[0], ast.Constant):
            found.append((message.values[0].value, False))
    return found


def python_exception_text(result):
    """Whether a verdict's blockers carry the text of a Python exception rather than the engine's own words.

    The engine's own words are what `evaluate` says when it stops and what
    `policy.py` raises; anything else in a configuration error came from
    Python itself — a missing key, a timestamp `fromisoformat` refused.
    """
    if result.get('state') != 'configuration-error' or result.get('status') != 'error':
        return False
    reasons = [b for b in result.get('blockers') or [] if not b.startswith('Proceeded without')]
    if not reasons:
        return False
    if any(reasons[0].startswith(own) for own in _OWN_BLOCKERS):
        return False
    return not any(reasons[0] == text if whole else reasons[0].startswith(text)
                   for text, whole in _raised_messages())


# The instant the engine's own clock reads during a harvest.
HARVEST_CLOCK = '2026-10-01T00:00:00Z'
# The one Python cases are harvested on, the one CI runs. What `fromisoformat`
# accepts and how `str` classifies characters change between minor versions,
# and a case made on another one would pin that Python's answer instead.
HARVEST_PYTHON = (3, 12)
CASE_FILE = re.compile(r'[a-z-]+-[0-9a-f]{12}\.json')
# Test modules a harvest does not run, every other one under pr_review/tests
# being the engine's own tests:
# - test_conformance replays the cases a harvest writes, so it would only feed
#   the corpus back into itself;
# - test_repositories and test_roster check the live policies under policies/
#   — who owns what, which branches are governed, today — so what they hand
#   the engine changes with every policy edit, and a policy change must land
#   without re-harvesting the corpus. Tests that only need a realistic policy
#   read the frozen copy in pr_review/tests/fixtures/ instead.
NOT_HARVESTED = ('test_conformance', 'test_repositories', 'test_roster')


def case_name(case):
    body = _dump({k: case[k] for k in ('policy', 'pr', 'admitted_at', 'now', 'telemetry_states', 'result')})
    return f"{case['result'].get('state') or 'none'}-{hashlib.sha256(body.encode()).hexdigest()[:12]}.json"


# ---------------------------------------------------------------- function cases

# The pure functions `main.py` relies on besides `evaluate`, by module and then
# the name within it. Each is harvested like `evaluate`, into a directory of
# that name: what another engine writes and reads back has to come out of
# these byte for byte, and a whole run is a poor place to find which one did not.
FUNCTIONS = ('main.checklist_block', 'main.move_text', 'github.GitHub.state_comment_body', 'main.state_record',
             'main.diff_record', 'policy.admit', 'main.admission_conflicts', 'policy.receipt_print',
             'policy.diff_print', 'telemetry.head_state')
FUNCTION_CASE_FILE = re.compile(r'[0-9a-f]{12}\.json')


def _home(name):
    """The object that holds the function `name` names, and its attribute there."""
    import importlib
    if name not in FUNCTIONS:
        raise RecordingError(f'{name!r} is not a function this corpus holds')
    module, _, path = name.partition('.')
    owner = importlib.import_module(f'{__package__}.{module}')
    *outer, attribute = path.split('.')
    for part in outer:
        owner = getattr(owner, part)
    return owner, attribute


def _written(value):
    """An answer as a case holds it. A set has neither JSON nor an order of its own, so it is written sorted."""
    return sorted(value) if isinstance(value, (set, frozenset)) else value


def function_case_name(case):
    body = _dump({k: case[k] for k in ('function', 'inputs', 'output')})
    return f'{hashlib.sha256(body.encode()).hexdigest()[:12]}.json'


def function_case_result(case):
    owner, attribute = _home(case['function'])
    return _written(getattr(owner, attribute)(**copy.deepcopy(case['inputs'])))


def replay_function_case(path):
    """How a function case's output differs from what the function answers now, compared as JSON text."""
    path = Path(path)
    case = json.loads(path.read_text())
    label = f'{path.parent.name}/{path.name}'
    try:
        replayed = function_case_result(case)
    except Exception as error:
        return [f'{label}: raised {type(error).__name__}: {error}']
    return [] if _dump(replayed) == _dump(case['output']) else [f'{label}: output differs']


# ---------------------------------------------------------------- harvesting

def _observing(original, keep, counts, watching):
    """`original`, handing each call's inputs, by parameter name, and its answer to `keep`
    for as long as `watching[0]` holds.

    The inputs are copied before the call, which may change what it was
    given. A call whose inputs or answer `keep` cannot hold still runs, and
    is counted; so is one that raised, which is not a case.
    """
    import functools
    import inspect
    signature = inspect.signature(original)

    @functools.wraps(original)
    def observed(*arguments, **keywords):
        if not watching[0]:
            return original(*arguments, **keywords)
        try:
            inputs = copy.deepcopy((arguments, keywords))
        except Exception:
            inputs = None
        try:
            result = original(*arguments, **keywords)
        except BaseException:
            counts['raised'] += 1
            raise
        counts['calls'] += 1
        try:
            bound = signature.bind(*inputs[0], **inputs[1])
            bound.apply_defaults()
            keep(dict(bound.arguments), copy.deepcopy(result))
        except Exception:
            counts['not_serialisable'] += 1
        return result

    return observed


def _write_cases(directory, kept, pattern):
    directory.mkdir(parents=True, exist_ok=True)
    # Only files a harvest names: anything else in the directory is not its own.
    for stale in directory.glob('*.json'):
        if pattern.fullmatch(stale.name) and stale.name not in kept:
            stale.unlink()
    for name, case in kept.items():
        (directory / name).write_text(json.dumps(case, ensure_ascii=False, indent=1) + '\n')


def harvest(destination, functions, start='pr_review/tests'):
    """Run the test suite with every `policy.evaluate` call, and every call of
    each of `FUNCTIONS`, written down as a case.

    Each is replaced before the engine or any test module is imported, so
    every name they bind it to is the observing one, and the engine itself
    carries no hook for it.
    """
    import inspect
    import unittest
    if sys.version_info[:2] != HARVEST_PYTHON:
        raise RecordingError('harvest runs on Python {}.{} only; this is Python {}'.format(
            *HARVEST_PYTHON, sys.version.split()[0]))
    python = '{}.{}'.format(*sys.version_info[:2])
    bound = [name for name in ('pr_review.main', 'pr_review.aggregate') if name in sys.modules]
    if bound or any(name.startswith('test_') or name.startswith('pr_review.tests') for name in sys.modules):
        raise RecordingError('the harvested functions must be observed before the engine or the tests are imported')
    counts = {name: {'calls': 0, 'not_serialisable': 0, 'raised': 0} for name in ('evaluate',) + FUNCTIONS}
    cases, calls, current = {}, {name: {} for name in FUNCTIONS}, [None]
    # Names bound by `from .policy import ...` keep the observing function
    # after it is put back, and checking the cases below calls some of them:
    # only calls the suite makes are cases.
    watching = [True]

    def keep_evaluation(given, result):
        case = {'policy': given['policy'], 'pr': given['pr'], 'admitted_at': given['admitted_at'],
                'now': given['nowISO'], 'telemetry_states': given['telemetry_states'], 'result': result}
        entry = cases.setdefault(case_name(case), dict(case, tests=set()))
        if current[0]:
            entry['tests'].add(current[0])

    def keeping(function):
        def keep(given, output):
            case = {'function': function, 'inputs': given, 'output': _written(output)}
            entry = calls[function].setdefault(function_case_name(case), dict(case, tests=set()))
            if current[0]:
                entry['tests'].add(current[0])
        return keep

    class Tracking(unittest.TextTestResult):
        def startTest(self, test):
            current[0] = test.id()
            super().startTest(test)

    def tests(suite):
        for test in suite:
            yield from tests(test) if isinstance(test, unittest.TestSuite) else [test]

    from . import policy as rules
    with ExitStack() as stack:
        def swap(owner, attribute, value):
            previous = inspect.getattr_static(owner, attribute)
            stack.callback(setattr, owner, attribute, previous)
            setattr(owner, attribute, staticmethod(value) if isinstance(previous, staticmethod) else value)

        swap(rules, 'evaluate', _observing(rules.evaluate, keep_evaluation, counts['evaluate'], watching))
        # Those outside `main` first: importing `main` binds `admit` and
        # `diff_print` to names of its own, and those must be the observing ones.
        for name in sorted(FUNCTIONS, key=lambda name: name.startswith('main.')):
            owner, attribute = _home(name)
            swap(owner, attribute, _observing(getattr(owner, attribute), keeping(name), counts[name], watching))
        # A few tests leave the second evaluation inside `publish` on the real
        # clock. Fixed here, those cases are the same on every harvest.
        from . import main as engine
        swap(engine, 'clock', lambda: _instant(HARVEST_CLOCK))
        suite = unittest.TestSuite(test for test in tests(unittest.TestLoader().discover(start))
                                   if test.id().split('.')[0] not in NOT_HARVESTED)
        report = io.StringIO()
        with redirect_stdout(io.StringIO()), redirect_stderr(io.StringIO()):
            outcome = unittest.TextTestRunner(resultclass=Tracking, stream=report).run(suite)
        watching[0] = False
    if not outcome.wasSuccessful():
        sys.stderr.write(report.getvalue())
        raise RecordingError(f'the test suite failed ({len(outcome.failures)} failures, '
                             f'{len(outcome.errors)} errors); no cases written')
    # Only what survives JSON as it was: a tuple read back as a list, or a key
    # that was not a string, can decide differently, and a case that does not
    # replay against itself teaches the port nothing.
    kept = {}
    counts['evaluate']['not_replayable'] = 0
    for name, case in sorted(cases.items()):
        case = dict(case, tests=sorted(case['tests']), python_exception_text=python_exception_text(case['result']),
                    python=python)
        try:
            stored = json.loads(json.dumps(case, ensure_ascii=False))
            exact = not _differing(evaluate_case_result(stored), stored['result'])
        except Exception:
            exact = False
        if exact:
            kept[name] = case
        else:
            counts['evaluate']['not_replayable'] += 1
    _write_cases(Path(destination), kept, CASE_FILE)
    counts['evaluate']['cases'] = len(kept)
    for function in FUNCTIONS:
        kept = {}
        counts[function]['not_replayable'] = 0
        for name, case in sorted(calls[function].items()):
            case = dict(case, tests=sorted(case['tests']), python=python)
            try:
                stored = json.loads(json.dumps(case, ensure_ascii=False))
                exact = _dump(function_case_result(stored)) == _dump(stored['output'])
            except Exception:
                exact = False
            if exact:
                kept[name] = case
            else:
                counts[function]['not_replayable'] += 1
        _write_cases(Path(functions) / function, kept, FUNCTION_CASE_FILE)
        counts[function]['cases'] = len(kept)
    evaluated = counts.pop('evaluate')
    return {'tests': outcome.testsRun, 'calls': evaluated['calls'], 'cases': evaluated['cases'],
            'not_serialisable': evaluated['not_serialisable'], 'not_replayable': evaluated['not_replayable'],
            'raised': evaluated['raised'], 'functions': counts}


# ---------------------------------------------------------------- redaction

WRITE_LEVELS = {'admin', 'maintain', 'write'}
_WRITE_FLAGS = {'admin': False, 'maintain': False, 'push': True, 'triage': True, 'pull': True}
_READ_FLAGS = {'admin': False, 'maintain': False, 'push': False, 'triage': False, 'pull': True}


def _level(flags, legacy=None):
    from .github import _capability
    level = _capability(flags) if isinstance(flags, dict) else None
    if level is None and legacy in {'admin', 'write', 'read', 'none'}:
        level = legacy
    return level


_NO_FLAGS = dict.fromkeys(_WRITE_FLAGS, False)


def _redacted_access(flags, legacy=None):
    """(flags, legacy permission, role name) for the class the engine distinguishes.

    The engine asks two things of a level: whether it can write, and whether
    it is known at all. Everything with write access reads as `write`,
    everything without as `read`, and a level it cannot read as nothing at
    all; which of admin, maintain or a custom role a person holds is not
    public and decides nothing.
    """
    level = _level(flags, legacy)
    if level is None:
        return dict(_NO_FLAGS), None, None
    if level in WRITE_LEVELS:
        return dict(_WRITE_FLAGS), 'write', 'write'
    return dict(_READ_FLAGS), 'read', 'read'


# Fields an answer carries about whoever is asking rather than about the pull
# request: `author_association` says MEMBER for a private member of the
# organisation when the token can see them, a repository's `permissions` are
# the token holder's own, and `requested_teams` can name a secret team. The
# engine reads none of them — the redaction rerun proves it — and they go.
VIEWER_FIELDS = frozenset({'author_association', 'authorAssociation', 'permissions', 'requested_teams'})


def _without_viewer_fields(value):
    if isinstance(value, dict):
        return {k: _without_viewer_fields(v) for k, v in value.items() if k not in VIEWER_FIELDS}
    if isinstance(value, list):
        return [_without_viewer_fields(v) for v in value]
    return value


def _collapsed(holder, flags_key, access):
    """`holder` with its flags, legacy level and role name replaced by `access`, wherever it carries them."""
    holder = dict(holder)
    if isinstance(holder.get(flags_key), dict):
        holder[flags_key] = access[0]
    if 'permission' in holder:
        holder['permission'] = access[1]
    if 'role_name' in holder:
        holder['role_name'] = access[2]
    return holder


def _logins(value, found):
    if isinstance(value, dict):
        for key, item in value.items():
            if key == 'login' and isinstance(item, str):
                found.add(item.lower().removesuffix('[bot]'))
            else:
                _logins(item, found)
    elif isinstance(value, list):
        for item in value:
            _logins(item, found)


_LISTING = re.compile(r'repos/[A-Za-z0-9_.-]+/[A-Za-z0-9_.-]+/collaborators\?')
_ONE = re.compile(r'repos/[A-Za-z0-9_.-]+/[A-Za-z0-9_.-]+/collaborators/[^/]+/permission')


def _path(entry):
    parsed = _parse(entry['args'])
    return parsed[1] if parsed else ''


def redact_calls(calls, policy):
    """The calls with every permission level collapsed, every collaborator nobody here names
    dropped, and every field about the token's holder removed.

    Kept: anyone the policy names and anyone whose login appears in any other
    answer — an author, a reviewer, a commenter — which is everyone the engine
    can ask about. Dropped: everyone else on the listing, whose access and
    whose being there at all are not public.
    """
    named = {h.lower() for area in policy['areas'] for h in area['owners'] + area['reviewers']}
    fallback = policy['fallback'] if isinstance(policy['fallback'], dict) else {'owners': policy['fallback']}
    named |= {h.lower() for h in fallback.get('owners', []) + fallback.get('reviewers', [])}
    seen = set()
    for entry in calls:
        if entry['kind'] == 'read' and entry.get('stdout') and not _LISTING.match(_path(entry)) \
                and not _ONE.fullmatch(_path(entry)):
            try:
                _logins(json.loads(entry['stdout']), seen)
            except ValueError:
                pass
    keep = named | seen
    counts, dropped = {'levels_collapsed': 0, 'unreadable_levels': 0, 'answers_stripped': 0}, set()
    redacted = []
    for entry in calls:
        entry = dict(entry)
        path = _path(entry)
        if entry['kind'] != 'read' or not entry.get('stdout'):
            redacted.append(entry)
            continue
        try:
            body = json.loads(entry['stdout'])
        except ValueError:
            redacted.append(entry)
            continue
        if entry.get('exit') == 0 and _LISTING.match(path) and isinstance(body, list):
            pages = []
            for page in body:
                kept = []
                for person in page if isinstance(page, list) else []:
                    login = ((person or {}).get('login') or '').lower()
                    if login not in keep:
                        dropped.add(login)
                        continue
                    access = _redacted_access(person.get('permissions'))
                    counts['unreadable_levels' if access[1] is None else 'levels_collapsed'] += 1
                    kept.append(_collapsed(person, 'permissions', access))
                pages.append(kept if isinstance(page, list) else page)
            body = pages
        elif entry.get('exit') == 0 and _ONE.fullmatch(path) and isinstance(body, dict):
            user = body.get('user') if isinstance(body.get('user'), dict) else None
            access = _redacted_access(user.get('permissions') if user else None, body.get('permission'))
            counts['unreadable_levels' if access[1] is None else 'levels_collapsed'] += 1
            body = _collapsed(body, 'permissions', access)
            if user is not None:
                body['user'] = _collapsed(user, 'permissions', access)
        else:
            stripped = _without_viewer_fields(body)
            if stripped == body:
                redacted.append(entry)
                continue
            counts['answers_stripped'] += 1
            body = stripped
        entry['stdout'] = json.dumps(body, ensure_ascii=False, separators=(',', ':'))
        redacted.append(entry)
    counts['collaborators_dropped'] = len(dropped)
    return redacted, counts


def redact(source, destination):
    """Write a redacted copy of `source` to `destination`, proven to decide exactly as the original did."""
    destination = Path(destination)
    if destination.exists() and (not destination.is_dir() or any(destination.iterdir())):
        raise RecordingError(f'{destination} already holds something; a recording is never written over')
    original = load_recording(source)
    calls, counts = redact_calls(original['calls'], original['meta']['policy'])
    candidate = dict(original, calls=calls)
    session, outcome = rerun(candidate)
    # The same decisions, call for call. Only the evidence print moves, since
    # it is a hash over the evidence, permissions included; it is never
    # compared across runs.
    differences = compare(original, session, outcome, mask=True)
    for before, after in zip(original['evaluations'], session.evaluations):
        if _same_form(_without_permissions(before['pr'])) != _same_form(_without_permissions(after['pr'])):
            differences.append(f"evidence #{before['pr'].get('number')} changed beyond its permissions")
            break
    if differences:
        raise RecordingError('Redaction would change what the engine decides: ' + '; '.join(differences))
    # What was done, not how much: how many people a listing held that no
    # answer names is itself something about who has access.
    meta = dict(original['meta'], redacted={'permissions': 'collapsed to write or read',
                                            'collaborators': 'only those an answer or the policy names',
                                            'removed': sorted(VIEWER_FIELDS)})
    # The rerun asked the same calls in the same order and was answered from
    # the redacted reads, so its own log is the redacted recording, its
    # writes carrying the evidence print they now produce.
    rebased = _Session(meta['clock'])
    rebased.calls = session.calls
    rebased.verdicts, rebased.outputs, rebased.evaluations = session.verdicts, session.outputs, session.evaluations
    rebased.printed = session.printed
    write_recording(destination, meta, rebased)
    summary, differences = replay_recording(destination)
    if differences:
        raise RecordingError(f'{destination} does not replay after redaction: ' + '; '.join(differences))
    return counts, summary


def _without_permissions(pr):
    pr = dict(pr)
    pr['permissions'] = {k: (None if v is None else v in WRITE_LEVELS) for k, v in (pr.get('permissions') or {}).items()}
    return pr


# ---------------------------------------------------------------- command line

def _targets(paths):
    """Each recording, evaluate case or function case under `paths`, in a stable order."""
    for path in map(Path, paths):
        if (path / 'recording.json').is_file():
            yield 'recording', path
        elif path.is_file() and path.suffix == '.json':
            yield 'function' if 'function' in json.loads(path.read_text()) else 'case', path
        elif path.is_dir():
            for child in sorted(path.iterdir()):
                if child.is_dir() or (child.suffix == '.json' and child.name != 'recording.json'):
                    yield from _targets([child])
        else:
            raise RecordingError(f'{path}: neither a recording nor a case')


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    commands = parser.add_subparsers(dest='command', required=True)
    replay = commands.add_parser('replay', help='re-run the engine against recordings, evaluate and function cases')
    replay.add_argument('paths', nargs='+')
    redaction = commands.add_parser('redact', help='write a redacted copy of a live recording')
    redaction.add_argument('source')
    redaction.add_argument('destination')
    harvesting = commands.add_parser('harvest', help='write every evaluate call the test suite makes as a case, '
                                                     'and every call of the functions main.py relies on')
    harvesting.add_argument('destination', nargs='?', default='conformance/evaluate')
    harvesting.add_argument('--functions', default='conformance/functions', metavar='DIR',
                            help='where the function cases go, one directory per function')
    args = parser.parse_args(argv)
    if args.command == 'harvest':
        print(json.dumps(harvest(args.destination, args.functions)))
        return 0
    if args.command == 'redact':
        counts, summary = redact(args.source, args.destination)
        print(f'{args.destination}: redacted ({json.dumps(counts)}); replays identically: {summary}')
        return 0
    failed, seen = 0, {'recording': 0, 'case': 0, 'function': 0}
    for kind, path in _targets(args.paths):
        seen[kind] += 1
        if kind == 'recording':
            summary, differences = replay_recording(path)
            print(f"{'FAIL' if differences else 'ok  '} {path}: {summary}")
        elif kind == 'function':
            differences = replay_function_case(path)
        else:
            differences = replay_case(path)
        for line in differences:
            print(f'     {line}')
        failed += bool(differences)
    print(f"{seen['recording']} recording(s), {seen['case']} evaluate case(s), {seen['function']} function case(s), "
          f'{failed} failed')
    return 1 if failed or not sum(seen.values()) else 0


if __name__ == '__main__':
    try:
        sys.exit(main())
    except RecordingError as error:
        print(f'conformance: {error}', file=sys.stderr)
        sys.exit(1)
