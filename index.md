---
---
# PR Hygiene Report
*Last updated: 2026-10-01 06:20 UTC · commit 3b98772*

## Summary
- Open PRs: **144** (10 clean · 6 CI failing · 1 changes requested · 29 unresolved comments · 0 deferred · 20 draft · 78 stale)
- PRs needing author action: **33**
- Total unresolved comments: **199**
- dashpay/platform: **95** open (7 clean · 2 CI failing · 0 changes requested · 12 unresolved comments · 0 deferred · 4 draft · 70 stale) · engine: 4 draft · 12 waiting-author · 4 waiting-bots · 5 waiting-self-review · 70 no verdict
- dashpay/rust-dashcore: **35** open (3 clean · 3 CI failing · 1 changes requested · 9 unresolved comments · 0 deferred · 14 draft · 5 stale) · engine: 17 draft · 7 waiting-author · 9 waiting-self-review · 2 no verdict
- dashpay/tenderdash: **5** open (0 clean · 1 CI failing · 0 changes requested · 3 unresolved comments · 0 deferred · 1 draft · 0 stale) · engine: 1 draft · 3 waiting-author · 1 waiting-self-review
- dashpay/grovedb: **0** open (0 clean · 0 CI failing · 0 changes requested · 0 unresolved comments · 0 deferred · 0 draft · 0 stale) · engine: no PRs
- dashpay/dash-evo-tool: **9** open (0 clean · 0 CI failing · 0 changes requested · 5 unresolved comments · 0 deferred · 1 draft · 3 stale) · engine: 1 draft · 5 waiting-author · 3 no verdict

## Scoreboard
_Sort: unresolved-comments desc → needs-action desc → ready-for-review desc. Click any number to jump to the specific PRs it covers._

