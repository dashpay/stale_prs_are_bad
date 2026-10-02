"""Write the synthetic boundary recordings beside this file.

Each is a run of the Python engine recorded offline by the real recorder
(`--record`), with the recorder's own test fake, `FakeGh` in
`pr_review/tests/test_conformance.py`, standing where `gh` does. The fake is
shaped, recording by recording, to walk a read path a live recording may
never show:

- `report`: both pull requests of the fake's repository, read in one batched
  history query; the collaborator listing and the per-person route for
  someone it does not name.
- `sync-pr-2`: a dry `sync --pr 2`, the one-author path the service runs;
  its last snapshot is taken after the caches are dropped, without the
  batched history, so it reads the comments alone and the issue timeline.
- `rich-evidence`: renamed, added, patch-less and type-changed files; a
  pending review; review threads over two pages, one a review bot's finding,
  one emptied; checks over two pages, re-runs and the engine's own run among
  them; the engine's statuses beside an impostor's; a `/skip-bots`; text
  outside ASCII.
- `long-history`: 150 comments, more than the batched window holds, so the
  conversation is read page by page with each comment's editor; the oldest
  is the engine's record and diff, refreshed by the engine.
- `edited-record`: the newest record edited by a person, which decides that
  nothing is known, beside an older genuine one.
- `partial-answer`: a GraphQL answer that fails with data: a pull request
  that no longer resolves, and a comment whose editor's account is gone.
- `transient-retry`: a 502 and an `i/o timeout` that `gh` reports, each
  asked once more and answered, and a call that runs out of Python's own
  sixty seconds, which is not.

    uv run -q --python 3.12 --no-project --with pyyaml python conformance/synthetic/generate.py

Python 3.12 only, as the corpus is. The same bytes on every run: the clock,
the fake's commit and the environment are fixed, and `python` in each
`recording.json` names the minor version only.
"""

import contextlib
import io
import json
import os
import shutil
import subprocess
import sys
import tempfile
from pathlib import Path
from unittest.mock import patch

HERE = Path(__file__).resolve().parent
sys.path.insert(0, str(HERE.parents[1]))

from pr_review import main  # noqa: E402
from pr_review.github import GitHub  # noqa: E402
from pr_review.tests.test_conformance import (CLOCK, OTHER_HEAD, REPO, FakeGh, _graph, _pr,  # noqa: E402
                                              policies)

PYTHON = (3, 12)
FILES = ('recording.json', 'calls.jsonl', 'evaluations.jsonl', 'verdicts.json', 'outputs.json', 'printed.txt')
ENGINE = {'login': 'github-actions', '__typename': 'Bot'}


def node(database_id, body, author, created_at, edited_at=None, editor=None):
    """A comment as the GraphQL comment queries answer it."""
    return {'databaseId': database_id, 'body': body, 'createdAt': created_at,
            'updatedAt': edited_at or created_at, 'lastEditedAt': edited_at, 'author': author, 'editor': editor}


def record_body(admitted_at, state='waiting-bots', diff=None):
    """The engine's record comment for pull request 2."""
    record = {'version': 1, 'number': 2, 'head': OTHER_HEAD, 'admitted_at': admitted_at, 'ready_since': None,
              'state': state, 'evidence': 'e' * 64, 'context': 'f' * 64}
    return GitHub.state_comment_body(record, 'The engine wrote this.', diff)


