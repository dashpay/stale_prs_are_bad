"""The conformance recorder: what it captures, that it never writes, and that a replay notices a change."""

import contextlib
import io
import json
import os
from datetime import datetime, timezone
from pathlib import Path
import subprocess
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
                    'createdAt': '2026-09-11T11:00:00Z', 'updatedAt': '2026-09-11T11:00:00Z',
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


class EvaluateCaseTests(unittest.TestCase):
    CASES = Path(__file__).resolve().parents[2] / 'conformance' / 'evaluate'

    def test_every_committed_case_replays(self):
        paths = sorted(self.CASES.glob('*.json'))
        self.assertGreater(len(paths), 100, 'the harvested cases are committed')
        failed = [line for path in paths for line in conformance.replay_case(path)]
        self.assertEqual(failed, [])

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
