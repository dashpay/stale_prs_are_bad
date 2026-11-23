---
---
# PR Hygiene Report
*Last updated: 2026-11-23 00:22 UTC · commit 8385ebf*

## Summary
- Open PRs: **215** (14 clean · 1 CI failing · 16 changes requested · 18 unresolved comments · 5 deferred · 19 draft · 142 stale)
- PRs needing author action: **35**
- Total unresolved comments: **362**
- dashpay/platform: **137** open (6 clean · 1 CI failing · 8 changes requested · 2 unresolved comments · 5 deferred · 1 draft · 114 stale) · engine: 24 draft · 7 ready-for-human · 3 ready-to-merge · 1 too-many-open-prs · 30 waiting-author · 11 waiting-bots · 1 waiting-build · 15 waiting-self-review · 45 no verdict
- dashpay/rust-dashcore: **25** open (3 clean · 0 CI failing · 6 changes requested · 2 unresolved comments · 0 deferred · 10 draft · 4 stale) · engine: 10 draft · 2 ready-for-human · 2 waiting-author · 7 waiting-self-review · 4 no verdict
- dashpay/tenderdash: **10** open (1 clean · 0 CI failing · 0 changes requested · 5 unresolved comments · 0 deferred · 4 draft · 0 stale) · engine: 4 draft · 4 waiting-author · 2 waiting-self-review
- dashpay/grovedb: **2** open (0 clean · 0 CI failing · 0 changes requested · 0 unresolved comments · 0 deferred · 2 draft · 0 stale) · engine: 2 draft
- dashpay/dash-evo-tool: **41** open (4 clean · 0 CI failing · 2 changes requested · 9 unresolved comments · 0 deferred · 2 draft · 24 stale) · engine: 2 draft · 1 ready-for-human · 10 waiting-author · 2 waiting-bots · 1 waiting-build · 1 waiting-self-review · 24 no verdict

## Scoreboard
_Sort: unresolved-comments desc → needs-action desc → ready-for-review desc. Click any number to jump to the specific PRs it covers._

