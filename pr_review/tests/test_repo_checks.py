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

    def test_every_governed_branch_has_its_caller_pin_checked(self):
        # A pull request into a branch runs the caller of that branch. Reading
        # only the default branch let v5.1-dev and v6.0-dev keep running an
        # engine the compatibility gate no longer validated once the default
        # branch moved to master.
        import base64
        sha, old = 'a' * 40, 'b' * 40
        files = {'main': CALLER.format(reference=checks.REUSABLE, pin='master'),
                 'v5.1-dev': CALLER.format(reference=checks.REUSABLE, pin=old),
                 'v6.0-dev': CALLER.format(reference=checks.REUSABLE, pin=sha)}
        answers = {'repos/dashpay/example/branches': [{'name': n} for n in ('main', 'v5.1-dev', 'v6.0-dev',
                                                                             'v7.0-dev', 'feature')]}
        for branch, text in files.items():
            answers[f'repos/dashpay/example/contents/{checks.CALLER}?ref={branch}'] = {
                'content': base64.b64encode(text.encode()).decode()}

        def api(path):
            if path not in answers:
                raise checks.NotFound(path)
            return answers[path]

        with tempfile.TemporaryDirectory() as directory:
            policies, _, _ = layout(directory)
            policy = json.loads((policies / 'example.json').read_text())
            policy['target_branches'] = ['main', 'v*-dev']
            (policies / 'example.json').write_text(json.dumps(policy))
            pins = sorted(checks.branch_pins(policies, api))
        # v7.0-dev has no caller yet and `feature` is not governed: neither is read as a pin.
        self.assertEqual(pins, [('dashpay/example@main', 'master'), ('dashpay/example@v5.1-dev', old),
                                ('dashpay/example@v6.0-dev', sha)])

    def test_policy_compatibility_accepts_a_full_sha_or_master_and_nothing_else(self):
        # `master` is the protected branch the policies are read from; a caller
        # tracking it runs whatever the proposed change merges, so that is the
        # engine its policies are checked against.
        self.assertEqual(checks.caller_pin(CALLER.format(reference=checks.REUSABLE, pin='master')), 'master')
        for pin in ('main', 'refs/heads/master', 'v1', 'a' * 7, '', 'a' * 40 + ';command'):
            with self.subTest(pin=pin), self.assertRaisesRegex(ValueError, 'full commit SHA or master'):
                checks.caller_pin(CALLER.format(reference=checks.REUSABLE, pin=pin))


if __name__ == '__main__':
    unittest.main()
