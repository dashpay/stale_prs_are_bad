import copy
import unittest

from pr_review.policy import admit, codeowners, evaluate, fingerprint, validate_policy


HEAD = 'a' * 40
NOW = '2026-09-11T12:00:00Z'


def rules(text):
    """CODEOWNERS lines that GitHub would act on."""
    return [line for line in text.splitlines() if line.strip() and not line.startswith('#')]


def fixture():
    policy = dict(version=1, repository='dashpay/platform', max_active_prs=5,
                  target_branches=['v4.2-dev'], fallback={'owners': ['fallback'], 'reviewers': []},
                  areas=[dict(id='drive', paths=['packages/drive/'], owners=['owner'], reviewers=['reviewer'])])
    pr = dict(number=1, author='owner', head=HEAD, base='v4.2-dev', base_sha='b'*40,
              created_at='2026-09-10T00:00:00Z', draft=False, state='open', url='url', title='title',
              files=[{'filename': 'packages/drive/a.rs'}],
              reviews=[dict(id=1,user='thepastaclaw',state='COMMENTED',commit_id=HEAD,
                            submitted_at='2026-09-11T10:00:00Z',
                            body=f'<!-- thepastaclaw-review-phase v1 phase=final sha={HEAD} -->'),
                       dict(id=2,user='coderabbitai[bot]',state='APPROVED',commit_id=HEAD,
                            submitted_at='2026-09-11T10:00:00Z',body='')],
              comments=[dict(id=3,user='owner',body=f'/self-reviewed {HEAD}',
                             created_at='2026-09-11T11:00:00Z',updated_at='2026-09-11T11:00:00Z')],
              threads=[], requested_reviewers=[], permissions={'owner':'write','reviewer':'write','fallback':'write'},
              controller_state=None, labels=[], complete=True, build='green')
    return policy, pr


