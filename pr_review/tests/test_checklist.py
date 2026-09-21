"""The checklist in the description, the move comments, and what the fast path compares."""

import copy
import json
import re
import unittest
from unittest.mock import Mock, patch

from pr_review import main
from pr_review.github import GitHub, current_checklist
from pr_review.policy import CHECKLIST_END, CHECKLIST_START, MOVE_MARKER, evaluate
from pr_review.tests.test_policy import HEAD, NOW, fixture

LATER = '2026-09-11T14:00:00Z'


def bartek():
    """dashpay/platform#4818: an approval in hand that covered nothing it needed."""
    policy = json.load(open('policies/platform.json'))
    _, pr = fixture()
    pr.update(author='llbartekll', base='v4.2-dev', build='green', head_seen_at='2026-09-11T09:00:00Z', body='',
              files=[{'filename': 'packages/swift-sdk/Sources/a.swift'}] + [{'filename': f} for f in (
                  '.editorconfig', '.github/workflows/swift-sdk-build.yml', '.github/workflows/tests.yml', 'AGENTS.md')],
              comments=[dict(id=1, user='llbartekll', body='/skip-bots', created_at='2026-09-11T10:00:00Z',
                             updated_at='2026-09-11T10:00:00Z')],
              reviews=[dict(id=5, user='romchornyi', state='APPROVED', commit_id=HEAD, submitted_at='2026-09-11T10:30:00Z', body='')],
              threads=[], labels=[], requested_reviewers=[],
              permissions={'llbartekll': 'write', 'romchornyi': 'write', 'QuantumExplorer': 'admin', 'shumkov': 'admin'})
    return policy, pr


def first_unchecked(block):
    for line in block.splitlines():
        if line.lstrip().startswith('- [ ]'):
            return line.strip()
    return None