class Scenario(FakeGh):
    """`FakeGh`, every answer of which a recording can reshape."""

    def __init__(self):
        super().__init__()
        self.history = {n: [node(100 + n, f"/self-reviewed {pr['head']['sha']}",
                                 {'login': pr['user']['login'], '__typename': 'User'}, '2026-09-11T11:00:00Z')]
                        for n, pr in self.prs.items()}
        self.lifecycle = {}
        self.vanished = set()
        self.history_errors = []
        self.files = {}
        self.thread_pages = {}
        self.rollups = {}
        self.statuses = {}
        self.timelines = {}
        # (method, path) answered once with a failure: what gh says on stderr.
        self.failing = {}
        # (method, path) that runs out of Python's sixty seconds once.
        self.timeouts = set()

    def __call__(self, command, input=None, **options):
        command = list(command)
        if command[:3] == ['git', 'rev-parse', 'HEAD']:
            return subprocess.CompletedProcess(command, 0, 'e' * 40 + '\n', '')
        self.received.append((command, input))
        arguments = command[2:]
        key = (arguments[1], arguments[2])
        if key in self.timeouts:
            self.timeouts.discard(key)
            raise subprocess.TimeoutExpired(command, options.get('timeout'))
        if key in self.failing:
            return subprocess.CompletedProcess(command, 1, '', self.failing.pop(key))
        body = self.answer(arguments[1], arguments[2], json.loads(input) if input else None)
        if isinstance(body, dict) and body.get('errors'):
            said = '; '.join(error.get('message', error['type']) for error in body['errors'])
            return subprocess.CompletedProcess(command, 1, json.dumps(body), f'gh: {said}')
        return subprocess.CompletedProcess(command, 0, json.dumps(body), '')

    def answer(self, method, path, payload):
        if method == 'POST' and path == 'graphql':
            query, variables = payload['query'], payload['variables']
            if 'fragment history' in query:
                return self.histories(query)
            if 'comments(first:100, after:$after)' in query:
                return self.comment_page(variables['number'], variables['after'])
            if 'reviewThreads' in query:
                pages = self.thread_pages.get(variables['number'], {None: ([], None)})
                nodes, cursor = pages[variables['cursor']]
                total = sum(len(page) for page, _ in pages.values())
                return _graph(pullRequest={'reviewThreads': {'totalCount': total, 'pageInfo': {
                    'hasNextPage': cursor is not None, 'endCursor': cursor}, 'nodes': nodes}})
            if 'statusCheckRollup' in query:
                head = self.prs[variables['number']]['head']['sha']
                pages = self.rollups.get(variables['number'])
                if pages is None:
                    rollup = None
                else:
                    nodes, cursor = pages[variables['cursor']]
                    rollup = {'contexts': {'totalCount': sum(len(page) for page, _ in pages.values()),
                                           'pageInfo': {'hasNextPage': cursor is not None, 'endCursor': cursor},
                                           'nodes': nodes}}
                return _graph(pullRequest={'commits': {'nodes': [{'commit': {'oid': head,
                                                                             'statusCheckRollup': rollup}}]}})
            raise AssertionError('unknown query')
        root = f'repos/{REPO}/'
        parts = path[len(root):].split('?')[0].split('/')
        if method == 'GET' and parts[0] == 'pulls' and len(parts) == 3 and parts[2] == 'files' \
                and int(parts[1]) in self.files:
            return [self.files[int(parts[1])]]
        if method == 'GET' and parts[0] == 'commits' and parts[2] == 'statuses':
            return [self.statuses.get(parts[1], [])]
        if method == 'GET' and parts[0] == 'issues' and parts[2] == 'timeline':
            return [self.timelines.get(int(parts[1]), [])]
        return super().answer(method, path, payload)

    def histories(self, query):
        repository, errors = {}, list(self.history_errors)
        for number in self.prs:
            if f'pr{number}:' not in query:
                continue
            if number in self.vanished:
                repository[f'pr{number}'] = None
                errors.append({'type': 'NOT_FOUND', 'path': ['repository', f'pr{number}'],
                               'message': f'Could not resolve to a PullRequest with the number of {number}.'})
                continue
            nodes = self.history[number]
            repository[f'pr{number}'] = {'number': number,
                                         'comments': {'totalCount': len(nodes), 'nodes': nodes[-100:]},
                                         'timelineItems': {'nodes': self.lifecycle.get(number, [])}}
        answer = {'data': {'repository': repository}}
        if errors:
            answer['errors'] = errors
        return answer

    def comment_page(self, number, after):
        nodes = self.history[number]
        start = 0 if after is None else int(after[1:])
        page = nodes[start:start + 100]
        more = start + 100 < len(nodes)
        return _graph(pullRequest={'comments': {
            'totalCount': len(nodes), 'pageInfo': {'hasNextPage': more, 'endCursor': f'c{start + 100}' if more else None},
            'nodes': page}})


