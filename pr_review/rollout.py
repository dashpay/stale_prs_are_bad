"""Prepare a governed repository's caller files for a pinned shared engine revision."""

import argparse
import json
from pathlib import Path
import re

from .event import BUILD_SCAN_CRON, SWEEP_CRON
from .policy import RATE_LIMITED_MARKER, RECEIPT_MARKER, codeowners, validate_policy
from .registry import CENTRAL_REPOSITORY, POLICIES, ROOT, entry_for, load_registry, policy_path


def caller_workflow(engine_revision):
    return f'''name: PR Hygiene policy
on:
  pull_request_target:
    types: [opened, reopened, synchronize, ready_for_review, converted_to_draft, closed, edited]
  issue_comment:
    # CodeRabbit publishes its completion by editing the comment it posted when
    # the review began: on Platform every observed receipt arrived that way and
    # none was accompanied by a review. Edits therefore have to be heard, but
    # only from the accounts whose comments this controller reads.
    types: [created, edited]
  pull_request_review:
    types: [submitted, edited, dismissed]
  workflow_dispatch:
    inputs:
      scope:
        description: 'all reconciles every governed pull request now; batch is the sweep'
        type: choice
        options: [batch, all]
        default: batch
  schedule:
    - cron: '{SWEEP_CRON}'
    # A build turning green raises no event this controller hears. This one
    # reconciles only the pull requests already recorded as waiting on one, so
    # it costs a listing and a single batched read when none are — which is
    # almost always. Offset from the hourly sweep so the two never collide.
    - cron: '{BUILD_SCAN_CRON}'
permissions:
  contents: read
  pull-requests: write
  issues: write
  statuses: write
jobs:
  policy:
    # Only comments this controller actually reads can change an outcome, and a
    # comment it reads only matters for what it carries. CodeRabbit never
    # approves — measured over 75 recent pull requests, every one of its
    # receipts arrived as a comment and none as a review — so its completion
    # marker and its rate-limit notice have to be heard. It keeps one comment
    # per pull request and edits it in place, so once it has reviewed once that
    # body carries the marker for ever and its later edits still wake the
    # controller: the saving is on pull requests it has not reviewed yet or
    # skips entirely. thepastaclaw is the mirror image: its receipt is always a
    # review, which arrives on its own event, and nothing reads its comments.
    #
    # Narrowing this costs latency and never correctness: comments are read from
    # the pull request when a run happens, not from the event that started it,
    # so a comment that starts no run is still seen by the next one.
    if: >-
      github.event_name != 'issue_comment' ||
      contains(github.event.comment.body, '/self-reviewed') ||
      contains(github.event.comment.body, '/skip-bots') ||
      (contains(fromJSON('["coderabbitai", "coderabbitai[bot]"]'), github.event.comment.user.login) &&
      (contains(github.event.comment.body, '{RECEIPT_MARKER}') ||
      contains(github.event.comment.body, '{RATE_LIMITED_MARKER}')))
    uses: {CENTRAL_REPOSITORY}/.github/workflows/pr-review-reusable.yml@{engine_revision}
    with:
      scope: ${{{{ inputs.scope || 'batch' }}}}
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

Shared engine revision: `{engine_revision}`. This commit must be merged to {CENTRAL_REPOSITORY}'s default branch before these workflows can run; a caller pinned to an older engine keeps running that engine's workflow definition until it is re-pinned. The policy itself is read live from that repository's default branch; nothing policy-related is copied here.

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
