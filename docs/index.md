---
---
# PR Hygiene Report
*Last updated: 2026-05-18 20:58 UTC · commit 2740c2c*

## Summary
- Open PRs: **66** (13 clean · 22 dirty · 9 deferred · 22 draft)
- PRs needing author action: **23**
- Total unresolved threads: **189**

## Scoreboard
_Sort: dirty PRs desc → needs-action desc → to-review desc. Clean players sink to the bottom; pure reviewers appear after authors._

| Author | Open | Clean | Dirty | Deferred | Draft | Needs action | Unresolved | CR | Human | To review |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| @PastaPastaPasta | 7 | 2 | 3 | — | 2 | 4 | 21 | 1 | 20 | — |
| @Claudius-Maginificent | 5 | — | 3 | — | 2 | 3 | 37 | 9 | 28 | — |
| @QuantumExplorer | 6 | — | 3 | 2 | 1 | 2 | 44 | 16 | 28 | 11 |
| @lklimek | 13 | 4 | 2 | 5 | 2 | 4 | 8 | 1 | 7 | — |
| @shumkov | 9 | 4 | 2 | 1 | 2 | 3 | 14 | 1 | 13 | 2 |
| @pshenmic | 4 | 1 | 2 | — | 1 | 2 | 29 | 10 | 19 | — |
| @ZocoLini | 5 | — | 2 | — | 3 | 2 | 6 | — | 6 | — |
| @thephez | 2 | — | 2 | — | — | 2 | 6 | — | 6 | — |
| @Inna333-cuber | 1 | — | 1 | — | — | 1 | 1 | — | 1 | — |
| @llbartekll | 2 | — | 1 | — | 1 | — | 16 | 2 | 14 | — |
| @thepastaclaw | 7 | — | 1 | — | 6 | — | 7 | 2 | 5 | — |
| @ktechmidas | 1 | 1 | — | — | — | — | — | — | — | — |
| @ogabrielides | 1 | — | — | 1 | — | — | — | — | — | — |
| @pauldelucia | 3 | 1 | — | — | 2 | — | — | — | — | — |