def admitted(gh, admitted_at, **edit):
    """Put the engine's genuine record, admitting pull request 2, before its other comments.

    Its admission then stands at the re-check before a write, so a dry sync
    reaches the last snapshot and evaluates it, as a first admission does not.
    """
    gh.history[2] = [node(200, record_body(admitted_at), ENGINE, admitted_at, **edit)] + gh.history[2]


def report():
    return Scenario(), ['report']


def sync_pr_2():
    return Scenario(), ['sync', '--pr', '2']


def rich_evidence():
    gh = Scenario()
    pr = gh.prs[2]
    pr.update(body='Speeds up the drive ⚡ — résumé', title='Make the drive faster ⚡',
              labels=[{'name': 'waiting-bots'}, {'name': 'bug'}], assignees=[{'login': 'owner'}],
              requested_reviewers=[{'login': 'owner', 'type': 'User'}], changed_files=4)
    gh.files[2] = [
        {'filename': 'packages/drive/b.rs', 'previous_filename': 'packages/drive/a.rs', 'status': 'renamed',
         'sha': '2' * 40, 'patch': '@@ -1 +1 @@\n-a\n+b'},
        # A new file GitHub sends no patch for: its blob decides it.
        {'filename': 'packages/drive/golden.bin', 'status': 'added', 'sha': '3' * 40},
        # A regular file becoming a symlink: one path, listed twice.
        {'filename': 'packages/drive/CLAUDE.md', 'status': 'removed', 'sha': '4' * 40,
         'patch': '@@ -1 +0,0 @@\n-x'},
        {'filename': 'packages/drive/CLAUDE.md', 'status': 'added', 'sha': '5' * 40,
         'patch': '@@ -0,0 +1 @@\n+docs/x'},
        {'filename': 'packages/drive/résumé.md', 'status': 'modified', 'sha': '6' * 40,
         'patch': '@@ -1 +1 @@\n-é\n+è'}]
    gh.reviews[2].append({'id': 24, 'user': {'login': 'owner'}, 'state': 'PENDING', 'commit_id': OTHER_HEAD,
                          'submitted_at': None, 'body': ''})
    gh.history[2] += [
        node(301, '/skip-bots', {'login': 'helper', '__typename': 'User'}, '2026-09-11T11:30:00Z'),
        node(302, 'Looks good to me \U0001f44d', {'login': 'coderabbitai', '__typename': 'Bot'},
             '2026-09-11T11:40:00Z', '2026-09-11T11:45:00Z', {'login': 'coderabbitai', '__typename': 'Bot'})]
    gh.lifecycle[2] = [{'createdAt': '2026-09-06T00:00:00Z'}]
    gh.timelines[2] = [{'event': 'closed', 'created_at': '2026-09-05T00:00:00Z'},
                       {'event': 'reopened', 'created_at': '2026-09-05T01:00:00Z'},
                       {'event': 'convert_to_draft', 'created_at': '2026-09-06T00:00:00Z'},
                       {'event': 'ready_for_review', 'created_at': '2026-09-07T00:00:00Z'}]
    admitted(gh, '2026-09-08T00:00:00Z')
    minor = ('_\U0001f3af Functional Correctness_ | _\U0001f7e1 Minor_ | _⚡ Quick win_\n\n'
             '**Reject an empty identifier.**\n<!-- cr-comment:v1:1 -->')
    gh.thread_pages[2] = {
        None: ([{'id': 'T1', 'isResolved': True, 'opening': {'nodes': [{'body': minor}]},
                 'comments': {'nodes': [{'author': {'login': 'coderabbitai'}, 'createdAt': '2026-09-11T10:30:00Z'}]}},
                {'id': 'T2', 'isResolved': True, 'opening': {'nodes': [{'body': 'Why this?'}]},
                 'comments': {'nodes': [{'author': {'login': 'reviewer'}, 'createdAt': '2026-09-11T10:31:00Z'},
                                        {'author': {'login': 'owner'}, 'createdAt': '2026-09-11T10:32:00Z'}]}}],
               't1'),
        't1': ([{'id': 'T3', 'isResolved': False, 'opening': {'nodes': []}, 'comments': {'nodes': []}}], None)}
    ci = '/dashpay/platform/actions/workflows/ci.yml'
    ours = '/dashpay/platform/actions/workflows/pr-review-policy.yml'

    def check(name, conclusion, started, workflow=ci, status='COMPLETED'):
        return {'__typename': 'CheckRun', 'name': name, 'conclusion': conclusion, 'status': status,
                'startedAt': started, 'detailsUrl': 'https://github.com/dashpay/platform/actions/runs/1/job/2',
                'checkSuite': {'workflowRun': {'workflow': {'resourcePath': workflow}}}}
    gh.rollups[2] = {
        None: ([check('tests', 'FAILURE', '2026-09-11T09:00:00Z'), check('tests', 'SUCCESS', '2026-09-11T09:30:00Z'),
                check('policy / reconcile', None, '2026-09-11T10:00:00Z', ours, 'IN_PROGRESS')], 'r1'),
        'r1': ([{'__typename': 'StatusContext', 'context': 'ci', 'state': 'SUCCESS', 'createdAt': '2026-09-11T09:40:00Z'},
                {'__typename': 'StatusContext', 'context': 'PR Hygiene', 'state': 'PENDING',
                 'createdAt': '2026-09-11T09:41:00Z'}], None)}
    engine = {'login': 'github-actions[bot]', 'type': 'Bot'}
    gh.statuses[OTHER_HEAD] = [
        {'id': 3, 'context': 'PR Hygiene', 'state': 'pending', 'description': 'ready-for-human',
         'created_at': '2026-09-11T12:00:00Z', 'creator': engine},
        {'id': 2, 'context': 'PR Hygiene', 'state': 'pending', 'description': 'waiting-bots',
         'created_at': '2026-09-11T09:00:00Z', 'creator': engine},
        {'id': 1, 'context': 'PR Hygiene', 'state': 'success', 'description': 'ready-to-merge',
         'created_at': '2020-01-01T00:00:00Z', 'creator': {'login': 'impostor', 'type': 'User'}},
        {'id': 0, 'context': 'CodeRabbit', 'state': 'success', 'description': 'Review completed',
         'created_at': '2026-09-11T10:00:00Z', 'creator': {'login': 'coderabbitai[bot]', 'type': 'Bot'}}]
    return gh, ['sync', '--pr', '2']


