"""The sheet's repository boundaries must remain explicit and permission-neutral."""

import json
import unittest

from pr_review.policy import codeowners, evaluate, validate_policy
from pr_review.registry import POLICIES, load_registry
from pr_review.tests.test_policy import fixture, NOW



def rules(text):
    """CODEOWNERS lines that GitHub would act on."""
    return [line for line in text.splitlines() if line.strip() and not line.startswith('#')]


class RepositoryConfigurationTests(unittest.TestCase):
    def setUp(self):
        self.registry = load_registry()
        self.policies = {entry['repository']: json.loads((POLICIES / entry['policy']).read_text())
                         for entry in self.registry['repositories']}

    def test_five_repository_rollout_starts_in_preview(self):
        self.assertEqual(self.registry['version'], 1)
        self.assertEqual(set(self.policies), {'dashpay/platform', 'dashpay/rust-dashcore',
                                            'dashpay/tenderdash', 'dashpay/grovedb', 'dashpay/dash-evo-tool'})
        self.assertEqual(len(self.registry['repositories']), 5)
        for entry in self.registry['repositories']:
            self.assertEqual(entry['mode'], 'preview')
            self.assertEqual(self.policies[entry['repository']]['repository'], entry['repository'])
            validate_policy(self.policies[entry['repository']])

    def test_external_whole_repository_roles_do_not_promote_contributors(self):
        expected = {'tenderdash': ('v1.8-dev', ['lklimek'], ['shumkov']),
                    'grovedb': ('develop', ['QuantumExplorer'], []),
                    'dash-evo-tool': ('v1.0-dev', ['lklimek'], [])}
        for name, (branch, owners, reviewers) in expected.items():
            with self.subTest(repository=name):
                policy = self.policies['dashpay/' + name]
                self.assertEqual(policy['target_branches'], [branch])
                self.assertEqual(len(policy['areas']), 1)
                area = policy['areas'][0]
                self.assertEqual((area['paths'], area['owners'], area['reviewers']), ([''], owners, reviewers))
                # Routing is the policy's, not CODEOWNERS'. A rule there would have
                # GitHub request these people the moment a pull request opens.
                self.assertEqual(rules(codeowners(policy)), [])

    def test_rust_dashcore_crates_are_owned_by_the_repository_wildcard(self):
        # These three crates had no owner in the responsibility sheet, and an
        # ownerless area is a configuration error for every pull request that
        # touches it — fifteen of thirty-three, once this check became the
        # gate. The repository's wildcard owners own them, as decided.
        policy = self.policies['dashpay/rust-dashcore']
        self.assertEqual(policy['target_branches'], ['dev'])
        self.assertEqual(policy['fallback'], {'owners': ['QuantumExplorer', 'xdustinface'], 'reviewers': []})
        areas = {area['id']: area for area in policy['areas']}
        self.assertEqual(set(areas), {'dash-spv', 'key-wallet', 'key-wallet-manager'})
        for name, area in areas.items():
            self.assertEqual(area['paths'], [name + '/'])
            self.assertEqual(area['owners'], policy['fallback']['owners'])
            self.assertEqual(area['reviewers'], ['ZocoLini'])
            self.assertNotIn('unresolved', area)
            _, pr = fixture()
            pr.update(base='dev', author='ZocoLini', files=[{'filename': name + '/src/lib.rs'}], comments=[])
            pr['permissions'].update(QuantumExplorer='admin', xdustinface='admin', ZocoLini='write')
            result = evaluate(policy, pr, NOW, NOW)
            self.assertNotEqual(result['state'], 'configuration-error', name)
    def test_fallbacks_name_each_repository_owner_not_platform_leads(self):
        expected = {'platform': (['QuantumExplorer', 'shumkov'], []),
                    'rust-dashcore': (['QuantumExplorer', 'xdustinface'], []),
                    'tenderdash': (['lklimek'], ['shumkov']),
                    'grovedb': (['QuantumExplorer'], []),
                    'dash-evo-tool': (['lklimek'], [])}
        for name, (owners, reviewers) in expected.items():
            with self.subTest(repository=name):
                policy = self.policies['dashpay/' + name]
                self.assertEqual(policy['fallback'], {'owners': owners, 'reviewers': reviewers})
                self.assertIn(f'policies/{name}.json', codeowners(policy), 'the stub points at the policy')

    def test_slack_roster_preserves_selected_people(self):
        handles = set()
        for policy in self.policies.values():
            self.assertEqual(policy['max_active_prs'], 5)
            for area in [policy['fallback']] + policy['areas']:
                handles.update(area['owners'] + area['reviewers'])
        self.assertFalse({'strophy', 'silvanassss'} & {handle.lower() for handle in handles})
        slack = json.loads((POLICIES / 'slack.json').read_text())
        self.assertEqual(slack['version'], 1)
        self.assertIsNone(slack['channel_id'])
        self.assertEqual(set(slack['users']), handles)
        self.assertTrue(all(value is None for value in slack['users'].values()))


if __name__ == '__main__':
    unittest.main()
