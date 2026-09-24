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
    _fields(policy, keys | {'required_bots', 'bot_timeouts', 'bot_authors'}, keys)
    _handles(policy.get('bot_authors', []))
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
    # A machine author needs no attestation because the approval it cannot do
    # without stands in for one. Let it own an area and it needs neither: the
    # owner exemption would merge its pull requests with nobody having read
    # them at all. Read after the handles are known to be handles — reaching
    # into them earlier turned a malformed policy from a reported
    # configuration error into a crash that left the whole run without a status.
    machines = {handle.lower() for handle in policy.get('bot_authors', [])}
    named = {handle.lower() for handle in policy['fallback']['owners'] + policy['fallback']['reviewers']}
    named |= {handle.lower() for area in policy['areas']
              for handle in area['owners'] + area['reviewers']}
    if machines & named:
        raise ValueError('A machine author cannot own or review: ' + ', '.join(sorted(machines & named)))


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


def diff_print(pr):
    """What a reviewer read: this pull request's own changes, whatever commit carries them.

    The changed-file list is taken against the merge base, so a commit that
    only brings the base forward leaves every entry identical, while anything
    else the merge carried appears as an entry that was not there. Both what
    each file holds and the patch that produced it are read: a conflict
    resolved by keeping your own side leaves the file byte for byte what the
    reviewer saw, and only the patch shows that it now also undoes what the
    base did.
    """
    files = pr.get('files') or []
    if any(not f.get('content') or not f.get('shape') for f in files):
        return None      # a read that cannot say what changed says nothing
    items = sorted((f['filename'], f.get('status') or '', f['content'], f['shape'],
                    f.get('previous_filename') or '')
                   for f in files)
    return hashlib.sha256(json.dumps(items, separators=(',', ':')).encode()).hexdigest()


def carried_heads(pr):
    """The commits whose review still applies here, newest last.

    A push that leaves the pull request's own diff untouched — a merge of the
    base, a rebase onto it — is not work anybody has to read again, so what
    the bots said about the commit before it still stands, and so does the
    author's attestation. Anything that changes the diff starts over.
    """
    record = pr.get('controller_diff') or {}
    print_now = diff_print(pr)
    if not print_now or record.get('diff') != print_now:
        return [pr['head']], pr.get('head_seen_at')
    kept = [h for h in (record.get('diff_heads') or []) if isinstance(h, str)][-20:]
    if pr['head'] not in kept:
        kept = kept + [pr['head']]
    # The moment the first of them was seen: an attestation written then was
    # written about this same diff.
    return kept, record.get('diff_seen') or pr.get('head_seen_at')


def fingerprint(pr):
    relevant = copy.deepcopy(pr)
    # The description carries this controller's own checklist, an effect.
    for name in ('controller_state', 'controller_comment_id', 'controller_diff',
                 'labels', 'requested_reviewers', 'build', 'body'):
        relevant.pop(name, None)
    # This controller's own comments are effects, not evidence: counting them
    # would make writing one look like the world changed underneath the write.
    # Who last edited a comment is not how a change is noticed — the time it
    # was edited is, and that is here. Keeping it would only make the print
    # depend on which route read the comment.
    for comment in relevant.get('comments', []):
        comment.pop('edited_by', None)
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
# What this producer says about a pull request, and where. The risk block
# holds its findings and the marker for the commit they cover; its checks are
# a table of verdicts and a heading that counts them. Everything else in the
# comment is how it says it — a banner, the tips, a reworded explanation of a
# check that still passes — and none of that is a report.
RISK_BLOCK = re.compile(r'<!-- final_review_risk_start -->(.*?)<!-- final_review_risk_end -->', re.S)
VERDICT_ROW = re.compile(r'(?m)^\|([^|\n]+)\|([^|\n]+)\|')
VERDICT_HEADING = re.compile(r'(?m)^<summary>([^<\n]*(?:✅|❌|⚠️|🚫)[^<\n]*)</summary>')
# The phrase, however the author spells it. It is still the author writing it
# in their own words, so the spelling weakens nothing — and `/self-review`
# without the d has cost two people a merge already.
ATTESTATION = re.compile(r'/self[- ]?review(?:ed)?(?:\s+(?P<head>[0-9a-fA-F]{40}))?', re.IGNORECASE)
# The caller workflow starts a run on a comment that contains one of these.
# Every spelling the pattern above accepts has to contain one, or the phrase
# is read by the next sweep hours later instead of at once — which is the
# complaint that widened the pattern in the first place.
ATTESTATION_TRIGGERS = ('/self-review', '/selfreview', '/self review')
RATE_LIMITED = '<!-- This is an auto-generated comment: rate limited by coderabbit.ai -->'
RATE_LIMITED_MARKER = 'rate limited by coderabbit.ai'
RATE_LIMITED_END = '<!-- end of auto-generated comment: rate limited by coderabbit.ai -->'


