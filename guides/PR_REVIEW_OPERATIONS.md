# PR Hygiene operations

All five repository configurations start in preview. The implementation does not grant permissions or change protection settings. The ownership rules and repository boundaries are described in [PR_REVIEW_ARCHITECTURE.md](PR_REVIEW_ARCHITECTURE.md).

## Read the queue

Use Python 3.10 or newer and an authenticated `gh` CLI. Run from this repository's checkout (the digest needs the read App token; a personal `gh` login with access to the five repositories works locally):

```sh
python3 -m pr_review.aggregate report
python3 -m pr_review.aggregate report --user shumkov
python3 -m pr_review.aggregate report --repo dashpay/rust-dashcore --format json
python3 -m pr_review.aggregate report --format deliveries
```

`/review-prs` uses this reporter. It combines repository-qualified PRs, actionable reviews, waiting times and author blockers. Counts include open, non-draft PRs on configured target branches. Five slots are enforced per repository; combined totals above five are workload warnings. Missing repository evidence is explicitly unavailable and totals are partial. The command exits nonzero for incomplete collection while preserving its report.

`policies/repositories.json` selects the five repositories and names each policy file. `mode` labels results as preview until that repository's caller is merged and verified; switch it to `active` then. CI discovers the engine commit each caller pins by reading its `pr-review-policy.yml` from the caller's default branch, and validates every policy change against every engine still in use. The `engine_revision` key in the registry is ignored and can be removed once no caller pins an engine older than this rule. Missing evidence for a repository is always reported as unavailable, never as an empty queue.

## Owner, reviewer and author flow

1. Keep unfinished work in draft. A draft records its state in the commit status only: no comment is opened on one, matching the review bots, and an existing comment is kept current if the pull request was ready earlier. The oldest eligible PRs fill up to five author slots, preserving existing admissions. The sixth waits; automation does not prevent PR creation.
2. Finish final review by thepastaclaw and CodeRabbit on the current head, address outstanding bot changes requests and resolve bot threads. Where a repository sets `bot_timeouts`, a bot that has not reported is asked once with `@<bot> review` — immediately if its own status page says the review failed, otherwise after `nudge_after_hours` — and is dropped for that commit after `waive_after_hours`, which labels the pull request `bot-review-skipped` and says so in the status. Anyone with write access — the author included — can post `/skip-bots` to drop the bots that have not reported for the current head without waiting for the window; the report names who did, and the same label goes on. A bot that has objected to the head, or left a thread open, has reported and is not skipped: dismiss the review or resolve the thread. A skip is read like the bare attestation, after this controller has reported on the head, unedited, and a new push clears it. A review the status page shows as still running delays that by at most the same window again. The label and the requirement return on the next push.
3. Inspect the final diff and verification yourself. Post `/self-reviewed` as an unedited comment after the bot outcomes — including a skip or a waiver, which count as outcomes — and after this controller has reported on the current head: it attests to everything pushed so far, and its timestamp is checked against that report. `/self-reviewed FULL_HEAD_SHA` names one commit explicitly and needs no such wait. New commits or later bot completion require a new attestation, and an edited comment never counts.
4. Humans are invited for areas that still need approval. Owners satisfy their own area's human requirement; reviewers cannot do so on their own PRs. Existing CI and native protections continue to apply.
5. Human objections still block merging. Renew self-review after addressing an objection so its reviewer is invited back; the objection/thread must also be cleared before merge.

Every state carries its own label (`waiting-slot`, `waiting-bots`, `waiting-build`, `waiting-self-review`, `waiting-author`, `ready-for-human`, `ready-to-merge`), exactly one at a time, worded as the status is; drafts carry none. They are outputs, not independent proof of readiness. Waiting time is the current recorded review cycle, not total PR age. Native or manual review requests may arrive earlier until native automatic routing is explicitly disabled during activation.

## Shared implementation and repository-local rollout

