---
---
# PR Hygiene Report
*Last updated: 2026-05-18 19:51 UTC · commit 999e40a*

## Summary
- Open PRs: **66** (37 clean · 29 dirty)
- PRs needing author action: **39**
- Total unresolved threads: **210**

## Scoreboard
_Sorted by dirty PRs (desc). Authors with all-clean PRs sink to the bottom._

| Author | Open | Clean | Dirty | Needs action | Unresolved | CR | Human | Oldest stale | Score |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| @Claudius-Maginificent | 5 | 0 | 5 | 4 | 37 | 9 | 28 | 16d | 250.5 |
| @QuantumExplorer | 6 | 2 | 4 | 5 | 67 | 32 | 35 | 75d | 809.2 |
| @lklimek | 13 | 10 | 3 | 7 | 9 | 2 | 7 | 196d | 85.2 |
| @pshenmic | 4 | 1 | 3 | 3 | 29 | 10 | 19 | 474d | 850.1 |
| @shumkov | 9 | 7 | 2 | 6 | 14 | 1 | 13 | 73d | 222.1 |
| @ZocoLini | 5 | 3 | 2 | 4 | 6 | 0 | 6 | 26d | 89.0 |
| @PastaPastaPasta | 7 | 5 | 2 | 3 | 16 | 1 | 15 | 27d | 237.4 |
| @thepastaclaw | 7 | 5 | 2 | 3 | 7 | 2 | 5 | 89d | 88.9 |
| @thephez | 2 | 0 | 2 | 2 | 6 | 0 | 6 | 26d | 38.7 |
| @llbartekll | 2 | 0 | 2 | 1 | 16 | 2 | 14 | 36d | 111.4 |
| @Inna333-cuber | 1 | 0 | 1 | 1 | 1 | 0 | 1 | 49d | 7.8 |
| @ogabrielides | 1 | 0 | 1 | 0 | 2 | 1 | 1 | 425d | 24.2 |
| @ktechmidas | 1 | 1 | 0 | 0 | 0 | 0 | 0 | — | — |
| @pauldelucia | 3 | 3 | 0 | 0 | 0 | 0 | 0 | — | — |

