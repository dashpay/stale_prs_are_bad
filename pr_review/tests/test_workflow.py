from pathlib import Path
import unittest

WORKFLOW = Path(__file__).resolve().parents[2] / '.github/workflows/pr-review-reusable.yml'
SWEEP = "github.event_name == 'schedule' || github.event_name == 'workflow_dispatch'"


class TargetCheckoutTests(unittest.TestCase):
    """The clone of the governed repository and the engine's view of it must agree.

    Validating a policy against a tree that was never checked out fails every
    area path, and the engine answers a failed policy validation by marking
    every open head a configuration error. So the clone and the root handed to
    the engine are gated on the same events, and nothing about that is obvious
    from either line on its own.
    """

    def setUp(self):
        self.text = WORKFLOW.read_text()

    def step(self, name):
        start = self.text.index('- name: ' + name)
        return self.text[start:self.text.index('- name: ', start + 1)]

    def test_the_clone_of_the_governed_repository_is_only_paid_on_a_sweep(self):
        self.assertIn(SWEEP, self.step('Checkout target default branch'))

    def test_the_check_that_needs_that_clone_is_gated_with_it(self):
        self.assertIn(SWEEP, self.step('Check policy paths and CODEOWNERS against the target tree'))

    def test_the_engine_is_given_no_tree_when_none_was_cloned(self):
        reconcile = self.text[self.text.index('- name: Reconcile with repository-local credentials'):]
        root = next(line for line in reconcile.splitlines() if 'PR_REVIEW_REPOSITORY_ROOT' in line)
        self.assertIn(SWEEP, root, 'the engine would validate against a tree that was never cloned')
        self.assertIn("|| ''", root, 'it must be empty, not a path that does not exist')

    def test_the_engine_skips_the_path_check_when_it_is_empty(self):
        # The workflow can only hand over an empty string; this is the other
        # half of that contract.
        event = (WORKFLOW.parents[2] / 'pr_review/event.py').read_text()
        self.assertIn("if os.environ.get('PR_REVIEW_REPOSITORY_ROOT'):", event)


if __name__ == '__main__':
    unittest.main()
