// The keyboard: a jump palette, a shortcut list, and single keys.
//   /        search              j / k    next / previous row
//   Enter    open the row        o        open the row's PR on GitHub
//   ⌘K, ^K   jump anywhere       g t/p/m  Team, People, Me
//   l        late only on Team   ?        the shortcut list   Esc  close
// A row is any element with [data-row] in the page; moving to it focuses its
// title button (or a person's name link), so a screen reader reads the row
// and whether it is open, and Enter works as it does for that control.
// Nothing fires while typing in a field. The single-key shortcuts can be
// turned off in the shortcut list; ⌘K and ? always work.

import { h } from "./dom.js";

const SINGLE_KEYS = "pr-hygiene:dashboard:single-keys";

const SHORTCUTS = [
  ["/", "Search"],
  ["j  k", "Next / previous row"],
  ["Enter", "Open the row's details, or the person's page"],
  ["o", "Open the row's PR on GitHub"],
  ["⌘K  Ctrl K", "Jump to a view, person, repository or PR"],
  ["g t  g p  g m", "Team, People, Me"],
  ["l", "Late only, on Team"],
  ["Esc", "Close, or leave the search"],
  ["?", "This list"],
];

function singleKeysOn() {
  try {
    return localStorage.getItem(SINGLE_KEYS) !== "off";
  } catch {
    return true;
  }
}

/** What j and k focus inside a row. */
export function focusTarget(row) {
  return row.querySelector(".row-title, .c-who a") || row;
}

function typing(el) {
  return !!el && (el.isContentEditable || /^(INPUT|SELECT|TEXTAREA)$/.test(el.tagName));
}

/**
 * api: { go(hash), rows(): Element[] in page order, openOnGitHub(row) for o,
 *        toggleLate(), items(): palette entries {kind, label, hint, run} }
 */