class ChecklistTests(unittest.TestCase):
    def test_every_requirement_is_shown_and_the_first_unchecked_is_the_state(self):
        policy, pr = bartek()
        result = evaluate(policy, pr, NOW, LATER)
        block = main.checklist_block(result)
        self.assertEqual(result['state'], 'waiting-self-review')
        self.assertTrue(block.startswith(CHECKLIST_START) and block.endswith(CHECKLIST_END))
        self.assertIn('- [x] Bots — coderabbitai skipped by llbartekll · thepastaclaw skipped by llbartekll', block)
        self.assertIn('- [x] Build green', block)
        self.assertIn('- [ ] Self-review — post `/self-reviewed`', block)
        self.assertIn('  - [x] `swift-sdk` — you own it', block)
        self.assertIn('  - [ ] files with no dedicated owner (`.editorconfig`, `.github/workflows/swift-sdk-build.yml`, '
                      '`.github/workflows/tests.yml` and 1 more) — QuantumExplorer or shumkov', block)
        self.assertIn('- [x] Within your 5 open PRs', block)
        self.assertTrue(first_unchecked(block).startswith('- [ ] Self-review'), 'the state is the first unchecked line')
        self.assertNotIn('@', block, 'a mention from this bot notifies')
        self.assertNotIn('\n- [ ] Approvals', block, 'a task parent would double-count beside its children')

    def test_after_the_attestation_only_the_uncovered_files_are_left(self):
        policy, pr = bartek()
        pr['comments'].append(dict(id=2, user='llbartekll', body='/self-reviewed', created_at='2026-09-11T11:00:00Z',
                                   updated_at='2026-09-11T11:00:00Z'))
        result = evaluate(policy, pr, NOW, LATER)
        self.assertEqual(result['state'], 'ready-for-human')
        block = main.checklist_block(result)
        self.assertIn('- [x] Self-review — posted; again after any push', block)
        self.assertTrue(first_unchecked(block).startswith('- [ ] files with no dedicated owner'))
        self.assertEqual(main.move_text(result).splitlines()[1], 'Ready for review — needs QuantumExplorer or shumkov.')

    def test_an_owner_sees_no_approvals_needed_and_a_red_build_named(self):
        policy, pr = fixture()
        pr['build'] = 'failed'
        result = evaluate(policy, pr, NOW, NOW)
        block = main.checklist_block(result)
        self.assertEqual(result['state'], 'waiting-build')
        self.assertIn('- [ ] Build failed', block)
        self.assertIn('- [x] Approvals — you own every area touched; none needed', block)
        self.assertTrue(first_unchecked(block).startswith('- [ ] Build'))

    def test_an_objection_before_the_attestation_is_the_authors_to_address(self):
        policy, pr = fixture()
        pr['author'] = pr['comments'][0]['user'] = 'reviewer'
        pr['threads'] = [dict(id=9, author='owner', is_resolved=False, created_at='2026-09-11T12:30:00Z',
                              voices=[dict(user='owner', created_at='2026-09-11T12:30:00Z')])]
        result = evaluate(policy, pr, NOW, '2026-09-11T13:00:00Z')
        self.assertEqual(result['state'], 'waiting-author')
        block = main.checklist_block(result)
        self.assertIn('- [ ] Self-review — address owner left a review thread unresolved, then post `/self-reviewed`', block)
        self.assertIn('your move: address owner left a review thread unresolved, then post `/self-reviewed`', main.move_text(result))

    def test_an_answered_objection_waits_on_the_objector_under_approvals(self):
        policy, pr = fixture()
        pr['author'] = pr['comments'][0]['user'] = 'reviewer'
        pr['threads'] = [dict(id=9, author='owner', is_resolved=False, created_at='2026-09-11T09:30:00Z',
                              voices=[dict(user='owner', created_at='2026-09-11T09:30:00Z')])]
        result = evaluate(policy, pr, NOW, NOW)
        self.assertEqual(result['state'], 'ready-for-human')
        block = main.checklist_block(result)
        self.assertIn('  - [ ] owner left a review thread unresolved — waiting for them to re-review or dismiss', block)
        self.assertIn('- [x] Self-review — posted', block)

    def test_beyond_the_limit_is_said_positively_and_unchecked(self):
        policy, pr = fixture()
        pr['author'] = pr['comments'][0]['user'] = 'reviewer'
        result = evaluate(policy, pr, None, NOW)
        self.assertEqual(result['state'], 'waiting-slot')
        block = main.checklist_block(result)
        self.assertIn('- [ ] Within your 5 open PRs — this one is beyond the limit; it waits until one merges', block)
        self.assertTrue(first_unchecked(block).startswith('- [ ] Within'))

    def test_the_skip_is_offered_only_while_a_bot_is_still_owed(self):
        policy, pr = bartek()
        pr['comments'] = []
        result = evaluate(policy, pr, NOW, LATER)
        self.assertEqual(result['state'], 'waiting-bots')
        block = main.checklist_block(result)
        self.assertIn('coderabbitai not yet · thepastaclaw not yet — `/skip-bots` proceeds without the ones not yet reported', block)
        self.assertIn('- [ ] Self-review — post `/self-reviewed` once the bots are done', block)
        self.assertIsNone(main.move_text(result), 'nobody\'s move: nothing is announced')

    def test_the_skip_is_not_offered_once_every_missing_bot_is_already_waived(self):
        policy, pr = bartek()
        pr['comments'] = []
        pr['head_seen_at'] = '2026-09-09T00:00:00Z'      # the window has long run out
        result = evaluate(policy, pr, NOW, LATER)
        self.assertEqual(result['waived'], ['coderabbitai', 'thepastaclaw'])
        self.assertNotIn('/skip-bots', main.checklist_block(result))

    def test_a_bot_that_reported_and_still_objects_says_both(self):
        # dashpay/platform#4392: thepastaclaw left its final receipt and two
        # threads unresolved. "thepastaclaw ✓" beside an open box explained
        # nothing.
        policy, pr = bartek()
        pr['comments'] = []
        pr['head_seen_at'] = '2026-09-09T00:00:00Z'
        pr['threads'] = [dict(id=i, author='thepastaclaw', is_resolved=False, created_at=NOW,
                              voices=[dict(user='thepastaclaw', created_at=NOW)]) for i in (1, 2)]
        pr['reviews'].append(dict(id=7, user='thepastaclaw', state='COMMENTED', commit_id=HEAD, submitted_at=NOW,
                                  body=f'<!-- thepastaclaw-review-phase v1 phase=final sha={HEAD} -->'))
        result = evaluate(policy, pr, NOW, LATER)
        self.assertEqual(result['state'], 'waiting-bots')
        block = main.checklist_block(result)
        self.assertIn('- [ ] Bots — coderabbitai skipped after the window · thepastaclaw ✓, 2 threads unresolved — resolve them', block)
        pr['threads'] = []
        pr['reviews'].append(dict(id=8, user='thepastaclaw', state='CHANGES_REQUESTED', commit_id=HEAD, submitted_at=LATER, body='no'))
        block = main.checklist_block(evaluate(policy, pr, NOW, LATER))
        self.assertIn('thepastaclaw ✓, requested changes — dismiss the review or push a fix', block)

    def test_a_path_is_never_quoted_unsafely(self):
        policy, pr = bartek()
        pr['files'].append({'filename': 'x/<!-- pr-hygiene:end -->.rs'})
        pr['files'].append({'filename': 'y/`z`.rs'})
        block = main.checklist_block(evaluate(policy, pr, NOW, LATER))
        self.assertEqual(block.count(CHECKLIST_END), 1)
        self.assertNotIn('`z`', block)

    def test_a_draft_gets_no_block(self):
        policy, pr = fixture()
        pr['draft'] = True
        self.assertIsNone(main.checklist_block(evaluate(policy, pr, NOW, NOW)))


