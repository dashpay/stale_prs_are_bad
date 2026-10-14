---
---
# PR Hygiene Report
*Last updated: 2026-10-14 13:16 UTC · commit a670cf0*

## Summary
- Open PRs: **136** (10 clean · 3 CI failing · 5 changes requested · 13 unresolved comments · 0 deferred · 16 draft · 89 stale)
- PRs needing author action: **20**
- Total unresolved comments: **131**
- dashpay/platform: **96** open (5 clean · 1 CI failing · 1 changes requested · 5 unresolved comments · 0 deferred · 2 draft · 82 stale) · engine: 15 draft · 22 ready-for-human · 2 ready-to-merge · 11 waiting-author · 9 waiting-bots · 6 waiting-build · 18 waiting-self-review · 13 no verdict
- dashpay/rust-dashcore: **27** open (1 clean · 1 CI failing · 4 changes requested · 6 unresolved comments · 0 deferred · 12 draft · 3 stale) · engine: 14 draft · 5 waiting-author · 1 waiting-bots · 6 waiting-self-review · 1 no verdict
- dashpay/tenderdash: **4** open (0 clean · 1 CI failing · 0 changes requested · 2 unresolved comments · 0 deferred · 1 draft · 0 stale) · engine: 1 draft · 1 ready-for-human · 1 ready-to-merge · 1 waiting-self-review
- dashpay/grovedb: **2** open (1 clean · 0 CI failing · 0 changes requested · 0 unresolved comments · 0 deferred · 1 draft · 0 stale) · engine: 1 draft · 1 waiting-self-review
- dashpay/dash-evo-tool: **7** open (3 clean · 0 CI failing · 0 changes requested · 0 unresolved comments · 0 deferred · 0 draft · 4 stale) · engine: 2 ready-for-human · 1 waiting-bots · 4 no verdict

## Scoreboard
_Sort: unresolved-comments desc → needs-action desc → ready-for-review desc. Click any number to jump to the specific PRs it covers._