def long_history():
    gh = Scenario()
    diff = {'number': 2, 'diff': 'a' * 64, 'diff_heads': [OTHER_HEAD], 'diff_seen': '2026-09-11T09:00:00Z'}
    record = node(1, record_body('2026-09-11T09:00:00Z', diff=diff), ENGINE, '2026-09-01T00:00:00Z',
                  '2026-09-11T12:00:00Z', ENGINE)
    chatter = [node(1000 + n, f'comment {n}', {'login': 'someone', '__typename': 'User'}, '2026-09-02T00:00:00Z')
               for n in range(2, 150)]
    gh.history[2] = [record] + chatter + gh.history[2]
    return gh, ['sync', '--pr', '2']


def edited_record():
    gh = Scenario()
    genuine = node(201, record_body('2026-09-05T00:00:00Z'), ENGINE, '2026-09-05T00:00:00Z')
    forged = node(202, record_body('2020-01-01T00:00:00Z', state='ready-for-human'), ENGINE, '2026-09-06T00:00:00Z',
                  '2026-09-12T00:00:00Z', {'login': 'mallory', '__typename': 'User'})
    gh.history[2] = [genuine, forged] + gh.history[2]
    return gh, ['sync', '--pr', '2']


def partial_answer():
    gh = Scenario()
    gh.prs[3] = _pr(3, 'reviewer', 'd' * 40, '2026-09-10T02:00:00Z')
    gh.history[3] = []
    gh.vanished.add(3)
    gh.history[2] = gh.history[2] + [
        node(401, 'Reworded since.', {'login': 'someone', '__typename': 'User'}, '2026-09-11T11:10:00Z',
             '2026-09-11T11:20:00Z', None)]
    gh.history_errors = [{'type': 'NOT_FOUND', 'path': ['repository', 'pr2', 'comments', 'nodes', 1, 'editor'],
                          'message': 'Could not resolve to a node with the global id of an editor.'}]
    return gh, ['sync', '--pr', '2']


