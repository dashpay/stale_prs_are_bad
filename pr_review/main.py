"""Read and reconcile a repository's PR Hygiene policy using trusted repository data."""

import argparse
import hashlib
import json
import os
import re
from collections import Counter
from concurrent.futures import ThreadPoolExecutor
from datetime import datetime, timezone
from pathlib import Path
import sys

from . import telemetry
from .github import STATE_MARKER, STATE_PATTERN, GitHub, GitHubError, current_checklist, parse_controller_state
from .policy import (CHECKLIST_END, CHECKLIST_START, LABEL_FOR_STATE, MOVE_MARKER, NUDGE_MARKER, RETIRED_LABELS,
                     STATE_LABELS, admit, codeowners, effective_admission, evaluate, fingerprint, missing_paths,
                     validate_policy)
from .registry import POLICIES, entry_for, load_registry, policy_path

WAIVED_LABEL = 'bot-review-skipped'
NUDGES_PER_RUN = 1



def utc_now():
    return datetime.now(timezone.utc).isoformat(timespec='seconds').replace('+00:00', 'Z')


def context_fingerprint(prs, author):
    """What this author's admission depends on, and nothing more.

    Hashing every open pull request made any push anywhere in the repository
    during a run demote the pull request being published to pending, and on an
    active repository something moves every few minutes. Only the author's own
    pull requests decide their slots.
    """
    # The fields admit() reads and no others: a push to another of the
    # author's pull requests changes its head, and their slots not at all.
    fields = ('number', 'base', 'draft', 'state')
    values = [tuple(pr.get(k) for k in fields) for pr in prs if pr['author'].lower() == author.lower()]
    return hashlib.sha256(json.dumps(sorted(values), sort_keys=True).encode()).hexdigest()


def load_histories(api, selected):
    """Admission evidence for every candidate, in one query rather than two each."""
    histories = api.histories([pr['number'] for pr in selected])
    loaded = []
    for pr in selected:
        history = histories.get(pr['number'])
        if history is None:
            # Closed or deleted since the listing: it holds no slot either way.
            continue
        state, comment_id = parse_controller_state(history['comments'])
        if state is not None and state['number'] != pr['number']:
            raise GitHubError('Controller admission history belongs to another PR')
        loaded.append(dict(pr, comments=history['comments'], controller_state=state,
                           controller_comment_id=comment_id, lifecycle_at=history['lifecycle_at']))
    return loaded


def admission_conflicts(policy, candidates):
    counts = {}
    for pr in candidates:
        if (pr['state'] == 'open' and not pr['draft'] and pr['base'] in policy['target_branches']
                and effective_admission(pr)):
            author = pr['author'].lower()
            counts[author] = counts.get(author, 0) + 1
    return {author for author, count in counts.items() if count > policy['max_active_prs']}


def admission_fingerprint(candidates):
    return sorted((p['number'], effective_admission(p), p.get('lifecycle_at')) for p in candidates)


# How far apart the scheduled sweeps that call periodic_batch actually are. The
# cursor is read from the clock rather than stored, so this has to match the
# caller workflow's cron: bucketing by a shorter interval advances the cursor
# further than one batch per sweep and leaves the skipped PRs unswept for ever.
SWEEP_SECONDS = 3600


