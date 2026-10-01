"""The sheet's repository boundaries must remain explicit and permission-neutral."""

import json
import unittest

from pr_review.policy import codeowners, evaluate, governs, validate_policy
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

    def test_platform_governs_every_development_branch_by_pattern(self):
        # The pattern its own branch protection uses, so a release that renames
        # the branches cannot take pull requests out of the policy.
        policy = self.policies['dashpay/platform']
        self.assertIn('v*-dev', policy['target_branches'])
        # A caller pinned to an engine that compares names exactly reads this
        # same live policy and would govern nothing from a pattern alone —
        # and strip its marks from every pull request. The current names stay
        # listed beside it, and must be ones the pattern covers anyway.
        names = [b for b in policy['target_branches'] if '*' not in b]
        self.assertTrue(names)
        self.assertTrue(all(governs(policy, name) for name in names), names)

    def test_external_whole_repository_roles_do_not_promote_contributors(self):
        expected = {'tenderdash': ('v1.8-dev', ['lklimek'], ['shumkov']),
                    'grovedb': ('develop', ['QuantumExplorer'], []),
                    'dash-evo-tool': ('v1.0-dev', ['lklimek'], [])}
        for name, (branch, owners, reviewers) in expected.items():
            with self.subTest(repository=name):
                policy = self.policies['dashpay/' + name]
                self.assertEqual(policy['target_branches'], [branch])
                # The whole repository is the fallback's: a catch-all area would
                # stop any directory being carved out of it, since areas cannot nest.
                self.assertEqual([area['id'] for area in policy['areas']], ['github'])
                self.assertEqual(policy['fallback'], {'owners': owners, 'reviewers': reviewers})
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
        # The fallback also names a reviewer, so more than one person can
        # unblock a pull request outside these crates. The crates themselves
        # are the policy as agreed, and stay.
        self.assertEqual(policy['fallback'], {'owners': ['QuantumExplorer', 'xdustinface'], 'reviewers': ['ZocoLini']})
        areas = {area['id']: area for area in policy['areas']}
        self.assertEqual(set(areas), {'dash-spv', 'key-wallet', 'key-wallet-manager', 'github'})
        for name, area in areas.items():
            if name == 'github':
                continue
            self.assertEqual(area['paths'], [name + '/'])
            self.assertEqual(area['owners'], policy['fallback']['owners'])
            self.assertEqual(area['reviewers'], ['ZocoLini'])
            self.assertNotIn('unresolved', area)
            _, pr = fixture()
            pr.update(base='dev', author='ZocoLini', files=[{'filename': name + '/src/lib.rs'}], comments=[])
            pr['permissions'].update(QuantumExplorer='admin', xdustinface='admin', ZocoLini='write')
            result = evaluate(policy, pr, NOW, NOW)
            self.assertNotEqual(result['state'], 'configuration-error', name)
    def test_github_directory_is_ktechmidas_everywhere_with_shumkov_as_backup(self):
        # CI, release workflows and the PR Hygiene caller itself live under
        # .github/, in every repository, so one person owns them all. A second
        # person can approve, so a change to CI never waits on one account.
        for name, policy in self.policies.items():
            with self.subTest(repository=name):
                areas = [area for area in policy['areas'] if area['id'] == 'github']
                self.assertEqual(len(areas), 1)
                self.assertEqual((areas[0]['paths'], areas[0]['owners'], areas[0]['reviewers']),
                                 (['.github/'], ['ktechmidas'], ['shumkov']))
                _, pr = fixture()
                pr.update(base=policy['target_branches'][0], author='someone',
                          files=[{'filename': '.github/workflows/ci.yml'}, {'filename': 'README.md'}], comments=[])
                pr['permissions'].update({handle: 'admin' for handle in
                                          ['ktechmidas', 'shumkov'] + policy['fallback']['owners'] + policy['fallback']['reviewers']})
                result = evaluate(policy, pr, NOW, NOW)
                self.assertNotEqual(result['state'], 'configuration-error', result)
                # The workflow is ktechmidas's to approve; the rest of the
                # repository is still whoever owned it before.
                self.assertEqual(result['areas'], ['fallback', 'github'])

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