The evaluator is shared through `.github/workflows/pr-review-reusable.yml`. Repository callers pin a full commit SHA of this repository; the callee checks out that engine revision, a sparse checkout of `policies/` from this repository's protected `master`, and the caller's default branch (for path and CODEOWNERS checks only). Before reading any policy it verifies that the pinned engine commit is reachable from `master` (a merged, reviewed engine) and that `master` is governed by a ruleset requiring pull requests with one approval, code-owner review, no force-push and no deletion; otherwise it refuses to run. The caller's GITHUB_TOKEN can mutate only its own repository. Target repository code is not executed with the write token.

Policy schema changes must be engine-first and backward-compatible: callers pick up policy changes immediately but engine changes only when re-pinned. `version` stays at 1.

Prepare reviewable rollout files after committing the shared implementation:

```sh
python3 -m pr_review.rollout --repo dashpay/tenderdash --engine-revision FULL_MERGE_COMMIT_SHA --output /tmp/tenderdash-review-policy
```

Repeat for Platform, GroveDB, Dash Evo Tool and rust-dashcore. The destination must not exist. Packets contain native CODEOWNERS and a pinned caller workflow; they contain no policy and no copied evaluator. The referenced commit must be a merge commit on this repository's `master` (squash merges discard PR head SHAs). Pins are discovered by CI; nothing needs recording here. Inspect and apply packet files in a clean target checkout; the generator never overwrites a target repository itself.

Before any caller can run, `master` of this repository must be governed by a ruleset that requires pull requests with at least one approval and code-owner review and blocks force-pushes and deletion (include administrators). The reusable workflow checks this through the public rulesets endpoint and refuses to read policies otherwise; classic branch protection is not visible there and does not satisfy the check.

Each repository needs its own review before activation:

- Confirm roster identities and repository permissions. Keep strophy and Silvanassss excluded and broad teams unchanged. rust-dashcore has explicit missing-owner and cropped-scope gaps.
- Confirm configured target branches, existing CODEOWNERS precedence, native approval rules and both bot producers. rust-dashcore's existing readiness automation needs an explicit migration; do not silently replace it. Tenderdash disables automatic CodeRabbit reviews, so its policy lists only thepastaclaw in `required_bots` — declare the producers a repository actually runs rather than turning a bot on for it. CI compares each declaration against that repository's own CodeRabbit configuration. Adding the field to a policy requires every caller to pin an engine that understands it first.
- Merge under existing protections and run preview from the default branch. Verify complete evidence reads, token permissions and API usage.
- Create the seven state labels and `bot-review-skipped`; a missing one is reported rather than fatal. Then opt into writes with repository variable `PR_REVIEW_AUTOMATION_ENABLED=true`.
- Verify real current-head statuses, comments and requests before requiring the `PR Hygiene` status. Remove conflicting native approval/code-owner rules only when the owner exemption is approved and the replacement is working. Keep CI requirements.
- To suppress native early invitations, replace the effective CODEOWNERS with a comment-only `.github/CODEOWNERS` during that separate activation. Generated native routing is not delayed routing.

Events reevaluate the affected PR and changes to its author's slot assignments. Only comments this controller reads trigger a run: a review bot's, or one containing `/self-reviewed` or `/skip-bots`. Coverage bots, other automation and human discussion cannot change an outcome, and the filter is on the event payload, so those start no runner at all. Edits count, because CodeRabbit publishes its completion by editing the comment it posted when the review began — every receipt observed on Platform arrived that way, and none was accompanied by a review. A CodeRabbit completion delivered by editing an existing comment is therefore picked up by the next event or the hourly sweep rather than immediately. Selector-less workflow invocations rotate six PRs at a time and the scheduled repair runs hourly, so a stable 57-PR queue is covered in about ten hours when event signals are missed. Pagination and event bursts can still exhaust quota; errors remain visible. Full local `sync` is an explicit unbounded sweep.

## Combined daily Slack delivery

One scheduled job at 02:00 UTC every day collects a single snapshot for the channel summary and all personal digests. GitHub schedules may be delayed. Each distinct mapped Slack user gets one combined DM, even if several GitHub identities map to that person. Empty personal digests are skipped; unavailable evidence is reported. The channel gets the actionable queue, author/bot blockers and workload warnings.

