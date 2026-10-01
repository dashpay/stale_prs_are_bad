// Small pieces every view uses: PR and person links, the time-in-stage cell,
// flag badges and a sortable table.

import { h, svg } from "./dom.js";
import { LATENESS, has, formatDuration, personHref, avatarUrl, shortRepo, areaLabel } from "./model.js";

export const BOT = "🤖";

export function prLink(pr, { withTitle = true } = {}) {
  const label = `${shortRepo(pr.repo)}#${pr.number}`;
  const ref = pr.url
    ? h("a", { href: pr.url, rel: "noopener noreferrer", target: "_blank" }, label)
    : h("span", { class: "nolink" }, label);
  if (!withTitle) return ref;
  return h("span", { class: "pr" }, ref, " ", h("span", { class: "pr-title" }, pr.title));
}

export function avatar(login) {
  const src = avatarUrl(login);
  return src
    ? h("img", { class: "avatar", src, alt: "", width: 20, height: 20, loading: "lazy", referrerpolicy: "no-referrer" })
    : h("span", { class: "avatar avatar-none", "aria-hidden": "true" });
}

export function botMark() {
  return h("span", { class: "bot", title: "bot account", "aria-label": "bot" }, BOT);
}

/** A person's name linking to their page; a bot gets the robot mark. */
export function personLink(login, isBot, { withAvatar = false } = {}) {
  if (login === null || login === undefined) return h("span", { class: "muted" }, "unknown");
  const href = personHref(login);
  const name = href ? h("a", { href }, login) : h("span", {}, String(login));
  return h("span", { class: "person" }, withAvatar ? avatar(login) : null, name, isBot ? [" ", botMark()] : null);
}

export function stageChip(data, key) {
  const s = has(data.stage, key) ? data.stage[key] : data.stage.unknown;
  return h("span", { class: `stage stage-${s.key}`, title: s.note }, s.label);
}

/** An area's name: an id in code style, or the words for files no area claims. */
export function areaName(area) {
  return area === "fallback" ? h("span", {}, areaLabel(area)) : h("code", {}, String(area));
}

/**
 * What review the PR still waits for, in the engine's own words: each area
 * nobody has approved with who may (an area nobody may approve says so), then
 * the people whose objection is still open.
 */
export function asksList(pr) {
  const items = pr.asks.map((a) => h("li", {}, areaName(a.area), ": ",
    a.approvers.length ? a.approvers.join(" or ") : h("strong", {}, "nobody may approve")));
  if (pr.objectors.length) items.push(h("li", {}, "re-review or resolve: ", pr.objectors.join(", ")));
  return items.length ? h("ul", { class: "asks" }, items) : null;
}

// Lateness is a shape as well as a colour, so it reads without colour vision:
// on time a circle, late a triangle, very late a diamond.
const SHAPES = {
  ok: () => svg("circle", { cx: 6, cy: 6, r: 4.5 }),
  late: () => svg("polygon", { points: "6,0.8 11.4,10.6 0.6,10.6" }),
  "very-late": () => svg("polygon", { points: "6,0.3 11.7,6 6,11.7 0.3,6" }),
};

/** The mark for a lateness key ("ok", "late", "very-late"), or a neutral circle. */
export function lateMark(kind, extraClass = "") {
  const shape = has(SHAPES, kind) ? SHAPES[kind]() : svg("circle", { cx: 6, cy: 6, r: 4.5 });
  return svg("svg", { class: `mark ${extraClass}`.trim(), width: 12, height: 12, viewBox: "0 0 12 12", "aria-hidden": "true" }, shape);
}

/** The status mark for one PR: its lateness shape and colour, hollow when the stage start is not recorded. */
export function statusMark(pr) {
  if (pr.since_basis !== "engine") return lateMark(null, "mark-opened");
  return pr.lateness ? lateMark(pr.lateness, `mark-${pr.lateness}`) : lateMark(null, "mark-untimed");
}

/** Time in the current stage with what it is measured from and, when known, its lateness. */
export function timeCell(pr) {
  if (pr.ageMs === null) return h("span", { class: "muted" }, "no start time");
  const parts = [statusMark(pr), " ", h("span", { class: "num" }, formatDuration(pr.ageMs))];
  if (pr.since_basis === "engine" && pr.lateness) {
    parts.push(" ", h("span", { class: `late-label late-${pr.lateness}` }, LATENESS[pr.lateness].label));
  } else if (pr.since_basis === "opened") {
    parts.push(" ", h("span", { class: "basis" }, "since opened"));
  }
  return h("span", { class: "time" }, parts);
}