def transient_retry():
    gh = Scenario()
    gh.failing[('GET', f'repos/{REPO}/pulls/2')] = 'HTTP 502: Bad Gateway (https://api.github.com/repos/x)'
    gh.failing[('GET', f'repos/{REPO}/collaborators?affiliation=all&per_page=100')] = (
        'Get "https://api.github.com/repos/x/collaborators": dial tcp 140.82.112.6:443: i/o timeout')
    gh.timeouts.add(('GET', f'repos/{REPO}/collaborators/passerby/permission'))
    admitted(gh, '2026-09-10T00:00:00Z')
    return gh, ['sync', '--pr', '2']


SCENARIOS = {'report': report, 'sync-pr-2': sync_pr_2, 'rich-evidence': rich_evidence, 'long-history': long_history,
             'edited-record': edited_record, 'partial-answer': partial_answer, 'transient-retry': transient_retry}


def record(name, destination):
    """Record one scenario into `destination`/`name`, replacing what was there."""
    gh, options = SCENARIOS[name]()
    with tempfile.TemporaryDirectory() as scratch:
        directory = Path(scratch, name)
        with patch('subprocess.run', side_effect=gh), patch('pr_review.github.time.sleep'), patch.dict(os.environ), \
                patch('pr_review.telemetry.fetch', side_effect=AssertionError('no status page offline')), \
                policies() as root, patch.object(main, 'clock', return_value=CLOCK), \
                contextlib.redirect_stdout(io.StringIO()), contextlib.redirect_stderr(io.StringIO()):
            os.environ.pop('GITHUB_RUN_ID', None)
            try:
                main.run([*options, '--repo', REPO, '--policies-root', root, '--record', str(directory)])
            except Exception as error:
                # A run that failed is recorded with its outcome; anything
                # else is a fault of this generator.
                if not (directory / 'recording.json').is_file():
                    raise
                outcome = json.loads((directory / 'recording.json').read_text())['outcome']
                if outcome.get('raised') != type(error).__name__:
                    raise
        path = directory / 'recording.json'
        meta = json.loads(path.read_text())
        meta['python'] = '{}.{}'.format(*PYTHON)
        path.write_text(json.dumps(meta, ensure_ascii=False, indent=1) + '\n')
        if gh.writes():
            raise AssertionError(f'{name}: a write reached the fake')
        target = Path(destination, name)
        target.mkdir(parents=True, exist_ok=True)
        for file in FILES:
            shutil.copyfile(directory / file, target / file)
    return target


def generate(destination=HERE):
    if sys.version_info[:2] != PYTHON:
        raise SystemExit('synthetic recordings are made on Python {}.{} only; this is {}'.format(
            *PYTHON, sys.version.split()[0]))
    return [record(name, destination) for name in SCENARIOS]


if __name__ == '__main__':
    for written in generate(Path(sys.argv[1]) if len(sys.argv) > 1 else HERE):
        print(written)
