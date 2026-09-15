"""Nudging a silent review bot, and eventually proceeding without it."""

from datetime import datetime, timedelta
import unittest

from pr_review.policy import bot_schedule, evaluate, fingerprint
from pr_review.tests.test_policy import fixture, NOW


def ago(hours):
    """An ISO timestamp that many hours before the fixture's `now`."""
    moment = datetime.fromisoformat(NOW.replace('Z', '+00:00')) - timedelta(hours=hours)
    return moment.isoformat().replace('+00:00', 'Z')


def waiting(hours_since_seen=0, **overrides):
    """A PR whose awaited bot has not reported, first seen `hours_since_seen` ago."""
    policy, pr = fixture()
    policy['bot_timeouts'] = {'nudge_after_hours': 6, 'waive_after_hours': 16}
    pr['reviews'] = [r for r in pr['reviews'] if r['user'] != 'thepastaclaw']
    pr['head_seen_at'] = ago(hours_since_seen)
    pr.update(overrides)
    return policy, pr


class BotTimeoutTests(unittest.TestCase):
    def plan(self, hours, state=None, comments=None, bot='thepastaclaw'):
        policy, pr = waiting(hours)
        if comments is not None:
            pr['comments'] = comments
        return bot_schedule(policy, pr, bot, NOW, state)

    def test_nothing_happens_before_the_window(self):
        self.assertEqual(self.plan(2), {'nudge': False, 'waived_at': None})

    def test_an_unheard_of_head_is_nudged_once_the_window_passes(self):
        self.assertTrue(self.plan(7)['nudge'])

    def test_a_failed_review_is_nudged_at_once(self):
        self.assertTrue(self.plan(0, 'failed')['nudge'])

    def test_a_running_or_queued_review_is_left_alone(self):
        for state in ('running', 'queued'):
            with self.subTest(state=state):
                self.assertFalse(self.plan(7, state)['nudge'])

    def test_a_bot_is_asked_only_once_for_a_head(self):
        policy, pr = waiting(7)
        plan = bot_schedule(policy, pr, 'thepastaclaw', NOW)
        self.assertTrue(plan['nudge'])
        pr['comments'] = [{'user': 'github-actions[bot]', 'created_at': NOW, 'updated_at': NOW,
                           'body': f"<!-- pr-hygiene-nudge v1 bot=thepastaclaw sha={pr['head']} -->\n@thepastaclaw review"}]
        self.assertFalse(bot_schedule(policy, pr, 'thepastaclaw', NOW)['nudge'])

    def test_a_nudge_for_another_head_does_not_count(self):
        policy, pr = waiting(7)
        pr['comments'] = [{'user': 'github-actions[bot]', 'created_at': NOW, 'updated_at': NOW,
                           'body': "<!-- pr-hygiene-nudge v1 bot=thepastaclaw sha=" + 'f' * 40 + " -->"}]
        self.assertTrue(bot_schedule(policy, pr, 'thepastaclaw', NOW)['nudge'])

    def test_coderabbit_announcing_its_own_limit_is_retried_not_awaited(self):
        policy, pr = waiting(2)
        limited = {'user': 'coderabbitai[bot]', 'created_at': ago(1.5), 'updated_at': ago(1.5),
                   'body': '<!-- This is an auto-generated comment: rate limited by coderabbit.ai -->\nwait'}
        pr['comments'] = [limited]
        # Two hours in, the ordinary window has not passed, but the announced limit has.
        self.assertTrue(bot_schedule(policy, pr, 'coderabbitai', NOW)['nudge'])
        pr['comments'] = [dict(limited, created_at=ago(0.2), updated_at=ago(0.2))]
        self.assertFalse(bot_schedule(policy, pr, 'coderabbitai', NOW)['nudge'])

    def test_the_waiver_arrives_on_time_without_any_telemetry(self):
        self.assertIsNone(self.plan(15)['waived_at'])
        self.assertIsNotNone(self.plan(17)['waived_at'])

    def test_the_status_page_cannot_postpone_a_waiver(self):
        for state in (None, 'running', 'queued', 'failed'):
            with self.subTest(state=state):
                self.assertIsNone(self.plan(15, state)['waived_at'])
                self.assertIsNotNone(self.plan(17, state)['waived_at'])

    def test_the_waiver_instant_does_not_move_with_the_clock(self):
        policy, pr = waiting(20)
        first = bot_schedule(policy, pr, 'thepastaclaw', NOW)['waived_at']
        later = bot_schedule(policy, pr, 'thepastaclaw', ago(-5))['waived_at']
        self.assertEqual(first, later)
        self.assertLess(first, NOW)

    def test_a_repository_without_timeouts_never_nudges_or_waives(self):
        policy, pr = waiting(100)
        del policy['bot_timeouts']
        self.assertEqual(bot_schedule(policy, pr, 'thepastaclaw', NOW), {'nudge': False, 'waived_at': None})

    def test_a_head_the_controller_has_not_reported_on_has_no_clock(self):
        policy, pr = waiting(100, head_seen_at=None)
        self.assertEqual(bot_schedule(policy, pr, 'thepastaclaw', NOW), {'nudge': False, 'waived_at': None})


