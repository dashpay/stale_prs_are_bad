---
---
# PR Hygiene Report
*Last updated: 2026-10-28 12:15 UTC · commit 0009053*

## Summary
- Open PRs: **191** (36 clean · 17 CI failing · 9 changes requested · 35 unresolved comments · 7 deferred · 42 draft · 45 stale)
- PRs needing author action: **52**
- Total unresolved comments: **190**
- dashpay/platform: **120** open (17 clean · 13 CI failing · 2 changes requested · 23 unresolved comments · 7 deferred · 26 draft · 32 stale) · engine: 27 draft · 20 ready-for-human · 3 ready-to-merge · 1 too-many-open-prs · 17 waiting-author · 6 waiting-bots · 8 waiting-build · 17 waiting-self-review · 21 no verdict
- dashpay/rust-dashcore: **31** open (4 clean · 2 CI failing · 5 changes requested · 6 unresolved comments · 0 deferred · 10 draft · 4 stale) · engine: 12 draft · 4 ready-for-human · 4 waiting-author · 1 waiting-bots · 8 waiting-self-review · 2 no verdict
- dashpay/tenderdash: **7** open (1 clean · 0 CI failing · 0 changes requested · 3 unresolved comments · 0 deferred · 3 draft · 0 stale) · engine: 3 draft · 3 waiting-author · 1 waiting-self-review
- dashpay/grovedb: **3** open (1 clean · 0 CI failing · 0 changes requested · 1 unresolved comments · 0 deferred · 1 draft · 0 stale) · engine: 1 draft · 1 waiting-author · 1 waiting-self-review
- dashpay/dash-evo-tool: **30** open (13 clean · 2 CI failing · 2 changes requested · 2 unresolved comments · 0 deferred · 2 draft · 9 stale) · engine: 2 draft · 1 ready-for-human · 1 ready-to-merge · 3 waiting-author · 14 waiting-bots · 9 no verdict

## Scoreboard
_Sort: unresolved-comments desc → needs-action desc → ready-for-review desc. Click any number to jump to the specific PRs it covers._

