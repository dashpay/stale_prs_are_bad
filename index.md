---
---
# PR Hygiene Report
*Last updated: 2026-10-29 18:14 UTC · commit 0009053*

## Summary
- Open PRs: **182** (16 clean · 22 CI failing · 9 changes requested · 33 unresolved comments · 8 deferred · 39 draft · 55 stale)
- PRs needing author action: **49**
- Total unresolved comments: **171**
- dashpay/platform: **126** open (9 clean · 20 CI failing · 3 changes requested · 21 unresolved comments · 8 deferred · 23 draft · 42 stale) · engine: 25 draft · 18 ready-for-human · 3 ready-to-merge · 22 waiting-author · 4 waiting-bots · 8 waiting-build · 19 waiting-self-review · 27 no verdict
- dashpay/rust-dashcore: **21** open (1 clean · 2 CI failing · 3 changes requested · 5 unresolved comments · 0 deferred · 7 draft · 3 stale) · engine: 8 draft · 1 ready-for-human · 3 waiting-author · 7 waiting-self-review · 2 no verdict
- dashpay/tenderdash: **7** open (2 clean · 0 CI failing · 0 changes requested · 3 unresolved comments · 0 deferred · 2 draft · 0 stale) · engine: 2 draft · 3 waiting-author · 2 waiting-self-review
- dashpay/grovedb: **3** open (1 clean · 0 CI failing · 0 changes requested · 1 unresolved comments · 0 deferred · 1 draft · 0 stale) · engine: 1 draft · 1 waiting-author · 1 waiting-self-review
- dashpay/dash-evo-tool: **25** open (3 clean · 0 CI failing · 3 changes requested · 3 unresolved comments · 0 deferred · 6 draft · 10 stale) · engine: 6 draft · 2 waiting-author · 7 waiting-bots · 10 no verdict

## Scoreboard
_Sort: unresolved-comments desc → needs-action desc → ready-for-review desc. Click any number to jump to the specific PRs it covers._

