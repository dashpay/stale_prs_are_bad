// Team view: whose move it is, how long each PR has sat in its stage, every
// open PR, and the ones to close or revive.

import { h, svg, openPr } from "./dom.js";
import {
  DAY_MS, LATENESS, PARKED, formatDuration, formatHours, shortRepo, closeOrRevive, idleWords,
} from "./model.js";
import { buildHash, setFilters } from "./state.js";
import { filterPrs } from "./filters.js";
import {
  prLink, personLink, stageChip, timeCell, badges, flagCount, section, sortableTable, lateMark, asksList,
} from "./ui.js";

export function teamView(data, f) {
  const visible = filterPrs(data.prs, f);
  const forTiles = filterPrs(data.prs, f, { ignoreOwner: true });
  const empty = emptyText(data, f);
  return [
    tiles(data, forTiles, f),
    section("How long each PR has been in its stage", agingPlot(data, visible, empty)),
    section(`Open PRs (${visible.length})`, prTable(data, visible, f, empty)),
    reviveSection(data, visible),
  ];
}

/** Why a filtered list is empty. "Late only" can only ever find stages that record their start. */
function emptyText(data, f) {
  if (!f.late) return "No PRs match the filters.";
  const timed = data.stages.filter((s) => s.entryRecorded && s.lateHours).map((s) => s.label);
  return timed.length
    ? `No late PRs match the filters. Only ${timed.join(", ")} records when a PR entered it, so only PRs there can be late.`
    : "No stage records when a PR entered it yet, so no PR can be late.";
}

function tiles(data, prs, f) {
  const who = f.who || "people";
  const scope = who === "people"
    ? "Open PRs waiting on people (humans' PRs, and bots' PRs waiting on reviewers or maintainers)"
    : who === "bot" ? "Open PRs by bots" : "All open PRs, bots' own included";
  const toggle = who === "people"
    ? h("a", { href: buildHash("/", { ...f, who: "all" }) }, "include all bot PRs")
    : h("a", { href: buildHash("/", { ...f, who: "" }) }, "only what waits on people");
  const tile = (o) => {
    const items = prs.filter((p) => p.owner === o.key);
    const parts = data.stages
      .filter((s) => (o.key === PARKED ? !s.inFlow : s.inFlow && s.owner === o.key))
      .map((s) => [s, items.filter((p) => p.stage === s.key).length])
      .filter(([, n]) => n);
    const late = items.filter((p) => p.lateRank >= 2);
    const worst = late.some((p) => p.lateness === "very-late") ? "very-late" : "late";
    const active = f.owner === o.key;
    return h("li", {},
      h("a", {
        href: buildHash("/", { ...f, owner: active ? "" : o.key }),
        id: `tile-${o.key}`,
        class: `tile${items.length ? "" : " tile-zero"}${active ? " tile-active" : ""}${o.key === PARKED ? " tile-parked" : ""}`,
        "aria-current": active ? "true" : null,
      },
      h("span", { class: "tile-label" }, o.label),
      h("span", { class: "tile-value" }, String(items.length)),
      h("span", { class: "tile-sub" }, parts.length
        ? parts.map(([s, n], i) => [i ? " · " : "", h("span", { class: s.key === "unknown" ? "hatched" : "" }, `${s.label} ${n}`)])
        : "none open"),
      late.length
        ? h("span", { class: "tile-late" }, lateMark(worst, `mark-${worst}`), ` ${late.length} late`,
          worst === "very-late" ? ` (${late.filter((p) => p.lateness === "very-late").length} very late)` : "")
        : null));
  };
  const flow = data.owners.filter((o) => o.key !== PARKED);
  const parked = data.owners.find((o) => o.key === PARKED);
  return h("section", { class: "card" },
    h("h2", {}, "Whose move"),
    h("p", { class: "sub" }, `${scope} (`, toggle, "), by who has to act next. Select a tile to list only those."),
    h("ul", { class: "tiles" }, flow.map(tile)),
    parked ? [
      h("h3", { class: "tiles-heading" }, "Parked"),
      h("p", { class: "sub" }, "Drafts, PRs off the governed branches and PRs with no engine verdict: nobody is asked to move them today."),
      h("ul", { class: "tiles" }, tile(parked)),
    ] : null);
}

