// Direction A: clean, dark-first and dense, in the manner of Linear or Vercel.
// Sans-serif text, monospace for PR numbers, counts and ages, a quiet accent
// per stage, and the keyboard model in ../shared/keys.js.

import { h, svg } from "../../site/js/dom.js";
import { has } from "../../site/js/model.js";
import { lateMark } from "../../site/js/ui.js";
import { start } from "../shared/views.js";

start({
  brand: [h("span", { class: "logo", "aria-hidden": "true" }), h("span", { class: "brand-text" }, "PR Hygiene")],
  tab: (label) => label,
  jumpLabel: "Jump to…",
  themeGlyph: (t) => ({ system: "◐", dark: "☾", light: "☀" })[t],
  warnGlyph: h("span", { class: "warn-glyph", "aria-hidden": "true" }, "!"),
  avatars: true,
  moveCursor: false,
  // The live marks: on time a circle, late a triangle, very late a diamond,
  // hollow when only the age is known; a small square for a stage that is
  // never late, so it does not read as "on time" by colour alone.
  mark: (kind) => {
    if (kind === "untimed") {
      return svg("svg", { class: "mark mark-untimed", width: 12, height: 12, viewBox: "0 0 12 12", "aria-hidden": "true" },
        svg("polygon", { points: "2.5,2.5 9.5,2.5 9.5,9.5 2.5,9.5" }));
    }
    return kind === "opened" ? lateMark(null, "mark-opened") : lateMark(kind, `mark-${kind}`);
  },
  stage: (data, key) => {
    const s = has(data.stage, key) ? data.stage[key] : data.stage.unknown;
    return h("span", { class: `stage s-${s.key}`, title: s.note }, h("span", { class: "dot", "aria-hidden": "true" }), s.label);
  },
  glyph: { comments: "💬 ", ci: "CI", conflict: "conflict", cr: "changes", idle: "idle" },
  footer: () => h("footer", { class: "foot" },
    h("span", {}, "Press ", h("kbd", {}, "?"), " for shortcuts, ", h("kbd", {}, "⌘K"), " to jump"),
    h("a", { href: "#/privacy" }, "Privacy"),
    h("span", { class: "proto" }, "Prototype A · live data")),
});