## PRs needing author action
### @PastaPastaPasta
- [#3509 fix(platform): default omitted proved query limits](https://github.com/dashpay/platform/pull/3509) — 14 unresolved (1 CodeRabbit, 13 human) · 27 days stale · ✋ changes requested · 🔴 CI failing
  - Top thread: "_⚠️ Potential issue_ \| _🟠 Major_" — 27 days old
- [#3657 fix(wasm-sdk): support binary grove path elements](https://github.com/dashpay/platform/pull/3657) — 5 unresolved (5 human) · 0 days stale
  - Top thread: "<!-- thepastaclaw-review v1 finding=e648cab09e16 dedupe=978d8e56f1ea50a0 -->" — 0 days old
- [#3663 ci: include omitted rust packages in ci filters](https://github.com/dashpay/platform/pull/3663) — 2 unresolved (2 human) · 0 days stale
  - Top thread: "<!-- thepastaclaw-review v1 finding=867572ccbe1b dedupe=59d0d71e5f8ee2e0 -->" — 0 days old
- [#2989 feat(sdk): add credentials-based API for DPNS registerName](https://github.com/dashpay/platform/pull/2989) — 0 unresolved · 0 days stale · ✋ changes requested · 🔴 CI failing

### @Claudius-Maginificent
- [#3637 feat(platform-wallet): serde support](https://github.com/dashpay/platform/pull/3637) — 5 unresolved (1 CodeRabbit, 4 human) · 5 days stale · ⚠ merge conflict
  - Top thread: "_⚠️ Potential issue_ \| _🔴 Critical_" — 4 days old
- [#3585 fix: case-insensitive .dash + atomic state on broadcast failure](https://github.com/dashpay/platform/pull/3585) — 1 unresolved (1 human) · 6 days stale · ✋ changes requested
  - Top thread: "<!-- thepastaclaw-review v1 finding=6301a94999c2 -->" — 6 days old
- [#3554 fix(platform-wallet): auto_select_inputs honors Σ inputs == Σ outputs](https://github.com/dashpay/platform/pull/3554) — 1 unresolved (1 human) · 5 days stale · ✋ changes requested
  - Top thread: "<!-- thepastaclaw-review v1 finding=5faa46ca0095 dedupe=97e40e670beeb9b9 -->" — 5 days old

### @QuantumExplorer
- [#3603 feat(swift-sdk,platform-wallet): wire shielded send end-to-end (all 4 transitions)](https://github.com/dashpay/platform/pull/3603) — 28 unresolved (9 CodeRabbit, 19 human) · 13 days stale · ⚠ merge conflict
  - Top thread: "_⚠️ Potential issue_ \| _🔴 Critical_ \| _⚡ Quick win_" — 13 days old
- [#3557 feat(platform-wallet): instrument wallet_manager RwLock behind lock-stats feature](https://github.com/dashpay/platform/pull/3557) — 11 unresolved (2 CodeRabbit, 9 human) · 20 days stale
  - Top thread: "_⚠️ Potential issue_ \| _🟠 Major_" — 20 days old

### @lklimek
- [#3350 fix(rs-dapi,sdk): decode base64 CBOR error messages from Tenderdash](https://github.com/dashpay/platform/pull/3350) — 3 unresolved (3 human) · 9 days stale · ✋ changes requested
  - Top thread: "<!-- thepastaclaw-review v1 finding=7594525c23d8 -->" — 9 days old
- [#3636 feat(platform-wallet): add birth_height_override to wallet creation API](https://github.com/dashpay/platform/pull/3636) — 5 unresolved (1 CodeRabbit, 4 human) · 5 days stale
  - Top thread: "_⚠️ Potential issue_ \| _🟡 Minor_ \| _⚡ Quick win_" — 5 days old
- [#3648 fix(platform-wallet): local-ledger ownership guard (V27-007)](https://github.com/dashpay/platform/pull/3648) — 0 unresolved · 0 days stale · ✋ changes requested
- [#3651 feat(platform-wallet): SPV cancel_background/identity_ids accessors + FFI no-selectable-inputs error mapping](https://github.com/dashpay/platform/pull/3651) — 0 unresolved · 0 days stale · ✋ changes requested · 🔴 CI failing

### @shumkov
- [#3166 chore: integrate Claude Code into devcontainer for autonomus development](https://github.com/dashpay/platform/pull/3166) — 8 unresolved (8 human) · 73 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing
  - Top thread: "as above." — 73 days old
- [#3634 feat: identity registration with asset-lock proofs](https://github.com/dashpay/platform/pull/3634) — 6 unresolved (1 CodeRabbit, 5 human) · 4 days stale · ✋ changes requested
  - Top thread: "_⚠️ Potential issue_ \| _🟡 Minor_ \| _⚡ Quick win_" — 4 days old
- [#2552 perf(dapi): fetch only specific unconfirmed transaction](https://github.com/dashpay/platform/pull/2552) — 0 unresolved · 0 days stale · ⚠ merge conflict · 🔴 CI failing

### @pshenmic
- [#2444 feat(sdk): add NFT actions in the JS Dash SDK](https://github.com/dashpay/platform/pull/2444) — 17 unresolved (5 CodeRabbit, 12 human) · 446 days stale · ⚠ merge conflict · ✋ changes requested
  - Top thread: "_:warning: Potential issue_" — 446 days old
- [#2465 fix(sdk): replace seeds with ip address](https://github.com/dashpay/platform/pull/2465) — 7 unresolved (7 human) · 456 days stale · ✋ changes requested
  - Top thread: "Why do we use testnet seeds for mainnet?" — 456 days old

### @ZocoLini
- [#3488 chore(swift-sdk): add wrappers for the missing TransactionRecord fields in the swift-sdk](https://github.com/dashpay/platform/pull/3488) — 5 unresolved (5 human) · 26 days stale · ✋ changes requested
  - Top thread: "<!-- thepastaclaw-review v1 finding=106ff7904b9f -->" — 26 days old
- [#3479 test(swift-sdk): swift-sdk test updated and added to CI](https://github.com/dashpay/platform/pull/3479) — 1 unresolved (1 human) · 3 days stale · ✋ changes requested
  - Top thread: "<!-- thepastaclaw-review v1 finding=a25233924084 dedupe=4de506d00bbda291 -->" — 3 days old

### @thephez
- [#3485 fix(wasm-sdk): update stale default testnet DAPI node addresses](https://github.com/dashpay/platform/pull/3485) — 4 unresolved (4 human) · 24 days stale · 🔴 CI failing
  - Top thread: "<!-- thepastaclaw-review v1 finding=b6db56a54dbd -->" — 24 days old
- [#3471 fix(dpp): remove erroneous keywords field from document-meta schema and fix contract keywords docs](https://github.com/dashpay/platform/pull/3471) — 2 unresolved (2 human) · 26 days stale · 🔴 CI failing
  - Top thread: "<!-- thepastaclaw-review v1 finding=72d0df4348f6 -->" — 26 days old

### @Inna333-cuber
- [#2499 docs: replaced the icon with twitter](https://github.com/dashpay/platform/pull/2499) — 1 unresolved (1 human) · 49 days stale
  - Top thread: "<!-- thepastaclaw-review v1 finding=1314ffa7c907 -->" — 49 days old

## Methodology
Generated nightly by [pr-hygiene](https://github.com/dashpay/stale_prs_are_bad). A thread counts as "unresolved" when it is open, not outdated, has a comment from someone other than the PR author, and the most recent comment is from a reviewer. **Dirty** = at least one such thread. **Deferred** = carries a configured deferred label (e.g. `postponed`) — visible but not counted as dirty. **Draft** = the PR is still marked draft on GitHub. **Clean** = open, not draft, not deferred, no unresolved threads. **Needs action** further requires changes-requested, merge conflict, or that the reviewer commented more recently than the author last pushed. **To review** counts clean, non-draft PRs (authored by someone else) where this person is in the requested-reviewer list. Configurable via [`https://github.com/dashpay/stale_prs_are_bad/blob/master/.pr-hygiene.yml`](https://github.com/dashpay/stale_prs_are_bad/blob/master/.pr-hygiene.yml)—edit defaults there.

_No history snapshot from last week was found, so week-over-week deltas are omitted this run._
