---
---
# PR Hygiene Report
*Last updated: 2026-11-24 18:11 UTC · commit 171af2b*

## Summary
- Open PRs: **190** (10 clean · 3 CI failing · 6 changes requested · 19 unresolved comments · 4 deferred · 15 draft · 133 stale)
- PRs needing author action: **25**
- Total unresolved comments: **290**
- dashpay/platform: **116** open (2 clean · 0 CI failing · 1 changes requested · 3 unresolved comments · 4 deferred · 0 draft · 106 stale) · engine: 23 draft · 6 ready-for-human · 2 ready-to-merge · 23 waiting-author · 4 waiting-bots · 2 waiting-build · 14 waiting-self-review · 42 no verdict
- dashpay/rust-dashcore: **24** open (4 clean · 2 CI failing · 3 changes requested · 2 unresolved comments · 0 deferred · 7 draft · 6 stale) · engine: 7 draft · 1 ready-for-human · 2 waiting-author · 2 waiting-bots · 6 waiting-self-review · 6 no verdict
- dashpay/tenderdash: **9** open (1 clean · 0 CI failing · 0 changes requested · 4 unresolved comments · 0 deferred · 4 draft · 0 stale) · engine: 4 draft · 3 waiting-author · 2 waiting-self-review
- dashpay/grovedb: **3** open (1 clean · 0 CI failing · 0 changes requested · 0 unresolved comments · 0 deferred · 2 draft · 0 stale) · engine: 2 draft · 1 waiting-self-review
- dashpay/dash-evo-tool: **38** open (2 clean · 1 CI failing · 2 changes requested · 10 unresolved comments · 0 deferred · 2 draft · 21 stale) · engine: 2 draft · 1 ready-to-merge · 13 waiting-author · 1 waiting-bots · 21 no verdict

## Scoreboard
_Sort: unresolved-comments desc → needs-action desc → ready-for-review desc. Click any number to jump to the specific PRs it covers._