// Deterministic vertical spread inside a stage's row, so dots do not stack
// and do not jump between renders.
function jitter(key) {
  const s = String(key);
  let x = 2166136261;
  for (let i = 0; i < s.length; i++) x = Math.imul(x ^ s.charCodeAt(i), 16777619);
  return (((x >>> 0) % 1000) / 1000 - 0.5) * 0.62;
}

function cssVar(name) {
  return getComputedStyle(document.documentElement).getPropertyValue(name).trim();
}

const TICKS = [
  [1 / 24, "1h"], [1, "1d"], [7, "1w"], [30, "1mo"], [90, "3mo"], [365, "1y"], [730, "2y"], [1095, "3y"],
];

/** The stages to draw, the review flow first and parked work below it. */
function plotRows(data, prs) {
  return data.stages
    .filter((s) => prs.some((p) => p.stage === s.key))
    .sort((a, b) => b.inFlow - a.inFlow || a.order - b.order);
}

function agingPlot(data, prs, emptyMessage) {
  const fig = h("figure", { class: "aging" });
  const plotted = prs.filter((p) => p.ageMs !== null);
  const missing = prs.length - plotted.length;
  const note = h("figcaption", { class: "sub" },
    "One dot per PR; further right has waited longer. Hover for the PR, click to open it on GitHub; the table below lists the same PRs.",
    missing ? ` ${missing} PR${missing === 1 ? " has" : "s have"} no start time and ${missing === 1 ? "is" : "are"} only in the table.` : "");
  const holder = h("div", { class: "plot-holder" });
  fig.append(agingLegend(data, plotted), holder, note);
  if (!plotted.length) {
    holder.append(h("p", { class: "empty" }, emptyMessage));
    return fig;
  }
  if (!globalThis.Plot) {
    holder.append(h("p", { class: "empty" }, "The chart library did not load; the table below lists the same PRs."));
    return fig;
  }
  // Reserve the chart's height up front so the page does not jump when it draws.
  holder.style.minHeight = `${plotHeight(plotRows(data, plotted).length, innerWidth < 560)}px`;
  // Drawn once laid out and again when the width changes; a colour-scheme
  // change re-renders the page, which builds a new plot.
  let last = 0;
  const ro = new ResizeObserver(() => {
    if (!holder.isConnected) return ro.disconnect();
    const width = holder.clientWidth;
    if (width && width !== last) {
      last = width;
      try {
        holder.replaceChildren(renderPlot(data, plotted, width));
      } catch (e) {
        holder.replaceChildren(h("p", { class: "empty" }, `The chart could not be drawn (${e.message}); the table below lists the same PRs.`));
      }
    }
  });
  ro.observe(holder);
  return fig;
}

function plotHeight(rows, narrow) {
  return rows * (narrow ? 40 : 46) + 46;
}

// Plot's symbol names for each lateness, matching the marks in tables and the legend.
const SYMBOL = { ok: "circle", late: "triangle", "very-late": "diamond" };