class DescriptionWriteTests(unittest.TestCase):
    def test_only_the_block_is_written_and_the_authors_text_is_kept_byte_for_byte(self):
        api = GitHub('dashpay/platform')
        author = "## Why\r\n\r\nBecause.\r\n\r\n- [ ] I have performed a self-review of my own code\r\n"
        old_block = f"{CHECKLIST_START}\nstale\n{CHECKLIST_END}"
        writes = []
        with patch.object(api, 'request', side_effect=lambda m, p, payload=None: (
                {'body': author + '\n' + old_block + '\n<!-- coderabbit -->\ntrailing'} if m == 'GET' else writes.append(payload))):
            api.set_checklist(1, f"{CHECKLIST_START}\nfresh\n{CHECKLIST_END}")
        body = writes[0]['body']
        self.assertTrue(body.startswith(author.rstrip()), 'the author\'s text, CRLF and all, untouched')
        self.assertIn('fresh', body)
        self.assertNotIn('stale', body)
        self.assertEqual(body.count(CHECKLIST_START), 1)
        self.assertTrue(body.endswith('\n<!-- coderabbit -->\ntrailing'), 'what follows the block is someone else\'s and stays')

    def test_an_identical_block_is_not_rewritten_even_when_line_endings_differ(self):
        api = GitHub('dashpay/platform')
        block = f"{CHECKLIST_START}\nsame\n{CHECKLIST_END}"
        with patch.object(api, 'request', return_value={'body': 'text\r\n\r\n' + block.replace('\n', '\r\n')}) as request:
            self.assertFalse(api.set_checklist(1, block))
        self.assertEqual([c.args[0] for c in request.call_args_list], ['GET'])

    def test_a_block_that_would_not_fit_is_refused_not_truncated(self):
        api = GitHub('dashpay/platform')
        with patch.object(api, 'request', return_value={'body': 'x' * 65000}):
            with self.assertRaises(main.GitHubError):
                api.set_checklist(1, f"{CHECKLIST_START}\n{'y' * 1000}\n{CHECKLIST_END}")

    def test_the_last_marker_pair_is_the_engines(self):
        quoted = f"```\n{CHECKLIST_START}\nexample\n{CHECKLIST_END}\n```"
        real = f"{CHECKLIST_START}\nreal\n{CHECKLIST_END}"
        self.assertEqual(current_checklist(quoted + '\n\n' + real), real)
        self.assertIsNone(current_checklist(f"{CHECKLIST_START}\nno end"))


