"""Deterministic ownership, review evidence and admission decisions."""

import copy
import hashlib
import json
import re
from collections import defaultdict
from datetime import datetime, timedelta
from pathlib import Path

STATE_MARKER = 'platform-pr-review-state-v1'
NUDGE_MARKER = '<!-- pr-hygiene-nudge v1'
BOTS = {'thepastaclaw', 'coderabbitai', 'coderabbitai[bot]'}
# Which bots a repository actually runs is a property of that repository, not of
# the review rules: requiring a producer that never reports would never resolve.
REVIEW_BOTS = ('thepastaclaw', 'coderabbitai')
WRITE = {'write', 'maintain', 'admin'}

# A label says whose move it is, in a listing. A red build is visible there
# already, a satisfied policy shows as a green check, a draft is not being
# reviewed and an error is loud enough — none of those carries a label.
LABEL_FOR_STATE = {'waiting-bots': 'waiting-bots',
                   'waiting-self-review': 'waiting-self-review',
                   'waiting-author': 'waiting-self-review',
                   'too-many-open-prs': 'too-many-open-prs',
                   'ready-for-human': 'ready-for-human'}
STATE_LABELS = tuple(dict.fromkeys(LABEL_FOR_STATE.values()))
# Labels this controller used to set. Cleared wherever still seen, so a
# repository that has not deleted them yet does not show two states at once.
RETIRED_LABELS = ('waiting-slot', 'waiting-build', 'waiting-author', 'ready-to-merge')
MOVE_MARKER = '<!-- pr-hygiene:move'
CHECKLIST_START = '<!-- pr-hygiene:start -->'
CHECKLIST_END = '<!-- pr-hygiene:end -->'


def _time(value):
    parsed = datetime.fromisoformat(value.replace('Z', '+00:00'))
    if parsed.tzinfo is None:
        raise ValueError('Timestamp requires timezone')
    return parsed


def _fields(value, allowed, required):
    if not isinstance(value, dict) or set(value) - allowed or required - set(value):
        raise ValueError('Unknown or missing policy fields')


def _handles(values, nonempty=False):
    if not isinstance(values, list) or (nonempty and not values):
        raise ValueError('Expected handle list')
    seen = set()
    for handle in values:
        if not isinstance(handle, str) or not re.fullmatch(r'[A-Za-z0-9](?:[A-Za-z0-9-]{0,37}[A-Za-z0-9])?', handle):
            raise ValueError('Malformed GitHub handle')
        if handle.lower() in seen or handle.lower() in {'strophy', 'silvanassss'} | BOTS | {'copilot', 'dependabot'}:
            raise ValueError('Duplicate or excluded identity')
        seen.add(handle.lower())


def missing_paths(policy, root: Path):
    """Policy directories that are not in this checkout of the governed repository."""
    return sorted(prefix for area in policy['areas'] for prefix in area['paths']
                  if not (root / prefix).is_dir())


