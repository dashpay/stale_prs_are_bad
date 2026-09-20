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

    def test_rust_dashcore_is_one_area_with_a_reviewer_who_can_unblock_it(self):
        # Over the last forty merged pull requests one person approved fifteen
        # and nobody else more than five, while the policy let that one person
        # -- and an owner who approves nothing there -- unblock everything.
        # ZocoLini approves across the repository, so the fallback names him;
        # the three crate areas said nothing the fallback does not say now.
        policy = self.policies['dashpay/rust-dashcore']
        self.assertEqual(policy['target_branches'], ['dev'])
        self.assertEqual(policy['fallback'], {'owners': ['QuantumExplorer', 'xdustinface'], 'reviewers': ['ZocoLini']})
        self.assertEqual(policy['areas'], [])
        self.assertNotIn('shumkov', policy['fallback']['owners'], 'the owner said not to; admins merge their own')
        _, pr = fixture()
        pr.update(base='dev', author='romchornyi', files=[{'filename': 'rpc-client/src/lib.rs'}], comments=[])
        pr['permissions'].update(QuantumExplorer='admin', xdustinface='admin', ZocoLini='write', romchornyi='write')
        result = evaluate(policy, pr, NOW, NOW)
        self.assertNotEqual(result['state'], 'configuration-error')
        pr['comments'] = [dict(id=3, user='romchornyi', body=f"/self-reviewed {pr['head']}", created_at=NOW, updated_at=NOW)]
        result = evaluate(policy, pr, NOW, NOW)
        self.assertEqual(result['state'], 'ready-for-human')
        self.assertEqual(result['reviewers'], ['QuantumExplorer', 'ZocoLini', 'xdustinface'], 'three people can unblock, not one')
    def test_fallbacks_name_each_repository_owner_not_platform_leads(self):
        expected = {'platform': (['QuantumExplorer', 'shumkov'], []),
                    'rust-dashcore': (['QuantumExplorer', 'xdustinface'], ['ZocoLini']),
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
