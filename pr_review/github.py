"""Read complete GitHub evidence and apply explicit review-workflow effects."""

import json
import re
import subprocess
from datetime import datetime
from urllib.parse import quote


class GitHubError(RuntimeError):
    """GitHub evidence or a requested effect could not be verified."""


STATE_MARKER = "<!-- platform-pr-review-state-v1"
STATE_PATTERN = re.compile(r"<!-- platform-pr-review-state-v1 (\{[^\r\n]*\}) -->")
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


def parse_controller_state(comments):
    """Ignore copied receipts; the oldest record wins; refuse corrupt history."""
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
        found.append((_text(comment["created_at"], "comment creation time"),
                      comment["id"], state))
    if not found:
        return (None, None)
    # Two runs reconciling one pull request at the same time can each open a
    # state comment, and refusing to read them held every pull request by that
    # author on an error status that no later run could clear. The oldest record
    # is authoritative: it carries the earliest admission, which is what orders
    # the author's slots, and every run reaches the same answer. GitHub reports
    # whole seconds, so the id breaks a tie between simultaneous writes.
    created_at, comment_id, state = min(found, key=lambda record: record[:2])
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

    def _run(self, arguments, payload=None):
        try:
            result = subprocess.run(
                ["gh", "api", *arguments],
                input=json.dumps(payload) if payload is not None else None,
                text=True, capture_output=True, timeout=60, check=False,
            )
        except (OSError, subprocess.TimeoutExpired) as error:
            raise GitHubError("GitHub API command unavailable or timed out") from error
        if result.returncode:
            # GraphQL answers with a usable payload and an errors array when only
            # part of a query resolved — a pull request closed since it was
            # listed answers null for its own alias while the rest answer
            # normally — and gh reports that as a failure. Hand the payload to
            # the caller, which decides which of those errors it tolerates.
            # Anything without one, and every REST call, still fails here.
            if "graphql" in arguments and result.stdout.strip():
                try:
                    body = json.loads(result.stdout)
                except ValueError:
                    body = None
                if isinstance(body, dict) and isinstance(body.get("data"), dict):
                    return body
            raise GitHubError(f"GitHub API command failed (exit {result.returncode})")
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
            result = [{"id": raw["id"], "user": _login(raw["user"]),
                       "body": raw["body"], "created_at": _text(raw["created_at"], "comment creation time"),
                       "updated_at": _text(raw["updated_at"], "comment update time")}
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
            nodes { databaseId body createdAt updatedAt author { login __typename } }
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
        # A pull request closed since the listing answers null for its own alias
        # while the rest answer normally; anything else is a real failure.
        for error in response.get("errors") or []:
            if (error or {}).get("type") != "NOT_FOUND":
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
                    comments = [{"id": comment["databaseId"], "user": _graphql_login(comment["author"]),
                                 "body": comment["body"],
                                 "created_at": _text(comment["createdAt"], "comment creation time"),
                                 "updated_at": _text(comment["updatedAt"], "comment update time")}
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
                nodes { id isResolved comments(first:1) {
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
                    if not isinstance(comments, list) or len(comments) > 1 or type(node["isResolved"]) is not bool:
                        raise GitHubError("Incomplete review thread")
                    if not comments:
                        # Every comment in the thread was deleted; nothing remains to resolve.
                        emptied += 1
                        continue
                    results.append({"id": _text(node["id"], "thread identity"), "is_resolved": node["isResolved"],
                                    "author": _login(comments[0]["author"]),
                                    "created_at": _text(comments[0]["createdAt"], "thread creation time")})
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
            reuse = history is not None
            result["comments"] = history["comments"] if reuse else self.comments(number)
            result["threads"] = self.threads(number)
            result["lifecycle_at"] = history["lifecycle_at"] if reuse else self.activity(number)
            result["head_seen_at"] = self.head_seen_at(result["head"])
            result["requested_reviewers"] = [_login(user) for user in raw["requested_reviewers"]]
            result["labels"] = [_text(label["name"], "label name") for label in raw["labels"]]
            state, comment_id = parse_controller_state(result["comments"])
            if state is not None and state["number"] != number:
                raise GitHubError("Controller state belongs to another PR")
            result["controller_state"] = state
            result["controller_comment_id"] = comment_id

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
            access = self.access()
            permissions = {}
            for user in sorted(users):
                if user.lower() in BOT_LOGINS or user.lower().endswith("[bot]"):
                    continue
                # Absent from the collaborator list means no write access. The
                # per-person endpoint would answer "read" for a stranger on a
                # public repository; every decision here asks only whether
                # someone can write, so the two agree where it counts.
                permissions[user] = access.get(user.lower(), "none")
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
        if not self._permissions:
            for entry in self.pages(f"{self.root}/collaborators?affiliation=all"):
                login = _login(entry)
                granted = entry.get("permissions")
                if not isinstance(granted, dict):
                    raise GitHubError("Collaborator listing is missing its permissions")
                # Read the capabilities, not the role's name: a custom organisation
                # role carries a name this policy has never heard of but still says
                # plainly whether its holder can push.
                for level in ("admin", "maintain", "push", "triage", "pull"):
                    if granted.get(level) is True:
                        self._permissions[login.lower()] = {"push": "write", "pull": "read"}.get(level, level)
                        break
        return self._permissions

    def forget_cached_access(self):
        """Re-read permissions and statuses, for the checks made before writing."""
        self._permissions.clear()
        self._statuses.clear()

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

    def upsert_state(self, number, state, body, comment_id=None):
        _validate_state(state)
        if state["number"] != number:
            raise GitHubError("Controller state belongs to another PR")
        marker = f"{STATE_MARKER} {json.dumps(state, separators=(',', ':'), sort_keys=True)} -->"
        if STATE_MARKER in body:
            raise GitHubError("Controller display body must not contain a state marker")
        payload = {"body": marker + "\n\n" + body}
        if comment_id is None:
            result = self.request("POST", f"{self.root}/issues/{number}/comments", payload)
        else:
            result = self.request("PATCH", f"{self.root}/issues/comments/{comment_id}", payload)
        if not isinstance(result, dict) or type(result.get("id")) is not int:
            raise GitHubError("State comment write returned no identity")
        return result["id"]

    def comment(self, number, body):
        return self.request("POST", f"{self.root}/issues/{number}/comments", {"body": body})

    def set_label(self, number, label, enabled, current_labels):
        if enabled and label not in current_labels:
            return self.request("POST", f"{self.root}/issues/{number}/labels", {"labels": [label]})
        if not enabled and label in current_labels:
            return self.request("DELETE", f"{self.root}/issues/{number}/labels/{label}")
        return None

    def set_ready_label(self, number, enabled, current_labels):
        label = "ready-for-human"
        if enabled and label not in current_labels:
            return self.request("POST", f"{self.root}/issues/{number}/labels", {"labels": [label]})
        if not enabled and label in current_labels:
            return self.request("DELETE", f"{self.root}/issues/{number}/labels/{label}")
        return None

    def request_reviewers(self, number, users):
        if not users:
            return None
        return self.request("POST", f"{self.root}/pulls/{number}/requested_reviewers", {"reviewers": list(users)})
