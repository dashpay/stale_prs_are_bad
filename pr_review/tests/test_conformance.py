"""The conformance recorder: what it captures, that it never writes, and that a replay notices a change."""

import contextlib
import io
import json
import os
from datetime import datetime, timezone
from pathlib import Path
import subprocess
import sys
import tempfile
import unittest
from unittest.mock import patch

from pr_review import conformance, main
from pr_review.policy import CHECKLIST_START
from pr_review.tests.test_policy import HEAD, fixture

OTHER_HEAD = 'c' * 40
REPO = 'dashpay/platform'
# An even hour: the hourly batch of one picks the first pull request.
CLOCK = datetime(2026, 9, 12, 10, 0, 0, tzinfo=timezone.utc)


def _pr(number, author, head, created_at):
    # `author_association`, the repository's `permissions` and
    # `requested_teams` describe whoever holds the token, as GitHub answers it.
    return {'number': number, 'user': {'login': author, 'type': 'User'}, 'body': 'Some text.', 'labels': [],
            'assignees': [], 'head': {'sha': head, 'repo': {'full_name': REPO, 'permissions': {
                'admin': True, 'maintain': True, 'push': True, 'triage': True, 'pull': True}}},
            'base': {'ref': 'v4.2-dev', 'sha': 'b' * 40},
            'created_at': created_at, 'draft': False, 'state': 'open', 'html_url': f'https://x/{number}',
            'title': f'PR {number}', 'changed_files': 1, 'requested_reviewers': [],
            'requested_teams': [{'slug': 'secret-team'}], 'author_association': 'MEMBER'}


def _graph(**repository):
    return {'data': {'repository': repository}}


class FakeGh:
    """`gh api` over a small, fixed repository. Every command it is handed is kept, writes included."""

    def __init__(self):
        self.received = []
        self.flaky = set()
        self.prs = {1: _pr(1, 'owner', HEAD, '2026-09-10T00:00:00Z'),
                    2: _pr(2, 'reviewer', OTHER_HEAD, '2026-09-10T01:00:00Z')}
        self.reviews = {n: [{'id': 10 * n + 1, 'user': {'login': 'thepastaclaw'}, 'state': 'COMMENTED',
                             'commit_id': pr['head']['sha'], 'submitted_at': '2026-09-11T10:00:00Z',
                             'body': f"<!-- thepastaclaw-review-phase v1 phase=final sha={pr['head']['sha']} -->"},
                            {'id': 10 * n + 2, 'user': {'login': 'coderabbitai[bot]'}, 'state': 'APPROVED',
                             'commit_id': pr['head']['sha'], 'submitted_at': '2026-09-11T10:00:00Z', 'body': ''},
                            # A passer-by the listing does not name: asked about one by one.
                            {'id': 10 * n + 3, 'user': {'login': 'passerby'}, 'state': 'COMMENTED',
                             'commit_id': pr['head']['sha'], 'submitted_at': '2026-09-11T10:30:00Z', 'body': 'nice',
                             'author_association': 'MEMBER'}]
                        for n, pr in self.prs.items()}
        self.collaborators = [
            {'login': 'owner', 'permissions': {'admin': True, 'maintain': True, 'push': True, 'triage': True,
                                               'pull': True}, 'role_name': 'admin'},
            {'login': 'reviewer', 'permissions': {'admin': False, 'maintain': True, 'push': True, 'triage': True,
                                                  'pull': True}, 'role_name': 'maintain'},
            # A custom role whose flags say nothing: a level the engine cannot read.
            {'login': 'fallback', 'permissions': {'admin': False, 'maintain': False, 'push': False, 'triage': False,
                                                  'pull': False}, 'role_name': 'security-auditors'},
            {'login': 'hidden-member', 'permissions': {'admin': True, 'maintain': True, 'push': True,
                                                       'triage': True, 'pull': True}, 'role_name': 'admin'}]

    def __call__(self, command, input=None, **options):
        command = list(command)
        if command[:3] == ['git', 'rev-parse', 'HEAD']:
            return subprocess.CompletedProcess(command, 0, 'e' * 40 + '\n', '')
        self.received.append((command, input))
        if command[:2] != ['gh', 'api']:
            raise AssertionError(f'not a gh call: {command}')
        arguments = command[2:]
        method, path = arguments[1], arguments[2]
        if (method, path) in self.flaky:
            self.flaky.discard((method, path))
            return subprocess.CompletedProcess(command, 1, '', 'HTTP 502: Bad Gateway')
        body = self.answer(method, path, json.loads(input) if input else None)
        return subprocess.CompletedProcess(command, 0, json.dumps(body), '')

    def answer(self, method, path, payload):
        root = f'repos/{REPO}/'
        if method == 'POST' and path == 'graphql':
            query, variables = payload['query'], payload['variables']
            if 'fragment history' in query:
                return _graph(**{f'pr{n}': {'number': n, 'comments': {'totalCount': 1, 'nodes': [{
                    'databaseId': 100 + n, 'body': f"/self-reviewed {pr['head']['sha']}",
                    'createdAt': '2026-09-11T11:00:00Z', 'updatedAt': '2026-09-11T11:00:00Z', 'lastEditedAt': None,
                    'author': {'login': pr['user']['login'], '__typename': 'User'}, 'editor': None}]},
                    'timelineItems': {'nodes': []}} for n, pr in self.prs.items() if f'pr{n}:' in query})
            if 'reviewThreads' in query:
                return _graph(pullRequest={'reviewThreads': {'totalCount': 0, 'pageInfo': {
                    'hasNextPage': False, 'endCursor': None}, 'nodes': []}})
            if 'statusCheckRollup' in query:
                head = self.prs[variables['number']]['head']['sha']
                return _graph(pullRequest={'commits': {'nodes': [{'commit': {'oid': head,
                                                                              'statusCheckRollup': None}}]}})
            raise AssertionError('unknown query')
        if method != 'GET':
            raise AssertionError(f'a write reached gh: {method} {path}')
        tail = path[len(root):].split('?')[0]
        if tail == 'pulls':
            return [list(self.prs.values())]
        parts = tail.split('/')
        if parts[0] == 'pulls' and len(parts) == 2:
            return self.prs[int(parts[1])]
        if parts[0] == 'pulls' and parts[2] == 'files':
            return [[{'filename': f'packages/drive/{parts[1]}.rs', 'status': 'modified', 'sha': '1' * 40,
                      'patch': '@@ -1 +1 @@\n-a\n+b'}]]
        if parts[0] == 'pulls' and parts[2] == 'reviews':
            return [self.reviews[int(parts[1])]]
        if parts[0] == 'commits' and parts[2] == 'statuses':
            return [[]]
        if parts[0] == 'issues' and parts[2] == 'timeline':
            return [[]]
        if tail == 'collaborators':
            return [self.collaborators]
        if parts[0] == 'collaborators' and parts[2] == 'permission':
            return {'permission': 'read', 'role_name': 'triage',
                    'user': {'login': parts[1], 'role_name': 'triage',
                             'permissions': {'admin': False, 'maintain': False, 'push': False,
                                             'triage': True, 'pull': True}}}
        if parts[0] == 'labels':
            return {'name': parts[1]}
        raise AssertionError(f'unknown route {path}')

    def writes(self):
        """Every command that was not plainly a read, by a test of its own rather than the recorder's."""
        sent = []
        for command, stdin in self.received:
            method, path = command[3], command[4]
            query = json.loads(stdin)['query'] if path == 'graphql' and stdin else ''
            if not (method == 'GET' or (method == 'POST' and path == 'graphql' and 'mutation' not in query)):
                sent.append(command)
        return sent


