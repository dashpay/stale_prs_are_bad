---
---
# PR Hygiene Report
*Last updated: 2026-07-07 00:41 UTC · commit a2f16c6*

## Summary
- Open PRs: **56** (8 clean · 1 CI failing · 3 changes requested · 13 unresolved comments · 10 deferred · 10 draft · 11 stale)
- PRs needing author action: **17**
- Total unresolved comments: **80**

## Scoreboard
_Sort: unresolved-comments desc → needs-action desc → ready-for-review desc. Click any number to jump to the specific PRs it covers._

| Author | Open | Clean | CI failing | Unresolved Comments | Changes Requested | Deferred | Draft | Stale | Needs action | Total Unresolved Comments | Ready for Review | Δ |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| [@PastaPastaPasta + (@thepastaclaw)](#pastapastapasta) | [5+(11)](#pastapastapasta-open) | [0+(2)](#pastapastapasta-clean) | — | [4+(1)](#pastapastapasta-unresolved-comments) | — | — | [1+(4)](#pastapastapasta-draft) | [0+(4)](#pastapastapasta-stale) | [4+(0)](#pastapastapasta-needs-action) | [10+(4)](#pastapastapasta-unresolved-comments) | — | — |
| [@QuantumExplorer](#quantumexplorer) | [7](#quantumexplorer-open) | — | — | [3](#quantumexplorer-unresolved-comments) | — | [2](#quantumexplorer-deferred) | [1](#quantumexplorer-draft) | [1](#quantumexplorer-stale) | [3](#quantumexplorer-needs-action) | [9](#quantumexplorer-unresolved-comments) | [3](#quantumexplorer-ready-for-review) | ↑ 2 |
| [@llbartekll](#llbartekll) | [3](#llbartekll-open) | — | — | [3](#llbartekll-unresolved-comments) | — | — | — | — | [3](#llbartekll-needs-action) | [5](#llbartekll-unresolved-comments) | [2](#llbartekll-ready-for-review) | — |
| [@shumkov](#shumkov) | [8](#shumkov-open) | — | — | [1](#shumkov-unresolved-comments) | [1](#shumkov-changes-requested) | [2](#shumkov-deferred) | [1](#shumkov-draft) | [3](#shumkov-stale) | [2](#shumkov-needs-action) | [17](#shumkov-unresolved-comments) | [1](#shumkov-ready-for-review) | ↓ 1 |
| [@thephez](#thephez) | [3](#thephez-open) | — | [1](#thephez-ci-failing) | [1](#thephez-unresolved-comments) | — | — | [1](#thephez-draft) | — | [1](#thephez-needs-action) | [3](#thephez-unresolved-comments) | — | — |
| [@ZocoLini](#zocolini) | [5](#zocolini-open) | [3](#zocolini-clean) | — | — | [2](#zocolini-changes-requested) | — | — | — | [3](#zocolini-needs-action) | — | — | ↑ 1 |
| [@lklimek + (@Claudius-Maginificent)](#lklimek) | [5+(6)](#lklimek-open) | [0+(2)](#lklimek-clean) | — | — | — | [5+(0)](#lklimek-deferred) | [0+(2)](#lklimek-draft) | [0+(2)](#lklimek-stale) | [0+(1)](#lklimek-needs-action) | [0+(27)](#lklimek-unresolved-comments) | [1+(0)](#lklimek-ready-for-review) | — |
| [@pshenmic](#pshenmic) | [1](#pshenmic-open) | — | — | — | — | — | — | [1](#pshenmic-stale) | — | [5](#pshenmic-unresolved-comments) | — | — |
| [@densmirnov](#densmirnov) | [1](#densmirnov-open) | [1](#densmirnov-clean) | — | — | — | — | — | — | — | — | — | — |
| [@ogabrielides](#ogabrielides) | [1](#ogabrielides-open) | — | — | — | — | [1](#ogabrielides-deferred) | — | — | — | — | — | — |

## Per-author detail

<a id="pastapastapasta"></a>
### @PastaPastaPasta + (@thepastaclaw)
<a id="pastapastapasta-open"></a>
#### Open (16)
- [#3096 feat(sdk): add client-side validation to state transition construction methods](https://github.com/dashpay/platform/pull/3096) — via @thepastaclaw · 4 unresolved (2 CodeRabbit, 2 human) · 139 days stale
  - Top thread: "_⚠️ Potential issue_ \| _🟡 Minor_" — 136 days old
- [#3680 refactor(dapi,dpp)!: move dapi-client and Identifier off Buffer to Uint8Array](https://github.com/dashpay/platform/pull/3680) — 5 unresolved (5 human) · 41 days stale · 🔴 CI failing
  - Top thread: "<!-- thepastaclaw-review v1 finding=1c101f507542 -->" — 41 days old
- [#3462 fix(dpp)!: version-gate distribution function floating-point evaluation](https://github.com/dashpay/platform/pull/3462) — 3 unresolved (3 human) · 35 days stale · 🔴 CI failing
  - Top thread: "<!-- thepastaclaw-review v1 finding=7d40971bd5ff dedupe=2846c55b07c76472 -->" — 35 days old
- [#3509 fix(platform): default omitted proved query limits](https://github.com/dashpay/platform/pull/3509) — 1 unresolved (1 human) · 65 days stale · ✋ changes requested · 🔴 CI failing
  - Top thread: "<!-- thepastaclaw-review v1 finding=d3df3e197206 -->" — 65 days old
- [#2988 feat(wasm-sdk): auto-generate entropy for document creation when not provided](https://github.com/dashpay/platform/pull/2988) — 1 unresolved (1 human) · 46 days stale · 🔴 CI failing
  - Top thread: "<!-- thepastaclaw-review v1 finding=30cbefe1c9ef -->" — 46 days old
- [#3635 ci: tolerate book preview comment permission errors](https://github.com/dashpay/platform/pull/3635) — via @thepastaclaw
- [#3689 refactor(sdk,wallet-lib,test-suite)!: convert dapi-client + wallet-lib + js-dash-sdk + platform-test-suite to ESM](https://github.com/dashpay/platform/pull/3689) — ⚠ merge conflict · 📝 draft
- [#3898 fix(dashmate): re-sync stale 3.x Drive and rs-dapi images](https://github.com/dashpay/platform/pull/3898) — via @thepastaclaw · 🐢 targets v4.0-dev
- [#3930 fix(platform): reject keep-history document deletes cleanly](https://github.com/dashpay/platform/pull/3930) — via @thepastaclaw · 🔴 CI failing · 🐢 targets v4.0-dev
- [#3938 test(rs-sdk): expect network floor in mock sdk seed test](https://github.com/dashpay/platform/pull/3938) — via @thepastaclaw
- [#3965 fix(drive-abci): use testnet Core RPC port in env](https://github.com/dashpay/platform/pull/3965) — via @thepastaclaw · 🐢 targets v4.0-dev
- [#3967 feat(wasm-sdk): expose tiered token pricing in setPrice](https://github.com/dashpay/platform/pull/3967) — via @thepastaclaw · 📝 draft
- [#3993 test(swift-sdk): assert transaction decoder error cases](https://github.com/dashpay/platform/pull/3993) — via @thepastaclaw · 🐢 targets feat/core-tx-decode-ffi
- [#4000 fix(dpp): report main control group authorization](https://github.com/dashpay/platform/pull/4000) — via @thepastaclaw · 📝 draft
- [#4015 fix(swift-example-app): gate identity resumes by funding type](https://github.com/dashpay/platform/pull/4015) — via @thepastaclaw · 📝 draft
- [#4016 fix(platform-wallet): close asset-lock resume broadcast race](https://github.com/dashpay/platform/pull/4016) — via @thepastaclaw · 📝 draft

<a id="pastapastapasta-needs-action"></a>
#### Needs action (4)
- [#3680 refactor(dapi,dpp)!: move dapi-client and Identifier off Buffer to Uint8Array](https://github.com/dashpay/platform/pull/3680) — 5 unresolved (5 human) · 41 days stale · 🔴 CI failing
  - Top thread: "<!-- thepastaclaw-review v1 finding=1c101f507542 -->" — 41 days old
- [#3462 fix(dpp)!: version-gate distribution function floating-point evaluation](https://github.com/dashpay/platform/pull/3462) — 3 unresolved (3 human) · 35 days stale · 🔴 CI failing
  - Top thread: "<!-- thepastaclaw-review v1 finding=7d40971bd5ff dedupe=2846c55b07c76472 -->" — 35 days old
- [#3509 fix(platform): default omitted proved query limits](https://github.com/dashpay/platform/pull/3509) — 1 unresolved (1 human) · 65 days stale · ✋ changes requested · 🔴 CI failing
  - Top thread: "<!-- thepastaclaw-review v1 finding=d3df3e197206 -->" — 65 days old
- [#2988 feat(wasm-sdk): auto-generate entropy for document creation when not provided](https://github.com/dashpay/platform/pull/2988) — 1 unresolved (1 human) · 46 days stale · 🔴 CI failing
  - Top thread: "<!-- thepastaclaw-review v1 finding=30cbefe1c9ef -->" — 46 days old

<a id="pastapastapasta-unresolved-comments"></a>
#### Unresolved Comments (5)
- [#3096 feat(sdk): add client-side validation to state transition construction methods](https://github.com/dashpay/platform/pull/3096) — via @thepastaclaw · 4 unresolved (2 CodeRabbit, 2 human) · 139 days stale
  - Top thread: "_⚠️ Potential issue_ \| _🟡 Minor_" — 136 days old
- [#3680 refactor(dapi,dpp)!: move dapi-client and Identifier off Buffer to Uint8Array](https://github.com/dashpay/platform/pull/3680) — 5 unresolved (5 human) · 41 days stale · 🔴 CI failing
  - Top thread: "<!-- thepastaclaw-review v1 finding=1c101f507542 -->" — 41 days old
- [#3462 fix(dpp)!: version-gate distribution function floating-point evaluation](https://github.com/dashpay/platform/pull/3462) — 3 unresolved (3 human) · 35 days stale · 🔴 CI failing
  - Top thread: "<!-- thepastaclaw-review v1 finding=7d40971bd5ff dedupe=2846c55b07c76472 -->" — 35 days old
- [#3509 fix(platform): default omitted proved query limits](https://github.com/dashpay/platform/pull/3509) — 1 unresolved (1 human) · 65 days stale · ✋ changes requested · 🔴 CI failing
  - Top thread: "<!-- thepastaclaw-review v1 finding=d3df3e197206 -->" — 65 days old
- [#2988 feat(wasm-sdk): auto-generate entropy for document creation when not provided](https://github.com/dashpay/platform/pull/2988) — 1 unresolved (1 human) · 46 days stale · 🔴 CI failing
  - Top thread: "<!-- thepastaclaw-review v1 finding=30cbefe1c9ef -->" — 46 days old

<a id="pastapastapasta-draft"></a>
#### Draft (5)
- [#3689 refactor(sdk,wallet-lib,test-suite)!: convert dapi-client + wallet-lib + js-dash-sdk + platform-test-suite to ESM](https://github.com/dashpay/platform/pull/3689) — ⚠ merge conflict · 📝 draft
- [#3967 feat(wasm-sdk): expose tiered token pricing in setPrice](https://github.com/dashpay/platform/pull/3967) — via @thepastaclaw · 📝 draft
- [#4000 fix(dpp): report main control group authorization](https://github.com/dashpay/platform/pull/4000) — via @thepastaclaw · 📝 draft
- [#4015 fix(swift-example-app): gate identity resumes by funding type](https://github.com/dashpay/platform/pull/4015) — via @thepastaclaw · 📝 draft
- [#4016 fix(platform-wallet): close asset-lock resume broadcast race](https://github.com/dashpay/platform/pull/4016) — via @thepastaclaw · 📝 draft

<a id="pastapastapasta-stale"></a>
#### Stale (4)
- [#3898 fix(dashmate): re-sync stale 3.x Drive and rs-dapi images](https://github.com/dashpay/platform/pull/3898) — via @thepastaclaw · 🐢 targets v4.0-dev
- [#3930 fix(platform): reject keep-history document deletes cleanly](https://github.com/dashpay/platform/pull/3930) — via @thepastaclaw · 🔴 CI failing · 🐢 targets v4.0-dev
- [#3965 fix(drive-abci): use testnet Core RPC port in env](https://github.com/dashpay/platform/pull/3965) — via @thepastaclaw · 🐢 targets v4.0-dev
- [#3993 test(swift-sdk): assert transaction decoder error cases](https://github.com/dashpay/platform/pull/3993) — via @thepastaclaw · 🐢 targets feat/core-tx-decode-ffi

<a id="pastapastapasta-clean"></a>
#### Clean (2)
- [#3635 ci: tolerate book preview comment permission errors](https://github.com/dashpay/platform/pull/3635) — via @thepastaclaw
- [#3938 test(rs-sdk): expect network floor in mock sdk seed test](https://github.com/dashpay/platform/pull/3938) — via @thepastaclaw

<a id="quantumexplorer"></a>
### @QuantumExplorer
<a id="quantumexplorer-open"></a>
#### Open (7)
- [#3740 feat: add time-range indexes for trending/leaderboard queries](https://github.com/dashpay/platform/pull/3740) — 5 unresolved (5 CodeRabbit) · 42 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing
  - Top thread: "_⚠️ Potential issue_ \| _🟠 Major_ \| _⚡ Quick win_" — 42 days old
- [#3792 fix(drive): authenticate boundary in compacted absence proofs](https://github.com/dashpay/platform/pull/3792) — 1 unresolved (1 human) · 32 days stale · 📝 draft · 🐢 targets v4.0-dev
  - Top thread: "<!-- thepastaclaw-review v1 finding=be8cd7f6fd00 dedupe=bb18c40415f6c4c4 -->" — 32 days old
- [#4032 test(platform-wallet): pin Orchard key derivation to official ZIP-32 vectors](https://github.com/dashpay/platform/pull/4032) — 2 unresolved (2 human) · 0 days stale · 🔴 CI failing
  - Top thread: "<!-- thepastaclaw-review v1 finding=c1a4301a8c5c dedupe=067fc579245c4cd6 -->" — 0 days old
- [#4033 fix(platform-wallet): poll Sync Now FFI passes on big-stack threads to stop SIGBUS crash](https://github.com/dashpay/platform/pull/4033) — 1 unresolved (1 human) · 0 days stale · 🔴 CI failing
  - Top thread: "<!-- thepastaclaw-review v1 finding=4257efc6b2b6 dedupe=35b4475f934db2cd -->" — 0 days old
- [#1834 feat(drive-abci)!: Statesync](https://github.com/dashpay/platform/pull/1834) — 📝 draft · ⏸ deferred
- [#3021 feat(platform): better contract state transitions](https://github.com/dashpay/platform/pull/3021) — 📝 draft · ⏸ deferred
- [#3417 feat(swift-sdk): use SPV-synced quorums for Platform proof verification](https://github.com/dashpay/platform/pull/3417) — ⚠ merge conflict · 🔴 CI failing · 📝 draft

<a id="quantumexplorer-needs-action"></a>
#### Needs action (3)
- [#3740 feat: add time-range indexes for trending/leaderboard queries](https://github.com/dashpay/platform/pull/3740) — 5 unresolved (5 CodeRabbit) · 42 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing
  - Top thread: "_⚠️ Potential issue_ \| _🟠 Major_ \| _⚡ Quick win_" — 42 days old
- [#4032 test(platform-wallet): pin Orchard key derivation to official ZIP-32 vectors](https://github.com/dashpay/platform/pull/4032) — 2 unresolved (2 human) · 0 days stale · 🔴 CI failing
  - Top thread: "<!-- thepastaclaw-review v1 finding=c1a4301a8c5c dedupe=067fc579245c4cd6 -->" — 0 days old
- [#4033 fix(platform-wallet): poll Sync Now FFI passes on big-stack threads to stop SIGBUS crash](https://github.com/dashpay/platform/pull/4033) — 1 unresolved (1 human) · 0 days stale · 🔴 CI failing
  - Top thread: "<!-- thepastaclaw-review v1 finding=4257efc6b2b6 dedupe=35b4475f934db2cd -->" — 0 days old

<a id="quantumexplorer-unresolved-comments"></a>
#### Unresolved Comments (3)
- [#3740 feat: add time-range indexes for trending/leaderboard queries](https://github.com/dashpay/platform/pull/3740) — 5 unresolved (5 CodeRabbit) · 42 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing
  - Top thread: "_⚠️ Potential issue_ \| _🟠 Major_ \| _⚡ Quick win_" — 42 days old
- [#4032 test(platform-wallet): pin Orchard key derivation to official ZIP-32 vectors](https://github.com/dashpay/platform/pull/4032) — 2 unresolved (2 human) · 0 days stale · 🔴 CI failing
  - Top thread: "<!-- thepastaclaw-review v1 finding=c1a4301a8c5c dedupe=067fc579245c4cd6 -->" — 0 days old
- [#4033 fix(platform-wallet): poll Sync Now FFI passes on big-stack threads to stop SIGBUS crash](https://github.com/dashpay/platform/pull/4033) — 1 unresolved (1 human) · 0 days stale · 🔴 CI failing
  - Top thread: "<!-- thepastaclaw-review v1 finding=4257efc6b2b6 dedupe=35b4475f934db2cd -->" — 0 days old

<a id="quantumexplorer-deferred"></a>
#### Deferred (2)
- [#1834 feat(drive-abci)!: Statesync](https://github.com/dashpay/platform/pull/1834) — 📝 draft · ⏸ deferred
- [#3021 feat(platform): better contract state transitions](https://github.com/dashpay/platform/pull/3021) — 📝 draft · ⏸ deferred

<a id="quantumexplorer-draft"></a>
#### Draft (1)
- [#3417 feat(swift-sdk): use SPV-synced quorums for Platform proof verification](https://github.com/dashpay/platform/pull/3417) — ⚠ merge conflict · 🔴 CI failing · 📝 draft

<a id="quantumexplorer-stale"></a>
#### Stale (1)
- [#3792 fix(drive): authenticate boundary in compacted absence proofs](https://github.com/dashpay/platform/pull/3792) — 1 unresolved (1 human) · 32 days stale · 📝 draft · 🐢 targets v4.0-dev
  - Top thread: "<!-- thepastaclaw-review v1 finding=be8cd7f6fd00 dedupe=bb18c40415f6c4c4 -->" — 32 days old

<a id="quantumexplorer-ready-for-review"></a>
#### Ready for Review (3)
- [#3635 ci: tolerate book preview comment permission errors](https://github.com/dashpay/platform/pull/3635) — by @thepastaclaw
- [#3787 docs: add DashPay contact request encryption guide](https://github.com/dashpay/platform/pull/3787) — by @densmirnov
- [#3938 test(rs-sdk): expect network floor in mock sdk seed test](https://github.com/dashpay/platform/pull/3938) — by @thepastaclaw

<a id="llbartekll"></a>
### @llbartekll
<a id="llbartekll-open"></a>
#### Open (3)
- [#3560 test(swift-sdk): add testnet identity-discovery UI test](https://github.com/dashpay/platform/pull/3560) — 2 unresolved (2 human) · 35 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing
  - Top thread: "<!-- thepastaclaw-review v1 finding=379c84ed00e1 dedupe=08a70e07f6e9ae45 -->" — 35 days old
- [#3694 fix(swift-example-app): unhide Create Identity submit button and auto-dismiss sheet on success](https://github.com/dashpay/platform/pull/3694) — 1 unresolved (1 human) · 35 days stale · ✋ changes requested
  - Top thread: "<!-- thepastaclaw-review v1 finding=e3a849e745d0 dedupe=d76d5740c5bc0f94 -->" — 35 days old
- [#3981 feat(platform-wallet): pure transaction decoder + thin FFI/Swift wrappers](https://github.com/dashpay/platform/pull/3981) — 2 unresolved (2 human) · 4 days stale · ✋ changes requested · 🔴 CI failing
  - Top thread: "<!-- thepastaclaw-review v1 finding=edda9764781f dedupe=47212b70ec6770b8 -->" — 4 days old

<a id="llbartekll-needs-action"></a>
#### Needs action (3)
- [#3560 test(swift-sdk): add testnet identity-discovery UI test](https://github.com/dashpay/platform/pull/3560) — 2 unresolved (2 human) · 35 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing
  - Top thread: "<!-- thepastaclaw-review v1 finding=379c84ed00e1 dedupe=08a70e07f6e9ae45 -->" — 35 days old
- [#3694 fix(swift-example-app): unhide Create Identity submit button and auto-dismiss sheet on success](https://github.com/dashpay/platform/pull/3694) — 1 unresolved (1 human) · 35 days stale · ✋ changes requested
  - Top thread: "<!-- thepastaclaw-review v1 finding=e3a849e745d0 dedupe=d76d5740c5bc0f94 -->" — 35 days old
- [#3981 feat(platform-wallet): pure transaction decoder + thin FFI/Swift wrappers](https://github.com/dashpay/platform/pull/3981) — 2 unresolved (2 human) · 4 days stale · ✋ changes requested · 🔴 CI failing
  - Top thread: "<!-- thepastaclaw-review v1 finding=edda9764781f dedupe=47212b70ec6770b8 -->" — 4 days old

<a id="llbartekll-unresolved-comments"></a>
#### Unresolved Comments (3)
- [#3560 test(swift-sdk): add testnet identity-discovery UI test](https://github.com/dashpay/platform/pull/3560) — 2 unresolved (2 human) · 35 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing
  - Top thread: "<!-- thepastaclaw-review v1 finding=379c84ed00e1 dedupe=08a70e07f6e9ae45 -->" — 35 days old
- [#3694 fix(swift-example-app): unhide Create Identity submit button and auto-dismiss sheet on success](https://github.com/dashpay/platform/pull/3694) — 1 unresolved (1 human) · 35 days stale · ✋ changes requested
  - Top thread: "<!-- thepastaclaw-review v1 finding=e3a849e745d0 dedupe=d76d5740c5bc0f94 -->" — 35 days old
- [#3981 feat(platform-wallet): pure transaction decoder + thin FFI/Swift wrappers](https://github.com/dashpay/platform/pull/3981) — 2 unresolved (2 human) · 4 days stale · ✋ changes requested · 🔴 CI failing
  - Top thread: "<!-- thepastaclaw-review v1 finding=edda9764781f dedupe=47212b70ec6770b8 -->" — 4 days old

<a id="llbartekll-ready-for-review"></a>
#### Ready for Review (2)
- [#3692 feat(platform-wallet): watch-only rehydration from persistor (seedless load)](https://github.com/dashpay/platform/pull/3692) — by @Claudius-Maginificent
- [#3853 fix(swift-sdk): drop legacy headers pre-processing in build_ios.sh](https://github.com/dashpay/platform/pull/3853) — by @ZocoLini

<a id="shumkov"></a>
### @shumkov
<a id="shumkov-open"></a>
#### Open (8)
- [#4030 feat(sdk): add the DashPay tab to KotlinExampleApp (migration K3)](https://github.com/dashpay/platform/pull/4030) — 5 unresolved (5 human) · 0 days stale · ✋ changes requested · 🐢 targets feat/kotlin-sdk-dashpay-k2
  - Top thread: "<!-- thepastaclaw-review v1 finding=d9b62091cb2f dedupe=44318136a6888094 -->" — 0 days old
- [#3936 chore(drive-abci): update to nested address in SML](https://github.com/dashpay/platform/pull/3936) — 2 unresolved (2 human) · 18 days stale · ⚠ merge conflict · 📝 draft
  - Top thread: "<!-- thepastaclaw-review v1 finding=2e1950367681 dedupe=494e3310440bc19a -->" — 18 days old
- [#4027 feat(sdk): add DashPay sync service, seedless unlock and writes to Kotlin SDK (migration K2)](https://github.com/dashpay/platform/pull/4027) — 5 unresolved (5 human) · 0 days stale · 🐢 targets feat/kotlin-sdk-dashpay-k1
  - Top thread: "<!-- thepastaclaw-review v1 finding=c7aa1844e727 dedupe=c5ba607d824a100b -->" — 0 days old
- [#4025 feat(sdk): add DashPay persistence and read bridges to Kotlin SDK (migration K1)](https://github.com/dashpay/platform/pull/4025) — 4 unresolved (4 human) · 0 days stale · 🐢 targets feat/kotlin-sdk-and-example-app
  - Top thread: "<!-- thepastaclaw-review v1 finding=5b99c7a1f59f dedupe=456ae9c5f38d5ca9 -->" — 0 days old
- [#3974 docs(book): fix drifted source links in the count group-by chapters](https://github.com/dashpay/platform/pull/3974) — 1 unresolved (1 human) · 6 days stale · 🔴 CI failing
  - Top thread: "<!-- thepastaclaw-review v1 finding=84431ddef276 dedupe=7651eb50687937ba -->" — 6 days old
- [#2392 feat(dashmate): single node local network [WIP\]](https://github.com/dashpay/platform/pull/2392) — ⏸ deferred
- [#2518 feat: token marketplace](https://github.com/dashpay/platform/pull/2518) — 📝 draft · ⏸ deferred
- [#3573 feat(dpp): unify JSON/Value conversion traits + comprehensive round-trip tests](https://github.com/dashpay/platform/pull/3573) — ✋ changes requested · 🔴 CI failing

<a id="shumkov-needs-action"></a>
#### Needs action (2)
- [#3974 docs(book): fix drifted source links in the count group-by chapters](https://github.com/dashpay/platform/pull/3974) — 1 unresolved (1 human) · 6 days stale · 🔴 CI failing
  - Top thread: "<!-- thepastaclaw-review v1 finding=84431ddef276 dedupe=7651eb50687937ba -->" — 6 days old
- [#3573 feat(dpp): unify JSON/Value conversion traits + comprehensive round-trip tests](https://github.com/dashpay/platform/pull/3573) — ✋ changes requested · 🔴 CI failing

<a id="shumkov-unresolved-comments"></a>
#### Unresolved Comments (1)
- [#3974 docs(book): fix drifted source links in the count group-by chapters](https://github.com/dashpay/platform/pull/3974) — 1 unresolved (1 human) · 6 days stale · 🔴 CI failing
  - Top thread: "<!-- thepastaclaw-review v1 finding=84431ddef276 dedupe=7651eb50687937ba -->" — 6 days old

<a id="shumkov-changes-requested"></a>
#### Changes Requested (1)
- [#3573 feat(dpp): unify JSON/Value conversion traits + comprehensive round-trip tests](https://github.com/dashpay/platform/pull/3573) — ✋ changes requested · 🔴 CI failing

<a id="shumkov-deferred"></a>
#### Deferred (2)
- [#2392 feat(dashmate): single node local network [WIP\]](https://github.com/dashpay/platform/pull/2392) — ⏸ deferred
- [#2518 feat: token marketplace](https://github.com/dashpay/platform/pull/2518) — 📝 draft · ⏸ deferred

<a id="shumkov-draft"></a>
#### Draft (1)
- [#3936 chore(drive-abci): update to nested address in SML](https://github.com/dashpay/platform/pull/3936) — 2 unresolved (2 human) · 18 days stale · ⚠ merge conflict · 📝 draft
  - Top thread: "<!-- thepastaclaw-review v1 finding=2e1950367681 dedupe=494e3310440bc19a -->" — 18 days old

<a id="shumkov-stale"></a>
#### Stale (3)
- [#4030 feat(sdk): add the DashPay tab to KotlinExampleApp (migration K3)](https://github.com/dashpay/platform/pull/4030) — 5 unresolved (5 human) · 0 days stale · ✋ changes requested · 🐢 targets feat/kotlin-sdk-dashpay-k2
  - Top thread: "<!-- thepastaclaw-review v1 finding=d9b62091cb2f dedupe=44318136a6888094 -->" — 0 days old
- [#4027 feat(sdk): add DashPay sync service, seedless unlock and writes to Kotlin SDK (migration K2)](https://github.com/dashpay/platform/pull/4027) — 5 unresolved (5 human) · 0 days stale · 🐢 targets feat/kotlin-sdk-dashpay-k1
  - Top thread: "<!-- thepastaclaw-review v1 finding=c7aa1844e727 dedupe=c5ba607d824a100b -->" — 0 days old
- [#4025 feat(sdk): add DashPay persistence and read bridges to Kotlin SDK (migration K1)](https://github.com/dashpay/platform/pull/4025) — 4 unresolved (4 human) · 0 days stale · 🐢 targets feat/kotlin-sdk-and-example-app
  - Top thread: "<!-- thepastaclaw-review v1 finding=5b99c7a1f59f dedupe=456ae9c5f38d5ca9 -->" — 0 days old

<a id="shumkov-ready-for-review"></a>
#### Ready for Review (1)
- [#3938 test(rs-sdk): expect network floor in mock sdk seed test](https://github.com/dashpay/platform/pull/3938) — by @thepastaclaw

<a id="thephez"></a>
### @thephez
<a id="thephez-open"></a>
#### Open (3)
- [#3961 test(drive): cover shared-prefix aggregate index insertion](https://github.com/dashpay/platform/pull/3961) — 3 unresolved (3 human) · 11 days stale
  - Top thread: "<!-- thepastaclaw-review v1 finding=651d0f445831 dedupe=e060ec72747ed6c2 -->" — 11 days old
- [#3849 test(drive-abci): add token supply edge-case coverage](https://github.com/dashpay/platform/pull/3849) — 🔴 CI failing
- [#3928 test(dpp,drive-abci): pin keepHistory + canBeDeleted contradiction](https://github.com/dashpay/platform/pull/3928) — 📝 draft

<a id="thephez-needs-action"></a>
#### Needs action (1)
- [#3961 test(drive): cover shared-prefix aggregate index insertion](https://github.com/dashpay/platform/pull/3961) — 3 unresolved (3 human) · 11 days stale
  - Top thread: "<!-- thepastaclaw-review v1 finding=651d0f445831 dedupe=e060ec72747ed6c2 -->" — 11 days old

<a id="thephez-unresolved-comments"></a>
#### Unresolved Comments (1)
- [#3961 test(drive): cover shared-prefix aggregate index insertion](https://github.com/dashpay/platform/pull/3961) — 3 unresolved (3 human) · 11 days stale
  - Top thread: "<!-- thepastaclaw-review v1 finding=651d0f445831 dedupe=e060ec72747ed6c2 -->" — 11 days old

<a id="thephez-ci-failing"></a>
#### CI Failing (1)
- [#3849 test(drive-abci): add token supply edge-case coverage](https://github.com/dashpay/platform/pull/3849) — 🔴 CI failing

<a id="thephez-draft"></a>
#### Draft (1)
- [#3928 test(dpp,drive-abci): pin keepHistory + canBeDeleted contradiction](https://github.com/dashpay/platform/pull/3928) — 📝 draft

<a id="zocolini"></a>
### @ZocoLini
<a id="zocolini-open"></a>
#### Open (5)
- [#3853 fix(swift-sdk): drop legacy headers pre-processing in build_ios.sh](https://github.com/dashpay/platform/pull/3853)
- [#3869 chore(swift-sdk): reduce swift-sdk test time in CI](https://github.com/dashpay/platform/pull/3869) — ⚠ merge conflict
- [#3948 chore: remove ignored cargo build config](https://github.com/dashpay/platform/pull/3948)
- [#3949 chore(swift-sdk): correctly refer to the swift sdk as swift in the build script and cargo profiles](https://github.com/dashpay/platform/pull/3949) — ⚠ merge conflict · ✋ changes requested · 🔴 CI failing
- [#3970 refactor(platform-wallet): expose the new core TransactionBuilder API](https://github.com/dashpay/platform/pull/3970) — ✋ changes requested · 🔴 CI failing

<a id="zocolini-needs-action"></a>
#### Needs action (3)
- [#3869 chore(swift-sdk): reduce swift-sdk test time in CI](https://github.com/dashpay/platform/pull/3869) — ⚠ merge conflict
- [#3949 chore(swift-sdk): correctly refer to the swift sdk as swift in the build script and cargo profiles](https://github.com/dashpay/platform/pull/3949) — ⚠ merge conflict · ✋ changes requested · 🔴 CI failing
- [#3970 refactor(platform-wallet): expose the new core TransactionBuilder API](https://github.com/dashpay/platform/pull/3970) — ✋ changes requested · 🔴 CI failing

<a id="zocolini-changes-requested"></a>
#### Changes Requested (2)
- [#3949 chore(swift-sdk): correctly refer to the swift sdk as swift in the build script and cargo profiles](https://github.com/dashpay/platform/pull/3949) — ⚠ merge conflict · ✋ changes requested · 🔴 CI failing
- [#3970 refactor(platform-wallet): expose the new core TransactionBuilder API](https://github.com/dashpay/platform/pull/3970) — ✋ changes requested · 🔴 CI failing

<a id="zocolini-clean"></a>
#### Clean (3)
- [#3853 fix(swift-sdk): drop legacy headers pre-processing in build_ios.sh](https://github.com/dashpay/platform/pull/3853)
- [#3869 chore(swift-sdk): reduce swift-sdk test time in CI](https://github.com/dashpay/platform/pull/3869) — ⚠ merge conflict
- [#3948 chore: remove ignored cargo build config](https://github.com/dashpay/platform/pull/3948)

<a id="lklimek"></a>
### @lklimek + (@Claudius-Maginificent)
<a id="lklimek-open"></a>
#### Open (11)
- [#3549 test(platform-wallet): e2e framework + full test suite — triage pins, Found-*/PA-* guards, fail-closed persist, Stage-2 merge](https://github.com/dashpay/platform/pull/3549) — via @Claudius-Maginificent · 20 unresolved (20 human) · 65 days stale · 📝 draft
  - Top thread: "<!-- thepastaclaw-review v1 finding=a3725e9d50ca -->" — 65 days old
- [#3992 feat(platform-wallet): manifest integrity checksum (Risk-6/R12.5 follow-up)](https://github.com/dashpay/platform/pull/3992) — via @Claudius-Maginificent · 4 unresolved (4 human) · 3 days stale · ✋ changes requested · 🐢 targets feat/platform-wallet-storage-rehydration
  - Top thread: "<!-- thepastaclaw-review v1 finding=3969d61c562f dedupe=57a8bf7151c0879e -->" — 3 days old
- [#3968 feat(platform-wallet): persistence readers + seedless load() wiring (split from #3692)](https://github.com/dashpay/platform/pull/3968) — via @Claudius-Maginificent · 3 unresolved (3 human) · 4 days stale · ✋ changes requested · 🐢 targets feat/platform-wallet-rehydration
  - Top thread: "<!-- thepastaclaw-review v1 finding=f294372e72bc dedupe=4a944df743c28c8d -->" — 4 days old
- [#2679 fix(drive-abci): don't panic on unsupported version error](https://github.com/dashpay/platform/pull/2679) — ⏸ deferred
- [#2795 feat(platform): add real-time platform event subscriptions via gRPC streaming](https://github.com/dashpay/platform/pull/2795) — ⏸ deferred
- [#2993 feat: identity reference validation](https://github.com/dashpay/platform/pull/2993) — 📝 draft · ⏸ deferred
- [#3009 feat: contract reference validation](https://github.com/dashpay/platform/pull/3009) — 📝 draft · ⏸ deferred
- [#3032 feat: document creation restriction by group membership](https://github.com/dashpay/platform/pull/3032) — 📝 draft · ⏸ deferred
- [#3692 feat(platform-wallet): watch-only rehydration from persistor (seedless load)](https://github.com/dashpay/platform/pull/3692) — via @Claudius-Maginificent
- [#3750 feat(platform-wallet)!: [NO MERGE\] consumer hardening — CODE-001/003-callsite/017/018 + PROJ-001 FFI + CODE-008/012/013](https://github.com/dashpay/platform/pull/3750) — via @Claudius-Maginificent · ⚠ merge conflict · 📝 draft
- [#3954 feat(platform-wallet)!: shared ThreadRegistry for coordinator lifecycle + shutdown UAF/data-loss fixes](https://github.com/dashpay/platform/pull/3954) — via @Claudius-Maginificent · ⚠ merge conflict

<a id="lklimek-needs-action"></a>
#### Needs action (1)
- [#3954 feat(platform-wallet)!: shared ThreadRegistry for coordinator lifecycle + shutdown UAF/data-loss fixes](https://github.com/dashpay/platform/pull/3954) — via @Claudius-Maginificent · ⚠ merge conflict

<a id="lklimek-deferred"></a>
#### Deferred (5)
- [#2679 fix(drive-abci): don't panic on unsupported version error](https://github.com/dashpay/platform/pull/2679) — ⏸ deferred
- [#2795 feat(platform): add real-time platform event subscriptions via gRPC streaming](https://github.com/dashpay/platform/pull/2795) — ⏸ deferred
- [#2993 feat: identity reference validation](https://github.com/dashpay/platform/pull/2993) — 📝 draft · ⏸ deferred
- [#3009 feat: contract reference validation](https://github.com/dashpay/platform/pull/3009) — 📝 draft · ⏸ deferred
- [#3032 feat: document creation restriction by group membership](https://github.com/dashpay/platform/pull/3032) — 📝 draft · ⏸ deferred

<a id="lklimek-draft"></a>
#### Draft (2)
- [#3549 test(platform-wallet): e2e framework + full test suite — triage pins, Found-*/PA-* guards, fail-closed persist, Stage-2 merge](https://github.com/dashpay/platform/pull/3549) — via @Claudius-Maginificent · 20 unresolved (20 human) · 65 days stale · 📝 draft
  - Top thread: "<!-- thepastaclaw-review v1 finding=a3725e9d50ca -->" — 65 days old
- [#3750 feat(platform-wallet)!: [NO MERGE\] consumer hardening — CODE-001/003-callsite/017/018 + PROJ-001 FFI + CODE-008/012/013](https://github.com/dashpay/platform/pull/3750) — via @Claudius-Maginificent · ⚠ merge conflict · 📝 draft

<a id="lklimek-stale"></a>
#### Stale (2)
- [#3992 feat(platform-wallet): manifest integrity checksum (Risk-6/R12.5 follow-up)](https://github.com/dashpay/platform/pull/3992) — via @Claudius-Maginificent · 4 unresolved (4 human) · 3 days stale · ✋ changes requested · 🐢 targets feat/platform-wallet-storage-rehydration
  - Top thread: "<!-- thepastaclaw-review v1 finding=3969d61c562f dedupe=57a8bf7151c0879e -->" — 3 days old
- [#3968 feat(platform-wallet): persistence readers + seedless load() wiring (split from #3692)](https://github.com/dashpay/platform/pull/3968) — via @Claudius-Maginificent · 3 unresolved (3 human) · 4 days stale · ✋ changes requested · 🐢 targets feat/platform-wallet-rehydration
  - Top thread: "<!-- thepastaclaw-review v1 finding=f294372e72bc dedupe=4a944df743c28c8d -->" — 4 days old

<a id="lklimek-clean"></a>
#### Clean (2)
- [#3692 feat(platform-wallet): watch-only rehydration from persistor (seedless load)](https://github.com/dashpay/platform/pull/3692) — via @Claudius-Maginificent
- [#3954 feat(platform-wallet)!: shared ThreadRegistry for coordinator lifecycle + shutdown UAF/data-loss fixes](https://github.com/dashpay/platform/pull/3954) — via @Claudius-Maginificent · ⚠ merge conflict

<a id="lklimek-ready-for-review"></a>
#### Ready for Review (1)
- [#3938 test(rs-sdk): expect network floor in mock sdk seed test](https://github.com/dashpay/platform/pull/3938) — by @thepastaclaw

<a id="pshenmic"></a>
### @pshenmic
<a id="pshenmic-open"></a>
#### Open (1)
- [#2446 feat(js-dapi-client): add contested resources query methods](https://github.com/dashpay/platform/pull/2446) — 5 unresolved (5 CodeRabbit) · 523 days stale · ⚠ merge conflict · 📝 draft · 🐢 targets v2.0-dev, untouched 514 days
  - Top thread: "_:warning: Potential issue_" — 523 days old

<a id="pshenmic-stale"></a>
#### Stale (1)
- [#2446 feat(js-dapi-client): add contested resources query methods](https://github.com/dashpay/platform/pull/2446) — 5 unresolved (5 CodeRabbit) · 523 days stale · ⚠ merge conflict · 📝 draft · 🐢 targets v2.0-dev, untouched 514 days
  - Top thread: "_:warning: Potential issue_" — 523 days old

<a id="densmirnov"></a>
### @densmirnov
<a id="densmirnov-open"></a>
#### Open (1)
- [#3787 docs: add DashPay contact request encryption guide](https://github.com/dashpay/platform/pull/3787)

<a id="densmirnov-clean"></a>
#### Clean (1)
- [#3787 docs: add DashPay contact request encryption guide](https://github.com/dashpay/platform/pull/3787)

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
