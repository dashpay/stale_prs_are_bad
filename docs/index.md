---
---
# PR Hygiene Report
*Last updated: 2026-06-05 07:33 UTC · commit b7f8400*

## Summary
- Open PRs: **52** (4 clean · 0 CI failing · 2 changes requested · 17 unresolved comments · 10 deferred · 11 draft · 8 stale)
- PRs needing author action: **18**
- Total unresolved comments: **132**

## Scoreboard
_Sort: unresolved-comments desc → needs-action desc → ready-for-review desc. Click any number to jump to the specific PRs it covers._

| Author | Open | Clean | CI failing | Unresolved Comments | Changes Requested | Deferred | Draft | Stale | Needs action | Total Unresolved Comments | Ready for Review | Δ |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| [@QuantumExplorer](#quantumexplorer) | [13](#quantumexplorer-open) | [2](#quantumexplorer-clean) | — | [7](#quantumexplorer-unresolved-comments) | — | [2](#quantumexplorer-deferred) | [1](#quantumexplorer-draft) | [1](#quantumexplorer-stale) | [7](#quantumexplorer-needs-action) | [38](#quantumexplorer-unresolved-comments) | [1](#quantumexplorer-ready-for-review) | ↑ 4 |
| [@PastaPastaPasta + (@thepastaclaw)](#pastapastapasta) | [5+(7)](#pastapastapasta-open) | — | — | [4+(0)](#pastapastapasta-unresolved-comments) | — | — | [1+(5)](#pastapastapasta-draft) | [0+(2)](#pastapastapasta-stale) | [4+(0)](#pastapastapasta-needs-action) | [10+(4)](#pastapastapasta-unresolved-comments) | — | ↓ 1 |
| [@lklimek + (@Claudius-Maginificent)](#lklimek) | [7+(7)](#lklimek-open) | — | — | [1+(3)](#lklimek-unresolved-comments) | — | [5+(0)](#lklimek-deferred) | [0+(1)](#lklimek-draft) | [1+(3)](#lklimek-stale) | [1+(2)](#lklimek-needs-action) | [4+(47)](#lklimek-unresolved-comments) | — | ↓ 2 |
| [@llbartekll](#llbartekll) | [2](#llbartekll-open) | — | — | [2](#llbartekll-unresolved-comments) | — | — | — | — | [2](#llbartekll-needs-action) | [3](#llbartekll-unresolved-comments) | [2](#llbartekll-ready-for-review) | — |
| [@shumkov](#shumkov) | [3](#shumkov-open) | — | — | — | [1](#shumkov-changes-requested) | [2](#shumkov-deferred) | — | — | [1](#shumkov-needs-action) | — | [1](#shumkov-ready-for-review) | ↓ 1 |
| [@ZocoLini](#zocolini) | [6](#zocolini-open) | [2](#zocolini-clean) | — | — | [1](#zocolini-changes-requested) | — | [3](#zocolini-draft) | — | [1](#zocolini-needs-action) | [21](#zocolini-unresolved-comments) | — | ↓ 2 |
| [@pshenmic](#pshenmic) | [1](#pshenmic-open) | — | — | — | — | — | — | [1](#pshenmic-stale) | — | [5](#pshenmic-unresolved-comments) | — | — |
| [@ogabrielides](#ogabrielides) | [1](#ogabrielides-open) | — | — | — | — | [1](#ogabrielides-deferred) | — | — | — | — | — | — |

## Per-author detail

<a id="quantumexplorer"></a>
### @QuantumExplorer
<a id="quantumexplorer-open"></a>
#### Open (13)
- [#3557 feat(platform-wallet): instrument wallet_manager RwLock behind lock-stats feature](https://github.com/dashpay/platform/pull/3557) — 11 unresolved (2 CodeRabbit, 9 human) · 38 days stale · ⚠ merge conflict
  - Top thread: "_⚠️ Potential issue_ \| _🟠 Major_" — 38 days old
- [#3740 feat: add time-range indexes for trending/leaderboard queries](https://github.com/dashpay/platform/pull/3740) — 5 unresolved (5 CodeRabbit) · 11 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing
  - Top thread: "_⚠️ Potential issue_ \| _🟠 Major_ \| _⚡ Quick win_" — 11 days old
- [#3792 fix(drive): authenticate boundary in compacted absence proofs](https://github.com/dashpay/platform/pull/3792) — 7 unresolved (7 human) · 1 days stale · ✋ changes requested
  - Top thread: "<!-- thepastaclaw-review v1 finding=5e3ebc92aecb dedupe=c7d872cefc90457c -->" — 1 days old
- [#3800 fix(drive): charge fees for unshield and shielded withdrawal](https://github.com/dashpay/platform/pull/3800) — 5 unresolved (3 CodeRabbit, 2 human) · 0 days stale · ✋ changes requested
  - Top thread: "_⚠️ Potential issue_ \| _🟡 Minor_ \| _⚡ Quick win_" — 0 days old
- [#3793 fix(drive): conserve credits in shield, debit only the shielded amount](https://github.com/dashpay/platform/pull/3793) — 6 unresolved (6 human) · 1 days stale
  - Top thread: "<!-- thepastaclaw-review v1 finding=563381bccfc4 dedupe=825462de0dd092bd -->" — 1 days old
- [#3772 fix(sdk): wallet-flow network fixes for SwiftExampleApp](https://github.com/dashpay/platform/pull/3772) — 3 unresolved (1 CodeRabbit, 2 human) · 4 days stale · ⚠ merge conflict
  - Top thread: "_🛠️ Refactor suggestion_ \| _🟠 Major_ \| _🏗️ Heavy lift_" — 4 days old
- [#3797 fix(platform-wallet): zeroize private keys when freeing preview rows](https://github.com/dashpay/platform/pull/3797) — 1 unresolved (1 human) · 1 days stale
  - Top thread: "<!-- thepastaclaw-review v1 finding=34578ec6f4bd dedupe=8800415a805a44ed -->" — 1 days old
- [#1834 feat(drive-abci)!: Statesync](https://github.com/dashpay/platform/pull/1834) — 📝 draft · ⏸ deferred
- [#3021 feat(platform): better contract state transitions](https://github.com/dashpay/platform/pull/3021) — 📝 draft · ⏸ deferred
- [#3417 feat(swift-sdk): use SPV-synced quorums for Platform proof verification](https://github.com/dashpay/platform/pull/3417) — ⚠ merge conflict · 🔴 CI failing · 📝 draft
- [#3784 docs: clarify address-sync catch-up nonce and buffer comments](https://github.com/dashpay/platform/pull/3784) — 🐢 targets fix/rs-sdk-address-sync-found-025
- [#3798 fix(rs-sdk-ffi): shrink signature allocation to len before leaking (capacity UB)](https://github.com/dashpay/platform/pull/3798)
- [#3799 fix(dpp): return error instead of panicking on storage-fee refund div-by-zero](https://github.com/dashpay/platform/pull/3799)

<a id="quantumexplorer-needs-action"></a>
#### Needs action (7)
- [#3557 feat(platform-wallet): instrument wallet_manager RwLock behind lock-stats feature](https://github.com/dashpay/platform/pull/3557) — 11 unresolved (2 CodeRabbit, 9 human) · 38 days stale · ⚠ merge conflict
  - Top thread: "_⚠️ Potential issue_ \| _🟠 Major_" — 38 days old
- [#3740 feat: add time-range indexes for trending/leaderboard queries](https://github.com/dashpay/platform/pull/3740) — 5 unresolved (5 CodeRabbit) · 11 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing
  - Top thread: "_⚠️ Potential issue_ \| _🟠 Major_ \| _⚡ Quick win_" — 11 days old
- [#3792 fix(drive): authenticate boundary in compacted absence proofs](https://github.com/dashpay/platform/pull/3792) — 7 unresolved (7 human) · 1 days stale · ✋ changes requested
  - Top thread: "<!-- thepastaclaw-review v1 finding=5e3ebc92aecb dedupe=c7d872cefc90457c -->" — 1 days old
- [#3800 fix(drive): charge fees for unshield and shielded withdrawal](https://github.com/dashpay/platform/pull/3800) — 5 unresolved (3 CodeRabbit, 2 human) · 0 days stale · ✋ changes requested
  - Top thread: "_⚠️ Potential issue_ \| _🟡 Minor_ \| _⚡ Quick win_" — 0 days old
- [#3793 fix(drive): conserve credits in shield, debit only the shielded amount](https://github.com/dashpay/platform/pull/3793) — 6 unresolved (6 human) · 1 days stale
  - Top thread: "<!-- thepastaclaw-review v1 finding=563381bccfc4 dedupe=825462de0dd092bd -->" — 1 days old
- [#3772 fix(sdk): wallet-flow network fixes for SwiftExampleApp](https://github.com/dashpay/platform/pull/3772) — 3 unresolved (1 CodeRabbit, 2 human) · 4 days stale · ⚠ merge conflict
  - Top thread: "_🛠️ Refactor suggestion_ \| _🟠 Major_ \| _🏗️ Heavy lift_" — 4 days old
- [#3797 fix(platform-wallet): zeroize private keys when freeing preview rows](https://github.com/dashpay/platform/pull/3797) — 1 unresolved (1 human) · 1 days stale
  - Top thread: "<!-- thepastaclaw-review v1 finding=34578ec6f4bd dedupe=8800415a805a44ed -->" — 1 days old

<a id="quantumexplorer-unresolved-comments"></a>
#### Unresolved Comments (7)
- [#3557 feat(platform-wallet): instrument wallet_manager RwLock behind lock-stats feature](https://github.com/dashpay/platform/pull/3557) — 11 unresolved (2 CodeRabbit, 9 human) · 38 days stale · ⚠ merge conflict
  - Top thread: "_⚠️ Potential issue_ \| _🟠 Major_" — 38 days old
- [#3740 feat: add time-range indexes for trending/leaderboard queries](https://github.com/dashpay/platform/pull/3740) — 5 unresolved (5 CodeRabbit) · 11 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing
  - Top thread: "_⚠️ Potential issue_ \| _🟠 Major_ \| _⚡ Quick win_" — 11 days old
- [#3792 fix(drive): authenticate boundary in compacted absence proofs](https://github.com/dashpay/platform/pull/3792) — 7 unresolved (7 human) · 1 days stale · ✋ changes requested
  - Top thread: "<!-- thepastaclaw-review v1 finding=5e3ebc92aecb dedupe=c7d872cefc90457c -->" — 1 days old
- [#3800 fix(drive): charge fees for unshield and shielded withdrawal](https://github.com/dashpay/platform/pull/3800) — 5 unresolved (3 CodeRabbit, 2 human) · 0 days stale · ✋ changes requested
  - Top thread: "_⚠️ Potential issue_ \| _🟡 Minor_ \| _⚡ Quick win_" — 0 days old
- [#3793 fix(drive): conserve credits in shield, debit only the shielded amount](https://github.com/dashpay/platform/pull/3793) — 6 unresolved (6 human) · 1 days stale
  - Top thread: "<!-- thepastaclaw-review v1 finding=563381bccfc4 dedupe=825462de0dd092bd -->" — 1 days old
- [#3772 fix(sdk): wallet-flow network fixes for SwiftExampleApp](https://github.com/dashpay/platform/pull/3772) — 3 unresolved (1 CodeRabbit, 2 human) · 4 days stale · ⚠ merge conflict
  - Top thread: "_🛠️ Refactor suggestion_ \| _🟠 Major_ \| _🏗️ Heavy lift_" — 4 days old
- [#3797 fix(platform-wallet): zeroize private keys when freeing preview rows](https://github.com/dashpay/platform/pull/3797) — 1 unresolved (1 human) · 1 days stale
  - Top thread: "<!-- thepastaclaw-review v1 finding=34578ec6f4bd dedupe=8800415a805a44ed -->" — 1 days old

<a id="quantumexplorer-deferred"></a>
#### Deferred (2)
- [#1834 feat(drive-abci)!: Statesync](https://github.com/dashpay/platform/pull/1834) — 📝 draft · ⏸ deferred
- [#3021 feat(platform): better contract state transitions](https://github.com/dashpay/platform/pull/3021) — 📝 draft · ⏸ deferred

<a id="quantumexplorer-draft"></a>
#### Draft (1)
- [#3417 feat(swift-sdk): use SPV-synced quorums for Platform proof verification](https://github.com/dashpay/platform/pull/3417) — ⚠ merge conflict · 🔴 CI failing · 📝 draft

<a id="quantumexplorer-stale"></a>
#### Stale (1)
- [#3784 docs: clarify address-sync catch-up nonce and buffer comments](https://github.com/dashpay/platform/pull/3784) — 🐢 targets fix/rs-sdk-address-sync-found-025

<a id="quantumexplorer-clean"></a>
#### Clean (2)
- [#3798 fix(rs-sdk-ffi): shrink signature allocation to len before leaking (capacity UB)](https://github.com/dashpay/platform/pull/3798)
- [#3799 fix(dpp): return error instead of panicking on storage-fee refund div-by-zero](https://github.com/dashpay/platform/pull/3799)

<a id="quantumexplorer-ready-for-review"></a>
#### Ready for Review (1)
- [#3778 ci(swift-sdk): remove swift-sdk artifact upload](https://github.com/dashpay/platform/pull/3778) — by @ZocoLini

<a id="pastapastapasta"></a>
### @PastaPastaPasta + (@thepastaclaw)
<a id="pastapastapasta-open"></a>
#### Open (12)
- [#3096 feat(sdk): add client-side validation to state transition construction methods](https://github.com/dashpay/platform/pull/3096) — via @thepastaclaw · 4 unresolved (2 CodeRabbit, 2 human) · 107 days stale · 📝 draft
  - Top thread: "_⚠️ Potential issue_ \| _🟡 Minor_" — 104 days old
- [#3680 refactor(dapi,dpp)!: move dapi-client and Identifier off Buffer to Uint8Array](https://github.com/dashpay/platform/pull/3680) — 5 unresolved (5 human) · 9 days stale · 🔴 CI failing
  - Top thread: "<!-- thepastaclaw-review v1 finding=1c101f507542 -->" — 9 days old
- [#3509 fix(platform): default omitted proved query limits](https://github.com/dashpay/platform/pull/3509) — 1 unresolved (1 human) · 33 days stale · ✋ changes requested · 🔴 CI failing
  - Top thread: "<!-- thepastaclaw-review v1 finding=d3df3e197206 -->" — 33 days old
- [#3462 fix(dpp)!: version-gate distribution function floating-point evaluation](https://github.com/dashpay/platform/pull/3462) — 3 unresolved (3 human) · 4 days stale · 🔴 CI failing
  - Top thread: "<!-- thepastaclaw-review v1 finding=7d40971bd5ff dedupe=2846c55b07c76472 -->" — 4 days old
- [#2988 feat(wasm-sdk): auto-generate entropy for document creation when not provided](https://github.com/dashpay/platform/pull/2988) — 1 unresolved (1 human) · 15 days stale · 🔴 CI failing
  - Top thread: "<!-- thepastaclaw-review v1 finding=30cbefe1c9ef -->" — 15 days old
- [#3091 feat(wasm-sdk): add prepare_* APIs for idempotent document state transitions](https://github.com/dashpay/platform/pull/3091) — via @thepastaclaw · ✋ changes requested · 📝 draft
- [#3092 feat(rs-sdk): expose transition hash from state transition methods](https://github.com/dashpay/platform/pull/3092) — via @thepastaclaw · ✋ changes requested · 📝 draft
- [#3133 fix(sdk): validate batch base structure before signing](https://github.com/dashpay/platform/pull/3133) — via @thepastaclaw · ✋ changes requested · 📝 draft
- [#3635 ci: tolerate book preview comment permission errors](https://github.com/dashpay/platform/pull/3635) — via @thepastaclaw · 📝 draft
- [#3669 fix(swift-sdk): wait for SDK rebuild before wallet activation](https://github.com/dashpay/platform/pull/3669) — via @thepastaclaw · 📝 draft · 🐢 targets claude/swift-xcuitest-wallet-persistence
- [#3689 refactor(sdk,wallet-lib,test-suite)!: convert dapi-client + wallet-lib + js-dash-sdk + platform-test-suite to ESM](https://github.com/dashpay/platform/pull/3689) — 📝 draft
- [#3722 fix(rs-sdk): replay chained unknown addresses](https://github.com/dashpay/platform/pull/3722) — via @thepastaclaw · 📝 draft · 🐢 targets fix/rs-sdk-address-sync-found-025

<a id="pastapastapasta-needs-action"></a>
#### Needs action (4)
- [#3680 refactor(dapi,dpp)!: move dapi-client and Identifier off Buffer to Uint8Array](https://github.com/dashpay/platform/pull/3680) — 5 unresolved (5 human) · 9 days stale · 🔴 CI failing
  - Top thread: "<!-- thepastaclaw-review v1 finding=1c101f507542 -->" — 9 days old
- [#3509 fix(platform): default omitted proved query limits](https://github.com/dashpay/platform/pull/3509) — 1 unresolved (1 human) · 33 days stale · ✋ changes requested · 🔴 CI failing
  - Top thread: "<!-- thepastaclaw-review v1 finding=d3df3e197206 -->" — 33 days old
- [#3462 fix(dpp)!: version-gate distribution function floating-point evaluation](https://github.com/dashpay/platform/pull/3462) — 3 unresolved (3 human) · 4 days stale · 🔴 CI failing
  - Top thread: "<!-- thepastaclaw-review v1 finding=7d40971bd5ff dedupe=2846c55b07c76472 -->" — 4 days old
- [#2988 feat(wasm-sdk): auto-generate entropy for document creation when not provided](https://github.com/dashpay/platform/pull/2988) — 1 unresolved (1 human) · 15 days stale · 🔴 CI failing
  - Top thread: "<!-- thepastaclaw-review v1 finding=30cbefe1c9ef -->" — 15 days old

<a id="pastapastapasta-unresolved-comments"></a>
#### Unresolved Comments (4)
- [#3680 refactor(dapi,dpp)!: move dapi-client and Identifier off Buffer to Uint8Array](https://github.com/dashpay/platform/pull/3680) — 5 unresolved (5 human) · 9 days stale · 🔴 CI failing
  - Top thread: "<!-- thepastaclaw-review v1 finding=1c101f507542 -->" — 9 days old
- [#3509 fix(platform): default omitted proved query limits](https://github.com/dashpay/platform/pull/3509) — 1 unresolved (1 human) · 33 days stale · ✋ changes requested · 🔴 CI failing
  - Top thread: "<!-- thepastaclaw-review v1 finding=d3df3e197206 -->" — 33 days old
- [#3462 fix(dpp)!: version-gate distribution function floating-point evaluation](https://github.com/dashpay/platform/pull/3462) — 3 unresolved (3 human) · 4 days stale · 🔴 CI failing
  - Top thread: "<!-- thepastaclaw-review v1 finding=7d40971bd5ff dedupe=2846c55b07c76472 -->" — 4 days old
- [#2988 feat(wasm-sdk): auto-generate entropy for document creation when not provided](https://github.com/dashpay/platform/pull/2988) — 1 unresolved (1 human) · 15 days stale · 🔴 CI failing
  - Top thread: "<!-- thepastaclaw-review v1 finding=30cbefe1c9ef -->" — 15 days old

<a id="pastapastapasta-draft"></a>
#### Draft (6)
- [#3096 feat(sdk): add client-side validation to state transition construction methods](https://github.com/dashpay/platform/pull/3096) — via @thepastaclaw · 4 unresolved (2 CodeRabbit, 2 human) · 107 days stale · 📝 draft
  - Top thread: "_⚠️ Potential issue_ \| _🟡 Minor_" — 104 days old
- [#3091 feat(wasm-sdk): add prepare_* APIs for idempotent document state transitions](https://github.com/dashpay/platform/pull/3091) — via @thepastaclaw · ✋ changes requested · 📝 draft
- [#3092 feat(rs-sdk): expose transition hash from state transition methods](https://github.com/dashpay/platform/pull/3092) — via @thepastaclaw · ✋ changes requested · 📝 draft
- [#3133 fix(sdk): validate batch base structure before signing](https://github.com/dashpay/platform/pull/3133) — via @thepastaclaw · ✋ changes requested · 📝 draft
- [#3635 ci: tolerate book preview comment permission errors](https://github.com/dashpay/platform/pull/3635) — via @thepastaclaw · 📝 draft
- [#3689 refactor(sdk,wallet-lib,test-suite)!: convert dapi-client + wallet-lib + js-dash-sdk + platform-test-suite to ESM](https://github.com/dashpay/platform/pull/3689) — 📝 draft

<a id="pastapastapasta-stale"></a>
#### Stale (2)
- [#3669 fix(swift-sdk): wait for SDK rebuild before wallet activation](https://github.com/dashpay/platform/pull/3669) — via @thepastaclaw · 📝 draft · 🐢 targets claude/swift-xcuitest-wallet-persistence
- [#3722 fix(rs-sdk): replay chained unknown addresses](https://github.com/dashpay/platform/pull/3722) — via @thepastaclaw · 📝 draft · 🐢 targets fix/rs-sdk-address-sync-found-025

<a id="lklimek"></a>
### @lklimek + (@Claudius-Maginificent)
<a id="lklimek-open"></a>
#### Open (14)
- [#3549 test(platform-wallet): e2e framework + full test suite — triage pins, Found-*/PA-* guards, fail-closed persist, Stage-2 merge](https://github.com/dashpay/platform/pull/3549) — via @Claudius-Maginificent · 20 unresolved (20 human) · 33 days stale · ⚠ merge conflict · 📝 draft
  - Top thread: "<!-- thepastaclaw-review v1 finding=a3725e9d50ca -->" — 33 days old
- [#3625 feat(platform-wallet)!: add platform-wallet-storage crate (sqlite persister)](https://github.com/dashpay/platform/pull/3625) — via @Claudius-Maginificent · 16 unresolved (5 CodeRabbit, 11 human) · 3 days stale
  - Top thread: "_⚠️ Potential issue_ \| _🟠 Major_ \| _⚡ Quick win_" — 2 days old
- [#3781 fix(dpp)!: make platform/orchard address decoders network-agnostic](https://github.com/dashpay/platform/pull/3781) — via @Claudius-Maginificent · 8 unresolved (3 CodeRabbit, 5 human) · 3 days stale · 🔴 CI failing
  - Top thread: "_⚠️ Potential issue_ \| _🟡 Minor_ \| _⚡ Quick win_" — 3 days old
- [#3585 fix: case-insensitive .dash resolution + UTXO double-spend race hardening](https://github.com/dashpay/platform/pull/3585) — via @Claudius-Maginificent · 3 unresolved (3 human) · 7 days stale · ✋ changes requested
  - Top thread: "<!-- thepastaclaw-review v1 finding=b44c47f34c15 dedupe=f7000f24c7c02c8d -->" — 7 days old
- [#3650 fix(sdk): address-sync no longer silently discards balance changes for post-snapshot addresses (Found-025)](https://github.com/dashpay/platform/pull/3650) — 4 unresolved (4 human) · 7 days stale
  - Top thread: "<!-- thepastaclaw-review v1 finding=611ad120f83c dedupe=8e6cde99fecb9bdf -->" — 7 days old
- [#2679 fix(drive-abci): don't panic on unsupported version error](https://github.com/dashpay/platform/pull/2679) — ⏸ deferred
- [#2795 feat(platform): add real-time platform event subscriptions via gRPC streaming](https://github.com/dashpay/platform/pull/2795) — ⏸ deferred
- [#2993 feat: identity reference validation](https://github.com/dashpay/platform/pull/2993) — 📝 draft · ⏸ deferred
- [#3009 feat: contract reference validation](https://github.com/dashpay/platform/pull/3009) — 📝 draft · ⏸ deferred
- [#3032 feat: document creation restriction by group membership](https://github.com/dashpay/platform/pull/3032) — 📝 draft · ⏸ deferred
- [#3692 feat(platform-wallet): watch-only rehydration from persistor (seedless load)](https://github.com/dashpay/platform/pull/3692) — via @Claudius-Maginificent · ⚠ merge conflict · 📝 draft · 🐢 targets feat/platform-wallet-sqlite-persistor
- [#3727 test(platform-wallet): shielded (Orchard) e2e suite — spec + Wave H harness](https://github.com/dashpay/platform/pull/3727) — 📝 draft · 🐢 targets feat/rs-platform-wallet-e2e
- [#3750 feat(platform-wallet)!: [NO MERGE\] consumer hardening — CODE-001/003-callsite/017/018 + PROJ-001 FFI + CODE-008/012/013](https://github.com/dashpay/platform/pull/3750) — via @Claudius-Maginificent · ⚠ merge conflict · 📝 draft · 🐢 targets feat/platform-wallet-sqlite-persistor
- [#3770 fix(platform-wallet): atomic multi-pool address reservations (receive + core BIP44 receive/change) + change-loop refactor](https://github.com/dashpay/platform/pull/3770) — via @Claudius-Maginificent · 📝 draft · 🐢 targets fix/dpns-case-and-utxo-race-v3.1-dev

<a id="lklimek-needs-action"></a>
#### Needs action (3)
- [#3781 fix(dpp)!: make platform/orchard address decoders network-agnostic](https://github.com/dashpay/platform/pull/3781) — via @Claudius-Maginificent · 8 unresolved (3 CodeRabbit, 5 human) · 3 days stale · 🔴 CI failing
  - Top thread: "_⚠️ Potential issue_ \| _🟡 Minor_ \| _⚡ Quick win_" — 3 days old
- [#3585 fix: case-insensitive .dash resolution + UTXO double-spend race hardening](https://github.com/dashpay/platform/pull/3585) — via @Claudius-Maginificent · 3 unresolved (3 human) · 7 days stale · ✋ changes requested
  - Top thread: "<!-- thepastaclaw-review v1 finding=b44c47f34c15 dedupe=f7000f24c7c02c8d -->" — 7 days old
- [#3650 fix(sdk): address-sync no longer silently discards balance changes for post-snapshot addresses (Found-025)](https://github.com/dashpay/platform/pull/3650) — 4 unresolved (4 human) · 7 days stale
  - Top thread: "<!-- thepastaclaw-review v1 finding=611ad120f83c dedupe=8e6cde99fecb9bdf -->" — 7 days old

<a id="lklimek-unresolved-comments"></a>
#### Unresolved Comments (4)
- [#3625 feat(platform-wallet)!: add platform-wallet-storage crate (sqlite persister)](https://github.com/dashpay/platform/pull/3625) — via @Claudius-Maginificent · 16 unresolved (5 CodeRabbit, 11 human) · 3 days stale
  - Top thread: "_⚠️ Potential issue_ \| _🟠 Major_ \| _⚡ Quick win_" — 2 days old
- [#3781 fix(dpp)!: make platform/orchard address decoders network-agnostic](https://github.com/dashpay/platform/pull/3781) — via @Claudius-Maginificent · 8 unresolved (3 CodeRabbit, 5 human) · 3 days stale · 🔴 CI failing
  - Top thread: "_⚠️ Potential issue_ \| _🟡 Minor_ \| _⚡ Quick win_" — 3 days old
- [#3585 fix: case-insensitive .dash resolution + UTXO double-spend race hardening](https://github.com/dashpay/platform/pull/3585) — via @Claudius-Maginificent · 3 unresolved (3 human) · 7 days stale · ✋ changes requested
  - Top thread: "<!-- thepastaclaw-review v1 finding=b44c47f34c15 dedupe=f7000f24c7c02c8d -->" — 7 days old
- [#3650 fix(sdk): address-sync no longer silently discards balance changes for post-snapshot addresses (Found-025)](https://github.com/dashpay/platform/pull/3650) — 4 unresolved (4 human) · 7 days stale
  - Top thread: "<!-- thepastaclaw-review v1 finding=611ad120f83c dedupe=8e6cde99fecb9bdf -->" — 7 days old

<a id="lklimek-deferred"></a>
#### Deferred (5)
- [#2679 fix(drive-abci): don't panic on unsupported version error](https://github.com/dashpay/platform/pull/2679) — ⏸ deferred
- [#2795 feat(platform): add real-time platform event subscriptions via gRPC streaming](https://github.com/dashpay/platform/pull/2795) — ⏸ deferred
- [#2993 feat: identity reference validation](https://github.com/dashpay/platform/pull/2993) — 📝 draft · ⏸ deferred
- [#3009 feat: contract reference validation](https://github.com/dashpay/platform/pull/3009) — 📝 draft · ⏸ deferred
- [#3032 feat: document creation restriction by group membership](https://github.com/dashpay/platform/pull/3032) — 📝 draft · ⏸ deferred

<a id="lklimek-draft"></a>
#### Draft (1)
- [#3549 test(platform-wallet): e2e framework + full test suite — triage pins, Found-*/PA-* guards, fail-closed persist, Stage-2 merge](https://github.com/dashpay/platform/pull/3549) — via @Claudius-Maginificent · 20 unresolved (20 human) · 33 days stale · ⚠ merge conflict · 📝 draft
  - Top thread: "<!-- thepastaclaw-review v1 finding=a3725e9d50ca -->" — 33 days old

<a id="lklimek-stale"></a>
#### Stale (4)
- [#3692 feat(platform-wallet): watch-only rehydration from persistor (seedless load)](https://github.com/dashpay/platform/pull/3692) — via @Claudius-Maginificent · ⚠ merge conflict · 📝 draft · 🐢 targets feat/platform-wallet-sqlite-persistor
- [#3727 test(platform-wallet): shielded (Orchard) e2e suite — spec + Wave H harness](https://github.com/dashpay/platform/pull/3727) — 📝 draft · 🐢 targets feat/rs-platform-wallet-e2e
- [#3750 feat(platform-wallet)!: [NO MERGE\] consumer hardening — CODE-001/003-callsite/017/018 + PROJ-001 FFI + CODE-008/012/013](https://github.com/dashpay/platform/pull/3750) — via @Claudius-Maginificent · ⚠ merge conflict · 📝 draft · 🐢 targets feat/platform-wallet-sqlite-persistor
- [#3770 fix(platform-wallet): atomic multi-pool address reservations (receive + core BIP44 receive/change) + change-loop refactor](https://github.com/dashpay/platform/pull/3770) — via @Claudius-Maginificent · 📝 draft · 🐢 targets fix/dpns-case-and-utxo-race-v3.1-dev

<a id="llbartekll"></a>
### @llbartekll
<a id="llbartekll-open"></a>
#### Open (2)
- [#3560 test(swift-sdk): add testnet identity-discovery UI test](https://github.com/dashpay/platform/pull/3560) — 2 unresolved (2 human) · 4 days stale · ✋ changes requested · 🔴 CI failing
  - Top thread: "<!-- thepastaclaw-review v1 finding=379c84ed00e1 dedupe=08a70e07f6e9ae45 -->" — 4 days old
- [#3694 fix(swift-example-app): unhide Create Identity submit button and auto-dismiss sheet on success](https://github.com/dashpay/platform/pull/3694) — 1 unresolved (1 human) · 4 days stale · ✋ changes requested
  - Top thread: "<!-- thepastaclaw-review v1 finding=e3a849e745d0 dedupe=d76d5740c5bc0f94 -->" — 4 days old

<a id="llbartekll-needs-action"></a>
#### Needs action (2)
- [#3560 test(swift-sdk): add testnet identity-discovery UI test](https://github.com/dashpay/platform/pull/3560) — 2 unresolved (2 human) · 4 days stale · ✋ changes requested · 🔴 CI failing
  - Top thread: "<!-- thepastaclaw-review v1 finding=379c84ed00e1 dedupe=08a70e07f6e9ae45 -->" — 4 days old
- [#3694 fix(swift-example-app): unhide Create Identity submit button and auto-dismiss sheet on success](https://github.com/dashpay/platform/pull/3694) — 1 unresolved (1 human) · 4 days stale · ✋ changes requested
  - Top thread: "<!-- thepastaclaw-review v1 finding=e3a849e745d0 dedupe=d76d5740c5bc0f94 -->" — 4 days old

<a id="llbartekll-unresolved-comments"></a>
#### Unresolved Comments (2)
- [#3560 test(swift-sdk): add testnet identity-discovery UI test](https://github.com/dashpay/platform/pull/3560) — 2 unresolved (2 human) · 4 days stale · ✋ changes requested · 🔴 CI failing
  - Top thread: "<!-- thepastaclaw-review v1 finding=379c84ed00e1 dedupe=08a70e07f6e9ae45 -->" — 4 days old
- [#3694 fix(swift-example-app): unhide Create Identity submit button and auto-dismiss sheet on success](https://github.com/dashpay/platform/pull/3694) — 1 unresolved (1 human) · 4 days stale · ✋ changes requested
  - Top thread: "<!-- thepastaclaw-review v1 finding=e3a849e745d0 dedupe=d76d5740c5bc0f94 -->" — 4 days old

<a id="llbartekll-ready-for-review"></a>
#### Ready for Review (2)
- [#3778 ci(swift-sdk): remove swift-sdk artifact upload](https://github.com/dashpay/platform/pull/3778) — by @ZocoLini
- [#3785 feat(swift-sdk): iOS simluator writes logs to disk](https://github.com/dashpay/platform/pull/3785) — by @ZocoLini

<a id="shumkov"></a>
### @shumkov
<a id="shumkov-open"></a>
#### Open (3)
- [#2392 feat(dashmate): single node local network [WIP\]](https://github.com/dashpay/platform/pull/2392) — ⏸ deferred
- [#2518 feat: token marketplace](https://github.com/dashpay/platform/pull/2518) — 📝 draft · ⏸ deferred
- [#3573 feat(dpp): unify JSON/Value conversion traits + comprehensive round-trip tests](https://github.com/dashpay/platform/pull/3573) — ⚠ merge conflict · ✋ changes requested · 🔴 CI failing

<a id="shumkov-needs-action"></a>
#### Needs action (1)
- [#3573 feat(dpp): unify JSON/Value conversion traits + comprehensive round-trip tests](https://github.com/dashpay/platform/pull/3573) — ⚠ merge conflict · ✋ changes requested · 🔴 CI failing

<a id="shumkov-changes-requested"></a>
#### Changes Requested (1)
- [#3573 feat(dpp): unify JSON/Value conversion traits + comprehensive round-trip tests](https://github.com/dashpay/platform/pull/3573) — ⚠ merge conflict · ✋ changes requested · 🔴 CI failing

<a id="shumkov-deferred"></a>
#### Deferred (2)
- [#2392 feat(dashmate): single node local network [WIP\]](https://github.com/dashpay/platform/pull/2392) — ⏸ deferred
- [#2518 feat: token marketplace](https://github.com/dashpay/platform/pull/2518) — 📝 draft · ⏸ deferred

<a id="shumkov-ready-for-review"></a>
#### Ready for Review (1)
- [#3798 fix(rs-sdk-ffi): shrink signature allocation to len before leaking (capacity UB)](https://github.com/dashpay/platform/pull/3798) — by @QuantumExplorer

<a id="zocolini"></a>
### @ZocoLini
<a id="zocolini-open"></a>
#### Open (6)
- [#3712 test(swift-sdk): first swift sdk integration tests with local network](https://github.com/dashpay/platform/pull/3712) — 21 unresolved (5 CodeRabbit, 16 human) · 9 days stale · ⚠ merge conflict · 📝 draft
  - Top thread: "_⚠️ Potential issue_ \| _🟠 Major_ \| _⚡ Quick win_" — 9 days old
- [#3639 feat(platform-wallet): external signable wallets](https://github.com/dashpay/platform/pull/3639) — ✋ changes requested
- [#3777 fix(swift-sdk): fixed mempool tx categorization after restart](https://github.com/dashpay/platform/pull/3777) — ✋ changes requested · 📝 draft
- [#3778 ci(swift-sdk): remove swift-sdk artifact upload](https://github.com/dashpay/platform/pull/3778)
- [#3785 feat(swift-sdk): iOS simluator writes logs to disk](https://github.com/dashpay/platform/pull/3785)
- [#3788 chore: bump rust-dashcore to rev without Network in dashcore::Address](https://github.com/dashpay/platform/pull/3788) — 📝 draft

<a id="zocolini-needs-action"></a>
#### Needs action (1)
- [#3639 feat(platform-wallet): external signable wallets](https://github.com/dashpay/platform/pull/3639) — ✋ changes requested

<a id="zocolini-changes-requested"></a>
#### Changes Requested (1)
- [#3639 feat(platform-wallet): external signable wallets](https://github.com/dashpay/platform/pull/3639) — ✋ changes requested

<a id="zocolini-draft"></a>
#### Draft (3)
- [#3712 test(swift-sdk): first swift sdk integration tests with local network](https://github.com/dashpay/platform/pull/3712) — 21 unresolved (5 CodeRabbit, 16 human) · 9 days stale · ⚠ merge conflict · 📝 draft
  - Top thread: "_⚠️ Potential issue_ \| _🟠 Major_ \| _⚡ Quick win_" — 9 days old
- [#3777 fix(swift-sdk): fixed mempool tx categorization after restart](https://github.com/dashpay/platform/pull/3777) — ✋ changes requested · 📝 draft
- [#3788 chore: bump rust-dashcore to rev without Network in dashcore::Address](https://github.com/dashpay/platform/pull/3788) — 📝 draft

<a id="zocolini-clean"></a>
#### Clean (2)
- [#3778 ci(swift-sdk): remove swift-sdk artifact upload](https://github.com/dashpay/platform/pull/3778)
- [#3785 feat(swift-sdk): iOS simluator writes logs to disk](https://github.com/dashpay/platform/pull/3785)

<a id="pshenmic"></a>
### @pshenmic
<a id="pshenmic-open"></a>
#### Open (1)
- [#2446 feat(js-dapi-client): add contested resources query methods](https://github.com/dashpay/platform/pull/2446) — 5 unresolved (5 CodeRabbit) · 492 days stale · ⚠ merge conflict · 📝 draft · 🐢 targets v2.0-dev, untouched 482 days
  - Top thread: "_:warning: Potential issue_" — 492 days old

<a id="pshenmic-stale"></a>
#### Stale (1)
- [#2446 feat(js-dapi-client): add contested resources query methods](https://github.com/dashpay/platform/pull/2446) — 5 unresolved (5 CodeRabbit) · 492 days stale · ⚠ merge conflict · 📝 draft · 🐢 targets v2.0-dev, untouched 482 days
  - Top thread: "_:warning: Potential issue_" — 492 days old

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