def validate_policy(policy, root: Path | None = None):
    keys = {'version', 'repository', 'fallback', 'max_active_prs', 'target_branches', 'areas'}
    _fields(policy, keys | {'required_bots', 'bot_timeouts'}, keys)
    timeouts = policy.get('bot_timeouts')
    if timeouts is not None:
        _fields(timeouts, {'nudge_after_hours', 'waive_after_hours'}, {'nudge_after_hours', 'waive_after_hours'})
        if any(type(timeouts[key]) is not int or not 1 <= timeouts[key] <= 168 for key in timeouts):
            raise ValueError('bot_timeouts must be whole hours between 1 and 168')
    bots = policy.get('required_bots', list(REVIEW_BOTS))
    if (not isinstance(bots, list) or len(set(bots)) != len(bots)
            or any(bot not in REVIEW_BOTS for bot in bots)):
        raise ValueError('required_bots must be a subset of ' + ', '.join(REVIEW_BOTS))
    if type(policy['version']) is not int or policy['version'] != 1:
        raise ValueError('Unsupported policy version')
    if not isinstance(policy['repository'], str) or not re.fullmatch(r'[\w.-]+/[\w.-]+', policy['repository']):
        raise ValueError('Invalid repository')
    if type(policy['max_active_prs']) is not int or policy['max_active_prs'] != 5:
        raise ValueError('Expected five active slots')
    branches = policy['target_branches']
    if not isinstance(branches, list) or not branches or any(not isinstance(x, str) or not x.strip() for x in branches) or len(set(branches)) != len(branches):
        raise ValueError('Invalid target branches')
    _fields(policy['fallback'], {'owners', 'reviewers'}, {'owners', 'reviewers'})
    _handles(policy['fallback']['owners'], True)
    _handles(policy['fallback']['reviewers'])
    if not isinstance(policy['areas'], list):
        raise ValueError('Expected areas list')
    names, prefixes = set(), []
    for area in policy['areas']:
        _fields(area, {'id', 'paths', 'owners', 'reviewers', 'unresolved', 'metadata'}, {'id', 'paths', 'owners', 'reviewers'})
        if not isinstance(area['id'], str) or not re.fullmatch(r'[a-z0-9][a-z0-9-]*', area['id']) or area['id'] in names or area['id'] == 'fallback':
            raise ValueError('Invalid or duplicate area id')
        names.add(area['id'])
        _handles(area['owners'])
        if not area['owners'] and not area.get('unresolved'):
            raise ValueError('Missing owners require an explicit unresolved identity')
        _handles(area['reviewers'])
        if {x.lower() for x in area['owners']} & {x.lower() for x in area['reviewers']}:
            raise ValueError('Owner and reviewer roles overlap')
        if 'unresolved' in area and (not isinstance(area['unresolved'], list) or any(not isinstance(x, str) or not x.strip() for x in area['unresolved'])):
            raise ValueError('Invalid unresolved identities')
        if 'metadata' in area and not isinstance(area['metadata'], dict):
            raise ValueError('Invalid area metadata')
        if not isinstance(area['paths'], list) or not area['paths']:
            raise ValueError('Expected literal directory prefixes')
        for prefix in area['paths']:
            if not isinstance(prefix, str) or (prefix != '' and not re.fullmatch(r'(?:[A-Za-z0-9_.-]+/)+', prefix)) or any(x in {'.', '..'} for x in prefix.split('/')):
                raise ValueError('Invalid literal directory prefix')
            if any(prefix.startswith(other) or other.startswith(prefix) for other in prefixes):
                raise ValueError('Overlapping directory prefixes')
            if root is not None and not (root / prefix).is_dir():
                raise ValueError(f'Missing policy directory: {prefix}')
            prefixes.append(prefix)


def codeowners(policy):
    """A CODEOWNERS file with no rules, on purpose.

    A CODEOWNERS with rules makes GitHub request every owner the moment a pull
    request is opened, before the bots have reported or the author has read the
    diff — the noise this controller exists to prevent, and nothing switches it
    off short of a rules-free file. The file stays so anyone looking for who
    reviews what finds the answer.
    """
    validate_policy(policy)
    name = policy['repository'].split('/')[1]
    return '\n'.join([
        f'# Generated from dashpay/stale_prs_are_bad policies/{name}.json; do not edit by hand.',
        '#',
        '# Review routing for this repository is controlled by PR Hygiene:',
        f'#   https://github.com/dashpay/stale_prs_are_bad/blob/master/policies/{name}.json',
        '#',
        '# This file has no rules on purpose. Reviewers are requested by the policy',
        '# once the bots have reported and the author has self-reviewed, not by',
        '# GitHub the moment a pull request is opened.',
    ]) + '\n'


def effective_admission(pr):
    recorded = (pr.get('controller_state') or {}).get('admitted_at')
    inactive = pr.get('lifecycle_at')
    if recorded and inactive and _time(recorded) <= _time(inactive):
        return None
    return recorded


