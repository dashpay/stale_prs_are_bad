// Direction B: a terminal UI. Monospace throughout, framed panes, green and
// amber accents, one-glyph status marks, a status line with the keys, and
// the keyboard model in ../shared/keys.js.

import { h } from "../../site/js/dom.js";
import { has } from "../../site/js/model.js";
import { start } from "../shared/views.js";

// Lateness keeps the live page's shapes: on time a circle, late a triangle,
// very late a diamond; hollow when only the PR's age is known.
const MARK = { ok: "●", late: "▲", "very-late": "◆", untimed: "•", opened: "○" };

const SHORT = {
  draft: "DRAFT", bots: "BOTS", "self-review": "SELF", ci: "CI", queued: "QUEUE", review: "REVIEW",
  mergeable: "MERGE", blocked: "BLOCK", "not-governed": "NOGOV", unknown: "??",
};

function glyph(kind) {
  return h("span", { class: `g g-${kind}`, "aria-hidden": "true" }, MARK[kind]);
}

start({
  brand: [h("span", { class: "prompt", "aria-hidden": "true" }, "~$"), " pr-hygiene", h("span", { class: "caret", "aria-hidden": "true" })],
  tab: (label) => [h("span", { class: "hot" }, label[0].toLowerCase()), label.slice(1).toLowerCase()],
  jumpLabel: "jump",
  themeGlyph: (t) => ({ system: "◐", dark: "◼", light: "◻" })[t],
  warnGlyph: h("span", { class: "warn-glyph", "aria-hidden": "true" }, "!!"),
  avatars: false,
  moveCursor: true,
  mark: (p) => glyph(p.since_basis !== "engine" ? "opened" : p.lateness || "untimed"),
  lateGlyph: (kind) => glyph(kind),
  slotGlyph: (used) => (used ? "■" : "□"),
  stage: (data, key) => {
    const s = has(data.stage, key) ? data.stage[key] : data.stage.unknown;
    return h("span", { class: `stage s-${s.key}`, title: s.note ? `${s.label}: ${s.note}` : s.label },
      h("span", { "aria-hidden": "true" }, has(SHORT, s.key) ? SHORT[s.key] : s.key.toUpperCase()),
      h("span", { class: "sr" }, s.label));
  },
  glyph: { comments: "c", ci: "CI✗", conflict: "CONFL", cr: "CHG", idle: "IDLE" },
  listHead: () => h("div", { class: "rows-head pr-head", "aria-hidden": "true" },
    h("span", {}, "age"), h("span", {}, "pr"), h("span", {}, "title / next action"), h("span", {}, "stage"), h("span", {}, "author"), h("span", { class: "n" }, "flags")),
  footer: (route) => h("footer", { class: "status" },
    h("span", { class: "mode" }, route.view === "person" ? "PERSON" : route.view.toUpperCase()),
    h("span", { class: "keys" },
      [["/", "search"], ["j/k", "move"], ["⏎", "expand"], ["o", "github"], ["⌘K", "jump"], ["g t/p/m", "views"], ["l", "late"], ["?", "help"]]
        .map(([k, v]) => h("span", { class: "key" }, h("kbd", {}, k), ` ${v}`))),
    h("a", { href: "#/privacy", class: "status-right" }, "privacy")),
});