Configure a GitHub App installed only on these five repositories with read access to contents, pull requests, issues and metadata. Set repository variable `PR_REVIEW_APP_ID` and secret `PR_REVIEW_APP_PRIVATE_KEY` in this repository. The workflow creates and revokes short-lived selected-repository tokens. It requests no write or administrator permissions. The App is mandatory: collaborator permission reads need push access on each governed repository, which this repository's own token never has, so the job fails early with a clear summary when the App is not configured.

Configure a Slack app with `chat:write`, invite it to the shared channel, and set secret `PR_REVIEW_SLACK_BOT_TOKEN`. Fill `policies/slack.json` with the exact channel ID and GitHub-login-to-Slack-user-ID mappings. Null values intentionally block sending and appear in delivery previews. No directory/email lookup or guessed identity is used. Set `PR_REVIEW_SLACK_ENABLED=true` only after inspecting the delivery preview and confirming recipients.

Test through workflow dispatch with `send_slack=true`. Enabled scheduled runs send automatically. Personal or repository filters cannot be combined with sending. Messages use plain-text blocks to prevent PR-controlled text from pinging channels. A missing mapping blocks all sends; partial repository evidence is disclosed in any delivered summary, never described as a complete empty queue.

Delivery is attempted once per destination and is not automatically retried. An error or uncertain acknowledgement stops later destinations and records which were delivered, uncertain or not attempted. Inspect Slack and those receipts before rerunning: a manual rerun is a new invocation and can duplicate earlier successful deliveries. The generated report is retained even when delivery fails.

## Why a run is never cancelled

A concurrency group holds one pending run by default and cancels it when the next event arrives, so a burst of events on one pull request cancelled the middle ones, and a cancelled run renders as a failed check wherever it is shown. `queue: max` lets up to a hundred runs wait in the group instead, in the order they arrived, so every event's work is kept and nothing is cancelled. Runs for one pull request remain serialised.

Runs for one pull request can still overlap by other routes, and the writes are built for it: a sweep keys its group on the run id, and an event run reconciles its author's whole queue, so it writes to pull requests that are not the event's subject and are grouped separately. The oldest controller state comment wins, a refused label or reviewer request is reported and skipped rather than aborting the run, and a run whose evidence changed under it publishes `pending` instead of a terminal state.

## Verification and recovery

```sh
python3 -m unittest discover -s pr_review/tests -v
python3 -m pr_review.main validate --repo dashpay/REPOSITORY                       # schema only
python3 -m pr_review.main validate --repo dashpay/REPOSITORY --repository-root DIR # also checks every area path exists in that checkout
python3 -m pr_review.main codeowners --check --repo dashpay/REPOSITORY --repository-root DIR
```

CI performs the path and packet checks against a fresh clone of every governed repository's default branch, and re-validates the proposed policies with every engine revision a caller still pins. A repository that calls the shared workflow without being registered here marks its own open heads as configuration errors once its writes are enabled.

Edit seed ownership manifests and regenerate CODEOWNERS deliberately. An empty owner list is permitted only when the named area carries an explicit unresolved configuration blocker. It never grants owner powers to its reviewers.

The Actions solution assumes trusted repository writers. It shares an app identity across workflows, and GitHub offers no atomic read-and-publish transaction. Existing administrator bypass remains. These limitations are not solved by adding a shared collector.

For rollback, restore the recorded native approval/routing settings before removing the required custom status and disabling evaluator writes. Disabling writes alone leaves old statuses and labels. Disable Slack separately with `PR_REVIEW_SLACK_ENABLED=false`. Never repair an attestation by editing it for the author; require a new author comment. Investigate malformed or conflicting controller history rather than deleting unrelated comments. A pull request carrying two controller state comments is not an error: the oldest is authoritative and every run agrees on it, because two runs reconciling one pull request at the same time can each open one. The younger is inert and can be left alone or removed by hand.
