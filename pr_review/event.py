"""Select bounded reconciliation work from event metadata, never event code."""

import json
import os
import sys
from pathlib import Path

from .main import run


# The two schedules, told apart by which cron GitHub says fired. The workflow
# needs the sweep's to know when to clone the governed repository, so both live
# here and everything else is generated from them.
SWEEP_CRON = '17 * * * *'
BUILD_SCAN_CRON = '2,32,47 * * * *'


def selections(kind, event):
    if kind == 'schedule':
        fired = event.get('schedule')
        if fired == BUILD_SCAN_CRON:
            # A build turning green raises no event this controller hears, so
            # the pull requests already waiting on one are looked at between
            # sweeps. Nothing is recorded as waiting where writes are off, so
            # there is nothing for this to find.
            if os.environ.get('PR_REVIEW_AUTOMATION_ENABLED') != 'true':
                return []
            return [['--waiting-on-build']]
        if fired and fired != SWEEP_CRON:
            # The caller's crons and this engine's idea of them are versioned
            # apart: a caller can carry a schedule an older pinned engine has
            # never heard of. Falling through runs a full sweep on it, so say
            # so rather than quietly sweeping four times an hour.
            print(f'Unrecognised schedule {fired!r}; treating it as the sweep. '
                  f'Re-pin this repository if its crons have moved.', file=sys.stderr)
    if kind in {'schedule', 'push', 'workflow_dispatch'}:
        # The sweep repairs what events missed. It runs hourly now, so it covers
        # more per run to keep a large repository's rotation inside a day.
        return [['--batch-size', '6']]
    if kind in {'pull_request_target', 'pull_request_review'}:
        numbers = [event['pull_request']['number']]
    elif kind == 'issue_comment':
        if 'pull_request' not in event['issue']:
            return []
        numbers = [event['issue']['number']]
    else:
        raise ValueError('Unsupported review event')
    if any(type(number) is not int or number <= 0 for number in numbers):
        raise ValueError('Invalid event PR number')
    return [['--pr', str(number)] for number in dict.fromkeys(numbers)]


def main():
    event = json.loads(Path(os.environ['GITHUB_EVENT_PATH']).read_text())
    options = ['sync', '--repo', os.environ['GITHUB_REPOSITORY']]
    if os.environ.get('PR_REVIEW_REPOSITORY_ROOT'):
        options += ['--repository-root', os.environ['PR_REVIEW_REPOSITORY_ROOT']]
    if os.environ.get('PR_REVIEW_POLICIES_ROOT'):
        options += ['--policies-root', os.environ['PR_REVIEW_POLICIES_ROOT']]
    if os.environ.get('PR_REVIEW_AUTOMATION_ENABLED') == 'true':
        options.append('--apply')
    for selection in selections(os.environ['GITHUB_EVENT_NAME'], event):
        result = run(options + selection)
        if result:
            return result
    return 0


if __name__ == '__main__':
    raise SystemExit(main())
