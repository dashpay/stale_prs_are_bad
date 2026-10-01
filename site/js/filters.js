// One filter row above everything it scopes. Every control writes the URL;
// the page re-renders from it.

import { h } from "./dom.js";
import { ownerLabel, shortRepo } from "./model.js";
import { buildHash, setFilters } from "./state.js";

/** Which author kind a view shows when the URL does not say. */
export function effectiveWho(f, fallback) {
  return f.who || fallback;
}

export function matchesText(pr, q) {
  const needle = q.trim().toLowerCase();
  if (!needle) return true;
  const hay = [pr.title, pr.author, pr.key, ...pr.reviewers].map((s) => String(s ?? "").toLowerCase());
  return hay.some((s) => s.includes(needle));
}

/** PRs left after the filters. `ignoreOwner` keeps the whose-move tiles counting every owner. */
export function filterPrs(prs, f, { ignoreOwner = false } = {}) {
  const who = effectiveWho(f, "human");
  return prs.filter((p) =>
    (!f.repo || p.repo === f.repo) &&
    (!f.stage || p.stage === f.stage) &&
    (ignoreOwner || !f.owner || p.owner === f.owner) &&
    (who === "all" || (who === "bot") === p.isBot) &&
    (!f.late || p.lateRank >= 2) &&
    matchesText(p, f.q));
}

/**
 * The filter row. `path` is the view's route; `show` lists the controls that
 * mean something on this view; `whoDefault` is what an empty "who" means here.
 */
export function filterBar(data, path, f, { show, whoDefault, onText, searchHint }) {
  const go = (change) => setFilters(path, f, change, { keepSort: true });
  const controls = [];

  if (show.includes("repo")) {
    controls.push(field("Repository", "f-repo", select("f-repo",
      [["", "All repositories"], ...[...data.repoSet].map((r) => [r, shortRepo(r)])],
      f.repo, (v) => go({ repo: v }))));
  }
  if (show.includes("stage")) {
    controls.push(field("Stage", "f-stage", select("f-stage",
      [["", "All stages"], ...data.stages.map((s) => [s.key, s.label])],
      f.stage, (v) => go({ stage: v }))));
  }
  if (show.includes("who")) {
    const opts = whoDefault === "human"
      ? [["human", "PRs by humans"], ["all", "Include bot PRs"], ["bot", "Only bot PRs"]]
      : [["all", "Humans and bots"], ["human", "Humans"], ["bot", "Bots"]];
    controls.push(field("Authors", "f-who", select("f-who", opts, effectiveWho(f, whoDefault),
      (v) => go({ who: v === whoDefault ? "" : v }))));
  }
  if (show.includes("late")) {
    const box = h("input", { type: "checkbox", id: "f-late", checked: f.late, on: { change: (e) => go({ late: e.target.checked }) } });
    controls.push(h("div", { class: "field field-check" }, box, h("label", { for: "f-late" }, "Late only")));
  }
  if (show.includes("q")) {
    const input = h("input", {
      type: "search", id: "f-q", value: f.q, placeholder: searchHint,
      autocomplete: "off", spellcheck: "false",
    });
    let timer = 0;
    input.addEventListener("input", () => {
      clearTimeout(timer);
      // Typing replaces the history entry instead of adding one per keystroke.
      // The text is kept as typed (a trailing space is the start of the next
      // word); matching trims it. A page that moved on meanwhile is left alone.
      timer = setTimeout(() => {
        if (!input.isConnected) return;
        history.replaceState(null, "", buildHash(path, { ...f, q: input.value.slice(0, 100) }));
        onText();
      }, 150);
    });
    controls.push(field("Search", "f-q", input));
  }
  if (show.includes("owner") && f.owner) {
    controls.push(h("div", { class: "field" },
      h("span", { class: "chip" }, `Whose move: ${ownerLabel(f.owner)} `,
        h("a", { href: buildHash(path, { ...f, owner: "" }), "aria-label": "clear whose-move filter" }, "✕"))));
  }
  return h("form", { class: "filters", role: "search", on: { submit: (e) => e.preventDefault() } }, controls);
}

function field(label, id, control) {
  return h("div", { class: "field" }, h("label", { for: id }, label), control);
}

function select(id, options, value, onChange) {
  return h("select", { id, on: { change: (e) => onChange(e.target.value) } },
    options.map(([v, label]) => h("option", { value: v, selected: v === value }, label)));
}
