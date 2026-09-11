"""The sheet's repository boundaries must remain explicit and permission-neutral."""

import json
import unittest

from pr_review.policy import codeowners, evaluate, validate_policy
from pr_review.registry import POLICIES, load_registry
from pr_review.tests.test_policy import fixture, NOW



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
            self.assertIsNone(entry['engine_revision'])
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
                # GitHub keeps only the last matching rule, so a second `*` line would silently win.
                wildcard = [line for line in codeowners(policy).splitlines() if line.startswith('* ')]
                self.assertEqual(wildcard, ['* ' + ' '.join('@' + user for user in owners + reviewers)])

    def test_rust_dashcore_only_maps_visible_crates_and_keeps_missing_owners_blocked(self):
        policy = self.policies['dashpay/rust-dashcore']
        self.assertEqual(policy['target_branches'], ['dev'])
        areas = {area['id']: area for area in policy['areas']}
        self.assertEqual(set(areas), {'dash-spv', 'key-wallet', 'key-wallet-manager'})
        for name, area in areas.items():
            self.assertEqual(area['paths'], [name + '/'])
            self.assertEqual(area['owners'], [])
            self.assertTrue(area['unresolved'])
            self.assertEqual(area['reviewers'], ['ZocoLini', 'xdustinface'] if name == 'dash-spv' else ['QuantumExplorer', 'ZocoLini', 'xdustinface'])
            _, pr = fixture()
            pr.update(base='dev', author='QuantumExplorer', files=[{'filename':name + '/src/lib.rs'}])
            pr['permissions'].update(QuantumExplorer='admin', shumkov='admin', ZocoLini='write')
            result = evaluate(policy, pr, NOW, NOW)
            self.assertEqual(result['state'], 'configuration-error')
            self.assertIn('Unresolved identities in ' + name, result['blockers'])

    def test_fallbacks_name_each_repository_owner_not_platform_leads(self):
        expected = {'platform': (['QuantumExplorer', 'shumkov'], []),
                    'rust-dashcore': (['QuantumExplorer'], []),
                    'tenderdash': (['lklimek'], ['shumkov']),
                    'grovedb': (['QuantumExplorer'], []),
                    'dash-evo-tool': (['lklimek'], [])}
        for name, (owners, reviewers) in expected.items():
            with self.subTest(repository=name):
                policy = self.policies['dashpay/' + name]
                self.assertEqual(policy['fallback'], {'owners': owners, 'reviewers': reviewers})
                if not any('' in area['paths'] and (area['owners'] or area['reviewers']) for area in policy['areas']):
                    self.assertEqual(codeowners(policy).splitlines()[2], '* ' + ' '.join('@' + user for user in owners + reviewers))

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