def periodic_batch(prs, size, epoch_seconds=None, cadence=SWEEP_SECONDS):
    """Rotate a bounded slice without a persisted scheduler cursor."""
    if type(size) is not int or size < 1:
        raise ValueError('Batch size must be positive')
    if type(cadence) is not int or cadence < 1:
        raise ValueError('Sweep cadence must be positive')
    ordered = sorted(prs, key=lambda p: p['number'])
    if not ordered:
        return []
    seconds = datetime.now(timezone.utc).timestamp() if epoch_seconds is None else epoch_seconds
    start = (int(seconds // cadence) * size) % len(ordered)
    return [ordered[(start + offset) % len(ordered)] for offset in range(min(size, len(ordered)))]


# The build scan runs between sweeps and reconciles only pull requests already
# recorded as waiting on one. A build turning green raises no event this
# controller hears, so without it a pull request waits for the hourly rotation
# to reach it; with it the scan costs a listing and one batched read when
# nothing is waiting, which is the usual case.
BUILD_SCAN_SECONDS = 900
BUILD_SCAN_SIZE = 12


def collect(api, policy, number=None, apply=False, reconcile_author=False, batch_size=None,
            waiting_on_build=False):
    """Load global admission history, then full evidence for requested PRs."""
    prs = api.open_prs()
    selected = [p for p in prs if p['base'] in policy['target_branches']]
    batch_numbers = None
    if batch_size is not None:
        if number is not None:
            raise ValueError('Batch selection cannot be combined with a PR selector')
        batch = periodic_batch(selected, batch_size)
        batch_numbers = {p['number'] for p in batch}
        authors = {p['author'].lower() for p in batch}
        selected = [p for p in selected if p['author'].lower() in authors]
    if number is not None:
        principal = next((p for p in prs if p['number'] == number), None) or api.pull(number)
        selected = [p for p in selected if p['author'].lower() == principal['author'].lower()]
    try:
        candidates = load_histories(api, selected)
        requested = [p for p in candidates if number is None or p['number'] == number]
        if number is not None and reconcile_author:
            slots = admit(policy, candidates, utc_now())
            conflicts = admission_conflicts(policy, candidates)
            requested = [p for p in candidates if p['number'] == number
                         or bool(slots.get(p['number'])) != bool(effective_admission(p))
                         or p['author'].lower() in conflicts]
        if batch_numbers is not None:
            requested = [p for p in requested if p['number'] in batch_numbers]
        if waiting_on_build:
            # The recorded state, not a fresh verdict: knowing which pull
            # requests are worth a snapshot is the whole saving. A pull request
            # with no record yet — a state reached before any move was
            # announced — is asked by its own last status, one read per head.
            def waiting_on(p):
                recorded = p.get('controller_state') or {}
                if recorded:
                    return recorded.get('state') == 'waiting-build'
                return api.latest_state_from_status(p['head']) == 'waiting-build'
            waiting = [p for p in candidates if waiting_on(p)]
            requested = periodic_batch(waiting, BUILD_SCAN_SIZE, cadence=BUILD_SCAN_SECONDS)
        if number is not None and not requested and not reconcile_author:
            raise GitHubError(f'PR #{number} is not open on a configured target branch')
        with ThreadPoolExecutor(max_workers=4) as pool:
            # load_histories already read these comments and this transition for
            # every candidate in one query; the snapshot reuses that read.
            def snapshot(p):
                try:
                    return api.snapshot(p['number'], policy,
                                        history={'comments': p['comments'], 'lifecycle_at': p['lifecycle_at']})
                except GitHubError as error:
                    return error
            taken = list(pool.map(snapshot, requested))
        # Admission was decided above, from every candidate's history, so one
        # pull request whose evidence could not be read invalidates only itself.
        # Marking everything selected — which a full pass makes everything
        # open — turned one rate-limited read into a repository-wide outage.
        snapshots = []
        for p, taken_one in zip(requested, taken):
            if isinstance(taken_one, GitHubError):
                print(f"PR #{p['number']}: {taken_one}; its status says so", file=sys.stderr)
                if apply:
                    _mark_unreadable(api, policy, p)
                continue
            snapshots.append(taken_one)
        return prs, candidates, snapshots
    except GitHubError:
        if apply:
            # Admission depends on all candidates, so incomplete history invalidates
            # every known active head, even when only one PR was requested.
            for pr in selected:
                _mark_unreadable(api, policy, pr)
        raise


def _mark_unreadable(api, policy, pr):
    try:
        current = api.pull(pr['number'])
        if current['state'] == 'open' and current['base'] in policy['target_branches']:
            api.post_status(current['head'], 'error', 'Incomplete policy evidence; reconciliation required')
    except GitHubError:
        print(f"PR #{pr['number']}: unable to publish evidence error status", file=sys.stderr)


def _was_ours(api, pr, apply):
    """Whether this controller's record comment is still on a pull request it no longer governs."""
    if not apply:
        return False
    try:
        return bool(bot_comments(dict(pr, comments=api.comments(pr['number'])), STATE_MARKER))
    except GitHubError:
        return False


def clear_marks(api, policy, prs, apply=False):
    """Take this controller's marks off a pull request it no longer governs.

    A pull request rebased onto a branch outside the policy is never selected
    again, so its labels and its checklist stayed exactly as they were the day
    it left — a pull request wearing `waiting-bots` and `bot-review-skipped`
    two days after this controller stopped looking at it, which reads as a
    verdict and is not one. The record comment goes with them: its words point
    at a checklist that is no longer there, and a pull request outside the
    policy holds no review slot, so the admission it records decides nothing.

    A name this controller has retired goes too, but only where it left a
    mark of its own: `ready-to-merge` is ordinary English, and a pull request
    that was never governed may be wearing somebody else's.
    """
    mine = set(STATE_LABELS) | {WAIVED_LABEL}
    for pr in prs:
        if pr['state'] != 'open' or pr['base'] in policy['target_branches']:
            continue
        labels = set(pr.get('labels') or [])
        block = current_checklist(pr.get('body'))
        # A retired name on its own proves nothing — `ready-to-merge` is
        # ordinary English — so the record comment is what says whether this
        # controller put it there. That read costs a request and is spent only
        # on the few pull requests still wearing one. A pull request with no
        # mark at all is not read at all: a record comment with no label and
        # no checklist beside it, a draft rebased away, shows as an empty line
        # and is not worth a request against every pull request every sweep.
        if not labels & mine and block is None:
            if not (labels & set(RETIRED_LABELS) and _was_ours(api, pr, apply)):
                continue
        stale = sorted(labels & (mine | set(RETIRED_LABELS)))
        print(f"PR #{pr['number']}: no longer governed ({pr['base']}); clearing "
              + ', '.join(filter(None, [', '.join(stale), 'the checklist' if block else ''])), file=sys.stderr)
        if not apply:
            continue
        for label in stale:
            try:
                api.set_label(pr['number'], label, False, pr.get('labels') or [])
            except GitHubError:
                print(f"PR #{pr['number']}: could not remove {label}", file=sys.stderr)
        if block is not None:
            try:
                api.remove_checklist(pr['number'])
            except GitHubError as error:
                print(f"PR #{pr['number']}: could not remove the checklist: {error}", file=sys.stderr)
        # The record comment goes with them. Its words point at a checklist
        # that is no longer there, and the record itself is inert: a pull
        # request outside the policy holds no review slot, so the admission it
        # carries decides nothing.
        try:
            records = bot_comments(dict(pr, comments=api.comments(pr['number'])), STATE_MARKER)
        except GitHubError as error:
            records = []
            print(f"PR #{pr['number']}: could not read the record comment: {error}", file=sys.stderr)
        # One that will not go does not keep the others.
        for comment in records:
            try:
                api.delete_comment(comment['id'])
            except GitHubError as error:
                print(f"PR #{pr['number']}: could not remove the record comment: {error}", file=sys.stderr)


def state_record(pr, result, context):
    return {key: result.get(key) for key in
            ('number', 'head', 'admitted_at', 'ready_since', 'state')} | {
                'version': 1, 'evidence': fingerprint(pr), 'context': context}


SAFE_PATH = re.compile(r'[A-Za-z0-9._/@+-]+')
MOVE_STATES = {'waiting-self-review': 'waiting-self-review', 'waiting-author': 'waiting-self-review',
               'ready-for-human': 'ready-for-human', 'ready-to-merge': 'ready-to-merge'}
POINTER = 'PR Hygiene: the checklist is in the description.'


def _files(files):
    """A few paths, quoted only when safe: a path is the pull request's to choose."""
    safe = [f for f in files if SAFE_PATH.fullmatch(f)]
    if not safe:
        return f'{len(files)} files'
    shown = ', '.join(f'`{f}`' for f in safe[:3])
    return shown + (f' and {len(files) - 3} more' if len(files) > 3 else '')


def checklist_block(result):
    """The description's block: every requirement, met or not, checked when met.

    One shape always. The state is the first unchecked line, which is what
    the label and the status say too, so the three surfaces never disagree —
    and an author with an approval in hand can see which files it did not
    cover, before anyone has to ask.
    """
    items = {item['item']: item for item in result.get('checklist') or []}
    if not items:
        return None
    box = lambda done: '[x]' if done else '[ ]'
    lines = [CHECKLIST_START, f"### PR Hygiene · `{result['head'][:7]}`"]
    bots = items['bots']
    line = f"- {box(bots['done'])} Bots — {' · '.join(bots['lines'])}"
    if bots['skippable']:
        line += ' — `/skip-bots` proceeds without the ones not yet reported'
    lines.append(line)
    attest = items['self_review']
    if attest['bot_author']:
        lines.append(f"- {box(attest['done'])} Self-review — not asked of a bot author")
    elif attest['address']:
        lines.append(f"- {box(attest['done'])} Self-review — address {'; '.join(attest['address'])}, then post `/self-reviewed`")
    elif attest['done']:
        lines.append('- [x] Self-review — posted; again after any push')
    elif not bots['done']:
        lines.append(f"- {box(attest['done'])} Self-review — post `/self-reviewed` once the bots are done")
    else:
        lines.append('- [ ] Self-review — post `/self-reviewed`')
    slot = items['slot']
    line = f"- {box(slot['done'])} Within your {slot['limit']} open PRs"
    if not slot['done']:
        line += ' — this one is beyond the limit; it waits until one merges'
    lines.append(line)
    build = items['build']
    text = {'green': 'Build green', 'failed': 'Build failed', 'running': 'Build running'}.get(build['state'], f"Build {build['state']}")
    if build['latched'] and not build['done']:
        text += ' — review was already requested; it still has to pass to merge'
    lines.append(f"- {box(build['done'])} {text}")
    approvals = items['approvals']
    areas = approvals['areas']
    if all(a.get('owned') for a in areas) and not approvals['awaiting']:
        lines.append(f"- {box(approvals['done'])} Approvals — you own every area touched; none needed")
    else:
        # A plain bullet, not a task: a task parent counts in GitHub's N-of-M
        # beside its children and the bar would read double.
        lines.append('- Approvals')
        for area in areas:
            name = 'files with no dedicated owner' if area['area'] == 'fallback' else f"`{area['area']}`"
            if area.get('owned'):
                lines.append(f'  - [x] {name} — you own it')
            elif area['approved_by']:
                lines.append(f"  - [x] {name} ({_files(area['files'])}) — approved by {', '.join(area['approved_by'])}")
            else:
                lines.append(f"  - [ ] {name} ({_files(area['files'])}) — {' or '.join(area['approvers'])}")
        for objection in approvals['awaiting']:
            lines.append(f'  - [ ] {objection} — waiting for them to re-review or dismiss')
    lines.append('')
    lines.append('When every box is checked the `PR Hygiene` check passes and this can merge.')
    lines.append(CHECKLIST_END)
    return '\n'.join(lines)


def move_text(result):
    """One line for the person whose move it now is. No mentions: the comment itself notifies."""
    move = MOVE_STATES.get(result['state'])
    if move is None:
        return None
    items = {item['item']: item for item in result.get('checklist') or []}
    if move == 'waiting-self-review':
        # What actually blocks it, not what usually does. A bot's own finding
        # lands in this move too, and "bots are done, post /self-reviewed" is
        # wrong three ways there: the bots are not done, the author has
        # already attested, and the thing owed goes unsaid. Which of the two
        # it is, is the bots line of the checklist — an objection to answer
        # can sit beside an open bot finding, and used to decide the wording.
        reasons = [b for b in result.get('blockers') or [] if not b.startswith('Proceeded without')]
        address = items['self_review']['address']
        if items['bots']['done']:
            what = f"address {'; '.join(address)}, then post `/self-reviewed`" if address else 'post `/self-reviewed`'
            line = f'Bots are done — your move: {what}.'
        else:
            what = '; '.join(reasons) or 'answer the review'
            line = f'Your move: {what}.'
    elif move == 'ready-for-human':
        line = f"Ready for review — needs {' or '.join(result.get('reviewers') or []) or 'an owner'}."
    else:
        line = 'Policy satisfied — this can merge.'
    return f"{MOVE_MARKER} state={move} sha={result['head']} -->\n{line}\nFull checklist in the description."


def _visible(comment_body):
    """A record comment's text, without the record."""
    return comment_body.split('-->', 1)[-1].strip() if comment_body.startswith(STATE_MARKER) else comment_body.strip()


def _same(a, b):
    return (a or '').replace('\r\n', '\n').strip() == (b or '').replace('\r\n', '\n').strip()


def bot_comments(pr, marker):
    """This controller's own comments carrying `marker`, oldest first by last write.

    Its own: written by github-actions[bot] AND carrying a record that parses
    and names this pull request. Another workflow's bot comment that quotes
    a marker is nobody's business here, least of all to edit or delete.
    """
    def own(c):
        if c.get('user', '').lower() != 'github-actions[bot]' or marker not in c.get('body', ''):
            return False
        found = STATE_PATTERN.findall(c.get('body', ''))
        if len(found) != 1:
            return False
        try:
            return json.loads(found[0]).get('number') == pr['number']
        except (ValueError, TypeError, AttributeError):
            return False
    return sorted((c for c in pr.get('comments', []) if own(c)),
                  key=lambda c: (c.get('updated_at') or c.get('created_at', ''), c.get('id', 0)))


def _hours_since(iso, now):
    from datetime import datetime
    a = datetime.fromisoformat(iso.replace('Z', '+00:00')); b = datetime.fromisoformat(now.replace('Z', '+00:00'))
    return (b - a).total_seconds() / 3600


def nudge(api, pr, result, allowance):
    """Ask a bot to look at this head, at most once per head and a few per run.

    Asking is best effort: a comment that cannot be posted is reported and
    retried on the next run. It must never fail a reconciliation, because the
    waiver that eventually unblocks the pull request does not depend on it.
    """
    posted = 0
    for bot in result.get('nudge', []):
        if posted >= allowance:
            break
        current = api.pull(pr['number'])
        if current['state'] != 'open' or current['head'] != pr['head']:
            break
        marker = f"{NUDGE_MARKER} bot={bot} sha={pr['head']} -->"
        body = (f"{marker}\n@{bot} review\n\n"
                f"No review for `{pr['head'][:8]}` yet, so PR Hygiene is asking once. "
                f"If nothing arrives, the requirement is dropped for this commit and the pull "
                f"request is labelled `{WAIVED_LABEL}`.")
        try:
            api.comment(pr['number'], body)
            posted += 1
        except GitHubError:
            print(f"PR #{pr['number']}: could not ask {bot} to review; will retry", file=sys.stderr)
    return posted


def publish(api, policy, pr, result, context_prs, apply=False, candidates=None):
    """Publish only after revalidating the PR and its policy evidence."""
    if not apply:
        return
    candidates = context_prs if candidates is None else candidates
    actionable = result['state'] in {'ready-for-human', 'ready-to-merge'}

    def admission_valid(expected):
        current_prs = api.open_prs()
        if context_fingerprint(current_prs, pr['author']) != context:
            return False
        # Only this author's histories can change this PR's admission decision.
        relevant = [p for p in current_prs if p['base'] in policy['target_branches']
                    and p['author'].lower() == pr['author'].lower()]
        histories = load_histories(api, relevant)
        baseline = [p for p in expected if p['author'].lower() == pr['author'].lower()]
        if admission_fingerprint(histories) != admission_fingerprint(baseline):
            return False
        slots = admit(policy, histories, result.get('admitted_at') or utc_now())
        return slots.get(pr['number']) == result.get('admitted_at')

    identity = ('head', 'base', 'base_sha', 'draft', 'state')

    def identity_matches():
        current = api.pull(pr['number'])
        return all(current.get(k) == pr.get(k) for k in identity)

    if not identity_matches():
        return
    context = context_fingerprint(context_prs, pr['author'])
    if actionable and fingerprint(api.snapshot(pr['number'], policy)) != fingerprint(pr):
        api.post_status(pr['head'], 'pending', 'Review evidence changed; reconciliation required')
        return
    if actionable and not admission_valid(candidates):
        api.post_status(pr['head'], 'pending', 'PR admission context changed; reconciliation required')
        return

    desired = state_record(pr, result, context)
    now = utc_now()

    def finish(expected):
        # Admission history reads can be slow. Read this PR's review evidence
        # after those reads so a dismissed approval is not reused from before them.
        valid_admission = admission_valid(expected)
        api.forget_cached_access()
        final = api.snapshot(pr['number'], policy)
        if not valid_admission or fingerprint(final) != fingerprint(pr):
            api.post_status(pr['head'], 'pending', 'Review evidence changed; reconciliation required')
            return
        check = evaluate(policy, final, result.get('admitted_at'), utc_now())
        if result['status'] == 'success' and check['status'] != 'success':
            api.post_status(pr['head'], 'pending', 'Policy changed; reconciliation required')
            return
        api.post_status(pr['head'], result['status'], result['state'])

    ready = result['state'] == 'ready-for-human'
    requested = set(pr.get('requested_reviewers', []))
    missing = [u for u in result.get('reviewers', []) if u not in requested] if ready else []
    labels = pr.get('labels', [])
    wanted = ({LABEL_FOR_STATE[result['state']]} if result['state'] in LABEL_FOR_STATE else set()) \
             | ({WAIVED_LABEL} if result.get('waived') else set())
    managed = set(STATE_LABELS) | set(RETIRED_LABELS) | {WAIVED_LABEL}
    label_correct = set(labels) & managed == wanted
    # The description carries the checklist; a draft's does not (its author is
    # still writing it, and nobody reviews a draft). A description with no room
    # for it is left alone, and not fought over on every run.
    body_now = pr.get('body') or ''
    block = None if result['state'] == 'draft' else checklist_block(result)
    fits = block is None or len(body_now) - len(current_checklist(body_now) or '') + len(block) + 2 <= 65536
    stale_block = block is None and current_checklist(body_now) is not None
    block_correct = not stale_block and (block is None or not fits or _same(current_checklist(body_now), block))
    # The record lives in this controller's newest comment, whichever kind. It
    # is refreshed there silently whenever what it holds has changed — the
    # state, the head, the admission, the waiting time — never by a new
    # comment: a new comment is a notification, and a record is not news.
    holders = bot_comments(pr, STATE_MARKER)
    # The holder is the comment the record was read from — the one written
    # last — not the one created last.
    holder = next((c for c in holders if c['id'] == pr.get('controller_comment_id')), holders[-1] if holders else None)
    recorded = pr.get('controller_state') or {}
    record_fields = ('state', 'head', 'admitted_at', 'ready_since')
    record_correct = holder is None or all(recorded.get(k) == desired.get(k) for k in record_fields)
    move = MOVE_STATES.get(result['state'])
    move_body = move_text(result) if move and not (pr.get('author_is_bot') and move == 'waiting-self-review') else None
    # An announcement is made when the move passes to somebody and is kept
    # current in place while it stays with them. Two things take it out of
    # their hands: the move going to someone else and coming back, and a bot
    # reporting after they were told — the finding that voids an attestation.
    # Editing through either leaves the person with nothing in their inbox,
    # which is how three of four pull requests on one repository went quiet in
    # a day. A state nobody is asked to act on — a build re-run, a permission
    # read that failed — is not a change of hands and must not repost.
    was = MOVE_STATES.get(recorded.get('state'))
    announced = [c for c in bot_comments(pr, MOVE_MARKER)
                 if f'{MOVE_MARKER} state={move} sha={pr["head"]} -->' in c['body']]
    same_hand = was == move or was is None
    fresh = not announced or not result.get('bot_completed_at') or \
        announced[-1]['created_at'] >= result['bot_completed_at']
    target = announced[-1] if announced and same_hand and fresh else None
    # A standing comment of the earlier engine that already recorded this move
    # for this head is that announcement: it becomes the move comment in place.
    standing = [c for c in holders if MOVE_MARKER not in c['body']]
    if target is None and move_body and standing and recorded.get('state') == result['state'] and recorded.get('head') == pr['head']:
        target = standing[-1]
    move_correct = move_body is None or (target is not None and _same(_visible(target['body']), move_body))
    standing_correct = not standing or (len(standing) == 1 and not bot_comments(pr, MOVE_MARKER)
                                        and _same(_visible(standing[0]['body']), POINTER))
    if block_correct and label_correct and not missing and move_correct and record_correct and standing_correct:
        # Read current evidence on every run, but avoid churning the description, labels and comments.
        if actionable:
            finish(candidates)
        else:
            api.post_status(pr['head'], result['status'], result['state'])
        return

    api.post_status(pr['head'], 'pending', 'Evaluating current review policy')
    if not identity_matches():
        return
    if stale_block:
        try:
            api.remove_checklist(pr['number'])
        except GitHubError as error:
            print(f"PR #{pr['number']}: stale checklist not removed: {error}", file=sys.stderr)
        if not identity_matches():
            return desired
    if block is not None and not block_correct:
        try:
            api.set_checklist(pr['number'], block)
        except GitHubError as error:
            # The description is the author's; the check and the labels still
            # carry the state, and one refused write must not mark the pull
            # request an error.
            print(f"PR #{pr['number']}: checklist not written: {error}", file=sys.stderr)
        if not identity_matches():
            return desired
    written, kept = False, None
    if move_body is not None and not move_correct:
        kept = target['id'] if target else None
        api.upsert_state(pr['number'], desired, move_body, kept)
        written = True
    elif holder is not None and (not record_correct or not standing_correct):
        # Same words, current record — carried by the announcement whose words
        # match this state when there is one, so the record never sits under
        # the wrong instruction; otherwise by the holder. The earlier engine's
        # standing comment gets the pointer at the description.
        carrier = target if (move_body is not None and target is not None) else holder
        text = move_body if carrier is target and move_body is not None else (
            _visible(carrier['body']) if MOVE_MARKER in carrier['body'] else POINTER)
        kept = carrier['id']
        api.upsert_state(pr['number'], desired, text, kept)
        written = True
    # Every standing comment that is not the record holder is noise, and goes:
    # once a move comment exists, all of them; before that, all but the one
    # carrying the record.
    if kept is None and not written and holder is not None and not bot_comments(pr, MOVE_MARKER):
        kept = holder['id']
    superseded = [c for c in standing if c['id'] != kept]
    for old in superseded:
        try:
            api.delete_comment(old['id'])
        except GitHubError:
            print(f"PR #{pr['number']}: could not remove an old state comment", file=sys.stderr)
    if (written or superseded) and not identity_matches():
        return desired
    try:
        api.set_state_label(pr['number'], result['state'], pr.get('labels', []))
    except GitHubError:
        # The labels were read from a snapshot that another run reconciling this
        # author can invalidate, and removing a label that is already gone is a
        # 404. The state is in the status and the description either way.
        print(f"PR #{pr['number']}: could not set the state label; the status and description still carry the state",
              file=sys.stderr)
    try:
        api.set_label(pr['number'], WAIVED_LABEL, bool(result.get('waived')), pr.get('labels', []))
    except GitHubError:
        # The repository may not have the label yet. Say so and carry on: the
        # waiver is already in the status and the description, and one missing
        # label must not abort the remaining pull requests.
        print(f"PR #{pr['number']}: could not set {WAIVED_LABEL}; create the label to see waivers in listings",
              file=sys.stderr)
    if missing:
        room = max(0, 15 - len(requested))
        if len(missing) > room:
            print(f"PR #{pr['number']}: reviewer request capacity reached; "
                  f"{len(missing) - room} request(s) deferred", file=sys.stderr)
        if room:
            if not identity_matches():
                return desired
            try:
                api.request_reviewers(pr['number'], missing[:room])
            except GitHubError:
                # The room left over is read from a snapshot that another run
                # reconciling this author can invalidate, and GitHub refuses a
                # request past its own cap. The reviewers are already named in
                # the description and the status, and one refusal must not abort
                # the remaining pull requests.
                print(f"PR #{pr['number']}: could not request {', '.join(missing[:room])}; "
                      f"the description still names them", file=sys.stderr)

    if not actionable:
        if identity_matches():
            api.post_status(pr['head'], result['status'], result['state'])
        return desired

    # Review decisions and comments can change without changing the commit SHA.
    expected = [dict(p, controller_state=desired) if p['number'] == pr['number'] else p for p in candidates]
    finish(expected)
    return desired


def age(since, now):
    if not since:
        return 'not recorded'
    try:
        seconds = max(0, (datetime.fromisoformat(now.replace('Z', '+00:00'))
                          - datetime.fromisoformat(since.replace('Z', '+00:00'))).total_seconds())
    except (ValueError, TypeError):
        return 'unknown'
    hours = int(seconds // 3600)
    return f'{hours // 24}d {hours % 24}h' if hours >= 24 else f'{hours}h'


def cell(value):
    return str(value).replace('&', '&amp;').replace('<', '&lt;').replace('>', '&gt;').replace('|', '&#124;').replace('\n', ' ')


def selected_rows(rows, user=None):
    return [r for r in rows if not user or r['author'].lower() == user.lower()
            or user.lower() in [x.lower() for x in r.get('reviewers', [])]]


def render_report(rows, now, user=None):
    lines = [f'# PR Hygiene — {now}', '',
             'Read-only snapshot. Waiting time is for the current actionable review cycle.', '',
             '| PR | Author | Areas | State | Awaiting review | Next action |',
             '| --- | --- | --- | --- | --- | --- |']
    for r in selected_rows(rows, user):
        next_action = '; '.join(r.get('blockers', [])) or 'Policy satisfied; check remaining GitHub gates'
        if r.get('reviewers'):
            next_action += '; reviewers: ' + ', '.join(r['reviewers'])
        link = f"[#{r['number']}: {cell(r.get('title', ''))}]({r.get('url', '')})"
        lines.append('| ' + ' | '.join([link, cell(r['author']), cell(', '.join(r.get('areas', []))),
                                      cell(r['state']), age(r.get('ready_since'), now), cell(next_action)]) + ' |')
    if not selected_rows(rows, user):
        lines += ['', 'No matching PRs.']
    return '\n'.join(lines) + '\n'


def telemetry_states(policy, snapshots, now, payload):
    """Per PR, what the review system last said about the head it is awaiting."""
    if not payload:
        return {}
    states = {}
    for pr in snapshots:
        state = telemetry.head_state(payload, policy['repository'], pr['number'],
                                     pr['head'], pr.get('head_seen_at'), now)
        if state:
            states[pr['number']] = {telemetry.BOT: state}
    return states


def evaluate_snapshots(policy, context, candidates, snapshots, now, payload=None):
    """Use the same policy decisions for local enforcement and combined reports."""
    states = telemetry_states(policy, snapshots, now, payload)
    admissions = admit(policy, candidates, now)
    # More than five persisted admissions is a race between two runs for one
    # author. admit() keeps the five oldest, deterministically, and the surplus
    # returns to waiting for a slot on its next write — marking the whole
    # queue an error instead held every pull request of that author.
    for author in sorted(admission_conflicts(policy, candidates)):
        print(f'{author}: more than five persisted admissions; keeping the five oldest', file=sys.stderr)
    rows = []
    head_counts = Counter(pr['head'] for pr in context)
    for pr in snapshots:
        result = evaluate(policy, pr, admissions.get(pr['number']), now, states.get(pr['number']))
        if head_counts[pr['head']] > 1:
            result.update(state='configuration-error', status='error', reviewers=[], ready_since=None)
            result['blockers'].append('Another open PR shares this head; commit-scoped status is ambiguous')
        result['repository'] = policy['repository']
        rows.append(result)
    return rows


def run(argv=None):
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('command', choices=['validate', 'codeowners', 'report', 'sync'])
    parser.add_argument('--repo', default='dashpay/platform')
    parser.add_argument('--policies-root', type=Path, default=POLICIES,
                        help='directory holding repositories.json and the policy files')
    parser.add_argument('--repository-root', type=Path,
                        help='checkout of the governed repository, for path and CODEOWNERS checks')
    parser.add_argument('--policy', type=Path, help='explicit policy file instead of the registry entry')
    parser.add_argument('--pr', type=int)
    parser.add_argument('--batch-size', type=int)
    parser.add_argument('--waiting-on-build', action='store_true',
                        help='reconcile only pull requests recorded as waiting on a build')
    parser.add_argument('--user')
    parser.add_argument('--format', choices=['markdown', 'json'], default='markdown')
    parser.add_argument('--check', action='store_true')
    parser.add_argument('--apply', action='store_true')
    args = parser.parse_args(argv)
    if args.pr is not None and args.pr <= 0:
        parser.error('--pr must be positive')
    if args.batch_size is not None and (args.batch_size < 1 or args.command != 'sync' or args.pr is not None):
        parser.error('--batch-size requires sync, a positive size, and no --pr')
    if args.apply and args.command != 'sync':
        parser.error('--apply is only valid for sync')
    if args.apply and args.policy:
        parser.error('apply requires the registered policy, not an explicit file')
    if args.command == 'codeowners' and args.check and not args.repository_root:
        parser.error('codeowners --check requires --repository-root')
    if args.waiting_on_build and (args.pr is not None or args.batch_size is not None):
        parser.error('waiting-on-build selects its own pull requests')
    if args.apply and (os.environ.get('GITHUB_ACTIONS') != 'true'
                       or os.environ.get('GITHUB_REPOSITORY') != args.repo
                       or os.environ.get('PR_REVIEW_AUTOMATION_ENABLED') != 'true'):
        parser.error('apply is restricted to the enabled repository Actions workflow')
    repository_root = args.repository_root.resolve() if args.repository_root else None
    try:
        registry = load_registry(args.policies_root)
        source = args.policy or policy_path(args.policies_root, entry_for(registry, args.repo))
        policy = json.loads(source.read_text())
        validate_policy(policy, repository_root if args.command == 'validate' else None)
        if repository_root is not None and args.command != 'validate':
            # The dedicated validate step reports a missing directory. Failing
            # the reconcile over it too would mark every open pull request an
            # error — and, as a required check, unmergeable — until a policy
            # change lands, for a directory someone has legitimately removed.
            for gone in missing_paths(policy, repository_root):
                print(f'Policy path {gone} is not in the target tree; reconciling anyway', file=sys.stderr)
        if policy['repository'] != args.repo:
            raise ValueError('Repository must match the registered policy')
    except (ValueError, OSError):
        if args.apply:
            # Broken configuration cannot identify its scope reliably. Revoke
            # known open heads using the independently authorized repository.
            api = GitHub(args.repo)
            try:
                known = api.open_prs()
            except GitHubError:
                known = []
                print('Unable to discover heads for configuration error statuses', file=sys.stderr)
            for pr in known:
                try:
                    api.post_status(pr['head'], 'error', 'Invalid policy configuration; inspect workflow log')
                except GitHubError:
                    print(f"PR #{pr['number']}: unable to publish configuration error status", file=sys.stderr)
        raise
    if args.command == 'validate':
        print('Policy schema and paths are valid.' if repository_root else 'Policy schema is valid.')
        unresolved = [(a['id'], a['unresolved']) for a in policy['areas'] if a.get('unresolved')]
        if unresolved:
            print('Activation blockers: ' + json.dumps(unresolved))
        return 0
    if args.command == 'codeowners':
        generated = codeowners(policy)
        if args.check:
            locations = [repository_root / path for path in
                         ('.github/CODEOWNERS', 'CODEOWNERS', 'docs/CODEOWNERS')]
            effective = next((path for path in locations if path.is_file()), locations[0])
            if effective.read_text() != generated:
                raise GitHubError('CODEOWNERS differs from canonical policy; regenerate it')
        else:
            print(generated, end='')
        return 0

    api = GitHub(args.repo)
    context, candidates, snapshots = collect(api, policy, args.pr, apply=args.apply,
                                           reconcile_author=args.command == 'sync', batch_size=args.batch_size,
                                           waiting_on_build=args.waiting_on_build)
    now = utc_now()
    # Read the review system's public page once per run, never inside evaluation:
    # that runs twice per publication and must give the same answer both times.
    payload = telemetry.fetch() if policy.get('bot_timeouts') else None
    rows = evaluate_snapshots(policy, context, candidates, snapshots, now, payload)
    nudged = 0
    failed = []
    checked_labels = []
    for pr, result in zip(snapshots, rows):
        if args.command == 'sync':
            try:
                if args.apply:
                    if not checked_labels:
                        # Say once which labels are missing. A POST creates one
                        # as a side effect, in a default colour — ugly, but a
                        # missing label must not mark every pull request an
                        # error under a required check.
                        for label in STATE_LABELS + (WAIVED_LABEL,):
                            try:
                                api.request('GET', f'repos/{args.repo}/labels/{label}')
                            except GitHubError:
                                print(f'Label {label} does not exist in {args.repo}; create it', file=sys.stderr)
                        checked_labels.append(True)
                    nudged += nudge(api, pr, result, NUDGES_PER_RUN - nudged)
                written = publish(api, policy, pr, result, context, args.apply, candidates=candidates)
                if written:
                    for candidate in candidates:
                        if candidate['number'] == pr['number']:
                            candidate['controller_state'] = written
            except GitHubError as error:
                # Every pull request selected gets its turn. Stopping at the
                # first failure left the rest with whatever status they had —
                # on a full pass, possibly a passing one from before the check
                # became the gate.
                failed.append(pr['number'])
                print(f"PR #{pr['number']}: {error}", file=sys.stderr)
                if args.apply:
                    try:
                        api.post_status(pr['head'], 'error', 'Policy reconciliation failed; inspect workflow log')
                    except GitHubError:
                        print(f"PR #{pr['number']}: unable to publish the failure either", file=sys.stderr)
    if failed:
        raise GitHubError(f"reconciliation failed for {', '.join(f'#{n}' for n in failed)}")
    # A sweep tidies what it passes; a run aimed at one pull request does not
    # go looking through the repository.
    if args.pr is None:
        clear_marks(api, policy, context, args.apply and args.command == 'sync')
    rows.sort(key=lambda r: (r['state'] != 'ready-for-human', r.get('ready_since') or now, r['number']))
    if args.format == 'json':
        print(json.dumps({'generated_at': now, 'pull_requests': selected_rows(rows, args.user)}, indent=2))
    else:
        print(render_report(rows, now, args.user), end='')
    return 0


if __name__ == '__main__':
    try:
        sys.exit(run())
    except (GitHubError, ValueError, KeyError, OSError) as exc:
        print(f'PR Hygiene error: {exc}', file=sys.stderr)
        sys.exit(1)
