"""Read complete GitHub evidence and apply explicit review-workflow effects."""

import hashlib
import json
import re
import subprocess
import time

from .policy import CHECKLIST_END, CHECKLIST_START, LABEL_FOR_STATE, RETIRED_LABELS, STATE_LABELS
import sys
from datetime import datetime
from urllib.parse import quote


class GitHubError(RuntimeError):
    """GitHub evidence or a requested effect could not be verified."""


STATE_MARKER = "<!-- platform-pr-review-state-v1"
STATE_PATTERN = re.compile(r"<!-- platform-pr-review-state-v1 (\{[^\r\n]*\}) -->")
# The diff this pull request carries, beside the record rather than inside it.
# The record's schema is an exact set of keys, so an engine that predates a new
# one would refuse a record carrying it and report a configuration error on
# every pull request in its repository until it is re-pinned. A marker of its
# own is simply not read by an engine that does not know it.
DIFF_MARKER = "<!-- pr-hygiene-diff-v1"
DIFF_PATTERN = re.compile(r"<!-- pr-hygiene-diff-v1 (\{[^\r\n]*\}) -->")
BOT_LOGINS = {"github-actions[bot]", "coderabbitai[bot]", "coderabbitai", "thepastaclaw"}


def _validate_state(state):
    keys = {"version", "number", "head", "admitted_at", "ready_since", "state", "evidence", "context"}
    if not isinstance(state, dict) or set(state) != keys or type(state["version"]) is not int or state["version"] != 1:
        raise GitHubError("Unknown or incomplete controller state schema")
    if type(state["number"]) is not int or state["number"] < 1:
        raise GitHubError("Invalid controller PR number")
    for key, length in (("head", 40), ("evidence", 64), ("context", 64)):
        if not isinstance(state[key], str) or not re.fullmatch(r"[0-9a-f]{" + str(length) + r"}", state[key]):
            raise GitHubError("Invalid controller " + key)
    if not isinstance(state["state"], str) or not state["state"]:
        raise GitHubError("Missing controller lifecycle state")
    for key in ("admitted_at", "ready_since"):
        if state[key] is not None:
            try:
                if not isinstance(state[key], str) or datetime.fromisoformat(state[key].replace("Z", "+00:00")).tzinfo is None:
                    raise ValueError("Timestamp requires a timezone")
            except ValueError as error:
                raise GitHubError("Invalid controller " + key) from error


# GraphQL spells these uppercase; the REST spellings are a different API and
# would be dead entries here. CANCELLED and STALE carry no verdict at all — a
# cancelled attempt says nothing about the code, and this controller is the
# most-cancelled check on these repositories by a wide margin. ACTION_REQUIRED
# is a conclusion, not a status: the check has finished and wants a human, so
# it is failing, not running.
BUILD_FAILED = {"FAILURE", "TIMED_OUT", "STARTUP_FAILURE", "ACTION_REQUIRED", "ERROR"}
BUILD_PASSED = {"SUCCESS", "SKIPPED", "NEUTRAL", "EXPECTED"}
BUILD_NO_VERDICT = {"CANCELLED", "STALE"}
CALLER_WORKFLOW = "/pr-review-policy.yml"


def _ours(node):
    """Whether a check run is this controller reviewing the pull request.

    It is a check on the pull requests it governs, so its own result is in the
    rollup and it cannot wait for itself. Matched on the caller workflow, which
    the engine refuses to run from any other path, rather than on the job name,
    which each repository is free to rename. An Actions job is distinguished
    from a check run some other tool filed into the same suite, which GitHub
    attributes to this workflow too.
    """
    suite = ((node.get("checkSuite") or {}).get("workflowRun") or {}).get("workflow") or {}
    details = node.get("detailsUrl") or ""
    return (suite.get("resourcePath") or "").endswith(CALLER_WORKFLOW) and "/actions/runs/" in details and "/job/" in details


def build_verdict(nodes):
    """`green`, `running` or `failed` over a head's checks.

    Only the newest run of each check counts. Re-running a check does not
    replace the run it repeats, it adds another beside it, so a head keeps every
    failed and cancelled attempt for ever — GitHub's own rollup reads FAILURE on
    pull requests whose every check has since passed. Without this a flaky test
    could never be cleared by re-running it.

    A result this code does not recognise is unfinished, never green: a gate
    that treats the unknown as a pass is not a gate.
    """
    latest = {}
    for node in nodes:
        if not isinstance(node, dict):
            raise GitHubError("Incomplete check list")
        if node.get("__typename") == "CheckRun":
            if _ours(node):
                continue
            suite = ((node.get("checkSuite") or {}).get("workflowRun") or {}).get("workflow") or {}
            # Keyed by workflow as well as name, so a job called `build` in two
            # workflows cannot stand in for the other.
            key = ("check", suite.get("resourcePath") or "", _text(node["name"], "check name"))
            when, state = node.get("startedAt") or "", node.get("conclusion") or node.get("status")
        else:
            if node.get("context") == "PR Hygiene":
                continue
            key = ("status", "", _text(node["context"], "status context"))
            when, state = node.get("createdAt") or "", node.get("state")
        # A cancelled attempt is not a result. Letting one overwrite an older
        # verdict would turn a failure green by cancelling its re-run.
        if state in BUILD_NO_VERDICT:
            continue
        if key not in latest or when >= latest[key][0]:
            latest[key] = (when, state)
    states = [state for _, state in latest.values()]
    if any(state in BUILD_FAILED for state in states):
        return "failed"
    if any(state not in BUILD_PASSED for state in states):
        return "running"
    return "green"


