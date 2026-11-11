---
---
# PR Hygiene Report
*Last updated: 2026-11-11 12:34 UTC · commit b100b61*

## Summary
- Open PRs: **184** (29 clean · 11 CI failing · 16 changes requested · 24 unresolved comments · 7 deferred · 43 draft · 54 stale)
- PRs needing author action: **47**
- Total unresolved comments: **183**
- dashpay/platform: **118** open (21 clean · 6 CI failing · 10 changes requested · 16 unresolved comments · 7 deferred · 22 draft · 36 stale) · engine: 23 draft · 3 ready-for-human · 4 ready-to-merge · 4 too-many-open-prs · 20 waiting-author · 11 waiting-bots · 2 waiting-build · 16 waiting-self-review · 35 no verdict
- dashpay/rust-dashcore: **23** open (5 clean · 0 CI failing · 3 changes requested · 3 unresolved comments · 0 deferred · 4 draft · 8 stale) · engine: 4 draft · 1 ready-for-human · 1 ready-to-merge · 2 waiting-author · 1 waiting-build · 6 waiting-self-review · 8 no verdict
- dashpay/tenderdash: **15** open (1 clean · 1 CI failing · 2 changes requested · 4 unresolved comments · 0 deferred · 7 draft · 0 stale) · engine: 7 draft · 3 waiting-author · 1 waiting-bots · 1 waiting-build · 3 waiting-self-review
- dashpay/grovedb: **1** open (0 clean · 0 CI failing · 0 changes requested · 0 unresolved comments · 0 deferred · 1 draft · 0 stale) · engine: 1 draft
- dashpay/dash-evo-tool: **27** open (2 clean · 4 CI failing · 1 changes requested · 1 unresolved comments · 0 deferred · 9 draft · 10 stale) · engine: 9 draft · 7 ready-for-human · 1 waiting-author · 10 no verdict

## Scoreboard
_Sort: unresolved-comments desc → needs-action desc → ready-for-review desc. Click any number to jump to the specific PRs it covers._

