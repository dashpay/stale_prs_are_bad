---
---
# PR Hygiene Report
*Last updated: 2026-11-27 00:19 UTC · commit 171af2b*

## Summary
- Open PRs: **221** (16 clean · 5 CI failing · 6 changes requested · 39 unresolved comments · 4 deferred · 60 draft · 91 stale)
- PRs needing author action: **48**
- Total unresolved comments: **344**
- dashpay/platform: **124** open (8 clean · 4 CI failing · 2 changes requested · 20 unresolved comments · 4 deferred · 34 draft · 52 stale) · engine: 36 draft · 3 ready-for-human · 2 ready-to-merge · 21 waiting-author · 1 waiting-build · 15 waiting-self-review · 46 no verdict
- dashpay/rust-dashcore: **35** open (6 clean · 1 CI failing · 4 changes requested · 2 unresolved comments · 0 deferred · 17 draft · 5 stale) · engine: 17 draft · 3 waiting-author · 10 waiting-self-review · 5 no verdict
- dashpay/tenderdash: **18** open (2 clean · 0 CI failing · 0 changes requested · 2 unresolved comments · 0 deferred · 5 draft · 9 stale) · engine: 5 draft · 1 ready-for-human · 1 waiting-author · 2 waiting-self-review · 9 no verdict
- dashpay/grovedb: **6** open (0 clean · 0 CI failing · 0 changes requested · 0 unresolved comments · 0 deferred · 2 draft · 4 stale) · engine: 2 draft · 4 no verdict
- dashpay/dash-evo-tool: **38** open (0 clean · 0 CI failing · 0 changes requested · 15 unresolved comments · 0 deferred · 2 draft · 21 stale) · engine: 2 draft · 14 waiting-author · 1 waiting-bots · 21 no verdict

## Scoreboard
_Sort: unresolved-comments desc → needs-action desc → ready-for-review desc. Click any number to jump to the specific PRs it covers._

