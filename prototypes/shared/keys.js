// The keyboard model both prototypes share:
//   /        search            j / k    next / previous row
//   Enter    expand the row    o        open the PR on GitHub
//   ⌘K, ^K   jump anywhere     g t/p/m  Team, People, Me
//   l        late only         ?        this list        Esc   close, clear
// A row is any element with [data-row] in the page; moving to it focuses
// its title button (or a person's name link), so a screen reader announces
// the row and whether it is expanded, and Enter works natively. The
// single-key shortcuts can be switched off in the help dialog.

const SINGLE_KEYS = "pr-hygiene:proto-single-keys";

function singleKeysOn() {
  try {
    return localStorage.getItem(SINGLE_KEYS) !== "off";
  } catch {
    return true;
  }
}

/** What j/k focus inside a row. */
export function focusTarget(row) {
  return row.querySelector("button.title, .c-who a") || row;
}

import { h } from "../../site/js/dom.js";
import { matchPalette } from "./core.js";

export const SHORTCUTS = [
  ["/", "Search"],
  ["j  k", "Next / previous row"],
  ["Enter", "Expand row, or open a person"],
  ["o", "Open the PR on GitHub"],
  ["⌘K  Ctrl K", "Jump to a view, person, repo or PR"],
  ["g t  g p  g m", "Team, People, Me"],
  ["l", "Late only, on Team"],
  ["Esc", "Close, or leave the search"],
  ["?", "These shortcuts"],
];

function typing(el) {
  return el && (el.isContentEditable || /^(INPUT|SELECT|TEXTAREA)$/.test(el.tagName));
}

/**
 * api: { go(hash), rows(): Element[], toggle(row), open(row), toggleLate(),
 *        items(): palette items, kindLabel(kind) }
 */
export function installKeys(api) {
  let pending = null;
  let pendingTimer = 0;

  const move = (delta) => {
    const rows = api.rows();
    if (!rows.length) return;
    const at = rows.indexOf(document.activeElement.closest?.("[data-row]"));
    const next = at < 0 ? (delta > 0 ? 0 : rows.length - 1) : Math.min(rows.length - 1, Math.max(0, at + delta));
    focusTarget(rows[next]).focus({ preventScroll: true });
    rows[next].scrollIntoView({ block: "nearest" });
  };

  addEventListener("keydown", (e) => {
    if ((e.metaKey || e.ctrlKey) && e.key.toLowerCase() === "k") {
      e.preventDefault();
      openPalette(api);
      return;
    }
    if (document.querySelector("dialog[open]")) return;
    const t = e.target;
    if (typing(t)) {
      if (e.key === "Escape" && t.type === "search") {
        // Leave the search, keeping what was typed (a search field clears itself on Esc).
        e.preventDefault();
        const first = api.rows()[0];
        if (first) focusTarget(first).focus();
        else t.blur();
      } else if (e.key === "ArrowDown" && t.type === "search") {
        e.preventDefault();
        move(1);
      }
      return;
    }
    if (e.metaKey || e.ctrlKey || e.altKey) return;
    if (e.key === "?") {
      e.preventDefault();
      openHelp();
      return;
    }
    if (!singleKeysOn()) return;
    if (pending === "g") {
      pending = null;
      clearTimeout(pendingTimer);
      const to = { t: "#/", p: "#/people", m: "#/me" }[e.key];
      if (to) {
        e.preventDefault();
        api.go(to);
      }
      return;
    }
    const row = document.activeElement?.closest?.("[data-row]");
    switch (e.key) {
      case "/": {
        const q = document.getElementById("f-q") || document.getElementById("me-q");
        if (q) {
          e.preventDefault();
          q.focus();
          q.select();
        }
        break;
      }
      case "j": case "ArrowDown":
        if (e.key === "ArrowDown" && !row) return;
        e.preventDefault();
        move(1);
        break;
      case "k": case "ArrowUp":
        if (e.key === "ArrowUp" && !row) return;
        e.preventDefault();
        move(-1);
        break;
      case "o":
        if (row) api.open(row);
        break;
      case "l":
        api.toggleLate();
        break;
      case "g":
        pending = "g";
        pendingTimer = setTimeout(() => { pending = null; }, 1200);
        break;
      default:
    }
  });
}

