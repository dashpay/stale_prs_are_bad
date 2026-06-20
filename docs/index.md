---
---
# PR Hygiene Report
*Last updated: 2026-06-20 00:50 UTC · commit 3c9ac41*

## Summary
- Open PRs: **51** (8 clean · 3 CI failing · 3 changes requested · 10 unresolved comments · 10 deferred · 14 draft · 3 stale)
- PRs needing author action: **14**
- Total unresolved comments: **68**

## Scoreboard
_Sort: unresolved-comments desc → needs-action desc → ready-for-review desc. Click any number to jump to the specific PRs it covers._

| Author | Open | Clean | CI failing | Unresolved Comments | Changes Requested | Deferred | Draft | Stale | Needs action | Total Unresolved Comments | Ready for Review | Δ |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| [@PastaPastaPasta + (@thepastaclaw)](#pastapastapasta) | [5+(10)](#pastapastapasta-open) | [0+(4)](#pastapastapasta-clean) | [0+(1)](#pastapastapasta-ci-failing) | [4+(0)](#pastapastapasta-unresolved-comments) | — | — | [1+(4)](#pastapastapasta-draft) | [0+(1)](#pastapastapasta-stale) | [4+(0)](#pastapastapasta-needs-action) | [10+(4)](#pastapastapasta-unresolved-comments) | — | — |
| [@QuantumExplorer](#quantumexplorer) | [9](#quantumexplorer-open) | [1](#quantumexplorer-clean) | [1](#quantumexplorer-ci-failing) | [3](#quantumexplorer-unresolved-comments) | — | [2](#quantumexplorer-deferred) | [2](#quantumexplorer-draft) | — | [3](#quantumexplorer-needs-action) | [9](#quantumexplorer-unresolved-comments) | [4](#quantumexplorer-ready-for-review) | ↓ 5 |
| [@llbartekll](#llbartekll) | [3](#llbartekll-open) | — | — | [3](#llbartekll-unresolved-comments) | — | — | — | — | [3](#llbartekll-needs-action) | [5](#llbartekll-unresolved-comments) | — | ↑ 1 |
| [@shumkov](#shumkov) | [5](#shumkov-open) | — | — | — | [2](#shumkov-changes-requested) | [2](#shumkov-deferred) | [1](#shumkov-draft) | — | [2](#shumkov-needs-action) | [3](#shumkov-unresolved-comments) | [4](#shumkov-ready-for-review) | ↑ 1 |
| [@ZocoLini](#zocolini) | [3](#zocolini-open) | [1](#zocolini-clean) | — | — | [1](#zocolini-changes-requested) | — | [1](#zocolini-draft) | — | [2](#zocolini-needs-action) | [12](#zocolini-unresolved-comments) | — | ↓ 1 |
| [@lklimek + (@Claudius-Maginificent)](#lklimek) | [6+(5)](#lklimek-open) | [1+(0)](#lklimek-clean) | — | — | — | [5+(0)](#lklimek-deferred) | [0+(4)](#lklimek-draft) | [0+(1)](#lklimek-stale) | — | [0+(20)](#lklimek-unresolved-comments) | [1+(0)](#lklimek-ready-for-review) | ↓ 2 |
| [@pshenmic](#pshenmic) | [1](#pshenmic-open) | — | — | — | — | — | — | [1](#pshenmic-stale) | — | [5](#pshenmic-unresolved-comments) | — | — |
| [@densmirnov](#densmirnov) | [1](#densmirnov-open) | [1](#densmirnov-clean) | — | — | — | — | — | — | — | — | — | — |
| [@ogabrielides](#ogabrielides) | [1](#ogabrielides-open) | — | — | — | — | [1](#ogabrielides-deferred) | — | — | — | — | — | — |
| [@thephez](#thephez) | [2](#thephez-open) | — | [1](#thephez-ci-failing) | — | — | — | [1](#thephez-draft) | — | — | — | — | — |

## Per-author detail

<a id="pastapastapasta"></a>
### @PastaPastaPasta + (@thepastaclaw)
<a id="pastapastapasta-open"></a>
#### Open (15)
- [#3096 feat(sdk): add client-side validation to state transition construction methods](https://github.com/dashpay/platform/pull/3096) — via @thepastaclaw · 4 unresolved (2 CodeRabbit, 2 human) · 122 days stale · 📝 draft
  - Top thread: "_⚠️ Potential issue_ \| _🟡 Minor_" — 119 days old
- [#3680 refactor(dapi,dpp)!: move dapi-client and Identifier off Buffer to Uint8Array](https://github.com/dashpay/platform/pull/3680) — 5 unresolved (5 human) · 24 days stale · 🔴 CI failing
  - Top thread: "<!-- thepastaclaw-review v1 finding=1c101f507542 -->" — 24 days old
- [#3509 fix(platform): default omitted proved query limits](https://github.com/dashpay/platform/pull/3509) — 1 unresolved (1 human) · 48 days stale · ✋ changes requested · 🔴 CI failing
  - Top thread: "<!-- thepastaclaw-review v1 finding=d3df3e197206 -->" — 48 days old
- [#3462 fix(dpp)!: version-gate distribution function floating-point evaluation](https://github.com/dashpay/platform/pull/3462) — 3 unresolved (3 human) · 18 days stale · 🔴 CI failing
  - Top thread: "<!-- thepastaclaw-review v1 finding=7d40971bd5ff dedupe=2846c55b07c76472 -->" — 18 days old
- [#2988 feat(wasm-sdk): auto-generate entropy for document creation when not provided](https://github.com/dashpay/platform/pull/2988) — 1 unresolved (1 human) · 29 days stale · 🔴 CI failing
  - Top thread: "<!-- thepastaclaw-review v1 finding=30cbefe1c9ef -->" — 29 days old
- [#3091 feat(wasm-sdk): add prepare_* APIs for idempotent document state transitions](https://github.com/dashpay/platform/pull/3091) — via @thepastaclaw · ✋ changes requested · 📝 draft
- [#3092 feat(rs-sdk): expose transition hash from state transition methods](https://github.com/dashpay/platform/pull/3092) — via @thepastaclaw · ⚠ merge conflict · ✋ changes requested · 📝 draft
- [#3133 fix(sdk): validate batch base structure before signing](https://github.com/dashpay/platform/pull/3133) — via @thepastaclaw · ⚠ merge conflict · ✋ changes requested · 📝 draft
- [#3635 ci: tolerate book preview comment permission errors](https://github.com/dashpay/platform/pull/3635) — via @thepastaclaw
- [#3669 fix(swift-sdk): wait for SDK rebuild before wallet activation](https://github.com/dashpay/platform/pull/3669) — via @thepastaclaw · 🐢 targets claude/swift-xcuitest-wallet-persistence
- [#3689 refactor(sdk,wallet-lib,test-suite)!: convert dapi-client + wallet-lib + js-dash-sdk + platform-test-suite to ESM](https://github.com/dashpay/platform/pull/3689) — ⚠ merge conflict · 📝 draft
- [#3898 fix(dashmate): re-sync stale 3.x Drive and rs-dapi images](https://github.com/dashpay/platform/pull/3898) — via @thepastaclaw
- [#3930 fix(platform): reject keep-history document deletes cleanly](https://github.com/dashpay/platform/pull/3930) — via @thepastaclaw · 🔴 CI failing
- [#3938 test(rs-sdk): expect network floor in mock sdk seed test](https://github.com/dashpay/platform/pull/3938) — via @thepastaclaw
- [#3941 fix(dashmate): use active_dkgs for safe DKG stop](https://github.com/dashpay/platform/pull/3941) — via @thepastaclaw

<a id="pastapastapasta-needs-action"></a>
#### Needs action (4)
- [#3680 refactor(dapi,dpp)!: move dapi-client and Identifier off Buffer to Uint8Array](https://github.com/dashpay/platform/pull/3680) — 5 unresolved (5 human) · 24 days stale · 🔴 CI failing
  - Top thread: "<!-- thepastaclaw-review v1 finding=1c101f507542 -->" — 24 days old
- [#3509 fix(platform): default omitted proved query limits](https://github.com/dashpay/platform/pull/3509) — 1 unresolved (1 human) · 48 days stale · ✋ changes requested · 🔴 CI failing
  - Top thread: "<!-- thepastaclaw-review v1 finding=d3df3e197206 -->" — 48 days old
- [#3462 fix(dpp)!: version-gate distribution function floating-point evaluation](https://github.com/dashpay/platform/pull/3462) — 3 unresolved (3 human) · 18 days stale · 🔴 CI failing
  - Top thread: "<!-- thepastaclaw-review v1 finding=7d40971bd5ff dedupe=2846c55b07c76472 -->" — 18 days old
- [#2988 feat(wasm-sdk): auto-generate entropy for document creation when not provided](https://github.com/dashpay/platform/pull/2988) — 1 unresolved (1 human) · 29 days stale · 🔴 CI failing
  - Top thread: "<!-- thepastaclaw-review v1 finding=30cbefe1c9ef -->" — 29 days old

<a id="pastapastapasta-unresolved-comments"></a>
#### Unresolved Comments (4)
- [#3680 refactor(dapi,dpp)!: move dapi-client and Identifier off Buffer to Uint8Array](https://github.com/dashpay/platform/pull/3680) — 5 unresolved (5 human) · 24 days stale · 🔴 CI failing
  - Top thread: "<!-- thepastaclaw-review v1 finding=1c101f507542 -->" — 24 days old
- [#3509 fix(platform): default omitted proved query limits](https://github.com/dashpay/platform/pull/3509) — 1 unresolved (1 human) · 48 days stale · ✋ changes requested · 🔴 CI failing
  - Top thread: "<!-- thepastaclaw-review v1 finding=d3df3e197206 -->" — 48 days old
- [#3462 fix(dpp)!: version-gate distribution function floating-point evaluation](https://github.com/dashpay/platform/pull/3462) — 3 unresolved (3 human) · 18 days stale · 🔴 CI failing
  - Top thread: "<!-- thepastaclaw-review v1 finding=7d40971bd5ff dedupe=2846c55b07c76472 -->" — 18 days old
- [#2988 feat(wasm-sdk): auto-generate entropy for document creation when not provided](https://github.com/dashpay/platform/pull/2988) — 1 unresolved (1 human) · 29 days stale · 🔴 CI failing
  - Top thread: "<!-- thepastaclaw-review v1 finding=30cbefe1c9ef -->" — 29 days old

<a id="pastapastapasta-ci-failing"></a>
#### CI Failing (1)
- [#3930 fix(platform): reject keep-history document deletes cleanly](https://github.com/dashpay/platform/pull/3930) — via @thepastaclaw · 🔴 CI failing

<a id="pastapastapasta-draft"></a>
#### Draft (5)
- [#3096 feat(sdk): add client-side validation to state transition construction methods](https://github.com/dashpay/platform/pull/3096) — via @thepastaclaw · 4 unresolved (2 CodeRabbit, 2 human) · 122 days stale · 📝 draft
  - Top thread: "_⚠️ Potential issue_ \| _🟡 Minor_" — 119 days old
- [#3091 feat(wasm-sdk): add prepare_* APIs for idempotent document state transitions](https://github.com/dashpay/platform/pull/3091) — via @thepastaclaw · ✋ changes requested · 📝 draft
- [#3092 feat(rs-sdk): expose transition hash from state transition methods](https://github.com/dashpay/platform/pull/3092) — via @thepastaclaw · ⚠ merge conflict · ✋ changes requested · 📝 draft
- [#3133 fix(sdk): validate batch base structure before signing](https://github.com/dashpay/platform/pull/3133) — via @thepastaclaw · ⚠ merge conflict · ✋ changes requested · 📝 draft
- [#3689 refactor(sdk,wallet-lib,test-suite)!: convert dapi-client + wallet-lib + js-dash-sdk + platform-test-suite to ESM](https://github.com/dashpay/platform/pull/3689) — ⚠ merge conflict · 📝 draft

<a id="pastapastapasta-stale"></a>
#### Stale (1)
- [#3669 fix(swift-sdk): wait for SDK rebuild before wallet activation](https://github.com/dashpay/platform/pull/3669) — via @thepastaclaw · 🐢 targets claude/swift-xcuitest-wallet-persistence

<a id="pastapastapasta-clean"></a>
#### Clean (4)
- [#3635 ci: tolerate book preview comment permission errors](https://github.com/dashpay/platform/pull/3635) — via @thepastaclaw
- [#3898 fix(dashmate): re-sync stale 3.x Drive and rs-dapi images](https://github.com/dashpay/platform/pull/3898) — via @thepastaclaw
- [#3938 test(rs-sdk): expect network floor in mock sdk seed test](https://github.com/dashpay/platform/pull/3938) — via @thepastaclaw
- [#3941 fix(dashmate): use active_dkgs for safe DKG stop](https://github.com/dashpay/platform/pull/3941) — via @thepastaclaw

<a id="quantumexplorer"></a>
### @QuantumExplorer
<a id="quantumexplorer-open"></a>
#### Open (9)
- [#3740 feat: add time-range indexes for trending/leaderboard queries](https://github.com/dashpay/platform/pull/3740) — 5 unresolved (5 CodeRabbit) · 25 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing
  - Top thread: "_⚠️ Potential issue_ \| _🟠 Major_ \| _⚡ Quick win_" — 25 days old
- [#3923 feat(swift-example-app): wallet-signed Transfer & Withdraw for platform addresses (ADDR-02/04)](https://github.com/dashpay/platform/pull/3923) — 2 unresolved (2 human) · 3 days stale · 🔴 CI failing
  - Top thread: "<!-- thepastaclaw-review v1 finding=cff29dbfbb9f dedupe=0f33e2cffe50512d -->" — 3 days old
- [#3792 fix(drive): authenticate boundary in compacted absence proofs](https://github.com/dashpay/platform/pull/3792) — 1 unresolved (1 human) · 15 days stale · 📝 draft
  - Top thread: "<!-- thepastaclaw-review v1 finding=be8cd7f6fd00 dedupe=bb18c40415f6c4c4 -->" — 15 days old
- [#3934 fix(swift-example-app): refresh token balance after transfer/burn (MW-02)](https://github.com/dashpay/platform/pull/3934) — 1 unresolved (1 human) · 1 days stale
  - Top thread: "<!-- thepastaclaw-review v1 finding=6d393ce1d31b dedupe=7378dc60e1c80002 -->" — 1 days old
- [#1834 feat(drive-abci)!: Statesync](https://github.com/dashpay/platform/pull/1834) — 📝 draft · ⏸ deferred
- [#3021 feat(platform): better contract state transitions](https://github.com/dashpay/platform/pull/3021) — 📝 draft · ⏸ deferred
- [#3417 feat(swift-sdk): use SPV-synced quorums for Platform proof verification](https://github.com/dashpay/platform/pull/3417) — ⚠ merge conflict · 🔴 CI failing · 📝 draft
- [#3910 feat(contract): on-chain QA framework storage layer (testCase + testRun)](https://github.com/dashpay/platform/pull/3910)
- [#3935 feat(sdk): implement document sum/average aggregation FFI (DOC-13/14)](https://github.com/dashpay/platform/pull/3935) — 🔴 CI failing

<a id="quantumexplorer-needs-action"></a>
#### Needs action (3)
- [#3740 feat: add time-range indexes for trending/leaderboard queries](https://github.com/dashpay/platform/pull/3740) — 5 unresolved (5 CodeRabbit) · 25 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing
  - Top thread: "_⚠️ Potential issue_ \| _🟠 Major_ \| _⚡ Quick win_" — 25 days old
- [#3923 feat(swift-example-app): wallet-signed Transfer & Withdraw for platform addresses (ADDR-02/04)](https://github.com/dashpay/platform/pull/3923) — 2 unresolved (2 human) · 3 days stale · 🔴 CI failing
  - Top thread: "<!-- thepastaclaw-review v1 finding=cff29dbfbb9f dedupe=0f33e2cffe50512d -->" — 3 days old
- [#3934 fix(swift-example-app): refresh token balance after transfer/burn (MW-02)](https://github.com/dashpay/platform/pull/3934) — 1 unresolved (1 human) · 1 days stale
  - Top thread: "<!-- thepastaclaw-review v1 finding=6d393ce1d31b dedupe=7378dc60e1c80002 -->" — 1 days old

<a id="quantumexplorer-unresolved-comments"></a>
#### Unresolved Comments (3)
- [#3740 feat: add time-range indexes for trending/leaderboard queries](https://github.com/dashpay/platform/pull/3740) — 5 unresolved (5 CodeRabbit) · 25 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing
  - Top thread: "_⚠️ Potential issue_ \| _🟠 Major_ \| _⚡ Quick win_" — 25 days old
- [#3923 feat(swift-example-app): wallet-signed Transfer & Withdraw for platform addresses (ADDR-02/04)](https://github.com/dashpay/platform/pull/3923) — 2 unresolved (2 human) · 3 days stale · 🔴 CI failing
  - Top thread: "<!-- thepastaclaw-review v1 finding=cff29dbfbb9f dedupe=0f33e2cffe50512d -->" — 3 days old
- [#3934 fix(swift-example-app): refresh token balance after transfer/burn (MW-02)](https://github.com/dashpay/platform/pull/3934) — 1 unresolved (1 human) · 1 days stale
  - Top thread: "<!-- thepastaclaw-review v1 finding=6d393ce1d31b dedupe=7378dc60e1c80002 -->" — 1 days old

<a id="quantumexplorer-ci-failing"></a>
#### CI Failing (1)
- [#3935 feat(sdk): implement document sum/average aggregation FFI (DOC-13/14)](https://github.com/dashpay/platform/pull/3935) — 🔴 CI failing

<a id="quantumexplorer-deferred"></a>
#### Deferred (2)
- [#1834 feat(drive-abci)!: Statesync](https://github.com/dashpay/platform/pull/1834) — 📝 draft · ⏸ deferred
- [#3021 feat(platform): better contract state transitions](https://github.com/dashpay/platform/pull/3021) — 📝 draft · ⏸ deferred

<a id="quantumexplorer-draft"></a>
#### Draft (2)
- [#3792 fix(drive): authenticate boundary in compacted absence proofs](https://github.com/dashpay/platform/pull/3792) — 1 unresolved (1 human) · 15 days stale · 📝 draft
  - Top thread: "<!-- thepastaclaw-review v1 finding=be8cd7f6fd00 dedupe=bb18c40415f6c4c4 -->" — 15 days old
- [#3417 feat(swift-sdk): use SPV-synced quorums for Platform proof verification](https://github.com/dashpay/platform/pull/3417) — ⚠ merge conflict · 🔴 CI failing · 📝 draft

<a id="quantumexplorer-clean"></a>
#### Clean (1)
- [#3910 feat(contract): on-chain QA framework storage layer (testCase + testRun)](https://github.com/dashpay/platform/pull/3910)

<a id="quantumexplorer-ready-for-review"></a>
#### Ready for Review (4)
- [#3635 ci: tolerate book preview comment permission errors](https://github.com/dashpay/platform/pull/3635) — by @thepastaclaw
- [#3650 fix(sdk): address-sync no longer silently discards balance changes for post-snapshot addresses (Found-025)](https://github.com/dashpay/platform/pull/3650) — by @lklimek
- [#3787 docs: add DashPay contact request encryption guide](https://github.com/dashpay/platform/pull/3787) — by @densmirnov
- [#3938 test(rs-sdk): expect network floor in mock sdk seed test](https://github.com/dashpay/platform/pull/3938) — by @thepastaclaw

<a id="llbartekll"></a>
### @llbartekll
<a id="llbartekll-open"></a>
#### Open (3)
- [#3560 test(swift-sdk): add testnet identity-discovery UI test](https://github.com/dashpay/platform/pull/3560) — 2 unresolved (2 human) · 18 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing
  - Top thread: "<!-- thepastaclaw-review v1 finding=379c84ed00e1 dedupe=08a70e07f6e9ae45 -->" — 18 days old
- [#3694 fix(swift-example-app): unhide Create Identity submit button and auto-dismiss sheet on success](https://github.com/dashpay/platform/pull/3694) — 1 unresolved (1 human) · 18 days stale · ✋ changes requested
  - Top thread: "<!-- thepastaclaw-review v1 finding=e3a849e745d0 dedupe=d76d5740c5bc0f94 -->" — 18 days old
- [#3817 feat(platform-wallet): sweep + recover CoinJoin mixed coins for the DashSync→SDK migration](https://github.com/dashpay/platform/pull/3817) — 2 unresolved (2 human) · 1 days stale · ✋ changes requested · 🔴 CI failing
  - Top thread: "Isn't a better idea to use the TransactionBuilder, doing the needed modifications or does it need to much code to adapt …" — 1 days old

<a id="llbartekll-needs-action"></a>
#### Needs action (3)
- [#3560 test(swift-sdk): add testnet identity-discovery UI test](https://github.com/dashpay/platform/pull/3560) — 2 unresolved (2 human) · 18 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing
  - Top thread: "<!-- thepastaclaw-review v1 finding=379c84ed00e1 dedupe=08a70e07f6e9ae45 -->" — 18 days old
- [#3694 fix(swift-example-app): unhide Create Identity submit button and auto-dismiss sheet on success](https://github.com/dashpay/platform/pull/3694) — 1 unresolved (1 human) · 18 days stale · ✋ changes requested
  - Top thread: "<!-- thepastaclaw-review v1 finding=e3a849e745d0 dedupe=d76d5740c5bc0f94 -->" — 18 days old
- [#3817 feat(platform-wallet): sweep + recover CoinJoin mixed coins for the DashSync→SDK migration](https://github.com/dashpay/platform/pull/3817) — 2 unresolved (2 human) · 1 days stale · ✋ changes requested · 🔴 CI failing
  - Top thread: "Isn't a better idea to use the TransactionBuilder, doing the needed modifications or does it need to much code to adapt …" — 1 days old

<a id="llbartekll-unresolved-comments"></a>
#### Unresolved Comments (3)
- [#3560 test(swift-sdk): add testnet identity-discovery UI test](https://github.com/dashpay/platform/pull/3560) — 2 unresolved (2 human) · 18 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing
  - Top thread: "<!-- thepastaclaw-review v1 finding=379c84ed00e1 dedupe=08a70e07f6e9ae45 -->" — 18 days old
- [#3694 fix(swift-example-app): unhide Create Identity submit button and auto-dismiss sheet on success](https://github.com/dashpay/platform/pull/3694) — 1 unresolved (1 human) · 18 days stale · ✋ changes requested
  - Top thread: "<!-- thepastaclaw-review v1 finding=e3a849e745d0 dedupe=d76d5740c5bc0f94 -->" — 18 days old
- [#3817 feat(platform-wallet): sweep + recover CoinJoin mixed coins for the DashSync→SDK migration](https://github.com/dashpay/platform/pull/3817) — 2 unresolved (2 human) · 1 days stale · ✋ changes requested · 🔴 CI failing
  - Top thread: "Isn't a better idea to use the TransactionBuilder, doing the needed modifications or does it need to much code to adapt …" — 1 days old

<a id="shumkov"></a>
### @shumkov
<a id="shumkov-open"></a>
#### Open (5)
- [#3936 chore(drive-abci): update to nested address in SML](https://github.com/dashpay/platform/pull/3936) — 3 unresolved (3 human) · 1 days stale · ⚠ merge conflict · 📝 draft
  - Top thread: "<!-- thepastaclaw-review v1 finding=2e1950367681 dedupe=494e3310440bc19a -->" — 1 days old
- [#2392 feat(dashmate): single node local network [WIP\]](https://github.com/dashpay/platform/pull/2392) — ⏸ deferred
- [#2518 feat: token marketplace](https://github.com/dashpay/platform/pull/2518) — 📝 draft · ⏸ deferred
- [#3573 feat(dpp): unify JSON/Value conversion traits + comprehensive round-trip tests](https://github.com/dashpay/platform/pull/3573) — ✋ changes requested · 🔴 CI failing
- [#3841 fix(platform-wallet)!: complete dashpay](https://github.com/dashpay/platform/pull/3841) — ⚠ merge conflict · ✋ changes requested

<a id="shumkov-needs-action"></a>
#### Needs action (2)
- [#3573 feat(dpp): unify JSON/Value conversion traits + comprehensive round-trip tests](https://github.com/dashpay/platform/pull/3573) — ✋ changes requested · 🔴 CI failing
- [#3841 fix(platform-wallet)!: complete dashpay](https://github.com/dashpay/platform/pull/3841) — ⚠ merge conflict · ✋ changes requested

<a id="shumkov-changes-requested"></a>
#### Changes Requested (2)
- [#3573 feat(dpp): unify JSON/Value conversion traits + comprehensive round-trip tests](https://github.com/dashpay/platform/pull/3573) — ✋ changes requested · 🔴 CI failing
- [#3841 fix(platform-wallet)!: complete dashpay](https://github.com/dashpay/platform/pull/3841) — ⚠ merge conflict · ✋ changes requested

<a id="shumkov-deferred"></a>
#### Deferred (2)
- [#2392 feat(dashmate): single node local network [WIP\]](https://github.com/dashpay/platform/pull/2392) — ⏸ deferred
- [#2518 feat: token marketplace](https://github.com/dashpay/platform/pull/2518) — 📝 draft · ⏸ deferred

<a id="shumkov-draft"></a>
#### Draft (1)
- [#3936 chore(drive-abci): update to nested address in SML](https://github.com/dashpay/platform/pull/3936) — 3 unresolved (3 human) · 1 days stale · ⚠ merge conflict · 📝 draft
  - Top thread: "<!-- thepastaclaw-review v1 finding=2e1950367681 dedupe=494e3310440bc19a -->" — 1 days old

<a id="shumkov-ready-for-review"></a>
#### Ready for Review (4)
- [#3650 fix(sdk): address-sync no longer silently discards balance changes for post-snapshot addresses (Found-025)](https://github.com/dashpay/platform/pull/3650) — by @lklimek
- [#3898 fix(dashmate): re-sync stale 3.x Drive and rs-dapi images](https://github.com/dashpay/platform/pull/3898) — by @thepastaclaw
- [#3938 test(rs-sdk): expect network floor in mock sdk seed test](https://github.com/dashpay/platform/pull/3938) — by @thepastaclaw
- [#3941 fix(dashmate): use active_dkgs for safe DKG stop](https://github.com/dashpay/platform/pull/3941) — by @thepastaclaw

<a id="zocolini"></a>
### @ZocoLini
<a id="zocolini-open"></a>
#### Open (3)
- [#3712 test(swift-sdk): first swift sdk integration tests with local network](https://github.com/dashpay/platform/pull/3712) — 12 unresolved (4 CodeRabbit, 8 human) · 24 days stale · 📝 draft
  - Top thread: "_⚠️ Potential issue_ \| _🟠 Major_ \| _⚡ Quick win_" — 24 days old
- [#3853 fix(swift-sdk): drop legacy headers pre-processing in build_ios.sh](https://github.com/dashpay/platform/pull/3853) — ⚠ merge conflict
- [#3869 chore(swift-sdk): reduce swift-sdk test time in CI](https://github.com/dashpay/platform/pull/3869) — ⚠ merge conflict · ✋ changes requested

<a id="zocolini-needs-action"></a>
#### Needs action (2)
- [#3853 fix(swift-sdk): drop legacy headers pre-processing in build_ios.sh](https://github.com/dashpay/platform/pull/3853) — ⚠ merge conflict
- [#3869 chore(swift-sdk): reduce swift-sdk test time in CI](https://github.com/dashpay/platform/pull/3869) — ⚠ merge conflict · ✋ changes requested

<a id="zocolini-changes-requested"></a>
#### Changes Requested (1)
- [#3869 chore(swift-sdk): reduce swift-sdk test time in CI](https://github.com/dashpay/platform/pull/3869) — ⚠ merge conflict · ✋ changes requested

<a id="zocolini-draft"></a>
#### Draft (1)
- [#3712 test(swift-sdk): first swift sdk integration tests with local network](https://github.com/dashpay/platform/pull/3712) — 12 unresolved (4 CodeRabbit, 8 human) · 24 days stale · 📝 draft
  - Top thread: "_⚠️ Potential issue_ \| _🟠 Major_ \| _⚡ Quick win_" — 24 days old

<a id="zocolini-clean"></a>
#### Clean (1)
- [#3853 fix(swift-sdk): drop legacy headers pre-processing in build_ios.sh](https://github.com/dashpay/platform/pull/3853) — ⚠ merge conflict

<a id="lklimek"></a>
### @lklimek + (@Claudius-Maginificent)
<a id="lklimek-open"></a>
#### Open (11)
- [#3549 test(platform-wallet): e2e framework + full test suite — triage pins, Found-*/PA-* guards, fail-closed persist, Stage-2 merge](https://github.com/dashpay/platform/pull/3549) — via @Claudius-Maginificent · 20 unresolved (20 human) · 48 days stale · 📝 draft
  - Top thread: "<!-- thepastaclaw-review v1 finding=a3725e9d50ca -->" — 48 days old
- [#2679 fix(drive-abci): don't panic on unsupported version error](https://github.com/dashpay/platform/pull/2679) — ⏸ deferred
- [#2795 feat(platform): add real-time platform event subscriptions via gRPC streaming](https://github.com/dashpay/platform/pull/2795) — ⏸ deferred
- [#2993 feat: identity reference validation](https://github.com/dashpay/platform/pull/2993) — 📝 draft · ⏸ deferred
- [#3009 feat: contract reference validation](https://github.com/dashpay/platform/pull/3009) — 📝 draft · ⏸ deferred
- [#3032 feat: document creation restriction by group membership](https://github.com/dashpay/platform/pull/3032) — 📝 draft · ⏸ deferred
- [#3650 fix(sdk): address-sync no longer silently discards balance changes for post-snapshot addresses (Found-025)](https://github.com/dashpay/platform/pull/3650)
- [#3692 feat(platform-wallet): watch-only rehydration from persistor (seedless load)](https://github.com/dashpay/platform/pull/3692) — via @Claudius-Maginificent · 📝 draft
- [#3750 feat(platform-wallet)!: [NO MERGE\] consumer hardening — CODE-001/003-callsite/017/018 + PROJ-001 FFI + CODE-008/012/013](https://github.com/dashpay/platform/pull/3750) — via @Claudius-Maginificent · ⚠ merge conflict · 📝 draft
- [#3828 fix(platform-wallet): in-band pool manifest fixes genesis-rescan flush loop; retire mirror/reconcile](https://github.com/dashpay/platform/pull/3828) — via @Claudius-Maginificent · 🐢 targets feat/platform-wallet-rehydration
- [#3940 build(dashmate): update Tenderdash image to v1.6.0](https://github.com/dashpay/platform/pull/3940) — via @Claudius-Maginificent · 📝 draft

<a id="lklimek-deferred"></a>
#### Deferred (5)
- [#2679 fix(drive-abci): don't panic on unsupported version error](https://github.com/dashpay/platform/pull/2679) — ⏸ deferred
- [#2795 feat(platform): add real-time platform event subscriptions via gRPC streaming](https://github.com/dashpay/platform/pull/2795) — ⏸ deferred
- [#2993 feat: identity reference validation](https://github.com/dashpay/platform/pull/2993) — 📝 draft · ⏸ deferred
- [#3009 feat: contract reference validation](https://github.com/dashpay/platform/pull/3009) — 📝 draft · ⏸ deferred
- [#3032 feat: document creation restriction by group membership](https://github.com/dashpay/platform/pull/3032) — 📝 draft · ⏸ deferred

<a id="lklimek-draft"></a>
#### Draft (4)
- [#3549 test(platform-wallet): e2e framework + full test suite — triage pins, Found-*/PA-* guards, fail-closed persist, Stage-2 merge](https://github.com/dashpay/platform/pull/3549) — via @Claudius-Maginificent · 20 unresolved (20 human) · 48 days stale · 📝 draft
  - Top thread: "<!-- thepastaclaw-review v1 finding=a3725e9d50ca -->" — 48 days old
- [#3692 feat(platform-wallet): watch-only rehydration from persistor (seedless load)](https://github.com/dashpay/platform/pull/3692) — via @Claudius-Maginificent · 📝 draft
- [#3750 feat(platform-wallet)!: [NO MERGE\] consumer hardening — CODE-001/003-callsite/017/018 + PROJ-001 FFI + CODE-008/012/013](https://github.com/dashpay/platform/pull/3750) — via @Claudius-Maginificent · ⚠ merge conflict · 📝 draft
- [#3940 build(dashmate): update Tenderdash image to v1.6.0](https://github.com/dashpay/platform/pull/3940) — via @Claudius-Maginificent · 📝 draft

<a id="lklimek-stale"></a>
#### Stale (1)
- [#3828 fix(platform-wallet): in-band pool manifest fixes genesis-rescan flush loop; retire mirror/reconcile](https://github.com/dashpay/platform/pull/3828) — via @Claudius-Maginificent · 🐢 targets feat/platform-wallet-rehydration

<a id="lklimek-clean"></a>
#### Clean (1)
- [#3650 fix(sdk): address-sync no longer silently discards balance changes for post-snapshot addresses (Found-025)](https://github.com/dashpay/platform/pull/3650)

<a id="lklimek-ready-for-review"></a>
#### Ready for Review (1)
- [#3938 test(rs-sdk): expect network floor in mock sdk seed test](https://github.com/dashpay/platform/pull/3938) — by @thepastaclaw

<a id="pshenmic"></a>
### @pshenmic
<a id="pshenmic-open"></a>
#### Open (1)
- [#2446 feat(js-dapi-client): add contested resources query methods](https://github.com/dashpay/platform/pull/2446) — 5 unresolved (5 CodeRabbit) · 506 days stale · ⚠ merge conflict · 📝 draft · 🐢 targets v2.0-dev, untouched 497 days
  - Top thread: "_:warning: Potential issue_" — 506 days old

<a id="pshenmic-stale"></a>
#### Stale (1)
- [#2446 feat(js-dapi-client): add contested resources query methods](https://github.com/dashpay/platform/pull/2446) — 5 unresolved (5 CodeRabbit) · 506 days stale · ⚠ merge conflict · 📝 draft · 🐢 targets v2.0-dev, untouched 497 days
  - Top thread: "_:warning: Potential issue_" — 506 days old

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

<a id="thephez"></a>
### @thephez
<a id="thephez-open"></a>
#### Open (2)
- [#3849 test(drive-abci): add token supply edge-case coverage](https://github.com/dashpay/platform/pull/3849) — 🔴 CI failing
- [#3928 test(dpp,drive-abci): pin keepHistory + canBeDeleted contradiction](https://github.com/dashpay/platform/pull/3928) — 📝 draft

<a id="thephez-ci-failing"></a>
#### CI Failing (1)
- [#3849 test(drive-abci): add token supply edge-case coverage](https://github.com/dashpay/platform/pull/3849) — 🔴 CI failing

<a id="thephez-draft"></a>
#### Draft (1)
- [#3928 test(dpp,drive-abci): pin keepHistory + canBeDeleted contradiction](https://github.com/dashpay/platform/pull/3928) — 📝 draft

## Methodology
Generated nightly by [pr-hygiene](https://github.com/dashpay/stale_prs_are_bad). A thread counts as "unresolved" when it is open, not outdated, has a comment from someone other than the PR author, and the most recent comment is from a reviewer. **Dirty** = at least one such thread. **Unresolved Comments** = at least one such thread. **Changes Requested** = no unresolved threads but a reviewer's most recent review is CHANGES_REQUESTED (still blocking until someone re-approves or dismisses). **Deferred** = carries a configured deferred label (e.g. `postponed`) — visible but not counted toward unresolved-comment counts. **Stale** = targets a non-default branch OR hasn't been touched in the configured threshold (default 120 days, but clean PRs are never reclassified as stale). **Draft** = the PR is still marked draft on GitHub. **CI failing** = no unresolved comments, no changes-requested, but the latest commit's status check is failing. **Clean** = open, not draft, not deferred, not stale, no unresolved comments, no changes-requested, CI green. **Needs action** further requires changes-requested, merge conflict, or that the reviewer commented more recently than the author last pushed. **Ready for Review** counts clean PRs (authored by someone else) where this person owes a review. When a `review_routing` rule matches a PR's changed files, the routed reviewer IS the queue (explicit GitHub reviewers are ignored); a routed reviewer who has already submitted any review is excluded — their job is done. Configurable via [`https://github.com/dashpay/stale_prs_are_bad/blob/master/.pr-hygiene.yml`](https://github.com/dashpay/stale_prs_are_bad/blob/master/.pr-hygiene.yml)—edit defaults there.