function renderPlot(data, prs, width) {
  const Plot = globalThis.Plot;
  const present = plotRows(data, prs);
  const row = new Map(present.map((s, i) => [s.key, i]));
  const firstParked = present.findIndex((s) => !s.inFlow);
  // x is log10(days), on a linear scale, so the page picks every tick and its
  // label; a log scale would drop labels it thinks are crowded.
  const points = prs.map((p) => ({
    p,
    x: Math.log10(Math.max(p.ageMs / DAY_MS, 1 / 24)),
    y: row.get(p.stage) + jitter(p.key),
  }));
  const maxDays = Math.max(30, ...points.map((d) => 10 ** d.x)) * 1.3;
  const ticks = TICKS.filter(([t]) => t <= maxDays).map(([t, text]) => [Math.log10(t), text]);
  const label = new Map(ticks);
  const surface = cssVar("--surface");
  const tip = (d) => {
    const p = d.p;
    const since = p.since_basis === "engine" ? "in this stage" : "since opened; stage start not recorded yet";
    const late = p.lateness ? ` · ${LATENESS[p.lateness].label}` : "";
    return `${shortRepo(p.repo)}#${p.number} ${p.title}\n${data.stage[p.stage].label} · ${formatDuration(p.ageMs)} ${since}${late}`;
  };
  const narrow = width < 560;
  const timed = (kind) => points.filter((d) => d.p.since_basis === "engine" && d.p.lateness === kind);
  const svgEl = Plot.plot({
    width,
    height: plotHeight(present.length, narrow),
    marginLeft: narrow ? 88 : 104,
    marginRight: 16,
    marginTop: 8,
    marginBottom: 38,
    style: { background: "transparent", color: cssVar("--ink-2"), fontFamily: "inherit", fontSize: "12px" },
    x: {
      domain: [Math.log10(1 / 24), Math.log10(maxDays)], ticks: ticks.map(([t]) => t),
      tickFormat: (t) => label.get(t) ?? "",
      // Honest about what most dots measure: only some stages record their start.
      label: narrow ? "time in stage, or since opened →" : "time in stage (since opened where the stage start isn't recorded) →",
      labelAnchor: "right", labelArrow: "none",
    },
    y: {
      domain: [present.length - 0.5, -0.5], ticks: present.map((_, i) => i),
      tickFormat: (i) => present[i]?.label ?? "", tickSize: 0, label: null,
    },
    marks: [
      Plot.gridX(ticks.map(([t]) => t), { stroke: cssVar("--grid"), strokeOpacity: 1 }),
      Plot.ruleY(present.slice(1).map((_, i) => i + 0.5), { stroke: cssVar("--grid") }),
      // Parked work sits below a heavier rule.
      ...(firstParked > 0 ? [Plot.ruleY([firstParked - 0.5], { stroke: cssVar("--axis"), strokeWidth: 2 })] : []),
      // Stage start unknown: hollow and neutral, never coloured as late or on time.
      Plot.dot(points.filter((d) => d.p.since_basis !== "engine"), { x: "x", y: "y", r: 4, fill: surface, stroke: cssVar("--neutral"), strokeWidth: 1.5 }),
      Plot.dot(points.filter((d) => d.p.since_basis === "engine" && !d.p.lateness), { x: "x", y: "y", r: 5, fill: cssVar("--untimed"), stroke: surface, strokeWidth: 2 }),
      ...Object.entries(SYMBOL).map(([kind, symbol]) => Plot.dot(timed(kind), {
        x: "x", y: "y", r: kind === "ok" ? 5 : 6.5, symbol, fill: cssVar(`--late-${kind}`), stroke: surface, strokeWidth: 1.5,
      })),
      // One invisible layer over every dot drives the tooltip: the pointer
      // picks the nearest dot, so it need not land on an 8px mark.
      Plot.dot(points, { x: "x", y: "y", r: 5, fill: "transparent", stroke: "none", title: tip, tip: { fontSize: 12, lineWidth: 40 } }),
    ],
  });
  // A click opens the PR the tooltip is showing, not whichever mark happens
  // to be on top where dots overlap.
  svgEl.addEventListener("input", () => {
    svgEl.style.cursor = svgEl.value?.p?.url ? "pointer" : "";
  });
  svgEl.addEventListener("click", () => {
    const url = svgEl.value?.p?.url;
    if (url) openPr(url);
  });
  // A picture to assistive technology; the table below is the keyboard and
  // screen-reader route to the same PRs.
  svgEl.setAttribute("role", "img");
  svgEl.setAttribute("aria-label", `Time in stage for ${prs.length} PRs; the table below lists them.`);
  return svgEl;
}

