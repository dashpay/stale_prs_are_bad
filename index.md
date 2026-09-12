---
---
# PR Hygiene Report
*Last updated: 2026-09-12 18:14 UTC · commit a95eab4*

## Summary
- Open PRs: **128** (22 clean · 8 CI failing · 4 changes requested · 43 unresolved comments · 5 deferred · 35 draft · 11 stale)
- PRs needing author action: **54**
- Total unresolved comments: **222**
- dashpay/platform: **72** open (9 clean · 3 CI failing · 2 changes requested · 31 unresolved comments · 5 deferred · 14 draft · 8 stale) · engine: 5 configuration-error · 15 draft · 26 waiting-bots · 2 waiting-self-review · 12 waiting-slot · 12 no verdict
- dashpay/rust-dashcore: **37** open (5 clean · 5 CI failing · 2 changes requested · 5 unresolved comments · 0 deferred · 17 draft · 3 stale) · engine: 12 configuration-error · 17 draft · 2 waiting-bots · 3 waiting-slot · 3 no verdict
- dashpay/tenderdash: **4** open (0 clean · 0 CI failing · 0 changes requested · 1 unresolved comments · 0 deferred · 3 draft · 0 stale) · engine: 3 draft · 1 waiting-bots
- dashpay/grovedb: **1** open (0 clean · 0 CI failing · 0 changes requested · 1 unresolved comments · 0 deferred · 0 draft · 0 stale) · engine: 1 waiting-bots
- dashpay/dash-evo-tool: **14** open (8 clean · 0 CI failing · 0 changes requested · 5 unresolved comments · 0 deferred · 1 draft · 0 stale) · engine: 1 draft · 11 waiting-bots · 1 waiting-self-review · 1 waiting-slot

## Scoreboard
_Sort: unresolved-comments desc → needs-action desc → ready-for-review desc. Click any number to jump to the specific PRs it covers._

