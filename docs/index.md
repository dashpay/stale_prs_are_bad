---
---
# PR Hygiene Report
*Last updated: 2026-05-19 01:48 UTC · commit c897952*

## Summary
- Open PRs: **65** (0 clean · 3 CI failing · 3 changes requested · 20 unresolved comments · 9 deferred · 16 draft · 14 stale)
- PRs needing author action: **20**
- Total unresolved comments: **194**

## Scoreboard
_Sort: unresolved-comments desc → needs-action desc → ready-for-review desc. Click any number to jump to the specific PRs it covers._

| Author | Open | Clean | CI failing | Unresolved Comments | Changes Requested | Deferred | Draft | Stale | Needs action | Total Unresolved Comments | Ready for Review |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| [@lklimek + (@Claudius-Maginificent)](#lklimek) | [13+(5)](#lklimek-open) | — | [1+(0)](#lklimek-ci-failing) | [2+(3)](#lklimek-unresolved-comments) | [3+(0)](#lklimek-changes-requested) | [5+(0)](#lklimek-deferred) | [2+(1)](#lklimek-draft) | [0+(1)](#lklimek-stale) | [5+(3)](#lklimek-needs-action) | [8+(37)](#lklimek-unresolved-comments) | — |
| [@PastaPastaPasta + (@thepastaclaw)](#pastapastapasta) | [7+(7)](#pastapastapasta-open) | — | — | [3+(1)](#pastapastapasta-unresolved-comments) | — | — | [2+(6)](#pastapastapasta-draft) | [2+(0)](#pastapastapasta-stale) | [3+(0)](#pastapastapasta-needs-action) | [21+(7)](#pastapastapasta-unresolved-comments) | — |
| [@QuantumExplorer](#quantumexplorer) | [6](#quantumexplorer-open) | — | — | [3](#quantumexplorer-unresolved-comments) | — | [2](#quantumexplorer-deferred) | [1](#quantumexplorer-draft) | — | [2](#quantumexplorer-needs-action) | [42](#quantumexplorer-unresolved-comments) | — |
| [@shumkov](#shumkov) | [8](#shumkov-open) | — | [1](#shumkov-ci-failing) | [2](#shumkov-unresolved-comments) | — | [1](#shumkov-deferred) | [1](#shumkov-draft) | [3](#shumkov-stale) | [2](#shumkov-needs-action) | [15](#shumkov-unresolved-comments) | — |
| [@ZocoLini](#zocolini) | [5](#zocolini-open) | — | [1](#zocolini-ci-failing) | [2](#zocolini-unresolved-comments) | — | — | [2](#zocolini-draft) | — | [2](#zocolini-needs-action) | [6](#zocolini-unresolved-comments) | — |
| [@thephez](#thephez) | [2](#thephez-open) | — | — | [2](#thephez-unresolved-comments) | — | — | — | — | [2](#thephez-needs-action) | [6](#thephez-unresolved-comments) | — |
| [@ktechmidas](#ktechmidas) | [1](#ktechmidas-open) | — | — | [1](#ktechmidas-unresolved-comments) | — | — | — | — | [1](#ktechmidas-needs-action) | [6](#ktechmidas-unresolved-comments) | — |
| [@llbartekll](#llbartekll) | [2](#llbartekll-open) | — | — | [1](#llbartekll-unresolved-comments) | — | — | [1](#llbartekll-draft) | — | — | [16](#llbartekll-unresolved-comments) | — |
| [@pshenmic](#pshenmic) | [4](#pshenmic-open) | — | — | — | — | — | — | [4](#pshenmic-stale) | — | [29](#pshenmic-unresolved-comments) | — |
| [@Inna333-cuber](#inna333-cuber) | [1](#inna333-cuber-open) | — | — | — | — | — | — | [1](#inna333-cuber-stale) | — | [1](#inna333-cuber-unresolved-comments) | — |
| [@ogabrielides](#ogabrielides) | [1](#ogabrielides-open) | — | — | — | — | [1](#ogabrielides-deferred) | — | — | — | — | — |
| [@pauldelucia](#pauldelucia) | [3](#pauldelucia-open) | — | — | — | — | — | — | [3](#pauldelucia-stale) | — | — | — |

## Per-author detail

<a id="lklimek"></a>
### @lklimek + (@Claudius-Maginificent)
<a id="lklimek-open"></a>
#### Open (18)
- [#3549 test(rs-platform-wallet): e2e suite, Found-025 fix + triage pins](https://github.com/dashpay/platform/pull/3549) — via @Claudius-Maginificent · 22 unresolved (22 human) · 16 days stale · 🔴 CI failing · 📝 draft · 🐢 targets fix/rs-platform-wallet-auto-select-inputs
  - Top thread: "<!-- thepastaclaw-review v1 finding=a3725e9d50ca -->" — 16 days old
- [#3625 feat(platform-wallet): add platform-wallet-storage crate (sqlite persister)](https://github.com/dashpay/platform/pull/3625) — via @Claudius-Maginificent · 8 unresolved (8 CodeRabbit) · 7 days stale · ⚠ merge conflict · 📝 draft
  - Top thread: "_⚠️ Potential issue_ \| _🟠 Major_ \| _⚡ Quick win_" — 7 days old
- [#3350 fix(rs-dapi,sdk): decode base64 CBOR error messages from Tenderdash](https://github.com/dashpay/platform/pull/3350) — 3 unresolved (3 human) · 9 days stale · ✋ changes requested
  - Top thread: "<!-- thepastaclaw-review v1 finding=7594525c23d8 -->" — 9 days old
- [#3636 feat(platform-wallet): add birth_height_override to wallet creation API](https://github.com/dashpay/platform/pull/3636) — 5 unresolved (1 CodeRabbit, 4 human) · 6 days stale
  - Top thread: "_⚠️ Potential issue_ \| _🟡 Minor_ \| _⚡ Quick win_" — 6 days old
- [#3637 feat(platform-wallet): serde support](https://github.com/dashpay/platform/pull/3637) — via @Claudius-Maginificent · 5 unresolved (1 CodeRabbit, 4 human) · 5 days stale · ⚠ merge conflict
  - Top thread: "_⚠️ Potential issue_ \| _🔴 Critical_" — 5 days old
- [#3585 fix: case-insensitive .dash + atomic state on broadcast failure](https://github.com/dashpay/platform/pull/3585) — via @Claudius-Maginificent · 1 unresolved (1 human) · 7 days stale · ✋ changes requested
  - Top thread: "<!-- thepastaclaw-review v1 finding=6301a94999c2 -->" — 7 days old
- [#3554 fix(platform-wallet): auto_select_inputs honors Σ inputs == Σ outputs](https://github.com/dashpay/platform/pull/3554) — via @Claudius-Maginificent · 1 unresolved (1 human) · 6 days stale · ✋ changes requested
  - Top thread: "<!-- thepastaclaw-review v1 finding=5faa46ca0095 dedupe=97e40e670beeb9b9 -->" — 6 days old
- [#2679 fix(drive-abci): don't panic on unsupported version error](https://github.com/dashpay/platform/pull/2679) — ⏸ deferred
- [#2795 feat(platform): add real-time platform event subscriptions via gRPC streaming](https://github.com/dashpay/platform/pull/2795) — ⏸ deferred
- [#2993 feat: identity reference validation](https://github.com/dashpay/platform/pull/2993) — 📝 draft · ⏸ deferred
- [#3009 feat: contract reference validation](https://github.com/dashpay/platform/pull/3009) — 📝 draft · ⏸ deferred
- [#3032 feat: document creation restriction by group membership](https://github.com/dashpay/platform/pull/3032) — 📝 draft · ⏸ deferred
- [#3647 feat(platform-wallet): watermark monotonic-merge](https://github.com/dashpay/platform/pull/3647) — ✋ changes requested
- [#3648 fix(platform-wallet): local-ledger ownership guard (V27-007)](https://github.com/dashpay/platform/pull/3648) — ✋ changes requested
- [#3650 fix(sdk): address-sync no longer silently discards balance changes for post-snapshot addresses (Found-025)](https://github.com/dashpay/platform/pull/3650) — 🔴 CI failing
- [#3651 feat(platform-wallet): SPV cancel_background/identity_ids accessors + FFI no-selectable-inputs error mapping](https://github.com/dashpay/platform/pull/3651) — ✋ changes requested · 🔴 CI failing
- [#3658 fix(rs-platform-wallet): reserve platform receive address on hand-out (Found-026) [backport\]](https://github.com/dashpay/platform/pull/3658) — 🔴 CI failing · 📝 draft
- [#3659 fix(rs-platform-wallet): fail-closed on registration persist error (Found-017) [backport\]](https://github.com/dashpay/platform/pull/3659) — 🔴 CI failing · 📝 draft

<a id="lklimek-needs-action"></a>
#### Needs action (8)
- [#3350 fix(rs-dapi,sdk): decode base64 CBOR error messages from Tenderdash](https://github.com/dashpay/platform/pull/3350) — 3 unresolved (3 human) · 9 days stale · ✋ changes requested
  - Top thread: "<!-- thepastaclaw-review v1 finding=7594525c23d8 -->" — 9 days old
- [#3636 feat(platform-wallet): add birth_height_override to wallet creation API](https://github.com/dashpay/platform/pull/3636) — 5 unresolved (1 CodeRabbit, 4 human) · 6 days stale
  - Top thread: "_⚠️ Potential issue_ \| _🟡 Minor_ \| _⚡ Quick win_" — 6 days old
- [#3637 feat(platform-wallet): serde support](https://github.com/dashpay/platform/pull/3637) — via @Claudius-Maginificent · 5 unresolved (1 CodeRabbit, 4 human) · 5 days stale · ⚠ merge conflict
  - Top thread: "_⚠️ Potential issue_ \| _🔴 Critical_" — 5 days old
- [#3585 fix: case-insensitive .dash + atomic state on broadcast failure](https://github.com/dashpay/platform/pull/3585) — via @Claudius-Maginificent · 1 unresolved (1 human) · 7 days stale · ✋ changes requested
  - Top thread: "<!-- thepastaclaw-review v1 finding=6301a94999c2 -->" — 7 days old
- [#3554 fix(platform-wallet): auto_select_inputs honors Σ inputs == Σ outputs](https://github.com/dashpay/platform/pull/3554) — via @Claudius-Maginificent · 1 unresolved (1 human) · 6 days stale · ✋ changes requested
  - Top thread: "<!-- thepastaclaw-review v1 finding=5faa46ca0095 dedupe=97e40e670beeb9b9 -->" — 6 days old
- [#3647 feat(platform-wallet): watermark monotonic-merge](https://github.com/dashpay/platform/pull/3647) — ✋ changes requested
- [#3648 fix(platform-wallet): local-ledger ownership guard (V27-007)](https://github.com/dashpay/platform/pull/3648) — ✋ changes requested
- [#3651 feat(platform-wallet): SPV cancel_background/identity_ids accessors + FFI no-selectable-inputs error mapping](https://github.com/dashpay/platform/pull/3651) — ✋ changes requested · 🔴 CI failing

<a id="lklimek-unresolved-comments"></a>
#### Unresolved Comments (5)
- [#3350 fix(rs-dapi,sdk): decode base64 CBOR error messages from Tenderdash](https://github.com/dashpay/platform/pull/3350) — 3 unresolved (3 human) · 9 days stale · ✋ changes requested
  - Top thread: "<!-- thepastaclaw-review v1 finding=7594525c23d8 -->" — 9 days old
- [#3636 feat(platform-wallet): add birth_height_override to wallet creation API](https://github.com/dashpay/platform/pull/3636) — 5 unresolved (1 CodeRabbit, 4 human) · 6 days stale
  - Top thread: "_⚠️ Potential issue_ \| _🟡 Minor_ \| _⚡ Quick win_" — 6 days old
- [#3637 feat(platform-wallet): serde support](https://github.com/dashpay/platform/pull/3637) — via @Claudius-Maginificent · 5 unresolved (1 CodeRabbit, 4 human) · 5 days stale · ⚠ merge conflict
  - Top thread: "_⚠️ Potential issue_ \| _🔴 Critical_" — 5 days old
- [#3585 fix: case-insensitive .dash + atomic state on broadcast failure](https://github.com/dashpay/platform/pull/3585) — via @Claudius-Maginificent · 1 unresolved (1 human) · 7 days stale · ✋ changes requested
  - Top thread: "<!-- thepastaclaw-review v1 finding=6301a94999c2 -->" — 7 days old
- [#3554 fix(platform-wallet): auto_select_inputs honors Σ inputs == Σ outputs](https://github.com/dashpay/platform/pull/3554) — via @Claudius-Maginificent · 1 unresolved (1 human) · 6 days stale · ✋ changes requested
  - Top thread: "<!-- thepastaclaw-review v1 finding=5faa46ca0095 dedupe=97e40e670beeb9b9 -->" — 6 days old

<a id="lklimek-changes-requested"></a>
#### Changes Requested (3)
- [#3647 feat(platform-wallet): watermark monotonic-merge](https://github.com/dashpay/platform/pull/3647) — ✋ changes requested
- [#3648 fix(platform-wallet): local-ledger ownership guard (V27-007)](https://github.com/dashpay/platform/pull/3648) — ✋ changes requested
- [#3651 feat(platform-wallet): SPV cancel_background/identity_ids accessors + FFI no-selectable-inputs error mapping](https://github.com/dashpay/platform/pull/3651) — ✋ changes requested · 🔴 CI failing

<a id="lklimek-ci-failing"></a>
#### CI Failing (1)
- [#3650 fix(sdk): address-sync no longer silently discards balance changes for post-snapshot addresses (Found-025)](https://github.com/dashpay/platform/pull/3650) — 🔴 CI failing

<a id="lklimek-deferred"></a>
#### Deferred (5)
- [#2679 fix(drive-abci): don't panic on unsupported version error](https://github.com/dashpay/platform/pull/2679) — ⏸ deferred
- [#2795 feat(platform): add real-time platform event subscriptions via gRPC streaming](https://github.com/dashpay/platform/pull/2795) — ⏸ deferred
- [#2993 feat: identity reference validation](https://github.com/dashpay/platform/pull/2993) — 📝 draft · ⏸ deferred
- [#3009 feat: contract reference validation](https://github.com/dashpay/platform/pull/3009) — 📝 draft · ⏸ deferred
- [#3032 feat: document creation restriction by group membership](https://github.com/dashpay/platform/pull/3032) — 📝 draft · ⏸ deferred

<a id="lklimek-draft"></a>
#### Draft (3)
- [#3625 feat(platform-wallet): add platform-wallet-storage crate (sqlite persister)](https://github.com/dashpay/platform/pull/3625) — via @Claudius-Maginificent · 8 unresolved (8 CodeRabbit) · 7 days stale · ⚠ merge conflict · 📝 draft
  - Top thread: "_⚠️ Potential issue_ \| _🟠 Major_ \| _⚡ Quick win_" — 7 days old
- [#3658 fix(rs-platform-wallet): reserve platform receive address on hand-out (Found-026) [backport\]](https://github.com/dashpay/platform/pull/3658) — 🔴 CI failing · 📝 draft
- [#3659 fix(rs-platform-wallet): fail-closed on registration persist error (Found-017) [backport\]](https://github.com/dashpay/platform/pull/3659) — 🔴 CI failing · 📝 draft

<a id="lklimek-stale"></a>
#### Stale (1)
- [#3549 test(rs-platform-wallet): e2e suite, Found-025 fix + triage pins](https://github.com/dashpay/platform/pull/3549) — via @Claudius-Maginificent · 22 unresolved (22 human) · 16 days stale · 🔴 CI failing · 📝 draft · 🐢 targets fix/rs-platform-wallet-auto-select-inputs
  - Top thread: "<!-- thepastaclaw-review v1 finding=a3725e9d50ca -->" — 16 days old

<a id="pastapastapasta"></a>
### @PastaPastaPasta + (@thepastaclaw)
<a id="pastapastapasta-open"></a>
#### Open (14)
- [#3509 fix(platform): default omitted proved query limits](https://github.com/dashpay/platform/pull/3509) — 14 unresolved (1 CodeRabbit, 13 human) · 27 days stale · ✋ changes requested · 🔴 CI failing
  - Top thread: "_⚠️ Potential issue_ \| _🟠 Major_" — 27 days old
- [#3096 feat(sdk): add client-side validation to state transition construction methods](https://github.com/dashpay/platform/pull/3096) — via @thepastaclaw · 4 unresolved (2 CodeRabbit, 2 human) · 90 days stale · 📝 draft
  - Top thread: "_⚠️ Potential issue_ \| _🟡 Minor_" — 87 days old
- [#3165 fix(drive): consolidate historical contract proof verification retry logic](https://github.com/dashpay/platform/pull/3165) — via @thepastaclaw · 3 unresolved (3 human) · 74 days stale
  - Top thread: "PROJ-003** (INFO) — No coverage for `in_multiple_contract_proof_form=true`" — 74 days old
- [#3657 fix(wasm-sdk): support binary grove path elements](https://github.com/dashpay/platform/pull/3657) — 5 unresolved (5 human) · 0 days stale
  - Top thread: "<!-- thepastaclaw-review v1 finding=e648cab09e16 dedupe=978d8e56f1ea50a0 -->" — 0 days old
- [#3663 ci: include omitted rust packages in ci filters](https://github.com/dashpay/platform/pull/3663) — 2 unresolved (2 human) · 0 days stale
  - Top thread: "<!-- thepastaclaw-review v1 finding=867572ccbe1b dedupe=59d0d71e5f8ee2e0 -->" — 0 days old
- [#2988 feat(wasm-sdk): auto-generate entropy for document creation when not provided](https://github.com/dashpay/platform/pull/2988) — 🔴 CI failing · 🐢 targets v3.0-dev
- [#2989 feat(sdk): add credentials-based API for DPNS registerName](https://github.com/dashpay/platform/pull/2989) — ✋ changes requested · 🔴 CI failing · 🐢 targets v3.0-dev
- [#3091 feat(wasm-sdk): add prepare_* APIs for idempotent document state transitions](https://github.com/dashpay/platform/pull/3091) — via @thepastaclaw · ✋ changes requested · 📝 draft
- [#3092 feat(rs-sdk): expose transition hash from state transition methods](https://github.com/dashpay/platform/pull/3092) — via @thepastaclaw · ✋ changes requested · 🔴 CI failing · 📝 draft
- [#3133 fix(sdk): validate batch base structure before signing](https://github.com/dashpay/platform/pull/3133) — via @thepastaclaw · ✋ changes requested · 📝 draft
- [#3461 fix(dpp): block pre-programmed distribution changes on token update](https://github.com/dashpay/platform/pull/3461) — 📝 draft
- [#3462 fix(dpp)!: version-gate distribution function floating-point evaluation](https://github.com/dashpay/platform/pull/3462) — 📝 draft
- [#3596 fix(platform-wallet): satisfy accessors clippy lints](https://github.com/dashpay/platform/pull/3596) — via @thepastaclaw · 📝 draft
- [#3635 ci: tolerate book preview comment permission errors](https://github.com/dashpay/platform/pull/3635) — via @thepastaclaw · 📝 draft

<a id="pastapastapasta-needs-action"></a>
#### Needs action (3)
- [#3509 fix(platform): default omitted proved query limits](https://github.com/dashpay/platform/pull/3509) — 14 unresolved (1 CodeRabbit, 13 human) · 27 days stale · ✋ changes requested · 🔴 CI failing
  - Top thread: "_⚠️ Potential issue_ \| _🟠 Major_" — 27 days old
- [#3657 fix(wasm-sdk): support binary grove path elements](https://github.com/dashpay/platform/pull/3657) — 5 unresolved (5 human) · 0 days stale
  - Top thread: "<!-- thepastaclaw-review v1 finding=e648cab09e16 dedupe=978d8e56f1ea50a0 -->" — 0 days old
- [#3663 ci: include omitted rust packages in ci filters](https://github.com/dashpay/platform/pull/3663) — 2 unresolved (2 human) · 0 days stale
  - Top thread: "<!-- thepastaclaw-review v1 finding=867572ccbe1b dedupe=59d0d71e5f8ee2e0 -->" — 0 days old

<a id="pastapastapasta-unresolved-comments"></a>
#### Unresolved Comments (4)
- [#3509 fix(platform): default omitted proved query limits](https://github.com/dashpay/platform/pull/3509) — 14 unresolved (1 CodeRabbit, 13 human) · 27 days stale · ✋ changes requested · 🔴 CI failing
  - Top thread: "_⚠️ Potential issue_ \| _🟠 Major_" — 27 days old
- [#3165 fix(drive): consolidate historical contract proof verification retry logic](https://github.com/dashpay/platform/pull/3165) — via @thepastaclaw · 3 unresolved (3 human) · 74 days stale
  - Top thread: "PROJ-003** (INFO) — No coverage for `in_multiple_contract_proof_form=true`" — 74 days old
- [#3657 fix(wasm-sdk): support binary grove path elements](https://github.com/dashpay/platform/pull/3657) — 5 unresolved (5 human) · 0 days stale
  - Top thread: "<!-- thepastaclaw-review v1 finding=e648cab09e16 dedupe=978d8e56f1ea50a0 -->" — 0 days old
- [#3663 ci: include omitted rust packages in ci filters](https://github.com/dashpay/platform/pull/3663) — 2 unresolved (2 human) · 0 days stale
  - Top thread: "<!-- thepastaclaw-review v1 finding=867572ccbe1b dedupe=59d0d71e5f8ee2e0 -->" — 0 days old

<a id="pastapastapasta-draft"></a>
#### Draft (8)
- [#3096 feat(sdk): add client-side validation to state transition construction methods](https://github.com/dashpay/platform/pull/3096) — via @thepastaclaw · 4 unresolved (2 CodeRabbit, 2 human) · 90 days stale · 📝 draft
  - Top thread: "_⚠️ Potential issue_ \| _🟡 Minor_" — 87 days old
- [#3091 feat(wasm-sdk): add prepare_* APIs for idempotent document state transitions](https://github.com/dashpay/platform/pull/3091) — via @thepastaclaw · ✋ changes requested · 📝 draft
- [#3092 feat(rs-sdk): expose transition hash from state transition methods](https://github.com/dashpay/platform/pull/3092) — via @thepastaclaw · ✋ changes requested · 🔴 CI failing · 📝 draft
- [#3133 fix(sdk): validate batch base structure before signing](https://github.com/dashpay/platform/pull/3133) — via @thepastaclaw · ✋ changes requested · 📝 draft
- [#3461 fix(dpp): block pre-programmed distribution changes on token update](https://github.com/dashpay/platform/pull/3461) — 📝 draft
- [#3462 fix(dpp)!: version-gate distribution function floating-point evaluation](https://github.com/dashpay/platform/pull/3462) — 📝 draft
- [#3596 fix(platform-wallet): satisfy accessors clippy lints](https://github.com/dashpay/platform/pull/3596) — via @thepastaclaw · 📝 draft
- [#3635 ci: tolerate book preview comment permission errors](https://github.com/dashpay/platform/pull/3635) — via @thepastaclaw · 📝 draft

<a id="pastapastapasta-stale"></a>
#### Stale (2)
- [#2988 feat(wasm-sdk): auto-generate entropy for document creation when not provided](https://github.com/dashpay/platform/pull/2988) — 🔴 CI failing · 🐢 targets v3.0-dev
- [#2989 feat(sdk): add credentials-based API for DPNS registerName](https://github.com/dashpay/platform/pull/2989) — ✋ changes requested · 🔴 CI failing · 🐢 targets v3.0-dev

<a id="quantumexplorer"></a>
### @QuantumExplorer
<a id="quantumexplorer-open"></a>
#### Open (6)
- [#3603 feat(swift-sdk,platform-wallet): wire shielded send end-to-end (all 4 transitions)](https://github.com/dashpay/platform/pull/3603) — 28 unresolved (9 CodeRabbit, 19 human) · 13 days stale · ⚠ merge conflict
  - Top thread: "_⚠️ Potential issue_ \| _🔴 Critical_ \| _⚡ Quick win_" — 13 days old
- [#3557 feat(platform-wallet): instrument wallet_manager RwLock behind lock-stats feature](https://github.com/dashpay/platform/pull/3557) — 11 unresolved (2 CodeRabbit, 9 human) · 21 days stale
  - Top thread: "_⚠️ Potential issue_ \| _🟠 Major_" — 21 days old
- [#3661 feat(drive): document sum + average proof primitives, with SDK fan-out scaffolding and reproducible benchmarks](https://github.com/dashpay/platform/pull/3661) — 3 unresolved (3 CodeRabbit) · 0 days stale · 🔴 CI failing
  - Top thread: "_⚠️ Potential issue_ \| _🟠 Major_ \| _🏗️ Heavy lift_" — 0 days old
- [#1834 feat(drive-abci)!: Statesync](https://github.com/dashpay/platform/pull/1834) — 📝 draft · ⏸ deferred
- [#3021 feat(platform): better contract state transitions](https://github.com/dashpay/platform/pull/3021) — 📝 draft · ⏸ deferred
- [#3417 feat(swift-sdk): use SPV-synced quorums for Platform proof verification](https://github.com/dashpay/platform/pull/3417) — ⚠ merge conflict · 🔴 CI failing · 📝 draft

<a id="quantumexplorer-needs-action"></a>
#### Needs action (2)
- [#3603 feat(swift-sdk,platform-wallet): wire shielded send end-to-end (all 4 transitions)](https://github.com/dashpay/platform/pull/3603) — 28 unresolved (9 CodeRabbit, 19 human) · 13 days stale · ⚠ merge conflict
  - Top thread: "_⚠️ Potential issue_ \| _🔴 Critical_ \| _⚡ Quick win_" — 13 days old
- [#3557 feat(platform-wallet): instrument wallet_manager RwLock behind lock-stats feature](https://github.com/dashpay/platform/pull/3557) — 11 unresolved (2 CodeRabbit, 9 human) · 21 days stale
  - Top thread: "_⚠️ Potential issue_ \| _🟠 Major_" — 21 days old

<a id="quantumexplorer-unresolved-comments"></a>
#### Unresolved Comments (3)
- [#3603 feat(swift-sdk,platform-wallet): wire shielded send end-to-end (all 4 transitions)](https://github.com/dashpay/platform/pull/3603) — 28 unresolved (9 CodeRabbit, 19 human) · 13 days stale · ⚠ merge conflict
  - Top thread: "_⚠️ Potential issue_ \| _🔴 Critical_ \| _⚡ Quick win_" — 13 days old
- [#3557 feat(platform-wallet): instrument wallet_manager RwLock behind lock-stats feature](https://github.com/dashpay/platform/pull/3557) — 11 unresolved (2 CodeRabbit, 9 human) · 21 days stale
  - Top thread: "_⚠️ Potential issue_ \| _🟠 Major_" — 21 days old
- [#3661 feat(drive): document sum + average proof primitives, with SDK fan-out scaffolding and reproducible benchmarks](https://github.com/dashpay/platform/pull/3661) — 3 unresolved (3 CodeRabbit) · 0 days stale · 🔴 CI failing
  - Top thread: "_⚠️ Potential issue_ \| _🟠 Major_ \| _🏗️ Heavy lift_" — 0 days old

<a id="quantumexplorer-deferred"></a>
#### Deferred (2)
- [#1834 feat(drive-abci)!: Statesync](https://github.com/dashpay/platform/pull/1834) — 📝 draft · ⏸ deferred
- [#3021 feat(platform): better contract state transitions](https://github.com/dashpay/platform/pull/3021) — 📝 draft · ⏸ deferred

<a id="quantumexplorer-draft"></a>
#### Draft (1)
- [#3417 feat(swift-sdk): use SPV-synced quorums for Platform proof verification](https://github.com/dashpay/platform/pull/3417) — ⚠ merge conflict · 🔴 CI failing · 📝 draft

<a id="shumkov"></a>
### @shumkov
<a id="shumkov-open"></a>
#### Open (8)
- [#3166 chore: integrate Claude Code into devcontainer for autonomus development](https://github.com/dashpay/platform/pull/3166) — 8 unresolved (8 human) · 73 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing
  - Top thread: "as above." — 73 days old
- [#3634 feat: identity registration with asset-lock proofs](https://github.com/dashpay/platform/pull/3634) — 7 unresolved (1 CodeRabbit, 6 human) · 5 days stale · ✋ changes requested
  - Top thread: "_⚠️ Potential issue_ \| _🟡 Minor_ \| _⚡ Quick win_" — 5 days old
- [#2392 feat(dashmate): single node local network [WIP\]](https://github.com/dashpay/platform/pull/2392) — 🔴 CI failing · 🐢 targets v2.0-dev
- [#2518 feat: token marketplace](https://github.com/dashpay/platform/pull/2518) — 📝 draft · ⏸ deferred
- [#2552 perf(dapi): fetch only specific unconfirmed transaction](https://github.com/dashpay/platform/pull/2552) — ⚠ merge conflict · 🔴 CI failing · 🐢 targets v2.1-dev
- [#2973 [temp\] Test non determinism](https://github.com/dashpay/platform/pull/2973) — ⚠ merge conflict · 🔴 CI failing · 📝 draft · 🐢 targets v3.0-dev, untouched 126 days
- [#3419 Feat/platform wallet](https://github.com/dashpay/platform/pull/3419) — ⚠ merge conflict · 🔴 CI failing · 📝 draft
- [#3573 feat(dpp): unify JSON/Value conversion traits + comprehensive round-trip tests](https://github.com/dashpay/platform/pull/3573) — 🔴 CI failing

<a id="shumkov-needs-action"></a>
#### Needs action (2)
- [#3166 chore: integrate Claude Code into devcontainer for autonomus development](https://github.com/dashpay/platform/pull/3166) — 8 unresolved (8 human) · 73 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing
  - Top thread: "as above." — 73 days old
- [#3634 feat: identity registration with asset-lock proofs](https://github.com/dashpay/platform/pull/3634) — 7 unresolved (1 CodeRabbit, 6 human) · 5 days stale · ✋ changes requested
  - Top thread: "_⚠️ Potential issue_ \| _🟡 Minor_ \| _⚡ Quick win_" — 5 days old

<a id="shumkov-unresolved-comments"></a>
#### Unresolved Comments (2)
- [#3166 chore: integrate Claude Code into devcontainer for autonomus development](https://github.com/dashpay/platform/pull/3166) — 8 unresolved (8 human) · 73 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing
  - Top thread: "as above." — 73 days old
- [#3634 feat: identity registration with asset-lock proofs](https://github.com/dashpay/platform/pull/3634) — 7 unresolved (1 CodeRabbit, 6 human) · 5 days stale · ✋ changes requested
  - Top thread: "_⚠️ Potential issue_ \| _🟡 Minor_ \| _⚡ Quick win_" — 5 days old

<a id="shumkov-ci-failing"></a>
#### CI Failing (1)
- [#3573 feat(dpp): unify JSON/Value conversion traits + comprehensive round-trip tests](https://github.com/dashpay/platform/pull/3573) — 🔴 CI failing

<a id="shumkov-deferred"></a>
#### Deferred (1)
- [#2518 feat: token marketplace](https://github.com/dashpay/platform/pull/2518) — 📝 draft · ⏸ deferred

<a id="shumkov-draft"></a>
#### Draft (1)
- [#3419 Feat/platform wallet](https://github.com/dashpay/platform/pull/3419) — ⚠ merge conflict · 🔴 CI failing · 📝 draft

<a id="shumkov-stale"></a>
#### Stale (3)
- [#2392 feat(dashmate): single node local network [WIP\]](https://github.com/dashpay/platform/pull/2392) — 🔴 CI failing · 🐢 targets v2.0-dev
- [#2552 perf(dapi): fetch only specific unconfirmed transaction](https://github.com/dashpay/platform/pull/2552) — ⚠ merge conflict · 🔴 CI failing · 🐢 targets v2.1-dev
- [#2973 [temp\] Test non determinism](https://github.com/dashpay/platform/pull/2973) — ⚠ merge conflict · 🔴 CI failing · 📝 draft · 🐢 targets v3.0-dev, untouched 126 days

<a id="zocolini"></a>
### @ZocoLini
<a id="zocolini-open"></a>
#### Open (5)
- [#3488 chore(swift-sdk): add wrappers for the missing TransactionRecord fields in the swift-sdk](https://github.com/dashpay/platform/pull/3488) — 5 unresolved (5 human) · 26 days stale · ✋ changes requested
  - Top thread: "<!-- thepastaclaw-review v1 finding=106ff7904b9f -->" — 26 days old
- [#3479 test(swift-sdk): swift-sdk test updated and added to CI](https://github.com/dashpay/platform/pull/3479) — 1 unresolved (1 human) · 3 days stale · ✋ changes requested
  - Top thread: "<!-- thepastaclaw-review v1 finding=a25233924084 dedupe=4de506d00bbda291 -->" — 3 days old
- [#3463 chore(swift-sdk): remove unused code in the swift-sdk and example app](https://github.com/dashpay/platform/pull/3463) — ⚠ merge conflict · ✋ changes requested · 📝 draft
- [#3639 feat: external signable wallets and tx building with signer](https://github.com/dashpay/platform/pull/3639) — ⚠ merge conflict · 📝 draft
- [#3664 refactor(swift-sdk): remove key-wallet-ffi crate usage, platform-wallet-ffi wraps all the logic](https://github.com/dashpay/platform/pull/3664) — 🔴 CI failing

<a id="zocolini-needs-action"></a>
#### Needs action (2)
- [#3488 chore(swift-sdk): add wrappers for the missing TransactionRecord fields in the swift-sdk](https://github.com/dashpay/platform/pull/3488) — 5 unresolved (5 human) · 26 days stale · ✋ changes requested
  - Top thread: "<!-- thepastaclaw-review v1 finding=106ff7904b9f -->" — 26 days old
- [#3479 test(swift-sdk): swift-sdk test updated and added to CI](https://github.com/dashpay/platform/pull/3479) — 1 unresolved (1 human) · 3 days stale · ✋ changes requested
  - Top thread: "<!-- thepastaclaw-review v1 finding=a25233924084 dedupe=4de506d00bbda291 -->" — 3 days old

<a id="zocolini-unresolved-comments"></a>
#### Unresolved Comments (2)
- [#3488 chore(swift-sdk): add wrappers for the missing TransactionRecord fields in the swift-sdk](https://github.com/dashpay/platform/pull/3488) — 5 unresolved (5 human) · 26 days stale · ✋ changes requested
  - Top thread: "<!-- thepastaclaw-review v1 finding=106ff7904b9f -->" — 26 days old
- [#3479 test(swift-sdk): swift-sdk test updated and added to CI](https://github.com/dashpay/platform/pull/3479) — 1 unresolved (1 human) · 3 days stale · ✋ changes requested
  - Top thread: "<!-- thepastaclaw-review v1 finding=a25233924084 dedupe=4de506d00bbda291 -->" — 3 days old

<a id="zocolini-ci-failing"></a>
#### CI Failing (1)
- [#3664 refactor(swift-sdk): remove key-wallet-ffi crate usage, platform-wallet-ffi wraps all the logic](https://github.com/dashpay/platform/pull/3664) — 🔴 CI failing

<a id="zocolini-draft"></a>
#### Draft (2)
- [#3463 chore(swift-sdk): remove unused code in the swift-sdk and example app](https://github.com/dashpay/platform/pull/3463) — ⚠ merge conflict · ✋ changes requested · 📝 draft
- [#3639 feat: external signable wallets and tx building with signer](https://github.com/dashpay/platform/pull/3639) — ⚠ merge conflict · 📝 draft

<a id="thephez"></a>
### @thephez
<a id="thephez-open"></a>
#### Open (2)
- [#3485 fix(wasm-sdk): update stale default testnet DAPI node addresses](https://github.com/dashpay/platform/pull/3485) — 4 unresolved (4 human) · 24 days stale · 🔴 CI failing
  - Top thread: "<!-- thepastaclaw-review v1 finding=b6db56a54dbd -->" — 24 days old
- [#3471 fix(dpp): remove erroneous keywords field from document-meta schema and fix contract keywords docs](https://github.com/dashpay/platform/pull/3471) — 2 unresolved (2 human) · 26 days stale · 🔴 CI failing
  - Top thread: "<!-- thepastaclaw-review v1 finding=72d0df4348f6 -->" — 26 days old

<a id="thephez-needs-action"></a>
#### Needs action (2)
- [#3485 fix(wasm-sdk): update stale default testnet DAPI node addresses](https://github.com/dashpay/platform/pull/3485) — 4 unresolved (4 human) · 24 days stale · 🔴 CI failing
  - Top thread: "<!-- thepastaclaw-review v1 finding=b6db56a54dbd -->" — 24 days old
- [#3471 fix(dpp): remove erroneous keywords field from document-meta schema and fix contract keywords docs](https://github.com/dashpay/platform/pull/3471) — 2 unresolved (2 human) · 26 days stale · 🔴 CI failing
  - Top thread: "<!-- thepastaclaw-review v1 finding=72d0df4348f6 -->" — 26 days old

<a id="thephez-unresolved-comments"></a>
#### Unresolved Comments (2)
- [#3485 fix(wasm-sdk): update stale default testnet DAPI node addresses](https://github.com/dashpay/platform/pull/3485) — 4 unresolved (4 human) · 24 days stale · 🔴 CI failing
  - Top thread: "<!-- thepastaclaw-review v1 finding=b6db56a54dbd -->" — 24 days old
- [#3471 fix(dpp): remove erroneous keywords field from document-meta schema and fix contract keywords docs](https://github.com/dashpay/platform/pull/3471) — 2 unresolved (2 human) · 26 days stale · 🔴 CI failing
  - Top thread: "<!-- thepastaclaw-review v1 finding=72d0df4348f6 -->" — 26 days old

<a id="ktechmidas"></a>
### @ktechmidas
<a id="ktechmidas-open"></a>
#### Open (1)
- [#3162 fix(dashmate): prevent orphaned verification container blocking SSL renewal](https://github.com/dashpay/platform/pull/3162) — 6 unresolved (6 human) · 0 days stale
  - Top thread: "<!-- thepastaclaw-review v1 finding=d7ee272d2ab4 dedupe=d8f79f6f47c868dc -->" — 0 days old

<a id="ktechmidas-needs-action"></a>
#### Needs action (1)
- [#3162 fix(dashmate): prevent orphaned verification container blocking SSL renewal](https://github.com/dashpay/platform/pull/3162) — 6 unresolved (6 human) · 0 days stale
  - Top thread: "<!-- thepastaclaw-review v1 finding=d7ee272d2ab4 dedupe=d8f79f6f47c868dc -->" — 0 days old

<a id="ktechmidas-unresolved-comments"></a>
#### Unresolved Comments (1)
- [#3162 fix(dashmate): prevent orphaned verification container blocking SSL renewal](https://github.com/dashpay/platform/pull/3162) — 6 unresolved (6 human) · 0 days stale
  - Top thread: "<!-- thepastaclaw-review v1 finding=d7ee272d2ab4 dedupe=d8f79f6f47c868dc -->" — 0 days old

<a id="llbartekll"></a>
### @llbartekll
<a id="llbartekll-open"></a>
#### Open (2)
- [#3560 test(swift-sdk): add testnet identity-discovery UI test](https://github.com/dashpay/platform/pull/3560) — 15 unresolved (1 CodeRabbit, 14 human) · 16 days stale · 🔴 CI failing
  - Top thread: "_⚠️ Potential issue_ \| _🟠 Major_ \| _🏗️ Heavy lift_" — 15 days old
- [#3481 feat(swift-sdk): expose rich transaction fields in WalletTransaction](https://github.com/dashpay/platform/pull/3481) — 1 unresolved (1 CodeRabbit) · 36 days stale · ⚠ merge conflict · 🔴 CI failing · 📝 draft
  - Top thread: "_⚠️ Potential issue_ \| _🟠 Major_" — 36 days old

<a id="llbartekll-unresolved-comments"></a>
#### Unresolved Comments (1)
- [#3560 test(swift-sdk): add testnet identity-discovery UI test](https://github.com/dashpay/platform/pull/3560) — 15 unresolved (1 CodeRabbit, 14 human) · 16 days stale · 🔴 CI failing
  - Top thread: "_⚠️ Potential issue_ \| _🟠 Major_ \| _🏗️ Heavy lift_" — 15 days old

<a id="llbartekll-draft"></a>
#### Draft (1)
- [#3481 feat(swift-sdk): expose rich transaction fields in WalletTransaction](https://github.com/dashpay/platform/pull/3481) — 1 unresolved (1 CodeRabbit) · 36 days stale · ⚠ merge conflict · 🔴 CI failing · 📝 draft
  - Top thread: "_⚠️ Potential issue_ \| _🟠 Major_" — 36 days old

<a id="pshenmic"></a>
### @pshenmic
<a id="pshenmic-open"></a>
#### Open (4)
- [#2444 feat(sdk): add NFT actions in the JS Dash SDK](https://github.com/dashpay/platform/pull/2444) — 17 unresolved (5 CodeRabbit, 12 human) · 446 days stale · ⚠ merge conflict · ✋ changes requested · 🐢 targets v2.0-dev
  - Top thread: "_:warning: Potential issue_" — 446 days old
- [#2465 fix(sdk): replace seeds with ip address](https://github.com/dashpay/platform/pull/2465) — 7 unresolved (7 human) · 456 days stale · ✋ changes requested · 🐢 targets master
  - Top thread: "Why do we use testnet seeds for mainnet?" — 456 days old
- [#2446 feat(js-dapi-client): add contested resources query methods](https://github.com/dashpay/platform/pull/2446) — 5 unresolved (5 CodeRabbit) · 474 days stale · ⚠ merge conflict · 📝 draft · 🐢 targets v2.0-dev, untouched 465 days
  - Top thread: "_:warning: Potential issue_" — 474 days old
- [#2727 fix(drive): fix verification data contract verification logic with keeps history](https://github.com/dashpay/platform/pull/2727) — 🔴 CI failing · 🐢 targets v2.1-dev

<a id="pshenmic-stale"></a>
#### Stale (4)
- [#2444 feat(sdk): add NFT actions in the JS Dash SDK](https://github.com/dashpay/platform/pull/2444) — 17 unresolved (5 CodeRabbit, 12 human) · 446 days stale · ⚠ merge conflict · ✋ changes requested · 🐢 targets v2.0-dev
  - Top thread: "_:warning: Potential issue_" — 446 days old
- [#2465 fix(sdk): replace seeds with ip address](https://github.com/dashpay/platform/pull/2465) — 7 unresolved (7 human) · 456 days stale · ✋ changes requested · 🐢 targets master
  - Top thread: "Why do we use testnet seeds for mainnet?" — 456 days old
- [#2446 feat(js-dapi-client): add contested resources query methods](https://github.com/dashpay/platform/pull/2446) — 5 unresolved (5 CodeRabbit) · 474 days stale · ⚠ merge conflict · 📝 draft · 🐢 targets v2.0-dev, untouched 465 days
  - Top thread: "_:warning: Potential issue_" — 474 days old
- [#2727 fix(drive): fix verification data contract verification logic with keeps history](https://github.com/dashpay/platform/pull/2727) — 🔴 CI failing · 🐢 targets v2.1-dev

<a id="inna333-cuber"></a>
### @Inna333-cuber
<a id="inna333-cuber-open"></a>
#### Open (1)
- [#2499 docs: replaced the icon with twitter](https://github.com/dashpay/platform/pull/2499) — 1 unresolved (1 human) · 49 days stale · 🐢 targets v2.0-dev
  - Top thread: "<!-- thepastaclaw-review v1 finding=1314ffa7c907 -->" — 49 days old

<a id="inna333-cuber-stale"></a>
#### Stale (1)
- [#2499 docs: replaced the icon with twitter](https://github.com/dashpay/platform/pull/2499) — 1 unresolved (1 human) · 49 days stale · 🐢 targets v2.0-dev
  - Top thread: "<!-- thepastaclaw-review v1 finding=1314ffa7c907 -->" — 49 days old

<a id="ogabrielides"></a>
### @ogabrielides
<a id="ogabrielides-open"></a>
#### Open (1)
- [#2486 feat(drive-abci): state sync - faster sync of new nodes](https://github.com/dashpay/platform/pull/2486) — ⏸ deferred

<a id="ogabrielides-deferred"></a>
#### Deferred (1)
- [#2486 feat(drive-abci): state sync - faster sync of new nodes](https://github.com/dashpay/platform/pull/2486) — ⏸ deferred

<a id="pauldelucia"></a>
### @pauldelucia
<a id="pauldelucia-open"></a>
#### Open (3)
- [#2974 feat(drive): add array element indexing support](https://github.com/dashpay/platform/pull/2974) — 🔴 CI failing · 📝 draft · 🐢 targets v3.0-dev, untouched 126 days
- [#2977 feat(dapi): return actual fees paid in state transition broadcast responses](https://github.com/dashpay/platform/pull/2977) — 🔴 CI failing · 📝 draft · 🐢 targets v3.0-dev, untouched 126 days
- [#2996 fix(drive-abci): version fix for decryption bounded key validation in contract bounds](https://github.com/dashpay/platform/pull/2996) — 🔴 CI failing · 🐢 targets v3.0-dev

<a id="pauldelucia-stale"></a>
#### Stale (3)
- [#2974 feat(drive): add array element indexing support](https://github.com/dashpay/platform/pull/2974) — 🔴 CI failing · 📝 draft · 🐢 targets v3.0-dev, untouched 126 days
- [#2977 feat(dapi): return actual fees paid in state transition broadcast responses](https://github.com/dashpay/platform/pull/2977) — 🔴 CI failing · 📝 draft · 🐢 targets v3.0-dev, untouched 126 days
- [#2996 fix(drive-abci): version fix for decryption bounded key validation in contract bounds](https://github.com/dashpay/platform/pull/2996) — 🔴 CI failing · 🐢 targets v3.0-dev

## Methodology
Generated nightly by [pr-hygiene](https://github.com/dashpay/stale_prs_are_bad). A thread counts as "unresolved" when it is open, not outdated, has a comment from someone other than the PR author, and the most recent comment is from a reviewer. **Dirty** = at least one such thread. **Unresolved Comments** = at least one such thread. **Changes Requested** = no unresolved threads but a reviewer's most recent review is CHANGES_REQUESTED (still blocking until someone re-approves or dismisses). **Deferred** = carries a configured deferred label (e.g. `postponed`) — visible but not counted toward unresolved-comment counts. **Stale** = targets a non-default branch OR hasn't been touched in the configured threshold (default 120 days, but clean PRs are never reclassified as stale). **Draft** = the PR is still marked draft on GitHub. **CI failing** = no unresolved comments, no changes-requested, but the latest commit's status check is failing. **Clean** = open, not draft, not deferred, not stale, no unresolved comments, no changes-requested, CI green. **Needs action** further requires changes-requested, merge conflict, or that the reviewer commented more recently than the author last pushed. **Ready for Review** counts clean PRs (authored by someone else) where this person owes a review. When a `review_routing` rule matches a PR's changed files, the routed reviewer IS the queue (explicit GitHub reviewers are ignored); a routed reviewer who has already submitted any review is excluded — their job is done. Configurable via [`https://github.com/dashpay/stale_prs_are_bad/blob/master/.pr-hygiene.yml`](https://github.com/dashpay/stale_prs_are_bad/blob/master/.pr-hygiene.yml)—edit defaults there.

_No history snapshot from last week was found, so week-over-week deltas are omitted this run._