| Author | Open | Clean | CI failing | Unresolved Comments | Changes Requested | Deferred | Draft | Stale | Needs action | Ready for human | Total Unresolved Comments | Ready for Review | Δ |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| [@QuantumExplorer](#quantumexplorer) | [23](#quantumexplorer-open) | [2](#quantumexplorer-clean) | [1](#quantumexplorer-ci-failing) | [8](#quantumexplorer-unresolved-comments) | — | — | [6](#quantumexplorer-draft) | [6](#quantumexplorer-stale) | [10](#quantumexplorer-needs-action) | — | [42](#quantumexplorer-unresolved-comments) | [14](#quantumexplorer-ready-for-review) | ↑ 6 |
| [@Claudius-Maginificent](#claudius-maginificent) | [42](#claudius-maginificent-open) | [13](#claudius-maginificent-clean) | [4](#claudius-maginificent-ci-failing) | [6](#claudius-maginificent-unresolved-comments) | [2](#claudius-maginificent-changes-requested) | [3](#claudius-maginificent-deferred) | [4](#claudius-maginificent-draft) | [10](#claudius-maginificent-stale) | [8](#claudius-maginificent-needs-action) | [5](#claudius-maginificent-ready-for-human) | [26](#claudius-maginificent-unresolved-comments) | — | ↑ 7 |
| [@shumkov](#shumkov) | [20](#shumkov-open) | [9](#shumkov-clean) | [1](#shumkov-ci-failing) | [5](#shumkov-unresolved-comments) | [2](#shumkov-changes-requested) | — | — | [3](#shumkov-stale) | [7](#shumkov-needs-action) | [2](#shumkov-ready-for-human) | [29](#shumkov-unresolved-comments) | [5](#shumkov-ready-for-review) | ↑ 6 |
| [@PastaPastaPasta](#pastapastapasta) | [23](#pastapastapasta-open) | [2](#pastapastapasta-clean) | [3](#pastapastapasta-ci-failing) | [4](#pastapastapasta-unresolved-comments) | [1](#pastapastapasta-changes-requested) | — | [10](#pastapastapasta-draft) | [3](#pastapastapasta-stale) | [8](#pastapastapasta-needs-action) | [2](#pastapastapasta-ready-for-human) | [31](#pastapastapasta-unresolved-comments) | — | ↑ 7 |
| [@romchornyi](#romchornyi) | [15](#romchornyi-open) | [2](#romchornyi-clean) | — | [3](#romchornyi-unresolved-comments) | [2](#romchornyi-changes-requested) | — | [6](#romchornyi-draft) | [2](#romchornyi-stale) | [5](#romchornyi-needs-action) | [4](#romchornyi-ready-for-human) | [21](#romchornyi-unresolved-comments) | [2](#romchornyi-ready-for-review) | ↑ 4 |
| [@lklimek](#lklimek) | [14](#lklimek-open) | [1](#lklimek-clean) | — | [3](#lklimek-unresolved-comments) | — | [4](#lklimek-deferred) | [5](#lklimek-draft) | [1](#lklimek-stale) | [3](#lklimek-needs-action) | — | [12](#lklimek-unresolved-comments) | [16](#lklimek-ready-for-review) | — |
| [@bfoss765](#bfoss765) | [2](#bfoss765-open) | — | — | [2](#bfoss765-unresolved-comments) | — | — | — | — | [2](#bfoss765-needs-action) | — | [3](#bfoss765-unresolved-comments) | — | ↑ 2 |
| [@HashEngineering](#hashengineering) | [3](#hashengineering-open) | [1](#hashengineering-clean) | [1](#hashengineering-ci-failing) | [1](#hashengineering-unresolved-comments) | — | — | — | — | [2](#hashengineering-needs-action) | [2](#hashengineering-ready-for-human) | [15](#hashengineering-unresolved-comments) | [2](#hashengineering-ready-for-review) | ↑ 2 |
| [@xdustinface](#xdustinface) | [9](#xdustinface-open) | [1](#xdustinface-clean) | — | [1](#xdustinface-unresolved-comments) | [1](#xdustinface-changes-requested) | — | [4](#xdustinface-draft) | [2](#xdustinface-stale) | [2](#xdustinface-needs-action) | — | [1](#xdustinface-unresolved-comments) | [2](#xdustinface-ready-for-review) | ↑ 1 |
| [@infraclaw-dash](#infraclaw-dash) | [3](#infraclaw-dash-open) | [1](#infraclaw-dash-clean) | — | [1](#infraclaw-dash-unresolved-comments) | — | — | — | [1](#infraclaw-dash-stale) | [2](#infraclaw-dash-needs-action) | [1](#infraclaw-dash-ready-for-human) | [5](#infraclaw-dash-unresolved-comments) | — | ↑ 2 |
| [@llbartekll](#llbartekll) | [2](#llbartekll-open) | — | — | [1](#llbartekll-unresolved-comments) | — | — | [1](#llbartekll-draft) | — | [1](#llbartekll-needs-action) | — | [4](#llbartekll-unresolved-comments) | [3](#llbartekll-ready-for-review) | ↓ 1 |
| [@ktechmidas](#ktechmidas) | [6](#ktechmidas-open) | [2](#ktechmidas-clean) | — | — | — | — | — | [4](#ktechmidas-stale) | [1](#ktechmidas-needs-action) | — | [1](#ktechmidas-unresolved-comments) | [5](#ktechmidas-ready-for-review) | ↑ 1 |
| [@ZocoLini](#zocolini) | [7](#zocolini-open) | — | — | — | [1](#zocolini-changes-requested) | — | [6](#zocolini-draft) | — | [1](#zocolini-needs-action) | — | — | [5](#zocolini-ready-for-review) | ↓ 3 |
| [@DCG-Claude](#dcg-claude) | [19](#dcg-claude-open) | [1](#dcg-claude-clean) | [5](#dcg-claude-ci-failing) | — | — | — | — | [13](#dcg-claude-stale) | — | [11](#dcg-claude-ready-for-human) | — | — | — |
| [@kwvg](#kwvg) | [1](#kwvg-open) | [1](#kwvg-clean) | — | — | — | — | — | — | — | [1](#kwvg-ready-for-human) | — | — | — |
| [@thepastaclaw](#thepastaclaw) | [2](#thepastaclaw-open) | — | [2](#thepastaclaw-ci-failing) | — | — | — | — | — | — | [1](#thepastaclaw-ready-for-human) | — | — | — |

## Per-author detail

<a id="quantumexplorer"></a>
### @QuantumExplorer
<a id="quantumexplorer-open"></a>
#### Open (23)
- [dashpay/rust-dashcore#1078 feat(key-wallet)!: lock masternode collateral out of coin selection](https://github.com/dashpay/rust-dashcore/pull/1078) — 8 unresolved (8 human) · 28 days stale · ⚠ merge conflict · ✋ changes requested · areas: key-wallet, key-wallet-manager · Policy: waiting-self-review
  - Top thread: "This sets the state by hand, and together with the serde round-trip it is the only coverage of the refresh inside \`updat…" — 28 days old
  - Blocker: Author must post /self-reviewed 44a9a010500fbd44aba09111b4c53a28b88e8073
- [dashpay/platform#4272 feat(dpp)!: dashpay payment detection keys and stealth derivation](https://github.com/dashpay/platform/pull/4272) — 9 unresolved (9 bot) · 86 days stale · ⚠ merge conflict · 🔴 CI failing · 📝 draft · areas: rs-drive, rs-drive-abci, dpp, rs-platform-wallet, swift-sdk, system-contracts, kotlin-sdk, fallback · Policy: draft
  - Top thread: "🔴 Blocking: Require compressed 33-byte encodings for payment keys**" — 86 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#860 feat(dash-spv): expose per-peer connection stats (ping RTT, bytes received)](https://github.com/dashpay/rust-dashcore/pull/860) — 3 unresolved (3 CodeRabbit) · 110 days stale · ⚠ merge conflict · ✋ changes requested · areas: dash-spv, fallback · Policy: waiting-author
  - Top thread: "_📐 Maintainability &amp; Code Quality_ \| _🟡 Minor_ \| _⚡ Quick win_" — 110 days old
  - Blocker: coderabbitai left review threads unresolved; resolve them
- [dashpay/platform#5238 fix(platform)!: refund sponsor-paid document storage to the gas sponsor](https://github.com/dashpay/platform/pull/5238) — 3 unresolved (3 bot) · 27 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · 📝 draft · areas: rs-drive, rs-drive-abci, fallback · Policy: draft
  - Top thread: "🟡 Suggestion: Make the transfer regression change the primary element's size**" — 27 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#4899 feat(platform)!: yes/no masternode vote poll kind with supermajority and minimum voting power](https://github.com/dashpay/platform/pull/4899) — 4 unresolved (4 bot) · 7 days stale · 🔴 CI failing · areas: rs-drive, rs-drive-abci, dpp, rust-sdk, js-wasm-sdk, fallback · Policy: waiting-self-review
  - Top thread: "🟡 Suggestion: Do not treat a page consumed entirely by empty end dates as exhaustion**" — 7 days old
  - Blocker: Author must post /self-reviewed b634524462f2e931db5236fee119d583f9212f31
- [dashpay/rust-dashcore#1077 feat(dash-spv)!: check masternode list diffs against the block coinbase](https://github.com/dashpay/rust-dashcore/pull/1077) — 2 unresolved (2 CodeRabbit) · 30 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · areas: dash-spv, fallback · Policy: waiting-author
  - Top thread: "_🩺 Stability &amp; Availability_ \| _🟠 Major_ \| _⚡ Quick win_" — 30 days old
  - Blocker: coderabbitai requested changes on this head; dismiss the review or push a fix
  - Blocker: coderabbitai left review threads unresolved; resolve them
- [dashpay/platform#5360 perf(platform)!: delete a consumed document from what its lookup read (PV15)](https://github.com/dashpay/platform/pull/5360) — 2 unresolved (2 bot) · 19 days stale · ⚠ merge conflict · areas: rs-drive, rs-drive-abci, dpp, rust-sdk, js-wasm-sdk, system-contracts, fallback · Policy: waiting-self-review
  - Top thread: "🟡 Suggestion: Add negative-path tests for the new read-document deletion guards**" — 19 days old
  - Blocker: Author must post /self-reviewed f338eb17d9c54ba6976686769b2de9c3d1d61104
- [dashpay/platform#5604 feat(platform)!: generatedFrom sys.tokens.decimals generates a token's decimals (PV15)](https://github.com/dashpay/platform/pull/5604) — 4 unresolved (4 bot) · 1 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · areas: rs-drive, rs-drive-abci, dpp, fallback · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Classify only known integer-generating token functions**" — 1 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#3417 feat(swift-sdk): use SPV-synced quorums for Platform proof verification](https://github.com/dashpay/platform/pull/3417) — 2 unresolved (2 bot) · 6 days stale · ✋ changes requested · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, rust-sdk, rust-sdk-ffi, fallback · Policy: waiting-author
  - Top thread: "🔴 Blocking: Require network-difficulty validation before trusting SPV headers**" — 6 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5529 feat(platform)!: token marketplace listings (PV15)](https://github.com/dashpay/platform/pull/5529) — 2 unresolved (2 bot) · 3 days stale · ✋ changes requested · 🐢 targets claude/token-market-escrow-contract · areas: rs-drive, rs-drive-abci, dpp, rs-platform-wallet, swift-sdk, kotlin-sdk, fallback
  - Top thread: "🔴 Blocking: Version the consensus order-ID derivation through platform-version tables**" — 3 days old
- [dashpay/platform#5632 fix(platform)!: refuse a token config update on a read-only contract (PV15)](https://github.com/dashpay/platform/pull/5632) — 2 unresolved (1 CodeRabbit, 1 bot) · 0 days stale · areas: rs-drive-abci, dpp, fallback · Policy: waiting-author
  - Top thread: "&lt;picture&gt;&lt;img src="https://storage.googleapis.com/coderabbit_public_assets/review-comment-icons-v1/amber-16.svg" width="…" — 0 days old
  - Blocker: coderabbitai left review threads unresolved; resolve them
- [dashpay/platform#5532 feat(platform)!: destroy a frozen identity's escrowed listings with its funds (PV15)](https://github.com/dashpay/platform/pull/5532) — 1 unresolved (1 bot) · 3 days stale · 🐢 targets claude/token-market-history · areas: rs-drive, rs-drive-abci, fallback
  - Top thread: "🟡 Suggestion: Avoid stateful frozen-listing reads during CheckTx cost approximation**" — 3 days old
- [dashpay/grovedb#1001 feat(batch)!: opt-in settlement of storage owner changes](https://github.com/dashpay/grovedb/pull/1001) — ⚠ merge conflict · areas: fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 814dfc765747cec0beb7a39e1cdf04d301ce9ac3
- [dashpay/platform#4519 feat(platform-wallet): rotate a masternode's keys into the wallet — ProUpRegTx orchestration, FFI, Swift](https://github.com/dashpay/platform/pull/4519) — ⚠ merge conflict · 🔴 CI failing · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 92a3ebe17adc12432fb756c63f172c663809007d
  - Blocker: Proceeded without thepastaclaw: no review within the configured window
- [dashpay/platform#4730 feat(platform)!: delta-based data contract update transition for protocol version 15](https://github.com/dashpay/platform/pull/4730) — ⚠ merge conflict · 🔴 CI failing · 📝 draft · areas: rs-drive, rs-drive-abci, dpp, rs-platform-wallet, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#4776 feat(platform-wallet)!: let contract updates clear the description through the FFI](https://github.com/dashpay/platform/pull/4776) — 🔴 CI failing · 🐢 targets feat/delta-contract-update-pv15 · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk, fallback
- [dashpay/platform#4933 feat(platform)!: dashpay contact requests declare their checks](https://github.com/dashpay/platform/pull/4933) — ⚠ merge conflict · 📝 draft · areas: rs-drive, rs-drive-abci, dpp, system-contracts, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#4986 fix(drive-abci)!: pay every reward share of a masternode and credit each identity once in the epoch payout (PV15)](https://github.com/dashpay/platform/pull/4986) — ⚠ merge conflict · 📝 draft · areas: rs-drive, rs-drive-abci, dpp, github, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5113 feat(platform-wallet)!: keep masternode collateral out of coin selection](https://github.com/dashpay/platform/pull/5113) — ⚠ merge conflict · 🔴 CI failing · 📝 draft · 🐢 targets chore/bump-rust-dashcore-secp-033 · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk, fallback
- [dashpay/platform#5526 feat(platform)!: token marketplace escrow trees and token market contract (PV15)](https://github.com/dashpay/platform/pull/5526) — areas: rs-drive, rs-drive-abci, dpp, rust-sdk, rust-sdk-ffi, js-wasm-sdk, system-contracts, github, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed eb5ddbebe2aef913cbda17ab2e9a780b94b9326f
- [dashpay/platform#5530 feat(platform)!: token marketplace offers (PV15)](https://github.com/dashpay/platform/pull/5530) — ✋ changes requested · 🐢 targets claude/token-market-listings · areas: rs-drive, rs-drive-abci, dpp, rs-platform-wallet, fallback
- [dashpay/platform#5531 feat(platform)!: record token marketplace events in token history (PV15)](https://github.com/dashpay/platform/pull/5531) — ⚠ merge conflict · 🐢 targets claude/token-market-offers · areas: rs-drive, rs-drive-abci, dpp, system-contracts, fallback
- [dashpay/tenderdash#1572 feat(consensus)!: hand a silent proposer's idle round to the next proposer](https://github.com/dashpay/tenderdash/pull/1572) — 📝 draft · areas: fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot

<a id="quantumexplorer-needs-action"></a>
#### Needs action (10)
- [dashpay/rust-dashcore#1078 feat(key-wallet)!: lock masternode collateral out of coin selection](https://github.com/dashpay/rust-dashcore/pull/1078) — 8 unresolved (8 human) · 28 days stale · ⚠ merge conflict · ✋ changes requested · areas: key-wallet, key-wallet-manager · Policy: waiting-self-review
  - Top thread: "This sets the state by hand, and together with the serde round-trip it is the only coverage of the refresh inside \`updat…" — 28 days old
  - Blocker: Author must post /self-reviewed 44a9a010500fbd44aba09111b4c53a28b88e8073
- [dashpay/rust-dashcore#860 feat(dash-spv): expose per-peer connection stats (ping RTT, bytes received)](https://github.com/dashpay/rust-dashcore/pull/860) — 3 unresolved (3 CodeRabbit) · 110 days stale · ⚠ merge conflict · ✋ changes requested · areas: dash-spv, fallback · Policy: waiting-author
  - Top thread: "_📐 Maintainability &amp; Code Quality_ \| _🟡 Minor_ \| _⚡ Quick win_" — 110 days old
  - Blocker: coderabbitai left review threads unresolved; resolve them
- [dashpay/platform#4899 feat(platform)!: yes/no masternode vote poll kind with supermajority and minimum voting power](https://github.com/dashpay/platform/pull/4899) — 4 unresolved (4 bot) · 7 days stale · 🔴 CI failing · areas: rs-drive, rs-drive-abci, dpp, rust-sdk, js-wasm-sdk, fallback · Policy: waiting-self-review
  - Top thread: "🟡 Suggestion: Do not treat a page consumed entirely by empty end dates as exhaustion**" — 7 days old
  - Blocker: Author must post /self-reviewed b634524462f2e931db5236fee119d583f9212f31
- [dashpay/rust-dashcore#1077 feat(dash-spv)!: check masternode list diffs against the block coinbase](https://github.com/dashpay/rust-dashcore/pull/1077) — 2 unresolved (2 CodeRabbit) · 30 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · areas: dash-spv, fallback · Policy: waiting-author
  - Top thread: "_🩺 Stability &amp; Availability_ \| _🟠 Major_ \| _⚡ Quick win_" — 30 days old
  - Blocker: coderabbitai requested changes on this head; dismiss the review or push a fix
  - Blocker: coderabbitai left review threads unresolved; resolve them
- [dashpay/platform#5360 perf(platform)!: delete a consumed document from what its lookup read (PV15)](https://github.com/dashpay/platform/pull/5360) — 2 unresolved (2 bot) · 19 days stale · ⚠ merge conflict · areas: rs-drive, rs-drive-abci, dpp, rust-sdk, js-wasm-sdk, system-contracts, fallback · Policy: waiting-self-review
  - Top thread: "🟡 Suggestion: Add negative-path tests for the new read-document deletion guards**" — 19 days old
  - Blocker: Author must post /self-reviewed f338eb17d9c54ba6976686769b2de9c3d1d61104
- [dashpay/platform#5604 feat(platform)!: generatedFrom sys.tokens.decimals generates a token's decimals (PV15)](https://github.com/dashpay/platform/pull/5604) — 4 unresolved (4 bot) · 1 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · areas: rs-drive, rs-drive-abci, dpp, fallback · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Classify only known integer-generating token functions**" — 1 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#3417 feat(swift-sdk): use SPV-synced quorums for Platform proof verification](https://github.com/dashpay/platform/pull/3417) — 2 unresolved (2 bot) · 6 days stale · ✋ changes requested · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, rust-sdk, rust-sdk-ffi, fallback · Policy: waiting-author
  - Top thread: "🔴 Blocking: Require network-difficulty validation before trusting SPV headers**" — 6 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5632 fix(platform)!: refuse a token config update on a read-only contract (PV15)](https://github.com/dashpay/platform/pull/5632) — 2 unresolved (1 CodeRabbit, 1 bot) · 0 days stale · areas: rs-drive-abci, dpp, fallback · Policy: waiting-author
  - Top thread: "&lt;picture&gt;&lt;img src="https://storage.googleapis.com/coderabbit_public_assets/review-comment-icons-v1/amber-16.svg" width="…" — 0 days old
  - Blocker: coderabbitai left review threads unresolved; resolve them
- [dashpay/grovedb#1001 feat(batch)!: opt-in settlement of storage owner changes](https://github.com/dashpay/grovedb/pull/1001) — ⚠ merge conflict · areas: fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 814dfc765747cec0beb7a39e1cdf04d301ce9ac3
- [dashpay/platform#4519 feat(platform-wallet): rotate a masternode's keys into the wallet — ProUpRegTx orchestration, FFI, Swift](https://github.com/dashpay/platform/pull/4519) — ⚠ merge conflict · 🔴 CI failing · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 92a3ebe17adc12432fb756c63f172c663809007d
  - Blocker: Proceeded without thepastaclaw: no review within the configured window

<a id="quantumexplorer-unresolved-comments"></a>
#### Unresolved Comments (8)
- [dashpay/rust-dashcore#1078 feat(key-wallet)!: lock masternode collateral out of coin selection](https://github.com/dashpay/rust-dashcore/pull/1078) — 8 unresolved (8 human) · 28 days stale · ⚠ merge conflict · ✋ changes requested · areas: key-wallet, key-wallet-manager · Policy: waiting-self-review
  - Top thread: "This sets the state by hand, and together with the serde round-trip it is the only coverage of the refresh inside \`updat…" — 28 days old
  - Blocker: Author must post /self-reviewed 44a9a010500fbd44aba09111b4c53a28b88e8073
- [dashpay/rust-dashcore#860 feat(dash-spv): expose per-peer connection stats (ping RTT, bytes received)](https://github.com/dashpay/rust-dashcore/pull/860) — 3 unresolved (3 CodeRabbit) · 110 days stale · ⚠ merge conflict · ✋ changes requested · areas: dash-spv, fallback · Policy: waiting-author
  - Top thread: "_📐 Maintainability &amp; Code Quality_ \| _🟡 Minor_ \| _⚡ Quick win_" — 110 days old
  - Blocker: coderabbitai left review threads unresolved; resolve them
- [dashpay/platform#4899 feat(platform)!: yes/no masternode vote poll kind with supermajority and minimum voting power](https://github.com/dashpay/platform/pull/4899) — 4 unresolved (4 bot) · 7 days stale · 🔴 CI failing · areas: rs-drive, rs-drive-abci, dpp, rust-sdk, js-wasm-sdk, fallback · Policy: waiting-self-review
  - Top thread: "🟡 Suggestion: Do not treat a page consumed entirely by empty end dates as exhaustion**" — 7 days old
  - Blocker: Author must post /self-reviewed b634524462f2e931db5236fee119d583f9212f31
- [dashpay/rust-dashcore#1077 feat(dash-spv)!: check masternode list diffs against the block coinbase](https://github.com/dashpay/rust-dashcore/pull/1077) — 2 unresolved (2 CodeRabbit) · 30 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · areas: dash-spv, fallback · Policy: waiting-author
  - Top thread: "_🩺 Stability &amp; Availability_ \| _🟠 Major_ \| _⚡ Quick win_" — 30 days old
  - Blocker: coderabbitai requested changes on this head; dismiss the review or push a fix
  - Blocker: coderabbitai left review threads unresolved; resolve them
- [dashpay/platform#5360 perf(platform)!: delete a consumed document from what its lookup read (PV15)](https://github.com/dashpay/platform/pull/5360) — 2 unresolved (2 bot) · 19 days stale · ⚠ merge conflict · areas: rs-drive, rs-drive-abci, dpp, rust-sdk, js-wasm-sdk, system-contracts, fallback · Policy: waiting-self-review
  - Top thread: "🟡 Suggestion: Add negative-path tests for the new read-document deletion guards**" — 19 days old
  - Blocker: Author must post /self-reviewed f338eb17d9c54ba6976686769b2de9c3d1d61104
- [dashpay/platform#5604 feat(platform)!: generatedFrom sys.tokens.decimals generates a token's decimals (PV15)](https://github.com/dashpay/platform/pull/5604) — 4 unresolved (4 bot) · 1 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · areas: rs-drive, rs-drive-abci, dpp, fallback · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Classify only known integer-generating token functions**" — 1 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#3417 feat(swift-sdk): use SPV-synced quorums for Platform proof verification](https://github.com/dashpay/platform/pull/3417) — 2 unresolved (2 bot) · 6 days stale · ✋ changes requested · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, rust-sdk, rust-sdk-ffi, fallback · Policy: waiting-author
  - Top thread: "🔴 Blocking: Require network-difficulty validation before trusting SPV headers**" — 6 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5632 fix(platform)!: refuse a token config update on a read-only contract (PV15)](https://github.com/dashpay/platform/pull/5632) — 2 unresolved (1 CodeRabbit, 1 bot) · 0 days stale · areas: rs-drive-abci, dpp, fallback · Policy: waiting-author
  - Top thread: "&lt;picture&gt;&lt;img src="https://storage.googleapis.com/coderabbit_public_assets/review-comment-icons-v1/amber-16.svg" width="…" — 0 days old
  - Blocker: coderabbitai left review threads unresolved; resolve them

<a id="quantumexplorer-ci-failing"></a>
#### CI Failing (1)
- [dashpay/platform#4519 feat(platform-wallet): rotate a masternode's keys into the wallet — ProUpRegTx orchestration, FFI, Swift](https://github.com/dashpay/platform/pull/4519) — ⚠ merge conflict · 🔴 CI failing · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 92a3ebe17adc12432fb756c63f172c663809007d
  - Blocker: Proceeded without thepastaclaw: no review within the configured window

<a id="quantumexplorer-draft"></a>
#### Draft (6)
- [dashpay/platform#4272 feat(dpp)!: dashpay payment detection keys and stealth derivation](https://github.com/dashpay/platform/pull/4272) — 9 unresolved (9 bot) · 86 days stale · ⚠ merge conflict · 🔴 CI failing · 📝 draft · areas: rs-drive, rs-drive-abci, dpp, rs-platform-wallet, swift-sdk, system-contracts, kotlin-sdk, fallback · Policy: draft
  - Top thread: "🔴 Blocking: Require compressed 33-byte encodings for payment keys**" — 86 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5238 fix(platform)!: refund sponsor-paid document storage to the gas sponsor](https://github.com/dashpay/platform/pull/5238) — 3 unresolved (3 bot) · 27 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · 📝 draft · areas: rs-drive, rs-drive-abci, fallback · Policy: draft
  - Top thread: "🟡 Suggestion: Make the transfer regression change the primary element's size**" — 27 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#4730 feat(platform)!: delta-based data contract update transition for protocol version 15](https://github.com/dashpay/platform/pull/4730) — ⚠ merge conflict · 🔴 CI failing · 📝 draft · areas: rs-drive, rs-drive-abci, dpp, rs-platform-wallet, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#4933 feat(platform)!: dashpay contact requests declare their checks](https://github.com/dashpay/platform/pull/4933) — ⚠ merge conflict · 📝 draft · areas: rs-drive, rs-drive-abci, dpp, system-contracts, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#4986 fix(drive-abci)!: pay every reward share of a masternode and credit each identity once in the epoch payout (PV15)](https://github.com/dashpay/platform/pull/4986) — ⚠ merge conflict · 📝 draft · areas: rs-drive, rs-drive-abci, dpp, github, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/tenderdash#1572 feat(consensus)!: hand a silent proposer's idle round to the next proposer](https://github.com/dashpay/tenderdash/pull/1572) — 📝 draft · areas: fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot

<a id="quantumexplorer-stale"></a>
#### Stale (6)
- [dashpay/platform#5529 feat(platform)!: token marketplace listings (PV15)](https://github.com/dashpay/platform/pull/5529) — 2 unresolved (2 bot) · 3 days stale · ✋ changes requested · 🐢 targets claude/token-market-escrow-contract · areas: rs-drive, rs-drive-abci, dpp, rs-platform-wallet, swift-sdk, kotlin-sdk, fallback
  - Top thread: "🔴 Blocking: Version the consensus order-ID derivation through platform-version tables**" — 3 days old
- [dashpay/platform#5532 feat(platform)!: destroy a frozen identity's escrowed listings with its funds (PV15)](https://github.com/dashpay/platform/pull/5532) — 1 unresolved (1 bot) · 3 days stale · 🐢 targets claude/token-market-history · areas: rs-drive, rs-drive-abci, fallback
  - Top thread: "🟡 Suggestion: Avoid stateful frozen-listing reads during CheckTx cost approximation**" — 3 days old
- [dashpay/platform#4776 feat(platform-wallet)!: let contract updates clear the description through the FFI](https://github.com/dashpay/platform/pull/4776) — 🔴 CI failing · 🐢 targets feat/delta-contract-update-pv15 · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk, fallback
- [dashpay/platform#5113 feat(platform-wallet)!: keep masternode collateral out of coin selection](https://github.com/dashpay/platform/pull/5113) — ⚠ merge conflict · 🔴 CI failing · 📝 draft · 🐢 targets chore/bump-rust-dashcore-secp-033 · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk, fallback
- [dashpay/platform#5530 feat(platform)!: token marketplace offers (PV15)](https://github.com/dashpay/platform/pull/5530) — ✋ changes requested · 🐢 targets claude/token-market-listings · areas: rs-drive, rs-drive-abci, dpp, rs-platform-wallet, fallback
- [dashpay/platform#5531 feat(platform)!: record token marketplace events in token history (PV15)](https://github.com/dashpay/platform/pull/5531) — ⚠ merge conflict · 🐢 targets claude/token-market-offers · areas: rs-drive, rs-drive-abci, dpp, system-contracts, fallback

<a id="quantumexplorer-clean"></a>
#### Clean (2)
- [dashpay/grovedb#1001 feat(batch)!: opt-in settlement of storage owner changes](https://github.com/dashpay/grovedb/pull/1001) — ⚠ merge conflict · areas: fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 814dfc765747cec0beb7a39e1cdf04d301ce9ac3
- [dashpay/platform#5526 feat(platform)!: token marketplace escrow trees and token market contract (PV15)](https://github.com/dashpay/platform/pull/5526) — areas: rs-drive, rs-drive-abci, dpp, rust-sdk, rust-sdk-ffi, js-wasm-sdk, system-contracts, github, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed eb5ddbebe2aef913cbda17ab2e9a780b94b9326f

<a id="quantumexplorer-ready-for-review"></a>
#### Ready for Review (14)
- [dashpay/platform#4706 feat(platform)!: require fee history for storage refunds and credit their recorded owners](https://github.com/dashpay/platform/pull/4706) — by @DCG-Claude · areas: rs-drive, rs-drive-abci, fallback · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head
- [dashpay/platform#5476 feat(platform-wallet)!: report a freshly built asset lock's outpoint to the caller](https://github.com/dashpay/platform/pull/5476) — by @romchornyi · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, fallback · Policy: ready-to-merge
  - Blocker: All policy requirements are satisfied
- [dashpay/platform#5612 fix(sdk): let wasm-sdk build without its default features](https://github.com/dashpay/platform/pull/5612) — by @PastaPastaPasta · areas: js-wasm-sdk, fallback · Policy: too-many-open-prs
  - Blocker: Human review is still required; automatic reviewer requests are paused until a review slot is available
- [dashpay/platform#5615 fix(sdk): refuse a non-unique identity V1 request before protocol 14 is proven](https://github.com/dashpay/platform/pull/5615) — by @shumkov · areas: rust-sdk, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 587ba0843d4104b65772b9341461eafd8a057f0f
- [dashpay/platform#5628 fix(sdk): read the proved vote of a masternode vote that already exists](https://github.com/dashpay/platform/pull/5628) — by @shumkov · areas: rs-drive-abci, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 2cfa0d137a372465d593ff022f0052d7890a0f74
- [dashpay/platform#5630 fix(drive-abci): make zero quorum windows and active-signer counts unrepresentable](https://github.com/dashpay/platform/pull/5630) — by @shumkov · areas: rs-drive-abci · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 61041b0828035c2d75df0bcf1a9ae73a64668058
- [dashpay/platform#5634 feat(drive-abci)!: emit version 2 asset unlocks once Core V24 is active (PV15)](https://github.com/dashpay/platform/pull/5634) — by @shumkov · areas: rs-drive-abci, fallback · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head
- [dashpay/platform#5635 fix(sdk)!: refuse document requests no node can answer instead of banning nodes](https://github.com/dashpay/platform/pull/5635) — by @shumkov · areas: rs-drive, rs-drive-abci, rust-sdk, rust-sdk-ffi, js-wasm-sdk, github, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 771e06ec7fc942639e36774aa152c783c8cf0f41
- [dashpay/platform#5636 feat(sdk)!: decide from the version tables which networks serve a document query](https://github.com/dashpay/platform/pull/5636) — by @shumkov · areas: rs-drive, rs-drive-abci, rust-sdk, rust-sdk-ffi, js-wasm-sdk, github, fallback · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head
- [dashpay/platform#5637 chore(platform)!: update rust-dashcore to dev 7329126e](https://github.com/dashpay/platform/pull/5637) — by @shumkov · areas: dpp, rs-platform-wallet, fallback · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head
- [dashpay/platform#5638 fix(sdk): check document windows and ranked indexes before sending](https://github.com/dashpay/platform/pull/5638) — by @shumkov · areas: rs-drive, rs-drive-abci, rust-sdk, rust-sdk-ffi, js-wasm-sdk, github, fallback · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head
- [dashpay/rust-dashcore#849 feat(dash-spv): add \`--birth-height\` CLI flag](https://github.com/dashpay/rust-dashcore/pull/849) — by @xdustinface · areas: dash-spv · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 9b1e709fd8e0bfab0dd69f7c86fd6b7d2e585fbe
- [dashpay/rust-dashcore#1223 feat(dash-network)!: source parameters from \`dash-params\`, bump Base SDK crates to dashpay/base-sdk@89828032](https://github.com/dashpay/rust-dashcore/pull/1223) — by @kwvg · areas: dash-spv, key-wallet, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
- [dashpay/rust-dashcore#1226 feat(dash): check address maps against Core v24 port and duplicate rules](https://github.com/dashpay/rust-dashcore/pull/1226) — by @shumkov · areas: fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 05b08aa8c2fa3aa44a3235ffeec9de7fb3f0324b

<a id="claudius-maginificent"></a>
### @Claudius-Maginificent
<a id="claudius-maginificent-open"></a>
#### Open (42)
- [dashpay/tenderdash#1562 docs(consensus): add source-derived consensus protocol specification](https://github.com/dashpay/tenderdash/pull/1562) — 5 unresolved (5 bot) · 6 days stale · ✋ changes requested · areas: fallback · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Spec cites ErrUnsupportedVoteExtensionType that does not exist until #1565 merges**" — 6 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5565 feat(platform-wallet)!: detect inconsistent shielded commitment trees and add explicit rebuild](https://github.com/dashpay/platform/pull/5565) — 4 unresolved (4 bot) · 4 days stale · ✋ changes requested · 🐢 targets fix/shielded-sync-concurrent-append · areas: rs-platform-wallet
  - Top thread: "🟡 Suggestion: Reset host-visible scan metadata during a tree rebuild**" — 4 days old
- [dashpay/platform#5562 fix(sdk): retry address nonce skew on another node](https://github.com/dashpay/platform/pull/5562) — 3 unresolved (3 bot) · 5 days stale · 🔴 CI failing · areas: rust-sdk · Policy: ready-to-merge
  - Top thread: "🟡 Suggestion: Add coverage for the real CheckTx consensus-error conversion path**" — 5 days old
  - Blocker: All policy requirements are satisfied
- [dashpay/dash-evo-tool#1108 feat(documents): recognise indexOnly document types and delete their entries by values](https://github.com/dashpay/dash-evo-tool/pull/1108) — 3 unresolved (3 bot) · 5 days stale · ✋ changes requested · 📝 draft · 🐢 targets feat/document-optional-token-cost · areas: fallback
  - Top thread: "🟡 Suggestion: Make the non-deletable error actionable**" — 5 days old
- [dashpay/dash-evo-tool#1112 feat(send): one From/To Send form with Auto routing](https://github.com/dashpay/dash-evo-tool/pull/1112) — 4 unresolved (4 bot) · 1 days stale · 🐢 targets feat/wallet-max-send-limits · areas: fallback
  - Top thread: "🟡 Suggestion: Do not cache identity database failures as an empty list**" — 1 days old
- [dashpay/platform#5588 feat(platform-wallet)!: expose protocol-14-aware signing-key selection](https://github.com/dashpay/platform/pull/5588) — 3 unresolved (3 bot) · 1 days stale · ✋ changes requested · 🔴 CI failing · areas: dpp, rs-platform-wallet, rs-platform-wallet-ffi · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Use an authoritative or conservative time source for expiry eligibility**" — 1 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
- [dashpay/platform#5564 fix(platform-wallet): stop concurrent shielded syncs from duplicating commitments](https://github.com/dashpay/platform/pull/5564) — 1 unresolved (1 bot) · 5 days stale · 🔴 CI failing · areas: rs-platform-wallet · Policy: waiting-build
  - Top thread: "🟡 Suggestion: Name the mock stream window to document its coupling to sync configuration**" — 5 days old
  - Blocker: The build must pass before a human is asked
- [dashpay/dash-evo-tool#1117 fix(identity): choose signing keys Platform accepts on protocol 14](https://github.com/dashpay/dash-evo-tool/pull/1117) — 1 unresolved (1 bot) · 1 days stale · ✋ changes requested · areas: fallback · Policy: waiting-author
  - Top thread: "🔴 Blocking: Use the same live, scoped eligibility rules for chooser gates and entries**" — 1 days old
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/dash-evo-tool#1124 feat(documents): let the user skip an optional token cost](https://github.com/dashpay/dash-evo-tool/pull/1124) — 1 unresolved (1 bot) · 2 days stale · ✋ changes requested · 📝 draft · 🐢 targets fix/protocol14-signing-key-selection · areas: fallback
  - Top thread: "🔴 Blocking: Show the credit-fee outcome when an optional token cost is disabled**" — 2 days old
- [dashpay/dash-evo-tool#1136 fix(tokens): make every token screen work and fit a small window](https://github.com/dashpay/dash-evo-tool/pull/1136) — 1 unresolved (1 CodeRabbit) · 1 days stale · ✋ changes requested · areas: fallback · Policy: waiting-author
  - Top thread: "&lt;picture&gt;&lt;img src="https://storage.googleapis.com/coderabbit_public_assets/review-comment-icons-v1/major-16.svg" width="…" — 1 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: coderabbitai left review threads unresolved; resolve them
- [dashpay/dash-evo-tool#1107 fix(wallet): make every Max and fee match what the upstream builders accept](https://github.com/dashpay/dash-evo-tool/pull/1107) — ✋ changes requested · areas: fallback · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/dash-evo-tool#1109 feat(masternode): use wallet-held node keys and withdraw via platform-wallet](https://github.com/dashpay/dash-evo-tool/pull/1109) — ✋ changes requested · areas: fallback · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/dash-evo-tool#1110 feat(identity): use proven owner balances and keep identity balances ordered by block height](https://github.com/dashpay/dash-evo-tool/pull/1110) — areas: fallback · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/dash-evo-tool#1120 feat(identity): add keys with spending limits and raise key limits](https://github.com/dashpay/dash-evo-tool/pull/1120) — ⚠ merge conflict · 📝 draft · 🐢 targets fix/protocol14-signing-key-selection · areas: fallback
- [dashpay/dash-evo-tool#1121 feat(contracts): view and claim contract fee pots](https://github.com/dashpay/dash-evo-tool/pull/1121) — ⚠ merge conflict · 📝 draft · 🐢 targets fix/protocol14-signing-key-selection · areas: fallback
- [dashpay/dash-evo-tool#1122 feat(documents): agree to document action fees before broadcasting](https://github.com/dashpay/dash-evo-tool/pull/1122) — ⚠ merge conflict · 📝 draft · 🐢 targets fix/protocol14-signing-key-selection · areas: fallback
- [dashpay/dash-evo-tool#1123 feat(identity): add authentication keys bound to a contract](https://github.com/dashpay/dash-evo-tool/pull/1123) — ⚠ merge conflict · 📝 draft · 🐢 targets feat/identity-key-limits · areas: fallback
- [dashpay/dash-evo-tool#1126 refactor: build Platform types with their upstream constructors](https://github.com/dashpay/dash-evo-tool/pull/1126) — areas: fallback · Policy: waiting-author
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
- [dashpay/dash-evo-tool#1128 refactor(tokens)!: send token actions through platform-wallet](https://github.com/dashpay/dash-evo-tool/pull/1128) — 📝 draft · 🐢 targets chore/bump-platform-0d91c133 · areas: fallback
- [dashpay/dash-evo-tool#1130 fix(tokens): keep the Token Creator form and build exactly the preset it shows](https://github.com/dashpay/dash-evo-tool/pull/1130) — areas: fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
  - Blocker: Proceeded without thepastaclaw: no review within the configured window
- [dashpay/dash-evo-tool#1135 fix(ci): fail the migration-fixture download step at once, with the reason](https://github.com/dashpay/dash-evo-tool/pull/1135) — 🔴 CI failing · areas: github, fallback · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/dash-evo-tool#1137 fix(identity): unlock the wallet that holds the chosen signing key](https://github.com/dashpay/dash-evo-tool/pull/1137) — 📝 draft · 🐢 targets fix/protocol14-signing-key-selection · areas: fallback
- [dashpay/dash-evo-tool#1138 fix(wallet): keep Send usable after cancelling a private-balance payment](https://github.com/dashpay/dash-evo-tool/pull/1138) — areas: fallback · Policy: ready-to-merge
  - Blocker: All policy requirements are satisfied
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/dash-evo-tool#1143 docs: bring user stories in line with the shipped send, contact and Platform Info behaviour](https://github.com/dashpay/dash-evo-tool/pull/1143) — areas: fallback · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/dash-evo-tool#1145 fix(identity): show contacts loading and open each contact's details at the top](https://github.com/dashpay/dash-evo-tool/pull/1145) — areas: fallback · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/dash-evo-tool#1146 fix(tokens): state the once-per-identity claim and explain the marketplace lock in plain words](https://github.com/dashpay/dash-evo-tool/pull/1146) — 🔴 CI failing · areas: fallback · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/dash-evo-tool#1147 fix(wallet): show the private-address refusal before Send, and keep suggestions under dialogs](https://github.com/dashpay/dash-evo-tool/pull/1147) — areas: fallback · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/dash-evo-tool#1149 fix(identity): refresh the identity's balance after a paid operation](https://github.com/dashpay/dash-evo-tool/pull/1149) — areas: fallback · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/dash-evo-tool#1150 fix(security): wipe parsed private-key copies from memory](https://github.com/dashpay/dash-evo-tool/pull/1150) — areas: fallback · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/dash-evo-tool#1151 fix(security): wipe private keys parsed on the add-key, import and new-wallet screens](https://github.com/dashpay/dash-evo-tool/pull/1151) — areas: fallback · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/dash-evo-tool#1153 chore(deps): bump dashpay/platform to v5.1-dev 0d91c133](https://github.com/dashpay/dash-evo-tool/pull/1153) — areas: fallback · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/dash-evo-tool#1154 fix(network): block the app while a network switch is in progress](https://github.com/dashpay/dash-evo-tool/pull/1154) — areas: fallback · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/dash-evo-tool#1155 fix(documents): drop fetched state from a previous contract or type](https://github.com/dashpay/dash-evo-tool/pull/1155) — areas: fallback · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head
- [dashpay/dash-evo-tool#1156 fix(dpns): keep a previous network's contest refresh off the new network](https://github.com/dashpay/dash-evo-tool/pull/1156) — 📝 draft · areas: fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/dash-evo-tool#1157 refactor(app): let each domain decide its reactions to backend results](https://github.com/dashpay/dash-evo-tool/pull/1157) — ⚠ merge conflict · 📝 draft · areas: fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#3549 test(platform-wallet): e2e framework + full test suite — triage pins, Found-*/PA-* guards, fail-closed persist, Stage-2 merge](https://github.com/dashpay/platform/pull/3549) — 📝 draft · ⏸ deferred · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk, rust-sdk, kotlin-sdk, github, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5384 fix(platform-wallet)!: persist chain truncations so a stored wallet follows a reorg](https://github.com/dashpay/platform/pull/5384) — ⏸ deferred · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk, kotlin-sdk, fallback
- [dashpay/platform#5417 fix(swift-sdk): reconcile SwiftData transaction accounting per wallet](https://github.com/dashpay/platform/pull/5417) — ⏸ deferred · areas: swift-sdk
- [dashpay/platform#5546 fix(platform-wallet): support HASH160 DashPay contact signing keys](https://github.com/dashpay/platform/pull/5546) — 🔴 CI failing · areas: rs-platform-wallet · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
- [dashpay/platform#5570 fix(platform-wallet): reject zero-amount shielded spends before reserving notes](https://github.com/dashpay/platform/pull/5570) — 🔴 CI failing · areas: dpp, rs-platform-wallet · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
  - Blocker: Proceeded without thepastaclaw: no review within the configured window
- [dashpay/platform#5613 fix(platform-wallet-storage): persist sentinel-scoped token balances of wallet-owned identities](https://github.com/dashpay/platform/pull/5613) — 🔴 CI failing · 📝 draft · areas: wallet-storage · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5642 chore: make layer-ownership catch implementation details leaking to clients](https://github.com/dashpay/platform/pull/5642) — 📝 draft · areas: fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot

<a id="claudius-maginificent-needs-action"></a>
#### Needs action (8)
- [dashpay/tenderdash#1562 docs(consensus): add source-derived consensus protocol specification](https://github.com/dashpay/tenderdash/pull/1562) — 5 unresolved (5 bot) · 6 days stale · ✋ changes requested · areas: fallback · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Spec cites ErrUnsupportedVoteExtensionType that does not exist until #1565 merges**" — 6 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5562 fix(sdk): retry address nonce skew on another node](https://github.com/dashpay/platform/pull/5562) — 3 unresolved (3 bot) · 5 days stale · 🔴 CI failing · areas: rust-sdk · Policy: ready-to-merge
  - Top thread: "🟡 Suggestion: Add coverage for the real CheckTx consensus-error conversion path**" — 5 days old
  - Blocker: All policy requirements are satisfied
- [dashpay/platform#5588 feat(platform-wallet)!: expose protocol-14-aware signing-key selection](https://github.com/dashpay/platform/pull/5588) — 3 unresolved (3 bot) · 1 days stale · ✋ changes requested · 🔴 CI failing · areas: dpp, rs-platform-wallet, rs-platform-wallet-ffi · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Use an authoritative or conservative time source for expiry eligibility**" — 1 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
- [dashpay/platform#5564 fix(platform-wallet): stop concurrent shielded syncs from duplicating commitments](https://github.com/dashpay/platform/pull/5564) — 1 unresolved (1 bot) · 5 days stale · 🔴 CI failing · areas: rs-platform-wallet · Policy: waiting-build
  - Top thread: "🟡 Suggestion: Name the mock stream window to document its coupling to sync configuration**" — 5 days old
  - Blocker: The build must pass before a human is asked
- [dashpay/dash-evo-tool#1117 fix(identity): choose signing keys Platform accepts on protocol 14](https://github.com/dashpay/dash-evo-tool/pull/1117) — 1 unresolved (1 bot) · 1 days stale · ✋ changes requested · areas: fallback · Policy: waiting-author
  - Top thread: "🔴 Blocking: Use the same live, scoped eligibility rules for chooser gates and entries**" — 1 days old
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/dash-evo-tool#1136 fix(tokens): make every token screen work and fit a small window](https://github.com/dashpay/dash-evo-tool/pull/1136) — 1 unresolved (1 CodeRabbit) · 1 days stale · ✋ changes requested · areas: fallback · Policy: waiting-author
  - Top thread: "&lt;picture&gt;&lt;img src="https://storage.googleapis.com/coderabbit_public_assets/review-comment-icons-v1/major-16.svg" width="…" — 1 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: coderabbitai left review threads unresolved; resolve them
- [dashpay/dash-evo-tool#1107 fix(wallet): make every Max and fee match what the upstream builders accept](https://github.com/dashpay/dash-evo-tool/pull/1107) — ✋ changes requested · areas: fallback · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/dash-evo-tool#1109 feat(masternode): use wallet-held node keys and withdraw via platform-wallet](https://github.com/dashpay/dash-evo-tool/pull/1109) — ✋ changes requested · areas: fallback · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return

<a id="claudius-maginificent-ready-for-human"></a>
#### Ready for human (5)
- [dashpay/platform#5562 fix(sdk): retry address nonce skew on another node](https://github.com/dashpay/platform/pull/5562) — 3 unresolved (3 bot) · 5 days stale · 🔴 CI failing · areas: rust-sdk · Policy: ready-to-merge
  - Top thread: "🟡 Suggestion: Add coverage for the real CheckTx consensus-error conversion path**" — 5 days old
  - Blocker: All policy requirements are satisfied
- [dashpay/dash-evo-tool#1130 fix(tokens): keep the Token Creator form and build exactly the preset it shows](https://github.com/dashpay/dash-evo-tool/pull/1130) — areas: fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
  - Blocker: Proceeded without thepastaclaw: no review within the configured window
- [dashpay/dash-evo-tool#1138 fix(wallet): keep Send usable after cancelling a private-balance payment](https://github.com/dashpay/dash-evo-tool/pull/1138) — areas: fallback · Policy: ready-to-merge
  - Blocker: All policy requirements are satisfied
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/platform#5546 fix(platform-wallet): support HASH160 DashPay contact signing keys](https://github.com/dashpay/platform/pull/5546) — 🔴 CI failing · areas: rs-platform-wallet · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
- [dashpay/platform#5570 fix(platform-wallet): reject zero-amount shielded spends before reserving notes](https://github.com/dashpay/platform/pull/5570) — 🔴 CI failing · areas: dpp, rs-platform-wallet · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
  - Blocker: Proceeded without thepastaclaw: no review within the configured window

<a id="claudius-maginificent-unresolved-comments"></a>
#### Unresolved Comments (6)
- [dashpay/tenderdash#1562 docs(consensus): add source-derived consensus protocol specification](https://github.com/dashpay/tenderdash/pull/1562) — 5 unresolved (5 bot) · 6 days stale · ✋ changes requested · areas: fallback · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Spec cites ErrUnsupportedVoteExtensionType that does not exist until #1565 merges**" — 6 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5562 fix(sdk): retry address nonce skew on another node](https://github.com/dashpay/platform/pull/5562) — 3 unresolved (3 bot) · 5 days stale · 🔴 CI failing · areas: rust-sdk · Policy: ready-to-merge
  - Top thread: "🟡 Suggestion: Add coverage for the real CheckTx consensus-error conversion path**" — 5 days old
  - Blocker: All policy requirements are satisfied
- [dashpay/platform#5588 feat(platform-wallet)!: expose protocol-14-aware signing-key selection](https://github.com/dashpay/platform/pull/5588) — 3 unresolved (3 bot) · 1 days stale · ✋ changes requested · 🔴 CI failing · areas: dpp, rs-platform-wallet, rs-platform-wallet-ffi · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Use an authoritative or conservative time source for expiry eligibility**" — 1 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
- [dashpay/platform#5564 fix(platform-wallet): stop concurrent shielded syncs from duplicating commitments](https://github.com/dashpay/platform/pull/5564) — 1 unresolved (1 bot) · 5 days stale · 🔴 CI failing · areas: rs-platform-wallet · Policy: waiting-build
  - Top thread: "🟡 Suggestion: Name the mock stream window to document its coupling to sync configuration**" — 5 days old
  - Blocker: The build must pass before a human is asked
- [dashpay/dash-evo-tool#1117 fix(identity): choose signing keys Platform accepts on protocol 14](https://github.com/dashpay/dash-evo-tool/pull/1117) — 1 unresolved (1 bot) · 1 days stale · ✋ changes requested · areas: fallback · Policy: waiting-author
  - Top thread: "🔴 Blocking: Use the same live, scoped eligibility rules for chooser gates and entries**" — 1 days old
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/dash-evo-tool#1136 fix(tokens): make every token screen work and fit a small window](https://github.com/dashpay/dash-evo-tool/pull/1136) — 1 unresolved (1 CodeRabbit) · 1 days stale · ✋ changes requested · areas: fallback · Policy: waiting-author
  - Top thread: "&lt;picture&gt;&lt;img src="https://storage.googleapis.com/coderabbit_public_assets/review-comment-icons-v1/major-16.svg" width="…" — 1 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: coderabbitai left review threads unresolved; resolve them

<a id="claudius-maginificent-changes-requested"></a>
#### Changes Requested (2)
- [dashpay/dash-evo-tool#1107 fix(wallet): make every Max and fee match what the upstream builders accept](https://github.com/dashpay/dash-evo-tool/pull/1107) — ✋ changes requested · areas: fallback · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/dash-evo-tool#1109 feat(masternode): use wallet-held node keys and withdraw via platform-wallet](https://github.com/dashpay/dash-evo-tool/pull/1109) — ✋ changes requested · areas: fallback · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return

<a id="claudius-maginificent-ci-failing"></a>
#### CI Failing (4)
- [dashpay/dash-evo-tool#1135 fix(ci): fail the migration-fixture download step at once, with the reason](https://github.com/dashpay/dash-evo-tool/pull/1135) — 🔴 CI failing · areas: github, fallback · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/dash-evo-tool#1146 fix(tokens): state the once-per-identity claim and explain the marketplace lock in plain words](https://github.com/dashpay/dash-evo-tool/pull/1146) — 🔴 CI failing · areas: fallback · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/platform#5546 fix(platform-wallet): support HASH160 DashPay contact signing keys](https://github.com/dashpay/platform/pull/5546) — 🔴 CI failing · areas: rs-platform-wallet · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
- [dashpay/platform#5570 fix(platform-wallet): reject zero-amount shielded spends before reserving notes](https://github.com/dashpay/platform/pull/5570) — 🔴 CI failing · areas: dpp, rs-platform-wallet · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
  - Blocker: Proceeded without thepastaclaw: no review within the configured window

<a id="claudius-maginificent-deferred"></a>
#### Deferred (3)
- [dashpay/platform#3549 test(platform-wallet): e2e framework + full test suite — triage pins, Found-*/PA-* guards, fail-closed persist, Stage-2 merge](https://github.com/dashpay/platform/pull/3549) — 📝 draft · ⏸ deferred · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk, rust-sdk, kotlin-sdk, github, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5384 fix(platform-wallet)!: persist chain truncations so a stored wallet follows a reorg](https://github.com/dashpay/platform/pull/5384) — ⏸ deferred · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk, kotlin-sdk, fallback
- [dashpay/platform#5417 fix(swift-sdk): reconcile SwiftData transaction accounting per wallet](https://github.com/dashpay/platform/pull/5417) — ⏸ deferred · areas: swift-sdk

<a id="claudius-maginificent-draft"></a>
#### Draft (4)
- [dashpay/dash-evo-tool#1156 fix(dpns): keep a previous network's contest refresh off the new network](https://github.com/dashpay/dash-evo-tool/pull/1156) — 📝 draft · areas: fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/dash-evo-tool#1157 refactor(app): let each domain decide its reactions to backend results](https://github.com/dashpay/dash-evo-tool/pull/1157) — ⚠ merge conflict · 📝 draft · areas: fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5613 fix(platform-wallet-storage): persist sentinel-scoped token balances of wallet-owned identities](https://github.com/dashpay/platform/pull/5613) — 🔴 CI failing · 📝 draft · areas: wallet-storage · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5642 chore: make layer-ownership catch implementation details leaking to clients](https://github.com/dashpay/platform/pull/5642) — 📝 draft · areas: fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot

<a id="claudius-maginificent-stale"></a>
#### Stale (10)
- [dashpay/platform#5565 feat(platform-wallet)!: detect inconsistent shielded commitment trees and add explicit rebuild](https://github.com/dashpay/platform/pull/5565) — 4 unresolved (4 bot) · 4 days stale · ✋ changes requested · 🐢 targets fix/shielded-sync-concurrent-append · areas: rs-platform-wallet
  - Top thread: "🟡 Suggestion: Reset host-visible scan metadata during a tree rebuild**" — 4 days old
- [dashpay/dash-evo-tool#1108 feat(documents): recognise indexOnly document types and delete their entries by values](https://github.com/dashpay/dash-evo-tool/pull/1108) — 3 unresolved (3 bot) · 5 days stale · ✋ changes requested · 📝 draft · 🐢 targets feat/document-optional-token-cost · areas: fallback
  - Top thread: "🟡 Suggestion: Make the non-deletable error actionable**" — 5 days old
- [dashpay/dash-evo-tool#1112 feat(send): one From/To Send form with Auto routing](https://github.com/dashpay/dash-evo-tool/pull/1112) — 4 unresolved (4 bot) · 1 days stale · 🐢 targets feat/wallet-max-send-limits · areas: fallback
  - Top thread: "🟡 Suggestion: Do not cache identity database failures as an empty list**" — 1 days old
- [dashpay/dash-evo-tool#1124 feat(documents): let the user skip an optional token cost](https://github.com/dashpay/dash-evo-tool/pull/1124) — 1 unresolved (1 bot) · 2 days stale · ✋ changes requested · 📝 draft · 🐢 targets fix/protocol14-signing-key-selection · areas: fallback
  - Top thread: "🔴 Blocking: Show the credit-fee outcome when an optional token cost is disabled**" — 2 days old
- [dashpay/dash-evo-tool#1120 feat(identity): add keys with spending limits and raise key limits](https://github.com/dashpay/dash-evo-tool/pull/1120) — ⚠ merge conflict · 📝 draft · 🐢 targets fix/protocol14-signing-key-selection · areas: fallback
- [dashpay/dash-evo-tool#1121 feat(contracts): view and claim contract fee pots](https://github.com/dashpay/dash-evo-tool/pull/1121) — ⚠ merge conflict · 📝 draft · 🐢 targets fix/protocol14-signing-key-selection · areas: fallback
- [dashpay/dash-evo-tool#1122 feat(documents): agree to document action fees before broadcasting](https://github.com/dashpay/dash-evo-tool/pull/1122) — ⚠ merge conflict · 📝 draft · 🐢 targets fix/protocol14-signing-key-selection · areas: fallback
- [dashpay/dash-evo-tool#1123 feat(identity): add authentication keys bound to a contract](https://github.com/dashpay/dash-evo-tool/pull/1123) — ⚠ merge conflict · 📝 draft · 🐢 targets feat/identity-key-limits · areas: fallback
- [dashpay/dash-evo-tool#1128 refactor(tokens)!: send token actions through platform-wallet](https://github.com/dashpay/dash-evo-tool/pull/1128) — 📝 draft · 🐢 targets chore/bump-platform-0d91c133 · areas: fallback
- [dashpay/dash-evo-tool#1137 fix(identity): unlock the wallet that holds the chosen signing key](https://github.com/dashpay/dash-evo-tool/pull/1137) — 📝 draft · 🐢 targets fix/protocol14-signing-key-selection · areas: fallback

<a id="claudius-maginificent-clean"></a>
#### Clean (13)
- [dashpay/dash-evo-tool#1110 feat(identity): use proven owner balances and keep identity balances ordered by block height](https://github.com/dashpay/dash-evo-tool/pull/1110) — areas: fallback · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/dash-evo-tool#1126 refactor: build Platform types with their upstream constructors](https://github.com/dashpay/dash-evo-tool/pull/1126) — areas: fallback · Policy: waiting-author
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
- [dashpay/dash-evo-tool#1130 fix(tokens): keep the Token Creator form and build exactly the preset it shows](https://github.com/dashpay/dash-evo-tool/pull/1130) — areas: fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
  - Blocker: Proceeded without thepastaclaw: no review within the configured window
- [dashpay/dash-evo-tool#1138 fix(wallet): keep Send usable after cancelling a private-balance payment](https://github.com/dashpay/dash-evo-tool/pull/1138) — areas: fallback · Policy: ready-to-merge
  - Blocker: All policy requirements are satisfied
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/dash-evo-tool#1143 docs: bring user stories in line with the shipped send, contact and Platform Info behaviour](https://github.com/dashpay/dash-evo-tool/pull/1143) — areas: fallback · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/dash-evo-tool#1145 fix(identity): show contacts loading and open each contact's details at the top](https://github.com/dashpay/dash-evo-tool/pull/1145) — areas: fallback · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/dash-evo-tool#1147 fix(wallet): show the private-address refusal before Send, and keep suggestions under dialogs](https://github.com/dashpay/dash-evo-tool/pull/1147) — areas: fallback · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/dash-evo-tool#1149 fix(identity): refresh the identity's balance after a paid operation](https://github.com/dashpay/dash-evo-tool/pull/1149) — areas: fallback · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/dash-evo-tool#1150 fix(security): wipe parsed private-key copies from memory](https://github.com/dashpay/dash-evo-tool/pull/1150) — areas: fallback · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/dash-evo-tool#1151 fix(security): wipe private keys parsed on the add-key, import and new-wallet screens](https://github.com/dashpay/dash-evo-tool/pull/1151) — areas: fallback · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/dash-evo-tool#1153 chore(deps): bump dashpay/platform to v5.1-dev 0d91c133](https://github.com/dashpay/dash-evo-tool/pull/1153) — areas: fallback · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/dash-evo-tool#1154 fix(network): block the app while a network switch is in progress](https://github.com/dashpay/dash-evo-tool/pull/1154) — areas: fallback · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/dash-evo-tool#1155 fix(documents): drop fetched state from a previous contract or type](https://github.com/dashpay/dash-evo-tool/pull/1155) — areas: fallback · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head

<a id="shumkov"></a>
### @shumkov
<a id="shumkov-open"></a>
#### Open (20)
- [dashpay/rust-dashcore#634 feat(key-wallet-manager): single-map WalletManager with Arc&lt;RwLock&lt;T&gt;&gt; per wallet](https://github.com/dashpay/rust-dashcore/pull/634) — 20 unresolved (20 CodeRabbit) · 203 days stale · ⚠ merge conflict · ✋ changes requested · areas: dash-spv, key-wallet, key-wallet-manager, fallback · Policy: waiting-author
  - Top thread: "_⚠️ Potential issue_ \| _🟡 Minor_" — 203 days old
  - Blocker: coderabbitai requested changes on this head; dismiss the review or push a fix
  - Blocker: coderabbitai left review threads unresolved; resolve them
- [dashpay/platform#4660 feat(sdk): expose the document erase and history lifecycle on mobile and FFI](https://github.com/dashpay/platform/pull/4660) — 2 unresolved (2 bot) · 35 days stale · ✋ changes requested · 🔴 CI failing · 🐢 targets keep-history-lifecycle · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, rust-sdk-ffi, kotlin-sdk, fallback
  - Top thread: "🟡 Suggestion: Validate history selector discriminant at the C boundary**" — 35 days old
- [dashpay/platform#4657 feat(platform)!: delete and erase lifecycle for keep-history documents](https://github.com/dashpay/platform/pull/4657) — 2 unresolved (2 bot) · 35 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · 🐢 targets keep-history-storage-v2 · areas: rs-drive, rs-drive-abci, dpp, rust-sdk, js-wasm-sdk, fallback
  - Top thread: "🟡 Suggestion: Deduplicate lifecycle containers at the Drive batch boundary**" — 35 days old
- [dashpay/platform#4283 fix(dashmate): keep the node up when an image pull fails, and stop reporting success when it did not](https://github.com/dashpay/platform/pull/4283) — 2 unresolved (2 bot) · 8 days stale · ✋ changes requested · 🔴 CI failing · areas: dashmate · Policy: waiting-author
  - Top thread: "🔴 Blocking: Preserve Docker login credentials in the restart preflight**" — 8 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#4652 feat(drive)!: preserve composite document history across protocol activation](https://github.com/dashpay/platform/pull/4652) — 1 unresolved (1 bot) · 6 days stale · ⚠ merge conflict · ✋ changes requested · areas: rs-drive, rs-drive-abci, rust-sdk, js-wasm-sdk, fallback · Policy: waiting-author
  - Top thread: "🔴 Blocking: Bound or stage the attacker-inflatable activation migration**" — 6 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/grovedb#1025 feat(query)!: page count trees by a branch-local offset in merged proofs (GROVE_V5)](https://github.com/dashpay/grovedb/pull/1025) — 1 unresolved (1 bot) · 1 days stale · 🔴 CI failing · areas: fallback · Policy: waiting-author
  - Top thread: "✅ **Resolved** at \`519277b7\`; see the replies below." — 1 days old
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5629 fix(platform-wallet): refuse ports Dash Core v24 refuses before building a version-3 ProUpServTx](https://github.com/dashpay/platform/pull/5629) — 1 unresolved (1 bot) · 0 days stale · areas: rs-platform-wallet · Policy: waiting-author
  - Top thread: "⏭️ **Deferred** (maintainer decision; not blocking; tracked in https://github.com/dashpay/rust-dashcore/issues/1226) at …" — 0 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
- [dashpay/platform#4282 fix(dashmate)!: give Debian packages versions apt can order](https://github.com/dashpay/platform/pull/4282) — 🔴 CI failing · areas: dashmate, github, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
- [dashpay/platform#5615 fix(sdk): refuse a non-unique identity V1 request before protocol 14 is proven](https://github.com/dashpay/platform/pull/5615) — areas: rust-sdk, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 587ba0843d4104b65772b9341461eafd8a057f0f
- [dashpay/platform#5628 fix(sdk): read the proved vote of a masternode vote that already exists](https://github.com/dashpay/platform/pull/5628) — areas: rs-drive-abci, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 2cfa0d137a372465d593ff022f0052d7890a0f74
- [dashpay/platform#5630 fix(drive-abci): make zero quorum windows and active-signer counts unrepresentable](https://github.com/dashpay/platform/pull/5630) — areas: rs-drive-abci · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 61041b0828035c2d75df0bcf1a9ae73a64668058
- [dashpay/platform#5634 feat(drive-abci)!: emit version 2 asset unlocks once Core V24 is active (PV15)](https://github.com/dashpay/platform/pull/5634) — areas: rs-drive-abci, fallback · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head
- [dashpay/platform#5635 fix(sdk)!: refuse document requests no node can answer instead of banning nodes](https://github.com/dashpay/platform/pull/5635) — areas: rs-drive, rs-drive-abci, rust-sdk, rust-sdk-ffi, js-wasm-sdk, github, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 771e06ec7fc942639e36774aa152c783c8cf0f41
- [dashpay/platform#5636 feat(sdk)!: decide from the version tables which networks serve a document query](https://github.com/dashpay/platform/pull/5636) — areas: rs-drive, rs-drive-abci, rust-sdk, rust-sdk-ffi, js-wasm-sdk, github, fallback · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head
- [dashpay/platform#5637 chore(platform)!: update rust-dashcore to dev 7329126e](https://github.com/dashpay/platform/pull/5637) — areas: dpp, rs-platform-wallet, fallback · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head
- [dashpay/platform#5638 fix(sdk): check document windows and ranked indexes before sending](https://github.com/dashpay/platform/pull/5638) — areas: rs-drive, rs-drive-abci, rust-sdk, rust-sdk-ffi, js-wasm-sdk, github, fallback · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head
- [dashpay/rust-dashcore#1205 fix(dash-spv)!: keep masternode sync live after refused responses](https://github.com/dashpay/rust-dashcore/pull/1205) — ⚠ merge conflict · ✋ changes requested · areas: dash-spv · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
- [dashpay/rust-dashcore#1206 feat(dash-spv)!: check masternode list diffs against the block coinbase](https://github.com/dashpay/rust-dashcore/pull/1206) — 📝 draft · 🐢 targets shumkov/masternode-sync-liveness · areas: dash-spv, fallback
- [dashpay/rust-dashcore#1212 feat(dash-spv)!: let a network request avoid given peers](https://github.com/dashpay/rust-dashcore/pull/1212) — ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · areas: dash-spv · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 1c67e9ad99aa3b92f537285355aa157dfd5ee52a
- [dashpay/rust-dashcore#1226 feat(dash): check address maps against Core v24 port and duplicate rules](https://github.com/dashpay/rust-dashcore/pull/1226) — areas: fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 05b08aa8c2fa3aa44a3235ffeec9de7fb3f0324b

<a id="shumkov-needs-action"></a>
#### Needs action (7)
- [dashpay/rust-dashcore#634 feat(key-wallet-manager): single-map WalletManager with Arc&lt;RwLock&lt;T&gt;&gt; per wallet](https://github.com/dashpay/rust-dashcore/pull/634) — 20 unresolved (20 CodeRabbit) · 203 days stale · ⚠ merge conflict · ✋ changes requested · areas: dash-spv, key-wallet, key-wallet-manager, fallback · Policy: waiting-author
  - Top thread: "_⚠️ Potential issue_ \| _🟡 Minor_" — 203 days old
  - Blocker: coderabbitai requested changes on this head; dismiss the review or push a fix
  - Blocker: coderabbitai left review threads unresolved; resolve them
- [dashpay/platform#4283 fix(dashmate): keep the node up when an image pull fails, and stop reporting success when it did not](https://github.com/dashpay/platform/pull/4283) — 2 unresolved (2 bot) · 8 days stale · ✋ changes requested · 🔴 CI failing · areas: dashmate · Policy: waiting-author
  - Top thread: "🔴 Blocking: Preserve Docker login credentials in the restart preflight**" — 8 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#4652 feat(drive)!: preserve composite document history across protocol activation](https://github.com/dashpay/platform/pull/4652) — 1 unresolved (1 bot) · 6 days stale · ⚠ merge conflict · ✋ changes requested · areas: rs-drive, rs-drive-abci, rust-sdk, js-wasm-sdk, fallback · Policy: waiting-author
  - Top thread: "🔴 Blocking: Bound or stage the attacker-inflatable activation migration**" — 6 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/grovedb#1025 feat(query)!: page count trees by a branch-local offset in merged proofs (GROVE_V5)](https://github.com/dashpay/grovedb/pull/1025) — 1 unresolved (1 bot) · 1 days stale · 🔴 CI failing · areas: fallback · Policy: waiting-author
  - Top thread: "✅ **Resolved** at \`519277b7\`; see the replies below." — 1 days old
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5629 fix(platform-wallet): refuse ports Dash Core v24 refuses before building a version-3 ProUpServTx](https://github.com/dashpay/platform/pull/5629) — 1 unresolved (1 bot) · 0 days stale · areas: rs-platform-wallet · Policy: waiting-author
  - Top thread: "⏭️ **Deferred** (maintainer decision; not blocking; tracked in https://github.com/dashpay/rust-dashcore/issues/1226) at …" — 0 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
- [dashpay/rust-dashcore#1205 fix(dash-spv)!: keep masternode sync live after refused responses](https://github.com/dashpay/rust-dashcore/pull/1205) — ⚠ merge conflict · ✋ changes requested · areas: dash-spv · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
- [dashpay/rust-dashcore#1212 feat(dash-spv)!: let a network request avoid given peers](https://github.com/dashpay/rust-dashcore/pull/1212) — ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · areas: dash-spv · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 1c67e9ad99aa3b92f537285355aa157dfd5ee52a

<a id="shumkov-ready-for-human"></a>
#### Ready for human (2)
- [dashpay/platform#4282 fix(dashmate)!: give Debian packages versions apt can order](https://github.com/dashpay/platform/pull/4282) — 🔴 CI failing · areas: dashmate, github, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
- [dashpay/rust-dashcore#1205 fix(dash-spv)!: keep masternode sync live after refused responses](https://github.com/dashpay/rust-dashcore/pull/1205) — ⚠ merge conflict · ✋ changes requested · areas: dash-spv · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required

<a id="shumkov-unresolved-comments"></a>
#### Unresolved Comments (5)
- [dashpay/rust-dashcore#634 feat(key-wallet-manager): single-map WalletManager with Arc&lt;RwLock&lt;T&gt;&gt; per wallet](https://github.com/dashpay/rust-dashcore/pull/634) — 20 unresolved (20 CodeRabbit) · 203 days stale · ⚠ merge conflict · ✋ changes requested · areas: dash-spv, key-wallet, key-wallet-manager, fallback · Policy: waiting-author
  - Top thread: "_⚠️ Potential issue_ \| _🟡 Minor_" — 203 days old
  - Blocker: coderabbitai requested changes on this head; dismiss the review or push a fix
  - Blocker: coderabbitai left review threads unresolved; resolve them
- [dashpay/platform#4283 fix(dashmate): keep the node up when an image pull fails, and stop reporting success when it did not](https://github.com/dashpay/platform/pull/4283) — 2 unresolved (2 bot) · 8 days stale · ✋ changes requested · 🔴 CI failing · areas: dashmate · Policy: waiting-author
  - Top thread: "🔴 Blocking: Preserve Docker login credentials in the restart preflight**" — 8 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#4652 feat(drive)!: preserve composite document history across protocol activation](https://github.com/dashpay/platform/pull/4652) — 1 unresolved (1 bot) · 6 days stale · ⚠ merge conflict · ✋ changes requested · areas: rs-drive, rs-drive-abci, rust-sdk, js-wasm-sdk, fallback · Policy: waiting-author
  - Top thread: "🔴 Blocking: Bound or stage the attacker-inflatable activation migration**" — 6 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/grovedb#1025 feat(query)!: page count trees by a branch-local offset in merged proofs (GROVE_V5)](https://github.com/dashpay/grovedb/pull/1025) — 1 unresolved (1 bot) · 1 days stale · 🔴 CI failing · areas: fallback · Policy: waiting-author
  - Top thread: "✅ **Resolved** at \`519277b7\`; see the replies below." — 1 days old
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5629 fix(platform-wallet): refuse ports Dash Core v24 refuses before building a version-3 ProUpServTx](https://github.com/dashpay/platform/pull/5629) — 1 unresolved (1 bot) · 0 days stale · areas: rs-platform-wallet · Policy: waiting-author
  - Top thread: "⏭️ **Deferred** (maintainer decision; not blocking; tracked in https://github.com/dashpay/rust-dashcore/issues/1226) at …" — 0 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix

<a id="shumkov-changes-requested"></a>
#### Changes Requested (2)
- [dashpay/rust-dashcore#1205 fix(dash-spv)!: keep masternode sync live after refused responses](https://github.com/dashpay/rust-dashcore/pull/1205) — ⚠ merge conflict · ✋ changes requested · areas: dash-spv · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
- [dashpay/rust-dashcore#1212 feat(dash-spv)!: let a network request avoid given peers](https://github.com/dashpay/rust-dashcore/pull/1212) — ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · areas: dash-spv · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 1c67e9ad99aa3b92f537285355aa157dfd5ee52a

<a id="shumkov-ci-failing"></a>
#### CI Failing (1)
- [dashpay/platform#4282 fix(dashmate)!: give Debian packages versions apt can order](https://github.com/dashpay/platform/pull/4282) — 🔴 CI failing · areas: dashmate, github, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required

<a id="shumkov-stale"></a>
#### Stale (3)
- [dashpay/platform#4660 feat(sdk): expose the document erase and history lifecycle on mobile and FFI](https://github.com/dashpay/platform/pull/4660) — 2 unresolved (2 bot) · 35 days stale · ✋ changes requested · 🔴 CI failing · 🐢 targets keep-history-lifecycle · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, rust-sdk-ffi, kotlin-sdk, fallback
  - Top thread: "🟡 Suggestion: Validate history selector discriminant at the C boundary**" — 35 days old
- [dashpay/platform#4657 feat(platform)!: delete and erase lifecycle for keep-history documents](https://github.com/dashpay/platform/pull/4657) — 2 unresolved (2 bot) · 35 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · 🐢 targets keep-history-storage-v2 · areas: rs-drive, rs-drive-abci, dpp, rust-sdk, js-wasm-sdk, fallback
  - Top thread: "🟡 Suggestion: Deduplicate lifecycle containers at the Drive batch boundary**" — 35 days old
- [dashpay/rust-dashcore#1206 feat(dash-spv)!: check masternode list diffs against the block coinbase](https://github.com/dashpay/rust-dashcore/pull/1206) — 📝 draft · 🐢 targets shumkov/masternode-sync-liveness · areas: dash-spv, fallback

<a id="shumkov-clean"></a>
#### Clean (9)
- [dashpay/platform#5615 fix(sdk): refuse a non-unique identity V1 request before protocol 14 is proven](https://github.com/dashpay/platform/pull/5615) — areas: rust-sdk, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 587ba0843d4104b65772b9341461eafd8a057f0f
- [dashpay/platform#5628 fix(sdk): read the proved vote of a masternode vote that already exists](https://github.com/dashpay/platform/pull/5628) — areas: rs-drive-abci, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 2cfa0d137a372465d593ff022f0052d7890a0f74
- [dashpay/platform#5630 fix(drive-abci): make zero quorum windows and active-signer counts unrepresentable](https://github.com/dashpay/platform/pull/5630) — areas: rs-drive-abci · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 61041b0828035c2d75df0bcf1a9ae73a64668058
- [dashpay/platform#5634 feat(drive-abci)!: emit version 2 asset unlocks once Core V24 is active (PV15)](https://github.com/dashpay/platform/pull/5634) — areas: rs-drive-abci, fallback · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head
- [dashpay/platform#5635 fix(sdk)!: refuse document requests no node can answer instead of banning nodes](https://github.com/dashpay/platform/pull/5635) — areas: rs-drive, rs-drive-abci, rust-sdk, rust-sdk-ffi, js-wasm-sdk, github, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 771e06ec7fc942639e36774aa152c783c8cf0f41
- [dashpay/platform#5636 feat(sdk)!: decide from the version tables which networks serve a document query](https://github.com/dashpay/platform/pull/5636) — areas: rs-drive, rs-drive-abci, rust-sdk, rust-sdk-ffi, js-wasm-sdk, github, fallback · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head
- [dashpay/platform#5637 chore(platform)!: update rust-dashcore to dev 7329126e](https://github.com/dashpay/platform/pull/5637) — areas: dpp, rs-platform-wallet, fallback · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head
- [dashpay/platform#5638 fix(sdk): check document windows and ranked indexes before sending](https://github.com/dashpay/platform/pull/5638) — areas: rs-drive, rs-drive-abci, rust-sdk, rust-sdk-ffi, js-wasm-sdk, github, fallback · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head
- [dashpay/rust-dashcore#1226 feat(dash): check address maps against Core v24 port and duplicate rules](https://github.com/dashpay/rust-dashcore/pull/1226) — areas: fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 05b08aa8c2fa3aa44a3235ffeec9de7fb3f0324b

<a id="shumkov-ready-for-review"></a>
#### Ready for Review (5)
- [dashpay/platform#4706 feat(platform)!: require fee history for storage refunds and credit their recorded owners](https://github.com/dashpay/platform/pull/4706) — by @DCG-Claude · areas: rs-drive, rs-drive-abci, fallback · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head
- [dashpay/platform#5187 ci: tolerate optional S3 cache export outages on v4.3-dev](https://github.com/dashpay/platform/pull/5187) — by @ktechmidas · areas: github · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed fbbf13ee41f8bd8b9f03f1dc5afec6a619967d97
- [dashpay/platform#5526 feat(platform)!: token marketplace escrow trees and token market contract (PV15)](https://github.com/dashpay/platform/pull/5526) — by @QuantumExplorer · areas: rs-drive, rs-drive-abci, dpp, rust-sdk, rust-sdk-ffi, js-wasm-sdk, system-contracts, github, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed eb5ddbebe2aef913cbda17ab2e9a780b94b9326f
- [dashpay/platform#5612 fix(sdk): let wasm-sdk build without its default features](https://github.com/dashpay/platform/pull/5612) — by @PastaPastaPasta · areas: js-wasm-sdk, fallback · Policy: too-many-open-prs
  - Blocker: Human review is still required; automatic reviewer requests are paused until a review slot is available
- [dashpay/tenderdash#1560 test(e2e): exercise block sync during slow local application](https://github.com/dashpay/tenderdash/pull/1560) — by @lklimek · areas: github, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 1f5fb2540ca89b3c6b2bb311ad149affdd577df4

<a id="pastapastapasta"></a>
### @PastaPastaPasta
<a id="pastapastapasta-open"></a>
#### Open (23)
- [dashpay/platform#4648 feat(drive-abci)!: state sync via ABCI snapshots with reduced platform state (protocol v15)](https://github.com/dashpay/platform/pull/4648) — 8 unresolved (8 bot) · 23 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · 📝 draft · areas: rs-drive, rs-drive-abci, dpp, fallback · Policy: draft
  - Top thread: "🟡 Suggestion: Continue the restored target through consensus and a restart**" — 23 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#137 fix: integration tests, continued dashificiation](https://github.com/dashpay/rust-dashcore/pull/137) — 2 unresolved (2 CodeRabbit) · 418 days stale · ⚠ merge conflict · 🔴 CI failing · 🐢 targets v0.40-dev, untouched 321 days · areas: fallback
  - Top thread: "_🛠️ Refactor suggestion_" — 418 days old
- [dashpay/platform#5288 feat(platform-wallet)!: support DashPay shielded tips with dedicated accounts](https://github.com/dashpay/platform/pull/5288) — 5 unresolved (5 bot) · 23 days stale · ⚠ merge conflict · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk, kotlin-sdk, fallback · Policy: waiting-self-review
  - Top thread: "🟡 Suggestion: Restore outstanding tip submissions before enabling another send**" — 23 days old
  - Blocker: Author must post /self-reviewed 17b627fa2172995dd9117d88f4484b0d02dc7316
- [dashpay/platform#5521 feat(sdk): verify Core quorum proof chains (DIP 175)](https://github.com/dashpay/platform/pull/5521) — 5 unresolved (5 bot) · 7 days stale · areas: github, fallback · Policy: ready-for-human
  - Top thread: "✅ **Resolved** at \`f1c413e9\`; see the replies below." — 7 days old
  - Blocker: Human approval or objection resolution is required
- [dashpay/platform#5523 feat(sdk): add opt-in verified HTTP context provider](https://github.com/dashpay/platform/pull/5523) — 5 unresolved (5 bot) · 7 days stale · 📝 draft · 🐢 targets feat/core-snapshot-proof-verifier · areas: github, fallback
  - Top thread: "✅ **Resolved** at \`78330297\`; see the replies below." — 7 days old
- [dashpay/platform#5277 perf(platform-wallet): don't hold the shielded store lock across sync network I/O](https://github.com/dashpay/platform/pull/5277) — 2 unresolved (1 human, 1 bot) · 12 days stale · ✋ changes requested · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk · Policy: waiting-author
  - Top thread: "Major, not blocking.** A wallet-scoped bump aborts the whole multi-wallet pass. Commit tree leaves are wallet-agnostic, …" — 12 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
- [dashpay/platform#4965 test(sdk): cargo-fuzz harness over drive-proof-verifier FromProof](https://github.com/dashpay/platform/pull/4965) — 1 unresolved (1 bot) · 22 days stale · 📝 draft · areas: github, fallback · Policy: draft
  - Top thread: "🟡 Suggestion: Correct the claim that PV13 reaches the V0 proof decoder**" — 22 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5280 perf(platform-wallet)!: reuse a recent recorded-anchor set on the shielded spend path](https://github.com/dashpay/platform/pull/5280) — 1 unresolved (1 bot) · 13 days stale · 🔴 CI failing · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk · Policy: waiting-self-review
  - Top thread: "🟡 Suggestion: Guard the background-prefetch export against nested Tokio entry**" — 13 days old
  - Blocker: Author must post /self-reviewed f0ef5a65d32dac0c196a572a66a94612eec7834d
- [dashpay/platform#5609 fix(sdk): restore JS DAPI client node discovery after Core V24](https://github.com/dashpay/platform/pull/5609) — 2 unresolved (2 bot) · 1 days stale · 🔴 CI failing · 📝 draft · areas: fallback · Policy: draft
  - Top thread: "✅ **Resolved** at \`5cf1b7d5\`; see the replies below." — 1 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/grovedb#1007 build(deps)!: use dashpay/orchard with the faster prover](https://github.com/dashpay/grovedb/pull/1007) — 🔴 CI failing · 📝 draft · areas: fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#4633 feat(sdk): add dash-sdk-cxx, a C++ binding of dash-sdk for Dash Core](https://github.com/dashpay/platform/pull/4633) — 📝 draft · 🐢 targets build/vendorable-workspace-lockfile · areas: github, fallback
- [dashpay/platform#5282 perf(platform-wallet)!: prove the shield bundle while waiting for the InstantSend lock](https://github.com/dashpay/platform/pull/5282) — ⚠ merge conflict · 🔴 CI failing · areas: dpp, rs-platform-wallet, rs-platform-wallet-ffi, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 6ca6ab3e2b64e7d5ca4c44c1395d15b6ff8165f2
- [dashpay/platform#5285 feat(platform)!: subscribe to committed state transitions matching document and address filters](https://github.com/dashpay/platform/pull/5285) — ⚠ merge conflict · 📝 draft · areas: rs-drive, rs-drive-abci, rust-sdk, dashmate, rust-dapi, github, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5289 feat(sdk)!: optional BIP-39 passphrase through the mnemonic resolver and wallet creation](https://github.com/dashpay/platform/pull/5289) — 📝 draft · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, rust-sdk-ffi, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5292 feat(dashmate): state sync configuration for tenderdash and drive snapshots](https://github.com/dashpay/platform/pull/5292) — ⚠ merge conflict · 📝 draft · areas: dashmate · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5473 build: make the workspace lockfile vendorable again](https://github.com/dashpay/platform/pull/5473) — ⚠ merge conflict · 📝 draft · areas: fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5475 feat(sdk): forward DashPay profiles, invitations and masternode voting in dash-sdk-cxx](https://github.com/dashpay/platform/pull/5475) — ⚠ merge conflict · 📝 draft · areas: rs-platform-wallet, rs-platform-wallet-ffi, rust-sdk, github, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5577 feat: gzip-compress DAPI responses](https://github.com/dashpay/platform/pull/5577) — ✋ changes requested · 🔴 CI failing · areas: dashmate, rust-dapi, fallback · Policy: waiting-author
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
- [dashpay/platform#5610 perf(sdk): verify each block's signature once instead of on every proved response](https://github.com/dashpay/platform/pull/5610) — 🔴 CI failing · 📝 draft · areas: fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5612 fix(sdk): let wasm-sdk build without its default features](https://github.com/dashpay/platform/pull/5612) — areas: js-wasm-sdk, fallback · Policy: too-many-open-prs
  - Blocker: Human review is still required; automatic reviewer requests are paused until a review slot is available
- [dashpay/rust-dashcore#264 feat(dash-spv): Add BIP324 v2 encrypted P2P transport](https://github.com/dashpay/rust-dashcore/pull/264) — ⚠ merge conflict · areas: dash-spv, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
- [dashpay/rust-dashcore#959 feat(dash-spv): adaptive gap-limit probe escalation for BIP44 discovery](https://github.com/dashpay/rust-dashcore/pull/959) — ⚠ merge conflict · 🔴 CI failing · areas: dash-spv, key-wallet, key-wallet-manager, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 7bcac9b9d89c2c4947080b2e6a15503e97eac9f4
- [dashpay/rust-dashcore#1224 build!: take \`base58ck\` from rust-bitcoin's revision so consumers can vendor](https://github.com/dashpay/rust-dashcore/pull/1224) — 🔴 CI failing · areas: fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 3874441c888a594bcf0d1487d52c6bf357a473ea

<a id="pastapastapasta-needs-action"></a>
#### Needs action (8)
- [dashpay/platform#5288 feat(platform-wallet)!: support DashPay shielded tips with dedicated accounts](https://github.com/dashpay/platform/pull/5288) — 5 unresolved (5 bot) · 23 days stale · ⚠ merge conflict · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk, kotlin-sdk, fallback · Policy: waiting-self-review
  - Top thread: "🟡 Suggestion: Restore outstanding tip submissions before enabling another send**" — 23 days old
  - Blocker: Author must post /self-reviewed 17b627fa2172995dd9117d88f4484b0d02dc7316
- [dashpay/platform#5521 feat(sdk): verify Core quorum proof chains (DIP 175)](https://github.com/dashpay/platform/pull/5521) — 5 unresolved (5 bot) · 7 days stale · areas: github, fallback · Policy: ready-for-human
  - Top thread: "✅ **Resolved** at \`f1c413e9\`; see the replies below." — 7 days old
  - Blocker: Human approval or objection resolution is required
- [dashpay/platform#5277 perf(platform-wallet): don't hold the shielded store lock across sync network I/O](https://github.com/dashpay/platform/pull/5277) — 2 unresolved (1 human, 1 bot) · 12 days stale · ✋ changes requested · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk · Policy: waiting-author
  - Top thread: "Major, not blocking.** A wallet-scoped bump aborts the whole multi-wallet pass. Commit tree leaves are wallet-agnostic, …" — 12 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
- [dashpay/platform#5280 perf(platform-wallet)!: reuse a recent recorded-anchor set on the shielded spend path](https://github.com/dashpay/platform/pull/5280) — 1 unresolved (1 bot) · 13 days stale · 🔴 CI failing · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk · Policy: waiting-self-review
  - Top thread: "🟡 Suggestion: Guard the background-prefetch export against nested Tokio entry**" — 13 days old
  - Blocker: Author must post /self-reviewed f0ef5a65d32dac0c196a572a66a94612eec7834d
- [dashpay/platform#5282 perf(platform-wallet)!: prove the shield bundle while waiting for the InstantSend lock](https://github.com/dashpay/platform/pull/5282) — ⚠ merge conflict · 🔴 CI failing · areas: dpp, rs-platform-wallet, rs-platform-wallet-ffi, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 6ca6ab3e2b64e7d5ca4c44c1395d15b6ff8165f2
- [dashpay/platform#5577 feat: gzip-compress DAPI responses](https://github.com/dashpay/platform/pull/5577) — ✋ changes requested · 🔴 CI failing · areas: dashmate, rust-dapi, fallback · Policy: waiting-author
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
- [dashpay/rust-dashcore#264 feat(dash-spv): Add BIP324 v2 encrypted P2P transport](https://github.com/dashpay/rust-dashcore/pull/264) — ⚠ merge conflict · areas: dash-spv, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
- [dashpay/rust-dashcore#959 feat(dash-spv): adaptive gap-limit probe escalation for BIP44 discovery](https://github.com/dashpay/rust-dashcore/pull/959) — ⚠ merge conflict · 🔴 CI failing · areas: dash-spv, key-wallet, key-wallet-manager, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 7bcac9b9d89c2c4947080b2e6a15503e97eac9f4

<a id="pastapastapasta-ready-for-human"></a>
#### Ready for human (2)
- [dashpay/platform#5521 feat(sdk): verify Core quorum proof chains (DIP 175)](https://github.com/dashpay/platform/pull/5521) — 5 unresolved (5 bot) · 7 days stale · areas: github, fallback · Policy: ready-for-human
  - Top thread: "✅ **Resolved** at \`f1c413e9\`; see the replies below." — 7 days old
  - Blocker: Human approval or objection resolution is required
- [dashpay/rust-dashcore#264 feat(dash-spv): Add BIP324 v2 encrypted P2P transport](https://github.com/dashpay/rust-dashcore/pull/264) — ⚠ merge conflict · areas: dash-spv, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required

<a id="pastapastapasta-unresolved-comments"></a>
#### Unresolved Comments (4)
- [dashpay/platform#5288 feat(platform-wallet)!: support DashPay shielded tips with dedicated accounts](https://github.com/dashpay/platform/pull/5288) — 5 unresolved (5 bot) · 23 days stale · ⚠ merge conflict · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk, kotlin-sdk, fallback · Policy: waiting-self-review
  - Top thread: "🟡 Suggestion: Restore outstanding tip submissions before enabling another send**" — 23 days old
  - Blocker: Author must post /self-reviewed 17b627fa2172995dd9117d88f4484b0d02dc7316
- [dashpay/platform#5521 feat(sdk): verify Core quorum proof chains (DIP 175)](https://github.com/dashpay/platform/pull/5521) — 5 unresolved (5 bot) · 7 days stale · areas: github, fallback · Policy: ready-for-human
  - Top thread: "✅ **Resolved** at \`f1c413e9\`; see the replies below." — 7 days old
  - Blocker: Human approval or objection resolution is required
- [dashpay/platform#5277 perf(platform-wallet): don't hold the shielded store lock across sync network I/O](https://github.com/dashpay/platform/pull/5277) — 2 unresolved (1 human, 1 bot) · 12 days stale · ✋ changes requested · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk · Policy: waiting-author
  - Top thread: "Major, not blocking.** A wallet-scoped bump aborts the whole multi-wallet pass. Commit tree leaves are wallet-agnostic, …" — 12 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
- [dashpay/platform#5280 perf(platform-wallet)!: reuse a recent recorded-anchor set on the shielded spend path](https://github.com/dashpay/platform/pull/5280) — 1 unresolved (1 bot) · 13 days stale · 🔴 CI failing · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk · Policy: waiting-self-review
  - Top thread: "🟡 Suggestion: Guard the background-prefetch export against nested Tokio entry**" — 13 days old
  - Blocker: Author must post /self-reviewed f0ef5a65d32dac0c196a572a66a94612eec7834d

<a id="pastapastapasta-changes-requested"></a>
#### Changes Requested (1)
- [dashpay/platform#5577 feat: gzip-compress DAPI responses](https://github.com/dashpay/platform/pull/5577) — ✋ changes requested · 🔴 CI failing · areas: dashmate, rust-dapi, fallback · Policy: waiting-author
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix

<a id="pastapastapasta-ci-failing"></a>
#### CI Failing (3)
- [dashpay/platform#5282 perf(platform-wallet)!: prove the shield bundle while waiting for the InstantSend lock](https://github.com/dashpay/platform/pull/5282) — ⚠ merge conflict · 🔴 CI failing · areas: dpp, rs-platform-wallet, rs-platform-wallet-ffi, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 6ca6ab3e2b64e7d5ca4c44c1395d15b6ff8165f2
- [dashpay/rust-dashcore#959 feat(dash-spv): adaptive gap-limit probe escalation for BIP44 discovery](https://github.com/dashpay/rust-dashcore/pull/959) — ⚠ merge conflict · 🔴 CI failing · areas: dash-spv, key-wallet, key-wallet-manager, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 7bcac9b9d89c2c4947080b2e6a15503e97eac9f4
- [dashpay/rust-dashcore#1224 build!: take \`base58ck\` from rust-bitcoin's revision so consumers can vendor](https://github.com/dashpay/rust-dashcore/pull/1224) — 🔴 CI failing · areas: fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 3874441c888a594bcf0d1487d52c6bf357a473ea

<a id="pastapastapasta-draft"></a>
#### Draft (10)
- [dashpay/platform#4648 feat(drive-abci)!: state sync via ABCI snapshots with reduced platform state (protocol v15)](https://github.com/dashpay/platform/pull/4648) — 8 unresolved (8 bot) · 23 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · 📝 draft · areas: rs-drive, rs-drive-abci, dpp, fallback · Policy: draft
  - Top thread: "🟡 Suggestion: Continue the restored target through consensus and a restart**" — 23 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#4965 test(sdk): cargo-fuzz harness over drive-proof-verifier FromProof](https://github.com/dashpay/platform/pull/4965) — 1 unresolved (1 bot) · 22 days stale · 📝 draft · areas: github, fallback · Policy: draft
  - Top thread: "🟡 Suggestion: Correct the claim that PV13 reaches the V0 proof decoder**" — 22 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5609 fix(sdk): restore JS DAPI client node discovery after Core V24](https://github.com/dashpay/platform/pull/5609) — 2 unresolved (2 bot) · 1 days stale · 🔴 CI failing · 📝 draft · areas: fallback · Policy: draft
  - Top thread: "✅ **Resolved** at \`5cf1b7d5\`; see the replies below." — 1 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/grovedb#1007 build(deps)!: use dashpay/orchard with the faster prover](https://github.com/dashpay/grovedb/pull/1007) — 🔴 CI failing · 📝 draft · areas: fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5285 feat(platform)!: subscribe to committed state transitions matching document and address filters](https://github.com/dashpay/platform/pull/5285) — ⚠ merge conflict · 📝 draft · areas: rs-drive, rs-drive-abci, rust-sdk, dashmate, rust-dapi, github, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5289 feat(sdk)!: optional BIP-39 passphrase through the mnemonic resolver and wallet creation](https://github.com/dashpay/platform/pull/5289) — 📝 draft · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, rust-sdk-ffi, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5292 feat(dashmate): state sync configuration for tenderdash and drive snapshots](https://github.com/dashpay/platform/pull/5292) — ⚠ merge conflict · 📝 draft · areas: dashmate · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5473 build: make the workspace lockfile vendorable again](https://github.com/dashpay/platform/pull/5473) — ⚠ merge conflict · 📝 draft · areas: fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5475 feat(sdk): forward DashPay profiles, invitations and masternode voting in dash-sdk-cxx](https://github.com/dashpay/platform/pull/5475) — ⚠ merge conflict · 📝 draft · areas: rs-platform-wallet, rs-platform-wallet-ffi, rust-sdk, github, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5610 perf(sdk): verify each block's signature once instead of on every proved response](https://github.com/dashpay/platform/pull/5610) — 🔴 CI failing · 📝 draft · areas: fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot

<a id="pastapastapasta-stale"></a>
#### Stale (3)
- [dashpay/rust-dashcore#137 fix: integration tests, continued dashificiation](https://github.com/dashpay/rust-dashcore/pull/137) — 2 unresolved (2 CodeRabbit) · 418 days stale · ⚠ merge conflict · 🔴 CI failing · 🐢 targets v0.40-dev, untouched 321 days · areas: fallback
  - Top thread: "_🛠️ Refactor suggestion_" — 418 days old
- [dashpay/platform#5523 feat(sdk): add opt-in verified HTTP context provider](https://github.com/dashpay/platform/pull/5523) — 5 unresolved (5 bot) · 7 days stale · 📝 draft · 🐢 targets feat/core-snapshot-proof-verifier · areas: github, fallback
  - Top thread: "✅ **Resolved** at \`78330297\`; see the replies below." — 7 days old
- [dashpay/platform#4633 feat(sdk): add dash-sdk-cxx, a C++ binding of dash-sdk for Dash Core](https://github.com/dashpay/platform/pull/4633) — 📝 draft · 🐢 targets build/vendorable-workspace-lockfile · areas: github, fallback

<a id="pastapastapasta-clean"></a>
#### Clean (2)
- [dashpay/platform#5612 fix(sdk): let wasm-sdk build without its default features](https://github.com/dashpay/platform/pull/5612) — areas: js-wasm-sdk, fallback · Policy: too-many-open-prs
  - Blocker: Human review is still required; automatic reviewer requests are paused until a review slot is available
- [dashpay/rust-dashcore#264 feat(dash-spv): Add BIP324 v2 encrypted P2P transport](https://github.com/dashpay/rust-dashcore/pull/264) — ⚠ merge conflict · areas: dash-spv, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required

<a id="romchornyi"></a>
### @romchornyi
<a id="romchornyi-open"></a>
#### Open (15)
- [dashpay/platform#5536 fix: check a DashPay invitation claim against Platform before broadcasting it](https://github.com/dashpay/platform/pull/5536) — 7 unresolved (7 bot) · 4 days stale · 🔴 CI failing · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, rust-sdk, kotlin-sdk, fallback · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Handle code 63 when the collided key belongs to this wallet**" — 4 days old
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5465 feat(platform-wallet)!: sign a DashPay contact payment before broadcasting it](https://github.com/dashpay/platform/pull/5465) — 5 unresolved (5 bot) · 5 days stale · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk · Policy: waiting-author
  - Top thread: "🔴 Blocking: Order event-derived pool snapshots with payment snapshots**" — 5 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5258 feat(platform-wallet): probe the network about sends with an unknown broadcast outcome](https://github.com/dashpay/platform/pull/5258) — 3 unresolved (3 human) · 12 days stale · ⚠ merge conflict · 🔴 CI failing · 📝 draft · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, rust-sdk · Policy: draft
  - Top thread: "\[P2\] Carry the originating wallet generation in probe events" — 12 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5395 feat(platform-wallet): spend only final coins and report funds waiting on the network](https://github.com/dashpay/platform/pull/5395) — 1 unresolved (1 human) · 12 days stale · ⚠ merge conflict · ✋ changes requested · 📝 draft · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk, fallback · Policy: draft
  - Top thread: "this should belong to another layer - likely platform-wallet, or even rust-dashcore's key-wallet. FFI is not meant to st…" — 12 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5549 fix(platform-wallet-storage): restore DashPay contact address pools on SQLite load](https://github.com/dashpay/platform/pull/5549) — 3 unresolved (3 bot) · 5 days stale · 🔴 CI failing · 📝 draft · areas: wallet-storage · Policy: draft
  - Top thread: "✅ **Withdrawn** at \`2bee2705\`; see the replies below." — 5 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5540 fix(platform-wallet)!: store externally supplied identity keys without a derivation breadcrumb](https://github.com/dashpay/platform/pull/5540) — 1 unresolved (1 bot) · 6 days stale · 📝 draft · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk, fallback · Policy: draft
  - Top thread: "🔴 Blocking: Preserve existing derived-key breadcrumbs on disable upserts**" — 6 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5487 fix(platform-wallet): keep the cached identity in step with the chain around updates](https://github.com/dashpay/platform/pull/5487) — 1 unresolved (1 CodeRabbit) · 0 days stale · areas: rs-platform-wallet, wallet-storage · Policy: waiting-bots
  - Top thread: "&lt;picture&gt;&lt;img src="https://storage.googleapis.com/coderabbit_public_assets/review-comment-icons-v1/amber-16.svg" width="…" — 0 days old
  - Blocker: thepastaclaw has not reported for the current head
  - Blocker: coderabbitai left review threads unresolved; resolve them
- [dashpay/platform#4992 feat(swift-sdk): durable wallet presence marker readable while the device is locked](https://github.com/dashpay/platform/pull/4992) — areas: swift-sdk · Policy: ready-to-merge
  - Blocker: All policy requirements are satisfied
- [dashpay/platform#5309 fix(sdk): treat AlreadyExists on a retried broadcast as submitted](https://github.com/dashpay/platform/pull/5309) — 📝 draft · ⚠ routing unavailable: no changed-file evidence · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5460 fix(platform-wallet): retry an oversized InstantSend asset-lock transition with a ChainLock proof](https://github.com/dashpay/platform/pull/5460) — ✋ changes requested · areas: rs-drive-abci, dpp, rs-platform-wallet, rs-platform-wallet-ffi, rust-sdk, rust-dapi · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
  - Blocker: Proceeded without thepastaclaw: skipped by @romchornyi
- [dashpay/platform#5476 feat(platform-wallet)!: report a freshly built asset lock's outpoint to the caller](https://github.com/dashpay/platform/pull/5476) — areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, fallback · Policy: ready-to-merge
  - Blocker: All policy requirements are satisfied
- [dashpay/platform#5587 fix(platform-wallet)!: keep a contact's payment account across relaunches and replace it cleanly on rotation](https://github.com/dashpay/platform/pull/5587) — ⚠ merge conflict · 🔴 CI failing · 📝 draft · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5589 feat(platform-wallet-ffi)!: let hosts reset an outbound contact account's pool and keep its request digest](https://github.com/dashpay/platform/pull/5589) — 🔴 CI failing · 📝 draft · 🐢 targets fix/contact-rebuild-keeps-pool · areas: rs-platform-wallet-ffi, swift-sdk
- [dashpay/platform#5590 feat(kotlin-sdk)!: reset an outbound contact account's pool and keep its request digest on Android](https://github.com/dashpay/platform/pull/5590) — 🔴 CI failing · 📝 draft · 🐢 targets fix/contact-rebuild-ffi-swift · areas: kotlin-sdk, fallback
- [dashpay/rust-dashcore#1105 feat(key-wallet): extend CoinJoin address pools before the first sync](https://github.com/dashpay/rust-dashcore/pull/1105) — ✋ changes requested · areas: key-wallet · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required

<a id="romchornyi-needs-action"></a>
#### Needs action (5)
- [dashpay/platform#5536 fix: check a DashPay invitation claim against Platform before broadcasting it](https://github.com/dashpay/platform/pull/5536) — 7 unresolved (7 bot) · 4 days stale · 🔴 CI failing · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, rust-sdk, kotlin-sdk, fallback · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Handle code 63 when the collided key belongs to this wallet**" — 4 days old
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5465 feat(platform-wallet)!: sign a DashPay contact payment before broadcasting it](https://github.com/dashpay/platform/pull/5465) — 5 unresolved (5 bot) · 5 days stale · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk · Policy: waiting-author
  - Top thread: "🔴 Blocking: Order event-derived pool snapshots with payment snapshots**" — 5 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5487 fix(platform-wallet): keep the cached identity in step with the chain around updates](https://github.com/dashpay/platform/pull/5487) — 1 unresolved (1 CodeRabbit) · 0 days stale · areas: rs-platform-wallet, wallet-storage · Policy: waiting-bots
  - Top thread: "&lt;picture&gt;&lt;img src="https://storage.googleapis.com/coderabbit_public_assets/review-comment-icons-v1/amber-16.svg" width="…" — 0 days old
  - Blocker: thepastaclaw has not reported for the current head
  - Blocker: coderabbitai left review threads unresolved; resolve them
- [dashpay/platform#5460 fix(platform-wallet): retry an oversized InstantSend asset-lock transition with a ChainLock proof](https://github.com/dashpay/platform/pull/5460) — ✋ changes requested · areas: rs-drive-abci, dpp, rs-platform-wallet, rs-platform-wallet-ffi, rust-sdk, rust-dapi · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
  - Blocker: Proceeded without thepastaclaw: skipped by @romchornyi
- [dashpay/rust-dashcore#1105 feat(key-wallet): extend CoinJoin address pools before the first sync](https://github.com/dashpay/rust-dashcore/pull/1105) — ✋ changes requested · areas: key-wallet · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required

<a id="romchornyi-ready-for-human"></a>
#### Ready for human (4)
- [dashpay/platform#4992 feat(swift-sdk): durable wallet presence marker readable while the device is locked](https://github.com/dashpay/platform/pull/4992) — areas: swift-sdk · Policy: ready-to-merge
  - Blocker: All policy requirements are satisfied
- [dashpay/platform#5460 fix(platform-wallet): retry an oversized InstantSend asset-lock transition with a ChainLock proof](https://github.com/dashpay/platform/pull/5460) — ✋ changes requested · areas: rs-drive-abci, dpp, rs-platform-wallet, rs-platform-wallet-ffi, rust-sdk, rust-dapi · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
  - Blocker: Proceeded without thepastaclaw: skipped by @romchornyi
- [dashpay/platform#5476 feat(platform-wallet)!: report a freshly built asset lock's outpoint to the caller](https://github.com/dashpay/platform/pull/5476) — areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, fallback · Policy: ready-to-merge
  - Blocker: All policy requirements are satisfied
- [dashpay/rust-dashcore#1105 feat(key-wallet): extend CoinJoin address pools before the first sync](https://github.com/dashpay/rust-dashcore/pull/1105) — ✋ changes requested · areas: key-wallet · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required

<a id="romchornyi-unresolved-comments"></a>
#### Unresolved Comments (3)
- [dashpay/platform#5536 fix: check a DashPay invitation claim against Platform before broadcasting it](https://github.com/dashpay/platform/pull/5536) — 7 unresolved (7 bot) · 4 days stale · 🔴 CI failing · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, rust-sdk, kotlin-sdk, fallback · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Handle code 63 when the collided key belongs to this wallet**" — 4 days old
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5465 feat(platform-wallet)!: sign a DashPay contact payment before broadcasting it](https://github.com/dashpay/platform/pull/5465) — 5 unresolved (5 bot) · 5 days stale · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk · Policy: waiting-author
  - Top thread: "🔴 Blocking: Order event-derived pool snapshots with payment snapshots**" — 5 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5487 fix(platform-wallet): keep the cached identity in step with the chain around updates](https://github.com/dashpay/platform/pull/5487) — 1 unresolved (1 CodeRabbit) · 0 days stale · areas: rs-platform-wallet, wallet-storage · Policy: waiting-bots
  - Top thread: "&lt;picture&gt;&lt;img src="https://storage.googleapis.com/coderabbit_public_assets/review-comment-icons-v1/amber-16.svg" width="…" — 0 days old
  - Blocker: thepastaclaw has not reported for the current head
  - Blocker: coderabbitai left review threads unresolved; resolve them

<a id="romchornyi-changes-requested"></a>
#### Changes Requested (2)
- [dashpay/platform#5460 fix(platform-wallet): retry an oversized InstantSend asset-lock transition with a ChainLock proof](https://github.com/dashpay/platform/pull/5460) — ✋ changes requested · areas: rs-drive-abci, dpp, rs-platform-wallet, rs-platform-wallet-ffi, rust-sdk, rust-dapi · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
  - Blocker: Proceeded without thepastaclaw: skipped by @romchornyi
- [dashpay/rust-dashcore#1105 feat(key-wallet): extend CoinJoin address pools before the first sync](https://github.com/dashpay/rust-dashcore/pull/1105) — ✋ changes requested · areas: key-wallet · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required

<a id="romchornyi-draft"></a>
#### Draft (6)
- [dashpay/platform#5258 feat(platform-wallet): probe the network about sends with an unknown broadcast outcome](https://github.com/dashpay/platform/pull/5258) — 3 unresolved (3 human) · 12 days stale · ⚠ merge conflict · 🔴 CI failing · 📝 draft · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, rust-sdk · Policy: draft
  - Top thread: "\[P2\] Carry the originating wallet generation in probe events" — 12 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5395 feat(platform-wallet): spend only final coins and report funds waiting on the network](https://github.com/dashpay/platform/pull/5395) — 1 unresolved (1 human) · 12 days stale · ⚠ merge conflict · ✋ changes requested · 📝 draft · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk, fallback · Policy: draft
  - Top thread: "this should belong to another layer - likely platform-wallet, or even rust-dashcore's key-wallet. FFI is not meant to st…" — 12 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5549 fix(platform-wallet-storage): restore DashPay contact address pools on SQLite load](https://github.com/dashpay/platform/pull/5549) — 3 unresolved (3 bot) · 5 days stale · 🔴 CI failing · 📝 draft · areas: wallet-storage · Policy: draft
  - Top thread: "✅ **Withdrawn** at \`2bee2705\`; see the replies below." — 5 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5540 fix(platform-wallet)!: store externally supplied identity keys without a derivation breadcrumb](https://github.com/dashpay/platform/pull/5540) — 1 unresolved (1 bot) · 6 days stale · 📝 draft · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk, fallback · Policy: draft
  - Top thread: "🔴 Blocking: Preserve existing derived-key breadcrumbs on disable upserts**" — 6 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5309 fix(sdk): treat AlreadyExists on a retried broadcast as submitted](https://github.com/dashpay/platform/pull/5309) — 📝 draft · ⚠ routing unavailable: no changed-file evidence · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5587 fix(platform-wallet)!: keep a contact's payment account across relaunches and replace it cleanly on rotation](https://github.com/dashpay/platform/pull/5587) — ⚠ merge conflict · 🔴 CI failing · 📝 draft · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage · Policy: draft
  - Blocker: Draft PR does not occupy a review slot

<a id="romchornyi-stale"></a>
#### Stale (2)
- [dashpay/platform#5589 feat(platform-wallet-ffi)!: let hosts reset an outbound contact account's pool and keep its request digest](https://github.com/dashpay/platform/pull/5589) — 🔴 CI failing · 📝 draft · 🐢 targets fix/contact-rebuild-keeps-pool · areas: rs-platform-wallet-ffi, swift-sdk
- [dashpay/platform#5590 feat(kotlin-sdk)!: reset an outbound contact account's pool and keep its request digest on Android](https://github.com/dashpay/platform/pull/5590) — 🔴 CI failing · 📝 draft · 🐢 targets fix/contact-rebuild-ffi-swift · areas: kotlin-sdk, fallback

<a id="romchornyi-clean"></a>
#### Clean (2)
- [dashpay/platform#4992 feat(swift-sdk): durable wallet presence marker readable while the device is locked](https://github.com/dashpay/platform/pull/4992) — areas: swift-sdk · Policy: ready-to-merge
  - Blocker: All policy requirements are satisfied
- [dashpay/platform#5476 feat(platform-wallet)!: report a freshly built asset lock's outpoint to the caller](https://github.com/dashpay/platform/pull/5476) — areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, fallback · Policy: ready-to-merge
  - Blocker: All policy requirements are satisfied

<a id="romchornyi-ready-for-review"></a>
#### Ready for Review (2)
- [dashpay/platform#5479 feat(platform-wallet-storage): persist DashPay backfill coverage and outbound-account markers](https://github.com/dashpay/platform/pull/5479) — by @HashEngineering · areas: rs-platform-wallet, wallet-storage · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
- [dashpay/platform#5637 chore(platform)!: update rust-dashcore to dev 7329126e](https://github.com/dashpay/platform/pull/5637) — by @shumkov · areas: dpp, rs-platform-wallet, fallback · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head

<a id="lklimek"></a>
### @lklimek
<a id="lklimek-open"></a>
#### Open (14)
- [dashpay/platform#5528 fix(platform-wallet): retain drain operation history through funding conflicts](https://github.com/dashpay/platform/pull/5528) — 7 unresolved (7 bot) · 6 days stale · ✋ changes requested · 📝 draft · 🐢 targets feat/sqlite-wallet-restore · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk, kotlin-sdk
  - Top thread: "🟡 Suggestion: Retain diagnostics for malformed FFI operation snapshots**" — 6 days old
- [dashpay/platform#5519 refactor: remove DPP raw secp256k1 dependency](https://github.com/dashpay/platform/pull/5519) — 2 unresolved (2 bot) · 7 days stale · ⚠ merge conflict · 📝 draft · areas: rs-drive-abci, dpp, rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, rust-sdk, rust-sdk-ffi, js-wasm-sdk, fallback · Policy: draft
  - Top thread: "🟡 Suggestion: Erase the raw ECDH secret created by the test provider**" — 7 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/tenderdash#1515 fix(service)!: join remaining service workers before shutdown completes](https://github.com/dashpay/tenderdash/pull/1515) — 1 unresolved (1 bot) · 5 days stale · ✋ changes requested · areas: fallback · Policy: waiting-author
  - Top thread: "🔴 Blocking: Release node-owned resources when startup fails**" — 5 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/tenderdash#1550 fix(p2p)!: enforce peer penalties and validate network settings](https://github.com/dashpay/tenderdash/pull/1550) — 1 unresolved (1 bot) · 5 days stale · ✋ changes requested · areas: fallback · Policy: waiting-author
  - Top thread: "🔴 Blocking: Do not reject max-connections zero before its runtime default is applied**" — 5 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/rust-dashcore#1225 fix: wipe secret key copies in sign_hash and PrivateKey::from_wif](https://github.com/dashpay/rust-dashcore/pull/1225) — 1 unresolved (1 CodeRabbit) · 0 days stale · ✋ changes requested · areas: fallback · Policy: waiting-author
  - Top thread: "&lt;picture&gt;&lt;img src="https://storage.googleapis.com/coderabbit_public_assets/review-comment-icons-v1/major-16.svg" width="…" — 0 days old
  - Blocker: coderabbitai requested changes on this head; dismiss the review or push a fix
  - Blocker: coderabbitai left review threads unresolved; resolve them
- [dashpay/platform#5150 fix(platform-wallet)!: restore Core spending state and repair persisted accounting](https://github.com/dashpay/platform/pull/5150) — ⏸ deferred · areas: rs-platform-wallet, wallet-storage, fallback · Policy: waiting-author
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5207 fix(platform-wallet): require only atomic tracked-lock writes for reconciliation](https://github.com/dashpay/platform/pull/5207) — ⏸ deferred · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage
- [dashpay/platform#5210 feat(wallet-storage)!: restore complete Core wallet snapshots from SQLite](https://github.com/dashpay/platform/pull/5210) — ⏸ deferred · areas: rs-platform-wallet, wallet-storage
- [dashpay/platform#5220 fix(platform-wallet)!: replay recorded transaction history on load for every persister](https://github.com/dashpay/platform/pull/5220) — ⏸ deferred · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk, fallback
- [dashpay/platform#5323 refactor: reject legacy BLS storage formats](https://github.com/dashpay/platform/pull/5323) — ⚠ merge conflict · 📝 draft · areas: rs-drive-abci · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5489 fix!: set zero consensus evidence budget at protocol 15](https://github.com/dashpay/platform/pull/5489) — ⚠ merge conflict · 📝 draft · areas: rs-drive-abci, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/tenderdash#1552 chore(release)!: prepare dev branch and Platform release workflow](https://github.com/dashpay/tenderdash/pull/1552) — 📝 draft · areas: github, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/tenderdash#1560 test(e2e): exercise block sync during slow local application](https://github.com/dashpay/tenderdash/pull/1560) — areas: github, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 1f5fb2540ca89b3c6b2bb311ad149affdd577df4
- [dashpay/tenderdash#1567 fix(consensus): replay blocks with their historical quorum](https://github.com/dashpay/tenderdash/pull/1567) — 📝 draft · areas: fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot

<a id="lklimek-needs-action"></a>
#### Needs action (3)
- [dashpay/tenderdash#1515 fix(service)!: join remaining service workers before shutdown completes](https://github.com/dashpay/tenderdash/pull/1515) — 1 unresolved (1 bot) · 5 days stale · ✋ changes requested · areas: fallback · Policy: waiting-author
  - Top thread: "🔴 Blocking: Release node-owned resources when startup fails**" — 5 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/tenderdash#1550 fix(p2p)!: enforce peer penalties and validate network settings](https://github.com/dashpay/tenderdash/pull/1550) — 1 unresolved (1 bot) · 5 days stale · ✋ changes requested · areas: fallback · Policy: waiting-author
  - Top thread: "🔴 Blocking: Do not reject max-connections zero before its runtime default is applied**" — 5 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/rust-dashcore#1225 fix: wipe secret key copies in sign_hash and PrivateKey::from_wif](https://github.com/dashpay/rust-dashcore/pull/1225) — 1 unresolved (1 CodeRabbit) · 0 days stale · ✋ changes requested · areas: fallback · Policy: waiting-author
  - Top thread: "&lt;picture&gt;&lt;img src="https://storage.googleapis.com/coderabbit_public_assets/review-comment-icons-v1/major-16.svg" width="…" — 0 days old
  - Blocker: coderabbitai requested changes on this head; dismiss the review or push a fix
  - Blocker: coderabbitai left review threads unresolved; resolve them

<a id="lklimek-unresolved-comments"></a>
#### Unresolved Comments (3)
- [dashpay/tenderdash#1515 fix(service)!: join remaining service workers before shutdown completes](https://github.com/dashpay/tenderdash/pull/1515) — 1 unresolved (1 bot) · 5 days stale · ✋ changes requested · areas: fallback · Policy: waiting-author
  - Top thread: "🔴 Blocking: Release node-owned resources when startup fails**" — 5 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/tenderdash#1550 fix(p2p)!: enforce peer penalties and validate network settings](https://github.com/dashpay/tenderdash/pull/1550) — 1 unresolved (1 bot) · 5 days stale · ✋ changes requested · areas: fallback · Policy: waiting-author
  - Top thread: "🔴 Blocking: Do not reject max-connections zero before its runtime default is applied**" — 5 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/rust-dashcore#1225 fix: wipe secret key copies in sign_hash and PrivateKey::from_wif](https://github.com/dashpay/rust-dashcore/pull/1225) — 1 unresolved (1 CodeRabbit) · 0 days stale · ✋ changes requested · areas: fallback · Policy: waiting-author
  - Top thread: "&lt;picture&gt;&lt;img src="https://storage.googleapis.com/coderabbit_public_assets/review-comment-icons-v1/major-16.svg" width="…" — 0 days old
  - Blocker: coderabbitai requested changes on this head; dismiss the review or push a fix
  - Blocker: coderabbitai left review threads unresolved; resolve them

<a id="lklimek-deferred"></a>
#### Deferred (4)
- [dashpay/platform#5150 fix(platform-wallet)!: restore Core spending state and repair persisted accounting](https://github.com/dashpay/platform/pull/5150) — ⏸ deferred · areas: rs-platform-wallet, wallet-storage, fallback · Policy: waiting-author
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5207 fix(platform-wallet): require only atomic tracked-lock writes for reconciliation](https://github.com/dashpay/platform/pull/5207) — ⏸ deferred · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage
- [dashpay/platform#5210 feat(wallet-storage)!: restore complete Core wallet snapshots from SQLite](https://github.com/dashpay/platform/pull/5210) — ⏸ deferred · areas: rs-platform-wallet, wallet-storage
- [dashpay/platform#5220 fix(platform-wallet)!: replay recorded transaction history on load for every persister](https://github.com/dashpay/platform/pull/5220) — ⏸ deferred · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk, fallback

<a id="lklimek-draft"></a>
#### Draft (5)
- [dashpay/platform#5519 refactor: remove DPP raw secp256k1 dependency](https://github.com/dashpay/platform/pull/5519) — 2 unresolved (2 bot) · 7 days stale · ⚠ merge conflict · 📝 draft · areas: rs-drive-abci, dpp, rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, rust-sdk, rust-sdk-ffi, js-wasm-sdk, fallback · Policy: draft
  - Top thread: "🟡 Suggestion: Erase the raw ECDH secret created by the test provider**" — 7 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5323 refactor: reject legacy BLS storage formats](https://github.com/dashpay/platform/pull/5323) — ⚠ merge conflict · 📝 draft · areas: rs-drive-abci · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5489 fix!: set zero consensus evidence budget at protocol 15](https://github.com/dashpay/platform/pull/5489) — ⚠ merge conflict · 📝 draft · areas: rs-drive-abci, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/tenderdash#1552 chore(release)!: prepare dev branch and Platform release workflow](https://github.com/dashpay/tenderdash/pull/1552) — 📝 draft · areas: github, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/tenderdash#1567 fix(consensus): replay blocks with their historical quorum](https://github.com/dashpay/tenderdash/pull/1567) — 📝 draft · areas: fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot

<a id="lklimek-stale"></a>
#### Stale (1)
- [dashpay/platform#5528 fix(platform-wallet): retain drain operation history through funding conflicts](https://github.com/dashpay/platform/pull/5528) — 7 unresolved (7 bot) · 6 days stale · ✋ changes requested · 📝 draft · 🐢 targets feat/sqlite-wallet-restore · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk, kotlin-sdk
  - Top thread: "🟡 Suggestion: Retain diagnostics for malformed FFI operation snapshots**" — 6 days old

<a id="lklimek-clean"></a>
#### Clean (1)
- [dashpay/tenderdash#1560 test(e2e): exercise block sync during slow local application](https://github.com/dashpay/tenderdash/pull/1560) — areas: github, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 1f5fb2540ca89b3c6b2bb311ad149affdd577df4

<a id="lklimek-ready-for-review"></a>
#### Ready for Review (16)
- [dashpay/dash-evo-tool#1110 feat(identity): use proven owner balances and keep identity balances ordered by block height](https://github.com/dashpay/dash-evo-tool/pull/1110) — by @Claudius-Maginificent · areas: fallback · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/dash-evo-tool#1130 fix(tokens): keep the Token Creator form and build exactly the preset it shows](https://github.com/dashpay/dash-evo-tool/pull/1130) — by @Claudius-Maginificent · areas: fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
  - Blocker: Proceeded without thepastaclaw: no review within the configured window
- [dashpay/dash-evo-tool#1143 docs: bring user stories in line with the shipped send, contact and Platform Info behaviour](https://github.com/dashpay/dash-evo-tool/pull/1143) — by @Claudius-Maginificent · areas: fallback · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/dash-evo-tool#1145 fix(identity): show contacts loading and open each contact's details at the top](https://github.com/dashpay/dash-evo-tool/pull/1145) — by @Claudius-Maginificent · areas: fallback · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/dash-evo-tool#1147 fix(wallet): show the private-address refusal before Send, and keep suggestions under dialogs](https://github.com/dashpay/dash-evo-tool/pull/1147) — by @Claudius-Maginificent · areas: fallback · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/dash-evo-tool#1149 fix(identity): refresh the identity's balance after a paid operation](https://github.com/dashpay/dash-evo-tool/pull/1149) — by @Claudius-Maginificent · areas: fallback · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/dash-evo-tool#1150 fix(security): wipe parsed private-key copies from memory](https://github.com/dashpay/dash-evo-tool/pull/1150) — by @Claudius-Maginificent · areas: fallback · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/dash-evo-tool#1151 fix(security): wipe private keys parsed on the add-key, import and new-wallet screens](https://github.com/dashpay/dash-evo-tool/pull/1151) — by @Claudius-Maginificent · areas: fallback · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/dash-evo-tool#1153 chore(deps): bump dashpay/platform to v5.1-dev 0d91c133](https://github.com/dashpay/dash-evo-tool/pull/1153) — by @Claudius-Maginificent · areas: fallback · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/dash-evo-tool#1154 fix(network): block the app while a network switch is in progress](https://github.com/dashpay/dash-evo-tool/pull/1154) — by @Claudius-Maginificent · areas: fallback · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/dash-evo-tool#1155 fix(documents): drop fetched state from a previous contract or type](https://github.com/dashpay/dash-evo-tool/pull/1155) — by @Claudius-Maginificent · areas: fallback · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head
- [dashpay/platform#5526 feat(platform)!: token marketplace escrow trees and token market contract (PV15)](https://github.com/dashpay/platform/pull/5526) — by @QuantumExplorer · areas: rs-drive, rs-drive-abci, dpp, rust-sdk, rust-sdk-ffi, js-wasm-sdk, system-contracts, github, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed eb5ddbebe2aef913cbda17ab2e9a780b94b9326f
- [dashpay/platform#5615 fix(sdk): refuse a non-unique identity V1 request before protocol 14 is proven](https://github.com/dashpay/platform/pull/5615) — by @shumkov · areas: rust-sdk, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 587ba0843d4104b65772b9341461eafd8a057f0f
- [dashpay/platform#5635 fix(sdk)!: refuse document requests no node can answer instead of banning nodes](https://github.com/dashpay/platform/pull/5635) — by @shumkov · areas: rs-drive, rs-drive-abci, rust-sdk, rust-sdk-ffi, js-wasm-sdk, github, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 771e06ec7fc942639e36774aa152c783c8cf0f41
- [dashpay/platform#5636 feat(sdk)!: decide from the version tables which networks serve a document query](https://github.com/dashpay/platform/pull/5636) — by @shumkov · areas: rs-drive, rs-drive-abci, rust-sdk, rust-sdk-ffi, js-wasm-sdk, github, fallback · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head
- [dashpay/platform#5638 fix(sdk): check document windows and ranked indexes before sending](https://github.com/dashpay/platform/pull/5638) — by @shumkov · areas: rs-drive, rs-drive-abci, rust-sdk, rust-sdk-ffi, js-wasm-sdk, github, fallback · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head

<a id="bfoss765"></a>
### @bfoss765
<a id="bfoss765-open"></a>
#### Open (2)
- [dashpay/platform#4313 feat(kotlin-sdk): one-time Orchard key shielded-invite API (inviter + claim)](https://github.com/dashpay/platform/pull/4313) — 2 unresolved (2 bot) · 58 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk, fallback · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Preserve claim-binding mismatch semantics across the FFI boundary**" — 58 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#4312 feat(platform-wallet): multi-output shielded transfers + output-aware fee predictor](https://github.com/dashpay/platform/pull/4312) — 1 unresolved (1 bot) · 58 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · areas: dpp, rs-platform-wallet, rs-platform-wallet-ffi, kotlin-sdk, fallback · Policy: waiting-self-review
  - Top thread: "🟡 Suggestion: Keep envelope-aware action counting behind the crate boundary**" — 58 days old
  - Blocker: Author must post /self-reviewed da932f96a8670c641065954de8b58752e99233a3

<a id="bfoss765-needs-action"></a>
#### Needs action (2)
- [dashpay/platform#4313 feat(kotlin-sdk): one-time Orchard key shielded-invite API (inviter + claim)](https://github.com/dashpay/platform/pull/4313) — 2 unresolved (2 bot) · 58 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk, fallback · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Preserve claim-binding mismatch semantics across the FFI boundary**" — 58 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#4312 feat(platform-wallet): multi-output shielded transfers + output-aware fee predictor](https://github.com/dashpay/platform/pull/4312) — 1 unresolved (1 bot) · 58 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · areas: dpp, rs-platform-wallet, rs-platform-wallet-ffi, kotlin-sdk, fallback · Policy: waiting-self-review
  - Top thread: "🟡 Suggestion: Keep envelope-aware action counting behind the crate boundary**" — 58 days old
  - Blocker: Author must post /self-reviewed da932f96a8670c641065954de8b58752e99233a3

<a id="bfoss765-unresolved-comments"></a>
#### Unresolved Comments (2)
- [dashpay/platform#4313 feat(kotlin-sdk): one-time Orchard key shielded-invite API (inviter + claim)](https://github.com/dashpay/platform/pull/4313) — 2 unresolved (2 bot) · 58 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk, fallback · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Preserve claim-binding mismatch semantics across the FFI boundary**" — 58 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#4312 feat(platform-wallet): multi-output shielded transfers + output-aware fee predictor](https://github.com/dashpay/platform/pull/4312) — 1 unresolved (1 bot) · 58 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · areas: dpp, rs-platform-wallet, rs-platform-wallet-ffi, kotlin-sdk, fallback · Policy: waiting-self-review
  - Top thread: "🟡 Suggestion: Keep envelope-aware action counting behind the crate boundary**" — 58 days old
  - Blocker: Author must post /self-reviewed da932f96a8670c641065954de8b58752e99233a3

<a id="hashengineering"></a>
### @HashEngineering
<a id="hashengineering-open"></a>
#### Open (3)
- [dashpay/platform#5259 fix(kotlin-sdk): port #4638 to Android — credit verdicts at save time and the post-sync TXO reconcile](https://github.com/dashpay/platform/pull/5259) — 15 unresolved (2 CodeRabbit, 13 bot) · 18 days stale · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk, fallback · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Add an index matching the unspent pagination query**" — 18 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: coderabbitai left review threads unresolved; resolve them
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5256 fix(platform-wallet): build our receiving account for one-way DashPay contacts](https://github.com/dashpay/platform/pull/5256) — ⚠ merge conflict · 🔴 CI failing · areas: rs-platform-wallet · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
- [dashpay/platform#5479 feat(platform-wallet-storage): persist DashPay backfill coverage and outbound-account markers](https://github.com/dashpay/platform/pull/5479) — areas: rs-platform-wallet, wallet-storage · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required

<a id="hashengineering-needs-action"></a>
#### Needs action (2)
- [dashpay/platform#5259 fix(kotlin-sdk): port #4638 to Android — credit verdicts at save time and the post-sync TXO reconcile](https://github.com/dashpay/platform/pull/5259) — 15 unresolved (2 CodeRabbit, 13 bot) · 18 days stale · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk, fallback · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Add an index matching the unspent pagination query**" — 18 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: coderabbitai left review threads unresolved; resolve them
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5256 fix(platform-wallet): build our receiving account for one-way DashPay contacts](https://github.com/dashpay/platform/pull/5256) — ⚠ merge conflict · 🔴 CI failing · areas: rs-platform-wallet · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required

<a id="hashengineering-ready-for-human"></a>
#### Ready for human (2)
- [dashpay/platform#5256 fix(platform-wallet): build our receiving account for one-way DashPay contacts](https://github.com/dashpay/platform/pull/5256) — ⚠ merge conflict · 🔴 CI failing · areas: rs-platform-wallet · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
- [dashpay/platform#5479 feat(platform-wallet-storage): persist DashPay backfill coverage and outbound-account markers](https://github.com/dashpay/platform/pull/5479) — areas: rs-platform-wallet, wallet-storage · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required

<a id="hashengineering-unresolved-comments"></a>
#### Unresolved Comments (1)
- [dashpay/platform#5259 fix(kotlin-sdk): port #4638 to Android — credit verdicts at save time and the post-sync TXO reconcile](https://github.com/dashpay/platform/pull/5259) — 15 unresolved (2 CodeRabbit, 13 bot) · 18 days stale · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk, fallback · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Add an index matching the unspent pagination query**" — 18 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: coderabbitai left review threads unresolved; resolve them
  - Blocker: thepastaclaw left review threads unresolved; resolve them

<a id="hashengineering-ci-failing"></a>
#### CI Failing (1)
- [dashpay/platform#5256 fix(platform-wallet): build our receiving account for one-way DashPay contacts](https://github.com/dashpay/platform/pull/5256) — ⚠ merge conflict · 🔴 CI failing · areas: rs-platform-wallet · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required

<a id="hashengineering-clean"></a>
#### Clean (1)
- [dashpay/platform#5479 feat(platform-wallet-storage): persist DashPay backfill coverage and outbound-account markers](https://github.com/dashpay/platform/pull/5479) — areas: rs-platform-wallet, wallet-storage · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required

<a id="hashengineering-ready-for-review"></a>
#### Ready for Review (2)
- [dashpay/platform#5476 feat(platform-wallet)!: report a freshly built asset lock's outpoint to the caller](https://github.com/dashpay/platform/pull/5476) — by @romchornyi · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, fallback · Policy: ready-to-merge
  - Blocker: All policy requirements are satisfied
- [dashpay/platform#5637 chore(platform)!: update rust-dashcore to dev 7329126e](https://github.com/dashpay/platform/pull/5637) — by @shumkov · areas: dpp, rs-platform-wallet, fallback · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head

<a id="xdustinface"></a>
### @xdustinface
<a id="xdustinface-open"></a>
#### Open (9)
- [dashpay/rust-dashcore#856 feat(dash-spv): force resync to re-anchor a running client at a lower checkpoint](https://github.com/dashpay/rust-dashcore/pull/856) — 1 unresolved (1 human) · 99 days stale · ⚠ merge conflict · 🔴 CI failing · areas: dash-spv · Policy: waiting-self-review
  - Top thread: "why do we have a periodic check here??" — 99 days old
  - Blocker: Author must post /self-reviewed 7282172a3bb4a0d9d85a40cf70b4dd111995f96b
- [dashpay/rust-dashcore#400 chore: remove BIP141 (SegWit) and BIP341 (Taproot) support](https://github.com/dashpay/rust-dashcore/pull/400) — ⚠ merge conflict · 🔴 CI failing · 📝 draft · 🐢 untouched 161 days · areas: dash-spv, key-wallet, github, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#541 ci: integrate Codecov test analytics via \`cargo-nextest\`](https://github.com/dashpay/rust-dashcore/pull/541) — ⚠ merge conflict · 📝 draft · areas: github, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#720 fix(dash-spv): split peer \`TcpStream\` and add per-peer writer task](https://github.com/dashpay/rust-dashcore/pull/720) — ⚠ merge conflict · 🔴 CI failing · 📝 draft · 🐢 untouched 161 days · areas: dash-spv, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#776 chore: remove FFI layer](https://github.com/dashpay/rust-dashcore/pull/776) — ⚠ merge conflict · 📝 draft · areas: dash-spv, key-wallet, github, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#799 feat: validate masternode list merkle root](https://github.com/dashpay/rust-dashcore/pull/799) — ⚠ merge conflict · 📝 draft · areas: fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#803 feat(dash-spv): block locator + staged fork detection](https://github.com/dashpay/rust-dashcore/pull/803) — ⚠ merge conflict · 📝 draft · areas: dash-spv, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#849 feat(dash-spv): add \`--birth-height\` CLI flag](https://github.com/dashpay/rust-dashcore/pull/849) — areas: dash-spv · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 9b1e709fd8e0bfab0dd69f7c86fd6b7d2e585fbe
- [dashpay/rust-dashcore#1227 refactor(dash-spv): snapshot the masternode engine without copying shared entries](https://github.com/dashpay/rust-dashcore/pull/1227) — ✋ changes requested · 🔴 CI failing · areas: dash-spv · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 0acde6fa6fbd6b26b1fb7d35b42ece48f131d7ce

<a id="xdustinface-needs-action"></a>
#### Needs action (2)
- [dashpay/rust-dashcore#856 feat(dash-spv): force resync to re-anchor a running client at a lower checkpoint](https://github.com/dashpay/rust-dashcore/pull/856) — 1 unresolved (1 human) · 99 days stale · ⚠ merge conflict · 🔴 CI failing · areas: dash-spv · Policy: waiting-self-review
  - Top thread: "why do we have a periodic check here??" — 99 days old
  - Blocker: Author must post /self-reviewed 7282172a3bb4a0d9d85a40cf70b4dd111995f96b
- [dashpay/rust-dashcore#1227 refactor(dash-spv): snapshot the masternode engine without copying shared entries](https://github.com/dashpay/rust-dashcore/pull/1227) — ✋ changes requested · 🔴 CI failing · areas: dash-spv · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 0acde6fa6fbd6b26b1fb7d35b42ece48f131d7ce

<a id="xdustinface-unresolved-comments"></a>
#### Unresolved Comments (1)
- [dashpay/rust-dashcore#856 feat(dash-spv): force resync to re-anchor a running client at a lower checkpoint](https://github.com/dashpay/rust-dashcore/pull/856) — 1 unresolved (1 human) · 99 days stale · ⚠ merge conflict · 🔴 CI failing · areas: dash-spv · Policy: waiting-self-review
  - Top thread: "why do we have a periodic check here??" — 99 days old
  - Blocker: Author must post /self-reviewed 7282172a3bb4a0d9d85a40cf70b4dd111995f96b

<a id="xdustinface-changes-requested"></a>
#### Changes Requested (1)
- [dashpay/rust-dashcore#1227 refactor(dash-spv): snapshot the masternode engine without copying shared entries](https://github.com/dashpay/rust-dashcore/pull/1227) — ✋ changes requested · 🔴 CI failing · areas: dash-spv · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 0acde6fa6fbd6b26b1fb7d35b42ece48f131d7ce

<a id="xdustinface-draft"></a>
#### Draft (4)
- [dashpay/rust-dashcore#541 ci: integrate Codecov test analytics via \`cargo-nextest\`](https://github.com/dashpay/rust-dashcore/pull/541) — ⚠ merge conflict · 📝 draft · areas: github, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#776 chore: remove FFI layer](https://github.com/dashpay/rust-dashcore/pull/776) — ⚠ merge conflict · 📝 draft · areas: dash-spv, key-wallet, github, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#799 feat: validate masternode list merkle root](https://github.com/dashpay/rust-dashcore/pull/799) — ⚠ merge conflict · 📝 draft · areas: fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#803 feat(dash-spv): block locator + staged fork detection](https://github.com/dashpay/rust-dashcore/pull/803) — ⚠ merge conflict · 📝 draft · areas: dash-spv, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot

<a id="xdustinface-stale"></a>
#### Stale (2)
- [dashpay/rust-dashcore#400 chore: remove BIP141 (SegWit) and BIP341 (Taproot) support](https://github.com/dashpay/rust-dashcore/pull/400) — ⚠ merge conflict · 🔴 CI failing · 📝 draft · 🐢 untouched 161 days · areas: dash-spv, key-wallet, github, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#720 fix(dash-spv): split peer \`TcpStream\` and add per-peer writer task](https://github.com/dashpay/rust-dashcore/pull/720) — ⚠ merge conflict · 🔴 CI failing · 📝 draft · 🐢 untouched 161 days · areas: dash-spv, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot

<a id="xdustinface-clean"></a>
#### Clean (1)
- [dashpay/rust-dashcore#849 feat(dash-spv): add \`--birth-height\` CLI flag](https://github.com/dashpay/rust-dashcore/pull/849) — areas: dash-spv · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 9b1e709fd8e0bfab0dd69f7c86fd6b7d2e585fbe

<a id="xdustinface-ready-for-review"></a>
#### Ready for Review (2)
- [dashpay/rust-dashcore#1223 feat(dash-network)!: source parameters from \`dash-params\`, bump Base SDK crates to dashpay/base-sdk@89828032](https://github.com/dashpay/rust-dashcore/pull/1223) — by @kwvg · areas: dash-spv, key-wallet, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
- [dashpay/rust-dashcore#1226 feat(dash): check address maps against Core v24 port and duplicate rules](https://github.com/dashpay/rust-dashcore/pull/1226) — by @shumkov · areas: fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 05b08aa8c2fa3aa44a3235ffeec9de7fb3f0324b

<a id="infraclaw-dash"></a>
### @infraclaw-dash
<a id="infraclaw-dash-open"></a>
#### Open (3)
- [dashpay/platform#5167 ci: align secp feature-base workflows with provisioned runner images](https://github.com/dashpay/platform/pull/5167) — 3 unresolved (3 bot) · 29 days stale · ⚠ merge conflict · 🐢 targets chore/bump-rust-dashcore-secp-033 · areas: github
  - Top thread: "🟡 Suggestion: Contract-failure error points to a doc file missing from this branch**" — 29 days old
- [dashpay/platform#3958 ci: add Platform testnet sync status reporting](https://github.com/dashpay/platform/pull/3958) — 2 unresolved (2 bot) · 126 days stale · ⚠ merge conflict · 🔴 CI failing · areas: github, fallback · Policy: waiting-author
  - Top thread: "🟡 Suggestion: workflow_dispatch lets any repo writer forge a sync status**" — 126 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: coderabbitai left review threads unresolved; resolve them
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5166 ci: adopt verified Rust and Kotlin runner images on v4.3](https://github.com/dashpay/platform/pull/5166) — ⚠ merge conflict · areas: github · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required

<a id="infraclaw-dash-needs-action"></a>
#### Needs action (2)
- [dashpay/platform#3958 ci: add Platform testnet sync status reporting](https://github.com/dashpay/platform/pull/3958) — 2 unresolved (2 bot) · 126 days stale · ⚠ merge conflict · 🔴 CI failing · areas: github, fallback · Policy: waiting-author
  - Top thread: "🟡 Suggestion: workflow_dispatch lets any repo writer forge a sync status**" — 126 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: coderabbitai left review threads unresolved; resolve them
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5166 ci: adopt verified Rust and Kotlin runner images on v4.3](https://github.com/dashpay/platform/pull/5166) — ⚠ merge conflict · areas: github · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required

<a id="infraclaw-dash-ready-for-human"></a>
#### Ready for human (1)
- [dashpay/platform#5166 ci: adopt verified Rust and Kotlin runner images on v4.3](https://github.com/dashpay/platform/pull/5166) — ⚠ merge conflict · areas: github · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required

<a id="infraclaw-dash-unresolved-comments"></a>
#### Unresolved Comments (1)
- [dashpay/platform#3958 ci: add Platform testnet sync status reporting](https://github.com/dashpay/platform/pull/3958) — 2 unresolved (2 bot) · 126 days stale · ⚠ merge conflict · 🔴 CI failing · areas: github, fallback · Policy: waiting-author
  - Top thread: "🟡 Suggestion: workflow_dispatch lets any repo writer forge a sync status**" — 126 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: coderabbitai left review threads unresolved; resolve them
  - Blocker: thepastaclaw left review threads unresolved; resolve them

<a id="infraclaw-dash-stale"></a>
#### Stale (1)
- [dashpay/platform#5167 ci: align secp feature-base workflows with provisioned runner images](https://github.com/dashpay/platform/pull/5167) — 3 unresolved (3 bot) · 29 days stale · ⚠ merge conflict · 🐢 targets chore/bump-rust-dashcore-secp-033 · areas: github
  - Top thread: "🟡 Suggestion: Contract-failure error points to a doc file missing from this branch**" — 29 days old

<a id="infraclaw-dash-clean"></a>
#### Clean (1)
- [dashpay/platform#5166 ci: adopt verified Rust and Kotlin runner images on v4.3](https://github.com/dashpay/platform/pull/5166) — ⚠ merge conflict · areas: github · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required

<a id="llbartekll"></a>
### @llbartekll
<a id="llbartekll-open"></a>
#### Open (2)
- [dashpay/platform#5556 fix(sdk)!: preserve uncertain submission results for recovery](https://github.com/dashpay/platform/pull/5556) — 4 unresolved (4 bot) · 1 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, rust-sdk, rust-sdk-ffi, js-wasm-sdk, kotlin-sdk, fallback · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Gate new submission tests with the mocks feature**" — 1 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5639 chore(platform)!: update rust-dashcore to dev 7329126e](https://github.com/dashpay/platform/pull/5639) — 🔴 CI failing · 📝 draft · areas: rs-platform-wallet, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot

<a id="llbartekll-needs-action"></a>
#### Needs action (1)
- [dashpay/platform#5556 fix(sdk)!: preserve uncertain submission results for recovery](https://github.com/dashpay/platform/pull/5556) — 4 unresolved (4 bot) · 1 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, rust-sdk, rust-sdk-ffi, js-wasm-sdk, kotlin-sdk, fallback · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Gate new submission tests with the mocks feature**" — 1 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them

<a id="llbartekll-unresolved-comments"></a>
#### Unresolved Comments (1)
- [dashpay/platform#5556 fix(sdk)!: preserve uncertain submission results for recovery](https://github.com/dashpay/platform/pull/5556) — 4 unresolved (4 bot) · 1 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, rust-sdk, rust-sdk-ffi, js-wasm-sdk, kotlin-sdk, fallback · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Gate new submission tests with the mocks feature**" — 1 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them

<a id="llbartekll-draft"></a>
#### Draft (1)
- [dashpay/platform#5639 chore(platform)!: update rust-dashcore to dev 7329126e](https://github.com/dashpay/platform/pull/5639) — 🔴 CI failing · 📝 draft · areas: rs-platform-wallet, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot

<a id="llbartekll-ready-for-review"></a>
#### Ready for Review (3)
- [dashpay/platform#4992 feat(swift-sdk): durable wallet presence marker readable while the device is locked](https://github.com/dashpay/platform/pull/4992) — by @romchornyi · areas: swift-sdk · Policy: ready-to-merge
  - Blocker: All policy requirements are satisfied
- [dashpay/platform#5479 feat(platform-wallet-storage): persist DashPay backfill coverage and outbound-account markers](https://github.com/dashpay/platform/pull/5479) — by @HashEngineering · areas: rs-platform-wallet, wallet-storage · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
- [dashpay/platform#5637 chore(platform)!: update rust-dashcore to dev 7329126e](https://github.com/dashpay/platform/pull/5637) — by @shumkov · areas: dpp, rs-platform-wallet, fallback · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head

<a id="ktechmidas"></a>
### @ktechmidas
<a id="ktechmidas-open"></a>
#### Open (6)
- [dashpay/platform#5188 ci: tolerate optional S3 cache export outages on chore/bump-rust-dashcore-secp-033](https://github.com/dashpay/platform/pull/5188) — 1 unresolved (1 bot) · 29 days stale · 🐢 targets chore/bump-rust-dashcore-secp-033 · areas: github
  - Top thread: "🟡 Suggestion: New regression tests are not executed by any CI workflow on this branch**" — 29 days old
- [dashpay/platform#5186 ci: tolerate optional S3 cache export outages on v5.0-dev](https://github.com/dashpay/platform/pull/5186) — 🔴 CI failing · 🐢 targets v6.0-dev · areas: github · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 843052c2dce26a80065c15b1219bbc8211264161
- [dashpay/platform#5187 ci: tolerate optional S3 cache export outages on v4.3-dev](https://github.com/dashpay/platform/pull/5187) — areas: github · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed fbbf13ee41f8bd8b9f03f1dc5afec6a619967d97
- [dashpay/platform#5198 ci: bound default Binaryen optimizer threads in NPM builds](https://github.com/dashpay/platform/pull/5198) — 🐢 targets v6.0-dev · areas: github · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 23b41b477bc038965b686b4a75412b1c21fc6d98
- [dashpay/platform#5204 fix(release): install verified cargo-binstall binary directly](https://github.com/dashpay/platform/pull/5204) — ⚠ merge conflict · areas: fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 63455a2563fd7271fb11083d3b0a89bdf01f3982
- [dashpay/platform#5205 fix(release): install verified cargo-binstall binary directly](https://github.com/dashpay/platform/pull/5205) — 🔴 CI failing · 🐢 targets v6.0-dev · areas: fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 8a8ac2c788b5338416e18045001d9115ba3ac80a

<a id="ktechmidas-needs-action"></a>
#### Needs action (1)
- [dashpay/platform#5204 fix(release): install verified cargo-binstall binary directly](https://github.com/dashpay/platform/pull/5204) — ⚠ merge conflict · areas: fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 63455a2563fd7271fb11083d3b0a89bdf01f3982

<a id="ktechmidas-stale"></a>
#### Stale (4)
- [dashpay/platform#5188 ci: tolerate optional S3 cache export outages on chore/bump-rust-dashcore-secp-033](https://github.com/dashpay/platform/pull/5188) — 1 unresolved (1 bot) · 29 days stale · 🐢 targets chore/bump-rust-dashcore-secp-033 · areas: github
  - Top thread: "🟡 Suggestion: New regression tests are not executed by any CI workflow on this branch**" — 29 days old
- [dashpay/platform#5186 ci: tolerate optional S3 cache export outages on v5.0-dev](https://github.com/dashpay/platform/pull/5186) — 🔴 CI failing · 🐢 targets v6.0-dev · areas: github · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 843052c2dce26a80065c15b1219bbc8211264161
- [dashpay/platform#5198 ci: bound default Binaryen optimizer threads in NPM builds](https://github.com/dashpay/platform/pull/5198) — 🐢 targets v6.0-dev · areas: github · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 23b41b477bc038965b686b4a75412b1c21fc6d98
- [dashpay/platform#5205 fix(release): install verified cargo-binstall binary directly](https://github.com/dashpay/platform/pull/5205) — 🔴 CI failing · 🐢 targets v6.0-dev · areas: fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 8a8ac2c788b5338416e18045001d9115ba3ac80a

<a id="ktechmidas-clean"></a>
#### Clean (2)
- [dashpay/platform#5187 ci: tolerate optional S3 cache export outages on v4.3-dev](https://github.com/dashpay/platform/pull/5187) — areas: github · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed fbbf13ee41f8bd8b9f03f1dc5afec6a619967d97
- [dashpay/platform#5204 fix(release): install verified cargo-binstall binary directly](https://github.com/dashpay/platform/pull/5204) — ⚠ merge conflict · areas: fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 63455a2563fd7271fb11083d3b0a89bdf01f3982

<a id="ktechmidas-ready-for-review"></a>
#### Ready for Review (5)
- [dashpay/platform#5526 feat(platform)!: token marketplace escrow trees and token market contract (PV15)](https://github.com/dashpay/platform/pull/5526) — by @QuantumExplorer · areas: rs-drive, rs-drive-abci, dpp, rust-sdk, rust-sdk-ffi, js-wasm-sdk, system-contracts, github, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed eb5ddbebe2aef913cbda17ab2e9a780b94b9326f
- [dashpay/platform#5635 fix(sdk)!: refuse document requests no node can answer instead of banning nodes](https://github.com/dashpay/platform/pull/5635) — by @shumkov · areas: rs-drive, rs-drive-abci, rust-sdk, rust-sdk-ffi, js-wasm-sdk, github, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 771e06ec7fc942639e36774aa152c783c8cf0f41
- [dashpay/platform#5636 feat(sdk)!: decide from the version tables which networks serve a document query](https://github.com/dashpay/platform/pull/5636) — by @shumkov · areas: rs-drive, rs-drive-abci, rust-sdk, rust-sdk-ffi, js-wasm-sdk, github, fallback · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head
- [dashpay/platform#5638 fix(sdk): check document windows and ranked indexes before sending](https://github.com/dashpay/platform/pull/5638) — by @shumkov · areas: rs-drive, rs-drive-abci, rust-sdk, rust-sdk-ffi, js-wasm-sdk, github, fallback · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head
- [dashpay/tenderdash#1560 test(e2e): exercise block sync during slow local application](https://github.com/dashpay/tenderdash/pull/1560) — by @lklimek · areas: github, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 1f5fb2540ca89b3c6b2bb311ad149affdd577df4

<a id="zocolini"></a>
### @ZocoLini
<a id="zocolini-open"></a>
#### Open (7)
- [dashpay/rust-dashcore#921 fix(dash-spv): sync backfill](https://github.com/dashpay/rust-dashcore/pull/921) — ⚠ merge conflict · 📝 draft · areas: dash-spv, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#1081 feat(dash-spv): restore the validated rotation cycles from the replayed engine](https://github.com/dashpay/rust-dashcore/pull/1081) — ⚠ merge conflict · 📝 draft · areas: dash-spv · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#1181 fix(dash-spv)!: bound the MessageDispatcher subscriber queues](https://github.com/dashpay/rust-dashcore/pull/1181) — ⚠ merge conflict · 🔴 CI failing · 📝 draft · areas: dash-spv · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#1189 refactor(dash-spv)!: make the sync module private](https://github.com/dashpay/rust-dashcore/pull/1189) — ⚠ merge conflict · 🔴 CI failing · 📝 draft · areas: dash-spv, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#1199 feat(key-wallet)!: only final coins fund a build](https://github.com/dashpay/rust-dashcore/pull/1199) — ⚠ merge conflict · 📝 draft · areas: dash-spv, key-wallet, key-wallet-manager · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#1200 fix(dash-spv)!: verify MnListDiff against its block's coinbase before applying](https://github.com/dashpay/rust-dashcore/pull/1200) — ✋ changes requested · 🔴 CI failing · areas: dash-spv, fallback · Policy: waiting-bots
  - Blocker: coderabbitai has not reported for the current head
- [dashpay/rust-dashcore#1202 feat(dash-spv): expose the coinbase payload of the newest validated diff](https://github.com/dashpay/rust-dashcore/pull/1202) — ⚠ merge conflict · 🔴 CI failing · 📝 draft · areas: dash-spv, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot

<a id="zocolini-needs-action"></a>
#### Needs action (1)
- [dashpay/rust-dashcore#1200 fix(dash-spv)!: verify MnListDiff against its block's coinbase before applying](https://github.com/dashpay/rust-dashcore/pull/1200) — ✋ changes requested · 🔴 CI failing · areas: dash-spv, fallback · Policy: waiting-bots
  - Blocker: coderabbitai has not reported for the current head

<a id="zocolini-changes-requested"></a>
#### Changes Requested (1)
- [dashpay/rust-dashcore#1200 fix(dash-spv)!: verify MnListDiff against its block's coinbase before applying](https://github.com/dashpay/rust-dashcore/pull/1200) — ✋ changes requested · 🔴 CI failing · areas: dash-spv, fallback · Policy: waiting-bots
  - Blocker: coderabbitai has not reported for the current head

<a id="zocolini-draft"></a>
#### Draft (6)
- [dashpay/rust-dashcore#921 fix(dash-spv): sync backfill](https://github.com/dashpay/rust-dashcore/pull/921) — ⚠ merge conflict · 📝 draft · areas: dash-spv, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#1081 feat(dash-spv): restore the validated rotation cycles from the replayed engine](https://github.com/dashpay/rust-dashcore/pull/1081) — ⚠ merge conflict · 📝 draft · areas: dash-spv · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#1181 fix(dash-spv)!: bound the MessageDispatcher subscriber queues](https://github.com/dashpay/rust-dashcore/pull/1181) — ⚠ merge conflict · 🔴 CI failing · 📝 draft · areas: dash-spv · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#1189 refactor(dash-spv)!: make the sync module private](https://github.com/dashpay/rust-dashcore/pull/1189) — ⚠ merge conflict · 🔴 CI failing · 📝 draft · areas: dash-spv, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#1199 feat(key-wallet)!: only final coins fund a build](https://github.com/dashpay/rust-dashcore/pull/1199) — ⚠ merge conflict · 📝 draft · areas: dash-spv, key-wallet, key-wallet-manager · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#1202 feat(dash-spv): expose the coinbase payload of the newest validated diff](https://github.com/dashpay/rust-dashcore/pull/1202) — ⚠ merge conflict · 🔴 CI failing · 📝 draft · areas: dash-spv, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot

<a id="zocolini-ready-for-review"></a>
#### Ready for Review (5)
- [dashpay/platform#5476 feat(platform-wallet)!: report a freshly built asset lock's outpoint to the caller](https://github.com/dashpay/platform/pull/5476) — by @romchornyi · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, fallback · Policy: ready-to-merge
  - Blocker: All policy requirements are satisfied
- [dashpay/platform#5479 feat(platform-wallet-storage): persist DashPay backfill coverage and outbound-account markers](https://github.com/dashpay/platform/pull/5479) — by @HashEngineering · areas: rs-platform-wallet, wallet-storage · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
- [dashpay/platform#5637 chore(platform)!: update rust-dashcore to dev 7329126e](https://github.com/dashpay/platform/pull/5637) — by @shumkov · areas: dpp, rs-platform-wallet, fallback · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head
- [dashpay/rust-dashcore#1223 feat(dash-network)!: source parameters from \`dash-params\`, bump Base SDK crates to dashpay/base-sdk@89828032](https://github.com/dashpay/rust-dashcore/pull/1223) — by @kwvg · areas: dash-spv, key-wallet, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
- [dashpay/rust-dashcore#1226 feat(dash): check address maps against Core v24 port and duplicate rules](https://github.com/dashpay/rust-dashcore/pull/1226) — by @shumkov · areas: fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 05b08aa8c2fa3aa44a3235ffeec9de7fb3f0324b

<a id="dcg-claude"></a>
### @DCG-Claude
<a id="dcg-claude-open"></a>
#### Open (19)
- [dashpay/platform#4703 fix(platform): resolve fee versions by registered number and price refunds at the storage epoch rate](https://github.com/dashpay/platform/pull/4703) — 🔴 CI failing · areas: rs-drive, rs-drive-abci, dpp, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
- [dashpay/platform#4704 fix(platform): record the genesis fee generation and add replay coverage across a fee-version boundary](https://github.com/dashpay/platform/pull/4704) — 🔴 CI failing · areas: rs-drive-abci, fallback · Policy: waiting-build
  - Blocker: The build must pass before a human is asked
- [dashpay/platform#4705 feat(platform): define protocol-versioned smart-contract computation limits and their gas representation](https://github.com/dashpay/platform/pull/4705) — 🔴 CI failing · 🐢 targets v6.0-dev · areas: rs-drive, rs-drive-abci, dpp, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
- [dashpay/platform#4706 feat(platform)!: require fee history for storage refunds and credit their recorded owners](https://github.com/dashpay/platform/pull/4706) — areas: rs-drive, rs-drive-abci, fallback · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head
- [dashpay/platform#4707 feat: alloc-only profiles for platform-value and platform-serialization](https://github.com/dashpay/platform/pull/4707) — 🔴 CI failing · areas: github, fallback · Policy: waiting-build
  - Blocker: The build must pass before a human is asked
- [dashpay/platform#4712 feat(platform): add the dashvm-validation crate and the DashVM protocol table](https://github.com/dashpay/platform/pull/4712) — ⚠ merge conflict · 🔴 CI failing · 🐢 targets v6.0-dev · areas: github, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
- [dashpay/platform#4716 feat(drive)!: add the contract credits root sum tree to genesis, upgrade and credit conservation](https://github.com/dashpay/platform/pull/4716) — 🔴 CI failing · 🐢 targets v6.0-dev · areas: rs-drive, rs-drive-abci, dpp, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
- [dashpay/platform#4717 feat(platform)!: add family-specific state transition size limits and the large contract envelope decode path](https://github.com/dashpay/platform/pull/4717) — 🐢 targets v6.0-dev · areas: rs-drive, rs-drive-abci, dpp, dashmate, rust-dapi, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
- [dashpay/platform#4718 ci: compare cross-architecture replay artifacts across a protocol upgrade and a restart](https://github.com/dashpay/platform/pull/4718) — 🔴 CI failing · 🐢 targets v6.0-dev · areas: rs-drive-abci, github, fallback · Policy: waiting-build
  - Blocker: The build must pass before a human is asked
- [dashpay/platform#4719 feat: add the dash-sdk-contract declaration model, grammar and diagnostics](https://github.com/dashpay/platform/pull/4719) — 🔴 CI failing · areas: github, fallback · Policy: waiting-build
  - Blocker: The build must pass before a human is asked
- [dashpay/platform#4784 feat(platform)!: validate contested index parameters and bind the native award to its poll](https://github.com/dashpay/platform/pull/4784) — ⚠ merge conflict · 🔴 CI failing · 🐢 targets v6.0-dev · areas: rs-drive, rs-drive-abci, dpp, fallback · Policy: waiting-build
  - Blocker: The build must pass before a human is asked
- [dashpay/platform#4789 feat(platform): add typed refund owners to storage flags and fee refunds](https://github.com/dashpay/platform/pull/4789) — 🐢 targets v6.0-dev · areas: rs-drive, dpp, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
- [dashpay/platform#4790 feat(drive)!: keep per-issuer token supply rollups and a destroyed-issuer ledger](https://github.com/dashpay/platform/pull/4790) — ⚠ merge conflict · 🔴 CI failing · 🐢 targets v6.0-dev · areas: rs-drive, rs-drive-abci, dpp, fallback · Policy: waiting-build
  - Blocker: The build must pass before a human is asked
- [dashpay/platform#4794 feat: pin contract configuration vectors and mirror them in wasm-dpp2 and the sdk ffi](https://github.com/dashpay/platform/pull/4794) — 🔴 CI failing · areas: dpp, rust-sdk-ffi, fallback · Policy: waiting-build
  - Blocker: The build must pass before a human is asked
- [dashpay/platform#4808 test(drive-abci): rehearse a reproducible scheduled host fault and the existing recovery path](https://github.com/dashpay/platform/pull/4808) — 🐢 targets v6.0-dev · areas: rs-drive-abci, github, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
- [dashpay/platform#4934 docs: correct the smart-contract target, version counts, transition list and mobile sdk availability](https://github.com/dashpay/platform/pull/4934) — 🐢 targets v6.0-dev · areas: swift-sdk, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
- [dashpay/platform#4938 docs(platform): distinguish current-state proofs, stored receipts and execution result text](https://github.com/dashpay/platform/pull/4938) — 🐢 targets v6.0-dev · areas: dpp, rust-sdk, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
- [dashpay/platform#4939 ci: audit the dashvm engine dependency set and document the equivalence hotfix rule](https://github.com/dashpay/platform/pull/4939) — 🐢 targets v6.0-dev · areas: github, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
- [dashpay/platform#5211 feat(platform)!: add compilation readiness rounds, reports and funds to drive](https://github.com/dashpay/platform/pull/5211) — 🔴 CI failing · 🐢 targets v6.0-dev · areas: rs-drive, rs-drive-abci, dpp, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required

<a id="dcg-claude-ready-for-human"></a>
#### Ready for human (11)
- [dashpay/platform#4703 fix(platform): resolve fee versions by registered number and price refunds at the storage epoch rate](https://github.com/dashpay/platform/pull/4703) — 🔴 CI failing · areas: rs-drive, rs-drive-abci, dpp, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
- [dashpay/platform#4705 feat(platform): define protocol-versioned smart-contract computation limits and their gas representation](https://github.com/dashpay/platform/pull/4705) — 🔴 CI failing · 🐢 targets v6.0-dev · areas: rs-drive, rs-drive-abci, dpp, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
- [dashpay/platform#4712 feat(platform): add the dashvm-validation crate and the DashVM protocol table](https://github.com/dashpay/platform/pull/4712) — ⚠ merge conflict · 🔴 CI failing · 🐢 targets v6.0-dev · areas: github, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
- [dashpay/platform#4716 feat(drive)!: add the contract credits root sum tree to genesis, upgrade and credit conservation](https://github.com/dashpay/platform/pull/4716) — 🔴 CI failing · 🐢 targets v6.0-dev · areas: rs-drive, rs-drive-abci, dpp, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
- [dashpay/platform#4717 feat(platform)!: add family-specific state transition size limits and the large contract envelope decode path](https://github.com/dashpay/platform/pull/4717) — 🐢 targets v6.0-dev · areas: rs-drive, rs-drive-abci, dpp, dashmate, rust-dapi, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
- [dashpay/platform#4789 feat(platform): add typed refund owners to storage flags and fee refunds](https://github.com/dashpay/platform/pull/4789) — 🐢 targets v6.0-dev · areas: rs-drive, dpp, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
- [dashpay/platform#4808 test(drive-abci): rehearse a reproducible scheduled host fault and the existing recovery path](https://github.com/dashpay/platform/pull/4808) — 🐢 targets v6.0-dev · areas: rs-drive-abci, github, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
- [dashpay/platform#4934 docs: correct the smart-contract target, version counts, transition list and mobile sdk availability](https://github.com/dashpay/platform/pull/4934) — 🐢 targets v6.0-dev · areas: swift-sdk, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
- [dashpay/platform#4938 docs(platform): distinguish current-state proofs, stored receipts and execution result text](https://github.com/dashpay/platform/pull/4938) — 🐢 targets v6.0-dev · areas: dpp, rust-sdk, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
- [dashpay/platform#4939 ci: audit the dashvm engine dependency set and document the equivalence hotfix rule](https://github.com/dashpay/platform/pull/4939) — 🐢 targets v6.0-dev · areas: github, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
- [dashpay/platform#5211 feat(platform)!: add compilation readiness rounds, reports and funds to drive](https://github.com/dashpay/platform/pull/5211) — 🔴 CI failing · 🐢 targets v6.0-dev · areas: rs-drive, rs-drive-abci, dpp, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required

<a id="dcg-claude-ci-failing"></a>
#### CI Failing (5)
- [dashpay/platform#4703 fix(platform): resolve fee versions by registered number and price refunds at the storage epoch rate](https://github.com/dashpay/platform/pull/4703) — 🔴 CI failing · areas: rs-drive, rs-drive-abci, dpp, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
- [dashpay/platform#4704 fix(platform): record the genesis fee generation and add replay coverage across a fee-version boundary](https://github.com/dashpay/platform/pull/4704) — 🔴 CI failing · areas: rs-drive-abci, fallback · Policy: waiting-build
  - Blocker: The build must pass before a human is asked
- [dashpay/platform#4707 feat: alloc-only profiles for platform-value and platform-serialization](https://github.com/dashpay/platform/pull/4707) — 🔴 CI failing · areas: github, fallback · Policy: waiting-build
  - Blocker: The build must pass before a human is asked
- [dashpay/platform#4719 feat: add the dash-sdk-contract declaration model, grammar and diagnostics](https://github.com/dashpay/platform/pull/4719) — 🔴 CI failing · areas: github, fallback · Policy: waiting-build
  - Blocker: The build must pass before a human is asked
- [dashpay/platform#4794 feat: pin contract configuration vectors and mirror them in wasm-dpp2 and the sdk ffi](https://github.com/dashpay/platform/pull/4794) — 🔴 CI failing · areas: dpp, rust-sdk-ffi, fallback · Policy: waiting-build
  - Blocker: The build must pass before a human is asked

<a id="dcg-claude-stale"></a>
#### Stale (13)
- [dashpay/platform#4705 feat(platform): define protocol-versioned smart-contract computation limits and their gas representation](https://github.com/dashpay/platform/pull/4705) — 🔴 CI failing · 🐢 targets v6.0-dev · areas: rs-drive, rs-drive-abci, dpp, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
- [dashpay/platform#4712 feat(platform): add the dashvm-validation crate and the DashVM protocol table](https://github.com/dashpay/platform/pull/4712) — ⚠ merge conflict · 🔴 CI failing · 🐢 targets v6.0-dev · areas: github, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
- [dashpay/platform#4716 feat(drive)!: add the contract credits root sum tree to genesis, upgrade and credit conservation](https://github.com/dashpay/platform/pull/4716) — 🔴 CI failing · 🐢 targets v6.0-dev · areas: rs-drive, rs-drive-abci, dpp, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
- [dashpay/platform#4717 feat(platform)!: add family-specific state transition size limits and the large contract envelope decode path](https://github.com/dashpay/platform/pull/4717) — 🐢 targets v6.0-dev · areas: rs-drive, rs-drive-abci, dpp, dashmate, rust-dapi, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
- [dashpay/platform#4718 ci: compare cross-architecture replay artifacts across a protocol upgrade and a restart](https://github.com/dashpay/platform/pull/4718) — 🔴 CI failing · 🐢 targets v6.0-dev · areas: rs-drive-abci, github, fallback · Policy: waiting-build
  - Blocker: The build must pass before a human is asked
- [dashpay/platform#4784 feat(platform)!: validate contested index parameters and bind the native award to its poll](https://github.com/dashpay/platform/pull/4784) — ⚠ merge conflict · 🔴 CI failing · 🐢 targets v6.0-dev · areas: rs-drive, rs-drive-abci, dpp, fallback · Policy: waiting-build
  - Blocker: The build must pass before a human is asked
- [dashpay/platform#4789 feat(platform): add typed refund owners to storage flags and fee refunds](https://github.com/dashpay/platform/pull/4789) — 🐢 targets v6.0-dev · areas: rs-drive, dpp, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
- [dashpay/platform#4790 feat(drive)!: keep per-issuer token supply rollups and a destroyed-issuer ledger](https://github.com/dashpay/platform/pull/4790) — ⚠ merge conflict · 🔴 CI failing · 🐢 targets v6.0-dev · areas: rs-drive, rs-drive-abci, dpp, fallback · Policy: waiting-build
  - Blocker: The build must pass before a human is asked
- [dashpay/platform#4808 test(drive-abci): rehearse a reproducible scheduled host fault and the existing recovery path](https://github.com/dashpay/platform/pull/4808) — 🐢 targets v6.0-dev · areas: rs-drive-abci, github, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
- [dashpay/platform#4934 docs: correct the smart-contract target, version counts, transition list and mobile sdk availability](https://github.com/dashpay/platform/pull/4934) — 🐢 targets v6.0-dev · areas: swift-sdk, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
- [dashpay/platform#4938 docs(platform): distinguish current-state proofs, stored receipts and execution result text](https://github.com/dashpay/platform/pull/4938) — 🐢 targets v6.0-dev · areas: dpp, rust-sdk, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
- [dashpay/platform#4939 ci: audit the dashvm engine dependency set and document the equivalence hotfix rule](https://github.com/dashpay/platform/pull/4939) — 🐢 targets v6.0-dev · areas: github, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
- [dashpay/platform#5211 feat(platform)!: add compilation readiness rounds, reports and funds to drive](https://github.com/dashpay/platform/pull/5211) — 🔴 CI failing · 🐢 targets v6.0-dev · areas: rs-drive, rs-drive-abci, dpp, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required

<a id="dcg-claude-clean"></a>
#### Clean (1)
- [dashpay/platform#4706 feat(platform)!: require fee history for storage refunds and credit their recorded owners](https://github.com/dashpay/platform/pull/4706) — areas: rs-drive, rs-drive-abci, fallback · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head

<a id="kwvg"></a>
### @kwvg
<a id="kwvg-open"></a>
#### Open (1)
- [dashpay/rust-dashcore#1223 feat(dash-network)!: source parameters from \`dash-params\`, bump Base SDK crates to dashpay/base-sdk@89828032](https://github.com/dashpay/rust-dashcore/pull/1223) — areas: dash-spv, key-wallet, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required

<a id="kwvg-ready-for-human"></a>
#### Ready for human (1)
- [dashpay/rust-dashcore#1223 feat(dash-network)!: source parameters from \`dash-params\`, bump Base SDK crates to dashpay/base-sdk@89828032](https://github.com/dashpay/rust-dashcore/pull/1223) — areas: dash-spv, key-wallet, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required

<a id="kwvg-clean"></a>
#### Clean (1)
- [dashpay/rust-dashcore#1223 feat(dash-network)!: source parameters from \`dash-params\`, bump Base SDK crates to dashpay/base-sdk@89828032](https://github.com/dashpay/rust-dashcore/pull/1223) — areas: dash-spv, key-wallet, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required

<a id="thepastaclaw"></a>
### @thepastaclaw
<a id="thepastaclaw-open"></a>
#### Open (2)
- [dashpay/platform#4298 fix(dashmate): load ZeroSSL config in force mode](https://github.com/dashpay/platform/pull/4298) — 🔴 CI failing · areas: dashmate · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
- [dashpay/platform#5581 feat(platform): add proof-backed asset lock outpoint query](https://github.com/dashpay/platform/pull/5581) — 🔴 CI failing · areas: rs-drive, rs-drive-abci, dpp, rs-platform-wallet, rust-sdk, rust-dapi, fallback · Policy: waiting-author
  - Blocker: coderabbitai left review threads unresolved; resolve them
  - Blocker: thepastaclaw left review threads unresolved; resolve them

<a id="thepastaclaw-ready-for-human"></a>
#### Ready for human (1)
- [dashpay/platform#4298 fix(dashmate): load ZeroSSL config in force mode](https://github.com/dashpay/platform/pull/4298) — 🔴 CI failing · areas: dashmate · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required

<a id="thepastaclaw-ci-failing"></a>
#### CI Failing (2)
- [dashpay/platform#4298 fix(dashmate): load ZeroSSL config in force mode](https://github.com/dashpay/platform/pull/4298) — 🔴 CI failing · areas: dashmate · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
- [dashpay/platform#5581 feat(platform): add proof-backed asset lock outpoint query](https://github.com/dashpay/platform/pull/5581) — 🔴 CI failing · areas: rs-drive, rs-drive-abci, dpp, rs-platform-wallet, rust-sdk, rust-dapi, fallback · Policy: waiting-author
  - Blocker: coderabbitai left review threads unresolved; resolve them
  - Blocker: thepastaclaw left review threads unresolved; resolve them

## Methodology
Generated nightly by [pr-hygiene](https://github.com/dashpay/stale_prs_are_bad). A thread counts as "unresolved" when it is open, not outdated, has a comment from someone other than the PR author, and the most recent comment is from a reviewer. **Dirty** = at least one such thread. **Unresolved Comments** = at least one such thread. **Changes Requested** = no unresolved threads but a reviewer's most recent review is CHANGES_REQUESTED (still blocking until someone re-approves or dismisses). **Deferred** = carries a configured deferred label (e.g. `postponed`) — visible but not counted toward unresolved-comment counts. **Stale** = targets a non-default branch OR hasn't been touched in the configured threshold (default 120 days, but clean PRs are never reclassified as stale). **Draft** = the PR is still marked draft on GitHub. **CI failing** = no unresolved comments, no changes-requested, but the latest commit's status check is failing. **Clean** = open, not draft, not deferred, not stale, no unresolved comments, no changes-requested, CI green. **Needs action** further requires changes-requested, merge conflict, or that the reviewer commented more recently than the author last pushed. **Ready for human** counts a person's own PRs that the shared review engine marks `ready-for-human` or `ready-to-merge`; each PR bullet shows the engine's state as `Policy:` with its blockers. **Ready for Review** counts clean PRs (authored by someone else) where this person owes a review: the union of the shared policy's routing (owners and reviewers of every area the changed files fall into, or the repository fallback for files no area claims) and GitHub's explicit review requests. The author is never routed to their own PR, and anyone who has already submitted any review is excluded — their job is done. `⚠ ownership unresolved` marks areas whose roster the policy still lists as open. Configurable via [`https://github.com/dashpay/stale_prs_are_bad/blob/master/.pr-hygiene.yml`](https://github.com/dashpay/stale_prs_are_bad/blob/master/.pr-hygiene.yml)—edit defaults there; ownership lives in `policies/`.