| Author | Open | Clean | CI failing | Unresolved Comments | Changes Requested | Deferred | Draft | Stale | Needs action | Ready for human | Total Unresolved Comments | Ready for Review | Δ |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| [@Claudius-Maginificent](#claudius-maginificent) | [68](#claudius-maginificent-open) | [1](#claudius-maginificent-clean) | — | [16](#claudius-maginificent-unresolved-comments) | — | [3](#claudius-maginificent-deferred) | [7](#claudius-maginificent-draft) | [41](#claudius-maginificent-stale) | [16](#claudius-maginificent-needs-action) | [1](#claudius-maginificent-ready-for-human) | [132](#claudius-maginificent-unresolved-comments) | — | ↑ 1 |
| [@lklimek](#lklimek) | [26](#lklimek-open) | — | — | [7](#lklimek-unresolved-comments) | [1](#lklimek-changes-requested) | [1](#lklimek-deferred) | [5](#lklimek-draft) | [12](#lklimek-stale) | [8](#lklimek-needs-action) | [1](#lklimek-ready-for-human) | [43](#lklimek-unresolved-comments) | [2](#lklimek-ready-for-review) | ↑ 4 |
| [@QuantumExplorer](#quantumexplorer) | [23](#quantumexplorer-open) | [2](#quantumexplorer-clean) | — | [5](#quantumexplorer-unresolved-comments) | [1](#quantumexplorer-changes-requested) | — | [4](#quantumexplorer-draft) | [11](#quantumexplorer-stale) | [6](#quantumexplorer-needs-action) | — | [61](#quantumexplorer-unresolved-comments) | [6](#quantumexplorer-ready-for-review) | ↓ 1 |
| [@shumkov](#shumkov) | [12](#shumkov-open) | [2](#shumkov-clean) | — | [3](#shumkov-unresolved-comments) | — | — | [2](#shumkov-draft) | [5](#shumkov-stale) | [3](#shumkov-needs-action) | — | [26](#shumkov-unresolved-comments) | [4](#shumkov-ready-for-review) | ↓ 1 |
| [@xdustinface](#xdustinface) | [8](#xdustinface-open) | — | — | [2](#xdustinface-unresolved-comments) | [2](#xdustinface-changes-requested) | — | [1](#xdustinface-draft) | [3](#xdustinface-stale) | [4](#xdustinface-needs-action) | — | [3](#xdustinface-unresolved-comments) | [6](#xdustinface-ready-for-review) | ↑ 3 |
| [@HashEngineering](#hashengineering) | [5](#hashengineering-open) | [2](#hashengineering-clean) | — | [2](#hashengineering-unresolved-comments) | — | — | — | [1](#hashengineering-stale) | [3](#hashengineering-needs-action) | [1](#hashengineering-ready-for-human) | [9](#hashengineering-unresolved-comments) | [1](#hashengineering-ready-for-review) | ↑ 3 |
| [@bfoss765](#bfoss765) | [2](#bfoss765-open) | — | — | [2](#bfoss765-unresolved-comments) | — | — | — | — | [2](#bfoss765-needs-action) | — | [7](#bfoss765-unresolved-comments) | — | ↑ 2 |
| [@llbartekll](#llbartekll) | [1](#llbartekll-open) | — | — | [1](#llbartekll-unresolved-comments) | — | — | — | — | [1](#llbartekll-needs-action) | — | [4](#llbartekll-unresolved-comments) | [2](#llbartekll-ready-for-review) | ↑ 1 |
| [@romchornyi](#romchornyi) | [16](#romchornyi-open) | [2](#romchornyi-clean) | [1](#romchornyi-ci-failing) | [1](#romchornyi-unresolved-comments) | — | — | [6](#romchornyi-draft) | [6](#romchornyi-stale) | [1](#romchornyi-needs-action) | [1](#romchornyi-ready-for-human) | [41](#romchornyi-unresolved-comments) | — | ↑ 1 |
| [@ktechmidas](#ktechmidas) | [6](#ktechmidas-open) | — | [1](#ktechmidas-ci-failing) | — | [1](#ktechmidas-changes-requested) | — | — | [4](#ktechmidas-stale) | [1](#ktechmidas-needs-action) | — | [1](#ktechmidas-unresolved-comments) | [4](#ktechmidas-ready-for-review) | ↑ 1 |
| [@ZocoLini](#zocolini) | [11](#zocolini-open) | [6](#zocolini-clean) | [1](#zocolini-ci-failing) | — | [1](#zocolini-changes-requested) | — | [2](#zocolini-draft) | [1](#zocolini-stale) | [1](#zocolini-needs-action) | — | — | [1](#zocolini-ready-for-review) | — |
| [@PastaPastaPasta](#pastapastapasta) | [39](#pastapastapasta-open) | [1](#pastapastapasta-clean) | — | — | — | — | [33](#pastapastapasta-draft) | [5](#pastapastapasta-stale) | [1](#pastapastapasta-needs-action) | — | [17](#pastapastapasta-unresolved-comments) | — | ↓ 1 |
| [@thepastaclaw](#thepastaclaw) | [1](#thepastaclaw-open) | — | [1](#thepastaclaw-ci-failing) | — | — | — | — | — | [1](#thepastaclaw-needs-action) | — | — | — | ↑ 1 |
| [@DCG-Claude](#dcg-claude) | [3](#dcg-claude-open) | — | [1](#dcg-claude-ci-failing) | — | — | — | — | [2](#dcg-claude-stale) | — | [2](#dcg-claude-ready-for-human) | — | — | — |

## Per-author detail

<a id="claudius-maginificent"></a>
### @Claudius-Maginificent
<a id="claudius-maginificent-open"></a>
#### Open (68)
- [dashpay/dash-evo-tool#1178 fix(tokens): keep JSON export responsive when the file chooser stalls](https://github.com/dashpay/dash-evo-tool/pull/1178) — 15 unresolved (7 CodeRabbit, 8 bot) · 6 days stale · ✋ changes requested · 📝 draft · 🐢 targets feat/app-design-system · areas: fallback
  - Top thread: "🟡 Suggestion: Exercise a publication failure after staging succeeds**" — 6 days old
- [dashpay/platform#5645 fix(platform-wallet)!: check identity creation keys and funding in the Rust core](https://github.com/dashpay/platform/pull/5645) — 9 unresolved (1 CodeRabbit, 8 bot) · 23 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · areas: dpp, rs-platform-wallet, rs-platform-wallet-ffi, rust-sdk · Policy: waiting-author
  - Top thread: "&lt;picture&gt;&lt;img src="https://storage.googleapis.com/coderabbit_public_assets/review-comment-icons-v1/amber-16.svg" width="…" — 23 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: coderabbitai left review threads unresolved; resolve them
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/dash-evo-tool#1177 feat(ui): align app-wide styles with the token proposal](https://github.com/dashpay/dash-evo-tool/pull/1177) — 13 unresolved (13 bot) · 8 days stale · ⚠ merge conflict · ✋ changes requested · 📝 draft · 🐢 targets feat/token-creator-workspace · areas: fallback
  - Top thread: "✅ **Resolved** at \`0993a6af\`; see the replies below." — 8 days old
- [dashpay/platform#5565 feat(platform-wallet)!: detect inconsistent shielded commitment trees and add explicit rebuild](https://github.com/dashpay/platform/pull/5565) — 7 unresolved (7 bot) · 27 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk · Policy: waiting-author
  - Top thread: "⏸️ **Needs a tracking issue** (not yet deferred) at \`3b17ab95\`; see the replies below." — 27 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/dash-evo-tool#1214 fix(tokens): identity token model](https://github.com/dashpay/dash-evo-tool/pull/1214) — 10 unresolved (1 CodeRabbit, 9 bot) · 3 days stale · ✋ changes requested · 🔴 CI failing · areas: fallback · Policy: waiting-author
  - Top thread: "🟡 Suggestion: My Tokens sorts names by characters it no longer displays**" — 3 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: coderabbitai left review threads unresolved; resolve them
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/dash-evo-tool#1175 docs: add maintainable token tutorial production toolkit](https://github.com/dashpay/dash-evo-tool/pull/1175) — 12 unresolved (12 CodeRabbit) · 2 days stale · areas: fallback · Policy: waiting-author
  - Top thread: "&lt;picture&gt;&lt;img src="https://storage.googleapis.com/coderabbit_public_assets/review-comment-icons-v1/major-16.svg" width="…" — 2 days old
  - Blocker: coderabbitai left review threads unresolved; resolve them
  - Blocker: Proceeded without thepastaclaw: no review within the configured window
- [dashpay/platform#5644 fix!: follow DIP-11 key rules when disabling keys and reading contact requests](https://github.com/dashpay/platform/pull/5644) — 8 unresolved (8 bot) · 4 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · areas: dpp, rs-platform-wallet, rs-platform-wallet-ffi, rust-sdk, rust-sdk-ffi, js-wasm-sdk, fallback · Policy: waiting-author
  - Top thread: "✅ **Resolved** at \`ed16501f\`; see the replies below." — 4 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/dash-evo-tool#1108 feat(documents): recognise indexOnly document types and delete their entries by values](https://github.com/dashpay/dash-evo-tool/pull/1108) — 3 unresolved (3 bot) · 34 days stale · ✋ changes requested · 📝 draft · 🐢 targets feat/document-optional-token-cost · areas: fallback
  - Top thread: "🟡 Suggestion: Make the non-deletable error actionable**" — 34 days old
- [dashpay/dash-evo-tool#1179 fix(tokens): retain the creator when opening a registered contract](https://github.com/dashpay/dash-evo-tool/pull/1179) — 4 unresolved (4 bot) · 6 days stale · ✋ changes requested · 📝 draft · 🐢 targets fix/token-json-export · areas: fallback
  - Top thread: "🟡 Suggestion: Assert that the preview displays fetched contract details**" — 6 days old
- [dashpay/platform#5546 fix(platform-wallet): support HASH160 DashPay contact signing keys](https://github.com/dashpay/platform/pull/5546) — 2 unresolved (2 bot) · 27 days stale · ⚠ merge conflict · 📝 draft · 🐢 targets feat/platform-wallet-public-signing-key-selection · areas: rs-drive-abci, dpp, rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk, fallback
  - Top thread: "🟡 Suggestion: Do not expose an unchecked signer-capability override in the public constructor**" — 27 days old
- [dashpay/platform#5588 feat(platform-wallet)!: expose protocol-14-aware signing-key selection](https://github.com/dashpay/platform/pull/5588) — 3 unresolved (3 bot) · 7 days stale · 🔴 CI failing · areas: dpp, rs-platform-wallet, rs-platform-wallet-ffi, rust-sdk · Policy: waiting-author
  - Top thread: "⏭️ **Deferred** (maintainer decision; not blocking) at \`d2eb7b54\`; see the replies below." — 7 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/dash-evo-tool#1252 test(backend-e2e): run the suite against a devnet](https://github.com/dashpay/dash-evo-tool/pull/1252) — 6 unresolved (6 bot) · 0 days stale · ✋ changes requested · 🔴 CI failing · areas: fallback · Policy: waiting-author
  - Top thread: "🟡 Suggestion: The documented 35 DASH leaves less than the suite minimum after bootstrap**" — 0 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/dash-evo-tool#1117 fix(identity): choose signing keys Platform accepts on protocol 14](https://github.com/dashpay/dash-evo-tool/pull/1117) — 4 unresolved (4 bot) · 3 days stale · ✋ changes requested · 🔴 CI failing · areas: fallback · Policy: waiting-bots
  - Top thread: "🟡 Suggestion: The signing selection type can report a usable key without containing one**" — 3 days old
  - Blocker: coderabbitai has not reported for the current head
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/dash-evo-tool#1174 feat(skills): mark defects in visual-diff galleries](https://github.com/dashpay/dash-evo-tool/pull/1174) — 4 unresolved (4 bot) · 3 days stale · ✋ changes requested · 🔴 CI failing · areas: fallback · Policy: waiting-author
  - Top thread: "🟡 Suggestion: The skill tells reviewers to omit unchanged regions and banners that show confirmed defects**" — 3 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/dash-evo-tool#1176 feat(tokens): add guided token creation with complete review](https://github.com/dashpay/dash-evo-tool/pull/1176) — 5 unresolved (5 bot) · 0 days stale · ✋ changes requested · 🔴 CI failing · areas: fallback · Policy: waiting-author
  - Top thread: "🟡 Suggestion: The offline save API accepts a pending record relabeled as confirmed**" — 0 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/tenderdash#1651 ci(docker): build release images before publication and promote by digest](https://github.com/dashpay/tenderdash/pull/1651) — 5 unresolved (5 bot) · 0 days stale · ✋ changes requested · 🔴 CI failing · 📝 draft · areas: github, fallback · Policy: draft
  - Top thread: "🟡 Suggestion: SHA or branch staging can succeed even when its image cannot be promoted**" — 0 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5981 refactor(platform-wallet-storage): drop the unreachable rehydration gap-refill guard](https://github.com/dashpay/platform/pull/5981) — 2 unresolved (2 bot) · 8 days stale · ✋ changes requested · 📝 draft · 🐢 targets refactor/platform-node-pool-single-owner · areas: wallet-storage
  - Top thread: "🔴 Blocking: Preserve script-based ownership after a failed refill**" — 8 days old
- [dashpay/dash-evo-tool#1240 feat(send): clearer confirmation and a steady form width](https://github.com/dashpay/dash-evo-tool/pull/1240) — 3 unresolved (3 bot) · 3 days stale · ✋ changes requested · 🔴 CI failing · areas: fallback · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Opening a batch confirmation reloads wallet and identity data for every recipient**" — 3 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/dash-evo-tool#1124 feat(documents): let the user skip an optional token cost](https://github.com/dashpay/dash-evo-tool/pull/1124) — 1 unresolved (1 bot) · 31 days stale · ⚠ merge conflict · ✋ changes requested · 📝 draft · 🐢 targets fix/protocol14-signing-key-selection · areas: fallback
  - Top thread: "🔴 Blocking: Show the credit-fee outcome when an optional token cost is disabled**" — 31 days old
- [dashpay/platform#5707 fix(sdk): recover the nonce cache when unconfirmed nonces leave the accepted window](https://github.com/dashpay/platform/pull/5707) — 1 unresolved (1 bot) · 24 days stale · ✋ changes requested · 🔴 CI failing · 📝 draft · areas: rust-sdk · Policy: draft
  - Top thread: "🔴 Blocking: Do not reissue nonces belonging to outstanding operations**" — 24 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/tenderdash#1641 fix(rpc): release upstream subscriptions orphaned by a resubscribe](https://github.com/dashpay/tenderdash/pull/1641) — 3 unresolved (3 bot) · 1 days stale · ✋ changes requested · 📝 draft · 🐢 targets fix/light-proxy-ws-worker-lifetime · areas: fallback
  - Top thread: "🟡 Suggestion: The replacement-first test can see the resend arrive first**" — 1 days old
- [dashpay/dash-evo-tool#1202 feat(boot): ask an AI assistant about a startup failure](https://github.com/dashpay/dash-evo-tool/pull/1202) — 1 unresolved (1 human) · 2 days stale · ✋ changes requested · 🔴 CI failing · areas: fallback · Policy: waiting-author
  - Top thread: "Create a separate "ask ai" component that will be in future used for other use cases. The component should be a dialog w…" — 2 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/dash-evo-tool#1249 fix(dashpay): cache DashPay profiles normalized like background sync](https://github.com/dashpay/dash-evo-tool/pull/1249) — 2 unresolved (2 bot) · 0 days stale · ✋ changes requested · 🔴 CI failing · areas: fallback · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Manual refresh caches an empty profile that background sync clears**" — 0 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/tenderdash#1642 fix(rpc): keep the event loop running during an error-triggered resubscribe](https://github.com/dashpay/tenderdash/pull/1642) — 2 unresolved (2 bot) · 1 days stale · 🔴 CI failing · 📝 draft · 🐢 targets fix/ws-resubscribe-orphan · areas: fallback
  - Top thread: "🟡 Suggestion: The error-and-drop test passes even if event delivery never resumes after reconnect**" — 1 days old
- [dashpay/platform#5647 feat: expose the Rust identity key layout to FFI, Swift and Kotlin hosts](https://github.com/dashpay/platform/pull/5647) — 1 unresolved (1 bot) · 4 days stale · ✋ changes requested · 🔴 CI failing · 📝 draft · 🐢 targets fix/platform-wallet-dip11-identity-key-layout · areas: rs-platform-wallet-ffi, swift-sdk, kotlin-sdk
  - Top thread: "🟡 Suggestion: Kotlin reports a failed protocol-version lookup as non-retryable**" — 4 days old
- [dashpay/dash-evo-tool#1206 fix(boot): show startup panics and window failures instead of closing](https://github.com/dashpay/dash-evo-tool/pull/1206) — 1 unresolved (1 bot) · 4 days stale · areas: fallback · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Startup tests pass even if the native alert is shown at the wrong time**" — 4 days old
  - Blocker: thepastaclaw left review threads unresolved; resolve them
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/platform#5661 feat: decide which identity keys may be disabled in the wallet, not in the mobile apps](https://github.com/dashpay/platform/pull/5661) — 1 unresolved (1 bot) · 3 days stale · 🔴 CI failing · 📝 draft · 🐢 targets fix/platform-wallet-dip11-key-rules · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk
  - Top thread: "🟡 Suggestion: The last-TRANSFER test passes even if submission checks the pre-refresh key set**" — 3 days old
- [dashpay/dash-evo-tool#1243 fix(identity): hint that locked wallets are not searched for identities](https://github.com/dashpay/dash-evo-tool/pull/1243) — 1 unresolved (1 bot) · 1 days stale · ✋ changes requested · 🔴 CI failing · areas: fallback · Policy: waiting-author
  - Top thread: "🔴 Blocking: Unlocking from the hint while disconnected can hide it without searching the wallet**" — 1 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#6035 fix(platform-wallet): keep pre-derived lost funding rows ineligible after restart](https://github.com/dashpay/platform/pull/6035) — 1 unresolved (1 bot) · 2 days stale · ✋ changes requested · 🔴 CI failing · areas: rs-platform-wallet · Policy: waiting-author
  - Top thread: "🔴 Blocking: A corrupt stored index can leave the wallet unable to issue invitations after restart**" — 2 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/tenderdash#1578 fix(statesync): refuse surplus light block responses without blocking](https://github.com/dashpay/tenderdash/pull/1578) — 1 unresolved (1 bot) · 1 days stale · 📝 draft · areas: fallback · Policy: draft
  - Top thread: "🟡 Suggestion: A duplicate light-block reply can go unreported after the caller takes the first answer**" — 1 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/tenderdash#1639 fix(light): retire WebSocket forwarders with their subscriptions](https://github.com/dashpay/tenderdash/pull/1639) — 1 unresolved (1 bot) · 1 days stale · ✋ changes requested · 📝 draft · 🐢 targets fix/base-service-stop · areas: fallback
  - Top thread: "🔴 Blocking: Retrying a timed-out unsubscribe reports success without freeing upstream subscription slots**" — 1 days old
- [dashpay/dash-evo-tool#1120 feat(identity): add keys with spending limits and raise key limits](https://github.com/dashpay/dash-evo-tool/pull/1120) — ⚠ merge conflict · 📝 draft · 🐢 targets fix/protocol14-signing-key-selection · areas: fallback
- [dashpay/dash-evo-tool#1121 feat(contracts): view and claim contract fee pots](https://github.com/dashpay/dash-evo-tool/pull/1121) — ⚠ merge conflict · 📝 draft · 🐢 targets fix/protocol14-signing-key-selection · areas: fallback
- [dashpay/dash-evo-tool#1122 feat(documents): agree to document action fees before broadcasting](https://github.com/dashpay/dash-evo-tool/pull/1122) — ⚠ merge conflict · 📝 draft · 🐢 targets fix/protocol14-signing-key-selection · areas: fallback
- [dashpay/dash-evo-tool#1123 feat(identity): add authentication keys bound to a contract](https://github.com/dashpay/dash-evo-tool/pull/1123) — 📝 draft · 🐢 targets feat/identity-key-limits · areas: fallback
- [dashpay/dash-evo-tool#1137 fix(identity): unlock the wallet that holds the chosen signing key](https://github.com/dashpay/dash-evo-tool/pull/1137) — ⚠ merge conflict · 📝 draft · 🐢 targets fix/protocol14-signing-key-selection · areas: fallback
- [dashpay/dash-evo-tool#1192 feat(identity): claim token rewards from the identity hub](https://github.com/dashpay/dash-evo-tool/pull/1192) — ⚠ merge conflict · 📝 draft · 🐢 targets feat/identity-hub-tokens · areas: fallback
- [dashpay/dash-evo-tool#1195 fix(dashpay): recover contact requests this device could not record](https://github.com/dashpay/dash-evo-tool/pull/1195) — 📝 draft · 🐢 targets fix/dashpay-contact-info-keys · areas: fallback
- [dashpay/dash-evo-tool#1197 fix(send): offer a private-balance Max the network accepts](https://github.com/dashpay/dash-evo-tool/pull/1197) — 📝 draft · areas: fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/dash-evo-tool#1212 fix(tokens): close the confirmation when a refresh moves the signing key](https://github.com/dashpay/dash-evo-tool/pull/1212) — ⚠ merge conflict · 📝 draft · 🐢 targets fix/protocol14-signing-key-selection · areas: fallback
- [dashpay/dash-evo-tool#1215 feat(tokens): refresh token balances on request](https://github.com/dashpay/dash-evo-tool/pull/1215) — ⚠ merge conflict · 📝 draft · 🐢 targets refactor/identity-token-model · areas: fallback
- [dashpay/dash-evo-tool#1216 feat(identity): Tokens tab in the identity hub](https://github.com/dashpay/dash-evo-tool/pull/1216) — 📝 draft · 🐢 targets feat/token-balance-refresh · areas: fallback
- [dashpay/dash-evo-tool#1217 feat(identity): token detail page in the identity hub](https://github.com/dashpay/dash-evo-tool/pull/1217) — 📝 draft · 🐢 targets feat/identity-hub-tokens-tab · areas: fallback
- [dashpay/dash-evo-tool#1218 feat(identity): tokens card on the identity Home tab](https://github.com/dashpay/dash-evo-tool/pull/1218) — 📝 draft · 🐢 targets feat/identity-hub-token-details · areas: fallback
- [dashpay/dash-evo-tool#1219 fix(tokens): re-importing a dismissed token restores its tracking](https://github.com/dashpay/dash-evo-tool/pull/1219) — 📝 draft · 🐢 targets feat/identity-hub-tokens-tab · areas: fallback
- [dashpay/dash-evo-tool#1222 fix(dashpay): drop the cached contact QR image with the code](https://github.com/dashpay/dash-evo-tool/pull/1222) — 📝 draft · 🐢 targets fix/protocol14-signing-key-selection · areas: fallback
- [dashpay/platform#3549 test(platform-wallet): e2e framework + full test suite — triage pins, Found-*/PA-* guards, fail-closed persist, Stage-2 merge](https://github.com/dashpay/platform/pull/3549) — 📝 draft · ⏸ deferred · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk, kotlin-sdk, github, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5384 fix(platform-wallet)!: persist chain truncations so a stored wallet follows a reorg](https://github.com/dashpay/platform/pull/5384) — ⏸ deferred · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk, kotlin-sdk · Policy: waiting-author
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5417 fix(platform-wallet): own per-wallet transaction accounting in Rust for SwiftData](https://github.com/dashpay/platform/pull/5417) — 📝 draft · ⏸ deferred · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk
- [dashpay/platform#5660 refactor(dpp): own the contract-bounds verdict for a document create](https://github.com/dashpay/platform/pull/5660) — 📝 draft · 🐢 targets fix/platform-wallet-contact-info-hash160 · areas: rs-drive-abci, dpp, rs-platform-wallet
- [dashpay/platform#5782 fix(wallet-storage): restore Platform and Core from one wallet snapshot](https://github.com/dashpay/platform/pull/5782) — 📝 draft · 🐢 targets fix/persist-drain-conflict-outcomes · areas: wallet-storage
- [dashpay/platform#5783 fix(wallet-storage): reconcile historical AssetUnlock identities](https://github.com/dashpay/platform/pull/5783) — 📝 draft · 🐢 targets fix/funding-issuance-floor-across-reload · areas: rs-platform-wallet, wallet-storage
- [dashpay/platform#5787 fix(platform-wallet): classify asset locks funding foreign credits as outgoing](https://github.com/dashpay/platform/pull/5787) — 📝 draft · 🐢 targets fix/sqlite-platform-address-snapshot · areas: rs-platform-wallet, wallet-storage
- [dashpay/platform#5961 fix(platform-wallet): don't pick a limited key to sign a data contract create or update](https://github.com/dashpay/platform/pull/5961) — 📝 draft · 🐢 targets refactor/document-create-bounds-in-dpp · areas: rs-platform-wallet
- [dashpay/platform#6025 feat(platform-wallet): read the chain-following masternode list with its height](https://github.com/dashpay/platform/pull/6025) — 📝 draft · 🐢 targets feat/mn-key-id-check · areas: rs-platform-wallet
- [dashpay/platform#6026 feat(platform-wallet): fetch the key references of any masternode](https://github.com/dashpay/platform/pull/6026) — 📝 draft · 🐢 targets feat/mn-current-list · areas: rs-platform-wallet
- [dashpay/platform#6027 feat(platform-wallet): return the proved block height with a withdrawal's remaining balance](https://github.com/dashpay/platform/pull/6027) — ⚠ merge conflict · 📝 draft · 🐢 targets feat/mn-key-reference · areas: rs-platform-wallet, rs-platform-wallet-ffi
- [dashpay/platform#6028 feat(platform-wallet): withdraw masternode credits with a wallet master-key provider or a host signer](https://github.com/dashpay/platform/pull/6028) — 📝 draft · 🐢 targets feat/mn-withdrawal-height · areas: rs-platform-wallet, rs-platform-wallet-ffi
- [dashpay/platform#6030 chore: update rust-dashcore for masternode lookup and lists](https://github.com/dashpay/platform/pull/6030) — ⚠ merge conflict · 🔴 CI failing · 📝 draft · 🐢 targets feat/mn-withdrawal-key-source · areas: dpp, rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk, js-wasm-sdk, fallback
- [dashpay/platform#6031 feat(platform-wallet): find a wallet's owner or voting key by its hash](https://github.com/dashpay/platform/pull/6031) — 📝 draft · 🐢 targets chore/mn-dashcore-k1-s1 · areas: rs-platform-wallet, rs-platform-wallet-ffi
- [dashpay/platform#6032 refactor(platform-wallet): read the current masternode list from dash-spv](https://github.com/dashpay/platform/pull/6032) — 📝 draft · 🐢 targets feat/mn-provider-key-lookup · areas: rs-platform-wallet, rs-platform-wallet-ffi
- [dashpay/platform#6036 refactor(platform-wallet): drop always-zero Core address balance](https://github.com/dashpay/platform/pull/6036) — 📝 draft · 🐢 targets fix/funding-pool-lost-row-in-window · areas: rs-platform-wallet-ffi, swift-sdk, kotlin-sdk
- [dashpay/platform#6037 refactor(platform-wallet): one owner for platform-node pool entries](https://github.com/dashpay/platform/pull/6037) — 📝 draft · 🐢 targets refactor/core-address-entry-drop-balance · areas: rs-platform-wallet, wallet-storage
- [dashpay/platform#6295 fix(platform-wallet)!: discover identities whose master key is ECDSA_HASH160](https://github.com/dashpay/platform/pull/6295) — 📝 draft · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#6346 fix(sdk): reject duplicate identity top-ups and retry chain-lock height lag](https://github.com/dashpay/platform/pull/6346) — 🔴 CI failing · 📝 draft · areas: rs-platform-wallet, rust-sdk, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/tenderdash#1600 fix(consensus): deliver and process a polka block for a validator left in its round](https://github.com/dashpay/tenderdash/pull/1600) — areas: fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
- [dashpay/tenderdash#1627 fix(p2p): bound the simple priority queue, honor channel priorities and count drops](https://github.com/dashpay/tenderdash/pull/1627) — 📝 draft · 🐢 targets fix/consensus-serve-polka-block · areas: fallback
- [dashpay/tenderdash#1647 refactor(privval)!: remove the socket and gRPC remote signer](https://github.com/dashpay/tenderdash/pull/1647) — 📝 draft · areas: fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot

<a id="claudius-maginificent-needs-action"></a>
#### Needs action (16)
- [dashpay/platform#5645 fix(platform-wallet)!: check identity creation keys and funding in the Rust core](https://github.com/dashpay/platform/pull/5645) — 9 unresolved (1 CodeRabbit, 8 bot) · 23 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · areas: dpp, rs-platform-wallet, rs-platform-wallet-ffi, rust-sdk · Policy: waiting-author
  - Top thread: "&lt;picture&gt;&lt;img src="https://storage.googleapis.com/coderabbit_public_assets/review-comment-icons-v1/amber-16.svg" width="…" — 23 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: coderabbitai left review threads unresolved; resolve them
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5565 feat(platform-wallet)!: detect inconsistent shielded commitment trees and add explicit rebuild](https://github.com/dashpay/platform/pull/5565) — 7 unresolved (7 bot) · 27 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk · Policy: waiting-author
  - Top thread: "⏸️ **Needs a tracking issue** (not yet deferred) at \`3b17ab95\`; see the replies below." — 27 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/dash-evo-tool#1214 fix(tokens): identity token model](https://github.com/dashpay/dash-evo-tool/pull/1214) — 10 unresolved (1 CodeRabbit, 9 bot) · 3 days stale · ✋ changes requested · 🔴 CI failing · areas: fallback · Policy: waiting-author
  - Top thread: "🟡 Suggestion: My Tokens sorts names by characters it no longer displays**" — 3 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: coderabbitai left review threads unresolved; resolve them
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/dash-evo-tool#1175 docs: add maintainable token tutorial production toolkit](https://github.com/dashpay/dash-evo-tool/pull/1175) — 12 unresolved (12 CodeRabbit) · 2 days stale · areas: fallback · Policy: waiting-author
  - Top thread: "&lt;picture&gt;&lt;img src="https://storage.googleapis.com/coderabbit_public_assets/review-comment-icons-v1/major-16.svg" width="…" — 2 days old
  - Blocker: coderabbitai left review threads unresolved; resolve them
  - Blocker: Proceeded without thepastaclaw: no review within the configured window
- [dashpay/platform#5644 fix!: follow DIP-11 key rules when disabling keys and reading contact requests](https://github.com/dashpay/platform/pull/5644) — 8 unresolved (8 bot) · 4 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · areas: dpp, rs-platform-wallet, rs-platform-wallet-ffi, rust-sdk, rust-sdk-ffi, js-wasm-sdk, fallback · Policy: waiting-author
  - Top thread: "✅ **Resolved** at \`ed16501f\`; see the replies below." — 4 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5588 feat(platform-wallet)!: expose protocol-14-aware signing-key selection](https://github.com/dashpay/platform/pull/5588) — 3 unresolved (3 bot) · 7 days stale · 🔴 CI failing · areas: dpp, rs-platform-wallet, rs-platform-wallet-ffi, rust-sdk · Policy: waiting-author
  - Top thread: "⏭️ **Deferred** (maintainer decision; not blocking) at \`d2eb7b54\`; see the replies below." — 7 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/dash-evo-tool#1252 test(backend-e2e): run the suite against a devnet](https://github.com/dashpay/dash-evo-tool/pull/1252) — 6 unresolved (6 bot) · 0 days stale · ✋ changes requested · 🔴 CI failing · areas: fallback · Policy: waiting-author
  - Top thread: "🟡 Suggestion: The documented 35 DASH leaves less than the suite minimum after bootstrap**" — 0 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/dash-evo-tool#1117 fix(identity): choose signing keys Platform accepts on protocol 14](https://github.com/dashpay/dash-evo-tool/pull/1117) — 4 unresolved (4 bot) · 3 days stale · ✋ changes requested · 🔴 CI failing · areas: fallback · Policy: waiting-bots
  - Top thread: "🟡 Suggestion: The signing selection type can report a usable key without containing one**" — 3 days old
  - Blocker: coderabbitai has not reported for the current head
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/dash-evo-tool#1174 feat(skills): mark defects in visual-diff galleries](https://github.com/dashpay/dash-evo-tool/pull/1174) — 4 unresolved (4 bot) · 3 days stale · ✋ changes requested · 🔴 CI failing · areas: fallback · Policy: waiting-author
  - Top thread: "🟡 Suggestion: The skill tells reviewers to omit unchanged regions and banners that show confirmed defects**" — 3 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/dash-evo-tool#1176 feat(tokens): add guided token creation with complete review](https://github.com/dashpay/dash-evo-tool/pull/1176) — 5 unresolved (5 bot) · 0 days stale · ✋ changes requested · 🔴 CI failing · areas: fallback · Policy: waiting-author
  - Top thread: "🟡 Suggestion: The offline save API accepts a pending record relabeled as confirmed**" — 0 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/dash-evo-tool#1240 feat(send): clearer confirmation and a steady form width](https://github.com/dashpay/dash-evo-tool/pull/1240) — 3 unresolved (3 bot) · 3 days stale · ✋ changes requested · 🔴 CI failing · areas: fallback · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Opening a batch confirmation reloads wallet and identity data for every recipient**" — 3 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/dash-evo-tool#1202 feat(boot): ask an AI assistant about a startup failure](https://github.com/dashpay/dash-evo-tool/pull/1202) — 1 unresolved (1 human) · 2 days stale · ✋ changes requested · 🔴 CI failing · areas: fallback · Policy: waiting-author
  - Top thread: "Create a separate "ask ai" component that will be in future used for other use cases. The component should be a dialog w…" — 2 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/dash-evo-tool#1249 fix(dashpay): cache DashPay profiles normalized like background sync](https://github.com/dashpay/dash-evo-tool/pull/1249) — 2 unresolved (2 bot) · 0 days stale · ✋ changes requested · 🔴 CI failing · areas: fallback · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Manual refresh caches an empty profile that background sync clears**" — 0 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/dash-evo-tool#1206 fix(boot): show startup panics and window failures instead of closing](https://github.com/dashpay/dash-evo-tool/pull/1206) — 1 unresolved (1 bot) · 4 days stale · areas: fallback · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Startup tests pass even if the native alert is shown at the wrong time**" — 4 days old
  - Blocker: thepastaclaw left review threads unresolved; resolve them
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/dash-evo-tool#1243 fix(identity): hint that locked wallets are not searched for identities](https://github.com/dashpay/dash-evo-tool/pull/1243) — 1 unresolved (1 bot) · 1 days stale · ✋ changes requested · 🔴 CI failing · areas: fallback · Policy: waiting-author
  - Top thread: "🔴 Blocking: Unlocking from the hint while disconnected can hide it without searching the wallet**" — 1 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#6035 fix(platform-wallet): keep pre-derived lost funding rows ineligible after restart](https://github.com/dashpay/platform/pull/6035) — 1 unresolved (1 bot) · 2 days stale · ✋ changes requested · 🔴 CI failing · areas: rs-platform-wallet · Policy: waiting-author
  - Top thread: "🔴 Blocking: A corrupt stored index can leave the wallet unable to issue invitations after restart**" — 2 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them

<a id="claudius-maginificent-ready-for-human"></a>
#### Ready for human (1)
- [dashpay/tenderdash#1600 fix(consensus): deliver and process a polka block for a validator left in its round](https://github.com/dashpay/tenderdash/pull/1600) — areas: fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required

<a id="claudius-maginificent-unresolved-comments"></a>
#### Unresolved Comments (16)
- [dashpay/platform#5645 fix(platform-wallet)!: check identity creation keys and funding in the Rust core](https://github.com/dashpay/platform/pull/5645) — 9 unresolved (1 CodeRabbit, 8 bot) · 23 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · areas: dpp, rs-platform-wallet, rs-platform-wallet-ffi, rust-sdk · Policy: waiting-author
  - Top thread: "&lt;picture&gt;&lt;img src="https://storage.googleapis.com/coderabbit_public_assets/review-comment-icons-v1/amber-16.svg" width="…" — 23 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: coderabbitai left review threads unresolved; resolve them
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5565 feat(platform-wallet)!: detect inconsistent shielded commitment trees and add explicit rebuild](https://github.com/dashpay/platform/pull/5565) — 7 unresolved (7 bot) · 27 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk · Policy: waiting-author
  - Top thread: "⏸️ **Needs a tracking issue** (not yet deferred) at \`3b17ab95\`; see the replies below." — 27 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/dash-evo-tool#1214 fix(tokens): identity token model](https://github.com/dashpay/dash-evo-tool/pull/1214) — 10 unresolved (1 CodeRabbit, 9 bot) · 3 days stale · ✋ changes requested · 🔴 CI failing · areas: fallback · Policy: waiting-author
  - Top thread: "🟡 Suggestion: My Tokens sorts names by characters it no longer displays**" — 3 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: coderabbitai left review threads unresolved; resolve them
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/dash-evo-tool#1175 docs: add maintainable token tutorial production toolkit](https://github.com/dashpay/dash-evo-tool/pull/1175) — 12 unresolved (12 CodeRabbit) · 2 days stale · areas: fallback · Policy: waiting-author
  - Top thread: "&lt;picture&gt;&lt;img src="https://storage.googleapis.com/coderabbit_public_assets/review-comment-icons-v1/major-16.svg" width="…" — 2 days old
  - Blocker: coderabbitai left review threads unresolved; resolve them
  - Blocker: Proceeded without thepastaclaw: no review within the configured window
- [dashpay/platform#5644 fix!: follow DIP-11 key rules when disabling keys and reading contact requests](https://github.com/dashpay/platform/pull/5644) — 8 unresolved (8 bot) · 4 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · areas: dpp, rs-platform-wallet, rs-platform-wallet-ffi, rust-sdk, rust-sdk-ffi, js-wasm-sdk, fallback · Policy: waiting-author
  - Top thread: "✅ **Resolved** at \`ed16501f\`; see the replies below." — 4 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5588 feat(platform-wallet)!: expose protocol-14-aware signing-key selection](https://github.com/dashpay/platform/pull/5588) — 3 unresolved (3 bot) · 7 days stale · 🔴 CI failing · areas: dpp, rs-platform-wallet, rs-platform-wallet-ffi, rust-sdk · Policy: waiting-author
  - Top thread: "⏭️ **Deferred** (maintainer decision; not blocking) at \`d2eb7b54\`; see the replies below." — 7 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/dash-evo-tool#1252 test(backend-e2e): run the suite against a devnet](https://github.com/dashpay/dash-evo-tool/pull/1252) — 6 unresolved (6 bot) · 0 days stale · ✋ changes requested · 🔴 CI failing · areas: fallback · Policy: waiting-author
  - Top thread: "🟡 Suggestion: The documented 35 DASH leaves less than the suite minimum after bootstrap**" — 0 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/dash-evo-tool#1117 fix(identity): choose signing keys Platform accepts on protocol 14](https://github.com/dashpay/dash-evo-tool/pull/1117) — 4 unresolved (4 bot) · 3 days stale · ✋ changes requested · 🔴 CI failing · areas: fallback · Policy: waiting-bots
  - Top thread: "🟡 Suggestion: The signing selection type can report a usable key without containing one**" — 3 days old
  - Blocker: coderabbitai has not reported for the current head
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/dash-evo-tool#1174 feat(skills): mark defects in visual-diff galleries](https://github.com/dashpay/dash-evo-tool/pull/1174) — 4 unresolved (4 bot) · 3 days stale · ✋ changes requested · 🔴 CI failing · areas: fallback · Policy: waiting-author
  - Top thread: "🟡 Suggestion: The skill tells reviewers to omit unchanged regions and banners that show confirmed defects**" — 3 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/dash-evo-tool#1176 feat(tokens): add guided token creation with complete review](https://github.com/dashpay/dash-evo-tool/pull/1176) — 5 unresolved (5 bot) · 0 days stale · ✋ changes requested · 🔴 CI failing · areas: fallback · Policy: waiting-author
  - Top thread: "🟡 Suggestion: The offline save API accepts a pending record relabeled as confirmed**" — 0 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/dash-evo-tool#1240 feat(send): clearer confirmation and a steady form width](https://github.com/dashpay/dash-evo-tool/pull/1240) — 3 unresolved (3 bot) · 3 days stale · ✋ changes requested · 🔴 CI failing · areas: fallback · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Opening a batch confirmation reloads wallet and identity data for every recipient**" — 3 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/dash-evo-tool#1202 feat(boot): ask an AI assistant about a startup failure](https://github.com/dashpay/dash-evo-tool/pull/1202) — 1 unresolved (1 human) · 2 days stale · ✋ changes requested · 🔴 CI failing · areas: fallback · Policy: waiting-author
  - Top thread: "Create a separate "ask ai" component that will be in future used for other use cases. The component should be a dialog w…" — 2 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/dash-evo-tool#1249 fix(dashpay): cache DashPay profiles normalized like background sync](https://github.com/dashpay/dash-evo-tool/pull/1249) — 2 unresolved (2 bot) · 0 days stale · ✋ changes requested · 🔴 CI failing · areas: fallback · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Manual refresh caches an empty profile that background sync clears**" — 0 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/dash-evo-tool#1206 fix(boot): show startup panics and window failures instead of closing](https://github.com/dashpay/dash-evo-tool/pull/1206) — 1 unresolved (1 bot) · 4 days stale · areas: fallback · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Startup tests pass even if the native alert is shown at the wrong time**" — 4 days old
  - Blocker: thepastaclaw left review threads unresolved; resolve them
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/dash-evo-tool#1243 fix(identity): hint that locked wallets are not searched for identities](https://github.com/dashpay/dash-evo-tool/pull/1243) — 1 unresolved (1 bot) · 1 days stale · ✋ changes requested · 🔴 CI failing · areas: fallback · Policy: waiting-author
  - Top thread: "🔴 Blocking: Unlocking from the hint while disconnected can hide it without searching the wallet**" — 1 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#6035 fix(platform-wallet): keep pre-derived lost funding rows ineligible after restart](https://github.com/dashpay/platform/pull/6035) — 1 unresolved (1 bot) · 2 days stale · ✋ changes requested · 🔴 CI failing · areas: rs-platform-wallet · Policy: waiting-author
  - Top thread: "🔴 Blocking: A corrupt stored index can leave the wallet unable to issue invitations after restart**" — 2 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them

<a id="claudius-maginificent-deferred"></a>
#### Deferred (3)
- [dashpay/platform#3549 test(platform-wallet): e2e framework + full test suite — triage pins, Found-*/PA-* guards, fail-closed persist, Stage-2 merge](https://github.com/dashpay/platform/pull/3549) — 📝 draft · ⏸ deferred · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk, kotlin-sdk, github, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5384 fix(platform-wallet)!: persist chain truncations so a stored wallet follows a reorg](https://github.com/dashpay/platform/pull/5384) — ⏸ deferred · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk, kotlin-sdk · Policy: waiting-author
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5417 fix(platform-wallet): own per-wallet transaction accounting in Rust for SwiftData](https://github.com/dashpay/platform/pull/5417) — 📝 draft · ⏸ deferred · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk

<a id="claudius-maginificent-draft"></a>
#### Draft (7)
- [dashpay/tenderdash#1651 ci(docker): build release images before publication and promote by digest](https://github.com/dashpay/tenderdash/pull/1651) — 5 unresolved (5 bot) · 0 days stale · ✋ changes requested · 🔴 CI failing · 📝 draft · areas: github, fallback · Policy: draft
  - Top thread: "🟡 Suggestion: SHA or branch staging can succeed even when its image cannot be promoted**" — 0 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5707 fix(sdk): recover the nonce cache when unconfirmed nonces leave the accepted window](https://github.com/dashpay/platform/pull/5707) — 1 unresolved (1 bot) · 24 days stale · ✋ changes requested · 🔴 CI failing · 📝 draft · areas: rust-sdk · Policy: draft
  - Top thread: "🔴 Blocking: Do not reissue nonces belonging to outstanding operations**" — 24 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/tenderdash#1578 fix(statesync): refuse surplus light block responses without blocking](https://github.com/dashpay/tenderdash/pull/1578) — 1 unresolved (1 bot) · 1 days stale · 📝 draft · areas: fallback · Policy: draft
  - Top thread: "🟡 Suggestion: A duplicate light-block reply can go unreported after the caller takes the first answer**" — 1 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/dash-evo-tool#1197 fix(send): offer a private-balance Max the network accepts](https://github.com/dashpay/dash-evo-tool/pull/1197) — 📝 draft · areas: fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#6295 fix(platform-wallet)!: discover identities whose master key is ECDSA_HASH160](https://github.com/dashpay/platform/pull/6295) — 📝 draft · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#6346 fix(sdk): reject duplicate identity top-ups and retry chain-lock height lag](https://github.com/dashpay/platform/pull/6346) — 🔴 CI failing · 📝 draft · areas: rs-platform-wallet, rust-sdk, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/tenderdash#1647 refactor(privval)!: remove the socket and gRPC remote signer](https://github.com/dashpay/tenderdash/pull/1647) — 📝 draft · areas: fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot

<a id="claudius-maginificent-stale"></a>
#### Stale (41)
- [dashpay/dash-evo-tool#1178 fix(tokens): keep JSON export responsive when the file chooser stalls](https://github.com/dashpay/dash-evo-tool/pull/1178) — 15 unresolved (7 CodeRabbit, 8 bot) · 6 days stale · ✋ changes requested · 📝 draft · 🐢 targets feat/app-design-system · areas: fallback
  - Top thread: "🟡 Suggestion: Exercise a publication failure after staging succeeds**" — 6 days old
- [dashpay/dash-evo-tool#1177 feat(ui): align app-wide styles with the token proposal](https://github.com/dashpay/dash-evo-tool/pull/1177) — 13 unresolved (13 bot) · 8 days stale · ⚠ merge conflict · ✋ changes requested · 📝 draft · 🐢 targets feat/token-creator-workspace · areas: fallback
  - Top thread: "✅ **Resolved** at \`0993a6af\`; see the replies below." — 8 days old
- [dashpay/dash-evo-tool#1108 feat(documents): recognise indexOnly document types and delete their entries by values](https://github.com/dashpay/dash-evo-tool/pull/1108) — 3 unresolved (3 bot) · 34 days stale · ✋ changes requested · 📝 draft · 🐢 targets feat/document-optional-token-cost · areas: fallback
  - Top thread: "🟡 Suggestion: Make the non-deletable error actionable**" — 34 days old
- [dashpay/dash-evo-tool#1179 fix(tokens): retain the creator when opening a registered contract](https://github.com/dashpay/dash-evo-tool/pull/1179) — 4 unresolved (4 bot) · 6 days stale · ✋ changes requested · 📝 draft · 🐢 targets fix/token-json-export · areas: fallback
  - Top thread: "🟡 Suggestion: Assert that the preview displays fetched contract details**" — 6 days old
- [dashpay/platform#5546 fix(platform-wallet): support HASH160 DashPay contact signing keys](https://github.com/dashpay/platform/pull/5546) — 2 unresolved (2 bot) · 27 days stale · ⚠ merge conflict · 📝 draft · 🐢 targets feat/platform-wallet-public-signing-key-selection · areas: rs-drive-abci, dpp, rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk, fallback
  - Top thread: "🟡 Suggestion: Do not expose an unchecked signer-capability override in the public constructor**" — 27 days old
- [dashpay/platform#5981 refactor(platform-wallet-storage): drop the unreachable rehydration gap-refill guard](https://github.com/dashpay/platform/pull/5981) — 2 unresolved (2 bot) · 8 days stale · ✋ changes requested · 📝 draft · 🐢 targets refactor/platform-node-pool-single-owner · areas: wallet-storage
  - Top thread: "🔴 Blocking: Preserve script-based ownership after a failed refill**" — 8 days old
- [dashpay/dash-evo-tool#1124 feat(documents): let the user skip an optional token cost](https://github.com/dashpay/dash-evo-tool/pull/1124) — 1 unresolved (1 bot) · 31 days stale · ⚠ merge conflict · ✋ changes requested · 📝 draft · 🐢 targets fix/protocol14-signing-key-selection · areas: fallback
  - Top thread: "🔴 Blocking: Show the credit-fee outcome when an optional token cost is disabled**" — 31 days old
- [dashpay/tenderdash#1641 fix(rpc): release upstream subscriptions orphaned by a resubscribe](https://github.com/dashpay/tenderdash/pull/1641) — 3 unresolved (3 bot) · 1 days stale · ✋ changes requested · 📝 draft · 🐢 targets fix/light-proxy-ws-worker-lifetime · areas: fallback
  - Top thread: "🟡 Suggestion: The replacement-first test can see the resend arrive first**" — 1 days old
- [dashpay/tenderdash#1642 fix(rpc): keep the event loop running during an error-triggered resubscribe](https://github.com/dashpay/tenderdash/pull/1642) — 2 unresolved (2 bot) · 1 days stale · 🔴 CI failing · 📝 draft · 🐢 targets fix/ws-resubscribe-orphan · areas: fallback
  - Top thread: "🟡 Suggestion: The error-and-drop test passes even if event delivery never resumes after reconnect**" — 1 days old
- [dashpay/platform#5647 feat: expose the Rust identity key layout to FFI, Swift and Kotlin hosts](https://github.com/dashpay/platform/pull/5647) — 1 unresolved (1 bot) · 4 days stale · ✋ changes requested · 🔴 CI failing · 📝 draft · 🐢 targets fix/platform-wallet-dip11-identity-key-layout · areas: rs-platform-wallet-ffi, swift-sdk, kotlin-sdk
  - Top thread: "🟡 Suggestion: Kotlin reports a failed protocol-version lookup as non-retryable**" — 4 days old
- [dashpay/platform#5661 feat: decide which identity keys may be disabled in the wallet, not in the mobile apps](https://github.com/dashpay/platform/pull/5661) — 1 unresolved (1 bot) · 3 days stale · 🔴 CI failing · 📝 draft · 🐢 targets fix/platform-wallet-dip11-key-rules · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk
  - Top thread: "🟡 Suggestion: The last-TRANSFER test passes even if submission checks the pre-refresh key set**" — 3 days old
- [dashpay/tenderdash#1639 fix(light): retire WebSocket forwarders with their subscriptions](https://github.com/dashpay/tenderdash/pull/1639) — 1 unresolved (1 bot) · 1 days stale · ✋ changes requested · 📝 draft · 🐢 targets fix/base-service-stop · areas: fallback
  - Top thread: "🔴 Blocking: Retrying a timed-out unsubscribe reports success without freeing upstream subscription slots**" — 1 days old
- [dashpay/dash-evo-tool#1120 feat(identity): add keys with spending limits and raise key limits](https://github.com/dashpay/dash-evo-tool/pull/1120) — ⚠ merge conflict · 📝 draft · 🐢 targets fix/protocol14-signing-key-selection · areas: fallback
- [dashpay/dash-evo-tool#1121 feat(contracts): view and claim contract fee pots](https://github.com/dashpay/dash-evo-tool/pull/1121) — ⚠ merge conflict · 📝 draft · 🐢 targets fix/protocol14-signing-key-selection · areas: fallback
- [dashpay/dash-evo-tool#1122 feat(documents): agree to document action fees before broadcasting](https://github.com/dashpay/dash-evo-tool/pull/1122) — ⚠ merge conflict · 📝 draft · 🐢 targets fix/protocol14-signing-key-selection · areas: fallback
- [dashpay/dash-evo-tool#1123 feat(identity): add authentication keys bound to a contract](https://github.com/dashpay/dash-evo-tool/pull/1123) — 📝 draft · 🐢 targets feat/identity-key-limits · areas: fallback
- [dashpay/dash-evo-tool#1137 fix(identity): unlock the wallet that holds the chosen signing key](https://github.com/dashpay/dash-evo-tool/pull/1137) — ⚠ merge conflict · 📝 draft · 🐢 targets fix/protocol14-signing-key-selection · areas: fallback
- [dashpay/dash-evo-tool#1192 feat(identity): claim token rewards from the identity hub](https://github.com/dashpay/dash-evo-tool/pull/1192) — ⚠ merge conflict · 📝 draft · 🐢 targets feat/identity-hub-tokens · areas: fallback
- [dashpay/dash-evo-tool#1195 fix(dashpay): recover contact requests this device could not record](https://github.com/dashpay/dash-evo-tool/pull/1195) — 📝 draft · 🐢 targets fix/dashpay-contact-info-keys · areas: fallback
- [dashpay/dash-evo-tool#1212 fix(tokens): close the confirmation when a refresh moves the signing key](https://github.com/dashpay/dash-evo-tool/pull/1212) — ⚠ merge conflict · 📝 draft · 🐢 targets fix/protocol14-signing-key-selection · areas: fallback
- [dashpay/dash-evo-tool#1215 feat(tokens): refresh token balances on request](https://github.com/dashpay/dash-evo-tool/pull/1215) — ⚠ merge conflict · 📝 draft · 🐢 targets refactor/identity-token-model · areas: fallback
- [dashpay/dash-evo-tool#1216 feat(identity): Tokens tab in the identity hub](https://github.com/dashpay/dash-evo-tool/pull/1216) — 📝 draft · 🐢 targets feat/token-balance-refresh · areas: fallback
- [dashpay/dash-evo-tool#1217 feat(identity): token detail page in the identity hub](https://github.com/dashpay/dash-evo-tool/pull/1217) — 📝 draft · 🐢 targets feat/identity-hub-tokens-tab · areas: fallback
- [dashpay/dash-evo-tool#1218 feat(identity): tokens card on the identity Home tab](https://github.com/dashpay/dash-evo-tool/pull/1218) — 📝 draft · 🐢 targets feat/identity-hub-token-details · areas: fallback
- [dashpay/dash-evo-tool#1219 fix(tokens): re-importing a dismissed token restores its tracking](https://github.com/dashpay/dash-evo-tool/pull/1219) — 📝 draft · 🐢 targets feat/identity-hub-tokens-tab · areas: fallback
- [dashpay/dash-evo-tool#1222 fix(dashpay): drop the cached contact QR image with the code](https://github.com/dashpay/dash-evo-tool/pull/1222) — 📝 draft · 🐢 targets fix/protocol14-signing-key-selection · areas: fallback
- [dashpay/platform#5660 refactor(dpp): own the contract-bounds verdict for a document create](https://github.com/dashpay/platform/pull/5660) — 📝 draft · 🐢 targets fix/platform-wallet-contact-info-hash160 · areas: rs-drive-abci, dpp, rs-platform-wallet
- [dashpay/platform#5782 fix(wallet-storage): restore Platform and Core from one wallet snapshot](https://github.com/dashpay/platform/pull/5782) — 📝 draft · 🐢 targets fix/persist-drain-conflict-outcomes · areas: wallet-storage
- [dashpay/platform#5783 fix(wallet-storage): reconcile historical AssetUnlock identities](https://github.com/dashpay/platform/pull/5783) — 📝 draft · 🐢 targets fix/funding-issuance-floor-across-reload · areas: rs-platform-wallet, wallet-storage
- [dashpay/platform#5787 fix(platform-wallet): classify asset locks funding foreign credits as outgoing](https://github.com/dashpay/platform/pull/5787) — 📝 draft · 🐢 targets fix/sqlite-platform-address-snapshot · areas: rs-platform-wallet, wallet-storage
- [dashpay/platform#5961 fix(platform-wallet): don't pick a limited key to sign a data contract create or update](https://github.com/dashpay/platform/pull/5961) — 📝 draft · 🐢 targets refactor/document-create-bounds-in-dpp · areas: rs-platform-wallet
- [dashpay/platform#6025 feat(platform-wallet): read the chain-following masternode list with its height](https://github.com/dashpay/platform/pull/6025) — 📝 draft · 🐢 targets feat/mn-key-id-check · areas: rs-platform-wallet
- [dashpay/platform#6026 feat(platform-wallet): fetch the key references of any masternode](https://github.com/dashpay/platform/pull/6026) — 📝 draft · 🐢 targets feat/mn-current-list · areas: rs-platform-wallet
- [dashpay/platform#6027 feat(platform-wallet): return the proved block height with a withdrawal's remaining balance](https://github.com/dashpay/platform/pull/6027) — ⚠ merge conflict · 📝 draft · 🐢 targets feat/mn-key-reference · areas: rs-platform-wallet, rs-platform-wallet-ffi
- [dashpay/platform#6028 feat(platform-wallet): withdraw masternode credits with a wallet master-key provider or a host signer](https://github.com/dashpay/platform/pull/6028) — 📝 draft · 🐢 targets feat/mn-withdrawal-height · areas: rs-platform-wallet, rs-platform-wallet-ffi
- [dashpay/platform#6030 chore: update rust-dashcore for masternode lookup and lists](https://github.com/dashpay/platform/pull/6030) — ⚠ merge conflict · 🔴 CI failing · 📝 draft · 🐢 targets feat/mn-withdrawal-key-source · areas: dpp, rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk, js-wasm-sdk, fallback
- [dashpay/platform#6031 feat(platform-wallet): find a wallet's owner or voting key by its hash](https://github.com/dashpay/platform/pull/6031) — 📝 draft · 🐢 targets chore/mn-dashcore-k1-s1 · areas: rs-platform-wallet, rs-platform-wallet-ffi
- [dashpay/platform#6032 refactor(platform-wallet): read the current masternode list from dash-spv](https://github.com/dashpay/platform/pull/6032) — 📝 draft · 🐢 targets feat/mn-provider-key-lookup · areas: rs-platform-wallet, rs-platform-wallet-ffi
- [dashpay/platform#6036 refactor(platform-wallet): drop always-zero Core address balance](https://github.com/dashpay/platform/pull/6036) — 📝 draft · 🐢 targets fix/funding-pool-lost-row-in-window · areas: rs-platform-wallet-ffi, swift-sdk, kotlin-sdk
- [dashpay/platform#6037 refactor(platform-wallet): one owner for platform-node pool entries](https://github.com/dashpay/platform/pull/6037) — 📝 draft · 🐢 targets refactor/core-address-entry-drop-balance · areas: rs-platform-wallet, wallet-storage
- [dashpay/tenderdash#1627 fix(p2p): bound the simple priority queue, honor channel priorities and count drops](https://github.com/dashpay/tenderdash/pull/1627) — 📝 draft · 🐢 targets fix/consensus-serve-polka-block · areas: fallback

<a id="claudius-maginificent-clean"></a>
#### Clean (1)
- [dashpay/tenderdash#1600 fix(consensus): deliver and process a polka block for a validator left in its round](https://github.com/dashpay/tenderdash/pull/1600) — areas: fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required

<a id="lklimek"></a>
### @lklimek
<a id="lklimek-open"></a>
#### Open (26)
- [dashpay/platform#5210 feat(wallet-storage)!: restore complete Core wallet snapshots from SQLite](https://github.com/dashpay/platform/pull/5210) — 5 unresolved (5 bot) · 9 days stale · 🔴 CI failing · areas: rs-platform-wallet, wallet-storage, swift-sdk, fallback · Policy: waiting-author
  - Top thread: "🔴 Blocking: Make one-time funding-key consumption survive row deletion**" — 9 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5528 fix(platform-wallet): retain asset-lock recovery metadata through funding conflicts](https://github.com/dashpay/platform/pull/5528) — 3 unresolved (3 bot) · 36 days stale · ⚠ merge conflict · ✋ changes requested · 📝 draft · 🐢 targets feat/sqlite-wallet-restore · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk, kotlin-sdk
  - Top thread: "🟡 Suggestion: Retain diagnostics for malformed FFI operation snapshots**" — 36 days old
- [dashpay/dash-evo-tool#1253 fix(identity): tell the user when wallet identities cannot be searched before sync](https://github.com/dashpay/dash-evo-tool/pull/1253) — 10 unresolved (2 CodeRabbit, 8 bot) · 0 days stale · areas: fallback · Policy: waiting-author
  - Top thread: "&lt;picture&gt;&lt;img src="https://storage.googleapis.com/coderabbit_public_assets/review-comment-icons-v1/amber-16.svg" width="…" — 0 days old
  - Blocker: coderabbitai left review threads unresolved; resolve them
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/dash-evo-tool#1255 fix(storage): make Clear Database clear everything or nothing](https://github.com/dashpay/dash-evo-tool/pull/1255) — 10 unresolved (2 CodeRabbit, 8 bot) · 0 days stale · ✋ changes requested · 🔴 CI failing · areas: fallback · Policy: waiting-author
  - Top thread: "&lt;picture&gt;&lt;img src="https://storage.googleapis.com/coderabbit_public_assets/review-comment-icons-v1/amber-16.svg" width="…" — 0 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: coderabbitai left review threads unresolved; resolve them
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/tenderdash#1593 feat(consensus)!: specify full consensus parameter commitment and remove app version parameter](https://github.com/dashpay/tenderdash/pull/1593) — 5 unresolved (5 bot) · 1 days stale · ⚠ merge conflict · 📝 draft · areas: fallback · Policy: draft
  - Top thread: "🟡 Suggestion: Dashmate shows the startup protocol version after an in-place upgrade**" — 1 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#6349 fix(dashmate): keep a node's own ID out of its Tenderdash seeds](https://github.com/dashpay/platform/pull/6349) — 4 unresolved (4 bot) · 0 days stale · ✋ changes requested · 🔴 CI failing · 🐢 targets v5.1-dev · areas: dashmate · Policy: waiting-author
  - Top thread: "🟡 Suggestion: A forced hard reset can abort if a config being migrated has no seed list**" — 0 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/dash-evo-tool#1230 fix(tokens): never ask to pay again for a registration Platform already confirmed](https://github.com/dashpay/dash-evo-tool/pull/1230) — 2 unresolved (2 bot) · 1 days stale · ✋ changes requested · 🔴 CI failing · areas: fallback · Policy: waiting-author
  - Top thread: "🟡 Suggestion: The recovery message blames another operation when the registration itself was rejected**" — 1 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/dash-evo-tool#1161 fix(ci): isolate release signing and constrain agent write paths](https://github.com/dashpay/dash-evo-tool/pull/1161) — 1 unresolved (1 bot) · 4 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · areas: github, fallback · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Release policy tests accept older or floating artifact downloader refs**" — 4 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/platform#6024 feat(platform-wallet): check a masternode key by its hash without its secret](https://github.com/dashpay/platform/pull/6024) — 1 unresolved (1 bot) · 3 days stale · areas: rs-platform-wallet · Policy: waiting-self-review
  - Top thread: "🟡 Suggestion: A registered Platform node ID looks like the wrong key in Core wire order**" — 3 days old
  - Blocker: Author must post /self-reviewed 18f90bbfd92650b5e717cfaca467e7981112def0
- [dashpay/platform#5519 refactor: remove DPP raw secp256k1 dependency](https://github.com/dashpay/platform/pull/5519) — 1 unresolved (1 bot) · 0 days stale · areas: rs-drive-abci, dpp, rs-platform-wallet, rs-platform-wallet-ffi, rust-sdk, rust-sdk-ffi, js-wasm-sdk, fallback · Policy: ready-for-human
  - Top thread: "🟡 Suggestion: Invitation fixtures still tie ordinary public-key derivation to the raw ECDH dependency**" — 0 days old
  - Blocker: Human approval or objection resolution is required
- [dashpay/tenderdash#1655 chore(e2e): remove the unused ci network](https://github.com/dashpay/tenderdash/pull/1655) — 1 unresolved (1 bot) · 0 days stale · ✋ changes requested · 🔴 CI failing · 📝 draft · 🐢 targets fix/consensus-single-validator-proposer · areas: fallback
  - Top thread: "🔴 Blocking: The documented manual E2E command cannot find the Dash Core manifest**" — 0 days old
- [dashpay/dash-evo-tool#1228 fix(tokens): show a group proposal to a co-signer whose wallet is locked](https://github.com/dashpay/dash-evo-tool/pull/1228) — 📝 draft · 🐢 targets fix/protocol14-signing-key-selection · areas: fallback
- [dashpay/dash-evo-tool#1229 fix(boot): show a panic while building the first screens instead of closing](https://github.com/dashpay/dash-evo-tool/pull/1229) — 📝 draft · 🐢 targets fix/boot-panic-dialog · ⚠ routing unavailable: no changed-file evidence
- [dashpay/dash-evo-tool#1254 fix(dpns): hold the balance Platform reserves before it admits a name](https://github.com/dashpay/dash-evo-tool/pull/1254) — 📝 draft · areas: fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5220 fix(platform-wallet)!: replay recorded transaction history on load for every persister](https://github.com/dashpay/platform/pull/5220) — 📝 draft · ⏸ deferred · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk, kotlin-sdk, fallback
- [dashpay/platform#6102 fix(platform-wallet): keep invitation issuance floor across reloads](https://github.com/dashpay/platform/pull/6102) — ✋ changes requested · 📝 draft · 🐢 targets chore/storage-drop-unreachable-refill-guard · areas: rs-platform-wallet, rs-platform-wallet-ffi
- [dashpay/platform#6350 chore: make gitleaks pre-commit pass on the existing tree](https://github.com/dashpay/platform/pull/6350) — 📝 draft · 🐢 targets v5.1-dev · areas: swift-sdk, dashmate, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#1416 feat(key-wallet): verify owner authorization of a pending spend](https://github.com/dashpay/rust-dashcore/pull/1416) — ✋ changes requested · 🔴 CI failing · areas: key-wallet, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed f134961bca06d23a70b88f02c7c03efbfd6f5508
- [dashpay/rust-dashcore#1448 fix(dash-spv): keep \`QRInfo\` catch-up across cycles](https://github.com/dashpay/rust-dashcore/pull/1448) — ⚠ merge conflict · 🔴 CI failing · 📝 draft · areas: dash-spv · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#1465 feat(key-wallet): find a provider key by its hash](https://github.com/dashpay/rust-dashcore/pull/1465) — 📝 draft · areas: key-wallet · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#1466 feat(dash-spv): read the masternode list only while it is current](https://github.com/dashpay/rust-dashcore/pull/1466) — 📝 draft · 🐢 targets feat/provider-key-lookup-by-hash · areas: dash-spv
- [dashpay/rust-dashcore#1499 feat(dash-spv)!: add \`InstantLock\` verification API](https://github.com/dashpay/rust-dashcore/pull/1499) — ⚠ merge conflict · 📝 draft · areas: dash-spv · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/tenderdash#1515 fix(service)!: join remaining service workers before shutdown completes](https://github.com/dashpay/tenderdash/pull/1515) — ✋ changes requested · 🔴 CI failing · 📝 draft · 🐢 targets fix/p2p-simple-queue-limits · areas: fallback
- [dashpay/tenderdash#1654 fix(consensus): select a proposer after rotating to a single validator](https://github.com/dashpay/tenderdash/pull/1654) — 📝 draft · 🐢 targets fix/ws-error-resubscribe-hang · areas: fallback
- [dashpay/tenderdash#1656 fix(state): let validators recover quorum public keys after a key-less update](https://github.com/dashpay/tenderdash/pull/1656) — 📝 draft · 🐢 targets master · areas: fallback
- [dashpay/tenderdash#1660 fix(node): skip own node ID in configured bootstrap and persistent peers](https://github.com/dashpay/tenderdash/pull/1660) — 🔴 CI failing · 📝 draft · 🐢 targets master · areas: fallback

<a id="lklimek-needs-action"></a>
#### Needs action (8)
- [dashpay/platform#5210 feat(wallet-storage)!: restore complete Core wallet snapshots from SQLite](https://github.com/dashpay/platform/pull/5210) — 5 unresolved (5 bot) · 9 days stale · 🔴 CI failing · areas: rs-platform-wallet, wallet-storage, swift-sdk, fallback · Policy: waiting-author
  - Top thread: "🔴 Blocking: Make one-time funding-key consumption survive row deletion**" — 9 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/dash-evo-tool#1253 fix(identity): tell the user when wallet identities cannot be searched before sync](https://github.com/dashpay/dash-evo-tool/pull/1253) — 10 unresolved (2 CodeRabbit, 8 bot) · 0 days stale · areas: fallback · Policy: waiting-author
  - Top thread: "&lt;picture&gt;&lt;img src="https://storage.googleapis.com/coderabbit_public_assets/review-comment-icons-v1/amber-16.svg" width="…" — 0 days old
  - Blocker: coderabbitai left review threads unresolved; resolve them
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/dash-evo-tool#1255 fix(storage): make Clear Database clear everything or nothing](https://github.com/dashpay/dash-evo-tool/pull/1255) — 10 unresolved (2 CodeRabbit, 8 bot) · 0 days stale · ✋ changes requested · 🔴 CI failing · areas: fallback · Policy: waiting-author
  - Top thread: "&lt;picture&gt;&lt;img src="https://storage.googleapis.com/coderabbit_public_assets/review-comment-icons-v1/amber-16.svg" width="…" — 0 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: coderabbitai left review threads unresolved; resolve them
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/dash-evo-tool#1230 fix(tokens): never ask to pay again for a registration Platform already confirmed](https://github.com/dashpay/dash-evo-tool/pull/1230) — 2 unresolved (2 bot) · 1 days stale · ✋ changes requested · 🔴 CI failing · areas: fallback · Policy: waiting-author
  - Top thread: "🟡 Suggestion: The recovery message blames another operation when the registration itself was rejected**" — 1 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/dash-evo-tool#1161 fix(ci): isolate release signing and constrain agent write paths](https://github.com/dashpay/dash-evo-tool/pull/1161) — 1 unresolved (1 bot) · 4 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · areas: github, fallback · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Release policy tests accept older or floating artifact downloader refs**" — 4 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/platform#6024 feat(platform-wallet): check a masternode key by its hash without its secret](https://github.com/dashpay/platform/pull/6024) — 1 unresolved (1 bot) · 3 days stale · areas: rs-platform-wallet · Policy: waiting-self-review
  - Top thread: "🟡 Suggestion: A registered Platform node ID looks like the wrong key in Core wire order**" — 3 days old
  - Blocker: Author must post /self-reviewed 18f90bbfd92650b5e717cfaca467e7981112def0
- [dashpay/platform#5519 refactor: remove DPP raw secp256k1 dependency](https://github.com/dashpay/platform/pull/5519) — 1 unresolved (1 bot) · 0 days stale · areas: rs-drive-abci, dpp, rs-platform-wallet, rs-platform-wallet-ffi, rust-sdk, rust-sdk-ffi, js-wasm-sdk, fallback · Policy: ready-for-human
  - Top thread: "🟡 Suggestion: Invitation fixtures still tie ordinary public-key derivation to the raw ECDH dependency**" — 0 days old
  - Blocker: Human approval or objection resolution is required
- [dashpay/rust-dashcore#1416 feat(key-wallet): verify owner authorization of a pending spend](https://github.com/dashpay/rust-dashcore/pull/1416) — ✋ changes requested · 🔴 CI failing · areas: key-wallet, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed f134961bca06d23a70b88f02c7c03efbfd6f5508

<a id="lklimek-ready-for-human"></a>
#### Ready for human (1)
- [dashpay/platform#5519 refactor: remove DPP raw secp256k1 dependency](https://github.com/dashpay/platform/pull/5519) — 1 unresolved (1 bot) · 0 days stale · areas: rs-drive-abci, dpp, rs-platform-wallet, rs-platform-wallet-ffi, rust-sdk, rust-sdk-ffi, js-wasm-sdk, fallback · Policy: ready-for-human
  - Top thread: "🟡 Suggestion: Invitation fixtures still tie ordinary public-key derivation to the raw ECDH dependency**" — 0 days old
  - Blocker: Human approval or objection resolution is required

<a id="lklimek-unresolved-comments"></a>
#### Unresolved Comments (7)
- [dashpay/platform#5210 feat(wallet-storage)!: restore complete Core wallet snapshots from SQLite](https://github.com/dashpay/platform/pull/5210) — 5 unresolved (5 bot) · 9 days stale · 🔴 CI failing · areas: rs-platform-wallet, wallet-storage, swift-sdk, fallback · Policy: waiting-author
  - Top thread: "🔴 Blocking: Make one-time funding-key consumption survive row deletion**" — 9 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/dash-evo-tool#1253 fix(identity): tell the user when wallet identities cannot be searched before sync](https://github.com/dashpay/dash-evo-tool/pull/1253) — 10 unresolved (2 CodeRabbit, 8 bot) · 0 days stale · areas: fallback · Policy: waiting-author
  - Top thread: "&lt;picture&gt;&lt;img src="https://storage.googleapis.com/coderabbit_public_assets/review-comment-icons-v1/amber-16.svg" width="…" — 0 days old
  - Blocker: coderabbitai left review threads unresolved; resolve them
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/dash-evo-tool#1255 fix(storage): make Clear Database clear everything or nothing](https://github.com/dashpay/dash-evo-tool/pull/1255) — 10 unresolved (2 CodeRabbit, 8 bot) · 0 days stale · ✋ changes requested · 🔴 CI failing · areas: fallback · Policy: waiting-author
  - Top thread: "&lt;picture&gt;&lt;img src="https://storage.googleapis.com/coderabbit_public_assets/review-comment-icons-v1/amber-16.svg" width="…" — 0 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: coderabbitai left review threads unresolved; resolve them
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/dash-evo-tool#1230 fix(tokens): never ask to pay again for a registration Platform already confirmed](https://github.com/dashpay/dash-evo-tool/pull/1230) — 2 unresolved (2 bot) · 1 days stale · ✋ changes requested · 🔴 CI failing · areas: fallback · Policy: waiting-author
  - Top thread: "🟡 Suggestion: The recovery message blames another operation when the registration itself was rejected**" — 1 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/dash-evo-tool#1161 fix(ci): isolate release signing and constrain agent write paths](https://github.com/dashpay/dash-evo-tool/pull/1161) — 1 unresolved (1 bot) · 4 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · areas: github, fallback · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Release policy tests accept older or floating artifact downloader refs**" — 4 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/platform#6024 feat(platform-wallet): check a masternode key by its hash without its secret](https://github.com/dashpay/platform/pull/6024) — 1 unresolved (1 bot) · 3 days stale · areas: rs-platform-wallet · Policy: waiting-self-review
  - Top thread: "🟡 Suggestion: A registered Platform node ID looks like the wrong key in Core wire order**" — 3 days old
  - Blocker: Author must post /self-reviewed 18f90bbfd92650b5e717cfaca467e7981112def0
- [dashpay/platform#5519 refactor: remove DPP raw secp256k1 dependency](https://github.com/dashpay/platform/pull/5519) — 1 unresolved (1 bot) · 0 days stale · areas: rs-drive-abci, dpp, rs-platform-wallet, rs-platform-wallet-ffi, rust-sdk, rust-sdk-ffi, js-wasm-sdk, fallback · Policy: ready-for-human
  - Top thread: "🟡 Suggestion: Invitation fixtures still tie ordinary public-key derivation to the raw ECDH dependency**" — 0 days old
  - Blocker: Human approval or objection resolution is required

<a id="lklimek-changes-requested"></a>
#### Changes Requested (1)
- [dashpay/rust-dashcore#1416 feat(key-wallet): verify owner authorization of a pending spend](https://github.com/dashpay/rust-dashcore/pull/1416) — ✋ changes requested · 🔴 CI failing · areas: key-wallet, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed f134961bca06d23a70b88f02c7c03efbfd6f5508

<a id="lklimek-deferred"></a>
#### Deferred (1)
- [dashpay/platform#5220 fix(platform-wallet)!: replay recorded transaction history on load for every persister](https://github.com/dashpay/platform/pull/5220) — 📝 draft · ⏸ deferred · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk, kotlin-sdk, fallback

<a id="lklimek-draft"></a>
#### Draft (5)
- [dashpay/tenderdash#1593 feat(consensus)!: specify full consensus parameter commitment and remove app version parameter](https://github.com/dashpay/tenderdash/pull/1593) — 5 unresolved (5 bot) · 1 days stale · ⚠ merge conflict · 📝 draft · areas: fallback · Policy: draft
  - Top thread: "🟡 Suggestion: Dashmate shows the startup protocol version after an in-place upgrade**" — 1 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/dash-evo-tool#1254 fix(dpns): hold the balance Platform reserves before it admits a name](https://github.com/dashpay/dash-evo-tool/pull/1254) — 📝 draft · areas: fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#1448 fix(dash-spv): keep \`QRInfo\` catch-up across cycles](https://github.com/dashpay/rust-dashcore/pull/1448) — ⚠ merge conflict · 🔴 CI failing · 📝 draft · areas: dash-spv · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#1465 feat(key-wallet): find a provider key by its hash](https://github.com/dashpay/rust-dashcore/pull/1465) — 📝 draft · areas: key-wallet · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#1499 feat(dash-spv)!: add \`InstantLock\` verification API](https://github.com/dashpay/rust-dashcore/pull/1499) — ⚠ merge conflict · 📝 draft · areas: dash-spv · Policy: draft
  - Blocker: Draft PR does not occupy a review slot

<a id="lklimek-stale"></a>
#### Stale (12)
- [dashpay/platform#5528 fix(platform-wallet): retain asset-lock recovery metadata through funding conflicts](https://github.com/dashpay/platform/pull/5528) — 3 unresolved (3 bot) · 36 days stale · ⚠ merge conflict · ✋ changes requested · 📝 draft · 🐢 targets feat/sqlite-wallet-restore · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk, kotlin-sdk
  - Top thread: "🟡 Suggestion: Retain diagnostics for malformed FFI operation snapshots**" — 36 days old
- [dashpay/platform#6349 fix(dashmate): keep a node's own ID out of its Tenderdash seeds](https://github.com/dashpay/platform/pull/6349) — 4 unresolved (4 bot) · 0 days stale · ✋ changes requested · 🔴 CI failing · 🐢 targets v5.1-dev · areas: dashmate · Policy: waiting-author
  - Top thread: "🟡 Suggestion: A forced hard reset can abort if a config being migrated has no seed list**" — 0 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/tenderdash#1655 chore(e2e): remove the unused ci network](https://github.com/dashpay/tenderdash/pull/1655) — 1 unresolved (1 bot) · 0 days stale · ✋ changes requested · 🔴 CI failing · 📝 draft · 🐢 targets fix/consensus-single-validator-proposer · areas: fallback
  - Top thread: "🔴 Blocking: The documented manual E2E command cannot find the Dash Core manifest**" — 0 days old
- [dashpay/dash-evo-tool#1228 fix(tokens): show a group proposal to a co-signer whose wallet is locked](https://github.com/dashpay/dash-evo-tool/pull/1228) — 📝 draft · 🐢 targets fix/protocol14-signing-key-selection · areas: fallback
- [dashpay/dash-evo-tool#1229 fix(boot): show a panic while building the first screens instead of closing](https://github.com/dashpay/dash-evo-tool/pull/1229) — 📝 draft · 🐢 targets fix/boot-panic-dialog · ⚠ routing unavailable: no changed-file evidence
- [dashpay/platform#6102 fix(platform-wallet): keep invitation issuance floor across reloads](https://github.com/dashpay/platform/pull/6102) — ✋ changes requested · 📝 draft · 🐢 targets chore/storage-drop-unreachable-refill-guard · areas: rs-platform-wallet, rs-platform-wallet-ffi
- [dashpay/platform#6350 chore: make gitleaks pre-commit pass on the existing tree](https://github.com/dashpay/platform/pull/6350) — 📝 draft · 🐢 targets v5.1-dev · areas: swift-sdk, dashmate, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#1466 feat(dash-spv): read the masternode list only while it is current](https://github.com/dashpay/rust-dashcore/pull/1466) — 📝 draft · 🐢 targets feat/provider-key-lookup-by-hash · areas: dash-spv
- [dashpay/tenderdash#1515 fix(service)!: join remaining service workers before shutdown completes](https://github.com/dashpay/tenderdash/pull/1515) — ✋ changes requested · 🔴 CI failing · 📝 draft · 🐢 targets fix/p2p-simple-queue-limits · areas: fallback
- [dashpay/tenderdash#1654 fix(consensus): select a proposer after rotating to a single validator](https://github.com/dashpay/tenderdash/pull/1654) — 📝 draft · 🐢 targets fix/ws-error-resubscribe-hang · areas: fallback
- [dashpay/tenderdash#1656 fix(state): let validators recover quorum public keys after a key-less update](https://github.com/dashpay/tenderdash/pull/1656) — 📝 draft · 🐢 targets master · areas: fallback
- [dashpay/tenderdash#1660 fix(node): skip own node ID in configured bootstrap and persistent peers](https://github.com/dashpay/tenderdash/pull/1660) — 🔴 CI failing · 📝 draft · 🐢 targets master · areas: fallback

<a id="lklimek-ready-for-review"></a>
#### Ready for Review (2)
- [dashpay/platform#6249 feat(platform)!: subscribe to committed state transitions matching document and address filters](https://github.com/dashpay/platform/pull/6249) — by @QuantumExplorer · areas: rs-drive, rs-drive-abci, rust-sdk, dashmate, rust-dapi, github, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 9a4d02bdc3f9800b16e52b0546dffc23af9ddb8d
- [dashpay/tenderdash#1600 fix(consensus): deliver and process a polka block for a validator left in its round](https://github.com/dashpay/tenderdash/pull/1600) — by @Claudius-Maginificent · areas: fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required

<a id="quantumexplorer"></a>
### @QuantumExplorer
<a id="quantumexplorer-open"></a>
#### Open (23)
- [dashpay/platform#5750 feat(platform)!: random token transfer and mint amounts](https://github.com/dashpay/platform/pull/5750) — 14 unresolved (14 bot) · 23 days stale · 🔴 CI failing · 🐢 targets claude/platform-tenderdash-randomness-149e4e · areas: rs-drive, dpp, rs-platform-wallet, rust-sdk, rust-sdk-ffi, fallback
  - Top thread: "✅ **Resolved** at \`f8fcc8fd\`; see the replies below." — 23 days old
- [dashpay/platform#5749 feat(platform)!: sys.random.uniform document properties](https://github.com/dashpay/platform/pull/5749) — 9 unresolved (9 bot) · 23 days stale · areas: rs-drive, rs-drive-abci, dpp, fallback · Policy: waiting-author
  - Top thread: "✅ **Resolved** at \`0ead2b47\`; see the replies below." — 23 days old
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5753 feat(drive-abci)!: apply block randomness through the ApplyBlockRandomness ABCI method](https://github.com/dashpay/platform/pull/5753) — 7 unresolved (7 bot) · 23 days stale · ✋ changes requested · 🔴 CI failing · 🐢 targets claude/random-token-amounts · areas: rs-drive, rs-drive-abci, dpp, fallback
  - Top thread: "🟡 Suggestion: Exercise the protocol upgrade with signed commits**" — 23 days old
- [dashpay/grovedb#1077 PERF-07-F1: the commit of a block measured on both engines, what a long commit waits for, and one command for the reference tables](https://github.com/dashpay/grovedb/pull/1077) — 12 unresolved (12 bot) · 0 days stale · ✋ changes requested · 🔴 CI failing · 🐢 targets feat/LigninDB · areas: fallback
  - Top thread: "✅ **Resolved** at \`d4300632\`; see the replies below." — 0 days old
- [dashpay/platform#5529 feat(platform)!: token marketplace listings (PV15)](https://github.com/dashpay/platform/pull/5529) — 2 unresolved (2 bot) · 32 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · areas: rs-drive, rs-drive-abci, dpp, rs-platform-wallet, swift-sdk, kotlin-sdk, fallback · Policy: waiting-author
  - Top thread: "🔴 Blocking: Version the consensus order-ID derivation through platform-version tables**" — 32 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/grovedb#1078 GROVE-13: the cost rules of stop points and finalize, under a grove version 6 that is not listed](https://github.com/dashpay/grovedb/pull/1078) — 6 unresolved (6 bot) · 0 days stale · 🔴 CI failing · 🐢 targets feat/LigninDB · areas: fallback
  - Top thread: "⏭️ **Deferred** (maintainer decision; not blocking) at \`464cacc8\`; see the replies below." — 0 days old
- [dashpay/platform#6328 feat(platform)!: raise the largest withdrawal to 4000 Dash in protocol version 16](https://github.com/dashpay/platform/pull/6328) — 5 unresolved (5 bot) · 1 days stale · ✋ changes requested · 🔴 CI failing · areas: rs-drive-abci, dpp, rs-platform-wallet, js-wasm-sdk, fallback · Policy: waiting-author
  - Top thread: "🔴 Blocking: An accepted 4,000-Dash withdrawal can leave the sender's funds stuck even with an empty queue**" — 1 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5530 feat(platform)!: token marketplace offers (PV15)](https://github.com/dashpay/platform/pull/5530) — 1 unresolved (1 bot) · 27 days stale · ✋ changes requested · 🔴 CI failing · 🐢 targets claude/token-market-listings · areas: rs-drive, rs-drive-abci, dpp, rs-platform-wallet, fallback
  - Top thread: "🔴 Blocking: Bound bid escrow by a buyer-signed credit limit**" — 27 days old
- [dashpay/tenderdash#1582 feat(abci)!: add the ApplyBlockRandomness method and commit beacon fields](https://github.com/dashpay/tenderdash/pull/1582) — 1 unresolved (1 bot) · 20 days stale · areas: fallback · Policy: waiting-author
  - Top thread: "✅ **Resolved** at \`cd6b4ec8\`; see the replies below." — 20 days old
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/grovedb#1076 TXN-11: a Merk's stale hashes are made inside a write transaction, and its root hash given before the commit](https://github.com/dashpay/grovedb/pull/1076) — 3 unresolved (3 bot) · 0 days stale · 🐢 targets feat/LigninDB · areas: fallback
  - Top thread: "🟡 Suggestion: A new hash dependency can escape scope claims if only the tree walk is updated**" — 0 days old
- [dashpay/tenderdash#1597 fix(p2p): trim the simple priority queue below its limit](https://github.com/dashpay/tenderdash/pull/1597) — 1 unresolved (1 bot) · 18 days stale · ⚠ merge conflict · areas: fallback · Policy: waiting-self-review
  - Top thread: "🟡 Suggestion: Count the larger overflow batches**" — 18 days old
  - Blocker: Author must post /self-reviewed fb3e02bfc772facdefade06c2eeca6743e23e329
- [dashpay/grovedb#1075 OPS-03-F1: EitherEngine opens a RocksDB database under a name ending in .creating](https://github.com/dashpay/grovedb/pull/1075) — 🐢 targets feat/LigninDB · areas: fallback
- [dashpay/platform#4730 feat(platform)!: delta-based data contract update transition for protocol version 15](https://github.com/dashpay/platform/pull/4730) — ⚠ merge conflict · 🔴 CI failing · 📝 draft · areas: rs-drive, rs-drive-abci, dpp, rs-platform-wallet, rust-sdk, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#4776 feat(platform-wallet)!: let contract updates clear the description through the FFI](https://github.com/dashpay/platform/pull/4776) — ⚠ merge conflict · 🔴 CI failing · 🐢 targets feat/delta-contract-update-pv15 · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk
- [dashpay/platform#4986 fix(drive-abci)!: pay every reward share of a masternode and credit each identity once in the epoch payout (PV15)](https://github.com/dashpay/platform/pull/4986) — ⚠ merge conflict · 📝 draft · areas: rs-drive-abci, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5531 feat(platform)!: record token marketplace events in token history (PV15)](https://github.com/dashpay/platform/pull/5531) — 🔴 CI failing · 🐢 targets claude/token-market-offers · areas: rs-drive, rs-drive-abci, dpp, system-contracts, fallback
- [dashpay/platform#5532 feat(platform)!: destroy a frozen identity's escrowed listings with its funds (PV15)](https://github.com/dashpay/platform/pull/5532) — 🔴 CI failing · 🐢 targets claude/token-market-history · areas: rs-drive, rs-drive-abci, fallback
- [dashpay/platform#5736 feat(platform)!: add a RedPallas signer key type with its own purpose (PV15)](https://github.com/dashpay/platform/pull/5736) — ⚠ merge conflict · 📝 draft · areas: rs-drive, rs-drive-abci, dpp, rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, rust-sdk-ffi, kotlin-sdk, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5751 feat(sdk): random token amounts in wasm-sdk and js-evo-sdk](https://github.com/dashpay/platform/pull/5751) — 🐢 targets claude/random-token-amounts · areas: js-wasm-sdk
- [dashpay/platform#6249 feat(platform)!: subscribe to committed state transitions matching document and address filters](https://github.com/dashpay/platform/pull/6249) — areas: rs-drive, rs-drive-abci, rust-sdk, dashmate, rust-dapi, github, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 9a4d02bdc3f9800b16e52b0546dffc23af9ddb8d
- [dashpay/platform#6282 chore: add a review-rounds AI skill and list the AI skills in the book](https://github.com/dashpay/platform/pull/6282) — ✋ changes requested · areas: github, claude, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 63fc67baa1bb749d6bd583c6ebbc66afb64b69f9
- [dashpay/tenderdash#1583 feat(consensus)!: recover a beacon from prevotes and sign precommits over the randomness-applied app hash](https://github.com/dashpay/tenderdash/pull/1583) — 🔴 CI failing · 📝 draft · areas: fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/tenderdash#1584 fix(statesync): restore LastAppHash of the snapshot height, not the next one](https://github.com/dashpay/tenderdash/pull/1584) — areas: fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 23c1443e4077c7c7dfedf00f9446c762e669660f

<a id="quantumexplorer-needs-action"></a>
#### Needs action (6)
- [dashpay/platform#5749 feat(platform)!: sys.random.uniform document properties](https://github.com/dashpay/platform/pull/5749) — 9 unresolved (9 bot) · 23 days stale · areas: rs-drive, rs-drive-abci, dpp, fallback · Policy: waiting-author
  - Top thread: "✅ **Resolved** at \`0ead2b47\`; see the replies below." — 23 days old
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5529 feat(platform)!: token marketplace listings (PV15)](https://github.com/dashpay/platform/pull/5529) — 2 unresolved (2 bot) · 32 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · areas: rs-drive, rs-drive-abci, dpp, rs-platform-wallet, swift-sdk, kotlin-sdk, fallback · Policy: waiting-author
  - Top thread: "🔴 Blocking: Version the consensus order-ID derivation through platform-version tables**" — 32 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#6328 feat(platform)!: raise the largest withdrawal to 4000 Dash in protocol version 16](https://github.com/dashpay/platform/pull/6328) — 5 unresolved (5 bot) · 1 days stale · ✋ changes requested · 🔴 CI failing · areas: rs-drive-abci, dpp, rs-platform-wallet, js-wasm-sdk, fallback · Policy: waiting-author
  - Top thread: "🔴 Blocking: An accepted 4,000-Dash withdrawal can leave the sender's funds stuck even with an empty queue**" — 1 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/tenderdash#1582 feat(abci)!: add the ApplyBlockRandomness method and commit beacon fields](https://github.com/dashpay/tenderdash/pull/1582) — 1 unresolved (1 bot) · 20 days stale · areas: fallback · Policy: waiting-author
  - Top thread: "✅ **Resolved** at \`cd6b4ec8\`; see the replies below." — 20 days old
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/tenderdash#1597 fix(p2p): trim the simple priority queue below its limit](https://github.com/dashpay/tenderdash/pull/1597) — 1 unresolved (1 bot) · 18 days stale · ⚠ merge conflict · areas: fallback · Policy: waiting-self-review
  - Top thread: "🟡 Suggestion: Count the larger overflow batches**" — 18 days old
  - Blocker: Author must post /self-reviewed fb3e02bfc772facdefade06c2eeca6743e23e329
- [dashpay/platform#6282 chore: add a review-rounds AI skill and list the AI skills in the book](https://github.com/dashpay/platform/pull/6282) — ✋ changes requested · areas: github, claude, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 63fc67baa1bb749d6bd583c6ebbc66afb64b69f9

<a id="quantumexplorer-unresolved-comments"></a>
#### Unresolved Comments (5)
- [dashpay/platform#5749 feat(platform)!: sys.random.uniform document properties](https://github.com/dashpay/platform/pull/5749) — 9 unresolved (9 bot) · 23 days stale · areas: rs-drive, rs-drive-abci, dpp, fallback · Policy: waiting-author
  - Top thread: "✅ **Resolved** at \`0ead2b47\`; see the replies below." — 23 days old
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5529 feat(platform)!: token marketplace listings (PV15)](https://github.com/dashpay/platform/pull/5529) — 2 unresolved (2 bot) · 32 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · areas: rs-drive, rs-drive-abci, dpp, rs-platform-wallet, swift-sdk, kotlin-sdk, fallback · Policy: waiting-author
  - Top thread: "🔴 Blocking: Version the consensus order-ID derivation through platform-version tables**" — 32 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#6328 feat(platform)!: raise the largest withdrawal to 4000 Dash in protocol version 16](https://github.com/dashpay/platform/pull/6328) — 5 unresolved (5 bot) · 1 days stale · ✋ changes requested · 🔴 CI failing · areas: rs-drive-abci, dpp, rs-platform-wallet, js-wasm-sdk, fallback · Policy: waiting-author
  - Top thread: "🔴 Blocking: An accepted 4,000-Dash withdrawal can leave the sender's funds stuck even with an empty queue**" — 1 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/tenderdash#1582 feat(abci)!: add the ApplyBlockRandomness method and commit beacon fields](https://github.com/dashpay/tenderdash/pull/1582) — 1 unresolved (1 bot) · 20 days stale · areas: fallback · Policy: waiting-author
  - Top thread: "✅ **Resolved** at \`cd6b4ec8\`; see the replies below." — 20 days old
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/tenderdash#1597 fix(p2p): trim the simple priority queue below its limit](https://github.com/dashpay/tenderdash/pull/1597) — 1 unresolved (1 bot) · 18 days stale · ⚠ merge conflict · areas: fallback · Policy: waiting-self-review
  - Top thread: "🟡 Suggestion: Count the larger overflow batches**" — 18 days old
  - Blocker: Author must post /self-reviewed fb3e02bfc772facdefade06c2eeca6743e23e329

<a id="quantumexplorer-changes-requested"></a>
#### Changes Requested (1)
- [dashpay/platform#6282 chore: add a review-rounds AI skill and list the AI skills in the book](https://github.com/dashpay/platform/pull/6282) — ✋ changes requested · areas: github, claude, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 63fc67baa1bb749d6bd583c6ebbc66afb64b69f9

<a id="quantumexplorer-draft"></a>
#### Draft (4)
- [dashpay/platform#4730 feat(platform)!: delta-based data contract update transition for protocol version 15](https://github.com/dashpay/platform/pull/4730) — ⚠ merge conflict · 🔴 CI failing · 📝 draft · areas: rs-drive, rs-drive-abci, dpp, rs-platform-wallet, rust-sdk, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#4986 fix(drive-abci)!: pay every reward share of a masternode and credit each identity once in the epoch payout (PV15)](https://github.com/dashpay/platform/pull/4986) — ⚠ merge conflict · 📝 draft · areas: rs-drive-abci, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5736 feat(platform)!: add a RedPallas signer key type with its own purpose (PV15)](https://github.com/dashpay/platform/pull/5736) — ⚠ merge conflict · 📝 draft · areas: rs-drive, rs-drive-abci, dpp, rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, rust-sdk-ffi, kotlin-sdk, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/tenderdash#1583 feat(consensus)!: recover a beacon from prevotes and sign precommits over the randomness-applied app hash](https://github.com/dashpay/tenderdash/pull/1583) — 🔴 CI failing · 📝 draft · areas: fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot

<a id="quantumexplorer-stale"></a>
#### Stale (11)
- [dashpay/platform#5750 feat(platform)!: random token transfer and mint amounts](https://github.com/dashpay/platform/pull/5750) — 14 unresolved (14 bot) · 23 days stale · 🔴 CI failing · 🐢 targets claude/platform-tenderdash-randomness-149e4e · areas: rs-drive, dpp, rs-platform-wallet, rust-sdk, rust-sdk-ffi, fallback
  - Top thread: "✅ **Resolved** at \`f8fcc8fd\`; see the replies below." — 23 days old
- [dashpay/platform#5753 feat(drive-abci)!: apply block randomness through the ApplyBlockRandomness ABCI method](https://github.com/dashpay/platform/pull/5753) — 7 unresolved (7 bot) · 23 days stale · ✋ changes requested · 🔴 CI failing · 🐢 targets claude/random-token-amounts · areas: rs-drive, rs-drive-abci, dpp, fallback
  - Top thread: "🟡 Suggestion: Exercise the protocol upgrade with signed commits**" — 23 days old
- [dashpay/grovedb#1077 PERF-07-F1: the commit of a block measured on both engines, what a long commit waits for, and one command for the reference tables](https://github.com/dashpay/grovedb/pull/1077) — 12 unresolved (12 bot) · 0 days stale · ✋ changes requested · 🔴 CI failing · 🐢 targets feat/LigninDB · areas: fallback
  - Top thread: "✅ **Resolved** at \`d4300632\`; see the replies below." — 0 days old
- [dashpay/grovedb#1078 GROVE-13: the cost rules of stop points and finalize, under a grove version 6 that is not listed](https://github.com/dashpay/grovedb/pull/1078) — 6 unresolved (6 bot) · 0 days stale · 🔴 CI failing · 🐢 targets feat/LigninDB · areas: fallback
  - Top thread: "⏭️ **Deferred** (maintainer decision; not blocking) at \`464cacc8\`; see the replies below." — 0 days old
- [dashpay/platform#5530 feat(platform)!: token marketplace offers (PV15)](https://github.com/dashpay/platform/pull/5530) — 1 unresolved (1 bot) · 27 days stale · ✋ changes requested · 🔴 CI failing · 🐢 targets claude/token-market-listings · areas: rs-drive, rs-drive-abci, dpp, rs-platform-wallet, fallback
  - Top thread: "🔴 Blocking: Bound bid escrow by a buyer-signed credit limit**" — 27 days old
- [dashpay/grovedb#1076 TXN-11: a Merk's stale hashes are made inside a write transaction, and its root hash given before the commit](https://github.com/dashpay/grovedb/pull/1076) — 3 unresolved (3 bot) · 0 days stale · 🐢 targets feat/LigninDB · areas: fallback
  - Top thread: "🟡 Suggestion: A new hash dependency can escape scope claims if only the tree walk is updated**" — 0 days old
- [dashpay/grovedb#1075 OPS-03-F1: EitherEngine opens a RocksDB database under a name ending in .creating](https://github.com/dashpay/grovedb/pull/1075) — 🐢 targets feat/LigninDB · areas: fallback
- [dashpay/platform#4776 feat(platform-wallet)!: let contract updates clear the description through the FFI](https://github.com/dashpay/platform/pull/4776) — ⚠ merge conflict · 🔴 CI failing · 🐢 targets feat/delta-contract-update-pv15 · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk
- [dashpay/platform#5531 feat(platform)!: record token marketplace events in token history (PV15)](https://github.com/dashpay/platform/pull/5531) — 🔴 CI failing · 🐢 targets claude/token-market-offers · areas: rs-drive, rs-drive-abci, dpp, system-contracts, fallback
- [dashpay/platform#5532 feat(platform)!: destroy a frozen identity's escrowed listings with its funds (PV15)](https://github.com/dashpay/platform/pull/5532) — 🔴 CI failing · 🐢 targets claude/token-market-history · areas: rs-drive, rs-drive-abci, fallback
- [dashpay/platform#5751 feat(sdk): random token amounts in wasm-sdk and js-evo-sdk](https://github.com/dashpay/platform/pull/5751) — 🐢 targets claude/random-token-amounts · areas: js-wasm-sdk

<a id="quantumexplorer-clean"></a>
#### Clean (2)
- [dashpay/platform#6249 feat(platform)!: subscribe to committed state transitions matching document and address filters](https://github.com/dashpay/platform/pull/6249) — areas: rs-drive, rs-drive-abci, rust-sdk, dashmate, rust-dapi, github, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 9a4d02bdc3f9800b16e52b0546dffc23af9ddb8d
- [dashpay/tenderdash#1584 fix(statesync): restore LastAppHash of the snapshot height, not the next one](https://github.com/dashpay/tenderdash/pull/1584) — areas: fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 23c1443e4077c7c7dfedf00f9446c762e669660f

<a id="quantumexplorer-ready-for-review"></a>
#### Ready for Review (6)
- [dashpay/rust-dashcore#1328 refactor(key-wallet)!: validate child indexes at construction with \`ChildIndex\`](https://github.com/dashpay/rust-dashcore/pull/1328) — by @ZocoLini · areas: dash-spv, key-wallet, key-wallet-manager · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 0733b9d0346d6e41ca7f2f7905064240da6e074a
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/rust-dashcore#1426 fix(dash-spv): tell an empty filter header store from genesis](https://github.com/dashpay/rust-dashcore/pull/1426) — by @ZocoLini · areas: dash-spv · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed d31ec981450f9216f70fd0a3877e22563060db0c
- [dashpay/rust-dashcore#1490 ci: test only the crates a pull request changes](https://github.com/dashpay/rust-dashcore/pull/1490) — by @ZocoLini · areas: github, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 57c19ebefff21e528e338b7cc11d43d483fc8012
- [dashpay/rust-dashcore#1502 refactor(dash-spv)!: keep only storage failures in \`StorageError\`](https://github.com/dashpay/rust-dashcore/pull/1502) — by @ZocoLini · areas: dash-spv · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 8523816d18ec0843407f1525fc4fdac9c46fc32f
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/rust-dashcore#1538 fix(rpc-json)!: fix \`protx info\` hash order and operator reward](https://github.com/dashpay/rust-dashcore/pull/1538) — by @ZocoLini · areas: fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed a6c62aa4dee2dd59051dcd53b7fa45bb2df3605e
- [dashpay/rust-dashcore#1539 fix(rpc-json)!: decode the status of a masternode that is not ready](https://github.com/dashpay/rust-dashcore/pull/1539) — by @ZocoLini · areas: fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed e0c90f77a8a7cdac445008bdd1da21b792b2dbac
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return

<a id="shumkov"></a>
### @shumkov
<a id="shumkov-open"></a>
#### Open (12)
- [dashpay/rust-dashcore#634 feat(key-wallet-manager): single-map WalletManager with Arc&lt;RwLock&lt;T&gt;&gt; per wallet](https://github.com/dashpay/rust-dashcore/pull/634) — 20 unresolved (20 CodeRabbit) · 232 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · areas: dash-spv, key-wallet, key-wallet-manager, fallback · Policy: waiting-author
  - Top thread: "_⚠️ Potential issue_ \| _🟡 Minor_" — 232 days old
  - Blocker: coderabbitai requested changes on this head; dismiss the review or push a fix
  - Blocker: coderabbitai left review threads unresolved; resolve them
- [dashpay/platform#6333 chore: merge v5.1-dev (5.1.0) into v5.2-dev](https://github.com/dashpay/platform/pull/6333) — 4 unresolved (4 CodeRabbit) · 0 days stale · areas: dashmate, js-wasm-sdk, system-contracts, github, fallback · Policy: waiting-author
  - Top thread: "&lt;picture&gt;&lt;img src="https://storage.googleapis.com/coderabbit_public_assets/review-comment-icons-v1/major-16.svg" width="…" — 0 days old
  - Blocker: coderabbitai left review threads unresolved; resolve them
- [dashpay/platform#5656 fix(dashmate): build local images for every group node that builds any](https://github.com/dashpay/platform/pull/5656) — 1 unresolved (1 bot) · 27 days stale · 🐢 targets fix/dashmate/update-path-bugs · areas: dashmate
  - Top thread: "🔴 Blocking: Build the shared base image before dependent service images across group configurations**" — 27 days old
- [dashpay/platform#4283 fix(dashmate): keep the node up when an image pull fails, and stop reporting success when it did not](https://github.com/dashpay/platform/pull/4283) — 1 unresolved (1 bot) · 2 days stale · ✋ changes requested · 🔴 CI failing · areas: dashmate · Policy: waiting-author
  - Top thread: "🔴 Blocking: Group restart can stop all nodes without building an image the previous restart would build**" — 2 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5763 docs: e2e test architecture and legacy JS retirement plans (draft, not for merge)](https://github.com/dashpay/platform/pull/5763) — 📝 draft · areas: fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5819 feat(dashmate): add the modules that let dashmate update itself](https://github.com/dashpay/platform/pull/5819) — 📝 draft · 🐢 targets fix/dashmate/update-path-bugs · areas: dashmate
- [dashpay/platform#6300 feat(dashmate): report nodes not restarted on the installed dashmate in doctor](https://github.com/dashpay/platform/pull/6300) — 📝 draft · 🐢 targets feat/dashmate/update · areas: dashmate
- [dashpay/platform#6304 fix(dashmate): guide recovery from a dashmate helper that is older than the config](https://github.com/dashpay/platform/pull/6304) — 📝 draft · areas: dashmate · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#6319 fix(dashmate): keep Core out of Platform-only resets and make reset failures visible and recoverable](https://github.com/dashpay/platform/pull/6319) — areas: dashmate · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 3a5478dcbb86053a3ab389680eac4ce67b521720
- [dashpay/platform#6321 feat(dashmate): add setup --platform to set up Platform again after a hard reset](https://github.com/dashpay/platform/pull/6321) — 🐢 targets dashmate-reset-fixes · areas: dashmate
- [dashpay/platform#6325 fix(dashmate): doctor report redaction follow-ups](https://github.com/dashpay/platform/pull/6325) — areas: dashmate · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed af2ab5f34c235380e35544b1828206440764430f
- [dashpay/platform#6348 fix(release): name the repository for changelog links](https://github.com/dashpay/platform/pull/6348) — 🐢 targets v5.1-dev · areas: fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 398931528416888d78f141ce3f7d7ddb1163f223

<a id="shumkov-needs-action"></a>
#### Needs action (3)
- [dashpay/rust-dashcore#634 feat(key-wallet-manager): single-map WalletManager with Arc&lt;RwLock&lt;T&gt;&gt; per wallet](https://github.com/dashpay/rust-dashcore/pull/634) — 20 unresolved (20 CodeRabbit) · 232 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · areas: dash-spv, key-wallet, key-wallet-manager, fallback · Policy: waiting-author
  - Top thread: "_⚠️ Potential issue_ \| _🟡 Minor_" — 232 days old
  - Blocker: coderabbitai requested changes on this head; dismiss the review or push a fix
  - Blocker: coderabbitai left review threads unresolved; resolve them
- [dashpay/platform#6333 chore: merge v5.1-dev (5.1.0) into v5.2-dev](https://github.com/dashpay/platform/pull/6333) — 4 unresolved (4 CodeRabbit) · 0 days stale · areas: dashmate, js-wasm-sdk, system-contracts, github, fallback · Policy: waiting-author
  - Top thread: "&lt;picture&gt;&lt;img src="https://storage.googleapis.com/coderabbit_public_assets/review-comment-icons-v1/major-16.svg" width="…" — 0 days old
  - Blocker: coderabbitai left review threads unresolved; resolve them
- [dashpay/platform#4283 fix(dashmate): keep the node up when an image pull fails, and stop reporting success when it did not](https://github.com/dashpay/platform/pull/4283) — 1 unresolved (1 bot) · 2 days stale · ✋ changes requested · 🔴 CI failing · areas: dashmate · Policy: waiting-author
  - Top thread: "🔴 Blocking: Group restart can stop all nodes without building an image the previous restart would build**" — 2 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them

<a id="shumkov-unresolved-comments"></a>
#### Unresolved Comments (3)
- [dashpay/rust-dashcore#634 feat(key-wallet-manager): single-map WalletManager with Arc&lt;RwLock&lt;T&gt;&gt; per wallet](https://github.com/dashpay/rust-dashcore/pull/634) — 20 unresolved (20 CodeRabbit) · 232 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · areas: dash-spv, key-wallet, key-wallet-manager, fallback · Policy: waiting-author
  - Top thread: "_⚠️ Potential issue_ \| _🟡 Minor_" — 232 days old
  - Blocker: coderabbitai requested changes on this head; dismiss the review or push a fix
  - Blocker: coderabbitai left review threads unresolved; resolve them
- [dashpay/platform#6333 chore: merge v5.1-dev (5.1.0) into v5.2-dev](https://github.com/dashpay/platform/pull/6333) — 4 unresolved (4 CodeRabbit) · 0 days stale · areas: dashmate, js-wasm-sdk, system-contracts, github, fallback · Policy: waiting-author
  - Top thread: "&lt;picture&gt;&lt;img src="https://storage.googleapis.com/coderabbit_public_assets/review-comment-icons-v1/major-16.svg" width="…" — 0 days old
  - Blocker: coderabbitai left review threads unresolved; resolve them
- [dashpay/platform#4283 fix(dashmate): keep the node up when an image pull fails, and stop reporting success when it did not](https://github.com/dashpay/platform/pull/4283) — 1 unresolved (1 bot) · 2 days stale · ✋ changes requested · 🔴 CI failing · areas: dashmate · Policy: waiting-author
  - Top thread: "🔴 Blocking: Group restart can stop all nodes without building an image the previous restart would build**" — 2 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them

<a id="shumkov-draft"></a>
#### Draft (2)
- [dashpay/platform#5763 docs: e2e test architecture and legacy JS retirement plans (draft, not for merge)](https://github.com/dashpay/platform/pull/5763) — 📝 draft · areas: fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#6304 fix(dashmate): guide recovery from a dashmate helper that is older than the config](https://github.com/dashpay/platform/pull/6304) — 📝 draft · areas: dashmate · Policy: draft
  - Blocker: Draft PR does not occupy a review slot

<a id="shumkov-stale"></a>
#### Stale (5)
- [dashpay/platform#5656 fix(dashmate): build local images for every group node that builds any](https://github.com/dashpay/platform/pull/5656) — 1 unresolved (1 bot) · 27 days stale · 🐢 targets fix/dashmate/update-path-bugs · areas: dashmate
  - Top thread: "🔴 Blocking: Build the shared base image before dependent service images across group configurations**" — 27 days old
- [dashpay/platform#5819 feat(dashmate): add the modules that let dashmate update itself](https://github.com/dashpay/platform/pull/5819) — 📝 draft · 🐢 targets fix/dashmate/update-path-bugs · areas: dashmate
- [dashpay/platform#6300 feat(dashmate): report nodes not restarted on the installed dashmate in doctor](https://github.com/dashpay/platform/pull/6300) — 📝 draft · 🐢 targets feat/dashmate/update · areas: dashmate
- [dashpay/platform#6321 feat(dashmate): add setup --platform to set up Platform again after a hard reset](https://github.com/dashpay/platform/pull/6321) — 🐢 targets dashmate-reset-fixes · areas: dashmate
- [dashpay/platform#6348 fix(release): name the repository for changelog links](https://github.com/dashpay/platform/pull/6348) — 🐢 targets v5.1-dev · areas: fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 398931528416888d78f141ce3f7d7ddb1163f223

<a id="shumkov-clean"></a>
#### Clean (2)
- [dashpay/platform#6319 fix(dashmate): keep Core out of Platform-only resets and make reset failures visible and recoverable](https://github.com/dashpay/platform/pull/6319) — areas: dashmate · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 3a5478dcbb86053a3ab389680eac4ce67b521720
- [dashpay/platform#6325 fix(dashmate): doctor report redaction follow-ups](https://github.com/dashpay/platform/pull/6325) — areas: dashmate · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed af2ab5f34c235380e35544b1828206440764430f

<a id="shumkov-ready-for-review"></a>
#### Ready for Review (4)
- [dashpay/platform#6249 feat(platform)!: subscribe to committed state transitions matching document and address filters](https://github.com/dashpay/platform/pull/6249) — by @QuantumExplorer · areas: rs-drive, rs-drive-abci, rust-sdk, dashmate, rust-dapi, github, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 9a4d02bdc3f9800b16e52b0546dffc23af9ddb8d
- [dashpay/rust-dashcore#1490 ci: test only the crates a pull request changes](https://github.com/dashpay/rust-dashcore/pull/1490) — by @ZocoLini · areas: github, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 57c19ebefff21e528e338b7cc11d43d483fc8012
- [dashpay/tenderdash#1584 fix(statesync): restore LastAppHash of the snapshot height, not the next one](https://github.com/dashpay/tenderdash/pull/1584) — by @QuantumExplorer · areas: fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 23c1443e4077c7c7dfedf00f9446c762e669660f
- [dashpay/tenderdash#1600 fix(consensus): deliver and process a polka block for a validator left in its round](https://github.com/dashpay/tenderdash/pull/1600) — by @Claudius-Maginificent · areas: fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required

<a id="xdustinface"></a>
### @xdustinface
<a id="xdustinface-open"></a>
#### Open (8)
- [dashpay/platform#5791 fix(platform-wallet): refuse a quorum type that does not fit a byte](https://github.com/dashpay/platform/pull/5791) — 2 unresolved (1 CodeRabbit, 1 bot) · 20 days stale · ⚠ merge conflict · areas: rs-platform-wallet · Policy: waiting-author
  - Top thread: "&lt;picture&gt;&lt;img src="https://storage.googleapis.com/coderabbit_public_assets/review-comment-icons-v1/amber-16.svg" width="…" — 20 days old
  - Blocker: coderabbitai left review threads unresolved; resolve them
- [dashpay/rust-dashcore#1474 fix(dash-spv): hedge a request once it waited twice its queue](https://github.com/dashpay/rust-dashcore/pull/1474) — 1 unresolved (1 CodeRabbit) · 1 days stale · ⚠ merge conflict · areas: dash-spv · Policy: waiting-author
  - Top thread: "&lt;picture&gt;&lt;img src="https://storage.googleapis.com/coderabbit_public_assets/review-comment-icons-v1/amber-16.svg" width="…" — 1 days old
  - Blocker: coderabbitai left review threads unresolved; resolve them
- [dashpay/rust-dashcore#541 ci: integrate Codecov test analytics via \`cargo-nextest\`](https://github.com/dashpay/rust-dashcore/pull/541) — ⚠ merge conflict · 📝 draft · areas: github, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#1372 fix(dash-spv): spread peers across network groups](https://github.com/dashpay/rust-dashcore/pull/1372) — ⚠ merge conflict · ✋ changes requested · areas: dash-spv · Policy: waiting-author
  - Blocker: coderabbitai left review threads unresolved; resolve them
- [dashpay/rust-dashcore#1373 fix(dash-spv): rotate the slowest peer of a slow set](https://github.com/dashpay/rust-dashcore/pull/1373) — ⚠ merge conflict · 📝 draft · 🐢 targets fix/peer-diversity · areas: dash-spv
- [dashpay/rust-dashcore#1374 fix(dash-spv): retry a refused request at another peer](https://github.com/dashpay/rust-dashcore/pull/1374) — 📝 draft · 🐢 targets fix/peer-exploration · areas: dash-spv
- [dashpay/rust-dashcore#1375 fix(dash-spv): rank a stalled peer last when routing](https://github.com/dashpay/rust-dashcore/pull/1375) — 📝 draft · 🐢 targets fix/peer-exploration · areas: dash-spv
- [dashpay/rust-dashcore#1536 ci: run CI on draft PRs only on request](https://github.com/dashpay/rust-dashcore/pull/1536) — ✋ changes requested · areas: github, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed a6ec27f1e6a8188c97fd3b386381aa822b22351b
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return

<a id="xdustinface-needs-action"></a>
#### Needs action (4)
- [dashpay/platform#5791 fix(platform-wallet): refuse a quorum type that does not fit a byte](https://github.com/dashpay/platform/pull/5791) — 2 unresolved (1 CodeRabbit, 1 bot) · 20 days stale · ⚠ merge conflict · areas: rs-platform-wallet · Policy: waiting-author
  - Top thread: "&lt;picture&gt;&lt;img src="https://storage.googleapis.com/coderabbit_public_assets/review-comment-icons-v1/amber-16.svg" width="…" — 20 days old
  - Blocker: coderabbitai left review threads unresolved; resolve them
- [dashpay/rust-dashcore#1474 fix(dash-spv): hedge a request once it waited twice its queue](https://github.com/dashpay/rust-dashcore/pull/1474) — 1 unresolved (1 CodeRabbit) · 1 days stale · ⚠ merge conflict · areas: dash-spv · Policy: waiting-author
  - Top thread: "&lt;picture&gt;&lt;img src="https://storage.googleapis.com/coderabbit_public_assets/review-comment-icons-v1/amber-16.svg" width="…" — 1 days old
  - Blocker: coderabbitai left review threads unresolved; resolve them
- [dashpay/rust-dashcore#1372 fix(dash-spv): spread peers across network groups](https://github.com/dashpay/rust-dashcore/pull/1372) — ⚠ merge conflict · ✋ changes requested · areas: dash-spv · Policy: waiting-author
  - Blocker: coderabbitai left review threads unresolved; resolve them
- [dashpay/rust-dashcore#1536 ci: run CI on draft PRs only on request](https://github.com/dashpay/rust-dashcore/pull/1536) — ✋ changes requested · areas: github, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed a6ec27f1e6a8188c97fd3b386381aa822b22351b
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return

<a id="xdustinface-unresolved-comments"></a>
#### Unresolved Comments (2)
- [dashpay/platform#5791 fix(platform-wallet): refuse a quorum type that does not fit a byte](https://github.com/dashpay/platform/pull/5791) — 2 unresolved (1 CodeRabbit, 1 bot) · 20 days stale · ⚠ merge conflict · areas: rs-platform-wallet · Policy: waiting-author
  - Top thread: "&lt;picture&gt;&lt;img src="https://storage.googleapis.com/coderabbit_public_assets/review-comment-icons-v1/amber-16.svg" width="…" — 20 days old
  - Blocker: coderabbitai left review threads unresolved; resolve them
- [dashpay/rust-dashcore#1474 fix(dash-spv): hedge a request once it waited twice its queue](https://github.com/dashpay/rust-dashcore/pull/1474) — 1 unresolved (1 CodeRabbit) · 1 days stale · ⚠ merge conflict · areas: dash-spv · Policy: waiting-author
  - Top thread: "&lt;picture&gt;&lt;img src="https://storage.googleapis.com/coderabbit_public_assets/review-comment-icons-v1/amber-16.svg" width="…" — 1 days old
  - Blocker: coderabbitai left review threads unresolved; resolve them

<a id="xdustinface-changes-requested"></a>
#### Changes Requested (2)
- [dashpay/rust-dashcore#1372 fix(dash-spv): spread peers across network groups](https://github.com/dashpay/rust-dashcore/pull/1372) — ⚠ merge conflict · ✋ changes requested · areas: dash-spv · Policy: waiting-author
  - Blocker: coderabbitai left review threads unresolved; resolve them
- [dashpay/rust-dashcore#1536 ci: run CI on draft PRs only on request](https://github.com/dashpay/rust-dashcore/pull/1536) — ✋ changes requested · areas: github, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed a6ec27f1e6a8188c97fd3b386381aa822b22351b
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return

<a id="xdustinface-draft"></a>
#### Draft (1)
- [dashpay/rust-dashcore#541 ci: integrate Codecov test analytics via \`cargo-nextest\`](https://github.com/dashpay/rust-dashcore/pull/541) — ⚠ merge conflict · 📝 draft · areas: github, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot

<a id="xdustinface-stale"></a>
#### Stale (3)
- [dashpay/rust-dashcore#1373 fix(dash-spv): rotate the slowest peer of a slow set](https://github.com/dashpay/rust-dashcore/pull/1373) — ⚠ merge conflict · 📝 draft · 🐢 targets fix/peer-diversity · areas: dash-spv
- [dashpay/rust-dashcore#1374 fix(dash-spv): retry a refused request at another peer](https://github.com/dashpay/rust-dashcore/pull/1374) — 📝 draft · 🐢 targets fix/peer-exploration · areas: dash-spv
- [dashpay/rust-dashcore#1375 fix(dash-spv): rank a stalled peer last when routing](https://github.com/dashpay/rust-dashcore/pull/1375) — 📝 draft · 🐢 targets fix/peer-exploration · areas: dash-spv

<a id="xdustinface-ready-for-review"></a>
#### Ready for Review (6)
- [dashpay/rust-dashcore#1328 refactor(key-wallet)!: validate child indexes at construction with \`ChildIndex\`](https://github.com/dashpay/rust-dashcore/pull/1328) — by @ZocoLini · areas: dash-spv, key-wallet, key-wallet-manager · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 0733b9d0346d6e41ca7f2f7905064240da6e074a
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/rust-dashcore#1426 fix(dash-spv): tell an empty filter header store from genesis](https://github.com/dashpay/rust-dashcore/pull/1426) — by @ZocoLini · areas: dash-spv · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed d31ec981450f9216f70fd0a3877e22563060db0c
- [dashpay/rust-dashcore#1490 ci: test only the crates a pull request changes](https://github.com/dashpay/rust-dashcore/pull/1490) — by @ZocoLini · areas: github, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 57c19ebefff21e528e338b7cc11d43d483fc8012
- [dashpay/rust-dashcore#1502 refactor(dash-spv)!: keep only storage failures in \`StorageError\`](https://github.com/dashpay/rust-dashcore/pull/1502) — by @ZocoLini · areas: dash-spv · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 8523816d18ec0843407f1525fc4fdac9c46fc32f
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/rust-dashcore#1538 fix(rpc-json)!: fix \`protx info\` hash order and operator reward](https://github.com/dashpay/rust-dashcore/pull/1538) — by @ZocoLini · areas: fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed a6c62aa4dee2dd59051dcd53b7fa45bb2df3605e
- [dashpay/rust-dashcore#1539 fix(rpc-json)!: decode the status of a masternode that is not ready](https://github.com/dashpay/rust-dashcore/pull/1539) — by @ZocoLini · areas: fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed e0c90f77a8a7cdac445008bdd1da21b792b2dbac
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return

<a id="hashengineering"></a>
### @HashEngineering
<a id="hashengineering-open"></a>
#### Open (5)
- [dashpay/platform#5931 feat(platform-wallet): top up someone else's identity from the Core balance](https://github.com/dashpay/platform/pull/5931) — 6 unresolved (6 human) · 0 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage · Policy: waiting-author
  - Top thread: "Suggestion (docs): the code-90 docs disagree with the code.**" — 0 days old
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5933 feat(kotlin-sdk): top up someone else's identity from the Core balance](https://github.com/dashpay/platform/pull/5933) — 2 unresolved (1 CodeRabbit, 1 bot) · 12 days stale · ⚠ merge conflict · ✋ changes requested · 📝 draft · 🐢 targets HashEngineering/platform-wallet-top-up-someone-elses-identity-fr · areas: kotlin-sdk, sdk-parity
  - Top thread: "&lt;picture&gt;&lt;img src="https://storage.googleapis.com/coderabbit_public_assets/review-comment-icons-v1/amber-16.svg" width="…" — 12 days old
- [dashpay/platform#6242 fix(kotlin-sdk): reject malformed UTF-16 string arguments at the JNI boundary](https://github.com/dashpay/platform/pull/6242) — 1 unresolved (1 bot) · 1 days stale · 🔴 CI failing · areas: kotlin-sdk · Policy: waiting-self-review
  - Top thread: "🟡 Suggestion: Private-document calls can abort while adding the C-string terminator**" — 1 days old
  - Blocker: Author must post /self-reviewed 5ac93c86834ee19484cbdfde668f66197d13ad36
- [dashpay/platform#5479 feat(platform-wallet-storage): persist DashPay backfill coverage and outbound-account markers](https://github.com/dashpay/platform/pull/5479) — ⚠ merge conflict · areas: rs-platform-wallet, wallet-storage · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 8e1ee2e24549cabf2d2e81a85a02fc613212aa1f
- [dashpay/platform#5906 fix(kotlin-sdk): refuse address types wider than a byte and document the address top-up outcomes](https://github.com/dashpay/platform/pull/5906) — areas: kotlin-sdk · Policy: ready-to-merge
  - Blocker: All policy requirements are satisfied

<a id="hashengineering-needs-action"></a>
#### Needs action (3)
- [dashpay/platform#5931 feat(platform-wallet): top up someone else's identity from the Core balance](https://github.com/dashpay/platform/pull/5931) — 6 unresolved (6 human) · 0 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage · Policy: waiting-author
  - Top thread: "Suggestion (docs): the code-90 docs disagree with the code.**" — 0 days old
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#6242 fix(kotlin-sdk): reject malformed UTF-16 string arguments at the JNI boundary](https://github.com/dashpay/platform/pull/6242) — 1 unresolved (1 bot) · 1 days stale · 🔴 CI failing · areas: kotlin-sdk · Policy: waiting-self-review
  - Top thread: "🟡 Suggestion: Private-document calls can abort while adding the C-string terminator**" — 1 days old
  - Blocker: Author must post /self-reviewed 5ac93c86834ee19484cbdfde668f66197d13ad36
- [dashpay/platform#5479 feat(platform-wallet-storage): persist DashPay backfill coverage and outbound-account markers](https://github.com/dashpay/platform/pull/5479) — ⚠ merge conflict · areas: rs-platform-wallet, wallet-storage · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 8e1ee2e24549cabf2d2e81a85a02fc613212aa1f

<a id="hashengineering-ready-for-human"></a>
#### Ready for human (1)
- [dashpay/platform#5906 fix(kotlin-sdk): refuse address types wider than a byte and document the address top-up outcomes](https://github.com/dashpay/platform/pull/5906) — areas: kotlin-sdk · Policy: ready-to-merge
  - Blocker: All policy requirements are satisfied

<a id="hashengineering-unresolved-comments"></a>
#### Unresolved Comments (2)
- [dashpay/platform#5931 feat(platform-wallet): top up someone else's identity from the Core balance](https://github.com/dashpay/platform/pull/5931) — 6 unresolved (6 human) · 0 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage · Policy: waiting-author
  - Top thread: "Suggestion (docs): the code-90 docs disagree with the code.**" — 0 days old
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#6242 fix(kotlin-sdk): reject malformed UTF-16 string arguments at the JNI boundary](https://github.com/dashpay/platform/pull/6242) — 1 unresolved (1 bot) · 1 days stale · 🔴 CI failing · areas: kotlin-sdk · Policy: waiting-self-review
  - Top thread: "🟡 Suggestion: Private-document calls can abort while adding the C-string terminator**" — 1 days old
  - Blocker: Author must post /self-reviewed 5ac93c86834ee19484cbdfde668f66197d13ad36

<a id="hashengineering-stale"></a>
#### Stale (1)
- [dashpay/platform#5933 feat(kotlin-sdk): top up someone else's identity from the Core balance](https://github.com/dashpay/platform/pull/5933) — 2 unresolved (1 CodeRabbit, 1 bot) · 12 days stale · ⚠ merge conflict · ✋ changes requested · 📝 draft · 🐢 targets HashEngineering/platform-wallet-top-up-someone-elses-identity-fr · areas: kotlin-sdk, sdk-parity
  - Top thread: "&lt;picture&gt;&lt;img src="https://storage.googleapis.com/coderabbit_public_assets/review-comment-icons-v1/amber-16.svg" width="…" — 12 days old

<a id="hashengineering-clean"></a>
#### Clean (2)
- [dashpay/platform#5479 feat(platform-wallet-storage): persist DashPay backfill coverage and outbound-account markers](https://github.com/dashpay/platform/pull/5479) — ⚠ merge conflict · areas: rs-platform-wallet, wallet-storage · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 8e1ee2e24549cabf2d2e81a85a02fc613212aa1f
- [dashpay/platform#5906 fix(kotlin-sdk): refuse address types wider than a byte and document the address top-up outcomes](https://github.com/dashpay/platform/pull/5906) — areas: kotlin-sdk · Policy: ready-to-merge
  - Blocker: All policy requirements are satisfied

<a id="hashengineering-ready-for-review"></a>
#### Ready for Review (1)
- [dashpay/platform#6309 fix(platform-wallet): report a token pool tree that does not build as a store error](https://github.com/dashpay/platform/pull/6309) — by @romchornyi · areas: rs-platform-wallet, rs-platform-wallet-ffi · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 4aa56d16b63d5a639c16ddfc7e4c3195f3036c58

<a id="bfoss765"></a>
### @bfoss765
<a id="bfoss765-open"></a>
#### Open (2)
- [dashpay/platform#4313 feat(kotlin-sdk): one-time Orchard key shielded-invite API (inviter + claim)](https://github.com/dashpay/platform/pull/4313) — 6 unresolved (6 bot) · 87 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk · Policy: waiting-author
  - Top thread: "✅ **Resolved** at \`eccc65fa\`; see the replies below." — 87 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#4312 feat(platform-wallet): multi-output shielded transfers + output-aware fee predictor](https://github.com/dashpay/platform/pull/4312) — 1 unresolved (1 bot) · 87 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · areas: dpp, rs-platform-wallet, rs-platform-wallet-ffi, kotlin-sdk, fallback · Policy: waiting-self-review
  - Top thread: "🟡 Suggestion: Keep envelope-aware action counting behind the crate boundary**" — 87 days old
  - Blocker: Author must post /self-reviewed da932f96a8670c641065954de8b58752e99233a3

<a id="bfoss765-needs-action"></a>
#### Needs action (2)
- [dashpay/platform#4313 feat(kotlin-sdk): one-time Orchard key shielded-invite API (inviter + claim)](https://github.com/dashpay/platform/pull/4313) — 6 unresolved (6 bot) · 87 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk · Policy: waiting-author
  - Top thread: "✅ **Resolved** at \`eccc65fa\`; see the replies below." — 87 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#4312 feat(platform-wallet): multi-output shielded transfers + output-aware fee predictor](https://github.com/dashpay/platform/pull/4312) — 1 unresolved (1 bot) · 87 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · areas: dpp, rs-platform-wallet, rs-platform-wallet-ffi, kotlin-sdk, fallback · Policy: waiting-self-review
  - Top thread: "🟡 Suggestion: Keep envelope-aware action counting behind the crate boundary**" — 87 days old
  - Blocker: Author must post /self-reviewed da932f96a8670c641065954de8b58752e99233a3

<a id="bfoss765-unresolved-comments"></a>
#### Unresolved Comments (2)
- [dashpay/platform#4313 feat(kotlin-sdk): one-time Orchard key shielded-invite API (inviter + claim)](https://github.com/dashpay/platform/pull/4313) — 6 unresolved (6 bot) · 87 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk · Policy: waiting-author
  - Top thread: "✅ **Resolved** at \`eccc65fa\`; see the replies below." — 87 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#4312 feat(platform-wallet): multi-output shielded transfers + output-aware fee predictor](https://github.com/dashpay/platform/pull/4312) — 1 unresolved (1 bot) · 87 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · areas: dpp, rs-platform-wallet, rs-platform-wallet-ffi, kotlin-sdk, fallback · Policy: waiting-self-review
  - Top thread: "🟡 Suggestion: Keep envelope-aware action counting behind the crate boundary**" — 87 days old
  - Blocker: Author must post /self-reviewed da932f96a8670c641065954de8b58752e99233a3

<a id="llbartekll"></a>
### @llbartekll
<a id="llbartekll-open"></a>
#### Open (1)
- [dashpay/platform#6326 fix(swift-sdk): remove an identity from the live wallet before deleting its rows and keys](https://github.com/dashpay/platform/pull/6326) — 4 unresolved (4 bot) · 0 days stale · ✋ changes requested · 🔴 CI failing · areas: swift-sdk · Policy: waiting-author
  - Top thread: "🟡 Suggestion: A removed identity's cached token balances remain readable**" — 0 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them

<a id="llbartekll-needs-action"></a>
#### Needs action (1)
- [dashpay/platform#6326 fix(swift-sdk): remove an identity from the live wallet before deleting its rows and keys](https://github.com/dashpay/platform/pull/6326) — 4 unresolved (4 bot) · 0 days stale · ✋ changes requested · 🔴 CI failing · areas: swift-sdk · Policy: waiting-author
  - Top thread: "🟡 Suggestion: A removed identity's cached token balances remain readable**" — 0 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them

<a id="llbartekll-unresolved-comments"></a>
#### Unresolved Comments (1)
- [dashpay/platform#6326 fix(swift-sdk): remove an identity from the live wallet before deleting its rows and keys](https://github.com/dashpay/platform/pull/6326) — 4 unresolved (4 bot) · 0 days stale · ✋ changes requested · 🔴 CI failing · areas: swift-sdk · Policy: waiting-author
  - Top thread: "🟡 Suggestion: A removed identity's cached token balances remain readable**" — 0 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them

<a id="llbartekll-ready-for-review"></a>
#### Ready for Review (2)
- [dashpay/platform#4992 feat(swift-sdk): durable wallet presence marker readable while the device is locked](https://github.com/dashpay/platform/pull/4992) — by @romchornyi · areas: swift-sdk · Policy: ready-to-merge
  - Blocker: All policy requirements are satisfied
- [dashpay/platform#6309 fix(platform-wallet): report a token pool tree that does not build as a store error](https://github.com/dashpay/platform/pull/6309) — by @romchornyi · areas: rs-platform-wallet, rs-platform-wallet-ffi · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 4aa56d16b63d5a639c16ddfc7e4c3195f3036c58

<a id="romchornyi"></a>
### @romchornyi
<a id="romchornyi-open"></a>
#### Open (16)
- [dashpay/platform#5930 feat(platform-wallet): record the payment intent of external asset locks](https://github.com/dashpay/platform/pull/5930) — 9 unresolved (9 bot) · 8 days stale · 🐢 targets HashEngineering/platform-wallet-top-up-someone-elses-identity-fr · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage
  - Top thread: "✅ **Resolved** at \`a2ed9e10\`; see the replies below." — 8 days old
- [dashpay/platform#5258 feat(platform-wallet): probe the network about sends with an unknown broadcast outcome](https://github.com/dashpay/platform/pull/5258) — 3 unresolved (3 human) · 42 days stale · ⚠ merge conflict · 📝 draft · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, rust-sdk · Policy: draft
  - Top thread: "\[P2\] Carry the originating wallet generation in probe events" — 42 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5540 fix(platform-wallet): store externally supplied identity keys without a derivation breadcrumb](https://github.com/dashpay/platform/pull/5540) — 3 unresolved (3 bot) · 35 days stale · ⚠ merge conflict · 🔴 CI failing · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk · Policy: waiting-author
  - Top thread: "✅ **Resolved** at \`b25b2543\`; see the replies below." — 35 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5549 fix(platform-wallet-storage): restore DashPay contact address pools on SQLite load](https://github.com/dashpay/platform/pull/5549) — 3 unresolved (3 bot) · 34 days stale · ⚠ merge conflict · 🔴 CI failing · 📝 draft · areas: wallet-storage · Policy: draft
  - Top thread: "✅ **Withdrawn** at \`2bee2705\`; see the replies below." — 34 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#6273 feat(kotlin-sdk): persist and resume the Platform addresses an external asset lock pays](https://github.com/dashpay/platform/pull/6273) — 10 unresolved (10 bot) · 1 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · 📝 draft · 🐢 targets feat/platform-wallet-asset-lock-intent · areas: kotlin-sdk, sdk-parity
  - Top thread: "🔴 Blocking: The Android wallet database cannot open after upgrading from the published 5.1 dev SDKs**" — 1 days old
- [dashpay/platform#5395 feat(platform-wallet): spend only final coins and report funds waiting on the network](https://github.com/dashpay/platform/pull/5395) — 1 unresolved (1 human) · 42 days stale · ⚠ merge conflict · ✋ changes requested · 📝 draft · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk · Policy: draft
  - Top thread: "this should belong to another layer - likely platform-wallet, or even rust-dashcore's key-wallet. FFI is not meant to st…" — 42 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#6139 feat(swift-sdk): top up someone else's identity from the Core balance](https://github.com/dashpay/platform/pull/6139) — 5 unresolved (5 bot) · 3 days stale · 🐢 targets HashEngineering/platform-wallet-top-up-someone-elses-identity-fr · areas: swift-sdk, kotlin-sdk, sdk-parity
  - Top thread: "✅ **Resolved** at \`a242c339\`; see the replies below." — 3 days old
- [dashpay/platform#6272 feat(swift-sdk): persist and resume the Platform addresses an external asset lock pays](https://github.com/dashpay/platform/pull/6272) — 7 unresolved (7 bot) · 1 days stale · ✋ changes requested · 🔴 CI failing · 📝 draft · 🐢 targets feat/platform-wallet-asset-lock-intent · areas: swift-sdk, kotlin-sdk, sdk-parity
  - Top thread: "✅ **Resolved** at \`c154235e\`; see the replies below." — 1 days old
- [dashpay/platform#4992 feat(swift-sdk): durable wallet presence marker readable while the device is locked](https://github.com/dashpay/platform/pull/4992) — areas: swift-sdk · Policy: ready-to-merge
  - Blocker: All policy requirements are satisfied
- [dashpay/platform#5587 fix(platform-wallet)!: keep a contact's payment account across relaunches and replace it cleanly on rotation](https://github.com/dashpay/platform/pull/5587) — ⚠ merge conflict · 📝 draft · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5589 feat(platform-wallet-ffi)!: let hosts reset an outbound contact account's pool and keep its request digest](https://github.com/dashpay/platform/pull/5589) — 🔴 CI failing · 📝 draft · 🐢 targets fix/contact-rebuild-keeps-pool · areas: rs-platform-wallet-ffi, swift-sdk
- [dashpay/platform#5590 feat(kotlin-sdk)!: reset an outbound contact account's pool and keep its request digest on Android](https://github.com/dashpay/platform/pull/5590) — 🔴 CI failing · 📝 draft · 🐢 targets fix/contact-rebuild-ffi-swift · areas: kotlin-sdk, sdk-parity
- [dashpay/platform#5853 feat: report a shielded identity create proven to have its inputs spent without its identity](https://github.com/dashpay/platform/pull/5853) — ⚠ merge conflict · 📝 draft · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, rust-sdk, kotlin-sdk, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5928 feat(platform-wallet): report whether a failed payment may have left the wallet](https://github.com/dashpay/platform/pull/5928) — 🔴 CI failing · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, rust-sdk, kotlin-sdk, sdk-parity · Policy: waiting-author
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
- [dashpay/platform#6014 feat(platform-wallet)!: give identity slots one wallet-owned lease](https://github.com/dashpay/platform/pull/6014) — ⚠ merge conflict · ✋ changes requested · 📝 draft · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, rust-sdk, kotlin-sdk, sdk-parity · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#6309 fix(platform-wallet): report a token pool tree that does not build as a store error](https://github.com/dashpay/platform/pull/6309) — areas: rs-platform-wallet, rs-platform-wallet-ffi · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 4aa56d16b63d5a639c16ddfc7e4c3195f3036c58

<a id="romchornyi-needs-action"></a>
#### Needs action (1)
- [dashpay/platform#5540 fix(platform-wallet): store externally supplied identity keys without a derivation breadcrumb](https://github.com/dashpay/platform/pull/5540) — 3 unresolved (3 bot) · 35 days stale · ⚠ merge conflict · 🔴 CI failing · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk · Policy: waiting-author
  - Top thread: "✅ **Resolved** at \`b25b2543\`; see the replies below." — 35 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them

<a id="romchornyi-ready-for-human"></a>
#### Ready for human (1)
- [dashpay/platform#4992 feat(swift-sdk): durable wallet presence marker readable while the device is locked](https://github.com/dashpay/platform/pull/4992) — areas: swift-sdk · Policy: ready-to-merge
  - Blocker: All policy requirements are satisfied

<a id="romchornyi-unresolved-comments"></a>
#### Unresolved Comments (1)
- [dashpay/platform#5540 fix(platform-wallet): store externally supplied identity keys without a derivation breadcrumb](https://github.com/dashpay/platform/pull/5540) — 3 unresolved (3 bot) · 35 days stale · ⚠ merge conflict · 🔴 CI failing · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk · Policy: waiting-author
  - Top thread: "✅ **Resolved** at \`b25b2543\`; see the replies below." — 35 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them

<a id="romchornyi-ci-failing"></a>
#### CI Failing (1)
- [dashpay/platform#5928 feat(platform-wallet): report whether a failed payment may have left the wallet](https://github.com/dashpay/platform/pull/5928) — 🔴 CI failing · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, rust-sdk, kotlin-sdk, sdk-parity · Policy: waiting-author
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix

<a id="romchornyi-draft"></a>
#### Draft (6)
- [dashpay/platform#5258 feat(platform-wallet): probe the network about sends with an unknown broadcast outcome](https://github.com/dashpay/platform/pull/5258) — 3 unresolved (3 human) · 42 days stale · ⚠ merge conflict · 📝 draft · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, rust-sdk · Policy: draft
  - Top thread: "\[P2\] Carry the originating wallet generation in probe events" — 42 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5549 fix(platform-wallet-storage): restore DashPay contact address pools on SQLite load](https://github.com/dashpay/platform/pull/5549) — 3 unresolved (3 bot) · 34 days stale · ⚠ merge conflict · 🔴 CI failing · 📝 draft · areas: wallet-storage · Policy: draft
  - Top thread: "✅ **Withdrawn** at \`2bee2705\`; see the replies below." — 34 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5395 feat(platform-wallet): spend only final coins and report funds waiting on the network](https://github.com/dashpay/platform/pull/5395) — 1 unresolved (1 human) · 42 days stale · ⚠ merge conflict · ✋ changes requested · 📝 draft · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk · Policy: draft
  - Top thread: "this should belong to another layer - likely platform-wallet, or even rust-dashcore's key-wallet. FFI is not meant to st…" — 42 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5587 fix(platform-wallet)!: keep a contact's payment account across relaunches and replace it cleanly on rotation](https://github.com/dashpay/platform/pull/5587) — ⚠ merge conflict · 📝 draft · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5853 feat: report a shielded identity create proven to have its inputs spent without its identity](https://github.com/dashpay/platform/pull/5853) — ⚠ merge conflict · 📝 draft · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, rust-sdk, kotlin-sdk, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#6014 feat(platform-wallet)!: give identity slots one wallet-owned lease](https://github.com/dashpay/platform/pull/6014) — ⚠ merge conflict · ✋ changes requested · 📝 draft · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, rust-sdk, kotlin-sdk, sdk-parity · Policy: draft
  - Blocker: Draft PR does not occupy a review slot

<a id="romchornyi-stale"></a>
#### Stale (6)
- [dashpay/platform#5930 feat(platform-wallet): record the payment intent of external asset locks](https://github.com/dashpay/platform/pull/5930) — 9 unresolved (9 bot) · 8 days stale · 🐢 targets HashEngineering/platform-wallet-top-up-someone-elses-identity-fr · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage
  - Top thread: "✅ **Resolved** at \`a2ed9e10\`; see the replies below." — 8 days old
- [dashpay/platform#6273 feat(kotlin-sdk): persist and resume the Platform addresses an external asset lock pays](https://github.com/dashpay/platform/pull/6273) — 10 unresolved (10 bot) · 1 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · 📝 draft · 🐢 targets feat/platform-wallet-asset-lock-intent · areas: kotlin-sdk, sdk-parity
  - Top thread: "🔴 Blocking: The Android wallet database cannot open after upgrading from the published 5.1 dev SDKs**" — 1 days old
- [dashpay/platform#6139 feat(swift-sdk): top up someone else's identity from the Core balance](https://github.com/dashpay/platform/pull/6139) — 5 unresolved (5 bot) · 3 days stale · 🐢 targets HashEngineering/platform-wallet-top-up-someone-elses-identity-fr · areas: swift-sdk, kotlin-sdk, sdk-parity
  - Top thread: "✅ **Resolved** at \`a242c339\`; see the replies below." — 3 days old
- [dashpay/platform#6272 feat(swift-sdk): persist and resume the Platform addresses an external asset lock pays](https://github.com/dashpay/platform/pull/6272) — 7 unresolved (7 bot) · 1 days stale · ✋ changes requested · 🔴 CI failing · 📝 draft · 🐢 targets feat/platform-wallet-asset-lock-intent · areas: swift-sdk, kotlin-sdk, sdk-parity
  - Top thread: "✅ **Resolved** at \`c154235e\`; see the replies below." — 1 days old
- [dashpay/platform#5589 feat(platform-wallet-ffi)!: let hosts reset an outbound contact account's pool and keep its request digest](https://github.com/dashpay/platform/pull/5589) — 🔴 CI failing · 📝 draft · 🐢 targets fix/contact-rebuild-keeps-pool · areas: rs-platform-wallet-ffi, swift-sdk
- [dashpay/platform#5590 feat(kotlin-sdk)!: reset an outbound contact account's pool and keep its request digest on Android](https://github.com/dashpay/platform/pull/5590) — 🔴 CI failing · 📝 draft · 🐢 targets fix/contact-rebuild-ffi-swift · areas: kotlin-sdk, sdk-parity

<a id="romchornyi-clean"></a>
#### Clean (2)
- [dashpay/platform#4992 feat(swift-sdk): durable wallet presence marker readable while the device is locked](https://github.com/dashpay/platform/pull/4992) — areas: swift-sdk · Policy: ready-to-merge
  - Blocker: All policy requirements are satisfied
- [dashpay/platform#6309 fix(platform-wallet): report a token pool tree that does not build as a store error](https://github.com/dashpay/platform/pull/6309) — areas: rs-platform-wallet, rs-platform-wallet-ffi · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 4aa56d16b63d5a639c16ddfc7e4c3195f3036c58

<a id="ktechmidas"></a>
### @ktechmidas
<a id="ktechmidas-open"></a>
#### Open (6)
- [dashpay/platform#5188 ci: tolerate optional S3 cache export outages on chore/bump-rust-dashcore-secp-033](https://github.com/dashpay/platform/pull/5188) — 1 unresolved (1 bot) · 59 days stale · 🐢 targets chore/bump-rust-dashcore-secp-033 · areas: github
  - Top thread: "🟡 Suggestion: New regression tests are not executed by any CI workflow on this branch**" — 59 days old
- [dashpay/platform#5186 ci: tolerate optional S3 cache export outages on v5.0-dev](https://github.com/dashpay/platform/pull/5186) — 🔴 CI failing · 🐢 targets v6.0-dev · areas: github · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 843052c2dce26a80065c15b1219bbc8211264161
- [dashpay/platform#5198 ci: bound default Binaryen optimizer threads in NPM builds](https://github.com/dashpay/platform/pull/5198) — 🐢 targets v6.0-dev · areas: github · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 23b41b477bc038965b686b4a75412b1c21fc6d98
- [dashpay/platform#5205 fix(release): install verified cargo-binstall binary directly](https://github.com/dashpay/platform/pull/5205) — 🐢 targets v6.0-dev · areas: fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 8a8ac2c788b5338416e18045001d9115ba3ac80a
- [dashpay/platform#5866 feat(dashmate): clean up historical Docker images after updates](https://github.com/dashpay/platform/pull/5866) — ✋ changes requested · 🔴 CI failing · areas: dashmate · Policy: waiting-author
  - Blocker: Author response is required after the latest human objection
- [dashpay/platform#6018 ci: move CI Docker images from ECR to GHCR (saves about $265 a month)](https://github.com/dashpay/platform/pull/6018) — 🔴 CI failing · areas: github, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 6e27cce9fd48813d3f356c5f34e70b30652dae87

<a id="ktechmidas-needs-action"></a>
#### Needs action (1)
- [dashpay/platform#5866 feat(dashmate): clean up historical Docker images after updates](https://github.com/dashpay/platform/pull/5866) — ✋ changes requested · 🔴 CI failing · areas: dashmate · Policy: waiting-author
  - Blocker: Author response is required after the latest human objection

<a id="ktechmidas-changes-requested"></a>
#### Changes Requested (1)
- [dashpay/platform#5866 feat(dashmate): clean up historical Docker images after updates](https://github.com/dashpay/platform/pull/5866) — ✋ changes requested · 🔴 CI failing · areas: dashmate · Policy: waiting-author
  - Blocker: Author response is required after the latest human objection

<a id="ktechmidas-ci-failing"></a>
#### CI Failing (1)
- [dashpay/platform#6018 ci: move CI Docker images from ECR to GHCR (saves about $265 a month)](https://github.com/dashpay/platform/pull/6018) — 🔴 CI failing · areas: github, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 6e27cce9fd48813d3f356c5f34e70b30652dae87

<a id="ktechmidas-stale"></a>
#### Stale (4)
- [dashpay/platform#5188 ci: tolerate optional S3 cache export outages on chore/bump-rust-dashcore-secp-033](https://github.com/dashpay/platform/pull/5188) — 1 unresolved (1 bot) · 59 days stale · 🐢 targets chore/bump-rust-dashcore-secp-033 · areas: github
  - Top thread: "🟡 Suggestion: New regression tests are not executed by any CI workflow on this branch**" — 59 days old
- [dashpay/platform#5186 ci: tolerate optional S3 cache export outages on v5.0-dev](https://github.com/dashpay/platform/pull/5186) — 🔴 CI failing · 🐢 targets v6.0-dev · areas: github · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 843052c2dce26a80065c15b1219bbc8211264161
- [dashpay/platform#5198 ci: bound default Binaryen optimizer threads in NPM builds](https://github.com/dashpay/platform/pull/5198) — 🐢 targets v6.0-dev · areas: github · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 23b41b477bc038965b686b4a75412b1c21fc6d98
- [dashpay/platform#5205 fix(release): install verified cargo-binstall binary directly](https://github.com/dashpay/platform/pull/5205) — 🐢 targets v6.0-dev · areas: fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 8a8ac2c788b5338416e18045001d9115ba3ac80a

<a id="ktechmidas-ready-for-review"></a>
#### Ready for Review (4)
- [dashpay/platform#6249 feat(platform)!: subscribe to committed state transitions matching document and address filters](https://github.com/dashpay/platform/pull/6249) — by @QuantumExplorer · areas: rs-drive, rs-drive-abci, rust-sdk, dashmate, rust-dapi, github, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 9a4d02bdc3f9800b16e52b0546dffc23af9ddb8d
- [dashpay/platform#6319 fix(dashmate): keep Core out of Platform-only resets and make reset failures visible and recoverable](https://github.com/dashpay/platform/pull/6319) — by @shumkov · areas: dashmate · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 3a5478dcbb86053a3ab389680eac4ce67b521720
- [dashpay/platform#6325 fix(dashmate): doctor report redaction follow-ups](https://github.com/dashpay/platform/pull/6325) — by @shumkov · areas: dashmate · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed af2ab5f34c235380e35544b1828206440764430f
- [dashpay/rust-dashcore#1490 ci: test only the crates a pull request changes](https://github.com/dashpay/rust-dashcore/pull/1490) — by @ZocoLini · areas: github, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 57c19ebefff21e528e338b7cc11d43d483fc8012

<a id="zocolini"></a>
### @ZocoLini
<a id="zocolini-open"></a>
#### Open (11)
- [dashpay/rust-dashcore#1199 feat(key-wallet)!: only final coins fund a build](https://github.com/dashpay/rust-dashcore/pull/1199) — 🔴 CI failing · 📝 draft · areas: dash-spv, key-wallet, key-wallet-manager · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#1328 refactor(key-wallet)!: validate child indexes at construction with \`ChildIndex\`](https://github.com/dashpay/rust-dashcore/pull/1328) — areas: dash-spv, key-wallet, key-wallet-manager · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 0733b9d0346d6e41ca7f2f7905064240da6e074a
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/rust-dashcore#1398 fix(dash-spv): detect forks with a full block locator](https://github.com/dashpay/rust-dashcore/pull/1398) — 🔴 CI failing · areas: dash-spv · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 9475a9f8360b0a04bf435f1098250bc6302a7d64
- [dashpay/rust-dashcore#1421 fix(dash-spv): propagate storage errors from sync paths](https://github.com/dashpay/rust-dashcore/pull/1421) — 📝 draft · 🐢 targets refactor/storage-error-variants · areas: dash-spv
- [dashpay/rust-dashcore#1426 fix(dash-spv): tell an empty filter header store from genesis](https://github.com/dashpay/rust-dashcore/pull/1426) — areas: dash-spv · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed d31ec981450f9216f70fd0a3877e22563060db0c
- [dashpay/rust-dashcore#1459 ci: merge audit and verify jobs into one](https://github.com/dashpay/rust-dashcore/pull/1459) — 🔴 CI failing · 📝 draft · areas: github · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#1490 ci: test only the crates a pull request changes](https://github.com/dashpay/rust-dashcore/pull/1490) — areas: github, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 57c19ebefff21e528e338b7cc11d43d483fc8012
- [dashpay/rust-dashcore#1502 refactor(dash-spv)!: keep only storage failures in \`StorageError\`](https://github.com/dashpay/rust-dashcore/pull/1502) — areas: dash-spv · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 8523816d18ec0843407f1525fc4fdac9c46fc32f
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/rust-dashcore#1505 feat(dash-spv)!: carry peer info in peer events](https://github.com/dashpay/rust-dashcore/pull/1505) — ✋ changes requested · 🔴 CI failing · areas: dash-spv · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 70e036db56c42dfc2df308de7f00a7d520e17a02
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/rust-dashcore#1538 fix(rpc-json)!: fix \`protx info\` hash order and operator reward](https://github.com/dashpay/rust-dashcore/pull/1538) — areas: fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed a6c62aa4dee2dd59051dcd53b7fa45bb2df3605e
- [dashpay/rust-dashcore#1539 fix(rpc-json)!: decode the status of a masternode that is not ready](https://github.com/dashpay/rust-dashcore/pull/1539) — areas: fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed e0c90f77a8a7cdac445008bdd1da21b792b2dbac
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return

<a id="zocolini-needs-action"></a>
#### Needs action (1)
- [dashpay/rust-dashcore#1505 feat(dash-spv)!: carry peer info in peer events](https://github.com/dashpay/rust-dashcore/pull/1505) — ✋ changes requested · 🔴 CI failing · areas: dash-spv · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 70e036db56c42dfc2df308de7f00a7d520e17a02
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return

<a id="zocolini-changes-requested"></a>
#### Changes Requested (1)
- [dashpay/rust-dashcore#1505 feat(dash-spv)!: carry peer info in peer events](https://github.com/dashpay/rust-dashcore/pull/1505) — ✋ changes requested · 🔴 CI failing · areas: dash-spv · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 70e036db56c42dfc2df308de7f00a7d520e17a02
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return

<a id="zocolini-ci-failing"></a>
#### CI Failing (1)
- [dashpay/rust-dashcore#1398 fix(dash-spv): detect forks with a full block locator](https://github.com/dashpay/rust-dashcore/pull/1398) — 🔴 CI failing · areas: dash-spv · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 9475a9f8360b0a04bf435f1098250bc6302a7d64

<a id="zocolini-draft"></a>
#### Draft (2)
- [dashpay/rust-dashcore#1199 feat(key-wallet)!: only final coins fund a build](https://github.com/dashpay/rust-dashcore/pull/1199) — 🔴 CI failing · 📝 draft · areas: dash-spv, key-wallet, key-wallet-manager · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#1459 ci: merge audit and verify jobs into one](https://github.com/dashpay/rust-dashcore/pull/1459) — 🔴 CI failing · 📝 draft · areas: github · Policy: draft
  - Blocker: Draft PR does not occupy a review slot

<a id="zocolini-stale"></a>
#### Stale (1)
- [dashpay/rust-dashcore#1421 fix(dash-spv): propagate storage errors from sync paths](https://github.com/dashpay/rust-dashcore/pull/1421) — 📝 draft · 🐢 targets refactor/storage-error-variants · areas: dash-spv

<a id="zocolini-clean"></a>
#### Clean (6)
- [dashpay/rust-dashcore#1328 refactor(key-wallet)!: validate child indexes at construction with \`ChildIndex\`](https://github.com/dashpay/rust-dashcore/pull/1328) — areas: dash-spv, key-wallet, key-wallet-manager · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 0733b9d0346d6e41ca7f2f7905064240da6e074a
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/rust-dashcore#1426 fix(dash-spv): tell an empty filter header store from genesis](https://github.com/dashpay/rust-dashcore/pull/1426) — areas: dash-spv · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed d31ec981450f9216f70fd0a3877e22563060db0c
- [dashpay/rust-dashcore#1490 ci: test only the crates a pull request changes](https://github.com/dashpay/rust-dashcore/pull/1490) — areas: github, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 57c19ebefff21e528e338b7cc11d43d483fc8012
- [dashpay/rust-dashcore#1502 refactor(dash-spv)!: keep only storage failures in \`StorageError\`](https://github.com/dashpay/rust-dashcore/pull/1502) — areas: dash-spv · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 8523816d18ec0843407f1525fc4fdac9c46fc32f
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/rust-dashcore#1538 fix(rpc-json)!: fix \`protx info\` hash order and operator reward](https://github.com/dashpay/rust-dashcore/pull/1538) — areas: fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed a6c62aa4dee2dd59051dcd53b7fa45bb2df3605e
- [dashpay/rust-dashcore#1539 fix(rpc-json)!: decode the status of a masternode that is not ready](https://github.com/dashpay/rust-dashcore/pull/1539) — areas: fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed e0c90f77a8a7cdac445008bdd1da21b792b2dbac
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return

<a id="zocolini-ready-for-review"></a>
#### Ready for Review (1)
- [dashpay/platform#6309 fix(platform-wallet): report a token pool tree that does not build as a store error](https://github.com/dashpay/platform/pull/6309) — by @romchornyi · areas: rs-platform-wallet, rs-platform-wallet-ffi · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 4aa56d16b63d5a639c16ddfc7e4c3195f3036c58

<a id="pastapastapasta"></a>
### @PastaPastaPasta
<a id="pastapastapasta-open"></a>
#### Open (39)
- [dashpay/platform#4648 feat(drive-abci)!: state sync via ABCI snapshots with reduced platform state (protocol v15)](https://github.com/dashpay/platform/pull/4648) — 8 unresolved (8 bot) · 52 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · 📝 draft · areas: rs-drive, rs-drive-abci, dpp, fallback · Policy: draft
  - Top thread: "🟡 Suggestion: Continue the restored target through consensus and a restart**" — 52 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5786 feat(sdk): tri-state quorum lookups, bounded funding retries and a DPNS contest policy in dash-sdk-cxx](https://github.com/dashpay/platform/pull/5786) — 3 unresolved (3 bot) · 20 days stale · ⚠ merge conflict · 📝 draft · areas: rs-drive, rs-platform-wallet, rust-sdk, js-wasm-sdk, fallback · Policy: draft
  - Top thread: "✅ **Resolved** at \`e46d43eb\`; see the replies below." — 20 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5792 fix(platform-wallet): persist coins a block's re-scan credits](https://github.com/dashpay/platform/pull/5792) — 5 unresolved (5 bot) · 1 days stale · ✋ changes requested · 🔴 CI failing · 📝 draft · areas: rs-platform-wallet, wallet-storage · Policy: draft
  - Top thread: "🟡 Suggestion: Sync advances scan spent-output history even for wallets with no holds**" — 1 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#4965 test(sdk): cargo-fuzz harness over drive-proof-verifier FromProof](https://github.com/dashpay/platform/pull/4965) — 1 unresolved (1 bot) · 51 days stale · 📝 draft · areas: github, fallback · Policy: draft
  - Top thread: "🟡 Suggestion: Correct the claim that PV13 reaches the V0 proof decoder**" — 51 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/grovedb#1007 build(deps)!: use dashpay/orchard with the faster prover](https://github.com/dashpay/grovedb/pull/1007) — 🔴 CI failing · 📝 draft · areas: fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/grovedb#1054 feat: LigninDB storage engine (tracking, do not merge)](https://github.com/dashpay/grovedb/pull/1054) — 📝 draft · areas: github, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5285 feat(platform)!: subscribe to committed state transitions matching document and address filters](https://github.com/dashpay/platform/pull/5285) — ⚠ merge conflict · 📝 draft · areas: rs-drive, rs-drive-abci, rust-sdk, dashmate, rust-dapi, github, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5289 feat(sdk)!: optional BIP-39 passphrase through the mnemonic resolver and wallet creation](https://github.com/dashpay/platform/pull/5289) — ⚠ merge conflict · 📝 draft · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, rust-sdk-ffi, kotlin-sdk · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5292 feat(dashmate): state sync configuration for tenderdash and drive snapshots](https://github.com/dashpay/platform/pull/5292) — ⚠ merge conflict · 📝 draft · areas: dashmate · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5475 feat(sdk): forward DashPay profiles, invitations and masternode voting in dash-sdk-cxx](https://github.com/dashpay/platform/pull/5475) — ⚠ merge conflict · 📝 draft · areas: rs-platform-wallet, rs-platform-wallet-ffi, rust-sdk, github, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5610 perf(sdk): verify each block's signature once instead of on every proved response](https://github.com/dashpay/platform/pull/5610) — ⚠ merge conflict · 📝 draft · areas: fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5784 fix(platform-wallet): export AssetLockBuildAmount](https://github.com/dashpay/platform/pull/5784) — 📝 draft · areas: rs-platform-wallet · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5802 fix(platform-wallet-storage): don't look for or count non-funds used addresses as unresolved](https://github.com/dashpay/platform/pull/5802) — ⚠ merge conflict · 📝 draft · areas: wallet-storage, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5934 fix(platform-wallet): own the shielded coordinator and wipe host rows inside Clear](https://github.com/dashpay/platform/pull/5934) — ⚠ merge conflict · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk, kotlin-sdk · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 2e8e1cf1990e3463ae50af94b863f79d33cb3f24
- [dashpay/platform#6042 feat(platform-wallet-ffi): prefetch shielded spend anchors when a send screen opens](https://github.com/dashpay/platform/pull/6042) — ⚠ merge conflict · 🔴 CI failing · 📝 draft · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, rust-sdk, rust-sdk-ffi, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#6257 fix(platform-wallet)!: allow one live shielded store per file](https://github.com/dashpay/platform/pull/6257) — ⚠ merge conflict · 📝 draft · areas: rs-platform-wallet · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#6258 feat(platform-wallet): fund platform addresses from a shielded lock's remaining value](https://github.com/dashpay/platform/pull/6258) — ⚠ merge conflict · 📝 draft · areas: rs-platform-wallet · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#6259 fix(platform-wallet-storage): restore only InstantSend locks stored as verified](https://github.com/dashpay/platform/pull/6259) — ⚠ merge conflict · 📝 draft · areas: rs-platform-wallet, wallet-storage · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#6260 fix(platform-wallet): keep a sync pass off wallets removed, re-keyed or cleared after its list](https://github.com/dashpay/platform/pull/6260) — ⚠ merge conflict · 📝 draft · areas: rs-platform-wallet · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#6264 fix(platform-wallet-storage): keep a DashPay contact payable after it rotates its payment xpub](https://github.com/dashpay/platform/pull/6264) — 📝 draft · 🐢 targets fix/storage-verified-instant-locks · areas: wallet-storage
- [dashpay/platform#6265 feat(platform-wallet): let a host see and refuse each shielded note spend before its broadcast](https://github.com/dashpay/platform/pull/6265) — ⚠ merge conflict · 📝 draft · areas: rs-platform-wallet · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#6266 feat(platform-wallet): ask the dispatch hook to admit each shielded re-broadcast](https://github.com/dashpay/platform/pull/6266) — 📝 draft · 🐢 targets feat/platform-wallet-shielded-dispatch-hook · areas: rs-platform-wallet
- [dashpay/platform#6269 fix(platform-wallet): forward the dismissal and sweep calls](https://github.com/dashpay/platform/pull/6269) — ⚠ merge conflict · 📝 draft · areas: dpp, rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk, js-wasm-sdk, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#6270 fix(platform-wallet)!: release the wallet's store on shutdown](https://github.com/dashpay/platform/pull/6270) — ⚠ merge conflict · 📝 draft · areas: rs-platform-wallet, wallet-storage · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#6271 feat(platform-wallet): spend shielded notes with a caller-held keyset](https://github.com/dashpay/platform/pull/6271) — 📝 draft · 🐢 targets feat/platform-wallet-shielded-dispatch-admit · areas: rs-platform-wallet
- [dashpay/platform#6285 feat(drive): choose GroveDB's storage engine by configuration](https://github.com/dashpay/platform/pull/6285) — 📝 draft · areas: rs-drive, rs-drive-abci, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#6293 feat(platform-wallet-storage): persist the wallet's dismissed transactions](https://github.com/dashpay/platform/pull/6293) — 📝 draft · 🐢 targets fix/storage-verified-instant-locks · areas: rs-platform-wallet, wallet-storage
- [dashpay/platform#6302 fix(platform-wallet): store key-wallet's dismissals and restore them on load](https://github.com/dashpay/platform/pull/6302) — 📝 draft · 🐢 targets integration/dashwallet-desktop-6 · areas: rs-platform-wallet, wallet-storage
- [dashpay/rust-dashcore#1507 feat(key-wallet-manager)!: give \`TransactionsSwept\` the winner's block](https://github.com/dashpay/rust-dashcore/pull/1507) — 📝 draft · areas: key-wallet-manager · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#1508 fix(key-wallet-manager): announce an InstantSend lock that came first](https://github.com/dashpay/rust-dashcore/pull/1508) — 📝 draft · areas: key-wallet-manager · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#1509 feat(key-wallet): add \`Signer::approve_inputs\`](https://github.com/dashpay/rust-dashcore/pull/1509) — 📝 draft · areas: key-wallet · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#1511 fix(dash-spv): certify a wallet only over scripts its batch tested](https://github.com/dashpay/rust-dashcore/pull/1511) — 📝 draft · areas: dash-spv, key-wallet-manager · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#1512 feat(dash-spv): opt-in mempool relay while filters still sync](https://github.com/dashpay/rust-dashcore/pull/1512) — 📝 draft · areas: dash-spv · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#1516 fix(dash-spv): look at the mempool again when the wallets change](https://github.com/dashpay/rust-dashcore/pull/1516) — 📝 draft · areas: dash-spv, key-wallet-manager · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#1518 feat(key-wallet)!: dismissed transactions stay out until settled](https://github.com/dashpay/rust-dashcore/pull/1518) — 🔴 CI failing · 📝 draft · areas: dash-spv, key-wallet, key-wallet-manager · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#1519 fix(key-wallet): credit again a swept loser's coins only it spent](https://github.com/dashpay/rust-dashcore/pull/1519) — 🔴 CI failing · 📝 draft · areas: dash-spv, key-wallet, key-wallet-manager · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#1520 fix(key-wallet-manager): keep a conflict for its ChainLock](https://github.com/dashpay/rust-dashcore/pull/1520) — 📝 draft · areas: dash-spv, key-wallet, key-wallet-manager · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#1522 feat(key-wallet-manager)!: report dismissal changes for persistence](https://github.com/dashpay/rust-dashcore/pull/1522) — 📝 draft · areas: dash-spv, key-wallet, key-wallet-manager · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#1523 fix(key-wallet): a pending spend needs its owner's signature](https://github.com/dashpay/rust-dashcore/pull/1523) — 🔴 CI failing · 📝 draft · areas: dash-spv, key-wallet, key-wallet-manager, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot

<a id="pastapastapasta-needs-action"></a>
#### Needs action (1)
- [dashpay/platform#5934 fix(platform-wallet): own the shielded coordinator and wipe host rows inside Clear](https://github.com/dashpay/platform/pull/5934) — ⚠ merge conflict · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk, kotlin-sdk · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 2e8e1cf1990e3463ae50af94b863f79d33cb3f24

<a id="pastapastapasta-draft"></a>
#### Draft (33)
- [dashpay/platform#4648 feat(drive-abci)!: state sync via ABCI snapshots with reduced platform state (protocol v15)](https://github.com/dashpay/platform/pull/4648) — 8 unresolved (8 bot) · 52 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · 📝 draft · areas: rs-drive, rs-drive-abci, dpp, fallback · Policy: draft
  - Top thread: "🟡 Suggestion: Continue the restored target through consensus and a restart**" — 52 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5786 feat(sdk): tri-state quorum lookups, bounded funding retries and a DPNS contest policy in dash-sdk-cxx](https://github.com/dashpay/platform/pull/5786) — 3 unresolved (3 bot) · 20 days stale · ⚠ merge conflict · 📝 draft · areas: rs-drive, rs-platform-wallet, rust-sdk, js-wasm-sdk, fallback · Policy: draft
  - Top thread: "✅ **Resolved** at \`e46d43eb\`; see the replies below." — 20 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5792 fix(platform-wallet): persist coins a block's re-scan credits](https://github.com/dashpay/platform/pull/5792) — 5 unresolved (5 bot) · 1 days stale · ✋ changes requested · 🔴 CI failing · 📝 draft · areas: rs-platform-wallet, wallet-storage · Policy: draft
  - Top thread: "🟡 Suggestion: Sync advances scan spent-output history even for wallets with no holds**" — 1 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#4965 test(sdk): cargo-fuzz harness over drive-proof-verifier FromProof](https://github.com/dashpay/platform/pull/4965) — 1 unresolved (1 bot) · 51 days stale · 📝 draft · areas: github, fallback · Policy: draft
  - Top thread: "🟡 Suggestion: Correct the claim that PV13 reaches the V0 proof decoder**" — 51 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/grovedb#1007 build(deps)!: use dashpay/orchard with the faster prover](https://github.com/dashpay/grovedb/pull/1007) — 🔴 CI failing · 📝 draft · areas: fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/grovedb#1054 feat: LigninDB storage engine (tracking, do not merge)](https://github.com/dashpay/grovedb/pull/1054) — 📝 draft · areas: github, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5285 feat(platform)!: subscribe to committed state transitions matching document and address filters](https://github.com/dashpay/platform/pull/5285) — ⚠ merge conflict · 📝 draft · areas: rs-drive, rs-drive-abci, rust-sdk, dashmate, rust-dapi, github, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5289 feat(sdk)!: optional BIP-39 passphrase through the mnemonic resolver and wallet creation](https://github.com/dashpay/platform/pull/5289) — ⚠ merge conflict · 📝 draft · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, rust-sdk-ffi, kotlin-sdk · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5292 feat(dashmate): state sync configuration for tenderdash and drive snapshots](https://github.com/dashpay/platform/pull/5292) — ⚠ merge conflict · 📝 draft · areas: dashmate · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5475 feat(sdk): forward DashPay profiles, invitations and masternode voting in dash-sdk-cxx](https://github.com/dashpay/platform/pull/5475) — ⚠ merge conflict · 📝 draft · areas: rs-platform-wallet, rs-platform-wallet-ffi, rust-sdk, github, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5610 perf(sdk): verify each block's signature once instead of on every proved response](https://github.com/dashpay/platform/pull/5610) — ⚠ merge conflict · 📝 draft · areas: fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5784 fix(platform-wallet): export AssetLockBuildAmount](https://github.com/dashpay/platform/pull/5784) — 📝 draft · areas: rs-platform-wallet · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5802 fix(platform-wallet-storage): don't look for or count non-funds used addresses as unresolved](https://github.com/dashpay/platform/pull/5802) — ⚠ merge conflict · 📝 draft · areas: wallet-storage, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#6042 feat(platform-wallet-ffi): prefetch shielded spend anchors when a send screen opens](https://github.com/dashpay/platform/pull/6042) — ⚠ merge conflict · 🔴 CI failing · 📝 draft · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, rust-sdk, rust-sdk-ffi, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#6257 fix(platform-wallet)!: allow one live shielded store per file](https://github.com/dashpay/platform/pull/6257) — ⚠ merge conflict · 📝 draft · areas: rs-platform-wallet · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#6258 feat(platform-wallet): fund platform addresses from a shielded lock's remaining value](https://github.com/dashpay/platform/pull/6258) — ⚠ merge conflict · 📝 draft · areas: rs-platform-wallet · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#6259 fix(platform-wallet-storage): restore only InstantSend locks stored as verified](https://github.com/dashpay/platform/pull/6259) — ⚠ merge conflict · 📝 draft · areas: rs-platform-wallet, wallet-storage · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#6260 fix(platform-wallet): keep a sync pass off wallets removed, re-keyed or cleared after its list](https://github.com/dashpay/platform/pull/6260) — ⚠ merge conflict · 📝 draft · areas: rs-platform-wallet · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#6265 feat(platform-wallet): let a host see and refuse each shielded note spend before its broadcast](https://github.com/dashpay/platform/pull/6265) — ⚠ merge conflict · 📝 draft · areas: rs-platform-wallet · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#6269 fix(platform-wallet): forward the dismissal and sweep calls](https://github.com/dashpay/platform/pull/6269) — ⚠ merge conflict · 📝 draft · areas: dpp, rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk, js-wasm-sdk, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#6270 fix(platform-wallet)!: release the wallet's store on shutdown](https://github.com/dashpay/platform/pull/6270) — ⚠ merge conflict · 📝 draft · areas: rs-platform-wallet, wallet-storage · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#6285 feat(drive): choose GroveDB's storage engine by configuration](https://github.com/dashpay/platform/pull/6285) — 📝 draft · areas: rs-drive, rs-drive-abci, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#1507 feat(key-wallet-manager)!: give \`TransactionsSwept\` the winner's block](https://github.com/dashpay/rust-dashcore/pull/1507) — 📝 draft · areas: key-wallet-manager · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#1508 fix(key-wallet-manager): announce an InstantSend lock that came first](https://github.com/dashpay/rust-dashcore/pull/1508) — 📝 draft · areas: key-wallet-manager · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#1509 feat(key-wallet): add \`Signer::approve_inputs\`](https://github.com/dashpay/rust-dashcore/pull/1509) — 📝 draft · areas: key-wallet · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#1511 fix(dash-spv): certify a wallet only over scripts its batch tested](https://github.com/dashpay/rust-dashcore/pull/1511) — 📝 draft · areas: dash-spv, key-wallet-manager · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#1512 feat(dash-spv): opt-in mempool relay while filters still sync](https://github.com/dashpay/rust-dashcore/pull/1512) — 📝 draft · areas: dash-spv · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#1516 fix(dash-spv): look at the mempool again when the wallets change](https://github.com/dashpay/rust-dashcore/pull/1516) — 📝 draft · areas: dash-spv, key-wallet-manager · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#1518 feat(key-wallet)!: dismissed transactions stay out until settled](https://github.com/dashpay/rust-dashcore/pull/1518) — 🔴 CI failing · 📝 draft · areas: dash-spv, key-wallet, key-wallet-manager · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#1519 fix(key-wallet): credit again a swept loser's coins only it spent](https://github.com/dashpay/rust-dashcore/pull/1519) — 🔴 CI failing · 📝 draft · areas: dash-spv, key-wallet, key-wallet-manager · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#1520 fix(key-wallet-manager): keep a conflict for its ChainLock](https://github.com/dashpay/rust-dashcore/pull/1520) — 📝 draft · areas: dash-spv, key-wallet, key-wallet-manager · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#1522 feat(key-wallet-manager)!: report dismissal changes for persistence](https://github.com/dashpay/rust-dashcore/pull/1522) — 📝 draft · areas: dash-spv, key-wallet, key-wallet-manager · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#1523 fix(key-wallet): a pending spend needs its owner's signature](https://github.com/dashpay/rust-dashcore/pull/1523) — 🔴 CI failing · 📝 draft · areas: dash-spv, key-wallet, key-wallet-manager, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot

<a id="pastapastapasta-stale"></a>
#### Stale (5)
- [dashpay/platform#6264 fix(platform-wallet-storage): keep a DashPay contact payable after it rotates its payment xpub](https://github.com/dashpay/platform/pull/6264) — 📝 draft · 🐢 targets fix/storage-verified-instant-locks · areas: wallet-storage
- [dashpay/platform#6266 feat(platform-wallet): ask the dispatch hook to admit each shielded re-broadcast](https://github.com/dashpay/platform/pull/6266) — 📝 draft · 🐢 targets feat/platform-wallet-shielded-dispatch-hook · areas: rs-platform-wallet
- [dashpay/platform#6271 feat(platform-wallet): spend shielded notes with a caller-held keyset](https://github.com/dashpay/platform/pull/6271) — 📝 draft · 🐢 targets feat/platform-wallet-shielded-dispatch-admit · areas: rs-platform-wallet
- [dashpay/platform#6293 feat(platform-wallet-storage): persist the wallet's dismissed transactions](https://github.com/dashpay/platform/pull/6293) — 📝 draft · 🐢 targets fix/storage-verified-instant-locks · areas: rs-platform-wallet, wallet-storage
- [dashpay/platform#6302 fix(platform-wallet): store key-wallet's dismissals and restore them on load](https://github.com/dashpay/platform/pull/6302) — 📝 draft · 🐢 targets integration/dashwallet-desktop-6 · areas: rs-platform-wallet, wallet-storage

<a id="pastapastapasta-clean"></a>
#### Clean (1)
- [dashpay/platform#5934 fix(platform-wallet): own the shielded coordinator and wipe host rows inside Clear](https://github.com/dashpay/platform/pull/5934) — ⚠ merge conflict · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk, kotlin-sdk · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 2e8e1cf1990e3463ae50af94b863f79d33cb3f24

<a id="thepastaclaw"></a>
### @thepastaclaw
<a id="thepastaclaw-open"></a>
#### Open (1)
- [dashpay/platform#5581 feat(platform): add proof-backed asset lock outpoint query](https://github.com/dashpay/platform/pull/5581) — ⚠ merge conflict · 🔴 CI failing · areas: rs-drive, rs-drive-abci, dpp, rs-platform-wallet, rust-sdk, rust-dapi, sdk-parity, fallback · Policy: waiting-author
  - Blocker: coderabbitai left review threads unresolved; resolve them
  - Blocker: thepastaclaw left review threads unresolved; resolve them

<a id="thepastaclaw-needs-action"></a>
#### Needs action (1)
- [dashpay/platform#5581 feat(platform): add proof-backed asset lock outpoint query](https://github.com/dashpay/platform/pull/5581) — ⚠ merge conflict · 🔴 CI failing · areas: rs-drive, rs-drive-abci, dpp, rs-platform-wallet, rust-sdk, rust-dapi, sdk-parity, fallback · Policy: waiting-author
  - Blocker: coderabbitai left review threads unresolved; resolve them
  - Blocker: thepastaclaw left review threads unresolved; resolve them

<a id="thepastaclaw-ci-failing"></a>
#### CI Failing (1)
- [dashpay/platform#5581 feat(platform): add proof-backed asset lock outpoint query](https://github.com/dashpay/platform/pull/5581) — ⚠ merge conflict · 🔴 CI failing · areas: rs-drive, rs-drive-abci, dpp, rs-platform-wallet, rust-sdk, rust-dapi, sdk-parity, fallback · Policy: waiting-author
  - Blocker: coderabbitai left review threads unresolved; resolve them
  - Blocker: thepastaclaw left review threads unresolved; resolve them

<a id="dcg-claude"></a>
### @DCG-Claude
<a id="dcg-claude-open"></a>
#### Open (3)
- [dashpay/platform#4794 feat: pin contract configuration vectors and mirror them in wasm-dpp2 and the sdk ffi](https://github.com/dashpay/platform/pull/4794) — 🔴 CI failing · areas: dpp, rust-sdk-ffi, fallback · Policy: waiting-build
  - Blocker: The build must pass before a human is asked
- [dashpay/platform#4934 docs: correct the smart-contract target, version counts, transition list and mobile sdk availability](https://github.com/dashpay/platform/pull/4934) — 🐢 targets v6.0-dev · areas: swift-sdk, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
- [dashpay/platform#4938 docs(platform): distinguish current-state proofs, stored receipts and execution result text](https://github.com/dashpay/platform/pull/4938) — 🐢 targets v6.0-dev · areas: dpp, rust-sdk, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required

<a id="dcg-claude-ready-for-human"></a>
#### Ready for human (2)
- [dashpay/platform#4934 docs: correct the smart-contract target, version counts, transition list and mobile sdk availability](https://github.com/dashpay/platform/pull/4934) — 🐢 targets v6.0-dev · areas: swift-sdk, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
- [dashpay/platform#4938 docs(platform): distinguish current-state proofs, stored receipts and execution result text](https://github.com/dashpay/platform/pull/4938) — 🐢 targets v6.0-dev · areas: dpp, rust-sdk, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required

<a id="dcg-claude-ci-failing"></a>
#### CI Failing (1)
- [dashpay/platform#4794 feat: pin contract configuration vectors and mirror them in wasm-dpp2 and the sdk ffi](https://github.com/dashpay/platform/pull/4794) — 🔴 CI failing · areas: dpp, rust-sdk-ffi, fallback · Policy: waiting-build
  - Blocker: The build must pass before a human is asked

<a id="dcg-claude-stale"></a>
#### Stale (2)
- [dashpay/platform#4934 docs: correct the smart-contract target, version counts, transition list and mobile sdk availability](https://github.com/dashpay/platform/pull/4934) — 🐢 targets v6.0-dev · areas: swift-sdk, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
- [dashpay/platform#4938 docs(platform): distinguish current-state proofs, stored receipts and execution result text](https://github.com/dashpay/platform/pull/4938) — 🐢 targets v6.0-dev · areas: dpp, rust-sdk, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required

## Methodology
Generated nightly by [pr-hygiene](https://github.com/dashpay/stale_prs_are_bad). A thread counts as "unresolved" when it is open, not outdated, has a comment from someone other than the PR author, and the most recent comment is from a reviewer. **Dirty** = at least one such thread. **Unresolved Comments** = at least one such thread. **Changes Requested** = no unresolved threads but a reviewer's most recent review is CHANGES_REQUESTED (still blocking until someone re-approves or dismisses). **Deferred** = carries a configured deferred label (e.g. `postponed`) — visible but not counted toward unresolved-comment counts. **Stale** = targets a non-default branch OR hasn't been touched in the configured threshold (default 120 days, but clean PRs are never reclassified as stale). **Draft** = the PR is still marked draft on GitHub. **CI failing** = no unresolved comments, no changes-requested, but the latest commit's status check is failing. **Clean** = open, not draft, not deferred, not stale, no unresolved comments, no changes-requested, CI green. **Needs action** further requires changes-requested, merge conflict, or that the reviewer commented more recently than the author last pushed. **Ready for human** counts a person's own PRs that the shared review engine marks `ready-for-human` or `ready-to-merge`; each PR bullet shows the engine's state as `Policy:` with its blockers. **Ready for Review** counts clean PRs (authored by someone else) where this person owes a review: the union of the shared policy's routing (owners and reviewers of every area the changed files fall into, or the repository fallback for files no area claims) and GitHub's explicit review requests. The author is never routed to their own PR, and anyone who has already submitted any review is excluded — their job is done. `⚠ ownership unresolved` marks areas whose roster the policy still lists as open. Configurable via [`https://github.com/dashpay/stale_prs_are_bad/blob/master/.pr-hygiene.yml`](https://github.com/dashpay/stale_prs_are_bad/blob/master/.pr-hygiene.yml)—edit defaults there; ownership lives in `policies/`.
