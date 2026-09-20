import json
from pathlib import Path
import re
import tempfile
import unittest

from pr_review.rollout import caller_workflow, write_bundle
from pr_review.main import run
from pr_review import policy
from pr_review.event import BUILD_SCAN_CRON


class CommentTriggerTests(unittest.TestCase):
    """The workflow decides from the event body whether a comment is worth a run.

    It can only do that by testing for the markers the engine parses. Asserting
    the two agree could not tell live code from a stale comment mentioning the
    same string, so they share one definition and these tests check the round
    trip: a comment the engine reads must be one the workflow would wake for.
    """

    def setUp(self):
        self.workflow = caller_workflow('a' * 40)
        start = self.workflow.index('if: >-')
        rule = self.workflow[start:self.workflow.index('uses:', start)]
        self.tested = re.findall(r"contains\(github\.event\.comment\.body, '([^']*)'\)", rule)
        self.rule = rule

    def admits(self, body):
        # contains() on a string is a case-insensitive substring match.
        return any(literal.lower() in body.lower() for literal in self.tested)

    def test_a_body_the_engine_accepts_as_a_receipt_would_start_a_run(self):
        head = 'b' * 40
        body = ('Some walkthrough.\n<!-- ' + policy.RECEIPT_MARKER + ': '
                + json.dumps({'kind': 'reviewed', 'coveredCommitId': head}) + ' -->\n')
        self.assertTrue(policy._rabbit_receipt(body, head), 'fixture no longer reads as a receipt')
        self.assertTrue(self.admits(body), 'the engine reads this and the workflow would sleep through it')

    def test_a_body_the_engine_reads_as_rate_limited_would_start_a_run(self):
        body = 'blah\n' + policy.RATE_LIMITED + '\n'
        self.assertIsNotNone(policy.rate_limited_at(
            [{'user': 'coderabbitai', 'body': body, 'created_at': '2026-09-02T00:00:00Z',
              'updated_at': '2026-09-02T00:00:00Z'}], '2026-09-01T00:00:00Z'))
        self.assertTrue(self.admits(body))

    def test_an_attestation_would_start_a_run_whoever_wrote_it(self):
        # The engine honours one only from the author, but over-hearing costs a
        # run and under-hearing costs an author a wait they cannot diagnose.
        self.assertTrue(self.admits('/self-reviewed'))
        self.assertTrue(self.admits('/self-reviewed ' + 'c' * 40))

    def test_a_walkthrough_carrying_neither_marker_would_not(self):
        self.assertFalse(self.admits('**Walkthrough**\n\nThis change adds a test.'))

    def test_a_dispatch_can_ask_for_everything_and_the_input_reaches_the_engine(self):
        # Three hops: the caller's dispatch input, the reusable workflow's
        # input, the engine's environment. Dropping any one silently turns a
        # full pass back into a batch of six.
        self.assertIn("scope: ${{ inputs.scope || 'batch' }}", self.workflow)
        self.assertIn('options: [batch, all]', self.workflow)

    def test_it_wakes_for_a_skip(self):
        # A skip that waited for the hourly sweep would be a skip nobody
        # could see working.
        self.assertTrue(self.admits('/skip-bots'))
        self.assertFalse(self.admits('lgtm'))

    def test_it_looks_again_at_pull_requests_waiting_on_a_build(self):
        # A build turning green raises no event this controller hears, so
        # without this schedule such a pull request waits for the hourly sweep.
        crons = re.findall(r"- cron: '([^']+)'", self.workflow)
        self.assertIn(BUILD_SCAN_CRON, crons)
        self.assertEqual(len(crons), 2, 'the hourly sweep is still the backstop')
        minutes = [set(cron.split()[0].split(',')) for cron in crons]
        self.assertFalse(minutes[0] & minutes[1], 'the two schedules must never fire together')

    def test_it_no_longer_wakes_for_a_bot_whose_receipt_is_a_review(self):
        # thepastaclaw reports by review, which arrives on its own event, and
        # nothing reads its comments.
        self.assertNotIn('thepastaclaw', self.rule)
        self.assertIn('pull_request_review', self.workflow, 'its receipts must still arrive')


def rules(text):
    """CODEOWNERS lines that GitHub would act on."""
    return [line for line in text.splitlines() if line.strip() and not line.startswith('#')]


class RolloutTests(unittest.TestCase):
    def test_bundle_uses_pinned_shared_engine_and_target_policy(self):
        with tempfile.TemporaryDirectory() as folder:
            root = Path(folder) / 'packet'
            write_bundle('dashpay/tenderdash','a'*40,root)
            workflow = (root / '.github/workflows/pr-review-policy.yml').read_text()
            # The filename is historical; the display name follows the suite.
            self.assertTrue(workflow.startswith('name: PR Hygiene policy\n'), workflow[:40])
            self.assertIn('dashpay/stale_prs_are_bad/.github/workflows/pr-review-reusable.yml@'+'a'*40,workflow)
            self.assertNotIn('engine_revision', workflow)
            self.assertNotIn('pr-review-policy.json', workflow)
            self.assertFalse((root / '.github/pr-review-policy.json').exists())
            self.assertEqual(rules((root / '.github/CODEOWNERS').read_text()), [],
                             'a rule here would have GitHub request reviewers on its own')
            self.assertIn('pull_request_review:', workflow)
            self.assertNotIn('workflow_run:', workflow)
            self.assertFalse((root / '.github/workflows/pr-review-signal.yml').exists())
            self.assertFalse((root / 'pr_review').exists())

    def test_generated_bundle_passes_effective_codeowners_check(self):
        with tempfile.TemporaryDirectory() as folder:
            root = Path(folder) / 'packet'
            write_bundle('dashpay/tenderdash', 'a' * 40, root)
            (root / 'CODEOWNERS').write_text('obsolete overridden root file')
            self.assertEqual(run(['codeowners', '--check', '--repo', 'dashpay/tenderdash',
                                   '--repository-root', str(root)]), 0)

    def test_bundle_never_overwrites_existing_work(self):
        with tempfile.TemporaryDirectory() as folder:
            path = Path(folder)
            (path / 'existing').write_text('preserve')
            with self.assertRaises(ValueError):
                write_bundle('dashpay/tenderdash','a'*40,path)
            self.assertEqual((path / 'existing').read_text(),'preserve')

    def test_bundle_rejects_unpinned_engine(self):
        with tempfile.TemporaryDirectory() as folder:
            with self.assertRaises(ValueError):
                write_bundle('dashpay/tenderdash','main',Path(folder)/'packet')


if __name__ == '__main__':
    unittest.main()