class WaiverTests(unittest.TestCase):
    def test_a_waiver_unblocks_the_pull_request_and_is_stated(self):
        policy, pr = waiting(20)
        result = evaluate(policy, pr, NOW, NOW)
        self.assertEqual(result['waived'], ['thepastaclaw'])
        self.assertIn('Proceeded without thepastaclaw: no review within the configured window',
                      result['blockers'])
        # The named state, not merely "not waiting-bots": a waiver that left the
        # pull request stuck somewhere else would still be a waiver that failed.
        self.assertEqual((result['state'], result['status']), ('ready-to-merge', 'success'))

    def test_a_sole_required_bot_can_be_waived_or_the_feature_is_pointless(self):
        policy, pr = waiting(20)
        policy['required_bots'] = ['thepastaclaw']
        pr['reviews'] = [r for r in pr['reviews'] if r['user'] != 'coderabbitai[bot]']
        result = evaluate(policy, pr, NOW, NOW)
        self.assertEqual(result['waived'], ['thepastaclaw'])
        self.assertEqual(result['status'], 'success')

    def test_a_self_review_written_before_the_waiver_does_not_count(self):
        policy, pr = waiting(20)
        pr['comments'] = [dict(pr['comments'][0], created_at=ago(19), updated_at=ago(19))]
        self.assertEqual(evaluate(policy, pr, NOW, NOW)['state'], 'waiting-self-review')

    def test_an_objection_on_the_current_head_still_blocks_and_is_never_waived(self):
        policy, pr = waiting(20)
        pr['reviews'].append({'id': 9, 'user': 'coderabbitai[bot]', 'state': 'CHANGES_REQUESTED',
                              'commit_id': pr['head'], 'submitted_at': NOW, 'body': ''})
        result = evaluate(policy, pr, NOW, NOW)
        self.assertEqual(result['state'], 'waiting-bots')
        self.assertIn('Bot changes request remains outstanding', result['blockers'])

    def test_an_objection_against_an_earlier_head_does_not_block_forever(self):
        policy, pr = waiting(20)
        pr['reviews'].append({'id': 9, 'user': 'coderabbitai[bot]', 'state': 'CHANGES_REQUESTED',
                              'commit_id': 'e' * 40, 'submitted_at': '2026-09-10T00:00:00Z', 'body': ''})
        self.assertNotIn('Bot changes request remains outstanding', evaluate(policy, pr, NOW, NOW)['blockers'])

    def test_a_bot_is_not_asked_while_the_pull_request_owes_it_an_answer(self):
        policy, pr = waiting(7)
        pr['threads'] = [{'id': 't1', 'is_resolved': False, 'author': 'coderabbitai[bot]',
                          'created_at': ago(8)}]
        self.assertEqual(evaluate(policy, pr, NOW, NOW)['nudge'], [])
        pr['threads'] = []
        self.assertEqual(evaluate(policy, pr, NOW, NOW)['nudge'], ['thepastaclaw'])

    def test_asking_a_bot_to_review_is_not_mistaken_for_changed_evidence(self):
        _, pr = waiting(7)
        before = fingerprint(pr)
        pr['comments'].append({'user': 'github-actions[bot]', 'created_at': NOW, 'updated_at': NOW,
                               'body': f"<!-- pr-hygiene-nudge v1 bot=thepastaclaw sha={pr['head']} -->\n@thepastaclaw review"})
        self.assertEqual(fingerprint(pr), before)



class TelemetryReaderTests(unittest.TestCase):
    """The status page is a third party's: it may inform, never obstruct."""

    def payload(self, **live):
        base = {'schema_version': 1, 'data_as_of': NOW,
                'live': {'active': [], 'heads': {}, 'capacity': {}},
                'history': {'recent_events': [], 'daily': []}}
        base['live'].update(live)
        return base

    def state(self, payload, head='a' * 40, seen=None):
        from pr_review import telemetry
        return telemetry.head_state(payload, 'dashpay/x', 7, head, seen, NOW)

    def test_rubbish_is_simply_absent(self):
        for payload in [None, {}, {'schema_version': 2}, {'schema_version': 1, 'live': 'nonsense'}]:
            with self.subTest(payload=payload):
                self.assertIsNone(self.state(payload))

    def test_a_stale_page_is_ignored_entirely(self):
        payload = self.payload(active=[{'repo': 'dashpay/x', 'number': 7, 'sha': 'a' * 40,
                                        'status': 'running', 'heartbeat_at': NOW, 'deadline_at': ago(-1)}])
        payload['data_as_of'] = ago(48)
        self.assertIsNone(self.state(payload))

    def test_a_run_whose_heartbeat_stopped_is_not_running(self):
        entry = {'repo': 'dashpay/x', 'number': 7, 'sha': 'a' * 40, 'status': 'running',
                 'heartbeat_at': NOW, 'deadline_at': ago(-1)}
        self.assertEqual(self.state(self.payload(active=[entry])), 'running')
        self.assertIsNone(self.state(self.payload(active=[dict(entry, heartbeat_at=ago(3))])))
        self.assertIsNone(self.state(self.payload(active=[dict(entry, deadline_at=ago(1))])))

    def test_a_failure_before_this_head_appeared_belongs_to_an_earlier_push(self):
        payload = self.payload()
        payload['history']['recent_events'] = [
            {'kind': 'head.failed', 'repo': 'dashpay/x', 'number': 7, 'ts': ago(5), 'detail': 'after 2 attempt(s)'}]
        self.assertEqual(self.state(payload, seen=ago(9)), 'failed')
        self.assertIsNone(self.state(payload, seen=ago(2)))

    def test_a_queued_event_is_matched_by_its_short_commit(self):
        payload = self.payload()
        payload['history']['recent_events'] = [
            {'kind': 'head.queued', 'repo': 'dashpay/x', 'number': 7, 'ts': ago(1),
             'detail': 'aaaaaaaa trigger=new_push priority=0'}]
        self.assertEqual(self.state(payload, seen=ago(9)), 'queued')
        self.assertIsNone(self.state(payload, head='b' * 40, seen=ago(9)))

if __name__ == '__main__':
    unittest.main()