def _may_object(permissions, user):
    """Whether this person's objection counts.

    An answer that never arrived is kept rather than discarded. Everywhere else
    an unreadable permission holds a pull request back; dropping an objection on
    the strength of it would be the one place the same uncertainty let one
    through, and a dropped objection is invisible to whoever raised it.
    """
    return permissions.get(user) in WRITE or permissions.get(user) is None


def receipt_print(comment):
    """What this producer said about the code, apart from how it said it.

    It keeps one comment and rewrites it — for a banner, for a re-run that
    found nothing, to reword the explanation of a check that still passes —
    and the time it was last written says only that it was written. Two
    authors were asked to attest a second time to reports that said nothing
    new, and this is what tells the two apart.

    Everything it states is read: its findings, the commit they cover, every
    check in its table and the heading that counts them. Only the prose
    explaining a verdict is left out, which is the column it rewrites. Read
    nothing at all, and there is nothing to compare — the time stands.
    """
    body = comment['body']
    said = list(RISK_BLOCK.findall(body))
    said += [f'{name.strip()}|{state.strip()}' for name, state in VERDICT_ROW.findall(body)
             if set(name.strip()) - set(': -')]
    said += [heading.strip() for heading in VERDICT_HEADING.findall(body)]
    if not said:
        return None
    # Per comment: two of them saying the same thing are still two reports.
    return hashlib.sha256(('\n'.join([str(comment.get('id', ''))] + said)).encode()).hexdigest()


def receipt_instant(pr, comment):
    """When this producer first said this, for this head.

    A block it has not changed is not a new report, however often the comment
    around it is rewritten. Anything in the block — a finding, a risk level,
    the commit it covers — makes it one.
    """
    said = receipt_print(comment)
    known = ((pr.get('controller_diff') or {}).get('receipts') or {})
    if said and isinstance(known.get(said), str):
        return known[said]
    return comment['updated_at']


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



def machine_author(policy, pr):
    """Whether this pull request was opened by something that cannot attest for itself.

    GitHub marks Copilot and dependabot as bots; the accounts a team runs its
    own automation from are ordinary users by every API, so the policy names
    them. Either way nobody is there to read the diff and say so, and the
    eligible approval such a pull request needs anyway stands in for it.
    """
    if pr.get('author_is_bot'):
        return True
    return (pr.get('author') or '').lower() in {h.lower() for h in policy.get('bot_authors', [])}


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


def nudged_at(comments, bot, heads):
    """When this controller last asked a bot to look at this work.

    Any commit carrying the same diff counts: the bot was asked about this
    code, and asking again because the base moved underneath it is noise.
    """
    markers = [f'{NUDGE_MARKER} bot={bot} sha={h} -->' for h in ([heads] if isinstance(heads, str) else heads)]
    stamps = [c['created_at'] for c in comments
              if c['user'].lower() == 'github-actions[bot]' and any(m in c['body'] for m in markers)]
    return max(stamps, key=_time) if stamps else None


def _limited_block(body):
    """The rate-limit notice itself, without the rest of the comment around it."""
    start = body.find(RATE_LIMITED)
    if start < 0:
        return ''
    end = body.find(RATE_LIMITED_END, start)
    # Without its end, the notice has no extent, and reading to the end of the
    # comment would let the walkthrough below it speak for the limit.
    return '' if end < 0 else body[start:end]


