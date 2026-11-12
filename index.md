---
---
# PR Hygiene Report
*Last updated: 2026-11-12 12:12 UTC · commit b100b61*

## Summary
- Open PRs: **183** (29 clean · 9 CI failing · 12 changes requested · 32 unresolved comments · 7 deferred · 44 draft · 50 stale)
- PRs needing author action: **45**
- Total unresolved comments: **204**
- dashpay/platform: **116** open (17 clean · 5 CI failing · 6 changes requested · 26 unresolved comments · 7 deferred · 22 draft · 33 stale) · engine: 23 draft · 4 ready-for-human · 2 ready-to-merge · 4 too-many-open-prs · 23 waiting-author · 7 waiting-bots · 3 waiting-build · 18 waiting-self-review · 32 no verdict
- dashpay/rust-dashcore: **24** open (7 clean · 1 CI failing · 3 changes requested · 2 unresolved comments · 0 deferred · 4 draft · 7 stale) · engine: 4 draft · 2 ready-for-human · 2 waiting-author · 1 waiting-bots · 1 waiting-build · 7 waiting-self-review · 7 no verdict
- dashpay/tenderdash: **14** open (2 clean · 0 CI failing · 2 changes requested · 3 unresolved comments · 0 deferred · 7 draft · 0 stale) · engine: 7 draft · 1 ready-for-human · 3 waiting-author · 3 waiting-self-review
- dashpay/grovedb: **1** open (0 clean · 0 CI failing · 0 changes requested · 0 unresolved comments · 0 deferred · 1 draft · 0 stale) · engine: 1 draft
- dashpay/dash-evo-tool: **28** open (3 clean · 3 CI failing · 1 changes requested · 1 unresolved comments · 0 deferred · 10 draft · 10 stale) · engine: 10 draft · 7 ready-for-human · 1 waiting-author · 10 no verdict

## Scoreboard
_Sort: unresolved-comments desc → needs-action desc → ready-for-review desc. Click any number to jump to the specific PRs it covers._