def admit(policy, prs, nowISO):
    validate_policy(policy)
    _time(nowISO)
    grouped = defaultdict(list)
    for pr in prs:
        if pr['state'] == 'open' and not pr['draft'] and pr['base'] in policy['target_branches']:
            grouped[pr['author'].lower()].append(pr)
    result = {}
    for candidates in grouped.values():
        def order(pr):
            admitted = effective_admission(pr)
            return (0, _time(admitted), pr['number']) if admitted else (1, _time(pr['created_at']), pr['number'])
        for pr in sorted(candidates, key=order)[:policy['max_active_prs']]:
            result[pr['number']] = effective_admission(pr) or nowISO
    return result


def fingerprint(pr):
    relevant = copy.deepcopy(pr)
    # The description carries this controller's own checklist, an effect.
    for name in ('controller_state', 'controller_comment_id', 'labels', 'requested_reviewers', 'build', 'body'):
        relevant.pop(name, None)
    # This controller's own comments are effects, not evidence: counting them
    # would make writing one look like the world changed underneath the write.
    relevant['comments'] = [x for x in relevant.get('comments', []) if not (
        x.get('user', '').lower() == 'github-actions[bot]' and
        (x.get('body', '').startswith(f'<!-- {STATE_MARKER}')
         or x.get('body', '').startswith(NUDGE_MARKER)))]
    for name in ('comments', 'reviews', 'threads', 'files'):
        if name in relevant:
            relevant[name] = sorted(relevant[name], key=lambda x: json.dumps(x, sort_keys=True))
    return hashlib.sha256(json.dumps(relevant, sort_keys=True, separators=(',', ':')).encode()).hexdigest()


def _latest_reviews(reviews):
    latest = {}
    for review in sorted(reviews, key=lambda x: (_time(x['submitted_at']), x['id'])):
        state = review['state'].upper()
        if state in {'APPROVED', 'CHANGES_REQUESTED', 'DISMISSED'}:
            latest[review['user'].lower()] = review
    return latest


# The two strings a CodeRabbit comment can carry that change an outcome. The
# caller workflow tests event bodies for these before starting a run, so they
# are shared rather than written twice: a marker that changed here and not
# there would stop receipts waking the controller, and nothing would fail.
RECEIPT_MARKER = 'final_review_risk_coverage'
RATE_LIMITED = '<!-- This is an auto-generated comment: rate limited by coderabbit.ai -->'
RATE_LIMITED_MARKER = 'rate limited by coderabbit.ai'


def _may_object(permissions, user):
    """Whether this person's objection counts.

    An answer that never arrived is kept rather than discarded. Everywhere else
    an unreadable permission holds a pull request back; dropping an objection on
    the strength of it would be the one place the same uncertainty let one
    through, and a dropped objection is invisible to whoever raised it.
    """
    return permissions.get(user) in WRITE or permissions.get(user) is None


def _rabbit_receipt(body, head):
    # The producer embeds a JSON object directly after its named HTML marker.
    for marker in re.finditer(r'(?m)^<!-- ' + re.escape(RECEIPT_MARKER) + r':\s*', body):
        try:
            value, end = json.JSONDecoder().raw_decode(body[marker.end():])
        except (ValueError, TypeError):
            continue
        if not re.match(r'\s*-->[ \t]*(?:\r?\n|$)', body[marker.end() + end:]):
            continue
        if isinstance(value, dict) and value.get('kind') == 'reviewed' and value.get('coveredCommitId') == head:
            return True
    return False



def _hours(stamp, nowISO):
    return (_time(nowISO) - _time(stamp)).total_seconds() / 3600


def skipped_by(comments, permissions, head_seen_at):
    """Who told this controller not to wait for the bots on this head, and when.

    Any writer may, the author included: whoever reviews next sees who did.
    Bare `/skip-bots` covers the head that has a status by then, exactly as the
    bare attestation does, so a skip written for an earlier head cannot carry.
    """
    if not head_seen_at:
        return None
    skips = [(c['created_at'], c['user']) for c in comments
             if c['body'].strip() == '/skip-bots' and c['created_at'] == c['updated_at']
             and permissions.get(c['user'].lower()) in WRITE
             and _time(c['created_at']) > _time(head_seen_at)]
    if not skips:
        return None
    when, user = min(skips, key=lambda item: _time(item[0]))
    return {'user': user, 'at': when}