@contextlib.contextmanager
def policies(policy=None):
    policy = policy or fixture()[0]
    with tempfile.TemporaryDirectory() as root:
        Path(root, 'platform.json').write_text(json.dumps(policy))
        Path(root, 'repositories.json').write_text(json.dumps({'version': 1, 'repositories': [
            {'repository': REPO, 'policy': 'platform.json', 'mode': 'preview'}]}))
        yield root


class RecordingCase(unittest.TestCase):
    def setUp(self):
        self.gh = FakeGh()
        self.scratch = tempfile.TemporaryDirectory()
        self.addCleanup(self.scratch.cleanup)
        # The fake stands where `gh` does: anything that reaches a real
        # process reaches it instead, and is kept.
        for target in ('subprocess.run',):
            started = patch(target, side_effect=self.gh)
            started.start()
            self.addCleanup(started.stop)
        quiet = patch('pr_review.github.time.sleep')
        quiet.start()
        self.addCleanup(quiet.stop)

    def record(self, *options, name='recording', clock=CLOCK, policy=None):
        directory = Path(self.scratch.name, name)
        with policies(policy) as root, patch.object(main, 'clock', return_value=clock), \
                contextlib.redirect_stdout(io.StringIO()), contextlib.redirect_stderr(io.StringIO()):
            main.run([*options, '--repo', REPO, '--policies-root', root, '--record', str(directory)])
        return directory

    def replay(self, directory):
        with contextlib.redirect_stderr(io.StringIO()):
            return conformance.replay_recording(directory)[1]

    @staticmethod
    def edit(directory, name, change):
        path = Path(directory, name)
        if name.endswith('.jsonl'):
            rows = [json.loads(line) for line in path.read_text().splitlines()]
            rows = change(rows) or rows
            path.write_text(''.join(json.dumps(row) + '\n' for row in rows))
        else:
            value = json.loads(path.read_text())
            value = change(value) or value
            path.write_text(json.dumps(value))


