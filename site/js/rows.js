// Dense rows for PR and people lists. A PR row is one line: time in stage,
// the PR, its title over what it waits for, stage, author and flags; its
// title button opens everything else the data says about it beneath the
// row. Lists sort by the same keys and URL parameters the tables used.

import { h } from "./dom.js";
import { LATENESS, has, formatDuration, formatHours, idleWords } from "./model.js";
import { setFilters } from "./state.js";
import { prLink, personLink, stageChip, timeCell, badges, asksList, areaName, sortRows } from "./ui.js";

// Rows for the same PR can appear twice on a page, so their element ids are numbered.
let rowSeq = 0;

/** A short visually hidden label for assistive technology. */
export function sr(text) {
  return h("span", { class: "sr" }, text);
}

/**
 * The row's second line, whole: whether the PR is known only from the
 * engine, its next action, the other blockers when asked for, and who it
 * waits on (the asks list every view uses, set on one line).
 */
function subLine(pr, withBlockers) {
  const parts = [];
  if (!pr.tracked) parts.push(h("span", { class: "basis", title: "Known only from the review engine; the board's own filters left it out" }, "engine only"));
  if (pr.next_action) parts.push(h("span", { class: "next-action" }, pr.next_action));
  if (withBlockers) {
    for (const b of pr.blockers.filter((x) => x !== pr.next_action)) parts.push(h("span", { class: "blocker" }, b));
  }
  const asks = asksList(pr);
  if (asks) parts.push(h("span", { class: "waits" }, "waits on ", asks));
  return parts.length ? h("span", { class: "row-sub" }, parts.map((p, i) => [i ? h("span", { class: "dot-sep", "aria-hidden": "true" }, " · ") : null, p])) : null;
}

/**
 * One PR. `part` replaces the second line (a reviewer's share of the PR);
 * `extra` is a short note after it (why a PR is listed to close or revive).
 */
export function prRow(data, pr, { milestone = "", part = null, extra = null, blockers = false } = {}) {
  const id = `row-${++rowSeq}`;
  const li = h("li", { class: `row prrow${pr.lateness ? ` row-${pr.lateness}` : ""}`, "data-row": "", "data-key": pr.key, id });
  // The title button is what j and k focus; it is described by the row's
  // time, stage and flags, so a screen reader hears the row, not the title alone.
  const title = h("button", {
    type: "button", class: "row-title", "aria-expanded": "false", "aria-describedby": `${id}-t ${id}-s ${id}-f`,
    on: { click: () => toggleDetail(data, li) },
  }, pr.title || "(no title)");
  li.append(
    h("span", { class: "c-time", id: `${id}-t` }, sr("In stage: "), timeCell(pr)),
    h("span", { class: "c-ref" }, prLink(pr, { withTitle: false })),
    h("span", { class: "c-main" }, title, part || subLine(pr, blockers), extra ? h("span", { class: "row-sub" }, extra) : null),
    h("span", { class: "c-stage", id: `${id}-s` }, sr("Stage: "), stageChip(data, pr.stage)),
    h("span", { class: "c-author" }, sr("Author: "), personLink(pr.author, pr.isBot, { withAvatar: true, milestone })),
    h("span", { class: "c-flags", id: `${id}-f` }, badges(pr)));
  li.addEventListener("click", (e) => {
    if (e.target.closest("a, button, summary")) return;
    toggleDetail(data, li);
  });
  return li;
}

function timeText(data, pr) {
  if (pr.ageMs === null) return "no start time";
  if (pr.since_basis !== "engine") return `${formatDuration(pr.ageMs)} since opened; this stage's start is not recorded yet`;
  const s = data.stage[pr.stage];
  const late = pr.lateness ? `, ${LATENESS[pr.lateness].label}` : "";
  const rule = s.entryRecorded && s.lateHours
    ? ` (late after ${formatHours(s.lateHours[0])}, very late after ${formatHours(s.lateHours[1])})`
    : " (never late in this stage)";
  return `${formatDuration(pr.ageMs)} in this stage${late}${rule}`;
}