function agingLegend(data, prs) {
  const keys = [
    [lateMark("ok", "mark-ok"), "on time"],
    [lateMark("late", "mark-late"), "late"],
    [lateMark("very-late", "mark-very-late"), "very late"],
  ];
  if (prs.some((p) => p.since_basis === "engine" && !p.lateness)) keys.push([lateMark(null, "mark-untimed"), "never late in this stage"]);
  keys.push([lateMark(null, "mark-opened"), "time since opened; stage start not recorded yet"]);
  const rules = data.stages
    .filter((s) => s.entryRecorded && s.lateHours)
    .map((s) => `${s.label}: late after ${formatHours(s.lateHours[0])}, very late after ${formatHours(s.lateHours[1])}`);
  const parked = plotRows(data, prs).filter((s) => !s.inFlow).map((s) => s.label);
  const anyFlow = prs.some((p) => p.inFlow);
  return h("div", {},
    h("ul", { class: "legend", "aria-label": "Legend" }, keys.map(([mark, text]) => h("li", {}, mark, text))),
    rules.length ? h("p", { class: "sub legend-rules" }, `${rules.join(" · ")}. Only these stages record when a PR entered them.`) : null,
    parked.length && anyFlow ? h("p", { class: "sub legend-rules" }, `Below the heavier line: parked work (${parked.join(", ")}).`) : null);
}

function prTable(data, prs, f, empty) {
  const columns = [
    { key: "repo", label: "Repo", className: "col-repo", sort: (p) => [p.repo, p.number ?? 0], cell: (p) => shortRepo(p.repo) },
    { key: "pr", label: "PR", sort: (p) => [p.title], cell: (p) => prCell(p) },
    { key: "author", label: "Author", sort: (p) => [p.author ?? ""], cell: (p) => personLink(p.author, p.isBot) },
    { key: "stage", label: "Stage", sort: (p) => [data.stage[p.stage].order, -(p.ageMs ?? -1)], cell: (p) => stageChip(data, p.stage) },
    {
      // The default order: the review flow before parked work, late first, then longest.
      key: "time", label: "Time in stage", firstDir: "desc", title: "In-flow PRs first, late first, then longest",
      sort: (p) => [Number(p.inFlow), p.lateRank, p.ageMs ?? -1], cell: (p) => timeCell(p),
    },
    { key: "next", label: "Next action", sort: (p) => [p.next_action ?? "~"], cell: (p) => nextCell(p) },
    { key: "flags", label: "Flags", firstDir: "desc", sort: (p) => [flagCount(p), p.unresolved_comments], cell: (p) => badges(p) },
  ];
  return sortableTable({
    id: "prs", columns, rows: prs, sort: f.sort || "time", dir: f.dir,
    onSort: (key, dir) => setFilters("/", f, { sort: key, dir }),
    empty, className: "pr-table",
  });
}

function nextCell(p) {
  const asks = asksList(p);
  if (!p.next_action && !asks) return h("span", { class: "muted" }, "—");
  return h("div", { class: "next" }, p.next_action ? h("span", {}, p.next_action) : null, asks);
}

function prCell(p) {
  return h("span", { class: "pr-cell" }, prLink(p, { withTitle: false }), h("span", { class: "pr-title" }, p.title),
    p.tracked ? null : h("span", { class: "basis", title: "Known only from the review engine; the board's own filters left it out" }, "engine only"));
}

function reviveSection(data, prs) {
  const rows = closeOrRevive(prs);
  const idle = idleWords(data);
  const table = sortableTable({
    columns: [
      { key: "pr", label: "PR", cell: (r) => prCell(r.pr) },
      { key: "author", label: "Author", cell: (r) => personLink(r.pr.author, r.pr.isBot) },
      { key: "stage", label: "Stage", cell: (r) => stageChip(data, r.pr.stage) },
      { key: "updated", label: "Last update", numeric: true, cell: (r) => r.pr.updatedAgoMs === null ? "—" : `${formatDuration(r.pr.updatedAgoMs)} ago` },
      { key: "why", label: "Why", cell: (r) => [r.idle ? idle : null, r.conflict ? "merge conflict" : null].filter(Boolean).join(" · ") },
    ],
    rows, empty: "Nothing idle or conflicting.",
  });
  const period = data.idleDays ? `for ${data.idleDays} days or more` : "for the idle period";
  return section(`Close or revive (${rows.length})`,
    h("p", { class: "sub" }, `Drafts and PRs off the governed branches that nobody has touched ${period}, and PRs with a merge conflict.`),
    table);
}