| Author | Open | Clean | CI failing | Unresolved Comments | Changes Requested | Deferred | Draft | Stale | Needs action | Ready for human | Total Unresolved Comments | Ready for Review | Δ |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| [@Claudius-Maginificent](#claudius-maginificent) | [55](#claudius-maginificent-open) | [2](#claudius-maginificent-clean) | [6](#claudius-maginificent-ci-failing) | [7](#claudius-maginificent-unresolved-comments) | [2](#claudius-maginificent-changes-requested) | [3](#claudius-maginificent-deferred) | [16](#claudius-maginificent-draft) | [19](#claudius-maginificent-stale) | [12](#claudius-maginificent-needs-action) | [8](#claudius-maginificent-ready-for-human) | [30](#claudius-maginificent-unresolved-comments) | — | ↑ 4 |
| [@QuantumExplorer](#quantumexplorer) | [27](#quantumexplorer-open) | [4](#quantumexplorer-clean) | [1](#quantumexplorer-ci-failing) | [5](#quantumexplorer-unresolved-comments) | [3](#quantumexplorer-changes-requested) | — | [5](#quantumexplorer-draft) | [9](#quantumexplorer-stale) | [8](#quantumexplorer-needs-action) | — | [45](#quantumexplorer-unresolved-comments) | [11](#quantumexplorer-ready-for-review) | ↑ 1 |
| [@PastaPastaPasta](#pastapastapasta) | [24](#pastapastapasta-open) | [7](#pastapastapasta-clean) | — | [2](#pastapastapasta-unresolved-comments) | [2](#pastapastapasta-changes-requested) | — | [11](#pastapastapasta-draft) | [2](#pastapastapasta-stale) | [5](#pastapastapasta-needs-action) | [1](#pastapastapasta-ready-for-human) | [42](#pastapastapasta-unresolved-comments) | [1](#pastapastapasta-ready-for-review) | ↑ 1 |
| [@romchornyi](#romchornyi) | [14](#romchornyi-open) | [4](#romchornyi-clean) | — | [2](#romchornyi-unresolved-comments) | [2](#romchornyi-changes-requested) | — | [4](#romchornyi-draft) | [2](#romchornyi-stale) | [4](#romchornyi-needs-action) | [1](#romchornyi-ready-for-human) | [13](#romchornyi-unresolved-comments) | [2](#romchornyi-ready-for-review) | ↑ 1 |
| [@bfoss765](#bfoss765) | [2](#bfoss765-open) | — | — | [2](#bfoss765-unresolved-comments) | — | — | — | — | [2](#bfoss765-needs-action) | — | [10](#bfoss765-unresolved-comments) | — | — |
| [@shumkov](#shumkov) | [16](#shumkov-open) | [3](#shumkov-clean) | [3](#shumkov-ci-failing) | [1](#shumkov-unresolved-comments) | [2](#shumkov-changes-requested) | — | [1](#shumkov-draft) | [6](#shumkov-stale) | [4](#shumkov-needs-action) | — | [25](#shumkov-unresolved-comments) | [8](#shumkov-ready-for-review) | ↓ 1 |
| [@ZocoLini](#zocolini) | [7](#zocolini-open) | [2](#zocolini-clean) | — | [1](#zocolini-unresolved-comments) | [2](#zocolini-changes-requested) | — | [2](#zocolini-draft) | — | [4](#zocolini-needs-action) | — | [1](#zocolini-unresolved-comments) | [8](#zocolini-ready-for-review) | ↑ 1 |
| [@lklimek](#lklimek) | [11](#lklimek-open) | — | — | [1](#lklimek-unresolved-comments) | [3](#lklimek-changes-requested) | [4](#lklimek-deferred) | [2](#lklimek-draft) | [1](#lklimek-stale) | [4](#lklimek-needs-action) | — | [8](#lklimek-unresolved-comments) | [6](#lklimek-ready-for-review) | ↓ 1 |
| [@HashEngineering](#hashengineering) | [5](#hashengineering-open) | [3](#hashengineering-clean) | — | [1](#hashengineering-unresolved-comments) | — | — | — | [1](#hashengineering-stale) | [1](#hashengineering-needs-action) | [2](#hashengineering-ready-for-human) | [4](#hashengineering-unresolved-comments) | [4](#hashengineering-ready-for-review) | — |
| [@xdustinface](#xdustinface) | [12](#xdustinface-open) | [1](#xdustinface-clean) | — | [1](#xdustinface-unresolved-comments) | — | — | [2](#xdustinface-draft) | [8](#xdustinface-stale) | [1](#xdustinface-needs-action) | [1](#xdustinface-ready-for-human) | [2](#xdustinface-unresolved-comments) | [3](#xdustinface-ready-for-review) | — |
| [@thepastaclaw](#thepastaclaw) | [2](#thepastaclaw-open) | [1](#thepastaclaw-clean) | — | [1](#thepastaclaw-unresolved-comments) | — | — | — | — | [1](#thepastaclaw-needs-action) | [1](#thepastaclaw-ready-for-human) | [2](#thepastaclaw-unresolved-comments) | — | ↑ 1 |
| [@ktechmidas](#ktechmidas) | [6](#ktechmidas-open) | [2](#ktechmidas-clean) | — | — | — | — | — | [4](#ktechmidas-stale) | [1](#ktechmidas-needs-action) | — | [1](#ktechmidas-unresolved-comments) | [2](#ktechmidas-ready-for-review) | — |
| [@llbartekll](#llbartekll) | — | — | — | — | — | — | — | — | — | — | — | [7](#llbartekll-ready-for-review) | ↓ 3 |
| [@DCG-Claude](#dcg-claude) | [3](#dcg-claude-open) | — | [1](#dcg-claude-ci-failing) | — | — | — | — | [2](#dcg-claude-stale) | — | [2](#dcg-claude-ready-for-human) | — | — | — |

## Per-author detail

<a id="claudius-maginificent"></a>
### @Claudius-Maginificent
<a id="claudius-maginificent-open"></a>
#### Open (55)
- [dashpay/platform#5645 fix(platform-wallet)!: check identity creation keys and funding in the Rust core](https://github.com/dashpay/platform/pull/5645) — 6 unresolved (4 CodeRabbit, 2 bot) · 8 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · areas: dpp, rs-platform-wallet, rs-platform-wallet-ffi, rust-sdk · Policy: waiting-author
  - Top thread: "&lt;picture&gt;&lt;img src="https://storage.googleapis.com/coderabbit_public_assets/review-comment-icons-v1/amber-16.svg" width="…" — 8 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: coderabbitai left review threads unresolved; resolve them
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/dash-evo-tool#1108 feat(documents): recognise indexOnly document types and delete their entries by values](https://github.com/dashpay/dash-evo-tool/pull/1108) — 3 unresolved (3 bot) · 19 days stale · ✋ changes requested · 📝 draft · 🐢 targets feat/document-optional-token-cost · areas: fallback
  - Top thread: "🟡 Suggestion: Make the non-deletable error actionable**" — 19 days old
- [dashpay/platform#5613 fix(platform-wallet-storage): persist sentinel-scoped token balances of wallet-owned identities](https://github.com/dashpay/platform/pull/5613) — 3 unresolved (3 bot) · 12 days stale · areas: wallet-storage · Policy: ready-to-merge
  - Top thread: "🟡 Suggestion: Assert the typed WalletIdMismatch payload**" — 12 days old
  - Blocker: All policy requirements are satisfied
- [dashpay/platform#5565 feat(platform-wallet)!: detect inconsistent shielded commitment trees and add explicit rebuild](https://github.com/dashpay/platform/pull/5565) — 3 unresolved (3 bot) · 12 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk, fallback · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Do not run the synchronous rebuild call on Swift's MainActor**" — 12 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
- [dashpay/dash-evo-tool#1117 fix(identity): choose signing keys Platform accepts on protocol 14](https://github.com/dashpay/dash-evo-tool/pull/1117) — 3 unresolved (3 bot) · 6 days stale · ✋ changes requested · 🔴 CI failing · areas: fallback · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Include create-consumption bounds in the upstream-selector follow-up**" — 6 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/platform#5546 fix(platform-wallet): support HASH160 DashPay contact signing keys](https://github.com/dashpay/platform/pull/5546) — 2 unresolved (2 bot) · 12 days stale · 🐢 targets feat/platform-wallet-public-signing-key-selection · areas: rs-drive-abci, dpp, rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk, fallback
  - Top thread: "🟡 Suggestion: Do not expose an unchecked signer-capability override in the public constructor**" — 12 days old
- [dashpay/tenderdash#1618 fix(blocksync): hand over to consensus at the tip despite unserved peer height claims](https://github.com/dashpay/tenderdash/pull/1618) — 4 unresolved (4 bot) · 0 days stale · ✋ changes requested · 🔴 CI failing · areas: fallback · Policy: waiting-author
  - Top thread: "🔴 Blocking: Credit the applied response independently of its pending entry**" — 0 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/tenderdash#1619 ci: assign merged pull requests to their release line's milestone](https://github.com/dashpay/tenderdash/pull/1619) — 3 unresolved (3 CodeRabbit) · 0 days stale · 🔴 CI failing · areas: github, fallback · Policy: waiting-bots
  - Top thread: "&lt;picture&gt;&lt;img src="https://storage.googleapis.com/coderabbit_public_assets/review-comment-icons-v1/amber-16.svg" width="…" — 0 days old
  - Blocker: thepastaclaw has not reported for the current head
  - Blocker: coderabbitai left review threads unresolved; resolve them
- [dashpay/dash-evo-tool#1124 feat(documents): let the user skip an optional token cost](https://github.com/dashpay/dash-evo-tool/pull/1124) — 1 unresolved (1 bot) · 16 days stale · ✋ changes requested · 📝 draft · 🐢 targets fix/protocol14-signing-key-selection · areas: fallback
  - Top thread: "🔴 Blocking: Show the credit-fee outcome when an optional token cost is disabled**" — 16 days old
- [dashpay/platform#5588 feat(platform-wallet)!: expose protocol-14-aware signing-key selection](https://github.com/dashpay/platform/pull/5588) — 1 unresolved (1 bot) · 12 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · areas: dpp, rs-platform-wallet, rs-platform-wallet-ffi, rust-sdk · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Include BLS keys in automatic document and token selection**" — 12 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
- [dashpay/platform#5707 fix(sdk): recover the nonce cache when unconfirmed nonces leave the accepted window](https://github.com/dashpay/platform/pull/5707) — 1 unresolved (1 bot) · 8 days stale · ✋ changes requested · 🔴 CI failing · 📝 draft · areas: rust-sdk · Policy: draft
  - Top thread: "🔴 Blocking: Do not reissue nonces belonging to outstanding operations**" — 8 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/dash-evo-tool#1109 feat(masternode): use wallet-held node keys and withdraw via platform-wallet](https://github.com/dashpay/dash-evo-tool/pull/1109) — ⚠ merge conflict · 🔴 CI failing · areas: fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
  - Blocker: Proceeded without thepastaclaw: no review within the configured window
- [dashpay/dash-evo-tool#1120 feat(identity): add keys with spending limits and raise key limits](https://github.com/dashpay/dash-evo-tool/pull/1120) — ⚠ merge conflict · 📝 draft · 🐢 targets fix/protocol14-signing-key-selection · areas: fallback
- [dashpay/dash-evo-tool#1121 feat(contracts): view and claim contract fee pots](https://github.com/dashpay/dash-evo-tool/pull/1121) — 📝 draft · 🐢 targets fix/protocol14-signing-key-selection · areas: fallback
- [dashpay/dash-evo-tool#1122 feat(documents): agree to document action fees before broadcasting](https://github.com/dashpay/dash-evo-tool/pull/1122) — ⚠ merge conflict · 📝 draft · 🐢 targets fix/protocol14-signing-key-selection · areas: fallback
- [dashpay/dash-evo-tool#1123 feat(identity): add authentication keys bound to a contract](https://github.com/dashpay/dash-evo-tool/pull/1123) — 📝 draft · 🐢 targets feat/identity-key-limits · areas: fallback
- [dashpay/dash-evo-tool#1128 refactor(tokens)!: send token actions through platform-wallet](https://github.com/dashpay/dash-evo-tool/pull/1128) — ⚠ merge conflict · 📝 draft · 🐢 targets chore/bump-platform-0d91c133 · areas: fallback
- [dashpay/dash-evo-tool#1137 fix(identity): unlock the wallet that holds the chosen signing key](https://github.com/dashpay/dash-evo-tool/pull/1137) — ⚠ merge conflict · 📝 draft · 🐢 targets fix/protocol14-signing-key-selection · areas: fallback
- [dashpay/dash-evo-tool#1153 chore(deps): bump dashpay/platform to v5.1-dev 6323a430](https://github.com/dashpay/dash-evo-tool/pull/1153) — ⚠ merge conflict · 🔴 CI failing · areas: fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
  - Blocker: Proceeded without thepastaclaw: no review within the configured window
- [dashpay/dash-evo-tool#1158 fix(security): keep formatted panic messages out of logs and the terminal](https://github.com/dashpay/dash-evo-tool/pull/1158) — 🔴 CI failing · areas: fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
  - Blocker: Proceeded without thepastaclaw: no review within the configured window
- [dashpay/dash-evo-tool#1159 fix(app): keep a previous network's background events and a request's progress where they belong](https://github.com/dashpay/dash-evo-tool/pull/1159) — areas: fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/dash-evo-tool#1160 feat(identity): show an identity's tokens in the Identities hub](https://github.com/dashpay/dash-evo-tool/pull/1160) — ✋ changes requested · 🔴 CI failing · areas: fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
  - Blocker: Proceeded without coderabbitai: no review within the configured window
  - Blocker: Proceeded without thepastaclaw: no review within the configured window
- [dashpay/dash-evo-tool#1162 fix(app): close promptly while a network switch is in progress](https://github.com/dashpay/dash-evo-tool/pull/1162) — ⚠ merge conflict · 📝 draft · areas: fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/dash-evo-tool#1165 fix(wallet): send Platform address transfers and withdrawals through platform-wallet](https://github.com/dashpay/dash-evo-tool/pull/1165) — 🔴 CI failing · areas: fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
  - Blocker: Proceeded without thepastaclaw: no review within the configured window
- [dashpay/dash-evo-tool#1166 fix(identity): reconcile address-funded identity registration through platform-wallet](https://github.com/dashpay/dash-evo-tool/pull/1166) — 🐢 targets fix/platform-address-ops-via-platform-wallet · areas: fallback
- [dashpay/dash-evo-tool#1167 perf(wallet): stop re-copying wallet funding state for Max quotes and snapshots](https://github.com/dashpay/dash-evo-tool/pull/1167) — areas: fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
- [dashpay/dash-evo-tool#1168 fix(tokens): keep the Token Creator's identity current and its registration state reliable](https://github.com/dashpay/dash-evo-tool/pull/1168) — 📝 draft · areas: fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/dash-evo-tool#1169 fix(ui): keep sub-navigation labels on one line](https://github.com/dashpay/dash-evo-tool/pull/1169) — 📝 draft · areas: fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/dash-evo-tool#1170 fix(ui): retire finished progress banners and describe key protection correctly](https://github.com/dashpay/dash-evo-tool/pull/1170) — 📝 draft · areas: fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/dash-evo-tool#1171 fix(wallet): show keys awaiting their old password right after a storage update](https://github.com/dashpay/dash-evo-tool/pull/1171) — 📝 draft · areas: fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/dash-evo-tool#1172 fix(mcp): keep embedded MCP requests' intermediate results off the GUI](https://github.com/dashpay/dash-evo-tool/pull/1172) — 📝 draft · areas: fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/dash-evo-tool#1173 feat(cli): import a wallet from a recovery phrase read from stdin](https://github.com/dashpay/dash-evo-tool/pull/1173) — 📝 draft · areas: fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/dash-evo-tool#1174 feat(skills): mark defects in visual-diff galleries](https://github.com/dashpay/dash-evo-tool/pull/1174) — 📝 draft · 🐢 targets feat/det-cli-mnemonic-stdin · areas: fallback
- [dashpay/dash-evo-tool#1175 docs: add maintainable token tutorial production toolkit](https://github.com/dashpay/dash-evo-tool/pull/1175) — 📝 draft · areas: fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#3549 test(platform-wallet): e2e framework + full test suite — triage pins, Found-*/PA-* guards, fail-closed persist, Stage-2 merge](https://github.com/dashpay/platform/pull/3549) — 📝 draft · ⏸ deferred · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk, rust-sdk, kotlin-sdk, github, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5384 fix(platform-wallet)!: persist chain truncations so a stored wallet follows a reorg](https://github.com/dashpay/platform/pull/5384) — ⏸ deferred · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk, kotlin-sdk, fallback
- [dashpay/platform#5417 fix(platform-wallet): own per-wallet transaction accounting in Rust for SwiftData](https://github.com/dashpay/platform/pull/5417) — ⏸ deferred · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk
- [dashpay/platform#5644 fix!: follow DIP-11 key rules when disabling keys and reading contact requests](https://github.com/dashpay/platform/pull/5644) — ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · areas: dpp, rs-platform-wallet, rs-platform-wallet-ffi, rust-sdk, js-wasm-sdk, fallback · Policy: waiting-author
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
- [dashpay/platform#5647 feat: expose the Rust identity key layout to FFI, Swift and Kotlin hosts](https://github.com/dashpay/platform/pull/5647) — ✋ changes requested · 🔴 CI failing · 🐢 targets fix/platform-wallet-dip11-identity-key-layout · areas: rs-platform-wallet-ffi, swift-sdk, kotlin-sdk, fallback
- [dashpay/platform#5660 refactor(dpp): own the contract-bounds verdict for a document create](https://github.com/dashpay/platform/pull/5660) — 🐢 targets feat/platform-wallet-public-signing-key-selection · areas: rs-drive-abci, dpp, rs-platform-wallet
- [dashpay/platform#5661 feat: decide which identity keys may be disabled in the wallet, not in the mobile apps](https://github.com/dashpay/platform/pull/5661) — ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · 🐢 targets fix/platform-wallet-dip11-key-rules · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk, fallback
- [dashpay/platform#5670 fix(platform)!: refuse high-S signatures on key-recovery paths from protocol version 15](https://github.com/dashpay/platform/pull/5670) — 📝 draft · 🐢 targets shumkov/v51-rust-dashcore-dev-bump · areas: rs-drive-abci, dpp, fallback
- [dashpay/platform#5683 fix(platform-wallet): fetch the identity before building an identity update](https://github.com/dashpay/platform/pull/5683) — ⚠ merge conflict · 🔴 CI failing · areas: rs-platform-wallet · Policy: waiting-build
  - Blocker: The build must pass before a human is asked
- [dashpay/platform#5781 fix(wallet-storage): validate transaction bodies during bulk history load](https://github.com/dashpay/platform/pull/5781) — 📝 draft · 🐢 targets fix/pr-5126 · areas: wallet-storage
- [dashpay/platform#5782 fix(wallet-storage): restore Platform and Core from one wallet snapshot](https://github.com/dashpay/platform/pull/5782) — 📝 draft · 🐢 targets feat/sqlite-wallet-restore · areas: wallet-storage
- [dashpay/platform#5783 fix(wallet-storage)!: reconcile historical AssetUnlock identities](https://github.com/dashpay/platform/pull/5783) — 📝 draft · 🐢 targets fix/sqlite-bulk-history-identity · areas: rs-platform-wallet, wallet-storage
- [dashpay/platform#5787 fix(platform-wallet): classify asset locks funding foreign credits as outgoing](https://github.com/dashpay/platform/pull/5787) — 📝 draft · 🐢 targets feat/sqlite-wallet-restore · areas: rs-platform-wallet, wallet-storage
- [dashpay/platform#5788 fix(platform-wallet): compile shielded-only drain helpers only with the shielded feature](https://github.com/dashpay/platform/pull/5788) — 📝 draft · areas: rs-platform-wallet · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5836 build!: upgrade Tenderdash to v2.0.0-dev.1](https://github.com/dashpay/platform/pull/5836) — ⚠ merge conflict · 📝 draft · areas: rs-drive-abci, rust-sdk, dashmate, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/tenderdash#1578 fix(statesync): refuse surplus light block responses without blocking](https://github.com/dashpay/tenderdash/pull/1578) — 📝 draft · areas: fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/tenderdash#1598 fix(consensus): periodically re-announce round state to peers](https://github.com/dashpay/tenderdash/pull/1598) — 📝 draft · areas: fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/tenderdash#1599 fix(consensus): resend same-round votes the peer never acknowledged](https://github.com/dashpay/tenderdash/pull/1599) — 📝 draft · areas: fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/tenderdash#1600 fix(consensus): deliver and process a polka block for a validator left in its round](https://github.com/dashpay/tenderdash/pull/1600) — 📝 draft · areas: fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/tenderdash#1601 fix(consensus): stop a previous-height precommit marking the peer's current-height votes](https://github.com/dashpay/tenderdash/pull/1601) — 📝 draft · areas: fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/tenderdash#1616 fix(consensus): validate a proposed block against state before ProcessProposal](https://github.com/dashpay/tenderdash/pull/1616) — 🔴 CI failing · areas: fallback · Policy: waiting-build
  - Blocker: The build must pass before a human is asked

<a id="claudius-maginificent-needs-action"></a>
#### Needs action (12)
- [dashpay/platform#5645 fix(platform-wallet)!: check identity creation keys and funding in the Rust core](https://github.com/dashpay/platform/pull/5645) — 6 unresolved (4 CodeRabbit, 2 bot) · 8 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · areas: dpp, rs-platform-wallet, rs-platform-wallet-ffi, rust-sdk · Policy: waiting-author
  - Top thread: "&lt;picture&gt;&lt;img src="https://storage.googleapis.com/coderabbit_public_assets/review-comment-icons-v1/amber-16.svg" width="…" — 8 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: coderabbitai left review threads unresolved; resolve them
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5613 fix(platform-wallet-storage): persist sentinel-scoped token balances of wallet-owned identities](https://github.com/dashpay/platform/pull/5613) — 3 unresolved (3 bot) · 12 days stale · areas: wallet-storage · Policy: ready-to-merge
  - Top thread: "🟡 Suggestion: Assert the typed WalletIdMismatch payload**" — 12 days old
  - Blocker: All policy requirements are satisfied
- [dashpay/platform#5565 feat(platform-wallet)!: detect inconsistent shielded commitment trees and add explicit rebuild](https://github.com/dashpay/platform/pull/5565) — 3 unresolved (3 bot) · 12 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk, fallback · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Do not run the synchronous rebuild call on Swift's MainActor**" — 12 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
- [dashpay/dash-evo-tool#1117 fix(identity): choose signing keys Platform accepts on protocol 14](https://github.com/dashpay/dash-evo-tool/pull/1117) — 3 unresolved (3 bot) · 6 days stale · ✋ changes requested · 🔴 CI failing · areas: fallback · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Include create-consumption bounds in the upstream-selector follow-up**" — 6 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/tenderdash#1618 fix(blocksync): hand over to consensus at the tip despite unserved peer height claims](https://github.com/dashpay/tenderdash/pull/1618) — 4 unresolved (4 bot) · 0 days stale · ✋ changes requested · 🔴 CI failing · areas: fallback · Policy: waiting-author
  - Top thread: "🔴 Blocking: Credit the applied response independently of its pending entry**" — 0 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/tenderdash#1619 ci: assign merged pull requests to their release line's milestone](https://github.com/dashpay/tenderdash/pull/1619) — 3 unresolved (3 CodeRabbit) · 0 days stale · 🔴 CI failing · areas: github, fallback · Policy: waiting-bots
  - Top thread: "&lt;picture&gt;&lt;img src="https://storage.googleapis.com/coderabbit_public_assets/review-comment-icons-v1/amber-16.svg" width="…" — 0 days old
  - Blocker: thepastaclaw has not reported for the current head
  - Blocker: coderabbitai left review threads unresolved; resolve them
- [dashpay/platform#5588 feat(platform-wallet)!: expose protocol-14-aware signing-key selection](https://github.com/dashpay/platform/pull/5588) — 1 unresolved (1 bot) · 12 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · areas: dpp, rs-platform-wallet, rs-platform-wallet-ffi, rust-sdk · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Include BLS keys in automatic document and token selection**" — 12 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
- [dashpay/dash-evo-tool#1109 feat(masternode): use wallet-held node keys and withdraw via platform-wallet](https://github.com/dashpay/dash-evo-tool/pull/1109) — ⚠ merge conflict · 🔴 CI failing · areas: fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
  - Blocker: Proceeded without thepastaclaw: no review within the configured window
- [dashpay/dash-evo-tool#1153 chore(deps): bump dashpay/platform to v5.1-dev 6323a430](https://github.com/dashpay/dash-evo-tool/pull/1153) — ⚠ merge conflict · 🔴 CI failing · areas: fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
  - Blocker: Proceeded without thepastaclaw: no review within the configured window
- [dashpay/dash-evo-tool#1160 feat(identity): show an identity's tokens in the Identities hub](https://github.com/dashpay/dash-evo-tool/pull/1160) — ✋ changes requested · 🔴 CI failing · areas: fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
  - Blocker: Proceeded without coderabbitai: no review within the configured window
  - Blocker: Proceeded without thepastaclaw: no review within the configured window
- [dashpay/platform#5644 fix!: follow DIP-11 key rules when disabling keys and reading contact requests](https://github.com/dashpay/platform/pull/5644) — ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · areas: dpp, rs-platform-wallet, rs-platform-wallet-ffi, rust-sdk, js-wasm-sdk, fallback · Policy: waiting-author
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
- [dashpay/platform#5683 fix(platform-wallet): fetch the identity before building an identity update](https://github.com/dashpay/platform/pull/5683) — ⚠ merge conflict · 🔴 CI failing · areas: rs-platform-wallet · Policy: waiting-build
  - Blocker: The build must pass before a human is asked

<a id="claudius-maginificent-ready-for-human"></a>
#### Ready for human (8)
- [dashpay/platform#5613 fix(platform-wallet-storage): persist sentinel-scoped token balances of wallet-owned identities](https://github.com/dashpay/platform/pull/5613) — 3 unresolved (3 bot) · 12 days stale · areas: wallet-storage · Policy: ready-to-merge
  - Top thread: "🟡 Suggestion: Assert the typed WalletIdMismatch payload**" — 12 days old
  - Blocker: All policy requirements are satisfied
- [dashpay/dash-evo-tool#1109 feat(masternode): use wallet-held node keys and withdraw via platform-wallet](https://github.com/dashpay/dash-evo-tool/pull/1109) — ⚠ merge conflict · 🔴 CI failing · areas: fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
  - Blocker: Proceeded without thepastaclaw: no review within the configured window
- [dashpay/dash-evo-tool#1153 chore(deps): bump dashpay/platform to v5.1-dev 6323a430](https://github.com/dashpay/dash-evo-tool/pull/1153) — ⚠ merge conflict · 🔴 CI failing · areas: fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
  - Blocker: Proceeded without thepastaclaw: no review within the configured window
- [dashpay/dash-evo-tool#1158 fix(security): keep formatted panic messages out of logs and the terminal](https://github.com/dashpay/dash-evo-tool/pull/1158) — 🔴 CI failing · areas: fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
  - Blocker: Proceeded without thepastaclaw: no review within the configured window
- [dashpay/dash-evo-tool#1159 fix(app): keep a previous network's background events and a request's progress where they belong](https://github.com/dashpay/dash-evo-tool/pull/1159) — areas: fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/dash-evo-tool#1160 feat(identity): show an identity's tokens in the Identities hub](https://github.com/dashpay/dash-evo-tool/pull/1160) — ✋ changes requested · 🔴 CI failing · areas: fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
  - Blocker: Proceeded without coderabbitai: no review within the configured window
  - Blocker: Proceeded without thepastaclaw: no review within the configured window
- [dashpay/dash-evo-tool#1165 fix(wallet): send Platform address transfers and withdrawals through platform-wallet](https://github.com/dashpay/dash-evo-tool/pull/1165) — 🔴 CI failing · areas: fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
  - Blocker: Proceeded without thepastaclaw: no review within the configured window
- [dashpay/dash-evo-tool#1167 perf(wallet): stop re-copying wallet funding state for Max quotes and snapshots](https://github.com/dashpay/dash-evo-tool/pull/1167) — areas: fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required

<a id="claudius-maginificent-unresolved-comments"></a>
#### Unresolved Comments (7)
- [dashpay/platform#5645 fix(platform-wallet)!: check identity creation keys and funding in the Rust core](https://github.com/dashpay/platform/pull/5645) — 6 unresolved (4 CodeRabbit, 2 bot) · 8 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · areas: dpp, rs-platform-wallet, rs-platform-wallet-ffi, rust-sdk · Policy: waiting-author
  - Top thread: "&lt;picture&gt;&lt;img src="https://storage.googleapis.com/coderabbit_public_assets/review-comment-icons-v1/amber-16.svg" width="…" — 8 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: coderabbitai left review threads unresolved; resolve them
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5613 fix(platform-wallet-storage): persist sentinel-scoped token balances of wallet-owned identities](https://github.com/dashpay/platform/pull/5613) — 3 unresolved (3 bot) · 12 days stale · areas: wallet-storage · Policy: ready-to-merge
  - Top thread: "🟡 Suggestion: Assert the typed WalletIdMismatch payload**" — 12 days old
  - Blocker: All policy requirements are satisfied
- [dashpay/platform#5565 feat(platform-wallet)!: detect inconsistent shielded commitment trees and add explicit rebuild](https://github.com/dashpay/platform/pull/5565) — 3 unresolved (3 bot) · 12 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk, fallback · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Do not run the synchronous rebuild call on Swift's MainActor**" — 12 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
- [dashpay/dash-evo-tool#1117 fix(identity): choose signing keys Platform accepts on protocol 14](https://github.com/dashpay/dash-evo-tool/pull/1117) — 3 unresolved (3 bot) · 6 days stale · ✋ changes requested · 🔴 CI failing · areas: fallback · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Include create-consumption bounds in the upstream-selector follow-up**" — 6 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/tenderdash#1618 fix(blocksync): hand over to consensus at the tip despite unserved peer height claims](https://github.com/dashpay/tenderdash/pull/1618) — 4 unresolved (4 bot) · 0 days stale · ✋ changes requested · 🔴 CI failing · areas: fallback · Policy: waiting-author
  - Top thread: "🔴 Blocking: Credit the applied response independently of its pending entry**" — 0 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/tenderdash#1619 ci: assign merged pull requests to their release line's milestone](https://github.com/dashpay/tenderdash/pull/1619) — 3 unresolved (3 CodeRabbit) · 0 days stale · 🔴 CI failing · areas: github, fallback · Policy: waiting-bots
  - Top thread: "&lt;picture&gt;&lt;img src="https://storage.googleapis.com/coderabbit_public_assets/review-comment-icons-v1/amber-16.svg" width="…" — 0 days old
  - Blocker: thepastaclaw has not reported for the current head
  - Blocker: coderabbitai left review threads unresolved; resolve them
- [dashpay/platform#5588 feat(platform-wallet)!: expose protocol-14-aware signing-key selection](https://github.com/dashpay/platform/pull/5588) — 1 unresolved (1 bot) · 12 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · areas: dpp, rs-platform-wallet, rs-platform-wallet-ffi, rust-sdk · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Include BLS keys in automatic document and token selection**" — 12 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix

<a id="claudius-maginificent-changes-requested"></a>
#### Changes Requested (2)
- [dashpay/dash-evo-tool#1160 feat(identity): show an identity's tokens in the Identities hub](https://github.com/dashpay/dash-evo-tool/pull/1160) — ✋ changes requested · 🔴 CI failing · areas: fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
  - Blocker: Proceeded without coderabbitai: no review within the configured window
  - Blocker: Proceeded without thepastaclaw: no review within the configured window
- [dashpay/platform#5644 fix!: follow DIP-11 key rules when disabling keys and reading contact requests](https://github.com/dashpay/platform/pull/5644) — ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · areas: dpp, rs-platform-wallet, rs-platform-wallet-ffi, rust-sdk, js-wasm-sdk, fallback · Policy: waiting-author
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix

<a id="claudius-maginificent-ci-failing"></a>
#### CI Failing (6)
- [dashpay/dash-evo-tool#1109 feat(masternode): use wallet-held node keys and withdraw via platform-wallet](https://github.com/dashpay/dash-evo-tool/pull/1109) — ⚠ merge conflict · 🔴 CI failing · areas: fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
  - Blocker: Proceeded without thepastaclaw: no review within the configured window
- [dashpay/dash-evo-tool#1153 chore(deps): bump dashpay/platform to v5.1-dev 6323a430](https://github.com/dashpay/dash-evo-tool/pull/1153) — ⚠ merge conflict · 🔴 CI failing · areas: fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
  - Blocker: Proceeded without thepastaclaw: no review within the configured window
- [dashpay/dash-evo-tool#1158 fix(security): keep formatted panic messages out of logs and the terminal](https://github.com/dashpay/dash-evo-tool/pull/1158) — 🔴 CI failing · areas: fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
  - Blocker: Proceeded without thepastaclaw: no review within the configured window
- [dashpay/dash-evo-tool#1165 fix(wallet): send Platform address transfers and withdrawals through platform-wallet](https://github.com/dashpay/dash-evo-tool/pull/1165) — 🔴 CI failing · areas: fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
  - Blocker: Proceeded without thepastaclaw: no review within the configured window
- [dashpay/platform#5683 fix(platform-wallet): fetch the identity before building an identity update](https://github.com/dashpay/platform/pull/5683) — ⚠ merge conflict · 🔴 CI failing · areas: rs-platform-wallet · Policy: waiting-build
  - Blocker: The build must pass before a human is asked
- [dashpay/tenderdash#1616 fix(consensus): validate a proposed block against state before ProcessProposal](https://github.com/dashpay/tenderdash/pull/1616) — 🔴 CI failing · areas: fallback · Policy: waiting-build
  - Blocker: The build must pass before a human is asked

<a id="claudius-maginificent-deferred"></a>
#### Deferred (3)
- [dashpay/platform#3549 test(platform-wallet): e2e framework + full test suite — triage pins, Found-*/PA-* guards, fail-closed persist, Stage-2 merge](https://github.com/dashpay/platform/pull/3549) — 📝 draft · ⏸ deferred · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk, rust-sdk, kotlin-sdk, github, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5384 fix(platform-wallet)!: persist chain truncations so a stored wallet follows a reorg](https://github.com/dashpay/platform/pull/5384) — ⏸ deferred · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk, kotlin-sdk, fallback
- [dashpay/platform#5417 fix(platform-wallet): own per-wallet transaction accounting in Rust for SwiftData](https://github.com/dashpay/platform/pull/5417) — ⏸ deferred · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk

<a id="claudius-maginificent-draft"></a>
#### Draft (16)
- [dashpay/platform#5707 fix(sdk): recover the nonce cache when unconfirmed nonces leave the accepted window](https://github.com/dashpay/platform/pull/5707) — 1 unresolved (1 bot) · 8 days stale · ✋ changes requested · 🔴 CI failing · 📝 draft · areas: rust-sdk · Policy: draft
  - Top thread: "🔴 Blocking: Do not reissue nonces belonging to outstanding operations**" — 8 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/dash-evo-tool#1162 fix(app): close promptly while a network switch is in progress](https://github.com/dashpay/dash-evo-tool/pull/1162) — ⚠ merge conflict · 📝 draft · areas: fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/dash-evo-tool#1168 fix(tokens): keep the Token Creator's identity current and its registration state reliable](https://github.com/dashpay/dash-evo-tool/pull/1168) — 📝 draft · areas: fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/dash-evo-tool#1169 fix(ui): keep sub-navigation labels on one line](https://github.com/dashpay/dash-evo-tool/pull/1169) — 📝 draft · areas: fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/dash-evo-tool#1170 fix(ui): retire finished progress banners and describe key protection correctly](https://github.com/dashpay/dash-evo-tool/pull/1170) — 📝 draft · areas: fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/dash-evo-tool#1171 fix(wallet): show keys awaiting their old password right after a storage update](https://github.com/dashpay/dash-evo-tool/pull/1171) — 📝 draft · areas: fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/dash-evo-tool#1172 fix(mcp): keep embedded MCP requests' intermediate results off the GUI](https://github.com/dashpay/dash-evo-tool/pull/1172) — 📝 draft · areas: fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/dash-evo-tool#1173 feat(cli): import a wallet from a recovery phrase read from stdin](https://github.com/dashpay/dash-evo-tool/pull/1173) — 📝 draft · areas: fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/dash-evo-tool#1175 docs: add maintainable token tutorial production toolkit](https://github.com/dashpay/dash-evo-tool/pull/1175) — 📝 draft · areas: fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5788 fix(platform-wallet): compile shielded-only drain helpers only with the shielded feature](https://github.com/dashpay/platform/pull/5788) — 📝 draft · areas: rs-platform-wallet · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5836 build!: upgrade Tenderdash to v2.0.0-dev.1](https://github.com/dashpay/platform/pull/5836) — ⚠ merge conflict · 📝 draft · areas: rs-drive-abci, rust-sdk, dashmate, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/tenderdash#1578 fix(statesync): refuse surplus light block responses without blocking](https://github.com/dashpay/tenderdash/pull/1578) — 📝 draft · areas: fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/tenderdash#1598 fix(consensus): periodically re-announce round state to peers](https://github.com/dashpay/tenderdash/pull/1598) — 📝 draft · areas: fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/tenderdash#1599 fix(consensus): resend same-round votes the peer never acknowledged](https://github.com/dashpay/tenderdash/pull/1599) — 📝 draft · areas: fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/tenderdash#1600 fix(consensus): deliver and process a polka block for a validator left in its round](https://github.com/dashpay/tenderdash/pull/1600) — 📝 draft · areas: fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/tenderdash#1601 fix(consensus): stop a previous-height precommit marking the peer's current-height votes](https://github.com/dashpay/tenderdash/pull/1601) — 📝 draft · areas: fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot

<a id="claudius-maginificent-stale"></a>
#### Stale (19)
- [dashpay/dash-evo-tool#1108 feat(documents): recognise indexOnly document types and delete their entries by values](https://github.com/dashpay/dash-evo-tool/pull/1108) — 3 unresolved (3 bot) · 19 days stale · ✋ changes requested · 📝 draft · 🐢 targets feat/document-optional-token-cost · areas: fallback
  - Top thread: "🟡 Suggestion: Make the non-deletable error actionable**" — 19 days old
- [dashpay/platform#5546 fix(platform-wallet): support HASH160 DashPay contact signing keys](https://github.com/dashpay/platform/pull/5546) — 2 unresolved (2 bot) · 12 days stale · 🐢 targets feat/platform-wallet-public-signing-key-selection · areas: rs-drive-abci, dpp, rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk, fallback
  - Top thread: "🟡 Suggestion: Do not expose an unchecked signer-capability override in the public constructor**" — 12 days old
- [dashpay/dash-evo-tool#1124 feat(documents): let the user skip an optional token cost](https://github.com/dashpay/dash-evo-tool/pull/1124) — 1 unresolved (1 bot) · 16 days stale · ✋ changes requested · 📝 draft · 🐢 targets fix/protocol14-signing-key-selection · areas: fallback
  - Top thread: "🔴 Blocking: Show the credit-fee outcome when an optional token cost is disabled**" — 16 days old
- [dashpay/dash-evo-tool#1120 feat(identity): add keys with spending limits and raise key limits](https://github.com/dashpay/dash-evo-tool/pull/1120) — ⚠ merge conflict · 📝 draft · 🐢 targets fix/protocol14-signing-key-selection · areas: fallback
- [dashpay/dash-evo-tool#1121 feat(contracts): view and claim contract fee pots](https://github.com/dashpay/dash-evo-tool/pull/1121) — 📝 draft · 🐢 targets fix/protocol14-signing-key-selection · areas: fallback
- [dashpay/dash-evo-tool#1122 feat(documents): agree to document action fees before broadcasting](https://github.com/dashpay/dash-evo-tool/pull/1122) — ⚠ merge conflict · 📝 draft · 🐢 targets fix/protocol14-signing-key-selection · areas: fallback
- [dashpay/dash-evo-tool#1123 feat(identity): add authentication keys bound to a contract](https://github.com/dashpay/dash-evo-tool/pull/1123) — 📝 draft · 🐢 targets feat/identity-key-limits · areas: fallback
- [dashpay/dash-evo-tool#1128 refactor(tokens)!: send token actions through platform-wallet](https://github.com/dashpay/dash-evo-tool/pull/1128) — ⚠ merge conflict · 📝 draft · 🐢 targets chore/bump-platform-0d91c133 · areas: fallback
- [dashpay/dash-evo-tool#1137 fix(identity): unlock the wallet that holds the chosen signing key](https://github.com/dashpay/dash-evo-tool/pull/1137) — ⚠ merge conflict · 📝 draft · 🐢 targets fix/protocol14-signing-key-selection · areas: fallback
- [dashpay/dash-evo-tool#1166 fix(identity): reconcile address-funded identity registration through platform-wallet](https://github.com/dashpay/dash-evo-tool/pull/1166) — 🐢 targets fix/platform-address-ops-via-platform-wallet · areas: fallback
- [dashpay/dash-evo-tool#1174 feat(skills): mark defects in visual-diff galleries](https://github.com/dashpay/dash-evo-tool/pull/1174) — 📝 draft · 🐢 targets feat/det-cli-mnemonic-stdin · areas: fallback
- [dashpay/platform#5647 feat: expose the Rust identity key layout to FFI, Swift and Kotlin hosts](https://github.com/dashpay/platform/pull/5647) — ✋ changes requested · 🔴 CI failing · 🐢 targets fix/platform-wallet-dip11-identity-key-layout · areas: rs-platform-wallet-ffi, swift-sdk, kotlin-sdk, fallback
- [dashpay/platform#5660 refactor(dpp): own the contract-bounds verdict for a document create](https://github.com/dashpay/platform/pull/5660) — 🐢 targets feat/platform-wallet-public-signing-key-selection · areas: rs-drive-abci, dpp, rs-platform-wallet
- [dashpay/platform#5661 feat: decide which identity keys may be disabled in the wallet, not in the mobile apps](https://github.com/dashpay/platform/pull/5661) — ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · 🐢 targets fix/platform-wallet-dip11-key-rules · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk, fallback
- [dashpay/platform#5670 fix(platform)!: refuse high-S signatures on key-recovery paths from protocol version 15](https://github.com/dashpay/platform/pull/5670) — 📝 draft · 🐢 targets shumkov/v51-rust-dashcore-dev-bump · areas: rs-drive-abci, dpp, fallback
- [dashpay/platform#5781 fix(wallet-storage): validate transaction bodies during bulk history load](https://github.com/dashpay/platform/pull/5781) — 📝 draft · 🐢 targets fix/pr-5126 · areas: wallet-storage
- [dashpay/platform#5782 fix(wallet-storage): restore Platform and Core from one wallet snapshot](https://github.com/dashpay/platform/pull/5782) — 📝 draft · 🐢 targets feat/sqlite-wallet-restore · areas: wallet-storage
- [dashpay/platform#5783 fix(wallet-storage)!: reconcile historical AssetUnlock identities](https://github.com/dashpay/platform/pull/5783) — 📝 draft · 🐢 targets fix/sqlite-bulk-history-identity · areas: rs-platform-wallet, wallet-storage
- [dashpay/platform#5787 fix(platform-wallet): classify asset locks funding foreign credits as outgoing](https://github.com/dashpay/platform/pull/5787) — 📝 draft · 🐢 targets feat/sqlite-wallet-restore · areas: rs-platform-wallet, wallet-storage

<a id="claudius-maginificent-clean"></a>
#### Clean (2)
- [dashpay/dash-evo-tool#1159 fix(app): keep a previous network's background events and a request's progress where they belong](https://github.com/dashpay/dash-evo-tool/pull/1159) — areas: fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/dash-evo-tool#1167 perf(wallet): stop re-copying wallet funding state for Max quotes and snapshots](https://github.com/dashpay/dash-evo-tool/pull/1167) — areas: fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required

<a id="quantumexplorer"></a>
### @QuantumExplorer
<a id="quantumexplorer-open"></a>
#### Open (27)
- [dashpay/rust-dashcore#1078 feat(key-wallet)!: lock masternode collateral out of coin selection](https://github.com/dashpay/rust-dashcore/pull/1078) — 8 unresolved (8 human) · 42 days stale · ⚠ merge conflict · ✋ changes requested · areas: key-wallet, key-wallet-manager · Policy: waiting-self-review
  - Top thread: "This sets the state by hand, and together with the serde round-trip it is the only coverage of the refresh inside \`updat…" — 42 days old
  - Blocker: Author must post /self-reviewed 44a9a010500fbd44aba09111b4c53a28b88e8073
- [dashpay/platform#4272 feat(dpp)!: dashpay payment detection keys and stealth derivation](https://github.com/dashpay/platform/pull/4272) — 9 unresolved (9 bot) · 100 days stale · ⚠ merge conflict · 📝 draft · areas: rs-drive, rs-drive-abci, dpp, rs-platform-wallet, swift-sdk, system-contracts, kotlin-sdk, fallback · Policy: draft
  - Top thread: "🔴 Blocking: Require compressed 33-byte encodings for payment keys**" — 100 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5750 feat(platform)!: random token transfer and mint amounts](https://github.com/dashpay/platform/pull/5750) — 8 unresolved (8 bot) · 8 days stale · 🐢 targets claude/platform-tenderdash-randomness-149e4e · areas: rs-drive, dpp, rs-platform-wallet, rust-sdk, rust-sdk-ffi, fallback
  - Top thread: "✅ **Resolved** at \`f8fcc8fd\`; see the replies below." — 8 days old
- [dashpay/platform#5749 feat(platform)!: sys.random.uniform document properties](https://github.com/dashpay/platform/pull/5749) — 8 unresolved (8 bot) · 8 days stale · areas: rs-drive, rs-drive-abci, dpp, fallback · Policy: waiting-author
  - Top thread: "✅ **Resolved** at \`0ead2b47\`; see the replies below." — 8 days old
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5753 feat(drive-abci)!: apply block randomness through the ApplyBlockRandomness ABCI method](https://github.com/dashpay/platform/pull/5753) — 7 unresolved (7 bot) · 7 days stale · ✋ changes requested · 🐢 targets claude/random-token-amounts · areas: rs-drive, rs-drive-abci, dpp, fallback
  - Top thread: "🟡 Suggestion: Exercise the protocol upgrade with signed commits**" — 7 days old
- [dashpay/platform#5529 feat(platform)!: token marketplace listings (PV15)](https://github.com/dashpay/platform/pull/5529) — 2 unresolved (2 bot) · 17 days stale · ✋ changes requested · areas: rs-drive, rs-drive-abci, dpp, rs-platform-wallet, swift-sdk, kotlin-sdk, fallback · Policy: waiting-author
  - Top thread: "🔴 Blocking: Version the consensus order-ID derivation through platform-version tables**" — 17 days old
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5530 feat(platform)!: token marketplace offers (PV15)](https://github.com/dashpay/platform/pull/5530) — 1 unresolved (1 bot) · 11 days stale · ✋ changes requested · 🔴 CI failing · 🐢 targets claude/token-market-listings · areas: rs-drive, rs-drive-abci, dpp, rs-platform-wallet, fallback
  - Top thread: "🔴 Blocking: Bound bid escrow by a buyer-signed credit limit**" — 11 days old
- [dashpay/tenderdash#1582 feat(abci)!: add the ApplyBlockRandomness method and commit beacon fields](https://github.com/dashpay/tenderdash/pull/1582) — 1 unresolved (1 bot) · 5 days stale · areas: fallback · Policy: waiting-author
  - Top thread: "✅ **Resolved** at \`cd6b4ec8\`; see the replies below." — 5 days old
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/tenderdash#1597 fix(p2p): trim the simple priority queue below its limit](https://github.com/dashpay/tenderdash/pull/1597) — 1 unresolved (1 bot) · 3 days stale · ⚠ merge conflict · areas: fallback · Policy: waiting-self-review
  - Top thread: "🟡 Suggestion: Count the larger overflow batches**" — 3 days old
  - Blocker: Author must post /self-reviewed fb3e02bfc772facdefade06c2eeca6743e23e329
- [dashpay/platform#3417 feat(swift-sdk): use SPV-synced quorums for Platform proof verification](https://github.com/dashpay/platform/pull/3417) — ✋ changes requested · 🔴 CI failing · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, rust-sdk, rust-sdk-ffi, fallback · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head
- [dashpay/platform#4519 feat(platform-wallet): rotate a masternode's keys into the wallet — ProUpRegTx orchestration, FFI, Swift](https://github.com/dashpay/platform/pull/4519) — ⚠ merge conflict · 🔴 CI failing · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 92a3ebe17adc12432fb756c63f172c663809007d
  - Blocker: Proceeded without thepastaclaw: no review within the configured window
- [dashpay/platform#4730 feat(platform)!: delta-based data contract update transition for protocol version 15](https://github.com/dashpay/platform/pull/4730) — ⚠ merge conflict · 🔴 CI failing · 📝 draft · areas: rs-drive, rs-drive-abci, dpp, rs-platform-wallet, rust-sdk, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#4776 feat(platform-wallet)!: let contract updates clear the description through the FFI](https://github.com/dashpay/platform/pull/4776) — ⚠ merge conflict · 🔴 CI failing · 🐢 targets feat/delta-contract-update-pv15 · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk, fallback
- [dashpay/platform#4986 fix(drive-abci)!: pay every reward share of a masternode and credit each identity once in the epoch payout (PV15)](https://github.com/dashpay/platform/pull/4986) — ⚠ merge conflict · 📝 draft · areas: rs-drive, rs-drive-abci, dpp, github, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5113 feat(platform-wallet)!: keep masternode collateral out of coin selection](https://github.com/dashpay/platform/pull/5113) — ⚠ merge conflict · 🔴 CI failing · 📝 draft · 🐢 targets chore/bump-rust-dashcore-secp-033 · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk, fallback
- [dashpay/platform#5531 feat(platform)!: record token marketplace events in token history (PV15)](https://github.com/dashpay/platform/pull/5531) — 🔴 CI failing · 🐢 targets claude/token-market-offers · areas: rs-drive, rs-drive-abci, dpp, system-contracts, fallback
- [dashpay/platform#5532 feat(platform)!: destroy a frozen identity's escrowed listings with its funds (PV15)](https://github.com/dashpay/platform/pull/5532) — 🔴 CI failing · 🐢 targets claude/token-market-history · areas: rs-drive, rs-drive-abci, fallback
- [dashpay/platform#5736 feat(platform)!: add a RedPallas signer key type with its own purpose (PV15)](https://github.com/dashpay/platform/pull/5736) — ⚠ merge conflict · 📝 draft · areas: rs-drive, rs-drive-abci, dpp, rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, rust-sdk-ffi, kotlin-sdk, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5751 feat(sdk): random token amounts in wasm-sdk and js-evo-sdk](https://github.com/dashpay/platform/pull/5751) — 🐢 targets claude/random-token-amounts · areas: js-wasm-sdk
- [dashpay/platform#5848 feat(platform)!: multiple choice decision proposals, voted on as a contest among documents (PV15)](https://github.com/dashpay/platform/pull/5848) — areas: rs-drive, rs-drive-abci, dpp, fallback · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head
- [dashpay/platform#5850 feat(platform)!: either-kind targets and slot conditions for document-named references (PV15)](https://github.com/dashpay/platform/pull/5850) — ✋ changes requested · areas: rs-drive-abci, dpp, fallback · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head
- [dashpay/platform#5851 feat(platform)!: multiple choice proposals take their description from a moderated text (PV15)](https://github.com/dashpay/platform/pull/5851) — ✋ changes requested · 🐢 targets claude/dynamic-refers-to-when · areas: rs-drive, rs-drive-abci, dpp, fallback
- [dashpay/platform#5859 fix(platform)!: end a contest without a winner when the award has no room (PV15)](https://github.com/dashpay/platform/pull/5859) — areas: rs-drive-abci, fallback · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head
- [dashpay/platform#5861 fix(platform)!: keys with a budget or an expiry sign no data contract create or update (PV15)](https://github.com/dashpay/platform/pull/5861) — areas: rs-drive-abci, dpp, fallback · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head
- [dashpay/tenderdash#1583 feat(consensus)!: recover a beacon from prevotes and sign precommits over the randomness-applied app hash](https://github.com/dashpay/tenderdash/pull/1583) — ⚠ merge conflict · 📝 draft · areas: fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/tenderdash#1584 fix(statesync): restore LastAppHash of the snapshot height, not the next one](https://github.com/dashpay/tenderdash/pull/1584) — areas: fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 23c1443e4077c7c7dfedf00f9446c762e669660f
- [dashpay/tenderdash#1595 fix(blocksync): serve blocks as the bytes they were committed with](https://github.com/dashpay/tenderdash/pull/1595) — ✋ changes requested · areas: fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed abfa42be14c81787071ca37a835b48155f509f98

<a id="quantumexplorer-needs-action"></a>
#### Needs action (8)
- [dashpay/rust-dashcore#1078 feat(key-wallet)!: lock masternode collateral out of coin selection](https://github.com/dashpay/rust-dashcore/pull/1078) — 8 unresolved (8 human) · 42 days stale · ⚠ merge conflict · ✋ changes requested · areas: key-wallet, key-wallet-manager · Policy: waiting-self-review
  - Top thread: "This sets the state by hand, and together with the serde round-trip it is the only coverage of the refresh inside \`updat…" — 42 days old
  - Blocker: Author must post /self-reviewed 44a9a010500fbd44aba09111b4c53a28b88e8073
- [dashpay/platform#5529 feat(platform)!: token marketplace listings (PV15)](https://github.com/dashpay/platform/pull/5529) — 2 unresolved (2 bot) · 17 days stale · ✋ changes requested · areas: rs-drive, rs-drive-abci, dpp, rs-platform-wallet, swift-sdk, kotlin-sdk, fallback · Policy: waiting-author
  - Top thread: "🔴 Blocking: Version the consensus order-ID derivation through platform-version tables**" — 17 days old
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/tenderdash#1582 feat(abci)!: add the ApplyBlockRandomness method and commit beacon fields](https://github.com/dashpay/tenderdash/pull/1582) — 1 unresolved (1 bot) · 5 days stale · areas: fallback · Policy: waiting-author
  - Top thread: "✅ **Resolved** at \`cd6b4ec8\`; see the replies below." — 5 days old
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/tenderdash#1597 fix(p2p): trim the simple priority queue below its limit](https://github.com/dashpay/tenderdash/pull/1597) — 1 unresolved (1 bot) · 3 days stale · ⚠ merge conflict · areas: fallback · Policy: waiting-self-review
  - Top thread: "🟡 Suggestion: Count the larger overflow batches**" — 3 days old
  - Blocker: Author must post /self-reviewed fb3e02bfc772facdefade06c2eeca6743e23e329
- [dashpay/platform#3417 feat(swift-sdk): use SPV-synced quorums for Platform proof verification](https://github.com/dashpay/platform/pull/3417) — ✋ changes requested · 🔴 CI failing · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, rust-sdk, rust-sdk-ffi, fallback · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head
- [dashpay/platform#4519 feat(platform-wallet): rotate a masternode's keys into the wallet — ProUpRegTx orchestration, FFI, Swift](https://github.com/dashpay/platform/pull/4519) — ⚠ merge conflict · 🔴 CI failing · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 92a3ebe17adc12432fb756c63f172c663809007d
  - Blocker: Proceeded without thepastaclaw: no review within the configured window
- [dashpay/platform#5850 feat(platform)!: either-kind targets and slot conditions for document-named references (PV15)](https://github.com/dashpay/platform/pull/5850) — ✋ changes requested · areas: rs-drive-abci, dpp, fallback · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head
- [dashpay/tenderdash#1595 fix(blocksync): serve blocks as the bytes they were committed with](https://github.com/dashpay/tenderdash/pull/1595) — ✋ changes requested · areas: fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed abfa42be14c81787071ca37a835b48155f509f98

<a id="quantumexplorer-unresolved-comments"></a>
#### Unresolved Comments (5)
- [dashpay/rust-dashcore#1078 feat(key-wallet)!: lock masternode collateral out of coin selection](https://github.com/dashpay/rust-dashcore/pull/1078) — 8 unresolved (8 human) · 42 days stale · ⚠ merge conflict · ✋ changes requested · areas: key-wallet, key-wallet-manager · Policy: waiting-self-review
  - Top thread: "This sets the state by hand, and together with the serde round-trip it is the only coverage of the refresh inside \`updat…" — 42 days old
  - Blocker: Author must post /self-reviewed 44a9a010500fbd44aba09111b4c53a28b88e8073
- [dashpay/platform#5749 feat(platform)!: sys.random.uniform document properties](https://github.com/dashpay/platform/pull/5749) — 8 unresolved (8 bot) · 8 days stale · areas: rs-drive, rs-drive-abci, dpp, fallback · Policy: waiting-author
  - Top thread: "✅ **Resolved** at \`0ead2b47\`; see the replies below." — 8 days old
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5529 feat(platform)!: token marketplace listings (PV15)](https://github.com/dashpay/platform/pull/5529) — 2 unresolved (2 bot) · 17 days stale · ✋ changes requested · areas: rs-drive, rs-drive-abci, dpp, rs-platform-wallet, swift-sdk, kotlin-sdk, fallback · Policy: waiting-author
  - Top thread: "🔴 Blocking: Version the consensus order-ID derivation through platform-version tables**" — 17 days old
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/tenderdash#1582 feat(abci)!: add the ApplyBlockRandomness method and commit beacon fields](https://github.com/dashpay/tenderdash/pull/1582) — 1 unresolved (1 bot) · 5 days stale · areas: fallback · Policy: waiting-author
  - Top thread: "✅ **Resolved** at \`cd6b4ec8\`; see the replies below." — 5 days old
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/tenderdash#1597 fix(p2p): trim the simple priority queue below its limit](https://github.com/dashpay/tenderdash/pull/1597) — 1 unresolved (1 bot) · 3 days stale · ⚠ merge conflict · areas: fallback · Policy: waiting-self-review
  - Top thread: "🟡 Suggestion: Count the larger overflow batches**" — 3 days old
  - Blocker: Author must post /self-reviewed fb3e02bfc772facdefade06c2eeca6743e23e329

<a id="quantumexplorer-changes-requested"></a>
#### Changes Requested (3)
- [dashpay/platform#3417 feat(swift-sdk): use SPV-synced quorums for Platform proof verification](https://github.com/dashpay/platform/pull/3417) — ✋ changes requested · 🔴 CI failing · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, rust-sdk, rust-sdk-ffi, fallback · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head
- [dashpay/platform#5850 feat(platform)!: either-kind targets and slot conditions for document-named references (PV15)](https://github.com/dashpay/platform/pull/5850) — ✋ changes requested · areas: rs-drive-abci, dpp, fallback · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head
- [dashpay/tenderdash#1595 fix(blocksync): serve blocks as the bytes they were committed with](https://github.com/dashpay/tenderdash/pull/1595) — ✋ changes requested · areas: fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed abfa42be14c81787071ca37a835b48155f509f98

<a id="quantumexplorer-ci-failing"></a>
#### CI Failing (1)
- [dashpay/platform#4519 feat(platform-wallet): rotate a masternode's keys into the wallet — ProUpRegTx orchestration, FFI, Swift](https://github.com/dashpay/platform/pull/4519) — ⚠ merge conflict · 🔴 CI failing · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 92a3ebe17adc12432fb756c63f172c663809007d
  - Blocker: Proceeded without thepastaclaw: no review within the configured window

<a id="quantumexplorer-draft"></a>
#### Draft (5)
- [dashpay/platform#4272 feat(dpp)!: dashpay payment detection keys and stealth derivation](https://github.com/dashpay/platform/pull/4272) — 9 unresolved (9 bot) · 100 days stale · ⚠ merge conflict · 📝 draft · areas: rs-drive, rs-drive-abci, dpp, rs-platform-wallet, swift-sdk, system-contracts, kotlin-sdk, fallback · Policy: draft
  - Top thread: "🔴 Blocking: Require compressed 33-byte encodings for payment keys**" — 100 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#4730 feat(platform)!: delta-based data contract update transition for protocol version 15](https://github.com/dashpay/platform/pull/4730) — ⚠ merge conflict · 🔴 CI failing · 📝 draft · areas: rs-drive, rs-drive-abci, dpp, rs-platform-wallet, rust-sdk, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#4986 fix(drive-abci)!: pay every reward share of a masternode and credit each identity once in the epoch payout (PV15)](https://github.com/dashpay/platform/pull/4986) — ⚠ merge conflict · 📝 draft · areas: rs-drive, rs-drive-abci, dpp, github, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5736 feat(platform)!: add a RedPallas signer key type with its own purpose (PV15)](https://github.com/dashpay/platform/pull/5736) — ⚠ merge conflict · 📝 draft · areas: rs-drive, rs-drive-abci, dpp, rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, rust-sdk-ffi, kotlin-sdk, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/tenderdash#1583 feat(consensus)!: recover a beacon from prevotes and sign precommits over the randomness-applied app hash](https://github.com/dashpay/tenderdash/pull/1583) — ⚠ merge conflict · 📝 draft · areas: fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot

<a id="quantumexplorer-stale"></a>
#### Stale (9)
- [dashpay/platform#5750 feat(platform)!: random token transfer and mint amounts](https://github.com/dashpay/platform/pull/5750) — 8 unresolved (8 bot) · 8 days stale · 🐢 targets claude/platform-tenderdash-randomness-149e4e · areas: rs-drive, dpp, rs-platform-wallet, rust-sdk, rust-sdk-ffi, fallback
  - Top thread: "✅ **Resolved** at \`f8fcc8fd\`; see the replies below." — 8 days old
- [dashpay/platform#5753 feat(drive-abci)!: apply block randomness through the ApplyBlockRandomness ABCI method](https://github.com/dashpay/platform/pull/5753) — 7 unresolved (7 bot) · 7 days stale · ✋ changes requested · 🐢 targets claude/random-token-amounts · areas: rs-drive, rs-drive-abci, dpp, fallback
  - Top thread: "🟡 Suggestion: Exercise the protocol upgrade with signed commits**" — 7 days old
- [dashpay/platform#5530 feat(platform)!: token marketplace offers (PV15)](https://github.com/dashpay/platform/pull/5530) — 1 unresolved (1 bot) · 11 days stale · ✋ changes requested · 🔴 CI failing · 🐢 targets claude/token-market-listings · areas: rs-drive, rs-drive-abci, dpp, rs-platform-wallet, fallback
  - Top thread: "🔴 Blocking: Bound bid escrow by a buyer-signed credit limit**" — 11 days old
- [dashpay/platform#4776 feat(platform-wallet)!: let contract updates clear the description through the FFI](https://github.com/dashpay/platform/pull/4776) — ⚠ merge conflict · 🔴 CI failing · 🐢 targets feat/delta-contract-update-pv15 · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk, fallback
- [dashpay/platform#5113 feat(platform-wallet)!: keep masternode collateral out of coin selection](https://github.com/dashpay/platform/pull/5113) — ⚠ merge conflict · 🔴 CI failing · 📝 draft · 🐢 targets chore/bump-rust-dashcore-secp-033 · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk, fallback
- [dashpay/platform#5531 feat(platform)!: record token marketplace events in token history (PV15)](https://github.com/dashpay/platform/pull/5531) — 🔴 CI failing · 🐢 targets claude/token-market-offers · areas: rs-drive, rs-drive-abci, dpp, system-contracts, fallback
- [dashpay/platform#5532 feat(platform)!: destroy a frozen identity's escrowed listings with its funds (PV15)](https://github.com/dashpay/platform/pull/5532) — 🔴 CI failing · 🐢 targets claude/token-market-history · areas: rs-drive, rs-drive-abci, fallback
- [dashpay/platform#5751 feat(sdk): random token amounts in wasm-sdk and js-evo-sdk](https://github.com/dashpay/platform/pull/5751) — 🐢 targets claude/random-token-amounts · areas: js-wasm-sdk
- [dashpay/platform#5851 feat(platform)!: multiple choice proposals take their description from a moderated text (PV15)](https://github.com/dashpay/platform/pull/5851) — ✋ changes requested · 🐢 targets claude/dynamic-refers-to-when · areas: rs-drive, rs-drive-abci, dpp, fallback

<a id="quantumexplorer-clean"></a>
#### Clean (4)
- [dashpay/platform#5848 feat(platform)!: multiple choice decision proposals, voted on as a contest among documents (PV15)](https://github.com/dashpay/platform/pull/5848) — areas: rs-drive, rs-drive-abci, dpp, fallback · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head
- [dashpay/platform#5859 fix(platform)!: end a contest without a winner when the award has no room (PV15)](https://github.com/dashpay/platform/pull/5859) — areas: rs-drive-abci, fallback · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head
- [dashpay/platform#5861 fix(platform)!: keys with a budget or an expiry sign no data contract create or update (PV15)](https://github.com/dashpay/platform/pull/5861) — areas: rs-drive-abci, dpp, fallback · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head
- [dashpay/tenderdash#1584 fix(statesync): restore LastAppHash of the snapshot height, not the next one](https://github.com/dashpay/tenderdash/pull/1584) — areas: fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 23c1443e4077c7c7dfedf00f9446c762e669660f

<a id="quantumexplorer-ready-for-review"></a>
#### Ready for Review (11)
- [dashpay/platform#4298 fix(dashmate): load ZeroSSL config in force mode](https://github.com/dashpay/platform/pull/4298) — by @thepastaclaw · areas: dashmate · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
- [dashpay/platform#5797 feat(kotlin-sdk): bind fund_from_asset_lock_external (Core → third-party Platform address)](https://github.com/dashpay/platform/pull/5797) — by @HashEngineering · areas: kotlin-sdk, fallback · Policy: ready-to-merge
  - Blocker: All policy requirements are satisfied
  - Blocker: Proceeded without thepastaclaw: no review within the configured window
- [dashpay/platform#5830 feat(kotlin-sdk): partial-amount platform-address withdrawal](https://github.com/dashpay/platform/pull/5830) — by @HashEngineering · areas: kotlin-sdk, fallback · Policy: ready-to-merge
  - Blocker: All policy requirements are satisfied
- [dashpay/platform#5854 fix(drive-abci): stop key-adding identity updates from taking the shielded proof verifier](https://github.com/dashpay/platform/pull/5854) — by @shumkov · areas: rs-drive-abci, fallback · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head
- [dashpay/platform#5858 fix(drive)!: estimate summable index entries with their sum items, and other PV15 fee estimate follow-ups](https://github.com/dashpay/platform/pull/5858) — by @shumkov · areas: rs-drive, fallback · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head
- [dashpay/platform#5860 feat(platform-wallet): top up an identity the wallet does not manage from Platform addresses](https://github.com/dashpay/platform/pull/5860) — by @romchornyi · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk, fallback · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head
- [dashpay/platform#5863 feat(swift-sdk): shieldedShieldFromIdentityToRecipient](https://github.com/dashpay/platform/pull/5863) — by @romchornyi · areas: rs-platform-wallet-ffi, swift-sdk, kotlin-sdk, fallback · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head
- [dashpay/rust-dashcore#1324 fix(dash-spv)!: refuse \`QRInfo\` entries missing from the tip list](https://github.com/dashpay/rust-dashcore/pull/1324) — by @PastaPastaPasta · areas: dash-spv, fallback · Policy: waiting-build
  - Blocker: Waiting for the build to finish
- [dashpay/rust-dashcore#1367 test(key-wallet-manager): cover rescans over chainlocked blocks](https://github.com/dashpay/rust-dashcore/pull/1367) — by @PastaPastaPasta · areas: key-wallet, key-wallet-manager · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
- [dashpay/rust-dashcore#1378 fix(key-wallet)!: return errors from managed account constructors](https://github.com/dashpay/rust-dashcore/pull/1378) — by @ZocoLini · areas: key-wallet · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed a165a33c8546a391663e0cc6dd2638100f56192f
- [dashpay/rust-dashcore#1387 fix(dash-spv): keep re-queued work off kicked peers](https://github.com/dashpay/rust-dashcore/pull/1387) — by @xdustinface · areas: dash-spv · Policy: ready-to-merge
  - Blocker: All policy requirements are satisfied

<a id="pastapastapasta"></a>
### @PastaPastaPasta
<a id="pastapastapasta-open"></a>
#### Open (24)
- [dashpay/platform#5709 feat(platform-wallet): own the shielded-tip payment decisions in the wallet](https://github.com/dashpay/platform/pull/5709) — 22 unresolved (22 bot) · 10 days stale · 📝 draft · 🐢 targets feat/dashpay-shielded-tips · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, rust-sdk, kotlin-sdk, fallback
  - Top thread: "✅ **Resolved** at \`688c75e1\`; see the replies below." — 10 days old
- [dashpay/platform#4648 feat(drive-abci)!: state sync via ABCI snapshots with reduced platform state (protocol v15)](https://github.com/dashpay/platform/pull/4648) — 8 unresolved (8 bot) · 37 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · 📝 draft · areas: rs-drive, rs-drive-abci, dpp, fallback · Policy: draft
  - Top thread: "🟡 Suggestion: Continue the restored target through consensus and a restart**" — 37 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5288 feat(platform-wallet): support DashPay shielded tips with dedicated accounts](https://github.com/dashpay/platform/pull/5288) — 4 unresolved (1 CodeRabbit, 3 bot) · 10 days stale · ⚠ merge conflict · 🔴 CI failing · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk, rust-sdk, kotlin-sdk, fallback · Policy: waiting-author
  - Top thread: "&lt;picture&gt;&lt;img src="https://storage.googleapis.com/coderabbit_public_assets/review-comment-icons-v1/amber-16.svg" width="…" — 10 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: coderabbitai left review threads unresolved; resolve them
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5711 feat: shielded-tip screens over the wallet's tip decisions](https://github.com/dashpay/platform/pull/5711) — 4 unresolved (4 bot) · 10 days stale · 📝 draft · 🐢 targets feat/dashpay-shielded-tips-wallet-policy · areas: swift-sdk, kotlin-sdk
  - Top thread: "✅ **Resolved** at \`451dab59\`; see the replies below." — 10 days old
- [dashpay/platform#5786 feat(sdk): tri-state quorum lookups, bounded funding retries and a DPNS contest policy in dash-sdk-cxx](https://github.com/dashpay/platform/pull/5786) — 3 unresolved (3 bot) · 4 days stale · areas: rs-drive, rs-platform-wallet, rust-sdk, js-wasm-sdk, fallback · Policy: too-many-open-prs
  - Top thread: "✅ **Resolved** at \`e46d43eb\`; see the replies below." — 4 days old
  - Blocker: Human review is still required; automatic reviewer requests are paused until a review slot is available
- [dashpay/platform#4965 test(sdk): cargo-fuzz harness over drive-proof-verifier FromProof](https://github.com/dashpay/platform/pull/4965) — 1 unresolved (1 bot) · 36 days stale · 📝 draft · areas: github, fallback · Policy: draft
  - Top thread: "🟡 Suggestion: Correct the claim that PV13 reaches the V0 proof decoder**" — 36 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/grovedb#1007 build(deps)!: use dashpay/orchard with the faster prover](https://github.com/dashpay/grovedb/pull/1007) — 🔴 CI failing · 📝 draft · areas: fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5277 perf(platform-wallet): don't hold the shielded store lock across sync network I/O](https://github.com/dashpay/platform/pull/5277) — areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed de486366461d17ef4845e90de4961499b0b1faeb
- [dashpay/platform#5280 perf(platform-wallet)!: reuse a recent recorded-anchor set on the shielded spend path](https://github.com/dashpay/platform/pull/5280) — ⚠ merge conflict · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed e56af70d1e78f203f4aa18aeeae640a0f75a351e
- [dashpay/platform#5282 perf(platform-wallet)!: prove the shield bundle while waiting for the InstantSend lock](https://github.com/dashpay/platform/pull/5282) — ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · areas: dpp, rs-platform-wallet, rs-platform-wallet-ffi, fallback · Policy: waiting-author
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
- [dashpay/platform#5285 feat(platform)!: subscribe to committed state transitions matching document and address filters](https://github.com/dashpay/platform/pull/5285) — ⚠ merge conflict · 📝 draft · areas: rs-drive, rs-drive-abci, rust-sdk, dashmate, rust-dapi, github, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5289 feat(sdk)!: optional BIP-39 passphrase through the mnemonic resolver and wallet creation](https://github.com/dashpay/platform/pull/5289) — ⚠ merge conflict · 📝 draft · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, rust-sdk-ffi, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5292 feat(dashmate): state sync configuration for tenderdash and drive snapshots](https://github.com/dashpay/platform/pull/5292) — ⚠ merge conflict · 📝 draft · areas: dashmate · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5475 feat(sdk): forward DashPay profiles, invitations and masternode voting in dash-sdk-cxx](https://github.com/dashpay/platform/pull/5475) — ⚠ merge conflict · 📝 draft · areas: rs-platform-wallet, rs-platform-wallet-ffi, rust-sdk, github, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5577 feat: gzip-compress DAPI responses](https://github.com/dashpay/platform/pull/5577) — ✋ changes requested · 🔴 CI failing · areas: dashmate, rust-dapi, fallback · Policy: waiting-author
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
- [dashpay/platform#5610 perf(sdk): verify each block's signature once instead of on every proved response](https://github.com/dashpay/platform/pull/5610) — ⚠ merge conflict · 📝 draft · areas: fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5780 fix(platform-wallet-storage): restore UTXOs into the account that derives them](https://github.com/dashpay/platform/pull/5780) — areas: wallet-storage · Policy: too-many-open-prs
  - Blocker: Human review is still required; automatic reviewer requests are paused until a review slot is available
- [dashpay/platform#5784 fix(platform-wallet): export AssetLockBuildAmount](https://github.com/dashpay/platform/pull/5784) — 📝 draft · areas: rs-platform-wallet · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5792 fix(platform-wallet): persist coins a block's re-scan credits](https://github.com/dashpay/platform/pull/5792) — 📝 draft · areas: rs-platform-wallet, wallet-storage · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5793 fix(platform-wallet): make SpvRuntime::header_hash_at public](https://github.com/dashpay/platform/pull/5793) — areas: rs-platform-wallet · Policy: too-many-open-prs
  - Blocker: Human review is still required; automatic reviewer requests are paused until a review slot is available
- [dashpay/platform#5794 feat(platform-wallet-storage): read one asset lock's status by outpoint](https://github.com/dashpay/platform/pull/5794) — areas: wallet-storage · Policy: too-many-open-prs
  - Blocker: Human review is still required; automatic reviewer requests are paused until a review slot is available
- [dashpay/platform#5802 fix(platform-wallet-storage): don't look for or count non-funds used addresses as unresolved](https://github.com/dashpay/platform/pull/5802) — 📝 draft · areas: wallet-storage, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#1324 fix(dash-spv)!: refuse \`QRInfo\` entries missing from the tip list](https://github.com/dashpay/rust-dashcore/pull/1324) — areas: dash-spv, fallback · Policy: waiting-build
  - Blocker: Waiting for the build to finish
- [dashpay/rust-dashcore#1367 test(key-wallet-manager): cover rescans over chainlocked blocks](https://github.com/dashpay/rust-dashcore/pull/1367) — areas: key-wallet, key-wallet-manager · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required

<a id="pastapastapasta-needs-action"></a>
#### Needs action (5)
- [dashpay/platform#5288 feat(platform-wallet): support DashPay shielded tips with dedicated accounts](https://github.com/dashpay/platform/pull/5288) — 4 unresolved (1 CodeRabbit, 3 bot) · 10 days stale · ⚠ merge conflict · 🔴 CI failing · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk, rust-sdk, kotlin-sdk, fallback · Policy: waiting-author
  - Top thread: "&lt;picture&gt;&lt;img src="https://storage.googleapis.com/coderabbit_public_assets/review-comment-icons-v1/amber-16.svg" width="…" — 10 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: coderabbitai left review threads unresolved; resolve them
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5786 feat(sdk): tri-state quorum lookups, bounded funding retries and a DPNS contest policy in dash-sdk-cxx](https://github.com/dashpay/platform/pull/5786) — 3 unresolved (3 bot) · 4 days stale · areas: rs-drive, rs-platform-wallet, rust-sdk, js-wasm-sdk, fallback · Policy: too-many-open-prs
  - Top thread: "✅ **Resolved** at \`e46d43eb\`; see the replies below." — 4 days old
  - Blocker: Human review is still required; automatic reviewer requests are paused until a review slot is available
- [dashpay/platform#5280 perf(platform-wallet)!: reuse a recent recorded-anchor set on the shielded spend path](https://github.com/dashpay/platform/pull/5280) — ⚠ merge conflict · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed e56af70d1e78f203f4aa18aeeae640a0f75a351e
- [dashpay/platform#5282 perf(platform-wallet)!: prove the shield bundle while waiting for the InstantSend lock](https://github.com/dashpay/platform/pull/5282) — ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · areas: dpp, rs-platform-wallet, rs-platform-wallet-ffi, fallback · Policy: waiting-author
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
- [dashpay/platform#5577 feat: gzip-compress DAPI responses](https://github.com/dashpay/platform/pull/5577) — ✋ changes requested · 🔴 CI failing · areas: dashmate, rust-dapi, fallback · Policy: waiting-author
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix

<a id="pastapastapasta-ready-for-human"></a>
#### Ready for human (1)
- [dashpay/rust-dashcore#1367 test(key-wallet-manager): cover rescans over chainlocked blocks](https://github.com/dashpay/rust-dashcore/pull/1367) — areas: key-wallet, key-wallet-manager · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required

<a id="pastapastapasta-unresolved-comments"></a>
#### Unresolved Comments (2)
- [dashpay/platform#5288 feat(platform-wallet): support DashPay shielded tips with dedicated accounts](https://github.com/dashpay/platform/pull/5288) — 4 unresolved (1 CodeRabbit, 3 bot) · 10 days stale · ⚠ merge conflict · 🔴 CI failing · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk, rust-sdk, kotlin-sdk, fallback · Policy: waiting-author
  - Top thread: "&lt;picture&gt;&lt;img src="https://storage.googleapis.com/coderabbit_public_assets/review-comment-icons-v1/amber-16.svg" width="…" — 10 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: coderabbitai left review threads unresolved; resolve them
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5786 feat(sdk): tri-state quorum lookups, bounded funding retries and a DPNS contest policy in dash-sdk-cxx](https://github.com/dashpay/platform/pull/5786) — 3 unresolved (3 bot) · 4 days stale · areas: rs-drive, rs-platform-wallet, rust-sdk, js-wasm-sdk, fallback · Policy: too-many-open-prs
  - Top thread: "✅ **Resolved** at \`e46d43eb\`; see the replies below." — 4 days old
  - Blocker: Human review is still required; automatic reviewer requests are paused until a review slot is available

<a id="pastapastapasta-changes-requested"></a>
#### Changes Requested (2)
- [dashpay/platform#5282 perf(platform-wallet)!: prove the shield bundle while waiting for the InstantSend lock](https://github.com/dashpay/platform/pull/5282) — ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · areas: dpp, rs-platform-wallet, rs-platform-wallet-ffi, fallback · Policy: waiting-author
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
- [dashpay/platform#5577 feat: gzip-compress DAPI responses](https://github.com/dashpay/platform/pull/5577) — ✋ changes requested · 🔴 CI failing · areas: dashmate, rust-dapi, fallback · Policy: waiting-author
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix

<a id="pastapastapasta-draft"></a>
#### Draft (11)
- [dashpay/platform#4648 feat(drive-abci)!: state sync via ABCI snapshots with reduced platform state (protocol v15)](https://github.com/dashpay/platform/pull/4648) — 8 unresolved (8 bot) · 37 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · 📝 draft · areas: rs-drive, rs-drive-abci, dpp, fallback · Policy: draft
  - Top thread: "🟡 Suggestion: Continue the restored target through consensus and a restart**" — 37 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#4965 test(sdk): cargo-fuzz harness over drive-proof-verifier FromProof](https://github.com/dashpay/platform/pull/4965) — 1 unresolved (1 bot) · 36 days stale · 📝 draft · areas: github, fallback · Policy: draft
  - Top thread: "🟡 Suggestion: Correct the claim that PV13 reaches the V0 proof decoder**" — 36 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/grovedb#1007 build(deps)!: use dashpay/orchard with the faster prover](https://github.com/dashpay/grovedb/pull/1007) — 🔴 CI failing · 📝 draft · areas: fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5285 feat(platform)!: subscribe to committed state transitions matching document and address filters](https://github.com/dashpay/platform/pull/5285) — ⚠ merge conflict · 📝 draft · areas: rs-drive, rs-drive-abci, rust-sdk, dashmate, rust-dapi, github, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5289 feat(sdk)!: optional BIP-39 passphrase through the mnemonic resolver and wallet creation](https://github.com/dashpay/platform/pull/5289) — ⚠ merge conflict · 📝 draft · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, rust-sdk-ffi, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5292 feat(dashmate): state sync configuration for tenderdash and drive snapshots](https://github.com/dashpay/platform/pull/5292) — ⚠ merge conflict · 📝 draft · areas: dashmate · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5475 feat(sdk): forward DashPay profiles, invitations and masternode voting in dash-sdk-cxx](https://github.com/dashpay/platform/pull/5475) — ⚠ merge conflict · 📝 draft · areas: rs-platform-wallet, rs-platform-wallet-ffi, rust-sdk, github, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5610 perf(sdk): verify each block's signature once instead of on every proved response](https://github.com/dashpay/platform/pull/5610) — ⚠ merge conflict · 📝 draft · areas: fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5784 fix(platform-wallet): export AssetLockBuildAmount](https://github.com/dashpay/platform/pull/5784) — 📝 draft · areas: rs-platform-wallet · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5792 fix(platform-wallet): persist coins a block's re-scan credits](https://github.com/dashpay/platform/pull/5792) — 📝 draft · areas: rs-platform-wallet, wallet-storage · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5802 fix(platform-wallet-storage): don't look for or count non-funds used addresses as unresolved](https://github.com/dashpay/platform/pull/5802) — 📝 draft · areas: wallet-storage, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot

<a id="pastapastapasta-stale"></a>
#### Stale (2)
- [dashpay/platform#5709 feat(platform-wallet): own the shielded-tip payment decisions in the wallet](https://github.com/dashpay/platform/pull/5709) — 22 unresolved (22 bot) · 10 days stale · 📝 draft · 🐢 targets feat/dashpay-shielded-tips · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, rust-sdk, kotlin-sdk, fallback
  - Top thread: "✅ **Resolved** at \`688c75e1\`; see the replies below." — 10 days old
- [dashpay/platform#5711 feat: shielded-tip screens over the wallet's tip decisions](https://github.com/dashpay/platform/pull/5711) — 4 unresolved (4 bot) · 10 days stale · 📝 draft · 🐢 targets feat/dashpay-shielded-tips-wallet-policy · areas: swift-sdk, kotlin-sdk
  - Top thread: "✅ **Resolved** at \`451dab59\`; see the replies below." — 10 days old

<a id="pastapastapasta-clean"></a>
#### Clean (7)
- [dashpay/platform#5277 perf(platform-wallet): don't hold the shielded store lock across sync network I/O](https://github.com/dashpay/platform/pull/5277) — areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed de486366461d17ef4845e90de4961499b0b1faeb
- [dashpay/platform#5280 perf(platform-wallet)!: reuse a recent recorded-anchor set on the shielded spend path](https://github.com/dashpay/platform/pull/5280) — ⚠ merge conflict · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed e56af70d1e78f203f4aa18aeeae640a0f75a351e
- [dashpay/platform#5780 fix(platform-wallet-storage): restore UTXOs into the account that derives them](https://github.com/dashpay/platform/pull/5780) — areas: wallet-storage · Policy: too-many-open-prs
  - Blocker: Human review is still required; automatic reviewer requests are paused until a review slot is available
- [dashpay/platform#5793 fix(platform-wallet): make SpvRuntime::header_hash_at public](https://github.com/dashpay/platform/pull/5793) — areas: rs-platform-wallet · Policy: too-many-open-prs
  - Blocker: Human review is still required; automatic reviewer requests are paused until a review slot is available
- [dashpay/platform#5794 feat(platform-wallet-storage): read one asset lock's status by outpoint](https://github.com/dashpay/platform/pull/5794) — areas: wallet-storage · Policy: too-many-open-prs
  - Blocker: Human review is still required; automatic reviewer requests are paused until a review slot is available
- [dashpay/rust-dashcore#1324 fix(dash-spv)!: refuse \`QRInfo\` entries missing from the tip list](https://github.com/dashpay/rust-dashcore/pull/1324) — areas: dash-spv, fallback · Policy: waiting-build
  - Blocker: Waiting for the build to finish
- [dashpay/rust-dashcore#1367 test(key-wallet-manager): cover rescans over chainlocked blocks](https://github.com/dashpay/rust-dashcore/pull/1367) — areas: key-wallet, key-wallet-manager · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required

<a id="pastapastapasta-ready-for-review"></a>
#### Ready for Review (1)
- [dashpay/platform#4298 fix(dashmate): load ZeroSSL config in force mode](https://github.com/dashpay/platform/pull/4298) — by @thepastaclaw · areas: dashmate · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required

<a id="romchornyi"></a>
### @romchornyi
<a id="romchornyi-open"></a>
#### Open (14)
- [dashpay/platform#5258 feat(platform-wallet): probe the network about sends with an unknown broadcast outcome](https://github.com/dashpay/platform/pull/5258) — 3 unresolved (3 human) · 26 days stale · ⚠ merge conflict · 📝 draft · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, rust-sdk · Policy: draft
  - Top thread: "\[P2\] Carry the originating wallet generation in probe events" — 26 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5540 fix(platform-wallet): store externally supplied identity keys without a derivation breadcrumb](https://github.com/dashpay/platform/pull/5540) — 3 unresolved (3 bot) · 20 days stale · 🔴 CI failing · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk, fallback · Policy: waiting-author
  - Top thread: "✅ **Resolved** at \`b25b2543\`; see the replies below." — 20 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5549 fix(platform-wallet-storage): restore DashPay contact address pools on SQLite load](https://github.com/dashpay/platform/pull/5549) — 3 unresolved (3 bot) · 19 days stale · ⚠ merge conflict · 🔴 CI failing · 📝 draft · areas: wallet-storage · Policy: draft
  - Top thread: "✅ **Withdrawn** at \`2bee2705\`; see the replies below." — 19 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5536 fix: check a DashPay invitation claim against Platform before broadcasting it](https://github.com/dashpay/platform/pull/5536) — 3 unresolved (1 CodeRabbit, 2 bot) · 16 days stale · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, rust-sdk, kotlin-sdk, fallback · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Guard the new invitation-status C entry point against Rust panics**" — 16 days old
  - Blocker: coderabbitai left review threads unresolved; resolve them
- [dashpay/platform#5395 feat(platform-wallet): spend only final coins and report funds waiting on the network](https://github.com/dashpay/platform/pull/5395) — 1 unresolved (1 human) · 26 days stale · ⚠ merge conflict · ✋ changes requested · 📝 draft · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk, fallback · Policy: draft
  - Top thread: "this should belong to another layer - likely platform-wallet, or even rust-dashcore's key-wallet. FFI is not meant to st…" — 26 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#4992 feat(swift-sdk): durable wallet presence marker readable while the device is locked](https://github.com/dashpay/platform/pull/4992) — areas: swift-sdk · Policy: ready-to-merge
  - Blocker: All policy requirements are satisfied
- [dashpay/platform#5587 fix(platform-wallet)!: keep a contact's payment account across relaunches and replace it cleanly on rotation](https://github.com/dashpay/platform/pull/5587) — ⚠ merge conflict · 📝 draft · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5589 feat(platform-wallet-ffi)!: let hosts reset an outbound contact account's pool and keep its request digest](https://github.com/dashpay/platform/pull/5589) — 🔴 CI failing · 📝 draft · 🐢 targets fix/contact-rebuild-keeps-pool · areas: rs-platform-wallet-ffi, swift-sdk
- [dashpay/platform#5590 feat(kotlin-sdk)!: reset an outbound contact account's pool and keep its request digest on Android](https://github.com/dashpay/platform/pull/5590) — 🔴 CI failing · 📝 draft · 🐢 targets fix/contact-rebuild-ffi-swift · areas: kotlin-sdk, fallback
- [dashpay/platform#5659 fix(dapi): bound the state transition size pre-filter by the largest known limit](https://github.com/dashpay/platform/pull/5659) — ✋ changes requested · areas: rust-dapi · Policy: waiting-author
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5853 feat: settle a shielded identity create whose inputs are spent without its identity](https://github.com/dashpay/platform/pull/5853) — ✋ changes requested · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, rust-sdk, kotlin-sdk, fallback · Policy: waiting-author
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5860 feat(platform-wallet): top up an identity the wallet does not manage from Platform addresses](https://github.com/dashpay/platform/pull/5860) — areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk, fallback · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head
- [dashpay/platform#5862 fix(swift-sdk): run identityGet and dpnsGetUsername off the main actor](https://github.com/dashpay/platform/pull/5862) — areas: swift-sdk · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head
- [dashpay/platform#5863 feat(swift-sdk): shieldedShieldFromIdentityToRecipient](https://github.com/dashpay/platform/pull/5863) — areas: rs-platform-wallet-ffi, swift-sdk, kotlin-sdk, fallback · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head

<a id="romchornyi-needs-action"></a>
#### Needs action (4)
- [dashpay/platform#5540 fix(platform-wallet): store externally supplied identity keys without a derivation breadcrumb](https://github.com/dashpay/platform/pull/5540) — 3 unresolved (3 bot) · 20 days stale · 🔴 CI failing · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk, fallback · Policy: waiting-author
  - Top thread: "✅ **Resolved** at \`b25b2543\`; see the replies below." — 20 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5536 fix: check a DashPay invitation claim against Platform before broadcasting it](https://github.com/dashpay/platform/pull/5536) — 3 unresolved (1 CodeRabbit, 2 bot) · 16 days stale · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, rust-sdk, kotlin-sdk, fallback · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Guard the new invitation-status C entry point against Rust panics**" — 16 days old
  - Blocker: coderabbitai left review threads unresolved; resolve them
- [dashpay/platform#5659 fix(dapi): bound the state transition size pre-filter by the largest known limit](https://github.com/dashpay/platform/pull/5659) — ✋ changes requested · areas: rust-dapi · Policy: waiting-author
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5853 feat: settle a shielded identity create whose inputs are spent without its identity](https://github.com/dashpay/platform/pull/5853) — ✋ changes requested · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, rust-sdk, kotlin-sdk, fallback · Policy: waiting-author
  - Blocker: thepastaclaw left review threads unresolved; resolve them

<a id="romchornyi-ready-for-human"></a>
#### Ready for human (1)
- [dashpay/platform#4992 feat(swift-sdk): durable wallet presence marker readable while the device is locked](https://github.com/dashpay/platform/pull/4992) — areas: swift-sdk · Policy: ready-to-merge
  - Blocker: All policy requirements are satisfied

<a id="romchornyi-unresolved-comments"></a>
#### Unresolved Comments (2)
- [dashpay/platform#5540 fix(platform-wallet): store externally supplied identity keys without a derivation breadcrumb](https://github.com/dashpay/platform/pull/5540) — 3 unresolved (3 bot) · 20 days stale · 🔴 CI failing · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk, fallback · Policy: waiting-author
  - Top thread: "✅ **Resolved** at \`b25b2543\`; see the replies below." — 20 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5536 fix: check a DashPay invitation claim against Platform before broadcasting it](https://github.com/dashpay/platform/pull/5536) — 3 unresolved (1 CodeRabbit, 2 bot) · 16 days stale · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, rust-sdk, kotlin-sdk, fallback · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Guard the new invitation-status C entry point against Rust panics**" — 16 days old
  - Blocker: coderabbitai left review threads unresolved; resolve them

<a id="romchornyi-changes-requested"></a>
#### Changes Requested (2)
- [dashpay/platform#5659 fix(dapi): bound the state transition size pre-filter by the largest known limit](https://github.com/dashpay/platform/pull/5659) — ✋ changes requested · areas: rust-dapi · Policy: waiting-author
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5853 feat: settle a shielded identity create whose inputs are spent without its identity](https://github.com/dashpay/platform/pull/5853) — ✋ changes requested · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, rust-sdk, kotlin-sdk, fallback · Policy: waiting-author
  - Blocker: thepastaclaw left review threads unresolved; resolve them

<a id="romchornyi-draft"></a>
#### Draft (4)
- [dashpay/platform#5258 feat(platform-wallet): probe the network about sends with an unknown broadcast outcome](https://github.com/dashpay/platform/pull/5258) — 3 unresolved (3 human) · 26 days stale · ⚠ merge conflict · 📝 draft · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, rust-sdk · Policy: draft
  - Top thread: "\[P2\] Carry the originating wallet generation in probe events" — 26 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5549 fix(platform-wallet-storage): restore DashPay contact address pools on SQLite load](https://github.com/dashpay/platform/pull/5549) — 3 unresolved (3 bot) · 19 days stale · ⚠ merge conflict · 🔴 CI failing · 📝 draft · areas: wallet-storage · Policy: draft
  - Top thread: "✅ **Withdrawn** at \`2bee2705\`; see the replies below." — 19 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5395 feat(platform-wallet): spend only final coins and report funds waiting on the network](https://github.com/dashpay/platform/pull/5395) — 1 unresolved (1 human) · 26 days stale · ⚠ merge conflict · ✋ changes requested · 📝 draft · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk, fallback · Policy: draft
  - Top thread: "this should belong to another layer - likely platform-wallet, or even rust-dashcore's key-wallet. FFI is not meant to st…" — 26 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5587 fix(platform-wallet)!: keep a contact's payment account across relaunches and replace it cleanly on rotation](https://github.com/dashpay/platform/pull/5587) — ⚠ merge conflict · 📝 draft · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage · Policy: draft
  - Blocker: Draft PR does not occupy a review slot

<a id="romchornyi-stale"></a>
#### Stale (2)
- [dashpay/platform#5589 feat(platform-wallet-ffi)!: let hosts reset an outbound contact account's pool and keep its request digest](https://github.com/dashpay/platform/pull/5589) — 🔴 CI failing · 📝 draft · 🐢 targets fix/contact-rebuild-keeps-pool · areas: rs-platform-wallet-ffi, swift-sdk
- [dashpay/platform#5590 feat(kotlin-sdk)!: reset an outbound contact account's pool and keep its request digest on Android](https://github.com/dashpay/platform/pull/5590) — 🔴 CI failing · 📝 draft · 🐢 targets fix/contact-rebuild-ffi-swift · areas: kotlin-sdk, fallback

<a id="romchornyi-clean"></a>
#### Clean (4)
- [dashpay/platform#4992 feat(swift-sdk): durable wallet presence marker readable while the device is locked](https://github.com/dashpay/platform/pull/4992) — areas: swift-sdk · Policy: ready-to-merge
  - Blocker: All policy requirements are satisfied
- [dashpay/platform#5860 feat(platform-wallet): top up an identity the wallet does not manage from Platform addresses](https://github.com/dashpay/platform/pull/5860) — areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk, fallback · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head
- [dashpay/platform#5862 fix(swift-sdk): run identityGet and dpnsGetUsername off the main actor](https://github.com/dashpay/platform/pull/5862) — areas: swift-sdk · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head
- [dashpay/platform#5863 feat(swift-sdk): shieldedShieldFromIdentityToRecipient](https://github.com/dashpay/platform/pull/5863) — areas: rs-platform-wallet-ffi, swift-sdk, kotlin-sdk, fallback · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head

<a id="romchornyi-ready-for-review"></a>
#### Ready for Review (2)
- [dashpay/platform#5479 feat(platform-wallet-storage): persist DashPay backfill coverage and outbound-account markers](https://github.com/dashpay/platform/pull/5479) — by @HashEngineering · areas: rs-platform-wallet, wallet-storage · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 8e1ee2e24549cabf2d2e81a85a02fc613212aa1f
- [dashpay/platform#5793 fix(platform-wallet): make SpvRuntime::header_hash_at public](https://github.com/dashpay/platform/pull/5793) — by @PastaPastaPasta · areas: rs-platform-wallet · Policy: too-many-open-prs
  - Blocker: Human review is still required; automatic reviewer requests are paused until a review slot is available

<a id="bfoss765"></a>
### @bfoss765
<a id="bfoss765-open"></a>
#### Open (2)
- [dashpay/platform#4313 feat(kotlin-sdk): one-time Orchard key shielded-invite API (inviter + claim)](https://github.com/dashpay/platform/pull/4313) — 9 unresolved (9 bot) · 72 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk, fallback · Policy: waiting-author
  - Top thread: "✅ **Resolved** at \`eccc65fa\`; see the replies below." — 72 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#4312 feat(platform-wallet): multi-output shielded transfers + output-aware fee predictor](https://github.com/dashpay/platform/pull/4312) — 1 unresolved (1 bot) · 72 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · areas: dpp, rs-platform-wallet, rs-platform-wallet-ffi, kotlin-sdk, fallback · Policy: waiting-self-review
  - Top thread: "🟡 Suggestion: Keep envelope-aware action counting behind the crate boundary**" — 72 days old
  - Blocker: Author must post /self-reviewed da932f96a8670c641065954de8b58752e99233a3

<a id="bfoss765-needs-action"></a>
#### Needs action (2)
- [dashpay/platform#4313 feat(kotlin-sdk): one-time Orchard key shielded-invite API (inviter + claim)](https://github.com/dashpay/platform/pull/4313) — 9 unresolved (9 bot) · 72 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk, fallback · Policy: waiting-author
  - Top thread: "✅ **Resolved** at \`eccc65fa\`; see the replies below." — 72 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#4312 feat(platform-wallet): multi-output shielded transfers + output-aware fee predictor](https://github.com/dashpay/platform/pull/4312) — 1 unresolved (1 bot) · 72 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · areas: dpp, rs-platform-wallet, rs-platform-wallet-ffi, kotlin-sdk, fallback · Policy: waiting-self-review
  - Top thread: "🟡 Suggestion: Keep envelope-aware action counting behind the crate boundary**" — 72 days old
  - Blocker: Author must post /self-reviewed da932f96a8670c641065954de8b58752e99233a3

<a id="bfoss765-unresolved-comments"></a>
#### Unresolved Comments (2)
- [dashpay/platform#4313 feat(kotlin-sdk): one-time Orchard key shielded-invite API (inviter + claim)](https://github.com/dashpay/platform/pull/4313) — 9 unresolved (9 bot) · 72 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk, fallback · Policy: waiting-author
  - Top thread: "✅ **Resolved** at \`eccc65fa\`; see the replies below." — 72 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#4312 feat(platform-wallet): multi-output shielded transfers + output-aware fee predictor](https://github.com/dashpay/platform/pull/4312) — 1 unresolved (1 bot) · 72 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · areas: dpp, rs-platform-wallet, rs-platform-wallet-ffi, kotlin-sdk, fallback · Policy: waiting-self-review
  - Top thread: "🟡 Suggestion: Keep envelope-aware action counting behind the crate boundary**" — 72 days old
  - Blocker: Author must post /self-reviewed da932f96a8670c641065954de8b58752e99233a3

<a id="shumkov"></a>
### @shumkov
<a id="shumkov-open"></a>
#### Open (16)
- [dashpay/rust-dashcore#634 feat(key-wallet-manager): single-map WalletManager with Arc&lt;RwLock&lt;T&gt;&gt; per wallet](https://github.com/dashpay/rust-dashcore/pull/634) — 20 unresolved (20 CodeRabbit) · 217 days stale · ⚠ merge conflict · ✋ changes requested · areas: dash-spv, key-wallet, key-wallet-manager, fallback · Policy: waiting-author
  - Top thread: "_⚠️ Potential issue_ \| _🟡 Minor_" — 217 days old
  - Blocker: coderabbitai requested changes on this head; dismiss the review or push a fix
  - Blocker: coderabbitai left review threads unresolved; resolve them
- [dashpay/platform#4660 feat(sdk): expose the document erase and history lifecycle on mobile and FFI](https://github.com/dashpay/platform/pull/4660) — 2 unresolved (2 bot) · 49 days stale · ✋ changes requested · 🔴 CI failing · 🐢 targets keep-history-lifecycle · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, rust-sdk-ffi, kotlin-sdk, fallback
  - Top thread: "🟡 Suggestion: Validate history selector discriminant at the C boundary**" — 49 days old
- [dashpay/platform#4657 feat(platform)!: delete and erase lifecycle for keep-history documents](https://github.com/dashpay/platform/pull/4657) — 2 unresolved (2 bot) · 49 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · 🐢 targets keep-history-storage-v2 · areas: rs-drive, rs-drive-abci, dpp, rust-sdk, js-wasm-sdk, fallback
  - Top thread: "🟡 Suggestion: Deduplicate lifecycle containers at the Drive batch boundary**" — 49 days old
- [dashpay/platform#5656 fix(dashmate): build local images for every group node that builds any](https://github.com/dashpay/platform/pull/5656) — 1 unresolved (1 bot) · 12 days stale · ✋ changes requested · 🔴 CI failing · 🐢 targets fix/dashmate/update-path-bugs · areas: dashmate
  - Top thread: "🔴 Blocking: Build the shared base image before dependent service images across group configurations**" — 12 days old
- [dashpay/platform#4283 fix(dashmate): keep the node up when an image pull fails, and stop reporting success when it did not](https://github.com/dashpay/platform/pull/4283) — 🔴 CI failing · areas: dashmate · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 3ceb3315ea9b383ae5816ea4f0e9961202770ed9
  - Blocker: Proceeded without thepastaclaw: no review within the configured window
- [dashpay/platform#4652 feat(drive)!: preserve composite document history across protocol activation](https://github.com/dashpay/platform/pull/4652) — ⚠ merge conflict · 🔴 CI failing · areas: rs-drive, rs-drive-abci, rust-sdk, js-wasm-sdk, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed d5ece21b83e71cd893a27574f699acd8187d35e9
  - Blocker: Proceeded without thepastaclaw: no review within the configured window
- [dashpay/platform#5654 fix(dashmate): restrict a gateway TLS private key an older version left readable](https://github.com/dashpay/platform/pull/5654) — ✋ changes requested · 🔴 CI failing · areas: dashmate · Policy: waiting-author
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
- [dashpay/platform#5655 fix(dashmate): print the default config or group when called without an argument](https://github.com/dashpay/platform/pull/5655) — areas: dashmate · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 105c7318e6e3cb846caeadaa8d87731b3142c2cf
- [dashpay/platform#5763 docs: e2e test architecture and legacy JS retirement plans (draft, not for merge)](https://github.com/dashpay/platform/pull/5763) — 📝 draft · areas: fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5819 feat(dashmate): add the modules that let dashmate update itself](https://github.com/dashpay/platform/pull/5819) — 📝 draft · 🐢 targets fix/dashmate/update-path-bugs · areas: dashmate
- [dashpay/platform#5831 test(drive-abci): guard chain lock, masternode list replay and version table invariants](https://github.com/dashpay/platform/pull/5831) — 🔴 CI failing · areas: rs-drive, rs-drive-abci, fallback · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head
- [dashpay/platform#5841 fix(dashmate): stop masternodes safely around DKGs on Core 24](https://github.com/dashpay/platform/pull/5841) — ✋ changes requested · 🔴 CI failing · areas: dashmate · Policy: waiting-author
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
- [dashpay/platform#5842 chore(dashmate): run Dash Core 24 by default](https://github.com/dashpay/platform/pull/5842) — 🐢 targets shumkov/dashmate-dkg-safe-stop · areas: dashmate
- [dashpay/platform#5843 feat(dashmate): show what a DKG-safe stop waits for and how long](https://github.com/dashpay/platform/pull/5843) — 🐢 targets shumkov/dashmate-core-24 · areas: dashmate
- [dashpay/platform#5854 fix(drive-abci): stop key-adding identity updates from taking the shielded proof verifier](https://github.com/dashpay/platform/pull/5854) — areas: rs-drive-abci, fallback · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head
- [dashpay/platform#5858 fix(drive)!: estimate summable index entries with their sum items, and other PV15 fee estimate follow-ups](https://github.com/dashpay/platform/pull/5858) — areas: rs-drive, fallback · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head

<a id="shumkov-needs-action"></a>
#### Needs action (4)
- [dashpay/rust-dashcore#634 feat(key-wallet-manager): single-map WalletManager with Arc&lt;RwLock&lt;T&gt;&gt; per wallet](https://github.com/dashpay/rust-dashcore/pull/634) — 20 unresolved (20 CodeRabbit) · 217 days stale · ⚠ merge conflict · ✋ changes requested · areas: dash-spv, key-wallet, key-wallet-manager, fallback · Policy: waiting-author
  - Top thread: "_⚠️ Potential issue_ \| _🟡 Minor_" — 217 days old
  - Blocker: coderabbitai requested changes on this head; dismiss the review or push a fix
  - Blocker: coderabbitai left review threads unresolved; resolve them
- [dashpay/platform#4652 feat(drive)!: preserve composite document history across protocol activation](https://github.com/dashpay/platform/pull/4652) — ⚠ merge conflict · 🔴 CI failing · areas: rs-drive, rs-drive-abci, rust-sdk, js-wasm-sdk, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed d5ece21b83e71cd893a27574f699acd8187d35e9
  - Blocker: Proceeded without thepastaclaw: no review within the configured window
- [dashpay/platform#5654 fix(dashmate): restrict a gateway TLS private key an older version left readable](https://github.com/dashpay/platform/pull/5654) — ✋ changes requested · 🔴 CI failing · areas: dashmate · Policy: waiting-author
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
- [dashpay/platform#5841 fix(dashmate): stop masternodes safely around DKGs on Core 24](https://github.com/dashpay/platform/pull/5841) — ✋ changes requested · 🔴 CI failing · areas: dashmate · Policy: waiting-author
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix

<a id="shumkov-unresolved-comments"></a>
#### Unresolved Comments (1)
- [dashpay/rust-dashcore#634 feat(key-wallet-manager): single-map WalletManager with Arc&lt;RwLock&lt;T&gt;&gt; per wallet](https://github.com/dashpay/rust-dashcore/pull/634) — 20 unresolved (20 CodeRabbit) · 217 days stale · ⚠ merge conflict · ✋ changes requested · areas: dash-spv, key-wallet, key-wallet-manager, fallback · Policy: waiting-author
  - Top thread: "_⚠️ Potential issue_ \| _🟡 Minor_" — 217 days old
  - Blocker: coderabbitai requested changes on this head; dismiss the review or push a fix
  - Blocker: coderabbitai left review threads unresolved; resolve them

<a id="shumkov-changes-requested"></a>
#### Changes Requested (2)
- [dashpay/platform#5654 fix(dashmate): restrict a gateway TLS private key an older version left readable](https://github.com/dashpay/platform/pull/5654) — ✋ changes requested · 🔴 CI failing · areas: dashmate · Policy: waiting-author
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
- [dashpay/platform#5841 fix(dashmate): stop masternodes safely around DKGs on Core 24](https://github.com/dashpay/platform/pull/5841) — ✋ changes requested · 🔴 CI failing · areas: dashmate · Policy: waiting-author
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix

<a id="shumkov-ci-failing"></a>
#### CI Failing (3)
- [dashpay/platform#4283 fix(dashmate): keep the node up when an image pull fails, and stop reporting success when it did not](https://github.com/dashpay/platform/pull/4283) — 🔴 CI failing · areas: dashmate · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 3ceb3315ea9b383ae5816ea4f0e9961202770ed9
  - Blocker: Proceeded without thepastaclaw: no review within the configured window
- [dashpay/platform#4652 feat(drive)!: preserve composite document history across protocol activation](https://github.com/dashpay/platform/pull/4652) — ⚠ merge conflict · 🔴 CI failing · areas: rs-drive, rs-drive-abci, rust-sdk, js-wasm-sdk, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed d5ece21b83e71cd893a27574f699acd8187d35e9
  - Blocker: Proceeded without thepastaclaw: no review within the configured window
- [dashpay/platform#5831 test(drive-abci): guard chain lock, masternode list replay and version table invariants](https://github.com/dashpay/platform/pull/5831) — 🔴 CI failing · areas: rs-drive, rs-drive-abci, fallback · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head

<a id="shumkov-draft"></a>
#### Draft (1)
- [dashpay/platform#5763 docs: e2e test architecture and legacy JS retirement plans (draft, not for merge)](https://github.com/dashpay/platform/pull/5763) — 📝 draft · areas: fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot

<a id="shumkov-stale"></a>
#### Stale (6)
- [dashpay/platform#4660 feat(sdk): expose the document erase and history lifecycle on mobile and FFI](https://github.com/dashpay/platform/pull/4660) — 2 unresolved (2 bot) · 49 days stale · ✋ changes requested · 🔴 CI failing · 🐢 targets keep-history-lifecycle · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, rust-sdk-ffi, kotlin-sdk, fallback
  - Top thread: "🟡 Suggestion: Validate history selector discriminant at the C boundary**" — 49 days old
- [dashpay/platform#4657 feat(platform)!: delete and erase lifecycle for keep-history documents](https://github.com/dashpay/platform/pull/4657) — 2 unresolved (2 bot) · 49 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · 🐢 targets keep-history-storage-v2 · areas: rs-drive, rs-drive-abci, dpp, rust-sdk, js-wasm-sdk, fallback
  - Top thread: "🟡 Suggestion: Deduplicate lifecycle containers at the Drive batch boundary**" — 49 days old
- [dashpay/platform#5656 fix(dashmate): build local images for every group node that builds any](https://github.com/dashpay/platform/pull/5656) — 1 unresolved (1 bot) · 12 days stale · ✋ changes requested · 🔴 CI failing · 🐢 targets fix/dashmate/update-path-bugs · areas: dashmate
  - Top thread: "🔴 Blocking: Build the shared base image before dependent service images across group configurations**" — 12 days old
- [dashpay/platform#5819 feat(dashmate): add the modules that let dashmate update itself](https://github.com/dashpay/platform/pull/5819) — 📝 draft · 🐢 targets fix/dashmate/update-path-bugs · areas: dashmate
- [dashpay/platform#5842 chore(dashmate): run Dash Core 24 by default](https://github.com/dashpay/platform/pull/5842) — 🐢 targets shumkov/dashmate-dkg-safe-stop · areas: dashmate
- [dashpay/platform#5843 feat(dashmate): show what a DKG-safe stop waits for and how long](https://github.com/dashpay/platform/pull/5843) — 🐢 targets shumkov/dashmate-core-24 · areas: dashmate

<a id="shumkov-clean"></a>
#### Clean (3)
- [dashpay/platform#5655 fix(dashmate): print the default config or group when called without an argument](https://github.com/dashpay/platform/pull/5655) — areas: dashmate · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 105c7318e6e3cb846caeadaa8d87731b3142c2cf
- [dashpay/platform#5854 fix(drive-abci): stop key-adding identity updates from taking the shielded proof verifier](https://github.com/dashpay/platform/pull/5854) — areas: rs-drive-abci, fallback · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head
- [dashpay/platform#5858 fix(drive)!: estimate summable index entries with their sum items, and other PV15 fee estimate follow-ups](https://github.com/dashpay/platform/pull/5858) — areas: rs-drive, fallback · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head

<a id="shumkov-ready-for-review"></a>
#### Ready for Review (8)
- [dashpay/platform#4298 fix(dashmate): load ZeroSSL config in force mode](https://github.com/dashpay/platform/pull/4298) — by @thepastaclaw · areas: dashmate · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
- [dashpay/platform#5187 ci: tolerate optional S3 cache export outages on v4.3-dev](https://github.com/dashpay/platform/pull/5187) — by @ktechmidas · areas: github · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed fbbf13ee41f8bd8b9f03f1dc5afec6a619967d97
- [dashpay/platform#5848 feat(platform)!: multiple choice decision proposals, voted on as a contest among documents (PV15)](https://github.com/dashpay/platform/pull/5848) — by @QuantumExplorer · areas: rs-drive, rs-drive-abci, dpp, fallback · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head
- [dashpay/platform#5859 fix(platform)!: end a contest without a winner when the award has no room (PV15)](https://github.com/dashpay/platform/pull/5859) — by @QuantumExplorer · areas: rs-drive-abci, fallback · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head
- [dashpay/platform#5860 feat(platform-wallet): top up an identity the wallet does not manage from Platform addresses](https://github.com/dashpay/platform/pull/5860) — by @romchornyi · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk, fallback · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head
- [dashpay/platform#5861 fix(platform)!: keys with a budget or an expiry sign no data contract create or update (PV15)](https://github.com/dashpay/platform/pull/5861) — by @QuantumExplorer · areas: rs-drive-abci, dpp, fallback · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head
- [dashpay/platform#5863 feat(swift-sdk): shieldedShieldFromIdentityToRecipient](https://github.com/dashpay/platform/pull/5863) — by @romchornyi · areas: rs-platform-wallet-ffi, swift-sdk, kotlin-sdk, fallback · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head
- [dashpay/tenderdash#1584 fix(statesync): restore LastAppHash of the snapshot height, not the next one](https://github.com/dashpay/tenderdash/pull/1584) — by @QuantumExplorer · areas: fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 23c1443e4077c7c7dfedf00f9446c762e669660f

<a id="zocolini"></a>
### @ZocoLini
<a id="zocolini-open"></a>
#### Open (7)
- [dashpay/rust-dashcore#1377 refactor(key-wallet): add \`AddressPool::next_unused_index\`](https://github.com/dashpay/rust-dashcore/pull/1377) — 1 unresolved (1 CodeRabbit) · 0 days stale · ✋ changes requested · areas: key-wallet · Policy: waiting-author
  - Top thread: "&lt;picture&gt;&lt;img src="https://storage.googleapis.com/coderabbit_public_assets/review-comment-icons-v1/amber-16.svg" width="…" — 0 days old
  - Blocker: coderabbitai requested changes on this head; dismiss the review or push a fix
  - Blocker: coderabbitai left review threads unresolved; resolve them
- [dashpay/rust-dashcore#1199 feat(key-wallet)!: only final coins fund a build](https://github.com/dashpay/rust-dashcore/pull/1199) — ⚠ merge conflict · 📝 draft · areas: dash-spv, key-wallet, key-wallet-manager · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#1321 fix(key-wallet): reject an account xpub another account already uses](https://github.com/dashpay/rust-dashcore/pull/1321) — ✋ changes requested · areas: key-wallet · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 957be3b94e776dfc99ed7795e709d082a60327b3
- [dashpay/rust-dashcore#1328 refactor(key-wallet)!: validate child indexes at construction with \`ChildIndex\`](https://github.com/dashpay/rust-dashcore/pull/1328) — ⚠ merge conflict · 📝 draft · areas: dash-spv, key-wallet, key-wallet-manager · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#1359 refactor!: detect bloom filter staleness with a hash of what the filter watches](https://github.com/dashpay/rust-dashcore/pull/1359) — ⚠ merge conflict · areas: dash-spv, key-wallet, key-wallet-manager · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 3d8ad5c60d7da6563c17e87c58225cc1bebbbbe0
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/rust-dashcore#1378 fix(key-wallet)!: return errors from managed account constructors](https://github.com/dashpay/rust-dashcore/pull/1378) — areas: key-wallet · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed a165a33c8546a391663e0cc6dd2638100f56192f
- [dashpay/rust-dashcore#1379 fix(dashcore): reject high-S signatures in \`verify_data_signature\`](https://github.com/dashpay/rust-dashcore/pull/1379) — ✋ changes requested · 🔴 CI failing · areas: fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 352f99c5b7628bb518353f95d51dc46eef164fba

<a id="zocolini-needs-action"></a>
#### Needs action (4)
- [dashpay/rust-dashcore#1377 refactor(key-wallet): add \`AddressPool::next_unused_index\`](https://github.com/dashpay/rust-dashcore/pull/1377) — 1 unresolved (1 CodeRabbit) · 0 days stale · ✋ changes requested · areas: key-wallet · Policy: waiting-author
  - Top thread: "&lt;picture&gt;&lt;img src="https://storage.googleapis.com/coderabbit_public_assets/review-comment-icons-v1/amber-16.svg" width="…" — 0 days old
  - Blocker: coderabbitai requested changes on this head; dismiss the review or push a fix
  - Blocker: coderabbitai left review threads unresolved; resolve them
- [dashpay/rust-dashcore#1321 fix(key-wallet): reject an account xpub another account already uses](https://github.com/dashpay/rust-dashcore/pull/1321) — ✋ changes requested · areas: key-wallet · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 957be3b94e776dfc99ed7795e709d082a60327b3
- [dashpay/rust-dashcore#1359 refactor!: detect bloom filter staleness with a hash of what the filter watches](https://github.com/dashpay/rust-dashcore/pull/1359) — ⚠ merge conflict · areas: dash-spv, key-wallet, key-wallet-manager · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 3d8ad5c60d7da6563c17e87c58225cc1bebbbbe0
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/rust-dashcore#1379 fix(dashcore): reject high-S signatures in \`verify_data_signature\`](https://github.com/dashpay/rust-dashcore/pull/1379) — ✋ changes requested · 🔴 CI failing · areas: fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 352f99c5b7628bb518353f95d51dc46eef164fba

<a id="zocolini-unresolved-comments"></a>
#### Unresolved Comments (1)
- [dashpay/rust-dashcore#1377 refactor(key-wallet): add \`AddressPool::next_unused_index\`](https://github.com/dashpay/rust-dashcore/pull/1377) — 1 unresolved (1 CodeRabbit) · 0 days stale · ✋ changes requested · areas: key-wallet · Policy: waiting-author
  - Top thread: "&lt;picture&gt;&lt;img src="https://storage.googleapis.com/coderabbit_public_assets/review-comment-icons-v1/amber-16.svg" width="…" — 0 days old
  - Blocker: coderabbitai requested changes on this head; dismiss the review or push a fix
  - Blocker: coderabbitai left review threads unresolved; resolve them

<a id="zocolini-changes-requested"></a>
#### Changes Requested (2)
- [dashpay/rust-dashcore#1321 fix(key-wallet): reject an account xpub another account already uses](https://github.com/dashpay/rust-dashcore/pull/1321) — ✋ changes requested · areas: key-wallet · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 957be3b94e776dfc99ed7795e709d082a60327b3
- [dashpay/rust-dashcore#1379 fix(dashcore): reject high-S signatures in \`verify_data_signature\`](https://github.com/dashpay/rust-dashcore/pull/1379) — ✋ changes requested · 🔴 CI failing · areas: fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 352f99c5b7628bb518353f95d51dc46eef164fba

<a id="zocolini-draft"></a>
#### Draft (2)
- [dashpay/rust-dashcore#1199 feat(key-wallet)!: only final coins fund a build](https://github.com/dashpay/rust-dashcore/pull/1199) — ⚠ merge conflict · 📝 draft · areas: dash-spv, key-wallet, key-wallet-manager · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#1328 refactor(key-wallet)!: validate child indexes at construction with \`ChildIndex\`](https://github.com/dashpay/rust-dashcore/pull/1328) — ⚠ merge conflict · 📝 draft · areas: dash-spv, key-wallet, key-wallet-manager · Policy: draft
  - Blocker: Draft PR does not occupy a review slot

<a id="zocolini-clean"></a>
#### Clean (2)
- [dashpay/rust-dashcore#1359 refactor!: detect bloom filter staleness with a hash of what the filter watches](https://github.com/dashpay/rust-dashcore/pull/1359) — ⚠ merge conflict · areas: dash-spv, key-wallet, key-wallet-manager · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 3d8ad5c60d7da6563c17e87c58225cc1bebbbbe0
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/rust-dashcore#1378 fix(key-wallet)!: return errors from managed account constructors](https://github.com/dashpay/rust-dashcore/pull/1378) — areas: key-wallet · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed a165a33c8546a391663e0cc6dd2638100f56192f

<a id="zocolini-ready-for-review"></a>
#### Ready for Review (8)
- [dashpay/platform#5277 perf(platform-wallet): don't hold the shielded store lock across sync network I/O](https://github.com/dashpay/platform/pull/5277) — by @PastaPastaPasta · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed de486366461d17ef4845e90de4961499b0b1faeb
- [dashpay/platform#5479 feat(platform-wallet-storage): persist DashPay backfill coverage and outbound-account markers](https://github.com/dashpay/platform/pull/5479) — by @HashEngineering · areas: rs-platform-wallet, wallet-storage · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 8e1ee2e24549cabf2d2e81a85a02fc613212aa1f
- [dashpay/platform#5793 fix(platform-wallet): make SpvRuntime::header_hash_at public](https://github.com/dashpay/platform/pull/5793) — by @PastaPastaPasta · areas: rs-platform-wallet · Policy: too-many-open-prs
  - Blocker: Human review is still required; automatic reviewer requests are paused until a review slot is available
- [dashpay/platform#5860 feat(platform-wallet): top up an identity the wallet does not manage from Platform addresses](https://github.com/dashpay/platform/pull/5860) — by @romchornyi · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk, fallback · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head
- [dashpay/platform#5863 feat(swift-sdk): shieldedShieldFromIdentityToRecipient](https://github.com/dashpay/platform/pull/5863) — by @romchornyi · areas: rs-platform-wallet-ffi, swift-sdk, kotlin-sdk, fallback · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head
- [dashpay/rust-dashcore#1324 fix(dash-spv)!: refuse \`QRInfo\` entries missing from the tip list](https://github.com/dashpay/rust-dashcore/pull/1324) — by @PastaPastaPasta · areas: dash-spv, fallback · Policy: waiting-build
  - Blocker: Waiting for the build to finish
- [dashpay/rust-dashcore#1367 test(key-wallet-manager): cover rescans over chainlocked blocks](https://github.com/dashpay/rust-dashcore/pull/1367) — by @PastaPastaPasta · areas: key-wallet, key-wallet-manager · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
- [dashpay/rust-dashcore#1387 fix(dash-spv): keep re-queued work off kicked peers](https://github.com/dashpay/rust-dashcore/pull/1387) — by @xdustinface · areas: dash-spv · Policy: ready-to-merge
  - Blocker: All policy requirements are satisfied

<a id="lklimek"></a>
### @lklimek
<a id="lklimek-open"></a>
#### Open (11)
- [dashpay/platform#5528 fix(platform-wallet): retain drain operation history through funding conflicts](https://github.com/dashpay/platform/pull/5528) — 7 unresolved (7 bot) · 21 days stale · ✋ changes requested · 📝 draft · 🐢 targets feat/sqlite-wallet-restore · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk, kotlin-sdk
  - Top thread: "🟡 Suggestion: Retain diagnostics for malformed FFI operation snapshots**" — 21 days old
- [dashpay/platform#5323 refactor: reject legacy BLS storage formats](https://github.com/dashpay/platform/pull/5323) — 1 unresolved (1 bot) · 2 days stale · 🔴 CI failing · areas: rs-drive-abci, github, fallback · Policy: waiting-self-review
  - Top thread: "🟡 Suggestion: Cover the required v4.0.0 migration boundary**" — 2 days old
  - Blocker: Author must post /self-reviewed 50c2487f6e349c16904b5bc9abbeeea3be316f10
- [dashpay/dash-evo-tool#1161 fix(ci): isolate release signing and constrain agent write paths](https://github.com/dashpay/dash-evo-tool/pull/1161) — 📝 draft · areas: github, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5150 fix(platform-wallet): correct wallet-level transaction history accounting](https://github.com/dashpay/platform/pull/5150) — ✋ changes requested · areas: rs-platform-wallet, wallet-storage, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 2d025ae336d2467a2da50c99e463026eb0d3910e
- [dashpay/platform#5207 fix(platform-wallet): require only atomic tracked-lock writes for reconciliation](https://github.com/dashpay/platform/pull/5207) — ⏸ deferred · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 6c01f7e279336cc6dd81347d2a069975876e3912
  - Blocker: Proceeded without thepastaclaw: no review within the configured window
- [dashpay/platform#5210 feat(wallet-storage)!: restore complete Core wallet snapshots from SQLite](https://github.com/dashpay/platform/pull/5210) — ⏸ deferred · areas: rs-platform-wallet, wallet-storage
- [dashpay/platform#5220 fix(platform-wallet)!: replay recorded transaction history on load for every persister](https://github.com/dashpay/platform/pull/5220) — ⏸ deferred · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk, fallback
- [dashpay/platform#5519 refactor: remove DPP raw secp256k1 dependency](https://github.com/dashpay/platform/pull/5519) — ⏸ deferred · areas: rs-drive-abci, dpp, rs-platform-wallet, rs-platform-wallet-ffi, rust-sdk, rust-sdk-ffi, js-wasm-sdk, fallback · Policy: waiting-author
  - Blocker: coderabbitai left review threads unresolved; resolve them
- [dashpay/rust-dashcore#1336 fix(wallet)!: authenticate pending spends and InstantLock evidence](https://github.com/dashpay/rust-dashcore/pull/1336) — ⚠ merge conflict · ✋ changes requested · areas: dash-spv, key-wallet, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 8137008f59197b29d8431196ce7546c4945886a6
- [dashpay/tenderdash#1515 fix(service)!: join remaining service workers before shutdown completes](https://github.com/dashpay/tenderdash/pull/1515) — ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · areas: fallback · Policy: waiting-author
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
- [dashpay/tenderdash#1593 feat(consensus)!: specify full consensus parameter commitment and remove app version parameter](https://github.com/dashpay/tenderdash/pull/1593) — ⚠ merge conflict · ✋ changes requested · 📝 draft · areas: fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot

<a id="lklimek-needs-action"></a>
#### Needs action (4)
- [dashpay/platform#5323 refactor: reject legacy BLS storage formats](https://github.com/dashpay/platform/pull/5323) — 1 unresolved (1 bot) · 2 days stale · 🔴 CI failing · areas: rs-drive-abci, github, fallback · Policy: waiting-self-review
  - Top thread: "🟡 Suggestion: Cover the required v4.0.0 migration boundary**" — 2 days old
  - Blocker: Author must post /self-reviewed 50c2487f6e349c16904b5bc9abbeeea3be316f10
- [dashpay/platform#5150 fix(platform-wallet): correct wallet-level transaction history accounting](https://github.com/dashpay/platform/pull/5150) — ✋ changes requested · areas: rs-platform-wallet, wallet-storage, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 2d025ae336d2467a2da50c99e463026eb0d3910e
- [dashpay/rust-dashcore#1336 fix(wallet)!: authenticate pending spends and InstantLock evidence](https://github.com/dashpay/rust-dashcore/pull/1336) — ⚠ merge conflict · ✋ changes requested · areas: dash-spv, key-wallet, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 8137008f59197b29d8431196ce7546c4945886a6
- [dashpay/tenderdash#1515 fix(service)!: join remaining service workers before shutdown completes](https://github.com/dashpay/tenderdash/pull/1515) — ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · areas: fallback · Policy: waiting-author
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix

<a id="lklimek-unresolved-comments"></a>
#### Unresolved Comments (1)
- [dashpay/platform#5323 refactor: reject legacy BLS storage formats](https://github.com/dashpay/platform/pull/5323) — 1 unresolved (1 bot) · 2 days stale · 🔴 CI failing · areas: rs-drive-abci, github, fallback · Policy: waiting-self-review
  - Top thread: "🟡 Suggestion: Cover the required v4.0.0 migration boundary**" — 2 days old
  - Blocker: Author must post /self-reviewed 50c2487f6e349c16904b5bc9abbeeea3be316f10

<a id="lklimek-changes-requested"></a>
#### Changes Requested (3)
- [dashpay/platform#5150 fix(platform-wallet): correct wallet-level transaction history accounting](https://github.com/dashpay/platform/pull/5150) — ✋ changes requested · areas: rs-platform-wallet, wallet-storage, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 2d025ae336d2467a2da50c99e463026eb0d3910e
- [dashpay/rust-dashcore#1336 fix(wallet)!: authenticate pending spends and InstantLock evidence](https://github.com/dashpay/rust-dashcore/pull/1336) — ⚠ merge conflict · ✋ changes requested · areas: dash-spv, key-wallet, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 8137008f59197b29d8431196ce7546c4945886a6
- [dashpay/tenderdash#1515 fix(service)!: join remaining service workers before shutdown completes](https://github.com/dashpay/tenderdash/pull/1515) — ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · areas: fallback · Policy: waiting-author
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix

<a id="lklimek-deferred"></a>
#### Deferred (4)
- [dashpay/platform#5207 fix(platform-wallet): require only atomic tracked-lock writes for reconciliation](https://github.com/dashpay/platform/pull/5207) — ⏸ deferred · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 6c01f7e279336cc6dd81347d2a069975876e3912
  - Blocker: Proceeded without thepastaclaw: no review within the configured window
- [dashpay/platform#5210 feat(wallet-storage)!: restore complete Core wallet snapshots from SQLite](https://github.com/dashpay/platform/pull/5210) — ⏸ deferred · areas: rs-platform-wallet, wallet-storage
- [dashpay/platform#5220 fix(platform-wallet)!: replay recorded transaction history on load for every persister](https://github.com/dashpay/platform/pull/5220) — ⏸ deferred · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk, fallback
- [dashpay/platform#5519 refactor: remove DPP raw secp256k1 dependency](https://github.com/dashpay/platform/pull/5519) — ⏸ deferred · areas: rs-drive-abci, dpp, rs-platform-wallet, rs-platform-wallet-ffi, rust-sdk, rust-sdk-ffi, js-wasm-sdk, fallback · Policy: waiting-author
  - Blocker: coderabbitai left review threads unresolved; resolve them

<a id="lklimek-draft"></a>
#### Draft (2)
- [dashpay/dash-evo-tool#1161 fix(ci): isolate release signing and constrain agent write paths](https://github.com/dashpay/dash-evo-tool/pull/1161) — 📝 draft · areas: github, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/tenderdash#1593 feat(consensus)!: specify full consensus parameter commitment and remove app version parameter](https://github.com/dashpay/tenderdash/pull/1593) — ⚠ merge conflict · ✋ changes requested · 📝 draft · areas: fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot

<a id="lklimek-stale"></a>
#### Stale (1)
- [dashpay/platform#5528 fix(platform-wallet): retain drain operation history through funding conflicts](https://github.com/dashpay/platform/pull/5528) — 7 unresolved (7 bot) · 21 days stale · ✋ changes requested · 📝 draft · 🐢 targets feat/sqlite-wallet-restore · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk, kotlin-sdk
  - Top thread: "🟡 Suggestion: Retain diagnostics for malformed FFI operation snapshots**" — 21 days old

<a id="lklimek-ready-for-review"></a>
#### Ready for Review (6)
- [dashpay/dash-evo-tool#1159 fix(app): keep a previous network's background events and a request's progress where they belong](https://github.com/dashpay/dash-evo-tool/pull/1159) — by @Claudius-Maginificent · areas: fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/dash-evo-tool#1167 perf(wallet): stop re-copying wallet funding state for Max quotes and snapshots](https://github.com/dashpay/dash-evo-tool/pull/1167) — by @Claudius-Maginificent · areas: fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
- [dashpay/platform#5479 feat(platform-wallet-storage): persist DashPay backfill coverage and outbound-account markers](https://github.com/dashpay/platform/pull/5479) — by @HashEngineering · areas: rs-platform-wallet, wallet-storage · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 8e1ee2e24549cabf2d2e81a85a02fc613212aa1f
- [dashpay/platform#5780 fix(platform-wallet-storage): restore UTXOs into the account that derives them](https://github.com/dashpay/platform/pull/5780) — by @PastaPastaPasta · areas: wallet-storage · Policy: too-many-open-prs
  - Blocker: Human review is still required; automatic reviewer requests are paused until a review slot is available
- [dashpay/platform#5794 feat(platform-wallet-storage): read one asset lock's status by outpoint](https://github.com/dashpay/platform/pull/5794) — by @PastaPastaPasta · areas: wallet-storage · Policy: too-many-open-prs
  - Blocker: Human review is still required; automatic reviewer requests are paused until a review slot is available
- [dashpay/tenderdash#1584 fix(statesync): restore LastAppHash of the snapshot height, not the next one](https://github.com/dashpay/tenderdash/pull/1584) — by @QuantumExplorer · areas: fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 23c1443e4077c7c7dfedf00f9446c762e669660f

<a id="hashengineering"></a>
### @HashEngineering
<a id="hashengineering-open"></a>
#### Open (5)
- [dashpay/platform#5796 refactor(platform-wallet): own the TXO reconcile policy in the wallet](https://github.com/dashpay/platform/pull/5796) — 3 unresolved (3 human) · 0 days stale · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk · Policy: waiting-author
  - Top thread: "Non-blocking. The classify pass resumes a later run by store offset, and the comment already says a resumed offset can s…" — 0 days old
  - Blocker: Author response is required after the latest human objection
- [dashpay/platform#5767 fix(kotlin-sdk): port #4638 to Android on the wallet's TXO reconcile policy](https://github.com/dashpay/platform/pull/5767) — 1 unresolved (1 bot) · 0 days stale · 🔴 CI failing · 🐢 targets refactor/platform-wallet-txo-reconcile-policy · areas: kotlin-sdk, fallback
  - Top thread: "🟡 Suggestion: Add a composite index for the paged unspent scan**" — 0 days old
- [dashpay/platform#5479 feat(platform-wallet-storage): persist DashPay backfill coverage and outbound-account markers](https://github.com/dashpay/platform/pull/5479) — areas: rs-platform-wallet, wallet-storage · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 8e1ee2e24549cabf2d2e81a85a02fc613212aa1f
- [dashpay/platform#5797 feat(kotlin-sdk): bind fund_from_asset_lock_external (Core → third-party Platform address)](https://github.com/dashpay/platform/pull/5797) — areas: kotlin-sdk, fallback · Policy: ready-to-merge
  - Blocker: All policy requirements are satisfied
  - Blocker: Proceeded without thepastaclaw: no review within the configured window
- [dashpay/platform#5830 feat(kotlin-sdk): partial-amount platform-address withdrawal](https://github.com/dashpay/platform/pull/5830) — areas: kotlin-sdk, fallback · Policy: ready-to-merge
  - Blocker: All policy requirements are satisfied

<a id="hashengineering-needs-action"></a>
#### Needs action (1)
- [dashpay/platform#5796 refactor(platform-wallet): own the TXO reconcile policy in the wallet](https://github.com/dashpay/platform/pull/5796) — 3 unresolved (3 human) · 0 days stale · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk · Policy: waiting-author
  - Top thread: "Non-blocking. The classify pass resumes a later run by store offset, and the comment already says a resumed offset can s…" — 0 days old
  - Blocker: Author response is required after the latest human objection

<a id="hashengineering-ready-for-human"></a>
#### Ready for human (2)
- [dashpay/platform#5797 feat(kotlin-sdk): bind fund_from_asset_lock_external (Core → third-party Platform address)](https://github.com/dashpay/platform/pull/5797) — areas: kotlin-sdk, fallback · Policy: ready-to-merge
  - Blocker: All policy requirements are satisfied
  - Blocker: Proceeded without thepastaclaw: no review within the configured window
- [dashpay/platform#5830 feat(kotlin-sdk): partial-amount platform-address withdrawal](https://github.com/dashpay/platform/pull/5830) — areas: kotlin-sdk, fallback · Policy: ready-to-merge
  - Blocker: All policy requirements are satisfied

<a id="hashengineering-unresolved-comments"></a>
#### Unresolved Comments (1)
- [dashpay/platform#5796 refactor(platform-wallet): own the TXO reconcile policy in the wallet](https://github.com/dashpay/platform/pull/5796) — 3 unresolved (3 human) · 0 days stale · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk · Policy: waiting-author
  - Top thread: "Non-blocking. The classify pass resumes a later run by store offset, and the comment already says a resumed offset can s…" — 0 days old
  - Blocker: Author response is required after the latest human objection

<a id="hashengineering-stale"></a>
#### Stale (1)
- [dashpay/platform#5767 fix(kotlin-sdk): port #4638 to Android on the wallet's TXO reconcile policy](https://github.com/dashpay/platform/pull/5767) — 1 unresolved (1 bot) · 0 days stale · 🔴 CI failing · 🐢 targets refactor/platform-wallet-txo-reconcile-policy · areas: kotlin-sdk, fallback
  - Top thread: "🟡 Suggestion: Add a composite index for the paged unspent scan**" — 0 days old

<a id="hashengineering-clean"></a>
#### Clean (3)
- [dashpay/platform#5479 feat(platform-wallet-storage): persist DashPay backfill coverage and outbound-account markers](https://github.com/dashpay/platform/pull/5479) — areas: rs-platform-wallet, wallet-storage · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 8e1ee2e24549cabf2d2e81a85a02fc613212aa1f
- [dashpay/platform#5797 feat(kotlin-sdk): bind fund_from_asset_lock_external (Core → third-party Platform address)](https://github.com/dashpay/platform/pull/5797) — areas: kotlin-sdk, fallback · Policy: ready-to-merge
  - Blocker: All policy requirements are satisfied
  - Blocker: Proceeded without thepastaclaw: no review within the configured window
- [dashpay/platform#5830 feat(kotlin-sdk): partial-amount platform-address withdrawal](https://github.com/dashpay/platform/pull/5830) — areas: kotlin-sdk, fallback · Policy: ready-to-merge
  - Blocker: All policy requirements are satisfied

<a id="hashengineering-ready-for-review"></a>
#### Ready for Review (4)
- [dashpay/platform#5277 perf(platform-wallet): don't hold the shielded store lock across sync network I/O](https://github.com/dashpay/platform/pull/5277) — by @PastaPastaPasta · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed de486366461d17ef4845e90de4961499b0b1faeb
- [dashpay/platform#5793 fix(platform-wallet): make SpvRuntime::header_hash_at public](https://github.com/dashpay/platform/pull/5793) — by @PastaPastaPasta · areas: rs-platform-wallet · Policy: too-many-open-prs
  - Blocker: Human review is still required; automatic reviewer requests are paused until a review slot is available
- [dashpay/platform#5860 feat(platform-wallet): top up an identity the wallet does not manage from Platform addresses](https://github.com/dashpay/platform/pull/5860) — by @romchornyi · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk, fallback · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head
- [dashpay/platform#5863 feat(swift-sdk): shieldedShieldFromIdentityToRecipient](https://github.com/dashpay/platform/pull/5863) — by @romchornyi · areas: rs-platform-wallet-ffi, swift-sdk, kotlin-sdk, fallback · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head

<a id="xdustinface"></a>
### @xdustinface
<a id="xdustinface-open"></a>
#### Open (12)
- [dashpay/platform#5791 fix(platform-wallet): refuse a quorum type that does not fit a byte](https://github.com/dashpay/platform/pull/5791) — 2 unresolved (1 CodeRabbit, 1 bot) · 5 days stale · areas: rs-platform-wallet · Policy: waiting-author
  - Top thread: "&lt;picture&gt;&lt;img src="https://storage.googleapis.com/coderabbit_public_assets/review-comment-icons-v1/amber-16.svg" width="…" — 5 days old
  - Blocker: coderabbitai left review threads unresolved; resolve them
- [dashpay/rust-dashcore#541 ci: integrate Codecov test analytics via \`cargo-nextest\`](https://github.com/dashpay/rust-dashcore/pull/541) — ⚠ merge conflict · 📝 draft · areas: github, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#803 feat(dash-spv): block locator + staged fork detection](https://github.com/dashpay/rust-dashcore/pull/803) — ⚠ merge conflict · 📝 draft · areas: dash-spv, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#1345 fix(dash-spv): stop a shared stall from emptying the peer set](https://github.com/dashpay/rust-dashcore/pull/1345) — 📝 draft · 🐢 targets fix/streaming-request-progress · areas: dash-spv
- [dashpay/rust-dashcore#1370 fix(dash-spv): settle filter requests on network arrival](https://github.com/dashpay/rust-dashcore/pull/1370) — 📝 draft · 🐢 targets fix/shared-stall-kicks · areas: dash-spv
- [dashpay/rust-dashcore#1371 fix(dash-spv): size byte caps from the served rate](https://github.com/dashpay/rust-dashcore/pull/1371) — 📝 draft · 🐢 targets fix/cfilters-settle-on-arrival · areas: dash-spv
- [dashpay/rust-dashcore#1372 fix(dash-spv): spread peers across network groups](https://github.com/dashpay/rust-dashcore/pull/1372) — 📝 draft · 🐢 targets fix/rate-sized-cap · areas: dash-spv
- [dashpay/rust-dashcore#1373 fix(dash-spv): rotate the slowest peer of a slow set](https://github.com/dashpay/rust-dashcore/pull/1373) — 📝 draft · 🐢 targets fix/peer-diversity · areas: dash-spv
- [dashpay/rust-dashcore#1374 fix(dash-spv): retry a refused request at another peer](https://github.com/dashpay/rust-dashcore/pull/1374) — 📝 draft · 🐢 targets fix/peer-exploration · areas: dash-spv
- [dashpay/rust-dashcore#1375 fix(dash-spv): rank a stalled peer last when routing](https://github.com/dashpay/rust-dashcore/pull/1375) — 📝 draft · 🐢 targets fix/peer-exploration · areas: dash-spv
- [dashpay/rust-dashcore#1387 fix(dash-spv): keep re-queued work off kicked peers](https://github.com/dashpay/rust-dashcore/pull/1387) — areas: dash-spv · Policy: ready-to-merge
  - Blocker: All policy requirements are satisfied
- [dashpay/rust-dashcore#1388 fix(dash-spv): count partial messages as progress](https://github.com/dashpay/rust-dashcore/pull/1388) — 📝 draft · 🐢 targets fix/requeue-after-kick · areas: dash-spv

<a id="xdustinface-needs-action"></a>
#### Needs action (1)
- [dashpay/platform#5791 fix(platform-wallet): refuse a quorum type that does not fit a byte](https://github.com/dashpay/platform/pull/5791) — 2 unresolved (1 CodeRabbit, 1 bot) · 5 days stale · areas: rs-platform-wallet · Policy: waiting-author
  - Top thread: "&lt;picture&gt;&lt;img src="https://storage.googleapis.com/coderabbit_public_assets/review-comment-icons-v1/amber-16.svg" width="…" — 5 days old
  - Blocker: coderabbitai left review threads unresolved; resolve them

<a id="xdustinface-ready-for-human"></a>
#### Ready for human (1)
- [dashpay/rust-dashcore#1387 fix(dash-spv): keep re-queued work off kicked peers](https://github.com/dashpay/rust-dashcore/pull/1387) — areas: dash-spv · Policy: ready-to-merge
  - Blocker: All policy requirements are satisfied

<a id="xdustinface-unresolved-comments"></a>
#### Unresolved Comments (1)
- [dashpay/platform#5791 fix(platform-wallet): refuse a quorum type that does not fit a byte](https://github.com/dashpay/platform/pull/5791) — 2 unresolved (1 CodeRabbit, 1 bot) · 5 days stale · areas: rs-platform-wallet · Policy: waiting-author
  - Top thread: "&lt;picture&gt;&lt;img src="https://storage.googleapis.com/coderabbit_public_assets/review-comment-icons-v1/amber-16.svg" width="…" — 5 days old
  - Blocker: coderabbitai left review threads unresolved; resolve them

<a id="xdustinface-draft"></a>
#### Draft (2)
- [dashpay/rust-dashcore#541 ci: integrate Codecov test analytics via \`cargo-nextest\`](https://github.com/dashpay/rust-dashcore/pull/541) — ⚠ merge conflict · 📝 draft · areas: github, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#803 feat(dash-spv): block locator + staged fork detection](https://github.com/dashpay/rust-dashcore/pull/803) — ⚠ merge conflict · 📝 draft · areas: dash-spv, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot

<a id="xdustinface-stale"></a>
#### Stale (8)
- [dashpay/rust-dashcore#1345 fix(dash-spv): stop a shared stall from emptying the peer set](https://github.com/dashpay/rust-dashcore/pull/1345) — 📝 draft · 🐢 targets fix/streaming-request-progress · areas: dash-spv
- [dashpay/rust-dashcore#1370 fix(dash-spv): settle filter requests on network arrival](https://github.com/dashpay/rust-dashcore/pull/1370) — 📝 draft · 🐢 targets fix/shared-stall-kicks · areas: dash-spv
- [dashpay/rust-dashcore#1371 fix(dash-spv): size byte caps from the served rate](https://github.com/dashpay/rust-dashcore/pull/1371) — 📝 draft · 🐢 targets fix/cfilters-settle-on-arrival · areas: dash-spv
- [dashpay/rust-dashcore#1372 fix(dash-spv): spread peers across network groups](https://github.com/dashpay/rust-dashcore/pull/1372) — 📝 draft · 🐢 targets fix/rate-sized-cap · areas: dash-spv
- [dashpay/rust-dashcore#1373 fix(dash-spv): rotate the slowest peer of a slow set](https://github.com/dashpay/rust-dashcore/pull/1373) — 📝 draft · 🐢 targets fix/peer-diversity · areas: dash-spv
- [dashpay/rust-dashcore#1374 fix(dash-spv): retry a refused request at another peer](https://github.com/dashpay/rust-dashcore/pull/1374) — 📝 draft · 🐢 targets fix/peer-exploration · areas: dash-spv
- [dashpay/rust-dashcore#1375 fix(dash-spv): rank a stalled peer last when routing](https://github.com/dashpay/rust-dashcore/pull/1375) — 📝 draft · 🐢 targets fix/peer-exploration · areas: dash-spv
- [dashpay/rust-dashcore#1388 fix(dash-spv): count partial messages as progress](https://github.com/dashpay/rust-dashcore/pull/1388) — 📝 draft · 🐢 targets fix/requeue-after-kick · areas: dash-spv

<a id="xdustinface-clean"></a>
#### Clean (1)
- [dashpay/rust-dashcore#1387 fix(dash-spv): keep re-queued work off kicked peers](https://github.com/dashpay/rust-dashcore/pull/1387) — areas: dash-spv · Policy: ready-to-merge
  - Blocker: All policy requirements are satisfied

<a id="xdustinface-ready-for-review"></a>
#### Ready for Review (3)
- [dashpay/rust-dashcore#1324 fix(dash-spv)!: refuse \`QRInfo\` entries missing from the tip list](https://github.com/dashpay/rust-dashcore/pull/1324) — by @PastaPastaPasta · areas: dash-spv, fallback · Policy: waiting-build
  - Blocker: Waiting for the build to finish
- [dashpay/rust-dashcore#1367 test(key-wallet-manager): cover rescans over chainlocked blocks](https://github.com/dashpay/rust-dashcore/pull/1367) — by @PastaPastaPasta · areas: key-wallet, key-wallet-manager · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
- [dashpay/rust-dashcore#1378 fix(key-wallet)!: return errors from managed account constructors](https://github.com/dashpay/rust-dashcore/pull/1378) — by @ZocoLini · areas: key-wallet · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed a165a33c8546a391663e0cc6dd2638100f56192f

<a id="thepastaclaw"></a>
### @thepastaclaw
<a id="thepastaclaw-open"></a>
#### Open (2)
- [dashpay/platform#5581 feat(platform): add proof-backed asset lock outpoint query](https://github.com/dashpay/platform/pull/5581) — 2 unresolved (2 CodeRabbit) · 1 days stale · areas: rs-drive, rs-drive-abci, dpp, rs-platform-wallet, rust-sdk, rust-dapi, fallback · Policy: waiting-author
  - Top thread: "&lt;picture&gt;&lt;img src="https://storage.googleapis.com/coderabbit_public_assets/review-comment-icons-v1/amber-16.svg" width="…" — 1 days old
  - Blocker: coderabbitai left review threads unresolved; resolve them
- [dashpay/platform#4298 fix(dashmate): load ZeroSSL config in force mode](https://github.com/dashpay/platform/pull/4298) — areas: dashmate · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required

<a id="thepastaclaw-needs-action"></a>
#### Needs action (1)
- [dashpay/platform#5581 feat(platform): add proof-backed asset lock outpoint query](https://github.com/dashpay/platform/pull/5581) — 2 unresolved (2 CodeRabbit) · 1 days stale · areas: rs-drive, rs-drive-abci, dpp, rs-platform-wallet, rust-sdk, rust-dapi, fallback · Policy: waiting-author
  - Top thread: "&lt;picture&gt;&lt;img src="https://storage.googleapis.com/coderabbit_public_assets/review-comment-icons-v1/amber-16.svg" width="…" — 1 days old
  - Blocker: coderabbitai left review threads unresolved; resolve them

<a id="thepastaclaw-ready-for-human"></a>
#### Ready for human (1)
- [dashpay/platform#4298 fix(dashmate): load ZeroSSL config in force mode](https://github.com/dashpay/platform/pull/4298) — areas: dashmate · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required

<a id="thepastaclaw-unresolved-comments"></a>
#### Unresolved Comments (1)
- [dashpay/platform#5581 feat(platform): add proof-backed asset lock outpoint query](https://github.com/dashpay/platform/pull/5581) — 2 unresolved (2 CodeRabbit) · 1 days stale · areas: rs-drive, rs-drive-abci, dpp, rs-platform-wallet, rust-sdk, rust-dapi, fallback · Policy: waiting-author
  - Top thread: "&lt;picture&gt;&lt;img src="https://storage.googleapis.com/coderabbit_public_assets/review-comment-icons-v1/amber-16.svg" width="…" — 1 days old
  - Blocker: coderabbitai left review threads unresolved; resolve them

<a id="thepastaclaw-clean"></a>
#### Clean (1)
- [dashpay/platform#4298 fix(dashmate): load ZeroSSL config in force mode](https://github.com/dashpay/platform/pull/4298) — areas: dashmate · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required

<a id="ktechmidas"></a>
### @ktechmidas
<a id="ktechmidas-open"></a>
#### Open (6)
- [dashpay/platform#5188 ci: tolerate optional S3 cache export outages on chore/bump-rust-dashcore-secp-033](https://github.com/dashpay/platform/pull/5188) — 1 unresolved (1 bot) · 43 days stale · 🐢 targets chore/bump-rust-dashcore-secp-033 · areas: github
  - Top thread: "🟡 Suggestion: New regression tests are not executed by any CI workflow on this branch**" — 43 days old
- [dashpay/platform#5186 ci: tolerate optional S3 cache export outages on v5.0-dev](https://github.com/dashpay/platform/pull/5186) — 🔴 CI failing · 🐢 targets v6.0-dev · areas: github · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 843052c2dce26a80065c15b1219bbc8211264161
- [dashpay/platform#5187 ci: tolerate optional S3 cache export outages on v4.3-dev](https://github.com/dashpay/platform/pull/5187) — areas: github · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed fbbf13ee41f8bd8b9f03f1dc5afec6a619967d97
- [dashpay/platform#5198 ci: bound default Binaryen optimizer threads in NPM builds](https://github.com/dashpay/platform/pull/5198) — 🐢 targets v6.0-dev · areas: github · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 23b41b477bc038965b686b4a75412b1c21fc6d98
- [dashpay/platform#5204 fix(release): install verified cargo-binstall binary directly](https://github.com/dashpay/platform/pull/5204) — ⚠ merge conflict · areas: fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 63455a2563fd7271fb11083d3b0a89bdf01f3982
- [dashpay/platform#5205 fix(release): install verified cargo-binstall binary directly](https://github.com/dashpay/platform/pull/5205) — 🐢 targets v6.0-dev · areas: fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 8a8ac2c788b5338416e18045001d9115ba3ac80a

<a id="ktechmidas-needs-action"></a>
#### Needs action (1)
- [dashpay/platform#5204 fix(release): install verified cargo-binstall binary directly](https://github.com/dashpay/platform/pull/5204) — ⚠ merge conflict · areas: fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 63455a2563fd7271fb11083d3b0a89bdf01f3982

<a id="ktechmidas-stale"></a>
#### Stale (4)
- [dashpay/platform#5188 ci: tolerate optional S3 cache export outages on chore/bump-rust-dashcore-secp-033](https://github.com/dashpay/platform/pull/5188) — 1 unresolved (1 bot) · 43 days stale · 🐢 targets chore/bump-rust-dashcore-secp-033 · areas: github
  - Top thread: "🟡 Suggestion: New regression tests are not executed by any CI workflow on this branch**" — 43 days old
- [dashpay/platform#5186 ci: tolerate optional S3 cache export outages on v5.0-dev](https://github.com/dashpay/platform/pull/5186) — 🔴 CI failing · 🐢 targets v6.0-dev · areas: github · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 843052c2dce26a80065c15b1219bbc8211264161
- [dashpay/platform#5198 ci: bound default Binaryen optimizer threads in NPM builds](https://github.com/dashpay/platform/pull/5198) — 🐢 targets v6.0-dev · areas: github · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 23b41b477bc038965b686b4a75412b1c21fc6d98
- [dashpay/platform#5205 fix(release): install verified cargo-binstall binary directly](https://github.com/dashpay/platform/pull/5205) — 🐢 targets v6.0-dev · areas: fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 8a8ac2c788b5338416e18045001d9115ba3ac80a

<a id="ktechmidas-clean"></a>
#### Clean (2)
- [dashpay/platform#5187 ci: tolerate optional S3 cache export outages on v4.3-dev](https://github.com/dashpay/platform/pull/5187) — areas: github · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed fbbf13ee41f8bd8b9f03f1dc5afec6a619967d97
- [dashpay/platform#5204 fix(release): install verified cargo-binstall binary directly](https://github.com/dashpay/platform/pull/5204) — ⚠ merge conflict · areas: fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 63455a2563fd7271fb11083d3b0a89bdf01f3982

<a id="ktechmidas-ready-for-review"></a>
#### Ready for Review (2)
- [dashpay/platform#4298 fix(dashmate): load ZeroSSL config in force mode](https://github.com/dashpay/platform/pull/4298) — by @thepastaclaw · areas: dashmate · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
- [dashpay/platform#5655 fix(dashmate): print the default config or group when called without an argument](https://github.com/dashpay/platform/pull/5655) — by @shumkov · areas: dashmate · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 105c7318e6e3cb846caeadaa8d87731b3142c2cf

<a id="llbartekll"></a>
### @llbartekll
<a id="llbartekll-ready-for-review"></a>
#### Ready for Review (7)
- [dashpay/platform#4992 feat(swift-sdk): durable wallet presence marker readable while the device is locked](https://github.com/dashpay/platform/pull/4992) — by @romchornyi · areas: swift-sdk · Policy: ready-to-merge
  - Blocker: All policy requirements are satisfied
- [dashpay/platform#5277 perf(platform-wallet): don't hold the shielded store lock across sync network I/O](https://github.com/dashpay/platform/pull/5277) — by @PastaPastaPasta · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed de486366461d17ef4845e90de4961499b0b1faeb
- [dashpay/platform#5479 feat(platform-wallet-storage): persist DashPay backfill coverage and outbound-account markers](https://github.com/dashpay/platform/pull/5479) — by @HashEngineering · areas: rs-platform-wallet, wallet-storage · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 8e1ee2e24549cabf2d2e81a85a02fc613212aa1f
- [dashpay/platform#5793 fix(platform-wallet): make SpvRuntime::header_hash_at public](https://github.com/dashpay/platform/pull/5793) — by @PastaPastaPasta · areas: rs-platform-wallet · Policy: too-many-open-prs
  - Blocker: Human review is still required; automatic reviewer requests are paused until a review slot is available
- [dashpay/platform#5860 feat(platform-wallet): top up an identity the wallet does not manage from Platform addresses](https://github.com/dashpay/platform/pull/5860) — by @romchornyi · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk, fallback · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head
- [dashpay/platform#5862 fix(swift-sdk): run identityGet and dpnsGetUsername off the main actor](https://github.com/dashpay/platform/pull/5862) — by @romchornyi · areas: swift-sdk · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head
- [dashpay/platform#5863 feat(swift-sdk): shieldedShieldFromIdentityToRecipient](https://github.com/dashpay/platform/pull/5863) — by @romchornyi · areas: rs-platform-wallet-ffi, swift-sdk, kotlin-sdk, fallback · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head

<a id="dcg-claude"></a>
### @DCG-Claude
<a id="dcg-claude-open"></a>
#### Open (3)
- [dashpay/platform#4794 feat: pin contract configuration vectors and mirror them in wasm-dpp2 and the sdk ffi](https://github.com/dashpay/platform/pull/4794) — 🔴 CI failing · areas: dpp, rust-sdk-ffi, fallback · Policy: waiting-build
  - Blocker: The build must pass before a human is asked
- [dashpay/platform#4934 docs: correct the smart-contract target, version counts, transition list and mobile sdk availability](https://github.com/dashpay/platform/pull/4934) — 🐢 targets v6.0-dev · areas: swift-sdk, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
- [dashpay/platform#4938 docs(platform): distinguish current-state proofs, stored receipts and execution result text](https://github.com/dashpay/platform/pull/4938) — 🐢 targets v6.0-dev · areas: dpp, rust-sdk, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required

<a id="dcg-claude-ready-for-human"></a>
#### Ready for human (2)
- [dashpay/platform#4934 docs: correct the smart-contract target, version counts, transition list and mobile sdk availability](https://github.com/dashpay/platform/pull/4934) — 🐢 targets v6.0-dev · areas: swift-sdk, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
- [dashpay/platform#4938 docs(platform): distinguish current-state proofs, stored receipts and execution result text](https://github.com/dashpay/platform/pull/4938) — 🐢 targets v6.0-dev · areas: dpp, rust-sdk, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required

<a id="dcg-claude-ci-failing"></a>
#### CI Failing (1)
- [dashpay/platform#4794 feat: pin contract configuration vectors and mirror them in wasm-dpp2 and the sdk ffi](https://github.com/dashpay/platform/pull/4794) — 🔴 CI failing · areas: dpp, rust-sdk-ffi, fallback · Policy: waiting-build
  - Blocker: The build must pass before a human is asked

<a id="dcg-claude-stale"></a>
#### Stale (2)
- [dashpay/platform#4934 docs: correct the smart-contract target, version counts, transition list and mobile sdk availability](https://github.com/dashpay/platform/pull/4934) — 🐢 targets v6.0-dev · areas: swift-sdk, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
- [dashpay/platform#4938 docs(platform): distinguish current-state proofs, stored receipts and execution result text](https://github.com/dashpay/platform/pull/4938) — 🐢 targets v6.0-dev · areas: dpp, rust-sdk, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required

## Methodology
Generated nightly by [pr-hygiene](https://github.com/dashpay/stale_prs_are_bad). A thread counts as "unresolved" when it is open, not outdated, has a comment from someone other than the PR author, and the most recent comment is from a reviewer. **Dirty** = at least one such thread. **Unresolved Comments** = at least one such thread. **Changes Requested** = no unresolved threads but a reviewer's most recent review is CHANGES_REQUESTED (still blocking until someone re-approves or dismisses). **Deferred** = carries a configured deferred label (e.g. `postponed`) — visible but not counted toward unresolved-comment counts. **Stale** = targets a non-default branch OR hasn't been touched in the configured threshold (default 120 days, but clean PRs are never reclassified as stale). **Draft** = the PR is still marked draft on GitHub. **CI failing** = no unresolved comments, no changes-requested, but the latest commit's status check is failing. **Clean** = open, not draft, not deferred, not stale, no unresolved comments, no changes-requested, CI green. **Needs action** further requires changes-requested, merge conflict, or that the reviewer commented more recently than the author last pushed. **Ready for human** counts a person's own PRs that the shared review engine marks `ready-for-human` or `ready-to-merge`; each PR bullet shows the engine's state as `Policy:` with its blockers. **Ready for Review** counts clean PRs (authored by someone else) where this person owes a review: the union of the shared policy's routing (owners and reviewers of every area the changed files fall into, or the repository fallback for files no area claims) and GitHub's explicit review requests. The author is never routed to their own PR, and anyone who has already submitted any review is excluded — their job is done. `⚠ ownership unresolved` marks areas whose roster the policy still lists as open. Configurable via [`https://github.com/dashpay/stale_prs_are_bad/blob/master/.pr-hygiene.yml`](https://github.com/dashpay/stale_prs_are_bad/blob/master/.pr-hygiene.yml)—edit defaults there; ownership lives in `policies/`.
