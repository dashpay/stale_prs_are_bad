from http.server import BaseHTTPRequestHandler, HTTPServer
import json
import os
from pathlib import Path
import re
import shutil
import subprocess
import tempfile
import threading
import unittest
from unittest.mock import patch

import yaml

from pr_review import event

ROOT = Path(__file__).resolve().parents[2]
WORKFLOW = ROOT / '.github/workflows/pr-review-reusable.yml'


class TargetCheckoutTests(unittest.TestCase):
    """The clone of the governed repository and the engine's view of it must agree.

    Validating a policy against a tree that was never checked out fails on its
    first area path. So the events that clone the tree and the
    events that hand its path to the engine have to be the same set — not
    merely overlapping, which is what a widened gate on one side would leave.
    """

    @classmethod
    def setUpClass(cls):
        steps = yaml.safe_load(WORKFLOW.read_text())['jobs']['reconcile']['steps']
        cls.steps = {step['name']: step for step in steps if 'name' in step}

    def condition(self, text):
        """The event test inside an expression, with its ${{ }} and spacing gone."""
        inner = re.sub(r'^\$\{\{|\}\}$', '', text.strip()).strip()
        return re.sub(r'\s+', ' ', inner)

    def test_the_dispatch_scope_reaches_the_engine(self):
        env = self.steps['Reconcile with repository-local credentials']['env']
        self.assertEqual(env.get('PR_REVIEW_SCOPE'), '${{ inputs.scope }}')
        doc = yaml.safe_load(WORKFLOW.read_text())
        self.assertIn('scope', (doc[True]['workflow_call'].get('inputs') or {}), 'yaml reads `on` as True')

    def test_the_scan_between_sweeps_does_not_pay_for_the_clone(self):
        # The scan is a schedule too, so gating on the event name alone would
        # re-add the slowest step in the run three times an hour per repository.
        gate = self.condition(self.steps['Checkout target default branch']['if'])
        self.assertIn(event.SWEEP_CRON, gate, 'the sweep is named, not every schedule')
        self.assertNotIn(event.BUILD_SCAN_CRON, gate)

    def test_the_clone_is_only_paid_when_the_policy_is_revalidated(self):
        gate = self.condition(self.steps['Checkout target default branch']['if'])
        self.assertIn('github.event_name', gate)
        self.assertNotIn('pull_request', gate, 'an event run must not pay for the clone')

    def test_every_step_that_needs_the_clone_is_gated_with_it(self):
        clone = self.condition(self.steps['Checkout target default branch']['if'])
        check = self.condition(self.steps['Check policy paths and CODEOWNERS against the target tree']['if'])
        self.assertEqual(clone, check)

    def test_the_engine_is_handed_that_tree_on_exactly_those_events(self):
        clone = self.condition(self.steps['Checkout target default branch']['if'])
        root = self.steps['Reconcile with repository-local credentials']['env']['PR_REVIEW_REPOSITORY_ROOT']
        gate, _, fallback = self.condition(root).partition('&&')
        # `&&` binds tighter than `||`, so without the parentheses a dispatch
        # hands the engine the string `true` instead of a path.
        self.assertTrue(gate.strip().startswith('(') and gate.strip().endswith(')'),
                        'the event test must be grouped or it does not gate the path at all')
        self.assertEqual(gate.strip().strip('()'), clone,
                         'a tree is handed to the engine on events that never cloned one')
        self.assertTrue(fallback.strip().endswith("|| ''"),
                        'the path must be empty on every other event, not merely wrong')


class PermissionTests(unittest.TestCase):
    """The reusable workflow takes exactly what its caller grants.

    A called workflow that asks for a scope its caller did not grant fails to
    start, and pull_request_target runs the caller from each pull request's
    base branch: an old release branch's caller would then stop the gate on
    every pull request into it. Declaring nothing inherits the caller's grant,
    which is never more than a caller chose to give.
    """

    def test_neither_the_workflow_nor_its_job_asks_for_anything(self):
        doc = yaml.safe_load(WORKFLOW.read_text())
        self.assertNotIn('permissions', doc)
        for name, job in doc['jobs'].items():
            self.assertNotIn('permissions', job, name)


