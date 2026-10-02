"""GitHub evidence must be complete before the policy can make a decision."""

import json
import subprocess
import unittest
from unittest.mock import patch

from pr_review.github import GitHub, GitHubError, build_verdict, parse_controller_state


def check(name, conclusion, started, workflow="/dashpay/x/actions/workflows/ci.yml", job=True):
    return {"__typename": "CheckRun", "name": name, "conclusion": conclusion, "status": "COMPLETED",
            "startedAt": started,
            "detailsUrl": "https://github.com/dashpay/x/actions/runs/1/job/2" if job
                          else "https://example.test/report",
            "checkSuite": {"workflowRun": {"workflow": {"resourcePath": workflow}}}}


OURS = "/dashpay/x/actions/workflows/pr-review-policy.yml"


def rollup(nodes=None, head="a" * 40, total=None, more=False, cursor=None):
    return {"data": {"repository": {"pullRequest": {"commits": {"nodes": [{"commit": {
        "oid": head,
        "statusCheckRollup": {"contexts": {
            "totalCount": len(nodes or []) if total is None else total,
            "pageInfo": {"hasNextPage": more, "endCursor": cursor},
            "nodes": nodes or []}}}}]}}}}}


class BuildVerdictTests(unittest.TestCase):
    checks = staticmethod(rollup)
    def test_a_diff_marker_somebody_else_edited_is_not_read(self):
        # It says which commits a review still covers. Anyone with write
        # access can edit anyone's comment, and a forged one carries a stale
        # approval — the only human gate left — onto code nobody read.
        from pr_review.github import parse_controller_diff
        diff = {'number': 7, 'diff': 'a' * 64, 'diff_heads': ['b' * 40], 'diff_seen': '2026-09-11T10:00:00Z'}
        body = GitHub.state_comment_body(
            {'version': 1, 'number': 7, 'head': 'b' * 40, 'admitted_at': None, 'ready_since': None,
              'state': 'ready-for-human', 'evidence': 'c' * 64, 'context': 'd' * 64}, 'text', diff)
        made = dict(id=1, user='github-actions[bot]', body=body,
                    created_at='2026-09-11T10:00:00Z', updated_at='2026-09-11T10:00:00Z')
        self.assertEqual(parse_controller_diff([made], 7), diff)
        edited = dict(made, updated_at='2026-09-11T12:00:00Z', edited_at='2026-09-11T12:00:00Z')
        # Edited by a person, or by an account GitHub can no longer name.
        for editor in ('llbartekll', None):
            self.assertIsNone(parse_controller_diff([dict(edited, edited_by=editor)], 7), repr(editor))
        # A moved update time with no edit recorded is no edit: GitHub moves
        # it when a comment is hidden. The record is read by the same rule.
        self.assertEqual(parse_controller_diff([dict(made, updated_at='2026-09-11T12:00:00Z')], 7), diff)
        # This controller's own hand: the editor is read with its type, so it
        # comes back as `github-actions[bot]`. Refusing it refused every record
        # the controller had refreshed — every record after its first write.
        # The bare name is one a person can register, and counts for nothing.
        self.assertEqual(parse_controller_diff([dict(edited, edited_by='github-actions[bot]')], 7), diff)
        self.assertIsNone(parse_controller_diff([dict(edited, edited_by='github-actions')], 7))
        # And only beside this controller's record for this same pull request:
        # any workflow can post as the Actions app.
        alone = dict(made, id=2, body='<!-- pr-hygiene-diff-v1 ' + json.dumps(diff) + ' -->')
        self.assertIsNone(parse_controller_diff([alone], 7))
        self.assertIsNone(parse_controller_diff([made], 8), 'another pull request')

    def record(self, updated_at, edited_by=None, **state):
        record = {'version': 1, 'number': 7, 'head': 'b' * 40, 'admitted_at': '2026-09-11T10:00:00Z',
                  'ready_since': None, 'state': 'waiting-bots', 'evidence': 'c' * 64, 'context': 'd' * 64}
        return dict(id=1, user='github-actions[bot]', body=GitHub.state_comment_body(dict(record, **state), 'text'),
                    created_at='2026-09-11T10:00:00Z', updated_at=updated_at, edited_by=edited_by,
                    edited_at=updated_at if edited_by else None)

    def test_a_record_whose_admission_a_collaborator_edited_is_not_trusted(self):
        # The record says when this pull request took one of its author's
        # review slots, and whether it was ever ready for a human on this
        # head. Anyone with write access can edit anyone's comment: an earlier
        # admission moves the pull request ahead of the author's own queue, and
        # `ready-for-human` on the current head is remembered as having passed
        # the green build that is asked for before a human is.
        forged = self.record('2026-09-11T12:00:00Z', 'llbartekll',
                             admitted_at='2026-01-01T00:00:00Z', state='ready-for-human')
        self.assertEqual(parse_controller_state([forged]), (None, None))
        # The bare name is one a person can register.
        self.assertEqual(parse_controller_state([dict(forged, edited_by='github-actions')]), (None, None))
        # An edit GitHub records but cannot say by whom, as for a deleted or
        # suspended account, is nobody this controller trusts.
        self.assertEqual(parse_controller_state([dict(forged, edited_by=None)]), (None, None))
        # Ignored, not refused: refusing it would hand anyone with write
        # access a configuration error on any pull request, one edit away.
        broken = dict(forged, body=forged['body'].replace('"version":1', '"version":2'))
        self.assertEqual(parse_controller_state([broken]), (None, None))
        # The newest record decides even forged: nothing older stands in for
        # it, or editing the newest would bring back what an older one held.
        kept = self.record('2026-09-11T10:00:00Z', None)
        self.assertEqual(parse_controller_state([kept, dict(forged, id=2)]), (None, None))

    def test_editing_the_newest_record_does_not_bring_back_an_older_admission(self):
        # An older announcement still carries the admission of its day; the
        # newest record, set aside when the pull request left the policy,
        # carries none. Were a forged newest record merely skipped, the older
        # one would be read in its place, and one edit would hand back the
        # place in the queue the pull request gave up.
        from pr_review.policy import effective_admission
        old = dict(self.record('2026-01-01T00:00:00Z', None, admitted_at='2026-01-01T00:00:00Z'),
                   id=1, created_at='2026-01-01T00:00:00Z')
        aside = dict(self.record('2026-09-12T00:00:00Z', 'github-actions[bot]', admitted_at=None, state='not-governed'),
                     id=2, created_at='2026-09-01T00:00:00Z')
        self.assertEqual(parse_controller_state([old, aside])[1], 2)
        forged = dict(aside, edited_by='mallory', edited_at='2026-09-13T00:00:00Z', updated_at='2026-09-13T00:00:00Z')
        state, comment_id = parse_controller_state([old, forged])
        self.assertEqual((state, comment_id), (None, None))
        self.assertIsNone(effective_admission({'controller_state': state, 'lifecycle_at': None}))

    def test_an_older_record_hidden_since_is_not_taken_for_the_newest(self):
        # Hiding a comment moves its update time and records no edit. The
        # current record is the one this controller wrote last, whatever was
        # done to an older one afterwards.
        old = dict(self.record('2026-09-14T00:00:00Z', None, admitted_at='2026-01-01T00:00:00Z'),
                   id=1, created_at='2026-01-01T00:00:00Z')
        newer = dict(self.record('2026-09-12T00:00:00Z', 'github-actions[bot]', admitted_at=None, state='not-governed'),
                     id=2, created_at='2026-09-01T00:00:00Z')
        self.assertEqual(parse_controller_state([old, newer])[1], 2)

    def test_an_older_diff_hidden_since_is_not_taken_for_the_newest(self):
        # The diff is read by the same clock as the record beside it, or the
        # two could come from different comments.
        from pr_review.github import parse_controller_diff
        record = {'version': 1, 'number': 7, 'head': 'b' * 40, 'admitted_at': None, 'ready_since': None,
                  'state': 'waiting-bots', 'evidence': 'c' * 64, 'context': 'd' * 64}
        older_diff = {'number': 7, 'diff': 'a' * 64, 'diff_heads': ['b' * 40]}
        newer_diff = {'number': 7, 'diff': 'e' * 64, 'diff_heads': ['b' * 40]}
        old = dict(id=1, user='github-actions[bot]', body=GitHub.state_comment_body(record, 'text', older_diff),
                   created_at='2026-09-01T00:00:00Z', updated_at='2026-09-14T00:00:00Z', edited_at=None, edited_by=None)
        newer = dict(id=2, user='github-actions[bot]', body=GitHub.state_comment_body(record, 'text', newer_diff),
                     created_at='2026-09-10T00:00:00Z', updated_at='2026-09-12T00:00:00Z',
                     edited_at='2026-09-12T00:00:00Z', edited_by='github-actions[bot]')
        self.assertEqual(parse_controller_diff([old, newer], 7), newer_diff)

    def test_a_record_this_controller_refreshed_is_still_read(self):
        # Every run that changes the record rewrites it in place, so all but
        # a new record have been edited — by this controller. Refusing those
        # would give up every pull request's slot and review clock each run.
        refreshed = self.record('2026-09-11T12:00:00Z', 'github-actions[bot]', state='ready-for-human')
        state, comment_id = parse_controller_state([refreshed])
        self.assertEqual((comment_id, state['state']), (1, 'ready-for-human'))

    def test_a_record_nobody_edited_is_read_though_its_update_time_moved(self):
        # GitHub moves a comment's update time for changes that are not
        # edits, and names no editor for them. Who edited the record decides,
        # not when it was last touched: a record nobody edited is this
        # controller's own words, and ignoring it gives up the pull request's
        # slot and review clock for nothing.
        touched = self.record('2026-09-11T12:00:00Z', None, state='ready-for-human')
        state, comment_id = parse_controller_state([touched])
        self.assertEqual((comment_id, state['state']), (1, 'ready-for-human'))

    def test_a_diff_timestamp_that_is_not_one_is_refused(self):
        # It is read back as a time. A string that is not one raised out of
        # the verdict and took the whole repository's run with it.
        from pr_review.github import _validate_diff
        for bad in ('banana', '2026-09-11', '2026-09-11T10:00:00+03:00', 5):
            with self.assertRaises(GitHubError, msg=repr(bad)):
                _validate_diff({'number': 1, 'diff': 'a' * 64, 'diff_heads': ['b' * 40], 'diff_seen': bad})
        # A key that is there is checked. An empty head list was accepted
        # once, and it handed whoever wrote it the instant an attestation is
        # measured against.
        with self.assertRaises(GitHubError):
            _validate_diff({'number': 1, 'diff': 'a' * 64, 'diff_heads': [], 'diff_seen': None})
        # A key that is not there is simply not read, so a marker written by a
        # newer engine does not make an older one refuse everything it knows.
        _validate_diff({'number': 1})
        _validate_diff({'number': 1, 'receipts': {'c' * 64: '2026-09-11T10:00:00Z'}})


    def test_re_running_a_flaky_check_clears_it(self):
        # The question this whole rule turns on. A re-run does not replace the
        # run it repeats, it adds another beside it, so the failure stays on the
        # head for ever. Counting it would mean a flake could never be cleared.
        # The newest run decides, and the newest is not the last one listed:
        # GitHub returns these in no useful order.
        self.assertEqual(build_verdict([
            check("tests", "SUCCESS", "2026-09-01T11:00:00Z"),
            check("tests", "FAILURE", "2026-09-01T10:00:00Z")]), "green")
        self.assertEqual(build_verdict([
            check("tests", "FAILURE", "2026-09-01T11:00:00Z"),
            check("tests", "SUCCESS", "2026-09-01T10:00:00Z")]), "failed")

    def test_this_controller_never_waits_for_itself(self):
        # It is a check on the pull requests it governs, and its own run ends
        # cancelled on about a fifth of heads — permanently. Counting any of
        # them would deadlock those pull requests with no error anywhere.
        # The decisive shape is its own run, still going, while it decides.
        running = dict(check("policy / reconcile", None, "2026-09-01T10:02:00Z", OURS), status="IN_PROGRESS")
        nodes = [check("policy / reconcile", "CANCELLED", "2026-09-01T10:00:00Z", OURS),
                 check("policy / reconcile", "CANCELLED", "2026-09-01T10:00:01Z", OURS),
                 running,
                 {"__typename": "StatusContext", "context": "PR Hygiene", "state": "pending",
                  "createdAt": "2026-09-01T10:00:00Z"},
                 check("tests", "SUCCESS", "2026-09-01T10:00:00Z")]
        self.assertEqual(build_verdict(nodes), "green")

    def test_a_renamed_job_in_our_workflow_is_still_ours(self):
        # Each repository owns its caller and may rename the job; the workflow
        # path is the part this engine refuses to run from anywhere else.
        renamed = dict(check("hygiene / anything", None, "2026-09-01T10:00:00Z", OURS), status="IN_PROGRESS")
        self.assertEqual(build_verdict([renamed]), "green")

    def test_another_tool_filed_into_our_suite_is_not_ours(self):
        # GitHub files API-created check runs into whichever Actions suite is
        # current, so workflow attribution alone would swallow real failures.
        self.assertEqual(build_verdict([
            check("Clippy Report", "FAILURE", "2026-09-01T10:00:00Z", OURS, job=False)]), "failed")

    def test_cancelled_alone_is_not_a_failure(self):
        # It is what concurrency looks like. Treating it as red made 6 of 16
        # rust-dashcore pull requests unmergeable for jobs cancelled by design.
        self.assertEqual(build_verdict([check("check-title", "CANCELLED", "2026-09-01T10:00:00Z")]), "green")

    def test_nothing_to_check_is_green_not_blocked_for_ever(self):
        self.assertEqual(build_verdict([]), "green")

    def test_skipped_and_neutral_do_not_fail(self):
        self.assertEqual(build_verdict([check("a", "SKIPPED", "1"), check("b", "NEUTRAL", "1")]), "green")

    def test_unfinished_work_is_running_not_green(self):
        for status in ["QUEUED", "IN_PROGRESS", "WAITING", "REQUESTED"]:
            self.assertEqual(build_verdict([dict(check("a", None, "1"), status=status)]), "running", status)
        self.assertEqual(build_verdict([{"__typename": "StatusContext", "context": "ci",
                                         "state": "pending", "createdAt": "1"}]), "running")

    def test_more_than_one_page_of_checks_is_read_not_refused(self):
        # Live precedent: platform #2974 carries 113 contexts. Refusing would
        # error every pull request of that author, and blank the repository out
        # of the digest.
        api = GitHub("dashpay/platform")
        pages = [self.checks(nodes=[check("a", "SUCCESS", "1")], more=True, cursor="next"),
                 self.checks(nodes=[check("b", "FAILURE", "1")])]
        with patch.object(api, "request", side_effect=pages):
            self.assertEqual(api.build_state(1, "a" * 40), "failed")

    def test_a_partial_answer_is_never_read_as_no_checks(self):
        # statusCheckRollup is nullable, so a rate-limited or timed-out read
        # nulls it and reports the error beside it. Reading that as "this
        # repository has no CI" would quietly open the gate for every pull
        # request in the run.
        api = GitHub("dashpay/platform")
        partial = {"data": {"repository": {"pullRequest": {"commits": {"nodes": [
            {"commit": {"oid": "a" * 40, "statusCheckRollup": None}}]}}}},
            "errors": [{"type": "RATE_LIMITED", "message": "rate limited"}]}
        with patch.object(api, "request", return_value=partial):
            with self.assertRaises(GitHubError):
                api.build_state(1, "a" * 40)

    def test_the_answer_is_read_once_per_head(self):
        api = GitHub("dashpay/platform")
        with patch.object(api, "request", return_value=self.checks()) as request:
            api.build_state(1, "a" * 40)
            api.build_state(1, "a" * 40)
        self.assertEqual(request.call_count, 1)

    def test_a_head_that_moved_under_the_read_is_not_reported_green(self):
        api = GitHub("dashpay/platform")
        with patch.object(api, "request", return_value=self.checks(nodes=[], head="b" * 40)):
            self.assertEqual(api.build_state(1, "a" * 40), "running")

    def test_a_pull_request_with_no_checks_at_all_is_green(self):
        api = GitHub("dashpay/platform")
        empty = {"data": {"repository": {"pullRequest": {"commits": {"nodes": [
            {"commit": {"oid": "a" * 40, "statusCheckRollup": None}}]}}}}}
        with patch.object(api, "request", return_value=empty):
            self.assertEqual(api.build_state(1, "a" * 40), "green")

    def test_a_finished_check_that_wants_a_human_is_not_waited_for(self):
        # ACTION_REQUIRED is a conclusion, not a status: the check has finished
        # and needs someone. Calling it running would hold the pull request at
        # "waiting for the build to finish" for ever, with nothing to clear it.
        self.assertEqual(build_verdict([check("deploy", "ACTION_REQUIRED", "1")]), "failed")

    def test_a_status_in_error_is_a_failure_not_a_pass(self):
        # Integrations post `error` for infrastructure failures as often as
        # `failure` for test failures, and GraphQL spells it ERROR.
        for state, expected in [("ERROR", "failed"), ("FAILURE", "failed"),
                                ("PENDING", "running"), ("EXPECTED", "green"), ("SUCCESS", "green")]:
            node = {"__typename": "StatusContext", "context": "ci", "state": state, "createdAt": "1"}
            self.assertEqual(build_verdict([node]), expected, state)

    def test_a_status_and_a_check_of_the_same_name_are_different_checks(self):
        nodes = [{"__typename": "StatusContext", "context": "build", "state": "FAILURE", "createdAt": "1"},
                 dict(check("build", "SUCCESS", "2"), checkSuite=None)]
        self.assertEqual(build_verdict(nodes), "failed")

    def test_whoever_posts_a_skip_has_their_access_read(self):
        # The snapshot reads access for area people, reviewers and thread
        # authors. A skip from anyone else was silently nobody's — which was
        # most writers, and most authors.
        api = GitHub("dashpay/platform")
        raw = {"number": 1, "user": {"login": "author", "type": "User"}, "head": {"sha": "a" * 40},
               "base": {"ref": "v4.2-dev", "sha": "b" * 40}, "draft": False, "state": "open",
               "created_at": "2026-09-11T00:00:00Z", "html_url": "u", "title": "t", "changed_files": 1,
               "requested_reviewers": [], "labels": []}
        comments = [{"id": 1, "user": "helper", "body": "/skip-bots", "created_at": "2026-09-11T01:00:00Z", "updated_at": "2026-09-11T01:00:00Z"},
                    {"id": 2, "user": "chatter", "body": "nice", "created_at": "2026-09-11T01:00:00Z", "updated_at": "2026-09-11T01:00:00Z"}]
        looked_up = []
        pages = {"/files": [{"filename": "packages/rs-drive/x"}], "/reviews": []}
        with patch.object(api, "request", return_value=raw), \
             patch.object(api, "pages", side_effect=lambda path: next(v for k, v in pages.items() if path.endswith(k))), \
             patch.object(api, "threads", return_value=[]), \
             patch.object(api, "build_state", return_value="green"), \
             patch.object(api, "head_seen_at", return_value=None), \
             patch.object(api, "ready_published", return_value=False), \
             patch.object(api, "permission", side_effect=lambda u: looked_up.append(u) or "write"):
            snapshot = api.snapshot(1, {"fallback": {"owners": ["QuantumExplorer"], "reviewers": []}, "areas": [],
                                        "target_branches": ["v4.2-dev"]},
                                    history={"comments": comments, "lifecycle_at": None})
        self.assertIn("helper", looked_up, "the skipper is looked up")
        self.assertNotIn("chatter", looked_up, "nobody else who merely commented is")
        self.assertEqual(snapshot["permissions"].get("helper"), "write")
    def test_a_flaky_read_is_asked_once_more_and_a_post_never_is(self):
        # dashpay/platform#4718: one "unexpected end of JSON input" from gh
        # marked the pull request an error under the required check for hours.
        api = GitHub("dashpay/platform")
        flaky = [subprocess.CompletedProcess([], 1, "", "unexpected end of JSON input"),
                 subprocess.CompletedProcess([], 0, '{"ok": true}', "")]
        with patch("pr_review.github.subprocess.run", side_effect=flaky) as run, patch("pr_review.github.time.sleep"):
            self.assertEqual(api.request("GET", "repos/dashpay/platform/pulls/1"), {"ok": True})
        self.assertEqual(run.call_count, 2)
        refused = [subprocess.CompletedProcess([], 1, "", "HTTP 422: Validation Failed")]
        with patch("pr_review.github.subprocess.run", side_effect=refused) as run:
            with self.assertRaises(GitHubError):
                api.request("GET", "repos/dashpay/platform/pulls/1")
        self.assertEqual(run.call_count, 1, "a refusal is not flakiness")
        posted = [subprocess.CompletedProcess([], 1, "", "unexpected end of JSON input")]
        with patch("pr_review.github.subprocess.run", side_effect=posted) as run:
            with self.assertRaises(GitHubError):
                api.request("POST", "repos/dashpay/platform/issues/1/comments", {"body": "x"})
        self.assertEqual(run.call_count, 1, "a second POST could write twice")

    def test_the_snapshot_says_whether_the_author_is_a_bot(self):
        # A bot author cannot attest, and the policy has to know that from the
        # snapshot rather than guess from a login.
        api = GitHub("dashpay/platform")
        raw = {"number": 1, "user": {"login": "Copilot", "type": "Bot"}, "head": {"sha": "a" * 40},
               "base": {"ref": "v4.2-dev", "sha": "b" * 40}, "draft": False, "state": "open",
               "created_at": "2026-09-11T00:00:00Z", "html_url": "https://github.com/dashpay/platform/pull/1",
               "title": "t"}
        self.assertTrue(GitHub._pr(raw)["author_is_bot"])
        raw["user"]["type"] = "User"
        self.assertFalse(GitHub._pr(raw)["author_is_bot"])

    def test_someone_the_listing_omits_is_asked_about_directly(self):
        # An organisation's own members reach a repository through the
        # organisation, and a repository-scoped token does not enumerate them.
        # Reading that absence as "no write access" marked six pull requests a
        # configuration error the day writes were turned on.
        api = GitHub("dashpay/platform")
        asked = []

        def request(method, path, payload=None):
            asked.append(path)
            return {"permission": "admin"}

        with patch.object(api, "pages", return_value=[
                {"login": "direct", "permissions": {"push": True, "pull": True}}]):
            with patch.object(api, "request", side_effect=request):
                self.assertEqual(api.permission("direct"), "write")
                self.assertEqual(api.permission("via-the-org"), "admin")
                self.assertEqual(api.permission("via-the-org"), "admin", 'asked once, then remembered')
        self.assertEqual([path for path in asked if "/permission" in path],
                         ["repos/dashpay/platform/collaborators/via-the-org/permission"])

    def test_an_unreadable_answer_is_asked_for_once_not_on_every_snapshot(self):
        # A reconciliation snapshots the same pull request up to three times,
        # and a sweep covers six. Re-asking each time turns one unreachable
        # answer into hundreds of requests — and a rate limit into a loop that
        # answers it with more requests.
        api = GitHub("dashpay/platform")
        with patch.object(api, "pages", return_value=[]):
            with patch.object(api, "request", side_effect=GitHubError("403")) as request:
                for _ in range(5):
                    self.assertIsNone(api.permission("someone"))
        self.assertEqual(request.call_count, 1)
        api.forget_cached_access()
        with patch.object(api, "pages", return_value=[]):
            with patch.object(api, "request", return_value={"permission": "admin"}) as again:
                self.assertEqual(api.permission("someone"), "admin", 'the pre-write re-read asks again')
        self.assertEqual(again.call_count, 1)

    def test_a_fallback_answer_does_not_stand_in_for_having_read_the_listing(self):
        # The listing is read once, and "have I read it" must not be answered
        # by an entry the fallback put there — otherwise a repository whose
        # listing came back empty never reads it again.
        api = GitHub("dashpay/platform")
        with patch.object(api, "pages", return_value=[]) as pages:
            api.access()
            api.access()
        self.assertEqual(pages.call_count, 1,
                         'a listing that came back empty is still a listing that was read')
        api.forget_cached_access()
        with patch.object(api, "pages", return_value=[]) as again:
            api.access()
        self.assertEqual(again.call_count, 1, 'and the pre-write re-read reads it afresh')

    def test_the_fallback_reads_the_same_capability_flags_as_the_listing(self):
        # A custom organisation role's legacy `permission` string is only an
        # approximation; the flags beside it say plainly whether they push.
        api = GitHub("dashpay/platform")
        answer = {"permission": "read",
                  "user": {"permissions": {"admin": False, "maintain": False, "push": True,
                                           "triage": True, "pull": True}}}
        with patch.object(api, "pages", return_value=[]):
            with patch.object(api, "request", return_value=answer):
                self.assertEqual(api.permission("custom-role"), "write")

    def test_an_answer_that_never_arrives_is_unknown_not_none(self):
        api = GitHub("dashpay/platform")
        with patch.object(api, "pages", return_value=[]):
            with patch.object(api, "request", side_effect=GitHubError("403")):
                self.assertIsNone(api.permission("someone"))

    def test_the_same_job_name_in_two_workflows_does_not_mask_the_other(self):
        self.assertEqual(build_verdict([
            check("build", "SUCCESS", "2026-09-01T11:00:00Z", "/dashpay/x/actions/workflows/a.yml"),
            check("build", "FAILURE", "2026-09-01T10:00:00Z", "/dashpay/x/actions/workflows/b.yml")]), "failed")


