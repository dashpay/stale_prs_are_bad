"""Pin the responsibility decisions that differ from the original team roster."""

import json
import unittest

from pr_review.policy import codeowners, validate_policy
from pr_review.registry import POLICIES


class RosterTests(unittest.TestCase):
    def test_sheet_roles_do_not_promote_reviewers_or_broad_teams(self):
        policy = json.loads((POLICIES / 'platform.json').read_text())
        validate_policy(policy)
        self.assertEqual(policy['fallback'], {
            'owners': ['QuantumExplorer', 'shumkov'], 'reviewers': []})
        paths = {path: area for area in policy['areas'] for path in area['paths']}
        self.assertEqual(paths['packages/rs-drive/']['owners'], ['QuantumExplorer'])
        self.assertEqual(paths['packages/rs-drive/']['reviewers'], ['shumkov'])
        self.assertEqual(paths['packages/rs-platform-wallet/']['owners'], ['llbartekll'])
        self.assertEqual(set(paths['packages/rs-platform-wallet/']['reviewers']),
                         {'ZocoLini', 'HashEngineering', 'romchornyi'})
        self.assertEqual(paths['packages/rs-dapi/']['owners'], ['lklimek'])
        self.assertEqual(paths['packages/dashmate/']['owners'], ['shumkov', 'ktechmidas'])
        self.assertNotIn('unresolved', paths['packages/dashmate/'])
        everyone = {x.lower() for a in policy['areas'] for x in a['owners'] + a['reviewers']}
        self.assertFalse({'strophy', 'silvanassss'} & everyone)
        self.assertFalse(any(x.startswith('dashpay/') for x in everyone), 'people, not teams')


if __name__ == '__main__':
    unittest.main()
