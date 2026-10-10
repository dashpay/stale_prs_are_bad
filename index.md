---
---
# PR Hygiene Report
*Last updated: 2026-10-10 06:20 UTC · commit e5ad558*

## Summary
- Open PRs: **157** (21 clean · 4 CI failing · 3 changes requested · 25 unresolved comments · 0 deferred · 18 draft · 86 stale)
- PRs needing author action: **32**
- Total unresolved comments: **169**
- dashpay/platform: **102** open (8 clean · 1 CI failing · 0 changes requested · 11 unresolved comments · 0 deferred · 3 draft · 79 stale) · engine: 15 draft · 17 ready-for-human · 1 ready-to-merge · 16 waiting-author · 5 waiting-bots · 5 waiting-build · 27 waiting-self-review · 16 no verdict
- dashpay/rust-dashcore: **39** open (8 clean · 2 CI failing · 3 changes requested · 8 unresolved comments · 0 deferred · 13 draft · 5 stale) · engine: 16 draft · 3 ready-for-human · 6 waiting-author · 1 waiting-build · 11 waiting-self-review · 2 no verdict
- dashpay/tenderdash: **4** open (0 clean · 1 CI failing · 0 changes requested · 2 unresolved comments · 0 deferred · 1 draft · 0 stale) · engine: 1 draft · 1 ready-for-human · 1 ready-to-merge · 1 waiting-self-review
- dashpay/grovedb: **3** open (1 clean · 0 CI failing · 0 changes requested · 1 unresolved comments · 0 deferred · 1 draft · 0 stale) · engine: 1 draft · 2 waiting-self-review
- dashpay/dash-evo-tool: **9** open (4 clean · 0 CI failing · 0 changes requested · 3 unresolved comments · 0 deferred · 0 draft · 2 stale) · engine: 4 ready-for-human · 3 waiting-author · 2 no verdict

## Scoreboard
_Sort: unresolved-comments desc → needs-action desc → ready-for-review desc. Click any number to jump to the specific PRs it covers._

