"""The verdicts a run hands to the dashboard, and why handing them over can never touch the gate.

The status each run writes on a head is the gate; the dashboard is a view of
it. So the hand-over comes after every status, and nothing that goes wrong in
it may change what a run publishes, what it prints or how it ends.
"""

import contextlib
import copy
import io
import json
import os
from pathlib import Path
import tempfile
import unittest
from unittest.mock import Mock, patch

from pr_review import main, push
from pr_review.tests.test_policy import NOW, fixture

READ_AT = '2026-09-11T11:59:00Z'


class PayloadTests(unittest.TestCase):
    def test_each_verdict_is_the_row_a_report_prints(self):
        # The service parses a pushed verdict the way the analyzer parses a
        # report row, so the two must be the same object, not a re-shaping.
        policy, pr = fixture()
        other = dict(copy.deepcopy(pr), number=2, head='c' * 40)
        rows = main.evaluate_snapshots(policy, [pr, other], [pr, other], [pr, other], NOW)
        closed_elsewhere = dict(copy.deepcopy(pr), number=7, head='d' * 40)

        payload = push.payload('dashpay/platform', READ_AT, [other, closed_elsewhere, pr], rows)

        self.assertEqual(payload['repo'], 'dashpay/platform')
        self.assertEqual(payload['read_at'], READ_AT)
        self.assertEqual(payload['open'], [1, 2, 7], 'every open pull request, not only the ones evaluated')
        self.assertEqual([(v['number'], v['head']) for v in payload['verdicts']], [(1, pr['head']), (2, 'c' * 40)])
        printed = json.loads(json.dumps({'pull_requests': main.selected_rows(rows)}))['pull_requests']
        self.assertEqual([v['verdict'] for v in json.loads(json.dumps(payload))['verdicts']], printed)
        for verdict in payload['verdicts']:
            self.assertTrue({'state', 'status', 'blockers', 'reviewers', 'objectors', 'areas',
                             'approvals', 'ready_since', 'title', 'author'} <= set(verdict['verdict']))


