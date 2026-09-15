"""GitHub evidence must be complete before the policy can make a decision."""

import json
import subprocess
import unittest
from unittest.mock import patch

from pr_review.github import GitHub, GitHubError, parse_controller_state


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

    def snapshot_fixture(self, comments=None, files=None, graph=None):
        def request(method, path, payload=None):
            if path == "graphql":
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
                return [{"login": "drive-owner", "role_name": "write"},
                        {"login": "swift-owner", "role_name": "read"},
                        {"login": "owner", "role_name": "admin"},
                        {"login": "custom-role", "role_name": "security-lead"}]
            return []
        return patch.object(self.api, "request", side_effect=request), patch.object(self.api, "pages", side_effect=pages)

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
        request, pages = self.snapshot_fixture(comments=[comment(2, "github-actions[bot]"), comment(3, "github-actions[bot]")])
        with request, pages, self.assertRaises(GitHubError):
            self.api.snapshot(1, {"fallback": ["owner"], "areas": []})

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
        # A custom organisation role means nothing to this policy: assume nothing.
        self.assertEqual(access["custom-role"], "none")
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
