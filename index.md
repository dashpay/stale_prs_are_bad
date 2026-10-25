---
---
# PR Hygiene Report
*Last updated: 2026-10-25 12:15 UTC · commit 2fa56d6*

## Summary
- Open PRs: **172** (8 clean · 4 CI failing · 7 changes requested · 12 unresolved comments · 7 deferred · 16 draft · 118 stale)
- PRs needing author action: **21**
- Total unresolved comments: **178**
- dashpay/platform: **112** open (4 clean · 0 CI failing · 0 changes requested · 3 unresolved comments · 7 deferred · 0 draft · 98 stale) · engine: 20 draft · 20 ready-for-human · 3 ready-to-merge · 11 waiting-author · 6 waiting-bots · 7 waiting-build · 23 waiting-self-review · 22 no verdict
- dashpay/rust-dashcore: **32** open (2 clean · 3 CI failing · 6 changes requested · 5 unresolved comments · 0 deferred · 11 draft · 5 stale) · engine: 13 draft · 2 ready-for-human · 3 waiting-author · 11 waiting-self-review · 3 no verdict
- dashpay/tenderdash: **6** open (1 clean · 0 CI failing · 0 changes requested · 3 unresolved comments · 0 deferred · 2 draft · 0 stale) · engine: 2 draft · 3 waiting-author · 1 waiting-self-review
- dashpay/grovedb: **2** open (1 clean · 0 CI failing · 0 changes requested · 0 unresolved comments · 0 deferred · 1 draft · 0 stale) · engine: 1 draft · 1 waiting-self-review
- dashpay/dash-evo-tool: **20** open (0 clean · 1 CI failing · 1 changes requested · 1 unresolved comments · 0 deferred · 2 draft · 15 stale) · engine: 2 draft · 1 waiting-author · 1 waiting-bots · 1 waiting-build · 15 no verdict

## Scoreboard
_Sort: unresolved-comments desc → needs-action desc → ready-for-review desc. Click any number to jump to the specific PRs it covers._

