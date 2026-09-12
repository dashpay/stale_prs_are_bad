"""Checks that compare each policy against the repository it governs.

Run from the workflow with `proposed/` (this repository) and `trees/<name>/`
(a clone of every governed repository's default branch) already in place.
"""

import argparse
import json
from pathlib import Path
import sys

import yaml

CENTRAL = 'dashpay/stale_prs_are_bad'
REUSABLE = f'{CENTRAL}/.github/workflows/pr-review-reusable.yml'.casefold()
CALLER = '.github/workflows/pr-review-policy.yml'
DEFAULT_BOTS = ('thepastaclaw', 'coderabbitai')


def governed(policies_root):
    policies_root = Path(policies_root)
    registry = json.loads((policies_root / 'repositories.json').read_text())
    for entry in registry['repositories']:
        yield entry['repository'], json.loads((policies_root / entry['policy']).read_text())


def auto_review_disabled(tree):
    """The CodeRabbit config file turning automatic reviews off, if the repository has one."""
    for candidate in ('.coderabbit.yaml', '.coderabbit.yml'):
        config = Path(tree) / candidate
        if config.is_file():
            settings = yaml.safe_load(config.read_text()) or {}
            enabled = (settings.get('reviews') or {}).get('auto_review', {}).get('enabled')
            return candidate if enabled is False else None
    return None


def bot_problems(policies_root, trees_root):
    """(errors, warnings) where a policy disagrees with the repository's CodeRabbit setting."""
    errors, warnings = [], []
    for repository, policy in governed(policies_root):
        name = repository.split('/')[1]
        required = 'coderabbitai' in policy.get('required_bots', DEFAULT_BOTS)
        disabled = auto_review_disabled(Path(trees_root) / name)
        if disabled and required:
            errors.append(f'{repository}: {disabled} disables automatic CodeRabbit reviews, so requiring '
                          'coderabbitai would never resolve; drop it from required_bots')
        elif not disabled and not required:
            warnings.append(f'{repository}: required_bots omits coderabbitai but nothing in the repository '
                            'disables automatic reviews; confirm this is deliberate')
    return errors, warnings


def caller_pin(workflow_text):
    """The engine commit a caller pins, read the way the reusable workflow reads it."""
    jobs = yaml.safe_load(workflow_text) or {}
    pins = []
    for job in (jobs.get('jobs') or {}).values():
        uses = job.get('uses') if isinstance(job, dict) else None
        if isinstance(uses, str) and uses.partition('@')[0].casefold() == REUSABLE:
            pins.append(uses.partition('@')[2])
    if len(pins) != 1:
        raise ValueError(f'expected exactly one job calling the shared workflow, found {len(pins)}')
    return pins[0]


def caller_pins(policies_root, trees_root):
    """(repository, pin) for every governed repository whose caller workflow exists."""
    for repository, _ in governed(policies_root):
        caller = Path(trees_root) / repository.split('/')[1] / CALLER
        if not caller.is_file():
            print(f'{repository}: no caller workflow yet', file=sys.stderr)
            continue
        yield repository, caller_pin(caller.read_text())


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('command', choices=['bots', 'pins'])
    parser.add_argument('--policies-root', default='proposed/policies')
    parser.add_argument('--trees-root', default='trees')
    args = parser.parse_args()
    if args.command == 'bots':
        errors, warnings = bot_problems(args.policies_root, args.trees_root)
        for warning in warnings:
            print(f'::warning::{warning}')
        for error in errors:
            print(f'::error::{error}')
        return 1 if errors else 0
    for repository, pin in caller_pins(args.policies_root, args.trees_root):
        print(f'{repository} {pin}')
    return 0


if __name__ == '__main__':
    raise SystemExit(main())
