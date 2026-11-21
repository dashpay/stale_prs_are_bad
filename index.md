---
---
# PR Hygiene Report
*Last updated: 2026-11-21 06:12 UTC · commit 80ff9f3*

## Summary
- Open PRs: **178** (6 clean · 4 CI failing · 3 changes requested · 17 unresolved comments · 5 deferred · 21 draft · 122 stale)
- PRs needing author action: **20**
- Total unresolved comments: **370**
- dashpay/platform: **110** open (1 clean · 1 CI failing · 1 changes requested · 4 unresolved comments · 5 deferred · 1 draft · 97 stale) · engine: 24 draft · 5 ready-for-human · 3 ready-to-merge · 1 too-many-open-prs · 33 waiting-author · 1 waiting-bots · 1 waiting-build · 12 waiting-self-review · 30 no verdict
- dashpay/rust-dashcore: **23** open (4 clean · 1 CI failing · 2 changes requested · 3 unresolved comments · 0 deferred · 7 draft · 6 stale) · engine: 7 draft · 3 waiting-author · 1 waiting-build · 6 waiting-self-review · 6 no verdict
- dashpay/tenderdash: **9** open (1 clean · 0 CI failing · 0 changes requested · 4 unresolved comments · 0 deferred · 4 draft · 0 stale) · engine: 4 draft · 3 waiting-author · 2 waiting-self-review
- dashpay/grovedb: **2** open (0 clean · 0 CI failing · 0 changes requested · 0 unresolved comments · 0 deferred · 2 draft · 0 stale) · engine: 2 draft
- dashpay/dash-evo-tool: **34** open (0 clean · 2 CI failing · 0 changes requested · 6 unresolved comments · 0 deferred · 7 draft · 19 stale) · engine: 7 draft · 4 waiting-author · 4 waiting-bots · 19 no verdict

## Scoreboard
_Sort: unresolved-comments desc → needs-action desc → ready-for-review desc. Click any number to jump to the specific PRs it covers._