| Author | Open | Clean | CI failing | Unresolved Comments | Changes Requested | Deferred | Draft | Stale | Needs action | Ready for human | Total Unresolved Comments | Ready for Review |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| [@Claudius-Maginificent](#claudius-maginificent) | [65](#claudius-maginificent-open) | [1](#claudius-maginificent-clean) | [1](#claudius-maginificent-ci-failing) | [10](#claudius-maginificent-unresolved-comments) | [1](#claudius-maginificent-changes-requested) | [3](#claudius-maginificent-deferred) | [4](#claudius-maginificent-draft) | [45](#claudius-maginificent-stale) | [11](#claudius-maginificent-needs-action) | [2](#claudius-maginificent-ready-for-human) | [124](#claudius-maginificent-unresolved-comments) | — |
| [@QuantumExplorer](#quantumexplorer) | [22](#quantumexplorer-open) | [4](#quantumexplorer-clean) | — | [3](#quantumexplorer-unresolved-comments) | [1](#quantumexplorer-changes-requested) | — | [1](#quantumexplorer-draft) | [13](#quantumexplorer-stale) | [4](#quantumexplorer-needs-action) | — | [41](#quantumexplorer-unresolved-comments) | [4](#quantumexplorer-ready-for-review) |
| [@shumkov](#shumkov) | [7](#shumkov-open) | — | — | [3](#shumkov-unresolved-comments) | — | — | — | [4](#shumkov-stale) | [3](#shumkov-needs-action) | — | [27](#shumkov-unresolved-comments) | [2](#shumkov-ready-for-review) |
| [@lklimek](#lklimek) | [20](#lklimek-open) | [1](#lklimek-clean) | — | [2](#lklimek-unresolved-comments) | [3](#lklimek-changes-requested) | [1](#lklimek-deferred) | [3](#lklimek-draft) | [10](#lklimek-stale) | [5](#lklimek-needs-action) | [2](#lklimek-ready-for-human) | [15](#lklimek-unresolved-comments) | — |
| [@xdustinface](#xdustinface) | [9](#xdustinface-open) | — | — | [1](#xdustinface-unresolved-comments) | [1](#xdustinface-changes-requested) | — | [3](#xdustinface-draft) | [4](#xdustinface-stale) | [2](#xdustinface-needs-action) | — | [3](#xdustinface-unresolved-comments) | [4](#xdustinface-ready-for-review) |
| [@romchornyi](#romchornyi) | [15](#romchornyi-open) | — | — | — | — | — | — | [15](#romchornyi-stale) | — | [2](#romchornyi-ready-for-human) | [32](#romchornyi-unresolved-comments) | [2](#romchornyi-ready-for-review) |
| [@llbartekll](#llbartekll) | [4](#llbartekll-open) | — | — | — | — | — | — | [4](#llbartekll-stale) | — | [1](#llbartekll-ready-for-human) | [3](#llbartekll-unresolved-comments) | [2](#llbartekll-ready-for-review) |
| [@HashEngineering](#hashengineering) | [6](#hashengineering-open) | — | — | — | — | — | — | [6](#hashengineering-stale) | — | — | [16](#hashengineering-unresolved-comments) | [1](#hashengineering-ready-for-review) |
| [@ktechmidas](#ktechmidas) | [6](#ktechmidas-open) | — | — | — | — | — | — | [6](#ktechmidas-stale) | — | — | [1](#ktechmidas-unresolved-comments) | [1](#ktechmidas-ready-for-review) |
| [@ZocoLini](#zocolini) | [9](#zocolini-open) | [4](#zocolini-clean) | [2](#zocolini-ci-failing) | — | — | — | [2](#zocolini-draft) | [1](#zocolini-stale) | — | — | — | [1](#zocolini-ready-for-review) |
| [@PastaPastaPasta](#pastapastapasta) | [20](#pastapastapasta-open) | — | — | — | — | — | [2](#pastapastapasta-draft) | [18](#pastapastapasta-stale) | — | [1](#pastapastapasta-ready-for-human) | [20](#pastapastapasta-unresolved-comments) | — |
| [@bfoss765](#bfoss765) | [2](#bfoss765-open) | — | — | — | — | — | — | [2](#bfoss765-stale) | — | — | [7](#bfoss765-unresolved-comments) | — |
| [@thephez](#thephez) | [1](#thephez-open) | — | — | — | — | — | — | [1](#thephez-stale) | — | — | [1](#thephez-unresolved-comments) | — |
| [@DCG-Claude](#dcg-claude) | [3](#dcg-claude-open) | — | — | — | — | — | — | [3](#dcg-claude-stale) | — | [2](#dcg-claude-ready-for-human) | — | — |
| [@thepastaclaw](#thepastaclaw) | [1](#thepastaclaw-open) | — | — | — | — | — | — | [1](#thepastaclaw-stale) | — | — | — | — |

## Per-author detail

<a id="claudius-maginificent"></a>
### @Claudius-Maginificent
<a id="claudius-maginificent-open"></a>
#### Open (65)
- [dashpay/platform#5645 fix(platform-wallet)!: check identity creation keys and funding in the Rust core](https://github.com/dashpay/platform/pull/5645) — 9 unresolved (1 CodeRabbit, 8 bot) · 21 days stale · ✋ changes requested · 🔴 CI failing · 🐢 targets v5.2-dev · areas: dpp, rs-platform-wallet, rs-platform-wallet-ffi, rust-sdk · Policy: waiting-author
  - Top thread: "&lt;picture&gt;&lt;img src="https://storage.googleapis.com/coderabbit_public_assets/review-comment-icons-v1/amber-16.svg" width="…" — 21 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: coderabbitai left review threads unresolved; resolve them
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/dash-evo-tool#1177 feat(ui): align app-wide styles with the token proposal](https://github.com/dashpay/dash-evo-tool/pull/1177) — 13 unresolved (13 bot) · 6 days stale · ✋ changes requested · 📝 draft · 🐢 targets feat/token-creator-workspace · areas: fallback
  - Top thread: "✅ **Resolved** at \`0993a6af\`; see the replies below." — 6 days old
- [dashpay/dash-evo-tool#1178 fix(tokens): keep JSON export responsive when the file chooser stalls](https://github.com/dashpay/dash-evo-tool/pull/1178) — 15 unresolved (7 CodeRabbit, 8 bot) · 4 days stale · ✋ changes requested · 📝 draft · 🐢 targets feat/app-design-system · areas: fallback
  - Top thread: "🟡 Suggestion: Exercise a publication failure after staging succeeds**" — 4 days old
- [dashpay/platform#5565 feat(platform-wallet)!: detect inconsistent shielded commitment trees and add explicit rebuild](https://github.com/dashpay/platform/pull/5565) — 7 unresolved (7 bot) · 25 days stale · ✋ changes requested · 🔴 CI failing · 🐢 targets v5.2-dev · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk · Policy: waiting-author
  - Top thread: "⏸️ **Needs a tracking issue** (not yet deferred) at \`3b17ab95\`; see the replies below." — 25 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5588 feat(platform-wallet)!: expose protocol-14-aware signing-key selection](https://github.com/dashpay/platform/pull/5588) — 5 unresolved (5 bot) · 25 days stale · 🔴 CI failing · 🐢 targets v5.2-dev · areas: dpp, rs-platform-wallet, rs-platform-wallet-ffi, rust-sdk · Policy: waiting-author
  - Top thread: "⏭️ **Deferred** (maintainer decision; not blocking) at \`7479fdcb\`; see the replies below." — 25 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/dash-evo-tool#1175 docs: add maintainable token tutorial production toolkit](https://github.com/dashpay/dash-evo-tool/pull/1175) — 12 unresolved (12 CodeRabbit) · 0 days stale · areas: fallback · Policy: waiting-author
  - Top thread: "&lt;picture&gt;&lt;img src="https://storage.googleapis.com/coderabbit_public_assets/review-comment-icons-v1/major-16.svg" width="…" — 0 days old
  - Blocker: coderabbitai left review threads unresolved; resolve them
  - Blocker: Proceeded without thepastaclaw: no review within the configured window
- [dashpay/dash-evo-tool#1108 feat(documents): recognise indexOnly document types and delete their entries by values](https://github.com/dashpay/dash-evo-tool/pull/1108) — 3 unresolved (3 bot) · 32 days stale · ✋ changes requested · 📝 draft · 🐢 targets feat/document-optional-token-cost · areas: fallback
  - Top thread: "🟡 Suggestion: Make the non-deletable error actionable**" — 32 days old
- [dashpay/dash-evo-tool#1200 feat(wallet)!: detect databases needing the platform bridge and remove the bridge](https://github.com/dashpay/dash-evo-tool/pull/1200) — 10 unresolved (5 CodeRabbit, 5 bot) · 2 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · areas: fallback · Policy: waiting-author
  - Top thread: "✅ **Resolved** at \`de6f75ee\`; see the replies below." — 2 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: coderabbitai left review threads unresolved; resolve them
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/dash-evo-tool#1214 fix(tokens): identity token model](https://github.com/dashpay/dash-evo-tool/pull/1214) — 9 unresolved (9 bot) · 1 days stale · ✋ changes requested · 🔴 CI failing · areas: fallback · Policy: waiting-author
  - Top thread: "🟡 Suggestion: My Tokens sorts names by characters it no longer displays**" — 1 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/platform#5644 fix!: follow DIP-11 key rules when disabling keys and reading contact requests](https://github.com/dashpay/platform/pull/5644) — 8 unresolved (8 bot) · 2 days stale · ✋ changes requested · 🔴 CI failing · 🐢 targets v5.2-dev · areas: dpp, rs-platform-wallet, rs-platform-wallet-ffi, rust-sdk, rust-sdk-ffi, js-wasm-sdk, fallback · Policy: waiting-author
  - Top thread: "✅ **Resolved** at \`ed16501f\`; see the replies below." — 2 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5546 fix(platform-wallet): support HASH160 DashPay contact signing keys](https://github.com/dashpay/platform/pull/5546) — 2 unresolved (2 bot) · 25 days stale · 📝 draft · 🐢 targets feat/platform-wallet-public-signing-key-selection · areas: rs-drive-abci, dpp, rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk, fallback
  - Top thread: "🟡 Suggestion: Do not expose an unchecked signer-capability override in the public constructor**" — 25 days old
- [dashpay/dash-evo-tool#1179 fix(tokens): retain the creator when opening a registered contract](https://github.com/dashpay/dash-evo-tool/pull/1179) — 4 unresolved (4 bot) · 4 days stale · ✋ changes requested · 📝 draft · 🐢 targets fix/token-json-export · areas: fallback
  - Top thread: "🟡 Suggestion: Assert that the preview displays fetched contract details**" — 4 days old
- [dashpay/dash-evo-tool#1225 fix(boot): give accurate advice for invalid settings and a denied vault](https://github.com/dashpay/dash-evo-tool/pull/1225) — 6 unresolved (1 CodeRabbit, 5 bot) · 1 days stale · ✋ changes requested · 🔴 CI failing · areas: fallback · Policy: waiting-author
  - Top thread: "&lt;picture&gt;&lt;img src="https://storage.googleapis.com/coderabbit_public_assets/review-comment-icons-v1/amber-16.svg" width="…" — 1 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: coderabbitai left review threads unresolved; resolve them
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/tenderdash#1600 fix(consensus): deliver and process a polka block for a validator left in its round](https://github.com/dashpay/tenderdash/pull/1600) — 2 unresolved (2 bot) · 7 days stale · ✋ changes requested · 🔴 CI failing · areas: fallback · Policy: waiting-author
  - Top thread: "🔴 Blocking: Avoid racing on PartSet completeness in the gossip snapshot**" — 7 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5981 refactor(platform-wallet-storage): drop the unreachable rehydration gap-refill guard](https://github.com/dashpay/platform/pull/5981) — 2 unresolved (2 bot) · 6 days stale · ✋ changes requested · 📝 draft · 🐢 targets refactor/platform-node-pool-single-owner · areas: wallet-storage
  - Top thread: "🔴 Blocking: Preserve script-based ownership after a failed refill**" — 6 days old
- [dashpay/dash-evo-tool#1124 feat(documents): let the user skip an optional token cost](https://github.com/dashpay/dash-evo-tool/pull/1124) — 1 unresolved (1 bot) · 29 days stale · ⚠ merge conflict · ✋ changes requested · 📝 draft · 🐢 targets fix/protocol14-signing-key-selection · areas: fallback
  - Top thread: "🔴 Blocking: Show the credit-fee outcome when an optional token cost is disabled**" — 29 days old
- [dashpay/platform#5707 fix(sdk): recover the nonce cache when unconfirmed nonces leave the accepted window](https://github.com/dashpay/platform/pull/5707) — 1 unresolved (1 bot) · 21 days stale · ✋ changes requested · 🔴 CI failing · 📝 draft · 🐢 targets v5.2-dev · areas: rust-sdk · Policy: draft
  - Top thread: "🔴 Blocking: Do not reissue nonces belonging to outstanding operations**" — 21 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/dash-evo-tool#1117 fix(identity): choose signing keys Platform accepts on protocol 14](https://github.com/dashpay/dash-evo-tool/pull/1117) — 3 unresolved (3 bot) · 1 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · areas: fallback · Policy: waiting-author
  - Top thread: "🟡 Suggestion: The signing selection type can report a usable key without containing one**" — 1 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/dash-evo-tool#1174 feat(skills): mark defects in visual-diff galleries](https://github.com/dashpay/dash-evo-tool/pull/1174) — 3 unresolved (3 bot) · 1 days stale · ✋ changes requested · 🔴 CI failing · areas: fallback · Policy: waiting-author
  - Top thread: "🟡 Suggestion: The skill tells reviewers to omit unchanged regions and banners that show confirmed defects**" — 1 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/dash-evo-tool#1240 feat(send): clearer confirmation and a steady form width](https://github.com/dashpay/dash-evo-tool/pull/1240) — 3 unresolved (3 bot) · 1 days stale · ✋ changes requested · 🔴 CI failing · areas: fallback · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Opening a batch confirmation reloads wallet and identity data for every recipient**" — 1 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/dash-evo-tool#1241 feat(ci): review each weekly build in the GUI before publishing it](https://github.com/dashpay/dash-evo-tool/pull/1241) — 2 unresolved (2 bot) · 0 days stale · ✋ changes requested · 🔴 CI failing · areas: github, fallback · Policy: waiting-bots
  - Top thread: "🔴 Blocking: Cleanup abandons the refund when Insight lists the same payout twice as it confirms**" — 0 days old
  - Blocker: coderabbitai has not reported for the current head
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5979 fix(sdk): keep PublicKey unchanged when the inner setter rejects a key](https://github.com/dashpay/platform/pull/5979) — 1 unresolved (1 bot) · 6 days stale · 🔴 CI failing · 📝 draft · 🐢 targets chore/crypto-core-consumers · areas: fallback
  - Top thread: "🟡 Suggestion: Add successful cross-format setter coverage**" — 6 days old
- [dashpay/dash-evo-tool#1206 fix(boot): show startup panics and window failures instead of closing](https://github.com/dashpay/dash-evo-tool/pull/1206) — 1 unresolved (1 bot) · 2 days stale · areas: fallback · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Startup tests pass even if the native alert is shown at the wrong time**" — 2 days old
  - Blocker: thepastaclaw left review threads unresolved; resolve them
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/platform#5647 feat: expose the Rust identity key layout to FFI, Swift and Kotlin hosts](https://github.com/dashpay/platform/pull/5647) — 1 unresolved (1 bot) · 2 days stale · ✋ changes requested · 🔴 CI failing · 📝 draft · 🐢 targets fix/platform-wallet-dip11-identity-key-layout · areas: rs-platform-wallet-ffi, swift-sdk, kotlin-sdk
  - Top thread: "🟡 Suggestion: Kotlin reports a failed protocol-version lookup as non-retryable**" — 2 days old
- [dashpay/platform#5661 feat: decide which identity keys may be disabled in the wallet, not in the mobile apps](https://github.com/dashpay/platform/pull/5661) — 1 unresolved (1 bot) · 0 days stale · 🔴 CI failing · 📝 draft · 🐢 targets fix/platform-wallet-dip11-key-rules · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk
  - Top thread: "🟡 Suggestion: The last-TRANSFER test passes even if submission checks the pre-refresh key set**" — 0 days old
- [dashpay/dash-evo-tool#1120 feat(identity): add keys with spending limits and raise key limits](https://github.com/dashpay/dash-evo-tool/pull/1120) — ⚠ merge conflict · 📝 draft · 🐢 targets fix/protocol14-signing-key-selection · areas: fallback
- [dashpay/dash-evo-tool#1121 feat(contracts): view and claim contract fee pots](https://github.com/dashpay/dash-evo-tool/pull/1121) — 📝 draft · 🐢 targets fix/protocol14-signing-key-selection · areas: fallback
- [dashpay/dash-evo-tool#1122 feat(documents): agree to document action fees before broadcasting](https://github.com/dashpay/dash-evo-tool/pull/1122) — ⚠ merge conflict · 📝 draft · 🐢 targets fix/protocol14-signing-key-selection · areas: fallback
- [dashpay/dash-evo-tool#1123 feat(identity): add authentication keys bound to a contract](https://github.com/dashpay/dash-evo-tool/pull/1123) — 📝 draft · 🐢 targets feat/identity-key-limits · areas: fallback
- [dashpay/dash-evo-tool#1137 fix(identity): unlock the wallet that holds the chosen signing key](https://github.com/dashpay/dash-evo-tool/pull/1137) — ⚠ merge conflict · 📝 draft · 🐢 targets fix/protocol14-signing-key-selection · areas: fallback
- [dashpay/dash-evo-tool#1176 feat(tokens): add guided token creation with complete review](https://github.com/dashpay/dash-evo-tool/pull/1176) — ⚠ merge conflict · ✋ changes requested · 📝 draft · areas: fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/dash-evo-tool#1192 feat(identity): claim token rewards from the identity hub](https://github.com/dashpay/dash-evo-tool/pull/1192) — ⚠ merge conflict · 📝 draft · 🐢 targets feat/identity-hub-tokens · areas: fallback
- [dashpay/dash-evo-tool#1195 fix(dashpay): recover contact requests this device could not record](https://github.com/dashpay/dash-evo-tool/pull/1195) — 📝 draft · 🐢 targets fix/dashpay-contact-info-keys · areas: fallback
- [dashpay/dash-evo-tool#1197 fix(send): offer a private-balance Max the network accepts](https://github.com/dashpay/dash-evo-tool/pull/1197) — ⚠ merge conflict · 📝 draft · areas: fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/dash-evo-tool#1202 feat(boot): ask an AI assistant about a startup failure](https://github.com/dashpay/dash-evo-tool/pull/1202) — ✋ changes requested · 🔴 CI failing · areas: fallback · Policy: waiting-author
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/dash-evo-tool#1210 fix(identity): write only what each paid task and refresh changed](https://github.com/dashpay/dash-evo-tool/pull/1210) — 🔴 CI failing · areas: fallback · Policy: waiting-author
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/dash-evo-tool#1212 fix(tokens): close the confirmation when a refresh moves the signing key](https://github.com/dashpay/dash-evo-tool/pull/1212) — ⚠ merge conflict · 📝 draft · 🐢 targets fix/protocol14-signing-key-selection · areas: fallback
- [dashpay/dash-evo-tool#1215 feat(tokens): refresh token balances on request](https://github.com/dashpay/dash-evo-tool/pull/1215) — ⚠ merge conflict · 📝 draft · 🐢 targets refactor/identity-token-model · areas: fallback
- [dashpay/dash-evo-tool#1216 feat(identity): Tokens tab in the identity hub](https://github.com/dashpay/dash-evo-tool/pull/1216) — 📝 draft · 🐢 targets feat/token-balance-refresh · areas: fallback
- [dashpay/dash-evo-tool#1217 feat(identity): token detail page in the identity hub](https://github.com/dashpay/dash-evo-tool/pull/1217) — 📝 draft · 🐢 targets feat/identity-hub-tokens-tab · areas: fallback
- [dashpay/dash-evo-tool#1218 feat(identity): tokens card on the identity Home tab](https://github.com/dashpay/dash-evo-tool/pull/1218) — 📝 draft · 🐢 targets feat/identity-hub-token-details · areas: fallback
- [dashpay/dash-evo-tool#1219 fix(tokens): re-importing a dismissed token restores its tracking](https://github.com/dashpay/dash-evo-tool/pull/1219) — 📝 draft · 🐢 targets feat/identity-hub-tokens-tab · areas: fallback
- [dashpay/dash-evo-tool#1222 fix(dashpay): drop the cached contact QR image with the code](https://github.com/dashpay/dash-evo-tool/pull/1222) — 📝 draft · 🐢 targets fix/protocol14-signing-key-selection · areas: fallback
- [dashpay/dash-evo-tool#1227 docs: add the project profile for AI reviewers](https://github.com/dashpay/dash-evo-tool/pull/1227) — areas: fallback · Policy: ready-to-merge
  - Blocker: All policy requirements are satisfied
  - Blocker: Proceeded without coderabbitai: skipped by @lklimek
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
- [dashpay/platform#5951 chore: bump rust-dashcore to dev f0ddfd45](https://github.com/dashpay/platform/pull/5951) — 🔴 CI failing · 🐢 targets v5.2-dev · areas: dpp, rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk, js-wasm-sdk, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
- [dashpay/platform#5961 fix(platform-wallet): don't pick a limited key to sign a data contract create or update](https://github.com/dashpay/platform/pull/5961) — 🔴 CI failing · 📝 draft · 🐢 targets refactor/document-create-bounds-in-dpp · areas: rs-platform-wallet
- [dashpay/platform#6025 feat(platform-wallet): read the chain-following masternode list with its height](https://github.com/dashpay/platform/pull/6025) — 📝 draft · 🐢 targets feat/mn-key-id-check · areas: rs-platform-wallet
- [dashpay/platform#6026 feat(platform-wallet): fetch the key references of any masternode](https://github.com/dashpay/platform/pull/6026) — 📝 draft · 🐢 targets feat/mn-current-list · areas: rs-platform-wallet
- [dashpay/platform#6027 feat(platform-wallet): return the proved block height with a withdrawal's remaining balance](https://github.com/dashpay/platform/pull/6027) — 📝 draft · 🐢 targets feat/mn-key-reference · areas: rs-platform-wallet, rs-platform-wallet-ffi
- [dashpay/platform#6028 feat(platform-wallet): withdraw masternode credits with a wallet master-key provider or a host signer](https://github.com/dashpay/platform/pull/6028) — 📝 draft · 🐢 targets feat/mn-withdrawal-height · areas: rs-platform-wallet, rs-platform-wallet-ffi
- [dashpay/platform#6030 chore: update rust-dashcore for masternode lookup and lists](https://github.com/dashpay/platform/pull/6030) — 🔴 CI failing · 📝 draft · 🐢 targets feat/mn-withdrawal-key-source · areas: dpp, rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk, js-wasm-sdk, fallback
- [dashpay/platform#6031 feat(platform-wallet): find a wallet's owner or voting key by its hash](https://github.com/dashpay/platform/pull/6031) — 📝 draft · 🐢 targets chore/mn-dashcore-k1-s1 · areas: rs-platform-wallet, rs-platform-wallet-ffi
- [dashpay/platform#6032 refactor(platform-wallet): read the current masternode list from dash-spv](https://github.com/dashpay/platform/pull/6032) — 📝 draft · 🐢 targets feat/mn-provider-key-lookup · areas: rs-platform-wallet, rs-platform-wallet-ffi
- [dashpay/platform#6035 fix(platform-wallet): keep pre-derived lost funding rows ineligible after restart](https://github.com/dashpay/platform/pull/6035) — 📝 draft · 🐢 targets chore/rust-dashcore-dev · areas: rs-platform-wallet
- [dashpay/platform#6036 refactor(platform-wallet): drop always-zero Core address balance](https://github.com/dashpay/platform/pull/6036) — 📝 draft · 🐢 targets fix/funding-pool-lost-row-in-window · areas: rs-platform-wallet-ffi, swift-sdk, kotlin-sdk
- [dashpay/platform#6037 refactor(platform-wallet): one owner for platform-node pool entries](https://github.com/dashpay/platform/pull/6037) — 📝 draft · 🐢 targets refactor/core-address-entry-drop-balance · areas: rs-platform-wallet, wallet-storage
- [dashpay/tenderdash#1578 fix(statesync): refuse surplus light block responses without blocking](https://github.com/dashpay/tenderdash/pull/1578) — 📝 draft · areas: fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/tenderdash#1627 fix(p2p): bound the simple priority queue, honor channel priorities and count drops](https://github.com/dashpay/tenderdash/pull/1627) — 🔴 CI failing · 📝 draft · areas: fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot

<a id="claudius-maginificent-needs-action"></a>
#### Needs action (11)
- [dashpay/dash-evo-tool#1175 docs: add maintainable token tutorial production toolkit](https://github.com/dashpay/dash-evo-tool/pull/1175) — 12 unresolved (12 CodeRabbit) · 0 days stale · areas: fallback · Policy: waiting-author
  - Top thread: "&lt;picture&gt;&lt;img src="https://storage.googleapis.com/coderabbit_public_assets/review-comment-icons-v1/major-16.svg" width="…" — 0 days old
  - Blocker: coderabbitai left review threads unresolved; resolve them
  - Blocker: Proceeded without thepastaclaw: no review within the configured window
- [dashpay/dash-evo-tool#1200 feat(wallet)!: detect databases needing the platform bridge and remove the bridge](https://github.com/dashpay/dash-evo-tool/pull/1200) — 10 unresolved (5 CodeRabbit, 5 bot) · 2 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · areas: fallback · Policy: waiting-author
  - Top thread: "✅ **Resolved** at \`de6f75ee\`; see the replies below." — 2 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: coderabbitai left review threads unresolved; resolve them
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/dash-evo-tool#1214 fix(tokens): identity token model](https://github.com/dashpay/dash-evo-tool/pull/1214) — 9 unresolved (9 bot) · 1 days stale · ✋ changes requested · 🔴 CI failing · areas: fallback · Policy: waiting-author
  - Top thread: "🟡 Suggestion: My Tokens sorts names by characters it no longer displays**" — 1 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/dash-evo-tool#1225 fix(boot): give accurate advice for invalid settings and a denied vault](https://github.com/dashpay/dash-evo-tool/pull/1225) — 6 unresolved (1 CodeRabbit, 5 bot) · 1 days stale · ✋ changes requested · 🔴 CI failing · areas: fallback · Policy: waiting-author
  - Top thread: "&lt;picture&gt;&lt;img src="https://storage.googleapis.com/coderabbit_public_assets/review-comment-icons-v1/amber-16.svg" width="…" — 1 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: coderabbitai left review threads unresolved; resolve them
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/tenderdash#1600 fix(consensus): deliver and process a polka block for a validator left in its round](https://github.com/dashpay/tenderdash/pull/1600) — 2 unresolved (2 bot) · 7 days stale · ✋ changes requested · 🔴 CI failing · areas: fallback · Policy: waiting-author
  - Top thread: "🔴 Blocking: Avoid racing on PartSet completeness in the gossip snapshot**" — 7 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/dash-evo-tool#1117 fix(identity): choose signing keys Platform accepts on protocol 14](https://github.com/dashpay/dash-evo-tool/pull/1117) — 3 unresolved (3 bot) · 1 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · areas: fallback · Policy: waiting-author
  - Top thread: "🟡 Suggestion: The signing selection type can report a usable key without containing one**" — 1 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/dash-evo-tool#1174 feat(skills): mark defects in visual-diff galleries](https://github.com/dashpay/dash-evo-tool/pull/1174) — 3 unresolved (3 bot) · 1 days stale · ✋ changes requested · 🔴 CI failing · areas: fallback · Policy: waiting-author
  - Top thread: "🟡 Suggestion: The skill tells reviewers to omit unchanged regions and banners that show confirmed defects**" — 1 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/dash-evo-tool#1240 feat(send): clearer confirmation and a steady form width](https://github.com/dashpay/dash-evo-tool/pull/1240) — 3 unresolved (3 bot) · 1 days stale · ✋ changes requested · 🔴 CI failing · areas: fallback · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Opening a batch confirmation reloads wallet and identity data for every recipient**" — 1 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/dash-evo-tool#1241 feat(ci): review each weekly build in the GUI before publishing it](https://github.com/dashpay/dash-evo-tool/pull/1241) — 2 unresolved (2 bot) · 0 days stale · ✋ changes requested · 🔴 CI failing · areas: github, fallback · Policy: waiting-bots
  - Top thread: "🔴 Blocking: Cleanup abandons the refund when Insight lists the same payout twice as it confirms**" — 0 days old
  - Blocker: coderabbitai has not reported for the current head
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/dash-evo-tool#1206 fix(boot): show startup panics and window failures instead of closing](https://github.com/dashpay/dash-evo-tool/pull/1206) — 1 unresolved (1 bot) · 2 days stale · areas: fallback · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Startup tests pass even if the native alert is shown at the wrong time**" — 2 days old
  - Blocker: thepastaclaw left review threads unresolved; resolve them
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/dash-evo-tool#1202 feat(boot): ask an AI assistant about a startup failure](https://github.com/dashpay/dash-evo-tool/pull/1202) — ✋ changes requested · 🔴 CI failing · areas: fallback · Policy: waiting-author
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return

<a id="claudius-maginificent-ready-for-human"></a>
#### Ready for human (2)
- [dashpay/dash-evo-tool#1227 docs: add the project profile for AI reviewers](https://github.com/dashpay/dash-evo-tool/pull/1227) — areas: fallback · Policy: ready-to-merge
  - Blocker: All policy requirements are satisfied
  - Blocker: Proceeded without coderabbitai: skipped by @lklimek
- [dashpay/platform#5951 chore: bump rust-dashcore to dev f0ddfd45](https://github.com/dashpay/platform/pull/5951) — 🔴 CI failing · 🐢 targets v5.2-dev · areas: dpp, rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk, js-wasm-sdk, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required

<a id="claudius-maginificent-unresolved-comments"></a>
#### Unresolved Comments (10)
- [dashpay/dash-evo-tool#1175 docs: add maintainable token tutorial production toolkit](https://github.com/dashpay/dash-evo-tool/pull/1175) — 12 unresolved (12 CodeRabbit) · 0 days stale · areas: fallback · Policy: waiting-author
  - Top thread: "&lt;picture&gt;&lt;img src="https://storage.googleapis.com/coderabbit_public_assets/review-comment-icons-v1/major-16.svg" width="…" — 0 days old
  - Blocker: coderabbitai left review threads unresolved; resolve them
  - Blocker: Proceeded without thepastaclaw: no review within the configured window
- [dashpay/dash-evo-tool#1200 feat(wallet)!: detect databases needing the platform bridge and remove the bridge](https://github.com/dashpay/dash-evo-tool/pull/1200) — 10 unresolved (5 CodeRabbit, 5 bot) · 2 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · areas: fallback · Policy: waiting-author
  - Top thread: "✅ **Resolved** at \`de6f75ee\`; see the replies below." — 2 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: coderabbitai left review threads unresolved; resolve them
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/dash-evo-tool#1214 fix(tokens): identity token model](https://github.com/dashpay/dash-evo-tool/pull/1214) — 9 unresolved (9 bot) · 1 days stale · ✋ changes requested · 🔴 CI failing · areas: fallback · Policy: waiting-author
  - Top thread: "🟡 Suggestion: My Tokens sorts names by characters it no longer displays**" — 1 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/dash-evo-tool#1225 fix(boot): give accurate advice for invalid settings and a denied vault](https://github.com/dashpay/dash-evo-tool/pull/1225) — 6 unresolved (1 CodeRabbit, 5 bot) · 1 days stale · ✋ changes requested · 🔴 CI failing · areas: fallback · Policy: waiting-author
  - Top thread: "&lt;picture&gt;&lt;img src="https://storage.googleapis.com/coderabbit_public_assets/review-comment-icons-v1/amber-16.svg" width="…" — 1 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: coderabbitai left review threads unresolved; resolve them
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/tenderdash#1600 fix(consensus): deliver and process a polka block for a validator left in its round](https://github.com/dashpay/tenderdash/pull/1600) — 2 unresolved (2 bot) · 7 days stale · ✋ changes requested · 🔴 CI failing · areas: fallback · Policy: waiting-author
  - Top thread: "🔴 Blocking: Avoid racing on PartSet completeness in the gossip snapshot**" — 7 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/dash-evo-tool#1117 fix(identity): choose signing keys Platform accepts on protocol 14](https://github.com/dashpay/dash-evo-tool/pull/1117) — 3 unresolved (3 bot) · 1 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · areas: fallback · Policy: waiting-author
  - Top thread: "🟡 Suggestion: The signing selection type can report a usable key without containing one**" — 1 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/dash-evo-tool#1174 feat(skills): mark defects in visual-diff galleries](https://github.com/dashpay/dash-evo-tool/pull/1174) — 3 unresolved (3 bot) · 1 days stale · ✋ changes requested · 🔴 CI failing · areas: fallback · Policy: waiting-author
  - Top thread: "🟡 Suggestion: The skill tells reviewers to omit unchanged regions and banners that show confirmed defects**" — 1 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/dash-evo-tool#1240 feat(send): clearer confirmation and a steady form width](https://github.com/dashpay/dash-evo-tool/pull/1240) — 3 unresolved (3 bot) · 1 days stale · ✋ changes requested · 🔴 CI failing · areas: fallback · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Opening a batch confirmation reloads wallet and identity data for every recipient**" — 1 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/dash-evo-tool#1241 feat(ci): review each weekly build in the GUI before publishing it](https://github.com/dashpay/dash-evo-tool/pull/1241) — 2 unresolved (2 bot) · 0 days stale · ✋ changes requested · 🔴 CI failing · areas: github, fallback · Policy: waiting-bots
  - Top thread: "🔴 Blocking: Cleanup abandons the refund when Insight lists the same payout twice as it confirms**" — 0 days old
  - Blocker: coderabbitai has not reported for the current head
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/dash-evo-tool#1206 fix(boot): show startup panics and window failures instead of closing](https://github.com/dashpay/dash-evo-tool/pull/1206) — 1 unresolved (1 bot) · 2 days stale · areas: fallback · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Startup tests pass even if the native alert is shown at the wrong time**" — 2 days old
  - Blocker: thepastaclaw left review threads unresolved; resolve them
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return

<a id="claudius-maginificent-changes-requested"></a>
#### Changes Requested (1)
- [dashpay/dash-evo-tool#1202 feat(boot): ask an AI assistant about a startup failure](https://github.com/dashpay/dash-evo-tool/pull/1202) — ✋ changes requested · 🔴 CI failing · areas: fallback · Policy: waiting-author
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return

<a id="claudius-maginificent-ci-failing"></a>
#### CI Failing (1)
- [dashpay/dash-evo-tool#1210 fix(identity): write only what each paid task and refresh changed](https://github.com/dashpay/dash-evo-tool/pull/1210) — 🔴 CI failing · areas: fallback · Policy: waiting-author
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return

<a id="claudius-maginificent-deferred"></a>
#### Deferred (3)
- [dashpay/platform#3549 test(platform-wallet): e2e framework + full test suite — triage pins, Found-*/PA-* guards, fail-closed persist, Stage-2 merge](https://github.com/dashpay/platform/pull/3549) — 📝 draft · ⏸ deferred · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk, kotlin-sdk, github, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5384 fix(platform-wallet)!: persist chain truncations so a stored wallet follows a reorg](https://github.com/dashpay/platform/pull/5384) — ⏸ deferred · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk, kotlin-sdk · Policy: waiting-author
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5417 fix(platform-wallet): own per-wallet transaction accounting in Rust for SwiftData](https://github.com/dashpay/platform/pull/5417) — 📝 draft · ⏸ deferred · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk

<a id="claudius-maginificent-draft"></a>
#### Draft (4)
- [dashpay/dash-evo-tool#1176 feat(tokens): add guided token creation with complete review](https://github.com/dashpay/dash-evo-tool/pull/1176) — ⚠ merge conflict · ✋ changes requested · 📝 draft · areas: fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/dash-evo-tool#1197 fix(send): offer a private-balance Max the network accepts](https://github.com/dashpay/dash-evo-tool/pull/1197) — ⚠ merge conflict · 📝 draft · areas: fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/tenderdash#1578 fix(statesync): refuse surplus light block responses without blocking](https://github.com/dashpay/tenderdash/pull/1578) — 📝 draft · areas: fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/tenderdash#1627 fix(p2p): bound the simple priority queue, honor channel priorities and count drops](https://github.com/dashpay/tenderdash/pull/1627) — 🔴 CI failing · 📝 draft · areas: fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot

<a id="claudius-maginificent-stale"></a>
#### Stale (45)
- [dashpay/platform#5645 fix(platform-wallet)!: check identity creation keys and funding in the Rust core](https://github.com/dashpay/platform/pull/5645) — 9 unresolved (1 CodeRabbit, 8 bot) · 21 days stale · ✋ changes requested · 🔴 CI failing · 🐢 targets v5.2-dev · areas: dpp, rs-platform-wallet, rs-platform-wallet-ffi, rust-sdk · Policy: waiting-author
  - Top thread: "&lt;picture&gt;&lt;img src="https://storage.googleapis.com/coderabbit_public_assets/review-comment-icons-v1/amber-16.svg" width="…" — 21 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: coderabbitai left review threads unresolved; resolve them
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/dash-evo-tool#1177 feat(ui): align app-wide styles with the token proposal](https://github.com/dashpay/dash-evo-tool/pull/1177) — 13 unresolved (13 bot) · 6 days stale · ✋ changes requested · 📝 draft · 🐢 targets feat/token-creator-workspace · areas: fallback
  - Top thread: "✅ **Resolved** at \`0993a6af\`; see the replies below." — 6 days old
- [dashpay/dash-evo-tool#1178 fix(tokens): keep JSON export responsive when the file chooser stalls](https://github.com/dashpay/dash-evo-tool/pull/1178) — 15 unresolved (7 CodeRabbit, 8 bot) · 4 days stale · ✋ changes requested · 📝 draft · 🐢 targets feat/app-design-system · areas: fallback
  - Top thread: "🟡 Suggestion: Exercise a publication failure after staging succeeds**" — 4 days old
- [dashpay/platform#5565 feat(platform-wallet)!: detect inconsistent shielded commitment trees and add explicit rebuild](https://github.com/dashpay/platform/pull/5565) — 7 unresolved (7 bot) · 25 days stale · ✋ changes requested · 🔴 CI failing · 🐢 targets v5.2-dev · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk · Policy: waiting-author
  - Top thread: "⏸️ **Needs a tracking issue** (not yet deferred) at \`3b17ab95\`; see the replies below." — 25 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5588 feat(platform-wallet)!: expose protocol-14-aware signing-key selection](https://github.com/dashpay/platform/pull/5588) — 5 unresolved (5 bot) · 25 days stale · 🔴 CI failing · 🐢 targets v5.2-dev · areas: dpp, rs-platform-wallet, rs-platform-wallet-ffi, rust-sdk · Policy: waiting-author
  - Top thread: "⏭️ **Deferred** (maintainer decision; not blocking) at \`7479fdcb\`; see the replies below." — 25 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/dash-evo-tool#1108 feat(documents): recognise indexOnly document types and delete their entries by values](https://github.com/dashpay/dash-evo-tool/pull/1108) — 3 unresolved (3 bot) · 32 days stale · ✋ changes requested · 📝 draft · 🐢 targets feat/document-optional-token-cost · areas: fallback
  - Top thread: "🟡 Suggestion: Make the non-deletable error actionable**" — 32 days old
- [dashpay/platform#5644 fix!: follow DIP-11 key rules when disabling keys and reading contact requests](https://github.com/dashpay/platform/pull/5644) — 8 unresolved (8 bot) · 2 days stale · ✋ changes requested · 🔴 CI failing · 🐢 targets v5.2-dev · areas: dpp, rs-platform-wallet, rs-platform-wallet-ffi, rust-sdk, rust-sdk-ffi, js-wasm-sdk, fallback · Policy: waiting-author
  - Top thread: "✅ **Resolved** at \`ed16501f\`; see the replies below." — 2 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5546 fix(platform-wallet): support HASH160 DashPay contact signing keys](https://github.com/dashpay/platform/pull/5546) — 2 unresolved (2 bot) · 25 days stale · 📝 draft · 🐢 targets feat/platform-wallet-public-signing-key-selection · areas: rs-drive-abci, dpp, rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk, fallback
  - Top thread: "🟡 Suggestion: Do not expose an unchecked signer-capability override in the public constructor**" — 25 days old
- [dashpay/dash-evo-tool#1179 fix(tokens): retain the creator when opening a registered contract](https://github.com/dashpay/dash-evo-tool/pull/1179) — 4 unresolved (4 bot) · 4 days stale · ✋ changes requested · 📝 draft · 🐢 targets fix/token-json-export · areas: fallback
  - Top thread: "🟡 Suggestion: Assert that the preview displays fetched contract details**" — 4 days old
- [dashpay/platform#5981 refactor(platform-wallet-storage): drop the unreachable rehydration gap-refill guard](https://github.com/dashpay/platform/pull/5981) — 2 unresolved (2 bot) · 6 days stale · ✋ changes requested · 📝 draft · 🐢 targets refactor/platform-node-pool-single-owner · areas: wallet-storage
  - Top thread: "🔴 Blocking: Preserve script-based ownership after a failed refill**" — 6 days old
- [dashpay/dash-evo-tool#1124 feat(documents): let the user skip an optional token cost](https://github.com/dashpay/dash-evo-tool/pull/1124) — 1 unresolved (1 bot) · 29 days stale · ⚠ merge conflict · ✋ changes requested · 📝 draft · 🐢 targets fix/protocol14-signing-key-selection · areas: fallback
  - Top thread: "🔴 Blocking: Show the credit-fee outcome when an optional token cost is disabled**" — 29 days old
- [dashpay/platform#5707 fix(sdk): recover the nonce cache when unconfirmed nonces leave the accepted window](https://github.com/dashpay/platform/pull/5707) — 1 unresolved (1 bot) · 21 days stale · ✋ changes requested · 🔴 CI failing · 📝 draft · 🐢 targets v5.2-dev · areas: rust-sdk · Policy: draft
  - Top thread: "🔴 Blocking: Do not reissue nonces belonging to outstanding operations**" — 21 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5979 fix(sdk): keep PublicKey unchanged when the inner setter rejects a key](https://github.com/dashpay/platform/pull/5979) — 1 unresolved (1 bot) · 6 days stale · 🔴 CI failing · 📝 draft · 🐢 targets chore/crypto-core-consumers · areas: fallback
  - Top thread: "🟡 Suggestion: Add successful cross-format setter coverage**" — 6 days old
- [dashpay/platform#5647 feat: expose the Rust identity key layout to FFI, Swift and Kotlin hosts](https://github.com/dashpay/platform/pull/5647) — 1 unresolved (1 bot) · 2 days stale · ✋ changes requested · 🔴 CI failing · 📝 draft · 🐢 targets fix/platform-wallet-dip11-identity-key-layout · areas: rs-platform-wallet-ffi, swift-sdk, kotlin-sdk
  - Top thread: "🟡 Suggestion: Kotlin reports a failed protocol-version lookup as non-retryable**" — 2 days old
- [dashpay/platform#5661 feat: decide which identity keys may be disabled in the wallet, not in the mobile apps](https://github.com/dashpay/platform/pull/5661) — 1 unresolved (1 bot) · 0 days stale · 🔴 CI failing · 📝 draft · 🐢 targets fix/platform-wallet-dip11-key-rules · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk
  - Top thread: "🟡 Suggestion: The last-TRANSFER test passes even if submission checks the pre-refresh key set**" — 0 days old
- [dashpay/dash-evo-tool#1120 feat(identity): add keys with spending limits and raise key limits](https://github.com/dashpay/dash-evo-tool/pull/1120) — ⚠ merge conflict · 📝 draft · 🐢 targets fix/protocol14-signing-key-selection · areas: fallback
- [dashpay/dash-evo-tool#1121 feat(contracts): view and claim contract fee pots](https://github.com/dashpay/dash-evo-tool/pull/1121) — 📝 draft · 🐢 targets fix/protocol14-signing-key-selection · areas: fallback
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
- [dashpay/platform#5951 chore: bump rust-dashcore to dev f0ddfd45](https://github.com/dashpay/platform/pull/5951) — 🔴 CI failing · 🐢 targets v5.2-dev · areas: dpp, rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk, js-wasm-sdk, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
- [dashpay/platform#5961 fix(platform-wallet): don't pick a limited key to sign a data contract create or update](https://github.com/dashpay/platform/pull/5961) — 🔴 CI failing · 📝 draft · 🐢 targets refactor/document-create-bounds-in-dpp · areas: rs-platform-wallet
- [dashpay/platform#6025 feat(platform-wallet): read the chain-following masternode list with its height](https://github.com/dashpay/platform/pull/6025) — 📝 draft · 🐢 targets feat/mn-key-id-check · areas: rs-platform-wallet
- [dashpay/platform#6026 feat(platform-wallet): fetch the key references of any masternode](https://github.com/dashpay/platform/pull/6026) — 📝 draft · 🐢 targets feat/mn-current-list · areas: rs-platform-wallet
- [dashpay/platform#6027 feat(platform-wallet): return the proved block height with a withdrawal's remaining balance](https://github.com/dashpay/platform/pull/6027) — 📝 draft · 🐢 targets feat/mn-key-reference · areas: rs-platform-wallet, rs-platform-wallet-ffi
- [dashpay/platform#6028 feat(platform-wallet): withdraw masternode credits with a wallet master-key provider or a host signer](https://github.com/dashpay/platform/pull/6028) — 📝 draft · 🐢 targets feat/mn-withdrawal-height · areas: rs-platform-wallet, rs-platform-wallet-ffi
- [dashpay/platform#6030 chore: update rust-dashcore for masternode lookup and lists](https://github.com/dashpay/platform/pull/6030) — 🔴 CI failing · 📝 draft · 🐢 targets feat/mn-withdrawal-key-source · areas: dpp, rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk, js-wasm-sdk, fallback
- [dashpay/platform#6031 feat(platform-wallet): find a wallet's owner or voting key by its hash](https://github.com/dashpay/platform/pull/6031) — 📝 draft · 🐢 targets chore/mn-dashcore-k1-s1 · areas: rs-platform-wallet, rs-platform-wallet-ffi
- [dashpay/platform#6032 refactor(platform-wallet): read the current masternode list from dash-spv](https://github.com/dashpay/platform/pull/6032) — 📝 draft · 🐢 targets feat/mn-provider-key-lookup · areas: rs-platform-wallet, rs-platform-wallet-ffi
- [dashpay/platform#6035 fix(platform-wallet): keep pre-derived lost funding rows ineligible after restart](https://github.com/dashpay/platform/pull/6035) — 📝 draft · 🐢 targets chore/rust-dashcore-dev · areas: rs-platform-wallet
- [dashpay/platform#6036 refactor(platform-wallet): drop always-zero Core address balance](https://github.com/dashpay/platform/pull/6036) — 📝 draft · 🐢 targets fix/funding-pool-lost-row-in-window · areas: rs-platform-wallet-ffi, swift-sdk, kotlin-sdk
- [dashpay/platform#6037 refactor(platform-wallet): one owner for platform-node pool entries](https://github.com/dashpay/platform/pull/6037) — 📝 draft · 🐢 targets refactor/core-address-entry-drop-balance · areas: rs-platform-wallet, wallet-storage

<a id="claudius-maginificent-clean"></a>
#### Clean (1)
- [dashpay/dash-evo-tool#1227 docs: add the project profile for AI reviewers](https://github.com/dashpay/dash-evo-tool/pull/1227) — areas: fallback · Policy: ready-to-merge
  - Blocker: All policy requirements are satisfied
  - Blocker: Proceeded without coderabbitai: skipped by @lklimek

<a id="quantumexplorer"></a>
### @QuantumExplorer
<a id="quantumexplorer-open"></a>
#### Open (22)
- [dashpay/platform#5750 feat(platform)!: random token transfer and mint amounts](https://github.com/dashpay/platform/pull/5750) — 13 unresolved (13 bot) · 21 days stale · 🐢 targets claude/platform-tenderdash-randomness-149e4e · areas: rs-drive, dpp, rs-platform-wallet, rust-sdk, rust-sdk-ffi, fallback
  - Top thread: "✅ **Resolved** at \`f8fcc8fd\`; see the replies below." — 21 days old
- [dashpay/platform#5749 feat(platform)!: sys.random.uniform document properties](https://github.com/dashpay/platform/pull/5749) — 9 unresolved (9 bot) · 21 days stale · 🔴 CI failing · 🐢 targets v5.2-dev · areas: rs-drive, rs-drive-abci, dpp, fallback · Policy: waiting-author
  - Top thread: "✅ **Resolved** at \`0ead2b47\`; see the replies below." — 21 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5753 feat(drive-abci)!: apply block randomness through the ApplyBlockRandomness ABCI method](https://github.com/dashpay/platform/pull/5753) — 8 unresolved (8 bot) · 20 days stale · ⚠ merge conflict · 🔴 CI failing · 🐢 targets claude/random-token-amounts · areas: rs-drive, rs-drive-abci, dpp, fallback
  - Top thread: "🟡 Suggestion: Exercise the protocol upgrade with signed commits**" — 20 days old
- [dashpay/platform#5529 feat(platform)!: token marketplace listings (PV15)](https://github.com/dashpay/platform/pull/5529) — 2 unresolved (2 bot) · 30 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · 🐢 targets v5.2-dev · areas: rs-drive, rs-drive-abci, dpp, rs-platform-wallet, swift-sdk, kotlin-sdk, fallback · Policy: waiting-author
  - Top thread: "🔴 Blocking: Version the consensus order-ID derivation through platform-version tables**" — 30 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#6038 feat(platform): prove the state before and after the block of a state transition](https://github.com/dashpay/platform/pull/6038) — 4 unresolved (4 bot) · 0 days stale · ✋ changes requested · 🔴 CI failing · 🐢 targets v5.2-dev · areas: rs-drive, rs-drive-abci, rs-platform-wallet, rs-platform-wallet-ffi, rust-sdk, js-wasm-sdk, rust-dapi, fallback · Policy: waiting-self-review
  - Top thread: "🟡 Suggestion: Pending waits keep querying Tenderdash when its transaction index is disabled**" — 0 days old
  - Blocker: Author must post /self-reviewed c1f838c4ecb042e728fa9031d208e10b9d0afc1c
- [dashpay/platform#5530 feat(platform)!: token marketplace offers (PV15)](https://github.com/dashpay/platform/pull/5530) — 1 unresolved (1 bot) · 25 days stale · ✋ changes requested · 🔴 CI failing · 🐢 targets claude/token-market-listings · areas: rs-drive, rs-drive-abci, dpp, rs-platform-wallet, fallback
  - Top thread: "🔴 Blocking: Bound bid escrow by a buyer-signed credit limit**" — 25 days old
- [dashpay/tenderdash#1582 feat(abci)!: add the ApplyBlockRandomness method and commit beacon fields](https://github.com/dashpay/tenderdash/pull/1582) — 1 unresolved (1 bot) · 18 days stale · areas: fallback · Policy: waiting-author
  - Top thread: "✅ **Resolved** at \`cd6b4ec8\`; see the replies below." — 18 days old
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/tenderdash#1597 fix(p2p): trim the simple priority queue below its limit](https://github.com/dashpay/tenderdash/pull/1597) — 1 unresolved (1 bot) · 16 days stale · ⚠ merge conflict · areas: fallback · Policy: waiting-self-review
  - Top thread: "🟡 Suggestion: Count the larger overflow batches**" — 16 days old
  - Blocker: Author must post /self-reviewed fb3e02bfc772facdefade06c2eeca6743e23e329
- [dashpay/platform#6222 fix(kotlin-sdk): watch an identity's tokens so the Tokens screen shows balances](https://github.com/dashpay/platform/pull/6222) — 2 unresolved (2 bot) · 0 days stale · ✋ changes requested · 🔴 CI failing · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk, sdk-parity · Policy: waiting-bots
  - Top thread: "🟡 Suggestion: The one-budget shutdown test can fail on a busy CI runner**" — 0 days old
  - Blocker: thepastaclaw has not reported for the current head
- [dashpay/grovedb#1066 docs(audit): record #1065 as a root-path delete gap Platform never reaches](https://github.com/dashpay/grovedb/pull/1066) — areas: fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed c9ed673ded05ecde64c0a4dca472e0bbc90ebe9c
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/platform#4730 feat(platform)!: delta-based data contract update transition for protocol version 15](https://github.com/dashpay/platform/pull/4730) — ⚠ merge conflict · 🔴 CI failing · 📝 draft · 🐢 targets v5.2-dev · areas: rs-drive, rs-drive-abci, dpp, rs-platform-wallet, rust-sdk, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#4776 feat(platform-wallet)!: let contract updates clear the description through the FFI](https://github.com/dashpay/platform/pull/4776) — ⚠ merge conflict · 🔴 CI failing · 🐢 targets feat/delta-contract-update-pv15 · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk
- [dashpay/platform#4986 fix(drive-abci)!: pay every reward share of a masternode and credit each identity once in the epoch payout (PV15)](https://github.com/dashpay/platform/pull/4986) — ⚠ merge conflict · 📝 draft · 🐢 targets v5.2-dev · areas: rs-drive-abci, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5531 feat(platform)!: record token marketplace events in token history (PV15)](https://github.com/dashpay/platform/pull/5531) — 🔴 CI failing · 🐢 targets claude/token-market-offers · areas: rs-drive, rs-drive-abci, dpp, system-contracts, fallback
- [dashpay/platform#5532 feat(platform)!: destroy a frozen identity's escrowed listings with its funds (PV15)](https://github.com/dashpay/platform/pull/5532) — 🔴 CI failing · 🐢 targets claude/token-market-history · areas: rs-drive, rs-drive-abci, fallback
- [dashpay/platform#5736 feat(platform)!: add a RedPallas signer key type with its own purpose (PV15)](https://github.com/dashpay/platform/pull/5736) — ⚠ merge conflict · 📝 draft · 🐢 targets v5.2-dev · areas: rs-drive, rs-drive-abci, dpp, rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, rust-sdk-ffi, kotlin-sdk, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5751 feat(sdk): random token amounts in wasm-sdk and js-evo-sdk](https://github.com/dashpay/platform/pull/5751) — 🐢 targets claude/random-token-amounts · areas: js-wasm-sdk
- [dashpay/platform#6002 fix(dashmate): run an added local node on its network's Core image](https://github.com/dashpay/platform/pull/6002) — ✋ changes requested · areas: dashmate · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 9cc7778487200a5f379f0cb47cfff854e1a520dd
- [dashpay/platform#6245 fix(swift-example-app): let the perpetual distribution's recipient claim it](https://github.com/dashpay/platform/pull/6245) — areas: swift-sdk · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head
- [dashpay/platform#6246 fix(platform-wallet): leave a contact's watch-only account out of per-account Core balances](https://github.com/dashpay/platform/pull/6246) — areas: rs-platform-wallet
- [dashpay/tenderdash#1583 feat(consensus)!: recover a beacon from prevotes and sign precommits over the randomness-applied app hash](https://github.com/dashpay/tenderdash/pull/1583) — ⚠ merge conflict · 📝 draft · areas: fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/tenderdash#1584 fix(statesync): restore LastAppHash of the snapshot height, not the next one](https://github.com/dashpay/tenderdash/pull/1584) — areas: fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 23c1443e4077c7c7dfedf00f9446c762e669660f

<a id="quantumexplorer-needs-action"></a>
#### Needs action (4)
- [dashpay/tenderdash#1582 feat(abci)!: add the ApplyBlockRandomness method and commit beacon fields](https://github.com/dashpay/tenderdash/pull/1582) — 1 unresolved (1 bot) · 18 days stale · areas: fallback · Policy: waiting-author
  - Top thread: "✅ **Resolved** at \`cd6b4ec8\`; see the replies below." — 18 days old
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/tenderdash#1597 fix(p2p): trim the simple priority queue below its limit](https://github.com/dashpay/tenderdash/pull/1597) — 1 unresolved (1 bot) · 16 days stale · ⚠ merge conflict · areas: fallback · Policy: waiting-self-review
  - Top thread: "🟡 Suggestion: Count the larger overflow batches**" — 16 days old
  - Blocker: Author must post /self-reviewed fb3e02bfc772facdefade06c2eeca6743e23e329
- [dashpay/platform#6222 fix(kotlin-sdk): watch an identity's tokens so the Tokens screen shows balances](https://github.com/dashpay/platform/pull/6222) — 2 unresolved (2 bot) · 0 days stale · ✋ changes requested · 🔴 CI failing · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk, sdk-parity · Policy: waiting-bots
  - Top thread: "🟡 Suggestion: The one-budget shutdown test can fail on a busy CI runner**" — 0 days old
  - Blocker: thepastaclaw has not reported for the current head
- [dashpay/platform#6002 fix(dashmate): run an added local node on its network's Core image](https://github.com/dashpay/platform/pull/6002) — ✋ changes requested · areas: dashmate · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 9cc7778487200a5f379f0cb47cfff854e1a520dd

<a id="quantumexplorer-unresolved-comments"></a>
#### Unresolved Comments (3)
- [dashpay/tenderdash#1582 feat(abci)!: add the ApplyBlockRandomness method and commit beacon fields](https://github.com/dashpay/tenderdash/pull/1582) — 1 unresolved (1 bot) · 18 days stale · areas: fallback · Policy: waiting-author
  - Top thread: "✅ **Resolved** at \`cd6b4ec8\`; see the replies below." — 18 days old
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/tenderdash#1597 fix(p2p): trim the simple priority queue below its limit](https://github.com/dashpay/tenderdash/pull/1597) — 1 unresolved (1 bot) · 16 days stale · ⚠ merge conflict · areas: fallback · Policy: waiting-self-review
  - Top thread: "🟡 Suggestion: Count the larger overflow batches**" — 16 days old
  - Blocker: Author must post /self-reviewed fb3e02bfc772facdefade06c2eeca6743e23e329
- [dashpay/platform#6222 fix(kotlin-sdk): watch an identity's tokens so the Tokens screen shows balances](https://github.com/dashpay/platform/pull/6222) — 2 unresolved (2 bot) · 0 days stale · ✋ changes requested · 🔴 CI failing · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk, sdk-parity · Policy: waiting-bots
  - Top thread: "🟡 Suggestion: The one-budget shutdown test can fail on a busy CI runner**" — 0 days old
  - Blocker: thepastaclaw has not reported for the current head

<a id="quantumexplorer-changes-requested"></a>
#### Changes Requested (1)
- [dashpay/platform#6002 fix(dashmate): run an added local node on its network's Core image](https://github.com/dashpay/platform/pull/6002) — ✋ changes requested · areas: dashmate · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 9cc7778487200a5f379f0cb47cfff854e1a520dd

<a id="quantumexplorer-draft"></a>
#### Draft (1)
- [dashpay/tenderdash#1583 feat(consensus)!: recover a beacon from prevotes and sign precommits over the randomness-applied app hash](https://github.com/dashpay/tenderdash/pull/1583) — ⚠ merge conflict · 📝 draft · areas: fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot

<a id="quantumexplorer-stale"></a>
#### Stale (13)
- [dashpay/platform#5750 feat(platform)!: random token transfer and mint amounts](https://github.com/dashpay/platform/pull/5750) — 13 unresolved (13 bot) · 21 days stale · 🐢 targets claude/platform-tenderdash-randomness-149e4e · areas: rs-drive, dpp, rs-platform-wallet, rust-sdk, rust-sdk-ffi, fallback
  - Top thread: "✅ **Resolved** at \`f8fcc8fd\`; see the replies below." — 21 days old
- [dashpay/platform#5749 feat(platform)!: sys.random.uniform document properties](https://github.com/dashpay/platform/pull/5749) — 9 unresolved (9 bot) · 21 days stale · 🔴 CI failing · 🐢 targets v5.2-dev · areas: rs-drive, rs-drive-abci, dpp, fallback · Policy: waiting-author
  - Top thread: "✅ **Resolved** at \`0ead2b47\`; see the replies below." — 21 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5753 feat(drive-abci)!: apply block randomness through the ApplyBlockRandomness ABCI method](https://github.com/dashpay/platform/pull/5753) — 8 unresolved (8 bot) · 20 days stale · ⚠ merge conflict · 🔴 CI failing · 🐢 targets claude/random-token-amounts · areas: rs-drive, rs-drive-abci, dpp, fallback
  - Top thread: "🟡 Suggestion: Exercise the protocol upgrade with signed commits**" — 20 days old
- [dashpay/platform#5529 feat(platform)!: token marketplace listings (PV15)](https://github.com/dashpay/platform/pull/5529) — 2 unresolved (2 bot) · 30 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · 🐢 targets v5.2-dev · areas: rs-drive, rs-drive-abci, dpp, rs-platform-wallet, swift-sdk, kotlin-sdk, fallback · Policy: waiting-author
  - Top thread: "🔴 Blocking: Version the consensus order-ID derivation through platform-version tables**" — 30 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#6038 feat(platform): prove the state before and after the block of a state transition](https://github.com/dashpay/platform/pull/6038) — 4 unresolved (4 bot) · 0 days stale · ✋ changes requested · 🔴 CI failing · 🐢 targets v5.2-dev · areas: rs-drive, rs-drive-abci, rs-platform-wallet, rs-platform-wallet-ffi, rust-sdk, js-wasm-sdk, rust-dapi, fallback · Policy: waiting-self-review
  - Top thread: "🟡 Suggestion: Pending waits keep querying Tenderdash when its transaction index is disabled**" — 0 days old
  - Blocker: Author must post /self-reviewed c1f838c4ecb042e728fa9031d208e10b9d0afc1c
- [dashpay/platform#5530 feat(platform)!: token marketplace offers (PV15)](https://github.com/dashpay/platform/pull/5530) — 1 unresolved (1 bot) · 25 days stale · ✋ changes requested · 🔴 CI failing · 🐢 targets claude/token-market-listings · areas: rs-drive, rs-drive-abci, dpp, rs-platform-wallet, fallback
  - Top thread: "🔴 Blocking: Bound bid escrow by a buyer-signed credit limit**" — 25 days old
- [dashpay/platform#4730 feat(platform)!: delta-based data contract update transition for protocol version 15](https://github.com/dashpay/platform/pull/4730) — ⚠ merge conflict · 🔴 CI failing · 📝 draft · 🐢 targets v5.2-dev · areas: rs-drive, rs-drive-abci, dpp, rs-platform-wallet, rust-sdk, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#4776 feat(platform-wallet)!: let contract updates clear the description through the FFI](https://github.com/dashpay/platform/pull/4776) — ⚠ merge conflict · 🔴 CI failing · 🐢 targets feat/delta-contract-update-pv15 · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk
- [dashpay/platform#4986 fix(drive-abci)!: pay every reward share of a masternode and credit each identity once in the epoch payout (PV15)](https://github.com/dashpay/platform/pull/4986) — ⚠ merge conflict · 📝 draft · 🐢 targets v5.2-dev · areas: rs-drive-abci, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5531 feat(platform)!: record token marketplace events in token history (PV15)](https://github.com/dashpay/platform/pull/5531) — 🔴 CI failing · 🐢 targets claude/token-market-offers · areas: rs-drive, rs-drive-abci, dpp, system-contracts, fallback
- [dashpay/platform#5532 feat(platform)!: destroy a frozen identity's escrowed listings with its funds (PV15)](https://github.com/dashpay/platform/pull/5532) — 🔴 CI failing · 🐢 targets claude/token-market-history · areas: rs-drive, rs-drive-abci, fallback
- [dashpay/platform#5736 feat(platform)!: add a RedPallas signer key type with its own purpose (PV15)](https://github.com/dashpay/platform/pull/5736) — ⚠ merge conflict · 📝 draft · 🐢 targets v5.2-dev · areas: rs-drive, rs-drive-abci, dpp, rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, rust-sdk-ffi, kotlin-sdk, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5751 feat(sdk): random token amounts in wasm-sdk and js-evo-sdk](https://github.com/dashpay/platform/pull/5751) — 🐢 targets claude/random-token-amounts · areas: js-wasm-sdk

<a id="quantumexplorer-clean"></a>
#### Clean (4)
- [dashpay/grovedb#1066 docs(audit): record #1065 as a root-path delete gap Platform never reaches](https://github.com/dashpay/grovedb/pull/1066) — areas: fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed c9ed673ded05ecde64c0a4dca472e0bbc90ebe9c
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/platform#6245 fix(swift-example-app): let the perpetual distribution's recipient claim it](https://github.com/dashpay/platform/pull/6245) — areas: swift-sdk · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head
- [dashpay/platform#6246 fix(platform-wallet): leave a contact's watch-only account out of per-account Core balances](https://github.com/dashpay/platform/pull/6246) — areas: rs-platform-wallet
- [dashpay/tenderdash#1584 fix(statesync): restore LastAppHash of the snapshot height, not the next one](https://github.com/dashpay/tenderdash/pull/1584) — areas: fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 23c1443e4077c7c7dfedf00f9446c762e669660f

<a id="quantumexplorer-ready-for-review"></a>
#### Ready for Review (4)
- [dashpay/rust-dashcore#1328 refactor(key-wallet)!: validate child indexes at construction with \`ChildIndex\`](https://github.com/dashpay/rust-dashcore/pull/1328) — by @ZocoLini · areas: dash-spv, key-wallet, key-wallet-manager · Policy: waiting-bots
  - Blocker: coderabbitai has not reported for the current head
- [dashpay/rust-dashcore#1426 fix(dash-spv): tell an empty filter header store from genesis](https://github.com/dashpay/rust-dashcore/pull/1426) — by @ZocoLini · areas: dash-spv · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed d31ec981450f9216f70fd0a3877e22563060db0c
- [dashpay/rust-dashcore#1490 ci: test only the crates a pull request changes](https://github.com/dashpay/rust-dashcore/pull/1490) — by @ZocoLini · areas: github, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 57c19ebefff21e528e338b7cc11d43d483fc8012
- [dashpay/rust-dashcore#1502 refactor(dash-spv)!: keep only storage failures in \`StorageError\`](https://github.com/dashpay/rust-dashcore/pull/1502) — by @ZocoLini · areas: dash-spv · Policy: waiting-bots
  - Blocker: coderabbitai has not reported for the current head

<a id="shumkov"></a>
### @shumkov
<a id="shumkov-open"></a>
#### Open (7)
- [dashpay/rust-dashcore#634 feat(key-wallet-manager): single-map WalletManager with Arc&lt;RwLock&lt;T&gt;&gt; per wallet](https://github.com/dashpay/rust-dashcore/pull/634) — 20 unresolved (20 CodeRabbit) · 230 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · areas: dash-spv, key-wallet, key-wallet-manager, fallback · Policy: waiting-author
  - Top thread: "_⚠️ Potential issue_ \| _🟡 Minor_" — 230 days old
  - Blocker: coderabbitai requested changes on this head; dismiss the review or push a fix
  - Blocker: coderabbitai left review threads unresolved; resolve them
- [dashpay/platform#5656 fix(dashmate): build local images for every group node that builds any](https://github.com/dashpay/platform/pull/5656) — 1 unresolved (1 bot) · 25 days stale · ✋ changes requested · 🔴 CI failing · 🐢 targets fix/dashmate/update-path-bugs · areas: dashmate
  - Top thread: "🔴 Blocking: Build the shared base image before dependent service images across group configurations**" — 25 days old
- [dashpay/platform#6209 fix(dashmate): move every Core 23 image of dashpay/dashd to Core 24](https://github.com/dashpay/platform/pull/6209) — 3 unresolved (3 bot) · 0 days stale · ✋ changes requested · 🔴 CI failing · areas: dashmate · Policy: waiting-author
  - Top thread: "🟡 Suggestion: The Core upgrade warning stays in helper logs when the helper migrates first**" — 0 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#6210 docs(dashmate): describe updating a Debian package installation](https://github.com/dashpay/platform/pull/6210) — 3 unresolved (3 bot) · 0 days stale · ✋ changes requested · 🔴 CI failing · areas: dashmate · Policy: waiting-author
  - Top thread: "🟡 Suggestion: A package replaced in a shared directory can be installed as root**" — 0 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#4283 fix(dashmate): keep the node up when an image pull fails, and stop reporting success when it did not](https://github.com/dashpay/platform/pull/4283) — ⚠ merge conflict · 🐢 targets v5.2-dev · areas: dashmate · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 3ceb3315ea9b383ae5816ea4f0e9961202770ed9
  - Blocker: Proceeded without thepastaclaw: no review within the configured window
- [dashpay/platform#5763 docs: e2e test architecture and legacy JS retirement plans (draft, not for merge)](https://github.com/dashpay/platform/pull/5763) — 📝 draft · 🐢 targets v5.2-dev · areas: fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5819 feat(dashmate): add the modules that let dashmate update itself](https://github.com/dashpay/platform/pull/5819) — 📝 draft · 🐢 targets fix/dashmate/update-path-bugs · areas: dashmate

<a id="shumkov-needs-action"></a>
#### Needs action (3)
- [dashpay/rust-dashcore#634 feat(key-wallet-manager): single-map WalletManager with Arc&lt;RwLock&lt;T&gt;&gt; per wallet](https://github.com/dashpay/rust-dashcore/pull/634) — 20 unresolved (20 CodeRabbit) · 230 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · areas: dash-spv, key-wallet, key-wallet-manager, fallback · Policy: waiting-author
  - Top thread: "_⚠️ Potential issue_ \| _🟡 Minor_" — 230 days old
  - Blocker: coderabbitai requested changes on this head; dismiss the review or push a fix
  - Blocker: coderabbitai left review threads unresolved; resolve them
- [dashpay/platform#6209 fix(dashmate): move every Core 23 image of dashpay/dashd to Core 24](https://github.com/dashpay/platform/pull/6209) — 3 unresolved (3 bot) · 0 days stale · ✋ changes requested · 🔴 CI failing · areas: dashmate · Policy: waiting-author
  - Top thread: "🟡 Suggestion: The Core upgrade warning stays in helper logs when the helper migrates first**" — 0 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#6210 docs(dashmate): describe updating a Debian package installation](https://github.com/dashpay/platform/pull/6210) — 3 unresolved (3 bot) · 0 days stale · ✋ changes requested · 🔴 CI failing · areas: dashmate · Policy: waiting-author
  - Top thread: "🟡 Suggestion: A package replaced in a shared directory can be installed as root**" — 0 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them

<a id="shumkov-unresolved-comments"></a>
#### Unresolved Comments (3)
- [dashpay/rust-dashcore#634 feat(key-wallet-manager): single-map WalletManager with Arc&lt;RwLock&lt;T&gt;&gt; per wallet](https://github.com/dashpay/rust-dashcore/pull/634) — 20 unresolved (20 CodeRabbit) · 230 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · areas: dash-spv, key-wallet, key-wallet-manager, fallback · Policy: waiting-author
  - Top thread: "_⚠️ Potential issue_ \| _🟡 Minor_" — 230 days old
  - Blocker: coderabbitai requested changes on this head; dismiss the review or push a fix
  - Blocker: coderabbitai left review threads unresolved; resolve them
- [dashpay/platform#6209 fix(dashmate): move every Core 23 image of dashpay/dashd to Core 24](https://github.com/dashpay/platform/pull/6209) — 3 unresolved (3 bot) · 0 days stale · ✋ changes requested · 🔴 CI failing · areas: dashmate · Policy: waiting-author
  - Top thread: "🟡 Suggestion: The Core upgrade warning stays in helper logs when the helper migrates first**" — 0 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#6210 docs(dashmate): describe updating a Debian package installation](https://github.com/dashpay/platform/pull/6210) — 3 unresolved (3 bot) · 0 days stale · ✋ changes requested · 🔴 CI failing · areas: dashmate · Policy: waiting-author
  - Top thread: "🟡 Suggestion: A package replaced in a shared directory can be installed as root**" — 0 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them

<a id="shumkov-stale"></a>
#### Stale (4)
- [dashpay/platform#5656 fix(dashmate): build local images for every group node that builds any](https://github.com/dashpay/platform/pull/5656) — 1 unresolved (1 bot) · 25 days stale · ✋ changes requested · 🔴 CI failing · 🐢 targets fix/dashmate/update-path-bugs · areas: dashmate
  - Top thread: "🔴 Blocking: Build the shared base image before dependent service images across group configurations**" — 25 days old
- [dashpay/platform#4283 fix(dashmate): keep the node up when an image pull fails, and stop reporting success when it did not](https://github.com/dashpay/platform/pull/4283) — ⚠ merge conflict · 🐢 targets v5.2-dev · areas: dashmate · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 3ceb3315ea9b383ae5816ea4f0e9961202770ed9
  - Blocker: Proceeded without thepastaclaw: no review within the configured window
- [dashpay/platform#5763 docs: e2e test architecture and legacy JS retirement plans (draft, not for merge)](https://github.com/dashpay/platform/pull/5763) — 📝 draft · 🐢 targets v5.2-dev · areas: fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5819 feat(dashmate): add the modules that let dashmate update itself](https://github.com/dashpay/platform/pull/5819) — 📝 draft · 🐢 targets fix/dashmate/update-path-bugs · areas: dashmate

<a id="shumkov-ready-for-review"></a>
#### Ready for Review (2)
- [dashpay/rust-dashcore#1490 ci: test only the crates a pull request changes](https://github.com/dashpay/rust-dashcore/pull/1490) — by @ZocoLini · areas: github, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 57c19ebefff21e528e338b7cc11d43d483fc8012
- [dashpay/tenderdash#1584 fix(statesync): restore LastAppHash of the snapshot height, not the next one](https://github.com/dashpay/tenderdash/pull/1584) — by @QuantumExplorer · areas: fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 23c1443e4077c7c7dfedf00f9446c762e669660f

<a id="lklimek"></a>
### @lklimek
<a id="lklimek-open"></a>
#### Open (20)
- [dashpay/platform#5210 feat(wallet-storage)!: restore complete Core wallet snapshots from SQLite](https://github.com/dashpay/platform/pull/5210) — 6 unresolved (6 bot) · 7 days stale · 🔴 CI failing · 🐢 targets v5.2-dev · areas: rs-platform-wallet, wallet-storage, swift-sdk, fallback · Policy: waiting-author
  - Top thread: "🔴 Blocking: Make one-time funding-key consumption survive row deletion**" — 7 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5528 fix(platform-wallet): retain asset-lock recovery metadata through funding conflicts](https://github.com/dashpay/platform/pull/5528) — 3 unresolved (3 bot) · 34 days stale · ✋ changes requested · 📝 draft · 🐢 targets feat/sqlite-wallet-restore · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk, kotlin-sdk
  - Top thread: "🟡 Suggestion: Retain diagnostics for malformed FFI operation snapshots**" — 34 days old
- [dashpay/tenderdash#1515 fix(service)!: join remaining service workers before shutdown completes](https://github.com/dashpay/tenderdash/pull/1515) — 4 unresolved (4 bot) · 7 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · areas: fallback · Policy: waiting-author
  - Top thread: "🔴 Blocking: Serialize reconnect resubscriptions with subscription changes**" — 7 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/dash-evo-tool#1161 fix(ci): isolate release signing and constrain agent write paths](https://github.com/dashpay/dash-evo-tool/pull/1161) — 1 unresolved (1 bot) · 2 days stale · ✋ changes requested · 🔴 CI failing · areas: github, fallback · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Release policy tests accept older or floating artifact downloader refs**" — 2 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/platform#6024 feat(platform-wallet): check a masternode key by its hash without its secret](https://github.com/dashpay/platform/pull/6024) — 1 unresolved (1 bot) · 0 days stale · 🐢 targets v5.2-dev · areas: rs-platform-wallet · Policy: waiting-self-review
  - Top thread: "🟡 Suggestion: A registered Platform node ID looks like the wrong key in Core wire order**" — 0 days old
  - Blocker: Author must post /self-reviewed 18f90bbfd92650b5e717cfaca467e7981112def0
- [dashpay/dash-evo-tool#1228 fix(tokens): show a group proposal to a co-signer whose wallet is locked](https://github.com/dashpay/dash-evo-tool/pull/1228) — 📝 draft · 🐢 targets fix/protocol14-signing-key-selection · areas: fallback
- [dashpay/dash-evo-tool#1229 fix(boot): show a panic while building the first screens instead of closing](https://github.com/dashpay/dash-evo-tool/pull/1229) — 📝 draft · 🐢 targets fix/boot-panic-dialog · ⚠ routing unavailable: no changed-file evidence
- [dashpay/dash-evo-tool#1230 fix(tokens): never ask to pay again for a registration Platform already confirmed](https://github.com/dashpay/dash-evo-tool/pull/1230) — ✋ changes requested · 🔴 CI failing · ⚠ routing unavailable: no changed-file evidence · Policy: waiting-author
  - Blocker: No changes against the base branch: close this pull request or push changes
- [dashpay/dash-evo-tool#1231 refactor(identity): discover wallet identities through platform-wallet](https://github.com/dashpay/dash-evo-tool/pull/1231) — ⚠ routing unavailable: no changed-file evidence · Policy: waiting-author
  - Blocker: No changes against the base branch: close this pull request or push changes
- [dashpay/platform#5220 fix(platform-wallet)!: replay recorded transaction history on load for every persister](https://github.com/dashpay/platform/pull/5220) — 📝 draft · ⏸ deferred · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk, kotlin-sdk, fallback
- [dashpay/platform#5519 refactor: remove DPP raw secp256k1 dependency](https://github.com/dashpay/platform/pull/5519) — 🔴 CI failing · 🐢 targets v5.2-dev · areas: rs-drive-abci, dpp, rs-platform-wallet, rs-platform-wallet-ffi, rust-sdk, rust-sdk-ffi, js-wasm-sdk, fallback · Policy: waiting-build
  - Blocker: The build must pass before a human is asked
- [dashpay/platform#5949 fix(sdk): bump tokio to 1.54.1 and fix the futurelocks it exposes](https://github.com/dashpay/platform/pull/5949) — 🐢 targets v5.2-dev · areas: rs-platform-wallet, rust-sdk, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
- [dashpay/platform#6102 fix(platform-wallet): keep invitation issuance floor across reloads](https://github.com/dashpay/platform/pull/6102) — ✋ changes requested · 📝 draft · 🐢 targets chore/storage-drop-unreachable-refill-guard · areas: rs-platform-wallet, rs-platform-wallet-ffi
- [dashpay/rust-dashcore#1336 fix(dash-spv): defer unverified InstantLocks](https://github.com/dashpay/rust-dashcore/pull/1336) — ✋ changes requested · areas: dash-spv · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
- [dashpay/rust-dashcore#1416 feat(key-wallet): verify owner authorization of a pending spend](https://github.com/dashpay/rust-dashcore/pull/1416) — ✋ changes requested · 🔴 CI failing · areas: key-wallet, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed f134961bca06d23a70b88f02c7c03efbfd6f5508
- [dashpay/rust-dashcore#1448 fix(dash-spv): keep \`QRInfo\` catch-up across cycles](https://github.com/dashpay/rust-dashcore/pull/1448) — ⚠ merge conflict · 🔴 CI failing · 📝 draft · areas: dash-spv · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#1465 feat(key-wallet): find a provider key by its hash](https://github.com/dashpay/rust-dashcore/pull/1465) — 📝 draft · areas: key-wallet · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#1466 feat(dash-spv): read the masternode list only while it is current](https://github.com/dashpay/rust-dashcore/pull/1466) — 📝 draft · 🐢 targets feat/provider-key-lookup-by-hash · areas: dash-spv
- [dashpay/rust-dashcore#1499 feat(dash-spv)!: add \`InstantLock\` verification API](https://github.com/dashpay/rust-dashcore/pull/1499) — 📝 draft · 🐢 targets fix/wallet-restore-authorization · areas: dash-spv
- [dashpay/tenderdash#1593 feat(consensus)!: specify full consensus parameter commitment and remove app version parameter](https://github.com/dashpay/tenderdash/pull/1593) — ⚠ merge conflict · ✋ changes requested · 📝 draft · areas: fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot

<a id="lklimek-needs-action"></a>
#### Needs action (5)
- [dashpay/tenderdash#1515 fix(service)!: join remaining service workers before shutdown completes](https://github.com/dashpay/tenderdash/pull/1515) — 4 unresolved (4 bot) · 7 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · areas: fallback · Policy: waiting-author
  - Top thread: "🔴 Blocking: Serialize reconnect resubscriptions with subscription changes**" — 7 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/dash-evo-tool#1161 fix(ci): isolate release signing and constrain agent write paths](https://github.com/dashpay/dash-evo-tool/pull/1161) — 1 unresolved (1 bot) · 2 days stale · ✋ changes requested · 🔴 CI failing · areas: github, fallback · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Release policy tests accept older or floating artifact downloader refs**" — 2 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/dash-evo-tool#1230 fix(tokens): never ask to pay again for a registration Platform already confirmed](https://github.com/dashpay/dash-evo-tool/pull/1230) — ✋ changes requested · 🔴 CI failing · ⚠ routing unavailable: no changed-file evidence · Policy: waiting-author
  - Blocker: No changes against the base branch: close this pull request or push changes
- [dashpay/rust-dashcore#1336 fix(dash-spv): defer unverified InstantLocks](https://github.com/dashpay/rust-dashcore/pull/1336) — ✋ changes requested · areas: dash-spv · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
- [dashpay/rust-dashcore#1416 feat(key-wallet): verify owner authorization of a pending spend](https://github.com/dashpay/rust-dashcore/pull/1416) — ✋ changes requested · 🔴 CI failing · areas: key-wallet, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed f134961bca06d23a70b88f02c7c03efbfd6f5508

<a id="lklimek-ready-for-human"></a>
#### Ready for human (2)
- [dashpay/platform#5949 fix(sdk): bump tokio to 1.54.1 and fix the futurelocks it exposes](https://github.com/dashpay/platform/pull/5949) — 🐢 targets v5.2-dev · areas: rs-platform-wallet, rust-sdk, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
- [dashpay/rust-dashcore#1336 fix(dash-spv): defer unverified InstantLocks](https://github.com/dashpay/rust-dashcore/pull/1336) — ✋ changes requested · areas: dash-spv · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required

<a id="lklimek-unresolved-comments"></a>
#### Unresolved Comments (2)
- [dashpay/tenderdash#1515 fix(service)!: join remaining service workers before shutdown completes](https://github.com/dashpay/tenderdash/pull/1515) — 4 unresolved (4 bot) · 7 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · areas: fallback · Policy: waiting-author
  - Top thread: "🔴 Blocking: Serialize reconnect resubscriptions with subscription changes**" — 7 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/dash-evo-tool#1161 fix(ci): isolate release signing and constrain agent write paths](https://github.com/dashpay/dash-evo-tool/pull/1161) — 1 unresolved (1 bot) · 2 days stale · ✋ changes requested · 🔴 CI failing · areas: github, fallback · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Release policy tests accept older or floating artifact downloader refs**" — 2 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: Proceeded without coderabbitai: no review within the configured window

<a id="lklimek-changes-requested"></a>
#### Changes Requested (3)
- [dashpay/dash-evo-tool#1230 fix(tokens): never ask to pay again for a registration Platform already confirmed](https://github.com/dashpay/dash-evo-tool/pull/1230) — ✋ changes requested · 🔴 CI failing · ⚠ routing unavailable: no changed-file evidence · Policy: waiting-author
  - Blocker: No changes against the base branch: close this pull request or push changes
- [dashpay/rust-dashcore#1336 fix(dash-spv): defer unverified InstantLocks](https://github.com/dashpay/rust-dashcore/pull/1336) — ✋ changes requested · areas: dash-spv · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
- [dashpay/rust-dashcore#1416 feat(key-wallet): verify owner authorization of a pending spend](https://github.com/dashpay/rust-dashcore/pull/1416) — ✋ changes requested · 🔴 CI failing · areas: key-wallet, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed f134961bca06d23a70b88f02c7c03efbfd6f5508

<a id="lklimek-deferred"></a>
#### Deferred (1)
- [dashpay/platform#5220 fix(platform-wallet)!: replay recorded transaction history on load for every persister](https://github.com/dashpay/platform/pull/5220) — 📝 draft · ⏸ deferred · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk, kotlin-sdk, fallback

<a id="lklimek-draft"></a>
#### Draft (3)
- [dashpay/rust-dashcore#1448 fix(dash-spv): keep \`QRInfo\` catch-up across cycles](https://github.com/dashpay/rust-dashcore/pull/1448) — ⚠ merge conflict · 🔴 CI failing · 📝 draft · areas: dash-spv · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#1465 feat(key-wallet): find a provider key by its hash](https://github.com/dashpay/rust-dashcore/pull/1465) — 📝 draft · areas: key-wallet · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/tenderdash#1593 feat(consensus)!: specify full consensus parameter commitment and remove app version parameter](https://github.com/dashpay/tenderdash/pull/1593) — ⚠ merge conflict · ✋ changes requested · 📝 draft · areas: fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot

<a id="lklimek-stale"></a>
#### Stale (10)
- [dashpay/platform#5210 feat(wallet-storage)!: restore complete Core wallet snapshots from SQLite](https://github.com/dashpay/platform/pull/5210) — 6 unresolved (6 bot) · 7 days stale · 🔴 CI failing · 🐢 targets v5.2-dev · areas: rs-platform-wallet, wallet-storage, swift-sdk, fallback · Policy: waiting-author
  - Top thread: "🔴 Blocking: Make one-time funding-key consumption survive row deletion**" — 7 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5528 fix(platform-wallet): retain asset-lock recovery metadata through funding conflicts](https://github.com/dashpay/platform/pull/5528) — 3 unresolved (3 bot) · 34 days stale · ✋ changes requested · 📝 draft · 🐢 targets feat/sqlite-wallet-restore · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk, kotlin-sdk
  - Top thread: "🟡 Suggestion: Retain diagnostics for malformed FFI operation snapshots**" — 34 days old
- [dashpay/platform#6024 feat(platform-wallet): check a masternode key by its hash without its secret](https://github.com/dashpay/platform/pull/6024) — 1 unresolved (1 bot) · 0 days stale · 🐢 targets v5.2-dev · areas: rs-platform-wallet · Policy: waiting-self-review
  - Top thread: "🟡 Suggestion: A registered Platform node ID looks like the wrong key in Core wire order**" — 0 days old
  - Blocker: Author must post /self-reviewed 18f90bbfd92650b5e717cfaca467e7981112def0
- [dashpay/dash-evo-tool#1228 fix(tokens): show a group proposal to a co-signer whose wallet is locked](https://github.com/dashpay/dash-evo-tool/pull/1228) — 📝 draft · 🐢 targets fix/protocol14-signing-key-selection · areas: fallback
- [dashpay/dash-evo-tool#1229 fix(boot): show a panic while building the first screens instead of closing](https://github.com/dashpay/dash-evo-tool/pull/1229) — 📝 draft · 🐢 targets fix/boot-panic-dialog · ⚠ routing unavailable: no changed-file evidence
- [dashpay/platform#5519 refactor: remove DPP raw secp256k1 dependency](https://github.com/dashpay/platform/pull/5519) — 🔴 CI failing · 🐢 targets v5.2-dev · areas: rs-drive-abci, dpp, rs-platform-wallet, rs-platform-wallet-ffi, rust-sdk, rust-sdk-ffi, js-wasm-sdk, fallback · Policy: waiting-build
  - Blocker: The build must pass before a human is asked
- [dashpay/platform#5949 fix(sdk): bump tokio to 1.54.1 and fix the futurelocks it exposes](https://github.com/dashpay/platform/pull/5949) — 🐢 targets v5.2-dev · areas: rs-platform-wallet, rust-sdk, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
- [dashpay/platform#6102 fix(platform-wallet): keep invitation issuance floor across reloads](https://github.com/dashpay/platform/pull/6102) — ✋ changes requested · 📝 draft · 🐢 targets chore/storage-drop-unreachable-refill-guard · areas: rs-platform-wallet, rs-platform-wallet-ffi
- [dashpay/rust-dashcore#1466 feat(dash-spv): read the masternode list only while it is current](https://github.com/dashpay/rust-dashcore/pull/1466) — 📝 draft · 🐢 targets feat/provider-key-lookup-by-hash · areas: dash-spv
- [dashpay/rust-dashcore#1499 feat(dash-spv)!: add \`InstantLock\` verification API](https://github.com/dashpay/rust-dashcore/pull/1499) — 📝 draft · 🐢 targets fix/wallet-restore-authorization · areas: dash-spv

<a id="lklimek-clean"></a>
#### Clean (1)
- [dashpay/dash-evo-tool#1231 refactor(identity): discover wallet identities through platform-wallet](https://github.com/dashpay/dash-evo-tool/pull/1231) — ⚠ routing unavailable: no changed-file evidence · Policy: waiting-author
  - Blocker: No changes against the base branch: close this pull request or push changes

<a id="xdustinface"></a>
### @xdustinface
<a id="xdustinface-open"></a>
#### Open (9)
- [dashpay/platform#5791 fix(platform-wallet): refuse a quorum type that does not fit a byte](https://github.com/dashpay/platform/pull/5791) — 2 unresolved (1 CodeRabbit, 1 bot) · 18 days stale · ⚠ merge conflict · 🐢 targets v5.2-dev · areas: rs-platform-wallet · Policy: waiting-author
  - Top thread: "&lt;picture&gt;&lt;img src="https://storage.googleapis.com/coderabbit_public_assets/review-comment-icons-v1/amber-16.svg" width="…" — 18 days old
  - Blocker: coderabbitai left review threads unresolved; resolve them
- [dashpay/rust-dashcore#1483 fix(dash-spv): keep bans in force across a redial](https://github.com/dashpay/rust-dashcore/pull/1483) — 1 unresolved (1 CodeRabbit) · 1 days stale · 🔴 CI failing · areas: dash-spv · Policy: waiting-author
  - Top thread: "&lt;picture&gt;&lt;img src="https://storage.googleapis.com/coderabbit_public_assets/review-comment-icons-v1/amber-16.svg" width="…" — 1 days old
  - Blocker: coderabbitai left review threads unresolved; resolve them
- [dashpay/rust-dashcore#541 ci: integrate Codecov test analytics via \`cargo-nextest\`](https://github.com/dashpay/rust-dashcore/pull/541) — ⚠ merge conflict · 📝 draft · areas: github, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#1372 fix(dash-spv): spread peers across network groups](https://github.com/dashpay/rust-dashcore/pull/1372) — 📝 draft · areas: dash-spv · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#1373 fix(dash-spv): rotate the slowest peer of a slow set](https://github.com/dashpay/rust-dashcore/pull/1373) — ⚠ merge conflict · 📝 draft · 🐢 targets fix/peer-diversity · areas: dash-spv
- [dashpay/rust-dashcore#1374 fix(dash-spv): retry a refused request at another peer](https://github.com/dashpay/rust-dashcore/pull/1374) — 📝 draft · 🐢 targets fix/peer-exploration · areas: dash-spv
- [dashpay/rust-dashcore#1375 fix(dash-spv): rank a stalled peer last when routing](https://github.com/dashpay/rust-dashcore/pull/1375) — 📝 draft · 🐢 targets fix/peer-exploration · areas: dash-spv
- [dashpay/rust-dashcore#1473 fix(dash-spv): hedge the filter batch that blocks the store](https://github.com/dashpay/rust-dashcore/pull/1473) — ✋ changes requested · 🔴 CI failing · areas: dash-spv · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 7f3fe33c04b4adaac8c12f03d078925a170b3299
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/rust-dashcore#1474 fix(dash-spv): hedge a request once it waited twice its queue](https://github.com/dashpay/rust-dashcore/pull/1474) — ⚠ merge conflict · 📝 draft · areas: dash-spv · Policy: draft
  - Blocker: Draft PR does not occupy a review slot

<a id="xdustinface-needs-action"></a>
#### Needs action (2)
- [dashpay/rust-dashcore#1483 fix(dash-spv): keep bans in force across a redial](https://github.com/dashpay/rust-dashcore/pull/1483) — 1 unresolved (1 CodeRabbit) · 1 days stale · 🔴 CI failing · areas: dash-spv · Policy: waiting-author
  - Top thread: "&lt;picture&gt;&lt;img src="https://storage.googleapis.com/coderabbit_public_assets/review-comment-icons-v1/amber-16.svg" width="…" — 1 days old
  - Blocker: coderabbitai left review threads unresolved; resolve them
- [dashpay/rust-dashcore#1473 fix(dash-spv): hedge the filter batch that blocks the store](https://github.com/dashpay/rust-dashcore/pull/1473) — ✋ changes requested · 🔴 CI failing · areas: dash-spv · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 7f3fe33c04b4adaac8c12f03d078925a170b3299
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return

<a id="xdustinface-unresolved-comments"></a>
#### Unresolved Comments (1)
- [dashpay/rust-dashcore#1483 fix(dash-spv): keep bans in force across a redial](https://github.com/dashpay/rust-dashcore/pull/1483) — 1 unresolved (1 CodeRabbit) · 1 days stale · 🔴 CI failing · areas: dash-spv · Policy: waiting-author
  - Top thread: "&lt;picture&gt;&lt;img src="https://storage.googleapis.com/coderabbit_public_assets/review-comment-icons-v1/amber-16.svg" width="…" — 1 days old
  - Blocker: coderabbitai left review threads unresolved; resolve them

<a id="xdustinface-changes-requested"></a>
#### Changes Requested (1)
- [dashpay/rust-dashcore#1473 fix(dash-spv): hedge the filter batch that blocks the store](https://github.com/dashpay/rust-dashcore/pull/1473) — ✋ changes requested · 🔴 CI failing · areas: dash-spv · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 7f3fe33c04b4adaac8c12f03d078925a170b3299
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return

<a id="xdustinface-draft"></a>
#### Draft (3)
- [dashpay/rust-dashcore#541 ci: integrate Codecov test analytics via \`cargo-nextest\`](https://github.com/dashpay/rust-dashcore/pull/541) — ⚠ merge conflict · 📝 draft · areas: github, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#1372 fix(dash-spv): spread peers across network groups](https://github.com/dashpay/rust-dashcore/pull/1372) — 📝 draft · areas: dash-spv · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#1474 fix(dash-spv): hedge a request once it waited twice its queue](https://github.com/dashpay/rust-dashcore/pull/1474) — ⚠ merge conflict · 📝 draft · areas: dash-spv · Policy: draft
  - Blocker: Draft PR does not occupy a review slot

<a id="xdustinface-stale"></a>
#### Stale (4)
- [dashpay/platform#5791 fix(platform-wallet): refuse a quorum type that does not fit a byte](https://github.com/dashpay/platform/pull/5791) — 2 unresolved (1 CodeRabbit, 1 bot) · 18 days stale · ⚠ merge conflict · 🐢 targets v5.2-dev · areas: rs-platform-wallet · Policy: waiting-author
  - Top thread: "&lt;picture&gt;&lt;img src="https://storage.googleapis.com/coderabbit_public_assets/review-comment-icons-v1/amber-16.svg" width="…" — 18 days old
  - Blocker: coderabbitai left review threads unresolved; resolve them
- [dashpay/rust-dashcore#1373 fix(dash-spv): rotate the slowest peer of a slow set](https://github.com/dashpay/rust-dashcore/pull/1373) — ⚠ merge conflict · 📝 draft · 🐢 targets fix/peer-diversity · areas: dash-spv
- [dashpay/rust-dashcore#1374 fix(dash-spv): retry a refused request at another peer](https://github.com/dashpay/rust-dashcore/pull/1374) — 📝 draft · 🐢 targets fix/peer-exploration · areas: dash-spv
- [dashpay/rust-dashcore#1375 fix(dash-spv): rank a stalled peer last when routing](https://github.com/dashpay/rust-dashcore/pull/1375) — 📝 draft · 🐢 targets fix/peer-exploration · areas: dash-spv

<a id="xdustinface-ready-for-review"></a>
#### Ready for Review (4)
- [dashpay/rust-dashcore#1328 refactor(key-wallet)!: validate child indexes at construction with \`ChildIndex\`](https://github.com/dashpay/rust-dashcore/pull/1328) — by @ZocoLini · areas: dash-spv, key-wallet, key-wallet-manager · Policy: waiting-bots
  - Blocker: coderabbitai has not reported for the current head
- [dashpay/rust-dashcore#1426 fix(dash-spv): tell an empty filter header store from genesis](https://github.com/dashpay/rust-dashcore/pull/1426) — by @ZocoLini · areas: dash-spv · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed d31ec981450f9216f70fd0a3877e22563060db0c
- [dashpay/rust-dashcore#1490 ci: test only the crates a pull request changes](https://github.com/dashpay/rust-dashcore/pull/1490) — by @ZocoLini · areas: github, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 57c19ebefff21e528e338b7cc11d43d483fc8012
- [dashpay/rust-dashcore#1502 refactor(dash-spv)!: keep only storage failures in \`StorageError\`](https://github.com/dashpay/rust-dashcore/pull/1502) — by @ZocoLini · areas: dash-spv · Policy: waiting-bots
  - Blocker: coderabbitai has not reported for the current head

<a id="romchornyi"></a>
### @romchornyi
<a id="romchornyi-open"></a>
#### Open (15)
- [dashpay/platform#5930 feat(platform-wallet): record the payment intent of external asset locks](https://github.com/dashpay/platform/pull/5930) — 9 unresolved (9 bot) · 9 days stale · ⚠ merge conflict · 🔴 CI failing · 🐢 targets v5.2-dev · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk, kotlin-sdk · Policy: waiting-author
  - Top thread: "✅ **Resolved** at \`c41628d3\`; see the replies below." — 9 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5258 feat(platform-wallet): probe the network about sends with an unknown broadcast outcome](https://github.com/dashpay/platform/pull/5258) — 3 unresolved (3 human) · 39 days stale · ⚠ merge conflict · 📝 draft · 🐢 targets v5.2-dev · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, rust-sdk · Policy: draft
  - Top thread: "\[P2\] Carry the originating wallet generation in probe events" — 39 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5540 fix(platform-wallet): store externally supplied identity keys without a derivation breadcrumb](https://github.com/dashpay/platform/pull/5540) — 3 unresolved (3 bot) · 33 days stale · ⚠ merge conflict · 🔴 CI failing · 🐢 targets v5.2-dev · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk · Policy: waiting-author
  - Top thread: "✅ **Resolved** at \`b25b2543\`; see the replies below." — 33 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5549 fix(platform-wallet-storage): restore DashPay contact address pools on SQLite load](https://github.com/dashpay/platform/pull/5549) — 3 unresolved (3 bot) · 32 days stale · ⚠ merge conflict · 🔴 CI failing · 📝 draft · 🐢 targets v5.2-dev · areas: wallet-storage · Policy: draft
  - Top thread: "✅ **Withdrawn** at \`2bee2705\`; see the replies below." — 32 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5395 feat(platform-wallet): spend only final coins and report funds waiting on the network](https://github.com/dashpay/platform/pull/5395) — 1 unresolved (1 human) · 39 days stale · ⚠ merge conflict · ✋ changes requested · 📝 draft · 🐢 targets v5.2-dev · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk · Policy: draft
  - Top thread: "this should belong to another layer - likely platform-wallet, or even rust-dashcore's key-wallet. FFI is not meant to st…" — 39 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#6139 feat(swift-sdk): top up someone else's identity from the Core balance](https://github.com/dashpay/platform/pull/6139) — 7 unresolved (7 bot) · 1 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · 📝 draft · 🐢 targets HashEngineering/platform-wallet-top-up-someone-elses-identity-fr · areas: swift-sdk, kotlin-sdk, sdk-parity
  - Top thread: "🟡 Suggestion: A conflicting consumed snapshot can make a live top-up look completed**" — 1 days old
- [dashpay/platform#5928 feat(platform-wallet): report whether a failed payment may have left the wallet](https://github.com/dashpay/platform/pull/5928) — 4 unresolved (4 bot) · 5 days stale · 🐢 targets v5.2-dev · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk · Policy: waiting-author
  - Top thread: "✅ **Resolved** at \`a32de9cc\`; see the replies below." — 5 days old
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5853 feat: report a shielded identity create proven to have its inputs spent without its identity](https://github.com/dashpay/platform/pull/5853) — 1 unresolved (1 bot) · 6 days stale · 🐢 targets v5.2-dev · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, rust-sdk, kotlin-sdk, fallback · Policy: waiting-author
  - Top thread: "✅ **Withdrawn** at \`73f13d31\`; see the replies below." — 6 days old
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5536 fix!: check a DashPay invitation claim against Platform before broadcasting it](https://github.com/dashpay/platform/pull/5536) — 1 unresolved (1 bot) · 1 days stale · 🐢 targets v5.2-dev · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, rust-sdk, kotlin-sdk, sdk-parity · Policy: waiting-self-review
  - Top thread: "✅ **Resolved** at \`97875d14\`; see the replies below." — 1 days old
  - Blocker: Author must post /self-reviewed 97875d14db9d952738cc0a246806d2e259df9cf1
- [dashpay/platform#4992 feat(swift-sdk): durable wallet presence marker readable while the device is locked](https://github.com/dashpay/platform/pull/4992) — 🐢 targets v5.2-dev · areas: swift-sdk · Policy: ready-to-merge
  - Blocker: All policy requirements are satisfied
- [dashpay/platform#5587 fix(platform-wallet)!: keep a contact's payment account across relaunches and replace it cleanly on rotation](https://github.com/dashpay/platform/pull/5587) — ⚠ merge conflict · 📝 draft · 🐢 targets v5.2-dev · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5589 feat(platform-wallet-ffi)!: let hosts reset an outbound contact account's pool and keep its request digest](https://github.com/dashpay/platform/pull/5589) — 🔴 CI failing · 📝 draft · 🐢 targets fix/contact-rebuild-keeps-pool · areas: rs-platform-wallet-ffi, swift-sdk
- [dashpay/platform#5590 feat(kotlin-sdk)!: reset an outbound contact account's pool and keep its request digest on Android](https://github.com/dashpay/platform/pull/5590) — 🔴 CI failing · 📝 draft · 🐢 targets fix/contact-rebuild-ffi-swift · areas: kotlin-sdk, sdk-parity
- [dashpay/platform#5865 feat(platform-wallet): plan a shielded spend with its exact fee and spend against the plan](https://github.com/dashpay/platform/pull/5865) — 🐢 targets v5.2-dev · areas: dpp, rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, rust-sdk, kotlin-sdk, sdk-parity, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
- [dashpay/platform#6014 feat(platform-wallet)!: give identity slots one wallet-owned lease](https://github.com/dashpay/platform/pull/6014) — ⚠ merge conflict · ✋ changes requested · 📝 draft · 🐢 targets feat/invitation-claim-preflight · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, rust-sdk, kotlin-sdk, sdk-parity

<a id="romchornyi-ready-for-human"></a>
#### Ready for human (2)
- [dashpay/platform#4992 feat(swift-sdk): durable wallet presence marker readable while the device is locked](https://github.com/dashpay/platform/pull/4992) — 🐢 targets v5.2-dev · areas: swift-sdk · Policy: ready-to-merge
  - Blocker: All policy requirements are satisfied
- [dashpay/platform#5865 feat(platform-wallet): plan a shielded spend with its exact fee and spend against the plan](https://github.com/dashpay/platform/pull/5865) — 🐢 targets v5.2-dev · areas: dpp, rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, rust-sdk, kotlin-sdk, sdk-parity, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required

<a id="romchornyi-stale"></a>
#### Stale (15)
- [dashpay/platform#5930 feat(platform-wallet): record the payment intent of external asset locks](https://github.com/dashpay/platform/pull/5930) — 9 unresolved (9 bot) · 9 days stale · ⚠ merge conflict · 🔴 CI failing · 🐢 targets v5.2-dev · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk, kotlin-sdk · Policy: waiting-author
  - Top thread: "✅ **Resolved** at \`c41628d3\`; see the replies below." — 9 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5258 feat(platform-wallet): probe the network about sends with an unknown broadcast outcome](https://github.com/dashpay/platform/pull/5258) — 3 unresolved (3 human) · 39 days stale · ⚠ merge conflict · 📝 draft · 🐢 targets v5.2-dev · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, rust-sdk · Policy: draft
  - Top thread: "\[P2\] Carry the originating wallet generation in probe events" — 39 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5540 fix(platform-wallet): store externally supplied identity keys without a derivation breadcrumb](https://github.com/dashpay/platform/pull/5540) — 3 unresolved (3 bot) · 33 days stale · ⚠ merge conflict · 🔴 CI failing · 🐢 targets v5.2-dev · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk · Policy: waiting-author
  - Top thread: "✅ **Resolved** at \`b25b2543\`; see the replies below." — 33 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5549 fix(platform-wallet-storage): restore DashPay contact address pools on SQLite load](https://github.com/dashpay/platform/pull/5549) — 3 unresolved (3 bot) · 32 days stale · ⚠ merge conflict · 🔴 CI failing · 📝 draft · 🐢 targets v5.2-dev · areas: wallet-storage · Policy: draft
  - Top thread: "✅ **Withdrawn** at \`2bee2705\`; see the replies below." — 32 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5395 feat(platform-wallet): spend only final coins and report funds waiting on the network](https://github.com/dashpay/platform/pull/5395) — 1 unresolved (1 human) · 39 days stale · ⚠ merge conflict · ✋ changes requested · 📝 draft · 🐢 targets v5.2-dev · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk · Policy: draft
  - Top thread: "this should belong to another layer - likely platform-wallet, or even rust-dashcore's key-wallet. FFI is not meant to st…" — 39 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#6139 feat(swift-sdk): top up someone else's identity from the Core balance](https://github.com/dashpay/platform/pull/6139) — 7 unresolved (7 bot) · 1 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · 📝 draft · 🐢 targets HashEngineering/platform-wallet-top-up-someone-elses-identity-fr · areas: swift-sdk, kotlin-sdk, sdk-parity
  - Top thread: "🟡 Suggestion: A conflicting consumed snapshot can make a live top-up look completed**" — 1 days old
- [dashpay/platform#5928 feat(platform-wallet): report whether a failed payment may have left the wallet](https://github.com/dashpay/platform/pull/5928) — 4 unresolved (4 bot) · 5 days stale · 🐢 targets v5.2-dev · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk · Policy: waiting-author
  - Top thread: "✅ **Resolved** at \`a32de9cc\`; see the replies below." — 5 days old
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5853 feat: report a shielded identity create proven to have its inputs spent without its identity](https://github.com/dashpay/platform/pull/5853) — 1 unresolved (1 bot) · 6 days stale · 🐢 targets v5.2-dev · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, rust-sdk, kotlin-sdk, fallback · Policy: waiting-author
  - Top thread: "✅ **Withdrawn** at \`73f13d31\`; see the replies below." — 6 days old
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5536 fix!: check a DashPay invitation claim against Platform before broadcasting it](https://github.com/dashpay/platform/pull/5536) — 1 unresolved (1 bot) · 1 days stale · 🐢 targets v5.2-dev · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, rust-sdk, kotlin-sdk, sdk-parity · Policy: waiting-self-review
  - Top thread: "✅ **Resolved** at \`97875d14\`; see the replies below." — 1 days old
  - Blocker: Author must post /self-reviewed 97875d14db9d952738cc0a246806d2e259df9cf1
- [dashpay/platform#4992 feat(swift-sdk): durable wallet presence marker readable while the device is locked](https://github.com/dashpay/platform/pull/4992) — 🐢 targets v5.2-dev · areas: swift-sdk · Policy: ready-to-merge
  - Blocker: All policy requirements are satisfied
- [dashpay/platform#5587 fix(platform-wallet)!: keep a contact's payment account across relaunches and replace it cleanly on rotation](https://github.com/dashpay/platform/pull/5587) — ⚠ merge conflict · 📝 draft · 🐢 targets v5.2-dev · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5589 feat(platform-wallet-ffi)!: let hosts reset an outbound contact account's pool and keep its request digest](https://github.com/dashpay/platform/pull/5589) — 🔴 CI failing · 📝 draft · 🐢 targets fix/contact-rebuild-keeps-pool · areas: rs-platform-wallet-ffi, swift-sdk
- [dashpay/platform#5590 feat(kotlin-sdk)!: reset an outbound contact account's pool and keep its request digest on Android](https://github.com/dashpay/platform/pull/5590) — 🔴 CI failing · 📝 draft · 🐢 targets fix/contact-rebuild-ffi-swift · areas: kotlin-sdk, sdk-parity
- [dashpay/platform#5865 feat(platform-wallet): plan a shielded spend with its exact fee and spend against the plan](https://github.com/dashpay/platform/pull/5865) — 🐢 targets v5.2-dev · areas: dpp, rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, rust-sdk, kotlin-sdk, sdk-parity, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
- [dashpay/platform#6014 feat(platform-wallet)!: give identity slots one wallet-owned lease](https://github.com/dashpay/platform/pull/6014) — ⚠ merge conflict · ✋ changes requested · 📝 draft · 🐢 targets feat/invitation-claim-preflight · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, rust-sdk, kotlin-sdk, sdk-parity

<a id="romchornyi-ready-for-review"></a>
#### Ready for Review (2)
- [dashpay/platform#6245 fix(swift-example-app): let the perpetual distribution's recipient claim it](https://github.com/dashpay/platform/pull/6245) — by @QuantumExplorer · areas: swift-sdk · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head
- [dashpay/platform#6246 fix(platform-wallet): leave a contact's watch-only account out of per-account Core balances](https://github.com/dashpay/platform/pull/6246) — by @QuantumExplorer · areas: rs-platform-wallet

<a id="llbartekll"></a>
### @llbartekll
<a id="llbartekll-open"></a>
#### Open (4)
- [dashpay/platform#6162 test(platform-wallet): pin sync coordinator lifecycle behaviour](https://github.com/dashpay/platform/pull/6162) — 2 unresolved (2 bot) · 0 days stale · 🐢 targets v5.2-dev · areas: rs-platform-wallet · Policy: waiting-self-review
  - Top thread: "🟡 Suggestion: The new shielded seal test hangs if the drain ignores its deadline**" — 0 days old
  - Blocker: Author must post /self-reviewed 1a6365b7793c3ab66aaa12df0935b0960dfbd72e
- [dashpay/platform#6142 refactor(platform-wallet): parse sent contact requests with the shared parser](https://github.com/dashpay/platform/pull/6142) — 1 unresolved (1 bot) · 1 days stale · 🐢 targets v5.2-dev · areas: rs-platform-wallet · Policy: ready-to-merge
  - Top thread: "✅ **Withdrawn** at \`464116a7\`; see the replies below." — 1 days old
  - Blocker: All policy requirements are satisfied
- [dashpay/platform#6143 fix(platform-wallet): a failed local key write no longer fails an accepted identity registration](https://github.com/dashpay/platform/pull/6143) — 🐢 targets v5.2-dev · areas: rs-platform-wallet · Policy: waiting-author
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#6163 refactor(platform-wallet): share the sync loop lifecycle across coordinators](https://github.com/dashpay/platform/pull/6163) — 📝 draft · 🐢 targets test/platform-wallet-sync-coordinator-lifecycle · areas: rs-platform-wallet

<a id="llbartekll-ready-for-human"></a>
#### Ready for human (1)
- [dashpay/platform#6142 refactor(platform-wallet): parse sent contact requests with the shared parser](https://github.com/dashpay/platform/pull/6142) — 1 unresolved (1 bot) · 1 days stale · 🐢 targets v5.2-dev · areas: rs-platform-wallet · Policy: ready-to-merge
  - Top thread: "✅ **Withdrawn** at \`464116a7\`; see the replies below." — 1 days old
  - Blocker: All policy requirements are satisfied

<a id="llbartekll-stale"></a>
#### Stale (4)
- [dashpay/platform#6162 test(platform-wallet): pin sync coordinator lifecycle behaviour](https://github.com/dashpay/platform/pull/6162) — 2 unresolved (2 bot) · 0 days stale · 🐢 targets v5.2-dev · areas: rs-platform-wallet · Policy: waiting-self-review
  - Top thread: "🟡 Suggestion: The new shielded seal test hangs if the drain ignores its deadline**" — 0 days old
  - Blocker: Author must post /self-reviewed 1a6365b7793c3ab66aaa12df0935b0960dfbd72e
- [dashpay/platform#6142 refactor(platform-wallet): parse sent contact requests with the shared parser](https://github.com/dashpay/platform/pull/6142) — 1 unresolved (1 bot) · 1 days stale · 🐢 targets v5.2-dev · areas: rs-platform-wallet · Policy: ready-to-merge
  - Top thread: "✅ **Withdrawn** at \`464116a7\`; see the replies below." — 1 days old
  - Blocker: All policy requirements are satisfied
- [dashpay/platform#6143 fix(platform-wallet): a failed local key write no longer fails an accepted identity registration](https://github.com/dashpay/platform/pull/6143) — 🐢 targets v5.2-dev · areas: rs-platform-wallet · Policy: waiting-author
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#6163 refactor(platform-wallet): share the sync loop lifecycle across coordinators](https://github.com/dashpay/platform/pull/6163) — 📝 draft · 🐢 targets test/platform-wallet-sync-coordinator-lifecycle · areas: rs-platform-wallet

<a id="llbartekll-ready-for-review"></a>
#### Ready for Review (2)
- [dashpay/platform#6245 fix(swift-example-app): let the perpetual distribution's recipient claim it](https://github.com/dashpay/platform/pull/6245) — by @QuantumExplorer · areas: swift-sdk · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head
- [dashpay/platform#6246 fix(platform-wallet): leave a contact's watch-only account out of per-account Core balances](https://github.com/dashpay/platform/pull/6246) — by @QuantumExplorer · areas: rs-platform-wallet

<a id="hashengineering"></a>
### @HashEngineering
<a id="hashengineering-open"></a>
#### Open (6)
- [dashpay/platform#5931 feat(platform-wallet): top up someone else's identity from the Core balance](https://github.com/dashpay/platform/pull/5931) — 14 unresolved (14 bot) · 10 days stale · ✋ changes requested · 🔴 CI failing · 🐢 targets v5.2-dev · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage · Policy: waiting-author
  - Top thread: "✅ **Resolved** at \`1bdda16c\`; see the replies below." — 10 days old
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5933 feat(kotlin-sdk): top up someone else's identity from the Core balance](https://github.com/dashpay/platform/pull/5933) — 2 unresolved (1 CodeRabbit, 1 bot) · 10 days stale · ✋ changes requested · 📝 draft · 🐢 targets HashEngineering/platform-wallet-top-up-someone-elses-identity-fr · areas: kotlin-sdk, sdk-parity
  - Top thread: "&lt;picture&gt;&lt;img src="https://storage.googleapis.com/coderabbit_public_assets/review-comment-icons-v1/amber-16.svg" width="…" — 10 days old
- [dashpay/platform#5479 feat(platform-wallet-storage): persist DashPay backfill coverage and outbound-account markers](https://github.com/dashpay/platform/pull/5479) — ⚠ merge conflict · 🐢 targets v5.2-dev · areas: rs-platform-wallet, wallet-storage · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 8e1ee2e24549cabf2d2e81a85a02fc613212aa1f
- [dashpay/platform#5767 fix(kotlin-sdk): port #4638 to Android on the wallet's TXO reconcile policy](https://github.com/dashpay/platform/pull/5767) — 🐢 targets v5.2-dev · areas: swift-sdk, kotlin-sdk, github, fallback · Policy: waiting-author
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5906 fix(kotlin-sdk): refuse address types wider than a byte and document the address top-up outcomes](https://github.com/dashpay/platform/pull/5906) — ✋ changes requested · 🐢 targets v5.2-dev · areas: kotlin-sdk · Policy: waiting-author
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#6242 fix(kotlin-sdk): reject malformed UTF-16 string arguments at the JNI boundary](https://github.com/dashpay/platform/pull/6242) — 📝 draft · 🐢 targets v5.2-dev · areas: kotlin-sdk · Policy: draft
  - Blocker: Draft PR does not occupy a review slot

<a id="hashengineering-stale"></a>
#### Stale (6)
- [dashpay/platform#5931 feat(platform-wallet): top up someone else's identity from the Core balance](https://github.com/dashpay/platform/pull/5931) — 14 unresolved (14 bot) · 10 days stale · ✋ changes requested · 🔴 CI failing · 🐢 targets v5.2-dev · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage · Policy: waiting-author
  - Top thread: "✅ **Resolved** at \`1bdda16c\`; see the replies below." — 10 days old
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5933 feat(kotlin-sdk): top up someone else's identity from the Core balance](https://github.com/dashpay/platform/pull/5933) — 2 unresolved (1 CodeRabbit, 1 bot) · 10 days stale · ✋ changes requested · 📝 draft · 🐢 targets HashEngineering/platform-wallet-top-up-someone-elses-identity-fr · areas: kotlin-sdk, sdk-parity
  - Top thread: "&lt;picture&gt;&lt;img src="https://storage.googleapis.com/coderabbit_public_assets/review-comment-icons-v1/amber-16.svg" width="…" — 10 days old
- [dashpay/platform#5479 feat(platform-wallet-storage): persist DashPay backfill coverage and outbound-account markers](https://github.com/dashpay/platform/pull/5479) — ⚠ merge conflict · 🐢 targets v5.2-dev · areas: rs-platform-wallet, wallet-storage · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 8e1ee2e24549cabf2d2e81a85a02fc613212aa1f
- [dashpay/platform#5767 fix(kotlin-sdk): port #4638 to Android on the wallet's TXO reconcile policy](https://github.com/dashpay/platform/pull/5767) — 🐢 targets v5.2-dev · areas: swift-sdk, kotlin-sdk, github, fallback · Policy: waiting-author
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5906 fix(kotlin-sdk): refuse address types wider than a byte and document the address top-up outcomes](https://github.com/dashpay/platform/pull/5906) — ✋ changes requested · 🐢 targets v5.2-dev · areas: kotlin-sdk · Policy: waiting-author
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#6242 fix(kotlin-sdk): reject malformed UTF-16 string arguments at the JNI boundary](https://github.com/dashpay/platform/pull/6242) — 📝 draft · 🐢 targets v5.2-dev · areas: kotlin-sdk · Policy: draft
  - Blocker: Draft PR does not occupy a review slot

<a id="hashengineering-ready-for-review"></a>
#### Ready for Review (1)
- [dashpay/platform#6246 fix(platform-wallet): leave a contact's watch-only account out of per-account Core balances](https://github.com/dashpay/platform/pull/6246) — by @QuantumExplorer · areas: rs-platform-wallet

<a id="ktechmidas"></a>
### @ktechmidas
<a id="ktechmidas-open"></a>
#### Open (6)
- [dashpay/platform#5188 ci: tolerate optional S3 cache export outages on chore/bump-rust-dashcore-secp-033](https://github.com/dashpay/platform/pull/5188) — 1 unresolved (1 bot) · 56 days stale · 🐢 targets chore/bump-rust-dashcore-secp-033 · areas: github
  - Top thread: "🟡 Suggestion: New regression tests are not executed by any CI workflow on this branch**" — 56 days old
- [dashpay/platform#5186 ci: tolerate optional S3 cache export outages on v5.0-dev](https://github.com/dashpay/platform/pull/5186) — 🔴 CI failing · 🐢 targets v6.0-dev · areas: github · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 843052c2dce26a80065c15b1219bbc8211264161
- [dashpay/platform#5198 ci: bound default Binaryen optimizer threads in NPM builds](https://github.com/dashpay/platform/pull/5198) — 🐢 targets v6.0-dev · areas: github · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 23b41b477bc038965b686b4a75412b1c21fc6d98
- [dashpay/platform#5205 fix(release): install verified cargo-binstall binary directly](https://github.com/dashpay/platform/pull/5205) — 🐢 targets v6.0-dev · areas: fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 8a8ac2c788b5338416e18045001d9115ba3ac80a
- [dashpay/platform#5866 feat(dashmate): clean up historical Docker images after updates](https://github.com/dashpay/platform/pull/5866) — ✋ changes requested · 🔴 CI failing · 🐢 targets v5.2-dev · areas: dashmate · Policy: waiting-author
  - Blocker: Author response is required after the latest human objection
- [dashpay/platform#6018 ci: move CI Docker images from ECR to GHCR (saves about $265 a month)](https://github.com/dashpay/platform/pull/6018) — 🔴 CI failing · 🐢 targets v5.2-dev · areas: github, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 6e27cce9fd48813d3f356c5f34e70b30652dae87

<a id="ktechmidas-stale"></a>
#### Stale (6)
- [dashpay/platform#5188 ci: tolerate optional S3 cache export outages on chore/bump-rust-dashcore-secp-033](https://github.com/dashpay/platform/pull/5188) — 1 unresolved (1 bot) · 56 days stale · 🐢 targets chore/bump-rust-dashcore-secp-033 · areas: github
  - Top thread: "🟡 Suggestion: New regression tests are not executed by any CI workflow on this branch**" — 56 days old
- [dashpay/platform#5186 ci: tolerate optional S3 cache export outages on v5.0-dev](https://github.com/dashpay/platform/pull/5186) — 🔴 CI failing · 🐢 targets v6.0-dev · areas: github · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 843052c2dce26a80065c15b1219bbc8211264161
- [dashpay/platform#5198 ci: bound default Binaryen optimizer threads in NPM builds](https://github.com/dashpay/platform/pull/5198) — 🐢 targets v6.0-dev · areas: github · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 23b41b477bc038965b686b4a75412b1c21fc6d98
- [dashpay/platform#5205 fix(release): install verified cargo-binstall binary directly](https://github.com/dashpay/platform/pull/5205) — 🐢 targets v6.0-dev · areas: fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 8a8ac2c788b5338416e18045001d9115ba3ac80a
- [dashpay/platform#5866 feat(dashmate): clean up historical Docker images after updates](https://github.com/dashpay/platform/pull/5866) — ✋ changes requested · 🔴 CI failing · 🐢 targets v5.2-dev · areas: dashmate · Policy: waiting-author
  - Blocker: Author response is required after the latest human objection
- [dashpay/platform#6018 ci: move CI Docker images from ECR to GHCR (saves about $265 a month)](https://github.com/dashpay/platform/pull/6018) — 🔴 CI failing · 🐢 targets v5.2-dev · areas: github, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 6e27cce9fd48813d3f356c5f34e70b30652dae87

<a id="ktechmidas-ready-for-review"></a>
#### Ready for Review (1)
- [dashpay/rust-dashcore#1490 ci: test only the crates a pull request changes](https://github.com/dashpay/rust-dashcore/pull/1490) — by @ZocoLini · areas: github, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 57c19ebefff21e528e338b7cc11d43d483fc8012

<a id="zocolini"></a>
### @ZocoLini
<a id="zocolini-open"></a>
#### Open (9)
- [dashpay/rust-dashcore#1199 feat(key-wallet)!: only final coins fund a build](https://github.com/dashpay/rust-dashcore/pull/1199) — ⚠ merge conflict · 📝 draft · areas: dash-spv, key-wallet, key-wallet-manager · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#1328 refactor(key-wallet)!: validate child indexes at construction with \`ChildIndex\`](https://github.com/dashpay/rust-dashcore/pull/1328) — areas: dash-spv, key-wallet, key-wallet-manager · Policy: waiting-bots
  - Blocker: coderabbitai has not reported for the current head
- [dashpay/rust-dashcore#1398 fix(dash-spv): detect forks with a full block locator](https://github.com/dashpay/rust-dashcore/pull/1398) — 🔴 CI failing · areas: dash-spv · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 9475a9f8360b0a04bf435f1098250bc6302a7d64
- [dashpay/rust-dashcore#1421 fix(dash-spv): propagate storage errors from sync paths](https://github.com/dashpay/rust-dashcore/pull/1421) — 📝 draft · 🐢 targets refactor/storage-error-variants · areas: dash-spv
- [dashpay/rust-dashcore#1426 fix(dash-spv): tell an empty filter header store from genesis](https://github.com/dashpay/rust-dashcore/pull/1426) — areas: dash-spv · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed d31ec981450f9216f70fd0a3877e22563060db0c
- [dashpay/rust-dashcore#1459 ci: merge audit and verify jobs into one](https://github.com/dashpay/rust-dashcore/pull/1459) — ⚠ merge conflict · 📝 draft · areas: github · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#1490 ci: test only the crates a pull request changes](https://github.com/dashpay/rust-dashcore/pull/1490) — areas: github, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 57c19ebefff21e528e338b7cc11d43d483fc8012
- [dashpay/rust-dashcore#1502 refactor(dash-spv)!: keep only storage failures in \`StorageError\`](https://github.com/dashpay/rust-dashcore/pull/1502) — areas: dash-spv · Policy: waiting-bots
  - Blocker: coderabbitai has not reported for the current head
- [dashpay/rust-dashcore#1503 test(dash-spv): corrupt every storage part in the resync tests](https://github.com/dashpay/rust-dashcore/pull/1503) — 🔴 CI failing · areas: dash-spv · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 34fa90bbe7fac9a6db3dcd04b4cd7a61c6e54738

<a id="zocolini-ci-failing"></a>
#### CI Failing (2)
- [dashpay/rust-dashcore#1398 fix(dash-spv): detect forks with a full block locator](https://github.com/dashpay/rust-dashcore/pull/1398) — 🔴 CI failing · areas: dash-spv · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 9475a9f8360b0a04bf435f1098250bc6302a7d64
- [dashpay/rust-dashcore#1503 test(dash-spv): corrupt every storage part in the resync tests](https://github.com/dashpay/rust-dashcore/pull/1503) — 🔴 CI failing · areas: dash-spv · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 34fa90bbe7fac9a6db3dcd04b4cd7a61c6e54738

<a id="zocolini-draft"></a>
#### Draft (2)
- [dashpay/rust-dashcore#1199 feat(key-wallet)!: only final coins fund a build](https://github.com/dashpay/rust-dashcore/pull/1199) — ⚠ merge conflict · 📝 draft · areas: dash-spv, key-wallet, key-wallet-manager · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#1459 ci: merge audit and verify jobs into one](https://github.com/dashpay/rust-dashcore/pull/1459) — ⚠ merge conflict · 📝 draft · areas: github · Policy: draft
  - Blocker: Draft PR does not occupy a review slot

<a id="zocolini-stale"></a>
#### Stale (1)
- [dashpay/rust-dashcore#1421 fix(dash-spv): propagate storage errors from sync paths](https://github.com/dashpay/rust-dashcore/pull/1421) — 📝 draft · 🐢 targets refactor/storage-error-variants · areas: dash-spv

<a id="zocolini-clean"></a>
#### Clean (4)
- [dashpay/rust-dashcore#1328 refactor(key-wallet)!: validate child indexes at construction with \`ChildIndex\`](https://github.com/dashpay/rust-dashcore/pull/1328) — areas: dash-spv, key-wallet, key-wallet-manager · Policy: waiting-bots
  - Blocker: coderabbitai has not reported for the current head
- [dashpay/rust-dashcore#1426 fix(dash-spv): tell an empty filter header store from genesis](https://github.com/dashpay/rust-dashcore/pull/1426) — areas: dash-spv · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed d31ec981450f9216f70fd0a3877e22563060db0c
- [dashpay/rust-dashcore#1490 ci: test only the crates a pull request changes](https://github.com/dashpay/rust-dashcore/pull/1490) — areas: github, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 57c19ebefff21e528e338b7cc11d43d483fc8012
- [dashpay/rust-dashcore#1502 refactor(dash-spv)!: keep only storage failures in \`StorageError\`](https://github.com/dashpay/rust-dashcore/pull/1502) — areas: dash-spv · Policy: waiting-bots
  - Blocker: coderabbitai has not reported for the current head

<a id="zocolini-ready-for-review"></a>
#### Ready for Review (1)
- [dashpay/platform#6246 fix(platform-wallet): leave a contact's watch-only account out of per-account Core balances](https://github.com/dashpay/platform/pull/6246) — by @QuantumExplorer · areas: rs-platform-wallet

<a id="pastapastapasta"></a>
### @PastaPastaPasta
<a id="pastapastapasta-open"></a>
#### Open (20)
- [dashpay/platform#4648 feat(drive-abci)!: state sync via ABCI snapshots with reduced platform state (protocol v15)](https://github.com/dashpay/platform/pull/4648) — 8 unresolved (8 bot) · 50 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · 📝 draft · 🐢 targets v5.2-dev · areas: rs-drive, rs-drive-abci, dpp, fallback · Policy: draft
  - Top thread: "🟡 Suggestion: Continue the restored target through consensus and a restart**" — 50 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5280 perf(platform-wallet)!: reuse a recent recorded-anchor set on the shielded spend path](https://github.com/dashpay/platform/pull/5280) — 7 unresolved (7 bot) · 4 days stale · ✋ changes requested · 🐢 targets v5.2-dev · areas: rs-platform-wallet, rust-sdk, rust-sdk-ffi, fallback · Policy: waiting-author
  - Top thread: "✅ **Resolved** at \`327654fc\`; see the replies below." — 4 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5786 feat(sdk): tri-state quorum lookups, bounded funding retries and a DPNS contest policy in dash-sdk-cxx](https://github.com/dashpay/platform/pull/5786) — 3 unresolved (3 bot) · 17 days stale · ⚠ merge conflict · 📝 draft · 🐢 targets v5.2-dev · areas: rs-drive, rs-platform-wallet, rust-sdk, js-wasm-sdk, fallback · Policy: draft
  - Top thread: "✅ **Resolved** at \`e46d43eb\`; see the replies below." — 17 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#4965 test(sdk): cargo-fuzz harness over drive-proof-verifier FromProof](https://github.com/dashpay/platform/pull/4965) — 1 unresolved (1 bot) · 49 days stale · 📝 draft · 🐢 targets v5.2-dev · areas: github, fallback · Policy: draft
  - Top thread: "🟡 Suggestion: Correct the claim that PV13 reaches the V0 proof decoder**" — 49 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5934 fix(platform-wallet): own the shielded coordinator and wipe host rows inside Clear](https://github.com/dashpay/platform/pull/5934) — 1 unresolved (1 bot) · 9 days stale · 🐢 targets v5.2-dev · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk, kotlin-sdk · Policy: ready-for-human
  - Top thread: "⚖️ **Disputed** (left to the maintainers; not blocking) at \`36e55493\`; see the replies below." — 9 days old
  - Blocker: Human approval or objection resolution is required
- [dashpay/grovedb#1007 build(deps)!: use dashpay/orchard with the faster prover](https://github.com/dashpay/grovedb/pull/1007) — 🔴 CI failing · 📝 draft · areas: fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/grovedb#1054 feat: LigninDB storage engine (tracking, do not merge)](https://github.com/dashpay/grovedb/pull/1054) — 🔴 CI failing · 📝 draft · areas: github, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5277 perf(platform-wallet): don't hold the shielded store lock across sync network I/O](https://github.com/dashpay/platform/pull/5277) — 📝 draft · 🐢 targets v5.2-dev · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5282 perf(platform-wallet)!: prove the shield bundle while waiting for the InstantSend lock](https://github.com/dashpay/platform/pull/5282) — ✋ changes requested · 🐢 targets v5.2-dev · areas: dpp, rs-platform-wallet, rs-platform-wallet-ffi, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed e54ab798399565b26a78105f50d419cf7903c3b0
- [dashpay/platform#5285 feat(platform)!: subscribe to committed state transitions matching document and address filters](https://github.com/dashpay/platform/pull/5285) — ⚠ merge conflict · 📝 draft · 🐢 targets v5.2-dev · areas: rs-drive, rs-drive-abci, rust-sdk, dashmate, rust-dapi, github, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5289 feat(sdk)!: optional BIP-39 passphrase through the mnemonic resolver and wallet creation](https://github.com/dashpay/platform/pull/5289) — ⚠ merge conflict · 📝 draft · 🐢 targets v5.2-dev · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, rust-sdk-ffi, kotlin-sdk · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5292 feat(dashmate): state sync configuration for tenderdash and drive snapshots](https://github.com/dashpay/platform/pull/5292) — ⚠ merge conflict · 📝 draft · 🐢 targets v5.2-dev · areas: dashmate · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5475 feat(sdk): forward DashPay profiles, invitations and masternode voting in dash-sdk-cxx](https://github.com/dashpay/platform/pull/5475) — ⚠ merge conflict · 📝 draft · 🐢 targets v5.2-dev · areas: rs-platform-wallet, rs-platform-wallet-ffi, rust-sdk, github, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5577 feat: gzip-compress DAPI responses](https://github.com/dashpay/platform/pull/5577) — 🐢 targets v5.2-dev · areas: dashmate, rust-dapi, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed bc43fe7046ac860cc8396910e15b734cc8d4b55c
- [dashpay/platform#5610 perf(sdk): verify each block's signature once instead of on every proved response](https://github.com/dashpay/platform/pull/5610) — ⚠ merge conflict · 📝 draft · 🐢 targets v5.2-dev · areas: fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5784 fix(platform-wallet): export AssetLockBuildAmount](https://github.com/dashpay/platform/pull/5784) — 📝 draft · 🐢 targets v5.2-dev · areas: rs-platform-wallet · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5792 fix(platform-wallet): persist coins a block's re-scan credits](https://github.com/dashpay/platform/pull/5792) — ⚠ merge conflict · 📝 draft · 🐢 targets v5.2-dev · areas: rs-platform-wallet, wallet-storage · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5802 fix(platform-wallet-storage): don't look for or count non-funds used addresses as unresolved](https://github.com/dashpay/platform/pull/5802) — ⚠ merge conflict · 📝 draft · 🐢 targets v5.2-dev · areas: wallet-storage, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#6042 feat(platform-wallet-ffi): prefetch shielded spend anchors when a send screen opens](https://github.com/dashpay/platform/pull/6042) — 🔴 CI failing · 📝 draft · 🐢 targets perf/shielded-anchor-cache · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, fallback
- [dashpay/platform#6241 fix(swift-sdk): preserve spv progress read provenance](https://github.com/dashpay/platform/pull/6241) — ✋ changes requested · 🐢 targets v5.2-dev · areas: swift-sdk · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head

<a id="pastapastapasta-ready-for-human"></a>
#### Ready for human (1)
- [dashpay/platform#5934 fix(platform-wallet): own the shielded coordinator and wipe host rows inside Clear](https://github.com/dashpay/platform/pull/5934) — 1 unresolved (1 bot) · 9 days stale · 🐢 targets v5.2-dev · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk, kotlin-sdk · Policy: ready-for-human
  - Top thread: "⚖️ **Disputed** (left to the maintainers; not blocking) at \`36e55493\`; see the replies below." — 9 days old
  - Blocker: Human approval or objection resolution is required

<a id="pastapastapasta-draft"></a>
#### Draft (2)
- [dashpay/grovedb#1007 build(deps)!: use dashpay/orchard with the faster prover](https://github.com/dashpay/grovedb/pull/1007) — 🔴 CI failing · 📝 draft · areas: fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/grovedb#1054 feat: LigninDB storage engine (tracking, do not merge)](https://github.com/dashpay/grovedb/pull/1054) — 🔴 CI failing · 📝 draft · areas: github, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot

<a id="pastapastapasta-stale"></a>
#### Stale (18)
- [dashpay/platform#4648 feat(drive-abci)!: state sync via ABCI snapshots with reduced platform state (protocol v15)](https://github.com/dashpay/platform/pull/4648) — 8 unresolved (8 bot) · 50 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · 📝 draft · 🐢 targets v5.2-dev · areas: rs-drive, rs-drive-abci, dpp, fallback · Policy: draft
  - Top thread: "🟡 Suggestion: Continue the restored target through consensus and a restart**" — 50 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5280 perf(platform-wallet)!: reuse a recent recorded-anchor set on the shielded spend path](https://github.com/dashpay/platform/pull/5280) — 7 unresolved (7 bot) · 4 days stale · ✋ changes requested · 🐢 targets v5.2-dev · areas: rs-platform-wallet, rust-sdk, rust-sdk-ffi, fallback · Policy: waiting-author
  - Top thread: "✅ **Resolved** at \`327654fc\`; see the replies below." — 4 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5786 feat(sdk): tri-state quorum lookups, bounded funding retries and a DPNS contest policy in dash-sdk-cxx](https://github.com/dashpay/platform/pull/5786) — 3 unresolved (3 bot) · 17 days stale · ⚠ merge conflict · 📝 draft · 🐢 targets v5.2-dev · areas: rs-drive, rs-platform-wallet, rust-sdk, js-wasm-sdk, fallback · Policy: draft
  - Top thread: "✅ **Resolved** at \`e46d43eb\`; see the replies below." — 17 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#4965 test(sdk): cargo-fuzz harness over drive-proof-verifier FromProof](https://github.com/dashpay/platform/pull/4965) — 1 unresolved (1 bot) · 49 days stale · 📝 draft · 🐢 targets v5.2-dev · areas: github, fallback · Policy: draft
  - Top thread: "🟡 Suggestion: Correct the claim that PV13 reaches the V0 proof decoder**" — 49 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5934 fix(platform-wallet): own the shielded coordinator and wipe host rows inside Clear](https://github.com/dashpay/platform/pull/5934) — 1 unresolved (1 bot) · 9 days stale · 🐢 targets v5.2-dev · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk, kotlin-sdk · Policy: ready-for-human
  - Top thread: "⚖️ **Disputed** (left to the maintainers; not blocking) at \`36e55493\`; see the replies below." — 9 days old
  - Blocker: Human approval or objection resolution is required
- [dashpay/platform#5277 perf(platform-wallet): don't hold the shielded store lock across sync network I/O](https://github.com/dashpay/platform/pull/5277) — 📝 draft · 🐢 targets v5.2-dev · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5282 perf(platform-wallet)!: prove the shield bundle while waiting for the InstantSend lock](https://github.com/dashpay/platform/pull/5282) — ✋ changes requested · 🐢 targets v5.2-dev · areas: dpp, rs-platform-wallet, rs-platform-wallet-ffi, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed e54ab798399565b26a78105f50d419cf7903c3b0
- [dashpay/platform#5285 feat(platform)!: subscribe to committed state transitions matching document and address filters](https://github.com/dashpay/platform/pull/5285) — ⚠ merge conflict · 📝 draft · 🐢 targets v5.2-dev · areas: rs-drive, rs-drive-abci, rust-sdk, dashmate, rust-dapi, github, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5289 feat(sdk)!: optional BIP-39 passphrase through the mnemonic resolver and wallet creation](https://github.com/dashpay/platform/pull/5289) — ⚠ merge conflict · 📝 draft · 🐢 targets v5.2-dev · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, rust-sdk-ffi, kotlin-sdk · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5292 feat(dashmate): state sync configuration for tenderdash and drive snapshots](https://github.com/dashpay/platform/pull/5292) — ⚠ merge conflict · 📝 draft · 🐢 targets v5.2-dev · areas: dashmate · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5475 feat(sdk): forward DashPay profiles, invitations and masternode voting in dash-sdk-cxx](https://github.com/dashpay/platform/pull/5475) — ⚠ merge conflict · 📝 draft · 🐢 targets v5.2-dev · areas: rs-platform-wallet, rs-platform-wallet-ffi, rust-sdk, github, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5577 feat: gzip-compress DAPI responses](https://github.com/dashpay/platform/pull/5577) — 🐢 targets v5.2-dev · areas: dashmate, rust-dapi, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed bc43fe7046ac860cc8396910e15b734cc8d4b55c
- [dashpay/platform#5610 perf(sdk): verify each block's signature once instead of on every proved response](https://github.com/dashpay/platform/pull/5610) — ⚠ merge conflict · 📝 draft · 🐢 targets v5.2-dev · areas: fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5784 fix(platform-wallet): export AssetLockBuildAmount](https://github.com/dashpay/platform/pull/5784) — 📝 draft · 🐢 targets v5.2-dev · areas: rs-platform-wallet · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5792 fix(platform-wallet): persist coins a block's re-scan credits](https://github.com/dashpay/platform/pull/5792) — ⚠ merge conflict · 📝 draft · 🐢 targets v5.2-dev · areas: rs-platform-wallet, wallet-storage · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5802 fix(platform-wallet-storage): don't look for or count non-funds used addresses as unresolved](https://github.com/dashpay/platform/pull/5802) — ⚠ merge conflict · 📝 draft · 🐢 targets v5.2-dev · areas: wallet-storage, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#6042 feat(platform-wallet-ffi): prefetch shielded spend anchors when a send screen opens](https://github.com/dashpay/platform/pull/6042) — 🔴 CI failing · 📝 draft · 🐢 targets perf/shielded-anchor-cache · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, fallback
- [dashpay/platform#6241 fix(swift-sdk): preserve spv progress read provenance](https://github.com/dashpay/platform/pull/6241) — ✋ changes requested · 🐢 targets v5.2-dev · areas: swift-sdk · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head

<a id="bfoss765"></a>
### @bfoss765
<a id="bfoss765-open"></a>
#### Open (2)
- [dashpay/platform#4313 feat(kotlin-sdk): one-time Orchard key shielded-invite API (inviter + claim)](https://github.com/dashpay/platform/pull/4313) — 6 unresolved (6 bot) · 85 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · 🐢 targets v5.2-dev · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk · Policy: waiting-author
  - Top thread: "✅ **Resolved** at \`eccc65fa\`; see the replies below." — 85 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#4312 feat(platform-wallet): multi-output shielded transfers + output-aware fee predictor](https://github.com/dashpay/platform/pull/4312) — 1 unresolved (1 bot) · 85 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · 🐢 targets v5.2-dev · areas: dpp, rs-platform-wallet, rs-platform-wallet-ffi, kotlin-sdk, fallback · Policy: waiting-self-review
  - Top thread: "🟡 Suggestion: Keep envelope-aware action counting behind the crate boundary**" — 85 days old
  - Blocker: Author must post /self-reviewed da932f96a8670c641065954de8b58752e99233a3

<a id="bfoss765-stale"></a>
#### Stale (2)
- [dashpay/platform#4313 feat(kotlin-sdk): one-time Orchard key shielded-invite API (inviter + claim)](https://github.com/dashpay/platform/pull/4313) — 6 unresolved (6 bot) · 85 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · 🐢 targets v5.2-dev · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk · Policy: waiting-author
  - Top thread: "✅ **Resolved** at \`eccc65fa\`; see the replies below." — 85 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#4312 feat(platform-wallet): multi-output shielded transfers + output-aware fee predictor](https://github.com/dashpay/platform/pull/4312) — 1 unresolved (1 bot) · 85 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · 🐢 targets v5.2-dev · areas: dpp, rs-platform-wallet, rs-platform-wallet-ffi, kotlin-sdk, fallback · Policy: waiting-self-review
  - Top thread: "🟡 Suggestion: Keep envelope-aware action counting behind the crate boundary**" — 85 days old
  - Blocker: Author must post /self-reviewed da932f96a8670c641065954de8b58752e99233a3

<a id="thephez"></a>
### @thephez
<a id="thephez-open"></a>
#### Open (1)
- [dashpay/platform#6167 build(wasm-sdk)!: cut release WASM size 42% with LTO, low-memory secp256k1 and no default mocks](https://github.com/dashpay/platform/pull/6167) — 1 unresolved (1 bot) · 0 days stale · 🐢 targets v5.2-dev · areas: rust-sdk, js-wasm-sdk, github, fallback · Policy: waiting-bots
  - Top thread: "✅ **Resolved** at \`ac0ec1b4\`; see the replies below." — 0 days old
  - Blocker: thepastaclaw has not reported for the current head

<a id="thephez-stale"></a>
#### Stale (1)
- [dashpay/platform#6167 build(wasm-sdk)!: cut release WASM size 42% with LTO, low-memory secp256k1 and no default mocks](https://github.com/dashpay/platform/pull/6167) — 1 unresolved (1 bot) · 0 days stale · 🐢 targets v5.2-dev · areas: rust-sdk, js-wasm-sdk, github, fallback · Policy: waiting-bots
  - Top thread: "✅ **Resolved** at \`ac0ec1b4\`; see the replies below." — 0 days old
  - Blocker: thepastaclaw has not reported for the current head

<a id="dcg-claude"></a>
### @DCG-Claude
<a id="dcg-claude-open"></a>
#### Open (3)
- [dashpay/platform#4794 feat: pin contract configuration vectors and mirror them in wasm-dpp2 and the sdk ffi](https://github.com/dashpay/platform/pull/4794) — 🔴 CI failing · 🐢 targets v5.2-dev · areas: dpp, rust-sdk-ffi, fallback · Policy: waiting-build
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

<a id="dcg-claude-stale"></a>
#### Stale (3)
- [dashpay/platform#4794 feat: pin contract configuration vectors and mirror them in wasm-dpp2 and the sdk ffi](https://github.com/dashpay/platform/pull/4794) — 🔴 CI failing · 🐢 targets v5.2-dev · areas: dpp, rust-sdk-ffi, fallback · Policy: waiting-build
  - Blocker: The build must pass before a human is asked
- [dashpay/platform#4934 docs: correct the smart-contract target, version counts, transition list and mobile sdk availability](https://github.com/dashpay/platform/pull/4934) — 🐢 targets v6.0-dev · areas: swift-sdk, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
- [dashpay/platform#4938 docs(platform): distinguish current-state proofs, stored receipts and execution result text](https://github.com/dashpay/platform/pull/4938) — 🐢 targets v6.0-dev · areas: dpp, rust-sdk, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required

<a id="thepastaclaw"></a>
### @thepastaclaw
<a id="thepastaclaw-open"></a>
#### Open (1)
- [dashpay/platform#5581 feat(platform): add proof-backed asset lock outpoint query](https://github.com/dashpay/platform/pull/5581) — 🔴 CI failing · 🐢 targets v5.2-dev · areas: rs-drive, rs-drive-abci, dpp, rs-platform-wallet, rust-sdk, rust-dapi, sdk-parity, fallback · Policy: waiting-author
  - Blocker: coderabbitai left review threads unresolved; resolve them
  - Blocker: thepastaclaw left review threads unresolved; resolve them

<a id="thepastaclaw-stale"></a>
#### Stale (1)
- [dashpay/platform#5581 feat(platform): add proof-backed asset lock outpoint query](https://github.com/dashpay/platform/pull/5581) — 🔴 CI failing · 🐢 targets v5.2-dev · areas: rs-drive, rs-drive-abci, dpp, rs-platform-wallet, rust-sdk, rust-dapi, sdk-parity, fallback · Policy: waiting-author
  - Blocker: coderabbitai left review threads unresolved; resolve them
  - Blocker: thepastaclaw left review threads unresolved; resolve them

## Methodology
Generated nightly by [pr-hygiene](https://github.com/dashpay/stale_prs_are_bad). A thread counts as "unresolved" when it is open, not outdated, has a comment from someone other than the PR author, and the most recent comment is from a reviewer. **Dirty** = at least one such thread. **Unresolved Comments** = at least one such thread. **Changes Requested** = no unresolved threads but a reviewer's most recent review is CHANGES_REQUESTED (still blocking until someone re-approves or dismisses). **Deferred** = carries a configured deferred label (e.g. `postponed`) — visible but not counted toward unresolved-comment counts. **Stale** = targets a non-default branch OR hasn't been touched in the configured threshold (default 120 days, but clean PRs are never reclassified as stale). **Draft** = the PR is still marked draft on GitHub. **CI failing** = no unresolved comments, no changes-requested, but the latest commit's status check is failing. **Clean** = open, not draft, not deferred, not stale, no unresolved comments, no changes-requested, CI green. **Needs action** further requires changes-requested, merge conflict, or that the reviewer commented more recently than the author last pushed. **Ready for human** counts a person's own PRs that the shared review engine marks `ready-for-human` or `ready-to-merge`; each PR bullet shows the engine's state as `Policy:` with its blockers. **Ready for Review** counts clean PRs (authored by someone else) where this person owes a review: the union of the shared policy's routing (owners and reviewers of every area the changed files fall into, or the repository fallback for files no area claims) and GitHub's explicit review requests. The author is never routed to their own PR, and anyone who has already submitted any review is excluded — their job is done. `⚠ ownership unresolved` marks areas whose roster the policy still lists as open. Configurable via [`https://github.com/dashpay/stale_prs_are_bad/blob/master/.pr-hygiene.yml`](https://github.com/dashpay/stale_prs_are_bad/blob/master/.pr-hygiene.yml)—edit defaults there; ownership lives in `policies/`.

_No history snapshot from last week was found, so week-over-week deltas are omitted this run._
