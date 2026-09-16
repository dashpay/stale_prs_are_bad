from pathlib import Path
import re
import unittest
from unittest.mock import patch

import yaml

from pr_review import event

ROOT = Path(__file__).resolve().parents[2]
WORKFLOW = ROOT / '.github/workflows/pr-review-reusable.yml'


class TargetCheckoutTests(unittest.TestCase):
    """The clone of the governed repository and the engine's view of it must agree.

    Validating a policy against a tree that was never checked out fails on its
    first area path, and the engine answers a failed policy validation by
    marking every open head an error. So the events that clone the tree and the
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
        self.assertEqual(gate.strip(), clone,
                         'a tree is handed to the engine on events that never cloned one')
        self.assertTrue(fallback.strip().endswith("|| ''"),
                        'the path must be empty on every other event, not merely wrong')


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
