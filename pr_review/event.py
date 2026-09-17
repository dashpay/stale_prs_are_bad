"""Select bounded reconciliation work from event metadata, never event code."""

import json
import os
from pathlib import Path

from .main import run


# The schedule that reconciles only pull requests waiting on a build. GitHub
# reports which cron fired, which is what tells the two schedules apart.
BUILD_SCAN_CRON = '2,32,47 * * * *'


def selections(kind, event):
    if kind == 'schedule' and event.get('schedule') == BUILD_SCAN_CRON:
        # A build turning green raises no event this controller hears, so the
        # pull requests already waiting on one are looked at between sweeps.
        return [['--waiting-on-build']]
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