def _capability(granted):
    """A collaborator's level from the capability flags, highest first.

    The flags are read rather than the role's name because a custom
    organisation role carries a name this policy has never heard of but still
    says plainly whether its holder can push.
    """
    for level in ("admin", "maintain", "push", "triage", "pull"):
        if granted.get(level) is True:
            return {"push": "write", "pull": "read"}.get(level, level)
    return None


IDEMPOTENT = {"GET", "PUT", "PATCH", "DELETE"}


def _transient(arguments, result):
    """A failure worth one more try: an idempotent call, and an answer that was not a refusal."""
    method = arguments[arguments.index("--method") + 1] if "--method" in arguments else "GET"
    if method not in IDEMPOTENT:
        return False
    text = (result.stderr or "") + (result.stdout or "")
    return any(sign in text for sign in ("unexpected end of JSON input", "502", "503", "504", "timeout", "EOF"))


def _normalise(text):
    return text.replace("\r\n", "\n").rstrip()


def _outside_fences(body):
    """Character offsets of `body` that are not inside a fenced code block."""
    inside, offset, spans = False, 0, []
    for line in body.splitlines(keepends=True):
        if line.lstrip().startswith("```") or line.lstrip().startswith("~~~"):
            inside = not inside
        elif not inside:
            spans.append((offset, offset + len(line)))
        offset += len(line)
    return spans


def _split_checklist(body):
    """(author's text, block, trailing text) around this controller's block, or (body, None, '').

    The block is the LAST start marker outside a fenced code block, up to the
    FIRST end marker after it. A quoted example in a fence is not a block; an
    extra end marker further down is the author's, and stays; a start with no
    end after it is no block at all.
    """
    starts = [s for lo, hi in _outside_fences(body)
              for s in [body.find(CHECKLIST_START, lo, hi)] if s != -1]
    if not starts:
        return body, None, ""
    start = max(starts)
    end = body.find(CHECKLIST_END, start + len(CHECKLIST_START))
    if end == -1:
        return body, None, ""
    end += len(CHECKLIST_END)
    return body[:start], body[start:end], body[end:]


def current_checklist(body):
    """The block a description carries now, or None."""
    return _split_checklist(body or "")[1]


def _validate_diff(diff):
    # What is there is checked; what is not there is simply not read. A field
    # added later must not make this refuse the whole marker, or every pull
    # request loses what the controller remembered about it the moment one
    # repository runs a newer engine than another.
    if not isinstance(diff, dict) or "number" not in diff or set(diff) - {
            "number", "diff", "diff_heads", "diff_seen", "receipts"}:
        raise GitHubError("Unknown or incomplete controller diff schema")
    if type(diff["number"]) is not int or diff["number"] < 1:
        raise GitHubError("Invalid controller diff PR number")
    if "diff" in diff and (not isinstance(diff["diff"], str)
                           or not re.fullmatch(r"[0-9a-f]{64}", diff["diff"])):
        raise GitHubError("Invalid controller diff print")
    # Present means at least one: an empty list was accepted once, and it
    # handed whoever wrote it the instant an attestation is measured against.
    heads = diff.get("diff_heads", ["0" * 40])
    if (not isinstance(heads, list) or not 1 <= len(heads) <= 20
            or any(not isinstance(h, str) or not re.fullmatch(r"[0-9a-f]{40}", h) for h in heads)):
        raise GitHubError("Invalid controller diff heads")
    # What each producer said, and when it first said it: a print of its own
    # words against the moment they were first read.
    said = diff.get("receipts", {})
    if (not isinstance(said, dict) or len(said) > 8
            or any(not re.fullmatch(r"[0-9a-f]{64}", k)
                   or not isinstance(v, str)
                   or not re.fullmatch(r"\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}Z", v)
                   for k, v in said.items())):
        raise GitHubError("Invalid controller receipt record")
    # A timestamp, checked as one: it is read back as a time, and a string
    # that is not one raised out of the verdict, which catches no such error,
    # and took the whole repository's run down with it.
    if diff.get("diff_seen") is not None and (
            not isinstance(diff["diff_seen"], str)
            or not re.fullmatch(r"\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}Z", diff["diff_seen"])):
        raise GitHubError("Invalid controller diff timestamp")


def parse_controller_diff(comments, number):
    """The newest recorded diff for this pull request, or None.

    Read beside the record, and never fatal: without it a push is read as new
    work, which is what happened before this was written down at all. Only
    this controller's own unedited words are read.
    """
    found = []
    for comment in comments:
        if comment["user"].lower() != "github-actions[bot]" or DIFF_MARKER not in comment["body"]:
            continue
        # Anyone with write access can edit anyone's comment, and this one
        # says which commits a review still covers — forge it and a stale
        # approval, the only human gate left, counts for code nobody read.
        # Unedited it is this controller's own words; edited, only if this
        # controller is who edited it. Where that cannot be known the pull
        # request starts over, which is what it did before this existed.
        edited = comment.get("updated_at") or comment["created_at"]
        # Both spellings: what comes back is the login, and the "[bot]" suffix
        # is only appended where the reader asked for the type. Refusing the
        # bare one refused this controller's own hand, which rewrites the
        # record on every refresh — so the marker became unreadable the second
        # time it was written, and stayed that way.
        if edited != comment["created_at"] and (comment.get("edited_by") or "").lower() not in {
                "github-actions", "github-actions[bot]"}:
            continue
        # In a comment of this controller's own, beside its record for this
        # same pull request. Any workflow can post as the Actions app, and one
        # that echoes text a person wrote would otherwise carry a marker with
        # it.
        state = list(STATE_PATTERN.finditer(comment["body"]))
        if len(state) != 1:
            continue
        try:
            if json.loads(state[0].group(1)).get("number") != number:
                continue
        except (ValueError, TypeError, AttributeError):
            continue
        matches = list(DIFF_PATTERN.finditer(comment["body"]))
        if len(matches) != 1:
            continue
        try:
            diff = json.loads(matches[0].group(1))
            _validate_diff(diff)
        except (ValueError, TypeError, GitHubError):
            continue
        if diff["number"] != number:
            continue
        found.append((_text(comment.get("updated_at") or comment["created_at"], "comment update time"),
                      comment["id"], diff))
    return max(found, key=lambda item: item[:2])[2] if found else None