| Author | Open | Clean | CI failing | Unresolved Comments | Changes Requested | Deferred | Draft | Stale | Needs action | Ready for human | Total Unresolved Comments | Ready for Review | Δ |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| [@QuantumExplorer](#quantumexplorer) | [16](#quantumexplorer-open) | [1](#quantumexplorer-clean) | — | [3](#quantumexplorer-unresolved-comments) | — | — | — | [12](#quantumexplorer-stale) | [4](#quantumexplorer-needs-action) | — | [32](#quantumexplorer-unresolved-comments) | [4](#quantumexplorer-ready-for-review) | ↓ 2 |
| [@shumkov](#shumkov) | [13](#shumkov-open) | [1](#shumkov-clean) | [1](#shumkov-ci-failing) | [3](#shumkov-unresolved-comments) | — | — | — | [8](#shumkov-stale) | [3](#shumkov-needs-action) | — | [37](#shumkov-unresolved-comments) | [3](#shumkov-ready-for-review) | ↓ 1 |
| [@lklimek](#lklimek) | [13](#lklimek-open) | [1](#lklimek-clean) | [1](#lklimek-ci-failing) | [2](#lklimek-unresolved-comments) | — | — | [2](#lklimek-draft) | [7](#lklimek-stale) | [3](#lklimek-needs-action) | [2](#lklimek-ready-for-human) | [5](#lklimek-unresolved-comments) | [4](#lklimek-ready-for-review) | — |
| [@romchornyi](#romchornyi) | [7](#romchornyi-open) | [1](#romchornyi-clean) | — | [2](#romchornyi-unresolved-comments) | [1](#romchornyi-changes-requested) | — | [1](#romchornyi-draft) | [2](#romchornyi-stale) | [2](#romchornyi-needs-action) | [3](#romchornyi-ready-for-human) | [6](#romchornyi-unresolved-comments) | [2](#romchornyi-ready-for-review) | — |
| [@PastaPastaPasta](#pastapastapasta) | [19](#pastapastapasta-open) | — | [1](#pastapastapasta-ci-failing) | [1](#pastapastapasta-unresolved-comments) | [2](#pastapastapasta-changes-requested) | — | [1](#pastapastapasta-draft) | [14](#pastapastapasta-stale) | [4](#pastapastapasta-needs-action) | — | [22](#pastapastapasta-unresolved-comments) | — | — |
| [@llbartekll](#llbartekll) | [6](#llbartekll-open) | [3](#llbartekll-clean) | — | [1](#llbartekll-unresolved-comments) | [1](#llbartekll-changes-requested) | — | [1](#llbartekll-draft) | — | [2](#llbartekll-needs-action) | [2](#llbartekll-ready-for-human) | [1](#llbartekll-unresolved-comments) | — | — |
| [@xdustinface](#xdustinface) | [8](#xdustinface-open) | [1](#xdustinface-clean) | — | [1](#xdustinface-unresolved-comments) | — | — | [4](#xdustinface-draft) | [2](#xdustinface-stale) | [1](#xdustinface-needs-action) | — | [1](#xdustinface-unresolved-comments) | — | — |
| [@HashEngineering](#hashengineering) | [4](#hashengineering-open) | — | — | — | [1](#hashengineering-changes-requested) | — | [1](#hashengineering-draft) | [2](#hashengineering-stale) | [1](#hashengineering-needs-action) | [2](#hashengineering-ready-for-human) | [7](#hashengineering-unresolved-comments) | [3](#hashengineering-ready-for-review) | — |
| [@ZocoLini](#zocolini) | [5](#zocolini-open) | — | — | — | — | — | [5](#zocolini-draft) | — | — | — | [2](#zocolini-unresolved-comments) | [3](#zocolini-ready-for-review) | ↓ 1 |
| [@ktechmidas](#ktechmidas) | [6](#ktechmidas-open) | — | — | — | — | — | — | [6](#ktechmidas-stale) | — | — | [1](#ktechmidas-unresolved-comments) | [1](#ktechmidas-ready-for-review) | — |
| [@thepastaclaw](#thepastaclaw) | [3](#thepastaclaw-open) | [2](#thepastaclaw-clean) | — | — | — | — | — | [1](#thepastaclaw-stale) | — | [3](#thepastaclaw-ready-for-human) | — | [1](#thepastaclaw-ready-for-review) | ↓ 3 |
| [@Claudius-Maginificent](#claudius-maginificent) | [11](#claudius-maginificent-open) | — | — | — | — | — | — | [11](#claudius-maginificent-stale) | — | [2](#claudius-maginificent-ready-for-human) | [9](#claudius-maginificent-unresolved-comments) | — | ↓ 1 |
| [@infraclaw-dash](#infraclaw-dash) | [3](#infraclaw-dash-open) | — | — | — | — | — | — | [3](#infraclaw-dash-stale) | — | [1](#infraclaw-dash-ready-for-human) | [5](#infraclaw-dash-unresolved-comments) | — | — |
| [@bfoss765](#bfoss765) | [2](#bfoss765-open) | — | — | — | — | — | — | [2](#bfoss765-stale) | — | — | [3](#bfoss765-unresolved-comments) | — | — |
| [@DCG-Claude](#dcg-claude) | [19](#dcg-claude-open) | — | — | — | — | — | — | [19](#dcg-claude-stale) | — | [13](#dcg-claude-ready-for-human) | — | — | — |
| [@kwvg](#kwvg) | [1](#kwvg-open) | — | — | — | — | — | [1](#kwvg-draft) | — | — | — | — | — | — |

## Per-author detail

<a id="quantumexplorer"></a>
### @QuantumExplorer
<a id="quantumexplorer-open"></a>
#### Open (16)
- [dashpay/rust-dashcore#1078 feat(key-wallet)!: lock masternode collateral out of coin selection](https://github.com/dashpay/rust-dashcore/pull/1078) — 8 unresolved (8 human) · 14 days stale · ⚠ merge conflict · ✋ changes requested · areas: key-wallet, key-wallet-manager · Policy: waiting-self-review
  - Top thread: "This sets the state by hand, and together with the serde round-trip it is the only coverage of the refresh inside \`updat…" — 14 days old
  - Blocker: Author must post /self-reviewed 44a9a010500fbd44aba09111b4c53a28b88e8073
- [dashpay/platform#4272 feat(dpp)!: dashpay payment detection keys and stealth derivation](https://github.com/dashpay/platform/pull/4272) — 9 unresolved (9 bot) · 72 days stale · ⚠ merge conflict · 🔴 CI failing · 📝 draft · 🐢 targets v5.1-dev · areas: rs-drive, rs-drive-abci, dpp, rs-platform-wallet, swift-sdk, system-contracts, kotlin-sdk, fallback · Policy: draft
  - Top thread: "🔴 Blocking: Require compressed 33-byte encodings for payment keys**" — 72 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#860 feat(dash-spv): expose per-peer connection stats (ping RTT, bytes received)](https://github.com/dashpay/rust-dashcore/pull/860) — 3 unresolved (3 CodeRabbit) · 96 days stale · ✋ changes requested · areas: dash-spv, fallback · Policy: waiting-author
  - Top thread: "_📐 Maintainability &amp; Code Quality_ \| _🟡 Minor_ \| _⚡ Quick win_" — 96 days old
  - Blocker: coderabbitai left review threads unresolved; resolve them
- [dashpay/platform#3417 feat(swift-sdk): use SPV-synced quorums for Platform proof verification](https://github.com/dashpay/platform/pull/3417) — 3 unresolved (3 bot) · 36 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · 🐢 targets v5.1-dev · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, rust-sdk, rust-sdk-ffi, fallback · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Exercise the production runtime bridge across Tokio runtime flavors**" — 36 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/platform#5238 fix(platform)!: refund sponsor-paid document storage to the gas sponsor](https://github.com/dashpay/platform/pull/5238) — 3 unresolved (3 bot) · 13 days stale · ⚠ merge conflict · ✋ changes requested · 🐢 targets v5.1-dev · areas: rs-drive, rs-drive-abci, fallback · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Make the transfer regression change the primary element's size**" — 13 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/rust-dashcore#1077 feat(dash-spv)!: check masternode list diffs against the block coinbase](https://github.com/dashpay/rust-dashcore/pull/1077) — 2 unresolved (2 CodeRabbit) · 16 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · areas: dash-spv, fallback · Policy: waiting-author
  - Top thread: "_🩺 Stability &amp; Availability_ \| _🟠 Major_ \| _⚡ Quick win_" — 16 days old
  - Blocker: coderabbitai requested changes on this head; dismiss the review or push a fix
  - Blocker: coderabbitai left review threads unresolved; resolve them
- [dashpay/platform#5360 perf(platform)!: delete a consumed document from what its lookup read (PV15)](https://github.com/dashpay/platform/pull/5360) — 2 unresolved (2 bot) · 5 days stale · 🐢 targets claude/wizardly-driscoll-44c981 · areas: rs-drive, rs-drive-abci, fallback
  - Top thread: "🟡 Suggestion: Add negative-path tests for the new read-document deletion guards**" — 5 days old
- [dashpay/platform#5337 feat(platform)!: dpns contract v3 checks name registration with schema keywords (PV15)](https://github.com/dashpay/platform/pull/5337) — 2 unresolved (2 bot) · 1 days stale · ⚠ merge conflict · ✋ changes requested · 🐢 targets v5.1-dev · areas: rs-drive, rs-drive-abci, dpp, rust-sdk, js-wasm-sdk, system-contracts, fallback · Policy: waiting-bots
  - Top thread: "🟡 Suggestion: Check a pending preorder's TTL before reusing it**" — 1 days old
  - Blocker: coderabbitai has not reported for the current head
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/grovedb#1001 feat(batch)!: opt-in settlement of storage owner changes](https://github.com/dashpay/grovedb/pull/1001) — ⚠ merge conflict · areas: fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 814dfc765747cec0beb7a39e1cdf04d301ce9ac3
- [dashpay/platform#4519 feat(platform-wallet): rotate a masternode's keys into the wallet — ProUpRegTx orchestration, FFI, Swift](https://github.com/dashpay/platform/pull/4519) — ⚠ merge conflict · 🔴 CI failing · 🐢 targets v5.1-dev · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 92a3ebe17adc12432fb756c63f172c663809007d
  - Blocker: Proceeded without coderabbitai: no review within the configured window
  - Blocker: Proceeded without thepastaclaw: no review within the configured window
- [dashpay/platform#4730 feat(platform)!: delta-based data contract update transition for protocol version 15](https://github.com/dashpay/platform/pull/4730) — ⚠ merge conflict · 🔴 CI failing · 🐢 targets v5.1-dev · areas: rs-drive, rs-drive-abci, dpp, rs-platform-wallet, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 53cb036efc244155d88a30c829148f90d7903d3e
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/platform#4776 feat(platform-wallet)!: let contract updates clear the description through the FFI](https://github.com/dashpay/platform/pull/4776) — 🔴 CI failing · 🐢 targets feat/delta-contract-update-pv15 · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk, fallback
- [dashpay/platform#4899 feat(platform)!: yes/no masternode vote poll kind with supermajority and minimum voting power](https://github.com/dashpay/platform/pull/4899) — ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · 📝 draft · 🐢 targets v5.1-dev · areas: rs-drive, rs-drive-abci, dpp, swift-sdk, rust-sdk, rust-sdk-ffi, js-wasm-sdk, rust-dapi, kotlin-sdk, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#4933 feat(platform)!: dashpay contact requests declare their checks](https://github.com/dashpay/platform/pull/4933) — ⚠ merge conflict · 🔴 CI failing · 📝 draft · 🐢 targets v5.1-dev · areas: rs-drive, rs-drive-abci, dpp, system-contracts, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#4986 fix(drive-abci)!: pay every reward share of a masternode and credit each identity once in the epoch payout (PV15)](https://github.com/dashpay/platform/pull/4986) — ⚠ merge conflict · 📝 draft · 🐢 targets v5.1-dev · areas: rs-drive, rs-drive-abci, dpp, github, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5113 feat(platform-wallet)!: keep masternode collateral out of coin selection](https://github.com/dashpay/platform/pull/5113) — ⚠ merge conflict · 🔴 CI failing · 📝 draft · 🐢 targets chore/bump-rust-dashcore-secp-033 · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk, fallback

<a id="quantumexplorer-needs-action"></a>
#### Needs action (4)
- [dashpay/rust-dashcore#1078 feat(key-wallet)!: lock masternode collateral out of coin selection](https://github.com/dashpay/rust-dashcore/pull/1078) — 8 unresolved (8 human) · 14 days stale · ⚠ merge conflict · ✋ changes requested · areas: key-wallet, key-wallet-manager · Policy: waiting-self-review
  - Top thread: "This sets the state by hand, and together with the serde round-trip it is the only coverage of the refresh inside \`updat…" — 14 days old
  - Blocker: Author must post /self-reviewed 44a9a010500fbd44aba09111b4c53a28b88e8073
- [dashpay/rust-dashcore#860 feat(dash-spv): expose per-peer connection stats (ping RTT, bytes received)](https://github.com/dashpay/rust-dashcore/pull/860) — 3 unresolved (3 CodeRabbit) · 96 days stale · ✋ changes requested · areas: dash-spv, fallback · Policy: waiting-author
  - Top thread: "_📐 Maintainability &amp; Code Quality_ \| _🟡 Minor_ \| _⚡ Quick win_" — 96 days old
  - Blocker: coderabbitai left review threads unresolved; resolve them
- [dashpay/rust-dashcore#1077 feat(dash-spv)!: check masternode list diffs against the block coinbase](https://github.com/dashpay/rust-dashcore/pull/1077) — 2 unresolved (2 CodeRabbit) · 16 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · areas: dash-spv, fallback · Policy: waiting-author
  - Top thread: "_🩺 Stability &amp; Availability_ \| _🟠 Major_ \| _⚡ Quick win_" — 16 days old
  - Blocker: coderabbitai requested changes on this head; dismiss the review or push a fix
  - Blocker: coderabbitai left review threads unresolved; resolve them
- [dashpay/grovedb#1001 feat(batch)!: opt-in settlement of storage owner changes](https://github.com/dashpay/grovedb/pull/1001) — ⚠ merge conflict · areas: fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 814dfc765747cec0beb7a39e1cdf04d301ce9ac3

<a id="quantumexplorer-unresolved-comments"></a>
#### Unresolved Comments (3)
- [dashpay/rust-dashcore#1078 feat(key-wallet)!: lock masternode collateral out of coin selection](https://github.com/dashpay/rust-dashcore/pull/1078) — 8 unresolved (8 human) · 14 days stale · ⚠ merge conflict · ✋ changes requested · areas: key-wallet, key-wallet-manager · Policy: waiting-self-review
  - Top thread: "This sets the state by hand, and together with the serde round-trip it is the only coverage of the refresh inside \`updat…" — 14 days old
  - Blocker: Author must post /self-reviewed 44a9a010500fbd44aba09111b4c53a28b88e8073
- [dashpay/rust-dashcore#860 feat(dash-spv): expose per-peer connection stats (ping RTT, bytes received)](https://github.com/dashpay/rust-dashcore/pull/860) — 3 unresolved (3 CodeRabbit) · 96 days stale · ✋ changes requested · areas: dash-spv, fallback · Policy: waiting-author
  - Top thread: "_📐 Maintainability &amp; Code Quality_ \| _🟡 Minor_ \| _⚡ Quick win_" — 96 days old
  - Blocker: coderabbitai left review threads unresolved; resolve them
- [dashpay/rust-dashcore#1077 feat(dash-spv)!: check masternode list diffs against the block coinbase](https://github.com/dashpay/rust-dashcore/pull/1077) — 2 unresolved (2 CodeRabbit) · 16 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · areas: dash-spv, fallback · Policy: waiting-author
  - Top thread: "_🩺 Stability &amp; Availability_ \| _🟠 Major_ \| _⚡ Quick win_" — 16 days old
  - Blocker: coderabbitai requested changes on this head; dismiss the review or push a fix
  - Blocker: coderabbitai left review threads unresolved; resolve them

<a id="quantumexplorer-stale"></a>
#### Stale (12)
- [dashpay/platform#4272 feat(dpp)!: dashpay payment detection keys and stealth derivation](https://github.com/dashpay/platform/pull/4272) — 9 unresolved (9 bot) · 72 days stale · ⚠ merge conflict · 🔴 CI failing · 📝 draft · 🐢 targets v5.1-dev · areas: rs-drive, rs-drive-abci, dpp, rs-platform-wallet, swift-sdk, system-contracts, kotlin-sdk, fallback · Policy: draft
  - Top thread: "🔴 Blocking: Require compressed 33-byte encodings for payment keys**" — 72 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#3417 feat(swift-sdk): use SPV-synced quorums for Platform proof verification](https://github.com/dashpay/platform/pull/3417) — 3 unresolved (3 bot) · 36 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · 🐢 targets v5.1-dev · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, rust-sdk, rust-sdk-ffi, fallback · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Exercise the production runtime bridge across Tokio runtime flavors**" — 36 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/platform#5238 fix(platform)!: refund sponsor-paid document storage to the gas sponsor](https://github.com/dashpay/platform/pull/5238) — 3 unresolved (3 bot) · 13 days stale · ⚠ merge conflict · ✋ changes requested · 🐢 targets v5.1-dev · areas: rs-drive, rs-drive-abci, fallback · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Make the transfer regression change the primary element's size**" — 13 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/platform#5360 perf(platform)!: delete a consumed document from what its lookup read (PV15)](https://github.com/dashpay/platform/pull/5360) — 2 unresolved (2 bot) · 5 days stale · 🐢 targets claude/wizardly-driscoll-44c981 · areas: rs-drive, rs-drive-abci, fallback
  - Top thread: "🟡 Suggestion: Add negative-path tests for the new read-document deletion guards**" — 5 days old
- [dashpay/platform#5337 feat(platform)!: dpns contract v3 checks name registration with schema keywords (PV15)](https://github.com/dashpay/platform/pull/5337) — 2 unresolved (2 bot) · 1 days stale · ⚠ merge conflict · ✋ changes requested · 🐢 targets v5.1-dev · areas: rs-drive, rs-drive-abci, dpp, rust-sdk, js-wasm-sdk, system-contracts, fallback · Policy: waiting-bots
  - Top thread: "🟡 Suggestion: Check a pending preorder's TTL before reusing it**" — 1 days old
  - Blocker: coderabbitai has not reported for the current head
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#4519 feat(platform-wallet): rotate a masternode's keys into the wallet — ProUpRegTx orchestration, FFI, Swift](https://github.com/dashpay/platform/pull/4519) — ⚠ merge conflict · 🔴 CI failing · 🐢 targets v5.1-dev · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 92a3ebe17adc12432fb756c63f172c663809007d
  - Blocker: Proceeded without coderabbitai: no review within the configured window
  - Blocker: Proceeded without thepastaclaw: no review within the configured window
- [dashpay/platform#4730 feat(platform)!: delta-based data contract update transition for protocol version 15](https://github.com/dashpay/platform/pull/4730) — ⚠ merge conflict · 🔴 CI failing · 🐢 targets v5.1-dev · areas: rs-drive, rs-drive-abci, dpp, rs-platform-wallet, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 53cb036efc244155d88a30c829148f90d7903d3e
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/platform#4776 feat(platform-wallet)!: let contract updates clear the description through the FFI](https://github.com/dashpay/platform/pull/4776) — 🔴 CI failing · 🐢 targets feat/delta-contract-update-pv15 · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk, fallback
- [dashpay/platform#4899 feat(platform)!: yes/no masternode vote poll kind with supermajority and minimum voting power](https://github.com/dashpay/platform/pull/4899) — ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · 📝 draft · 🐢 targets v5.1-dev · areas: rs-drive, rs-drive-abci, dpp, swift-sdk, rust-sdk, rust-sdk-ffi, js-wasm-sdk, rust-dapi, kotlin-sdk, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#4933 feat(platform)!: dashpay contact requests declare their checks](https://github.com/dashpay/platform/pull/4933) — ⚠ merge conflict · 🔴 CI failing · 📝 draft · 🐢 targets v5.1-dev · areas: rs-drive, rs-drive-abci, dpp, system-contracts, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#4986 fix(drive-abci)!: pay every reward share of a masternode and credit each identity once in the epoch payout (PV15)](https://github.com/dashpay/platform/pull/4986) — ⚠ merge conflict · 📝 draft · 🐢 targets v5.1-dev · areas: rs-drive, rs-drive-abci, dpp, github, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5113 feat(platform-wallet)!: keep masternode collateral out of coin selection](https://github.com/dashpay/platform/pull/5113) — ⚠ merge conflict · 🔴 CI failing · 📝 draft · 🐢 targets chore/bump-rust-dashcore-secp-033 · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk, fallback

<a id="quantumexplorer-clean"></a>
#### Clean (1)
- [dashpay/grovedb#1001 feat(batch)!: opt-in settlement of storage owner changes](https://github.com/dashpay/grovedb/pull/1001) — ⚠ merge conflict · areas: fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 814dfc765747cec0beb7a39e1cdf04d301ce9ac3

<a id="quantumexplorer-ready-for-review"></a>
#### Ready for Review (4)
- [dashpay/platform#5396 fix(platform-wallet)!: drain Core balance with restart-safe shielded recovery](https://github.com/dashpay/platform/pull/5396) — by @llbartekll · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk, kotlin-sdk, fallback · Policy: waiting-bots
  - Blocker: coderabbitai has not reported for the current head
- [dashpay/platform#5409 feat(sdk): persist the owner balance a document write's proof carries](https://github.com/dashpay/platform/pull/5409) — by @llbartekll · areas: rs-platform-wallet, rust-sdk, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/platform#5431 fix(drive-abci)!: refuse IdentityCreate identity id and key proof failures before they consume the asset lock](https://github.com/dashpay/platform/pull/5431) — by @shumkov · areas: rs-drive-abci, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed afb322c48b40c86ebb289004442cdd7a1d152827
- [dashpay/rust-dashcore#849 feat(dash-spv): add \`--birth-height\` CLI flag](https://github.com/dashpay/rust-dashcore/pull/849) — by @xdustinface · areas: dash-spv · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 9b1e709fd8e0bfab0dd69f7c86fd6b7d2e585fbe

<a id="shumkov"></a>
### @shumkov
<a id="shumkov-open"></a>
#### Open (13)
- [dashpay/rust-dashcore#634 feat(key-wallet-manager): single-map WalletManager with Arc&lt;RwLock&lt;T&gt;&gt; per wallet](https://github.com/dashpay/rust-dashcore/pull/634) — 20 unresolved (20 CodeRabbit) · 189 days stale · ⚠ merge conflict · ✋ changes requested · areas: dash-spv, key-wallet, key-wallet-manager, fallback · Policy: waiting-author
  - Top thread: "_⚠️ Potential issue_ \| _🟡 Minor_" — 189 days old
  - Blocker: coderabbitai requested changes on this head; dismiss the review or push a fix
  - Blocker: coderabbitai left review threads unresolved; resolve them
- [dashpay/platform#4282 fix(dashmate)!: give Debian packages versions apt can order](https://github.com/dashpay/platform/pull/4282) — 6 unresolved (1 CodeRabbit, 5 bot) · 44 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · 🐢 targets v5.1-dev · areas: dashmate, js-wasm-sdk, system-contracts, github, fallback · Policy: waiting-author
  - Top thread: "_🗄️ Data Integrity &amp; Integration_ \| _🟡 Minor_ \| _⚡ Quick win_" — 44 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/platform#3936 chore(drive-abci): update to nested address in SML](https://github.com/dashpay/platform/pull/3936) — 2 unresolved (2 bot) · 117 days stale · ⚠ merge conflict · 📝 draft · 🐢 targets v5.1-dev · areas: rs-drive-abci, rust-sdk, fallback · Policy: draft
  - Top thread: "🟡 Suggestion: V0 reverse synthesizes \`addresses\` from a Core-22 port, masking subsequent legacy port-change diffs after …" — 117 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#4283 fix(dashmate): keep the node up when an image pull fails, and stop reporting success when it did not](https://github.com/dashpay/platform/pull/4283) — 2 unresolved (2 bot) · 38 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · 🐢 targets v5.1-dev · areas: dashmate · Policy: waiting-author
  - Top thread: "🟡 Suggestion: providers.js sanitizer misses U+061C (Arabic letter mark) that sanitizeRemoteText covers**" — 38 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/platform#4660 feat(sdk): expose the document erase and history lifecycle on mobile and FFI](https://github.com/dashpay/platform/pull/4660) — 2 unresolved (2 bot) · 22 days stale · ✋ changes requested · 🔴 CI failing · 🐢 targets keep-history-lifecycle · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, rust-sdk-ffi, kotlin-sdk, fallback
  - Top thread: "🟡 Suggestion: Validate history selector discriminant at the C boundary**" — 22 days old
- [dashpay/platform#4657 feat(platform)!: delete and erase lifecycle for keep-history documents](https://github.com/dashpay/platform/pull/4657) — 2 unresolved (2 bot) · 21 days stale · ✋ changes requested · 🔴 CI failing · 🐢 targets keep-history-storage-v2 · areas: rs-drive, rs-drive-abci, dpp, rust-sdk, js-wasm-sdk, fallback
  - Top thread: "🟡 Suggestion: Deduplicate lifecycle containers at the Drive batch boundary**" — 21 days old
- [dashpay/platform#5242 chore(skills): show each reviewer their part in /prs](https://github.com/dashpay/platform/pull/5242) — 2 unresolved (1 CodeRabbit, 1 bot) · 13 days stale · 🔴 CI failing · areas: fallback · Policy: waiting-self-review
  - Top thread: "_🎯 Functional Correctness_ \| _🟡 Minor_ \| _⚡ Quick win_" — 13 days old
  - Blocker: Author must post /self-reviewed ecd196e89d61bbcfcbf8b346563577d7f871efcc
- [dashpay/platform#4993 fix(drive-abci): a transition whose version is not active is not a decode failure](https://github.com/dashpay/platform/pull/4993) — 1 unresolved (1 bot) · 19 days stale · 🔴 CI failing · areas: rs-drive-abci · Policy: waiting-self-review
  - Top thread: "🟡 Suggestion: Assert the client-visible metadata in the inactive-version regression test**" — 19 days old
  - Blocker: Author must post /self-reviewed 32b49b0611944a65453ae688d095d4825a1ab18d
- [dashpay/platform#4243 fix(sdk)!: complete encrypted txMetadata parity](https://github.com/dashpay/platform/pull/4243) — ⚠ merge conflict · 📝 draft · 🐢 targets v5.1-dev · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk, github, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#4285 feat(platform-wallet)!: invitation links are AppsFlyer applinks only](https://github.com/dashpay/platform/pull/4285) — ⚠ merge conflict · 🔴 CI failing · 📝 draft · 🐢 targets v5.1-dev · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#4652 feat(drive)!: preserve composite document history across protocol activation](https://github.com/dashpay/platform/pull/4652) — ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · 🐢 targets v5.1-dev · areas: rs-drive, rs-drive-abci, rust-sdk, js-wasm-sdk, fallback · Policy: waiting-author
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/platform#5408 fix(platform-wallet): build version-3 ProUpServTx for Core v24 masternodes](https://github.com/dashpay/platform/pull/5408) — 🔴 CI failing · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed bfe8cdb01f72b7d050c33794a677a95f5138a086
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/platform#5431 fix(drive-abci)!: refuse IdentityCreate identity id and key proof failures before they consume the asset lock](https://github.com/dashpay/platform/pull/5431) — areas: rs-drive-abci, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed afb322c48b40c86ebb289004442cdd7a1d152827

<a id="shumkov-needs-action"></a>
#### Needs action (3)
- [dashpay/rust-dashcore#634 feat(key-wallet-manager): single-map WalletManager with Arc&lt;RwLock&lt;T&gt;&gt; per wallet](https://github.com/dashpay/rust-dashcore/pull/634) — 20 unresolved (20 CodeRabbit) · 189 days stale · ⚠ merge conflict · ✋ changes requested · areas: dash-spv, key-wallet, key-wallet-manager, fallback · Policy: waiting-author
  - Top thread: "_⚠️ Potential issue_ \| _🟡 Minor_" — 189 days old
  - Blocker: coderabbitai requested changes on this head; dismiss the review or push a fix
  - Blocker: coderabbitai left review threads unresolved; resolve them
- [dashpay/platform#5242 chore(skills): show each reviewer their part in /prs](https://github.com/dashpay/platform/pull/5242) — 2 unresolved (1 CodeRabbit, 1 bot) · 13 days stale · 🔴 CI failing · areas: fallback · Policy: waiting-self-review
  - Top thread: "_🎯 Functional Correctness_ \| _🟡 Minor_ \| _⚡ Quick win_" — 13 days old
  - Blocker: Author must post /self-reviewed ecd196e89d61bbcfcbf8b346563577d7f871efcc
- [dashpay/platform#4993 fix(drive-abci): a transition whose version is not active is not a decode failure](https://github.com/dashpay/platform/pull/4993) — 1 unresolved (1 bot) · 19 days stale · 🔴 CI failing · areas: rs-drive-abci · Policy: waiting-self-review
  - Top thread: "🟡 Suggestion: Assert the client-visible metadata in the inactive-version regression test**" — 19 days old
  - Blocker: Author must post /self-reviewed 32b49b0611944a65453ae688d095d4825a1ab18d

<a id="shumkov-unresolved-comments"></a>
#### Unresolved Comments (3)
- [dashpay/rust-dashcore#634 feat(key-wallet-manager): single-map WalletManager with Arc&lt;RwLock&lt;T&gt;&gt; per wallet](https://github.com/dashpay/rust-dashcore/pull/634) — 20 unresolved (20 CodeRabbit) · 189 days stale · ⚠ merge conflict · ✋ changes requested · areas: dash-spv, key-wallet, key-wallet-manager, fallback · Policy: waiting-author
  - Top thread: "_⚠️ Potential issue_ \| _🟡 Minor_" — 189 days old
  - Blocker: coderabbitai requested changes on this head; dismiss the review or push a fix
  - Blocker: coderabbitai left review threads unresolved; resolve them
- [dashpay/platform#5242 chore(skills): show each reviewer their part in /prs](https://github.com/dashpay/platform/pull/5242) — 2 unresolved (1 CodeRabbit, 1 bot) · 13 days stale · 🔴 CI failing · areas: fallback · Policy: waiting-self-review
  - Top thread: "_🎯 Functional Correctness_ \| _🟡 Minor_ \| _⚡ Quick win_" — 13 days old
  - Blocker: Author must post /self-reviewed ecd196e89d61bbcfcbf8b346563577d7f871efcc
- [dashpay/platform#4993 fix(drive-abci): a transition whose version is not active is not a decode failure](https://github.com/dashpay/platform/pull/4993) — 1 unresolved (1 bot) · 19 days stale · 🔴 CI failing · areas: rs-drive-abci · Policy: waiting-self-review
  - Top thread: "🟡 Suggestion: Assert the client-visible metadata in the inactive-version regression test**" — 19 days old
  - Blocker: Author must post /self-reviewed 32b49b0611944a65453ae688d095d4825a1ab18d

<a id="shumkov-ci-failing"></a>
#### CI Failing (1)
- [dashpay/platform#5408 fix(platform-wallet): build version-3 ProUpServTx for Core v24 masternodes](https://github.com/dashpay/platform/pull/5408) — 🔴 CI failing · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed bfe8cdb01f72b7d050c33794a677a95f5138a086
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return

<a id="shumkov-stale"></a>
#### Stale (8)
- [dashpay/platform#4282 fix(dashmate)!: give Debian packages versions apt can order](https://github.com/dashpay/platform/pull/4282) — 6 unresolved (1 CodeRabbit, 5 bot) · 44 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · 🐢 targets v5.1-dev · areas: dashmate, js-wasm-sdk, system-contracts, github, fallback · Policy: waiting-author
  - Top thread: "_🗄️ Data Integrity &amp; Integration_ \| _🟡 Minor_ \| _⚡ Quick win_" — 44 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/platform#3936 chore(drive-abci): update to nested address in SML](https://github.com/dashpay/platform/pull/3936) — 2 unresolved (2 bot) · 117 days stale · ⚠ merge conflict · 📝 draft · 🐢 targets v5.1-dev · areas: rs-drive-abci, rust-sdk, fallback · Policy: draft
  - Top thread: "🟡 Suggestion: V0 reverse synthesizes \`addresses\` from a Core-22 port, masking subsequent legacy port-change diffs after …" — 117 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#4283 fix(dashmate): keep the node up when an image pull fails, and stop reporting success when it did not](https://github.com/dashpay/platform/pull/4283) — 2 unresolved (2 bot) · 38 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · 🐢 targets v5.1-dev · areas: dashmate · Policy: waiting-author
  - Top thread: "🟡 Suggestion: providers.js sanitizer misses U+061C (Arabic letter mark) that sanitizeRemoteText covers**" — 38 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/platform#4660 feat(sdk): expose the document erase and history lifecycle on mobile and FFI](https://github.com/dashpay/platform/pull/4660) — 2 unresolved (2 bot) · 22 days stale · ✋ changes requested · 🔴 CI failing · 🐢 targets keep-history-lifecycle · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, rust-sdk-ffi, kotlin-sdk, fallback
  - Top thread: "🟡 Suggestion: Validate history selector discriminant at the C boundary**" — 22 days old
- [dashpay/platform#4657 feat(platform)!: delete and erase lifecycle for keep-history documents](https://github.com/dashpay/platform/pull/4657) — 2 unresolved (2 bot) · 21 days stale · ✋ changes requested · 🔴 CI failing · 🐢 targets keep-history-storage-v2 · areas: rs-drive, rs-drive-abci, dpp, rust-sdk, js-wasm-sdk, fallback
  - Top thread: "🟡 Suggestion: Deduplicate lifecycle containers at the Drive batch boundary**" — 21 days old
- [dashpay/platform#4243 fix(sdk)!: complete encrypted txMetadata parity](https://github.com/dashpay/platform/pull/4243) — ⚠ merge conflict · 📝 draft · 🐢 targets v5.1-dev · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk, github, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#4285 feat(platform-wallet)!: invitation links are AppsFlyer applinks only](https://github.com/dashpay/platform/pull/4285) — ⚠ merge conflict · 🔴 CI failing · 📝 draft · 🐢 targets v5.1-dev · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#4652 feat(drive)!: preserve composite document history across protocol activation](https://github.com/dashpay/platform/pull/4652) — ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · 🐢 targets v5.1-dev · areas: rs-drive, rs-drive-abci, rust-sdk, js-wasm-sdk, fallback · Policy: waiting-author
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: Proceeded without coderabbitai: no review within the configured window

<a id="shumkov-clean"></a>
#### Clean (1)
- [dashpay/platform#5431 fix(drive-abci)!: refuse IdentityCreate identity id and key proof failures before they consume the asset lock](https://github.com/dashpay/platform/pull/5431) — areas: rs-drive-abci, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed afb322c48b40c86ebb289004442cdd7a1d152827

<a id="shumkov-ready-for-review"></a>
#### Ready for Review (3)
- [dashpay/platform#5396 fix(platform-wallet)!: drain Core balance with restart-safe shielded recovery](https://github.com/dashpay/platform/pull/5396) — by @llbartekll · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk, kotlin-sdk, fallback · Policy: waiting-bots
  - Blocker: coderabbitai has not reported for the current head
- [dashpay/platform#5409 feat(sdk): persist the owner balance a document write's proof carries](https://github.com/dashpay/platform/pull/5409) — by @llbartekll · areas: rs-platform-wallet, rust-sdk, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/platform#5413 ci(swift-sdk): refresh open schema freeze PRs after Swift SDK changes](https://github.com/dashpay/platform/pull/5413) — by @llbartekll · areas: swift-sdk, github · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required

<a id="lklimek"></a>
### @lklimek
<a id="lklimek-open"></a>
#### Open (13)
- [dashpay/platform#5220 fix(platform-wallet)!: replay recorded transaction history on load for every persister](https://github.com/dashpay/platform/pull/5220) — 3 unresolved (3 bot) · 1 days stale · 🐢 targets fix/pr-5126 · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk, fallback
  - Top thread: "🔴 Blocking: Preserve InstantSend finality when decoding FFI history rows**" — 1 days old
- [dashpay/tenderdash#1523 fix(consensus): never extend a relock precommit without ProcessProposal for the round](https://github.com/dashpay/tenderdash/pull/1523) — 1 unresolved (1 bot) · 13 days stale · areas: fallback · Policy: ready-to-merge
  - Top thread: "🟡 Suggestion: Avoid consuming a round-0 timeout in the round-1 assertion**" — 13 days old
  - Blocker: All policy requirements are satisfied
- [dashpay/tenderdash#1527 fix(consensus)!: validate fresh block proposer before prevoting](https://github.com/dashpay/tenderdash/pull/1527) — 1 unresolved (1 bot) · 13 days stale · areas: github, fallback · Policy: ready-for-human
  - Top thread: "🟡 Suggestion: Use a sentinel for fresh-proposal proposer mismatch**" — 13 days old
  - Blocker: Human approval or objection resolution is required
- [dashpay/dash-evo-tool#1074 feat(identity): align Home and reuse identity-bound send flows](https://github.com/dashpay/dash-evo-tool/pull/1074) — areas: fallback · Policy: waiting-bots
  - Blocker: coderabbitai has not reported for the current head
  - Blocker: thepastaclaw has not reported for the current head
- [dashpay/platform#4740 fix(platform-wallet): rescan DashPay contact accounts from the contact request height](https://github.com/dashpay/platform/pull/4740) — 🔴 CI failing · 🐢 targets v5.1-dev · areas: rs-platform-wallet, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 2ae4d4d2109a9e3daaaa983ab8bf914877d7e9dd
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/platform#5150 fix(platform-wallet)!: restore Core spending state and repair persisted accounting](https://github.com/dashpay/platform/pull/5150) — ✋ changes requested · 🐢 targets v5.1-dev · areas: rs-platform-wallet, wallet-storage, fallback · Policy: waiting-bots
  - Blocker: coderabbitai has not reported for the current head
  - Blocker: thepastaclaw has not reported for the current head
- [dashpay/platform#5207 fix(platform-wallet): require only atomic tracked-lock writes for reconciliation](https://github.com/dashpay/platform/pull/5207) — 🐢 targets fix/pr-5126 · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage
- [dashpay/platform#5210 feat(wallet-storage)!: restore complete Core wallet snapshots from SQLite](https://github.com/dashpay/platform/pull/5210) — ⚠ merge conflict · 🐢 targets fix/sqlite-asset-lock-reconciliation · areas: rs-platform-wallet, wallet-storage, fallback
- [dashpay/platform#5306 feat(platform-wallet)!: persist engine spent claims and restore them across backends](https://github.com/dashpay/platform/pull/5306) — ⚠ merge conflict · 📝 draft · 🐢 targets fix/pr-5126 · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk, kotlin-sdk, fallback
- [dashpay/platform#5323 refactor: reject legacy BLS storage formats](https://github.com/dashpay/platform/pull/5323) — ⚠ merge conflict · 📝 draft · 🐢 targets v5.1-dev · areas: rs-drive-abci, dpp, rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk, rust-sdk, rust-sdk-ffi, dashmate, js-wasm-sdk, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#1112 feat(key-wallet): restore externally persisted spent claims](https://github.com/dashpay/rust-dashcore/pull/1112) — ✋ changes requested · 📝 draft · areas: key-wallet, key-wallet-manager, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/tenderdash#1513 fix(service)!: share worker lifecycle for consensus shutdown](https://github.com/dashpay/tenderdash/pull/1513) — ⚠ merge conflict · 🔴 CI failing · areas: fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed b8a6bebd2654465a996515b6303f124052d8a860
- [dashpay/tenderdash#1515 fix(service)!: join background work before shutdown completes](https://github.com/dashpay/tenderdash/pull/1515) — 🔴 CI failing · 📝 draft · areas: fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot

<a id="lklimek-needs-action"></a>
#### Needs action (3)
- [dashpay/tenderdash#1523 fix(consensus): never extend a relock precommit without ProcessProposal for the round](https://github.com/dashpay/tenderdash/pull/1523) — 1 unresolved (1 bot) · 13 days stale · areas: fallback · Policy: ready-to-merge
  - Top thread: "🟡 Suggestion: Avoid consuming a round-0 timeout in the round-1 assertion**" — 13 days old
  - Blocker: All policy requirements are satisfied
- [dashpay/tenderdash#1527 fix(consensus)!: validate fresh block proposer before prevoting](https://github.com/dashpay/tenderdash/pull/1527) — 1 unresolved (1 bot) · 13 days stale · areas: github, fallback · Policy: ready-for-human
  - Top thread: "🟡 Suggestion: Use a sentinel for fresh-proposal proposer mismatch**" — 13 days old
  - Blocker: Human approval or objection resolution is required
- [dashpay/tenderdash#1513 fix(service)!: share worker lifecycle for consensus shutdown](https://github.com/dashpay/tenderdash/pull/1513) — ⚠ merge conflict · 🔴 CI failing · areas: fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed b8a6bebd2654465a996515b6303f124052d8a860

<a id="lklimek-ready-for-human"></a>
#### Ready for human (2)
- [dashpay/tenderdash#1523 fix(consensus): never extend a relock precommit without ProcessProposal for the round](https://github.com/dashpay/tenderdash/pull/1523) — 1 unresolved (1 bot) · 13 days stale · areas: fallback · Policy: ready-to-merge
  - Top thread: "🟡 Suggestion: Avoid consuming a round-0 timeout in the round-1 assertion**" — 13 days old
  - Blocker: All policy requirements are satisfied
- [dashpay/tenderdash#1527 fix(consensus)!: validate fresh block proposer before prevoting](https://github.com/dashpay/tenderdash/pull/1527) — 1 unresolved (1 bot) · 13 days stale · areas: github, fallback · Policy: ready-for-human
  - Top thread: "🟡 Suggestion: Use a sentinel for fresh-proposal proposer mismatch**" — 13 days old
  - Blocker: Human approval or objection resolution is required

<a id="lklimek-unresolved-comments"></a>
#### Unresolved Comments (2)
- [dashpay/tenderdash#1523 fix(consensus): never extend a relock precommit without ProcessProposal for the round](https://github.com/dashpay/tenderdash/pull/1523) — 1 unresolved (1 bot) · 13 days stale · areas: fallback · Policy: ready-to-merge
  - Top thread: "🟡 Suggestion: Avoid consuming a round-0 timeout in the round-1 assertion**" — 13 days old
  - Blocker: All policy requirements are satisfied
- [dashpay/tenderdash#1527 fix(consensus)!: validate fresh block proposer before prevoting](https://github.com/dashpay/tenderdash/pull/1527) — 1 unresolved (1 bot) · 13 days stale · areas: github, fallback · Policy: ready-for-human
  - Top thread: "🟡 Suggestion: Use a sentinel for fresh-proposal proposer mismatch**" — 13 days old
  - Blocker: Human approval or objection resolution is required

<a id="lklimek-ci-failing"></a>
#### CI Failing (1)
- [dashpay/tenderdash#1513 fix(service)!: share worker lifecycle for consensus shutdown](https://github.com/dashpay/tenderdash/pull/1513) — ⚠ merge conflict · 🔴 CI failing · areas: fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed b8a6bebd2654465a996515b6303f124052d8a860

<a id="lklimek-draft"></a>
#### Draft (2)
- [dashpay/rust-dashcore#1112 feat(key-wallet): restore externally persisted spent claims](https://github.com/dashpay/rust-dashcore/pull/1112) — ✋ changes requested · 📝 draft · areas: key-wallet, key-wallet-manager, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/tenderdash#1515 fix(service)!: join background work before shutdown completes](https://github.com/dashpay/tenderdash/pull/1515) — 🔴 CI failing · 📝 draft · areas: fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot

<a id="lklimek-stale"></a>
#### Stale (7)
- [dashpay/platform#5220 fix(platform-wallet)!: replay recorded transaction history on load for every persister](https://github.com/dashpay/platform/pull/5220) — 3 unresolved (3 bot) · 1 days stale · 🐢 targets fix/pr-5126 · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk, fallback
  - Top thread: "🔴 Blocking: Preserve InstantSend finality when decoding FFI history rows**" — 1 days old
- [dashpay/platform#4740 fix(platform-wallet): rescan DashPay contact accounts from the contact request height](https://github.com/dashpay/platform/pull/4740) — 🔴 CI failing · 🐢 targets v5.1-dev · areas: rs-platform-wallet, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 2ae4d4d2109a9e3daaaa983ab8bf914877d7e9dd
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/platform#5150 fix(platform-wallet)!: restore Core spending state and repair persisted accounting](https://github.com/dashpay/platform/pull/5150) — ✋ changes requested · 🐢 targets v5.1-dev · areas: rs-platform-wallet, wallet-storage, fallback · Policy: waiting-bots
  - Blocker: coderabbitai has not reported for the current head
  - Blocker: thepastaclaw has not reported for the current head
- [dashpay/platform#5207 fix(platform-wallet): require only atomic tracked-lock writes for reconciliation](https://github.com/dashpay/platform/pull/5207) — 🐢 targets fix/pr-5126 · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage
- [dashpay/platform#5210 feat(wallet-storage)!: restore complete Core wallet snapshots from SQLite](https://github.com/dashpay/platform/pull/5210) — ⚠ merge conflict · 🐢 targets fix/sqlite-asset-lock-reconciliation · areas: rs-platform-wallet, wallet-storage, fallback
- [dashpay/platform#5306 feat(platform-wallet)!: persist engine spent claims and restore them across backends](https://github.com/dashpay/platform/pull/5306) — ⚠ merge conflict · 📝 draft · 🐢 targets fix/pr-5126 · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk, kotlin-sdk, fallback
- [dashpay/platform#5323 refactor: reject legacy BLS storage formats](https://github.com/dashpay/platform/pull/5323) — ⚠ merge conflict · 📝 draft · 🐢 targets v5.1-dev · areas: rs-drive-abci, dpp, rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk, rust-sdk, rust-sdk-ffi, dashmate, js-wasm-sdk, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot

<a id="lklimek-clean"></a>
#### Clean (1)
- [dashpay/dash-evo-tool#1074 feat(identity): align Home and reuse identity-bound send flows](https://github.com/dashpay/dash-evo-tool/pull/1074) — areas: fallback · Policy: waiting-bots
  - Blocker: coderabbitai has not reported for the current head
  - Blocker: thepastaclaw has not reported for the current head

<a id="lklimek-ready-for-review"></a>
#### Ready for Review (4)
- [dashpay/dash-evo-tool#853 fix: show parsed platform-address transitions](https://github.com/dashpay/dash-evo-tool/pull/853) — by @thepastaclaw · areas: fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
- [dashpay/dash-evo-tool#859 perf: detect system theme off UI thread](https://github.com/dashpay/dash-evo-tool/pull/859) — by @thepastaclaw · areas: fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/platform#5396 fix(platform-wallet)!: drain Core balance with restart-safe shielded recovery](https://github.com/dashpay/platform/pull/5396) — by @llbartekll · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk, kotlin-sdk, fallback · Policy: waiting-bots
  - Blocker: coderabbitai has not reported for the current head
- [dashpay/platform#5409 feat(sdk): persist the owner balance a document write's proof carries](https://github.com/dashpay/platform/pull/5409) — by @llbartekll · areas: rs-platform-wallet, rust-sdk, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return

<a id="romchornyi"></a>
### @romchornyi
<a id="romchornyi-open"></a>
#### Open (7)
- [dashpay/platform#5258 feat(platform-wallet): probe the network about sends with an unknown broadcast outcome](https://github.com/dashpay/platform/pull/5258) — 5 unresolved (5 bot) · 2 days stale · 🔴 CI failing · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, rust-sdk · Policy: waiting-build
  - Top thread: "🟡 Suggestion: Fence queued verdict callbacks against Swift wallet deletion**" — 2 days old
  - Blocker: The build must pass before a human is asked
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/platform#5395 feat(platform-wallet): spend only final coins and report funds waiting on the network](https://github.com/dashpay/platform/pull/5395) — 1 unresolved (1 bot) · 2 days stale · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk, fallback · Policy: ready-for-human
  - Top thread: "✅ **Resolved** at \`dfa8251a\`; see the replies below." — 2 days old
  - Blocker: Human approval or objection resolution is required
- [dashpay/platform#4992 feat(swift-sdk): durable wallet presence marker readable while the device is locked](https://github.com/dashpay/platform/pull/4992) — 🐢 targets v5.1-dev · areas: swift-sdk · Policy: ready-to-merge
  - Blocker: All policy requirements are satisfied
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/platform#5309 fix(sdk): treat AlreadyExists on a retried broadcast as submitted](https://github.com/dashpay/platform/pull/5309) — 📝 draft · ⚠ routing unavailable: no changed-file evidence · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5393 fix(platform-wallet): store the InstantSend lock of a send on its record](https://github.com/dashpay/platform/pull/5393) — areas: rs-platform-wallet, rs-platform-wallet-ffi · Policy: ready-to-merge
  - Blocker: All policy requirements are satisfied
- [dashpay/platform#5419 feat(platform-wallet): delete an index-only document by its values](https://github.com/dashpay/platform/pull/5419) — 🐢 targets v5.1-dev · areas: dpp, rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, rust-sdk · Policy: waiting-author
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/rust-dashcore#1105 fix(key-wallet): recover CoinJoin coins stranded past the gap limit by DashSync](https://github.com/dashpay/rust-dashcore/pull/1105) — ✋ changes requested · 🔴 CI failing · areas: key-wallet · Policy: waiting-author
  - Blocker: Author response is required after the latest human objection

<a id="romchornyi-needs-action"></a>
#### Needs action (2)
- [dashpay/platform#5395 feat(platform-wallet): spend only final coins and report funds waiting on the network](https://github.com/dashpay/platform/pull/5395) — 1 unresolved (1 bot) · 2 days stale · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk, fallback · Policy: ready-for-human
  - Top thread: "✅ **Resolved** at \`dfa8251a\`; see the replies below." — 2 days old
  - Blocker: Human approval or objection resolution is required
- [dashpay/rust-dashcore#1105 fix(key-wallet): recover CoinJoin coins stranded past the gap limit by DashSync](https://github.com/dashpay/rust-dashcore/pull/1105) — ✋ changes requested · 🔴 CI failing · areas: key-wallet · Policy: waiting-author
  - Blocker: Author response is required after the latest human objection

<a id="romchornyi-ready-for-human"></a>
#### Ready for human (3)
- [dashpay/platform#5395 feat(platform-wallet): spend only final coins and report funds waiting on the network](https://github.com/dashpay/platform/pull/5395) — 1 unresolved (1 bot) · 2 days stale · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk, fallback · Policy: ready-for-human
  - Top thread: "✅ **Resolved** at \`dfa8251a\`; see the replies below." — 2 days old
  - Blocker: Human approval or objection resolution is required
- [dashpay/platform#4992 feat(swift-sdk): durable wallet presence marker readable while the device is locked](https://github.com/dashpay/platform/pull/4992) — 🐢 targets v5.1-dev · areas: swift-sdk · Policy: ready-to-merge
  - Blocker: All policy requirements are satisfied
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/platform#5393 fix(platform-wallet): store the InstantSend lock of a send on its record](https://github.com/dashpay/platform/pull/5393) — areas: rs-platform-wallet, rs-platform-wallet-ffi · Policy: ready-to-merge
  - Blocker: All policy requirements are satisfied

<a id="romchornyi-unresolved-comments"></a>
#### Unresolved Comments (2)
- [dashpay/platform#5258 feat(platform-wallet): probe the network about sends with an unknown broadcast outcome](https://github.com/dashpay/platform/pull/5258) — 5 unresolved (5 bot) · 2 days stale · 🔴 CI failing · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, rust-sdk · Policy: waiting-build
  - Top thread: "🟡 Suggestion: Fence queued verdict callbacks against Swift wallet deletion**" — 2 days old
  - Blocker: The build must pass before a human is asked
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/platform#5395 feat(platform-wallet): spend only final coins and report funds waiting on the network](https://github.com/dashpay/platform/pull/5395) — 1 unresolved (1 bot) · 2 days stale · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk, fallback · Policy: ready-for-human
  - Top thread: "✅ **Resolved** at \`dfa8251a\`; see the replies below." — 2 days old
  - Blocker: Human approval or objection resolution is required

<a id="romchornyi-changes-requested"></a>
#### Changes Requested (1)
- [dashpay/rust-dashcore#1105 fix(key-wallet): recover CoinJoin coins stranded past the gap limit by DashSync](https://github.com/dashpay/rust-dashcore/pull/1105) — ✋ changes requested · 🔴 CI failing · areas: key-wallet · Policy: waiting-author
  - Blocker: Author response is required after the latest human objection

<a id="romchornyi-draft"></a>
#### Draft (1)
- [dashpay/platform#5309 fix(sdk): treat AlreadyExists on a retried broadcast as submitted](https://github.com/dashpay/platform/pull/5309) — 📝 draft · ⚠ routing unavailable: no changed-file evidence · Policy: draft
  - Blocker: Draft PR does not occupy a review slot

<a id="romchornyi-stale"></a>
#### Stale (2)
- [dashpay/platform#4992 feat(swift-sdk): durable wallet presence marker readable while the device is locked](https://github.com/dashpay/platform/pull/4992) — 🐢 targets v5.1-dev · areas: swift-sdk · Policy: ready-to-merge
  - Blocker: All policy requirements are satisfied
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/platform#5419 feat(platform-wallet): delete an index-only document by its values](https://github.com/dashpay/platform/pull/5419) — 🐢 targets v5.1-dev · areas: dpp, rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, rust-sdk · Policy: waiting-author
  - Blocker: thepastaclaw left review threads unresolved; resolve them

<a id="romchornyi-clean"></a>
#### Clean (1)
- [dashpay/platform#5393 fix(platform-wallet): store the InstantSend lock of a send on its record](https://github.com/dashpay/platform/pull/5393) — areas: rs-platform-wallet, rs-platform-wallet-ffi · Policy: ready-to-merge
  - Blocker: All policy requirements are satisfied

<a id="romchornyi-ready-for-review"></a>
#### Ready for Review (2)
- [dashpay/platform#5396 fix(platform-wallet)!: drain Core balance with restart-safe shielded recovery](https://github.com/dashpay/platform/pull/5396) — by @llbartekll · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk, kotlin-sdk, fallback · Policy: waiting-bots
  - Blocker: coderabbitai has not reported for the current head
- [dashpay/platform#5409 feat(sdk): persist the owner balance a document write's proof carries](https://github.com/dashpay/platform/pull/5409) — by @llbartekll · areas: rs-platform-wallet, rust-sdk, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return

<a id="pastapastapasta"></a>
### @PastaPastaPasta
<a id="pastapastapasta-open"></a>
#### Open (19)
- [dashpay/rust-dashcore#137 fix: integration tests, continued dashificiation](https://github.com/dashpay/rust-dashcore/pull/137) — 2 unresolved (2 CodeRabbit) · 404 days stale · ⚠ merge conflict · 🔴 CI failing · 🐢 targets v0.40-dev, untouched 307 days · areas: fallback
  - Top thread: "_🛠️ Refactor suggestion_" — 404 days old
- [dashpay/platform#4648 feat(drive-abci)!: state sync via ABCI snapshots with reduced platform state (protocol v15)](https://github.com/dashpay/platform/pull/4648) — 8 unresolved (8 bot) · 9 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · 🐢 targets v5.1-dev · areas: rs-drive, rs-drive-abci, dpp, fallback · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Continue the restored target through consensus and a restart**" — 9 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/platform#5288 feat(platform-wallet)!: support DashPay shielded tips with dedicated accounts](https://github.com/dashpay/platform/pull/5288) — 4 unresolved (4 bot) · 9 days stale · ⚠ merge conflict · 🔴 CI failing · 🐢 targets v5.1-dev · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk, kotlin-sdk, fallback · Policy: waiting-self-review
  - Top thread: "🟡 Suggestion: Restore outstanding tip submissions before enabling another send**" — 9 days old
  - Blocker: Author must post /self-reviewed 74a473f8f46674f504adffd942df79136f279445
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/rust-dashcore#264 feat(dash-spv): Add BIP324 v2 encrypted P2P transport](https://github.com/dashpay/rust-dashcore/pull/264) — 2 unresolved (2 CodeRabbit) · 24 days stale · ⚠ merge conflict · areas: dash-spv, fallback · Policy: waiting-author
  - Top thread: "_📐 Maintainability &amp; Code Quality_ \| _🟡 Minor_ \| _⚡ Quick win_" — 24 days old
  - Blocker: coderabbitai requested changes on this head; dismiss the review or push a fix
- [dashpay/platform#4844 feat(sdk)!: key limits, DIP-14 sub-feature derivation and decode-any-kind for DashPay Connect](https://github.com/dashpay/platform/pull/4844) — 3 unresolved (3 bot) · 0 days stale · ⚠ merge conflict · ✋ changes requested · 🐢 targets v5.1-dev · areas: rs-drive-abci, dpp, rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk, rust-sdk, rust-sdk-ffi, dashmate, js-wasm-sdk, rust-dapi, kotlin-sdk, fallback · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Keep transition authentication semantics owned by DPP**" — 0 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
- [dashpay/platform#4965 test(sdk): cargo-fuzz harness over drive-proof-verifier FromProof](https://github.com/dashpay/platform/pull/4965) — 1 unresolved (1 bot) · 8 days stale · 🐢 targets v5.1-dev · areas: github, fallback · Policy: waiting-self-review
  - Top thread: "🟡 Suggestion: Correct the claim that PV13 reaches the V0 proof decoder**" — 8 days old
  - Blocker: Author must post /self-reviewed 503a6aad352976451e87acef9b6169d57e6f7205
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/platform#5280 perf(platform-wallet)!: reuse a recent recorded-anchor set on the shielded spend path](https://github.com/dashpay/platform/pull/5280) — 2 unresolved (2 bot) · 0 days stale · 🐢 targets v5.1-dev · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk · Policy: waiting-bots
  - Top thread: "🟡 Suggestion: Qualify the same-anchor guarantee for lagging query nodes**" — 0 days old
  - Blocker: coderabbitai has not reported for the current head
- [dashpay/grovedb#1007 build(deps)!: use dashpay/orchard with the faster prover](https://github.com/dashpay/grovedb/pull/1007) — 🔴 CI failing · 📝 draft · areas: fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#4633 feat(sdk): add dash-platform-cxx, a thin CXX shell over dash-sdk for C++ embedders](https://github.com/dashpay/platform/pull/4633) — ⚠ merge conflict · 📝 draft · 🐢 targets v5.1-dev · areas: rust-sdk, github, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5231 fix(dashmate): use complete DKG membership for safe stops](https://github.com/dashpay/platform/pull/5231) — 🔴 CI failing · 🐢 targets v5.1-dev · areas: dashmate · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 3b540cc2b3e40afb0db75e1eb8d0dc7b64f04fda
  - Blocker: Proceeded without coderabbitai: no review within the configured window
  - Blocker: Proceeded without thepastaclaw: no review within the configured window
- [dashpay/platform#5277 perf(platform-wallet): don't hold the shielded store lock across sync network I/O](https://github.com/dashpay/platform/pull/5277) — 🐢 targets v5.1-dev · areas: rs-platform-wallet, rs-platform-wallet-ffi · Policy: waiting-bots
  - Blocker: coderabbitai has not reported for the current head
- [dashpay/platform#5282 perf(platform-wallet)!: prove the shield bundle while waiting for the InstantSend lock](https://github.com/dashpay/platform/pull/5282) — 🐢 targets v5.1-dev · areas: dpp, rs-platform-wallet, rs-platform-wallet-ffi, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 6ca6ab3e2b64e7d5ca4c44c1395d15b6ff8165f2
- [dashpay/platform#5285 feat(platform)!: subscribe to committed state transitions matching document, address, identity, token and contract filters](https://github.com/dashpay/platform/pull/5285) — ⚠ merge conflict · 📝 draft · 🐢 targets v5.1-dev · areas: rs-drive, rs-drive-abci, dpp, rust-sdk, dashmate, rust-dapi, github, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5289 feat(sdk)!: optional BIP-39 passphrase through the mnemonic resolver and wallet creation](https://github.com/dashpay/platform/pull/5289) — 📝 draft · 🐢 targets v5.1-dev · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, rust-sdk-ffi, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5292 feat(dashmate): state sync configuration for tenderdash and drive snapshots](https://github.com/dashpay/platform/pull/5292) — ⚠ merge conflict · 📝 draft · 🐢 targets v5.1-dev · areas: dashmate · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5421 feat(swift-sdk): read a contract group's owner with a proof](https://github.com/dashpay/platform/pull/5421) — 🐢 targets v5.1-dev · areas: swift-sdk, rust-sdk-ffi · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 618af9bd86c7f690b3dd1aa2117228f049a706b2
- [dashpay/rust-dashcore#959 feat(dash-spv): adaptive gap-limit probe escalation for BIP44 discovery](https://github.com/dashpay/rust-dashcore/pull/959) — ⚠ merge conflict · 🔴 CI failing · areas: dash-spv, key-wallet, key-wallet-manager, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 7bcac9b9d89c2c4947080b2e6a15503e97eac9f4
- [dashpay/rust-dashcore#1168 fix(dash-spv)!: only verify ChainLocks with a verified quorum](https://github.com/dashpay/rust-dashcore/pull/1168) — ✋ changes requested · areas: dash-spv · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 12a743c67362da537d7c3acc1ebde68b01f433e0
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/rust-dashcore#1169 fix!: check block transactions against the header merkle root](https://github.com/dashpay/rust-dashcore/pull/1169) — ✋ changes requested · areas: dash-spv, fallback · Policy: waiting-bots
  - Blocker: coderabbitai has not reported for the current head

<a id="pastapastapasta-needs-action"></a>
#### Needs action (4)
- [dashpay/rust-dashcore#264 feat(dash-spv): Add BIP324 v2 encrypted P2P transport](https://github.com/dashpay/rust-dashcore/pull/264) — 2 unresolved (2 CodeRabbit) · 24 days stale · ⚠ merge conflict · areas: dash-spv, fallback · Policy: waiting-author
  - Top thread: "_📐 Maintainability &amp; Code Quality_ \| _🟡 Minor_ \| _⚡ Quick win_" — 24 days old
  - Blocker: coderabbitai requested changes on this head; dismiss the review or push a fix
- [dashpay/rust-dashcore#959 feat(dash-spv): adaptive gap-limit probe escalation for BIP44 discovery](https://github.com/dashpay/rust-dashcore/pull/959) — ⚠ merge conflict · 🔴 CI failing · areas: dash-spv, key-wallet, key-wallet-manager, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 7bcac9b9d89c2c4947080b2e6a15503e97eac9f4
- [dashpay/rust-dashcore#1168 fix(dash-spv)!: only verify ChainLocks with a verified quorum](https://github.com/dashpay/rust-dashcore/pull/1168) — ✋ changes requested · areas: dash-spv · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 12a743c67362da537d7c3acc1ebde68b01f433e0
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/rust-dashcore#1169 fix!: check block transactions against the header merkle root](https://github.com/dashpay/rust-dashcore/pull/1169) — ✋ changes requested · areas: dash-spv, fallback · Policy: waiting-bots
  - Blocker: coderabbitai has not reported for the current head

<a id="pastapastapasta-unresolved-comments"></a>
#### Unresolved Comments (1)
- [dashpay/rust-dashcore#264 feat(dash-spv): Add BIP324 v2 encrypted P2P transport](https://github.com/dashpay/rust-dashcore/pull/264) — 2 unresolved (2 CodeRabbit) · 24 days stale · ⚠ merge conflict · areas: dash-spv, fallback · Policy: waiting-author
  - Top thread: "_📐 Maintainability &amp; Code Quality_ \| _🟡 Minor_ \| _⚡ Quick win_" — 24 days old
  - Blocker: coderabbitai requested changes on this head; dismiss the review or push a fix

<a id="pastapastapasta-changes-requested"></a>
#### Changes Requested (2)
- [dashpay/rust-dashcore#1168 fix(dash-spv)!: only verify ChainLocks with a verified quorum](https://github.com/dashpay/rust-dashcore/pull/1168) — ✋ changes requested · areas: dash-spv · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 12a743c67362da537d7c3acc1ebde68b01f433e0
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/rust-dashcore#1169 fix!: check block transactions against the header merkle root](https://github.com/dashpay/rust-dashcore/pull/1169) — ✋ changes requested · areas: dash-spv, fallback · Policy: waiting-bots
  - Blocker: coderabbitai has not reported for the current head

<a id="pastapastapasta-ci-failing"></a>
#### CI Failing (1)
- [dashpay/rust-dashcore#959 feat(dash-spv): adaptive gap-limit probe escalation for BIP44 discovery](https://github.com/dashpay/rust-dashcore/pull/959) — ⚠ merge conflict · 🔴 CI failing · areas: dash-spv, key-wallet, key-wallet-manager, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 7bcac9b9d89c2c4947080b2e6a15503e97eac9f4

<a id="pastapastapasta-draft"></a>
#### Draft (1)
- [dashpay/grovedb#1007 build(deps)!: use dashpay/orchard with the faster prover](https://github.com/dashpay/grovedb/pull/1007) — 🔴 CI failing · 📝 draft · areas: fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot

<a id="pastapastapasta-stale"></a>
#### Stale (14)
- [dashpay/rust-dashcore#137 fix: integration tests, continued dashificiation](https://github.com/dashpay/rust-dashcore/pull/137) — 2 unresolved (2 CodeRabbit) · 404 days stale · ⚠ merge conflict · 🔴 CI failing · 🐢 targets v0.40-dev, untouched 307 days · areas: fallback
  - Top thread: "_🛠️ Refactor suggestion_" — 404 days old
- [dashpay/platform#4648 feat(drive-abci)!: state sync via ABCI snapshots with reduced platform state (protocol v15)](https://github.com/dashpay/platform/pull/4648) — 8 unresolved (8 bot) · 9 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · 🐢 targets v5.1-dev · areas: rs-drive, rs-drive-abci, dpp, fallback · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Continue the restored target through consensus and a restart**" — 9 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/platform#5288 feat(platform-wallet)!: support DashPay shielded tips with dedicated accounts](https://github.com/dashpay/platform/pull/5288) — 4 unresolved (4 bot) · 9 days stale · ⚠ merge conflict · 🔴 CI failing · 🐢 targets v5.1-dev · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk, kotlin-sdk, fallback · Policy: waiting-self-review
  - Top thread: "🟡 Suggestion: Restore outstanding tip submissions before enabling another send**" — 9 days old
  - Blocker: Author must post /self-reviewed 74a473f8f46674f504adffd942df79136f279445
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/platform#4844 feat(sdk)!: key limits, DIP-14 sub-feature derivation and decode-any-kind for DashPay Connect](https://github.com/dashpay/platform/pull/4844) — 3 unresolved (3 bot) · 0 days stale · ⚠ merge conflict · ✋ changes requested · 🐢 targets v5.1-dev · areas: rs-drive-abci, dpp, rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk, rust-sdk, rust-sdk-ffi, dashmate, js-wasm-sdk, rust-dapi, kotlin-sdk, fallback · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Keep transition authentication semantics owned by DPP**" — 0 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
- [dashpay/platform#4965 test(sdk): cargo-fuzz harness over drive-proof-verifier FromProof](https://github.com/dashpay/platform/pull/4965) — 1 unresolved (1 bot) · 8 days stale · 🐢 targets v5.1-dev · areas: github, fallback · Policy: waiting-self-review
  - Top thread: "🟡 Suggestion: Correct the claim that PV13 reaches the V0 proof decoder**" — 8 days old
  - Blocker: Author must post /self-reviewed 503a6aad352976451e87acef9b6169d57e6f7205
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/platform#5280 perf(platform-wallet)!: reuse a recent recorded-anchor set on the shielded spend path](https://github.com/dashpay/platform/pull/5280) — 2 unresolved (2 bot) · 0 days stale · 🐢 targets v5.1-dev · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk · Policy: waiting-bots
  - Top thread: "🟡 Suggestion: Qualify the same-anchor guarantee for lagging query nodes**" — 0 days old
  - Blocker: coderabbitai has not reported for the current head
- [dashpay/platform#4633 feat(sdk): add dash-platform-cxx, a thin CXX shell over dash-sdk for C++ embedders](https://github.com/dashpay/platform/pull/4633) — ⚠ merge conflict · 📝 draft · 🐢 targets v5.1-dev · areas: rust-sdk, github, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5231 fix(dashmate): use complete DKG membership for safe stops](https://github.com/dashpay/platform/pull/5231) — 🔴 CI failing · 🐢 targets v5.1-dev · areas: dashmate · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 3b540cc2b3e40afb0db75e1eb8d0dc7b64f04fda
  - Blocker: Proceeded without coderabbitai: no review within the configured window
  - Blocker: Proceeded without thepastaclaw: no review within the configured window
- [dashpay/platform#5277 perf(platform-wallet): don't hold the shielded store lock across sync network I/O](https://github.com/dashpay/platform/pull/5277) — 🐢 targets v5.1-dev · areas: rs-platform-wallet, rs-platform-wallet-ffi · Policy: waiting-bots
  - Blocker: coderabbitai has not reported for the current head
- [dashpay/platform#5282 perf(platform-wallet)!: prove the shield bundle while waiting for the InstantSend lock](https://github.com/dashpay/platform/pull/5282) — 🐢 targets v5.1-dev · areas: dpp, rs-platform-wallet, rs-platform-wallet-ffi, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 6ca6ab3e2b64e7d5ca4c44c1395d15b6ff8165f2
- [dashpay/platform#5285 feat(platform)!: subscribe to committed state transitions matching document, address, identity, token and contract filters](https://github.com/dashpay/platform/pull/5285) — ⚠ merge conflict · 📝 draft · 🐢 targets v5.1-dev · areas: rs-drive, rs-drive-abci, dpp, rust-sdk, dashmate, rust-dapi, github, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5289 feat(sdk)!: optional BIP-39 passphrase through the mnemonic resolver and wallet creation](https://github.com/dashpay/platform/pull/5289) — 📝 draft · 🐢 targets v5.1-dev · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, rust-sdk-ffi, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5292 feat(dashmate): state sync configuration for tenderdash and drive snapshots](https://github.com/dashpay/platform/pull/5292) — ⚠ merge conflict · 📝 draft · 🐢 targets v5.1-dev · areas: dashmate · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5421 feat(swift-sdk): read a contract group's owner with a proof](https://github.com/dashpay/platform/pull/5421) — 🐢 targets v5.1-dev · areas: swift-sdk, rust-sdk-ffi · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 618af9bd86c7f690b3dd1aa2117228f049a706b2

<a id="llbartekll"></a>
### @llbartekll
<a id="llbartekll-open"></a>
#### Open (6)
- [dashpay/platform#3560 test(swift-sdk): add testnet identity-discovery UI test](https://github.com/dashpay/platform/pull/3560) — 1 unresolved (1 bot) · 135 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · areas: swift-sdk, github · Policy: waiting-author
  - Top thread: "🔴 Blocking: Carried-forward prior finding (STILL VALID): Docker-setup toggle leaves the cached regtest wallet manager bo…" — 135 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/platform#5396 fix(platform-wallet)!: drain Core balance with restart-safe shielded recovery](https://github.com/dashpay/platform/pull/5396) — areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk, kotlin-sdk, fallback · Policy: waiting-bots
  - Blocker: coderabbitai has not reported for the current head
- [dashpay/platform#5409 feat(sdk): persist the owner balance a document write's proof carries](https://github.com/dashpay/platform/pull/5409) — areas: rs-platform-wallet, rust-sdk, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/platform#5413 ci(swift-sdk): refresh open schema freeze PRs after Swift SDK changes](https://github.com/dashpay/platform/pull/5413) — areas: swift-sdk, github · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
- [dashpay/rust-dashcore#819 feat(key-wallet): CoinJoin sweep + gap-limit recovery; TransactionBuilder drain mode](https://github.com/dashpay/rust-dashcore/pull/819) — ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · areas: key-wallet · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed e406d1cbb86a03503fa41a9d796f25c0bacc3e12
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/rust-dashcore#910 fix(dash-spv): re-anchor incompatible shared storage](https://github.com/dashpay/rust-dashcore/pull/910) — ⚠ merge conflict · 📝 draft · areas: dash-spv · Policy: draft
  - Blocker: Draft PR does not occupy a review slot

<a id="llbartekll-needs-action"></a>
#### Needs action (2)
- [dashpay/platform#3560 test(swift-sdk): add testnet identity-discovery UI test](https://github.com/dashpay/platform/pull/3560) — 1 unresolved (1 bot) · 135 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · areas: swift-sdk, github · Policy: waiting-author
  - Top thread: "🔴 Blocking: Carried-forward prior finding (STILL VALID): Docker-setup toggle leaves the cached regtest wallet manager bo…" — 135 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/rust-dashcore#819 feat(key-wallet): CoinJoin sweep + gap-limit recovery; TransactionBuilder drain mode](https://github.com/dashpay/rust-dashcore/pull/819) — ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · areas: key-wallet · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed e406d1cbb86a03503fa41a9d796f25c0bacc3e12
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return

<a id="llbartekll-ready-for-human"></a>
#### Ready for human (2)
- [dashpay/platform#5409 feat(sdk): persist the owner balance a document write's proof carries](https://github.com/dashpay/platform/pull/5409) — areas: rs-platform-wallet, rust-sdk, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/platform#5413 ci(swift-sdk): refresh open schema freeze PRs after Swift SDK changes](https://github.com/dashpay/platform/pull/5413) — areas: swift-sdk, github · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required

<a id="llbartekll-unresolved-comments"></a>
#### Unresolved Comments (1)
- [dashpay/platform#3560 test(swift-sdk): add testnet identity-discovery UI test](https://github.com/dashpay/platform/pull/3560) — 1 unresolved (1 bot) · 135 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · areas: swift-sdk, github · Policy: waiting-author
  - Top thread: "🔴 Blocking: Carried-forward prior finding (STILL VALID): Docker-setup toggle leaves the cached regtest wallet manager bo…" — 135 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
  - Blocker: Proceeded without coderabbitai: no review within the configured window

<a id="llbartekll-changes-requested"></a>
#### Changes Requested (1)
- [dashpay/rust-dashcore#819 feat(key-wallet): CoinJoin sweep + gap-limit recovery; TransactionBuilder drain mode](https://github.com/dashpay/rust-dashcore/pull/819) — ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · areas: key-wallet · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed e406d1cbb86a03503fa41a9d796f25c0bacc3e12
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return

<a id="llbartekll-draft"></a>
#### Draft (1)
- [dashpay/rust-dashcore#910 fix(dash-spv): re-anchor incompatible shared storage](https://github.com/dashpay/rust-dashcore/pull/910) — ⚠ merge conflict · 📝 draft · areas: dash-spv · Policy: draft
  - Blocker: Draft PR does not occupy a review slot

<a id="llbartekll-clean"></a>
#### Clean (3)
- [dashpay/platform#5396 fix(platform-wallet)!: drain Core balance with restart-safe shielded recovery](https://github.com/dashpay/platform/pull/5396) — areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk, kotlin-sdk, fallback · Policy: waiting-bots
  - Blocker: coderabbitai has not reported for the current head
- [dashpay/platform#5409 feat(sdk): persist the owner balance a document write's proof carries](https://github.com/dashpay/platform/pull/5409) — areas: rs-platform-wallet, rust-sdk, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/platform#5413 ci(swift-sdk): refresh open schema freeze PRs after Swift SDK changes](https://github.com/dashpay/platform/pull/5413) — areas: swift-sdk, github · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required

<a id="xdustinface"></a>
### @xdustinface
<a id="xdustinface-open"></a>
#### Open (8)
- [dashpay/rust-dashcore#856 feat(dash-spv): force resync to re-anchor a running client at a lower checkpoint](https://github.com/dashpay/rust-dashcore/pull/856) — 1 unresolved (1 human) · 85 days stale · ⚠ merge conflict · 🔴 CI failing · areas: dash-spv · Policy: waiting-self-review
  - Top thread: "why do we have a periodic check here??" — 85 days old
  - Blocker: Author must post /self-reviewed 7282172a3bb4a0d9d85a40cf70b4dd111995f96b
- [dashpay/rust-dashcore#400 chore: remove BIP141 (SegWit) and BIP341 (Taproot) support](https://github.com/dashpay/rust-dashcore/pull/400) — ⚠ merge conflict · 🔴 CI failing · 📝 draft · 🐢 untouched 147 days · areas: dash-spv, key-wallet, github, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#541 ci: integrate Codecov test analytics via \`cargo-nextest\`](https://github.com/dashpay/rust-dashcore/pull/541) — ⚠ merge conflict · 📝 draft · areas: github, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#720 fix(dash-spv): split peer \`TcpStream\` and add per-peer writer task](https://github.com/dashpay/rust-dashcore/pull/720) — ⚠ merge conflict · 🔴 CI failing · 📝 draft · 🐢 untouched 147 days · areas: dash-spv, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#776 chore: remove FFI layer](https://github.com/dashpay/rust-dashcore/pull/776) — ⚠ merge conflict · 📝 draft · areas: dash-spv, key-wallet, github, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#799 feat: validate masternode list merkle root](https://github.com/dashpay/rust-dashcore/pull/799) — ⚠ merge conflict · 📝 draft · areas: fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#803 feat(dash-spv): block locator + staged fork detection](https://github.com/dashpay/rust-dashcore/pull/803) — ⚠ merge conflict · 📝 draft · areas: dash-spv, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#849 feat(dash-spv): add \`--birth-height\` CLI flag](https://github.com/dashpay/rust-dashcore/pull/849) — areas: dash-spv · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 9b1e709fd8e0bfab0dd69f7c86fd6b7d2e585fbe

<a id="xdustinface-needs-action"></a>
#### Needs action (1)
- [dashpay/rust-dashcore#856 feat(dash-spv): force resync to re-anchor a running client at a lower checkpoint](https://github.com/dashpay/rust-dashcore/pull/856) — 1 unresolved (1 human) · 85 days stale · ⚠ merge conflict · 🔴 CI failing · areas: dash-spv · Policy: waiting-self-review
  - Top thread: "why do we have a periodic check here??" — 85 days old
  - Blocker: Author must post /self-reviewed 7282172a3bb4a0d9d85a40cf70b4dd111995f96b

<a id="xdustinface-unresolved-comments"></a>
#### Unresolved Comments (1)
- [dashpay/rust-dashcore#856 feat(dash-spv): force resync to re-anchor a running client at a lower checkpoint](https://github.com/dashpay/rust-dashcore/pull/856) — 1 unresolved (1 human) · 85 days stale · ⚠ merge conflict · 🔴 CI failing · areas: dash-spv · Policy: waiting-self-review
  - Top thread: "why do we have a periodic check here??" — 85 days old
  - Blocker: Author must post /self-reviewed 7282172a3bb4a0d9d85a40cf70b4dd111995f96b

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
- [dashpay/rust-dashcore#400 chore: remove BIP141 (SegWit) and BIP341 (Taproot) support](https://github.com/dashpay/rust-dashcore/pull/400) — ⚠ merge conflict · 🔴 CI failing · 📝 draft · 🐢 untouched 147 days · areas: dash-spv, key-wallet, github, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#720 fix(dash-spv): split peer \`TcpStream\` and add per-peer writer task](https://github.com/dashpay/rust-dashcore/pull/720) — ⚠ merge conflict · 🔴 CI failing · 📝 draft · 🐢 untouched 147 days · areas: dash-spv, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot

<a id="xdustinface-clean"></a>
#### Clean (1)
- [dashpay/rust-dashcore#849 feat(dash-spv): add \`--birth-height\` CLI flag](https://github.com/dashpay/rust-dashcore/pull/849) — areas: dash-spv · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 9b1e709fd8e0bfab0dd69f7c86fd6b7d2e585fbe

<a id="hashengineering"></a>
### @HashEngineering
<a id="hashengineering-open"></a>
#### Open (4)
- [dashpay/platform#5259 fix(kotlin-sdk): port #4638 to Android — credit verdicts at save time and the post-sync TXO reconcile](https://github.com/dashpay/platform/pull/5259) — 5 unresolved (5 bot) · 6 days stale · 🔴 CI failing · 🐢 targets v5.1-dev · areas: kotlin-sdk, fallback · Policy: ready-for-human
  - Top thread: "✅ **Resolved** at \`0c674bce\`; see the replies below." — 6 days old
  - Blocker: Human approval or objection resolution is required
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/platform#5352 fix(platform-wallet-storage): open the wallet store on Android](https://github.com/dashpay/platform/pull/5352) — 1 unresolved (1 CodeRabbit) · 5 days stale · 📝 draft · areas: wallet-storage · Policy: draft
  - Top thread: "🎯 Functional Correctness** \| **🟡 Minor** \| **⚡ Quick win**" — 5 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5256 fix(platform-wallet): build our receiving account for one-way DashPay contacts](https://github.com/dashpay/platform/pull/5256) — 1 unresolved (1 bot) · 1 days stale · 🐢 targets v5.1-dev · areas: rs-platform-wallet · Policy: ready-for-human
  - Top thread: "🟡 Suggestion: Pin recovery when only the external account survives**" — 1 days old
  - Blocker: Human approval or objection resolution is required
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/platform#5026 fix(platform-wallet)!: persist DashPay coreHeight backfill coverage so a relaunch resumes instead of rewinding again](https://github.com/dashpay/platform/pull/5026) — ✋ changes requested · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, kotlin-sdk, fallback · Policy: waiting-bots
  - Blocker: coderabbitai has not reported for the current head

<a id="hashengineering-needs-action"></a>
#### Needs action (1)
- [dashpay/platform#5026 fix(platform-wallet)!: persist DashPay coreHeight backfill coverage so a relaunch resumes instead of rewinding again](https://github.com/dashpay/platform/pull/5026) — ✋ changes requested · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, kotlin-sdk, fallback · Policy: waiting-bots
  - Blocker: coderabbitai has not reported for the current head

<a id="hashengineering-ready-for-human"></a>
#### Ready for human (2)
- [dashpay/platform#5259 fix(kotlin-sdk): port #4638 to Android — credit verdicts at save time and the post-sync TXO reconcile](https://github.com/dashpay/platform/pull/5259) — 5 unresolved (5 bot) · 6 days stale · 🔴 CI failing · 🐢 targets v5.1-dev · areas: kotlin-sdk, fallback · Policy: ready-for-human
  - Top thread: "✅ **Resolved** at \`0c674bce\`; see the replies below." — 6 days old
  - Blocker: Human approval or objection resolution is required
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/platform#5256 fix(platform-wallet): build our receiving account for one-way DashPay contacts](https://github.com/dashpay/platform/pull/5256) — 1 unresolved (1 bot) · 1 days stale · 🐢 targets v5.1-dev · areas: rs-platform-wallet · Policy: ready-for-human
  - Top thread: "🟡 Suggestion: Pin recovery when only the external account survives**" — 1 days old
  - Blocker: Human approval or objection resolution is required
  - Blocker: Proceeded without coderabbitai: no review within the configured window

<a id="hashengineering-changes-requested"></a>
#### Changes Requested (1)
- [dashpay/platform#5026 fix(platform-wallet)!: persist DashPay coreHeight backfill coverage so a relaunch resumes instead of rewinding again](https://github.com/dashpay/platform/pull/5026) — ✋ changes requested · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, kotlin-sdk, fallback · Policy: waiting-bots
  - Blocker: coderabbitai has not reported for the current head

<a id="hashengineering-draft"></a>
#### Draft (1)
- [dashpay/platform#5352 fix(platform-wallet-storage): open the wallet store on Android](https://github.com/dashpay/platform/pull/5352) — 1 unresolved (1 CodeRabbit) · 5 days stale · 📝 draft · areas: wallet-storage · Policy: draft
  - Top thread: "🎯 Functional Correctness** \| **🟡 Minor** \| **⚡ Quick win**" — 5 days old
  - Blocker: Draft PR does not occupy a review slot

<a id="hashengineering-stale"></a>
#### Stale (2)
- [dashpay/platform#5259 fix(kotlin-sdk): port #4638 to Android — credit verdicts at save time and the post-sync TXO reconcile](https://github.com/dashpay/platform/pull/5259) — 5 unresolved (5 bot) · 6 days stale · 🔴 CI failing · 🐢 targets v5.1-dev · areas: kotlin-sdk, fallback · Policy: ready-for-human
  - Top thread: "✅ **Resolved** at \`0c674bce\`; see the replies below." — 6 days old
  - Blocker: Human approval or objection resolution is required
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/platform#5256 fix(platform-wallet): build our receiving account for one-way DashPay contacts](https://github.com/dashpay/platform/pull/5256) — 1 unresolved (1 bot) · 1 days stale · 🐢 targets v5.1-dev · areas: rs-platform-wallet · Policy: ready-for-human
  - Top thread: "🟡 Suggestion: Pin recovery when only the external account survives**" — 1 days old
  - Blocker: Human approval or objection resolution is required
  - Blocker: Proceeded without coderabbitai: no review within the configured window

<a id="hashengineering-ready-for-review"></a>
#### Ready for Review (3)
- [dashpay/platform#5393 fix(platform-wallet): store the InstantSend lock of a send on its record](https://github.com/dashpay/platform/pull/5393) — by @romchornyi · areas: rs-platform-wallet, rs-platform-wallet-ffi · Policy: ready-to-merge
  - Blocker: All policy requirements are satisfied
- [dashpay/platform#5396 fix(platform-wallet)!: drain Core balance with restart-safe shielded recovery](https://github.com/dashpay/platform/pull/5396) — by @llbartekll · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk, kotlin-sdk, fallback · Policy: waiting-bots
  - Blocker: coderabbitai has not reported for the current head
- [dashpay/platform#5409 feat(sdk): persist the owner balance a document write's proof carries](https://github.com/dashpay/platform/pull/5409) — by @llbartekll · areas: rs-platform-wallet, rust-sdk, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return

<a id="zocolini"></a>
### @ZocoLini
<a id="zocolini-open"></a>
#### Open (5)
- [dashpay/rust-dashcore#375 refactor(dash-spv): checkpoint rewrite](https://github.com/dashpay/rust-dashcore/pull/375) — 2 unresolved (2 CodeRabbit) · 266 days stale · ⚠ merge conflict · 📝 draft · areas: dash-spv, fallback · Policy: draft
  - Top thread: "_⚠️ Potential issue_ \| _🟡 Minor_" — 266 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#902 refactor(dash-spv): network manager refactor and sync pipelines optimized ](https://github.com/dashpay/rust-dashcore/pull/902) — ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · 📝 draft · areas: dash-spv, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#921 fix(dash-spv): sync backfill](https://github.com/dashpay/rust-dashcore/pull/921) — ⚠ merge conflict · 📝 draft · areas: dash-spv, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#1081 feat(dash-spv): restore the validated rotation cycles from the replayed engine](https://github.com/dashpay/rust-dashcore/pull/1081) — ⚠ merge conflict · 📝 draft · areas: dash-spv · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#1161 feat(dash-spv)!: validate header difficulty against Dark Gravity Wave](https://github.com/dashpay/rust-dashcore/pull/1161) — ⚠ merge conflict · 📝 draft · areas: dash-spv, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot

<a id="zocolini-draft"></a>
#### Draft (5)
- [dashpay/rust-dashcore#375 refactor(dash-spv): checkpoint rewrite](https://github.com/dashpay/rust-dashcore/pull/375) — 2 unresolved (2 CodeRabbit) · 266 days stale · ⚠ merge conflict · 📝 draft · areas: dash-spv, fallback · Policy: draft
  - Top thread: "_⚠️ Potential issue_ \| _🟡 Minor_" — 266 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#902 refactor(dash-spv): network manager refactor and sync pipelines optimized ](https://github.com/dashpay/rust-dashcore/pull/902) — ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · 📝 draft · areas: dash-spv, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#921 fix(dash-spv): sync backfill](https://github.com/dashpay/rust-dashcore/pull/921) — ⚠ merge conflict · 📝 draft · areas: dash-spv, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#1081 feat(dash-spv): restore the validated rotation cycles from the replayed engine](https://github.com/dashpay/rust-dashcore/pull/1081) — ⚠ merge conflict · 📝 draft · areas: dash-spv · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#1161 feat(dash-spv)!: validate header difficulty against Dark Gravity Wave](https://github.com/dashpay/rust-dashcore/pull/1161) — ⚠ merge conflict · 📝 draft · areas: dash-spv, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot

<a id="zocolini-ready-for-review"></a>
#### Ready for Review (3)
- [dashpay/platform#5393 fix(platform-wallet): store the InstantSend lock of a send on its record](https://github.com/dashpay/platform/pull/5393) — by @romchornyi · areas: rs-platform-wallet, rs-platform-wallet-ffi · Policy: ready-to-merge
  - Blocker: All policy requirements are satisfied
- [dashpay/platform#5396 fix(platform-wallet)!: drain Core balance with restart-safe shielded recovery](https://github.com/dashpay/platform/pull/5396) — by @llbartekll · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk, kotlin-sdk, fallback · Policy: waiting-bots
  - Blocker: coderabbitai has not reported for the current head
- [dashpay/platform#5409 feat(sdk): persist the owner balance a document write's proof carries](https://github.com/dashpay/platform/pull/5409) — by @llbartekll · areas: rs-platform-wallet, rust-sdk, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return

<a id="ktechmidas"></a>
### @ktechmidas
<a id="ktechmidas-open"></a>
#### Open (6)
- [dashpay/platform#5188 ci: tolerate optional S3 cache export outages on chore/bump-rust-dashcore-secp-033](https://github.com/dashpay/platform/pull/5188) — 1 unresolved (1 bot) · 15 days stale · 🐢 targets chore/bump-rust-dashcore-secp-033 · areas: github
  - Top thread: "🟡 Suggestion: New regression tests are not executed by any CI workflow on this branch**" — 15 days old
- [dashpay/platform#5186 ci: tolerate optional S3 cache export outages on v5.0-dev](https://github.com/dashpay/platform/pull/5186) — 🔴 CI failing · 🐢 targets v6.0-dev · areas: github · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 843052c2dce26a80065c15b1219bbc8211264161
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/platform#5187 ci: tolerate optional S3 cache export outages on v4.3-dev](https://github.com/dashpay/platform/pull/5187) — 🐢 targets v5.1-dev · areas: github · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed fbbf13ee41f8bd8b9f03f1dc5afec6a619967d97
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/platform#5198 ci: bound default Binaryen optimizer threads in NPM builds](https://github.com/dashpay/platform/pull/5198) — 🔴 CI failing · 🐢 targets v6.0-dev · areas: github · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 23b41b477bc038965b686b4a75412b1c21fc6d98
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/platform#5204 fix(release): install verified cargo-binstall binary directly](https://github.com/dashpay/platform/pull/5204) — 🐢 targets v5.1-dev · areas: fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 63455a2563fd7271fb11083d3b0a89bdf01f3982
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/platform#5205 fix(release): install verified cargo-binstall binary directly](https://github.com/dashpay/platform/pull/5205) — 🐢 targets v6.0-dev · areas: fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 8a8ac2c788b5338416e18045001d9115ba3ac80a
  - Blocker: Proceeded without coderabbitai: no review within the configured window

<a id="ktechmidas-stale"></a>
#### Stale (6)
- [dashpay/platform#5188 ci: tolerate optional S3 cache export outages on chore/bump-rust-dashcore-secp-033](https://github.com/dashpay/platform/pull/5188) — 1 unresolved (1 bot) · 15 days stale · 🐢 targets chore/bump-rust-dashcore-secp-033 · areas: github
  - Top thread: "🟡 Suggestion: New regression tests are not executed by any CI workflow on this branch**" — 15 days old
- [dashpay/platform#5186 ci: tolerate optional S3 cache export outages on v5.0-dev](https://github.com/dashpay/platform/pull/5186) — 🔴 CI failing · 🐢 targets v6.0-dev · areas: github · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 843052c2dce26a80065c15b1219bbc8211264161
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/platform#5187 ci: tolerate optional S3 cache export outages on v4.3-dev](https://github.com/dashpay/platform/pull/5187) — 🐢 targets v5.1-dev · areas: github · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed fbbf13ee41f8bd8b9f03f1dc5afec6a619967d97
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/platform#5198 ci: bound default Binaryen optimizer threads in NPM builds](https://github.com/dashpay/platform/pull/5198) — 🔴 CI failing · 🐢 targets v6.0-dev · areas: github · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 23b41b477bc038965b686b4a75412b1c21fc6d98
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/platform#5204 fix(release): install verified cargo-binstall binary directly](https://github.com/dashpay/platform/pull/5204) — 🐢 targets v5.1-dev · areas: fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 63455a2563fd7271fb11083d3b0a89bdf01f3982
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/platform#5205 fix(release): install verified cargo-binstall binary directly](https://github.com/dashpay/platform/pull/5205) — 🐢 targets v6.0-dev · areas: fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 8a8ac2c788b5338416e18045001d9115ba3ac80a
  - Blocker: Proceeded without coderabbitai: no review within the configured window

<a id="ktechmidas-ready-for-review"></a>
#### Ready for Review (1)
- [dashpay/platform#5413 ci(swift-sdk): refresh open schema freeze PRs after Swift SDK changes](https://github.com/dashpay/platform/pull/5413) — by @llbartekll · areas: swift-sdk, github · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required

<a id="thepastaclaw"></a>
### @thepastaclaw
<a id="thepastaclaw-open"></a>
#### Open (3)
- [dashpay/dash-evo-tool#853 fix: show parsed platform-address transitions](https://github.com/dashpay/dash-evo-tool/pull/853) — areas: fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
- [dashpay/dash-evo-tool#859 perf: detect system theme off UI thread](https://github.com/dashpay/dash-evo-tool/pull/859) — areas: fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/platform#4298 fix(dashmate): load ZeroSSL config in force mode](https://github.com/dashpay/platform/pull/4298) — 🐢 targets v5.1-dev · areas: dashmate · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
  - Blocker: Proceeded without coderabbitai: no review within the configured window

<a id="thepastaclaw-ready-for-human"></a>
#### Ready for human (3)
- [dashpay/dash-evo-tool#853 fix: show parsed platform-address transitions](https://github.com/dashpay/dash-evo-tool/pull/853) — areas: fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
- [dashpay/dash-evo-tool#859 perf: detect system theme off UI thread](https://github.com/dashpay/dash-evo-tool/pull/859) — areas: fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/platform#4298 fix(dashmate): load ZeroSSL config in force mode](https://github.com/dashpay/platform/pull/4298) — 🐢 targets v5.1-dev · areas: dashmate · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
  - Blocker: Proceeded without coderabbitai: no review within the configured window

<a id="thepastaclaw-stale"></a>
#### Stale (1)
- [dashpay/platform#4298 fix(dashmate): load ZeroSSL config in force mode](https://github.com/dashpay/platform/pull/4298) — 🐢 targets v5.1-dev · areas: dashmate · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
  - Blocker: Proceeded without coderabbitai: no review within the configured window

<a id="thepastaclaw-clean"></a>
#### Clean (2)
- [dashpay/dash-evo-tool#853 fix: show parsed platform-address transitions](https://github.com/dashpay/dash-evo-tool/pull/853) — areas: fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
- [dashpay/dash-evo-tool#859 perf: detect system theme off UI thread](https://github.com/dashpay/dash-evo-tool/pull/859) — areas: fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return

<a id="thepastaclaw-ready-for-review"></a>
#### Ready for Review (1)
- [dashpay/platform#5396 fix(platform-wallet)!: drain Core balance with restart-safe shielded recovery](https://github.com/dashpay/platform/pull/5396) — by @llbartekll · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk, kotlin-sdk, fallback · Policy: waiting-bots
  - Blocker: coderabbitai has not reported for the current head

<a id="claudius-maginificent"></a>
### @Claudius-Maginificent
<a id="claudius-maginificent-open"></a>
#### Open (11)
- [dashpay/platform#3549 test(platform-wallet): e2e framework + full test suite — triage pins, Found-*/PA-* guards, fail-closed persist, Stage-2 merge](https://github.com/dashpay/platform/pull/3549) — 9 unresolved (9 bot) · 165 days stale · ⚠ merge conflict · 📝 draft · 🐢 targets v5.1-dev · areas: rs-drive, rs-drive-abci, rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk, rust-sdk, github, fallback · Policy: draft
  - Top thread: "🟡 Suggestion: \`from_seed_for_identity\` is misleadingly named, half-functional, and unused**" — 165 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/dash-evo-tool#1082 fix(identity): clear the send notice on leaving the form and keep the Receive code current](https://github.com/dashpay/dash-evo-tool/pull/1082) — 🐢 targets ui/identity-screen · areas: fallback
- [dashpay/dash-evo-tool#1089 refactor(ui): draw every QR code through one shared widget](https://github.com/dashpay/dash-evo-tool/pull/1089) — 🐢 targets fix/identity-send-form-followups · areas: fallback
- [dashpay/dash-evo-tool#1090 fix(identity): keep Home's data on screen when returning to the hub](https://github.com/dashpay/dash-evo-tool/pull/1090) — ⚠ merge conflict · 🐢 targets fix/identity-send-form-followups · areas: fallback
- [dashpay/dash-evo-tool#1091 fix(ui): deliver each task's result to the screen that started it](https://github.com/dashpay/dash-evo-tool/pull/1091) — ✋ changes requested · 🐢 targets fix/identity-send-form-followups · areas: fallback
- [dashpay/platform#5312 feat(sdk)!: read how many times a masternode has voted on a contested resource](https://github.com/dashpay/platform/pull/5312) — 🐢 targets v5.1-dev · areas: rs-drive, rs-drive-abci, swift-sdk, rust-sdk, rust-sdk-ffi, js-wasm-sdk, kotlin-sdk, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/platform#5384 fix(platform-wallet)!: persist chain truncations so a stored wallet follows a reorg](https://github.com/dashpay/platform/pull/5384) — 🐢 targets fix/pr-5126 · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk, kotlin-sdk, fallback
- [dashpay/platform#5407 ci: run platform-wallet-storage tests named "shield" on the full workspace path](https://github.com/dashpay/platform/pull/5407) — 🔴 CI failing · 🐢 targets v5.1-dev · areas: github · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/platform#5417 fix(swift-sdk): reconcile SwiftData transaction accounting per wallet](https://github.com/dashpay/platform/pull/5417) — 🐢 targets fix/pr-5126 · areas: swift-sdk
- [dashpay/platform#5425 fix(platform-wallet)!: verify Platform proofs only against verified SPV quorum keys](https://github.com/dashpay/platform/pull/5425) — 🐢 targets v5.1-dev · areas: rs-platform-wallet, fallback · Policy: waiting-bots
  - Blocker: coderabbitai has not reported for the current head
- [dashpay/platform#5426 refactor(platform-wallet): wake the SPV readiness wait on events instead of polling](https://github.com/dashpay/platform/pull/5426) — 🐢 targets v5.1-dev · areas: rs-platform-wallet, fallback · Policy: waiting-bots
  - Blocker: coderabbitai has not reported for the current head

<a id="claudius-maginificent-ready-for-human"></a>
#### Ready for human (2)
- [dashpay/platform#5312 feat(sdk)!: read how many times a masternode has voted on a contested resource](https://github.com/dashpay/platform/pull/5312) — 🐢 targets v5.1-dev · areas: rs-drive, rs-drive-abci, swift-sdk, rust-sdk, rust-sdk-ffi, js-wasm-sdk, kotlin-sdk, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/platform#5407 ci: run platform-wallet-storage tests named "shield" on the full workspace path](https://github.com/dashpay/platform/pull/5407) — 🔴 CI failing · 🐢 targets v5.1-dev · areas: github · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
  - Blocker: Proceeded without coderabbitai: no review within the configured window

<a id="claudius-maginificent-stale"></a>
#### Stale (11)
- [dashpay/platform#3549 test(platform-wallet): e2e framework + full test suite — triage pins, Found-*/PA-* guards, fail-closed persist, Stage-2 merge](https://github.com/dashpay/platform/pull/3549) — 9 unresolved (9 bot) · 165 days stale · ⚠ merge conflict · 📝 draft · 🐢 targets v5.1-dev · areas: rs-drive, rs-drive-abci, rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk, rust-sdk, github, fallback · Policy: draft
  - Top thread: "🟡 Suggestion: \`from_seed_for_identity\` is misleadingly named, half-functional, and unused**" — 165 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/dash-evo-tool#1082 fix(identity): clear the send notice on leaving the form and keep the Receive code current](https://github.com/dashpay/dash-evo-tool/pull/1082) — 🐢 targets ui/identity-screen · areas: fallback
- [dashpay/dash-evo-tool#1089 refactor(ui): draw every QR code through one shared widget](https://github.com/dashpay/dash-evo-tool/pull/1089) — 🐢 targets fix/identity-send-form-followups · areas: fallback
- [dashpay/dash-evo-tool#1090 fix(identity): keep Home's data on screen when returning to the hub](https://github.com/dashpay/dash-evo-tool/pull/1090) — ⚠ merge conflict · 🐢 targets fix/identity-send-form-followups · areas: fallback
- [dashpay/dash-evo-tool#1091 fix(ui): deliver each task's result to the screen that started it](https://github.com/dashpay/dash-evo-tool/pull/1091) — ✋ changes requested · 🐢 targets fix/identity-send-form-followups · areas: fallback
- [dashpay/platform#5312 feat(sdk)!: read how many times a masternode has voted on a contested resource](https://github.com/dashpay/platform/pull/5312) — 🐢 targets v5.1-dev · areas: rs-drive, rs-drive-abci, swift-sdk, rust-sdk, rust-sdk-ffi, js-wasm-sdk, kotlin-sdk, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/platform#5384 fix(platform-wallet)!: persist chain truncations so a stored wallet follows a reorg](https://github.com/dashpay/platform/pull/5384) — 🐢 targets fix/pr-5126 · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk, kotlin-sdk, fallback
- [dashpay/platform#5407 ci: run platform-wallet-storage tests named "shield" on the full workspace path](https://github.com/dashpay/platform/pull/5407) — 🔴 CI failing · 🐢 targets v5.1-dev · areas: github · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/platform#5417 fix(swift-sdk): reconcile SwiftData transaction accounting per wallet](https://github.com/dashpay/platform/pull/5417) — 🐢 targets fix/pr-5126 · areas: swift-sdk
- [dashpay/platform#5425 fix(platform-wallet)!: verify Platform proofs only against verified SPV quorum keys](https://github.com/dashpay/platform/pull/5425) — 🐢 targets v5.1-dev · areas: rs-platform-wallet, fallback · Policy: waiting-bots
  - Blocker: coderabbitai has not reported for the current head
- [dashpay/platform#5426 refactor(platform-wallet): wake the SPV readiness wait on events instead of polling](https://github.com/dashpay/platform/pull/5426) — 🐢 targets v5.1-dev · areas: rs-platform-wallet, fallback · Policy: waiting-bots
  - Blocker: coderabbitai has not reported for the current head

<a id="infraclaw-dash"></a>
### @infraclaw-dash
<a id="infraclaw-dash-open"></a>
#### Open (3)
- [dashpay/platform#3958 ci: add Platform testnet sync status reporting](https://github.com/dashpay/platform/pull/3958) — 2 unresolved (2 bot) · 112 days stale · ⚠ merge conflict · 🐢 targets v5.1-dev · areas: github, fallback · Policy: waiting-author
  - Top thread: "🟡 Suggestion: workflow_dispatch lets any repo writer forge a sync status**" — 112 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: coderabbitai left review threads unresolved; resolve them
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5167 ci: align secp feature-base workflows with provisioned runner images](https://github.com/dashpay/platform/pull/5167) — 3 unresolved (3 bot) · 15 days stale · ⚠ merge conflict · 🐢 targets chore/bump-rust-dashcore-secp-033 · areas: github
  - Top thread: "🟡 Suggestion: Contract-failure error points to a doc file missing from this branch**" — 15 days old
- [dashpay/platform#5166 ci: adopt verified Rust and Kotlin runner images on v4.3](https://github.com/dashpay/platform/pull/5166) — ⚠ merge conflict · 🐢 targets v5.1-dev · areas: github · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required

<a id="infraclaw-dash-ready-for-human"></a>
#### Ready for human (1)
- [dashpay/platform#5166 ci: adopt verified Rust and Kotlin runner images on v4.3](https://github.com/dashpay/platform/pull/5166) — ⚠ merge conflict · 🐢 targets v5.1-dev · areas: github · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required

<a id="infraclaw-dash-stale"></a>
#### Stale (3)
- [dashpay/platform#3958 ci: add Platform testnet sync status reporting](https://github.com/dashpay/platform/pull/3958) — 2 unresolved (2 bot) · 112 days stale · ⚠ merge conflict · 🐢 targets v5.1-dev · areas: github, fallback · Policy: waiting-author
  - Top thread: "🟡 Suggestion: workflow_dispatch lets any repo writer forge a sync status**" — 112 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: coderabbitai left review threads unresolved; resolve them
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5167 ci: align secp feature-base workflows with provisioned runner images](https://github.com/dashpay/platform/pull/5167) — 3 unresolved (3 bot) · 15 days stale · ⚠ merge conflict · 🐢 targets chore/bump-rust-dashcore-secp-033 · areas: github
  - Top thread: "🟡 Suggestion: Contract-failure error points to a doc file missing from this branch**" — 15 days old
- [dashpay/platform#5166 ci: adopt verified Rust and Kotlin runner images on v4.3](https://github.com/dashpay/platform/pull/5166) — ⚠ merge conflict · 🐢 targets v5.1-dev · areas: github · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required

<a id="bfoss765"></a>
### @bfoss765
<a id="bfoss765-open"></a>
#### Open (2)
- [dashpay/platform#4313 feat(kotlin-sdk): one-time Orchard key shielded-invite API (inviter + claim)](https://github.com/dashpay/platform/pull/4313) — 2 unresolved (2 bot) · 44 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · 🐢 targets v5.1-dev · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk, fallback · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Preserve claim-binding mismatch semantics across the FFI boundary**" — 44 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/platform#4312 feat(platform-wallet): multi-output shielded transfers + output-aware fee predictor](https://github.com/dashpay/platform/pull/4312) — 1 unresolved (1 bot) · 44 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · 🐢 targets v5.1-dev · areas: dpp, rs-platform-wallet, rs-platform-wallet-ffi, kotlin-sdk, fallback · Policy: waiting-self-review
  - Top thread: "🟡 Suggestion: Keep envelope-aware action counting behind the crate boundary**" — 44 days old
  - Blocker: Author must post /self-reviewed da932f96a8670c641065954de8b58752e99233a3
  - Blocker: Proceeded without coderabbitai: no review within the configured window

<a id="bfoss765-stale"></a>
#### Stale (2)
- [dashpay/platform#4313 feat(kotlin-sdk): one-time Orchard key shielded-invite API (inviter + claim)](https://github.com/dashpay/platform/pull/4313) — 2 unresolved (2 bot) · 44 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · 🐢 targets v5.1-dev · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk, fallback · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Preserve claim-binding mismatch semantics across the FFI boundary**" — 44 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/platform#4312 feat(platform-wallet): multi-output shielded transfers + output-aware fee predictor](https://github.com/dashpay/platform/pull/4312) — 1 unresolved (1 bot) · 44 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · 🐢 targets v5.1-dev · areas: dpp, rs-platform-wallet, rs-platform-wallet-ffi, kotlin-sdk, fallback · Policy: waiting-self-review
  - Top thread: "🟡 Suggestion: Keep envelope-aware action counting behind the crate boundary**" — 44 days old
  - Blocker: Author must post /self-reviewed da932f96a8670c641065954de8b58752e99233a3
  - Blocker: Proceeded without coderabbitai: no review within the configured window

<a id="dcg-claude"></a>
### @DCG-Claude
<a id="dcg-claude-open"></a>
#### Open (19)
- [dashpay/platform#4703 fix(platform): resolve fee versions by registered number and price refunds at the storage epoch rate](https://github.com/dashpay/platform/pull/4703) — 🔴 CI failing · 🐢 targets v5.1-dev · areas: rs-drive, rs-drive-abci, dpp, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
- [dashpay/platform#4704 fix(platform): record the genesis fee generation and add replay coverage across a fee-version boundary](https://github.com/dashpay/platform/pull/4704) — 🔴 CI failing · 🐢 targets v5.1-dev · areas: rs-drive-abci, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/platform#4705 feat(platform): define protocol-versioned smart-contract computation limits and their gas representation](https://github.com/dashpay/platform/pull/4705) — 🐢 targets v6.0-dev · areas: rs-drive, rs-drive-abci, dpp, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/platform#4706 feat(platform)!: require fee history for storage refunds and credit their recorded owners](https://github.com/dashpay/platform/pull/4706) — 🐢 targets v5.1-dev · areas: rs-drive, rs-drive-abci, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/platform#4707 feat: alloc-only profiles for platform-value and platform-serialization](https://github.com/dashpay/platform/pull/4707) — 🐢 targets v5.1-dev · areas: github, fallback · Policy: waiting-bots
  - Blocker: coderabbitai has not reported for the current head
- [dashpay/platform#4712 feat(platform): add the dashvm-validation crate and the DashVM protocol table](https://github.com/dashpay/platform/pull/4712) — ⚠ merge conflict · 🔴 CI failing · 🐢 targets v6.0-dev · areas: github, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/platform#4716 feat(drive)!: add the contract credits root sum tree to genesis, upgrade and credit conservation](https://github.com/dashpay/platform/pull/4716) — 🐢 targets v6.0-dev · areas: rs-drive, rs-drive-abci, dpp, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
- [dashpay/platform#4717 feat(platform)!: add family-specific state transition size limits and the large contract envelope decode path](https://github.com/dashpay/platform/pull/4717) — 🐢 targets v6.0-dev · areas: rs-drive, rs-drive-abci, dpp, dashmate, rust-dapi, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/platform#4718 ci: compare cross-architecture replay artifacts across a protocol upgrade and a restart](https://github.com/dashpay/platform/pull/4718) — 🔴 CI failing · 🐢 targets v6.0-dev · areas: rs-drive-abci, github, fallback · Policy: waiting-build
  - Blocker: The build must pass before a human is asked
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/platform#4719 feat: add the dash-sdk-contract declaration model, grammar and diagnostics](https://github.com/dashpay/platform/pull/4719) — 🔴 CI failing · 🐢 targets v5.1-dev · areas: github, fallback · Policy: waiting-build
  - Blocker: The build must pass before a human is asked
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/platform#4784 feat(platform)!: validate contested index parameters and bind the native award to its poll](https://github.com/dashpay/platform/pull/4784) — ⚠ merge conflict · 🔴 CI failing · 🐢 targets v6.0-dev · areas: rs-drive, rs-drive-abci, dpp, fallback · Policy: waiting-build
  - Blocker: The build must pass before a human is asked
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/platform#4789 feat(platform): add typed refund owners to storage flags and fee refunds](https://github.com/dashpay/platform/pull/4789) — 🔴 CI failing · 🐢 targets v6.0-dev · areas: rs-drive, dpp, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/platform#4790 feat(drive)!: keep per-issuer token supply rollups and a destroyed-issuer ledger](https://github.com/dashpay/platform/pull/4790) — ⚠ merge conflict · 🔴 CI failing · 🐢 targets v6.0-dev · areas: rs-drive, rs-drive-abci, dpp, fallback · Policy: waiting-build
  - Blocker: The build must pass before a human is asked
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/platform#4794 feat: pin contract configuration vectors and mirror them in wasm-dpp2 and the sdk ffi](https://github.com/dashpay/platform/pull/4794) — 🔴 CI failing · 🐢 targets v5.1-dev · areas: dpp, rust-sdk-ffi, fallback · Policy: waiting-build
  - Blocker: The build must pass before a human is asked
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/platform#4808 test(drive-abci): rehearse a reproducible scheduled host fault and the existing recovery path](https://github.com/dashpay/platform/pull/4808) — 🔴 CI failing · 🐢 targets v6.0-dev · areas: rs-drive-abci, github, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
- [dashpay/platform#4934 docs: correct the smart-contract target, version counts, transition list and mobile sdk availability](https://github.com/dashpay/platform/pull/4934) — 🐢 targets v6.0-dev · areas: swift-sdk, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/platform#4938 docs(platform): distinguish current-state proofs, stored receipts and execution result text](https://github.com/dashpay/platform/pull/4938) — 🐢 targets v6.0-dev · areas: dpp, rust-sdk, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/platform#4939 ci: audit the dashvm engine dependency set and document the equivalence hotfix rule](https://github.com/dashpay/platform/pull/4939) — 🐢 targets v6.0-dev · areas: github, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/platform#5211 feat(platform)!: add compilation readiness rounds, reports and funds to drive](https://github.com/dashpay/platform/pull/5211) — 🐢 targets v6.0-dev · areas: rs-drive, rs-drive-abci, dpp, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
  - Blocker: Proceeded without coderabbitai: no review within the configured window

<a id="dcg-claude-ready-for-human"></a>
#### Ready for human (13)
- [dashpay/platform#4703 fix(platform): resolve fee versions by registered number and price refunds at the storage epoch rate](https://github.com/dashpay/platform/pull/4703) — 🔴 CI failing · 🐢 targets v5.1-dev · areas: rs-drive, rs-drive-abci, dpp, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
- [dashpay/platform#4704 fix(platform): record the genesis fee generation and add replay coverage across a fee-version boundary](https://github.com/dashpay/platform/pull/4704) — 🔴 CI failing · 🐢 targets v5.1-dev · areas: rs-drive-abci, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/platform#4705 feat(platform): define protocol-versioned smart-contract computation limits and their gas representation](https://github.com/dashpay/platform/pull/4705) — 🐢 targets v6.0-dev · areas: rs-drive, rs-drive-abci, dpp, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/platform#4706 feat(platform)!: require fee history for storage refunds and credit their recorded owners](https://github.com/dashpay/platform/pull/4706) — 🐢 targets v5.1-dev · areas: rs-drive, rs-drive-abci, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/platform#4712 feat(platform): add the dashvm-validation crate and the DashVM protocol table](https://github.com/dashpay/platform/pull/4712) — ⚠ merge conflict · 🔴 CI failing · 🐢 targets v6.0-dev · areas: github, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/platform#4716 feat(drive)!: add the contract credits root sum tree to genesis, upgrade and credit conservation](https://github.com/dashpay/platform/pull/4716) — 🐢 targets v6.0-dev · areas: rs-drive, rs-drive-abci, dpp, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
- [dashpay/platform#4717 feat(platform)!: add family-specific state transition size limits and the large contract envelope decode path](https://github.com/dashpay/platform/pull/4717) — 🐢 targets v6.0-dev · areas: rs-drive, rs-drive-abci, dpp, dashmate, rust-dapi, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/platform#4789 feat(platform): add typed refund owners to storage flags and fee refunds](https://github.com/dashpay/platform/pull/4789) — 🔴 CI failing · 🐢 targets v6.0-dev · areas: rs-drive, dpp, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/platform#4808 test(drive-abci): rehearse a reproducible scheduled host fault and the existing recovery path](https://github.com/dashpay/platform/pull/4808) — 🔴 CI failing · 🐢 targets v6.0-dev · areas: rs-drive-abci, github, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
- [dashpay/platform#4934 docs: correct the smart-contract target, version counts, transition list and mobile sdk availability](https://github.com/dashpay/platform/pull/4934) — 🐢 targets v6.0-dev · areas: swift-sdk, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/platform#4938 docs(platform): distinguish current-state proofs, stored receipts and execution result text](https://github.com/dashpay/platform/pull/4938) — 🐢 targets v6.0-dev · areas: dpp, rust-sdk, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/platform#4939 ci: audit the dashvm engine dependency set and document the equivalence hotfix rule](https://github.com/dashpay/platform/pull/4939) — 🐢 targets v6.0-dev · areas: github, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/platform#5211 feat(platform)!: add compilation readiness rounds, reports and funds to drive](https://github.com/dashpay/platform/pull/5211) — 🐢 targets v6.0-dev · areas: rs-drive, rs-drive-abci, dpp, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
  - Blocker: Proceeded without coderabbitai: no review within the configured window

<a id="dcg-claude-stale"></a>
#### Stale (19)
- [dashpay/platform#4703 fix(platform): resolve fee versions by registered number and price refunds at the storage epoch rate](https://github.com/dashpay/platform/pull/4703) — 🔴 CI failing · 🐢 targets v5.1-dev · areas: rs-drive, rs-drive-abci, dpp, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
- [dashpay/platform#4704 fix(platform): record the genesis fee generation and add replay coverage across a fee-version boundary](https://github.com/dashpay/platform/pull/4704) — 🔴 CI failing · 🐢 targets v5.1-dev · areas: rs-drive-abci, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/platform#4705 feat(platform): define protocol-versioned smart-contract computation limits and their gas representation](https://github.com/dashpay/platform/pull/4705) — 🐢 targets v6.0-dev · areas: rs-drive, rs-drive-abci, dpp, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/platform#4706 feat(platform)!: require fee history for storage refunds and credit their recorded owners](https://github.com/dashpay/platform/pull/4706) — 🐢 targets v5.1-dev · areas: rs-drive, rs-drive-abci, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/platform#4707 feat: alloc-only profiles for platform-value and platform-serialization](https://github.com/dashpay/platform/pull/4707) — 🐢 targets v5.1-dev · areas: github, fallback · Policy: waiting-bots
  - Blocker: coderabbitai has not reported for the current head
- [dashpay/platform#4712 feat(platform): add the dashvm-validation crate and the DashVM protocol table](https://github.com/dashpay/platform/pull/4712) — ⚠ merge conflict · 🔴 CI failing · 🐢 targets v6.0-dev · areas: github, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/platform#4716 feat(drive)!: add the contract credits root sum tree to genesis, upgrade and credit conservation](https://github.com/dashpay/platform/pull/4716) — 🐢 targets v6.0-dev · areas: rs-drive, rs-drive-abci, dpp, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
- [dashpay/platform#4717 feat(platform)!: add family-specific state transition size limits and the large contract envelope decode path](https://github.com/dashpay/platform/pull/4717) — 🐢 targets v6.0-dev · areas: rs-drive, rs-drive-abci, dpp, dashmate, rust-dapi, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/platform#4718 ci: compare cross-architecture replay artifacts across a protocol upgrade and a restart](https://github.com/dashpay/platform/pull/4718) — 🔴 CI failing · 🐢 targets v6.0-dev · areas: rs-drive-abci, github, fallback · Policy: waiting-build
  - Blocker: The build must pass before a human is asked
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/platform#4719 feat: add the dash-sdk-contract declaration model, grammar and diagnostics](https://github.com/dashpay/platform/pull/4719) — 🔴 CI failing · 🐢 targets v5.1-dev · areas: github, fallback · Policy: waiting-build
  - Blocker: The build must pass before a human is asked
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/platform#4784 feat(platform)!: validate contested index parameters and bind the native award to its poll](https://github.com/dashpay/platform/pull/4784) — ⚠ merge conflict · 🔴 CI failing · 🐢 targets v6.0-dev · areas: rs-drive, rs-drive-abci, dpp, fallback · Policy: waiting-build
  - Blocker: The build must pass before a human is asked
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/platform#4789 feat(platform): add typed refund owners to storage flags and fee refunds](https://github.com/dashpay/platform/pull/4789) — 🔴 CI failing · 🐢 targets v6.0-dev · areas: rs-drive, dpp, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/platform#4790 feat(drive)!: keep per-issuer token supply rollups and a destroyed-issuer ledger](https://github.com/dashpay/platform/pull/4790) — ⚠ merge conflict · 🔴 CI failing · 🐢 targets v6.0-dev · areas: rs-drive, rs-drive-abci, dpp, fallback · Policy: waiting-build
  - Blocker: The build must pass before a human is asked
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/platform#4794 feat: pin contract configuration vectors and mirror them in wasm-dpp2 and the sdk ffi](https://github.com/dashpay/platform/pull/4794) — 🔴 CI failing · 🐢 targets v5.1-dev · areas: dpp, rust-sdk-ffi, fallback · Policy: waiting-build
  - Blocker: The build must pass before a human is asked
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/platform#4808 test(drive-abci): rehearse a reproducible scheduled host fault and the existing recovery path](https://github.com/dashpay/platform/pull/4808) — 🔴 CI failing · 🐢 targets v6.0-dev · areas: rs-drive-abci, github, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
- [dashpay/platform#4934 docs: correct the smart-contract target, version counts, transition list and mobile sdk availability](https://github.com/dashpay/platform/pull/4934) — 🐢 targets v6.0-dev · areas: swift-sdk, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/platform#4938 docs(platform): distinguish current-state proofs, stored receipts and execution result text](https://github.com/dashpay/platform/pull/4938) — 🐢 targets v6.0-dev · areas: dpp, rust-sdk, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/platform#4939 ci: audit the dashvm engine dependency set and document the equivalence hotfix rule](https://github.com/dashpay/platform/pull/4939) — 🐢 targets v6.0-dev · areas: github, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/platform#5211 feat(platform)!: add compilation readiness rounds, reports and funds to drive](https://github.com/dashpay/platform/pull/5211) — 🐢 targets v6.0-dev · areas: rs-drive, rs-drive-abci, dpp, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
  - Blocker: Proceeded without coderabbitai: no review within the configured window

<a id="kwvg"></a>
### @kwvg
<a id="kwvg-open"></a>
#### Open (1)
- [dashpay/rust-dashcore#1151 refactor!: pin existing \`serde\` layout, abstract EDDSA backend types behind newtypes, switch to \`dash-pkc\`, re-export some \`bitcoin-crypto\` ECDSA types](https://github.com/dashpay/rust-dashcore/pull/1151) — 📝 draft · areas: key-wallet, key-wallet-manager, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot

<a id="kwvg-draft"></a>
#### Draft (1)
- [dashpay/rust-dashcore#1151 refactor!: pin existing \`serde\` layout, abstract EDDSA backend types behind newtypes, switch to \`dash-pkc\`, re-export some \`bitcoin-crypto\` ECDSA types](https://github.com/dashpay/rust-dashcore/pull/1151) — 📝 draft · areas: key-wallet, key-wallet-manager, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot

## Methodology
Generated nightly by [pr-hygiene](https://github.com/dashpay/stale_prs_are_bad). A thread counts as "unresolved" when it is open, not outdated, has a comment from someone other than the PR author, and the most recent comment is from a reviewer. **Dirty** = at least one such thread. **Unresolved Comments** = at least one such thread. **Changes Requested** = no unresolved threads but a reviewer's most recent review is CHANGES_REQUESTED (still blocking until someone re-approves or dismisses). **Deferred** = carries a configured deferred label (e.g. `postponed`) — visible but not counted toward unresolved-comment counts. **Stale** = targets a non-default branch OR hasn't been touched in the configured threshold (default 120 days, but clean PRs are never reclassified as stale). **Draft** = the PR is still marked draft on GitHub. **CI failing** = no unresolved comments, no changes-requested, but the latest commit's status check is failing. **Clean** = open, not draft, not deferred, not stale, no unresolved comments, no changes-requested, CI green. **Needs action** further requires changes-requested, merge conflict, or that the reviewer commented more recently than the author last pushed. **Ready for human** counts a person's own PRs that the shared review engine marks `ready-for-human` or `ready-to-merge`; each PR bullet shows the engine's state as `Policy:` with its blockers. **Ready for Review** counts clean PRs (authored by someone else) where this person owes a review: the union of the shared policy's routing (owners and reviewers of every area the changed files fall into, or the repository fallback for files no area claims) and GitHub's explicit review requests. The author is never routed to their own PR, and anyone who has already submitted any review is excluded — their job is done. `⚠ ownership unresolved` marks areas whose roster the policy still lists as open. Configurable via [`https://github.com/dashpay/stale_prs_are_bad/blob/master/.pr-hygiene.yml`](https://github.com/dashpay/stale_prs_are_bad/blob/master/.pr-hygiene.yml)—edit defaults there; ownership lives in `policies/`.
