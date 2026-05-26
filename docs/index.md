---
---
# PR Hygiene Report
*Last updated: 2026-05-26 00:46 UTC · commit 5f5c8eb*

## Summary
- Open PRs: **63** (5 clean · 0 CI failing · 12 changes requested · 7 unresolved comments · 9 deferred · 17 draft · 13 stale)
- PRs needing author action: **19**
- Total unresolved comments: **90**

## Scoreboard
_Sort: unresolved-comments desc → needs-action desc → ready-for-review desc. Click any number to jump to the specific PRs it covers._

| Author | Open | Clean | CI failing | Unresolved Comments | Changes Requested | Deferred | Draft | Stale | Needs action | Total Unresolved Comments | Ready for Review | Δ |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| [@PastaPastaPasta + (@thepastaclaw)](#pastapastapasta) | [8+(11)](#pastapastapasta-open) | [1+(0)](#pastapastapasta-clean) | — | [3+(0)](#pastapastapasta-unresolved-comments) | [0+(1)](#pastapastapasta-changes-requested) | — | [4+(8)](#pastapastapasta-draft) | [0+(2)](#pastapastapasta-stale) | [3+(1)](#pastapastapasta-needs-action) | [3+(5)](#pastapastapasta-unresolved-comments) | — | ↓ 1 |
| [@QuantumExplorer](#quantumexplorer) | [6](#quantumexplorer-open) | [1](#quantumexplorer-clean) | — | [2](#quantumexplorer-unresolved-comments) | — | [2](#quantumexplorer-deferred) | [1](#quantumexplorer-draft) | — | [2](#quantumexplorer-needs-action) | [16](#quantumexplorer-unresolved-comments) | [4](#quantumexplorer-ready-for-review) | — |
| [@lklimek + (@Claudius-Maginificent)](#lklimek) | [10+(9)](#lklimek-open) | [2+(1)](#lklimek-clean) | — | [0+(1)](#lklimek-unresolved-comments) | [2+(2)](#lklimek-changes-requested) | [5+(0)](#lklimek-deferred) | [0+(1)](#lklimek-draft) | [1+(4)](#lklimek-stale) | [2+(3)](#lklimek-needs-action) | [0+(30)](#lklimek-unresolved-comments) | — | ↓ 7 |
| [@ktechmidas](#ktechmidas) | [1](#ktechmidas-open) | — | — | [1](#ktechmidas-unresolved-comments) | — | — | — | — | [1](#ktechmidas-needs-action) | [6](#ktechmidas-unresolved-comments) | — | — |
| [@ZocoLini](#zocolini) | [4](#zocolini-open) | — | — | — | [3](#zocolini-changes-requested) | — | [1](#zocolini-draft) | — | [3](#zocolini-needs-action) | — | — | — |
| [@shumkov](#shumkov) | [6](#shumkov-open) | — | — | — | [2](#shumkov-changes-requested) | [1](#shumkov-deferred) | [1](#shumkov-draft) | [2](#shumkov-stale) | [2](#shumkov-needs-action) | — | [3](#shumkov-ready-for-review) | ↑ 1 |
| [@llbartekll](#llbartekll) | [3](#llbartekll-open) | — | — | — | [2](#llbartekll-changes-requested) | — | [1](#llbartekll-draft) | — | [2](#llbartekll-needs-action) | [1](#llbartekll-unresolved-comments) | [1](#llbartekll-ready-for-review) | ↑ 1 |
| [@pshenmic](#pshenmic) | [4](#pshenmic-open) | — | — | — | — | — | — | [4](#pshenmic-stale) | — | [29](#pshenmic-unresolved-comments) | — | — |
| [@ogabrielides](#ogabrielides) | [1](#ogabrielides-open) | — | — | — | — | [1](#ogabrielides-deferred) | — | — | — | — | — | — |

## Per-author detail

<a id="pastapastapasta"></a>
### @PastaPastaPasta + (@thepastaclaw)
<a id="pastapastapasta-open"></a>
#### Open (19)
- [#3096 feat(sdk): add client-side validation to state transition construction methods](https://github.com/dashpay/platform/pull/3096) — via @thepastaclaw · 4 unresolved (2 CodeRabbit, 2 human) · 97 days stale · 📝 draft
  - Top thread: "_⚠️ Potential issue_ \| _🟡 Minor_" — 94 days old
- [#3509 fix(platform): default omitted proved query limits](https://github.com/dashpay/platform/pull/3509) — 1 unresolved (1 human) · 23 days stale · ✋ changes requested · 🔴 CI failing
  - Top thread: "<!-- thepastaclaw-review v1 finding=d3df3e197206 -->" — 23 days old
- [#3725 feat(drive): add document history retrieval](https://github.com/dashpay/platform/pull/3725) — 1 unresolved (1 CodeRabbit) · 4 days stale · ✋ changes requested
  - Top thread: "_⚠️ Potential issue_ \| _🟠 Major_ \| _🏗️ Heavy lift_" — 4 days old
- [#2988 feat(wasm-sdk): auto-generate entropy for document creation when not provided](https://github.com/dashpay/platform/pull/2988) — 1 unresolved (1 human) · 4 days stale · 🔴 CI failing
  - Top thread: "<!-- thepastaclaw-review v1 finding=30cbefe1c9ef -->" — 4 days old
- [#3734 feat(rs-sdk-ffi): expose SDK protocol version pinning](https://github.com/dashpay/platform/pull/3734) — via @thepastaclaw · 1 unresolved (1 human) · 0 days stale · 📝 draft
  - Top thread: "The Python example updates the call to `dash_sdk_create_with_protocol_version`, but the `DashSDKConfig` ctypes definitio…" — 0 days old
- [#3091 feat(wasm-sdk): add prepare_* APIs for idempotent document state transitions](https://github.com/dashpay/platform/pull/3091) — via @thepastaclaw · ✋ changes requested · 📝 draft
- [#3092 feat(rs-sdk): expose transition hash from state transition methods](https://github.com/dashpay/platform/pull/3092) — via @thepastaclaw · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · 📝 draft
- [#3133 fix(sdk): validate batch base structure before signing](https://github.com/dashpay/platform/pull/3133) — via @thepastaclaw · ✋ changes requested · 📝 draft
- [#3165 fix(drive): consolidate historical contract proof verification retry logic](https://github.com/dashpay/platform/pull/3165) — via @thepastaclaw · ✋ changes requested
- [#3461 fix(dpp): block pre-programmed distribution changes on token update](https://github.com/dashpay/platform/pull/3461) — 📝 draft
- [#3462 fix(dpp)!: version-gate distribution function floating-point evaluation](https://github.com/dashpay/platform/pull/3462) — 📝 draft
- [#3596 fix(platform-wallet): satisfy accessors clippy lints](https://github.com/dashpay/platform/pull/3596) — via @thepastaclaw · 📝 draft
- [#3635 ci: tolerate book preview comment permission errors](https://github.com/dashpay/platform/pull/3635) — via @thepastaclaw · 📝 draft
- [#3669 fix(swift-sdk): wait for SDK rebuild before wallet activation](https://github.com/dashpay/platform/pull/3669) — via @thepastaclaw · 📝 draft · 🐢 targets claude/swift-xcuitest-wallet-persistence
- [#3679 chore(dapi): cleanup — drop unused deps, inline winston/fetch/promisify shims](https://github.com/dashpay/platform/pull/3679)
- [#3680 refactor(dapi)!: expose Uint8Array instead of Buffer in public API of dapi-client](https://github.com/dashpay/platform/pull/3680) — ✋ changes requested · 📝 draft
- [#3689 refactor(sdk,wallet-lib,test-suite)!: convert dapi-client + wallet-lib + js-dash-sdk + platform-test-suite to ESM](https://github.com/dashpay/platform/pull/3689) — 📝 draft
- [#3722 fix(rs-sdk): replay chained unknown addresses](https://github.com/dashpay/platform/pull/3722) — via @thepastaclaw · 📝 draft · 🐢 targets fix/rs-sdk-address-sync-found-025
- [#3728 ci: upgrade yarn for Node 24 TypeDoc build](https://github.com/dashpay/platform/pull/3728) — via @thepastaclaw · 📝 draft

<a id="pastapastapasta-needs-action"></a>
#### Needs action (4)
- [#3509 fix(platform): default omitted proved query limits](https://github.com/dashpay/platform/pull/3509) — 1 unresolved (1 human) · 23 days stale · ✋ changes requested · 🔴 CI failing
  - Top thread: "<!-- thepastaclaw-review v1 finding=d3df3e197206 -->" — 23 days old
- [#3725 feat(drive): add document history retrieval](https://github.com/dashpay/platform/pull/3725) — 1 unresolved (1 CodeRabbit) · 4 days stale · ✋ changes requested
  - Top thread: "_⚠️ Potential issue_ \| _🟠 Major_ \| _🏗️ Heavy lift_" — 4 days old
- [#2988 feat(wasm-sdk): auto-generate entropy for document creation when not provided](https://github.com/dashpay/platform/pull/2988) — 1 unresolved (1 human) · 4 days stale · 🔴 CI failing
  - Top thread: "<!-- thepastaclaw-review v1 finding=30cbefe1c9ef -->" — 4 days old
- [#3165 fix(drive): consolidate historical contract proof verification retry logic](https://github.com/dashpay/platform/pull/3165) — via @thepastaclaw · ✋ changes requested

<a id="pastapastapasta-unresolved-comments"></a>
#### Unresolved Comments (3)
- [#3509 fix(platform): default omitted proved query limits](https://github.com/dashpay/platform/pull/3509) — 1 unresolved (1 human) · 23 days stale · ✋ changes requested · 🔴 CI failing
  - Top thread: "<!-- thepastaclaw-review v1 finding=d3df3e197206 -->" — 23 days old
- [#3725 feat(drive): add document history retrieval](https://github.com/dashpay/platform/pull/3725) — 1 unresolved (1 CodeRabbit) · 4 days stale · ✋ changes requested
  - Top thread: "_⚠️ Potential issue_ \| _🟠 Major_ \| _🏗️ Heavy lift_" — 4 days old
- [#2988 feat(wasm-sdk): auto-generate entropy for document creation when not provided](https://github.com/dashpay/platform/pull/2988) — 1 unresolved (1 human) · 4 days stale · 🔴 CI failing
  - Top thread: "<!-- thepastaclaw-review v1 finding=30cbefe1c9ef -->" — 4 days old

<a id="pastapastapasta-changes-requested"></a>
#### Changes Requested (1)
- [#3165 fix(drive): consolidate historical contract proof verification retry logic](https://github.com/dashpay/platform/pull/3165) — via @thepastaclaw · ✋ changes requested

<a id="pastapastapasta-draft"></a>
#### Draft (12)
- [#3096 feat(sdk): add client-side validation to state transition construction methods](https://github.com/dashpay/platform/pull/3096) — via @thepastaclaw · 4 unresolved (2 CodeRabbit, 2 human) · 97 days stale · 📝 draft
  - Top thread: "_⚠️ Potential issue_ \| _🟡 Minor_" — 94 days old
- [#3734 feat(rs-sdk-ffi): expose SDK protocol version pinning](https://github.com/dashpay/platform/pull/3734) — via @thepastaclaw · 1 unresolved (1 human) · 0 days stale · 📝 draft
  - Top thread: "The Python example updates the call to `dash_sdk_create_with_protocol_version`, but the `DashSDKConfig` ctypes definitio…" — 0 days old
- [#3091 feat(wasm-sdk): add prepare_* APIs for idempotent document state transitions](https://github.com/dashpay/platform/pull/3091) — via @thepastaclaw · ✋ changes requested · 📝 draft
- [#3092 feat(rs-sdk): expose transition hash from state transition methods](https://github.com/dashpay/platform/pull/3092) — via @thepastaclaw · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · 📝 draft
- [#3133 fix(sdk): validate batch base structure before signing](https://github.com/dashpay/platform/pull/3133) — via @thepastaclaw · ✋ changes requested · 📝 draft
- [#3461 fix(dpp): block pre-programmed distribution changes on token update](https://github.com/dashpay/platform/pull/3461) — 📝 draft
- [#3462 fix(dpp)!: version-gate distribution function floating-point evaluation](https://github.com/dashpay/platform/pull/3462) — 📝 draft
- [#3596 fix(platform-wallet): satisfy accessors clippy lints](https://github.com/dashpay/platform/pull/3596) — via @thepastaclaw · 📝 draft
- [#3635 ci: tolerate book preview comment permission errors](https://github.com/dashpay/platform/pull/3635) — via @thepastaclaw · 📝 draft
- [#3680 refactor(dapi)!: expose Uint8Array instead of Buffer in public API of dapi-client](https://github.com/dashpay/platform/pull/3680) — ✋ changes requested · 📝 draft
- [#3689 refactor(sdk,wallet-lib,test-suite)!: convert dapi-client + wallet-lib + js-dash-sdk + platform-test-suite to ESM](https://github.com/dashpay/platform/pull/3689) — 📝 draft
- [#3728 ci: upgrade yarn for Node 24 TypeDoc build](https://github.com/dashpay/platform/pull/3728) — via @thepastaclaw · 📝 draft

<a id="pastapastapasta-stale"></a>
#### Stale (2)
- [#3669 fix(swift-sdk): wait for SDK rebuild before wallet activation](https://github.com/dashpay/platform/pull/3669) — via @thepastaclaw · 📝 draft · 🐢 targets claude/swift-xcuitest-wallet-persistence
- [#3722 fix(rs-sdk): replay chained unknown addresses](https://github.com/dashpay/platform/pull/3722) — via @thepastaclaw · 📝 draft · 🐢 targets fix/rs-sdk-address-sync-found-025

<a id="pastapastapasta-clean"></a>
#### Clean (1)
- [#3679 chore(dapi): cleanup — drop unused deps, inline winston/fetch/promisify shims](https://github.com/dashpay/platform/pull/3679)

<a id="quantumexplorer"></a>
### @QuantumExplorer
<a id="quantumexplorer-open"></a>
#### Open (6)
- [#3557 feat(platform-wallet): instrument wallet_manager RwLock behind lock-stats feature](https://github.com/dashpay/platform/pull/3557) — 11 unresolved (2 CodeRabbit, 9 human) · 28 days stale
  - Top thread: "_⚠️ Potential issue_ \| _🟠 Major_" — 28 days old
- [#3740 feat: add time-range indexes for trending/leaderboard queries](https://github.com/dashpay/platform/pull/3740) — 5 unresolved (5 CodeRabbit) · 0 days stale · ✋ changes requested · 🔴 CI failing
  - Top thread: "_⚠️ Potential issue_ \| _🟠 Major_ \| _⚡ Quick win_" — 0 days old
- [#1834 feat(drive-abci)!: Statesync](https://github.com/dashpay/platform/pull/1834) — 📝 draft · ⏸ deferred
- [#3021 feat(platform): better contract state transitions](https://github.com/dashpay/platform/pull/3021) — 📝 draft · ⏸ deferred
- [#3417 feat(swift-sdk): use SPV-synced quorums for Platform proof verification](https://github.com/dashpay/platform/pull/3417) — ⚠ merge conflict · 🔴 CI failing · 📝 draft
- [#3733 fix(swift-sdk): tie shielded completion suppression to a sync generation](https://github.com/dashpay/platform/pull/3733)

<a id="quantumexplorer-needs-action"></a>
#### Needs action (2)
- [#3557 feat(platform-wallet): instrument wallet_manager RwLock behind lock-stats feature](https://github.com/dashpay/platform/pull/3557) — 11 unresolved (2 CodeRabbit, 9 human) · 28 days stale
  - Top thread: "_⚠️ Potential issue_ \| _🟠 Major_" — 28 days old
- [#3740 feat: add time-range indexes for trending/leaderboard queries](https://github.com/dashpay/platform/pull/3740) — 5 unresolved (5 CodeRabbit) · 0 days stale · ✋ changes requested · 🔴 CI failing
  - Top thread: "_⚠️ Potential issue_ \| _🟠 Major_ \| _⚡ Quick win_" — 0 days old

<a id="quantumexplorer-unresolved-comments"></a>
#### Unresolved Comments (2)
- [#3557 feat(platform-wallet): instrument wallet_manager RwLock behind lock-stats feature](https://github.com/dashpay/platform/pull/3557) — 11 unresolved (2 CodeRabbit, 9 human) · 28 days stale
  - Top thread: "_⚠️ Potential issue_ \| _🟠 Major_" — 28 days old
- [#3740 feat: add time-range indexes for trending/leaderboard queries](https://github.com/dashpay/platform/pull/3740) — 5 unresolved (5 CodeRabbit) · 0 days stale · ✋ changes requested · 🔴 CI failing
  - Top thread: "_⚠️ Potential issue_ \| _🟠 Major_ \| _⚡ Quick win_" — 0 days old

<a id="quantumexplorer-deferred"></a>
#### Deferred (2)
- [#1834 feat(drive-abci)!: Statesync](https://github.com/dashpay/platform/pull/1834) — 📝 draft · ⏸ deferred
- [#3021 feat(platform): better contract state transitions](https://github.com/dashpay/platform/pull/3021) — 📝 draft · ⏸ deferred

<a id="quantumexplorer-draft"></a>
#### Draft (1)
- [#3417 feat(swift-sdk): use SPV-synced quorums for Platform proof verification](https://github.com/dashpay/platform/pull/3417) — ⚠ merge conflict · 🔴 CI failing · 📝 draft

<a id="quantumexplorer-clean"></a>
#### Clean (1)
- [#3733 fix(swift-sdk): tie shielded completion suppression to a sync generation](https://github.com/dashpay/platform/pull/3733)

<a id="quantumexplorer-ready-for-review"></a>
#### Ready for Review (4)
- [#3637 feat(platform-wallet): serde support](https://github.com/dashpay/platform/pull/3637) — by @Claudius-Maginificent
- [#3650 fix(sdk): address-sync no longer silently discards balance changes for post-snapshot addresses (Found-025)](https://github.com/dashpay/platform/pull/3650) — by @lklimek
- [#3679 chore(dapi): cleanup — drop unused deps, inline winston/fetch/promisify shims](https://github.com/dashpay/platform/pull/3679) — by @PastaPastaPasta
- [#3721 test(rs-sdk): relocate DPNS network tests from src/ to tests/](https://github.com/dashpay/platform/pull/3721) — by @lklimek

<a id="lklimek"></a>
### @lklimek + (@Claudius-Maginificent)
<a id="lklimek-open"></a>
#### Open (19)
- [#3549 test(platform-wallet): e2e framework + full test suite — triage pins, Found-*/PA-* guards, fail-closed persist, Stage-2 merge](https://github.com/dashpay/platform/pull/3549) — via @Claudius-Maginificent · 20 unresolved (20 human) · 23 days stale · ⚠ merge conflict · 📝 draft · 🐢 targets fix/rs-platform-wallet-auto-select-inputs
  - Top thread: "<!-- thepastaclaw-review v1 finding=a3725e9d50ca -->" — 23 days old
- [#3625 feat(platform-wallet): add platform-wallet-storage crate (sqlite persister)](https://github.com/dashpay/platform/pull/3625) — via @Claudius-Maginificent · 10 unresolved (6 CodeRabbit, 4 human) · 0 days stale · ✋ changes requested
  - Top thread: "README lists a `delete-wallet` CLI subcommand, but the actual CLI (src/bin/platform-wallet-storage.rs) does not define t…" — 0 days old
- [#2679 fix(drive-abci): don't panic on unsupported version error](https://github.com/dashpay/platform/pull/2679) — ⏸ deferred
- [#2795 feat(platform): add real-time platform event subscriptions via gRPC streaming](https://github.com/dashpay/platform/pull/2795) — ⏸ deferred
- [#2993 feat: identity reference validation](https://github.com/dashpay/platform/pull/2993) — 📝 draft · ⏸ deferred
- [#3009 feat: contract reference validation](https://github.com/dashpay/platform/pull/3009) — 📝 draft · ⏸ deferred
- [#3032 feat: document creation restriction by group membership](https://github.com/dashpay/platform/pull/3032) — 📝 draft · ⏸ deferred
- [#3554 fix(platform-wallet): auto_select_inputs honors Σ inputs == Σ outputs](https://github.com/dashpay/platform/pull/3554) — via @Claudius-Maginificent · ⚠ merge conflict · ✋ changes requested
- [#3585 fix: case-insensitive .dash + atomic state on broadcast failure](https://github.com/dashpay/platform/pull/3585) — via @Claudius-Maginificent · ⚠ merge conflict · ✋ changes requested
- [#3637 feat(platform-wallet): serde support](https://github.com/dashpay/platform/pull/3637) — via @Claudius-Maginificent
- [#3650 fix(sdk): address-sync no longer silently discards balance changes for post-snapshot addresses (Found-025)](https://github.com/dashpay/platform/pull/3650)
- [#3658 fix(platform-wallet): reserve platform receive address on hand-out (Found-026) [backport\]](https://github.com/dashpay/platform/pull/3658) — ✋ changes requested
- [#3659 fix(platform-wallet): fail-closed on registration persist error (Found-017) [backport\]](https://github.com/dashpay/platform/pull/3659) — ✋ changes requested
- [#3672 feat(platform-wallet): keyring_core secret backends — encrypted-file + OS keyring (secrets feature)](https://github.com/dashpay/platform/pull/3672) — via @Claudius-Maginificent · ✋ changes requested · 🐢 targets feat/platform-wallet-sqlite-persistor
- [#3692 feat(platform-wallet): watch-only rehydration from persistor (seedless load)](https://github.com/dashpay/platform/pull/3692) — via @Claudius-Maginificent · ⚠ merge conflict · 📝 draft · 🐢 targets feat/platform-wallet-storage-secrets
- [#3693 feat(platform-wallet): add contacts and identity-key rehydration (item G)](https://github.com/dashpay/platform/pull/3693) — via @Claudius-Maginificent · 📝 draft · 🐢 targets feat/platform-wallet-rehydration
- [#3721 test(rs-sdk): relocate DPNS network tests from src/ to tests/](https://github.com/dashpay/platform/pull/3721)
- [#3727 test(platform-wallet): shielded (Orchard) e2e suite — spec + Wave H harness](https://github.com/dashpay/platform/pull/3727) — 📝 draft · 🐢 targets feat/rs-platform-wallet-e2e
- [#3735 fix(platform-wallet-ffi): wallet_id gate on resolver-fed sign entrypoints](https://github.com/dashpay/platform/pull/3735) — via @Claudius-Maginificent · ⚠ merge conflict · 🔴 CI failing · 📝 draft

<a id="lklimek-needs-action"></a>
#### Needs action (5)
- [#3625 feat(platform-wallet): add platform-wallet-storage crate (sqlite persister)](https://github.com/dashpay/platform/pull/3625) — via @Claudius-Maginificent · 10 unresolved (6 CodeRabbit, 4 human) · 0 days stale · ✋ changes requested
  - Top thread: "README lists a `delete-wallet` CLI subcommand, but the actual CLI (src/bin/platform-wallet-storage.rs) does not define t…" — 0 days old
- [#3554 fix(platform-wallet): auto_select_inputs honors Σ inputs == Σ outputs](https://github.com/dashpay/platform/pull/3554) — via @Claudius-Maginificent · ⚠ merge conflict · ✋ changes requested
- [#3585 fix: case-insensitive .dash + atomic state on broadcast failure](https://github.com/dashpay/platform/pull/3585) — via @Claudius-Maginificent · ⚠ merge conflict · ✋ changes requested
- [#3658 fix(platform-wallet): reserve platform receive address on hand-out (Found-026) [backport\]](https://github.com/dashpay/platform/pull/3658) — ✋ changes requested
- [#3659 fix(platform-wallet): fail-closed on registration persist error (Found-017) [backport\]](https://github.com/dashpay/platform/pull/3659) — ✋ changes requested

<a id="lklimek-unresolved-comments"></a>
#### Unresolved Comments (1)
- [#3625 feat(platform-wallet): add platform-wallet-storage crate (sqlite persister)](https://github.com/dashpay/platform/pull/3625) — via @Claudius-Maginificent · 10 unresolved (6 CodeRabbit, 4 human) · 0 days stale · ✋ changes requested
  - Top thread: "README lists a `delete-wallet` CLI subcommand, but the actual CLI (src/bin/platform-wallet-storage.rs) does not define t…" — 0 days old

<a id="lklimek-changes-requested"></a>
#### Changes Requested (4)
- [#3554 fix(platform-wallet): auto_select_inputs honors Σ inputs == Σ outputs](https://github.com/dashpay/platform/pull/3554) — via @Claudius-Maginificent · ⚠ merge conflict · ✋ changes requested
- [#3585 fix: case-insensitive .dash + atomic state on broadcast failure](https://github.com/dashpay/platform/pull/3585) — via @Claudius-Maginificent · ⚠ merge conflict · ✋ changes requested
- [#3658 fix(platform-wallet): reserve platform receive address on hand-out (Found-026) [backport\]](https://github.com/dashpay/platform/pull/3658) — ✋ changes requested
- [#3659 fix(platform-wallet): fail-closed on registration persist error (Found-017) [backport\]](https://github.com/dashpay/platform/pull/3659) — ✋ changes requested

<a id="lklimek-deferred"></a>
#### Deferred (5)
- [#2679 fix(drive-abci): don't panic on unsupported version error](https://github.com/dashpay/platform/pull/2679) — ⏸ deferred
- [#2795 feat(platform): add real-time platform event subscriptions via gRPC streaming](https://github.com/dashpay/platform/pull/2795) — ⏸ deferred
- [#2993 feat: identity reference validation](https://github.com/dashpay/platform/pull/2993) — 📝 draft · ⏸ deferred
- [#3009 feat: contract reference validation](https://github.com/dashpay/platform/pull/3009) — 📝 draft · ⏸ deferred
- [#3032 feat: document creation restriction by group membership](https://github.com/dashpay/platform/pull/3032) — 📝 draft · ⏸ deferred

<a id="lklimek-draft"></a>
#### Draft (1)
- [#3735 fix(platform-wallet-ffi): wallet_id gate on resolver-fed sign entrypoints](https://github.com/dashpay/platform/pull/3735) — via @Claudius-Maginificent · ⚠ merge conflict · 🔴 CI failing · 📝 draft

<a id="lklimek-stale"></a>
#### Stale (5)
- [#3549 test(platform-wallet): e2e framework + full test suite — triage pins, Found-*/PA-* guards, fail-closed persist, Stage-2 merge](https://github.com/dashpay/platform/pull/3549) — via @Claudius-Maginificent · 20 unresolved (20 human) · 23 days stale · ⚠ merge conflict · 📝 draft · 🐢 targets fix/rs-platform-wallet-auto-select-inputs
  - Top thread: "<!-- thepastaclaw-review v1 finding=a3725e9d50ca -->" — 23 days old
- [#3672 feat(platform-wallet): keyring_core secret backends — encrypted-file + OS keyring (secrets feature)](https://github.com/dashpay/platform/pull/3672) — via @Claudius-Maginificent · ✋ changes requested · 🐢 targets feat/platform-wallet-sqlite-persistor
- [#3692 feat(platform-wallet): watch-only rehydration from persistor (seedless load)](https://github.com/dashpay/platform/pull/3692) — via @Claudius-Maginificent · ⚠ merge conflict · 📝 draft · 🐢 targets feat/platform-wallet-storage-secrets
- [#3693 feat(platform-wallet): add contacts and identity-key rehydration (item G)](https://github.com/dashpay/platform/pull/3693) — via @Claudius-Maginificent · 📝 draft · 🐢 targets feat/platform-wallet-rehydration
- [#3727 test(platform-wallet): shielded (Orchard) e2e suite — spec + Wave H harness](https://github.com/dashpay/platform/pull/3727) — 📝 draft · 🐢 targets feat/rs-platform-wallet-e2e

<a id="lklimek-clean"></a>
#### Clean (3)
- [#3637 feat(platform-wallet): serde support](https://github.com/dashpay/platform/pull/3637) — via @Claudius-Maginificent
- [#3650 fix(sdk): address-sync no longer silently discards balance changes for post-snapshot addresses (Found-025)](https://github.com/dashpay/platform/pull/3650)
- [#3721 test(rs-sdk): relocate DPNS network tests from src/ to tests/](https://github.com/dashpay/platform/pull/3721)

<a id="ktechmidas"></a>
### @ktechmidas
<a id="ktechmidas-open"></a>
#### Open (1)
- [#3162 fix(dashmate): prevent orphaned verification container blocking SSL renewal](https://github.com/dashpay/platform/pull/3162) — 6 unresolved (6 human) · 7 days stale
  - Top thread: "<!-- thepastaclaw-review v1 finding=d7ee272d2ab4 dedupe=d8f79f6f47c868dc -->" — 7 days old

<a id="ktechmidas-needs-action"></a>
#### Needs action (1)
- [#3162 fix(dashmate): prevent orphaned verification container blocking SSL renewal](https://github.com/dashpay/platform/pull/3162) — 6 unresolved (6 human) · 7 days stale
  - Top thread: "<!-- thepastaclaw-review v1 finding=d7ee272d2ab4 dedupe=d8f79f6f47c868dc -->" — 7 days old

<a id="ktechmidas-unresolved-comments"></a>
#### Unresolved Comments (1)
- [#3162 fix(dashmate): prevent orphaned verification container blocking SSL renewal](https://github.com/dashpay/platform/pull/3162) — 6 unresolved (6 human) · 7 days stale
  - Top thread: "<!-- thepastaclaw-review v1 finding=d7ee272d2ab4 dedupe=d8f79f6f47c868dc -->" — 7 days old

<a id="zocolini"></a>
### @ZocoLini
<a id="zocolini-open"></a>
#### Open (4)
- [#3479 test(swift-sdk): swift-sdk test updated and added to CI](https://github.com/dashpay/platform/pull/3479) — ✋ changes requested
- [#3639 feat(platform-wallet): external signable wallets](https://github.com/dashpay/platform/pull/3639) — ✋ changes requested
- [#3712 test(swift-sdk): first swift sdk integration tests with local network](https://github.com/dashpay/platform/pull/3712) — 📝 draft
- [#3742 fix(platform-wallet): fix spv client deadlocking himself when trying to stop](https://github.com/dashpay/platform/pull/3742) — ✋ changes requested

<a id="zocolini-needs-action"></a>
#### Needs action (3)
- [#3479 test(swift-sdk): swift-sdk test updated and added to CI](https://github.com/dashpay/platform/pull/3479) — ✋ changes requested
- [#3639 feat(platform-wallet): external signable wallets](https://github.com/dashpay/platform/pull/3639) — ✋ changes requested
- [#3742 fix(platform-wallet): fix spv client deadlocking himself when trying to stop](https://github.com/dashpay/platform/pull/3742) — ✋ changes requested

<a id="zocolini-changes-requested"></a>
#### Changes Requested (3)
- [#3479 test(swift-sdk): swift-sdk test updated and added to CI](https://github.com/dashpay/platform/pull/3479) — ✋ changes requested
- [#3639 feat(platform-wallet): external signable wallets](https://github.com/dashpay/platform/pull/3639) — ✋ changes requested
- [#3742 fix(platform-wallet): fix spv client deadlocking himself when trying to stop](https://github.com/dashpay/platform/pull/3742) — ✋ changes requested

<a id="zocolini-draft"></a>
#### Draft (1)
- [#3712 test(swift-sdk): first swift sdk integration tests with local network](https://github.com/dashpay/platform/pull/3712) — 📝 draft

<a id="shumkov"></a>
### @shumkov
<a id="shumkov-open"></a>
#### Open (6)
- [#2392 feat(dashmate): single node local network [WIP\]](https://github.com/dashpay/platform/pull/2392) — 🔴 CI failing · 🐢 targets v2.0-dev
- [#2518 feat: token marketplace](https://github.com/dashpay/platform/pull/2518) — 📝 draft · ⏸ deferred
- [#2552 perf(dapi): fetch only specific unconfirmed transaction](https://github.com/dashpay/platform/pull/2552) — ⚠ merge conflict · 🔴 CI failing · 🐢 targets v2.1-dev
- [#3573 feat(dpp): unify JSON/Value conversion traits + comprehensive round-trip tests](https://github.com/dashpay/platform/pull/3573) — ⚠ merge conflict · ✋ changes requested · 🔴 CI failing
- [#3671 feat: platform-address funding from asset-lock proofs](https://github.com/dashpay/platform/pull/3671) — ✋ changes requested · 🔴 CI failing
- [#3732 [DON'T MERGE\] feat(drive-abci,dashmate): seed Orchard shielded pool at SDK_TEST_DATA devnet genesis](https://github.com/dashpay/platform/pull/3732) — 🔴 CI failing · 📝 draft

<a id="shumkov-needs-action"></a>
#### Needs action (2)
- [#3573 feat(dpp): unify JSON/Value conversion traits + comprehensive round-trip tests](https://github.com/dashpay/platform/pull/3573) — ⚠ merge conflict · ✋ changes requested · 🔴 CI failing
- [#3671 feat: platform-address funding from asset-lock proofs](https://github.com/dashpay/platform/pull/3671) — ✋ changes requested · 🔴 CI failing

<a id="shumkov-changes-requested"></a>
#### Changes Requested (2)
- [#3573 feat(dpp): unify JSON/Value conversion traits + comprehensive round-trip tests](https://github.com/dashpay/platform/pull/3573) — ⚠ merge conflict · ✋ changes requested · 🔴 CI failing
- [#3671 feat: platform-address funding from asset-lock proofs](https://github.com/dashpay/platform/pull/3671) — ✋ changes requested · 🔴 CI failing

<a id="shumkov-deferred"></a>
#### Deferred (1)
- [#2518 feat: token marketplace](https://github.com/dashpay/platform/pull/2518) — 📝 draft · ⏸ deferred

<a id="shumkov-draft"></a>
#### Draft (1)
- [#3732 [DON'T MERGE\] feat(drive-abci,dashmate): seed Orchard shielded pool at SDK_TEST_DATA devnet genesis](https://github.com/dashpay/platform/pull/3732) — 🔴 CI failing · 📝 draft

<a id="shumkov-stale"></a>
#### Stale (2)
- [#2392 feat(dashmate): single node local network [WIP\]](https://github.com/dashpay/platform/pull/2392) — 🔴 CI failing · 🐢 targets v2.0-dev
- [#2552 perf(dapi): fetch only specific unconfirmed transaction](https://github.com/dashpay/platform/pull/2552) — ⚠ merge conflict · 🔴 CI failing · 🐢 targets v2.1-dev

<a id="shumkov-ready-for-review"></a>
#### Ready for Review (3)
- [#3637 feat(platform-wallet): serde support](https://github.com/dashpay/platform/pull/3637) — by @Claudius-Maginificent
- [#3650 fix(sdk): address-sync no longer silently discards balance changes for post-snapshot addresses (Found-025)](https://github.com/dashpay/platform/pull/3650) — by @lklimek
- [#3721 test(rs-sdk): relocate DPNS network tests from src/ to tests/](https://github.com/dashpay/platform/pull/3721) — by @lklimek

<a id="llbartekll"></a>
### @llbartekll
<a id="llbartekll-open"></a>
#### Open (3)
- [#3481 feat(swift-sdk): expose rich transaction fields in WalletTransaction](https://github.com/dashpay/platform/pull/3481) — 1 unresolved (1 CodeRabbit) · 43 days stale · ⚠ merge conflict · 🔴 CI failing · 📝 draft
  - Top thread: "_⚠️ Potential issue_ \| _🟠 Major_" — 43 days old
- [#3560 test(swift-sdk): add testnet identity-discovery UI test](https://github.com/dashpay/platform/pull/3560) — ✋ changes requested
- [#3694 fix(swift-example-app): unhide Create Identity submit button and auto-dismiss sheet on success](https://github.com/dashpay/platform/pull/3694) — ✋ changes requested · 🔴 CI failing

<a id="llbartekll-needs-action"></a>
#### Needs action (2)
- [#3560 test(swift-sdk): add testnet identity-discovery UI test](https://github.com/dashpay/platform/pull/3560) — ✋ changes requested
- [#3694 fix(swift-example-app): unhide Create Identity submit button and auto-dismiss sheet on success](https://github.com/dashpay/platform/pull/3694) — ✋ changes requested · 🔴 CI failing

<a id="llbartekll-changes-requested"></a>
#### Changes Requested (2)
- [#3560 test(swift-sdk): add testnet identity-discovery UI test](https://github.com/dashpay/platform/pull/3560) — ✋ changes requested
- [#3694 fix(swift-example-app): unhide Create Identity submit button and auto-dismiss sheet on success](https://github.com/dashpay/platform/pull/3694) — ✋ changes requested · 🔴 CI failing

<a id="llbartekll-draft"></a>
#### Draft (1)
- [#3481 feat(swift-sdk): expose rich transaction fields in WalletTransaction](https://github.com/dashpay/platform/pull/3481) — 1 unresolved (1 CodeRabbit) · 43 days stale · ⚠ merge conflict · 🔴 CI failing · 📝 draft
  - Top thread: "_⚠️ Potential issue_ \| _🟠 Major_" — 43 days old

<a id="llbartekll-ready-for-review"></a>
#### Ready for Review (1)
- [#3733 fix(swift-sdk): tie shielded completion suppression to a sync generation](https://github.com/dashpay/platform/pull/3733) — by @QuantumExplorer

<a id="pshenmic"></a>
### @pshenmic
<a id="pshenmic-open"></a>
#### Open (4)
- [#2444 feat(sdk): add NFT actions in the JS Dash SDK](https://github.com/dashpay/platform/pull/2444) — 17 unresolved (5 CodeRabbit, 12 human) · 453 days stale · ⚠ merge conflict · ✋ changes requested · 🐢 targets v2.0-dev
  - Top thread: "_:warning: Potential issue_" — 453 days old
- [#2465 fix(sdk): replace seeds with ip address](https://github.com/dashpay/platform/pull/2465) — 7 unresolved (7 human) · 463 days stale · ✋ changes requested · 🐢 targets master
  - Top thread: "Why do we use testnet seeds for mainnet?" — 463 days old
- [#2446 feat(js-dapi-client): add contested resources query methods](https://github.com/dashpay/platform/pull/2446) — 5 unresolved (5 CodeRabbit) · 481 days stale · ⚠ merge conflict · 📝 draft · 🐢 targets v2.0-dev, untouched 472 days
  - Top thread: "_:warning: Potential issue_" — 481 days old
- [#2727 fix(drive): fix verification data contract verification logic with keeps history](https://github.com/dashpay/platform/pull/2727) — 🔴 CI failing · 🐢 targets v2.1-dev

<a id="pshenmic-stale"></a>
#### Stale (4)
- [#2444 feat(sdk): add NFT actions in the JS Dash SDK](https://github.com/dashpay/platform/pull/2444) — 17 unresolved (5 CodeRabbit, 12 human) · 453 days stale · ⚠ merge conflict · ✋ changes requested · 🐢 targets v2.0-dev
  - Top thread: "_:warning: Potential issue_" — 453 days old
- [#2465 fix(sdk): replace seeds with ip address](https://github.com/dashpay/platform/pull/2465) — 7 unresolved (7 human) · 463 days stale · ✋ changes requested · 🐢 targets master
  - Top thread: "Why do we use testnet seeds for mainnet?" — 463 days old
- [#2446 feat(js-dapi-client): add contested resources query methods](https://github.com/dashpay/platform/pull/2446) — 5 unresolved (5 CodeRabbit) · 481 days stale · ⚠ merge conflict · 📝 draft · 🐢 targets v2.0-dev, untouched 472 days
  - Top thread: "_:warning: Potential issue_" — 481 days old
- [#2727 fix(drive): fix verification data contract verification logic with keeps history](https://github.com/dashpay/platform/pull/2727) — 🔴 CI failing · 🐢 targets v2.1-dev

<a id="ogabrielides"></a>
### @ogabrielides
<a id="ogabrielides-open"></a>
#### Open (1)
- [#2486 feat(drive-abci): state sync - faster sync of new nodes](https://github.com/dashpay/platform/pull/2486) — ⏸ deferred

<a id="ogabrielides-deferred"></a>
#### Deferred (1)
- [#2486 feat(drive-abci): state sync - faster sync of new nodes](https://github.com/dashpay/platform/pull/2486) — ⏸ deferred

## Methodology
Generated nightly by [pr-hygiene](https://github.com/dashpay/stale_prs_are_bad). A thread counts as "unresolved" when it is open, not outdated, has a comment from someone other than the PR author, and the most recent comment is from a reviewer. **Dirty** = at least one such thread. **Unresolved Comments** = at least one such thread. **Changes Requested** = no unresolved threads but a reviewer's most recent review is CHANGES_REQUESTED (still blocking until someone re-approves or dismisses). **Deferred** = carries a configured deferred label (e.g. `postponed`) — visible but not counted toward unresolved-comment counts. **Stale** = targets a non-default branch OR hasn't been touched in the configured threshold (default 120 days, but clean PRs are never reclassified as stale). **Draft** = the PR is still marked draft on GitHub. **CI failing** = no unresolved comments, no changes-requested, but the latest commit's status check is failing. **Clean** = open, not draft, not deferred, not stale, no unresolved comments, no changes-requested, CI green. **Needs action** further requires changes-requested, merge conflict, or that the reviewer commented more recently than the author last pushed. **Ready for Review** counts clean PRs (authored by someone else) where this person owes a review. When a `review_routing` rule matches a PR's changed files, the routed reviewer IS the queue (explicit GitHub reviewers are ignored); a routed reviewer who has already submitted any review is excluded — their job is done. Configurable via [`https://github.com/dashpay/stale_prs_are_bad/blob/master/.pr-hygiene.yml`](https://github.com/dashpay/stale_prs_are_bad/blob/master/.pr-hygiene.yml)—edit defaults there.