| Author | Open | Clean | CI failing | Unresolved Comments | Changes Requested | Deferred | Draft | Stale | Needs action | Ready for human | Total Unresolved Comments | Ready for Review | Δ |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| [@PastaPastaPasta + (@thepastaclaw)](#pastapastapasta) | [25+(13)](#pastapastapasta-open) | [2+(8)](#pastapastapasta-clean) | [1+(1)](#pastapastapasta-ci-failing) | [12+(1)](#pastapastapasta-unresolved-comments) | — | — | [5+(2)](#pastapastapasta-draft) | [5+(1)](#pastapastapasta-stale) | [13+(4)](#pastapastapasta-needs-action) | — | [61+(6)](#pastapastapasta-unresolved-comments) | — | ↑ 5 |
| [@lklimek + (@Claudius-Maginificent)](#lklimek) | [11+(5)](#lklimek-open) | [2+(0)](#lklimek-clean) | — | [4+(2)](#lklimek-unresolved-comments) | — | — | [5+(3)](#lklimek-draft) | — | [5+(2)](#lklimek-needs-action) | — | [5+(35)](#lklimek-unresolved-comments) | [7+(0)](#lklimek-ready-for-review) | ↑ 3 |
| [@shumkov](#shumkov) | [13](#shumkov-open) | [1](#shumkov-clean) | [2](#shumkov-ci-failing) | [5](#shumkov-unresolved-comments) | [1](#shumkov-changes-requested) | [2](#shumkov-deferred) | [2](#shumkov-draft) | — | [7](#shumkov-needs-action) | — | [56](#shumkov-unresolved-comments) | [5](#shumkov-ready-for-review) | ↑ 3 |
| [@QuantumExplorer](#quantumexplorer) | [16](#quantumexplorer-open) | [2](#quantumexplorer-clean) | [1](#quantumexplorer-ci-failing) | [5](#quantumexplorer-unresolved-comments) | [1](#quantumexplorer-changes-requested) | [2](#quantumexplorer-deferred) | [2](#quantumexplorer-draft) | [3](#quantumexplorer-stale) | [6](#quantumexplorer-needs-action) | — | [23](#quantumexplorer-unresolved-comments) | [6](#quantumexplorer-ready-for-review) | — |
| [@romchornyi](#romchornyi) | [5](#romchornyi-open) | [2](#romchornyi-clean) | — | [3](#romchornyi-unresolved-comments) | — | — | — | — | [4](#romchornyi-needs-action) | — | [6](#romchornyi-unresolved-comments) | [4](#romchornyi-ready-for-review) | ↑ 3 |
| [@llbartekll](#llbartekll) | [6](#llbartekll-open) | [1](#llbartekll-clean) | — | [2](#llbartekll-unresolved-comments) | [1](#llbartekll-changes-requested) | — | [2](#llbartekll-draft) | — | [3](#llbartekll-needs-action) | — | [4](#llbartekll-unresolved-comments) | [3](#llbartekll-ready-for-review) | — |
| [@HashEngineering](#hashengineering) | [4](#hashengineering-open) | — | — | [2](#hashengineering-unresolved-comments) | — | — | [2](#hashengineering-draft) | — | [2](#hashengineering-needs-action) | — | [2](#hashengineering-unresolved-comments) | [3](#hashengineering-ready-for-review) | ↑ 1 |
| [@infraclaw-dash](#infraclaw-dash) | [2](#infraclaw-dash-open) | — | — | [2](#infraclaw-dash-unresolved-comments) | — | — | — | — | [2](#infraclaw-dash-needs-action) | — | [3](#infraclaw-dash-unresolved-comments) | — | ↑ 1 |
| [@bfoss765](#bfoss765) | [2](#bfoss765-open) | — | — | [2](#bfoss765-unresolved-comments) | — | — | — | — | [2](#bfoss765-needs-action) | — | [3](#bfoss765-unresolved-comments) | — | — |
| [@ZocoLini](#zocolini) | [14](#zocolini-open) | [2](#zocolini-clean) | [3](#zocolini-ci-failing) | [1](#zocolini-unresolved-comments) | [1](#zocolini-changes-requested) | — | [6](#zocolini-draft) | [1](#zocolini-stale) | [2](#zocolini-needs-action) | — | [10](#zocolini-unresolved-comments) | [5](#zocolini-ready-for-review) | ↑ 2 |
| [@xdustinface](#xdustinface) | [8](#xdustinface-open) | [1](#xdustinface-clean) | — | [1](#xdustinface-unresolved-comments) | — | — | [6](#xdustinface-draft) | — | [1](#xdustinface-needs-action) | — | [1](#xdustinface-unresolved-comments) | [3](#xdustinface-ready-for-review) | — |
| [@vivekgsharma](#vivekgsharma) | [1](#vivekgsharma-open) | — | — | [1](#vivekgsharma-unresolved-comments) | — | — | — | — | [1](#vivekgsharma-needs-action) | — | [2](#vivekgsharma-unresolved-comments) | — | — |
| [@pshenmic](#pshenmic) | [1](#pshenmic-open) | — | — | — | — | — | — | [1](#pshenmic-stale) | — | — | [5](#pshenmic-unresolved-comments) | — | — |
| [@ogabrielides](#ogabrielides) | [1](#ogabrielides-open) | — | — | — | — | [1](#ogabrielides-deferred) | — | — | — | — | — | — | — |
| [@thephez](#thephez) | [1](#thephez-open) | [1](#thephez-clean) | — | — | — | — | — | — | — | — | — | — | — |

## Per-author detail

<a id="pastapastapasta"></a>
### @PastaPastaPasta + (@thepastaclaw)
<a id="pastapastapasta-open"></a>
#### Open (38)
- [dashpay/rust-dashcore#264 feat(dash-spv): Add BIP324 v2 encrypted P2P transport](https://github.com/dashpay/rust-dashcore/pull/264) — 3 unresolved (3 CodeRabbit) · 275 days stale · ⚠ merge conflict · 🔴 CI failing · areas: dash-spv, fallback · ⚠ ownership unresolved: dash-spv · Policy: configuration-error
  - Top thread: "_⚠️ Potential issue_ \| _🟡 Minor_" — 275 days old
  - Blocker: Unresolved identities in dash-spv
- [dashpay/platform#3096 feat(sdk): add client-side validation to state transition construction methods](https://github.com/dashpay/platform/pull/3096) — via @thepastaclaw · 4 unresolved (2 CodeRabbit, 2 human) · 206 days stale · areas: rs-drive-abci, dpp, js-wasm-sdk, fallback · Policy: waiting-bots
  - Top thread: "_⚠️ Potential issue_ \| _🟡 Minor_" — 204 days old
  - Blocker: thepastaclaw final review missing for current head
  - Blocker: CodeRabbit completion missing for current head
  - Blocker: Bot review threads remain unresolved
- [dashpay/platform#4573 feat(drive-abci): debug-only per-block phase timing](https://github.com/dashpay/platform/pull/4573) — 8 unresolved (5 human, 3 bot) · 4 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · areas: rs-drive-abci · Policy: waiting-slot
  - Top thread: "I would add some prefix here" — 3 days old
  - Blocker: Waiting for one of five author review slots
- [dashpay/rust-dashcore#137 fix: integration tests, continued dashificiation](https://github.com/dashpay/rust-dashcore/pull/137) — 2 unresolved (2 CodeRabbit) · 372 days stale · ⚠ merge conflict · 🔴 CI failing · 🐢 targets v0.40-dev, untouched 275 days · areas: fallback
  - Top thread: "_🛠️ Refactor suggestion_" — 372 days old
- [dashpay/platform#4616 feat(platform-wallet)!: support DashPay shielded tips with dedicated accounts](https://github.com/dashpay/platform/pull/4616) — 12 unresolved (7 human, 5 bot) · 4 days stale · ⚠ merge conflict · areas: rs-drive, rs-drive-abci, rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk, rust-sdk, system-contracts, kotlin-sdk, fallback · Policy: waiting-slot
  - Top thread: "🟡 Suggestion: Exercise flush failures before returning a publishable tip address**" — 4 days old
  - Blocker: Waiting for one of five author review slots
- [dashpay/platform#4562 ci: build release SDKs and NPM packages on self-hosted runners](https://github.com/dashpay/platform/pull/4562) — 7 unresolved (3 CodeRabbit, 4 bot) · 11 days stale · ✋ changes requested · areas: fallback · Policy: waiting-bots
  - Top thread: "_🎯 Functional Correctness_ \| _🟠 Major_ \| _⚡ Quick win_" — 11 days old
  - Blocker: thepastaclaw final review missing for current head
  - Blocker: CodeRabbit completion missing for current head
  - Blocker: Bot changes request remains outstanding
  - … 1 more blocker(s)
- [dashpay/platform#4530 test(dashmate): live state sync e2e — join tooling, churn, re-sync and fallback coverage](https://github.com/dashpay/platform/pull/4530) — 6 unresolved (6 bot) · 13 days stale · ⚠ merge conflict · 📝 draft · areas: rs-drive, rs-drive-abci, dpp, dashmate, js-wasm-sdk, fallback · ⚠ ownership unresolved: dashmate · Policy: draft
  - Top thread: "🟡 Suggestion: Require the churn scenario to interrupt an active state sync**" — 13 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#4432 fix(sdk): enforce server limit parity in the aggregate proof verifiers](https://github.com/dashpay/platform/pull/4432) — 3 unresolved (3 bot) · 23 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · 📝 draft · areas: fallback · Policy: draft
  - Top thread: "🔴 Blocking: The default limit is not the server's actual cap**" — 23 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#4571 perf(drive-abci): stop rewriting the whole platform state every block](https://github.com/dashpay/platform/pull/4571) — 5 unresolved (1 CodeRabbit, 4 bot) · 3 days stale · areas: rs-drive, rs-drive-abci, fallback · Policy: waiting-bots
  - Top thread: "🟡 Suggestion: Encapsulate the fields that participate in dirty tracking**" — 3 days old
  - Blocker: Bot review threads remain unresolved
- [dashpay/rust-dashcore#901 fix(key-wallet): discover Coinbase/AssetUnlock outputs for all fund-bearing accounts](https://github.com/dashpay/rust-dashcore/pull/901) — via @thepastaclaw · 2 unresolved (2 CodeRabbit) · 37 days stale · ⚠ merge conflict · ✋ changes requested · 📝 draft · areas: dash-spv, key-wallet · ⚠ ownership unresolved: dash-spv, key-wallet · Policy: draft
  - Top thread: "_🎯 Functional Correctness_ \| _🟡 Minor_ \| _⚡ Quick win_" — 37 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#4521 feat(dashmate): state sync configuration for tenderdash and drive snapshots](https://github.com/dashpay/platform/pull/4521) — 2 unresolved (2 bot) · 13 days stale · 📝 draft · areas: dashmate · ⚠ ownership unresolved: dashmate · Policy: draft
  - Top thread: "🔴 Blocking: Default snapshot serving exposes unbounded remote checkpoint retention**" — 13 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#4538 fix(sdk): honor explicit regtest addresses in dapi-client; tolerate empty discovery in wasm-sdk trusted context](https://github.com/dashpay/platform/pull/4538) — 2 unresolved (2 bot) · 13 days stale · 📝 draft · areas: js-wasm-sdk, fallback · Policy: draft
  - Top thread: "🟡 Suggestion: Add deterministic tests for the network-scoped discovery fallback**" — 13 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#4648 feat(drive-abci)!: state sync via ABCI snapshots with reduced platform state (protocol v15)](https://github.com/dashpay/platform/pull/4648) — 3 unresolved (3 bot) · 2 days stale · 🔴 CI failing · areas: rs-drive, rs-drive-abci, dpp, fallback · Policy: waiting-slot
  - Top thread: "🟡 Suggestion: Do not silently skip unsupported trees beyond a full discovery page**" — 2 days old
  - Blocker: Waiting for one of five author review slots
- [dashpay/platform#4613 feat(platform)!: add contract-scoped authentication keys](https://github.com/dashpay/platform/pull/4613) — 2 unresolved (1 CodeRabbit, 1 bot) · 2 days stale · areas: rs-drive, rs-drive-abci, dpp, rs-platform-wallet-ffi, fallback · Policy: waiting-slot
  - Top thread: "_🎯 Functional Correctness_ \| _🟠 Major_ \| _⚡ Quick win_" — 2 days old
  - Blocker: Waiting for one of five author review slots
- [dashpay/platform#4632 feat(sdk)!: pure DPNS and DashPay document builders shared with embedders](https://github.com/dashpay/platform/pull/4632) — 2 unresolved (2 bot) · 1 days stale · areas: rust-sdk, fallback · Policy: waiting-slot
  - Top thread: "🟡 Suggestion: Builder documentation recommends a pre-check that rejects valid contract labels**" — 1 days old
  - Blocker: Waiting for one of five author review slots
- [dashpay/platform#4623 feat(platform-wallet): reserve DashPay payout addresses without Core funding](https://github.com/dashpay/platform/pull/4623) — 1 unresolved (1 bot) · 4 days stale · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk · Policy: waiting-slot
  - Top thread: "🟡 Suggestion: Exercise overlapping persistence to protect the new payment gate**" — 4 days old
  - Blocker: Waiting for one of five author review slots
- [dashpay/platform#4539 fix(dashmate): generate release seeds and complete missing node identities](https://github.com/dashpay/platform/pull/4539) — 1 unresolved (1 bot) · 4 days stale · areas: dashmate, fallback · ⚠ ownership unresolved: dashmate · Policy: configuration-error
  - Top thread: "🔴 Blocking: Avoid ranking bootstrap peers by operator-grindable node IDs**" — 4 days old
  - Blocker: Unresolved identities in dashmate
- [dashpay/platform#4392 perf(swift-sdk): linear wallet-changeset rounds via per-round bulk-prefetch cache](https://github.com/dashpay/platform/pull/4392) — 1 unresolved (1 bot) · 4 days stale · ⚠ merge conflict · areas: swift-sdk · Policy: waiting-bots
  - Top thread: "🟡 Suggestion: Exercise transient pending-input fetch failures followed by recovery**" — 4 days old
  - Blocker: CodeRabbit completion missing for current head
  - Blocker: Bot review threads remain unresolved
- [dashpay/platform#4662 fix(drive-abci)!: reject contested document id collisions](https://github.com/dashpay/platform/pull/4662) — 1 unresolved (1 bot) · 1 days stale · areas: rs-drive-abci, fallback · Policy: waiting-slot
  - Top thread: "🟡 Suggestion: Exercise the collision on both sides of the protocol upgrade**" — 1 days old
  - Blocker: Waiting for one of five author review slots
- [dashpay/dash-evo-tool#624 fix(ui): add wallet alias trimming and 64-char length limit](https://github.com/dashpay/dash-evo-tool/pull/624) — via @thepastaclaw · areas: dash-evo-tool · Policy: waiting-bots
  - Blocker: CodeRabbit completion missing for current head
  - Blocker: Bot review threads remain unresolved
- [dashpay/dash-evo-tool#761 fix(ui): improve auth key error message and suppress on startup auto-select](https://github.com/dashpay/dash-evo-tool/pull/761) — via @thepastaclaw · ⚠ merge conflict · areas: dash-evo-tool · Policy: waiting-bots
  - Blocker: CodeRabbit completion missing for current head
  - Blocker: Bot review threads remain unresolved
- [dashpay/dash-evo-tool#762 fix: accept HIGH security level keys for profile updates](https://github.com/dashpay/dash-evo-tool/pull/762) — via @thepastaclaw · areas: dash-evo-tool · Policy: waiting-bots
  - Blocker: thepastaclaw final review missing for current head
  - Blocker: CodeRabbit completion missing for current head
  - Blocker: Bot review threads remain unresolved
- [dashpay/dash-evo-tool#772 docs: add weekly smoke test cases and prerequisites](https://github.com/dashpay/dash-evo-tool/pull/772) — via @thepastaclaw · areas: dash-evo-tool · Policy: waiting-bots
  - Blocker: thepastaclaw final review missing for current head
  - Blocker: CodeRabbit completion missing for current head
  - Blocker: Bot review threads remain unresolved
- [dashpay/dash-evo-tool#853 fix: show parsed platform-address transitions](https://github.com/dashpay/dash-evo-tool/pull/853) — via @thepastaclaw · areas: dash-evo-tool · Policy: waiting-bots
  - Blocker: CodeRabbit completion missing for current head
  - Blocker: Bot review threads remain unresolved
- [dashpay/dash-evo-tool#859 perf: detect system theme off UI thread](https://github.com/dashpay/dash-evo-tool/pull/859) — via @thepastaclaw · areas: dash-evo-tool · Policy: waiting-slot
  - Blocker: Waiting for one of five author review slots
- [dashpay/platform#4136 fix(dashmate): use live Tenderdash app version for protocol status](https://github.com/dashpay/platform/pull/4136) — via @thepastaclaw · ⚠ merge conflict · areas: dashmate · ⚠ ownership unresolved: dashmate · Policy: configuration-error
  - Blocker: Unresolved identities in dashmate
- [dashpay/platform#4298 fix(dashmate): load ZeroSSL config in force mode](https://github.com/dashpay/platform/pull/4298) — via @thepastaclaw · 🐢 targets v4.1-dev · areas: dashmate · ⚠ ownership unresolved: dashmate
- [dashpay/platform#4454 test(swift-sdk): pin non-English BIP-39 mnemonic support against the fixed FFI](https://github.com/dashpay/platform/pull/4454) — areas: swift-sdk · Policy: waiting-bots
  - Blocker: CodeRabbit completion missing for current head
- [dashpay/platform#4615 feat(sdk)!: optional BIP-39 passphrase through the mnemonic resolver and wallet creation](https://github.com/dashpay/platform/pull/4615) — 📝 draft · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, rust-sdk-ffi, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#4618 refactor(drive-abci): share the v1 document-query wire decoders](https://github.com/dashpay/platform/pull/4618) — areas: rs-drive-abci, fallback · Policy: waiting-slot
  - Blocker: Waiting for one of five author review slots
- [dashpay/platform#4619 feat(sdk)!: verify document proofs against the wire request and share DPNS/DashPay document assembly](https://github.com/dashpay/platform/pull/4619) — 📝 draft · 🐢 targets refactor/platform-query-wire-and-dpp-bounds · areas: dpp, rust-sdk, fallback
- [dashpay/platform#4620 feat(sdk): add transport-free CXX bindings for C++ embedders](https://github.com/dashpay/platform/pull/4620) — 📝 draft · 🐢 targets feat/request-driven-document-verification · areas: fallback
- [dashpay/platform#4633 feat(sdk): add CXX bindings over dash-sdk for C++ embedders](https://github.com/dashpay/platform/pull/4633) — ⚠ merge conflict · 📝 draft · 🐢 targets feat/shared-dpns-dashpay-builders · areas: dpp, rust-sdk, fallback
- [dashpay/platform#4655 feat(sdk)!: add scoped authentication key SDK support](https://github.com/dashpay/platform/pull/4655) — 🐢 targets feat/scoped-contract-auth-keys · areas: rs-platform-wallet-ffi, swift-sdk, rust-sdk-ffi, js-wasm-sdk, kotlin-sdk, fallback
- [dashpay/rust-dashcore#749 fix(ffi): free transaction byte buffers correctly](https://github.com/dashpay/rust-dashcore/pull/749) — via @thepastaclaw · ⚠ merge conflict · 🔴 CI failing · areas: fallback · Policy: waiting-bots
  - Blocker: thepastaclaw final review missing for current head
- [dashpay/rust-dashcore#752 fix(ffi): validate account seed lengths](https://github.com/dashpay/rust-dashcore/pull/752) — via @thepastaclaw · ⚠ merge conflict · areas: fallback · Policy: waiting-bots
  - Blocker: thepastaclaw final review missing for current head
- [dashpay/rust-dashcore#904 fix(dash-spv): harden Windows dashd_sync startup wallet load](https://github.com/dashpay/rust-dashcore/pull/904) — via @thepastaclaw · ⚠ merge conflict · 📝 draft · areas: dash-spv · ⚠ ownership unresolved: dash-spv · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#959 feat(dash-spv): adaptive gap-limit probe escalation for BIP44 discovery](https://github.com/dashpay/rust-dashcore/pull/959) — ⚠ merge conflict · 🔴 CI failing · areas: dash-spv, key-wallet, key-wallet-manager, fallback · ⚠ ownership unresolved: dash-spv, key-wallet, key-wallet-manager · Policy: configuration-error
  - Blocker: Unresolved identities in dash-spv

<a id="pastapastapasta-needs-action"></a>
#### Needs action (17)
- [dashpay/rust-dashcore#264 feat(dash-spv): Add BIP324 v2 encrypted P2P transport](https://github.com/dashpay/rust-dashcore/pull/264) — 3 unresolved (3 CodeRabbit) · 275 days stale · ⚠ merge conflict · 🔴 CI failing · areas: dash-spv, fallback · ⚠ ownership unresolved: dash-spv · Policy: configuration-error
  - Top thread: "_⚠️ Potential issue_ \| _🟡 Minor_" — 275 days old
  - Blocker: Unresolved identities in dash-spv
- [dashpay/platform#4573 feat(drive-abci): debug-only per-block phase timing](https://github.com/dashpay/platform/pull/4573) — 8 unresolved (5 human, 3 bot) · 4 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · areas: rs-drive-abci · Policy: waiting-slot
  - Top thread: "I would add some prefix here" — 3 days old
  - Blocker: Waiting for one of five author review slots
- [dashpay/platform#4616 feat(platform-wallet)!: support DashPay shielded tips with dedicated accounts](https://github.com/dashpay/platform/pull/4616) — 12 unresolved (7 human, 5 bot) · 4 days stale · ⚠ merge conflict · areas: rs-drive, rs-drive-abci, rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk, rust-sdk, system-contracts, kotlin-sdk, fallback · Policy: waiting-slot
  - Top thread: "🟡 Suggestion: Exercise flush failures before returning a publishable tip address**" — 4 days old
  - Blocker: Waiting for one of five author review slots
- [dashpay/platform#4562 ci: build release SDKs and NPM packages on self-hosted runners](https://github.com/dashpay/platform/pull/4562) — 7 unresolved (3 CodeRabbit, 4 bot) · 11 days stale · ✋ changes requested · areas: fallback · Policy: waiting-bots
  - Top thread: "_🎯 Functional Correctness_ \| _🟠 Major_ \| _⚡ Quick win_" — 11 days old
  - Blocker: thepastaclaw final review missing for current head
  - Blocker: CodeRabbit completion missing for current head
  - Blocker: Bot changes request remains outstanding
  - … 1 more blocker(s)
- [dashpay/platform#4571 perf(drive-abci): stop rewriting the whole platform state every block](https://github.com/dashpay/platform/pull/4571) — 5 unresolved (1 CodeRabbit, 4 bot) · 3 days stale · areas: rs-drive, rs-drive-abci, fallback · Policy: waiting-bots
  - Top thread: "🟡 Suggestion: Encapsulate the fields that participate in dirty tracking**" — 3 days old
  - Blocker: Bot review threads remain unresolved
- [dashpay/platform#4648 feat(drive-abci)!: state sync via ABCI snapshots with reduced platform state (protocol v15)](https://github.com/dashpay/platform/pull/4648) — 3 unresolved (3 bot) · 2 days stale · 🔴 CI failing · areas: rs-drive, rs-drive-abci, dpp, fallback · Policy: waiting-slot
  - Top thread: "🟡 Suggestion: Do not silently skip unsupported trees beyond a full discovery page**" — 2 days old
  - Blocker: Waiting for one of five author review slots
- [dashpay/platform#4613 feat(platform)!: add contract-scoped authentication keys](https://github.com/dashpay/platform/pull/4613) — 2 unresolved (1 CodeRabbit, 1 bot) · 2 days stale · areas: rs-drive, rs-drive-abci, dpp, rs-platform-wallet-ffi, fallback · Policy: waiting-slot
  - Top thread: "_🎯 Functional Correctness_ \| _🟠 Major_ \| _⚡ Quick win_" — 2 days old
  - Blocker: Waiting for one of five author review slots
- [dashpay/platform#4632 feat(sdk)!: pure DPNS and DashPay document builders shared with embedders](https://github.com/dashpay/platform/pull/4632) — 2 unresolved (2 bot) · 1 days stale · areas: rust-sdk, fallback · Policy: waiting-slot
  - Top thread: "🟡 Suggestion: Builder documentation recommends a pre-check that rejects valid contract labels**" — 1 days old
  - Blocker: Waiting for one of five author review slots
- [dashpay/platform#4623 feat(platform-wallet): reserve DashPay payout addresses without Core funding](https://github.com/dashpay/platform/pull/4623) — 1 unresolved (1 bot) · 4 days stale · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk · Policy: waiting-slot
  - Top thread: "🟡 Suggestion: Exercise overlapping persistence to protect the new payment gate**" — 4 days old
  - Blocker: Waiting for one of five author review slots
- [dashpay/platform#4539 fix(dashmate): generate release seeds and complete missing node identities](https://github.com/dashpay/platform/pull/4539) — 1 unresolved (1 bot) · 4 days stale · areas: dashmate, fallback · ⚠ ownership unresolved: dashmate · Policy: configuration-error
  - Top thread: "🔴 Blocking: Avoid ranking bootstrap peers by operator-grindable node IDs**" — 4 days old
  - Blocker: Unresolved identities in dashmate
- [dashpay/platform#4392 perf(swift-sdk): linear wallet-changeset rounds via per-round bulk-prefetch cache](https://github.com/dashpay/platform/pull/4392) — 1 unresolved (1 bot) · 4 days stale · ⚠ merge conflict · areas: swift-sdk · Policy: waiting-bots
  - Top thread: "🟡 Suggestion: Exercise transient pending-input fetch failures followed by recovery**" — 4 days old
  - Blocker: CodeRabbit completion missing for current head
  - Blocker: Bot review threads remain unresolved
- [dashpay/platform#4662 fix(drive-abci)!: reject contested document id collisions](https://github.com/dashpay/platform/pull/4662) — 1 unresolved (1 bot) · 1 days stale · areas: rs-drive-abci, fallback · Policy: waiting-slot
  - Top thread: "🟡 Suggestion: Exercise the collision on both sides of the protocol upgrade**" — 1 days old
  - Blocker: Waiting for one of five author review slots
- [dashpay/dash-evo-tool#761 fix(ui): improve auth key error message and suppress on startup auto-select](https://github.com/dashpay/dash-evo-tool/pull/761) — via @thepastaclaw · ⚠ merge conflict · areas: dash-evo-tool · Policy: waiting-bots
  - Blocker: CodeRabbit completion missing for current head
  - Blocker: Bot review threads remain unresolved
- [dashpay/platform#4136 fix(dashmate): use live Tenderdash app version for protocol status](https://github.com/dashpay/platform/pull/4136) — via @thepastaclaw · ⚠ merge conflict · areas: dashmate · ⚠ ownership unresolved: dashmate · Policy: configuration-error
  - Blocker: Unresolved identities in dashmate
- [dashpay/rust-dashcore#749 fix(ffi): free transaction byte buffers correctly](https://github.com/dashpay/rust-dashcore/pull/749) — via @thepastaclaw · ⚠ merge conflict · 🔴 CI failing · areas: fallback · Policy: waiting-bots
  - Blocker: thepastaclaw final review missing for current head
- [dashpay/rust-dashcore#752 fix(ffi): validate account seed lengths](https://github.com/dashpay/rust-dashcore/pull/752) — via @thepastaclaw · ⚠ merge conflict · areas: fallback · Policy: waiting-bots
  - Blocker: thepastaclaw final review missing for current head
- [dashpay/rust-dashcore#959 feat(dash-spv): adaptive gap-limit probe escalation for BIP44 discovery](https://github.com/dashpay/rust-dashcore/pull/959) — ⚠ merge conflict · 🔴 CI failing · areas: dash-spv, key-wallet, key-wallet-manager, fallback · ⚠ ownership unresolved: dash-spv, key-wallet, key-wallet-manager · Policy: configuration-error
  - Blocker: Unresolved identities in dash-spv

<a id="pastapastapasta-unresolved-comments"></a>
#### Unresolved Comments (13)
- [dashpay/rust-dashcore#264 feat(dash-spv): Add BIP324 v2 encrypted P2P transport](https://github.com/dashpay/rust-dashcore/pull/264) — 3 unresolved (3 CodeRabbit) · 275 days stale · ⚠ merge conflict · 🔴 CI failing · areas: dash-spv, fallback · ⚠ ownership unresolved: dash-spv · Policy: configuration-error
  - Top thread: "_⚠️ Potential issue_ \| _🟡 Minor_" — 275 days old
  - Blocker: Unresolved identities in dash-spv
- [dashpay/platform#3096 feat(sdk): add client-side validation to state transition construction methods](https://github.com/dashpay/platform/pull/3096) — via @thepastaclaw · 4 unresolved (2 CodeRabbit, 2 human) · 206 days stale · areas: rs-drive-abci, dpp, js-wasm-sdk, fallback · Policy: waiting-bots
  - Top thread: "_⚠️ Potential issue_ \| _🟡 Minor_" — 204 days old
  - Blocker: thepastaclaw final review missing for current head
  - Blocker: CodeRabbit completion missing for current head
  - Blocker: Bot review threads remain unresolved
- [dashpay/platform#4573 feat(drive-abci): debug-only per-block phase timing](https://github.com/dashpay/platform/pull/4573) — 8 unresolved (5 human, 3 bot) · 4 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · areas: rs-drive-abci · Policy: waiting-slot
  - Top thread: "I would add some prefix here" — 3 days old
  - Blocker: Waiting for one of five author review slots
- [dashpay/platform#4616 feat(platform-wallet)!: support DashPay shielded tips with dedicated accounts](https://github.com/dashpay/platform/pull/4616) — 12 unresolved (7 human, 5 bot) · 4 days stale · ⚠ merge conflict · areas: rs-drive, rs-drive-abci, rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk, rust-sdk, system-contracts, kotlin-sdk, fallback · Policy: waiting-slot
  - Top thread: "🟡 Suggestion: Exercise flush failures before returning a publishable tip address**" — 4 days old
  - Blocker: Waiting for one of five author review slots
- [dashpay/platform#4562 ci: build release SDKs and NPM packages on self-hosted runners](https://github.com/dashpay/platform/pull/4562) — 7 unresolved (3 CodeRabbit, 4 bot) · 11 days stale · ✋ changes requested · areas: fallback · Policy: waiting-bots
  - Top thread: "_🎯 Functional Correctness_ \| _🟠 Major_ \| _⚡ Quick win_" — 11 days old
  - Blocker: thepastaclaw final review missing for current head
  - Blocker: CodeRabbit completion missing for current head
  - Blocker: Bot changes request remains outstanding
  - … 1 more blocker(s)
- [dashpay/platform#4571 perf(drive-abci): stop rewriting the whole platform state every block](https://github.com/dashpay/platform/pull/4571) — 5 unresolved (1 CodeRabbit, 4 bot) · 3 days stale · areas: rs-drive, rs-drive-abci, fallback · Policy: waiting-bots
  - Top thread: "🟡 Suggestion: Encapsulate the fields that participate in dirty tracking**" — 3 days old
  - Blocker: Bot review threads remain unresolved
- [dashpay/platform#4648 feat(drive-abci)!: state sync via ABCI snapshots with reduced platform state (protocol v15)](https://github.com/dashpay/platform/pull/4648) — 3 unresolved (3 bot) · 2 days stale · 🔴 CI failing · areas: rs-drive, rs-drive-abci, dpp, fallback · Policy: waiting-slot
  - Top thread: "🟡 Suggestion: Do not silently skip unsupported trees beyond a full discovery page**" — 2 days old
  - Blocker: Waiting for one of five author review slots
- [dashpay/platform#4613 feat(platform)!: add contract-scoped authentication keys](https://github.com/dashpay/platform/pull/4613) — 2 unresolved (1 CodeRabbit, 1 bot) · 2 days stale · areas: rs-drive, rs-drive-abci, dpp, rs-platform-wallet-ffi, fallback · Policy: waiting-slot
  - Top thread: "_🎯 Functional Correctness_ \| _🟠 Major_ \| _⚡ Quick win_" — 2 days old
  - Blocker: Waiting for one of five author review slots
- [dashpay/platform#4632 feat(sdk)!: pure DPNS and DashPay document builders shared with embedders](https://github.com/dashpay/platform/pull/4632) — 2 unresolved (2 bot) · 1 days stale · areas: rust-sdk, fallback · Policy: waiting-slot
  - Top thread: "🟡 Suggestion: Builder documentation recommends a pre-check that rejects valid contract labels**" — 1 days old
  - Blocker: Waiting for one of five author review slots
- [dashpay/platform#4623 feat(platform-wallet): reserve DashPay payout addresses without Core funding](https://github.com/dashpay/platform/pull/4623) — 1 unresolved (1 bot) · 4 days stale · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk · Policy: waiting-slot
  - Top thread: "🟡 Suggestion: Exercise overlapping persistence to protect the new payment gate**" — 4 days old
  - Blocker: Waiting for one of five author review slots
- [dashpay/platform#4539 fix(dashmate): generate release seeds and complete missing node identities](https://github.com/dashpay/platform/pull/4539) — 1 unresolved (1 bot) · 4 days stale · areas: dashmate, fallback · ⚠ ownership unresolved: dashmate · Policy: configuration-error
  - Top thread: "🔴 Blocking: Avoid ranking bootstrap peers by operator-grindable node IDs**" — 4 days old
  - Blocker: Unresolved identities in dashmate
- [dashpay/platform#4392 perf(swift-sdk): linear wallet-changeset rounds via per-round bulk-prefetch cache](https://github.com/dashpay/platform/pull/4392) — 1 unresolved (1 bot) · 4 days stale · ⚠ merge conflict · areas: swift-sdk · Policy: waiting-bots
  - Top thread: "🟡 Suggestion: Exercise transient pending-input fetch failures followed by recovery**" — 4 days old
  - Blocker: CodeRabbit completion missing for current head
  - Blocker: Bot review threads remain unresolved
- [dashpay/platform#4662 fix(drive-abci)!: reject contested document id collisions](https://github.com/dashpay/platform/pull/4662) — 1 unresolved (1 bot) · 1 days stale · areas: rs-drive-abci, fallback · Policy: waiting-slot
  - Top thread: "🟡 Suggestion: Exercise the collision on both sides of the protocol upgrade**" — 1 days old
  - Blocker: Waiting for one of five author review slots

<a id="pastapastapasta-ci-failing"></a>
#### CI Failing (2)
- [dashpay/rust-dashcore#749 fix(ffi): free transaction byte buffers correctly](https://github.com/dashpay/rust-dashcore/pull/749) — via @thepastaclaw · ⚠ merge conflict · 🔴 CI failing · areas: fallback · Policy: waiting-bots
  - Blocker: thepastaclaw final review missing for current head
- [dashpay/rust-dashcore#959 feat(dash-spv): adaptive gap-limit probe escalation for BIP44 discovery](https://github.com/dashpay/rust-dashcore/pull/959) — ⚠ merge conflict · 🔴 CI failing · areas: dash-spv, key-wallet, key-wallet-manager, fallback · ⚠ ownership unresolved: dash-spv, key-wallet, key-wallet-manager · Policy: configuration-error
  - Blocker: Unresolved identities in dash-spv

<a id="pastapastapasta-draft"></a>
#### Draft (7)
- [dashpay/platform#4530 test(dashmate): live state sync e2e — join tooling, churn, re-sync and fallback coverage](https://github.com/dashpay/platform/pull/4530) — 6 unresolved (6 bot) · 13 days stale · ⚠ merge conflict · 📝 draft · areas: rs-drive, rs-drive-abci, dpp, dashmate, js-wasm-sdk, fallback · ⚠ ownership unresolved: dashmate · Policy: draft
  - Top thread: "🟡 Suggestion: Require the churn scenario to interrupt an active state sync**" — 13 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#4432 fix(sdk): enforce server limit parity in the aggregate proof verifiers](https://github.com/dashpay/platform/pull/4432) — 3 unresolved (3 bot) · 23 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · 📝 draft · areas: fallback · Policy: draft
  - Top thread: "🔴 Blocking: The default limit is not the server's actual cap**" — 23 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#901 fix(key-wallet): discover Coinbase/AssetUnlock outputs for all fund-bearing accounts](https://github.com/dashpay/rust-dashcore/pull/901) — via @thepastaclaw · 2 unresolved (2 CodeRabbit) · 37 days stale · ⚠ merge conflict · ✋ changes requested · 📝 draft · areas: dash-spv, key-wallet · ⚠ ownership unresolved: dash-spv, key-wallet · Policy: draft
  - Top thread: "_🎯 Functional Correctness_ \| _🟡 Minor_ \| _⚡ Quick win_" — 37 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#4521 feat(dashmate): state sync configuration for tenderdash and drive snapshots](https://github.com/dashpay/platform/pull/4521) — 2 unresolved (2 bot) · 13 days stale · 📝 draft · areas: dashmate · ⚠ ownership unresolved: dashmate · Policy: draft
  - Top thread: "🔴 Blocking: Default snapshot serving exposes unbounded remote checkpoint retention**" — 13 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#4538 fix(sdk): honor explicit regtest addresses in dapi-client; tolerate empty discovery in wasm-sdk trusted context](https://github.com/dashpay/platform/pull/4538) — 2 unresolved (2 bot) · 13 days stale · 📝 draft · areas: js-wasm-sdk, fallback · Policy: draft
  - Top thread: "🟡 Suggestion: Add deterministic tests for the network-scoped discovery fallback**" — 13 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#4615 feat(sdk)!: optional BIP-39 passphrase through the mnemonic resolver and wallet creation](https://github.com/dashpay/platform/pull/4615) — 📝 draft · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, rust-sdk-ffi, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#904 fix(dash-spv): harden Windows dashd_sync startup wallet load](https://github.com/dashpay/rust-dashcore/pull/904) — via @thepastaclaw · ⚠ merge conflict · 📝 draft · areas: dash-spv · ⚠ ownership unresolved: dash-spv · Policy: draft
  - Blocker: Draft PR does not occupy a review slot

<a id="pastapastapasta-stale"></a>
#### Stale (6)
- [dashpay/rust-dashcore#137 fix: integration tests, continued dashificiation](https://github.com/dashpay/rust-dashcore/pull/137) — 2 unresolved (2 CodeRabbit) · 372 days stale · ⚠ merge conflict · 🔴 CI failing · 🐢 targets v0.40-dev, untouched 275 days · areas: fallback
  - Top thread: "_🛠️ Refactor suggestion_" — 372 days old
- [dashpay/platform#4298 fix(dashmate): load ZeroSSL config in force mode](https://github.com/dashpay/platform/pull/4298) — via @thepastaclaw · 🐢 targets v4.1-dev · areas: dashmate · ⚠ ownership unresolved: dashmate
- [dashpay/platform#4619 feat(sdk)!: verify document proofs against the wire request and share DPNS/DashPay document assembly](https://github.com/dashpay/platform/pull/4619) — 📝 draft · 🐢 targets refactor/platform-query-wire-and-dpp-bounds · areas: dpp, rust-sdk, fallback
- [dashpay/platform#4620 feat(sdk): add transport-free CXX bindings for C++ embedders](https://github.com/dashpay/platform/pull/4620) — 📝 draft · 🐢 targets feat/request-driven-document-verification · areas: fallback
- [dashpay/platform#4633 feat(sdk): add CXX bindings over dash-sdk for C++ embedders](https://github.com/dashpay/platform/pull/4633) — ⚠ merge conflict · 📝 draft · 🐢 targets feat/shared-dpns-dashpay-builders · areas: dpp, rust-sdk, fallback
- [dashpay/platform#4655 feat(sdk)!: add scoped authentication key SDK support](https://github.com/dashpay/platform/pull/4655) — 🐢 targets feat/scoped-contract-auth-keys · areas: rs-platform-wallet-ffi, swift-sdk, rust-sdk-ffi, js-wasm-sdk, kotlin-sdk, fallback

<a id="pastapastapasta-clean"></a>
#### Clean (10)
- [dashpay/dash-evo-tool#624 fix(ui): add wallet alias trimming and 64-char length limit](https://github.com/dashpay/dash-evo-tool/pull/624) — via @thepastaclaw · areas: dash-evo-tool · Policy: waiting-bots
  - Blocker: CodeRabbit completion missing for current head
  - Blocker: Bot review threads remain unresolved
- [dashpay/dash-evo-tool#761 fix(ui): improve auth key error message and suppress on startup auto-select](https://github.com/dashpay/dash-evo-tool/pull/761) — via @thepastaclaw · ⚠ merge conflict · areas: dash-evo-tool · Policy: waiting-bots
  - Blocker: CodeRabbit completion missing for current head
  - Blocker: Bot review threads remain unresolved
- [dashpay/dash-evo-tool#762 fix: accept HIGH security level keys for profile updates](https://github.com/dashpay/dash-evo-tool/pull/762) — via @thepastaclaw · areas: dash-evo-tool · Policy: waiting-bots
  - Blocker: thepastaclaw final review missing for current head
  - Blocker: CodeRabbit completion missing for current head
  - Blocker: Bot review threads remain unresolved
- [dashpay/dash-evo-tool#772 docs: add weekly smoke test cases and prerequisites](https://github.com/dashpay/dash-evo-tool/pull/772) — via @thepastaclaw · areas: dash-evo-tool · Policy: waiting-bots
  - Blocker: thepastaclaw final review missing for current head
  - Blocker: CodeRabbit completion missing for current head
  - Blocker: Bot review threads remain unresolved
- [dashpay/dash-evo-tool#853 fix: show parsed platform-address transitions](https://github.com/dashpay/dash-evo-tool/pull/853) — via @thepastaclaw · areas: dash-evo-tool · Policy: waiting-bots
  - Blocker: CodeRabbit completion missing for current head
  - Blocker: Bot review threads remain unresolved
- [dashpay/dash-evo-tool#859 perf: detect system theme off UI thread](https://github.com/dashpay/dash-evo-tool/pull/859) — via @thepastaclaw · areas: dash-evo-tool · Policy: waiting-slot
  - Blocker: Waiting for one of five author review slots
- [dashpay/platform#4136 fix(dashmate): use live Tenderdash app version for protocol status](https://github.com/dashpay/platform/pull/4136) — via @thepastaclaw · ⚠ merge conflict · areas: dashmate · ⚠ ownership unresolved: dashmate · Policy: configuration-error
  - Blocker: Unresolved identities in dashmate
- [dashpay/platform#4454 test(swift-sdk): pin non-English BIP-39 mnemonic support against the fixed FFI](https://github.com/dashpay/platform/pull/4454) — areas: swift-sdk · Policy: waiting-bots
  - Blocker: CodeRabbit completion missing for current head
- [dashpay/platform#4618 refactor(drive-abci): share the v1 document-query wire decoders](https://github.com/dashpay/platform/pull/4618) — areas: rs-drive-abci, fallback · Policy: waiting-slot
  - Blocker: Waiting for one of five author review slots
- [dashpay/rust-dashcore#752 fix(ffi): validate account seed lengths](https://github.com/dashpay/rust-dashcore/pull/752) — via @thepastaclaw · ⚠ merge conflict · areas: fallback · Policy: waiting-bots
  - Blocker: thepastaclaw final review missing for current head

<a id="lklimek"></a>
### @lklimek + (@Claudius-Maginificent)
<a id="lklimek-open"></a>
#### Open (16)
- [dashpay/platform#3549 test(platform-wallet): e2e framework + full test suite — triage pins, Found-*/PA-* guards, fail-closed persist, Stage-2 merge](https://github.com/dashpay/platform/pull/3549) — via @Claudius-Maginificent · 9 unresolved (9 bot) · 133 days stale · 📝 draft · areas: rs-drive, rs-drive-abci, rs-platform-wallet, rs-platform-wallet-ffi, rust-sdk, fallback · Policy: draft
  - Top thread: "🟡 Suggestion: `from_seed_for_identity` is misleadingly named, half-functional, and unused**" — 133 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/dash-evo-tool#983 test(migration): add cross-version migration test matrix](https://github.com/dashpay/dash-evo-tool/pull/983) — via @Claudius-Maginificent · 21 unresolved (11 CodeRabbit, 10 bot) · 1 days stale · ✋ changes requested · areas: dash-evo-tool · Policy: waiting-bots
  - Top thread: "_🔒 Security & Privacy_ \| _🛡️ Analyzed with Security Review_ \| _🟠 Major_ \| _⚡ Quick win_" — 1 days old
  - Blocker: thepastaclaw final review missing for current head
  - Blocker: CodeRabbit completion missing for current head
  - Blocker: Bot changes request remains outstanding
  - … 1 more blocker(s)
- [dashpay/platform#3992 feat(platform-wallet): manifest integrity checksum (Risk-6/R12.5 follow-up)](https://github.com/dashpay/platform/pull/3992) — via @Claudius-Maginificent · 4 unresolved (4 bot) · 71 days stale · ⚠ merge conflict · ✋ changes requested · 📝 draft · areas: rs-platform-wallet, wallet-storage, fallback · Policy: draft
  - Top thread: "🟡 Suggestion: verify_manifest_checksums fails hard on oversize blob, breaking the per-wallet skip contract for exactly t…" — 71 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/dash-evo-tool#851 fix(ui): center text inside buttons with min_size across the app](https://github.com/dashpay/dash-evo-tool/pull/851) — 2 unresolved (2 bot) · 140 days stale · ⚠ merge conflict · areas: dash-evo-tool · Policy: waiting-bots
  - Top thread: "🟡 Suggestion: Add a kittest harness for the new add_sized button-sizing contract**" — 140 days old
  - Blocker: thepastaclaw final review missing for current head
  - Blocker: CodeRabbit completion missing for current head
  - Blocker: Bot review threads remain unresolved
- [dashpay/dash-evo-tool#976 fix(security): protect identity keys before import storage](https://github.com/dashpay/dash-evo-tool/pull/976) — 1 unresolved (1 CodeRabbit) · 3 days stale · areas: dash-evo-tool · Policy: waiting-bots
  - Top thread: "_📐 Maintainability & Code Quality_ \| _🟡 Minor_ \| _⚡ Quick win_" — 3 days old
  - Blocker: Bot review threads remain unresolved
- [dashpay/dash-evo-tool#979 refactor(dashpay): publish profiles through platform-wallet](https://github.com/dashpay/dash-evo-tool/pull/979) — 1 unresolved (1 bot) · 1 days stale · ⚠ merge conflict · areas: dash-evo-tool · Policy: waiting-bots
  - Top thread: "🟡 Suggestion: Preserve profile timestamps when mirroring a fetched profile**" — 1 days old
  - Blocker: CodeRabbit completion missing for current head
  - Blocker: Bot changes request remains outstanding
  - Blocker: Bot review threads remain unresolved
- [dashpay/platform#4661 build(dashmate): update Tenderdash image to 1.8.0-dev.2](https://github.com/dashpay/platform/pull/4661) — via @Claudius-Maginificent · 1 unresolved (1 bot) · 1 days stale · ✋ changes requested · areas: dashmate · ⚠ ownership unresolved: dashmate · Policy: configuration-error
  - Top thread: "🔴 Blocking: Use a new migration key for configs already stamped 4.2.0**" — 1 days old
  - Blocker: Unresolved identities in dashmate
- [dashpay/tenderdash#1457 test(consensus): deflake TestWALRoundsSkipper](https://github.com/dashpay/tenderdash/pull/1457) — 1 unresolved (1 bot) · 1 days stale · areas: tenderdash · Policy: waiting-bots
  - Top thread: "🟡 Suggestion: Stop replay-side file writes before temporary-directory cleanup**" — 1 days old
  - Blocker: CodeRabbit completion missing for current head
  - Blocker: Bot review threads remain unresolved
- [dashpay/dash-evo-tool#901 feat(dpns): unify safe masternode voting operations](https://github.com/dashpay/dash-evo-tool/pull/901) — ⚠ merge conflict · areas: dash-evo-tool · Policy: waiting-bots
  - Blocker: thepastaclaw final review missing for current head
  - Blocker: CodeRabbit completion missing for current head
- [dashpay/dash-evo-tool#977 test(backend-e2e): reserve platform withdrawal fees](https://github.com/dashpay/dash-evo-tool/pull/977) — areas: dash-evo-tool · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 4720e751900082d344215c457302b2e844eb1c23 after bot completion
- [dashpay/dash-evo-tool#980 feat(identity): add wallet-derived keys to existing identities](https://github.com/dashpay/dash-evo-tool/pull/980) — 📝 draft · areas: dash-evo-tool · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#4587 fix(platform-wallet): register contact accounts via add_managed_account, dedup provider-key rebuild, and cleanup](https://github.com/dashpay/platform/pull/4587) — via @Claudius-Maginificent · 📝 draft · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#4653 fix(platform-wallet): support HASH160 DashPay profile signing keys](https://github.com/dashpay/platform/pull/4653) — 📝 draft · areas: rs-platform-wallet · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/tenderdash#1458 ci: restore the Go cache in build, govulncheck and check-generated](https://github.com/dashpay/tenderdash/pull/1458) — 📝 draft · areas: tenderdash · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/tenderdash#1459 ci(e2e): cut avoidable setup time from the e2e workflow](https://github.com/dashpay/tenderdash/pull/1459) — 🔴 CI failing · 📝 draft · areas: tenderdash · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/tenderdash#1460 fix(e2e): restore state sync and fix the p2p startup race that slow the test networks](https://github.com/dashpay/tenderdash/pull/1460) — 📝 draft · areas: tenderdash · Policy: draft
  - Blocker: Draft PR does not occupy a review slot

<a id="lklimek-needs-action"></a>
#### Needs action (7)
- [dashpay/dash-evo-tool#983 test(migration): add cross-version migration test matrix](https://github.com/dashpay/dash-evo-tool/pull/983) — via @Claudius-Maginificent · 21 unresolved (11 CodeRabbit, 10 bot) · 1 days stale · ✋ changes requested · areas: dash-evo-tool · Policy: waiting-bots
  - Top thread: "_🔒 Security & Privacy_ \| _🛡️ Analyzed with Security Review_ \| _🟠 Major_ \| _⚡ Quick win_" — 1 days old
  - Blocker: thepastaclaw final review missing for current head
  - Blocker: CodeRabbit completion missing for current head
  - Blocker: Bot changes request remains outstanding
  - … 1 more blocker(s)
- [dashpay/dash-evo-tool#851 fix(ui): center text inside buttons with min_size across the app](https://github.com/dashpay/dash-evo-tool/pull/851) — 2 unresolved (2 bot) · 140 days stale · ⚠ merge conflict · areas: dash-evo-tool · Policy: waiting-bots
  - Top thread: "🟡 Suggestion: Add a kittest harness for the new add_sized button-sizing contract**" — 140 days old
  - Blocker: thepastaclaw final review missing for current head
  - Blocker: CodeRabbit completion missing for current head
  - Blocker: Bot review threads remain unresolved
- [dashpay/dash-evo-tool#976 fix(security): protect identity keys before import storage](https://github.com/dashpay/dash-evo-tool/pull/976) — 1 unresolved (1 CodeRabbit) · 3 days stale · areas: dash-evo-tool · Policy: waiting-bots
  - Top thread: "_📐 Maintainability & Code Quality_ \| _🟡 Minor_ \| _⚡ Quick win_" — 3 days old
  - Blocker: Bot review threads remain unresolved
- [dashpay/dash-evo-tool#979 refactor(dashpay): publish profiles through platform-wallet](https://github.com/dashpay/dash-evo-tool/pull/979) — 1 unresolved (1 bot) · 1 days stale · ⚠ merge conflict · areas: dash-evo-tool · Policy: waiting-bots
  - Top thread: "🟡 Suggestion: Preserve profile timestamps when mirroring a fetched profile**" — 1 days old
  - Blocker: CodeRabbit completion missing for current head
  - Blocker: Bot changes request remains outstanding
  - Blocker: Bot review threads remain unresolved
- [dashpay/platform#4661 build(dashmate): update Tenderdash image to 1.8.0-dev.2](https://github.com/dashpay/platform/pull/4661) — via @Claudius-Maginificent · 1 unresolved (1 bot) · 1 days stale · ✋ changes requested · areas: dashmate · ⚠ ownership unresolved: dashmate · Policy: configuration-error
  - Top thread: "🔴 Blocking: Use a new migration key for configs already stamped 4.2.0**" — 1 days old
  - Blocker: Unresolved identities in dashmate
- [dashpay/tenderdash#1457 test(consensus): deflake TestWALRoundsSkipper](https://github.com/dashpay/tenderdash/pull/1457) — 1 unresolved (1 bot) · 1 days stale · areas: tenderdash · Policy: waiting-bots
  - Top thread: "🟡 Suggestion: Stop replay-side file writes before temporary-directory cleanup**" — 1 days old
  - Blocker: CodeRabbit completion missing for current head
  - Blocker: Bot review threads remain unresolved
- [dashpay/dash-evo-tool#901 feat(dpns): unify safe masternode voting operations](https://github.com/dashpay/dash-evo-tool/pull/901) — ⚠ merge conflict · areas: dash-evo-tool · Policy: waiting-bots
  - Blocker: thepastaclaw final review missing for current head
  - Blocker: CodeRabbit completion missing for current head

<a id="lklimek-unresolved-comments"></a>
#### Unresolved Comments (6)
- [dashpay/dash-evo-tool#983 test(migration): add cross-version migration test matrix](https://github.com/dashpay/dash-evo-tool/pull/983) — via @Claudius-Maginificent · 21 unresolved (11 CodeRabbit, 10 bot) · 1 days stale · ✋ changes requested · areas: dash-evo-tool · Policy: waiting-bots
  - Top thread: "_🔒 Security & Privacy_ \| _🛡️ Analyzed with Security Review_ \| _🟠 Major_ \| _⚡ Quick win_" — 1 days old
  - Blocker: thepastaclaw final review missing for current head
  - Blocker: CodeRabbit completion missing for current head
  - Blocker: Bot changes request remains outstanding
  - … 1 more blocker(s)
- [dashpay/dash-evo-tool#851 fix(ui): center text inside buttons with min_size across the app](https://github.com/dashpay/dash-evo-tool/pull/851) — 2 unresolved (2 bot) · 140 days stale · ⚠ merge conflict · areas: dash-evo-tool · Policy: waiting-bots
  - Top thread: "🟡 Suggestion: Add a kittest harness for the new add_sized button-sizing contract**" — 140 days old
  - Blocker: thepastaclaw final review missing for current head
  - Blocker: CodeRabbit completion missing for current head
  - Blocker: Bot review threads remain unresolved
- [dashpay/dash-evo-tool#976 fix(security): protect identity keys before import storage](https://github.com/dashpay/dash-evo-tool/pull/976) — 1 unresolved (1 CodeRabbit) · 3 days stale · areas: dash-evo-tool · Policy: waiting-bots
  - Top thread: "_📐 Maintainability & Code Quality_ \| _🟡 Minor_ \| _⚡ Quick win_" — 3 days old
  - Blocker: Bot review threads remain unresolved
- [dashpay/dash-evo-tool#979 refactor(dashpay): publish profiles through platform-wallet](https://github.com/dashpay/dash-evo-tool/pull/979) — 1 unresolved (1 bot) · 1 days stale · ⚠ merge conflict · areas: dash-evo-tool · Policy: waiting-bots
  - Top thread: "🟡 Suggestion: Preserve profile timestamps when mirroring a fetched profile**" — 1 days old
  - Blocker: CodeRabbit completion missing for current head
  - Blocker: Bot changes request remains outstanding
  - Blocker: Bot review threads remain unresolved
- [dashpay/platform#4661 build(dashmate): update Tenderdash image to 1.8.0-dev.2](https://github.com/dashpay/platform/pull/4661) — via @Claudius-Maginificent · 1 unresolved (1 bot) · 1 days stale · ✋ changes requested · areas: dashmate · ⚠ ownership unresolved: dashmate · Policy: configuration-error
  - Top thread: "🔴 Blocking: Use a new migration key for configs already stamped 4.2.0**" — 1 days old
  - Blocker: Unresolved identities in dashmate
- [dashpay/tenderdash#1457 test(consensus): deflake TestWALRoundsSkipper](https://github.com/dashpay/tenderdash/pull/1457) — 1 unresolved (1 bot) · 1 days stale · areas: tenderdash · Policy: waiting-bots
  - Top thread: "🟡 Suggestion: Stop replay-side file writes before temporary-directory cleanup**" — 1 days old
  - Blocker: CodeRabbit completion missing for current head
  - Blocker: Bot review threads remain unresolved

<a id="lklimek-draft"></a>
#### Draft (8)
- [dashpay/platform#3549 test(platform-wallet): e2e framework + full test suite — triage pins, Found-*/PA-* guards, fail-closed persist, Stage-2 merge](https://github.com/dashpay/platform/pull/3549) — via @Claudius-Maginificent · 9 unresolved (9 bot) · 133 days stale · 📝 draft · areas: rs-drive, rs-drive-abci, rs-platform-wallet, rs-platform-wallet-ffi, rust-sdk, fallback · Policy: draft
  - Top thread: "🟡 Suggestion: `from_seed_for_identity` is misleadingly named, half-functional, and unused**" — 133 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#3992 feat(platform-wallet): manifest integrity checksum (Risk-6/R12.5 follow-up)](https://github.com/dashpay/platform/pull/3992) — via @Claudius-Maginificent · 4 unresolved (4 bot) · 71 days stale · ⚠ merge conflict · ✋ changes requested · 📝 draft · areas: rs-platform-wallet, wallet-storage, fallback · Policy: draft
  - Top thread: "🟡 Suggestion: verify_manifest_checksums fails hard on oversize blob, breaking the per-wallet skip contract for exactly t…" — 71 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/dash-evo-tool#980 feat(identity): add wallet-derived keys to existing identities](https://github.com/dashpay/dash-evo-tool/pull/980) — 📝 draft · areas: dash-evo-tool · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#4587 fix(platform-wallet): register contact accounts via add_managed_account, dedup provider-key rebuild, and cleanup](https://github.com/dashpay/platform/pull/4587) — via @Claudius-Maginificent · 📝 draft · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#4653 fix(platform-wallet): support HASH160 DashPay profile signing keys](https://github.com/dashpay/platform/pull/4653) — 📝 draft · areas: rs-platform-wallet · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/tenderdash#1458 ci: restore the Go cache in build, govulncheck and check-generated](https://github.com/dashpay/tenderdash/pull/1458) — 📝 draft · areas: tenderdash · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/tenderdash#1459 ci(e2e): cut avoidable setup time from the e2e workflow](https://github.com/dashpay/tenderdash/pull/1459) — 🔴 CI failing · 📝 draft · areas: tenderdash · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/tenderdash#1460 fix(e2e): restore state sync and fix the p2p startup race that slow the test networks](https://github.com/dashpay/tenderdash/pull/1460) — 📝 draft · areas: tenderdash · Policy: draft
  - Blocker: Draft PR does not occupy a review slot

<a id="lklimek-clean"></a>
#### Clean (2)
- [dashpay/dash-evo-tool#901 feat(dpns): unify safe masternode voting operations](https://github.com/dashpay/dash-evo-tool/pull/901) — ⚠ merge conflict · areas: dash-evo-tool · Policy: waiting-bots
  - Blocker: thepastaclaw final review missing for current head
  - Blocker: CodeRabbit completion missing for current head
- [dashpay/dash-evo-tool#977 test(backend-e2e): reserve platform withdrawal fees](https://github.com/dashpay/dash-evo-tool/pull/977) — areas: dash-evo-tool · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 4720e751900082d344215c457302b2e844eb1c23 after bot completion

<a id="lklimek-ready-for-review"></a>
#### Ready for Review (7)
- [dashpay/dash-evo-tool#762 fix: accept HIGH security level keys for profile updates](https://github.com/dashpay/dash-evo-tool/pull/762) — by @thepastaclaw · areas: dash-evo-tool · Policy: waiting-bots
  - Blocker: thepastaclaw final review missing for current head
  - Blocker: CodeRabbit completion missing for current head
  - Blocker: Bot review threads remain unresolved
- [dashpay/dash-evo-tool#772 docs: add weekly smoke test cases and prerequisites](https://github.com/dashpay/dash-evo-tool/pull/772) — by @thepastaclaw · areas: dash-evo-tool · Policy: waiting-bots
  - Blocker: thepastaclaw final review missing for current head
  - Blocker: CodeRabbit completion missing for current head
  - Blocker: Bot review threads remain unresolved
- [dashpay/dash-evo-tool#853 fix: show parsed platform-address transitions](https://github.com/dashpay/dash-evo-tool/pull/853) — by @thepastaclaw · areas: dash-evo-tool · Policy: waiting-bots
  - Blocker: CodeRabbit completion missing for current head
  - Blocker: Bot review threads remain unresolved
- [dashpay/dash-evo-tool#859 perf: detect system theme off UI thread](https://github.com/dashpay/dash-evo-tool/pull/859) — by @thepastaclaw · areas: dash-evo-tool · Policy: waiting-slot
  - Blocker: Waiting for one of five author review slots
- [dashpay/platform#4519 feat(platform-wallet): rotate a masternode's keys into the wallet — ProUpRegTx orchestration, FFI, Swift](https://github.com/dashpay/platform/pull/4519) — by @QuantumExplorer · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk · Policy: waiting-bots
  - Blocker: thepastaclaw final review missing for current head
  - Blocker: CodeRabbit completion missing for current head
  - Blocker: Bot changes request remains outstanding
- [dashpay/platform#4618 refactor(drive-abci): share the v1 document-query wire decoders](https://github.com/dashpay/platform/pull/4618) — by @PastaPastaPasta · areas: rs-drive-abci, fallback · Policy: waiting-slot
  - Blocker: Waiting for one of five author review slots
- [dashpay/platform#4708 feat(platform)!: add ShieldFromIdentity state transition (identity balance to shielded pool)](https://github.com/dashpay/platform/pull/4708) — by @QuantumExplorer · areas: rs-drive, rs-drive-abci, dpp, rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, rust-sdk, kotlin-sdk, fallback · Policy: waiting-slot
  - Blocker: Waiting for one of five author review slots

<a id="shumkov"></a>
### @shumkov
<a id="shumkov-open"></a>
#### Open (13)
- [dashpay/rust-dashcore#634 feat(key-wallet-manager): single-map WalletManager with Arc<RwLock<T>> per wallet](https://github.com/dashpay/rust-dashcore/pull/634) — 20 unresolved (20 CodeRabbit) · 157 days stale · ⚠ merge conflict · ✋ changes requested · areas: dash-spv, key-wallet, key-wallet-manager, fallback · ⚠ ownership unresolved: dash-spv, key-wallet, key-wallet-manager · Policy: configuration-error
  - Top thread: "_⚠️ Potential issue_ \| _🟡 Minor_" — 157 days old
  - Blocker: Unresolved identities in dash-spv
- [dashpay/dash-evo-tool#890 test(masternode-upgrade): standalone masternode-identity upgrade harness + spec](https://github.com/dashpay/dash-evo-tool/pull/890) — 24 unresolved (20 human, 4 bot) · 57 days stale · ✋ changes requested · areas: dash-evo-tool · Policy: waiting-bots
  - Top thread: "🟡 **Backup recipe writes a secret-bearing `~/.dashmate` tarball into `$PWD`, and the root `.gitignore` does not cover it…" — 57 days old
  - Blocker: thepastaclaw final review missing for current head
  - Blocker: CodeRabbit completion missing for current head
  - Blocker: Bot changes request remains outstanding
  - … 1 more blocker(s)
- [dashpay/platform#4282 fix(dashmate)!: give Debian packages versions apt can order](https://github.com/dashpay/platform/pull/4282) — 6 unresolved (1 CodeRabbit, 5 bot) · 12 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · areas: dashmate, js-wasm-sdk, system-contracts, fallback · ⚠ ownership unresolved: dashmate · Policy: configuration-error
  - Top thread: "_🗄️ Data Integrity & Integration_ \| _🟡 Minor_ \| _⚡ Quick win_" — 12 days old
  - Blocker: Unresolved identities in dashmate
- [dashpay/platform#3936 chore(drive-abci): update to nested address in SML](https://github.com/dashpay/platform/pull/3936) — 2 unresolved (2 bot) · 85 days stale · ⚠ merge conflict · 📝 draft · areas: rs-drive-abci, rust-sdk, fallback · Policy: draft
  - Top thread: "🟡 Suggestion: V0 reverse synthesizes `addresses` from a Core-22 port, masking subsequent legacy port-change diffs after …" — 85 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#4283 fix(dashmate): keep the node up when an image pull fails, and stop reporting success when it did not](https://github.com/dashpay/platform/pull/4283) — 2 unresolved (2 bot) · 6 days stale · ⚠ merge conflict · ✋ changes requested · areas: dashmate · ⚠ ownership unresolved: dashmate · Policy: configuration-error
  - Top thread: "🟡 Suggestion: providers.js sanitizer misses U+061C (Arabic letter mark) that sanitizeRemoteText covers**" — 6 days old
  - Blocker: Unresolved identities in dashmate
- [dashpay/platform#4657 feat(platform)!: delete and erase lifecycle for keep-history documents](https://github.com/dashpay/platform/pull/4657) — 2 unresolved (2 bot) · 0 days stale · areas: rs-drive, rs-drive-abci, dpp, rust-sdk, js-wasm-sdk, fallback · Policy: waiting-slot
  - Top thread: "🟡 Suggestion: Reject unsupported erase versions before reserving a nonce**" — 0 days old
  - Blocker: Waiting for one of five author review slots
- [dashpay/platform#2392 feat(dashmate): single node local network [WIP\]](https://github.com/dashpay/platform/pull/2392) — ⏸ deferred · areas: dashmate, js-wasm-sdk, fallback · ⚠ ownership unresolved: dashmate
- [dashpay/platform#2518 feat: token marketplace](https://github.com/dashpay/platform/pull/2518) — 📝 draft · ⏸ deferred · areas: rs-drive, rs-drive-abci, dpp, system-contracts, fallback
- [dashpay/platform#4243 fix(sdk)!: complete encrypted txMetadata parity](https://github.com/dashpay/platform/pull/4243) — ⚠ merge conflict · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk, fallback · Policy: waiting-bots
  - Blocker: CodeRabbit completion missing for current head
  - Blocker: Bot review threads remain unresolved
- [dashpay/platform#4285 feat(platform-wallet)!: invitation links are AppsFlyer applinks only](https://github.com/dashpay/platform/pull/4285) — ⚠ merge conflict · 🔴 CI failing · 📝 draft · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#4636 fix(platform-wallet): close the asset-lock resume broadcast race](https://github.com/dashpay/platform/pull/4636) — 🔴 CI failing · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 751cd3b1033a99d5f503bef95e1f87ad08ca12de after bot completion
- [dashpay/platform#4652 feat(drive)!: preserve composite document history across protocol activation](https://github.com/dashpay/platform/pull/4652) — ✋ changes requested · 🔴 CI failing · areas: rs-drive, rs-drive-abci, rust-sdk, js-wasm-sdk, fallback · Policy: waiting-bots
  - Blocker: thepastaclaw final review missing for current head
  - Blocker: Bot changes request remains outstanding
- [dashpay/platform#4660 feat(sdk): expose the document erase and history lifecycle on mobile and FFI](https://github.com/dashpay/platform/pull/4660) — 🔴 CI failing · areas: rs-drive, rs-drive-abci, dpp, rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, rust-sdk, rust-sdk-ffi, js-wasm-sdk, kotlin-sdk, fallback · Policy: waiting-slot
  - Blocker: Waiting for one of five author review slots

<a id="shumkov-needs-action"></a>
#### Needs action (7)
- [dashpay/rust-dashcore#634 feat(key-wallet-manager): single-map WalletManager with Arc<RwLock<T>> per wallet](https://github.com/dashpay/rust-dashcore/pull/634) — 20 unresolved (20 CodeRabbit) · 157 days stale · ⚠ merge conflict · ✋ changes requested · areas: dash-spv, key-wallet, key-wallet-manager, fallback · ⚠ ownership unresolved: dash-spv, key-wallet, key-wallet-manager · Policy: configuration-error
  - Top thread: "_⚠️ Potential issue_ \| _🟡 Minor_" — 157 days old
  - Blocker: Unresolved identities in dash-spv
- [dashpay/dash-evo-tool#890 test(masternode-upgrade): standalone masternode-identity upgrade harness + spec](https://github.com/dashpay/dash-evo-tool/pull/890) — 24 unresolved (20 human, 4 bot) · 57 days stale · ✋ changes requested · areas: dash-evo-tool · Policy: waiting-bots
  - Top thread: "🟡 **Backup recipe writes a secret-bearing `~/.dashmate` tarball into `$PWD`, and the root `.gitignore` does not cover it…" — 57 days old
  - Blocker: thepastaclaw final review missing for current head
  - Blocker: CodeRabbit completion missing for current head
  - Blocker: Bot changes request remains outstanding
  - … 1 more blocker(s)
- [dashpay/platform#4282 fix(dashmate)!: give Debian packages versions apt can order](https://github.com/dashpay/platform/pull/4282) — 6 unresolved (1 CodeRabbit, 5 bot) · 12 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · areas: dashmate, js-wasm-sdk, system-contracts, fallback · ⚠ ownership unresolved: dashmate · Policy: configuration-error
  - Top thread: "_🗄️ Data Integrity & Integration_ \| _🟡 Minor_ \| _⚡ Quick win_" — 12 days old
  - Blocker: Unresolved identities in dashmate
- [dashpay/platform#4283 fix(dashmate): keep the node up when an image pull fails, and stop reporting success when it did not](https://github.com/dashpay/platform/pull/4283) — 2 unresolved (2 bot) · 6 days stale · ⚠ merge conflict · ✋ changes requested · areas: dashmate · ⚠ ownership unresolved: dashmate · Policy: configuration-error
  - Top thread: "🟡 Suggestion: providers.js sanitizer misses U+061C (Arabic letter mark) that sanitizeRemoteText covers**" — 6 days old
  - Blocker: Unresolved identities in dashmate
- [dashpay/platform#4657 feat(platform)!: delete and erase lifecycle for keep-history documents](https://github.com/dashpay/platform/pull/4657) — 2 unresolved (2 bot) · 0 days stale · areas: rs-drive, rs-drive-abci, dpp, rust-sdk, js-wasm-sdk, fallback · Policy: waiting-slot
  - Top thread: "🟡 Suggestion: Reject unsupported erase versions before reserving a nonce**" — 0 days old
  - Blocker: Waiting for one of five author review slots
- [dashpay/platform#4243 fix(sdk)!: complete encrypted txMetadata parity](https://github.com/dashpay/platform/pull/4243) — ⚠ merge conflict · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk, fallback · Policy: waiting-bots
  - Blocker: CodeRabbit completion missing for current head
  - Blocker: Bot review threads remain unresolved
- [dashpay/platform#4652 feat(drive)!: preserve composite document history across protocol activation](https://github.com/dashpay/platform/pull/4652) — ✋ changes requested · 🔴 CI failing · areas: rs-drive, rs-drive-abci, rust-sdk, js-wasm-sdk, fallback · Policy: waiting-bots
  - Blocker: thepastaclaw final review missing for current head
  - Blocker: Bot changes request remains outstanding

<a id="shumkov-unresolved-comments"></a>
#### Unresolved Comments (5)
- [dashpay/rust-dashcore#634 feat(key-wallet-manager): single-map WalletManager with Arc<RwLock<T>> per wallet](https://github.com/dashpay/rust-dashcore/pull/634) — 20 unresolved (20 CodeRabbit) · 157 days stale · ⚠ merge conflict · ✋ changes requested · areas: dash-spv, key-wallet, key-wallet-manager, fallback · ⚠ ownership unresolved: dash-spv, key-wallet, key-wallet-manager · Policy: configuration-error
  - Top thread: "_⚠️ Potential issue_ \| _🟡 Minor_" — 157 days old
  - Blocker: Unresolved identities in dash-spv
- [dashpay/dash-evo-tool#890 test(masternode-upgrade): standalone masternode-identity upgrade harness + spec](https://github.com/dashpay/dash-evo-tool/pull/890) — 24 unresolved (20 human, 4 bot) · 57 days stale · ✋ changes requested · areas: dash-evo-tool · Policy: waiting-bots
  - Top thread: "🟡 **Backup recipe writes a secret-bearing `~/.dashmate` tarball into `$PWD`, and the root `.gitignore` does not cover it…" — 57 days old
  - Blocker: thepastaclaw final review missing for current head
  - Blocker: CodeRabbit completion missing for current head
  - Blocker: Bot changes request remains outstanding
  - … 1 more blocker(s)
- [dashpay/platform#4282 fix(dashmate)!: give Debian packages versions apt can order](https://github.com/dashpay/platform/pull/4282) — 6 unresolved (1 CodeRabbit, 5 bot) · 12 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · areas: dashmate, js-wasm-sdk, system-contracts, fallback · ⚠ ownership unresolved: dashmate · Policy: configuration-error
  - Top thread: "_🗄️ Data Integrity & Integration_ \| _🟡 Minor_ \| _⚡ Quick win_" — 12 days old
  - Blocker: Unresolved identities in dashmate
- [dashpay/platform#4283 fix(dashmate): keep the node up when an image pull fails, and stop reporting success when it did not](https://github.com/dashpay/platform/pull/4283) — 2 unresolved (2 bot) · 6 days stale · ⚠ merge conflict · ✋ changes requested · areas: dashmate · ⚠ ownership unresolved: dashmate · Policy: configuration-error
  - Top thread: "🟡 Suggestion: providers.js sanitizer misses U+061C (Arabic letter mark) that sanitizeRemoteText covers**" — 6 days old
  - Blocker: Unresolved identities in dashmate
- [dashpay/platform#4657 feat(platform)!: delete and erase lifecycle for keep-history documents](https://github.com/dashpay/platform/pull/4657) — 2 unresolved (2 bot) · 0 days stale · areas: rs-drive, rs-drive-abci, dpp, rust-sdk, js-wasm-sdk, fallback · Policy: waiting-slot
  - Top thread: "🟡 Suggestion: Reject unsupported erase versions before reserving a nonce**" — 0 days old
  - Blocker: Waiting for one of five author review slots

<a id="shumkov-changes-requested"></a>
#### Changes Requested (1)
- [dashpay/platform#4652 feat(drive)!: preserve composite document history across protocol activation](https://github.com/dashpay/platform/pull/4652) — ✋ changes requested · 🔴 CI failing · areas: rs-drive, rs-drive-abci, rust-sdk, js-wasm-sdk, fallback · Policy: waiting-bots
  - Blocker: thepastaclaw final review missing for current head
  - Blocker: Bot changes request remains outstanding

<a id="shumkov-ci-failing"></a>
#### CI Failing (2)
- [dashpay/platform#4636 fix(platform-wallet): close the asset-lock resume broadcast race](https://github.com/dashpay/platform/pull/4636) — 🔴 CI failing · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 751cd3b1033a99d5f503bef95e1f87ad08ca12de after bot completion
- [dashpay/platform#4660 feat(sdk): expose the document erase and history lifecycle on mobile and FFI](https://github.com/dashpay/platform/pull/4660) — 🔴 CI failing · areas: rs-drive, rs-drive-abci, dpp, rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, rust-sdk, rust-sdk-ffi, js-wasm-sdk, kotlin-sdk, fallback · Policy: waiting-slot
  - Blocker: Waiting for one of five author review slots

<a id="shumkov-deferred"></a>
#### Deferred (2)
- [dashpay/platform#2392 feat(dashmate): single node local network [WIP\]](https://github.com/dashpay/platform/pull/2392) — ⏸ deferred · areas: dashmate, js-wasm-sdk, fallback · ⚠ ownership unresolved: dashmate
- [dashpay/platform#2518 feat: token marketplace](https://github.com/dashpay/platform/pull/2518) — 📝 draft · ⏸ deferred · areas: rs-drive, rs-drive-abci, dpp, system-contracts, fallback

<a id="shumkov-draft"></a>
#### Draft (2)
- [dashpay/platform#3936 chore(drive-abci): update to nested address in SML](https://github.com/dashpay/platform/pull/3936) — 2 unresolved (2 bot) · 85 days stale · ⚠ merge conflict · 📝 draft · areas: rs-drive-abci, rust-sdk, fallback · Policy: draft
  - Top thread: "🟡 Suggestion: V0 reverse synthesizes `addresses` from a Core-22 port, masking subsequent legacy port-change diffs after …" — 85 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#4285 feat(platform-wallet)!: invitation links are AppsFlyer applinks only](https://github.com/dashpay/platform/pull/4285) — ⚠ merge conflict · 🔴 CI failing · 📝 draft · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot

<a id="shumkov-clean"></a>
#### Clean (1)
- [dashpay/platform#4243 fix(sdk)!: complete encrypted txMetadata parity](https://github.com/dashpay/platform/pull/4243) — ⚠ merge conflict · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk, fallback · Policy: waiting-bots
  - Blocker: CodeRabbit completion missing for current head
  - Blocker: Bot review threads remain unresolved

<a id="shumkov-ready-for-review"></a>
#### Ready for Review (5)
- [dashpay/platform#4454 test(swift-sdk): pin non-English BIP-39 mnemonic support against the fixed FFI](https://github.com/dashpay/platform/pull/4454) — by @PastaPastaPasta · areas: swift-sdk · Policy: waiting-bots
  - Blocker: CodeRabbit completion missing for current head
- [dashpay/platform#4519 feat(platform-wallet): rotate a masternode's keys into the wallet — ProUpRegTx orchestration, FFI, Swift](https://github.com/dashpay/platform/pull/4519) — by @QuantumExplorer · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk · Policy: waiting-bots
  - Blocker: thepastaclaw final review missing for current head
  - Blocker: CodeRabbit completion missing for current head
  - Blocker: Bot changes request remains outstanding
- [dashpay/platform#4576 docs(wasm-sdk): clarify aggregate groupBy behavior](https://github.com/dashpay/platform/pull/4576) — by @thephez · areas: js-wasm-sdk · Policy: waiting-bots
  - Blocker: CodeRabbit completion missing for current head
- [dashpay/platform#4618 refactor(drive-abci): share the v1 document-query wire decoders](https://github.com/dashpay/platform/pull/4618) — by @PastaPastaPasta · areas: rs-drive-abci, fallback · Policy: waiting-slot
  - Blocker: Waiting for one of five author review slots
- [dashpay/platform#4708 feat(platform)!: add ShieldFromIdentity state transition (identity balance to shielded pool)](https://github.com/dashpay/platform/pull/4708) — by @QuantumExplorer · areas: rs-drive, rs-drive-abci, dpp, rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, rust-sdk, kotlin-sdk, fallback · Policy: waiting-slot
  - Blocker: Waiting for one of five author review slots

<a id="quantumexplorer"></a>
### @QuantumExplorer
<a id="quantumexplorer-open"></a>
#### Open (16)
- [dashpay/platform#4272 feat(dpp)!: dashpay payment detection keys and stealth derivation](https://github.com/dashpay/platform/pull/4272) — 9 unresolved (9 bot) · 40 days stale · ⚠ merge conflict · 📝 draft · areas: rs-drive, rs-drive-abci, dpp, rs-platform-wallet, swift-sdk, system-contracts, kotlin-sdk, fallback · Policy: draft
  - Top thread: "🔴 Blocking: Require compressed 33-byte encodings for payment keys**" — 40 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#622 fix: apply BIP-69 sorting to asset lock credit outputs](https://github.com/dashpay/rust-dashcore/pull/622) — 1 unresolved (1 CodeRabbit) · 163 days stale · ⚠ merge conflict · ✋ changes requested · 📝 draft · areas: key-wallet · ⚠ ownership unresolved: key-wallet · Policy: draft
  - Top thread: "_⚠️ Potential issue_ \| _🟠 Major_" — 163 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#860 feat(dash-spv): expose per-peer connection stats (ping RTT, bytes received)](https://github.com/dashpay/rust-dashcore/pull/860) — 3 unresolved (3 CodeRabbit) · 64 days stale · ✋ changes requested · areas: dash-spv, fallback · ⚠ ownership unresolved: dash-spv · Policy: configuration-error
  - Top thread: "_📐 Maintainability & Code Quality_ \| _🟡 Minor_ \| _⚡ Quick win_" — 64 days old
  - Blocker: Unresolved identities in dash-spv
- [dashpay/platform#3417 feat(swift-sdk): use SPV-synced quorums for Platform proof verification](https://github.com/dashpay/platform/pull/3417) — 5 unresolved (5 bot) · 4 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, rust-sdk, rust-sdk-ffi, fallback · Policy: waiting-bots
  - Top thread: "🟡 Suggestion: Exercise the production runtime bridge across Tokio runtime flavors**" — 4 days old
  - Blocker: thepastaclaw final review missing for current head
  - Blocker: CodeRabbit completion missing for current head
  - Blocker: Bot changes request remains outstanding
  - … 1 more blocker(s)
- [dashpay/rust-dashcore#136 feat: update integration test script for Dash compatibility](https://github.com/dashpay/rust-dashcore/pull/136) — 1 unresolved (1 CodeRabbit) · 373 days stale · 🔴 CI failing · 🐢 targets v0.40-dev, untouched 373 days · areas: fallback
  - Top thread: "_🛠️ Refactor suggestion_" — 373 days old
- [dashpay/grovedb#953 feat!: declare per operation, through DontCheck op twins, whether the displaced value may be a backward-reference participant](https://github.com/dashpay/grovedb/pull/953) — 2 unresolved (2 CodeRabbit) · 0 days stale · 🔴 CI failing · areas: grovedb · Policy: waiting-bots
  - Top thread: "_🎯 Functional Correctness_ \| _🟡 Minor_ \| _⚡ Quick win_" — 0 days old
  - Blocker: thepastaclaw final review missing for current head
  - Blocker: Bot review threads remain unresolved
- [dashpay/platform#4634 feat(platform)!: halve contested name fee in protocol 14](https://github.com/dashpay/platform/pull/4634) — 1 unresolved (1 bot) · 3 days stale · 🔴 CI failing · areas: rs-drive-abci, fallback · Policy: waiting-bots
  - Top thread: "🟡 Suggestion: Document the deliberate mismatch between FEE_VERSION3 and fee-version lookup identity**" — 3 days old
  - Blocker: CodeRabbit completion missing for current head
  - Blocker: Bot review threads remain unresolved
- [dashpay/platform#4701 fix(proof-verification)!: require V1 GroveDB proof envelopes](https://github.com/dashpay/platform/pull/4701) — 1 unresolved (1 bot) · 0 days stale · 🔴 CI failing · areas: rs-drive, fallback · Policy: waiting-slot
  - Top thread: "🟡 Suggestion: Test envelope enforcement through public verification entry points**" — 0 days old
  - Blocker: Waiting for one of five author review slots
- [dashpay/platform#1834 feat(drive-abci)!: Statesync](https://github.com/dashpay/platform/pull/1834) — 📝 draft · ⏸ deferred · areas: rs-drive, rs-drive-abci, dpp, rust-sdk, dashmate, js-wasm-sdk, system-contracts, fallback · ⚠ ownership unresolved: dashmate
- [dashpay/platform#3021 feat(platform): better contract state transitions](https://github.com/dashpay/platform/pull/3021) — 📝 draft · ⏸ deferred · areas: rs-drive, rs-drive-abci, dpp, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#4519 feat(platform-wallet): rotate a masternode's keys into the wallet — ProUpRegTx orchestration, FFI, Swift](https://github.com/dashpay/platform/pull/4519) — areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk · Policy: waiting-bots
  - Blocker: thepastaclaw final review missing for current head
  - Blocker: CodeRabbit completion missing for current head
  - Blocker: Bot changes request remains outstanding
- [dashpay/platform#4581 feat(drive): time-range index TTL — O(1) flat-drop drainage and ephemeral-bytes fees](https://github.com/dashpay/platform/pull/4581) — ✋ changes requested · areas: rs-drive, rs-drive-abci, dpp, fallback · Policy: waiting-bots
  - Blocker: CodeRabbit completion missing for current head
- [dashpay/platform#4625 fix(platform)!: decode external input with untrusted bincode decoders](https://github.com/dashpay/platform/pull/4625) — 🔴 CI failing · 🐢 targets build/grovedb-6-0-0-bincode-2-1-0 · areas: rs-drive, rs-drive-abci, dpp, rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, rust-sdk, rust-sdk-ffi, fallback
- [dashpay/platform#4635 build(platform)!: adopt GroveDB 6.0 with automatic backward references and grovedb-bincode 2.1.0](https://github.com/dashpay/platform/pull/4635) — 🔴 CI failing · areas: rs-drive, rs-drive-abci, dpp, rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, rust-sdk, rust-sdk-ffi, js-wasm-sdk, fallback · Policy: waiting-bots
  - Blocker: thepastaclaw final review missing for current head
  - Blocker: CodeRabbit completion missing for current head
- [dashpay/platform#4708 feat(platform)!: add ShieldFromIdentity state transition (identity balance to shielded pool)](https://github.com/dashpay/platform/pull/4708) — areas: rs-drive, rs-drive-abci, dpp, rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, rust-sdk, kotlin-sdk, fallback · Policy: waiting-slot
  - Blocker: Waiting for one of five author review slots
- [dashpay/platform#4711 feat(platform)!: add IdentityTopUpFromShieldedPool state transition (shielded pool to identity)](https://github.com/dashpay/platform/pull/4711) — 🔴 CI failing · 🐢 targets claude/identity-shielded-pool-transition-847713 · areas: rs-drive, rs-drive-abci, dpp, rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, rust-sdk, kotlin-sdk, fallback

<a id="quantumexplorer-needs-action"></a>
#### Needs action (6)
- [dashpay/rust-dashcore#860 feat(dash-spv): expose per-peer connection stats (ping RTT, bytes received)](https://github.com/dashpay/rust-dashcore/pull/860) — 3 unresolved (3 CodeRabbit) · 64 days stale · ✋ changes requested · areas: dash-spv, fallback · ⚠ ownership unresolved: dash-spv · Policy: configuration-error
  - Top thread: "_📐 Maintainability & Code Quality_ \| _🟡 Minor_ \| _⚡ Quick win_" — 64 days old
  - Blocker: Unresolved identities in dash-spv
- [dashpay/platform#3417 feat(swift-sdk): use SPV-synced quorums for Platform proof verification](https://github.com/dashpay/platform/pull/3417) — 5 unresolved (5 bot) · 4 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, rust-sdk, rust-sdk-ffi, fallback · Policy: waiting-bots
  - Top thread: "🟡 Suggestion: Exercise the production runtime bridge across Tokio runtime flavors**" — 4 days old
  - Blocker: thepastaclaw final review missing for current head
  - Blocker: CodeRabbit completion missing for current head
  - Blocker: Bot changes request remains outstanding
  - … 1 more blocker(s)
- [dashpay/grovedb#953 feat!: declare per operation, through DontCheck op twins, whether the displaced value may be a backward-reference participant](https://github.com/dashpay/grovedb/pull/953) — 2 unresolved (2 CodeRabbit) · 0 days stale · 🔴 CI failing · areas: grovedb · Policy: waiting-bots
  - Top thread: "_🎯 Functional Correctness_ \| _🟡 Minor_ \| _⚡ Quick win_" — 0 days old
  - Blocker: thepastaclaw final review missing for current head
  - Blocker: Bot review threads remain unresolved
- [dashpay/platform#4634 feat(platform)!: halve contested name fee in protocol 14](https://github.com/dashpay/platform/pull/4634) — 1 unresolved (1 bot) · 3 days stale · 🔴 CI failing · areas: rs-drive-abci, fallback · Policy: waiting-bots
  - Top thread: "🟡 Suggestion: Document the deliberate mismatch between FEE_VERSION3 and fee-version lookup identity**" — 3 days old
  - Blocker: CodeRabbit completion missing for current head
  - Blocker: Bot review threads remain unresolved
- [dashpay/platform#4701 fix(proof-verification)!: require V1 GroveDB proof envelopes](https://github.com/dashpay/platform/pull/4701) — 1 unresolved (1 bot) · 0 days stale · 🔴 CI failing · areas: rs-drive, fallback · Policy: waiting-slot
  - Top thread: "🟡 Suggestion: Test envelope enforcement through public verification entry points**" — 0 days old
  - Blocker: Waiting for one of five author review slots
- [dashpay/platform#4581 feat(drive): time-range index TTL — O(1) flat-drop drainage and ephemeral-bytes fees](https://github.com/dashpay/platform/pull/4581) — ✋ changes requested · areas: rs-drive, rs-drive-abci, dpp, fallback · Policy: waiting-bots
  - Blocker: CodeRabbit completion missing for current head

<a id="quantumexplorer-unresolved-comments"></a>
#### Unresolved Comments (5)
- [dashpay/rust-dashcore#860 feat(dash-spv): expose per-peer connection stats (ping RTT, bytes received)](https://github.com/dashpay/rust-dashcore/pull/860) — 3 unresolved (3 CodeRabbit) · 64 days stale · ✋ changes requested · areas: dash-spv, fallback · ⚠ ownership unresolved: dash-spv · Policy: configuration-error
  - Top thread: "_📐 Maintainability & Code Quality_ \| _🟡 Minor_ \| _⚡ Quick win_" — 64 days old
  - Blocker: Unresolved identities in dash-spv
- [dashpay/platform#3417 feat(swift-sdk): use SPV-synced quorums for Platform proof verification](https://github.com/dashpay/platform/pull/3417) — 5 unresolved (5 bot) · 4 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, rust-sdk, rust-sdk-ffi, fallback · Policy: waiting-bots
  - Top thread: "🟡 Suggestion: Exercise the production runtime bridge across Tokio runtime flavors**" — 4 days old
  - Blocker: thepastaclaw final review missing for current head
  - Blocker: CodeRabbit completion missing for current head
  - Blocker: Bot changes request remains outstanding
  - … 1 more blocker(s)
- [dashpay/grovedb#953 feat!: declare per operation, through DontCheck op twins, whether the displaced value may be a backward-reference participant](https://github.com/dashpay/grovedb/pull/953) — 2 unresolved (2 CodeRabbit) · 0 days stale · 🔴 CI failing · areas: grovedb · Policy: waiting-bots
  - Top thread: "_🎯 Functional Correctness_ \| _🟡 Minor_ \| _⚡ Quick win_" — 0 days old
  - Blocker: thepastaclaw final review missing for current head
  - Blocker: Bot review threads remain unresolved
- [dashpay/platform#4634 feat(platform)!: halve contested name fee in protocol 14](https://github.com/dashpay/platform/pull/4634) — 1 unresolved (1 bot) · 3 days stale · 🔴 CI failing · areas: rs-drive-abci, fallback · Policy: waiting-bots
  - Top thread: "🟡 Suggestion: Document the deliberate mismatch between FEE_VERSION3 and fee-version lookup identity**" — 3 days old
  - Blocker: CodeRabbit completion missing for current head
  - Blocker: Bot review threads remain unresolved
- [dashpay/platform#4701 fix(proof-verification)!: require V1 GroveDB proof envelopes](https://github.com/dashpay/platform/pull/4701) — 1 unresolved (1 bot) · 0 days stale · 🔴 CI failing · areas: rs-drive, fallback · Policy: waiting-slot
  - Top thread: "🟡 Suggestion: Test envelope enforcement through public verification entry points**" — 0 days old
  - Blocker: Waiting for one of five author review slots

<a id="quantumexplorer-changes-requested"></a>
#### Changes Requested (1)
- [dashpay/platform#4581 feat(drive): time-range index TTL — O(1) flat-drop drainage and ephemeral-bytes fees](https://github.com/dashpay/platform/pull/4581) — ✋ changes requested · areas: rs-drive, rs-drive-abci, dpp, fallback · Policy: waiting-bots
  - Blocker: CodeRabbit completion missing for current head

<a id="quantumexplorer-ci-failing"></a>
#### CI Failing (1)
- [dashpay/platform#4635 build(platform)!: adopt GroveDB 6.0 with automatic backward references and grovedb-bincode 2.1.0](https://github.com/dashpay/platform/pull/4635) — 🔴 CI failing · areas: rs-drive, rs-drive-abci, dpp, rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, rust-sdk, rust-sdk-ffi, js-wasm-sdk, fallback · Policy: waiting-bots
  - Blocker: thepastaclaw final review missing for current head
  - Blocker: CodeRabbit completion missing for current head

<a id="quantumexplorer-deferred"></a>
#### Deferred (2)
- [dashpay/platform#1834 feat(drive-abci)!: Statesync](https://github.com/dashpay/platform/pull/1834) — 📝 draft · ⏸ deferred · areas: rs-drive, rs-drive-abci, dpp, rust-sdk, dashmate, js-wasm-sdk, system-contracts, fallback · ⚠ ownership unresolved: dashmate
- [dashpay/platform#3021 feat(platform): better contract state transitions](https://github.com/dashpay/platform/pull/3021) — 📝 draft · ⏸ deferred · areas: rs-drive, rs-drive-abci, dpp, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot

<a id="quantumexplorer-draft"></a>
#### Draft (2)
- [dashpay/platform#4272 feat(dpp)!: dashpay payment detection keys and stealth derivation](https://github.com/dashpay/platform/pull/4272) — 9 unresolved (9 bot) · 40 days stale · ⚠ merge conflict · 📝 draft · areas: rs-drive, rs-drive-abci, dpp, rs-platform-wallet, swift-sdk, system-contracts, kotlin-sdk, fallback · Policy: draft
  - Top thread: "🔴 Blocking: Require compressed 33-byte encodings for payment keys**" — 40 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#622 fix: apply BIP-69 sorting to asset lock credit outputs](https://github.com/dashpay/rust-dashcore/pull/622) — 1 unresolved (1 CodeRabbit) · 163 days stale · ⚠ merge conflict · ✋ changes requested · 📝 draft · areas: key-wallet · ⚠ ownership unresolved: key-wallet · Policy: draft
  - Top thread: "_⚠️ Potential issue_ \| _🟠 Major_" — 163 days old
  - Blocker: Draft PR does not occupy a review slot

<a id="quantumexplorer-stale"></a>
#### Stale (3)
- [dashpay/rust-dashcore#136 feat: update integration test script for Dash compatibility](https://github.com/dashpay/rust-dashcore/pull/136) — 1 unresolved (1 CodeRabbit) · 373 days stale · 🔴 CI failing · 🐢 targets v0.40-dev, untouched 373 days · areas: fallback
  - Top thread: "_🛠️ Refactor suggestion_" — 373 days old
- [dashpay/platform#4625 fix(platform)!: decode external input with untrusted bincode decoders](https://github.com/dashpay/platform/pull/4625) — 🔴 CI failing · 🐢 targets build/grovedb-6-0-0-bincode-2-1-0 · areas: rs-drive, rs-drive-abci, dpp, rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, rust-sdk, rust-sdk-ffi, fallback
- [dashpay/platform#4711 feat(platform)!: add IdentityTopUpFromShieldedPool state transition (shielded pool to identity)](https://github.com/dashpay/platform/pull/4711) — 🔴 CI failing · 🐢 targets claude/identity-shielded-pool-transition-847713 · areas: rs-drive, rs-drive-abci, dpp, rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, rust-sdk, kotlin-sdk, fallback

<a id="quantumexplorer-clean"></a>
#### Clean (2)
- [dashpay/platform#4519 feat(platform-wallet): rotate a masternode's keys into the wallet — ProUpRegTx orchestration, FFI, Swift](https://github.com/dashpay/platform/pull/4519) — areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk · Policy: waiting-bots
  - Blocker: thepastaclaw final review missing for current head
  - Blocker: CodeRabbit completion missing for current head
  - Blocker: Bot changes request remains outstanding
- [dashpay/platform#4708 feat(platform)!: add ShieldFromIdentity state transition (identity balance to shielded pool)](https://github.com/dashpay/platform/pull/4708) — areas: rs-drive, rs-drive-abci, dpp, rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, rust-sdk, kotlin-sdk, fallback · Policy: waiting-slot
  - Blocker: Waiting for one of five author review slots

<a id="quantumexplorer-ready-for-review"></a>
#### Ready for Review (6)
- [dashpay/platform#4454 test(swift-sdk): pin non-English BIP-39 mnemonic support against the fixed FFI](https://github.com/dashpay/platform/pull/4454) — by @PastaPastaPasta · areas: swift-sdk · Policy: waiting-bots
  - Blocker: CodeRabbit completion missing for current head
- [dashpay/platform#4576 docs(wasm-sdk): clarify aggregate groupBy behavior](https://github.com/dashpay/platform/pull/4576) — by @thephez · areas: js-wasm-sdk · Policy: waiting-bots
  - Blocker: CodeRabbit completion missing for current head
- [dashpay/platform#4618 refactor(drive-abci): share the v1 document-query wire decoders](https://github.com/dashpay/platform/pull/4618) — by @PastaPastaPasta · areas: rs-drive-abci, fallback · Policy: waiting-slot
  - Blocker: Waiting for one of five author review slots
- [dashpay/rust-dashcore#993 fix(dash-spv): wire up masternode persistence and shrink it 67x](https://github.com/dashpay/rust-dashcore/pull/993) — by @ZocoLini · areas: dash-spv, fallback · ⚠ ownership unresolved: dash-spv · Policy: configuration-error
  - Blocker: Unresolved identities in dash-spv
- [dashpay/rust-dashcore#1002 fix(dash-spv): make backward coverage durable by rewinding synced_height instead of sweeping in memory](https://github.com/dashpay/rust-dashcore/pull/1002) — by @romchornyi · areas: dash-spv, key-wallet-manager, fallback · ⚠ ownership unresolved: dash-spv, key-wallet-manager · Policy: configuration-error
  - Blocker: Unresolved identities in dash-spv
- [dashpay/rust-dashcore#1009 fix(key-wallet): consult every fund-bearing account for provider transactions](https://github.com/dashpay/rust-dashcore/pull/1009) — by @ZocoLini · areas: key-wallet · ⚠ ownership unresolved: key-wallet · Policy: configuration-error
  - Blocker: Unresolved identities in key-wallet

<a id="romchornyi"></a>
### @romchornyi
<a id="romchornyi-open"></a>
#### Open (5)
- [dashpay/platform#4595 fix(swift-sdk): make a changeset round linear without giving up atomicity](https://github.com/dashpay/platform/pull/4595) — 2 unresolved (2 bot) · 5 days stale · ⚠ merge conflict · ✋ changes requested · areas: swift-sdk · Policy: waiting-bots
  - Top thread: "🟡 Suggestion: Add round-level regression coverage for registry bookkeeping**" — 5 days old
  - Blocker: thepastaclaw final review missing for current head
  - Blocker: Bot changes request remains outstanding
  - Blocker: Bot review threads remain unresolved
- [dashpay/platform#4651 fix(platform-wallet): resolve a swept sent payment's verdict on the round that swept it](https://github.com/dashpay/platform/pull/4651) — 2 unresolved (2 bot) · 2 days stale · areas: rs-platform-wallet, wallet-storage, fallback · Policy: waiting-bots
  - Top thread: "🟡 Suggestion: Exercise payment verdicts through the actual adapter drain**" — 2 days old
  - Blocker: Bot review threads remain unresolved
- [dashpay/platform#4659 fix(platform-wallet): give an unconfirmed outgoing send an owner across a restart](https://github.com/dashpay/platform/pull/4659) — 2 unresolved (2 bot) · 1 days stale · ✋ changes requested · 🔴 CI failing · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk, fallback · Policy: waiting-bots
  - Top thread: "🟡 Suggestion: Add deterministic coverage for the resend lifecycle**" — 1 days old
  - Blocker: thepastaclaw final review missing for current head
  - Blocker: CodeRabbit completion missing for current head
  - Blocker: Bot changes request remains outstanding
  - … 1 more blocker(s)
- [dashpay/platform#4627 chore: bump rust-dashcore to dev head (#1000 merged: a known transaction is not announced new twice)](https://github.com/dashpay/platform/pull/4627) — ⚠ merge conflict · areas: fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed c1d5218f12c8c3caa512326c5ed5238667249b09 after bot completion
- [dashpay/rust-dashcore#1002 fix(dash-spv): make backward coverage durable by rewinding synced_height instead of sweeping in memory](https://github.com/dashpay/rust-dashcore/pull/1002) — areas: dash-spv, key-wallet-manager, fallback · ⚠ ownership unresolved: dash-spv, key-wallet-manager · Policy: configuration-error
  - Blocker: Unresolved identities in dash-spv

<a id="romchornyi-needs-action"></a>
#### Needs action (4)
- [dashpay/platform#4595 fix(swift-sdk): make a changeset round linear without giving up atomicity](https://github.com/dashpay/platform/pull/4595) — 2 unresolved (2 bot) · 5 days stale · ⚠ merge conflict · ✋ changes requested · areas: swift-sdk · Policy: waiting-bots
  - Top thread: "🟡 Suggestion: Add round-level regression coverage for registry bookkeeping**" — 5 days old
  - Blocker: thepastaclaw final review missing for current head
  - Blocker: Bot changes request remains outstanding
  - Blocker: Bot review threads remain unresolved
- [dashpay/platform#4651 fix(platform-wallet): resolve a swept sent payment's verdict on the round that swept it](https://github.com/dashpay/platform/pull/4651) — 2 unresolved (2 bot) · 2 days stale · areas: rs-platform-wallet, wallet-storage, fallback · Policy: waiting-bots
  - Top thread: "🟡 Suggestion: Exercise payment verdicts through the actual adapter drain**" — 2 days old
  - Blocker: Bot review threads remain unresolved
- [dashpay/platform#4659 fix(platform-wallet): give an unconfirmed outgoing send an owner across a restart](https://github.com/dashpay/platform/pull/4659) — 2 unresolved (2 bot) · 1 days stale · ✋ changes requested · 🔴 CI failing · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk, fallback · Policy: waiting-bots
  - Top thread: "🟡 Suggestion: Add deterministic coverage for the resend lifecycle**" — 1 days old
  - Blocker: thepastaclaw final review missing for current head
  - Blocker: CodeRabbit completion missing for current head
  - Blocker: Bot changes request remains outstanding
  - … 1 more blocker(s)
- [dashpay/platform#4627 chore: bump rust-dashcore to dev head (#1000 merged: a known transaction is not announced new twice)](https://github.com/dashpay/platform/pull/4627) — ⚠ merge conflict · areas: fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed c1d5218f12c8c3caa512326c5ed5238667249b09 after bot completion

<a id="romchornyi-unresolved-comments"></a>
#### Unresolved Comments (3)
- [dashpay/platform#4595 fix(swift-sdk): make a changeset round linear without giving up atomicity](https://github.com/dashpay/platform/pull/4595) — 2 unresolved (2 bot) · 5 days stale · ⚠ merge conflict · ✋ changes requested · areas: swift-sdk · Policy: waiting-bots
  - Top thread: "🟡 Suggestion: Add round-level regression coverage for registry bookkeeping**" — 5 days old
  - Blocker: thepastaclaw final review missing for current head
  - Blocker: Bot changes request remains outstanding
  - Blocker: Bot review threads remain unresolved
- [dashpay/platform#4651 fix(platform-wallet): resolve a swept sent payment's verdict on the round that swept it](https://github.com/dashpay/platform/pull/4651) — 2 unresolved (2 bot) · 2 days stale · areas: rs-platform-wallet, wallet-storage, fallback · Policy: waiting-bots
  - Top thread: "🟡 Suggestion: Exercise payment verdicts through the actual adapter drain**" — 2 days old
  - Blocker: Bot review threads remain unresolved
- [dashpay/platform#4659 fix(platform-wallet): give an unconfirmed outgoing send an owner across a restart](https://github.com/dashpay/platform/pull/4659) — 2 unresolved (2 bot) · 1 days stale · ✋ changes requested · 🔴 CI failing · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk, fallback · Policy: waiting-bots
  - Top thread: "🟡 Suggestion: Add deterministic coverage for the resend lifecycle**" — 1 days old
  - Blocker: thepastaclaw final review missing for current head
  - Blocker: CodeRabbit completion missing for current head
  - Blocker: Bot changes request remains outstanding
  - … 1 more blocker(s)

<a id="romchornyi-clean"></a>
#### Clean (2)
- [dashpay/platform#4627 chore: bump rust-dashcore to dev head (#1000 merged: a known transaction is not announced new twice)](https://github.com/dashpay/platform/pull/4627) — ⚠ merge conflict · areas: fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed c1d5218f12c8c3caa512326c5ed5238667249b09 after bot completion
- [dashpay/rust-dashcore#1002 fix(dash-spv): make backward coverage durable by rewinding synced_height instead of sweeping in memory](https://github.com/dashpay/rust-dashcore/pull/1002) — areas: dash-spv, key-wallet-manager, fallback · ⚠ ownership unresolved: dash-spv, key-wallet-manager · Policy: configuration-error
  - Blocker: Unresolved identities in dash-spv

<a id="romchornyi-ready-for-review"></a>
#### Ready for Review (4)
- [dashpay/platform#4454 test(swift-sdk): pin non-English BIP-39 mnemonic support against the fixed FFI](https://github.com/dashpay/platform/pull/4454) — by @PastaPastaPasta · areas: swift-sdk · Policy: waiting-bots
  - Blocker: CodeRabbit completion missing for current head
- [dashpay/platform#4519 feat(platform-wallet): rotate a masternode's keys into the wallet — ProUpRegTx orchestration, FFI, Swift](https://github.com/dashpay/platform/pull/4519) — by @QuantumExplorer · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk · Policy: waiting-bots
  - Blocker: thepastaclaw final review missing for current head
  - Blocker: CodeRabbit completion missing for current head
  - Blocker: Bot changes request remains outstanding
- [dashpay/platform#4708 feat(platform)!: add ShieldFromIdentity state transition (identity balance to shielded pool)](https://github.com/dashpay/platform/pull/4708) — by @QuantumExplorer · areas: rs-drive, rs-drive-abci, dpp, rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, rust-sdk, kotlin-sdk, fallback · Policy: waiting-slot
  - Blocker: Waiting for one of five author review slots
- [dashpay/platform#4709 fix(swift-sdk)!: expose reservation-aware local shielded balance snapshots](https://github.com/dashpay/platform/pull/4709) — by @llbartekll · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk · Policy: waiting-bots
  - Blocker: thepastaclaw final review missing for current head

<a id="llbartekll"></a>
### @llbartekll
<a id="llbartekll-open"></a>
#### Open (6)
- [dashpay/platform#4170 feat(swift-sdk): make wallet deletion asynchronous](https://github.com/dashpay/platform/pull/4170) — 2 unresolved (2 bot) · 54 days stale · ⚠ merge conflict · ✋ changes requested · 📝 draft · areas: swift-sdk · Policy: draft
  - Top thread: "🔴 Blocking: Quiesce child FFI handles before awaiting deletion**" — 54 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#3560 test(swift-sdk): add testnet identity-discovery UI test](https://github.com/dashpay/platform/pull/3560) — 1 unresolved (1 bot) · 103 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · areas: swift-sdk, fallback · Policy: waiting-bots
  - Top thread: "🔴 Blocking: Carried-forward prior finding (STILL VALID): Docker-setup toggle leaves the cached regtest wallet manager bo…" — 103 days old
  - Blocker: thepastaclaw final review missing for current head
  - Blocker: CodeRabbit completion missing for current head
  - Blocker: Bot changes request remains outstanding
  - … 1 more blocker(s)
- [dashpay/platform#3694 fix(swift-example-app): unhide Create Identity submit button and auto-dismiss sheet on success](https://github.com/dashpay/platform/pull/3694) — 1 unresolved (1 bot) · 103 days stale · ✋ changes requested · areas: swift-sdk · Policy: waiting-bots
  - Top thread: "🟡 Suggestion: Sub-0.0001 DASH balances still collapse the prefill to an unparseable "0"**" — 103 days old
  - Blocker: thepastaclaw final review missing for current head
  - Blocker: CodeRabbit completion missing for current head
  - Blocker: Bot review threads remain unresolved
- [dashpay/platform#4709 fix(swift-sdk)!: expose reservation-aware local shielded balance snapshots](https://github.com/dashpay/platform/pull/4709) — areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk · Policy: waiting-bots
  - Blocker: thepastaclaw final review missing for current head
- [dashpay/rust-dashcore#819 feat(key-wallet): CoinJoin sweep + gap-limit recovery; TransactionBuilder drain mode](https://github.com/dashpay/rust-dashcore/pull/819) — ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · areas: key-wallet · ⚠ ownership unresolved: key-wallet · Policy: configuration-error
  - Blocker: Unresolved identities in key-wallet
- [dashpay/rust-dashcore#910 fix(dash-spv): re-anchor incompatible shared storage](https://github.com/dashpay/rust-dashcore/pull/910) — 📝 draft · areas: dash-spv · ⚠ ownership unresolved: dash-spv · Policy: draft
  - Blocker: Draft PR does not occupy a review slot

<a id="llbartekll-needs-action"></a>
#### Needs action (3)
- [dashpay/platform#3560 test(swift-sdk): add testnet identity-discovery UI test](https://github.com/dashpay/platform/pull/3560) — 1 unresolved (1 bot) · 103 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · areas: swift-sdk, fallback · Policy: waiting-bots
  - Top thread: "🔴 Blocking: Carried-forward prior finding (STILL VALID): Docker-setup toggle leaves the cached regtest wallet manager bo…" — 103 days old
  - Blocker: thepastaclaw final review missing for current head
  - Blocker: CodeRabbit completion missing for current head
  - Blocker: Bot changes request remains outstanding
  - … 1 more blocker(s)
- [dashpay/platform#3694 fix(swift-example-app): unhide Create Identity submit button and auto-dismiss sheet on success](https://github.com/dashpay/platform/pull/3694) — 1 unresolved (1 bot) · 103 days stale · ✋ changes requested · areas: swift-sdk · Policy: waiting-bots
  - Top thread: "🟡 Suggestion: Sub-0.0001 DASH balances still collapse the prefill to an unparseable "0"**" — 103 days old
  - Blocker: thepastaclaw final review missing for current head
  - Blocker: CodeRabbit completion missing for current head
  - Blocker: Bot review threads remain unresolved
- [dashpay/rust-dashcore#819 feat(key-wallet): CoinJoin sweep + gap-limit recovery; TransactionBuilder drain mode](https://github.com/dashpay/rust-dashcore/pull/819) — ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · areas: key-wallet · ⚠ ownership unresolved: key-wallet · Policy: configuration-error
  - Blocker: Unresolved identities in key-wallet

<a id="llbartekll-unresolved-comments"></a>
#### Unresolved Comments (2)
- [dashpay/platform#3560 test(swift-sdk): add testnet identity-discovery UI test](https://github.com/dashpay/platform/pull/3560) — 1 unresolved (1 bot) · 103 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · areas: swift-sdk, fallback · Policy: waiting-bots
  - Top thread: "🔴 Blocking: Carried-forward prior finding (STILL VALID): Docker-setup toggle leaves the cached regtest wallet manager bo…" — 103 days old
  - Blocker: thepastaclaw final review missing for current head
  - Blocker: CodeRabbit completion missing for current head
  - Blocker: Bot changes request remains outstanding
  - … 1 more blocker(s)
- [dashpay/platform#3694 fix(swift-example-app): unhide Create Identity submit button and auto-dismiss sheet on success](https://github.com/dashpay/platform/pull/3694) — 1 unresolved (1 bot) · 103 days stale · ✋ changes requested · areas: swift-sdk · Policy: waiting-bots
  - Top thread: "🟡 Suggestion: Sub-0.0001 DASH balances still collapse the prefill to an unparseable "0"**" — 103 days old
  - Blocker: thepastaclaw final review missing for current head
  - Blocker: CodeRabbit completion missing for current head
  - Blocker: Bot review threads remain unresolved

<a id="llbartekll-changes-requested"></a>
#### Changes Requested (1)
- [dashpay/rust-dashcore#819 feat(key-wallet): CoinJoin sweep + gap-limit recovery; TransactionBuilder drain mode](https://github.com/dashpay/rust-dashcore/pull/819) — ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · areas: key-wallet · ⚠ ownership unresolved: key-wallet · Policy: configuration-error
  - Blocker: Unresolved identities in key-wallet

<a id="llbartekll-draft"></a>
#### Draft (2)
- [dashpay/platform#4170 feat(swift-sdk): make wallet deletion asynchronous](https://github.com/dashpay/platform/pull/4170) — 2 unresolved (2 bot) · 54 days stale · ⚠ merge conflict · ✋ changes requested · 📝 draft · areas: swift-sdk · Policy: draft
  - Top thread: "🔴 Blocking: Quiesce child FFI handles before awaiting deletion**" — 54 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#910 fix(dash-spv): re-anchor incompatible shared storage](https://github.com/dashpay/rust-dashcore/pull/910) — 📝 draft · areas: dash-spv · ⚠ ownership unresolved: dash-spv · Policy: draft
  - Blocker: Draft PR does not occupy a review slot

<a id="llbartekll-clean"></a>
#### Clean (1)
- [dashpay/platform#4709 fix(swift-sdk)!: expose reservation-aware local shielded balance snapshots](https://github.com/dashpay/platform/pull/4709) — areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk · Policy: waiting-bots
  - Blocker: thepastaclaw final review missing for current head

<a id="llbartekll-ready-for-review"></a>
#### Ready for Review (3)
- [dashpay/platform#4454 test(swift-sdk): pin non-English BIP-39 mnemonic support against the fixed FFI](https://github.com/dashpay/platform/pull/4454) — by @PastaPastaPasta · areas: swift-sdk · Policy: waiting-bots
  - Blocker: CodeRabbit completion missing for current head
- [dashpay/platform#4519 feat(platform-wallet): rotate a masternode's keys into the wallet — ProUpRegTx orchestration, FFI, Swift](https://github.com/dashpay/platform/pull/4519) — by @QuantumExplorer · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk · Policy: waiting-bots
  - Blocker: thepastaclaw final review missing for current head
  - Blocker: CodeRabbit completion missing for current head
  - Blocker: Bot changes request remains outstanding
- [dashpay/platform#4708 feat(platform)!: add ShieldFromIdentity state transition (identity balance to shielded pool)](https://github.com/dashpay/platform/pull/4708) — by @QuantumExplorer · areas: rs-drive, rs-drive-abci, dpp, rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, rust-sdk, kotlin-sdk, fallback · Policy: waiting-slot
  - Blocker: Waiting for one of five author review slots

<a id="hashengineering"></a>
### @HashEngineering
<a id="hashengineering-open"></a>
#### Open (4)
- [dashpay/platform#4643 fix(sdk)!: keep Keystore's unlocked-device gate from bricking wallets and signing on defective OEM builds](https://github.com/dashpay/platform/pull/4643) — 1 unresolved (1 CodeRabbit) · 1 days stale · areas: kotlin-sdk · Policy: waiting-bots
  - Top thread: "_🎯 Functional Correctness_ \| _🟠 Major_ \| _🏗️ Heavy lift_" — 1 days old
  - Blocker: Bot changes request remains outstanding
  - Blocker: Bot review threads remain unresolved
- [dashpay/platform#4658 feat(kotlin-sdk): bind the ordered wallet bring-up (startWalletSubsystems) over JNI](https://github.com/dashpay/platform/pull/4658) — 1 unresolved (1 CodeRabbit) · 0 days stale · areas: kotlin-sdk, fallback · Policy: waiting-bots
  - Top thread: "_📐 Maintainability & Code Quality_ \| _🟡 Minor_ \| _⚡ Quick win_" — 0 days old
  - Blocker: thepastaclaw final review missing for current head
  - Blocker: Bot review threads remain unresolved
- [dashpay/platform#4439 fix(kotlin-sdk): reconcile the TXO store against the engine and repair restored address pools](https://github.com/dashpay/platform/pull/4439) — ⚠ merge conflict · 📝 draft · areas: rs-platform-wallet, rs-platform-wallet-ffi, kotlin-sdk, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#979 fix(key-wallet, dash-spv): re-emit late knowledge — record corrections, durable rescans, address-pool repair](https://github.com/dashpay/rust-dashcore/pull/979) — 📝 draft · areas: dash-spv, key-wallet · ⚠ ownership unresolved: dash-spv, key-wallet · Policy: draft
  - Blocker: Draft PR does not occupy a review slot

<a id="hashengineering-needs-action"></a>
#### Needs action (2)
- [dashpay/platform#4643 fix(sdk)!: keep Keystore's unlocked-device gate from bricking wallets and signing on defective OEM builds](https://github.com/dashpay/platform/pull/4643) — 1 unresolved (1 CodeRabbit) · 1 days stale · areas: kotlin-sdk · Policy: waiting-bots
  - Top thread: "_🎯 Functional Correctness_ \| _🟠 Major_ \| _🏗️ Heavy lift_" — 1 days old
  - Blocker: Bot changes request remains outstanding
  - Blocker: Bot review threads remain unresolved
- [dashpay/platform#4658 feat(kotlin-sdk): bind the ordered wallet bring-up (startWalletSubsystems) over JNI](https://github.com/dashpay/platform/pull/4658) — 1 unresolved (1 CodeRabbit) · 0 days stale · areas: kotlin-sdk, fallback · Policy: waiting-bots
  - Top thread: "_📐 Maintainability & Code Quality_ \| _🟡 Minor_ \| _⚡ Quick win_" — 0 days old
  - Blocker: thepastaclaw final review missing for current head
  - Blocker: Bot review threads remain unresolved

<a id="hashengineering-unresolved-comments"></a>
#### Unresolved Comments (2)
- [dashpay/platform#4643 fix(sdk)!: keep Keystore's unlocked-device gate from bricking wallets and signing on defective OEM builds](https://github.com/dashpay/platform/pull/4643) — 1 unresolved (1 CodeRabbit) · 1 days stale · areas: kotlin-sdk · Policy: waiting-bots
  - Top thread: "_🎯 Functional Correctness_ \| _🟠 Major_ \| _🏗️ Heavy lift_" — 1 days old
  - Blocker: Bot changes request remains outstanding
  - Blocker: Bot review threads remain unresolved
- [dashpay/platform#4658 feat(kotlin-sdk): bind the ordered wallet bring-up (startWalletSubsystems) over JNI](https://github.com/dashpay/platform/pull/4658) — 1 unresolved (1 CodeRabbit) · 0 days stale · areas: kotlin-sdk, fallback · Policy: waiting-bots
  - Top thread: "_📐 Maintainability & Code Quality_ \| _🟡 Minor_ \| _⚡ Quick win_" — 0 days old
  - Blocker: thepastaclaw final review missing for current head
  - Blocker: Bot review threads remain unresolved

<a id="hashengineering-draft"></a>
#### Draft (2)
- [dashpay/platform#4439 fix(kotlin-sdk): reconcile the TXO store against the engine and repair restored address pools](https://github.com/dashpay/platform/pull/4439) — ⚠ merge conflict · 📝 draft · areas: rs-platform-wallet, rs-platform-wallet-ffi, kotlin-sdk, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#979 fix(key-wallet, dash-spv): re-emit late knowledge — record corrections, durable rescans, address-pool repair](https://github.com/dashpay/rust-dashcore/pull/979) — 📝 draft · areas: dash-spv, key-wallet · ⚠ ownership unresolved: dash-spv, key-wallet · Policy: draft
  - Blocker: Draft PR does not occupy a review slot

<a id="hashengineering-ready-for-review"></a>
#### Ready for Review (3)
- [dashpay/platform#4519 feat(platform-wallet): rotate a masternode's keys into the wallet — ProUpRegTx orchestration, FFI, Swift](https://github.com/dashpay/platform/pull/4519) — by @QuantumExplorer · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk · Policy: waiting-bots
  - Blocker: thepastaclaw final review missing for current head
  - Blocker: CodeRabbit completion missing for current head
  - Blocker: Bot changes request remains outstanding
- [dashpay/platform#4708 feat(platform)!: add ShieldFromIdentity state transition (identity balance to shielded pool)](https://github.com/dashpay/platform/pull/4708) — by @QuantumExplorer · areas: rs-drive, rs-drive-abci, dpp, rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, rust-sdk, kotlin-sdk, fallback · Policy: waiting-slot
  - Blocker: Waiting for one of five author review slots
- [dashpay/platform#4709 fix(swift-sdk)!: expose reservation-aware local shielded balance snapshots](https://github.com/dashpay/platform/pull/4709) — by @llbartekll · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk · Policy: waiting-bots
  - Blocker: thepastaclaw final review missing for current head

<a id="infraclaw-dash"></a>
### @infraclaw-dash
<a id="infraclaw-dash-open"></a>
#### Open (2)
- [dashpay/platform#3958 ci: add Platform testnet sync status reporting](https://github.com/dashpay/platform/pull/3958) — 2 unresolved (2 bot) · 80 days stale · ⚠ merge conflict · areas: fallback · Policy: waiting-bots
  - Top thread: "🟡 Suggestion: workflow_dispatch lets any repo writer forge a sync status**" — 80 days old
  - Blocker: thepastaclaw final review missing for current head
  - Blocker: CodeRabbit completion missing for current head
  - Blocker: Bot changes request remains outstanding
  - … 1 more blocker(s)
- [dashpay/platform#4702 ci: remove host privilege requirements from persistent Linux runners](https://github.com/dashpay/platform/pull/4702) — 1 unresolved (1 bot) · 0 days stale · 🔴 CI failing · areas: fallback · Policy: waiting-bots
  - Top thread: "🟡 Suggestion: Do not require self-hosted Debian packages in the shared action on hosted runners**" — 0 days old
  - Blocker: CodeRabbit completion missing for current head
  - Blocker: Bot review threads remain unresolved

<a id="infraclaw-dash-needs-action"></a>
#### Needs action (2)
- [dashpay/platform#3958 ci: add Platform testnet sync status reporting](https://github.com/dashpay/platform/pull/3958) — 2 unresolved (2 bot) · 80 days stale · ⚠ merge conflict · areas: fallback · Policy: waiting-bots
  - Top thread: "🟡 Suggestion: workflow_dispatch lets any repo writer forge a sync status**" — 80 days old
  - Blocker: thepastaclaw final review missing for current head
  - Blocker: CodeRabbit completion missing for current head
  - Blocker: Bot changes request remains outstanding
  - … 1 more blocker(s)
- [dashpay/platform#4702 ci: remove host privilege requirements from persistent Linux runners](https://github.com/dashpay/platform/pull/4702) — 1 unresolved (1 bot) · 0 days stale · 🔴 CI failing · areas: fallback · Policy: waiting-bots
  - Top thread: "🟡 Suggestion: Do not require self-hosted Debian packages in the shared action on hosted runners**" — 0 days old
  - Blocker: CodeRabbit completion missing for current head
  - Blocker: Bot review threads remain unresolved

<a id="infraclaw-dash-unresolved-comments"></a>
#### Unresolved Comments (2)
- [dashpay/platform#3958 ci: add Platform testnet sync status reporting](https://github.com/dashpay/platform/pull/3958) — 2 unresolved (2 bot) · 80 days stale · ⚠ merge conflict · areas: fallback · Policy: waiting-bots
  - Top thread: "🟡 Suggestion: workflow_dispatch lets any repo writer forge a sync status**" — 80 days old
  - Blocker: thepastaclaw final review missing for current head
  - Blocker: CodeRabbit completion missing for current head
  - Blocker: Bot changes request remains outstanding
  - … 1 more blocker(s)
- [dashpay/platform#4702 ci: remove host privilege requirements from persistent Linux runners](https://github.com/dashpay/platform/pull/4702) — 1 unresolved (1 bot) · 0 days stale · 🔴 CI failing · areas: fallback · Policy: waiting-bots
  - Top thread: "🟡 Suggestion: Do not require self-hosted Debian packages in the shared action on hosted runners**" — 0 days old
  - Blocker: CodeRabbit completion missing for current head
  - Blocker: Bot review threads remain unresolved

<a id="bfoss765"></a>
### @bfoss765
<a id="bfoss765-open"></a>
#### Open (2)
- [dashpay/platform#4313 feat(kotlin-sdk): one-time Orchard key shielded-invite API (inviter + claim)](https://github.com/dashpay/platform/pull/4313) — 2 unresolved (2 bot) · 12 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk, fallback · Policy: waiting-bots
  - Top thread: "🟡 Suggestion: Preserve claim-binding mismatch semantics across the FFI boundary**" — 12 days old
  - Blocker: thepastaclaw final review missing for current head
  - Blocker: CodeRabbit completion missing for current head
  - Blocker: Bot changes request remains outstanding
  - … 1 more blocker(s)
- [dashpay/platform#4312 feat(platform-wallet): multi-output shielded transfers + output-aware fee predictor](https://github.com/dashpay/platform/pull/4312) — 1 unresolved (1 bot) · 12 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · areas: dpp, rs-platform-wallet, rs-platform-wallet-ffi, kotlin-sdk, fallback · Policy: waiting-bots
  - Top thread: "🟡 Suggestion: Keep envelope-aware action counting behind the crate boundary**" — 12 days old
  - Blocker: CodeRabbit completion missing for current head
  - Blocker: Bot review threads remain unresolved

<a id="bfoss765-needs-action"></a>
#### Needs action (2)
- [dashpay/platform#4313 feat(kotlin-sdk): one-time Orchard key shielded-invite API (inviter + claim)](https://github.com/dashpay/platform/pull/4313) — 2 unresolved (2 bot) · 12 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk, fallback · Policy: waiting-bots
  - Top thread: "🟡 Suggestion: Preserve claim-binding mismatch semantics across the FFI boundary**" — 12 days old
  - Blocker: thepastaclaw final review missing for current head
  - Blocker: CodeRabbit completion missing for current head
  - Blocker: Bot changes request remains outstanding
  - … 1 more blocker(s)
- [dashpay/platform#4312 feat(platform-wallet): multi-output shielded transfers + output-aware fee predictor](https://github.com/dashpay/platform/pull/4312) — 1 unresolved (1 bot) · 12 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · areas: dpp, rs-platform-wallet, rs-platform-wallet-ffi, kotlin-sdk, fallback · Policy: waiting-bots
  - Top thread: "🟡 Suggestion: Keep envelope-aware action counting behind the crate boundary**" — 12 days old
  - Blocker: CodeRabbit completion missing for current head
  - Blocker: Bot review threads remain unresolved

<a id="bfoss765-unresolved-comments"></a>
#### Unresolved Comments (2)
- [dashpay/platform#4313 feat(kotlin-sdk): one-time Orchard key shielded-invite API (inviter + claim)](https://github.com/dashpay/platform/pull/4313) — 2 unresolved (2 bot) · 12 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk, fallback · Policy: waiting-bots
  - Top thread: "🟡 Suggestion: Preserve claim-binding mismatch semantics across the FFI boundary**" — 12 days old
  - Blocker: thepastaclaw final review missing for current head
  - Blocker: CodeRabbit completion missing for current head
  - Blocker: Bot changes request remains outstanding
  - … 1 more blocker(s)
- [dashpay/platform#4312 feat(platform-wallet): multi-output shielded transfers + output-aware fee predictor](https://github.com/dashpay/platform/pull/4312) — 1 unresolved (1 bot) · 12 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · areas: dpp, rs-platform-wallet, rs-platform-wallet-ffi, kotlin-sdk, fallback · Policy: waiting-bots
  - Top thread: "🟡 Suggestion: Keep envelope-aware action counting behind the crate boundary**" — 12 days old
  - Blocker: CodeRabbit completion missing for current head
  - Blocker: Bot review threads remain unresolved

<a id="zocolini"></a>
### @ZocoLini
<a id="zocolini-open"></a>
#### Open (14)
- [dashpay/rust-dashcore#496 Refactor/transaction builder](https://github.com/dashpay/rust-dashcore/pull/496) — 8 unresolved (8 CodeRabbit) · 191 days stale · ⚠ merge conflict · 📝 draft · areas: key-wallet, key-wallet-manager, fallback · ⚠ ownership unresolved: key-wallet, key-wallet-manager · Policy: draft
  - Top thread: "_⚠️ Potential issue_ \| _🔴 Critical_" — 191 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#375 refactor(dash-spv): checkpoint rewrite](https://github.com/dashpay/rust-dashcore/pull/375) — 2 unresolved (2 CodeRabbit) · 234 days stale · ⚠ merge conflict · areas: dash-spv, fallback · ⚠ ownership unresolved: dash-spv · Policy: configuration-error
  - Top thread: "_⚠️ Potential issue_ \| _🟡 Minor_" — 234 days old
  - Blocker: Unresolved identities in dash-spv
- [dashpay/rust-dashcore#370 Refactor: config builder to ensure validation](https://github.com/dashpay/rust-dashcore/pull/370) — ⚠ merge conflict · 📝 draft · areas: dash-spv, fallback · ⚠ ownership unresolved: dash-spv · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#374 Feat: config can be built from any json reader](https://github.com/dashpay/rust-dashcore/pull/374) — ⚠ merge conflict · 📝 draft · 🐢 targets refacor/config-builder, untouched 235 days · areas: dash-spv · ⚠ ownership unresolved: dash-spv
- [dashpay/rust-dashcore#498 chore(key-wallet): drop unnecesary codebase complexity removing the bincode dependency](https://github.com/dashpay/rust-dashcore/pull/498) — ⚠ merge conflict · 🔴 CI failing · 📝 draft · areas: key-wallet, key-wallet-manager, fallback · ⚠ ownership unresolved: key-wallet, key-wallet-manager · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#515 chore(dashcore): restrict ServiceFlags api](https://github.com/dashpay/rust-dashcore/pull/515) — 🔴 CI failing · 📝 draft · areas: dash-spv, fallback · ⚠ ownership unresolved: dash-spv · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#902 refactor(dash-spv): network manager refactor and sync pipelines optimized ](https://github.com/dashpay/rust-dashcore/pull/902) — ✋ changes requested · 🔴 CI failing · areas: dash-spv, fallback · ⚠ ownership unresolved: dash-spv · Policy: configuration-error
  - Blocker: Unresolved identities in dash-spv
- [dashpay/rust-dashcore#921 fix(dash-spv): sync backfill](https://github.com/dashpay/rust-dashcore/pull/921) — ⚠ merge conflict · 📝 draft · areas: dash-spv, fallback · ⚠ ownership unresolved: dash-spv · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#993 fix(dash-spv): wire up masternode persistence and shrink it 67x](https://github.com/dashpay/rust-dashcore/pull/993) — areas: dash-spv, fallback · ⚠ ownership unresolved: dash-spv · Policy: configuration-error
  - Blocker: Unresolved identities in dash-spv
- [dashpay/rust-dashcore#1009 fix(key-wallet): consult every fund-bearing account for provider transactions](https://github.com/dashpay/rust-dashcore/pull/1009) — areas: key-wallet · ⚠ ownership unresolved: key-wallet · Policy: configuration-error
  - Blocker: Unresolved identities in key-wallet
- [dashpay/rust-dashcore#1013 feat(dash-spv-bench): report the client's peak RSS](https://github.com/dashpay/rust-dashcore/pull/1013) — 🔴 CI failing · areas: fallback · Policy: waiting-slot
  - Blocker: Waiting for one of five author review slots
- [dashpay/rust-dashcore#1014 fix(key-wallet): prune observed spends during the initial sync](https://github.com/dashpay/rust-dashcore/pull/1014) — 🔴 CI failing · areas: dash-spv, key-wallet, key-wallet-manager · ⚠ ownership unresolved: dash-spv, key-wallet, key-wallet-manager · Policy: waiting-slot
  - Blocker: Waiting for one of five author review slots
- [dashpay/rust-dashcore#1015 fix(key-wallet): re-apply a spend whose coin was funded after it](https://github.com/dashpay/rust-dashcore/pull/1015) — 🔴 CI failing · areas: dash-spv, key-wallet, key-wallet-manager · ⚠ ownership unresolved: dash-spv, key-wallet, key-wallet-manager · Policy: waiting-slot
  - Blocker: Waiting for one of five author review slots
- [dashpay/rust-dashcore#1016 Refactor/drop committed range sweep](https://github.com/dashpay/rust-dashcore/pull/1016) — 🔴 CI failing · 📝 draft · areas: dash-spv, key-wallet, key-wallet-manager · ⚠ ownership unresolved: dash-spv, key-wallet, key-wallet-manager · Policy: draft
  - Blocker: Draft PR does not occupy a review slot

<a id="zocolini-needs-action"></a>
#### Needs action (2)
- [dashpay/rust-dashcore#375 refactor(dash-spv): checkpoint rewrite](https://github.com/dashpay/rust-dashcore/pull/375) — 2 unresolved (2 CodeRabbit) · 234 days stale · ⚠ merge conflict · areas: dash-spv, fallback · ⚠ ownership unresolved: dash-spv · Policy: configuration-error
  - Top thread: "_⚠️ Potential issue_ \| _🟡 Minor_" — 234 days old
  - Blocker: Unresolved identities in dash-spv
- [dashpay/rust-dashcore#902 refactor(dash-spv): network manager refactor and sync pipelines optimized ](https://github.com/dashpay/rust-dashcore/pull/902) — ✋ changes requested · 🔴 CI failing · areas: dash-spv, fallback · ⚠ ownership unresolved: dash-spv · Policy: configuration-error
  - Blocker: Unresolved identities in dash-spv

<a id="zocolini-unresolved-comments"></a>
#### Unresolved Comments (1)
- [dashpay/rust-dashcore#375 refactor(dash-spv): checkpoint rewrite](https://github.com/dashpay/rust-dashcore/pull/375) — 2 unresolved (2 CodeRabbit) · 234 days stale · ⚠ merge conflict · areas: dash-spv, fallback · ⚠ ownership unresolved: dash-spv · Policy: configuration-error
  - Top thread: "_⚠️ Potential issue_ \| _🟡 Minor_" — 234 days old
  - Blocker: Unresolved identities in dash-spv

<a id="zocolini-changes-requested"></a>
#### Changes Requested (1)
- [dashpay/rust-dashcore#902 refactor(dash-spv): network manager refactor and sync pipelines optimized ](https://github.com/dashpay/rust-dashcore/pull/902) — ✋ changes requested · 🔴 CI failing · areas: dash-spv, fallback · ⚠ ownership unresolved: dash-spv · Policy: configuration-error
  - Blocker: Unresolved identities in dash-spv

<a id="zocolini-ci-failing"></a>
#### CI Failing (3)
- [dashpay/rust-dashcore#1013 feat(dash-spv-bench): report the client's peak RSS](https://github.com/dashpay/rust-dashcore/pull/1013) — 🔴 CI failing · areas: fallback · Policy: waiting-slot
  - Blocker: Waiting for one of five author review slots
- [dashpay/rust-dashcore#1014 fix(key-wallet): prune observed spends during the initial sync](https://github.com/dashpay/rust-dashcore/pull/1014) — 🔴 CI failing · areas: dash-spv, key-wallet, key-wallet-manager · ⚠ ownership unresolved: dash-spv, key-wallet, key-wallet-manager · Policy: waiting-slot
  - Blocker: Waiting for one of five author review slots
- [dashpay/rust-dashcore#1015 fix(key-wallet): re-apply a spend whose coin was funded after it](https://github.com/dashpay/rust-dashcore/pull/1015) — 🔴 CI failing · areas: dash-spv, key-wallet, key-wallet-manager · ⚠ ownership unresolved: dash-spv, key-wallet, key-wallet-manager · Policy: waiting-slot
  - Blocker: Waiting for one of five author review slots

<a id="zocolini-draft"></a>
#### Draft (6)
- [dashpay/rust-dashcore#496 Refactor/transaction builder](https://github.com/dashpay/rust-dashcore/pull/496) — 8 unresolved (8 CodeRabbit) · 191 days stale · ⚠ merge conflict · 📝 draft · areas: key-wallet, key-wallet-manager, fallback · ⚠ ownership unresolved: key-wallet, key-wallet-manager · Policy: draft
  - Top thread: "_⚠️ Potential issue_ \| _🔴 Critical_" — 191 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#370 Refactor: config builder to ensure validation](https://github.com/dashpay/rust-dashcore/pull/370) — ⚠ merge conflict · 📝 draft · areas: dash-spv, fallback · ⚠ ownership unresolved: dash-spv · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#498 chore(key-wallet): drop unnecesary codebase complexity removing the bincode dependency](https://github.com/dashpay/rust-dashcore/pull/498) — ⚠ merge conflict · 🔴 CI failing · 📝 draft · areas: key-wallet, key-wallet-manager, fallback · ⚠ ownership unresolved: key-wallet, key-wallet-manager · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#515 chore(dashcore): restrict ServiceFlags api](https://github.com/dashpay/rust-dashcore/pull/515) — 🔴 CI failing · 📝 draft · areas: dash-spv, fallback · ⚠ ownership unresolved: dash-spv · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#921 fix(dash-spv): sync backfill](https://github.com/dashpay/rust-dashcore/pull/921) — ⚠ merge conflict · 📝 draft · areas: dash-spv, fallback · ⚠ ownership unresolved: dash-spv · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#1016 Refactor/drop committed range sweep](https://github.com/dashpay/rust-dashcore/pull/1016) — 🔴 CI failing · 📝 draft · areas: dash-spv, key-wallet, key-wallet-manager · ⚠ ownership unresolved: dash-spv, key-wallet, key-wallet-manager · Policy: draft
  - Blocker: Draft PR does not occupy a review slot

<a id="zocolini-stale"></a>
#### Stale (1)
- [dashpay/rust-dashcore#374 Feat: config can be built from any json reader](https://github.com/dashpay/rust-dashcore/pull/374) — ⚠ merge conflict · 📝 draft · 🐢 targets refacor/config-builder, untouched 235 days · areas: dash-spv · ⚠ ownership unresolved: dash-spv

<a id="zocolini-clean"></a>
#### Clean (2)
- [dashpay/rust-dashcore#993 fix(dash-spv): wire up masternode persistence and shrink it 67x](https://github.com/dashpay/rust-dashcore/pull/993) — areas: dash-spv, fallback · ⚠ ownership unresolved: dash-spv · Policy: configuration-error
  - Blocker: Unresolved identities in dash-spv
- [dashpay/rust-dashcore#1009 fix(key-wallet): consult every fund-bearing account for provider transactions](https://github.com/dashpay/rust-dashcore/pull/1009) — areas: key-wallet · ⚠ ownership unresolved: key-wallet · Policy: configuration-error
  - Blocker: Unresolved identities in key-wallet

<a id="zocolini-ready-for-review"></a>
#### Ready for Review (5)
- [dashpay/platform#4454 test(swift-sdk): pin non-English BIP-39 mnemonic support against the fixed FFI](https://github.com/dashpay/platform/pull/4454) — by @PastaPastaPasta · areas: swift-sdk · Policy: waiting-bots
  - Blocker: CodeRabbit completion missing for current head
- [dashpay/platform#4519 feat(platform-wallet): rotate a masternode's keys into the wallet — ProUpRegTx orchestration, FFI, Swift](https://github.com/dashpay/platform/pull/4519) — by @QuantumExplorer · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk · Policy: waiting-bots
  - Blocker: thepastaclaw final review missing for current head
  - Blocker: CodeRabbit completion missing for current head
  - Blocker: Bot changes request remains outstanding
- [dashpay/platform#4708 feat(platform)!: add ShieldFromIdentity state transition (identity balance to shielded pool)](https://github.com/dashpay/platform/pull/4708) — by @QuantumExplorer · areas: rs-drive, rs-drive-abci, dpp, rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, rust-sdk, kotlin-sdk, fallback · Policy: waiting-slot
  - Blocker: Waiting for one of five author review slots
- [dashpay/platform#4709 fix(swift-sdk)!: expose reservation-aware local shielded balance snapshots](https://github.com/dashpay/platform/pull/4709) — by @llbartekll · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk · Policy: waiting-bots
  - Blocker: thepastaclaw final review missing for current head
- [dashpay/rust-dashcore#1002 fix(dash-spv): make backward coverage durable by rewinding synced_height instead of sweeping in memory](https://github.com/dashpay/rust-dashcore/pull/1002) — by @romchornyi · areas: dash-spv, key-wallet-manager, fallback · ⚠ ownership unresolved: dash-spv, key-wallet-manager · Policy: configuration-error
  - Blocker: Unresolved identities in dash-spv

<a id="xdustinface"></a>
### @xdustinface
<a id="xdustinface-open"></a>
#### Open (8)
- [dashpay/rust-dashcore#856 feat(dash-spv): force resync to re-anchor a running client at a lower checkpoint](https://github.com/dashpay/rust-dashcore/pull/856) — 1 unresolved (1 human) · 53 days stale · ⚠ merge conflict · 🔴 CI failing · areas: dash-spv · ⚠ ownership unresolved: dash-spv · Policy: configuration-error
  - Top thread: "why do we have a periodic check here??" — 53 days old
  - Blocker: Unresolved identities in dash-spv
- [dashpay/rust-dashcore#400 chore: remove BIP141 (SegWit) and BIP341 (Taproot) support](https://github.com/dashpay/rust-dashcore/pull/400) — ⚠ merge conflict · 🔴 CI failing · 📝 draft · areas: dash-spv, key-wallet, fallback · ⚠ ownership unresolved: dash-spv, key-wallet · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#541 ci: integrate Codecov test analytics via `cargo-nextest`](https://github.com/dashpay/rust-dashcore/pull/541) — ⚠ merge conflict · 📝 draft · areas: fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#720 fix(dash-spv): split peer `TcpStream` and add per-peer writer task](https://github.com/dashpay/rust-dashcore/pull/720) — ⚠ merge conflict · 🔴 CI failing · 📝 draft · areas: dash-spv, fallback · ⚠ ownership unresolved: dash-spv · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#776 chore: remove FFI layer](https://github.com/dashpay/rust-dashcore/pull/776) — ⚠ merge conflict · 📝 draft · areas: dash-spv, key-wallet, fallback · ⚠ ownership unresolved: dash-spv, key-wallet · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#799 feat: validate masternode list merkle root](https://github.com/dashpay/rust-dashcore/pull/799) — ⚠ merge conflict · 📝 draft · areas: fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#803 feat(dash-spv): block locator + staged fork detection](https://github.com/dashpay/rust-dashcore/pull/803) — ⚠ merge conflict · 📝 draft · areas: dash-spv, fallback · ⚠ ownership unresolved: dash-spv · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#849 feat(dash-spv): add `--birth-height` CLI flag](https://github.com/dashpay/rust-dashcore/pull/849) — areas: dash-spv · ⚠ ownership unresolved: dash-spv · Policy: configuration-error
  - Blocker: Unresolved identities in dash-spv

<a id="xdustinface-needs-action"></a>
#### Needs action (1)
- [dashpay/rust-dashcore#856 feat(dash-spv): force resync to re-anchor a running client at a lower checkpoint](https://github.com/dashpay/rust-dashcore/pull/856) — 1 unresolved (1 human) · 53 days stale · ⚠ merge conflict · 🔴 CI failing · areas: dash-spv · ⚠ ownership unresolved: dash-spv · Policy: configuration-error
  - Top thread: "why do we have a periodic check here??" — 53 days old
  - Blocker: Unresolved identities in dash-spv

<a id="xdustinface-unresolved-comments"></a>
#### Unresolved Comments (1)
- [dashpay/rust-dashcore#856 feat(dash-spv): force resync to re-anchor a running client at a lower checkpoint](https://github.com/dashpay/rust-dashcore/pull/856) — 1 unresolved (1 human) · 53 days stale · ⚠ merge conflict · 🔴 CI failing · areas: dash-spv · ⚠ ownership unresolved: dash-spv · Policy: configuration-error
  - Top thread: "why do we have a periodic check here??" — 53 days old
  - Blocker: Unresolved identities in dash-spv

<a id="xdustinface-draft"></a>
#### Draft (6)
- [dashpay/rust-dashcore#400 chore: remove BIP141 (SegWit) and BIP341 (Taproot) support](https://github.com/dashpay/rust-dashcore/pull/400) — ⚠ merge conflict · 🔴 CI failing · 📝 draft · areas: dash-spv, key-wallet, fallback · ⚠ ownership unresolved: dash-spv, key-wallet · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#541 ci: integrate Codecov test analytics via `cargo-nextest`](https://github.com/dashpay/rust-dashcore/pull/541) — ⚠ merge conflict · 📝 draft · areas: fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#720 fix(dash-spv): split peer `TcpStream` and add per-peer writer task](https://github.com/dashpay/rust-dashcore/pull/720) — ⚠ merge conflict · 🔴 CI failing · 📝 draft · areas: dash-spv, fallback · ⚠ ownership unresolved: dash-spv · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#776 chore: remove FFI layer](https://github.com/dashpay/rust-dashcore/pull/776) — ⚠ merge conflict · 📝 draft · areas: dash-spv, key-wallet, fallback · ⚠ ownership unresolved: dash-spv, key-wallet · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#799 feat: validate masternode list merkle root](https://github.com/dashpay/rust-dashcore/pull/799) — ⚠ merge conflict · 📝 draft · areas: fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#803 feat(dash-spv): block locator + staged fork detection](https://github.com/dashpay/rust-dashcore/pull/803) — ⚠ merge conflict · 📝 draft · areas: dash-spv, fallback · ⚠ ownership unresolved: dash-spv · Policy: draft
  - Blocker: Draft PR does not occupy a review slot

<a id="xdustinface-clean"></a>
#### Clean (1)
- [dashpay/rust-dashcore#849 feat(dash-spv): add `--birth-height` CLI flag](https://github.com/dashpay/rust-dashcore/pull/849) — areas: dash-spv · ⚠ ownership unresolved: dash-spv · Policy: configuration-error
  - Blocker: Unresolved identities in dash-spv

<a id="xdustinface-ready-for-review"></a>
#### Ready for Review (3)
- [dashpay/rust-dashcore#993 fix(dash-spv): wire up masternode persistence and shrink it 67x](https://github.com/dashpay/rust-dashcore/pull/993) — by @ZocoLini · areas: dash-spv, fallback · ⚠ ownership unresolved: dash-spv · Policy: configuration-error
  - Blocker: Unresolved identities in dash-spv
- [dashpay/rust-dashcore#1002 fix(dash-spv): make backward coverage durable by rewinding synced_height instead of sweeping in memory](https://github.com/dashpay/rust-dashcore/pull/1002) — by @romchornyi · areas: dash-spv, key-wallet-manager, fallback · ⚠ ownership unresolved: dash-spv, key-wallet-manager · Policy: configuration-error
  - Blocker: Unresolved identities in dash-spv
- [dashpay/rust-dashcore#1009 fix(key-wallet): consult every fund-bearing account for provider transactions](https://github.com/dashpay/rust-dashcore/pull/1009) — by @ZocoLini · areas: key-wallet · ⚠ ownership unresolved: key-wallet · Policy: configuration-error
  - Blocker: Unresolved identities in key-wallet

<a id="vivekgsharma"></a>
### @vivekgsharma
<a id="vivekgsharma-open"></a>
#### Open (1)
- [dashpay/platform#4189 ci: enforce macOS runner disk reserve](https://github.com/dashpay/platform/pull/4189) — 2 unresolved (1 CodeRabbit, 1 bot) · 53 days stale · ✋ changes requested · 🔴 CI failing · areas: fallback · Policy: waiting-bots
  - Top thread: "_🩺 Stability & Availability_ \| _🔴 Critical_ \| _⚡ Quick win_" — 53 days old
  - Blocker: thepastaclaw final review missing for current head
  - Blocker: CodeRabbit completion missing for current head
  - Blocker: Bot changes request remains outstanding
  - … 1 more blocker(s)

<a id="vivekgsharma-needs-action"></a>
#### Needs action (1)
- [dashpay/platform#4189 ci: enforce macOS runner disk reserve](https://github.com/dashpay/platform/pull/4189) — 2 unresolved (1 CodeRabbit, 1 bot) · 53 days stale · ✋ changes requested · 🔴 CI failing · areas: fallback · Policy: waiting-bots
  - Top thread: "_🩺 Stability & Availability_ \| _🔴 Critical_ \| _⚡ Quick win_" — 53 days old
  - Blocker: thepastaclaw final review missing for current head
  - Blocker: CodeRabbit completion missing for current head
  - Blocker: Bot changes request remains outstanding
  - … 1 more blocker(s)

<a id="vivekgsharma-unresolved-comments"></a>
#### Unresolved Comments (1)
- [dashpay/platform#4189 ci: enforce macOS runner disk reserve](https://github.com/dashpay/platform/pull/4189) — 2 unresolved (1 CodeRabbit, 1 bot) · 53 days stale · ✋ changes requested · 🔴 CI failing · areas: fallback · Policy: waiting-bots
  - Top thread: "_🩺 Stability & Availability_ \| _🔴 Critical_ \| _⚡ Quick win_" — 53 days old
  - Blocker: thepastaclaw final review missing for current head
  - Blocker: CodeRabbit completion missing for current head
  - Blocker: Bot changes request remains outstanding
  - … 1 more blocker(s)

<a id="pshenmic"></a>
### @pshenmic
<a id="pshenmic-open"></a>
#### Open (1)
- [dashpay/platform#2446 feat(js-dapi-client): add contested resources query methods](https://github.com/dashpay/platform/pull/2446) — 5 unresolved (5 CodeRabbit) · 591 days stale · ⚠ merge conflict · 📝 draft · 🐢 targets v2.0-dev, untouched 581 days · areas: fallback
  - Top thread: "_:warning: Potential issue_" — 591 days old

<a id="pshenmic-stale"></a>
#### Stale (1)
- [dashpay/platform#2446 feat(js-dapi-client): add contested resources query methods](https://github.com/dashpay/platform/pull/2446) — 5 unresolved (5 CodeRabbit) · 591 days stale · ⚠ merge conflict · 📝 draft · 🐢 targets v2.0-dev, untouched 581 days · areas: fallback
  - Top thread: "_:warning: Potential issue_" — 591 days old

<a id="ogabrielides"></a>
### @ogabrielides
<a id="ogabrielides-open"></a>
#### Open (1)
- [dashpay/platform#2486 feat(drive-abci): state sync - faster sync of new nodes](https://github.com/dashpay/platform/pull/2486) — ⏸ deferred · areas: rs-drive, rs-drive-abci, dpp, dashmate, fallback · ⚠ ownership unresolved: dashmate

<a id="ogabrielides-deferred"></a>
#### Deferred (1)
- [dashpay/platform#2486 feat(drive-abci): state sync - faster sync of new nodes](https://github.com/dashpay/platform/pull/2486) — ⏸ deferred · areas: rs-drive, rs-drive-abci, dpp, dashmate, fallback · ⚠ ownership unresolved: dashmate

<a id="thephez"></a>
### @thephez
<a id="thephez-open"></a>
#### Open (1)
- [dashpay/platform#4576 docs(wasm-sdk): clarify aggregate groupBy behavior](https://github.com/dashpay/platform/pull/4576) — areas: js-wasm-sdk · Policy: waiting-bots
  - Blocker: CodeRabbit completion missing for current head

<a id="thephez-clean"></a>
#### Clean (1)
- [dashpay/platform#4576 docs(wasm-sdk): clarify aggregate groupBy behavior](https://github.com/dashpay/platform/pull/4576) — areas: js-wasm-sdk · Policy: waiting-bots
  - Blocker: CodeRabbit completion missing for current head

## Methodology
Generated nightly by [pr-hygiene](https://github.com/dashpay/stale_prs_are_bad). A thread counts as "unresolved" when it is open, not outdated, has a comment from someone other than the PR author, and the most recent comment is from a reviewer. **Dirty** = at least one such thread. **Unresolved Comments** = at least one such thread. **Changes Requested** = no unresolved threads but a reviewer's most recent review is CHANGES_REQUESTED (still blocking until someone re-approves or dismisses). **Deferred** = carries a configured deferred label (e.g. `postponed`) — visible but not counted toward unresolved-comment counts. **Stale** = targets a non-default branch OR hasn't been touched in the configured threshold (default 120 days, but clean PRs are never reclassified as stale). **Draft** = the PR is still marked draft on GitHub. **CI failing** = no unresolved comments, no changes-requested, but the latest commit's status check is failing. **Clean** = open, not draft, not deferred, not stale, no unresolved comments, no changes-requested, CI green. **Needs action** further requires changes-requested, merge conflict, or that the reviewer commented more recently than the author last pushed. **Ready for human** counts a person's own PRs that the shared review engine marks `ready-for-human` or `ready-to-merge`; each PR bullet shows the engine's state as `Policy:` with its blockers. **Ready for Review** counts clean PRs (authored by someone else) where this person owes a review: the union of the shared policy's routing (owners and reviewers of every area the changed files fall into, or the repository fallback for files no area claims) and GitHub's explicit review requests. The author is never routed to their own PR, and anyone who has already submitted any review is excluded — their job is done. `⚠ ownership unresolved` marks areas whose roster the policy still lists as open. Configurable via [`https://github.com/dashpay/stale_prs_are_bad/blob/master/.pr-hygiene.yml`](https://github.com/dashpay/stale_prs_are_bad/blob/master/.pr-hygiene.yml)—edit defaults there; ownership lives in `policies/`.