| Author | Open | Clean | CI failing | Unresolved Comments | Changes Requested | Deferred | Draft | Stale | Needs action | Ready for human | Total Unresolved Comments | Ready for Review | Δ |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| [@PastaPastaPasta + (@thepastaclaw)](#pastapastapasta) | [23+(8)](#pastapastapasta-open) | [3+(1)](#pastapastapasta-clean) | [1+(1)](#pastapastapasta-ci-failing) | [7+(2)](#pastapastapasta-unresolved-comments) | — | — | [3+(2)](#pastapastapasta-draft) | [9+(2)](#pastapastapasta-stale) | [8+(3)](#pastapastapasta-needs-action) | — | [49+(9)](#pastapastapasta-unresolved-comments) | — | ↓ 5 |
| [@lklimek + (@Claudius-Maginificent)](#lklimek) | [16+(3)](#lklimek-open) | — | [1+(0)](#lklimek-ci-failing) | [6+(1)](#lklimek-unresolved-comments) | — | — | [2+(0)](#lklimek-draft) | [7+(2)](#lklimek-stale) | [6+(1)](#lklimek-needs-action) | — | [32+(18)](#lklimek-unresolved-comments) | [3+(0)](#lklimek-ready-for-review) | ↑ 2 |
| [@shumkov](#shumkov) | [14](#shumkov-open) | — | — | [5](#shumkov-unresolved-comments) | — | — | — | [9](#shumkov-stale) | [5](#shumkov-needs-action) | — | [39](#shumkov-unresolved-comments) | [6](#shumkov-ready-for-review) | ↑ 4 |
| [@QuantumExplorer](#quantumexplorer) | [19](#quantumexplorer-open) | [1](#quantumexplorer-clean) | [2](#quantumexplorer-ci-failing) | [4](#quantumexplorer-unresolved-comments) | — | — | [2](#quantumexplorer-draft) | [10](#quantumexplorer-stale) | [5](#quantumexplorer-needs-action) | — | [35](#quantumexplorer-unresolved-comments) | [5](#quantumexplorer-ready-for-review) | ↑ 2 |
| [@llbartekll](#llbartekll) | [3](#llbartekll-open) | — | — | [1](#llbartekll-unresolved-comments) | [1](#llbartekll-changes-requested) | — | [1](#llbartekll-draft) | — | [2](#llbartekll-needs-action) | — | [1](#llbartekll-unresolved-comments) | [1](#llbartekll-ready-for-review) | — |
| [@ZocoLini](#zocolini) | [9](#zocolini-open) | [1](#zocolini-clean) | [1](#zocolini-ci-failing) | [1](#zocolini-unresolved-comments) | — | — | [6](#zocolini-draft) | — | [1](#zocolini-needs-action) | — | [3](#zocolini-unresolved-comments) | [1](#zocolini-ready-for-review) | ↑ 1 |
| [@romchornyi](#romchornyi) | [5](#romchornyi-open) | — | — | [1](#romchornyi-unresolved-comments) | — | — | — | [4](#romchornyi-stale) | [1](#romchornyi-needs-action) | — | [3](#romchornyi-unresolved-comments) | [1](#romchornyi-ready-for-review) | — |
| [@xdustinface](#xdustinface) | [8](#xdustinface-open) | [1](#xdustinface-clean) | — | [1](#xdustinface-unresolved-comments) | — | — | [3](#xdustinface-draft) | [3](#xdustinface-stale) | [1](#xdustinface-needs-action) | — | [1](#xdustinface-unresolved-comments) | [1](#xdustinface-ready-for-review) | — |
| [@infraclaw-dash](#infraclaw-dash) | [4](#infraclaw-dash-open) | — | — | — | — | — | — | [4](#infraclaw-dash-stale) | — | — | [5](#infraclaw-dash-unresolved-comments) | — | — |
| [@bfoss765](#bfoss765) | [2](#bfoss765-open) | — | — | — | — | — | — | [2](#bfoss765-stale) | — | — | [3](#bfoss765-unresolved-comments) | — | — |
| [@ktechmidas](#ktechmidas) | [8](#ktechmidas-open) | [2](#ktechmidas-clean) | — | — | — | — | — | [6](#ktechmidas-stale) | — | — | [1](#ktechmidas-unresolved-comments) | — | — |
| [@DCG-Claude](#dcg-claude) | [19](#dcg-claude-open) | — | — | — | — | — | — | [19](#dcg-claude-stale) | — | — | — | — | — |
| [@HashEngineering](#hashengineering) | [3](#hashengineering-open) | [1](#hashengineering-clean) | — | — | — | — | [1](#hashengineering-draft) | [1](#hashengineering-stale) | — | — | — | — | — |

## Per-author detail

<a id="pastapastapasta"></a>
### @PastaPastaPasta + (@thepastaclaw)
<a id="pastapastapasta-open"></a>
#### Open (31)
- [dashpay/platform#3096 feat(sdk): add client-side validation to state transition construction methods](https://github.com/dashpay/platform/pull/3096) — via @thepastaclaw · 4 unresolved (2 CodeRabbit, 2 human) · 225 days stale · ⚠ merge conflict · 🐢 targets v5.1-dev · areas: rs-drive-abci, dpp, js-wasm-sdk, fallback
  - Top thread: "_⚠️ Potential issue_ \| _🟡 Minor_" — 222 days old
- [dashpay/rust-dashcore#137 fix: integration tests, continued dashificiation](https://github.com/dashpay/rust-dashcore/pull/137) — 2 unresolved (2 CodeRabbit) · 391 days stale · ⚠ merge conflict · 🔴 CI failing · 🐢 targets v0.40-dev, untouched 294 days · areas: fallback
  - Top thread: "_🛠️ Refactor suggestion_" — 391 days old
- [dashpay/platform#4530 test(dashmate): live state sync e2e — join tooling, churn, re-sync and fallback coverage](https://github.com/dashpay/platform/pull/4530) — 6 unresolved (6 bot) · 32 days stale · ⚠ merge conflict · 📝 draft · 🐢 targets v5.1-dev · areas: rs-drive, rs-drive-abci, dpp, dashmate, js-wasm-sdk, github, fallback
  - Top thread: "🟡 Suggestion: Require the churn scenario to interrupt an active state sync**" — 32 days old
- [dashpay/platform#4616 feat(platform-wallet): support DashPay shielded tips with dedicated accounts](https://github.com/dashpay/platform/pull/4616) — 7 unresolved (2 CodeRabbit, 5 bot) · 15 days stale · ⚠ merge conflict · 🐢 targets v5.1-dev · areas: rs-drive, rs-drive-abci, rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk, rust-sdk, system-contracts, kotlin-sdk, fallback
  - Top thread: "_🗄️ Data Integrity & Integration_ \| _🟠 Major_ \| _🏗️ Heavy lift_" — 15 days old
- [dashpay/platform#4844 feat(sdk)!: key limits, DIP-14 sub-feature derivation and decode-any-kind for DashPay Connect](https://github.com/dashpay/platform/pull/4844) — 6 unresolved (4 CodeRabbit, 2 bot) · 11 days stale · ⚠ merge conflict · 🔴 CI failing · areas: rs-drive-abci, dpp, rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, rust-sdk, rust-sdk-ffi, js-wasm-sdk, kotlin-sdk, fallback · Policy: waiting-author
  - Top thread: "🔴 Blocking: Initialize the native library before standalone parsing**" — 11 days old
  - Blocker: coderabbitai left review threads unresolved; resolve them
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#4633 feat(sdk): add dash-platform-cxx, a thin CXX shell over dash-sdk for C++ embedders](https://github.com/dashpay/platform/pull/4633) — 10 unresolved (10 bot) · 2 days stale · ✋ changes requested · 🐢 targets feat/dapi-client-socks5-proxy · areas: github, fallback
  - Top thread: "🟡 Suggestion: Reject contradictory recipient IDs before building a contact request**" — 2 days old
- [dashpay/platform#4648 feat(drive-abci)!: state sync via ABCI snapshots with reduced platform state (protocol v15)](https://github.com/dashpay/platform/pull/4648) — 3 unresolved (3 bot) · 20 days stale · ⚠ merge conflict · 🔴 CI failing · 🐢 targets v5.1-dev · areas: rs-drive, rs-drive-abci, dpp, fallback
  - Top thread: "🟡 Suggestion: Do not silently skip unsupported trees beyond a full discovery page**" — 20 days old
- [dashpay/rust-dashcore#901 fix(key-wallet): discover Coinbase/AssetUnlock outputs for all fund-bearing accounts](https://github.com/dashpay/rust-dashcore/pull/901) — via @thepastaclaw · 2 unresolved (2 CodeRabbit) · 56 days stale · ⚠ merge conflict · ✋ changes requested · 📝 draft · areas: dash-spv, key-wallet · Policy: draft
  - Top thread: "_🎯 Functional Correctness_ \| _🟡 Minor_ \| _⚡ Quick win_" — 56 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#4521 feat(dashmate): state sync configuration for tenderdash and drive snapshots](https://github.com/dashpay/platform/pull/4521) — 2 unresolved (2 bot) · 32 days stale · 📝 draft · 🐢 targets v5.1-dev · areas: dashmate
  - Top thread: "🔴 Blocking: Default snapshot serving exposes unbounded remote checkpoint retention**" — 32 days old
- [dashpay/platform#4538 fix(sdk): honor explicit regtest addresses in dapi-client; tolerate empty discovery in wasm-sdk trusted context](https://github.com/dashpay/platform/pull/4538) — 2 unresolved (2 bot) · 32 days stale · ⚠ merge conflict · 📝 draft · areas: js-wasm-sdk, fallback · Policy: draft
  - Top thread: "🟡 Suggestion: Add deterministic tests for the network-scoped discovery fallback**" — 32 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#4539 fix(dashmate): refresh quorum-server seeds and complete node identities](https://github.com/dashpay/platform/pull/4539) — 2 unresolved (1 CodeRabbit, 1 bot) · 15 days stale · 🐢 targets v5.1-dev · areas: dashmate, github, fallback
  - Top thread: "_🩺 Stability & Availability_ \| _🟡 Minor_ \| _⚡ Quick win_" — 15 days old
- [dashpay/platform#4623 feat(platform-wallet): reserve DashPay payout addresses without Core funding](https://github.com/dashpay/platform/pull/4623) — 2 unresolved (2 bot) · 13 days stale · ✋ changes requested · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Null the string out-param before the signer null-check**" — 13 days old
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/rust-dashcore#264 feat(dash-spv): Add BIP324 v2 encrypted P2P transport](https://github.com/dashpay/rust-dashcore/pull/264) — 2 unresolved (2 CodeRabbit) · 11 days stale · areas: dash-spv, fallback · Policy: waiting-author
  - Top thread: "_📐 Maintainability & Code Quality_ \| _🟡 Minor_ \| _⚡ Quick win_" — 11 days old
  - Blocker: coderabbitai requested changes on this head; dismiss the review or push a fix
  - Blocker: coderabbitai left review threads unresolved; resolve them
- [dashpay/dash-evo-tool#859 perf: detect system theme off UI thread](https://github.com/dashpay/dash-evo-tool/pull/859) — via @thepastaclaw · 2 unresolved (1 CodeRabbit, 1 human) · 7 days stale · areas: fallback · Policy: waiting-author
  - Top thread: "we don't really want to grow the ThemeState. Is there no simpler solution? Also check architecture documents and put the…" — 7 days old
  - Blocker: coderabbitai left review threads unresolved; resolve them
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#4392 perf(swift-sdk): linear wallet-changeset rounds via per-round bulk-prefetch cache](https://github.com/dashpay/platform/pull/4392) — 1 unresolved (1 bot) · 22 days stale · ⚠ merge conflict · ✋ changes requested · areas: swift-sdk · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Exercise transient pending-input fetch failures followed by recovery**" — 22 days old
  - Blocker: thepastaclaw left review threads unresolved; resolve them
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/platform#5160 feat(sdk): route DAPI connections through a SOCKS5 proxy](https://github.com/dashpay/platform/pull/5160) — 2 unresolved (2 bot) · 2 days stale · areas: rust-sdk, fallback · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Preserve the underlying TLS configuration error in diagnostics**" — 2 days old
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#4932 chore(platform)!: bump rust-dashcore to 719de34b (secp256k1 0.33)](https://github.com/dashpay/platform/pull/4932) — 1 unresolved (1 bot) · 7 days stale · ⚠ merge conflict · ✋ changes requested · areas: rs-drive-abci, dpp, rs-platform-wallet, rs-platform-wallet-ffi, rust-sdk, rust-sdk-ffi, js-wasm-sdk, github, fallback · Policy: waiting-author
  - Top thread: "🔴 Blocking: Initialize WebCrypto for supported Node 18 WASM consumers**" — 7 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/dash-evo-tool#853 fix: show parsed platform-address transitions](https://github.com/dashpay/dash-evo-tool/pull/853) — via @thepastaclaw · 1 unresolved (1 CodeRabbit) · 7 days stale · 🔴 CI failing · areas: fallback · Policy: waiting-author
  - Top thread: "_🎯 Functional Correctness_ \| _🟠 Major_ \| _⚡ Quick win_" — 7 days old
  - Blocker: coderabbitai left review threads unresolved; resolve them
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5231 fix(dashmate): only block stops for DKGs this masternode is in](https://github.com/dashpay/platform/pull/5231) — 1 unresolved (1 bot) · 0 days stale · ✋ changes requested · 🔴 CI failing · areas: dashmate · Policy: waiting-author
  - Top thread: "🔴 Blocking: Fail closed until Core confirms start-at-tip session membership**" — 0 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#4298 fix(dashmate): load ZeroSSL config in force mode](https://github.com/dashpay/platform/pull/4298) — via @thepastaclaw · ⚠ merge conflict · 🐢 targets v5.1-dev · areas: dashmate
- [dashpay/platform#4432 fix(sdk): mirror the COUNT dispatcher's outer-walk limit when verifying range-outer carrier proofs](https://github.com/dashpay/platform/pull/4432) — areas: rs-drive, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 2c0234e378cd4dd3ca2c1f3f40317eddfb454374 after bot completion
- [dashpay/platform#4615 feat(sdk)!: optional BIP-39 passphrase through the mnemonic resolver and wallet creation](https://github.com/dashpay/platform/pull/4615) — ⚠ merge conflict · 📝 draft · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, rust-sdk-ffi, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#4965 test(sdk): cargo-fuzz harness over drive-proof-verifier FromProof](https://github.com/dashpay/platform/pull/4965) — 📝 draft · areas: github, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5154 test(drive): track latest protocol version in grovedb structure snapshot](https://github.com/dashpay/platform/pull/5154) — 🔴 CI failing · 🐢 targets v5.1-dev · areas: rs-drive
- [dashpay/platform#5169 feat(sdk): let document writes state the action fee agreement](https://github.com/dashpay/platform/pull/5169) — areas: rust-sdk, js-wasm-sdk · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed b720c0bfa4340e66e34854b0049afac61a0bb69a after bot completion
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/platform#5212 fix(platform): point PLATFORM_V15 at query versions V2 after v4.2 merge](https://github.com/dashpay/platform/pull/5212) — 🔴 CI failing · 🐢 targets v5.1-dev · areas: rs-platform-wallet, fallback
- [dashpay/platform#5236 fix(sdk): refresh stale quorum keys instead of banning every node](https://github.com/dashpay/platform/pull/5236) — areas: rust-sdk, js-wasm-sdk · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/rust-dashcore#749 fix(ffi): free transaction byte buffers correctly](https://github.com/dashpay/rust-dashcore/pull/749) — via @thepastaclaw · ⚠ merge conflict · 🔴 CI failing · areas: fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed a7d48fb68ca3feb61b15d969085c8b78befc0745 after bot completion
- [dashpay/rust-dashcore#752 fix(ffi): validate account seed lengths](https://github.com/dashpay/rust-dashcore/pull/752) — via @thepastaclaw · ⚠ merge conflict · areas: fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 94918c7f1085cf5b1548088b6fa6dc8473d87664 after bot completion
- [dashpay/rust-dashcore#904 fix(dash-spv): harden Windows dashd_sync startup wallet load](https://github.com/dashpay/rust-dashcore/pull/904) — via @thepastaclaw · ⚠ merge conflict · 📝 draft · areas: dash-spv · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#959 feat(dash-spv): adaptive gap-limit probe escalation for BIP44 discovery](https://github.com/dashpay/rust-dashcore/pull/959) — ⚠ merge conflict · 🔴 CI failing · areas: dash-spv, key-wallet, key-wallet-manager, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 7bcac9b9d89c2c4947080b2e6a15503e97eac9f4 after bot completion

<a id="pastapastapasta-needs-action"></a>
#### Needs action (11)
- [dashpay/platform#4844 feat(sdk)!: key limits, DIP-14 sub-feature derivation and decode-any-kind for DashPay Connect](https://github.com/dashpay/platform/pull/4844) — 6 unresolved (4 CodeRabbit, 2 bot) · 11 days stale · ⚠ merge conflict · 🔴 CI failing · areas: rs-drive-abci, dpp, rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, rust-sdk, rust-sdk-ffi, js-wasm-sdk, kotlin-sdk, fallback · Policy: waiting-author
  - Top thread: "🔴 Blocking: Initialize the native library before standalone parsing**" — 11 days old
  - Blocker: coderabbitai left review threads unresolved; resolve them
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#4623 feat(platform-wallet): reserve DashPay payout addresses without Core funding](https://github.com/dashpay/platform/pull/4623) — 2 unresolved (2 bot) · 13 days stale · ✋ changes requested · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Null the string out-param before the signer null-check**" — 13 days old
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/rust-dashcore#264 feat(dash-spv): Add BIP324 v2 encrypted P2P transport](https://github.com/dashpay/rust-dashcore/pull/264) — 2 unresolved (2 CodeRabbit) · 11 days stale · areas: dash-spv, fallback · Policy: waiting-author
  - Top thread: "_📐 Maintainability & Code Quality_ \| _🟡 Minor_ \| _⚡ Quick win_" — 11 days old
  - Blocker: coderabbitai requested changes on this head; dismiss the review or push a fix
  - Blocker: coderabbitai left review threads unresolved; resolve them
- [dashpay/platform#4392 perf(swift-sdk): linear wallet-changeset rounds via per-round bulk-prefetch cache](https://github.com/dashpay/platform/pull/4392) — 1 unresolved (1 bot) · 22 days stale · ⚠ merge conflict · ✋ changes requested · areas: swift-sdk · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Exercise transient pending-input fetch failures followed by recovery**" — 22 days old
  - Blocker: thepastaclaw left review threads unresolved; resolve them
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/platform#5160 feat(sdk): route DAPI connections through a SOCKS5 proxy](https://github.com/dashpay/platform/pull/5160) — 2 unresolved (2 bot) · 2 days stale · areas: rust-sdk, fallback · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Preserve the underlying TLS configuration error in diagnostics**" — 2 days old
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#4932 chore(platform)!: bump rust-dashcore to 719de34b (secp256k1 0.33)](https://github.com/dashpay/platform/pull/4932) — 1 unresolved (1 bot) · 7 days stale · ⚠ merge conflict · ✋ changes requested · areas: rs-drive-abci, dpp, rs-platform-wallet, rs-platform-wallet-ffi, rust-sdk, rust-sdk-ffi, js-wasm-sdk, github, fallback · Policy: waiting-author
  - Top thread: "🔴 Blocking: Initialize WebCrypto for supported Node 18 WASM consumers**" — 7 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/dash-evo-tool#853 fix: show parsed platform-address transitions](https://github.com/dashpay/dash-evo-tool/pull/853) — via @thepastaclaw · 1 unresolved (1 CodeRabbit) · 7 days stale · 🔴 CI failing · areas: fallback · Policy: waiting-author
  - Top thread: "_🎯 Functional Correctness_ \| _🟠 Major_ \| _⚡ Quick win_" — 7 days old
  - Blocker: coderabbitai left review threads unresolved; resolve them
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5231 fix(dashmate): only block stops for DKGs this masternode is in](https://github.com/dashpay/platform/pull/5231) — 1 unresolved (1 bot) · 0 days stale · ✋ changes requested · 🔴 CI failing · areas: dashmate · Policy: waiting-author
  - Top thread: "🔴 Blocking: Fail closed until Core confirms start-at-tip session membership**" — 0 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/rust-dashcore#749 fix(ffi): free transaction byte buffers correctly](https://github.com/dashpay/rust-dashcore/pull/749) — via @thepastaclaw · ⚠ merge conflict · 🔴 CI failing · areas: fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed a7d48fb68ca3feb61b15d969085c8b78befc0745 after bot completion
- [dashpay/rust-dashcore#752 fix(ffi): validate account seed lengths](https://github.com/dashpay/rust-dashcore/pull/752) — via @thepastaclaw · ⚠ merge conflict · areas: fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 94918c7f1085cf5b1548088b6fa6dc8473d87664 after bot completion
- [dashpay/rust-dashcore#959 feat(dash-spv): adaptive gap-limit probe escalation for BIP44 discovery](https://github.com/dashpay/rust-dashcore/pull/959) — ⚠ merge conflict · 🔴 CI failing · areas: dash-spv, key-wallet, key-wallet-manager, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 7bcac9b9d89c2c4947080b2e6a15503e97eac9f4 after bot completion

<a id="pastapastapasta-unresolved-comments"></a>
#### Unresolved Comments (9)
- [dashpay/platform#4844 feat(sdk)!: key limits, DIP-14 sub-feature derivation and decode-any-kind for DashPay Connect](https://github.com/dashpay/platform/pull/4844) — 6 unresolved (4 CodeRabbit, 2 bot) · 11 days stale · ⚠ merge conflict · 🔴 CI failing · areas: rs-drive-abci, dpp, rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, rust-sdk, rust-sdk-ffi, js-wasm-sdk, kotlin-sdk, fallback · Policy: waiting-author
  - Top thread: "🔴 Blocking: Initialize the native library before standalone parsing**" — 11 days old
  - Blocker: coderabbitai left review threads unresolved; resolve them
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#4623 feat(platform-wallet): reserve DashPay payout addresses without Core funding](https://github.com/dashpay/platform/pull/4623) — 2 unresolved (2 bot) · 13 days stale · ✋ changes requested · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Null the string out-param before the signer null-check**" — 13 days old
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/rust-dashcore#264 feat(dash-spv): Add BIP324 v2 encrypted P2P transport](https://github.com/dashpay/rust-dashcore/pull/264) — 2 unresolved (2 CodeRabbit) · 11 days stale · areas: dash-spv, fallback · Policy: waiting-author
  - Top thread: "_📐 Maintainability & Code Quality_ \| _🟡 Minor_ \| _⚡ Quick win_" — 11 days old
  - Blocker: coderabbitai requested changes on this head; dismiss the review or push a fix
  - Blocker: coderabbitai left review threads unresolved; resolve them
- [dashpay/dash-evo-tool#859 perf: detect system theme off UI thread](https://github.com/dashpay/dash-evo-tool/pull/859) — via @thepastaclaw · 2 unresolved (1 CodeRabbit, 1 human) · 7 days stale · areas: fallback · Policy: waiting-author
  - Top thread: "we don't really want to grow the ThemeState. Is there no simpler solution? Also check architecture documents and put the…" — 7 days old
  - Blocker: coderabbitai left review threads unresolved; resolve them
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#4392 perf(swift-sdk): linear wallet-changeset rounds via per-round bulk-prefetch cache](https://github.com/dashpay/platform/pull/4392) — 1 unresolved (1 bot) · 22 days stale · ⚠ merge conflict · ✋ changes requested · areas: swift-sdk · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Exercise transient pending-input fetch failures followed by recovery**" — 22 days old
  - Blocker: thepastaclaw left review threads unresolved; resolve them
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/platform#5160 feat(sdk): route DAPI connections through a SOCKS5 proxy](https://github.com/dashpay/platform/pull/5160) — 2 unresolved (2 bot) · 2 days stale · areas: rust-sdk, fallback · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Preserve the underlying TLS configuration error in diagnostics**" — 2 days old
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#4932 chore(platform)!: bump rust-dashcore to 719de34b (secp256k1 0.33)](https://github.com/dashpay/platform/pull/4932) — 1 unresolved (1 bot) · 7 days stale · ⚠ merge conflict · ✋ changes requested · areas: rs-drive-abci, dpp, rs-platform-wallet, rs-platform-wallet-ffi, rust-sdk, rust-sdk-ffi, js-wasm-sdk, github, fallback · Policy: waiting-author
  - Top thread: "🔴 Blocking: Initialize WebCrypto for supported Node 18 WASM consumers**" — 7 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/dash-evo-tool#853 fix: show parsed platform-address transitions](https://github.com/dashpay/dash-evo-tool/pull/853) — via @thepastaclaw · 1 unresolved (1 CodeRabbit) · 7 days stale · 🔴 CI failing · areas: fallback · Policy: waiting-author
  - Top thread: "_🎯 Functional Correctness_ \| _🟠 Major_ \| _⚡ Quick win_" — 7 days old
  - Blocker: coderabbitai left review threads unresolved; resolve them
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5231 fix(dashmate): only block stops for DKGs this masternode is in](https://github.com/dashpay/platform/pull/5231) — 1 unresolved (1 bot) · 0 days stale · ✋ changes requested · 🔴 CI failing · areas: dashmate · Policy: waiting-author
  - Top thread: "🔴 Blocking: Fail closed until Core confirms start-at-tip session membership**" — 0 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them

<a id="pastapastapasta-ci-failing"></a>
#### CI Failing (2)
- [dashpay/rust-dashcore#749 fix(ffi): free transaction byte buffers correctly](https://github.com/dashpay/rust-dashcore/pull/749) — via @thepastaclaw · ⚠ merge conflict · 🔴 CI failing · areas: fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed a7d48fb68ca3feb61b15d969085c8b78befc0745 after bot completion
- [dashpay/rust-dashcore#959 feat(dash-spv): adaptive gap-limit probe escalation for BIP44 discovery](https://github.com/dashpay/rust-dashcore/pull/959) — ⚠ merge conflict · 🔴 CI failing · areas: dash-spv, key-wallet, key-wallet-manager, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 7bcac9b9d89c2c4947080b2e6a15503e97eac9f4 after bot completion

<a id="pastapastapasta-draft"></a>
#### Draft (5)
- [dashpay/rust-dashcore#901 fix(key-wallet): discover Coinbase/AssetUnlock outputs for all fund-bearing accounts](https://github.com/dashpay/rust-dashcore/pull/901) — via @thepastaclaw · 2 unresolved (2 CodeRabbit) · 56 days stale · ⚠ merge conflict · ✋ changes requested · 📝 draft · areas: dash-spv, key-wallet · Policy: draft
  - Top thread: "_🎯 Functional Correctness_ \| _🟡 Minor_ \| _⚡ Quick win_" — 56 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#4538 fix(sdk): honor explicit regtest addresses in dapi-client; tolerate empty discovery in wasm-sdk trusted context](https://github.com/dashpay/platform/pull/4538) — 2 unresolved (2 bot) · 32 days stale · ⚠ merge conflict · 📝 draft · areas: js-wasm-sdk, fallback · Policy: draft
  - Top thread: "🟡 Suggestion: Add deterministic tests for the network-scoped discovery fallback**" — 32 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#4615 feat(sdk)!: optional BIP-39 passphrase through the mnemonic resolver and wallet creation](https://github.com/dashpay/platform/pull/4615) — ⚠ merge conflict · 📝 draft · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, rust-sdk-ffi, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#4965 test(sdk): cargo-fuzz harness over drive-proof-verifier FromProof](https://github.com/dashpay/platform/pull/4965) — 📝 draft · areas: github, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#904 fix(dash-spv): harden Windows dashd_sync startup wallet load](https://github.com/dashpay/rust-dashcore/pull/904) — via @thepastaclaw · ⚠ merge conflict · 📝 draft · areas: dash-spv · Policy: draft
  - Blocker: Draft PR does not occupy a review slot

<a id="pastapastapasta-stale"></a>
#### Stale (11)
- [dashpay/platform#3096 feat(sdk): add client-side validation to state transition construction methods](https://github.com/dashpay/platform/pull/3096) — via @thepastaclaw · 4 unresolved (2 CodeRabbit, 2 human) · 225 days stale · ⚠ merge conflict · 🐢 targets v5.1-dev · areas: rs-drive-abci, dpp, js-wasm-sdk, fallback
  - Top thread: "_⚠️ Potential issue_ \| _🟡 Minor_" — 222 days old
- [dashpay/rust-dashcore#137 fix: integration tests, continued dashificiation](https://github.com/dashpay/rust-dashcore/pull/137) — 2 unresolved (2 CodeRabbit) · 391 days stale · ⚠ merge conflict · 🔴 CI failing · 🐢 targets v0.40-dev, untouched 294 days · areas: fallback
  - Top thread: "_🛠️ Refactor suggestion_" — 391 days old
- [dashpay/platform#4530 test(dashmate): live state sync e2e — join tooling, churn, re-sync and fallback coverage](https://github.com/dashpay/platform/pull/4530) — 6 unresolved (6 bot) · 32 days stale · ⚠ merge conflict · 📝 draft · 🐢 targets v5.1-dev · areas: rs-drive, rs-drive-abci, dpp, dashmate, js-wasm-sdk, github, fallback
  - Top thread: "🟡 Suggestion: Require the churn scenario to interrupt an active state sync**" — 32 days old
- [dashpay/platform#4616 feat(platform-wallet): support DashPay shielded tips with dedicated accounts](https://github.com/dashpay/platform/pull/4616) — 7 unresolved (2 CodeRabbit, 5 bot) · 15 days stale · ⚠ merge conflict · 🐢 targets v5.1-dev · areas: rs-drive, rs-drive-abci, rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk, rust-sdk, system-contracts, kotlin-sdk, fallback
  - Top thread: "_🗄️ Data Integrity & Integration_ \| _🟠 Major_ \| _🏗️ Heavy lift_" — 15 days old
- [dashpay/platform#4633 feat(sdk): add dash-platform-cxx, a thin CXX shell over dash-sdk for C++ embedders](https://github.com/dashpay/platform/pull/4633) — 10 unresolved (10 bot) · 2 days stale · ✋ changes requested · 🐢 targets feat/dapi-client-socks5-proxy · areas: github, fallback
  - Top thread: "🟡 Suggestion: Reject contradictory recipient IDs before building a contact request**" — 2 days old
- [dashpay/platform#4648 feat(drive-abci)!: state sync via ABCI snapshots with reduced platform state (protocol v15)](https://github.com/dashpay/platform/pull/4648) — 3 unresolved (3 bot) · 20 days stale · ⚠ merge conflict · 🔴 CI failing · 🐢 targets v5.1-dev · areas: rs-drive, rs-drive-abci, dpp, fallback
  - Top thread: "🟡 Suggestion: Do not silently skip unsupported trees beyond a full discovery page**" — 20 days old
- [dashpay/platform#4521 feat(dashmate): state sync configuration for tenderdash and drive snapshots](https://github.com/dashpay/platform/pull/4521) — 2 unresolved (2 bot) · 32 days stale · 📝 draft · 🐢 targets v5.1-dev · areas: dashmate
  - Top thread: "🔴 Blocking: Default snapshot serving exposes unbounded remote checkpoint retention**" — 32 days old
- [dashpay/platform#4539 fix(dashmate): refresh quorum-server seeds and complete node identities](https://github.com/dashpay/platform/pull/4539) — 2 unresolved (1 CodeRabbit, 1 bot) · 15 days stale · 🐢 targets v5.1-dev · areas: dashmate, github, fallback
  - Top thread: "_🩺 Stability & Availability_ \| _🟡 Minor_ \| _⚡ Quick win_" — 15 days old
- [dashpay/platform#4298 fix(dashmate): load ZeroSSL config in force mode](https://github.com/dashpay/platform/pull/4298) — via @thepastaclaw · ⚠ merge conflict · 🐢 targets v5.1-dev · areas: dashmate
- [dashpay/platform#5154 test(drive): track latest protocol version in grovedb structure snapshot](https://github.com/dashpay/platform/pull/5154) — 🔴 CI failing · 🐢 targets v5.1-dev · areas: rs-drive
- [dashpay/platform#5212 fix(platform): point PLATFORM_V15 at query versions V2 after v4.2 merge](https://github.com/dashpay/platform/pull/5212) — 🔴 CI failing · 🐢 targets v5.1-dev · areas: rs-platform-wallet, fallback

<a id="pastapastapasta-clean"></a>
#### Clean (4)
- [dashpay/platform#4432 fix(sdk): mirror the COUNT dispatcher's outer-walk limit when verifying range-outer carrier proofs](https://github.com/dashpay/platform/pull/4432) — areas: rs-drive, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 2c0234e378cd4dd3ca2c1f3f40317eddfb454374 after bot completion
- [dashpay/platform#5169 feat(sdk): let document writes state the action fee agreement](https://github.com/dashpay/platform/pull/5169) — areas: rust-sdk, js-wasm-sdk · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed b720c0bfa4340e66e34854b0049afac61a0bb69a after bot completion
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/platform#5236 fix(sdk): refresh stale quorum keys instead of banning every node](https://github.com/dashpay/platform/pull/5236) — areas: rust-sdk, js-wasm-sdk · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/rust-dashcore#752 fix(ffi): validate account seed lengths](https://github.com/dashpay/rust-dashcore/pull/752) — via @thepastaclaw · ⚠ merge conflict · areas: fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 94918c7f1085cf5b1548088b6fa6dc8473d87664 after bot completion

<a id="lklimek"></a>
### @lklimek + (@Claudius-Maginificent)
<a id="lklimek-open"></a>
#### Open (19)
- [dashpay/platform#3549 test(platform-wallet): e2e framework + full test suite — triage pins, Found-*/PA-* guards, fail-closed persist, Stage-2 merge](https://github.com/dashpay/platform/pull/3549) — via @Claudius-Maginificent · 9 unresolved (9 bot) · 151 days stale · ⚠ merge conflict · 📝 draft · 🐢 targets v5.1-dev · areas: rs-drive, rs-drive-abci, rs-platform-wallet, rs-platform-wallet-ffi, rust-sdk, github, fallback
  - Top thread: "🟡 Suggestion: `from_seed_for_identity` is misleadingly named, half-functional, and unused**" — 151 days old
- [dashpay/dash-evo-tool#901 feat(dpns): unify safe masternode voting operations](https://github.com/dashpay/dash-evo-tool/pull/901) — 12 unresolved (12 bot) · 13 days stale · ⚠ merge conflict · areas: fallback · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Outcome classified twice; empty Confirmed match arm**" — 13 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/rust-dashcore#1082 fix(wallet): reconcile late inputs and publish accounting corrections](https://github.com/dashpay/rust-dashcore/pull/1082) — 7 unresolved (7 human) · 1 days stale · ✋ changes requested · 🔴 CI failing · areas: key-wallet, key-wallet-manager · Policy: waiting-author
  - Top thread: "The state is edited by hand (account 1's record and UTXOs removed) and the child is mined while its parent is still unco…" — 1 days old
  - Blocker: Author response is required after the latest human objection
- [dashpay/dash-evo-tool#994 fix(wallet): safe upgrade backups with time-based retention and guarded file deletion](https://github.com/dashpay/dash-evo-tool/pull/994) — via @Claudius-Maginificent · 9 unresolved (5 human, 4 bot) · 1 days stale · ✋ changes requested · areas: fallback · Policy: waiting-author
  - Top thread: "SEC-005** · MEDIUM · BLOCKING — **The new lifecycle lock file is an undeletable single point of failure with a misleadin…" — 1 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/dash-evo-tool#1038 feat(identity): use profile display names and improve identity selection](https://github.com/dashpay/dash-evo-tool/pull/1038) — 4 unresolved (1 human, 3 bot) · 0 days stale · ✋ changes requested · areas: fallback · Policy: waiting-author
  - Top thread: "SEC-001** · MEDIUM — **Avatar fetch enforces HTTPS only on the initial URL; redirects can downgrade to http:// and reach…" — 0 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5220 fix(platform-wallet)!: replay recorded transaction history on load for every persister](https://github.com/dashpay/platform/pull/5220) — 4 unresolved (4 bot) · 1 days stale · ✋ changes requested · 🐢 targets fix/pr-5126 · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk, fallback
  - Top thread: "🔴 Blocking: Do not skip history merely because a raw-restored transaction record exists**" — 1 days old
- [dashpay/tenderdash#1523 fix(consensus): never extend a relock precommit without ProcessProposal for the round](https://github.com/dashpay/tenderdash/pull/1523) — 2 unresolved (2 bot) · 1 days stale · ✋ changes requested · areas: fallback · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Relock ValidBlock invariant asserted for only one arrival ordering**" — 1 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/tenderdash#1526 chore(ci): use main version of claudius-review-action](https://github.com/dashpay/tenderdash/pull/1526) — 2 unresolved (1 human, 1 bot) · 0 days stale · areas: github · Policy: waiting-author
  - Top thread: "SEC-001** · MEDIUM — **Third-party action unpinned from commit SHA to mutable @​main, reversing a deliberate prior pin**" — 0 days old
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/tenderdash#1520 fix(consensus): verify commit vote extensions before a commit is taken](https://github.com/dashpay/tenderdash/pull/1520) — 1 unresolved (1 bot) · 1 days stale · areas: github, fallback · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Avoid quadratic candidate insertion during WAL replay**" — 1 days old
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/dash-evo-tool#1020 fix(wallet): give access-denied migrations their own guidance](https://github.com/dashpay/dash-evo-tool/pull/1020) — via @Claudius-Maginificent · ⚠ merge conflict · 🐢 targets fix/release-review-blocking-findings · areas: fallback
- [dashpay/dash-evo-tool#1028 fix(wallet): surface unresolved Platform funding transfers](https://github.com/dashpay/dash-evo-tool/pull/1028) — 📝 draft · areas: github, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/dash-evo-tool#1035 fix(wallet): resolve saved asset-lock keys through platform-wallet](https://github.com/dashpay/dash-evo-tool/pull/1035) — 📝 draft · 🐢 targets fix/orphaned-tx · areas: fallback
- [dashpay/dash-evo-tool#1036 fix(wallet): preserve consumed asset-lock reports with SQLite](https://github.com/dashpay/dash-evo-tool/pull/1036) — 📝 draft · 🐢 targets fix/orphaned-tx · areas: fallback
- [dashpay/platform#4740 fix(platform-wallet): rescan DashPay contact accounts from the contact request height](https://github.com/dashpay/platform/pull/4740) — 🐢 targets v5.1-dev · areas: rs-platform-wallet, fallback
- [dashpay/platform#5150 fix(platform-wallet)!: restore Core spending state and repair persisted accounting](https://github.com/dashpay/platform/pull/5150) — ✋ changes requested · 🐢 targets v5.1-dev · areas: rs-drive, rs-platform-wallet, wallet-storage, swift-sdk, fallback
- [dashpay/platform#5207 fix(wallet): require only atomic tracked-lock writes for reconciliation](https://github.com/dashpay/platform/pull/5207) — ⚠ merge conflict · 🔴 CI failing · 📝 draft · 🐢 targets fix/pr-5126 · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, fallback
- [dashpay/platform#5210 feat(wallet-storage): restore complete Core wallet snapshots from SQLite](https://github.com/dashpay/platform/pull/5210) — 📝 draft · 🐢 targets fix/sqlite-asset-lock-reconciliation · areas: rs-platform-wallet, wallet-storage, swift-sdk, fallback
- [dashpay/tenderdash#1513 fix(service)!: share worker lifecycle for consensus shutdown](https://github.com/dashpay/tenderdash/pull/1513) — 🔴 CI failing · areas: fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed b8a6bebd2654465a996515b6303f124052d8a860 after bot completion
- [dashpay/tenderdash#1515 fix(service)!: join background work before shutdown completes](https://github.com/dashpay/tenderdash/pull/1515) — 🔴 CI failing · 📝 draft · areas: fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot

<a id="lklimek-needs-action"></a>
#### Needs action (7)
- [dashpay/dash-evo-tool#901 feat(dpns): unify safe masternode voting operations](https://github.com/dashpay/dash-evo-tool/pull/901) — 12 unresolved (12 bot) · 13 days stale · ⚠ merge conflict · areas: fallback · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Outcome classified twice; empty Confirmed match arm**" — 13 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/rust-dashcore#1082 fix(wallet): reconcile late inputs and publish accounting corrections](https://github.com/dashpay/rust-dashcore/pull/1082) — 7 unresolved (7 human) · 1 days stale · ✋ changes requested · 🔴 CI failing · areas: key-wallet, key-wallet-manager · Policy: waiting-author
  - Top thread: "The state is edited by hand (account 1's record and UTXOs removed) and the child is mined while its parent is still unco…" — 1 days old
  - Blocker: Author response is required after the latest human objection
- [dashpay/dash-evo-tool#994 fix(wallet): safe upgrade backups with time-based retention and guarded file deletion](https://github.com/dashpay/dash-evo-tool/pull/994) — via @Claudius-Maginificent · 9 unresolved (5 human, 4 bot) · 1 days stale · ✋ changes requested · areas: fallback · Policy: waiting-author
  - Top thread: "SEC-005** · MEDIUM · BLOCKING — **The new lifecycle lock file is an undeletable single point of failure with a misleadin…" — 1 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/dash-evo-tool#1038 feat(identity): use profile display names and improve identity selection](https://github.com/dashpay/dash-evo-tool/pull/1038) — 4 unresolved (1 human, 3 bot) · 0 days stale · ✋ changes requested · areas: fallback · Policy: waiting-author
  - Top thread: "SEC-001** · MEDIUM — **Avatar fetch enforces HTTPS only on the initial URL; redirects can downgrade to http:// and reach…" — 0 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/tenderdash#1523 fix(consensus): never extend a relock precommit without ProcessProposal for the round](https://github.com/dashpay/tenderdash/pull/1523) — 2 unresolved (2 bot) · 1 days stale · ✋ changes requested · areas: fallback · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Relock ValidBlock invariant asserted for only one arrival ordering**" — 1 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/tenderdash#1526 chore(ci): use main version of claudius-review-action](https://github.com/dashpay/tenderdash/pull/1526) — 2 unresolved (1 human, 1 bot) · 0 days stale · areas: github · Policy: waiting-author
  - Top thread: "SEC-001** · MEDIUM — **Third-party action unpinned from commit SHA to mutable @​main, reversing a deliberate prior pin**" — 0 days old
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/tenderdash#1520 fix(consensus): verify commit vote extensions before a commit is taken](https://github.com/dashpay/tenderdash/pull/1520) — 1 unresolved (1 bot) · 1 days stale · areas: github, fallback · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Avoid quadratic candidate insertion during WAL replay**" — 1 days old
  - Blocker: thepastaclaw left review threads unresolved; resolve them

<a id="lklimek-unresolved-comments"></a>
#### Unresolved Comments (7)
- [dashpay/dash-evo-tool#901 feat(dpns): unify safe masternode voting operations](https://github.com/dashpay/dash-evo-tool/pull/901) — 12 unresolved (12 bot) · 13 days stale · ⚠ merge conflict · areas: fallback · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Outcome classified twice; empty Confirmed match arm**" — 13 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/rust-dashcore#1082 fix(wallet): reconcile late inputs and publish accounting corrections](https://github.com/dashpay/rust-dashcore/pull/1082) — 7 unresolved (7 human) · 1 days stale · ✋ changes requested · 🔴 CI failing · areas: key-wallet, key-wallet-manager · Policy: waiting-author
  - Top thread: "The state is edited by hand (account 1's record and UTXOs removed) and the child is mined while its parent is still unco…" — 1 days old
  - Blocker: Author response is required after the latest human objection
- [dashpay/dash-evo-tool#994 fix(wallet): safe upgrade backups with time-based retention and guarded file deletion](https://github.com/dashpay/dash-evo-tool/pull/994) — via @Claudius-Maginificent · 9 unresolved (5 human, 4 bot) · 1 days stale · ✋ changes requested · areas: fallback · Policy: waiting-author
  - Top thread: "SEC-005** · MEDIUM · BLOCKING — **The new lifecycle lock file is an undeletable single point of failure with a misleadin…" — 1 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/dash-evo-tool#1038 feat(identity): use profile display names and improve identity selection](https://github.com/dashpay/dash-evo-tool/pull/1038) — 4 unresolved (1 human, 3 bot) · 0 days stale · ✋ changes requested · areas: fallback · Policy: waiting-author
  - Top thread: "SEC-001** · MEDIUM — **Avatar fetch enforces HTTPS only on the initial URL; redirects can downgrade to http:// and reach…" — 0 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/tenderdash#1523 fix(consensus): never extend a relock precommit without ProcessProposal for the round](https://github.com/dashpay/tenderdash/pull/1523) — 2 unresolved (2 bot) · 1 days stale · ✋ changes requested · areas: fallback · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Relock ValidBlock invariant asserted for only one arrival ordering**" — 1 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/tenderdash#1526 chore(ci): use main version of claudius-review-action](https://github.com/dashpay/tenderdash/pull/1526) — 2 unresolved (1 human, 1 bot) · 0 days stale · areas: github · Policy: waiting-author
  - Top thread: "SEC-001** · MEDIUM — **Third-party action unpinned from commit SHA to mutable @​main, reversing a deliberate prior pin**" — 0 days old
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/tenderdash#1520 fix(consensus): verify commit vote extensions before a commit is taken](https://github.com/dashpay/tenderdash/pull/1520) — 1 unresolved (1 bot) · 1 days stale · areas: github, fallback · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Avoid quadratic candidate insertion during WAL replay**" — 1 days old
  - Blocker: thepastaclaw left review threads unresolved; resolve them

<a id="lklimek-ci-failing"></a>
#### CI Failing (1)
- [dashpay/tenderdash#1513 fix(service)!: share worker lifecycle for consensus shutdown](https://github.com/dashpay/tenderdash/pull/1513) — 🔴 CI failing · areas: fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed b8a6bebd2654465a996515b6303f124052d8a860 after bot completion

<a id="lklimek-draft"></a>
#### Draft (2)
- [dashpay/dash-evo-tool#1028 fix(wallet): surface unresolved Platform funding transfers](https://github.com/dashpay/dash-evo-tool/pull/1028) — 📝 draft · areas: github, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/tenderdash#1515 fix(service)!: join background work before shutdown completes](https://github.com/dashpay/tenderdash/pull/1515) — 🔴 CI failing · 📝 draft · areas: fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot

<a id="lklimek-stale"></a>
#### Stale (9)
- [dashpay/platform#3549 test(platform-wallet): e2e framework + full test suite — triage pins, Found-*/PA-* guards, fail-closed persist, Stage-2 merge](https://github.com/dashpay/platform/pull/3549) — via @Claudius-Maginificent · 9 unresolved (9 bot) · 151 days stale · ⚠ merge conflict · 📝 draft · 🐢 targets v5.1-dev · areas: rs-drive, rs-drive-abci, rs-platform-wallet, rs-platform-wallet-ffi, rust-sdk, github, fallback
  - Top thread: "🟡 Suggestion: `from_seed_for_identity` is misleadingly named, half-functional, and unused**" — 151 days old
- [dashpay/platform#5220 fix(platform-wallet)!: replay recorded transaction history on load for every persister](https://github.com/dashpay/platform/pull/5220) — 4 unresolved (4 bot) · 1 days stale · ✋ changes requested · 🐢 targets fix/pr-5126 · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk, fallback
  - Top thread: "🔴 Blocking: Do not skip history merely because a raw-restored transaction record exists**" — 1 days old
- [dashpay/dash-evo-tool#1020 fix(wallet): give access-denied migrations their own guidance](https://github.com/dashpay/dash-evo-tool/pull/1020) — via @Claudius-Maginificent · ⚠ merge conflict · 🐢 targets fix/release-review-blocking-findings · areas: fallback
- [dashpay/dash-evo-tool#1035 fix(wallet): resolve saved asset-lock keys through platform-wallet](https://github.com/dashpay/dash-evo-tool/pull/1035) — 📝 draft · 🐢 targets fix/orphaned-tx · areas: fallback
- [dashpay/dash-evo-tool#1036 fix(wallet): preserve consumed asset-lock reports with SQLite](https://github.com/dashpay/dash-evo-tool/pull/1036) — 📝 draft · 🐢 targets fix/orphaned-tx · areas: fallback
- [dashpay/platform#4740 fix(platform-wallet): rescan DashPay contact accounts from the contact request height](https://github.com/dashpay/platform/pull/4740) — 🐢 targets v5.1-dev · areas: rs-platform-wallet, fallback
- [dashpay/platform#5150 fix(platform-wallet)!: restore Core spending state and repair persisted accounting](https://github.com/dashpay/platform/pull/5150) — ✋ changes requested · 🐢 targets v5.1-dev · areas: rs-drive, rs-platform-wallet, wallet-storage, swift-sdk, fallback
- [dashpay/platform#5207 fix(wallet): require only atomic tracked-lock writes for reconciliation](https://github.com/dashpay/platform/pull/5207) — ⚠ merge conflict · 🔴 CI failing · 📝 draft · 🐢 targets fix/pr-5126 · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, fallback
- [dashpay/platform#5210 feat(wallet-storage): restore complete Core wallet snapshots from SQLite](https://github.com/dashpay/platform/pull/5210) — 📝 draft · 🐢 targets fix/sqlite-asset-lock-reconciliation · areas: rs-platform-wallet, wallet-storage, swift-sdk, fallback

<a id="lklimek-ready-for-review"></a>
#### Ready for Review (3)
- [dashpay/platform#5026 fix(platform-wallet)!: persist DashPay coreHeight backfill coverage so a relaunch resumes instead of rewinding again](https://github.com/dashpay/platform/pull/5026) — by @HashEngineering · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, kotlin-sdk, fallback · Policy: waiting-bots
  - Blocker: coderabbitai has not reported for the current head
- [dashpay/platform#5169 feat(sdk): let document writes state the action fee agreement](https://github.com/dashpay/platform/pull/5169) — by @PastaPastaPasta · areas: rust-sdk, js-wasm-sdk · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed b720c0bfa4340e66e34854b0049afac61a0bb69a after bot completion
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/platform#5236 fix(sdk): refresh stale quorum keys instead of banning every node](https://github.com/dashpay/platform/pull/5236) — by @PastaPastaPasta · areas: rust-sdk, js-wasm-sdk · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return

<a id="shumkov"></a>
### @shumkov
<a id="shumkov-open"></a>
#### Open (14)
- [dashpay/rust-dashcore#634 feat(key-wallet-manager): single-map WalletManager with Arc<RwLock<T>> per wallet](https://github.com/dashpay/rust-dashcore/pull/634) — 20 unresolved (20 CodeRabbit) · 176 days stale · ⚠ merge conflict · ✋ changes requested · areas: dash-spv, key-wallet, key-wallet-manager, fallback · Policy: waiting-author
  - Top thread: "_⚠️ Potential issue_ \| _🟡 Minor_" — 176 days old
  - Blocker: coderabbitai requested changes on this head; dismiss the review or push a fix
  - Blocker: coderabbitai left review threads unresolved; resolve them
- [dashpay/platform#4282 fix(dashmate)!: give Debian packages versions apt can order](https://github.com/dashpay/platform/pull/4282) — 6 unresolved (1 CodeRabbit, 5 bot) · 30 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · 🐢 targets v5.1-dev · areas: dashmate, js-wasm-sdk, system-contracts, github, fallback
  - Top thread: "_🗄️ Data Integrity & Integration_ \| _🟡 Minor_ \| _⚡ Quick win_" — 30 days old
- [dashpay/platform#3936 chore(drive-abci): update to nested address in SML](https://github.com/dashpay/platform/pull/3936) — 2 unresolved (2 bot) · 104 days stale · ⚠ merge conflict · 📝 draft · 🐢 targets v5.1-dev · areas: rs-drive-abci, rust-sdk, fallback
  - Top thread: "🟡 Suggestion: V0 reverse synthesizes `addresses` from a Core-22 port, masking subsequent legacy port-change diffs after …" — 104 days old
- [dashpay/platform#4283 fix(dashmate): keep the node up when an image pull fails, and stop reporting success when it did not](https://github.com/dashpay/platform/pull/4283) — 2 unresolved (2 bot) · 24 days stale · ⚠ merge conflict · ✋ changes requested · 🐢 targets v5.1-dev · areas: dashmate
  - Top thread: "🟡 Suggestion: providers.js sanitizer misses U+061C (Arabic letter mark) that sanitizeRemoteText covers**" — 24 days old
- [dashpay/platform#4660 feat(sdk): expose the document erase and history lifecycle on mobile and FFI](https://github.com/dashpay/platform/pull/4660) — 2 unresolved (2 bot) · 8 days stale · ✋ changes requested · 🔴 CI failing · 🐢 targets keep-history-lifecycle · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, rust-sdk-ffi, kotlin-sdk, fallback
  - Top thread: "🟡 Suggestion: Validate history selector discriminant at the C boundary**" — 8 days old
- [dashpay/platform#4657 feat(platform)!: delete and erase lifecycle for keep-history documents](https://github.com/dashpay/platform/pull/4657) — 2 unresolved (2 bot) · 8 days stale · ✋ changes requested · 🔴 CI failing · 🐢 targets keep-history-storage-v2 · areas: rs-drive, rs-drive-abci, dpp, rust-sdk, js-wasm-sdk, fallback
  - Top thread: "🟡 Suggestion: Deduplicate lifecycle containers at the Drive batch boundary**" — 8 days old
- [dashpay/rust-dashcore#1093 fix(rpc-json)!: parse Core v24 masternode list (shared/multi-payout masternodes, nested addresses)](https://github.com/dashpay/rust-dashcore/pull/1093) — 2 unresolved (2 CodeRabbit) · 1 days stale · ✋ changes requested · 🔴 CI failing · areas: fallback · Policy: waiting-author
  - Top thread: "_🎯 Functional Correctness_ \| _🟡 Minor_ \| _⚡ Quick win_" — 1 days old
  - Blocker: coderabbitai requested changes on this head; dismiss the review or push a fix
  - Blocker: coderabbitai left review threads unresolved; resolve them
- [dashpay/platform#4993 fix(drive-abci): a transition whose version is not active is not a decode failure](https://github.com/dashpay/platform/pull/4993) — 1 unresolved (1 bot) · 6 days stale · 🔴 CI failing · areas: rs-drive-abci · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Assert the client-visible metadata in the inactive-version regression test**" — 6 days old
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5072 fix(dpp): read $entropy as base64, not base58, in the create-transition map parser](https://github.com/dashpay/platform/pull/5072) — 1 unresolved (1 bot) · 2 days stale · areas: dpp · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Add coverage for the ambiguous unpadded base64/base58 case**" — 2 days old
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5227 fix(platform): keep Drive running on Core v24 masternode lists (shared and multi-payout masternodes)](https://github.com/dashpay/platform/pull/5227) — 1 unresolved (1 bot) · 0 days stale · 🔴 CI failing · areas: rs-drive-abci, dashmate, fallback · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Zero-address sentinel collapses Some([0;20\]) into None without a pinning test**" — 0 days old
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#4243 fix(sdk)!: complete encrypted txMetadata parity](https://github.com/dashpay/platform/pull/4243) — ⚠ merge conflict · 📝 draft · 🐢 targets v5.1-dev · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk, github, fallback
- [dashpay/platform#4285 feat(platform-wallet)!: invitation links are AppsFlyer applinks only](https://github.com/dashpay/platform/pull/4285) — ⚠ merge conflict · 🔴 CI failing · 📝 draft · 🐢 targets v5.1-dev · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk, fallback
- [dashpay/platform#4652 feat(drive)!: preserve composite document history across protocol activation](https://github.com/dashpay/platform/pull/4652) — ⚠ merge conflict · ✋ changes requested · 🐢 targets v5.1-dev · areas: rs-drive, rs-drive-abci, rust-sdk, js-wasm-sdk, fallback
- [dashpay/platform#5228 fix(drive-abci): take evonode platform ports from Core's nested addresses](https://github.com/dashpay/platform/pull/5228) — 🐢 targets fix/core-v24-masternode-list · areas: rs-drive-abci

<a id="shumkov-needs-action"></a>
#### Needs action (5)
- [dashpay/rust-dashcore#634 feat(key-wallet-manager): single-map WalletManager with Arc<RwLock<T>> per wallet](https://github.com/dashpay/rust-dashcore/pull/634) — 20 unresolved (20 CodeRabbit) · 176 days stale · ⚠ merge conflict · ✋ changes requested · areas: dash-spv, key-wallet, key-wallet-manager, fallback · Policy: waiting-author
  - Top thread: "_⚠️ Potential issue_ \| _🟡 Minor_" — 176 days old
  - Blocker: coderabbitai requested changes on this head; dismiss the review or push a fix
  - Blocker: coderabbitai left review threads unresolved; resolve them
- [dashpay/rust-dashcore#1093 fix(rpc-json)!: parse Core v24 masternode list (shared/multi-payout masternodes, nested addresses)](https://github.com/dashpay/rust-dashcore/pull/1093) — 2 unresolved (2 CodeRabbit) · 1 days stale · ✋ changes requested · 🔴 CI failing · areas: fallback · Policy: waiting-author
  - Top thread: "_🎯 Functional Correctness_ \| _🟡 Minor_ \| _⚡ Quick win_" — 1 days old
  - Blocker: coderabbitai requested changes on this head; dismiss the review or push a fix
  - Blocker: coderabbitai left review threads unresolved; resolve them
- [dashpay/platform#4993 fix(drive-abci): a transition whose version is not active is not a decode failure](https://github.com/dashpay/platform/pull/4993) — 1 unresolved (1 bot) · 6 days stale · 🔴 CI failing · areas: rs-drive-abci · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Assert the client-visible metadata in the inactive-version regression test**" — 6 days old
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5072 fix(dpp): read $entropy as base64, not base58, in the create-transition map parser](https://github.com/dashpay/platform/pull/5072) — 1 unresolved (1 bot) · 2 days stale · areas: dpp · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Add coverage for the ambiguous unpadded base64/base58 case**" — 2 days old
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5227 fix(platform): keep Drive running on Core v24 masternode lists (shared and multi-payout masternodes)](https://github.com/dashpay/platform/pull/5227) — 1 unresolved (1 bot) · 0 days stale · 🔴 CI failing · areas: rs-drive-abci, dashmate, fallback · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Zero-address sentinel collapses Some([0;20\]) into None without a pinning test**" — 0 days old
  - Blocker: thepastaclaw left review threads unresolved; resolve them

<a id="shumkov-unresolved-comments"></a>
#### Unresolved Comments (5)
- [dashpay/rust-dashcore#634 feat(key-wallet-manager): single-map WalletManager with Arc<RwLock<T>> per wallet](https://github.com/dashpay/rust-dashcore/pull/634) — 20 unresolved (20 CodeRabbit) · 176 days stale · ⚠ merge conflict · ✋ changes requested · areas: dash-spv, key-wallet, key-wallet-manager, fallback · Policy: waiting-author
  - Top thread: "_⚠️ Potential issue_ \| _🟡 Minor_" — 176 days old
  - Blocker: coderabbitai requested changes on this head; dismiss the review or push a fix
  - Blocker: coderabbitai left review threads unresolved; resolve them
- [dashpay/rust-dashcore#1093 fix(rpc-json)!: parse Core v24 masternode list (shared/multi-payout masternodes, nested addresses)](https://github.com/dashpay/rust-dashcore/pull/1093) — 2 unresolved (2 CodeRabbit) · 1 days stale · ✋ changes requested · 🔴 CI failing · areas: fallback · Policy: waiting-author
  - Top thread: "_🎯 Functional Correctness_ \| _🟡 Minor_ \| _⚡ Quick win_" — 1 days old
  - Blocker: coderabbitai requested changes on this head; dismiss the review or push a fix
  - Blocker: coderabbitai left review threads unresolved; resolve them
- [dashpay/platform#4993 fix(drive-abci): a transition whose version is not active is not a decode failure](https://github.com/dashpay/platform/pull/4993) — 1 unresolved (1 bot) · 6 days stale · 🔴 CI failing · areas: rs-drive-abci · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Assert the client-visible metadata in the inactive-version regression test**" — 6 days old
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5072 fix(dpp): read $entropy as base64, not base58, in the create-transition map parser](https://github.com/dashpay/platform/pull/5072) — 1 unresolved (1 bot) · 2 days stale · areas: dpp · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Add coverage for the ambiguous unpadded base64/base58 case**" — 2 days old
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5227 fix(platform): keep Drive running on Core v24 masternode lists (shared and multi-payout masternodes)](https://github.com/dashpay/platform/pull/5227) — 1 unresolved (1 bot) · 0 days stale · 🔴 CI failing · areas: rs-drive-abci, dashmate, fallback · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Zero-address sentinel collapses Some([0;20\]) into None without a pinning test**" — 0 days old
  - Blocker: thepastaclaw left review threads unresolved; resolve them

<a id="shumkov-stale"></a>
#### Stale (9)
- [dashpay/platform#4282 fix(dashmate)!: give Debian packages versions apt can order](https://github.com/dashpay/platform/pull/4282) — 6 unresolved (1 CodeRabbit, 5 bot) · 30 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · 🐢 targets v5.1-dev · areas: dashmate, js-wasm-sdk, system-contracts, github, fallback
  - Top thread: "_🗄️ Data Integrity & Integration_ \| _🟡 Minor_ \| _⚡ Quick win_" — 30 days old
- [dashpay/platform#3936 chore(drive-abci): update to nested address in SML](https://github.com/dashpay/platform/pull/3936) — 2 unresolved (2 bot) · 104 days stale · ⚠ merge conflict · 📝 draft · 🐢 targets v5.1-dev · areas: rs-drive-abci, rust-sdk, fallback
  - Top thread: "🟡 Suggestion: V0 reverse synthesizes `addresses` from a Core-22 port, masking subsequent legacy port-change diffs after …" — 104 days old
- [dashpay/platform#4283 fix(dashmate): keep the node up when an image pull fails, and stop reporting success when it did not](https://github.com/dashpay/platform/pull/4283) — 2 unresolved (2 bot) · 24 days stale · ⚠ merge conflict · ✋ changes requested · 🐢 targets v5.1-dev · areas: dashmate
  - Top thread: "🟡 Suggestion: providers.js sanitizer misses U+061C (Arabic letter mark) that sanitizeRemoteText covers**" — 24 days old
- [dashpay/platform#4660 feat(sdk): expose the document erase and history lifecycle on mobile and FFI](https://github.com/dashpay/platform/pull/4660) — 2 unresolved (2 bot) · 8 days stale · ✋ changes requested · 🔴 CI failing · 🐢 targets keep-history-lifecycle · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, rust-sdk-ffi, kotlin-sdk, fallback
  - Top thread: "🟡 Suggestion: Validate history selector discriminant at the C boundary**" — 8 days old
- [dashpay/platform#4657 feat(platform)!: delete and erase lifecycle for keep-history documents](https://github.com/dashpay/platform/pull/4657) — 2 unresolved (2 bot) · 8 days stale · ✋ changes requested · 🔴 CI failing · 🐢 targets keep-history-storage-v2 · areas: rs-drive, rs-drive-abci, dpp, rust-sdk, js-wasm-sdk, fallback
  - Top thread: "🟡 Suggestion: Deduplicate lifecycle containers at the Drive batch boundary**" — 8 days old
- [dashpay/platform#4243 fix(sdk)!: complete encrypted txMetadata parity](https://github.com/dashpay/platform/pull/4243) — ⚠ merge conflict · 📝 draft · 🐢 targets v5.1-dev · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk, github, fallback
- [dashpay/platform#4285 feat(platform-wallet)!: invitation links are AppsFlyer applinks only](https://github.com/dashpay/platform/pull/4285) — ⚠ merge conflict · 🔴 CI failing · 📝 draft · 🐢 targets v5.1-dev · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk, fallback
- [dashpay/platform#4652 feat(drive)!: preserve composite document history across protocol activation](https://github.com/dashpay/platform/pull/4652) — ⚠ merge conflict · ✋ changes requested · 🐢 targets v5.1-dev · areas: rs-drive, rs-drive-abci, rust-sdk, js-wasm-sdk, fallback
- [dashpay/platform#5228 fix(drive-abci): take evonode platform ports from Core's nested addresses](https://github.com/dashpay/platform/pull/5228) — 🐢 targets fix/core-v24-masternode-list · areas: rs-drive-abci

<a id="shumkov-ready-for-review"></a>
#### Ready for Review (6)
- [dashpay/platform#4432 fix(sdk): mirror the COUNT dispatcher's outer-walk limit when verifying range-outer carrier proofs](https://github.com/dashpay/platform/pull/4432) — by @PastaPastaPasta · areas: rs-drive, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 2c0234e378cd4dd3ca2c1f3f40317eddfb454374 after bot completion
- [dashpay/platform#5026 fix(platform-wallet)!: persist DashPay coreHeight backfill coverage so a relaunch resumes instead of rewinding again](https://github.com/dashpay/platform/pull/5026) — by @HashEngineering · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, kotlin-sdk, fallback · Policy: waiting-bots
  - Blocker: coderabbitai has not reported for the current head
- [dashpay/platform#5169 feat(sdk): let document writes state the action fee agreement](https://github.com/dashpay/platform/pull/5169) — by @PastaPastaPasta · areas: rust-sdk, js-wasm-sdk · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed b720c0bfa4340e66e34854b0049afac61a0bb69a after bot completion
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/platform#5172 ci: stage reviewed AMD64 recipe for candidate validation](https://github.com/dashpay/platform/pull/5172) — by @ktechmidas · areas: github · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 2c25cdf6c30ac2fc256390a6faef55e5464241f5 after bot completion
- [dashpay/platform#5203 fix(release): install verified cargo-binstall binary directly](https://github.com/dashpay/platform/pull/5203) — by @ktechmidas · areas: fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed f8fee0b29313e97840ce6a67fa335e90b5da7301 after bot completion
- [dashpay/platform#5236 fix(sdk): refresh stale quorum keys instead of banning every node](https://github.com/dashpay/platform/pull/5236) — by @PastaPastaPasta · areas: rust-sdk, js-wasm-sdk · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return

<a id="quantumexplorer"></a>
### @QuantumExplorer
<a id="quantumexplorer-open"></a>
#### Open (19)
- [dashpay/platform#4272 feat(dpp)!: dashpay payment detection keys and stealth derivation](https://github.com/dashpay/platform/pull/4272) — 9 unresolved (9 bot) · 59 days stale · ⚠ merge conflict · 📝 draft · 🐢 targets v5.1-dev · areas: rs-drive, rs-drive-abci, dpp, rs-platform-wallet, swift-sdk, system-contracts, kotlin-sdk, fallback
  - Top thread: "🔴 Blocking: Require compressed 33-byte encodings for payment keys**" — 59 days old
- [dashpay/rust-dashcore#1078 feat(key-wallet)!: lock masternode collateral out of coin selection](https://github.com/dashpay/rust-dashcore/pull/1078) — 8 unresolved (8 human) · 1 days stale · ✋ changes requested · areas: key-wallet, key-wallet-manager · Policy: waiting-self-review
  - Top thread: "This sets the state by hand, and together with the serde round-trip it is the only coverage of the refresh inside `updat…" — 1 days old
  - Blocker: Author must post /self-reviewed 44a9a010500fbd44aba09111b4c53a28b88e8073 after bot completion
- [dashpay/platform#4760 feat(platform)!: token shielded pools](https://github.com/dashpay/platform/pull/4760) — 3 unresolved (3 human) · 9 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · areas: rs-drive, rs-drive-abci, dpp, rs-platform-wallet, rust-sdk, fallback · Policy: waiting-author
  - Top thread: "Fixed, and now pinned by a test." — 9 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/rust-dashcore#860 feat(dash-spv): expose per-peer connection stats (ping RTT, bytes received)](https://github.com/dashpay/rust-dashcore/pull/860) — 3 unresolved (3 CodeRabbit) · 83 days stale · ✋ changes requested · areas: dash-spv, fallback · Policy: waiting-author
  - Top thread: "_📐 Maintainability & Code Quality_ \| _🟡 Minor_ \| _⚡ Quick win_" — 83 days old
  - Blocker: coderabbitai left review threads unresolved; resolve them
- [dashpay/rust-dashcore#622 fix: apply BIP-69 sorting to asset lock credit outputs](https://github.com/dashpay/rust-dashcore/pull/622) — 1 unresolved (1 CodeRabbit) · 182 days stale · ⚠ merge conflict · ✋ changes requested · 📝 draft · areas: key-wallet · Policy: draft
  - Top thread: "_⚠️ Potential issue_ \| _🟠 Major_" — 182 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#3417 feat(swift-sdk): use SPV-synced quorums for Platform proof verification](https://github.com/dashpay/platform/pull/3417) — 3 unresolved (3 bot) · 22 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · 🐢 targets v5.1-dev · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, rust-sdk, rust-sdk-ffi, fallback
  - Top thread: "🟡 Suggestion: Exercise the production runtime bridge across Tokio runtime flavors**" — 22 days old
- [dashpay/platform#5014 fix(drive-abci)!: record and check the nullifiers of shielding transitions (PV14)](https://github.com/dashpay/platform/pull/5014) — 5 unresolved (5 bot) · 5 days stale · 🐢 targets claude/tokens-shielded-pools-6642a6 · areas: rs-drive, rs-drive-abci, dpp, rs-platform-wallet-ffi, fallback
  - Top thread: "🟡 Suggestion: Seed the shared Orchard bundle fixture**" — 5 days old
- [dashpay/rust-dashcore#136 feat: update integration test script for Dash compatibility](https://github.com/dashpay/rust-dashcore/pull/136) — 1 unresolved (1 CodeRabbit) · 392 days stale · 🔴 CI failing · 🐢 targets v0.40-dev, untouched 392 days · areas: fallback
  - Top thread: "_🛠️ Refactor suggestion_" — 392 days old
- [dashpay/rust-dashcore#1077 feat(dash-spv)!: check masternode list diffs against the block coinbase](https://github.com/dashpay/rust-dashcore/pull/1077) — 2 unresolved (2 CodeRabbit) · 3 days stale · ✋ changes requested · 🔴 CI failing · areas: dash-spv, fallback · Policy: waiting-author
  - Top thread: "_🩺 Stability & Availability_ \| _🟠 Major_ \| _⚡ Quick win_" — 3 days old
  - Blocker: coderabbitai requested changes on this head; dismiss the review or push a fix
  - Blocker: coderabbitai left review threads unresolved; resolve them
- [dashpay/platform#4519 feat(platform-wallet): rotate a masternode's keys into the wallet — ProUpRegTx orchestration, FFI, Swift](https://github.com/dashpay/platform/pull/4519) — ⚠ merge conflict · 🐢 targets v5.1-dev · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk
- [dashpay/platform#4730 feat(platform)!: delta-based data contract update transition for protocol version 15](https://github.com/dashpay/platform/pull/4730) — ⚠ merge conflict · 🔴 CI failing · 🐢 targets v5.1-dev · areas: rs-drive, rs-drive-abci, dpp, rs-platform-wallet, fallback
- [dashpay/platform#4776 feat(platform-wallet)!: let contract updates clear the description through the FFI](https://github.com/dashpay/platform/pull/4776) — 🔴 CI failing · 🐢 targets feat/delta-contract-update-pv15 · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk, fallback
- [dashpay/platform#4899 feat(platform)!: yes/no masternode vote poll kind with supermajority and minimum voting power](https://github.com/dashpay/platform/pull/4899) — ⚠ merge conflict · ✋ changes requested · 📝 draft · 🐢 targets v5.1-dev · areas: rs-drive, rs-drive-abci, dpp, swift-sdk, rust-sdk, rust-sdk-ffi, js-wasm-sdk, rust-dapi, kotlin-sdk, fallback
- [dashpay/platform#4933 feat(platform)!: dashpay contact requests declare their checks (PV14)](https://github.com/dashpay/platform/pull/4933) — ⚠ merge conflict · 📝 draft · areas: rs-drive, rs-drive-abci, dpp, system-contracts, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#4986 fix(drive-abci)!: pay every reward share of a masternode and credit each identity once in the epoch payout (PV15)](https://github.com/dashpay/platform/pull/4986) — ⚠ merge conflict · 📝 draft · 🐢 targets v5.1-dev · areas: rs-drive, rs-drive-abci, dpp, github, fallback
- [dashpay/platform#5107 ci: let the SDK release workflows rebuild tags older than v4.2.0-beta.5](https://github.com/dashpay/platform/pull/5107) — ⚠ merge conflict · areas: github · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 6bb0c65ef034906461c1c4cd2f3be0b85c4b6c2f after bot completion
- [dashpay/platform#5113 feat(platform-wallet)!: keep masternode collateral out of coin selection](https://github.com/dashpay/platform/pull/5113) — 🔴 CI failing · 📝 draft · 🐢 targets chore/bump-rust-dashcore-secp-033 · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk, fallback
- [dashpay/platform#5237 feat(platform)!: withdrawals also fit a Core-anchored limit (PV14)](https://github.com/dashpay/platform/pull/5237) — 🔴 CI failing · areas: rs-drive, rs-drive-abci, dpp, dashmate, fallback · Policy: waiting-bots
  - Blocker: coderabbitai has not reported for the current head
  - Blocker: thepastaclaw has not reported for the current head
- [dashpay/platform#5238 fix(platform)!: refund sponsor-paid document storage to the gas sponsor (PV14)](https://github.com/dashpay/platform/pull/5238) — 🔴 CI failing · areas: rs-drive, rs-drive-abci, dashmate, fallback · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head

<a id="quantumexplorer-needs-action"></a>
#### Needs action (5)
- [dashpay/rust-dashcore#1078 feat(key-wallet)!: lock masternode collateral out of coin selection](https://github.com/dashpay/rust-dashcore/pull/1078) — 8 unresolved (8 human) · 1 days stale · ✋ changes requested · areas: key-wallet, key-wallet-manager · Policy: waiting-self-review
  - Top thread: "This sets the state by hand, and together with the serde round-trip it is the only coverage of the refresh inside `updat…" — 1 days old
  - Blocker: Author must post /self-reviewed 44a9a010500fbd44aba09111b4c53a28b88e8073 after bot completion
- [dashpay/platform#4760 feat(platform)!: token shielded pools](https://github.com/dashpay/platform/pull/4760) — 3 unresolved (3 human) · 9 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · areas: rs-drive, rs-drive-abci, dpp, rs-platform-wallet, rust-sdk, fallback · Policy: waiting-author
  - Top thread: "Fixed, and now pinned by a test." — 9 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/rust-dashcore#860 feat(dash-spv): expose per-peer connection stats (ping RTT, bytes received)](https://github.com/dashpay/rust-dashcore/pull/860) — 3 unresolved (3 CodeRabbit) · 83 days stale · ✋ changes requested · areas: dash-spv, fallback · Policy: waiting-author
  - Top thread: "_📐 Maintainability & Code Quality_ \| _🟡 Minor_ \| _⚡ Quick win_" — 83 days old
  - Blocker: coderabbitai left review threads unresolved; resolve them
- [dashpay/rust-dashcore#1077 feat(dash-spv)!: check masternode list diffs against the block coinbase](https://github.com/dashpay/rust-dashcore/pull/1077) — 2 unresolved (2 CodeRabbit) · 3 days stale · ✋ changes requested · 🔴 CI failing · areas: dash-spv, fallback · Policy: waiting-author
  - Top thread: "_🩺 Stability & Availability_ \| _🟠 Major_ \| _⚡ Quick win_" — 3 days old
  - Blocker: coderabbitai requested changes on this head; dismiss the review or push a fix
  - Blocker: coderabbitai left review threads unresolved; resolve them
- [dashpay/platform#5107 ci: let the SDK release workflows rebuild tags older than v4.2.0-beta.5](https://github.com/dashpay/platform/pull/5107) — ⚠ merge conflict · areas: github · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 6bb0c65ef034906461c1c4cd2f3be0b85c4b6c2f after bot completion

<a id="quantumexplorer-unresolved-comments"></a>
#### Unresolved Comments (4)
- [dashpay/rust-dashcore#1078 feat(key-wallet)!: lock masternode collateral out of coin selection](https://github.com/dashpay/rust-dashcore/pull/1078) — 8 unresolved (8 human) · 1 days stale · ✋ changes requested · areas: key-wallet, key-wallet-manager · Policy: waiting-self-review
  - Top thread: "This sets the state by hand, and together with the serde round-trip it is the only coverage of the refresh inside `updat…" — 1 days old
  - Blocker: Author must post /self-reviewed 44a9a010500fbd44aba09111b4c53a28b88e8073 after bot completion
- [dashpay/platform#4760 feat(platform)!: token shielded pools](https://github.com/dashpay/platform/pull/4760) — 3 unresolved (3 human) · 9 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · areas: rs-drive, rs-drive-abci, dpp, rs-platform-wallet, rust-sdk, fallback · Policy: waiting-author
  - Top thread: "Fixed, and now pinned by a test." — 9 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/rust-dashcore#860 feat(dash-spv): expose per-peer connection stats (ping RTT, bytes received)](https://github.com/dashpay/rust-dashcore/pull/860) — 3 unresolved (3 CodeRabbit) · 83 days stale · ✋ changes requested · areas: dash-spv, fallback · Policy: waiting-author
  - Top thread: "_📐 Maintainability & Code Quality_ \| _🟡 Minor_ \| _⚡ Quick win_" — 83 days old
  - Blocker: coderabbitai left review threads unresolved; resolve them
- [dashpay/rust-dashcore#1077 feat(dash-spv)!: check masternode list diffs against the block coinbase](https://github.com/dashpay/rust-dashcore/pull/1077) — 2 unresolved (2 CodeRabbit) · 3 days stale · ✋ changes requested · 🔴 CI failing · areas: dash-spv, fallback · Policy: waiting-author
  - Top thread: "_🩺 Stability & Availability_ \| _🟠 Major_ \| _⚡ Quick win_" — 3 days old
  - Blocker: coderabbitai requested changes on this head; dismiss the review or push a fix
  - Blocker: coderabbitai left review threads unresolved; resolve them

<a id="quantumexplorer-ci-failing"></a>
#### CI Failing (2)
- [dashpay/platform#5237 feat(platform)!: withdrawals also fit a Core-anchored limit (PV14)](https://github.com/dashpay/platform/pull/5237) — 🔴 CI failing · areas: rs-drive, rs-drive-abci, dpp, dashmate, fallback · Policy: waiting-bots
  - Blocker: coderabbitai has not reported for the current head
  - Blocker: thepastaclaw has not reported for the current head
- [dashpay/platform#5238 fix(platform)!: refund sponsor-paid document storage to the gas sponsor (PV14)](https://github.com/dashpay/platform/pull/5238) — 🔴 CI failing · areas: rs-drive, rs-drive-abci, dashmate, fallback · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head

<a id="quantumexplorer-draft"></a>
#### Draft (2)
- [dashpay/rust-dashcore#622 fix: apply BIP-69 sorting to asset lock credit outputs](https://github.com/dashpay/rust-dashcore/pull/622) — 1 unresolved (1 CodeRabbit) · 182 days stale · ⚠ merge conflict · ✋ changes requested · 📝 draft · areas: key-wallet · Policy: draft
  - Top thread: "_⚠️ Potential issue_ \| _🟠 Major_" — 182 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#4933 feat(platform)!: dashpay contact requests declare their checks (PV14)](https://github.com/dashpay/platform/pull/4933) — ⚠ merge conflict · 📝 draft · areas: rs-drive, rs-drive-abci, dpp, system-contracts, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot

<a id="quantumexplorer-stale"></a>
#### Stale (10)
- [dashpay/platform#4272 feat(dpp)!: dashpay payment detection keys and stealth derivation](https://github.com/dashpay/platform/pull/4272) — 9 unresolved (9 bot) · 59 days stale · ⚠ merge conflict · 📝 draft · 🐢 targets v5.1-dev · areas: rs-drive, rs-drive-abci, dpp, rs-platform-wallet, swift-sdk, system-contracts, kotlin-sdk, fallback
  - Top thread: "🔴 Blocking: Require compressed 33-byte encodings for payment keys**" — 59 days old
- [dashpay/platform#3417 feat(swift-sdk): use SPV-synced quorums for Platform proof verification](https://github.com/dashpay/platform/pull/3417) — 3 unresolved (3 bot) · 22 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · 🐢 targets v5.1-dev · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, rust-sdk, rust-sdk-ffi, fallback
  - Top thread: "🟡 Suggestion: Exercise the production runtime bridge across Tokio runtime flavors**" — 22 days old
- [dashpay/platform#5014 fix(drive-abci)!: record and check the nullifiers of shielding transitions (PV14)](https://github.com/dashpay/platform/pull/5014) — 5 unresolved (5 bot) · 5 days stale · 🐢 targets claude/tokens-shielded-pools-6642a6 · areas: rs-drive, rs-drive-abci, dpp, rs-platform-wallet-ffi, fallback
  - Top thread: "🟡 Suggestion: Seed the shared Orchard bundle fixture**" — 5 days old
- [dashpay/rust-dashcore#136 feat: update integration test script for Dash compatibility](https://github.com/dashpay/rust-dashcore/pull/136) — 1 unresolved (1 CodeRabbit) · 392 days stale · 🔴 CI failing · 🐢 targets v0.40-dev, untouched 392 days · areas: fallback
  - Top thread: "_🛠️ Refactor suggestion_" — 392 days old
- [dashpay/platform#4519 feat(platform-wallet): rotate a masternode's keys into the wallet — ProUpRegTx orchestration, FFI, Swift](https://github.com/dashpay/platform/pull/4519) — ⚠ merge conflict · 🐢 targets v5.1-dev · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk
- [dashpay/platform#4730 feat(platform)!: delta-based data contract update transition for protocol version 15](https://github.com/dashpay/platform/pull/4730) — ⚠ merge conflict · 🔴 CI failing · 🐢 targets v5.1-dev · areas: rs-drive, rs-drive-abci, dpp, rs-platform-wallet, fallback
- [dashpay/platform#4776 feat(platform-wallet)!: let contract updates clear the description through the FFI](https://github.com/dashpay/platform/pull/4776) — 🔴 CI failing · 🐢 targets feat/delta-contract-update-pv15 · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk, fallback
- [dashpay/platform#4899 feat(platform)!: yes/no masternode vote poll kind with supermajority and minimum voting power](https://github.com/dashpay/platform/pull/4899) — ⚠ merge conflict · ✋ changes requested · 📝 draft · 🐢 targets v5.1-dev · areas: rs-drive, rs-drive-abci, dpp, swift-sdk, rust-sdk, rust-sdk-ffi, js-wasm-sdk, rust-dapi, kotlin-sdk, fallback
- [dashpay/platform#4986 fix(drive-abci)!: pay every reward share of a masternode and credit each identity once in the epoch payout (PV15)](https://github.com/dashpay/platform/pull/4986) — ⚠ merge conflict · 📝 draft · 🐢 targets v5.1-dev · areas: rs-drive, rs-drive-abci, dpp, github, fallback
- [dashpay/platform#5113 feat(platform-wallet)!: keep masternode collateral out of coin selection](https://github.com/dashpay/platform/pull/5113) — 🔴 CI failing · 📝 draft · 🐢 targets chore/bump-rust-dashcore-secp-033 · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk, fallback

<a id="quantumexplorer-clean"></a>
#### Clean (1)
- [dashpay/platform#5107 ci: let the SDK release workflows rebuild tags older than v4.2.0-beta.5](https://github.com/dashpay/platform/pull/5107) — ⚠ merge conflict · areas: github · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 6bb0c65ef034906461c1c4cd2f3be0b85c4b6c2f after bot completion

<a id="quantumexplorer-ready-for-review"></a>
#### Ready for Review (5)
- [dashpay/platform#4432 fix(sdk): mirror the COUNT dispatcher's outer-walk limit when verifying range-outer carrier proofs](https://github.com/dashpay/platform/pull/4432) — by @PastaPastaPasta · areas: rs-drive, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 2c0234e378cd4dd3ca2c1f3f40317eddfb454374 after bot completion
- [dashpay/platform#5026 fix(platform-wallet)!: persist DashPay coreHeight backfill coverage so a relaunch resumes instead of rewinding again](https://github.com/dashpay/platform/pull/5026) — by @HashEngineering · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, kotlin-sdk, fallback · Policy: waiting-bots
  - Blocker: coderabbitai has not reported for the current head
- [dashpay/platform#5203 fix(release): install verified cargo-binstall binary directly](https://github.com/dashpay/platform/pull/5203) — by @ktechmidas · areas: fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed f8fee0b29313e97840ce6a67fa335e90b5da7301 after bot completion
- [dashpay/rust-dashcore#849 feat(dash-spv): add `--birth-height` CLI flag](https://github.com/dashpay/rust-dashcore/pull/849) — by @xdustinface · areas: dash-spv · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 9b1e709fd8e0bfab0dd69f7c86fd6b7d2e585fbe after bot completion
- [dashpay/rust-dashcore#1092 perf(dash-spv): segment size per type](https://github.com/dashpay/rust-dashcore/pull/1092) — by @ZocoLini · areas: dash-spv · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 163f7ec13b63b43320f30f336def21d05bc6013b after bot completion
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return

<a id="llbartekll"></a>
### @llbartekll
<a id="llbartekll-open"></a>
#### Open (3)
- [dashpay/platform#3560 test(swift-sdk): add testnet identity-discovery UI test](https://github.com/dashpay/platform/pull/3560) — 1 unresolved (1 bot) · 122 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · areas: swift-sdk, github · Policy: waiting-author
  - Top thread: "🔴 Blocking: Carried-forward prior finding (STILL VALID): Docker-setup toggle leaves the cached regtest wallet manager bo…" — 122 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/rust-dashcore#819 feat(key-wallet): CoinJoin sweep + gap-limit recovery; TransactionBuilder drain mode](https://github.com/dashpay/rust-dashcore/pull/819) — ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · areas: key-wallet · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed e406d1cbb86a03503fa41a9d796f25c0bacc3e12 after bot completion
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/rust-dashcore#910 fix(dash-spv): re-anchor incompatible shared storage](https://github.com/dashpay/rust-dashcore/pull/910) — 📝 draft · areas: dash-spv · Policy: draft
  - Blocker: Draft PR does not occupy a review slot

<a id="llbartekll-needs-action"></a>
#### Needs action (2)
- [dashpay/platform#3560 test(swift-sdk): add testnet identity-discovery UI test](https://github.com/dashpay/platform/pull/3560) — 1 unresolved (1 bot) · 122 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · areas: swift-sdk, github · Policy: waiting-author
  - Top thread: "🔴 Blocking: Carried-forward prior finding (STILL VALID): Docker-setup toggle leaves the cached regtest wallet manager bo…" — 122 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/rust-dashcore#819 feat(key-wallet): CoinJoin sweep + gap-limit recovery; TransactionBuilder drain mode](https://github.com/dashpay/rust-dashcore/pull/819) — ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · areas: key-wallet · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed e406d1cbb86a03503fa41a9d796f25c0bacc3e12 after bot completion
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return

<a id="llbartekll-unresolved-comments"></a>
#### Unresolved Comments (1)
- [dashpay/platform#3560 test(swift-sdk): add testnet identity-discovery UI test](https://github.com/dashpay/platform/pull/3560) — 1 unresolved (1 bot) · 122 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · areas: swift-sdk, github · Policy: waiting-author
  - Top thread: "🔴 Blocking: Carried-forward prior finding (STILL VALID): Docker-setup toggle leaves the cached regtest wallet manager bo…" — 122 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
  - Blocker: Proceeded without coderabbitai: no review within the configured window

<a id="llbartekll-changes-requested"></a>
#### Changes Requested (1)
- [dashpay/rust-dashcore#819 feat(key-wallet): CoinJoin sweep + gap-limit recovery; TransactionBuilder drain mode](https://github.com/dashpay/rust-dashcore/pull/819) — ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · areas: key-wallet · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed e406d1cbb86a03503fa41a9d796f25c0bacc3e12 after bot completion
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return

<a id="llbartekll-draft"></a>
#### Draft (1)
- [dashpay/rust-dashcore#910 fix(dash-spv): re-anchor incompatible shared storage](https://github.com/dashpay/rust-dashcore/pull/910) — 📝 draft · areas: dash-spv · Policy: draft
  - Blocker: Draft PR does not occupy a review slot

<a id="llbartekll-ready-for-review"></a>
#### Ready for Review (1)
- [dashpay/platform#5026 fix(platform-wallet)!: persist DashPay coreHeight backfill coverage so a relaunch resumes instead of rewinding again](https://github.com/dashpay/platform/pull/5026) — by @HashEngineering · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, kotlin-sdk, fallback · Policy: waiting-bots
  - Blocker: coderabbitai has not reported for the current head

<a id="zocolini"></a>
### @ZocoLini
<a id="zocolini-open"></a>
#### Open (9)
- [dashpay/rust-dashcore#375 refactor(dash-spv): checkpoint rewrite](https://github.com/dashpay/rust-dashcore/pull/375) — 2 unresolved (2 CodeRabbit) · 252 days stale · ⚠ merge conflict · 📝 draft · areas: dash-spv, fallback · Policy: draft
  - Top thread: "_⚠️ Potential issue_ \| _🟡 Minor_" — 252 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#1090 refactor(dash-spv): delete dropped segment files right away](https://github.com/dashpay/rust-dashcore/pull/1090) — 1 unresolved (1 CodeRabbit) · 1 days stale · ✋ changes requested · areas: dash-spv · Policy: waiting-author
  - Top thread: "_🗄️ Data Integrity & Integration_ \| _🟡 Minor_ \| _⚡ Quick win_" — 1 days old
  - Blocker: coderabbitai left review threads unresolved; resolve them
- [dashpay/rust-dashcore#902 refactor(dash-spv): network manager refactor and sync pipelines optimized ](https://github.com/dashpay/rust-dashcore/pull/902) — ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · 📝 draft · areas: dash-spv, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#921 fix(dash-spv): sync backfill](https://github.com/dashpay/rust-dashcore/pull/921) — ⚠ merge conflict · 📝 draft · areas: dash-spv, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#1065 test(dash-spv): look up quorums the way Platform does, across heights](https://github.com/dashpay/rust-dashcore/pull/1065) — 🔴 CI failing · areas: dash-spv · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 70e33e710ad44df6b26d29ea064088d2d1d3836b after bot completion
- [dashpay/rust-dashcore#1081 feat(dash-spv): restore the validated rotation cycles from the replayed engine](https://github.com/dashpay/rust-dashcore/pull/1081) — 📝 draft · areas: dash-spv · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#1092 perf(dash-spv): segment size per type](https://github.com/dashpay/rust-dashcore/pull/1092) — areas: dash-spv · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 163f7ec13b63b43320f30f336def21d05bc6013b after bot completion
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/rust-dashcore#1094 fix(dash-spv): resolve masternode engine heights from the header storage](https://github.com/dashpay/rust-dashcore/pull/1094) — 🔴 CI failing · 📝 draft · areas: dash-spv, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#1095 perf(dash-spv): keep the header hash to height index on disk](https://github.com/dashpay/rust-dashcore/pull/1095) — 🔴 CI failing · 📝 draft · areas: dash-spv · Policy: draft
  - Blocker: Draft PR does not occupy a review slot

<a id="zocolini-needs-action"></a>
#### Needs action (1)
- [dashpay/rust-dashcore#1090 refactor(dash-spv): delete dropped segment files right away](https://github.com/dashpay/rust-dashcore/pull/1090) — 1 unresolved (1 CodeRabbit) · 1 days stale · ✋ changes requested · areas: dash-spv · Policy: waiting-author
  - Top thread: "_🗄️ Data Integrity & Integration_ \| _🟡 Minor_ \| _⚡ Quick win_" — 1 days old
  - Blocker: coderabbitai left review threads unresolved; resolve them

<a id="zocolini-unresolved-comments"></a>
#### Unresolved Comments (1)
- [dashpay/rust-dashcore#1090 refactor(dash-spv): delete dropped segment files right away](https://github.com/dashpay/rust-dashcore/pull/1090) — 1 unresolved (1 CodeRabbit) · 1 days stale · ✋ changes requested · areas: dash-spv · Policy: waiting-author
  - Top thread: "_🗄️ Data Integrity & Integration_ \| _🟡 Minor_ \| _⚡ Quick win_" — 1 days old
  - Blocker: coderabbitai left review threads unresolved; resolve them

<a id="zocolini-ci-failing"></a>
#### CI Failing (1)
- [dashpay/rust-dashcore#1065 test(dash-spv): look up quorums the way Platform does, across heights](https://github.com/dashpay/rust-dashcore/pull/1065) — 🔴 CI failing · areas: dash-spv · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 70e33e710ad44df6b26d29ea064088d2d1d3836b after bot completion

<a id="zocolini-draft"></a>
#### Draft (6)
- [dashpay/rust-dashcore#375 refactor(dash-spv): checkpoint rewrite](https://github.com/dashpay/rust-dashcore/pull/375) — 2 unresolved (2 CodeRabbit) · 252 days stale · ⚠ merge conflict · 📝 draft · areas: dash-spv, fallback · Policy: draft
  - Top thread: "_⚠️ Potential issue_ \| _🟡 Minor_" — 252 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#902 refactor(dash-spv): network manager refactor and sync pipelines optimized ](https://github.com/dashpay/rust-dashcore/pull/902) — ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · 📝 draft · areas: dash-spv, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#921 fix(dash-spv): sync backfill](https://github.com/dashpay/rust-dashcore/pull/921) — ⚠ merge conflict · 📝 draft · areas: dash-spv, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#1081 feat(dash-spv): restore the validated rotation cycles from the replayed engine](https://github.com/dashpay/rust-dashcore/pull/1081) — 📝 draft · areas: dash-spv · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#1094 fix(dash-spv): resolve masternode engine heights from the header storage](https://github.com/dashpay/rust-dashcore/pull/1094) — 🔴 CI failing · 📝 draft · areas: dash-spv, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#1095 perf(dash-spv): keep the header hash to height index on disk](https://github.com/dashpay/rust-dashcore/pull/1095) — 🔴 CI failing · 📝 draft · areas: dash-spv · Policy: draft
  - Blocker: Draft PR does not occupy a review slot

<a id="zocolini-clean"></a>
#### Clean (1)
- [dashpay/rust-dashcore#1092 perf(dash-spv): segment size per type](https://github.com/dashpay/rust-dashcore/pull/1092) — areas: dash-spv · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 163f7ec13b63b43320f30f336def21d05bc6013b after bot completion
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return

<a id="zocolini-ready-for-review"></a>
#### Ready for Review (1)
- [dashpay/platform#5026 fix(platform-wallet)!: persist DashPay coreHeight backfill coverage so a relaunch resumes instead of rewinding again](https://github.com/dashpay/platform/pull/5026) — by @HashEngineering · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, kotlin-sdk, fallback · Policy: waiting-bots
  - Blocker: coderabbitai has not reported for the current head

<a id="romchornyi"></a>
### @romchornyi
<a id="romchornyi-open"></a>
#### Open (5)
- [dashpay/platform#4595 fix(swift-sdk): make a changeset round linear without giving up atomicity](https://github.com/dashpay/platform/pull/4595) — 2 unresolved (2 bot) · 24 days stale · ⚠ merge conflict · ✋ changes requested · areas: swift-sdk · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Add round-level regression coverage for registry bookkeeping**" — 24 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5146 fix(sdk): run blocking Swift SDK Platform queries off the caller's actor](https://github.com/dashpay/platform/pull/5146) — 1 unresolved (1 bot) · 2 days stale · ⚠ merge conflict · 🐢 targets v5.1-dev · areas: swift-sdk, rust-sdk-ffi
  - Top thread: "🔴 Blocking: Prevent shared query borrows from overlapping exclusive SDK borrows**" — 2 days old
- [dashpay/platform#4978 fix(sdk): keep the chosen DPNS name across wallet sync](https://github.com/dashpay/platform/pull/4978) — 🔴 CI failing · 🐢 targets v5.1-dev · areas: rs-drive, rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, rust-sdk
- [dashpay/platform#4992 feat(swift-sdk): durable wallet presence marker readable while the device is locked](https://github.com/dashpay/platform/pull/4992) — 🐢 targets v5.1-dev · areas: swift-sdk
- [dashpay/platform#5206 fix(platform-wallet): report an identity balance refusal on withdrawal as insufficient credits](https://github.com/dashpay/platform/pull/5206) — 🔴 CI failing · 🐢 targets v5.1-dev · areas: rs-platform-wallet, rs-platform-wallet-ffi

<a id="romchornyi-needs-action"></a>
#### Needs action (1)
- [dashpay/platform#4595 fix(swift-sdk): make a changeset round linear without giving up atomicity](https://github.com/dashpay/platform/pull/4595) — 2 unresolved (2 bot) · 24 days stale · ⚠ merge conflict · ✋ changes requested · areas: swift-sdk · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Add round-level regression coverage for registry bookkeeping**" — 24 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them

<a id="romchornyi-unresolved-comments"></a>
#### Unresolved Comments (1)
- [dashpay/platform#4595 fix(swift-sdk): make a changeset round linear without giving up atomicity](https://github.com/dashpay/platform/pull/4595) — 2 unresolved (2 bot) · 24 days stale · ⚠ merge conflict · ✋ changes requested · areas: swift-sdk · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Add round-level regression coverage for registry bookkeeping**" — 24 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them

<a id="romchornyi-stale"></a>
#### Stale (4)
- [dashpay/platform#5146 fix(sdk): run blocking Swift SDK Platform queries off the caller's actor](https://github.com/dashpay/platform/pull/5146) — 1 unresolved (1 bot) · 2 days stale · ⚠ merge conflict · 🐢 targets v5.1-dev · areas: swift-sdk, rust-sdk-ffi
  - Top thread: "🔴 Blocking: Prevent shared query borrows from overlapping exclusive SDK borrows**" — 2 days old
- [dashpay/platform#4978 fix(sdk): keep the chosen DPNS name across wallet sync](https://github.com/dashpay/platform/pull/4978) — 🔴 CI failing · 🐢 targets v5.1-dev · areas: rs-drive, rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, rust-sdk
- [dashpay/platform#4992 feat(swift-sdk): durable wallet presence marker readable while the device is locked](https://github.com/dashpay/platform/pull/4992) — 🐢 targets v5.1-dev · areas: swift-sdk
- [dashpay/platform#5206 fix(platform-wallet): report an identity balance refusal on withdrawal as insufficient credits](https://github.com/dashpay/platform/pull/5206) — 🔴 CI failing · 🐢 targets v5.1-dev · areas: rs-platform-wallet, rs-platform-wallet-ffi

<a id="romchornyi-ready-for-review"></a>
#### Ready for Review (1)
- [dashpay/platform#5026 fix(platform-wallet)!: persist DashPay coreHeight backfill coverage so a relaunch resumes instead of rewinding again](https://github.com/dashpay/platform/pull/5026) — by @HashEngineering · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, kotlin-sdk, fallback · Policy: waiting-bots
  - Blocker: coderabbitai has not reported for the current head

<a id="xdustinface"></a>
### @xdustinface
<a id="xdustinface-open"></a>
#### Open (8)
- [dashpay/rust-dashcore#856 feat(dash-spv): force resync to re-anchor a running client at a lower checkpoint](https://github.com/dashpay/rust-dashcore/pull/856) — 1 unresolved (1 human) · 72 days stale · ⚠ merge conflict · 🔴 CI failing · areas: dash-spv · Policy: waiting-self-review
  - Top thread: "why do we have a periodic check here??" — 72 days old
  - Blocker: Author must post /self-reviewed 7282172a3bb4a0d9d85a40cf70b4dd111995f96b after bot completion
- [dashpay/rust-dashcore#400 chore: remove BIP141 (SegWit) and BIP341 (Taproot) support](https://github.com/dashpay/rust-dashcore/pull/400) — ⚠ merge conflict · 🔴 CI failing · 📝 draft · 🐢 untouched 134 days · areas: dash-spv, key-wallet, github, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#541 ci: integrate Codecov test analytics via `cargo-nextest`](https://github.com/dashpay/rust-dashcore/pull/541) — ⚠ merge conflict · 📝 draft · areas: github, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#720 fix(dash-spv): split peer `TcpStream` and add per-peer writer task](https://github.com/dashpay/rust-dashcore/pull/720) — ⚠ merge conflict · 🔴 CI failing · 📝 draft · 🐢 untouched 134 days · areas: dash-spv, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#776 chore: remove FFI layer](https://github.com/dashpay/rust-dashcore/pull/776) — ⚠ merge conflict · 📝 draft · 🐢 untouched 134 days · areas: dash-spv, key-wallet, github, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#799 feat: validate masternode list merkle root](https://github.com/dashpay/rust-dashcore/pull/799) — ⚠ merge conflict · 📝 draft · areas: fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#803 feat(dash-spv): block locator + staged fork detection](https://github.com/dashpay/rust-dashcore/pull/803) — ⚠ merge conflict · 📝 draft · areas: dash-spv, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#849 feat(dash-spv): add `--birth-height` CLI flag](https://github.com/dashpay/rust-dashcore/pull/849) — areas: dash-spv · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 9b1e709fd8e0bfab0dd69f7c86fd6b7d2e585fbe after bot completion

<a id="xdustinface-needs-action"></a>
#### Needs action (1)
- [dashpay/rust-dashcore#856 feat(dash-spv): force resync to re-anchor a running client at a lower checkpoint](https://github.com/dashpay/rust-dashcore/pull/856) — 1 unresolved (1 human) · 72 days stale · ⚠ merge conflict · 🔴 CI failing · areas: dash-spv · Policy: waiting-self-review
  - Top thread: "why do we have a periodic check here??" — 72 days old
  - Blocker: Author must post /self-reviewed 7282172a3bb4a0d9d85a40cf70b4dd111995f96b after bot completion

<a id="xdustinface-unresolved-comments"></a>
#### Unresolved Comments (1)
- [dashpay/rust-dashcore#856 feat(dash-spv): force resync to re-anchor a running client at a lower checkpoint](https://github.com/dashpay/rust-dashcore/pull/856) — 1 unresolved (1 human) · 72 days stale · ⚠ merge conflict · 🔴 CI failing · areas: dash-spv · Policy: waiting-self-review
  - Top thread: "why do we have a periodic check here??" — 72 days old
  - Blocker: Author must post /self-reviewed 7282172a3bb4a0d9d85a40cf70b4dd111995f96b after bot completion

<a id="xdustinface-draft"></a>
#### Draft (3)
- [dashpay/rust-dashcore#541 ci: integrate Codecov test analytics via `cargo-nextest`](https://github.com/dashpay/rust-dashcore/pull/541) — ⚠ merge conflict · 📝 draft · areas: github, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#799 feat: validate masternode list merkle root](https://github.com/dashpay/rust-dashcore/pull/799) — ⚠ merge conflict · 📝 draft · areas: fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#803 feat(dash-spv): block locator + staged fork detection](https://github.com/dashpay/rust-dashcore/pull/803) — ⚠ merge conflict · 📝 draft · areas: dash-spv, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot

<a id="xdustinface-stale"></a>
#### Stale (3)
- [dashpay/rust-dashcore#400 chore: remove BIP141 (SegWit) and BIP341 (Taproot) support](https://github.com/dashpay/rust-dashcore/pull/400) — ⚠ merge conflict · 🔴 CI failing · 📝 draft · 🐢 untouched 134 days · areas: dash-spv, key-wallet, github, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#720 fix(dash-spv): split peer `TcpStream` and add per-peer writer task](https://github.com/dashpay/rust-dashcore/pull/720) — ⚠ merge conflict · 🔴 CI failing · 📝 draft · 🐢 untouched 134 days · areas: dash-spv, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#776 chore: remove FFI layer](https://github.com/dashpay/rust-dashcore/pull/776) — ⚠ merge conflict · 📝 draft · 🐢 untouched 134 days · areas: dash-spv, key-wallet, github, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot

<a id="xdustinface-clean"></a>
#### Clean (1)
- [dashpay/rust-dashcore#849 feat(dash-spv): add `--birth-height` CLI flag](https://github.com/dashpay/rust-dashcore/pull/849) — areas: dash-spv · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 9b1e709fd8e0bfab0dd69f7c86fd6b7d2e585fbe after bot completion

<a id="xdustinface-ready-for-review"></a>
#### Ready for Review (1)
- [dashpay/rust-dashcore#1092 perf(dash-spv): segment size per type](https://github.com/dashpay/rust-dashcore/pull/1092) — by @ZocoLini · areas: dash-spv · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 163f7ec13b63b43320f30f336def21d05bc6013b after bot completion
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return

<a id="infraclaw-dash"></a>
### @infraclaw-dash
<a id="infraclaw-dash-open"></a>
#### Open (4)
- [dashpay/platform#3958 ci: add Platform testnet sync status reporting](https://github.com/dashpay/platform/pull/3958) — 2 unresolved (2 bot) · 99 days stale · ⚠ merge conflict · 🐢 targets v5.1-dev · areas: github, fallback
  - Top thread: "🟡 Suggestion: workflow_dispatch lets any repo writer forge a sync status**" — 99 days old
- [dashpay/platform#5167 ci: align secp feature-base workflows with provisioned runner images](https://github.com/dashpay/platform/pull/5167) — 3 unresolved (3 bot) · 2 days stale · 🐢 targets chore/bump-rust-dashcore-secp-033 · areas: github
  - Top thread: "🟡 Suggestion: Contract-failure error points to a doc file missing from this branch**" — 2 days old
- [dashpay/platform#5166 ci: adopt verified Rust and Kotlin runner images on v4.3](https://github.com/dashpay/platform/pull/5166) — ⚠ merge conflict · 🐢 targets v5.1-dev · areas: github
- [dashpay/platform#5185 ci: backport reliability fixes to v5 without changing runner images](https://github.com/dashpay/platform/pull/5185) — 🐢 targets v6.0-dev · areas: github

<a id="infraclaw-dash-stale"></a>
#### Stale (4)
- [dashpay/platform#3958 ci: add Platform testnet sync status reporting](https://github.com/dashpay/platform/pull/3958) — 2 unresolved (2 bot) · 99 days stale · ⚠ merge conflict · 🐢 targets v5.1-dev · areas: github, fallback
  - Top thread: "🟡 Suggestion: workflow_dispatch lets any repo writer forge a sync status**" — 99 days old
- [dashpay/platform#5167 ci: align secp feature-base workflows with provisioned runner images](https://github.com/dashpay/platform/pull/5167) — 3 unresolved (3 bot) · 2 days stale · 🐢 targets chore/bump-rust-dashcore-secp-033 · areas: github
  - Top thread: "🟡 Suggestion: Contract-failure error points to a doc file missing from this branch**" — 2 days old
- [dashpay/platform#5166 ci: adopt verified Rust and Kotlin runner images on v4.3](https://github.com/dashpay/platform/pull/5166) — ⚠ merge conflict · 🐢 targets v5.1-dev · areas: github
- [dashpay/platform#5185 ci: backport reliability fixes to v5 without changing runner images](https://github.com/dashpay/platform/pull/5185) — 🐢 targets v6.0-dev · areas: github

<a id="bfoss765"></a>
### @bfoss765
<a id="bfoss765-open"></a>
#### Open (2)
- [dashpay/platform#4313 feat(kotlin-sdk): one-time Orchard key shielded-invite API (inviter + claim)](https://github.com/dashpay/platform/pull/4313) — 2 unresolved (2 bot) · 31 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · 🐢 targets v5.1-dev · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk, fallback
  - Top thread: "🟡 Suggestion: Preserve claim-binding mismatch semantics across the FFI boundary**" — 31 days old
- [dashpay/platform#4312 feat(platform-wallet): multi-output shielded transfers + output-aware fee predictor](https://github.com/dashpay/platform/pull/4312) — 1 unresolved (1 bot) · 31 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · 🐢 targets v5.1-dev · areas: dpp, rs-platform-wallet, rs-platform-wallet-ffi, kotlin-sdk, fallback
  - Top thread: "🟡 Suggestion: Keep envelope-aware action counting behind the crate boundary**" — 31 days old

<a id="bfoss765-stale"></a>
#### Stale (2)
- [dashpay/platform#4313 feat(kotlin-sdk): one-time Orchard key shielded-invite API (inviter + claim)](https://github.com/dashpay/platform/pull/4313) — 2 unresolved (2 bot) · 31 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · 🐢 targets v5.1-dev · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk, fallback
  - Top thread: "🟡 Suggestion: Preserve claim-binding mismatch semantics across the FFI boundary**" — 31 days old
- [dashpay/platform#4312 feat(platform-wallet): multi-output shielded transfers + output-aware fee predictor](https://github.com/dashpay/platform/pull/4312) — 1 unresolved (1 bot) · 31 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · 🐢 targets v5.1-dev · areas: dpp, rs-platform-wallet, rs-platform-wallet-ffi, kotlin-sdk, fallback
  - Top thread: "🟡 Suggestion: Keep envelope-aware action counting behind the crate boundary**" — 31 days old

<a id="ktechmidas"></a>
### @ktechmidas
<a id="ktechmidas-open"></a>
#### Open (8)
- [dashpay/platform#5188 ci: tolerate optional S3 cache export outages on chore/bump-rust-dashcore-secp-033](https://github.com/dashpay/platform/pull/5188) — 1 unresolved (1 bot) · 2 days stale · 🐢 targets chore/bump-rust-dashcore-secp-033 · areas: github
  - Top thread: "🟡 Suggestion: New regression tests are not executed by any CI workflow on this branch**" — 2 days old
- [dashpay/platform#5172 ci: stage reviewed AMD64 recipe for candidate validation](https://github.com/dashpay/platform/pull/5172) — areas: github · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 2c25cdf6c30ac2fc256390a6faef55e5464241f5 after bot completion
- [dashpay/platform#5186 ci: tolerate optional S3 cache export outages on v5.0-dev](https://github.com/dashpay/platform/pull/5186) — 🔴 CI failing · 🐢 targets v6.0-dev · areas: github
- [dashpay/platform#5187 ci: tolerate optional S3 cache export outages on v4.3-dev](https://github.com/dashpay/platform/pull/5187) — 🐢 targets v5.1-dev · areas: github
- [dashpay/platform#5198 ci: bound default Binaryen optimizer threads in NPM builds](https://github.com/dashpay/platform/pull/5198) — 🐢 targets v6.0-dev · areas: github
- [dashpay/platform#5203 fix(release): install verified cargo-binstall binary directly](https://github.com/dashpay/platform/pull/5203) — areas: fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed f8fee0b29313e97840ce6a67fa335e90b5da7301 after bot completion
- [dashpay/platform#5204 fix(release): install verified cargo-binstall binary directly](https://github.com/dashpay/platform/pull/5204) — 🐢 targets v5.1-dev · areas: fallback
- [dashpay/platform#5205 fix(release): install verified cargo-binstall binary directly](https://github.com/dashpay/platform/pull/5205) — 🐢 targets v6.0-dev · areas: fallback

<a id="ktechmidas-stale"></a>
#### Stale (6)
- [dashpay/platform#5188 ci: tolerate optional S3 cache export outages on chore/bump-rust-dashcore-secp-033](https://github.com/dashpay/platform/pull/5188) — 1 unresolved (1 bot) · 2 days stale · 🐢 targets chore/bump-rust-dashcore-secp-033 · areas: github
  - Top thread: "🟡 Suggestion: New regression tests are not executed by any CI workflow on this branch**" — 2 days old
- [dashpay/platform#5186 ci: tolerate optional S3 cache export outages on v5.0-dev](https://github.com/dashpay/platform/pull/5186) — 🔴 CI failing · 🐢 targets v6.0-dev · areas: github
- [dashpay/platform#5187 ci: tolerate optional S3 cache export outages on v4.3-dev](https://github.com/dashpay/platform/pull/5187) — 🐢 targets v5.1-dev · areas: github
- [dashpay/platform#5198 ci: bound default Binaryen optimizer threads in NPM builds](https://github.com/dashpay/platform/pull/5198) — 🐢 targets v6.0-dev · areas: github
- [dashpay/platform#5204 fix(release): install verified cargo-binstall binary directly](https://github.com/dashpay/platform/pull/5204) — 🐢 targets v5.1-dev · areas: fallback
- [dashpay/platform#5205 fix(release): install verified cargo-binstall binary directly](https://github.com/dashpay/platform/pull/5205) — 🐢 targets v6.0-dev · areas: fallback

<a id="ktechmidas-clean"></a>
#### Clean (2)
- [dashpay/platform#5172 ci: stage reviewed AMD64 recipe for candidate validation](https://github.com/dashpay/platform/pull/5172) — areas: github · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 2c25cdf6c30ac2fc256390a6faef55e5464241f5 after bot completion
- [dashpay/platform#5203 fix(release): install verified cargo-binstall binary directly](https://github.com/dashpay/platform/pull/5203) — areas: fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed f8fee0b29313e97840ce6a67fa335e90b5da7301 after bot completion

<a id="dcg-claude"></a>
### @DCG-Claude
<a id="dcg-claude-open"></a>
#### Open (19)
- [dashpay/platform#4703 fix(platform): resolve fee versions by registered number and price refunds at the storage epoch rate](https://github.com/dashpay/platform/pull/4703) — 🐢 targets v5.1-dev · areas: rs-drive, rs-drive-abci, dpp, fallback
- [dashpay/platform#4704 fix(platform): record the genesis fee generation and add replay coverage across a fee-version boundary](https://github.com/dashpay/platform/pull/4704) — 🐢 targets v5.1-dev · areas: rs-drive-abci, fallback
- [dashpay/platform#4705 feat(platform): define protocol-versioned smart-contract computation limits and their gas representation](https://github.com/dashpay/platform/pull/4705) — 🐢 targets v6.0-dev · areas: rs-drive, rs-drive-abci, dpp, fallback
- [dashpay/platform#4706 feat(platform)!: require fee history for storage refunds and credit their recorded owners](https://github.com/dashpay/platform/pull/4706) — 🔴 CI failing · 🐢 targets v5.1-dev · areas: rs-drive, rs-drive-abci, fallback
- [dashpay/platform#4707 feat: alloc-only profiles for platform-value and platform-serialization](https://github.com/dashpay/platform/pull/4707) — 🔴 CI failing · 🐢 targets v5.1-dev · areas: github, fallback
- [dashpay/platform#4712 feat(platform): add the dashvm-validation crate and the DashVM protocol table](https://github.com/dashpay/platform/pull/4712) — ⚠ merge conflict · 🔴 CI failing · 🐢 targets v6.0-dev · areas: github, fallback
- [dashpay/platform#4716 feat(drive)!: add the contract credits root sum tree to genesis, upgrade and credit conservation](https://github.com/dashpay/platform/pull/4716) — 🔴 CI failing · 🐢 targets v6.0-dev · areas: rs-drive, rs-drive-abci, dpp, fallback
- [dashpay/platform#4717 feat(platform)!: add family-specific state transition size limits and the large contract envelope decode path](https://github.com/dashpay/platform/pull/4717) — 🐢 targets v6.0-dev · areas: rs-drive, rs-drive-abci, dpp, dashmate, rust-dapi, fallback
- [dashpay/platform#4718 ci: compare cross-architecture replay artifacts across a protocol upgrade and a restart](https://github.com/dashpay/platform/pull/4718) — 🔴 CI failing · 🐢 targets v6.0-dev · areas: rs-drive-abci, github, fallback
- [dashpay/platform#4719 feat: add the dash-sdk-contract declaration model, grammar and diagnostics](https://github.com/dashpay/platform/pull/4719) — 🔴 CI failing · 🐢 targets v5.1-dev · areas: github, fallback
- [dashpay/platform#4784 feat(platform)!: validate contested index parameters and bind the native award to its poll](https://github.com/dashpay/platform/pull/4784) — ⚠ merge conflict · 🔴 CI failing · 🐢 targets v6.0-dev · areas: rs-drive, rs-drive-abci, dpp, fallback
- [dashpay/platform#4789 feat(platform): add typed refund owners to storage flags and fee refunds](https://github.com/dashpay/platform/pull/4789) — 🐢 targets v6.0-dev · areas: rs-drive, dpp, fallback
- [dashpay/platform#4790 feat(drive)!: keep per-issuer token supply rollups and a destroyed-issuer ledger](https://github.com/dashpay/platform/pull/4790) — ⚠ merge conflict · 🔴 CI failing · 🐢 targets v6.0-dev · areas: rs-drive, rs-drive-abci, dpp, fallback
- [dashpay/platform#4794 feat: pin contract configuration vectors and mirror them in wasm-dpp2 and the sdk ffi](https://github.com/dashpay/platform/pull/4794) — 🔴 CI failing · 🐢 targets v5.1-dev · areas: dpp, rust-sdk-ffi, fallback
- [dashpay/platform#4808 test(drive-abci): rehearse a reproducible scheduled host fault and the existing recovery path](https://github.com/dashpay/platform/pull/4808) — 🐢 targets v6.0-dev · areas: rs-drive-abci, github, fallback
- [dashpay/platform#4934 docs: correct the smart-contract target, version counts, transition list and mobile sdk availability](https://github.com/dashpay/platform/pull/4934) — 🐢 targets v6.0-dev · areas: swift-sdk, fallback
- [dashpay/platform#4938 docs(platform): distinguish current-state proofs, stored receipts and execution result text](https://github.com/dashpay/platform/pull/4938) — 🐢 targets v6.0-dev · areas: dpp, rust-sdk, fallback
- [dashpay/platform#4939 ci: audit the dashvm engine dependency set and document the equivalence hotfix rule](https://github.com/dashpay/platform/pull/4939) — 🐢 targets v6.0-dev · areas: github, fallback
- [dashpay/platform#5211 feat(platform)!: add compilation readiness rounds, reports and funds to drive](https://github.com/dashpay/platform/pull/5211) — 🐢 targets v6.0-dev · areas: rs-drive, rs-drive-abci, dpp, fallback

<a id="dcg-claude-stale"></a>
#### Stale (19)
- [dashpay/platform#4703 fix(platform): resolve fee versions by registered number and price refunds at the storage epoch rate](https://github.com/dashpay/platform/pull/4703) — 🐢 targets v5.1-dev · areas: rs-drive, rs-drive-abci, dpp, fallback
- [dashpay/platform#4704 fix(platform): record the genesis fee generation and add replay coverage across a fee-version boundary](https://github.com/dashpay/platform/pull/4704) — 🐢 targets v5.1-dev · areas: rs-drive-abci, fallback
- [dashpay/platform#4705 feat(platform): define protocol-versioned smart-contract computation limits and their gas representation](https://github.com/dashpay/platform/pull/4705) — 🐢 targets v6.0-dev · areas: rs-drive, rs-drive-abci, dpp, fallback
- [dashpay/platform#4706 feat(platform)!: require fee history for storage refunds and credit their recorded owners](https://github.com/dashpay/platform/pull/4706) — 🔴 CI failing · 🐢 targets v5.1-dev · areas: rs-drive, rs-drive-abci, fallback
- [dashpay/platform#4707 feat: alloc-only profiles for platform-value and platform-serialization](https://github.com/dashpay/platform/pull/4707) — 🔴 CI failing · 🐢 targets v5.1-dev · areas: github, fallback
- [dashpay/platform#4712 feat(platform): add the dashvm-validation crate and the DashVM protocol table](https://github.com/dashpay/platform/pull/4712) — ⚠ merge conflict · 🔴 CI failing · 🐢 targets v6.0-dev · areas: github, fallback
- [dashpay/platform#4716 feat(drive)!: add the contract credits root sum tree to genesis, upgrade and credit conservation](https://github.com/dashpay/platform/pull/4716) — 🔴 CI failing · 🐢 targets v6.0-dev · areas: rs-drive, rs-drive-abci, dpp, fallback
- [dashpay/platform#4717 feat(platform)!: add family-specific state transition size limits and the large contract envelope decode path](https://github.com/dashpay/platform/pull/4717) — 🐢 targets v6.0-dev · areas: rs-drive, rs-drive-abci, dpp, dashmate, rust-dapi, fallback
- [dashpay/platform#4718 ci: compare cross-architecture replay artifacts across a protocol upgrade and a restart](https://github.com/dashpay/platform/pull/4718) — 🔴 CI failing · 🐢 targets v6.0-dev · areas: rs-drive-abci, github, fallback
- [dashpay/platform#4719 feat: add the dash-sdk-contract declaration model, grammar and diagnostics](https://github.com/dashpay/platform/pull/4719) — 🔴 CI failing · 🐢 targets v5.1-dev · areas: github, fallback
- [dashpay/platform#4784 feat(platform)!: validate contested index parameters and bind the native award to its poll](https://github.com/dashpay/platform/pull/4784) — ⚠ merge conflict · 🔴 CI failing · 🐢 targets v6.0-dev · areas: rs-drive, rs-drive-abci, dpp, fallback
- [dashpay/platform#4789 feat(platform): add typed refund owners to storage flags and fee refunds](https://github.com/dashpay/platform/pull/4789) — 🐢 targets v6.0-dev · areas: rs-drive, dpp, fallback
- [dashpay/platform#4790 feat(drive)!: keep per-issuer token supply rollups and a destroyed-issuer ledger](https://github.com/dashpay/platform/pull/4790) — ⚠ merge conflict · 🔴 CI failing · 🐢 targets v6.0-dev · areas: rs-drive, rs-drive-abci, dpp, fallback
- [dashpay/platform#4794 feat: pin contract configuration vectors and mirror them in wasm-dpp2 and the sdk ffi](https://github.com/dashpay/platform/pull/4794) — 🔴 CI failing · 🐢 targets v5.1-dev · areas: dpp, rust-sdk-ffi, fallback
- [dashpay/platform#4808 test(drive-abci): rehearse a reproducible scheduled host fault and the existing recovery path](https://github.com/dashpay/platform/pull/4808) — 🐢 targets v6.0-dev · areas: rs-drive-abci, github, fallback
- [dashpay/platform#4934 docs: correct the smart-contract target, version counts, transition list and mobile sdk availability](https://github.com/dashpay/platform/pull/4934) — 🐢 targets v6.0-dev · areas: swift-sdk, fallback
- [dashpay/platform#4938 docs(platform): distinguish current-state proofs, stored receipts and execution result text](https://github.com/dashpay/platform/pull/4938) — 🐢 targets v6.0-dev · areas: dpp, rust-sdk, fallback
- [dashpay/platform#4939 ci: audit the dashvm engine dependency set and document the equivalence hotfix rule](https://github.com/dashpay/platform/pull/4939) — 🐢 targets v6.0-dev · areas: github, fallback
- [dashpay/platform#5211 feat(platform)!: add compilation readiness rounds, reports and funds to drive](https://github.com/dashpay/platform/pull/5211) — 🐢 targets v6.0-dev · areas: rs-drive, rs-drive-abci, dpp, fallback

<a id="hashengineering"></a>
### @HashEngineering
<a id="hashengineering-open"></a>
#### Open (3)
- [dashpay/platform#4439 fix(kotlin-sdk): reconcile the TXO store against the engine and repair restored address pools](https://github.com/dashpay/platform/pull/4439) — 📝 draft · 🐢 targets v5.1-dev · areas: rs-platform-wallet-ffi, kotlin-sdk, fallback
- [dashpay/platform#5026 fix(platform-wallet)!: persist DashPay coreHeight backfill coverage so a relaunch resumes instead of rewinding again](https://github.com/dashpay/platform/pull/5026) — areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, kotlin-sdk, fallback · Policy: waiting-bots
  - Blocker: coderabbitai has not reported for the current head
- [dashpay/rust-dashcore#979 fix(key-wallet, dash-spv): re-emit late knowledge — record corrections, durable rescans, address-pool repair](https://github.com/dashpay/rust-dashcore/pull/979) — ⚠ merge conflict · 📝 draft · areas: dash-spv, key-wallet · Policy: draft
  - Blocker: Draft PR does not occupy a review slot

<a id="hashengineering-draft"></a>
#### Draft (1)
- [dashpay/rust-dashcore#979 fix(key-wallet, dash-spv): re-emit late knowledge — record corrections, durable rescans, address-pool repair](https://github.com/dashpay/rust-dashcore/pull/979) — ⚠ merge conflict · 📝 draft · areas: dash-spv, key-wallet · Policy: draft
  - Blocker: Draft PR does not occupy a review slot

<a id="hashengineering-stale"></a>
#### Stale (1)
- [dashpay/platform#4439 fix(kotlin-sdk): reconcile the TXO store against the engine and repair restored address pools](https://github.com/dashpay/platform/pull/4439) — 📝 draft · 🐢 targets v5.1-dev · areas: rs-platform-wallet-ffi, kotlin-sdk, fallback

<a id="hashengineering-clean"></a>
#### Clean (1)
- [dashpay/platform#5026 fix(platform-wallet)!: persist DashPay coreHeight backfill coverage so a relaunch resumes instead of rewinding again](https://github.com/dashpay/platform/pull/5026) — areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, kotlin-sdk, fallback · Policy: waiting-bots
  - Blocker: coderabbitai has not reported for the current head

## Methodology
Generated nightly by [pr-hygiene](https://github.com/dashpay/stale_prs_are_bad). A thread counts as "unresolved" when it is open, not outdated, has a comment from someone other than the PR author, and the most recent comment is from a reviewer. **Dirty** = at least one such thread. **Unresolved Comments** = at least one such thread. **Changes Requested** = no unresolved threads but a reviewer's most recent review is CHANGES_REQUESTED (still blocking until someone re-approves or dismisses). **Deferred** = carries a configured deferred label (e.g. `postponed`) — visible but not counted toward unresolved-comment counts. **Stale** = targets a non-default branch OR hasn't been touched in the configured threshold (default 120 days, but clean PRs are never reclassified as stale). **Draft** = the PR is still marked draft on GitHub. **CI failing** = no unresolved comments, no changes-requested, but the latest commit's status check is failing. **Clean** = open, not draft, not deferred, not stale, no unresolved comments, no changes-requested, CI green. **Needs action** further requires changes-requested, merge conflict, or that the reviewer commented more recently than the author last pushed. **Ready for human** counts a person's own PRs that the shared review engine marks `ready-for-human` or `ready-to-merge`; each PR bullet shows the engine's state as `Policy:` with its blockers. **Ready for Review** counts clean PRs (authored by someone else) where this person owes a review: the union of the shared policy's routing (owners and reviewers of every area the changed files fall into, or the repository fallback for files no area claims) and GitHub's explicit review requests. The author is never routed to their own PR, and anyone who has already submitted any review is excluded — their job is done. `⚠ ownership unresolved` marks areas whose roster the policy still lists as open. Configurable via [`https://github.com/dashpay/stale_prs_are_bad/blob/master/.pr-hygiene.yml`](https://github.com/dashpay/stale_prs_are_bad/blob/master/.pr-hygiene.yml)—edit defaults there; ownership lives in `policies/`.