| Author | Open | Clean | CI failing | Unresolved Comments | Changes Requested | Deferred | Draft | Stale | Needs action | Ready for human | Total Unresolved Comments | Ready for Review | Δ |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| [@QuantumExplorer](#quantumexplorer) | [22](#quantumexplorer-open) | [1](#quantumexplorer-clean) | — | [3](#quantumexplorer-unresolved-comments) | — | — | — | [18](#quantumexplorer-stale) | [4](#quantumexplorer-needs-action) | — | [33](#quantumexplorer-unresolved-comments) | [6](#quantumexplorer-ready-for-review) | — |
| [@Claudius-Maginificent](#claudius-maginificent) | [30](#claudius-maginificent-open) | — | [1](#claudius-maginificent-ci-failing) | [2](#claudius-maginificent-unresolved-comments) | [1](#claudius-maginificent-changes-requested) | [3](#claudius-maginificent-deferred) | [1](#claudius-maginificent-draft) | [22](#claudius-maginificent-stale) | [3](#claudius-maginificent-needs-action) | [3](#claudius-maginificent-ready-for-human) | [27](#claudius-maginificent-unresolved-comments) | — | ↑ 3 |
| [@lklimek](#lklimek) | [15](#lklimek-open) | [1](#lklimek-clean) | — | [2](#lklimek-unresolved-comments) | — | [4](#lklimek-deferred) | [3](#lklimek-draft) | [5](#lklimek-stale) | [2](#lklimek-needs-action) | — | [15](#lklimek-unresolved-comments) | [3](#lklimek-ready-for-review) | ↓ 2 |
| [@shumkov](#shumkov) | [14](#shumkov-open) | [4](#shumkov-clean) | [1](#shumkov-ci-failing) | [2](#shumkov-unresolved-comments) | — | — | — | [7](#shumkov-stale) | [2](#shumkov-needs-action) | [1](#shumkov-ready-for-human) | [30](#shumkov-unresolved-comments) | [1](#shumkov-ready-for-review) | ↓ 1 |
| [@llbartekll](#llbartekll) | [5](#llbartekll-open) | — | — | [1](#llbartekll-unresolved-comments) | [1](#llbartekll-changes-requested) | — | [1](#llbartekll-draft) | [2](#llbartekll-stale) | [2](#llbartekll-needs-action) | — | [4](#llbartekll-unresolved-comments) | — | — |
| [@xdustinface](#xdustinface) | [8](#xdustinface-open) | [1](#xdustinface-clean) | — | [1](#xdustinface-unresolved-comments) | — | — | [4](#xdustinface-draft) | [2](#xdustinface-stale) | [1](#xdustinface-needs-action) | — | [1](#xdustinface-unresolved-comments) | [1](#xdustinface-ready-for-review) | — |
| [@HashEngineering](#hashengineering) | [4](#hashengineering-open) | — | — | [1](#hashengineering-unresolved-comments) | — | — | — | [3](#hashengineering-stale) | [1](#hashengineering-needs-action) | — | [11](#hashengineering-unresolved-comments) | — | ↑ 1 |
| [@ZocoLini](#zocolini) | [11](#zocolini-open) | — | [1](#zocolini-ci-failing) | — | [4](#zocolini-changes-requested) | — | [6](#zocolini-draft) | — | [4](#zocolini-needs-action) | — | — | [1](#zocolini-ready-for-review) | ↑ 2 |
| [@PastaPastaPasta](#pastapastapasta) | [20](#pastapastapasta-open) | [1](#pastapastapasta-clean) | [1](#pastapastapasta-ci-failing) | — | — | — | [1](#pastapastapasta-draft) | [17](#pastapastapasta-stale) | [1](#pastapastapasta-needs-action) | [2](#pastapastapasta-ready-for-human) | [28](#pastapastapasta-unresolved-comments) | — | ↓ 1 |
| [@romchornyi](#romchornyi) | [12](#romchornyi-open) | — | — | — | [1](#romchornyi-changes-requested) | — | — | [11](#romchornyi-stale) | [1](#romchornyi-needs-action) | [3](#romchornyi-ready-for-human) | [20](#romchornyi-unresolved-comments) | — | ↓ 4 |
| [@ktechmidas](#ktechmidas) | [6](#ktechmidas-open) | — | — | — | — | — | — | [6](#ktechmidas-stale) | — | — | [1](#ktechmidas-unresolved-comments) | [1](#ktechmidas-ready-for-review) | — |
| [@infraclaw-dash](#infraclaw-dash) | [3](#infraclaw-dash-open) | — | — | — | — | — | — | [3](#infraclaw-dash-stale) | — | [1](#infraclaw-dash-ready-for-human) | [5](#infraclaw-dash-unresolved-comments) | — | — |
| [@bfoss765](#bfoss765) | [2](#bfoss765-open) | — | — | — | — | — | — | [2](#bfoss765-stale) | — | — | [3](#bfoss765-unresolved-comments) | — | — |
| [@DCG-Claude](#dcg-claude) | [19](#dcg-claude-open) | — | — | — | — | — | — | [19](#dcg-claude-stale) | — | [14](#dcg-claude-ready-for-human) | — | — | — |
| [@thepastaclaw](#thepastaclaw) | [1](#thepastaclaw-open) | — | — | — | — | — | — | [1](#thepastaclaw-stale) | — | [1](#thepastaclaw-ready-for-human) | — | — | — |

## Per-author detail

<a id="quantumexplorer"></a>
### @QuantumExplorer
<a id="quantumexplorer-open"></a>
#### Open (22)
- [dashpay/rust-dashcore#1078 feat(key-wallet)!: lock masternode collateral out of coin selection](https://github.com/dashpay/rust-dashcore/pull/1078) — 8 unresolved (8 human) · 25 days stale · ⚠ merge conflict · ✋ changes requested · areas: key-wallet, key-wallet-manager · Policy: waiting-self-review
  - Top thread: "This sets the state by hand, and together with the serde round-trip it is the only coverage of the refresh inside \`updat…" — 25 days old
  - Blocker: Author must post /self-reviewed 44a9a010500fbd44aba09111b4c53a28b88e8073
- [dashpay/platform#4272 feat(dpp)!: dashpay payment detection keys and stealth derivation](https://github.com/dashpay/platform/pull/4272) — 9 unresolved (9 bot) · 83 days stale · ⚠ merge conflict · 📝 draft · 🐢 targets v5.1-dev · areas: rs-drive, rs-drive-abci, dpp, rs-platform-wallet, swift-sdk, system-contracts, kotlin-sdk, fallback · Policy: draft
  - Top thread: "🔴 Blocking: Require compressed 33-byte encodings for payment keys**" — 83 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#860 feat(dash-spv): expose per-peer connection stats (ping RTT, bytes received)](https://github.com/dashpay/rust-dashcore/pull/860) — 3 unresolved (3 CodeRabbit) · 107 days stale · ✋ changes requested · areas: dash-spv, fallback · Policy: waiting-author
  - Top thread: "_📐 Maintainability &amp; Code Quality_ \| _🟡 Minor_ \| _⚡ Quick win_" — 107 days old
  - Blocker: coderabbitai left review threads unresolved; resolve them
- [dashpay/platform#5238 fix(platform)!: refund sponsor-paid document storage to the gas sponsor](https://github.com/dashpay/platform/pull/5238) — 3 unresolved (3 bot) · 24 days stale · ⚠ merge conflict · ✋ changes requested · 📝 draft · 🐢 targets v5.1-dev · areas: rs-drive, rs-drive-abci, fallback · Policy: draft
  - Top thread: "🟡 Suggestion: Make the transfer regression change the primary element's size**" — 24 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#1077 feat(dash-spv)!: check masternode list diffs against the block coinbase](https://github.com/dashpay/rust-dashcore/pull/1077) — 2 unresolved (2 CodeRabbit) · 27 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · areas: dash-spv, fallback · Policy: waiting-author
  - Top thread: "_🩺 Stability &amp; Availability_ \| _🟠 Major_ \| _⚡ Quick win_" — 27 days old
  - Blocker: coderabbitai requested changes on this head; dismiss the review or push a fix
  - Blocker: coderabbitai left review threads unresolved; resolve them
- [dashpay/platform#5360 perf(platform)!: delete a consumed document from what its lookup read (PV15)](https://github.com/dashpay/platform/pull/5360) — 2 unresolved (2 bot) · 16 days stale · ⚠ merge conflict · 🐢 targets v5.1-dev · areas: rs-drive, rs-drive-abci, dpp, rust-sdk, js-wasm-sdk, system-contracts, fallback · Policy: waiting-self-review
  - Top thread: "🟡 Suggestion: Add negative-path tests for the new read-document deletion guards**" — 16 days old
  - Blocker: Author must post /self-reviewed f338eb17d9c54ba6976686769b2de9c3d1d61104
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/platform#4899 feat(platform)!: yes/no masternode vote poll kind with supermajority and minimum voting power](https://github.com/dashpay/platform/pull/4899) — 2 unresolved (2 bot) · 4 days stale · ⚠ merge conflict · ✋ changes requested · 🐢 targets v5.1-dev · areas: rs-drive, rs-drive-abci, dpp, rust-sdk, fallback · Policy: waiting-bots
  - Top thread: "🟡 Suggestion: Do not treat a page consumed entirely by empty end dates as exhaustion**" — 4 days old
  - Blocker: coderabbitai has not reported for the current head
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5533 feat(dpp)!: validate documents natively at protocol version 15](https://github.com/dashpay/platform/pull/5533) — 3 unresolved (3 bot) · 0 days stale · 🔴 CI failing · 🐢 targets v5.1-dev · areas: dpp, fallback · Policy: waiting-bots
  - Top thread: "🟡 Suggestion: Freeze the format vocabulary used by validation-schema generation 0**" — 0 days old
  - Blocker: coderabbitai has not reported for the current head
- [dashpay/platform#3417 feat(swift-sdk): use SPV-synced quorums for Platform proof verification](https://github.com/dashpay/platform/pull/3417) — 1 unresolved (1 bot) · 3 days stale · ✋ changes requested · 🐢 targets v5.1-dev · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, rust-sdk, rust-sdk-ffi, fallback · Policy: waiting-author
  - Top thread: "🔴 Blocking: Require network-difficulty validation before trusting SPV headers**" — 3 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/grovedb#1001 feat(batch)!: opt-in settlement of storage owner changes](https://github.com/dashpay/grovedb/pull/1001) — ⚠ merge conflict · areas: fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 814dfc765747cec0beb7a39e1cdf04d301ce9ac3
- [dashpay/platform#4519 feat(platform-wallet): rotate a masternode's keys into the wallet — ProUpRegTx orchestration, FFI, Swift](https://github.com/dashpay/platform/pull/4519) — ⚠ merge conflict · 🐢 targets v5.1-dev · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 92a3ebe17adc12432fb756c63f172c663809007d
  - Blocker: Proceeded without coderabbitai: no review within the configured window
  - Blocker: Proceeded without thepastaclaw: no review within the configured window
- [dashpay/platform#4730 feat(platform)!: delta-based data contract update transition for protocol version 15](https://github.com/dashpay/platform/pull/4730) — ⚠ merge conflict · 🔴 CI failing · 📝 draft · 🐢 targets v5.1-dev · areas: rs-drive, rs-drive-abci, dpp, rs-platform-wallet, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#4776 feat(platform-wallet)!: let contract updates clear the description through the FFI](https://github.com/dashpay/platform/pull/4776) — 🔴 CI failing · 🐢 targets feat/delta-contract-update-pv15 · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk, fallback
- [dashpay/platform#4933 feat(platform)!: dashpay contact requests declare their checks](https://github.com/dashpay/platform/pull/4933) — ⚠ merge conflict · 📝 draft · 🐢 targets v5.1-dev · areas: rs-drive, rs-drive-abci, dpp, system-contracts, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#4986 fix(drive-abci)!: pay every reward share of a masternode and credit each identity once in the epoch payout (PV15)](https://github.com/dashpay/platform/pull/4986) — ⚠ merge conflict · 📝 draft · 🐢 targets v5.1-dev · areas: rs-drive, rs-drive-abci, dpp, github, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5113 feat(platform-wallet)!: keep masternode collateral out of coin selection](https://github.com/dashpay/platform/pull/5113) — ⚠ merge conflict · 🔴 CI failing · 📝 draft · 🐢 targets chore/bump-rust-dashcore-secp-033 · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk, fallback
- [dashpay/platform#5526 feat(platform)!: token marketplace escrow trees and token market contract (PV15)](https://github.com/dashpay/platform/pull/5526) — 🐢 targets v5.1-dev · areas: rs-drive, rs-drive-abci, dpp, rust-sdk, rust-sdk-ffi, js-wasm-sdk, system-contracts, github, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed ad0d84f80a741b979735e1d7ce09cf2efc2d75fb
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/platform#5529 feat(platform)!: token marketplace listings (PV15)](https://github.com/dashpay/platform/pull/5529) — 🐢 targets claude/token-market-escrow-contract · areas: rs-drive, rs-drive-abci, dpp, rs-platform-wallet, fallback
- [dashpay/platform#5530 feat(platform)!: token marketplace offers (PV15)](https://github.com/dashpay/platform/pull/5530) — ✋ changes requested · 🐢 targets claude/token-market-listings · areas: rs-drive, rs-drive-abci, dpp, rs-platform-wallet, fallback
- [dashpay/platform#5531 feat(platform)!: record token marketplace events in token history (PV15)](https://github.com/dashpay/platform/pull/5531) — 🐢 targets claude/token-market-offers · areas: rs-drive, rs-drive-abci, dpp, system-contracts, fallback
- [dashpay/platform#5532 feat(platform)!: destroy a frozen identity's escrowed listings with its funds (PV15)](https://github.com/dashpay/platform/pull/5532) — 🐢 targets claude/token-market-history · areas: rs-drive, rs-drive-abci, fallback
- [dashpay/platform#5538 feat(dpp)!: check document schemas natively at registration from protocol version 15](https://github.com/dashpay/platform/pull/5538) — ✋ changes requested · 🐢 targets claude/native-document-validator · areas: dpp, fallback

<a id="quantumexplorer-needs-action"></a>
#### Needs action (4)
- [dashpay/rust-dashcore#1078 feat(key-wallet)!: lock masternode collateral out of coin selection](https://github.com/dashpay/rust-dashcore/pull/1078) — 8 unresolved (8 human) · 25 days stale · ⚠ merge conflict · ✋ changes requested · areas: key-wallet, key-wallet-manager · Policy: waiting-self-review
  - Top thread: "This sets the state by hand, and together with the serde round-trip it is the only coverage of the refresh inside \`updat…" — 25 days old
  - Blocker: Author must post /self-reviewed 44a9a010500fbd44aba09111b4c53a28b88e8073
- [dashpay/rust-dashcore#860 feat(dash-spv): expose per-peer connection stats (ping RTT, bytes received)](https://github.com/dashpay/rust-dashcore/pull/860) — 3 unresolved (3 CodeRabbit) · 107 days stale · ✋ changes requested · areas: dash-spv, fallback · Policy: waiting-author
  - Top thread: "_📐 Maintainability &amp; Code Quality_ \| _🟡 Minor_ \| _⚡ Quick win_" — 107 days old
  - Blocker: coderabbitai left review threads unresolved; resolve them
- [dashpay/rust-dashcore#1077 feat(dash-spv)!: check masternode list diffs against the block coinbase](https://github.com/dashpay/rust-dashcore/pull/1077) — 2 unresolved (2 CodeRabbit) · 27 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · areas: dash-spv, fallback · Policy: waiting-author
  - Top thread: "_🩺 Stability &amp; Availability_ \| _🟠 Major_ \| _⚡ Quick win_" — 27 days old
  - Blocker: coderabbitai requested changes on this head; dismiss the review or push a fix
  - Blocker: coderabbitai left review threads unresolved; resolve them
- [dashpay/grovedb#1001 feat(batch)!: opt-in settlement of storage owner changes](https://github.com/dashpay/grovedb/pull/1001) — ⚠ merge conflict · areas: fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 814dfc765747cec0beb7a39e1cdf04d301ce9ac3

<a id="quantumexplorer-unresolved-comments"></a>
#### Unresolved Comments (3)
- [dashpay/rust-dashcore#1078 feat(key-wallet)!: lock masternode collateral out of coin selection](https://github.com/dashpay/rust-dashcore/pull/1078) — 8 unresolved (8 human) · 25 days stale · ⚠ merge conflict · ✋ changes requested · areas: key-wallet, key-wallet-manager · Policy: waiting-self-review
  - Top thread: "This sets the state by hand, and together with the serde round-trip it is the only coverage of the refresh inside \`updat…" — 25 days old
  - Blocker: Author must post /self-reviewed 44a9a010500fbd44aba09111b4c53a28b88e8073
- [dashpay/rust-dashcore#860 feat(dash-spv): expose per-peer connection stats (ping RTT, bytes received)](https://github.com/dashpay/rust-dashcore/pull/860) — 3 unresolved (3 CodeRabbit) · 107 days stale · ✋ changes requested · areas: dash-spv, fallback · Policy: waiting-author
  - Top thread: "_📐 Maintainability &amp; Code Quality_ \| _🟡 Minor_ \| _⚡ Quick win_" — 107 days old
  - Blocker: coderabbitai left review threads unresolved; resolve them
- [dashpay/rust-dashcore#1077 feat(dash-spv)!: check masternode list diffs against the block coinbase](https://github.com/dashpay/rust-dashcore/pull/1077) — 2 unresolved (2 CodeRabbit) · 27 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · areas: dash-spv, fallback · Policy: waiting-author
  - Top thread: "_🩺 Stability &amp; Availability_ \| _🟠 Major_ \| _⚡ Quick win_" — 27 days old
  - Blocker: coderabbitai requested changes on this head; dismiss the review or push a fix
  - Blocker: coderabbitai left review threads unresolved; resolve them

<a id="quantumexplorer-stale"></a>
#### Stale (18)
- [dashpay/platform#4272 feat(dpp)!: dashpay payment detection keys and stealth derivation](https://github.com/dashpay/platform/pull/4272) — 9 unresolved (9 bot) · 83 days stale · ⚠ merge conflict · 📝 draft · 🐢 targets v5.1-dev · areas: rs-drive, rs-drive-abci, dpp, rs-platform-wallet, swift-sdk, system-contracts, kotlin-sdk, fallback · Policy: draft
  - Top thread: "🔴 Blocking: Require compressed 33-byte encodings for payment keys**" — 83 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5238 fix(platform)!: refund sponsor-paid document storage to the gas sponsor](https://github.com/dashpay/platform/pull/5238) — 3 unresolved (3 bot) · 24 days stale · ⚠ merge conflict · ✋ changes requested · 📝 draft · 🐢 targets v5.1-dev · areas: rs-drive, rs-drive-abci, fallback · Policy: draft
  - Top thread: "🟡 Suggestion: Make the transfer regression change the primary element's size**" — 24 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5360 perf(platform)!: delete a consumed document from what its lookup read (PV15)](https://github.com/dashpay/platform/pull/5360) — 2 unresolved (2 bot) · 16 days stale · ⚠ merge conflict · 🐢 targets v5.1-dev · areas: rs-drive, rs-drive-abci, dpp, rust-sdk, js-wasm-sdk, system-contracts, fallback · Policy: waiting-self-review
  - Top thread: "🟡 Suggestion: Add negative-path tests for the new read-document deletion guards**" — 16 days old
  - Blocker: Author must post /self-reviewed f338eb17d9c54ba6976686769b2de9c3d1d61104
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/platform#4899 feat(platform)!: yes/no masternode vote poll kind with supermajority and minimum voting power](https://github.com/dashpay/platform/pull/4899) — 2 unresolved (2 bot) · 4 days stale · ⚠ merge conflict · ✋ changes requested · 🐢 targets v5.1-dev · areas: rs-drive, rs-drive-abci, dpp, rust-sdk, fallback · Policy: waiting-bots
  - Top thread: "🟡 Suggestion: Do not treat a page consumed entirely by empty end dates as exhaustion**" — 4 days old
  - Blocker: coderabbitai has not reported for the current head
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5533 feat(dpp)!: validate documents natively at protocol version 15](https://github.com/dashpay/platform/pull/5533) — 3 unresolved (3 bot) · 0 days stale · 🔴 CI failing · 🐢 targets v5.1-dev · areas: dpp, fallback · Policy: waiting-bots
  - Top thread: "🟡 Suggestion: Freeze the format vocabulary used by validation-schema generation 0**" — 0 days old
  - Blocker: coderabbitai has not reported for the current head
- [dashpay/platform#3417 feat(swift-sdk): use SPV-synced quorums for Platform proof verification](https://github.com/dashpay/platform/pull/3417) — 1 unresolved (1 bot) · 3 days stale · ✋ changes requested · 🐢 targets v5.1-dev · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, rust-sdk, rust-sdk-ffi, fallback · Policy: waiting-author
  - Top thread: "🔴 Blocking: Require network-difficulty validation before trusting SPV headers**" — 3 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/platform#4519 feat(platform-wallet): rotate a masternode's keys into the wallet — ProUpRegTx orchestration, FFI, Swift](https://github.com/dashpay/platform/pull/4519) — ⚠ merge conflict · 🐢 targets v5.1-dev · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 92a3ebe17adc12432fb756c63f172c663809007d
  - Blocker: Proceeded without coderabbitai: no review within the configured window
  - Blocker: Proceeded without thepastaclaw: no review within the configured window
- [dashpay/platform#4730 feat(platform)!: delta-based data contract update transition for protocol version 15](https://github.com/dashpay/platform/pull/4730) — ⚠ merge conflict · 🔴 CI failing · 📝 draft · 🐢 targets v5.1-dev · areas: rs-drive, rs-drive-abci, dpp, rs-platform-wallet, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#4776 feat(platform-wallet)!: let contract updates clear the description through the FFI](https://github.com/dashpay/platform/pull/4776) — 🔴 CI failing · 🐢 targets feat/delta-contract-update-pv15 · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk, fallback
- [dashpay/platform#4933 feat(platform)!: dashpay contact requests declare their checks](https://github.com/dashpay/platform/pull/4933) — ⚠ merge conflict · 📝 draft · 🐢 targets v5.1-dev · areas: rs-drive, rs-drive-abci, dpp, system-contracts, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#4986 fix(drive-abci)!: pay every reward share of a masternode and credit each identity once in the epoch payout (PV15)](https://github.com/dashpay/platform/pull/4986) — ⚠ merge conflict · 📝 draft · 🐢 targets v5.1-dev · areas: rs-drive, rs-drive-abci, dpp, github, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5113 feat(platform-wallet)!: keep masternode collateral out of coin selection](https://github.com/dashpay/platform/pull/5113) — ⚠ merge conflict · 🔴 CI failing · 📝 draft · 🐢 targets chore/bump-rust-dashcore-secp-033 · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk, fallback
- [dashpay/platform#5526 feat(platform)!: token marketplace escrow trees and token market contract (PV15)](https://github.com/dashpay/platform/pull/5526) — 🐢 targets v5.1-dev · areas: rs-drive, rs-drive-abci, dpp, rust-sdk, rust-sdk-ffi, js-wasm-sdk, system-contracts, github, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed ad0d84f80a741b979735e1d7ce09cf2efc2d75fb
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/platform#5529 feat(platform)!: token marketplace listings (PV15)](https://github.com/dashpay/platform/pull/5529) — 🐢 targets claude/token-market-escrow-contract · areas: rs-drive, rs-drive-abci, dpp, rs-platform-wallet, fallback
- [dashpay/platform#5530 feat(platform)!: token marketplace offers (PV15)](https://github.com/dashpay/platform/pull/5530) — ✋ changes requested · 🐢 targets claude/token-market-listings · areas: rs-drive, rs-drive-abci, dpp, rs-platform-wallet, fallback
- [dashpay/platform#5531 feat(platform)!: record token marketplace events in token history (PV15)](https://github.com/dashpay/platform/pull/5531) — 🐢 targets claude/token-market-offers · areas: rs-drive, rs-drive-abci, dpp, system-contracts, fallback
- [dashpay/platform#5532 feat(platform)!: destroy a frozen identity's escrowed listings with its funds (PV15)](https://github.com/dashpay/platform/pull/5532) — 🐢 targets claude/token-market-history · areas: rs-drive, rs-drive-abci, fallback
- [dashpay/platform#5538 feat(dpp)!: check document schemas natively at registration from protocol version 15](https://github.com/dashpay/platform/pull/5538) — ✋ changes requested · 🐢 targets claude/native-document-validator · areas: dpp, fallback

<a id="quantumexplorer-clean"></a>
#### Clean (1)
- [dashpay/grovedb#1001 feat(batch)!: opt-in settlement of storage owner changes](https://github.com/dashpay/grovedb/pull/1001) — ⚠ merge conflict · areas: fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 814dfc765747cec0beb7a39e1cdf04d301ce9ac3

<a id="quantumexplorer-ready-for-review"></a>
#### Ready for Review (6)
- [dashpay/platform#5557 fix(drive-abci): answer queries for features not active yet with INVALID_ARGUMENT](https://github.com/dashpay/platform/pull/5557) — by @shumkov · areas: rs-drive-abci, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 98591f1166b60c256a3afbabec8186a87b9702f7
- [dashpay/platform#5558 fix(sdk): check document query shapes before sending them](https://github.com/dashpay/platform/pull/5558) — by @shumkov · areas: rs-drive-abci, rust-sdk, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 5f76f73f741c398ae76dd078ef5c5a79031edacf
- [dashpay/platform#5559 test(sdk): run operation-proof tests in CI and select the verifier generation from the verifier](https://github.com/dashpay/platform/pull/5559) — by @shumkov · areas: rs-drive, rs-drive-abci, rust-sdk, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed fe8a7dde6e85b1455da3ac3ba1412675516a9625
- [dashpay/platform#5561 fix(sdk)!: refuse requests the network cannot serve yet instead of banning nodes](https://github.com/dashpay/platform/pull/5561) — by @shumkov · areas: rs-drive-abci, rust-sdk, rust-sdk-ffi, js-wasm-sdk, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed b36f0017d00008f28067484b3f6c1930d52c7867
- [dashpay/rust-dashcore#264 feat(dash-spv): Add BIP324 v2 encrypted P2P transport](https://github.com/dashpay/rust-dashcore/pull/264) — by @PastaPastaPasta · areas: dash-spv, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
- [dashpay/rust-dashcore#849 feat(dash-spv): add \`--birth-height\` CLI flag](https://github.com/dashpay/rust-dashcore/pull/849) — by @xdustinface · areas: dash-spv · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 9b1e709fd8e0bfab0dd69f7c86fd6b7d2e585fbe

<a id="claudius-maginificent"></a>
### @Claudius-Maginificent
<a id="claudius-maginificent-open"></a>
#### Open (30)
- [dashpay/tenderdash#1562 docs(consensus): add source-derived consensus protocol specification](https://github.com/dashpay/tenderdash/pull/1562) — 5 unresolved (5 bot) · 3 days stale · ✋ changes requested · areas: fallback · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Spec cites ErrUnsupportedVoteExtensionType that does not exist until #1565 merges**" — 3 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5565 feat(platform-wallet)!: detect inconsistent shielded commitment trees and add explicit rebuild](https://github.com/dashpay/platform/pull/5565) — 4 unresolved (4 bot) · 1 days stale · ✋ changes requested · 🐢 targets fix/shielded-sync-concurrent-append · areas: rs-platform-wallet
  - Top thread: "🟡 Suggestion: Reset host-visible scan metadata during a tree rebuild**" — 1 days old
- [dashpay/dash-evo-tool#1108 feat(documents): recognise indexOnly document types and delete their entries by values](https://github.com/dashpay/dash-evo-tool/pull/1108) — 3 unresolved (3 bot) · 2 days stale · ⚠ merge conflict · ✋ changes requested · 📝 draft · 🐢 targets chore/bump-platform-v4.3 · areas: fallback
  - Top thread: "🟡 Suggestion: Make the non-deletable error actionable**" — 2 days old
- [dashpay/dash-evo-tool#1109 feat(masternode): use wallet-held node keys and withdraw via platform-wallet](https://github.com/dashpay/dash-evo-tool/pull/1109) — 3 unresolved (3 bot) · 1 days stale · ✋ changes requested · 📝 draft · 🐢 targets chore/bump-platform-v4.3 · areas: fallback
  - Top thread: "🔴 Blocking: Do not let a disabled stored key suppress discovery of the current node key**" — 1 days old
- [dashpay/dash-evo-tool#1112 feat(send): one From/To Send form with Auto routing](https://github.com/dashpay/dash-evo-tool/pull/1112) — 3 unresolved (3 bot) · 1 days stale · ✋ changes requested · 📝 draft · 🐢 targets feat/wallet-max-send-limits · areas: fallback
  - Top thread: "🟡 Suggestion: Do not rotate an own-wallet destination on every refresh**" — 1 days old
- [dashpay/platform#5562 fix(sdk): retry address nonce skew on another node](https://github.com/dashpay/platform/pull/5562) — 3 unresolved (3 bot) · 2 days stale · 🐢 targets v5.1-dev · areas: rust-sdk · Policy: ready-to-merge
  - Top thread: "🟡 Suggestion: Add coverage for the real CheckTx consensus-error conversion path**" — 2 days old
  - Blocker: All policy requirements are satisfied
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/dash-evo-tool#1111 docs(wallet): redesign the Send screen as one From/To form](https://github.com/dashpay/dash-evo-tool/pull/1111) — 2 unresolved (2 bot) · 2 days stale · 📝 draft · areas: fallback · Policy: draft
  - Top thread: "🟡 Suggestion: Define how destination Auto handles an address that becomes used**" — 2 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5546 fix(platform-wallet): support HASH160 DashPay contact signing keys](https://github.com/dashpay/platform/pull/5546) — 1 unresolved (1 bot) · 2 days stale · ⚠ merge conflict · 📝 draft · 🐢 targets v5.1-dev · areas: rs-platform-wallet · Policy: draft
  - Top thread: "🟡 Suggestion: contact_request_signing_key can select bounded or expired keys that cannot complete the write**" — 2 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/dash-evo-tool#1107 fix(wallet): make every Max and fee match what the upstream builders accept](https://github.com/dashpay/dash-evo-tool/pull/1107) — 1 unresolved (1 bot) · 1 days stale · ✋ changes requested · 🐢 targets chore/bump-platform-v4.3 · areas: fallback
  - Top thread: "🟡 Suggestion: Preserve typed insufficient-funds errors when pooled finalization reports no UTXOs**" — 1 days old
- [dashpay/dash-evo-tool#1114 chore(deps)!: update Platform to v5.1-dev 063589bf and follow the network's protocol version](https://github.com/dashpay/dash-evo-tool/pull/1114) — 1 unresolved (1 bot) · 0 days stale · ✋ changes requested · areas: fallback · Policy: waiting-bots
  - Top thread: "🔴 Blocking: Connection rebuild leaves WalletBackend bound to the old SDK**" — 0 days old
  - Blocker: coderabbitai has not reported for the current head
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5564 fix(platform-wallet): stop concurrent shielded syncs from duplicating commitments](https://github.com/dashpay/platform/pull/5564) — 1 unresolved (1 bot) · 2 days stale · 🔴 CI failing · 🐢 targets v5.1-dev · areas: rs-platform-wallet · Policy: waiting-build
  - Top thread: "🟡 Suggestion: Name the mock stream window to document its coupling to sync configuration**" — 2 days old
  - Blocker: The build must pass before a human is asked
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/dash-evo-tool#1096 feat(identity): show contact details and a private note in the hub's Contacts tab](https://github.com/dashpay/dash-evo-tool/pull/1096) — ✋ changes requested · areas: fallback · Policy: waiting-author
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/dash-evo-tool#1110 feat(identity): use proven owner balances and keep identity balances ordered by block height](https://github.com/dashpay/dash-evo-tool/pull/1110) — ⚠ merge conflict · 📝 draft · 🐢 targets chore/bump-platform-v4.3 · areas: fallback
- [dashpay/dash-evo-tool#1113 fix(wallet): refuse Core and Platform sends to someone else's private address](https://github.com/dashpay/dash-evo-tool/pull/1113) — 🔴 CI failing · areas: fallback · Policy: waiting-build
  - Blocker: The build must pass before this can merge
- [dashpay/dash-evo-tool#1115 feat(platform-info): explain that a failed withdrawal is not refunded](https://github.com/dashpay/dash-evo-tool/pull/1115) — ⚠ merge conflict · 📝 draft · 🐢 targets chore/bump-platform-063589bf · areas: fallback
- [dashpay/dash-evo-tool#1116 feat(connection): say when wallet sync data could not be saved](https://github.com/dashpay/dash-evo-tool/pull/1116) — 📝 draft · 🐢 targets chore/bump-platform-063589bf · areas: fallback
- [dashpay/dash-evo-tool#1117 fix(identity): choose signing keys Platform accepts on protocol 14](https://github.com/dashpay/dash-evo-tool/pull/1117) — 📝 draft · 🐢 targets chore/bump-platform-063589bf · areas: fallback
- [dashpay/dash-evo-tool#1118 feat(tokens): create non-transferable tokens and stop offering their transfer](https://github.com/dashpay/dash-evo-tool/pull/1118) — 📝 draft · 🐢 targets chore/bump-platform-063589bf · areas: fallback
- [dashpay/dash-evo-tool#1119 feat(tokens): create and claim once-per-identity token distributions](https://github.com/dashpay/dash-evo-tool/pull/1119) — 📝 draft · 🐢 targets chore/bump-platform-063589bf · areas: fallback
- [dashpay/dash-evo-tool#1120 feat(identity): add keys with spending limits and raise key limits](https://github.com/dashpay/dash-evo-tool/pull/1120) — 📝 draft · 🐢 targets fix/protocol14-signing-key-selection · areas: fallback
- [dashpay/dash-evo-tool#1121 feat(contracts): view and claim contract fee pots](https://github.com/dashpay/dash-evo-tool/pull/1121) — 📝 draft · 🐢 targets fix/protocol14-signing-key-selection · areas: fallback
- [dashpay/dash-evo-tool#1122 feat(documents): agree to document action fees before broadcasting](https://github.com/dashpay/dash-evo-tool/pull/1122) — 📝 draft · 🐢 targets fix/protocol14-signing-key-selection · areas: fallback
- [dashpay/dash-evo-tool#1123 feat(identity): add authentication keys bound to a contract](https://github.com/dashpay/dash-evo-tool/pull/1123) — 📝 draft · 🐢 targets feat/identity-key-limits · areas: fallback
- [dashpay/dash-evo-tool#1124 feat(documents): let the user skip an optional token cost](https://github.com/dashpay/dash-evo-tool/pull/1124) — 📝 draft · 🐢 targets fix/protocol14-signing-key-selection · areas: fallback
- [dashpay/platform#3549 test(platform-wallet): e2e framework + full test suite — triage pins, Found-*/PA-* guards, fail-closed persist, Stage-2 merge](https://github.com/dashpay/platform/pull/3549) — 📝 draft · ⏸ deferred · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk, rust-sdk, kotlin-sdk, github, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5384 fix(platform-wallet)!: persist chain truncations so a stored wallet follows a reorg](https://github.com/dashpay/platform/pull/5384) — ⏸ deferred · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk, kotlin-sdk, fallback
- [dashpay/platform#5417 fix(swift-sdk): reconcile SwiftData transaction accounting per wallet](https://github.com/dashpay/platform/pull/5417) — ⏸ deferred · areas: swift-sdk
- [dashpay/platform#5570 fix(platform-wallet): reject zero-amount shielded spends before reserving notes](https://github.com/dashpay/platform/pull/5570) — 🐢 targets v5.1-dev · areas: rs-platform-wallet · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/platform#5571 test(sdk): fill the verified protocol version in identities contract keys query tests](https://github.com/dashpay/platform/pull/5571) — 🐢 targets v5.1-dev · areas: rust-sdk · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/platform#5572 fix(sdk): query compacted address balances when the recent proof has no boundary](https://github.com/dashpay/platform/pull/5572) — 🐢 targets fix/sdk-contract-keys-test-settings · areas: rust-sdk

<a id="claudius-maginificent-needs-action"></a>
#### Needs action (3)
- [dashpay/tenderdash#1562 docs(consensus): add source-derived consensus protocol specification](https://github.com/dashpay/tenderdash/pull/1562) — 5 unresolved (5 bot) · 3 days stale · ✋ changes requested · areas: fallback · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Spec cites ErrUnsupportedVoteExtensionType that does not exist until #1565 merges**" — 3 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/dash-evo-tool#1114 chore(deps)!: update Platform to v5.1-dev 063589bf and follow the network's protocol version](https://github.com/dashpay/dash-evo-tool/pull/1114) — 1 unresolved (1 bot) · 0 days stale · ✋ changes requested · areas: fallback · Policy: waiting-bots
  - Top thread: "🔴 Blocking: Connection rebuild leaves WalletBackend bound to the old SDK**" — 0 days old
  - Blocker: coderabbitai has not reported for the current head
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/dash-evo-tool#1096 feat(identity): show contact details and a private note in the hub's Contacts tab](https://github.com/dashpay/dash-evo-tool/pull/1096) — ✋ changes requested · areas: fallback · Policy: waiting-author
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: Proceeded without coderabbitai: no review within the configured window

<a id="claudius-maginificent-ready-for-human"></a>
#### Ready for human (3)
- [dashpay/platform#5562 fix(sdk): retry address nonce skew on another node](https://github.com/dashpay/platform/pull/5562) — 3 unresolved (3 bot) · 2 days stale · 🐢 targets v5.1-dev · areas: rust-sdk · Policy: ready-to-merge
  - Top thread: "🟡 Suggestion: Add coverage for the real CheckTx consensus-error conversion path**" — 2 days old
  - Blocker: All policy requirements are satisfied
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/platform#5570 fix(platform-wallet): reject zero-amount shielded spends before reserving notes](https://github.com/dashpay/platform/pull/5570) — 🐢 targets v5.1-dev · areas: rs-platform-wallet · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/platform#5571 test(sdk): fill the verified protocol version in identities contract keys query tests](https://github.com/dashpay/platform/pull/5571) — 🐢 targets v5.1-dev · areas: rust-sdk · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
  - Blocker: Proceeded without coderabbitai: no review within the configured window

<a id="claudius-maginificent-unresolved-comments"></a>
#### Unresolved Comments (2)
- [dashpay/tenderdash#1562 docs(consensus): add source-derived consensus protocol specification](https://github.com/dashpay/tenderdash/pull/1562) — 5 unresolved (5 bot) · 3 days stale · ✋ changes requested · areas: fallback · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Spec cites ErrUnsupportedVoteExtensionType that does not exist until #1565 merges**" — 3 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/dash-evo-tool#1114 chore(deps)!: update Platform to v5.1-dev 063589bf and follow the network's protocol version](https://github.com/dashpay/dash-evo-tool/pull/1114) — 1 unresolved (1 bot) · 0 days stale · ✋ changes requested · areas: fallback · Policy: waiting-bots
  - Top thread: "🔴 Blocking: Connection rebuild leaves WalletBackend bound to the old SDK**" — 0 days old
  - Blocker: coderabbitai has not reported for the current head
  - Blocker: thepastaclaw left review threads unresolved; resolve them

<a id="claudius-maginificent-changes-requested"></a>
#### Changes Requested (1)
- [dashpay/dash-evo-tool#1096 feat(identity): show contact details and a private note in the hub's Contacts tab](https://github.com/dashpay/dash-evo-tool/pull/1096) — ✋ changes requested · areas: fallback · Policy: waiting-author
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: Proceeded without coderabbitai: no review within the configured window

<a id="claudius-maginificent-ci-failing"></a>
#### CI Failing (1)
- [dashpay/dash-evo-tool#1113 fix(wallet): refuse Core and Platform sends to someone else's private address](https://github.com/dashpay/dash-evo-tool/pull/1113) — 🔴 CI failing · areas: fallback · Policy: waiting-build
  - Blocker: The build must pass before this can merge

<a id="claudius-maginificent-deferred"></a>
#### Deferred (3)
- [dashpay/platform#3549 test(platform-wallet): e2e framework + full test suite — triage pins, Found-*/PA-* guards, fail-closed persist, Stage-2 merge](https://github.com/dashpay/platform/pull/3549) — 📝 draft · ⏸ deferred · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk, rust-sdk, kotlin-sdk, github, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5384 fix(platform-wallet)!: persist chain truncations so a stored wallet follows a reorg](https://github.com/dashpay/platform/pull/5384) — ⏸ deferred · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk, kotlin-sdk, fallback
- [dashpay/platform#5417 fix(swift-sdk): reconcile SwiftData transaction accounting per wallet](https://github.com/dashpay/platform/pull/5417) — ⏸ deferred · areas: swift-sdk

<a id="claudius-maginificent-draft"></a>
#### Draft (1)
- [dashpay/dash-evo-tool#1111 docs(wallet): redesign the Send screen as one From/To form](https://github.com/dashpay/dash-evo-tool/pull/1111) — 2 unresolved (2 bot) · 2 days stale · 📝 draft · areas: fallback · Policy: draft
  - Top thread: "🟡 Suggestion: Define how destination Auto handles an address that becomes used**" — 2 days old
  - Blocker: Draft PR does not occupy a review slot

<a id="claudius-maginificent-stale"></a>
#### Stale (22)
- [dashpay/platform#5565 feat(platform-wallet)!: detect inconsistent shielded commitment trees and add explicit rebuild](https://github.com/dashpay/platform/pull/5565) — 4 unresolved (4 bot) · 1 days stale · ✋ changes requested · 🐢 targets fix/shielded-sync-concurrent-append · areas: rs-platform-wallet
  - Top thread: "🟡 Suggestion: Reset host-visible scan metadata during a tree rebuild**" — 1 days old
- [dashpay/dash-evo-tool#1108 feat(documents): recognise indexOnly document types and delete their entries by values](https://github.com/dashpay/dash-evo-tool/pull/1108) — 3 unresolved (3 bot) · 2 days stale · ⚠ merge conflict · ✋ changes requested · 📝 draft · 🐢 targets chore/bump-platform-v4.3 · areas: fallback
  - Top thread: "🟡 Suggestion: Make the non-deletable error actionable**" — 2 days old
- [dashpay/dash-evo-tool#1109 feat(masternode): use wallet-held node keys and withdraw via platform-wallet](https://github.com/dashpay/dash-evo-tool/pull/1109) — 3 unresolved (3 bot) · 1 days stale · ✋ changes requested · 📝 draft · 🐢 targets chore/bump-platform-v4.3 · areas: fallback
  - Top thread: "🔴 Blocking: Do not let a disabled stored key suppress discovery of the current node key**" — 1 days old
- [dashpay/dash-evo-tool#1112 feat(send): one From/To Send form with Auto routing](https://github.com/dashpay/dash-evo-tool/pull/1112) — 3 unresolved (3 bot) · 1 days stale · ✋ changes requested · 📝 draft · 🐢 targets feat/wallet-max-send-limits · areas: fallback
  - Top thread: "🟡 Suggestion: Do not rotate an own-wallet destination on every refresh**" — 1 days old
- [dashpay/platform#5562 fix(sdk): retry address nonce skew on another node](https://github.com/dashpay/platform/pull/5562) — 3 unresolved (3 bot) · 2 days stale · 🐢 targets v5.1-dev · areas: rust-sdk · Policy: ready-to-merge
  - Top thread: "🟡 Suggestion: Add coverage for the real CheckTx consensus-error conversion path**" — 2 days old
  - Blocker: All policy requirements are satisfied
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/platform#5546 fix(platform-wallet): support HASH160 DashPay contact signing keys](https://github.com/dashpay/platform/pull/5546) — 1 unresolved (1 bot) · 2 days stale · ⚠ merge conflict · 📝 draft · 🐢 targets v5.1-dev · areas: rs-platform-wallet · Policy: draft
  - Top thread: "🟡 Suggestion: contact_request_signing_key can select bounded or expired keys that cannot complete the write**" — 2 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/dash-evo-tool#1107 fix(wallet): make every Max and fee match what the upstream builders accept](https://github.com/dashpay/dash-evo-tool/pull/1107) — 1 unresolved (1 bot) · 1 days stale · ✋ changes requested · 🐢 targets chore/bump-platform-v4.3 · areas: fallback
  - Top thread: "🟡 Suggestion: Preserve typed insufficient-funds errors when pooled finalization reports no UTXOs**" — 1 days old
- [dashpay/platform#5564 fix(platform-wallet): stop concurrent shielded syncs from duplicating commitments](https://github.com/dashpay/platform/pull/5564) — 1 unresolved (1 bot) · 2 days stale · 🔴 CI failing · 🐢 targets v5.1-dev · areas: rs-platform-wallet · Policy: waiting-build
  - Top thread: "🟡 Suggestion: Name the mock stream window to document its coupling to sync configuration**" — 2 days old
  - Blocker: The build must pass before a human is asked
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/dash-evo-tool#1110 feat(identity): use proven owner balances and keep identity balances ordered by block height](https://github.com/dashpay/dash-evo-tool/pull/1110) — ⚠ merge conflict · 📝 draft · 🐢 targets chore/bump-platform-v4.3 · areas: fallback
- [dashpay/dash-evo-tool#1115 feat(platform-info): explain that a failed withdrawal is not refunded](https://github.com/dashpay/dash-evo-tool/pull/1115) — ⚠ merge conflict · 📝 draft · 🐢 targets chore/bump-platform-063589bf · areas: fallback
- [dashpay/dash-evo-tool#1116 feat(connection): say when wallet sync data could not be saved](https://github.com/dashpay/dash-evo-tool/pull/1116) — 📝 draft · 🐢 targets chore/bump-platform-063589bf · areas: fallback
- [dashpay/dash-evo-tool#1117 fix(identity): choose signing keys Platform accepts on protocol 14](https://github.com/dashpay/dash-evo-tool/pull/1117) — 📝 draft · 🐢 targets chore/bump-platform-063589bf · areas: fallback
- [dashpay/dash-evo-tool#1118 feat(tokens): create non-transferable tokens and stop offering their transfer](https://github.com/dashpay/dash-evo-tool/pull/1118) — 📝 draft · 🐢 targets chore/bump-platform-063589bf · areas: fallback
- [dashpay/dash-evo-tool#1119 feat(tokens): create and claim once-per-identity token distributions](https://github.com/dashpay/dash-evo-tool/pull/1119) — 📝 draft · 🐢 targets chore/bump-platform-063589bf · areas: fallback
- [dashpay/dash-evo-tool#1120 feat(identity): add keys with spending limits and raise key limits](https://github.com/dashpay/dash-evo-tool/pull/1120) — 📝 draft · 🐢 targets fix/protocol14-signing-key-selection · areas: fallback
- [dashpay/dash-evo-tool#1121 feat(contracts): view and claim contract fee pots](https://github.com/dashpay/dash-evo-tool/pull/1121) — 📝 draft · 🐢 targets fix/protocol14-signing-key-selection · areas: fallback
- [dashpay/dash-evo-tool#1122 feat(documents): agree to document action fees before broadcasting](https://github.com/dashpay/dash-evo-tool/pull/1122) — 📝 draft · 🐢 targets fix/protocol14-signing-key-selection · areas: fallback
- [dashpay/dash-evo-tool#1123 feat(identity): add authentication keys bound to a contract](https://github.com/dashpay/dash-evo-tool/pull/1123) — 📝 draft · 🐢 targets feat/identity-key-limits · areas: fallback
- [dashpay/dash-evo-tool#1124 feat(documents): let the user skip an optional token cost](https://github.com/dashpay/dash-evo-tool/pull/1124) — 📝 draft · 🐢 targets fix/protocol14-signing-key-selection · areas: fallback
- [dashpay/platform#5570 fix(platform-wallet): reject zero-amount shielded spends before reserving notes](https://github.com/dashpay/platform/pull/5570) — 🐢 targets v5.1-dev · areas: rs-platform-wallet · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/platform#5571 test(sdk): fill the verified protocol version in identities contract keys query tests](https://github.com/dashpay/platform/pull/5571) — 🐢 targets v5.1-dev · areas: rust-sdk · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/platform#5572 fix(sdk): query compacted address balances when the recent proof has no boundary](https://github.com/dashpay/platform/pull/5572) — 🐢 targets fix/sdk-contract-keys-test-settings · areas: rust-sdk

<a id="lklimek"></a>
### @lklimek
<a id="lklimek-open"></a>
#### Open (15)
- [dashpay/platform#5528 fix(platform-wallet): retain drain operation history through funding conflicts](https://github.com/dashpay/platform/pull/5528) — 7 unresolved (7 bot) · 3 days stale · ✋ changes requested · 📝 draft · 🐢 targets feat/sqlite-wallet-restore · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk, kotlin-sdk
  - Top thread: "🟡 Suggestion: Retain diagnostics for malformed FFI operation snapshots**" — 3 days old
- [dashpay/dash-evo-tool#1100 refactor(app): separate task routing, domain state, and shutdown responsibilities](https://github.com/dashpay/dash-evo-tool/pull/1100) — 4 unresolved (4 bot) · 1 days stale · ⚠ merge conflict · ✋ changes requested · 📝 draft · areas: fallback · Policy: draft
  - Top thread: "🟡 Suggestion: Attribute progress events to their dispatch owner**" — 1 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5519 refactor: remove DPP raw secp256k1 dependency](https://github.com/dashpay/platform/pull/5519) — 2 unresolved (2 bot) · 4 days stale · ⚠ merge conflict · 📝 draft · 🐢 targets v5.1-dev · areas: rs-drive-abci, dpp, rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, rust-sdk, rust-sdk-ffi, js-wasm-sdk, fallback · Policy: draft
  - Top thread: "🟡 Suggestion: Erase the raw ECDH secret created by the test provider**" — 4 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/tenderdash#1515 fix(service)!: join remaining service workers before shutdown completes](https://github.com/dashpay/tenderdash/pull/1515) — 1 unresolved (1 bot) · 2 days stale · ✋ changes requested · areas: fallback · Policy: waiting-author
  - Top thread: "🔴 Blocking: Release node-owned resources when startup fails**" — 2 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/tenderdash#1550 fix(p2p)!: enforce peer penalties and validate network settings](https://github.com/dashpay/tenderdash/pull/1550) — 1 unresolved (1 bot) · 2 days stale · ✋ changes requested · areas: fallback · Policy: waiting-author
  - Top thread: "🔴 Blocking: Do not reject max-connections zero before its runtime default is applied**" — 2 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#4740 fix(platform-wallet): rescan DashPay contact accounts from the contact request height](https://github.com/dashpay/platform/pull/4740) — 🐢 targets v5.1-dev · areas: rs-platform-wallet, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 6d293f2a0890d6b374a999e4fc6f760c409b434a
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/platform#5150 fix(platform-wallet)!: restore Core spending state and repair persisted accounting](https://github.com/dashpay/platform/pull/5150) — ⏸ deferred · areas: rs-platform-wallet, wallet-storage, fallback · Policy: waiting-author
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/platform#5207 fix(platform-wallet): require only atomic tracked-lock writes for reconciliation](https://github.com/dashpay/platform/pull/5207) — ⏸ deferred · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage
- [dashpay/platform#5210 feat(wallet-storage)!: restore complete Core wallet snapshots from SQLite](https://github.com/dashpay/platform/pull/5210) — ⏸ deferred · areas: rs-platform-wallet, wallet-storage
- [dashpay/platform#5220 fix(platform-wallet)!: replay recorded transaction history on load for every persister](https://github.com/dashpay/platform/pull/5220) — ⏸ deferred · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk, fallback
- [dashpay/platform#5323 refactor: reject legacy BLS storage formats](https://github.com/dashpay/platform/pull/5323) — 📝 draft · 🐢 targets v5.1-dev · areas: rs-drive-abci · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5489 fix!: set zero consensus evidence budget at protocol 15](https://github.com/dashpay/platform/pull/5489) — ⚠ merge conflict · 📝 draft · 🐢 targets v5.1-dev · areas: rs-drive-abci, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/tenderdash#1552 chore(release)!: prepare dev branch and Platform release workflow](https://github.com/dashpay/tenderdash/pull/1552) — 📝 draft · areas: github, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/tenderdash#1560 test(e2e): exercise block sync during slow local application](https://github.com/dashpay/tenderdash/pull/1560) — areas: github, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 1f5fb2540ca89b3c6b2bb311ad149affdd577df4
- [dashpay/tenderdash#1567 fix(consensus): replay blocks with their historical quorum](https://github.com/dashpay/tenderdash/pull/1567) — 📝 draft · areas: fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot

<a id="lklimek-needs-action"></a>
#### Needs action (2)
- [dashpay/tenderdash#1515 fix(service)!: join remaining service workers before shutdown completes](https://github.com/dashpay/tenderdash/pull/1515) — 1 unresolved (1 bot) · 2 days stale · ✋ changes requested · areas: fallback · Policy: waiting-author
  - Top thread: "🔴 Blocking: Release node-owned resources when startup fails**" — 2 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/tenderdash#1550 fix(p2p)!: enforce peer penalties and validate network settings](https://github.com/dashpay/tenderdash/pull/1550) — 1 unresolved (1 bot) · 2 days stale · ✋ changes requested · areas: fallback · Policy: waiting-author
  - Top thread: "🔴 Blocking: Do not reject max-connections zero before its runtime default is applied**" — 2 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them

<a id="lklimek-unresolved-comments"></a>
#### Unresolved Comments (2)
- [dashpay/tenderdash#1515 fix(service)!: join remaining service workers before shutdown completes](https://github.com/dashpay/tenderdash/pull/1515) — 1 unresolved (1 bot) · 2 days stale · ✋ changes requested · areas: fallback · Policy: waiting-author
  - Top thread: "🔴 Blocking: Release node-owned resources when startup fails**" — 2 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/tenderdash#1550 fix(p2p)!: enforce peer penalties and validate network settings](https://github.com/dashpay/tenderdash/pull/1550) — 1 unresolved (1 bot) · 2 days stale · ✋ changes requested · areas: fallback · Policy: waiting-author
  - Top thread: "🔴 Blocking: Do not reject max-connections zero before its runtime default is applied**" — 2 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them

<a id="lklimek-deferred"></a>
#### Deferred (4)
- [dashpay/platform#5150 fix(platform-wallet)!: restore Core spending state and repair persisted accounting](https://github.com/dashpay/platform/pull/5150) — ⏸ deferred · areas: rs-platform-wallet, wallet-storage, fallback · Policy: waiting-author
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/platform#5207 fix(platform-wallet): require only atomic tracked-lock writes for reconciliation](https://github.com/dashpay/platform/pull/5207) — ⏸ deferred · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage
- [dashpay/platform#5210 feat(wallet-storage)!: restore complete Core wallet snapshots from SQLite](https://github.com/dashpay/platform/pull/5210) — ⏸ deferred · areas: rs-platform-wallet, wallet-storage
- [dashpay/platform#5220 fix(platform-wallet)!: replay recorded transaction history on load for every persister](https://github.com/dashpay/platform/pull/5220) — ⏸ deferred · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk, fallback

<a id="lklimek-draft"></a>
#### Draft (3)
- [dashpay/dash-evo-tool#1100 refactor(app): separate task routing, domain state, and shutdown responsibilities](https://github.com/dashpay/dash-evo-tool/pull/1100) — 4 unresolved (4 bot) · 1 days stale · ⚠ merge conflict · ✋ changes requested · 📝 draft · areas: fallback · Policy: draft
  - Top thread: "🟡 Suggestion: Attribute progress events to their dispatch owner**" — 1 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/tenderdash#1552 chore(release)!: prepare dev branch and Platform release workflow](https://github.com/dashpay/tenderdash/pull/1552) — 📝 draft · areas: github, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/tenderdash#1567 fix(consensus): replay blocks with their historical quorum](https://github.com/dashpay/tenderdash/pull/1567) — 📝 draft · areas: fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot

<a id="lklimek-stale"></a>
#### Stale (5)
- [dashpay/platform#5528 fix(platform-wallet): retain drain operation history through funding conflicts](https://github.com/dashpay/platform/pull/5528) — 7 unresolved (7 bot) · 3 days stale · ✋ changes requested · 📝 draft · 🐢 targets feat/sqlite-wallet-restore · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk, kotlin-sdk
  - Top thread: "🟡 Suggestion: Retain diagnostics for malformed FFI operation snapshots**" — 3 days old
- [dashpay/platform#5519 refactor: remove DPP raw secp256k1 dependency](https://github.com/dashpay/platform/pull/5519) — 2 unresolved (2 bot) · 4 days stale · ⚠ merge conflict · 📝 draft · 🐢 targets v5.1-dev · areas: rs-drive-abci, dpp, rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, rust-sdk, rust-sdk-ffi, js-wasm-sdk, fallback · Policy: draft
  - Top thread: "🟡 Suggestion: Erase the raw ECDH secret created by the test provider**" — 4 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#4740 fix(platform-wallet): rescan DashPay contact accounts from the contact request height](https://github.com/dashpay/platform/pull/4740) — 🐢 targets v5.1-dev · areas: rs-platform-wallet, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 6d293f2a0890d6b374a999e4fc6f760c409b434a
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/platform#5323 refactor: reject legacy BLS storage formats](https://github.com/dashpay/platform/pull/5323) — 📝 draft · 🐢 targets v5.1-dev · areas: rs-drive-abci · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5489 fix!: set zero consensus evidence budget at protocol 15](https://github.com/dashpay/platform/pull/5489) — ⚠ merge conflict · 📝 draft · 🐢 targets v5.1-dev · areas: rs-drive-abci, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot

<a id="lklimek-clean"></a>
#### Clean (1)
- [dashpay/tenderdash#1560 test(e2e): exercise block sync during slow local application](https://github.com/dashpay/tenderdash/pull/1560) — areas: github, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 1f5fb2540ca89b3c6b2bb311ad149affdd577df4

<a id="lklimek-ready-for-review"></a>
#### Ready for Review (3)
- [dashpay/platform#5558 fix(sdk): check document query shapes before sending them](https://github.com/dashpay/platform/pull/5558) — by @shumkov · areas: rs-drive-abci, rust-sdk, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 5f76f73f741c398ae76dd078ef5c5a79031edacf
- [dashpay/platform#5559 test(sdk): run operation-proof tests in CI and select the verifier generation from the verifier](https://github.com/dashpay/platform/pull/5559) — by @shumkov · areas: rs-drive, rs-drive-abci, rust-sdk, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed fe8a7dde6e85b1455da3ac3ba1412675516a9625
- [dashpay/platform#5561 fix(sdk)!: refuse requests the network cannot serve yet instead of banning nodes](https://github.com/dashpay/platform/pull/5561) — by @shumkov · areas: rs-drive-abci, rust-sdk, rust-sdk-ffi, js-wasm-sdk, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed b36f0017d00008f28067484b3f6c1930d52c7867

<a id="shumkov"></a>
### @shumkov
<a id="shumkov-open"></a>
#### Open (14)
- [dashpay/rust-dashcore#634 feat(key-wallet-manager): single-map WalletManager with Arc&lt;RwLock&lt;T&gt;&gt; per wallet](https://github.com/dashpay/rust-dashcore/pull/634) — 20 unresolved (20 CodeRabbit) · 200 days stale · ⚠ merge conflict · ✋ changes requested · areas: dash-spv, key-wallet, key-wallet-manager, fallback · Policy: waiting-author
  - Top thread: "_⚠️ Potential issue_ \| _🟡 Minor_" — 200 days old
  - Blocker: coderabbitai requested changes on this head; dismiss the review or push a fix
  - Blocker: coderabbitai left review threads unresolved; resolve them
- [dashpay/platform#4660 feat(sdk): expose the document erase and history lifecycle on mobile and FFI](https://github.com/dashpay/platform/pull/4660) — 2 unresolved (2 bot) · 32 days stale · ✋ changes requested · 🔴 CI failing · 🐢 targets keep-history-lifecycle · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, rust-sdk-ffi, kotlin-sdk, fallback
  - Top thread: "🟡 Suggestion: Validate history selector discriminant at the C boundary**" — 32 days old
- [dashpay/platform#4657 feat(platform)!: delete and erase lifecycle for keep-history documents](https://github.com/dashpay/platform/pull/4657) — 2 unresolved (2 bot) · 32 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · 🐢 targets keep-history-storage-v2 · areas: rs-drive, rs-drive-abci, dpp, rust-sdk, js-wasm-sdk, fallback
  - Top thread: "🟡 Suggestion: Deduplicate lifecycle containers at the Drive batch boundary**" — 32 days old
- [dashpay/platform#4283 fix(dashmate): keep the node up when an image pull fails, and stop reporting success when it did not](https://github.com/dashpay/platform/pull/4283) — 2 unresolved (2 bot) · 5 days stale · ✋ changes requested · 🐢 targets v5.1-dev · areas: dashmate · Policy: waiting-author
  - Top thread: "🔴 Blocking: Preserve Docker login credentials in the restart preflight**" — 5 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/platform#4652 feat(drive)!: preserve composite document history across protocol activation](https://github.com/dashpay/platform/pull/4652) — 2 unresolved (2 bot) · 3 days stale · ⚠ merge conflict · ✋ changes requested · 🐢 targets v5.1-dev · areas: rs-drive, rs-drive-abci, rust-sdk, js-wasm-sdk, fallback · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Track composable offset and absence proofs in GroveDB**" — 3 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/platform#5560 fix(sdk): retry document queries answered by nodes on an older protocol version](https://github.com/dashpay/platform/pull/5560) — 2 unresolved (2 bot) · 1 days stale · areas: rust-sdk, rust-sdk-ffi, js-wasm-sdk, github, fallback · Policy: waiting-self-review
  - Top thread: "🟡 Suggestion: Declare public error-enum additions as an API-breaking release change**" — 1 days old
  - Blocker: Author must post /self-reviewed b631f72c6fb5636256229de9112ca6d2fd1bde1e
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/platform#4282 fix(dashmate)!: give Debian packages versions apt can order](https://github.com/dashpay/platform/pull/4282) — 🐢 targets v5.1-dev · areas: dashmate, github, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/platform#5557 fix(drive-abci): answer queries for features not active yet with INVALID_ARGUMENT](https://github.com/dashpay/platform/pull/5557) — areas: rs-drive-abci, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 98591f1166b60c256a3afbabec8186a87b9702f7
- [dashpay/platform#5558 fix(sdk): check document query shapes before sending them](https://github.com/dashpay/platform/pull/5558) — areas: rs-drive-abci, rust-sdk, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 5f76f73f741c398ae76dd078ef5c5a79031edacf
- [dashpay/platform#5559 test(sdk): run operation-proof tests in CI and select the verifier generation from the verifier](https://github.com/dashpay/platform/pull/5559) — areas: rs-drive, rs-drive-abci, rust-sdk, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed fe8a7dde6e85b1455da3ac3ba1412675516a9625
- [dashpay/platform#5561 fix(sdk)!: refuse requests the network cannot serve yet instead of banning nodes](https://github.com/dashpay/platform/pull/5561) — areas: rs-drive-abci, rust-sdk, rust-sdk-ffi, js-wasm-sdk, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed b36f0017d00008f28067484b3f6c1930d52c7867
- [dashpay/rust-dashcore#1205 fix(dash-spv)!: keep masternode sync live after refused responses](https://github.com/dashpay/rust-dashcore/pull/1205) — ✋ changes requested · 🔴 CI failing · 🐢 targets shumkov/network-request-avoiding-peers · areas: dash-spv
- [dashpay/rust-dashcore#1206 feat(dash-spv)!: check masternode list diffs against the block coinbase](https://github.com/dashpay/rust-dashcore/pull/1206) — 🔴 CI failing · 📝 draft · 🐢 targets shumkov/masternode-sync-liveness · areas: dash-spv, fallback
- [dashpay/rust-dashcore#1212 feat(dash-spv)!: let a network request avoid given peers](https://github.com/dashpay/rust-dashcore/pull/1212) — 🔴 CI failing · areas: dash-spv · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 1c67e9ad99aa3b92f537285355aa157dfd5ee52a

<a id="shumkov-needs-action"></a>
#### Needs action (2)
- [dashpay/rust-dashcore#634 feat(key-wallet-manager): single-map WalletManager with Arc&lt;RwLock&lt;T&gt;&gt; per wallet](https://github.com/dashpay/rust-dashcore/pull/634) — 20 unresolved (20 CodeRabbit) · 200 days stale · ⚠ merge conflict · ✋ changes requested · areas: dash-spv, key-wallet, key-wallet-manager, fallback · Policy: waiting-author
  - Top thread: "_⚠️ Potential issue_ \| _🟡 Minor_" — 200 days old
  - Blocker: coderabbitai requested changes on this head; dismiss the review or push a fix
  - Blocker: coderabbitai left review threads unresolved; resolve them
- [dashpay/platform#5560 fix(sdk): retry document queries answered by nodes on an older protocol version](https://github.com/dashpay/platform/pull/5560) — 2 unresolved (2 bot) · 1 days stale · areas: rust-sdk, rust-sdk-ffi, js-wasm-sdk, github, fallback · Policy: waiting-self-review
  - Top thread: "🟡 Suggestion: Declare public error-enum additions as an API-breaking release change**" — 1 days old
  - Blocker: Author must post /self-reviewed b631f72c6fb5636256229de9112ca6d2fd1bde1e
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return

<a id="shumkov-ready-for-human"></a>
#### Ready for human (1)
- [dashpay/platform#4282 fix(dashmate)!: give Debian packages versions apt can order](https://github.com/dashpay/platform/pull/4282) — 🐢 targets v5.1-dev · areas: dashmate, github, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
  - Blocker: Proceeded without coderabbitai: no review within the configured window

<a id="shumkov-unresolved-comments"></a>
#### Unresolved Comments (2)
- [dashpay/rust-dashcore#634 feat(key-wallet-manager): single-map WalletManager with Arc&lt;RwLock&lt;T&gt;&gt; per wallet](https://github.com/dashpay/rust-dashcore/pull/634) — 20 unresolved (20 CodeRabbit) · 200 days stale · ⚠ merge conflict · ✋ changes requested · areas: dash-spv, key-wallet, key-wallet-manager, fallback · Policy: waiting-author
  - Top thread: "_⚠️ Potential issue_ \| _🟡 Minor_" — 200 days old
  - Blocker: coderabbitai requested changes on this head; dismiss the review or push a fix
  - Blocker: coderabbitai left review threads unresolved; resolve them
- [dashpay/platform#5560 fix(sdk): retry document queries answered by nodes on an older protocol version](https://github.com/dashpay/platform/pull/5560) — 2 unresolved (2 bot) · 1 days stale · areas: rust-sdk, rust-sdk-ffi, js-wasm-sdk, github, fallback · Policy: waiting-self-review
  - Top thread: "🟡 Suggestion: Declare public error-enum additions as an API-breaking release change**" — 1 days old
  - Blocker: Author must post /self-reviewed b631f72c6fb5636256229de9112ca6d2fd1bde1e
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return

<a id="shumkov-ci-failing"></a>
#### CI Failing (1)
- [dashpay/rust-dashcore#1212 feat(dash-spv)!: let a network request avoid given peers](https://github.com/dashpay/rust-dashcore/pull/1212) — 🔴 CI failing · areas: dash-spv · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 1c67e9ad99aa3b92f537285355aa157dfd5ee52a

<a id="shumkov-stale"></a>
#### Stale (7)
- [dashpay/platform#4660 feat(sdk): expose the document erase and history lifecycle on mobile and FFI](https://github.com/dashpay/platform/pull/4660) — 2 unresolved (2 bot) · 32 days stale · ✋ changes requested · 🔴 CI failing · 🐢 targets keep-history-lifecycle · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, rust-sdk-ffi, kotlin-sdk, fallback
  - Top thread: "🟡 Suggestion: Validate history selector discriminant at the C boundary**" — 32 days old
- [dashpay/platform#4657 feat(platform)!: delete and erase lifecycle for keep-history documents](https://github.com/dashpay/platform/pull/4657) — 2 unresolved (2 bot) · 32 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · 🐢 targets keep-history-storage-v2 · areas: rs-drive, rs-drive-abci, dpp, rust-sdk, js-wasm-sdk, fallback
  - Top thread: "🟡 Suggestion: Deduplicate lifecycle containers at the Drive batch boundary**" — 32 days old
- [dashpay/platform#4283 fix(dashmate): keep the node up when an image pull fails, and stop reporting success when it did not](https://github.com/dashpay/platform/pull/4283) — 2 unresolved (2 bot) · 5 days stale · ✋ changes requested · 🐢 targets v5.1-dev · areas: dashmate · Policy: waiting-author
  - Top thread: "🔴 Blocking: Preserve Docker login credentials in the restart preflight**" — 5 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/platform#4652 feat(drive)!: preserve composite document history across protocol activation](https://github.com/dashpay/platform/pull/4652) — 2 unresolved (2 bot) · 3 days stale · ⚠ merge conflict · ✋ changes requested · 🐢 targets v5.1-dev · areas: rs-drive, rs-drive-abci, rust-sdk, js-wasm-sdk, fallback · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Track composable offset and absence proofs in GroveDB**" — 3 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/platform#4282 fix(dashmate)!: give Debian packages versions apt can order](https://github.com/dashpay/platform/pull/4282) — 🐢 targets v5.1-dev · areas: dashmate, github, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/rust-dashcore#1205 fix(dash-spv)!: keep masternode sync live after refused responses](https://github.com/dashpay/rust-dashcore/pull/1205) — ✋ changes requested · 🔴 CI failing · 🐢 targets shumkov/network-request-avoiding-peers · areas: dash-spv
- [dashpay/rust-dashcore#1206 feat(dash-spv)!: check masternode list diffs against the block coinbase](https://github.com/dashpay/rust-dashcore/pull/1206) — 🔴 CI failing · 📝 draft · 🐢 targets shumkov/masternode-sync-liveness · areas: dash-spv, fallback

<a id="shumkov-clean"></a>
#### Clean (4)
- [dashpay/platform#5557 fix(drive-abci): answer queries for features not active yet with INVALID_ARGUMENT](https://github.com/dashpay/platform/pull/5557) — areas: rs-drive-abci, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 98591f1166b60c256a3afbabec8186a87b9702f7
- [dashpay/platform#5558 fix(sdk): check document query shapes before sending them](https://github.com/dashpay/platform/pull/5558) — areas: rs-drive-abci, rust-sdk, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 5f76f73f741c398ae76dd078ef5c5a79031edacf
- [dashpay/platform#5559 test(sdk): run operation-proof tests in CI and select the verifier generation from the verifier](https://github.com/dashpay/platform/pull/5559) — areas: rs-drive, rs-drive-abci, rust-sdk, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed fe8a7dde6e85b1455da3ac3ba1412675516a9625
- [dashpay/platform#5561 fix(sdk)!: refuse requests the network cannot serve yet instead of banning nodes](https://github.com/dashpay/platform/pull/5561) — areas: rs-drive-abci, rust-sdk, rust-sdk-ffi, js-wasm-sdk, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed b36f0017d00008f28067484b3f6c1930d52c7867

<a id="shumkov-ready-for-review"></a>
#### Ready for Review (1)
- [dashpay/tenderdash#1560 test(e2e): exercise block sync during slow local application](https://github.com/dashpay/tenderdash/pull/1560) — by @lklimek · areas: github, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 1f5fb2540ca89b3c6b2bb311ad149affdd577df4

<a id="llbartekll"></a>
### @llbartekll
<a id="llbartekll-open"></a>
#### Open (5)
- [dashpay/platform#3560 test(swift-sdk): add testnet identity-discovery UI test](https://github.com/dashpay/platform/pull/3560) — 1 unresolved (1 bot) · 146 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · areas: swift-sdk, github · Policy: waiting-author
  - Top thread: "🔴 Blocking: Carried-forward prior finding (STILL VALID): Docker-setup toggle leaves the cached regtest wallet manager bo…" — 146 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/platform#5511 feat(sdk): expose cached verified protocol version to Swift](https://github.com/dashpay/platform/pull/5511) — 3 unresolved (3 bot) · 2 days stale · 🐢 targets v5.1-dev · areas: swift-sdk, rust-sdk, rust-sdk-ffi · Policy: waiting-self-review
  - Top thread: "✅ **Resolved** at \`5b89fe8b\`; see the replies below." — 2 days old
  - Blocker: Author must post /self-reviewed 5b89fe8b529c66d5e972880196545c902285258b
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/platform#5556 fix(sdk)!: preserve uncertain submission results for recovery](https://github.com/dashpay/platform/pull/5556) — 📝 draft · 🐢 targets codex/protocol-compatibility · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, rust-sdk, rust-sdk-ffi, js-wasm-sdk, kotlin-sdk, fallback
- [dashpay/rust-dashcore#819 feat(key-wallet): CoinJoin sweep + gap-limit recovery; TransactionBuilder drain mode](https://github.com/dashpay/rust-dashcore/pull/819) — ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · areas: key-wallet · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed e406d1cbb86a03503fa41a9d796f25c0bacc3e12
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/rust-dashcore#910 fix(dash-spv): re-anchor incompatible shared storage](https://github.com/dashpay/rust-dashcore/pull/910) — ⚠ merge conflict · 📝 draft · areas: dash-spv · Policy: draft
  - Blocker: Draft PR does not occupy a review slot

<a id="llbartekll-needs-action"></a>
#### Needs action (2)
- [dashpay/platform#3560 test(swift-sdk): add testnet identity-discovery UI test](https://github.com/dashpay/platform/pull/3560) — 1 unresolved (1 bot) · 146 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · areas: swift-sdk, github · Policy: waiting-author
  - Top thread: "🔴 Blocking: Carried-forward prior finding (STILL VALID): Docker-setup toggle leaves the cached regtest wallet manager bo…" — 146 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/rust-dashcore#819 feat(key-wallet): CoinJoin sweep + gap-limit recovery; TransactionBuilder drain mode](https://github.com/dashpay/rust-dashcore/pull/819) — ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · areas: key-wallet · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed e406d1cbb86a03503fa41a9d796f25c0bacc3e12
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return

<a id="llbartekll-unresolved-comments"></a>
#### Unresolved Comments (1)
- [dashpay/platform#3560 test(swift-sdk): add testnet identity-discovery UI test](https://github.com/dashpay/platform/pull/3560) — 1 unresolved (1 bot) · 146 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · areas: swift-sdk, github · Policy: waiting-author
  - Top thread: "🔴 Blocking: Carried-forward prior finding (STILL VALID): Docker-setup toggle leaves the cached regtest wallet manager bo…" — 146 days old
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

<a id="llbartekll-stale"></a>
#### Stale (2)
- [dashpay/platform#5511 feat(sdk): expose cached verified protocol version to Swift](https://github.com/dashpay/platform/pull/5511) — 3 unresolved (3 bot) · 2 days stale · 🐢 targets v5.1-dev · areas: swift-sdk, rust-sdk, rust-sdk-ffi · Policy: waiting-self-review
  - Top thread: "✅ **Resolved** at \`5b89fe8b\`; see the replies below." — 2 days old
  - Blocker: Author must post /self-reviewed 5b89fe8b529c66d5e972880196545c902285258b
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/platform#5556 fix(sdk)!: preserve uncertain submission results for recovery](https://github.com/dashpay/platform/pull/5556) — 📝 draft · 🐢 targets codex/protocol-compatibility · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, rust-sdk, rust-sdk-ffi, js-wasm-sdk, kotlin-sdk, fallback

<a id="xdustinface"></a>
### @xdustinface
<a id="xdustinface-open"></a>
#### Open (8)
- [dashpay/rust-dashcore#856 feat(dash-spv): force resync to re-anchor a running client at a lower checkpoint](https://github.com/dashpay/rust-dashcore/pull/856) — 1 unresolved (1 human) · 96 days stale · ⚠ merge conflict · 🔴 CI failing · areas: dash-spv · Policy: waiting-self-review
  - Top thread: "why do we have a periodic check here??" — 96 days old
  - Blocker: Author must post /self-reviewed 7282172a3bb4a0d9d85a40cf70b4dd111995f96b
- [dashpay/rust-dashcore#400 chore: remove BIP141 (SegWit) and BIP341 (Taproot) support](https://github.com/dashpay/rust-dashcore/pull/400) — ⚠ merge conflict · 🔴 CI failing · 📝 draft · 🐢 untouched 158 days · areas: dash-spv, key-wallet, github, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#541 ci: integrate Codecov test analytics via \`cargo-nextest\`](https://github.com/dashpay/rust-dashcore/pull/541) — ⚠ merge conflict · 📝 draft · areas: github, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#720 fix(dash-spv): split peer \`TcpStream\` and add per-peer writer task](https://github.com/dashpay/rust-dashcore/pull/720) — ⚠ merge conflict · 🔴 CI failing · 📝 draft · 🐢 untouched 158 days · areas: dash-spv, fallback · Policy: draft
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
- [dashpay/rust-dashcore#856 feat(dash-spv): force resync to re-anchor a running client at a lower checkpoint](https://github.com/dashpay/rust-dashcore/pull/856) — 1 unresolved (1 human) · 96 days stale · ⚠ merge conflict · 🔴 CI failing · areas: dash-spv · Policy: waiting-self-review
  - Top thread: "why do we have a periodic check here??" — 96 days old
  - Blocker: Author must post /self-reviewed 7282172a3bb4a0d9d85a40cf70b4dd111995f96b

<a id="xdustinface-unresolved-comments"></a>
#### Unresolved Comments (1)
- [dashpay/rust-dashcore#856 feat(dash-spv): force resync to re-anchor a running client at a lower checkpoint](https://github.com/dashpay/rust-dashcore/pull/856) — 1 unresolved (1 human) · 96 days stale · ⚠ merge conflict · 🔴 CI failing · areas: dash-spv · Policy: waiting-self-review
  - Top thread: "why do we have a periodic check here??" — 96 days old
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
- [dashpay/rust-dashcore#400 chore: remove BIP141 (SegWit) and BIP341 (Taproot) support](https://github.com/dashpay/rust-dashcore/pull/400) — ⚠ merge conflict · 🔴 CI failing · 📝 draft · 🐢 untouched 158 days · areas: dash-spv, key-wallet, github, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#720 fix(dash-spv): split peer \`TcpStream\` and add per-peer writer task](https://github.com/dashpay/rust-dashcore/pull/720) — ⚠ merge conflict · 🔴 CI failing · 📝 draft · 🐢 untouched 158 days · areas: dash-spv, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot

<a id="xdustinface-clean"></a>
#### Clean (1)
- [dashpay/rust-dashcore#849 feat(dash-spv): add \`--birth-height\` CLI flag](https://github.com/dashpay/rust-dashcore/pull/849) — areas: dash-spv · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 9b1e709fd8e0bfab0dd69f7c86fd6b7d2e585fbe

<a id="xdustinface-ready-for-review"></a>
#### Ready for Review (1)
- [dashpay/rust-dashcore#264 feat(dash-spv): Add BIP324 v2 encrypted P2P transport](https://github.com/dashpay/rust-dashcore/pull/264) — by @PastaPastaPasta · areas: dash-spv, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required

<a id="hashengineering"></a>
### @HashEngineering
<a id="hashengineering-open"></a>
#### Open (4)
- [dashpay/platform#5259 fix(kotlin-sdk): port #4638 to Android — credit verdicts at save time and the post-sync TXO reconcile](https://github.com/dashpay/platform/pull/5259) — 10 unresolved (10 bot) · 15 days stale · ✋ changes requested · 🔴 CI failing · 🐢 targets v5.1-dev · areas: rs-platform-wallet, rs-platform-wallet-ffi, kotlin-sdk, fallback · Policy: waiting-bots
  - Top thread: "🟡 Suggestion: Add an index matching the unspent pagination query**" — 15 days old
  - Blocker: coderabbitai has not reported for the current head
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5352 fix(platform-wallet-storage): open the wallet store on Android](https://github.com/dashpay/platform/pull/5352) — 1 unresolved (1 bot) · 1 days stale · areas: wallet-storage · Policy: waiting-self-review
  - Top thread: "🟡 Suggestion: Test the Android trusted-group constructor directly**" — 1 days old
  - Blocker: Author must post /self-reviewed 917b18c22f00697e6094495658b4d338df7e93a6
- [dashpay/platform#5256 fix(platform-wallet): build our receiving account for one-way DashPay contacts](https://github.com/dashpay/platform/pull/5256) — 🔴 CI failing · 🐢 targets v5.1-dev · areas: rs-platform-wallet · Policy: waiting-build
  - Blocker: The build must pass before a human is asked
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/platform#5479 feat(platform-wallet-storage): persist DashPay backfill coverage and outbound-account markers](https://github.com/dashpay/platform/pull/5479) — 🔴 CI failing · 🐢 targets v5.1-dev · areas: rs-platform-wallet, wallet-storage · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 48783d9e141653ae8896b2c5d3fb92eb5f23a646
  - Blocker: Proceeded without coderabbitai: no review within the configured window

<a id="hashengineering-needs-action"></a>
#### Needs action (1)
- [dashpay/platform#5352 fix(platform-wallet-storage): open the wallet store on Android](https://github.com/dashpay/platform/pull/5352) — 1 unresolved (1 bot) · 1 days stale · areas: wallet-storage · Policy: waiting-self-review
  - Top thread: "🟡 Suggestion: Test the Android trusted-group constructor directly**" — 1 days old
  - Blocker: Author must post /self-reviewed 917b18c22f00697e6094495658b4d338df7e93a6

<a id="hashengineering-unresolved-comments"></a>
#### Unresolved Comments (1)
- [dashpay/platform#5352 fix(platform-wallet-storage): open the wallet store on Android](https://github.com/dashpay/platform/pull/5352) — 1 unresolved (1 bot) · 1 days stale · areas: wallet-storage · Policy: waiting-self-review
  - Top thread: "🟡 Suggestion: Test the Android trusted-group constructor directly**" — 1 days old
  - Blocker: Author must post /self-reviewed 917b18c22f00697e6094495658b4d338df7e93a6

<a id="hashengineering-stale"></a>
#### Stale (3)
- [dashpay/platform#5259 fix(kotlin-sdk): port #4638 to Android — credit verdicts at save time and the post-sync TXO reconcile](https://github.com/dashpay/platform/pull/5259) — 10 unresolved (10 bot) · 15 days stale · ✋ changes requested · 🔴 CI failing · 🐢 targets v5.1-dev · areas: rs-platform-wallet, rs-platform-wallet-ffi, kotlin-sdk, fallback · Policy: waiting-bots
  - Top thread: "🟡 Suggestion: Add an index matching the unspent pagination query**" — 15 days old
  - Blocker: coderabbitai has not reported for the current head
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5256 fix(platform-wallet): build our receiving account for one-way DashPay contacts](https://github.com/dashpay/platform/pull/5256) — 🔴 CI failing · 🐢 targets v5.1-dev · areas: rs-platform-wallet · Policy: waiting-build
  - Blocker: The build must pass before a human is asked
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/platform#5479 feat(platform-wallet-storage): persist DashPay backfill coverage and outbound-account markers](https://github.com/dashpay/platform/pull/5479) — 🔴 CI failing · 🐢 targets v5.1-dev · areas: rs-platform-wallet, wallet-storage · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 48783d9e141653ae8896b2c5d3fb92eb5f23a646
  - Blocker: Proceeded without coderabbitai: no review within the configured window

<a id="zocolini"></a>
### @ZocoLini
<a id="zocolini-open"></a>
#### Open (11)
- [dashpay/rust-dashcore#902 refactor(dash-spv): network module rewrite](https://github.com/dashpay/rust-dashcore/pull/902) — ✋ changes requested · 🔴 CI failing · areas: dash-spv, key-wallet-manager, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 748816fecf69c511e28b7a59c9e6921b42cae936
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/rust-dashcore#921 fix(dash-spv): sync backfill](https://github.com/dashpay/rust-dashcore/pull/921) — ⚠ merge conflict · 📝 draft · areas: dash-spv, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#1081 feat(dash-spv): restore the validated rotation cycles from the replayed engine](https://github.com/dashpay/rust-dashcore/pull/1081) — ⚠ merge conflict · 📝 draft · areas: dash-spv · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#1161 feat(dash-spv)!: validate header difficulty against Dark Gravity Wave](https://github.com/dashpay/rust-dashcore/pull/1161) — ⚠ merge conflict · ✋ changes requested · areas: dash-spv, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 6383467ac779647bcdea51322e862725040321e9
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/rust-dashcore#1178 fix(dash-spv): make stop wait for the tasks the sync loop detaches](https://github.com/dashpay/rust-dashcore/pull/1178) — ✋ changes requested · areas: dash-spv · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 76ed80d252674b5d869472973f27889d8c3c2b09
- [dashpay/rust-dashcore#1181 fix(dash-spv)!: bound the MessageDispatcher subscriber queues](https://github.com/dashpay/rust-dashcore/pull/1181) — 🔴 CI failing · 📝 draft · areas: dash-spv · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#1189 refactor(dash-spv)!: make the sync module private](https://github.com/dashpay/rust-dashcore/pull/1189) — 🔴 CI failing · 📝 draft · areas: dash-spv, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#1199 feat(key-wallet)!: only final coins fund a build](https://github.com/dashpay/rust-dashcore/pull/1199) — ⚠ merge conflict · 📝 draft · areas: dash-spv, key-wallet, key-wallet-manager · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#1200 fix(dash-spv)!: verify MnListDiff against its block's coinbase before applying](https://github.com/dashpay/rust-dashcore/pull/1200) — ⚠ merge conflict · ✋ changes requested · areas: dash-spv, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 04996569818d59b8d7016232dcac23c82e91e93a
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/rust-dashcore#1202 feat(dash-spv): expose the coinbase payload of the newest validated diff](https://github.com/dashpay/rust-dashcore/pull/1202) — ⚠ merge conflict · 🔴 CI failing · 📝 draft · areas: dash-spv, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#1216 ci: catch dead code in clippy, fix the sanitizer path filters](https://github.com/dashpay/rust-dashcore/pull/1216) — 🔴 CI failing · areas: dash-spv, github, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 8b0622acbdf9f6cf9e63a71e1b43c9065346ddf6
  - Blocker: Proceeded without coderabbitai: no review within the configured window

<a id="zocolini-needs-action"></a>
#### Needs action (4)
- [dashpay/rust-dashcore#902 refactor(dash-spv): network module rewrite](https://github.com/dashpay/rust-dashcore/pull/902) — ✋ changes requested · 🔴 CI failing · areas: dash-spv, key-wallet-manager, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 748816fecf69c511e28b7a59c9e6921b42cae936
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/rust-dashcore#1161 feat(dash-spv)!: validate header difficulty against Dark Gravity Wave](https://github.com/dashpay/rust-dashcore/pull/1161) — ⚠ merge conflict · ✋ changes requested · areas: dash-spv, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 6383467ac779647bcdea51322e862725040321e9
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/rust-dashcore#1178 fix(dash-spv): make stop wait for the tasks the sync loop detaches](https://github.com/dashpay/rust-dashcore/pull/1178) — ✋ changes requested · areas: dash-spv · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 76ed80d252674b5d869472973f27889d8c3c2b09
- [dashpay/rust-dashcore#1200 fix(dash-spv)!: verify MnListDiff against its block's coinbase before applying](https://github.com/dashpay/rust-dashcore/pull/1200) — ⚠ merge conflict · ✋ changes requested · areas: dash-spv, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 04996569818d59b8d7016232dcac23c82e91e93a
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return

<a id="zocolini-changes-requested"></a>
#### Changes Requested (4)
- [dashpay/rust-dashcore#902 refactor(dash-spv): network module rewrite](https://github.com/dashpay/rust-dashcore/pull/902) — ✋ changes requested · 🔴 CI failing · areas: dash-spv, key-wallet-manager, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 748816fecf69c511e28b7a59c9e6921b42cae936
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/rust-dashcore#1161 feat(dash-spv)!: validate header difficulty against Dark Gravity Wave](https://github.com/dashpay/rust-dashcore/pull/1161) — ⚠ merge conflict · ✋ changes requested · areas: dash-spv, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 6383467ac779647bcdea51322e862725040321e9
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/rust-dashcore#1178 fix(dash-spv): make stop wait for the tasks the sync loop detaches](https://github.com/dashpay/rust-dashcore/pull/1178) — ✋ changes requested · areas: dash-spv · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 76ed80d252674b5d869472973f27889d8c3c2b09
- [dashpay/rust-dashcore#1200 fix(dash-spv)!: verify MnListDiff against its block's coinbase before applying](https://github.com/dashpay/rust-dashcore/pull/1200) — ⚠ merge conflict · ✋ changes requested · areas: dash-spv, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 04996569818d59b8d7016232dcac23c82e91e93a
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return

<a id="zocolini-ci-failing"></a>
#### CI Failing (1)
- [dashpay/rust-dashcore#1216 ci: catch dead code in clippy, fix the sanitizer path filters](https://github.com/dashpay/rust-dashcore/pull/1216) — 🔴 CI failing · areas: dash-spv, github, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 8b0622acbdf9f6cf9e63a71e1b43c9065346ddf6
  - Blocker: Proceeded without coderabbitai: no review within the configured window

<a id="zocolini-draft"></a>
#### Draft (6)
- [dashpay/rust-dashcore#921 fix(dash-spv): sync backfill](https://github.com/dashpay/rust-dashcore/pull/921) — ⚠ merge conflict · 📝 draft · areas: dash-spv, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#1081 feat(dash-spv): restore the validated rotation cycles from the replayed engine](https://github.com/dashpay/rust-dashcore/pull/1081) — ⚠ merge conflict · 📝 draft · areas: dash-spv · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#1181 fix(dash-spv)!: bound the MessageDispatcher subscriber queues](https://github.com/dashpay/rust-dashcore/pull/1181) — 🔴 CI failing · 📝 draft · areas: dash-spv · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#1189 refactor(dash-spv)!: make the sync module private](https://github.com/dashpay/rust-dashcore/pull/1189) — 🔴 CI failing · 📝 draft · areas: dash-spv, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#1199 feat(key-wallet)!: only final coins fund a build](https://github.com/dashpay/rust-dashcore/pull/1199) — ⚠ merge conflict · 📝 draft · areas: dash-spv, key-wallet, key-wallet-manager · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#1202 feat(dash-spv): expose the coinbase payload of the newest validated diff](https://github.com/dashpay/rust-dashcore/pull/1202) — ⚠ merge conflict · 🔴 CI failing · 📝 draft · areas: dash-spv, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot

<a id="zocolini-ready-for-review"></a>
#### Ready for Review (1)
- [dashpay/rust-dashcore#264 feat(dash-spv): Add BIP324 v2 encrypted P2P transport](https://github.com/dashpay/rust-dashcore/pull/264) — by @PastaPastaPasta · areas: dash-spv, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required

<a id="pastapastapasta"></a>
### @PastaPastaPasta
<a id="pastapastapasta-open"></a>
#### Open (20)
- [dashpay/platform#4648 feat(drive-abci)!: state sync via ABCI snapshots with reduced platform state (protocol v15)](https://github.com/dashpay/platform/pull/4648) — 8 unresolved (8 bot) · 20 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · 📝 draft · 🐢 targets v5.1-dev · areas: rs-drive, rs-drive-abci, dpp, fallback · Policy: draft
  - Top thread: "🟡 Suggestion: Continue the restored target through consensus and a restart**" — 20 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#137 fix: integration tests, continued dashificiation](https://github.com/dashpay/rust-dashcore/pull/137) — 2 unresolved (2 CodeRabbit) · 415 days stale · ⚠ merge conflict · 🔴 CI failing · 🐢 targets v0.40-dev, untouched 318 days · areas: fallback
  - Top thread: "_🛠️ Refactor suggestion_" — 415 days old
- [dashpay/platform#5288 feat(platform-wallet)!: support DashPay shielded tips with dedicated accounts](https://github.com/dashpay/platform/pull/5288) — 5 unresolved (5 bot) · 20 days stale · ⚠ merge conflict · 🐢 targets v5.1-dev · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk, kotlin-sdk, fallback · Policy: waiting-self-review
  - Top thread: "🟡 Suggestion: Restore outstanding tip submissions before enabling another send**" — 20 days old
  - Blocker: Author must post /self-reviewed 17b627fa2172995dd9117d88f4484b0d02dc7316
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/platform#5521 feat(sdk): verify Core quorum proof chains (DIP 175)](https://github.com/dashpay/platform/pull/5521) — 5 unresolved (5 bot) · 4 days stale · 🐢 targets v5.1-dev · areas: github, fallback · Policy: ready-for-human
  - Top thread: "✅ **Resolved** at \`f1c413e9\`; see the replies below." — 4 days old
  - Blocker: Human approval or objection resolution is required
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/platform#5523 feat(sdk): add opt-in verified HTTP context provider](https://github.com/dashpay/platform/pull/5523) — 5 unresolved (5 bot) · 4 days stale · ✋ changes requested · 🐢 targets feat/core-snapshot-proof-verifier · areas: github, fallback
  - Top thread: "🔴 Blocking: Break the strong-reference cycle in running proof fetches**" — 4 days old
- [dashpay/platform#4965 test(sdk): cargo-fuzz harness over drive-proof-verifier FromProof](https://github.com/dashpay/platform/pull/4965) — 1 unresolved (1 bot) · 19 days stale · 📝 draft · 🐢 targets v5.1-dev · areas: github, fallback · Policy: draft
  - Top thread: "🟡 Suggestion: Correct the claim that PV13 reaches the V0 proof decoder**" — 19 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5280 perf(platform-wallet)!: reuse a recent recorded-anchor set on the shielded spend path](https://github.com/dashpay/platform/pull/5280) — 1 unresolved (1 bot) · 10 days stale · 🐢 targets v5.1-dev · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk · Policy: waiting-self-review
  - Top thread: "🟡 Suggestion: Guard the background-prefetch export against nested Tokio entry**" — 10 days old
  - Blocker: Author must post /self-reviewed f0ef5a65d32dac0c196a572a66a94612eec7834d
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/platform#5277 perf(platform-wallet): don't hold the shielded store lock across sync network I/O](https://github.com/dashpay/platform/pull/5277) — 1 unresolved (1 human) · 9 days stale · ⚠ merge conflict · 🐢 targets v5.1-dev · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk · Policy: waiting-author
  - Top thread: "Major, not blocking.** A wallet-scoped bump aborts the whole multi-wallet pass. Commit tree leaves are wallet-agnostic, …" — 9 days old
  - Blocker: Author response is required after the latest human objection
- [dashpay/grovedb#1007 build(deps)!: use dashpay/orchard with the faster prover](https://github.com/dashpay/grovedb/pull/1007) — 🔴 CI failing · 📝 draft · areas: fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#4633 feat(sdk): add dash-sdk-cxx, a C++ binding of dash-sdk for Dash Core](https://github.com/dashpay/platform/pull/4633) — 📝 draft · 🐢 targets build/vendorable-workspace-lockfile · areas: github, fallback
- [dashpay/platform#5282 perf(platform-wallet)!: prove the shield bundle while waiting for the InstantSend lock](https://github.com/dashpay/platform/pull/5282) — ⚠ merge conflict · 🐢 targets v5.1-dev · areas: dpp, rs-platform-wallet, rs-platform-wallet-ffi, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 6ca6ab3e2b64e7d5ca4c44c1395d15b6ff8165f2
- [dashpay/platform#5285 feat(platform)!: subscribe to committed state transitions matching document and address filters](https://github.com/dashpay/platform/pull/5285) — ⚠ merge conflict · 📝 draft · 🐢 targets v5.1-dev · areas: rs-drive, rs-drive-abci, rust-sdk, dashmate, rust-dapi, github, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5289 feat(sdk)!: optional BIP-39 passphrase through the mnemonic resolver and wallet creation](https://github.com/dashpay/platform/pull/5289) — 📝 draft · 🐢 targets v5.1-dev · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, rust-sdk-ffi, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5292 feat(dashmate): state sync configuration for tenderdash and drive snapshots](https://github.com/dashpay/platform/pull/5292) — ⚠ merge conflict · 📝 draft · 🐢 targets v5.1-dev · areas: dashmate · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5473 build: make the workspace lockfile vendorable again](https://github.com/dashpay/platform/pull/5473) — ⚠ merge conflict · 📝 draft · 🐢 targets v5.1-dev · areas: fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5475 feat(sdk): forward DashPay profiles, invitations and masternode voting in dash-sdk-cxx](https://github.com/dashpay/platform/pull/5475) — ⚠ merge conflict · 📝 draft · 🐢 targets v5.1-dev · areas: rs-platform-wallet, rs-platform-wallet-ffi, rust-sdk, github, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5569 fix(sdk): send wasm DAPI requests to a single-slash path](https://github.com/dashpay/platform/pull/5569) — 🐢 targets v5.1-dev · areas: js-wasm-sdk, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 2fcc63391dc11fd465992ea93a2f497eeca8e626
- [dashpay/platform#5577 feat: gzip-compress DAPI responses](https://github.com/dashpay/platform/pull/5577) — 🔴 CI failing · 🐢 targets v5.1-dev · areas: dashmate, rust-dapi, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 57fa2074886289b9128231a850a9e8b31542247a
- [dashpay/rust-dashcore#264 feat(dash-spv): Add BIP324 v2 encrypted P2P transport](https://github.com/dashpay/rust-dashcore/pull/264) — areas: dash-spv, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
- [dashpay/rust-dashcore#959 feat(dash-spv): adaptive gap-limit probe escalation for BIP44 discovery](https://github.com/dashpay/rust-dashcore/pull/959) — ⚠ merge conflict · 🔴 CI failing · areas: dash-spv, key-wallet, key-wallet-manager, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 7bcac9b9d89c2c4947080b2e6a15503e97eac9f4

<a id="pastapastapasta-needs-action"></a>
#### Needs action (1)
- [dashpay/rust-dashcore#959 feat(dash-spv): adaptive gap-limit probe escalation for BIP44 discovery](https://github.com/dashpay/rust-dashcore/pull/959) — ⚠ merge conflict · 🔴 CI failing · areas: dash-spv, key-wallet, key-wallet-manager, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 7bcac9b9d89c2c4947080b2e6a15503e97eac9f4

<a id="pastapastapasta-ready-for-human"></a>
#### Ready for human (2)
- [dashpay/platform#5521 feat(sdk): verify Core quorum proof chains (DIP 175)](https://github.com/dashpay/platform/pull/5521) — 5 unresolved (5 bot) · 4 days stale · 🐢 targets v5.1-dev · areas: github, fallback · Policy: ready-for-human
  - Top thread: "✅ **Resolved** at \`f1c413e9\`; see the replies below." — 4 days old
  - Blocker: Human approval or objection resolution is required
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/rust-dashcore#264 feat(dash-spv): Add BIP324 v2 encrypted P2P transport](https://github.com/dashpay/rust-dashcore/pull/264) — areas: dash-spv, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required

<a id="pastapastapasta-ci-failing"></a>
#### CI Failing (1)
- [dashpay/rust-dashcore#959 feat(dash-spv): adaptive gap-limit probe escalation for BIP44 discovery](https://github.com/dashpay/rust-dashcore/pull/959) — ⚠ merge conflict · 🔴 CI failing · areas: dash-spv, key-wallet, key-wallet-manager, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 7bcac9b9d89c2c4947080b2e6a15503e97eac9f4

<a id="pastapastapasta-draft"></a>
#### Draft (1)
- [dashpay/grovedb#1007 build(deps)!: use dashpay/orchard with the faster prover](https://github.com/dashpay/grovedb/pull/1007) — 🔴 CI failing · 📝 draft · areas: fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot

<a id="pastapastapasta-stale"></a>
#### Stale (17)
- [dashpay/platform#4648 feat(drive-abci)!: state sync via ABCI snapshots with reduced platform state (protocol v15)](https://github.com/dashpay/platform/pull/4648) — 8 unresolved (8 bot) · 20 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · 📝 draft · 🐢 targets v5.1-dev · areas: rs-drive, rs-drive-abci, dpp, fallback · Policy: draft
  - Top thread: "🟡 Suggestion: Continue the restored target through consensus and a restart**" — 20 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#137 fix: integration tests, continued dashificiation](https://github.com/dashpay/rust-dashcore/pull/137) — 2 unresolved (2 CodeRabbit) · 415 days stale · ⚠ merge conflict · 🔴 CI failing · 🐢 targets v0.40-dev, untouched 318 days · areas: fallback
  - Top thread: "_🛠️ Refactor suggestion_" — 415 days old
- [dashpay/platform#5288 feat(platform-wallet)!: support DashPay shielded tips with dedicated accounts](https://github.com/dashpay/platform/pull/5288) — 5 unresolved (5 bot) · 20 days stale · ⚠ merge conflict · 🐢 targets v5.1-dev · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk, kotlin-sdk, fallback · Policy: waiting-self-review
  - Top thread: "🟡 Suggestion: Restore outstanding tip submissions before enabling another send**" — 20 days old
  - Blocker: Author must post /self-reviewed 17b627fa2172995dd9117d88f4484b0d02dc7316
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/platform#5521 feat(sdk): verify Core quorum proof chains (DIP 175)](https://github.com/dashpay/platform/pull/5521) — 5 unresolved (5 bot) · 4 days stale · 🐢 targets v5.1-dev · areas: github, fallback · Policy: ready-for-human
  - Top thread: "✅ **Resolved** at \`f1c413e9\`; see the replies below." — 4 days old
  - Blocker: Human approval or objection resolution is required
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/platform#5523 feat(sdk): add opt-in verified HTTP context provider](https://github.com/dashpay/platform/pull/5523) — 5 unresolved (5 bot) · 4 days stale · ✋ changes requested · 🐢 targets feat/core-snapshot-proof-verifier · areas: github, fallback
  - Top thread: "🔴 Blocking: Break the strong-reference cycle in running proof fetches**" — 4 days old
- [dashpay/platform#4965 test(sdk): cargo-fuzz harness over drive-proof-verifier FromProof](https://github.com/dashpay/platform/pull/4965) — 1 unresolved (1 bot) · 19 days stale · 📝 draft · 🐢 targets v5.1-dev · areas: github, fallback · Policy: draft
  - Top thread: "🟡 Suggestion: Correct the claim that PV13 reaches the V0 proof decoder**" — 19 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5280 perf(platform-wallet)!: reuse a recent recorded-anchor set on the shielded spend path](https://github.com/dashpay/platform/pull/5280) — 1 unresolved (1 bot) · 10 days stale · 🐢 targets v5.1-dev · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk · Policy: waiting-self-review
  - Top thread: "🟡 Suggestion: Guard the background-prefetch export against nested Tokio entry**" — 10 days old
  - Blocker: Author must post /self-reviewed f0ef5a65d32dac0c196a572a66a94612eec7834d
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/platform#5277 perf(platform-wallet): don't hold the shielded store lock across sync network I/O](https://github.com/dashpay/platform/pull/5277) — 1 unresolved (1 human) · 9 days stale · ⚠ merge conflict · 🐢 targets v5.1-dev · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk · Policy: waiting-author
  - Top thread: "Major, not blocking.** A wallet-scoped bump aborts the whole multi-wallet pass. Commit tree leaves are wallet-agnostic, …" — 9 days old
  - Blocker: Author response is required after the latest human objection
- [dashpay/platform#4633 feat(sdk): add dash-sdk-cxx, a C++ binding of dash-sdk for Dash Core](https://github.com/dashpay/platform/pull/4633) — 📝 draft · 🐢 targets build/vendorable-workspace-lockfile · areas: github, fallback
- [dashpay/platform#5282 perf(platform-wallet)!: prove the shield bundle while waiting for the InstantSend lock](https://github.com/dashpay/platform/pull/5282) — ⚠ merge conflict · 🐢 targets v5.1-dev · areas: dpp, rs-platform-wallet, rs-platform-wallet-ffi, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 6ca6ab3e2b64e7d5ca4c44c1395d15b6ff8165f2
- [dashpay/platform#5285 feat(platform)!: subscribe to committed state transitions matching document and address filters](https://github.com/dashpay/platform/pull/5285) — ⚠ merge conflict · 📝 draft · 🐢 targets v5.1-dev · areas: rs-drive, rs-drive-abci, rust-sdk, dashmate, rust-dapi, github, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5289 feat(sdk)!: optional BIP-39 passphrase through the mnemonic resolver and wallet creation](https://github.com/dashpay/platform/pull/5289) — 📝 draft · 🐢 targets v5.1-dev · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, rust-sdk-ffi, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5292 feat(dashmate): state sync configuration for tenderdash and drive snapshots](https://github.com/dashpay/platform/pull/5292) — ⚠ merge conflict · 📝 draft · 🐢 targets v5.1-dev · areas: dashmate · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5473 build: make the workspace lockfile vendorable again](https://github.com/dashpay/platform/pull/5473) — ⚠ merge conflict · 📝 draft · 🐢 targets v5.1-dev · areas: fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5475 feat(sdk): forward DashPay profiles, invitations and masternode voting in dash-sdk-cxx](https://github.com/dashpay/platform/pull/5475) — ⚠ merge conflict · 📝 draft · 🐢 targets v5.1-dev · areas: rs-platform-wallet, rs-platform-wallet-ffi, rust-sdk, github, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5569 fix(sdk): send wasm DAPI requests to a single-slash path](https://github.com/dashpay/platform/pull/5569) — 🐢 targets v5.1-dev · areas: js-wasm-sdk, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 2fcc63391dc11fd465992ea93a2f497eeca8e626
- [dashpay/platform#5577 feat: gzip-compress DAPI responses](https://github.com/dashpay/platform/pull/5577) — 🔴 CI failing · 🐢 targets v5.1-dev · areas: dashmate, rust-dapi, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 57fa2074886289b9128231a850a9e8b31542247a

<a id="pastapastapasta-clean"></a>
#### Clean (1)
- [dashpay/rust-dashcore#264 feat(dash-spv): Add BIP324 v2 encrypted P2P transport](https://github.com/dashpay/rust-dashcore/pull/264) — areas: dash-spv, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required

<a id="romchornyi"></a>
### @romchornyi
<a id="romchornyi-open"></a>
#### Open (12)
- [dashpay/platform#5465 feat(platform-wallet): sign a DashPay contact payment before broadcasting it](https://github.com/dashpay/platform/pull/5465) — 6 unresolved (6 bot) · 9 days stale · 🐢 targets v5.1-dev · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk · Policy: waiting-author
  - Top thread: "✅ **Resolved** at \`55c237ef\`; see the replies below." — 9 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/platform#5258 feat(platform-wallet): probe the network about sends with an unknown broadcast outcome](https://github.com/dashpay/platform/pull/5258) — 3 unresolved (3 human) · 9 days stale · ⚠ merge conflict · 📝 draft · 🐢 targets v5.1-dev · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, rust-sdk · Policy: draft
  - Top thread: "\[P2\] Carry the originating wallet generation in probe events" — 9 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5395 feat(platform-wallet): spend only final coins and report funds waiting on the network](https://github.com/dashpay/platform/pull/5395) — 1 unresolved (1 human) · 9 days stale · ⚠ merge conflict · ✋ changes requested · 📝 draft · 🐢 targets v5.1-dev · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk, fallback · Policy: draft
  - Top thread: "this should belong to another layer - likely platform-wallet, or even rust-dashcore's key-wallet. FFI is not meant to st…" — 9 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5487 feat(platform-wallet)!: update an identity at a revision the caller names](https://github.com/dashpay/platform/pull/5487) — 3 unresolved (3 bot) · 2 days stale · 🔴 CI failing · 🐢 targets v5.1-dev · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk, kotlin-sdk, fallback · Policy: waiting-bots
  - Top thread: "🔴 Blocking: Persist the refusal revision used by next_update_revision across restarts**" — 2 days old
  - Blocker: coderabbitai has not reported for the current head
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5460 fix(platform-wallet): retry an oversized InstantSend asset-lock transition with a ChainLock proof](https://github.com/dashpay/platform/pull/5460) — 2 unresolved (2 bot) · 1 days stale · ✋ changes requested · 🔴 CI failing · 🐢 targets v5.1-dev · areas: rs-drive-abci, dpp, rs-platform-wallet, rs-platform-wallet-ffi, rust-sdk, rust-dapi · Policy: waiting-bots
  - Top thread: "✅ **Withdrawn** at \`1831b7d9\`; see the replies below." — 1 days old
  - Blocker: coderabbitai has not reported for the current head
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5536 fix: check a DashPay invitation claim against Platform before broadcasting it](https://github.com/dashpay/platform/pull/5536) — 2 unresolved (2 bot) · 1 days stale · ✋ changes requested · 🔴 CI failing · 🐢 targets v5.1-dev · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, rust-sdk, kotlin-sdk, fallback · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Handle code 63 when the collided key belongs to this wallet**" — 1 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/platform#5549 fix(platform-wallet-storage): restore DashPay contact address pools on SQLite load](https://github.com/dashpay/platform/pull/5549) — 2 unresolved (2 bot) · 2 days stale · 🔴 CI failing · 🐢 targets v5.1-dev · areas: wallet-storage · Policy: waiting-bots
  - Top thread: "✅ **Withdrawn** at \`2bee2705\`; see the replies below." — 2 days old
  - Blocker: coderabbitai has not reported for the current head
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
- [dashpay/platform#5540 fix(platform-wallet)!: store externally supplied identity keys without a derivation breadcrumb](https://github.com/dashpay/platform/pull/5540) — 1 unresolved (1 bot) · 3 days stale · 🐢 targets v5.1-dev · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk, fallback · Policy: waiting-author
  - Top thread: "🔴 Blocking: Preserve existing derived-key breadcrumbs on disable upserts**" — 3 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/platform#4992 feat(swift-sdk): durable wallet presence marker readable while the device is locked](https://github.com/dashpay/platform/pull/4992) — 🐢 targets v5.1-dev · areas: swift-sdk · Policy: ready-to-merge
  - Blocker: All policy requirements are satisfied
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/platform#5309 fix(sdk): treat AlreadyExists on a retried broadcast as submitted](https://github.com/dashpay/platform/pull/5309) — 📝 draft · 🐢 targets v5.1-dev · ⚠ routing unavailable: no changed-file evidence · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5476 feat(platform-wallet)!: report a freshly built asset lock's outpoint to the caller](https://github.com/dashpay/platform/pull/5476) — 🐢 targets v5.1-dev · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, fallback · Policy: ready-to-merge
  - Blocker: All policy requirements are satisfied
  - Blocker: Proceeded without coderabbitai: skipped by @romchornyi
- [dashpay/rust-dashcore#1105 feat(key-wallet): extend CoinJoin address pools before the first sync](https://github.com/dashpay/rust-dashcore/pull/1105) — ✋ changes requested · areas: key-wallet · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required

<a id="romchornyi-needs-action"></a>
#### Needs action (1)
- [dashpay/rust-dashcore#1105 feat(key-wallet): extend CoinJoin address pools before the first sync](https://github.com/dashpay/rust-dashcore/pull/1105) — ✋ changes requested · areas: key-wallet · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required

<a id="romchornyi-ready-for-human"></a>
#### Ready for human (3)
- [dashpay/platform#4992 feat(swift-sdk): durable wallet presence marker readable while the device is locked](https://github.com/dashpay/platform/pull/4992) — 🐢 targets v5.1-dev · areas: swift-sdk · Policy: ready-to-merge
  - Blocker: All policy requirements are satisfied
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/platform#5476 feat(platform-wallet)!: report a freshly built asset lock's outpoint to the caller](https://github.com/dashpay/platform/pull/5476) — 🐢 targets v5.1-dev · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, fallback · Policy: ready-to-merge
  - Blocker: All policy requirements are satisfied
  - Blocker: Proceeded without coderabbitai: skipped by @romchornyi
- [dashpay/rust-dashcore#1105 feat(key-wallet): extend CoinJoin address pools before the first sync](https://github.com/dashpay/rust-dashcore/pull/1105) — ✋ changes requested · areas: key-wallet · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required

<a id="romchornyi-changes-requested"></a>
#### Changes Requested (1)
- [dashpay/rust-dashcore#1105 feat(key-wallet): extend CoinJoin address pools before the first sync](https://github.com/dashpay/rust-dashcore/pull/1105) — ✋ changes requested · areas: key-wallet · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required

<a id="romchornyi-stale"></a>
#### Stale (11)
- [dashpay/platform#5465 feat(platform-wallet): sign a DashPay contact payment before broadcasting it](https://github.com/dashpay/platform/pull/5465) — 6 unresolved (6 bot) · 9 days stale · 🐢 targets v5.1-dev · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk · Policy: waiting-author
  - Top thread: "✅ **Resolved** at \`55c237ef\`; see the replies below." — 9 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/platform#5258 feat(platform-wallet): probe the network about sends with an unknown broadcast outcome](https://github.com/dashpay/platform/pull/5258) — 3 unresolved (3 human) · 9 days stale · ⚠ merge conflict · 📝 draft · 🐢 targets v5.1-dev · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, rust-sdk · Policy: draft
  - Top thread: "\[P2\] Carry the originating wallet generation in probe events" — 9 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5395 feat(platform-wallet): spend only final coins and report funds waiting on the network](https://github.com/dashpay/platform/pull/5395) — 1 unresolved (1 human) · 9 days stale · ⚠ merge conflict · ✋ changes requested · 📝 draft · 🐢 targets v5.1-dev · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk, fallback · Policy: draft
  - Top thread: "this should belong to another layer - likely platform-wallet, or even rust-dashcore's key-wallet. FFI is not meant to st…" — 9 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5487 feat(platform-wallet)!: update an identity at a revision the caller names](https://github.com/dashpay/platform/pull/5487) — 3 unresolved (3 bot) · 2 days stale · 🔴 CI failing · 🐢 targets v5.1-dev · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk, kotlin-sdk, fallback · Policy: waiting-bots
  - Top thread: "🔴 Blocking: Persist the refusal revision used by next_update_revision across restarts**" — 2 days old
  - Blocker: coderabbitai has not reported for the current head
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5460 fix(platform-wallet): retry an oversized InstantSend asset-lock transition with a ChainLock proof](https://github.com/dashpay/platform/pull/5460) — 2 unresolved (2 bot) · 1 days stale · ✋ changes requested · 🔴 CI failing · 🐢 targets v5.1-dev · areas: rs-drive-abci, dpp, rs-platform-wallet, rs-platform-wallet-ffi, rust-sdk, rust-dapi · Policy: waiting-bots
  - Top thread: "✅ **Withdrawn** at \`1831b7d9\`; see the replies below." — 1 days old
  - Blocker: coderabbitai has not reported for the current head
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5536 fix: check a DashPay invitation claim against Platform before broadcasting it](https://github.com/dashpay/platform/pull/5536) — 2 unresolved (2 bot) · 1 days stale · ✋ changes requested · 🔴 CI failing · 🐢 targets v5.1-dev · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, rust-sdk, kotlin-sdk, fallback · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Handle code 63 when the collided key belongs to this wallet**" — 1 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/platform#5549 fix(platform-wallet-storage): restore DashPay contact address pools on SQLite load](https://github.com/dashpay/platform/pull/5549) — 2 unresolved (2 bot) · 2 days stale · 🔴 CI failing · 🐢 targets v5.1-dev · areas: wallet-storage · Policy: waiting-bots
  - Top thread: "✅ **Withdrawn** at \`2bee2705\`; see the replies below." — 2 days old
  - Blocker: coderabbitai has not reported for the current head
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
- [dashpay/platform#5540 fix(platform-wallet)!: store externally supplied identity keys without a derivation breadcrumb](https://github.com/dashpay/platform/pull/5540) — 1 unresolved (1 bot) · 3 days stale · 🐢 targets v5.1-dev · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk, fallback · Policy: waiting-author
  - Top thread: "🔴 Blocking: Preserve existing derived-key breadcrumbs on disable upserts**" — 3 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/platform#4992 feat(swift-sdk): durable wallet presence marker readable while the device is locked](https://github.com/dashpay/platform/pull/4992) — 🐢 targets v5.1-dev · areas: swift-sdk · Policy: ready-to-merge
  - Blocker: All policy requirements are satisfied
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/platform#5309 fix(sdk): treat AlreadyExists on a retried broadcast as submitted](https://github.com/dashpay/platform/pull/5309) — 📝 draft · 🐢 targets v5.1-dev · ⚠ routing unavailable: no changed-file evidence · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5476 feat(platform-wallet)!: report a freshly built asset lock's outpoint to the caller](https://github.com/dashpay/platform/pull/5476) — 🐢 targets v5.1-dev · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, fallback · Policy: ready-to-merge
  - Blocker: All policy requirements are satisfied
  - Blocker: Proceeded without coderabbitai: skipped by @romchornyi

<a id="ktechmidas"></a>
### @ktechmidas
<a id="ktechmidas-open"></a>
#### Open (6)
- [dashpay/platform#5188 ci: tolerate optional S3 cache export outages on chore/bump-rust-dashcore-secp-033](https://github.com/dashpay/platform/pull/5188) — 1 unresolved (1 bot) · 26 days stale · 🐢 targets chore/bump-rust-dashcore-secp-033 · areas: github
  - Top thread: "🟡 Suggestion: New regression tests are not executed by any CI workflow on this branch**" — 26 days old
- [dashpay/platform#5186 ci: tolerate optional S3 cache export outages on v5.0-dev](https://github.com/dashpay/platform/pull/5186) — 🔴 CI failing · 🐢 targets v6.0-dev · areas: github · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 843052c2dce26a80065c15b1219bbc8211264161
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/platform#5187 ci: tolerate optional S3 cache export outages on v4.3-dev](https://github.com/dashpay/platform/pull/5187) — 🐢 targets v5.1-dev · areas: github · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed fbbf13ee41f8bd8b9f03f1dc5afec6a619967d97
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/platform#5198 ci: bound default Binaryen optimizer threads in NPM builds](https://github.com/dashpay/platform/pull/5198) — 🐢 targets v6.0-dev · areas: github · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 23b41b477bc038965b686b4a75412b1c21fc6d98
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/platform#5204 fix(release): install verified cargo-binstall binary directly](https://github.com/dashpay/platform/pull/5204) — ⚠ merge conflict · 🐢 targets v5.1-dev · areas: fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 63455a2563fd7271fb11083d3b0a89bdf01f3982
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/platform#5205 fix(release): install verified cargo-binstall binary directly](https://github.com/dashpay/platform/pull/5205) — 🐢 targets v6.0-dev · areas: fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 8a8ac2c788b5338416e18045001d9115ba3ac80a
  - Blocker: Proceeded without coderabbitai: no review within the configured window

<a id="ktechmidas-stale"></a>
#### Stale (6)
- [dashpay/platform#5188 ci: tolerate optional S3 cache export outages on chore/bump-rust-dashcore-secp-033](https://github.com/dashpay/platform/pull/5188) — 1 unresolved (1 bot) · 26 days stale · 🐢 targets chore/bump-rust-dashcore-secp-033 · areas: github
  - Top thread: "🟡 Suggestion: New regression tests are not executed by any CI workflow on this branch**" — 26 days old
- [dashpay/platform#5186 ci: tolerate optional S3 cache export outages on v5.0-dev](https://github.com/dashpay/platform/pull/5186) — 🔴 CI failing · 🐢 targets v6.0-dev · areas: github · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 843052c2dce26a80065c15b1219bbc8211264161
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/platform#5187 ci: tolerate optional S3 cache export outages on v4.3-dev](https://github.com/dashpay/platform/pull/5187) — 🐢 targets v5.1-dev · areas: github · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed fbbf13ee41f8bd8b9f03f1dc5afec6a619967d97
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/platform#5198 ci: bound default Binaryen optimizer threads in NPM builds](https://github.com/dashpay/platform/pull/5198) — 🐢 targets v6.0-dev · areas: github · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 23b41b477bc038965b686b4a75412b1c21fc6d98
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/platform#5204 fix(release): install verified cargo-binstall binary directly](https://github.com/dashpay/platform/pull/5204) — ⚠ merge conflict · 🐢 targets v5.1-dev · areas: fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 63455a2563fd7271fb11083d3b0a89bdf01f3982
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/platform#5205 fix(release): install verified cargo-binstall binary directly](https://github.com/dashpay/platform/pull/5205) — 🐢 targets v6.0-dev · areas: fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 8a8ac2c788b5338416e18045001d9115ba3ac80a
  - Blocker: Proceeded without coderabbitai: no review within the configured window

<a id="ktechmidas-ready-for-review"></a>
#### Ready for Review (1)
- [dashpay/tenderdash#1560 test(e2e): exercise block sync during slow local application](https://github.com/dashpay/tenderdash/pull/1560) — by @lklimek · areas: github, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 1f5fb2540ca89b3c6b2bb311ad149affdd577df4

<a id="infraclaw-dash"></a>
### @infraclaw-dash
<a id="infraclaw-dash-open"></a>
#### Open (3)
- [dashpay/platform#5167 ci: align secp feature-base workflows with provisioned runner images](https://github.com/dashpay/platform/pull/5167) — 3 unresolved (3 bot) · 26 days stale · ⚠ merge conflict · 🐢 targets chore/bump-rust-dashcore-secp-033 · areas: github
  - Top thread: "🟡 Suggestion: Contract-failure error points to a doc file missing from this branch**" — 26 days old
- [dashpay/platform#3958 ci: add Platform testnet sync status reporting](https://github.com/dashpay/platform/pull/3958) — 2 unresolved (2 bot) · 123 days stale · ⚠ merge conflict · 🐢 targets v5.1-dev · areas: github, fallback · Policy: waiting-author
  - Top thread: "🟡 Suggestion: workflow_dispatch lets any repo writer forge a sync status**" — 123 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: coderabbitai left review threads unresolved; resolve them
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5166 ci: adopt verified Rust and Kotlin runner images on v4.3](https://github.com/dashpay/platform/pull/5166) — ⚠ merge conflict · 🐢 targets v5.1-dev · areas: github · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required

<a id="infraclaw-dash-ready-for-human"></a>
#### Ready for human (1)
- [dashpay/platform#5166 ci: adopt verified Rust and Kotlin runner images on v4.3](https://github.com/dashpay/platform/pull/5166) — ⚠ merge conflict · 🐢 targets v5.1-dev · areas: github · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required

<a id="infraclaw-dash-stale"></a>
#### Stale (3)
- [dashpay/platform#5167 ci: align secp feature-base workflows with provisioned runner images](https://github.com/dashpay/platform/pull/5167) — 3 unresolved (3 bot) · 26 days stale · ⚠ merge conflict · 🐢 targets chore/bump-rust-dashcore-secp-033 · areas: github
  - Top thread: "🟡 Suggestion: Contract-failure error points to a doc file missing from this branch**" — 26 days old
- [dashpay/platform#3958 ci: add Platform testnet sync status reporting](https://github.com/dashpay/platform/pull/3958) — 2 unresolved (2 bot) · 123 days stale · ⚠ merge conflict · 🐢 targets v5.1-dev · areas: github, fallback · Policy: waiting-author
  - Top thread: "🟡 Suggestion: workflow_dispatch lets any repo writer forge a sync status**" — 123 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: coderabbitai left review threads unresolved; resolve them
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5166 ci: adopt verified Rust and Kotlin runner images on v4.3](https://github.com/dashpay/platform/pull/5166) — ⚠ merge conflict · 🐢 targets v5.1-dev · areas: github · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required

<a id="bfoss765"></a>
### @bfoss765
<a id="bfoss765-open"></a>
#### Open (2)
- [dashpay/platform#4313 feat(kotlin-sdk): one-time Orchard key shielded-invite API (inviter + claim)](https://github.com/dashpay/platform/pull/4313) — 2 unresolved (2 bot) · 55 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · 🐢 targets v5.1-dev · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk, fallback · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Preserve claim-binding mismatch semantics across the FFI boundary**" — 55 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/platform#4312 feat(platform-wallet): multi-output shielded transfers + output-aware fee predictor](https://github.com/dashpay/platform/pull/4312) — 1 unresolved (1 bot) · 55 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · 🐢 targets v5.1-dev · areas: dpp, rs-platform-wallet, rs-platform-wallet-ffi, kotlin-sdk, fallback · Policy: waiting-self-review
  - Top thread: "🟡 Suggestion: Keep envelope-aware action counting behind the crate boundary**" — 55 days old
  - Blocker: Author must post /self-reviewed da932f96a8670c641065954de8b58752e99233a3
  - Blocker: Proceeded without coderabbitai: no review within the configured window

<a id="bfoss765-stale"></a>
#### Stale (2)
- [dashpay/platform#4313 feat(kotlin-sdk): one-time Orchard key shielded-invite API (inviter + claim)](https://github.com/dashpay/platform/pull/4313) — 2 unresolved (2 bot) · 55 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · 🐢 targets v5.1-dev · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk, fallback · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Preserve claim-binding mismatch semantics across the FFI boundary**" — 55 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/platform#4312 feat(platform-wallet): multi-output shielded transfers + output-aware fee predictor](https://github.com/dashpay/platform/pull/4312) — 1 unresolved (1 bot) · 55 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · 🐢 targets v5.1-dev · areas: dpp, rs-platform-wallet, rs-platform-wallet-ffi, kotlin-sdk, fallback · Policy: waiting-self-review
  - Top thread: "🟡 Suggestion: Keep envelope-aware action counting behind the crate boundary**" — 55 days old
  - Blocker: Author must post /self-reviewed da932f96a8670c641065954de8b58752e99233a3
  - Blocker: Proceeded without coderabbitai: no review within the configured window

<a id="dcg-claude"></a>
### @DCG-Claude
<a id="dcg-claude-open"></a>
#### Open (19)
- [dashpay/platform#4703 fix(platform): resolve fee versions by registered number and price refunds at the storage epoch rate](https://github.com/dashpay/platform/pull/4703) — 🐢 targets v5.1-dev · areas: rs-drive, rs-drive-abci, dpp, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
- [dashpay/platform#4704 fix(platform): record the genesis fee generation and add replay coverage across a fee-version boundary](https://github.com/dashpay/platform/pull/4704) — 🐢 targets v5.1-dev · areas: rs-drive-abci, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/platform#4705 feat(platform): define protocol-versioned smart-contract computation limits and their gas representation](https://github.com/dashpay/platform/pull/4705) — 🐢 targets v6.0-dev · areas: rs-drive, rs-drive-abci, dpp, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/platform#4706 feat(platform)!: require fee history for storage refunds and credit their recorded owners](https://github.com/dashpay/platform/pull/4706) — 🐢 targets v5.1-dev · areas: rs-drive, rs-drive-abci, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/platform#4707 feat: alloc-only profiles for platform-value and platform-serialization](https://github.com/dashpay/platform/pull/4707) — 🐢 targets v5.1-dev · areas: github, fallback · Policy: ready-for-human
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
- [dashpay/platform#4718 ci: compare cross-architecture replay artifacts across a protocol upgrade and a restart](https://github.com/dashpay/platform/pull/4718) — 🔴 CI failing · 🐢 targets v6.0-dev · areas: rs-drive-abci, github, fallback · Policy: waiting-build
  - Blocker: The build must pass before a human is asked
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/platform#4719 feat: add the dash-sdk-contract declaration model, grammar and diagnostics](https://github.com/dashpay/platform/pull/4719) — 🔴 CI failing · 🐢 targets v5.1-dev · areas: github, fallback · Policy: waiting-build
  - Blocker: The build must pass before a human is asked
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/platform#4784 feat(platform)!: validate contested index parameters and bind the native award to its poll](https://github.com/dashpay/platform/pull/4784) — ⚠ merge conflict · 🔴 CI failing · 🐢 targets v6.0-dev · areas: rs-drive, rs-drive-abci, dpp, fallback · Policy: waiting-build
  - Blocker: The build must pass before a human is asked
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/platform#4789 feat(platform): add typed refund owners to storage flags and fee refunds](https://github.com/dashpay/platform/pull/4789) — 🐢 targets v6.0-dev · areas: rs-drive, dpp, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/platform#4790 feat(drive)!: keep per-issuer token supply rollups and a destroyed-issuer ledger](https://github.com/dashpay/platform/pull/4790) — ⚠ merge conflict · 🔴 CI failing · 🐢 targets v6.0-dev · areas: rs-drive, rs-drive-abci, dpp, fallback · Policy: waiting-build
  - Blocker: The build must pass before a human is asked
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/platform#4794 feat: pin contract configuration vectors and mirror them in wasm-dpp2 and the sdk ffi](https://github.com/dashpay/platform/pull/4794) — 🔴 CI failing · 🐢 targets v5.1-dev · areas: dpp, rust-sdk-ffi, fallback · Policy: waiting-build
  - Blocker: The build must pass before a human is asked
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/platform#4808 test(drive-abci): rehearse a reproducible scheduled host fault and the existing recovery path](https://github.com/dashpay/platform/pull/4808) — 🐢 targets v6.0-dev · areas: rs-drive-abci, github, fallback · Policy: ready-for-human
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
#### Ready for human (14)
- [dashpay/platform#4703 fix(platform): resolve fee versions by registered number and price refunds at the storage epoch rate](https://github.com/dashpay/platform/pull/4703) — 🐢 targets v5.1-dev · areas: rs-drive, rs-drive-abci, dpp, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
- [dashpay/platform#4704 fix(platform): record the genesis fee generation and add replay coverage across a fee-version boundary](https://github.com/dashpay/platform/pull/4704) — 🐢 targets v5.1-dev · areas: rs-drive-abci, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/platform#4705 feat(platform): define protocol-versioned smart-contract computation limits and their gas representation](https://github.com/dashpay/platform/pull/4705) — 🐢 targets v6.0-dev · areas: rs-drive, rs-drive-abci, dpp, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/platform#4706 feat(platform)!: require fee history for storage refunds and credit their recorded owners](https://github.com/dashpay/platform/pull/4706) — 🐢 targets v5.1-dev · areas: rs-drive, rs-drive-abci, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/platform#4707 feat: alloc-only profiles for platform-value and platform-serialization](https://github.com/dashpay/platform/pull/4707) — 🐢 targets v5.1-dev · areas: github, fallback · Policy: ready-for-human
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
- [dashpay/platform#4789 feat(platform): add typed refund owners to storage flags and fee refunds](https://github.com/dashpay/platform/pull/4789) — 🐢 targets v6.0-dev · areas: rs-drive, dpp, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/platform#4808 test(drive-abci): rehearse a reproducible scheduled host fault and the existing recovery path](https://github.com/dashpay/platform/pull/4808) — 🐢 targets v6.0-dev · areas: rs-drive-abci, github, fallback · Policy: ready-for-human
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
- [dashpay/platform#4703 fix(platform): resolve fee versions by registered number and price refunds at the storage epoch rate](https://github.com/dashpay/platform/pull/4703) — 🐢 targets v5.1-dev · areas: rs-drive, rs-drive-abci, dpp, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
- [dashpay/platform#4704 fix(platform): record the genesis fee generation and add replay coverage across a fee-version boundary](https://github.com/dashpay/platform/pull/4704) — 🐢 targets v5.1-dev · areas: rs-drive-abci, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/platform#4705 feat(platform): define protocol-versioned smart-contract computation limits and their gas representation](https://github.com/dashpay/platform/pull/4705) — 🐢 targets v6.0-dev · areas: rs-drive, rs-drive-abci, dpp, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/platform#4706 feat(platform)!: require fee history for storage refunds and credit their recorded owners](https://github.com/dashpay/platform/pull/4706) — 🐢 targets v5.1-dev · areas: rs-drive, rs-drive-abci, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/platform#4707 feat: alloc-only profiles for platform-value and platform-serialization](https://github.com/dashpay/platform/pull/4707) — 🐢 targets v5.1-dev · areas: github, fallback · Policy: ready-for-human
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
- [dashpay/platform#4718 ci: compare cross-architecture replay artifacts across a protocol upgrade and a restart](https://github.com/dashpay/platform/pull/4718) — 🔴 CI failing · 🐢 targets v6.0-dev · areas: rs-drive-abci, github, fallback · Policy: waiting-build
  - Blocker: The build must pass before a human is asked
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/platform#4719 feat: add the dash-sdk-contract declaration model, grammar and diagnostics](https://github.com/dashpay/platform/pull/4719) — 🔴 CI failing · 🐢 targets v5.1-dev · areas: github, fallback · Policy: waiting-build
  - Blocker: The build must pass before a human is asked
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/platform#4784 feat(platform)!: validate contested index parameters and bind the native award to its poll](https://github.com/dashpay/platform/pull/4784) — ⚠ merge conflict · 🔴 CI failing · 🐢 targets v6.0-dev · areas: rs-drive, rs-drive-abci, dpp, fallback · Policy: waiting-build
  - Blocker: The build must pass before a human is asked
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/platform#4789 feat(platform): add typed refund owners to storage flags and fee refunds](https://github.com/dashpay/platform/pull/4789) — 🐢 targets v6.0-dev · areas: rs-drive, dpp, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/platform#4790 feat(drive)!: keep per-issuer token supply rollups and a destroyed-issuer ledger](https://github.com/dashpay/platform/pull/4790) — ⚠ merge conflict · 🔴 CI failing · 🐢 targets v6.0-dev · areas: rs-drive, rs-drive-abci, dpp, fallback · Policy: waiting-build
  - Blocker: The build must pass before a human is asked
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/platform#4794 feat: pin contract configuration vectors and mirror them in wasm-dpp2 and the sdk ffi](https://github.com/dashpay/platform/pull/4794) — 🔴 CI failing · 🐢 targets v5.1-dev · areas: dpp, rust-sdk-ffi, fallback · Policy: waiting-build
  - Blocker: The build must pass before a human is asked
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/platform#4808 test(drive-abci): rehearse a reproducible scheduled host fault and the existing recovery path](https://github.com/dashpay/platform/pull/4808) — 🐢 targets v6.0-dev · areas: rs-drive-abci, github, fallback · Policy: ready-for-human
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

<a id="thepastaclaw"></a>
### @thepastaclaw
<a id="thepastaclaw-open"></a>
#### Open (1)
- [dashpay/platform#4298 fix(dashmate): load ZeroSSL config in force mode](https://github.com/dashpay/platform/pull/4298) — 🐢 targets v5.1-dev · areas: dashmate · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
  - Blocker: Proceeded without coderabbitai: no review within the configured window

<a id="thepastaclaw-ready-for-human"></a>
#### Ready for human (1)
- [dashpay/platform#4298 fix(dashmate): load ZeroSSL config in force mode](https://github.com/dashpay/platform/pull/4298) — 🐢 targets v5.1-dev · areas: dashmate · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
  - Blocker: Proceeded without coderabbitai: no review within the configured window

<a id="thepastaclaw-stale"></a>
#### Stale (1)
- [dashpay/platform#4298 fix(dashmate): load ZeroSSL config in force mode](https://github.com/dashpay/platform/pull/4298) — 🐢 targets v5.1-dev · areas: dashmate · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
  - Blocker: Proceeded without coderabbitai: no review within the configured window

## Methodology
Generated nightly by [pr-hygiene](https://github.com/dashpay/stale_prs_are_bad). A thread counts as "unresolved" when it is open, not outdated, has a comment from someone other than the PR author, and the most recent comment is from a reviewer. **Dirty** = at least one such thread. **Unresolved Comments** = at least one such thread. **Changes Requested** = no unresolved threads but a reviewer's most recent review is CHANGES_REQUESTED (still blocking until someone re-approves or dismisses). **Deferred** = carries a configured deferred label (e.g. `postponed`) — visible but not counted toward unresolved-comment counts. **Stale** = targets a non-default branch OR hasn't been touched in the configured threshold (default 120 days, but clean PRs are never reclassified as stale). **Draft** = the PR is still marked draft on GitHub. **CI failing** = no unresolved comments, no changes-requested, but the latest commit's status check is failing. **Clean** = open, not draft, not deferred, not stale, no unresolved comments, no changes-requested, CI green. **Needs action** further requires changes-requested, merge conflict, or that the reviewer commented more recently than the author last pushed. **Ready for human** counts a person's own PRs that the shared review engine marks `ready-for-human` or `ready-to-merge`; each PR bullet shows the engine's state as `Policy:` with its blockers. **Ready for Review** counts clean PRs (authored by someone else) where this person owes a review: the union of the shared policy's routing (owners and reviewers of every area the changed files fall into, or the repository fallback for files no area claims) and GitHub's explicit review requests. The author is never routed to their own PR, and anyone who has already submitted any review is excluded — their job is done. `⚠ ownership unresolved` marks areas whose roster the policy still lists as open. Configurable via [`https://github.com/dashpay/stale_prs_are_bad/blob/master/.pr-hygiene.yml`](https://github.com/dashpay/stale_prs_are_bad/blob/master/.pr-hygiene.yml)—edit defaults there; ownership lives in `policies/`.