| Author | Open | Clean | CI failing | Unresolved Comments | Changes Requested | Deferred | Draft | Stale | Needs action | Ready for human | Total Unresolved Comments | Ready for Review |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| [@Claudius-Maginificent](#claudius-maginificent) | [73](#claudius-maginificent-open) | [2](#claudius-maginificent-clean) | — | [10](#claudius-maginificent-unresolved-comments) | [2](#claudius-maginificent-changes-requested) | [3](#claudius-maginificent-deferred) | [4](#claudius-maginificent-draft) | [52](#claudius-maginificent-stale) | [12](#claudius-maginificent-needs-action) | [4](#claudius-maginificent-ready-for-human) | [113](#claudius-maginificent-unresolved-comments) | — |
| [@QuantumExplorer](#quantumexplorer) | [34](#quantumexplorer-open) | [5](#quantumexplorer-clean) | — | [3](#quantumexplorer-unresolved-comments) | [8](#quantumexplorer-changes-requested) | — | [2](#quantumexplorer-draft) | [16](#quantumexplorer-stale) | [11](#quantumexplorer-needs-action) | — | [44](#quantumexplorer-unresolved-comments) | [3](#quantumexplorer-ready-for-review) |
| [@shumkov](#shumkov) | [12](#shumkov-open) | [2](#shumkov-clean) | — | [3](#shumkov-unresolved-comments) | [1](#shumkov-changes-requested) | — | — | [6](#shumkov-stale) | [4](#shumkov-needs-action) | — | [28](#shumkov-unresolved-comments) | [7](#shumkov-ready-for-review) |
| [@lklimek](#lklimek) | [16](#lklimek-open) | [1](#lklimek-clean) | — | [1](#lklimek-unresolved-comments) | [1](#lklimek-changes-requested) | [2](#lklimek-deferred) | [4](#lklimek-draft) | [7](#lklimek-stale) | [2](#lklimek-needs-action) | [2](#lklimek-ready-for-human) | [9](#lklimek-unresolved-comments) | [4](#lklimek-ready-for-review) |
| [@ZocoLini](#zocolini) | [6](#zocolini-open) | [1](#zocolini-clean) | — | [1](#zocolini-unresolved-comments) | — | — | [4](#zocolini-draft) | — | [1](#zocolini-needs-action) | — | [3](#zocolini-unresolved-comments) | [2](#zocolini-ready-for-review) |
| [@xdustinface](#xdustinface) | [12](#xdustinface-open) | [3](#xdustinface-clean) | — | — | [2](#xdustinface-changes-requested) | — | [3](#xdustinface-draft) | [4](#xdustinface-stale) | [2](#xdustinface-needs-action) | — | [2](#xdustinface-unresolved-comments) | [1](#xdustinface-ready-for-review) |
| [@PastaPastaPasta](#pastapastapasta) | [26](#pastapastapasta-open) | — | — | — | [2](#pastapastapasta-changes-requested) | — | [2](#pastapastapasta-draft) | [22](#pastapastapasta-stale) | [2](#pastapastapasta-needs-action) | [3](#pastapastapasta-ready-for-human) | [59](#pastapastapasta-unresolved-comments) | — |
| [@llbartekll](#llbartekll) | [3](#llbartekll-open) | — | [1](#llbartekll-ci-failing) | — | — | — | — | [2](#llbartekll-stale) | [1](#llbartekll-needs-action) | — | [5](#llbartekll-unresolved-comments) | [3](#llbartekll-ready-for-review) |
| [@ktechmidas](#ktechmidas) | [6](#ktechmidas-open) | — | — | — | — | — | — | [6](#ktechmidas-stale) | — | — | [5](#ktechmidas-unresolved-comments) | [6](#ktechmidas-ready-for-review) |
| [@romchornyi](#romchornyi) | [15](#romchornyi-open) | — | — | — | — | — | — | [15](#romchornyi-stale) | — | [1](#romchornyi-ready-for-human) | [67](#romchornyi-unresolved-comments) | [3](#romchornyi-ready-for-review) |
| [@HashEngineering](#hashengineering) | [6](#hashengineering-open) | — | — | — | — | — | — | [6](#hashengineering-stale) | — | [1](#hashengineering-ready-for-human) | [20](#hashengineering-unresolved-comments) | [1](#hashengineering-ready-for-review) |
| [@bfoss765](#bfoss765) | [2](#bfoss765-open) | — | — | — | — | — | — | [2](#bfoss765-stale) | — | — | [7](#bfoss765-unresolved-comments) | — |
| [@DCG-Claude](#dcg-claude) | [3](#dcg-claude-open) | — | — | — | — | — | — | [3](#dcg-claude-stale) | — | [2](#dcg-claude-ready-for-human) | — | — |
| [@thepastaclaw](#thepastaclaw) | [1](#thepastaclaw-open) | — | — | — | — | — | — | [1](#thepastaclaw-stale) | — | — | — | — |

## Per-author detail

<a id="claudius-maginificent"></a>
### @Claudius-Maginificent
<a id="claudius-maginificent-open"></a>
#### Open (73)
- [dashpay/platform#5645 fix(platform-wallet)!: check identity creation keys and funding in the Rust core](https://github.com/dashpay/platform/pull/5645) — 8 unresolved (1 CodeRabbit, 7 bot) · 19 days stale · ✋ changes requested · 🔴 CI failing · 🐢 targets v5.2-dev · areas: dpp, rs-platform-wallet, rs-platform-wallet-ffi, rust-sdk · Policy: waiting-author
  - Top thread: "&lt;picture&gt;&lt;img src="https://storage.googleapis.com/coderabbit_public_assets/review-comment-icons-v1/amber-16.svg" width="…" — 19 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: coderabbitai left review threads unresolved; resolve them
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5565 feat(platform-wallet)!: detect inconsistent shielded commitment trees and add explicit rebuild](https://github.com/dashpay/platform/pull/5565) — 7 unresolved (7 bot) · 23 days stale · ✋ changes requested · 🔴 CI failing · 🐢 targets v5.2-dev · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk · Policy: waiting-author
  - Top thread: "⏸️ **Needs a tracking issue** (not yet deferred) at \`3b17ab95\`; see the replies below." — 23 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/dash-evo-tool#1177 feat(ui): align app-wide styles with the token proposal](https://github.com/dashpay/dash-evo-tool/pull/1177) — 13 unresolved (13 bot) · 4 days stale · ✋ changes requested · 📝 draft · 🐢 targets feat/token-creator-workspace · areas: fallback
  - Top thread: "✅ **Resolved** at \`0993a6af\`; see the replies below." — 4 days old
- [dashpay/dash-evo-tool#1178 fix(tokens): keep JSON export responsive when the file chooser stalls](https://github.com/dashpay/dash-evo-tool/pull/1178) — 15 unresolved (7 CodeRabbit, 8 bot) · 2 days stale · ✋ changes requested · 📝 draft · 🐢 targets feat/app-design-system · areas: fallback
  - Top thread: "🟡 Suggestion: Exercise a publication failure after staging succeeds**" — 2 days old
- [dashpay/platform#5588 feat(platform-wallet)!: expose protocol-14-aware signing-key selection](https://github.com/dashpay/platform/pull/5588) — 5 unresolved (5 bot) · 23 days stale · 🔴 CI failing · 🐢 targets v5.2-dev · areas: dpp, rs-platform-wallet, rs-platform-wallet-ffi, rust-sdk · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Include BLS keys in automatic document and token selection**" — 23 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/dash-evo-tool#1108 feat(documents): recognise indexOnly document types and delete their entries by values](https://github.com/dashpay/dash-evo-tool/pull/1108) — 3 unresolved (3 bot) · 30 days stale · ✋ changes requested · 📝 draft · 🐢 targets feat/document-optional-token-cost · areas: fallback
  - Top thread: "🟡 Suggestion: Make the non-deletable error actionable**" — 30 days old
- [dashpay/dash-evo-tool#1168 fix(tokens): keep the Token Creator's identity current and its registration state reliable](https://github.com/dashpay/dash-evo-tool/pull/1168) — 9 unresolved (9 bot) · 1 days stale · ✋ changes requested · 🔴 CI failing · areas: fallback · Policy: waiting-author
  - Top thread: "🔴 Blocking: Make the new unit tests compile without the testing feature**" — 1 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/platform#5644 fix!: follow DIP-11 key rules when disabling keys and reading contact requests](https://github.com/dashpay/platform/pull/5644) — 8 unresolved (8 bot) · 0 days stale · ✋ changes requested · 🔴 CI failing · 🐢 targets v5.2-dev · areas: dpp, rs-platform-wallet, rs-platform-wallet-ffi, rust-sdk, rust-sdk-ffi, js-wasm-sdk, fallback · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Update the PR migration notes for the final client API**" — 0 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/dash-evo-tool#1206 fix(boot): show startup panics and window failures instead of closing](https://github.com/dashpay/dash-evo-tool/pull/1206) — 7 unresolved (7 bot) · 1 days stale · ✋ changes requested · 🔴 CI failing · areas: fallback · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Exercise a constructor panic through the production boot path**" — 1 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/platform#5546 fix(platform-wallet): support HASH160 DashPay contact signing keys](https://github.com/dashpay/platform/pull/5546) — 2 unresolved (2 bot) · 23 days stale · ⚠ merge conflict · 🐢 targets feat/platform-wallet-public-signing-key-selection · areas: rs-drive-abci, dpp, rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk, fallback
  - Top thread: "🟡 Suggestion: Do not expose an unchecked signer-capability override in the public constructor**" — 23 days old
- [dashpay/dash-evo-tool#1112 feat(send): one From/To Send form with Auto routing](https://github.com/dashpay/dash-evo-tool/pull/1112) — 5 unresolved (5 bot) · 1 days stale · 🔴 CI failing · areas: fallback · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Ignore parked recipient rows when deciding whether to show Fee options**" — 1 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/dash-evo-tool#1170 fix(ui): retire finished progress banners and describe key protection correctly](https://github.com/dashpay/dash-evo-tool/pull/1170) — 5 unresolved (5 bot) · 1 days stale · ✋ changes requested · 🔴 CI failing · areas: fallback · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Decouple cached preset rules from selector order**" — 1 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/dash-evo-tool#1179 fix(tokens): retain the creator when opening a registered contract](https://github.com/dashpay/dash-evo-tool/pull/1179) — 4 unresolved (4 bot) · 2 days stale · ✋ changes requested · 📝 draft · 🐢 targets fix/token-json-export · areas: fallback
  - Top thread: "🟡 Suggestion: Assert that the preview displays fetched contract details**" — 2 days old
- [dashpay/platform#6029 feat(platform-wallet): report which identities an identity sync pass read and at which height](https://github.com/dashpay/platform/pull/6029) — 4 unresolved (4 bot) · 2 days stale · 🐢 targets v5.2-dev · areas: rs-platform-wallet · Policy: ready-for-human
  - Top thread: "🟡 Suggestion: Document that the summary reports reads, not persistence receipts**" — 2 days old
  - Blocker: Human approval or objection resolution is required
- [dashpay/tenderdash#1600 fix(consensus): deliver and process a polka block for a validator left in its round](https://github.com/dashpay/tenderdash/pull/1600) — 2 unresolved (2 bot) · 5 days stale · ✋ changes requested · 🔴 CI failing · areas: fallback · Policy: waiting-author
  - Top thread: "🔴 Blocking: Avoid racing on PartSet completeness in the gossip snapshot**" — 5 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/dash-evo-tool#1200 feat(wallet)!: detect databases needing the platform bridge and remove the bridge](https://github.com/dashpay/dash-evo-tool/pull/1200) — 3 unresolved (3 bot) · 2 days stale · 🔴 CI failing · areas: fallback · Policy: waiting-author
  - Top thread: "🔴 Blocking: The recovery instructions omit previously opened networks without wallets**" — 2 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/dash-evo-tool#1124 feat(documents): let the user skip an optional token cost](https://github.com/dashpay/dash-evo-tool/pull/1124) — 1 unresolved (1 bot) · 27 days stale · ⚠ merge conflict · ✋ changes requested · 📝 draft · 🐢 targets fix/protocol14-signing-key-selection · areas: fallback
  - Top thread: "🔴 Blocking: Show the credit-fee outcome when an optional token cost is disabled**" — 27 days old
- [dashpay/platform#5981 refactor(platform-wallet-storage): drop the unreachable rehydration gap-refill guard](https://github.com/dashpay/platform/pull/5981) — 2 unresolved (2 bot) · 4 days stale · ✋ changes requested · 🔴 CI failing · 📝 draft · 🐢 targets chore/rust-dashcore-dev · areas: wallet-storage
  - Top thread: "🔴 Blocking: Preserve script-based ownership after a failed refill**" — 4 days old
- [dashpay/platform#5707 fix(sdk): recover the nonce cache when unconfirmed nonces leave the accepted window](https://github.com/dashpay/platform/pull/5707) — 1 unresolved (1 bot) · 20 days stale · ✋ changes requested · 🔴 CI failing · 📝 draft · 🐢 targets v5.2-dev · areas: rust-sdk · Policy: draft
  - Top thread: "🔴 Blocking: Do not reissue nonces belonging to outstanding operations**" — 20 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/dash-evo-tool#1193 feat!: require Platform protocol 14](https://github.com/dashpay/dash-evo-tool/pull/1193) — 3 unresolved (3 bot) · 2 days stale · ✋ changes requested · 🔴 CI failing · areas: fallback · Policy: waiting-author
  - Top thread: "🔴 Blocking: Retry metadata reads without waiting for the Core wallet scan**" — 2 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/dash-evo-tool#1210 fix(identity): write only what each paid task and refresh changed](https://github.com/dashpay/dash-evo-tool/pull/1210) — 2 unresolved (2 bot) · 1 days stale · ✋ changes requested · 🔴 CI failing · areas: fallback · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Bound the refresh worker's completion wait as well as its lock arrival**" — 1 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/platform#5979 fix(sdk): keep PublicKey unchanged when the inner setter rejects a key](https://github.com/dashpay/platform/pull/5979) — 1 unresolved (1 bot) · 4 days stale · 🐢 targets chore/crypto-core-consumers · areas: fallback
  - Top thread: "🟡 Suggestion: Add successful cross-format setter coverage**" — 4 days old
- [dashpay/platform#5781 fix(wallet-storage): validate transaction bodies during bulk history load](https://github.com/dashpay/platform/pull/5781) — 1 unresolved (1 bot) · 3 days stale · 🔴 CI failing · 🐢 targets v5.2-dev · areas: wallet-storage · Policy: waiting-author
  - Top thread: "🔴 Blocking: Bulk identity validation can panic before Recovery isolates the wallet**" — 3 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/dash-evo-tool#1117 fix(identity): choose signing keys Platform accepts on protocol 14](https://github.com/dashpay/dash-evo-tool/pull/1117) — 1 unresolved (1 bot) · 1 days stale · ✋ changes requested · 🔴 CI failing · areas: fallback · Policy: waiting-bots
  - Top thread: "🟡 Suggestion: Use one signing-key selection per token form frame**" — 1 days old
  - Blocker: coderabbitai has not reported for the current head
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
- [dashpay/dash-evo-tool#1202 feat(boot): ask an AI assistant about a startup failure](https://github.com/dashpay/dash-evo-tool/pull/1202) — 1 unresolved (1 bot) · 1 days stale · ✋ changes requested · 🔴 CI failing · areas: fallback · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Preserve literal filenames in troubleshooting instructions**" — 1 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/dash-evo-tool#1120 feat(identity): add keys with spending limits and raise key limits](https://github.com/dashpay/dash-evo-tool/pull/1120) — ⚠ merge conflict · 📝 draft · 🐢 targets fix/protocol14-signing-key-selection · areas: fallback
- [dashpay/dash-evo-tool#1121 feat(contracts): view and claim contract fee pots](https://github.com/dashpay/dash-evo-tool/pull/1121) — 📝 draft · 🐢 targets fix/protocol14-signing-key-selection · areas: fallback
- [dashpay/dash-evo-tool#1122 feat(documents): agree to document action fees before broadcasting](https://github.com/dashpay/dash-evo-tool/pull/1122) — ⚠ merge conflict · 📝 draft · 🐢 targets fix/protocol14-signing-key-selection · areas: fallback
- [dashpay/dash-evo-tool#1123 feat(identity): add authentication keys bound to a contract](https://github.com/dashpay/dash-evo-tool/pull/1123) — 📝 draft · 🐢 targets feat/identity-key-limits · areas: fallback
- [dashpay/dash-evo-tool#1137 fix(identity): unlock the wallet that holds the chosen signing key](https://github.com/dashpay/dash-evo-tool/pull/1137) — ⚠ merge conflict · 📝 draft · 🐢 targets fix/protocol14-signing-key-selection · areas: fallback
- [dashpay/dash-evo-tool#1172 fix(mcp): keep embedded MCP requests' intermediate results off the GUI](https://github.com/dashpay/dash-evo-tool/pull/1172) — ✋ changes requested · 🔴 CI failing · areas: fallback · Policy: waiting-author
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/dash-evo-tool#1174 feat(skills): mark defects in visual-diff galleries](https://github.com/dashpay/dash-evo-tool/pull/1174) — ⚠ merge conflict · 📝 draft · areas: fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/dash-evo-tool#1175 docs: add maintainable token tutorial production toolkit](https://github.com/dashpay/dash-evo-tool/pull/1175) — areas: fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
  - Blocker: Proceeded without thepastaclaw: no review within the configured window
- [dashpay/dash-evo-tool#1176 feat(tokens): add guided token creation with complete review](https://github.com/dashpay/dash-evo-tool/pull/1176) — ⚠ merge conflict · ✋ changes requested · 📝 draft · 🐢 targets fix/token-creator-stale-identity · areas: github, fallback
- [dashpay/dash-evo-tool#1192 feat(identity): claim token rewards from the identity hub](https://github.com/dashpay/dash-evo-tool/pull/1192) — ⚠ merge conflict · 📝 draft · 🐢 targets feat/identity-hub-tokens · areas: fallback
- [dashpay/dash-evo-tool#1195 fix(dashpay): recover contact requests this device could not record](https://github.com/dashpay/dash-evo-tool/pull/1195) — 📝 draft · 🐢 targets fix/dashpay-contact-info-keys · areas: fallback
- [dashpay/dash-evo-tool#1197 fix(send): offer a private-balance Max the network accepts](https://github.com/dashpay/dash-evo-tool/pull/1197) — 📝 draft · 🐢 targets feat/send-screen-redesign · areas: fallback
- [dashpay/dash-evo-tool#1212 fix(tokens): close the confirmation when a refresh moves the signing key](https://github.com/dashpay/dash-evo-tool/pull/1212) — ⚠ merge conflict · 📝 draft · 🐢 targets fix/protocol14-signing-key-selection · areas: fallback
- [dashpay/dash-evo-tool#1213 refactor(tokens): token send wording and shared UI blocks](https://github.com/dashpay/dash-evo-tool/pull/1213) — ✋ changes requested · 🔴 CI failing · areas: fallback · Policy: waiting-author
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/dash-evo-tool#1214 fix(tokens): identity token model](https://github.com/dashpay/dash-evo-tool/pull/1214) — ⚠ merge conflict · 📝 draft · 🐢 targets refactor/token-send-wording-ui-blocks · areas: fallback
- [dashpay/dash-evo-tool#1215 feat(tokens): refresh token balances on request](https://github.com/dashpay/dash-evo-tool/pull/1215) — 📝 draft · 🐢 targets refactor/identity-token-model · areas: fallback
- [dashpay/dash-evo-tool#1216 feat(identity): Tokens tab in the identity hub](https://github.com/dashpay/dash-evo-tool/pull/1216) — 📝 draft · 🐢 targets feat/token-balance-refresh · areas: fallback
- [dashpay/dash-evo-tool#1217 feat(identity): token detail page in the identity hub](https://github.com/dashpay/dash-evo-tool/pull/1217) — 📝 draft · 🐢 targets feat/identity-hub-tokens-tab · areas: fallback
- [dashpay/dash-evo-tool#1218 feat(identity): tokens card on the identity Home tab](https://github.com/dashpay/dash-evo-tool/pull/1218) — 📝 draft · 🐢 targets feat/identity-hub-token-details · areas: fallback
- [dashpay/dash-evo-tool#1219 fix(tokens): re-importing a dismissed token restores its tracking](https://github.com/dashpay/dash-evo-tool/pull/1219) — 📝 draft · 🐢 targets feat/identity-hub-tokens-tab · areas: fallback
- [dashpay/dash-evo-tool#1222 fix(dashpay): drop the cached contact QR image with the code](https://github.com/dashpay/dash-evo-tool/pull/1222) — 📝 draft · 🐢 targets fix/protocol14-signing-key-selection · areas: fallback
- [dashpay/dash-evo-tool#1224 fix(identity): return identity-save warnings to whoever requested the load](https://github.com/dashpay/dash-evo-tool/pull/1224) — 📝 draft · 🐢 targets fix/mcp-results-off-gui · areas: fallback
- [dashpay/dash-evo-tool#1225 fix(boot): give accurate advice for invalid settings and a denied vault](https://github.com/dashpay/dash-evo-tool/pull/1225) — ⚠ merge conflict · 📝 draft · areas: fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/dash-evo-tool#1227 docs: add the project profile for AI reviewers](https://github.com/dashpay/dash-evo-tool/pull/1227) — areas: fallback · Policy: waiting-build
  - Blocker: Waiting for the build to finish
  - Blocker: Proceeded without coderabbitai: skipped by @lklimek
  - Blocker: Proceeded without thepastaclaw: skipped by @lklimek
- [dashpay/platform#3549 test(platform-wallet): e2e framework + full test suite — triage pins, Found-*/PA-* guards, fail-closed persist, Stage-2 merge](https://github.com/dashpay/platform/pull/3549) — 📝 draft · ⏸ deferred · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk, kotlin-sdk, github, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5384 fix(platform-wallet)!: persist chain truncations so a stored wallet follows a reorg](https://github.com/dashpay/platform/pull/5384) — ⏸ deferred · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk, kotlin-sdk · Policy: waiting-author
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5417 fix(platform-wallet): own per-wallet transaction accounting in Rust for SwiftData](https://github.com/dashpay/platform/pull/5417) — ⏸ deferred · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk
- [dashpay/platform#5647 feat: expose the Rust identity key layout to FFI, Swift and Kotlin hosts](https://github.com/dashpay/platform/pull/5647) — ✋ changes requested · 🐢 targets fix/platform-wallet-dip11-identity-key-layout · areas: rs-platform-wallet-ffi, swift-sdk, kotlin-sdk
- [dashpay/platform#5660 refactor(dpp): own the contract-bounds verdict for a document create](https://github.com/dashpay/platform/pull/5660) — 🐢 targets feat/platform-wallet-public-signing-key-selection · areas: rs-drive-abci, dpp, rs-platform-wallet
- [dashpay/platform#5661 feat: decide which identity keys may be disabled in the wallet, not in the mobile apps](https://github.com/dashpay/platform/pull/5661) — ⚠ merge conflict · 🔴 CI failing · 🐢 targets fix/platform-wallet-dip11-key-rules · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk
- [dashpay/platform#5782 fix(wallet-storage): restore Platform and Core from one wallet snapshot](https://github.com/dashpay/platform/pull/5782) — ⚠ merge conflict · 📝 draft · 🐢 targets feat/sqlite-wallet-restore · areas: wallet-storage
- [dashpay/platform#5783 fix(wallet-storage)!: reconcile historical AssetUnlock identities](https://github.com/dashpay/platform/pull/5783) — ⚠ merge conflict · 📝 draft · 🐢 targets fix/sqlite-bulk-history-identity · areas: rs-platform-wallet, wallet-storage
- [dashpay/platform#5787 fix(platform-wallet): classify asset locks funding foreign credits as outgoing](https://github.com/dashpay/platform/pull/5787) — ⚠ merge conflict · 📝 draft · 🐢 targets feat/sqlite-wallet-restore · areas: rs-platform-wallet, wallet-storage
- [dashpay/platform#5788 fix(platform-wallet): compile shielded-only drain helpers only with the shielded feature](https://github.com/dashpay/platform/pull/5788) — 🐢 targets v5.2-dev · areas: rs-platform-wallet · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
- [dashpay/platform#5951 chore: bump rust-dashcore to dev f0ddfd45](https://github.com/dashpay/platform/pull/5951) — 🐢 targets v5.2-dev · areas: dpp, rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk, js-wasm-sdk, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
- [dashpay/platform#5961 fix(platform-wallet): don't pick a limited key to sign a data contract create or update](https://github.com/dashpay/platform/pull/5961) — 📝 draft · 🐢 targets feat/platform-wallet-public-signing-key-selection · areas: rs-platform-wallet
- [dashpay/platform#6025 feat(platform-wallet): read the chain-following masternode list with its height](https://github.com/dashpay/platform/pull/6025) — 📝 draft · 🐢 targets feat/mn-key-id-check · areas: rs-platform-wallet
- [dashpay/platform#6026 feat(platform-wallet): fetch the key references of any masternode](https://github.com/dashpay/platform/pull/6026) — 📝 draft · 🐢 targets feat/mn-current-list · areas: rs-platform-wallet
- [dashpay/platform#6027 feat(platform-wallet): return the proved block height with a withdrawal's remaining balance](https://github.com/dashpay/platform/pull/6027) — 📝 draft · 🐢 targets feat/mn-key-reference · areas: rs-platform-wallet, rs-platform-wallet-ffi
- [dashpay/platform#6028 feat(platform-wallet): withdraw masternode credits with a wallet master-key provider or a host signer](https://github.com/dashpay/platform/pull/6028) — 📝 draft · 🐢 targets feat/mn-withdrawal-height · areas: rs-platform-wallet, rs-platform-wallet-ffi
- [dashpay/platform#6030 chore(deps): bump rust-dashcore to 27073153 for the provider key lookup and the current masternode list](https://github.com/dashpay/platform/pull/6030) — 🔴 CI failing · 📝 draft · 🐢 targets feat/mn-withdrawal-key-source · areas: dpp, rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk, js-wasm-sdk, rust-dapi, kotlin-sdk, sdk-parity, fallback
- [dashpay/platform#6031 feat(platform-wallet): find a wallet's owner or voting key by its hash](https://github.com/dashpay/platform/pull/6031) — 📝 draft · 🐢 targets chore/mn-dashcore-k1-s1 · areas: rs-platform-wallet, rs-platform-wallet-ffi
- [dashpay/platform#6032 refactor(platform-wallet): read the current masternode list from dash-spv](https://github.com/dashpay/platform/pull/6032) — 📝 draft · 🐢 targets chore/mn-dashcore-k1-s1 · areas: rs-platform-wallet, rs-platform-wallet-ffi
- [dashpay/platform#6035 fix(platform-wallet): keep pre-derived lost funding rows ineligible after restart](https://github.com/dashpay/platform/pull/6035) — 📝 draft · 🐢 targets chore/rust-dashcore-dev · areas: rs-platform-wallet
- [dashpay/platform#6036 refactor(platform-wallet-ffi)!: drop the always-zero balance from CoreAddressEntryFFI](https://github.com/dashpay/platform/pull/6036) — 🔴 CI failing · 📝 draft · 🐢 targets fix/funding-pool-lost-row-in-window · areas: rs-platform-wallet-ffi, swift-sdk, kotlin-sdk
- [dashpay/platform#6037 refactor(platform-wallet): one owner for platform-node pool entries](https://github.com/dashpay/platform/pull/6037) — 📝 draft · 🐢 targets refactor/core-address-entry-drop-balance · areas: rs-platform-wallet, wallet-storage
- [dashpay/tenderdash#1578 fix(statesync): refuse surplus light block responses without blocking](https://github.com/dashpay/tenderdash/pull/1578) — 📝 draft · areas: fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/tenderdash#1627 fix(p2p): bound the simple priority queue, honor channel priorities and count drops](https://github.com/dashpay/tenderdash/pull/1627) — 🔴 CI failing · 📝 draft · areas: fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot

<a id="claudius-maginificent-needs-action"></a>
#### Needs action (12)
- [dashpay/dash-evo-tool#1168 fix(tokens): keep the Token Creator's identity current and its registration state reliable](https://github.com/dashpay/dash-evo-tool/pull/1168) — 9 unresolved (9 bot) · 1 days stale · ✋ changes requested · 🔴 CI failing · areas: fallback · Policy: waiting-author
  - Top thread: "🔴 Blocking: Make the new unit tests compile without the testing feature**" — 1 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/dash-evo-tool#1206 fix(boot): show startup panics and window failures instead of closing](https://github.com/dashpay/dash-evo-tool/pull/1206) — 7 unresolved (7 bot) · 1 days stale · ✋ changes requested · 🔴 CI failing · areas: fallback · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Exercise a constructor panic through the production boot path**" — 1 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/dash-evo-tool#1112 feat(send): one From/To Send form with Auto routing](https://github.com/dashpay/dash-evo-tool/pull/1112) — 5 unresolved (5 bot) · 1 days stale · 🔴 CI failing · areas: fallback · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Ignore parked recipient rows when deciding whether to show Fee options**" — 1 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/dash-evo-tool#1170 fix(ui): retire finished progress banners and describe key protection correctly](https://github.com/dashpay/dash-evo-tool/pull/1170) — 5 unresolved (5 bot) · 1 days stale · ✋ changes requested · 🔴 CI failing · areas: fallback · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Decouple cached preset rules from selector order**" — 1 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/tenderdash#1600 fix(consensus): deliver and process a polka block for a validator left in its round](https://github.com/dashpay/tenderdash/pull/1600) — 2 unresolved (2 bot) · 5 days stale · ✋ changes requested · 🔴 CI failing · areas: fallback · Policy: waiting-author
  - Top thread: "🔴 Blocking: Avoid racing on PartSet completeness in the gossip snapshot**" — 5 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/dash-evo-tool#1200 feat(wallet)!: detect databases needing the platform bridge and remove the bridge](https://github.com/dashpay/dash-evo-tool/pull/1200) — 3 unresolved (3 bot) · 2 days stale · 🔴 CI failing · areas: fallback · Policy: waiting-author
  - Top thread: "🔴 Blocking: The recovery instructions omit previously opened networks without wallets**" — 2 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/dash-evo-tool#1193 feat!: require Platform protocol 14](https://github.com/dashpay/dash-evo-tool/pull/1193) — 3 unresolved (3 bot) · 2 days stale · ✋ changes requested · 🔴 CI failing · areas: fallback · Policy: waiting-author
  - Top thread: "🔴 Blocking: Retry metadata reads without waiting for the Core wallet scan**" — 2 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/dash-evo-tool#1210 fix(identity): write only what each paid task and refresh changed](https://github.com/dashpay/dash-evo-tool/pull/1210) — 2 unresolved (2 bot) · 1 days stale · ✋ changes requested · 🔴 CI failing · areas: fallback · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Bound the refresh worker's completion wait as well as its lock arrival**" — 1 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/dash-evo-tool#1117 fix(identity): choose signing keys Platform accepts on protocol 14](https://github.com/dashpay/dash-evo-tool/pull/1117) — 1 unresolved (1 bot) · 1 days stale · ✋ changes requested · 🔴 CI failing · areas: fallback · Policy: waiting-bots
  - Top thread: "🟡 Suggestion: Use one signing-key selection per token form frame**" — 1 days old
  - Blocker: coderabbitai has not reported for the current head
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
- [dashpay/dash-evo-tool#1202 feat(boot): ask an AI assistant about a startup failure](https://github.com/dashpay/dash-evo-tool/pull/1202) — 1 unresolved (1 bot) · 1 days stale · ✋ changes requested · 🔴 CI failing · areas: fallback · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Preserve literal filenames in troubleshooting instructions**" — 1 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/dash-evo-tool#1172 fix(mcp): keep embedded MCP requests' intermediate results off the GUI](https://github.com/dashpay/dash-evo-tool/pull/1172) — ✋ changes requested · 🔴 CI failing · areas: fallback · Policy: waiting-author
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/dash-evo-tool#1213 refactor(tokens): token send wording and shared UI blocks](https://github.com/dashpay/dash-evo-tool/pull/1213) — ✋ changes requested · 🔴 CI failing · areas: fallback · Policy: waiting-author
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return

<a id="claudius-maginificent-ready-for-human"></a>
#### Ready for human (4)
- [dashpay/platform#6029 feat(platform-wallet): report which identities an identity sync pass read and at which height](https://github.com/dashpay/platform/pull/6029) — 4 unresolved (4 bot) · 2 days stale · 🐢 targets v5.2-dev · areas: rs-platform-wallet · Policy: ready-for-human
  - Top thread: "🟡 Suggestion: Document that the summary reports reads, not persistence receipts**" — 2 days old
  - Blocker: Human approval or objection resolution is required
- [dashpay/dash-evo-tool#1175 docs: add maintainable token tutorial production toolkit](https://github.com/dashpay/dash-evo-tool/pull/1175) — areas: fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
  - Blocker: Proceeded without thepastaclaw: no review within the configured window
- [dashpay/platform#5788 fix(platform-wallet): compile shielded-only drain helpers only with the shielded feature](https://github.com/dashpay/platform/pull/5788) — 🐢 targets v5.2-dev · areas: rs-platform-wallet · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
- [dashpay/platform#5951 chore: bump rust-dashcore to dev f0ddfd45](https://github.com/dashpay/platform/pull/5951) — 🐢 targets v5.2-dev · areas: dpp, rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk, js-wasm-sdk, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required

<a id="claudius-maginificent-unresolved-comments"></a>
#### Unresolved Comments (10)
- [dashpay/dash-evo-tool#1168 fix(tokens): keep the Token Creator's identity current and its registration state reliable](https://github.com/dashpay/dash-evo-tool/pull/1168) — 9 unresolved (9 bot) · 1 days stale · ✋ changes requested · 🔴 CI failing · areas: fallback · Policy: waiting-author
  - Top thread: "🔴 Blocking: Make the new unit tests compile without the testing feature**" — 1 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/dash-evo-tool#1206 fix(boot): show startup panics and window failures instead of closing](https://github.com/dashpay/dash-evo-tool/pull/1206) — 7 unresolved (7 bot) · 1 days stale · ✋ changes requested · 🔴 CI failing · areas: fallback · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Exercise a constructor panic through the production boot path**" — 1 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/dash-evo-tool#1112 feat(send): one From/To Send form with Auto routing](https://github.com/dashpay/dash-evo-tool/pull/1112) — 5 unresolved (5 bot) · 1 days stale · 🔴 CI failing · areas: fallback · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Ignore parked recipient rows when deciding whether to show Fee options**" — 1 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/dash-evo-tool#1170 fix(ui): retire finished progress banners and describe key protection correctly](https://github.com/dashpay/dash-evo-tool/pull/1170) — 5 unresolved (5 bot) · 1 days stale · ✋ changes requested · 🔴 CI failing · areas: fallback · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Decouple cached preset rules from selector order**" — 1 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/tenderdash#1600 fix(consensus): deliver and process a polka block for a validator left in its round](https://github.com/dashpay/tenderdash/pull/1600) — 2 unresolved (2 bot) · 5 days stale · ✋ changes requested · 🔴 CI failing · areas: fallback · Policy: waiting-author
  - Top thread: "🔴 Blocking: Avoid racing on PartSet completeness in the gossip snapshot**" — 5 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/dash-evo-tool#1200 feat(wallet)!: detect databases needing the platform bridge and remove the bridge](https://github.com/dashpay/dash-evo-tool/pull/1200) — 3 unresolved (3 bot) · 2 days stale · 🔴 CI failing · areas: fallback · Policy: waiting-author
  - Top thread: "🔴 Blocking: The recovery instructions omit previously opened networks without wallets**" — 2 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/dash-evo-tool#1193 feat!: require Platform protocol 14](https://github.com/dashpay/dash-evo-tool/pull/1193) — 3 unresolved (3 bot) · 2 days stale · ✋ changes requested · 🔴 CI failing · areas: fallback · Policy: waiting-author
  - Top thread: "🔴 Blocking: Retry metadata reads without waiting for the Core wallet scan**" — 2 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/dash-evo-tool#1210 fix(identity): write only what each paid task and refresh changed](https://github.com/dashpay/dash-evo-tool/pull/1210) — 2 unresolved (2 bot) · 1 days stale · ✋ changes requested · 🔴 CI failing · areas: fallback · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Bound the refresh worker's completion wait as well as its lock arrival**" — 1 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/dash-evo-tool#1117 fix(identity): choose signing keys Platform accepts on protocol 14](https://github.com/dashpay/dash-evo-tool/pull/1117) — 1 unresolved (1 bot) · 1 days stale · ✋ changes requested · 🔴 CI failing · areas: fallback · Policy: waiting-bots
  - Top thread: "🟡 Suggestion: Use one signing-key selection per token form frame**" — 1 days old
  - Blocker: coderabbitai has not reported for the current head
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
- [dashpay/dash-evo-tool#1202 feat(boot): ask an AI assistant about a startup failure](https://github.com/dashpay/dash-evo-tool/pull/1202) — 1 unresolved (1 bot) · 1 days stale · ✋ changes requested · 🔴 CI failing · areas: fallback · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Preserve literal filenames in troubleshooting instructions**" — 1 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return

<a id="claudius-maginificent-changes-requested"></a>
#### Changes Requested (2)
- [dashpay/dash-evo-tool#1172 fix(mcp): keep embedded MCP requests' intermediate results off the GUI](https://github.com/dashpay/dash-evo-tool/pull/1172) — ✋ changes requested · 🔴 CI failing · areas: fallback · Policy: waiting-author
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/dash-evo-tool#1213 refactor(tokens): token send wording and shared UI blocks](https://github.com/dashpay/dash-evo-tool/pull/1213) — ✋ changes requested · 🔴 CI failing · areas: fallback · Policy: waiting-author
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return

<a id="claudius-maginificent-deferred"></a>
#### Deferred (3)
- [dashpay/platform#3549 test(platform-wallet): e2e framework + full test suite — triage pins, Found-*/PA-* guards, fail-closed persist, Stage-2 merge](https://github.com/dashpay/platform/pull/3549) — 📝 draft · ⏸ deferred · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk, kotlin-sdk, github, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5384 fix(platform-wallet)!: persist chain truncations so a stored wallet follows a reorg](https://github.com/dashpay/platform/pull/5384) — ⏸ deferred · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk, kotlin-sdk · Policy: waiting-author
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5417 fix(platform-wallet): own per-wallet transaction accounting in Rust for SwiftData](https://github.com/dashpay/platform/pull/5417) — ⏸ deferred · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk

<a id="claudius-maginificent-draft"></a>
#### Draft (4)
- [dashpay/dash-evo-tool#1174 feat(skills): mark defects in visual-diff galleries](https://github.com/dashpay/dash-evo-tool/pull/1174) — ⚠ merge conflict · 📝 draft · areas: fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/dash-evo-tool#1225 fix(boot): give accurate advice for invalid settings and a denied vault](https://github.com/dashpay/dash-evo-tool/pull/1225) — ⚠ merge conflict · 📝 draft · areas: fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/tenderdash#1578 fix(statesync): refuse surplus light block responses without blocking](https://github.com/dashpay/tenderdash/pull/1578) — 📝 draft · areas: fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/tenderdash#1627 fix(p2p): bound the simple priority queue, honor channel priorities and count drops](https://github.com/dashpay/tenderdash/pull/1627) — 🔴 CI failing · 📝 draft · areas: fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot

<a id="claudius-maginificent-stale"></a>
#### Stale (52)
- [dashpay/platform#5645 fix(platform-wallet)!: check identity creation keys and funding in the Rust core](https://github.com/dashpay/platform/pull/5645) — 8 unresolved (1 CodeRabbit, 7 bot) · 19 days stale · ✋ changes requested · 🔴 CI failing · 🐢 targets v5.2-dev · areas: dpp, rs-platform-wallet, rs-platform-wallet-ffi, rust-sdk · Policy: waiting-author
  - Top thread: "&lt;picture&gt;&lt;img src="https://storage.googleapis.com/coderabbit_public_assets/review-comment-icons-v1/amber-16.svg" width="…" — 19 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: coderabbitai left review threads unresolved; resolve them
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5565 feat(platform-wallet)!: detect inconsistent shielded commitment trees and add explicit rebuild](https://github.com/dashpay/platform/pull/5565) — 7 unresolved (7 bot) · 23 days stale · ✋ changes requested · 🔴 CI failing · 🐢 targets v5.2-dev · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk · Policy: waiting-author
  - Top thread: "⏸️ **Needs a tracking issue** (not yet deferred) at \`3b17ab95\`; see the replies below." — 23 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/dash-evo-tool#1177 feat(ui): align app-wide styles with the token proposal](https://github.com/dashpay/dash-evo-tool/pull/1177) — 13 unresolved (13 bot) · 4 days stale · ✋ changes requested · 📝 draft · 🐢 targets feat/token-creator-workspace · areas: fallback
  - Top thread: "✅ **Resolved** at \`0993a6af\`; see the replies below." — 4 days old
- [dashpay/dash-evo-tool#1178 fix(tokens): keep JSON export responsive when the file chooser stalls](https://github.com/dashpay/dash-evo-tool/pull/1178) — 15 unresolved (7 CodeRabbit, 8 bot) · 2 days stale · ✋ changes requested · 📝 draft · 🐢 targets feat/app-design-system · areas: fallback
  - Top thread: "🟡 Suggestion: Exercise a publication failure after staging succeeds**" — 2 days old
- [dashpay/platform#5588 feat(platform-wallet)!: expose protocol-14-aware signing-key selection](https://github.com/dashpay/platform/pull/5588) — 5 unresolved (5 bot) · 23 days stale · 🔴 CI failing · 🐢 targets v5.2-dev · areas: dpp, rs-platform-wallet, rs-platform-wallet-ffi, rust-sdk · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Include BLS keys in automatic document and token selection**" — 23 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/dash-evo-tool#1108 feat(documents): recognise indexOnly document types and delete their entries by values](https://github.com/dashpay/dash-evo-tool/pull/1108) — 3 unresolved (3 bot) · 30 days stale · ✋ changes requested · 📝 draft · 🐢 targets feat/document-optional-token-cost · areas: fallback
  - Top thread: "🟡 Suggestion: Make the non-deletable error actionable**" — 30 days old
- [dashpay/platform#5644 fix!: follow DIP-11 key rules when disabling keys and reading contact requests](https://github.com/dashpay/platform/pull/5644) — 8 unresolved (8 bot) · 0 days stale · ✋ changes requested · 🔴 CI failing · 🐢 targets v5.2-dev · areas: dpp, rs-platform-wallet, rs-platform-wallet-ffi, rust-sdk, rust-sdk-ffi, js-wasm-sdk, fallback · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Update the PR migration notes for the final client API**" — 0 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5546 fix(platform-wallet): support HASH160 DashPay contact signing keys](https://github.com/dashpay/platform/pull/5546) — 2 unresolved (2 bot) · 23 days stale · ⚠ merge conflict · 🐢 targets feat/platform-wallet-public-signing-key-selection · areas: rs-drive-abci, dpp, rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk, fallback
  - Top thread: "🟡 Suggestion: Do not expose an unchecked signer-capability override in the public constructor**" — 23 days old
- [dashpay/dash-evo-tool#1179 fix(tokens): retain the creator when opening a registered contract](https://github.com/dashpay/dash-evo-tool/pull/1179) — 4 unresolved (4 bot) · 2 days stale · ✋ changes requested · 📝 draft · 🐢 targets fix/token-json-export · areas: fallback
  - Top thread: "🟡 Suggestion: Assert that the preview displays fetched contract details**" — 2 days old
- [dashpay/platform#6029 feat(platform-wallet): report which identities an identity sync pass read and at which height](https://github.com/dashpay/platform/pull/6029) — 4 unresolved (4 bot) · 2 days stale · 🐢 targets v5.2-dev · areas: rs-platform-wallet · Policy: ready-for-human
  - Top thread: "🟡 Suggestion: Document that the summary reports reads, not persistence receipts**" — 2 days old
  - Blocker: Human approval or objection resolution is required
- [dashpay/dash-evo-tool#1124 feat(documents): let the user skip an optional token cost](https://github.com/dashpay/dash-evo-tool/pull/1124) — 1 unresolved (1 bot) · 27 days stale · ⚠ merge conflict · ✋ changes requested · 📝 draft · 🐢 targets fix/protocol14-signing-key-selection · areas: fallback
  - Top thread: "🔴 Blocking: Show the credit-fee outcome when an optional token cost is disabled**" — 27 days old
- [dashpay/platform#5981 refactor(platform-wallet-storage): drop the unreachable rehydration gap-refill guard](https://github.com/dashpay/platform/pull/5981) — 2 unresolved (2 bot) · 4 days stale · ✋ changes requested · 🔴 CI failing · 📝 draft · 🐢 targets chore/rust-dashcore-dev · areas: wallet-storage
  - Top thread: "🔴 Blocking: Preserve script-based ownership after a failed refill**" — 4 days old
- [dashpay/platform#5707 fix(sdk): recover the nonce cache when unconfirmed nonces leave the accepted window](https://github.com/dashpay/platform/pull/5707) — 1 unresolved (1 bot) · 20 days stale · ✋ changes requested · 🔴 CI failing · 📝 draft · 🐢 targets v5.2-dev · areas: rust-sdk · Policy: draft
  - Top thread: "🔴 Blocking: Do not reissue nonces belonging to outstanding operations**" — 20 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5979 fix(sdk): keep PublicKey unchanged when the inner setter rejects a key](https://github.com/dashpay/platform/pull/5979) — 1 unresolved (1 bot) · 4 days stale · 🐢 targets chore/crypto-core-consumers · areas: fallback
  - Top thread: "🟡 Suggestion: Add successful cross-format setter coverage**" — 4 days old
- [dashpay/platform#5781 fix(wallet-storage): validate transaction bodies during bulk history load](https://github.com/dashpay/platform/pull/5781) — 1 unresolved (1 bot) · 3 days stale · 🔴 CI failing · 🐢 targets v5.2-dev · areas: wallet-storage · Policy: waiting-author
  - Top thread: "🔴 Blocking: Bulk identity validation can panic before Recovery isolates the wallet**" — 3 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/dash-evo-tool#1120 feat(identity): add keys with spending limits and raise key limits](https://github.com/dashpay/dash-evo-tool/pull/1120) — ⚠ merge conflict · 📝 draft · 🐢 targets fix/protocol14-signing-key-selection · areas: fallback
- [dashpay/dash-evo-tool#1121 feat(contracts): view and claim contract fee pots](https://github.com/dashpay/dash-evo-tool/pull/1121) — 📝 draft · 🐢 targets fix/protocol14-signing-key-selection · areas: fallback
- [dashpay/dash-evo-tool#1122 feat(documents): agree to document action fees before broadcasting](https://github.com/dashpay/dash-evo-tool/pull/1122) — ⚠ merge conflict · 📝 draft · 🐢 targets fix/protocol14-signing-key-selection · areas: fallback
- [dashpay/dash-evo-tool#1123 feat(identity): add authentication keys bound to a contract](https://github.com/dashpay/dash-evo-tool/pull/1123) — 📝 draft · 🐢 targets feat/identity-key-limits · areas: fallback
- [dashpay/dash-evo-tool#1137 fix(identity): unlock the wallet that holds the chosen signing key](https://github.com/dashpay/dash-evo-tool/pull/1137) — ⚠ merge conflict · 📝 draft · 🐢 targets fix/protocol14-signing-key-selection · areas: fallback
- [dashpay/dash-evo-tool#1176 feat(tokens): add guided token creation with complete review](https://github.com/dashpay/dash-evo-tool/pull/1176) — ⚠ merge conflict · ✋ changes requested · 📝 draft · 🐢 targets fix/token-creator-stale-identity · areas: github, fallback
- [dashpay/dash-evo-tool#1192 feat(identity): claim token rewards from the identity hub](https://github.com/dashpay/dash-evo-tool/pull/1192) — ⚠ merge conflict · 📝 draft · 🐢 targets feat/identity-hub-tokens · areas: fallback
- [dashpay/dash-evo-tool#1195 fix(dashpay): recover contact requests this device could not record](https://github.com/dashpay/dash-evo-tool/pull/1195) — 📝 draft · 🐢 targets fix/dashpay-contact-info-keys · areas: fallback
- [dashpay/dash-evo-tool#1197 fix(send): offer a private-balance Max the network accepts](https://github.com/dashpay/dash-evo-tool/pull/1197) — 📝 draft · 🐢 targets feat/send-screen-redesign · areas: fallback
- [dashpay/dash-evo-tool#1212 fix(tokens): close the confirmation when a refresh moves the signing key](https://github.com/dashpay/dash-evo-tool/pull/1212) — ⚠ merge conflict · 📝 draft · 🐢 targets fix/protocol14-signing-key-selection · areas: fallback
- [dashpay/dash-evo-tool#1214 fix(tokens): identity token model](https://github.com/dashpay/dash-evo-tool/pull/1214) — ⚠ merge conflict · 📝 draft · 🐢 targets refactor/token-send-wording-ui-blocks · areas: fallback
- [dashpay/dash-evo-tool#1215 feat(tokens): refresh token balances on request](https://github.com/dashpay/dash-evo-tool/pull/1215) — 📝 draft · 🐢 targets refactor/identity-token-model · areas: fallback
- [dashpay/dash-evo-tool#1216 feat(identity): Tokens tab in the identity hub](https://github.com/dashpay/dash-evo-tool/pull/1216) — 📝 draft · 🐢 targets feat/token-balance-refresh · areas: fallback
- [dashpay/dash-evo-tool#1217 feat(identity): token detail page in the identity hub](https://github.com/dashpay/dash-evo-tool/pull/1217) — 📝 draft · 🐢 targets feat/identity-hub-tokens-tab · areas: fallback
- [dashpay/dash-evo-tool#1218 feat(identity): tokens card on the identity Home tab](https://github.com/dashpay/dash-evo-tool/pull/1218) — 📝 draft · 🐢 targets feat/identity-hub-token-details · areas: fallback
- [dashpay/dash-evo-tool#1219 fix(tokens): re-importing a dismissed token restores its tracking](https://github.com/dashpay/dash-evo-tool/pull/1219) — 📝 draft · 🐢 targets feat/identity-hub-tokens-tab · areas: fallback
- [dashpay/dash-evo-tool#1222 fix(dashpay): drop the cached contact QR image with the code](https://github.com/dashpay/dash-evo-tool/pull/1222) — 📝 draft · 🐢 targets fix/protocol14-signing-key-selection · areas: fallback
- [dashpay/dash-evo-tool#1224 fix(identity): return identity-save warnings to whoever requested the load](https://github.com/dashpay/dash-evo-tool/pull/1224) — 📝 draft · 🐢 targets fix/mcp-results-off-gui · areas: fallback
- [dashpay/platform#5647 feat: expose the Rust identity key layout to FFI, Swift and Kotlin hosts](https://github.com/dashpay/platform/pull/5647) — ✋ changes requested · 🐢 targets fix/platform-wallet-dip11-identity-key-layout · areas: rs-platform-wallet-ffi, swift-sdk, kotlin-sdk
- [dashpay/platform#5660 refactor(dpp): own the contract-bounds verdict for a document create](https://github.com/dashpay/platform/pull/5660) — 🐢 targets feat/platform-wallet-public-signing-key-selection · areas: rs-drive-abci, dpp, rs-platform-wallet
- [dashpay/platform#5661 feat: decide which identity keys may be disabled in the wallet, not in the mobile apps](https://github.com/dashpay/platform/pull/5661) — ⚠ merge conflict · 🔴 CI failing · 🐢 targets fix/platform-wallet-dip11-key-rules · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk
- [dashpay/platform#5782 fix(wallet-storage): restore Platform and Core from one wallet snapshot](https://github.com/dashpay/platform/pull/5782) — ⚠ merge conflict · 📝 draft · 🐢 targets feat/sqlite-wallet-restore · areas: wallet-storage
- [dashpay/platform#5783 fix(wallet-storage)!: reconcile historical AssetUnlock identities](https://github.com/dashpay/platform/pull/5783) — ⚠ merge conflict · 📝 draft · 🐢 targets fix/sqlite-bulk-history-identity · areas: rs-platform-wallet, wallet-storage
- [dashpay/platform#5787 fix(platform-wallet): classify asset locks funding foreign credits as outgoing](https://github.com/dashpay/platform/pull/5787) — ⚠ merge conflict · 📝 draft · 🐢 targets feat/sqlite-wallet-restore · areas: rs-platform-wallet, wallet-storage
- [dashpay/platform#5788 fix(platform-wallet): compile shielded-only drain helpers only with the shielded feature](https://github.com/dashpay/platform/pull/5788) — 🐢 targets v5.2-dev · areas: rs-platform-wallet · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
- [dashpay/platform#5951 chore: bump rust-dashcore to dev f0ddfd45](https://github.com/dashpay/platform/pull/5951) — 🐢 targets v5.2-dev · areas: dpp, rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk, js-wasm-sdk, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
- [dashpay/platform#5961 fix(platform-wallet): don't pick a limited key to sign a data contract create or update](https://github.com/dashpay/platform/pull/5961) — 📝 draft · 🐢 targets feat/platform-wallet-public-signing-key-selection · areas: rs-platform-wallet
- [dashpay/platform#6025 feat(platform-wallet): read the chain-following masternode list with its height](https://github.com/dashpay/platform/pull/6025) — 📝 draft · 🐢 targets feat/mn-key-id-check · areas: rs-platform-wallet
- [dashpay/platform#6026 feat(platform-wallet): fetch the key references of any masternode](https://github.com/dashpay/platform/pull/6026) — 📝 draft · 🐢 targets feat/mn-current-list · areas: rs-platform-wallet
- [dashpay/platform#6027 feat(platform-wallet): return the proved block height with a withdrawal's remaining balance](https://github.com/dashpay/platform/pull/6027) — 📝 draft · 🐢 targets feat/mn-key-reference · areas: rs-platform-wallet, rs-platform-wallet-ffi
- [dashpay/platform#6028 feat(platform-wallet): withdraw masternode credits with a wallet master-key provider or a host signer](https://github.com/dashpay/platform/pull/6028) — 📝 draft · 🐢 targets feat/mn-withdrawal-height · areas: rs-platform-wallet, rs-platform-wallet-ffi
- [dashpay/platform#6030 chore(deps): bump rust-dashcore to 27073153 for the provider key lookup and the current masternode list](https://github.com/dashpay/platform/pull/6030) — 🔴 CI failing · 📝 draft · 🐢 targets feat/mn-withdrawal-key-source · areas: dpp, rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk, js-wasm-sdk, rust-dapi, kotlin-sdk, sdk-parity, fallback
- [dashpay/platform#6031 feat(platform-wallet): find a wallet's owner or voting key by its hash](https://github.com/dashpay/platform/pull/6031) — 📝 draft · 🐢 targets chore/mn-dashcore-k1-s1 · areas: rs-platform-wallet, rs-platform-wallet-ffi
- [dashpay/platform#6032 refactor(platform-wallet): read the current masternode list from dash-spv](https://github.com/dashpay/platform/pull/6032) — 📝 draft · 🐢 targets chore/mn-dashcore-k1-s1 · areas: rs-platform-wallet, rs-platform-wallet-ffi
- [dashpay/platform#6035 fix(platform-wallet): keep pre-derived lost funding rows ineligible after restart](https://github.com/dashpay/platform/pull/6035) — 📝 draft · 🐢 targets chore/rust-dashcore-dev · areas: rs-platform-wallet
- [dashpay/platform#6036 refactor(platform-wallet-ffi)!: drop the always-zero balance from CoreAddressEntryFFI](https://github.com/dashpay/platform/pull/6036) — 🔴 CI failing · 📝 draft · 🐢 targets fix/funding-pool-lost-row-in-window · areas: rs-platform-wallet-ffi, swift-sdk, kotlin-sdk
- [dashpay/platform#6037 refactor(platform-wallet): one owner for platform-node pool entries](https://github.com/dashpay/platform/pull/6037) — 📝 draft · 🐢 targets refactor/core-address-entry-drop-balance · areas: rs-platform-wallet, wallet-storage

<a id="claudius-maginificent-clean"></a>
#### Clean (2)
- [dashpay/dash-evo-tool#1175 docs: add maintainable token tutorial production toolkit](https://github.com/dashpay/dash-evo-tool/pull/1175) — areas: fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
  - Blocker: Proceeded without thepastaclaw: no review within the configured window
- [dashpay/dash-evo-tool#1227 docs: add the project profile for AI reviewers](https://github.com/dashpay/dash-evo-tool/pull/1227) — areas: fallback · Policy: waiting-build
  - Blocker: Waiting for the build to finish
  - Blocker: Proceeded without coderabbitai: skipped by @lklimek
  - Blocker: Proceeded without thepastaclaw: skipped by @lklimek

<a id="quantumexplorer"></a>
### @QuantumExplorer
<a id="quantumexplorer-open"></a>
#### Open (34)
- [dashpay/platform#5750 feat(platform)!: random token transfer and mint amounts](https://github.com/dashpay/platform/pull/5750) — 13 unresolved (13 bot) · 19 days stale · 🐢 targets claude/platform-tenderdash-randomness-149e4e · areas: rs-drive, dpp, rs-platform-wallet, rust-sdk, rust-sdk-ffi, fallback
  - Top thread: "✅ **Resolved** at \`f8fcc8fd\`; see the replies below." — 19 days old
- [dashpay/platform#5749 feat(platform)!: sys.random.uniform document properties](https://github.com/dashpay/platform/pull/5749) — 9 unresolved (9 bot) · 19 days stale · 🔴 CI failing · 🐢 targets v5.2-dev · areas: rs-drive, rs-drive-abci, dpp, fallback · Policy: waiting-author
  - Top thread: "✅ **Resolved** at \`0ead2b47\`; see the replies below." — 19 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5753 feat(drive-abci)!: apply block randomness through the ApplyBlockRandomness ABCI method](https://github.com/dashpay/platform/pull/5753) — 8 unresolved (8 bot) · 19 days stale · ⚠ merge conflict · 🔴 CI failing · 🐢 targets claude/random-token-amounts · areas: rs-drive, rs-drive-abci, dpp, fallback
  - Top thread: "🟡 Suggestion: Exercise the protocol upgrade with signed commits**" — 19 days old
- [dashpay/platform#6097 fix(sdk): start an unpinned SDK at protocol version 14 on every network](https://github.com/dashpay/platform/pull/6097) — 9 unresolved (3 CodeRabbit, 6 bot) · 0 days stale · ✋ changes requested · 🔴 CI failing · areas: swift-sdk, rust-sdk, js-wasm-sdk, fallback · Policy: waiting-author
  - Top thread: "&lt;picture&gt;&lt;img src="https://storage.googleapis.com/coderabbit_public_assets/review-comment-icons-v1/amber-16.svg" width="…" — 0 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: coderabbitai left review threads unresolved; resolve them
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5529 feat(platform)!: token marketplace listings (PV15)](https://github.com/dashpay/platform/pull/5529) — 2 unresolved (2 bot) · 28 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · 🐢 targets v5.2-dev · areas: rs-drive, rs-drive-abci, dpp, rs-platform-wallet, swift-sdk, kotlin-sdk, fallback · Policy: waiting-author
  - Top thread: "🔴 Blocking: Version the consensus order-ID derivation through platform-version tables**" — 28 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5530 feat(platform)!: token marketplace offers (PV15)](https://github.com/dashpay/platform/pull/5530) — 1 unresolved (1 bot) · 23 days stale · ✋ changes requested · 🔴 CI failing · 🐢 targets claude/token-market-listings · areas: rs-drive, rs-drive-abci, dpp, rs-platform-wallet, fallback
  - Top thread: "🔴 Blocking: Bound bid escrow by a buyer-signed credit limit**" — 23 days old
- [dashpay/tenderdash#1582 feat(abci)!: add the ApplyBlockRandomness method and commit beacon fields](https://github.com/dashpay/tenderdash/pull/1582) — 1 unresolved (1 bot) · 16 days stale · areas: fallback · Policy: waiting-author
  - Top thread: "✅ **Resolved** at \`cd6b4ec8\`; see the replies below." — 16 days old
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/tenderdash#1597 fix(p2p): trim the simple priority queue below its limit](https://github.com/dashpay/tenderdash/pull/1597) — 1 unresolved (1 bot) · 14 days stale · ⚠ merge conflict · areas: fallback · Policy: waiting-self-review
  - Top thread: "🟡 Suggestion: Count the larger overflow batches**" — 14 days old
  - Blocker: Author must post /self-reviewed fb3e02bfc772facdefade06c2eeca6743e23e329
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
- [dashpay/platform#5967 feat(sdk): host apps can prepare an identity's shielded account](https://github.com/dashpay/platform/pull/5967) — 📝 draft · areas: rs-platform-wallet-ffi, swift-sdk, kotlin-sdk · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#6002 fix(dashmate): run an added local node on its network's Core image](https://github.com/dashpay/platform/pull/6002) — areas: dashmate · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 9cc7778487200a5f379f0cb47cfff854e1a520dd
- [dashpay/platform#6038 feat(platform): prove the state before and after the block of a state transition](https://github.com/dashpay/platform/pull/6038) — ✋ changes requested · 🔴 CI failing · 🐢 targets v5.2-dev · areas: rs-drive, rs-drive-abci, rs-platform-wallet, rs-platform-wallet-ffi, rust-sdk, js-wasm-sdk, rust-dapi, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed ce87c4ba91c01d646c7fa966b385ec7e215d90af
- [dashpay/platform#6084 fix(swift-example-app): remove an identity from the device children first, keys after the rows](https://github.com/dashpay/platform/pull/6084) — ✋ changes requested · 🔴 CI failing · areas: swift-sdk · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head
- [dashpay/platform#6085 fix(swift-example-app): keep key signing metadata and contract bounds when refreshing an identity](https://github.com/dashpay/platform/pull/6085) — ✋ changes requested · areas: swift-sdk · Policy: waiting-author
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#6086 feat: check a masternode voting key against Platform when it is loaded](https://github.com/dashpay/platform/pull/6086) — areas: swift-sdk, rust-sdk, rust-sdk-ffi · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head
- [dashpay/platform#6087 feat: sign KotlinExampleApp masternode votes with a stored voting key through the signer](https://github.com/dashpay/platform/pull/6087) — ✋ changes requested · areas: kotlin-sdk, sdk-parity · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head
- [dashpay/platform#6088 feat(swift-example-app): show a wallet's Platform address balance history, and list the address-sync queries in the catalog](https://github.com/dashpay/platform/pull/6088) — ✋ changes requested · 🔴 CI failing · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, rust-sdk, rust-sdk-ffi, kotlin-sdk, sdk-parity · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head
- [dashpay/platform#6089 feat: show where an identity's withdrawals to Core stand in SwiftExampleApp](https://github.com/dashpay/platform/pull/6089) — ✋ changes requested · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, rust-sdk, kotlin-sdk, sdk-parity · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head
- [dashpay/platform#6090 feat(swift-example-app): buy a token from the shielded pool (type 28)](https://github.com/dashpay/platform/pull/6090) — ✋ changes requested · areas: dpp, rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, rust-sdk, kotlin-sdk, sdk-parity · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head
- [dashpay/platform#6091 feat(swift-example-app): browse blocks and check a state transition's status by hash](https://github.com/dashpay/platform/pull/6091) — areas: swift-sdk, rust-sdk, rust-sdk-ffi, kotlin-sdk, sdk-parity, fallback · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head
- [dashpay/platform#6092 fix(platform)!: bound decoded size and refuse zero-width arrays (PV15)](https://github.com/dashpay/platform/pull/6092) — ✋ changes requested · areas: rs-drive, dpp, rust-sdk, rust-sdk-ffi, js-wasm-sdk, fallback · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head
- [dashpay/platform#6094 feat: set, change and remove DashPay profile payment addresses in SwiftExampleApp](https://github.com/dashpay/platform/pull/6094) — 📝 draft · 🐢 targets feat/dashpay-shielded-tips · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk, sdk-parity
- [dashpay/platform#6095 feat: support document sums past 64 bits end to end on mobile](https://github.com/dashpay/platform/pull/6095) — ✋ changes requested · 🔴 CI failing · areas: rs-drive, rs-drive-abci, swift-sdk, rust-sdk, rust-sdk-ffi, js-wasm-sdk, kotlin-sdk, github, fallback · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head
- [dashpay/platform#6101 fix(swift-example-app): fix the compile error and the three failing tests in the Swift job](https://github.com/dashpay/platform/pull/6101) — areas: swift-sdk · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head
- [dashpay/tenderdash#1583 feat(consensus)!: recover a beacon from prevotes and sign precommits over the randomness-applied app hash](https://github.com/dashpay/tenderdash/pull/1583) — ⚠ merge conflict · 📝 draft · areas: fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/tenderdash#1584 fix(statesync): restore LastAppHash of the snapshot height, not the next one](https://github.com/dashpay/tenderdash/pull/1584) — areas: fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 23c1443e4077c7c7dfedf00f9446c762e669660f

<a id="quantumexplorer-needs-action"></a>
#### Needs action (11)
- [dashpay/platform#6097 fix(sdk): start an unpinned SDK at protocol version 14 on every network](https://github.com/dashpay/platform/pull/6097) — 9 unresolved (3 CodeRabbit, 6 bot) · 0 days stale · ✋ changes requested · 🔴 CI failing · areas: swift-sdk, rust-sdk, js-wasm-sdk, fallback · Policy: waiting-author
  - Top thread: "&lt;picture&gt;&lt;img src="https://storage.googleapis.com/coderabbit_public_assets/review-comment-icons-v1/amber-16.svg" width="…" — 0 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: coderabbitai left review threads unresolved; resolve them
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/tenderdash#1582 feat(abci)!: add the ApplyBlockRandomness method and commit beacon fields](https://github.com/dashpay/tenderdash/pull/1582) — 1 unresolved (1 bot) · 16 days stale · areas: fallback · Policy: waiting-author
  - Top thread: "✅ **Resolved** at \`cd6b4ec8\`; see the replies below." — 16 days old
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/tenderdash#1597 fix(p2p): trim the simple priority queue below its limit](https://github.com/dashpay/tenderdash/pull/1597) — 1 unresolved (1 bot) · 14 days stale · ⚠ merge conflict · areas: fallback · Policy: waiting-self-review
  - Top thread: "🟡 Suggestion: Count the larger overflow batches**" — 14 days old
  - Blocker: Author must post /self-reviewed fb3e02bfc772facdefade06c2eeca6743e23e329
- [dashpay/platform#6084 fix(swift-example-app): remove an identity from the device children first, keys after the rows](https://github.com/dashpay/platform/pull/6084) — ✋ changes requested · 🔴 CI failing · areas: swift-sdk · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head
- [dashpay/platform#6085 fix(swift-example-app): keep key signing metadata and contract bounds when refreshing an identity](https://github.com/dashpay/platform/pull/6085) — ✋ changes requested · areas: swift-sdk · Policy: waiting-author
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#6087 feat: sign KotlinExampleApp masternode votes with a stored voting key through the signer](https://github.com/dashpay/platform/pull/6087) — ✋ changes requested · areas: kotlin-sdk, sdk-parity · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head
- [dashpay/platform#6088 feat(swift-example-app): show a wallet's Platform address balance history, and list the address-sync queries in the catalog](https://github.com/dashpay/platform/pull/6088) — ✋ changes requested · 🔴 CI failing · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, rust-sdk, rust-sdk-ffi, kotlin-sdk, sdk-parity · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head
- [dashpay/platform#6089 feat: show where an identity's withdrawals to Core stand in SwiftExampleApp](https://github.com/dashpay/platform/pull/6089) — ✋ changes requested · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, rust-sdk, kotlin-sdk, sdk-parity · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head
- [dashpay/platform#6090 feat(swift-example-app): buy a token from the shielded pool (type 28)](https://github.com/dashpay/platform/pull/6090) — ✋ changes requested · areas: dpp, rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, rust-sdk, kotlin-sdk, sdk-parity · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head
- [dashpay/platform#6092 fix(platform)!: bound decoded size and refuse zero-width arrays (PV15)](https://github.com/dashpay/platform/pull/6092) — ✋ changes requested · areas: rs-drive, dpp, rust-sdk, rust-sdk-ffi, js-wasm-sdk, fallback · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head
- [dashpay/platform#6095 feat: support document sums past 64 bits end to end on mobile](https://github.com/dashpay/platform/pull/6095) — ✋ changes requested · 🔴 CI failing · areas: rs-drive, rs-drive-abci, swift-sdk, rust-sdk, rust-sdk-ffi, js-wasm-sdk, kotlin-sdk, github, fallback · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head

<a id="quantumexplorer-unresolved-comments"></a>
#### Unresolved Comments (3)
- [dashpay/platform#6097 fix(sdk): start an unpinned SDK at protocol version 14 on every network](https://github.com/dashpay/platform/pull/6097) — 9 unresolved (3 CodeRabbit, 6 bot) · 0 days stale · ✋ changes requested · 🔴 CI failing · areas: swift-sdk, rust-sdk, js-wasm-sdk, fallback · Policy: waiting-author
  - Top thread: "&lt;picture&gt;&lt;img src="https://storage.googleapis.com/coderabbit_public_assets/review-comment-icons-v1/amber-16.svg" width="…" — 0 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: coderabbitai left review threads unresolved; resolve them
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/tenderdash#1582 feat(abci)!: add the ApplyBlockRandomness method and commit beacon fields](https://github.com/dashpay/tenderdash/pull/1582) — 1 unresolved (1 bot) · 16 days stale · areas: fallback · Policy: waiting-author
  - Top thread: "✅ **Resolved** at \`cd6b4ec8\`; see the replies below." — 16 days old
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/tenderdash#1597 fix(p2p): trim the simple priority queue below its limit](https://github.com/dashpay/tenderdash/pull/1597) — 1 unresolved (1 bot) · 14 days stale · ⚠ merge conflict · areas: fallback · Policy: waiting-self-review
  - Top thread: "🟡 Suggestion: Count the larger overflow batches**" — 14 days old
  - Blocker: Author must post /self-reviewed fb3e02bfc772facdefade06c2eeca6743e23e329

<a id="quantumexplorer-changes-requested"></a>
#### Changes Requested (8)
- [dashpay/platform#6084 fix(swift-example-app): remove an identity from the device children first, keys after the rows](https://github.com/dashpay/platform/pull/6084) — ✋ changes requested · 🔴 CI failing · areas: swift-sdk · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head
- [dashpay/platform#6085 fix(swift-example-app): keep key signing metadata and contract bounds when refreshing an identity](https://github.com/dashpay/platform/pull/6085) — ✋ changes requested · areas: swift-sdk · Policy: waiting-author
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#6087 feat: sign KotlinExampleApp masternode votes with a stored voting key through the signer](https://github.com/dashpay/platform/pull/6087) — ✋ changes requested · areas: kotlin-sdk, sdk-parity · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head
- [dashpay/platform#6088 feat(swift-example-app): show a wallet's Platform address balance history, and list the address-sync queries in the catalog](https://github.com/dashpay/platform/pull/6088) — ✋ changes requested · 🔴 CI failing · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, rust-sdk, rust-sdk-ffi, kotlin-sdk, sdk-parity · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head
- [dashpay/platform#6089 feat: show where an identity's withdrawals to Core stand in SwiftExampleApp](https://github.com/dashpay/platform/pull/6089) — ✋ changes requested · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, rust-sdk, kotlin-sdk, sdk-parity · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head
- [dashpay/platform#6090 feat(swift-example-app): buy a token from the shielded pool (type 28)](https://github.com/dashpay/platform/pull/6090) — ✋ changes requested · areas: dpp, rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, rust-sdk, kotlin-sdk, sdk-parity · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head
- [dashpay/platform#6092 fix(platform)!: bound decoded size and refuse zero-width arrays (PV15)](https://github.com/dashpay/platform/pull/6092) — ✋ changes requested · areas: rs-drive, dpp, rust-sdk, rust-sdk-ffi, js-wasm-sdk, fallback · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head
- [dashpay/platform#6095 feat: support document sums past 64 bits end to end on mobile](https://github.com/dashpay/platform/pull/6095) — ✋ changes requested · 🔴 CI failing · areas: rs-drive, rs-drive-abci, swift-sdk, rust-sdk, rust-sdk-ffi, js-wasm-sdk, kotlin-sdk, github, fallback · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head

<a id="quantumexplorer-draft"></a>
#### Draft (2)
- [dashpay/platform#5967 feat(sdk): host apps can prepare an identity's shielded account](https://github.com/dashpay/platform/pull/5967) — 📝 draft · areas: rs-platform-wallet-ffi, swift-sdk, kotlin-sdk · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/tenderdash#1583 feat(consensus)!: recover a beacon from prevotes and sign precommits over the randomness-applied app hash](https://github.com/dashpay/tenderdash/pull/1583) — ⚠ merge conflict · 📝 draft · areas: fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot

<a id="quantumexplorer-stale"></a>
#### Stale (16)
- [dashpay/platform#5750 feat(platform)!: random token transfer and mint amounts](https://github.com/dashpay/platform/pull/5750) — 13 unresolved (13 bot) · 19 days stale · 🐢 targets claude/platform-tenderdash-randomness-149e4e · areas: rs-drive, dpp, rs-platform-wallet, rust-sdk, rust-sdk-ffi, fallback
  - Top thread: "✅ **Resolved** at \`f8fcc8fd\`; see the replies below." — 19 days old
- [dashpay/platform#5749 feat(platform)!: sys.random.uniform document properties](https://github.com/dashpay/platform/pull/5749) — 9 unresolved (9 bot) · 19 days stale · 🔴 CI failing · 🐢 targets v5.2-dev · areas: rs-drive, rs-drive-abci, dpp, fallback · Policy: waiting-author
  - Top thread: "✅ **Resolved** at \`0ead2b47\`; see the replies below." — 19 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5753 feat(drive-abci)!: apply block randomness through the ApplyBlockRandomness ABCI method](https://github.com/dashpay/platform/pull/5753) — 8 unresolved (8 bot) · 19 days stale · ⚠ merge conflict · 🔴 CI failing · 🐢 targets claude/random-token-amounts · areas: rs-drive, rs-drive-abci, dpp, fallback
  - Top thread: "🟡 Suggestion: Exercise the protocol upgrade with signed commits**" — 19 days old
- [dashpay/platform#5529 feat(platform)!: token marketplace listings (PV15)](https://github.com/dashpay/platform/pull/5529) — 2 unresolved (2 bot) · 28 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · 🐢 targets v5.2-dev · areas: rs-drive, rs-drive-abci, dpp, rs-platform-wallet, swift-sdk, kotlin-sdk, fallback · Policy: waiting-author
  - Top thread: "🔴 Blocking: Version the consensus order-ID derivation through platform-version tables**" — 28 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5530 feat(platform)!: token marketplace offers (PV15)](https://github.com/dashpay/platform/pull/5530) — 1 unresolved (1 bot) · 23 days stale · ✋ changes requested · 🔴 CI failing · 🐢 targets claude/token-market-listings · areas: rs-drive, rs-drive-abci, dpp, rs-platform-wallet, fallback
  - Top thread: "🔴 Blocking: Bound bid escrow by a buyer-signed credit limit**" — 23 days old
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
- [dashpay/platform#6038 feat(platform): prove the state before and after the block of a state transition](https://github.com/dashpay/platform/pull/6038) — ✋ changes requested · 🔴 CI failing · 🐢 targets v5.2-dev · areas: rs-drive, rs-drive-abci, rs-platform-wallet, rs-platform-wallet-ffi, rust-sdk, js-wasm-sdk, rust-dapi, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed ce87c4ba91c01d646c7fa966b385ec7e215d90af
- [dashpay/platform#6094 feat: set, change and remove DashPay profile payment addresses in SwiftExampleApp](https://github.com/dashpay/platform/pull/6094) — 📝 draft · 🐢 targets feat/dashpay-shielded-tips · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk, sdk-parity

<a id="quantumexplorer-clean"></a>
#### Clean (5)
- [dashpay/platform#6002 fix(dashmate): run an added local node on its network's Core image](https://github.com/dashpay/platform/pull/6002) — areas: dashmate · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 9cc7778487200a5f379f0cb47cfff854e1a520dd
- [dashpay/platform#6086 feat: check a masternode voting key against Platform when it is loaded](https://github.com/dashpay/platform/pull/6086) — areas: swift-sdk, rust-sdk, rust-sdk-ffi · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head
- [dashpay/platform#6091 feat(swift-example-app): browse blocks and check a state transition's status by hash](https://github.com/dashpay/platform/pull/6091) — areas: swift-sdk, rust-sdk, rust-sdk-ffi, kotlin-sdk, sdk-parity, fallback · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head
- [dashpay/platform#6101 fix(swift-example-app): fix the compile error and the three failing tests in the Swift job](https://github.com/dashpay/platform/pull/6101) — areas: swift-sdk · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head
- [dashpay/tenderdash#1584 fix(statesync): restore LastAppHash of the snapshot height, not the next one](https://github.com/dashpay/tenderdash/pull/1584) — areas: fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 23c1443e4077c7c7dfedf00f9446c762e669660f

<a id="quantumexplorer-ready-for-review"></a>
#### Ready for Review (3)
- [dashpay/platform#6103 ci: set up Manki review](https://github.com/dashpay/platform/pull/6103) — by @xdustinface · areas: github, fallback · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head
- [dashpay/rust-dashcore#1482 fix(dash-spv): skip banned peers before sampling candidates](https://github.com/dashpay/rust-dashcore/pull/1482) — by @xdustinface · areas: dash-spv · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed a5aaf4a4212990db26ba8ab745d58a1ac98fbdb5
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/rust-dashcore#1483 fix(dash-spv): keep bans in force across a redial](https://github.com/dashpay/rust-dashcore/pull/1483) — by @xdustinface · areas: dash-spv · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 1f7aca21e9eae5dedde21234a29fce56fb3e0965
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return

<a id="shumkov"></a>
### @shumkov
<a id="shumkov-open"></a>
#### Open (12)
- [dashpay/rust-dashcore#634 feat(key-wallet-manager): single-map WalletManager with Arc&lt;RwLock&lt;T&gt;&gt; per wallet](https://github.com/dashpay/rust-dashcore/pull/634) — 20 unresolved (20 CodeRabbit) · 228 days stale · ⚠ merge conflict · ✋ changes requested · areas: dash-spv, key-wallet, key-wallet-manager, fallback · Policy: waiting-author
  - Top thread: "_⚠️ Potential issue_ \| _🟡 Minor_" — 228 days old
  - Blocker: coderabbitai requested changes on this head; dismiss the review or push a fix
  - Blocker: coderabbitai left review threads unresolved; resolve them
- [dashpay/platform#6013 fix(platform)!: close the gaps left in nested member update rules (PV15)](https://github.com/dashpay/platform/pull/6013) — 4 unresolved (1 CodeRabbit, 3 bot) · 0 days stale · ✋ changes requested · 🔴 CI failing · areas: rs-drive, rs-drive-abci, dpp, fallback · Policy: waiting-author
  - Top thread: "&lt;picture&gt;&lt;img src="https://storage.googleapis.com/coderabbit_public_assets/review-comment-icons-v1/amber-16.svg" width="…" — 0 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: coderabbitai left review threads unresolved; resolve them
- [dashpay/platform#5656 fix(dashmate): build local images for every group node that builds any](https://github.com/dashpay/platform/pull/5656) — 1 unresolved (1 bot) · 23 days stale · ✋ changes requested · 🔴 CI failing · 🐢 targets fix/dashmate/update-path-bugs · areas: dashmate
  - Top thread: "🔴 Blocking: Build the shared base image before dependent service images across group configurations**" — 23 days old
- [dashpay/platform#6040 ci: let PR Hygiene post its verdicts to the dashboard](https://github.com/dashpay/platform/pull/6040) — 1 unresolved (1 bot) · 1 days stale · 🐢 targets v5.2-dev · areas: github · Policy: waiting-self-review
  - Top thread: "🟡 Suggestion: Send the published gate outcome to the dashboard**" — 1 days old
  - Blocker: Author must post /self-reviewed c3274e2f70658beb10bef680020e34d7e8ee7d14
- [dashpay/platform#6041 ci: let PR Hygiene post its verdicts to the dashboard](https://github.com/dashpay/platform/pull/6041) — 1 unresolved (1 bot) · 1 days stale · 🐢 targets v6.0-dev · areas: github · Policy: waiting-self-review
  - Top thread: "🟡 Suggestion: Only push verdicts that match the status published to GitHub**" — 1 days old
  - Blocker: Author must post /self-reviewed 39dba09c717a065949849349981b0de45582486e
- [dashpay/tenderdash#1634 ci: let PR Hygiene post its verdicts to the dashboard](https://github.com/dashpay/tenderdash/pull/1634) — 1 unresolved (1 bot) · 1 days stale · ✋ changes requested · 🔴 CI failing · areas: github · Policy: waiting-author
  - Top thread: "🔴 Blocking: Do not push a mergeable verdict that reconciliation withheld**" — 1 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/dash-evo-tool#1223 ci: let PR Hygiene post its verdicts to the dashboard](https://github.com/dashpay/dash-evo-tool/pull/1223) — areas: github · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 4eadf3941f9fdacf6d0d3308f6d5541c451cc9fa
- [dashpay/platform#4283 fix(dashmate): keep the node up when an image pull fails, and stop reporting success when it did not](https://github.com/dashpay/platform/pull/4283) — ⚠ merge conflict · 🐢 targets v5.2-dev · areas: dashmate · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 3ceb3315ea9b383ae5816ea4f0e9961202770ed9
  - Blocker: Proceeded without thepastaclaw: no review within the configured window
- [dashpay/platform#5763 docs: e2e test architecture and legacy JS retirement plans (draft, not for merge)](https://github.com/dashpay/platform/pull/5763) — 📝 draft · 🐢 targets v5.2-dev · areas: fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5819 feat(dashmate): add the modules that let dashmate update itself](https://github.com/dashpay/platform/pull/5819) — 📝 draft · 🐢 targets fix/dashmate/update-path-bugs · areas: dashmate
- [dashpay/platform#6039 ci: let PR Hygiene post its verdicts to the dashboard](https://github.com/dashpay/platform/pull/6039) — areas: github · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed b3c26087e7c379feb37d33b49714c1ff192d6aae
- [dashpay/rust-dashcore#1475 ci: let PR Hygiene post its verdicts to the dashboard](https://github.com/dashpay/rust-dashcore/pull/1475) — ✋ changes requested · 🔴 CI failing · areas: github · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed b1677af1446b4882a66acb1ef6fe90c80b0c763f

<a id="shumkov-needs-action"></a>
#### Needs action (4)
- [dashpay/rust-dashcore#634 feat(key-wallet-manager): single-map WalletManager with Arc&lt;RwLock&lt;T&gt;&gt; per wallet](https://github.com/dashpay/rust-dashcore/pull/634) — 20 unresolved (20 CodeRabbit) · 228 days stale · ⚠ merge conflict · ✋ changes requested · areas: dash-spv, key-wallet, key-wallet-manager, fallback · Policy: waiting-author
  - Top thread: "_⚠️ Potential issue_ \| _🟡 Minor_" — 228 days old
  - Blocker: coderabbitai requested changes on this head; dismiss the review or push a fix
  - Blocker: coderabbitai left review threads unresolved; resolve them
- [dashpay/platform#6013 fix(platform)!: close the gaps left in nested member update rules (PV15)](https://github.com/dashpay/platform/pull/6013) — 4 unresolved (1 CodeRabbit, 3 bot) · 0 days stale · ✋ changes requested · 🔴 CI failing · areas: rs-drive, rs-drive-abci, dpp, fallback · Policy: waiting-author
  - Top thread: "&lt;picture&gt;&lt;img src="https://storage.googleapis.com/coderabbit_public_assets/review-comment-icons-v1/amber-16.svg" width="…" — 0 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: coderabbitai left review threads unresolved; resolve them
- [dashpay/tenderdash#1634 ci: let PR Hygiene post its verdicts to the dashboard](https://github.com/dashpay/tenderdash/pull/1634) — 1 unresolved (1 bot) · 1 days stale · ✋ changes requested · 🔴 CI failing · areas: github · Policy: waiting-author
  - Top thread: "🔴 Blocking: Do not push a mergeable verdict that reconciliation withheld**" — 1 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/rust-dashcore#1475 ci: let PR Hygiene post its verdicts to the dashboard](https://github.com/dashpay/rust-dashcore/pull/1475) — ✋ changes requested · 🔴 CI failing · areas: github · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed b1677af1446b4882a66acb1ef6fe90c80b0c763f

<a id="shumkov-unresolved-comments"></a>
#### Unresolved Comments (3)
- [dashpay/rust-dashcore#634 feat(key-wallet-manager): single-map WalletManager with Arc&lt;RwLock&lt;T&gt;&gt; per wallet](https://github.com/dashpay/rust-dashcore/pull/634) — 20 unresolved (20 CodeRabbit) · 228 days stale · ⚠ merge conflict · ✋ changes requested · areas: dash-spv, key-wallet, key-wallet-manager, fallback · Policy: waiting-author
  - Top thread: "_⚠️ Potential issue_ \| _🟡 Minor_" — 228 days old
  - Blocker: coderabbitai requested changes on this head; dismiss the review or push a fix
  - Blocker: coderabbitai left review threads unresolved; resolve them
- [dashpay/platform#6013 fix(platform)!: close the gaps left in nested member update rules (PV15)](https://github.com/dashpay/platform/pull/6013) — 4 unresolved (1 CodeRabbit, 3 bot) · 0 days stale · ✋ changes requested · 🔴 CI failing · areas: rs-drive, rs-drive-abci, dpp, fallback · Policy: waiting-author
  - Top thread: "&lt;picture&gt;&lt;img src="https://storage.googleapis.com/coderabbit_public_assets/review-comment-icons-v1/amber-16.svg" width="…" — 0 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: coderabbitai left review threads unresolved; resolve them
- [dashpay/tenderdash#1634 ci: let PR Hygiene post its verdicts to the dashboard](https://github.com/dashpay/tenderdash/pull/1634) — 1 unresolved (1 bot) · 1 days stale · ✋ changes requested · 🔴 CI failing · areas: github · Policy: waiting-author
  - Top thread: "🔴 Blocking: Do not push a mergeable verdict that reconciliation withheld**" — 1 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them

<a id="shumkov-changes-requested"></a>
#### Changes Requested (1)
- [dashpay/rust-dashcore#1475 ci: let PR Hygiene post its verdicts to the dashboard](https://github.com/dashpay/rust-dashcore/pull/1475) — ✋ changes requested · 🔴 CI failing · areas: github · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed b1677af1446b4882a66acb1ef6fe90c80b0c763f

<a id="shumkov-stale"></a>
#### Stale (6)
- [dashpay/platform#5656 fix(dashmate): build local images for every group node that builds any](https://github.com/dashpay/platform/pull/5656) — 1 unresolved (1 bot) · 23 days stale · ✋ changes requested · 🔴 CI failing · 🐢 targets fix/dashmate/update-path-bugs · areas: dashmate
  - Top thread: "🔴 Blocking: Build the shared base image before dependent service images across group configurations**" — 23 days old
- [dashpay/platform#6040 ci: let PR Hygiene post its verdicts to the dashboard](https://github.com/dashpay/platform/pull/6040) — 1 unresolved (1 bot) · 1 days stale · 🐢 targets v5.2-dev · areas: github · Policy: waiting-self-review
  - Top thread: "🟡 Suggestion: Send the published gate outcome to the dashboard**" — 1 days old
  - Blocker: Author must post /self-reviewed c3274e2f70658beb10bef680020e34d7e8ee7d14
- [dashpay/platform#6041 ci: let PR Hygiene post its verdicts to the dashboard](https://github.com/dashpay/platform/pull/6041) — 1 unresolved (1 bot) · 1 days stale · 🐢 targets v6.0-dev · areas: github · Policy: waiting-self-review
  - Top thread: "🟡 Suggestion: Only push verdicts that match the status published to GitHub**" — 1 days old
  - Blocker: Author must post /self-reviewed 39dba09c717a065949849349981b0de45582486e
- [dashpay/platform#4283 fix(dashmate): keep the node up when an image pull fails, and stop reporting success when it did not](https://github.com/dashpay/platform/pull/4283) — ⚠ merge conflict · 🐢 targets v5.2-dev · areas: dashmate · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 3ceb3315ea9b383ae5816ea4f0e9961202770ed9
  - Blocker: Proceeded without thepastaclaw: no review within the configured window
- [dashpay/platform#5763 docs: e2e test architecture and legacy JS retirement plans (draft, not for merge)](https://github.com/dashpay/platform/pull/5763) — 📝 draft · 🐢 targets v5.2-dev · areas: fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5819 feat(dashmate): add the modules that let dashmate update itself](https://github.com/dashpay/platform/pull/5819) — 📝 draft · 🐢 targets fix/dashmate/update-path-bugs · areas: dashmate

<a id="shumkov-clean"></a>
#### Clean (2)
- [dashpay/dash-evo-tool#1223 ci: let PR Hygiene post its verdicts to the dashboard](https://github.com/dashpay/dash-evo-tool/pull/1223) — areas: github · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 4eadf3941f9fdacf6d0d3308f6d5541c451cc9fa
- [dashpay/platform#6039 ci: let PR Hygiene post its verdicts to the dashboard](https://github.com/dashpay/platform/pull/6039) — areas: github · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed b3c26087e7c379feb37d33b49714c1ff192d6aae

<a id="shumkov-ready-for-review"></a>
#### Ready for Review (7)
- [dashpay/dash-evo-tool#1161 fix(ci): isolate release signing and constrain agent write paths](https://github.com/dashpay/dash-evo-tool/pull/1161) — by @lklimek · areas: github, fallback · Policy: waiting-bots
  - Blocker: coderabbitai has not reported for the current head
  - Blocker: thepastaclaw has not reported for the current head
- [dashpay/platform#6002 fix(dashmate): run an added local node on its network's Core image](https://github.com/dashpay/platform/pull/6002) — by @QuantumExplorer · areas: dashmate · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 9cc7778487200a5f379f0cb47cfff854e1a520dd
- [dashpay/platform#6086 feat: check a masternode voting key against Platform when it is loaded](https://github.com/dashpay/platform/pull/6086) — by @QuantumExplorer · areas: swift-sdk, rust-sdk, rust-sdk-ffi · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head
- [dashpay/platform#6091 feat(swift-example-app): browse blocks and check a state transition's status by hash](https://github.com/dashpay/platform/pull/6091) — by @QuantumExplorer · areas: swift-sdk, rust-sdk, rust-sdk-ffi, kotlin-sdk, sdk-parity, fallback · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head
- [dashpay/platform#6103 ci: set up Manki review](https://github.com/dashpay/platform/pull/6103) — by @xdustinface · areas: github, fallback · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head
- [dashpay/rust-dashcore#1459 ci: merge audit and verify jobs into one](https://github.com/dashpay/rust-dashcore/pull/1459) — by @ZocoLini · areas: github · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed e20edef48b4122dad05e5bc08ab8b19f190c0f38
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/tenderdash#1584 fix(statesync): restore LastAppHash of the snapshot height, not the next one](https://github.com/dashpay/tenderdash/pull/1584) — by @QuantumExplorer · areas: fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 23c1443e4077c7c7dfedf00f9446c762e669660f

<a id="lklimek"></a>
### @lklimek
<a id="lklimek-open"></a>
#### Open (16)
- [dashpay/platform#5528 fix(platform-wallet): retain drain operation history through funding conflicts](https://github.com/dashpay/platform/pull/5528) — 5 unresolved (5 bot) · 32 days stale · ⚠ merge conflict · ✋ changes requested · 📝 draft · 🐢 targets feat/sqlite-wallet-restore · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk, kotlin-sdk
  - Top thread: "🟡 Suggestion: Retain diagnostics for malformed FFI operation snapshots**" — 32 days old
- [dashpay/tenderdash#1515 fix(service)!: join remaining service workers before shutdown completes](https://github.com/dashpay/tenderdash/pull/1515) — 4 unresolved (4 bot) · 5 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · areas: fallback · Policy: waiting-author
  - Top thread: "🔴 Blocking: Serialize reconnect resubscriptions with subscription changes**" — 5 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/dash-evo-tool#1161 fix(ci): isolate release signing and constrain agent write paths](https://github.com/dashpay/dash-evo-tool/pull/1161) — areas: github, fallback · Policy: waiting-bots
  - Blocker: coderabbitai has not reported for the current head
  - Blocker: thepastaclaw has not reported for the current head
- [dashpay/dash-evo-tool#1228 fix(tokens): show a group proposal to a co-signer whose wallet is locked](https://github.com/dashpay/dash-evo-tool/pull/1228) — 📝 draft · 🐢 targets fix/protocol14-signing-key-selection · ⚠ routing unavailable: no changed-file evidence
- [dashpay/platform#5210 feat(wallet-storage)!: restore complete Core wallet snapshots from SQLite](https://github.com/dashpay/platform/pull/5210) — ⏸ deferred · areas: rs-platform-wallet, wallet-storage, swift-sdk, fallback · Policy: waiting-author
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5220 fix(platform-wallet)!: replay recorded transaction history on load for every persister](https://github.com/dashpay/platform/pull/5220) — ⏸ deferred · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk, kotlin-sdk, fallback
- [dashpay/platform#5519 refactor: remove DPP raw secp256k1 dependency](https://github.com/dashpay/platform/pull/5519) — 🐢 targets v5.2-dev · areas: rs-drive-abci, dpp, rs-platform-wallet, rs-platform-wallet-ffi, rust-sdk, rust-sdk-ffi, js-wasm-sdk, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
- [dashpay/platform#5949 fix(sdk): bump tokio to 1.54.1 and fix the futurelocks it exposes](https://github.com/dashpay/platform/pull/5949) — 🐢 targets v5.2-dev · areas: rs-platform-wallet, rust-sdk, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
- [dashpay/platform#6024 feat(platform-wallet): check a masternode key by its hash without its secret](https://github.com/dashpay/platform/pull/6024) — 📝 draft · 🐢 targets feat/masternode-key-rotation · areas: rs-platform-wallet
- [dashpay/platform#6102 fix(platform-wallet): keep invitation issuance floor across reloads](https://github.com/dashpay/platform/pull/6102) — 📝 draft · 🐢 targets chore/rust-dashcore-dev · ⚠ routing unavailable: no changed-file evidence
- [dashpay/rust-dashcore#1336 fix(dash-spv)!: deliver only verified InstantLocks](https://github.com/dashpay/rust-dashcore/pull/1336) — ✋ changes requested · 📝 draft · areas: dash-spv · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#1416 feat(key-wallet): verify owner authorization of a pending spend](https://github.com/dashpay/rust-dashcore/pull/1416) — ⚠ merge conflict · ✋ changes requested · areas: key-wallet, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed d85f215d8cbdd2418e469628351219534edd341c
- [dashpay/rust-dashcore#1448 fix(dash-spv): keep \`QRInfo\` catch-up across cycles](https://github.com/dashpay/rust-dashcore/pull/1448) — ⚠ merge conflict · 🔴 CI failing · 📝 draft · areas: dash-spv · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#1465 feat(key-wallet): find a provider key by its hash](https://github.com/dashpay/rust-dashcore/pull/1465) — 📝 draft · areas: key-wallet · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#1466 feat(dash-spv): read the masternode list only while it is current](https://github.com/dashpay/rust-dashcore/pull/1466) — 📝 draft · 🐢 targets feat/provider-key-lookup-by-hash · areas: dash-spv
- [dashpay/tenderdash#1593 feat(consensus)!: specify full consensus parameter commitment and remove app version parameter](https://github.com/dashpay/tenderdash/pull/1593) — ⚠ merge conflict · ✋ changes requested · 📝 draft · areas: fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot

<a id="lklimek-needs-action"></a>
#### Needs action (2)
- [dashpay/tenderdash#1515 fix(service)!: join remaining service workers before shutdown completes](https://github.com/dashpay/tenderdash/pull/1515) — 4 unresolved (4 bot) · 5 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · areas: fallback · Policy: waiting-author
  - Top thread: "🔴 Blocking: Serialize reconnect resubscriptions with subscription changes**" — 5 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/rust-dashcore#1416 feat(key-wallet): verify owner authorization of a pending spend](https://github.com/dashpay/rust-dashcore/pull/1416) — ⚠ merge conflict · ✋ changes requested · areas: key-wallet, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed d85f215d8cbdd2418e469628351219534edd341c

<a id="lklimek-ready-for-human"></a>
#### Ready for human (2)
- [dashpay/platform#5519 refactor: remove DPP raw secp256k1 dependency](https://github.com/dashpay/platform/pull/5519) — 🐢 targets v5.2-dev · areas: rs-drive-abci, dpp, rs-platform-wallet, rs-platform-wallet-ffi, rust-sdk, rust-sdk-ffi, js-wasm-sdk, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
- [dashpay/platform#5949 fix(sdk): bump tokio to 1.54.1 and fix the futurelocks it exposes](https://github.com/dashpay/platform/pull/5949) — 🐢 targets v5.2-dev · areas: rs-platform-wallet, rust-sdk, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required

<a id="lklimek-unresolved-comments"></a>
#### Unresolved Comments (1)
- [dashpay/tenderdash#1515 fix(service)!: join remaining service workers before shutdown completes](https://github.com/dashpay/tenderdash/pull/1515) — 4 unresolved (4 bot) · 5 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · areas: fallback · Policy: waiting-author
  - Top thread: "🔴 Blocking: Serialize reconnect resubscriptions with subscription changes**" — 5 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them

<a id="lklimek-changes-requested"></a>
#### Changes Requested (1)
- [dashpay/rust-dashcore#1416 feat(key-wallet): verify owner authorization of a pending spend](https://github.com/dashpay/rust-dashcore/pull/1416) — ⚠ merge conflict · ✋ changes requested · areas: key-wallet, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed d85f215d8cbdd2418e469628351219534edd341c

<a id="lklimek-deferred"></a>
#### Deferred (2)
- [dashpay/platform#5210 feat(wallet-storage)!: restore complete Core wallet snapshots from SQLite](https://github.com/dashpay/platform/pull/5210) — ⏸ deferred · areas: rs-platform-wallet, wallet-storage, swift-sdk, fallback · Policy: waiting-author
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5220 fix(platform-wallet)!: replay recorded transaction history on load for every persister](https://github.com/dashpay/platform/pull/5220) — ⏸ deferred · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk, kotlin-sdk, fallback

<a id="lklimek-draft"></a>
#### Draft (4)
- [dashpay/rust-dashcore#1336 fix(dash-spv)!: deliver only verified InstantLocks](https://github.com/dashpay/rust-dashcore/pull/1336) — ✋ changes requested · 📝 draft · areas: dash-spv · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#1448 fix(dash-spv): keep \`QRInfo\` catch-up across cycles](https://github.com/dashpay/rust-dashcore/pull/1448) — ⚠ merge conflict · 🔴 CI failing · 📝 draft · areas: dash-spv · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#1465 feat(key-wallet): find a provider key by its hash](https://github.com/dashpay/rust-dashcore/pull/1465) — 📝 draft · areas: key-wallet · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/tenderdash#1593 feat(consensus)!: specify full consensus parameter commitment and remove app version parameter](https://github.com/dashpay/tenderdash/pull/1593) — ⚠ merge conflict · ✋ changes requested · 📝 draft · areas: fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot

<a id="lklimek-stale"></a>
#### Stale (7)
- [dashpay/platform#5528 fix(platform-wallet): retain drain operation history through funding conflicts](https://github.com/dashpay/platform/pull/5528) — 5 unresolved (5 bot) · 32 days stale · ⚠ merge conflict · ✋ changes requested · 📝 draft · 🐢 targets feat/sqlite-wallet-restore · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk, kotlin-sdk
  - Top thread: "🟡 Suggestion: Retain diagnostics for malformed FFI operation snapshots**" — 32 days old
- [dashpay/dash-evo-tool#1228 fix(tokens): show a group proposal to a co-signer whose wallet is locked](https://github.com/dashpay/dash-evo-tool/pull/1228) — 📝 draft · 🐢 targets fix/protocol14-signing-key-selection · ⚠ routing unavailable: no changed-file evidence
- [dashpay/platform#5519 refactor: remove DPP raw secp256k1 dependency](https://github.com/dashpay/platform/pull/5519) — 🐢 targets v5.2-dev · areas: rs-drive-abci, dpp, rs-platform-wallet, rs-platform-wallet-ffi, rust-sdk, rust-sdk-ffi, js-wasm-sdk, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
- [dashpay/platform#5949 fix(sdk): bump tokio to 1.54.1 and fix the futurelocks it exposes](https://github.com/dashpay/platform/pull/5949) — 🐢 targets v5.2-dev · areas: rs-platform-wallet, rust-sdk, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
- [dashpay/platform#6024 feat(platform-wallet): check a masternode key by its hash without its secret](https://github.com/dashpay/platform/pull/6024) — 📝 draft · 🐢 targets feat/masternode-key-rotation · areas: rs-platform-wallet
- [dashpay/platform#6102 fix(platform-wallet): keep invitation issuance floor across reloads](https://github.com/dashpay/platform/pull/6102) — 📝 draft · 🐢 targets chore/rust-dashcore-dev · ⚠ routing unavailable: no changed-file evidence
- [dashpay/rust-dashcore#1466 feat(dash-spv): read the masternode list only while it is current](https://github.com/dashpay/rust-dashcore/pull/1466) — 📝 draft · 🐢 targets feat/provider-key-lookup-by-hash · areas: dash-spv

<a id="lklimek-clean"></a>
#### Clean (1)
- [dashpay/dash-evo-tool#1161 fix(ci): isolate release signing and constrain agent write paths](https://github.com/dashpay/dash-evo-tool/pull/1161) — areas: github, fallback · Policy: waiting-bots
  - Blocker: coderabbitai has not reported for the current head
  - Blocker: thepastaclaw has not reported for the current head

<a id="lklimek-ready-for-review"></a>
#### Ready for Review (4)
- [dashpay/dash-evo-tool#1175 docs: add maintainable token tutorial production toolkit](https://github.com/dashpay/dash-evo-tool/pull/1175) — by @Claudius-Maginificent · areas: fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
  - Blocker: Proceeded without thepastaclaw: no review within the configured window
- [dashpay/dash-evo-tool#1223 ci: let PR Hygiene post its verdicts to the dashboard](https://github.com/dashpay/dash-evo-tool/pull/1223) — by @shumkov · areas: github · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 4eadf3941f9fdacf6d0d3308f6d5541c451cc9fa
- [dashpay/platform#6086 feat: check a masternode voting key against Platform when it is loaded](https://github.com/dashpay/platform/pull/6086) — by @QuantumExplorer · areas: swift-sdk, rust-sdk, rust-sdk-ffi · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head
- [dashpay/platform#6091 feat(swift-example-app): browse blocks and check a state transition's status by hash](https://github.com/dashpay/platform/pull/6091) — by @QuantumExplorer · areas: swift-sdk, rust-sdk, rust-sdk-ffi, kotlin-sdk, sdk-parity, fallback · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head

<a id="zocolini"></a>
### @ZocoLini
<a id="zocolini-open"></a>
#### Open (6)
- [dashpay/rust-dashcore#1426 fix(dash-spv): tell an empty filter header store from genesis](https://github.com/dashpay/rust-dashcore/pull/1426) — 3 unresolved (3 CodeRabbit) · 4 days stale · ✋ changes requested · areas: dash-spv · Policy: waiting-author
  - Top thread: "&lt;picture&gt;&lt;img src="https://storage.googleapis.com/coderabbit_public_assets/review-comment-icons-v1/amber-16.svg" width="…" — 4 days old
  - Blocker: coderabbitai requested changes on this head; dismiss the review or push a fix
  - Blocker: coderabbitai left review threads unresolved; resolve them
- [dashpay/rust-dashcore#1199 feat(key-wallet)!: only final coins fund a build](https://github.com/dashpay/rust-dashcore/pull/1199) — ⚠ merge conflict · 📝 draft · areas: dash-spv, key-wallet, key-wallet-manager · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#1328 refactor(key-wallet)!: validate child indexes at construction with \`ChildIndex\`](https://github.com/dashpay/rust-dashcore/pull/1328) — ⚠ merge conflict · 📝 draft · areas: dash-spv, key-wallet, key-wallet-manager · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#1398 fix(dash-spv): detect forks with a full block locator](https://github.com/dashpay/rust-dashcore/pull/1398) — ✋ changes requested · 🔴 CI failing · 📝 draft · areas: dash-spv · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#1421 fix(dash-spv): propagate storage errors from sync paths](https://github.com/dashpay/rust-dashcore/pull/1421) — ⚠ merge conflict · 🔴 CI failing · 📝 draft · areas: dash-spv, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#1459 ci: merge audit and verify jobs into one](https://github.com/dashpay/rust-dashcore/pull/1459) — areas: github · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed e20edef48b4122dad05e5bc08ab8b19f190c0f38
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return

<a id="zocolini-needs-action"></a>
#### Needs action (1)
- [dashpay/rust-dashcore#1426 fix(dash-spv): tell an empty filter header store from genesis](https://github.com/dashpay/rust-dashcore/pull/1426) — 3 unresolved (3 CodeRabbit) · 4 days stale · ✋ changes requested · areas: dash-spv · Policy: waiting-author
  - Top thread: "&lt;picture&gt;&lt;img src="https://storage.googleapis.com/coderabbit_public_assets/review-comment-icons-v1/amber-16.svg" width="…" — 4 days old
  - Blocker: coderabbitai requested changes on this head; dismiss the review or push a fix
  - Blocker: coderabbitai left review threads unresolved; resolve them

<a id="zocolini-unresolved-comments"></a>
#### Unresolved Comments (1)
- [dashpay/rust-dashcore#1426 fix(dash-spv): tell an empty filter header store from genesis](https://github.com/dashpay/rust-dashcore/pull/1426) — 3 unresolved (3 CodeRabbit) · 4 days stale · ✋ changes requested · areas: dash-spv · Policy: waiting-author
  - Top thread: "&lt;picture&gt;&lt;img src="https://storage.googleapis.com/coderabbit_public_assets/review-comment-icons-v1/amber-16.svg" width="…" — 4 days old
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
- [dashpay/rust-dashcore#1421 fix(dash-spv): propagate storage errors from sync paths](https://github.com/dashpay/rust-dashcore/pull/1421) — ⚠ merge conflict · 🔴 CI failing · 📝 draft · areas: dash-spv, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot

<a id="zocolini-clean"></a>
#### Clean (1)
- [dashpay/rust-dashcore#1459 ci: merge audit and verify jobs into one](https://github.com/dashpay/rust-dashcore/pull/1459) — areas: github · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed e20edef48b4122dad05e5bc08ab8b19f190c0f38
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return

<a id="zocolini-ready-for-review"></a>
#### Ready for Review (2)
- [dashpay/rust-dashcore#1482 fix(dash-spv): skip banned peers before sampling candidates](https://github.com/dashpay/rust-dashcore/pull/1482) — by @xdustinface · areas: dash-spv · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed a5aaf4a4212990db26ba8ab745d58a1ac98fbdb5
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/rust-dashcore#1483 fix(dash-spv): keep bans in force across a redial](https://github.com/dashpay/rust-dashcore/pull/1483) — by @xdustinface · areas: dash-spv · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 1f7aca21e9eae5dedde21234a29fce56fb3e0965
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return

<a id="xdustinface"></a>
### @xdustinface
<a id="xdustinface-open"></a>
#### Open (12)
- [dashpay/platform#5791 fix(platform-wallet): refuse a quorum type that does not fit a byte](https://github.com/dashpay/platform/pull/5791) — 2 unresolved (1 CodeRabbit, 1 bot) · 16 days stale · ⚠ merge conflict · 🐢 targets v5.2-dev · areas: rs-platform-wallet · Policy: waiting-author
  - Top thread: "&lt;picture&gt;&lt;img src="https://storage.googleapis.com/coderabbit_public_assets/review-comment-icons-v1/amber-16.svg" width="…" — 16 days old
  - Blocker: coderabbitai left review threads unresolved; resolve them
- [dashpay/platform#6103 ci: set up Manki review](https://github.com/dashpay/platform/pull/6103) — areas: github, fallback · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head
- [dashpay/rust-dashcore#541 ci: integrate Codecov test analytics via \`cargo-nextest\`](https://github.com/dashpay/rust-dashcore/pull/541) — ⚠ merge conflict · 📝 draft · areas: github, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#1372 fix(dash-spv): spread peers across network groups](https://github.com/dashpay/rust-dashcore/pull/1372) — ⚠ merge conflict · 📝 draft · areas: dash-spv · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#1373 fix(dash-spv): rotate the slowest peer of a slow set](https://github.com/dashpay/rust-dashcore/pull/1373) — ⚠ merge conflict · 📝 draft · 🐢 targets fix/peer-diversity · areas: dash-spv
- [dashpay/rust-dashcore#1374 fix(dash-spv): retry a refused request at another peer](https://github.com/dashpay/rust-dashcore/pull/1374) — 📝 draft · 🐢 targets fix/peer-exploration · areas: dash-spv
- [dashpay/rust-dashcore#1375 fix(dash-spv): rank a stalled peer last when routing](https://github.com/dashpay/rust-dashcore/pull/1375) — 📝 draft · 🐢 targets fix/peer-exploration · areas: dash-spv
- [dashpay/rust-dashcore#1406 ci: publish a release when it merges into \`master\`](https://github.com/dashpay/rust-dashcore/pull/1406) — ✋ changes requested · areas: github, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 9cae22d42a3a7bf98a414fe973e64752698788c2
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/rust-dashcore#1473 fix(dash-spv): hedge the filter batch that blocks the store](https://github.com/dashpay/rust-dashcore/pull/1473) — ✋ changes requested · 🔴 CI failing · areas: dash-spv · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 7f3fe33c04b4adaac8c12f03d078925a170b3299
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/rust-dashcore#1474 fix(dash-spv): hedge a request once it waited twice its queue](https://github.com/dashpay/rust-dashcore/pull/1474) — ⚠ merge conflict · 📝 draft · areas: dash-spv · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#1482 fix(dash-spv): skip banned peers before sampling candidates](https://github.com/dashpay/rust-dashcore/pull/1482) — areas: dash-spv · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed a5aaf4a4212990db26ba8ab745d58a1ac98fbdb5
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/rust-dashcore#1483 fix(dash-spv): keep bans in force across a redial](https://github.com/dashpay/rust-dashcore/pull/1483) — areas: dash-spv · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 1f7aca21e9eae5dedde21234a29fce56fb3e0965
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return

<a id="xdustinface-needs-action"></a>
#### Needs action (2)
- [dashpay/rust-dashcore#1406 ci: publish a release when it merges into \`master\`](https://github.com/dashpay/rust-dashcore/pull/1406) — ✋ changes requested · areas: github, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 9cae22d42a3a7bf98a414fe973e64752698788c2
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/rust-dashcore#1473 fix(dash-spv): hedge the filter batch that blocks the store](https://github.com/dashpay/rust-dashcore/pull/1473) — ✋ changes requested · 🔴 CI failing · areas: dash-spv · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 7f3fe33c04b4adaac8c12f03d078925a170b3299
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return

<a id="xdustinface-changes-requested"></a>
#### Changes Requested (2)
- [dashpay/rust-dashcore#1406 ci: publish a release when it merges into \`master\`](https://github.com/dashpay/rust-dashcore/pull/1406) — ✋ changes requested · areas: github, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 9cae22d42a3a7bf98a414fe973e64752698788c2
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/rust-dashcore#1473 fix(dash-spv): hedge the filter batch that blocks the store](https://github.com/dashpay/rust-dashcore/pull/1473) — ✋ changes requested · 🔴 CI failing · areas: dash-spv · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 7f3fe33c04b4adaac8c12f03d078925a170b3299
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return

<a id="xdustinface-draft"></a>
#### Draft (3)
- [dashpay/rust-dashcore#541 ci: integrate Codecov test analytics via \`cargo-nextest\`](https://github.com/dashpay/rust-dashcore/pull/541) — ⚠ merge conflict · 📝 draft · areas: github, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#1372 fix(dash-spv): spread peers across network groups](https://github.com/dashpay/rust-dashcore/pull/1372) — ⚠ merge conflict · 📝 draft · areas: dash-spv · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#1474 fix(dash-spv): hedge a request once it waited twice its queue](https://github.com/dashpay/rust-dashcore/pull/1474) — ⚠ merge conflict · 📝 draft · areas: dash-spv · Policy: draft
  - Blocker: Draft PR does not occupy a review slot

<a id="xdustinface-stale"></a>
#### Stale (4)
- [dashpay/platform#5791 fix(platform-wallet): refuse a quorum type that does not fit a byte](https://github.com/dashpay/platform/pull/5791) — 2 unresolved (1 CodeRabbit, 1 bot) · 16 days stale · ⚠ merge conflict · 🐢 targets v5.2-dev · areas: rs-platform-wallet · Policy: waiting-author
  - Top thread: "&lt;picture&gt;&lt;img src="https://storage.googleapis.com/coderabbit_public_assets/review-comment-icons-v1/amber-16.svg" width="…" — 16 days old
  - Blocker: coderabbitai left review threads unresolved; resolve them
- [dashpay/rust-dashcore#1373 fix(dash-spv): rotate the slowest peer of a slow set](https://github.com/dashpay/rust-dashcore/pull/1373) — ⚠ merge conflict · 📝 draft · 🐢 targets fix/peer-diversity · areas: dash-spv
- [dashpay/rust-dashcore#1374 fix(dash-spv): retry a refused request at another peer](https://github.com/dashpay/rust-dashcore/pull/1374) — 📝 draft · 🐢 targets fix/peer-exploration · areas: dash-spv
- [dashpay/rust-dashcore#1375 fix(dash-spv): rank a stalled peer last when routing](https://github.com/dashpay/rust-dashcore/pull/1375) — 📝 draft · 🐢 targets fix/peer-exploration · areas: dash-spv

<a id="xdustinface-clean"></a>
#### Clean (3)
- [dashpay/platform#6103 ci: set up Manki review](https://github.com/dashpay/platform/pull/6103) — areas: github, fallback · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head
- [dashpay/rust-dashcore#1482 fix(dash-spv): skip banned peers before sampling candidates](https://github.com/dashpay/rust-dashcore/pull/1482) — areas: dash-spv · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed a5aaf4a4212990db26ba8ab745d58a1ac98fbdb5
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/rust-dashcore#1483 fix(dash-spv): keep bans in force across a redial](https://github.com/dashpay/rust-dashcore/pull/1483) — areas: dash-spv · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 1f7aca21e9eae5dedde21234a29fce56fb3e0965
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return

<a id="xdustinface-ready-for-review"></a>
#### Ready for Review (1)
- [dashpay/rust-dashcore#1459 ci: merge audit and verify jobs into one](https://github.com/dashpay/rust-dashcore/pull/1459) — by @ZocoLini · areas: github · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed e20edef48b4122dad05e5bc08ab8b19f190c0f38
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return

<a id="pastapastapasta"></a>
### @PastaPastaPasta
<a id="pastapastapasta-open"></a>
#### Open (26)
- [dashpay/platform#5709 feat(platform-wallet): own the shielded-tip payment decisions in the wallet](https://github.com/dashpay/platform/pull/5709) — 22 unresolved (22 bot) · 21 days stale · ⚠ merge conflict · 📝 draft · 🐢 targets feat/dashpay-shielded-tips · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk, rust-sdk, kotlin-sdk, fallback
  - Top thread: "✅ **Resolved** at \`688c75e1\`; see the replies below." — 21 days old
- [dashpay/platform#4648 feat(drive-abci)!: state sync via ABCI snapshots with reduced platform state (protocol v15)](https://github.com/dashpay/platform/pull/4648) — 8 unresolved (8 bot) · 48 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · 📝 draft · 🐢 targets v5.2-dev · areas: rs-drive, rs-drive-abci, dpp, fallback · Policy: draft
  - Top thread: "🟡 Suggestion: Continue the restored target through consensus and a restart**" — 48 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5934 fix(platform-wallet): own the shielded coordinator and wipe host rows inside Clear](https://github.com/dashpay/platform/pull/5934) — 7 unresolved (7 bot) · 8 days stale · ⚠ merge conflict · 🔴 CI failing · 📝 draft · 🐢 targets v5.2-dev · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk, kotlin-sdk · Policy: draft
  - Top thread: "✅ **Withdrawn** at \`2352c3c6\`; see the replies below." — 8 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5711 feat: shielded-tip screens over the wallet's tip decisions](https://github.com/dashpay/platform/pull/5711) — 4 unresolved (4 bot) · 21 days stale · 📝 draft · 🐢 targets feat/dashpay-shielded-tips-wallet-policy · areas: swift-sdk, kotlin-sdk
  - Top thread: "✅ **Resolved** at \`451dab59\`; see the replies below." — 21 days old
- [dashpay/platform#5786 feat(sdk): tri-state quorum lookups, bounded funding retries and a DPNS contest policy in dash-sdk-cxx](https://github.com/dashpay/platform/pull/5786) — 3 unresolved (3 bot) · 16 days stale · ⚠ merge conflict · 📝 draft · 🐢 targets v5.2-dev · areas: rs-drive, rs-platform-wallet, rust-sdk, js-wasm-sdk, fallback · Policy: draft
  - Top thread: "✅ **Resolved** at \`e46d43eb\`; see the replies below." — 16 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5280 perf(platform-wallet)!: reuse a recent recorded-anchor set on the shielded spend path](https://github.com/dashpay/platform/pull/5280) — 7 unresolved (7 bot) · 2 days stale · ✋ changes requested · 🔴 CI failing · 🐢 targets v5.2-dev · areas: rs-platform-wallet, rust-sdk, rust-sdk-ffi, fallback · Policy: waiting-author
  - Top thread: "✅ **Resolved** at \`327654fc\`; see the replies below." — 2 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5288 feat(platform-wallet): support DashPay shielded tips with dedicated accounts](https://github.com/dashpay/platform/pull/5288) — 7 unresolved (7 bot) · 0 days stale · 🔴 CI failing · 🐢 targets v5.2-dev · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk, rust-sdk, kotlin-sdk, fallback · Policy: waiting-author
  - Top thread: "🔴 Blocking: Make live identity registration respect tip-source placement locking**" — 0 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#4965 test(sdk): cargo-fuzz harness over drive-proof-verifier FromProof](https://github.com/dashpay/platform/pull/4965) — 1 unresolved (1 bot) · 47 days stale · 📝 draft · 🐢 targets v5.2-dev · areas: github, fallback · Policy: draft
  - Top thread: "🟡 Suggestion: Correct the claim that PV13 reaches the V0 proof decoder**" — 47 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/grovedb#1007 build(deps)!: use dashpay/orchard with the faster prover](https://github.com/dashpay/grovedb/pull/1007) — 🔴 CI failing · 📝 draft · areas: fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/grovedb#1054 feat: LigninDB storage engine (tracking, do not merge)](https://github.com/dashpay/grovedb/pull/1054) — 📝 draft · areas: github, fallback · Policy: draft
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
- [dashpay/platform#6042 feat(platform-wallet-ffi): prefetch shielded spend anchors when a send screen opens](https://github.com/dashpay/platform/pull/6042) — 🔴 CI failing · 📝 draft · 🐢 targets perf/shielded-anchor-cache · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, fallback
- [dashpay/rust-dashcore#1456 fix(key-wallet): extend address windows before confirming](https://github.com/dashpay/rust-dashcore/pull/1456) — ✋ changes requested · areas: key-wallet · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
- [dashpay/rust-dashcore#1467 fix(key-wallet): report addresses used within one check as used](https://github.com/dashpay/rust-dashcore/pull/1467) — ✋ changes requested · areas: key-wallet · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return

<a id="pastapastapasta-needs-action"></a>
#### Needs action (2)
- [dashpay/rust-dashcore#1456 fix(key-wallet): extend address windows before confirming](https://github.com/dashpay/rust-dashcore/pull/1456) — ✋ changes requested · areas: key-wallet · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
- [dashpay/rust-dashcore#1467 fix(key-wallet): report addresses used within one check as used](https://github.com/dashpay/rust-dashcore/pull/1467) — ✋ changes requested · areas: key-wallet · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return

<a id="pastapastapasta-ready-for-human"></a>
#### Ready for human (3)
- [dashpay/platform#5793 fix(platform-wallet): make SpvRuntime::header_hash_at public](https://github.com/dashpay/platform/pull/5793) — 🐢 targets v5.2-dev · areas: rs-platform-wallet · Policy: ready-to-merge
  - Blocker: All policy requirements are satisfied
- [dashpay/rust-dashcore#1456 fix(key-wallet): extend address windows before confirming](https://github.com/dashpay/rust-dashcore/pull/1456) — ✋ changes requested · areas: key-wallet · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
- [dashpay/rust-dashcore#1467 fix(key-wallet): report addresses used within one check as used](https://github.com/dashpay/rust-dashcore/pull/1467) — ✋ changes requested · areas: key-wallet · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return

<a id="pastapastapasta-changes-requested"></a>
#### Changes Requested (2)
- [dashpay/rust-dashcore#1456 fix(key-wallet): extend address windows before confirming](https://github.com/dashpay/rust-dashcore/pull/1456) — ✋ changes requested · areas: key-wallet · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
- [dashpay/rust-dashcore#1467 fix(key-wallet): report addresses used within one check as used](https://github.com/dashpay/rust-dashcore/pull/1467) — ✋ changes requested · areas: key-wallet · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return

<a id="pastapastapasta-draft"></a>
#### Draft (2)
- [dashpay/grovedb#1007 build(deps)!: use dashpay/orchard with the faster prover](https://github.com/dashpay/grovedb/pull/1007) — 🔴 CI failing · 📝 draft · areas: fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/grovedb#1054 feat: LigninDB storage engine (tracking, do not merge)](https://github.com/dashpay/grovedb/pull/1054) — 📝 draft · areas: github, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot

<a id="pastapastapasta-stale"></a>
#### Stale (22)
- [dashpay/platform#5709 feat(platform-wallet): own the shielded-tip payment decisions in the wallet](https://github.com/dashpay/platform/pull/5709) — 22 unresolved (22 bot) · 21 days stale · ⚠ merge conflict · 📝 draft · 🐢 targets feat/dashpay-shielded-tips · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk, rust-sdk, kotlin-sdk, fallback
  - Top thread: "✅ **Resolved** at \`688c75e1\`; see the replies below." — 21 days old
- [dashpay/platform#4648 feat(drive-abci)!: state sync via ABCI snapshots with reduced platform state (protocol v15)](https://github.com/dashpay/platform/pull/4648) — 8 unresolved (8 bot) · 48 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · 📝 draft · 🐢 targets v5.2-dev · areas: rs-drive, rs-drive-abci, dpp, fallback · Policy: draft
  - Top thread: "🟡 Suggestion: Continue the restored target through consensus and a restart**" — 48 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5934 fix(platform-wallet): own the shielded coordinator and wipe host rows inside Clear](https://github.com/dashpay/platform/pull/5934) — 7 unresolved (7 bot) · 8 days stale · ⚠ merge conflict · 🔴 CI failing · 📝 draft · 🐢 targets v5.2-dev · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk, kotlin-sdk · Policy: draft
  - Top thread: "✅ **Withdrawn** at \`2352c3c6\`; see the replies below." — 8 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5711 feat: shielded-tip screens over the wallet's tip decisions](https://github.com/dashpay/platform/pull/5711) — 4 unresolved (4 bot) · 21 days stale · 📝 draft · 🐢 targets feat/dashpay-shielded-tips-wallet-policy · areas: swift-sdk, kotlin-sdk
  - Top thread: "✅ **Resolved** at \`451dab59\`; see the replies below." — 21 days old
- [dashpay/platform#5786 feat(sdk): tri-state quorum lookups, bounded funding retries and a DPNS contest policy in dash-sdk-cxx](https://github.com/dashpay/platform/pull/5786) — 3 unresolved (3 bot) · 16 days stale · ⚠ merge conflict · 📝 draft · 🐢 targets v5.2-dev · areas: rs-drive, rs-platform-wallet, rust-sdk, js-wasm-sdk, fallback · Policy: draft
  - Top thread: "✅ **Resolved** at \`e46d43eb\`; see the replies below." — 16 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5280 perf(platform-wallet)!: reuse a recent recorded-anchor set on the shielded spend path](https://github.com/dashpay/platform/pull/5280) — 7 unresolved (7 bot) · 2 days stale · ✋ changes requested · 🔴 CI failing · 🐢 targets v5.2-dev · areas: rs-platform-wallet, rust-sdk, rust-sdk-ffi, fallback · Policy: waiting-author
  - Top thread: "✅ **Resolved** at \`327654fc\`; see the replies below." — 2 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5288 feat(platform-wallet): support DashPay shielded tips with dedicated accounts](https://github.com/dashpay/platform/pull/5288) — 7 unresolved (7 bot) · 0 days stale · 🔴 CI failing · 🐢 targets v5.2-dev · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk, rust-sdk, kotlin-sdk, fallback · Policy: waiting-author
  - Top thread: "🔴 Blocking: Make live identity registration respect tip-source placement locking**" — 0 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#4965 test(sdk): cargo-fuzz harness over drive-proof-verifier FromProof](https://github.com/dashpay/platform/pull/4965) — 1 unresolved (1 bot) · 47 days stale · 📝 draft · 🐢 targets v5.2-dev · areas: github, fallback · Policy: draft
  - Top thread: "🟡 Suggestion: Correct the claim that PV13 reaches the V0 proof decoder**" — 47 days old
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
- [dashpay/platform#6042 feat(platform-wallet-ffi): prefetch shielded spend anchors when a send screen opens](https://github.com/dashpay/platform/pull/6042) — 🔴 CI failing · 📝 draft · 🐢 targets perf/shielded-anchor-cache · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, fallback

<a id="llbartekll"></a>
### @llbartekll
<a id="llbartekll-open"></a>
#### Open (3)
- [dashpay/platform#5997 refactor(platform-wallet): share the masternode update helpers and use rust-dashcore's key and script types](https://github.com/dashpay/platform/pull/5997) — 5 unresolved (5 bot) · 3 days stale · 🐢 targets feat/masternode-key-rotation · areas: rs-platform-wallet
  - Top thread: "✅ **Resolved** at \`cd1020ff\`; see the replies below." — 3 days old
- [dashpay/platform#5889 feat(platform-wallet): rotate a masternode's keys into the wallet](https://github.com/dashpay/platform/pull/5889) — ✋ changes requested · 🔴 CI failing · 🐢 targets v5.2-dev · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk · Policy: waiting-author
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
- [dashpay/platform#5982 feat(platform-wallet)!: apps show the shielded balance the wallet announces, without identity accounts](https://github.com/dashpay/platform/pull/5982) — ⚠ merge conflict · 🔴 CI failing · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk · Policy: waiting-author
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix

<a id="llbartekll-needs-action"></a>
#### Needs action (1)
- [dashpay/platform#5982 feat(platform-wallet)!: apps show the shielded balance the wallet announces, without identity accounts](https://github.com/dashpay/platform/pull/5982) — ⚠ merge conflict · 🔴 CI failing · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk · Policy: waiting-author
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix

<a id="llbartekll-ci-failing"></a>
#### CI Failing (1)
- [dashpay/platform#5982 feat(platform-wallet)!: apps show the shielded balance the wallet announces, without identity accounts](https://github.com/dashpay/platform/pull/5982) — ⚠ merge conflict · 🔴 CI failing · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk · Policy: waiting-author
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix

<a id="llbartekll-stale"></a>
#### Stale (2)
- [dashpay/platform#5997 refactor(platform-wallet): share the masternode update helpers and use rust-dashcore's key and script types](https://github.com/dashpay/platform/pull/5997) — 5 unresolved (5 bot) · 3 days stale · 🐢 targets feat/masternode-key-rotation · areas: rs-platform-wallet
  - Top thread: "✅ **Resolved** at \`cd1020ff\`; see the replies below." — 3 days old
- [dashpay/platform#5889 feat(platform-wallet): rotate a masternode's keys into the wallet](https://github.com/dashpay/platform/pull/5889) — ✋ changes requested · 🔴 CI failing · 🐢 targets v5.2-dev · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk · Policy: waiting-author
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix

<a id="llbartekll-ready-for-review"></a>
#### Ready for Review (3)
- [dashpay/platform#6086 feat: check a masternode voting key against Platform when it is loaded](https://github.com/dashpay/platform/pull/6086) — by @QuantumExplorer · areas: swift-sdk, rust-sdk, rust-sdk-ffi · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head
- [dashpay/platform#6091 feat(swift-example-app): browse blocks and check a state transition's status by hash](https://github.com/dashpay/platform/pull/6091) — by @QuantumExplorer · areas: swift-sdk, rust-sdk, rust-sdk-ffi, kotlin-sdk, sdk-parity, fallback · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head
- [dashpay/platform#6101 fix(swift-example-app): fix the compile error and the three failing tests in the Swift job](https://github.com/dashpay/platform/pull/6101) — by @QuantumExplorer · areas: swift-sdk · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head

<a id="ktechmidas"></a>
### @ktechmidas
<a id="ktechmidas-open"></a>
#### Open (6)
- [dashpay/platform#5188 ci: tolerate optional S3 cache export outages on chore/bump-rust-dashcore-secp-033](https://github.com/dashpay/platform/pull/5188) — 1 unresolved (1 bot) · 55 days stale · 🐢 targets chore/bump-rust-dashcore-secp-033 · areas: github
  - Top thread: "🟡 Suggestion: New regression tests are not executed by any CI workflow on this branch**" — 55 days old
- [dashpay/platform#6018 ci: move CI Docker images from ECR to GHCR (saves about $265 a month)](https://github.com/dashpay/platform/pull/6018) — 4 unresolved (4 bot) · 0 days stale · ✋ changes requested · 🔴 CI failing · 🐢 targets v5.2-dev · areas: github, fallback · Policy: waiting-author
  - Top thread: "🔴 Blocking: Give the PR cleanup dry run a read-only package token**" — 0 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
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
- [dashpay/platform#5188 ci: tolerate optional S3 cache export outages on chore/bump-rust-dashcore-secp-033](https://github.com/dashpay/platform/pull/5188) — 1 unresolved (1 bot) · 55 days stale · 🐢 targets chore/bump-rust-dashcore-secp-033 · areas: github
  - Top thread: "🟡 Suggestion: New regression tests are not executed by any CI workflow on this branch**" — 55 days old
- [dashpay/platform#6018 ci: move CI Docker images from ECR to GHCR (saves about $265 a month)](https://github.com/dashpay/platform/pull/6018) — 4 unresolved (4 bot) · 0 days stale · ✋ changes requested · 🔴 CI failing · 🐢 targets v5.2-dev · areas: github, fallback · Policy: waiting-author
  - Top thread: "🔴 Blocking: Give the PR cleanup dry run a read-only package token**" — 0 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5186 ci: tolerate optional S3 cache export outages on v5.0-dev](https://github.com/dashpay/platform/pull/5186) — 🔴 CI failing · 🐢 targets v6.0-dev · areas: github · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 843052c2dce26a80065c15b1219bbc8211264161
- [dashpay/platform#5198 ci: bound default Binaryen optimizer threads in NPM builds](https://github.com/dashpay/platform/pull/5198) — 🐢 targets v6.0-dev · areas: github · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 23b41b477bc038965b686b4a75412b1c21fc6d98
- [dashpay/platform#5205 fix(release): install verified cargo-binstall binary directly](https://github.com/dashpay/platform/pull/5205) — 🐢 targets v6.0-dev · areas: fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 8a8ac2c788b5338416e18045001d9115ba3ac80a
- [dashpay/platform#5866 feat(dashmate): clean up historical Docker images after updates](https://github.com/dashpay/platform/pull/5866) — ✋ changes requested · 🔴 CI failing · 🐢 targets v5.2-dev · areas: dashmate · Policy: waiting-author
  - Blocker: Author response is required after the latest human objection

<a id="ktechmidas-ready-for-review"></a>
#### Ready for Review (6)
- [dashpay/dash-evo-tool#1161 fix(ci): isolate release signing and constrain agent write paths](https://github.com/dashpay/dash-evo-tool/pull/1161) — by @lklimek · areas: github, fallback · Policy: waiting-bots
  - Blocker: coderabbitai has not reported for the current head
  - Blocker: thepastaclaw has not reported for the current head
- [dashpay/dash-evo-tool#1223 ci: let PR Hygiene post its verdicts to the dashboard](https://github.com/dashpay/dash-evo-tool/pull/1223) — by @shumkov · areas: github · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 4eadf3941f9fdacf6d0d3308f6d5541c451cc9fa
- [dashpay/platform#6002 fix(dashmate): run an added local node on its network's Core image](https://github.com/dashpay/platform/pull/6002) — by @QuantumExplorer · areas: dashmate · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 9cc7778487200a5f379f0cb47cfff854e1a520dd
- [dashpay/platform#6039 ci: let PR Hygiene post its verdicts to the dashboard](https://github.com/dashpay/platform/pull/6039) — by @shumkov · areas: github · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed b3c26087e7c379feb37d33b49714c1ff192d6aae
- [dashpay/platform#6103 ci: set up Manki review](https://github.com/dashpay/platform/pull/6103) — by @xdustinface · areas: github, fallback · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head
- [dashpay/rust-dashcore#1459 ci: merge audit and verify jobs into one](https://github.com/dashpay/rust-dashcore/pull/1459) — by @ZocoLini · areas: github · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed e20edef48b4122dad05e5bc08ab8b19f190c0f38
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return

<a id="romchornyi"></a>
### @romchornyi
<a id="romchornyi-open"></a>
#### Open (15)
- [dashpay/platform#5928 feat(platform-wallet): report whether a failed payment may have left the wallet](https://github.com/dashpay/platform/pull/5928) — 16 unresolved (16 bot) · 4 days stale · 🔴 CI failing · 🐢 targets v5.2-dev · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk · Policy: waiting-author
  - Top thread: "⏭️ **Deferred** (maintainer decision; not blocking) at \`19c00501\`; see the replies below." — 4 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5853 feat: settle a shielded identity create whose inputs are spent without its identity](https://github.com/dashpay/platform/pull/5853) — 11 unresolved (11 bot) · 5 days stale · ⚠ merge conflict · 🔴 CI failing · 🐢 targets v5.2-dev · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, rust-sdk, kotlin-sdk, fallback · Policy: waiting-author
  - Top thread: "✅ **Withdrawn** at \`c4a79650\`; see the replies below." — 5 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5930 feat(platform-wallet): record the payment intent of external asset locks](https://github.com/dashpay/platform/pull/5930) — 9 unresolved (9 bot) · 7 days stale · ⚠ merge conflict · 🔴 CI failing · 🐢 targets v5.2-dev · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk, kotlin-sdk · Policy: waiting-author
  - Top thread: "✅ **Resolved** at \`c41628d3\`; see the replies below." — 7 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5865 feat(platform-wallet): plan a shielded spend with its exact fee and spend against the plan](https://github.com/dashpay/platform/pull/5865) — 11 unresolved (11 bot) · 3 days stale · 🔴 CI failing · 🐢 targets v5.2-dev · areas: dpp, rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, rust-sdk, kotlin-sdk, sdk-parity, fallback · Policy: waiting-author
  - Top thread: "✅ **Resolved** at \`564797d1\`; see the replies below." — 3 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5536 fix: check a DashPay invitation claim against Platform before broadcasting it](https://github.com/dashpay/platform/pull/5536) — 6 unresolved (6 bot) · 10 days stale · ⚠ merge conflict · 🔴 CI failing · 🐢 targets v5.2-dev · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, rust-sdk, kotlin-sdk, sdk-parity · Policy: waiting-author
  - Top thread: "⏭️ **Deferred** (maintainer decision; not blocking) at \`f312d906\`; see the replies below." — 10 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5258 feat(platform-wallet): probe the network about sends with an unknown broadcast outcome](https://github.com/dashpay/platform/pull/5258) — 3 unresolved (3 human) · 38 days stale · ⚠ merge conflict · 📝 draft · 🐢 targets v5.2-dev · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, rust-sdk · Policy: draft
  - Top thread: "\[P2\] Carry the originating wallet generation in probe events" — 38 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5540 fix(platform-wallet): store externally supplied identity keys without a derivation breadcrumb](https://github.com/dashpay/platform/pull/5540) — 3 unresolved (3 bot) · 31 days stale · ⚠ merge conflict · 🔴 CI failing · 🐢 targets v5.2-dev · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk · Policy: waiting-author
  - Top thread: "✅ **Resolved** at \`b25b2543\`; see the replies below." — 31 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5549 fix(platform-wallet-storage): restore DashPay contact address pools on SQLite load](https://github.com/dashpay/platform/pull/5549) — 3 unresolved (3 bot) · 30 days stale · ⚠ merge conflict · 🔴 CI failing · 📝 draft · 🐢 targets v5.2-dev · areas: wallet-storage · Policy: draft
  - Top thread: "✅ **Withdrawn** at \`2bee2705\`; see the replies below." — 30 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5395 feat(platform-wallet): spend only final coins and report funds waiting on the network](https://github.com/dashpay/platform/pull/5395) — 1 unresolved (1 human) · 38 days stale · ⚠ merge conflict · ✋ changes requested · 📝 draft · 🐢 targets v5.2-dev · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk · Policy: draft
  - Top thread: "this should belong to another layer - likely platform-wallet, or even rust-dashcore's key-wallet. FFI is not meant to st…" — 38 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5929 feat(platform-wallet): one address top-up for any identity](https://github.com/dashpay/platform/pull/5929) — 4 unresolved (4 bot) · 7 days stale · 🐢 targets v5.2-dev · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk, sdk-parity · Policy: waiting-self-review
  - Top thread: "🟡 Suggestion: Make the liveness regression test remove the wallet after the reads**" — 7 days old
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
- [dashpay/platform#5928 feat(platform-wallet): report whether a failed payment may have left the wallet](https://github.com/dashpay/platform/pull/5928) — 16 unresolved (16 bot) · 4 days stale · 🔴 CI failing · 🐢 targets v5.2-dev · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk · Policy: waiting-author
  - Top thread: "⏭️ **Deferred** (maintainer decision; not blocking) at \`19c00501\`; see the replies below." — 4 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5853 feat: settle a shielded identity create whose inputs are spent without its identity](https://github.com/dashpay/platform/pull/5853) — 11 unresolved (11 bot) · 5 days stale · ⚠ merge conflict · 🔴 CI failing · 🐢 targets v5.2-dev · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, rust-sdk, kotlin-sdk, fallback · Policy: waiting-author
  - Top thread: "✅ **Withdrawn** at \`c4a79650\`; see the replies below." — 5 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5930 feat(platform-wallet): record the payment intent of external asset locks](https://github.com/dashpay/platform/pull/5930) — 9 unresolved (9 bot) · 7 days stale · ⚠ merge conflict · 🔴 CI failing · 🐢 targets v5.2-dev · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk, kotlin-sdk · Policy: waiting-author
  - Top thread: "✅ **Resolved** at \`c41628d3\`; see the replies below." — 7 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5865 feat(platform-wallet): plan a shielded spend with its exact fee and spend against the plan](https://github.com/dashpay/platform/pull/5865) — 11 unresolved (11 bot) · 3 days stale · 🔴 CI failing · 🐢 targets v5.2-dev · areas: dpp, rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, rust-sdk, kotlin-sdk, sdk-parity, fallback · Policy: waiting-author
  - Top thread: "✅ **Resolved** at \`564797d1\`; see the replies below." — 3 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5536 fix: check a DashPay invitation claim against Platform before broadcasting it](https://github.com/dashpay/platform/pull/5536) — 6 unresolved (6 bot) · 10 days stale · ⚠ merge conflict · 🔴 CI failing · 🐢 targets v5.2-dev · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, rust-sdk, kotlin-sdk, sdk-parity · Policy: waiting-author
  - Top thread: "⏭️ **Deferred** (maintainer decision; not blocking) at \`f312d906\`; see the replies below." — 10 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5258 feat(platform-wallet): probe the network about sends with an unknown broadcast outcome](https://github.com/dashpay/platform/pull/5258) — 3 unresolved (3 human) · 38 days stale · ⚠ merge conflict · 📝 draft · 🐢 targets v5.2-dev · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, rust-sdk · Policy: draft
  - Top thread: "\[P2\] Carry the originating wallet generation in probe events" — 38 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5540 fix(platform-wallet): store externally supplied identity keys without a derivation breadcrumb](https://github.com/dashpay/platform/pull/5540) — 3 unresolved (3 bot) · 31 days stale · ⚠ merge conflict · 🔴 CI failing · 🐢 targets v5.2-dev · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk · Policy: waiting-author
  - Top thread: "✅ **Resolved** at \`b25b2543\`; see the replies below." — 31 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5549 fix(platform-wallet-storage): restore DashPay contact address pools on SQLite load](https://github.com/dashpay/platform/pull/5549) — 3 unresolved (3 bot) · 30 days stale · ⚠ merge conflict · 🔴 CI failing · 📝 draft · 🐢 targets v5.2-dev · areas: wallet-storage · Policy: draft
  - Top thread: "✅ **Withdrawn** at \`2bee2705\`; see the replies below." — 30 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5395 feat(platform-wallet): spend only final coins and report funds waiting on the network](https://github.com/dashpay/platform/pull/5395) — 1 unresolved (1 human) · 38 days stale · ⚠ merge conflict · ✋ changes requested · 📝 draft · 🐢 targets v5.2-dev · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk · Policy: draft
  - Top thread: "this should belong to another layer - likely platform-wallet, or even rust-dashcore's key-wallet. FFI is not meant to st…" — 38 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5929 feat(platform-wallet): one address top-up for any identity](https://github.com/dashpay/platform/pull/5929) — 4 unresolved (4 bot) · 7 days stale · 🐢 targets v5.2-dev · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk, sdk-parity · Policy: waiting-self-review
  - Top thread: "🟡 Suggestion: Make the liveness regression test remove the wallet after the reads**" — 7 days old
  - Blocker: Author must post /self-reviewed 661ce185556b61208615cc646c88d145c487176e
- [dashpay/platform#4992 feat(swift-sdk): durable wallet presence marker readable while the device is locked](https://github.com/dashpay/platform/pull/4992) — 🐢 targets v5.2-dev · areas: swift-sdk · Policy: ready-to-merge
  - Blocker: All policy requirements are satisfied
- [dashpay/platform#5587 fix(platform-wallet)!: keep a contact's payment account across relaunches and replace it cleanly on rotation](https://github.com/dashpay/platform/pull/5587) — ⚠ merge conflict · 📝 draft · 🐢 targets v5.2-dev · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5589 feat(platform-wallet-ffi)!: let hosts reset an outbound contact account's pool and keep its request digest](https://github.com/dashpay/platform/pull/5589) — 🔴 CI failing · 📝 draft · 🐢 targets fix/contact-rebuild-keeps-pool · areas: rs-platform-wallet-ffi, swift-sdk
- [dashpay/platform#5590 feat(kotlin-sdk)!: reset an outbound contact account's pool and keep its request digest on Android](https://github.com/dashpay/platform/pull/5590) — 🔴 CI failing · 📝 draft · 🐢 targets fix/contact-rebuild-ffi-swift · areas: kotlin-sdk, sdk-parity
- [dashpay/platform#6014 feat(platform-wallet)!: give identity slots one wallet-owned lease](https://github.com/dashpay/platform/pull/6014) — 🔴 CI failing · 📝 draft · 🐢 targets feat/invitation-claim-preflight · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, rust-sdk, kotlin-sdk, sdk-parity

<a id="romchornyi-ready-for-review"></a>
#### Ready for Review (3)
- [dashpay/platform#6086 feat: check a masternode voting key against Platform when it is loaded](https://github.com/dashpay/platform/pull/6086) — by @QuantumExplorer · areas: swift-sdk, rust-sdk, rust-sdk-ffi · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head
- [dashpay/platform#6091 feat(swift-example-app): browse blocks and check a state transition's status by hash](https://github.com/dashpay/platform/pull/6091) — by @QuantumExplorer · areas: swift-sdk, rust-sdk, rust-sdk-ffi, kotlin-sdk, sdk-parity, fallback · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head
- [dashpay/platform#6101 fix(swift-example-app): fix the compile error and the three failing tests in the Swift job](https://github.com/dashpay/platform/pull/6101) — by @QuantumExplorer · areas: swift-sdk · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head

<a id="hashengineering"></a>
### @HashEngineering
<a id="hashengineering-open"></a>
#### Open (6)
- [dashpay/platform#5931 feat(platform-wallet): top up someone else's identity from the Core balance](https://github.com/dashpay/platform/pull/5931) — 13 unresolved (13 bot) · 9 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · 🐢 targets v5.2-dev · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Top-up result can report a stale cached balance**" — 9 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5767 fix(kotlin-sdk): port #4638 to Android on the wallet's TXO reconcile policy](https://github.com/dashpay/platform/pull/5767) — 3 unresolved (3 bot) · 12 days stale · 🐢 targets v5.2-dev · areas: swift-sdk, kotlin-sdk, github, fallback · Policy: waiting-author
  - Top thread: "⏭️ **Deferred** (maintainer decision; not blocking; tracked in https://github.com/dashpay/platform/issues/5977) at \`2efb…" — 12 days old
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5933 feat(kotlin-sdk): top up someone else's identity from the Core balance](https://github.com/dashpay/platform/pull/5933) — 2 unresolved (1 CodeRabbit, 1 bot) · 8 days stale · ✋ changes requested · 📝 draft · 🐢 targets HashEngineering/platform-wallet-top-up-someone-elses-identity-fr · areas: kotlin-sdk, sdk-parity
  - Top thread: "&lt;picture&gt;&lt;img src="https://storage.googleapis.com/coderabbit_public_assets/review-comment-icons-v1/amber-16.svg" width="…" — 8 days old
- [dashpay/platform#5906 feat(kotlin-sdk): topUpExternalIdentityFromAddresses](https://github.com/dashpay/platform/pull/5906) — 1 unresolved (1 bot) · 3 days stale · ✋ changes requested · 🔴 CI failing · 🐢 targets v5.2-dev · areas: kotlin-sdk, sdk-parity · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Tell callers when they must record a completed top-up**" — 3 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
- [dashpay/platform#5896 feat(kotlin-sdk): shield from identity to a third-party Orchard address](https://github.com/dashpay/platform/pull/5896) — 1 unresolved (1 bot) · 3 days stale · 🐢 targets v5.2-dev · areas: kotlin-sdk, sdk-parity · Policy: ready-to-merge
  - Top thread: "⏭️ **Deferred** (maintainer decision; not blocking) at \`defca9c3\`; see the replies below." — 3 days old
  - Blocker: All policy requirements are satisfied
- [dashpay/platform#5479 feat(platform-wallet-storage): persist DashPay backfill coverage and outbound-account markers](https://github.com/dashpay/platform/pull/5479) — ⚠ merge conflict · 🐢 targets v5.2-dev · areas: rs-platform-wallet, wallet-storage · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 8e1ee2e24549cabf2d2e81a85a02fc613212aa1f

<a id="hashengineering-ready-for-human"></a>
#### Ready for human (1)
- [dashpay/platform#5896 feat(kotlin-sdk): shield from identity to a third-party Orchard address](https://github.com/dashpay/platform/pull/5896) — 1 unresolved (1 bot) · 3 days stale · 🐢 targets v5.2-dev · areas: kotlin-sdk, sdk-parity · Policy: ready-to-merge
  - Top thread: "⏭️ **Deferred** (maintainer decision; not blocking) at \`defca9c3\`; see the replies below." — 3 days old
  - Blocker: All policy requirements are satisfied

<a id="hashengineering-stale"></a>
#### Stale (6)
- [dashpay/platform#5931 feat(platform-wallet): top up someone else's identity from the Core balance](https://github.com/dashpay/platform/pull/5931) — 13 unresolved (13 bot) · 9 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · 🐢 targets v5.2-dev · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Top-up result can report a stale cached balance**" — 9 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5767 fix(kotlin-sdk): port #4638 to Android on the wallet's TXO reconcile policy](https://github.com/dashpay/platform/pull/5767) — 3 unresolved (3 bot) · 12 days stale · 🐢 targets v5.2-dev · areas: swift-sdk, kotlin-sdk, github, fallback · Policy: waiting-author
  - Top thread: "⏭️ **Deferred** (maintainer decision; not blocking; tracked in https://github.com/dashpay/platform/issues/5977) at \`2efb…" — 12 days old
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5933 feat(kotlin-sdk): top up someone else's identity from the Core balance](https://github.com/dashpay/platform/pull/5933) — 2 unresolved (1 CodeRabbit, 1 bot) · 8 days stale · ✋ changes requested · 📝 draft · 🐢 targets HashEngineering/platform-wallet-top-up-someone-elses-identity-fr · areas: kotlin-sdk, sdk-parity
  - Top thread: "&lt;picture&gt;&lt;img src="https://storage.googleapis.com/coderabbit_public_assets/review-comment-icons-v1/amber-16.svg" width="…" — 8 days old
- [dashpay/platform#5906 feat(kotlin-sdk): topUpExternalIdentityFromAddresses](https://github.com/dashpay/platform/pull/5906) — 1 unresolved (1 bot) · 3 days stale · ✋ changes requested · 🔴 CI failing · 🐢 targets v5.2-dev · areas: kotlin-sdk, sdk-parity · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Tell callers when they must record a completed top-up**" — 3 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
- [dashpay/platform#5896 feat(kotlin-sdk): shield from identity to a third-party Orchard address](https://github.com/dashpay/platform/pull/5896) — 1 unresolved (1 bot) · 3 days stale · 🐢 targets v5.2-dev · areas: kotlin-sdk, sdk-parity · Policy: ready-to-merge
  - Top thread: "⏭️ **Deferred** (maintainer decision; not blocking) at \`defca9c3\`; see the replies below." — 3 days old
  - Blocker: All policy requirements are satisfied
- [dashpay/platform#5479 feat(platform-wallet-storage): persist DashPay backfill coverage and outbound-account markers](https://github.com/dashpay/platform/pull/5479) — ⚠ merge conflict · 🐢 targets v5.2-dev · areas: rs-platform-wallet, wallet-storage · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 8e1ee2e24549cabf2d2e81a85a02fc613212aa1f

<a id="hashengineering-ready-for-review"></a>
#### Ready for Review (1)
- [dashpay/platform#6091 feat(swift-example-app): browse blocks and check a state transition's status by hash](https://github.com/dashpay/platform/pull/6091) — by @QuantumExplorer · areas: swift-sdk, rust-sdk, rust-sdk-ffi, kotlin-sdk, sdk-parity, fallback · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head

<a id="bfoss765"></a>
### @bfoss765
<a id="bfoss765-open"></a>
#### Open (2)
- [dashpay/platform#4313 feat(kotlin-sdk): one-time Orchard key shielded-invite API (inviter + claim)](https://github.com/dashpay/platform/pull/4313) — 6 unresolved (6 bot) · 83 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · 🐢 targets v5.2-dev · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk · Policy: waiting-author
  - Top thread: "✅ **Resolved** at \`eccc65fa\`; see the replies below." — 83 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#4312 feat(platform-wallet): multi-output shielded transfers + output-aware fee predictor](https://github.com/dashpay/platform/pull/4312) — 1 unresolved (1 bot) · 83 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · 🐢 targets v5.2-dev · areas: dpp, rs-platform-wallet, rs-platform-wallet-ffi, kotlin-sdk, fallback · Policy: waiting-self-review
  - Top thread: "🟡 Suggestion: Keep envelope-aware action counting behind the crate boundary**" — 83 days old
  - Blocker: Author must post /self-reviewed da932f96a8670c641065954de8b58752e99233a3

<a id="bfoss765-stale"></a>
#### Stale (2)
- [dashpay/platform#4313 feat(kotlin-sdk): one-time Orchard key shielded-invite API (inviter + claim)](https://github.com/dashpay/platform/pull/4313) — 6 unresolved (6 bot) · 83 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · 🐢 targets v5.2-dev · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk · Policy: waiting-author
  - Top thread: "✅ **Resolved** at \`eccc65fa\`; see the replies below." — 83 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#4312 feat(platform-wallet): multi-output shielded transfers + output-aware fee predictor](https://github.com/dashpay/platform/pull/4312) — 1 unresolved (1 bot) · 83 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · 🐢 targets v5.2-dev · areas: dpp, rs-platform-wallet, rs-platform-wallet-ffi, kotlin-sdk, fallback · Policy: waiting-self-review
  - Top thread: "🟡 Suggestion: Keep envelope-aware action counting behind the crate boundary**" — 83 days old
  - Blocker: Author must post /self-reviewed da932f96a8670c641065954de8b58752e99233a3

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