class RecordAndReplayTests(RecordingCase):
    def test_a_report_round_trips(self):
        directory = self.record('report')
        recording = conformance.load_recording(directory)
        self.assertEqual([v['state'] for v in recording['verdicts']], ['ready-to-merge', 'ready-for-human'])
        self.assertTrue(all(c['kind'] == 'read' for c in recording['calls']), 'a report writes nothing')
        self.assertEqual([c['ordinal'] for c in recording['calls']], list(range(1, len(recording['calls']) + 1)))
        first = recording['calls'][0]
        self.assertEqual(first['args'][:3], ['--method', 'GET', f'repos/{REPO}/pulls?state=open&per_page=100'])
        self.assertEqual((first['exit'], first['stderr']), (0, ''))
        self.assertIn('"number": 1', first['stdout'])
        self.assertEqual(len(recording['evaluations']), 2)
        self.assertEqual(recording['evaluations'][0]['pr']['permissions'],
                         {'owner': 'admin', 'passerby': 'triage', 'reviewer': 'maintain'})
        self.assertEqual(self.replay(directory), [])

    def test_a_dry_sync_captures_its_writes_and_sends_none(self):
        directory = self.record('sync')
        recording = conformance.load_recording(directory)
        writes = [c for c in recording['calls'] if c['kind'] == 'write']
        routes = [(c['args'][1], c['args'][2].split(REPO + '/')[1].split('/')[0]) for c in writes]
        self.assertIn(('PATCH', 'pulls'), routes, 'the checklist')
        self.assertIn(('POST', 'issues'), routes, 'the move comment and the label')
        self.assertIn(('POST', 'statuses'), routes)
        self.assertIn(('POST', 'pulls'), routes, 'the reviewer request')
        self.assertEqual(self.gh.writes(), [], 'nothing but reads reached gh')
        # The canned answers are what the engine read back: its own status,
        # under its own name, at the recorded instant.
        status = json.loads(next(c['stdout'] for c in writes if '/statuses/' in c['args'][2]))
        self.assertEqual((status['creator']['login'], status['created_at']), ('github-actions[bot]', '2026-09-12T10:00:00Z'))
        self.assertGreaterEqual(status['id'], conformance.CANNED_ID_BASE)
        self.assertEqual(self.replay(directory), [])

    def test_a_dry_sync_of_one_pull_request_round_trips(self):
        # `sync --pr N` is what an event on one pull request runs, over that
        # author's pull requests only: it has to replay as a sweep does, and
        # what it would write touches that pull request alone.
        directory = self.record('sync', '--pr', '2')
        recording = conformance.load_recording(directory)
        self.assertEqual(recording['meta']['argv'], ['sync', '--repo', REPO, '--pr', '2', '--format', 'markdown'])
        self.assertEqual([(v['number'], v['state']) for v in recording['verdicts']], [(2, 'ready-for-human')])
        routes = [(c['args'][1], c['args'][2].split(REPO + '/')[1]) for c in recording['calls'] if c['kind'] == 'write']
        self.assertEqual(routes, [('POST', f'statuses/{OTHER_HEAD}'), ('PATCH', 'pulls/2'),
                                  ('POST', 'issues/2/comments'), ('POST', 'issues/2/labels'),
                                  ('POST', 'pulls/2/requested_reviewers'), ('POST', f'statuses/{OTHER_HEAD}')])
        self.assertEqual(self.gh.writes(), [], 'nothing but reads reached gh')
        self.assertEqual(self.replay(directory), [])

    def test_the_outputs_are_the_bytes_the_write_path_sends(self):
        directory = self.record('sync')
        recording = conformance.load_recording(directory)
        outputs = {o['number']: o for o in recording['outputs']}
        bodies = [json.loads(c['stdin'])['body'] for c in recording['calls']
                  if c['kind'] == 'write' and c['args'][1] in {'POST', 'PATCH'} and c['stdin']
                  and 'body' in json.loads(c['stdin'])]
        for number, output in outputs.items():
            self.assertIn(output['comment'], bodies, f'#{number}: the record comment as written')
            self.assertTrue(output['checklist'].startswith(CHECKLIST_START))
            self.assertIn('Some text.\n\n' + output['checklist'], bodies, f'#{number}: the description as written')
            self.assertTrue(output['markers'].startswith('<!-- platform-pr-review-state-v1 {'))
            self.assertIn(output['record'], output['markers'])
            self.assertIsNotNone(output['diff_print'])
        self.assertEqual(outputs[2]['labels'], ['ready-for-human'])
        self.assertEqual(outputs[2]['status'], {'state': 'pending', 'context': 'PR Hygiene',
                                                'description': 'ready-for-human'})

    def test_a_flaky_read_is_recorded_with_its_retry(self):
        self.gh.flaky.add(('GET', f'repos/{REPO}/pulls/1'))
        directory = self.record('report', '--pr', '1')
        calls = conformance.load_recording(directory)['calls']
        tries = [c for c in calls if c['args'][2] == f'repos/{REPO}/pulls/1']
        self.assertEqual([c['exit'] for c in tries[:2]], [1, 0])
        self.assertEqual(tries[0]['stderr'], 'HTTP 502: Bad Gateway')
        self.assertEqual(self.replay(directory), [])

    def test_the_status_page_is_read_once_and_replayed(self):
        policy = fixture()[0]
        policy['bot_timeouts'] = {'nudge_after_hours': 2, 'waive_after_hours': 16}
        page = {'schema_version': 1, 'data_as_of': '2026-09-12T09:59:00Z'}
        with patch('pr_review.telemetry.fetch', return_value=page) as fetch:
            directory = self.record('report', policy=policy)
        fetch.assert_called_once()
        meta = json.loads(Path(directory, 'recording.json').read_text())
        self.assertEqual((meta['telemetry'], meta['telemetry_reads']), (page, 1))
        with patch('pr_review.telemetry.fetch', side_effect=AssertionError('read live during a replay')):
            self.assertEqual(self.replay(directory), [])