class RunTests(unittest.TestCase):
    """`run()` with the GitHub side mocked, as a sync in a workflow sees it."""

    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.addCleanup(self.temp.cleanup)
        self.file = Path(self.temp.name) / 'verdicts.json'
        self.prs = [{'number': n, 'head': str(n) * 40, 'author': 'alice'} for n in (1, 2, 3)]
        self.rows = [{'number': n, 'head': str(n) * 40, 'author': 'alice', 'title': f'PR {n}', 'url': '',
                      'state': 'ready-for-human', 'status': 'pending', 'blockers': [], 'reviewers': [],
                      'objectors': [], 'areas': [], 'approvals': [], 'ready_since': None}
                     for n in (1, 2)]
        self.events = []
        self.stdout = io.StringIO()
        self.stderr = io.StringIO()

    def sync(self, publish=None, environment=None):
        """Run one sync; return its result or the exception it raised."""
        clock = iter(['2026-09-11T11:00:00Z', '2026-09-11T11:00:30Z', '2026-09-11T11:01:00Z'])

        def collect(*args, **kwargs):
            # The reads take time, and the real collect reads the clock too.
            main.utc_now()
            self.events.append('collect')
            return copy.deepcopy(self.prs), copy.deepcopy(self.prs[:2]), copy.deepcopy(self.prs[:2])

        def published(api, policy, pr, result, *args, **kwargs):
            self.events.append(f"publish #{pr['number']}")
            if publish:
                publish(pr)

        real_write = push.write

        def write(*args, **kwargs):
            self.events.append('push')
            return real_write(*args, **kwargs)

        environment = {'RUNNER_TEMP': self.temp.name} if environment is None else environment
        with patch.dict(os.environ, environment), \
                patch.object(main, 'utc_now', side_effect=lambda: next(clock)), \
                patch.object(main, 'collect', side_effect=collect), \
                patch.object(main, 'evaluate_snapshots', return_value=copy.deepcopy(self.rows)), \
                patch.object(main, 'publish', side_effect=published), \
                patch.object(main, 'clear_marks'), \
                patch.object(main, 'GitHub', return_value=Mock()), \
                patch.object(push, 'write', side_effect=write), \
                patch('sys.stdout', self.stdout), patch('sys.stderr', self.stderr):
            try:
                return main.run(['sync', '--repo', 'dashpay/platform'])
            except Exception as error:
                return error

    def test_the_verdicts_are_written_after_every_status(self):
        self.assertEqual(self.sync(), 0)
        self.assertEqual(self.events, ['collect', 'publish #1', 'publish #2', 'push'])
        written = json.loads(self.file.read_text())
        self.assertEqual(written['open'], [1, 2, 3])
        self.assertEqual([v['verdict'] for v in written['verdicts']], self.rows)

    def test_read_at_is_stamped_before_anything_is_read(self):
        # Stamped after the reads, a push could claim to be newer than a
        # snapshot whose reads came after its own.
        self.sync()
        self.assertEqual(json.loads(self.file.read_text())['read_at'], '2026-09-11T11:00:00Z')

    def test_a_failed_status_still_hands_over_the_verdicts_and_the_run_still_fails(self):
        def fail_second(pr):
            if pr['number'] == 2:
                raise main.GitHubError('boom')
        outcome = self.sync(publish=fail_second)
        self.assertIsInstance(outcome, main.GitHubError)
        self.assertIn('#2', str(outcome))
        self.assertIn('push', self.events)
        self.assertTrue(self.file.exists())

    def test_a_pull_request_whose_status_failed_is_left_to_the_last_snapshot(self):
        # Its head now says error. Handing over the verdict the run could not
        # publish would show the board a state the gate does not carry.
        def fail_second(pr):
            if pr['number'] == 2:
                raise main.GitHubError('boom')
        self.sync(publish=fail_second)
        written = json.loads(self.file.read_text())
        self.assertEqual([v['number'] for v in written['verdicts']], [1])
        self.assertIn(2, written['open'], 'still open: the board must not hide it')

    def test_nothing_that_goes_wrong_in_the_hand_over_changes_the_run(self):
        # The same run, with the hand-over broken in every way it can break:
        # the statuses, the report and the exit code must be what they are
        # with it working.
        self.assertEqual(self.sync(), 0)
        expected_events, expected_report = list(self.events), self.stdout.getvalue()
        not_a_folder = Path(self.temp.name) / 'a-file'
        not_a_folder.write_text('')
        breakages = {
            'payload raises': (patch.object(push, 'payload', side_effect=RuntimeError('broken')), None),
            'no such folder': (contextlib.nullcontext(), {'RUNNER_TEMP': '/nonexistent/folder'}),
            'not a folder': (contextlib.nullcontext(), {'RUNNER_TEMP': str(not_a_folder)}),
        }
        for name, (breakage, environment) in breakages.items():
            with self.subTest(name), breakage:
                self.events, self.stdout, self.stderr = [], io.StringIO(), io.StringIO()
                outcome = self.sync(environment=environment)
                self.assertEqual(outcome, 0)
                self.assertEqual(self.events, expected_events)
                self.assertEqual(self.stdout.getvalue(), expected_report,
                                 'stdout is the step summary; the hand-over writes nothing to it')
                self.assertIn('::warning::', self.stderr.getvalue())

    def test_an_unserialisable_verdict_only_warns(self):
        self.rows[1]['admitted_at'] = object()
        self.assertEqual(self.sync(), 0)
        self.assertEqual(self.events, ['collect', 'publish #1', 'publish #2', 'push'])
        self.assertIn('::warning::', self.stderr.getvalue())

    def test_outside_actions_nothing_is_written(self):
        self.assertEqual(self.sync(environment={'RUNNER_TEMP': ''}), 0)
        self.assertEqual(list(Path(self.temp.name).iterdir()), [])
        self.assertNotIn('::warning::', self.stderr.getvalue())


if __name__ == '__main__':
    unittest.main()
