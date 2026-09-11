"""Collect a combined read-only PR snapshot and optional explicit Slack delivery."""

import argparse
from collections import Counter
import json
from pathlib import Path
import sys

from . import main
from .github import GitHub, GitHubError
from .policy import validate_policy
from .notifications import build_delivery_plan, deliver
from .registry import POLICIES, load_registry, policy_path, validate_registry


def load_policy(entry, policies_root):
    policy = json.loads(policy_path(policies_root, entry).read_text())
    validate_policy(policy)
    if policy['repository'] != entry['repository']:
        raise ValueError('Registered policy repository mismatch')
    return policy


def collect_snapshot(registry, policies_root=POLICIES, api_factory=GitHub):
    validate_registry(registry)
    now = main.utc_now()
    result = {'generated_at': now, 'complete': True, 'repositories': [], 'pull_requests': [], 'workload': [], 'roster': []}
    roster = {}
    workloads = {}
    for entry in registry['repositories']:
        repo = entry['repository']
        info = {'repository': repo, 'mode': entry['mode'], 'complete': False}
        result['repositories'].append(info)
        try:
            api = api_factory(repo)
            policy = load_policy(entry, policies_root)
            for area in [policy['fallback'], *policy['areas']]:
                for login in area['owners'] + area['reviewers']:
                    roster.setdefault(login.lower(), login)
            context, candidates, snapshots = main.collect(api, policy)
            rows = main.evaluate_snapshots(policy, context, candidates, snapshots, now)
            result['pull_requests'].extend(rows)
            counts = Counter(p['author'].lower() for p in context if p['state'] == 'open'
                             and not p['draft'] and p['base'] in policy['target_branches'])
            for author, count in counts.items():
                workloads.setdefault(author, {})[repo] = count
            info['complete'] = True
        except (GitHubError, ValueError, OSError, KeyError, TypeError):
            # Remote errors can contain payloads; report availability without copying them.
            info['error'] = 'Policy or PR evidence unavailable; inspect repository access and configuration'
            result['complete'] = False
    result['roster'] = sorted(roster.values(), key=str.lower)
    result['workload'] = [{'author': author, 'repositories': counts, 'total': sum(counts.values()),
                           'over_limit': sum(counts.values()) > 5}
                          for author, counts in sorted(workloads.items())]
    result['pull_requests'].sort(key=lambda row: (row['state'] != 'ready-for-human', row.get('ready_since') or now,
                                                row['repository'], row['number']))
    return result


def render_report(snapshot, user=None):
    lines = [f"# PR reviews — {snapshot['generated_at']}", '',
             'Read-only snapshot. Five active slots per repository; combined totals are workload warnings.', '']
    for repo in snapshot['repositories']:
        lines.append(f"- {repo['repository']} ({repo['mode']}): " + ('available' if repo['complete'] else repo['error']))
    lines += ['', '| PR | Author | State | Waiting | Next action |', '| --- | --- | --- | --- | --- |']
    for row in main.selected_rows(snapshot['pull_requests'], user):
        action = '; '.join(row.get('blockers', []))
        if row['state'] == 'ready-for-human':
            action += '; reviewers: ' + ', '.join(row.get('reviewers', []))
        link = f"[{row['repository']}#{row['number']}]({row.get('url', '')})"
        lines.append('| ' + ' | '.join([link, main.cell(row['author']), main.cell(row['state']),
                                       main.age(row.get('ready_since'), snapshot['generated_at']), main.cell(action)]) + ' |')
    lines += ['', '## Combined workload', '']
    for item in snapshot['workload']:
        if user and item['author'].lower() != user.lower():
            continue
        counts = ', '.join(f'{repo}: {count}' for repo, count in item['repositories'].items())
        lines.append(f"- {item['author']}: {item['total']} ({counts})" + (' — above five overall' if item['over_limit'] else ''))
    if not snapshot['complete']:
        lines += ['', 'Incomplete snapshot: totals cover available repositories only.']
    return '\n'.join(lines) + '\n'


def run(argv=None):
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('command', choices=['report'])
    parser.add_argument('--repo')
    parser.add_argument('--policies-root', type=Path, default=POLICIES)
    parser.add_argument('--user')
    parser.add_argument('--format', choices=['markdown', 'json', 'deliveries'], default='markdown')
    parser.add_argument('--output', type=Path)
    parser.add_argument('--send-slack', action='store_true')
    args = parser.parse_args(argv)
    if args.send_slack and (args.user or args.repo):
        parser.error('Slack digests require the complete configured scope; omit --user and --repo')
    registry = load_registry(args.policies_root)
    if args.repo:
        registry['repositories'] = [r for r in registry['repositories'] if r['repository'] == args.repo]
        if not registry['repositories']:
            parser.error('Repository is not in the configured registry')
    snapshot = collect_snapshot(registry, args.policies_root)
    plan = None
    if args.format == 'deliveries' or args.send_slack:
        config = json.loads((Path(args.policies_root) / 'slack.json').read_text())
        plan = build_delivery_plan(snapshot, config)
    if args.format == 'markdown':
        text = render_report(snapshot, args.user)
    elif args.format == 'json':
        # A display filter must not silently alter shared counts or delivery input.
        display = dict(snapshot, pull_requests=main.selected_rows(snapshot['pull_requests'], args.user))
        text = json.dumps(display, indent=2) + '\n'
    else:
        text = json.dumps(plan, indent=2) + '\n'
    if args.output:
        args.output.write_text(text)
    else:
        print(text, end='')
    if args.send_slack:
        results = deliver(plan)
        print(json.dumps({'deliveries': results}), file=sys.stderr)
        if any(r['state'] != 'delivered' for r in results):
            return 1
    return 0 if snapshot['complete'] else 1


if __name__ == '__main__':
    try:
        sys.exit(run())
    except (GitHubError, ValueError, OSError, KeyError) as exc:
        print(f'Review report failed: {exc}', file=sys.stderr)
        sys.exit(1)