| Author | Open | Clean | CI failing | Unresolved Comments | Changes Requested | Deferred | Draft | Stale | Needs action | Ready for human | Total Unresolved Comments | Ready for Review | Δ |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| [@QuantumExplorer](#quantumexplorer) | [21](#quantumexplorer-open) | [1](#quantumexplorer-clean) | — | [8](#quantumexplorer-unresolved-comments) | — | — | [2](#quantumexplorer-draft) | [10](#quantumexplorer-stale) | [8](#quantumexplorer-needs-action) | — | [40](#quantumexplorer-unresolved-comments) | [14](#quantumexplorer-ready-for-review) | ↓ 3 |
| [@shumkov](#shumkov) | [18](#shumkov-open) | [6](#shumkov-clean) | — | [4](#shumkov-unresolved-comments) | — | — | — | [8](#shumkov-stale) | [4](#shumkov-needs-action) | — | [38](#shumkov-unresolved-comments) | [2](#shumkov-ready-for-review) | ↑ 1 |
| [@thepastaclaw](#thepastaclaw) | [8](#thepastaclaw-open) | [1](#thepastaclaw-clean) | [1](#thepastaclaw-ci-failing) | [2](#thepastaclaw-unresolved-comments) | — | — | [2](#thepastaclaw-draft) | [2](#thepastaclaw-stale) | [4](#thepastaclaw-needs-action) | [2](#thepastaclaw-ready-for-human) | [9](#thepastaclaw-unresolved-comments) | — | ↑ 1 |
| [@lklimek](#lklimek) | [15](#lklimek-open) | [1](#lklimek-clean) | [1](#lklimek-ci-failing) | [2](#lklimek-unresolved-comments) | — | — | [3](#lklimek-draft) | [8](#lklimek-stale) | [3](#lklimek-needs-action) | [2](#lklimek-ready-for-human) | [12](#lklimek-unresolved-comments) | [6](#lklimek-ready-for-review) | ↓ 2 |
| [@ZocoLini](#zocolini) | [10](#zocolini-open) | [3](#zocolini-clean) | — | [2](#zocolini-unresolved-comments) | [1](#zocolini-changes-requested) | — | [4](#zocolini-draft) | — | [3](#zocolini-needs-action) | — | [7](#zocolini-unresolved-comments) | [3](#zocolini-ready-for-review) | ↑ 3 |
| [@PastaPastaPasta](#pastapastapasta) | [22](#pastapastapasta-open) | — | [1](#pastapastapasta-ci-failing) | [2](#pastapastapasta-unresolved-comments) | — | — | [1](#pastapastapasta-draft) | [18](#pastapastapasta-stale) | [3](#pastapastapasta-needs-action) | [1](#pastapastapasta-ready-for-human) | [31](#pastapastapasta-unresolved-comments) | — | ↓ 6 |
| [@romchornyi](#romchornyi) | [5](#romchornyi-open) | — | [1](#romchornyi-ci-failing) | [1](#romchornyi-unresolved-comments) | [1](#romchornyi-changes-requested) | — | [1](#romchornyi-draft) | [1](#romchornyi-stale) | [2](#romchornyi-needs-action) | [1](#romchornyi-ready-for-human) | [2](#romchornyi-unresolved-comments) | — | — |
| [@llbartekll](#llbartekll) | [3](#llbartekll-open) | — | — | [1](#llbartekll-unresolved-comments) | [1](#llbartekll-changes-requested) | — | [1](#llbartekll-draft) | — | [2](#llbartekll-needs-action) | — | [1](#llbartekll-unresolved-comments) | — | — |
| [@xdustinface](#xdustinface) | [8](#xdustinface-open) | [1](#xdustinface-clean) | — | [1](#xdustinface-unresolved-comments) | — | — | [3](#xdustinface-draft) | [3](#xdustinface-stale) | [1](#xdustinface-needs-action) | — | [1](#xdustinface-unresolved-comments) | [6](#xdustinface-ready-for-review) | — |
| [@Claudius-Maginificent](#claudius-maginificent) | [9](#claudius-maginificent-open) | [4](#claudius-maginificent-clean) | — | [1](#claudius-maginificent-unresolved-comments) | — | — | — | [4](#claudius-maginificent-stale) | [1](#claudius-maginificent-needs-action) | [4](#claudius-maginificent-ready-for-human) | [11](#claudius-maginificent-unresolved-comments) | — | ↑ 1 |
| [@HashEngineering](#hashengineering) | [4](#hashengineering-open) | — | — | [1](#hashengineering-unresolved-comments) | — | — | [1](#hashengineering-draft) | [2](#hashengineering-stale) | [1](#hashengineering-needs-action) | — | [8](#hashengineering-unresolved-comments) | — | — |
| [@infraclaw-dash](#infraclaw-dash) | [3](#infraclaw-dash-open) | — | — | — | — | — | — | [3](#infraclaw-dash-stale) | — | [1](#infraclaw-dash-ready-for-human) | [5](#infraclaw-dash-unresolved-comments) | — | — |
| [@bfoss765](#bfoss765) | [2](#bfoss765-open) | — | — | — | — | — | — | [2](#bfoss765-stale) | — | — | [3](#bfoss765-unresolved-comments) | — | — |
| [@ktechmidas](#ktechmidas) | [8](#ktechmidas-open) | [2](#ktechmidas-clean) | — | — | — | — | — | [6](#ktechmidas-stale) | — | — | [1](#ktechmidas-unresolved-comments) | — | — |
| [@DCG-Claude](#dcg-claude) | [19](#dcg-claude-open) | — | — | — | — | — | — | [19](#dcg-claude-stale) | — | [14](#dcg-claude-ready-for-human) | — | — | — |
| [@kwvg](#kwvg) | [2](#kwvg-open) | [2](#kwvg-clean) | — | — | — | — | — | — | — | [2](#kwvg-ready-for-human) | — | — | — |

## Per-author detail

<a id="quantumexplorer"></a>
### @QuantumExplorer
<a id="quantumexplorer-open"></a>
#### Open (21)
- [dashpay/rust-dashcore#1078 feat(key-wallet)!: lock masternode collateral out of coin selection](https://github.com/dashpay/rust-dashcore/pull/1078) — 8 unresolved (8 human) · 10 days stale · ⚠ merge conflict · ✋ changes requested · areas: key-wallet, key-wallet-manager · Policy: waiting-self-review
  - Top thread: "This sets the state by hand, and together with the serde round-trip it is the only coverage of the refresh inside \`updat…" — 10 days old
  - Blocker: Author must post /self-reviewed 44a9a010500fbd44aba09111b4c53a28b88e8073
- [dashpay/platform#4272 feat(dpp)!: dashpay payment detection keys and stealth derivation](https://github.com/dashpay/platform/pull/4272) — 9 unresolved (9 bot) · 68 days stale · ⚠ merge conflict · 📝 draft · 🐢 targets v5.1-dev · areas: rs-drive, rs-drive-abci, dpp, rs-platform-wallet, swift-sdk, system-contracts, kotlin-sdk, fallback · Policy: draft
  - Top thread: "🔴 Blocking: Require compressed 33-byte encodings for payment keys**" — 68 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#860 feat(dash-spv): expose per-peer connection stats (ping RTT, bytes received)](https://github.com/dashpay/rust-dashcore/pull/860) — 3 unresolved (3 CodeRabbit) · 92 days stale · ✋ changes requested · areas: dash-spv, fallback · Policy: waiting-author
  - Top thread: "_📐 Maintainability &amp; Code Quality_ \| _🟡 Minor_ \| _⚡ Quick win_" — 92 days old
  - Blocker: coderabbitai left review threads unresolved; resolve them
- [dashpay/rust-dashcore#622 fix: apply BIP-69 sorting to asset lock credit outputs](https://github.com/dashpay/rust-dashcore/pull/622) — 1 unresolved (1 CodeRabbit) · 191 days stale · ⚠ merge conflict · ✋ changes requested · 📝 draft · areas: key-wallet · Policy: draft
  - Top thread: "_⚠️ Potential issue_ \| _🟠 Major_" — 191 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#3417 feat(swift-sdk): use SPV-synced quorums for Platform proof verification](https://github.com/dashpay/platform/pull/3417) — 3 unresolved (3 bot) · 31 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · 🐢 targets v5.1-dev · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, rust-sdk, rust-sdk-ffi, fallback · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Exercise the production runtime bridge across Tokio runtime flavors**" — 31 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/platform#5238 fix(platform)!: refund sponsor-paid document storage to the gas sponsor (PV14)](https://github.com/dashpay/platform/pull/5238) — 3 unresolved (3 bot) · 8 days stale · ⚠ merge conflict · ✋ changes requested · areas: rs-drive, rs-drive-abci, fallback · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Make the transfer regression change the primary element's size**" — 8 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/rust-dashcore#136 feat: update integration test script for Dash compatibility](https://github.com/dashpay/rust-dashcore/pull/136) — 1 unresolved (1 CodeRabbit) · 401 days stale · 🔴 CI failing · 🐢 targets v0.40-dev, untouched 401 days · areas: fallback
  - Top thread: "_🛠️ Refactor suggestion_" — 401 days old
- [dashpay/rust-dashcore#1077 feat(dash-spv)!: check masternode list diffs against the block coinbase](https://github.com/dashpay/rust-dashcore/pull/1077) — 2 unresolved (2 CodeRabbit) · 12 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · areas: dash-spv, fallback · Policy: waiting-author
  - Top thread: "_🩺 Stability &amp; Availability_ \| _🟠 Major_ \| _⚡ Quick win_" — 12 days old
  - Blocker: coderabbitai requested changes on this head; dismiss the review or push a fix
  - Blocker: coderabbitai left review threads unresolved; resolve them
- [dashpay/grovedb#1015 feat(batch)!: move an element to a new key, keeping its stored subtree](https://github.com/dashpay/grovedb/pull/1015) — 3 unresolved (3 bot) · 1 days stale · areas: fallback · Policy: waiting-self-review
  - Top thread: "🟡 Suggestion: Exercise iterator failures after the copy has staged records**" — 1 days old
  - Blocker: Author must post /self-reviewed 3d89590b43c479fb37dae8dac9b5a78aed02eebe
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/platform#5365 feat(dpp): consume and minimumAgeBlocks on any reference judged on the create alone](https://github.com/dashpay/platform/pull/5365) — 3 unresolved (3 bot) · 1 days stale · ✋ changes requested · areas: rs-drive-abci, dpp, fallback · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Record the breaking public DPP method signature**" — 1 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/platform#5337 feat(platform)!: dpns contract v3 checks name registration with schema keywords](https://github.com/dashpay/platform/pull/5337) — 2 unresolved (2 bot) · 1 days stale · areas: rs-drive, rs-drive-abci, dpp, rust-sdk, js-wasm-sdk, system-contracts, fallback · Policy: waiting-bots
  - Top thread: "🟡 Suggestion: Cover authenticated proof-version fallback through the production entry point**" — 1 days old
  - Blocker: coderabbitai has not reported for the current head
- [dashpay/platform#5350 fix(platform)!: key unsigned integer index values in value order (PV14)](https://github.com/dashpay/platform/pull/5350) — 1 unresolved (1 bot) · 1 days stale · ✋ changes requested · areas: rs-drive, rs-drive-abci, dpp, fallback · Policy: waiting-author
  - Top thread: "🔴 Blocking: Keep skipped indexes compatible with PV14 readers and writers**" — 1 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/platform#5360 perf(drive): delete a consumed document from what its lookup read](https://github.com/dashpay/platform/pull/5360) — 1 unresolved (1 bot) · 1 days stale · 🐢 targets claude/wizardly-driscoll-44c981 · areas: rs-drive, rs-drive-abci, fallback
  - Top thread: "🟡 Suggestion: Add negative-path tests for the new read-document deletion guards**" — 1 days old
- [dashpay/grovedb#1001 feat(batch)!: opt-in settlement of storage owner changes](https://github.com/dashpay/grovedb/pull/1001) — areas: fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 814dfc765747cec0beb7a39e1cdf04d301ce9ac3
- [dashpay/platform#4519 feat(platform-wallet): rotate a masternode's keys into the wallet — ProUpRegTx orchestration, FFI, Swift](https://github.com/dashpay/platform/pull/4519) — ⚠ merge conflict · 🐢 targets v5.1-dev · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 92a3ebe17adc12432fb756c63f172c663809007d
  - Blocker: Proceeded without coderabbitai: no review within the configured window
  - Blocker: Proceeded without thepastaclaw: no review within the configured window
- [dashpay/platform#4730 feat(platform)!: delta-based data contract update transition for protocol version 15](https://github.com/dashpay/platform/pull/4730) — ⚠ merge conflict · 🔴 CI failing · 🐢 targets v5.1-dev · areas: rs-drive, rs-drive-abci, dpp, rs-platform-wallet, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 53cb036efc244155d88a30c829148f90d7903d3e
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/platform#4776 feat(platform-wallet)!: let contract updates clear the description through the FFI](https://github.com/dashpay/platform/pull/4776) — 🔴 CI failing · 🐢 targets feat/delta-contract-update-pv15 · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk, fallback
- [dashpay/platform#4899 feat(platform)!: yes/no masternode vote poll kind with supermajority and minimum voting power](https://github.com/dashpay/platform/pull/4899) — ⚠ merge conflict · ✋ changes requested · 📝 draft · 🐢 targets v5.1-dev · areas: rs-drive, rs-drive-abci, dpp, swift-sdk, rust-sdk, rust-sdk-ffi, js-wasm-sdk, rust-dapi, kotlin-sdk, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#4933 feat(platform)!: dashpay contact requests declare their checks (PV14)](https://github.com/dashpay/platform/pull/4933) — ⚠ merge conflict · 📝 draft · areas: rs-drive, rs-drive-abci, dpp, system-contracts, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#4986 fix(drive-abci)!: pay every reward share of a masternode and credit each identity once in the epoch payout (PV15)](https://github.com/dashpay/platform/pull/4986) — ⚠ merge conflict · 📝 draft · 🐢 targets v5.1-dev · areas: rs-drive, rs-drive-abci, dpp, github, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5113 feat(platform-wallet)!: keep masternode collateral out of coin selection](https://github.com/dashpay/platform/pull/5113) — ⚠ merge conflict · 🔴 CI failing · 📝 draft · 🐢 targets chore/bump-rust-dashcore-secp-033 · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk, fallback

<a id="quantumexplorer-needs-action"></a>
#### Needs action (8)
- [dashpay/rust-dashcore#1078 feat(key-wallet)!: lock masternode collateral out of coin selection](https://github.com/dashpay/rust-dashcore/pull/1078) — 8 unresolved (8 human) · 10 days stale · ⚠ merge conflict · ✋ changes requested · areas: key-wallet, key-wallet-manager · Policy: waiting-self-review
  - Top thread: "This sets the state by hand, and together with the serde round-trip it is the only coverage of the refresh inside \`updat…" — 10 days old
  - Blocker: Author must post /self-reviewed 44a9a010500fbd44aba09111b4c53a28b88e8073
- [dashpay/rust-dashcore#860 feat(dash-spv): expose per-peer connection stats (ping RTT, bytes received)](https://github.com/dashpay/rust-dashcore/pull/860) — 3 unresolved (3 CodeRabbit) · 92 days stale · ✋ changes requested · areas: dash-spv, fallback · Policy: waiting-author
  - Top thread: "_📐 Maintainability &amp; Code Quality_ \| _🟡 Minor_ \| _⚡ Quick win_" — 92 days old
  - Blocker: coderabbitai left review threads unresolved; resolve them
- [dashpay/platform#5238 fix(platform)!: refund sponsor-paid document storage to the gas sponsor (PV14)](https://github.com/dashpay/platform/pull/5238) — 3 unresolved (3 bot) · 8 days stale · ⚠ merge conflict · ✋ changes requested · areas: rs-drive, rs-drive-abci, fallback · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Make the transfer regression change the primary element's size**" — 8 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/rust-dashcore#1077 feat(dash-spv)!: check masternode list diffs against the block coinbase](https://github.com/dashpay/rust-dashcore/pull/1077) — 2 unresolved (2 CodeRabbit) · 12 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · areas: dash-spv, fallback · Policy: waiting-author
  - Top thread: "_🩺 Stability &amp; Availability_ \| _🟠 Major_ \| _⚡ Quick win_" — 12 days old
  - Blocker: coderabbitai requested changes on this head; dismiss the review or push a fix
  - Blocker: coderabbitai left review threads unresolved; resolve them
- [dashpay/grovedb#1015 feat(batch)!: move an element to a new key, keeping its stored subtree](https://github.com/dashpay/grovedb/pull/1015) — 3 unresolved (3 bot) · 1 days stale · areas: fallback · Policy: waiting-self-review
  - Top thread: "🟡 Suggestion: Exercise iterator failures after the copy has staged records**" — 1 days old
  - Blocker: Author must post /self-reviewed 3d89590b43c479fb37dae8dac9b5a78aed02eebe
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/platform#5365 feat(dpp): consume and minimumAgeBlocks on any reference judged on the create alone](https://github.com/dashpay/platform/pull/5365) — 3 unresolved (3 bot) · 1 days stale · ✋ changes requested · areas: rs-drive-abci, dpp, fallback · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Record the breaking public DPP method signature**" — 1 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/platform#5337 feat(platform)!: dpns contract v3 checks name registration with schema keywords](https://github.com/dashpay/platform/pull/5337) — 2 unresolved (2 bot) · 1 days stale · areas: rs-drive, rs-drive-abci, dpp, rust-sdk, js-wasm-sdk, system-contracts, fallback · Policy: waiting-bots
  - Top thread: "🟡 Suggestion: Cover authenticated proof-version fallback through the production entry point**" — 1 days old
  - Blocker: coderabbitai has not reported for the current head
- [dashpay/platform#5350 fix(platform)!: key unsigned integer index values in value order (PV14)](https://github.com/dashpay/platform/pull/5350) — 1 unresolved (1 bot) · 1 days stale · ✋ changes requested · areas: rs-drive, rs-drive-abci, dpp, fallback · Policy: waiting-author
  - Top thread: "🔴 Blocking: Keep skipped indexes compatible with PV14 readers and writers**" — 1 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return

<a id="quantumexplorer-unresolved-comments"></a>
#### Unresolved Comments (8)
- [dashpay/rust-dashcore#1078 feat(key-wallet)!: lock masternode collateral out of coin selection](https://github.com/dashpay/rust-dashcore/pull/1078) — 8 unresolved (8 human) · 10 days stale · ⚠ merge conflict · ✋ changes requested · areas: key-wallet, key-wallet-manager · Policy: waiting-self-review
  - Top thread: "This sets the state by hand, and together with the serde round-trip it is the only coverage of the refresh inside \`updat…" — 10 days old
  - Blocker: Author must post /self-reviewed 44a9a010500fbd44aba09111b4c53a28b88e8073
- [dashpay/rust-dashcore#860 feat(dash-spv): expose per-peer connection stats (ping RTT, bytes received)](https://github.com/dashpay/rust-dashcore/pull/860) — 3 unresolved (3 CodeRabbit) · 92 days stale · ✋ changes requested · areas: dash-spv, fallback · Policy: waiting-author
  - Top thread: "_📐 Maintainability &amp; Code Quality_ \| _🟡 Minor_ \| _⚡ Quick win_" — 92 days old
  - Blocker: coderabbitai left review threads unresolved; resolve them
- [dashpay/platform#5238 fix(platform)!: refund sponsor-paid document storage to the gas sponsor (PV14)](https://github.com/dashpay/platform/pull/5238) — 3 unresolved (3 bot) · 8 days stale · ⚠ merge conflict · ✋ changes requested · areas: rs-drive, rs-drive-abci, fallback · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Make the transfer regression change the primary element's size**" — 8 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/rust-dashcore#1077 feat(dash-spv)!: check masternode list diffs against the block coinbase](https://github.com/dashpay/rust-dashcore/pull/1077) — 2 unresolved (2 CodeRabbit) · 12 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · areas: dash-spv, fallback · Policy: waiting-author
  - Top thread: "_🩺 Stability &amp; Availability_ \| _🟠 Major_ \| _⚡ Quick win_" — 12 days old
  - Blocker: coderabbitai requested changes on this head; dismiss the review or push a fix
  - Blocker: coderabbitai left review threads unresolved; resolve them
- [dashpay/grovedb#1015 feat(batch)!: move an element to a new key, keeping its stored subtree](https://github.com/dashpay/grovedb/pull/1015) — 3 unresolved (3 bot) · 1 days stale · areas: fallback · Policy: waiting-self-review
  - Top thread: "🟡 Suggestion: Exercise iterator failures after the copy has staged records**" — 1 days old
  - Blocker: Author must post /self-reviewed 3d89590b43c479fb37dae8dac9b5a78aed02eebe
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/platform#5365 feat(dpp): consume and minimumAgeBlocks on any reference judged on the create alone](https://github.com/dashpay/platform/pull/5365) — 3 unresolved (3 bot) · 1 days stale · ✋ changes requested · areas: rs-drive-abci, dpp, fallback · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Record the breaking public DPP method signature**" — 1 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/platform#5337 feat(platform)!: dpns contract v3 checks name registration with schema keywords](https://github.com/dashpay/platform/pull/5337) — 2 unresolved (2 bot) · 1 days stale · areas: rs-drive, rs-drive-abci, dpp, rust-sdk, js-wasm-sdk, system-contracts, fallback · Policy: waiting-bots
  - Top thread: "🟡 Suggestion: Cover authenticated proof-version fallback through the production entry point**" — 1 days old
  - Blocker: coderabbitai has not reported for the current head
- [dashpay/platform#5350 fix(platform)!: key unsigned integer index values in value order (PV14)](https://github.com/dashpay/platform/pull/5350) — 1 unresolved (1 bot) · 1 days stale · ✋ changes requested · areas: rs-drive, rs-drive-abci, dpp, fallback · Policy: waiting-author
  - Top thread: "🔴 Blocking: Keep skipped indexes compatible with PV14 readers and writers**" — 1 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return

<a id="quantumexplorer-draft"></a>
#### Draft (2)
- [dashpay/rust-dashcore#622 fix: apply BIP-69 sorting to asset lock credit outputs](https://github.com/dashpay/rust-dashcore/pull/622) — 1 unresolved (1 CodeRabbit) · 191 days stale · ⚠ merge conflict · ✋ changes requested · 📝 draft · areas: key-wallet · Policy: draft
  - Top thread: "_⚠️ Potential issue_ \| _🟠 Major_" — 191 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#4933 feat(platform)!: dashpay contact requests declare their checks (PV14)](https://github.com/dashpay/platform/pull/4933) — ⚠ merge conflict · 📝 draft · areas: rs-drive, rs-drive-abci, dpp, system-contracts, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot

<a id="quantumexplorer-stale"></a>
#### Stale (10)
- [dashpay/platform#4272 feat(dpp)!: dashpay payment detection keys and stealth derivation](https://github.com/dashpay/platform/pull/4272) — 9 unresolved (9 bot) · 68 days stale · ⚠ merge conflict · 📝 draft · 🐢 targets v5.1-dev · areas: rs-drive, rs-drive-abci, dpp, rs-platform-wallet, swift-sdk, system-contracts, kotlin-sdk, fallback · Policy: draft
  - Top thread: "🔴 Blocking: Require compressed 33-byte encodings for payment keys**" — 68 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#3417 feat(swift-sdk): use SPV-synced quorums for Platform proof verification](https://github.com/dashpay/platform/pull/3417) — 3 unresolved (3 bot) · 31 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · 🐢 targets v5.1-dev · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, rust-sdk, rust-sdk-ffi, fallback · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Exercise the production runtime bridge across Tokio runtime flavors**" — 31 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/rust-dashcore#136 feat: update integration test script for Dash compatibility](https://github.com/dashpay/rust-dashcore/pull/136) — 1 unresolved (1 CodeRabbit) · 401 days stale · 🔴 CI failing · 🐢 targets v0.40-dev, untouched 401 days · areas: fallback
  - Top thread: "_🛠️ Refactor suggestion_" — 401 days old
- [dashpay/platform#5360 perf(drive): delete a consumed document from what its lookup read](https://github.com/dashpay/platform/pull/5360) — 1 unresolved (1 bot) · 1 days stale · 🐢 targets claude/wizardly-driscoll-44c981 · areas: rs-drive, rs-drive-abci, fallback
  - Top thread: "🟡 Suggestion: Add negative-path tests for the new read-document deletion guards**" — 1 days old
- [dashpay/platform#4519 feat(platform-wallet): rotate a masternode's keys into the wallet — ProUpRegTx orchestration, FFI, Swift](https://github.com/dashpay/platform/pull/4519) — ⚠ merge conflict · 🐢 targets v5.1-dev · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 92a3ebe17adc12432fb756c63f172c663809007d
  - Blocker: Proceeded without coderabbitai: no review within the configured window
  - Blocker: Proceeded without thepastaclaw: no review within the configured window
- [dashpay/platform#4730 feat(platform)!: delta-based data contract update transition for protocol version 15](https://github.com/dashpay/platform/pull/4730) — ⚠ merge conflict · 🔴 CI failing · 🐢 targets v5.1-dev · areas: rs-drive, rs-drive-abci, dpp, rs-platform-wallet, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 53cb036efc244155d88a30c829148f90d7903d3e
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/platform#4776 feat(platform-wallet)!: let contract updates clear the description through the FFI](https://github.com/dashpay/platform/pull/4776) — 🔴 CI failing · 🐢 targets feat/delta-contract-update-pv15 · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk, fallback
- [dashpay/platform#4899 feat(platform)!: yes/no masternode vote poll kind with supermajority and minimum voting power](https://github.com/dashpay/platform/pull/4899) — ⚠ merge conflict · ✋ changes requested · 📝 draft · 🐢 targets v5.1-dev · areas: rs-drive, rs-drive-abci, dpp, swift-sdk, rust-sdk, rust-sdk-ffi, js-wasm-sdk, rust-dapi, kotlin-sdk, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#4986 fix(drive-abci)!: pay every reward share of a masternode and credit each identity once in the epoch payout (PV15)](https://github.com/dashpay/platform/pull/4986) — ⚠ merge conflict · 📝 draft · 🐢 targets v5.1-dev · areas: rs-drive, rs-drive-abci, dpp, github, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5113 feat(platform-wallet)!: keep masternode collateral out of coin selection](https://github.com/dashpay/platform/pull/5113) — ⚠ merge conflict · 🔴 CI failing · 📝 draft · 🐢 targets chore/bump-rust-dashcore-secp-033 · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk, fallback

<a id="quantumexplorer-clean"></a>
#### Clean (1)
- [dashpay/grovedb#1001 feat(batch)!: opt-in settlement of storage owner changes](https://github.com/dashpay/grovedb/pull/1001) — areas: fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 814dfc765747cec0beb7a39e1cdf04d301ce9ac3

<a id="quantumexplorer-ready-for-review"></a>
#### Ready for Review (14)
- [dashpay/platform#5203 fix(release): install verified cargo-binstall binary directly](https://github.com/dashpay/platform/pull/5203) — by @ktechmidas · areas: fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed f8fee0b29313e97840ce6a67fa335e90b5da7301
- [dashpay/platform#5341 fix(platform): backfill historical credit nullifiers at pv14 activation](https://github.com/dashpay/platform/pull/5341) — by @shumkov · areas: rs-drive, rs-drive-abci, fallback · Policy: waiting-author
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/platform#5343 test(drive-abci): verify query errors over gRPC and original proofs](https://github.com/dashpay/platform/pull/5343) — by @shumkov · areas: rs-drive-abci, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed da55fa317f53e9196dd79a8e0f5fea2413837330
- [dashpay/platform#5349 fix(drive-abci): classify unsupported offsets and composite limits](https://github.com/dashpay/platform/pull/5349) — by @shumkov · areas: rs-drive-abci, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 90200c2fb795e81853b48c565bd555e950deafb3
- [dashpay/platform#5351 fix(dapi): classify malformed protobuf requests as invalid arguments](https://github.com/dashpay/platform/pull/5351) — by @shumkov · areas: rs-drive-abci, rust-dapi, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 1b34e746be0a0e148d16db7e3e1b63b1a4f9a647
- [dashpay/platform#5356 fix(sdk): bind voting proof queries to the default limit](https://github.com/dashpay/platform/pull/5356) — by @shumkov · areas: rs-drive, rs-drive-abci, rust-sdk, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 7f9a95470aaaebbadf2132e80545e72082e921d6
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/platform#5362 docs: correct nonce validation wording](https://github.com/dashpay/platform/pull/5362) — by @shumkov · areas: fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 13a6be83c5508534cb3eac5e3ebcda3e83fa3aa6
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/rust-dashcore#849 feat(dash-spv): add \`--birth-height\` CLI flag](https://github.com/dashpay/rust-dashcore/pull/849) — by @xdustinface · areas: dash-spv · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 9b1e709fd8e0bfab0dd69f7c86fd6b7d2e585fbe
- [dashpay/rust-dashcore#1106 fix(wallet): correct late inputs during InstantSend backfill](https://github.com/dashpay/rust-dashcore/pull/1106) — by @lklimek · areas: dash-spv, key-wallet, key-wallet-manager, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 61099afed724fa3c359e16f47c8533459f7b28e8
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/rust-dashcore#1136 feat(dash)!: decode unknown special transaction types as raw payloads](https://github.com/dashpay/rust-dashcore/pull/1136) — by @ZocoLini · areas: key-wallet, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 249ef12b0740cfca9947d0e823c4b9fd334b7379
- [dashpay/rust-dashcore#1142 fix(dash)!: adopt \`dash_num::Arith256\` around \`U256\` implementation, align closer to reference implementation](https://github.com/dashpay/rust-dashcore/pull/1142) — by @kwvg · areas: fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
- [dashpay/rust-dashcore#1144 fix!: reject WIF keys without \`0x01\` flag, reject sighash types &gt;\`0xff\`, maintain symmetry in &#123;sign,recover&#125; compact signatures, drop unused divergent segments](https://github.com/dashpay/rust-dashcore/pull/1144) — by @kwvg · areas: key-wallet, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
- [dashpay/rust-dashcore#1147 fix(rpc-client): keep request arguments and results out of the logs](https://github.com/dashpay/rust-dashcore/pull/1147) — by @ZocoLini · areas: fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 39048d217ecd02387cfd89ac0a79e45d210abc51
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/rust-dashcore#1148 fix(rpc-client): send the ChainLock block hash in display order to \`submitchainlock\`](https://github.com/dashpay/rust-dashcore/pull/1148) — by @ZocoLini · areas: fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 9240f4599b47a0b3936bbba06df7c99087f5e103

<a id="shumkov"></a>
### @shumkov
<a id="shumkov-open"></a>
#### Open (18)
- [dashpay/rust-dashcore#634 feat(key-wallet-manager): single-map WalletManager with Arc&lt;RwLock&lt;T&gt;&gt; per wallet](https://github.com/dashpay/rust-dashcore/pull/634) — 20 unresolved (20 CodeRabbit) · 185 days stale · ⚠ merge conflict · ✋ changes requested · areas: dash-spv, key-wallet, key-wallet-manager, fallback · Policy: waiting-author
  - Top thread: "_⚠️ Potential issue_ \| _🟡 Minor_" — 185 days old
  - Blocker: coderabbitai requested changes on this head; dismiss the review or push a fix
  - Blocker: coderabbitai left review threads unresolved; resolve them
- [dashpay/platform#4282 fix(dashmate)!: give Debian packages versions apt can order](https://github.com/dashpay/platform/pull/4282) — 6 unresolved (1 CodeRabbit, 5 bot) · 39 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · 🐢 targets v5.1-dev · areas: dashmate, js-wasm-sdk, system-contracts, github, fallback · Policy: waiting-author
  - Top thread: "_🗄️ Data Integrity &amp; Integration_ \| _🟡 Minor_ \| _⚡ Quick win_" — 39 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/platform#3936 chore(drive-abci): update to nested address in SML](https://github.com/dashpay/platform/pull/3936) — 2 unresolved (2 bot) · 113 days stale · ⚠ merge conflict · 📝 draft · 🐢 targets v5.1-dev · areas: rs-drive-abci, rust-sdk, fallback · Policy: draft
  - Top thread: "🟡 Suggestion: V0 reverse synthesizes \`addresses\` from a Core-22 port, masking subsequent legacy port-change diffs after …" — 113 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#4283 fix(dashmate): keep the node up when an image pull fails, and stop reporting success when it did not](https://github.com/dashpay/platform/pull/4283) — 2 unresolved (2 bot) · 33 days stale · ⚠ merge conflict · ✋ changes requested · 🐢 targets v5.1-dev · areas: dashmate · Policy: waiting-author
  - Top thread: "🟡 Suggestion: providers.js sanitizer misses U+061C (Arabic letter mark) that sanitizeRemoteText covers**" — 33 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/platform#4660 feat(sdk): expose the document erase and history lifecycle on mobile and FFI](https://github.com/dashpay/platform/pull/4660) — 2 unresolved (2 bot) · 17 days stale · ✋ changes requested · 🔴 CI failing · 🐢 targets keep-history-lifecycle · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, rust-sdk-ffi, kotlin-sdk, fallback
  - Top thread: "🟡 Suggestion: Validate history selector discriminant at the C boundary**" — 17 days old
- [dashpay/platform#4657 feat(platform)!: delete and erase lifecycle for keep-history documents](https://github.com/dashpay/platform/pull/4657) — 2 unresolved (2 bot) · 17 days stale · ✋ changes requested · 🔴 CI failing · 🐢 targets keep-history-storage-v2 · areas: rs-drive, rs-drive-abci, dpp, rust-sdk, js-wasm-sdk, fallback
  - Top thread: "🟡 Suggestion: Deduplicate lifecycle containers at the Drive batch boundary**" — 17 days old
- [dashpay/platform#5242 chore(skills): show each reviewer their part in /prs](https://github.com/dashpay/platform/pull/5242) — 2 unresolved (1 CodeRabbit, 1 bot) · 9 days stale · 🔴 CI failing · areas: fallback · Policy: waiting-self-review
  - Top thread: "_🎯 Functional Correctness_ \| _🟡 Minor_ \| _⚡ Quick win_" — 9 days old
  - Blocker: Author must post /self-reviewed ecd196e89d61bbcfcbf8b346563577d7f871efcc
- [dashpay/platform#4993 fix(drive-abci): a transition whose version is not active is not a decode failure](https://github.com/dashpay/platform/pull/4993) — 1 unresolved (1 bot) · 15 days stale · 🔴 CI failing · areas: rs-drive-abci · Policy: waiting-self-review
  - Top thread: "🟡 Suggestion: Assert the client-visible metadata in the inactive-version regression test**" — 15 days old
  - Blocker: Author must post /self-reviewed 32b49b0611944a65453ae688d095d4825a1ab18d
- [dashpay/platform#5361 fix(sdk)!: verify proofs with the protocol version their quorum signed](https://github.com/dashpay/platform/pull/5361) — 1 unresolved (1 bot) · 1 days stale · areas: rust-sdk, rust-sdk-ffi · Policy: waiting-self-review
  - Top thread: "🟡 Suggestion: Handle reserved mock-version values without panicking before authentication**" — 1 days old
  - Blocker: Author must post /self-reviewed 8c63e80fb23d2f45527375757b20103c0a6cd8b6
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/platform#4243 fix(sdk)!: complete encrypted txMetadata parity](https://github.com/dashpay/platform/pull/4243) — ⚠ merge conflict · 📝 draft · 🐢 targets v5.1-dev · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk, github, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#4285 feat(platform-wallet)!: invitation links are AppsFlyer applinks only](https://github.com/dashpay/platform/pull/4285) — ⚠ merge conflict · 🔴 CI failing · 📝 draft · 🐢 targets v5.1-dev · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#4652 feat(drive)!: preserve composite document history across protocol activation](https://github.com/dashpay/platform/pull/4652) — ⚠ merge conflict · ✋ changes requested · 🐢 targets v5.1-dev · areas: rs-drive, rs-drive-abci, rust-sdk, js-wasm-sdk, fallback · Policy: waiting-author
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/platform#5341 fix(platform): backfill historical credit nullifiers at pv14 activation](https://github.com/dashpay/platform/pull/5341) — areas: rs-drive, rs-drive-abci, fallback · Policy: waiting-author
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/platform#5343 test(drive-abci): verify query errors over gRPC and original proofs](https://github.com/dashpay/platform/pull/5343) — areas: rs-drive-abci, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed da55fa317f53e9196dd79a8e0f5fea2413837330
- [dashpay/platform#5349 fix(drive-abci): classify unsupported offsets and composite limits](https://github.com/dashpay/platform/pull/5349) — areas: rs-drive-abci, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 90200c2fb795e81853b48c565bd555e950deafb3
- [dashpay/platform#5351 fix(dapi): classify malformed protobuf requests as invalid arguments](https://github.com/dashpay/platform/pull/5351) — areas: rs-drive-abci, rust-dapi, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 1b34e746be0a0e148d16db7e3e1b63b1a4f9a647
- [dashpay/platform#5356 fix(sdk): bind voting proof queries to the default limit](https://github.com/dashpay/platform/pull/5356) — areas: rs-drive, rs-drive-abci, rust-sdk, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 7f9a95470aaaebbadf2132e80545e72082e921d6
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/platform#5362 docs: correct nonce validation wording](https://github.com/dashpay/platform/pull/5362) — areas: fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 13a6be83c5508534cb3eac5e3ebcda3e83fa3aa6
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return

<a id="shumkov-needs-action"></a>
#### Needs action (4)
- [dashpay/rust-dashcore#634 feat(key-wallet-manager): single-map WalletManager with Arc&lt;RwLock&lt;T&gt;&gt; per wallet](https://github.com/dashpay/rust-dashcore/pull/634) — 20 unresolved (20 CodeRabbit) · 185 days stale · ⚠ merge conflict · ✋ changes requested · areas: dash-spv, key-wallet, key-wallet-manager, fallback · Policy: waiting-author
  - Top thread: "_⚠️ Potential issue_ \| _🟡 Minor_" — 185 days old
  - Blocker: coderabbitai requested changes on this head; dismiss the review or push a fix
  - Blocker: coderabbitai left review threads unresolved; resolve them
- [dashpay/platform#5242 chore(skills): show each reviewer their part in /prs](https://github.com/dashpay/platform/pull/5242) — 2 unresolved (1 CodeRabbit, 1 bot) · 9 days stale · 🔴 CI failing · areas: fallback · Policy: waiting-self-review
  - Top thread: "_🎯 Functional Correctness_ \| _🟡 Minor_ \| _⚡ Quick win_" — 9 days old
  - Blocker: Author must post /self-reviewed ecd196e89d61bbcfcbf8b346563577d7f871efcc
- [dashpay/platform#4993 fix(drive-abci): a transition whose version is not active is not a decode failure](https://github.com/dashpay/platform/pull/4993) — 1 unresolved (1 bot) · 15 days stale · 🔴 CI failing · areas: rs-drive-abci · Policy: waiting-self-review
  - Top thread: "🟡 Suggestion: Assert the client-visible metadata in the inactive-version regression test**" — 15 days old
  - Blocker: Author must post /self-reviewed 32b49b0611944a65453ae688d095d4825a1ab18d
- [dashpay/platform#5361 fix(sdk)!: verify proofs with the protocol version their quorum signed](https://github.com/dashpay/platform/pull/5361) — 1 unresolved (1 bot) · 1 days stale · areas: rust-sdk, rust-sdk-ffi · Policy: waiting-self-review
  - Top thread: "🟡 Suggestion: Handle reserved mock-version values without panicking before authentication**" — 1 days old
  - Blocker: Author must post /self-reviewed 8c63e80fb23d2f45527375757b20103c0a6cd8b6
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return

<a id="shumkov-unresolved-comments"></a>
#### Unresolved Comments (4)
- [dashpay/rust-dashcore#634 feat(key-wallet-manager): single-map WalletManager with Arc&lt;RwLock&lt;T&gt;&gt; per wallet](https://github.com/dashpay/rust-dashcore/pull/634) — 20 unresolved (20 CodeRabbit) · 185 days stale · ⚠ merge conflict · ✋ changes requested · areas: dash-spv, key-wallet, key-wallet-manager, fallback · Policy: waiting-author
  - Top thread: "_⚠️ Potential issue_ \| _🟡 Minor_" — 185 days old
  - Blocker: coderabbitai requested changes on this head; dismiss the review or push a fix
  - Blocker: coderabbitai left review threads unresolved; resolve them
- [dashpay/platform#5242 chore(skills): show each reviewer their part in /prs](https://github.com/dashpay/platform/pull/5242) — 2 unresolved (1 CodeRabbit, 1 bot) · 9 days stale · 🔴 CI failing · areas: fallback · Policy: waiting-self-review
  - Top thread: "_🎯 Functional Correctness_ \| _🟡 Minor_ \| _⚡ Quick win_" — 9 days old
  - Blocker: Author must post /self-reviewed ecd196e89d61bbcfcbf8b346563577d7f871efcc
- [dashpay/platform#4993 fix(drive-abci): a transition whose version is not active is not a decode failure](https://github.com/dashpay/platform/pull/4993) — 1 unresolved (1 bot) · 15 days stale · 🔴 CI failing · areas: rs-drive-abci · Policy: waiting-self-review
  - Top thread: "🟡 Suggestion: Assert the client-visible metadata in the inactive-version regression test**" — 15 days old
  - Blocker: Author must post /self-reviewed 32b49b0611944a65453ae688d095d4825a1ab18d
- [dashpay/platform#5361 fix(sdk)!: verify proofs with the protocol version their quorum signed](https://github.com/dashpay/platform/pull/5361) — 1 unresolved (1 bot) · 1 days stale · areas: rust-sdk, rust-sdk-ffi · Policy: waiting-self-review
  - Top thread: "🟡 Suggestion: Handle reserved mock-version values without panicking before authentication**" — 1 days old
  - Blocker: Author must post /self-reviewed 8c63e80fb23d2f45527375757b20103c0a6cd8b6
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return

<a id="shumkov-stale"></a>
#### Stale (8)
- [dashpay/platform#4282 fix(dashmate)!: give Debian packages versions apt can order](https://github.com/dashpay/platform/pull/4282) — 6 unresolved (1 CodeRabbit, 5 bot) · 39 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · 🐢 targets v5.1-dev · areas: dashmate, js-wasm-sdk, system-contracts, github, fallback · Policy: waiting-author
  - Top thread: "_🗄️ Data Integrity &amp; Integration_ \| _🟡 Minor_ \| _⚡ Quick win_" — 39 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/platform#3936 chore(drive-abci): update to nested address in SML](https://github.com/dashpay/platform/pull/3936) — 2 unresolved (2 bot) · 113 days stale · ⚠ merge conflict · 📝 draft · 🐢 targets v5.1-dev · areas: rs-drive-abci, rust-sdk, fallback · Policy: draft
  - Top thread: "🟡 Suggestion: V0 reverse synthesizes \`addresses\` from a Core-22 port, masking subsequent legacy port-change diffs after …" — 113 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#4283 fix(dashmate): keep the node up when an image pull fails, and stop reporting success when it did not](https://github.com/dashpay/platform/pull/4283) — 2 unresolved (2 bot) · 33 days stale · ⚠ merge conflict · ✋ changes requested · 🐢 targets v5.1-dev · areas: dashmate · Policy: waiting-author
  - Top thread: "🟡 Suggestion: providers.js sanitizer misses U+061C (Arabic letter mark) that sanitizeRemoteText covers**" — 33 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/platform#4660 feat(sdk): expose the document erase and history lifecycle on mobile and FFI](https://github.com/dashpay/platform/pull/4660) — 2 unresolved (2 bot) · 17 days stale · ✋ changes requested · 🔴 CI failing · 🐢 targets keep-history-lifecycle · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, rust-sdk-ffi, kotlin-sdk, fallback
  - Top thread: "🟡 Suggestion: Validate history selector discriminant at the C boundary**" — 17 days old
- [dashpay/platform#4657 feat(platform)!: delete and erase lifecycle for keep-history documents](https://github.com/dashpay/platform/pull/4657) — 2 unresolved (2 bot) · 17 days stale · ✋ changes requested · 🔴 CI failing · 🐢 targets keep-history-storage-v2 · areas: rs-drive, rs-drive-abci, dpp, rust-sdk, js-wasm-sdk, fallback
  - Top thread: "🟡 Suggestion: Deduplicate lifecycle containers at the Drive batch boundary**" — 17 days old
- [dashpay/platform#4243 fix(sdk)!: complete encrypted txMetadata parity](https://github.com/dashpay/platform/pull/4243) — ⚠ merge conflict · 📝 draft · 🐢 targets v5.1-dev · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk, github, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#4285 feat(platform-wallet)!: invitation links are AppsFlyer applinks only](https://github.com/dashpay/platform/pull/4285) — ⚠ merge conflict · 🔴 CI failing · 📝 draft · 🐢 targets v5.1-dev · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#4652 feat(drive)!: preserve composite document history across protocol activation](https://github.com/dashpay/platform/pull/4652) — ⚠ merge conflict · ✋ changes requested · 🐢 targets v5.1-dev · areas: rs-drive, rs-drive-abci, rust-sdk, js-wasm-sdk, fallback · Policy: waiting-author
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: Proceeded without coderabbitai: no review within the configured window

<a id="shumkov-clean"></a>
#### Clean (6)
- [dashpay/platform#5341 fix(platform): backfill historical credit nullifiers at pv14 activation](https://github.com/dashpay/platform/pull/5341) — areas: rs-drive, rs-drive-abci, fallback · Policy: waiting-author
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/platform#5343 test(drive-abci): verify query errors over gRPC and original proofs](https://github.com/dashpay/platform/pull/5343) — areas: rs-drive-abci, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed da55fa317f53e9196dd79a8e0f5fea2413837330
- [dashpay/platform#5349 fix(drive-abci): classify unsupported offsets and composite limits](https://github.com/dashpay/platform/pull/5349) — areas: rs-drive-abci, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 90200c2fb795e81853b48c565bd555e950deafb3
- [dashpay/platform#5351 fix(dapi): classify malformed protobuf requests as invalid arguments](https://github.com/dashpay/platform/pull/5351) — areas: rs-drive-abci, rust-dapi, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 1b34e746be0a0e148d16db7e3e1b63b1a4f9a647
- [dashpay/platform#5356 fix(sdk): bind voting proof queries to the default limit](https://github.com/dashpay/platform/pull/5356) — areas: rs-drive, rs-drive-abci, rust-sdk, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 7f9a95470aaaebbadf2132e80545e72082e921d6
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/platform#5362 docs: correct nonce validation wording](https://github.com/dashpay/platform/pull/5362) — areas: fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 13a6be83c5508534cb3eac5e3ebcda3e83fa3aa6
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return

<a id="shumkov-ready-for-review"></a>
#### Ready for Review (2)
- [dashpay/platform#5172 ci: stage reviewed AMD64 recipe for candidate validation](https://github.com/dashpay/platform/pull/5172) — by @ktechmidas · areas: github · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 2c25cdf6c30ac2fc256390a6faef55e5464241f5
- [dashpay/platform#5203 fix(release): install verified cargo-binstall binary directly](https://github.com/dashpay/platform/pull/5203) — by @ktechmidas · areas: fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed f8fee0b29313e97840ce6a67fa335e90b5da7301

<a id="thepastaclaw"></a>
### @thepastaclaw
<a id="thepastaclaw-open"></a>
#### Open (8)
- [dashpay/platform#3096 feat(sdk): add client-side validation to state transition construction methods](https://github.com/dashpay/platform/pull/3096) — 4 unresolved (2 CodeRabbit, 2 human) · 234 days stale · ⚠ merge conflict · 🐢 targets v5.1-dev · areas: rs-drive-abci, dpp, js-wasm-sdk, fallback · Policy: waiting-author
  - Top thread: "_⚠️ Potential issue_ \| _🟡 Minor_" — 231 days old
  - Blocker: thepastaclaw left review threads unresolved; resolve them
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/rust-dashcore#901 fix(key-wallet): discover Coinbase/AssetUnlock outputs for all fund-bearing accounts](https://github.com/dashpay/rust-dashcore/pull/901) — 2 unresolved (2 CodeRabbit) · 65 days stale · ⚠ merge conflict · ✋ changes requested · 📝 draft · areas: dash-spv, key-wallet · Policy: draft
  - Top thread: "_🎯 Functional Correctness_ \| _🟡 Minor_ \| _⚡ Quick win_" — 65 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/dash-evo-tool#859 perf: detect system theme off UI thread](https://github.com/dashpay/dash-evo-tool/pull/859) — 2 unresolved (1 CodeRabbit, 1 human) · 16 days stale · ⚠ merge conflict · areas: fallback · Policy: waiting-author
  - Top thread: "we don't really want to grow the ThemeState. Is there no simpler solution? Also check architecture documents and put the…" — 16 days old
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/dash-evo-tool#853 fix: show parsed platform-address transitions](https://github.com/dashpay/dash-evo-tool/pull/853) — 1 unresolved (1 CodeRabbit) · 16 days stale · 🔴 CI failing · areas: fallback · Policy: waiting-author
  - Top thread: "_🎯 Functional Correctness_ \| _🟠 Major_ \| _⚡ Quick win_" — 16 days old
  - Blocker: coderabbitai left review threads unresolved; resolve them
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#4298 fix(dashmate): load ZeroSSL config in force mode](https://github.com/dashpay/platform/pull/4298) — ⚠ merge conflict · 🐢 targets v5.1-dev · areas: dashmate · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/rust-dashcore#749 fix(ffi): free transaction byte buffers correctly](https://github.com/dashpay/rust-dashcore/pull/749) — ⚠ merge conflict · 🔴 CI failing · areas: fallback · Policy: waiting-build
  - Blocker: The build must pass before a human is asked
- [dashpay/rust-dashcore#752 fix(ffi): validate account seed lengths](https://github.com/dashpay/rust-dashcore/pull/752) — ⚠ merge conflict · areas: fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
- [dashpay/rust-dashcore#904 fix(dash-spv): harden Windows dashd_sync startup wallet load](https://github.com/dashpay/rust-dashcore/pull/904) — ⚠ merge conflict · 📝 draft · areas: dash-spv · Policy: draft
  - Blocker: Draft PR does not occupy a review slot

<a id="thepastaclaw-needs-action"></a>
#### Needs action (4)
- [dashpay/dash-evo-tool#859 perf: detect system theme off UI thread](https://github.com/dashpay/dash-evo-tool/pull/859) — 2 unresolved (1 CodeRabbit, 1 human) · 16 days stale · ⚠ merge conflict · areas: fallback · Policy: waiting-author
  - Top thread: "we don't really want to grow the ThemeState. Is there no simpler solution? Also check architecture documents and put the…" — 16 days old
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/dash-evo-tool#853 fix: show parsed platform-address transitions](https://github.com/dashpay/dash-evo-tool/pull/853) — 1 unresolved (1 CodeRabbit) · 16 days stale · 🔴 CI failing · areas: fallback · Policy: waiting-author
  - Top thread: "_🎯 Functional Correctness_ \| _🟠 Major_ \| _⚡ Quick win_" — 16 days old
  - Blocker: coderabbitai left review threads unresolved; resolve them
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/rust-dashcore#749 fix(ffi): free transaction byte buffers correctly](https://github.com/dashpay/rust-dashcore/pull/749) — ⚠ merge conflict · 🔴 CI failing · areas: fallback · Policy: waiting-build
  - Blocker: The build must pass before a human is asked
- [dashpay/rust-dashcore#752 fix(ffi): validate account seed lengths](https://github.com/dashpay/rust-dashcore/pull/752) — ⚠ merge conflict · areas: fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required

<a id="thepastaclaw-ready-for-human"></a>
#### Ready for human (2)
- [dashpay/platform#4298 fix(dashmate): load ZeroSSL config in force mode](https://github.com/dashpay/platform/pull/4298) — ⚠ merge conflict · 🐢 targets v5.1-dev · areas: dashmate · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/rust-dashcore#752 fix(ffi): validate account seed lengths](https://github.com/dashpay/rust-dashcore/pull/752) — ⚠ merge conflict · areas: fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required

<a id="thepastaclaw-unresolved-comments"></a>
#### Unresolved Comments (2)
- [dashpay/dash-evo-tool#859 perf: detect system theme off UI thread](https://github.com/dashpay/dash-evo-tool/pull/859) — 2 unresolved (1 CodeRabbit, 1 human) · 16 days stale · ⚠ merge conflict · areas: fallback · Policy: waiting-author
  - Top thread: "we don't really want to grow the ThemeState. Is there no simpler solution? Also check architecture documents and put the…" — 16 days old
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/dash-evo-tool#853 fix: show parsed platform-address transitions](https://github.com/dashpay/dash-evo-tool/pull/853) — 1 unresolved (1 CodeRabbit) · 16 days stale · 🔴 CI failing · areas: fallback · Policy: waiting-author
  - Top thread: "_🎯 Functional Correctness_ \| _🟠 Major_ \| _⚡ Quick win_" — 16 days old
  - Blocker: coderabbitai left review threads unresolved; resolve them
  - Blocker: thepastaclaw left review threads unresolved; resolve them

<a id="thepastaclaw-ci-failing"></a>
#### CI Failing (1)
- [dashpay/rust-dashcore#749 fix(ffi): free transaction byte buffers correctly](https://github.com/dashpay/rust-dashcore/pull/749) — ⚠ merge conflict · 🔴 CI failing · areas: fallback · Policy: waiting-build
  - Blocker: The build must pass before a human is asked

<a id="thepastaclaw-draft"></a>
#### Draft (2)
- [dashpay/rust-dashcore#901 fix(key-wallet): discover Coinbase/AssetUnlock outputs for all fund-bearing accounts](https://github.com/dashpay/rust-dashcore/pull/901) — 2 unresolved (2 CodeRabbit) · 65 days stale · ⚠ merge conflict · ✋ changes requested · 📝 draft · areas: dash-spv, key-wallet · Policy: draft
  - Top thread: "_🎯 Functional Correctness_ \| _🟡 Minor_ \| _⚡ Quick win_" — 65 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#904 fix(dash-spv): harden Windows dashd_sync startup wallet load](https://github.com/dashpay/rust-dashcore/pull/904) — ⚠ merge conflict · 📝 draft · areas: dash-spv · Policy: draft
  - Blocker: Draft PR does not occupy a review slot

<a id="thepastaclaw-stale"></a>
#### Stale (2)
- [dashpay/platform#3096 feat(sdk): add client-side validation to state transition construction methods](https://github.com/dashpay/platform/pull/3096) — 4 unresolved (2 CodeRabbit, 2 human) · 234 days stale · ⚠ merge conflict · 🐢 targets v5.1-dev · areas: rs-drive-abci, dpp, js-wasm-sdk, fallback · Policy: waiting-author
  - Top thread: "_⚠️ Potential issue_ \| _🟡 Minor_" — 231 days old
  - Blocker: thepastaclaw left review threads unresolved; resolve them
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/platform#4298 fix(dashmate): load ZeroSSL config in force mode](https://github.com/dashpay/platform/pull/4298) — ⚠ merge conflict · 🐢 targets v5.1-dev · areas: dashmate · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
  - Blocker: Proceeded without coderabbitai: no review within the configured window

<a id="thepastaclaw-clean"></a>
#### Clean (1)
- [dashpay/rust-dashcore#752 fix(ffi): validate account seed lengths](https://github.com/dashpay/rust-dashcore/pull/752) — ⚠ merge conflict · areas: fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required

<a id="lklimek"></a>
### @lklimek
<a id="lklimek-open"></a>
#### Open (15)
- [dashpay/platform#5150 fix(platform-wallet)!: restore Core spending state and repair persisted accounting](https://github.com/dashpay/platform/pull/5150) — 5 unresolved (5 bot) · 8 days stale · ✋ changes requested · 🔴 CI failing · 🐢 targets chore/rust-dashcore-1112-v5.1 · areas: rs-platform-wallet, wallet-storage, swift-sdk
  - Top thread: "🟡 Suggestion: Shared-wallet receipts remain unavailable when foreign inputs are pending**" — 8 days old
- [dashpay/platform#5220 fix(platform-wallet)!: replay recorded transaction history on load for every persister](https://github.com/dashpay/platform/pull/5220) — 4 unresolved (4 bot) · 9 days stale · ⚠ merge conflict · ✋ changes requested · 🐢 targets fix/pr-5126 · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk, fallback
  - Top thread: "🔴 Blocking: Reconcile final conflicts after raw-history restoration**" — 9 days old
- [dashpay/tenderdash#1523 fix(consensus): never extend a relock precommit without ProcessProposal for the round](https://github.com/dashpay/tenderdash/pull/1523) — 1 unresolved (1 bot) · 9 days stale · areas: fallback · Policy: ready-to-merge
  - Top thread: "🟡 Suggestion: Avoid consuming a round-0 timeout in the round-1 assertion**" — 9 days old
  - Blocker: All policy requirements are satisfied
- [dashpay/tenderdash#1527 fix(consensus)!: validate fresh block proposer before prevoting](https://github.com/dashpay/tenderdash/pull/1527) — 1 unresolved (1 bot) · 8 days stale · areas: github, fallback · Policy: ready-for-human
  - Top thread: "🟡 Suggestion: Use a sentinel for fresh-proposal proposer mismatch**" — 8 days old
  - Blocker: Human approval or objection resolution is required
- [dashpay/platform#5307 chore(platform)!: update rust-dashcore (incl secp256k1 0.33)](https://github.com/dashpay/platform/pull/5307) — 1 unresolved (1 bot) · 1 days stale · ⚠ merge conflict · ✋ changes requested · 🐢 targets v5.1-dev · areas: rs-drive-abci, dpp, rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk, rust-sdk, rust-sdk-ffi, dashmate, js-wasm-sdk, kotlin-sdk, fallback · Policy: waiting-bots
  - Top thread: "🔴 Blocking: Enforce the compatible blst minimum for Core-only BLS callers**" — 1 days old
  - Blocker: coderabbitai has not reported for the current head
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#4740 fix(platform-wallet): rescan DashPay contact accounts from the contact request height](https://github.com/dashpay/platform/pull/4740) — 🐢 targets v5.1-dev · areas: rs-platform-wallet, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed f9426a4236d2c9fe58bbffd57a34e9a9d84ddf37
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/platform#5207 fix(wallet): require only atomic tracked-lock writes for reconciliation](https://github.com/dashpay/platform/pull/5207) — ⚠ merge conflict · 🔴 CI failing · 📝 draft · 🐢 targets fix/pr-5126 · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, fallback
- [dashpay/platform#5210 feat(wallet-storage): restore complete Core wallet snapshots from SQLite](https://github.com/dashpay/platform/pull/5210) — 📝 draft · 🐢 targets fix/sqlite-asset-lock-reconciliation · areas: rs-platform-wallet, wallet-storage, swift-sdk, fallback
- [dashpay/platform#5306 feat(platform-wallet)!: persist engine spent claims and restore them across backends](https://github.com/dashpay/platform/pull/5306) — 🔴 CI failing · 📝 draft · 🐢 targets fix/pr-5126 · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk, kotlin-sdk, fallback
- [dashpay/platform#5323 refactor: reject legacy BLS storage formats](https://github.com/dashpay/platform/pull/5323) — ⚠ merge conflict · 📝 draft · 🐢 targets chore/rust-dashcore-1112-v5.1 · areas: rs-drive-abci, dpp, rs-platform-wallet, rust-sdk-ffi, fallback
- [dashpay/rust-dashcore#1106 fix(wallet): correct late inputs during InstantSend backfill](https://github.com/dashpay/rust-dashcore/pull/1106) — areas: dash-spv, key-wallet, key-wallet-manager, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 61099afed724fa3c359e16f47c8533459f7b28e8
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/rust-dashcore#1112 feat(key-wallet): restore externally persisted spent claims](https://github.com/dashpay/rust-dashcore/pull/1112) — 📝 draft · areas: key-wallet, key-wallet-manager, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#1143 fix(key-wallet): sweep a conflict loser's keys-only records too](https://github.com/dashpay/rust-dashcore/pull/1143) — 📝 draft · areas: key-wallet · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/tenderdash#1513 fix(service)!: share worker lifecycle for consensus shutdown](https://github.com/dashpay/tenderdash/pull/1513) — ⚠ merge conflict · 🔴 CI failing · areas: fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed b8a6bebd2654465a996515b6303f124052d8a860
- [dashpay/tenderdash#1515 fix(service)!: join background work before shutdown completes](https://github.com/dashpay/tenderdash/pull/1515) — 🔴 CI failing · 📝 draft · areas: fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot

<a id="lklimek-needs-action"></a>
#### Needs action (3)
- [dashpay/tenderdash#1523 fix(consensus): never extend a relock precommit without ProcessProposal for the round](https://github.com/dashpay/tenderdash/pull/1523) — 1 unresolved (1 bot) · 9 days stale · areas: fallback · Policy: ready-to-merge
  - Top thread: "🟡 Suggestion: Avoid consuming a round-0 timeout in the round-1 assertion**" — 9 days old
  - Blocker: All policy requirements are satisfied
- [dashpay/tenderdash#1527 fix(consensus)!: validate fresh block proposer before prevoting](https://github.com/dashpay/tenderdash/pull/1527) — 1 unresolved (1 bot) · 8 days stale · areas: github, fallback · Policy: ready-for-human
  - Top thread: "🟡 Suggestion: Use a sentinel for fresh-proposal proposer mismatch**" — 8 days old
  - Blocker: Human approval or objection resolution is required
- [dashpay/tenderdash#1513 fix(service)!: share worker lifecycle for consensus shutdown](https://github.com/dashpay/tenderdash/pull/1513) — ⚠ merge conflict · 🔴 CI failing · areas: fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed b8a6bebd2654465a996515b6303f124052d8a860

<a id="lklimek-ready-for-human"></a>
#### Ready for human (2)
- [dashpay/tenderdash#1523 fix(consensus): never extend a relock precommit without ProcessProposal for the round](https://github.com/dashpay/tenderdash/pull/1523) — 1 unresolved (1 bot) · 9 days stale · areas: fallback · Policy: ready-to-merge
  - Top thread: "🟡 Suggestion: Avoid consuming a round-0 timeout in the round-1 assertion**" — 9 days old
  - Blocker: All policy requirements are satisfied
- [dashpay/tenderdash#1527 fix(consensus)!: validate fresh block proposer before prevoting](https://github.com/dashpay/tenderdash/pull/1527) — 1 unresolved (1 bot) · 8 days stale · areas: github, fallback · Policy: ready-for-human
  - Top thread: "🟡 Suggestion: Use a sentinel for fresh-proposal proposer mismatch**" — 8 days old
  - Blocker: Human approval or objection resolution is required

<a id="lklimek-unresolved-comments"></a>
#### Unresolved Comments (2)
- [dashpay/tenderdash#1523 fix(consensus): never extend a relock precommit without ProcessProposal for the round](https://github.com/dashpay/tenderdash/pull/1523) — 1 unresolved (1 bot) · 9 days stale · areas: fallback · Policy: ready-to-merge
  - Top thread: "🟡 Suggestion: Avoid consuming a round-0 timeout in the round-1 assertion**" — 9 days old
  - Blocker: All policy requirements are satisfied
- [dashpay/tenderdash#1527 fix(consensus)!: validate fresh block proposer before prevoting](https://github.com/dashpay/tenderdash/pull/1527) — 1 unresolved (1 bot) · 8 days stale · areas: github, fallback · Policy: ready-for-human
  - Top thread: "🟡 Suggestion: Use a sentinel for fresh-proposal proposer mismatch**" — 8 days old
  - Blocker: Human approval or objection resolution is required

<a id="lklimek-ci-failing"></a>
#### CI Failing (1)
- [dashpay/tenderdash#1513 fix(service)!: share worker lifecycle for consensus shutdown](https://github.com/dashpay/tenderdash/pull/1513) — ⚠ merge conflict · 🔴 CI failing · areas: fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed b8a6bebd2654465a996515b6303f124052d8a860

<a id="lklimek-draft"></a>
#### Draft (3)
- [dashpay/rust-dashcore#1112 feat(key-wallet): restore externally persisted spent claims](https://github.com/dashpay/rust-dashcore/pull/1112) — 📝 draft · areas: key-wallet, key-wallet-manager, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#1143 fix(key-wallet): sweep a conflict loser's keys-only records too](https://github.com/dashpay/rust-dashcore/pull/1143) — 📝 draft · areas: key-wallet · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/tenderdash#1515 fix(service)!: join background work before shutdown completes](https://github.com/dashpay/tenderdash/pull/1515) — 🔴 CI failing · 📝 draft · areas: fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot

<a id="lklimek-stale"></a>
#### Stale (8)
- [dashpay/platform#5150 fix(platform-wallet)!: restore Core spending state and repair persisted accounting](https://github.com/dashpay/platform/pull/5150) — 5 unresolved (5 bot) · 8 days stale · ✋ changes requested · 🔴 CI failing · 🐢 targets chore/rust-dashcore-1112-v5.1 · areas: rs-platform-wallet, wallet-storage, swift-sdk
  - Top thread: "🟡 Suggestion: Shared-wallet receipts remain unavailable when foreign inputs are pending**" — 8 days old
- [dashpay/platform#5220 fix(platform-wallet)!: replay recorded transaction history on load for every persister](https://github.com/dashpay/platform/pull/5220) — 4 unresolved (4 bot) · 9 days stale · ⚠ merge conflict · ✋ changes requested · 🐢 targets fix/pr-5126 · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk, fallback
  - Top thread: "🔴 Blocking: Reconcile final conflicts after raw-history restoration**" — 9 days old
- [dashpay/platform#5307 chore(platform)!: update rust-dashcore (incl secp256k1 0.33)](https://github.com/dashpay/platform/pull/5307) — 1 unresolved (1 bot) · 1 days stale · ⚠ merge conflict · ✋ changes requested · 🐢 targets v5.1-dev · areas: rs-drive-abci, dpp, rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk, rust-sdk, rust-sdk-ffi, dashmate, js-wasm-sdk, kotlin-sdk, fallback · Policy: waiting-bots
  - Top thread: "🔴 Blocking: Enforce the compatible blst minimum for Core-only BLS callers**" — 1 days old
  - Blocker: coderabbitai has not reported for the current head
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#4740 fix(platform-wallet): rescan DashPay contact accounts from the contact request height](https://github.com/dashpay/platform/pull/4740) — 🐢 targets v5.1-dev · areas: rs-platform-wallet, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed f9426a4236d2c9fe58bbffd57a34e9a9d84ddf37
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/platform#5207 fix(wallet): require only atomic tracked-lock writes for reconciliation](https://github.com/dashpay/platform/pull/5207) — ⚠ merge conflict · 🔴 CI failing · 📝 draft · 🐢 targets fix/pr-5126 · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, fallback
- [dashpay/platform#5210 feat(wallet-storage): restore complete Core wallet snapshots from SQLite](https://github.com/dashpay/platform/pull/5210) — 📝 draft · 🐢 targets fix/sqlite-asset-lock-reconciliation · areas: rs-platform-wallet, wallet-storage, swift-sdk, fallback
- [dashpay/platform#5306 feat(platform-wallet)!: persist engine spent claims and restore them across backends](https://github.com/dashpay/platform/pull/5306) — 🔴 CI failing · 📝 draft · 🐢 targets fix/pr-5126 · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk, kotlin-sdk, fallback
- [dashpay/platform#5323 refactor: reject legacy BLS storage formats](https://github.com/dashpay/platform/pull/5323) — ⚠ merge conflict · 📝 draft · 🐢 targets chore/rust-dashcore-1112-v5.1 · areas: rs-drive-abci, dpp, rs-platform-wallet, rust-sdk-ffi, fallback

<a id="lklimek-clean"></a>
#### Clean (1)
- [dashpay/rust-dashcore#1106 fix(wallet): correct late inputs during InstantSend backfill](https://github.com/dashpay/rust-dashcore/pull/1106) — areas: dash-spv, key-wallet, key-wallet-manager, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 61099afed724fa3c359e16f47c8533459f7b28e8
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return

<a id="lklimek-ready-for-review"></a>
#### Ready for Review (6)
- [dashpay/dash-evo-tool#1066 fix(dpns): stop re-reading finished name contests on every refresh](https://github.com/dashpay/dash-evo-tool/pull/1066) — by @Claudius-Maginificent · areas: fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/dash-evo-tool#1069 docs: align changelog, kv-keys and stories with shipped voting behaviour](https://github.com/dashpay/dash-evo-tool/pull/1069) — by @Claudius-Maginificent · areas: fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/dash-evo-tool#1070 fix(voting): stop the old-schedule notice for votes a pre-release build already cast](https://github.com/dashpay/dash-evo-tool/pull/1070) — by @Claudius-Maginificent · areas: fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/dash-evo-tool#1072 fix(tools): keep a Platform info fetch and its result through background refreshes](https://github.com/dashpay/dash-evo-tool/pull/1072) — by @Claudius-Maginificent · areas: fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/platform#5351 fix(dapi): classify malformed protobuf requests as invalid arguments](https://github.com/dashpay/platform/pull/5351) — by @shumkov · areas: rs-drive-abci, rust-dapi, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 1b34e746be0a0e148d16db7e3e1b63b1a4f9a647
- [dashpay/platform#5356 fix(sdk): bind voting proof queries to the default limit](https://github.com/dashpay/platform/pull/5356) — by @shumkov · areas: rs-drive, rs-drive-abci, rust-sdk, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 7f9a95470aaaebbadf2132e80545e72082e921d6
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return

<a id="zocolini"></a>
### @ZocoLini
<a id="zocolini-open"></a>
#### Open (10)
- [dashpay/rust-dashcore#375 refactor(dash-spv): checkpoint rewrite](https://github.com/dashpay/rust-dashcore/pull/375) — 2 unresolved (2 CodeRabbit) · 261 days stale · ⚠ merge conflict · 📝 draft · areas: dash-spv, fallback · Policy: draft
  - Top thread: "_⚠️ Potential issue_ \| _🟡 Minor_" — 261 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#1104 feat(wallet)!: wallet truncate above](https://github.com/dashpay/rust-dashcore/pull/1104) — 2 unresolved (2 CodeRabbit) · 4 days stale · ✋ changes requested · areas: dash-spv, key-wallet, key-wallet-manager, fallback · Policy: waiting-author
  - Top thread: "🗄️ Data Integrity &amp; Integration** \| **🟠 Major** \| **🏗️ Heavy lift**" — 4 days old
  - Blocker: coderabbitai left review threads unresolved; resolve them
- [dashpay/rust-dashcore#1149 fix(dash)!: restore the serde formats of \`PlatformNodeId\`, \`BLSPublicKey\` and pre-v4 \`CoinbasePayload\`](https://github.com/dashpay/rust-dashcore/pull/1149) — 3 unresolved (3 human) · 1 days stale · areas: key-wallet, fallback · Policy: waiting-self-review
  - Top thread: "This should probably be located in \`dashcore-crypto\` since it's an expression of an API contract for a cryptographic typ…" — 1 days old
  - Blocker: Author must post /self-reviewed 8fe0a38170059742ff902acc29a696b72490903c
- [dashpay/rust-dashcore#902 refactor(dash-spv): network manager refactor and sync pipelines optimized ](https://github.com/dashpay/rust-dashcore/pull/902) — ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · 📝 draft · areas: dash-spv, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#921 fix(dash-spv): sync backfill](https://github.com/dashpay/rust-dashcore/pull/921) — ⚠ merge conflict · 📝 draft · areas: dash-spv, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#1081 feat(dash-spv): restore the validated rotation cycles from the replayed engine](https://github.com/dashpay/rust-dashcore/pull/1081) — ⚠ merge conflict · 📝 draft · areas: dash-spv · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#1136 feat(dash)!: decode unknown special transaction types as raw payloads](https://github.com/dashpay/rust-dashcore/pull/1136) — areas: key-wallet, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 249ef12b0740cfca9947d0e823c4b9fd334b7379
- [dashpay/rust-dashcore#1138 feat(dash)!: decode and encode ProUpServTx and ProUpRegTx version 3](https://github.com/dashpay/rust-dashcore/pull/1138) — ⚠ merge conflict · ✋ changes requested · areas: key-wallet, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed b409ba5c85dec0fbd033fb9d0a5b210f81fc0ad5
- [dashpay/rust-dashcore#1147 fix(rpc-client): keep request arguments and results out of the logs](https://github.com/dashpay/rust-dashcore/pull/1147) — areas: fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 39048d217ecd02387cfd89ac0a79e45d210abc51
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/rust-dashcore#1148 fix(rpc-client): send the ChainLock block hash in display order to \`submitchainlock\`](https://github.com/dashpay/rust-dashcore/pull/1148) — areas: fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 9240f4599b47a0b3936bbba06df7c99087f5e103

<a id="zocolini-needs-action"></a>
#### Needs action (3)
- [dashpay/rust-dashcore#1104 feat(wallet)!: wallet truncate above](https://github.com/dashpay/rust-dashcore/pull/1104) — 2 unresolved (2 CodeRabbit) · 4 days stale · ✋ changes requested · areas: dash-spv, key-wallet, key-wallet-manager, fallback · Policy: waiting-author
  - Top thread: "🗄️ Data Integrity &amp; Integration** \| **🟠 Major** \| **🏗️ Heavy lift**" — 4 days old
  - Blocker: coderabbitai left review threads unresolved; resolve them
- [dashpay/rust-dashcore#1149 fix(dash)!: restore the serde formats of \`PlatformNodeId\`, \`BLSPublicKey\` and pre-v4 \`CoinbasePayload\`](https://github.com/dashpay/rust-dashcore/pull/1149) — 3 unresolved (3 human) · 1 days stale · areas: key-wallet, fallback · Policy: waiting-self-review
  - Top thread: "This should probably be located in \`dashcore-crypto\` since it's an expression of an API contract for a cryptographic typ…" — 1 days old
  - Blocker: Author must post /self-reviewed 8fe0a38170059742ff902acc29a696b72490903c
- [dashpay/rust-dashcore#1138 feat(dash)!: decode and encode ProUpServTx and ProUpRegTx version 3](https://github.com/dashpay/rust-dashcore/pull/1138) — ⚠ merge conflict · ✋ changes requested · areas: key-wallet, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed b409ba5c85dec0fbd033fb9d0a5b210f81fc0ad5

<a id="zocolini-unresolved-comments"></a>
#### Unresolved Comments (2)
- [dashpay/rust-dashcore#1104 feat(wallet)!: wallet truncate above](https://github.com/dashpay/rust-dashcore/pull/1104) — 2 unresolved (2 CodeRabbit) · 4 days stale · ✋ changes requested · areas: dash-spv, key-wallet, key-wallet-manager, fallback · Policy: waiting-author
  - Top thread: "🗄️ Data Integrity &amp; Integration** \| **🟠 Major** \| **🏗️ Heavy lift**" — 4 days old
  - Blocker: coderabbitai left review threads unresolved; resolve them
- [dashpay/rust-dashcore#1149 fix(dash)!: restore the serde formats of \`PlatformNodeId\`, \`BLSPublicKey\` and pre-v4 \`CoinbasePayload\`](https://github.com/dashpay/rust-dashcore/pull/1149) — 3 unresolved (3 human) · 1 days stale · areas: key-wallet, fallback · Policy: waiting-self-review
  - Top thread: "This should probably be located in \`dashcore-crypto\` since it's an expression of an API contract for a cryptographic typ…" — 1 days old
  - Blocker: Author must post /self-reviewed 8fe0a38170059742ff902acc29a696b72490903c

<a id="zocolini-changes-requested"></a>
#### Changes Requested (1)
- [dashpay/rust-dashcore#1138 feat(dash)!: decode and encode ProUpServTx and ProUpRegTx version 3](https://github.com/dashpay/rust-dashcore/pull/1138) — ⚠ merge conflict · ✋ changes requested · areas: key-wallet, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed b409ba5c85dec0fbd033fb9d0a5b210f81fc0ad5

<a id="zocolini-draft"></a>
#### Draft (4)
- [dashpay/rust-dashcore#375 refactor(dash-spv): checkpoint rewrite](https://github.com/dashpay/rust-dashcore/pull/375) — 2 unresolved (2 CodeRabbit) · 261 days stale · ⚠ merge conflict · 📝 draft · areas: dash-spv, fallback · Policy: draft
  - Top thread: "_⚠️ Potential issue_ \| _🟡 Minor_" — 261 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#902 refactor(dash-spv): network manager refactor and sync pipelines optimized ](https://github.com/dashpay/rust-dashcore/pull/902) — ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · 📝 draft · areas: dash-spv, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#921 fix(dash-spv): sync backfill](https://github.com/dashpay/rust-dashcore/pull/921) — ⚠ merge conflict · 📝 draft · areas: dash-spv, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#1081 feat(dash-spv): restore the validated rotation cycles from the replayed engine](https://github.com/dashpay/rust-dashcore/pull/1081) — ⚠ merge conflict · 📝 draft · areas: dash-spv · Policy: draft
  - Blocker: Draft PR does not occupy a review slot

<a id="zocolini-clean"></a>
#### Clean (3)
- [dashpay/rust-dashcore#1136 feat(dash)!: decode unknown special transaction types as raw payloads](https://github.com/dashpay/rust-dashcore/pull/1136) — areas: key-wallet, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 249ef12b0740cfca9947d0e823c4b9fd334b7379
- [dashpay/rust-dashcore#1147 fix(rpc-client): keep request arguments and results out of the logs](https://github.com/dashpay/rust-dashcore/pull/1147) — areas: fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 39048d217ecd02387cfd89ac0a79e45d210abc51
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/rust-dashcore#1148 fix(rpc-client): send the ChainLock block hash in display order to \`submitchainlock\`](https://github.com/dashpay/rust-dashcore/pull/1148) — areas: fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 9240f4599b47a0b3936bbba06df7c99087f5e103

<a id="zocolini-ready-for-review"></a>
#### Ready for Review (3)
- [dashpay/rust-dashcore#1106 fix(wallet): correct late inputs during InstantSend backfill](https://github.com/dashpay/rust-dashcore/pull/1106) — by @lklimek · areas: dash-spv, key-wallet, key-wallet-manager, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 61099afed724fa3c359e16f47c8533459f7b28e8
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/rust-dashcore#1142 fix(dash)!: adopt \`dash_num::Arith256\` around \`U256\` implementation, align closer to reference implementation](https://github.com/dashpay/rust-dashcore/pull/1142) — by @kwvg · areas: fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
- [dashpay/rust-dashcore#1144 fix!: reject WIF keys without \`0x01\` flag, reject sighash types &gt;\`0xff\`, maintain symmetry in &#123;sign,recover&#125; compact signatures, drop unused divergent segments](https://github.com/dashpay/rust-dashcore/pull/1144) — by @kwvg · areas: key-wallet, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required

<a id="pastapastapasta"></a>
### @PastaPastaPasta
<a id="pastapastapasta-open"></a>
#### Open (22)
- [dashpay/rust-dashcore#137 fix: integration tests, continued dashificiation](https://github.com/dashpay/rust-dashcore/pull/137) — 2 unresolved (2 CodeRabbit) · 400 days stale · ⚠ merge conflict · 🔴 CI failing · 🐢 targets v0.40-dev, untouched 303 days · areas: fallback
  - Top thread: "_🛠️ Refactor suggestion_" — 400 days old
- [dashpay/platform#4648 feat(drive-abci)!: state sync via ABCI snapshots with reduced platform state (protocol v15)](https://github.com/dashpay/platform/pull/4648) — 8 unresolved (8 bot) · 5 days stale · ✋ changes requested · 🔴 CI failing · 🐢 targets v5.1-dev · areas: rs-drive, rs-drive-abci, dpp, fallback · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Continue the restored target through consensus and a restart**" — 5 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/platform#5288 feat(platform-wallet)!: support DashPay shielded tips with dedicated accounts](https://github.com/dashpay/platform/pull/5288) — 4 unresolved (4 bot) · 5 days stale · 🔴 CI failing · 🐢 targets v5.1-dev · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk, kotlin-sdk, fallback · Policy: waiting-self-review
  - Top thread: "🟡 Suggestion: Restore outstanding tip submissions before enabling another send**" — 5 days old
  - Blocker: Author must post /self-reviewed 74a473f8f46674f504adffd942df79136f279445
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/platform#5290 fix(dashmate): refresh quorum-server seeds and complete node identities](https://github.com/dashpay/platform/pull/5290) — 4 unresolved (1 CodeRabbit, 3 bot) · 4 days stale · 🔴 CI failing · 🐢 targets v5.1-dev · areas: dashmate, github, fallback · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Null seed entry throws TypeError instead of friendly validation error**" — 4 days old
  - Blocker: coderabbitai left review threads unresolved; resolve them
- [dashpay/rust-dashcore#264 feat(dash-spv): Add BIP324 v2 encrypted P2P transport](https://github.com/dashpay/rust-dashcore/pull/264) — 2 unresolved (2 CodeRabbit) · 20 days stale · ⚠ merge conflict · areas: dash-spv, fallback · Policy: waiting-author
  - Top thread: "_📐 Maintainability &amp; Code Quality_ \| _🟡 Minor_ \| _⚡ Quick win_" — 20 days old
  - Blocker: coderabbitai requested changes on this head; dismiss the review or push a fix
- [dashpay/platform#5278 perf(platform-wallet)!: build the Orchard proving key once off the async runtime and prove off tokio workers](https://github.com/dashpay/platform/pull/5278) — 3 unresolved (3 bot) · 5 days stale · 🐢 targets v5.1-dev · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, fallback · Policy: waiting-self-review
  - Top thread: "🟡 Suggestion: Make pre-broadcast note reservations cancellation-safe at the new proof await**" — 5 days old
  - Blocker: Author must post /self-reviewed 797c1aaebc1d0c61adaae20d9cc48e7c4775c44b
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/platform#5280 perf(platform-wallet)!: reuse a recent recorded-anchor set on the shielded spend path](https://github.com/dashpay/platform/pull/5280) — 3 unresolved (3 bot) · 3 days stale · 🐢 targets v5.1-dev · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk · Policy: waiting-self-review
  - Top thread: "🟡 Suggestion: Only the invalid-handle FFI branch is tested**" — 3 days old
  - Blocker: Author must post /self-reviewed c4b553aed3592691cfbd95547a4d0d399daf99b5
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/platform#5160 feat(sdk): route DAPI connections through a SOCKS5 proxy](https://github.com/dashpay/platform/pull/5160) — 2 unresolved (2 bot) · 3 days stale · 🐢 targets v5.1-dev · areas: rust-sdk, fallback · Policy: waiting-self-review
  - Top thread: "🟡 Suggestion: Propagate OS randomness failures instead of panicking during connection establishment**" — 3 days old
  - Blocker: Author must post /self-reviewed 6109d6eab8c864b388189355171ab7062c789deb
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/platform#5169 feat(sdk): let document writes state the action fee agreement](https://github.com/dashpay/platform/pull/5169) — 1 unresolved (1 bot) · 5 days stale · 🔴 CI failing · areas: rust-sdk, js-wasm-sdk, fallback · Policy: ready-for-human
  - Top thread: "🟡 Suggestion: Delete-builder handoff is pinned only by tests CI never executes**" — 5 days old
  - Blocker: Human approval or objection resolution is required
- [dashpay/platform#4965 test(sdk): cargo-fuzz harness over drive-proof-verifier FromProof](https://github.com/dashpay/platform/pull/4965) — 1 unresolved (1 bot) · 3 days stale · 🐢 targets v5.1-dev · areas: github, fallback · Policy: waiting-self-review
  - Top thread: "🟡 Suggestion: Correct the claim that PV13 reaches the V0 proof decoder**" — 3 days old
  - Blocker: Author must post /self-reviewed 503a6aad352976451e87acef9b6169d57e6f7205
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/platform#5277 perf(platform-wallet): don't hold the shielded store lock across sync network I/O](https://github.com/dashpay/platform/pull/5277) — 1 unresolved (1 bot) · 3 days stale · 🐢 targets v5.1-dev · areas: rs-platform-wallet, rs-platform-wallet-ffi · Policy: waiting-self-review
  - Top thread: "🟡 Suggestion: Enforce invalidation at the exposed store mutation boundary**" — 3 days old
  - Blocker: Author must post /self-reviewed 9b7b5615b96d15c235403b37e19decb5ed143cc4
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/grovedb#1007 build(deps)!: use dashpay/orchard with the faster prover](https://github.com/dashpay/grovedb/pull/1007) — 🔴 CI failing · 📝 draft · areas: fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#4633 feat(sdk): add dash-platform-cxx, a thin CXX shell over dash-sdk for C++ embedders](https://github.com/dashpay/platform/pull/4633) — 📝 draft · 🐢 targets feat/dapi-client-socks5-proxy · areas: github, fallback
- [dashpay/platform#4844 feat(sdk)!: key limits, DIP-14 sub-feature derivation and decode-any-kind for DashPay Connect](https://github.com/dashpay/platform/pull/4844) — ⚠ merge conflict · 📝 draft · 🐢 targets chore/rust-dashcore-1112-v5.1 · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk, fallback
- [dashpay/platform#5231 fix(dashmate): use complete DKG membership for safe stops](https://github.com/dashpay/platform/pull/5231) — 🔴 CI failing · 🐢 targets v5.1-dev · areas: dashmate · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 3b540cc2b3e40afb0db75e1eb8d0dc7b64f04fda
  - Blocker: Proceeded without coderabbitai: no review within the configured window
  - Blocker: Proceeded without thepastaclaw: no review within the configured window
- [dashpay/platform#5279 perf(platform-wallet): prove shield bundles while fetching nonces](https://github.com/dashpay/platform/pull/5279) — 📝 draft · 🐢 targets perf/shielded-prover-prepare · areas: dpp, rs-platform-wallet
- [dashpay/platform#5281 build: bump grovedb for the faster Orchard prover](https://github.com/dashpay/platform/pull/5281) — ⚠ merge conflict · 📝 draft · 🐢 targets v5.1-dev · areas: rs-drive-abci, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5282 perf(platform-wallet)!: prove the shield bundle while waiting for the InstantSend lock](https://github.com/dashpay/platform/pull/5282) — 🔴 CI failing · 🐢 targets v5.1-dev · areas: dpp, rs-platform-wallet, rs-platform-wallet-ffi, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 6ca6ab3e2b64e7d5ca4c44c1395d15b6ff8165f2
- [dashpay/platform#5285 feat(platform)!: subscribe to committed state transitions matching document, address, identity, token and contract filters](https://github.com/dashpay/platform/pull/5285) — 📝 draft · 🐢 targets v5.1-dev · areas: rs-drive, rs-drive-abci, dpp, rust-sdk, dashmate, rust-dapi, github, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5289 feat(sdk)!: optional BIP-39 passphrase through the mnemonic resolver and wallet creation](https://github.com/dashpay/platform/pull/5289) — 📝 draft · 🐢 targets v5.1-dev · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, rust-sdk-ffi, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5292 feat(dashmate): state sync configuration for tenderdash and drive snapshots](https://github.com/dashpay/platform/pull/5292) — ⚠ merge conflict · 📝 draft · 🐢 targets v5.1-dev · areas: dashmate · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#959 feat(dash-spv): adaptive gap-limit probe escalation for BIP44 discovery](https://github.com/dashpay/rust-dashcore/pull/959) — ⚠ merge conflict · 🔴 CI failing · areas: dash-spv, key-wallet, key-wallet-manager, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 7bcac9b9d89c2c4947080b2e6a15503e97eac9f4

<a id="pastapastapasta-needs-action"></a>
#### Needs action (3)
- [dashpay/rust-dashcore#264 feat(dash-spv): Add BIP324 v2 encrypted P2P transport](https://github.com/dashpay/rust-dashcore/pull/264) — 2 unresolved (2 CodeRabbit) · 20 days stale · ⚠ merge conflict · areas: dash-spv, fallback · Policy: waiting-author
  - Top thread: "_📐 Maintainability &amp; Code Quality_ \| _🟡 Minor_ \| _⚡ Quick win_" — 20 days old
  - Blocker: coderabbitai requested changes on this head; dismiss the review or push a fix
- [dashpay/platform#5169 feat(sdk): let document writes state the action fee agreement](https://github.com/dashpay/platform/pull/5169) — 1 unresolved (1 bot) · 5 days stale · 🔴 CI failing · areas: rust-sdk, js-wasm-sdk, fallback · Policy: ready-for-human
  - Top thread: "🟡 Suggestion: Delete-builder handoff is pinned only by tests CI never executes**" — 5 days old
  - Blocker: Human approval or objection resolution is required
- [dashpay/rust-dashcore#959 feat(dash-spv): adaptive gap-limit probe escalation for BIP44 discovery](https://github.com/dashpay/rust-dashcore/pull/959) — ⚠ merge conflict · 🔴 CI failing · areas: dash-spv, key-wallet, key-wallet-manager, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 7bcac9b9d89c2c4947080b2e6a15503e97eac9f4

<a id="pastapastapasta-ready-for-human"></a>
#### Ready for human (1)
- [dashpay/platform#5169 feat(sdk): let document writes state the action fee agreement](https://github.com/dashpay/platform/pull/5169) — 1 unresolved (1 bot) · 5 days stale · 🔴 CI failing · areas: rust-sdk, js-wasm-sdk, fallback · Policy: ready-for-human
  - Top thread: "🟡 Suggestion: Delete-builder handoff is pinned only by tests CI never executes**" — 5 days old
  - Blocker: Human approval or objection resolution is required

<a id="pastapastapasta-unresolved-comments"></a>
#### Unresolved Comments (2)
- [dashpay/rust-dashcore#264 feat(dash-spv): Add BIP324 v2 encrypted P2P transport](https://github.com/dashpay/rust-dashcore/pull/264) — 2 unresolved (2 CodeRabbit) · 20 days stale · ⚠ merge conflict · areas: dash-spv, fallback · Policy: waiting-author
  - Top thread: "_📐 Maintainability &amp; Code Quality_ \| _🟡 Minor_ \| _⚡ Quick win_" — 20 days old
  - Blocker: coderabbitai requested changes on this head; dismiss the review or push a fix
- [dashpay/platform#5169 feat(sdk): let document writes state the action fee agreement](https://github.com/dashpay/platform/pull/5169) — 1 unresolved (1 bot) · 5 days stale · 🔴 CI failing · areas: rust-sdk, js-wasm-sdk, fallback · Policy: ready-for-human
  - Top thread: "🟡 Suggestion: Delete-builder handoff is pinned only by tests CI never executes**" — 5 days old
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
#### Stale (18)
- [dashpay/rust-dashcore#137 fix: integration tests, continued dashificiation](https://github.com/dashpay/rust-dashcore/pull/137) — 2 unresolved (2 CodeRabbit) · 400 days stale · ⚠ merge conflict · 🔴 CI failing · 🐢 targets v0.40-dev, untouched 303 days · areas: fallback
  - Top thread: "_🛠️ Refactor suggestion_" — 400 days old
- [dashpay/platform#4648 feat(drive-abci)!: state sync via ABCI snapshots with reduced platform state (protocol v15)](https://github.com/dashpay/platform/pull/4648) — 8 unresolved (8 bot) · 5 days stale · ✋ changes requested · 🔴 CI failing · 🐢 targets v5.1-dev · areas: rs-drive, rs-drive-abci, dpp, fallback · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Continue the restored target through consensus and a restart**" — 5 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/platform#5288 feat(platform-wallet)!: support DashPay shielded tips with dedicated accounts](https://github.com/dashpay/platform/pull/5288) — 4 unresolved (4 bot) · 5 days stale · 🔴 CI failing · 🐢 targets v5.1-dev · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk, kotlin-sdk, fallback · Policy: waiting-self-review
  - Top thread: "🟡 Suggestion: Restore outstanding tip submissions before enabling another send**" — 5 days old
  - Blocker: Author must post /self-reviewed 74a473f8f46674f504adffd942df79136f279445
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/platform#5290 fix(dashmate): refresh quorum-server seeds and complete node identities](https://github.com/dashpay/platform/pull/5290) — 4 unresolved (1 CodeRabbit, 3 bot) · 4 days stale · 🔴 CI failing · 🐢 targets v5.1-dev · areas: dashmate, github, fallback · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Null seed entry throws TypeError instead of friendly validation error**" — 4 days old
  - Blocker: coderabbitai left review threads unresolved; resolve them
- [dashpay/platform#5278 perf(platform-wallet)!: build the Orchard proving key once off the async runtime and prove off tokio workers](https://github.com/dashpay/platform/pull/5278) — 3 unresolved (3 bot) · 5 days stale · 🐢 targets v5.1-dev · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, fallback · Policy: waiting-self-review
  - Top thread: "🟡 Suggestion: Make pre-broadcast note reservations cancellation-safe at the new proof await**" — 5 days old
  - Blocker: Author must post /self-reviewed 797c1aaebc1d0c61adaae20d9cc48e7c4775c44b
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/platform#5280 perf(platform-wallet)!: reuse a recent recorded-anchor set on the shielded spend path](https://github.com/dashpay/platform/pull/5280) — 3 unresolved (3 bot) · 3 days stale · 🐢 targets v5.1-dev · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk · Policy: waiting-self-review
  - Top thread: "🟡 Suggestion: Only the invalid-handle FFI branch is tested**" — 3 days old
  - Blocker: Author must post /self-reviewed c4b553aed3592691cfbd95547a4d0d399daf99b5
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/platform#5160 feat(sdk): route DAPI connections through a SOCKS5 proxy](https://github.com/dashpay/platform/pull/5160) — 2 unresolved (2 bot) · 3 days stale · 🐢 targets v5.1-dev · areas: rust-sdk, fallback · Policy: waiting-self-review
  - Top thread: "🟡 Suggestion: Propagate OS randomness failures instead of panicking during connection establishment**" — 3 days old
  - Blocker: Author must post /self-reviewed 6109d6eab8c864b388189355171ab7062c789deb
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/platform#4965 test(sdk): cargo-fuzz harness over drive-proof-verifier FromProof](https://github.com/dashpay/platform/pull/4965) — 1 unresolved (1 bot) · 3 days stale · 🐢 targets v5.1-dev · areas: github, fallback · Policy: waiting-self-review
  - Top thread: "🟡 Suggestion: Correct the claim that PV13 reaches the V0 proof decoder**" — 3 days old
  - Blocker: Author must post /self-reviewed 503a6aad352976451e87acef9b6169d57e6f7205
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/platform#5277 perf(platform-wallet): don't hold the shielded store lock across sync network I/O](https://github.com/dashpay/platform/pull/5277) — 1 unresolved (1 bot) · 3 days stale · 🐢 targets v5.1-dev · areas: rs-platform-wallet, rs-platform-wallet-ffi · Policy: waiting-self-review
  - Top thread: "🟡 Suggestion: Enforce invalidation at the exposed store mutation boundary**" — 3 days old
  - Blocker: Author must post /self-reviewed 9b7b5615b96d15c235403b37e19decb5ed143cc4
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/platform#4633 feat(sdk): add dash-platform-cxx, a thin CXX shell over dash-sdk for C++ embedders](https://github.com/dashpay/platform/pull/4633) — 📝 draft · 🐢 targets feat/dapi-client-socks5-proxy · areas: github, fallback
- [dashpay/platform#4844 feat(sdk)!: key limits, DIP-14 sub-feature derivation and decode-any-kind for DashPay Connect](https://github.com/dashpay/platform/pull/4844) — ⚠ merge conflict · 📝 draft · 🐢 targets chore/rust-dashcore-1112-v5.1 · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk, fallback
- [dashpay/platform#5231 fix(dashmate): use complete DKG membership for safe stops](https://github.com/dashpay/platform/pull/5231) — 🔴 CI failing · 🐢 targets v5.1-dev · areas: dashmate · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 3b540cc2b3e40afb0db75e1eb8d0dc7b64f04fda
  - Blocker: Proceeded without coderabbitai: no review within the configured window
  - Blocker: Proceeded without thepastaclaw: no review within the configured window
- [dashpay/platform#5279 perf(platform-wallet): prove shield bundles while fetching nonces](https://github.com/dashpay/platform/pull/5279) — 📝 draft · 🐢 targets perf/shielded-prover-prepare · areas: dpp, rs-platform-wallet
- [dashpay/platform#5281 build: bump grovedb for the faster Orchard prover](https://github.com/dashpay/platform/pull/5281) — ⚠ merge conflict · 📝 draft · 🐢 targets v5.1-dev · areas: rs-drive-abci, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5282 perf(platform-wallet)!: prove the shield bundle while waiting for the InstantSend lock](https://github.com/dashpay/platform/pull/5282) — 🔴 CI failing · 🐢 targets v5.1-dev · areas: dpp, rs-platform-wallet, rs-platform-wallet-ffi, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 6ca6ab3e2b64e7d5ca4c44c1395d15b6ff8165f2
- [dashpay/platform#5285 feat(platform)!: subscribe to committed state transitions matching document, address, identity, token and contract filters](https://github.com/dashpay/platform/pull/5285) — 📝 draft · 🐢 targets v5.1-dev · areas: rs-drive, rs-drive-abci, dpp, rust-sdk, dashmate, rust-dapi, github, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5289 feat(sdk)!: optional BIP-39 passphrase through the mnemonic resolver and wallet creation](https://github.com/dashpay/platform/pull/5289) — 📝 draft · 🐢 targets v5.1-dev · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, rust-sdk-ffi, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5292 feat(dashmate): state sync configuration for tenderdash and drive snapshots](https://github.com/dashpay/platform/pull/5292) — ⚠ merge conflict · 📝 draft · 🐢 targets v5.1-dev · areas: dashmate · Policy: draft
  - Blocker: Draft PR does not occupy a review slot

<a id="romchornyi"></a>
### @romchornyi
<a id="romchornyi-open"></a>
#### Open (5)
- [dashpay/platform#4595 fix(swift-sdk): make a changeset round linear without giving up atomicity](https://github.com/dashpay/platform/pull/4595) — 2 unresolved (2 bot) · 33 days stale · ⚠ merge conflict · ✋ changes requested · areas: swift-sdk · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Add round-level regression coverage for registry bookkeeping**" — 33 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#4992 feat(swift-sdk): durable wallet presence marker readable while the device is locked](https://github.com/dashpay/platform/pull/4992) — 🐢 targets v5.1-dev · areas: swift-sdk · Policy: ready-to-merge
  - Blocker: All policy requirements are satisfied
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/platform#5258 feat(platform-wallet)!: resolve a send with an unknown broadcast outcome and spend only final coins](https://github.com/dashpay/platform/pull/5258) — 🔴 CI failing · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, rust-sdk, kotlin-sdk · Policy: waiting-author
  - Blocker: thepastaclaw left review threads unresolved; resolve them
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/platform#5309 fix(sdk): treat AlreadyExists on a retried broadcast as submitted](https://github.com/dashpay/platform/pull/5309) — 📝 draft · ⚠ routing unavailable: no changed-file evidence · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#1105 fix(key-wallet): recover CoinJoin coins stranded past the gap limit by DashSync](https://github.com/dashpay/rust-dashcore/pull/1105) — ✋ changes requested · 🔴 CI failing · areas: key-wallet · Policy: waiting-author
  - Blocker: Author response is required after the latest human objection

<a id="romchornyi-needs-action"></a>
#### Needs action (2)
- [dashpay/platform#4595 fix(swift-sdk): make a changeset round linear without giving up atomicity](https://github.com/dashpay/platform/pull/4595) — 2 unresolved (2 bot) · 33 days stale · ⚠ merge conflict · ✋ changes requested · areas: swift-sdk · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Add round-level regression coverage for registry bookkeeping**" — 33 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/rust-dashcore#1105 fix(key-wallet): recover CoinJoin coins stranded past the gap limit by DashSync](https://github.com/dashpay/rust-dashcore/pull/1105) — ✋ changes requested · 🔴 CI failing · areas: key-wallet · Policy: waiting-author
  - Blocker: Author response is required after the latest human objection

<a id="romchornyi-ready-for-human"></a>
#### Ready for human (1)
- [dashpay/platform#4992 feat(swift-sdk): durable wallet presence marker readable while the device is locked](https://github.com/dashpay/platform/pull/4992) — 🐢 targets v5.1-dev · areas: swift-sdk · Policy: ready-to-merge
  - Blocker: All policy requirements are satisfied
  - Blocker: Proceeded without coderabbitai: no review within the configured window

<a id="romchornyi-unresolved-comments"></a>
#### Unresolved Comments (1)
- [dashpay/platform#4595 fix(swift-sdk): make a changeset round linear without giving up atomicity](https://github.com/dashpay/platform/pull/4595) — 2 unresolved (2 bot) · 33 days stale · ⚠ merge conflict · ✋ changes requested · areas: swift-sdk · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Add round-level regression coverage for registry bookkeeping**" — 33 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them

<a id="romchornyi-changes-requested"></a>
#### Changes Requested (1)
- [dashpay/rust-dashcore#1105 fix(key-wallet): recover CoinJoin coins stranded past the gap limit by DashSync](https://github.com/dashpay/rust-dashcore/pull/1105) — ✋ changes requested · 🔴 CI failing · areas: key-wallet · Policy: waiting-author
  - Blocker: Author response is required after the latest human objection

<a id="romchornyi-ci-failing"></a>
#### CI Failing (1)
- [dashpay/platform#5258 feat(platform-wallet)!: resolve a send with an unknown broadcast outcome and spend only final coins](https://github.com/dashpay/platform/pull/5258) — 🔴 CI failing · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, rust-sdk, kotlin-sdk · Policy: waiting-author
  - Blocker: thepastaclaw left review threads unresolved; resolve them
  - Blocker: Proceeded without coderabbitai: no review within the configured window

<a id="romchornyi-draft"></a>
#### Draft (1)
- [dashpay/platform#5309 fix(sdk): treat AlreadyExists on a retried broadcast as submitted](https://github.com/dashpay/platform/pull/5309) — 📝 draft · ⚠ routing unavailable: no changed-file evidence · Policy: draft
  - Blocker: Draft PR does not occupy a review slot

<a id="romchornyi-stale"></a>
#### Stale (1)
- [dashpay/platform#4992 feat(swift-sdk): durable wallet presence marker readable while the device is locked](https://github.com/dashpay/platform/pull/4992) — 🐢 targets v5.1-dev · areas: swift-sdk · Policy: ready-to-merge
  - Blocker: All policy requirements are satisfied
  - Blocker: Proceeded without coderabbitai: no review within the configured window

<a id="llbartekll"></a>
### @llbartekll
<a id="llbartekll-open"></a>
#### Open (3)
- [dashpay/platform#3560 test(swift-sdk): add testnet identity-discovery UI test](https://github.com/dashpay/platform/pull/3560) — 1 unresolved (1 bot) · 131 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · areas: swift-sdk, github · Policy: waiting-author
  - Top thread: "🔴 Blocking: Carried-forward prior finding (STILL VALID): Docker-setup toggle leaves the cached regtest wallet manager bo…" — 131 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/rust-dashcore#819 feat(key-wallet): CoinJoin sweep + gap-limit recovery; TransactionBuilder drain mode](https://github.com/dashpay/rust-dashcore/pull/819) — ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · areas: key-wallet · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed e406d1cbb86a03503fa41a9d796f25c0bacc3e12
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/rust-dashcore#910 fix(dash-spv): re-anchor incompatible shared storage](https://github.com/dashpay/rust-dashcore/pull/910) — ⚠ merge conflict · 📝 draft · areas: dash-spv · Policy: draft
  - Blocker: Draft PR does not occupy a review slot

<a id="llbartekll-needs-action"></a>
#### Needs action (2)
- [dashpay/platform#3560 test(swift-sdk): add testnet identity-discovery UI test](https://github.com/dashpay/platform/pull/3560) — 1 unresolved (1 bot) · 131 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · areas: swift-sdk, github · Policy: waiting-author
  - Top thread: "🔴 Blocking: Carried-forward prior finding (STILL VALID): Docker-setup toggle leaves the cached regtest wallet manager bo…" — 131 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/rust-dashcore#819 feat(key-wallet): CoinJoin sweep + gap-limit recovery; TransactionBuilder drain mode](https://github.com/dashpay/rust-dashcore/pull/819) — ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · areas: key-wallet · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed e406d1cbb86a03503fa41a9d796f25c0bacc3e12
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return

<a id="llbartekll-unresolved-comments"></a>
#### Unresolved Comments (1)
- [dashpay/platform#3560 test(swift-sdk): add testnet identity-discovery UI test](https://github.com/dashpay/platform/pull/3560) — 1 unresolved (1 bot) · 131 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · areas: swift-sdk, github · Policy: waiting-author
  - Top thread: "🔴 Blocking: Carried-forward prior finding (STILL VALID): Docker-setup toggle leaves the cached regtest wallet manager bo…" — 131 days old
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

<a id="xdustinface"></a>
### @xdustinface
<a id="xdustinface-open"></a>
#### Open (8)
- [dashpay/rust-dashcore#856 feat(dash-spv): force resync to re-anchor a running client at a lower checkpoint](https://github.com/dashpay/rust-dashcore/pull/856) — 1 unresolved (1 human) · 81 days stale · ⚠ merge conflict · 🔴 CI failing · areas: dash-spv · Policy: waiting-self-review
  - Top thread: "why do we have a periodic check here??" — 81 days old
  - Blocker: Author must post /self-reviewed 7282172a3bb4a0d9d85a40cf70b4dd111995f96b
- [dashpay/rust-dashcore#400 chore: remove BIP141 (SegWit) and BIP341 (Taproot) support](https://github.com/dashpay/rust-dashcore/pull/400) — ⚠ merge conflict · 🔴 CI failing · 📝 draft · 🐢 untouched 143 days · areas: dash-spv, key-wallet, github, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#541 ci: integrate Codecov test analytics via \`cargo-nextest\`](https://github.com/dashpay/rust-dashcore/pull/541) — ⚠ merge conflict · 📝 draft · areas: github, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#720 fix(dash-spv): split peer \`TcpStream\` and add per-peer writer task](https://github.com/dashpay/rust-dashcore/pull/720) — ⚠ merge conflict · 🔴 CI failing · 📝 draft · 🐢 untouched 143 days · areas: dash-spv, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#776 chore: remove FFI layer](https://github.com/dashpay/rust-dashcore/pull/776) — ⚠ merge conflict · 📝 draft · 🐢 untouched 143 days · areas: dash-spv, key-wallet, github, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#799 feat: validate masternode list merkle root](https://github.com/dashpay/rust-dashcore/pull/799) — ⚠ merge conflict · 📝 draft · areas: fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#803 feat(dash-spv): block locator + staged fork detection](https://github.com/dashpay/rust-dashcore/pull/803) — ⚠ merge conflict · 📝 draft · areas: dash-spv, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#849 feat(dash-spv): add \`--birth-height\` CLI flag](https://github.com/dashpay/rust-dashcore/pull/849) — areas: dash-spv · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 9b1e709fd8e0bfab0dd69f7c86fd6b7d2e585fbe

<a id="xdustinface-needs-action"></a>
#### Needs action (1)
- [dashpay/rust-dashcore#856 feat(dash-spv): force resync to re-anchor a running client at a lower checkpoint](https://github.com/dashpay/rust-dashcore/pull/856) — 1 unresolved (1 human) · 81 days stale · ⚠ merge conflict · 🔴 CI failing · areas: dash-spv · Policy: waiting-self-review
  - Top thread: "why do we have a periodic check here??" — 81 days old
  - Blocker: Author must post /self-reviewed 7282172a3bb4a0d9d85a40cf70b4dd111995f96b

<a id="xdustinface-unresolved-comments"></a>
#### Unresolved Comments (1)
- [dashpay/rust-dashcore#856 feat(dash-spv): force resync to re-anchor a running client at a lower checkpoint](https://github.com/dashpay/rust-dashcore/pull/856) — 1 unresolved (1 human) · 81 days stale · ⚠ merge conflict · 🔴 CI failing · areas: dash-spv · Policy: waiting-self-review
  - Top thread: "why do we have a periodic check here??" — 81 days old
  - Blocker: Author must post /self-reviewed 7282172a3bb4a0d9d85a40cf70b4dd111995f96b

<a id="xdustinface-draft"></a>
#### Draft (3)
- [dashpay/rust-dashcore#541 ci: integrate Codecov test analytics via \`cargo-nextest\`](https://github.com/dashpay/rust-dashcore/pull/541) — ⚠ merge conflict · 📝 draft · areas: github, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#799 feat: validate masternode list merkle root](https://github.com/dashpay/rust-dashcore/pull/799) — ⚠ merge conflict · 📝 draft · areas: fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#803 feat(dash-spv): block locator + staged fork detection](https://github.com/dashpay/rust-dashcore/pull/803) — ⚠ merge conflict · 📝 draft · areas: dash-spv, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot

<a id="xdustinface-stale"></a>
#### Stale (3)
- [dashpay/rust-dashcore#400 chore: remove BIP141 (SegWit) and BIP341 (Taproot) support](https://github.com/dashpay/rust-dashcore/pull/400) — ⚠ merge conflict · 🔴 CI failing · 📝 draft · 🐢 untouched 143 days · areas: dash-spv, key-wallet, github, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#720 fix(dash-spv): split peer \`TcpStream\` and add per-peer writer task](https://github.com/dashpay/rust-dashcore/pull/720) — ⚠ merge conflict · 🔴 CI failing · 📝 draft · 🐢 untouched 143 days · areas: dash-spv, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#776 chore: remove FFI layer](https://github.com/dashpay/rust-dashcore/pull/776) — ⚠ merge conflict · 📝 draft · 🐢 untouched 143 days · areas: dash-spv, key-wallet, github, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot

<a id="xdustinface-clean"></a>
#### Clean (1)
- [dashpay/rust-dashcore#849 feat(dash-spv): add \`--birth-height\` CLI flag](https://github.com/dashpay/rust-dashcore/pull/849) — areas: dash-spv · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 9b1e709fd8e0bfab0dd69f7c86fd6b7d2e585fbe

<a id="xdustinface-ready-for-review"></a>
#### Ready for Review (6)
- [dashpay/rust-dashcore#1106 fix(wallet): correct late inputs during InstantSend backfill](https://github.com/dashpay/rust-dashcore/pull/1106) — by @lklimek · areas: dash-spv, key-wallet, key-wallet-manager, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 61099afed724fa3c359e16f47c8533459f7b28e8
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/rust-dashcore#1136 feat(dash)!: decode unknown special transaction types as raw payloads](https://github.com/dashpay/rust-dashcore/pull/1136) — by @ZocoLini · areas: key-wallet, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 249ef12b0740cfca9947d0e823c4b9fd334b7379
- [dashpay/rust-dashcore#1142 fix(dash)!: adopt \`dash_num::Arith256\` around \`U256\` implementation, align closer to reference implementation](https://github.com/dashpay/rust-dashcore/pull/1142) — by @kwvg · areas: fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
- [dashpay/rust-dashcore#1144 fix!: reject WIF keys without \`0x01\` flag, reject sighash types &gt;\`0xff\`, maintain symmetry in &#123;sign,recover&#125; compact signatures, drop unused divergent segments](https://github.com/dashpay/rust-dashcore/pull/1144) — by @kwvg · areas: key-wallet, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
- [dashpay/rust-dashcore#1147 fix(rpc-client): keep request arguments and results out of the logs](https://github.com/dashpay/rust-dashcore/pull/1147) — by @ZocoLini · areas: fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 39048d217ecd02387cfd89ac0a79e45d210abc51
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/rust-dashcore#1148 fix(rpc-client): send the ChainLock block hash in display order to \`submitchainlock\`](https://github.com/dashpay/rust-dashcore/pull/1148) — by @ZocoLini · areas: fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 9240f4599b47a0b3936bbba06df7c99087f5e103

<a id="claudius-maginificent"></a>
### @Claudius-Maginificent
<a id="claudius-maginificent-open"></a>
#### Open (9)
- [dashpay/platform#3549 test(platform-wallet): e2e framework + full test suite — triage pins, Found-*/PA-* guards, fail-closed persist, Stage-2 merge](https://github.com/dashpay/platform/pull/3549) — 9 unresolved (9 bot) · 160 days stale · ⚠ merge conflict · 📝 draft · 🐢 targets v5.1-dev · areas: rs-drive, rs-drive-abci, rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk, rust-sdk, github, fallback · Policy: draft
  - Top thread: "🟡 Suggestion: \`from_seed_for_identity\` is misleadingly named, half-functional, and unused**" — 160 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/dash-evo-tool#1068 docs(gui-testing): add A/B scenarios for votes hub, usernames, funding minimums and wallet picker](https://github.com/dashpay/dash-evo-tool/pull/1068) — 2 unresolved (2 CodeRabbit) · 0 days stale · areas: fallback · Policy: waiting-author
  - Top thread: "🩺 Stability &amp; Availability** \| **🟡 Minor** \| **⚡ Quick win**" — 0 days old
  - Blocker: coderabbitai left review threads unresolved; resolve them
- [dashpay/dash-evo-tool#1066 fix(dpns): stop re-reading finished name contests on every refresh](https://github.com/dashpay/dash-evo-tool/pull/1066) — areas: fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/dash-evo-tool#1067 fix(dpns): let operators vote while a long contest history loads](https://github.com/dashpay/dash-evo-tool/pull/1067) — 🐢 targets fix/votes-history-dapi-flood · areas: fallback
- [dashpay/dash-evo-tool#1069 docs: align changelog, kv-keys and stories with shipped voting behaviour](https://github.com/dashpay/dash-evo-tool/pull/1069) — areas: fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/dash-evo-tool#1070 fix(voting): stop the old-schedule notice for votes a pre-release build already cast](https://github.com/dashpay/dash-evo-tool/pull/1070) — areas: fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/dash-evo-tool#1071 fix(dpns): keep the background contest refresh from resetting unrelated screens](https://github.com/dashpay/dash-evo-tool/pull/1071) — 🐢 targets fix/votes-cold-load-early-vote-state · areas: fallback
- [dashpay/dash-evo-tool#1072 fix(tools): keep a Platform info fetch and its result through background refreshes](https://github.com/dashpay/dash-evo-tool/pull/1072) — areas: fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/platform#5312 feat(sdk): read how many times a masternode has voted on a contested resource](https://github.com/dashpay/platform/pull/5312) — 📝 draft · 🐢 targets v5.1-dev · areas: rs-drive, rs-drive-abci, rust-sdk, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot

<a id="claudius-maginificent-needs-action"></a>
#### Needs action (1)
- [dashpay/dash-evo-tool#1068 docs(gui-testing): add A/B scenarios for votes hub, usernames, funding minimums and wallet picker](https://github.com/dashpay/dash-evo-tool/pull/1068) — 2 unresolved (2 CodeRabbit) · 0 days stale · areas: fallback · Policy: waiting-author
  - Top thread: "🩺 Stability &amp; Availability** \| **🟡 Minor** \| **⚡ Quick win**" — 0 days old
  - Blocker: coderabbitai left review threads unresolved; resolve them

<a id="claudius-maginificent-ready-for-human"></a>
#### Ready for human (4)
- [dashpay/dash-evo-tool#1066 fix(dpns): stop re-reading finished name contests on every refresh](https://github.com/dashpay/dash-evo-tool/pull/1066) — areas: fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/dash-evo-tool#1069 docs: align changelog, kv-keys and stories with shipped voting behaviour](https://github.com/dashpay/dash-evo-tool/pull/1069) — areas: fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/dash-evo-tool#1070 fix(voting): stop the old-schedule notice for votes a pre-release build already cast](https://github.com/dashpay/dash-evo-tool/pull/1070) — areas: fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/dash-evo-tool#1072 fix(tools): keep a Platform info fetch and its result through background refreshes](https://github.com/dashpay/dash-evo-tool/pull/1072) — areas: fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return

<a id="claudius-maginificent-unresolved-comments"></a>
#### Unresolved Comments (1)
- [dashpay/dash-evo-tool#1068 docs(gui-testing): add A/B scenarios for votes hub, usernames, funding minimums and wallet picker](https://github.com/dashpay/dash-evo-tool/pull/1068) — 2 unresolved (2 CodeRabbit) · 0 days stale · areas: fallback · Policy: waiting-author
  - Top thread: "🩺 Stability &amp; Availability** \| **🟡 Minor** \| **⚡ Quick win**" — 0 days old
  - Blocker: coderabbitai left review threads unresolved; resolve them

<a id="claudius-maginificent-stale"></a>
#### Stale (4)
- [dashpay/platform#3549 test(platform-wallet): e2e framework + full test suite — triage pins, Found-*/PA-* guards, fail-closed persist, Stage-2 merge](https://github.com/dashpay/platform/pull/3549) — 9 unresolved (9 bot) · 160 days stale · ⚠ merge conflict · 📝 draft · 🐢 targets v5.1-dev · areas: rs-drive, rs-drive-abci, rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk, rust-sdk, github, fallback · Policy: draft
  - Top thread: "🟡 Suggestion: \`from_seed_for_identity\` is misleadingly named, half-functional, and unused**" — 160 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/dash-evo-tool#1067 fix(dpns): let operators vote while a long contest history loads](https://github.com/dashpay/dash-evo-tool/pull/1067) — 🐢 targets fix/votes-history-dapi-flood · areas: fallback
- [dashpay/dash-evo-tool#1071 fix(dpns): keep the background contest refresh from resetting unrelated screens](https://github.com/dashpay/dash-evo-tool/pull/1071) — 🐢 targets fix/votes-cold-load-early-vote-state · areas: fallback
- [dashpay/platform#5312 feat(sdk): read how many times a masternode has voted on a contested resource](https://github.com/dashpay/platform/pull/5312) — 📝 draft · 🐢 targets v5.1-dev · areas: rs-drive, rs-drive-abci, rust-sdk, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot

<a id="claudius-maginificent-clean"></a>
#### Clean (4)
- [dashpay/dash-evo-tool#1066 fix(dpns): stop re-reading finished name contests on every refresh](https://github.com/dashpay/dash-evo-tool/pull/1066) — areas: fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/dash-evo-tool#1069 docs: align changelog, kv-keys and stories with shipped voting behaviour](https://github.com/dashpay/dash-evo-tool/pull/1069) — areas: fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/dash-evo-tool#1070 fix(voting): stop the old-schedule notice for votes a pre-release build already cast](https://github.com/dashpay/dash-evo-tool/pull/1070) — areas: fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/dash-evo-tool#1072 fix(tools): keep a Platform info fetch and its result through background refreshes](https://github.com/dashpay/dash-evo-tool/pull/1072) — areas: fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return

<a id="hashengineering"></a>
### @HashEngineering
<a id="hashengineering-open"></a>
#### Open (4)
- [dashpay/platform#5259 fix(kotlin-sdk): port #4638 to Android — credit verdicts at save time and the post-sync TXO reconcile](https://github.com/dashpay/platform/pull/5259) — 5 unresolved (5 bot) · 2 days stale · 🐢 targets v5.1-dev · areas: kotlin-sdk, fallback · Policy: waiting-bots
  - Top thread: "🟡 Suggestion: Require inventory account fields to enforce fail-loud decoding**" — 2 days old
  - Blocker: coderabbitai has not reported for the current head
- [dashpay/platform#5026 fix(platform-wallet)!: persist DashPay coreHeight backfill coverage so a relaunch resumes instead of rewinding again](https://github.com/dashpay/platform/pull/5026) — 2 unresolved (2 bot) · 0 days stale · ✋ changes requested · 🔴 CI failing · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, kotlin-sdk, fallback · Policy: waiting-bots
  - Top thread: "🔴 Blocking: Bind the restored outbound marker to the account's registration generation**" — 0 days old
  - Blocker: coderabbitai has not reported for the current head
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5352 fix(platform-wallet-storage): open the wallet store on Android](https://github.com/dashpay/platform/pull/5352) — 1 unresolved (1 CodeRabbit) · 1 days stale · 📝 draft · areas: wallet-storage · Policy: draft
  - Top thread: "🎯 Functional Correctness** \| **🟡 Minor** \| **⚡ Quick win**" — 1 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5256 fix(platform-wallet): build our receiving account for one-way DashPay contacts](https://github.com/dashpay/platform/pull/5256) — 🔴 CI failing · 🐢 targets v5.1-dev · areas: rs-platform-wallet · Policy: waiting-bots
  - Blocker: coderabbitai has not reported for the current head

<a id="hashengineering-needs-action"></a>
#### Needs action (1)
- [dashpay/platform#5026 fix(platform-wallet)!: persist DashPay coreHeight backfill coverage so a relaunch resumes instead of rewinding again](https://github.com/dashpay/platform/pull/5026) — 2 unresolved (2 bot) · 0 days stale · ✋ changes requested · 🔴 CI failing · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, kotlin-sdk, fallback · Policy: waiting-bots
  - Top thread: "🔴 Blocking: Bind the restored outbound marker to the account's registration generation**" — 0 days old
  - Blocker: coderabbitai has not reported for the current head
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them

<a id="hashengineering-unresolved-comments"></a>
#### Unresolved Comments (1)
- [dashpay/platform#5026 fix(platform-wallet)!: persist DashPay coreHeight backfill coverage so a relaunch resumes instead of rewinding again](https://github.com/dashpay/platform/pull/5026) — 2 unresolved (2 bot) · 0 days stale · ✋ changes requested · 🔴 CI failing · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, kotlin-sdk, fallback · Policy: waiting-bots
  - Top thread: "🔴 Blocking: Bind the restored outbound marker to the account's registration generation**" — 0 days old
  - Blocker: coderabbitai has not reported for the current head
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them

<a id="hashengineering-draft"></a>
#### Draft (1)
- [dashpay/platform#5352 fix(platform-wallet-storage): open the wallet store on Android](https://github.com/dashpay/platform/pull/5352) — 1 unresolved (1 CodeRabbit) · 1 days stale · 📝 draft · areas: wallet-storage · Policy: draft
  - Top thread: "🎯 Functional Correctness** \| **🟡 Minor** \| **⚡ Quick win**" — 1 days old
  - Blocker: Draft PR does not occupy a review slot

<a id="hashengineering-stale"></a>
#### Stale (2)
- [dashpay/platform#5259 fix(kotlin-sdk): port #4638 to Android — credit verdicts at save time and the post-sync TXO reconcile](https://github.com/dashpay/platform/pull/5259) — 5 unresolved (5 bot) · 2 days stale · 🐢 targets v5.1-dev · areas: kotlin-sdk, fallback · Policy: waiting-bots
  - Top thread: "🟡 Suggestion: Require inventory account fields to enforce fail-loud decoding**" — 2 days old
  - Blocker: coderabbitai has not reported for the current head
- [dashpay/platform#5256 fix(platform-wallet): build our receiving account for one-way DashPay contacts](https://github.com/dashpay/platform/pull/5256) — 🔴 CI failing · 🐢 targets v5.1-dev · areas: rs-platform-wallet · Policy: waiting-bots
  - Blocker: coderabbitai has not reported for the current head

<a id="infraclaw-dash"></a>
### @infraclaw-dash
<a id="infraclaw-dash-open"></a>
#### Open (3)
- [dashpay/platform#3958 ci: add Platform testnet sync status reporting](https://github.com/dashpay/platform/pull/3958) — 2 unresolved (2 bot) · 108 days stale · ⚠ merge conflict · 🐢 targets v5.1-dev · areas: github, fallback · Policy: waiting-author
  - Top thread: "🟡 Suggestion: workflow_dispatch lets any repo writer forge a sync status**" — 108 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: coderabbitai left review threads unresolved; resolve them
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5167 ci: align secp feature-base workflows with provisioned runner images](https://github.com/dashpay/platform/pull/5167) — 3 unresolved (3 bot) · 11 days stale · ⚠ merge conflict · 🐢 targets chore/bump-rust-dashcore-secp-033 · areas: github
  - Top thread: "🟡 Suggestion: Contract-failure error points to a doc file missing from this branch**" — 11 days old
- [dashpay/platform#5166 ci: adopt verified Rust and Kotlin runner images on v4.3](https://github.com/dashpay/platform/pull/5166) — ⚠ merge conflict · 🐢 targets v5.1-dev · areas: github · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required

<a id="infraclaw-dash-ready-for-human"></a>
#### Ready for human (1)
- [dashpay/platform#5166 ci: adopt verified Rust and Kotlin runner images on v4.3](https://github.com/dashpay/platform/pull/5166) — ⚠ merge conflict · 🐢 targets v5.1-dev · areas: github · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required

<a id="infraclaw-dash-stale"></a>
#### Stale (3)
- [dashpay/platform#3958 ci: add Platform testnet sync status reporting](https://github.com/dashpay/platform/pull/3958) — 2 unresolved (2 bot) · 108 days stale · ⚠ merge conflict · 🐢 targets v5.1-dev · areas: github, fallback · Policy: waiting-author
  - Top thread: "🟡 Suggestion: workflow_dispatch lets any repo writer forge a sync status**" — 108 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: coderabbitai left review threads unresolved; resolve them
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5167 ci: align secp feature-base workflows with provisioned runner images](https://github.com/dashpay/platform/pull/5167) — 3 unresolved (3 bot) · 11 days stale · ⚠ merge conflict · 🐢 targets chore/bump-rust-dashcore-secp-033 · areas: github
  - Top thread: "🟡 Suggestion: Contract-failure error points to a doc file missing from this branch**" — 11 days old
- [dashpay/platform#5166 ci: adopt verified Rust and Kotlin runner images on v4.3](https://github.com/dashpay/platform/pull/5166) — ⚠ merge conflict · 🐢 targets v5.1-dev · areas: github · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required

<a id="bfoss765"></a>
### @bfoss765
<a id="bfoss765-open"></a>
#### Open (2)
- [dashpay/platform#4313 feat(kotlin-sdk): one-time Orchard key shielded-invite API (inviter + claim)](https://github.com/dashpay/platform/pull/4313) — 2 unresolved (2 bot) · 40 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · 🐢 targets v5.1-dev · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk, fallback · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Preserve claim-binding mismatch semantics across the FFI boundary**" — 40 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/platform#4312 feat(platform-wallet): multi-output shielded transfers + output-aware fee predictor](https://github.com/dashpay/platform/pull/4312) — 1 unresolved (1 bot) · 40 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · 🐢 targets v5.1-dev · areas: dpp, rs-platform-wallet, rs-platform-wallet-ffi, kotlin-sdk, fallback · Policy: waiting-self-review
  - Top thread: "🟡 Suggestion: Keep envelope-aware action counting behind the crate boundary**" — 40 days old
  - Blocker: Author must post /self-reviewed da932f96a8670c641065954de8b58752e99233a3
  - Blocker: Proceeded without coderabbitai: no review within the configured window

<a id="bfoss765-stale"></a>
#### Stale (2)
- [dashpay/platform#4313 feat(kotlin-sdk): one-time Orchard key shielded-invite API (inviter + claim)](https://github.com/dashpay/platform/pull/4313) — 2 unresolved (2 bot) · 40 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · 🐢 targets v5.1-dev · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk, fallback · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Preserve claim-binding mismatch semantics across the FFI boundary**" — 40 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/platform#4312 feat(platform-wallet): multi-output shielded transfers + output-aware fee predictor](https://github.com/dashpay/platform/pull/4312) — 1 unresolved (1 bot) · 40 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · 🐢 targets v5.1-dev · areas: dpp, rs-platform-wallet, rs-platform-wallet-ffi, kotlin-sdk, fallback · Policy: waiting-self-review
  - Top thread: "🟡 Suggestion: Keep envelope-aware action counting behind the crate boundary**" — 40 days old
  - Blocker: Author must post /self-reviewed da932f96a8670c641065954de8b58752e99233a3
  - Blocker: Proceeded without coderabbitai: no review within the configured window

<a id="ktechmidas"></a>
### @ktechmidas
<a id="ktechmidas-open"></a>
#### Open (8)
- [dashpay/platform#5188 ci: tolerate optional S3 cache export outages on chore/bump-rust-dashcore-secp-033](https://github.com/dashpay/platform/pull/5188) — 1 unresolved (1 bot) · 11 days stale · 🐢 targets chore/bump-rust-dashcore-secp-033 · areas: github
  - Top thread: "🟡 Suggestion: New regression tests are not executed by any CI workflow on this branch**" — 11 days old
- [dashpay/platform#5172 ci: stage reviewed AMD64 recipe for candidate validation](https://github.com/dashpay/platform/pull/5172) — areas: github · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 2c25cdf6c30ac2fc256390a6faef55e5464241f5
- [dashpay/platform#5186 ci: tolerate optional S3 cache export outages on v5.0-dev](https://github.com/dashpay/platform/pull/5186) — 🔴 CI failing · 🐢 targets v6.0-dev · areas: github · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 843052c2dce26a80065c15b1219bbc8211264161
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/platform#5187 ci: tolerate optional S3 cache export outages on v4.3-dev](https://github.com/dashpay/platform/pull/5187) — 🐢 targets v5.1-dev · areas: github · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed fbbf13ee41f8bd8b9f03f1dc5afec6a619967d97
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/platform#5198 ci: bound default Binaryen optimizer threads in NPM builds](https://github.com/dashpay/platform/pull/5198) — 🐢 targets v6.0-dev · areas: github · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 23b41b477bc038965b686b4a75412b1c21fc6d98
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/platform#5203 fix(release): install verified cargo-binstall binary directly](https://github.com/dashpay/platform/pull/5203) — areas: fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed f8fee0b29313e97840ce6a67fa335e90b5da7301
- [dashpay/platform#5204 fix(release): install verified cargo-binstall binary directly](https://github.com/dashpay/platform/pull/5204) — 🐢 targets v5.1-dev · areas: fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 63455a2563fd7271fb11083d3b0a89bdf01f3982
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/platform#5205 fix(release): install verified cargo-binstall binary directly](https://github.com/dashpay/platform/pull/5205) — 🐢 targets v6.0-dev · areas: fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 8a8ac2c788b5338416e18045001d9115ba3ac80a
  - Blocker: Proceeded without coderabbitai: no review within the configured window

<a id="ktechmidas-stale"></a>
#### Stale (6)
- [dashpay/platform#5188 ci: tolerate optional S3 cache export outages on chore/bump-rust-dashcore-secp-033](https://github.com/dashpay/platform/pull/5188) — 1 unresolved (1 bot) · 11 days stale · 🐢 targets chore/bump-rust-dashcore-secp-033 · areas: github
  - Top thread: "🟡 Suggestion: New regression tests are not executed by any CI workflow on this branch**" — 11 days old
- [dashpay/platform#5186 ci: tolerate optional S3 cache export outages on v5.0-dev](https://github.com/dashpay/platform/pull/5186) — 🔴 CI failing · 🐢 targets v6.0-dev · areas: github · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 843052c2dce26a80065c15b1219bbc8211264161
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/platform#5187 ci: tolerate optional S3 cache export outages on v4.3-dev](https://github.com/dashpay/platform/pull/5187) — 🐢 targets v5.1-dev · areas: github · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed fbbf13ee41f8bd8b9f03f1dc5afec6a619967d97
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/platform#5198 ci: bound default Binaryen optimizer threads in NPM builds](https://github.com/dashpay/platform/pull/5198) — 🐢 targets v6.0-dev · areas: github · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 23b41b477bc038965b686b4a75412b1c21fc6d98
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/platform#5204 fix(release): install verified cargo-binstall binary directly](https://github.com/dashpay/platform/pull/5204) — 🐢 targets v5.1-dev · areas: fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 63455a2563fd7271fb11083d3b0a89bdf01f3982
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/platform#5205 fix(release): install verified cargo-binstall binary directly](https://github.com/dashpay/platform/pull/5205) — 🐢 targets v6.0-dev · areas: fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 8a8ac2c788b5338416e18045001d9115ba3ac80a
  - Blocker: Proceeded without coderabbitai: no review within the configured window

<a id="ktechmidas-clean"></a>
#### Clean (2)
- [dashpay/platform#5172 ci: stage reviewed AMD64 recipe for candidate validation](https://github.com/dashpay/platform/pull/5172) — areas: github · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 2c25cdf6c30ac2fc256390a6faef55e5464241f5
- [dashpay/platform#5203 fix(release): install verified cargo-binstall binary directly](https://github.com/dashpay/platform/pull/5203) — areas: fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed f8fee0b29313e97840ce6a67fa335e90b5da7301

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
- [dashpay/platform#4707 feat: alloc-only profiles for platform-value and platform-serialization](https://github.com/dashpay/platform/pull/4707) — 🔴 CI failing · 🐢 targets v5.1-dev · areas: github, fallback · Policy: ready-for-human
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
- [dashpay/platform#4707 feat: alloc-only profiles for platform-value and platform-serialization](https://github.com/dashpay/platform/pull/4707) — 🔴 CI failing · 🐢 targets v5.1-dev · areas: github, fallback · Policy: ready-for-human
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
- [dashpay/platform#4707 feat: alloc-only profiles for platform-value and platform-serialization](https://github.com/dashpay/platform/pull/4707) — 🔴 CI failing · 🐢 targets v5.1-dev · areas: github, fallback · Policy: ready-for-human
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

<a id="kwvg"></a>
### @kwvg
<a id="kwvg-open"></a>
#### Open (2)
- [dashpay/rust-dashcore#1142 fix(dash)!: adopt \`dash_num::Arith256\` around \`U256\` implementation, align closer to reference implementation](https://github.com/dashpay/rust-dashcore/pull/1142) — areas: fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
- [dashpay/rust-dashcore#1144 fix!: reject WIF keys without \`0x01\` flag, reject sighash types &gt;\`0xff\`, maintain symmetry in &#123;sign,recover&#125; compact signatures, drop unused divergent segments](https://github.com/dashpay/rust-dashcore/pull/1144) — areas: key-wallet, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required

<a id="kwvg-ready-for-human"></a>
#### Ready for human (2)
- [dashpay/rust-dashcore#1142 fix(dash)!: adopt \`dash_num::Arith256\` around \`U256\` implementation, align closer to reference implementation](https://github.com/dashpay/rust-dashcore/pull/1142) — areas: fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
- [dashpay/rust-dashcore#1144 fix!: reject WIF keys without \`0x01\` flag, reject sighash types &gt;\`0xff\`, maintain symmetry in &#123;sign,recover&#125; compact signatures, drop unused divergent segments](https://github.com/dashpay/rust-dashcore/pull/1144) — areas: key-wallet, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required

<a id="kwvg-clean"></a>
#### Clean (2)
- [dashpay/rust-dashcore#1142 fix(dash)!: adopt \`dash_num::Arith256\` around \`U256\` implementation, align closer to reference implementation](https://github.com/dashpay/rust-dashcore/pull/1142) — areas: fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
- [dashpay/rust-dashcore#1144 fix!: reject WIF keys without \`0x01\` flag, reject sighash types &gt;\`0xff\`, maintain symmetry in &#123;sign,recover&#125; compact signatures, drop unused divergent segments](https://github.com/dashpay/rust-dashcore/pull/1144) — areas: key-wallet, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required

## Methodology
Generated nightly by [pr-hygiene](https://github.com/dashpay/stale_prs_are_bad). A thread counts as "unresolved" when it is open, not outdated, has a comment from someone other than the PR author, and the most recent comment is from a reviewer. **Dirty** = at least one such thread. **Unresolved Comments** = at least one such thread. **Changes Requested** = no unresolved threads but a reviewer's most recent review is CHANGES_REQUESTED (still blocking until someone re-approves or dismisses). **Deferred** = carries a configured deferred label (e.g. `postponed`) — visible but not counted toward unresolved-comment counts. **Stale** = targets a non-default branch OR hasn't been touched in the configured threshold (default 120 days, but clean PRs are never reclassified as stale). **Draft** = the PR is still marked draft on GitHub. **CI failing** = no unresolved comments, no changes-requested, but the latest commit's status check is failing. **Clean** = open, not draft, not deferred, not stale, no unresolved comments, no changes-requested, CI green. **Needs action** further requires changes-requested, merge conflict, or that the reviewer commented more recently than the author last pushed. **Ready for human** counts a person's own PRs that the shared review engine marks `ready-for-human` or `ready-to-merge`; each PR bullet shows the engine's state as `Policy:` with its blockers. **Ready for Review** counts clean PRs (authored by someone else) where this person owes a review: the union of the shared policy's routing (owners and reviewers of every area the changed files fall into, or the repository fallback for files no area claims) and GitHub's explicit review requests. The author is never routed to their own PR, and anyone who has already submitted any review is excluded — their job is done. `⚠ ownership unresolved` marks areas whose roster the policy still lists as open. Configurable via [`https://github.com/dashpay/stale_prs_are_bad/blob/master/.pr-hygiene.yml`](https://github.com/dashpay/stale_prs_are_bad/blob/master/.pr-hygiene.yml)—edit defaults there; ownership lives in `policies/`.
