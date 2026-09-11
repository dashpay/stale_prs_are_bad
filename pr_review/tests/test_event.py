import json
import os
from pathlib import Path
import tempfile
import unittest
from unittest.mock import patch

from pr_review import event
from pr_review.event import selections


class EventTests(unittest.TestCase):
    def test_schedule_push_and_dispatch_are_bounded(self):
        for kind in ['schedule','push','workflow_dispatch']:
            self.assertEqual(selections(kind,{}),[['--batch-size','3']])

    def test_event_number_only_selects_freshly_refetched_pr(self):
        self.assertEqual(selections('issue_comment',{'issue':{'number':44,'pull_request':{}}}),[['--pr','44']])
        self.assertEqual(selections('issue_comment',{'issue':{'number':44}}),[])
        with self.assertRaises(ValueError):
            selections('pull_request_target',{'pull_request':{'number':'$(secret)'}})

    def test_empty_fork_signal_does_not_trigger_full_sweep(self):
        self.assertEqual(selections('workflow_run',{'workflow_run':{'pull_requests':[]}}),[])

    def test_workflow_passes_calling_repository_and_both_roots_to_the_engine(self):
        with tempfile.TemporaryDirectory() as directory:
            payload = Path(directory) / 'event.json'
            payload.write_text(json.dumps({'pull_request': {'number': 7}}))
            environment = {'GITHUB_EVENT_PATH': str(payload), 'GITHUB_EVENT_NAME': 'pull_request_target',
                           'GITHUB_REPOSITORY': 'dashpay/tenderdash', 'PR_REVIEW_POLICIES_ROOT': '/live/policies',
                           'PR_REVIEW_REPOSITORY_ROOT': '/target'}
            with patch.dict(os.environ, environment), patch.object(event, 'run', return_value=0) as run:
                self.assertEqual(event.main(), 0)
            self.assertEqual(run.call_args.args[0], ['sync', '--repo', 'dashpay/tenderdash', '--repository-root', '/target',
                                                     '--policies-root', '/live/policies', '--pr', '7'])


if __name__ == '__main__':
    unittest.main()