def rate_limited_at(comments, head_seen_at, head=None):
    """When CodeRabbit first said, about this head, that it was rate limited.

    It keeps one comment and edits it, so a notice about this head can sit
    under a creation date from the week the pull request opened; reading that
    date found nothing at all and the pull request waited the whole window.
    An edit counts as the bot's word only when the bot made it — anyone with
    write access can edit anyone's comment, and a stranger's edit of a stale
    notice would otherwise read as "it cannot review this head", with the
    report crediting the bot for a skip a person performed.

    The instant returned is a property of the head: the later of the notice
    and the head, never the moment of the edit that revealed it. An instant
    that moved with the clock would keep raising the floor an attestation has
    to clear, so a no-op edit would void one already given.

    The marker is in the body only while the limit stands: a review that
    succeeds later rewrites the comment without it.
    """
    if not head_seen_at:
        return None
    stamps = []
    for c in comments:
        if c['user'].lower() not in {'coderabbitai', 'coderabbitai[bot]'} or RATE_LIMITED not in c['body']:
            continue
        edited = c.get('updated_at') or c['created_at']
        # The notice names the commit it could not review, and that is what
        # says it is about this head. Its own timestamp does not: the bot can
        # say it before this controller has reported on the head, and on
        # dash-evo-tool#1021 it did, by four minutes — so the pull request
        # waited the whole window for a review the bot had already refused.
        about_this_head = _limited_block(c['body']).find(head) >= 0 if head else False
        if not about_this_head and _time(edited) < _time(head_seen_at):
            continue
        # Unedited, it is the bot's own words. Edited, only if the bot is who
        # edited it — and where that cannot be known, the ordinary window
        # applies rather than a skip nobody can attribute.
        if edited != c['created_at'] and (c.get('edited_by') or '').lower() not in {'coderabbitai', 'coderabbitai[bot]'}:
            continue
        stamps.append(max(c['created_at'], head_seen_at, key=_time))
    return min(stamps, key=_time) if stamps else None


def bot_schedule(policy, pr, bot, nowISO, telemetry_state=None):
    """Whether to nudge a missing bot now, and whether to stop requiring it.

    Positive evidence that a review is in flight delays both; the absence of
    evidence delays neither, so a stale or hostile status page can cost one
    extra nudge but can never hold a pull request back.
    """
    timeouts = policy.get('bot_timeouts')
    seen = pr.get('head_seen_at')
    if not timeouts or not seen:
        return {'nudge': False, 'waived_at': None, 'waived_reason': None}
    waited = _hours(seen, nowISO)
    nudge_after, waive_after = timeouts['nudge_after_hours'], timeouts['waive_after_hours']
    already = nudged_at(pr['comments'], bot, pr.get('reviewed_heads') or [pr['head']])

    limited = rate_limited_at(pr['comments'], seen, pr.get('head')) if bot == 'coderabbitai' else None
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
    reason = 'window' if waived else None
    # A bot that has said it cannot review this head is not a bot that has not
    # answered yet. CodeRabbit documents an hour's retry; when the hour passes
    # with no receipt, proceed without it rather than hold the pull request
    # for the whole window — sixteen hours of waiting for a review its author
    # was already told is not coming.
    if limited is not None and _hours(limited, nowISO) >= 1:
        limit_at = (_time(limited) + timedelta(hours=1)).isoformat().replace('+00:00', 'Z')
        if not waived or _time(limit_at) < _time(due_at):
            due_at, waived, reason = limit_at, True, 'rate-limit'
    return {'nudge': due and already is None and not waived,
            'waived_at': due_at if waived else None, 'waived_reason': reason}


def _checklist(**items):
    """Every requirement, met or not, in the order the verdict weighs them."""
    return [dict(item=name, **facts) for name, facts in items.items()]