class SafetyTests(RecordingCase):
    def test_record_refuses_apply_and_commands_that_are_not_a_report_or_a_sync(self):
        environment = {'GITHUB_ACTIONS': 'true', 'GITHUB_REPOSITORY': REPO, 'PR_REVIEW_AUTOMATION_ENABLED': 'true'}
        for options in (['sync', '--apply'], ['validate'], ['codeowners']):
            with self.subTest(options=options), patch.dict(os.environ, environment), policies() as root, \
                    contextlib.redirect_stderr(io.StringIO()), self.assertRaises(SystemExit):
                main.run([*options, '--repo', REPO, '--policies-root', root,
                          '--record', str(Path(self.scratch.name, 'refused'))])
        self.assertEqual(self.gh.received, [])
        self.assertFalse(Path(self.scratch.name, 'refused').exists())

    def test_a_recording_that_does_not_hold_the_boundary_never_runs_the_engine(self):
        session = conformance.LiveSession('2026-09-12T10:00:00Z')
        with policies() as root, self.assertRaises(RuntimeError):
            main.run(['sync', '--repo', REPO, '--policies-root', root], recording=session)
        self.assertEqual(self.gh.received, [])

    def test_a_recording_is_never_written_over(self):
        directory = self.record('report')
        with self.assertRaises(conformance.RecordingError):
            self.record('report')
        self.assertTrue(Path(directory, 'recording.json').is_file())

    def test_every_patch_is_undone(self):
        from pr_review import github, telemetry
        before = (github.subprocess, github.time, main.clock, telemetry.fetch, main.ThreadPoolExecutor,
                  main.evaluate, main.evaluate_snapshots)
        directory = self.record('sync')
        self.replay(directory)
        self.assertEqual((github.subprocess, github.time, main.clock, telemetry.fetch, main.ThreadPoolExecutor,
                          main.evaluate, main.evaluate_snapshots), before)

    def test_only_a_proven_read_is_a_read(self):
        query = json.dumps({'query': 'query($owner:String!) { viewer { login } }', 'variables': {}})
        reads = [(['--method', 'GET', 'repos/a/b/pulls/1'], None),
                 (['--method', 'GET', 'repos/a/b/pulls?state=open&per_page=100', '--paginate', '--slurp'], None),
                 (['--method', 'POST', 'graphql', '--input', '-'], query)]
        writes = [(['--method', 'POST', 'repos/a/b/issues/1/comments', '--input', '-'], '{"body": "x"}'),
                  (['--method', 'PATCH', 'repos/a/b/pulls/1', '--input', '-'], '{"body": "x"}'),
                  (['--method', 'DELETE', 'repos/a/b/issues/comments/5'], None),
                  (['--method', 'PUT', 'repos/a/b/pulls/1/merge'], None),
                  (['--method', 'POST', 'graphql', '--input', '-'],
                   json.dumps({'query': 'mutation { addComment(input: {}) { clientMutationId } }'})),
                  (['--method', 'POST', 'graphql', '--input', '-'],
                   json.dumps({'query': 'query { a } mutation { b }'})),
                  (['--method', 'POST', 'graphql', '--input', '-'],
                   json.dumps({'query': 'query { a }', 'operationName': 'b'})),
                  (['--method', 'GET', 'repos/a/b/pulls/1', '--input', '-'], '{}'),
                  (['--method', 'GET', 'repos/a/b/pulls/1', '-f', 'x=1'], None),
                  (['repos/a/b/pulls/1'], None),
                  (['--method', 'GET', '--method', 'POST', 'repos/a/b/x'], None),
                  (['--method', 'GET', '/graphql?query=x'], None),
                  (['--method', 'GET', 'https://api.github.com/repos/a/b/pulls/1'], None),
                  (['--method', 'GET', 'user'], None),
                  (['--method', 'POST', 'graphql', '--input', '-'],
                   '{"query": "mutation { a }", "query": "query { b }"}')]
        for arguments, stdin in reads:
            self.assertTrue(conformance.is_read(arguments, stdin), arguments)
        for arguments, stdin in writes:
            self.assertFalse(conformance.is_read(arguments, stdin), (arguments, stdin))

    def test_a_write_to_an_unplanned_route_stops_the_recording(self):
        with self.assertRaises(conformance.RecordingError):
            conformance.canned_answer(['--method', 'PUT', f'repos/{REPO}/pulls/1/merge'], None,
                                      '2026-09-12T10:00:00Z', 1)


