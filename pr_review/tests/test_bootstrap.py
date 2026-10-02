"""Execute the workflow bootstrap with mocked GitHub reads, not a second resolver."""

import json
import os
from pathlib import Path
import subprocess
import tempfile
import unittest
from unittest.mock import patch

import yaml

WORKFLOW = Path(__file__).resolve().parents[2] / '.github/workflows/pr-review-reusable.yml'
CENTRAL = 'dashpay/stale_prs_are_bad'
PIN = '5e561c705a09eb80782557d57e905d444b03dffd'


class BootstrapTests(unittest.TestCase):
    def setUp(self):
        steps = yaml.safe_load(WORKFLOW.read_text())['jobs']['reconcile']['steps']
        self.step = next(s for s in steps if s.get('id') == 'target')
        self.script = '\n'.join(self.step['run'].splitlines()[1:-1])
        self.environment = {
            'TARGET_REPOSITORY': 'dashpay/platform',
            'WORKFLOW_REVISION': PIN,
            'WORKFLOW_REFERENCE': f'{CENTRAL}/.github/workflows/pr-review-reusable.yml@{PIN}',
            'CALLER_REFERENCE': 'dashpay/platform/.github/workflows/pr-review-policy.yml@refs/heads/v4.2-dev',
        }
        self.responses = {
            'repos/dashpay/platform': {'default_branch': 'v4.2-dev'},
            f'repos/{CENTRAL}/compare/master...{PIN}': {'status': 'behind'},
            f'repos/{CENTRAL}/rules/branches/master': [
                {'type': 'non_fast_forward'}, {'type': 'deletion'},
                {'type': 'pull_request', 'parameters': {
                    'required_approving_review_count': 1, 'require_code_owner_review': True}}],
        }
        self.calls = []

    def api(self, command, **kwargs):
        self.assertEqual(command[:2], ['gh', 'api'])
        self.assertEqual(len(command), 3, 'bootstrap makes GET requests only')
        self.calls.append(command[2])
        return subprocess.CompletedProcess(command, 0, json.dumps(self.responses[command[2]]), '')

    def run_bootstrap(self):
        with tempfile.TemporaryDirectory() as directory:
            output = Path(directory) / 'output'
            with patch.dict(os.environ, dict(self.environment, GITHUB_OUTPUT=str(output)), clear=True):
                with patch('subprocess.run', side_effect=self.api):
                    exec(compile(self.script, str(WORKFLOW), 'exec'), {})
            return dict(line.split('=', 1) for line in output.read_text().splitlines())

    def test_current_job_pin_wins_over_a_stale_stacked_pr_base(self):
        # The failing run used this wrapper, but re-read ef64a3c from the base.
        self.environment['WORKFLOW_BRANCH'] = 'chore/bump-rust-dashcore-secp-033'
        self.assertEqual(self.run_bootstrap(), {'branch': 'v4.2-dev', 'engine': PIN})
        self.assertFalse(any('/contents/' in path for path in self.calls))

    def test_workflow_supplies_the_documented_reusable_job_context(self):
        self.assertEqual(self.step['env']['WORKFLOW_REVISION'], '${{ job.workflow_sha }}')
        self.assertEqual(self.step['env']['WORKFLOW_REFERENCE'], '${{ job.workflow_ref }}')
        self.assertNotIn('WORKFLOW_BRANCH', self.step['env'])

    def test_missing_revision_never_silently_disables_verification(self):
        for revision in ('', 'master', 'a' * 7, 'a' * 40):
            with self.subTest(revision=revision):
                self.calls.clear()
                self.environment['WORKFLOW_REVISION'] = revision
                with self.assertRaisesRegex(SystemExit, 'full commit SHA or by master'):
                    self.run_bootstrap()
                self.assertEqual(self.calls, [])

    def test_a_different_workflow_or_mutable_reference_is_rejected(self):
        for reference in (f'{CENTRAL}/.github/workflows/other.yml@{PIN}',
                          f'{CENTRAL}/.github/workflows/pr-review-reusable.yml@master', ''):
            with self.subTest(reference=reference):
                self.environment['WORKFLOW_REFERENCE'] = reference
                with self.assertRaisesRegex(SystemExit, 'full commit SHA or by master'):
                    self.run_bootstrap()

    def test_a_caller_tracking_master_runs_the_commit_github_resolved(self):
        # One engine everywhere, at once: a fix merged here reaches every
        # repository on its next run instead of after a re-pin in each. GitHub
        # names the branch as a ref and still reports the commit it runs.
        self.environment['WORKFLOW_REFERENCE'] = f'{CENTRAL}/.github/workflows/pr-review-reusable.yml@refs/heads/master'
        self.responses[f'repos/{CENTRAL}/compare/master...{PIN}'] = {'status': 'identical'}
        self.assertEqual(self.run_bootstrap(), {'branch': 'v4.2-dev', 'engine': PIN})

    def test_master_is_the_only_branch_and_must_be_the_merged_master(self):
        for reference in ('refs/heads/feature', 'refs/heads/master-old', 'refs/tags/v1', PIN[:7]):
            with self.subTest(reference=reference):
                self.environment['WORKFLOW_REFERENCE'] = f'{CENTRAL}/.github/workflows/pr-review-reusable.yml@{reference}'
                with self.assertRaisesRegex(SystemExit, 'full commit SHA or by master'):
                    self.run_bootstrap()
        self.environment['WORKFLOW_REFERENCE'] = f'{CENTRAL}/.github/workflows/pr-review-reusable.yml@refs/heads/master'
        self.responses[f'repos/{CENTRAL}/compare/master...{PIN}'] = {'status': 'ahead'}
        with self.assertRaisesRegex(SystemExit, 'not a commit merged'):
            self.run_bootstrap()

    def test_an_unexpected_caller_is_rejected(self):
        self.environment['CALLER_REFERENCE'] = 'dashpay/other/.github/workflows/pr-review-policy.yml@refs/heads/main'
        with self.assertRaisesRegex(SystemExit, 'Unexpected caller'):
            self.run_bootstrap()

    def test_the_engine_still_must_be_merged_and_policies_protected(self):
        comparison = f'repos/{CENTRAL}/compare/master...{PIN}'
        self.responses[comparison] = {'status': 'ahead'}
        with self.assertRaisesRegex(SystemExit, 'not a commit merged'):
            self.run_bootstrap()
        self.responses[comparison] = {'status': 'identical'}
        self.responses[f'repos/{CENTRAL}/rules/branches/master'] = []
        with self.assertRaisesRegex(SystemExit, 'not protected'):
            self.run_bootstrap()


if __name__ == '__main__':
    unittest.main()