class PublishTests(unittest.TestCase):
    """What publish writes, and what it leaves alone."""

    def run_publish(self, pr, result, api=None):
        api = api or Mock()
        api.state_comment_body.side_effect = GitHub.state_comment_body
        api.snapshot.return_value = copy.deepcopy(pr)
        api.pull.return_value = copy.deepcopy(pr)
        api.open_prs.return_value = [copy.deepcopy(pr)]
        api.histories.side_effect = lambda numbers: {n: {'comments': pr.get('comments', []), 'lifecycle_at': None} for n in numbers}
        with patch.object(main, 'load_histories', side_effect=lambda a, s: [copy.deepcopy(pr)]):
            main.publish(api, self.policy, pr, result, [pr], apply=True)
        return api

    def setUp(self):
        self.policy, self.pr = bartek()
        self.pr['controller_state'] = None

    def test_a_move_is_announced_once_with_the_record_and_the_block_written(self):
        result = evaluate(self.policy, self.pr, NOW, LATER)
        api = self.run_publish(self.pr, result)
        api.set_checklist.assert_called_once()
        self.assertEqual(api.set_checklist.call_args.args[1], main.checklist_block(result))
        api.upsert_state.assert_called_once()
        record, body, comment_id = api.upsert_state.call_args.args[1:]
        self.assertEqual(record['state'], 'waiting-self-review')
        self.assertTrue(body.startswith(f'{MOVE_MARKER} state=waiting-self-review sha={HEAD} -->'))
        self.assertIsNone(comment_id, 'a new comment: that is what notifies')

    def test_nothing_is_rewritten_when_description_labels_and_comment_already_agree(self):
        result = evaluate(self.policy, self.pr, NOW, LATER)
        record = main.state_record(self.pr, result, main.context_fingerprint([self.pr], self.pr['author']))
        pr = dict(self.pr, body='text\n\n' + main.checklist_block(result), labels=['waiting-self-review', 'bot-review-skipped'])
        pr['comments'] = pr['comments'] + [dict(id=50, user='github-actions[bot]', created_at=NOW, updated_at=NOW,
                                                 body=GitHub.state_comment_body(record, main.move_text(result)))]
        pr['controller_state'] = record
        api = self.run_publish(pr, evaluate(self.policy, pr, NOW, LATER))
        api.set_checklist.assert_not_called()
        api.upsert_state.assert_not_called()
        api.set_state_label.assert_not_called()

    def test_a_hand_ticked_box_or_a_wiped_block_is_repaired(self):
        result = evaluate(self.policy, self.pr, NOW, LATER)
        ticked = main.checklist_block(result).replace('- [ ] Self-review', '- [x] Self-review')
        pr = dict(self.pr, body='text\n\n' + ticked, labels=['waiting-self-review', 'bot-review-skipped'])
        api = self.run_publish(pr, evaluate(self.policy, pr, NOW, LATER))
        api.set_checklist.assert_called_once()
        wiped = dict(self.pr, body='text only, block gone', labels=['waiting-self-review', 'bot-review-skipped'])
        api = self.run_publish(wiped, evaluate(self.policy, wiped, NOW, LATER))
        api.set_checklist.assert_called_once()

    def test_a_repeat_within_a_day_edits_the_announcement_and_later_posts_anew(self):
        result = evaluate(self.policy, self.pr, NOW, LATER)
        stale = GitHub.state_comment_body(dict(main.state_record(self.pr, result, 'c' * 64), head='b' * 40),
                                          main.move_text(result).replace(HEAD, 'b' * 40))
        for age, expect_new in (('2026-09-11T02:00:00Z', False), ('2026-09-09T02:00:00Z', True)):
            pr = dict(self.pr, body='text\n\n' + main.checklist_block(result), labels=['waiting-self-review', 'bot-review-skipped'])
            pr['comments'] = pr['comments'] + [dict(id=50, user='github-actions[bot]', created_at=age, updated_at=age, body=stale)]
            with patch.object(main, 'utc_now', return_value=LATER):
                api = self.run_publish(pr, evaluate(self.policy, pr, NOW, LATER))
            api.upsert_state.assert_called_once()
            self.assertEqual(api.upsert_state.call_args.args[3], None if expect_new else 50, age)

    def test_the_old_standing_comment_points_at_the_description_then_goes(self):
        result = evaluate(self.policy, self.pr, NOW, LATER)
        old_record = main.state_record(self.pr, dict(result, state='waiting-bots'), 'c' * 64)
        standing = dict(id=7, user='github-actions[bot]', created_at='2026-09-10T00:00:00Z', updated_at='2026-09-10T00:00:00Z',
                        body=GitHub.state_comment_body(old_record, '### PR Hygiene\nState: **waiting-bots**'))
        # No move yet: the old comment is kept as the record, its text repointed.
        quiet = dict(self.pr, comments=[]); quiet['comments'] = [standing]
        quiet_result = evaluate(self.policy, quiet, NOW, LATER)
        self.assertEqual(quiet_result['state'], 'waiting-bots')
        api = self.run_publish(quiet, quiet_result)
        api.upsert_state.assert_called_once()
        self.assertEqual(api.upsert_state.call_args.args[2:], (main.POINTER, 7))
        api.delete_comment.assert_not_called()
        # A move: the announcement carries the record, the old comment is removed.
        moving = dict(self.pr); moving['comments'] = self.pr['comments'] + [standing]
        api = self.run_publish(moving, evaluate(self.policy, moving, NOW, LATER))
        self.assertIsNone(api.upsert_state.call_args.args[3])
        api.delete_comment.assert_called_once_with(7)

    def test_a_bot_author_is_not_told_its_move(self):
        pr = dict(self.pr, author_is_bot=True, comments=[])
        result = evaluate(self.policy, pr, NOW, LATER)
        self.assertEqual(result['state'], 'waiting-bots')
        pr['comments'] = [dict(id=1, user='llbartekll', body='/skip-bots', created_at='2026-09-11T10:00:00Z', updated_at='2026-09-11T10:00:00Z')]
        result = evaluate(self.policy, pr, NOW, LATER)
        self.assertEqual(result['state'], 'ready-for-human', 'a bot author skips the attestation')
        api = self.run_publish(pr, result)
        self.assertTrue(api.upsert_state.called, 'the reviewer is told; that is not the bot')

    def test_a_description_too_long_is_a_warning_not_an_error(self):
        result = evaluate(self.policy, self.pr, NOW, LATER)
        api = Mock()
        api.set_checklist.side_effect = main.GitHubError('Description too long for the checklist')
        with patch('sys.stderr'):
            api = self.run_publish(self.pr, result, api)
        self.assertFalse(any(c.args[1] == 'error' for c in api.post_status.call_args_list))
        api.upsert_state.assert_called_once()


