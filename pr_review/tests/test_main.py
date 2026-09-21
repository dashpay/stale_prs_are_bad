import contextlib
import copy
import json
import os
from pathlib import Path
import tempfile
import io
import unittest
from unittest.mock import Mock, patch

from pr_review import main
from pr_review.github import GitHubError
from pr_review.tests.test_policy import fixture, NOW


@contextlib.contextmanager
def broken_policies_root():
    with tempfile.TemporaryDirectory() as directory:
        root = Path(directory)
        (root / 'repositories.json').write_text(json.dumps({'version': 1, 'repositories': [
            {'repository': 'dashpay/platform', 'policy': 'platform.json', 'mode': 'preview'}]}))
        (root / 'platform.json').write_text('{broken json')
        yield str(root)


class PublicationTests(unittest.TestCase):
    def setUp(self):
        self.pr = {'number': 1, 'author': 'alice', 'head': 'a' * 40,
                   'base': 'v4.2-dev', 'base_sha': 'b' * 40, 'state': 'open',
                   'draft': False, 'labels': [], 'requested_reviewers': [],
                   'controller_comment_id': None, 'reviews': [], 'comments': [],'created_at':NOW,
                   'lifecycle_at': None, 'build': 'green'}
        self.result = {'number': 1, 'head': 'a' * 40, 'author': 'alice',
                       'state': 'ready-to-merge', 'status': 'success',
                       'blockers': [], 'reviewers': [], 'areas': ['core'],
                       'admitted_at': '2026-09-11T00:00:00Z', 'ready_since': None}
        self.api = Mock()
        self.api.snapshot.return_value = copy.deepcopy(self.pr)
        self.api.open_prs.return_value = [copy.deepcopy(self.pr)]
        self.api.pull.return_value = copy.deepcopy(self.pr)
        self.api.comments.return_value = []
        self.api.activity.return_value = None
        # One query answers the admission evidence for every candidate.
        self.api.histories.side_effect = lambda numbers: {
            number: {'comments': [], 'lifecycle_at': None} for number in numbers}
        self.policy, _ = fixture()
        self.pr['created_at'] = NOW

    def wrote_status_after(self, name):
        names = [call[0] for call in self.api.mock_calls]
        return 'post_status' in names[names.index(name) + 1:]

    def test_preview_never_mutates_github(self):
        main.publish(self.api, {}, self.pr, self.result, [self.pr], apply=False)
        self.assertEqual(self.api.mock_calls, [])

    def test_review_change_during_publication_cannot_publish_success(self):
        changed = copy.deepcopy(self.pr)
        changed['reviews'] = [{'id': 2, 'state': 'CHANGES_REQUESTED'}]
        self.api.snapshot.side_effect = [copy.deepcopy(self.pr), changed]
        with patch.object(main, 'fingerprint', side_effect=lambda p: repr(p['reviews'])):
            with patch.object(main, 'evaluate', return_value=self.result):
                main.publish(self.api, self.policy, self.pr, self.result, [self.pr], apply=True)
        states = [c.args[1] for c in self.api.post_status.call_args_list]
        self.assertNotIn('success', states)

    def test_new_head_aborts_before_any_mutation(self):
        self.api.pull.return_value['head'] = 'c' * 40
        main.publish(self.api, {}, self.pr, self.result, [self.pr], apply=True)
        self.api.post_status.assert_not_called()
        self.api.upsert_state.assert_not_called()

    def test_user_report_includes_pending_author_work_and_review_requests(self):
        rows = [dict(self.result, state='waiting-bots', title='Owned PR', url='u'),
                dict(self.result, number=2, author='bob', state='ready-for-human', reviewers=['alice'], title='Review PR', url='v')]
        text = main.render_report(rows, '2026-09-11T00:00:00Z', user='alice')
        self.assertIn('Owned PR', text)
        self.assertIn('Review PR', text)
        self.assertIn('waiting-bots', text)

    def test_foreign_controller_history_is_rejected(self):
        state = {'number':2, 'admitted_at':NOW}
        with patch.object(main, 'parse_controller_state', return_value=(state,7)):
            with self.assertRaises(main.GitHubError):
                main.collect(self.api,self.policy)

    def test_unreadable_history_revokes_every_known_head_only_in_apply(self):
        # Admission is decided from every candidate's history, so a history
        # that cannot be read leaves no pull request's slot knowable.
        self.api.histories.side_effect = main.GitHubError('missing history')
        with self.assertRaises(main.GitHubError):
            main.collect(self.api,self.policy,apply=True)
        self.assertEqual(self.api.post_status.call_args.args[1], 'error')
        self.api.post_status.reset_mock()
        with self.assertRaises(main.GitHubError):
            main.collect(self.api,self.policy)
        self.api.post_status.assert_not_called()

    def test_unreadable_evidence_marks_only_the_pull_request_it_belongs_to(self):
        # One rate-limited read used to mark everything selected an error —
        # and a full pass selects everything open. Admission is already
        # decided by then; the one pull request says so, the rest proceed.
        other = dict(self.pr, number=2, head='b' * 40)
        self.api.open_prs.return_value = [self.pr, other]
        self.api.pull.side_effect = lambda n: {1: self.pr, 2: other}[n]
        good = dict(self.pr, number=2, head='b' * 40)
        self.api.snapshot.side_effect = lambda n, policy, history=None: (
            good if n == 2 else (_ for _ in ()).throw(main.GitHubError('rate limited')))
        prs, candidates, snapshots = main.collect(self.api, self.policy, apply=True)
        self.assertEqual([s['number'] for s in snapshots], [2], 'the readable one is reconciled')
        errors = [c for c in self.api.post_status.call_args_list if c.args[1] == 'error']
        self.assertEqual([c.args[0] for c in errors], [self.pr['head']], 'only the unreadable head')

    def test_a_surplus_of_admissions_is_detected(self):
        candidates = [dict(self.pr,number=n,controller_state={'admitted_at':NOW}) for n in range(1,7)]
        self.assertEqual(main.admission_conflicts(self.policy,candidates), {'alice'})

    def test_changed_other_pr_admission_blocks_success(self):
        other = dict(self.pr,number=2,controller_state=None)
        baseline = [self.pr,other]
        changed = [dict(self.pr,controller_state={'admitted_at':self.result['admitted_at']}),
                   dict(other,controller_state={'admitted_at':NOW})]
        self.api.open_prs.return_value = baseline
        with patch.object(main,'load_histories',side_effect=[baseline,changed]), patch.object(main,'evaluate',return_value=self.result):
            main.publish(self.api,self.policy,self.pr,self.result,baseline,apply=True,candidates=baseline)
        self.assertNotIn('success',[c.args[1] for c in self.api.post_status.call_args_list])

    def test_new_head_before_request_does_not_invite_reviewers(self):
        self.result.update(state='ready-for-human',status='pending',reviewers=['reviewer'])
        other_head = dict(self.pr,head='c'*40)
        self.api.pull.side_effect = [self.pr,self.pr,self.pr,other_head]
        with patch.object(main,'admit',return_value={1:self.result['admitted_at']}):
            main.publish(self.api,self.policy,self.pr,self.result,[self.pr],apply=True)
        self.api.request_reviewers.assert_not_called()

    def test_own_new_admission_is_expected_during_success_revalidation(self):
        baseline = [self.pr]
        after = [dict(self.pr,controller_state={'admitted_at':self.result['admitted_at']})]
        with patch.object(main,'load_histories',side_effect=[baseline,after]), patch.object(main,'evaluate',return_value=self.result):
            main.publish(self.api,self.policy,self.pr,self.result,baseline,apply=True,candidates=baseline)
        self.assertEqual(self.api.post_status.call_args.args[1], 'success')

    def test_event_collection_scopes_to_author_and_closed_trigger_releases_slots(self):
        sibling = dict(self.pr,number=2)
        outsider = dict(self.pr,number=3,author='bob')
        self.api.open_prs.return_value = [sibling,outsider]
        self.api.pull.return_value = dict(self.pr,state='closed')
        _, candidates, _ = main.collect(self.api,self.policy,number=1,reconcile_author=True)
        self.assertEqual([p['number'] for p in candidates],[2])
        self.api.snapshot.assert_called_once_with(
            2, self.policy, history={'comments': [], 'lifecycle_at': None})
        self.api.histories.assert_called_once_with([2])

    def test_pr_report_does_not_fetch_sibling_full_evidence(self):
        sibling = dict(self.pr,number=2)
        self.api.open_prs.return_value = [self.pr,sibling]
        _, candidates, _ = main.collect(self.api,self.policy,number=1)
        self.assertEqual(len(candidates),2)
        self.api.snapshot.assert_called_once_with(
            1, self.policy, history={'comments': [], 'lifecycle_at': None})

    def test_rotating_periodic_batch_covers_stable_queue(self):
        prs = [dict(self.pr,number=n) for n in range(1,69)]
        seen = set()
        for slot in range(23):
            batch = main.periodic_batch(prs,3,slot*main.SWEEP_SECONDS)
            self.assertEqual(len(batch),3)
            seen.update(p['number'] for p in batch)
        self.assertEqual(seen,set(range(1,69)))

    def test_rotation_covers_every_queue_size_at_the_real_sweep_cadence(self):
        # The cursor is derived from the clock, so it only advances by one batch
        # per sweep when it is bucketed by the interval the sweep actually runs
        # at. Bucketing by a shorter interval skips whole slices: at a queue size
        # sharing a factor with the overshoot, the same few PRs were swept for
        # ever and the rest were never repaired.
        for count in [6, 8, 12, 24, 30, 48, 57]:
            prs = [dict(self.pr, number=n) for n in range(1, count + 1)]
            seen = set()
            for sweep in range(240):
                seen.update(p['number'] for p in
                            main.periodic_batch(prs, 6, sweep * main.SWEEP_SECONDS))
            self.assertEqual(seen, set(range(1, count + 1)), f'{count} open PRs')

    def test_the_cursor_advances_once_per_sweep_at_whatever_cadence_is_given(self):
        # The cadence has to be honoured rather than merely accepted. A cursor
        # that keeps its own idea of how often the sweep runs is the defect this
        # guards against, and tests that derive their timestamps from the
        # default alone cannot see it.
        prs = [dict(self.pr, number=n) for n in range(1, 13)]
        self.assertEqual([p['number'] for p in main.periodic_batch(prs, 3, 900, cadence=900)],
                         [4, 5, 6])
        self.assertEqual([p['number'] for p in main.periodic_batch(prs, 3, 1800, cadence=900)],
                         [7, 8, 9])
        self.assertEqual(main.periodic_batch(prs, 3, 900), main.periodic_batch(prs, 3, 1800),
                         'sweeps inside one bucket of the real cadence select the same batch')
        for rejected in [0, -900, 1.5, '900', None]:
            with self.assertRaises(ValueError):
                main.periodic_batch(prs, 3, 900, cadence=rejected)

    def test_a_refused_reviewer_request_does_not_abort_the_pull_request(self):
        # The room left is read from a snapshot that another run reconciling the
        # same author can invalidate, so this POST can be refused. Setting the
        # waiver label already survives its own failure; this one aborted the
        # run and left an error status on a pull request that was otherwise fine.
        result = dict(self.result, state='ready-for-human', reviewers=['bob'])
        self.api.request_reviewers.side_effect = GitHubError('reviewer request refused')
        main.publish(self.api, self.policy, self.pr, result, [self.pr], apply=True)
        self.api.request_reviewers.assert_called_once()
        self.assertTrue(self.wrote_status_after('request_reviewers'),
                        'the run must carry on and publish a status, not stop at the refusal')

    def test_a_refused_ready_label_does_not_abort_the_pull_request(self):
        # Removing a label another run has already removed is a 404, and the
        # labels are read from a snapshot that run can invalidate. Its sibling
        # write, the waiver label, has survived its own failure from the start.
        self.api.set_state_label.side_effect = GitHubError('label does not exist')
        result = dict(self.result, state='ready-for-human')
        main.publish(self.api, self.policy, self.pr, result, [self.pr], apply=True)
        self.api.set_state_label.assert_called_once()
        self.api.set_label.assert_called_once()
        self.assertTrue(self.wrote_status_after('set_state_label'),
                        'the run must carry on and publish a status, not stop at the refusal')

    def waiting_on_build(self, *states):
        prs = [dict(self.pr, number=n + 1) for n in range(len(states))]
        self.api.open_prs.return_value = prs
        self.api.histories.side_effect = lambda numbers: {
            pr['number']: {'comments': [], 'lifecycle_at': None} for pr in prs}
        recorded = {pr['number']: state for pr, state in zip(prs, states)}
        with patch.object(main, 'parse_controller_state',
                          side_effect=lambda comments: (None, None)):
            with patch.object(main, 'load_histories', side_effect=lambda api, selected: [
                    dict(p, comments=[], lifecycle_at=None, controller_comment_id=None,
                         controller_state={'state': recorded[p['number']], 'head': p['head'],
                                           'admitted_at': NOW, 'ready_since': None}
                         if recorded[p['number']] else None) for p in selected]):
                return main.collect(self.api, self.policy, waiting_on_build=True)

    def test_the_build_scan_looks_only_at_what_is_waiting_on_a_build(self):
        self.waiting_on_build('waiting-bots', 'waiting-build', 'ready-for-human')
        self.assertEqual([call.args[0] for call in self.api.snapshot.call_args_list], [2])

    def test_the_build_scan_costs_nothing_when_nothing_is_waiting(self):
        # The point of reading the recorded state first: on a quiet repository
        # this is a listing and one batched read, and no snapshots at all.
        self.waiting_on_build('waiting-bots', 'ready-for-human', None)
        self.api.snapshot.assert_not_called()

    def test_the_build_scan_selects_its_own_pull_requests(self):
        # Combining it with a selector silently discarded that selector, and
        # asking for one pull request that was not waiting reported it as not
        # open on a configured branch, which was not true.
        for extra in [['--pr', '1'], ['--batch-size', '6']]:
            with self.assertRaises(SystemExit):
                main.run(['sync', '--repo', 'dashpay/platform', '--waiting-on-build'] + extra)

    def test_a_scan_can_carry_a_pull_request_out_of_waiting_for_a_build(self):
        # The point of the whole schedule: a build that went green with no
        # event to announce it still reaches a human.
        from pr_review.tests.test_policy import fixture
        policy, pr = fixture()
        # The default fixture's author owns the area it touches, so nobody is
        # asked; this is the shape that needs a human.
        pr['author'] = pr['comments'][0]['user'] = 'reviewer'
        pr.update(build='green', ready_published=False,
                  controller_state=dict(admitted_at=NOW, head=pr['head'],
                                        state='waiting-build', ready_since=None))
        result = main.evaluate(policy, pr, NOW, NOW)
        self.assertEqual(result['state'], 'ready-for-human', 'the scan found it green')
        self.policy = policy
        self.api.snapshot.return_value = copy.deepcopy(pr)
        self.api.pull.return_value = copy.deepcopy(pr)
        self.api.open_prs.return_value = [copy.deepcopy(pr)]
        # The admission re-read must see the same recorded state the candidate
        # carries, or publish stops at "admission context changed".
        with patch.object(main, 'load_histories', side_effect=lambda api, selected: [copy.deepcopy(pr)]):
            main.publish(self.api, self.policy, pr, result, [pr], apply=True)
        self.api.request_reviewers.assert_called_once()
        self.api.set_state_label.assert_called_once()
        self.assertEqual(self.api.set_state_label.call_args.args[1], 'ready-for-human', 'the label goes on')

    def test_the_build_scan_rotates_at_its_own_cadence(self):
        # The rotation cursor is read from the clock, so a slice bucketed by the
        # hourly sweep would advance four times per scan and skip the rest —
        # the defect that left the same six pull requests swept for ever.
        with patch.object(main, 'periodic_batch', return_value=[]) as batch:
            self.waiting_on_build('waiting-build')
        self.assertEqual(batch.call_args.args[1], main.BUILD_SCAN_SIZE)
        self.assertEqual(batch.call_args.kwargs['cadence'], main.BUILD_SCAN_SECONDS)

    def test_the_report_asks_for_one_thing_in_one_way(self):
        # The attestation is the only instruction in this comment an author has
        # to act on, and it used to be given three times over: the bare form,
        # then when to post it, then the same thing again with the commit
        # spelled out. Naming a commit still works; it is no longer advertised.
        body = main.state_body(dict(self.result, head='f' * 40))
        self.assertIn('`/self-reviewed`  — covers everything pushed so far', body)
        self.assertEqual(body.count('/self-reviewed'), 1, 'asked for once, not three ways')
        # The header names the commit the report is about; nothing asks an
        # author to copy it.
        self.assertEqual(body.count('f' * 40), 1)
        self.assertNotIn('f' * 40, body.split('Self-review')[1])

    def test_someone_elses_push_does_not_demote_a_ready_pull_request(self):
        # The admission context used to be every open pull request in the
        # repository, so a push anywhere during a run sent the one being
        # published back to pending — and on a busy repository something moves
        # every few minutes. Only the author's own pull requests decide slots.
        mine = dict(self.pr, number=1, author='me', head='a' * 40)
        theirs = dict(self.pr, number=2, author='someone', head='b' * 40)
        before = main.context_fingerprint([mine, theirs], 'me')
        theirs_pushed = dict(theirs, head='c' * 40)
        self.assertEqual(main.context_fingerprint([mine, theirs_pushed], 'me'), before)
        # Nor does the author's own push to a different pull request: a slot
        # depends on which of their pull requests are open and not drafts, not
        # on what any of them currently points at.
        mine_pushed = dict(mine, head='d' * 40)
        self.assertEqual(main.context_fingerprint([mine_pushed, theirs], 'me'), before)
        mine_drafted = dict(mine, draft=True)
        self.assertNotEqual(main.context_fingerprint([mine_drafted, theirs], 'me'), before)
        mine_closed = dict(mine, state='closed')
        self.assertNotEqual(main.context_fingerprint([mine_closed, theirs], 'me'), before)

    def test_a_surplus_of_admissions_heals_instead_of_erroring_the_author(self):
        # Two runs reconciling two pull requests of one author can both admit
        # past the check. Six persisted admissions then marked every pull
        # request of that author an error — under a required check, an outage
        # for that person. admit() keeps the five oldest; the sixth waits.
        from pr_review.tests.test_policy import fixture
        policy, base = fixture()
        prs = [dict(copy.deepcopy(base), number=n, author='busy', head=str(n) * 40,
                    controller_state=dict(admitted_at=f'2026-09-10T0{n}:00:00Z', head=str(n) * 40,
                                          state='waiting-bots', ready_since=None))
               for n in range(1, 7)]
        for pr in prs:
            pr['comments'][0]['user'] = 'busy'
        with patch.object(main, 'evaluate', side_effect=lambda p, pr, admitted, now, states=None:
                          dict(state='ready-to-merge' if admitted else 'waiting-slot', status='success' if admitted else 'pending',
                               blockers=[], reviewers=[], head=pr['head'], number=pr['number'],
                               admitted_at=admitted, ready_since=None)):
            rows = main.evaluate_snapshots(policy, prs, prs, prs, NOW)
        states = {row['number']: row['state'] for row in rows}
        self.assertNotIn('configuration-error', states.values())
        self.assertEqual(states[6], 'waiting-slot', 'the newest admission is the one that yields')
        self.assertEqual([n for n, s in states.items() if s == 'ready-to-merge'], [1, 2, 3, 4, 5])

    def test_a_policy_directory_gone_from_the_tree_does_not_fail_the_sweep(self):
        # The validate step reports it. Failing the reconcile too marked every
        # open pull request an error — as the gate, unmergeable — until a policy
        # change landed, over a directory someone legitimately removed.
        import tempfile
        from pr_review.policy import missing_paths
        policy = json.loads(Path('policies/platform.json').read_text())
        with tempfile.TemporaryDirectory() as root:
            root = Path(root)
            for area in policy['areas'][1:]:
                for prefix in area['paths']:
                    (root / prefix).mkdir(parents=True, exist_ok=True)
            gone = policy['areas'][0]['paths']
            self.assertEqual(missing_paths(policy, root), sorted(gone))
            with patch('sys.stderr', new_callable=io.StringIO) as err:
                with patch.object(main, 'collect', return_value=([], [], [])) as collect:
                    with patch.object(main, 'GitHub'):
                        main.run(['report', '--repo', 'dashpay/platform', '--repository-root', str(root)])
            self.assertIn(gone[0], err.getvalue(), 'said, not fatal')
            self.assertTrue(collect.called, 'the reconcile went ahead')
            with self.assertRaises(ValueError):
                main.run(['validate', '--repo', 'dashpay/platform', '--repository-root', str(root)])

    def test_labels_read_like_the_status(self):
        # One state label at a time, the waiver beside it, unrelated labels
        # untouched — so a pull request list says what the status says.
        for state, waived, labels, correct in [
            ('waiting-bots', [], ['waiting-bots'], True),
            ('waiting-bots', [], ['ready-for-human'], False),
            ('waiting-bots', ['thepastaclaw'], ['waiting-bots', 'bot-review-skipped', 'bug'], True),
            ('waiting-bots', ['thepastaclaw'], ['waiting-bots'], False),
            ('ready-to-merge', [], ['ready-to-merge', 'bug'], True),
            ('draft', [], ['waiting-bots'], False),
            ('draft', [], ['bug'], True),
        ]:
            result = dict(self.result, state=state, waived=waived)
            pr = dict(self.pr, labels=labels, controller_state=main.state_record(
                self.pr, result, main.context_fingerprint([self.pr], self.pr['author'])))
            self.api.reset_mock()
            self.api.snapshot.return_value = copy.deepcopy(pr)
            self.api.pull.return_value = copy.deepcopy(pr)
            main.publish(self.api, self.policy, pr, result, [pr], apply=True)
            self.assertEqual(self.api.set_state_label.called, not correct, (state, waived, labels))

    def test_the_report_offers_the_skip_only_while_bots_are_awaited(self):
        awaited = main.state_body(dict(self.result, state='waiting-bots', head='f' * 40))
        self.assertIn('/skip-bots', awaited)
        self.assertIn('the report says who did', awaited)
        later = main.state_body(dict(self.result, state='waiting-self-review', head='f' * 40))
        self.assertNotIn('/skip-bots', later)

    def test_the_report_names_who_must_approve_and_for_which_files(self):
        result = dict(self.result, state='ready-for-human', head='f' * 40, approvals=[
            {'area': 'swift-sdk', 'files': ['packages/swift-sdk/a.swift'], 'approvers': [], 'approved_by': [], 'owned': True},
            {'area': 'rust-sdk', 'files': ['packages/rs-sdk/lib.rs'], 'approvers': ['lklimek', 'shumkov'], 'approved_by': ['lklimek'], 'owned': False},
            {'area': 'fallback', 'files': ['.editorconfig', '.github/a.yml', '.github/b.yml', 'AGENTS.md', 'Cargo.lock'],
             'approvers': ['QuantumExplorer', 'shumkov'], 'approved_by': [], 'owned': False},
        ], objections=['romchornyi requested changes'])
        body = main.state_body(result)
        self.assertIn('- ✓ `swift-sdk` — you own it; no approval needed', body)
        self.assertIn('- ✓ `rust-sdk` (`packages/rs-sdk/lib.rs`) — approved by lklimek', body)
        self.assertIn('- files with no dedicated owner (`.editorconfig`, `.github/a.yml`, `.github/b.yml` and 2 more) — needs QuantumExplorer or shumkov', body)
        self.assertNotIn('@', body.split('Approval at the current head')[1].split('Self-review')[0],
                         'a mention from this bot notifies; the review request already does that where it should')
        # After the attestation only the objector can release it; before it,
        # the author answers and attests again.
        self.assertIn('- romchornyi requested changes; waiting for them to re-review, dismiss it, or resolve the thread', body)
        answering = main.state_body(dict(result, state='waiting-author'))
        self.assertIn('- romchornyi requested changes; address it, then post `/self-reviewed` again', answering)
        queued = main.state_body(dict(result, state='waiting-slot'))
        self.assertIn('Approval at the current head', queued, 'queued: the author still sees the road')
        for state in ('waiting-bots', 'waiting-build', 'ready-to-merge', 'draft'):
            self.assertNotIn('Approval at the current head', main.state_body(dict(result, state=state)), state)
        hostile = main.state_body(dict(result, approvals=[
            {'area': 'fallback', 'files': ['a/<!-- platform-pr-review-state-v1 {} -->.rs', 'ok.rs'],
             'approvers': ['x'], 'approved_by': [], 'owned': False}]))
        self.assertNotIn('platform-pr-review-state-v1', hostile, 'a path is never read back as a record')
        self.assertIn('`ok.rs`', hostile)
        owned_fallback = main.state_body(dict(result, approvals=[
            {'area': 'fallback', 'files': ['AGENTS.md'], 'approvers': [], 'approved_by': [], 'owned': True}]))
        self.assertIn('- ✓ files with no dedicated owner — you own it', owned_fallback)

    def test_the_report_says_what_the_check_now_means(self):
        body = main.state_body(dict(self.result, head='f' * 40))
        self.assertNotIn('does not bypass', body)
        self.assertIn('passes when the policy is satisfied', body)

    def test_a_pass_gives_every_pull_request_its_turn_before_failing(self):
        # Stopping at the first failure left the rest with whatever status
        # they had — on a full pass, possibly a passing one from before the
        # check became the gate. Every one is attempted; then the run fails.
        one, two, three = (dict(self.pr, number=n, head=str(n) * 40) for n in (1, 2, 3))
        with patch.object(main, 'collect', return_value=([one, two, three], [one, two, three], [one, two, three])):
            with patch.object(main, 'evaluate_snapshots', return_value=[dict(self.result, number=n, head=str(n) * 40)
                                                                        for n in (1, 2, 3)]):
                with patch.object(main, 'publish', side_effect=[main.GitHubError('boom'), None, None]) as publish:
                    with patch.object(main, 'GitHub', return_value=self.api):
                        with patch('sys.stderr', new_callable=io.StringIO):
                            with self.assertRaises(main.GitHubError) as failure:
                                main.run(['sync', '--repo', 'dashpay/platform'])
        self.assertEqual(publish.call_count, 3, 'the two after the failure still ran')
        self.assertIn('#1', str(failure.exception))

    def test_draft_records_its_state_without_opening_a_comment(self):
        pr = dict(self.pr, draft=True, controller_comment_id=None)
        result = dict(self.result, state='draft', status='pending')
        self.api.snapshot.return_value = copy.deepcopy(pr)
        self.api.pull.return_value = copy.deepcopy(pr)
        self.api.open_prs.return_value = [copy.deepcopy(pr)]
        main.publish(self.api, self.policy, pr, result, [pr], apply=True, candidates=[pr])
        self.api.upsert_state.assert_not_called()
        self.assertEqual(self.api.post_status.call_args.args[1:], ('pending', 'draft'))

    def test_draft_keeps_an_existing_comment_current(self):
        pr = dict(self.pr, draft=True, controller_comment_id=99)
        result = dict(self.result, state='draft', status='pending')
        self.api.snapshot.return_value = copy.deepcopy(pr)
        self.api.pull.return_value = copy.deepcopy(pr)
        self.api.open_prs.return_value = [copy.deepcopy(pr)]
        main.publish(self.api, self.policy, pr, result, [pr], apply=True, candidates=[pr])
        self.assertEqual(self.api.upsert_state.call_args.args[3], 99)

    def test_periodic_collection_bounds_snapshots_and_author_history(self):
        prs = [dict(self.pr,number=n,author=f'user{n}') for n in range(1,69)]
        self.api.open_prs.return_value = prs
        with patch.object(main,'periodic_batch',return_value=prs[:3]):
            _, candidates, _ = main.collect(self.api,self.policy,batch_size=3)
        self.assertEqual(len(candidates),3)
        self.assertEqual(self.api.snapshot.call_count,3)
        # Three candidates, still one query, not one request each.
        self.assertEqual(self.api.histories.call_count,1)
        self.assertEqual(sorted(self.api.histories.call_args.args[0]),[1,2,3])

    def test_invalid_configuration_revokes_previous_success_in_authorized_apply(self):
        environment = {'GITHUB_ACTIONS':'true', 'GITHUB_REPOSITORY':'dashpay/platform',
                       'PR_REVIEW_AUTOMATION_ENABLED':'true'}
        with patch.dict(os.environ,environment), patch.object(main,'GitHub',return_value=self.api), \
                broken_policies_root() as root:
            with self.assertRaises(ValueError):
                main.run(['sync','--apply','--policies-root',root])
        self.assertEqual(self.api.post_status.call_args.args[1],'error')

    def test_invalid_configuration_preview_never_revokes_status(self):
        with patch.object(main,'GitHub',return_value=self.api), broken_policies_root() as root:
            with self.assertRaises(ValueError):
                main.run(['sync','--policies-root',root])
        self.api.post_status.assert_not_called()

    def test_event_does_not_rescan_unchanged_author_pr_reviews(self):
        prs = [dict(self.pr,number=n,controller_state={'number':n,'admitted_at':NOW} if n<=5 else None)
               for n in range(1,15)]
        self.api.open_prs.return_value = prs
        with patch.object(main,'load_histories',return_value=prs):
            main.collect(self.api,self.policy,number=2,reconcile_author=True)
        self.assertEqual([call.args[0] for call in self.api.snapshot.call_args_list],[2])

    def test_close_event_refreshes_newly_admitted_waiter(self):
        prs = [dict(self.pr,number=n,controller_state={'number':n,'admitted_at':NOW} if n<=5 else None)
               for n in range(1,8) if n!=2]
        self.api.open_prs.return_value = prs
        self.api.pull.return_value = dict(self.pr,number=2,state='closed')
        with patch.object(main,'load_histories',return_value=prs):
            main.collect(self.api,self.policy,number=2,reconcile_author=True)
        self.assertEqual([call.args[0] for call in self.api.snapshot.call_args_list],[6])

    def test_noop_success_rechecks_reviews_after_admission_history_reads(self):
        self.pr['controller_state'] = main.state_record(self.pr,self.result,main.context_fingerprint([self.pr], self.pr['author']))
        changed = dict(self.pr,reviews=[{'id':9,'state':'DISMISSED'}])
        self.api.snapshot.side_effect = [self.pr,changed]
        with patch.object(main,'load_histories',return_value=[self.pr]), patch.object(main,'evaluate',return_value=self.result):
            main.publish(self.api,self.policy,self.pr,self.result,[self.pr],apply=True,candidates=[self.pr])
        self.assertNotIn('success',[c.args[1] for c in self.api.post_status.call_args_list])


if __name__ == '__main__':
    unittest.main()