| Author | Open | Clean | CI failing | Unresolved Comments | Changes Requested | Deferred | Draft | Stale | Needs action | Ready for human | Total Unresolved Comments | Ready for Review | Δ |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| [@shumkov](#shumkov) | [24](#shumkov-open) | [1](#shumkov-clean) | [3](#shumkov-ci-failing) | [8](#shumkov-unresolved-comments) | [4](#shumkov-changes-requested) | — | — | [8](#shumkov-stale) | [13](#shumkov-needs-action) | [2](#shumkov-ready-for-human) | [37](#shumkov-unresolved-comments) | [8](#shumkov-ready-for-review) | ↑ 12 |
| [@Claudius-Maginificent](#claudius-maginificent) | [41](#claudius-maginificent-open) | [4](#claudius-maginificent-clean) | [3](#claudius-maginificent-ci-failing) | [6](#claudius-maginificent-unresolved-comments) | [3](#claudius-maginificent-changes-requested) | [3](#claudius-maginificent-deferred) | [6](#claudius-maginificent-draft) | [16](#claudius-maginificent-stale) | [9](#claudius-maginificent-needs-action) | [3](#claudius-maginificent-ready-for-human) | [34](#claudius-maginificent-unresolved-comments) | — | ↑ 6 |
| [@QuantumExplorer](#quantumexplorer) | [22](#quantumexplorer-open) | [3](#quantumexplorer-clean) | [2](#quantumexplorer-ci-failing) | [6](#quantumexplorer-unresolved-comments) | — | — | [5](#quantumexplorer-draft) | [6](#quantumexplorer-stale) | [8](#quantumexplorer-needs-action) | — | [36](#quantumexplorer-unresolved-comments) | [6](#quantumexplorer-ready-for-review) | ↑ 4 |
| [@PastaPastaPasta](#pastapastapasta) | [20](#pastapastapasta-open) | [2](#pastapastapasta-clean) | [2](#pastapastapasta-ci-failing) | [3](#pastapastapasta-unresolved-comments) | [1](#pastapastapasta-changes-requested) | — | [10](#pastapastapasta-draft) | [2](#pastapastapasta-stale) | [5](#pastapastapasta-needs-action) | — | [21](#pastapastapasta-unresolved-comments) | [1](#pastapastapasta-ready-for-review) | ↑ 4 |
| [@romchornyi](#romchornyi) | [12](#romchornyi-open) | — | [2](#romchornyi-ci-failing) | [2](#romchornyi-unresolved-comments) | [1](#romchornyi-changes-requested) | — | [5](#romchornyi-draft) | [2](#romchornyi-stale) | [3](#romchornyi-needs-action) | [2](#romchornyi-ready-for-human) | [16](#romchornyi-unresolved-comments) | [3](#romchornyi-ready-for-review) | ↑ 2 |
| [@lklimek](#lklimek) | [14](#lklimek-open) | [1](#lklimek-clean) | — | [2](#lklimek-unresolved-comments) | — | [5](#lklimek-deferred) | [5](#lklimek-draft) | [1](#lklimek-stale) | [2](#lklimek-needs-action) | — | [9](#lklimek-unresolved-comments) | [6](#lklimek-ready-for-review) | ↓ 1 |
| [@bfoss765](#bfoss765) | [2](#bfoss765-open) | — | — | [2](#bfoss765-unresolved-comments) | — | — | — | — | [2](#bfoss765-needs-action) | — | [3](#bfoss765-unresolved-comments) | — | ↑ 2 |
| [@infraclaw-dash](#infraclaw-dash) | [3](#infraclaw-dash-open) | — | [1](#infraclaw-dash-ci-failing) | [1](#infraclaw-dash-unresolved-comments) | — | — | — | [1](#infraclaw-dash-stale) | [2](#infraclaw-dash-needs-action) | [1](#infraclaw-dash-ready-for-human) | [5](#infraclaw-dash-unresolved-comments) | — | ↑ 2 |
| [@HashEngineering](#hashengineering) | [3](#hashengineering-open) | — | [1](#hashengineering-ci-failing) | [1](#hashengineering-unresolved-comments) | — | — | [1](#hashengineering-draft) | — | [1](#hashengineering-needs-action) | [1](#hashengineering-ready-for-human) | [2](#hashengineering-unresolved-comments) | [3](#hashengineering-ready-for-review) | ↑ 1 |
| [@llbartekll](#llbartekll) | [4](#llbartekll-open) | [2](#llbartekll-clean) | — | [1](#llbartekll-unresolved-comments) | — | — | — | [1](#llbartekll-stale) | [1](#llbartekll-needs-action) | — | [6](#llbartekll-unresolved-comments) | [2](#llbartekll-ready-for-review) | ↓ 1 |
| [@xdustinface](#xdustinface) | [5](#xdustinface-open) | — | — | [1](#xdustinface-unresolved-comments) | — | — | [3](#xdustinface-draft) | [1](#xdustinface-stale) | [1](#xdustinface-needs-action) | — | [1](#xdustinface-unresolved-comments) | [1](#xdustinface-ready-for-review) | — |
| [@ZocoLini](#zocolini) | [5](#zocolini-open) | — | [1](#zocolini-ci-failing) | — | — | — | [4](#zocolini-draft) | — | [1](#zocolini-needs-action) | — | — | [4](#zocolini-ready-for-review) | ↓ 2 |
| [@ktechmidas](#ktechmidas) | [6](#ktechmidas-open) | — | [2](#ktechmidas-ci-failing) | — | — | — | — | [4](#ktechmidas-stale) | [1](#ktechmidas-needs-action) | — | [1](#ktechmidas-unresolved-comments) | [3](#ktechmidas-ready-for-review) | ↑ 1 |
| [@DCG-Claude](#dcg-claude) | [19](#dcg-claude-open) | [2](#dcg-claude-clean) | [4](#dcg-claude-ci-failing) | — | — | — | — | [13](#dcg-claude-stale) | — | [12](#dcg-claude-ready-for-human) | — | — | — |
| [@thepastaclaw](#thepastaclaw) | [2](#thepastaclaw-open) | [1](#thepastaclaw-clean) | [1](#thepastaclaw-ci-failing) | — | — | — | — | — | — | [1](#thepastaclaw-ready-for-human) | — | — | — |

## Per-author detail

<a id="shumkov"></a>
### @shumkov
<a id="shumkov-open"></a>
#### Open (24)
- [dashpay/rust-dashcore#634 feat(key-wallet-manager): single-map WalletManager with Arc&lt;RwLock&lt;T&gt;&gt; per wallet](https://github.com/dashpay/rust-dashcore/pull/634) — 20 unresolved (20 CodeRabbit) · 204 days stale · ⚠ merge conflict · ✋ changes requested · areas: dash-spv, key-wallet, key-wallet-manager, fallback · Policy: waiting-author
  - Top thread: "_⚠️ Potential issue_ \| _🟡 Minor_" — 204 days old
  - Blocker: coderabbitai requested changes on this head; dismiss the review or push a fix
  - Blocker: coderabbitai left review threads unresolved; resolve them
- [dashpay/platform#4660 feat(sdk): expose the document erase and history lifecycle on mobile and FFI](https://github.com/dashpay/platform/pull/4660) — 2 unresolved (2 bot) · 37 days stale · ✋ changes requested · 🔴 CI failing · 🐢 targets keep-history-lifecycle · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, rust-sdk-ffi, kotlin-sdk, fallback
  - Top thread: "🟡 Suggestion: Validate history selector discriminant at the C boundary**" — 37 days old
- [dashpay/platform#4657 feat(platform)!: delete and erase lifecycle for keep-history documents](https://github.com/dashpay/platform/pull/4657) — 2 unresolved (2 bot) · 37 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · 🐢 targets keep-history-storage-v2 · areas: rs-drive, rs-drive-abci, dpp, rust-sdk, js-wasm-sdk, fallback
  - Top thread: "🟡 Suggestion: Deduplicate lifecycle containers at the Drive batch boundary**" — 37 days old
- [dashpay/platform#5638 fix(sdk): check document windows and ranked indexes before sending](https://github.com/dashpay/platform/pull/5638) — 3 unresolved (3 bot) · 0 days stale · ✋ changes requested · 🔴 CI failing · areas: rs-drive, rs-drive-abci, rust-sdk, rust-sdk-ffi, js-wasm-sdk, github, fallback · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Keep the feature test inventory exhaustive by construction**" — 0 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/grovedb#1025 feat(query)!: page count trees by a branch-local offset in merged proofs (GROVE_V5)](https://github.com/dashpay/grovedb/pull/1025) — 2 unresolved (2 bot) · 2 days stale · 🔴 CI failing · areas: fallback · Policy: waiting-author
  - Top thread: "✅ **Resolved** at \`519277b7\`; see the replies below." — 2 days old
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#4652 feat(drive)!: preserve composite document history across protocol activation](https://github.com/dashpay/platform/pull/4652) — 1 unresolved (1 bot) · 8 days stale · ⚠ merge conflict · ✋ changes requested · areas: rs-drive, rs-drive-abci, rust-sdk, js-wasm-sdk, fallback · Policy: waiting-author
  - Top thread: "🔴 Blocking: Bound or stage the attacker-inflatable activation migration**" — 8 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5634 feat(drive-abci)!: emit version 2 asset unlocks once Core V24 is active (PV15)](https://github.com/dashpay/platform/pull/5634) — 2 unresolved (2 bot) · 1 days stale · ⚠ merge conflict · 🔴 CI failing · areas: rs-drive-abci, fallback · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Track or remove the new Core RPC dependency on every PV15 pooling block**" — 1 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5656 fix(dashmate): build local images for every group node that builds any](https://github.com/dashpay/platform/pull/5656) — 2 unresolved (2 bot) · 0 days stale · ✋ changes requested · 🐢 targets fix/dashmate/update-path-bugs · areas: dashmate
  - Top thread: "🔴 Blocking: Include base-only builds in the selected group configurations**" — 0 days old
- [dashpay/platform#4283 fix(dashmate): keep the node up when an image pull fails, and stop reporting success when it did not](https://github.com/dashpay/platform/pull/4283) — 1 unresolved (1 CodeRabbit) · 0 days stale · ✋ changes requested · 🔴 CI failing · areas: dashmate · Policy: waiting-author
  - Top thread: "&lt;picture&gt;&lt;img src="https://storage.googleapis.com/coderabbit_public_assets/review-comment-icons-v1/major-16.svg" width="…" — 0 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: coderabbitai left review threads unresolved; resolve them
- [dashpay/platform#5654 fix(dashmate): restrict a gateway TLS private key an older version left readable](https://github.com/dashpay/platform/pull/5654) — 1 unresolved (1 bot) · 0 days stale · ✋ changes requested · 🔴 CI failing · areas: dashmate · Policy: waiting-author
  - Top thread: "🔴 Blocking: Windows startup hardening does not restrict ACL-readable private keys**" — 0 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5655 fix(dashmate): print the default config or group when called without an argument](https://github.com/dashpay/platform/pull/5655) — 1 unresolved (1 bot) · 0 days stale · 🔴 CI failing · areas: dashmate · Policy: waiting-self-review
  - Top thread: "🟡 Suggestion: No-argument invocation prints "null" when no default is configured**" — 0 days old
  - Blocker: Author must post /self-reviewed 894c2b2dc460a32c5a19b1eadb42a88a7f176879
- [dashpay/platform#4282 fix(dashmate)!: give Debian packages versions apt can order](https://github.com/dashpay/platform/pull/4282) — 🔴 CI failing · areas: dashmate, github, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
- [dashpay/platform#5615 fix(sdk): refuse a non-unique identity V1 request before protocol 14 is proven](https://github.com/dashpay/platform/pull/5615) — 🔴 CI failing · 🐢 targets v5.0-dev · areas: rust-sdk, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed f585aa6f47ad383ac688076548af6fe427828203
- [dashpay/platform#5628 fix(sdk): read the proved vote of a masternode vote that already exists](https://github.com/dashpay/platform/pull/5628) — ✋ changes requested · 🔴 CI failing · 🐢 targets v5.0-dev · areas: rs-drive-abci, fallback · Policy: waiting-author
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
- [dashpay/platform#5629 fix(platform-wallet): refuse ports Dash Core v24 refuses before building a version-3 ProUpServTx](https://github.com/dashpay/platform/pull/5629) — 🔴 CI failing · 🐢 targets v5.0-dev · areas: rs-platform-wallet · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 8ca225b3c77221df0e1d182182564012e9d72ebc
- [dashpay/platform#5630 fix(drive-abci): make zero quorum windows and active-signer counts unrepresentable](https://github.com/dashpay/platform/pull/5630) — 🔴 CI failing · 🐢 targets v5.0-dev · areas: rs-drive-abci · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed f3fe4ce3e3e8848b539a9f778e3e8439ca9f7374
- [dashpay/platform#5635 fix(sdk)!: refuse document requests no node can answer instead of banning nodes](https://github.com/dashpay/platform/pull/5635) — 🔴 CI failing · areas: rs-drive, rs-drive-abci, rust-sdk, rust-sdk-ffi, js-wasm-sdk, github, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 771e06ec7fc942639e36774aa152c783c8cf0f41
- [dashpay/platform#5636 feat(sdk)!: decide from the version tables which networks serve a document query](https://github.com/dashpay/platform/pull/5636) — ✋ changes requested · areas: rs-drive, rs-drive-abci, rust-sdk, rust-sdk-ffi, js-wasm-sdk, github, fallback · Policy: waiting-author
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
- [dashpay/platform#5637 chore(platform)!: update rust-dashcore to dev 7329126e](https://github.com/dashpay/platform/pull/5637) — ⚠ merge conflict · 🔴 CI failing · areas: dpp, rs-platform-wallet, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed a637fbd49425ad577331b346527ea95a2f72d955
- [dashpay/platform#5653 fix(dashmate): stop the GitHub release lookup throwing and validate its version](https://github.com/dashpay/platform/pull/5653) — areas: dashmate · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed dc05e4eb7ae997febcb7b1bc9f06625b1504b6b4
- [dashpay/rust-dashcore#1205 fix(dash-spv)!: keep masternode sync live after refused responses](https://github.com/dashpay/rust-dashcore/pull/1205) — ⚠ merge conflict · ✋ changes requested · areas: dash-spv · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
- [dashpay/rust-dashcore#1206 feat(dash-spv)!: check masternode list diffs against the block coinbase](https://github.com/dashpay/rust-dashcore/pull/1206) — 📝 draft · 🐢 targets shumkov/masternode-sync-liveness · areas: dash-spv, fallback
- [dashpay/rust-dashcore#1212 feat(dash-spv)!: let a network request avoid given peers](https://github.com/dashpay/rust-dashcore/pull/1212) — ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · areas: dash-spv · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 1c67e9ad99aa3b92f537285355aa157dfd5ee52a
- [dashpay/rust-dashcore#1226 feat(dash): check address maps against Core v24 port and duplicate rules](https://github.com/dashpay/rust-dashcore/pull/1226) — ⚠ merge conflict · ✋ changes requested · areas: fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 05b08aa8c2fa3aa44a3235ffeec9de7fb3f0324b

<a id="shumkov-needs-action"></a>
#### Needs action (13)
- [dashpay/rust-dashcore#634 feat(key-wallet-manager): single-map WalletManager with Arc&lt;RwLock&lt;T&gt;&gt; per wallet](https://github.com/dashpay/rust-dashcore/pull/634) — 20 unresolved (20 CodeRabbit) · 204 days stale · ⚠ merge conflict · ✋ changes requested · areas: dash-spv, key-wallet, key-wallet-manager, fallback · Policy: waiting-author
  - Top thread: "_⚠️ Potential issue_ \| _🟡 Minor_" — 204 days old
  - Blocker: coderabbitai requested changes on this head; dismiss the review or push a fix
  - Blocker: coderabbitai left review threads unresolved; resolve them
- [dashpay/platform#5638 fix(sdk): check document windows and ranked indexes before sending](https://github.com/dashpay/platform/pull/5638) — 3 unresolved (3 bot) · 0 days stale · ✋ changes requested · 🔴 CI failing · areas: rs-drive, rs-drive-abci, rust-sdk, rust-sdk-ffi, js-wasm-sdk, github, fallback · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Keep the feature test inventory exhaustive by construction**" — 0 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/grovedb#1025 feat(query)!: page count trees by a branch-local offset in merged proofs (GROVE_V5)](https://github.com/dashpay/grovedb/pull/1025) — 2 unresolved (2 bot) · 2 days stale · 🔴 CI failing · areas: fallback · Policy: waiting-author
  - Top thread: "✅ **Resolved** at \`519277b7\`; see the replies below." — 2 days old
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#4652 feat(drive)!: preserve composite document history across protocol activation](https://github.com/dashpay/platform/pull/4652) — 1 unresolved (1 bot) · 8 days stale · ⚠ merge conflict · ✋ changes requested · areas: rs-drive, rs-drive-abci, rust-sdk, js-wasm-sdk, fallback · Policy: waiting-author
  - Top thread: "🔴 Blocking: Bound or stage the attacker-inflatable activation migration**" — 8 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5634 feat(drive-abci)!: emit version 2 asset unlocks once Core V24 is active (PV15)](https://github.com/dashpay/platform/pull/5634) — 2 unresolved (2 bot) · 1 days stale · ⚠ merge conflict · 🔴 CI failing · areas: rs-drive-abci, fallback · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Track or remove the new Core RPC dependency on every PV15 pooling block**" — 1 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#4283 fix(dashmate): keep the node up when an image pull fails, and stop reporting success when it did not](https://github.com/dashpay/platform/pull/4283) — 1 unresolved (1 CodeRabbit) · 0 days stale · ✋ changes requested · 🔴 CI failing · areas: dashmate · Policy: waiting-author
  - Top thread: "&lt;picture&gt;&lt;img src="https://storage.googleapis.com/coderabbit_public_assets/review-comment-icons-v1/major-16.svg" width="…" — 0 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: coderabbitai left review threads unresolved; resolve them
- [dashpay/platform#5654 fix(dashmate): restrict a gateway TLS private key an older version left readable](https://github.com/dashpay/platform/pull/5654) — 1 unresolved (1 bot) · 0 days stale · ✋ changes requested · 🔴 CI failing · areas: dashmate · Policy: waiting-author
  - Top thread: "🔴 Blocking: Windows startup hardening does not restrict ACL-readable private keys**" — 0 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5655 fix(dashmate): print the default config or group when called without an argument](https://github.com/dashpay/platform/pull/5655) — 1 unresolved (1 bot) · 0 days stale · 🔴 CI failing · areas: dashmate · Policy: waiting-self-review
  - Top thread: "🟡 Suggestion: No-argument invocation prints "null" when no default is configured**" — 0 days old
  - Blocker: Author must post /self-reviewed 894c2b2dc460a32c5a19b1eadb42a88a7f176879
- [dashpay/platform#5636 feat(sdk)!: decide from the version tables which networks serve a document query](https://github.com/dashpay/platform/pull/5636) — ✋ changes requested · areas: rs-drive, rs-drive-abci, rust-sdk, rust-sdk-ffi, js-wasm-sdk, github, fallback · Policy: waiting-author
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
- [dashpay/platform#5637 chore(platform)!: update rust-dashcore to dev 7329126e](https://github.com/dashpay/platform/pull/5637) — ⚠ merge conflict · 🔴 CI failing · areas: dpp, rs-platform-wallet, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed a637fbd49425ad577331b346527ea95a2f72d955
- [dashpay/rust-dashcore#1205 fix(dash-spv)!: keep masternode sync live after refused responses](https://github.com/dashpay/rust-dashcore/pull/1205) — ⚠ merge conflict · ✋ changes requested · areas: dash-spv · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
- [dashpay/rust-dashcore#1212 feat(dash-spv)!: let a network request avoid given peers](https://github.com/dashpay/rust-dashcore/pull/1212) — ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · areas: dash-spv · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 1c67e9ad99aa3b92f537285355aa157dfd5ee52a
- [dashpay/rust-dashcore#1226 feat(dash): check address maps against Core v24 port and duplicate rules](https://github.com/dashpay/rust-dashcore/pull/1226) — ⚠ merge conflict · ✋ changes requested · areas: fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 05b08aa8c2fa3aa44a3235ffeec9de7fb3f0324b

<a id="shumkov-ready-for-human"></a>
#### Ready for human (2)
- [dashpay/platform#4282 fix(dashmate)!: give Debian packages versions apt can order](https://github.com/dashpay/platform/pull/4282) — 🔴 CI failing · areas: dashmate, github, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
- [dashpay/rust-dashcore#1205 fix(dash-spv)!: keep masternode sync live after refused responses](https://github.com/dashpay/rust-dashcore/pull/1205) — ⚠ merge conflict · ✋ changes requested · areas: dash-spv · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required

<a id="shumkov-unresolved-comments"></a>
#### Unresolved Comments (8)
- [dashpay/rust-dashcore#634 feat(key-wallet-manager): single-map WalletManager with Arc&lt;RwLock&lt;T&gt;&gt; per wallet](https://github.com/dashpay/rust-dashcore/pull/634) — 20 unresolved (20 CodeRabbit) · 204 days stale · ⚠ merge conflict · ✋ changes requested · areas: dash-spv, key-wallet, key-wallet-manager, fallback · Policy: waiting-author
  - Top thread: "_⚠️ Potential issue_ \| _🟡 Minor_" — 204 days old
  - Blocker: coderabbitai requested changes on this head; dismiss the review or push a fix
  - Blocker: coderabbitai left review threads unresolved; resolve them
- [dashpay/platform#5638 fix(sdk): check document windows and ranked indexes before sending](https://github.com/dashpay/platform/pull/5638) — 3 unresolved (3 bot) · 0 days stale · ✋ changes requested · 🔴 CI failing · areas: rs-drive, rs-drive-abci, rust-sdk, rust-sdk-ffi, js-wasm-sdk, github, fallback · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Keep the feature test inventory exhaustive by construction**" — 0 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/grovedb#1025 feat(query)!: page count trees by a branch-local offset in merged proofs (GROVE_V5)](https://github.com/dashpay/grovedb/pull/1025) — 2 unresolved (2 bot) · 2 days stale · 🔴 CI failing · areas: fallback · Policy: waiting-author
  - Top thread: "✅ **Resolved** at \`519277b7\`; see the replies below." — 2 days old
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#4652 feat(drive)!: preserve composite document history across protocol activation](https://github.com/dashpay/platform/pull/4652) — 1 unresolved (1 bot) · 8 days stale · ⚠ merge conflict · ✋ changes requested · areas: rs-drive, rs-drive-abci, rust-sdk, js-wasm-sdk, fallback · Policy: waiting-author
  - Top thread: "🔴 Blocking: Bound or stage the attacker-inflatable activation migration**" — 8 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5634 feat(drive-abci)!: emit version 2 asset unlocks once Core V24 is active (PV15)](https://github.com/dashpay/platform/pull/5634) — 2 unresolved (2 bot) · 1 days stale · ⚠ merge conflict · 🔴 CI failing · areas: rs-drive-abci, fallback · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Track or remove the new Core RPC dependency on every PV15 pooling block**" — 1 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#4283 fix(dashmate): keep the node up when an image pull fails, and stop reporting success when it did not](https://github.com/dashpay/platform/pull/4283) — 1 unresolved (1 CodeRabbit) · 0 days stale · ✋ changes requested · 🔴 CI failing · areas: dashmate · Policy: waiting-author
  - Top thread: "&lt;picture&gt;&lt;img src="https://storage.googleapis.com/coderabbit_public_assets/review-comment-icons-v1/major-16.svg" width="…" — 0 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: coderabbitai left review threads unresolved; resolve them
- [dashpay/platform#5654 fix(dashmate): restrict a gateway TLS private key an older version left readable](https://github.com/dashpay/platform/pull/5654) — 1 unresolved (1 bot) · 0 days stale · ✋ changes requested · 🔴 CI failing · areas: dashmate · Policy: waiting-author
  - Top thread: "🔴 Blocking: Windows startup hardening does not restrict ACL-readable private keys**" — 0 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5655 fix(dashmate): print the default config or group when called without an argument](https://github.com/dashpay/platform/pull/5655) — 1 unresolved (1 bot) · 0 days stale · 🔴 CI failing · areas: dashmate · Policy: waiting-self-review
  - Top thread: "🟡 Suggestion: No-argument invocation prints "null" when no default is configured**" — 0 days old
  - Blocker: Author must post /self-reviewed 894c2b2dc460a32c5a19b1eadb42a88a7f176879

<a id="shumkov-changes-requested"></a>
#### Changes Requested (4)
- [dashpay/platform#5636 feat(sdk)!: decide from the version tables which networks serve a document query](https://github.com/dashpay/platform/pull/5636) — ✋ changes requested · areas: rs-drive, rs-drive-abci, rust-sdk, rust-sdk-ffi, js-wasm-sdk, github, fallback · Policy: waiting-author
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
- [dashpay/rust-dashcore#1205 fix(dash-spv)!: keep masternode sync live after refused responses](https://github.com/dashpay/rust-dashcore/pull/1205) — ⚠ merge conflict · ✋ changes requested · areas: dash-spv · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
- [dashpay/rust-dashcore#1212 feat(dash-spv)!: let a network request avoid given peers](https://github.com/dashpay/rust-dashcore/pull/1212) — ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · areas: dash-spv · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 1c67e9ad99aa3b92f537285355aa157dfd5ee52a
- [dashpay/rust-dashcore#1226 feat(dash): check address maps against Core v24 port and duplicate rules](https://github.com/dashpay/rust-dashcore/pull/1226) — ⚠ merge conflict · ✋ changes requested · areas: fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 05b08aa8c2fa3aa44a3235ffeec9de7fb3f0324b

<a id="shumkov-ci-failing"></a>
#### CI Failing (3)
- [dashpay/platform#4282 fix(dashmate)!: give Debian packages versions apt can order](https://github.com/dashpay/platform/pull/4282) — 🔴 CI failing · areas: dashmate, github, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
- [dashpay/platform#5635 fix(sdk)!: refuse document requests no node can answer instead of banning nodes](https://github.com/dashpay/platform/pull/5635) — 🔴 CI failing · areas: rs-drive, rs-drive-abci, rust-sdk, rust-sdk-ffi, js-wasm-sdk, github, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 771e06ec7fc942639e36774aa152c783c8cf0f41
- [dashpay/platform#5637 chore(platform)!: update rust-dashcore to dev 7329126e](https://github.com/dashpay/platform/pull/5637) — ⚠ merge conflict · 🔴 CI failing · areas: dpp, rs-platform-wallet, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed a637fbd49425ad577331b346527ea95a2f72d955

<a id="shumkov-stale"></a>
#### Stale (8)
- [dashpay/platform#4660 feat(sdk): expose the document erase and history lifecycle on mobile and FFI](https://github.com/dashpay/platform/pull/4660) — 2 unresolved (2 bot) · 37 days stale · ✋ changes requested · 🔴 CI failing · 🐢 targets keep-history-lifecycle · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, rust-sdk-ffi, kotlin-sdk, fallback
  - Top thread: "🟡 Suggestion: Validate history selector discriminant at the C boundary**" — 37 days old
- [dashpay/platform#4657 feat(platform)!: delete and erase lifecycle for keep-history documents](https://github.com/dashpay/platform/pull/4657) — 2 unresolved (2 bot) · 37 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · 🐢 targets keep-history-storage-v2 · areas: rs-drive, rs-drive-abci, dpp, rust-sdk, js-wasm-sdk, fallback
  - Top thread: "🟡 Suggestion: Deduplicate lifecycle containers at the Drive batch boundary**" — 37 days old
- [dashpay/platform#5656 fix(dashmate): build local images for every group node that builds any](https://github.com/dashpay/platform/pull/5656) — 2 unresolved (2 bot) · 0 days stale · ✋ changes requested · 🐢 targets fix/dashmate/update-path-bugs · areas: dashmate
  - Top thread: "🔴 Blocking: Include base-only builds in the selected group configurations**" — 0 days old
- [dashpay/platform#5615 fix(sdk): refuse a non-unique identity V1 request before protocol 14 is proven](https://github.com/dashpay/platform/pull/5615) — 🔴 CI failing · 🐢 targets v5.0-dev · areas: rust-sdk, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed f585aa6f47ad383ac688076548af6fe427828203
- [dashpay/platform#5628 fix(sdk): read the proved vote of a masternode vote that already exists](https://github.com/dashpay/platform/pull/5628) — ✋ changes requested · 🔴 CI failing · 🐢 targets v5.0-dev · areas: rs-drive-abci, fallback · Policy: waiting-author
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
- [dashpay/platform#5629 fix(platform-wallet): refuse ports Dash Core v24 refuses before building a version-3 ProUpServTx](https://github.com/dashpay/platform/pull/5629) — 🔴 CI failing · 🐢 targets v5.0-dev · areas: rs-platform-wallet · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 8ca225b3c77221df0e1d182182564012e9d72ebc
- [dashpay/platform#5630 fix(drive-abci): make zero quorum windows and active-signer counts unrepresentable](https://github.com/dashpay/platform/pull/5630) — 🔴 CI failing · 🐢 targets v5.0-dev · areas: rs-drive-abci · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed f3fe4ce3e3e8848b539a9f778e3e8439ca9f7374
- [dashpay/rust-dashcore#1206 feat(dash-spv)!: check masternode list diffs against the block coinbase](https://github.com/dashpay/rust-dashcore/pull/1206) — 📝 draft · 🐢 targets shumkov/masternode-sync-liveness · areas: dash-spv, fallback

<a id="shumkov-clean"></a>
#### Clean (1)
- [dashpay/platform#5653 fix(dashmate): stop the GitHub release lookup throwing and validate its version](https://github.com/dashpay/platform/pull/5653) — areas: dashmate · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed dc05e4eb7ae997febcb7b1bc9f06625b1504b6b4

<a id="shumkov-ready-for-review"></a>
#### Ready for Review (8)
- [dashpay/platform#4298 fix(dashmate): load ZeroSSL config in force mode](https://github.com/dashpay/platform/pull/4298) — by @thepastaclaw · areas: dashmate · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
- [dashpay/platform#4703 fix(platform): resolve fee versions by registered number and price refunds at the storage epoch rate](https://github.com/dashpay/platform/pull/4703) — by @DCG-Claude · areas: rs-drive, rs-drive-abci, dpp, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
- [dashpay/platform#4706 feat(platform)!: require fee history for storage refunds and credit their recorded owners](https://github.com/dashpay/platform/pull/4706) — by @DCG-Claude · areas: rs-drive, rs-drive-abci, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
- [dashpay/platform#5288 feat(platform-wallet)!: support DashPay shielded tips with dedicated accounts](https://github.com/dashpay/platform/pull/5288) — by @PastaPastaPasta · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk, rust-sdk, kotlin-sdk, fallback · Policy: waiting-author
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5644 fix!: follow DIP-11 key rules when disabling keys and reading contact requests](https://github.com/dashpay/platform/pull/5644) — by @Claudius-Maginificent · areas: dpp, rs-platform-wallet, rs-platform-wallet-ffi, rust-sdk, js-wasm-sdk, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
- [dashpay/platform#5668 fix(sdk): do not ban nodes when the SDK has no context provider](https://github.com/dashpay/platform/pull/5668) — by @llbartekll · areas: rust-sdk · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head
- [dashpay/tenderdash#1560 test(e2e): exercise block sync during slow local application](https://github.com/dashpay/tenderdash/pull/1560) — by @lklimek · areas: github, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 1f5fb2540ca89b3c6b2bb311ad149affdd577df4
- [dashpay/tenderdash#1572 feat(consensus)!: hand a silent proposer's idle round to the next proposer](https://github.com/dashpay/tenderdash/pull/1572) — by @QuantumExplorer · areas: fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 93aa98de29832ad3b89ac2753d05ce5880cb4148

<a id="claudius-maginificent"></a>
### @Claudius-Maginificent
<a id="claudius-maginificent-open"></a>
#### Open (41)
- [dashpay/tenderdash#1562 docs(consensus): add source-derived consensus protocol specification](https://github.com/dashpay/tenderdash/pull/1562) — 5 unresolved (5 bot) · 7 days stale · ✋ changes requested · areas: fallback · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Spec cites ErrUnsupportedVoteExtensionType that does not exist until #1565 merges**" — 7 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5661 feat: decide which identity keys may be disabled in the wallet, not in the mobile apps](https://github.com/dashpay/platform/pull/5661) — 7 unresolved (7 bot) · 0 days stale · ✋ changes requested · 🐢 targets fix/platform-wallet-dip11-key-rules · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk, fallback
  - Top thread: "🟡 Suggestion: Reject a null disable-list pointer with a nonzero count**" — 0 days old
- [dashpay/dash-evo-tool#1108 feat(documents): recognise indexOnly document types and delete their entries by values](https://github.com/dashpay/dash-evo-tool/pull/1108) — 3 unresolved (3 bot) · 6 days stale · ✋ changes requested · 📝 draft · 🐢 targets feat/document-optional-token-cost · areas: fallback
  - Top thread: "🟡 Suggestion: Make the non-deletable error actionable**" — 6 days old
- [dashpay/platform#5565 feat(platform-wallet)!: detect inconsistent shielded commitment trees and add explicit rebuild](https://github.com/dashpay/platform/pull/5565) — 5 unresolved (5 bot) · 1 days stale · 🐢 targets fix/shielded-sync-concurrent-append · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk, fallback
  - Top thread: "🟡 Suggestion: Coordinate Swift callback generations with the native rebuild restart**" — 1 days old
- [dashpay/dash-evo-tool#1107 fix(wallet): make every Max and fee match what the upstream builders accept](https://github.com/dashpay/dash-evo-tool/pull/1107) — 4 unresolved (1 CodeRabbit, 3 bot) · 0 days stale · ✋ changes requested · areas: fallback · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Do not use the all-input drain as the asset-lock Max upper bound**" — 0 days old
  - Blocker: coderabbitai left review threads unresolved; resolve them
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/dash-evo-tool#1153 chore(deps): bump dashpay/platform to v5.1-dev 6323a430](https://github.com/dashpay/dash-evo-tool/pull/1153) — 2 unresolved (2 CodeRabbit) · 0 days stale · ✋ changes requested · areas: fallback · Policy: waiting-bots
  - Top thread: "&lt;picture&gt;&lt;img src="https://storage.googleapis.com/coderabbit_public_assets/review-comment-icons-v1/amber-16.svg" width="…" — 0 days old
  - Blocker: thepastaclaw has not reported for the current head
  - Blocker: coderabbitai left review threads unresolved; resolve them
- [dashpay/platform#5546 fix(platform-wallet): support HASH160 DashPay contact signing keys](https://github.com/dashpay/platform/pull/5546) — 2 unresolved (2 bot) · 0 days stale · ✋ changes requested · 🐢 targets feat/platform-wallet-public-signing-key-selection · areas: rs-drive-abci, dpp, rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk, fallback
  - Top thread: "🟡 Suggestion: Wrap the new JNI capability export in the standard panic guard**" — 0 days old
- [dashpay/dash-evo-tool#1124 feat(documents): let the user skip an optional token cost](https://github.com/dashpay/dash-evo-tool/pull/1124) — 1 unresolved (1 bot) · 3 days stale · ✋ changes requested · 📝 draft · 🐢 targets fix/protocol14-signing-key-selection · areas: fallback
  - Top thread: "🔴 Blocking: Show the credit-fee outcome when an optional token cost is disabled**" — 3 days old
- [dashpay/platform#5588 feat(platform-wallet)!: expose protocol-14-aware signing-key selection](https://github.com/dashpay/platform/pull/5588) — 1 unresolved (1 bot) · 2 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · areas: dpp, rs-platform-wallet, rs-platform-wallet-ffi, rust-sdk · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Use an authoritative or conservative time source for expiry eligibility**" — 2 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
- [dashpay/dash-evo-tool#1136 fix(tokens): make every token screen work and fit a small window](https://github.com/dashpay/dash-evo-tool/pull/1136) — 1 unresolved (1 bot) · 0 days stale · areas: fallback · Policy: waiting-bots
  - Top thread: "🟡 Suggestion: Move pure token-input validators into the model layer**" — 0 days old
  - Blocker: thepastaclaw has not reported for the current head
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/platform#5645 fix(platform-wallet)!: check identity creation keys and funding in the Rust core](https://github.com/dashpay/platform/pull/5645) — 1 unresolved (1 bot) · 0 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · areas: rs-platform-wallet, rs-platform-wallet-ffi · Policy: waiting-author
  - Top thread: "🔴 Blocking: Creation preflight accepts consensus-invalid key sets**" — 0 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5647 feat: expose the Rust identity key layout to FFI, Swift and Kotlin hosts](https://github.com/dashpay/platform/pull/5647) — 1 unresolved (1 bot) · 0 days stale · ✋ changes requested · 🐢 targets fix/platform-wallet-dip11-identity-key-layout · areas: rs-platform-wallet-ffi, swift-sdk, kotlin-sdk, fallback
  - Top thread: "🟡 Suggestion: Validate layout discriminants before crossing the C ABI**" — 0 days old
- [dashpay/platform#5660 refactor(dpp): own the contract-bounds verdict for a document create](https://github.com/dashpay/platform/pull/5660) — 1 unresolved (1 bot) · 0 days stale · 🐢 targets feat/platform-wallet-public-signing-key-selection · areas: rs-drive-abci, dpp, rs-platform-wallet
  - Top thread: "🟡 Suggestion: Make the consumption verdict derive and validate its contract ID**" — 0 days old
- [dashpay/dash-evo-tool#1109 feat(masternode): use wallet-held node keys and withdraw via platform-wallet](https://github.com/dashpay/dash-evo-tool/pull/1109) — areas: fallback · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/dash-evo-tool#1110 feat(identity): use proven owner balances and keep identity balances ordered by block height](https://github.com/dashpay/dash-evo-tool/pull/1110) — ✋ changes requested · areas: fallback · Policy: waiting-author
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
- [dashpay/dash-evo-tool#1117 fix(identity): choose signing keys Platform accepts on protocol 14](https://github.com/dashpay/dash-evo-tool/pull/1117) — ⚠ merge conflict · ✋ changes requested · areas: fallback · Policy: waiting-bots
  - Blocker: coderabbitai has not reported for the current head
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
- [dashpay/dash-evo-tool#1120 feat(identity): add keys with spending limits and raise key limits](https://github.com/dashpay/dash-evo-tool/pull/1120) — ⚠ merge conflict · 📝 draft · 🐢 targets fix/protocol14-signing-key-selection · areas: fallback
- [dashpay/dash-evo-tool#1121 feat(contracts): view and claim contract fee pots](https://github.com/dashpay/dash-evo-tool/pull/1121) — ⚠ merge conflict · 📝 draft · 🐢 targets fix/protocol14-signing-key-selection · areas: fallback
- [dashpay/dash-evo-tool#1122 feat(documents): agree to document action fees before broadcasting](https://github.com/dashpay/dash-evo-tool/pull/1122) — ⚠ merge conflict · 📝 draft · 🐢 targets fix/protocol14-signing-key-selection · areas: fallback
- [dashpay/dash-evo-tool#1123 feat(identity): add authentication keys bound to a contract](https://github.com/dashpay/dash-evo-tool/pull/1123) — ⚠ merge conflict · 📝 draft · 🐢 targets feat/identity-key-limits · areas: fallback
- [dashpay/dash-evo-tool#1128 refactor(tokens)!: send token actions through platform-wallet](https://github.com/dashpay/dash-evo-tool/pull/1128) — ⚠ merge conflict · 📝 draft · 🐢 targets chore/bump-platform-0d91c133 · areas: fallback
- [dashpay/dash-evo-tool#1137 fix(identity): unlock the wallet that holds the chosen signing key](https://github.com/dashpay/dash-evo-tool/pull/1137) — ⚠ merge conflict · 📝 draft · 🐢 targets fix/protocol14-signing-key-selection · areas: fallback
- [dashpay/dash-evo-tool#1149 fix(identity): refresh the identity's balance after a paid operation](https://github.com/dashpay/dash-evo-tool/pull/1149) — areas: fallback · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head
- [dashpay/dash-evo-tool#1156 fix(dpns): keep a previous network's contest refresh off the new network](https://github.com/dashpay/dash-evo-tool/pull/1156) — 📝 draft · areas: fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/dash-evo-tool#1157 refactor(app): let each domain decide its reactions to backend results](https://github.com/dashpay/dash-evo-tool/pull/1157) — areas: fallback · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/dash-evo-tool#1158 fix(security): keep formatted panic messages out of logs and the terminal](https://github.com/dashpay/dash-evo-tool/pull/1158) — 📝 draft · 🐢 targets fix/dpns-refresh-network-attribution · areas: fallback
- [dashpay/dash-evo-tool#1159 fix(app): keep a previous network's background events and a request's progress where they belong](https://github.com/dashpay/dash-evo-tool/pull/1159) — 📝 draft · 🐢 targets fix/dpns-refresh-network-attribution · areas: fallback
- [dashpay/dash-evo-tool#1160 feat(identity): show an identity's tokens in the Identities hub](https://github.com/dashpay/dash-evo-tool/pull/1160) — ✋ changes requested · 🔴 CI failing · areas: fallback · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/dash-evo-tool#1162 fix(app): close promptly while a network switch is in progress](https://github.com/dashpay/dash-evo-tool/pull/1162) — 📝 draft · areas: fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/dash-evo-tool#1163 test: generate test key material at runtime instead of committing it](https://github.com/dashpay/dash-evo-tool/pull/1163) — 📝 draft · areas: fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/dash-evo-tool#1164 fix(dpns): fetch every contest that ends at a page boundary](https://github.com/dashpay/dash-evo-tool/pull/1164) — 📝 draft · areas: fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/dash-evo-tool#1165 fix(wallet): send Platform address transfers and withdrawals through platform-wallet](https://github.com/dashpay/dash-evo-tool/pull/1165) — 📝 draft · areas: fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#3549 test(platform-wallet): e2e framework + full test suite — triage pins, Found-*/PA-* guards, fail-closed persist, Stage-2 merge](https://github.com/dashpay/platform/pull/3549) — 📝 draft · ⏸ deferred · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk, rust-sdk, kotlin-sdk, github, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5384 fix(platform-wallet)!: persist chain truncations so a stored wallet follows a reorg](https://github.com/dashpay/platform/pull/5384) — ⏸ deferred · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk, kotlin-sdk, fallback
- [dashpay/platform#5417 fix(platform-wallet): own per-wallet transaction accounting in Rust for SwiftData](https://github.com/dashpay/platform/pull/5417) — ⏸ deferred · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk
- [dashpay/platform#5564 fix(platform-wallet): stop concurrent shielded syncs from duplicating commitments](https://github.com/dashpay/platform/pull/5564) — 🔴 CI failing · areas: rs-platform-wallet · Policy: waiting-build
  - Blocker: The build must pass before a human is asked
- [dashpay/platform#5570 fix(platform-wallet): reject zero-amount shielded spends before reserving notes](https://github.com/dashpay/platform/pull/5570) — 🔴 CI failing · areas: dpp, rs-platform-wallet · Policy: ready-to-merge
  - Blocker: All policy requirements are satisfied
  - Blocker: Proceeded without thepastaclaw: no review within the configured window
- [dashpay/platform#5613 fix(platform-wallet-storage): persist sentinel-scoped token balances of wallet-owned identities](https://github.com/dashpay/platform/pull/5613) — 🔴 CI failing · 📝 draft · areas: wallet-storage · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5642 docs: make layer-ownership catch implementation details leaking to clients](https://github.com/dashpay/platform/pull/5642) — 🔴 CI failing · areas: fallback · Policy: ready-to-merge
  - Blocker: All policy requirements are satisfied
- [dashpay/platform#5644 fix!: follow DIP-11 key rules when disabling keys and reading contact requests](https://github.com/dashpay/platform/pull/5644) — areas: dpp, rs-platform-wallet, rs-platform-wallet-ffi, rust-sdk, js-wasm-sdk, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
- [dashpay/platform#5670 fix(platform)!: refuse high-S signatures on key-recovery paths from protocol version 15](https://github.com/dashpay/platform/pull/5670) — 📝 draft · 🐢 targets shumkov/v51-rust-dashcore-dev-bump · areas: rs-drive-abci, dpp, fallback

<a id="claudius-maginificent-needs-action"></a>
#### Needs action (9)
- [dashpay/tenderdash#1562 docs(consensus): add source-derived consensus protocol specification](https://github.com/dashpay/tenderdash/pull/1562) — 5 unresolved (5 bot) · 7 days stale · ✋ changes requested · areas: fallback · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Spec cites ErrUnsupportedVoteExtensionType that does not exist until #1565 merges**" — 7 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/dash-evo-tool#1107 fix(wallet): make every Max and fee match what the upstream builders accept](https://github.com/dashpay/dash-evo-tool/pull/1107) — 4 unresolved (1 CodeRabbit, 3 bot) · 0 days stale · ✋ changes requested · areas: fallback · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Do not use the all-input drain as the asset-lock Max upper bound**" — 0 days old
  - Blocker: coderabbitai left review threads unresolved; resolve them
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/dash-evo-tool#1153 chore(deps): bump dashpay/platform to v5.1-dev 6323a430](https://github.com/dashpay/dash-evo-tool/pull/1153) — 2 unresolved (2 CodeRabbit) · 0 days stale · ✋ changes requested · areas: fallback · Policy: waiting-bots
  - Top thread: "&lt;picture&gt;&lt;img src="https://storage.googleapis.com/coderabbit_public_assets/review-comment-icons-v1/amber-16.svg" width="…" — 0 days old
  - Blocker: thepastaclaw has not reported for the current head
  - Blocker: coderabbitai left review threads unresolved; resolve them
- [dashpay/platform#5588 feat(platform-wallet)!: expose protocol-14-aware signing-key selection](https://github.com/dashpay/platform/pull/5588) — 1 unresolved (1 bot) · 2 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · areas: dpp, rs-platform-wallet, rs-platform-wallet-ffi, rust-sdk · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Use an authoritative or conservative time source for expiry eligibility**" — 2 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
- [dashpay/dash-evo-tool#1136 fix(tokens): make every token screen work and fit a small window](https://github.com/dashpay/dash-evo-tool/pull/1136) — 1 unresolved (1 bot) · 0 days stale · areas: fallback · Policy: waiting-bots
  - Top thread: "🟡 Suggestion: Move pure token-input validators into the model layer**" — 0 days old
  - Blocker: thepastaclaw has not reported for the current head
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/platform#5645 fix(platform-wallet)!: check identity creation keys and funding in the Rust core](https://github.com/dashpay/platform/pull/5645) — 1 unresolved (1 bot) · 0 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · areas: rs-platform-wallet, rs-platform-wallet-ffi · Policy: waiting-author
  - Top thread: "🔴 Blocking: Creation preflight accepts consensus-invalid key sets**" — 0 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/dash-evo-tool#1110 feat(identity): use proven owner balances and keep identity balances ordered by block height](https://github.com/dashpay/dash-evo-tool/pull/1110) — ✋ changes requested · areas: fallback · Policy: waiting-author
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
- [dashpay/dash-evo-tool#1117 fix(identity): choose signing keys Platform accepts on protocol 14](https://github.com/dashpay/dash-evo-tool/pull/1117) — ⚠ merge conflict · ✋ changes requested · areas: fallback · Policy: waiting-bots
  - Blocker: coderabbitai has not reported for the current head
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
- [dashpay/dash-evo-tool#1160 feat(identity): show an identity's tokens in the Identities hub](https://github.com/dashpay/dash-evo-tool/pull/1160) — ✋ changes requested · 🔴 CI failing · areas: fallback · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return

<a id="claudius-maginificent-ready-for-human"></a>
#### Ready for human (3)
- [dashpay/platform#5570 fix(platform-wallet): reject zero-amount shielded spends before reserving notes](https://github.com/dashpay/platform/pull/5570) — 🔴 CI failing · areas: dpp, rs-platform-wallet · Policy: ready-to-merge
  - Blocker: All policy requirements are satisfied
  - Blocker: Proceeded without thepastaclaw: no review within the configured window
- [dashpay/platform#5642 docs: make layer-ownership catch implementation details leaking to clients](https://github.com/dashpay/platform/pull/5642) — 🔴 CI failing · areas: fallback · Policy: ready-to-merge
  - Blocker: All policy requirements are satisfied
- [dashpay/platform#5644 fix!: follow DIP-11 key rules when disabling keys and reading contact requests](https://github.com/dashpay/platform/pull/5644) — areas: dpp, rs-platform-wallet, rs-platform-wallet-ffi, rust-sdk, js-wasm-sdk, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required

<a id="claudius-maginificent-unresolved-comments"></a>
#### Unresolved Comments (6)
- [dashpay/tenderdash#1562 docs(consensus): add source-derived consensus protocol specification](https://github.com/dashpay/tenderdash/pull/1562) — 5 unresolved (5 bot) · 7 days stale · ✋ changes requested · areas: fallback · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Spec cites ErrUnsupportedVoteExtensionType that does not exist until #1565 merges**" — 7 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/dash-evo-tool#1107 fix(wallet): make every Max and fee match what the upstream builders accept](https://github.com/dashpay/dash-evo-tool/pull/1107) — 4 unresolved (1 CodeRabbit, 3 bot) · 0 days stale · ✋ changes requested · areas: fallback · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Do not use the all-input drain as the asset-lock Max upper bound**" — 0 days old
  - Blocker: coderabbitai left review threads unresolved; resolve them
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/dash-evo-tool#1153 chore(deps): bump dashpay/platform to v5.1-dev 6323a430](https://github.com/dashpay/dash-evo-tool/pull/1153) — 2 unresolved (2 CodeRabbit) · 0 days stale · ✋ changes requested · areas: fallback · Policy: waiting-bots
  - Top thread: "&lt;picture&gt;&lt;img src="https://storage.googleapis.com/coderabbit_public_assets/review-comment-icons-v1/amber-16.svg" width="…" — 0 days old
  - Blocker: thepastaclaw has not reported for the current head
  - Blocker: coderabbitai left review threads unresolved; resolve them
- [dashpay/platform#5588 feat(platform-wallet)!: expose protocol-14-aware signing-key selection](https://github.com/dashpay/platform/pull/5588) — 1 unresolved (1 bot) · 2 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · areas: dpp, rs-platform-wallet, rs-platform-wallet-ffi, rust-sdk · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Use an authoritative or conservative time source for expiry eligibility**" — 2 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
- [dashpay/dash-evo-tool#1136 fix(tokens): make every token screen work and fit a small window](https://github.com/dashpay/dash-evo-tool/pull/1136) — 1 unresolved (1 bot) · 0 days stale · areas: fallback · Policy: waiting-bots
  - Top thread: "🟡 Suggestion: Move pure token-input validators into the model layer**" — 0 days old
  - Blocker: thepastaclaw has not reported for the current head
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/platform#5645 fix(platform-wallet)!: check identity creation keys and funding in the Rust core](https://github.com/dashpay/platform/pull/5645) — 1 unresolved (1 bot) · 0 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · areas: rs-platform-wallet, rs-platform-wallet-ffi · Policy: waiting-author
  - Top thread: "🔴 Blocking: Creation preflight accepts consensus-invalid key sets**" — 0 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them

<a id="claudius-maginificent-changes-requested"></a>
#### Changes Requested (3)
- [dashpay/dash-evo-tool#1110 feat(identity): use proven owner balances and keep identity balances ordered by block height](https://github.com/dashpay/dash-evo-tool/pull/1110) — ✋ changes requested · areas: fallback · Policy: waiting-author
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
- [dashpay/dash-evo-tool#1117 fix(identity): choose signing keys Platform accepts on protocol 14](https://github.com/dashpay/dash-evo-tool/pull/1117) — ⚠ merge conflict · ✋ changes requested · areas: fallback · Policy: waiting-bots
  - Blocker: coderabbitai has not reported for the current head
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
- [dashpay/dash-evo-tool#1160 feat(identity): show an identity's tokens in the Identities hub](https://github.com/dashpay/dash-evo-tool/pull/1160) — ✋ changes requested · 🔴 CI failing · areas: fallback · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return

<a id="claudius-maginificent-ci-failing"></a>
#### CI Failing (3)
- [dashpay/platform#5564 fix(platform-wallet): stop concurrent shielded syncs from duplicating commitments](https://github.com/dashpay/platform/pull/5564) — 🔴 CI failing · areas: rs-platform-wallet · Policy: waiting-build
  - Blocker: The build must pass before a human is asked
- [dashpay/platform#5570 fix(platform-wallet): reject zero-amount shielded spends before reserving notes](https://github.com/dashpay/platform/pull/5570) — 🔴 CI failing · areas: dpp, rs-platform-wallet · Policy: ready-to-merge
  - Blocker: All policy requirements are satisfied
  - Blocker: Proceeded without thepastaclaw: no review within the configured window
- [dashpay/platform#5642 docs: make layer-ownership catch implementation details leaking to clients](https://github.com/dashpay/platform/pull/5642) — 🔴 CI failing · areas: fallback · Policy: ready-to-merge
  - Blocker: All policy requirements are satisfied

<a id="claudius-maginificent-deferred"></a>
#### Deferred (3)
- [dashpay/platform#3549 test(platform-wallet): e2e framework + full test suite — triage pins, Found-*/PA-* guards, fail-closed persist, Stage-2 merge](https://github.com/dashpay/platform/pull/3549) — 📝 draft · ⏸ deferred · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk, rust-sdk, kotlin-sdk, github, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5384 fix(platform-wallet)!: persist chain truncations so a stored wallet follows a reorg](https://github.com/dashpay/platform/pull/5384) — ⏸ deferred · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk, kotlin-sdk, fallback
- [dashpay/platform#5417 fix(platform-wallet): own per-wallet transaction accounting in Rust for SwiftData](https://github.com/dashpay/platform/pull/5417) — ⏸ deferred · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk

<a id="claudius-maginificent-draft"></a>
#### Draft (6)
- [dashpay/dash-evo-tool#1156 fix(dpns): keep a previous network's contest refresh off the new network](https://github.com/dashpay/dash-evo-tool/pull/1156) — 📝 draft · areas: fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/dash-evo-tool#1162 fix(app): close promptly while a network switch is in progress](https://github.com/dashpay/dash-evo-tool/pull/1162) — 📝 draft · areas: fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/dash-evo-tool#1163 test: generate test key material at runtime instead of committing it](https://github.com/dashpay/dash-evo-tool/pull/1163) — 📝 draft · areas: fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/dash-evo-tool#1164 fix(dpns): fetch every contest that ends at a page boundary](https://github.com/dashpay/dash-evo-tool/pull/1164) — 📝 draft · areas: fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/dash-evo-tool#1165 fix(wallet): send Platform address transfers and withdrawals through platform-wallet](https://github.com/dashpay/dash-evo-tool/pull/1165) — 📝 draft · areas: fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5613 fix(platform-wallet-storage): persist sentinel-scoped token balances of wallet-owned identities](https://github.com/dashpay/platform/pull/5613) — 🔴 CI failing · 📝 draft · areas: wallet-storage · Policy: draft
  - Blocker: Draft PR does not occupy a review slot

<a id="claudius-maginificent-stale"></a>
#### Stale (16)
- [dashpay/platform#5661 feat: decide which identity keys may be disabled in the wallet, not in the mobile apps](https://github.com/dashpay/platform/pull/5661) — 7 unresolved (7 bot) · 0 days stale · ✋ changes requested · 🐢 targets fix/platform-wallet-dip11-key-rules · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk, fallback
  - Top thread: "🟡 Suggestion: Reject a null disable-list pointer with a nonzero count**" — 0 days old
- [dashpay/dash-evo-tool#1108 feat(documents): recognise indexOnly document types and delete their entries by values](https://github.com/dashpay/dash-evo-tool/pull/1108) — 3 unresolved (3 bot) · 6 days stale · ✋ changes requested · 📝 draft · 🐢 targets feat/document-optional-token-cost · areas: fallback
  - Top thread: "🟡 Suggestion: Make the non-deletable error actionable**" — 6 days old
- [dashpay/platform#5565 feat(platform-wallet)!: detect inconsistent shielded commitment trees and add explicit rebuild](https://github.com/dashpay/platform/pull/5565) — 5 unresolved (5 bot) · 1 days stale · 🐢 targets fix/shielded-sync-concurrent-append · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk, fallback
  - Top thread: "🟡 Suggestion: Coordinate Swift callback generations with the native rebuild restart**" — 1 days old
- [dashpay/platform#5546 fix(platform-wallet): support HASH160 DashPay contact signing keys](https://github.com/dashpay/platform/pull/5546) — 2 unresolved (2 bot) · 0 days stale · ✋ changes requested · 🐢 targets feat/platform-wallet-public-signing-key-selection · areas: rs-drive-abci, dpp, rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk, fallback
  - Top thread: "🟡 Suggestion: Wrap the new JNI capability export in the standard panic guard**" — 0 days old
- [dashpay/dash-evo-tool#1124 feat(documents): let the user skip an optional token cost](https://github.com/dashpay/dash-evo-tool/pull/1124) — 1 unresolved (1 bot) · 3 days stale · ✋ changes requested · 📝 draft · 🐢 targets fix/protocol14-signing-key-selection · areas: fallback
  - Top thread: "🔴 Blocking: Show the credit-fee outcome when an optional token cost is disabled**" — 3 days old
- [dashpay/platform#5647 feat: expose the Rust identity key layout to FFI, Swift and Kotlin hosts](https://github.com/dashpay/platform/pull/5647) — 1 unresolved (1 bot) · 0 days stale · ✋ changes requested · 🐢 targets fix/platform-wallet-dip11-identity-key-layout · areas: rs-platform-wallet-ffi, swift-sdk, kotlin-sdk, fallback
  - Top thread: "🟡 Suggestion: Validate layout discriminants before crossing the C ABI**" — 0 days old
- [dashpay/platform#5660 refactor(dpp): own the contract-bounds verdict for a document create](https://github.com/dashpay/platform/pull/5660) — 1 unresolved (1 bot) · 0 days stale · 🐢 targets feat/platform-wallet-public-signing-key-selection · areas: rs-drive-abci, dpp, rs-platform-wallet
  - Top thread: "🟡 Suggestion: Make the consumption verdict derive and validate its contract ID**" — 0 days old
- [dashpay/dash-evo-tool#1120 feat(identity): add keys with spending limits and raise key limits](https://github.com/dashpay/dash-evo-tool/pull/1120) — ⚠ merge conflict · 📝 draft · 🐢 targets fix/protocol14-signing-key-selection · areas: fallback
- [dashpay/dash-evo-tool#1121 feat(contracts): view and claim contract fee pots](https://github.com/dashpay/dash-evo-tool/pull/1121) — ⚠ merge conflict · 📝 draft · 🐢 targets fix/protocol14-signing-key-selection · areas: fallback
- [dashpay/dash-evo-tool#1122 feat(documents): agree to document action fees before broadcasting](https://github.com/dashpay/dash-evo-tool/pull/1122) — ⚠ merge conflict · 📝 draft · 🐢 targets fix/protocol14-signing-key-selection · areas: fallback
- [dashpay/dash-evo-tool#1123 feat(identity): add authentication keys bound to a contract](https://github.com/dashpay/dash-evo-tool/pull/1123) — ⚠ merge conflict · 📝 draft · 🐢 targets feat/identity-key-limits · areas: fallback
- [dashpay/dash-evo-tool#1128 refactor(tokens)!: send token actions through platform-wallet](https://github.com/dashpay/dash-evo-tool/pull/1128) — ⚠ merge conflict · 📝 draft · 🐢 targets chore/bump-platform-0d91c133 · areas: fallback
- [dashpay/dash-evo-tool#1137 fix(identity): unlock the wallet that holds the chosen signing key](https://github.com/dashpay/dash-evo-tool/pull/1137) — ⚠ merge conflict · 📝 draft · 🐢 targets fix/protocol14-signing-key-selection · areas: fallback
- [dashpay/dash-evo-tool#1158 fix(security): keep formatted panic messages out of logs and the terminal](https://github.com/dashpay/dash-evo-tool/pull/1158) — 📝 draft · 🐢 targets fix/dpns-refresh-network-attribution · areas: fallback
- [dashpay/dash-evo-tool#1159 fix(app): keep a previous network's background events and a request's progress where they belong](https://github.com/dashpay/dash-evo-tool/pull/1159) — 📝 draft · 🐢 targets fix/dpns-refresh-network-attribution · areas: fallback
- [dashpay/platform#5670 fix(platform)!: refuse high-S signatures on key-recovery paths from protocol version 15](https://github.com/dashpay/platform/pull/5670) — 📝 draft · 🐢 targets shumkov/v51-rust-dashcore-dev-bump · areas: rs-drive-abci, dpp, fallback

<a id="claudius-maginificent-clean"></a>
#### Clean (4)
- [dashpay/dash-evo-tool#1109 feat(masternode): use wallet-held node keys and withdraw via platform-wallet](https://github.com/dashpay/dash-evo-tool/pull/1109) — areas: fallback · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/dash-evo-tool#1149 fix(identity): refresh the identity's balance after a paid operation](https://github.com/dashpay/dash-evo-tool/pull/1149) — areas: fallback · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head
- [dashpay/dash-evo-tool#1157 refactor(app): let each domain decide its reactions to backend results](https://github.com/dashpay/dash-evo-tool/pull/1157) — areas: fallback · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/platform#5644 fix!: follow DIP-11 key rules when disabling keys and reading contact requests](https://github.com/dashpay/platform/pull/5644) — areas: dpp, rs-platform-wallet, rs-platform-wallet-ffi, rust-sdk, js-wasm-sdk, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required

<a id="quantumexplorer"></a>
### @QuantumExplorer
<a id="quantumexplorer-open"></a>
#### Open (22)
- [dashpay/rust-dashcore#1078 feat(key-wallet)!: lock masternode collateral out of coin selection](https://github.com/dashpay/rust-dashcore/pull/1078) — 8 unresolved (8 human) · 29 days stale · ⚠ merge conflict · ✋ changes requested · areas: key-wallet, key-wallet-manager · Policy: waiting-self-review
  - Top thread: "This sets the state by hand, and together with the serde round-trip it is the only coverage of the refresh inside \`updat…" — 29 days old
  - Blocker: Author must post /self-reviewed 44a9a010500fbd44aba09111b4c53a28b88e8073
- [dashpay/platform#4272 feat(dpp)!: dashpay payment detection keys and stealth derivation](https://github.com/dashpay/platform/pull/4272) — 9 unresolved (9 bot) · 87 days stale · ⚠ merge conflict · 🔴 CI failing · 📝 draft · areas: rs-drive, rs-drive-abci, dpp, rs-platform-wallet, swift-sdk, system-contracts, kotlin-sdk, fallback · Policy: draft
  - Top thread: "🔴 Blocking: Require compressed 33-byte encodings for payment keys**" — 87 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#860 feat(dash-spv): expose per-peer connection stats (ping RTT, bytes received)](https://github.com/dashpay/rust-dashcore/pull/860) — 3 unresolved (3 CodeRabbit) · 111 days stale · ⚠ merge conflict · ✋ changes requested · areas: dash-spv, fallback · Policy: waiting-author
  - Top thread: "_📐 Maintainability &amp; Code Quality_ \| _🟡 Minor_ \| _⚡ Quick win_" — 111 days old
  - Blocker: coderabbitai left review threads unresolved; resolve them
- [dashpay/platform#5238 fix(platform)!: refund sponsor-paid document storage to the gas sponsor](https://github.com/dashpay/platform/pull/5238) — 3 unresolved (3 bot) · 28 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · 📝 draft · areas: rs-drive, rs-drive-abci, fallback · Policy: draft
  - Top thread: "🟡 Suggestion: Make the transfer regression change the primary element's size**" — 28 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#1077 feat(dash-spv)!: check masternode list diffs against the block coinbase](https://github.com/dashpay/rust-dashcore/pull/1077) — 2 unresolved (2 CodeRabbit) · 32 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · areas: dash-spv, fallback · Policy: waiting-author
  - Top thread: "_🩺 Stability &amp; Availability_ \| _🟠 Major_ \| _⚡ Quick win_" — 32 days old
  - Blocker: coderabbitai requested changes on this head; dismiss the review or push a fix
  - Blocker: coderabbitai left review threads unresolved; resolve them
- [dashpay/platform#5604 feat(platform)!: generatedFrom sys.tokens.decimals generates a token's decimals (PV15)](https://github.com/dashpay/platform/pull/5604) — 4 unresolved (4 bot) · 2 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · areas: rs-drive, rs-drive-abci, dpp, fallback · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Classify only known integer-generating token functions**" — 2 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#3417 feat(swift-sdk): use SPV-synced quorums for Platform proof verification](https://github.com/dashpay/platform/pull/3417) — 2 unresolved (2 bot) · 7 days stale · ✋ changes requested · 🔴 CI failing · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, rust-sdk, rust-sdk-ffi, fallback · Policy: waiting-author
  - Top thread: "🔴 Blocking: Require network-difficulty validation before trusting SPV headers**" — 7 days old
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5529 feat(platform)!: token marketplace listings (PV15)](https://github.com/dashpay/platform/pull/5529) — 2 unresolved (2 bot) · 4 days stale · ✋ changes requested · 🐢 targets claude/token-market-escrow-contract · areas: rs-drive, rs-drive-abci, dpp, rs-platform-wallet, swift-sdk, kotlin-sdk, fallback
  - Top thread: "🔴 Blocking: Version the consensus order-ID derivation through platform-version tables**" — 4 days old
- [dashpay/platform#5532 feat(platform)!: destroy a frozen identity's escrowed listings with its funds (PV15)](https://github.com/dashpay/platform/pull/5532) — 2 unresolved (2 bot) · 4 days stale · 🐢 targets claude/token-market-history · areas: rs-drive, rs-drive-abci, fallback
  - Top thread: "🟡 Suggestion: Avoid stateful frozen-listing reads during CheckTx cost approximation**" — 4 days old
- [dashpay/platform#5360 perf(platform)!: delete a consumed document from what its lookup read (PV15)](https://github.com/dashpay/platform/pull/5360) — 1 unresolved (1 bot) · 0 days stale · areas: rs-drive, rs-drive-abci, fallback · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Use the flags query only for consuming lookups**" — 0 days old
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/grovedb#1001 feat(batch)!: opt-in settlement of storage owner changes](https://github.com/dashpay/grovedb/pull/1001) — ⚠ merge conflict · areas: fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 814dfc765747cec0beb7a39e1cdf04d301ce9ac3
- [dashpay/platform#4519 feat(platform-wallet): rotate a masternode's keys into the wallet — ProUpRegTx orchestration, FFI, Swift](https://github.com/dashpay/platform/pull/4519) — ⚠ merge conflict · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 92a3ebe17adc12432fb756c63f172c663809007d
  - Blocker: Proceeded without thepastaclaw: no review within the configured window
- [dashpay/platform#4730 feat(platform)!: delta-based data contract update transition for protocol version 15](https://github.com/dashpay/platform/pull/4730) — ⚠ merge conflict · 🔴 CI failing · 📝 draft · areas: rs-drive, rs-drive-abci, dpp, rs-platform-wallet, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#4776 feat(platform-wallet)!: let contract updates clear the description through the FFI](https://github.com/dashpay/platform/pull/4776) — 🔴 CI failing · 🐢 targets feat/delta-contract-update-pv15 · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk, fallback
- [dashpay/platform#4933 feat(platform)!: dashpay contact requests declare their checks](https://github.com/dashpay/platform/pull/4933) — ⚠ merge conflict · 🔴 CI failing · 📝 draft · areas: rs-drive, rs-drive-abci, dpp, system-contracts, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#4986 fix(drive-abci)!: pay every reward share of a masternode and credit each identity once in the epoch payout (PV15)](https://github.com/dashpay/platform/pull/4986) — ⚠ merge conflict · 🔴 CI failing · 📝 draft · areas: rs-drive, rs-drive-abci, dpp, github, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5113 feat(platform-wallet)!: keep masternode collateral out of coin selection](https://github.com/dashpay/platform/pull/5113) — ⚠ merge conflict · 🔴 CI failing · 📝 draft · 🐢 targets chore/bump-rust-dashcore-secp-033 · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk, fallback
- [dashpay/platform#5526 feat(platform)!: token marketplace escrow trees and token market contract (PV15)](https://github.com/dashpay/platform/pull/5526) — 🔴 CI failing · areas: rs-drive, rs-drive-abci, dpp, rust-sdk, rust-sdk-ffi, js-wasm-sdk, system-contracts, github, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 3f80b802df233105fb2fcdffb43f50eac808956e
- [dashpay/platform#5530 feat(platform)!: token marketplace offers (PV15)](https://github.com/dashpay/platform/pull/5530) — ✋ changes requested · 🐢 targets claude/token-market-listings · areas: rs-drive, rs-drive-abci, dpp, rs-platform-wallet, fallback
- [dashpay/platform#5531 feat(platform)!: record token marketplace events in token history (PV15)](https://github.com/dashpay/platform/pull/5531) — 🐢 targets claude/token-market-offers · areas: rs-drive, rs-drive-abci, dpp, system-contracts, fallback
- [dashpay/platform#5651 feat(platform): yes/no vote poll state and masternode answer queries](https://github.com/dashpay/platform/pull/5651) — 🔴 CI failing · areas: rs-drive, rs-drive-abci, rust-sdk, js-wasm-sdk, rust-dapi, fallback · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head
- [dashpay/tenderdash#1572 feat(consensus)!: hand a silent proposer's idle round to the next proposer](https://github.com/dashpay/tenderdash/pull/1572) — areas: fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 93aa98de29832ad3b89ac2753d05ce5880cb4148

<a id="quantumexplorer-needs-action"></a>
#### Needs action (8)
- [dashpay/rust-dashcore#1078 feat(key-wallet)!: lock masternode collateral out of coin selection](https://github.com/dashpay/rust-dashcore/pull/1078) — 8 unresolved (8 human) · 29 days stale · ⚠ merge conflict · ✋ changes requested · areas: key-wallet, key-wallet-manager · Policy: waiting-self-review
  - Top thread: "This sets the state by hand, and together with the serde round-trip it is the only coverage of the refresh inside \`updat…" — 29 days old
  - Blocker: Author must post /self-reviewed 44a9a010500fbd44aba09111b4c53a28b88e8073
- [dashpay/rust-dashcore#860 feat(dash-spv): expose per-peer connection stats (ping RTT, bytes received)](https://github.com/dashpay/rust-dashcore/pull/860) — 3 unresolved (3 CodeRabbit) · 111 days stale · ⚠ merge conflict · ✋ changes requested · areas: dash-spv, fallback · Policy: waiting-author
  - Top thread: "_📐 Maintainability &amp; Code Quality_ \| _🟡 Minor_ \| _⚡ Quick win_" — 111 days old
  - Blocker: coderabbitai left review threads unresolved; resolve them
- [dashpay/rust-dashcore#1077 feat(dash-spv)!: check masternode list diffs against the block coinbase](https://github.com/dashpay/rust-dashcore/pull/1077) — 2 unresolved (2 CodeRabbit) · 32 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · areas: dash-spv, fallback · Policy: waiting-author
  - Top thread: "_🩺 Stability &amp; Availability_ \| _🟠 Major_ \| _⚡ Quick win_" — 32 days old
  - Blocker: coderabbitai requested changes on this head; dismiss the review or push a fix
  - Blocker: coderabbitai left review threads unresolved; resolve them
- [dashpay/platform#5604 feat(platform)!: generatedFrom sys.tokens.decimals generates a token's decimals (PV15)](https://github.com/dashpay/platform/pull/5604) — 4 unresolved (4 bot) · 2 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · areas: rs-drive, rs-drive-abci, dpp, fallback · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Classify only known integer-generating token functions**" — 2 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#3417 feat(swift-sdk): use SPV-synced quorums for Platform proof verification](https://github.com/dashpay/platform/pull/3417) — 2 unresolved (2 bot) · 7 days stale · ✋ changes requested · 🔴 CI failing · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, rust-sdk, rust-sdk-ffi, fallback · Policy: waiting-author
  - Top thread: "🔴 Blocking: Require network-difficulty validation before trusting SPV headers**" — 7 days old
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5360 perf(platform)!: delete a consumed document from what its lookup read (PV15)](https://github.com/dashpay/platform/pull/5360) — 1 unresolved (1 bot) · 0 days stale · areas: rs-drive, rs-drive-abci, fallback · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Use the flags query only for consuming lookups**" — 0 days old
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/grovedb#1001 feat(batch)!: opt-in settlement of storage owner changes](https://github.com/dashpay/grovedb/pull/1001) — ⚠ merge conflict · areas: fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 814dfc765747cec0beb7a39e1cdf04d301ce9ac3
- [dashpay/platform#4519 feat(platform-wallet): rotate a masternode's keys into the wallet — ProUpRegTx orchestration, FFI, Swift](https://github.com/dashpay/platform/pull/4519) — ⚠ merge conflict · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 92a3ebe17adc12432fb756c63f172c663809007d
  - Blocker: Proceeded without thepastaclaw: no review within the configured window

<a id="quantumexplorer-unresolved-comments"></a>
#### Unresolved Comments (6)
- [dashpay/rust-dashcore#1078 feat(key-wallet)!: lock masternode collateral out of coin selection](https://github.com/dashpay/rust-dashcore/pull/1078) — 8 unresolved (8 human) · 29 days stale · ⚠ merge conflict · ✋ changes requested · areas: key-wallet, key-wallet-manager · Policy: waiting-self-review
  - Top thread: "This sets the state by hand, and together with the serde round-trip it is the only coverage of the refresh inside \`updat…" — 29 days old
  - Blocker: Author must post /self-reviewed 44a9a010500fbd44aba09111b4c53a28b88e8073
- [dashpay/rust-dashcore#860 feat(dash-spv): expose per-peer connection stats (ping RTT, bytes received)](https://github.com/dashpay/rust-dashcore/pull/860) — 3 unresolved (3 CodeRabbit) · 111 days stale · ⚠ merge conflict · ✋ changes requested · areas: dash-spv, fallback · Policy: waiting-author
  - Top thread: "_📐 Maintainability &amp; Code Quality_ \| _🟡 Minor_ \| _⚡ Quick win_" — 111 days old
  - Blocker: coderabbitai left review threads unresolved; resolve them
- [dashpay/rust-dashcore#1077 feat(dash-spv)!: check masternode list diffs against the block coinbase](https://github.com/dashpay/rust-dashcore/pull/1077) — 2 unresolved (2 CodeRabbit) · 32 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · areas: dash-spv, fallback · Policy: waiting-author
  - Top thread: "_🩺 Stability &amp; Availability_ \| _🟠 Major_ \| _⚡ Quick win_" — 32 days old
  - Blocker: coderabbitai requested changes on this head; dismiss the review or push a fix
  - Blocker: coderabbitai left review threads unresolved; resolve them
- [dashpay/platform#5604 feat(platform)!: generatedFrom sys.tokens.decimals generates a token's decimals (PV15)](https://github.com/dashpay/platform/pull/5604) — 4 unresolved (4 bot) · 2 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · areas: rs-drive, rs-drive-abci, dpp, fallback · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Classify only known integer-generating token functions**" — 2 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#3417 feat(swift-sdk): use SPV-synced quorums for Platform proof verification](https://github.com/dashpay/platform/pull/3417) — 2 unresolved (2 bot) · 7 days stale · ✋ changes requested · 🔴 CI failing · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, rust-sdk, rust-sdk-ffi, fallback · Policy: waiting-author
  - Top thread: "🔴 Blocking: Require network-difficulty validation before trusting SPV headers**" — 7 days old
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5360 perf(platform)!: delete a consumed document from what its lookup read (PV15)](https://github.com/dashpay/platform/pull/5360) — 1 unresolved (1 bot) · 0 days stale · areas: rs-drive, rs-drive-abci, fallback · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Use the flags query only for consuming lookups**" — 0 days old
  - Blocker: thepastaclaw left review threads unresolved; resolve them

<a id="quantumexplorer-ci-failing"></a>
#### CI Failing (2)
- [dashpay/platform#5526 feat(platform)!: token marketplace escrow trees and token market contract (PV15)](https://github.com/dashpay/platform/pull/5526) — 🔴 CI failing · areas: rs-drive, rs-drive-abci, dpp, rust-sdk, rust-sdk-ffi, js-wasm-sdk, system-contracts, github, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 3f80b802df233105fb2fcdffb43f50eac808956e
- [dashpay/platform#5651 feat(platform): yes/no vote poll state and masternode answer queries](https://github.com/dashpay/platform/pull/5651) — 🔴 CI failing · areas: rs-drive, rs-drive-abci, rust-sdk, js-wasm-sdk, rust-dapi, fallback · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head

<a id="quantumexplorer-draft"></a>
#### Draft (5)
- [dashpay/platform#4272 feat(dpp)!: dashpay payment detection keys and stealth derivation](https://github.com/dashpay/platform/pull/4272) — 9 unresolved (9 bot) · 87 days stale · ⚠ merge conflict · 🔴 CI failing · 📝 draft · areas: rs-drive, rs-drive-abci, dpp, rs-platform-wallet, swift-sdk, system-contracts, kotlin-sdk, fallback · Policy: draft
  - Top thread: "🔴 Blocking: Require compressed 33-byte encodings for payment keys**" — 87 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5238 fix(platform)!: refund sponsor-paid document storage to the gas sponsor](https://github.com/dashpay/platform/pull/5238) — 3 unresolved (3 bot) · 28 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · 📝 draft · areas: rs-drive, rs-drive-abci, fallback · Policy: draft
  - Top thread: "🟡 Suggestion: Make the transfer regression change the primary element's size**" — 28 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#4730 feat(platform)!: delta-based data contract update transition for protocol version 15](https://github.com/dashpay/platform/pull/4730) — ⚠ merge conflict · 🔴 CI failing · 📝 draft · areas: rs-drive, rs-drive-abci, dpp, rs-platform-wallet, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#4933 feat(platform)!: dashpay contact requests declare their checks](https://github.com/dashpay/platform/pull/4933) — ⚠ merge conflict · 🔴 CI failing · 📝 draft · areas: rs-drive, rs-drive-abci, dpp, system-contracts, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#4986 fix(drive-abci)!: pay every reward share of a masternode and credit each identity once in the epoch payout (PV15)](https://github.com/dashpay/platform/pull/4986) — ⚠ merge conflict · 🔴 CI failing · 📝 draft · areas: rs-drive, rs-drive-abci, dpp, github, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot

<a id="quantumexplorer-stale"></a>
#### Stale (6)
- [dashpay/platform#5529 feat(platform)!: token marketplace listings (PV15)](https://github.com/dashpay/platform/pull/5529) — 2 unresolved (2 bot) · 4 days stale · ✋ changes requested · 🐢 targets claude/token-market-escrow-contract · areas: rs-drive, rs-drive-abci, dpp, rs-platform-wallet, swift-sdk, kotlin-sdk, fallback
  - Top thread: "🔴 Blocking: Version the consensus order-ID derivation through platform-version tables**" — 4 days old
- [dashpay/platform#5532 feat(platform)!: destroy a frozen identity's escrowed listings with its funds (PV15)](https://github.com/dashpay/platform/pull/5532) — 2 unresolved (2 bot) · 4 days stale · 🐢 targets claude/token-market-history · areas: rs-drive, rs-drive-abci, fallback
  - Top thread: "🟡 Suggestion: Avoid stateful frozen-listing reads during CheckTx cost approximation**" — 4 days old
- [dashpay/platform#4776 feat(platform-wallet)!: let contract updates clear the description through the FFI](https://github.com/dashpay/platform/pull/4776) — 🔴 CI failing · 🐢 targets feat/delta-contract-update-pv15 · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk, fallback
- [dashpay/platform#5113 feat(platform-wallet)!: keep masternode collateral out of coin selection](https://github.com/dashpay/platform/pull/5113) — ⚠ merge conflict · 🔴 CI failing · 📝 draft · 🐢 targets chore/bump-rust-dashcore-secp-033 · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk, fallback
- [dashpay/platform#5530 feat(platform)!: token marketplace offers (PV15)](https://github.com/dashpay/platform/pull/5530) — ✋ changes requested · 🐢 targets claude/token-market-listings · areas: rs-drive, rs-drive-abci, dpp, rs-platform-wallet, fallback
- [dashpay/platform#5531 feat(platform)!: record token marketplace events in token history (PV15)](https://github.com/dashpay/platform/pull/5531) — 🐢 targets claude/token-market-offers · areas: rs-drive, rs-drive-abci, dpp, system-contracts, fallback

<a id="quantumexplorer-clean"></a>
#### Clean (3)
- [dashpay/grovedb#1001 feat(batch)!: opt-in settlement of storage owner changes](https://github.com/dashpay/grovedb/pull/1001) — ⚠ merge conflict · areas: fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 814dfc765747cec0beb7a39e1cdf04d301ce9ac3
- [dashpay/platform#4519 feat(platform-wallet): rotate a masternode's keys into the wallet — ProUpRegTx orchestration, FFI, Swift](https://github.com/dashpay/platform/pull/4519) — ⚠ merge conflict · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 92a3ebe17adc12432fb756c63f172c663809007d
  - Blocker: Proceeded without thepastaclaw: no review within the configured window
- [dashpay/tenderdash#1572 feat(consensus)!: hand a silent proposer's idle round to the next proposer](https://github.com/dashpay/tenderdash/pull/1572) — areas: fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 93aa98de29832ad3b89ac2753d05ce5880cb4148

<a id="quantumexplorer-ready-for-review"></a>
#### Ready for Review (6)
- [dashpay/platform#4298 fix(dashmate): load ZeroSSL config in force mode](https://github.com/dashpay/platform/pull/4298) — by @thepastaclaw · areas: dashmate · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
- [dashpay/platform#4703 fix(platform): resolve fee versions by registered number and price refunds at the storage epoch rate](https://github.com/dashpay/platform/pull/4703) — by @DCG-Claude · areas: rs-drive, rs-drive-abci, dpp, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
- [dashpay/platform#4706 feat(platform)!: require fee history for storage refunds and credit their recorded owners](https://github.com/dashpay/platform/pull/4706) — by @DCG-Claude · areas: rs-drive, rs-drive-abci, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
- [dashpay/platform#5288 feat(platform-wallet)!: support DashPay shielded tips with dedicated accounts](https://github.com/dashpay/platform/pull/5288) — by @PastaPastaPasta · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk, rust-sdk, kotlin-sdk, fallback · Policy: waiting-author
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5644 fix!: follow DIP-11 key rules when disabling keys and reading contact requests](https://github.com/dashpay/platform/pull/5644) — by @Claudius-Maginificent · areas: dpp, rs-platform-wallet, rs-platform-wallet-ffi, rust-sdk, js-wasm-sdk, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
- [dashpay/rust-dashcore#264 feat(dash-spv): Add BIP324 v2 encrypted P2P transport](https://github.com/dashpay/rust-dashcore/pull/264) — by @PastaPastaPasta · areas: dash-spv, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 907775512ddd78a9e8e6d2ff12a30cf306ea6c05

<a id="pastapastapasta"></a>
### @PastaPastaPasta
<a id="pastapastapasta-open"></a>
#### Open (20)
- [dashpay/platform#4648 feat(drive-abci)!: state sync via ABCI snapshots with reduced platform state (protocol v15)](https://github.com/dashpay/platform/pull/4648) — 8 unresolved (8 bot) · 24 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · 📝 draft · areas: rs-drive, rs-drive-abci, dpp, fallback · Policy: draft
  - Top thread: "🟡 Suggestion: Continue the restored target through consensus and a restart**" — 24 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#137 fix: integration tests, continued dashificiation](https://github.com/dashpay/rust-dashcore/pull/137) — 2 unresolved (2 CodeRabbit) · 419 days stale · ⚠ merge conflict · 🔴 CI failing · 🐢 targets v0.40-dev, untouched 322 days · areas: fallback
  - Top thread: "_🛠️ Refactor suggestion_" — 419 days old
- [dashpay/platform#5669 feat(sdk): refresh the embedded quorum-proof checkpoints on every release](https://github.com/dashpay/platform/pull/5669) — 5 unresolved (5 bot) · 0 days stale · ✋ changes requested · 🐢 targets feat/verified-quorum-context-provider · areas: fallback
  - Top thread: "🟡 Suggestion: Cap relay responses before buffering them**" — 0 days old
- [dashpay/platform#4965 test(sdk): cargo-fuzz harness over drive-proof-verifier FromProof](https://github.com/dashpay/platform/pull/4965) — 1 unresolved (1 bot) · 23 days stale · 🔴 CI failing · 📝 draft · areas: github, fallback · Policy: draft
  - Top thread: "🟡 Suggestion: Correct the claim that PV13 reaches the V0 proof decoder**" — 23 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5280 perf(platform-wallet)!: reuse a recent recorded-anchor set on the shielded spend path](https://github.com/dashpay/platform/pull/5280) — 1 unresolved (1 bot) · 15 days stale · 🔴 CI failing · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk · Policy: waiting-self-review
  - Top thread: "🟡 Suggestion: Guard the background-prefetch export against nested Tokio entry**" — 15 days old
  - Blocker: Author must post /self-reviewed f0ef5a65d32dac0c196a572a66a94612eec7834d
- [dashpay/platform#5609 fix(sdk): restore JS DAPI client node discovery after Core V24](https://github.com/dashpay/platform/pull/5609) — 2 unresolved (2 bot) · 2 days stale · 📝 draft · areas: fallback · Policy: draft
  - Top thread: "✅ **Resolved** at \`5cf1b7d5\`; see the replies below." — 2 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5277 perf(platform-wallet): don't hold the shielded store lock across sync network I/O](https://github.com/dashpay/platform/pull/5277) — 1 unresolved (1 bot) · 0 days stale · ✋ changes requested · 🔴 CI failing · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk · Policy: waiting-author
  - Top thread: "🔴 Blocking: Prevent a completed sync from overwriting a concurrent spend in durable persistence**" — 0 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5523 feat(sdk): add opt-in verified HTTP context provider](https://github.com/dashpay/platform/pull/5523) — 1 unresolved (1 bot) · 0 days stale · areas: github, fallback · Policy: waiting-self-review
  - Top thread: "🟡 Suggestion: Pin the expected embedded checkpoint identity**" — 0 days old
  - Blocker: Author must post /self-reviewed 5feef634b3fa0cbe8fb0722f263b49b7a5c7f260
- [dashpay/grovedb#1007 build(deps)!: use dashpay/orchard with the faster prover](https://github.com/dashpay/grovedb/pull/1007) — 🔴 CI failing · 📝 draft · areas: fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#4633 feat(sdk): add dash-sdk-cxx, a C++ binding of dash-sdk for Dash Core](https://github.com/dashpay/platform/pull/4633) — 📝 draft · areas: rust-sdk, github, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5282 perf(platform-wallet)!: prove the shield bundle while waiting for the InstantSend lock](https://github.com/dashpay/platform/pull/5282) — ⚠ merge conflict · 🔴 CI failing · areas: dpp, rs-platform-wallet, rs-platform-wallet-ffi, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 0f547bc62991d50a0a316fc50f712db1da78c9f7
- [dashpay/platform#5285 feat(platform)!: subscribe to committed state transitions matching document and address filters](https://github.com/dashpay/platform/pull/5285) — ⚠ merge conflict · 🔴 CI failing · 📝 draft · areas: rs-drive, rs-drive-abci, rust-sdk, dashmate, rust-dapi, github, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5288 feat(platform-wallet)!: support DashPay shielded tips with dedicated accounts](https://github.com/dashpay/platform/pull/5288) — areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk, rust-sdk, kotlin-sdk, fallback · Policy: waiting-author
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5289 feat(sdk)!: optional BIP-39 passphrase through the mnemonic resolver and wallet creation](https://github.com/dashpay/platform/pull/5289) — 🔴 CI failing · 📝 draft · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, rust-sdk-ffi, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5292 feat(dashmate): state sync configuration for tenderdash and drive snapshots](https://github.com/dashpay/platform/pull/5292) — ⚠ merge conflict · 🔴 CI failing · 📝 draft · areas: dashmate · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5475 feat(sdk): forward DashPay profiles, invitations and masternode voting in dash-sdk-cxx](https://github.com/dashpay/platform/pull/5475) — ⚠ merge conflict · 🔴 CI failing · 📝 draft · areas: rs-platform-wallet, rs-platform-wallet-ffi, rust-sdk, github, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5577 feat: gzip-compress DAPI responses](https://github.com/dashpay/platform/pull/5577) — ✋ changes requested · 🔴 CI failing · areas: dashmate, rust-dapi, fallback · Policy: waiting-author
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
- [dashpay/platform#5610 perf(sdk): verify each block's signature once instead of on every proved response](https://github.com/dashpay/platform/pull/5610) — 🔴 CI failing · 📝 draft · areas: fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#264 feat(dash-spv): Add BIP324 v2 encrypted P2P transport](https://github.com/dashpay/rust-dashcore/pull/264) — areas: dash-spv, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 907775512ddd78a9e8e6d2ff12a30cf306ea6c05
- [dashpay/rust-dashcore#959 feat(dash-spv): adaptive gap-limit probe escalation for BIP44 discovery](https://github.com/dashpay/rust-dashcore/pull/959) — 🔴 CI failing · areas: dash-spv, key-wallet, key-wallet-manager, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed d4aa206c23b6f523d2c51dd5aa3c083c3730a911

<a id="pastapastapasta-needs-action"></a>
#### Needs action (5)
- [dashpay/platform#5280 perf(platform-wallet)!: reuse a recent recorded-anchor set on the shielded spend path](https://github.com/dashpay/platform/pull/5280) — 1 unresolved (1 bot) · 15 days stale · 🔴 CI failing · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk · Policy: waiting-self-review
  - Top thread: "🟡 Suggestion: Guard the background-prefetch export against nested Tokio entry**" — 15 days old
  - Blocker: Author must post /self-reviewed f0ef5a65d32dac0c196a572a66a94612eec7834d
- [dashpay/platform#5277 perf(platform-wallet): don't hold the shielded store lock across sync network I/O](https://github.com/dashpay/platform/pull/5277) — 1 unresolved (1 bot) · 0 days stale · ✋ changes requested · 🔴 CI failing · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk · Policy: waiting-author
  - Top thread: "🔴 Blocking: Prevent a completed sync from overwriting a concurrent spend in durable persistence**" — 0 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5523 feat(sdk): add opt-in verified HTTP context provider](https://github.com/dashpay/platform/pull/5523) — 1 unresolved (1 bot) · 0 days stale · areas: github, fallback · Policy: waiting-self-review
  - Top thread: "🟡 Suggestion: Pin the expected embedded checkpoint identity**" — 0 days old
  - Blocker: Author must post /self-reviewed 5feef634b3fa0cbe8fb0722f263b49b7a5c7f260
- [dashpay/platform#5282 perf(platform-wallet)!: prove the shield bundle while waiting for the InstantSend lock](https://github.com/dashpay/platform/pull/5282) — ⚠ merge conflict · 🔴 CI failing · areas: dpp, rs-platform-wallet, rs-platform-wallet-ffi, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 0f547bc62991d50a0a316fc50f712db1da78c9f7
- [dashpay/platform#5577 feat: gzip-compress DAPI responses](https://github.com/dashpay/platform/pull/5577) — ✋ changes requested · 🔴 CI failing · areas: dashmate, rust-dapi, fallback · Policy: waiting-author
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix

<a id="pastapastapasta-unresolved-comments"></a>
#### Unresolved Comments (3)
- [dashpay/platform#5280 perf(platform-wallet)!: reuse a recent recorded-anchor set on the shielded spend path](https://github.com/dashpay/platform/pull/5280) — 1 unresolved (1 bot) · 15 days stale · 🔴 CI failing · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk · Policy: waiting-self-review
  - Top thread: "🟡 Suggestion: Guard the background-prefetch export against nested Tokio entry**" — 15 days old
  - Blocker: Author must post /self-reviewed f0ef5a65d32dac0c196a572a66a94612eec7834d
- [dashpay/platform#5277 perf(platform-wallet): don't hold the shielded store lock across sync network I/O](https://github.com/dashpay/platform/pull/5277) — 1 unresolved (1 bot) · 0 days stale · ✋ changes requested · 🔴 CI failing · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk · Policy: waiting-author
  - Top thread: "🔴 Blocking: Prevent a completed sync from overwriting a concurrent spend in durable persistence**" — 0 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5523 feat(sdk): add opt-in verified HTTP context provider](https://github.com/dashpay/platform/pull/5523) — 1 unresolved (1 bot) · 0 days stale · areas: github, fallback · Policy: waiting-self-review
  - Top thread: "🟡 Suggestion: Pin the expected embedded checkpoint identity**" — 0 days old
  - Blocker: Author must post /self-reviewed 5feef634b3fa0cbe8fb0722f263b49b7a5c7f260

<a id="pastapastapasta-changes-requested"></a>
#### Changes Requested (1)
- [dashpay/platform#5577 feat: gzip-compress DAPI responses](https://github.com/dashpay/platform/pull/5577) — ✋ changes requested · 🔴 CI failing · areas: dashmate, rust-dapi, fallback · Policy: waiting-author
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix

<a id="pastapastapasta-ci-failing"></a>
#### CI Failing (2)
- [dashpay/platform#5282 perf(platform-wallet)!: prove the shield bundle while waiting for the InstantSend lock](https://github.com/dashpay/platform/pull/5282) — ⚠ merge conflict · 🔴 CI failing · areas: dpp, rs-platform-wallet, rs-platform-wallet-ffi, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 0f547bc62991d50a0a316fc50f712db1da78c9f7
- [dashpay/rust-dashcore#959 feat(dash-spv): adaptive gap-limit probe escalation for BIP44 discovery](https://github.com/dashpay/rust-dashcore/pull/959) — 🔴 CI failing · areas: dash-spv, key-wallet, key-wallet-manager, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed d4aa206c23b6f523d2c51dd5aa3c083c3730a911

<a id="pastapastapasta-draft"></a>
#### Draft (10)
- [dashpay/platform#4648 feat(drive-abci)!: state sync via ABCI snapshots with reduced platform state (protocol v15)](https://github.com/dashpay/platform/pull/4648) — 8 unresolved (8 bot) · 24 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · 📝 draft · areas: rs-drive, rs-drive-abci, dpp, fallback · Policy: draft
  - Top thread: "🟡 Suggestion: Continue the restored target through consensus and a restart**" — 24 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#4965 test(sdk): cargo-fuzz harness over drive-proof-verifier FromProof](https://github.com/dashpay/platform/pull/4965) — 1 unresolved (1 bot) · 23 days stale · 🔴 CI failing · 📝 draft · areas: github, fallback · Policy: draft
  - Top thread: "🟡 Suggestion: Correct the claim that PV13 reaches the V0 proof decoder**" — 23 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5609 fix(sdk): restore JS DAPI client node discovery after Core V24](https://github.com/dashpay/platform/pull/5609) — 2 unresolved (2 bot) · 2 days stale · 📝 draft · areas: fallback · Policy: draft
  - Top thread: "✅ **Resolved** at \`5cf1b7d5\`; see the replies below." — 2 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/grovedb#1007 build(deps)!: use dashpay/orchard with the faster prover](https://github.com/dashpay/grovedb/pull/1007) — 🔴 CI failing · 📝 draft · areas: fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#4633 feat(sdk): add dash-sdk-cxx, a C++ binding of dash-sdk for Dash Core](https://github.com/dashpay/platform/pull/4633) — 📝 draft · areas: rust-sdk, github, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5285 feat(platform)!: subscribe to committed state transitions matching document and address filters](https://github.com/dashpay/platform/pull/5285) — ⚠ merge conflict · 🔴 CI failing · 📝 draft · areas: rs-drive, rs-drive-abci, rust-sdk, dashmate, rust-dapi, github, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5289 feat(sdk)!: optional BIP-39 passphrase through the mnemonic resolver and wallet creation](https://github.com/dashpay/platform/pull/5289) — 🔴 CI failing · 📝 draft · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, rust-sdk-ffi, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5292 feat(dashmate): state sync configuration for tenderdash and drive snapshots](https://github.com/dashpay/platform/pull/5292) — ⚠ merge conflict · 🔴 CI failing · 📝 draft · areas: dashmate · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5475 feat(sdk): forward DashPay profiles, invitations and masternode voting in dash-sdk-cxx](https://github.com/dashpay/platform/pull/5475) — ⚠ merge conflict · 🔴 CI failing · 📝 draft · areas: rs-platform-wallet, rs-platform-wallet-ffi, rust-sdk, github, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5610 perf(sdk): verify each block's signature once instead of on every proved response](https://github.com/dashpay/platform/pull/5610) — 🔴 CI failing · 📝 draft · areas: fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot

<a id="pastapastapasta-stale"></a>
#### Stale (2)
- [dashpay/rust-dashcore#137 fix: integration tests, continued dashificiation](https://github.com/dashpay/rust-dashcore/pull/137) — 2 unresolved (2 CodeRabbit) · 419 days stale · ⚠ merge conflict · 🔴 CI failing · 🐢 targets v0.40-dev, untouched 322 days · areas: fallback
  - Top thread: "_🛠️ Refactor suggestion_" — 419 days old
- [dashpay/platform#5669 feat(sdk): refresh the embedded quorum-proof checkpoints on every release](https://github.com/dashpay/platform/pull/5669) — 5 unresolved (5 bot) · 0 days stale · ✋ changes requested · 🐢 targets feat/verified-quorum-context-provider · areas: fallback
  - Top thread: "🟡 Suggestion: Cap relay responses before buffering them**" — 0 days old

<a id="pastapastapasta-clean"></a>
#### Clean (2)
- [dashpay/platform#5288 feat(platform-wallet)!: support DashPay shielded tips with dedicated accounts](https://github.com/dashpay/platform/pull/5288) — areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk, rust-sdk, kotlin-sdk, fallback · Policy: waiting-author
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/rust-dashcore#264 feat(dash-spv): Add BIP324 v2 encrypted P2P transport](https://github.com/dashpay/rust-dashcore/pull/264) — areas: dash-spv, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 907775512ddd78a9e8e6d2ff12a30cf306ea6c05

<a id="pastapastapasta-ready-for-review"></a>
#### Ready for Review (1)
- [dashpay/platform#4298 fix(dashmate): load ZeroSSL config in force mode](https://github.com/dashpay/platform/pull/4298) — by @thepastaclaw · areas: dashmate · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required

<a id="romchornyi"></a>
### @romchornyi
<a id="romchornyi-open"></a>
#### Open (12)
- [dashpay/platform#5536 fix: check a DashPay invitation claim against Platform before broadcasting it](https://github.com/dashpay/platform/pull/5536) — 7 unresolved (7 bot) · 5 days stale · ⚠ merge conflict · 🔴 CI failing · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, rust-sdk, kotlin-sdk, fallback · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Handle code 63 when the collided key belongs to this wallet**" — 5 days old
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5258 feat(platform-wallet): probe the network about sends with an unknown broadcast outcome](https://github.com/dashpay/platform/pull/5258) — 3 unresolved (3 human) · 13 days stale · ⚠ merge conflict · 🔴 CI failing · 📝 draft · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, rust-sdk · Policy: draft
  - Top thread: "\[P2\] Carry the originating wallet generation in probe events" — 13 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5395 feat(platform-wallet): spend only final coins and report funds waiting on the network](https://github.com/dashpay/platform/pull/5395) — 1 unresolved (1 human) · 13 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · 📝 draft · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk, fallback · Policy: draft
  - Top thread: "this should belong to another layer - likely platform-wallet, or even rust-dashcore's key-wallet. FFI is not meant to st…" — 13 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5549 fix(platform-wallet-storage): restore DashPay contact address pools on SQLite load](https://github.com/dashpay/platform/pull/5549) — 3 unresolved (3 bot) · 6 days stale · ⚠ merge conflict · 🔴 CI failing · 📝 draft · areas: wallet-storage · Policy: draft
  - Top thread: "✅ **Withdrawn** at \`2bee2705\`; see the replies below." — 6 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5540 fix(platform-wallet)!: store externally supplied identity keys without a derivation breadcrumb](https://github.com/dashpay/platform/pull/5540) — 1 unresolved (1 bot) · 7 days stale · 🔴 CI failing · 📝 draft · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk, fallback · Policy: draft
  - Top thread: "🔴 Blocking: Preserve existing derived-key breadcrumbs on disable upserts**" — 7 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5476 feat(platform-wallet)!: report a freshly built asset lock's outpoint to the caller](https://github.com/dashpay/platform/pull/5476) — 1 unresolved (1 bot) · 0 days stale · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, fallback · Policy: waiting-author
  - Top thread: "🔴 Blocking: Do not change existing C export signatures in place**" — 0 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#4992 feat(swift-sdk): durable wallet presence marker readable while the device is locked](https://github.com/dashpay/platform/pull/4992) — 🔴 CI failing · areas: swift-sdk · Policy: ready-to-merge
  - Blocker: All policy requirements are satisfied
- [dashpay/platform#5487 fix(platform-wallet): keep the cached identity in step with the chain around updates](https://github.com/dashpay/platform/pull/5487) — 🔴 CI failing · areas: rs-platform-wallet, wallet-storage · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
  - Blocker: Proceeded without thepastaclaw: skipped by @romchornyi
- [dashpay/platform#5587 fix(platform-wallet)!: keep a contact's payment account across relaunches and replace it cleanly on rotation](https://github.com/dashpay/platform/pull/5587) — ⚠ merge conflict · 🔴 CI failing · 📝 draft · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5589 feat(platform-wallet-ffi)!: let hosts reset an outbound contact account's pool and keep its request digest](https://github.com/dashpay/platform/pull/5589) — 🔴 CI failing · 📝 draft · 🐢 targets fix/contact-rebuild-keeps-pool · areas: rs-platform-wallet-ffi, swift-sdk
- [dashpay/platform#5590 feat(kotlin-sdk)!: reset an outbound contact account's pool and keep its request digest on Android](https://github.com/dashpay/platform/pull/5590) — 🔴 CI failing · 📝 draft · 🐢 targets fix/contact-rebuild-ffi-swift · areas: kotlin-sdk, fallback
- [dashpay/platform#5659 fix(dapi): bound the state transition size pre-filter by the largest known limit](https://github.com/dashpay/platform/pull/5659) — ✋ changes requested · 🔴 CI failing · areas: rust-dapi · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head

<a id="romchornyi-needs-action"></a>
#### Needs action (3)
- [dashpay/platform#5536 fix: check a DashPay invitation claim against Platform before broadcasting it](https://github.com/dashpay/platform/pull/5536) — 7 unresolved (7 bot) · 5 days stale · ⚠ merge conflict · 🔴 CI failing · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, rust-sdk, kotlin-sdk, fallback · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Handle code 63 when the collided key belongs to this wallet**" — 5 days old
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5476 feat(platform-wallet)!: report a freshly built asset lock's outpoint to the caller](https://github.com/dashpay/platform/pull/5476) — 1 unresolved (1 bot) · 0 days stale · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, fallback · Policy: waiting-author
  - Top thread: "🔴 Blocking: Do not change existing C export signatures in place**" — 0 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5659 fix(dapi): bound the state transition size pre-filter by the largest known limit](https://github.com/dashpay/platform/pull/5659) — ✋ changes requested · 🔴 CI failing · areas: rust-dapi · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head

<a id="romchornyi-ready-for-human"></a>
#### Ready for human (2)
- [dashpay/platform#4992 feat(swift-sdk): durable wallet presence marker readable while the device is locked](https://github.com/dashpay/platform/pull/4992) — 🔴 CI failing · areas: swift-sdk · Policy: ready-to-merge
  - Blocker: All policy requirements are satisfied
- [dashpay/platform#5487 fix(platform-wallet): keep the cached identity in step with the chain around updates](https://github.com/dashpay/platform/pull/5487) — 🔴 CI failing · areas: rs-platform-wallet, wallet-storage · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
  - Blocker: Proceeded without thepastaclaw: skipped by @romchornyi

<a id="romchornyi-unresolved-comments"></a>
#### Unresolved Comments (2)
- [dashpay/platform#5536 fix: check a DashPay invitation claim against Platform before broadcasting it](https://github.com/dashpay/platform/pull/5536) — 7 unresolved (7 bot) · 5 days stale · ⚠ merge conflict · 🔴 CI failing · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, rust-sdk, kotlin-sdk, fallback · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Handle code 63 when the collided key belongs to this wallet**" — 5 days old
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5476 feat(platform-wallet)!: report a freshly built asset lock's outpoint to the caller](https://github.com/dashpay/platform/pull/5476) — 1 unresolved (1 bot) · 0 days stale · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, fallback · Policy: waiting-author
  - Top thread: "🔴 Blocking: Do not change existing C export signatures in place**" — 0 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them

<a id="romchornyi-changes-requested"></a>
#### Changes Requested (1)
- [dashpay/platform#5659 fix(dapi): bound the state transition size pre-filter by the largest known limit](https://github.com/dashpay/platform/pull/5659) — ✋ changes requested · 🔴 CI failing · areas: rust-dapi · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head

<a id="romchornyi-ci-failing"></a>
#### CI Failing (2)
- [dashpay/platform#4992 feat(swift-sdk): durable wallet presence marker readable while the device is locked](https://github.com/dashpay/platform/pull/4992) — 🔴 CI failing · areas: swift-sdk · Policy: ready-to-merge
  - Blocker: All policy requirements are satisfied
- [dashpay/platform#5487 fix(platform-wallet): keep the cached identity in step with the chain around updates](https://github.com/dashpay/platform/pull/5487) — 🔴 CI failing · areas: rs-platform-wallet, wallet-storage · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
  - Blocker: Proceeded without thepastaclaw: skipped by @romchornyi

<a id="romchornyi-draft"></a>
#### Draft (5)
- [dashpay/platform#5258 feat(platform-wallet): probe the network about sends with an unknown broadcast outcome](https://github.com/dashpay/platform/pull/5258) — 3 unresolved (3 human) · 13 days stale · ⚠ merge conflict · 🔴 CI failing · 📝 draft · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, rust-sdk · Policy: draft
  - Top thread: "\[P2\] Carry the originating wallet generation in probe events" — 13 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5395 feat(platform-wallet): spend only final coins and report funds waiting on the network](https://github.com/dashpay/platform/pull/5395) — 1 unresolved (1 human) · 13 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · 📝 draft · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk, fallback · Policy: draft
  - Top thread: "this should belong to another layer - likely platform-wallet, or even rust-dashcore's key-wallet. FFI is not meant to st…" — 13 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5549 fix(platform-wallet-storage): restore DashPay contact address pools on SQLite load](https://github.com/dashpay/platform/pull/5549) — 3 unresolved (3 bot) · 6 days stale · ⚠ merge conflict · 🔴 CI failing · 📝 draft · areas: wallet-storage · Policy: draft
  - Top thread: "✅ **Withdrawn** at \`2bee2705\`; see the replies below." — 6 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5540 fix(platform-wallet)!: store externally supplied identity keys without a derivation breadcrumb](https://github.com/dashpay/platform/pull/5540) — 1 unresolved (1 bot) · 7 days stale · 🔴 CI failing · 📝 draft · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk, fallback · Policy: draft
  - Top thread: "🔴 Blocking: Preserve existing derived-key breadcrumbs on disable upserts**" — 7 days old
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5587 fix(platform-wallet)!: keep a contact's payment account across relaunches and replace it cleanly on rotation](https://github.com/dashpay/platform/pull/5587) — ⚠ merge conflict · 🔴 CI failing · 📝 draft · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage · Policy: draft
  - Blocker: Draft PR does not occupy a review slot

<a id="romchornyi-stale"></a>
#### Stale (2)
- [dashpay/platform#5589 feat(platform-wallet-ffi)!: let hosts reset an outbound contact account's pool and keep its request digest](https://github.com/dashpay/platform/pull/5589) — 🔴 CI failing · 📝 draft · 🐢 targets fix/contact-rebuild-keeps-pool · areas: rs-platform-wallet-ffi, swift-sdk
- [dashpay/platform#5590 feat(kotlin-sdk)!: reset an outbound contact account's pool and keep its request digest on Android](https://github.com/dashpay/platform/pull/5590) — 🔴 CI failing · 📝 draft · 🐢 targets fix/contact-rebuild-ffi-swift · areas: kotlin-sdk, fallback

<a id="romchornyi-ready-for-review"></a>
#### Ready for Review (3)
- [dashpay/platform#5288 feat(platform-wallet)!: support DashPay shielded tips with dedicated accounts](https://github.com/dashpay/platform/pull/5288) — by @PastaPastaPasta · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk, rust-sdk, kotlin-sdk, fallback · Policy: waiting-author
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5644 fix!: follow DIP-11 key rules when disabling keys and reading contact requests](https://github.com/dashpay/platform/pull/5644) — by @Claudius-Maginificent · areas: dpp, rs-platform-wallet, rs-platform-wallet-ffi, rust-sdk, js-wasm-sdk, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
- [dashpay/platform#5663 fix(platform-wallet): reopen a latched quiesce gate when a sync loop starts](https://github.com/dashpay/platform/pull/5663) — by @llbartekll · areas: rs-platform-wallet · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head

<a id="lklimek"></a>
### @lklimek
<a id="lklimek-open"></a>
#### Open (14)
- [dashpay/platform#5528 fix(platform-wallet): retain drain operation history through funding conflicts](https://github.com/dashpay/platform/pull/5528) — 7 unresolved (7 bot) · 8 days stale · ✋ changes requested · 📝 draft · 🐢 targets feat/sqlite-wallet-restore · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk, kotlin-sdk
  - Top thread: "🟡 Suggestion: Retain diagnostics for malformed FFI operation snapshots**" — 8 days old
- [dashpay/tenderdash#1515 fix(service)!: join remaining service workers before shutdown completes](https://github.com/dashpay/tenderdash/pull/1515) — 1 unresolved (1 bot) · 6 days stale · ✋ changes requested · areas: fallback · Policy: waiting-author
  - Top thread: "🔴 Blocking: Release node-owned resources when startup fails**" — 6 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/tenderdash#1550 fix(p2p)!: enforce peer penalties and validate network settings](https://github.com/dashpay/tenderdash/pull/1550) — 1 unresolved (1 bot) · 6 days stale · ✋ changes requested · areas: fallback · Policy: waiting-author
  - Top thread: "🔴 Blocking: Do not reject max-connections zero before its runtime default is applied**" — 6 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/dash-evo-tool#1161 fix(ci): isolate release signing and constrain agent write paths](https://github.com/dashpay/dash-evo-tool/pull/1161) — 📝 draft · areas: github, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5150 fix(platform-wallet)!: restore Core spending state and repair persisted accounting](https://github.com/dashpay/platform/pull/5150) — ⏸ deferred · areas: rs-platform-wallet, wallet-storage, fallback · Policy: waiting-author
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5207 fix(platform-wallet): require only atomic tracked-lock writes for reconciliation](https://github.com/dashpay/platform/pull/5207) — ⏸ deferred · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage
- [dashpay/platform#5210 feat(wallet-storage)!: restore complete Core wallet snapshots from SQLite](https://github.com/dashpay/platform/pull/5210) — ⏸ deferred · areas: rs-platform-wallet, wallet-storage
- [dashpay/platform#5220 fix(platform-wallet)!: replay recorded transaction history on load for every persister](https://github.com/dashpay/platform/pull/5220) — ⏸ deferred · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk, fallback
- [dashpay/platform#5323 refactor: reject legacy BLS storage formats](https://github.com/dashpay/platform/pull/5323) — ⚠ merge conflict · 🔴 CI failing · 📝 draft · areas: rs-drive-abci · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5489 fix!: set zero consensus evidence budget at protocol 15](https://github.com/dashpay/platform/pull/5489) — ⚠ merge conflict · 🔴 CI failing · 📝 draft · areas: rs-drive-abci, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5519 refactor: remove DPP raw secp256k1 dependency](https://github.com/dashpay/platform/pull/5519) — 📝 draft · ⏸ deferred · areas: rs-drive-abci, dpp, rs-platform-wallet, rs-platform-wallet-ffi, rust-sdk, rust-sdk-ffi, js-wasm-sdk, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/tenderdash#1552 chore(release)!: prepare dev branch and Platform release workflow](https://github.com/dashpay/tenderdash/pull/1552) — 📝 draft · areas: github, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/tenderdash#1560 test(e2e): exercise block sync during slow local application](https://github.com/dashpay/tenderdash/pull/1560) — areas: github, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 1f5fb2540ca89b3c6b2bb311ad149affdd577df4
- [dashpay/tenderdash#1567 fix(consensus): replay blocks with their historical quorum](https://github.com/dashpay/tenderdash/pull/1567) — 📝 draft · areas: fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot

<a id="lklimek-needs-action"></a>
#### Needs action (2)
- [dashpay/tenderdash#1515 fix(service)!: join remaining service workers before shutdown completes](https://github.com/dashpay/tenderdash/pull/1515) — 1 unresolved (1 bot) · 6 days stale · ✋ changes requested · areas: fallback · Policy: waiting-author
  - Top thread: "🔴 Blocking: Release node-owned resources when startup fails**" — 6 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/tenderdash#1550 fix(p2p)!: enforce peer penalties and validate network settings](https://github.com/dashpay/tenderdash/pull/1550) — 1 unresolved (1 bot) · 6 days stale · ✋ changes requested · areas: fallback · Policy: waiting-author
  - Top thread: "🔴 Blocking: Do not reject max-connections zero before its runtime default is applied**" — 6 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them

<a id="lklimek-unresolved-comments"></a>
#### Unresolved Comments (2)
- [dashpay/tenderdash#1515 fix(service)!: join remaining service workers before shutdown completes](https://github.com/dashpay/tenderdash/pull/1515) — 1 unresolved (1 bot) · 6 days stale · ✋ changes requested · areas: fallback · Policy: waiting-author
  - Top thread: "🔴 Blocking: Release node-owned resources when startup fails**" — 6 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/tenderdash#1550 fix(p2p)!: enforce peer penalties and validate network settings](https://github.com/dashpay/tenderdash/pull/1550) — 1 unresolved (1 bot) · 6 days stale · ✋ changes requested · areas: fallback · Policy: waiting-author
  - Top thread: "🔴 Blocking: Do not reject max-connections zero before its runtime default is applied**" — 6 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them

<a id="lklimek-deferred"></a>
#### Deferred (5)
- [dashpay/platform#5150 fix(platform-wallet)!: restore Core spending state and repair persisted accounting](https://github.com/dashpay/platform/pull/5150) — ⏸ deferred · areas: rs-platform-wallet, wallet-storage, fallback · Policy: waiting-author
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5207 fix(platform-wallet): require only atomic tracked-lock writes for reconciliation](https://github.com/dashpay/platform/pull/5207) — ⏸ deferred · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage
- [dashpay/platform#5210 feat(wallet-storage)!: restore complete Core wallet snapshots from SQLite](https://github.com/dashpay/platform/pull/5210) — ⏸ deferred · areas: rs-platform-wallet, wallet-storage
- [dashpay/platform#5220 fix(platform-wallet)!: replay recorded transaction history on load for every persister](https://github.com/dashpay/platform/pull/5220) — ⏸ deferred · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk, fallback
- [dashpay/platform#5519 refactor: remove DPP raw secp256k1 dependency](https://github.com/dashpay/platform/pull/5519) — 📝 draft · ⏸ deferred · areas: rs-drive-abci, dpp, rs-platform-wallet, rs-platform-wallet-ffi, rust-sdk, rust-sdk-ffi, js-wasm-sdk, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot

<a id="lklimek-draft"></a>
#### Draft (5)
- [dashpay/dash-evo-tool#1161 fix(ci): isolate release signing and constrain agent write paths](https://github.com/dashpay/dash-evo-tool/pull/1161) — 📝 draft · areas: github, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5323 refactor: reject legacy BLS storage formats](https://github.com/dashpay/platform/pull/5323) — ⚠ merge conflict · 🔴 CI failing · 📝 draft · areas: rs-drive-abci · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5489 fix!: set zero consensus evidence budget at protocol 15](https://github.com/dashpay/platform/pull/5489) — ⚠ merge conflict · 🔴 CI failing · 📝 draft · areas: rs-drive-abci, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/tenderdash#1552 chore(release)!: prepare dev branch and Platform release workflow](https://github.com/dashpay/tenderdash/pull/1552) — 📝 draft · areas: github, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/tenderdash#1567 fix(consensus): replay blocks with their historical quorum](https://github.com/dashpay/tenderdash/pull/1567) — 📝 draft · areas: fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot

<a id="lklimek-stale"></a>
#### Stale (1)
- [dashpay/platform#5528 fix(platform-wallet): retain drain operation history through funding conflicts](https://github.com/dashpay/platform/pull/5528) — 7 unresolved (7 bot) · 8 days stale · ✋ changes requested · 📝 draft · 🐢 targets feat/sqlite-wallet-restore · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk, kotlin-sdk
  - Top thread: "🟡 Suggestion: Retain diagnostics for malformed FFI operation snapshots**" — 8 days old

<a id="lklimek-clean"></a>
#### Clean (1)
- [dashpay/tenderdash#1560 test(e2e): exercise block sync during slow local application](https://github.com/dashpay/tenderdash/pull/1560) — areas: github, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 1f5fb2540ca89b3c6b2bb311ad149affdd577df4

<a id="lklimek-ready-for-review"></a>
#### Ready for Review (6)
- [dashpay/dash-evo-tool#1109 feat(masternode): use wallet-held node keys and withdraw via platform-wallet](https://github.com/dashpay/dash-evo-tool/pull/1109) — by @Claudius-Maginificent · areas: fallback · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/dash-evo-tool#1149 fix(identity): refresh the identity's balance after a paid operation](https://github.com/dashpay/dash-evo-tool/pull/1149) — by @Claudius-Maginificent · areas: fallback · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head
- [dashpay/dash-evo-tool#1157 refactor(app): let each domain decide its reactions to backend results](https://github.com/dashpay/dash-evo-tool/pull/1157) — by @Claudius-Maginificent · areas: fallback · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head
  - Blocker: Proceeded without coderabbitai: it reported a rate limit and did not return
- [dashpay/platform#5288 feat(platform-wallet)!: support DashPay shielded tips with dedicated accounts](https://github.com/dashpay/platform/pull/5288) — by @PastaPastaPasta · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk, rust-sdk, kotlin-sdk, fallback · Policy: waiting-author
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5668 fix(sdk): do not ban nodes when the SDK has no context provider](https://github.com/dashpay/platform/pull/5668) — by @llbartekll · areas: rust-sdk · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head
- [dashpay/tenderdash#1572 feat(consensus)!: hand a silent proposer's idle round to the next proposer](https://github.com/dashpay/tenderdash/pull/1572) — by @QuantumExplorer · areas: fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 93aa98de29832ad3b89ac2753d05ce5880cb4148

<a id="bfoss765"></a>
### @bfoss765
<a id="bfoss765-open"></a>
#### Open (2)
- [dashpay/platform#4313 feat(kotlin-sdk): one-time Orchard key shielded-invite API (inviter + claim)](https://github.com/dashpay/platform/pull/4313) — 2 unresolved (2 bot) · 59 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk, fallback · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Preserve claim-binding mismatch semantics across the FFI boundary**" — 59 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#4312 feat(platform-wallet): multi-output shielded transfers + output-aware fee predictor](https://github.com/dashpay/platform/pull/4312) — 1 unresolved (1 bot) · 59 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · areas: dpp, rs-platform-wallet, rs-platform-wallet-ffi, kotlin-sdk, fallback · Policy: waiting-self-review
  - Top thread: "🟡 Suggestion: Keep envelope-aware action counting behind the crate boundary**" — 59 days old
  - Blocker: Author must post /self-reviewed da932f96a8670c641065954de8b58752e99233a3

<a id="bfoss765-needs-action"></a>
#### Needs action (2)
- [dashpay/platform#4313 feat(kotlin-sdk): one-time Orchard key shielded-invite API (inviter + claim)](https://github.com/dashpay/platform/pull/4313) — 2 unresolved (2 bot) · 59 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk, fallback · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Preserve claim-binding mismatch semantics across the FFI boundary**" — 59 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#4312 feat(platform-wallet): multi-output shielded transfers + output-aware fee predictor](https://github.com/dashpay/platform/pull/4312) — 1 unresolved (1 bot) · 59 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · areas: dpp, rs-platform-wallet, rs-platform-wallet-ffi, kotlin-sdk, fallback · Policy: waiting-self-review
  - Top thread: "🟡 Suggestion: Keep envelope-aware action counting behind the crate boundary**" — 59 days old
  - Blocker: Author must post /self-reviewed da932f96a8670c641065954de8b58752e99233a3

<a id="bfoss765-unresolved-comments"></a>
#### Unresolved Comments (2)
- [dashpay/platform#4313 feat(kotlin-sdk): one-time Orchard key shielded-invite API (inviter + claim)](https://github.com/dashpay/platform/pull/4313) — 2 unresolved (2 bot) · 59 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk, fallback · Policy: waiting-author
  - Top thread: "🟡 Suggestion: Preserve claim-binding mismatch semantics across the FFI boundary**" — 59 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#4312 feat(platform-wallet): multi-output shielded transfers + output-aware fee predictor](https://github.com/dashpay/platform/pull/4312) — 1 unresolved (1 bot) · 59 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · areas: dpp, rs-platform-wallet, rs-platform-wallet-ffi, kotlin-sdk, fallback · Policy: waiting-self-review
  - Top thread: "🟡 Suggestion: Keep envelope-aware action counting behind the crate boundary**" — 59 days old
  - Blocker: Author must post /self-reviewed da932f96a8670c641065954de8b58752e99233a3

<a id="infraclaw-dash"></a>
### @infraclaw-dash
<a id="infraclaw-dash-open"></a>
#### Open (3)
- [dashpay/platform#5167 ci: align secp feature-base workflows with provisioned runner images](https://github.com/dashpay/platform/pull/5167) — 3 unresolved (3 bot) · 31 days stale · ⚠ merge conflict · 🐢 targets chore/bump-rust-dashcore-secp-033 · areas: github
  - Top thread: "🟡 Suggestion: Contract-failure error points to a doc file missing from this branch**" — 31 days old
- [dashpay/platform#3958 ci: add Platform testnet sync status reporting](https://github.com/dashpay/platform/pull/3958) — 2 unresolved (2 bot) · 127 days stale · ⚠ merge conflict · 🔴 CI failing · areas: github, fallback · Policy: waiting-author
  - Top thread: "🟡 Suggestion: workflow_dispatch lets any repo writer forge a sync status**" — 127 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: coderabbitai left review threads unresolved; resolve them
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5166 ci: adopt verified Rust and Kotlin runner images on v4.3](https://github.com/dashpay/platform/pull/5166) — ⚠ merge conflict · 🔴 CI failing · areas: github · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required

<a id="infraclaw-dash-needs-action"></a>
#### Needs action (2)
- [dashpay/platform#3958 ci: add Platform testnet sync status reporting](https://github.com/dashpay/platform/pull/3958) — 2 unresolved (2 bot) · 127 days stale · ⚠ merge conflict · 🔴 CI failing · areas: github, fallback · Policy: waiting-author
  - Top thread: "🟡 Suggestion: workflow_dispatch lets any repo writer forge a sync status**" — 127 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: coderabbitai left review threads unresolved; resolve them
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5166 ci: adopt verified Rust and Kotlin runner images on v4.3](https://github.com/dashpay/platform/pull/5166) — ⚠ merge conflict · 🔴 CI failing · areas: github · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required

<a id="infraclaw-dash-ready-for-human"></a>
#### Ready for human (1)
- [dashpay/platform#5166 ci: adopt verified Rust and Kotlin runner images on v4.3](https://github.com/dashpay/platform/pull/5166) — ⚠ merge conflict · 🔴 CI failing · areas: github · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required

<a id="infraclaw-dash-unresolved-comments"></a>
#### Unresolved Comments (1)
- [dashpay/platform#3958 ci: add Platform testnet sync status reporting](https://github.com/dashpay/platform/pull/3958) — 2 unresolved (2 bot) · 127 days stale · ⚠ merge conflict · 🔴 CI failing · areas: github, fallback · Policy: waiting-author
  - Top thread: "🟡 Suggestion: workflow_dispatch lets any repo writer forge a sync status**" — 127 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: coderabbitai left review threads unresolved; resolve them
  - Blocker: thepastaclaw left review threads unresolved; resolve them

<a id="infraclaw-dash-ci-failing"></a>
#### CI Failing (1)
- [dashpay/platform#5166 ci: adopt verified Rust and Kotlin runner images on v4.3](https://github.com/dashpay/platform/pull/5166) — ⚠ merge conflict · 🔴 CI failing · areas: github · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required

<a id="infraclaw-dash-stale"></a>
#### Stale (1)
- [dashpay/platform#5167 ci: align secp feature-base workflows with provisioned runner images](https://github.com/dashpay/platform/pull/5167) — 3 unresolved (3 bot) · 31 days stale · ⚠ merge conflict · 🐢 targets chore/bump-rust-dashcore-secp-033 · areas: github
  - Top thread: "🟡 Suggestion: Contract-failure error points to a doc file missing from this branch**" — 31 days old

<a id="hashengineering"></a>
### @HashEngineering
<a id="hashengineering-open"></a>
#### Open (3)
- [dashpay/platform#5259 fix(kotlin-sdk): port #4638 to Android — credit verdicts at save time and the post-sync TXO reconcile](https://github.com/dashpay/platform/pull/5259) — 2 unresolved (2 bot) · 20 days stale · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk, fallback · Policy: waiting-self-review
  - Top thread: "🟡 Suggestion: Add an index matching the unspent pagination query**" — 20 days old
  - Blocker: Author must post /self-reviewed cec231b54cfd36a4b9d8cbbe45aeae28af4c8498
- [dashpay/platform#5256 fix(platform-wallet): build our receiving account for one-way DashPay contacts](https://github.com/dashpay/platform/pull/5256) — ⚠ merge conflict · 🔴 CI failing · 📝 draft · areas: rs-platform-wallet · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/platform#5479 feat(platform-wallet-storage): persist DashPay backfill coverage and outbound-account markers](https://github.com/dashpay/platform/pull/5479) — 🔴 CI failing · areas: rs-platform-wallet, wallet-storage · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required

<a id="hashengineering-needs-action"></a>
#### Needs action (1)
- [dashpay/platform#5259 fix(kotlin-sdk): port #4638 to Android — credit verdicts at save time and the post-sync TXO reconcile](https://github.com/dashpay/platform/pull/5259) — 2 unresolved (2 bot) · 20 days stale · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk, fallback · Policy: waiting-self-review
  - Top thread: "🟡 Suggestion: Add an index matching the unspent pagination query**" — 20 days old
  - Blocker: Author must post /self-reviewed cec231b54cfd36a4b9d8cbbe45aeae28af4c8498

<a id="hashengineering-ready-for-human"></a>
#### Ready for human (1)
- [dashpay/platform#5479 feat(platform-wallet-storage): persist DashPay backfill coverage and outbound-account markers](https://github.com/dashpay/platform/pull/5479) — 🔴 CI failing · areas: rs-platform-wallet, wallet-storage · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required

<a id="hashengineering-unresolved-comments"></a>
#### Unresolved Comments (1)
- [dashpay/platform#5259 fix(kotlin-sdk): port #4638 to Android — credit verdicts at save time and the post-sync TXO reconcile](https://github.com/dashpay/platform/pull/5259) — 2 unresolved (2 bot) · 20 days stale · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, kotlin-sdk, fallback · Policy: waiting-self-review
  - Top thread: "🟡 Suggestion: Add an index matching the unspent pagination query**" — 20 days old
  - Blocker: Author must post /self-reviewed cec231b54cfd36a4b9d8cbbe45aeae28af4c8498

<a id="hashengineering-ci-failing"></a>
#### CI Failing (1)
- [dashpay/platform#5479 feat(platform-wallet-storage): persist DashPay backfill coverage and outbound-account markers](https://github.com/dashpay/platform/pull/5479) — 🔴 CI failing · areas: rs-platform-wallet, wallet-storage · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required

<a id="hashengineering-draft"></a>
#### Draft (1)
- [dashpay/platform#5256 fix(platform-wallet): build our receiving account for one-way DashPay contacts](https://github.com/dashpay/platform/pull/5256) — ⚠ merge conflict · 🔴 CI failing · 📝 draft · areas: rs-platform-wallet · Policy: draft
  - Blocker: Draft PR does not occupy a review slot

<a id="hashengineering-ready-for-review"></a>
#### Ready for Review (3)
- [dashpay/platform#5288 feat(platform-wallet)!: support DashPay shielded tips with dedicated accounts](https://github.com/dashpay/platform/pull/5288) — by @PastaPastaPasta · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk, rust-sdk, kotlin-sdk, fallback · Policy: waiting-author
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5644 fix!: follow DIP-11 key rules when disabling keys and reading contact requests](https://github.com/dashpay/platform/pull/5644) — by @Claudius-Maginificent · areas: dpp, rs-platform-wallet, rs-platform-wallet-ffi, rust-sdk, js-wasm-sdk, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
- [dashpay/platform#5663 fix(platform-wallet): reopen a latched quiesce gate when a sync loop starts](https://github.com/dashpay/platform/pull/5663) — by @llbartekll · areas: rs-platform-wallet · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head

<a id="llbartekll"></a>
### @llbartekll
<a id="llbartekll-open"></a>
#### Open (4)
- [dashpay/platform#5556 fix(sdk)!: preserve uncertain submission results for recovery](https://github.com/dashpay/platform/pull/5556) — 3 unresolved (1 CodeRabbit, 2 bot) · 0 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, rust-sdk, rust-sdk-ffi, js-wasm-sdk, kotlin-sdk, fallback · Policy: waiting-author
  - Top thread: "&lt;picture&gt;&lt;img src="https://storage.googleapis.com/coderabbit_public_assets/review-comment-icons-v1/amber-16.svg" width="…" — 0 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: coderabbitai left review threads unresolved; resolve them
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5664 fix(platform-wallet): let a shielded sync pass observe cancellation](https://github.com/dashpay/platform/pull/5664) — 3 unresolved (3 bot) · 0 days stale · 🐢 targets fix/reopen-quiesce-gate-on-start · areas: rs-platform-wallet
  - Top thread: "✅ **Withdrawn** at \`7e1fac2a\`; see the replies below." — 0 days old
- [dashpay/platform#5663 fix(platform-wallet): reopen a latched quiesce gate when a sync loop starts](https://github.com/dashpay/platform/pull/5663) — areas: rs-platform-wallet · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head
- [dashpay/platform#5668 fix(sdk): do not ban nodes when the SDK has no context provider](https://github.com/dashpay/platform/pull/5668) — areas: rust-sdk · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head

<a id="llbartekll-needs-action"></a>
#### Needs action (1)
- [dashpay/platform#5556 fix(sdk)!: preserve uncertain submission results for recovery](https://github.com/dashpay/platform/pull/5556) — 3 unresolved (1 CodeRabbit, 2 bot) · 0 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, rust-sdk, rust-sdk-ffi, js-wasm-sdk, kotlin-sdk, fallback · Policy: waiting-author
  - Top thread: "&lt;picture&gt;&lt;img src="https://storage.googleapis.com/coderabbit_public_assets/review-comment-icons-v1/amber-16.svg" width="…" — 0 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: coderabbitai left review threads unresolved; resolve them
  - Blocker: thepastaclaw left review threads unresolved; resolve them

<a id="llbartekll-unresolved-comments"></a>
#### Unresolved Comments (1)
- [dashpay/platform#5556 fix(sdk)!: preserve uncertain submission results for recovery](https://github.com/dashpay/platform/pull/5556) — 3 unresolved (1 CodeRabbit, 2 bot) · 0 days stale · ⚠ merge conflict · ✋ changes requested · 🔴 CI failing · areas: rs-platform-wallet, rs-platform-wallet-ffi, swift-sdk, rust-sdk, rust-sdk-ffi, js-wasm-sdk, kotlin-sdk, fallback · Policy: waiting-author
  - Top thread: "&lt;picture&gt;&lt;img src="https://storage.googleapis.com/coderabbit_public_assets/review-comment-icons-v1/amber-16.svg" width="…" — 0 days old
  - Blocker: thepastaclaw requested changes on this head; dismiss the review or push a fix
  - Blocker: coderabbitai left review threads unresolved; resolve them
  - Blocker: thepastaclaw left review threads unresolved; resolve them

<a id="llbartekll-stale"></a>
#### Stale (1)
- [dashpay/platform#5664 fix(platform-wallet): let a shielded sync pass observe cancellation](https://github.com/dashpay/platform/pull/5664) — 3 unresolved (3 bot) · 0 days stale · 🐢 targets fix/reopen-quiesce-gate-on-start · areas: rs-platform-wallet
  - Top thread: "✅ **Withdrawn** at \`7e1fac2a\`; see the replies below." — 0 days old

<a id="llbartekll-clean"></a>
#### Clean (2)
- [dashpay/platform#5663 fix(platform-wallet): reopen a latched quiesce gate when a sync loop starts](https://github.com/dashpay/platform/pull/5663) — areas: rs-platform-wallet · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head
- [dashpay/platform#5668 fix(sdk): do not ban nodes when the SDK has no context provider](https://github.com/dashpay/platform/pull/5668) — areas: rust-sdk · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head

<a id="llbartekll-ready-for-review"></a>
#### Ready for Review (2)
- [dashpay/platform#5288 feat(platform-wallet)!: support DashPay shielded tips with dedicated accounts](https://github.com/dashpay/platform/pull/5288) — by @PastaPastaPasta · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk, rust-sdk, kotlin-sdk, fallback · Policy: waiting-author
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5644 fix!: follow DIP-11 key rules when disabling keys and reading contact requests](https://github.com/dashpay/platform/pull/5644) — by @Claudius-Maginificent · areas: dpp, rs-platform-wallet, rs-platform-wallet-ffi, rust-sdk, js-wasm-sdk, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required

<a id="xdustinface"></a>
### @xdustinface
<a id="xdustinface-open"></a>
#### Open (5)
- [dashpay/rust-dashcore#856 feat(dash-spv): force resync to re-anchor a running client at a lower checkpoint](https://github.com/dashpay/rust-dashcore/pull/856) — 1 unresolved (1 human) · 100 days stale · ⚠ merge conflict · 🔴 CI failing · areas: dash-spv · Policy: waiting-self-review
  - Top thread: "why do we have a periodic check here??" — 100 days old
  - Blocker: Author must post /self-reviewed 7282172a3bb4a0d9d85a40cf70b4dd111995f96b
- [dashpay/rust-dashcore#400 chore: remove BIP141 (SegWit) and BIP341 (Taproot) support](https://github.com/dashpay/rust-dashcore/pull/400) — ⚠ merge conflict · 🔴 CI failing · 📝 draft · 🐢 untouched 163 days · areas: dash-spv, key-wallet, github, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#541 ci: integrate Codecov test analytics via \`cargo-nextest\`](https://github.com/dashpay/rust-dashcore/pull/541) — ⚠ merge conflict · 📝 draft · areas: github, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#776 chore: remove FFI layer](https://github.com/dashpay/rust-dashcore/pull/776) — ⚠ merge conflict · 📝 draft · areas: dash-spv, key-wallet, github, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#803 feat(dash-spv): block locator + staged fork detection](https://github.com/dashpay/rust-dashcore/pull/803) — ⚠ merge conflict · 📝 draft · areas: dash-spv, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot

<a id="xdustinface-needs-action"></a>
#### Needs action (1)
- [dashpay/rust-dashcore#856 feat(dash-spv): force resync to re-anchor a running client at a lower checkpoint](https://github.com/dashpay/rust-dashcore/pull/856) — 1 unresolved (1 human) · 100 days stale · ⚠ merge conflict · 🔴 CI failing · areas: dash-spv · Policy: waiting-self-review
  - Top thread: "why do we have a periodic check here??" — 100 days old
  - Blocker: Author must post /self-reviewed 7282172a3bb4a0d9d85a40cf70b4dd111995f96b

<a id="xdustinface-unresolved-comments"></a>
#### Unresolved Comments (1)
- [dashpay/rust-dashcore#856 feat(dash-spv): force resync to re-anchor a running client at a lower checkpoint](https://github.com/dashpay/rust-dashcore/pull/856) — 1 unresolved (1 human) · 100 days stale · ⚠ merge conflict · 🔴 CI failing · areas: dash-spv · Policy: waiting-self-review
  - Top thread: "why do we have a periodic check here??" — 100 days old
  - Blocker: Author must post /self-reviewed 7282172a3bb4a0d9d85a40cf70b4dd111995f96b

<a id="xdustinface-draft"></a>
#### Draft (3)
- [dashpay/rust-dashcore#541 ci: integrate Codecov test analytics via \`cargo-nextest\`](https://github.com/dashpay/rust-dashcore/pull/541) — ⚠ merge conflict · 📝 draft · areas: github, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#776 chore: remove FFI layer](https://github.com/dashpay/rust-dashcore/pull/776) — ⚠ merge conflict · 📝 draft · areas: dash-spv, key-wallet, github, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#803 feat(dash-spv): block locator + staged fork detection](https://github.com/dashpay/rust-dashcore/pull/803) — ⚠ merge conflict · 📝 draft · areas: dash-spv, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot

<a id="xdustinface-stale"></a>
#### Stale (1)
- [dashpay/rust-dashcore#400 chore: remove BIP141 (SegWit) and BIP341 (Taproot) support](https://github.com/dashpay/rust-dashcore/pull/400) — ⚠ merge conflict · 🔴 CI failing · 📝 draft · 🐢 untouched 163 days · areas: dash-spv, key-wallet, github, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot

<a id="xdustinface-ready-for-review"></a>
#### Ready for Review (1)
- [dashpay/rust-dashcore#264 feat(dash-spv): Add BIP324 v2 encrypted P2P transport](https://github.com/dashpay/rust-dashcore/pull/264) — by @PastaPastaPasta · areas: dash-spv, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 907775512ddd78a9e8e6d2ff12a30cf306ea6c05

<a id="zocolini"></a>
### @ZocoLini
<a id="zocolini-open"></a>
#### Open (5)
- [dashpay/rust-dashcore#921 fix(dash-spv): sync backfill](https://github.com/dashpay/rust-dashcore/pull/921) — ⚠ merge conflict · 🔴 CI failing · areas: dash-spv, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 28a777837b8b897e5eb7fb678c66f44df000ccf0
- [dashpay/rust-dashcore#1181 fix(dash-spv)!: bound the MessageDispatcher subscriber queues](https://github.com/dashpay/rust-dashcore/pull/1181) — ⚠ merge conflict · 🔴 CI failing · 📝 draft · areas: dash-spv · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#1199 feat(key-wallet)!: only final coins fund a build](https://github.com/dashpay/rust-dashcore/pull/1199) — ⚠ merge conflict · 📝 draft · areas: dash-spv, key-wallet, key-wallet-manager · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#1202 feat(dash-spv): expose the coinbase payload of the newest validated diff](https://github.com/dashpay/rust-dashcore/pull/1202) — ⚠ merge conflict · 🔴 CI failing · 📝 draft · areas: dash-spv, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#1248 chore: deny \`unnameable_types\` across the workspace](https://github.com/dashpay/rust-dashcore/pull/1248) — ⚠ merge conflict · 📝 draft · areas: dash-spv, key-wallet, key-wallet-manager, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot

<a id="zocolini-needs-action"></a>
#### Needs action (1)
- [dashpay/rust-dashcore#921 fix(dash-spv): sync backfill](https://github.com/dashpay/rust-dashcore/pull/921) — ⚠ merge conflict · 🔴 CI failing · areas: dash-spv, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 28a777837b8b897e5eb7fb678c66f44df000ccf0

<a id="zocolini-ci-failing"></a>
#### CI Failing (1)
- [dashpay/rust-dashcore#921 fix(dash-spv): sync backfill](https://github.com/dashpay/rust-dashcore/pull/921) — ⚠ merge conflict · 🔴 CI failing · areas: dash-spv, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 28a777837b8b897e5eb7fb678c66f44df000ccf0

<a id="zocolini-draft"></a>
#### Draft (4)
- [dashpay/rust-dashcore#1181 fix(dash-spv)!: bound the MessageDispatcher subscriber queues](https://github.com/dashpay/rust-dashcore/pull/1181) — ⚠ merge conflict · 🔴 CI failing · 📝 draft · areas: dash-spv · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#1199 feat(key-wallet)!: only final coins fund a build](https://github.com/dashpay/rust-dashcore/pull/1199) — ⚠ merge conflict · 📝 draft · areas: dash-spv, key-wallet, key-wallet-manager · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#1202 feat(dash-spv): expose the coinbase payload of the newest validated diff](https://github.com/dashpay/rust-dashcore/pull/1202) — ⚠ merge conflict · 🔴 CI failing · 📝 draft · areas: dash-spv, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot
- [dashpay/rust-dashcore#1248 chore: deny \`unnameable_types\` across the workspace](https://github.com/dashpay/rust-dashcore/pull/1248) — ⚠ merge conflict · 📝 draft · areas: dash-spv, key-wallet, key-wallet-manager, fallback · Policy: draft
  - Blocker: Draft PR does not occupy a review slot

<a id="zocolini-ready-for-review"></a>
#### Ready for Review (4)
- [dashpay/platform#5288 feat(platform-wallet)!: support DashPay shielded tips with dedicated accounts](https://github.com/dashpay/platform/pull/5288) — by @PastaPastaPasta · areas: rs-platform-wallet, rs-platform-wallet-ffi, wallet-storage, swift-sdk, rust-sdk, kotlin-sdk, fallback · Policy: waiting-author
  - Blocker: thepastaclaw left review threads unresolved; resolve them
- [dashpay/platform#5644 fix!: follow DIP-11 key rules when disabling keys and reading contact requests](https://github.com/dashpay/platform/pull/5644) — by @Claudius-Maginificent · areas: dpp, rs-platform-wallet, rs-platform-wallet-ffi, rust-sdk, js-wasm-sdk, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
- [dashpay/platform#5663 fix(platform-wallet): reopen a latched quiesce gate when a sync loop starts](https://github.com/dashpay/platform/pull/5663) — by @llbartekll · areas: rs-platform-wallet · Policy: waiting-bots
  - Blocker: thepastaclaw has not reported for the current head
- [dashpay/rust-dashcore#264 feat(dash-spv): Add BIP324 v2 encrypted P2P transport](https://github.com/dashpay/rust-dashcore/pull/264) — by @PastaPastaPasta · areas: dash-spv, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 907775512ddd78a9e8e6d2ff12a30cf306ea6c05

<a id="ktechmidas"></a>
### @ktechmidas
<a id="ktechmidas-open"></a>
#### Open (6)
- [dashpay/platform#5188 ci: tolerate optional S3 cache export outages on chore/bump-rust-dashcore-secp-033](https://github.com/dashpay/platform/pull/5188) — 1 unresolved (1 bot) · 30 days stale · 🐢 targets chore/bump-rust-dashcore-secp-033 · areas: github
  - Top thread: "🟡 Suggestion: New regression tests are not executed by any CI workflow on this branch**" — 30 days old
- [dashpay/platform#5186 ci: tolerate optional S3 cache export outages on v5.0-dev](https://github.com/dashpay/platform/pull/5186) — 🔴 CI failing · 🐢 targets v6.0-dev · areas: github · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 843052c2dce26a80065c15b1219bbc8211264161
- [dashpay/platform#5187 ci: tolerate optional S3 cache export outages on v4.3-dev](https://github.com/dashpay/platform/pull/5187) — 🔴 CI failing · areas: github · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed fbbf13ee41f8bd8b9f03f1dc5afec6a619967d97
- [dashpay/platform#5198 ci: bound default Binaryen optimizer threads in NPM builds](https://github.com/dashpay/platform/pull/5198) — 🔴 CI failing · 🐢 targets v6.0-dev · areas: github · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 23b41b477bc038965b686b4a75412b1c21fc6d98
- [dashpay/platform#5204 fix(release): install verified cargo-binstall binary directly](https://github.com/dashpay/platform/pull/5204) — ⚠ merge conflict · 🔴 CI failing · areas: fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 63455a2563fd7271fb11083d3b0a89bdf01f3982
- [dashpay/platform#5205 fix(release): install verified cargo-binstall binary directly](https://github.com/dashpay/platform/pull/5205) — 🔴 CI failing · 🐢 targets v6.0-dev · areas: fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 8a8ac2c788b5338416e18045001d9115ba3ac80a

<a id="ktechmidas-needs-action"></a>
#### Needs action (1)
- [dashpay/platform#5204 fix(release): install verified cargo-binstall binary directly](https://github.com/dashpay/platform/pull/5204) — ⚠ merge conflict · 🔴 CI failing · areas: fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 63455a2563fd7271fb11083d3b0a89bdf01f3982

<a id="ktechmidas-ci-failing"></a>
#### CI Failing (2)
- [dashpay/platform#5187 ci: tolerate optional S3 cache export outages on v4.3-dev](https://github.com/dashpay/platform/pull/5187) — 🔴 CI failing · areas: github · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed fbbf13ee41f8bd8b9f03f1dc5afec6a619967d97
- [dashpay/platform#5204 fix(release): install verified cargo-binstall binary directly](https://github.com/dashpay/platform/pull/5204) — ⚠ merge conflict · 🔴 CI failing · areas: fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 63455a2563fd7271fb11083d3b0a89bdf01f3982

<a id="ktechmidas-stale"></a>
#### Stale (4)
- [dashpay/platform#5188 ci: tolerate optional S3 cache export outages on chore/bump-rust-dashcore-secp-033](https://github.com/dashpay/platform/pull/5188) — 1 unresolved (1 bot) · 30 days stale · 🐢 targets chore/bump-rust-dashcore-secp-033 · areas: github
  - Top thread: "🟡 Suggestion: New regression tests are not executed by any CI workflow on this branch**" — 30 days old
- [dashpay/platform#5186 ci: tolerate optional S3 cache export outages on v5.0-dev](https://github.com/dashpay/platform/pull/5186) — 🔴 CI failing · 🐢 targets v6.0-dev · areas: github · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 843052c2dce26a80065c15b1219bbc8211264161
- [dashpay/platform#5198 ci: bound default Binaryen optimizer threads in NPM builds](https://github.com/dashpay/platform/pull/5198) — 🔴 CI failing · 🐢 targets v6.0-dev · areas: github · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 23b41b477bc038965b686b4a75412b1c21fc6d98
- [dashpay/platform#5205 fix(release): install verified cargo-binstall binary directly](https://github.com/dashpay/platform/pull/5205) — 🔴 CI failing · 🐢 targets v6.0-dev · areas: fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 8a8ac2c788b5338416e18045001d9115ba3ac80a

<a id="ktechmidas-ready-for-review"></a>
#### Ready for Review (3)
- [dashpay/platform#4298 fix(dashmate): load ZeroSSL config in force mode](https://github.com/dashpay/platform/pull/4298) — by @thepastaclaw · areas: dashmate · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
- [dashpay/platform#5653 fix(dashmate): stop the GitHub release lookup throwing and validate its version](https://github.com/dashpay/platform/pull/5653) — by @shumkov · areas: dashmate · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed dc05e4eb7ae997febcb7b1bc9f06625b1504b6b4
- [dashpay/tenderdash#1560 test(e2e): exercise block sync during slow local application](https://github.com/dashpay/tenderdash/pull/1560) — by @lklimek · areas: github, fallback · Policy: waiting-self-review
  - Blocker: Author must post /self-reviewed 1f5fb2540ca89b3c6b2bb311ad149affdd577df4

<a id="dcg-claude"></a>
### @DCG-Claude
<a id="dcg-claude-open"></a>
#### Open (19)
- [dashpay/platform#4703 fix(platform): resolve fee versions by registered number and price refunds at the storage epoch rate](https://github.com/dashpay/platform/pull/4703) — areas: rs-drive, rs-drive-abci, dpp, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
- [dashpay/platform#4704 fix(platform): record the genesis fee generation and add replay coverage across a fee-version boundary](https://github.com/dashpay/platform/pull/4704) — 🔴 CI failing · areas: rs-drive-abci, fallback · Policy: waiting-build
  - Blocker: The build must pass before a human is asked
- [dashpay/platform#4705 feat(platform): define protocol-versioned smart-contract computation limits and their gas representation](https://github.com/dashpay/platform/pull/4705) — 🐢 targets v6.0-dev · areas: rs-drive, rs-drive-abci, dpp, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
- [dashpay/platform#4706 feat(platform)!: require fee history for storage refunds and credit their recorded owners](https://github.com/dashpay/platform/pull/4706) — areas: rs-drive, rs-drive-abci, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
- [dashpay/platform#4707 feat: alloc-only profiles for platform-value and platform-serialization](https://github.com/dashpay/platform/pull/4707) — 🔴 CI failing · areas: github, fallback · Policy: waiting-build
  - Blocker: The build must pass before a human is asked
- [dashpay/platform#4712 feat(platform): add the dashvm-validation crate and the DashVM protocol table](https://github.com/dashpay/platform/pull/4712) — ⚠ merge conflict · 🔴 CI failing · 🐢 targets v6.0-dev · areas: github, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
- [dashpay/platform#4716 feat(drive)!: add the contract credits root sum tree to genesis, upgrade and credit conservation](https://github.com/dashpay/platform/pull/4716) — 🔴 CI failing · 🐢 targets v6.0-dev · areas: rs-drive, rs-drive-abci, dpp, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
- [dashpay/platform#4717 feat(platform)!: add family-specific state transition size limits and the large contract envelope decode path](https://github.com/dashpay/platform/pull/4717) — 🔴 CI failing · 🐢 targets v6.0-dev · areas: rs-drive, rs-drive-abci, dpp, dashmate, rust-dapi, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
- [dashpay/platform#4718 ci: compare cross-architecture replay artifacts across a protocol upgrade and a restart](https://github.com/dashpay/platform/pull/4718) — 🔴 CI failing · 🐢 targets v6.0-dev · areas: rs-drive-abci, github, fallback · Policy: waiting-build
  - Blocker: The build must pass before a human is asked
- [dashpay/platform#4719 feat: add the dash-sdk-contract declaration model, grammar and diagnostics](https://github.com/dashpay/platform/pull/4719) — 🔴 CI failing · areas: github, fallback · Policy: waiting-build
  - Blocker: The build must pass before a human is asked
- [dashpay/platform#4784 feat(platform)!: validate contested index parameters and bind the native award to its poll](https://github.com/dashpay/platform/pull/4784) — ⚠ merge conflict · 🔴 CI failing · 🐢 targets v6.0-dev · areas: rs-drive, rs-drive-abci, dpp, fallback · Policy: waiting-build
  - Blocker: The build must pass before a human is asked
- [dashpay/platform#4789 feat(platform): add typed refund owners to storage flags and fee refunds](https://github.com/dashpay/platform/pull/4789) — 🔴 CI failing · 🐢 targets v6.0-dev · areas: rs-drive, dpp, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
- [dashpay/platform#4790 feat(drive)!: keep per-issuer token supply rollups and a destroyed-issuer ledger](https://github.com/dashpay/platform/pull/4790) — ⚠ merge conflict · 🔴 CI failing · 🐢 targets v6.0-dev · areas: rs-drive, rs-drive-abci, dpp, fallback · Policy: waiting-build
  - Blocker: The build must pass before a human is asked
- [dashpay/platform#4794 feat: pin contract configuration vectors and mirror them in wasm-dpp2 and the sdk ffi](https://github.com/dashpay/platform/pull/4794) — 🔴 CI failing · areas: dpp, rust-sdk-ffi, fallback · Policy: waiting-build
  - Blocker: The build must pass before a human is asked
- [dashpay/platform#4808 test(drive-abci): rehearse a reproducible scheduled host fault and the existing recovery path](https://github.com/dashpay/platform/pull/4808) — 🔴 CI failing · 🐢 targets v6.0-dev · areas: rs-drive-abci, github, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
- [dashpay/platform#4934 docs: correct the smart-contract target, version counts, transition list and mobile sdk availability](https://github.com/dashpay/platform/pull/4934) — 🔴 CI failing · 🐢 targets v6.0-dev · areas: swift-sdk, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
- [dashpay/platform#4938 docs(platform): distinguish current-state proofs, stored receipts and execution result text](https://github.com/dashpay/platform/pull/4938) — 🔴 CI failing · 🐢 targets v6.0-dev · areas: dpp, rust-sdk, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
- [dashpay/platform#4939 ci: audit the dashvm engine dependency set and document the equivalence hotfix rule](https://github.com/dashpay/platform/pull/4939) — 🔴 CI failing · 🐢 targets v6.0-dev · areas: github, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
- [dashpay/platform#5211 feat(platform)!: add compilation readiness rounds, reports and funds to drive](https://github.com/dashpay/platform/pull/5211) — 🔴 CI failing · 🐢 targets v6.0-dev · areas: rs-drive, rs-drive-abci, dpp, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required

<a id="dcg-claude-ready-for-human"></a>
#### Ready for human (12)
- [dashpay/platform#4703 fix(platform): resolve fee versions by registered number and price refunds at the storage epoch rate](https://github.com/dashpay/platform/pull/4703) — areas: rs-drive, rs-drive-abci, dpp, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
- [dashpay/platform#4705 feat(platform): define protocol-versioned smart-contract computation limits and their gas representation](https://github.com/dashpay/platform/pull/4705) — 🐢 targets v6.0-dev · areas: rs-drive, rs-drive-abci, dpp, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
- [dashpay/platform#4706 feat(platform)!: require fee history for storage refunds and credit their recorded owners](https://github.com/dashpay/platform/pull/4706) — areas: rs-drive, rs-drive-abci, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
- [dashpay/platform#4712 feat(platform): add the dashvm-validation crate and the DashVM protocol table](https://github.com/dashpay/platform/pull/4712) — ⚠ merge conflict · 🔴 CI failing · 🐢 targets v6.0-dev · areas: github, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
- [dashpay/platform#4716 feat(drive)!: add the contract credits root sum tree to genesis, upgrade and credit conservation](https://github.com/dashpay/platform/pull/4716) — 🔴 CI failing · 🐢 targets v6.0-dev · areas: rs-drive, rs-drive-abci, dpp, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
- [dashpay/platform#4717 feat(platform)!: add family-specific state transition size limits and the large contract envelope decode path](https://github.com/dashpay/platform/pull/4717) — 🔴 CI failing · 🐢 targets v6.0-dev · areas: rs-drive, rs-drive-abci, dpp, dashmate, rust-dapi, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
- [dashpay/platform#4789 feat(platform): add typed refund owners to storage flags and fee refunds](https://github.com/dashpay/platform/pull/4789) — 🔴 CI failing · 🐢 targets v6.0-dev · areas: rs-drive, dpp, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
- [dashpay/platform#4808 test(drive-abci): rehearse a reproducible scheduled host fault and the existing recovery path](https://github.com/dashpay/platform/pull/4808) — 🔴 CI failing · 🐢 targets v6.0-dev · areas: rs-drive-abci, github, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
- [dashpay/platform#4934 docs: correct the smart-contract target, version counts, transition list and mobile sdk availability](https://github.com/dashpay/platform/pull/4934) — 🔴 CI failing · 🐢 targets v6.0-dev · areas: swift-sdk, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
- [dashpay/platform#4938 docs(platform): distinguish current-state proofs, stored receipts and execution result text](https://github.com/dashpay/platform/pull/4938) — 🔴 CI failing · 🐢 targets v6.0-dev · areas: dpp, rust-sdk, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
- [dashpay/platform#4939 ci: audit the dashvm engine dependency set and document the equivalence hotfix rule](https://github.com/dashpay/platform/pull/4939) — 🔴 CI failing · 🐢 targets v6.0-dev · areas: github, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
- [dashpay/platform#5211 feat(platform)!: add compilation readiness rounds, reports and funds to drive](https://github.com/dashpay/platform/pull/5211) — 🔴 CI failing · 🐢 targets v6.0-dev · areas: rs-drive, rs-drive-abci, dpp, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required

<a id="dcg-claude-ci-failing"></a>
#### CI Failing (4)
- [dashpay/platform#4704 fix(platform): record the genesis fee generation and add replay coverage across a fee-version boundary](https://github.com/dashpay/platform/pull/4704) — 🔴 CI failing · areas: rs-drive-abci, fallback · Policy: waiting-build
  - Blocker: The build must pass before a human is asked
- [dashpay/platform#4707 feat: alloc-only profiles for platform-value and platform-serialization](https://github.com/dashpay/platform/pull/4707) — 🔴 CI failing · areas: github, fallback · Policy: waiting-build
  - Blocker: The build must pass before a human is asked
- [dashpay/platform#4719 feat: add the dash-sdk-contract declaration model, grammar and diagnostics](https://github.com/dashpay/platform/pull/4719) — 🔴 CI failing · areas: github, fallback · Policy: waiting-build
  - Blocker: The build must pass before a human is asked
- [dashpay/platform#4794 feat: pin contract configuration vectors and mirror them in wasm-dpp2 and the sdk ffi](https://github.com/dashpay/platform/pull/4794) — 🔴 CI failing · areas: dpp, rust-sdk-ffi, fallback · Policy: waiting-build
  - Blocker: The build must pass before a human is asked

<a id="dcg-claude-stale"></a>
#### Stale (13)
- [dashpay/platform#4705 feat(platform): define protocol-versioned smart-contract computation limits and their gas representation](https://github.com/dashpay/platform/pull/4705) — 🐢 targets v6.0-dev · areas: rs-drive, rs-drive-abci, dpp, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
- [dashpay/platform#4712 feat(platform): add the dashvm-validation crate and the DashVM protocol table](https://github.com/dashpay/platform/pull/4712) — ⚠ merge conflict · 🔴 CI failing · 🐢 targets v6.0-dev · areas: github, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
- [dashpay/platform#4716 feat(drive)!: add the contract credits root sum tree to genesis, upgrade and credit conservation](https://github.com/dashpay/platform/pull/4716) — 🔴 CI failing · 🐢 targets v6.0-dev · areas: rs-drive, rs-drive-abci, dpp, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
- [dashpay/platform#4717 feat(platform)!: add family-specific state transition size limits and the large contract envelope decode path](https://github.com/dashpay/platform/pull/4717) — 🔴 CI failing · 🐢 targets v6.0-dev · areas: rs-drive, rs-drive-abci, dpp, dashmate, rust-dapi, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
- [dashpay/platform#4718 ci: compare cross-architecture replay artifacts across a protocol upgrade and a restart](https://github.com/dashpay/platform/pull/4718) — 🔴 CI failing · 🐢 targets v6.0-dev · areas: rs-drive-abci, github, fallback · Policy: waiting-build
  - Blocker: The build must pass before a human is asked
- [dashpay/platform#4784 feat(platform)!: validate contested index parameters and bind the native award to its poll](https://github.com/dashpay/platform/pull/4784) — ⚠ merge conflict · 🔴 CI failing · 🐢 targets v6.0-dev · areas: rs-drive, rs-drive-abci, dpp, fallback · Policy: waiting-build
  - Blocker: The build must pass before a human is asked
- [dashpay/platform#4789 feat(platform): add typed refund owners to storage flags and fee refunds](https://github.com/dashpay/platform/pull/4789) — 🔴 CI failing · 🐢 targets v6.0-dev · areas: rs-drive, dpp, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
- [dashpay/platform#4790 feat(drive)!: keep per-issuer token supply rollups and a destroyed-issuer ledger](https://github.com/dashpay/platform/pull/4790) — ⚠ merge conflict · 🔴 CI failing · 🐢 targets v6.0-dev · areas: rs-drive, rs-drive-abci, dpp, fallback · Policy: waiting-build
  - Blocker: The build must pass before a human is asked
- [dashpay/platform#4808 test(drive-abci): rehearse a reproducible scheduled host fault and the existing recovery path](https://github.com/dashpay/platform/pull/4808) — 🔴 CI failing · 🐢 targets v6.0-dev · areas: rs-drive-abci, github, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
- [dashpay/platform#4934 docs: correct the smart-contract target, version counts, transition list and mobile sdk availability](https://github.com/dashpay/platform/pull/4934) — 🔴 CI failing · 🐢 targets v6.0-dev · areas: swift-sdk, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
- [dashpay/platform#4938 docs(platform): distinguish current-state proofs, stored receipts and execution result text](https://github.com/dashpay/platform/pull/4938) — 🔴 CI failing · 🐢 targets v6.0-dev · areas: dpp, rust-sdk, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
- [dashpay/platform#4939 ci: audit the dashvm engine dependency set and document the equivalence hotfix rule](https://github.com/dashpay/platform/pull/4939) — 🔴 CI failing · 🐢 targets v6.0-dev · areas: github, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
- [dashpay/platform#5211 feat(platform)!: add compilation readiness rounds, reports and funds to drive](https://github.com/dashpay/platform/pull/5211) — 🔴 CI failing · 🐢 targets v6.0-dev · areas: rs-drive, rs-drive-abci, dpp, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required

<a id="dcg-claude-clean"></a>
#### Clean (2)
- [dashpay/platform#4703 fix(platform): resolve fee versions by registered number and price refunds at the storage epoch rate](https://github.com/dashpay/platform/pull/4703) — areas: rs-drive, rs-drive-abci, dpp, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
- [dashpay/platform#4706 feat(platform)!: require fee history for storage refunds and credit their recorded owners](https://github.com/dashpay/platform/pull/4706) — areas: rs-drive, rs-drive-abci, fallback · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required

<a id="thepastaclaw"></a>
### @thepastaclaw
<a id="thepastaclaw-open"></a>
#### Open (2)
- [dashpay/platform#4298 fix(dashmate): load ZeroSSL config in force mode](https://github.com/dashpay/platform/pull/4298) — areas: dashmate · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required
- [dashpay/platform#5581 feat(platform): add proof-backed asset lock outpoint query](https://github.com/dashpay/platform/pull/5581) — 🔴 CI failing · areas: rs-drive, rs-drive-abci, dpp, rs-platform-wallet, rust-sdk, rust-dapi, fallback · Policy: waiting-author
  - Blocker: coderabbitai left review threads unresolved; resolve them
  - Blocker: thepastaclaw left review threads unresolved; resolve them

<a id="thepastaclaw-ready-for-human"></a>
#### Ready for human (1)
- [dashpay/platform#4298 fix(dashmate): load ZeroSSL config in force mode](https://github.com/dashpay/platform/pull/4298) — areas: dashmate · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required

<a id="thepastaclaw-ci-failing"></a>
#### CI Failing (1)
- [dashpay/platform#5581 feat(platform): add proof-backed asset lock outpoint query](https://github.com/dashpay/platform/pull/5581) — 🔴 CI failing · areas: rs-drive, rs-drive-abci, dpp, rs-platform-wallet, rust-sdk, rust-dapi, fallback · Policy: waiting-author
  - Blocker: coderabbitai left review threads unresolved; resolve them
  - Blocker: thepastaclaw left review threads unresolved; resolve them

<a id="thepastaclaw-clean"></a>
#### Clean (1)
- [dashpay/platform#4298 fix(dashmate): load ZeroSSL config in force mode](https://github.com/dashpay/platform/pull/4298) — areas: dashmate · Policy: ready-for-human
  - Blocker: Human approval or objection resolution is required

## Methodology
Generated nightly by [pr-hygiene](https://github.com/dashpay/stale_prs_are_bad). A thread counts as "unresolved" when it is open, not outdated, has a comment from someone other than the PR author, and the most recent comment is from a reviewer. **Dirty** = at least one such thread. **Unresolved Comments** = at least one such thread. **Changes Requested** = no unresolved threads but a reviewer's most recent review is CHANGES_REQUESTED (still blocking until someone re-approves or dismisses). **Deferred** = carries a configured deferred label (e.g. `postponed`) — visible but not counted toward unresolved-comment counts. **Stale** = targets a non-default branch OR hasn't been touched in the configured threshold (default 120 days, but clean PRs are never reclassified as stale). **Draft** = the PR is still marked draft on GitHub. **CI failing** = no unresolved comments, no changes-requested, but the latest commit's status check is failing. **Clean** = open, not draft, not deferred, not stale, no unresolved comments, no changes-requested, CI green. **Needs action** further requires changes-requested, merge conflict, or that the reviewer commented more recently than the author last pushed. **Ready for human** counts a person's own PRs that the shared review engine marks `ready-for-human` or `ready-to-merge`; each PR bullet shows the engine's state as `Policy:` with its blockers. **Ready for Review** counts clean PRs (authored by someone else) where this person owes a review: the union of the shared policy's routing (owners and reviewers of every area the changed files fall into, or the repository fallback for files no area claims) and GitHub's explicit review requests. The author is never routed to their own PR, and anyone who has already submitted any review is excluded — their job is done. `⚠ ownership unresolved` marks areas whose roster the policy still lists as open. Configurable via [`https://github.com/dashpay/stale_prs_are_bad/blob/master/.pr-hygiene.yml`](https://github.com/dashpay/stale_prs_are_bad/blob/master/.pr-hygiene.yml)—edit defaults there; ownership lives in `policies/`.
