"""The CI checks that compare a policy against the repository it governs."""

import importlib.util
import json
from pathlib import Path
import tempfile
import unittest

SCRIPT = Path(__file__).resolve().parents[2] / '.github/scripts/repo_checks.py'
spec = importlib.util.spec_from_file_location('repo_checks', SCRIPT)
checks = importlib.util.module_from_spec(spec)
spec.loader.exec_module(checks)

CALLER = '''name: PR review policy
on:
  schedule:
    - cron: '*/15 * * * *'
jobs:
  policy:
    uses: {reference}@{pin}
'''


def layout(directory, required_bots=None, coderabbit=None):
    """A policies root and a matching tree for one governed repository."""
    root = Path(directory)
    policies, tree = root / 'policies', root / 'trees/example'
    (tree / '.github/workflows').mkdir(parents=True)
    policies.mkdir()
    (policies / 'repositories.json').write_text(json.dumps({'version': 1, 'repositories': [
        {'repository': 'dashpay/example', 'policy': 'example.json', 'mode': 'preview'}]}))
    policy = {'version': 1, 'repository': 'dashpay/example', 'max_active_prs': 5,
              'target_branches': ['main'], 'fallback': {'owners': ['owner'], 'reviewers': []}, 'areas': []}
    if required_bots is not None:
        policy['required_bots'] = required_bots
    (policies / 'example.json').write_text(json.dumps(policy))
    if coderabbit is not None:
        (tree / '.coderabbit.yaml').write_text(coderabbit)
    return policies, root / 'trees', tree


class RepoCheckTests(unittest.TestCase):
    def test_requiring_coderabbit_where_it_is_switched_off_is_an_error(self):
        with tempfile.TemporaryDirectory() as directory:
            policies, trees, _ = layout(directory, coderabbit='reviews:\n  auto_review:\n    enabled: false\n')
            errors, warnings = checks.bot_problems(policies, trees)
            self.assertEqual(warnings, [])
            self.assertIn('would never resolve', errors[0])

    def test_dropping_coderabbit_where_it_is_on_is_only_a_warning(self):
        for coderabbit in [None, 'reviews:\n  auto_review:\n    enabled: true\n', 'reviews: {}\n']:
            with self.subTest(coderabbit=coderabbit), tempfile.TemporaryDirectory() as directory:
                policies, trees, _ = layout(directory, required_bots=['thepastaclaw'], coderabbit=coderabbit)
                errors, warnings = checks.bot_problems(policies, trees)
                self.assertEqual(errors, [])
                self.assertIn('confirm this is deliberate', warnings[0])

    def test_matching_declarations_report_nothing(self):
        cases = [(['thepastaclaw'], 'reviews:\n  auto_review:\n    enabled: false\n'), (None, None)]
        for required_bots, coderabbit in cases:
            with self.subTest(required_bots=required_bots), tempfile.TemporaryDirectory() as directory:
                policies, trees, _ = layout(directory, required_bots=required_bots, coderabbit=coderabbit)
                self.assertEqual(checks.bot_problems(policies, trees), ([], []))

    def test_pin_is_read_the_way_the_reusable_workflow_reads_it(self):
        pin = 'a' * 40
        self.assertEqual(checks.caller_pin(CALLER.format(reference=checks.REUSABLE, pin=pin)), pin)
        mixed = 'dashpay/Stale_PRs_Are_Bad/.github/workflows/pr-review-reusable.yml'
        self.assertEqual(checks.caller_pin(CALLER.format(reference=mixed, pin=pin)), pin)
        for text in [CALLER.format(reference='dashpay/other/.github/workflows/x.yml', pin=pin),
                     CALLER.format(reference=checks.REUSABLE, pin=pin) + f'  second:\n    uses: {checks.REUSABLE}@{"b" * 40}\n']:
            with self.subTest(text=text[-60:]), self.assertRaises(ValueError):
                checks.caller_pin(text)

    def test_a_repository_without_a_caller_workflow_is_skipped(self):
        with tempfile.TemporaryDirectory() as directory:
            policies, trees, tree = layout(directory)
            self.assertEqual(list(checks.caller_pins(policies, trees)), [])
            (tree / '.github/workflows/pr-review-policy.yml').write_text(
                CALLER.format(reference=checks.REUSABLE, pin='c' * 40))
            self.assertEqual(list(checks.caller_pins(policies, trees)), [('dashpay/example', 'c' * 40)])


if __name__ == '__main__':
    unittest.main()
