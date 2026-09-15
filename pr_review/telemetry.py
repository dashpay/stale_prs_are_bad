"""Read the review system's public status page. Never authoritative, never fatal.

The page is a third party's; a stale, missing or hostile payload may only make a
report less informative or cause one extra nudge. It can never hold a pull
request back, so nothing here is consulted when deciding to waive.
"""

import json
import urllib.error
import urllib.request

SOURCE = 'https://thepastaclaw.github.io/review-system/data/status.json'
LIMIT = 2 * 1024 * 1024
STALE_SECONDS = 20 * 60
BOT = 'thepastaclaw'


class _RejectRedirects(urllib.request.HTTPRedirectHandler):
    def redirect_request(self, req, fp, code, msg, headers, newurl):
        raise urllib.error.HTTPError(req.full_url, code, 'Telemetry redirects are not followed', headers, fp)


_OPENER = urllib.request.build_opener(_RejectRedirects)


def fetch(source=SOURCE, timeout=10):
    """The parsed payload, or None. One attempt; any problem at all returns None."""
    try:
        with _OPENER.open(urllib.request.Request(source), timeout=timeout) as response:
            if response.status != 200:
                return None
            payload = json.loads(response.read(LIMIT + 1).decode('utf-8', 'replace'))
    except Exception:
        return None
    if not isinstance(payload, dict) or payload.get('schema_version') != 1:
        return None
    return payload


def _rows(payload, section, key):
    value = (payload.get(section) or {}).get(key)
    return [row for row in value if isinstance(row, dict)] if isinstance(value, list) else []


def head_state(payload, repository, number, head, head_seen_at, now):
    """What the review system last said about exactly this head.

    Only `head.queued` and `run.spawned` name a commit, and then only its first
    eight characters; failures name none. So an event counts for this head when
    it either names a prefix of it, or names no commit at all and happened after
    this head was first seen — a failure recorded before that belongs to an
    earlier push.

    'running' is believed only while the entry's own heartbeat and deadline say
    so, so a leaked entry whose runner died cannot look alive forever. 'failed'
    means no receipt is ever coming. None means nothing visible was said.
    """
    if not payload or _age(payload.get('data_as_of'), now) > STALE_SECONDS:
        return None

    def mine(row):
        return (row.get('repo') == repository and row.get('number') == number
                and isinstance(row.get('sha'), str) and row['sha'] == head)

    for row in _rows(payload, 'live', 'active'):
        if mine(row) and row.get('status') == 'running':
            fresh = _age(row.get('heartbeat_at'), now) <= STALE_SECONDS
            if fresh and _age(row.get('deadline_at'), now) < 0:
                return 'running'

    def concerns_this_head(event):
        named = [word for word in str(event.get('detail') or '').split()
                 if len(word) >= 7 and all(c in '0123456789abcdef' for c in word)]
        if named:
            return any(head.startswith(word) for word in named)
        return bool(head_seen_at) and _age(event.get('ts'), head_seen_at) <= 0

    for event in sorted(_rows(payload, 'history', 'recent_events'),
                        key=lambda row: row.get('ts') or '', reverse=True):
        if event.get('repo') != repository or event.get('number') != number:
            continue
        if not concerns_this_head(event):
            continue
        if event.get('kind') in {'head.failed', 'run.failed'}:
            return 'failed'
        if event.get('kind') in {'head.queued', 'run.spawned'}:
            return 'queued'
    return None


def expected_wait_seconds(payload):
    """A rough queue drain time, for display only."""
    if not payload:
        return None
    heads = (payload.get('live') or {}).get('heads') or {}
    capacity = (payload.get('live') or {}).get('capacity') or {}
    daily = _rows(payload, 'history', 'daily')
    queued = heads.get('queued')
    concurrency = capacity.get('typical')
    averages = [row['avg_seconds'] for row in daily
                if isinstance(row.get('avg_seconds'), (int, float)) and row['avg_seconds'] > 0]
    if not isinstance(queued, int) or queued < 0 or not isinstance(concurrency, int) or concurrency < 1 or not averages:
        return None
    return int(queued * (sum(averages[:7]) / len(averages[:7])) / concurrency)


def summary(payload, now):
    """One line for a report, or None when there is nothing trustworthy to say."""
    if not payload or _age(payload.get('data_as_of'), now) > STALE_SECONDS:
        return None
    heads = (payload.get('live') or {}).get('heads') or {}
    capacity = (payload.get('live') or {}).get('capacity') or {}
    counts = [f'{heads[key]} {key}' for key in ('running', 'queued') if isinstance(heads.get(key), int)]
    if not counts:
        return None
    text = f"{BOT}: " + ', '.join(counts)
    if isinstance(capacity.get('typical'), int) and isinstance(capacity.get('maximum'), int):
        text += f" (capacity {capacity['typical']}-{capacity['maximum']})"
    wait = expected_wait_seconds(payload)
    if wait:
        text += f', queue drains in about {wait // 3600} h {wait % 3600 // 60} m'
    streak = (payload.get('live') or {}).get('ingest_error_streak')
    if isinstance(streak, int) and streak > 0:
        text += f'; ingest failing ({streak} in a row)'
    return text


def _age(stamp, now):
    """Seconds between a payload timestamp and now; enormous when unusable."""
    from .policy import _time
    try:
        return (_time(now) - _time(stamp)).total_seconds()
    except Exception:
        return float('inf')