def parse_controller_state(comments):
    """Ignore copied receipts; the newest record wins; refuse corrupt history."""
    found = []
    for comment in comments:
        if comment["user"].lower() != "github-actions[bot]":
            continue
        body = comment["body"]
        if STATE_MARKER not in body:
            continue
        matches = list(STATE_PATTERN.finditer(body))
        if len(matches) != 1 or body.count(STATE_MARKER) != 1:
            raise GitHubError("Malformed controller state marker")
        try:
            state = json.loads(matches[0].group(1))
        except (ValueError, TypeError) as error:
            raise GitHubError("Malformed controller state JSON") from error
        _validate_state(state)
        found.append((_text(comment.get("updated_at") or comment["created_at"], "comment update time"),
                      comment["id"], state))
    if not found:
        return (None, None)
    # The record most recently written is the current one: a refresh edits
    # the newest holder in place and every edit bumps updated_at, so whichever
    # comment was written last carries the truth. Admission is carried forward
    # unchanged from record to record, which is what keeps the author's slots
    # stable. GitHub reports whole seconds, so the id breaks a tie.
    written_at, comment_id, state = max(found, key=lambda record: record[:2])
    return state, comment_id


def _text(value, label):
    if not isinstance(value, str) or not value:
        raise GitHubError(f"Missing or invalid {label}")
    return value


def _graphql_login(author):
    """REST reports an app as `name[bot]`; GraphQL reports the bare name.

    The controller finds its own record by that login, so a bare one would make
    its state invisible and it would open a second report on every pull request.
    """
    if not isinstance(author, dict):
        raise GitHubError("Missing comment author")
    login = _text(author.get("login"), "comment author")
    if author.get("__typename") == "Bot" and not login.endswith("[bot]"):
        return login + "[bot]"
    return login


def _login(user):
    if not isinstance(user, dict):
        raise GitHubError("Missing account identity")
    return _text(user.get("login"), "account login")


def _unique(items, key, label):
    values = [item[key] for item in items]
    if len(values) != len(set(values)):
        raise GitHubError(f"Duplicate {label}; pagination may have changed during collection")
    return items


