# Explain bot waits and recognize CodeRabbit status-only rate limits

## Problem and evidence

Platform PR #5014 at `ee44bba9dbc2725dbe0075a7542026fecf83e8fc` has a
head-matched final PastaClaw approval. CodeRabbit's latest commit status is
`success`, context `CodeRabbit`, description `Review rate limited`, created
2026-10-04T15:21:11Z by `coderabbitai[bot]`. Its standing comment still describes
an older review and contains no rate-limit notice. The engine therefore waits
the full 16-hour window instead of applying its existing one-hour rate-limit
rule. Its comment still tells the author to address findings on an older head:
waiting-bots and too-many-open-prs currently have no announcement text.

## Approach and alternatives

Reuse the already cached REST statuses for the exact current head. GitHub
documents this list as newest first and includes the creator on each entry:
https://docs.github.com/en/rest/commits/statuses#list-commit-statuses-for-a-reference
Select the latest status authored by CodeRabbit in its exact `CodeRabbit`
context. Only the exact `success` / `Review rate limited` pair counts. A newer
CodeRabbit status supersedes the older limit. A status written by somebody
else cannot impersonate CodeRabbit. Normalize a matching timestamp into an
optional `coderabbit_rate_limited_at` snapshot field; omit it when absent so
existing snapshots and fingerprints remain compatible. Include a present
field in evidence fingerprints so final publication rechecks detect changes.

Use the later of the status timestamp and the existing diff/head waiting
origin for the one-hour clock. Combine valid comment and status notices by
their earliest effective instant. Keep the ordinary waiver deadline as the
upper bound. A rate limit never supplies an approval and never waives existing
blocking findings. No additional network call or status-event rollout is
needed: existing events and sweeps collect the cached status list.

Add next-step comments for waiting-bots and too-many-open-prs using the existing
announcement/update mechanism. Waiting text names each bot's actual checklist
state and offers waiting or `/skip-bots` by a writer; admission text explains
that another active PR must merge or become draft. A new head or change of
responsibility gets a fresh comment; unchanged waits update in place and then
perform no comment writes. Cover bot-authored PRs too: these are informational
waiting explanations, not requests for self-attestation.

Rejected: treating green CodeRabbit checks as approval (a limit is green),
requiring a new rate-limit comment (producer does not always write one),
reducing all timeouts (changes unrelated policy), or only posting a manual
explanation on #5014 (does not repair future cases).

## Implementation and failure modes

Change both Python production engine and Rust mirror. Reuse their existing
cached status reads and timestamp validation. Missing/foreign/malformed
evidence must never create a waiver. Preserve existing comment-based limits,
review completion, blocking threads, manual skips, and policy timeouts.
Retain prior comments as history; the newest announcement carries current
instructions and the state record. Existing first- and final-read guards
continue to apply. #5014 may still be held by its author's active-PR limit
and an outstanding wallet-FFI human approval after CodeRabbit is waived.

## Verification and delivery

First reproduce the status-only limit and missing waiting comments in tests
and observe failures. Test exact creator/context/head binding, newer statuses,
one-hour boundary, ordinary timeout, stable timestamps, evidence fingerprints,
blocking findings, and idempotent publication for both waiting states.
Run Python tests, Rust engine tests and cross-language conformance; update
golden outputs only for the intended new comments. Run formatting and workspace
checks. Independent agents review spec before code and diff after code.
Then push a signed commit, open and admin-merge the fix PR as authorized.
Reconcile #5014 through the merged workflow and verify the live explanation.

## Spec review decisions

Two independent reviews found no design blocker. Select the newest trusted
status before validating its limit fields; a newer malformed or non-limit
status must not revive an older limit. Test a final refresh that removes the
optional field and refuses stale publication. Publication regressions cover
old author instructions becoming a fresh bot-wait announcement, unchanged
reconciliation writing no comment, changed bot details editing in place, and
responsibility changing away and returning. `/skip-bots` text must say it
skips only missing reports; existing blocking findings remain visible.
Historical move announcements remain; existing legacy standing-comment cleanup
is unchanged.

## Verification results

The status-only regression and obsolete-author-comment regression failed
against the old behavior, then passed with the fix. The admission-comment
regression also failed before its renderer was added. Python: 535 tests pass;
fresh Python 3.12 harvest matches the committed corpus; 275 evaluation and
664 function cases replay with no failures. Rust engine: 485 tests pass, two
pre-existing tests remain ignored; all 13 synthetic recordings match. Both
independent code reviewers found no blocking issue. Live read-only evaluation
of #5014 recognizes the rate limit and reports `too-many-open-prs` with the
new active-slot instruction. Workspace Clippy and formatting pass.