class RecordTests(unittest.TestCase):
    """The record is refreshed silently; a comment is posted only for news."""

    def setUp(self):
        self.policy, self.pr = bartek()
        self.pr['controller_state'] = None

    def publish(self, pr, result, now=LATER):
        api = Mock()
        api.state_comment_body.side_effect = GitHub.state_comment_body
        api.snapshot.return_value = copy.deepcopy(pr)
        api.pull.return_value = copy.deepcopy(pr)
        api.open_prs.return_value = [copy.deepcopy(pr)]
        with patch.object(main, 'load_histories', side_effect=lambda a, s: [copy.deepcopy(pr)]), \
             patch.object(main, 'utc_now', return_value=now):
            main.publish(api, self.policy, pr, result, [pr], apply=True)
        return api

    def settled(self, pr, result, move_comment_id=50, created_at=NOW):
        """A pull request whose description, labels and announcement already match `result`."""
        record = main.state_record(pr, result, main.context_fingerprint([pr], pr['author']))
        settled = dict(pr, body='text\n\n' + main.checklist_block(result),
                       labels=[main.LABEL_FOR_STATE.get(result['state'])] + (['bot-review-skipped'] if result.get('waived') else []),
                       requested_reviewers=list(result.get('reviewers') or []))
        settled['labels'] = [l for l in settled['labels'] if l]
        settled['comments'] = pr['comments'] + [dict(id=move_comment_id, user='github-actions[bot]', created_at=created_at,
                                                     updated_at=created_at,
                                                     body=GitHub.state_comment_body(record, main.move_text(result)))]
        settled['controller_state'] = record
        settled['controller_comment_id'] = move_comment_id
        return settled

    def test_an_evidence_only_change_refreshes_the_record_in_place_and_posts_nothing(self):
        result = evaluate(self.policy, self.pr, NOW, LATER)
        pr = self.settled(self.pr, result)
        # A reviewer's plain comment: evidence changed, nothing else did.
        pr['comments'].append(dict(id=60, user='romchornyi', body='looks fine', created_at=LATER, updated_at=LATER))
        again = evaluate(self.policy, pr, NOW, LATER)
        api = self.publish(pr, again)
        api.upsert_state.assert_not_called()
        self.assertFalse(any(c.args[1] == 'pending' and 'Evaluating' in c.args[2] for c in api.post_status.call_args_list),
                         'the fast path: the record holds only what matters, not the evidence fingerprint')

    def test_a_state_change_that_is_not_a_move_refreshes_the_record_silently(self):
        # The build scan selects on the recorded state. waiting-build is not a
        # move, so it must still reach the record — by editing the newest
        # holder, never by a new comment.
        result = evaluate(self.policy, self.pr, NOW, LATER)
        pr = self.settled(self.pr, result)
        pr['comments'].append(dict(id=2, user='llbartekll', body='/self-reviewed', created_at=LATER, updated_at=LATER))
        pr['build'] = 'running'
        later = evaluate(self.policy, pr, NOW, '2026-09-11T15:00:00Z')
        self.assertEqual(later['state'], 'waiting-build')
        api = self.publish(pr, later, now='2026-09-11T15:00:00Z')
        api.upsert_state.assert_called_once()
        record, text, comment_id = api.upsert_state.call_args.args[1:]
        self.assertEqual(record['state'], 'waiting-build')
        self.assertEqual(comment_id, 50, 'the newest holder, edited')
        self.assertIn('Bots are done', text, 'its words unchanged')

    def test_the_first_pass_converts_a_standing_record_of_this_move_rather_than_announcing_it(self):
        pr = dict(self.pr)
        pr['comments'] = pr['comments'] + [dict(id=2, user='llbartekll', body='/self-reviewed', created_at=LATER, updated_at=LATER)]
        result = evaluate(self.policy, pr, NOW, '2026-09-11T15:00:00Z')
        self.assertEqual(result['state'], 'ready-for-human')
        record = main.state_record(pr, result, 'c' * 64)
        standing = dict(id=7, user='github-actions[bot]', created_at='2026-09-10T00:00:00Z', updated_at='2026-09-10T00:00:00Z',
                        body=GitHub.state_comment_body(record, '### PR Hygiene\nState: **ready-for-human**'))
        pr['comments'] = pr['comments'] + [standing]
        pr['controller_state'] = record
        api = self.publish(pr, evaluate(self.policy, pr, NOW, '2026-09-11T15:00:00Z'), now='2026-09-11T15:00:00Z')
        api.upsert_state.assert_called_once()
        self.assertEqual(api.upsert_state.call_args.args[3], 7, 'the standing comment becomes the announcement in place')
        self.assertIn('Ready for review', api.upsert_state.call_args.args[2])
        api.delete_comment.assert_not_called()

    def test_ready_since_does_not_drift_between_announcements(self):
        # Two announcements exist. The record is read from whichever was
        # written last, so a refresh of the older one is still the truth.
        result = evaluate(self.policy, self.pr, NOW, LATER)
        pr = self.settled(self.pr, result)                                 # waiting-self-review, id 50
        pr['comments'].append(dict(id=2, user='llbartekll', body='/self-reviewed', created_at=LATER, updated_at=LATER))
        ready = evaluate(self.policy, pr, NOW, '2026-09-11T15:00:00Z')
        self.assertEqual(ready['state'], 'ready-for-human')
        api = self.publish(pr, ready, now='2026-09-11T15:00:00Z')
        record = api.upsert_state.call_args.args[1]
        self.assertEqual(record['ready_since'], '2026-09-11T15:00:00Z')
        # That announcement exists now, written after the first.
        pr['comments'].append(dict(id=51, user='github-actions[bot]', created_at='2026-09-11T15:00:00Z', updated_at='2026-09-11T15:00:00Z',
                                   body=GitHub.state_comment_body(record, main.move_text(ready))))
        parsed, holder = main.parse_controller_state(pr['comments'])
        self.assertEqual((parsed['ready_since'], holder), ('2026-09-11T15:00:00Z', 51))
        pr['controller_state'] = parsed
        pr['body'] = 'text\n\n' + main.checklist_block(ready); pr['labels'] = ['ready-for-human', 'bot-review-skipped']
        pr['requested_reviewers'] = ready['reviewers']
        api = self.publish(pr, evaluate(self.policy, pr, NOW, '2026-09-11T16:00:00Z'), now='2026-09-11T16:00:00Z')
        api.upsert_state.assert_not_called()

    def test_two_standing_comments_converge_to_one(self):
        quiet = dict(self.pr, comments=[])
        result = evaluate(self.policy, quiet, NOW, LATER)
        record = main.state_record(quiet, result, 'c' * 64)
        two = [dict(id=i, user='github-actions[bot]', created_at=f'2026-09-1{i}T00:00:00Z', updated_at=f'2026-09-1{i}T00:00:00Z',
                    body=GitHub.state_comment_body(record, main.POINTER if i == 1 else 'old text')) for i in (0, 1)]
        quiet['comments'] = two
        quiet['controller_state'] = record
        api = self.publish(quiet, evaluate(self.policy, quiet, NOW, LATER))
        self.assertEqual([c.args[0] for c in api.delete_comment.call_args_list], [0], 'all but the record holder go')

    def test_a_description_with_no_room_is_left_alone_on_the_fast_path(self):
        result = evaluate(self.policy, self.pr, NOW, LATER)
        pr = self.settled(self.pr, result)
        pr['body'] = 'x' * 65000
        api = self.publish(pr, evaluate(self.policy, pr, NOW, LATER))
        api.set_checklist.assert_not_called()
        self.assertFalse(any('Evaluating' in c.args[2] for c in api.post_status.call_args_list))

    def test_a_configuration_error_writes_no_block_and_no_announcement(self):
        pr = dict(self.pr, files=[])
        result = evaluate(self.policy, pr, NOW, LATER)
        self.assertEqual(result['state'], 'configuration-error')
        api = self.publish(pr, result)
        api.set_checklist.assert_not_called()
        api.upsert_state.assert_not_called()


