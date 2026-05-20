---
---
# PR Hygiene Report
*Last updated: 2026-05-20 07:17 UTC · commit 3718f82*

## Summary
- Open PRs: **67** (0 clean · 3 CI failing · 6 changes requested · 15 unresolved comments · 9 deferred · 20 draft · 14 stale)
- PRs needing author action: **20**
- Total unresolved comments: **156**

## Scoreboard
_Sort: unresolved-comments desc → needs-action desc → ready-for-review desc. Click any number to jump to the specific PRs it covers._

| Author | Open | Clean | CI failing | Unresolved Comments | Changes Requested | Deferred | Draft | Stale | Needs action | Total Unresolved Comments | Ready for Review |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| [@lklimek + (@Claudius-Maginificent)](#lklimek) | [13+(6)](#lklimek-open) | — | — | [3+(2)](#lklimek-unresolved-comments) | [3+(1)](#lklimek-changes-requested) | [5+(0)](#lklimek-deferred) | [2+(1)](#lklimek-draft) | [0+(2)](#lklimek-stale) | [6+(3)](#lklimek-needs-action) | [7+(31)](#lklimek-unresolved-comments) | — |
| [@PastaPastaPasta + (@thepastaclaw)](#pastapastapasta) | [8+(8)](#pastapastapasta-open) | — | [2+(0)](#pastapastapasta-ci-failing) | [2+(1)](#pastapastapasta-unresolved-comments) | — | — | [4+(6)](#pastapastapasta-draft) | [0+(1)](#pastapastapasta-stale) | [2+(0)](#pastapastapasta-needs-action) | [15+(7)](#pastapastapasta-unresolved-comments) | — |
| [@ZocoLini](#zocolini) | [5](#zocolini-open) | — | — | [2](#zocolini-unresolved-comments) | [1](#zocolini-changes-requested) | — | [2](#zocolini-draft) | — | [3](#zocolini-needs-action) | [8](#zocolini-unresolved-comments) | — |
| [@QuantumExplorer](#quantumexplorer) | [5](#quantumexplorer-open) | — | — | [2](#quantumexplorer-unresolved-comments) | — | [2](#quantumexplorer-deferred) | [1](#quantumexplorer-draft) | — | [2](#quantumexplorer-needs-action) | [38](#quantumexplorer-unresolved-comments) | — |
| [@shumkov](#shumkov) | [9](#shumkov-open) | — | [1](#shumkov-ci-failing) | [1](#shumkov-unresolved-comments) | — | [1](#shumkov-deferred) | [3](#shumkov-draft) | [3](#shumkov-stale) | [1](#shumkov-needs-action) | [11](#shumkov-unresolved-comments) | — |
| [@thephez](#thephez) | [1](#thephez-open) | — | — | [1](#thephez-unresolved-comments) | — | — | — | — | [1](#thephez-needs-action) | [2](#thephez-unresolved-comments) | — |
| [@ktechmidas](#ktechmidas) | [1](#ktechmidas-open) | — | — | [1](#ktechmidas-unresolved-comments) | — | — | — | — | [1](#ktechmidas-needs-action) | [6](#ktechmidas-unresolved-comments) | — |
| [@llbartekll](#llbartekll) | [2](#llbartekll-open) | — | — | — | [1](#llbartekll-changes-requested) | — | [1](#llbartekll-draft) | — | [1](#llbartekll-needs-action) | [1](#llbartekll-unresolved-comments) | — |
| [@pshenmic](#pshenmic) | [4](#pshenmic-open) | — | — | — | — | — | — | [4](#pshenmic-stale) | — | [29](#pshenmic-unresolved-comments) | — |
| [@Inna333-cuber](#inna333-cuber) | [1](#inna333-cuber-open) | — | — | — | — | — | — | [1](#inna333-cuber-stale) | — | [1](#inna333-cuber-unresolved-comments) | — |
| [@ogabrielides](#ogabrielides) | [1](#ogabrielides-open) | — | — | — | — | [1](#ogabrielides-deferred) | — | — | — | — | — |
| [@pauldelucia](#pauldelucia) | [3](#pauldelucia-open) | — | — | — | — | — | — | [3](#pauldelucia-stale) | — | — | — |

## Per-author detail

<a id="lklimek"></a>
### @lklimek + (@Claudius-Maginificent)
<a id="lklimek-open"></a>
#### Open (19)
- [#3549 test(platform-wallet): e2e framework + full test suite — triage pins, Found-*/PA-* guards, fail-closed persist, Stage-2 merge](https://github.com/dashpay/platform/pull/3549) — via @Claudius-Maginificent · 21 unresolved (21 human) · 17 days stale · 📝 draft · 🐢 targets fix/rs-platform-wallet-auto-select-inputs
  - Top thread: "<!-- thepastaclaw-review v1 finding=a3725e9d50ca -->" — 17 days old
- [#3625 feat(platform-wallet): add platform-wallet-storage crate (sqlite persister)](https://github.com/dashpay/platform/pull/3625) — via @Claudius-Maginificent · 4 unresolved (4 CodeRabbit) · 9 days stale · 📝 draft
  - Top thread: "_⚠️ Potential issue_ \| _🟡 Minor_ \| _⚡ Quick win_" — 9 days old
- [#3637 feat(platform-wallet): serde support](https://github.com/dashpay/platform/pull/3637) — via @Claudius-Maginificent · 5 unresolved (1 CodeRabbit, 4 human) · 7 days stale · ⚠ merge conflict
  - Top thread: "_⚠️ Potential issue_ \| _🔴 Critical_" — 6 days old
- [#3350 fix(rs-dapi,sdk): decode base64 CBOR error messages from Tenderdash](https://github.com/dashpay/platform/pull/3350) — 5 unresolved (5 human) · 1 days stale · ✋ changes requested
  - Top thread: "<!-- thepastaclaw-review v1 finding=29dc37ff2916 dedupe=7c69c1eb42d474df -->" — 1 days old
- [#3554 fix(platform-wallet): auto_select_inputs honors Σ inputs == Σ outputs](https://github.com/dashpay/platform/pull/3554) — via @Claudius-Maginificent · 1 unresolved (1 human) · 7 days stale · ⚠ merge conflict · ✋ changes requested
  - Top thread: "<!-- thepastaclaw-review v1 finding=5faa46ca0095 dedupe=97e40e670beeb9b9 -->" — 7 days old
- [#3650 fix(sdk): address-sync no longer silently discards balance changes for post-snapshot addresses (Found-025)](https://github.com/dashpay/platform/pull/3650) — 1 unresolved (1 CodeRabbit) · 1 days stale · 🔴 CI failing
  - Top thread: "_⚠️ Potential issue_ \| _🟠 Major_ \| _⚡ Quick win_" — 1 days old
- [#3636 feat(platform-wallet)!: add birth_height_override to wallet creation API](https://github.com/dashpay/platform/pull/3636) — 1 unresolved (1 human) · 1 days stale · ⚠ merge conflict
  - Top thread: "<!-- thepastaclaw-review v1 finding=62820ba0bba7 dedupe=c050d200e87b9f5f -->" — 1 days old
- [#2679 fix(drive-abci): don't panic on unsupported version error](https://github.com/dashpay/platform/pull/2679) — ⏸ deferred
- [#2795 feat(platform): add real-time platform event subscriptions via gRPC streaming](https://github.com/dashpay/platform/pull/2795) — ⏸ deferred
- [#2993 feat: identity reference validation](https://github.com/dashpay/platform/pull/2993) — 📝 draft · ⏸ deferred
- [#3009 feat: contract reference validation](https://github.com/dashpay/platform/pull/3009) — 📝 draft · ⏸ deferred
- [#3032 feat: document creation restriction by group membership](https://github.com/dashpay/platform/pull/3032) — 📝 draft · ⏸ deferred
- [#3585 fix: case-insensitive .dash + atomic state on broadcast failure](https://github.com/dashpay/platform/pull/3585) — via @Claudius-Maginificent · ⚠ merge conflict · ✋ changes requested
- [#3647 feat(platform-wallet): watermark monotonic-merge](https://github.com/dashpay/platform/pull/3647) — ✋ changes requested
- [#3648 fix(platform-wallet): local-ledger ownership guard (V27-007)](https://github.com/dashpay/platform/pull/3648) — ✋ changes requested
- [#3651 feat(platform-wallet): SPV cancel_background/identity_ids accessors + FFI no-selectable-inputs error mapping](https://github.com/dashpay/platform/pull/3651) — ✋ changes requested · 🔴 CI failing
- [#3658 fix(rs-platform-wallet): reserve platform receive address on hand-out (Found-026) [backport\]](https://github.com/dashpay/platform/pull/3658) — 🔴 CI failing · 📝 draft
- [#3659 fix(platform-wallet): fail-closed on registration persist error (Found-017) [backport\]](https://github.com/dashpay/platform/pull/3659) — 📝 draft
- [#3672 feat(platform-wallet): add SecretStore keyring + encrypted-file secret backends (secrets feature)](https://github.com/dashpay/platform/pull/3672) — via @Claudius-Maginificent · 📝 draft · 🐢 targets feat/platform-wallet-sqlite-persistor

<a id="lklimek-needs-action"></a>
#### Needs action (9)
- [#3637 feat(platform-wallet): serde support](https://github.com/dashpay/platform/pull/3637) — via @Claudius-Maginificent · 5 unresolved (1 CodeRabbit, 4 human) · 7 days stale · ⚠ merge conflict
  - Top thread: "_⚠️ Potential issue_ \| _🔴 Critical_" — 6 days old
- [#3350 fix(rs-dapi,sdk): decode base64 CBOR error messages from Tenderdash](https://github.com/dashpay/platform/pull/3350) — 5 unresolved (5 human) · 1 days stale · ✋ changes requested
  - Top thread: "<!-- thepastaclaw-review v1 finding=29dc37ff2916 dedupe=7c69c1eb42d474df -->" — 1 days old
- [#3554 fix(platform-wallet): auto_select_inputs honors Σ inputs == Σ outputs](https://github.com/dashpay/platform/pull/3554) — via @Claudius-Maginificent · 1 unresolved (1 human) · 7 days stale · ⚠ merge conflict · ✋ changes requested
  - Top thread: "<!-- thepastaclaw-review v1 finding=5faa46ca0095 dedupe=97e40e670beeb9b9 -->" — 7 days old
- [#3650 fix(sdk): address-sync no longer silently discards balance changes for post-snapshot addresses (Found-025)](https://github.com/dashpay/platform/pull/3650) — 1 unresolved (1 CodeRabbit) · 1 days stale · 🔴 CI failing
  - Top thread: "_⚠️ Potential issue_ \| _🟠 Major_ \| _⚡ Quick win_" — 1 days old
- [#3636 feat(platform-wallet)!: add birth_height_override to wallet creation API](https://github.com/dashpay/platform/pull/3636) — 1 unresolved (1 human) · 1 days stale · ⚠ merge conflict
  - Top thread: "<!-- thepastaclaw-review v1 finding=62820ba0bba7 dedupe=c050d200e87b9f5f -->" — 1 days old
- [#3585 fix: case-insensitive .dash + atomic state on broadcast failure](https://github.com/dashpay/platform/pull/3585) — via @Claudius-Maginificent · ⚠ merge conflict · ✋ changes requested
- [#3647 feat(platform-wallet): watermark monotonic-merge](https://github.com/dashpay/platform/pull/3647) — ✋ changes requested
- [#3648 fix(platform-wallet): local-ledger ownership guard (V27-007)](https://github.com/dashpay/platform/pull/3648) — ✋ changes requested
- [#3651 feat(platform-wallet): SPV cancel_background/identity_ids accessors + FFI no-selectable-inputs error mapping](https://github.com/dashpay/platform/pull/3651) — ✋ changes requested · 🔴 CI failing

<a id="lklimek-unresolved-comments"></a>
#### Unresolved Comments (5)
- [#3637 feat(platform-wallet): serde support](https://github.com/dashpay/platform/pull/3637) — via @Claudius-Maginificent · 5 unresolved (1 CodeRabbit, 4 human) · 7 days stale · ⚠ merge conflict
  - Top thread: "_⚠️ Potential issue_ \| _🔴 Critical_" — 6 days old
- [#3350 fix(rs-dapi,sdk): decode base64 CBOR error messages from Tenderdash](https://github.com/dashpay/platform/pull/3350) — 5 unresolved (5 human) · 1 days stale · ✋ changes requested
  - Top thread: "<!-- thepastaclaw-review v1 finding=29dc37ff2916 dedupe=7c69c1eb42d474df -->" — 1 days old
- [#3554 fix(platform-wallet): auto_select_inputs honors Σ inputs == Σ outputs](https://github.com/dashpay/platform/pull/3554) — via @Claudius-Maginificent · 1 unresolved (1 human) · 7 days stale · ⚠ merge conflict · ✋ changes requested
  - Top thread: "<!-- thepastaclaw-review v1 finding=5faa46ca0095 dedupe=97e40e670beeb9b9 -->" — 7 days old
- [#3650 fix(sdk): address-sync no longer silently discards balance changes for post-snapshot addresses (Found-025)](https://github.com/dashpay/platform/pull/3650) — 1 unresolved (1 CodeRabbit) · 1 days stale · 🔴 CI failing
  - Top thread: "_⚠️ Potential issue_ \| _🟠 Major_ \| _⚡ Quick win_" — 1 days old
- [#3636 feat(platform-wallet)!: add birth_height_override to wallet creation API](https://github.com/dashpay/platform/pull/3636) — 1 unresolved (1 human) · 1 days stale · ⚠ merge conflict
  - Top thread: "<!-- thepastaclaw-review v1 finding=62820ba0bba7 dedupe=c050d200e87b9f5f -->" — 1 days old

<a id="lklimek-changes-requested"></a>
#### Changes Requested (4)
- [#3585 fix: case-insensitive .dash + atomic state on broadcast failure](https://github.com/dashpay/platform/pull/3585) — via @Claudius-Maginificent · ⚠ merge conflict · ✋ changes requested
- [#3647 feat(platform-wallet): watermark monotonic-merge](https://github.com/dashpay/platform/pull/3647) — ✋ changes requested
- [#3648 fix(platform-wallet): local-ledger ownership guard (V27-007)](https://github.com/dashpay/platform/pull/3648) — ✋ changes requested
- [#3651 feat(platform-wallet): SPV cancel_background/identity_ids accessors + FFI no-selectable-inputs error mapping](https://github.com/dashpay/platform/pull/3651) — ✋ changes requested · 🔴 CI failing

<a id="lklimek-deferred"></a>
#### Deferred (5)
- [#2679 fix(drive-abci): don't panic on unsupported version error](https://github.com/dashpay/platform/pull/2679) — ⏸ deferred
- [#2795 feat(platform): add real-time platform event subscriptions via gRPC streaming](https://github.com/dashpay/platform/pull/2795) — ⏸ deferred
- [#2993 feat: identity reference validation](https://github.com/dashpay/platform/pull/2993) — 📝 draft · ⏸ deferred
- [#3009 feat: contract reference validation](https://github.com/dashpay/platform/pull/3009) — 📝 draft · ⏸ deferred
- [#3032 feat: document creation restriction by group membership](https://github.com/dashpay/platform/pull/3032) — 📝 draft · ⏸ deferred

<a id="lklimek-draft"></a>
#### Draft (3)
- [#3625 feat(platform-wallet): add platform-wallet-storage crate (sqlite persister)](https://github.com/dashpay/platform/pull/3625) — via @Claudius-Maginificent · 4 unresolved (4 CodeRabbit) · 9 days stale · 📝 draft
  - Top thread: "_⚠️ Potential issue_ \| _🟡 Minor_ \| _⚡ Quick win_" — 9 days old
- [#3658 fix(rs-platform-wallet): reserve platform receive address on hand-out (Found-026) [backport\]](https://github.com/dashpay/platform/pull/3658) — 🔴 CI failing · 📝 draft
- [#3659 fix(platform-wallet): fail-closed on registration persist error (Found-017) [backport\]](https://github.com/dashpay/platform/pull/3659) — 📝 draft

<a id="lklimek-stale"></a>
#### Stale (2)
- [#3549 test(platform-wallet): e2e framework + full test suite — triage pins, Found-*/PA-* guards, fail-closed persist, Stage-2 merge](https://github.com/dashpay/platform/pull/3549) — via @Claudius-Maginificent · 21 unresolved (21 human) · 17 days stale · 📝 draft · 🐢 targets fix/rs-platform-wallet-auto-select-inputs
  - Top thread: "<!-- thepastaclaw-review v1 finding=a3725e9d50ca -->" — 17 days old
- [#3672 feat(platform-wallet): add SecretStore keyring + encrypted-file secret backends (secrets feature)](https://github.com/dashpay/platform/pull/3672) — via @Claudius-Maginificent · 📝 draft · 🐢 targets feat/platform-wallet-sqlite-persistor

<a id="pastapastapasta"></a>
### @PastaPastaPasta + (@thepastaclaw)
<a id="pastapastapasta-open"></a>
#### Open (16)
- [#3509 fix(platform): default omitted proved query limits](https://github.com/dashpay/platform/pull/3509) — 14 unresolved (1 CodeRabbit, 13 human) · 29 days stale · ✋ changes requested · 🔴 CI failing
  - Top thread: "_⚠️ Potential issue_ \| _🟠 Major_" — 29 days old
- [#3096 feat(sdk): add client-side validation to state transition construction methods](https://github.com/dashpay/platform/pull/3096) — via @thepastaclaw · 4 unresolved (2 CodeRabbit, 2 human) · 91 days stale · ⚠ merge conflict · 📝 draft
  - Top thread: "_⚠️ Potential issue_ \| _🟡 Minor_" — 88 days old
- [#3165 fix(drive): consolidate historical contract proof verification retry logic](https://github.com/dashpay/platform/pull/3165) — via @thepastaclaw · 3 unresolved (3 human) · 75 days stale
  - Top thread: "PROJ-003** (INFO) — No coverage for `in_multiple_contract_proof_form=true`" — 75 days old
- [#2988 feat(wasm-sdk): auto-generate entropy for document creation when not provided](https://github.com/dashpay/platform/pull/2988) — 1 unresolved (1 human) · 1 days stale · ✋ changes requested
  - Top thread: "<!-- thepastaclaw-review v1 finding=a478cfcdfef4 dedupe=1719a6a0eb171ddd -->" — 1 days old
- [#3091 feat(wasm-sdk): add prepare_* APIs for idempotent document state transitions](https://github.com/dashpay/platform/pull/3091) — via @thepastaclaw · ✋ changes requested · 📝 draft
- [#3092 feat(rs-sdk): expose transition hash from state transition methods](https://github.com/dashpay/platform/pull/3092) — via @thepastaclaw · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · 📝 draft
- [#3133 fix(sdk): validate batch base structure before signing](https://github.com/dashpay/platform/pull/3133) — via @thepastaclaw · ✋ changes requested · 📝 draft
- [#3461 fix(dpp): block pre-programmed distribution changes on token update](https://github.com/dashpay/platform/pull/3461) — 📝 draft
- [#3462 fix(dpp)!: version-gate distribution function floating-point evaluation](https://github.com/dashpay/platform/pull/3462) — 📝 draft
- [#3596 fix(platform-wallet): satisfy accessors clippy lints](https://github.com/dashpay/platform/pull/3596) — via @thepastaclaw · 📝 draft
- [#3635 ci: tolerate book preview comment permission errors](https://github.com/dashpay/platform/pull/3635) — via @thepastaclaw · 📝 draft
- [#3657 fix(wasm-sdk): support binary grove path elements](https://github.com/dashpay/platform/pull/3657) — 🔴 CI failing
- [#3669 fix(swift-sdk): wait for SDK rebuild before wallet activation](https://github.com/dashpay/platform/pull/3669) — via @thepastaclaw · 📝 draft · 🐢 targets claude/swift-xcuitest-wallet-persistence
- [#3679 chore(dapi): cleanup — drop unused deps, inline winston/fetch/promisify shims](https://github.com/dashpay/platform/pull/3679) — 🔴 CI failing
- [#3680 refactor(dapi)!: expose Uint8Array instead of Buffer in public API of dapi-client](https://github.com/dashpay/platform/pull/3680) — ✋ changes requested · 📝 draft
- [#3689 refactor(sdk,wallet-lib,test-suite)!: convert dapi-client + wallet-lib + js-dash-sdk + platform-test-suite to ESM](https://github.com/dashpay/platform/pull/3689) — 📝 draft

<a id="pastapastapasta-needs-action"></a>
#### Needs action (2)
- [#3509 fix(platform): default omitted proved query limits](https://github.com/dashpay/platform/pull/3509) — 14 unresolved (1 CodeRabbit, 13 human) · 29 days stale · ✋ changes requested · 🔴 CI failing
  - Top thread: "_⚠️ Potential issue_ \| _🟠 Major_" — 29 days old
- [#2988 feat(wasm-sdk): auto-generate entropy for document creation when not provided](https://github.com/dashpay/platform/pull/2988) — 1 unresolved (1 human) · 1 days stale · ✋ changes requested
  - Top thread: "<!-- thepastaclaw-review v1 finding=a478cfcdfef4 dedupe=1719a6a0eb171ddd -->" — 1 days old

<a id="pastapastapasta-unresolved-comments"></a>
#### Unresolved Comments (3)
- [#3509 fix(platform): default omitted proved query limits](https://github.com/dashpay/platform/pull/3509) — 14 unresolved (1 CodeRabbit, 13 human) · 29 days stale · ✋ changes requested · 🔴 CI failing
  - Top thread: "_⚠️ Potential issue_ \| _🟠 Major_" — 29 days old
- [#3165 fix(drive): consolidate historical contract proof verification retry logic](https://github.com/dashpay/platform/pull/3165) — via @thepastaclaw · 3 unresolved (3 human) · 75 days stale
  - Top thread: "PROJ-003** (INFO) — No coverage for `in_multiple_contract_proof_form=true`" — 75 days old
- [#2988 feat(wasm-sdk): auto-generate entropy for document creation when not provided](https://github.com/dashpay/platform/pull/2988) — 1 unresolved (1 human) · 1 days stale · ✋ changes requested
  - Top thread: "<!-- thepastaclaw-review v1 finding=a478cfcdfef4 dedupe=1719a6a0eb171ddd -->" — 1 days old

<a id="pastapastapasta-ci-failing"></a>
#### CI Failing (2)
- [#3657 fix(wasm-sdk): support binary grove path elements](https://github.com/dashpay/platform/pull/3657) — 🔴 CI failing
- [#3679 chore(dapi): cleanup — drop unused deps, inline winston/fetch/promisify shims](https://github.com/dashpay/platform/pull/3679) — 🔴 CI failing

<a id="pastapastapasta-draft"></a>
#### Draft (10)
- [#3096 feat(sdk): add client-side validation to state transition construction methods](https://github.com/dashpay/platform/pull/3096) — via @thepastaclaw · 4 unresolved (2 CodeRabbit, 2 human) · 91 days stale · ⚠ merge conflict · 📝 draft
  - Top thread: "_⚠️ Potential issue_ \| _🟡 Minor_" — 88 days old
- [#3091 feat(wasm-sdk): add prepare_* APIs for idempotent document state transitions](https://github.com/dashpay/platform/pull/3091) — via @thepastaclaw · ✋ changes requested · 📝 draft
- [#3092 feat(rs-sdk): expose transition hash from state transition methods](https://github.com/dashpay/platform/pull/3092) — via @thepastaclaw · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · 📝 draft
- [#3133 fix(sdk): validate batch base structure before signing](https://github.com/dashpay/platform/pull/3133) — via @thepastaclaw · ✋ changes requested · 📝 draft
- [#3461 fix(dpp): block pre-programmed distribution changes on token update](https://github.com/dashpay/platform/pull/3461) — 📝 draft
- [#3462 fix(dpp)!: version-gate distribution function floating-point evaluation](https://github.com/dashpay/platform/pull/3462) — 📝 draft
- [#3596 fix(platform-wallet): satisfy accessors clippy lints](https://github.com/dashpay/platform/pull/3596) — via @thepastaclaw · 📝 draft
- [#3635 ci: tolerate book preview comment permission errors](https://github.com/dashpay/platform/pull/3635) — via @thepastaclaw · 📝 draft
- [#3680 refactor(dapi)!: expose Uint8Array instead of Buffer in public API of dapi-client](https://github.com/dashpay/platform/pull/3680) — ✋ changes requested · 📝 draft
- [#3689 refactor(sdk,wallet-lib,test-suite)!: convert dapi-client + wallet-lib + js-dash-sdk + platform-test-suite to ESM](https://github.com/dashpay/platform/pull/3689) — 📝 draft

<a id="pastapastapasta-stale"></a>
#### Stale (1)
- [#3669 fix(swift-sdk): wait for SDK rebuild before wallet activation](https://github.com/dashpay/platform/pull/3669) — via @thepastaclaw · 📝 draft · 🐢 targets claude/swift-xcuitest-wallet-persistence

<a id="zocolini"></a>
### @ZocoLini
<a id="zocolini-open"></a>
#### Open (5)
- [#3488 chore(swift-sdk): add wrappers for the missing TransactionRecord fields in the swift-sdk](https://github.com/dashpay/platform/pull/3488) — 5 unresolved (5 human) · 27 days stale · ✋ changes requested
  - Top thread: "<!-- thepastaclaw-review v1 finding=106ff7904b9f -->" — 27 days old
- [#3639 feat: external signable wallets](https://github.com/dashpay/platform/pull/3639) — 3 unresolved (3 human) · 0 days stale
  - Top thread: "<!-- thepastaclaw-review v1 finding=c032297273a0 -->" — 0 days old
- [#3463 chore(swift-sdk): remove unused code in the swift-sdk and example app](https://github.com/dashpay/platform/pull/3463) — ⚠ merge conflict · ✋ changes requested · 📝 draft
- [#3479 test(swift-sdk): swift-sdk test updated and added to CI](https://github.com/dashpay/platform/pull/3479) — ✋ changes requested · 🔴 CI failing
- [#3688 feat(swift-sdk): core to platform asset-lock funding from Send screen](https://github.com/dashpay/platform/pull/3688) — 📝 draft

<a id="zocolini-needs-action"></a>
#### Needs action (3)
- [#3488 chore(swift-sdk): add wrappers for the missing TransactionRecord fields in the swift-sdk](https://github.com/dashpay/platform/pull/3488) — 5 unresolved (5 human) · 27 days stale · ✋ changes requested
  - Top thread: "<!-- thepastaclaw-review v1 finding=106ff7904b9f -->" — 27 days old
- [#3639 feat: external signable wallets](https://github.com/dashpay/platform/pull/3639) — 3 unresolved (3 human) · 0 days stale
  - Top thread: "<!-- thepastaclaw-review v1 finding=c032297273a0 -->" — 0 days old
- [#3479 test(swift-sdk): swift-sdk test updated and added to CI](https://github.com/dashpay/platform/pull/3479) — ✋ changes requested · 🔴 CI failing

<a id="zocolini-unresolved-comments"></a>
#### Unresolved Comments (2)
- [#3488 chore(swift-sdk): add wrappers for the missing TransactionRecord fields in the swift-sdk](https://github.com/dashpay/platform/pull/3488) — 5 unresolved (5 human) · 27 days stale · ✋ changes requested
  - Top thread: "<!-- thepastaclaw-review v1 finding=106ff7904b9f -->" — 27 days old
- [#3639 feat: external signable wallets](https://github.com/dashpay/platform/pull/3639) — 3 unresolved (3 human) · 0 days stale
  - Top thread: "<!-- thepastaclaw-review v1 finding=c032297273a0 -->" — 0 days old

<a id="zocolini-changes-requested"></a>
#### Changes Requested (1)
- [#3479 test(swift-sdk): swift-sdk test updated and added to CI](https://github.com/dashpay/platform/pull/3479) — ✋ changes requested · 🔴 CI failing

<a id="zocolini-draft"></a>
#### Draft (2)
- [#3463 chore(swift-sdk): remove unused code in the swift-sdk and example app](https://github.com/dashpay/platform/pull/3463) — ⚠ merge conflict · ✋ changes requested · 📝 draft
- [#3688 feat(swift-sdk): core to platform asset-lock funding from Send screen](https://github.com/dashpay/platform/pull/3688) — 📝 draft

<a id="quantumexplorer"></a>
### @QuantumExplorer
<a id="quantumexplorer-open"></a>
#### Open (5)
- [#3603 feat(swift-sdk,platform-wallet): wire shielded send end-to-end (all 4 transitions)](https://github.com/dashpay/platform/pull/3603) — 27 unresolved (8 CodeRabbit, 19 human) · 14 days stale
  - Top thread: "_⚠️ Potential issue_ \| _🔴 Critical_ \| _⚡ Quick win_" — 14 days old
- [#3557 feat(platform-wallet): instrument wallet_manager RwLock behind lock-stats feature](https://github.com/dashpay/platform/pull/3557) — 11 unresolved (2 CodeRabbit, 9 human) · 22 days stale
  - Top thread: "_⚠️ Potential issue_ \| _🟠 Major_" — 22 days old
- [#1834 feat(drive-abci)!: Statesync](https://github.com/dashpay/platform/pull/1834) — 📝 draft · ⏸ deferred
- [#3021 feat(platform): better contract state transitions](https://github.com/dashpay/platform/pull/3021) — 📝 draft · ⏸ deferred
- [#3417 feat(swift-sdk): use SPV-synced quorums for Platform proof verification](https://github.com/dashpay/platform/pull/3417) — ⚠ merge conflict · 🔴 CI failing · 📝 draft

<a id="quantumexplorer-needs-action"></a>
#### Needs action (2)
- [#3603 feat(swift-sdk,platform-wallet): wire shielded send end-to-end (all 4 transitions)](https://github.com/dashpay/platform/pull/3603) — 27 unresolved (8 CodeRabbit, 19 human) · 14 days stale
  - Top thread: "_⚠️ Potential issue_ \| _🔴 Critical_ \| _⚡ Quick win_" — 14 days old
- [#3557 feat(platform-wallet): instrument wallet_manager RwLock behind lock-stats feature](https://github.com/dashpay/platform/pull/3557) — 11 unresolved (2 CodeRabbit, 9 human) · 22 days stale
  - Top thread: "_⚠️ Potential issue_ \| _🟠 Major_" — 22 days old

<a id="quantumexplorer-unresolved-comments"></a>
#### Unresolved Comments (2)
- [#3603 feat(swift-sdk,platform-wallet): wire shielded send end-to-end (all 4 transitions)](https://github.com/dashpay/platform/pull/3603) — 27 unresolved (8 CodeRabbit, 19 human) · 14 days stale
  - Top thread: "_⚠️ Potential issue_ \| _🔴 Critical_ \| _⚡ Quick win_" — 14 days old
- [#3557 feat(platform-wallet): instrument wallet_manager RwLock behind lock-stats feature](https://github.com/dashpay/platform/pull/3557) — 11 unresolved (2 CodeRabbit, 9 human) · 22 days stale
  - Top thread: "_⚠️ Potential issue_ \| _🟠 Major_" — 22 days old

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
#### Open (9)
- [#3166 chore: integrate Claude Code into devcontainer for autonomus development](https://github.com/dashpay/platform/pull/3166) — 8 unresolved (8 human) · 75 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing
  - Top thread: "as above." — 75 days old
- [#3671 feat: platform-address funding from asset-lock proofs](https://github.com/dashpay/platform/pull/3671) — 3 unresolved (3 CodeRabbit) · 1 days stale · 📝 draft
  - Top thread: "_⚠️ Potential issue_ \| _🟡 Minor_ \| _⚡ Quick win_" — 1 days old
- [#2392 feat(dashmate): single node local network [WIP\]](https://github.com/dashpay/platform/pull/2392) — 🔴 CI failing · 🐢 targets v2.0-dev
- [#2518 feat: token marketplace](https://github.com/dashpay/platform/pull/2518) — 📝 draft · ⏸ deferred
- [#2552 perf(dapi): fetch only specific unconfirmed transaction](https://github.com/dashpay/platform/pull/2552) — ⚠ merge conflict · 🔴 CI failing · 🐢 targets v2.1-dev
- [#2973 [temp\] Test non determinism](https://github.com/dashpay/platform/pull/2973) — ⚠ merge conflict · 🔴 CI failing · 📝 draft · 🐢 targets v3.0-dev, untouched 127 days
- [#3419 Feat/platform wallet](https://github.com/dashpay/platform/pull/3419) — ⚠ merge conflict · 🔴 CI failing · 📝 draft
- [#3573 feat(dpp): unify JSON/Value conversion traits + comprehensive round-trip tests](https://github.com/dashpay/platform/pull/3573) — 🔴 CI failing
- [#3670 fix(drive-abci): bill batch transformer drive reads (B7)](https://github.com/dashpay/platform/pull/3670) — 📝 draft

<a id="shumkov-needs-action"></a>
#### Needs action (1)
- [#3166 chore: integrate Claude Code into devcontainer for autonomus development](https://github.com/dashpay/platform/pull/3166) — 8 unresolved (8 human) · 75 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing
  - Top thread: "as above." — 75 days old

<a id="shumkov-unresolved-comments"></a>
#### Unresolved Comments (1)
- [#3166 chore: integrate Claude Code into devcontainer for autonomus development](https://github.com/dashpay/platform/pull/3166) — 8 unresolved (8 human) · 75 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing
  - Top thread: "as above." — 75 days old

<a id="shumkov-ci-failing"></a>
#### CI Failing (1)
- [#3573 feat(dpp): unify JSON/Value conversion traits + comprehensive round-trip tests](https://github.com/dashpay/platform/pull/3573) — 🔴 CI failing

<a id="shumkov-deferred"></a>
#### Deferred (1)
- [#2518 feat: token marketplace](https://github.com/dashpay/platform/pull/2518) — 📝 draft · ⏸ deferred

<a id="shumkov-draft"></a>
#### Draft (3)
- [#3671 feat: platform-address funding from asset-lock proofs](https://github.com/dashpay/platform/pull/3671) — 3 unresolved (3 CodeRabbit) · 1 days stale · 📝 draft
  - Top thread: "_⚠️ Potential issue_ \| _🟡 Minor_ \| _⚡ Quick win_" — 1 days old
- [#3419 Feat/platform wallet](https://github.com/dashpay/platform/pull/3419) — ⚠ merge conflict · 🔴 CI failing · 📝 draft
- [#3670 fix(drive-abci): bill batch transformer drive reads (B7)](https://github.com/dashpay/platform/pull/3670) — 📝 draft

<a id="shumkov-stale"></a>
#### Stale (3)
- [#2392 feat(dashmate): single node local network [WIP\]](https://github.com/dashpay/platform/pull/2392) — 🔴 CI failing · 🐢 targets v2.0-dev
- [#2552 perf(dapi): fetch only specific unconfirmed transaction](https://github.com/dashpay/platform/pull/2552) — ⚠ merge conflict · 🔴 CI failing · 🐢 targets v2.1-dev
- [#2973 [temp\] Test non determinism](https://github.com/dashpay/platform/pull/2973) — ⚠ merge conflict · 🔴 CI failing · 📝 draft · 🐢 targets v3.0-dev, untouched 127 days

<a id="thephez"></a>
### @thephez
<a id="thephez-open"></a>
#### Open (1)
- [#3471 fix(dpp): remove erroneous keywords field from document-meta schema and fix contract keywords docs](https://github.com/dashpay/platform/pull/3471) — 2 unresolved (2 human) · 27 days stale
  - Top thread: "<!-- thepastaclaw-review v1 finding=72d0df4348f6 -->" — 27 days old

<a id="thephez-needs-action"></a>
#### Needs action (1)
- [#3471 fix(dpp): remove erroneous keywords field from document-meta schema and fix contract keywords docs](https://github.com/dashpay/platform/pull/3471) — 2 unresolved (2 human) · 27 days stale
  - Top thread: "<!-- thepastaclaw-review v1 finding=72d0df4348f6 -->" — 27 days old

<a id="thephez-unresolved-comments"></a>
#### Unresolved Comments (1)
- [#3471 fix(dpp): remove erroneous keywords field from document-meta schema and fix contract keywords docs](https://github.com/dashpay/platform/pull/3471) — 2 unresolved (2 human) · 27 days stale
  - Top thread: "<!-- thepastaclaw-review v1 finding=72d0df4348f6 -->" — 27 days old

<a id="ktechmidas"></a>
### @ktechmidas
<a id="ktechmidas-open"></a>
#### Open (1)
- [#3162 fix(dashmate): prevent orphaned verification container blocking SSL renewal](https://github.com/dashpay/platform/pull/3162) — 6 unresolved (6 human) · 1 days stale
  - Top thread: "<!-- thepastaclaw-review v1 finding=d7ee272d2ab4 dedupe=d8f79f6f47c868dc -->" — 1 days old

<a id="ktechmidas-needs-action"></a>
#### Needs action (1)
- [#3162 fix(dashmate): prevent orphaned verification container blocking SSL renewal](https://github.com/dashpay/platform/pull/3162) — 6 unresolved (6 human) · 1 days stale
  - Top thread: "<!-- thepastaclaw-review v1 finding=d7ee272d2ab4 dedupe=d8f79f6f47c868dc -->" — 1 days old

<a id="ktechmidas-unresolved-comments"></a>
#### Unresolved Comments (1)
- [#3162 fix(dashmate): prevent orphaned verification container blocking SSL renewal](https://github.com/dashpay/platform/pull/3162) — 6 unresolved (6 human) · 1 days stale
  - Top thread: "<!-- thepastaclaw-review v1 finding=d7ee272d2ab4 dedupe=d8f79f6f47c868dc -->" — 1 days old

<a id="llbartekll"></a>
### @llbartekll
<a id="llbartekll-open"></a>
#### Open (2)
- [#3481 feat(swift-sdk): expose rich transaction fields in WalletTransaction](https://github.com/dashpay/platform/pull/3481) — 1 unresolved (1 CodeRabbit) · 37 days stale · ⚠ merge conflict · 🔴 CI failing · 📝 draft
  - Top thread: "_⚠️ Potential issue_ \| _🟠 Major_" — 37 days old
- [#3560 test(swift-sdk): add testnet identity-discovery UI test](https://github.com/dashpay/platform/pull/3560) — ✋ changes requested

<a id="llbartekll-needs-action"></a>
#### Needs action (1)
- [#3560 test(swift-sdk): add testnet identity-discovery UI test](https://github.com/dashpay/platform/pull/3560) — ✋ changes requested

<a id="llbartekll-changes-requested"></a>
#### Changes Requested (1)
- [#3560 test(swift-sdk): add testnet identity-discovery UI test](https://github.com/dashpay/platform/pull/3560) — ✋ changes requested

<a id="llbartekll-draft"></a>
#### Draft (1)
- [#3481 feat(swift-sdk): expose rich transaction fields in WalletTransaction](https://github.com/dashpay/platform/pull/3481) — 1 unresolved (1 CodeRabbit) · 37 days stale · ⚠ merge conflict · 🔴 CI failing · 📝 draft
  - Top thread: "_⚠️ Potential issue_ \| _🟠 Major_" — 37 days old

<a id="pshenmic"></a>
### @pshenmic
<a id="pshenmic-open"></a>
#### Open (4)
- [#2444 feat(sdk): add NFT actions in the JS Dash SDK](https://github.com/dashpay/platform/pull/2444) — 17 unresolved (5 CodeRabbit, 12 human) · 447 days stale · ⚠ merge conflict · ✋ changes requested · 🐢 targets v2.0-dev
  - Top thread: "_:warning: Potential issue_" — 447 days old
- [#2465 fix(sdk): replace seeds with ip address](https://github.com/dashpay/platform/pull/2465) — 7 unresolved (7 human) · 457 days stale · ✋ changes requested · 🐢 targets master
  - Top thread: "Why do we use testnet seeds for mainnet?" — 457 days old
- [#2446 feat(js-dapi-client): add contested resources query methods](https://github.com/dashpay/platform/pull/2446) — 5 unresolved (5 CodeRabbit) · 476 days stale · ⚠ merge conflict · 📝 draft · 🐢 targets v2.0-dev, untouched 466 days
  - Top thread: "_:warning: Potential issue_" — 476 days old
- [#2727 fix(drive): fix verification data contract verification logic with keeps history](https://github.com/dashpay/platform/pull/2727) — 🔴 CI failing · 🐢 targets v2.1-dev

<a id="pshenmic-stale"></a>
#### Stale (4)
- [#2444 feat(sdk): add NFT actions in the JS Dash SDK](https://github.com/dashpay/platform/pull/2444) — 17 unresolved (5 CodeRabbit, 12 human) · 447 days stale · ⚠ merge conflict · ✋ changes requested · 🐢 targets v2.0-dev
  - Top thread: "_:warning: Potential issue_" — 447 days old
- [#2465 fix(sdk): replace seeds with ip address](https://github.com/dashpay/platform/pull/2465) — 7 unresolved (7 human) · 457 days stale · ✋ changes requested · 🐢 targets master
  - Top thread: "Why do we use testnet seeds for mainnet?" — 457 days old
- [#2446 feat(js-dapi-client): add contested resources query methods](https://github.com/dashpay/platform/pull/2446) — 5 unresolved (5 CodeRabbit) · 476 days stale · ⚠ merge conflict · 📝 draft · 🐢 targets v2.0-dev, untouched 466 days
  - Top thread: "_:warning: Potential issue_" — 476 days old
- [#2727 fix(drive): fix verification data contract verification logic with keeps history](https://github.com/dashpay/platform/pull/2727) — 🔴 CI failing · 🐢 targets v2.1-dev

<a id="inna333-cuber"></a>
### @Inna333-cuber
<a id="inna333-cuber-open"></a>
#### Open (1)
- [#2499 docs: replaced the icon with twitter](https://github.com/dashpay/platform/pull/2499) — 1 unresolved (1 human) · 51 days stale · 🐢 targets v2.0-dev
  - Top thread: "<!-- thepastaclaw-review v1 finding=1314ffa7c907 -->" — 51 days old

<a id="inna333-cuber-stale"></a>
#### Stale (1)
- [#2499 docs: replaced the icon with twitter](https://github.com/dashpay/platform/pull/2499) — 1 unresolved (1 human) · 51 days stale · 🐢 targets v2.0-dev
  - Top thread: "<!-- thepastaclaw-review v1 finding=1314ffa7c907 -->" — 51 days old

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
- [#2974 feat(drive): add array element indexing support](https://github.com/dashpay/platform/pull/2974) — 🔴 CI failing · 📝 draft · 🐢 targets v3.0-dev, untouched 127 days
- [#2977 feat(dapi): return actual fees paid in state transition broadcast responses](https://github.com/dashpay/platform/pull/2977) — 🔴 CI failing · 📝 draft · 🐢 targets v3.0-dev, untouched 127 days
- [#2996 fix(drive-abci): version fix for decryption bounded key validation in contract bounds](https://github.com/dashpay/platform/pull/2996) — 🔴 CI failing · 🐢 targets v3.0-dev

<a id="pauldelucia-stale"></a>
#### Stale (3)
- [#2974 feat(drive): add array element indexing support](https://github.com/dashpay/platform/pull/2974) — 🔴 CI failing · 📝 draft · 🐢 targets v3.0-dev, untouched 127 days
- [#2977 feat(dapi): return actual fees paid in state transition broadcast responses](https://github.com/dashpay/platform/pull/2977) — 🔴 CI failing · 📝 draft · 🐢 targets v3.0-dev, untouched 127 days
- [#2996 fix(drive-abci): version fix for decryption bounded key validation in contract bounds](https://github.com/dashpay/platform/pull/2996) — 🔴 CI failing · 🐢 targets v3.0-dev

## Methodology
Generated nightly by [pr-hygiene](https://github.com/dashpay/stale_prs_are_bad). A thread counts as "unresolved" when it is open, not outdated, has a comment from someone other than the PR author, and the most recent comment is from a reviewer. **Dirty** = at least one such thread. **Unresolved Comments** = at least one such thread. **Changes Requested** = no unresolved threads but a reviewer's most recent review is CHANGES_REQUESTED (still blocking until someone re-approves or dismisses). **Deferred** = carries a configured deferred label (e.g. `postponed`) — visible but not counted toward unresolved-comment counts. **Stale** = targets a non-default branch OR hasn't been touched in the configured threshold (default 120 days, but clean PRs are never reclassified as stale). **Draft** = the PR is still marked draft on GitHub. **CI failing** = no unresolved comments, no changes-requested, but the latest commit's status check is failing. **Clean** = open, not draft, not deferred, not stale, no unresolved comments, no changes-requested, CI green. **Needs action** further requires changes-requested, merge conflict, or that the reviewer commented more recently than the author last pushed. **Ready for Review** counts clean PRs (authored by someone else) where this person owes a review. When a `review_routing` rule matches a PR's changed files, the routed reviewer IS the queue (explicit GitHub reviewers are ignored); a routed reviewer who has already submitted any review is excluded — their job is done. Configurable via [`https://github.com/dashpay/stale_prs_are_bad/blob/master/.pr-hygiene.yml`](https://github.com/dashpay/stale_prs_are_bad/blob/master/.pr-hygiene.yml)—edit defaults there.

_No history snapshot from last week was found, so week-over-week deltas are omitted this run._
