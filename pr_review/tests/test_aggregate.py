import json
from pathlib import Path
import tempfile
import unittest
from unittest.mock import Mock, patch
from pr_review import aggregate as a


class AggregateTests(unittest.TestCase):
    def policy(self, repo):
        return {'version': 1, 'repository': repo, 'max_active_prs': 5, 'target_branches': ['dev'],
                'fallback': {'owners': ['Alice'], 'reviewers': []}, 'areas': []}

    def test_repo_numbers_distinct_counts_and_partial_failures(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            entries = []
            for name in ['a', 'b', 'c']:
                repo = 'dashpay/' + name
                (root / (name + '.json')).write_text(json.dumps(self.policy(repo)))
                entries.append({'repository': repo, 'policy': name + '.json', 'mode': 'preview', 'engine_revision': None})
            context = [{'number': x, 'author': 'Alice', 'draft': False, 'state': 'open', 'base': 'dev'} for x in range(3)]
            with patch.object(a.main, 'collect', side_effect=[(context, [], []), (context, [], []), a.GitHubError('unavailable')]):
                with patch.object(a.main, 'evaluate_snapshots', create=True,
                                  side_effect=lambda policy, *args: [{'repository': policy['repository'], 'number': 1, 'author': 'Alice', 'state': 'waiting-bots'}]):
                    data = a.collect_snapshot({'version': 1, 'repositories': entries}, root, lambda repo: Mock())
        self.assertFalse(data['complete'])
        self.assertEqual(len(data['pull_requests']), 2)
        self.assertEqual(data['workload'][0]['total'], 6)
        self.assertTrue(data['workload'][0]['over_limit'])
        self.assertEqual(data['roster'], ['Alice'])
        self.assertFalse(data['repositories'][2]['complete'])

    def test_policy_source_is_the_registered_file_regardless_of_mode(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            (root / 'a.json').write_text(json.dumps(self.policy('dashpay/a')))
            for mode in ['preview', 'active']:
                policy = a.load_policy({'repository': 'dashpay/a', 'mode': mode, 'policy': 'a.json', 'engine_revision': None}, root)
                self.assertEqual(policy['repository'], 'dashpay/a')

    def test_registry_rejects_duplicate_repos_and_path_escape(self):
        entry = {'repository': 'dashpay/a', 'policy': '../outside.json', 'mode': 'preview', 'engine_revision': None}
        with self.assertRaises(ValueError):
            a.validate_registry({'version': 1, 'repositories': [entry]})
        entry['policy'] = 'a.json'
        with self.assertRaises(ValueError):
            a.validate_registry({'version': 1, 'repositories': [entry, entry]})

    def test_filtered_scope_cannot_send_partial_shared_digest(self):
        with self.assertRaises(SystemExit):
            a.run(['report', '--repo', 'dashpay/a', '--send-slack'])
        with self.assertRaises(SystemExit):
            a.run(['report', '--user', 'Alice', '--send-slack'])

    def test_registered_policy_identity_mismatch_fails_closed(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            (root / 'a.json').write_text(json.dumps(self.policy('dashpay/other')))
            with self.assertRaises(ValueError):
                a.load_policy({'repository': 'dashpay/a', 'mode': 'active', 'policy': 'a.json', 'engine_revision': None}, root)

    def test_registry_rejects_noninteger_version_unknown_fields_and_bad_entries(self):
        valid = {'repository': 'dashpay/a', 'policy': 'a.json', 'mode': 'preview', 'engine_revision': None}
        invalid = [
            {'version': 1, 'repositories': [dict(valid, engine_revision='main')]},
            {'version': 1, 'repositories': [dict(valid, policy='nested/a.json')]},
            {'version': 1.0, 'repositories': [valid]},
            {'version': True, 'repositories': [valid]},
            {'version': 1, 'repositories': []},
            {'version': 1, 'repositories': [None]},
            {'version': 1, 'repositories': [valid], 'extra': True},
            {'version': 1, 'repositories': [dict(valid, extra=True)]},
        ]
        for registry in invalid:
            with self.subTest(registry=registry), self.assertRaises(ValueError):
                a.validate_registry(registry)