class GitHub:
    def __init__(self, repo):
        if not isinstance(repo, str) or not re.fullmatch(r"[A-Za-z0-9_.-]+/[A-Za-z0-9_.-]+", repo):
            raise GitHubError("Expected repository identity owner/name")
        if any(part in {".", ".."} for part in repo.split("/")):
            raise GitHubError("Invalid repository identity")
        self.repo = repo
        self.root = f"repos/{repo}"
        # Neither a person's access nor a commit's own status history changes
        # under one reconciliation. Asking again for every pull request was a
        # large part of this tool's traffic against the organisation's limit.
        self._permissions = {}
        self._statuses = {}
        self._builds = {}
        self._listed = False

    def _run(self, arguments, payload=None, attempt=1):
        try:
            result = subprocess.run(
                ["gh", "api", *arguments],
                input=json.dumps(payload) if payload is not None else None,
                text=True, capture_output=True, timeout=60, check=False,
            )
        except (OSError, subprocess.TimeoutExpired) as error:
            raise GitHubError("GitHub API command unavailable or timed out") from error
        if result.returncode and attempt == 1 and _transient(arguments, result):
            # One flaky answer marked a pull request an error under a required
            # check until its next event. Ask once more, only where asking
            # twice cannot do anything asking once would not.
            time.sleep(2)
            return self._run(arguments, payload, attempt=2)
        if result.returncode:
            # GraphQL answers with a usable payload and an errors array when
            # only part of a query resolved: one aliased field is null while the
            # rest answer normally. gh reports that as a failure. Hand the
            # payload to the caller, which decides which of those errors it
            # tolerates. Anything without one, and every REST call, still fails.
            if "graphql" in arguments and result.stdout.strip():
                try:
                    body = json.loads(result.stdout)
                except ValueError:
                    body = None
                if isinstance(body, dict) and isinstance(body.get("data"), dict):
                    return body
            detail = result.stderr.strip()[:300]
            raise GitHubError(f"GitHub API command failed (exit {result.returncode})"
                              + (f": {detail}" if detail else ""))
        if not result.stdout.strip():
            return None
        try:
            return json.loads(result.stdout)
        except ValueError as error:
            raise GitHubError("GitHub API returned invalid JSON") from error

    def request(self, method, path, payload=None):
        arguments = ["--method", method, path]
        if payload is not None:
            arguments.extend(["--input", "-"])
        return self._run(arguments, payload)

    def pages(self, path):
        separator = "&" if "?" in path else "?"
        if "per_page=" not in path:
            path += separator + "per_page=100"
        result = self._run(["--method", "GET", path, "--paginate", "--slurp"])
        if not isinstance(result, list) or any(not isinstance(page, list) for page in result):
            raise GitHubError("Expected paginated GitHub list")
        return [item for page in result for item in page]

    @staticmethod
    def _pr(raw):
        try:
            result = {
                "number": raw["number"], "author": _login(raw["user"]),
                "author_is_bot": (raw.get("user") or {}).get("type") == "Bot",
                "body": raw.get("body") or "",
                "labels": [_text(label["name"], "label name") for label in raw.get("labels") or []],
                "head": _text(raw["head"]["sha"], "head SHA"),
                "base": _text(raw["base"]["ref"], "base branch"),
                "base_sha": _text(raw["base"]["sha"], "base SHA"),
                "created_at": _text(raw["created_at"], "PR creation time"),
                "draft": raw["draft"], "state": raw["state"],
                "url": _text(raw["html_url"], "PR URL"), "title": raw["title"],
            }
            if type(result["number"]) is not int or result["number"] < 1 or type(result["draft"]) is not bool:
                raise GitHubError("Invalid PR number or draft state")
            if result["state"] not in {"open", "closed"} or not isinstance(result["title"], str):
                raise GitHubError("Invalid PR state or title")
            return result
        except (KeyError, TypeError) as error:
            raise GitHubError("Incomplete PR identity") from error

    def pull(self, number):
        return self._pr(self.request("GET", f"{self.root}/pulls/{number}"))

    def open_prs(self):
        return _unique([self._pr(raw) for raw in self.pages(f"{self.root}/pulls?state=open")], "number", "PR")

    def comments(self, number):
        try:
            # This route carries no editor, so who rewrote an edited comment
            # is unknown here and the reader treats it as unknown — which
            # costs only CodeRabbit's rate-limit waiver, never a merge. The
            # evidence print must not carry it either, or a pull request read
            # by both routes would look changed between the read and the
            # write on every run and never be written to again.
            result = [{"id": raw["id"], "user": _login(raw["user"]),
                       "body": raw["body"], "created_at": _text(raw["created_at"], "comment creation time"),
                       "updated_at": _text(raw["updated_at"], "comment update time"), "edited_by": None}
                      for raw in self.pages(f"{self.root}/issues/{number}/comments")]
            if any(not isinstance(item["body"], str) or type(item["id"]) is not int for item in result):
                raise GitHubError("Invalid comment identity or body")
            return _unique(result, "id", "comment")
        except (KeyError, TypeError) as error:
            raise GitHubError("Incomplete issue comments") from error

    def activity(self, number):
        """Find slot releases even when close/reopen or draft events were missed."""
        transitions = []
        for event in self.pages(f"{self.root}/issues/{number}/timeline"):
            if not isinstance(event, dict) or not isinstance(event.get("event"), str):
                raise GitHubError("Incomplete issue timeline")
            if event["event"] in {"closed", "convert_to_draft"}:
                value = _text(event.get("created_at"), "inactive lifecycle timestamp")
                try:
                    timestamp = datetime.fromisoformat(value.replace("Z", "+00:00"))
                    if timestamp.tzinfo is None:
                        raise ValueError("Timezone required")
                except ValueError as error:
                    raise GitHubError("Invalid inactive lifecycle timestamp") from error
                transitions.append((timestamp, value))
        return max(transitions)[1] if transitions else None

    def histories(self, numbers):
        """Comments and the latest inactive transition for many pull requests at once.

        Rebuilding an author's slots asked for both per pull request, which was
        most of this tool's traffic. One query answers it for every candidate.
        """
        if not numbers:
            return {}
        owner, repo = self.repo.split("/")
        aliases = {f"pr{number}": number for number in sorted(set(numbers))}
        selections = "\n".join(f"{alias}: pullRequest(number:{number}) {{ ...history }}"
                                for alias, number in aliases.items())
        query = ("query($owner:String!, $repo:String!) { repository(owner:$owner, name:$repo) {"
                 + selections + """ } }
        fragment history on PullRequest {
          number
          comments(last:100) {
            totalCount
            nodes { databaseId body createdAt updatedAt author { login __typename }
                    editor { login } }
          }
          timelineItems(last:1, itemTypes:[CLOSED_EVENT, CONVERT_TO_DRAFT_EVENT]) {
            nodes {
              ... on ClosedEvent { createdAt }
              ... on ConvertToDraftEvent { createdAt }
            }
          }
        }""")
        response = self.request("POST", "graphql", {"query": query, "variables": {"owner": owner, "repo": repo}})
        if not isinstance(response, dict) or not isinstance(response.get("data"), dict):
            raise GitHubError("GraphQL history query failed")
        # A pull request the listing saw but GraphQL can no longer resolve — a
        # deleted one — answers null for its own alias while the rest answer
        # normally. Every other error, and any errors array that is not a list
        # of objects, is a real failure.
        errors = response.get("errors") or []
        if not isinstance(errors, list) or any(not isinstance(error, dict)
                                               or error.get("type") != "NOT_FOUND" for error in errors):
            raise GitHubError("GraphQL history query failed")
        repository = response["data"].get("repository")
        if not isinstance(repository, dict):
            raise GitHubError("GraphQL history query returned no repository")
        histories = {}
        for alias, number in aliases.items():
            node = repository.get(alias)
            if node is None:
                continue
            try:
                connection = node["comments"]
                total = connection["totalCount"]
                nodes = connection["nodes"]
                if type(total) is not int or not isinstance(nodes, list):
                    raise GitHubError("Incomplete comment connection")
                if total > len(nodes):
                    # Older than the window we asked for: read it the slow way
                    # rather than miss this controller's own record.
                    comments = self.comments(number)
                else:
                    # Who last wrote it, not only when: a comment edited by
                    # somebody other than its author is that person speaking.
                    comments = [{"id": comment["databaseId"], "user": _graphql_login(comment["author"]),
                                 "body": comment["body"],
                                 "created_at": _text(comment["createdAt"], "comment creation time"),
                                 "updated_at": _text(comment["updatedAt"], "comment update time"),
                                 "edited_by": _graphql_login(comment["editor"]) if comment.get("editor") else None}
                                for comment in nodes]
                    if any(not isinstance(item["body"], str) or type(item["id"]) is not int for item in comments):
                        raise GitHubError("Invalid comment identity or body")
                    comments = _unique(comments, "id", "comment")
                events = node["timelineItems"]["nodes"]
                if not isinstance(events, list):
                    raise GitHubError("Incomplete pull request timeline")
                lifecycle = _text(events[0]["createdAt"], "inactive lifecycle timestamp") if events else None
            except (KeyError, TypeError) as error:
                raise GitHubError("Incomplete pull request history") from error
            histories[number] = {"comments": comments, "lifecycle_at": lifecycle}
        return histories

    def threads(self, number):
        query = """query($owner:String!, $repo:String!, $number:Int!, $cursor:String) {
          repository(owner:$owner, name:$repo) {
            pullRequest(number:$number) {
              reviewThreads(first:100, after:$cursor) {
                totalCount
                pageInfo { hasNextPage endCursor }
                nodes { id isResolved comments(first:100) {
                  nodes { author { login } createdAt }
                } }
              }
            }
          }
        }"""
        owner, repo = self.repo.split("/")
        cursor = None
        seen = set()
        results = []
        emptied = 0
        total = None
        while True:
            response = self.request("POST", "graphql", {"query": query, "variables": {
                "owner": owner, "repo": repo, "number": number, "cursor": cursor}})
            if not isinstance(response, dict) or response.get("errors"):
                raise GitHubError("GraphQL review-thread query failed")
            try:
                connection = response["data"]["repository"]["pullRequest"]["reviewThreads"]
                count = connection["totalCount"]
                if type(count) is not int or count < 0 or (total is not None and count != total):
                    raise GitHubError("Review thread count unavailable or changed during collection")
                total = count
                if not isinstance(connection["nodes"], list):
                    raise GitHubError("Missing review thread nodes")
                for node in connection["nodes"]:
                    comments = node["comments"]["nodes"]
                    if not isinstance(comments, list) or type(node["isResolved"]) is not bool:
                        raise GitHubError("Incomplete review thread")
                    if not comments:
                        # Every comment in the thread was deleted; nothing remains to resolve.
                        emptied += 1
                        continue
                    # Whoever opened the thread names it; whoever spoke in it can
                    # be objecting. An author's own thread with a reviewer's
                    # objection in reply was read as the author's alone.
                    results.append({"id": _text(node["id"], "thread identity"), "is_resolved": node["isResolved"],
                                    "author": _login(comments[0]["author"]),
                                    "created_at": _text(comments[0]["createdAt"], "thread creation time"),
                                    "voices": [{"user": _login(c["author"]),
                                                "created_at": _text(c["createdAt"], "thread comment time")}
                                               for c in comments]})
                info = connection["pageInfo"]
                if type(info["hasNextPage"]) is not bool:
                    raise GitHubError("Missing review-thread pagination state")
                if not info["hasNextPage"]:
                    break
                cursor = _text(info["endCursor"], "review-thread cursor")
                if cursor in seen:
                    raise GitHubError("Review-thread pagination did not advance")
                seen.add(cursor)
                if len(seen) >= 100:
                    raise GitHubError("Review-thread pagination exceeds collection bound")
            except (KeyError, TypeError) as error:
                raise GitHubError("Incomplete review-thread evidence") from error
        if len(results) + emptied != total:
            raise GitHubError("Incomplete review-thread list")
        return _unique(results, "id", "review thread")

    BUILD_QUERY = """query($owner:String!, $repo:String!, $number:Int!, $cursor:String) {
      repository(owner:$owner, name:$repo) { pullRequest(number:$number) {
        commits(last:1) { nodes { commit { oid statusCheckRollup {
          contexts(first:100, after:$cursor) {
            totalCount pageInfo { hasNextPage endCursor }
            nodes {
              __typename
              ... on CheckRun { name conclusion status startedAt detailsUrl
                checkSuite { workflowRun { workflow { resourcePath } } } }
              ... on StatusContext { context state createdAt }
            } } } } } } } } }"""

    def build_state(self, number, head):
        """This head's build, as `green`, `running` or `failed`.

        Cached per head like the head's statuses: a reconciliation reads the
        same pull request up to three times and the answer cannot differ
        usefully between them.
        """
        if head in self._builds:
            return self._builds[head]
        owner, repo = self.repo.split("/")
        nodes, cursor = [], None
        while True:
            response = self.request("POST", "graphql", {
                "query": self.BUILD_QUERY,
                "variables": {"owner": owner, "repo": repo, "number": number, "cursor": cursor}})
            # A partial answer nulls the field it could not resolve and says so.
            # Reading that as "no checks" would turn a rate-limited read into a
            # green light for every pull request in the run.
            if not isinstance(response, dict) or response.get("errors"):
                raise GitHubError("Build state query failed")
            try:
                commits = response["data"]["repository"]["pullRequest"]["commits"]["nodes"]
            except (KeyError, TypeError) as error:
                raise GitHubError("Build state unavailable") from error
            if not commits:
                raise GitHubError("Build state unavailable")
            commit = commits[0]["commit"]
            if commit.get("oid") != head:
                # The head moved while this was read, so the answer describes
                # another commit. Unfinished, rather than a stale green.
                return "running"
            rollup = commit.get("statusCheckRollup")
            if rollup is None:
                # No checks at all, and no error to explain it away. Several
                # governed repositories have none, and blocking them for ever
                # is not what the rule is for.
                verdict = "green"
                self._builds[head] = verdict
                return verdict
            connection = rollup["contexts"]
            page, info = connection["nodes"], connection["pageInfo"]
            if not isinstance(page, list) or not isinstance(info, dict):
                raise GitHubError("Incomplete check connection")
            nodes += page
            if not info.get("hasNextPage"):
                break
            cursor = info.get("endCursor")
            if not cursor:
                raise GitHubError("Check pagination did not advance")
        verdict = build_verdict(nodes)
        self._builds[head] = verdict
        return verdict

    def snapshot(self, number, policy, history=None):
        """Full evidence for one pull request.

        `history` is the comments and latest inactive transition already read
        for this pull request by `histories()`. Rebuilding an author's slots
        reads both for every candidate in one query, and reading them again per
        pull request cost a second comment request and a walk of the whole issue
        timeline, which pages.
        """
        try:
            raw = self.request("GET", f"{self.root}/pulls/{number}")
            result = self._pr(raw)
            count = raw["changed_files"]
            if type(count) is not int or count < 0 or count > 3000:
                raise GitHubError("Changed-file count unavailable or above GitHub's 3000-file limit")
            files = self.pages(f"{self.root}/pulls/{number}/files")
            if len(files) != count:
                raise GitHubError("Incomplete changed-file list")
            result["files"] = []
            for file in files:
                normalized = {"filename": _text(file["filename"], "changed-file path")}
                if file.get("status") == "renamed" and not file.get("previous_filename"):
                    raise GitHubError("Renamed file is missing its source path")
                if "previous_filename" in file:
                    normalized["previous_filename"] = _text(file["previous_filename"], "rename source path")
                # What this file holds, and what happened to it. This route
                # lists the pull request against its merge base, so these two
                # are the pull request's own changes and nothing else — which
                # is what says a new head carries the same work as the old.
                # A read that carries none of these says nothing about whether
                # a later commit is the same work, and the reader treats it as
                # unknown rather than as unchanged.
                if isinstance(file.get("status"), str):
                    normalized["status"] = file["status"]
                if isinstance(file.get("sha"), str) and file["sha"]:
                    normalized["content"] = file["sha"]
                # And the patch itself, digested. What the file holds is not
                # enough: a conflict resolved by keeping your own side leaves
                # the file byte for byte what the reviewer saw while the patch
                # against the moved base now also undoes what the base did.
                # Only the patch itself. Counting its lines instead was two
                # small integers, and they are equal for the one case this
                # exists to catch: a conflict resolved by keeping your own
                # side changes the patch and not its line counts. Where
                # GitHub sends no patch — a file too large, a binary — this
                # says nothing, and nothing is carried.
                patch = file.get("patch")
                if isinstance(patch, str):
                    normalized["shape"] = hashlib.sha256(patch.encode()).hexdigest()
                result["files"].append(normalized)
            _unique(result["files"], "filename", "changed file")
            reviews = []
            for review in self.pages(f"{self.root}/pulls/{number}/reviews"):
                state = _text(review["state"], "review state")
                if state not in {"APPROVED", "CHANGES_REQUESTED", "COMMENTED", "DISMISSED", "PENDING"}:
                    raise GitHubError("Unknown review state")
                if state == "PENDING":
                    continue
                reviews.append({"id": review["id"], "user": _login(review["user"]), "state": state,
                                "commit_id": _text(review["commit_id"], "review commit"),
                                "submitted_at": _text(review["submitted_at"], "review time"),
                                "body": review["body"]})
            if any(type(item["id"]) is not int or not isinstance(item["body"], str) for item in reviews):
                raise GitHubError("Invalid review identity or body")
            result["reviews"] = _unique(reviews, "id", "review")
            # One route for comments, always. The other carries no editor,
            # and three defects in one day came from the same shape: a field
            # one read can supply and the other cannot, read by something
            # that decides a verdict — so the same pull request got two
            # different answers in one run, and the write was refused or the
            # check held pending for ever.
            reuse = history is not None
            result["comments"] = history["comments"] if reuse else self.histories([number]).get(
                number, {"comments": []})["comments"]
            result["threads"] = self.threads(number)
            result["lifecycle_at"] = history["lifecycle_at"] if reuse else self.activity(number)
            result["head_seen_at"] = self.head_seen_at(result["head"])
            result["build"] = self.build_state(number, result["head"])
            result["ready_published"] = self.ready_published(result["head"])
            result["requested_reviewers"] = [_login(user) for user in raw["requested_reviewers"]]
            # Who is holding this pull request now. A pull request handed to
            # somebody else is theirs to attest to, and this is how a hand-over
            # is written down.
            result["assignees"] = [_login(user) for user in raw.get("assignees") or []]
            result["labels"] = [_text(label["name"], "label name") for label in raw["labels"]]
            state, comment_id = parse_controller_state(result["comments"])
            if state is not None and state["number"] != number:
                raise GitHubError("Controller state belongs to another PR")
            result["controller_state"] = state
            result["controller_comment_id"] = comment_id
            result["controller_diff"] = parse_controller_diff(result["comments"], number)

            fallback = policy["fallback"]
            if not isinstance(fallback, dict):
                fallback = {"owners": fallback}
            groups = []
            for file in result["files"]:
                for path in {file["filename"], file.get("previous_filename", file["filename"])}:
                    group = next((area for area in policy["areas"]
                                  if any(path.startswith(prefix) for prefix in area["paths"])), fallback)
                    groups.append(group)
            users = set()
            for group in groups:
                users.update(group.get("owners", []))
                users.update(group.get("reviewers", []))
            users.update(review["user"] for review in reviews)
            users.update(thread["author"] for thread in result["threads"])
            # Whoever tells this controller to stop waiting for the bots has to
            # be someone it can vouch for, so their access is read like a
            # reviewer's. Anyone else's comment is nobody's business here.
            users.update(c["user"] for c in result["comments"] if c["body"].strip() == "/skip-bots")
            permissions = {}
            for user in sorted(users):
                if user.lower() in BOT_LOGINS or user.lower().endswith("[bot]"):
                    continue
                # Absent from the collaborator list means no write access. The
                # per-person endpoint would answer "read" for a stranger on a
                # public repository; every decision here asks only whether
                # someone can write, so the two agree where it counts.
                permissions[user] = self.permission(user)
            result["permissions"] = permissions
            result["repo"] = self.repo
            result["complete"] = True
            return result
        except (KeyError, TypeError) as error:
            raise GitHubError("Incomplete PR snapshot") from error

    def access(self):
        """Every collaborator's level, read once per reconciliation.

        One list answers what a request per person used to, and asking per
        person per pull request was thousands of requests a day.
        """
        if not self._listed:
            self._listed = True
            for entry in self.pages(f"{self.root}/collaborators?affiliation=all"):
                login = _login(entry)
                granted = entry.get("permissions")
                if not isinstance(granted, dict):
                    raise GitHubError("Collaborator listing is missing its permissions")
                self._permissions[login.lower()] = _capability(granted)
        return self._permissions

    def permission(self, login):
        """One person's level, asking directly when the listing did not name them.

        The listing answers for the collaborators a token can see, and a
        repository-scoped Actions token does not enumerate the people who reach
        a repository through the organisation rather than as collaborators of
        it. On one governed repository that is an administrator, and reading
        their absence as "no write access" marked six pull requests a
        configuration error on the day writes were turned on.

        `None` means the answer is unknown, which is not the same as `none`.
        """
        key = login.lower()
        if key in self._permissions:
            return self._permissions[key]
        self.access()
        if key in self._permissions:
            return self._permissions[key]
        try:
            answer = self.request("GET", f"{self.root}/collaborators/{quote(login, safe='')}/permission")
        except GitHubError as error:
            # Remembered for the run. Asking again on every snapshot turns one
            # unreachable answer into hundreds of requests, and a rate limit
            # into a loop that answers it with more requests. The pre-write
            # re-read clears this, so access revoked mid-run is still caught.
            print(f'Could not read access for {login}: {error}', file=sys.stderr)
            self._permissions[key] = None
            return None
        # The same capability flags the listing reads, not the legacy role
        # string beside them: a custom organisation role carries a name this
        # policy has never heard of but says plainly whether its holder pushes.
        granted = (answer or {}).get("user", {}).get("permissions") if isinstance(answer, dict) else None
        level = _capability(granted) if isinstance(granted, dict) else None
        if level is None:
            legacy = answer.get("permission") if isinstance(answer, dict) else None
            level = legacy if legacy in {"admin", "write", "read", "none"} else None
        if level is None:
            print(f'Unreadable access answer for {login}', file=sys.stderr)
        self._permissions[key] = level
        return level

    def forget_cached_access(self):
        """Re-read permissions and statuses, for the checks made before writing."""
        self._permissions.clear()
        self._statuses.clear()
        self._builds.clear()
        self._listed = False

    def _head_statuses(self, head):
        if head not in self._statuses:
            self._statuses[head] = self.pages(f"{self.root}/commits/{quote(head, safe='')}/statuses")
        return self._statuses[head]

    def head_seen_at(self, head):
        """When this controller first published a status for this head.

        Statuses cannot be edited or deleted, so this is a timestamp no author
        can move, unlike a commit date or the body of a comment.
        """
        ours = [item for item in self._head_statuses(head) if item.get("context") == "PR Hygiene"
                and (item.get("creator") or {}).get("login", "").lower() == "github-actions[bot]"]
        if not ours:
            return None
        stamps = [_text(item["created_at"], "status creation time") for item in ours]
        if any(not re.fullmatch(r"\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}Z", stamp) for stamp in stamps):
            raise GitHubError("Unexpected status timestamp format")
        return min(stamps)

    def ready_published(self, head):
        """Whether a human has ever been asked to review this head.

        Commit statuses cannot be edited or deleted, so this is a record no
        later run can take back. The recorded controller state cannot serve:
        it is rewritten on every run, so a pull request passing through any
        other state — an unresolved bot thread, a withdrawn objection, a
        transient configuration error — would lose the fact that it was ready.
        """
        return any(item.get("context") == "PR Hygiene"
                   and (item.get("creator") or {}).get("login", "").lower() == "github-actions[bot]"
                   and item.get("description") == "ready-for-human"
                   for item in self._head_statuses(head))

    def post_status(self, head, state, description, target_url=None):
        if state not in {"pending", "success", "failure", "error"}:
            raise GitHubError("Invalid commit status state")
        payload = {"state": state, "context": "PR Hygiene", "description": description[:140]}
        if target_url is not None:
            payload["target_url"] = target_url
        statuses = self._head_statuses(head)
        latest = next((item for item in statuses if item.get("context") == payload["context"]), None)
        if latest and (latest.get("creator") or {}).get("login", "").lower() == "github-actions[bot]":
            if all(latest.get(key) == payload.get(key) for key in ("state", "description", "target_url")):
                return latest
        written = self.request("POST", f"{self.root}/statuses/{quote(head, safe='')}", payload)
        if not isinstance(written, dict):
            raise GitHubError("Commit status was not acknowledged")
        self._statuses[head] = [dict(payload, creator={"login": "github-actions[bot]"},
                                     created_at=written.get("created_at"))] + statuses
        return written

    @staticmethod
    def state_comment_body(state, body, diff=None):
        marker = f"{STATE_MARKER} {json.dumps(state, separators=(',', ':'), sort_keys=True)} -->"
        if STATE_MARKER in body or DIFF_MARKER in body:
            raise GitHubError("Controller display body must not contain a state marker")
        if diff is not None:
            _validate_diff(diff)
            marker += f"\n{DIFF_MARKER} {json.dumps(diff, separators=(',', ':'), sort_keys=True)} -->"
        return marker + "\n\n" + body

    def upsert_state(self, number, state, body, comment_id=None, diff=None):
        _validate_state(state)
        if state["number"] != number:
            raise GitHubError("Controller state belongs to another PR")
        if diff is not None and diff["number"] != number:
            raise GitHubError("Controller diff belongs to another PR")
        payload = {"body": self.state_comment_body(state, body, diff)}
        if comment_id is None:
            result = self.request("POST", f"{self.root}/issues/{number}/comments", payload)
        else:
            result = self.request("PATCH", f"{self.root}/issues/comments/{comment_id}", payload)
        if not isinstance(result, dict) or type(result.get("id")) is not int:
            raise GitHubError("State comment write returned no identity")
        return result["id"]

    def comment(self, number, body):
        return self.request("POST", f"{self.root}/issues/{number}/comments", {"body": body})

    def delete_comment(self, comment_id):
        return self.request("DELETE", f"{self.root}/issues/comments/{comment_id}")

    def remove_checklist(self, number):
        """Take this controller's block out of the description, and nothing else."""
        current = self.request("GET", f"{self.root}/pulls/{number}")
        body = (current.get("body") or "") if isinstance(current, dict) else ""
        head, block, tail = _split_checklist(body)
        if block is None:
            return False
        self.request("PATCH", f"{self.root}/pulls/{number}", {"body": (head.rstrip() + tail).rstrip()})
        return True

    def latest_state_from_status(self, head):
        """The state this controller last published for `head`, from the commit status.

        For a pull request that has no record comment yet — a state reached
        before any move was announced — the status is the only trace, and it
        is this controller's own, posted by github-actions[bot].
        """
        mine = [item for item in self._head_statuses(head)
                if item.get("context") == "PR Hygiene"
                and (item.get("creator") or {}).get("login", "").lower() == "github-actions[bot]"]
        if not mine:
            return None
        return max(mine, key=lambda item: (item.get("created_at") or "", item.get("id") or 0)).get("description")

    def set_checklist(self, number, block):
        """Put this controller's block at the end of the description, and nothing else.

        The description is the author's. Only the block between the markers
        is this controller's to write: read the body immediately before the
        write, replace the LAST marker pair or append, and leave everything
        else byte for byte. Line endings are normalised for the comparison
        only, since the web form re-saves whole bodies as CRLF.
        """
        if CHECKLIST_START in block[len(CHECKLIST_START):] or CHECKLIST_END in block[:-len(CHECKLIST_END)]:
            raise GitHubError("Checklist block must not contain its own delimiters")
        current = self.request("GET", f"{self.root}/pulls/{number}")
        body = (current.get("body") or "") if isinstance(current, dict) else ""
        head, _, tail = _split_checklist(body)
        # Whatever follows the block is someone else's — CodeRabbit appends its
        # own — and stays exactly where it was.
        wanted = ((head.rstrip() + "\n\n" + block) if head.strip() else block) + tail
        if len(wanted) > 65536:
            raise GitHubError("Description too long for the checklist")
        if _normalise(body) == _normalise(wanted):
            return False
        self.request("PATCH", f"{self.root}/pulls/{number}", {"body": wanted})
        return True

    def set_label(self, number, label, enabled, current_labels):
        if enabled and label not in current_labels:
            return self.request("POST", f"{self.root}/issues/{number}/labels", {"labels": [label]})
        if not enabled and label in current_labels:
            return self.request("DELETE", f"{self.root}/issues/{number}/labels/{label}")
        return None

    def set_state_label(self, number, state, current_labels):
        """Exactly one state label, or none for a state that has none; retired names cleared."""
        wanted = LABEL_FOR_STATE.get(state)
        if wanted and wanted not in current_labels:
            self.request("POST", f"{self.root}/issues/{number}/labels", {"labels": [wanted]})
        for label in dict.fromkeys(STATE_LABELS + RETIRED_LABELS):
            if label in current_labels and label != wanted:
                self.request("DELETE", f"{self.root}/issues/{number}/labels/{label}")

    def request_reviewers(self, number, users):
        if not users:
            return None
        return self.request("POST", f"{self.root}/pulls/{number}/requested_reviewers", {"reviewers": list(users)})