export function installKeys(api) {
  let pending = false;
  let pendingTimer = 0;

  const move = (delta) => {
    const rows = api.rows();
    if (!rows.length) return;
    const at = rows.indexOf(document.activeElement?.closest?.("[data-row]"));
    const next = at < 0 ? (delta > 0 ? 0 : rows.length - 1) : Math.min(rows.length - 1, Math.max(0, at + delta));
    focusTarget(rows[next]).focus({ preventScroll: true });
    rows[next].scrollIntoView({ block: "nearest" });
  };

  addEventListener("keydown", (e) => {
    if ((e.metaKey || e.ctrlKey) && !e.altKey && e.key.toLowerCase() === "k") {
      e.preventDefault();
      openPalette(api);
      return;
    }
    if (e.defaultPrevented || document.querySelector("dialog[open]")) return;
    const t = e.target;
    if (typing(t)) {
      if (e.key === "Escape" && t.type === "search") {
        // Leave the search for the list, keeping what was typed: a search
        // field would otherwise clear itself on Escape.
        e.preventDefault();
        const first = api.rows()[0];
        if (first) focusTarget(first).focus();
        else t.blur();
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
    if (pending) {
      pending = false;
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
      case "j":
        e.preventDefault();
        move(1);
        break;
      case "k":
        e.preventDefault();
        move(-1);
        break;
      case "o":
        if (row) api.openOnGitHub(row);
        break;
      case "l":
        api.toggleLate();
        break;
      case "g":
        pending = true;
        pendingTimer = setTimeout(() => {
          pending = false;
        }, 1200);
        break;
      default:
    }
  });
}

/** A modal dialog; one at a time, closed by Esc, a click outside, or its own button. */
function modal(cls, label, ...children) {
  for (const dlg of document.querySelectorAll("dialog[open]")) dlg.close();
  const d = h("dialog", { class: cls, "aria-label": label }, children);
  d.addEventListener("click", (e) => {
    if (e.target === d) d.close();
  });
  d.addEventListener("close", () => d.remove());
  document.body.append(d);
  d.showModal();
  return d;
}

function closeButton(get) {
  return h("button", { type: "button", class: "dialog-close", on: { click: () => get().close() } }, "Close");
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
  const heading = h("h2", { tabindex: "-1" }, "Keyboard shortcuts");
  const d = modal("help", "Keyboard shortcuts",
    heading,
    h("dl", {}, SHORTCUTS.map(([k, v]) => [h("dt", {}, k.split("  ").map((x) => h("kbd", {}, x))), h("dd", {}, v)])),
    h("p", { class: "help-option" }, toggle,
      h("label", { for: "single-keys" }, "Single-key shortcuts (/ j k o l g); ⌘K and ? always work")),
    h("p", { class: "dialog-foot" }, closeButton(() => d), h("span", {}, "or Esc")));
  heading.focus();
}

const KIND = { view: "View", person: "Person", repo: "Repo", pr: "PR" };

/**
 * Entries whose text holds every word typed. Those matched on their label
 * come first, and among them labels with a part starting with a word typed
 * ("49" puts platform#4934 before platform#3549); then the rest, in order.
 */
function match(items, q, limit = 40) {
  const words = q.trim().toLowerCase().split(/\s+/).filter(Boolean);
  const onLabel = [];
  const elsewhere = [];
  for (const it of items) {
    const label = `${it.label} ${it.kind}`.toLowerCase();
    if (words.every((w) => label.includes(w))) onLabel.push(it);
    else if (words.every((w) => `${label} ${it.hint}`.toLowerCase().includes(w))) elsewhere.push(it);
  }
  const starts = (it) => {
    const parts = `${it.label} ${it.kind}`.toLowerCase().split(/[#\s/]+/);
    return words.filter((w) => parts.some((p) => p.startsWith(w))).length;
  };
  const ranked = onLabel.map((it, i) => [it, starts(it), i]).sort((a, b) => b[1] - a[1] || a[2] - b[2]).map(([it]) => it);
  return [...ranked, ...elsewhere].slice(0, limit);
}

export function openPalette(api) {
  if (document.querySelector("dialog.palette")) return;
  const items = api.items();
  const input = h("input", {
    type: "text", class: "palette-input", placeholder: "Jump to a view, person, repository or PR…",
    autocomplete: "off", spellcheck: "false", role: "combobox", "aria-expanded": "true",
    "aria-controls": "palette-list", "aria-autocomplete": "list", "aria-label": "Jump to",
  });
  const list = h("ul", { class: "palette-list", id: "palette-list", role: "listbox", "aria-label": "Matches" });
  const status = h("p", { class: "palette-status", role: "status" });
  let shown = [];
  let at = 0;
  const mark = () => {
    list.querySelectorAll("[role=option]").forEach((el, i) => el.setAttribute("aria-selected", i === at ? "true" : "false"));
    if (shown.length) {
      input.setAttribute("aria-activedescendant", `palette-${at}`);
      document.getElementById(`palette-${at}`)?.scrollIntoView({ block: "nearest" });
    } else {
      input.removeAttribute("aria-activedescendant");
    }
  };
  const paint = () => {
    shown = match(items, input.value);
    at = Math.min(at, Math.max(0, shown.length - 1));
    list.replaceChildren(...shown.map((it, i) => h("li", {
      id: `palette-${i}`, role: "option", class: "palette-item", "aria-selected": "false",
      on: {
        click: () => run(i),
        mousemove: () => {
          if (at !== i) {
            at = i;
            mark();
          }
        },
      },
    }, h("span", { class: `palette-kind palette-kind-${it.kind}` }, KIND[it.kind] || it.kind),
    h("span", { class: "palette-label" }, it.label),
    h("span", { class: "palette-hint" }, it.hint))));
    status.textContent = shown.length ? `${shown.length}${shown.length === 40 ? "+" : ""} matches` : "Nothing matches.";
    status.classList.toggle("sr", shown.length > 0);
    mark();
  };
  const d = modal("palette", "Jump to", input, list, status,
    h("p", { class: "dialog-foot" }, h("kbd", {}, "↑"), h("kbd", {}, "↓"), h("span", {}, "move"),
      h("kbd", {}, "Enter"), h("span", {}, "go"), h("kbd", {}, "Esc"), h("span", {}, "close")));
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
    if (e.key === "ArrowDown") {
      e.preventDefault();
      at = Math.min(shown.length - 1, at + 1);
      mark();
    } else if (e.key === "ArrowUp") {
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
