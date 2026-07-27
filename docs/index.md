---
---
# PR Hygiene Report
*Last updated: 2026-07-27 00:37 UTC · commit b8eb3bb*

## Summary
- Open PRs: **54** (4 clean · 1 CI failing · 1 changes requested · 26 unresolved comments · 11 deferred · 6 draft · 5 stale)
- PRs needing author action: **25**
- Total unresolved comments: **131**

## Scoreboard
_Sort: unresolved-comments desc → needs-action desc → ready-for-review desc. Click any number to jump to the specific PRs it covers._

| Author | Open | Clean | CI failing | Unresolved Comments | Changes Requested | Deferred | Draft | Stale | Needs action | Total Unresolved Comments | Ready for Review |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| [@bfoss765](#bfoss765) | [11](#bfoss765-open) | [1](#bfoss765-clean) | — | [10](#bfoss765-unresolved-comments) | — | — | — | — | [9](#bfoss765-needs-action) | [37](#bfoss765-unresolved-comments) | — |
| [@PastaPastaPasta + (@thepastaclaw)](#pastapastapasta) | [5+(6)](#pastapastapasta-open) | [0+(3)](#pastapastapasta-clean) | — | [4+(1)](#pastapastapasta-unresolved-comments) | — | — | [1+(2)](#pastapastapasta-draft) | — | [4+(0)](#pastapastapasta-needs-action) | [10+(4)](#pastapastapasta-unresolved-comments) | — |
| [@shumkov](#shumkov) | [7](#shumkov-open) | — | — | [3](#shumkov-unresolved-comments) | — | [2](#shumkov-deferred) | [1](#shumkov-draft) | [1](#shumkov-stale) | [3](#shumkov-needs-action) | [12](#shumkov-unresolved-comments) | [2](#shumkov-ready-for-review) |
| [@llbartekll](#llbartekll) | [3](#llbartekll-open) | — | — | [2](#llbartekll-unresolved-comments) | — | — | [1](#llbartekll-draft) | — | [2](#llbartekll-needs-action) | [5](#llbartekll-unresolved-comments) | [1](#llbartekll-ready-for-review) |
| [@lklimek + (@Claudius-Maginificent)](#lklimek) | [5+(4)](#lklimek-open) | — | — | [0+(2)](#lklimek-unresolved-comments) | — | [5+(0)](#lklimek-deferred) | [0+(1)](#lklimek-draft) | [0+(1)](#lklimek-stale) | [0+(2)](#lklimek-needs-action) | [0+(41)](#lklimek-unresolved-comments) | — |
| [@QuantumExplorer](#quantumexplorer) | [8](#quantumexplorer-open) | — | [1](#quantumexplorer-ci-failing) | [1](#quantumexplorer-unresolved-comments) | [1](#quantumexplorer-changes-requested) | [3](#quantumexplorer-deferred) | — | [2](#quantumexplorer-stale) | [2](#quantumexplorer-needs-action) | [5](#quantumexplorer-unresolved-comments) | [1](#quantumexplorer-ready-for-review) |
| [@ZocoLini](#zocolini) | [1](#zocolini-open) | — | — | [1](#zocolini-unresolved-comments) | — | — | — | — | [1](#zocolini-needs-action) | [8](#zocolini-unresolved-comments) | — |
| [@infraclaw-dash](#infraclaw-dash) | [1](#infraclaw-dash-open) | — | — | [1](#infraclaw-dash-unresolved-comments) | — | — | — | — | [1](#infraclaw-dash-needs-action) | [2](#infraclaw-dash-unresolved-comments) | — |
| [@vivekgsharma](#vivekgsharma) | [1](#vivekgsharma-open) | — | — | [1](#vivekgsharma-unresolved-comments) | — | — | — | — | [1](#vivekgsharma-needs-action) | [2](#vivekgsharma-unresolved-comments) | — |
| [@pshenmic](#pshenmic) | [1](#pshenmic-open) | — | — | — | — | — | — | [1](#pshenmic-stale) | — | [5](#pshenmic-unresolved-comments) | — |
| [@ogabrielides](#ogabrielides) | [1](#ogabrielides-open) | — | — | — | — | [1](#ogabrielides-deferred) | — | — | — | — | — |

## Per-author detail

<a id="bfoss765"></a>
### @bfoss765
<a id="bfoss765-open"></a>
#### Open (11)
- [#4195 feat(platform-wallet): allocate txMetadata encryptionKeyIndex in Rust, not the host (stacked on #4186)](https://github.com/dashpay/platform/pull/4195) — 10 unresolved (10 human) · 3 days stale · 🔴 CI failing
  - Top thread: "<!-- thepastaclaw-review v1 finding=d6f036f0e17a dedupe=d55b1e77ca845fdc -->" — 3 days old
- [#4184 fix: shield asset-lock funding from all funds accounts incl. CoinJoin (#4073)](https://github.com/dashpay/platform/pull/4184) — 3 unresolved (3 human) · 3 days stale · ✋ changes requested
  - Top thread: "<!-- thepastaclaw-review v1 finding=f015408c624a dedupe=8f314b58221e4f8c -->" — 3 days old
- [#4204 feat(kotlin-sdk): one-time Orchard key shielded-invite API (inviter + claim)](https://github.com/dashpay/platform/pull/4204) — 7 unresolved (2 CodeRabbit, 5 human) · 3 days stale
  - Top thread: "<!-- thepastaclaw-review v1 finding=cc2b890af356 dedupe=864a088719a9b104 -->" — 3 days old
- [#4186 feat(kotlin-sdk): wire-compatible encrypted txMetadata document create + decrypt-on-fetch](https://github.com/dashpay/platform/pull/4186) — 5 unresolved (5 human) · 3 days stale · 🔴 CI failing
  - Top thread: "<!-- thepastaclaw-review v1 finding=02bc78a3519f dedupe=9a959dd67ee262d6 -->" — 3 days old
- [#4193 ci(kotlin-sdk): couple Maven Central deploy to the kotlin-sdk-v* release tag (stacked on #4182)](https://github.com/dashpay/platform/pull/4193) — 3 unresolved (3 human) · 4 days stale
  - Top thread: "<!-- thepastaclaw-review v1 finding=551eab54463b dedupe=fd6492031a581ba7 -->" — 4 days old
- [#4196 fix(platform-wallet): age-guard the V2 finalized-transaction handle broadcast (stacked on #4185)](https://github.com/dashpay/platform/pull/4196) — 3 unresolved (3 human) · 3 days stale · ⚠ merge conflict · 🔴 CI failing
  - Top thread: "<!-- thepastaclaw-review v1 finding=8c93f86ba99d dedupe=1a072494405ae03d -->" — 3 days old
- [#4185 feat(kotlin-sdk): split build/broadcast with reservation release for BIP70-style deferred submission](https://github.com/dashpay/platform/pull/4185) — 2 unresolved (2 human) · 3 days stale
  - Top thread: "<!-- thepastaclaw-review v1 finding=f37f8de736e4 dedupe=f44ae14fc273d813 -->" — 3 days old
- [#4183 feat(kotlin-sdk)!: keystore rework — policy-alias split, layered key recovery, durable repair, structured signer errors (stacked on #4191)](https://github.com/dashpay/platform/pull/4183) — 2 unresolved (2 human) · 3 days stale
  - Top thread: "<!-- thepastaclaw-review v1 finding=4efa67555719 dedupe=e8ff87653dae49fe -->" — 3 days old
- [#4194 feat(swift-sdk): wrappers for the txMetadata encrypted-document FFI exports (stacked on #4186)](https://github.com/dashpay/platform/pull/4194) — 1 unresolved (1 human) · 3 days stale · 🔴 CI failing
  - Top thread: "<!-- thepastaclaw-review v1 finding=158e033ecf95 dedupe=388d8b432b12c1fc -->" — 3 days old
- [#4220 docs(kotlin-sdk): maven-central environment infra runbook (infra half of #4193)](https://github.com/dashpay/platform/pull/4220) — 1 unresolved (1 human) · 3 days stale
  - Top thread: "<!-- thepastaclaw-review v1 finding=e9a109a75a41 dedupe=0dde21465b405aae -->" — 3 days old
- [#4191 fix(kotlin-sdk): unmanaged-identity reads return absence + typed SigningKeyUnavailable (split from #4183)](https://github.com/dashpay/platform/pull/4191)

<a id="bfoss765-needs-action"></a>
#### Needs action (9)
- [#4195 feat(platform-wallet): allocate txMetadata encryptionKeyIndex in Rust, not the host (stacked on #4186)](https://github.com/dashpay/platform/pull/4195) — 10 unresolved (10 human) · 3 days stale · 🔴 CI failing
  - Top thread: "<!-- thepastaclaw-review v1 finding=d6f036f0e17a dedupe=d55b1e77ca845fdc -->" — 3 days old
- [#4184 fix: shield asset-lock funding from all funds accounts incl. CoinJoin (#4073)](https://github.com/dashpay/platform/pull/4184) — 3 unresolved (3 human) · 3 days stale · ✋ changes requested
  - Top thread: "<!-- thepastaclaw-review v1 finding=f015408c624a dedupe=8f314b58221e4f8c -->" — 3 days old
- [#4204 feat(kotlin-sdk): one-time Orchard key shielded-invite API (inviter + claim)](https://github.com/dashpay/platform/pull/4204) — 7 unresolved (2 CodeRabbit, 5 human) · 3 days stale
  - Top thread: "<!-- thepastaclaw-review v1 finding=cc2b890af356 dedupe=864a088719a9b104 -->" — 3 days old
- [#4186 feat(kotlin-sdk): wire-compatible encrypted txMetadata document create + decrypt-on-fetch](https://github.com/dashpay/platform/pull/4186) — 5 unresolved (5 human) · 3 days stale · 🔴 CI failing
  - Top thread: "<!-- thepastaclaw-review v1 finding=02bc78a3519f dedupe=9a959dd67ee262d6 -->" — 3 days old
- [#4193 ci(kotlin-sdk): couple Maven Central deploy to the kotlin-sdk-v* release tag (stacked on #4182)](https://github.com/dashpay/platform/pull/4193) — 3 unresolved (3 human) · 4 days stale
  - Top thread: "<!-- thepastaclaw-review v1 finding=551eab54463b dedupe=fd6492031a581ba7 -->" — 4 days old
- [#4196 fix(platform-wallet): age-guard the V2 finalized-transaction handle broadcast (stacked on #4185)](https://github.com/dashpay/platform/pull/4196) — 3 unresolved (3 human) · 3 days stale · ⚠ merge conflict · 🔴 CI failing
  - Top thread: "<!-- thepastaclaw-review v1 finding=8c93f86ba99d dedupe=1a072494405ae03d -->" — 3 days old
- [#4185 feat(kotlin-sdk): split build/broadcast with reservation release for BIP70-style deferred submission](https://github.com/dashpay/platform/pull/4185) — 2 unresolved (2 human) · 3 days stale
  - Top thread: "<!-- thepastaclaw-review v1 finding=f37f8de736e4 dedupe=f44ae14fc273d813 -->" — 3 days old
- [#4194 feat(swift-sdk): wrappers for the txMetadata encrypted-document FFI exports (stacked on #4186)](https://github.com/dashpay/platform/pull/4194) — 1 unresolved (1 human) · 3 days stale · 🔴 CI failing
  - Top thread: "<!-- thepastaclaw-review v1 finding=158e033ecf95 dedupe=388d8b432b12c1fc -->" — 3 days old
- [#4220 docs(kotlin-sdk): maven-central environment infra runbook (infra half of #4193)](https://github.com/dashpay/platform/pull/4220) — 1 unresolved (1 human) · 3 days stale
  - Top thread: "<!-- thepastaclaw-review v1 finding=e9a109a75a41 dedupe=0dde21465b405aae -->" — 3 days old

<a id="bfoss765-unresolved-comments"></a>
#### Unresolved Comments (10)
- [#4195 feat(platform-wallet): allocate txMetadata encryptionKeyIndex in Rust, not the host (stacked on #4186)](https://github.com/dashpay/platform/pull/4195) — 10 unresolved (10 human) · 3 days stale · 🔴 CI failing
  - Top thread: "<!-- thepastaclaw-review v1 finding=d6f036f0e17a dedupe=d55b1e77ca845fdc -->" — 3 days old
- [#4184 fix: shield asset-lock funding from all funds accounts incl. CoinJoin (#4073)](https://github.com/dashpay/platform/pull/4184) — 3 unresolved (3 human) · 3 days stale · ✋ changes requested
  - Top thread: "<!-- thepastaclaw-review v1 finding=f015408c624a dedupe=8f314b58221e4f8c -->" — 3 days old
- [#4204 feat(kotlin-sdk): one-time Orchard key shielded-invite API (inviter + claim)](https://github.com/dashpay/platform/pull/4204) — 7 unresolved (2 CodeRabbit, 5 human) · 3 days stale
  - Top thread: "<!-- thepastaclaw-review v1 finding=cc2b890af356 dedupe=864a088719a9b104 -->" — 3 days old
- [#4186 feat(kotlin-sdk): wire-compatible encrypted txMetadata document create + decrypt-on-fetch](https://github.com/dashpay/platform/pull/4186) — 5 unresolved (5 human) · 3 days stale · 🔴 CI failing
  - Top thread: "<!-- thepastaclaw-review v1 finding=02bc78a3519f dedupe=9a959dd67ee262d6 -->" — 3 days old
- [#4193 ci(kotlin-sdk): couple Maven Central deploy to the kotlin-sdk-v* release tag (stacked on #4182)](https://github.com/dashpay/platform/pull/4193) — 3 unresolved (3 human) · 4 days stale
  - Top thread: "<!-- thepastaclaw-review v1 finding=551eab54463b dedupe=fd6492031a581ba7 -->" — 4 days old
- [#4196 fix(platform-wallet): age-guard the V2 finalized-transaction handle broadcast (stacked on #4185)](https://github.com/dashpay/platform/pull/4196) — 3 unresolved (3 human) · 3 days stale · ⚠ merge conflict · 🔴 CI failing
  - Top thread: "<!-- thepastaclaw-review v1 finding=8c93f86ba99d dedupe=1a072494405ae03d -->" — 3 days old
- [#4185 feat(kotlin-sdk): split build/broadcast with reservation release for BIP70-style deferred submission](https://github.com/dashpay/platform/pull/4185) — 2 unresolved (2 human) · 3 days stale
  - Top thread: "<!-- thepastaclaw-review v1 finding=f37f8de736e4 dedupe=f44ae14fc273d813 -->" — 3 days old
- [#4183 feat(kotlin-sdk)!: keystore rework — policy-alias split, layered key recovery, durable repair, structured signer errors (stacked on #4191)](https://github.com/dashpay/platform/pull/4183) — 2 unresolved (2 human) · 3 days stale
  - Top thread: "<!-- thepastaclaw-review v1 finding=4efa67555719 dedupe=e8ff87653dae49fe -->" — 3 days old
- [#4194 feat(swift-sdk): wrappers for the txMetadata encrypted-document FFI exports (stacked on #4186)](https://github.com/dashpay/platform/pull/4194) — 1 unresolved (1 human) · 3 days stale · 🔴 CI failing
  - Top thread: "<!-- thepastaclaw-review v1 finding=158e033ecf95 dedupe=388d8b432b12c1fc -->" — 3 days old
- [#4220 docs(kotlin-sdk): maven-central environment infra runbook (infra half of #4193)](https://github.com/dashpay/platform/pull/4220) — 1 unresolved (1 human) · 3 days stale
  - Top thread: "<!-- thepastaclaw-review v1 finding=e9a109a75a41 dedupe=0dde21465b405aae -->" — 3 days old

<a id="bfoss765-clean"></a>
#### Clean (1)
- [#4191 fix(kotlin-sdk): unmanaged-identity reads return absence + typed SigningKeyUnavailable (split from #4183)](https://github.com/dashpay/platform/pull/4191)

<a id="pastapastapasta"></a>
### @PastaPastaPasta + (@thepastaclaw)
<a id="pastapastapasta-open"></a>
#### Open (11)
- [#3096 feat(sdk): add client-side validation to state transition construction methods](https://github.com/dashpay/platform/pull/3096) — via @thepastaclaw · 4 unresolved (2 CodeRabbit, 2 human) · 159 days stale
  - Top thread: "_⚠️ Potential issue_ \| _🟡 Minor_" — 156 days old
- [#3680 refactor(dapi,dpp)!: move dapi-client and Identifier off Buffer to Uint8Array](https://github.com/dashpay/platform/pull/3680) — 5 unresolved (5 human) · 61 days stale · ⚠ merge conflict · 🔴 CI failing
  - Top thread: "<!-- thepastaclaw-review v1 finding=1c101f507542 -->" — 61 days old
- [#3462 fix(dpp)!: version-gate distribution function floating-point evaluation](https://github.com/dashpay/platform/pull/3462) — 3 unresolved (3 human) · 55 days stale · 🔴 CI failing
  - Top thread: "<!-- thepastaclaw-review v1 finding=7d40971bd5ff dedupe=2846c55b07c76472 -->" — 55 days old
- [#3509 fix(platform): default omitted proved query limits](https://github.com/dashpay/platform/pull/3509) — 1 unresolved (1 human) · 85 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing
  - Top thread: "<!-- thepastaclaw-review v1 finding=d3df3e197206 -->" — 85 days old
- [#2988 feat(wasm-sdk): auto-generate entropy for document creation when not provided](https://github.com/dashpay/platform/pull/2988) — 1 unresolved (1 human) · 66 days stale · 🔴 CI failing
  - Top thread: "<!-- thepastaclaw-review v1 finding=30cbefe1c9ef -->" — 66 days old
- [#3635 ci: tolerate book preview comment permission errors](https://github.com/dashpay/platform/pull/3635) — via @thepastaclaw
- [#3689 refactor(sdk,wallet-lib,test-suite)!: convert dapi-client + wallet-lib + js-dash-sdk + platform-test-suite to ESM](https://github.com/dashpay/platform/pull/3689) — ⚠ merge conflict · 📝 draft
- [#3898 fix(dashmate): re-sync stale 3.x Drive and rs-dapi images](https://github.com/dashpay/platform/pull/3898) — via @thepastaclaw
- [#4015 fix(swift-example-app): gate identity resumes by funding type](https://github.com/dashpay/platform/pull/4015) — via @thepastaclaw
- [#4016 fix(platform-wallet): close asset-lock resume broadcast race](https://github.com/dashpay/platform/pull/4016) — via @thepastaclaw · ⚠ merge conflict · 📝 draft
- [#4136 fix(dashmate): use live Tenderdash app version for protocol status](https://github.com/dashpay/platform/pull/4136) — via @thepastaclaw · 📝 draft

<a id="pastapastapasta-needs-action"></a>
#### Needs action (4)
- [#3680 refactor(dapi,dpp)!: move dapi-client and Identifier off Buffer to Uint8Array](https://github.com/dashpay/platform/pull/3680) — 5 unresolved (5 human) · 61 days stale · ⚠ merge conflict · 🔴 CI failing
  - Top thread: "<!-- thepastaclaw-review v1 finding=1c101f507542 -->" — 61 days old
- [#3462 fix(dpp)!: version-gate distribution function floating-point evaluation](https://github.com/dashpay/platform/pull/3462) — 3 unresolved (3 human) · 55 days stale · 🔴 CI failing
  - Top thread: "<!-- thepastaclaw-review v1 finding=7d40971bd5ff dedupe=2846c55b07c76472 -->" — 55 days old
- [#3509 fix(platform): default omitted proved query limits](https://github.com/dashpay/platform/pull/3509) — 1 unresolved (1 human) · 85 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing
  - Top thread: "<!-- thepastaclaw-review v1 finding=d3df3e197206 -->" — 85 days old
- [#2988 feat(wasm-sdk): auto-generate entropy for document creation when not provided](https://github.com/dashpay/platform/pull/2988) — 1 unresolved (1 human) · 66 days stale · 🔴 CI failing
  - Top thread: "<!-- thepastaclaw-review v1 finding=30cbefe1c9ef -->" — 66 days old

<a id="pastapastapasta-unresolved-comments"></a>
#### Unresolved Comments (5)
- [#3096 feat(sdk): add client-side validation to state transition construction methods](https://github.com/dashpay/platform/pull/3096) — via @thepastaclaw · 4 unresolved (2 CodeRabbit, 2 human) · 159 days stale
  - Top thread: "_⚠️ Potential issue_ \| _🟡 Minor_" — 156 days old
- [#3680 refactor(dapi,dpp)!: move dapi-client and Identifier off Buffer to Uint8Array](https://github.com/dashpay/platform/pull/3680) — 5 unresolved (5 human) · 61 days stale · ⚠ merge conflict · 🔴 CI failing
  - Top thread: "<!-- thepastaclaw-review v1 finding=1c101f507542 -->" — 61 days old
- [#3462 fix(dpp)!: version-gate distribution function floating-point evaluation](https://github.com/dashpay/platform/pull/3462) — 3 unresolved (3 human) · 55 days stale · 🔴 CI failing
  - Top thread: "<!-- thepastaclaw-review v1 finding=7d40971bd5ff dedupe=2846c55b07c76472 -->" — 55 days old
- [#3509 fix(platform): default omitted proved query limits](https://github.com/dashpay/platform/pull/3509) — 1 unresolved (1 human) · 85 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing
  - Top thread: "<!-- thepastaclaw-review v1 finding=d3df3e197206 -->" — 85 days old
- [#2988 feat(wasm-sdk): auto-generate entropy for document creation when not provided](https://github.com/dashpay/platform/pull/2988) — 1 unresolved (1 human) · 66 days stale · 🔴 CI failing
  - Top thread: "<!-- thepastaclaw-review v1 finding=30cbefe1c9ef -->" — 66 days old

<a id="pastapastapasta-draft"></a>
#### Draft (3)
- [#3689 refactor(sdk,wallet-lib,test-suite)!: convert dapi-client + wallet-lib + js-dash-sdk + platform-test-suite to ESM](https://github.com/dashpay/platform/pull/3689) — ⚠ merge conflict · 📝 draft
- [#4016 fix(platform-wallet): close asset-lock resume broadcast race](https://github.com/dashpay/platform/pull/4016) — via @thepastaclaw · ⚠ merge conflict · 📝 draft
- [#4136 fix(dashmate): use live Tenderdash app version for protocol status](https://github.com/dashpay/platform/pull/4136) — via @thepastaclaw · 📝 draft

<a id="pastapastapasta-clean"></a>
#### Clean (3)
- [#3635 ci: tolerate book preview comment permission errors](https://github.com/dashpay/platform/pull/3635) — via @thepastaclaw
- [#3898 fix(dashmate): re-sync stale 3.x Drive and rs-dapi images](https://github.com/dashpay/platform/pull/3898) — via @thepastaclaw
- [#4015 fix(swift-example-app): gate identity resumes by funding type](https://github.com/dashpay/platform/pull/4015) — via @thepastaclaw

<a id="shumkov"></a>
### @shumkov
<a id="shumkov-open"></a>
#### Open (7)
- [#4228 ci: release Kotlin and Swift SDKs with the platform release](https://github.com/dashpay/platform/pull/4228) — 7 unresolved (3 CodeRabbit, 4 human) · 2 days stale · ✋ changes requested · 🔴 CI failing
  - Top thread: "<!-- thepastaclaw-review v1 finding=1c08f22582e0 dedupe=d76a94ed65ca0aba -->" — 2 days old
- [#3936 chore(drive-abci): update to nested address in SML](https://github.com/dashpay/platform/pull/3936) — 2 unresolved (2 human) · 38 days stale · ⚠ merge conflict · 📝 draft
  - Top thread: "<!-- thepastaclaw-review v1 finding=2e1950367681 dedupe=494e3310440bc19a -->" — 38 days old
- [#4229 fix(release): base changelog on the immediately-preceding release](https://github.com/dashpay/platform/pull/4229) — 2 unresolved (2 human) · 2 days stale · ✋ changes requested · 🔴 CI failing
  - Top thread: "<!-- thepastaclaw-review v1 finding=e00e58b65b7a dedupe=cf68de8bf3e413ce -->" — 2 days old
- [#4214 fix(test-suite): stabilize platform e2e tests](https://github.com/dashpay/platform/pull/4214) — 1 unresolved (1 human) · 2 days stale · ✋ changes requested
  - Top thread: "<!-- thepastaclaw-review v1 finding=b6714334b2e7 dedupe=6b54ff97f7218bda -->" — 2 days old
- [#2392 feat(dashmate): single node local network [WIP\]](https://github.com/dashpay/platform/pull/2392) — ⏸ deferred
- [#2518 feat: token marketplace](https://github.com/dashpay/platform/pull/2518) — 📝 draft · ⏸ deferred
- [#4235 fix(dashmate): migrate Drive and DAPI images onto the prerelease line](https://github.com/dashpay/platform/pull/4235) — 🔴 CI failing · 🐢 targets v4.1-dev

<a id="shumkov-needs-action"></a>
#### Needs action (3)
- [#4228 ci: release Kotlin and Swift SDKs with the platform release](https://github.com/dashpay/platform/pull/4228) — 7 unresolved (3 CodeRabbit, 4 human) · 2 days stale · ✋ changes requested · 🔴 CI failing
  - Top thread: "<!-- thepastaclaw-review v1 finding=1c08f22582e0 dedupe=d76a94ed65ca0aba -->" — 2 days old
- [#4229 fix(release): base changelog on the immediately-preceding release](https://github.com/dashpay/platform/pull/4229) — 2 unresolved (2 human) · 2 days stale · ✋ changes requested · 🔴 CI failing
  - Top thread: "<!-- thepastaclaw-review v1 finding=e00e58b65b7a dedupe=cf68de8bf3e413ce -->" — 2 days old
- [#4214 fix(test-suite): stabilize platform e2e tests](https://github.com/dashpay/platform/pull/4214) — 1 unresolved (1 human) · 2 days stale · ✋ changes requested
  - Top thread: "<!-- thepastaclaw-review v1 finding=b6714334b2e7 dedupe=6b54ff97f7218bda -->" — 2 days old

<a id="shumkov-unresolved-comments"></a>
#### Unresolved Comments (3)
- [#4228 ci: release Kotlin and Swift SDKs with the platform release](https://github.com/dashpay/platform/pull/4228) — 7 unresolved (3 CodeRabbit, 4 human) · 2 days stale · ✋ changes requested · 🔴 CI failing
  - Top thread: "<!-- thepastaclaw-review v1 finding=1c08f22582e0 dedupe=d76a94ed65ca0aba -->" — 2 days old
- [#4229 fix(release): base changelog on the immediately-preceding release](https://github.com/dashpay/platform/pull/4229) — 2 unresolved (2 human) · 2 days stale · ✋ changes requested · 🔴 CI failing
  - Top thread: "<!-- thepastaclaw-review v1 finding=e00e58b65b7a dedupe=cf68de8bf3e413ce -->" — 2 days old
- [#4214 fix(test-suite): stabilize platform e2e tests](https://github.com/dashpay/platform/pull/4214) — 1 unresolved (1 human) · 2 days stale · ✋ changes requested
  - Top thread: "<!-- thepastaclaw-review v1 finding=b6714334b2e7 dedupe=6b54ff97f7218bda -->" — 2 days old

<a id="shumkov-deferred"></a>
#### Deferred (2)
- [#2392 feat(dashmate): single node local network [WIP\]](https://github.com/dashpay/platform/pull/2392) — ⏸ deferred
- [#2518 feat: token marketplace](https://github.com/dashpay/platform/pull/2518) — 📝 draft · ⏸ deferred

<a id="shumkov-draft"></a>
#### Draft (1)
- [#3936 chore(drive-abci): update to nested address in SML](https://github.com/dashpay/platform/pull/3936) — 2 unresolved (2 human) · 38 days stale · ⚠ merge conflict · 📝 draft
  - Top thread: "<!-- thepastaclaw-review v1 finding=2e1950367681 dedupe=494e3310440bc19a -->" — 38 days old

<a id="shumkov-stale"></a>
#### Stale (1)
- [#4235 fix(dashmate): migrate Drive and DAPI images onto the prerelease line](https://github.com/dashpay/platform/pull/4235) — 🔴 CI failing · 🐢 targets v4.1-dev

<a id="shumkov-ready-for-review"></a>
#### Ready for Review (2)
- [#3898 fix(dashmate): re-sync stale 3.x Drive and rs-dapi images](https://github.com/dashpay/platform/pull/3898) — by @thepastaclaw
- [#4191 fix(kotlin-sdk): unmanaged-identity reads return absence + typed SigningKeyUnavailable (split from #4183)](https://github.com/dashpay/platform/pull/4191) — by @bfoss765

<a id="llbartekll"></a>
### @llbartekll
<a id="llbartekll-open"></a>
#### Open (3)
- [#3560 test(swift-sdk): add testnet identity-discovery UI test](https://github.com/dashpay/platform/pull/3560) — 2 unresolved (2 human) · 55 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing
  - Top thread: "<!-- thepastaclaw-review v1 finding=379c84ed00e1 dedupe=08a70e07f6e9ae45 -->" — 55 days old
- [#3694 fix(swift-example-app): unhide Create Identity submit button and auto-dismiss sheet on success](https://github.com/dashpay/platform/pull/3694) — 1 unresolved (1 human) · 55 days stale · ✋ changes requested
  - Top thread: "<!-- thepastaclaw-review v1 finding=e3a849e745d0 dedupe=d76d5740c5bc0f94 -->" — 55 days old
- [#4170 feat(swift-sdk): make wallet deletion asynchronous](https://github.com/dashpay/platform/pull/4170) — 2 unresolved (2 human) · 6 days stale · ✋ changes requested · 📝 draft
  - Top thread: "<!-- thepastaclaw-review v1 finding=39eeb5416a91 dedupe=a66ec193a2ba712e -->" — 6 days old

<a id="llbartekll-needs-action"></a>
#### Needs action (2)
- [#3560 test(swift-sdk): add testnet identity-discovery UI test](https://github.com/dashpay/platform/pull/3560) — 2 unresolved (2 human) · 55 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing
  - Top thread: "<!-- thepastaclaw-review v1 finding=379c84ed00e1 dedupe=08a70e07f6e9ae45 -->" — 55 days old
- [#3694 fix(swift-example-app): unhide Create Identity submit button and auto-dismiss sheet on success](https://github.com/dashpay/platform/pull/3694) — 1 unresolved (1 human) · 55 days stale · ✋ changes requested
  - Top thread: "<!-- thepastaclaw-review v1 finding=e3a849e745d0 dedupe=d76d5740c5bc0f94 -->" — 55 days old

<a id="llbartekll-unresolved-comments"></a>
#### Unresolved Comments (2)
- [#3560 test(swift-sdk): add testnet identity-discovery UI test](https://github.com/dashpay/platform/pull/3560) — 2 unresolved (2 human) · 55 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing
  - Top thread: "<!-- thepastaclaw-review v1 finding=379c84ed00e1 dedupe=08a70e07f6e9ae45 -->" — 55 days old
- [#3694 fix(swift-example-app): unhide Create Identity submit button and auto-dismiss sheet on success](https://github.com/dashpay/platform/pull/3694) — 1 unresolved (1 human) · 55 days stale · ✋ changes requested
  - Top thread: "<!-- thepastaclaw-review v1 finding=e3a849e745d0 dedupe=d76d5740c5bc0f94 -->" — 55 days old

<a id="llbartekll-draft"></a>
#### Draft (1)
- [#4170 feat(swift-sdk): make wallet deletion asynchronous](https://github.com/dashpay/platform/pull/4170) — 2 unresolved (2 human) · 6 days stale · ✋ changes requested · 📝 draft
  - Top thread: "<!-- thepastaclaw-review v1 finding=39eeb5416a91 dedupe=a66ec193a2ba712e -->" — 6 days old

<a id="llbartekll-ready-for-review"></a>
#### Ready for Review (1)
- [#4015 fix(swift-example-app): gate identity resumes by funding type](https://github.com/dashpay/platform/pull/4015) — by @thepastaclaw

<a id="lklimek"></a>
### @lklimek + (@Claudius-Maginificent)
<a id="lklimek-open"></a>
#### Open (9)
- [#3549 test(platform-wallet): e2e framework + full test suite — triage pins, Found-*/PA-* guards, fail-closed persist, Stage-2 merge](https://github.com/dashpay/platform/pull/3549) — via @Claudius-Maginificent · 20 unresolved (20 human) · 85 days stale · 📝 draft
  - Top thread: "<!-- thepastaclaw-review v1 finding=a3725e9d50ca -->" — 85 days old
- [#3968 feat(platform-wallet-storage): embeddable SQLite persistence backend with seedless rehydration](https://github.com/dashpay/platform/pull/3968) — via @Claudius-Maginificent · 11 unresolved (11 human) · 14 days stale · ✋ changes requested · 🔴 CI failing
  - Top thread: "Vault durability: failed parent-dir fsync still returns `Ok`.** After the atomic `persist()` rename, a failed directory …" — 14 days old
- [#3992 feat(platform-wallet): manifest integrity checksum (Risk-6/R12.5 follow-up)](https://github.com/dashpay/platform/pull/3992) — via @Claudius-Maginificent · 7 unresolved (7 human) · 23 days stale · ⚠ merge conflict · ✋ changes requested · 🐢 targets feat/platform-wallet-storage-rehydration
  - Top thread: "<!-- thepastaclaw-review v1 finding=3969d61c562f dedupe=57a8bf7151c0879e -->" — 23 days old
- [#3954 feat(platform-wallet)!: shared ThreadRegistry for coordinator lifecycle + shutdown UAF/data-loss fixes](https://github.com/dashpay/platform/pull/3954) — via @Claudius-Maginificent · 3 unresolved (3 human) · 6 days stale · 🔴 CI failing
  - Top thread: "<!-- thepastaclaw-review v1 finding=bb45a50a6995 dedupe=a8a93d43b67d79b9 -->" — 6 days old
- [#2679 fix(drive-abci): don't panic on unsupported version error](https://github.com/dashpay/platform/pull/2679) — ⏸ deferred
- [#2795 feat(platform): add real-time platform event subscriptions via gRPC streaming](https://github.com/dashpay/platform/pull/2795) — ⏸ deferred
- [#2993 feat: identity reference validation](https://github.com/dashpay/platform/pull/2993) — 📝 draft · ⏸ deferred
- [#3009 feat: contract reference validation](https://github.com/dashpay/platform/pull/3009) — 📝 draft · ⏸ deferred
- [#3032 feat: document creation restriction by group membership](https://github.com/dashpay/platform/pull/3032) — 📝 draft · ⏸ deferred

<a id="lklimek-needs-action"></a>
#### Needs action (2)
- [#3968 feat(platform-wallet-storage): embeddable SQLite persistence backend with seedless rehydration](https://github.com/dashpay/platform/pull/3968) — via @Claudius-Maginificent · 11 unresolved (11 human) · 14 days stale · ✋ changes requested · 🔴 CI failing
  - Top thread: "Vault durability: failed parent-dir fsync still returns `Ok`.** After the atomic `persist()` rename, a failed directory …" — 14 days old
- [#3954 feat(platform-wallet)!: shared ThreadRegistry for coordinator lifecycle + shutdown UAF/data-loss fixes](https://github.com/dashpay/platform/pull/3954) — via @Claudius-Maginificent · 3 unresolved (3 human) · 6 days stale · 🔴 CI failing
  - Top thread: "<!-- thepastaclaw-review v1 finding=bb45a50a6995 dedupe=a8a93d43b67d79b9 -->" — 6 days old

<a id="lklimek-unresolved-comments"></a>
#### Unresolved Comments (2)
- [#3968 feat(platform-wallet-storage): embeddable SQLite persistence backend with seedless rehydration](https://github.com/dashpay/platform/pull/3968) — via @Claudius-Maginificent · 11 unresolved (11 human) · 14 days stale · ✋ changes requested · 🔴 CI failing
  - Top thread: "Vault durability: failed parent-dir fsync still returns `Ok`.** After the atomic `persist()` rename, a failed directory …" — 14 days old
- [#3954 feat(platform-wallet)!: shared ThreadRegistry for coordinator lifecycle + shutdown UAF/data-loss fixes](https://github.com/dashpay/platform/pull/3954) — via @Claudius-Maginificent · 3 unresolved (3 human) · 6 days stale · 🔴 CI failing
  - Top thread: "<!-- thepastaclaw-review v1 finding=bb45a50a6995 dedupe=a8a93d43b67d79b9 -->" — 6 days old

<a id="lklimek-deferred"></a>
#### Deferred (5)
- [#2679 fix(drive-abci): don't panic on unsupported version error](https://github.com/dashpay/platform/pull/2679) — ⏸ deferred
- [#2795 feat(platform): add real-time platform event subscriptions via gRPC streaming](https://github.com/dashpay/platform/pull/2795) — ⏸ deferred
- [#2993 feat: identity reference validation](https://github.com/dashpay/platform/pull/2993) — 📝 draft · ⏸ deferred
- [#3009 feat: contract reference validation](https://github.com/dashpay/platform/pull/3009) — 📝 draft · ⏸ deferred
- [#3032 feat: document creation restriction by group membership](https://github.com/dashpay/platform/pull/3032) — 📝 draft · ⏸ deferred

<a id="lklimek-draft"></a>
#### Draft (1)
- [#3549 test(platform-wallet): e2e framework + full test suite — triage pins, Found-*/PA-* guards, fail-closed persist, Stage-2 merge](https://github.com/dashpay/platform/pull/3549) — via @Claudius-Maginificent · 20 unresolved (20 human) · 85 days stale · 📝 draft
  - Top thread: "<!-- thepastaclaw-review v1 finding=a3725e9d50ca -->" — 85 days old

<a id="lklimek-stale"></a>
#### Stale (1)
- [#3992 feat(platform-wallet): manifest integrity checksum (Risk-6/R12.5 follow-up)](https://github.com/dashpay/platform/pull/3992) — via @Claudius-Maginificent · 7 unresolved (7 human) · 23 days stale · ⚠ merge conflict · ✋ changes requested · 🐢 targets feat/platform-wallet-storage-rehydration
  - Top thread: "<!-- thepastaclaw-review v1 finding=3969d61c562f dedupe=57a8bf7151c0879e -->" — 23 days old

<a id="quantumexplorer"></a>
### @QuantumExplorer
<a id="quantumexplorer-open"></a>
#### Open (8)
- [#4231 fix(sdk): restore proved current-epoch fetch with two-step explicit-start query](https://github.com/dashpay/platform/pull/4231) — 3 unresolved (3 human) · 1 days stale · ✋ changes requested · 🔴 CI failing · 🐢 targets v4.1-dev
  - Top thread: "<!-- thepastaclaw-review v1 finding=8abf8a8bb346 dedupe=ca79307d30432343 -->" — 1 days old
- [#4232 fix(drive-abci): refresh the contract cache when the v13 migration rewrites DPNS](https://github.com/dashpay/platform/pull/4232) — 1 unresolved (1 human) · 1 days stale · ✋ changes requested · 🔴 CI failing · 🐢 targets v4.1-dev
  - Top thread: "<!-- thepastaclaw-review v1 finding=9ba873adec79 dedupe=3a54ee184ee2152a -->" — 1 days old
- [#4218 fix(platform): reject contradictory keep-history document deletes](https://github.com/dashpay/platform/pull/4218) — 1 unresolved (1 human) · 3 days stale · 🔴 CI failing
  - Top thread: "<!-- thepastaclaw-review v1 finding=e868506e7d4f dedupe=effb5abb63dcd615 -->" — 3 days old
- [#1834 feat(drive-abci)!: Statesync](https://github.com/dashpay/platform/pull/1834) — 📝 draft · ⏸ deferred
- [#3021 feat(platform): better contract state transitions](https://github.com/dashpay/platform/pull/3021) — 📝 draft · ⏸ deferred
- [#3417 feat(swift-sdk): use SPV-synced quorums for Platform proof verification](https://github.com/dashpay/platform/pull/3417) — ⚠ merge conflict · ✋ changes requested · 🔴 CI failing
- [#3740 feat: add time-range indexes for trending/leaderboard queries](https://github.com/dashpay/platform/pull/3740) — ⏸ deferred
- [#4221 feat(sdk): prefer DAPI peers supporting the client's latest protocol version](https://github.com/dashpay/platform/pull/4221) — 🔴 CI failing

<a id="quantumexplorer-needs-action"></a>
#### Needs action (2)
- [#4218 fix(platform): reject contradictory keep-history document deletes](https://github.com/dashpay/platform/pull/4218) — 1 unresolved (1 human) · 3 days stale · 🔴 CI failing
  - Top thread: "<!-- thepastaclaw-review v1 finding=e868506e7d4f dedupe=effb5abb63dcd615 -->" — 3 days old
- [#3417 feat(swift-sdk): use SPV-synced quorums for Platform proof verification](https://github.com/dashpay/platform/pull/3417) — ⚠ merge conflict · ✋ changes requested · 🔴 CI failing

<a id="quantumexplorer-unresolved-comments"></a>
#### Unresolved Comments (1)
- [#4218 fix(platform): reject contradictory keep-history document deletes](https://github.com/dashpay/platform/pull/4218) — 1 unresolved (1 human) · 3 days stale · 🔴 CI failing
  - Top thread: "<!-- thepastaclaw-review v1 finding=e868506e7d4f dedupe=effb5abb63dcd615 -->" — 3 days old

<a id="quantumexplorer-changes-requested"></a>
#### Changes Requested (1)
- [#3417 feat(swift-sdk): use SPV-synced quorums for Platform proof verification](https://github.com/dashpay/platform/pull/3417) — ⚠ merge conflict · ✋ changes requested · 🔴 CI failing

<a id="quantumexplorer-ci-failing"></a>
#### CI Failing (1)
- [#4221 feat(sdk): prefer DAPI peers supporting the client's latest protocol version](https://github.com/dashpay/platform/pull/4221) — 🔴 CI failing

<a id="quantumexplorer-deferred"></a>
#### Deferred (3)
- [#1834 feat(drive-abci)!: Statesync](https://github.com/dashpay/platform/pull/1834) — 📝 draft · ⏸ deferred
- [#3021 feat(platform): better contract state transitions](https://github.com/dashpay/platform/pull/3021) — 📝 draft · ⏸ deferred
- [#3740 feat: add time-range indexes for trending/leaderboard queries](https://github.com/dashpay/platform/pull/3740) — ⏸ deferred

<a id="quantumexplorer-stale"></a>
#### Stale (2)
- [#4231 fix(sdk): restore proved current-epoch fetch with two-step explicit-start query](https://github.com/dashpay/platform/pull/4231) — 3 unresolved (3 human) · 1 days stale · ✋ changes requested · 🔴 CI failing · 🐢 targets v4.1-dev
  - Top thread: "<!-- thepastaclaw-review v1 finding=8abf8a8bb346 dedupe=ca79307d30432343 -->" — 1 days old
- [#4232 fix(drive-abci): refresh the contract cache when the v13 migration rewrites DPNS](https://github.com/dashpay/platform/pull/4232) — 1 unresolved (1 human) · 1 days stale · ✋ changes requested · 🔴 CI failing · 🐢 targets v4.1-dev
  - Top thread: "<!-- thepastaclaw-review v1 finding=9ba873adec79 dedupe=3a54ee184ee2152a -->" — 1 days old

<a id="quantumexplorer-ready-for-review"></a>
#### Ready for Review (1)
- [#3635 ci: tolerate book preview comment permission errors](https://github.com/dashpay/platform/pull/3635) — by @thepastaclaw

<a id="zocolini"></a>
### @ZocoLini
<a id="zocolini-open"></a>
#### Open (1)
- [#4064 test(swift-sdk): mid flight added wallet integration test](https://github.com/dashpay/platform/pull/4064) — 8 unresolved (2 CodeRabbit, 6 human) · 16 days stale
  - Top thread: "_🩺 Stability & Availability_ \| _🟡 Minor_ \| _⚡ Quick win_" — 16 days old

<a id="zocolini-needs-action"></a>
#### Needs action (1)
- [#4064 test(swift-sdk): mid flight added wallet integration test](https://github.com/dashpay/platform/pull/4064) — 8 unresolved (2 CodeRabbit, 6 human) · 16 days stale
  - Top thread: "_🩺 Stability & Availability_ \| _🟡 Minor_ \| _⚡ Quick win_" — 16 days old

<a id="zocolini-unresolved-comments"></a>
#### Unresolved Comments (1)
- [#4064 test(swift-sdk): mid flight added wallet integration test](https://github.com/dashpay/platform/pull/4064) — 8 unresolved (2 CodeRabbit, 6 human) · 16 days stale
  - Top thread: "_🩺 Stability & Availability_ \| _🟡 Minor_ \| _⚡ Quick win_" — 16 days old

<a id="infraclaw-dash"></a>
### @infraclaw-dash
<a id="infraclaw-dash-open"></a>
#### Open (1)
- [#3958 ci: add Platform testnet sync status reporting](https://github.com/dashpay/platform/pull/3958) — 2 unresolved (2 human) · 33 days stale · ⚠ merge conflict
  - Top thread: "<!-- thepastaclaw-review v1 finding=5ae1197b579a dedupe=6feca8368dcb61f8 -->" — 33 days old

<a id="infraclaw-dash-needs-action"></a>
#### Needs action (1)
- [#3958 ci: add Platform testnet sync status reporting](https://github.com/dashpay/platform/pull/3958) — 2 unresolved (2 human) · 33 days stale · ⚠ merge conflict
  - Top thread: "<!-- thepastaclaw-review v1 finding=5ae1197b579a dedupe=6feca8368dcb61f8 -->" — 33 days old

<a id="infraclaw-dash-unresolved-comments"></a>
#### Unresolved Comments (1)
- [#3958 ci: add Platform testnet sync status reporting](https://github.com/dashpay/platform/pull/3958) — 2 unresolved (2 human) · 33 days stale · ⚠ merge conflict
  - Top thread: "<!-- thepastaclaw-review v1 finding=5ae1197b579a dedupe=6feca8368dcb61f8 -->" — 33 days old

<a id="vivekgsharma"></a>
### @vivekgsharma
<a id="vivekgsharma-open"></a>
#### Open (1)
- [#4189 ci: enforce macOS runner disk reserve](https://github.com/dashpay/platform/pull/4189) — 2 unresolved (1 CodeRabbit, 1 human) · 5 days stale · ✋ changes requested · 🔴 CI failing
  - Top thread: "<!-- thepastaclaw-review v1 finding=cb3ec4ed35e4 dedupe=7c8bd1045d5ae48a -->" — 4 days old

<a id="vivekgsharma-needs-action"></a>
#### Needs action (1)
- [#4189 ci: enforce macOS runner disk reserve](https://github.com/dashpay/platform/pull/4189) — 2 unresolved (1 CodeRabbit, 1 human) · 5 days stale · ✋ changes requested · 🔴 CI failing
  - Top thread: "<!-- thepastaclaw-review v1 finding=cb3ec4ed35e4 dedupe=7c8bd1045d5ae48a -->" — 4 days old

<a id="vivekgsharma-unresolved-comments"></a>
#### Unresolved Comments (1)
- [#4189 ci: enforce macOS runner disk reserve](https://github.com/dashpay/platform/pull/4189) — 2 unresolved (1 CodeRabbit, 1 human) · 5 days stale · ✋ changes requested · 🔴 CI failing
  - Top thread: "<!-- thepastaclaw-review v1 finding=cb3ec4ed35e4 dedupe=7c8bd1045d5ae48a -->" — 4 days old

<a id="pshenmic"></a>
### @pshenmic
<a id="pshenmic-open"></a>
#### Open (1)
- [#2446 feat(js-dapi-client): add contested resources query methods](https://github.com/dashpay/platform/pull/2446) — 5 unresolved (5 CodeRabbit) · 543 days stale · ⚠ merge conflict · 📝 draft · 🐢 targets v2.0-dev, untouched 534 days
  - Top thread: "_:warning: Potential issue_" — 543 days old

<a id="pshenmic-stale"></a>
#### Stale (1)
- [#2446 feat(js-dapi-client): add contested resources query methods](https://github.com/dashpay/platform/pull/2446) — 5 unresolved (5 CodeRabbit) · 543 days stale · ⚠ merge conflict · 📝 draft · 🐢 targets v2.0-dev, untouched 534 days
  - Top thread: "_:warning: Potential issue_" — 543 days old

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

_No history snapshot from last week was found, so week-over-week deltas are omitted this run._