| Author | Open | Clean | CI failing | Unresolved Comments | Changes Requested | Deferred | Draft | Stale | Needs action | Ready for human | Total Unresolved Comments | Ready for Review | Δ |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| [@romchornyi](#romchornyi) | [16](#romchornyi-open) | [2](#romchornyi-clean) | — | [8](#romchornyi-unresolved-comments) | — | — | [4](#romchornyi-draft) | [2](#romchornyi-stale) | [5](#romchornyi-needs-action) | [2](#romchornyi-ready-for-human) | [34](#romchornyi-unresolved-comments) | [4](#romchornyi-ready-for-review) | ↑ 2 |
| [@Claudius-Maginificent](#claudius-maginificent) | [55](#claudius-maginificent-open) | [4](#claudius-maginificent-clean) | [4](#claudius-maginificent-ci-failing) | [6](#claudius-maginificent-unresolved-comments) | [2](#claudius-maginificent-changes-requested) | [3](#claudius-maginificent-deferred) | [17](#claudius-maginificent-draft) | [19](#claudius-maginificent-stale) | [9](#claudius-maginificent-needs-action) | [9](#claudius-maginificent-ready-for-human) | [27](#claudius-maginificent-unresolved-comments) | — | ↑ 1 |
| [@QuantumExplorer](#quantumexplorer) | [24](#quantumexplorer-open) | [2](#quantumexplorer-clean) | [1](#quantumexplorer-ci-failing) | [6](#quantumexplorer-unresolved-comments) | [2](#quantumexplorer-changes-requested) | — | [5](#quantumexplorer-draft) | [8](#quantumexplorer-stale) | [8](#quantumexplorer-needs-action) | — | [46](#quantumexplorer-unresolved-comments) | [11](#quantumexplorer-ready-for-review) | ↑ 1 |
| [@PastaPastaPasta](#pastapastapasta) | [26](#pastapastapasta-open) | [8](#pastapastapasta-clean) | — | [3](#pastapastapasta-unresolved-comments) | [2](#pastapastapasta-changes-requested) | — | [11](#pastapastapasta-draft) | [2](#pastapastapasta-stale) | [6](#pastapastapasta-needs-action) | [2](#pastapastapasta-ready-for-human) | [45](#pastapastapasta-unresolved-comments) | [1](#pastapastapasta-ready-for-review) | ↑ 2 |
| [@shumkov](#shumkov) | [12](#shumkov-open) | [1](#shumkov-clean) | [1](#shumkov-ci-failing) | [3](#shumkov-unresolved-comments) | [1](#shumkov-changes-requested) | — | [1](#shumkov-draft) | [5](#shumkov-stale) | [5](#shumkov-needs-action) | — | [26](#shumkov-unresolved-comments) | [8](#shumkov-ready-for-review) | — |
| [@bfoss765](#bfoss765) | [2](#bfoss765-open) | — | — | [2](#bfoss765-unresolved-comments) | — | — | — | — | [2](#bfoss765-needs-action) | — | [10](#bfoss765-unresolved-comments) | — | — |
| [@lklimek](#lklimek) | [11](#lklimek-open) | — | — | [1](#lklimek-unresolved-comments) | [3](#lklimek-changes-requested) | [4](#lklimek-deferred) | [2](#lklimek-draft) | [1](#lklimek-stale) | [4](#lklimek-needs-action) | — | [8](#lklimek-unresolved-comments) | [9](#lklimek-ready-for-review) | ↓ 1 |
| [@xdustinface](#xdustinface) | [11](#xdustinface-open) | — | — | [1](#xdustinface-unresolved-comments) | [1](#xdustinface-changes-requested) | — | [2](#xdustinface-draft) | [7](#xdustinface-stale) | [2](#xdustinface-needs-action) | — | [2](#xdustinface-unresolved-comments) | [6](#xdustinface-ready-for-review) | ↑ 1 |
| [@HashEngineering](#hashengineering) | [4](#hashengineering-open) | [3](#hashengineering-clean) | — | [1](#hashengineering-unresolved-comments) | — | — | — | — | [1](#hashengineering-needs-action) | — | [3](#hashengineering-unresolved-comments) | [5](#hashengineering-ready-for-review) | — |
| [@thepastaclaw](#thepastaclaw) | [2](#thepastaclaw-open) | [1](#thepastaclaw-clean) | — | [1](#thepastaclaw-unresolved-comments) | — | — | — | — | — | [1](#thepastaclaw-ready-for-human) | [2](#thepastaclaw-unresolved-comments) | — | — |
| [@ZocoLini](#zocolini) | [7](#zocolini-open) | [3](#zocolini-clean) | [1](#zocolini-ci-failing) | — | [1](#zocolini-changes-requested) | — | [2](#zocolini-draft) | — | [2](#zocolini-needs-action) | — | — | [10](#zocolini-ready-for-review) | ↓ 1 |
| [@ktechmidas](#ktechmidas) | [7](#ktechmidas-open) | [2](#ktechmidas-clean) | [1](#ktechmidas-ci-failing) | — | — | — | — | [4](#ktechmidas-stale) | [1](#ktechmidas-needs-action) | — | [1](#ktechmidas-unresolved-comments) | [2](#ktechmidas-ready-for-review) | — |
| [@llbartekll](#llbartekll) | [3](#llbartekll-open) | [3](#llbartekll-clean) | — | — | — | — | — | — | — | — | — | [3](#llbartekll-ready-for-review) | ↓ 3 |
| [@DCG-Claude](#dcg-claude) | [3](#dcg-claude-open) | — | [1](#dcg-claude-ci-failing) | — | — | — | — | [2](#dcg-claude-stale) | — | [2](#dcg-claude-ready-for-human) | — | — | — |

## Per-author detail

<a id="romchornyi"></a>
### @romchornyi
<a id="romchornyi-open"></a>
#### Open (16)
- [dashpay/platform#5258 feat(platform-wallet): probe the network about sends with an unknown broadcast outcome](https://github.com/dashpay/platform/pull/5258) — 3 unresolved (3 human) · 27 days stale · ⚠ merge conflict · 📝 draft · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, rust-sdk · Policy: draft
  - Top thread: "\[P2\] Carry the originating wallet generation in probe events" — 27 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5540 fix(platform-wallet): store externally supplied identity keys without a derivation breadcrumb](https://github.com/dashpay/platform/pull/5540) — 3 unresolved (3 bot) · 21 days stale · 🔴 CI failing · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk, fallback · Policy: waiting-author
  - Top thread: "✅ **Resolved** at \`b25b2543\`; see the replies below." — 21 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5549 fix(platform-wallet-storage): restore DashPay contact address pools on SQLite load](https://github.com/dashpay/platform/pull/5549) — 3 unresolved (3 bot) · 20 days stale · ⚠ merge conflict · 🔴 CI failing · 📝 draft · areas: wallet-storage · Policy: draft
  - Top thread: "✅ **Withdrawn** at \`2bee2705\`; see the replies below." — 20 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5395 feat(platform-wallet): spend only final coins and report funds waiting on the network](https://github.com/dashpay/platform/pull/5395) — 1 unresolved (1 human) · 27 days stale · ⚠ merge conflict · ✋ changes requested · 📝 draft · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk, fallback · Policy: draft
  - Top thread: "this should belong to another layer - likely platform-wallet, or even rust-dashcore's key-wallet. FFI is not meant to st…" — 27 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5853 feat: settle a shielded identity create whose inputs are spent without its identity](https://github.com/dashpay/platform/pull/5853) — 7 unresolved (7 bot) · 1 days stale · 🔴 CI failing · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, rust-sdk, kotlin-sdk, fallback · Policy: waiting-author
  - Top thread: "✅ **Resolved** at \`c9979afd\`; see the replies below." — 1 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5865 feat(platform-wallet): plan a shielded spend with its exact fee and spend against the plan](https://github.com/dashpay/platform/pull/5865) — 7 unresolved (7 bot) · 1 days stale · 🔴 CI failing · areas: dpp, rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk, fallback · Policy: waiting-author
  - Top thread: "✅ **Resolved** at \`99877748\`; see the replies below." — 1 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5863 feat(swift-sdk): shieldedShieldFromIdentityToRecipient](https://github.com/dashpay/platform/pull/5863) — 4 unresolved (4 bot) · 1 days stale · areas: rs-platform-wallet-ffi, swift-sdk, kotlin-sdk, fallback · Policy: waiting-author
  - Top thread: "✅ **Resolved** at \`31eb8df2\`; see the replies below." — 1 days old
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5659 fix(dapi): bound the state transition size pre-filter by the largest known limit](https://github.com/dashpay/platform/pull/5659) — 1 unresolved (1 human) · 0 days stale · ✋ changes requested · areas: rust-dapi · Policy: waiting-author
  - Top thread: "Why we can't use current active protocol version limit?" — 0 days old
  - Blocker: Author response is required after the latest human objection
- [dashpay/platform#5860 feat(platform-wallet): top up an identity the wallet does not manage from Platform addresses](https://github.com/dashpay/platform/pull/5860) — 2 unresolved (2 bot) · 1 days stale · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk, fallback · Policy: waiting-author
  - Top thread: "✅ **Resolved** at \`2271e8cd\`; see the replies below." — 1 days old
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5888 feat(platform-wallet): fresh shielded addresses and an own-address check](https://github.com/dashpay/platform/pull/5888) — 2 unresolved (2 bot) · 0 days stale · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk · Policy: waiting-bots
  - Top thread: "🟡 Suggestion: Cover malformed recipient lengths through the FFI export**" — 0 days old
  - Blocker: thepastaclaw has not reported for the current head
- [dashpay/platform#5862 fix(swift-sdk): run identityGet and dpnsGetUsername off the main actor](https://github.com/dashpay/platform/pull/5862) — 1 unresolved (1 bot) · 1 days stale · areas: swift-sdk · Policy: waiting-bots
  - Top thread: "✅ **Resolved** at \`5d99479c\`; see the replies below." — 1 days old
  - Blocker: thepastaclaw has not reported for the current head
- [dashpay/platform#4992 feat(swift-sdk): durable wallet presence marker readable while the device is locked](https://github.com/dashpay/platform/pull/4992) — areas: swift-sdk · Policy: ready-to-merge
  - Blocker: All policy requirements are satisfied
- [dashpay/platform#5536 fix: check a DashPay invitation claim against Platform before broadcasting it](https://github.com/dashpay/platform/pull/5536) — areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, rust-sdk, kotlin-sdk, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
- [dashpay/platform#5587 fix(platform-wallet)!: keep a contact's payment account across relaunches and replace it cleanly on rotation](https://github.com/dashpay/platform/pull/5587) — ⚠ merge conflict · 📝 draft · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5589 feat(platform-wallet-ffi)!: let hosts reset an outbound contact account's pool and keep its request digest](https://github.com/dashpay/platform/pull/5589) — 🔴 CI failing · 📝 draft · 🐢 targets fix/contact-rebuild-keeps-pool · areas: rs-platform-wallet-ffi, swift-sdk
- [dashpay/platform#5590 feat(kotlin-sdk)!: reset an outbound contact account's pool and keep its request digest on Android](https://github.com/dashpay/platform/pull/5590) — 🔴 CI failing · 📝 draft · 🐢 targets fix/contact-rebuild-ffi-swift · areas: kotlin-sdk, fallback

<a id="romchornyi-needs-action"></a>
#### Needs action (5)
- [dashpay/platform#5540 fix(platform-wallet): store externally supplied identity keys without a derivation breadcrumb](https://github.com/dashpay/platform/pull/5540) — 3 unresolved (3 bot) · 21 days stale · 🔴 CI failing · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk, fallback · Policy: waiting-author
  - Top thread: "✅ **Resolved** at \`b25b2543\`; see the replies below." — 21 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5853 feat: settle a shielded identity create whose inputs are spent without its identity](https://github.com/dashpay/platform/pull/5853) — 7 unresolved (7 bot) · 1 days stale · 🔴 CI failing · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, rust-sdk, kotlin-sdk, fallback · Policy: waiting-author
  - Top thread: "✅ **Resolved** at \`c9979afd\`; see the replies below." — 1 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5865 feat(platform-wallet): plan a shielded spend with its exact fee and spend against the plan](https://github.com/dashpay/platform/pull/5865) — 7 unresolved (7 bot) · 1 days stale · 🔴 CI failing · areas: dpp, rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk, fallback · Policy: waiting-author
  - Top thread: "✅ **Resolved** at \`99877748\`; see the replies below." — 1 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5863 feat(swift-sdk): shieldedShieldFromIdentityToRecipient](https://github.com/dashpay/platform/pull/5863) — 4 unresolved (4 bot) · 1 days stale · areas: rs-platform-wallet-ffi, swift-sdk, kotlin-sdk, fallback · Policy: waiting-author
  - Top thread: "✅ **Resolved** at \`31eb8df2\`; see the replies below." — 1 days old
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5659 fix(dapi): bound the state transition size pre-filter by the largest known limit](https://github.com/dashpay/platform/pull/5659) — 1 unresolved (1 human) · 0 days stale · ✋ changes requested · areas: rust-dapi · Policy: waiting-author
  - Top thread: "Why we can't use current active protocol version limit?" — 0 days old
  - Blocker: Author response is required after the latest human objection

<a id="romchornyi-ready-for-human"></a>
#### Ready for human (2)
- [dashpay/platform#4992 feat(swift-sdk): durable wallet presence marker readable while the device is locked](https://github.com/dashpay/platform/pull/4992) — areas: swift-sdk · Policy: ready-to-merge
  - Blocker: All policy requirements are satisfied
- [dashpay/platform#5536 fix: check a DashPay invitation claim against Platform before broadcasting it](https://github.com/dashpay/platform/pull/5536) — areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, rust-sdk, kotlin-sdk, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required

<a id="romchornyi-unresolved-comments"></a>
#### Unresolved Comments (8)
- [dashpay/platform#5540 fix(platform-wallet): store externally supplied identity keys without a derivation breadcrumb](https://github.com/dashpay/platform/pull/5540) — 3 unresolved (3 bot) · 21 days stale · 🔴 CI failing · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk, fallback · Policy: waiting-author
  - Top thread: "✅ **Resolved** at \`b25b2543\`; see the replies below." — 21 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5853 feat: settle a shielded identity create whose inputs are spent without its identity](https://github.com/dashpay/platform/pull/5853) — 7 unresolved (7 bot) · 1 days stale · 🔴 CI failing · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, rust-sdk, kotlin-sdk, fallback · Policy: waiting-author
  - Top thread: "✅ **Resolved** at \`c9979afd\`; see the replies below." — 1 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5865 feat(platform-wallet): plan a shielded spend with its exact fee and spend against the plan](https://github.com/dashpay/platform/pull/5865) — 7 unresolved (7 bot) · 1 days stale · 🔴 CI failing · areas: dpp, rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk, fallback · Policy: waiting-author
  - Top thread: "✅ **Resolved** at \`99877748\`; see the replies below." — 1 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5863 feat(swift-sdk): shieldedShieldFromIdentityToRecipient](https://github.com/dashpay/platform/pull/5863) — 4 unresolved (4 bot) · 1 days stale · areas: rs-platform-wallet-ffi, swift-sdk, kotlin-sdk, fallback · Policy: waiting-author
  - Top thread: "✅ **Resolved** at \`31eb8df2\`; see the replies below." — 1 days old
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5659 fix(dapi): bound the state transition size pre-filter by the largest known limit](https://github.com/dashpay/platform/pull/5659) — 1 unresolved (1 human) · 0 days stale · ✋ changes requested · areas: rust-dapi · Policy: waiting-author
  - Top thread: "Why we can't use current active protocol version limit?" — 0 days old
  - Blocker: Author response is required after the latest human objection
- [dashpay/platform#5860 feat(platform-wallet): top up an identity the wallet does not manage from Platform addresses](https://github.com/dashpay/platform/pull/5860) — 2 unresolved (2 bot) · 1 days stale · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk, fallback · Policy: waiting-author
  - Top thread: "✅ **Resolved** at \`2271e8cd\`; see the replies below." — 1 days old
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5888 feat(platform-wallet): fresh shielded addresses and an own-address check](https://github.com/dashpay/platform/pull/5888) — 2 unresolved (2 bot) · 0 days stale · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk · Policy: waiting-bots
  - Top thread: "🟡 Suggestion: Cover malformed recipient lengths through the FFI export**" — 0 days old
  - Blocker: thepastaclaw has not reported for the current head
- [dashpay/platform#5862 fix(swift-sdk): run identityGet and dpnsGetUsername off the main actor](https://github.com/dashpay/platform/pull/5862) — 1 unresolved (1 bot) · 1 days stale · areas: swift-sdk · Policy: waiting-bots
  - Top thread: "✅ **Resolved** at \`5d99479c\`; see the replies below." — 1 days old
  - Blocker: thepastaclaw has not reported for the current head

<a id="romchornyi-draft"></a>
#### Draft (4)
- [dashpay/platform#5258 feat(platform-wallet): probe the network about sends with an unknown broadcast outcome](https://github.com/dashpay/platform/pull/5258) — 3 unresolved (3 human) · 27 days stale · ⚠ merge conflict · 📝 draft · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, rust-sdk · Policy: draft
  - Top thread: "\[P2\] Carry the originating wallet generation in probe events" — 27 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5549 fix(platform-wallet-storage): restore DashPay contact address pools on SQLite load](https://github.com/dashpay/platform/pull/5549) — 3 unresolved (3 bot) · 20 days stale · ⚠ merge conflict · 🔴 CI failing · 📝 draft · areas: wallet-storage · Policy: draft
  - Top thread: "✅ **Withdrawn** at \`2bee2705\`; see the replies below." — 20 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5395 feat(platform-wallet): spend only final coins and report funds waiting on the network](https://github.com/dashpay/platform/pull/5395) — 1 unresolved (1 human) · 27 days stale · ⚠ merge conflict · ✋ changes requested · 📝 draft · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk, fallback · Policy: draft
  - Top thread: "this should belong to another layer - likely platform-wallet, or even rust-dashcore's key-wallet. FFI is not meant to st…" — 27 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5587 fix(platform-wallet)!: keep a contact's payment account across relaunches and replace it cleanly on rotation](https://github.com/dashpay/platform/pull/5587) — ⚠ merge conflict · 📝 draft · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage · Policy: draft
  - Blocker: Draft PR does not occupy a review slot

<a id="romchornyi-stale"></a>
#### Stale (2)
- [dashpay/platform#5589 feat(platform-wallet-ffi)!: let hosts reset an outbound contact account's pool and keep its request digest](https://github.com/dashpay/platform/pull/5589) — 🔴 CI failing · 📝 draft · 🐢 targets fix/contact-rebuild-keeps-pool · areas: rs-platform-wallet-ffi, swift-sdk
- [dashpay/platform#5590 feat(kotlin-sdk)!: reset an outbound contact account's pool and keep its request digest on Android](https://github.com/dashpay/platform/pull/5590) — 🔴 CI failing · 📝 draft · 🐢 targets fix/contact-rebuild-ffi-swift · areas: kotlin-sdk, fallback

<a id="romchornyi-clean"></a>
#### Clean (2)
- [dashpay/platform#4992 feat(swift-sdk): durable wallet presence marker readable while the device is locked](https://github.com/dashpay/platform/pull/4992) — areas: swift-sdk · Policy: ready-to-merge
  - Blocker: All policy requirements are satisfied
- [dashpay/platform#5536 fix: check a DashPay invitation claim against Platform before broadcasting it](https://github.com/dashpay/platform/pull/5536) — areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, rust-sdk, kotlin-sdk, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required

<a id="romchornyi-ready-for-review"></a>
#### Ready for Review (4)
- [dashpay/platform#5479 feat(platform-wallet-storage): persist DashPay backfill coverage and outbound-account markers](https://github.com/dashpay/platform/pull/5479) — by @HashEngineering · areas: rs-platform-wallet, wallet-storage · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 8e1ee2e24549cabf2d2e81a85a02fc613212aa1f
- [dashpay/platform#5793 fix(platform-wallet): make SpvRuntime::header_hash_at public](https://github.com/dashpay/platform/pull/5793) — by @PastaPastaPasta · areas: rs-platform-wallet · Policy: too-many-open-prs
  - Blocker: Human review is still required; automatic reviewer requests are paused until a review slot is available
- [dashpay/platform#5889 feat(platform-wallet): rotate a masternode's keys into the wallet](https://github.com/dashpay/platform/pull/5889) — by @llbartekll · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head
- [dashpay/platform#5890 feat(platform-wallet): keep masternode collateral out of coin selection](https://github.com/dashpay/platform/pull/5890) — by @llbartekll · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk, fallback · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head

<a id="claudius-maginificent"></a>
### @Claudius-Maginificent
<a id="claudius-maginificent-open"></a>
#### Open (55)
- [dashpay/platform#5645 fix(platform-wallet)!: check identity creation keys and funding in the Rust core](https://github.com/dashpay/platform/pull/5645) — 6 unresolved (4 CodeRabbit, 2 bot) · 9 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · areas: dpp, rs-platform-wallet, rs-platform-wallet-ffi, rust-sdk · Policy: waiting-author
  - Top thread: "&lt;picture&gt;&lt;img src="https://storage.googleapis.com/coderabbit_public_assets/review-comment-icons-v1/amber-16.svg" width="…" — 9 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: coderabbitai left review threads unresolved; resolve them
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/dash-evo-tool#1108 feat(documents): recognise indexOnly document types and delete their entries by values](https://github.com/dashpay/dash-evo-tool/pull/1108) — 3 unresolved (3 bot) · 20 days stale · ✋ changes requested · 📝 draft · 🐢 targets feat/document-optional-token-cost · areas: fallback
  - Top thread: "🟡 Suggestion: Make the non-deletable error actionable**" — 20 days old
- [dashpay/platform#5613 fix(platform-wallet-storage): persist sentinel-scoped token balances of wallet-owned identities](https://github.com/dashpay/platform/pull/5613) — 3 unresolved (3 bot) · 13 days stale · areas: wallet-storage · Policy: ready-to-merge
  - Top thread: "🟡 Suggestion: Assert the typed WalletIdMismatch payload**" — 13 days old
  - Blocker: All policy requirements are satisfied
- [dashpay/platform#5565 feat(platform-wallet)!: detect inconsistent shielded commitment trees and add explicit rebuild](https://github.com/dashpay/platform/pull/5565) — 3 unresolved (3 bot) · 13 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk, fallback · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Do not run the synchronous rebuild call on Swift's MainActor**" — 13 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
- [dashpay/dash-evo-tool#1117 fix(identity): choose signing keys Platform accepts on protocol 14](https://github.com/dashpay/dash-evo-tool/pull/1117) — 3 unresolved (3 bot) · 7 days stale · ✋ changes requested · 🔴 CI failing · areas: fallback · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Include create-consumption bounds in the upstream-selector follow-up**" — 7 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/platform#5546 fix(platform-wallet): support HASH160 DashPay contact signing keys](https://github.com/dashpay/platform/pull/5546) — 2 unresolved (2 bot) · 13 days stale · 🐢 targets feat/platform-wallet-public-signing-key-selection · areas: rs-drive-abci, dpp, rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk, fallback
  - Top thread: "🟡 Suggestion: Do not expose an unchecked signer-capability override in the public constructor**" — 13 days old
- [dashpay/tenderdash#1619 ci: assign merged pull requests to their release line's milestone](https://github.com/dashpay/tenderdash/pull/1619) — 4 unresolved (3 CodeRabbit, 1 bot) · 1 days stale · areas: github, fallback · Policy: waiting-author
  - Top thread: "&lt;picture&gt;&lt;img src="https://storage.googleapis.com/coderabbit_public_assets/review-comment-icons-v1/amber-16.svg" width="…" — 1 days old
  - Blocker: coderabbitai left review threads unresolved; resolve them
- [dashpay/dash-evo-tool#1124 feat(documents): let the user skip an optional token cost](https://github.com/dashpay/dash-evo-tool/pull/1124) — 1 unresolved (1 bot) · 17 days stale · ✋ changes requested · 📝 draft · 🐢 targets fix/protocol14-signing-key-selection · areas: fallback
  - Top thread: "🔴 Blocking: Show the credit-fee outcome when an optional token cost is disabled**" — 17 days old
- [dashpay/platform#5588 feat(platform-wallet)!: expose protocol-14-aware signing-key selection](https://github.com/dashpay/platform/pull/5588) — 1 unresolved (1 bot) · 13 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · areas: dpp, rs-platform-wallet, rs-platform-wallet-ffi, rust-sdk · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Include BLS keys in automatic document and token selection**" — 13 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
- [dashpay/platform#5707 fix(sdk): recover the nonce cache when unconfirmed nonces leave the accepted window](https://github.com/dashpay/platform/pull/5707) — 1 unresolved (1 bot) · 9 days stale · ✋ changes requested · 🔴 CI failing · 📝 draft · areas: rust-sdk · Policy: draft
  - Top thread: "🔴 Blocking: Do not reissue nonces belonging to outstanding operations**" — 9 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/dash-evo-tool#1109 feat(masternode): use wallet-held node keys and withdraw via platform-wallet](https://github.com/dashpay/dash-evo-tool/pull/1109) — ⚠ merge conflict · 🔴 CI failing · areas: fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
  - Blocker: Proceeded without thepastaclaw: no review within the configured window
- [dashpay/dash-evo-tool#1120 feat(identity): add keys with spending limits and raise key limits](https://github.com/dashpay/dash-evo-tool/pull/1120) — ⚠ merge conflict · 📝 draft · 🐢 targets fix/protocol14-signing-key-selection · areas: fallback
- [dashpay/dash-evo-tool#1121 feat(contracts): view and claim contract fee pots](https://github.com/dashpay/dash-evo-tool/pull/1121) — 📝 draft · 🐢 targets fix/protocol14-signing-key-selection · areas: fallback
- [dashpay/dash-evo-tool#1122 feat(documents): agree to document action fees before broadcasting](https://github.com/dashpay/dash-evo-tool/pull/1122) — ⚠ merge conflict · 📝 draft · 🐢 targets fix/protocol14-signing-key-selection · areas: fallback
- [dashpay/dash-evo-tool#1123 feat(identity): add authentication keys bound to a contract](https://github.com/dashpay/dash-evo-tool/pull/1123) — 📝 draft · 🐢 targets feat/identity-key-limits · areas: fallback
- [dashpay/dash-evo-tool#1128 refactor(tokens)!: send token actions through platform-wallet](https://github.com/dashpay/dash-evo-tool/pull/1128) — ⚠ merge conflict · 📝 draft · 🐢 targets chore/bump-platform-0d91c133 · areas: fallback
- [dashpay/dash-evo-tool#1137 fix(identity): unlock the wallet that holds the chosen signing key](https://github.com/dashpay/dash-evo-tool/pull/1137) — ⚠ merge conflict · 📝 draft · 🐢 targets fix/protocol14-signing-key-selection · areas: fallback
- [dashpay/dash-evo-tool#1153 chore(deps): bump dashpay/platform to v5.1-dev 6323a430](https://github.com/dashpay/dash-evo-tool/pull/1153) — areas: fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/dash-evo-tool#1158 fix(security): keep formatted panic messages out of logs and the terminal](https://github.com/dashpay/dash-evo-tool/pull/1158) — 🔴 CI failing · areas: fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
  - Blocker: Proceeded without thepastaclaw: no review within the configured window
- [dashpay/dash-evo-tool#1159 fix(app): keep a previous network's background events and a request's progress where they belong](https://github.com/dashpay/dash-evo-tool/pull/1159) — areas: fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/dash-evo-tool#1160 feat(identity): show an identity's tokens in the Identities hub](https://github.com/dashpay/dash-evo-tool/pull/1160) — ✋ changes requested · 🔴 CI failing · areas: fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
  - Blocker: Proceeded without coderabbitai: no review within the configured window
  - Blocker: Proceeded without thepastaclaw: no review within the configured window
- [dashpay/dash-evo-tool#1162 fix(app): close promptly while a network switch is in progress](https://github.com/dashpay/dash-evo-tool/pull/1162) — ⚠ merge conflict · 📝 draft · areas: fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/dash-evo-tool#1165 fix(wallet): send Platform address transfers and withdrawals through platform-wallet](https://github.com/dashpay/dash-evo-tool/pull/1165) — 🔴 CI failing · areas: fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
  - Blocker: Proceeded without thepastaclaw: no review within the configured window
- [dashpay/dash-evo-tool#1166 fix(identity): reconcile address-funded identity registration through platform-wallet](https://github.com/dashpay/dash-evo-tool/pull/1166) — 🐢 targets fix/platform-address-ops-via-platform-wallet · areas: fallback
- [dashpay/dash-evo-tool#1167 perf(wallet): stop re-copying wallet funding state for Max quotes and snapshots](https://github.com/dashpay/dash-evo-tool/pull/1167) — areas: fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
- [dashpay/dash-evo-tool#1168 fix(tokens): keep the Token Creator's identity current and its registration state reliable](https://github.com/dashpay/dash-evo-tool/pull/1168) — 📝 draft · areas: fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/dash-evo-tool#1169 fix(ui): keep sub-navigation labels on one line](https://github.com/dashpay/dash-evo-tool/pull/1169) — 📝 draft · areas: fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/dash-evo-tool#1170 fix(ui): retire finished progress banners and describe key protection correctly](https://github.com/dashpay/dash-evo-tool/pull/1170) — 📝 draft · areas: fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/dash-evo-tool#1171 fix(wallet): show keys awaiting their old password right after a storage update](https://github.com/dashpay/dash-evo-tool/pull/1171) — 📝 draft · areas: fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/dash-evo-tool#1172 fix(mcp): keep embedded MCP requests' intermediate results off the GUI](https://github.com/dashpay/dash-evo-tool/pull/1172) — 📝 draft · areas: fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/dash-evo-tool#1173 feat(cli): import a wallet from a recovery phrase read from stdin](https://github.com/dashpay/dash-evo-tool/pull/1173) — 📝 draft · areas: fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/dash-evo-tool#1174 feat(skills): mark defects in visual-diff galleries](https://github.com/dashpay/dash-evo-tool/pull/1174) — 📝 draft · 🐢 targets feat/det-cli-mnemonic-stdin · areas: fallback
- [dashpay/dash-evo-tool#1175 docs: add maintainable token tutorial production toolkit](https://github.com/dashpay/dash-evo-tool/pull/1175) — 📝 draft · areas: fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/dash-evo-tool#1176 feat(tokens): add guided token creation with complete review](https://github.com/dashpay/dash-evo-tool/pull/1176) — 📝 draft · areas: fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#3549 test(platform-wallet): e2e framework + full test suite — triage pins, Found-*/PA-* guards, fail-closed persist, Stage-2 merge](https://github.com/dashpay/platform/pull/3549) — 📝 draft · ⏸ deferred · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk, rust-sdk, kotlin-sdk, github, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5384 fix(platform-wallet)!: persist chain truncations so a stored wallet follows a reorg](https://github.com/dashpay/platform/pull/5384) — ⏸ deferred · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk, kotlin-sdk, fallback
- [dashpay/platform#5417 fix(platform-wallet): own per-wallet transaction accounting in Rust for SwiftData](https://github.com/dashpay/platform/pull/5417) — ⏸ deferred · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk
- [dashpay/platform#5644 fix!: follow DIP-11 key rules when disabling keys and reading contact requests](https://github.com/dashpay/platform/pull/5644) — ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · areas: dpp, rs-platform-wallet, rs-platform-wallet-ffi, rust-sdk, js-wasm-sdk, fallback · Policy: waiting-author
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
- [dashpay/platform#5647 feat: expose the Rust identity key layout to FFI, Swift and Kotlin hosts](https://github.com/dashpay/platform/pull/5647) — ✋ changes requested · 🔴 CI failing · 🐢 targets fix/platform-wallet-dip11-identity-key-layout · areas: rs-platform-wallet-ffi, swift-sdk, kotlin-sdk, fallback
- [dashpay/platform#5660 refactor(dpp): own the contract-bounds verdict for a document create](https://github.com/dashpay/platform/pull/5660) — 🐢 targets feat/platform-wallet-public-signing-key-selection · areas: rs-drive-abci, dpp, rs-platform-wallet
- [dashpay/platform#5661 feat: decide which identity keys may be disabled in the wallet, not in the mobile apps](https://github.com/dashpay/platform/pull/5661) — ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · 🐢 targets fix/platform-wallet-dip11-key-rules · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk, fallback
- [dashpay/platform#5670 fix(platform)!: refuse high-S signatures on key-recovery paths from protocol version 15](https://github.com/dashpay/platform/pull/5670) — 📝 draft · 🐢 targets shumkov/v51-rust-dashcore-dev-bump · areas: rs-drive-abci, dpp, fallback
- [dashpay/platform#5683 fix(platform-wallet): fetch the identity before building an identity update](https://github.com/dashpay/platform/pull/5683) — ⚠ merge conflict · 🔴 CI failing · areas: rs-platform-wallet · Policy: waiting-build
  - Blocker: The build must pass before a human is asked
- [dashpay/platform#5781 fix(wallet-storage): validate transaction bodies during bulk history load](https://github.com/dashpay/platform/pull/5781) — 📝 draft · 🐢 targets fix/pr-5126 · areas: wallet-storage
- [dashpay/platform#5782 fix(wallet-storage): restore Platform and Core from one wallet snapshot](https://github.com/dashpay/platform/pull/5782) — 📝 draft · 🐢 targets feat/sqlite-wallet-restore · areas: wallet-storage
- [dashpay/platform#5783 fix(wallet-storage)!: reconcile historical AssetUnlock identities](https://github.com/dashpay/platform/pull/5783) — 📝 draft · 🐢 targets fix/sqlite-bulk-history-identity · areas: rs-platform-wallet, wallet-storage
- [dashpay/platform#5787 fix(platform-wallet): classify asset locks funding foreign credits as outgoing](https://github.com/dashpay/platform/pull/5787) — 📝 draft · 🐢 targets feat/sqlite-wallet-restore · areas: rs-platform-wallet, wallet-storage
- [dashpay/platform#5788 fix(platform-wallet): compile shielded-only drain helpers only with the shielded feature](https://github.com/dashpay/platform/pull/5788) — 📝 draft · areas: rs-platform-wallet · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5836 build!: upgrade Tenderdash to v2.0.0-dev.1](https://github.com/dashpay/platform/pull/5836) — ⚠ merge conflict · 📝 draft · areas: rs-drive-abci, rust-sdk, dashmate, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/tenderdash#1578 fix(statesync): refuse surplus light block responses without blocking](https://github.com/dashpay/tenderdash/pull/1578) — 📝 draft · areas: fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/tenderdash#1598 fix(consensus): periodically re-announce round state to peers](https://github.com/dashpay/tenderdash/pull/1598) — 📝 draft · areas: fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/tenderdash#1599 fix(consensus): resend same-round votes the peer never acknowledged](https://github.com/dashpay/tenderdash/pull/1599) — 📝 draft · areas: fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/tenderdash#1600 fix(consensus): deliver and process a polka block for a validator left in its round](https://github.com/dashpay/tenderdash/pull/1600) — 📝 draft · areas: fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/tenderdash#1601 fix(consensus): stop a previous-height precommit marking the peer's current-height votes](https://github.com/dashpay/tenderdash/pull/1601) — 📝 draft · areas: fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/tenderdash#1618 fix(blocksync): hand over to consensus at the tip despite unserved peer height claims](https://github.com/dashpay/tenderdash/pull/1618) — areas: fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required

<a id="claudius-maginificent-needs-action"></a>
#### Needs action (9)
- [dashpay/platform#5645 fix(platform-wallet)!: check identity creation keys and funding in the Rust core](https://github.com/dashpay/platform/pull/5645) — 6 unresolved (4 CodeRabbit, 2 bot) · 9 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · areas: dpp, rs-platform-wallet, rs-platform-wallet-ffi, rust-sdk · Policy: waiting-author
  - Top thread: "&lt;picture&gt;&lt;img src="https://storage.googleapis.com/coderabbit_public_assets/review-comment-icons-v1/amber-16.svg" width="…" — 9 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: coderabbitai left review threads unresolved; resolve them
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5613 fix(platform-wallet-storage): persist sentinel-scoped token balances of wallet-owned identities](https://github.com/dashpay/platform/pull/5613) — 3 unresolved (3 bot) · 13 days stale · areas: wallet-storage · Policy: ready-to-merge
  - Top thread: "🟡 Suggestion: Assert the typed WalletIdMismatch payload**" — 13 days old
  - Blocker: All policy requirements are satisfied
- [dashpay/platform#5565 feat(platform-wallet)!: detect inconsistent shielded commitment trees and add explicit rebuild](https://github.com/dashpay/platform/pull/5565) — 3 unresolved (3 bot) · 13 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk, fallback · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Do not run the synchronous rebuild call on Swift's MainActor**" — 13 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
- [dashpay/dash-evo-tool#1117 fix(identity): choose signing keys Platform accepts on protocol 14](https://github.com/dashpay/dash-evo-tool/pull/1117) — 3 unresolved (3 bot) · 7 days stale · ✋ changes requested · 🔴 CI failing · areas: fallback · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Include create-consumption bounds in the upstream-selector follow-up**" — 7 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/platform#5588 feat(platform-wallet)!: expose protocol-14-aware signing-key selection](https://github.com/dashpay/platform/pull/5588) — 1 unresolved (1 bot) · 13 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · areas: dpp, rs-platform-wallet, rs-platform-wallet-ffi, rust-sdk · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Include BLS keys in automatic document and token selection**" — 13 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
- [dashpay/dash-evo-tool#1109 feat(masternode): use wallet-held node keys and withdraw via platform-wallet](https://github.com/dashpay/dash-evo-tool/pull/1109) — ⚠ merge conflict · 🔴 CI failing · areas: fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
  - Blocker: Proceeded without thepastaclaw: no review within the configured window
- [dashpay/dash-evo-tool#1160 feat(identity): show an identity's tokens in the Identities hub](https://github.com/dashpay/dash-evo-tool/pull/1160) — ✋ changes requested · 🔴 CI failing · areas: fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
  - Blocker: Proceeded without coderabbitai: no review within the configured window
  - Blocker: Proceeded without thepastaclaw: no review within the configured window
- [dashpay/platform#5644 fix!: follow DIP-11 key rules when disabling keys and reading contact requests](https://github.com/dashpay/platform/pull/5644) — ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · areas: dpp, rs-platform-wallet, rs-platform-wallet-ffi, rust-sdk, js-wasm-sdk, fallback · Policy: waiting-author
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
- [dashpay/platform#5683 fix(platform-wallet): fetch the identity before building an identity update](https://github.com/dashpay/platform/pull/5683) — ⚠ merge conflict · 🔴 CI failing · areas: rs-platform-wallet · Policy: waiting-build
  - Blocker: The build must pass before a human is asked

<a id="claudius-maginificent-ready-for-human"></a>
#### Ready for human (9)
- [dashpay/platform#5613 fix(platform-wallet-storage): persist sentinel-scoped token balances of wallet-owned identities](https://github.com/dashpay/platform/pull/5613) — 3 unresolved (3 bot) · 13 days stale · areas: wallet-storage · Policy: ready-to-merge
  - Top thread: "🟡 Suggestion: Assert the typed WalletIdMismatch payload**" — 13 days old
  - Blocker: All policy requirements are satisfied
- [dashpay/dash-evo-tool#1109 feat(masternode): use wallet-held node keys and withdraw via platform-wallet](https://github.com/dashpay/dash-evo-tool/pull/1109) — ⚠ merge conflict · 🔴 CI failing · areas: fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
  - Blocker: Proceeded without thepastaclaw: no review within the configured window
- [dashpay/dash-evo-tool#1153 chore(deps): bump dashpay/platform to v5.1-dev 6323a430](https://github.com/dashpay/dash-evo-tool/pull/1153) — areas: fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/dash-evo-tool#1158 fix(security): keep formatted panic messages out of logs and the terminal](https://github.com/dashpay/dash-evo-tool/pull/1158) — 🔴 CI failing · areas: fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
  - Blocker: Proceeded without thepastaclaw: no review within the configured window
- [dashpay/dash-evo-tool#1159 fix(app): keep a previous network's background events and a request's progress where they belong](https://github.com/dashpay/dash-evo-tool/pull/1159) — areas: fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/dash-evo-tool#1160 feat(identity): show an identity's tokens in the Identities hub](https://github.com/dashpay/dash-evo-tool/pull/1160) — ✋ changes requested · 🔴 CI failing · areas: fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
  - Blocker: Proceeded without coderabbitai: no review within the configured window
  - Blocker: Proceeded without thepastaclaw: no review within the configured window
- [dashpay/dash-evo-tool#1165 fix(wallet): send Platform address transfers and withdrawals through platform-wallet](https://github.com/dashpay/dash-evo-tool/pull/1165) — 🔴 CI failing · areas: fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
  - Blocker: Proceeded without thepastaclaw: no review within the configured window
- [dashpay/dash-evo-tool#1167 perf(wallet): stop re-copying wallet funding state for Max quotes and snapshots](https://github.com/dashpay/dash-evo-tool/pull/1167) — areas: fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
- [dashpay/tenderdash#1618 fix(blocksync): hand over to consensus at the tip despite unserved peer height claims](https://github.com/dashpay/tenderdash/pull/1618) — areas: fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required

<a id="claudius-maginificent-unresolved-comments"></a>
#### Unresolved Comments (6)
- [dashpay/platform#5645 fix(platform-wallet)!: check identity creation keys and funding in the Rust core](https://github.com/dashpay/platform/pull/5645) — 6 unresolved (4 CodeRabbit, 2 bot) · 9 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · areas: dpp, rs-platform-wallet, rs-platform-wallet-ffi, rust-sdk · Policy: waiting-author
  - Top thread: "&lt;picture&gt;&lt;img src="https://storage.googleapis.com/coderabbit_public_assets/review-comment-icons-v1/amber-16.svg" width="…" — 9 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: coderabbitai left review threads unresolved; resolve them
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5613 fix(platform-wallet-storage): persist sentinel-scoped token balances of wallet-owned identities](https://github.com/dashpay/platform/pull/5613) — 3 unresolved (3 bot) · 13 days stale · areas: wallet-storage · Policy: ready-to-merge
  - Top thread: "🟡 Suggestion: Assert the typed WalletIdMismatch payload**" — 13 days old
  - Blocker: All policy requirements are satisfied
- [dashpay/platform#5565 feat(platform-wallet)!: detect inconsistent shielded commitment trees and add explicit rebuild](https://github.com/dashpay/platform/pull/5565) — 3 unresolved (3 bot) · 13 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk, fallback · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Do not run the synchronous rebuild call on Swift's MainActor**" — 13 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
- [dashpay/dash-evo-tool#1117 fix(identity): choose signing keys Platform accepts on protocol 14](https://github.com/dashpay/dash-evo-tool/pull/1117) — 3 unresolved (3 bot) · 7 days stale · ✋ changes requested · 🔴 CI failing · areas: fallback · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Include create-consumption bounds in the upstream-selector follow-up**" — 7 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
  - Blocker: Proceeded without coderabbitai: no review within the configured window
- [dashpay/tenderdash#1619 ci: assign merged pull requests to their release line's milestone](https://github.com/dashpay/tenderdash/pull/1619) — 4 unresolved (3 CodeRabbit, 1 bot) · 1 days stale · areas: github, fallback · Policy: waiting-author
  - Top thread: "&lt;picture&gt;&lt;img src="https://storage.googleapis.com/coderabbit_public_assets/review-comment-icons-v1/amber-16.svg" width="…" — 1 days old
  - Blocker: coderabbitai left review threads unresolved; resolve them
- [dashpay/platform#5588 feat(platform-wallet)!: expose protocol-14-aware signing-key selection](https://github.com/dashpay/platform/pull/5588) — 1 unresolved (1 bot) · 13 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · areas: dpp, rs-platform-wallet, rs-platform-wallet-ffi, rust-sdk · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Include BLS keys in automatic document and token selection**" — 13 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix

<a id="claudius-maginificent-changes-requested"></a>
#### Changes Requested (2)
- [dashpay/dash-evo-tool#1160 feat(identity): show an identity's tokens in the Identities hub](https://github.com/dashpay/dash-evo-tool/pull/1160) — ✋ changes requested · 🔴 CI failing · areas: fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
  - Blocker: Proceeded without coderabbitai: no review within the configured window
  - Blocker: Proceeded without thepastaclaw: no review within the configured window
- [dashpay/platform#5644 fix!: follow DIP-11 key rules when disabling keys and reading contact requests](https://github.com/dashpay/platform/pull/5644) — ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · areas: dpp, rs-platform-wallet, rs-platform-wallet-ffi, rust-sdk, js-wasm-sdk, fallback · Policy: waiting-author
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix

<a id="claudius-maginificent-ci-failing"></a>
#### CI Failing (4)
- [dashpay/dash-evo-tool#1109 feat(masternode): use wallet-held node keys and withdraw via platform-wallet](https://github.com/dashpay/dash-evo-tool/pull/1109) — ⚠ merge conflict · 🔴 CI failing · areas: fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
  - Blocker: Proceeded without thepastaclaw: no review within the configured window
- [dashpay/dash-evo-tool#1158 fix(security): keep formatted panic messages out of logs and the terminal](https://github.com/dashpay/dash-evo-tool/pull/1158) — 🔴 CI failing · areas: fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
  - Blocker: Proceeded without thepastaclaw: no review within the configured window
- [dashpay/dash-evo-tool#1165 fix(wallet): send Platform address transfers and withdrawals through platform-wallet](https://github.com/dashpay/dash-evo-tool/pull/1165) — 🔴 CI failing · areas: fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
  - Blocker: Proceeded without thepastaclaw: no review within the configured window
- [dashpay/platform#5683 fix(platform-wallet): fetch the identity before building an identity update](https://github.com/dashpay/platform/pull/5683) — ⚠ merge conflict · 🔴 CI failing · areas: rs-platform-wallet · Policy: waiting-build
  - Blocker: The build must pass before a human is asked

<a id="claudius-maginificent-deferred"></a>
#### Deferred (3)
- [dashpay/platform#3549 test(platform-wallet): e2e framework + full test suite — triage pins, Found-*/PA-* guards, fail-closed persist, Stage-2 merge](https://github.com/dashpay/platform/pull/3549) — 📝 draft · ⏸ deferred · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk, rust-sdk, kotlin-sdk, github, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5384 fix(platform-wallet)!: persist chain truncations so a stored wallet follows a reorg](https://github.com/dashpay/platform/pull/5384) — ⏸ deferred · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk, kotlin-sdk, fallback
- [dashpay/platform#5417 fix(platform-wallet): own per-wallet transaction accounting in Rust for SwiftData](https://github.com/dashpay/platform/pull/5417) — ⏸ deferred · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk

<a id="claudius-maginificent-draft"></a>
#### Draft (17)
- [dashpay/platform#5707 fix(sdk): recover the nonce cache when unconfirmed nonces leave the accepted window](https://github.com/dashpay/platform/pull/5707) — 1 unresolved (1 bot) · 9 days stale · ✋ changes requested · 🔴 CI failing · 📝 draft · areas: rust-sdk · Policy: draft
  - Top thread: "🔴 Blocking: Do not reissue nonces belonging to outstanding operations**" — 9 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/dash-evo-tool#1162 fix(app): close promptly while a network switch is in progress](https://github.com/dashpay/dash-evo-tool/pull/1162) — ⚠ merge conflict · 📝 draft · areas: fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/dash-evo-tool#1168 fix(tokens): keep the Token Creator's identity current and its registration state reliable](https://github.com/dashpay/dash-evo-tool/pull/1168) — 📝 draft · areas: fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/dash-evo-tool#1169 fix(ui): keep sub-navigation labels on one line](https://github.com/dashpay/dash-evo-tool/pull/1169) — 📝 draft · areas: fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/dash-evo-tool#1170 fix(ui): retire finished progress banners and describe key protection correctly](https://github.com/dashpay/dash-evo-tool/pull/1170) — 📝 draft · areas: fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/dash-evo-tool#1171 fix(wallet): show keys awaiting their old password right after a storage update](https://github.com/dashpay/dash-evo-tool/pull/1171) — 📝 draft · areas: fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/dash-evo-tool#1172 fix(mcp): keep embedded MCP requests' intermediate results off the GUI](https://github.com/dashpay/dash-evo-tool/pull/1172) — 📝 draft · areas: fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/dash-evo-tool#1173 feat(cli): import a wallet from a recovery phrase read from stdin](https://github.com/dashpay/dash-evo-tool/pull/1173) — 📝 draft · areas: fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/dash-evo-tool#1175 docs: add maintainable token tutorial production toolkit](https://github.com/dashpay/dash-evo-tool/pull/1175) — 📝 draft · areas: fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/dash-evo-tool#1176 feat(tokens): add guided token creation with complete review](https://github.com/dashpay/dash-evo-tool/pull/1176) — 📝 draft · areas: fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5788 fix(platform-wallet): compile shielded-only drain helpers only with the shielded feature](https://github.com/dashpay/platform/pull/5788) — 📝 draft · areas: rs-platform-wallet · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5836 build!: upgrade Tenderdash to v2.0.0-dev.1](https://github.com/dashpay/platform/pull/5836) — ⚠ merge conflict · 📝 draft · areas: rs-drive-abci, rust-sdk, dashmate, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/tenderdash#1578 fix(statesync): refuse surplus light block responses without blocking](https://github.com/dashpay/tenderdash/pull/1578) — 📝 draft · areas: fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/tenderdash#1598 fix(consensus): periodically re-announce round state to peers](https://github.com/dashpay/tenderdash/pull/1598) — 📝 draft · areas: fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/tenderdash#1599 fix(consensus): resend same-round votes the peer never acknowledged](https://github.com/dashpay/tenderdash/pull/1599) — 📝 draft · areas: fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/tenderdash#1600 fix(consensus): deliver and process a polka block for a validator left in its round](https://github.com/dashpay/tenderdash/pull/1600) — 📝 draft · areas: fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/tenderdash#1601 fix(consensus): stop a previous-height precommit marking the peer's current-height votes](https://github.com/dashpay/tenderdash/pull/1601) — 📝 draft · areas: fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot

<a id="claudius-maginificent-stale"></a>
#### Stale (19)
- [dashpay/dash-evo-tool#1108 feat(documents): recognise indexOnly document types and delete their entries by values](https://github.com/dashpay/dash-evo-tool/pull/1108) — 3 unresolved (3 bot) · 20 days stale · ✋ changes requested · 📝 draft · 🐢 targets feat/document-optional-token-cost · areas: fallback
  - Top thread: "🟡 Suggestion: Make the non-deletable error actionable**" — 20 days old
- [dashpay/platform#5546 fix(platform-wallet): support HASH160 DashPay contact signing keys](https://github.com/dashpay/platform/pull/5546) — 2 unresolved (2 bot) · 13 days stale · 🐢 targets feat/platform-wallet-public-signing-key-selection · areas: rs-drive-abci, dpp, rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk, fallback
  - Top thread: "🟡 Suggestion: Do not expose an unchecked signer-capability override in the public constructor**" — 13 days old
- [dashpay/dash-evo-tool#1124 feat(documents): let the user skip an optional token cost](https://github.com/dashpay/dash-evo-tool/pull/1124) — 1 unresolved (1 bot) · 17 days stale · ✋ changes requested · 📝 draft · 🐢 targets fix/protocol14-signing-key-selection · areas: fallback
  - Top thread: "🔴 Blocking: Show the credit-fee outcome when an optional token cost is disabled**" — 17 days old
- [dashpay/dash-evo-tool#1120 feat(identity): add keys with spending limits and raise key limits](https://github.com/dashpay/dash-evo-tool/pull/1120) — ⚠ merge conflict · 📝 draft · 🐢 targets fix/protocol14-signing-key-selection · areas: fallback
- [dashpay/dash-evo-tool#1121 feat(contracts): view and claim contract fee pots](https://github.com/dashpay/dash-evo-tool/pull/1121) — 📝 draft · 🐢 targets fix/protocol14-signing-key-selection · areas: fallback
- [dashpay/dash-evo-tool#1122 feat(documents): agree to document action fees before broadcasting](https://github.com/dashpay/dash-evo-tool/pull/1122) — ⚠ merge conflict · 📝 draft · 🐢 targets fix/protocol14-signing-key-selection · areas: fallback
- [dashpay/dash-evo-tool#1123 feat(identity): add authentication keys bound to a contract](https://github.com/dashpay/dash-evo-tool/pull/1123) — 📝 draft · 🐢 targets feat/identity-key-limits · areas: fallback
- [dashpay/dash-evo-tool#1128 refactor(tokens)!: send token actions through platform-wallet](https://github.com/dashpay/dash-evo-tool/pull/1128) — ⚠ merge conflict · 📝 draft · 🐢 targets chore/bump-platform-0d91c133 · areas: fallback
- [dashpay/dash-evo-tool#1137 fix(identity): unlock the wallet that holds the chosen signing key](https://github.com/dashpay/dash-evo-tool/pull/1137) — ⚠ merge conflict · 📝 draft · 🐢 targets fix/protocol14-signing-key-selection · areas: fallback
- [dashpay/dash-evo-tool#1166 fix(identity): reconcile address-funded identity registration through platform-wallet](https://github.com/dashpay/dash-evo-tool/pull/1166) — 🐢 targets fix/platform-address-ops-via-platform-wallet · areas: fallback
- [dashpay/dash-evo-tool#1174 feat(skills): mark defects in visual-diff galleries](https://github.com/dashpay/dash-evo-tool/pull/1174) — 📝 draft · 🐢 targets feat/det-cli-mnemonic-stdin · areas: fallback
- [dashpay/platform#5647 feat: expose the Rust identity key layout to FFI, Swift and Kotlin hosts](https://github.com/dashpay/platform/pull/5647) — ✋ changes requested · 🔴 CI failing · 🐢 targets fix/platform-wallet-dip11-identity-key-layout · areas: rs-platform-wallet-ffi, swift-sdk, kotlin-sdk, fallback
- [dashpay/platform#5660 refactor(dpp): own the contract-bounds verdict for a document create](https://github.com/dashpay/platform/pull/5660) — 🐢 targets feat/platform-wallet-public-signing-key-selection · areas: rs-drive-abci, dpp, rs-platform-wallet
- [dashpay/platform#5661 feat: decide which identity keys may be disabled in the wallet, not in the mobile apps](https://github.com/dashpay/platform/pull/5661) — ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · 🐢 targets fix/platform-wallet-dip11-key-rules · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk, fallback
- [dashpay/platform#5670 fix(platform)!: refuse high-S signatures on key-recovery paths from protocol version 15](https://github.com/dashpay/platform/pull/5670) — 📝 draft · 🐢 targets shumkov/v51-rust-dashcore-dev-bump · areas: rs-drive-abci, dpp, fallback
- [dashpay/platform#5781 fix(wallet-storage): validate transaction bodies during bulk history load](https://github.com/dashpay/platform/pull/5781) — 📝 draft · 🐢 targets fix/pr-5126 · areas: wallet-storage
- [dashpay/platform#5782 fix(wallet-storage): restore Platform and Core from one wallet snapshot](https://github.com/dashpay/platform/pull/5782) — 📝 draft · 🐢 targets feat/sqlite-wallet-restore · areas: wallet-storage
- [dashpay/platform#5783 fix(wallet-storage)!: reconcile historical AssetUnlock identities](https://github.com/dashpay/platform/pull/5783) — 📝 draft · 🐢 targets fix/sqlite-bulk-history-identity · areas: rs-platform-wallet, wallet-storage
- [dashpay/platform#5787 fix(platform-wallet): classify asset locks funding foreign credits as outgoing](https://github.com/dashpay/platform/pull/5787) — 📝 draft · 🐢 targets feat/sqlite-wallet-restore · areas: rs-platform-wallet, wallet-storage

<a id="claudius-maginificent-clean"></a>
#### Clean (4)
- [dashpay/dash-evo-tool#1153 chore(deps): bump dashpay/platform to v5.1-dev 6323a430](https://github.com/dashpay/dash-evo-tool/pull/1153) — areas: fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/dash-evo-tool#1159 fix(app): keep a previous network's background events and a request's progress where they belong](https://github.com/dashpay/dash-evo-tool/pull/1159) — areas: fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/dash-evo-tool#1167 perf(wallet): stop re-copying wallet funding state for Max quotes and snapshots](https://github.com/dashpay/dash-evo-tool/pull/1167) — areas: fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
- [dashpay/tenderdash#1618 fix(blocksync): hand over to consensus at the tip despite unserved peer height claims](https://github.com/dashpay/tenderdash/pull/1618) — areas: fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required

<a id="quantumexplorer"></a>
### @QuantumExplorer
<a id="quantumexplorer-open"></a>
#### Open (24)
- [dashpay/rust-dashcore#1078 feat(key-wallet)!: lock masternode collateral out of coin selection](https://github.com/dashpay/rust-dashcore/pull/1078) — 8 unresolved (8 human) · 43 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · areas: key-wallet, key-wallet-manager · Policy: waiting-self-review
  - Top thread: "This sets the state by hand, and together with the serde round-trip it is the only coverage of the refresh inside \`updat…" — 43 days old
  - Blocker: Author must post /self-reviewed 44a9a010500fbd44aba09111b4c53a28b88e8073
- [dashpay/platform#4272 feat(dpp)!: dashpay payment detection keys and stealth derivation](https://github.com/dashpay/platform/pull/4272) — 9 unresolved (9 bot) · 101 days stale · ⚠ merge conflict · 📝 draft · areas: rs-drive, rs-drive-abci, dpp, rs-platform-wallet, swift-sdk, system-contracts, kotlin-sdk, fallback · Policy: draft
  - Top thread: "🔴 Blocking: Require compressed 33-byte encodings for payment keys**" — 101 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5750 feat(platform)!: random token transfer and mint amounts](https://github.com/dashpay/platform/pull/5750) — 8 unresolved (8 bot) · 9 days stale · 🐢 targets claude/platform-tenderdash-randomness-149e4e · areas: rs-drive, dpp, rs-platform-wallet, rust-sdk, rust-sdk-ffi, fallback
  - Top thread: "✅ **Resolved** at \`f8fcc8fd\`; see the replies below." — 9 days old
- [dashpay/platform#5749 feat(platform)!: sys.random.uniform document properties](https://github.com/dashpay/platform/pull/5749) — 8 unresolved (8 bot) · 9 days stale · areas: rs-drive, rs-drive-abci, dpp, fallback · Policy: waiting-author
  - Top thread: "✅ **Resolved** at \`0ead2b47\`; see the replies below." — 9 days old
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5753 feat(drive-abci)!: apply block randomness through the ApplyBlockRandomness ABCI method](https://github.com/dashpay/platform/pull/5753) — 7 unresolved (7 bot) · 8 days stale · ✋ changes requested · 🔴 CI failing · 🐢 targets claude/random-token-amounts · areas: rs-drive, rs-drive-abci, dpp, fallback
  - Top thread: "🟡 Suggestion: Exercise the protocol upgrade with signed commits**" — 8 days old
- [dashpay/platform#5529 feat(platform)!: token marketplace listings (PV15)](https://github.com/dashpay/platform/pull/5529) — 2 unresolved (2 bot) · 18 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · areas: rs-drive, rs-drive-abci, dpp, rs-platform-wallet, swift-sdk, kotlin-sdk, fallback · Policy: waiting-author
  - Top thread: "🔴 Blocking: Version the consensus order-ID derivation through platform-version tables**" — 18 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5530 feat(platform)!: token marketplace offers (PV15)](https://github.com/dashpay/platform/pull/5530) — 1 unresolved (1 bot) · 12 days stale · ✋ changes requested · 🔴 CI failing · 🐢 targets claude/token-market-listings · areas: rs-drive, rs-drive-abci, dpp, rs-platform-wallet, fallback
  - Top thread: "🔴 Blocking: Bound bid escrow by a buyer-signed credit limit**" — 12 days old
- [dashpay/tenderdash#1582 feat(abci)!: add the ApplyBlockRandomness method and commit beacon fields](https://github.com/dashpay/tenderdash/pull/1582) — 1 unresolved (1 bot) · 6 days stale · areas: fallback · Policy: waiting-author
  - Top thread: "✅ **Resolved** at \`cd6b4ec8\`; see the replies below." — 6 days old
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/tenderdash#1597 fix(p2p): trim the simple priority queue below its limit](https://github.com/dashpay/tenderdash/pull/1597) — 1 unresolved (1 bot) · 4 days stale · ⚠ merge conflict · areas: fallback · Policy: waiting-self-review
  - Top thread: "🟡 Suggestion: Count the larger overflow batches**" — 4 days old
  - Blocker: Author must post /self-reviewed fb3e02bfc772facdefade06c2eeca6743e23e329
- [dashpay/platform#5851 feat(platform)!: multiple choice proposals take their description from a moderated text (PV15)](https://github.com/dashpay/platform/pull/5851) — 1 unresolved (1 bot) · 0 days stale · ✋ changes requested · 🔴 CI failing · areas: rs-drive, rs-drive-abci, dpp, fallback · Policy: waiting-bots
  - Top thread: "🟡 Suggestion: Centralize the list of electable abilities instead of hardcoding it at the parser**" — 0 days old
  - Blocker: thepastaclaw has not reported for the current head
- [dashpay/platform#4519 feat(platform-wallet): rotate a masternode's keys into the wallet — ProUpRegTx orchestration, FFI, Swift](https://github.com/dashpay/platform/pull/4519) — ⚠ merge conflict · 🔴 CI failing · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 92a3ebe17adc12432fb756c63f172c663809007d
  - Blocker: Proceeded without thepastaclaw: no review within the configured window
- [dashpay/platform#4730 feat(platform)!: delta-based data contract update transition for protocol version 15](https://github.com/dashpay/platform/pull/4730) — ⚠ merge conflict · 🔴 CI failing · 📝 draft · areas: rs-drive, rs-drive-abci, dpp, rs-platform-wallet, rust-sdk, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#4776 feat(platform-wallet)!: let contract updates clear the description through the FFI](https://github.com/dashpay/platform/pull/4776) — ⚠ merge conflict · 🔴 CI failing · 🐢 targets feat/delta-contract-update-pv15 · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk, fallback
- [dashpay/platform#4986 fix(drive-abci)!: pay every reward share of a masternode and credit each identity once in the epoch payout (PV15)](https://github.com/dashpay/platform/pull/4986) — ⚠ merge conflict · 📝 draft · areas: rs-drive, rs-drive-abci, dpp, github, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5113 feat(platform-wallet)!: keep masternode collateral out of coin selection](https://github.com/dashpay/platform/pull/5113) — ⚠ merge conflict · 🔴 CI failing · 📝 draft · 🐢 targets chore/bump-rust-dashcore-secp-033 · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk, fallback
- [dashpay/platform#5531 feat(platform)!: record token marketplace events in token history (PV15)](https://github.com/dashpay/platform/pull/5531) — 🔴 CI failing · 🐢 targets claude/token-market-offers · areas: rs-drive, rs-drive-abci, dpp, system-contracts, fallback
- [dashpay/platform#5532 feat(platform)!: destroy a frozen identity's escrowed listings with its funds (PV15)](https://github.com/dashpay/platform/pull/5532) — 🔴 CI failing · 🐢 targets claude/token-market-history · areas: rs-drive, rs-drive-abci, fallback
- [dashpay/platform#5736 feat(platform)!: add a RedPallas signer key type with its own purpose (PV15)](https://github.com/dashpay/platform/pull/5736) — ⚠ merge conflict · 📝 draft · areas: rs-drive, rs-drive-abci, dpp, rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, rust-sdk-ffi, kotlin-sdk, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5751 feat(sdk): random token amounts in wasm-sdk and js-evo-sdk](https://github.com/dashpay/platform/pull/5751) — 🐢 targets claude/random-token-amounts · areas: js-wasm-sdk
- [dashpay/platform#5848 feat(platform)!: multiple choice decision proposals, voted on as a contest among documents (PV15)](https://github.com/dashpay/platform/pull/5848) — areas: rs-drive, rs-drive-abci, dpp, fallback · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head
- [dashpay/platform#5886 fix(platform)!: one vote poll per contest for index values sent in different forms (PV15)](https://github.com/dashpay/platform/pull/5886) — ✋ changes requested · areas: rs-drive, rs-drive-abci, dpp, fallback · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head
- [dashpay/tenderdash#1583 feat(consensus)!: recover a beacon from prevotes and sign precommits over the randomness-applied app hash](https://github.com/dashpay/tenderdash/pull/1583) — ⚠ merge conflict · 📝 draft · areas: fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/tenderdash#1584 fix(statesync): restore LastAppHash of the snapshot height, not the next one](https://github.com/dashpay/tenderdash/pull/1584) — areas: fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 23c1443e4077c7c7dfedf00f9446c762e669660f
- [dashpay/tenderdash#1595 fix(blocksync): serve blocks as the bytes they were committed with](https://github.com/dashpay/tenderdash/pull/1595) — ✋ changes requested · areas: fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed abfa42be14c81787071ca37a835b48155f509f98

<a id="quantumexplorer-needs-action"></a>
#### Needs action (8)
- [dashpay/rust-dashcore#1078 feat(key-wallet)!: lock masternode collateral out of coin selection](https://github.com/dashpay/rust-dashcore/pull/1078) — 8 unresolved (8 human) · 43 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · areas: key-wallet, key-wallet-manager · Policy: waiting-self-review
  - Top thread: "This sets the state by hand, and together with the serde round-trip it is the only coverage of the refresh inside \`updat…" — 43 days old
  - Blocker: Author must post /self-reviewed 44a9a010500fbd44aba09111b4c53a28b88e8073
- [dashpay/platform#5529 feat(platform)!: token marketplace listings (PV15)](https://github.com/dashpay/platform/pull/5529) — 2 unresolved (2 bot) · 18 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · areas: rs-drive, rs-drive-abci, dpp, rs-platform-wallet, swift-sdk, kotlin-sdk, fallback · Policy: waiting-author
  - Top thread: "🔴 Blocking: Version the consensus order-ID derivation through platform-version tables**" — 18 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/tenderdash#1582 feat(abci)!: add the ApplyBlockRandomness method and commit beacon fields](https://github.com/dashpay/tenderdash/pull/1582) — 1 unresolved (1 bot) · 6 days stale · areas: fallback · Policy: waiting-author
  - Top thread: "✅ **Resolved** at \`cd6b4ec8\`; see the replies below." — 6 days old
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/tenderdash#1597 fix(p2p): trim the simple priority queue below its limit](https://github.com/dashpay/tenderdash/pull/1597) — 1 unresolved (1 bot) · 4 days stale · ⚠ merge conflict · areas: fallback · Policy: waiting-self-review
  - Top thread: "🟡 Suggestion: Count the larger overflow batches**" — 4 days old
  - Blocker: Author must post /self-reviewed fb3e02bfc772facdefade06c2eeca6743e23e329
- [dashpay/platform#5851 feat(platform)!: multiple choice proposals take their description from a moderated text (PV15)](https://github.com/dashpay/platform/pull/5851) — 1 unresolved (1 bot) · 0 days stale · ✋ changes requested · 🔴 CI failing · areas: rs-drive, rs-drive-abci, dpp, fallback · Policy: waiting-bots
  - Top thread: "🟡 Suggestion: Centralize the list of electable abilities instead of hardcoding it at the parser**" — 0 days old
  - Blocker: thepastaclaw has not reported for the current head
- [dashpay/platform#4519 feat(platform-wallet): rotate a masternode's keys into the wallet — ProUpRegTx orchestration, FFI, Swift](https://github.com/dashpay/platform/pull/4519) — ⚠ merge conflict · 🔴 CI failing · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 92a3ebe17adc12432fb756c63f172c663809007d
  - Blocker: Proceeded without thepastaclaw: no review within the configured window
- [dashpay/platform#5886 fix(platform)!: one vote poll per contest for index values sent in different forms (PV15)](https://github.com/dashpay/platform/pull/5886) — ✋ changes requested · areas: rs-drive, rs-drive-abci, dpp, fallback · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head
- [dashpay/tenderdash#1595 fix(blocksync): serve blocks as the bytes they were committed with](https://github.com/dashpay/tenderdash/pull/1595) — ✋ changes requested · areas: fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed abfa42be14c81787071ca37a835b48155f509f98

<a id="quantumexplorer-unresolved-comments"></a>
#### Unresolved Comments (6)
- [dashpay/rust-dashcore#1078 feat(key-wallet)!: lock masternode collateral out of coin selection](https://github.com/dashpay/rust-dashcore/pull/1078) — 8 unresolved (8 human) · 43 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · areas: key-wallet, key-wallet-manager · Policy: waiting-self-review
  - Top thread: "This sets the state by hand, and together with the serde round-trip it is the only coverage of the refresh inside \`updat…" — 43 days old
  - Blocker: Author must post /self-reviewed 44a9a010500fbd44aba09111b4c53a28b88e8073
- [dashpay/platform#5749 feat(platform)!: sys.random.uniform document properties](https://github.com/dashpay/platform/pull/5749) — 8 unresolved (8 bot) · 9 days stale · areas: rs-drive, rs-drive-abci, dpp, fallback · Policy: waiting-author
  - Top thread: "✅ **Resolved** at \`0ead2b47\`; see the replies below." — 9 days old
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5529 feat(platform)!: token marketplace listings (PV15)](https://github.com/dashpay/platform/pull/5529) — 2 unresolved (2 bot) · 18 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · areas: rs-drive, rs-drive-abci, dpp, rs-platform-wallet, swift-sdk, kotlin-sdk, fallback · Policy: waiting-author
  - Top thread: "🔴 Blocking: Version the consensus order-ID derivation through platform-version tables**" — 18 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/tenderdash#1582 feat(abci)!: add the ApplyBlockRandomness method and commit beacon fields](https://github.com/dashpay/tenderdash/pull/1582) — 1 unresolved (1 bot) · 6 days stale · areas: fallback · Policy: waiting-author
  - Top thread: "✅ **Resolved** at \`cd6b4ec8\`; see the replies below." — 6 days old
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/tenderdash#1597 fix(p2p): trim the simple priority queue below its limit](https://github.com/dashpay/tenderdash/pull/1597) — 1 unresolved (1 bot) · 4 days stale · ⚠ merge conflict · areas: fallback · Policy: waiting-self-review
  - Top thread: "🟡 Suggestion: Count the larger overflow batches**" — 4 days old
  - Blocker: Author must post /self-reviewed fb3e02bfc772facdefade06c2eeca6743e23e329
- [dashpay/platform#5851 feat(platform)!: multiple choice proposals take their description from a moderated text (PV15)](https://github.com/dashpay/platform/pull/5851) — 1 unresolved (1 bot) · 0 days stale · ✋ changes requested · 🔴 CI failing · areas: rs-drive, rs-drive-abci, dpp, fallback · Policy: waiting-bots
  - Top thread: "🟡 Suggestion: Centralize the list of electable abilities instead of hardcoding it at the parser**" — 0 days old
  - Blocker: thepastaclaw has not reported for the current head

<a id="quantumexplorer-changes-requested"></a>
#### Changes Requested (2)
- [dashpay/platform#5886 fix(platform)!: one vote poll per contest for index values sent in different forms (PV15)](https://github.com/dashpay/platform/pull/5886) — ✋ changes requested · areas: rs-drive, rs-drive-abci, dpp, fallback · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head
- [dashpay/tenderdash#1595 fix(blocksync): serve blocks as the bytes they were committed with](https://github.com/dashpay/tenderdash/pull/1595) — ✋ changes requested · areas: fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed abfa42be14c81787071ca37a835b48155f509f98

<a id="quantumexplorer-ci-failing"></a>
#### CI Failing (1)
- [dashpay/platform#4519 feat(platform-wallet): rotate a masternode's keys into the wallet — ProUpRegTx orchestration, FFI, Swift](https://github.com/dashpay/platform/pull/4519) — ⚠ merge conflict · 🔴 CI failing · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 92a3ebe17adc12432fb756c63f172c663809007d
  - Blocker: Proceeded without thepastaclaw: no review within the configured window

<a id="quantumexplorer-draft"></a>
#### Draft (5)
- [dashpay/platform#4272 feat(dpp)!: dashpay payment detection keys and stealth derivation](https://github.com/dashpay/platform/pull/4272) — 9 unresolved (9 bot) · 101 days stale · ⚠ merge conflict · 📝 draft · areas: rs-drive, rs-drive-abci, dpp, rs-platform-wallet, swift-sdk, system-contracts, kotlin-sdk, fallback · Policy: draft
  - Top thread: "🔴 Blocking: Require compressed 33-byte encodings for payment keys**" — 101 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#4730 feat(platform)!: delta-based data contract update transition for protocol version 15](https://github.com/dashpay/platform/pull/4730) — ⚠ merge conflict · 🔴 CI failing · 📝 draft · areas: rs-drive, rs-drive-abci, dpp, rs-platform-wallet, rust-sdk, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#4986 fix(drive-abci)!: pay every reward share of a masternode and credit each identity once in the epoch payout (PV15)](https://github.com/dashpay/platform/pull/4986) — ⚠ merge conflict · 📝 draft · areas: rs-drive, rs-drive-abci, dpp, github, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5736 feat(platform)!: add a RedPallas signer key type with its own purpose (PV15)](https://github.com/dashpay/platform/pull/5736) — ⚠ merge conflict · 📝 draft · areas: rs-drive, rs-drive-abci, dpp, rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, rust-sdk-ffi, kotlin-sdk, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/tenderdash#1583 feat(consensus)!: recover a beacon from prevotes and sign precommits over the randomness-applied app hash](https://github.com/dashpay/tenderdash/pull/1583) — ⚠ merge conflict · 📝 draft · areas: fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot

<a id="quantumexplorer-stale"></a>
#### Stale (8)
- [dashpay/platform#5750 feat(platform)!: random token transfer and mint amounts](https://github.com/dashpay/platform/pull/5750) — 8 unresolved (8 bot) · 9 days stale · 🐢 targets claude/platform-tenderdash-randomness-149e4e · areas: rs-drive, dpp, rs-platform-wallet, rust-sdk, rust-sdk-ffi, fallback
  - Top thread: "✅ **Resolved** at \`f8fcc8fd\`; see the replies below." — 9 days old
- [dashpay/platform#5753 feat(drive-abci)!: apply block randomness through the ApplyBlockRandomness ABCI method](https://github.com/dashpay/platform/pull/5753) — 7 unresolved (7 bot) · 8 days stale · ✋ changes requested · 🔴 CI failing · 🐢 targets claude/random-token-amounts · areas: rs-drive, rs-drive-abci, dpp, fallback
  - Top thread: "🟡 Suggestion: Exercise the protocol upgrade with signed commits**" — 8 days old
- [dashpay/platform#5530 feat(platform)!: token marketplace offers (PV15)](https://github.com/dashpay/platform/pull/5530) — 1 unresolved (1 bot) · 12 days stale · ✋ changes requested · 🔴 CI failing · 🐢 targets claude/token-market-listings · areas: rs-drive, rs-drive-abci, dpp, rs-platform-wallet, fallback
  - Top thread: "🔴 Blocking: Bound bid escrow by a buyer-signed credit limit**" — 12 days old
- [dashpay/platform#4776 feat(platform-wallet)!: let contract updates clear the description through the FFI](https://github.com/dashpay/platform/pull/4776) — ⚠ merge conflict · 🔴 CI failing · 🐢 targets feat/delta-contract-update-pv15 · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk, fallback
- [dashpay/platform#5113 feat(platform-wallet)!: keep masternode collateral out of coin selection](https://github.com/dashpay/platform/pull/5113) — ⚠ merge conflict · 🔴 CI failing · 📝 draft · 🐢 targets chore/bump-rust-dashcore-secp-033 · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk, fallback
- [dashpay/platform#5531 feat(platform)!: record token marketplace events in token history (PV15)](https://github.com/dashpay/platform/pull/5531) — 🔴 CI failing · 🐢 targets claude/token-market-offers · areas: rs-drive, rs-drive-abci, dpp, system-contracts, fallback
- [dashpay/platform#5532 feat(platform)!: destroy a frozen identity's escrowed listings with its funds (PV15)](https://github.com/dashpay/platform/pull/5532) — 🔴 CI failing · 🐢 targets claude/token-market-history · areas: rs-drive, rs-drive-abci, fallback
- [dashpay/platform#5751 feat(sdk): random token amounts in wasm-sdk and js-evo-sdk](https://github.com/dashpay/platform/pull/5751) — 🐢 targets claude/random-token-amounts · areas: js-wasm-sdk

<a id="quantumexplorer-clean"></a>
#### Clean (2)
- [dashpay/platform#5848 feat(platform)!: multiple choice decision proposals, voted on as a contest among documents (PV15)](https://github.com/dashpay/platform/pull/5848) — areas: rs-drive, rs-drive-abci, dpp, fallback · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head
- [dashpay/tenderdash#1584 fix(statesync): restore LastAppHash of the snapshot height, not the next one](https://github.com/dashpay/tenderdash/pull/1584) — areas: fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 23c1443e4077c7c7dfedf00f9446c762e669660f

<a id="quantumexplorer-ready-for-review"></a>
#### Ready for Review (11)
- [dashpay/platform#4298 fix(dashmate): load ZeroSSL config in force mode](https://github.com/dashpay/platform/pull/4298) — by @thepastaclaw · areas: dashmate · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
- [dashpay/platform#5536 fix: check a DashPay invitation claim against Platform before broadcasting it](https://github.com/dashpay/platform/pull/5536) — by @romchornyi · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, rust-sdk, kotlin-sdk, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
- [dashpay/platform#5830 feat(kotlin-sdk): partial-amount platform-address withdrawal](https://github.com/dashpay/platform/pull/5830) — by @HashEngineering · areas: kotlin-sdk, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed a109e0fe1cdb448e7d0ca52dfb41c0bcb9c37053
- [dashpay/platform#5873 feat(kotlin-sdk): bind Platform → third-party Orchard shield (shieldedShieldToRecipient)](https://github.com/dashpay/platform/pull/5873) — by @HashEngineering · areas: kotlin-sdk, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed fbeb778fc7afe70db32fc3feaa332128405f306c
- [dashpay/platform#5890 feat(platform-wallet): keep masternode collateral out of coin selection](https://github.com/dashpay/platform/pull/5890) — by @llbartekll · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk, fallback · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head
- [dashpay/rust-dashcore#1367 test(key-wallet-manager): cover rescans over chainlocked blocks](https://github.com/dashpay/rust-dashcore/pull/1367) — by @PastaPastaPasta · areas: key-wallet, key-wallet-manager · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
- [dashpay/rust-dashcore#1378 fix(key-wallet)!: return errors from managed account constructors](https://github.com/dashpay/rust-dashcore/pull/1378) — by @ZocoLini · areas: key-wallet · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed f3111ed9f3cafff79b55eb4533cdf77cc18935d2
- [dashpay/rust-dashcore#1379 fix(dashcore): reject high-S signatures in \`verify_data_signature\`](https://github.com/dashpay/rust-dashcore/pull/1379) — by @ZocoLini · areas: fallback · Policy: waiting-bots
  - Blocker: coderabbitai has not reported for the current head
- [dashpay/rust-dashcore#1391 feat(key-wallet)!: lock masternode collateral out of coin selection](https://github.com/dashpay/rust-dashcore/pull/1391) — by @llbartekll · areas: key-wallet, key-wallet-manager, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 2cdb0d6f6bc3a402a9fee475bccd59a05b8ddd8e
- [dashpay/rust-dashcore#1392 test(dash-spv): drop a restored cycle with another entry hash](https://github.com/dashpay/rust-dashcore/pull/1392) — by @PastaPastaPasta · areas: dash-spv · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
- [dashpay/rust-dashcore#1393 fix(key-wallet): derive the BIP39 seed in wiped buffers](https://github.com/dashpay/rust-dashcore/pull/1393) — by @PastaPastaPasta · areas: key-wallet · Policy: waiting-build
  - Blocker: Waiting for the build to finish

<a id="pastapastapasta"></a>
### @PastaPastaPasta
<a id="pastapastapasta-open"></a>
#### Open (26)
- [dashpay/platform#5709 feat(platform-wallet): own the shielded-tip payment decisions in the wallet](https://github.com/dashpay/platform/pull/5709) — 22 unresolved (22 bot) · 11 days stale · 📝 draft · 🐢 targets feat/dashpay-shielded-tips · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, rust-sdk, kotlin-sdk, fallback
  - Top thread: "✅ **Resolved** at \`688c75e1\`; see the replies below." — 11 days old
- [dashpay/platform#4648 feat(drive-abci)!: state sync via ABCI snapshots with reduced platform state (protocol v15)](https://github.com/dashpay/platform/pull/4648) — 8 unresolved (8 bot) · 38 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · 📝 draft · areas: rs-drive, rs-drive-abci, dpp, fallback · Policy: draft
  - Top thread: "🟡 Suggestion: Continue the restored target through consensus and a restart**" — 38 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5288 feat(platform-wallet): support DashPay shielded tips with dedicated accounts](https://github.com/dashpay/platform/pull/5288) — 4 unresolved (1 CodeRabbit, 3 bot) · 11 days stale · ⚠ merge conflict · 🔴 CI failing · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk, rust-sdk, kotlin-sdk, fallback · Policy: waiting-author
  - Top thread: "&lt;picture&gt;&lt;img src="https://storage.googleapis.com/coderabbit_public_assets/review-comment-icons-v1/amber-16.svg" width="…" — 11 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: coderabbitai left review threads unresolved; resolve them
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5711 feat: shielded-tip screens over the wallet's tip decisions](https://github.com/dashpay/platform/pull/5711) — 4 unresolved (4 bot) · 11 days stale · 📝 draft · 🐢 targets feat/dashpay-shielded-tips-wallet-policy · areas: swift-sdk, kotlin-sdk
  - Top thread: "✅ **Resolved** at \`451dab59\`; see the replies below." — 11 days old
- [dashpay/platform#5786 feat(sdk): tri-state quorum lookups, bounded funding retries and a DPNS contest policy in dash-sdk-cxx](https://github.com/dashpay/platform/pull/5786) — 3 unresolved (3 bot) · 5 days stale · areas: rs-drive, rs-platform-wallet, rust-sdk, js-wasm-sdk, fallback · Policy: too-many-open-prs
  - Top thread: "✅ **Resolved** at \`e46d43eb\`; see the replies below." — 5 days old
  - Blocker: Human review is still required; automatic reviewer requests are paused until a review slot is available
- [dashpay/platform#4965 test(sdk): cargo-fuzz harness over drive-proof-verifier FromProof](https://github.com/dashpay/platform/pull/4965) — 1 unresolved (1 bot) · 37 days stale · 📝 draft · areas: github, fallback · Policy: draft
  - Top thread: "🟡 Suggestion: Correct the claim that PV13 reaches the V0 proof decoder**" — 37 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5879 fix(platform)!: a data contract holds a bounded number of patterns of bounded compiled size (PV15)](https://github.com/dashpay/platform/pull/5879) — 3 unresolved (1 CodeRabbit, 2 bot) · 0 days stale · ⚠ merge conflict · 🔴 CI failing · areas: rs-drive-abci, dpp, fallback · Policy: waiting-author
  - Top thread: "🔴 Blocking: Record the cost of pattern validation before returning errors**" — 0 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: coderabbitai left review threads unresolved; resolve them
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/grovedb#1007 build(deps)!: use dashpay/orchard with the faster prover](https://github.com/dashpay/grovedb/pull/1007) — 🔴 CI failing · 📝 draft · areas: fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5277 perf(platform-wallet): don't hold the shielded store lock across sync network I/O](https://github.com/dashpay/platform/pull/5277) — areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed de486366461d17ef4845e90de4961499b0b1faeb
- [dashpay/platform#5280 perf(platform-wallet)!: reuse a recent recorded-anchor set on the shielded spend path](https://github.com/dashpay/platform/pull/5280) — ⚠ merge conflict · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed e56af70d1e78f203f4aa18aeeae640a0f75a351e
- [dashpay/platform#5282 perf(platform-wallet)!: prove the shield bundle while waiting for the InstantSend lock](https://github.com/dashpay/platform/pull/5282) — ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · areas: dpp, rs-platform-wallet, rs-platform-wallet-ffi, fallback · Policy: waiting-author
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
- [dashpay/platform#5285 feat(platform)!: subscribe to committed state transitions matching document and address filters](https://github.com/dashpay/platform/pull/5285) — ⚠ merge conflict · 📝 draft · areas: rs-drive, rs-drive-abci, rust-sdk, dashmate, rust-dapi, github, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5289 feat(sdk)!: optional BIP-39 passphrase through the mnemonic resolver and wallet creation](https://github.com/dashpay/platform/pull/5289) — ⚠ merge conflict · 📝 draft · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, rust-sdk-ffi, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5292 feat(dashmate): state sync configuration for tenderdash and drive snapshots](https://github.com/dashpay/platform/pull/5292) — ⚠ merge conflict · 📝 draft · areas: dashmate · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5475 feat(sdk): forward DashPay profiles, invitations and masternode voting in dash-sdk-cxx](https://github.com/dashpay/platform/pull/5475) — ⚠ merge conflict · 📝 draft · areas: rs-platform-wallet, rs-platform-wallet-ffi, rust-sdk, github, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5577 feat: gzip-compress DAPI responses](https://github.com/dashpay/platform/pull/5577) — ✋ changes requested · 🔴 CI failing · areas: dashmate, rust-dapi, fallback · Policy: waiting-author
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
- [dashpay/platform#5610 perf(sdk): verify each block's signature once instead of on every proved response](https://github.com/dashpay/platform/pull/5610) — ⚠ merge conflict · 📝 draft · areas: fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5780 fix(platform-wallet-storage): restore UTXOs into the account that derives them](https://github.com/dashpay/platform/pull/5780) — areas: wallet-storage · Policy: too-many-open-prs
  - Blocker: Human review is still required; automatic reviewer requests are paused until a review slot is available
- [dashpay/platform#5784 fix(platform-wallet): export AssetLockBuildAmount](https://github.com/dashpay/platform/pull/5784) — 📝 draft · areas: rs-platform-wallet · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5792 fix(platform-wallet): persist coins a block's re-scan credits](https://github.com/dashpay/platform/pull/5792) — 📝 draft · areas: rs-platform-wallet, wallet-storage · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5793 fix(platform-wallet): make SpvRuntime::header_hash_at public](https://github.com/dashpay/platform/pull/5793) — areas: rs-platform-wallet · Policy: too-many-open-prs
  - Blocker: Human review is still required; automatic reviewer requests are paused until a review slot is available
- [dashpay/platform#5794 feat(platform-wallet-storage): read one asset lock's status by outpoint](https://github.com/dashpay/platform/pull/5794) — areas: wallet-storage · Policy: too-many-open-prs
  - Blocker: Human review is still required; automatic reviewer requests are paused until a review slot is available
- [dashpay/platform#5802 fix(platform-wallet-storage): don't look for or count non-funds used addresses as unresolved](https://github.com/dashpay/platform/pull/5802) — 📝 draft · areas: wallet-storage, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#1367 test(key-wallet-manager): cover rescans over chainlocked blocks](https://github.com/dashpay/rust-dashcore/pull/1367) — areas: key-wallet, key-wallet-manager · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
- [dashpay/rust-dashcore#1392 test(dash-spv): drop a restored cycle with another entry hash](https://github.com/dashpay/rust-dashcore/pull/1392) — areas: dash-spv · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
- [dashpay/rust-dashcore#1393 fix(key-wallet): derive the BIP39 seed in wiped buffers](https://github.com/dashpay/rust-dashcore/pull/1393) — areas: key-wallet · Policy: waiting-build
  - Blocker: Waiting for the build to finish

<a id="pastapastapasta-needs-action"></a>
#### Needs action (6)
- [dashpay/platform#5288 feat(platform-wallet): support DashPay shielded tips with dedicated accounts](https://github.com/dashpay/platform/pull/5288) — 4 unresolved (1 CodeRabbit, 3 bot) · 11 days stale · ⚠ merge conflict · 🔴 CI failing · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk, rust-sdk, kotlin-sdk, fallback · Policy: waiting-author
  - Top thread: "&lt;picture&gt;&lt;img src="https://storage.googleapis.com/coderabbit_public_assets/review-comment-icons-v1/amber-16.svg" width="…" — 11 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: coderabbitai left review threads unresolved; resolve them
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5786 feat(sdk): tri-state quorum lookups, bounded funding retries and a DPNS contest policy in dash-sdk-cxx](https://github.com/dashpay/platform/pull/5786) — 3 unresolved (3 bot) · 5 days stale · areas: rs-drive, rs-platform-wallet, rust-sdk, js-wasm-sdk, fallback · Policy: too-many-open-prs
  - Top thread: "✅ **Resolved** at \`e46d43eb\`; see the replies below." — 5 days old
  - Blocker: Human review is still required; automatic reviewer requests are paused until a review slot is available
- [dashpay/platform#5879 fix(platform)!: a data contract holds a bounded number of patterns of bounded compiled size (PV15)](https://github.com/dashpay/platform/pull/5879) — 3 unresolved (1 CodeRabbit, 2 bot) · 0 days stale · ⚠ merge conflict · 🔴 CI failing · areas: rs-drive-abci, dpp, fallback · Policy: waiting-author
  - Top thread: "🔴 Blocking: Record the cost of pattern validation before returning errors**" — 0 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: coderabbitai left review threads unresolved; resolve them
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5280 perf(platform-wallet)!: reuse a recent recorded-anchor set on the shielded spend path](https://github.com/dashpay/platform/pull/5280) — ⚠ merge conflict · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed e56af70d1e78f203f4aa18aeeae640a0f75a351e
- [dashpay/platform#5282 perf(platform-wallet)!: prove the shield bundle while waiting for the InstantSend lock](https://github.com/dashpay/platform/pull/5282) — ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · areas: dpp, rs-platform-wallet, rs-platform-wallet-ffi, fallback · Policy: waiting-author
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
- [dashpay/platform#5577 feat: gzip-compress DAPI responses](https://github.com/dashpay/platform/pull/5577) — ✋ changes requested · 🔴 CI failing · areas: dashmate, rust-dapi, fallback · Policy: waiting-author
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix

<a id="pastapastapasta-ready-for-human"></a>
#### Ready for human (2)
- [dashpay/rust-dashcore#1367 test(key-wallet-manager): cover rescans over chainlocked blocks](https://github.com/dashpay/rust-dashcore/pull/1367) — areas: key-wallet, key-wallet-manager · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
- [dashpay/rust-dashcore#1392 test(dash-spv): drop a restored cycle with another entry hash](https://github.com/dashpay/rust-dashcore/pull/1392) — areas: dash-spv · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required

<a id="pastapastapasta-unresolved-comments"></a>
#### Unresolved Comments (3)
- [dashpay/platform#5288 feat(platform-wallet): support DashPay shielded tips with dedicated accounts](https://github.com/dashpay/platform/pull/5288) — 4 unresolved (1 CodeRabbit, 3 bot) · 11 days stale · ⚠ merge conflict · 🔴 CI failing · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk, rust-sdk, kotlin-sdk, fallback · Policy: waiting-author
  - Top thread: "&lt;picture&gt;&lt;img src="https://storage.googleapis.com/coderabbit_public_assets/review-comment-icons-v1/amber-16.svg" width="…" — 11 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: coderabbitai left review threads unresolved; resolve them
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5786 feat(sdk): tri-state quorum lookups, bounded funding retries and a DPNS contest policy in dash-sdk-cxx](https://github.com/dashpay/platform/pull/5786) — 3 unresolved (3 bot) · 5 days stale · areas: rs-drive, rs-platform-wallet, rust-sdk, js-wasm-sdk, fallback · Policy: too-many-open-prs
  - Top thread: "✅ **Resolved** at \`e46d43eb\`; see the replies below." — 5 days old
  - Blocker: Human review is still required; automatic reviewer requests are paused until a review slot is available
- [dashpay/platform#5879 fix(platform)!: a data contract holds a bounded number of patterns of bounded compiled size (PV15)](https://github.com/dashpay/platform/pull/5879) — 3 unresolved (1 CodeRabbit, 2 bot) · 0 days stale · ⚠ merge conflict · 🔴 CI failing · areas: rs-drive-abci, dpp, fallback · Policy: waiting-author
  - Top thread: "🔴 Blocking: Record the cost of pattern validation before returning errors**" — 0 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: coderabbitai left review threads unresolved; resolve them
  - Blocker: thepastaclaw left review threads unresolved; resolve them

<a id="pastapastapasta-changes-requested"></a>
#### Changes Requested (2)
- [dashpay/platform#5282 perf(platform-wallet)!: prove the shield bundle while waiting for the InstantSend lock](https://github.com/dashpay/platform/pull/5282) — ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · areas: dpp, rs-platform-wallet, rs-platform-wallet-ffi, fallback · Policy: waiting-author
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
- [dashpay/platform#5577 feat: gzip-compress DAPI responses](https://github.com/dashpay/platform/pull/5577) — ✋ changes requested · 🔴 CI failing · areas: dashmate, rust-dapi, fallback · Policy: waiting-author
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix

<a id="pastapastapasta-draft"></a>
#### Draft (11)
- [dashpay/platform#4648 feat(drive-abci)!: state sync via ABCI snapshots with reduced platform state (protocol v15)](https://github.com/dashpay/platform/pull/4648) — 8 unresolved (8 bot) · 38 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · 📝 draft · areas: rs-drive, rs-drive-abci, dpp, fallback · Policy: draft
  - Top thread: "🟡 Suggestion: Continue the restored target through consensus and a restart**" — 38 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#4965 test(sdk): cargo-fuzz harness over drive-proof-verifier FromProof](https://github.com/dashpay/platform/pull/4965) — 1 unresolved (1 bot) · 37 days stale · 📝 draft · areas: github, fallback · Policy: draft
  - Top thread: "🟡 Suggestion: Correct the claim that PV13 reaches the V0 proof decoder**" — 37 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/grovedb#1007 build(deps)!: use dashpay/orchard with the faster prover](https://github.com/dashpay/grovedb/pull/1007) — 🔴 CI failing · 📝 draft · areas: fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5285 feat(platform)!: subscribe to committed state transitions matching document and address filters](https://github.com/dashpay/platform/pull/5285) — ⚠ merge conflict · 📝 draft · areas: rs-drive, rs-drive-abci, rust-sdk, dashmate, rust-dapi, github, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5289 feat(sdk)!: optional BIP-39 passphrase through the mnemonic resolver and wallet creation](https://github.com/dashpay/platform/pull/5289) — ⚠ merge conflict · 📝 draft · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, rust-sdk-ffi, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5292 feat(dashmate): state sync configuration for tenderdash and drive snapshots](https://github.com/dashpay/platform/pull/5292) — ⚠ merge conflict · 📝 draft · areas: dashmate · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5475 feat(sdk): forward DashPay profiles, invitations and masternode voting in dash-sdk-cxx](https://github.com/dashpay/platform/pull/5475) — ⚠ merge conflict · 📝 draft · areas: rs-platform-wallet, rs-platform-wallet-ffi, rust-sdk, github, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5610 perf(sdk): verify each block's signature once instead of on every proved response](https://github.com/dashpay/platform/pull/5610) — ⚠ merge conflict · 📝 draft · areas: fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5784 fix(platform-wallet): export AssetLockBuildAmount](https://github.com/dashpay/platform/pull/5784) — 📝 draft · areas: rs-platform-wallet · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5792 fix(platform-wallet): persist coins a block's re-scan credits](https://github.com/dashpay/platform/pull/5792) — 📝 draft · areas: rs-platform-wallet, wallet-storage · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5802 fix(platform-wallet-storage): don't look for or count non-funds used addresses as unresolved](https://github.com/dashpay/platform/pull/5802) — 📝 draft · areas: wallet-storage, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot

<a id="pastapastapasta-stale"></a>
#### Stale (2)
- [dashpay/platform#5709 feat(platform-wallet): own the shielded-tip payment decisions in the wallet](https://github.com/dashpay/platform/pull/5709) — 22 unresolved (22 bot) · 11 days stale · 📝 draft · 🐢 targets feat/dashpay-shielded-tips · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, rust-sdk, kotlin-sdk, fallback
  - Top thread: "✅ **Resolved** at \`688c75e1\`; see the replies below." — 11 days old
- [dashpay/platform#5711 feat: shielded-tip screens over the wallet's tip decisions](https://github.com/dashpay/platform/pull/5711) — 4 unresolved (4 bot) · 11 days stale · 📝 draft · 🐢 targets feat/dashpay-shielded-tips-wallet-policy · areas: swift-sdk, kotlin-sdk
  - Top thread: "✅ **Resolved** at \`451dab59\`; see the replies below." — 11 days old

<a id="pastapastapasta-clean"></a>
#### Clean (8)
- [dashpay/platform#5277 perf(platform-wallet): don't hold the shielded store lock across sync network I/O](https://github.com/dashpay/platform/pull/5277) — areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed de486366461d17ef4845e90de4961499b0b1faeb
- [dashpay/platform#5280 perf(platform-wallet)!: reuse a recent recorded-anchor set on the shielded spend path](https://github.com/dashpay/platform/pull/5280) — ⚠ merge conflict · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed e56af70d1e78f203f4aa18aeeae640a0f75a351e
- [dashpay/platform#5780 fix(platform-wallet-storage): restore UTXOs into the account that derives them](https://github.com/dashpay/platform/pull/5780) — areas: wallet-storage · Policy: too-many-open-prs
  - Blocker: Human review is still required; automatic reviewer requests are paused until a review slot is available
- [dashpay/platform#5793 fix(platform-wallet): make SpvRuntime::header_hash_at public](https://github.com/dashpay/platform/pull/5793) — areas: rs-platform-wallet · Policy: too-many-open-prs
  - Blocker: Human review is still required; automatic reviewer requests are paused until a review slot is available
- [dashpay/platform#5794 feat(platform-wallet-storage): read one asset lock's status by outpoint](https://github.com/dashpay/platform/pull/5794) — areas: wallet-storage · Policy: too-many-open-prs
  - Blocker: Human review is still required; automatic reviewer requests are paused until a review slot is available
- [dashpay/rust-dashcore#1367 test(key-wallet-manager): cover rescans over chainlocked blocks](https://github.com/dashpay/rust-dashcore/pull/1367) — areas: key-wallet, key-wallet-manager · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
- [dashpay/rust-dashcore#1392 test(dash-spv): drop a restored cycle with another entry hash](https://github.com/dashpay/rust-dashcore/pull/1392) — areas: dash-spv · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
- [dashpay/rust-dashcore#1393 fix(key-wallet): derive the BIP39 seed in wiped buffers](https://github.com/dashpay/rust-dashcore/pull/1393) — areas: key-wallet · Policy: waiting-build
  - Blocker: Waiting for the build to finish

<a id="pastapastapasta-ready-for-review"></a>
#### Ready for Review (1)
- [dashpay/platform#4298 fix(dashmate): load ZeroSSL config in force mode](https://github.com/dashpay/platform/pull/4298) — by @thepastaclaw · areas: dashmate · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required

<a id="shumkov"></a>
### @shumkov
<a id="shumkov-open"></a>
#### Open (12)
- [dashpay/rust-dashcore#634 feat(key-wallet-manager): single-map WalletManager with Arc&lt;RwLock&lt;T&gt;&gt; per wallet](https://github.com/dashpay/rust-dashcore/pull/634) — 20 unresolved (20 CodeRabbit) · 218 days stale · ⚠ merge conflict · ✋ changes requested · areas: dash-spv, key-wallet, key-wallet-manager, fallback · Policy: waiting-author
  - Top thread: "_⚠️ Potential issue_ \| _🟡 Minor_" — 218 days old
  - Blocker: coderabbitai requested changes on this head; dismiss the review or push a fix
  - Blocker: coderabbitai left review threads unresolved; resolve them
- [dashpay/platform#4660 feat(sdk): expose the document erase and history lifecycle on mobile and FFI](https://github.com/dashpay/platform/pull/4660) — 2 unresolved (2 bot) · 50 days stale · ✋ changes requested · 🔴 CI failing · 🐢 targets keep-history-lifecycle · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, rust-sdk-ffi, kotlin-sdk, fallback
  - Top thread: "🟡 Suggestion: Validate history selector discriminant at the C boundary**" — 50 days old
- [dashpay/platform#4657 feat(platform)!: delete and erase lifecycle for keep-history documents](https://github.com/dashpay/platform/pull/4657) — 2 unresolved (2 bot) · 50 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · areas: rs-drive, rs-drive-abci, dpp, rust-sdk, js-wasm-sdk, fallback · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Deduplicate lifecycle containers at the Drive batch boundary**" — 50 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5656 fix(dashmate): build local images for every group node that builds any](https://github.com/dashpay/platform/pull/5656) — 1 unresolved (1 bot) · 13 days stale · ✋ changes requested · 🔴 CI failing · 🐢 targets fix/dashmate/update-path-bugs · areas: dashmate
  - Top thread: "🔴 Blocking: Build the shared base image before dependent service images across group configurations**" — 13 days old
- [dashpay/platform#5882 feat(sdk): back SPV up with the trusted quorum service in Auto mode](https://github.com/dashpay/platform/pull/5882) — 1 unresolved (1 bot) · 0 days stale · areas: rs-platform-wallet, rust-sdk, rust-sdk-ffi, github, fallback · Policy: waiting-self-review
  - Top thread: "🟡 Suggestion: Update the existing Swift trust indicator with Auto's new semantics**" — 0 days old
  - Blocker: Author must post /self-reviewed 8bbb3d09f7da44890130121f5b4d129440d4dfb2
- [dashpay/platform#4283 fix(dashmate): keep the node up when an image pull fails, and stop reporting success when it did not](https://github.com/dashpay/platform/pull/4283) — ⚠ merge conflict · 🔴 CI failing · areas: dashmate · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 3ceb3315ea9b383ae5816ea4f0e9961202770ed9
  - Blocker: Proceeded without thepastaclaw: no review within the configured window
- [dashpay/platform#5654 fix(dashmate): restrict a gateway TLS private key an older version left readable](https://github.com/dashpay/platform/pull/5654) — ✋ changes requested · 🔴 CI failing · areas: dashmate · Policy: waiting-author
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
- [dashpay/platform#5655 fix(dashmate): print the default config or group when called without an argument](https://github.com/dashpay/platform/pull/5655) — areas: dashmate · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 105c7318e6e3cb846caeadaa8d87731b3142c2cf
- [dashpay/platform#5763 docs: e2e test architecture and legacy JS retirement plans (draft, not for merge)](https://github.com/dashpay/platform/pull/5763) — 📝 draft · areas: fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5819 feat(dashmate): add the modules that let dashmate update itself](https://github.com/dashpay/platform/pull/5819) — 📝 draft · 🐢 targets fix/dashmate/update-path-bugs · areas: dashmate
- [dashpay/platform#5883 feat(swift-sdk): default SDK to the adaptive context provider in Auto mode](https://github.com/dashpay/platform/pull/5883) — 🐢 targets shumkov/adaptive-auto-fallback · areas: swift-sdk, fallback
- [dashpay/platform#5884 feat(kotlin-sdk): bridge the adaptive context provider and build it by default](https://github.com/dashpay/platform/pull/5884) — ✋ changes requested · 🐢 targets shumkov/adaptive-swift-default · areas: kotlin-sdk, fallback

<a id="shumkov-needs-action"></a>
#### Needs action (5)
- [dashpay/rust-dashcore#634 feat(key-wallet-manager): single-map WalletManager with Arc&lt;RwLock&lt;T&gt;&gt; per wallet](https://github.com/dashpay/rust-dashcore/pull/634) — 20 unresolved (20 CodeRabbit) · 218 days stale · ⚠ merge conflict · ✋ changes requested · areas: dash-spv, key-wallet, key-wallet-manager, fallback · Policy: waiting-author
  - Top thread: "_⚠️ Potential issue_ \| _🟡 Minor_" — 218 days old
  - Blocker: coderabbitai requested changes on this head; dismiss the review or push a fix
  - Blocker: coderabbitai left review threads unresolved; resolve them
- [dashpay/platform#4657 feat(platform)!: delete and erase lifecycle for keep-history documents](https://github.com/dashpay/platform/pull/4657) — 2 unresolved (2 bot) · 50 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · areas: rs-drive, rs-drive-abci, dpp, rust-sdk, js-wasm-sdk, fallback · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Deduplicate lifecycle containers at the Drive batch boundary**" — 50 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5882 feat(sdk): back SPV up with the trusted quorum service in Auto mode](https://github.com/dashpay/platform/pull/5882) — 1 unresolved (1 bot) · 0 days stale · areas: rs-platform-wallet, rust-sdk, rust-sdk-ffi, github, fallback · Policy: waiting-self-review
  - Top thread: "🟡 Suggestion: Update the existing Swift trust indicator with Auto's new semantics**" — 0 days old
  - Blocker: Author must post /self-reviewed 8bbb3d09f7da44890130121f5b4d129440d4dfb2
- [dashpay/platform#4283 fix(dashmate): keep the node up when an image pull fails, and stop reporting success when it did not](https://github.com/dashpay/platform/pull/4283) — ⚠ merge conflict · 🔴 CI failing · areas: dashmate · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 3ceb3315ea9b383ae5816ea4f0e9961202770ed9
  - Blocker: Proceeded without thepastaclaw: no review within the configured window
- [dashpay/platform#5654 fix(dashmate): restrict a gateway TLS private key an older version left readable](https://github.com/dashpay/platform/pull/5654) — ✋ changes requested · 🔴 CI failing · areas: dashmate · Policy: waiting-author
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix

<a id="shumkov-unresolved-comments"></a>
#### Unresolved Comments (3)
- [dashpay/rust-dashcore#634 feat(key-wallet-manager): single-map WalletManager with Arc&lt;RwLock&lt;T&gt;&gt; per wallet](https://github.com/dashpay/rust-dashcore/pull/634) — 20 unresolved (20 CodeRabbit) · 218 days stale · ⚠ merge conflict · ✋ changes requested · areas: dash-spv, key-wallet, key-wallet-manager, fallback · Policy: waiting-author
  - Top thread: "_⚠️ Potential issue_ \| _🟡 Minor_" — 218 days old
  - Blocker: coderabbitai requested changes on this head; dismiss the review or push a fix
  - Blocker: coderabbitai left review threads unresolved; resolve them
- [dashpay/platform#4657 feat(platform)!: delete and erase lifecycle for keep-history documents](https://github.com/dashpay/platform/pull/4657) — 2 unresolved (2 bot) · 50 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · areas: rs-drive, rs-drive-abci, dpp, rust-sdk, js-wasm-sdk, fallback · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Deduplicate lifecycle containers at the Drive batch boundary**" — 50 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5882 feat(sdk): back SPV up with the trusted quorum service in Auto mode](https://github.com/dashpay/platform/pull/5882) — 1 unresolved (1 bot) · 0 days stale · areas: rs-platform-wallet, rust-sdk, rust-sdk-ffi, github, fallback · Policy: waiting-self-review
  - Top thread: "🟡 Suggestion: Update the existing Swift trust indicator with Auto's new semantics**" — 0 days old
  - Blocker: Author must post /self-reviewed 8bbb3d09f7da44890130121f5b4d129440d4dfb2

<a id="shumkov-changes-requested"></a>
#### Changes Requested (1)
- [dashpay/platform#5654 fix(dashmate): restrict a gateway TLS private key an older version left readable](https://github.com/dashpay/platform/pull/5654) — ✋ changes requested · 🔴 CI failing · areas: dashmate · Policy: waiting-author
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix

<a id="shumkov-ci-failing"></a>
#### CI Failing (1)
- [dashpay/platform#4283 fix(dashmate): keep the node up when an image pull fails, and stop reporting success when it did not](https://github.com/dashpay/platform/pull/4283) — ⚠ merge conflict · 🔴 CI failing · areas: dashmate · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 3ceb3315ea9b383ae5816ea4f0e9961202770ed9
  - Blocker: Proceeded without thepastaclaw: no review within the configured window

<a id="shumkov-draft"></a>
#### Draft (1)
- [dashpay/platform#5763 docs: e2e test architecture and legacy JS retirement plans (draft, not for merge)](https://github.com/dashpay/platform/pull/5763) — 📝 draft · areas: fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot

<a id="shumkov-stale"></a>
#### Stale (5)
- [dashpay/platform#4660 feat(sdk): expose the document erase and history lifecycle on mobile and FFI](https://github.com/dashpay/platform/pull/4660) — 2 unresolved (2 bot) · 50 days stale · ✋ changes requested · 🔴 CI failing · 🐢 targets keep-history-lifecycle · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, rust-sdk-ffi, kotlin-sdk, fallback
  - Top thread: "🟡 Suggestion: Validate history selector discriminant at the C boundary**" — 50 days old
- [dashpay/platform#5656 fix(dashmate): build local images for every group node that builds any](https://github.com/dashpay/platform/pull/5656) — 1 unresolved (1 bot) · 13 days stale · ✋ changes requested · 🔴 CI failing · 🐢 targets fix/dashmate/update-path-bugs · areas: dashmate
  - Top thread: "🔴 Blocking: Build the shared base image before dependent service images across group configurations**" — 13 days old
- [dashpay/platform#5819 feat(dashmate): add the modules that let dashmate update itself](https://github.com/dashpay/platform/pull/5819) — 📝 draft · 🐢 targets fix/dashmate/update-path-bugs · areas: dashmate
- [dashpay/platform#5883 feat(swift-sdk): default SDK to the adaptive context provider in Auto mode](https://github.com/dashpay/platform/pull/5883) — 🐢 targets shumkov/adaptive-auto-fallback · areas: swift-sdk, fallback
- [dashpay/platform#5884 feat(kotlin-sdk): bridge the adaptive context provider and build it by default](https://github.com/dashpay/platform/pull/5884) — ✋ changes requested · 🐢 targets shumkov/adaptive-swift-default · areas: kotlin-sdk, fallback

<a id="shumkov-clean"></a>
#### Clean (1)
- [dashpay/platform#5655 fix(dashmate): print the default config or group when called without an argument](https://github.com/dashpay/platform/pull/5655) — areas: dashmate · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 105c7318e6e3cb846caeadaa8d87731b3142c2cf

<a id="shumkov-ready-for-review"></a>
#### Ready for Review (8)
- [dashpay/platform#4298 fix(dashmate): load ZeroSSL config in force mode](https://github.com/dashpay/platform/pull/4298) — by @thepastaclaw · areas: dashmate · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
- [dashpay/platform#5187 ci: tolerate optional S3 cache export outages on v4.3-dev](https://github.com/dashpay/platform/pull/5187) — by @ktechmidas · areas: github · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed fbbf13ee41f8bd8b9f03f1dc5afec6a619967d97
- [dashpay/platform#5536 fix: check a DashPay invitation claim against Platform before broadcasting it](https://github.com/dashpay/platform/pull/5536) — by @romchornyi · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, rust-sdk, kotlin-sdk, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
- [dashpay/platform#5848 feat(platform)!: multiple choice decision proposals, voted on as a contest among documents (PV15)](https://github.com/dashpay/platform/pull/5848) — by @QuantumExplorer · areas: rs-drive, rs-drive-abci, dpp, fallback · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head
- [dashpay/platform#5873 feat(kotlin-sdk): bind Platform → third-party Orchard shield (shieldedShieldToRecipient)](https://github.com/dashpay/platform/pull/5873) — by @HashEngineering · areas: kotlin-sdk, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed fbeb778fc7afe70db32fc3feaa332128405f306c
- [dashpay/platform#5890 feat(platform-wallet): keep masternode collateral out of coin selection](https://github.com/dashpay/platform/pull/5890) — by @llbartekll · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk, fallback · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head
- [dashpay/tenderdash#1584 fix(statesync): restore LastAppHash of the snapshot height, not the next one](https://github.com/dashpay/tenderdash/pull/1584) — by @QuantumExplorer · areas: fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 23c1443e4077c7c7dfedf00f9446c762e669660f
- [dashpay/tenderdash#1618 fix(blocksync): hand over to consensus at the tip despite unserved peer height claims](https://github.com/dashpay/tenderdash/pull/1618) — by @Claudius-Maginificent · areas: fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required

<a id="bfoss765"></a>
### @bfoss765
<a id="bfoss765-open"></a>
#### Open (2)
- [dashpay/platform#4313 feat(kotlin-sdk): one-time Orchard key shielded-invite API (inviter + claim)](https://github.com/dashpay/platform/pull/4313) — 9 unresolved (9 bot) · 73 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk, fallback · Policy: waiting-author
  - Top thread: "✅ **Resolved** at \`eccc65fa\`; see the replies below." — 73 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#4312 feat(platform-wallet): multi-output shielded transfers + output-aware fee predictor](https://github.com/dashpay/platform/pull/4312) — 1 unresolved (1 bot) · 73 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · areas: dpp, rs-platform-wallet, rs-platform-wallet-ffi, kotlin-sdk, fallback · Policy: waiting-self-review
  - Top thread: "🟡 Suggestion: Keep envelope-aware action counting behind the crate boundary**" — 73 days old
  - Blocker: Author must post /self-reviewed da932f96a8670c641065954de8b58752e99233a3

<a id="bfoss765-needs-action"></a>
#### Needs action (2)
- [dashpay/platform#4313 feat(kotlin-sdk): one-time Orchard key shielded-invite API (inviter + claim)](https://github.com/dashpay/platform/pull/4313) — 9 unresolved (9 bot) · 73 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk, fallback · Policy: waiting-author
  - Top thread: "✅ **Resolved** at \`eccc65fa\`; see the replies below." — 73 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#4312 feat(platform-wallet): multi-output shielded transfers + output-aware fee predictor](https://github.com/dashpay/platform/pull/4312) — 1 unresolved (1 bot) · 73 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · areas: dpp, rs-platform-wallet, rs-platform-wallet-ffi, kotlin-sdk, fallback · Policy: waiting-self-review
  - Top thread: "🟡 Suggestion: Keep envelope-aware action counting behind the crate boundary**" — 73 days old
  - Blocker: Author must post /self-reviewed da932f96a8670c641065954de8b58752e99233a3

<a id="bfoss765-unresolved-comments"></a>
#### Unresolved Comments (2)
- [dashpay/platform#4313 feat(kotlin-sdk): one-time Orchard key shielded-invite API (inviter + claim)](https://github.com/dashpay/platform/pull/4313) — 9 unresolved (9 bot) · 73 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk, fallback · Policy: waiting-author
  - Top thread: "✅ **Resolved** at \`eccc65fa\`; see the replies below." — 73 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#4312 feat(platform-wallet): multi-output shielded transfers + output-aware fee predictor](https://github.com/dashpay/platform/pull/4312) — 1 unresolved (1 bot) · 73 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · areas: dpp, rs-platform-wallet, rs-platform-wallet-ffi, kotlin-sdk, fallback · Policy: waiting-self-review
  - Top thread: "🟡 Suggestion: Keep envelope-aware action counting behind the crate boundary**" — 73 days old
  - Blocker: Author must post /self-reviewed da932f96a8670c641065954de8b58752e99233a3

<a id="lklimek"></a>
### @lklimek
<a id="lklimek-open"></a>
#### Open (11)
- [dashpay/platform#5528 fix(platform-wallet): retain drain operation history through funding conflicts](https://github.com/dashpay/platform/pull/5528) — 7 unresolved (7 bot) · 21 days stale · ✋ changes requested · 📝 draft · 🐢 targets feat/sqlite-wallet-restore · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk, kotlin-sdk
  - Top thread: "🟡 Suggestion: Retain diagnostics for malformed FFI operation snapshots**" — 21 days old
- [dashpay/platform#5323 refactor: reject legacy BLS storage formats](https://github.com/dashpay/platform/pull/5323) — 1 unresolved (1 bot) · 3 days stale · 🔴 CI failing · areas: rs-drive-abci, github, fallback · Policy: waiting-self-review
  - Top thread: "🟡 Suggestion: Cover the required v4.0.0 migration boundary**" — 3 days old
  - Blocker: Author must post /self-reviewed 50c2487f6e349c16904b5bc9abbeeea3be316f10
- [dashpay/dash-evo-tool#1161 fix(ci): isolate release signing and constrain agent write paths](https://github.com/dashpay/dash-evo-tool/pull/1161) — 📝 draft · areas: github, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5150 fix(platform-wallet): correct wallet-level transaction history accounting](https://github.com/dashpay/platform/pull/5150) — ✋ changes requested · areas: rs-platform-wallet, wallet-storage, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 2d025ae336d2467a2da50c99e463026eb0d3910e
- [dashpay/platform#5207 fix(platform-wallet): require only atomic tracked-lock writes for reconciliation](https://github.com/dashpay/platform/pull/5207) — ⏸ deferred · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 6c01f7e279336cc6dd81347d2a069975876e3912
  - Blocker: Proceeded without thepastaclaw: no review within the configured window
- [dashpay/platform#5210 feat(wallet-storage)!: restore complete Core wallet snapshots from SQLite](https://github.com/dashpay/platform/pull/5210) — ⏸ deferred · areas: rs-platform-wallet, wallet-storage
- [dashpay/platform#5220 fix(platform-wallet)!: replay recorded transaction history on load for every persister](https://github.com/dashpay/platform/pull/5220) — ⏸ deferred · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk, fallback
- [dashpay/platform#5519 refactor: remove DPP raw secp256k1 dependency](https://github.com/dashpay/platform/pull/5519) — ⏸ deferred · areas: rs-drive-abci, dpp, rs-platform-wallet, rs-platform-wallet-ffi, rust-sdk, rust-sdk-ffi, js-wasm-sdk, fallback · Policy: waiting-author
  - Blocker: coderabbitai left review threads unresolved; resolve them
- [dashpay/rust-dashcore#1336 fix(wallet)!: authenticate pending spends and InstantLock evidence](https://github.com/dashpay/rust-dashcore/pull/1336) — ⚠ merge conflict · ✋ changes requested · areas: dash-spv, key-wallet, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 8137008f59197b29d8431196ce7546c4945886a6
- [dashpay/tenderdash#1515 fix(service)!: join remaining service workers before shutdown completes](https://github.com/dashpay/tenderdash/pull/1515) — ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · areas: fallback · Policy: waiting-author
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
- [dashpay/tenderdash#1593 feat(consensus)!: specify full consensus parameter commitment and remove app version parameter](https://github.com/dashpay/tenderdash/pull/1593) — ⚠ merge conflict · ✋ changes requested · 📝 draft · areas: fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot

<a id="lklimek-needs-action"></a>
#### Needs action (4)
- [dashpay/platform#5323 refactor: reject legacy BLS storage formats](https://github.com/dashpay/platform/pull/5323) — 1 unresolved (1 bot) · 3 days stale · 🔴 CI failing · areas: rs-drive-abci, github, fallback · Policy: waiting-self-review
  - Top thread: "🟡 Suggestion: Cover the required v4.0.0 migration boundary**" — 3 days old
  - Blocker: Author must post /self-reviewed 50c2487f6e349c16904b5bc9abbeeea3be316f10
- [dashpay/platform#5150 fix(platform-wallet): correct wallet-level transaction history accounting](https://github.com/dashpay/platform/pull/5150) — ✋ changes requested · areas: rs-platform-wallet, wallet-storage, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 2d025ae336d2467a2da50c99e463026eb0d3910e
- [dashpay/rust-dashcore#1336 fix(wallet)!: authenticate pending spends and InstantLock evidence](https://github.com/dashpay/rust-dashcore/pull/1336) — ⚠ merge conflict · ✋ changes requested · areas: dash-spv, key-wallet, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 8137008f59197b29d8431196ce7546c4945886a6
- [dashpay/tenderdash#1515 fix(service)!: join remaining service workers before shutdown completes](https://github.com/dashpay/tenderdash/pull/1515) — ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · areas: fallback · Policy: waiting-author
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix

<a id="lklimek-unresolved-comments"></a>
#### Unresolved Comments (1)
- [dashpay/platform#5323 refactor: reject legacy BLS storage formats](https://github.com/dashpay/platform/pull/5323) — 1 unresolved (1 bot) · 3 days stale · 🔴 CI failing · areas: rs-drive-abci, github, fallback · Policy: waiting-self-review
  - Top thread: "🟡 Suggestion: Cover the required v4.0.0 migration boundary**" — 3 days old
  - Blocker: Author must post /self-reviewed 50c2487f6e349c16904b5bc9abbeeea3be316f10

<a id="lklimek-changes-requested"></a>
#### Changes Requested (3)
- [dashpay/platform#5150 fix(platform-wallet): correct wallet-level transaction history accounting](https://github.com/dashpay/platform/pull/5150) — ✋ changes requested · areas: rs-platform-wallet, wallet-storage, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 2d025ae336d2467a2da50c99e463026eb0d3910e
- [dashpay/rust-dashcore#1336 fix(wallet)!: authenticate pending spends and InstantLock evidence](https://github.com/dashpay/rust-dashcore/pull/1336) — ⚠ merge conflict · ✋ changes requested · areas: dash-spv, key-wallet, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 8137008f59197b29d8431196ce7546c4945886a6
- [dashpay/tenderdash#1515 fix(service)!: join remaining service workers before shutdown completes](https://github.com/dashpay/tenderdash/pull/1515) — ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · areas: fallback · Policy: waiting-author
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix

<a id="lklimek-deferred"></a>
#### Deferred (4)
- [dashpay/platform#5207 fix(platform-wallet): require only atomic tracked-lock writes for reconciliation](https://github.com/dashpay/platform/pull/5207) — ⏸ deferred · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 6c01f7e279336cc6dd81347d2a069975876e3912
  - Blocker: Proceeded without thepastaclaw: no review within the configured window
- [dashpay/platform#5210 feat(wallet-storage)!: restore complete Core wallet snapshots from SQLite](https://github.com/dashpay/platform/pull/5210) — ⏸ deferred · areas: rs-platform-wallet, wallet-storage
- [dashpay/platform#5220 fix(platform-wallet)!: replay recorded transaction history on load for every persister](https://github.com/dashpay/platform/pull/5220) — ⏸ deferred · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk, fallback
- [dashpay/platform#5519 refactor: remove DPP raw secp256k1 dependency](https://github.com/dashpay/platform/pull/5519) — ⏸ deferred · areas: rs-drive-abci, dpp, rs-platform-wallet, rs-platform-wallet-ffi, rust-sdk, rust-sdk-ffi, js-wasm-sdk, fallback · Policy: waiting-author
  - Blocker: coderabbitai left review threads unresolved; resolve them

<a id="lklimek-draft"></a>
#### Draft (2)
- [dashpay/dash-evo-tool#1161 fix(ci): isolate release signing and constrain agent write paths](https://github.com/dashpay/dash-evo-tool/pull/1161) — 📝 draft · areas: github, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/tenderdash#1593 feat(consensus)!: specify full consensus parameter commitment and remove app version parameter](https://github.com/dashpay/tenderdash/pull/1593) — ⚠ merge conflict · ✋ changes requested · 📝 draft · areas: fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot

<a id="lklimek-stale"></a>
#### Stale (1)
- [dashpay/platform#5528 fix(platform-wallet): retain drain operation history through funding conflicts](https://github.com/dashpay/platform/pull/5528) — 7 unresolved (7 bot) · 21 days stale · ✋ changes requested · 📝 draft · 🐢 targets feat/sqlite-wallet-restore · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk, kotlin-sdk
  - Top thread: "🟡 Suggestion: Retain diagnostics for malformed FFI operation snapshots**" — 21 days old

<a id="lklimek-ready-for-review"></a>
#### Ready for Review (9)
- [dashpay/dash-evo-tool#1153 chore(deps): bump dashpay/platform to v5.1-dev 6323a430](https://github.com/dashpay/dash-evo-tool/pull/1153) — by @Claudius-Maginificent · areas: fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/dash-evo-tool#1159 fix(app): keep a previous network's background events and a request's progress where they belong](https://github.com/dashpay/dash-evo-tool/pull/1159) — by @Claudius-Maginificent · areas: fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/dash-evo-tool#1167 perf(wallet): stop re-copying wallet funding state for Max quotes and snapshots](https://github.com/dashpay/dash-evo-tool/pull/1167) — by @Claudius-Maginificent · areas: fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
- [dashpay/platform#5479 feat(platform-wallet-storage): persist DashPay backfill coverage and outbound-account markers](https://github.com/dashpay/platform/pull/5479) — by @HashEngineering · areas: rs-platform-wallet, wallet-storage · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 8e1ee2e24549cabf2d2e81a85a02fc613212aa1f
- [dashpay/platform#5536 fix: check a DashPay invitation claim against Platform before broadcasting it](https://github.com/dashpay/platform/pull/5536) — by @romchornyi · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, rust-sdk, kotlin-sdk, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
- [dashpay/platform#5780 fix(platform-wallet-storage): restore UTXOs into the account that derives them](https://github.com/dashpay/platform/pull/5780) — by @PastaPastaPasta · areas: wallet-storage · Policy: too-many-open-prs
  - Blocker: Human review is still required; automatic reviewer requests are paused until a review slot is available
- [dashpay/platform#5794 feat(platform-wallet-storage): read one asset lock's status by outpoint](https://github.com/dashpay/platform/pull/5794) — by @PastaPastaPasta · areas: wallet-storage · Policy: too-many-open-prs
  - Blocker: Human review is still required; automatic reviewer requests are paused until a review slot is available
- [dashpay/platform#5890 feat(platform-wallet): keep masternode collateral out of coin selection](https://github.com/dashpay/platform/pull/5890) — by @llbartekll · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk, fallback · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head
- [dashpay/tenderdash#1618 fix(blocksync): hand over to consensus at the tip despite unserved peer height claims](https://github.com/dashpay/tenderdash/pull/1618) — by @Claudius-Maginificent · areas: fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required

<a id="xdustinface"></a>
### @xdustinface
<a id="xdustinface-open"></a>
#### Open (11)
- [dashpay/platform#5791 fix(platform-wallet): refuse a quorum type that does not fit a byte](https://github.com/dashpay/platform/pull/5791) — 2 unresolved (1 CodeRabbit, 1 bot) · 6 days stale · ⚠ merge conflict · areas: rs-platform-wallet · Policy: waiting-author
  - Top thread: "&lt;picture&gt;&lt;img src="https://storage.googleapis.com/coderabbit_public_assets/review-comment-icons-v1/amber-16.svg" width="…" — 6 days old
  - Blocker: coderabbitai left review threads unresolved; resolve them
- [dashpay/rust-dashcore#541 ci: integrate Codecov test analytics via \`cargo-nextest\`](https://github.com/dashpay/rust-dashcore/pull/541) — ⚠ merge conflict · 📝 draft · areas: github, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#803 feat(dash-spv): block locator + staged fork detection](https://github.com/dashpay/rust-dashcore/pull/803) — ⚠ merge conflict · 📝 draft · areas: dash-spv, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#1345 fix(dash-spv): stop a shared stall from emptying the peer set](https://github.com/dashpay/rust-dashcore/pull/1345) — ⚠ merge conflict · 📝 draft · 🐢 targets fix/streaming-request-progress · areas: dash-spv
- [dashpay/rust-dashcore#1370 fix(dash-spv): settle filter requests on network arrival](https://github.com/dashpay/rust-dashcore/pull/1370) — 📝 draft · 🐢 targets fix/shared-stall-kicks · areas: dash-spv
- [dashpay/rust-dashcore#1371 fix(dash-spv): size byte caps from the served rate](https://github.com/dashpay/rust-dashcore/pull/1371) — 📝 draft · 🐢 targets fix/cfilters-settle-on-arrival · areas: dash-spv
- [dashpay/rust-dashcore#1372 fix(dash-spv): spread peers across network groups](https://github.com/dashpay/rust-dashcore/pull/1372) — 📝 draft · 🐢 targets fix/rate-sized-cap · areas: dash-spv
- [dashpay/rust-dashcore#1373 fix(dash-spv): rotate the slowest peer of a slow set](https://github.com/dashpay/rust-dashcore/pull/1373) — 📝 draft · 🐢 targets fix/peer-diversity · areas: dash-spv
- [dashpay/rust-dashcore#1374 fix(dash-spv): retry a refused request at another peer](https://github.com/dashpay/rust-dashcore/pull/1374) — 📝 draft · 🐢 targets fix/peer-exploration · areas: dash-spv
- [dashpay/rust-dashcore#1375 fix(dash-spv): rank a stalled peer last when routing](https://github.com/dashpay/rust-dashcore/pull/1375) — 📝 draft · 🐢 targets fix/peer-exploration · areas: dash-spv
- [dashpay/rust-dashcore#1388 fix(dash-spv): count partial messages as progress](https://github.com/dashpay/rust-dashcore/pull/1388) — ✋ changes requested · areas: dash-spv · Policy: waiting-author
  - Blocker: coderabbitai left review threads unresolved; resolve them

<a id="xdustinface-needs-action"></a>
#### Needs action (2)
- [dashpay/platform#5791 fix(platform-wallet): refuse a quorum type that does not fit a byte](https://github.com/dashpay/platform/pull/5791) — 2 unresolved (1 CodeRabbit, 1 bot) · 6 days stale · ⚠ merge conflict · areas: rs-platform-wallet · Policy: waiting-author
  - Top thread: "&lt;picture&gt;&lt;img src="https://storage.googleapis.com/coderabbit_public_assets/review-comment-icons-v1/amber-16.svg" width="…" — 6 days old
  - Blocker: coderabbitai left review threads unresolved; resolve them
- [dashpay/rust-dashcore#1388 fix(dash-spv): count partial messages as progress](https://github.com/dashpay/rust-dashcore/pull/1388) — ✋ changes requested · areas: dash-spv · Policy: waiting-author
  - Blocker: coderabbitai left review threads unresolved; resolve them

<a id="xdustinface-unresolved-comments"></a>
#### Unresolved Comments (1)
- [dashpay/platform#5791 fix(platform-wallet): refuse a quorum type that does not fit a byte](https://github.com/dashpay/platform/pull/5791) — 2 unresolved (1 CodeRabbit, 1 bot) · 6 days stale · ⚠ merge conflict · areas: rs-platform-wallet · Policy: waiting-author
  - Top thread: "&lt;picture&gt;&lt;img src="https://storage.googleapis.com/coderabbit_public_assets/review-comment-icons-v1/amber-16.svg" width="…" — 6 days old
  - Blocker: coderabbitai left review threads unresolved; resolve them

<a id="xdustinface-changes-requested"></a>
#### Changes Requested (1)
- [dashpay/rust-dashcore#1388 fix(dash-spv): count partial messages as progress](https://github.com/dashpay/rust-dashcore/pull/1388) — ✋ changes requested · areas: dash-spv · Policy: waiting-author
  - Blocker: coderabbitai left review threads unresolved; resolve them

<a id="xdustinface-draft"></a>
#### Draft (2)
- [dashpay/rust-dashcore#541 ci: integrate Codecov test analytics via \`cargo-nextest\`](https://github.com/dashpay/rust-dashcore/pull/541) — ⚠ merge conflict · 📝 draft · areas: github, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#803 feat(dash-spv): block locator + staged fork detection](https://github.com/dashpay/rust-dashcore/pull/803) — ⚠ merge conflict · 📝 draft · areas: dash-spv, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot

<a id="xdustinface-stale"></a>
#### Stale (7)
- [dashpay/rust-dashcore#1345 fix(dash-spv): stop a shared stall from emptying the peer set](https://github.com/dashpay/rust-dashcore/pull/1345) — ⚠ merge conflict · 📝 draft · 🐢 targets fix/streaming-request-progress · areas: dash-spv
- [dashpay/rust-dashcore#1370 fix(dash-spv): settle filter requests on network arrival](https://github.com/dashpay/rust-dashcore/pull/1370) — 📝 draft · 🐢 targets fix/shared-stall-kicks · areas: dash-spv
- [dashpay/rust-dashcore#1371 fix(dash-spv): size byte caps from the served rate](https://github.com/dashpay/rust-dashcore/pull/1371) — 📝 draft · 🐢 targets fix/cfilters-settle-on-arrival · areas: dash-spv
- [dashpay/rust-dashcore#1372 fix(dash-spv): spread peers across network groups](https://github.com/dashpay/rust-dashcore/pull/1372) — 📝 draft · 🐢 targets fix/rate-sized-cap · areas: dash-spv
- [dashpay/rust-dashcore#1373 fix(dash-spv): rotate the slowest peer of a slow set](https://github.com/dashpay/rust-dashcore/pull/1373) — 📝 draft · 🐢 targets fix/peer-diversity · areas: dash-spv
- [dashpay/rust-dashcore#1374 fix(dash-spv): retry a refused request at another peer](https://github.com/dashpay/rust-dashcore/pull/1374) — 📝 draft · 🐢 targets fix/peer-exploration · areas: dash-spv
- [dashpay/rust-dashcore#1375 fix(dash-spv): rank a stalled peer last when routing](https://github.com/dashpay/rust-dashcore/pull/1375) — 📝 draft · 🐢 targets fix/peer-exploration · areas: dash-spv

<a id="xdustinface-ready-for-review"></a>
#### Ready for Review (6)
- [dashpay/rust-dashcore#1367 test(key-wallet-manager): cover rescans over chainlocked blocks](https://github.com/dashpay/rust-dashcore/pull/1367) — by @PastaPastaPasta · areas: key-wallet, key-wallet-manager · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
- [dashpay/rust-dashcore#1378 fix(key-wallet)!: return errors from managed account constructors](https://github.com/dashpay/rust-dashcore/pull/1378) — by @ZocoLini · areas: key-wallet · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed f3111ed9f3cafff79b55eb4533cdf77cc18935d2
- [dashpay/rust-dashcore#1379 fix(dashcore): reject high-S signatures in \`verify_data_signature\`](https://github.com/dashpay/rust-dashcore/pull/1379) — by @ZocoLini · areas: fallback · Policy: waiting-bots
  - Blocker: coderabbitai has not reported for the current head
- [dashpay/rust-dashcore#1391 feat(key-wallet)!: lock masternode collateral out of coin selection](https://github.com/dashpay/rust-dashcore/pull/1391) — by @llbartekll · areas: key-wallet, key-wallet-manager, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 2cdb0d6f6bc3a402a9fee475bccd59a05b8ddd8e
- [dashpay/rust-dashcore#1392 test(dash-spv): drop a restored cycle with another entry hash](https://github.com/dashpay/rust-dashcore/pull/1392) — by @PastaPastaPasta · areas: dash-spv · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
- [dashpay/rust-dashcore#1393 fix(key-wallet): derive the BIP39 seed in wiped buffers](https://github.com/dashpay/rust-dashcore/pull/1393) — by @PastaPastaPasta · areas: key-wallet · Policy: waiting-build
  - Blocker: Waiting for the build to finish

<a id="hashengineering"></a>
### @HashEngineering
<a id="hashengineering-open"></a>
#### Open (4)
- [dashpay/platform#5767 fix(kotlin-sdk): port #4638 to Android on the wallet's TXO reconcile policy](https://github.com/dashpay/platform/pull/5767) — 3 unresolved (1 CodeRabbit, 2 bot) · 1 days stale · ✋ changes requested · 🔴 CI failing · areas: kotlin-sdk, fallback · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Add a composite index for the paged unspent scan**" — 1 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: coderabbitai left review threads unresolved; resolve them
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5479 feat(platform-wallet-storage): persist DashPay backfill coverage and outbound-account markers](https://github.com/dashpay/platform/pull/5479) — areas: rs-platform-wallet, wallet-storage · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 8e1ee2e24549cabf2d2e81a85a02fc613212aa1f
- [dashpay/platform#5830 feat(kotlin-sdk): partial-amount platform-address withdrawal](https://github.com/dashpay/platform/pull/5830) — areas: kotlin-sdk, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed a109e0fe1cdb448e7d0ca52dfb41c0bcb9c37053
- [dashpay/platform#5873 feat(kotlin-sdk): bind Platform → third-party Orchard shield (shieldedShieldToRecipient)](https://github.com/dashpay/platform/pull/5873) — areas: kotlin-sdk, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed fbeb778fc7afe70db32fc3feaa332128405f306c

<a id="hashengineering-needs-action"></a>
#### Needs action (1)
- [dashpay/platform#5767 fix(kotlin-sdk): port #4638 to Android on the wallet's TXO reconcile policy](https://github.com/dashpay/platform/pull/5767) — 3 unresolved (1 CodeRabbit, 2 bot) · 1 days stale · ✋ changes requested · 🔴 CI failing · areas: kotlin-sdk, fallback · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Add a composite index for the paged unspent scan**" — 1 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: coderabbitai left review threads unresolved; resolve them
  - Blocker: thepastaclaw left review threads unresolved; resolve them

<a id="hashengineering-unresolved-comments"></a>
#### Unresolved Comments (1)
- [dashpay/platform#5767 fix(kotlin-sdk): port #4638 to Android on the wallet's TXO reconcile policy](https://github.com/dashpay/platform/pull/5767) — 3 unresolved (1 CodeRabbit, 2 bot) · 1 days stale · ✋ changes requested · 🔴 CI failing · areas: kotlin-sdk, fallback · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Add a composite index for the paged unspent scan**" — 1 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: coderabbitai left review threads unresolved; resolve them
  - Blocker: thepastaclaw left review threads unresolved; resolve them

<a id="hashengineering-clean"></a>
#### Clean (3)
- [dashpay/platform#5479 feat(platform-wallet-storage): persist DashPay backfill coverage and outbound-account markers](https://github.com/dashpay/platform/pull/5479) — areas: rs-platform-wallet, wallet-storage · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 8e1ee2e24549cabf2d2e81a85a02fc613212aa1f
- [dashpay/platform#5830 feat(kotlin-sdk): partial-amount platform-address withdrawal](https://github.com/dashpay/platform/pull/5830) — areas: kotlin-sdk, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed a109e0fe1cdb448e7d0ca52dfb41c0bcb9c37053
- [dashpay/platform#5873 feat(kotlin-sdk): bind Platform → third-party Orchard shield (shieldedShieldToRecipient)](https://github.com/dashpay/platform/pull/5873) — areas: kotlin-sdk, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed fbeb778fc7afe70db32fc3feaa332128405f306c

<a id="hashengineering-ready-for-review"></a>
#### Ready for Review (5)
- [dashpay/platform#5277 perf(platform-wallet): don't hold the shielded store lock across sync network I/O](https://github.com/dashpay/platform/pull/5277) — by @PastaPastaPasta · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed de486366461d17ef4845e90de4961499b0b1faeb
- [dashpay/platform#5536 fix: check a DashPay invitation claim against Platform before broadcasting it](https://github.com/dashpay/platform/pull/5536) — by @romchornyi · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, rust-sdk, kotlin-sdk, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
- [dashpay/platform#5793 fix(platform-wallet): make SpvRuntime::header_hash_at public](https://github.com/dashpay/platform/pull/5793) — by @PastaPastaPasta · areas: rs-platform-wallet · Policy: too-many-open-prs
  - Blocker: Human review is still required; automatic reviewer requests are paused until a review slot is available
- [dashpay/platform#5889 feat(platform-wallet): rotate a masternode's keys into the wallet](https://github.com/dashpay/platform/pull/5889) — by @llbartekll · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head
- [dashpay/platform#5890 feat(platform-wallet): keep masternode collateral out of coin selection](https://github.com/dashpay/platform/pull/5890) — by @llbartekll · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk, fallback · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head

<a id="thepastaclaw"></a>
### @thepastaclaw
<a id="thepastaclaw-open"></a>
#### Open (2)
- [dashpay/platform#5581 feat(platform): add proof-backed asset lock outpoint query](https://github.com/dashpay/platform/pull/5581) — 2 unresolved (2 CodeRabbit) · 2 days stale · areas: rs-drive, rs-drive-abci, dpp, rs-platform-wallet, rust-sdk, rust-dapi, fallback · Policy: waiting-author
  - Top thread: "&lt;picture&gt;&lt;img src="https://storage.googleapis.com/coderabbit_public_assets/review-comment-icons-v1/amber-16.svg" width="…" — 2 days old
  - Blocker: coderabbitai left review threads unresolved; resolve them
- [dashpay/platform#4298 fix(dashmate): load ZeroSSL config in force mode](https://github.com/dashpay/platform/pull/4298) — areas: dashmate · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required

<a id="thepastaclaw-ready-for-human"></a>
#### Ready for human (1)
- [dashpay/platform#4298 fix(dashmate): load ZeroSSL config in force mode](https://github.com/dashpay/platform/pull/4298) — areas: dashmate · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required

<a id="thepastaclaw-unresolved-comments"></a>
#### Unresolved Comments (1)
- [dashpay/platform#5581 feat(platform): add proof-backed asset lock outpoint query](https://github.com/dashpay/platform/pull/5581) — 2 unresolved (2 CodeRabbit) · 2 days stale · areas: rs-drive, rs-drive-abci, dpp, rs-platform-wallet, rust-sdk, rust-dapi, fallback · Policy: waiting-author
  - Top thread: "&lt;picture&gt;&lt;img src="https://storage.googleapis.com/coderabbit_public_assets/review-comment-icons-v1/amber-16.svg" width="…" — 2 days old
  - Blocker: coderabbitai left review threads unresolved; resolve them

<a id="thepastaclaw-clean"></a>
#### Clean (1)
- [dashpay/platform#4298 fix(dashmate): load ZeroSSL config in force mode](https://github.com/dashpay/platform/pull/4298) — areas: dashmate · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required

<a id="zocolini"></a>
### @ZocoLini
<a id="zocolini-open"></a>
#### Open (7)
- [dashpay/rust-dashcore#1199 feat(key-wallet)!: only final coins fund a build](https://github.com/dashpay/rust-dashcore/pull/1199) — ⚠ merge conflict · 📝 draft · areas: dash-spv, key-wallet, key-wallet-manager · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#1321 fix(key-wallet): reject an account xpub another account already uses](https://github.com/dashpay/rust-dashcore/pull/1321) — ✋ changes requested · areas: key-wallet · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 957be3b94e776dfc99ed7795e709d082a60327b3
- [dashpay/rust-dashcore#1328 refactor(key-wallet)!: validate child indexes at construction with \`ChildIndex\`](https://github.com/dashpay/rust-dashcore/pull/1328) — ⚠ merge conflict · 📝 draft · areas: dash-spv, key-wallet, key-wallet-manager · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#1359 refactor!: detect bloom filter staleness with a hash of what the filter watches](https://github.com/dashpay/rust-dashcore/pull/1359) — ⚠ merge conflict · areas: dash-spv, key-wallet, key-wallet-manager · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 3d8ad5c60d7da6563c17e87c58225cc1bebbbbe0
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/rust-dashcore#1377 refactor(key-wallet)!: simplify and encapsulate \`AddressPool\`](https://github.com/dashpay/rust-dashcore/pull/1377) — 🔴 CI failing · areas: dash-spv, key-wallet, key-wallet-manager · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 850431fe011a428417fe9a9bea21f942a56568ca
- [dashpay/rust-dashcore#1378 fix(key-wallet)!: return errors from managed account constructors](https://github.com/dashpay/rust-dashcore/pull/1378) — areas: key-wallet · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed f3111ed9f3cafff79b55eb4533cdf77cc18935d2
- [dashpay/rust-dashcore#1379 fix(dashcore): reject high-S signatures in \`verify_data_signature\`](https://github.com/dashpay/rust-dashcore/pull/1379) — areas: fallback · Policy: waiting-bots
  - Blocker: coderabbitai has not reported for the current head

<a id="zocolini-needs-action"></a>
#### Needs action (2)
- [dashpay/rust-dashcore#1321 fix(key-wallet): reject an account xpub another account already uses](https://github.com/dashpay/rust-dashcore/pull/1321) — ✋ changes requested · areas: key-wallet · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 957be3b94e776dfc99ed7795e709d082a60327b3
- [dashpay/rust-dashcore#1359 refactor!: detect bloom filter staleness with a hash of what the filter watches](https://github.com/dashpay/rust-dashcore/pull/1359) — ⚠ merge conflict · areas: dash-spv, key-wallet, key-wallet-manager · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 3d8ad5c60d7da6563c17e87c58225cc1bebbbbe0
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return

<a id="zocolini-changes-requested"></a>
#### Changes Requested (1)
- [dashpay/rust-dashcore#1321 fix(key-wallet): reject an account xpub another account already uses](https://github.com/dashpay/rust-dashcore/pull/1321) — ✋ changes requested · areas: key-wallet · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 957be3b94e776dfc99ed7795e709d082a60327b3

<a id="zocolini-ci-failing"></a>
#### CI Failing (1)
- [dashpay/rust-dashcore#1377 refactor(key-wallet)!: simplify and encapsulate \`AddressPool\`](https://github.com/dashpay/rust-dashcore/pull/1377) — 🔴 CI failing · areas: dash-spv, key-wallet, key-wallet-manager · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 850431fe011a428417fe9a9bea21f942a56568ca

<a id="zocolini-draft"></a>
#### Draft (2)
- [dashpay/rust-dashcore#1199 feat(key-wallet)!: only final coins fund a build](https://github.com/dashpay/rust-dashcore/pull/1199) — ⚠ merge conflict · 📝 draft · areas: dash-spv, key-wallet, key-wallet-manager · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#1328 refactor(key-wallet)!: validate child indexes at construction with \`ChildIndex\`](https://github.com/dashpay/rust-dashcore/pull/1328) — ⚠ merge conflict · 📝 draft · areas: dash-spv, key-wallet, key-wallet-manager · Policy: draft
  - Blocker: Draft PR does not occupy a review slot

<a id="zocolini-clean"></a>
#### Clean (3)
- [dashpay/rust-dashcore#1359 refactor!: detect bloom filter staleness with a hash of what the filter watches](https://github.com/dashpay/rust-dashcore/pull/1359) — ⚠ merge conflict · areas: dash-spv, key-wallet, key-wallet-manager · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 3d8ad5c60d7da6563c17e87c58225cc1bebbbbe0
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/rust-dashcore#1378 fix(key-wallet)!: return errors from managed account constructors](https://github.com/dashpay/rust-dashcore/pull/1378) — areas: key-wallet · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed f3111ed9f3cafff79b55eb4533cdf77cc18935d2
- [dashpay/rust-dashcore#1379 fix(dashcore): reject high-S signatures in \`verify_data_signature\`](https://github.com/dashpay/rust-dashcore/pull/1379) — areas: fallback · Policy: waiting-bots
  - Blocker: coderabbitai has not reported for the current head

<a id="zocolini-ready-for-review"></a>
#### Ready for Review (10)
- [dashpay/platform#5277 perf(platform-wallet): don't hold the shielded store lock across sync network I/O](https://github.com/dashpay/platform/pull/5277) — by @PastaPastaPasta · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed de486366461d17ef4845e90de4961499b0b1faeb
- [dashpay/platform#5479 feat(platform-wallet-storage): persist DashPay backfill coverage and outbound-account markers](https://github.com/dashpay/platform/pull/5479) — by @HashEngineering · areas: rs-platform-wallet, wallet-storage · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 8e1ee2e24549cabf2d2e81a85a02fc613212aa1f
- [dashpay/platform#5536 fix: check a DashPay invitation claim against Platform before broadcasting it](https://github.com/dashpay/platform/pull/5536) — by @romchornyi · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, rust-sdk, kotlin-sdk, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
- [dashpay/platform#5793 fix(platform-wallet): make SpvRuntime::header_hash_at public](https://github.com/dashpay/platform/pull/5793) — by @PastaPastaPasta · areas: rs-platform-wallet · Policy: too-many-open-prs
  - Blocker: Human review is still required; automatic reviewer requests are paused until a review slot is available
- [dashpay/platform#5889 feat(platform-wallet): rotate a masternode's keys into the wallet](https://github.com/dashpay/platform/pull/5889) — by @llbartekll · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head
- [dashpay/platform#5890 feat(platform-wallet): keep masternode collateral out of coin selection](https://github.com/dashpay/platform/pull/5890) — by @llbartekll · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk, fallback · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head
- [dashpay/rust-dashcore#1367 test(key-wallet-manager): cover rescans over chainlocked blocks](https://github.com/dashpay/rust-dashcore/pull/1367) — by @PastaPastaPasta · areas: key-wallet, key-wallet-manager · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
- [dashpay/rust-dashcore#1391 feat(key-wallet)!: lock masternode collateral out of coin selection](https://github.com/dashpay/rust-dashcore/pull/1391) — by @llbartekll · areas: key-wallet, key-wallet-manager, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 2cdb0d6f6bc3a402a9fee475bccd59a05b8ddd8e
- [dashpay/rust-dashcore#1392 test(dash-spv): drop a restored cycle with another entry hash](https://github.com/dashpay/rust-dashcore/pull/1392) — by @PastaPastaPasta · areas: dash-spv · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
- [dashpay/rust-dashcore#1393 fix(key-wallet): derive the BIP39 seed in wiped buffers](https://github.com/dashpay/rust-dashcore/pull/1393) — by @PastaPastaPasta · areas: key-wallet · Policy: waiting-build
  - Blocker: Waiting for the build to finish

<a id="ktechmidas"></a>
### @ktechmidas
<a id="ktechmidas-open"></a>
#### Open (7)
- [dashpay/platform#5188 ci: tolerate optional S3 cache export outages on chore/bump-rust-dashcore-secp-033](https://github.com/dashpay/platform/pull/5188) — 1 unresolved (1 bot) · 44 days stale · 🐢 targets chore/bump-rust-dashcore-secp-033 · areas: github
  - Top thread: "🟡 Suggestion: New regression tests are not executed by any CI workflow on this branch**" — 44 days old
- [dashpay/platform#5186 ci: tolerate optional S3 cache export outages on v5.0-dev](https://github.com/dashpay/platform/pull/5186) — 🔴 CI failing · 🐢 targets v6.0-dev · areas: github · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 843052c2dce26a80065c15b1219bbc8211264161
- [dashpay/platform#5187 ci: tolerate optional S3 cache export outages on v4.3-dev](https://github.com/dashpay/platform/pull/5187) — areas: github · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed fbbf13ee41f8bd8b9f03f1dc5afec6a619967d97
- [dashpay/platform#5198 ci: bound default Binaryen optimizer threads in NPM builds](https://github.com/dashpay/platform/pull/5198) — 🐢 targets v6.0-dev · areas: github · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 23b41b477bc038965b686b4a75412b1c21fc6d98
- [dashpay/platform#5204 fix(release): install verified cargo-binstall binary directly](https://github.com/dashpay/platform/pull/5204) — ⚠ merge conflict · areas: fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 63455a2563fd7271fb11083d3b0a89bdf01f3982
- [dashpay/platform#5205 fix(release): install verified cargo-binstall binary directly](https://github.com/dashpay/platform/pull/5205) — 🐢 targets v6.0-dev · areas: fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 8a8ac2c788b5338416e18045001d9115ba3ac80a
- [dashpay/platform#5866 feat(dashmate): clean up historical Docker images after updates](https://github.com/dashpay/platform/pull/5866) — 🔴 CI failing · areas: dashmate · Policy: waiting-build
  - Blocker: The build must pass before this can merge

<a id="ktechmidas-needs-action"></a>
#### Needs action (1)
- [dashpay/platform#5204 fix(release): install verified cargo-binstall binary directly](https://github.com/dashpay/platform/pull/5204) — ⚠ merge conflict · areas: fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 63455a2563fd7271fb11083d3b0a89bdf01f3982

<a id="ktechmidas-ci-failing"></a>
#### CI Failing (1)
- [dashpay/platform#5866 feat(dashmate): clean up historical Docker images after updates](https://github.com/dashpay/platform/pull/5866) — 🔴 CI failing · areas: dashmate · Policy: waiting-build
  - Blocker: The build must pass before this can merge

<a id="ktechmidas-stale"></a>
#### Stale (4)
- [dashpay/platform#5188 ci: tolerate optional S3 cache export outages on chore/bump-rust-dashcore-secp-033](https://github.com/dashpay/platform/pull/5188) — 1 unresolved (1 bot) · 44 days stale · 🐢 targets chore/bump-rust-dashcore-secp-033 · areas: github
  - Top thread: "🟡 Suggestion: New regression tests are not executed by any CI workflow on this branch**" — 44 days old
- [dashpay/platform#5186 ci: tolerate optional S3 cache export outages on v5.0-dev](https://github.com/dashpay/platform/pull/5186) — 🔴 CI failing · 🐢 targets v6.0-dev · areas: github · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 843052c2dce26a80065c15b1219bbc8211264161
- [dashpay/platform#5198 ci: bound default Binaryen optimizer threads in NPM builds](https://github.com/dashpay/platform/pull/5198) — 🐢 targets v6.0-dev · areas: github · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 23b41b477bc038965b686b4a75412b1c21fc6d98
- [dashpay/platform#5205 fix(release): install verified cargo-binstall binary directly](https://github.com/dashpay/platform/pull/5205) — 🐢 targets v6.0-dev · areas: fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 8a8ac2c788b5338416e18045001d9115ba3ac80a

<a id="ktechmidas-clean"></a>
#### Clean (2)
- [dashpay/platform#5187 ci: tolerate optional S3 cache export outages on v4.3-dev](https://github.com/dashpay/platform/pull/5187) — areas: github · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed fbbf13ee41f8bd8b9f03f1dc5afec6a619967d97
- [dashpay/platform#5204 fix(release): install verified cargo-binstall binary directly](https://github.com/dashpay/platform/pull/5204) — ⚠ merge conflict · areas: fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 63455a2563fd7271fb11083d3b0a89bdf01f3982

<a id="ktechmidas-ready-for-review"></a>
#### Ready for Review (2)
- [dashpay/platform#4298 fix(dashmate): load ZeroSSL config in force mode](https://github.com/dashpay/platform/pull/4298) — by @thepastaclaw · areas: dashmate · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
- [dashpay/platform#5655 fix(dashmate): print the default config or group when called without an argument](https://github.com/dashpay/platform/pull/5655) — by @shumkov · areas: dashmate · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 105c7318e6e3cb846caeadaa8d87731b3142c2cf

<a id="llbartekll"></a>
### @llbartekll
<a id="llbartekll-open"></a>
#### Open (3)
- [dashpay/platform#5889 feat(platform-wallet): rotate a masternode's keys into the wallet](https://github.com/dashpay/platform/pull/5889) — areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head
- [dashpay/platform#5890 feat(platform-wallet): keep masternode collateral out of coin selection](https://github.com/dashpay/platform/pull/5890) — areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk, fallback · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head
- [dashpay/rust-dashcore#1391 feat(key-wallet)!: lock masternode collateral out of coin selection](https://github.com/dashpay/rust-dashcore/pull/1391) — areas: key-wallet, key-wallet-manager, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 2cdb0d6f6bc3a402a9fee475bccd59a05b8ddd8e

<a id="llbartekll-clean"></a>
#### Clean (3)
- [dashpay/platform#5889 feat(platform-wallet): rotate a masternode's keys into the wallet](https://github.com/dashpay/platform/pull/5889) — areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head
- [dashpay/platform#5890 feat(platform-wallet): keep masternode collateral out of coin selection](https://github.com/dashpay/platform/pull/5890) — areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk, fallback · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head
- [dashpay/rust-dashcore#1391 feat(key-wallet)!: lock masternode collateral out of coin selection](https://github.com/dashpay/rust-dashcore/pull/1391) — areas: key-wallet, key-wallet-manager, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 2cdb0d6f6bc3a402a9fee475bccd59a05b8ddd8e

<a id="llbartekll-ready-for-review"></a>
#### Ready for Review (3)
- [dashpay/platform#4992 feat(swift-sdk): durable wallet presence marker readable while the device is locked](https://github.com/dashpay/platform/pull/4992) — by @romchornyi · areas: swift-sdk · Policy: ready-to-merge
  - Blocker: All policy requirements are satisfied
- [dashpay/platform#5277 perf(platform-wallet): don't hold the shielded store lock across sync network I/O](https://github.com/dashpay/platform/pull/5277) — by @PastaPastaPasta · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed de486366461d17ef4845e90de4961499b0b1faeb
- [dashpay/platform#5793 fix(platform-wallet): make SpvRuntime::header_hash_at public](https://github.com/dashpay/platform/pull/5793) — by @PastaPastaPasta · areas: rs-platform-wallet · Policy: too-many-open-prs
  - Blocker: Human review is still required; automatic reviewer requests are paused until a review slot is available

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