class ClockTests(RecordingCase):
    def test_the_engine_reads_the_time_nowhere_but_its_clock(self):
        # A reading anywhere else would run on real time inside a recording,
        # and a replay at the recorded instant would no longer reproduce it.
        reads = ('datetime.now(', 'datetime.utcnow(', 'datetime.today(', 'date.today(', 'time.time(',
                 'time.monotonic(', 'time.perf_counter(', 'time.localtime(', 'time.gmtime(')
        engine = Path(main.__file__).resolve().parent
        found = [f'{path.name}:{number}' for path in sorted(engine.glob('*.py')) if path.name != 'conformance.py'
                 for number, line in enumerate(path.read_text().splitlines(), 1)
                 if any(read in line for read in reads)]
        self.assertEqual(found, ['main.py:30'], 'main.clock() is the only reading')

    def test_the_clock_is_read_once_and_every_reading_returns_it(self):
        directory = self.record('report')
        meta = json.loads(Path(directory, 'recording.json').read_text())
        self.assertEqual(meta['clock'], '2026-09-12T10:00:00Z')
        verdicts = json.loads(Path(directory, 'verdicts.json').read_text())
        # A first admission is dated by the clock.
        self.assertEqual({v['admitted_at'] for v in verdicts}, {'2026-09-12T10:00:00Z'})
        self.assertEqual({e['now'] for e in conformance.load_recording(directory)['evaluations']},
                         {'2026-09-12T10:00:00Z'})

    def test_every_clock_read_is_logged_with_where_the_engine_asked(self):
        # Every read returns the recorded instant, so an engine that reads the
        # time where Python does not still gives the same answers; only this
        # log tells the two apart. `after` is the last call made before it.
        directory = self.record('sync', '--pr', '1')
        meta = json.loads(Path(directory, 'recording.json').read_text())
        self.assertEqual(meta['format'], conformance.FORMAT)
        self.assertEqual(meta['clock_reads'], [{'site': 'collect', 'after': 2}, {'site': 'run', 'after': 10}])
        history = conformance.load_recording(directory)['calls'][1]
        self.assertIn('fragment history', history['stdin'], 'admission is dated right after the histories are read')
        self.assertEqual(self.replay(directory), [])

    def test_a_clock_read_somewhere_else_is_caught(self):
        directory = self.record('sync', '--pr', '1')
        self.edit(directory, 'recording.json', lambda meta: meta['clock_reads'][1].update(site='collect'))
        self.assertIn('clock reads: recorded 2, replayed 2; read #2 recorded collect after call 10, '
                      'replayed run after call 10', self.replay(directory))

    def test_an_extra_clock_read_is_caught(self):
        directory = self.record('sync', '--pr', '1')
        # The same answers, read at two more points: `collect` and
        # `evaluate_snapshots` both ask who holds too many slots.
        real = main.admission_conflicts
        with patch.object(main, 'admission_conflicts', side_effect=lambda *a: (main.utc_now(), real(*a))[1]):
            differences = self.replay(directory)
        self.assertEqual(differences, ['clock reads: recorded 2, replayed 4; read #2 recorded run after call 10, '
                                       'replayed collect after call 2'])

    def test_a_recording_made_before_clock_reads_were_logged_still_replays(self):
        directory = self.record('report')

        def first_format(meta):
            meta['format'] = 1
            del meta['clock_reads']
        self.edit(directory, 'recording.json', first_format)
        with contextlib.redirect_stderr(io.StringIO()):
            summary, differences = conformance.replay_recording(directory)
        self.assertEqual(differences, [])
        self.assertIn('format 1: clock reads not recorded, not compared', summary)
        self.edit(directory, 'recording.json', lambda meta: meta.update(format=3))
        with self.assertRaisesRegex(conformance.RecordingError, 'unknown recording format 3'):
            conformance.load_recording(directory)
        # A current recording that lost its log would replay with nothing to
        # compare it against, and pass.
        self.edit(directory, 'recording.json', lambda meta: meta.update(format=conformance.FORMAT))
        with self.assertRaisesRegex(conformance.RecordingError, 'clock reads belong to format 2'):
            conformance.load_recording(directory)

    def test_a_replay_runs_at_the_recorded_instant_whatever_the_time_now(self):
        directory = self.record('report')
        with patch.object(main, 'clock', side_effect=AssertionError('the real clock was read')):
            self.assertEqual(self.replay(directory), [])

    def test_the_batch_rotation_reads_the_injected_clock(self):
        directory = self.record('sync', '--batch-size', '1')
        self.assertEqual([v['number'] for v in json.loads(Path(directory, 'verdicts.json').read_text())], [1])
        self.assertEqual(self.replay(directory), [])
        # An hour later the rotation reaches the other pull request, whose
        # history this recording never read.
        self.edit(directory, 'recording.json', lambda meta: meta.update(clock='2026-09-12T11:00:00Z'))
        differences = self.replay(directory)
        self.assertTrue(any('not in the recording' in d for d in differences), differences)
        self.assertIn('verdicts: recorded 1, replayed 0', differences)


