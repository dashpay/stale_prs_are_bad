---
---
# PR Hygiene Report
*Last updated: 2026-08-06 12:39 UTC · commit 0a3c94a*

## Summary
- Open PRs: **57** (7 clean · 0 CI failing · 2 changes requested · 26 unresolved comments · 11 deferred · 8 draft · 3 stale)
- PRs needing author action: **29**
- Total unresolved comments: **135**

## Scoreboard
_Sort: unresolved-comments desc → needs-action desc → ready-for-review desc. Click any number to jump to the specific PRs it covers._

| Author | Open | Clean | CI failing | Unresolved Comments | Changes Requested | Deferred | Draft | Stale | Needs action | Total Unresolved Comments | Ready for Review | Δ |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| [@bfoss765](#bfoss765) | [10](#bfoss765-open) | [1](#bfoss765-clean) | — | [9](#bfoss765-unresolved-comments) | — | — | — | — | [10](#bfoss765-needs-action) | [40](#bfoss765-unresolved-comments) | — | — |
| [@PastaPastaPasta + (@thepastaclaw)](#pastapastapasta) | [5+(7)](#pastapastapasta-open) | [0+(3)](#pastapastapasta-clean) | — | [4+(1)](#pastapastapasta-unresolved-comments) | — | — | [1+(2)](#pastapastapasta-draft) | [0+(1)](#pastapastapasta-stale) | [4+(0)](#pastapastapasta-needs-action) | [10+(4)](#pastapastapasta-unresolved-comments) | — | — |
| [@shumkov](#shumkov) | [10](#shumkov-open) | [1](#shumkov-clean) | — | [3](#shumkov-unresolved-comments) | [1](#shumkov-changes-requested) | [2](#shumkov-deferred) | [3](#shumkov-draft) | — | [5](#shumkov-needs-action) | [9](#shumkov-unresolved-comments) | [4](#shumkov-ready-for-review) | ↑ 1 |
| [@QuantumExplorer](#quantumexplorer) | [8](#quantumexplorer-open) | [2](#quantumexplorer-clean) | — | [2](#quantumexplorer-unresolved-comments) | [1](#quantumexplorer-changes-requested) | [3](#quantumexplorer-deferred) | — | — | [3](#quantumexplorer-needs-action) | [10](#quantumexplorer-unresolved-comments) | [1](#quantumexplorer-ready-for-review) | — |
| [@llbartekll](#llbartekll) | [3](#llbartekll-open) | — | — | [2](#llbartekll-unresolved-comments) | — | — | [1](#llbartekll-draft) | — | [2](#llbartekll-needs-action) | [5](#llbartekll-unresolved-comments) | [2](#llbartekll-ready-for-review) | — |
| [@lklimek + (@Claudius-Maginificent)](#lklimek) | [5+(4)](#lklimek-open) | — | — | [0+(2)](#lklimek-unresolved-comments) | — | [5+(0)](#lklimek-deferred) | [0+(1)](#lklimek-draft) | [0+(1)](#lklimek-stale) | [0+(2)](#lklimek-needs-action) | [0+(40)](#lklimek-unresolved-comments) | [1+(0)](#lklimek-ready-for-review) | ↓ 2 |
| [@ZocoLini](#zocolini) | [1](#zocolini-open) | — | — | [1](#zocolini-unresolved-comments) | — | — | — | — | [1](#zocolini-needs-action) | [8](#zocolini-unresolved-comments) | [1](#zocolini-ready-for-review) | — |
| [@vivekgsharma](#vivekgsharma) | [1](#vivekgsharma-open) | — | — | [1](#vivekgsharma-unresolved-comments) | — | — | — | — | [1](#vivekgsharma-needs-action) | [2](#vivekgsharma-unresolved-comments) | — | — |
| [@infraclaw-dash](#infraclaw-dash) | [1](#infraclaw-dash-open) | — | — | [1](#infraclaw-dash-unresolved-comments) | — | — | — | — | [1](#infraclaw-dash-needs-action) | [2](#infraclaw-dash-unresolved-comments) | — | — |
| [@pshenmic](#pshenmic) | [1](#pshenmic-open) | — | — | — | — | — | — | [1](#pshenmic-stale) | — | [5](#pshenmic-unresolved-comments) | — | — |
| [@ogabrielides](#ogabrielides) | [1](#ogabrielides-open) | — | — | — | — | [1](#ogabrielides-deferred) | — | — | — | — | — | — |

## Per-author detail

<a id="bfoss765"></a>
### @bfoss765
<a id="bfoss765-open"></a>
#### Open (10)
- [#4310 feat(kotlin-sdk): single-account build_signed_payment send API (funding_path)](https://github.com/dashpay/platform/pull/4310) — 13 unresolved (4 CodeRabbit, 9 human) · 0 days stale · ⚠ merge conflict · ✋ changes requested
  - Top thread: "<!-- thepastaclaw-review v1 finding=9515faef4ca7 dedupe=14153f195b077360 -->" — 0 days old
- [#4311 feat(platform-wallet): token-minting finalize from a funding path (spendable DashPay receival accounts)](https://github.com/dashpay/platform/pull/4311) — 13 unresolved (8 CodeRabbit, 5 human) · 0 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing
  - Top thread: "<!-- thepastaclaw-review v1 finding=c85378dde81e dedupe=785509120f6ac873 -->" — 0 days old
- [#4315 fix(platform-wallet): lossless mpsc persistence drain — root-cause fix for the sync-watermark freeze](https://github.com/dashpay/platform/pull/4315) — 3 unresolved (3 human) · 0 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing
  - Top thread: "<!-- thepastaclaw-review v1 finding=30c2e8e95003 dedupe=d6140bc9d29121d2 -->" — 0 days old
- [#4313 feat(kotlin-sdk): one-time Orchard key shielded-invite API (inviter + claim)](https://github.com/dashpay/platform/pull/4313) — 4 unresolved (3 CodeRabbit, 1 human) · 0 days stale · ⚠ merge conflict · ✋ changes requested
  - Top thread: "<!-- thepastaclaw-review v1 finding=d19c5cf84a9f dedupe=53bcfc51c6e43512 -->" — 0 days old
- [#4312 feat(platform-wallet): multi-output shielded transfers + output-aware fee predictor (two-note invites)](https://github.com/dashpay/platform/pull/4312) — 2 unresolved (1 CodeRabbit, 1 human) · 0 days stale · ✋ changes requested
  - Top thread: "<!-- thepastaclaw-review v1 finding=3a18a774d281 dedupe=409ef31f76ca593c -->" — 0 days old
- [#4316 fix: shield asset-lock funding from all funds accounts incl. CoinJoin (#4073)](https://github.com/dashpay/platform/pull/4316) — 1 unresolved (1 human) · 0 days stale · ⚠ merge conflict · ✋ changes requested
  - Top thread: "<!-- thepastaclaw-review v1 finding=134106565880 dedupe=5d403abf91a6c35a -->" — 0 days old
- [#4317 feat(swift-sdk): add Swift wrappers for the encrypted-txMetadata FFI exports](https://github.com/dashpay/platform/pull/4317) — 1 unresolved (1 human) · 0 days stale · ✋ changes requested · 🔴 CI failing
  - Top thread: "<!-- thepastaclaw-review v1 finding=b57d6ce7f630 dedupe=bad399d8cdf29704 -->" — 0 days old
- [#4318 docs(platform-wallet): error-code registry for the FFI result space](https://github.com/dashpay/platform/pull/4318) — 1 unresolved (1 human) · 0 days stale · ✋ changes requested
  - Top thread: "<!-- thepastaclaw-review v1 finding=5310f59011b8 dedupe=0e038a8a3ff78997 -->" — 0 days old
- [#4309 fix(platform-wallet): age-guard the V2 finalized-transaction handle broadcast](https://github.com/dashpay/platform/pull/4309) — 2 unresolved (2 CodeRabbit) · 0 days stale
  - Top thread: "_📐 Maintainability & Code Quality_ \| _🟡 Minor_ \| _⚡ Quick win_" — 0 days old
- [#4240 feat(kotlin-sdk): add L1 invitation create/claim JNI bridge + DIP-13 invitation persistence](https://github.com/dashpay/platform/pull/4240) — ⚠ merge conflict

<a id="bfoss765-needs-action"></a>
#### Needs action (10)
- [#4310 feat(kotlin-sdk): single-account build_signed_payment send API (funding_path)](https://github.com/dashpay/platform/pull/4310) — 13 unresolved (4 CodeRabbit, 9 human) · 0 days stale · ⚠ merge conflict · ✋ changes requested
  - Top thread: "<!-- thepastaclaw-review v1 finding=9515faef4ca7 dedupe=14153f195b077360 -->" — 0 days old
- [#4311 feat(platform-wallet): token-minting finalize from a funding path (spendable DashPay receival accounts)](https://github.com/dashpay/platform/pull/4311) — 13 unresolved (8 CodeRabbit, 5 human) · 0 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing
  - Top thread: "<!-- thepastaclaw-review v1 finding=c85378dde81e dedupe=785509120f6ac873 -->" — 0 days old
- [#4315 fix(platform-wallet): lossless mpsc persistence drain — root-cause fix for the sync-watermark freeze](https://github.com/dashpay/platform/pull/4315) — 3 unresolved (3 human) · 0 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing
  - Top thread: "<!-- thepastaclaw-review v1 finding=30c2e8e95003 dedupe=d6140bc9d29121d2 -->" — 0 days old
- [#4313 feat(kotlin-sdk): one-time Orchard key shielded-invite API (inviter + claim)](https://github.com/dashpay/platform/pull/4313) — 4 unresolved (3 CodeRabbit, 1 human) · 0 days stale · ⚠ merge conflict · ✋ changes requested
  - Top thread: "<!-- thepastaclaw-review v1 finding=d19c5cf84a9f dedupe=53bcfc51c6e43512 -->" — 0 days old
- [#4312 feat(platform-wallet): multi-output shielded transfers + output-aware fee predictor (two-note invites)](https://github.com/dashpay/platform/pull/4312) — 2 unresolved (1 CodeRabbit, 1 human) · 0 days stale · ✋ changes requested
  - Top thread: "<!-- thepastaclaw-review v1 finding=3a18a774d281 dedupe=409ef31f76ca593c -->" — 0 days old
- [#4316 fix: shield asset-lock funding from all funds accounts incl. CoinJoin (#4073)](https://github.com/dashpay/platform/pull/4316) — 1 unresolved (1 human) · 0 days stale · ⚠ merge conflict · ✋ changes requested
  - Top thread: "<!-- thepastaclaw-review v1 finding=134106565880 dedupe=5d403abf91a6c35a -->" — 0 days old
- [#4317 feat(swift-sdk): add Swift wrappers for the encrypted-txMetadata FFI exports](https://github.com/dashpay/platform/pull/4317) — 1 unresolved (1 human) · 0 days stale · ✋ changes requested · 🔴 CI failing
  - Top thread: "<!-- thepastaclaw-review v1 finding=b57d6ce7f630 dedupe=bad399d8cdf29704 -->" — 0 days old
- [#4318 docs(platform-wallet): error-code registry for the FFI result space](https://github.com/dashpay/platform/pull/4318) — 1 unresolved (1 human) · 0 days stale · ✋ changes requested
  - Top thread: "<!-- thepastaclaw-review v1 finding=5310f59011b8 dedupe=0e038a8a3ff78997 -->" — 0 days old
- [#4309 fix(platform-wallet): age-guard the V2 finalized-transaction handle broadcast](https://github.com/dashpay/platform/pull/4309) — 2 unresolved (2 CodeRabbit) · 0 days stale
  - Top thread: "_📐 Maintainability & Code Quality_ \| _🟡 Minor_ \| _⚡ Quick win_" — 0 days old
- [#4240 feat(kotlin-sdk): add L1 invitation create/claim JNI bridge + DIP-13 invitation persistence](https://github.com/dashpay/platform/pull/4240) — ⚠ merge conflict

<a id="bfoss765-unresolved-comments"></a>
#### Unresolved Comments (9)
- [#4310 feat(kotlin-sdk): single-account build_signed_payment send API (funding_path)](https://github.com/dashpay/platform/pull/4310) — 13 unresolved (4 CodeRabbit, 9 human) · 0 days stale · ⚠ merge conflict · ✋ changes requested
  - Top thread: "<!-- thepastaclaw-review v1 finding=9515faef4ca7 dedupe=14153f195b077360 -->" — 0 days old
- [#4311 feat(platform-wallet): token-minting finalize from a funding path (spendable DashPay receival accounts)](https://github.com/dashpay/platform/pull/4311) — 13 unresolved (8 CodeRabbit, 5 human) · 0 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing
  - Top thread: "<!-- thepastaclaw-review v1 finding=c85378dde81e dedupe=785509120f6ac873 -->" — 0 days old
- [#4315 fix(platform-wallet): lossless mpsc persistence drain — root-cause fix for the sync-watermark freeze](https://github.com/dashpay/platform/pull/4315) — 3 unresolved (3 human) · 0 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing
  - Top thread: "<!-- thepastaclaw-review v1 finding=30c2e8e95003 dedupe=d6140bc9d29121d2 -->" — 0 days old
- [#4313 feat(kotlin-sdk): one-time Orchard key shielded-invite API (inviter + claim)](https://github.com/dashpay/platform/pull/4313) — 4 unresolved (3 CodeRabbit, 1 human) · 0 days stale · ⚠ merge conflict · ✋ changes requested
  - Top thread: "<!-- thepastaclaw-review v1 finding=d19c5cf84a9f dedupe=53bcfc51c6e43512 -->" — 0 days old
- [#4312 feat(platform-wallet): multi-output shielded transfers + output-aware fee predictor (two-note invites)](https://github.com/dashpay/platform/pull/4312) — 2 unresolved (1 CodeRabbit, 1 human) · 0 days stale · ✋ changes requested
  - Top thread: "<!-- thepastaclaw-review v1 finding=3a18a774d281 dedupe=409ef31f76ca593c -->" — 0 days old
- [#4316 fix: shield asset-lock funding from all funds accounts incl. CoinJoin (#4073)](https://github.com/dashpay/platform/pull/4316) — 1 unresolved (1 human) · 0 days stale · ⚠ merge conflict · ✋ changes requested
  - Top thread: "<!-- thepastaclaw-review v1 finding=134106565880 dedupe=5d403abf91a6c35a -->" — 0 days old
- [#4317 feat(swift-sdk): add Swift wrappers for the encrypted-txMetadata FFI exports](https://github.com/dashpay/platform/pull/4317) — 1 unresolved (1 human) · 0 days stale · ✋ changes requested · 🔴 CI failing
  - Top thread: "<!-- thepastaclaw-review v1 finding=b57d6ce7f630 dedupe=bad399d8cdf29704 -->" — 0 days old
- [#4318 docs(platform-wallet): error-code registry for the FFI result space](https://github.com/dashpay/platform/pull/4318) — 1 unresolved (1 human) · 0 days stale · ✋ changes requested
  - Top thread: "<!-- thepastaclaw-review v1 finding=5310f59011b8 dedupe=0e038a8a3ff78997 -->" — 0 days old
- [#4309 fix(platform-wallet): age-guard the V2 finalized-transaction handle broadcast](https://github.com/dashpay/platform/pull/4309) — 2 unresolved (2 CodeRabbit) · 0 days stale
  - Top thread: "_📐 Maintainability & Code Quality_ \| _🟡 Minor_ \| _⚡ Quick win_" — 0 days old

<a id="bfoss765-clean"></a>
#### Clean (1)
- [#4240 feat(kotlin-sdk): add L1 invitation create/claim JNI bridge + DIP-13 invitation persistence](https://github.com/dashpay/platform/pull/4240) — ⚠ merge conflict

<a id="pastapastapasta"></a>
### @PastaPastaPasta + (@thepastaclaw)
<a id="pastapastapasta-open"></a>
#### Open (12)
- [#3096 feat(sdk): add client-side validation to state transition construction methods](https://github.com/dashpay/platform/pull/3096) — via @thepastaclaw · 4 unresolved (2 CodeRabbit, 2 human) · 169 days stale
  - Top thread: "_⚠️ Potential issue_ \| _🟡 Minor_" — 166 days old
- [#3680 refactor(dapi,dpp)!: move dapi-client and Identifier off Buffer to Uint8Array](https://github.com/dashpay/platform/pull/3680) — 5 unresolved (5 human) · 72 days stale · ⚠ merge conflict · 🔴 CI failing
  - Top thread: "<!-- thepastaclaw-review v1 finding=1c101f507542 -->" — 72 days old
- [#3462 fix(dpp)!: version-gate distribution function floating-point evaluation](https://github.com/dashpay/platform/pull/3462) — 3 unresolved (3 human) · 66 days stale · 🔴 CI failing
  - Top thread: "<!-- thepastaclaw-review v1 finding=7d40971bd5ff dedupe=2846c55b07c76472 -->" — 66 days old
- [#3509 fix(platform): default omitted proved query limits](https://github.com/dashpay/platform/pull/3509) — 1 unresolved (1 human) · 96 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing
  - Top thread: "<!-- thepastaclaw-review v1 finding=d3df3e197206 -->" — 96 days old
- [#2988 feat(wasm-sdk): auto-generate entropy for document creation when not provided](https://github.com/dashpay/platform/pull/2988) — 1 unresolved (1 human) · 77 days stale · 🔴 CI failing
  - Top thread: "<!-- thepastaclaw-review v1 finding=30cbefe1c9ef -->" — 77 days old
- [#3635 ci: tolerate book preview comment permission errors](https://github.com/dashpay/platform/pull/3635) — via @thepastaclaw
- [#3689 refactor(sdk,wallet-lib,test-suite)!: convert dapi-client + wallet-lib + js-dash-sdk + platform-test-suite to ESM](https://github.com/dashpay/platform/pull/3689) — ⚠ merge conflict · 📝 draft
- [#3898 fix(dashmate): re-sync stale 3.x Drive and rs-dapi images](https://github.com/dashpay/platform/pull/3898) — via @thepastaclaw
- [#4016 fix(platform-wallet): close asset-lock resume broadcast race](https://github.com/dashpay/platform/pull/4016) — via @thepastaclaw · ⚠ merge conflict · 📝 draft
- [#4136 fix(dashmate): use live Tenderdash app version for protocol status](https://github.com/dashpay/platform/pull/4136) — via @thepastaclaw · 📝 draft
- [#4298 fix(dashmate): load ZeroSSL config in force mode](https://github.com/dashpay/platform/pull/4298) — via @thepastaclaw · 📝 draft · 🐢 targets v4.1-dev
- [#4307 fix(dashmate): extend Core shutdown grace period](https://github.com/dashpay/platform/pull/4307) — via @thepastaclaw

<a id="pastapastapasta-needs-action"></a>
#### Needs action (4)
- [#3680 refactor(dapi,dpp)!: move dapi-client and Identifier off Buffer to Uint8Array](https://github.com/dashpay/platform/pull/3680) — 5 unresolved (5 human) · 72 days stale · ⚠ merge conflict · 🔴 CI failing
  - Top thread: "<!-- thepastaclaw-review v1 finding=1c101f507542 -->" — 72 days old
- [#3462 fix(dpp)!: version-gate distribution function floating-point evaluation](https://github.com/dashpay/platform/pull/3462) — 3 unresolved (3 human) · 66 days stale · 🔴 CI failing
  - Top thread: "<!-- thepastaclaw-review v1 finding=7d40971bd5ff dedupe=2846c55b07c76472 -->" — 66 days old
- [#3509 fix(platform): default omitted proved query limits](https://github.com/dashpay/platform/pull/3509) — 1 unresolved (1 human) · 96 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing
  - Top thread: "<!-- thepastaclaw-review v1 finding=d3df3e197206 -->" — 96 days old
- [#2988 feat(wasm-sdk): auto-generate entropy for document creation when not provided](https://github.com/dashpay/platform/pull/2988) — 1 unresolved (1 human) · 77 days stale · 🔴 CI failing
  - Top thread: "<!-- thepastaclaw-review v1 finding=30cbefe1c9ef -->" — 77 days old

<a id="pastapastapasta-unresolved-comments"></a>
#### Unresolved Comments (5)
- [#3096 feat(sdk): add client-side validation to state transition construction methods](https://github.com/dashpay/platform/pull/3096) — via @thepastaclaw · 4 unresolved (2 CodeRabbit, 2 human) · 169 days stale
  - Top thread: "_⚠️ Potential issue_ \| _🟡 Minor_" — 166 days old
- [#3680 refactor(dapi,dpp)!: move dapi-client and Identifier off Buffer to Uint8Array](https://github.com/dashpay/platform/pull/3680) — 5 unresolved (5 human) · 72 days stale · ⚠ merge conflict · 🔴 CI failing
  - Top thread: "<!-- thepastaclaw-review v1 finding=1c101f507542 -->" — 72 days old
- [#3462 fix(dpp)!: version-gate distribution function floating-point evaluation](https://github.com/dashpay/platform/pull/3462) — 3 unresolved (3 human) · 66 days stale · 🔴 CI failing
  - Top thread: "<!-- thepastaclaw-review v1 finding=7d40971bd5ff dedupe=2846c55b07c76472 -->" — 66 days old
- [#3509 fix(platform): default omitted proved query limits](https://github.com/dashpay/platform/pull/3509) — 1 unresolved (1 human) · 96 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing
  - Top thread: "<!-- thepastaclaw-review v1 finding=d3df3e197206 -->" — 96 days old
- [#2988 feat(wasm-sdk): auto-generate entropy for document creation when not provided](https://github.com/dashpay/platform/pull/2988) — 1 unresolved (1 human) · 77 days stale · 🔴 CI failing
  - Top thread: "<!-- thepastaclaw-review v1 finding=30cbefe1c9ef -->" — 77 days old

<a id="pastapastapasta-draft"></a>
#### Draft (3)
- [#3689 refactor(sdk,wallet-lib,test-suite)!: convert dapi-client + wallet-lib + js-dash-sdk + platform-test-suite to ESM](https://github.com/dashpay/platform/pull/3689) — ⚠ merge conflict · 📝 draft
- [#4016 fix(platform-wallet): close asset-lock resume broadcast race](https://github.com/dashpay/platform/pull/4016) — via @thepastaclaw · ⚠ merge conflict · 📝 draft
- [#4136 fix(dashmate): use live Tenderdash app version for protocol status](https://github.com/dashpay/platform/pull/4136) — via @thepastaclaw · 📝 draft

<a id="pastapastapasta-stale"></a>
#### Stale (1)
- [#4298 fix(dashmate): load ZeroSSL config in force mode](https://github.com/dashpay/platform/pull/4298) — via @thepastaclaw · 📝 draft · 🐢 targets v4.1-dev

<a id="pastapastapasta-clean"></a>
#### Clean (3)
- [#3635 ci: tolerate book preview comment permission errors](https://github.com/dashpay/platform/pull/3635) — via @thepastaclaw
- [#3898 fix(dashmate): re-sync stale 3.x Drive and rs-dapi images](https://github.com/dashpay/platform/pull/3898) — via @thepastaclaw
- [#4307 fix(dashmate): extend Core shutdown grace period](https://github.com/dashpay/platform/pull/4307) — via @thepastaclaw

<a id="shumkov"></a>
### @shumkov
<a id="shumkov-open"></a>
#### Open (10)
- [#3936 chore(drive-abci): update to nested address in SML](https://github.com/dashpay/platform/pull/3936) — 2 unresolved (2 human) · 48 days stale · ⚠ merge conflict · 📝 draft
  - Top thread: "<!-- thepastaclaw-review v1 finding=2e1950367681 dedupe=494e3310440bc19a -->" — 48 days old
- [#4283 fix(dashmate): keep the node up when an image pull fails, and stop reporting success when it did not](https://github.com/dashpay/platform/pull/4283) — 3 unresolved (3 human) · 2 days stale · ✋ changes requested · 🔴 CI failing
  - Top thread: "<!-- thepastaclaw-review v1 finding=00d71d6c14ff dedupe=7febbb362c746b49 -->" — 2 days old
- [#4248 fix(dashmate)!: stop concurrent commands reverting each other's config changes](https://github.com/dashpay/platform/pull/4248) — 1 unresolved (1 human) · 8 days stale · ✋ changes requested
  - Top thread: "<!-- thepastaclaw-review v1 finding=283db0d490a7 dedupe=2b445bd7e1066073 -->" — 8 days old
- [#4282 fix(dashmate)!: give Debian packages versions apt can order](https://github.com/dashpay/platform/pull/4282) — 3 unresolved (1 CodeRabbit, 2 human) · 2 days stale · ⚠ merge conflict · 🔴 CI failing
  - Top thread: "_🔒 Security & Privacy_ \| _🟡 Minor_ \| _⚡ Quick win_" — 2 days old
- [#2392 feat(dashmate): single node local network [WIP\]](https://github.com/dashpay/platform/pull/2392) — ⏸ deferred
- [#2518 feat: token marketplace](https://github.com/dashpay/platform/pull/2518) — 📝 draft · ⏸ deferred
- [#4229 fix(release): base changelog on the immediately-preceding release](https://github.com/dashpay/platform/pull/4229) — ✋ changes requested · 🔴 CI failing
- [#4243 fix(sdk)!: complete encrypted txMetadata parity](https://github.com/dashpay/platform/pull/4243) — ⚠ merge conflict
- [#4274 feat(dashmate): expose Tenderdash consensus DoS rate-limit knobs](https://github.com/dashpay/platform/pull/4274) — 🔴 CI failing · 📝 draft
- [#4285 feat(platform-wallet)!: invitation links are AppsFlyer applinks only](https://github.com/dashpay/platform/pull/4285) — ⚠ merge conflict · 🔴 CI failing · 📝 draft

<a id="shumkov-needs-action"></a>
#### Needs action (5)
- [#4283 fix(dashmate): keep the node up when an image pull fails, and stop reporting success when it did not](https://github.com/dashpay/platform/pull/4283) — 3 unresolved (3 human) · 2 days stale · ✋ changes requested · 🔴 CI failing
  - Top thread: "<!-- thepastaclaw-review v1 finding=00d71d6c14ff dedupe=7febbb362c746b49 -->" — 2 days old
- [#4248 fix(dashmate)!: stop concurrent commands reverting each other's config changes](https://github.com/dashpay/platform/pull/4248) — 1 unresolved (1 human) · 8 days stale · ✋ changes requested
  - Top thread: "<!-- thepastaclaw-review v1 finding=283db0d490a7 dedupe=2b445bd7e1066073 -->" — 8 days old
- [#4282 fix(dashmate)!: give Debian packages versions apt can order](https://github.com/dashpay/platform/pull/4282) — 3 unresolved (1 CodeRabbit, 2 human) · 2 days stale · ⚠ merge conflict · 🔴 CI failing
  - Top thread: "_🔒 Security & Privacy_ \| _🟡 Minor_ \| _⚡ Quick win_" — 2 days old
- [#4229 fix(release): base changelog on the immediately-preceding release](https://github.com/dashpay/platform/pull/4229) — ✋ changes requested · 🔴 CI failing
- [#4243 fix(sdk)!: complete encrypted txMetadata parity](https://github.com/dashpay/platform/pull/4243) — ⚠ merge conflict

<a id="shumkov-unresolved-comments"></a>
#### Unresolved Comments (3)
- [#4283 fix(dashmate): keep the node up when an image pull fails, and stop reporting success when it did not](https://github.com/dashpay/platform/pull/4283) — 3 unresolved (3 human) · 2 days stale · ✋ changes requested · 🔴 CI failing
  - Top thread: "<!-- thepastaclaw-review v1 finding=00d71d6c14ff dedupe=7febbb362c746b49 -->" — 2 days old
- [#4248 fix(dashmate)!: stop concurrent commands reverting each other's config changes](https://github.com/dashpay/platform/pull/4248) — 1 unresolved (1 human) · 8 days stale · ✋ changes requested
  - Top thread: "<!-- thepastaclaw-review v1 finding=283db0d490a7 dedupe=2b445bd7e1066073 -->" — 8 days old
- [#4282 fix(dashmate)!: give Debian packages versions apt can order](https://github.com/dashpay/platform/pull/4282) — 3 unresolved (1 CodeRabbit, 2 human) · 2 days stale · ⚠ merge conflict · 🔴 CI failing
  - Top thread: "_🔒 Security & Privacy_ \| _🟡 Minor_ \| _⚡ Quick win_" — 2 days old

<a id="shumkov-changes-requested"></a>
#### Changes Requested (1)
- [#4229 fix(release): base changelog on the immediately-preceding release](https://github.com/dashpay/platform/pull/4229) — ✋ changes requested · 🔴 CI failing

<a id="shumkov-deferred"></a>
#### Deferred (2)
- [#2392 feat(dashmate): single node local network [WIP\]](https://github.com/dashpay/platform/pull/2392) — ⏸ deferred
- [#2518 feat: token marketplace](https://github.com/dashpay/platform/pull/2518) — 📝 draft · ⏸ deferred

<a id="shumkov-draft"></a>
#### Draft (3)
- [#3936 chore(drive-abci): update to nested address in SML](https://github.com/dashpay/platform/pull/3936) — 2 unresolved (2 human) · 48 days stale · ⚠ merge conflict · 📝 draft
  - Top thread: "<!-- thepastaclaw-review v1 finding=2e1950367681 dedupe=494e3310440bc19a -->" — 48 days old
- [#4274 feat(dashmate): expose Tenderdash consensus DoS rate-limit knobs](https://github.com/dashpay/platform/pull/4274) — 🔴 CI failing · 📝 draft
- [#4285 feat(platform-wallet)!: invitation links are AppsFlyer applinks only](https://github.com/dashpay/platform/pull/4285) — ⚠ merge conflict · 🔴 CI failing · 📝 draft

<a id="shumkov-clean"></a>
#### Clean (1)
- [#4243 fix(sdk)!: complete encrypted txMetadata parity](https://github.com/dashpay/platform/pull/4243) — ⚠ merge conflict

<a id="shumkov-ready-for-review"></a>
#### Ready for Review (4)
- [#3898 fix(dashmate): re-sync stale 3.x Drive and rs-dapi images](https://github.com/dashpay/platform/pull/3898) — by @thepastaclaw
- [#4221 feat(sdk): prefer DAPI peers supporting the client's latest protocol version](https://github.com/dashpay/platform/pull/4221) — by @QuantumExplorer
- [#4307 fix(dashmate): extend Core shutdown grace period](https://github.com/dashpay/platform/pull/4307) — by @thepastaclaw
- [#4321 fix(platform-wallet): type signer-reported missing key as MessageSigningKeyUnavailable](https://github.com/dashpay/platform/pull/4321) — by @QuantumExplorer

<a id="quantumexplorer"></a>
### @QuantumExplorer
<a id="quantumexplorer-open"></a>
#### Open (8)
- [#4272 feat(dpp)!: payment addresses and payment key purposes (DIP-33)](https://github.com/dashpay/platform/pull/4272) — 9 unresolved (3 CodeRabbit, 6 human) · 3 days stale · ✋ changes requested · 🔴 CI failing
  - Top thread: "<!-- thepastaclaw-review v1 finding=754e3f1b8923 dedupe=5e6944517d4fedbc -->" — 3 days old
- [#4218 fix(platform): reject contradictory keep-history document deletes](https://github.com/dashpay/platform/pull/4218) — 1 unresolved (1 human) · 13 days stale · ⚠ merge conflict · 🔴 CI failing
  - Top thread: "<!-- thepastaclaw-review v1 finding=e868506e7d4f dedupe=effb5abb63dcd615 -->" — 13 days old
- [#1834 feat(drive-abci)!: Statesync](https://github.com/dashpay/platform/pull/1834) — 📝 draft · ⏸ deferred
- [#3021 feat(platform): better contract state transitions](https://github.com/dashpay/platform/pull/3021) — 📝 draft · ⏸ deferred
- [#3417 feat(swift-sdk): use SPV-synced quorums for Platform proof verification](https://github.com/dashpay/platform/pull/3417) — ⚠ merge conflict · ✋ changes requested · 🔴 CI failing
- [#3740 feat: add time-range indexes for trending/leaderboard queries](https://github.com/dashpay/platform/pull/3740) — ⏸ deferred
- [#4221 feat(sdk): prefer DAPI peers supporting the client's latest protocol version](https://github.com/dashpay/platform/pull/4221)
- [#4321 fix(platform-wallet): type signer-reported missing key as MessageSigningKeyUnavailable](https://github.com/dashpay/platform/pull/4321)

<a id="quantumexplorer-needs-action"></a>
#### Needs action (3)
- [#4272 feat(dpp)!: payment addresses and payment key purposes (DIP-33)](https://github.com/dashpay/platform/pull/4272) — 9 unresolved (3 CodeRabbit, 6 human) · 3 days stale · ✋ changes requested · 🔴 CI failing
  - Top thread: "<!-- thepastaclaw-review v1 finding=754e3f1b8923 dedupe=5e6944517d4fedbc -->" — 3 days old
- [#4218 fix(platform): reject contradictory keep-history document deletes](https://github.com/dashpay/platform/pull/4218) — 1 unresolved (1 human) · 13 days stale · ⚠ merge conflict · 🔴 CI failing
  - Top thread: "<!-- thepastaclaw-review v1 finding=e868506e7d4f dedupe=effb5abb63dcd615 -->" — 13 days old
- [#3417 feat(swift-sdk): use SPV-synced quorums for Platform proof verification](https://github.com/dashpay/platform/pull/3417) — ⚠ merge conflict · ✋ changes requested · 🔴 CI failing

<a id="quantumexplorer-unresolved-comments"></a>
#### Unresolved Comments (2)
- [#4272 feat(dpp)!: payment addresses and payment key purposes (DIP-33)](https://github.com/dashpay/platform/pull/4272) — 9 unresolved (3 CodeRabbit, 6 human) · 3 days stale · ✋ changes requested · 🔴 CI failing
  - Top thread: "<!-- thepastaclaw-review v1 finding=754e3f1b8923 dedupe=5e6944517d4fedbc -->" — 3 days old
- [#4218 fix(platform): reject contradictory keep-history document deletes](https://github.com/dashpay/platform/pull/4218) — 1 unresolved (1 human) · 13 days stale · ⚠ merge conflict · 🔴 CI failing
  - Top thread: "<!-- thepastaclaw-review v1 finding=e868506e7d4f dedupe=effb5abb63dcd615 -->" — 13 days old

<a id="quantumexplorer-changes-requested"></a>
#### Changes Requested (1)
- [#3417 feat(swift-sdk): use SPV-synced quorums for Platform proof verification](https://github.com/dashpay/platform/pull/3417) — ⚠ merge conflict · ✋ changes requested · 🔴 CI failing

<a id="quantumexplorer-deferred"></a>
#### Deferred (3)
- [#1834 feat(drive-abci)!: Statesync](https://github.com/dashpay/platform/pull/1834) — 📝 draft · ⏸ deferred
- [#3021 feat(platform): better contract state transitions](https://github.com/dashpay/platform/pull/3021) — 📝 draft · ⏸ deferred
- [#3740 feat: add time-range indexes for trending/leaderboard queries](https://github.com/dashpay/platform/pull/3740) — ⏸ deferred

<a id="quantumexplorer-clean"></a>
#### Clean (2)
- [#4221 feat(sdk): prefer DAPI peers supporting the client's latest protocol version](https://github.com/dashpay/platform/pull/4221)
- [#4321 fix(platform-wallet): type signer-reported missing key as MessageSigningKeyUnavailable](https://github.com/dashpay/platform/pull/4321)

<a id="quantumexplorer-ready-for-review"></a>
#### Ready for Review (1)
- [#3635 ci: tolerate book preview comment permission errors](https://github.com/dashpay/platform/pull/3635) — by @thepastaclaw

<a id="llbartekll"></a>
### @llbartekll
<a id="llbartekll-open"></a>
#### Open (3)
- [#3560 test(swift-sdk): add testnet identity-discovery UI test](https://github.com/dashpay/platform/pull/3560) — 2 unresolved (2 human) · 66 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing
  - Top thread: "<!-- thepastaclaw-review v1 finding=379c84ed00e1 dedupe=08a70e07f6e9ae45 -->" — 66 days old
- [#4170 feat(swift-sdk): make wallet deletion asynchronous](https://github.com/dashpay/platform/pull/4170) — 2 unresolved (2 human) · 17 days stale · ⚠ merge conflict · ✋ changes requested · 📝 draft
  - Top thread: "<!-- thepastaclaw-review v1 finding=39eeb5416a91 dedupe=a66ec193a2ba712e -->" — 17 days old
- [#3694 fix(swift-example-app): unhide Create Identity submit button and auto-dismiss sheet on success](https://github.com/dashpay/platform/pull/3694) — 1 unresolved (1 human) · 66 days stale · ✋ changes requested
  - Top thread: "<!-- thepastaclaw-review v1 finding=e3a849e745d0 dedupe=d76d5740c5bc0f94 -->" — 66 days old

<a id="llbartekll-needs-action"></a>
#### Needs action (2)
- [#3560 test(swift-sdk): add testnet identity-discovery UI test](https://github.com/dashpay/platform/pull/3560) — 2 unresolved (2 human) · 66 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing
  - Top thread: "<!-- thepastaclaw-review v1 finding=379c84ed00e1 dedupe=08a70e07f6e9ae45 -->" — 66 days old
- [#3694 fix(swift-example-app): unhide Create Identity submit button and auto-dismiss sheet on success](https://github.com/dashpay/platform/pull/3694) — 1 unresolved (1 human) · 66 days stale · ✋ changes requested
  - Top thread: "<!-- thepastaclaw-review v1 finding=e3a849e745d0 dedupe=d76d5740c5bc0f94 -->" — 66 days old

<a id="llbartekll-unresolved-comments"></a>
#### Unresolved Comments (2)
- [#3560 test(swift-sdk): add testnet identity-discovery UI test](https://github.com/dashpay/platform/pull/3560) — 2 unresolved (2 human) · 66 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing
  - Top thread: "<!-- thepastaclaw-review v1 finding=379c84ed00e1 dedupe=08a70e07f6e9ae45 -->" — 66 days old
- [#3694 fix(swift-example-app): unhide Create Identity submit button and auto-dismiss sheet on success](https://github.com/dashpay/platform/pull/3694) — 1 unresolved (1 human) · 66 days stale · ✋ changes requested
  - Top thread: "<!-- thepastaclaw-review v1 finding=e3a849e745d0 dedupe=d76d5740c5bc0f94 -->" — 66 days old

<a id="llbartekll-draft"></a>
#### Draft (1)
- [#4170 feat(swift-sdk): make wallet deletion asynchronous](https://github.com/dashpay/platform/pull/4170) — 2 unresolved (2 human) · 17 days stale · ⚠ merge conflict · ✋ changes requested · 📝 draft
  - Top thread: "<!-- thepastaclaw-review v1 finding=39eeb5416a91 dedupe=a66ec193a2ba712e -->" — 17 days old

<a id="llbartekll-ready-for-review"></a>
#### Ready for Review (2)
- [#4221 feat(sdk): prefer DAPI peers supporting the client's latest protocol version](https://github.com/dashpay/platform/pull/4221) — by @QuantumExplorer
- [#4321 fix(platform-wallet): type signer-reported missing key as MessageSigningKeyUnavailable](https://github.com/dashpay/platform/pull/4321) — by @QuantumExplorer

<a id="lklimek"></a>
### @lklimek + (@Claudius-Maginificent)
<a id="lklimek-open"></a>
#### Open (9)
- [#3549 test(platform-wallet): e2e framework + full test suite — triage pins, Found-*/PA-* guards, fail-closed persist, Stage-2 merge](https://github.com/dashpay/platform/pull/3549) — via @Claudius-Maginificent · 20 unresolved (20 human) · 96 days stale · ⚠ merge conflict · 📝 draft
  - Top thread: "<!-- thepastaclaw-review v1 finding=a3725e9d50ca -->" — 96 days old
- [#3968 feat(platform-wallet-storage): embeddable SQLite persistence backend with seedless rehydration](https://github.com/dashpay/platform/pull/3968) — via @Claudius-Maginificent · 11 unresolved (11 human) · 24 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing
  - Top thread: "Vault durability: failed parent-dir fsync still returns `Ok`.** After the atomic `persist()` rename, a failed directory …" — 24 days old
- [#3992 feat(platform-wallet): manifest integrity checksum (Risk-6/R12.5 follow-up)](https://github.com/dashpay/platform/pull/3992) — via @Claudius-Maginificent · 7 unresolved (7 human) · 34 days stale · ⚠ merge conflict · ✋ changes requested · 🐢 targets feat/platform-wallet-storage-rehydration
  - Top thread: "<!-- thepastaclaw-review v1 finding=3969d61c562f dedupe=57a8bf7151c0879e -->" — 34 days old
- [#4257 fix(platform-wallet): emit the real locking script for spent UTXOs](https://github.com/dashpay/platform/pull/4257) — via @Claudius-Maginificent · 2 unresolved (2 human) · 6 days stale · 🔴 CI failing
  - Top thread: "<!-- thepastaclaw-review v1 finding=9af4ec9cbfe0 dedupe=cb2470596bb4de92 -->" — 6 days old
- [#2679 fix(drive-abci): don't panic on unsupported version error](https://github.com/dashpay/platform/pull/2679) — ⏸ deferred
- [#2795 feat(platform): add real-time platform event subscriptions via gRPC streaming](https://github.com/dashpay/platform/pull/2795) — ⏸ deferred
- [#2993 feat: identity reference validation](https://github.com/dashpay/platform/pull/2993) — 📝 draft · ⏸ deferred
- [#3009 feat: contract reference validation](https://github.com/dashpay/platform/pull/3009) — 📝 draft · ⏸ deferred
- [#3032 feat: document creation restriction by group membership](https://github.com/dashpay/platform/pull/3032) — 📝 draft · ⏸ deferred

<a id="lklimek-needs-action"></a>
#### Needs action (2)
- [#3968 feat(platform-wallet-storage): embeddable SQLite persistence backend with seedless rehydration](https://github.com/dashpay/platform/pull/3968) — via @Claudius-Maginificent · 11 unresolved (11 human) · 24 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing
  - Top thread: "Vault durability: failed parent-dir fsync still returns `Ok`.** After the atomic `persist()` rename, a failed directory …" — 24 days old
- [#4257 fix(platform-wallet): emit the real locking script for spent UTXOs](https://github.com/dashpay/platform/pull/4257) — via @Claudius-Maginificent · 2 unresolved (2 human) · 6 days stale · 🔴 CI failing
  - Top thread: "<!-- thepastaclaw-review v1 finding=9af4ec9cbfe0 dedupe=cb2470596bb4de92 -->" — 6 days old

<a id="lklimek-unresolved-comments"></a>
#### Unresolved Comments (2)
- [#3968 feat(platform-wallet-storage): embeddable SQLite persistence backend with seedless rehydration](https://github.com/dashpay/platform/pull/3968) — via @Claudius-Maginificent · 11 unresolved (11 human) · 24 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing
  - Top thread: "Vault durability: failed parent-dir fsync still returns `Ok`.** After the atomic `persist()` rename, a failed directory …" — 24 days old
- [#4257 fix(platform-wallet): emit the real locking script for spent UTXOs](https://github.com/dashpay/platform/pull/4257) — via @Claudius-Maginificent · 2 unresolved (2 human) · 6 days stale · 🔴 CI failing
  - Top thread: "<!-- thepastaclaw-review v1 finding=9af4ec9cbfe0 dedupe=cb2470596bb4de92 -->" — 6 days old

<a id="lklimek-deferred"></a>
#### Deferred (5)
- [#2679 fix(drive-abci): don't panic on unsupported version error](https://github.com/dashpay/platform/pull/2679) — ⏸ deferred
- [#2795 feat(platform): add real-time platform event subscriptions via gRPC streaming](https://github.com/dashpay/platform/pull/2795) — ⏸ deferred
- [#2993 feat: identity reference validation](https://github.com/dashpay/platform/pull/2993) — 📝 draft · ⏸ deferred
- [#3009 feat: contract reference validation](https://github.com/dashpay/platform/pull/3009) — 📝 draft · ⏸ deferred
- [#3032 feat: document creation restriction by group membership](https://github.com/dashpay/platform/pull/3032) — 📝 draft · ⏸ deferred

<a id="lklimek-draft"></a>
#### Draft (1)
- [#3549 test(platform-wallet): e2e framework + full test suite — triage pins, Found-*/PA-* guards, fail-closed persist, Stage-2 merge](https://github.com/dashpay/platform/pull/3549) — via @Claudius-Maginificent · 20 unresolved (20 human) · 96 days stale · ⚠ merge conflict · 📝 draft
  - Top thread: "<!-- thepastaclaw-review v1 finding=a3725e9d50ca -->" — 96 days old

<a id="lklimek-stale"></a>
#### Stale (1)
- [#3992 feat(platform-wallet): manifest integrity checksum (Risk-6/R12.5 follow-up)](https://github.com/dashpay/platform/pull/3992) — via @Claudius-Maginificent · 7 unresolved (7 human) · 34 days stale · ⚠ merge conflict · ✋ changes requested · 🐢 targets feat/platform-wallet-storage-rehydration
  - Top thread: "<!-- thepastaclaw-review v1 finding=3969d61c562f dedupe=57a8bf7151c0879e -->" — 34 days old

<a id="lklimek-ready-for-review"></a>
#### Ready for Review (1)
- [#4321 fix(platform-wallet): type signer-reported missing key as MessageSigningKeyUnavailable](https://github.com/dashpay/platform/pull/4321) — by @QuantumExplorer

<a id="zocolini"></a>
### @ZocoLini
<a id="zocolini-open"></a>
#### Open (1)
- [#4064 test(swift-sdk): mid flight added wallet integration test](https://github.com/dashpay/platform/pull/4064) — 8 unresolved (2 CodeRabbit, 6 human) · 27 days stale
  - Top thread: "_🩺 Stability & Availability_ \| _🟡 Minor_ \| _⚡ Quick win_" — 27 days old

<a id="zocolini-needs-action"></a>
#### Needs action (1)
- [#4064 test(swift-sdk): mid flight added wallet integration test](https://github.com/dashpay/platform/pull/4064) — 8 unresolved (2 CodeRabbit, 6 human) · 27 days stale
  - Top thread: "_🩺 Stability & Availability_ \| _🟡 Minor_ \| _⚡ Quick win_" — 27 days old

<a id="zocolini-unresolved-comments"></a>
#### Unresolved Comments (1)
- [#4064 test(swift-sdk): mid flight added wallet integration test](https://github.com/dashpay/platform/pull/4064) — 8 unresolved (2 CodeRabbit, 6 human) · 27 days stale
  - Top thread: "_🩺 Stability & Availability_ \| _🟡 Minor_ \| _⚡ Quick win_" — 27 days old

<a id="zocolini-ready-for-review"></a>
#### Ready for Review (1)
- [#4321 fix(platform-wallet): type signer-reported missing key as MessageSigningKeyUnavailable](https://github.com/dashpay/platform/pull/4321) — by @QuantumExplorer

<a id="vivekgsharma"></a>
### @vivekgsharma
<a id="vivekgsharma-open"></a>
#### Open (1)
- [#4189 ci: enforce macOS runner disk reserve](https://github.com/dashpay/platform/pull/4189) — 2 unresolved (1 CodeRabbit, 1 human) · 16 days stale · ✋ changes requested · 🔴 CI failing
  - Top thread: "<!-- thepastaclaw-review v1 finding=cb3ec4ed35e4 dedupe=7c8bd1045d5ae48a -->" — 15 days old

<a id="vivekgsharma-needs-action"></a>
#### Needs action (1)
- [#4189 ci: enforce macOS runner disk reserve](https://github.com/dashpay/platform/pull/4189) — 2 unresolved (1 CodeRabbit, 1 human) · 16 days stale · ✋ changes requested · 🔴 CI failing
  - Top thread: "<!-- thepastaclaw-review v1 finding=cb3ec4ed35e4 dedupe=7c8bd1045d5ae48a -->" — 15 days old

<a id="vivekgsharma-unresolved-comments"></a>
#### Unresolved Comments (1)
- [#4189 ci: enforce macOS runner disk reserve](https://github.com/dashpay/platform/pull/4189) — 2 unresolved (1 CodeRabbit, 1 human) · 16 days stale · ✋ changes requested · 🔴 CI failing
  - Top thread: "<!-- thepastaclaw-review v1 finding=cb3ec4ed35e4 dedupe=7c8bd1045d5ae48a -->" — 15 days old

<a id="infraclaw-dash"></a>
### @infraclaw-dash
<a id="infraclaw-dash-open"></a>
#### Open (1)
- [#3958 ci: add Platform testnet sync status reporting](https://github.com/dashpay/platform/pull/3958) — 2 unresolved (2 human) · 43 days stale · ⚠ merge conflict
  - Top thread: "<!-- thepastaclaw-review v1 finding=5ae1197b579a dedupe=6feca8368dcb61f8 -->" — 43 days old

<a id="infraclaw-dash-needs-action"></a>
#### Needs action (1)
- [#3958 ci: add Platform testnet sync status reporting](https://github.com/dashpay/platform/pull/3958) — 2 unresolved (2 human) · 43 days stale · ⚠ merge conflict
  - Top thread: "<!-- thepastaclaw-review v1 finding=5ae1197b579a dedupe=6feca8368dcb61f8 -->" — 43 days old

<a id="infraclaw-dash-unresolved-comments"></a>
#### Unresolved Comments (1)
- [#3958 ci: add Platform testnet sync status reporting](https://github.com/dashpay/platform/pull/3958) — 2 unresolved (2 human) · 43 days stale · ⚠ merge conflict
  - Top thread: "<!-- thepastaclaw-review v1 finding=5ae1197b579a dedupe=6feca8368dcb61f8 -->" — 43 days old

<a id="pshenmic"></a>
### @pshenmic
<a id="pshenmic-open"></a>
#### Open (1)
- [#2446 feat(js-dapi-client): add contested resources query methods](https://github.com/dashpay/platform/pull/2446) — 5 unresolved (5 CodeRabbit) · 554 days stale · ⚠ merge conflict · 📝 draft · 🐢 targets v2.0-dev, untouched 544 days
  - Top thread: "_:warning: Potential issue_" — 554 days old

<a id="pshenmic-stale"></a>
#### Stale (1)
- [#2446 feat(js-dapi-client): add contested resources query methods](https://github.com/dashpay/platform/pull/2446) — 5 unresolved (5 CodeRabbit) · 554 days stale · ⚠ merge conflict · 📝 draft · 🐢 targets v2.0-dev, untouched 544 days
  - Top thread: "_:warning: Potential issue_" — 554 days old

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