## PRs needing author action
### @Claudius-Maginificent
- [#3625 feat(platform-wallet): add platform-wallet-storage crate (sqlite persister)](https://github.com/dashpay/platform/pull/3625) — 8 unresolved (8 CodeRabbit) · 7 days stale · ⚠ merge conflict
  - Top thread: "_⚠️ Potential issue_ \| _🟠 Major_ \| _⚡ Quick win_" — 7 days old
- [#3637 feat(platform-wallet): serde support](https://github.com/dashpay/platform/pull/3637) — 5 unresolved (1 CodeRabbit, 4 human) · 5 days stale · ⚠ merge conflict
  - Top thread: "_⚠️ Potential issue_ \| _🔴 Critical_" — 4 days old
- [#3585 fix: case-insensitive .dash + atomic state on broadcast failure](https://github.com/dashpay/platform/pull/3585) — 1 unresolved (1 human) · 6 days stale · ✋ changes requested
  - Top thread: "<!-- thepastaclaw-review v1 finding=6301a94999c2 -->" — 6 days old
- [#3554 fix(platform-wallet): auto_select_inputs honors Σ inputs == Σ outputs](https://github.com/dashpay/platform/pull/3554) — 1 unresolved (1 human) · 5 days stale · ✋ changes requested
  - Top thread: "<!-- thepastaclaw-review v1 finding=5faa46ca0095 dedupe=97e40e670beeb9b9 -->" — 5 days old

### @QuantumExplorer
- [#3021 feat(platform): better contract state transitions](https://github.com/dashpay/platform/pull/3021) — 22 unresolved (15 CodeRabbit, 7 human) · 75 days stale · ⚠ merge conflict · ✋ changes requested
  - Top thread: "_⚠️ Potential issue_ \| _🟠 Major_" — 75 days old
- [#3603 feat(swift-sdk,platform-wallet): wire shielded send end-to-end (all 4 transitions)](https://github.com/dashpay/platform/pull/3603) — 28 unresolved (9 CodeRabbit, 19 human) · 13 days stale · ⚠ merge conflict
  - Top thread: "_⚠️ Potential issue_ \| _🔴 Critical_ \| _⚡ Quick win_" — 13 days old
- [#3557 feat(platform-wallet): instrument wallet_manager RwLock behind lock-stats feature](https://github.com/dashpay/platform/pull/3557) — 11 unresolved (2 CodeRabbit, 9 human) · 20 days stale
  - Top thread: "_⚠️ Potential issue_ \| _🟠 Major_" — 20 days old
- [#1834 feat(drive-abci)!: Statesync](https://github.com/dashpay/platform/pull/1834) — 0 unresolved · 0 days stale · ⚠ merge conflict
- [#3417 feat(swift-sdk): use SPV-synced quorums for Platform proof verification](https://github.com/dashpay/platform/pull/3417) — 0 unresolved · 0 days stale · ⚠ merge conflict · 🔴 CI failing

### @lklimek
- [#3350 fix(rs-dapi,sdk): decode base64 CBOR error messages from Tenderdash](https://github.com/dashpay/platform/pull/3350) — 3 unresolved (3 human) · 9 days stale · ✋ changes requested
  - Top thread: "<!-- thepastaclaw-review v1 finding=7594525c23d8 -->" — 9 days old
- [#2795 feat(platform): add real-time platform event subscriptions via gRPC streaming](https://github.com/dashpay/platform/pull/2795) — 1 unresolved (1 CodeRabbit) · 196 days stale · ⚠ merge conflict · 🔴 CI failing
  - Top thread: "_⚠️ Potential issue_ \| _🟡 Minor_" — 196 days old
- [#3636 feat(platform-wallet): add birth_height_override to wallet creation API](https://github.com/dashpay/platform/pull/3636) — 5 unresolved (1 CodeRabbit, 4 human) · 5 days stale
  - Top thread: "_⚠️ Potential issue_ \| _🟡 Minor_ \| _⚡ Quick win_" — 5 days old
- [#2993 feat: identity reference validation](https://github.com/dashpay/platform/pull/2993) — 0 unresolved · 0 days stale · ⚠ merge conflict · 🔴 CI failing
- [#3032 feat: document creation restriction by group membership](https://github.com/dashpay/platform/pull/3032) — 0 unresolved · 0 days stale · ⚠ merge conflict · 🔴 CI failing
- [#3648 fix(platform-wallet): local-ledger ownership guard (V27-007)](https://github.com/dashpay/platform/pull/3648) — 0 unresolved · 0 days stale · ✋ changes requested
- [#3651 feat(platform-wallet): SPV cancel_background/identity_ids accessors + FFI no-selectable-inputs error mapping](https://github.com/dashpay/platform/pull/3651) — 0 unresolved · 0 days stale · ✋ changes requested · 🔴 CI failing

### @pshenmic
- [#2444 feat(sdk): add NFT actions in the JS Dash SDK](https://github.com/dashpay/platform/pull/2444) — 17 unresolved (5 CodeRabbit, 12 human) · 446 days stale · ⚠ merge conflict · ✋ changes requested
  - Top thread: "_:warning: Potential issue_" — 446 days old
- [#2465 fix(sdk): replace seeds with ip address](https://github.com/dashpay/platform/pull/2465) — 7 unresolved (7 human) · 456 days stale · ✋ changes requested
  - Top thread: "Why do we use testnet seeds for mainnet?" — 456 days old
- [#2446 feat(js-dapi-client): add contested resources query methods](https://github.com/dashpay/platform/pull/2446) — 5 unresolved (5 CodeRabbit) · 474 days stale · ⚠ merge conflict
  - Top thread: "_:warning: Potential issue_" — 474 days old

### @shumkov
- [#3166 chore: integrate Claude Code into devcontainer for autonomus development](https://github.com/dashpay/platform/pull/3166) — 8 unresolved (8 human) · 73 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing
  - Top thread: "as above." — 73 days old
- [#3634 feat: identity registration with asset-lock proofs](https://github.com/dashpay/platform/pull/3634) — 6 unresolved (1 CodeRabbit, 5 human) · 4 days stale · ✋ changes requested
  - Top thread: "_⚠️ Potential issue_ \| _🟡 Minor_ \| _⚡ Quick win_" — 4 days old
- [#2518 feat: token marketplace](https://github.com/dashpay/platform/pull/2518) — 0 unresolved · 0 days stale · ⚠ merge conflict
- [#2552 perf(dapi): fetch only specific unconfirmed transaction](https://github.com/dashpay/platform/pull/2552) — 0 unresolved · 0 days stale · ⚠ merge conflict · 🔴 CI failing
- [#2973 [temp\] Test non determinism](https://github.com/dashpay/platform/pull/2973) — 0 unresolved · 0 days stale · ⚠ merge conflict · 🔴 CI failing
- [#3419 Feat/platform wallet](https://github.com/dashpay/platform/pull/3419) — 0 unresolved · 0 days stale · ⚠ merge conflict · 🔴 CI failing

### @ZocoLini
- [#3488 chore(swift-sdk): add wrappers for the missing TransactionRecord fields in the swift-sdk](https://github.com/dashpay/platform/pull/3488) — 5 unresolved (5 human) · 26 days stale · ✋ changes requested
  - Top thread: "<!-- thepastaclaw-review v1 finding=106ff7904b9f -->" — 26 days old
- [#3479 test(swift-sdk): swift-sdk test updated and added to CI](https://github.com/dashpay/platform/pull/3479) — 1 unresolved (1 human) · 3 days stale · ✋ changes requested
  - Top thread: "<!-- thepastaclaw-review v1 finding=a25233924084 dedupe=4de506d00bbda291 -->" — 3 days old
- [#3463 chore(swift-sdk): remove unused code in the swift-sdk and example app](https://github.com/dashpay/platform/pull/3463) — 0 unresolved · 0 days stale · ⚠ merge conflict · ✋ changes requested
- [#3639 feat: external signable wallets and tx building with signer](https://github.com/dashpay/platform/pull/3639) — 0 unresolved · 0 days stale · ⚠ merge conflict

### @PastaPastaPasta
- [#3509 fix(platform): default omitted proved query limits](https://github.com/dashpay/platform/pull/3509) — 14 unresolved (1 CodeRabbit, 13 human) · 27 days stale · ✋ changes requested · 🔴 CI failing
  - Top thread: "_⚠️ Potential issue_ \| _🟠 Major_" — 27 days old
- [#3663 ci: include omitted rust packages in ci filters](https://github.com/dashpay/platform/pull/3663) — 2 unresolved (2 human) · 0 days stale
  - Top thread: "<!-- thepastaclaw-review v1 finding=867572ccbe1b dedupe=59d0d71e5f8ee2e0 -->" — 0 days old
- [#2989 feat(sdk): add credentials-based API for DPNS registerName](https://github.com/dashpay/platform/pull/2989) — 0 unresolved · 0 days stale · ✋ changes requested · 🔴 CI failing

### @thepastaclaw
- [#3091 feat(wasm-sdk): add prepare_* APIs for idempotent document state transitions](https://github.com/dashpay/platform/pull/3091) — 0 unresolved · 0 days stale · ✋ changes requested
- [#3092 feat(rs-sdk): expose transition hash from state transition methods](https://github.com/dashpay/platform/pull/3092) — 0 unresolved · 0 days stale · ✋ changes requested · 🔴 CI failing
- [#3133 fix(sdk): validate batch base structure before signing](https://github.com/dashpay/platform/pull/3133) — 0 unresolved · 0 days stale · ✋ changes requested

### @thephez
- [#3485 fix(wasm-sdk): update stale default testnet DAPI node addresses](https://github.com/dashpay/platform/pull/3485) — 4 unresolved (4 human) · 23 days stale · 🔴 CI failing
  - Top thread: "<!-- thepastaclaw-review v1 finding=b6db56a54dbd -->" — 23 days old
- [#3471 fix(dpp): remove erroneous keywords field from document-meta schema and fix contract keywords docs](https://github.com/dashpay/platform/pull/3471) — 2 unresolved (2 human) · 26 days stale · 🔴 CI failing
  - Top thread: "<!-- thepastaclaw-review v1 finding=72d0df4348f6 -->" — 26 days old

### @llbartekll
- [#3481 feat(swift-sdk): expose rich transaction fields in WalletTransaction](https://github.com/dashpay/platform/pull/3481) — 1 unresolved (1 CodeRabbit) · 36 days stale · ⚠ merge conflict · 🔴 CI failing
  - Top thread: "_⚠️ Potential issue_ \| _🟠 Major_" — 36 days old

### @Inna333-cuber
- [#2499 docs: replaced the icon with twitter](https://github.com/dashpay/platform/pull/2499) — 1 unresolved (1 human) · 49 days stale
  - Top thread: "<!-- thepastaclaw-review v1 finding=1314ffa7c907 -->" — 49 days old

## Methodology
Generated nightly by [pr-hygiene](https://github.com/dashpay/stale_prs_are_bad). A thread counts as "unresolved" when it is open, not outdated, has a comment from someone other than the PR author, and the most recent comment is from a reviewer. A PR is "dirty" when it has at least one such thread; "needs author action" further requires either changes-requested, merge conflict, or that the reviewer commented more recently than the author last pushed. Score weighs threads by severity (`high*5 + medium*2 + low*0.5`) and amplifies for staleness (`ln(oldest+1)`). Configurable via [`https://github.com/dashpay/stale_prs_are_bad/blob/master/.pr-hygiene.yml`](https://github.com/dashpay/stale_prs_are_bad/blob/master/.pr-hygiene.yml)—edit defaults there.

_No history snapshot from last week was found, so week-over-week deltas are omitted this run._