class PolicyTests(unittest.TestCase):
    def test_owner_exemption_does_not_apply_to_reviewer(self):
        p, pr = fixture()
        self.assertEqual(evaluate(p, pr, NOW, NOW)['status'], 'success')
        pr['author'] = pr['comments'][0]['user'] = 'reviewer'
        result = evaluate(p, pr, NOW, NOW)
        self.assertEqual(result['state'], 'ready-for-human')
        self.assertEqual(result['reviewers'], ['owner'])

    def ready(self):
        """A pull request that reaches ready-for-human on the current head."""
        p, pr = fixture()
        pr['author'] = pr['comments'][0]['user'] = 'reviewer'
        return p, pr

    def test_a_human_is_not_asked_until_the_build_is_green(self):
        p, pr = self.ready()
        pr['build'] = 'failed'
        result = evaluate(p, pr, NOW, NOW)
        self.assertEqual(result['state'], 'waiting-build')
        self.assertEqual(result['blockers'], ['The build must pass before a human is asked'])
        self.assertEqual(result['reviewers'], [], 'nobody is requested for a red pull request')
        pr['build'] = 'running'
        self.assertEqual(evaluate(p, pr, NOW, NOW)['blockers'], ['Waiting for the build to finish'])
        pr['build'] = 'green'
        self.assertEqual(evaluate(p, pr, NOW, NOW)['state'], 'ready-for-human')

    def test_a_build_going_red_afterwards_does_not_take_the_review_back(self):
        # The owner's rule: latch on green, never retract. A flake must not
        # withdraw a review request already sent or drop its author's slot.
        p, pr = self.ready()
        pr['build'] = 'failed'
        pr['controller_state'] = dict(admitted_at=NOW, head=pr['head'], state='ready-for-human', ready_since=NOW)
        result = evaluate(p, pr, NOW, NOW)
        self.assertEqual(result['state'], 'ready-for-human')
        self.assertEqual(result['ready_since'], NOW, 'the clock keeps running from when it first went green')

    def test_a_pull_request_ready_without_a_recorded_clock_still_latches(self):
        # The run that first makes a pull request ready records no ready_since,
        # so a latch keyed on that value would send exactly those back the
        # moment their build went red — the newest ready pull requests, the
        # ones most likely to have a build still settling.
        p, pr = self.ready()
        pr['build'] = 'failed'
        pr['controller_state'] = dict(admitted_at=NOW, head=pr['head'],
                                      state='ready-for-human', ready_since=None)
        self.assertEqual(evaluate(p, pr, NOW, NOW)['state'], 'ready-for-human')

    def test_the_latch_is_for_this_head_only(self):
        # A new push restarts the bots and the attestation; it restarts this too.
        p, pr = self.ready()
        pr['build'] = 'failed'
        pr['controller_state'] = dict(admitted_at=NOW, head='b' * 40, state='ready-for-human', ready_since=NOW)
        self.assertEqual(evaluate(p, pr, NOW, NOW)['state'], 'waiting-build')

    def test_the_first_run_that_makes_it_ready_is_not_sent_back(self):
        # There is no ready_since yet on that run, so a latch keyed on one would
        # bounce a pull request that had just been admitted.
        p, pr = self.ready()
        pr['build'] = 'green'
        pr['controller_state'] = None
        self.assertEqual(evaluate(p, pr, NOW, NOW)['state'], 'ready-for-human')

    def test_an_objection_is_not_dropped_because_access_could_not_be_read(self):
        # Everywhere else an unreadable permission holds a pull request back.
        # Dropping an objection on the strength of it was the one place the
        # same uncertainty let one through, invisibly to whoever raised it.
        p, pr = fixture()
        pr['author'] = pr['comments'][0]['user'] = 'reviewer'
        pr['threads'] = [dict(id=9, author='outsider', is_resolved=False,
                              created_at=NOW, body='this is wrong')]
        pr['permissions'] = dict(pr['permissions'], outsider=None)
        self.assertEqual(evaluate(p, pr, NOW, NOW)['state'], 'waiting-author')
        pr['permissions'] = dict(pr['permissions'], outsider='read')
        self.assertNotEqual(evaluate(p, pr, NOW, NOW)['state'], 'waiting-author')

    def test_an_unverifiable_reviewer_is_reported_as_unverifiable(self):
        # "lacks verified write access" said something false about an
        # administrator whose access this token simply could not enumerate.
        p, pr = fixture()
        pr['permissions'] = dict(pr['permissions'], owner=None)
        result = evaluate(p, pr, NOW, NOW)
        self.assertEqual(result['state'], 'configuration-error')
        self.assertEqual(result['blockers'], ['Cannot verify write access for owner'])

    def test_the_owner_path_also_needs_a_green_build(self):
        # An owner's pull request asks no human, so nothing else ever looks at
        # the build. Before this check was the gate an approver would; now the
        # check is the only thing standing between a red build and a merge.
        p, pr = fixture()
        self.assertEqual(evaluate(p, pr, NOW, NOW)['state'], 'ready-to-merge')
        for build, state in [('failed', 'waiting-build'), ('running', 'waiting-build'), ('green', 'ready-to-merge')]:
            pr['build'] = build
            result = evaluate(p, pr, NOW, NOW)
            self.assertEqual(result['state'], state, build)
            self.assertEqual(result['status'], 'success' if state == 'ready-to-merge' else 'pending', build)

    def test_a_bot_author_is_not_asked_to_attest(self):
        # Copilot and dependabot cannot post /self-reviewed, so a pull request
        # of theirs would wait for an attestation for ever. They never own an
        # area, so the approval they need anyway stands in for it.
        p, pr = fixture()
        pr['author'] = 'Copilot'
        pr['author_is_bot'] = True
        pr['comments'] = []
        result = evaluate(p, pr, NOW, NOW)
        self.assertEqual(result['state'], 'ready-for-human')
        self.assertTrue(result['reviewers'], 'a human is asked, not an attestation')
        pr['reviews'].append(dict(id=4, user='owner', state='APPROVED', commit_id=HEAD, submitted_at=NOW, body=''))
        self.assertEqual(evaluate(p, pr, NOW, NOW)['state'], 'ready-to-merge')
        pr['author_is_bot'] = False
        self.assertEqual(evaluate(p, pr, NOW, NOW)['state'], 'waiting-self-review',
                         'a human author with no attestation still has to give one')

    def test_a_reviewers_objection_inside_the_authors_thread_still_counts(self):
        # The author opens a thread; a reviewer replies objecting. Reading
        # only who opened it dropped the objection — and as the gate, merged.
        p, pr = fixture()
        pr['threads'] = [dict(id=9, author='owner', is_resolved=False, created_at='2026-09-11T11:00:00Z',
                              voices=[dict(user='owner', created_at='2026-09-11T11:00:00Z'),
                                      dict(user='reviewer', created_at='2026-09-11T12:00:00Z')])]
        self.assertEqual(evaluate(p, pr, NOW, NOW)['state'], 'waiting-author')

    def test_an_authors_own_thread_is_not_an_objection(self):
        # It made the author an objector to their own pull request, which then
        # asked for a human with nobody to name — and, as the gate, would have
        # held the pull request with no one able to release it.
        p, pr = fixture()
        pr['threads'] = [dict(id=9, author='owner', is_resolved=False, created_at=NOW, body='note to self')]
        result = evaluate(p, pr, NOW, NOW)
        self.assertEqual(result['state'], 'ready-to-merge')

    def test_missing_build_evidence_is_not_a_pass(self):
        # A repository with no CI reads green from the snapshot, which is what
        # keeps it moving; evidence that never arrived is a different thing and
        # a gate that treats the unknown as a pass is not a gate.
        p, pr = self.ready()
        pr.pop('build', None)
        result = evaluate(p, pr, NOW, NOW)
        self.assertEqual(result['state'], 'configuration-error')
        self.assertEqual(result['status'], 'error')

    def test_a_pull_request_already_asked_of_a_human_is_not_asked_again_for_green(self):
        # The recorded state is rewritten every run, so a pull request that
        # passes through any other state loses it — and one unresolved bot
        # thread does that. The published status cannot be edited or deleted,
        # so it records that a human was asked and nothing takes it back.
        p, pr = self.ready()
        pr['build'] = 'failed'
        pr['controller_state'] = dict(admitted_at=NOW, head=pr['head'],
                                      state='waiting-bots', ready_since=None)
        self.assertEqual(evaluate(p, pr, NOW, NOW)['state'], 'waiting-build')
        pr['ready_published'] = True
        self.assertEqual(evaluate(p, pr, NOW, NOW)['state'], 'ready-for-human')

    def test_the_build_is_not_evidence_that_can_change_under_a_write(self):
        # publish re-reads and bails when the fingerprint moves. On a pull
        # request with 45 checks one finishing mid-run would strand it.
        p, pr = self.ready()
        pr['build'] = 'green'
        original = fingerprint(pr)
        pr['build'] = 'failed'
        self.assertEqual(original, fingerprint(pr))

    def test_rename_requires_source_and_destination_coverage(self):
        p, pr = fixture()
        pr['files'] = [{'filename':'packages/drive/a.rs', 'previous_filename':'root.rs'}]
        self.assertEqual(evaluate(p,pr,NOW,NOW)['reviewers'], ['fallback'])

    def test_repository_without_coderabbit_waits_only_for_the_bots_it_runs(self):
        p, pr = fixture()
        pr['reviews'] = [r for r in pr['reviews'] if r['user'] != 'coderabbitai[bot]']
        self.assertEqual(evaluate(p, pr, NOW, NOW)['state'], 'waiting-bots')
        p['required_bots'] = ['thepastaclaw']
        self.assertEqual(evaluate(p, pr, NOW, NOW)['status'], 'success')

    def test_unrequired_bot_still_blocks_while_it_objects(self):
        p, pr = fixture()
        p['required_bots'] = ['thepastaclaw']
        pr['reviews'] = [r for r in pr['reviews'] if r['user'] != 'coderabbitai[bot]']
        pr['threads'] = [dict(id='t1', is_resolved=False, author='coderabbitai[bot]', created_at=NOW)]
        result = evaluate(p, pr, NOW, NOW)
        self.assertEqual(result['state'], 'waiting-bots')
        self.assertIn('coderabbitai left review threads unresolved; resolve them', result['blockers'])

    def test_required_bots_must_name_known_producers(self):
        p, _ = fixture()
        for value in [['thepastaclaw', 'thepastaclaw'], ['dependabot'], 'thepastaclaw', [None]]:
            with self.subTest(value=value), self.assertRaises(ValueError):
                validate_policy(dict(p, required_bots=value))
        validate_policy(dict(p, required_bots=[]))

    def test_only_a_satisfied_policy_passes_the_check(self):
        # This status is a required check, so it can pass only when the policy
        # is satisfied. Anything still waiting is pending — not red, because
        # every open pull request would be red most of its life — and only a
        # configuration problem someone must fix is an error.
        p, pr = fixture()
        self.assertEqual(evaluate(p, pr, NOW, NOW)['status'], 'success', 'the owner path, satisfied')
        for shape in [dict(reviews=[], author='reviewer'), dict(draft=True), dict(build='running')]:
            q, waiting = fixture()
            waiting.update(shape)
            if 'author' in shape:
                waiting['comments'][0]['user'] = shape['author']
            result = evaluate(q, waiting, NOW, NOW)
            self.assertEqual(result['status'], 'pending', shape)
            self.assertNotEqual(result['state'], 'ready-to-merge', shape)
        p['areas'][0]['unresolved'] = ['Owner: unknown']
        result = evaluate(p, pr, NOW, NOW)
        self.assertEqual((result['state'], result['status']), ('configuration-error', 'error'))


    def test_a_bot_cannot_be_named_as_an_owner(self):
        # A bot owning an area would satisfy the owner path with no human at
        # all: the bots report, the bot author is exempt from attesting, and
        # the check passes. validate_policy is where that has to be refused.
        for bot in ('Copilot', 'coderabbitai', 'thepastaclaw'):
            p, _ = fixture()
            p['areas'][0]['owners'] = [bot]
            with self.assertRaises(ValueError, msg=bot):
                validate_policy(p)

    def test_the_check_passes_in_exactly_one_state(self):
        # The invariant the gate rests on. Every reachable state is driven
        # here so that changing any one of them to success — one line — turns
        # this red, and the test fails if a state stops being reachable.
        def shape(**changes):
            p, pr = fixture()
            admitted = changes.pop('admitted', NOW)
            if 'author' in changes:
                pr['comments'][0]['user'] = changes['author']
            pr.update(changes)
            return evaluate(p, pr, admitted, NOW)
        objection = dict(id=9, author='reviewer', is_resolved=False, created_at='2026-09-11T12:00:00Z', body='no')
        seen = {
            'draft': shape(draft=True),
            'waiting-slot': shape(author='reviewer', admitted=None),
            'waiting-bots': shape(reviews=[]),
            'waiting-build': shape(build='running'),
            'waiting-self-review': shape(comments=[]),
            'waiting-author': shape(threads=[objection]),
            'ready-for-human': shape(author='reviewer'),
            'ready-to-merge': shape(),
            'configuration-error': shape(complete=False),
        }
        for expected, result in seen.items():
            self.assertEqual(result['state'], expected, 'the shape must reach the state it names')
            self.assertEqual(result['status'],
                             {'ready-to-merge': 'success', 'configuration-error': 'error'}.get(expected, 'pending'),
                             expected)
        self.assertEqual(set(seen), {'draft', 'waiting-slot', 'waiting-bots', 'waiting-build', 'waiting-self-review',
                                     'waiting-author', 'ready-for-human', 'ready-to-merge', 'configuration-error'})

    def test_bare_self_review_covers_everything_pushed_so_far(self):
        p, pr = fixture()
        pr['head_seen_at'] = '2026-09-11T09:00:00Z'
        pr['comments'][0]['body'] = '/self-reviewed'
        self.assertEqual(evaluate(p, pr, NOW, NOW)['status'], 'success')

    def test_bare_self_review_written_before_the_push_does_not_count(self):
        p, pr = fixture()
        pr['head_seen_at'] = '2026-09-11T12:00:00Z'
        pr['comments'][0]['body'] = '/self-reviewed'
        result = evaluate(p, pr, NOW, NOW)
        self.assertEqual(result['state'], 'waiting-self-review')

    def test_bare_self_review_before_the_bots_finish_does_not_count(self):
        p, pr = fixture()
        pr['head_seen_at'] = '2026-09-11T09:00:00Z'
        pr['comments'][0].update(body='/self-reviewed', created_at='2026-09-11T09:30:00Z',
                                 updated_at='2026-09-11T09:30:00Z')
        self.assertEqual(evaluate(p, pr, NOW, NOW)['state'], 'waiting-self-review')

    def test_an_attestation_sharing_a_second_with_its_floor_does_not_count(self):
        p, pr = fixture()
        pr['head_seen_at'] = '2026-09-11T09:00:00Z'
        floor = max(r['submitted_at'] for r in pr['reviews'])
        pr['comments'][0].update(body='/self-reviewed', created_at=floor, updated_at=floor)
        self.assertEqual(evaluate(p, pr, NOW, NOW)['state'], 'waiting-self-review')

    def test_bare_self_review_needs_a_head_the_controller_has_seen(self):
        p, pr = fixture()
        pr['head_seen_at'] = None
        pr['comments'][0]['body'] = '/self-reviewed'
        self.assertEqual(evaluate(p, pr, NOW, NOW)['state'], 'waiting-self-review')

    def test_self_review_still_names_a_commit_and_rejects_other_text(self):
        p, pr = fixture()
        for body, expected in [('/self-reviewed ' + pr['head'], 'success'),
                               ('/self-reviewed ' + 'f' * 40, 'waiting-self-review'),
                               ('please /self-reviewed', 'waiting-self-review'),
                               ('/self-reviewed now', 'waiting-self-review')]:
            with self.subTest(body=body):
                pr['comments'][0]['body'] = body
                result = evaluate(p, pr, NOW, NOW)
                self.assertEqual(result.get('status') if expected == 'success' else result['state'], expected)

    def test_bot_failure_blocks_even_owner(self):
        p, pr = fixture()
        for update in [{'state':'CHANGES_REQUESTED'}, {'commit_id':'c'*40}, {'body':'preliminary'}]:
            changed = copy.deepcopy(pr)
            changed['reviews'][0].update(update)
            self.assertEqual(evaluate(p,changed,NOW,NOW)['state'], 'waiting-bots')

    def test_self_review_requires_unedited_author_confirmation_after_bots(self):
        p, pr = fixture()
        for update in [{'user':'reviewer'}, {'body':'/self-reviewed old'},
                       {'updated_at':NOW}, {'created_at':'2026-09-11T09:00:00Z','updated_at':'2026-09-11T09:00:00Z'}]:
            changed = copy.deepcopy(pr)
            changed['comments'][0].update(update)
            self.assertEqual(evaluate(p,changed,NOW,NOW)['state'], 'waiting-self-review')

    def test_fixed_owner_pr_returns_to_objecting_reviewers_queue(self):
        p, pr = fixture()
        pr['reviews'].append(dict(id=4,user='reviewer',state='CHANGES_REQUESTED',commit_id='c'*40,
                                 submitted_at='2026-09-11T10:30:00Z',body=''))
        self.assertEqual(evaluate(p,pr,NOW,NOW)['reviewers'], ['reviewer'])
        pr['reviews'][-1]['submitted_at'] = '2026-09-11T11:30:00Z'
        self.assertEqual(evaluate(p,pr,NOW,NOW)['state'], 'waiting-author')

    def test_dismissal_and_stale_approval_do_not_satisfy_human_area(self):
        p, pr = fixture()
        pr['author'] = pr['comments'][0]['user'] = 'reviewer'
        review = dict(id=4,user='owner',state='APPROVED',commit_id=HEAD,submitted_at=NOW,body='')
        pr['reviews'].append(review)
        self.assertEqual(evaluate(p,pr,NOW,NOW)['state'], 'ready-to-merge')
        review['state'] = 'DISMISSED'
        self.assertEqual(evaluate(p,pr,NOW,NOW)['state'], 'ready-for-human')
        review.update(state='APPROVED',commit_id='c'*40)
        self.assertEqual(evaluate(p,pr,NOW,NOW)['state'], 'ready-for-human')

    def test_sixth_waits_without_blocking_admitted_five(self):
        p, pr = fixture()
        prs = [dict(copy.deepcopy(pr),number=n) for n in range(1,7)]
        prs[-1]['controller_state'] = {'admitted_at':'2026-09-10T01:00:00Z'}
        slots = admit(p,prs,NOW)
        self.assertEqual(set(slots), {1,2,3,4,6})
        # The five slots limit human attention. The sixth waits for one only
        # when it needs a human; the fixture's author owns what it touches, so
        # it needs none and merges without taking a slot from the others.
        self.assertEqual(evaluate(p,prs[4],slots.get(5),NOW)['state'], 'ready-to-merge')
        prs[4]['author'] = prs[4]['comments'][0]['user'] = 'reviewer'
        sixth = evaluate(p,prs[4],slots.get(5),NOW)
        self.assertEqual(sixth['state'], 'waiting-slot')
        self.assertEqual(sixth['reviewers'], [], 'nobody is asked while it waits')

    def test_incomplete_or_unresolved_never_succeeds(self):
        p, pr = fixture()
        pr['complete'] = False
        self.assertEqual(evaluate(p,pr,NOW,NOW)['status'], 'error')
        pr['complete'] = True
        p['areas'][0]['unresolved'] = ['Daniel']
        self.assertEqual(evaluate(p,pr,NOW,NOW)['state'], 'configuration-error')

    def test_integer_configuration_does_not_accept_float_lookalikes(self):
        for key,value in [('version',1.0),('max_active_prs',5.0)]:
            policy,_ = fixture()
            policy[key] = value
            with self.subTest(key=key), self.assertRaises(ValueError):
                validate_policy(policy)

    def test_validation_rejects_overlap_and_excluded_identity(self):
        p, _ = fixture()
        validate_policy(p)
        p['areas'][0]['paths'].append('packages/drive/nested/')
        with self.assertRaises(ValueError): validate_policy(p)
        p, _ = fixture()
        p['areas'][0]['reviewers'] = ['strophy']
        with self.assertRaises(ValueError): validate_policy(p)

    def test_fingerprint_ignores_controller_effects_but_not_evidence(self):
        _, pr = fixture()
        original = fingerprint(pr)
        pr['labels'] = ['ready-for-human']
        pr['requested_reviewers'] = ['reviewer']
        pr['comments'].append(dict(user='github-actions[bot]',body='<!-- platform-pr-review-state-v1 -->'))
        self.assertEqual(original,fingerprint(pr))
        pr['reviews'][0]['state'] = 'CHANGES_REQUESTED'
        self.assertNotEqual(original,fingerprint(pr))

    def test_edited_bot_summary_requires_fresh_author_confirmation(self):
        p, pr = fixture()
        pr['reviews'] = pr['reviews'][:1]
        pr['comments'].append(dict(id=8,user='coderabbitai[bot]',created_at='2026-09-11T10:00:00Z',
                                  updated_at='2026-09-11T10:00:00Z',
                                  body='<!-- final_review_risk_coverage:{"kind":"reviewed","coveredCommitId":"'+HEAD+'"} -->'))
        self.assertEqual(evaluate(p,pr,NOW,NOW)['status'], 'success')
        pr['comments'][-1]['updated_at'] = NOW
        self.assertEqual(evaluate(p,pr,NOW,NOW)['state'], 'waiting-self-review')
        pr['comments'][-1]['body'] = 'Skipped review'
        self.assertEqual(evaluate(p,pr,NOW,NOW)['state'], 'waiting-bots')

    def test_unresolved_bot_and_human_threads_have_different_queue_effects(self):
        p, pr = fixture()
        pr['threads'] = [dict(id='thread',is_resolved=False,author='reviewer',created_at='2026-09-11T10:00:00Z')]
        self.assertEqual(evaluate(p,pr,NOW,NOW)['state'], 'ready-for-human')
        pr['threads'][0]['author'] = 'thepastaclaw'
        self.assertEqual(evaluate(p,pr,NOW,NOW)['state'], 'waiting-bots')

    def test_non_writer_thread_cannot_block_owner_pr(self):
        for permission in ['read', 'triage', 'none']:
            for created in ['2026-09-11T10:00:00Z', NOW]:
                with self.subTest(permission=permission, created=created):
                    p, pr = fixture()
                    pr['permissions']['outsider'] = permission
                    pr['threads'] = [dict(id='thread', is_resolved=False,
                                          author='outsider', created_at=created)]
                    self.assertEqual(evaluate(p, pr, NOW, NOW)['state'], 'ready-to-merge')

    def test_comment_review_does_not_erase_decisive_approval(self):
        p, pr = fixture()
        pr['author'] = pr['comments'][0]['user'] = 'reviewer'
        pr['reviews'].extend([dict(id=8,user='owner',state='APPROVED',commit_id=HEAD,submitted_at=NOW,body=''),
                              dict(id=9,user='owner',state='COMMENTED',commit_id=HEAD,submitted_at=NOW,body='')])
        self.assertEqual(evaluate(p,pr,NOW,NOW)['status'], 'success')

    def test_roster_permission_loss_blocks_owner_exemption(self):
        p, pr = fixture()
        pr['permissions']['reviewer'] = 'read'
        self.assertEqual(evaluate(p,pr,NOW,NOW)['state'], 'configuration-error')

    def test_ready_age_only_carries_on_same_head(self):
        p, pr = fixture()
        pr['author'] = pr['comments'][0]['user'] = 'reviewer'
        previous = '2026-09-11T11:30:00Z'
        pr['controller_state'] = dict(admitted_at=previous,head=HEAD,state='ready-for-human',ready_since=previous)
        self.assertEqual(evaluate(p,pr,NOW,NOW)['ready_since'], previous)
        pr['controller_state']['head'] = 'b'*40
        self.assertEqual(evaluate(p,pr,NOW,NOW)['ready_since'], NOW)

    def test_draft_releases_sticky_slot_and_retargeted_pr_is_excluded(self):
        p, pr = fixture()
        prs = [dict(copy.deepcopy(pr),number=n) for n in range(1,7)]
        prs[0]['draft'] = True
        prs[0]['controller_state'] = {'admitted_at':NOW}
        self.assertEqual(set(admit(p,prs,NOW)), {2,3,4,5,6})
        prs[1]['base'] = 'another-branch'
        self.assertNotIn(2,admit(p,prs,NOW))

    def test_new_bot_completion_requires_author_to_review_latest_outcome(self):
        p, pr = fixture()
        pr['reviews'].append(dict(pr['reviews'][1],id=20,submitted_at=NOW))
        self.assertEqual(evaluate(p,pr,NOW,NOW)['state'], 'waiting-self-review')

    def test_controller_comment_creation_does_not_invalidate_evidence(self):
        _, pr = fixture()
        before = fingerprint(pr)
        pr['controller_comment_id'] = 55
        self.assertEqual(fingerprint(pr), before)

    def test_outside_area_writer_objection_blocks_owner_and_requests_rereview(self):
        p, pr = fixture()
        pr['permissions']['maintainer'] = 'write'
        pr['reviews'].append(dict(id=40,user='maintainer',state='CHANGES_REQUESTED',commit_id=HEAD,
                                 submitted_at='2026-09-11T10:30:00Z',body=''))
        result = evaluate(p,pr,NOW,NOW)
        self.assertEqual(result['state'], 'ready-for-human')
        self.assertEqual(result['reviewers'], ['maintainer'])

    def test_reopened_pr_does_not_retain_slot_from_previous_open_cycle(self):
        p, pr = fixture()
        prs = [dict(copy.deepcopy(pr),number=n) for n in range(1,7)]
        prs[-1]['controller_state'] = {'admitted_at':'2026-09-10T01:00:00Z'}
        prs[-1]['lifecycle_at'] = '2026-09-11T01:00:00Z'
        self.assertEqual(set(admit(p,prs,NOW)),{1,2,3,4,5})

    def test_coderabbit_quoted_marker_text_is_not_a_completion_receipt(self):
        p, pr = fixture()
        pr['reviews'] = pr['reviews'][:1]
        marker = 'final_review_risk_coverage:{"kind":"reviewed","coveredCommitId":"'+HEAD+'"}'
        for body in [marker, 'Producer example: '+marker,
                     '> <!-- '+marker+' -->', '<!-- '+marker+' trailing garbage -->']:
            changed = copy.deepcopy(pr)
            changed['comments'].append(dict(id=8,user='coderabbitai[bot]',body=body,
                                           created_at='2026-09-11T10:00:00Z',updated_at='2026-09-11T10:00:00Z'))
            with self.subTest(body=body):
                self.assertEqual(evaluate(p,changed,NOW,NOW)['state'],'waiting-bots')


    def test_whole_repository_owner_covers_root_files_and_cross_directory_rename(self):
        policy, pr = fixture()
        policy['areas'][0]['paths'] = ['']
        validate_policy(policy)
        pr['files'] = [{'filename': 'README.md', 'previous_filename': 'nested/old.md'}]
        self.assertEqual(evaluate(policy, pr, NOW, NOW)['status'], 'success')
        self.assertEqual(evaluate(policy, pr, NOW, NOW)['areas'], ['drive'])
        self.assertEqual(rules(codeowners(policy)), [])

    def test_whole_repository_prefix_cannot_overlap_other_areas(self):
        policy, _ = fixture()
        policy['areas'].append(dict(id='whole', paths=[''], owners=['whole'], reviewers=[]))
        with self.assertRaisesRegex(ValueError, 'Overlapping'):
            validate_policy(policy)

    def test_named_area_without_owner_is_explicit_configuration_gap(self):
        policy, pr = fixture()
        policy['areas'][0].update(owners=[], unresolved=['Owner missing from source sheet'])
        validate_policy(policy)
        pr['author'] = pr['comments'][0]['user'] = 'fallback'
        result = evaluate(policy, pr, NOW, NOW)
        self.assertEqual(result['status'], 'error')
        self.assertIn('Unresolved identities in drive', result['blockers'])
        self.assertEqual(rules(codeowners(policy)), [])

    def test_empty_unresolved_area_never_emits_native_owner_suppression(self):
        policy, _ = fixture()
        policy['areas'][0].update(paths=[''], owners=[], reviewers=[], unresolved=['Owner missing'])
        validate_policy(policy)
        self.assertEqual(rules(codeowners(policy)), [])

    def test_empty_owner_without_explicit_gap_is_rejected(self):
        for unresolved in [None, [], [''], 'missing']:
            policy, _ = fixture()
            policy['areas'][0]['owners'] = []
            if unresolved is not None:
                policy['areas'][0]['unresolved'] = unresolved
            with self.subTest(unresolved=unresolved), self.assertRaises(ValueError):
                validate_policy(policy)


if __name__ == '__main__':
    unittest.main()
