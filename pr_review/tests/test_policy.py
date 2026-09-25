import copy
import unittest

from pr_review.policy import admit, codeowners, evaluate, fingerprint, validate_policy


HEAD = 'a' * 40
NOW = '2026-09-11T12:00:00Z'


def rules(text):
    """CODEOWNERS lines that GitHub would act on."""
    return [line for line in text.splitlines() if line.strip() and not line.startswith('#')]


OLD_HEAD = 'f' * 40


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

    def test_a_finding_is_named_even_while_another_bot_is_still_out(self):
        # The state is waiting-bots — one bot has not reported — but the other
        # already left a thread open. Reporting only "waiting for X" hides
        # work the author can do now, and the checklist would show a Bots line
        # naming the thread while the blockers said nothing about it.
        p, pr = fixture()
        pr['reviews'] = [r for r in pr['reviews'] if r['user'] != 'thepastaclaw']
        pr['threads'] = [dict(author='coderabbitai[bot]', is_resolved=False)]
        result = evaluate(p, pr, NOW, NOW)
        self.assertEqual(result['state'], 'waiting-bots')
        self.assertTrue(any('coderabbitai' in b for b in result['blockers']), result['blockers'])
        self.assertTrue(any('thepastaclaw' in b for b in result['blockers']), result['blockers'])

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

    def test_an_account_the_policy_names_a_machine_is_not_asked_to_attest(self):
        # The accounts a team runs its own automation from are ordinary users
        # by every API — infraclaw-dash opened a pull request, a colleague
        # posted `/self-reviewed` on it, and it counted for nothing because
        # the attestation has to be the author's. Nobody is there to give one.
        p, pr = fixture()
        pr.update(author='infraclaw-dash', comments=[])
        self.assertEqual(evaluate(p, pr, NOW, NOW)['state'], 'waiting-self-review')
        p['bot_authors'] = ['infraclaw-dash']
        result = evaluate(p, pr, NOW, NOW)
        self.assertEqual(result['state'], 'ready-for-human')
        self.assertTrue(result['reviewers'], 'a human is asked, not an attestation')
        pr['reviews'].append(dict(id=4, user='owner', state='APPROVED', commit_id=HEAD, submitted_at=NOW, body=''))
        self.assertEqual(evaluate(p, pr, NOW, NOW)['state'], 'ready-to-merge')

    def test_a_machine_author_is_a_github_handle_like_any_other(self):
        p, _ = fixture()
        p['bot_authors'] = ['infraclaw-dash']
        validate_policy(p)
        for bad in ('not a handle', '', '-leading', 'a' * 40, 1, ['x']):
            p['bot_authors'] = [bad]
            with self.assertRaises(ValueError, msg=repr(bad)):
                validate_policy(p)

    def test_a_machine_author_that_owns_an_area_is_refused_by_the_policy(self):
        # It would need neither an attestation nor an approval: the owner
        # exemption and the stand-in together merge a pull request nobody has
        # read at all.
        p, _ = fixture()
        p['bot_authors'] = ['infraclaw-dash']
        validate_policy(p)
        for where in (p['areas'][0]['owners'], p['areas'][0]['reviewers'],
                      p['fallback']['owners'], p['fallback']['reviewers']):
            for spelling in ('infraclaw-dash', 'INFRACLAW-DASH'):
                # Handles are case-insensitive, and that is what keeps the
                # exemption safe: a set intersection without it would let the
                # owner exemption through on a capital letter.
                where.append(spelling)
                with self.assertRaises(ValueError, msg=spelling):
                    validate_policy(p)
                where.remove(spelling)

    def test_a_machine_author_cannot_approve_its_own_pull_request(self):
        # It is refused as an owner or reviewer, so it is never eligible —
        # today only as a consequence of that refusal, which is why it is
        # pinned here too.
        p, pr = fixture()
        p['bot_authors'] = ['infraclaw-dash', 'dcg-claude']
        pr.update(author='infraclaw-dash', comments=[],
                  permissions=dict(pr['permissions'], **{'infraclaw-dash': 'write', 'dcg-claude': 'write'}))
        for who in ('infraclaw-dash', 'dcg-claude'):
            reviews = pr['reviews'] + [dict(id=9, user=who, state='APPROVED', commit_id=HEAD, submitted_at=NOW, body='')]
            result = evaluate(p, dict(pr, reviews=reviews), NOW, NOW)
            self.assertEqual(result['state'], 'ready-for-human', who)
            self.assertFalse(result['approvals'] and all(a.get('approved_by') for a in result['approvals']), who)

    def test_the_attestation_is_the_phrase_however_it_is_spelled(self):
        # kwvg posted `/self-review` on rust-dashcore#1048 and nothing
        # happened. It is still the author writing it in their own comment.
        p, pr = fixture()
        pr['head_seen_at'] = '2026-09-11T10:30:00Z'
        for said in ('/self-reviewed', '/self-review', '/Self-Reviewed', '/selfreview',
                     '/self reviewed', f'/self-review {HEAD}', f'/self-reviewed {HEAD.upper()}'):
            pr['comments'] = [dict(id=3, user='owner', body=said,
                                   created_at='2026-09-11T11:00:00Z', updated_at='2026-09-11T11:00:00Z')]
            self.assertEqual(evaluate(p, pr, NOW, NOW)['status'], 'success', said)
        for not_said in ('/self-reviewedish', 'about to /self-review', '/self-reviewed later',
                         f'/self-reviewed {"b" * 40}', '/review', 'self-reviewed'):
            pr['comments'] = [dict(id=3, user='owner', body=not_said,
                                   created_at='2026-09-11T11:00:00Z', updated_at='2026-09-11T11:00:00Z')]
            self.assertEqual(evaluate(p, pr, NOW, NOW)['state'], 'waiting-self-review', not_said)

    def test_the_evidence_print_does_not_depend_on_which_route_read_a_comment(self):
        # One route carries who last edited a comment and the other cannot.
        # With that in the print, a pull request with an edited comment read
        # as changed between the read and the write on every single run: the
        # write was refused every time, and platform#4707, #4712 and #4717
        # stopped being written to at all. The time of the edit is what says
        # something changed, and it is in the print.
        _, pr = fixture()
        pr['comments'] = [dict(pr['comments'][0], updated_at='2026-09-11T11:30:00Z')]
        graphql = dict(pr, comments=[dict(pr['comments'][0], edited_by='coderabbitai')])
        rest = dict(pr, comments=[dict(pr['comments'][0], edited_by=None)])
        self.assertEqual(fingerprint(graphql), fingerprint(rest))
        later = dict(pr, comments=[dict(pr['comments'][0], updated_at='2026-09-11T12:00:00Z')])
        self.assertNotEqual(fingerprint(pr), fingerprint(later), 'an edit is still noticed')

    def carried(self, files=None, heads=None, print_of=None):
        """A pull request on a new head, with the record of the one before it."""
        from pr_review.policy import diff_print
        p, pr = fixture()
        pr['files'] = files or [{'filename': 'packages/drive/a.rs', 'status': 'modified', 'content': 'c' * 40, 'shape': 'a' * 64}]
        before = dict(pr, files=print_of if print_of is not None else pr['files'])
        pr['controller_diff'] = {'number': 1, 'diff': diff_print(before),
                                 'diff_heads': heads or [OLD_HEAD], 'diff_seen': '2026-09-11T09:00:00Z'}
        pr['head'] = HEAD
        pr['head_seen_at'] = '2026-09-11T13:00:00Z'
        return p, pr

    def test_a_push_that_leaves_the_diff_alone_keeps_the_review_it_had(self):
        # A merge of the base is not work anybody has to read again. Without
        # this, every such push threw away both bots' reports and the author's
        # attestation, and asked for all three afresh.
        p, pr = self.carried()
        for review in pr['reviews']:
            review['commit_id'] = OLD_HEAD
            review['body'] = review['body'].replace(HEAD, OLD_HEAD)
        pr['comments'] = [dict(id=3, user='owner', body=f'/self-reviewed {OLD_HEAD}',
                               created_at='2026-09-11T11:00:00Z', updated_at='2026-09-11T11:00:00Z')]
        result = evaluate(p, pr, NOW, NOW)
        self.assertEqual(result['status'], 'success', result['blockers'])
        self.assertEqual(result['reviewed_heads'], [OLD_HEAD, HEAD])

    def test_a_push_that_changes_the_diff_starts_over(self):
        # A conflict resolved inside a merge commit is code that exists there
        # and nowhere else, and nobody has read it.
        p, pr = self.carried(print_of=[{'filename': 'packages/drive/a.rs', 'status': 'modified', 'content': 'd' * 40, 'shape': 'b' * 64}])
        for review in pr['reviews']:
            review['commit_id'] = OLD_HEAD
            review['body'] = review['body'].replace(HEAD, OLD_HEAD)
        pr['comments'] = [dict(id=3, user='owner', body=f'/self-reviewed {OLD_HEAD}',
                               created_at='2026-09-11T11:00:00Z', updated_at='2026-09-11T11:00:00Z')]
        result = evaluate(p, pr, NOW, NOW)
        self.assertEqual(result['state'], 'waiting-bots')
        self.assertEqual(result['reviewed_heads'], [HEAD])

    def test_a_file_the_merge_brought_in_starts_over(self):
        # Anything else a merge carries appears as an entry that was not there.
        p, pr = self.carried(files=[{'filename': 'packages/drive/a.rs', 'status': 'modified', 'content': 'c' * 40, 'shape': 'a' * 64},
                                    {'filename': 'packages/drive/b.rs', 'status': 'added', 'content': 'e' * 40, 'shape': 'e' * 64}],
                             print_of=[{'filename': 'packages/drive/a.rs', 'status': 'modified', 'content': 'c' * 40, 'shape': 'a' * 64}])
        self.assertEqual(evaluate(p, pr, NOW, NOW)['reviewed_heads'], [HEAD])

    def test_a_read_that_cannot_say_what_changed_starts_over(self):
        # An older read carries no content ids, and says nothing about whether
        # a later commit is the same work.
        p, pr = self.carried(files=[{'filename': 'packages/drive/a.rs'}])
        self.assertEqual(evaluate(p, pr, NOW, NOW)['reviewed_heads'], [HEAD])

    def test_the_carried_commits_do_not_grow_without_bound(self):
        p, pr = self.carried(heads=['%040x' % n for n in range(30)])
        self.assertLessEqual(len(evaluate(p, pr, NOW, NOW)['reviewed_heads']), 21)

    def test_keeping_your_own_side_of_a_conflict_is_not_the_same_work(self):
        # The file is byte for byte what the reviewer saw, so its content id
        # does not move — but the patch against the base that moved now also
        # undoes what the base did, and nobody read that.
        p, pr = self.carried(
            files=[{'filename': 'packages/drive/a.rs', 'status': 'modified',
                    'content': 'c' * 40, 'shape': 'new patch against the moved base'}],
            print_of=[{'filename': 'packages/drive/a.rs', 'status': 'modified',
                       'content': 'c' * 40, 'shape': 'the patch the reviewer read'}])
        self.assertEqual(evaluate(p, pr, NOW, NOW)['reviewed_heads'], [HEAD])

    def test_a_read_that_cannot_say_how_a_file_changed_is_not_carried(self):
        # Content without a patch says what the file holds and not what it
        # took to get there, and keeping your own side of a conflict is
        # exactly the case those two disagree about.
        from pr_review.policy import diff_print
        self.assertIsNone(diff_print({'files': [{'filename': 'a.rs', 'status': 'modified', 'content': 'c' * 40}]}))

    def test_a_change_with_no_content_is_not_carried(self):
        # A mode bit or a type change shows as an entry with nothing in it,
        # and this cannot see what it did.
        from pr_review.policy import diff_print
        self.assertIsNone(diff_print({'files': [{'filename': 'a.sh', 'status': 'modified', 'shape': 'x' * 64}]}))

    def test_the_diff_is_kept_through_a_verdict_that_stops_early(self):
        # A configuration error reaches the writer like any other verdict. One
        # that forgot what carries this diff would cut the chain, and the pull
        # request would quietly start asking for its reviews again.
        p, pr = self.carried()
        p['areas'][0]['owners'] = ['nobody-at-all']
        del p['areas'][0]['paths']
        result = evaluate(p, pr, NOW, NOW)
        self.assertEqual(result['state'], 'configuration-error')
        self.assertEqual(result['reviewed_heads'], [OLD_HEAD, HEAD])

    def test_the_print_is_not_part_of_the_evidence_print(self):
        # It is this controller's own note about the evidence, not evidence.
        # In the print, it would differ between the read and the write.
        _, pr = fixture()
        self.assertEqual(fingerprint(dict(pr, controller_diff={'diff': 'a' * 64})),
                         fingerprint(dict(pr, controller_diff=None)))

    def test_a_bot_asked_about_this_work_is_not_asked_again_on_the_next_commit(self):
        # The clock the bots are waited on keeps running across a push that
        # only moved the base, so asking again because the commit id changed
        # would ask and then proceed without them minutes later.
        p, pr = self.carried()
        p['bot_timeouts'] = {'nudge_after_hours': 1, 'waive_after_hours': 48}
        pr['reviews'] = []
        pr['comments'] = [dict(id=8, user='github-actions[bot]',
                               body=f'<!-- pr-hygiene-nudge v1 bot=coderabbitai sha={OLD_HEAD} -->',
                               created_at='2026-09-11T10:00:00Z', updated_at='2026-09-11T10:00:00Z')]
        pr['controller_diff'] = dict(pr['controller_diff'], diff_seen='2026-09-11T09:00:00Z')
        self.assertNotIn('coderabbitai', evaluate(p, pr, NOW, NOW)['nudge'])
        # A commit nobody was ever asked about is asked about.
        pr['comments'] = []
        self.assertIn('coderabbitai', evaluate(p, pr, NOW, NOW)['nudge'])

    def rabbit(self, finding='', head=None, extra='', checks='✅ Passed', why='It reads well.',
               summary='Adds a field and its tests.', rows=None):
        """CodeRabbit's comment as it writes it, in the shape it really writes.

        The findings and the commit they cover are in one block; the checks are
        a table somewhere else in the same comment, with a heading that counts
        them and a column of prose it rewrites without changing a verdict.
        """
        import json as _json
        covered = _json.dumps({'sourceCommitId': head or HEAD, 'coveredCommitId': head or HEAD,
                               'kind': 'reviewed'}, separators=(',', ':'))
        return ('<!-- This is an auto-generated comment: summarize by coderabbit.ai -->\n'
                + extra
                + '<!-- final_review_risk_start -->\n'
                + '**Merge Risk:** Minimal\n'
                + f'<!-- final_review_risk_coverage:{covered} -->\n'
                + finding + '\n<!-- final_review_risk_end -->\n'
                + '<!-- walkthrough_start -->\n'
                + '| Layer / File(s) | Summary |\n| :--- | :--- |\n'
                + f'| `a.rs` | {summary} |\n'
                + '<!-- walkthrough_end -->\n'
                + '<!-- pre_merge_checks_walkthrough_start -->\n'
                + f'<summary>🚥 Pre-merge checks | {checks}</summary>\n\n'
                + '| Check name | Status | Explanation |\n| :---: | :--- | :--- |\n'
                + ''.join(f'| {name} | {state} | {why} |\n' for name, state in (rows or [('Title check', checks)]))
                + '<!-- pre_merge_checks_walkthrough_end -->\n<!-- tips -->\n')

    def test_a_cosmetic_edit_is_not_the_bot_speaking_again(self):
        # tenderdash#1489, rust-dashcore#1048: the author attested, CodeRabbit
        # rewrote its comment hours later to add a banner, and the attestation
        # was thrown away — the author was asked for another that would read
        # nothing new. Both of them posted it twice.
        p, pr = fixture()
        pr['reviews'] = [r for r in pr['reviews'] if r['user'] != 'coderabbitai[bot]']
        pr['comments'] = [dict(id=1, user='coderabbitai[bot]', body=self.rabbit(),
                               created_at='2026-09-11T09:00:00Z', updated_at='2026-09-11T09:00:00Z'),
                          dict(id=3, user='owner', body=f'/self-reviewed {HEAD}',
                               created_at='2026-09-11T11:00:00Z', updated_at='2026-09-11T11:00:00Z')]
        first = evaluate(p, pr, NOW, NOW)
        self.assertEqual(first['status'], 'success', first['blockers'])
        # The banner it adds later, with the same words about the code.
        pr['comments'][0] = dict(pr['comments'][0], updated_at='2026-09-11T13:00:00Z',
                                 body=self.rabbit(extra='<a href="#">Review in Change Stack</a>\n'))
        pr['controller_diff'] = {'number': 1, 'receipts': first['receipts']}
        again = evaluate(p, pr, NOW, NOW)
        self.assertEqual(again['status'], 'success', again['blockers'])

    def test_a_check_that_fails_is_the_bot_speaking_again(self):
        # The checks are not in the block with the findings, and they are
        # verdicts too: "Out of Scope Changes check ❌" is this producer
        # saying something about the code, in prose, with no thread and no
        # changes request — the shape that makes guessing unsafe.
        p, pr = fixture()
        pr['reviews'] = [r for r in pr['reviews'] if r['user'] != 'coderabbitai[bot]']
        pr['comments'] = [dict(id=1, user='coderabbitai[bot]', body=self.rabbit(),
                               created_at='2026-09-11T09:00:00Z', updated_at='2026-09-11T09:00:00Z'),
                          dict(id=3, user='owner', body=f'/self-reviewed {HEAD}',
                               created_at='2026-09-11T11:00:00Z', updated_at='2026-09-11T11:00:00Z')]
        first = evaluate(p, pr, NOW, NOW)
        self.assertEqual(first['status'], 'success', first['blockers'])
        pr['controller_diff'] = {'number': 1, 'receipts': first['receipts']}
        # Same words about the code; the explanation of a passing check is
        # reworded. That is what it did on rust-dashcore#1048.
        pr['comments'][0] = dict(pr['comments'][0], updated_at='2026-09-11T13:00:00Z',
                                 body=self.rabbit(why='It identifies the change well.'))
        self.assertEqual(evaluate(p, pr, NOW, NOW)['status'], 'success', 'a reworded explanation')
        # The verdict itself changes.
        pr['comments'][0] = dict(pr['comments'][0], body=self.rabbit(checks='❌ Failed'))
        self.assertEqual(evaluate(p, pr, NOW, NOW)['state'], 'waiting-self-review')

    def test_the_checks_are_read_as_a_set_and_the_file_summaries_not_at_all(self):
        # It reorders its own table between two writes of the same report —
        # that happened twice in the four recorded versions of the comment on
        # rust-dashcore#1048 — and it rewords the summary of each file freely.
        # Neither is it saying anything new about the code.
        from pr_review.policy import receipt_print
        first = {'id': 1, 'body': self.rabbit(rows=[('Title check', '✅ Passed'),
                                                    ('Docstring Coverage', '✅ Passed')])}
        same = {'id': 1, 'body': self.rabbit(rows=[('Docstring Coverage', '✅ Passed'),
                                                   ('Title check', '✅ Passed')],
                                             summary='Adds a field, and tests for it.')}
        self.assertEqual(receipt_print(first), receipt_print(same))
        flipped = {'id': 1, 'body': self.rabbit(rows=[('Title check', '❌ Failed'),
                                                      ('Docstring Coverage', '✅ Passed')])}
        self.assertNotEqual(receipt_print(first), receipt_print(flipped))

    def test_findings_this_cannot_read_are_not_read_as_nothing(self):
        # The marker that makes a comment a receipt sits at the top of the
        # findings block. A block with no end — markup that changed, a comment
        # trimmed at GitHub's limit — would leave the findings invisible while
        # the checks around them still produced a print, and a blocker written
        # into them afterwards would never move the floor.
        from pr_review.policy import receipt_print
        whole = self.rabbit()
        self.assertIsNotNone(receipt_print({'id': 1, 'body': whole}))
        torn = whole.replace('<!-- final_review_risk_end -->', '<!-- final_review_risk_finished -->')
        self.assertIsNone(receipt_print({'id': 1, 'body': torn}))
        self.assertIsNone(receipt_print({'id': 1, 'body': torn + '\nCRITICAL: do not merge.'}))

    def test_a_finding_shaped_like_a_table_row_is_not_a_table_row(self):
        # The parts are kept apart, so text in one cannot be mistaken for the
        # other and two different reports cannot print the same.
        from pr_review.policy import receipt_print
        start, end = '<!-- final_review_risk_start -->', '<!-- final_review_risk_end -->'
        # The same words: in one they are the finding, in the other a check.
        a = {'id': 1, 'body': f'{start}A{end}\n|B|✅|\n'}
        b = {'id': 1, 'body': f'{start}A\nB|✅{end}\n'}
        self.assertNotEqual(receipt_print(a), receipt_print(b))

    def test_a_new_finding_is_the_bot_speaking_again(self):
        # platform#4653: the blocker was in the prose of the receipt itself —
        # "Add signer support or defer selecting these keys before merging" —
        # with no thread and no changes request. An attestation written before
        # it has not read it.
        p, pr = fixture()
        pr['reviews'] = [r for r in pr['reviews'] if r['user'] != 'coderabbitai[bot]']
        pr['comments'] = [dict(id=1, user='coderabbitai[bot]', body=self.rabbit(),
                               created_at='2026-09-11T09:00:00Z', updated_at='2026-09-11T09:00:00Z'),
                          dict(id=3, user='owner', body=f'/self-reviewed {HEAD}',
                               created_at='2026-09-11T11:00:00Z', updated_at='2026-09-11T11:00:00Z')]
        first = evaluate(p, pr, NOW, NOW)
        pr['comments'][0] = dict(pr['comments'][0], updated_at='2026-09-11T13:00:00Z',
                                 body=self.rabbit(finding='Add signer support before merging.'))
        pr['controller_diff'] = {'number': 1, 'receipts': first['receipts']}
        again = evaluate(p, pr, NOW, NOW)
        self.assertEqual(again['state'], 'waiting-self-review')

    def test_what_two_comments_said_is_told_apart(self):
        # Two reports are two reports even when they say the same thing, and
        # one of them must not inherit the moment the other was first read.
        from pr_review.policy import receipt_print
        body = self.rabbit()
        self.assertNotEqual(receipt_print({'id': 1, 'body': body}),
                            receipt_print({'id': 2, 'body': body}))

    def test_a_comment_that_states_nothing_has_nothing_to_compare(self):
        # Then the time it was written stands, which is what it was before any
        # of this — never a print that two unrelated comments would share.
        from pr_review.policy import receipt_print
        self.assertIsNone(receipt_print({'id': 1, 'body': 'thanks!'}))
        self.assertIsNone(receipt_print({'id': 2, 'body': ''}))

    def test_a_receipt_this_controller_has_not_seen_dates_from_the_comment(self):
        # Nothing remembered yet, and nothing assumed: the comment's own time
        # is the floor, which is what it was before any of this.
        p, pr = fixture()
        pr['reviews'] = [r for r in pr['reviews'] if r['user'] != 'coderabbitai[bot]']
        pr['comments'] = [dict(id=1, user='coderabbitai[bot]', body=self.rabbit(),
                               created_at='2026-09-11T09:00:00Z', updated_at='2026-09-11T13:00:00Z'),
                          dict(id=3, user='owner', body=f'/self-reviewed {HEAD}',
                               created_at='2026-09-11T11:00:00Z', updated_at='2026-09-11T11:00:00Z')]
        self.assertEqual(evaluate(p, pr, NOW, NOW)['state'], 'waiting-self-review')

    def test_a_machine_author_does_not_spend_a_review_slot(self):
        # The five are a limit on one person's attention. An account that
        # opens pull requests on its own has none to ration, and what its pull
        # requests cost reviewers is governed by the approvals they still need.
        p, pr = fixture()
        p['bot_authors'] = ['infraclaw-dash']
        pr.update(author='infraclaw-dash', comments=[])
        result = evaluate(p, pr, None, NOW)
        self.assertEqual(result['state'], 'ready-for-human')
        # It holds no slot, so admission is never recorded for it — and the
        # waiting time keyed off admission, leaving exactly these pull
        # requests reported as "not recorded" and sorted for ever as the
        # freshest thing in the queue.
        self.assertEqual(result['ready_since'], NOW)
        # The same pull request from a person, attested, still waits its turn.
        pr.update(author='reviewer', comments=[dict(id=3, user='reviewer', body=f'/self-reviewed {HEAD}',
                                                    created_at='2026-09-11T11:00:00Z',
                                                    updated_at='2026-09-11T11:00:00Z')])
        self.assertEqual(evaluate(p, pr, None, NOW)['state'], 'too-many-open-prs')

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
        self.assertEqual(result['state'], 'waiting-author')
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

    def test_the_verdict_says_who_must_approve_what_and_what_is_already_covered(self):
        # A pull request was held with one approval in hand: seventy-one files
        # the author owned needed none, five files no area owns needed a lead,
        # and the report said only that "human approval is required".
        p, pr = fixture()
        p['areas'].append(dict(id='docs', paths=['docs/'], owners=['scribe'], reviewers=['editor']))
        pr['author'] = pr['comments'][0]['user'] = 'owner'          # owns the fixture's first area
        pr['files'] = [{'filename': 'packages/drive/x.rs'}, {'filename': 'docs/a.md'},
                       {'filename': '.github/workflows/ci.yml'}, {'filename': 'AGENTS.md'}]
        pr['permissions'].update(scribe='write', editor='write')
        pr['reviews'].append(dict(id=7, user='editor', state='APPROVED', commit_id=HEAD, submitted_at=NOW, body=''))
        result = evaluate(p, pr, NOW, NOW)
        self.assertEqual(result['state'], 'ready-for-human')
        by_area = {a['area']: a for a in result['approvals']}
        self.assertTrue(by_area[p['areas'][0]['id']]['owned'], 'owned: no approval needed, and said so')
        self.assertEqual(by_area['docs']['approved_by'], ['editor'], 'covered, and by whom')
        self.assertEqual(by_area['fallback']['approved_by'], [])
        self.assertEqual(by_area['fallback']['files'], ['.github/workflows/ci.yml', 'AGENTS.md'], 'the files that put it in play')
        self.assertEqual(by_area['fallback']['approvers'], ['fallback'])
        self.assertEqual(result['reviewers'], ['fallback'], 'only the uncovered area is asked')

    def test_an_objection_is_named_and_the_approval_picture_still_shows(self):
        p, pr = fixture()
        pr['author'] = pr['comments'][0]['user'] = 'reviewer'
        pr['threads'] = [dict(id=9, author='owner', is_resolved=False, created_at='2026-09-11T12:30:00Z',
                              voices=[dict(user='owner', created_at='2026-09-11T12:30:00Z')])]
        result = evaluate(p, pr, NOW, '2026-09-11T13:00:00Z')
        self.assertEqual(result['state'], 'waiting-author')
        self.assertEqual(result['objections'], ['owner left a review thread unresolved'])
        self.assertTrue(result['approvals'], 'the author still sees what approval the areas need')

    def test_the_attestation_can_be_the_body_of_the_authors_own_review(self):
        # tenderdash#1490: the author submitted `/self-reviewed` as a review
        # from the files view, which is one action on the diff being attested
        # to. Nothing read it, and they had to post it again as a comment.
        p, pr = fixture()
        pr['author'] = pr['comments'][0]['user'] = 'reviewer'
        pr['comments'] = []
        pr['head_seen_at'] = '2026-09-11T09:00:00Z'
        self.assertEqual(evaluate(p, pr, NOW, NOW)['state'], 'waiting-self-review')
        pr['reviews'].append(dict(id=9, user='reviewer', state='COMMENTED', commit_id=HEAD,
                                  submitted_at='2026-09-11T12:00:00Z', body='/self-reviewed'))
        self.assertEqual(evaluate(p, pr, NOW, NOW)['state'], 'ready-for-human')
        # The same rules: it must follow the bots, and nobody else's review counts.
        pr['threads'] = [dict(id=1, author='coderabbitai[bot]', is_resolved=True, created_at='2026-09-11T10:00:00Z')]
        pr['reviews'][-1]['submitted_at'] = '2026-09-11T09:30:00Z'
        self.assertEqual(evaluate(p, pr, NOW, NOW)['state'], 'waiting-self-review', 'before what a bot said')
        pr['reviews'][-1].update(submitted_at='2026-09-11T12:00:00Z', user='owner')
        self.assertEqual(evaluate(p, pr, NOW, NOW)['state'], 'waiting-self-review', 'only the author attests')

    def test_an_ordinary_review_by_the_author_is_not_an_attestation(self):
        # Sixty-two of these exist across the governed repositories, most with
        # no body at all — replies to a bot's findings. Reading any of them as
        # "I have read the final diff" would attest by accident.
        p, pr = fixture()
        pr['author'] = pr['comments'][0]['user'] = 'reviewer'
        pr['comments'] = []
        pr['head_seen_at'] = '2026-09-11T09:00:00Z'
        for body in ('', 'LGTM', 'Reviewed', 'nit: rename this'):
            pr['reviews'] = [r for r in pr['reviews'] if r['user'] != 'reviewer']
            pr['reviews'].append(dict(id=9, user='reviewer', state='COMMENTED', commit_id=HEAD,
                                      submitted_at='2026-09-11T12:00:00Z', body=body))
            self.assertEqual(evaluate(p, pr, NOW, NOW)['state'], 'waiting-self-review', repr(body))

    def test_a_bot_authors_stand_in_attestation_is_dated_when_the_bots_were_done(self):
        # It stands in for an attestation, so it must not be dated at the
        # floor: with clean bots the floor is the day the pull request opened,
        # and every objection since would read as unanswered.
        p, pr = fixture()
        pr.update(author='Copilot', author_is_bot=True, comments=[], head_seen_at=None)
        pr['reviews'].append(dict(id=8, user='owner', state='CHANGES_REQUESTED', commit_id=HEAD,
                                  submitted_at='2026-09-11T09:30:00Z', body=''))
        pr['permissions'] = dict(pr['permissions'], owner='write')
        result = evaluate(p, pr, NOW, NOW)
        self.assertNotEqual(result['state'], 'waiting-author',
                            "the objection predates the bots finishing; a bot author cannot post an answer")

    def test_waiting_for_the_bots_means_a_bot_has_not_reported(self):
        # Forty-five pull requests sat in waiting-bots across the governed
        # repositories and only nine were waiting for a bot; the rest were
        # waiting for their author to answer what a bot had already said, and
        # eighteen of those also wore the label saying a bot had been skipped.
        # A pull request cannot be waiting for a bot it gave up on.
        p, pr = fixture()
        silent = copy.deepcopy(pr)
        silent['reviews'] = [r for r in silent['reviews'] if 'pastaclaw' not in r['user']]
        self.assertEqual(evaluate(p, silent, NOW, NOW)['state'], 'waiting-bots')
        spoke = copy.deepcopy(pr)
        spoke['threads'] = [dict(id=1, author='coderabbitai[bot]', is_resolved=False, created_at=NOW)]
        result = evaluate(p, spoke, NOW, NOW)
        self.assertEqual(result['state'], 'waiting-author')
        self.assertEqual(result['blockers'], ['coderabbitai left review threads unresolved; resolve them'])

    def test_a_skipped_bot_and_a_wait_for_the_bots_cannot_both_be_true(self):
        p, pr = fixture()
        p['bot_timeouts'] = {'nudge_after_hours': 1, 'waive_after_hours': 2}
        pr['head_seen_at'] = '2026-09-10T00:00:00Z'          # the window has run out
        pr['reviews'] = [r for r in pr['reviews'] if 'pastaclaw' not in r['user']]
        pr['threads'] = [dict(id=1, author='coderabbitai[bot]', is_resolved=False, created_at=NOW)]
        result = evaluate(p, pr, NOW, NOW)
        self.assertEqual(result['waived'], ['thepastaclaw'], 'one bot given up on')
        self.assertNotEqual(result['state'], 'waiting-bots', 'and so nothing is waiting for a bot')
        self.assertEqual(result['state'], 'waiting-author')

    def test_every_bot_receipt_raises_the_floor_however_the_finding_is_shaped(self):
        # A bot states a blocker in the prose of the receipt itself — "Add
        # signer support or defer selecting these keys before merging", no
        # thread, no changes request (dashpay/platform#4653). "It had nothing
        # to say" cannot be read off the evidence, so every receipt counts.
        p, pr = fixture()
        pr['head_seen_at'] = '2026-09-11T09:00:00Z'
        pr['threads'] = []
        pr['comments'][0].update(body='/self-reviewed', created_at='2026-09-11T09:30:00Z',
                                 updated_at='2026-09-11T09:30:00Z')
        self.assertEqual(evaluate(p, pr, NOW, NOW)['state'], 'waiting-self-review',
                         'the bots reported after it; nothing here can prove they found nothing')
        pr['comments'][0].update(created_at='2026-09-11T11:00:00Z', updated_at='2026-09-11T11:00:00Z')
        self.assertEqual(evaluate(p, pr, NOW, NOW)['state'], 'ready-to-merge')

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
            'too-many-open-prs': shape(author='reviewer', admitted=None),
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
        self.assertEqual(set(seen), {'draft', 'too-many-open-prs', 'waiting-bots', 'waiting-build', 'waiting-self-review',
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
        # A bot that said something on this head is what puts a floor under
        # the attestation; answered or not, it still had to be read.
        pr['threads'] = [dict(id=1, author='coderabbitai[bot]', is_resolved=True, created_at='2026-09-11T10:00:00Z')]
        pr['comments'][0].update(body='/self-reviewed', created_at='2026-09-11T09:30:00Z',
                                 updated_at='2026-09-11T09:30:00Z')
        self.assertEqual(evaluate(p, pr, NOW, NOW)['state'], 'waiting-self-review')

    def test_an_attestation_sharing_a_second_with_its_floor_does_not_count(self):
        p, pr = fixture()
        pr['head_seen_at'] = '2026-09-11T09:00:00Z'
        # A bot that said something on this head is what puts a floor under
        # the attestation; answered or not, it still had to be read.
        pr['threads'] = [dict(id=1, author='coderabbitai[bot]', is_resolved=True, created_at='2026-09-11T10:00:00Z')]
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
        # A receipt on another commit, or one that is not the final phase, is
        # a bot that has not reported on this head: the wait is the bot's.
        for update in [{'commit_id':'c'*40}, {'body':'preliminary'}]:
            changed = copy.deepcopy(pr)
            changed['reviews'][0].update(update)
            self.assertEqual(evaluate(p,changed,NOW,NOW)['state'], 'waiting-bots', update)
        # It objected: it has reported, and answering it is the author's move.
        changed = copy.deepcopy(pr)
        changed['reviews'][0].update(state='CHANGES_REQUESTED')
        self.assertEqual(evaluate(p,changed,NOW,NOW)['state'], 'waiting-author')

    def test_self_review_requires_unedited_author_confirmation_after_bots(self):
        p, pr = fixture()
        # A bot that said something on this head is what puts a floor under
        # the attestation; answered or not, it still had to be read.
        pr['threads'] = [dict(id=1, author='coderabbitai[bot]', is_resolved=True, created_at='2026-09-11T10:00:00Z')]
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
        self.assertEqual(sixth['state'], 'too-many-open-prs')
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
        # A bot that said something on this head is what puts a floor under
        # the attestation; answered or not, it still had to be read.
        pr['threads'] = [dict(id=1, author='coderabbitai[bot]', is_resolved=True, created_at='2026-09-11T10:00:00Z')]
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
        self.assertEqual(evaluate(p,pr,NOW,NOW)['state'], 'waiting-author')

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
        # A bot that said something on this head is what puts a floor under
        # the attestation; answered or not, it still had to be read.
        pr['threads'] = [dict(id=1, author='coderabbitai[bot]', is_resolved=True, created_at='2026-09-11T10:00:00Z')]
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