class SecondReviewTests(unittest.TestCase):
    """What the cross-model review found."""

    def setUp(self):
        self.policy, self.pr = bartek()
        self.pr['controller_state'] = None

    def publish(self, pr, result, now=LATER, api=None):
        api = api or Mock()
        api.state_comment_body.side_effect = GitHub.state_comment_body
        api.snapshot.return_value = copy.deepcopy(pr)
        api.pull.return_value = copy.deepcopy(pr)
        api.open_prs.return_value = [copy.deepcopy(pr)]
        with patch.object(main, 'load_histories', side_effect=lambda a, s: [copy.deepcopy(pr)]), \
             patch.object(main, 'utc_now', return_value=now):
            main.publish(api, self.policy, pr, result, [pr], apply=True)
        return api

    def test_a_bot_author_still_gets_the_merge_announcement_and_its_record(self):
        # Only the author-directed move is withheld from a bot: "you can merge"
        # is for the humans, and it carries the record.
        pr = dict(self.pr, author_is_bot=True)
        pr['reviews'].append(dict(id=6, user='QuantumExplorer', state='APPROVED', commit_id=HEAD, submitted_at=LATER, body=''))
        result = evaluate(self.policy, pr, NOW, LATER)
        self.assertEqual(result['state'], 'ready-to-merge')
        api = self.publish(pr, result)
        api.upsert_state.assert_called_once()
        self.assertIn('state=ready-to-merge', api.upsert_state.call_args.args[2])

    def test_a_first_waiting_build_with_no_comment_is_found_by_the_scan_through_its_status(self):
        # An attestation naming the commit, posted before the bots finish,
        # skips waiting-self-review: no move, no comment, no record. The scan
        # asks the head's own last status instead.
        pr = dict(self.pr, build='running')
        pr['comments'] = pr['comments'] + [dict(id=2, user='llbartekll', body=f'/self-reviewed {HEAD}',
                                                created_at='2026-09-11T10:30:00Z', updated_at='2026-09-11T10:30:00Z')]
        result = evaluate(self.policy, pr, NOW, LATER)
        self.assertEqual(result['state'], 'waiting-build')
        api = self.publish(pr, result)
        api.upsert_state.assert_not_called()
        api = Mock()
        api.open_prs.return_value = [dict(pr, controller_state=None)]
        api.histories.side_effect = lambda numbers: {n: {'comments': [], 'lifecycle_at': None} for n in numbers}
        api.latest_state_from_status.return_value = 'waiting-build'
        with patch.object(main, 'load_histories', side_effect=lambda a, s: [
                dict(p, controller_state=None, controller_comment_id=None, comments=[], lifecycle_at=None) for p in s]):
            main.collect(api, self.policy, waiting_on_build=True)
        self.assertEqual([c.args[0] for c in api.snapshot.call_args_list], [pr['number']])
        api.latest_state_from_status.assert_called_once_with(pr['head'])

    def test_the_holder_is_the_comment_the_record_was_read_from(self):
        # Two record comments; the older one was written last. The record was
        # read from it, and it is the one edited and kept.
        result = evaluate(self.policy, self.pr, NOW, LATER)
        record = main.state_record(self.pr, result, 'c' * 64)
        older_written_last = dict(id=1, user='github-actions[bot]', created_at='2026-09-10T00:00:00Z', updated_at='2026-09-11T11:00:00Z',
                                  body=GitHub.state_comment_body(record, 'old text'))
        newer = dict(id=2, user='github-actions[bot]', created_at='2026-09-10T12:00:00Z', updated_at='2026-09-10T12:00:00Z',
                     body=GitHub.state_comment_body(dict(record, ready_since='2026-09-10T12:00:00Z'), 'other text'))
        parsed, holder_id = main.parse_controller_state([newer, older_written_last])
        self.assertEqual(holder_id, 1)
        quiet = dict(self.pr, comments=[older_written_last, newer], controller_state=parsed, controller_comment_id=holder_id)
        api = self.publish(quiet, evaluate(self.policy, quiet, NOW, LATER))
        self.assertEqual(api.upsert_state.call_args.args[3], 1)
        self.assertEqual([c.args[0] for c in api.delete_comment.call_args_list], [2])

    def test_a_foreign_bot_comment_quoting_the_marker_is_left_alone(self):
        result = evaluate(self.policy, self.pr, NOW, LATER)
        record = main.state_record(self.pr, result, 'c' * 64)
        foreign = dict(id=9, user='github-actions[bot]', created_at=NOW, updated_at=NOW,
                       body=GitHub.state_comment_body(dict(record, number=999), 'another workflow, another pull request'))
        pr = dict(self.pr, comments=self.pr['comments'] + [foreign])
        api = self.publish(pr, evaluate(self.policy, pr, NOW, LATER))
        api.delete_comment.assert_not_called()
        self.assertIsNone(api.upsert_state.call_args.args[3], 'a new comment; the foreign one is not edited')

    def test_a_move_cycle_keeps_the_record_under_the_matching_words(self):
        # A → B → A on one head: the record goes back to A's announcement,
        # not to B's, whose words would then say the wrong thing.
        result = evaluate(self.policy, self.pr, NOW, LATER)
        a_record = main.state_record(self.pr, result, 'c' * 64)
        a = dict(id=50, user='github-actions[bot]', created_at='2026-09-11T10:00:00Z', updated_at='2026-09-11T10:00:00Z',
                 body=GitHub.state_comment_body(a_record, main.move_text(result)))
        b_text = f'{MOVE_MARKER} state=ready-for-human sha={HEAD} -->\nReady for review — needs QuantumExplorer or shumkov.\nFull checklist in the description.'
        b = dict(id=51, user='github-actions[bot]', created_at='2026-09-11T11:00:00Z', updated_at='2026-09-11T11:00:00Z',
                 body=GitHub.state_comment_body(dict(a_record, state='ready-for-human'), b_text))
        pr = dict(self.pr, comments=self.pr['comments'] + [a, b], body='text\n\n' + main.checklist_block(result),
                  labels=['waiting-self-review', 'bot-review-skipped'])
        pr['controller_state'], pr['controller_comment_id'] = main.parse_controller_state(pr['comments'])
        self.assertEqual(pr['controller_comment_id'], 51)
        again = evaluate(self.policy, pr, NOW, LATER)
        self.assertEqual(again['state'], 'waiting-self-review')
        api = self.publish(pr, again)
        api.upsert_state.assert_called_once()
        record, text, comment_id = api.upsert_state.call_args.args[1:]
        self.assertEqual(comment_id, 50, "A's own announcement carries A's record")
        self.assertIn('state=waiting-self-review', text)

    def test_a_pull_request_back_in_draft_loses_the_stale_block(self):
        result = evaluate(self.policy, self.pr, NOW, LATER)
        pr = dict(self.pr, draft=True, body='text\n\n' + main.checklist_block(result))
        api = self.publish(pr, evaluate(self.policy, pr, NOW, LATER))
        api.remove_checklist.assert_called_once_with(pr['number'])
        api.set_checklist.assert_not_called()

    def test_the_status_never_depends_on_the_description_or_the_labels(self):
        # At capacity, labels refused, description refused: the check is still
        # what the evidence says.
        pr = dict(self.pr)
        pr['comments'] = pr['comments'] + [dict(id=2, user='llbartekll', body='/self-reviewed', created_at='2026-09-11T11:00:00Z', updated_at='2026-09-11T11:00:00Z')]
        pr['reviews'] = pr['reviews'] + [dict(id=6, user='shumkov', state='APPROVED', commit_id=HEAD, submitted_at='2026-09-11T11:30:00Z', body='')]
        pr['body'] = 'x' * 65500
        pr['labels'] = ['waiting-bots', 'ready-to-merge', 'nonsense']
        # The admission is already on record, as it would be once a human was involved.
        pr['controller_state'] = dict(admitted_at=NOW, head=HEAD, state='waiting-bots', ready_since=None)
        result = evaluate(self.policy, pr, NOW, LATER)
        self.assertEqual(result['state'], 'ready-to-merge')
        api = Mock()
        api.set_state_label.side_effect = main.GitHubError('no')
        api.set_label.side_effect = main.GitHubError('no')
        api.set_checklist.side_effect = main.GitHubError('no')
        with patch('sys.stderr'):
            api = self.publish(pr, result, api=api)
        self.assertEqual(api.post_status.call_args.args[1:], ('success', 'ready-to-merge'))


