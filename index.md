---
---
# PR Hygiene Report
*Last updated: 2026-09-18 12:15 UTC · commit 6d2fffb*

## Summary
- Open PRs: **99** (15 clean · 4 CI failing · 5 changes requested · 16 unresolved comments · 0 deferred · 14 draft · 45 stale)
- PRs needing author action: **25**
- Total unresolved comments: **146**
- dashpay/platform: **48** open (0 clean · 1 CI failing · 0 changes requested · 9 unresolved comments · 0 deferred · 2 draft · 36 stale) · engine: 2 draft · 10 waiting-bots · 36 no verdict
- dashpay/rust-dashcore: **32** open (7 clean · 2 CI failing · 2 changes requested · 2 unresolved comments · 0 deferred · 11 draft · 8 stale) · engine: 13 configuration-error · 14 draft · 2 waiting-self-review · 1 waiting-slot · 2 no verdict
- dashpay/tenderdash: **3** open (2 clean · 1 CI failing · 0 changes requested · 0 unresolved comments · 0 deferred · 0 draft · 0 stale) · engine: 3 waiting-self-review
- dashpay/grovedb: **1** open (0 clean · 0 CI failing · 0 changes requested · 1 unresolved comments · 0 deferred · 0 draft · 0 stale) · engine: 1 waiting-bots
- dashpay/dash-evo-tool: **15** open (6 clean · 0 CI failing · 3 changes requested · 4 unresolved comments · 0 deferred · 1 draft · 1 stale) · engine: 1 draft · 13 waiting-bots · 1 no verdict

## Scoreboard
_Sort: unresolved-comments desc → needs-action desc → ready-for-review desc. Click any number to jump to the specific PRs it covers._