class GitHubTests(unittest.TestCase):
    def setUp(self):
        self.api = GitHub("dashpay/platform")

    @patch("pr_review.github.subprocess.run")
    def test_should_pass_untrusted_text_as_json_stdin(self, run):
        run.return_value = subprocess.CompletedProcess([], 0, '{"id": 7}', "")
        body = "$(touch /tmp/never-run) `echo no`\n\"quotes\""
        self.assertEqual(self.api.request("POST", "repos/dashpay/platform/issues/1/comments", {"body": body}), {"id": 7})
        args, kwargs = run.call_args
        self.assertIsInstance(args[0], list)
        self.assertNotIn(body, args[0])
        self.assertEqual(json.loads(kwargs["input"]), {"body": body})
        self.assertFalse(kwargs.get("shell", False))

    @patch("pr_review.github.subprocess.run")
    def test_should_flatten_all_rest_pages(self, run):
        run.return_value = subprocess.CompletedProcess([], 0, '[[{"id":1}],[{"id":2}]]', "")
        self.assertEqual(self.api.pages("repos/dashpay/platform/issues?per_page=100"), [{"id": 1}, {"id": 2}])
        self.assertIn("--paginate", run.call_args.args[0])
        self.assertIn("--slurp", run.call_args.args[0])

    @patch("pr_review.github.subprocess.run")
    def test_should_fail_on_transport_and_non_list_pages(self, run):
        run.return_value = subprocess.CompletedProcess([], 1, "", "API denied")
        with self.assertRaises(GitHubError):
            self.api.pages("repos/dashpay/platform/issues")
        run.return_value = subprocess.CompletedProcess([], 0, '[{"message":"not a list"}]', "")
        with self.assertRaises(GitHubError):
            self.api.pages("repos/dashpay/platform/issues")

    @patch("pr_review.github.subprocess.run")
    def test_empty_collection_is_valid_but_missing_evidence_is_not(self, run):
        for output in ['[]', '[[]]']:
            run.return_value = subprocess.CompletedProcess([], 0, output, "")
            self.assertEqual(self.api.pages("repos/dashpay/platform/issues"), [])
        for output in ['null', '']:
            run.return_value = subprocess.CompletedProcess([], 0, output, "")
            with self.assertRaises(GitHubError):
                self.api.pages("repos/dashpay/platform/issues")

    def test_should_reject_repository_path_injection(self):
        for name in ("dashpay/platform/../x", "--hostname=evil", "https://github.com/a/b"):
            with self.assertRaises(GitHubError):
                GitHub(name)

    def pr(self):
        return {"number": 1, "user": {"login": "author"}, "head": {"sha": "a" * 40},
                "base": {"sha": "b" * 40, "ref": "v4.2-dev"}, "created_at": "2026-09-01T00:00:00Z",
                "draft": False, "state": "open", "html_url": "https://github.com/dashpay/platform/pull/1",
                "title": "Example", "changed_files": 1, "requested_reviewers": [], "labels": [],
                "assignees": [{"login": "romchornyi"}]}

    def graph(self, nodes=None, more=False, cursor=None, total=None):
        return {"data": {"repository": {"pullRequest": {"reviewThreads": {
            "totalCount": len(nodes or []) if total is None else total,
            "nodes": nodes or [], "pageInfo": {"hasNextPage": more, "endCursor": cursor}}}}}}

    def checks(self, nodes=None, head="a" * 40, total=None, more=False, cursor=None):
        return rollup(nodes, head, total, more, cursor)

    def snapshot_fixture(self, comments=None, files=None, graph=None, checks=None):
        def request(method, path, payload=None):
            if path == "graphql":
                # One fixture answers three queries; they are told apart the
                # same way the engine tells them apart — by what was asked for.
                query = (payload or {}).get("query", "")
                if "statusCheckRollup" in query:
                    return checks or self.checks()
                if "timelineItems" in query:
                    return {"data": {"repository": {"pr1": {
                        "number": 1,
                        "comments": {"totalCount": len(comments or []), "nodes": [
                            {"databaseId": c["id"], "body": c["body"],
                             "createdAt": c["created_at"], "updatedAt": c["updated_at"],
                             "lastEditedAt": c.get("edited_at", c["updated_at"] if c.get("edited_by") else None),
                             "author": {"login": (c["user"]["login"] if isinstance(c["user"], dict) else c["user"]).removesuffix("[bot]"),
                                        "__typename": "Bot"},
                             "editor": ({"login": c["edited_by"].removesuffix("[bot]"),
                                         "__typename": "Bot" if c["edited_by"].endswith("[bot]") else "User"}
                                        if c.get("edited_by") else None)}
                            for c in (comments or [])]},
                        "timelineItems": {"nodes": []}}}}}
                return graph or self.graph()
            if path.endswith("/permission"):
                return {"permission": "write"}
            return self.pr()

        def pages(path):
            if "/files" in path:
                return files if files is not None else [{"filename": "packages/rs-drive/a.rs", "status": "modified"}]
            if "/comments" in path:
                return comments or []
            if "/collaborators" in path:
                def rights(**granted):
                    return {level: granted.get(level, False)
                            for level in ("admin", "maintain", "push", "triage", "pull")}
                return [{"login": "drive-owner", "role_name": "write", "permissions": rights(push=True, pull=True)},
                        {"login": "swift-owner", "role_name": "read", "permissions": rights(pull=True)},
                        {"login": "owner", "role_name": "admin", "permissions": rights(admin=True, push=True, pull=True)},
                        {"login": "custom-role", "role_name": "security-lead",
                         "permissions": rights(push=True, pull=True, triage=True)}]
            return []
        return patch.object(self.api, "request", side_effect=request), patch.object(self.api, "pages", side_effect=pages)

    def test_evidence_already_read_for_admission_is_not_read_again(self):
        # collect() reads every candidate's comments and lifecycle in one
        # batched query, and the snapshot then read both a second time over
        # REST: the comments again and the whole issue timeline, which pages.
        comment = {"id": 7, "user": {"login": "author"}, "body": "hello",
                   "created_at": "2026-09-01T00:00:00Z", "updated_at": "2026-09-01T00:00:00Z"}
        request, pages = self.snapshot_fixture(comments=[comment])
        with request, pages:
            fresh = self.api.snapshot(1, {"fallback": ["owner"], "areas": []})
        history = {"comments": fresh["comments"], "lifecycle_at": fresh["lifecycle_at"]}

        # Distinct values, so the assertions below cannot pass by both sides
        # reading the same fixture and agreeing vacuously.
        other = dict(fresh["comments"][0], id=8, body="from the batched read")
        history = {"comments": [other], "lifecycle_at": "2026-09-02T00:00:00Z"}
        request, pages = self.snapshot_fixture(comments=[comment])
        with request, pages:
            reused = self.api.snapshot(1, {"fallback": ["owner"], "areas": []}, history=history)
            read = [call.args[0] for call in self.api.pages.call_args_list]
        self.assertEqual([item["id"] for item in reused["comments"]], [8])
        self.assertEqual(reused["lifecycle_at"], "2026-09-02T00:00:00Z")
        self.assertFalse([path for path in read if "/comments" in path or "/timeline" in path],
                         'neither the comments nor the timeline may be read a second time')

    def test_the_batched_reader_and_the_per_pull_request_reader_agree(self):
        """Both readers feed the same fingerprint, so their shapes must match.

        The batched query names a bot `coderabbitai` where REST names it
        `coderabbitai[bot]`. If those ever stop agreeing, the evidence
        fingerprint differs from itself between one read and the next and every
        ready pull request reports that its evidence changed, for ever.
        """
        raw = {"databaseId": 11, "body": "receipt", "createdAt": "2026-09-01T00:00:00Z",
               "updatedAt": "2026-09-01T00:05:00Z", "lastEditedAt": "2026-09-01T00:05:00Z"}
        graph = {"data": {"repository": {"pr1": {
            "number": 1,
            "comments": {"totalCount": 1, "nodes": [dict(raw, author={"login": "coderabbitai",
                                                                     "__typename": "Bot"})]},
            "timelineItems": {"nodes": []}}}}}
        with patch.object(self.api, "request", side_effect=lambda *a, **k: graph):
            batched = self.api.histories([1])[1]["comments"]
        rest_page = [{"id": 11, "user": {"login": "coderabbitai[bot]"}, "body": "receipt",
                      "created_at": "2026-09-01T00:00:00Z", "updated_at": "2026-09-01T00:05:00Z"}]
        with patch.object(self.api, "pages", side_effect=lambda path: rest_page):
            per_pr = self.api.comments(1)
        # Who last edited a comment, and when, are what the listing cannot
        # say, and it says so. The print leaves both out, so both still agree.
        editless = lambda comments: [{k: v for k, v in c.items() if k not in ("edited_by", "edited_at")}
                                     for c in comments]
        self.assertEqual(editless(per_pr), editless(batched))
        from pr_review.policy import fingerprint
        self.assertEqual(fingerprint({"comments": batched}), fingerprint({"comments": per_pr}))

    def test_comments_read_without_their_editors_are_refused_by_the_record_reader(self):
        # The comments listing names no editor. Read from there, a record a
        # collaborator edited looks like one nobody edited, and their
        # admission or their `ready-for-human` would be believed; so the
        # record reader refuses what this route read rather than guess.
        body = GitHub.state_comment_body(
            {"version": 1, "number": 1, "head": "a" * 40, "admitted_at": "2026-01-01T00:00:00Z",
             "ready_since": None, "state": "ready-for-human", "evidence": "c" * 64, "context": "d" * 64}, "text")
        listing = [{"id": 1, "user": {"login": "github-actions[bot]"}, "body": body,
                    "created_at": "2026-09-11T10:00:00Z", "updated_at": "2026-09-11T12:00:00Z"}]
        with patch.object(self.api, "pages", return_value=listing):
            rest = self.api.comments(1)
        with self.assertRaises(GitHubError):
            parse_controller_state(rest)

    def test_should_refuse_truncated_changed_files(self):
        request, pages = self.snapshot_fixture(files=[])
        with request, pages, self.assertRaises(GitHubError):
            self.api.snapshot(1, {"fallback": ["owner"], "areas": []})

    def test_should_ignore_copied_controller_markers_and_reject_duplicate_trusted_state(self):
        state = {"version": 1, "number": 1, "head": "a" * 40, "admitted_at": None,
                 "ready_since": None, "state": "too-many-open-prs", "evidence": "b" * 64, "context": "c" * 64}
        marker = '<!-- platform-pr-review-state-v1 ' + json.dumps(state) + ' -->'
        def comment(number, actor):
            return {"id": number, "user": {"login": actor}, "body": marker,
                    "created_at": "2026-09-01T00:00:00Z", "updated_at": "2026-09-01T00:00:00Z"}
        request, pages = self.snapshot_fixture(comments=[comment(1, "author"), comment(2, "github-actions[bot]")])
        with request, pages:
            result = self.api.snapshot(1, {"fallback": ["owner"], "areas": []})
        self.assertEqual(result["controller_state"], state)
        self.assertEqual(result["controller_comment_id"], 2)
        # Every announcement of a move carries the record as of then, so the
        # newest is the current one. Two runs writing at once still agree.
        request, pages = self.snapshot_fixture(comments=[comment(3, "github-actions[bot]"), comment(2, "github-actions[bot]")])
        with request, pages:
            result = self.api.snapshot(1, {"fallback": ["owner"], "areas": []})
        self.assertEqual(result["controller_comment_id"], 3)

    def test_the_newest_state_comment_wins_however_the_page_is_ordered(self):
        def state(number):
            return {"version": 1, "number": 1, "head": "a" * 40, "admitted_at": None,
                    "ready_since": None, "state": "too-many-open-prs",
                    "evidence": str(number) * 64, "context": "c" * 64}

        def comment(number, created_at):
            body = '<!-- platform-pr-review-state-v1 ' + json.dumps(state(number)) + ' -->'
            # parse_controller_state reads normalised comments, where the author
            # is already a login rather than the API's nested user object.
            return {"id": number, "user": "github-actions[bot]", "body": body,
                    "created_at": created_at, "updated_at": created_at}

        earlier = comment(9, "2026-09-01T00:00:00Z")
        later = comment(4, "2026-09-02T00:00:00Z")
        for page in ([earlier, later], [later, earlier]):
            self.assertEqual(parse_controller_state(page)[1], 4, 'latest comment, not highest id')
        # GitHub timestamps are whole seconds, so simultaneous writes can tie.
        # Without a second key two runs could each keep a different comment.
        tied = [comment(7, "2026-09-01T00:00:00Z"), comment(5, "2026-09-01T00:00:00Z")]
        self.assertEqual(parse_controller_state(tied)[1], 7)
        self.assertEqual(parse_controller_state(list(reversed(tied)))[1], 7)

    def test_a_pull_request_that_vanished_mid_run_does_not_fail_the_others(self):
        """`gh` exits non-zero whenever GraphQL answers with any errors array.

        A pull request closed between listing the queue and reading its history
        answers null for its own alias while every other alias answers normally.
        Treating that exit code as a failure aborted the whole reconciliation,
        and the handler for it posts an error status on every pull request in
        scope, so one person closing a pull request marked the rest red.
        """
        body = json.dumps({
            "data": {"repository": {
                "pr1": {"number": 1, "comments": {"totalCount": 0, "nodes": []},
                        "timelineItems": {"nodes": []}},
                "pr2": None}},
            "errors": [{"type": "NOT_FOUND", "path": ["repository", "pr2"],
                        "message": "Could not resolve to a PullRequest with the number of 2."}]})
        completed = subprocess.CompletedProcess(args=[], returncode=1, stdout=body, stderr="not found")
        with patch.object(subprocess, "run", return_value=completed):
            histories = self.api.histories([1, 2])
        self.assertEqual(sorted(histories), [1], 'the surviving pull request still has its history')

    def test_a_failing_rest_call_is_never_relaxed(self):
        # The relaxation is keyed on the command being a GraphQL one. Without
        # that, every failing REST read whose body happens to carry a data key
        # would be treated as a success.
        completed = subprocess.CompletedProcess(
            args=[], returncode=1, stdout='{"data": {"whatever": 1}}', stderr="not found")
        with patch.object(subprocess, "run", return_value=completed):
            with self.assertRaises(GitHubError):
                self.api.request("GET", "repos/dashpay/platform/pulls/1")

    def test_a_partial_failure_that_is_not_a_missing_pull_request_still_fails(self):
        # These reach the caller's filter rather than stopping at the exit code,
        # which is the only behaviour this relaxation actually changes.
        good = {"number": 1, "comments": {"totalCount": 0, "nodes": []}, "timelineItems": {"nodes": []}}
        for errors, label in [
            ([{"type": "FORBIDDEN", "message": "no"}], 'one field refused'),
            ([{"message": "spec-shaped error with no type"}], 'no type at all'),
            ([{"type": "NOT_FOUND"}, {"message": "and something else"}], 'mixed with a real one'),
            ([{"type": "RATE_LIMITED"}], 'rate limited'),
            ("boom", 'errors is not a list'),
            (["boom"], 'errors is not a list of objects'),
        ]:
            body = json.dumps({"data": {"repository": {"pr1": good}}, "errors": errors})
            completed = subprocess.CompletedProcess(args=[], returncode=1, stdout=body, stderr="failed")
            with patch.object(subprocess, "run", return_value=completed):
                with self.assertRaises(GitHubError, msg=label):
                    self.api.histories([1])

    def test_a_real_graphql_failure_is_still_a_failure(self):
        for body, label in [('{"errors":[{"type":"FORBIDDEN","message":"nope"}]}', 'no data'),
                            ('{"data":null,"errors":[{"type":"NOT_FOUND"}]}', 'null data'),
                            ('not json', 'unparseable'),
                            ('', 'empty')]:
            completed = subprocess.CompletedProcess(args=[], returncode=1, stdout=body, stderr="failed")
            with patch.object(subprocess, "run", return_value=completed):
                with self.assertRaises(GitHubError, msg=label):
                    self.api.histories([1])

    def test_should_fail_on_malformed_trusted_state(self):
        comments = [{"id": 2, "user": {"login": "github-actions[bot]"},
                     "body": "<!-- platform-pr-review-state-v1 {oops} -->",
                     "created_at": "2026-09-01T00:00:00Z", "updated_at": "2026-09-01T00:00:00Z"}]
        request, pages = self.snapshot_fixture(comments=comments)
        with request, pages, self.assertRaises(GitHubError):
            self.api.snapshot(1, {"fallback": [], "areas": []})

    def test_should_paginate_graphql_threads_and_reject_stuck_cursor(self):
        node = {"id": "T1", "isResolved": False, "opening": {"nodes": [{"body": "Rename this."}]}, "comments": {"nodes": [
            {"author": {"login": "reviewer"}, "createdAt": "2026-09-01T00:00:00Z"}]}}
        with patch.object(self.api, "request", side_effect=[self.graph([node], True, "c1"), self.graph(total=1)] ) as request:
            self.assertEqual(self.api.threads(1), [{"id": "T1", "is_resolved": False, "author": "reviewer", "created_at": "2026-09-01T00:00:00Z",
                                                   "voices": [{"user": "reviewer", "created_at": "2026-09-01T00:00:00Z"}],
                                                   "severities": []}])
            self.assertEqual(request.call_args.args[2]["variables"]["cursor"], "c1")
        with patch.object(self.api, "request", return_value=self.graph([], True, "same")), self.assertRaises(GitHubError):
            self.api.threads(1)

    def test_thread_whose_comments_were_all_deleted_is_skipped(self):
        live = {"id": "T1", "isResolved": False, "opening": {"nodes": [{"body": "Rename this."}]}, "comments": {"nodes": [
            {"author": {"login": "reviewer"}, "createdAt": "2026-09-01T00:00:00Z"}]}}
        emptied = {"id": "T2", "isResolved": False, "opening": {"nodes": []}, "comments": {"nodes": []}}
        with patch.object(self.api, "request", return_value=self.graph([emptied, live], total=2)):
            self.assertEqual([t["id"] for t in self.api.threads(1)], ["T1"])

    def test_a_thread_carries_everyone_who_spoke_in_it(self):
        # An author's own thread with a reviewer's objection in reply was read
        # as the author's alone, and the objection vanished with it.
        thread = {"id": "T1", "isResolved": False, "opening": {"nodes": [{"body": "Why?"}]}, "comments": {"nodes": [
            {"author": {"login": "author"}, "createdAt": "2026-09-01T00:00:00Z"},
            {"author": {"login": "reviewer"}, "createdAt": "2026-09-02T00:00:00Z"}]}}
        with patch.object(self.api, "request", return_value=self.graph([thread], total=1)):
            (only,) = self.api.threads(1)
        self.assertEqual(only["author"], "author")
        self.assertEqual([v["user"] for v in only["voices"]], ["author", "reviewer"])

    def test_a_thread_carries_the_severity_of_its_findings_not_their_text(self):
        # Whether a bot's thread holds the pull request depends on how it
        # labelled its findings. The text is not kept: CodeRabbit appends to
        # its opening once a finding is addressed, and keeping that would read
        # as review evidence changing underneath a write.
        body = ("_🎯 Functional Correctness_ | _🟡 Minor_ | _⚡ Quick win_\n\n"
                "**Reject an empty identifier.**\n<!-- cr-comment:v1:1 -->")
        def node(text):
            return {"id": "T1", "isResolved": False, "opening": {"nodes": [{"body": text}]}, "comments": {"nodes": [
                {"author": {"login": "coderabbitai"}, "createdAt": "2026-09-01T00:00:00Z"}]}}
        with patch.object(self.api, "request", return_value=self.graph([node(body)], total=1)):
            (only,) = self.api.threads(1)
        self.assertEqual(only["severities"], ["🟡 Minor"])
        self.assertNotIn("body", only)
        addressed = body + "\n\n✅ Addressed in commit 1a2b3c4"
        with patch.object(self.api, "request", return_value=self.graph([node(addressed)], total=1)):
            self.assertEqual(self.api.threads(1), [only])

    def test_a_thread_whose_opening_did_not_arrive_is_refused(self):
        # Read as no label it would hold the pull request; read as anything
        # else it could let a blocker through. Neither is an answer.
        thread = {"id": "T1", "isResolved": False, "comments": {"nodes": [
            {"author": {"login": "thepastaclaw"}, "createdAt": "2026-09-01T00:00:00Z"}]}}
        for opening in (None, {"nodes": None}, {"nodes": []}, {"nodes": [{"body": None}]}):
            with self.subTest(opening=opening):
                broken = dict(thread) if opening is None else dict(thread, opening=opening)
                with patch.object(self.api, "request", return_value=self.graph([broken], total=1)), \
                        self.assertRaises(GitHubError):
                    self.api.threads(1)

    def test_should_refuse_truncated_thread_connection(self):
        with patch.object(self.api, "request", return_value=self.graph(total=1)), self.assertRaises(GitHubError):
            self.api.threads(1)

    def test_head_seen_at_is_the_earliest_status_this_controller_wrote(self):
        ours = {"context": "PR Hygiene", "creator": {"login": "github-actions[bot]"}}
        pages = [dict(ours, created_at="2026-09-14T09:35:13Z"),
                 dict(ours, created_at="2026-09-13T08:43:23Z"),
                 {"context": "CodeRabbit", "creator": {"login": "coderabbitai[bot]"}, "created_at": "2026-01-01T00:00:00Z"},
                 {"context": "PR Hygiene", "creator": {"login": "impostor"}, "created_at": "2020-01-01T00:00:00Z"}]
        with patch.object(self.api, "pages", return_value=pages) as paged:
            self.assertEqual(self.api.head_seen_at("a" * 40), "2026-09-13T08:43:23Z")
            self.assertEqual(self.api.head_seen_at("a" * 40), "2026-09-13T08:43:23Z")
            # A commit's own status history is read once per reconciliation.
            self.assertEqual(paged.call_count, 1)
        self.api.forget_cached_access()
        with patch.object(self.api, "pages", return_value=[]):
            self.assertIsNone(self.api.head_seen_at("a" * 40))
        self.api.forget_cached_access()
        with patch.object(self.api, "pages", return_value=[dict(ours, created_at="whenever")]):
            with self.assertRaises(GitHubError):
                self.api.head_seen_at("a" * 40)

    def history_response(self, **overrides):
        comment = {"databaseId": 7, "body": "hello", "createdAt": "2026-09-11T10:00:00Z",
                   "updatedAt": "2026-09-11T10:00:00Z", "lastEditedAt": None,
                   "author": {"login": "github-actions", "__typename": "Bot"}}
        node = {"number": 1, "comments": {"totalCount": 1, "nodes": [comment]},
                "timelineItems": {"nodes": [{"createdAt": "2026-09-04T00:00:00Z"}]}}
        node.update(overrides)
        return {"data": {"repository": {"pr1": node}}}

    def test_batched_history_restores_the_bot_suffix_graphql_omits(self):
        # REST says github-actions[bot]; GraphQL says github-actions. The
        # controller finds its own record by that login, so a bare one would
        # make its own state invisible and duplicate the report.
        with patch.object(self.api, "request", return_value=self.history_response()):
            history = self.api.histories([1])
        self.assertEqual(history[1]["comments"][0]["user"], "github-actions[bot]")
        self.assertEqual(history[1]["comments"][0]["id"], 7)
        self.assertEqual(history[1]["lifecycle_at"], "2026-09-04T00:00:00Z")

    def test_batched_history_treats_a_vanished_pull_request_as_gone_not_broken(self):
        response = {"data": {"repository": {"pr1": None, "pr2": self.history_response()["data"]["repository"]["pr1"]}},
                    "errors": [{"type": "NOT_FOUND", "path": ["repository", "pr1"]}]}
        with patch.object(self.api, "request", return_value=response):
            history = self.api.histories([1, 2])
        self.assertEqual(sorted(history), [2])

    def test_batched_history_still_fails_on_a_real_error(self):
        response = {"data": {"repository": {}}, "errors": [{"type": "RATE_LIMITED"}]}
        with patch.object(self.api, "request", return_value=response), self.assertRaises(GitHubError):
            self.api.histories([1])

    @staticmethod
    def record_node(number, editor, edited_at="2026-09-11T10:00:00Z", **state):
        """This controller's record as GraphQL answers it, last edited by `editor`."""
        record = {"version": 1, "number": 1, "head": "a" * 40, "admitted_at": "2026-09-01T10:00:00Z",
                  "ready_since": None, "state": "waiting-bots", "evidence": "c" * 64, "context": "d" * 64}
        return {"databaseId": number, "body": GitHub.state_comment_body(dict(record, **state), "text"),
                "createdAt": "2026-09-01T10:00:00Z", "updatedAt": edited_at or "2026-09-01T10:00:00Z",
                "lastEditedAt": edited_at, "author": {"login": "github-actions", "__typename": "Bot"},
                "editor": editor}

    @classmethod
    def long_comment(cls, number):
        # The first comment is this controller's record, since refreshed by it.
        if number == 1:
            return cls.record_node(1, {"login": "github-actions", "__typename": "Bot"})
        return {"databaseId": number, "body": f"comment {number}", "createdAt": "2026-09-02T00:00:00Z",
                "updatedAt": "2026-09-02T00:00:00Z", "lastEditedAt": None,
                "author": {"login": "someone", "__typename": "User"}, "editor": None}

    @staticmethod
    def comment_page(nodes, total, cursor=None):
        return {"data": {"repository": {"pullRequest": {"comments": {
            "totalCount": total, "pageInfo": {"hasNextPage": cursor is not None, "endCursor": cursor},
            "nodes": nodes}}}}}

    def read_long(self, pages, total=150):
        """The history of a pull request whose comments outgrow the batched window.

        `pages` answers each cursor; the comments listing answers the same
        comments as GitHub's REST route does, naming no editor.
        """
        everything = [self.long_comment(n) for n in range(1, total + 1)]
        window = self.history_response(comments={"totalCount": total, "nodes": everything[-100:]})
        listing = [{"id": c["databaseId"], "body": c["body"], "created_at": c["createdAt"], "updated_at": c["updatedAt"],
                    "user": {"login": c["author"]["login"] + ("[bot]" if c["author"]["__typename"] == "Bot" else "")}}
                   for c in everything]

        def answer(method, path, payload=None):
            return window if "fragment history" in payload["query"] else pages[payload["variables"]["after"]]
        with patch.object(self.api, "request", side_effect=answer), \
                patch.object(self.api, "pages", return_value=listing):
            return self.api.histories([1])

    def test_both_comment_queries_ask_who_edited_and_when(self):
        # Whether a comment is still this controller's own words is decided
        # by whether GitHub records an edit and whom it names as the editor.
        # A query that stopped asking would read every forged record as one
        # nobody touched.
        everything = [self.long_comment(n) for n in range(1, 151)]
        window = self.history_response(comments={"totalCount": 150, "nodes": everything[-100:]})
        pages = {None: self.comment_page(everything[:100], 150, "c1"), "c1": self.comment_page(everything[100:], 150)}
        asked = []

        def answer(method, path, payload=None):
            asked.append(" ".join(payload["query"].split()))
            return window if "fragment history" in payload["query"] else pages[payload["variables"]["after"]]
        with patch.object(self.api, "request", side_effect=answer):
            self.api.histories([1])
        self.assertEqual(len(asked), 3, "the batched query and both pages")
        for query in asked:
            self.assertIn("lastEditedAt", query)
            self.assertIn("editor { login __typename }", query)

    def test_a_forged_record_on_a_later_page_is_the_newest_and_nothing_is_read(self):
        # The editor of a comment on the second page is read as surely as on
        # the first, and the forged record there, being the newest, decides:
        # the genuine one on the first page does not stand in for it.
        everything = [self.long_comment(n) for n in range(1, 151)]
        everything[119] = self.record_node(120, {"login": "llbartekll", "__typename": "User"}, "2026-09-12T00:00:00Z",
                                           admitted_at="2020-01-01T00:00:00Z", state="ready-for-human")
        comments = self.read_long({None: self.comment_page(everything[:100], 150, "c1"),
                                   "c1": self.comment_page(everything[100:], 150)})[1]["comments"]
        self.assertEqual(comments[119]["edited_by"], "llbartekll")
        self.assertEqual(parse_controller_state(comments), (None, None))

    def test_a_record_somebody_else_edited_is_not_trusted_through_the_batched_query(self):
        # A person's edit names them. A deleted or suspended account's edit is
        # still recorded, but its editor answers null, beside a NOT_FOUND
        # error the history read tolerates; read as never edited, a forged
        # admission would stand.
        for editor in ({"login": "llbartekll", "__typename": "User"}, None):
            node = self.record_node(7, editor, "2026-09-12T00:00:00Z",
                                    admitted_at="2020-01-01T00:00:00Z", state="ready-for-human")
            response = self.history_response(comments={"totalCount": 1, "nodes": [node]})
            response["errors"] = [{"type": "NOT_FOUND", "path": ["repository", "pr1", "comments", "nodes", 0, "editor"]}]
            with patch.object(self.api, "request", return_value=response):
                comments = self.api.histories([1])[1]["comments"]
            self.assertEqual(parse_controller_state(comments), (None, None), repr(editor))

    def test_a_long_conversation_is_read_page_by_page_with_its_editors(self):
        # The controller's own record can be older than the window the batched
        # query asks for, and it is rewritten in place, so it is believed only
        # once its editor is known. The comments listing names no editor:
        # read from there, a record this controller refreshed is ignored and
        # the pull request loses its slot, and a hundred comments would hide
        # anybody's edit of it.
        everything = [self.long_comment(n) for n in range(1, 151)]
        history = self.read_long({None: self.comment_page(everything[:100], 150, "c1"),
                                  "c1": self.comment_page(everything[100:], 150)})
        comments = history[1]["comments"]
        self.assertEqual([c["id"] for c in comments], list(range(1, 151)))
        self.assertEqual(comments[0]["edited_by"], "github-actions[bot]")
        state, comment_id = parse_controller_state(comments)
        self.assertEqual((comment_id, state["admitted_at"]), (1, "2026-09-01T10:00:00Z"))

    def test_a_long_conversation_read_in_pieces_that_do_not_add_up_is_refused(self):
        # Pages short of what they must carry, or holding more comments than
        # the conversation has (one deleted from a page already read), are
        # not the whole conversation, and the record may be the part missing.
        everything = [self.long_comment(n) for n in range(1, 152)]
        first = self.comment_page(everything[:100], 150, "c1")
        # A comment posted during the read lands on the last page and is
        # counted by its total: that read is whole.
        added = self.read_long({None: first, "c1": self.comment_page(everything[100:], 151)})
        self.assertEqual(len(added[1]["comments"]), 151)
        # Deleted during the read: gone, as the batched query treats it.
        gone = {"data": {"repository": {"pullRequest": None}}, "errors": [{"type": "NOT_FOUND"}]}
        self.assertEqual(self.read_long({None: first, "c1": gone}), {})
        no_page_info = self.comment_page(everything[100:150], 150)
        del no_page_info["data"]["repository"]["pullRequest"]["comments"]["pageInfo"]
        no_cursor = self.comment_page(everything[:100], 150, "c1")
        no_cursor["data"]["repository"]["pullRequest"]["comments"]["pageInfo"]["endCursor"] = None
        for pages, label in [
                ({None: first, "c1": no_page_info}, 'a page without its page information'),
                ({None: no_cursor}, 'more to read and no cursor to read it from'),
                ({None: first, "c1": self.comment_page(everything[100:150], 149)}, 'more comments than there are'),
                ({None: first, "c1": dict(self.comment_page(everything[100:150], 150),
                                          errors=[{"type": "RATE_LIMITED"}])}, 'an error that is not a deletion')]:
            with self.assertRaises(GitHubError, msg=label):
                self.read_long(pages)

    def test_batched_history_of_nothing_asks_nothing(self):
        with patch.object(self.api, "request") as request:
            self.assertEqual(self.api.histories([]), {})
            request.assert_not_called()

    def test_should_recover_latest_inactive_transition_from_timeline(self):
        events = [
            {"event": "closed", "created_at": "2026-09-02T00:00:00Z"},
            {"event": "reopened", "created_at": "2026-09-03T00:00:00Z"},
            {"event": "convert_to_draft", "created_at": "2026-09-04T00:00:00Z"},
            {"event": "ready_for_review", "created_at": "2026-09-05T00:00:00Z"},
        ]
        with patch.object(self.api, "pages", return_value=events) as pages:
            self.assertEqual(self.api.activity(1), "2026-09-04T00:00:00Z")
            self.assertEqual(pages.call_args.args, ("repos/dashpay/platform/issues/1/timeline",))
        with patch.object(self.api, "pages", return_value=[]):
            self.assertIsNone(self.api.activity(1))

    def test_should_not_query_unrelated_roster_permissions(self):
        policy = {"fallback": {"owners": ["fallback"], "reviewers": []}, "areas": [
            {"paths": ["packages/rs-drive/"], "owners": ["drive-owner"], "reviewers": []},
            {"paths": ["packages/swift-sdk/"], "owners": ["swift-owner"], "reviewers": []},
        ]}
        request, pages = self.snapshot_fixture()
        with request as requests, pages:
            result = self.api.snapshot(1, policy)
        self.assertEqual(result["permissions"], {"drive-owner": "write"})
        # One list answers for everyone, instead of a request per person.
        self.assertEqual(sum(call.args[1].endswith("/permission") for call in requests.call_args_list), 0)

    def test_access_reads_one_list_and_distrusts_what_it_does_not_recognise(self):
        _, pages = self.snapshot_fixture()
        with pages as paged:
            access = self.api.access()
            self.api.access()
            self.assertEqual(paged.call_count, 1)
        self.assertEqual(access["drive-owner"], "write")
        self.assertEqual(access["swift-owner"], "read")
        # A custom organisation role has a name this policy never heard of, but
        # its capabilities still say whether the holder can push.
        self.assertEqual(access["custom-role"], "write")
        # Anyone absent from the list has no access at all.
        self.assertNotIn("stranger", access)

    def test_a_file_read_without_its_patch_says_nothing_about_being_the_same_work(self):
        # Line counts are two small integers, and they are equal for the one
        # case this exists to catch: keeping your own side of a conflict
        # changes the patch and not its counts.
        from pr_review.policy import diff_print
        request, pages = self.snapshot_fixture(files=[
            {"filename": "a.rs", "status": "modified", "sha": "c" * 40,
             "additions": 1, "deletions": 1, "changes": 2}])
        with request, pages:
            no_patch = self.api.snapshot(1, {"fallback": ["owner"], "areas": []})
        self.assertNotIn("shape", no_patch["files"][0])
        self.assertIsNone(diff_print(no_patch))
        request, pages = self.snapshot_fixture(files=[
            {"filename": "a.rs", "status": "modified", "sha": "c" * 40, "patch": "@@ -1 +1 @@\n-a\n+b"}])
        with request, pages:
            with_patch = self.api.snapshot(1, {"fallback": ["owner"], "areas": []})
        self.assertIsNotNone(diff_print(with_patch))
        # A file the pull request adds is not in the merge base, so nothing the
        # base did can be hiding in its patch and the blob decides it alone.
        # GitHub sends no patch for a large or binary one, and one such file
        # used to stop the whole pull request carrying anything: 32 of the 69
        # patch-less files across the five repositories are new files, among
        # them the only one in platform#4760 and in #4730.
        request, pages = self.snapshot_fixture(files=[
            {"filename": "golden.bin", "status": "added", "sha": "c" * 40,
             "additions": 0, "deletions": 0, "changes": 0}])
        with request, pages:
            added = self.api.snapshot(1, {"fallback": ["owner"], "areas": []})
        self.assertEqual(added["files"][0]["shape"], "added")
        self.assertIsNotNone(diff_print(added))

    def test_both_reads_of_one_pull_request_see_the_same_comments(self):
        # Three defects in one day were the same shape: a field one read can
        # supply and the other cannot, read by something that decides a
        # verdict. Comments come by one route now — with the editor beside
        # each of them — so the read before the verdict and the read before
        # the write cannot disagree.
        diff = {"number": 1, "diff": "a" * 64, "diff_heads": ["b" * 40], "diff_seen": "2026-09-11T10:00:00Z"}
        body = GitHub.state_comment_body(
            {"version": 1, "number": 1, "head": "b" * 40, "admitted_at": None, "ready_since": None,
             "state": "ready-for-human", "evidence": "c" * 64, "context": "d" * 64}, "text", diff)
        comment = {"id": 7, "user": "github-actions[bot]", "body": body,
                   "created_at": "2026-09-11T10:00:00Z", "updated_at": "2026-09-11T12:00:00Z",
                   "edited_by": "github-actions[bot]"}
        request, pages = self.snapshot_fixture(comments=[comment])
        with request, pages:
            read = self.api.snapshot(1, {"fallback": ["owner"], "areas": []})
            reused = self.api.snapshot(1, {"fallback": ["owner"], "areas": []},
                                       history={"comments": [comment], "lifecycle_at": None})
        # The editor is read with its type, so it carries `[bot]` like an author.
        self.assertEqual(read["comments"][0].get("edited_by"), "github-actions[bot]")
        self.assertEqual(read["controller_diff"], diff)
        self.assertEqual(read["controller_diff"], reused["controller_diff"])

    def test_a_file_that_changed_type_is_listed_twice_and_is_not_drift(self):
        # A regular file becoming a symlink is one path listed twice — removed
        # and added, a different blob each time. platform 8b466abc74ee is
        # CLAUDE.md doing exactly that. Reading it as the pagination shifting
        # under the read left the pull request an error status on a required
        # check, telling whoever looked to go and investigate pagination.
        request, pages = self.snapshot_fixture(files=[
            {"filename": "CLAUDE.md", "status": "removed", "sha": "a" * 40, "patch": "@@ -1 +0,0 @@\n-x"},
            {"filename": "CLAUDE.md", "status": "added", "sha": "b" * 40, "patch": "@@ -0,0 +1 @@\n+docs/x"}])
        with request, pages:
            read = self.api.snapshot(1, {"fallback": ["owner"], "areas": []})
        self.assertEqual([f["status"] for f in read["files"]], ["removed", "added"])
        # Still drift when the same path arrives twice the same way.
        request, pages = self.snapshot_fixture(files=[
            {"filename": "a.rs", "status": "modified", "sha": "a" * 40, "patch": "@@ -1 +1 @@\n-a\n+b"},
            {"filename": "a.rs", "status": "modified", "sha": "a" * 40, "patch": "@@ -1 +1 @@\n-a\n+b"}])
        with request, pages:
            with self.assertRaisesRegex(GitHubError, "Duplicate"):
                self.api.snapshot(1, {"fallback": ["owner"], "areas": []})

    def test_who_the_pull_request_was_handed_to_is_read(self):
        # A hand-over is written down as an assignment, and whoever is holding
        # the pull request is who may say they have read it. Every route that
        # reads a pull request carries it: a field one read supplies and
        # another does not is how three defects in a day began, and the one
        # that omitted it would report that nobody is holding anything.
        request, pages = self.snapshot_fixture()
        with request, pages:
            read = self.api.snapshot(1, {"fallback": ["owner"], "areas": []})
        self.assertEqual(read["assignees"], ["romchornyi"])
        with patch.object(self.api, "request", return_value=self.pr()):
            self.assertEqual(self.api.pull(1)["assignees"], ["romchornyi"])
        with patch.object(self.api, "pages", return_value=[self.pr()]):
            self.assertEqual(self.api.open_prs()[0]["assignees"], ["romchornyi"])

    def test_should_preserve_rename_source_and_reuse_access_until_told_otherwise(self):
        request, pages = self.snapshot_fixture(files=[{"filename": "new/a.rs", "previous_filename": "old/a.rs", "status": "renamed"}])
        with request, pages as paged:
            first = self.api.snapshot(1, {"fallback": ["owner"], "areas": []})
            self.api.snapshot(1, {"fallback": ["owner"], "areas": []})
            # Access is read once per reconciliation rather than once per pull
            # request: repeating it was a large share of this tool's traffic.
            self.assertEqual(sum("/collaborators" in call.args[0] for call in paged.call_args_list), 1)
            # The check made immediately before writing must not trust that.
            self.api.forget_cached_access()
            self.api.snapshot(1, {"fallback": ["owner"], "areas": []})
            self.assertEqual(sum("/collaborators" in call.args[0] for call in paged.call_args_list), 2)
        self.assertEqual(first["files"][0]["previous_filename"], "old/a.rs")
        self.assertTrue(first["complete"])

    def test_exactly_one_state_label_and_nothing_else_is_touched(self):
        with patch.object(self.api, "request") as request:
            self.api.set_state_label(1, "ready-for-human", ["ready-for-human", "bug"])
            self.api.set_state_label(1, "draft", ["bug"])
            self.api.set_state_label(1, "waiting-build", ["bug"])
            self.api.set_state_label(1, "ready-to-merge", ["bug"])
            request.assert_not_called()
            self.api.set_state_label(1, "waiting-bots", ["ready-for-human", "bug", "bot-review-skipped"])
            calls = [(c.args[0], c.args[1].rsplit("/", 1)[-1] if c.args[0] == "DELETE" else c.args[2]["labels"])
                     for c in request.call_args_list]
            self.assertEqual(calls, [("POST", ["waiting-bots"]), ("DELETE", "ready-for-human")],
                             "the new label first, so a refused removal never leaves it label-less; unrelated labels stay")
            request.reset_mock()
            self.api.set_state_label(1, "configuration-error", ["waiting-bots"])
            self.assertEqual([c.args[0] for c in request.call_args_list], ["DELETE"], "a state with no label clears the old one")
            request.reset_mock()
            # An objection and a missing attestation are one move — the author's.
            self.api.set_state_label(1, "waiting-author", [])
            self.assertEqual(request.call_args.args[2]["labels"], ["waiting-self-review"])
            request.reset_mock()
            self.api.set_state_label(1, "too-many-open-prs", [])
            self.assertEqual(request.call_args.args[2]["labels"], ["too-many-open-prs"])
            request.reset_mock()
            # Names this controller used to set are cleared wherever still seen.
            self.api.set_state_label(1, "waiting-bots", ["waiting-bots", "waiting-author", "ready-to-merge"])
            self.assertEqual(sorted(c.args[1].rsplit("/", 1)[-1] for c in request.call_args_list), ["ready-to-merge", "waiting-author"])
    def test_should_reject_unknown_controller_schema(self):
        with self.assertRaises(GitHubError):
            parse_controller_state([{"id": 1, "user": "github-actions[bot]",
                                     "body": '<!-- platform-pr-review-state-v1 {"version":99} -->'}])

    def test_should_avoid_republishing_identical_status(self):
        status = {"context": "PR Hygiene", "state": "success", "description": "ready-to-merge",
                  "target_url": None, "creator": {"login": "github-actions[bot]"}}
        written = {"context": "PR Hygiene", "state": "pending", "created_at": "2026-09-15T00:00:00Z"}
        with patch.object(self.api, "pages", return_value=[status]) as paged, \
                patch.object(self.api, "request", return_value=written) as request:
            self.api.post_status("a" * 40, "success", "ready-to-merge")
            request.assert_not_called()
            self.api.post_status("a" * 40, "pending", "new evidence")
            self.assertEqual(request.call_args.args[0], "POST")
            # A status just written is remembered, so the next write on the same
            # head does not re-read the commit's whole history.
            self.assertEqual(paged.call_count, 1)

    def test_should_refuse_a_status_write_that_was_not_acknowledged(self):
        with patch.object(self.api, "pages", return_value=[]), \
                patch.object(self.api, "request", return_value=None):
            with self.assertRaises(GitHubError):
                self.api.post_status("a" * 40, "success", "ready-to-merge")