class VerdictPostTests(unittest.TestCase):
    """The last step hands this run's verdicts to the dashboard.

    Every status is already written when it starts, so it can only ever warn:
    it must never fail the job, hold it long, or print the token it mints.
    """

    @classmethod
    def setUpClass(cls):
        cls.steps = yaml.safe_load(WORKFLOW.read_text())['jobs']['reconcile']['steps']
        cls.step = cls.steps[-1]

    def test_it_runs_last_and_whatever_happened_before_it(self):
        self.assertEqual(self.step['name'], 'Post verdicts to the dashboard')
        self.assertEqual(self.step['if'], 'always()')

    def test_it_can_neither_fail_the_job_nor_hold_it(self):
        self.assertIs(self.step['continue-on-error'], True)
        self.assertLessEqual(self.step['timeout-minutes'], 2)
        calls = re.findall(r'\bcurl\b[^\n]*(?:\\\n[^\n]*)*', self.step['run'])
        self.assertEqual(len(calls), 2, 'one call mints the token, one posts')
        for call in calls:
            self.assertIn('--max-time 20', call)

    def test_it_posts_to_the_one_service_over_tls(self):
        self.assertEqual(self.step['env']['SERVICE_URL'], 'https://pr-hygiene.services.dash.org')
        self.assertIn('/ingest/verdicts', self.step['run'])
        self.assertIn("--proto '=https'", self.step['run'])

    def post(self, environment, folder):
        """Run the step's script as the runner would, with its default shell."""
        bash = shutil.which('bash')
        env = {'PATH': os.environ['PATH'], 'RUNNER_TEMP': folder,
               'SERVICE_URL': self.step['env']['SERVICE_URL'], **environment}
        done = subprocess.run([bash, '-e', '-c', self.step['run']], env=env, capture_output=True,
                              text=True, timeout=60)
        return done.returncode, done.stdout + done.stderr

    def test_without_verdicts_it_does_nothing(self):
        with tempfile.TemporaryDirectory() as folder:
            self.assertEqual(self.post({}, folder), (0, ''))

    def test_a_caller_that_grants_no_token_gets_a_warning(self):
        with tempfile.TemporaryDirectory() as folder:
            Path(folder, 'verdicts.json').write_text('{}')
            code, output = self.post({}, folder)
        self.assertEqual(code, 0)
        self.assertIn('::warning::', output)

    def test_a_failed_post_warns_and_never_prints_the_token(self):
        secret = 'eyJ-the-minted-token'
        request_token = 'the-request-token'
        seen = []

        class Mint(BaseHTTPRequestHandler):
            def do_GET(self):
                seen.append((self.path, self.headers.get('Authorization')))
                body = json.dumps({'value': secret}).encode()
                self.send_response(200)
                self.send_header('Content-Length', str(len(body)))
                self.end_headers()
                self.wfile.write(body)

            def log_message(self, *args):
                pass

        server = HTTPServer(('127.0.0.1', 0), Mint)
        threading.Thread(target=server.serve_forever, daemon=True).start()
        self.addCleanup(server.server_close)
        self.addCleanup(server.shutdown)
        with tempfile.TemporaryDirectory() as folder:
            Path(folder, 'verdicts.json').write_text('{}')
            for service in ('https://127.0.0.1:1', 'http://127.0.0.1:1'):
                with self.subTest(service):
                    code, output = self.post({
                        'ACTIONS_ID_TOKEN_REQUEST_URL': f'http://127.0.0.1:{server.server_port}/token?x=1',
                        'ACTIONS_ID_TOKEN_REQUEST_TOKEN': request_token,
                        'SERVICE_URL': service}, folder)
                    self.assertEqual(code, 0, output)
                    self.assertIn('::warning::', output)
                    self.assertIn(f'::add-mask::{secret}', output, 'masked before anything else can print it')
                    self.assertNotIn(secret, output.replace(f'::add-mask::{secret}', ''))
                    self.assertNotIn(request_token, output)
            self.assertEqual(os.listdir(folder), ['verdicts.json'], 'no header file is left behind')
        self.assertTrue(seen)
        path, authorization = seen[0]
        self.assertEqual(authorization, f'bearer {request_token}')
        self.assertIn('audience=', path)


class RepositoryRootTests(unittest.TestCase):
    def test_no_tree_means_the_engine_is_not_asked_to_check_one(self):
        # The other half of the workflow's contract: an empty value has to mean
        # "no tree", not a path that does not exist.
        for value, expected in [('', False), ('/somewhere/review-target', True)]:
            with patch.dict('os.environ', {'GITHUB_EVENT_NAME': 'schedule',
                                           'GITHUB_REPOSITORY': 'dashpay/platform',
                                           'GITHUB_EVENT_PATH': str(ROOT / 'pr_review/tests/__init__.py'),
                                           'PR_REVIEW_REPOSITORY_ROOT': value}, clear=False):
                with patch.object(event, 'run', return_value=0) as run:
                    with patch.object(Path, 'read_text', return_value='{}'):
                        event.main()
            options = run.call_args.args[0]
            self.assertEqual('--repository-root' in options, expected, repr(value))


if __name__ == '__main__':
    unittest.main()