class MutationTests(RecordingCase):
    """A replay that cannot fail proves nothing: each of these must be caught."""

    def test_a_changed_verdict_is_caught(self):
        directory = self.record('report')
        self.edit(directory, 'verdicts.json', lambda rows: rows[1].update(state='ready-to-merge'))
        self.assertTrue(any(d.startswith('verdict #2: state') for d in self.replay(directory)))

    def test_a_changed_write_order_is_caught(self):
        directory = self.record('sync')

        def swap(calls):
            first, second = [i for i, c in enumerate(calls) if c['kind'] == 'write'][:2]
            calls[first], calls[second] = calls[second], calls[first]
        self.edit(directory, 'calls.jsonl', swap)
        differences = self.replay(directory)
        self.assertTrue(any(d.startswith('writes:') or d.startswith('write 1') for d in differences), differences)

    def test_requests_are_matched_on_what_they_say_not_how_python_spelled_them(self):
        # Another engine writes its own JSON: compact, keys in another order.
        # The same request must still find its answer, and the same write
        # still count as the same.
        directory = self.record('sync')

        def respell(calls):
            for call in calls:
                if call['stdin']:
                    value = json.loads(call['stdin'])
                    call['stdin'] = json.dumps(dict(reversed(list(value.items()))), separators=(',', ':'))
        self.edit(directory, 'calls.jsonl', respell)
        self.assertEqual(self.replay(directory), [])

    def test_maps_compare_by_their_entries_not_their_order(self):
        directory = self.record('report')
        self.edit(directory, 'verdicts.json', lambda rows: [dict(reversed(list(row.items()))) for row in rows])
        self.assertEqual(self.replay(directory), [])

    def test_a_changed_write_body_is_caught(self):
        directory = self.record('sync')

        def reword(calls):
            patch_call = next(c for c in calls if c['kind'] == 'write' and c['args'][1] == 'PATCH')
            patch_call['stdin'] = patch_call['stdin'].replace('Build green', 'Build grene')
        self.edit(directory, 'calls.jsonl', reword)
        self.assertTrue(any('same route, different body' in d for d in self.replay(directory)))

    def test_a_changed_answer_from_github_is_caught(self):
        directory = self.record('report')

        def withdraw(calls):
            reviews = next(c for c in calls if c['args'][2].startswith(f'repos/{REPO}/pulls/1/reviews'))
            reviews['stdout'] = reviews['stdout'].replace('"APPROVED"', '"COMMENTED"')
        self.edit(directory, 'calls.jsonl', withdraw)
        differences = self.replay(directory)
        self.assertTrue(any(d.startswith('verdict #1: state') for d in differences), differences)

    def test_an_output_with_more_or_less_in_it_is_caught(self):
        directory = self.record('sync')
        self.edit(directory, 'outputs.json', lambda rows: rows[0].update(extra='surplus'))
        self.assertIn('output #1: extra differs', self.replay(directory))

    def test_a_changed_printed_report_is_caught(self):
        directory = self.record('report')
        path = Path(directory, 'printed.txt')
        self.assertIn('| [#2: PR 2](https://x/2) | reviewer |', path.read_text())
        path.write_text(path.read_text().replace('ready-for-human', 'ready-to-merge'))
        self.assertIn('the printed report differs', self.replay(directory))

    def test_a_read_the_recording_lacks_fails_the_replay(self):
        directory = self.record('report')
        self.edit(directory, 'calls.jsonl', lambda calls: [c for c in calls if '/reviews' not in c['args'][2]])
        differences = self.replay(directory)
        self.assertTrue(any('not in the recording' in d for d in differences), differences)


class RedactionTests(RecordingCase):
    def redacted(self, *options):
        source = self.record(*options)
        destination = Path(self.scratch.name, 'redacted')
        with contextlib.redirect_stderr(io.StringIO()):
            counts, _ = conformance.redact(source, destination)
        return source, destination, counts

    def test_levels_collapse_and_strangers_go_while_every_decision_stays(self):
        source, destination, counts = self.redacted('sync')
        self.assertEqual(counts['collaborators_dropped'], 1)
        self.assertGreaterEqual(counts['unreadable_levels'], 1, 'one per listing read: a sync reads it again')
        text = ''.join(p.read_text() for p in destination.iterdir())
        self.assertNotIn('hidden-member', text)
        # The custom role nobody could read, nested role names, and whatever
        # GitHub said about the token's own holder all go.
        for secret in ('security-auditors', '"role_name":"triage"', '"role_name":"maintain"', '"role_name":"admin"',
                       '"maintain":true', '"admin":true', 'MEMBER', 'secret-team'):
            self.assertNotIn(secret, text.replace(' ', '').replace('\\"', '"'), secret)
        evidence = conformance.load_recording(destination)['evaluations'][0]['pr']['permissions']
        self.assertEqual(evidence, {'owner': 'write', 'passerby': 'read', 'reviewer': 'write'})
        before, after = (json.loads(Path(d, 'verdicts.json').read_text()) for d in (source, destination))
        self.assertEqual(before, after)
        self.assertEqual(self.replay(destination), [])
        meta = json.loads(Path(destination, 'recording.json').read_text())
        self.assertEqual(meta['redacted']['permissions'], 'collapsed to write or read')
        self.assertNotIn('collaborators_dropped', json.dumps(meta), 'how many people were dropped is itself telling')

    def test_a_redaction_that_would_change_a_decision_is_refused(self):
        source = self.record('report')
        # A redaction that took write access away would turn a ready pull
        # request into a configuration error, and must never be written.
        with patch.object(conformance, '_redacted_access', return_value=(dict(conformance._READ_FLAGS), 'read', 'read')):
            with self.assertRaises(conformance.RecordingError):
                conformance.redact(source, Path(self.scratch.name, 'refused'))
        self.assertFalse(Path(self.scratch.name, 'refused').exists())