class EngineIdentityTests(unittest.TestCase):
    """The engine's memory is recognised by who wrote it, from one list.

    Repositories move to the PR Hygiene App one at a time, and either writer
    must be able to continue the other's records — or a rollback starts
    every pull request over. Listing an identity is the one change needed;
    an unlisted one is still nobody's.
    """

    APP = 'pr-hygiene[bot]'
    NOW = '2026-09-11T10:00:00Z'

    def setUp(self):
        from pr_review import main
        self.main = main
        self.record = main.state_record({}, {'number': 7, 'head': 'a' * 40, 'state': 'waiting-bots',
                                             'admitted_at': '2026-09-01T10:00:00Z', 'ready_since': None},
                                        'c' * 64)
        self.diff = {'number': 7, 'diff': 'd' * 64, 'diff_heads': ['a' * 40], 'receipts': {}}

    def comments(self, user):
        body = GitHub.state_comment_body(self.record, 'x', self.diff)
        return [dict(id=1, user=user, created_at=self.NOW, updated_at=self.NOW, body=body)]

    def listed(self):
        from pr_review import policy
        return patch.object(policy, 'ENGINE_LOGINS', frozenset({'github-actions[bot]', self.APP}))

    def test_a_listed_identity_continues_the_record(self):
        from pr_review.github import parse_controller_diff
        from pr_review.policy import nudged_at
        with self.listed():
            state, comment_id = parse_controller_state(self.comments(self.APP))
            self.assertEqual((state['admitted_at'], comment_id), ('2026-09-01T10:00:00Z', 1))
            self.assertIsNotNone(parse_controller_diff(self.comments(self.APP), 7))
            self.assertEqual(len(self.main.bot_comments({'number': 7, 'comments': self.comments(self.APP)},
                                                        self.main.STATE_MARKER)), 1)
            nudge = [dict(id=2, user=self.APP, created_at=self.NOW, updated_at=self.NOW,
                          body=f"{self.main.NUDGE_MARKER} bot=coderabbitai sha={'a' * 40} -->")]
            self.assertEqual(nudged_at(nudge, 'coderabbitai', 'a' * 40), self.NOW)

    def test_the_list_is_lowercase(self):
        from pr_review.policy import ENGINE_LOGINS
        self.assertEqual(ENGINE_LOGINS, {x.lower() for x in ENGINE_LOGINS})

    def test_an_unlisted_identity_is_nobodys(self):
        from pr_review.github import parse_controller_diff
        state, comment_id = parse_controller_state(self.comments(self.APP))
        self.assertIsNone(comment_id)
        self.assertIsNone(parse_controller_diff(self.comments(self.APP), 7))
        self.assertEqual(self.main.bot_comments({'number': 7, 'comments': self.comments(self.APP)},
                                                self.main.STATE_MARKER), [])


    def test_only_the_bot_spelling_counts_in_any_case(self):
        # The bare name is one a person can register; only the `[bot]` login
        # GitHub gives an App is the engine, whatever its case.
        with self.listed():
            self.assertIsNone(parse_controller_state(self.comments('pr-hygiene'))[1])
            self.assertIsNone(parse_controller_state(self.comments('github-actions'))[1])
            self.assertEqual(parse_controller_state(self.comments('PR-Hygiene[bot]'))[1], 1)

    def test_a_status_by_a_listed_identity_is_the_engines_own(self):
        api = GitHub('dashpay/platform')
        status = {'state': 'pending', 'context': 'PR Hygiene', 'description': 'ready-for-human',
                  'created_at': self.NOW, 'creator': {'login': self.APP}}
        with self.listed(), patch.object(api, '_head_statuses', return_value=[status]):
            self.assertTrue(api.ready_published('a' * 40))
            self.assertEqual(api.head_seen_at('a' * 40), self.NOW)
        with patch.object(api, '_head_statuses', return_value=[status]):
            self.assertFalse(api.ready_published('a' * 40))


if __name__ == "__main__":
    unittest.main()
