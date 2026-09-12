"""Prepare a governed repository's caller files for a pinned shared engine revision."""

import argparse
import json
from pathlib import Path
import re

from .policy import codeowners, validate_policy
from .registry import CENTRAL_REPOSITORY, POLICIES, ROOT, entry_for, load_registry, policy_path


def caller_workflow(engine_revision):
    return f'''name: PR review policy
on:
  pull_request_target:
    types: [opened, reopened, synchronize, ready_for_review, converted_to_draft, closed, edited]
  issue_comment:
    types: [created, edited, deleted]
  pull_request_review:
    types: [submitted, edited, dismissed]
  workflow_dispatch:
  schedule:
    - cron: '*/15 * * * *'
permissions:
  contents: read
  pull-requests: write
  issues: write
  statuses: write
jobs:
  policy:
    uses: {CENTRAL_REPOSITORY}/.github/workflows/pr-review-reusable.yml@{engine_revision}
'''


def write_bundle(repository, engine_revision, destination, policies_root=POLICIES):
    if not re.fullmatch(r'[0-9a-f]{40}', engine_revision):
        raise ValueError('A full engine commit SHA is required')
    destination = Path(destination)
    if destination.exists():
        raise ValueError('Destination must not exist; existing work is never overwritten')
    entry = entry_for(load_registry(policies_root), repository)
    policy = json.loads(policy_path(policies_root, entry).read_text())
    validate_policy(policy)
    if policy['repository'] != repository:
        raise ValueError('Policy repository mismatch')
    files = {
        '.github/CODEOWNERS': codeowners(policy),
        '.github/workflows/pr-review-policy.yml': caller_workflow(engine_revision),
    }
    for relative, content in files.items():
        path = destination / relative
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text(content)
    unresolved = [a['id']+': '+', '.join(a['unresolved']) for a in policy['areas'] if a.get('unresolved')]
    note = f'''# {repository} review policy packet

Shared engine revision: `{engine_revision}`. This commit must be published in {CENTRAL_REPOSITORY} before these workflows can run. The policy itself is read live from that repository's default branch; nothing policy-related is copied here.

Delete `.github/workflows/pr-review-signal.yml` if the repository still has it: review events now reach the policy workflow directly.

Review these files against the target repository's current default branch. The packet is not a patch application and must not overwrite unrelated local work. Keep the target repository's existing protections and readiness automation until its reviewed activation plan replaces them. `.github/CODEOWNERS` takes precedence over any root CODEOWNERS; inspect the resulting roster explicitly.

Writes are disabled unless the target repository sets PR_REVIEW_AUTOMATION_ENABLED=true. Create the ready-for-human label and verify both bot producers before enabling. Confirm permissions and target branches separately. Do not relax existing native approval requirements until the replacement status has been exercised.

Configuration blockers: {('; '.join(unresolved)) or 'none recorded; live preflight still required'}.
'''
    (destination / 'REVIEW_POLICY_ROLLOUT.md').write_text(note)
    return destination


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--repo', required=True)
    parser.add_argument('--engine-revision', required=True)
    parser.add_argument('--output', type=Path, required=True)
    parser.add_argument('--policies-root', type=Path, default=POLICIES)
    args = parser.parse_args()
    print(write_bundle(args.repo, args.engine_revision, args.output, args.policies_root))


if __name__ == '__main__':
    main()