function modal(cls, label, ...children) {
  // One dialog at a time: a new one replaces any that is open.
  for (const open of document.querySelectorAll("dialog[open]")) open.close();
  const d = h("dialog", { class: cls, "aria-label": label }, children);
  // A click on the backdrop closes it.
  d.addEventListener("click", (e) => {
    if (e.target === d) d.close();
  });
  d.addEventListener("close", () => d.remove());
  document.body.append(d);
  d.showModal();
  return d;
}

export function openHelp() {
  const toggle = h("input", { type: "checkbox", id: "single-keys", checked: singleKeysOn() });
  toggle.addEventListener("change", () => {
    try {
      localStorage.setItem(SINGLE_KEYS, toggle.checked ? "on" : "off");
    } catch {
      // Storage blocked: the choice is not kept.
    }
  });
  const d = modal("help", "Keyboard shortcuts",
    h("h2", {}, "Keyboard"),
    h("dl", {}, SHORTCUTS.map(([k, v]) => [h("dt", {}, k.split("  ").map((x) => h("kbd", {}, x))), h("dd", {}, v)])),
    h("p", { class: "help-opt" }, toggle, h("label", { for: "single-keys" }, " Single-key shortcuts (j k o l g /); ⌘K and ? always work")),
    h("p", { class: "help-foot" }, h("button", { type: "button", class: "dlg-close", on: { click: () => d.close() } }, "Close"), " or Esc"));
  d.querySelector("h2").setAttribute("tabindex", "-1");
  d.querySelector("h2").focus();
}

const KIND = { view: "View", person: "Person", repo: "Repo", pr: "PR" };

export function openPalette(api) {
  if (document.querySelector("dialog.pal")) return;
  const items = api.items();
  const input = h("input", {
    type: "text", class: "pal-input", placeholder: "Jump to a view, person, repo or PR…", autocomplete: "off", spellcheck: "false",
    role: "combobox", "aria-expanded": "true", "aria-controls": "pal-list", "aria-autocomplete": "list", "aria-label": "Jump to",
  });
  const list = h("ul", { class: "pal-list", id: "pal-list", role: "listbox", "aria-label": "Matches" });
  const empty = h("p", { class: "pal-empty", role: "status" }, "Nothing matches.");
  let shown = [];
  let at = 0;
  const paint = () => {
    shown = matchPalette(items, input.value);
    at = Math.min(at, Math.max(0, shown.length - 1));
    list.replaceChildren(...shown.map((it, i) => h("li", {
      id: `pal-${i}`, role: "option", class: "pal-item", "aria-selected": i === at ? "true" : "false",
      on: { click: () => run(i), mousemove: () => { if (at !== i) { at = i; mark(); } } },
    }, h("span", { class: `pal-kind pal-kind-${it.kind}` }, KIND[it.kind] || it.kind),
    h("span", { class: "pal-label" }, it.label),
    h("span", { class: "pal-hint" }, it.hint))));
    empty.hidden = shown.length > 0;
    mark();
  };
  const mark = () => {
    list.querySelectorAll("[role=option]").forEach((el, i) => el.setAttribute("aria-selected", i === at ? "true" : "false"));
    if (shown.length) {
      input.setAttribute("aria-activedescendant", `pal-${at}`);
      document.getElementById(`pal-${at}`)?.scrollIntoView({ block: "nearest" });
    } else {
      input.removeAttribute("aria-activedescendant");
    }
  };
  const d = modal("pal", "Jump to", input, list, empty,
    h("p", { class: "pal-foot" }, h("kbd", {}, "↑↓"), " move ", h("kbd", {}, "Enter"), " go ", h("kbd", {}, "Esc"), " close"));
  const run = (i) => {
    const it = shown[i];
    if (!it) return;
    d.close();
    it.run();
  };
  input.addEventListener("input", () => {
    at = 0;
    paint();
  });
  input.addEventListener("keydown", (e) => {
    if (e.key === "ArrowDown" || (e.ctrlKey && e.key === "n")) {
      e.preventDefault();
      at = Math.min(shown.length - 1, at + 1);
      mark();
    } else if (e.key === "ArrowUp" || (e.ctrlKey && e.key === "p")) {
      e.preventDefault();
      at = Math.max(0, at - 1);
      mark();
    } else if (e.key === "Enter") {
      e.preventDefault();
      run(at);
    }
  });
  paint();
  input.focus();
}