def nudged_at(comments, bot, head):
    """When this controller last asked a bot to look at exactly this head."""
    marker = f'{NUDGE_MARKER} bot={bot} sha={head} -->'
    stamps = [c['created_at'] for c in comments
              if c['user'].lower() == 'github-actions[bot]' and marker in c['body']]
    return max(stamps, key=_time) if stamps else None


def rate_limited_at(comments, head_seen_at):
    """When CodeRabbit last said it was rate limited, after this head appeared."""
    if not head_seen_at:
        return None
    stamps = [c['created_at'] for c in comments
              if c['user'].lower() in {'coderabbitai', 'coderabbitai[bot]'} and RATE_LIMITED in c['body']
              and _time(c['created_at']) >= _time(head_seen_at)]
    return max(stamps, key=_time) if stamps else None


def bot_schedule(policy, pr, bot, nowISO, telemetry_state=None):
    """Whether to nudge a missing bot now, and whether to stop requiring it.

    Positive evidence that a review is in flight delays both; the absence of
    evidence delays neither, so a stale or hostile status page can cost one
    extra nudge but can never hold a pull request back.
    """
    timeouts = policy.get('bot_timeouts')
    seen = pr.get('head_seen_at')
    if not timeouts or not seen:
        return {'nudge': False, 'waived_at': None}
    waited = _hours(seen, nowISO)
    nudge_after, waive_after = timeouts['nudge_after_hours'], timeouts['waive_after_hours']
    already = nudged_at(pr['comments'], bot, pr['head'])

    limited = rate_limited_at(pr['comments'], seen) if bot == 'coderabbitai' else None
    if limited is not None:
        # CodeRabbit announced its own limit and documents this exact retry.
        due = _hours(limited, nowISO) >= 1
    elif telemetry_state == 'failed':
        due = True            # no receipt is ever coming for this head
    elif telemetry_state in {'running', 'queued'}:
        due = False           # in flight: nudging would only add load
    else:
        due = waited >= nudge_after

    # A review demonstrably still running may finish late; nothing else waits.
    # The moment a waiver takes effect is a property of the head, not of the run
    # that noticed it: an instant that moved with the clock would be later than
    # every comment, and the author's self-review could never satisfy it.
    # Nothing here consults the status page, so no third party can hold a pull
    # request back by claiming a review is still running.
    due_at = (_time(seen) + timedelta(hours=waive_after)).isoformat().replace('+00:00', 'Z')
    waived = waited >= waive_after
    return {'nudge': due and already is None and not waived, 'waived_at': due_at if waived else None}


def _checklist(**items):
    """Every requirement, met or not, in the order the verdict weighs them."""
    return [dict(item=name, **facts) for name, facts in items.items()]