def evaluate(policy, pr, admitted_at, nowISO, telemetry_states=None):
    result = {k: pr.get(k) for k in ('number', 'head', 'author', 'title', 'url')}
    result.update(state='configuration-error', status='error', blockers=[], reviewers=[], areas=[],
                  ready_since=None, admitted_at=admitted_at, bot_completed_at=None, self_reviewed_at=None,
                  nudge=[], waived=[], approvals=[], objections=[], checklist=[],
                  reviewed_heads=[pr.get('head')] if pr.get('head') else [], reviewed_since=pr.get('head_seen_at'),
                  receipts=((pr.get('controller_diff') or {}).get('receipts') or {}))
    # Before anything can gate: a verdict that stops early must still record
    # what carries this diff, or one configuration error cuts the chain and
    # the pull request silently starts asking for its reviews again.
    try:
        result['reviewed_heads'], result['reviewed_since'] = carried_heads(pr)
    except (KeyError, TypeError, ValueError):
        pass

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
        # A push that leaves this pull request's own diff untouched carries
        # the review of the commit before it: what the bots said still
        # describes the code, and so does the author's attestation. Anything
        # that changes the diff — a conflict resolved inside a merge commit is
        # code that exists nowhere else — starts over.
        reviewed, seen_first = carried_heads(pr)
        heads = {h.lower() for h in reviewed}
        result['reviewed_heads'] = reviewed
        result['reviewed_since'] = seen_first
        latest = _latest_reviews(pr['reviews'])
        bot_blocks = [r for u,r in latest.items() if u in BOTS and r['state'].upper() == 'CHANGES_REQUESTED']
        bot_threads = [t for t in pr['threads'] if not t['is_resolved'] and t['author'].lower() in BOTS]
        pasta, rabbit = [], []
        for review in pr['reviews']:
            user, state = review['user'].lower(), review['state'].upper()
            if (review['commit_id'] or '').lower() not in heads:
                continue
            if user == 'thepastaclaw' and state in {'APPROVED','COMMENTED'} and re.search(
                r'(?mi)^<!-- thepastaclaw-review-phase v1 phase=final sha=(' + '|'.join(re.escape(h) for h in sorted(heads))
                    + r')(?:\s+[^<>]*?)?\s*-->', review['body']):
                pasta.append(review['submitted_at'])
            if user in {'coderabbitai','coderabbitai[bot]'} and state == 'APPROVED':
                rabbit.append(review['submitted_at'])
        seen_said = {}
        for comment in pr['comments']:
            if comment['user'].lower() in {'coderabbitai','coderabbitai[bot]'} and any(
                    _rabbit_receipt(comment['body'], h) for h in reviewed):
                instant = receipt_instant(pr, comment)
                rabbit.append(instant)
                said = receipt_print(comment)
                if said:
                    seen_said[said] = instant
        result['receipts'] = seen_said
        # A bot that requested changes on an earlier head has not reported on
        # this one: that is the shape a nudge and a waiver exist for. An
        # objection raised against the current head is a report, and blocks.
        bot_blocks = [r for r in bot_blocks if (r.get('commit_id') or '').lower() in heads]
        receipts = {'thepastaclaw': pasta, 'coderabbitai': rabbit}
        # A bot that objected to this head, or left a thread open, has
        # reported. It is not missing, so nothing waives it: the objection is
        # answered by dismissing the review or resolving the thread, in the
        # open, not by telling this controller to stop waiting.
        heard = {u for u, r in latest.items() if u in BOTS and (r.get('commit_id') or '').lower() in heads
                 and r['state'].upper() == 'CHANGES_REQUESTED'}
        heard |= {t['author'].lower().removesuffix('[bot]') for t in bot_threads}
        missing = [bot for bot in sorted(required) if not receipts[bot] and bot not in heard]
        waived, why, reasons = {}, {}, []
        outstanding = bool(bot_blocks or bot_threads)
        skip = skipped_by(pr['comments'], permissions, seen_first)
        for bot in missing:
            plan = bot_schedule(policy, dict(pr, head_seen_at=seen_first, reviewed_heads=reviewed),
                                bot, nowISO, (telemetry_states or {}).get(bot))
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
                why[bot] = plan['waived_reason']
            else:
                reasons.append(f'{bot} has not reported for the current head')
        result['waived'] = sorted(waived)
        notes.extend((f"Proceeded without {bot}: skipped by @{skip['user']}" if skip
                      else f'Proceeded without {bot}: it reported a rate limit and did not return'
                      if why.get(bot) == 'rate-limit'
                      else f'Proceeded without {bot}: no review within the configured window')
                     for bot in sorted(waived))
        # Everything about each bot, not the first thing: a bot can have
        # reported and still hold the box open with an objection or a thread,
        # and a line that said only "✓" left the open box unexplained.
        objecting = {u.removesuffix('[bot]') for u, r in latest.items() if u in BOTS
                     and (r.get('commit_id') or '').lower() in heads and r['state'].upper() == 'CHANGES_REQUESTED'}
        threads_by = {}
        for thread in bot_threads:
            who = thread['author'].lower().removesuffix('[bot]')
            threads_by[who] = threads_by.get(who, 0) + 1
        bot_lines = []
        for bot in sorted(set(required) | set(objecting) | set(threads_by)):
            parts = []
            if bot in waived:
                parts.append(f"skipped by {skip['user']}" if skip
                             else 'skipped after its own rate limit' if why.get(bot) == 'rate-limit'
                             else 'skipped after the window')
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
        # When the bots last spoke, recorded whether or not they are done with
        # it. A finding arriving after the author was asked to attest is what
        # voids the attestation, and whoever has to tell them again reads this.
        result['bot_completed_at'] = completed if instants else None
        # `/self-reviewed <sha>` names the commit it covers. Bare `/self-reviewed`
        # means "everything pushed so far", which is only safe once this head has
        # a status: that timestamp cannot be moved, so an attestation written
        # before the last push can never be reused for it.
        seen = seen_first
        # An attestation is the author saying it, wherever they say it: a
        # comment, or the body of their own review — which is one action from
        # the diff they are attesting to. A review carries no edit history
        # here, and needs none: editing an old one cannot move its timestamp
        # forward, and the floor below is what a later edit would have to beat.
        written = [dict(user=c['user'], body=c['body'], at=c['created_at'])
                   for c in pr['comments'] if c['created_at'] == c['updated_at']]
        written += [dict(user=r['user'], body=r['body'] or '', at=r['submitted_at']) for r in pr['reviews']]
        # Twice in two days a colleague posted the phrase on somebody else's
        # pull request and nothing happened. It cannot count — an attestation
        # is the author saying they read their own diff — but silence about it
        # leaves them believing they have done the thing.
        attestations, on_their_behalf = [], []
        for comment in written:
            if comment['user'].lower() != pr['author'].lower():
                if ATTESTATION.fullmatch(comment['body'].strip()) and comment['user'].lower() not in BOTS:
                    on_their_behalf.append(comment['user'])
                continue
            said = ATTESTATION.fullmatch(comment['body'].strip())
            if not said:
                continue
            if said['head'] and said['head'].lower() not in heads:
                continue
            if said['head']:
                floor = floor_at
            elif seen:
                floor = max(floor_at, seen, key=_time)
            else:
                continue
            if _time(comment['at']) > _time(floor):
                attestations.append(comment['at'])
        if not attestations and machine_author(policy, pr):
            # An account that opens pull requests without a person behind it
            # cannot post an attestation. It never owns an area — the policy
            # refuses to load if it does — so the eligible approval it needs
            # anyway is what stands in for one.
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
                                 if u in latest and latest[u]['state'].upper() == 'APPROVED'
                                 and (latest[u]['commit_id'] or '').lower() in heads)
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
            if not admitted_at and not machine_author(policy, pr):
                # Five at a time per author is a limit on human attention, so
                # it applies here, where a human would be asked, and not to a
                # pull request that needs none. An account that opens pull
                # requests on its own has no attention to ration; what it
                # costs reviewers is governed by the approvals it still needs,
                # not by a queue its author never feels.
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
                elif previous.get('admitted_at') or machine_author(policy, pr):
                    # A machine author holds no slot, so admission is never
                    # recorded for it — and keying on admission left exactly
                    # the pull requests this exemption surfaces with no waiting
                    # time at all, reported as "not recorded" for ever and
                    # sorted for ever as the freshest thing in the queue.
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
                             bot_author=machine_author(policy, pr),
                             on_their_behalf=sorted(set(on_their_behalf)),
                             address=[line for line in objection_lines
                                      if line.split(' ', 1)[0].lower() in unanswered or self_time is None]),
            slot=dict(done=not human or bool(admitted_at) or machine_author(policy, pr),
                      limit=policy['max_active_prs']),
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