| Author | Open | Clean | CI failing | Unresolved Comments | Changes Requested | Deferred | Draft | Stale | Needs action | Ready for human | Total Unresolved Comments | Ready for Review | Δ |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| [@PastaPastaPasta + (@thepastaclaw)](#pastapastapasta) | [20+(11)](#pastapastapasta-open) | [0+(6)](#pastapastapasta-clean) | [1+(1)](#pastapastapasta-ci-failing) | [4+(0)](#pastapastapasta-unresolved-comments) | — | — | [2+(2)](#pastapastapasta-draft) | [13+(2)](#pastapastapasta-stale) | [5+(4)](#pastapastapasta-needs-action) | — | [36+(6)](#pastapastapasta-unresolved-comments) | — | ↓ 6 |
| [@lklimek + (@Claudius-Maginificent)](#lklimek) | [10+(5)](#lklimek-open) | [2+(0)](#lklimek-clean) | [1+(0)](#lklimek-ci-failing) | [3+(1)](#lklimek-unresolved-comments) | [0+(3)](#lklimek-changes-requested) | — | [1+(0)](#lklimek-draft) | [3+(1)](#lklimek-stale) | [2+(4)](#lklimek-needs-action) | — | [4+(10)](#lklimek-unresolved-comments) | [4+(0)](#lklimek-ready-for-review) | ↑ 2 |
| [@QuantumExplorer](#quantumexplorer) | [13](#quantumexplorer-open) | — | [1](#quantumexplorer-ci-failing) | [3](#quantumexplorer-unresolved-comments) | — | — | [1](#quantumexplorer-draft) | [8](#quantumexplorer-stale) | [3](#quantumexplorer-needs-action) | — | [20](#quantumexplorer-unresolved-comments) | [5](#quantumexplorer-ready-for-review) | ↓ 2 |
| [@llbartekll](#llbartekll) | [4](#llbartekll-open) | — | — | [2](#llbartekll-unresolved-comments) | [1](#llbartekll-changes-requested) | — | [1](#llbartekll-draft) | — | [3](#llbartekll-needs-action) | — | [5](#llbartekll-unresolved-comments) | — | ↑ 1 |
| [@xdustinface](#xdustinface) | [8](#xdustinface-open) | [1](#xdustinface-clean) | — | [1](#xdustinface-unresolved-comments) | — | — | [3](#xdustinface-draft) | [3](#xdustinface-stale) | [1](#xdustinface-needs-action) | — | [1](#xdustinface-unresolved-comments) | [5](#xdustinface-ready-for-review) | — |
| [@shumkov](#shumkov) | [11](#shumkov-open) | [1](#shumkov-clean) | — | [1](#shumkov-unresolved-comments) | — | — | — | [9](#shumkov-stale) | [1](#shumkov-needs-action) | — | [54](#shumkov-unresolved-comments) | [1](#shumkov-ready-for-review) | ↓ 5 |
| [@romchornyi](#romchornyi) | [2](#romchornyi-open) | [1](#romchornyi-clean) | — | [1](#romchornyi-unresolved-comments) | — | — | — | — | [1](#romchornyi-needs-action) | — | [2](#romchornyi-unresolved-comments) | — | ↓ 3 |
| [@ZocoLini](#zocolini) | [9](#zocolini-open) | [4](#zocolini-clean) | — | — | [1](#zocolini-changes-requested) | — | [3](#zocolini-draft) | [1](#zocolini-stale) | [1](#zocolini-needs-action) | — | [2](#zocolini-unresolved-comments) | [1](#zocolini-ready-for-review) | ↑ 1 |
| [@HashEngineering](#hashengineering) | [2](#hashengineering-open) | — | — | — | — | — | [1](#hashengineering-draft) | [1](#hashengineering-stale) | — | — | — | [1](#hashengineering-ready-for-review) | ↓ 2 |
| [@infraclaw-dash](#infraclaw-dash) | [2](#infraclaw-dash-open) | — | — | — | — | — | — | [2](#infraclaw-dash-stale) | — | — | [3](#infraclaw-dash-unresolved-comments) | — | ↓ 1 |
| [@bfoss765](#bfoss765) | [2](#bfoss765-open) | — | — | — | — | — | — | [2](#bfoss765-stale) | — | — | [3](#bfoss765-unresolved-comments) | — | ↓ 2 |

## Per-author detail

<a id="pastapastapasta"></a>
### @PastaPastaPasta + (@thepastaclaw)
<a id="pastapastapasta-open"></a>
#### Open (31)
- [dashpay/rust-dashcore#264 feat(dash-spv): Add BIP324 v2 encrypted P2P transport](https://github.com/dashpay/rust-dashcore/pull/264) — 3 unresolved (3 CodeRabbit) · 281 days stale · ⚠ merge conflict · 🔴 CI failing · 🐢 untouched 121 days · areas: dash-spv, fallback · ⚠ ownership unresolved: dash-spv · Policy: configuration-error
  - Top thread: "_⚠️ Potential issue_ \| _🟡 Minor_" — 281 days old
  - Blocker: Unresolved identities in dash-spv
- [dashpay/platform#3096 feat(sdk): add client-side validation to state transition construction methods](https://github.com/dashpay/platform/pull/3096) — via @thepastaclaw · 4 unresolved (2 CodeRabbit, 2 human) · 212 days stale · ⚠ merge conflict · 🐢 targets v4.3-dev · areas: rs-drive-abci, dpp, js-wasm-sdk, fallback
  - Top thread: "_⚠️ Potential issue_ \| _🟡 Minor_" — 209 days old
- [dashpay/rust-dashcore#137 fix: integration tests, continued dashificiation](https://github.com/dashpay/rust-dashcore/pull/137) — 2 unresolved (2 CodeRabbit) · 378 days stale · ⚠ merge conflict · 🔴 CI failing · 🐢 targets v0.40-dev, untouched 281 days · areas: fallback
  - Top thread: "_🛠️ Refactor suggestion_" — 378 days old
- [dashpay/platform#4530 test(dashmate): live state sync e2e — join tooling, churn, re-sync and fallback coverage](https://github.com/dashpay/platform/pull/4530) — 6 unresolved (6 bot) · 19 days stale · ⚠ merge conflict · 📝 draft · 🐢 targets v4.3-dev · areas: rs-drive, rs-drive-abci, dpp, dashmate, js-wasm-sdk, fallback
  - Top thread: "🟡 Suggestion: Require the churn scenario to interrupt an active state sync**" — 19 days old
- [dashpay/platform#4616 feat(platform-wallet): support DashPay shielded tips with dedicated accounts](https://github.com/dashpay/platform/pull/4616) — 7 unresolved (2 CodeRabbit, 5 bot) · 3 days stale · 🐢 targets v4.3-dev · areas: rs-drive, rs-drive-abci, rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk, rust-sdk, system-contracts, kotlin-sdk, fallback
  - Top thread: "_🗄️ Data Integrity & Integration_ \| _🟠 Major_ \| _🏗️ Heavy lift_" — 3 days old
- [dashpay/platform#4562 ci: build release SDKs and NPM packages on self-hosted runners](https://github.com/dashpay/platform/pull/4562) — 3 unresolved (3 CodeRabbit) · 17 days stale · 🔴 CI failing · 🐢 targets v4.3-dev · areas: fallback
  - Top thread: "_🎯 Functional Correctness_ \| _🟠 Major_ \| _⚡ Quick win_" — 17 days old
- [dashpay/rust-dashcore#901 fix(key-wallet): discover Coinbase/AssetUnlock outputs for all fund-bearing accounts](https://github.com/dashpay/rust-dashcore/pull/901) — via @thepastaclaw · 2 unresolved (2 CodeRabbit) · 43 days stale · ⚠ merge conflict · ✋ changes requested · 📝 draft · areas: dash-spv, key-wallet · ⚠ ownership unresolved: dash-spv, key-wallet · Policy: draft
  - Top thread: "_🎯 Functional Correctness_ \| _🟡 Minor_ \| _⚡ Quick win_" — 43 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#4648 feat(drive-abci)!: state sync via ABCI snapshots with reduced platform state (protocol v15)](https://github.com/dashpay/platform/pull/4648) — 3 unresolved (3 bot) · 8 days stale · ⚠ merge conflict · 🔴 CI failing · 🐢 targets v4.3-dev · areas: rs-drive, rs-drive-abci, dpp, fallback
  - Top thread: "🟡 Suggestion: Do not silently skip unsupported trees beyond a full discovery page**" — 8 days old
- [dashpay/platform#4521 feat(dashmate): state sync configuration for tenderdash and drive snapshots](https://github.com/dashpay/platform/pull/4521) — 2 unresolved (2 bot) · 19 days stale · 📝 draft · 🐢 targets v4.3-dev · areas: dashmate
  - Top thread: "🔴 Blocking: Default snapshot serving exposes unbounded remote checkpoint retention**" — 19 days old
- [dashpay/platform#4538 fix(sdk): honor explicit regtest addresses in dapi-client; tolerate empty discovery in wasm-sdk trusted context](https://github.com/dashpay/platform/pull/4538) — 2 unresolved (2 bot) · 19 days stale · ⚠ merge conflict · 📝 draft · areas: js-wasm-sdk, fallback · Policy: draft
  - Top thread: "🟡 Suggestion: Add deterministic tests for the network-scoped discovery fallback**" — 19 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#4632 feat(sdk)!: pure DPNS and DashPay document builders shared with embedders](https://github.com/dashpay/platform/pull/4632) — 2 unresolved (2 bot) · 7 days stale · ✋ changes requested · areas: rust-sdk, fallback · Policy: waiting-bots
  - Top thread: "🟡 Suggestion: Builder documentation recommends a pre-check that rejects valid contract labels**" — 7 days old
  - Blocker: Bot review threads remain unresolved
- [dashpay/platform#4539 fix(dashmate): refresh quorum-server seeds and complete node identities](https://github.com/dashpay/platform/pull/4539) — 2 unresolved (1 CodeRabbit, 1 bot) · 2 days stale · 🐢 targets v4.3-dev · areas: dashmate, fallback
  - Top thread: "_🩺 Stability & Availability_ \| _🟡 Minor_ \| _⚡ Quick win_" — 2 days old
- [dashpay/platform#4392 perf(swift-sdk): linear wallet-changeset rounds via per-round bulk-prefetch cache](https://github.com/dashpay/platform/pull/4392) — 1 unresolved (1 bot) · 9 days stale · ⚠ merge conflict · ✋ changes requested · areas: swift-sdk · Policy: waiting-bots
  - Top thread: "🟡 Suggestion: Exercise transient pending-input fetch failures followed by recovery**" — 9 days old
  - Blocker: Bot review threads remain unresolved
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/platform#4623 feat(platform-wallet): reserve DashPay payout addresses without Core funding](https://github.com/dashpay/platform/pull/4623) — 2 unresolved (2 bot) · 0 days stale · ✋ changes requested · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk · Policy: waiting-bots
  - Top thread: "🟡 Suggestion: Null the string out-param before the signer null-check**" — 0 days old
  - Blocker: Bot review threads remain unresolved
- [dashpay/platform#4432 fix(sdk): mirror the COUNT dispatcher's outer-walk limit when verifying range-outer carrier proofs](https://github.com/dashpay/platform/pull/4432) — 1 unresolved (1 bot) · 1 days stale · 🔴 CI failing · areas: rs-drive, fallback · Policy: waiting-bots
  - Top thread: "🟡 Suggestion: Share carrier-limit lowering with the Drive dispatcher**" — 1 days old
  - Blocker: Bot review threads remain unresolved
- [dashpay/dash-evo-tool#761 fix(ui): improve auth key error message and suppress on startup auto-select](https://github.com/dashpay/dash-evo-tool/pull/761) — via @thepastaclaw · ⚠ merge conflict · areas: dash-evo-tool · Policy: waiting-bots
  - Blocker: coderabbitai has not reported for the current head
  - Blocker: Bot review threads remain unresolved
- [dashpay/dash-evo-tool#762 fix: accept HIGH security level keys for profile updates](https://github.com/dashpay/dash-evo-tool/pull/762) — via @thepastaclaw · ⚠ merge conflict · areas: dash-evo-tool · Policy: waiting-bots
  - Blocker: coderabbitai has not reported for the current head
  - Blocker: thepastaclaw has not reported for the current head
  - Blocker: Bot review threads remain unresolved
- [dashpay/dash-evo-tool#772 docs: add weekly smoke test cases and prerequisites](https://github.com/dashpay/dash-evo-tool/pull/772) — via @thepastaclaw · areas: dash-evo-tool · Policy: waiting-bots
  - Blocker: coderabbitai has not reported for the current head
  - Blocker: thepastaclaw has not reported for the current head
  - Blocker: Bot review threads remain unresolved
- [dashpay/dash-evo-tool#853 fix: show parsed platform-address transitions](https://github.com/dashpay/dash-evo-tool/pull/853) — via @thepastaclaw · areas: dash-evo-tool · Policy: waiting-bots
  - Blocker: coderabbitai has not reported for the current head
  - Blocker: Bot review threads remain unresolved
- [dashpay/dash-evo-tool#859 perf: detect system theme off UI thread](https://github.com/dashpay/dash-evo-tool/pull/859) — via @thepastaclaw · areas: dash-evo-tool · Policy: waiting-bots
  - Blocker: coderabbitai has not reported for the current head
  - Blocker: thepastaclaw has not reported for the current head
  - Blocker: Bot review threads remain unresolved
- [dashpay/platform#4298 fix(dashmate): load ZeroSSL config in force mode](https://github.com/dashpay/platform/pull/4298) — via @thepastaclaw · ⚠ merge conflict · 🐢 targets v4.3-dev · areas: dashmate
- [dashpay/platform#4615 feat(sdk)!: optional BIP-39 passphrase through the mnemonic resolver and wallet creation](https://github.com/dashpay/platform/pull/4615) — ⚠ merge conflict · 📝 draft · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, rust-sdk-ffi, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#4618 refactor(drive-abci): share the v1 document-query wire decoders](https://github.com/dashpay/platform/pull/4618) — 🐢 targets v4.3-dev · areas: rs-drive-abci, fallback
- [dashpay/platform#4619 feat(sdk)!: verify document proofs against the wire request and share DPNS/DashPay document assembly](https://github.com/dashpay/platform/pull/4619) — 📝 draft · 🐢 targets refactor/platform-query-wire-and-dpp-bounds · areas: dpp, rust-sdk, fallback
- [dashpay/platform#4620 feat(sdk): add transport-free CXX bindings for C++ embedders](https://github.com/dashpay/platform/pull/4620) — 📝 draft · 🐢 targets feat/request-driven-document-verification · areas: fallback
- [dashpay/platform#4633 feat(sdk): add CXX bindings over dash-sdk for C++ embedders](https://github.com/dashpay/platform/pull/4633) — ⚠ merge conflict · 📝 draft · 🐢 targets feat/shared-dpns-dashpay-builders · areas: dpp, rust-sdk, fallback
- [dashpay/platform#4655 feat(sdk)!: add scoped authentication key SDK support](https://github.com/dashpay/platform/pull/4655) — ⚠ merge conflict · 🐢 targets feat/scoped-contract-auth-keys · areas: rs-platform-wallet-ffi, swift-sdk, rust-sdk-ffi, js-wasm-sdk, kotlin-sdk, fallback
- [dashpay/rust-dashcore#749 fix(ffi): free transaction byte buffers correctly](https://github.com/dashpay/rust-dashcore/pull/749) — via @thepastaclaw · ⚠ merge conflict · 🔴 CI failing · areas: fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed a7d48fb68ca3feb61b15d969085c8b78befc0745 after bot completion
- [dashpay/rust-dashcore#752 fix(ffi): validate account seed lengths](https://github.com/dashpay/rust-dashcore/pull/752) — via @thepastaclaw · ⚠ merge conflict · areas: fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 94918c7f1085cf5b1548088b6fa6dc8473d87664 after bot completion
- [dashpay/rust-dashcore#904 fix(dash-spv): harden Windows dashd_sync startup wallet load](https://github.com/dashpay/rust-dashcore/pull/904) — via @thepastaclaw · ⚠ merge conflict · 📝 draft · areas: dash-spv · ⚠ ownership unresolved: dash-spv · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#959 feat(dash-spv): adaptive gap-limit probe escalation for BIP44 discovery](https://github.com/dashpay/rust-dashcore/pull/959) — ⚠ merge conflict · 🔴 CI failing · areas: dash-spv, key-wallet, key-wallet-manager, fallback · ⚠ ownership unresolved: dash-spv, key-wallet, key-wallet-manager · Policy: configuration-error
  - Blocker: Unresolved identities in dash-spv

<a id="pastapastapasta-needs-action"></a>
#### Needs action (9)
- [dashpay/platform#4632 feat(sdk)!: pure DPNS and DashPay document builders shared with embedders](https://github.com/dashpay/platform/pull/4632) — 2 unresolved (2 bot) · 7 days stale · ✋ changes requested · areas: rust-sdk, fallback · Policy: waiting-bots
  - Top thread: "🟡 Suggestion: Builder documentation recommends a pre-check that rejects valid contract labels**" — 7 days old
  - Blocker: Bot review threads remain unresolved
- [dashpay/platform#4392 perf(swift-sdk): linear wallet-changeset rounds via per-round bulk-prefetch cache](https://github.com/dashpay/platform/pull/4392) — 1 unresolved (1 bot) · 9 days stale · ⚠ merge conflict · ✋ changes requested · areas: swift-sdk · Policy: waiting-bots
  - Top thread: "🟡 Suggestion: Exercise transient pending-input fetch failures followed by recovery**" — 9 days old
  - Blocker: Bot review threads remain unresolved
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/platform#4623 feat(platform-wallet): reserve DashPay payout addresses without Core funding](https://github.com/dashpay/platform/pull/4623) — 2 unresolved (2 bot) · 0 days stale · ✋ changes requested · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk · Policy: waiting-bots
  - Top thread: "🟡 Suggestion: Null the string out-param before the signer null-check**" — 0 days old
  - Blocker: Bot review threads remain unresolved
- [dashpay/platform#4432 fix(sdk): mirror the COUNT dispatcher's outer-walk limit when verifying range-outer carrier proofs](https://github.com/dashpay/platform/pull/4432) — 1 unresolved (1 bot) · 1 days stale · 🔴 CI failing · areas: rs-drive, fallback · Policy: waiting-bots
  - Top thread: "🟡 Suggestion: Share carrier-limit lowering with the Drive dispatcher**" — 1 days old
  - Blocker: Bot review threads remain unresolved
- [dashpay/dash-evo-tool#761 fix(ui): improve auth key error message and suppress on startup auto-select](https://github.com/dashpay/dash-evo-tool/pull/761) — via @thepastaclaw · ⚠ merge conflict · areas: dash-evo-tool · Policy: waiting-bots
  - Blocker: coderabbitai has not reported for the current head
  - Blocker: Bot review threads remain unresolved
- [dashpay/dash-evo-tool#762 fix: accept HIGH security level keys for profile updates](https://github.com/dashpay/dash-evo-tool/pull/762) — via @thepastaclaw · ⚠ merge conflict · areas: dash-evo-tool · Policy: waiting-bots
  - Blocker: coderabbitai has not reported for the current head
  - Blocker: thepastaclaw has not reported for the current head
  - Blocker: Bot review threads remain unresolved
- [dashpay/rust-dashcore#749 fix(ffi): free transaction byte buffers correctly](https://github.com/dashpay/rust-dashcore/pull/749) — via @thepastaclaw · ⚠ merge conflict · 🔴 CI failing · areas: fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed a7d48fb68ca3feb61b15d969085c8b78befc0745 after bot completion
- [dashpay/rust-dashcore#752 fix(ffi): validate account seed lengths](https://github.com/dashpay/rust-dashcore/pull/752) — via @thepastaclaw · ⚠ merge conflict · areas: fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 94918c7f1085cf5b1548088b6fa6dc8473d87664 after bot completion
- [dashpay/rust-dashcore#959 feat(dash-spv): adaptive gap-limit probe escalation for BIP44 discovery](https://github.com/dashpay/rust-dashcore/pull/959) — ⚠ merge conflict · 🔴 CI failing · areas: dash-spv, key-wallet, key-wallet-manager, fallback · ⚠ ownership unresolved: dash-spv, key-wallet, key-wallet-manager · Policy: configuration-error
  - Blocker: Unresolved identities in dash-spv

<a id="pastapastapasta-unresolved-comments"></a>
#### Unresolved Comments (4)
- [dashpay/platform#4632 feat(sdk)!: pure DPNS and DashPay document builders shared with embedders](https://github.com/dashpay/platform/pull/4632) — 2 unresolved (2 bot) · 7 days stale · ✋ changes requested · areas: rust-sdk, fallback · Policy: waiting-bots
  - Top thread: "🟡 Suggestion: Builder documentation recommends a pre-check that rejects valid contract labels**" — 7 days old
  - Blocker: Bot review threads remain unresolved
- [dashpay/platform#4392 perf(swift-sdk): linear wallet-changeset rounds via per-round bulk-prefetch cache](https://github.com/dashpay/platform/pull/4392) — 1 unresolved (1 bot) · 9 days stale · ⚠ merge conflict · ✋ changes requested · areas: swift-sdk · Policy: waiting-bots
  - Top thread: "🟡 Suggestion: Exercise transient pending-input fetch failures followed by recovery**" — 9 days old
  - Blocker: Bot review threads remain unresolved
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/platform#4623 feat(platform-wallet): reserve DashPay payout addresses without Core funding](https://github.com/dashpay/platform/pull/4623) — 2 unresolved (2 bot) · 0 days stale · ✋ changes requested · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk · Policy: waiting-bots
  - Top thread: "🟡 Suggestion: Null the string out-param before the signer null-check**" — 0 days old
  - Blocker: Bot review threads remain unresolved
- [dashpay/platform#4432 fix(sdk): mirror the COUNT dispatcher's outer-walk limit when verifying range-outer carrier proofs](https://github.com/dashpay/platform/pull/4432) — 1 unresolved (1 bot) · 1 days stale · 🔴 CI failing · areas: rs-drive, fallback · Policy: waiting-bots
  - Top thread: "🟡 Suggestion: Share carrier-limit lowering with the Drive dispatcher**" — 1 days old
  - Blocker: Bot review threads remain unresolved

<a id="pastapastapasta-ci-failing"></a>
#### CI Failing (2)
- [dashpay/rust-dashcore#749 fix(ffi): free transaction byte buffers correctly](https://github.com/dashpay/rust-dashcore/pull/749) — via @thepastaclaw · ⚠ merge conflict · 🔴 CI failing · areas: fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed a7d48fb68ca3feb61b15d969085c8b78befc0745 after bot completion
- [dashpay/rust-dashcore#959 feat(dash-spv): adaptive gap-limit probe escalation for BIP44 discovery](https://github.com/dashpay/rust-dashcore/pull/959) — ⚠ merge conflict · 🔴 CI failing · areas: dash-spv, key-wallet, key-wallet-manager, fallback · ⚠ ownership unresolved: dash-spv, key-wallet, key-wallet-manager · Policy: configuration-error
  - Blocker: Unresolved identities in dash-spv

<a id="pastapastapasta-draft"></a>
#### Draft (4)
- [dashpay/rust-dashcore#901 fix(key-wallet): discover Coinbase/AssetUnlock outputs for all fund-bearing accounts](https://github.com/dashpay/rust-dashcore/pull/901) — via @thepastaclaw · 2 unresolved (2 CodeRabbit) · 43 days stale · ⚠ merge conflict · ✋ changes requested · 📝 draft · areas: dash-spv, key-wallet · ⚠ ownership unresolved: dash-spv, key-wallet · Policy: draft
  - Top thread: "_🎯 Functional Correctness_ \| _🟡 Minor_ \| _⚡ Quick win_" — 43 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#4538 fix(sdk): honor explicit regtest addresses in dapi-client; tolerate empty discovery in wasm-sdk trusted context](https://github.com/dashpay/platform/pull/4538) — 2 unresolved (2 bot) · 19 days stale · ⚠ merge conflict · 📝 draft · areas: js-wasm-sdk, fallback · Policy: draft
  - Top thread: "🟡 Suggestion: Add deterministic tests for the network-scoped discovery fallback**" — 19 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#4615 feat(sdk)!: optional BIP-39 passphrase through the mnemonic resolver and wallet creation](https://github.com/dashpay/platform/pull/4615) — ⚠ merge conflict · 📝 draft · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, rust-sdk-ffi, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#904 fix(dash-spv): harden Windows dashd_sync startup wallet load](https://github.com/dashpay/rust-dashcore/pull/904) — via @thepastaclaw · ⚠ merge conflict · 📝 draft · areas: dash-spv · ⚠ ownership unresolved: dash-spv · Policy: draft
  - Blocker: Draft PR does not occupy a review slot

<a id="pastapastapasta-stale"></a>
#### Stale (15)
- [dashpay/rust-dashcore#264 feat(dash-spv): Add BIP324 v2 encrypted P2P transport](https://github.com/dashpay/rust-dashcore/pull/264) — 3 unresolved (3 CodeRabbit) · 281 days stale · ⚠ merge conflict · 🔴 CI failing · 🐢 untouched 121 days · areas: dash-spv, fallback · ⚠ ownership unresolved: dash-spv · Policy: configuration-error
  - Top thread: "_⚠️ Potential issue_ \| _🟡 Minor_" — 281 days old
  - Blocker: Unresolved identities in dash-spv
- [dashpay/platform#3096 feat(sdk): add client-side validation to state transition construction methods](https://github.com/dashpay/platform/pull/3096) — via @thepastaclaw · 4 unresolved (2 CodeRabbit, 2 human) · 212 days stale · ⚠ merge conflict · 🐢 targets v4.3-dev · areas: rs-drive-abci, dpp, js-wasm-sdk, fallback
  - Top thread: "_⚠️ Potential issue_ \| _🟡 Minor_" — 209 days old
- [dashpay/rust-dashcore#137 fix: integration tests, continued dashificiation](https://github.com/dashpay/rust-dashcore/pull/137) — 2 unresolved (2 CodeRabbit) · 378 days stale · ⚠ merge conflict · 🔴 CI failing · 🐢 targets v0.40-dev, untouched 281 days · areas: fallback
  - Top thread: "_🛠️ Refactor suggestion_" — 378 days old
- [dashpay/platform#4530 test(dashmate): live state sync e2e — join tooling, churn, re-sync and fallback coverage](https://github.com/dashpay/platform/pull/4530) — 6 unresolved (6 bot) · 19 days stale · ⚠ merge conflict · 📝 draft · 🐢 targets v4.3-dev · areas: rs-drive, rs-drive-abci, dpp, dashmate, js-wasm-sdk, fallback
  - Top thread: "🟡 Suggestion: Require the churn scenario to interrupt an active state sync**" — 19 days old
- [dashpay/platform#4616 feat(platform-wallet): support DashPay shielded tips with dedicated accounts](https://github.com/dashpay/platform/pull/4616) — 7 unresolved (2 CodeRabbit, 5 bot) · 3 days stale · 🐢 targets v4.3-dev · areas: rs-drive, rs-drive-abci, rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk, rust-sdk, system-contracts, kotlin-sdk, fallback
  - Top thread: "_🗄️ Data Integrity & Integration_ \| _🟠 Major_ \| _🏗️ Heavy lift_" — 3 days old
- [dashpay/platform#4562 ci: build release SDKs and NPM packages on self-hosted runners](https://github.com/dashpay/platform/pull/4562) — 3 unresolved (3 CodeRabbit) · 17 days stale · 🔴 CI failing · 🐢 targets v4.3-dev · areas: fallback
  - Top thread: "_🎯 Functional Correctness_ \| _🟠 Major_ \| _⚡ Quick win_" — 17 days old
- [dashpay/platform#4648 feat(drive-abci)!: state sync via ABCI snapshots with reduced platform state (protocol v15)](https://github.com/dashpay/platform/pull/4648) — 3 unresolved (3 bot) · 8 days stale · ⚠ merge conflict · 🔴 CI failing · 🐢 targets v4.3-dev · areas: rs-drive, rs-drive-abci, dpp, fallback
  - Top thread: "🟡 Suggestion: Do not silently skip unsupported trees beyond a full discovery page**" — 8 days old
- [dashpay/platform#4521 feat(dashmate): state sync configuration for tenderdash and drive snapshots](https://github.com/dashpay/platform/pull/4521) — 2 unresolved (2 bot) · 19 days stale · 📝 draft · 🐢 targets v4.3-dev · areas: dashmate
  - Top thread: "🔴 Blocking: Default snapshot serving exposes unbounded remote checkpoint retention**" — 19 days old
- [dashpay/platform#4539 fix(dashmate): refresh quorum-server seeds and complete node identities](https://github.com/dashpay/platform/pull/4539) — 2 unresolved (1 CodeRabbit, 1 bot) · 2 days stale · 🐢 targets v4.3-dev · areas: dashmate, fallback
  - Top thread: "_🩺 Stability & Availability_ \| _🟡 Minor_ \| _⚡ Quick win_" — 2 days old
- [dashpay/platform#4298 fix(dashmate): load ZeroSSL config in force mode](https://github.com/dashpay/platform/pull/4298) — via @thepastaclaw · ⚠ merge conflict · 🐢 targets v4.3-dev · areas: dashmate
- [dashpay/platform#4618 refactor(drive-abci): share the v1 document-query wire decoders](https://github.com/dashpay/platform/pull/4618) — 🐢 targets v4.3-dev · areas: rs-drive-abci, fallback
- [dashpay/platform#4619 feat(sdk)!: verify document proofs against the wire request and share DPNS/DashPay document assembly](https://github.com/dashpay/platform/pull/4619) — 📝 draft · 🐢 targets refactor/platform-query-wire-and-dpp-bounds · areas: dpp, rust-sdk, fallback
- [dashpay/platform#4620 feat(sdk): add transport-free CXX bindings for C++ embedders](https://github.com/dashpay/platform/pull/4620) — 📝 draft · 🐢 targets feat/request-driven-document-verification · areas: fallback
- [dashpay/platform#4633 feat(sdk): add CXX bindings over dash-sdk for C++ embedders](https://github.com/dashpay/platform/pull/4633) — ⚠ merge conflict · 📝 draft · 🐢 targets feat/shared-dpns-dashpay-builders · areas: dpp, rust-sdk, fallback
- [dashpay/platform#4655 feat(sdk)!: add scoped authentication key SDK support](https://github.com/dashpay/platform/pull/4655) — ⚠ merge conflict · 🐢 targets feat/scoped-contract-auth-keys · areas: rs-platform-wallet-ffi, swift-sdk, rust-sdk-ffi, js-wasm-sdk, kotlin-sdk, fallback

<a id="pastapastapasta-clean"></a>
#### Clean (6)
- [dashpay/dash-evo-tool#761 fix(ui): improve auth key error message and suppress on startup auto-select](https://github.com/dashpay/dash-evo-tool/pull/761) — via @thepastaclaw · ⚠ merge conflict · areas: dash-evo-tool · Policy: waiting-bots
  - Blocker: coderabbitai has not reported for the current head
  - Blocker: Bot review threads remain unresolved
- [dashpay/dash-evo-tool#762 fix: accept HIGH security level keys for profile updates](https://github.com/dashpay/dash-evo-tool/pull/762) — via @thepastaclaw · ⚠ merge conflict · areas: dash-evo-tool · Policy: waiting-bots
  - Blocker: coderabbitai has not reported for the current head
  - Blocker: thepastaclaw has not reported for the current head
  - Blocker: Bot review threads remain unresolved
- [dashpay/dash-evo-tool#772 docs: add weekly smoke test cases and prerequisites](https://github.com/dashpay/dash-evo-tool/pull/772) — via @thepastaclaw · areas: dash-evo-tool · Policy: waiting-bots
  - Blocker: coderabbitai has not reported for the current head
  - Blocker: thepastaclaw has not reported for the current head
  - Blocker: Bot review threads remain unresolved
- [dashpay/dash-evo-tool#853 fix: show parsed platform-address transitions](https://github.com/dashpay/dash-evo-tool/pull/853) — via @thepastaclaw · areas: dash-evo-tool · Policy: waiting-bots
  - Blocker: coderabbitai has not reported for the current head
  - Blocker: Bot review threads remain unresolved
- [dashpay/dash-evo-tool#859 perf: detect system theme off UI thread](https://github.com/dashpay/dash-evo-tool/pull/859) — via @thepastaclaw · areas: dash-evo-tool · Policy: waiting-bots
  - Blocker: coderabbitai has not reported for the current head
  - Blocker: thepastaclaw has not reported for the current head
  - Blocker: Bot review threads remain unresolved
- [dashpay/rust-dashcore#752 fix(ffi): validate account seed lengths](https://github.com/dashpay/rust-dashcore/pull/752) — via @thepastaclaw · ⚠ merge conflict · areas: fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 94918c7f1085cf5b1548088b6fa6dc8473d87664 after bot completion

<a id="lklimek"></a>
### @lklimek + (@Claudius-Maginificent)
<a id="lklimek-open"></a>
#### Open (15)
- [dashpay/platform#3549 test(platform-wallet): e2e framework + full test suite — triage pins, Found-*/PA-* guards, fail-closed persist, Stage-2 merge](https://github.com/dashpay/platform/pull/3549) — via @Claudius-Maginificent · 9 unresolved (9 bot) · 139 days stale · ⚠ merge conflict · 📝 draft · 🐢 targets v4.3-dev · areas: rs-drive, rs-drive-abci, rs-platform-wallet, rs-platform-wallet-ffi, rust-sdk, fallback
  - Top thread: "🟡 Suggestion: `from_seed_for_identity` is misleadingly named, half-functional, and unused**" — 139 days old
- [dashpay/dash-evo-tool#851 fix(ui): center text inside buttons with min_size across the app](https://github.com/dashpay/dash-evo-tool/pull/851) — 2 unresolved (2 bot) · 146 days stale · ⚠ merge conflict · areas: dash-evo-tool · Policy: waiting-bots
  - Top thread: "🟡 Suggestion: Add a kittest harness for the new add_sized button-sizing contract**" — 146 days old
  - Blocker: coderabbitai has not reported for the current head
  - Blocker: thepastaclaw has not reported for the current head
  - Blocker: Bot review threads remain unresolved
- [dashpay/dash-evo-tool#983 test(migration): add cross-version migration test matrix](https://github.com/dashpay/dash-evo-tool/pull/983) — via @Claudius-Maginificent · 1 unresolved (1 CodeRabbit) · 7 days stale · ✋ changes requested · areas: dash-evo-tool · Policy: waiting-bots
  - Top thread: "_🩺 Stability & Availability_ \| _🟠 Major_ \| _🏗️ Heavy lift_" — 7 days old
  - Blocker: coderabbitai has not reported for the current head
  - Blocker: thepastaclaw has not reported for the current head
  - Blocker: Bot review threads remain unresolved
- [dashpay/platform#4740 fix(platform-wallet): scan DashPay contact accounts from request height](https://github.com/dashpay/platform/pull/4740) — 1 unresolved (1 bot) · 3 days stale · areas: rs-platform-wallet, swift-sdk · Policy: waiting-bots
  - Top thread: "🟡 Suggestion: Validate the contact-request height before using it as a scan checkpoint**" — 3 days old
  - Blocker: Bot review threads remain unresolved
  - Blocker: Proceeded without thepastaclaw: no review within the configured window
- [dashpay/dash-evo-tool#976 fix(security): protect identity keys before import storage](https://github.com/dashpay/dash-evo-tool/pull/976) — 1 unresolved (1 CodeRabbit) · 0 days stale · ✋ changes requested · areas: dash-evo-tool · Policy: waiting-bots
  - Top thread: "_🗄️ Data Integrity & Integration_ \| _🟡 Minor_ \| _⚡ Quick win_" — 0 days old
  - Blocker: thepastaclaw has not reported for the current head
  - Blocker: Bot review threads remain unresolved
- [dashpay/dash-evo-tool#901 feat(dpns): unify safe masternode voting operations](https://github.com/dashpay/dash-evo-tool/pull/901) — areas: dash-evo-tool · Policy: waiting-bots
  - Blocker: coderabbitai has not reported for the current head
  - Blocker: thepastaclaw has not reported for the current head
- [dashpay/dash-evo-tool#980 feat(identity): add wallet-derived keys to existing identities](https://github.com/dashpay/dash-evo-tool/pull/980) — 📝 draft · areas: dash-evo-tool · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/dash-evo-tool#987 fix(wallet): single source of truth for wallet and key names](https://github.com/dashpay/dash-evo-tool/pull/987) — via @Claudius-Maginificent · ✋ changes requested · areas: dash-evo-tool · Policy: waiting-bots
  - Blocker: coderabbitai has not reported for the current head
  - Blocker: thepastaclaw has not reported for the current head
- [dashpay/dash-evo-tool#989 ci: renew migration fixture archives through pull requests](https://github.com/dashpay/dash-evo-tool/pull/989) — ⚠ merge conflict · 📝 draft · 🐢 targets feat/migration-test-matrix · areas: dash-evo-tool
- [dashpay/dash-evo-tool#994 fix(wallet): make incompatible-data guidance and upgrade backups safe](https://github.com/dashpay/dash-evo-tool/pull/994) — via @Claudius-Maginificent · ✋ changes requested · areas: dash-evo-tool · Policy: waiting-bots
  - Blocker: coderabbitai has not reported for the current head
  - Blocker: thepastaclaw has not reported for the current head
- [dashpay/dash-evo-tool#996 fix(masternodes): keep the Add-voting-key prompt through unrelated task results](https://github.com/dashpay/dash-evo-tool/pull/996) — via @Claudius-Maginificent · ✋ changes requested · areas: dash-evo-tool · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head
- [dashpay/platform#4764 fix(platform-wallet): select available signing keys across identity operations](https://github.com/dashpay/platform/pull/4764) — ✋ changes requested · 🐢 targets v4.3-dev · areas: rs-platform-wallet, rs-platform-wallet-ffi
- [dashpay/platform#4777 fix(platform-wallet)!: preserve Core wallet snapshots on restart](https://github.com/dashpay/platform/pull/4777) — ✋ changes requested · 🐢 targets v4.3-dev · areas: rs-platform-wallet, wallet-storage, fallback
- [dashpay/tenderdash#1478 fix(rpc): bound asynchronous transaction broadcasts](https://github.com/dashpay/tenderdash/pull/1478) — 🔴 CI failing · areas: tenderdash · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed f946fa830ae8e3af32cbdf1b3c3ddef906065e2a after bot completion
- [dashpay/tenderdash#1479 fix(consensus): preserve catch-up allowance for rejected votes](https://github.com/dashpay/tenderdash/pull/1479) — areas: tenderdash · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 1207045fa4a1907c959ba6a28039afb67522916a after bot completion

<a id="lklimek-needs-action"></a>
#### Needs action (6)
- [dashpay/dash-evo-tool#851 fix(ui): center text inside buttons with min_size across the app](https://github.com/dashpay/dash-evo-tool/pull/851) — 2 unresolved (2 bot) · 146 days stale · ⚠ merge conflict · areas: dash-evo-tool · Policy: waiting-bots
  - Top thread: "🟡 Suggestion: Add a kittest harness for the new add_sized button-sizing contract**" — 146 days old
  - Blocker: coderabbitai has not reported for the current head
  - Blocker: thepastaclaw has not reported for the current head
  - Blocker: Bot review threads remain unresolved
- [dashpay/dash-evo-tool#983 test(migration): add cross-version migration test matrix](https://github.com/dashpay/dash-evo-tool/pull/983) — via @Claudius-Maginificent · 1 unresolved (1 CodeRabbit) · 7 days stale · ✋ changes requested · areas: dash-evo-tool · Policy: waiting-bots
  - Top thread: "_🩺 Stability & Availability_ \| _🟠 Major_ \| _🏗️ Heavy lift_" — 7 days old
  - Blocker: coderabbitai has not reported for the current head
  - Blocker: thepastaclaw has not reported for the current head
  - Blocker: Bot review threads remain unresolved
- [dashpay/dash-evo-tool#976 fix(security): protect identity keys before import storage](https://github.com/dashpay/dash-evo-tool/pull/976) — 1 unresolved (1 CodeRabbit) · 0 days stale · ✋ changes requested · areas: dash-evo-tool · Policy: waiting-bots
  - Top thread: "_🗄️ Data Integrity & Integration_ \| _🟡 Minor_ \| _⚡ Quick win_" — 0 days old
  - Blocker: thepastaclaw has not reported for the current head
  - Blocker: Bot review threads remain unresolved
- [dashpay/dash-evo-tool#987 fix(wallet): single source of truth for wallet and key names](https://github.com/dashpay/dash-evo-tool/pull/987) — via @Claudius-Maginificent · ✋ changes requested · areas: dash-evo-tool · Policy: waiting-bots
  - Blocker: coderabbitai has not reported for the current head
  - Blocker: thepastaclaw has not reported for the current head
- [dashpay/dash-evo-tool#994 fix(wallet): make incompatible-data guidance and upgrade backups safe](https://github.com/dashpay/dash-evo-tool/pull/994) — via @Claudius-Maginificent · ✋ changes requested · areas: dash-evo-tool · Policy: waiting-bots
  - Blocker: coderabbitai has not reported for the current head
  - Blocker: thepastaclaw has not reported for the current head
- [dashpay/dash-evo-tool#996 fix(masternodes): keep the Add-voting-key prompt through unrelated task results](https://github.com/dashpay/dash-evo-tool/pull/996) — via @Claudius-Maginificent · ✋ changes requested · areas: dash-evo-tool · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head

<a id="lklimek-unresolved-comments"></a>
#### Unresolved Comments (4)
- [dashpay/dash-evo-tool#851 fix(ui): center text inside buttons with min_size across the app](https://github.com/dashpay/dash-evo-tool/pull/851) — 2 unresolved (2 bot) · 146 days stale · ⚠ merge conflict · areas: dash-evo-tool · Policy: waiting-bots
  - Top thread: "🟡 Suggestion: Add a kittest harness for the new add_sized button-sizing contract**" — 146 days old
  - Blocker: coderabbitai has not reported for the current head
  - Blocker: thepastaclaw has not reported for the current head
  - Blocker: Bot review threads remain unresolved
- [dashpay/dash-evo-tool#983 test(migration): add cross-version migration test matrix](https://github.com/dashpay/dash-evo-tool/pull/983) — via @Claudius-Maginificent · 1 unresolved (1 CodeRabbit) · 7 days stale · ✋ changes requested · areas: dash-evo-tool · Policy: waiting-bots
  - Top thread: "_🩺 Stability & Availability_ \| _🟠 Major_ \| _🏗️ Heavy lift_" — 7 days old
  - Blocker: coderabbitai has not reported for the current head
  - Blocker: thepastaclaw has not reported for the current head
  - Blocker: Bot review threads remain unresolved
- [dashpay/platform#4740 fix(platform-wallet): scan DashPay contact accounts from request height](https://github.com/dashpay/platform/pull/4740) — 1 unresolved (1 bot) · 3 days stale · areas: rs-platform-wallet, swift-sdk · Policy: waiting-bots
  - Top thread: "🟡 Suggestion: Validate the contact-request height before using it as a scan checkpoint**" — 3 days old
  - Blocker: Bot review threads remain unresolved
  - Blocker: Proceeded without thepastaclaw: no review within the configured window
- [dashpay/dash-evo-tool#976 fix(security): protect identity keys before import storage](https://github.com/dashpay/dash-evo-tool/pull/976) — 1 unresolved (1 CodeRabbit) · 0 days stale · ✋ changes requested · areas: dash-evo-tool · Policy: waiting-bots
  - Top thread: "_🗄️ Data Integrity & Integration_ \| _🟡 Minor_ \| _⚡ Quick win_" — 0 days old
  - Blocker: thepastaclaw has not reported for the current head
  - Blocker: Bot review threads remain unresolved

<a id="lklimek-changes-requested"></a>
#### Changes Requested (3)
- [dashpay/dash-evo-tool#987 fix(wallet): single source of truth for wallet and key names](https://github.com/dashpay/dash-evo-tool/pull/987) — via @Claudius-Maginificent · ✋ changes requested · areas: dash-evo-tool · Policy: waiting-bots
  - Blocker: coderabbitai has not reported for the current head
  - Blocker: thepastaclaw has not reported for the current head
- [dashpay/dash-evo-tool#994 fix(wallet): make incompatible-data guidance and upgrade backups safe](https://github.com/dashpay/dash-evo-tool/pull/994) — via @Claudius-Maginificent · ✋ changes requested · areas: dash-evo-tool · Policy: waiting-bots
  - Blocker: coderabbitai has not reported for the current head
  - Blocker: thepastaclaw has not reported for the current head
- [dashpay/dash-evo-tool#996 fix(masternodes): keep the Add-voting-key prompt through unrelated task results](https://github.com/dashpay/dash-evo-tool/pull/996) — via @Claudius-Maginificent · ✋ changes requested · areas: dash-evo-tool · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head

<a id="lklimek-ci-failing"></a>
#### CI Failing (1)
- [dashpay/tenderdash#1478 fix(rpc): bound asynchronous transaction broadcasts](https://github.com/dashpay/tenderdash/pull/1478) — 🔴 CI failing · areas: tenderdash · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed f946fa830ae8e3af32cbdf1b3c3ddef906065e2a after bot completion

<a id="lklimek-draft"></a>
#### Draft (1)
- [dashpay/dash-evo-tool#980 feat(identity): add wallet-derived keys to existing identities](https://github.com/dashpay/dash-evo-tool/pull/980) — 📝 draft · areas: dash-evo-tool · Policy: draft
  - Blocker: Draft PR does not occupy a review slot

<a id="lklimek-stale"></a>
#### Stale (4)
- [dashpay/platform#3549 test(platform-wallet): e2e framework + full test suite — triage pins, Found-*/PA-* guards, fail-closed persist, Stage-2 merge](https://github.com/dashpay/platform/pull/3549) — via @Claudius-Maginificent · 9 unresolved (9 bot) · 139 days stale · ⚠ merge conflict · 📝 draft · 🐢 targets v4.3-dev · areas: rs-drive, rs-drive-abci, rs-platform-wallet, rs-platform-wallet-ffi, rust-sdk, fallback
  - Top thread: "🟡 Suggestion: `from_seed_for_identity` is misleadingly named, half-functional, and unused**" — 139 days old
- [dashpay/dash-evo-tool#989 ci: renew migration fixture archives through pull requests](https://github.com/dashpay/dash-evo-tool/pull/989) — ⚠ merge conflict · 📝 draft · 🐢 targets feat/migration-test-matrix · areas: dash-evo-tool
- [dashpay/platform#4764 fix(platform-wallet): select available signing keys across identity operations](https://github.com/dashpay/platform/pull/4764) — ✋ changes requested · 🐢 targets v4.3-dev · areas: rs-platform-wallet, rs-platform-wallet-ffi
- [dashpay/platform#4777 fix(platform-wallet)!: preserve Core wallet snapshots on restart](https://github.com/dashpay/platform/pull/4777) — ✋ changes requested · 🐢 targets v4.3-dev · areas: rs-platform-wallet, wallet-storage, fallback

<a id="lklimek-clean"></a>
#### Clean (2)
- [dashpay/dash-evo-tool#901 feat(dpns): unify safe masternode voting operations](https://github.com/dashpay/dash-evo-tool/pull/901) — areas: dash-evo-tool · Policy: waiting-bots
  - Blocker: coderabbitai has not reported for the current head
  - Blocker: thepastaclaw has not reported for the current head
- [dashpay/tenderdash#1479 fix(consensus): preserve catch-up allowance for rejected votes](https://github.com/dashpay/tenderdash/pull/1479) — areas: tenderdash · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 1207045fa4a1907c959ba6a28039afb67522916a after bot completion

<a id="lklimek-ready-for-review"></a>
#### Ready for Review (4)
- [dashpay/dash-evo-tool#772 docs: add weekly smoke test cases and prerequisites](https://github.com/dashpay/dash-evo-tool/pull/772) — by @thepastaclaw · areas: dash-evo-tool · Policy: waiting-bots
  - Blocker: coderabbitai has not reported for the current head
  - Blocker: thepastaclaw has not reported for the current head
  - Blocker: Bot review threads remain unresolved
- [dashpay/dash-evo-tool#853 fix: show parsed platform-address transitions](https://github.com/dashpay/dash-evo-tool/pull/853) — by @thepastaclaw · areas: dash-evo-tool · Policy: waiting-bots
  - Blocker: coderabbitai has not reported for the current head
  - Blocker: Bot review threads remain unresolved
- [dashpay/dash-evo-tool#859 perf: detect system theme off UI thread](https://github.com/dashpay/dash-evo-tool/pull/859) — by @thepastaclaw · areas: dash-evo-tool · Policy: waiting-bots
  - Blocker: coderabbitai has not reported for the current head
  - Blocker: thepastaclaw has not reported for the current head
  - Blocker: Bot review threads remain unresolved
- [dashpay/tenderdash#1484 ci: re-pin PR Hygiene so it can verify access granted by the organisation](https://github.com/dashpay/tenderdash/pull/1484) — by @shumkov · areas: tenderdash · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 28f85449d98188642fed3d6923ec2c2b6f6b6fc2 after bot completion

<a id="quantumexplorer"></a>
### @QuantumExplorer
<a id="quantumexplorer-open"></a>
#### Open (13)
- [dashpay/platform#4272 feat(dpp)!: dashpay payment detection keys and stealth derivation](https://github.com/dashpay/platform/pull/4272) — 9 unresolved (9 bot) · 46 days stale · ⚠ merge conflict · 📝 draft · 🐢 targets v4.3-dev · areas: rs-drive, rs-drive-abci, dpp, rs-platform-wallet, swift-sdk, system-contracts, kotlin-sdk, fallback
  - Top thread: "🔴 Blocking: Require compressed 33-byte encodings for payment keys**" — 46 days old
- [dashpay/rust-dashcore#622 fix: apply BIP-69 sorting to asset lock credit outputs](https://github.com/dashpay/rust-dashcore/pull/622) — 1 unresolved (1 CodeRabbit) · 169 days stale · ⚠ merge conflict · ✋ changes requested · 📝 draft · areas: key-wallet · ⚠ ownership unresolved: key-wallet · Policy: draft
  - Top thread: "_⚠️ Potential issue_ \| _🟠 Major_" — 169 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#860 feat(dash-spv): expose per-peer connection stats (ping RTT, bytes received)](https://github.com/dashpay/rust-dashcore/pull/860) — 3 unresolved (3 CodeRabbit) · 70 days stale · ✋ changes requested · areas: dash-spv, fallback · ⚠ ownership unresolved: dash-spv · Policy: configuration-error
  - Top thread: "_📐 Maintainability & Code Quality_ \| _🟡 Minor_ \| _⚡ Quick win_" — 70 days old
  - Blocker: Unresolved identities in dash-spv
- [dashpay/platform#3417 feat(swift-sdk): use SPV-synced quorums for Platform proof verification](https://github.com/dashpay/platform/pull/3417) — 3 unresolved (3 bot) · 10 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · 🐢 targets v4.3-dev · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, rust-sdk, rust-sdk-ffi, fallback
  - Top thread: "🟡 Suggestion: Exercise the production runtime bridge across Tokio runtime flavors**" — 10 days old
- [dashpay/rust-dashcore#136 feat: update integration test script for Dash compatibility](https://github.com/dashpay/rust-dashcore/pull/136) — 1 unresolved (1 CodeRabbit) · 379 days stale · 🔴 CI failing · 🐢 targets v0.40-dev, untouched 379 days · areas: fallback
  - Top thread: "_🛠️ Refactor suggestion_" — 379 days old
- [dashpay/platform#4800 fix(sdk): persist the contract bounds kind on Android and iOS](https://github.com/dashpay/platform/pull/4800) — 2 unresolved (2 bot) · 0 days stale · areas: swift-sdk, kotlin-sdk · Policy: waiting-bots
  - Top thread: "🟡 Suggestion: contractBounds setter comment omits the kind column it now resets**" — 0 days old
  - Blocker: Bot review threads remain unresolved
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/grovedb#977 fix(storage-flags): accept only canonical multi-epoch flag bytes](https://github.com/dashpay/grovedb/pull/977) — 1 unresolved (1 bot) · 1 days stale · areas: grovedb · Policy: waiting-bots
  - Top thread: "🟡 Suggestion: Mark the new exhaustive error variant as a breaking public API change**" — 1 days old
  - Blocker: coderabbitai has not reported for the current head
  - Blocker: Bot review threads remain unresolved
- [dashpay/platform#4519 feat(platform-wallet): rotate a masternode's keys into the wallet — ProUpRegTx orchestration, FFI, Swift](https://github.com/dashpay/platform/pull/4519) — 🐢 targets v4.3-dev · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk
- [dashpay/platform#4730 feat(platform)!: delta-based data contract update transition for protocol version 15](https://github.com/dashpay/platform/pull/4730) — 🔴 CI failing · 🐢 targets v4.3-dev · areas: rs-drive, rs-drive-abci, dpp, rs-platform-wallet, fallback
- [dashpay/platform#4751 ci: drop stale coverage objects before the Rust workspace test step](https://github.com/dashpay/platform/pull/4751) — 🐢 targets v4.3-dev · areas: fallback
- [dashpay/platform#4760 feat(platform)!: token shielded pools](https://github.com/dashpay/platform/pull/4760) — ✋ changes requested · 🐢 targets v4.3-dev · areas: rs-drive, rs-drive-abci, dpp, rust-sdk, fallback
- [dashpay/platform#4776 feat(platform-wallet)!: let contract updates clear the description through the FFI](https://github.com/dashpay/platform/pull/4776) — 🐢 targets feat/delta-contract-update-pv15 · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk, fallback
- [dashpay/platform#4811 feat(sdk)!: key limits on every client: wasm-dpp2, platform-wallet, FFI, Kotlin and Swift](https://github.com/dashpay/platform/pull/4811) — 🔴 CI failing · areas: rs-drive, rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, rust-sdk-ffi, js-wasm-sdk, kotlin-sdk, fallback · Policy: waiting-bots
  - Blocker: coderabbitai has not reported for the current head
  - Blocker: thepastaclaw has not reported for the current head

<a id="quantumexplorer-needs-action"></a>
#### Needs action (3)
- [dashpay/rust-dashcore#860 feat(dash-spv): expose per-peer connection stats (ping RTT, bytes received)](https://github.com/dashpay/rust-dashcore/pull/860) — 3 unresolved (3 CodeRabbit) · 70 days stale · ✋ changes requested · areas: dash-spv, fallback · ⚠ ownership unresolved: dash-spv · Policy: configuration-error
  - Top thread: "_📐 Maintainability & Code Quality_ \| _🟡 Minor_ \| _⚡ Quick win_" — 70 days old
  - Blocker: Unresolved identities in dash-spv
- [dashpay/platform#4800 fix(sdk): persist the contract bounds kind on Android and iOS](https://github.com/dashpay/platform/pull/4800) — 2 unresolved (2 bot) · 0 days stale · areas: swift-sdk, kotlin-sdk · Policy: waiting-bots
  - Top thread: "🟡 Suggestion: contractBounds setter comment omits the kind column it now resets**" — 0 days old
  - Blocker: Bot review threads remain unresolved
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/grovedb#977 fix(storage-flags): accept only canonical multi-epoch flag bytes](https://github.com/dashpay/grovedb/pull/977) — 1 unresolved (1 bot) · 1 days stale · areas: grovedb · Policy: waiting-bots
  - Top thread: "🟡 Suggestion: Mark the new exhaustive error variant as a breaking public API change**" — 1 days old
  - Blocker: coderabbitai has not reported for the current head
  - Blocker: Bot review threads remain unresolved

<a id="quantumexplorer-unresolved-comments"></a>
#### Unresolved Comments (3)
- [dashpay/rust-dashcore#860 feat(dash-spv): expose per-peer connection stats (ping RTT, bytes received)](https://github.com/dashpay/rust-dashcore/pull/860) — 3 unresolved (3 CodeRabbit) · 70 days stale · ✋ changes requested · areas: dash-spv, fallback · ⚠ ownership unresolved: dash-spv · Policy: configuration-error
  - Top thread: "_📐 Maintainability & Code Quality_ \| _🟡 Minor_ \| _⚡ Quick win_" — 70 days old
  - Blocker: Unresolved identities in dash-spv
- [dashpay/platform#4800 fix(sdk): persist the contract bounds kind on Android and iOS](https://github.com/dashpay/platform/pull/4800) — 2 unresolved (2 bot) · 0 days stale · areas: swift-sdk, kotlin-sdk · Policy: waiting-bots
  - Top thread: "🟡 Suggestion: contractBounds setter comment omits the kind column it now resets**" — 0 days old
  - Blocker: Bot review threads remain unresolved
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/grovedb#977 fix(storage-flags): accept only canonical multi-epoch flag bytes](https://github.com/dashpay/grovedb/pull/977) — 1 unresolved (1 bot) · 1 days stale · areas: grovedb · Policy: waiting-bots
  - Top thread: "🟡 Suggestion: Mark the new exhaustive error variant as a breaking public API change**" — 1 days old
  - Blocker: coderabbitai has not reported for the current head
  - Blocker: Bot review threads remain unresolved

<a id="quantumexplorer-ci-failing"></a>
#### CI Failing (1)
- [dashpay/platform#4811 feat(sdk)!: key limits on every client: wasm-dpp2, platform-wallet, FFI, Kotlin and Swift](https://github.com/dashpay/platform/pull/4811) — 🔴 CI failing · areas: rs-drive, rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, rust-sdk-ffi, js-wasm-sdk, kotlin-sdk, fallback · Policy: waiting-bots
  - Blocker: coderabbitai has not reported for the current head
  - Blocker: thepastaclaw has not reported for the current head

<a id="quantumexplorer-draft"></a>
#### Draft (1)
- [dashpay/rust-dashcore#622 fix: apply BIP-69 sorting to asset lock credit outputs](https://github.com/dashpay/rust-dashcore/pull/622) — 1 unresolved (1 CodeRabbit) · 169 days stale · ⚠ merge conflict · ✋ changes requested · 📝 draft · areas: key-wallet · ⚠ ownership unresolved: key-wallet · Policy: draft
  - Top thread: "_⚠️ Potential issue_ \| _🟠 Major_" — 169 days old
  - Blocker: Draft PR does not occupy a review slot

<a id="quantumexplorer-stale"></a>
#### Stale (8)
- [dashpay/platform#4272 feat(dpp)!: dashpay payment detection keys and stealth derivation](https://github.com/dashpay/platform/pull/4272) — 9 unresolved (9 bot) · 46 days stale · ⚠ merge conflict · 📝 draft · 🐢 targets v4.3-dev · areas: rs-drive, rs-drive-abci, dpp, rs-platform-wallet, swift-sdk, system-contracts, kotlin-sdk, fallback
  - Top thread: "🔴 Blocking: Require compressed 33-byte encodings for payment keys**" — 46 days old
- [dashpay/platform#3417 feat(swift-sdk): use SPV-synced quorums for Platform proof verification](https://github.com/dashpay/platform/pull/3417) — 3 unresolved (3 bot) · 10 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · 🐢 targets v4.3-dev · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, rust-sdk, rust-sdk-ffi, fallback
  - Top thread: "🟡 Suggestion: Exercise the production runtime bridge across Tokio runtime flavors**" — 10 days old
- [dashpay/rust-dashcore#136 feat: update integration test script for Dash compatibility](https://github.com/dashpay/rust-dashcore/pull/136) — 1 unresolved (1 CodeRabbit) · 379 days stale · 🔴 CI failing · 🐢 targets v0.40-dev, untouched 379 days · areas: fallback
  - Top thread: "_🛠️ Refactor suggestion_" — 379 days old
- [dashpay/platform#4519 feat(platform-wallet): rotate a masternode's keys into the wallet — ProUpRegTx orchestration, FFI, Swift](https://github.com/dashpay/platform/pull/4519) — 🐢 targets v4.3-dev · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk
- [dashpay/platform#4730 feat(platform)!: delta-based data contract update transition for protocol version 15](https://github.com/dashpay/platform/pull/4730) — 🔴 CI failing · 🐢 targets v4.3-dev · areas: rs-drive, rs-drive-abci, dpp, rs-platform-wallet, fallback
- [dashpay/platform#4751 ci: drop stale coverage objects before the Rust workspace test step](https://github.com/dashpay/platform/pull/4751) — 🐢 targets v4.3-dev · areas: fallback
- [dashpay/platform#4760 feat(platform)!: token shielded pools](https://github.com/dashpay/platform/pull/4760) — ✋ changes requested · 🐢 targets v4.3-dev · areas: rs-drive, rs-drive-abci, dpp, rust-sdk, fallback
- [dashpay/platform#4776 feat(platform-wallet)!: let contract updates clear the description through the FFI](https://github.com/dashpay/platform/pull/4776) — 🐢 targets feat/delta-contract-update-pv15 · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk, fallback

<a id="quantumexplorer-ready-for-review"></a>
#### Ready for Review (5)
- [dashpay/rust-dashcore#993 fix(dash-spv): wire up masternode persistence and shrink it 67x](https://github.com/dashpay/rust-dashcore/pull/993) — by @ZocoLini · areas: dash-spv, fallback · ⚠ ownership unresolved: dash-spv · Policy: configuration-error
  - Blocker: Unresolved identities in dash-spv
- [dashpay/rust-dashcore#1002 fix(dash-spv): make backward coverage durable by rewinding synced_height instead of sweeping in memory](https://github.com/dashpay/rust-dashcore/pull/1002) — by @romchornyi · areas: dash-spv, key-wallet-manager, fallback · ⚠ ownership unresolved: dash-spv, key-wallet-manager · Policy: configuration-error
  - Blocker: Unresolved identities in dash-spv
- [dashpay/rust-dashcore#1014 fix(key-wallet): prune observed spends during the initial sync](https://github.com/dashpay/rust-dashcore/pull/1014) — by @ZocoLini · areas: dash-spv, key-wallet, key-wallet-manager · ⚠ ownership unresolved: dash-spv, key-wallet, key-wallet-manager · Policy: configuration-error
  - Blocker: Unresolved identities in dash-spv
- [dashpay/rust-dashcore#1015 fix(key-wallet): re-apply a spend whose coin was funded after it](https://github.com/dashpay/rust-dashcore/pull/1015) — by @ZocoLini · areas: dash-spv, key-wallet, key-wallet-manager · ⚠ ownership unresolved: dash-spv, key-wallet, key-wallet-manager · Policy: configuration-error
  - Blocker: Unresolved identities in dash-spv
- [dashpay/rust-dashcore#1035 fix(key-wallet): extend DIP-15 contact pools as their payments arrive](https://github.com/dashpay/rust-dashcore/pull/1035) — by @ZocoLini · areas: key-wallet · ⚠ ownership unresolved: key-wallet · Policy: waiting-slot
  - Blocker: Waiting for one of five author review slots

<a id="llbartekll"></a>
### @llbartekll
<a id="llbartekll-open"></a>
#### Open (4)
- [dashpay/platform#3560 test(swift-sdk): add testnet identity-discovery UI test](https://github.com/dashpay/platform/pull/3560) — 1 unresolved (1 bot) · 109 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · areas: swift-sdk, fallback · Policy: waiting-bots
  - Top thread: "🔴 Blocking: Carried-forward prior finding (STILL VALID): Docker-setup toggle leaves the cached regtest wallet manager bo…" — 109 days old
  - Blocker: Bot changes request remains outstanding
  - Blocker: Bot review threads remain unresolved
  - Blocker: Proceeded without coderabbitai: no review within the configured window
  - … 1 more blocker(s)
- [dashpay/platform#4799 fix(sdk): fetch and persist managed identity credit balances](https://github.com/dashpay/platform/pull/4799) — 4 unresolved (1 human, 3 bot) · 1 days stale · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk · Policy: waiting-bots
  - Top thread: "Major (not a blocker).** The guard protects against a slower *overlapping refresh*, but not against a newer **locally ap…" — 1 days old
  - Blocker: coderabbitai has not reported for the current head
  - Blocker: thepastaclaw has not reported for the current head
  - Blocker: Bot review threads remain unresolved
- [dashpay/rust-dashcore#819 feat(key-wallet): CoinJoin sweep + gap-limit recovery; TransactionBuilder drain mode](https://github.com/dashpay/rust-dashcore/pull/819) — ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · areas: key-wallet · ⚠ ownership unresolved: key-wallet · Policy: configuration-error
  - Blocker: Unresolved identities in key-wallet
- [dashpay/rust-dashcore#910 fix(dash-spv): re-anchor incompatible shared storage](https://github.com/dashpay/rust-dashcore/pull/910) — 📝 draft · areas: dash-spv · ⚠ ownership unresolved: dash-spv · Policy: draft
  - Blocker: Draft PR does not occupy a review slot

<a id="llbartekll-needs-action"></a>
#### Needs action (3)
- [dashpay/platform#3560 test(swift-sdk): add testnet identity-discovery UI test](https://github.com/dashpay/platform/pull/3560) — 1 unresolved (1 bot) · 109 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · areas: swift-sdk, fallback · Policy: waiting-bots
  - Top thread: "🔴 Blocking: Carried-forward prior finding (STILL VALID): Docker-setup toggle leaves the cached regtest wallet manager bo…" — 109 days old
  - Blocker: Bot changes request remains outstanding
  - Blocker: Bot review threads remain unresolved
  - Blocker: Proceeded without coderabbitai: no review within the configured window
  - … 1 more blocker(s)
- [dashpay/platform#4799 fix(sdk): fetch and persist managed identity credit balances](https://github.com/dashpay/platform/pull/4799) — 4 unresolved (1 human, 3 bot) · 1 days stale · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk · Policy: waiting-bots
  - Top thread: "Major (not a blocker).** The guard protects against a slower *overlapping refresh*, but not against a newer **locally ap…" — 1 days old
  - Blocker: coderabbitai has not reported for the current head
  - Blocker: thepastaclaw has not reported for the current head
  - Blocker: Bot review threads remain unresolved
- [dashpay/rust-dashcore#819 feat(key-wallet): CoinJoin sweep + gap-limit recovery; TransactionBuilder drain mode](https://github.com/dashpay/rust-dashcore/pull/819) — ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · areas: key-wallet · ⚠ ownership unresolved: key-wallet · Policy: configuration-error
  - Blocker: Unresolved identities in key-wallet

<a id="llbartekll-unresolved-comments"></a>
#### Unresolved Comments (2)
- [dashpay/platform#3560 test(swift-sdk): add testnet identity-discovery UI test](https://github.com/dashpay/platform/pull/3560) — 1 unresolved (1 bot) · 109 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · areas: swift-sdk, fallback · Policy: waiting-bots
  - Top thread: "🔴 Blocking: Carried-forward prior finding (STILL VALID): Docker-setup toggle leaves the cached regtest wallet manager bo…" — 109 days old
  - Blocker: Bot changes request remains outstanding
  - Blocker: Bot review threads remain unresolved
  - Blocker: Proceeded without coderabbitai: no review within the configured window
  - … 1 more blocker(s)
- [dashpay/platform#4799 fix(sdk): fetch and persist managed identity credit balances](https://github.com/dashpay/platform/pull/4799) — 4 unresolved (1 human, 3 bot) · 1 days stale · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk · Policy: waiting-bots
  - Top thread: "Major (not a blocker).** The guard protects against a slower *overlapping refresh*, but not against a newer **locally ap…" — 1 days old
  - Blocker: coderabbitai has not reported for the current head
  - Blocker: thepastaclaw has not reported for the current head
  - Blocker: Bot review threads remain unresolved

<a id="llbartekll-changes-requested"></a>
#### Changes Requested (1)
- [dashpay/rust-dashcore#819 feat(key-wallet): CoinJoin sweep + gap-limit recovery; TransactionBuilder drain mode](https://github.com/dashpay/rust-dashcore/pull/819) — ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · areas: key-wallet · ⚠ ownership unresolved: key-wallet · Policy: configuration-error
  - Blocker: Unresolved identities in key-wallet

<a id="llbartekll-draft"></a>
#### Draft (1)
- [dashpay/rust-dashcore#910 fix(dash-spv): re-anchor incompatible shared storage](https://github.com/dashpay/rust-dashcore/pull/910) — 📝 draft · areas: dash-spv · ⚠ ownership unresolved: dash-spv · Policy: draft
  - Blocker: Draft PR does not occupy a review slot

<a id="xdustinface"></a>
### @xdustinface
<a id="xdustinface-open"></a>
#### Open (8)
- [dashpay/rust-dashcore#856 feat(dash-spv): force resync to re-anchor a running client at a lower checkpoint](https://github.com/dashpay/rust-dashcore/pull/856) — 1 unresolved (1 human) · 59 days stale · ⚠ merge conflict · 🔴 CI failing · areas: dash-spv · ⚠ ownership unresolved: dash-spv · Policy: configuration-error
  - Top thread: "why do we have a periodic check here??" — 59 days old
  - Blocker: Unresolved identities in dash-spv
- [dashpay/rust-dashcore#400 chore: remove BIP141 (SegWit) and BIP341 (Taproot) support](https://github.com/dashpay/rust-dashcore/pull/400) — ⚠ merge conflict · 🔴 CI failing · 📝 draft · 🐢 untouched 121 days · areas: dash-spv, key-wallet, fallback · ⚠ ownership unresolved: dash-spv, key-wallet · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#541 ci: integrate Codecov test analytics via `cargo-nextest`](https://github.com/dashpay/rust-dashcore/pull/541) — ⚠ merge conflict · 📝 draft · areas: fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#720 fix(dash-spv): split peer `TcpStream` and add per-peer writer task](https://github.com/dashpay/rust-dashcore/pull/720) — ⚠ merge conflict · 🔴 CI failing · 📝 draft · 🐢 untouched 121 days · areas: dash-spv, fallback · ⚠ ownership unresolved: dash-spv · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#776 chore: remove FFI layer](https://github.com/dashpay/rust-dashcore/pull/776) — ⚠ merge conflict · 📝 draft · 🐢 untouched 121 days · areas: dash-spv, key-wallet, fallback · ⚠ ownership unresolved: dash-spv, key-wallet · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#799 feat: validate masternode list merkle root](https://github.com/dashpay/rust-dashcore/pull/799) — ⚠ merge conflict · 📝 draft · areas: fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#803 feat(dash-spv): block locator + staged fork detection](https://github.com/dashpay/rust-dashcore/pull/803) — ⚠ merge conflict · 📝 draft · areas: dash-spv, fallback · ⚠ ownership unresolved: dash-spv · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#849 feat(dash-spv): add `--birth-height` CLI flag](https://github.com/dashpay/rust-dashcore/pull/849) — areas: dash-spv · ⚠ ownership unresolved: dash-spv · Policy: configuration-error
  - Blocker: Unresolved identities in dash-spv

<a id="xdustinface-needs-action"></a>
#### Needs action (1)
- [dashpay/rust-dashcore#856 feat(dash-spv): force resync to re-anchor a running client at a lower checkpoint](https://github.com/dashpay/rust-dashcore/pull/856) — 1 unresolved (1 human) · 59 days stale · ⚠ merge conflict · 🔴 CI failing · areas: dash-spv · ⚠ ownership unresolved: dash-spv · Policy: configuration-error
  - Top thread: "why do we have a periodic check here??" — 59 days old
  - Blocker: Unresolved identities in dash-spv

<a id="xdustinface-unresolved-comments"></a>
#### Unresolved Comments (1)
- [dashpay/rust-dashcore#856 feat(dash-spv): force resync to re-anchor a running client at a lower checkpoint](https://github.com/dashpay/rust-dashcore/pull/856) — 1 unresolved (1 human) · 59 days stale · ⚠ merge conflict · 🔴 CI failing · areas: dash-spv · ⚠ ownership unresolved: dash-spv · Policy: configuration-error
  - Top thread: "why do we have a periodic check here??" — 59 days old
  - Blocker: Unresolved identities in dash-spv

<a id="xdustinface-draft"></a>
#### Draft (3)
- [dashpay/rust-dashcore#541 ci: integrate Codecov test analytics via `cargo-nextest`](https://github.com/dashpay/rust-dashcore/pull/541) — ⚠ merge conflict · 📝 draft · areas: fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#799 feat: validate masternode list merkle root](https://github.com/dashpay/rust-dashcore/pull/799) — ⚠ merge conflict · 📝 draft · areas: fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#803 feat(dash-spv): block locator + staged fork detection](https://github.com/dashpay/rust-dashcore/pull/803) — ⚠ merge conflict · 📝 draft · areas: dash-spv, fallback · ⚠ ownership unresolved: dash-spv · Policy: draft
  - Blocker: Draft PR does not occupy a review slot

<a id="xdustinface-stale"></a>
#### Stale (3)
- [dashpay/rust-dashcore#400 chore: remove BIP141 (SegWit) and BIP341 (Taproot) support](https://github.com/dashpay/rust-dashcore/pull/400) — ⚠ merge conflict · 🔴 CI failing · 📝 draft · 🐢 untouched 121 days · areas: dash-spv, key-wallet, fallback · ⚠ ownership unresolved: dash-spv, key-wallet · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#720 fix(dash-spv): split peer `TcpStream` and add per-peer writer task](https://github.com/dashpay/rust-dashcore/pull/720) — ⚠ merge conflict · 🔴 CI failing · 📝 draft · 🐢 untouched 121 days · areas: dash-spv, fallback · ⚠ ownership unresolved: dash-spv · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#776 chore: remove FFI layer](https://github.com/dashpay/rust-dashcore/pull/776) — ⚠ merge conflict · 📝 draft · 🐢 untouched 121 days · areas: dash-spv, key-wallet, fallback · ⚠ ownership unresolved: dash-spv, key-wallet · Policy: draft
  - Blocker: Draft PR does not occupy a review slot

<a id="xdustinface-clean"></a>
#### Clean (1)
- [dashpay/rust-dashcore#849 feat(dash-spv): add `--birth-height` CLI flag](https://github.com/dashpay/rust-dashcore/pull/849) — areas: dash-spv · ⚠ ownership unresolved: dash-spv · Policy: configuration-error
  - Blocker: Unresolved identities in dash-spv

<a id="xdustinface-ready-for-review"></a>
#### Ready for Review (5)
- [dashpay/rust-dashcore#993 fix(dash-spv): wire up masternode persistence and shrink it 67x](https://github.com/dashpay/rust-dashcore/pull/993) — by @ZocoLini · areas: dash-spv, fallback · ⚠ ownership unresolved: dash-spv · Policy: configuration-error
  - Blocker: Unresolved identities in dash-spv
- [dashpay/rust-dashcore#1002 fix(dash-spv): make backward coverage durable by rewinding synced_height instead of sweeping in memory](https://github.com/dashpay/rust-dashcore/pull/1002) — by @romchornyi · areas: dash-spv, key-wallet-manager, fallback · ⚠ ownership unresolved: dash-spv, key-wallet-manager · Policy: configuration-error
  - Blocker: Unresolved identities in dash-spv
- [dashpay/rust-dashcore#1014 fix(key-wallet): prune observed spends during the initial sync](https://github.com/dashpay/rust-dashcore/pull/1014) — by @ZocoLini · areas: dash-spv, key-wallet, key-wallet-manager · ⚠ ownership unresolved: dash-spv, key-wallet, key-wallet-manager · Policy: configuration-error
  - Blocker: Unresolved identities in dash-spv
- [dashpay/rust-dashcore#1015 fix(key-wallet): re-apply a spend whose coin was funded after it](https://github.com/dashpay/rust-dashcore/pull/1015) — by @ZocoLini · areas: dash-spv, key-wallet, key-wallet-manager · ⚠ ownership unresolved: dash-spv, key-wallet, key-wallet-manager · Policy: configuration-error
  - Blocker: Unresolved identities in dash-spv
- [dashpay/rust-dashcore#1035 fix(key-wallet): extend DIP-15 contact pools as their payments arrive](https://github.com/dashpay/rust-dashcore/pull/1035) — by @ZocoLini · areas: key-wallet · ⚠ ownership unresolved: key-wallet · Policy: waiting-slot
  - Blocker: Waiting for one of five author review slots

<a id="shumkov"></a>
### @shumkov
<a id="shumkov-open"></a>
#### Open (11)
- [dashpay/rust-dashcore#634 feat(key-wallet-manager): single-map WalletManager with Arc<RwLock<T>> per wallet](https://github.com/dashpay/rust-dashcore/pull/634) — 20 unresolved (20 CodeRabbit) · 163 days stale · ⚠ merge conflict · ✋ changes requested · 🐢 untouched 121 days · areas: dash-spv, key-wallet, key-wallet-manager, fallback · ⚠ ownership unresolved: dash-spv, key-wallet, key-wallet-manager · Policy: configuration-error
  - Top thread: "_⚠️ Potential issue_ \| _🟡 Minor_" — 163 days old
  - Blocker: Unresolved identities in dash-spv
- [dashpay/dash-evo-tool#890 test(masternode-upgrade): standalone masternode-identity upgrade harness + spec](https://github.com/dashpay/dash-evo-tool/pull/890) — 24 unresolved (20 human, 4 bot) · 63 days stale · ✋ changes requested · areas: dash-evo-tool · Policy: waiting-bots
  - Top thread: "🟡 **Backup recipe writes a secret-bearing `~/.dashmate` tarball into `$PWD`, and the root `.gitignore` does not cover it…" — 63 days old
  - Blocker: coderabbitai has not reported for the current head
  - Blocker: thepastaclaw has not reported for the current head
  - Blocker: Bot changes request remains outstanding
  - … 1 more blocker(s)
- [dashpay/platform#4282 fix(dashmate)!: give Debian packages versions apt can order](https://github.com/dashpay/platform/pull/4282) — 6 unresolved (1 CodeRabbit, 5 bot) · 18 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · 🐢 targets v4.3-dev · areas: dashmate, js-wasm-sdk, system-contracts, fallback
  - Top thread: "_🗄️ Data Integrity & Integration_ \| _🟡 Minor_ \| _⚡ Quick win_" — 18 days old
- [dashpay/platform#3936 chore(drive-abci): update to nested address in SML](https://github.com/dashpay/platform/pull/3936) — 2 unresolved (2 bot) · 91 days stale · ⚠ merge conflict · 📝 draft · 🐢 targets v4.3-dev · areas: rs-drive-abci, rust-sdk, fallback
  - Top thread: "🟡 Suggestion: V0 reverse synthesizes `addresses` from a Core-22 port, masking subsequent legacy port-change diffs after …" — 91 days old
- [dashpay/platform#4283 fix(dashmate): keep the node up when an image pull fails, and stop reporting success when it did not](https://github.com/dashpay/platform/pull/4283) — 2 unresolved (2 bot) · 12 days stale · ⚠ merge conflict · ✋ changes requested · 🐢 targets v4.3-dev · areas: dashmate
  - Top thread: "🟡 Suggestion: providers.js sanitizer misses U+061C (Arabic letter mark) that sanitizeRemoteText covers**" — 12 days old
- [dashpay/platform#4243 fix(sdk)!: complete encrypted txMetadata parity](https://github.com/dashpay/platform/pull/4243) — ⚠ merge conflict · 📝 draft · 🐢 targets v4.3-dev · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk, fallback
- [dashpay/platform#4285 feat(platform-wallet)!: invitation links are AppsFlyer applinks only](https://github.com/dashpay/platform/pull/4285) — ⚠ merge conflict · 🔴 CI failing · 📝 draft · 🐢 targets v4.3-dev · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk, fallback
- [dashpay/platform#4652 feat(drive)!: preserve composite document history across protocol activation](https://github.com/dashpay/platform/pull/4652) — ✋ changes requested · 🐢 targets v4.3-dev · areas: rs-drive, rs-drive-abci, rust-sdk, js-wasm-sdk, fallback
- [dashpay/platform#4657 feat(platform)!: delete and erase lifecycle for keep-history documents](https://github.com/dashpay/platform/pull/4657) — ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · 🐢 targets v4.3-dev · areas: rs-drive, rs-drive-abci, dpp, rust-sdk, js-wasm-sdk, fallback
- [dashpay/platform#4660 feat(sdk): expose the document erase and history lifecycle on mobile and FFI](https://github.com/dashpay/platform/pull/4660) — ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · 🐢 targets v4.3-dev · areas: rs-drive, rs-drive-abci, dpp, rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, rust-sdk, rust-sdk-ffi, js-wasm-sdk, kotlin-sdk, fallback
- [dashpay/tenderdash#1484 ci: re-pin PR Hygiene so it can verify access granted by the organisation](https://github.com/dashpay/tenderdash/pull/1484) — areas: tenderdash · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 28f85449d98188642fed3d6923ec2c2b6f6b6fc2 after bot completion

<a id="shumkov-needs-action"></a>
#### Needs action (1)
- [dashpay/dash-evo-tool#890 test(masternode-upgrade): standalone masternode-identity upgrade harness + spec](https://github.com/dashpay/dash-evo-tool/pull/890) — 24 unresolved (20 human, 4 bot) · 63 days stale · ✋ changes requested · areas: dash-evo-tool · Policy: waiting-bots
  - Top thread: "🟡 **Backup recipe writes a secret-bearing `~/.dashmate` tarball into `$PWD`, and the root `.gitignore` does not cover it…" — 63 days old
  - Blocker: coderabbitai has not reported for the current head
  - Blocker: thepastaclaw has not reported for the current head
  - Blocker: Bot changes request remains outstanding
  - … 1 more blocker(s)

<a id="shumkov-unresolved-comments"></a>
#### Unresolved Comments (1)
- [dashpay/dash-evo-tool#890 test(masternode-upgrade): standalone masternode-identity upgrade harness + spec](https://github.com/dashpay/dash-evo-tool/pull/890) — 24 unresolved (20 human, 4 bot) · 63 days stale · ✋ changes requested · areas: dash-evo-tool · Policy: waiting-bots
  - Top thread: "🟡 **Backup recipe writes a secret-bearing `~/.dashmate` tarball into `$PWD`, and the root `.gitignore` does not cover it…" — 63 days old
  - Blocker: coderabbitai has not reported for the current head
  - Blocker: thepastaclaw has not reported for the current head
  - Blocker: Bot changes request remains outstanding
  - … 1 more blocker(s)

<a id="shumkov-stale"></a>
#### Stale (9)
- [dashpay/rust-dashcore#634 feat(key-wallet-manager): single-map WalletManager with Arc<RwLock<T>> per wallet](https://github.com/dashpay/rust-dashcore/pull/634) — 20 unresolved (20 CodeRabbit) · 163 days stale · ⚠ merge conflict · ✋ changes requested · 🐢 untouched 121 days · areas: dash-spv, key-wallet, key-wallet-manager, fallback · ⚠ ownership unresolved: dash-spv, key-wallet, key-wallet-manager · Policy: configuration-error
  - Top thread: "_⚠️ Potential issue_ \| _🟡 Minor_" — 163 days old
  - Blocker: Unresolved identities in dash-spv
- [dashpay/platform#4282 fix(dashmate)!: give Debian packages versions apt can order](https://github.com/dashpay/platform/pull/4282) — 6 unresolved (1 CodeRabbit, 5 bot) · 18 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · 🐢 targets v4.3-dev · areas: dashmate, js-wasm-sdk, system-contracts, fallback
  - Top thread: "_🗄️ Data Integrity & Integration_ \| _🟡 Minor_ \| _⚡ Quick win_" — 18 days old
- [dashpay/platform#3936 chore(drive-abci): update to nested address in SML](https://github.com/dashpay/platform/pull/3936) — 2 unresolved (2 bot) · 91 days stale · ⚠ merge conflict · 📝 draft · 🐢 targets v4.3-dev · areas: rs-drive-abci, rust-sdk, fallback
  - Top thread: "🟡 Suggestion: V0 reverse synthesizes `addresses` from a Core-22 port, masking subsequent legacy port-change diffs after …" — 91 days old
- [dashpay/platform#4283 fix(dashmate): keep the node up when an image pull fails, and stop reporting success when it did not](https://github.com/dashpay/platform/pull/4283) — 2 unresolved (2 bot) · 12 days stale · ⚠ merge conflict · ✋ changes requested · 🐢 targets v4.3-dev · areas: dashmate
  - Top thread: "🟡 Suggestion: providers.js sanitizer misses U+061C (Arabic letter mark) that sanitizeRemoteText covers**" — 12 days old
- [dashpay/platform#4243 fix(sdk)!: complete encrypted txMetadata parity](https://github.com/dashpay/platform/pull/4243) — ⚠ merge conflict · 📝 draft · 🐢 targets v4.3-dev · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk, fallback
- [dashpay/platform#4285 feat(platform-wallet)!: invitation links are AppsFlyer applinks only](https://github.com/dashpay/platform/pull/4285) — ⚠ merge conflict · 🔴 CI failing · 📝 draft · 🐢 targets v4.3-dev · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk, fallback
- [dashpay/platform#4652 feat(drive)!: preserve composite document history across protocol activation](https://github.com/dashpay/platform/pull/4652) — ✋ changes requested · 🐢 targets v4.3-dev · areas: rs-drive, rs-drive-abci, rust-sdk, js-wasm-sdk, fallback
- [dashpay/platform#4657 feat(platform)!: delete and erase lifecycle for keep-history documents](https://github.com/dashpay/platform/pull/4657) — ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · 🐢 targets v4.3-dev · areas: rs-drive, rs-drive-abci, dpp, rust-sdk, js-wasm-sdk, fallback
- [dashpay/platform#4660 feat(sdk): expose the document erase and history lifecycle on mobile and FFI](https://github.com/dashpay/platform/pull/4660) — ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · 🐢 targets v4.3-dev · areas: rs-drive, rs-drive-abci, dpp, rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, rust-sdk, rust-sdk-ffi, js-wasm-sdk, kotlin-sdk, fallback

<a id="shumkov-clean"></a>
#### Clean (1)
- [dashpay/tenderdash#1484 ci: re-pin PR Hygiene so it can verify access granted by the organisation](https://github.com/dashpay/tenderdash/pull/1484) — areas: tenderdash · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 28f85449d98188642fed3d6923ec2c2b6f6b6fc2 after bot completion

<a id="shumkov-ready-for-review"></a>
#### Ready for Review (1)
- [dashpay/tenderdash#1479 fix(consensus): preserve catch-up allowance for rejected votes](https://github.com/dashpay/tenderdash/pull/1479) — by @lklimek · areas: tenderdash · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 1207045fa4a1907c959ba6a28039afb67522916a after bot completion

<a id="romchornyi"></a>
### @romchornyi
<a id="romchornyi-open"></a>
#### Open (2)
- [dashpay/platform#4595 fix(swift-sdk): make a changeset round linear without giving up atomicity](https://github.com/dashpay/platform/pull/4595) — 2 unresolved (2 bot) · 11 days stale · ⚠ merge conflict · ✋ changes requested · areas: swift-sdk · Policy: waiting-bots
  - Top thread: "🟡 Suggestion: Add round-level regression coverage for registry bookkeeping**" — 11 days old
  - Blocker: Bot changes request remains outstanding
  - Blocker: Bot review threads remain unresolved
  - Blocker: Proceeded without thepastaclaw: no review within the configured window
- [dashpay/rust-dashcore#1002 fix(dash-spv): make backward coverage durable by rewinding synced_height instead of sweeping in memory](https://github.com/dashpay/rust-dashcore/pull/1002) — areas: dash-spv, key-wallet-manager, fallback · ⚠ ownership unresolved: dash-spv, key-wallet-manager · Policy: configuration-error
  - Blocker: Unresolved identities in dash-spv

<a id="romchornyi-needs-action"></a>
#### Needs action (1)
- [dashpay/platform#4595 fix(swift-sdk): make a changeset round linear without giving up atomicity](https://github.com/dashpay/platform/pull/4595) — 2 unresolved (2 bot) · 11 days stale · ⚠ merge conflict · ✋ changes requested · areas: swift-sdk · Policy: waiting-bots
  - Top thread: "🟡 Suggestion: Add round-level regression coverage for registry bookkeeping**" — 11 days old
  - Blocker: Bot changes request remains outstanding
  - Blocker: Bot review threads remain unresolved
  - Blocker: Proceeded without thepastaclaw: no review within the configured window

<a id="romchornyi-unresolved-comments"></a>
#### Unresolved Comments (1)
- [dashpay/platform#4595 fix(swift-sdk): make a changeset round linear without giving up atomicity](https://github.com/dashpay/platform/pull/4595) — 2 unresolved (2 bot) · 11 days stale · ⚠ merge conflict · ✋ changes requested · areas: swift-sdk · Policy: waiting-bots
  - Top thread: "🟡 Suggestion: Add round-level regression coverage for registry bookkeeping**" — 11 days old
  - Blocker: Bot changes request remains outstanding
  - Blocker: Bot review threads remain unresolved
  - Blocker: Proceeded without thepastaclaw: no review within the configured window

<a id="romchornyi-clean"></a>
#### Clean (1)
- [dashpay/rust-dashcore#1002 fix(dash-spv): make backward coverage durable by rewinding synced_height instead of sweeping in memory](https://github.com/dashpay/rust-dashcore/pull/1002) — areas: dash-spv, key-wallet-manager, fallback · ⚠ ownership unresolved: dash-spv, key-wallet-manager · Policy: configuration-error
  - Blocker: Unresolved identities in dash-spv

<a id="zocolini"></a>
### @ZocoLini
<a id="zocolini-open"></a>
#### Open (9)
- [dashpay/rust-dashcore#375 refactor(dash-spv): checkpoint rewrite](https://github.com/dashpay/rust-dashcore/pull/375) — 2 unresolved (2 CodeRabbit) · 240 days stale · ⚠ merge conflict · 🐢 untouched 121 days · areas: dash-spv, fallback · ⚠ ownership unresolved: dash-spv · Policy: configuration-error
  - Top thread: "_⚠️ Potential issue_ \| _🟡 Minor_" — 240 days old
  - Blocker: Unresolved identities in dash-spv
- [dashpay/rust-dashcore#902 refactor(dash-spv): network manager refactor and sync pipelines optimized ](https://github.com/dashpay/rust-dashcore/pull/902) — ✋ changes requested · 🔴 CI failing · areas: dash-spv, fallback · ⚠ ownership unresolved: dash-spv · Policy: configuration-error
  - Blocker: Unresolved identities in dash-spv
- [dashpay/rust-dashcore#921 fix(dash-spv): sync backfill](https://github.com/dashpay/rust-dashcore/pull/921) — ⚠ merge conflict · 📝 draft · areas: dash-spv, fallback · ⚠ ownership unresolved: dash-spv · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#993 fix(dash-spv): wire up masternode persistence and shrink it 67x](https://github.com/dashpay/rust-dashcore/pull/993) — areas: dash-spv, fallback · ⚠ ownership unresolved: dash-spv · Policy: configuration-error
  - Blocker: Unresolved identities in dash-spv
- [dashpay/rust-dashcore#1014 fix(key-wallet): prune observed spends during the initial sync](https://github.com/dashpay/rust-dashcore/pull/1014) — areas: dash-spv, key-wallet, key-wallet-manager · ⚠ ownership unresolved: dash-spv, key-wallet, key-wallet-manager · Policy: configuration-error
  - Blocker: Unresolved identities in dash-spv
- [dashpay/rust-dashcore#1015 fix(key-wallet): re-apply a spend whose coin was funded after it](https://github.com/dashpay/rust-dashcore/pull/1015) — areas: dash-spv, key-wallet, key-wallet-manager · ⚠ ownership unresolved: dash-spv, key-wallet, key-wallet-manager · Policy: configuration-error
  - Blocker: Unresolved identities in dash-spv
- [dashpay/rust-dashcore#1016 Refactor/drop committed range sweep](https://github.com/dashpay/rust-dashcore/pull/1016) — 🔴 CI failing · 📝 draft · areas: dash-spv, key-wallet, key-wallet-manager · ⚠ ownership unresolved: dash-spv, key-wallet, key-wallet-manager · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#1024 perf(dash-spv): reduce ram usage](https://github.com/dashpay/rust-dashcore/pull/1024) — 🔴 CI failing · 📝 draft · areas: dash-spv, fallback · ⚠ ownership unresolved: dash-spv · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#1035 fix(key-wallet): extend DIP-15 contact pools as their payments arrive](https://github.com/dashpay/rust-dashcore/pull/1035) — areas: key-wallet · ⚠ ownership unresolved: key-wallet · Policy: waiting-slot
  - Blocker: Waiting for one of five author review slots

<a id="zocolini-needs-action"></a>
#### Needs action (1)
- [dashpay/rust-dashcore#902 refactor(dash-spv): network manager refactor and sync pipelines optimized ](https://github.com/dashpay/rust-dashcore/pull/902) — ✋ changes requested · 🔴 CI failing · areas: dash-spv, fallback · ⚠ ownership unresolved: dash-spv · Policy: configuration-error
  - Blocker: Unresolved identities in dash-spv

<a id="zocolini-changes-requested"></a>
#### Changes Requested (1)
- [dashpay/rust-dashcore#902 refactor(dash-spv): network manager refactor and sync pipelines optimized ](https://github.com/dashpay/rust-dashcore/pull/902) — ✋ changes requested · 🔴 CI failing · areas: dash-spv, fallback · ⚠ ownership unresolved: dash-spv · Policy: configuration-error
  - Blocker: Unresolved identities in dash-spv

<a id="zocolini-draft"></a>
#### Draft (3)
- [dashpay/rust-dashcore#921 fix(dash-spv): sync backfill](https://github.com/dashpay/rust-dashcore/pull/921) — ⚠ merge conflict · 📝 draft · areas: dash-spv, fallback · ⚠ ownership unresolved: dash-spv · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#1016 Refactor/drop committed range sweep](https://github.com/dashpay/rust-dashcore/pull/1016) — 🔴 CI failing · 📝 draft · areas: dash-spv, key-wallet, key-wallet-manager · ⚠ ownership unresolved: dash-spv, key-wallet, key-wallet-manager · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#1024 perf(dash-spv): reduce ram usage](https://github.com/dashpay/rust-dashcore/pull/1024) — 🔴 CI failing · 📝 draft · areas: dash-spv, fallback · ⚠ ownership unresolved: dash-spv · Policy: draft
  - Blocker: Draft PR does not occupy a review slot

<a id="zocolini-stale"></a>
#### Stale (1)
- [dashpay/rust-dashcore#375 refactor(dash-spv): checkpoint rewrite](https://github.com/dashpay/rust-dashcore/pull/375) — 2 unresolved (2 CodeRabbit) · 240 days stale · ⚠ merge conflict · 🐢 untouched 121 days · areas: dash-spv, fallback · ⚠ ownership unresolved: dash-spv · Policy: configuration-error
  - Top thread: "_⚠️ Potential issue_ \| _🟡 Minor_" — 240 days old
  - Blocker: Unresolved identities in dash-spv

<a id="zocolini-clean"></a>
#### Clean (4)
- [dashpay/rust-dashcore#993 fix(dash-spv): wire up masternode persistence and shrink it 67x](https://github.com/dashpay/rust-dashcore/pull/993) — areas: dash-spv, fallback · ⚠ ownership unresolved: dash-spv · Policy: configuration-error
  - Blocker: Unresolved identities in dash-spv
- [dashpay/rust-dashcore#1014 fix(key-wallet): prune observed spends during the initial sync](https://github.com/dashpay/rust-dashcore/pull/1014) — areas: dash-spv, key-wallet, key-wallet-manager · ⚠ ownership unresolved: dash-spv, key-wallet, key-wallet-manager · Policy: configuration-error
  - Blocker: Unresolved identities in dash-spv
- [dashpay/rust-dashcore#1015 fix(key-wallet): re-apply a spend whose coin was funded after it](https://github.com/dashpay/rust-dashcore/pull/1015) — areas: dash-spv, key-wallet, key-wallet-manager · ⚠ ownership unresolved: dash-spv, key-wallet, key-wallet-manager · Policy: configuration-error
  - Blocker: Unresolved identities in dash-spv
- [dashpay/rust-dashcore#1035 fix(key-wallet): extend DIP-15 contact pools as their payments arrive](https://github.com/dashpay/rust-dashcore/pull/1035) — areas: key-wallet · ⚠ ownership unresolved: key-wallet · Policy: waiting-slot
  - Blocker: Waiting for one of five author review slots

<a id="zocolini-ready-for-review"></a>
#### Ready for Review (1)
- [dashpay/rust-dashcore#1002 fix(dash-spv): make backward coverage durable by rewinding synced_height instead of sweeping in memory](https://github.com/dashpay/rust-dashcore/pull/1002) — by @romchornyi · areas: dash-spv, key-wallet-manager, fallback · ⚠ ownership unresolved: dash-spv, key-wallet-manager · Policy: configuration-error
  - Blocker: Unresolved identities in dash-spv

<a id="hashengineering"></a>
### @HashEngineering
<a id="hashengineering-open"></a>
#### Open (2)
- [dashpay/platform#4439 fix(kotlin-sdk): reconcile the TXO store against the engine and repair restored address pools](https://github.com/dashpay/platform/pull/4439) — ⚠ merge conflict · 📝 draft · 🐢 targets v4.3-dev · areas: rs-platform-wallet, rs-platform-wallet-ffi, kotlin-sdk, fallback
- [dashpay/rust-dashcore#979 fix(key-wallet, dash-spv): re-emit late knowledge — record corrections, durable rescans, address-pool repair](https://github.com/dashpay/rust-dashcore/pull/979) — 📝 draft · areas: dash-spv, key-wallet · ⚠ ownership unresolved: dash-spv, key-wallet · Policy: draft
  - Blocker: Draft PR does not occupy a review slot

<a id="hashengineering-draft"></a>
#### Draft (1)
- [dashpay/rust-dashcore#979 fix(key-wallet, dash-spv): re-emit late knowledge — record corrections, durable rescans, address-pool repair](https://github.com/dashpay/rust-dashcore/pull/979) — 📝 draft · areas: dash-spv, key-wallet · ⚠ ownership unresolved: dash-spv, key-wallet · Policy: draft
  - Blocker: Draft PR does not occupy a review slot

<a id="hashengineering-stale"></a>
#### Stale (1)
- [dashpay/platform#4439 fix(kotlin-sdk): reconcile the TXO store against the engine and repair restored address pools](https://github.com/dashpay/platform/pull/4439) — ⚠ merge conflict · 📝 draft · 🐢 targets v4.3-dev · areas: rs-platform-wallet, rs-platform-wallet-ffi, kotlin-sdk, fallback

<a id="hashengineering-ready-for-review"></a>
#### Ready for Review (1)
- [dashpay/rust-dashcore#1035 fix(key-wallet): extend DIP-15 contact pools as their payments arrive](https://github.com/dashpay/rust-dashcore/pull/1035) — by @ZocoLini · areas: key-wallet · ⚠ ownership unresolved: key-wallet · Policy: waiting-slot
  - Blocker: Waiting for one of five author review slots

<a id="infraclaw-dash"></a>
### @infraclaw-dash
<a id="infraclaw-dash-open"></a>
#### Open (2)
- [dashpay/platform#3958 ci: add Platform testnet sync status reporting](https://github.com/dashpay/platform/pull/3958) — 2 unresolved (2 bot) · 86 days stale · ⚠ merge conflict · 🐢 targets v4.3-dev · areas: fallback
  - Top thread: "🟡 Suggestion: workflow_dispatch lets any repo writer forge a sync status**" — 86 days old
- [dashpay/platform#4702 ci: remove host privilege requirements from persistent Linux runners](https://github.com/dashpay/platform/pull/4702) — 1 unresolved (1 bot) · 6 days stale · 🔴 CI failing · 🐢 targets v4.3-dev · areas: fallback
  - Top thread: "🟡 Suggestion: Do not require self-hosted Debian packages in the shared action on hosted runners**" — 6 days old

<a id="infraclaw-dash-stale"></a>
#### Stale (2)
- [dashpay/platform#3958 ci: add Platform testnet sync status reporting](https://github.com/dashpay/platform/pull/3958) — 2 unresolved (2 bot) · 86 days stale · ⚠ merge conflict · 🐢 targets v4.3-dev · areas: fallback
  - Top thread: "🟡 Suggestion: workflow_dispatch lets any repo writer forge a sync status**" — 86 days old
- [dashpay/platform#4702 ci: remove host privilege requirements from persistent Linux runners](https://github.com/dashpay/platform/pull/4702) — 1 unresolved (1 bot) · 6 days stale · 🔴 CI failing · 🐢 targets v4.3-dev · areas: fallback
  - Top thread: "🟡 Suggestion: Do not require self-hosted Debian packages in the shared action on hosted runners**" — 6 days old

<a id="bfoss765"></a>
### @bfoss765
<a id="bfoss765-open"></a>
#### Open (2)
- [dashpay/platform#4313 feat(kotlin-sdk): one-time Orchard key shielded-invite API (inviter + claim)](https://github.com/dashpay/platform/pull/4313) — 2 unresolved (2 bot) · 18 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · 🐢 targets v4.3-dev · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk, fallback
  - Top thread: "🟡 Suggestion: Preserve claim-binding mismatch semantics across the FFI boundary**" — 18 days old
- [dashpay/platform#4312 feat(platform-wallet): multi-output shielded transfers + output-aware fee predictor](https://github.com/dashpay/platform/pull/4312) — 1 unresolved (1 bot) · 18 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · 🐢 targets v4.3-dev · areas: dpp, rs-platform-wallet, rs-platform-wallet-ffi, kotlin-sdk, fallback
  - Top thread: "🟡 Suggestion: Keep envelope-aware action counting behind the crate boundary**" — 18 days old

<a id="bfoss765-stale"></a>
#### Stale (2)
- [dashpay/platform#4313 feat(kotlin-sdk): one-time Orchard key shielded-invite API (inviter + claim)](https://github.com/dashpay/platform/pull/4313) — 2 unresolved (2 bot) · 18 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · 🐢 targets v4.3-dev · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk, fallback
  - Top thread: "🟡 Suggestion: Preserve claim-binding mismatch semantics across the FFI boundary**" — 18 days old
- [dashpay/platform#4312 feat(platform-wallet): multi-output shielded transfers + output-aware fee predictor](https://github.com/dashpay/platform/pull/4312) — 1 unresolved (1 bot) · 18 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · 🐢 targets v4.3-dev · areas: dpp, rs-platform-wallet, rs-platform-wallet-ffi, kotlin-sdk, fallback
  - Top thread: "🟡 Suggestion: Keep envelope-aware action counting behind the crate boundary**" — 18 days old

## Methodology
Generated nightly by [pr-hygiene](https://github.com/dashpay/stale_prs_are_bad). A thread counts as "unresolved" when it is open, not outdated, has a comment from someone other than the PR author, and the most recent comment is from a reviewer. **Dirty** = at least one such thread. **Unresolved Comments** = at least one such thread. **Changes Requested** = no unresolved threads but a reviewer's most recent review is CHANGES_REQUESTED (still blocking until someone re-approves or dismisses). **Deferred** = carries a configured deferred label (e.g. `postponed`) — visible but not counted toward unresolved-comment counts. **Stale** = targets a non-default branch OR hasn't been touched in the configured threshold (default 120 days, but clean PRs are never reclassified as stale). **Draft** = the PR is still marked draft on GitHub. **CI failing** = no unresolved comments, no changes-requested, but the latest commit's status check is failing. **Clean** = open, not draft, not deferred, not stale, no unresolved comments, no changes-requested, CI green. **Needs action** further requires changes-requested, merge conflict, or that the reviewer commented more recently than the author last pushed. **Ready for human** counts a person's own PRs that the shared review engine marks `ready-for-human` or `ready-to-merge`; each PR bullet shows the engine's state as `Policy:` with its blockers. **Ready for Review** counts clean PRs (authored by someone else) where this person owes a review: the union of the shared policy's routing (owners and reviewers of every area the changed files fall into, or the repository fallback for files no area claims) and GitHub's explicit review requests. The author is never routed to their own PR, and anyone who has already submitted any review is excluded — their job is done. `⚠ ownership unresolved` marks areas whose roster the policy still lists as open. Configurable via [`https://github.com/dashpay/stale_prs_are_bad/blob/master/.pr-hygiene.yml`](https://github.com/dashpay/stale_prs_are_bad/blob/master/.pr-hygiene.yml)—edit defaults there; ownership lives in `policies/`.