class SyntheticRecordingTests(unittest.TestCase):
    """The recordings `conformance/synthetic/generate.py` writes, which another engine rebuilds its reads from."""

    HERE = Path(__file__).resolve().parents[2] / 'conformance' / 'synthetic'

    def recordings(self):
        return sorted(path for path in self.HERE.iterdir() if (path / 'recording.json').is_file())

    def test_every_committed_synthetic_recording_replays(self):
        paths = self.recordings()
        self.assertGreaterEqual(len(paths), 7, 'the synthetic recordings are committed')
        for path in paths:
            with self.subTest(recording=path.name), contextlib.redirect_stderr(io.StringIO()):
                self.assertEqual(conformance.replay_recording(path)[1], [])

    @unittest.skipUnless(sys.version_info[:2] == (3, 12), 'the recordings are made on Python 3.12 only')
    def test_the_committed_synthetic_recordings_are_what_the_generator_writes(self):
        # A change to the engine that changes what it reads, or how, has to
        # carry the regenerated recordings with it, or another engine is held
        # to reads this one no longer makes.
        import importlib.util
        spec = importlib.util.spec_from_file_location('synthetic_generate', self.HERE / 'generate.py')
        generator = importlib.util.module_from_spec(spec)
        spec.loader.exec_module(generator)
        with tempfile.TemporaryDirectory() as root:
            generator.generate(Path(root))
            fresh = sorted(path for path in Path(root).iterdir())
            self.assertEqual([path.name for path in fresh], [path.name for path in self.recordings()])
            for path in fresh:
                for file in generator.FILES:
                    with self.subTest(recording=path.name, file=file):
                        self.assertEqual((path / file).read_text(), (self.HERE / path.name / file).read_text(),
                                         'stale: run conformance/synthetic/generate.py and commit the result')


class EvaluateCaseTests(unittest.TestCase):
    CASES = Path(__file__).resolve().parents[2] / 'conformance' / 'evaluate'

    def test_every_committed_case_replays(self):
        paths = sorted(self.CASES.glob('*.json'))
        self.assertGreater(len(paths), 100, 'the harvested cases are committed')
        failed = [line for path in paths for line in conformance.replay_case(path)]
        self.assertEqual(failed, [])

    def test_every_committed_case_says_which_python_made_it(self):
        made_on = {json.loads(path.read_text()).get('python') for path in self.CASES.glob('*.json')}
        self.assertEqual(made_on, {'{}.{}'.format(*conformance.HARVEST_PYTHON)})

    def test_a_harvest_on_another_python_is_refused(self):
        # A case pins what the Python that made it answered, and `fromisoformat`
        # answers differently from one minor version to the next: a corpus
        # harvested on the wrong one would hold another engine to that one.
        with tempfile.TemporaryDirectory() as root:
            with patch.object(conformance.sys, 'version_info', (3, 13, 0, 'final', 0)), \
                    self.assertRaisesRegex(conformance.RecordingError, r'Python 3\.12 only'):
                conformance.harvest(Path(root, 'evaluate'), Path(root, 'functions'))
            self.assertEqual(list(Path(root).iterdir()), [], 'nothing is written')

    def test_only_text_python_wrote_is_marked_as_an_exception(self):
        policy, pr = fixture()
        own = conformance.evaluate_case_result({'policy': dict(policy, unknown=True), 'pr': pr,
                                                'admitted_at': None, 'now': '2026-09-11T12:00:00Z',
                                                'telemetry_states': None})
        self.assertEqual(own['blockers'], ['Unknown or missing policy fields'])
        self.assertFalse(conformance.python_exception_text(own), 'raised by policy.py in its own words')
        del pr['files'][0]['filename']
        python = conformance.evaluate_case_result({'policy': policy, 'pr': pr, 'admitted_at': None,
                                                   'now': '2026-09-11T12:00:00Z', 'telemetry_states': None})
        self.assertEqual(python['blockers'], ["'filename'"])
        self.assertTrue(conformance.python_exception_text(python), "a KeyError's text is Python's")

    def test_a_changed_case_result_is_caught(self):
        policy, pr = fixture()
        case = {'policy': policy, 'pr': pr, 'admitted_at': '2026-09-11T12:00:00Z', 'now': '2026-09-11T12:00:00Z',
                'telemetry_states': None}
        case['result'] = conformance.evaluate_case_result(case)
        with tempfile.TemporaryDirectory() as root:
            path = Path(root, conformance.case_name(case))
            path.write_text(json.dumps(case))
            self.assertEqual(conformance.replay_case(path), [])
            case['result']['blockers'] = ['something else']
            path.write_text(json.dumps(case))
            self.assertEqual(conformance.replay_case(path), [f'{path.name}: blockers differ'])