| Author | Open | Clean | CI failing | Unresolved Comments | Changes Requested | Deferred | Draft | Stale | Needs action | Ready for human | Total Unresolved Comments | Ready for Review | Δ |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| [@Claudius-Maginificent](#claudius-maginificent) | [58](#claudius-maginificent-open) | — | [2](#claudius-maginificent-ci-failing) | [7](#claudius-maginificent-unresolved-comments) | — | [3](#claudius-maginificent-deferred) | [8](#claudius-maginificent-draft) | [38](#claudius-maginificent-stale) | [7](#claudius-maginificent-needs-action) | [1](#claudius-maginificent-ready-for-human) | [104](#claudius-maginificent-unresolved-comments) | — | ↓ 2 |
| [@QuantumExplorer](#quantumexplorer) | [24](#quantumexplorer-open) | [2](#quantumexplorer-clean) | — | [6](#quantumexplorer-unresolved-comments) | — | — | [2](#quantumexplorer-draft) | [14](#quantumexplorer-stale) | [6](#quantumexplorer-needs-action) | — | [56](#quantumexplorer-unresolved-comments) | [3](#quantumexplorer-ready-for-review) | ↓ 2 |
| [@lklimek](#lklimek) | [12](#lklimek-open) | — | — | [1](#lklimek-unresolved-comments) | [2](#lklimek-changes-requested) | [2](#lklimek-deferred) | [3](#lklimek-draft) | [4](#lklimek-stale) | [3](#lklimek-needs-action) | [2](#lklimek-ready-for-human) | [25](#lklimek-unresolved-comments) | — | ↓ 1 |
| [@shumkov](#shumkov) | [6](#shumkov-open) | — | — | [1](#shumkov-unresolved-comments) | [1](#shumkov-changes-requested) | — | — | [4](#shumkov-stale) | [2](#shumkov-needs-action) | — | [21](#shumkov-unresolved-comments) | [5](#shumkov-ready-for-review) | ↓ 3 |
| [@xdustinface](#xdustinface) | [10](#xdustinface-open) | [1](#xdustinface-clean) | — | [1](#xdustinface-unresolved-comments) | — | — | [2](#xdustinface-draft) | [6](#xdustinface-stale) | [1](#xdustinface-needs-action) | — | [3](#xdustinface-unresolved-comments) | [3](#xdustinface-ready-for-review) | ↓ 1 |
| [@ZocoLini](#zocolini) | [8](#zocolini-open) | [3](#zocolini-clean) | — | [1](#zocolini-unresolved-comments) | — | — | [4](#zocolini-draft) | — | [1](#zocolini-needs-action) | — | [3](#zocolini-unresolved-comments) | [1](#zocolini-ready-for-review) | ↓ 1 |
| [@ktechmidas](#ktechmidas) | [6](#ktechmidas-open) | — | — | — | — | — | — | [6](#ktechmidas-stale) | — | — | [6](#ktechmidas-unresolved-comments) | [4](#ktechmidas-ready-for-review) | ↓ 1 |
| [@PastaPastaPasta](#pastapastapasta) | [24](#pastapastapasta-open) | — | [1](#pastapastapasta-ci-failing) | — | — | — | [2](#pastapastapasta-draft) | [21](#pastapastapasta-stale) | — | [1](#pastapastapasta-ready-for-human) | [53](#pastapastapasta-unresolved-comments) | — | ↓ 6 |
| [@romchornyi](#romchornyi) | [15](#romchornyi-open) | — | — | — | — | — | — | [15](#romchornyi-stale) | — | [1](#romchornyi-ready-for-human) | [67](#romchornyi-unresolved-comments) | — | ↓ 5 |
| [@HashEngineering](#hashengineering) | [6](#hashengineering-open) | — | — | — | — | — | — | [6](#hashengineering-stale) | — | [1](#hashengineering-ready-for-human) | [20](#hashengineering-unresolved-comments) | — | ↓ 1 |
| [@bfoss765](#bfoss765) | [2](#bfoss765-open) | — | — | — | — | — | — | [2](#bfoss765-stale) | — | — | [7](#bfoss765-unresolved-comments) | — | ↓ 2 |
| [@llbartekll](#llbartekll) | [3](#llbartekll-open) | — | [1](#llbartekll-ci-failing) | — | — | — | — | [2](#llbartekll-stale) | — | — | [5](#llbartekll-unresolved-comments) | — | — |
| [@DCG-Claude](#dcg-claude) | [3](#dcg-claude-open) | — | — | — | — | — | — | [3](#dcg-claude-stale) | — | [2](#dcg-claude-ready-for-human) | — | — | — |
| [@thepastaclaw](#thepastaclaw) | [1](#thepastaclaw-open) | — | — | — | — | — | — | [1](#thepastaclaw-stale) | — | — | — | — | — |

## Per-author detail

<a id="claudius-maginificent"></a>
### @Claudius-Maginificent
<a id="claudius-maginificent-open"></a>
#### Open (58)
- [dashpay/platform#5588 feat(platform-wallet)!: expose protocol-14-aware signing-key selection](https://github.com/dashpay/platform/pull/5588) — 7 unresolved (7 bot) · 22 days stale · 🔴 CI failing · 🐢 targets v5.2-dev · areas: dpp, rs-platform-wallet, rs-platform-wallet-ffi, rust-sdk · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Include BLS keys in automatic document and token selection**" — 22 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5565 feat(platform-wallet)!: detect inconsistent shielded commitment trees and add explicit rebuild](https://github.com/dashpay/platform/pull/5565) — 7 unresolved (7 bot) · 22 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · 🐢 targets v5.2-dev · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Do not run the synchronous rebuild call on Swift's MainActor**" — 22 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5645 fix(platform-wallet)!: check identity creation keys and funding in the Rust core](https://github.com/dashpay/platform/pull/5645) — 7 unresolved (4 CodeRabbit, 3 bot) · 18 days stale · 🔴 CI failing · 🐢 targets v5.2-dev · areas: dpp, rs-platform-wallet, rs-platform-wallet-ffi, rust-sdk · Policy: waiting-author
  - Top thread: "&lt;picture&gt;&lt;img src="https://storage.googleapis.com/coderabbit_public_assets/review-comment-icons-v1/amber-16.svg" width="…" — 18 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: coderabbitai left review threads unresolved; resolve them
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5644 fix!: follow DIP-11 key rules when disabling keys and reading contact requests](https://github.com/dashpay/platform/pull/5644) — 8 unresolved (8 bot) · 4 days stale · ✋ changes requested · 🔴 CI failing · 🐢 targets v5.2-dev · areas: dpp, rs-platform-wallet, rs-platform-wallet-ffi, rust-sdk, js-wasm-sdk, fallback · Policy: waiting-author
  - Top thread: "🔴 Blocking: Fix the TypeScript type error in the new identities update spec**" — 4 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/dash-evo-tool#1173 feat(cli): import a wallet from a recovery phrase read from stdin](https://github.com/dashpay/dash-evo-tool/pull/1173) — 10 unresolved (10 bot) · 2 days stale · ✋ changes requested · 📝 draft · areas: fallback · Policy: draft
  - Top thread: "🔴 Blocking: Reject unsupported escapes and quotes in unquoted values**" — 2 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/dash-evo-tool#1177 feat(ui): align app-wide styles with the token proposal](https://github.com/dashpay/dash-evo-tool/pull/1177) — 10 unresolved (10 bot) · 2 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · 🐢 targets feat/token-creator-workspace · areas: fallback
  - Top thread: "🔴 Blocking: Meet the 3:1 contrast target for light-mode field boundaries**" — 2 days old
- [dashpay/dash-evo-tool#1108 feat(documents): recognise indexOnly document types and delete their entries by values](https://github.com/dashpay/dash-evo-tool/pull/1108) — 3 unresolved (3 bot) · 28 days stale · ✋ changes requested · 📝 draft · 🐢 targets feat/document-optional-token-cost · areas: fallback
  - Top thread: "🟡 Suggestion: Make the non-deletable error actionable**" — 28 days old
- [dashpay/dash-evo-tool#1178 fix(tokens): keep JSON export responsive when the file chooser stalls](https://github.com/dashpay/dash-evo-tool/pull/1178) — 8 unresolved (8 bot) · 0 days stale · ✋ changes requested · 🔴 CI failing · 🐢 targets feat/app-design-system · areas: fallback
  - Top thread: "🟡 Suggestion: Exercise a publication failure after staging succeeds**" — 0 days old
- [dashpay/platform#5546 fix(platform-wallet): support HASH160 DashPay contact signing keys](https://github.com/dashpay/platform/pull/5546) — 2 unresolved (2 bot) · 22 days stale · ⚠ merge conflict · 🐢 targets feat/platform-wallet-public-signing-key-selection · areas: rs-drive-abci, dpp, rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk, fallback
  - Top thread: "🟡 Suggestion: Do not expose an unchecked signer-capability override in the public constructor**" — 22 days old
- [dashpay/dash-evo-tool#1176 feat(tokens): add guided token creation with complete review](https://github.com/dashpay/dash-evo-tool/pull/1176) — 6 unresolved (6 bot) · 0 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · areas: fallback · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Require a worked formula and note for every emission function**" — 0 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/dash-evo-tool#1200 feat(wallet)!: detect databases needing the platform bridge and remove the bridge](https://github.com/dashpay/dash-evo-tool/pull/1200) — 6 unresolved (6 bot) · 0 days stale · ✋ changes requested · 📝 draft · 🐢 targets feat/startup-error-dialog · areas: fallback
  - Top thread: "🔴 Blocking: The history probe allocates database strings before the storage size guard**" — 0 days old
- [dashpay/dash-evo-tool#1165 fix(wallet): send Platform address transfers and withdrawals through platform-wallet](https://github.com/dashpay/dash-evo-tool/pull/1165) — 5 unresolved (5 bot) · 0 days stale · 🔴 CI failing · areas: fallback · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Avoid recomputing every wallet’s owned Platform total on every frame**" — 0 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/dash-evo-tool#1117 fix(identity): choose signing keys Platform accepts on protocol 14](https://github.com/dashpay/dash-evo-tool/pull/1117) — 4 unresolved (4 bot) · 0 days stale · ✋ changes requested · 🔴 CI failing · areas: fallback · Policy: waiting-bots
  - Top thread: "🔴 Blocking: Keep existing group mint and burn proposals visible without a signer**" — 0 days old
  - Blocker: coderabbitai has not reported for the current head
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/dash-evo-tool#1124 feat(documents): let the user skip an optional token cost](https://github.com/dashpay/dash-evo-tool/pull/1124) — 1 unresolved (1 bot) · 25 days stale · ⚠ merge conflict · ✋ changes requested · 📝 draft · 🐢 targets fix/protocol14-signing-key-selection · areas: fallback
  - Top thread: "🔴 Blocking: Show the credit-fee outcome when an optional token cost is disabled**" — 25 days old
- [dashpay/tenderdash#1600 fix(consensus): deliver and process a polka block for a validator left in its round](https://github.com/dashpay/tenderdash/pull/1600) — 2 unresolved (2 bot) · 4 days stale · ✋ changes requested · 🔴 CI failing · areas: fallback · Policy: waiting-author
  - Top thread: "🔴 Blocking: Avoid racing on PartSet completeness in the gossip snapshot**" — 4 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/dash-evo-tool#1179 fix(tokens): retain the creator when opening a registered contract](https://github.com/dashpay/dash-evo-tool/pull/1179) — 3 unresolved (3 bot) · 0 days stale · 🔴 CI failing · 🐢 targets fix/token-json-export · areas: fallback
  - Top thread: "🟡 Suggestion: Assert that the preview displays fetched contract details**" — 0 days old
- [dashpay/platform#5707 fix(sdk): recover the nonce cache when unconfirmed nonces leave the accepted window](https://github.com/dashpay/platform/pull/5707) — 1 unresolved (1 bot) · 18 days stale · ✋ changes requested · 🔴 CI failing · 📝 draft · 🐢 targets v5.2-dev · areas: rust-sdk · Policy: draft
  - Top thread: "🔴 Blocking: Do not reissue nonces belonging to outstanding operations**" — 18 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5981 refactor(platform-wallet-storage): drop the unreachable rehydration gap-refill guard](https://github.com/dashpay/platform/pull/5981) — 2 unresolved (2 bot) · 2 days stale · ✋ changes requested · 🔴 CI failing · 📝 draft · 🐢 targets chore/rust-dashcore-dev · areas: wallet-storage
  - Top thread: "🔴 Blocking: Preserve script-based ownership after a failed refill**" — 2 days old
- [dashpay/dash-evo-tool#1160 feat(identity): show an identity's tokens in the Identities hub](https://github.com/dashpay/dash-evo-tool/pull/1160) — 2 unresolved (2 bot) · 0 days stale · ✋ changes requested · 🔴 CI failing · areas: fallback · Policy: waiting-bots
  - Top thread: "🔴 Blocking: Show reward information even when the identity has no balance**" — 0 days old
  - Blocker: coderabbitai has not reported for the current head
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/dash-evo-tool#1193 feat!: require Platform protocol 14](https://github.com/dashpay/dash-evo-tool/pull/1193) — 2 unresolved (2 bot) · 0 days stale · ✋ changes requested · 🔴 CI failing · areas: fallback · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Build the token options without a verified network version**" — 0 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/platform#5781 fix(wallet-storage): validate transaction bodies during bulk history load](https://github.com/dashpay/platform/pull/5781) — 2 unresolved (2 bot) · 1 days stale · ✋ changes requested · 🔴 CI failing · 🐢 targets v5.2-dev · areas: wallet-storage · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Test that Recovery isolates a wallet with a body-to-ID mismatch**" — 1 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#6016 fix(platform-wallet-storage): warn about insecure ancestor directories instead of refusing to open](https://github.com/dashpay/platform/pull/6016) — 2 unresolved (2 bot) · 0 days stale · ✋ changes requested · 🔴 CI failing · 🐢 targets v5.2-dev · areas: wallet-storage · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Keep paths out of copyable remediation commands**" — 0 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
- [dashpay/platform#5979 fix(sdk): keep PublicKey unchanged when the inner setter rejects a key](https://github.com/dashpay/platform/pull/5979) — 1 unresolved (1 bot) · 3 days stale · 🐢 targets chore/crypto-core-consumers · areas: fallback
  - Top thread: "🟡 Suggestion: Add successful cross-format setter coverage**" — 3 days old
- [dashpay/dash-evo-tool#1172 fix(mcp): keep embedded MCP requests' intermediate results off the GUI](https://github.com/dashpay/dash-evo-tool/pull/1172) — 1 unresolved (1 bot) · 2 days stale · ✋ changes requested · 🔴 CI failing · 📝 draft · areas: fallback · Policy: draft
  - Top thread: "🟡 Suggestion: Exercise report suppression through the MCP dispatch path**" — 2 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/dash-evo-tool#1199 feat(boot): show startup failures in a window](https://github.com/dashpay/dash-evo-tool/pull/1199) — 1 unresolved (1 bot) · 0 days stale · ✋ changes requested · 🔴 CI failing · areas: fallback · Policy: waiting-author
  - Top thread: "🔴 Blocking: Return a failure status when users quit from the macOS application menu**" — 0 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/platform#5951 chore: bump rust-dashcore to dev 399f47f9](https://github.com/dashpay/platform/pull/5951) — 1 unresolved (1 bot) · 0 days stale · ✋ changes requested · 🔴 CI failing · 🐢 targets v5.2-dev · areas: dpp, rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk, js-wasm-sdk, kotlin-sdk, fallback · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Assert that a failed DashPay build releases the address reservation**" — 0 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
- [dashpay/dash-evo-tool#1120 feat(identity): add keys with spending limits and raise key limits](https://github.com/dashpay/dash-evo-tool/pull/1120) — ⚠ merge conflict · 📝 draft · 🐢 targets fix/protocol14-signing-key-selection · areas: fallback
- [dashpay/dash-evo-tool#1121 feat(contracts): view and claim contract fee pots](https://github.com/dashpay/dash-evo-tool/pull/1121) — 📝 draft · 🐢 targets fix/protocol14-signing-key-selection · areas: fallback
- [dashpay/dash-evo-tool#1122 feat(documents): agree to document action fees before broadcasting](https://github.com/dashpay/dash-evo-tool/pull/1122) — ⚠ merge conflict · 📝 draft · 🐢 targets fix/protocol14-signing-key-selection · areas: fallback
- [dashpay/dash-evo-tool#1123 feat(identity): add authentication keys bound to a contract](https://github.com/dashpay/dash-evo-tool/pull/1123) — 📝 draft · 🐢 targets feat/identity-key-limits · areas: fallback
- [dashpay/dash-evo-tool#1128 refactor(tokens)!: send token actions through platform-wallet](https://github.com/dashpay/dash-evo-tool/pull/1128) — ⚠ merge conflict · 📝 draft · areas: fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/dash-evo-tool#1137 fix(identity): unlock the wallet that holds the chosen signing key](https://github.com/dashpay/dash-evo-tool/pull/1137) — ⚠ merge conflict · 📝 draft · 🐢 targets fix/protocol14-signing-key-selection · areas: fallback
- [dashpay/dash-evo-tool#1162 fix(app): close promptly while a network switch is in progress](https://github.com/dashpay/dash-evo-tool/pull/1162) — ⚠ merge conflict · 📝 draft · areas: fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/dash-evo-tool#1168 fix(tokens): keep the Token Creator's identity current and its registration state reliable](https://github.com/dashpay/dash-evo-tool/pull/1168) — 📝 draft · areas: fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/dash-evo-tool#1170 fix(ui): retire finished progress banners and describe key protection correctly](https://github.com/dashpay/dash-evo-tool/pull/1170) — 📝 draft · areas: fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/dash-evo-tool#1174 feat(skills): mark defects in visual-diff galleries](https://github.com/dashpay/dash-evo-tool/pull/1174) — 📝 draft · 🐢 targets feat/det-cli-mnemonic-stdin · areas: fallback
- [dashpay/dash-evo-tool#1175 docs: add maintainable token tutorial production toolkit](https://github.com/dashpay/dash-evo-tool/pull/1175) — 🔴 CI failing · areas: fallback · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/dash-evo-tool#1187 refactor(mcp): check the shield pre-flight against the shieldable maximum](https://github.com/dashpay/dash-evo-tool/pull/1187) — 🔴 CI failing · areas: fallback · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head
- [dashpay/dash-evo-tool#1192 feat(identity): claim token rewards from the identity hub](https://github.com/dashpay/dash-evo-tool/pull/1192) — ⚠ merge conflict · 📝 draft · 🐢 targets feat/identity-hub-tokens · areas: fallback
- [dashpay/dash-evo-tool#1195 fix(dashpay): recover contact requests this device could not record](https://github.com/dashpay/dash-evo-tool/pull/1195) — 📝 draft · 🐢 targets fix/dashpay-contact-info-keys · areas: fallback
- [dashpay/dash-evo-tool#1197 fix(send): offer a private-balance Max the network accepts](https://github.com/dashpay/dash-evo-tool/pull/1197) — 📝 draft · 🐢 targets feat/send-screen-redesign · areas: fallback
- [dashpay/dash-evo-tool#1202 feat(boot): ask an AI assistant about a startup failure](https://github.com/dashpay/dash-evo-tool/pull/1202) — 📝 draft · 🐢 targets feat/startup-error-dialog · areas: fallback
- [dashpay/dash-evo-tool#1203 feat(boot): keep the real cause of startup errors](https://github.com/dashpay/dash-evo-tool/pull/1203) — 📝 draft · 🐢 targets feat/startup-error-dialog · areas: fallback
- [dashpay/dash-evo-tool#1205 fix(identity): sign address-funded registration inputs with the proven nonce only](https://github.com/dashpay/dash-evo-tool/pull/1205) — 📝 draft · 🐢 targets fix/platform-address-ops-via-platform-wallet · areas: fallback
- [dashpay/dash-evo-tool#1206 fix(boot): show startup panics and window failures instead of closing](https://github.com/dashpay/dash-evo-tool/pull/1206) — 📝 draft · 🐢 targets feat/startup-error-dialog · areas: fallback
- [dashpay/platform#3549 test(platform-wallet): e2e framework + full test suite — triage pins, Found-*/PA-* guards, fail-closed persist, Stage-2 merge](https://github.com/dashpay/platform/pull/3549) — 📝 draft · ⏸ deferred · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk, kotlin-sdk, github, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5384 fix(platform-wallet)!: persist chain truncations so a stored wallet follows a reorg](https://github.com/dashpay/platform/pull/5384) — ⏸ deferred · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk, kotlin-sdk, fallback · Policy: waiting-author
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5417 fix(platform-wallet): own per-wallet transaction accounting in Rust for SwiftData](https://github.com/dashpay/platform/pull/5417) — ⏸ deferred · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk
- [dashpay/platform#5647 feat: expose the Rust identity key layout to FFI, Swift and Kotlin hosts](https://github.com/dashpay/platform/pull/5647) — ✋ changes requested · 🔴 CI failing · 🐢 targets fix/platform-wallet-dip11-identity-key-layout · areas: rs-platform-wallet-ffi, swift-sdk, kotlin-sdk
- [dashpay/platform#5660 refactor(dpp): own the contract-bounds verdict for a document create](https://github.com/dashpay/platform/pull/5660) — 🐢 targets feat/platform-wallet-public-signing-key-selection · areas: rs-drive-abci, dpp, rs-platform-wallet
- [dashpay/platform#5661 feat: decide which identity keys may be disabled in the wallet, not in the mobile apps](https://github.com/dashpay/platform/pull/5661) — ⚠ merge conflict · 🔴 CI failing · 🐢 targets fix/platform-wallet-dip11-key-rules · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk
- [dashpay/platform#5782 fix(wallet-storage): restore Platform and Core from one wallet snapshot](https://github.com/dashpay/platform/pull/5782) — ⚠ merge conflict · 📝 draft · 🐢 targets feat/sqlite-wallet-restore · areas: wallet-storage
- [dashpay/platform#5783 fix(wallet-storage)!: reconcile historical AssetUnlock identities](https://github.com/dashpay/platform/pull/5783) — ⚠ merge conflict · 📝 draft · 🐢 targets fix/sqlite-bulk-history-identity · areas: rs-platform-wallet, wallet-storage
- [dashpay/platform#5787 fix(platform-wallet): classify asset locks funding foreign credits as outgoing](https://github.com/dashpay/platform/pull/5787) — ⚠ merge conflict · 📝 draft · 🐢 targets feat/sqlite-wallet-restore · areas: rs-platform-wallet, wallet-storage
- [dashpay/platform#5788 fix(platform-wallet): compile shielded-only drain helpers only with the shielded feature](https://github.com/dashpay/platform/pull/5788) — 🐢 targets v5.2-dev · areas: rs-platform-wallet · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
- [dashpay/platform#5961 fix(platform-wallet): don't pick a limited key to sign a data contract create or update](https://github.com/dashpay/platform/pull/5961) — 📝 draft · 🐢 targets feat/platform-wallet-public-signing-key-selection · areas: rs-platform-wallet
- [dashpay/tenderdash#1578 fix(statesync): refuse surplus light block responses without blocking](https://github.com/dashpay/tenderdash/pull/1578) — 📝 draft · areas: fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/tenderdash#1627 fix(p2p): bound the simple priority queue, honor channel priorities and count drops](https://github.com/dashpay/tenderdash/pull/1627) — 🔴 CI failing · 📝 draft · areas: fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot

<a id="claudius-maginificent-needs-action"></a>
#### Needs action (7)
- [dashpay/dash-evo-tool#1176 feat(tokens): add guided token creation with complete review](https://github.com/dashpay/dash-evo-tool/pull/1176) — 6 unresolved (6 bot) · 0 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · areas: fallback · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Require a worked formula and note for every emission function**" — 0 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/dash-evo-tool#1165 fix(wallet): send Platform address transfers and withdrawals through platform-wallet](https://github.com/dashpay/dash-evo-tool/pull/1165) — 5 unresolved (5 bot) · 0 days stale · 🔴 CI failing · areas: fallback · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Avoid recomputing every wallet’s owned Platform total on every frame**" — 0 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/dash-evo-tool#1117 fix(identity): choose signing keys Platform accepts on protocol 14](https://github.com/dashpay/dash-evo-tool/pull/1117) — 4 unresolved (4 bot) · 0 days stale · ✋ changes requested · 🔴 CI failing · areas: fallback · Policy: waiting-bots
  - Top thread: "🔴 Blocking: Keep existing group mint and burn proposals visible without a signer**" — 0 days old
  - Blocker: coderabbitai has not reported for the current head
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/tenderdash#1600 fix(consensus): deliver and process a polka block for a validator left in its round](https://github.com/dashpay/tenderdash/pull/1600) — 2 unresolved (2 bot) · 4 days stale · ✋ changes requested · 🔴 CI failing · areas: fallback · Policy: waiting-author
  - Top thread: "🔴 Blocking: Avoid racing on PartSet completeness in the gossip snapshot**" — 4 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/dash-evo-tool#1160 feat(identity): show an identity's tokens in the Identities hub](https://github.com/dashpay/dash-evo-tool/pull/1160) — 2 unresolved (2 bot) · 0 days stale · ✋ changes requested · 🔴 CI failing · areas: fallback · Policy: waiting-bots
  - Top thread: "🔴 Blocking: Show reward information even when the identity has no balance**" — 0 days old
  - Blocker: coderabbitai has not reported for the current head
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/dash-evo-tool#1193 feat!: require Platform protocol 14](https://github.com/dashpay/dash-evo-tool/pull/1193) — 2 unresolved (2 bot) · 0 days stale · ✋ changes requested · 🔴 CI failing · areas: fallback · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Build the token options without a verified network version**" — 0 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/dash-evo-tool#1199 feat(boot): show startup failures in a window](https://github.com/dashpay/dash-evo-tool/pull/1199) — 1 unresolved (1 bot) · 0 days stale · ✋ changes requested · 🔴 CI failing · areas: fallback · Policy: waiting-author
  - Top thread: "🔴 Blocking: Return a failure status when users quit from the macOS application menu**" — 0 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return

<a id="claudius-maginificent-ready-for-human"></a>
#### Ready for human (1)
- [dashpay/platform#5788 fix(platform-wallet): compile shielded-only drain helpers only with the shielded feature](https://github.com/dashpay/platform/pull/5788) — 🐢 targets v5.2-dev · areas: rs-platform-wallet · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required

<a id="claudius-maginificent-unresolved-comments"></a>
#### Unresolved Comments (7)
- [dashpay/dash-evo-tool#1176 feat(tokens): add guided token creation with complete review](https://github.com/dashpay/dash-evo-tool/pull/1176) — 6 unresolved (6 bot) · 0 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · areas: fallback · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Require a worked formula and note for every emission function**" — 0 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/dash-evo-tool#1165 fix(wallet): send Platform address transfers and withdrawals through platform-wallet](https://github.com/dashpay/dash-evo-tool/pull/1165) — 5 unresolved (5 bot) · 0 days stale · 🔴 CI failing · areas: fallback · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Avoid recomputing every wallet’s owned Platform total on every frame**" — 0 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/dash-evo-tool#1117 fix(identity): choose signing keys Platform accepts on protocol 14](https://github.com/dashpay/dash-evo-tool/pull/1117) — 4 unresolved (4 bot) · 0 days stale · ✋ changes requested · 🔴 CI failing · areas: fallback · Policy: waiting-bots
  - Top thread: "🔴 Blocking: Keep existing group mint and burn proposals visible without a signer**" — 0 days old
  - Blocker: coderabbitai has not reported for the current head
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/tenderdash#1600 fix(consensus): deliver and process a polka block for a validator left in its round](https://github.com/dashpay/tenderdash/pull/1600) — 2 unresolved (2 bot) · 4 days stale · ✋ changes requested · 🔴 CI failing · areas: fallback · Policy: waiting-author
  - Top thread: "🔴 Blocking: Avoid racing on PartSet completeness in the gossip snapshot**" — 4 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/dash-evo-tool#1160 feat(identity): show an identity's tokens in the Identities hub](https://github.com/dashpay/dash-evo-tool/pull/1160) — 2 unresolved (2 bot) · 0 days stale · ✋ changes requested · 🔴 CI failing · areas: fallback · Policy: waiting-bots
  - Top thread: "🔴 Blocking: Show reward information even when the identity has no balance**" — 0 days old
  - Blocker: coderabbitai has not reported for the current head
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/dash-evo-tool#1193 feat!: require Platform protocol 14](https://github.com/dashpay/dash-evo-tool/pull/1193) — 2 unresolved (2 bot) · 0 days stale · ✋ changes requested · 🔴 CI failing · areas: fallback · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Build the token options without a verified network version**" — 0 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/dash-evo-tool#1199 feat(boot): show startup failures in a window](https://github.com/dashpay/dash-evo-tool/pull/1199) — 1 unresolved (1 bot) · 0 days stale · ✋ changes requested · 🔴 CI failing · areas: fallback · Policy: waiting-author
  - Top thread: "🔴 Blocking: Return a failure status when users quit from the macOS application menu**" — 0 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return

<a id="claudius-maginificent-ci-failing"></a>
#### CI Failing (2)
- [dashpay/dash-evo-tool#1175 docs: add maintainable token tutorial production toolkit](https://github.com/dashpay/dash-evo-tool/pull/1175) — 🔴 CI failing · areas: fallback · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/dash-evo-tool#1187 refactor(mcp): check the shield pre-flight against the shieldable maximum](https://github.com/dashpay/dash-evo-tool/pull/1187) — 🔴 CI failing · areas: fallback · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head

<a id="claudius-maginificent-deferred"></a>
#### Deferred (3)
- [dashpay/platform#3549 test(platform-wallet): e2e framework + full test suite — triage pins, Found-*/PA-* guards, fail-closed persist, Stage-2 merge](https://github.com/dashpay/platform/pull/3549) — 📝 draft · ⏸ deferred · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk, kotlin-sdk, github, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5384 fix(platform-wallet)!: persist chain truncations so a stored wallet follows a reorg](https://github.com/dashpay/platform/pull/5384) — ⏸ deferred · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk, kotlin-sdk, fallback · Policy: waiting-author
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5417 fix(platform-wallet): own per-wallet transaction accounting in Rust for SwiftData](https://github.com/dashpay/platform/pull/5417) — ⏸ deferred · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk

<a id="claudius-maginificent-draft"></a>
#### Draft (8)
- [dashpay/dash-evo-tool#1173 feat(cli): import a wallet from a recovery phrase read from stdin](https://github.com/dashpay/dash-evo-tool/pull/1173) — 10 unresolved (10 bot) · 2 days stale · ✋ changes requested · 📝 draft · areas: fallback · Policy: draft
  - Top thread: "🔴 Blocking: Reject unsupported escapes and quotes in unquoted values**" — 2 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/dash-evo-tool#1172 fix(mcp): keep embedded MCP requests' intermediate results off the GUI](https://github.com/dashpay/dash-evo-tool/pull/1172) — 1 unresolved (1 bot) · 2 days stale · ✋ changes requested · 🔴 CI failing · 📝 draft · areas: fallback · Policy: draft
  - Top thread: "🟡 Suggestion: Exercise report suppression through the MCP dispatch path**" — 2 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/dash-evo-tool#1128 refactor(tokens)!: send token actions through platform-wallet](https://github.com/dashpay/dash-evo-tool/pull/1128) — ⚠ merge conflict · 📝 draft · areas: fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/dash-evo-tool#1162 fix(app): close promptly while a network switch is in progress](https://github.com/dashpay/dash-evo-tool/pull/1162) — ⚠ merge conflict · 📝 draft · areas: fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/dash-evo-tool#1168 fix(tokens): keep the Token Creator's identity current and its registration state reliable](https://github.com/dashpay/dash-evo-tool/pull/1168) — 📝 draft · areas: fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/dash-evo-tool#1170 fix(ui): retire finished progress banners and describe key protection correctly](https://github.com/dashpay/dash-evo-tool/pull/1170) — 📝 draft · areas: fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/tenderdash#1578 fix(statesync): refuse surplus light block responses without blocking](https://github.com/dashpay/tenderdash/pull/1578) — 📝 draft · areas: fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/tenderdash#1627 fix(p2p): bound the simple priority queue, honor channel priorities and count drops](https://github.com/dashpay/tenderdash/pull/1627) — 🔴 CI failing · 📝 draft · areas: fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot

<a id="claudius-maginificent-stale"></a>
#### Stale (38)
- [dashpay/platform#5588 feat(platform-wallet)!: expose protocol-14-aware signing-key selection](https://github.com/dashpay/platform/pull/5588) — 7 unresolved (7 bot) · 22 days stale · 🔴 CI failing · 🐢 targets v5.2-dev · areas: dpp, rs-platform-wallet, rs-platform-wallet-ffi, rust-sdk · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Include BLS keys in automatic document and token selection**" — 22 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5565 feat(platform-wallet)!: detect inconsistent shielded commitment trees and add explicit rebuild](https://github.com/dashpay/platform/pull/5565) — 7 unresolved (7 bot) · 22 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · 🐢 targets v5.2-dev · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Do not run the synchronous rebuild call on Swift's MainActor**" — 22 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5645 fix(platform-wallet)!: check identity creation keys and funding in the Rust core](https://github.com/dashpay/platform/pull/5645) — 7 unresolved (4 CodeRabbit, 3 bot) · 18 days stale · 🔴 CI failing · 🐢 targets v5.2-dev · areas: dpp, rs-platform-wallet, rs-platform-wallet-ffi, rust-sdk · Policy: waiting-author
  - Top thread: "&lt;picture&gt;&lt;img src="https://storage.googleapis.com/coderabbit_public_assets/review-comment-icons-v1/amber-16.svg" width="…" — 18 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: coderabbitai left review threads unresolved; resolve them
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5644 fix!: follow DIP-11 key rules when disabling keys and reading contact requests](https://github.com/dashpay/platform/pull/5644) — 8 unresolved (8 bot) · 4 days stale · ✋ changes requested · 🔴 CI failing · 🐢 targets v5.2-dev · areas: dpp, rs-platform-wallet, rs-platform-wallet-ffi, rust-sdk, js-wasm-sdk, fallback · Policy: waiting-author
  - Top thread: "🔴 Blocking: Fix the TypeScript type error in the new identities update spec**" — 4 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/dash-evo-tool#1177 feat(ui): align app-wide styles with the token proposal](https://github.com/dashpay/dash-evo-tool/pull/1177) — 10 unresolved (10 bot) · 2 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · 🐢 targets feat/token-creator-workspace · areas: fallback
  - Top thread: "🔴 Blocking: Meet the 3:1 contrast target for light-mode field boundaries**" — 2 days old
- [dashpay/dash-evo-tool#1108 feat(documents): recognise indexOnly document types and delete their entries by values](https://github.com/dashpay/dash-evo-tool/pull/1108) — 3 unresolved (3 bot) · 28 days stale · ✋ changes requested · 📝 draft · 🐢 targets feat/document-optional-token-cost · areas: fallback
  - Top thread: "🟡 Suggestion: Make the non-deletable error actionable**" — 28 days old
- [dashpay/dash-evo-tool#1178 fix(tokens): keep JSON export responsive when the file chooser stalls](https://github.com/dashpay/dash-evo-tool/pull/1178) — 8 unresolved (8 bot) · 0 days stale · ✋ changes requested · 🔴 CI failing · 🐢 targets feat/app-design-system · areas: fallback
  - Top thread: "🟡 Suggestion: Exercise a publication failure after staging succeeds**" — 0 days old
- [dashpay/platform#5546 fix(platform-wallet): support HASH160 DashPay contact signing keys](https://github.com/dashpay/platform/pull/5546) — 2 unresolved (2 bot) · 22 days stale · ⚠ merge conflict · 🐢 targets feat/platform-wallet-public-signing-key-selection · areas: rs-drive-abci, dpp, rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk, fallback
  - Top thread: "🟡 Suggestion: Do not expose an unchecked signer-capability override in the public constructor**" — 22 days old
- [dashpay/dash-evo-tool#1200 feat(wallet)!: detect databases needing the platform bridge and remove the bridge](https://github.com/dashpay/dash-evo-tool/pull/1200) — 6 unresolved (6 bot) · 0 days stale · ✋ changes requested · 📝 draft · 🐢 targets feat/startup-error-dialog · areas: fallback
  - Top thread: "🔴 Blocking: The history probe allocates database strings before the storage size guard**" — 0 days old
- [dashpay/dash-evo-tool#1124 feat(documents): let the user skip an optional token cost](https://github.com/dashpay/dash-evo-tool/pull/1124) — 1 unresolved (1 bot) · 25 days stale · ⚠ merge conflict · ✋ changes requested · 📝 draft · 🐢 targets fix/protocol14-signing-key-selection · areas: fallback
  - Top thread: "🔴 Blocking: Show the credit-fee outcome when an optional token cost is disabled**" — 25 days old
- [dashpay/dash-evo-tool#1179 fix(tokens): retain the creator when opening a registered contract](https://github.com/dashpay/dash-evo-tool/pull/1179) — 3 unresolved (3 bot) · 0 days stale · 🔴 CI failing · 🐢 targets fix/token-json-export · areas: fallback
  - Top thread: "🟡 Suggestion: Assert that the preview displays fetched contract details**" — 0 days old
- [dashpay/platform#5707 fix(sdk): recover the nonce cache when unconfirmed nonces leave the accepted window](https://github.com/dashpay/platform/pull/5707) — 1 unresolved (1 bot) · 18 days stale · ✋ changes requested · 🔴 CI failing · 📝 draft · 🐢 targets v5.2-dev · areas: rust-sdk · Policy: draft
  - Top thread: "🔴 Blocking: Do not reissue nonces belonging to outstanding operations**" — 18 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5981 refactor(platform-wallet-storage): drop the unreachable rehydration gap-refill guard](https://github.com/dashpay/platform/pull/5981) — 2 unresolved (2 bot) · 2 days stale · ✋ changes requested · 🔴 CI failing · 📝 draft · 🐢 targets chore/rust-dashcore-dev · areas: wallet-storage
  - Top thread: "🔴 Blocking: Preserve script-based ownership after a failed refill**" — 2 days old
- [dashpay/platform#5781 fix(wallet-storage): validate transaction bodies during bulk history load](https://github.com/dashpay/platform/pull/5781) — 2 unresolved (2 bot) · 1 days stale · ✋ changes requested · 🔴 CI failing · 🐢 targets v5.2-dev · areas: wallet-storage · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Test that Recovery isolates a wallet with a body-to-ID mismatch**" — 1 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#6016 fix(platform-wallet-storage): warn about insecure ancestor directories instead of refusing to open](https://github.com/dashpay/platform/pull/6016) — 2 unresolved (2 bot) · 0 days stale · ✋ changes requested · 🔴 CI failing · 🐢 targets v5.2-dev · areas: wallet-storage · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Keep paths out of copyable remediation commands**" — 0 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
- [dashpay/platform#5979 fix(sdk): keep PublicKey unchanged when the inner setter rejects a key](https://github.com/dashpay/platform/pull/5979) — 1 unresolved (1 bot) · 3 days stale · 🐢 targets chore/crypto-core-consumers · areas: fallback
  - Top thread: "🟡 Suggestion: Add successful cross-format setter coverage**" — 3 days old
- [dashpay/platform#5951 chore: bump rust-dashcore to dev 399f47f9](https://github.com/dashpay/platform/pull/5951) — 1 unresolved (1 bot) · 0 days stale · ✋ changes requested · 🔴 CI failing · 🐢 targets v5.2-dev · areas: dpp, rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk, js-wasm-sdk, kotlin-sdk, fallback · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Assert that a failed DashPay build releases the address reservation**" — 0 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
- [dashpay/dash-evo-tool#1120 feat(identity): add keys with spending limits and raise key limits](https://github.com/dashpay/dash-evo-tool/pull/1120) — ⚠ merge conflict · 📝 draft · 🐢 targets fix/protocol14-signing-key-selection · areas: fallback
- [dashpay/dash-evo-tool#1121 feat(contracts): view and claim contract fee pots](https://github.com/dashpay/dash-evo-tool/pull/1121) — 📝 draft · 🐢 targets fix/protocol14-signing-key-selection · areas: fallback
- [dashpay/dash-evo-tool#1122 feat(documents): agree to document action fees before broadcasting](https://github.com/dashpay/dash-evo-tool/pull/1122) — ⚠ merge conflict · 📝 draft · 🐢 targets fix/protocol14-signing-key-selection · areas: fallback
- [dashpay/dash-evo-tool#1123 feat(identity): add authentication keys bound to a contract](https://github.com/dashpay/dash-evo-tool/pull/1123) — 📝 draft · 🐢 targets feat/identity-key-limits · areas: fallback
- [dashpay/dash-evo-tool#1137 fix(identity): unlock the wallet that holds the chosen signing key](https://github.com/dashpay/dash-evo-tool/pull/1137) — ⚠ merge conflict · 📝 draft · 🐢 targets fix/protocol14-signing-key-selection · areas: fallback
- [dashpay/dash-evo-tool#1174 feat(skills): mark defects in visual-diff galleries](https://github.com/dashpay/dash-evo-tool/pull/1174) — 📝 draft · 🐢 targets feat/det-cli-mnemonic-stdin · areas: fallback
- [dashpay/dash-evo-tool#1192 feat(identity): claim token rewards from the identity hub](https://github.com/dashpay/dash-evo-tool/pull/1192) — ⚠ merge conflict · 📝 draft · 🐢 targets feat/identity-hub-tokens · areas: fallback
- [dashpay/dash-evo-tool#1195 fix(dashpay): recover contact requests this device could not record](https://github.com/dashpay/dash-evo-tool/pull/1195) — 📝 draft · 🐢 targets fix/dashpay-contact-info-keys · areas: fallback
- [dashpay/dash-evo-tool#1197 fix(send): offer a private-balance Max the network accepts](https://github.com/dashpay/dash-evo-tool/pull/1197) — 📝 draft · 🐢 targets feat/send-screen-redesign · areas: fallback
- [dashpay/dash-evo-tool#1202 feat(boot): ask an AI assistant about a startup failure](https://github.com/dashpay/dash-evo-tool/pull/1202) — 📝 draft · 🐢 targets feat/startup-error-dialog · areas: fallback
- [dashpay/dash-evo-tool#1203 feat(boot): keep the real cause of startup errors](https://github.com/dashpay/dash-evo-tool/pull/1203) — 📝 draft · 🐢 targets feat/startup-error-dialog · areas: fallback
- [dashpay/dash-evo-tool#1205 fix(identity): sign address-funded registration inputs with the proven nonce only](https://github.com/dashpay/dash-evo-tool/pull/1205) — 📝 draft · 🐢 targets fix/platform-address-ops-via-platform-wallet · areas: fallback
- [dashpay/dash-evo-tool#1206 fix(boot): show startup panics and window failures instead of closing](https://github.com/dashpay/dash-evo-tool/pull/1206) — 📝 draft · 🐢 targets feat/startup-error-dialog · areas: fallback
- [dashpay/platform#5647 feat: expose the Rust identity key layout to FFI, Swift and Kotlin hosts](https://github.com/dashpay/platform/pull/5647) — ✋ changes requested · 🔴 CI failing · 🐢 targets fix/platform-wallet-dip11-identity-key-layout · areas: rs-platform-wallet-ffi, swift-sdk, kotlin-sdk
- [dashpay/platform#5660 refactor(dpp): own the contract-bounds verdict for a document create](https://github.com/dashpay/platform/pull/5660) — 🐢 targets feat/platform-wallet-public-signing-key-selection · areas: rs-drive-abci, dpp, rs-platform-wallet
- [dashpay/platform#5661 feat: decide which identity keys may be disabled in the wallet, not in the mobile apps](https://github.com/dashpay/platform/pull/5661) — ⚠ merge conflict · 🔴 CI failing · 🐢 targets fix/platform-wallet-dip11-key-rules · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk
- [dashpay/platform#5782 fix(wallet-storage): restore Platform and Core from one wallet snapshot](https://github.com/dashpay/platform/pull/5782) — ⚠ merge conflict · 📝 draft · 🐢 targets feat/sqlite-wallet-restore · areas: wallet-storage
- [dashpay/platform#5783 fix(wallet-storage)!: reconcile historical AssetUnlock identities](https://github.com/dashpay/platform/pull/5783) — ⚠ merge conflict · 📝 draft · 🐢 targets fix/sqlite-bulk-history-identity · areas: rs-platform-wallet, wallet-storage
- [dashpay/platform#5787 fix(platform-wallet): classify asset locks funding foreign credits as outgoing](https://github.com/dashpay/platform/pull/5787) — ⚠ merge conflict · 📝 draft · 🐢 targets feat/sqlite-wallet-restore · areas: rs-platform-wallet, wallet-storage
- [dashpay/platform#5788 fix(platform-wallet): compile shielded-only drain helpers only with the shielded feature](https://github.com/dashpay/platform/pull/5788) — 🐢 targets v5.2-dev · areas: rs-platform-wallet · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
- [dashpay/platform#5961 fix(platform-wallet): don't pick a limited key to sign a data contract create or update](https://github.com/dashpay/platform/pull/5961) — 📝 draft · 🐢 targets feat/platform-wallet-public-signing-key-selection · areas: rs-platform-wallet

<a id="quantumexplorer"></a>
### @QuantumExplorer
<a id="quantumexplorer-open"></a>
#### Open (24)
- [dashpay/platform#5750 feat(platform)!: random token transfer and mint amounts](https://github.com/dashpay/platform/pull/5750) — 13 unresolved (13 bot) · 17 days stale · 🐢 targets claude/platform-tenderdash-randomness-149e4e · areas: rs-drive, dpp, rs-platform-wallet, rust-sdk, rust-sdk-ffi, fallback
  - Top thread: "✅ **Resolved** at \`f8fcc8fd\`; see the replies below." — 17 days old
- [dashpay/platform#5749 feat(platform)!: sys.random.uniform document properties](https://github.com/dashpay/platform/pull/5749) — 9 unresolved (9 bot) · 17 days stale · 🔴 CI failing · 🐢 targets v5.2-dev · areas: rs-drive, rs-drive-abci, dpp, fallback · Policy: waiting-author
  - Top thread: "✅ **Resolved** at \`0ead2b47\`; see the replies below." — 17 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5753 feat(drive-abci)!: apply block randomness through the ApplyBlockRandomness ABCI method](https://github.com/dashpay/platform/pull/5753) — 8 unresolved (8 bot) · 17 days stale · ⚠ merge conflict · 🔴 CI failing · 🐢 targets claude/random-token-amounts · areas: rs-drive, rs-drive-abci, dpp, fallback
  - Top thread: "🟡 Suggestion: Exercise the protocol upgrade with signed commits**" — 17 days old
- [dashpay/platform#6023 fix(kotlin-example-app): connect to a devnet by name and stop showing "null" ban reasons](https://github.com/dashpay/platform/pull/6023) — 10 unresolved (10 bot) · 0 days stale · ✋ changes requested · 🔴 CI failing · areas: kotlin-sdk · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Add regression coverage for nullable ban-info fields**" — 0 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#6022 fix(sdk)!: read document history on a network below protocol version 15](https://github.com/dashpay/platform/pull/6022) — 8 unresolved (8 bot) · 0 days stale · ✋ changes requested · 🔴 CI failing · areas: rs-drive-abci, rust-sdk, js-wasm-sdk, fallback · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Document the Rust API break without marking it consensus-breaking**" — 0 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5529 feat(platform)!: token marketplace listings (PV15)](https://github.com/dashpay/platform/pull/5529) — 2 unresolved (2 bot) · 27 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · 🐢 targets v5.2-dev · areas: rs-drive, rs-drive-abci, dpp, rs-platform-wallet, swift-sdk, kotlin-sdk, fallback · Policy: waiting-author
  - Top thread: "🔴 Blocking: Version the consensus order-ID derivation through platform-version tables**" — 27 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5530 feat(platform)!: token marketplace offers (PV15)](https://github.com/dashpay/platform/pull/5530) — 1 unresolved (1 bot) · 21 days stale · ✋ changes requested · 🔴 CI failing · 🐢 targets claude/token-market-listings · areas: rs-drive, rs-drive-abci, dpp, rs-platform-wallet, fallback
  - Top thread: "🔴 Blocking: Bound bid escrow by a buyer-signed credit limit**" — 21 days old
- [dashpay/tenderdash#1582 feat(abci)!: add the ApplyBlockRandomness method and commit beacon fields](https://github.com/dashpay/tenderdash/pull/1582) — 1 unresolved (1 bot) · 15 days stale · areas: fallback · Policy: waiting-author
  - Top thread: "✅ **Resolved** at \`cd6b4ec8\`; see the replies below." — 15 days old
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/tenderdash#1597 fix(p2p): trim the simple priority queue below its limit](https://github.com/dashpay/tenderdash/pull/1597) — 1 unresolved (1 bot) · 12 days stale · ⚠ merge conflict · areas: fallback · Policy: waiting-self-review
  - Top thread: "🟡 Suggestion: Count the larger overflow batches**" — 12 days old
  - Blocker: Author must post /self-reviewed fb3e02bfc772facdefade06c2eeca6743e23e329
- [dashpay/platform#6021 fix(drive-abci): refuse disabling a missing key with a consensus error at check_tx](https://github.com/dashpay/platform/pull/6021) — 2 unresolved (2 bot) · 0 days stale · areas: rs-drive-abci · Policy: waiting-self-review
  - Top thread: "🟡 Suggestion: Document the admission change under the legacy fee estimator**" — 0 days old
  - Blocker: Author must post /self-reviewed 0a7c3bfbcbd3dcd28d7cc7703e7d56dcc70e158f
- [dashpay/platform#5968 feat(platform-wallet)!: apps show the shielded balance the wallet announces, without identity accounts](https://github.com/dashpay/platform/pull/5968) — 1 unresolved (1 bot) · 3 days stale · 🔴 CI failing · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Keep JNI balance-trampoline conversion inside the panic guard**" — 3 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
- [dashpay/platform#4519 feat(platform-wallet): rotate a masternode's keys into the wallet — ProUpRegTx orchestration, FFI, Swift](https://github.com/dashpay/platform/pull/4519) — ⚠ merge conflict · 🔴 CI failing · 🐢 targets v5.2-dev · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 92a3ebe17adc12432fb756c63f172c663809007d
  - Blocker: Proceeded without thepastaclaw: no review within the configured window
- [dashpay/platform#4730 feat(platform)!: delta-based data contract update transition for protocol version 15](https://github.com/dashpay/platform/pull/4730) — ⚠ merge conflict · 🔴 CI failing · 📝 draft · 🐢 targets v5.2-dev · areas: rs-drive, rs-drive-abci, dpp, rs-platform-wallet, rust-sdk, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#4776 feat(platform-wallet)!: let contract updates clear the description through the FFI](https://github.com/dashpay/platform/pull/4776) — ⚠ merge conflict · 🔴 CI failing · 🐢 targets feat/delta-contract-update-pv15 · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk
- [dashpay/platform#4986 fix(drive-abci)!: pay every reward share of a masternode and credit each identity once in the epoch payout (PV15)](https://github.com/dashpay/platform/pull/4986) — ⚠ merge conflict · 📝 draft · 🐢 targets v5.2-dev · areas: rs-drive-abci, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5113 feat(platform-wallet)!: keep masternode collateral out of coin selection](https://github.com/dashpay/platform/pull/5113) — ⚠ merge conflict · 🔴 CI failing · 📝 draft · 🐢 targets chore/bump-rust-dashcore-secp-033 · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk, fallback
- [dashpay/platform#5531 feat(platform)!: record token marketplace events in token history (PV15)](https://github.com/dashpay/platform/pull/5531) — 🔴 CI failing · 🐢 targets claude/token-market-offers · areas: rs-drive, rs-drive-abci, dpp, system-contracts, fallback
- [dashpay/platform#5532 feat(platform)!: destroy a frozen identity's escrowed listings with its funds (PV15)](https://github.com/dashpay/platform/pull/5532) — 🔴 CI failing · 🐢 targets claude/token-market-history · areas: rs-drive, rs-drive-abci, fallback
- [dashpay/platform#5736 feat(platform)!: add a RedPallas signer key type with its own purpose (PV15)](https://github.com/dashpay/platform/pull/5736) — ⚠ merge conflict · 📝 draft · 🐢 targets v5.2-dev · areas: rs-drive, rs-drive-abci, dpp, rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, rust-sdk-ffi, kotlin-sdk, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5751 feat(sdk): random token amounts in wasm-sdk and js-evo-sdk](https://github.com/dashpay/platform/pull/5751) — 🐢 targets claude/random-token-amounts · areas: js-wasm-sdk
- [dashpay/platform#5967 feat(sdk): host apps can prepare an identity's shielded account](https://github.com/dashpay/platform/pull/5967) — 🔴 CI failing · 📝 draft · areas: rs-platform-wallet-ffi, swift-sdk, kotlin-sdk · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#6002 fix(dashmate): run an added local node on its network's Core image](https://github.com/dashpay/platform/pull/6002) — areas: dashmate · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 9cc7778487200a5f379f0cb47cfff854e1a520dd
- [dashpay/tenderdash#1583 feat(consensus)!: recover a beacon from prevotes and sign precommits over the randomness-applied app hash](https://github.com/dashpay/tenderdash/pull/1583) — ⚠ merge conflict · 📝 draft · areas: fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/tenderdash#1584 fix(statesync): restore LastAppHash of the snapshot height, not the next one](https://github.com/dashpay/tenderdash/pull/1584) — areas: fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 23c1443e4077c7c7dfedf00f9446c762e669660f

<a id="quantumexplorer-needs-action"></a>
#### Needs action (6)
- [dashpay/platform#6023 fix(kotlin-example-app): connect to a devnet by name and stop showing "null" ban reasons](https://github.com/dashpay/platform/pull/6023) — 10 unresolved (10 bot) · 0 days stale · ✋ changes requested · 🔴 CI failing · areas: kotlin-sdk · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Add regression coverage for nullable ban-info fields**" — 0 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#6022 fix(sdk)!: read document history on a network below protocol version 15](https://github.com/dashpay/platform/pull/6022) — 8 unresolved (8 bot) · 0 days stale · ✋ changes requested · 🔴 CI failing · areas: rs-drive-abci, rust-sdk, js-wasm-sdk, fallback · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Document the Rust API break without marking it consensus-breaking**" — 0 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/tenderdash#1582 feat(abci)!: add the ApplyBlockRandomness method and commit beacon fields](https://github.com/dashpay/tenderdash/pull/1582) — 1 unresolved (1 bot) · 15 days stale · areas: fallback · Policy: waiting-author
  - Top thread: "✅ **Resolved** at \`cd6b4ec8\`; see the replies below." — 15 days old
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/tenderdash#1597 fix(p2p): trim the simple priority queue below its limit](https://github.com/dashpay/tenderdash/pull/1597) — 1 unresolved (1 bot) · 12 days stale · ⚠ merge conflict · areas: fallback · Policy: waiting-self-review
  - Top thread: "🟡 Suggestion: Count the larger overflow batches**" — 12 days old
  - Blocker: Author must post /self-reviewed fb3e02bfc772facdefade06c2eeca6743e23e329
- [dashpay/platform#6021 fix(drive-abci): refuse disabling a missing key with a consensus error at check_tx](https://github.com/dashpay/platform/pull/6021) — 2 unresolved (2 bot) · 0 days stale · areas: rs-drive-abci · Policy: waiting-self-review
  - Top thread: "🟡 Suggestion: Document the admission change under the legacy fee estimator**" — 0 days old
  - Blocker: Author must post /self-reviewed 0a7c3bfbcbd3dcd28d7cc7703e7d56dcc70e158f
- [dashpay/platform#5968 feat(platform-wallet)!: apps show the shielded balance the wallet announces, without identity accounts](https://github.com/dashpay/platform/pull/5968) — 1 unresolved (1 bot) · 3 days stale · 🔴 CI failing · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Keep JNI balance-trampoline conversion inside the panic guard**" — 3 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix

<a id="quantumexplorer-unresolved-comments"></a>
#### Unresolved Comments (6)
- [dashpay/platform#6023 fix(kotlin-example-app): connect to a devnet by name and stop showing "null" ban reasons](https://github.com/dashpay/platform/pull/6023) — 10 unresolved (10 bot) · 0 days stale · ✋ changes requested · 🔴 CI failing · areas: kotlin-sdk · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Add regression coverage for nullable ban-info fields**" — 0 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#6022 fix(sdk)!: read document history on a network below protocol version 15](https://github.com/dashpay/platform/pull/6022) — 8 unresolved (8 bot) · 0 days stale · ✋ changes requested · 🔴 CI failing · areas: rs-drive-abci, rust-sdk, js-wasm-sdk, fallback · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Document the Rust API break without marking it consensus-breaking**" — 0 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/tenderdash#1582 feat(abci)!: add the ApplyBlockRandomness method and commit beacon fields](https://github.com/dashpay/tenderdash/pull/1582) — 1 unresolved (1 bot) · 15 days stale · areas: fallback · Policy: waiting-author
  - Top thread: "✅ **Resolved** at \`cd6b4ec8\`; see the replies below." — 15 days old
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/tenderdash#1597 fix(p2p): trim the simple priority queue below its limit](https://github.com/dashpay/tenderdash/pull/1597) — 1 unresolved (1 bot) · 12 days stale · ⚠ merge conflict · areas: fallback · Policy: waiting-self-review
  - Top thread: "🟡 Suggestion: Count the larger overflow batches**" — 12 days old
  - Blocker: Author must post /self-reviewed fb3e02bfc772facdefade06c2eeca6743e23e329
- [dashpay/platform#6021 fix(drive-abci): refuse disabling a missing key with a consensus error at check_tx](https://github.com/dashpay/platform/pull/6021) — 2 unresolved (2 bot) · 0 days stale · areas: rs-drive-abci · Policy: waiting-self-review
  - Top thread: "🟡 Suggestion: Document the admission change under the legacy fee estimator**" — 0 days old
  - Blocker: Author must post /self-reviewed 0a7c3bfbcbd3dcd28d7cc7703e7d56dcc70e158f
- [dashpay/platform#5968 feat(platform-wallet)!: apps show the shielded balance the wallet announces, without identity accounts](https://github.com/dashpay/platform/pull/5968) — 1 unresolved (1 bot) · 3 days stale · 🔴 CI failing · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Keep JNI balance-trampoline conversion inside the panic guard**" — 3 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix

<a id="quantumexplorer-draft"></a>
#### Draft (2)
- [dashpay/platform#5967 feat(sdk): host apps can prepare an identity's shielded account](https://github.com/dashpay/platform/pull/5967) — 🔴 CI failing · 📝 draft · areas: rs-platform-wallet-ffi, swift-sdk, kotlin-sdk · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/tenderdash#1583 feat(consensus)!: recover a beacon from prevotes and sign precommits over the randomness-applied app hash](https://github.com/dashpay/tenderdash/pull/1583) — ⚠ merge conflict · 📝 draft · areas: fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot

<a id="quantumexplorer-stale"></a>
#### Stale (14)
- [dashpay/platform#5750 feat(platform)!: random token transfer and mint amounts](https://github.com/dashpay/platform/pull/5750) — 13 unresolved (13 bot) · 17 days stale · 🐢 targets claude/platform-tenderdash-randomness-149e4e · areas: rs-drive, dpp, rs-platform-wallet, rust-sdk, rust-sdk-ffi, fallback
  - Top thread: "✅ **Resolved** at \`f8fcc8fd\`; see the replies below." — 17 days old
- [dashpay/platform#5749 feat(platform)!: sys.random.uniform document properties](https://github.com/dashpay/platform/pull/5749) — 9 unresolved (9 bot) · 17 days stale · 🔴 CI failing · 🐢 targets v5.2-dev · areas: rs-drive, rs-drive-abci, dpp, fallback · Policy: waiting-author
  - Top thread: "✅ **Resolved** at \`0ead2b47\`; see the replies below." — 17 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5753 feat(drive-abci)!: apply block randomness through the ApplyBlockRandomness ABCI method](https://github.com/dashpay/platform/pull/5753) — 8 unresolved (8 bot) · 17 days stale · ⚠ merge conflict · 🔴 CI failing · 🐢 targets claude/random-token-amounts · areas: rs-drive, rs-drive-abci, dpp, fallback
  - Top thread: "🟡 Suggestion: Exercise the protocol upgrade with signed commits**" — 17 days old
- [dashpay/platform#5529 feat(platform)!: token marketplace listings (PV15)](https://github.com/dashpay/platform/pull/5529) — 2 unresolved (2 bot) · 27 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · 🐢 targets v5.2-dev · areas: rs-drive, rs-drive-abci, dpp, rs-platform-wallet, swift-sdk, kotlin-sdk, fallback · Policy: waiting-author
  - Top thread: "🔴 Blocking: Version the consensus order-ID derivation through platform-version tables**" — 27 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5530 feat(platform)!: token marketplace offers (PV15)](https://github.com/dashpay/platform/pull/5530) — 1 unresolved (1 bot) · 21 days stale · ✋ changes requested · 🔴 CI failing · 🐢 targets claude/token-market-listings · areas: rs-drive, rs-drive-abci, dpp, rs-platform-wallet, fallback
  - Top thread: "🔴 Blocking: Bound bid escrow by a buyer-signed credit limit**" — 21 days old
- [dashpay/platform#4519 feat(platform-wallet): rotate a masternode's keys into the wallet — ProUpRegTx orchestration, FFI, Swift](https://github.com/dashpay/platform/pull/4519) — ⚠ merge conflict · 🔴 CI failing · 🐢 targets v5.2-dev · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 92a3ebe17adc12432fb756c63f172c663809007d
  - Blocker: Proceeded without thepastaclaw: no review within the configured window
- [dashpay/platform#4730 feat(platform)!: delta-based data contract update transition for protocol version 15](https://github.com/dashpay/platform/pull/4730) — ⚠ merge conflict · 🔴 CI failing · 📝 draft · 🐢 targets v5.2-dev · areas: rs-drive, rs-drive-abci, dpp, rs-platform-wallet, rust-sdk, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#4776 feat(platform-wallet)!: let contract updates clear the description through the FFI](https://github.com/dashpay/platform/pull/4776) — ⚠ merge conflict · 🔴 CI failing · 🐢 targets feat/delta-contract-update-pv15 · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk
- [dashpay/platform#4986 fix(drive-abci)!: pay every reward share of a masternode and credit each identity once in the epoch payout (PV15)](https://github.com/dashpay/platform/pull/4986) — ⚠ merge conflict · 📝 draft · 🐢 targets v5.2-dev · areas: rs-drive-abci, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5113 feat(platform-wallet)!: keep masternode collateral out of coin selection](https://github.com/dashpay/platform/pull/5113) — ⚠ merge conflict · 🔴 CI failing · 📝 draft · 🐢 targets chore/bump-rust-dashcore-secp-033 · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk, fallback
- [dashpay/platform#5531 feat(platform)!: record token marketplace events in token history (PV15)](https://github.com/dashpay/platform/pull/5531) — 🔴 CI failing · 🐢 targets claude/token-market-offers · areas: rs-drive, rs-drive-abci, dpp, system-contracts, fallback
- [dashpay/platform#5532 feat(platform)!: destroy a frozen identity's escrowed listings with its funds (PV15)](https://github.com/dashpay/platform/pull/5532) — 🔴 CI failing · 🐢 targets claude/token-market-history · areas: rs-drive, rs-drive-abci, fallback
- [dashpay/platform#5736 feat(platform)!: add a RedPallas signer key type with its own purpose (PV15)](https://github.com/dashpay/platform/pull/5736) — ⚠ merge conflict · 📝 draft · 🐢 targets v5.2-dev · areas: rs-drive, rs-drive-abci, dpp, rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, rust-sdk-ffi, kotlin-sdk, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5751 feat(sdk): random token amounts in wasm-sdk and js-evo-sdk](https://github.com/dashpay/platform/pull/5751) — 🐢 targets claude/random-token-amounts · areas: js-wasm-sdk

<a id="quantumexplorer-clean"></a>
#### Clean (2)
- [dashpay/platform#6002 fix(dashmate): run an added local node on its network's Core image](https://github.com/dashpay/platform/pull/6002) — areas: dashmate · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 9cc7778487200a5f379f0cb47cfff854e1a520dd
- [dashpay/tenderdash#1584 fix(statesync): restore LastAppHash of the snapshot height, not the next one](https://github.com/dashpay/tenderdash/pull/1584) — areas: fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 23c1443e4077c7c7dfedf00f9446c762e669660f

<a id="quantumexplorer-ready-for-review"></a>
#### Ready for Review (3)
- [dashpay/rust-dashcore#1370 fix(dash-spv)!: settle filter requests on network arrival](https://github.com/dashpay/rust-dashcore/pull/1370) — by @xdustinface · areas: dash-spv · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 51a854b726cc7324fc4673a5808cbeb7908f869e
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/rust-dashcore#1458 ci: run core, wallet and rpc tests in one job](https://github.com/dashpay/rust-dashcore/pull/1458) — by @ZocoLini · areas: github, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed fa7f0830a9103080e90894df3e05a21a6777d75b
- [dashpay/rust-dashcore#1460 ci: cache dependencies in sanitizer jobs](https://github.com/dashpay/rust-dashcore/pull/1460) — by @ZocoLini · areas: github, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 9699bbf9ea1edf6ba723c1606fe704043b1591b8

<a id="lklimek"></a>
### @lklimek
<a id="lklimek-open"></a>
#### Open (12)
- [dashpay/platform#5528 fix(platform-wallet): retain drain operation history through funding conflicts](https://github.com/dashpay/platform/pull/5528) — 5 unresolved (5 bot) · 30 days stale · ✋ changes requested · 📝 draft · 🐢 targets feat/sqlite-wallet-restore · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk, kotlin-sdk
  - Top thread: "🟡 Suggestion: Retain diagnostics for malformed FFI operation snapshots**" — 30 days old
- [dashpay/dash-evo-tool#1161 fix(ci): isolate release signing and constrain agent write paths](https://github.com/dashpay/dash-evo-tool/pull/1161) — 15 unresolved (7 CodeRabbit, 8 bot) · 0 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · 📝 draft · areas: github, fallback · Policy: draft
  - Top thread: "&lt;picture&gt;&lt;img src="https://storage.googleapis.com/coderabbit_public_assets/review-comment-icons-v1/amber-16.svg" width="…" — 0 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/tenderdash#1515 fix(service)!: join remaining service workers before shutdown completes](https://github.com/dashpay/tenderdash/pull/1515) — 4 unresolved (4 bot) · 4 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · areas: fallback · Policy: waiting-author
  - Top thread: "🔴 Blocking: Serialize reconnect resubscriptions with subscription changes**" — 4 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5519 refactor: remove DPP raw secp256k1 dependency](https://github.com/dashpay/platform/pull/5519) — 1 unresolved (1 bot) · 3 days stale · 🐢 targets v5.2-dev · areas: rs-drive-abci, dpp, rs-platform-wallet, rs-platform-wallet-ffi, rust-sdk, rust-sdk-ffi, js-wasm-sdk, fallback · Policy: ready-for-human
  - Top thread: "🟡 Suggestion: Track the typed RNG constructor in rust-dashcore**" — 3 days old
  - Blocker: Human approval or objection resolution is required
- [dashpay/platform#5210 feat(wallet-storage)!: restore complete Core wallet snapshots from SQLite](https://github.com/dashpay/platform/pull/5210) — ⏸ deferred · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage · Policy: waiting-author
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: coderabbitai left review threads unresolved; resolve them
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5220 fix(platform-wallet)!: replay recorded transaction history on load for every persister](https://github.com/dashpay/platform/pull/5220) — ⏸ deferred · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk, kotlin-sdk, fallback · Policy: waiting-author
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5949 fix(sdk): bump tokio to 1.54.1 and fix the futurelocks it exposes](https://github.com/dashpay/platform/pull/5949) — 🐢 targets v5.2-dev · areas: rs-platform-wallet, rust-sdk, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
- [dashpay/rust-dashcore#1336 fix(dash-spv)!: deliver only verified InstantLocks](https://github.com/dashpay/rust-dashcore/pull/1336) — ⚠ merge conflict · ✋ changes requested · 📝 draft · 🐢 targets fix/dash-spv-instant-send-cycle · areas: dash-spv, github, fallback
- [dashpay/rust-dashcore#1416 feat(key-wallet): verify owner authorization of a pending spend](https://github.com/dashpay/rust-dashcore/pull/1416) — ✋ changes requested · areas: key-wallet, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed d85f215d8cbdd2418e469628351219534edd341c
- [dashpay/rust-dashcore#1419 fix(dash-spv)!: let the engine own InstantSend cycle eligibility](https://github.com/dashpay/rust-dashcore/pull/1419) — ✋ changes requested · 🔴 CI failing · areas: dash-spv · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 02d4d561dc536d0c47c4fd683a5e5dcc1a70562e
- [dashpay/rust-dashcore#1448 fix(dash-spv): keep \`QRInfo\` catch-up across cycles](https://github.com/dashpay/rust-dashcore/pull/1448) — 🔴 CI failing · 📝 draft · areas: dash-spv · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/tenderdash#1593 feat(consensus)!: specify full consensus parameter commitment and remove app version parameter](https://github.com/dashpay/tenderdash/pull/1593) — ⚠ merge conflict · ✋ changes requested · 📝 draft · areas: fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot

<a id="lklimek-needs-action"></a>
#### Needs action (3)
- [dashpay/tenderdash#1515 fix(service)!: join remaining service workers before shutdown completes](https://github.com/dashpay/tenderdash/pull/1515) — 4 unresolved (4 bot) · 4 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · areas: fallback · Policy: waiting-author
  - Top thread: "🔴 Blocking: Serialize reconnect resubscriptions with subscription changes**" — 4 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/rust-dashcore#1416 feat(key-wallet): verify owner authorization of a pending spend](https://github.com/dashpay/rust-dashcore/pull/1416) — ✋ changes requested · areas: key-wallet, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed d85f215d8cbdd2418e469628351219534edd341c
- [dashpay/rust-dashcore#1419 fix(dash-spv)!: let the engine own InstantSend cycle eligibility](https://github.com/dashpay/rust-dashcore/pull/1419) — ✋ changes requested · 🔴 CI failing · areas: dash-spv · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 02d4d561dc536d0c47c4fd683a5e5dcc1a70562e

<a id="lklimek-ready-for-human"></a>
#### Ready for human (2)
- [dashpay/platform#5519 refactor: remove DPP raw secp256k1 dependency](https://github.com/dashpay/platform/pull/5519) — 1 unresolved (1 bot) · 3 days stale · 🐢 targets v5.2-dev · areas: rs-drive-abci, dpp, rs-platform-wallet, rs-platform-wallet-ffi, rust-sdk, rust-sdk-ffi, js-wasm-sdk, fallback · Policy: ready-for-human
  - Top thread: "🟡 Suggestion: Track the typed RNG constructor in rust-dashcore**" — 3 days old
  - Blocker: Human approval or objection resolution is required
- [dashpay/platform#5949 fix(sdk): bump tokio to 1.54.1 and fix the futurelocks it exposes](https://github.com/dashpay/platform/pull/5949) — 🐢 targets v5.2-dev · areas: rs-platform-wallet, rust-sdk, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required

<a id="lklimek-unresolved-comments"></a>
#### Unresolved Comments (1)
- [dashpay/tenderdash#1515 fix(service)!: join remaining service workers before shutdown completes](https://github.com/dashpay/tenderdash/pull/1515) — 4 unresolved (4 bot) · 4 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · areas: fallback · Policy: waiting-author
  - Top thread: "🔴 Blocking: Serialize reconnect resubscriptions with subscription changes**" — 4 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them

<a id="lklimek-changes-requested"></a>
#### Changes Requested (2)
- [dashpay/rust-dashcore#1416 feat(key-wallet): verify owner authorization of a pending spend](https://github.com/dashpay/rust-dashcore/pull/1416) — ✋ changes requested · areas: key-wallet, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed d85f215d8cbdd2418e469628351219534edd341c
- [dashpay/rust-dashcore#1419 fix(dash-spv)!: let the engine own InstantSend cycle eligibility](https://github.com/dashpay/rust-dashcore/pull/1419) — ✋ changes requested · 🔴 CI failing · areas: dash-spv · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 02d4d561dc536d0c47c4fd683a5e5dcc1a70562e

<a id="lklimek-deferred"></a>
#### Deferred (2)
- [dashpay/platform#5210 feat(wallet-storage)!: restore complete Core wallet snapshots from SQLite](https://github.com/dashpay/platform/pull/5210) — ⏸ deferred · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage · Policy: waiting-author
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: coderabbitai left review threads unresolved; resolve them
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5220 fix(platform-wallet)!: replay recorded transaction history on load for every persister](https://github.com/dashpay/platform/pull/5220) — ⏸ deferred · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk, kotlin-sdk, fallback · Policy: waiting-author
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them

<a id="lklimek-draft"></a>
#### Draft (3)
- [dashpay/dash-evo-tool#1161 fix(ci): isolate release signing and constrain agent write paths](https://github.com/dashpay/dash-evo-tool/pull/1161) — 15 unresolved (7 CodeRabbit, 8 bot) · 0 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · 📝 draft · areas: github, fallback · Policy: draft
  - Top thread: "&lt;picture&gt;&lt;img src="https://storage.googleapis.com/coderabbit_public_assets/review-comment-icons-v1/amber-16.svg" width="…" — 0 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#1448 fix(dash-spv): keep \`QRInfo\` catch-up across cycles](https://github.com/dashpay/rust-dashcore/pull/1448) — 🔴 CI failing · 📝 draft · areas: dash-spv · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/tenderdash#1593 feat(consensus)!: specify full consensus parameter commitment and remove app version parameter](https://github.com/dashpay/tenderdash/pull/1593) — ⚠ merge conflict · ✋ changes requested · 📝 draft · areas: fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot

<a id="lklimek-stale"></a>
#### Stale (4)
- [dashpay/platform#5528 fix(platform-wallet): retain drain operation history through funding conflicts](https://github.com/dashpay/platform/pull/5528) — 5 unresolved (5 bot) · 30 days stale · ✋ changes requested · 📝 draft · 🐢 targets feat/sqlite-wallet-restore · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk, kotlin-sdk
  - Top thread: "🟡 Suggestion: Retain diagnostics for malformed FFI operation snapshots**" — 30 days old
- [dashpay/platform#5519 refactor: remove DPP raw secp256k1 dependency](https://github.com/dashpay/platform/pull/5519) — 1 unresolved (1 bot) · 3 days stale · 🐢 targets v5.2-dev · areas: rs-drive-abci, dpp, rs-platform-wallet, rs-platform-wallet-ffi, rust-sdk, rust-sdk-ffi, js-wasm-sdk, fallback · Policy: ready-for-human
  - Top thread: "🟡 Suggestion: Track the typed RNG constructor in rust-dashcore**" — 3 days old
  - Blocker: Human approval or objection resolution is required
- [dashpay/platform#5949 fix(sdk): bump tokio to 1.54.1 and fix the futurelocks it exposes](https://github.com/dashpay/platform/pull/5949) — 🐢 targets v5.2-dev · areas: rs-platform-wallet, rust-sdk, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
- [dashpay/rust-dashcore#1336 fix(dash-spv)!: deliver only verified InstantLocks](https://github.com/dashpay/rust-dashcore/pull/1336) — ⚠ merge conflict · ✋ changes requested · 📝 draft · 🐢 targets fix/dash-spv-instant-send-cycle · areas: dash-spv, github, fallback

<a id="shumkov"></a>
### @shumkov
<a id="shumkov-open"></a>
#### Open (6)
- [dashpay/rust-dashcore#634 feat(key-wallet-manager): single-map WalletManager with Arc&lt;RwLock&lt;T&gt;&gt; per wallet](https://github.com/dashpay/rust-dashcore/pull/634) — 20 unresolved (20 CodeRabbit) · 227 days stale · ⚠ merge conflict · ✋ changes requested · areas: dash-spv, key-wallet, key-wallet-manager, fallback · Policy: waiting-author
  - Top thread: "_⚠️ Potential issue_ \| _🟡 Minor_" — 227 days old
  - Blocker: coderabbitai requested changes on this head; dismiss the review or push a fix
  - Blocker: coderabbitai left review threads unresolved; resolve them
- [dashpay/platform#5656 fix(dashmate): build local images for every group node that builds any](https://github.com/dashpay/platform/pull/5656) — 1 unresolved (1 bot) · 22 days stale · ✋ changes requested · 🔴 CI failing · 🐢 targets fix/dashmate/update-path-bugs · areas: dashmate
  - Top thread: "🔴 Blocking: Build the shared base image before dependent service images across group configurations**" — 22 days old
- [dashpay/platform#4283 fix(dashmate): keep the node up when an image pull fails, and stop reporting success when it did not](https://github.com/dashpay/platform/pull/4283) — ⚠ merge conflict · 🔴 CI failing · 🐢 targets v5.2-dev · areas: dashmate · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 3ceb3315ea9b383ae5816ea4f0e9961202770ed9
  - Blocker: Proceeded without thepastaclaw: no review within the configured window
- [dashpay/platform#5763 docs: e2e test architecture and legacy JS retirement plans (draft, not for merge)](https://github.com/dashpay/platform/pull/5763) — 📝 draft · 🐢 targets v5.2-dev · areas: fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5819 feat(dashmate): add the modules that let dashmate update itself](https://github.com/dashpay/platform/pull/5819) — 📝 draft · 🐢 targets fix/dashmate/update-path-bugs · areas: dashmate
- [dashpay/platform#6013 fix(platform)!: close the gaps left in nested member update rules (PV15)](https://github.com/dashpay/platform/pull/6013) — ✋ changes requested · areas: rs-drive-abci, dpp, fallback · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head

<a id="shumkov-needs-action"></a>
#### Needs action (2)
- [dashpay/rust-dashcore#634 feat(key-wallet-manager): single-map WalletManager with Arc&lt;RwLock&lt;T&gt;&gt; per wallet](https://github.com/dashpay/rust-dashcore/pull/634) — 20 unresolved (20 CodeRabbit) · 227 days stale · ⚠ merge conflict · ✋ changes requested · areas: dash-spv, key-wallet, key-wallet-manager, fallback · Policy: waiting-author
  - Top thread: "_⚠️ Potential issue_ \| _🟡 Minor_" — 227 days old
  - Blocker: coderabbitai requested changes on this head; dismiss the review or push a fix
  - Blocker: coderabbitai left review threads unresolved; resolve them
- [dashpay/platform#6013 fix(platform)!: close the gaps left in nested member update rules (PV15)](https://github.com/dashpay/platform/pull/6013) — ✋ changes requested · areas: rs-drive-abci, dpp, fallback · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head

<a id="shumkov-unresolved-comments"></a>
#### Unresolved Comments (1)
- [dashpay/rust-dashcore#634 feat(key-wallet-manager): single-map WalletManager with Arc&lt;RwLock&lt;T&gt;&gt; per wallet](https://github.com/dashpay/rust-dashcore/pull/634) — 20 unresolved (20 CodeRabbit) · 227 days stale · ⚠ merge conflict · ✋ changes requested · areas: dash-spv, key-wallet, key-wallet-manager, fallback · Policy: waiting-author
  - Top thread: "_⚠️ Potential issue_ \| _🟡 Minor_" — 227 days old
  - Blocker: coderabbitai requested changes on this head; dismiss the review or push a fix
  - Blocker: coderabbitai left review threads unresolved; resolve them

<a id="shumkov-changes-requested"></a>
#### Changes Requested (1)
- [dashpay/platform#6013 fix(platform)!: close the gaps left in nested member update rules (PV15)](https://github.com/dashpay/platform/pull/6013) — ✋ changes requested · areas: rs-drive-abci, dpp, fallback · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head

<a id="shumkov-stale"></a>
#### Stale (4)
- [dashpay/platform#5656 fix(dashmate): build local images for every group node that builds any](https://github.com/dashpay/platform/pull/5656) — 1 unresolved (1 bot) · 22 days stale · ✋ changes requested · 🔴 CI failing · 🐢 targets fix/dashmate/update-path-bugs · areas: dashmate
  - Top thread: "🔴 Blocking: Build the shared base image before dependent service images across group configurations**" — 22 days old
- [dashpay/platform#4283 fix(dashmate): keep the node up when an image pull fails, and stop reporting success when it did not](https://github.com/dashpay/platform/pull/4283) — ⚠ merge conflict · 🔴 CI failing · 🐢 targets v5.2-dev · areas: dashmate · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 3ceb3315ea9b383ae5816ea4f0e9961202770ed9
  - Blocker: Proceeded without thepastaclaw: no review within the configured window
- [dashpay/platform#5763 docs: e2e test architecture and legacy JS retirement plans (draft, not for merge)](https://github.com/dashpay/platform/pull/5763) — 📝 draft · 🐢 targets v5.2-dev · areas: fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5819 feat(dashmate): add the modules that let dashmate update itself](https://github.com/dashpay/platform/pull/5819) — 📝 draft · 🐢 targets fix/dashmate/update-path-bugs · areas: dashmate

<a id="shumkov-ready-for-review"></a>
#### Ready for Review (5)
- [dashpay/platform#6002 fix(dashmate): run an added local node on its network's Core image](https://github.com/dashpay/platform/pull/6002) — by @QuantumExplorer · areas: dashmate · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 9cc7778487200a5f379f0cb47cfff854e1a520dd
- [dashpay/rust-dashcore#1458 ci: run core, wallet and rpc tests in one job](https://github.com/dashpay/rust-dashcore/pull/1458) — by @ZocoLini · areas: github, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed fa7f0830a9103080e90894df3e05a21a6777d75b
- [dashpay/rust-dashcore#1459 ci: merge audit and verify jobs into one](https://github.com/dashpay/rust-dashcore/pull/1459) — by @ZocoLini · areas: github · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed e20edef48b4122dad05e5bc08ab8b19f190c0f38
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/rust-dashcore#1460 ci: cache dependencies in sanitizer jobs](https://github.com/dashpay/rust-dashcore/pull/1460) — by @ZocoLini · areas: github, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 9699bbf9ea1edf6ba723c1606fe704043b1591b8
- [dashpay/tenderdash#1584 fix(statesync): restore LastAppHash of the snapshot height, not the next one](https://github.com/dashpay/tenderdash/pull/1584) — by @QuantumExplorer · areas: fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 23c1443e4077c7c7dfedf00f9446c762e669660f

<a id="xdustinface"></a>
### @xdustinface
<a id="xdustinface-open"></a>
#### Open (10)
- [dashpay/platform#5791 fix(platform-wallet): refuse a quorum type that does not fit a byte](https://github.com/dashpay/platform/pull/5791) — 2 unresolved (1 CodeRabbit, 1 bot) · 14 days stale · ⚠ merge conflict · 🐢 targets v5.2-dev · areas: rs-platform-wallet · Policy: waiting-author
  - Top thread: "&lt;picture&gt;&lt;img src="https://storage.googleapis.com/coderabbit_public_assets/review-comment-icons-v1/amber-16.svg" width="…" — 14 days old
  - Blocker: coderabbitai left review threads unresolved; resolve them
- [dashpay/rust-dashcore#1461 fix(dash-spv): request a rejected filter batch again](https://github.com/dashpay/rust-dashcore/pull/1461) — 1 unresolved (1 CodeRabbit) · 0 days stale · ✋ changes requested · 🔴 CI failing · areas: dash-spv · Policy: waiting-author
  - Top thread: "&lt;picture&gt;&lt;img src="https://storage.googleapis.com/coderabbit_public_assets/review-comment-icons-v1/major-16.svg" width="…" — 0 days old
  - Blocker: coderabbitai requested changes on this head; dismiss the review or push a fix
  - Blocker: coderabbitai left review threads unresolved; resolve them
- [dashpay/rust-dashcore#541 ci: integrate Codecov test analytics via \`cargo-nextest\`](https://github.com/dashpay/rust-dashcore/pull/541) — ⚠ merge conflict · 📝 draft · areas: github, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#1370 fix(dash-spv)!: settle filter requests on network arrival](https://github.com/dashpay/rust-dashcore/pull/1370) — areas: dash-spv · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 51a854b726cc7324fc4673a5808cbeb7908f869e
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/rust-dashcore#1371 fix(dash-spv): size byte caps from the served rate](https://github.com/dashpay/rust-dashcore/pull/1371) — ⚠ merge conflict · 📝 draft · 🐢 targets fix/cfilters-settle-on-arrival · areas: dash-spv
- [dashpay/rust-dashcore#1372 fix(dash-spv): spread peers across network groups](https://github.com/dashpay/rust-dashcore/pull/1372) — 📝 draft · 🐢 targets fix/rate-sized-cap · areas: dash-spv
- [dashpay/rust-dashcore#1373 fix(dash-spv): rotate the slowest peer of a slow set](https://github.com/dashpay/rust-dashcore/pull/1373) — 📝 draft · 🐢 targets fix/peer-diversity · areas: dash-spv
- [dashpay/rust-dashcore#1374 fix(dash-spv): retry a refused request at another peer](https://github.com/dashpay/rust-dashcore/pull/1374) — 📝 draft · 🐢 targets fix/peer-exploration · areas: dash-spv
- [dashpay/rust-dashcore#1375 fix(dash-spv): rank a stalled peer last when routing](https://github.com/dashpay/rust-dashcore/pull/1375) — 📝 draft · 🐢 targets fix/peer-exploration · areas: dash-spv
- [dashpay/rust-dashcore#1406 ci: publish a release when it merges into \`master\`](https://github.com/dashpay/rust-dashcore/pull/1406) — 🔴 CI failing · 📝 draft · areas: github, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot

<a id="xdustinface-needs-action"></a>
#### Needs action (1)
- [dashpay/rust-dashcore#1461 fix(dash-spv): request a rejected filter batch again](https://github.com/dashpay/rust-dashcore/pull/1461) — 1 unresolved (1 CodeRabbit) · 0 days stale · ✋ changes requested · 🔴 CI failing · areas: dash-spv · Policy: waiting-author
  - Top thread: "&lt;picture&gt;&lt;img src="https://storage.googleapis.com/coderabbit_public_assets/review-comment-icons-v1/major-16.svg" width="…" — 0 days old
  - Blocker: coderabbitai requested changes on this head; dismiss the review or push a fix
  - Blocker: coderabbitai left review threads unresolved; resolve them

<a id="xdustinface-unresolved-comments"></a>
#### Unresolved Comments (1)
- [dashpay/rust-dashcore#1461 fix(dash-spv): request a rejected filter batch again](https://github.com/dashpay/rust-dashcore/pull/1461) — 1 unresolved (1 CodeRabbit) · 0 days stale · ✋ changes requested · 🔴 CI failing · areas: dash-spv · Policy: waiting-author
  - Top thread: "&lt;picture&gt;&lt;img src="https://storage.googleapis.com/coderabbit_public_assets/review-comment-icons-v1/major-16.svg" width="…" — 0 days old
  - Blocker: coderabbitai requested changes on this head; dismiss the review or push a fix
  - Blocker: coderabbitai left review threads unresolved; resolve them

<a id="xdustinface-draft"></a>
#### Draft (2)
- [dashpay/rust-dashcore#541 ci: integrate Codecov test analytics via \`cargo-nextest\`](https://github.com/dashpay/rust-dashcore/pull/541) — ⚠ merge conflict · 📝 draft · areas: github, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#1406 ci: publish a release when it merges into \`master\`](https://github.com/dashpay/rust-dashcore/pull/1406) — 🔴 CI failing · 📝 draft · areas: github, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot

<a id="xdustinface-stale"></a>
#### Stale (6)
- [dashpay/platform#5791 fix(platform-wallet): refuse a quorum type that does not fit a byte](https://github.com/dashpay/platform/pull/5791) — 2 unresolved (1 CodeRabbit, 1 bot) · 14 days stale · ⚠ merge conflict · 🐢 targets v5.2-dev · areas: rs-platform-wallet · Policy: waiting-author
  - Top thread: "&lt;picture&gt;&lt;img src="https://storage.googleapis.com/coderabbit_public_assets/review-comment-icons-v1/amber-16.svg" width="…" — 14 days old
  - Blocker: coderabbitai left review threads unresolved; resolve them
- [dashpay/rust-dashcore#1371 fix(dash-spv): size byte caps from the served rate](https://github.com/dashpay/rust-dashcore/pull/1371) — ⚠ merge conflict · 📝 draft · 🐢 targets fix/cfilters-settle-on-arrival · areas: dash-spv
- [dashpay/rust-dashcore#1372 fix(dash-spv): spread peers across network groups](https://github.com/dashpay/rust-dashcore/pull/1372) — 📝 draft · 🐢 targets fix/rate-sized-cap · areas: dash-spv
- [dashpay/rust-dashcore#1373 fix(dash-spv): rotate the slowest peer of a slow set](https://github.com/dashpay/rust-dashcore/pull/1373) — 📝 draft · 🐢 targets fix/peer-diversity · areas: dash-spv
- [dashpay/rust-dashcore#1374 fix(dash-spv): retry a refused request at another peer](https://github.com/dashpay/rust-dashcore/pull/1374) — 📝 draft · 🐢 targets fix/peer-exploration · areas: dash-spv
- [dashpay/rust-dashcore#1375 fix(dash-spv): rank a stalled peer last when routing](https://github.com/dashpay/rust-dashcore/pull/1375) — 📝 draft · 🐢 targets fix/peer-exploration · areas: dash-spv

<a id="xdustinface-clean"></a>
#### Clean (1)
- [dashpay/rust-dashcore#1370 fix(dash-spv)!: settle filter requests on network arrival](https://github.com/dashpay/rust-dashcore/pull/1370) — areas: dash-spv · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 51a854b726cc7324fc4673a5808cbeb7908f869e
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return

<a id="xdustinface-ready-for-review"></a>
#### Ready for Review (3)
- [dashpay/rust-dashcore#1458 ci: run core, wallet and rpc tests in one job](https://github.com/dashpay/rust-dashcore/pull/1458) — by @ZocoLini · areas: github, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed fa7f0830a9103080e90894df3e05a21a6777d75b
- [dashpay/rust-dashcore#1459 ci: merge audit and verify jobs into one](https://github.com/dashpay/rust-dashcore/pull/1459) — by @ZocoLini · areas: github · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed e20edef48b4122dad05e5bc08ab8b19f190c0f38
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/rust-dashcore#1460 ci: cache dependencies in sanitizer jobs](https://github.com/dashpay/rust-dashcore/pull/1460) — by @ZocoLini · areas: github, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 9699bbf9ea1edf6ba723c1606fe704043b1591b8

<a id="zocolini"></a>
### @ZocoLini
<a id="zocolini-open"></a>
#### Open (8)
- [dashpay/rust-dashcore#1426 fix(dash-spv): tell an empty filter header store from genesis](https://github.com/dashpay/rust-dashcore/pull/1426) — 3 unresolved (3 CodeRabbit) · 3 days stale · ✋ changes requested · areas: dash-spv · Policy: waiting-author
  - Top thread: "&lt;picture&gt;&lt;img src="https://storage.googleapis.com/coderabbit_public_assets/review-comment-icons-v1/amber-16.svg" width="…" — 3 days old
  - Blocker: coderabbitai requested changes on this head; dismiss the review or push a fix
  - Blocker: coderabbitai left review threads unresolved; resolve them
- [dashpay/rust-dashcore#1199 feat(key-wallet)!: only final coins fund a build](https://github.com/dashpay/rust-dashcore/pull/1199) — ⚠ merge conflict · 📝 draft · areas: dash-spv, key-wallet, key-wallet-manager · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#1328 refactor(key-wallet)!: validate child indexes at construction with \`ChildIndex\`](https://github.com/dashpay/rust-dashcore/pull/1328) — ⚠ merge conflict · 📝 draft · areas: dash-spv, key-wallet, key-wallet-manager · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#1398 fix(dash-spv): detect forks with a full block locator](https://github.com/dashpay/rust-dashcore/pull/1398) — ✋ changes requested · 🔴 CI failing · 📝 draft · areas: dash-spv · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#1421 fix(dash-spv): propagate storage errors from sync paths](https://github.com/dashpay/rust-dashcore/pull/1421) — 🔴 CI failing · 📝 draft · areas: dash-spv, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#1458 ci: run core, wallet and rpc tests in one job](https://github.com/dashpay/rust-dashcore/pull/1458) — areas: github, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed fa7f0830a9103080e90894df3e05a21a6777d75b
- [dashpay/rust-dashcore#1459 ci: merge audit and verify jobs into one](https://github.com/dashpay/rust-dashcore/pull/1459) — areas: github · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed e20edef48b4122dad05e5bc08ab8b19f190c0f38
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/rust-dashcore#1460 ci: cache dependencies in sanitizer jobs](https://github.com/dashpay/rust-dashcore/pull/1460) — areas: github, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 9699bbf9ea1edf6ba723c1606fe704043b1591b8

<a id="zocolini-needs-action"></a>
#### Needs action (1)
- [dashpay/rust-dashcore#1426 fix(dash-spv): tell an empty filter header store from genesis](https://github.com/dashpay/rust-dashcore/pull/1426) — 3 unresolved (3 CodeRabbit) · 3 days stale · ✋ changes requested · areas: dash-spv · Policy: waiting-author
  - Top thread: "&lt;picture&gt;&lt;img src="https://storage.googleapis.com/coderabbit_public_assets/review-comment-icons-v1/amber-16.svg" width="…" — 3 days old
  - Blocker: coderabbitai requested changes on this head; dismiss the review or push a fix
  - Blocker: coderabbitai left review threads unresolved; resolve them

<a id="zocolini-unresolved-comments"></a>
#### Unresolved Comments (1)
- [dashpay/rust-dashcore#1426 fix(dash-spv): tell an empty filter header store from genesis](https://github.com/dashpay/rust-dashcore/pull/1426) — 3 unresolved (3 CodeRabbit) · 3 days stale · ✋ changes requested · areas: dash-spv · Policy: waiting-author
  - Top thread: "&lt;picture&gt;&lt;img src="https://storage.googleapis.com/coderabbit_public_assets/review-comment-icons-v1/amber-16.svg" width="…" — 3 days old
  - Blocker: coderabbitai requested changes on this head; dismiss the review or push a fix
  - Blocker: coderabbitai left review threads unresolved; resolve them

<a id="zocolini-draft"></a>
#### Draft (4)
- [dashpay/rust-dashcore#1199 feat(key-wallet)!: only final coins fund a build](https://github.com/dashpay/rust-dashcore/pull/1199) — ⚠ merge conflict · 📝 draft · areas: dash-spv, key-wallet, key-wallet-manager · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#1328 refactor(key-wallet)!: validate child indexes at construction with \`ChildIndex\`](https://github.com/dashpay/rust-dashcore/pull/1328) — ⚠ merge conflict · 📝 draft · areas: dash-spv, key-wallet, key-wallet-manager · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#1398 fix(dash-spv): detect forks with a full block locator](https://github.com/dashpay/rust-dashcore/pull/1398) — ✋ changes requested · 🔴 CI failing · 📝 draft · areas: dash-spv · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#1421 fix(dash-spv): propagate storage errors from sync paths](https://github.com/dashpay/rust-dashcore/pull/1421) — 🔴 CI failing · 📝 draft · areas: dash-spv, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot

<a id="zocolini-clean"></a>
#### Clean (3)
- [dashpay/rust-dashcore#1458 ci: run core, wallet and rpc tests in one job](https://github.com/dashpay/rust-dashcore/pull/1458) — areas: github, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed fa7f0830a9103080e90894df3e05a21a6777d75b
- [dashpay/rust-dashcore#1459 ci: merge audit and verify jobs into one](https://github.com/dashpay/rust-dashcore/pull/1459) — areas: github · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed e20edef48b4122dad05e5bc08ab8b19f190c0f38
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/rust-dashcore#1460 ci: cache dependencies in sanitizer jobs](https://github.com/dashpay/rust-dashcore/pull/1460) — areas: github, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 9699bbf9ea1edf6ba723c1606fe704043b1591b8

<a id="zocolini-ready-for-review"></a>
#### Ready for Review (1)
- [dashpay/rust-dashcore#1370 fix(dash-spv)!: settle filter requests on network arrival](https://github.com/dashpay/rust-dashcore/pull/1370) — by @xdustinface · areas: dash-spv · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 51a854b726cc7324fc4673a5808cbeb7908f869e
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return

<a id="ktechmidas"></a>
### @ktechmidas
<a id="ktechmidas-open"></a>
#### Open (6)
- [dashpay/platform#6018 ci: move CI Docker images from ECR to GHCR (saves about $265 a month)](https://github.com/dashpay/platform/pull/6018) — 5 unresolved (5 bot) · 0 days stale · ✋ changes requested · 🔴 CI failing · 🐢 targets v5.2-dev · areas: github, fallback · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Distinguish nightly ECR images from GHCR images on migrated branches**" — 0 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5188 ci: tolerate optional S3 cache export outages on chore/bump-rust-dashcore-secp-033](https://github.com/dashpay/platform/pull/5188) — 1 unresolved (1 bot) · 53 days stale · 🐢 targets chore/bump-rust-dashcore-secp-033 · areas: github
  - Top thread: "🟡 Suggestion: New regression tests are not executed by any CI workflow on this branch**" — 53 days old
- [dashpay/platform#5186 ci: tolerate optional S3 cache export outages on v5.0-dev](https://github.com/dashpay/platform/pull/5186) — 🔴 CI failing · 🐢 targets v6.0-dev · areas: github · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 843052c2dce26a80065c15b1219bbc8211264161
- [dashpay/platform#5198 ci: bound default Binaryen optimizer threads in NPM builds](https://github.com/dashpay/platform/pull/5198) — 🐢 targets v6.0-dev · areas: github · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 23b41b477bc038965b686b4a75412b1c21fc6d98
- [dashpay/platform#5205 fix(release): install verified cargo-binstall binary directly](https://github.com/dashpay/platform/pull/5205) — 🐢 targets v6.0-dev · areas: fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 8a8ac2c788b5338416e18045001d9115ba3ac80a
- [dashpay/platform#5866 feat(dashmate): clean up historical Docker images after updates](https://github.com/dashpay/platform/pull/5866) — ✋ changes requested · 🔴 CI failing · 🐢 targets v5.2-dev · areas: dashmate · Policy: waiting-author
  - Blocker: Author response is required after the latest human objection

<a id="ktechmidas-stale"></a>
#### Stale (6)
- [dashpay/platform#6018 ci: move CI Docker images from ECR to GHCR (saves about $265 a month)](https://github.com/dashpay/platform/pull/6018) — 5 unresolved (5 bot) · 0 days stale · ✋ changes requested · 🔴 CI failing · 🐢 targets v5.2-dev · areas: github, fallback · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Distinguish nightly ECR images from GHCR images on migrated branches**" — 0 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5188 ci: tolerate optional S3 cache export outages on chore/bump-rust-dashcore-secp-033](https://github.com/dashpay/platform/pull/5188) — 1 unresolved (1 bot) · 53 days stale · 🐢 targets chore/bump-rust-dashcore-secp-033 · areas: github
  - Top thread: "🟡 Suggestion: New regression tests are not executed by any CI workflow on this branch**" — 53 days old
- [dashpay/platform#5186 ci: tolerate optional S3 cache export outages on v5.0-dev](https://github.com/dashpay/platform/pull/5186) — 🔴 CI failing · 🐢 targets v6.0-dev · areas: github · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 843052c2dce26a80065c15b1219bbc8211264161
- [dashpay/platform#5198 ci: bound default Binaryen optimizer threads in NPM builds](https://github.com/dashpay/platform/pull/5198) — 🐢 targets v6.0-dev · areas: github · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 23b41b477bc038965b686b4a75412b1c21fc6d98
- [dashpay/platform#5205 fix(release): install verified cargo-binstall binary directly](https://github.com/dashpay/platform/pull/5205) — 🐢 targets v6.0-dev · areas: fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 8a8ac2c788b5338416e18045001d9115ba3ac80a
- [dashpay/platform#5866 feat(dashmate): clean up historical Docker images after updates](https://github.com/dashpay/platform/pull/5866) — ✋ changes requested · 🔴 CI failing · 🐢 targets v5.2-dev · areas: dashmate · Policy: waiting-author
  - Blocker: Author response is required after the latest human objection

<a id="ktechmidas-ready-for-review"></a>
#### Ready for Review (4)
- [dashpay/platform#6002 fix(dashmate): run an added local node on its network's Core image](https://github.com/dashpay/platform/pull/6002) — by @QuantumExplorer · areas: dashmate · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 9cc7778487200a5f379f0cb47cfff854e1a520dd
- [dashpay/rust-dashcore#1458 ci: run core, wallet and rpc tests in one job](https://github.com/dashpay/rust-dashcore/pull/1458) — by @ZocoLini · areas: github, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed fa7f0830a9103080e90894df3e05a21a6777d75b
- [dashpay/rust-dashcore#1459 ci: merge audit and verify jobs into one](https://github.com/dashpay/rust-dashcore/pull/1459) — by @ZocoLini · areas: github · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed e20edef48b4122dad05e5bc08ab8b19f190c0f38
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/rust-dashcore#1460 ci: cache dependencies in sanitizer jobs](https://github.com/dashpay/rust-dashcore/pull/1460) — by @ZocoLini · areas: github, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 9699bbf9ea1edf6ba723c1606fe704043b1591b8

<a id="pastapastapasta"></a>
### @PastaPastaPasta
<a id="pastapastapasta-open"></a>
#### Open (24)
- [dashpay/platform#5709 feat(platform-wallet): own the shielded-tip payment decisions in the wallet](https://github.com/dashpay/platform/pull/5709) — 22 unresolved (22 bot) · 19 days stale · ⚠ merge conflict · 📝 draft · 🐢 targets feat/dashpay-shielded-tips · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk, rust-sdk, kotlin-sdk, fallback
  - Top thread: "✅ **Resolved** at \`688c75e1\`; see the replies below." — 19 days old
- [dashpay/platform#4648 feat(drive-abci)!: state sync via ABCI snapshots with reduced platform state (protocol v15)](https://github.com/dashpay/platform/pull/4648) — 8 unresolved (8 bot) · 47 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · 📝 draft · 🐢 targets v5.2-dev · areas: rs-drive, rs-drive-abci, dpp, fallback · Policy: draft
  - Top thread: "🟡 Suggestion: Continue the restored target through consensus and a restart**" — 47 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5934 fix(platform-wallet): own the shielded coordinator and wipe host rows inside Clear](https://github.com/dashpay/platform/pull/5934) — 7 unresolved (7 bot) · 6 days stale · ⚠ merge conflict · 🔴 CI failing · 📝 draft · 🐢 targets v5.2-dev · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk, kotlin-sdk · Policy: draft
  - Top thread: "✅ **Withdrawn** at \`2352c3c6\`; see the replies below." — 6 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5711 feat: shielded-tip screens over the wallet's tip decisions](https://github.com/dashpay/platform/pull/5711) — 4 unresolved (4 bot) · 19 days stale · 📝 draft · 🐢 targets feat/dashpay-shielded-tips-wallet-policy · areas: swift-sdk, kotlin-sdk
  - Top thread: "✅ **Resolved** at \`451dab59\`; see the replies below." — 19 days old
- [dashpay/platform#5786 feat(sdk): tri-state quorum lookups, bounded funding retries and a DPNS contest policy in dash-sdk-cxx](https://github.com/dashpay/platform/pull/5786) — 3 unresolved (3 bot) · 14 days stale · ⚠ merge conflict · 📝 draft · 🐢 targets v5.2-dev · areas: rs-drive, rs-platform-wallet, rust-sdk, js-wasm-sdk, fallback · Policy: draft
  - Top thread: "✅ **Resolved** at \`e46d43eb\`; see the replies below." — 14 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5280 perf(platform-wallet)!: reuse a recent recorded-anchor set on the shielded spend path](https://github.com/dashpay/platform/pull/5280) — 7 unresolved (7 bot) · 0 days stale · ✋ changes requested · 🔴 CI failing · 🐢 targets v5.2-dev · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, fallback · Policy: waiting-author
  - Top thread: "🔴 Blocking: Bind cached anchor evidence to the active verification policy**" — 0 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#4965 test(sdk): cargo-fuzz harness over drive-proof-verifier FromProof](https://github.com/dashpay/platform/pull/4965) — 1 unresolved (1 bot) · 45 days stale · 📝 draft · 🐢 targets v5.2-dev · areas: github, fallback · Policy: draft
  - Top thread: "🟡 Suggestion: Correct the claim that PV13 reaches the V0 proof decoder**" — 45 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5288 feat(platform-wallet): support DashPay shielded tips with dedicated accounts](https://github.com/dashpay/platform/pull/5288) — 1 unresolved (1 bot) · 4 days stale · ⚠ merge conflict · 🔴 CI failing · 🐢 targets v5.2-dev · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk, rust-sdk, kotlin-sdk, fallback · Policy: waiting-author
  - Top thread: "⏭️ **Deferred** (maintainer decision; not blocking; tracked in https://github.com/dashpay/platform/issues/5950) at \`e14a…" — 4 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
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
- [dashpay/platform#5793 fix(platform-wallet): make SpvRuntime::header_hash_at public](https://github.com/dashpay/platform/pull/5793) — 🐢 targets v5.2-dev · areas: rs-platform-wallet · Policy: ready-to-merge
  - Blocker: All policy requirements are satisfied
- [dashpay/platform#5794 feat(platform-wallet-storage): read one asset lock's status by outpoint](https://github.com/dashpay/platform/pull/5794) — 🐢 targets v5.2-dev · areas: wallet-storage · Policy: too-many-open-prs
  - Blocker: Human review is still required; automatic reviewer requests are paused until a review slot is available
- [dashpay/platform#5802 fix(platform-wallet-storage): don't look for or count non-funds used addresses as unresolved](https://github.com/dashpay/platform/pull/5802) — ⚠ merge conflict · 📝 draft · 🐢 targets v5.2-dev · areas: wallet-storage, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#1456 fix(key-wallet): extend address windows before confirming](https://github.com/dashpay/rust-dashcore/pull/1456) — 🔴 CI failing · areas: key-wallet · Policy: waiting-build
  - Blocker: The build must pass before a human is asked

<a id="pastapastapasta-ready-for-human"></a>
#### Ready for human (1)
- [dashpay/platform#5793 fix(platform-wallet): make SpvRuntime::header_hash_at public](https://github.com/dashpay/platform/pull/5793) — 🐢 targets v5.2-dev · areas: rs-platform-wallet · Policy: ready-to-merge
  - Blocker: All policy requirements are satisfied

<a id="pastapastapasta-ci-failing"></a>
#### CI Failing (1)
- [dashpay/rust-dashcore#1456 fix(key-wallet): extend address windows before confirming](https://github.com/dashpay/rust-dashcore/pull/1456) — 🔴 CI failing · areas: key-wallet · Policy: waiting-build
  - Blocker: The build must pass before a human is asked

<a id="pastapastapasta-draft"></a>
#### Draft (2)
- [dashpay/grovedb#1007 build(deps)!: use dashpay/orchard with the faster prover](https://github.com/dashpay/grovedb/pull/1007) — 🔴 CI failing · 📝 draft · areas: fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/grovedb#1054 feat: LigninDB storage engine (tracking, do not merge)](https://github.com/dashpay/grovedb/pull/1054) — 🔴 CI failing · 📝 draft · areas: github, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot

<a id="pastapastapasta-stale"></a>
#### Stale (21)
- [dashpay/platform#5709 feat(platform-wallet): own the shielded-tip payment decisions in the wallet](https://github.com/dashpay/platform/pull/5709) — 22 unresolved (22 bot) · 19 days stale · ⚠ merge conflict · 📝 draft · 🐢 targets feat/dashpay-shielded-tips · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk, rust-sdk, kotlin-sdk, fallback
  - Top thread: "✅ **Resolved** at \`688c75e1\`; see the replies below." — 19 days old
- [dashpay/platform#4648 feat(drive-abci)!: state sync via ABCI snapshots with reduced platform state (protocol v15)](https://github.com/dashpay/platform/pull/4648) — 8 unresolved (8 bot) · 47 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · 📝 draft · 🐢 targets v5.2-dev · areas: rs-drive, rs-drive-abci, dpp, fallback · Policy: draft
  - Top thread: "🟡 Suggestion: Continue the restored target through consensus and a restart**" — 47 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5934 fix(platform-wallet): own the shielded coordinator and wipe host rows inside Clear](https://github.com/dashpay/platform/pull/5934) — 7 unresolved (7 bot) · 6 days stale · ⚠ merge conflict · 🔴 CI failing · 📝 draft · 🐢 targets v5.2-dev · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk, kotlin-sdk · Policy: draft
  - Top thread: "✅ **Withdrawn** at \`2352c3c6\`; see the replies below." — 6 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5711 feat: shielded-tip screens over the wallet's tip decisions](https://github.com/dashpay/platform/pull/5711) — 4 unresolved (4 bot) · 19 days stale · 📝 draft · 🐢 targets feat/dashpay-shielded-tips-wallet-policy · areas: swift-sdk, kotlin-sdk
  - Top thread: "✅ **Resolved** at \`451dab59\`; see the replies below." — 19 days old
- [dashpay/platform#5786 feat(sdk): tri-state quorum lookups, bounded funding retries and a DPNS contest policy in dash-sdk-cxx](https://github.com/dashpay/platform/pull/5786) — 3 unresolved (3 bot) · 14 days stale · ⚠ merge conflict · 📝 draft · 🐢 targets v5.2-dev · areas: rs-drive, rs-platform-wallet, rust-sdk, js-wasm-sdk, fallback · Policy: draft
  - Top thread: "✅ **Resolved** at \`e46d43eb\`; see the replies below." — 14 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5280 perf(platform-wallet)!: reuse a recent recorded-anchor set on the shielded spend path](https://github.com/dashpay/platform/pull/5280) — 7 unresolved (7 bot) · 0 days stale · ✋ changes requested · 🔴 CI failing · 🐢 targets v5.2-dev · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, fallback · Policy: waiting-author
  - Top thread: "🔴 Blocking: Bind cached anchor evidence to the active verification policy**" — 0 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#4965 test(sdk): cargo-fuzz harness over drive-proof-verifier FromProof](https://github.com/dashpay/platform/pull/4965) — 1 unresolved (1 bot) · 45 days stale · 📝 draft · 🐢 targets v5.2-dev · areas: github, fallback · Policy: draft
  - Top thread: "🟡 Suggestion: Correct the claim that PV13 reaches the V0 proof decoder**" — 45 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5288 feat(platform-wallet): support DashPay shielded tips with dedicated accounts](https://github.com/dashpay/platform/pull/5288) — 1 unresolved (1 bot) · 4 days stale · ⚠ merge conflict · 🔴 CI failing · 🐢 targets v5.2-dev · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk, rust-sdk, kotlin-sdk, fallback · Policy: waiting-author
  - Top thread: "⏭️ **Deferred** (maintainer decision; not blocking; tracked in https://github.com/dashpay/platform/issues/5950) at \`e14a…" — 4 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
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
- [dashpay/platform#5793 fix(platform-wallet): make SpvRuntime::header_hash_at public](https://github.com/dashpay/platform/pull/5793) — 🐢 targets v5.2-dev · areas: rs-platform-wallet · Policy: ready-to-merge
  - Blocker: All policy requirements are satisfied
- [dashpay/platform#5794 feat(platform-wallet-storage): read one asset lock's status by outpoint](https://github.com/dashpay/platform/pull/5794) — 🐢 targets v5.2-dev · areas: wallet-storage · Policy: too-many-open-prs
  - Blocker: Human review is still required; automatic reviewer requests are paused until a review slot is available
- [dashpay/platform#5802 fix(platform-wallet-storage): don't look for or count non-funds used addresses as unresolved](https://github.com/dashpay/platform/pull/5802) — ⚠ merge conflict · 📝 draft · 🐢 targets v5.2-dev · areas: wallet-storage, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot

<a id="romchornyi"></a>
### @romchornyi
<a id="romchornyi-open"></a>
#### Open (15)
- [dashpay/platform#5928 feat(platform-wallet): report whether a failed payment may have left the wallet](https://github.com/dashpay/platform/pull/5928) — 16 unresolved (16 bot) · 2 days stale · 🔴 CI failing · 🐢 targets v5.2-dev · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk · Policy: waiting-author
  - Top thread: "⏭️ **Deferred** (maintainer decision; not blocking) at \`19c00501\`; see the replies below." — 2 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5853 feat: settle a shielded identity create whose inputs are spent without its identity](https://github.com/dashpay/platform/pull/5853) — 11 unresolved (11 bot) · 4 days stale · ⚠ merge conflict · 🔴 CI failing · 🐢 targets v5.2-dev · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, rust-sdk, kotlin-sdk, fallback · Policy: waiting-author
  - Top thread: "✅ **Withdrawn** at \`c4a79650\`; see the replies below." — 4 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5930 feat(platform-wallet): record the payment intent of external asset locks](https://github.com/dashpay/platform/pull/5930) — 9 unresolved (9 bot) · 5 days stale · ⚠ merge conflict · 🔴 CI failing · 🐢 targets v5.2-dev · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk, kotlin-sdk · Policy: waiting-author
  - Top thread: "✅ **Resolved** at \`c41628d3\`; see the replies below." — 5 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5536 fix: check a DashPay invitation claim against Platform before broadcasting it](https://github.com/dashpay/platform/pull/5536) — 6 unresolved (6 bot) · 8 days stale · ⚠ merge conflict · 🔴 CI failing · 🐢 targets v5.2-dev · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, rust-sdk, kotlin-sdk, sdk-parity · Policy: waiting-author
  - Top thread: "⏭️ **Deferred** (maintainer decision; not blocking) at \`f312d906\`; see the replies below." — 8 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5865 feat(platform-wallet): plan a shielded spend with its exact fee and spend against the plan](https://github.com/dashpay/platform/pull/5865) — 11 unresolved (11 bot) · 1 days stale · 🔴 CI failing · 🐢 targets v5.2-dev · areas: dpp, rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, rust-sdk, kotlin-sdk, sdk-parity, fallback · Policy: waiting-author
  - Top thread: "✅ **Resolved** at \`564797d1\`; see the replies below." — 1 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5258 feat(platform-wallet): probe the network about sends with an unknown broadcast outcome](https://github.com/dashpay/platform/pull/5258) — 3 unresolved (3 human) · 36 days stale · ⚠ merge conflict · 📝 draft · 🐢 targets v5.2-dev · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, rust-sdk · Policy: draft
  - Top thread: "\[P2\] Carry the originating wallet generation in probe events" — 36 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5540 fix(platform-wallet): store externally supplied identity keys without a derivation breadcrumb](https://github.com/dashpay/platform/pull/5540) — 3 unresolved (3 bot) · 30 days stale · ⚠ merge conflict · 🔴 CI failing · 🐢 targets v5.2-dev · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk · Policy: waiting-author
  - Top thread: "✅ **Resolved** at \`b25b2543\`; see the replies below." — 30 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5549 fix(platform-wallet-storage): restore DashPay contact address pools on SQLite load](https://github.com/dashpay/platform/pull/5549) — 3 unresolved (3 bot) · 28 days stale · ⚠ merge conflict · 🔴 CI failing · 📝 draft · 🐢 targets v5.2-dev · areas: wallet-storage · Policy: draft
  - Top thread: "✅ **Withdrawn** at \`2bee2705\`; see the replies below." — 28 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5395 feat(platform-wallet): spend only final coins and report funds waiting on the network](https://github.com/dashpay/platform/pull/5395) — 1 unresolved (1 human) · 36 days stale · ⚠ merge conflict · ✋ changes requested · 📝 draft · 🐢 targets v5.2-dev · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk · Policy: draft
  - Top thread: "this should belong to another layer - likely platform-wallet, or even rust-dashcore's key-wallet. FFI is not meant to st…" — 36 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5929 feat(platform-wallet): one address top-up for any identity](https://github.com/dashpay/platform/pull/5929) — 4 unresolved (4 bot) · 5 days stale · 🐢 targets v5.2-dev · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk, sdk-parity · Policy: waiting-self-review
  - Top thread: "🟡 Suggestion: Make the liveness regression test remove the wallet after the reads**" — 5 days old
  - Blocker: Author must post /self-reviewed 661ce185556b61208615cc646c88d145c487176e
- [dashpay/platform#4992 feat(swift-sdk): durable wallet presence marker readable while the device is locked](https://github.com/dashpay/platform/pull/4992) — 🐢 targets v5.2-dev · areas: swift-sdk · Policy: ready-to-merge
  - Blocker: All policy requirements are satisfied
- [dashpay/platform#5587 fix(platform-wallet)!: keep a contact's payment account across relaunches and replace it cleanly on rotation](https://github.com/dashpay/platform/pull/5587) — ⚠ merge conflict · 📝 draft · 🐢 targets v5.2-dev · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5589 feat(platform-wallet-ffi)!: let hosts reset an outbound contact account's pool and keep its request digest](https://github.com/dashpay/platform/pull/5589) — 🔴 CI failing · 📝 draft · 🐢 targets fix/contact-rebuild-keeps-pool · areas: rs-platform-wallet-ffi, swift-sdk
- [dashpay/platform#5590 feat(kotlin-sdk)!: reset an outbound contact account's pool and keep its request digest on Android](https://github.com/dashpay/platform/pull/5590) — 🔴 CI failing · 📝 draft · 🐢 targets fix/contact-rebuild-ffi-swift · areas: kotlin-sdk, sdk-parity
- [dashpay/platform#6014 feat(platform-wallet)!: give identity slots one wallet-owned lease](https://github.com/dashpay/platform/pull/6014) — 🔴 CI failing · 📝 draft · 🐢 targets feat/invitation-claim-preflight · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, rust-sdk, kotlin-sdk, sdk-parity

<a id="romchornyi-ready-for-human"></a>
#### Ready for human (1)
- [dashpay/platform#4992 feat(swift-sdk): durable wallet presence marker readable while the device is locked](https://github.com/dashpay/platform/pull/4992) — 🐢 targets v5.2-dev · areas: swift-sdk · Policy: ready-to-merge
  - Blocker: All policy requirements are satisfied

<a id="romchornyi-stale"></a>
#### Stale (15)
- [dashpay/platform#5928 feat(platform-wallet): report whether a failed payment may have left the wallet](https://github.com/dashpay/platform/pull/5928) — 16 unresolved (16 bot) · 2 days stale · 🔴 CI failing · 🐢 targets v5.2-dev · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk · Policy: waiting-author
  - Top thread: "⏭️ **Deferred** (maintainer decision; not blocking) at \`19c00501\`; see the replies below." — 2 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5853 feat: settle a shielded identity create whose inputs are spent without its identity](https://github.com/dashpay/platform/pull/5853) — 11 unresolved (11 bot) · 4 days stale · ⚠ merge conflict · 🔴 CI failing · 🐢 targets v5.2-dev · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, rust-sdk, kotlin-sdk, fallback · Policy: waiting-author
  - Top thread: "✅ **Withdrawn** at \`c4a79650\`; see the replies below." — 4 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5930 feat(platform-wallet): record the payment intent of external asset locks](https://github.com/dashpay/platform/pull/5930) — 9 unresolved (9 bot) · 5 days stale · ⚠ merge conflict · 🔴 CI failing · 🐢 targets v5.2-dev · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk, kotlin-sdk · Policy: waiting-author
  - Top thread: "✅ **Resolved** at \`c41628d3\`; see the replies below." — 5 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5536 fix: check a DashPay invitation claim against Platform before broadcasting it](https://github.com/dashpay/platform/pull/5536) — 6 unresolved (6 bot) · 8 days stale · ⚠ merge conflict · 🔴 CI failing · 🐢 targets v5.2-dev · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, rust-sdk, kotlin-sdk, sdk-parity · Policy: waiting-author
  - Top thread: "⏭️ **Deferred** (maintainer decision; not blocking) at \`f312d906\`; see the replies below." — 8 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5865 feat(platform-wallet): plan a shielded spend with its exact fee and spend against the plan](https://github.com/dashpay/platform/pull/5865) — 11 unresolved (11 bot) · 1 days stale · 🔴 CI failing · 🐢 targets v5.2-dev · areas: dpp, rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, rust-sdk, kotlin-sdk, sdk-parity, fallback · Policy: waiting-author
  - Top thread: "✅ **Resolved** at \`564797d1\`; see the replies below." — 1 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5258 feat(platform-wallet): probe the network about sends with an unknown broadcast outcome](https://github.com/dashpay/platform/pull/5258) — 3 unresolved (3 human) · 36 days stale · ⚠ merge conflict · 📝 draft · 🐢 targets v5.2-dev · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, rust-sdk · Policy: draft
  - Top thread: "\[P2\] Carry the originating wallet generation in probe events" — 36 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5540 fix(platform-wallet): store externally supplied identity keys without a derivation breadcrumb](https://github.com/dashpay/platform/pull/5540) — 3 unresolved (3 bot) · 30 days stale · ⚠ merge conflict · 🔴 CI failing · 🐢 targets v5.2-dev · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk · Policy: waiting-author
  - Top thread: "✅ **Resolved** at \`b25b2543\`; see the replies below." — 30 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5549 fix(platform-wallet-storage): restore DashPay contact address pools on SQLite load](https://github.com/dashpay/platform/pull/5549) — 3 unresolved (3 bot) · 28 days stale · ⚠ merge conflict · 🔴 CI failing · 📝 draft · 🐢 targets v5.2-dev · areas: wallet-storage · Policy: draft
  - Top thread: "✅ **Withdrawn** at \`2bee2705\`; see the replies below." — 28 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5395 feat(platform-wallet): spend only final coins and report funds waiting on the network](https://github.com/dashpay/platform/pull/5395) — 1 unresolved (1 human) · 36 days stale · ⚠ merge conflict · ✋ changes requested · 📝 draft · 🐢 targets v5.2-dev · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk · Policy: draft
  - Top thread: "this should belong to another layer - likely platform-wallet, or even rust-dashcore's key-wallet. FFI is not meant to st…" — 36 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5929 feat(platform-wallet): one address top-up for any identity](https://github.com/dashpay/platform/pull/5929) — 4 unresolved (4 bot) · 5 days stale · 🐢 targets v5.2-dev · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk, sdk-parity · Policy: waiting-self-review
  - Top thread: "🟡 Suggestion: Make the liveness regression test remove the wallet after the reads**" — 5 days old
  - Blocker: Author must post /self-reviewed 661ce185556b61208615cc646c88d145c487176e
- [dashpay/platform#4992 feat(swift-sdk): durable wallet presence marker readable while the device is locked](https://github.com/dashpay/platform/pull/4992) — 🐢 targets v5.2-dev · areas: swift-sdk · Policy: ready-to-merge
  - Blocker: All policy requirements are satisfied
- [dashpay/platform#5587 fix(platform-wallet)!: keep a contact's payment account across relaunches and replace it cleanly on rotation](https://github.com/dashpay/platform/pull/5587) — ⚠ merge conflict · 📝 draft · 🐢 targets v5.2-dev · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5589 feat(platform-wallet-ffi)!: let hosts reset an outbound contact account's pool and keep its request digest](https://github.com/dashpay/platform/pull/5589) — 🔴 CI failing · 📝 draft · 🐢 targets fix/contact-rebuild-keeps-pool · areas: rs-platform-wallet-ffi, swift-sdk
- [dashpay/platform#5590 feat(kotlin-sdk)!: reset an outbound contact account's pool and keep its request digest on Android](https://github.com/dashpay/platform/pull/5590) — 🔴 CI failing · 📝 draft · 🐢 targets fix/contact-rebuild-ffi-swift · areas: kotlin-sdk, sdk-parity
- [dashpay/platform#6014 feat(platform-wallet)!: give identity slots one wallet-owned lease](https://github.com/dashpay/platform/pull/6014) — 🔴 CI failing · 📝 draft · 🐢 targets feat/invitation-claim-preflight · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, rust-sdk, kotlin-sdk, sdk-parity

<a id="hashengineering"></a>
### @HashEngineering
<a id="hashengineering-open"></a>
#### Open (6)
- [dashpay/platform#5931 feat(platform-wallet): top up someone else's identity from the Core balance](https://github.com/dashpay/platform/pull/5931) — 13 unresolved (13 bot) · 7 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · 🐢 targets v5.2-dev · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Top-up result can report a stale cached balance**" — 7 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5767 fix(kotlin-sdk): port #4638 to Android on the wallet's TXO reconcile policy](https://github.com/dashpay/platform/pull/5767) — 3 unresolved (3 bot) · 10 days stale · 🐢 targets v5.2-dev · areas: swift-sdk, kotlin-sdk, github, fallback · Policy: waiting-author
  - Top thread: "⏭️ **Deferred** (maintainer decision; not blocking; tracked in https://github.com/dashpay/platform/issues/5977) at \`2efb…" — 10 days old
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5933 feat(kotlin-sdk): top up someone else's identity from the Core balance](https://github.com/dashpay/platform/pull/5933) — 2 unresolved (1 CodeRabbit, 1 bot) · 6 days stale · ✋ changes requested · 📝 draft · 🐢 targets HashEngineering/platform-wallet-top-up-someone-elses-identity-fr · areas: kotlin-sdk, sdk-parity
  - Top thread: "&lt;picture&gt;&lt;img src="https://storage.googleapis.com/coderabbit_public_assets/review-comment-icons-v1/amber-16.svg" width="…" — 6 days old
- [dashpay/platform#5896 feat(kotlin-sdk): shield from identity to a third-party Orchard address](https://github.com/dashpay/platform/pull/5896) — 1 unresolved (1 bot) · 1 days stale · 🐢 targets v5.2-dev · areas: kotlin-sdk, sdk-parity · Policy: ready-to-merge
  - Top thread: "⏭️ **Deferred** (maintainer decision; not blocking) at \`defca9c3\`; see the replies below." — 1 days old
  - Blocker: All policy requirements are satisfied
- [dashpay/platform#5906 feat(kotlin-sdk): topUpExternalIdentityFromAddresses](https://github.com/dashpay/platform/pull/5906) — 1 unresolved (1 bot) · 1 days stale · ✋ changes requested · 🔴 CI failing · 🐢 targets v5.2-dev · areas: kotlin-sdk, sdk-parity · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Tell callers when they must record a completed top-up**" — 1 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
- [dashpay/platform#5479 feat(platform-wallet-storage): persist DashPay backfill coverage and outbound-account markers](https://github.com/dashpay/platform/pull/5479) — ⚠ merge conflict · 🐢 targets v5.2-dev · areas: rs-platform-wallet, wallet-storage · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 8e1ee2e24549cabf2d2e81a85a02fc613212aa1f

<a id="hashengineering-ready-for-human"></a>
#### Ready for human (1)
- [dashpay/platform#5896 feat(kotlin-sdk): shield from identity to a third-party Orchard address](https://github.com/dashpay/platform/pull/5896) — 1 unresolved (1 bot) · 1 days stale · 🐢 targets v5.2-dev · areas: kotlin-sdk, sdk-parity · Policy: ready-to-merge
  - Top thread: "⏭️ **Deferred** (maintainer decision; not blocking) at \`defca9c3\`; see the replies below." — 1 days old
  - Blocker: All policy requirements are satisfied

<a id="hashengineering-stale"></a>
#### Stale (6)
- [dashpay/platform#5931 feat(platform-wallet): top up someone else's identity from the Core balance](https://github.com/dashpay/platform/pull/5931) — 13 unresolved (13 bot) · 7 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · 🐢 targets v5.2-dev · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Top-up result can report a stale cached balance**" — 7 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5767 fix(kotlin-sdk): port #4638 to Android on the wallet's TXO reconcile policy](https://github.com/dashpay/platform/pull/5767) — 3 unresolved (3 bot) · 10 days stale · 🐢 targets v5.2-dev · areas: swift-sdk, kotlin-sdk, github, fallback · Policy: waiting-author
  - Top thread: "⏭️ **Deferred** (maintainer decision; not blocking; tracked in https://github.com/dashpay/platform/issues/5977) at \`2efb…" — 10 days old
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5933 feat(kotlin-sdk): top up someone else's identity from the Core balance](https://github.com/dashpay/platform/pull/5933) — 2 unresolved (1 CodeRabbit, 1 bot) · 6 days stale · ✋ changes requested · 📝 draft · 🐢 targets HashEngineering/platform-wallet-top-up-someone-elses-identity-fr · areas: kotlin-sdk, sdk-parity
  - Top thread: "&lt;picture&gt;&lt;img src="https://storage.googleapis.com/coderabbit_public_assets/review-comment-icons-v1/amber-16.svg" width="…" — 6 days old
- [dashpay/platform#5896 feat(kotlin-sdk): shield from identity to a third-party Orchard address](https://github.com/dashpay/platform/pull/5896) — 1 unresolved (1 bot) · 1 days stale · 🐢 targets v5.2-dev · areas: kotlin-sdk, sdk-parity · Policy: ready-to-merge
  - Top thread: "⏭️ **Deferred** (maintainer decision; not blocking) at \`defca9c3\`; see the replies below." — 1 days old
  - Blocker: All policy requirements are satisfied
- [dashpay/platform#5906 feat(kotlin-sdk): topUpExternalIdentityFromAddresses](https://github.com/dashpay/platform/pull/5906) — 1 unresolved (1 bot) · 1 days stale · ✋ changes requested · 🔴 CI failing · 🐢 targets v5.2-dev · areas: kotlin-sdk, sdk-parity · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Tell callers when they must record a completed top-up**" — 1 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
- [dashpay/platform#5479 feat(platform-wallet-storage): persist DashPay backfill coverage and outbound-account markers](https://github.com/dashpay/platform/pull/5479) — ⚠ merge conflict · 🐢 targets v5.2-dev · areas: rs-platform-wallet, wallet-storage · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 8e1ee2e24549cabf2d2e81a85a02fc613212aa1f

<a id="bfoss765"></a>
### @bfoss765
<a id="bfoss765-open"></a>
#### Open (2)
- [dashpay/platform#4313 feat(kotlin-sdk): one-time Orchard key shielded-invite API (inviter + claim)](https://github.com/dashpay/platform/pull/4313) — 6 unresolved (6 bot) · 82 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · 🐢 targets v5.2-dev · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk · Policy: waiting-author
  - Top thread: "✅ **Resolved** at \`eccc65fa\`; see the replies below." — 82 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#4312 feat(platform-wallet): multi-output shielded transfers + output-aware fee predictor](https://github.com/dashpay/platform/pull/4312) — 1 unresolved (1 bot) · 82 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · 🐢 targets v5.2-dev · areas: dpp, rs-platform-wallet, rs-platform-wallet-ffi, kotlin-sdk, fallback · Policy: waiting-self-review
  - Top thread: "🟡 Suggestion: Keep envelope-aware action counting behind the crate boundary**" — 82 days old
  - Blocker: Author must post /self-reviewed da932f96a8670c641065954de8b58752e99233a3

<a id="bfoss765-stale"></a>
#### Stale (2)
- [dashpay/platform#4313 feat(kotlin-sdk): one-time Orchard key shielded-invite API (inviter + claim)](https://github.com/dashpay/platform/pull/4313) — 6 unresolved (6 bot) · 82 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · 🐢 targets v5.2-dev · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk · Policy: waiting-author
  - Top thread: "✅ **Resolved** at \`eccc65fa\`; see the replies below." — 82 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#4312 feat(platform-wallet): multi-output shielded transfers + output-aware fee predictor](https://github.com/dashpay/platform/pull/4312) — 1 unresolved (1 bot) · 82 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · 🐢 targets v5.2-dev · areas: dpp, rs-platform-wallet, rs-platform-wallet-ffi, kotlin-sdk, fallback · Policy: waiting-self-review
  - Top thread: "🟡 Suggestion: Keep envelope-aware action counting behind the crate boundary**" — 82 days old
  - Blocker: Author must post /self-reviewed da932f96a8670c641065954de8b58752e99233a3

<a id="llbartekll"></a>
### @llbartekll
<a id="llbartekll-open"></a>
#### Open (3)
- [dashpay/platform#5997 refactor(platform-wallet): share the masternode update helpers and use rust-dashcore's key and script types](https://github.com/dashpay/platform/pull/5997) — 5 unresolved (5 bot) · 1 days stale · 🐢 targets feat/masternode-key-rotation · areas: rs-platform-wallet
  - Top thread: "✅ **Resolved** at \`cd1020ff\`; see the replies below." — 1 days old
- [dashpay/platform#5889 feat(platform-wallet): rotate a masternode's keys into the wallet](https://github.com/dashpay/platform/pull/5889) — ✋ changes requested · 🔴 CI failing · 🐢 targets v5.2-dev · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk · Policy: waiting-author
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
- [dashpay/platform#5982 feat(platform-wallet)!: apps show the shielded balance the wallet announces, without identity accounts](https://github.com/dashpay/platform/pull/5982) — 🔴 CI failing · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk · Policy: waiting-author
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix

<a id="llbartekll-ci-failing"></a>
#### CI Failing (1)
- [dashpay/platform#5982 feat(platform-wallet)!: apps show the shielded balance the wallet announces, without identity accounts](https://github.com/dashpay/platform/pull/5982) — 🔴 CI failing · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk · Policy: waiting-author
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix

<a id="llbartekll-stale"></a>
#### Stale (2)
- [dashpay/platform#5997 refactor(platform-wallet): share the masternode update helpers and use rust-dashcore's key and script types](https://github.com/dashpay/platform/pull/5997) — 5 unresolved (5 bot) · 1 days stale · 🐢 targets feat/masternode-key-rotation · areas: rs-platform-wallet
  - Top thread: "✅ **Resolved** at \`cd1020ff\`; see the replies below." — 1 days old
- [dashpay/platform#5889 feat(platform-wallet): rotate a masternode's keys into the wallet](https://github.com/dashpay/platform/pull/5889) — ✋ changes requested · 🔴 CI failing · 🐢 targets v5.2-dev · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk · Policy: waiting-author
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix

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
