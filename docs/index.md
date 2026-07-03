---
---
# PR Hygiene Report
*Last updated: 2026-07-03 00:39 UTC · commit 8d8af3c*

## Summary
- Open PRs: **57** (12 clean · 2 CI failing · 3 changes requested · 13 unresolved comments · 10 deferred · 8 draft · 9 stale)
- PRs needing author action: **17**
- Total unresolved comments: **73**

## Scoreboard
_Sort: unresolved-comments desc → needs-action desc → ready-for-review desc. Click any number to jump to the specific PRs it covers._

| Author | Open | Clean | CI failing | Unresolved Comments | Changes Requested | Deferred | Draft | Stale | Needs action | Total Unresolved Comments | Ready for Review | Δ |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| [@PastaPastaPasta + (@thepastaclaw)](#pastapastapasta) | [5+(7)](#pastapastapasta-open) | [0+(2)](#pastapastapasta-clean) | — | [4+(1)](#pastapastapasta-unresolved-comments) | — | — | [1+(1)](#pastapastapasta-draft) | [0+(3)](#pastapastapasta-stale) | [4+(0)](#pastapastapasta-needs-action) | [10+(4)](#pastapastapasta-unresolved-comments) | — | ↓ 1 |
| [@shumkov](#shumkov) | [7](#shumkov-open) | [1](#shumkov-clean) | — | [2](#shumkov-unresolved-comments) | [1](#shumkov-changes-requested) | [2](#shumkov-deferred) | [1](#shumkov-draft) | — | [3](#shumkov-needs-action) | [8](#shumkov-unresolved-comments) | [3](#shumkov-ready-for-review) | ↑ 1 |
| [@lklimek + (@Claudius-Maginificent)](#lklimek) | [6+(8)](#lklimek-open) | [0+(2)](#lklimek-clean) | — | [1+(1)](#lklimek-unresolved-comments) | [0+(1)](#lklimek-changes-requested) | [5+(0)](#lklimek-deferred) | [0+(2)](#lklimek-draft) | [0+(2)](#lklimek-stale) | [1+(2)](#lklimek-needs-action) | [1+(26)](#lklimek-unresolved-comments) | [1+(0)](#lklimek-ready-for-review) | ↑ 1 |
| [@llbartekll](#llbartekll) | [3](#llbartekll-open) | — | — | [2](#llbartekll-unresolved-comments) | — | — | [1](#llbartekll-draft) | — | [2](#llbartekll-needs-action) | [3](#llbartekll-unresolved-comments) | [4](#llbartekll-ready-for-review) | ↓ 1 |
| [@QuantumExplorer](#quantumexplorer) | [7](#quantumexplorer-open) | — | — | [1](#quantumexplorer-unresolved-comments) | — | [2](#quantumexplorer-deferred) | [1](#quantumexplorer-draft) | [3](#quantumexplorer-stale) | [1](#quantumexplorer-needs-action) | [13](#quantumexplorer-unresolved-comments) | [6](#quantumexplorer-ready-for-review) | — |
| [@thephez](#thephez) | [3](#thephez-open) | — | [1](#thephez-ci-failing) | [1](#thephez-unresolved-comments) | — | — | [1](#thephez-draft) | — | [1](#thephez-needs-action) | [3](#thephez-unresolved-comments) | — | — |
| [@ZocoLini](#zocolini) | [8](#zocolini-open) | [6](#zocolini-clean) | [1](#zocolini-ci-failing) | — | [1](#zocolini-changes-requested) | — | — | — | [3](#zocolini-needs-action) | — | [1](#zocolini-ready-for-review) | ↑ 2 |
| [@pshenmic](#pshenmic) | [1](#pshenmic-open) | — | — | — | — | — | — | [1](#pshenmic-stale) | — | [5](#pshenmic-unresolved-comments) | — | — |
| [@densmirnov](#densmirnov) | [1](#densmirnov-open) | [1](#densmirnov-clean) | — | — | — | — | — | — | — | — | — | — |
| [@ogabrielides](#ogabrielides) | [1](#ogabrielides-open) | — | — | — | — | [1](#ogabrielides-deferred) | — | — | — | — | — | — |

## Per-author detail

<a id="pastapastapasta"></a>
### @PastaPastaPasta + (@thepastaclaw)
<a id="pastapastapasta-open"></a>
#### Open (12)
- [#3096 feat(sdk): add client-side validation to state transition construction methods](https://github.com/dashpay/platform/pull/3096) — via @thepastaclaw · 4 unresolved (2 CodeRabbit, 2 human) · 135 days stale · 🔴 CI failing
  - Top thread: "_⚠️ Potential issue_ \| _🟡 Minor_" — 132 days old
- [#3680 refactor(dapi,dpp)!: move dapi-client and Identifier off Buffer to Uint8Array](https://github.com/dashpay/platform/pull/3680) — 5 unresolved (5 human) · 37 days stale · 🔴 CI failing
  - Top thread: "<!-- thepastaclaw-review v1 finding=1c101f507542 -->" — 37 days old
- [#3462 fix(dpp)!: version-gate distribution function floating-point evaluation](https://github.com/dashpay/platform/pull/3462) — 3 unresolved (3 human) · 31 days stale · 🔴 CI failing
  - Top thread: "<!-- thepastaclaw-review v1 finding=7d40971bd5ff dedupe=2846c55b07c76472 -->" — 31 days old
- [#3509 fix(platform): default omitted proved query limits](https://github.com/dashpay/platform/pull/3509) — 1 unresolved (1 human) · 61 days stale · ✋ changes requested · 🔴 CI failing
  - Top thread: "<!-- thepastaclaw-review v1 finding=d3df3e197206 -->" — 61 days old
- [#2988 feat(wasm-sdk): auto-generate entropy for document creation when not provided](https://github.com/dashpay/platform/pull/2988) — 1 unresolved (1 human) · 42 days stale · 🔴 CI failing
  - Top thread: "<!-- thepastaclaw-review v1 finding=30cbefe1c9ef -->" — 42 days old
- [#3635 ci: tolerate book preview comment permission errors](https://github.com/dashpay/platform/pull/3635) — via @thepastaclaw
- [#3689 refactor(sdk,wallet-lib,test-suite)!: convert dapi-client + wallet-lib + js-dash-sdk + platform-test-suite to ESM](https://github.com/dashpay/platform/pull/3689) — ⚠ merge conflict · 📝 draft
- [#3898 fix(dashmate): re-sync stale 3.x Drive and rs-dapi images](https://github.com/dashpay/platform/pull/3898) — via @thepastaclaw · 🐢 targets v4.0-dev
- [#3930 fix(platform): reject keep-history document deletes cleanly](https://github.com/dashpay/platform/pull/3930) — via @thepastaclaw · 🔴 CI failing · 🐢 targets v4.0-dev
- [#3938 test(rs-sdk): expect network floor in mock sdk seed test](https://github.com/dashpay/platform/pull/3938) — via @thepastaclaw
- [#3965 fix(drive-abci): use testnet Core RPC port in env](https://github.com/dashpay/platform/pull/3965) — via @thepastaclaw · 🐢 targets v4.0-dev
- [#3967 feat(wasm-sdk): expose tiered token pricing in setPrice](https://github.com/dashpay/platform/pull/3967) — via @thepastaclaw · 📝 draft

<a id="pastapastapasta-needs-action"></a>
#### Needs action (4)
- [#3680 refactor(dapi,dpp)!: move dapi-client and Identifier off Buffer to Uint8Array](https://github.com/dashpay/platform/pull/3680) — 5 unresolved (5 human) · 37 days stale · 🔴 CI failing
  - Top thread: "<!-- thepastaclaw-review v1 finding=1c101f507542 -->" — 37 days old
- [#3462 fix(dpp)!: version-gate distribution function floating-point evaluation](https://github.com/dashpay/platform/pull/3462) — 3 unresolved (3 human) · 31 days stale · 🔴 CI failing
  - Top thread: "<!-- thepastaclaw-review v1 finding=7d40971bd5ff dedupe=2846c55b07c76472 -->" — 31 days old
- [#3509 fix(platform): default omitted proved query limits](https://github.com/dashpay/platform/pull/3509) — 1 unresolved (1 human) · 61 days stale · ✋ changes requested · 🔴 CI failing
  - Top thread: "<!-- thepastaclaw-review v1 finding=d3df3e197206 -->" — 61 days old
- [#2988 feat(wasm-sdk): auto-generate entropy for document creation when not provided](https://github.com/dashpay/platform/pull/2988) — 1 unresolved (1 human) · 42 days stale · 🔴 CI failing
  - Top thread: "<!-- thepastaclaw-review v1 finding=30cbefe1c9ef -->" — 42 days old

<a id="pastapastapasta-unresolved-comments"></a>
#### Unresolved Comments (5)
- [#3096 feat(sdk): add client-side validation to state transition construction methods](https://github.com/dashpay/platform/pull/3096) — via @thepastaclaw · 4 unresolved (2 CodeRabbit, 2 human) · 135 days stale · 🔴 CI failing
  - Top thread: "_⚠️ Potential issue_ \| _🟡 Minor_" — 132 days old
- [#3680 refactor(dapi,dpp)!: move dapi-client and Identifier off Buffer to Uint8Array](https://github.com/dashpay/platform/pull/3680) — 5 unresolved (5 human) · 37 days stale · 🔴 CI failing
  - Top thread: "<!-- thepastaclaw-review v1 finding=1c101f507542 -->" — 37 days old
- [#3462 fix(dpp)!: version-gate distribution function floating-point evaluation](https://github.com/dashpay/platform/pull/3462) — 3 unresolved (3 human) · 31 days stale · 🔴 CI failing
  - Top thread: "<!-- thepastaclaw-review v1 finding=7d40971bd5ff dedupe=2846c55b07c76472 -->" — 31 days old
- [#3509 fix(platform): default omitted proved query limits](https://github.com/dashpay/platform/pull/3509) — 1 unresolved (1 human) · 61 days stale · ✋ changes requested · 🔴 CI failing
  - Top thread: "<!-- thepastaclaw-review v1 finding=d3df3e197206 -->" — 61 days old
- [#2988 feat(wasm-sdk): auto-generate entropy for document creation when not provided](https://github.com/dashpay/platform/pull/2988) — 1 unresolved (1 human) · 42 days stale · 🔴 CI failing
  - Top thread: "<!-- thepastaclaw-review v1 finding=30cbefe1c9ef -->" — 42 days old

<a id="pastapastapasta-draft"></a>
#### Draft (2)
- [#3689 refactor(sdk,wallet-lib,test-suite)!: convert dapi-client + wallet-lib + js-dash-sdk + platform-test-suite to ESM](https://github.com/dashpay/platform/pull/3689) — ⚠ merge conflict · 📝 draft
- [#3967 feat(wasm-sdk): expose tiered token pricing in setPrice](https://github.com/dashpay/platform/pull/3967) — via @thepastaclaw · 📝 draft

<a id="pastapastapasta-stale"></a>
#### Stale (3)
- [#3898 fix(dashmate): re-sync stale 3.x Drive and rs-dapi images](https://github.com/dashpay/platform/pull/3898) — via @thepastaclaw · 🐢 targets v4.0-dev
- [#3930 fix(platform): reject keep-history document deletes cleanly](https://github.com/dashpay/platform/pull/3930) — via @thepastaclaw · 🔴 CI failing · 🐢 targets v4.0-dev
- [#3965 fix(drive-abci): use testnet Core RPC port in env](https://github.com/dashpay/platform/pull/3965) — via @thepastaclaw · 🐢 targets v4.0-dev

<a id="pastapastapasta-clean"></a>
#### Clean (2)
- [#3635 ci: tolerate book preview comment permission errors](https://github.com/dashpay/platform/pull/3635) — via @thepastaclaw
- [#3938 test(rs-sdk): expect network floor in mock sdk seed test](https://github.com/dashpay/platform/pull/3938) — via @thepastaclaw

<a id="shumkov"></a>
### @shumkov
<a id="shumkov-open"></a>
#### Open (7)
- [#3841 feat: complete dashpay in platform wallet and swift example app](https://github.com/dashpay/platform/pull/3841) — 5 unresolved (5 human) · 4 days stale · ⚠ merge conflict · ✋ changes requested
  - Top thread: "<!-- thepastaclaw-review v1 finding=f798194317bf dedupe=cfcead18f202a388 -->" — 4 days old
- [#3936 chore(drive-abci): update to nested address in SML](https://github.com/dashpay/platform/pull/3936) — 2 unresolved (2 human) · 14 days stale · ⚠ merge conflict · 📝 draft
  - Top thread: "<!-- thepastaclaw-review v1 finding=2e1950367681 dedupe=494e3310440bc19a -->" — 14 days old
- [#3974 docs(book): fix drifted source links in the count group-by chapters](https://github.com/dashpay/platform/pull/3974) — 1 unresolved (1 human) · 2 days stale · 🔴 CI failing
  - Top thread: "<!-- thepastaclaw-review v1 finding=84431ddef276 dedupe=7651eb50687937ba -->" — 2 days old
- [#2392 feat(dashmate): single node local network [WIP\]](https://github.com/dashpay/platform/pull/2392) — ⏸ deferred
- [#2518 feat: token marketplace](https://github.com/dashpay/platform/pull/2518) — 📝 draft · ⏸ deferred
- [#3573 feat(dpp): unify JSON/Value conversion traits + comprehensive round-trip tests](https://github.com/dashpay/platform/pull/3573) — ✋ changes requested · 🔴 CI failing
- [#3983 docs: point README badges + NIGHTLY_STATUS at v4.1-dev (new default)](https://github.com/dashpay/platform/pull/3983)

<a id="shumkov-needs-action"></a>
#### Needs action (3)
- [#3841 feat: complete dashpay in platform wallet and swift example app](https://github.com/dashpay/platform/pull/3841) — 5 unresolved (5 human) · 4 days stale · ⚠ merge conflict · ✋ changes requested
  - Top thread: "<!-- thepastaclaw-review v1 finding=f798194317bf dedupe=cfcead18f202a388 -->" — 4 days old
- [#3974 docs(book): fix drifted source links in the count group-by chapters](https://github.com/dashpay/platform/pull/3974) — 1 unresolved (1 human) · 2 days stale · 🔴 CI failing
  - Top thread: "<!-- thepastaclaw-review v1 finding=84431ddef276 dedupe=7651eb50687937ba -->" — 2 days old
- [#3573 feat(dpp): unify JSON/Value conversion traits + comprehensive round-trip tests](https://github.com/dashpay/platform/pull/3573) — ✋ changes requested · 🔴 CI failing

<a id="shumkov-unresolved-comments"></a>
#### Unresolved Comments (2)
- [#3841 feat: complete dashpay in platform wallet and swift example app](https://github.com/dashpay/platform/pull/3841) — 5 unresolved (5 human) · 4 days stale · ⚠ merge conflict · ✋ changes requested
  - Top thread: "<!-- thepastaclaw-review v1 finding=f798194317bf dedupe=cfcead18f202a388 -->" — 4 days old
- [#3974 docs(book): fix drifted source links in the count group-by chapters](https://github.com/dashpay/platform/pull/3974) — 1 unresolved (1 human) · 2 days stale · 🔴 CI failing
  - Top thread: "<!-- thepastaclaw-review v1 finding=84431ddef276 dedupe=7651eb50687937ba -->" — 2 days old

<a id="shumkov-changes-requested"></a>
#### Changes Requested (1)
- [#3573 feat(dpp): unify JSON/Value conversion traits + comprehensive round-trip tests](https://github.com/dashpay/platform/pull/3573) — ✋ changes requested · 🔴 CI failing

<a id="shumkov-deferred"></a>
#### Deferred (2)
- [#2392 feat(dashmate): single node local network [WIP\]](https://github.com/dashpay/platform/pull/2392) — ⏸ deferred
- [#2518 feat: token marketplace](https://github.com/dashpay/platform/pull/2518) — 📝 draft · ⏸ deferred

<a id="shumkov-draft"></a>
#### Draft (1)
- [#3936 chore(drive-abci): update to nested address in SML](https://github.com/dashpay/platform/pull/3936) — 2 unresolved (2 human) · 14 days stale · ⚠ merge conflict · 📝 draft
  - Top thread: "<!-- thepastaclaw-review v1 finding=2e1950367681 dedupe=494e3310440bc19a -->" — 14 days old

<a id="shumkov-clean"></a>
#### Clean (1)
- [#3983 docs: point README badges + NIGHTLY_STATUS at v4.1-dev (new default)](https://github.com/dashpay/platform/pull/3983)

<a id="shumkov-ready-for-review"></a>
#### Ready for Review (3)
- [#3938 test(rs-sdk): expect network floor in mock sdk seed test](https://github.com/dashpay/platform/pull/3938) — by @thepastaclaw
- [#3954 feat(platform-wallet)!: shared ThreadRegistry for coordinator lifecycle + shutdown UAF/data-loss fixes](https://github.com/dashpay/platform/pull/3954) — by @Claudius-Maginificent
- [#3976 chore: bump rust-dashcore to afcff156, export xpub via ExtendedPubKeySigner](https://github.com/dashpay/platform/pull/3976) — by @Claudius-Maginificent

<a id="lklimek"></a>
### @lklimek + (@Claudius-Maginificent)
<a id="lklimek-open"></a>
#### Open (14)
- [#3549 test(platform-wallet): e2e framework + full test suite — triage pins, Found-*/PA-* guards, fail-closed persist, Stage-2 merge](https://github.com/dashpay/platform/pull/3549) — via @Claudius-Maginificent · 20 unresolved (20 human) · 61 days stale · ⚠ merge conflict · 📝 draft
  - Top thread: "<!-- thepastaclaw-review v1 finding=a3725e9d50ca -->" — 61 days old
- [#3985 fix(platform-wallet): release UTXO reservation when broadcast fails](https://github.com/dashpay/platform/pull/3985) — via @Claudius-Maginificent · 4 unresolved (4 human) · 0 days stale · 🐢 targets chore/bump-rust-dashcore-dev
  - Top thread: "<!-- thepastaclaw-review v1 finding=1c5641bda540 dedupe=0a6ea271f87143d2 -->" — 0 days old
- [#3650 fix(sdk): address-sync no longer silently discards balance changes for post-snapshot addresses (Found-025)](https://github.com/dashpay/platform/pull/3650) — 1 unresolved (1 human) · 1 days stale · ✋ changes requested · 🔴 CI failing
  - Top thread: "<!-- thepastaclaw-review v1 finding=510fab5574e6 dedupe=e2ae3a72a7df44ad -->" — 1 days old
- [#3692 feat(platform-wallet): watch-only rehydration from persistor (seedless load)](https://github.com/dashpay/platform/pull/3692) — via @Claudius-Maginificent · 2 unresolved (1 CodeRabbit, 1 human) · 0 days stale · 🔴 CI failing
  - Top thread: "_🎯 Functional Correctness_ \| _🟡 Minor_ \| _⚡ Quick win_" — 0 days old
- [#2679 fix(drive-abci): don't panic on unsupported version error](https://github.com/dashpay/platform/pull/2679) — ⏸ deferred
- [#2795 feat(platform): add real-time platform event subscriptions via gRPC streaming](https://github.com/dashpay/platform/pull/2795) — ⏸ deferred
- [#2993 feat: identity reference validation](https://github.com/dashpay/platform/pull/2993) — 📝 draft · ⏸ deferred
- [#3009 feat: contract reference validation](https://github.com/dashpay/platform/pull/3009) — 📝 draft · ⏸ deferred
- [#3032 feat: document creation restriction by group membership](https://github.com/dashpay/platform/pull/3032) — 📝 draft · ⏸ deferred
- [#3750 feat(platform-wallet)!: [NO MERGE\] consumer hardening — CODE-001/003-callsite/017/018 + PROJ-001 FFI + CODE-008/012/013](https://github.com/dashpay/platform/pull/3750) — via @Claudius-Maginificent · ⚠ merge conflict · 📝 draft
- [#3954 feat(platform-wallet)!: shared ThreadRegistry for coordinator lifecycle + shutdown UAF/data-loss fixes](https://github.com/dashpay/platform/pull/3954) — via @Claudius-Maginificent
- [#3968 feat(platform-wallet): persistence readers + seedless load() wiring (split from #3692)](https://github.com/dashpay/platform/pull/3968) — via @Claudius-Maginificent · ✋ changes requested
- [#3976 chore: bump rust-dashcore to afcff156, export xpub via ExtendedPubKeySigner](https://github.com/dashpay/platform/pull/3976) — via @Claudius-Maginificent
- [#3986 feat(platform-wallet-storage): snapshot readers + unified V002 schema migration](https://github.com/dashpay/platform/pull/3986) — via @Claudius-Maginificent · 🔴 CI failing · 📝 draft · 🐢 targets feat/platform-wallet-storage-rehydration

<a id="lklimek-needs-action"></a>
#### Needs action (3)
- [#3650 fix(sdk): address-sync no longer silently discards balance changes for post-snapshot addresses (Found-025)](https://github.com/dashpay/platform/pull/3650) — 1 unresolved (1 human) · 1 days stale · ✋ changes requested · 🔴 CI failing
  - Top thread: "<!-- thepastaclaw-review v1 finding=510fab5574e6 dedupe=e2ae3a72a7df44ad -->" — 1 days old
- [#3692 feat(platform-wallet): watch-only rehydration from persistor (seedless load)](https://github.com/dashpay/platform/pull/3692) — via @Claudius-Maginificent · 2 unresolved (1 CodeRabbit, 1 human) · 0 days stale · 🔴 CI failing
  - Top thread: "_🎯 Functional Correctness_ \| _🟡 Minor_ \| _⚡ Quick win_" — 0 days old
- [#3968 feat(platform-wallet): persistence readers + seedless load() wiring (split from #3692)](https://github.com/dashpay/platform/pull/3968) — via @Claudius-Maginificent · ✋ changes requested

<a id="lklimek-unresolved-comments"></a>
#### Unresolved Comments (2)
- [#3650 fix(sdk): address-sync no longer silently discards balance changes for post-snapshot addresses (Found-025)](https://github.com/dashpay/platform/pull/3650) — 1 unresolved (1 human) · 1 days stale · ✋ changes requested · 🔴 CI failing
  - Top thread: "<!-- thepastaclaw-review v1 finding=510fab5574e6 dedupe=e2ae3a72a7df44ad -->" — 1 days old
- [#3692 feat(platform-wallet): watch-only rehydration from persistor (seedless load)](https://github.com/dashpay/platform/pull/3692) — via @Claudius-Maginificent · 2 unresolved (1 CodeRabbit, 1 human) · 0 days stale · 🔴 CI failing
  - Top thread: "_🎯 Functional Correctness_ \| _🟡 Minor_ \| _⚡ Quick win_" — 0 days old

<a id="lklimek-changes-requested"></a>
#### Changes Requested (1)
- [#3968 feat(platform-wallet): persistence readers + seedless load() wiring (split from #3692)](https://github.com/dashpay/platform/pull/3968) — via @Claudius-Maginificent · ✋ changes requested

<a id="lklimek-deferred"></a>
#### Deferred (5)
- [#2679 fix(drive-abci): don't panic on unsupported version error](https://github.com/dashpay/platform/pull/2679) — ⏸ deferred
- [#2795 feat(platform): add real-time platform event subscriptions via gRPC streaming](https://github.com/dashpay/platform/pull/2795) — ⏸ deferred
- [#2993 feat: identity reference validation](https://github.com/dashpay/platform/pull/2993) — 📝 draft · ⏸ deferred
- [#3009 feat: contract reference validation](https://github.com/dashpay/platform/pull/3009) — 📝 draft · ⏸ deferred
- [#3032 feat: document creation restriction by group membership](https://github.com/dashpay/platform/pull/3032) — 📝 draft · ⏸ deferred

<a id="lklimek-draft"></a>
#### Draft (2)
- [#3549 test(platform-wallet): e2e framework + full test suite — triage pins, Found-*/PA-* guards, fail-closed persist, Stage-2 merge](https://github.com/dashpay/platform/pull/3549) — via @Claudius-Maginificent · 20 unresolved (20 human) · 61 days stale · ⚠ merge conflict · 📝 draft
  - Top thread: "<!-- thepastaclaw-review v1 finding=a3725e9d50ca -->" — 61 days old
- [#3750 feat(platform-wallet)!: [NO MERGE\] consumer hardening — CODE-001/003-callsite/017/018 + PROJ-001 FFI + CODE-008/012/013](https://github.com/dashpay/platform/pull/3750) — via @Claudius-Maginificent · ⚠ merge conflict · 📝 draft

<a id="lklimek-stale"></a>
#### Stale (2)
- [#3985 fix(platform-wallet): release UTXO reservation when broadcast fails](https://github.com/dashpay/platform/pull/3985) — via @Claudius-Maginificent · 4 unresolved (4 human) · 0 days stale · 🐢 targets chore/bump-rust-dashcore-dev
  - Top thread: "<!-- thepastaclaw-review v1 finding=1c5641bda540 dedupe=0a6ea271f87143d2 -->" — 0 days old
- [#3986 feat(platform-wallet-storage): snapshot readers + unified V002 schema migration](https://github.com/dashpay/platform/pull/3986) — via @Claudius-Maginificent · 🔴 CI failing · 📝 draft · 🐢 targets feat/platform-wallet-storage-rehydration

<a id="lklimek-clean"></a>
#### Clean (2)
- [#3954 feat(platform-wallet)!: shared ThreadRegistry for coordinator lifecycle + shutdown UAF/data-loss fixes](https://github.com/dashpay/platform/pull/3954) — via @Claudius-Maginificent
- [#3976 chore: bump rust-dashcore to afcff156, export xpub via ExtendedPubKeySigner](https://github.com/dashpay/platform/pull/3976) — via @Claudius-Maginificent

<a id="lklimek-ready-for-review"></a>
#### Ready for Review (1)
- [#3938 test(rs-sdk): expect network floor in mock sdk seed test](https://github.com/dashpay/platform/pull/3938) — by @thepastaclaw

<a id="llbartekll"></a>
### @llbartekll
<a id="llbartekll-open"></a>
#### Open (3)
- [#3560 test(swift-sdk): add testnet identity-discovery UI test](https://github.com/dashpay/platform/pull/3560) — 2 unresolved (2 human) · 31 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing
  - Top thread: "<!-- thepastaclaw-review v1 finding=379c84ed00e1 dedupe=08a70e07f6e9ae45 -->" — 31 days old
- [#3694 fix(swift-example-app): unhide Create Identity submit button and auto-dismiss sheet on success](https://github.com/dashpay/platform/pull/3694) — 1 unresolved (1 human) · 31 days stale · ✋ changes requested
  - Top thread: "<!-- thepastaclaw-review v1 finding=e3a849e745d0 dedupe=d76d5740c5bc0f94 -->" — 31 days old
- [#3981 feat(platform-wallet-ffi): pure transaction-decode FFI + Swift wrapper](https://github.com/dashpay/platform/pull/3981) — 🔴 CI failing · 📝 draft

<a id="llbartekll-needs-action"></a>
#### Needs action (2)
- [#3560 test(swift-sdk): add testnet identity-discovery UI test](https://github.com/dashpay/platform/pull/3560) — 2 unresolved (2 human) · 31 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing
  - Top thread: "<!-- thepastaclaw-review v1 finding=379c84ed00e1 dedupe=08a70e07f6e9ae45 -->" — 31 days old
- [#3694 fix(swift-example-app): unhide Create Identity submit button and auto-dismiss sheet on success](https://github.com/dashpay/platform/pull/3694) — 1 unresolved (1 human) · 31 days stale · ✋ changes requested
  - Top thread: "<!-- thepastaclaw-review v1 finding=e3a849e745d0 dedupe=d76d5740c5bc0f94 -->" — 31 days old

<a id="llbartekll-unresolved-comments"></a>
#### Unresolved Comments (2)
- [#3560 test(swift-sdk): add testnet identity-discovery UI test](https://github.com/dashpay/platform/pull/3560) — 2 unresolved (2 human) · 31 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing
  - Top thread: "<!-- thepastaclaw-review v1 finding=379c84ed00e1 dedupe=08a70e07f6e9ae45 -->" — 31 days old
- [#3694 fix(swift-example-app): unhide Create Identity submit button and auto-dismiss sheet on success](https://github.com/dashpay/platform/pull/3694) — 1 unresolved (1 human) · 31 days stale · ✋ changes requested
  - Top thread: "<!-- thepastaclaw-review v1 finding=e3a849e745d0 dedupe=d76d5740c5bc0f94 -->" — 31 days old

<a id="llbartekll-draft"></a>
#### Draft (1)
- [#3981 feat(platform-wallet-ffi): pure transaction-decode FFI + Swift wrapper](https://github.com/dashpay/platform/pull/3981) — 🔴 CI failing · 📝 draft

<a id="llbartekll-ready-for-review"></a>
#### Ready for Review (4)
- [#3712 test(swift-sdk): first swift sdk integration tests with local network](https://github.com/dashpay/platform/pull/3712) — by @ZocoLini
- [#3853 fix(swift-sdk): drop legacy headers pre-processing in build_ios.sh](https://github.com/dashpay/platform/pull/3853) — by @ZocoLini
- [#3950 chore(swift-sdk): script to get spv stortage from iOS sim](https://github.com/dashpay/platform/pull/3950) — by @ZocoLini
- [#3954 feat(platform-wallet)!: shared ThreadRegistry for coordinator lifecycle + shutdown UAF/data-loss fixes](https://github.com/dashpay/platform/pull/3954) — by @Claudius-Maginificent

<a id="quantumexplorer"></a>
### @QuantumExplorer
<a id="quantumexplorer-open"></a>
#### Open (7)
- [#3740 feat: add time-range indexes for trending/leaderboard queries](https://github.com/dashpay/platform/pull/3740) — 5 unresolved (5 CodeRabbit) · 38 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing
  - Top thread: "_⚠️ Potential issue_ \| _🟠 Major_ \| _⚡ Quick win_" — 38 days old
- [#3988 feat(platform-wallet): actively re-drive unconfirmed shielded spends](https://github.com/dashpay/platform/pull/3988) — 7 unresolved (7 human) · 0 days stale · ✋ changes requested · 🐢 targets v4.0-dev
  - Top thread: "<!-- thepastaclaw-review v1 finding=158700e57418 dedupe=49ca05afc9ec7481 -->" — 0 days old
- [#3792 fix(drive): authenticate boundary in compacted absence proofs](https://github.com/dashpay/platform/pull/3792) — 1 unresolved (1 human) · 28 days stale · 📝 draft · 🐢 targets v4.0-dev
  - Top thread: "<!-- thepastaclaw-review v1 finding=be8cd7f6fd00 dedupe=bb18c40415f6c4c4 -->" — 28 days old
- [#1834 feat(drive-abci)!: Statesync](https://github.com/dashpay/platform/pull/1834) — 📝 draft · ⏸ deferred
- [#3021 feat(platform): better contract state transitions](https://github.com/dashpay/platform/pull/3021) — 📝 draft · ⏸ deferred
- [#3417 feat(swift-sdk): use SPV-synced quorums for Platform proof verification](https://github.com/dashpay/platform/pull/3417) — ⚠ merge conflict · 🔴 CI failing · 📝 draft
- [#3923 feat(swift-example-app): wallet-signed Transfer & Withdraw for platform addresses (ADDR-02/04)](https://github.com/dashpay/platform/pull/3923) — 🔴 CI failing · 🐢 targets v4.0-dev

<a id="quantumexplorer-needs-action"></a>
#### Needs action (1)
- [#3740 feat: add time-range indexes for trending/leaderboard queries](https://github.com/dashpay/platform/pull/3740) — 5 unresolved (5 CodeRabbit) · 38 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing
  - Top thread: "_⚠️ Potential issue_ \| _🟠 Major_ \| _⚡ Quick win_" — 38 days old

<a id="quantumexplorer-unresolved-comments"></a>
#### Unresolved Comments (1)
- [#3740 feat: add time-range indexes for trending/leaderboard queries](https://github.com/dashpay/platform/pull/3740) — 5 unresolved (5 CodeRabbit) · 38 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing
  - Top thread: "_⚠️ Potential issue_ \| _🟠 Major_ \| _⚡ Quick win_" — 38 days old

<a id="quantumexplorer-deferred"></a>
#### Deferred (2)
- [#1834 feat(drive-abci)!: Statesync](https://github.com/dashpay/platform/pull/1834) — 📝 draft · ⏸ deferred
- [#3021 feat(platform): better contract state transitions](https://github.com/dashpay/platform/pull/3021) — 📝 draft · ⏸ deferred

<a id="quantumexplorer-draft"></a>
#### Draft (1)
- [#3417 feat(swift-sdk): use SPV-synced quorums for Platform proof verification](https://github.com/dashpay/platform/pull/3417) — ⚠ merge conflict · 🔴 CI failing · 📝 draft

<a id="quantumexplorer-stale"></a>
#### Stale (3)
- [#3988 feat(platform-wallet): actively re-drive unconfirmed shielded spends](https://github.com/dashpay/platform/pull/3988) — 7 unresolved (7 human) · 0 days stale · ✋ changes requested · 🐢 targets v4.0-dev
  - Top thread: "<!-- thepastaclaw-review v1 finding=158700e57418 dedupe=49ca05afc9ec7481 -->" — 0 days old
- [#3792 fix(drive): authenticate boundary in compacted absence proofs](https://github.com/dashpay/platform/pull/3792) — 1 unresolved (1 human) · 28 days stale · 📝 draft · 🐢 targets v4.0-dev
  - Top thread: "<!-- thepastaclaw-review v1 finding=be8cd7f6fd00 dedupe=bb18c40415f6c4c4 -->" — 28 days old
- [#3923 feat(swift-example-app): wallet-signed Transfer & Withdraw for platform addresses (ADDR-02/04)](https://github.com/dashpay/platform/pull/3923) — 🔴 CI failing · 🐢 targets v4.0-dev

<a id="quantumexplorer-ready-for-review"></a>
#### Ready for Review (6)
- [#3635 ci: tolerate book preview comment permission errors](https://github.com/dashpay/platform/pull/3635) — by @thepastaclaw
- [#3787 docs: add DashPay contact request encryption guide](https://github.com/dashpay/platform/pull/3787) — by @densmirnov
- [#3938 test(rs-sdk): expect network floor in mock sdk seed test](https://github.com/dashpay/platform/pull/3938) — by @thepastaclaw
- [#3954 feat(platform-wallet)!: shared ThreadRegistry for coordinator lifecycle + shutdown UAF/data-loss fixes](https://github.com/dashpay/platform/pull/3954) — by @Claudius-Maginificent
- [#3976 chore: bump rust-dashcore to afcff156, export xpub via ExtendedPubKeySigner](https://github.com/dashpay/platform/pull/3976) — by @Claudius-Maginificent
- [#3983 docs: point README badges + NIGHTLY_STATUS at v4.1-dev (new default)](https://github.com/dashpay/platform/pull/3983) — by @shumkov

<a id="thephez"></a>
### @thephez
<a id="thephez-open"></a>
#### Open (3)
- [#3961 test(drive): cover shared-prefix aggregate index insertion](https://github.com/dashpay/platform/pull/3961) — 3 unresolved (3 human) · 7 days stale
  - Top thread: "<!-- thepastaclaw-review v1 finding=651d0f445831 dedupe=e060ec72747ed6c2 -->" — 7 days old
- [#3849 test(drive-abci): add token supply edge-case coverage](https://github.com/dashpay/platform/pull/3849) — 🔴 CI failing
- [#3928 test(dpp,drive-abci): pin keepHistory + canBeDeleted contradiction](https://github.com/dashpay/platform/pull/3928) — 📝 draft

<a id="thephez-needs-action"></a>
#### Needs action (1)
- [#3961 test(drive): cover shared-prefix aggregate index insertion](https://github.com/dashpay/platform/pull/3961) — 3 unresolved (3 human) · 7 days stale
  - Top thread: "<!-- thepastaclaw-review v1 finding=651d0f445831 dedupe=e060ec72747ed6c2 -->" — 7 days old

<a id="thephez-unresolved-comments"></a>
#### Unresolved Comments (1)
- [#3961 test(drive): cover shared-prefix aggregate index insertion](https://github.com/dashpay/platform/pull/3961) — 3 unresolved (3 human) · 7 days stale
  - Top thread: "<!-- thepastaclaw-review v1 finding=651d0f445831 dedupe=e060ec72747ed6c2 -->" — 7 days old

<a id="thephez-ci-failing"></a>
#### CI Failing (1)
- [#3849 test(drive-abci): add token supply edge-case coverage](https://github.com/dashpay/platform/pull/3849) — 🔴 CI failing

<a id="thephez-draft"></a>
#### Draft (1)
- [#3928 test(dpp,drive-abci): pin keepHistory + canBeDeleted contradiction](https://github.com/dashpay/platform/pull/3928) — 📝 draft

<a id="zocolini"></a>
### @ZocoLini
<a id="zocolini-open"></a>
#### Open (8)
- [#3712 test(swift-sdk): first swift sdk integration tests with local network](https://github.com/dashpay/platform/pull/3712)
- [#3853 fix(swift-sdk): drop legacy headers pre-processing in build_ios.sh](https://github.com/dashpay/platform/pull/3853)
- [#3869 chore(swift-sdk): reduce swift-sdk test time in CI](https://github.com/dashpay/platform/pull/3869) — ⚠ merge conflict
- [#3948 chore: remove ignored cargo build config](https://github.com/dashpay/platform/pull/3948)
- [#3949 chore(swift-sdk): correctly refer to the swift sdk as swift in the build script and cargo profiles](https://github.com/dashpay/platform/pull/3949) — ✋ changes requested · 🔴 CI failing
- [#3950 chore(swift-sdk): script to get spv stortage from iOS sim](https://github.com/dashpay/platform/pull/3950)
- [#3955 chore: bump rust-dashcore rev to 95a3c8ff04d75d7abe324db4af01e8521092aff9](https://github.com/dashpay/platform/pull/3955) — ⚠ merge conflict
- [#3970 refactor(platform-wallet): expose the new core TransactionBuilder API + rust-dashcore bump](https://github.com/dashpay/platform/pull/3970) — 🔴 CI failing

<a id="zocolini-needs-action"></a>
#### Needs action (3)
- [#3869 chore(swift-sdk): reduce swift-sdk test time in CI](https://github.com/dashpay/platform/pull/3869) — ⚠ merge conflict
- [#3949 chore(swift-sdk): correctly refer to the swift sdk as swift in the build script and cargo profiles](https://github.com/dashpay/platform/pull/3949) — ✋ changes requested · 🔴 CI failing
- [#3955 chore: bump rust-dashcore rev to 95a3c8ff04d75d7abe324db4af01e8521092aff9](https://github.com/dashpay/platform/pull/3955) — ⚠ merge conflict

<a id="zocolini-changes-requested"></a>
#### Changes Requested (1)
- [#3949 chore(swift-sdk): correctly refer to the swift sdk as swift in the build script and cargo profiles](https://github.com/dashpay/platform/pull/3949) — ✋ changes requested · 🔴 CI failing

<a id="zocolini-ci-failing"></a>
#### CI Failing (1)
- [#3970 refactor(platform-wallet): expose the new core TransactionBuilder API + rust-dashcore bump](https://github.com/dashpay/platform/pull/3970) — 🔴 CI failing

<a id="zocolini-clean"></a>
#### Clean (6)
- [#3712 test(swift-sdk): first swift sdk integration tests with local network](https://github.com/dashpay/platform/pull/3712)
- [#3853 fix(swift-sdk): drop legacy headers pre-processing in build_ios.sh](https://github.com/dashpay/platform/pull/3853)
- [#3869 chore(swift-sdk): reduce swift-sdk test time in CI](https://github.com/dashpay/platform/pull/3869) — ⚠ merge conflict
- [#3948 chore: remove ignored cargo build config](https://github.com/dashpay/platform/pull/3948)
- [#3950 chore(swift-sdk): script to get spv stortage from iOS sim](https://github.com/dashpay/platform/pull/3950)
- [#3955 chore: bump rust-dashcore rev to 95a3c8ff04d75d7abe324db4af01e8521092aff9](https://github.com/dashpay/platform/pull/3955) — ⚠ merge conflict

<a id="zocolini-ready-for-review"></a>
#### Ready for Review (1)
- [#3954 feat(platform-wallet)!: shared ThreadRegistry for coordinator lifecycle + shutdown UAF/data-loss fixes](https://github.com/dashpay/platform/pull/3954) — by @Claudius-Maginificent

<a id="pshenmic"></a>
### @pshenmic
<a id="pshenmic-open"></a>
#### Open (1)
- [#2446 feat(js-dapi-client): add contested resources query methods](https://github.com/dashpay/platform/pull/2446) — 5 unresolved (5 CodeRabbit) · 519 days stale · ⚠ merge conflict · 📝 draft · 🐢 targets v2.0-dev, untouched 510 days
  - Top thread: "_:warning: Potential issue_" — 519 days old

<a id="pshenmic-stale"></a>
#### Stale (1)
- [#2446 feat(js-dapi-client): add contested resources query methods](https://github.com/dashpay/platform/pull/2446) — 5 unresolved (5 CodeRabbit) · 519 days stale · ⚠ merge conflict · 📝 draft · 🐢 targets v2.0-dev, untouched 510 days
  - Top thread: "_:warning: Potential issue_" — 519 days old

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
