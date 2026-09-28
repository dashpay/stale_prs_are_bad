"""A broken policy must not let one PR event overwrite the repository's queue."""

import copy
import os
import unittest
from unittest.mock import Mock, patch

from pr_review import main
from pr_review.tests.test_main import broken_policies_root
from pr_review.tests.test_policy import fixture


class ConfigurationIsolationTests(unittest.TestCase):
    def setUp(self):
        self.policy, self.pr = fixture()
        self.policy['future_unknown_field'] = True
        self.api = Mock()
        self.api.pull.return_value = self.pr
        self.other = dict(self.pr, number=2, head='b' * 40, author='another-author')
        self.api.open_prs.return_value = [self.pr, self.other]
        self.environment = {'GITHUB_ACTIONS': 'true', 'GITHUB_REPOSITORY': 'dashpay/platform',
                            'PR_REVIEW_AUTOMATION_ENABLED': 'true', 'GITHUB_RUN_ID': '36384504598'}

    def run_invalid(self, selection=('--pr', '1'), apply=True):
        with patch.dict(os.environ, self.environment), patch.object(main, 'GitHub', return_value=self.api):
            with broken_policies_root(self.policy) as root, self.assertRaises(ValueError):
                main.run(['sync', '--policies-root', root, *selection] + (['--apply'] if apply else []))

    def test_unknown_schema_field_invalidates_only_the_explicit_current_head(self):
        self.run_invalid()
        self.api.post_status.assert_called_once_with(
            self.pr['head'], 'error', 'Invalid policy configuration; inspect workflow log',
            target_url='https://github.com/dashpay/platform/actions/runs/36384504598')
        self.assertEqual([c.args[0] for c in self.api.pull.call_args_list], [1, 1])

    def test_draft_stacked_pr_cannot_invalidate_unrelated_heads(self):
        # platform#5113: draft on a feature base, with other PRs already green.
        self.pr.update(draft=True, base='chore/bump-rust-dashcore-secp-033')
        self.run_invalid()
        self.api.open_prs.assert_not_called()
        self.api.post_status.assert_not_called()

    def test_draft_closed_and_out_of_scope_targets_do_not_write(self):
        for change in ({'draft': True}, {'state': 'closed'}, {'base': 'feature/stack'}):
            with self.subTest(change=change):
                self.api.reset_mock()
                self.api.pull.return_value = dict(self.pr, **change)
                self.run_invalid()
                self.api.post_status.assert_not_called()

    def test_batch_full_and_build_scans_do_not_guess_a_write_scope(self):
        for selection in ((), ('--batch-size', '6'), ('--waiting-on-build',)):
            with self.subTest(selection=selection):
                self.api.reset_mock()
                self.run_invalid(selection)
                self.assertEqual(self.api.mock_calls, [])

    def test_a_head_shared_with_another_pr_is_not_an_isolated_status_target(self):
        self.other['head'] = self.pr['head']
        self.run_invalid()
        self.api.post_status.assert_not_called()

    def test_a_changed_head_or_scope_aborts_the_error_write(self):
        for change in ({'head': 'c' * 40}, {'base': 'feature/stack'}, {'state': 'closed'}, {'draft': True}):
            with self.subTest(change=change):
                self.api.reset_mock()
                self.api.pull.side_effect = [self.pr, dict(self.pr, **change)]
                self.run_invalid()
                self.api.post_status.assert_not_called()

    def test_unreadable_repository_or_branch_scope_is_reported_only_in_the_job(self):
        original = copy.deepcopy(self.policy)
        for change in ({'repository': 'dashpay/other'}, {'target_branches': None},
                       {'target_branches': []}, {'target_branches': [1]}):
            with self.subTest(change=change):
                self.api.reset_mock()
                self.policy = dict(original, **change)
                self.run_invalid()
                self.assertEqual(self.api.mock_calls, [])

    def test_preview_cannot_revoke_any_status(self):
        self.run_invalid(apply=False)
        self.assertEqual(self.api.mock_calls, [])

    def test_failure_to_read_identity_does_not_expand_scope_or_hide_schema_error(self):
        self.api.pull.side_effect = main.GitHubError('unavailable')
        self.run_invalid()
        self.api.open_prs.assert_not_called()
        self.api.post_status.assert_not_called()


if __name__ == '__main__':
    unittest.main()