class FunctionCaseTests(unittest.TestCase):
    CASES = Path(__file__).resolve().parents[2] / 'conformance' / 'functions'

    def cases(self, function):
        return sorted((self.CASES / function).glob('*.json'))

    def replay(self, paths):
        return [line for path in paths for line in conformance.replay_function_case(path)]

    def test_every_function_has_committed_cases_and_every_one_replays(self):
        for function in conformance.FUNCTIONS:
            with self.subTest(function=function):
                paths = self.cases(function)
                self.assertTrue(paths, 'harvested and committed')
                self.assertEqual(self.replay(paths), [])

    def test_one_character_more_in_what_the_engine_writes_fails_its_cases(self):
        # The checklist and the move comment are read back on the next run and
        # compared as text: a port one character off rewrites them every run.
        for function in ('main.checklist_block', 'main.move_text'):
            name = function.split('.')[1]
            real = getattr(main, name)
            with self.subTest(function=function), \
                    patch.object(main, name, side_effect=lambda *a, real=real, **k: (real(*a, **k) or '') + ' '):
                failed = self.replay(self.cases(function))
            self.assertTrue(failed and all(line.endswith('output differs') for line in failed), failed)

    def test_an_edited_output_is_caught(self):
        source = next(p for p in self.cases('policy.admit') if len(json.loads(p.read_text())['output']) > 1)
        case = json.loads(source.read_text())
        with tempfile.TemporaryDirectory() as root:
            path = Path(root, 'policy.admit', source.name)
            path.parent.mkdir()
            path.write_text(json.dumps(case))
            self.assertEqual(self.replay([path]), [])
            # The same entries in another order are other bytes.
            case['output'] = dict(reversed(list(case['output'].items())))
            path.write_text(json.dumps(case))
            self.assertEqual(self.replay([path]), [f'policy.admit/{source.name}: output differs'])

    def test_a_set_is_written_in_order(self):
        # A set iterates in an order that moves with the hash seed; a case
        # written in that order would differ from one harvest to the next.
        self.assertEqual(conformance._written({'b', 'a', 'c'}), ['a', 'b', 'c'])
        written = [json.loads(p.read_text())['output'] for p in self.cases('main.admission_conflicts')]
        self.assertTrue(any(written), 'a case where somebody holds too many admissions')

    def test_a_case_names_only_the_tests_that_made_it(self):
        # Once the suite has run, checking the cases calls some functions again
        # through names `main` bound to the observing ones. Were those calls
        # observed, each would be credited to whichever test ran last: the
        # workflow tests, which read YAML and call nothing harvested here.
        named = {test for path in self.CASES.rglob('*.json') for test in json.loads(path.read_text())['tests']}
        self.assertEqual({test for test in named if test.startswith('test_workflow.')}, set())

    def test_no_case_comes_from_a_module_the_harvest_does_not_run(self):
        # The live-policy tests hand the engine whatever policies/ says today;
        # a case of theirs would turn every policy edit into a corpus change.
        paths = [*(self.CASES.parent / 'evaluate').glob('*.json'), *self.CASES.glob('*/*.json')]
        named = {test.split('.')[0] for path in paths for test in json.loads(path.read_text())['tests']}
        self.assertEqual(named & set(conformance.NOT_HARVESTED), set())

    def test_a_call_made_after_the_suite_is_neither_kept_nor_counted(self):
        watching, kept = [True], []
        counts = {'calls': 0, 'not_serialisable': 0, 'raised': 0}
        observed = conformance._observing(lambda value: value * 2, lambda given, output: kept.append((given, output)),
                                          counts, watching)
        self.assertEqual(observed(1), 2)
        watching[0] = False
        self.assertEqual(observed(2), 4, 'still the function it stands for')
        self.assertEqual((kept, counts['calls']), ([({'value': 1}, 2)], 1))

    def test_a_case_can_only_name_a_function_the_corpus_holds(self):
        # A case is data: it must not be able to name any callable it likes.
        with self.assertRaises(conformance.RecordingError):
            conformance.function_case_result({'function': 'main.os.system', 'inputs': {'command': 'true'}})

    def test_replay_counts_function_cases_as_their_own_kind(self):
        with contextlib.redirect_stdout(io.StringIO()) as printed:
            code = conformance.main(['replay', str(self.CASES / 'telemetry.head_state')])
        self.assertEqual(code, 0)
        count = len(self.cases('telemetry.head_state'))
        self.assertIn(f'0 recording(s), 0 evaluate case(s), {count} function case(s), 0 failed', printed.getvalue())