def evaluate(policy, pr, admitted_at, nowISO, telemetry_states=None):
    result = {k: pr.get(k) for k in ('number', 'head', 'author', 'title', 'url')}
    result.update(state='configuration-error', status='error', blockers=[], reviewers=[], areas=[],
                  ready_since=None, admitted_at=admitted_at, bot_completed_at=None, self_reviewed_at=None,
                  nudge=[], waived=[], approvals=[], objections=[], checklist=[])

    # This status is a required check, so it passes only when the policy is
    # satisfied. Everything still waiting is pending — not red, because an
    # open pull request spends most of its life waiting and red the whole way
    # would hide the one red that matters — and only a configuration problem
    # someone must fix is an error.
    # A waiver is reported wherever the pull request ends up, not only where it
    # was granted: whoever reads the status has to know a bot was given up on.
    notes = []

    def stop(state, *reasons, status='pending'):
        result.update(state=state, status=status, blockers=list(reasons) + notes)
        return result

    try:
        validate_policy(policy)
        _time(nowISO)
        required = {'number','author','head','base','base_sha','created_at','draft','state','files','reviews','comments','threads','permissions','complete','build'}
        if required - set(pr) or not pr['complete']:
            return stop('configuration-error', 'Incomplete GitHub snapshot', status='error')
        if not re.fullmatch(r'[0-9a-f]{40}', pr['head']):
            return stop('configuration-error', 'Invalid head SHA', status='error')
        if pr['state'] != 'open' or pr['base'] not in policy['target_branches']:
            return stop('configuration-error', 'PR is outside the active policy scope')
        if pr['draft']:
            return stop('draft', 'Draft PR does not occupy a review slot')
        if admitted_at:
            _time(admitted_at)
        if not pr['files']:
            return stop('configuration-error', 'No changed-file evidence', status='error')
        touched, files_by_area = {}, {}
        for file in pr['files']:
            for path in {file['filename'], file.get('previous_filename', file['filename'])}:
                if not isinstance(path, str) or path.startswith('/') or any(x in {'.','..',''} for x in path.split('/')):
                    raise ValueError('Invalid changed path')
                area = next((a for a in policy['areas'] if any(path.startswith(p) for p in a['paths'])), None)
                area = area or dict(policy['fallback'], id='fallback')
                touched[area['id']] = area
                files_by_area.setdefault(area['id'], set()).add(path)
        result['areas'] = sorted(touched)
        permissions = {k.lower(): v for k,v in pr['permissions'].items()}
        people = {x.lower(): x for a in touched.values() for x in a['owners'] + a['reviewers']}
        for area in touched.values():
            if area.get('unresolved'):
                return stop('configuration-error', 'Unresolved identities in ' + area['id'], status='error')
        unverified = sorted(x for x in people if permissions.get(x) is None)
        if unverified:
            # Not the same as lacking access: the answer never arrived.
            return stop('configuration-error',
                        'Cannot verify write access for ' + ', '.join(people.get(x, x) for x in unverified),
                        status='error')
        if any(permissions.get(x) not in WRITE for x in people):
            return stop('configuration-error', 'An assigned owner/reviewer lacks verified write access', status='error')
        # From here on every requirement is computed, whether or not an earlier
        # one is met: the checklist shows the whole road. The verdict is still
        # the first unmet requirement in this order, and fields that used to be
        # set only past a gate are still set only past it.
        first = None

        def gate(state, *reasons):
            nonlocal first
            if first is None:
                first = (state, list(reasons))

        required = set(policy.get('required_bots', REVIEW_BOTS))
        seen_at = pr.get('head_seen_at')
        latest = _latest_reviews(pr['reviews'])
        bot_blocks = [r for u,r in latest.items() if u in BOTS and r['state'].upper() == 'CHANGES_REQUESTED']
        bot_threads = [t for t in pr['threads'] if not t['is_resolved'] and t['author'].lower() in BOTS]
        pasta, rabbit = [], []
        for review in pr['reviews']:
            user, state = review['user'].lower(), review['state'].upper()
            if review['commit_id'] != pr['head']:
                continue
            if user == 'thepastaclaw' and state in {'APPROVED','COMMENTED'} and re.search(
                r'(?m)^<!-- thepastaclaw-review-phase v1 phase=final sha=' + re.escape(pr['head']) + r'(?:\s+[^<>]*?)?\s*-->', review['body']):
                pasta.append(review['submitted_at'])
            if user in {'coderabbitai','coderabbitai[bot]'} and state == 'APPROVED':
                rabbit.append(review['submitted_at'])
        for comment in pr['comments']:
            if comment['user'].lower() in {'coderabbitai','coderabbitai[bot]'} and _rabbit_receipt(comment['body'], pr['head']):
                rabbit.append(comment['updated_at'])
        # A bot that requested changes on an earlier head has not reported on
        # this one: that is the shape a nudge and a waiver exist for. An
        # objection raised against the current head is a report, and blocks.
        bot_blocks = [r for r in bot_blocks if r.get('commit_id') == pr['head']]
        receipts = {'thepastaclaw': pasta, 'coderabbitai': rabbit}
        # A bot that objected to this head, or left a thread open, has
        # reported. It is not missing, so nothing waives it: the objection is
        # answered by dismissing the review or resolving the thread, in the
        # open, not by telling this controller to stop waiting.
        heard = {u for u, r in latest.items() if u in BOTS and r.get('commit_id') == pr['head']
                 and r['state'].upper() == 'CHANGES_REQUESTED'}
        heard |= {t['author'].lower().removesuffix('[bot]') for t in bot_threads}
        missing = [bot for bot in sorted(required) if not receipts[bot] and bot not in heard]
        waived, reasons = {}, []
        outstanding = bool(bot_blocks or bot_threads)
        skip = skipped_by(pr['comments'], permissions, pr.get('head_seen_at'))
        for bot in missing:
            plan = bot_schedule(policy, pr, bot, nowISO, (telemetry_states or {}).get(bot))
            if skip:
                # A human decided the bots are not coming. That is a waiver
                # with a name on it, and the name is what keeps it honest. A
                # waiver that had already taken effect keeps its earlier
                # instant, or a late skip would send an attested pull request
                # back for a fresh attestation.
                instants = [skip['at']] + ([plan['waived_at']] if plan['waived_at'] else [])
                waived[bot] = min(instants, key=_time)
                result['skipped_by'] = skip['user']
                continue
            # Asking for a review while the pull request owes the bots an answer
            # anyway would spend someone else's capacity on nothing.
            if plan['nudge'] and not outstanding:
                result['nudge'].append(bot)
            # A repository that sets timeouts has accepted that a silent bot is
            # eventually proceeded without, including when it is the only one.
            # The label and the stated blocker are what keep that honest.
            if plan['waived_at']:
                waived[bot] = plan['waived_at']
            else:
                reasons.append(f'{bot} has not reported for the current head')
        result['waived'] = sorted(waived)
        notes.extend((f"Proceeded without {bot}: skipped by @{skip['user']}" if skip
                      else f'Proceeded without {bot}: no review within the configured window')
                     for bot in sorted(waived))
        # Everything about each bot, not the first thing: a bot can have
        # reported and still hold the box open with an objection or a thread,
        # and a line that said only "✓" left the open box unexplained.
        objecting = {u.removesuffix('[bot]') for u, r in latest.items() if u in BOTS
                     and r.get('commit_id') == pr['head'] and r['state'].upper() == 'CHANGES_REQUESTED'}
        threads_by = {}
        for thread in bot_threads:
            who = thread['author'].lower().removesuffix('[bot]')
            threads_by[who] = threads_by.get(who, 0) + 1
        bot_lines = []
        for bot in sorted(set(required) | set(objecting) | set(threads_by)):
            parts = []
            if bot in waived:
                parts.append(f"skipped by {skip['user']}" if skip else 'skipped after the window')
            elif receipts.get(bot):
                parts.append('✓')
            elif bot not in objecting and bot not in threads_by:
                parts.append('not yet')
            if bot in objecting:
                parts.append('requested changes — dismiss the review or push a fix')
            if bot in threads_by:
                n = threads_by[bot]
                parts.append(f"{n} thread{'s' if n > 1 else ''} unresolved — resolve {'them' if n > 1 else 'it'}")
            bot_lines.append(f"{bot} {', '.join(parts)}")
        # Waiting for the bots means a bot has not reported. Once it has, what
        # it said is the author's to answer — and a pull request cannot be
        # waiting for a bot that was skipped, which is what those two labels
        # said together on a third of the queue.
        findings = []
        for review in bot_blocks:
            who = review['user'].lower().removesuffix('[bot]')
            findings.append(f'{who} requested changes on this head; dismiss the review or push a fix')
        for bot in sorted({th['author'].lower().removesuffix('[bot]') for th in bot_threads}):
            findings.append(f'{bot} left review threads unresolved; resolve them')
        if reasons:
            gate('waiting-bots', *(reasons + findings))
        elif findings:
            gate('waiting-author', *findings)
        bots_done = first is None
        # Self-review must follow whichever producers this repository runs. With
        # none, the author's own attestation is the only gate.
        # A waiver is itself an event the author's self-review must follow, so a
        # attestation written before the bots were given up on cannot count.
        # Every receipt raises the floor an attestation must clear, whether or
        # not this controller can see a finding in it. A bot states a blocker
        # in the prose of the receipt itself — "Add signer support or defer
        # selecting these keys before merging", no thread, no changes request
        # — so "it had nothing to say" is not a thing that can be read off the
        # evidence, and guessing it merges pull requests nobody has read.
        instants = pasta + rabbit + list(waived.values())
        completed = max(instants, key=_time) if instants else pr['created_at']
        floor_at = completed
        if bots_done:
            result['bot_completed_at'] = completed
        # `/self-reviewed <sha>` names the commit it covers. Bare `/self-reviewed`
        # means "everything pushed so far", which is only safe once this head has
        # a status: that timestamp cannot be moved, so an attestation written
        # before the last push can never be reused for it.
        seen = pr.get('head_seen_at')
        # An attestation is the author saying it, wherever they say it: a
        # comment, or the body of their own review — which is one action from
        # the diff they are attesting to. A review carries no edit history
        # here, and needs none: editing an old one cannot move its timestamp
        # forward, and the floor below is what a later edit would have to beat.
        written = [dict(user=c['user'], body=c['body'], at=c['created_at'])
                   for c in pr['comments'] if c['created_at'] == c['updated_at']]
        written += [dict(user=r['user'], body=r['body'] or '', at=r['submitted_at']) for r in pr['reviews']]
        attestations = []
        for comment in written:
            if comment['user'].lower() != pr['author'].lower():
                continue
            body = comment['body'].strip()
            if body == '/self-reviewed ' + pr['head']:
                floor = floor_at
            elif body == '/self-reviewed' and seen:
                floor = max(floor_at, seen, key=_time)
            else:
                continue
            if _time(comment['at']) > _time(floor):
                attestations.append(comment['at'])
        if not attestations and pr.get('author_is_bot'):
            # Copilot and dependabot cannot post an attestation. Their pull
            # requests never own an area, so the eligible approval they need
            # anyway is what stands in for it.
            # Stands in for an attestation, so it is dated when the bots were
            # done — not at the floor, which can predate an objection the pull
            # request has already answered.
            attestations = [seen or completed]
        if not attestations:
            gate('waiting-self-review', 'Author must post /self-reviewed ' + pr['head'] + ' after bot completion')
        self_time = max(attestations, key=_time) if attestations else None
        if first is None:
            result['self_reviewed_at'] = self_time
        objectors, objection_lines = {}, []
        for user, review in latest.items():
            if user not in BOTS and _may_object(permissions, user) and review['state'].upper() == 'CHANGES_REQUESTED':
                objectors[user] = review['submitted_at']
                objection_lines.append(f'{review["user"]} requested changes')
        for thread in pr['threads']:
            if thread['is_resolved'] or thread['author'].lower() in BOTS:
                continue
            # Everyone who spoke in the thread, not only whoever opened it: an
            # author's own thread carrying a reviewer's objection in reply is
            # that reviewer's objection. The author's own words are not one.
            for voice in thread.get('voices') or [dict(user=thread['author'], created_at=thread['created_at'])]:
                user = voice['user'].lower()
                if user in BOTS or user == pr['author'].lower() or not _may_object(permissions, user):
                    continue
                if user not in objectors:
                    objection_lines.append(f"{voice['user']} left a review thread unresolved")
                objectors[user] = max(objectors.get(user, voice['created_at']), voice['created_at'], key=_time)
        if first is None:
            result['objections'] = objection_lines
        author = pr['author'].lower()
        needed = set()
        approvals = []
        for area in touched.values():
            if author in {x.lower() for x in area['owners']}:
                approvals.append({'area': area['id'], 'files': sorted(files_by_area.get(area['id'], ())),
                                  'approvers': [], 'approved_by': [], 'owned': True})
                continue
            eligible = {x.lower() for x in area['owners'] + area['reviewers']} - {author}
            approved_by = sorted(people[u] for u in eligible
                                 if u in latest and latest[u]['state'].upper() == 'APPROVED' and latest[u]['commit_id'] == pr['head'])
            # Recorded either way: an approval that already covers an area is
            # as much of the answer as the one still missing, and the author
            # seeing "romchornyi approved swift-sdk" beside "nobody has
            # approved .github/" is what tells them what they are waiting for.
            approvals.append({
                'area': area['id'], 'files': sorted(files_by_area.get(area['id'], ())),
                'approvers': sorted(people[u] for u in eligible), 'approved_by': approved_by, 'owned': False})
            if not approved_by:
                needed.update(eligible)
        if first is None:
            result['approvals'] = approvals
        # An objection at or after the attestation is the author's to answer;
        # one before it has been answered and waits on the objector.
        unanswered = {u for u, at in objectors.items() if self_time is None or _time(at) >= _time(self_time)}
        if self_time is not None and unanswered:
            gate('waiting-author', 'Author response is required after the latest human objection')
        needed.update(u for u in objectors if u != author and _may_object(permissions, u) and u not in BOTS)
        human = bool(needed or objectors)
        previous = pr.get('controller_state') or {}
        # Latching on the recorded state, not on ready_since: a pull request
        # can be ready with no ready_since yet, on the first run that makes
        # it ready, and that one would otherwise be sent back.
        # Ever ready for this head, not still ready. The recorded state is
        # rewritten on every run, so a pull request that passed through any
        # other state would come back needing a green build again — and one
        # unresolved bot thread is enough to do that.
        was_ready = bool(pr.get('ready_published')
                         or (previous.get('state') == 'ready-for-human' and previous.get('head') == pr['head']))
        build = pr['build']
        if human:
            if not admitted_at:
                # Five at a time per author is a limit on human attention, so
                # it applies here, where a human would be asked, and not to a
                # pull request that needs none.
                gate('too-many-open-prs', f"More than {policy['max_active_prs']} open pull requests; this one waits until one merges")
            if not was_ready and build != 'green':
                # Green before a human is asked; red afterwards does not take it
                # back, so a flake cannot withdraw a review request already sent
                # or drop its author out of a slot.
                gate('waiting-build',
                     'The build must pass before a human is asked' if build == 'failed'
                     else 'Waiting for the build to finish')
            if first is None:
                result['reviewers'] = sorted(people.get(u,u) for u in needed)
                if was_ready and previous.get('ready_since'):
                    _time(previous['ready_since'])
                    result['ready_since'] = previous['ready_since']
                elif previous.get('admitted_at'):
                    result['ready_since'] = nowISO
            gate('ready-for-human', 'Human approval or objection resolution is required')
        elif build != 'green':
            # The owner path asks no human, so nothing else looks at the build.
            gate('waiting-build',
                 'The build must pass before this can merge' if build == 'failed'
                 else 'Waiting for the build to finish')
        # In the order the verdict weighs them, so the first unchecked line is
        # the state. Attested-and-nothing-objected-since is what checks
        # self-review: an objection at or after the attestation is the
        # author's to answer, and the box stays open.
        result['checklist'] = _checklist(
            bots=dict(done=bots_done, lines=bot_lines, skippable=any(bot not in waived for bot in missing)),
            self_review=dict(done=bool(attestations) and not (self_time is not None and unanswered),
                             bot_author=bool(pr.get('author_is_bot')),
                             address=[line for line in objection_lines
                                      if line.split(' ', 1)[0].lower() in unanswered or self_time is None]),
            slot=dict(done=not human or bool(admitted_at), limit=policy['max_active_prs']),
            build=dict(done=build == 'green', state=build, latched=was_ready and human),
            approvals=dict(done=not human, areas=approvals,
                           awaiting=[line for line in objection_lines
                                     if self_time is not None and line.split(' ', 1)[0].lower() not in unanswered]))
        if first is not None:
            state, reasons = first
            return stop(state, *reasons)
        return stop('ready-to-merge', 'All policy requirements are satisfied', status='success')
    except (ValueError, TypeError, KeyError) as error:
        return stop('configuration-error', str(error), status='error')