export function badges(pr) {
  const out = [];
  if (pr.unresolved_comments > 0) {
    out.push(h("span", { class: "badge", title: "unresolved review comments" }, `💬 ${pr.unresolved_comments}`));
  }
  if (pr.ci_failing) out.push(h("span", { class: "badge badge-bad" }, "CI failing"));
  if (pr.merge_conflict) out.push(h("span", { class: "badge badge-bad" }, "conflict"));
  if (pr.changes_requested) out.push(h("span", { class: "badge" }, "changes requested"));
  if (pr.idle) out.push(h("span", { class: "badge", title: "no update for the idle period" }, "idle"));
  return out.length ? h("span", { class: "badges" }, out) : null;
}

export function flagCount(pr) {
  return (pr.unresolved_comments > 0) + !!pr.ci_failing + !!pr.merge_conflict + !!pr.changes_requested + pr.idle;
}

export function section(title, ...children) {
  return h("section", { class: "card" }, h("h2", {}, title), children);
}

/**
 * A table whose headers sort it. `columns`: {key, label, cell(row), sort?(row), firstDir?, numeric?, title?, className?}.
 * `sort`/`dir` come from the URL; `onSort(key, dir)` navigates; `id` names the sort buttons.
 */
export function sortableTable({ id, caption, columns, rows, sort, dir, onSort, empty, className = "" }) {
  const col = columns.find((c) => c.key === sort && c.sort) || null;
  // A link that names a column but no direction sorts the way a first click would.
  dir = dir || col?.firstDir || "asc";
  let sorted = rows;
  if (col) {
    const sign = dir === "desc" ? -1 : 1;
    sorted = [...rows].sort((a, b) => sign * compare(col.sort(a), col.sort(b)));
  }
  const sortable = !!onSort && columns.some((c) => c.sort);
  // Explicit roles keep the table a table for screen readers when narrow
  // screens lay its rows out as cards.
  const head = h("tr", { role: "row" }, columns.map((c) => {
    const active = col && c.key === col.key;
    const ariaSort = active ? (dir === "desc" ? "descending" : "ascending") : null;
    const canSort = sortable && c.sort;
    const th = h("th", { scope: "col", role: "columnheader", class: cellClass(c, canSort ? "" : "th-plain"), "aria-sort": ariaSort, title: c.title || null });
    if (!canSort) {
      th.append(c.label);
      return th;
    }
    const nextDir = active ? (dir === "desc" ? "asc" : "desc") : (c.firstDir || "asc");
    // The id lets the page put focus back on this button after it re-renders.
    th.append(h("button", { type: "button", class: "sort", id: id ? `sort-${id}-${c.key}` : null, on: { click: () => onSort(c.key, nextDir) } },
      c.label, h("span", { class: "sort-arrow", "aria-hidden": "true" }, active ? (dir === "desc" ? " ▼" : " ▲") : "")));
    return th;
  }));
  const body = sorted.length
    ? sorted.map((r) => h("tr", { role: "row" }, columns.map((c) =>
      h("td", { role: "cell", class: cellClass(c), "data-label": c.label }, h("div", { class: "cell" }, c.cell(r))))))
    : h("tr", { role: "row" }, h("td", { role: "cell", colspan: columns.length, class: "empty" }, empty || "Nothing here."));
  return h("div", { class: `table-scroll${sortable ? "" : " unsorted"} ${className}` },
    h("table", { role: "table" }, caption ? h("caption", {}, caption) : null,
      h("thead", { role: "rowgroup" }, head), h("tbody", { role: "rowgroup" }, body)));
}

function cellClass(c, extra = "") {
  return [c.numeric ? "n" : "", c.className || "", extra].filter(Boolean).join(" ") || null;
}

function compare(a, b) {
  if (Array.isArray(a)) {
    for (let i = 0; i < a.length; i++) {
      const c = compare(a[i], b[i]);
      if (c) return c;
    }
    return 0;
  }
  if (typeof a === "number" && typeof b === "number") return a - b;
  return String(a ?? "").localeCompare(String(b ?? ""), undefined, { sensitivity: "base", numeric: true });
}