/** Everything the data says about one PR, shown beneath its row. */
function detail(data, pr) {
  const kv = (k, ...v) => [h("dt", {}, k), h("dd", {}, v)];
  const rest = pr.blockers.filter((b) => b !== pr.next_action);
  const asks = asksList(pr);
  const s = data.stage[pr.stage];
  const flags = [
    pr.unresolved_comments ? `${pr.unresolved_comments} unresolved review comments` : null,
    pr.ci_failing ? "CI failing" : null,
    pr.merge_conflict ? "merge conflict" : null,
    pr.changes_requested ? "changes requested" : null,
    pr.idle ? idleWords(data) : null,
  ].filter(Boolean);
  return h("li", { class: "row-detail" },
    h("dl", {},
      kv("Stage", stageChip(data, pr.stage), s.note ? ` ${s.note}` : ""),
      kv("Time", timeText(data, pr)),
      pr.next_action ? kv("Next action", pr.next_action) : null,
      rest.length ? kv("Blockers", h("ul", { class: "blockers" }, rest.map((b) => h("li", {}, b)))) : null,
      asks ? kv("Waiting for", asks) : null,
      flags.length ? kv("Flags", flags.join(" · ")) : null,
      pr.areas.length ? kv("Areas", pr.areas.map((a, i) => [i ? ", " : "", areaName(a)])) : null,
      pr.milestone ? kv("Milestone", pr.milestone) : null,
      pr.updatedAgoMs !== null ? kv("Last update", `${formatDuration(pr.updatedAgoMs)} ago`) : null,
      pr.tracked ? null : kv("Note", "Known only from the review engine; the board's own filters left it out.")),
    pr.url ? h("p", { class: "row-open" }, prLink(pr, { withTitle: false }), " on GitHub ↗") : null);
}

/** Open or close the detail beneath a PR row. */
export function toggleDetail(data, li) {
  const button = li.querySelector(".row-title");
  const next = li.nextElementSibling;
  if (next && next.classList.contains("row-detail")) {
    next.remove();
    li.classList.remove("open");
    button?.setAttribute("aria-expanded", "false");
    return;
  }
  const pr = data.prByKey.get(li.dataset.key);
  if (!pr) return;
  const d = detail(data, pr);
  d.id = `${li.id}-d`;
  li.after(d);
  li.classList.add("open");
  button?.setAttribute("aria-controls", d.id);
  button?.setAttribute("aria-expanded", "true");
}

/** A list, or a line saying why it is empty. */
export function rowList(items, make, empty, label) {
  return items.length
    ? h("ul", { class: "rows", "aria-label": label || null }, items.map(make))
    : h("p", { class: "empty" }, empty);
}

/**
 * The sort menu and direction button for a list. `sorts`: {key: {label,
 * firstDir, sort(row)}}; the choice lives in the URL as the tables' did.
 */
export function sortControl(path, f, sorts, fallback) {
  const cur = has(sorts, f.sort) ? f.sort : fallback;
  const dir = f.dir || sorts[cur].firstDir || "asc";
  return h("div", { class: "sort" },
    h("label", { for: "sort", class: "sort-label" }, "Sort"),
    h("select", {
      id: "sort", title: sorts[cur].title || null,
      on: { change: (e) => setFilters(path, f, { sort: e.target.value, dir: "" }) },
    }, Object.entries(sorts).map(([k, s]) => h("option", { value: k, selected: k === cur }, s.label))),
    h("button", {
      type: "button", id: "sort-dir", class: "icon-button",
      "aria-label": dir === "desc" ? "Descending; select for ascending" : "Ascending; select for descending",
      on: { click: () => setFilters(path, f, { sort: cur, dir: dir === "desc" ? "asc" : "desc" }) },
    }, dir === "desc" ? "↓" : "↑"));
}

/** Rows sorted as the URL says, falling back to `fallback`. */
export function sorted(rows, sorts, f, fallback) {
  const cur = has(sorts, f.sort) ? f.sort : fallback;
  return sortRows(rows, sorts[cur].sort, f.dir || sorts[cur].firstDir || "asc");
}