class MarkerParsingTests(unittest.TestCase):
    def test_an_extra_end_marker_after_the_block_is_the_authors_and_stays(self):
        real = f"{CHECKLIST_START}\nreal\n{CHECKLIST_END}"
        body = f"author\n\n{real}\nKEEP ME\n{CHECKLIST_END}"
        api = GitHub('dashpay/platform')
        writes = []
        with patch.object(api, 'request', side_effect=lambda m, p, payload=None: {'body': body} if m == 'GET' else writes.append(payload)):
            api.set_checklist(1, f"{CHECKLIST_START}\nfresh\n{CHECKLIST_END}")
        self.assertIn('KEEP ME', writes[0]['body'])
        self.assertEqual(writes[0]['body'].count('fresh'), 1)

    def test_a_fenced_example_below_the_block_is_not_the_block(self):
        real = f"{CHECKLIST_START}\nreal\n{CHECKLIST_END}"
        fenced = f"```\n{CHECKLIST_START}\nexample\n{CHECKLIST_END}\n```"
        self.assertEqual(current_checklist(f"{real}\n\n{fenced}"), real)
        self.assertIsNone(current_checklist(fenced), 'a fenced example alone is no block: the first write appends')
        self.assertIsNone(current_checklist(f"{CHECKLIST_END}\nreversed\n{CHECKLIST_START}"))
