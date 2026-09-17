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
        if handle.lower() in seen or handle.lower() in {'strophy', 'silvanassss'}:
            raise ValueError('Duplicate or excluded identity')
        seen.add(handle.lower())


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
    validate_policy(policy)
    name = policy['repository'].split('/')[1]
    lines = [f'# Generated from dashpay/stale_prs_are_bad policies/{name}.json; do not edit by hand.',
             '# Owners and reviewers are combined here for native review routing.']
    # GitHub honours only the last matching pattern, so a whole-repository area
    # would silently override a preceding fallback `*` line; emit one or the other.
    if not any('' in area['paths'] and (area['owners'] or area['reviewers']) for area in policy['areas']):
        lines.append('* ' + ' '.join('@' + x for x in policy['fallback']['owners'] + policy['fallback']['reviewers']))
    for area in policy['areas']:
        if area.get('unresolved'):
            lines.append('# Unresolved ownership or reviewer identities; see responsibility documentation.')
        people = ' '.join('@' + x for x in area['owners'] + area['reviewers'])
        if people:
            lines.extend(('/' + path if path else '*') + ' ' + people for path in area['paths'])
    return '\n'.join(lines) + '\n'


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
    for name in ('controller_state', 'controller_comment_id', 'labels', 'requested_reviewers', 'build'):
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


def evaluate(policy, pr, admitted_at, nowISO, telemetry_states=None):
    result = {k: pr.get(k) for k in ('number', 'head', 'author', 'title', 'url')}
    result.update(state='configuration-error', status='error', blockers=[], reviewers=[], areas=[],
                  ready_since=None, admitted_at=admitted_at, bot_completed_at=None, self_reviewed_at=None,
                  nudge=[], waived=[])

    # A pull request that is progressing normally reports success with its state
    # in the description: a permanently amber check reads as something broken.
    # Only a configuration problem someone must fix is not green.
    # A waiver is reported wherever the pull request ends up, not only where it
    # was granted: whoever reads the status has to know a bot was given up on.
    notes = []

    def stop(state, *reasons, status='success'):
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
        if not admitted_at:
            return stop('waiting-slot', 'Waiting for one of five author review slots')
        _time(admitted_at)
        if not pr['files']:
            return stop('configuration-error', 'No changed-file evidence', status='error')
        touched = {}
        for file in pr['files']:
            for path in {file['filename'], file.get('previous_filename', file['filename'])}:
                if not isinstance(path, str) or path.startswith('/') or any(x in {'.','..',''} for x in path.split('/')):
                    raise ValueError('Invalid changed path')
                area = next((a for a in policy['areas'] if any(path.startswith(p) for p in a['paths'])), None)
                area = area or dict(policy['fallback'], id='fallback')
                touched[area['id']] = area
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
        required = set(policy.get('required_bots', REVIEW_BOTS))
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
        missing = [bot for bot in sorted(required) if not receipts[bot]]
        waived, reasons = {}, []
        outstanding = bool(bot_blocks or bot_threads)
        for bot in missing:
            plan = bot_schedule(policy, pr, bot, nowISO, (telemetry_states or {}).get(bot))
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
        notes.extend(f'Proceeded without {bot}: no review within the configured window'
                     for bot in sorted(waived))
        if reasons or bot_blocks or bot_threads:
            if bot_blocks: reasons.append('Bot changes request remains outstanding')
            if bot_threads: reasons.append('Bot review threads remain unresolved')
            return stop('waiting-bots', *reasons)
        # Self-review must follow whichever producers this repository runs. With
        # none, the author's own attestation is the only gate.
        # A waiver is itself an event the author's self-review must follow, so a
        # attestation written before the bots were given up on cannot count.
        instants = pasta + rabbit + list(waived.values())
        completed = max(instants, key=_time) if instants else pr['created_at']
        result['bot_completed_at'] = completed
        # `/self-reviewed <sha>` names the commit it covers. Bare `/self-reviewed`
        # means "everything pushed so far", which is only safe once this head has
        # a status: that timestamp cannot be moved, so an attestation written
        # before the last push can never be reused for it.
        seen = pr.get('head_seen_at')
        attestations = []
        for comment in pr['comments']:
            if comment['user'].lower() != pr['author'].lower() or comment['created_at'] != comment['updated_at']:
                continue
            body = comment['body'].strip()
            if body == '/self-reviewed ' + pr['head']:
                floor = completed
            elif body == '/self-reviewed' and seen:
                floor = max(completed, seen, key=_time)
            else:
                continue
            if _time(comment['created_at']) > _time(floor):
                attestations.append(comment['created_at'])
        if not attestations:
            return stop('waiting-self-review', 'Author must post /self-reviewed ' + pr['head'] + ' after bot completion')
        self_time = max(attestations,key=_time)
        result['self_reviewed_at'] = self_time
        objectors = {}
        for user, review in latest.items():
            if user not in BOTS and permissions.get(user) in WRITE and review['state'].upper() == 'CHANGES_REQUESTED':
                objectors[user] = review['submitted_at']
        for thread in pr['threads']:
            if not thread['is_resolved'] and thread['author'].lower() not in BOTS:
                user = thread['author'].lower()
                if permissions.get(user) not in WRITE:
                    continue
                objectors[user] = max(objectors.get(user, thread['created_at']), thread['created_at'],key=_time)
        if any(_time(value) >= _time(self_time) for value in objectors.values()):
            return stop('waiting-author', 'Author response is required after the latest human objection')
        author = pr['author'].lower()
        needed = set()
        for area in touched.values():
            if author in {x.lower() for x in area['owners']}:
                continue
            eligible = {x.lower() for x in area['owners'] + area['reviewers']} - {author}
            if not any(u in latest and latest[u]['state'].upper() == 'APPROVED' and latest[u]['commit_id'] == pr['head'] for u in eligible):
                needed.update(eligible)
        needed.update(u for u in objectors if u != author and permissions.get(u) in WRITE and u not in BOTS)
        if needed or objectors:
            previous = pr.get('controller_state') or {}
            # Latching on the recorded state, not on ready_since: a pull request
            # can be ready with no ready_since yet, on the first run that makes
            # it ready, and that one would otherwise be sent back.
            # Ever ready for this head, not still ready. The recorded state is
            # rewritten on every run, so a pull request that passed through any
            # other state would come back needing a green build again — and one
            # unresolved bot thread is enough to do that.
            was_ready = (pr.get('ready_published')
                         or (previous.get('state') == 'ready-for-human' and previous.get('head') == pr['head']))
            build = pr['build']
            if not was_ready and build != 'green':
                # Green before a human is asked; red afterwards does not take it
                # back, so a flake cannot withdraw a review request already sent
                # or drop its author out of a slot.
                return stop('waiting-build',
                            'The build must pass before a human is asked' if build == 'failed'
                            else 'Waiting for the build to finish')
            result['reviewers'] = sorted(people.get(u,u) for u in needed)
            if was_ready and previous.get('ready_since'):
                _time(previous['ready_since'])
                result['ready_since'] = previous['ready_since']
            elif previous.get('admitted_at'):
                result['ready_since'] = nowISO
            return stop('ready-for-human', 'Human approval or objection resolution is required')
        return stop('ready-to-merge', status='success')
    except (ValueError, TypeError, KeyError) as error:
        return stop('configuration-error', str(error), status='error')
