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
                "title": "Example", "changed_files": 1, "requested_reviewers": [], "labels": []}

    def graph(self, nodes=None, more=False, cursor=None, total=None):
        return {"data": {"repository": {"pullRequest": {"reviewThreads": {
            "totalCount": len(nodes or []) if total is None else total,
            "nodes": nodes or [], "pageInfo": {"hasNextPage": more, "endCursor": cursor}}}}}}

    def checks(self, nodes=None, head="a" * 40, total=None, more=False, cursor=None):
        return rollup(nodes, head, total, more, cursor)

    def snapshot_fixture(self, comments=None, files=None, graph=None, checks=None):
        def request(method, path, payload=None):
            if path == "graphql":
                # One fixture answers two queries; they are told apart the same
                # way the engine tells them apart — by what was asked for.
                if "statusCheckRollup" in (payload or {}).get("query", ""):
                    return checks or self.checks()
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
               "updatedAt": "2026-09-01T00:05:00Z"}
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
        self.assertEqual(batched, per_pr)

    def test_should_refuse_truncated_changed_files(self):
        request, pages = self.snapshot_fixture(files=[])
        with request, pages, self.assertRaises(GitHubError):
            self.api.snapshot(1, {"fallback": ["owner"], "areas": []})

    def test_should_ignore_copied_controller_markers_and_reject_duplicate_trusted_state(self):
        state = {"version": 1, "number": 1, "head": "a" * 40, "admitted_at": None,
                 "ready_since": None, "state": "waiting-slot", "evidence": "b" * 64, "context": "c" * 64}
        marker = '<!-- platform-pr-review-state-v1 ' + json.dumps(state) + ' -->'
        def comment(number, actor):
            return {"id": number, "user": {"login": actor}, "body": marker,
                    "created_at": "2026-09-01T00:00:00Z", "updated_at": "2026-09-01T00:00:00Z"}
        request, pages = self.snapshot_fixture(comments=[comment(1, "author"), comment(2, "github-actions[bot]")])
        with request, pages:
            result = self.api.snapshot(1, {"fallback": ["owner"], "areas": []})
        self.assertEqual(result["controller_state"], state)
        self.assertEqual(result["controller_comment_id"], 2)
        # Two runs reconciling one pull request at once can each open a state
        # comment. Refusing to read them left every pull request by that author
        # on an error status until somebody deleted one by hand, and no run
        # could clear it. The oldest is authoritative and every later run agrees.
        request, pages = self.snapshot_fixture(comments=[comment(3, "github-actions[bot]"), comment(2, "github-actions[bot]")])
        with request, pages:
            result = self.api.snapshot(1, {"fallback": ["owner"], "areas": []})
        self.assertEqual(result["controller_comment_id"], 2)

    def test_the_oldest_state_comment_wins_however_the_page_is_ordered(self):
        def state(number):
            return {"version": 1, "number": 1, "head": "a" * 40, "admitted_at": None,
                    "ready_since": None, "state": "waiting-slot",
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
            self.assertEqual(parse_controller_state(page)[1], 9, 'earliest comment, not lowest id')
        # GitHub timestamps are whole seconds, so simultaneous writes can tie.
        # Without a second key two runs could each keep a different comment.
        tied = [comment(7, "2026-09-01T00:00:00Z"), comment(5, "2026-09-01T00:00:00Z")]
        self.assertEqual(parse_controller_state(tied)[1], 5)
        self.assertEqual(parse_controller_state(list(reversed(tied)))[1], 5)

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
        node = {"id": "T1", "isResolved": False, "comments": {"nodes": [
            {"author": {"login": "reviewer"}, "createdAt": "2026-09-01T00:00:00Z"}]}}
        with patch.object(self.api, "request", side_effect=[self.graph([node], True, "c1"), self.graph(total=1)] ) as request:
            self.assertEqual(self.api.threads(1), [{"id": "T1", "is_resolved": False, "author": "reviewer", "created_at": "2026-09-01T00:00:00Z"}])
            self.assertEqual(request.call_args.args[2]["variables"]["cursor"], "c1")
        with patch.object(self.api, "request", return_value=self.graph([], True, "same")), self.assertRaises(GitHubError):
            self.api.threads(1)

    def test_thread_whose_comments_were_all_deleted_is_skipped(self):
        live = {"id": "T1", "isResolved": False, "comments": {"nodes": [
            {"author": {"login": "reviewer"}, "createdAt": "2026-09-01T00:00:00Z"}]}}
        emptied = {"id": "T2", "isResolved": False, "comments": {"nodes": []}}
        with patch.object(self.api, "request", return_value=self.graph([emptied, live], total=2)):
            self.assertEqual([t["id"] for t in self.api.threads(1)], ["T1"])
        overfull = dict(live, comments={"nodes": [live["comments"]["nodes"][0]] * 2})
        with patch.object(self.api, "request", return_value=self.graph([overfull], total=1)), self.assertRaises(GitHubError):
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
                   "updatedAt": "2026-09-11T10:00:00Z",
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

    def test_batched_history_reads_a_long_conversation_the_slow_way(self):
        # The controller's own record can be older than the window we ask for.
        truncated = self.history_response(comments={"totalCount": 250, "nodes": []})
        with patch.object(self.api, "request", return_value=truncated):
            with patch.object(self.api, "comments", return_value=[{"id": 1, "user": "u", "body": "b",
                                                                   "created_at": "x", "updated_at": "x"}]) as rest:
                history = self.api.histories([1])
        rest.assert_called_once_with(1)
        self.assertEqual(history[1]["comments"][0]["id"], 1)

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

    def test_should_only_mutate_requested_label_delta(self):
        with patch.object(self.api, "request") as request:
            self.api.set_ready_label(1, True, ["ready-for-human", "bug"])
            self.api.set_ready_label(1, False, ["bug"])
            request.assert_not_called()
            self.api.set_ready_label(1, False, ["ready-for-human", "bug"])
            self.assertEqual(request.call_args.args[:2], ("DELETE", "repos/dashpay/platform/issues/1/labels/ready-for-human"))

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


if __name__ == "__main__":
    unittest.main()
