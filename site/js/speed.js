// Your own speed, for the person signed in: three measures by month, each a
// small chart with its latest value, and every month as a table. The service
// sends rolling 3-month medians in hours with how many they count; with
// fewer than 3 it sends no median, and the page shows the count alone.
// Speed is never coloured as good or bad: the page does not judge or rank it.

import { h } from "./dom.js";
import { sortableTable } from "./ui.js";

const MONTH_RE = /^([0-9]{4})-(0[1-9]|1[0-2])$/;
const MONTH_NAMES = ["Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"];
// More than a year and its margin is not something this panel draws.
const MAX_MONTHS = 24;

const MEASURES = [
  {
    key: "review_wait",
    title: "Review wait you gave",
    counts: ["answered ask", "answered asks"],
    about: "From when you were first asked to review a PR to your first decisive review. Only answered asks are timed.",
  },
  {
    key: "your_turn",
    title: "Your turn",
    counts: ["turn", "turns"],
    about: "On your PRs, the time each turn spent in your own stages: self-review, answering an objection, a failed build.",
  },
  {
    key: "cycle_time",
    title: "Cycle time",
    counts: ["merged PR", "merged PRs"],
    about: "First ready for review to merge, on your merged PRs. Includes other people's review time.",
  },
];

const obj = (x) => (x && typeof x === "object" && !Array.isArray(x) ? x : {});
const count = (x) => (Number.isSafeInteger(x) && x >= 0 ? x : 0);
const hours = (x) => (typeof x === "number" && Number.isFinite(x) && x >= 0 ? x : null);
const stat = (x) => ({ median: hours(obj(x).median_hours), n: count(obj(x).n) });

/**
 * The speed answer, checked: `{optedOut: true}`, or `{since, months}` with
 * `since` a Date or null and `months` oldest first. Anything malformed in a
 * month reads as nothing recorded; a month that is not a month is dropped.
 */
export function readSpeed(raw) {
  const o = obj(raw);
  if (o.opted_out === true) return { optedOut: true };
  const since = typeof o.since === "string" ? Date.parse(o.since) : NaN;
  const byMonth = new Map();
  for (const m of Array.isArray(o.months) ? o.months : []) {
    const month = obj(m).month;
    if (typeof month !== "string" || !MONTH_RE.test(month) || byMonth.has(month)) continue;
    const wait = obj(m.review_wait);
    byMonth.set(month, {
      month,
      review_wait: { ...stat(wait), asked: count(wait.asked), answered: count(wait.answered), open: count(wait.open) },
      your_turn: stat(m.your_turn),
      cycle_time: stat(m.cycle_time),
    });
  }
  const months = [...byMonth.values()].sort((a, b) => (a.month < b.month ? -1 : 1)).slice(-MAX_MONTHS);
  return { optedOut: false, since: Number.isFinite(since) ? new Date(since) : null, months };
}

// Hours up to 72, days beyond, in the table, the tiles and the chart axes alike.
const DAYS_FROM_HOURS = 72;

/** "45m", "7.5h", "36h", "4.2d": a median, to one decimal while it is small. */
function formatMedian(h) {
  if (h < 1) return `${Math.round(h * 60)}m`;
  if (h < DAYS_FROM_HOURS) return `${h < 10 ? +h.toFixed(1) : Math.round(h)}h`;
  const d = h / 24;
  return `${d < 10 ? +d.toFixed(1) : Math.round(d)}d`;
}

function monthParts(month) {
  const [, y, m] = MONTH_RE.exec(month);
  return [Number(y), Number(m)];
}

function monthLabel(month) {
  const [y, m] = monthParts(month);
  return `${MONTH_NAMES[m - 1]} ${y}`;
}

/** The three months a rolling median covers: "Aug–Oct 2026", or "Nov 2025–Jan 2026". */
function windowLabel(month) {
  const [y, m] = monthParts(month);
  const startY = m > 2 ? y : y - 1;
  const startM = ((m + 9) % 12) + 1;
  return startY === y ? `${MONTH_NAMES[startM - 1]}–${MONTH_NAMES[m - 1]} ${y}` : `${MONTH_NAMES[startM - 1]} ${startY}–${MONTH_NAMES[m - 1]} ${y}`;
}

function dateLabel(d) {
  return `${d.getUTCDate()} ${MONTH_NAMES[d.getUTCMonth()]} ${d.getUTCFullYear()}`;
}

function plural(measure, n) {
  return `${n} ${measure.counts[n === 1 ? 0 : 1]}`;
}

function anything(key, s) {
  return s.n > 0 || (key === "review_wait" && (s.asked > 0 || s.answered > 0 || s.open > 0));
}

/** The index of the first month with anything recorded for a measure, or -1. */
function firstRecorded(months, key) {
  return months.findIndex((m) => anything(key, m[key]));
}

/** A measure for one month in words: "18h · n 12", or the count alone. */
function statText(s) {
  if (s.median !== null) return `${formatMedian(s.median)} · n ${s.n}`;
  return s.n ? `n ${s.n}, too few for a median` : "none";
}

function asksText(w) {
  return `asked ${w.asked} · answered ${w.answered} · still open ${w.open}`;
}

/** The panel's body for a speed answer that is not an opt-out. */
export function speedBody(speed) {
  const { since, months } = speed;
  const sinceText = since ? `Recorded since ${dateLabel(since)}. ` : "";
  if (!months.length) {
    return [h("p", {}, `${sinceText}Nothing of yours has been recorded yet.`)];
  }
  return [
    h("p", { class: "sub" }, sinceText,
      "Each month shows the median of the three months up to it, and how many it counts (n); with fewer than 3, only n."),
    h("div", { class: "speed-tiles" }, MEASURES.map((m) => tile(m, months))),
    h("details", { class: "speed-table" },
      h("summary", {}, "Every month as a table, newest first"),
      h("p", { class: "sub" }, "— : nothing recorded for you yet in that measure."),
      monthTable(months)),
  ];
}

function tile(measure, months) {
  const latest = months[months.length - 1];
  const s = latest[measure.key];
  const recorded = firstRecorded(months, measure.key) >= 0;
  const value = s.median !== null ? formatMedian(s.median) : "—";
  let of;
  if (s.median !== null) of = `median of ${plural(measure, s.n)}`;
  else if (s.n) of = `${plural(measure, s.n)}, too few for a median`;
  else of = recorded ? `no ${measure.counts[1]}` : "nothing recorded yet";
  const holder = h("div", { class: "plot-holder speed-plot" });
  const out = h("div", { class: "speed-tile" },
    h("h3", {}, measure.title),
    h("p", { class: "speed-value" }, value),
    h("p", { class: "speed-of" }, `${windowLabel(latest.month)} · ${of}`),
    measure.key === "review_wait" && recorded ? h("p", { class: "speed-of" }, asksText(s)) : null,
    holder,
    h("p", { class: "speed-about" }, measure.about));
  if (!months.some((m) => m[measure.key].median !== null)) {
    holder.append(h("p", { class: "muted" }, "No month has enough yet for a median."));
  } else {
    drawWhenSized(holder, (width) => chart(measure, months, width));
  }
  return out;
}

function monthTable(months) {
  const first = Object.fromEntries(MEASURES.map((m) => [m.key, firstRecorded(months, m.key)]));
  // Newest first; a month before a measure's first record shows "—", not "none".
  const rows = months.map((m, i) => ({ m, i })).reverse();
  const cell = (key, { m, i }) => {
    if (first[key] < 0 || i < first[key]) return h("span", { class: "muted" }, "—");
    const s = m[key];
    if (key !== "review_wait" || !anything(key, s)) return statText(s);
    return [h("div", {}, statText(s)), h("div", { class: "muted" }, asksText(s))];
  };
  return sortableTable({
    columns: [
      { key: "month", label: "Month", cell: ({ m }) => monthLabel(m.month) },
      ...MEASURES.map((measure) => ({ key: measure.key, label: measure.title, cell: (r) => cell(measure.key, r) })),
    ],
    rows,
  });
}

function cssVar(name) {
  return getComputedStyle(document.documentElement).getPropertyValue(name).trim();
}

const CHART_HEIGHT = 128;

/** Draw once laid out and again when the width changes; a colour-scheme change re-renders the page. */
function drawWhenSized(holder, draw) {
  if (!globalThis.Plot) {
    holder.append(h("p", { class: "muted" }, "The chart library did not load; the table below has every month."));
    return;
  }
  holder.style.minHeight = `${CHART_HEIGHT}px`;
  let last = 0;
  const ro = new ResizeObserver(() => {
    if (!holder.isConnected) return ro.disconnect();
    const width = holder.clientWidth;
    if (width && width !== last) {
      last = width;
      try {
        holder.replaceChildren(draw(width));
      } catch (e) {
        holder.replaceChildren(h("p", { class: "muted" }, `The chart could not be drawn (${e.message}); the table below has every month.`));
      }
    }
  });
  ro.observe(holder);
}

/** The unit a chart's axis counts in, the same as formatMedian picks for its largest value. */
function axisUnit(maxHours) {
  if (maxHours < 1) return { per: 1 / 60, suffix: "m" };
  if (maxHours < DAYS_FROM_HOURS) return { per: 1, suffix: "h" };
  return { per: 24, suffix: "d" };
}

// Month-label steps that divide a year, so January (labelled with its year)
// is always one of the labelled months.
const TICK_STEPS = [1, 2, 3, 4, 6, 12];

function chart(measure, months, width) {
  const Plot = globalThis.Plot;
  const domain = months.map((m) => m.month);
  const maxHours = Math.max(...months.map((m) => m[measure.key].median ?? 0));
  const unit = axisUnit(maxHours);
  const points = months.map((m) => {
    const s = m[measure.key];
    return { month: m.month, s, y: s.median === null ? NaN : s.median / unit.per };
  });
  const marginLeft = 36;
  const marginRight = 10;
  const need = Math.ceil((domain.length * 30) / Math.max(1, width - marginLeft - marginRight));
  const step = TICK_STEPS.find((s) => s >= need) ?? 12;
  const ticks = domain.filter((m) => (monthParts(m)[1] - 1) % step === 0);
  const series = cssVar("--series");
  const surface = cssVar("--surface");
  const ink2 = cssVar("--ink-2");
  const tipText = (d) => {
    const lines = [`${windowLabel(d.month)}: ${statText(d.s)}`];
    if (measure.key === "review_wait") lines.push(asksText(d.s));
    return lines.join("\n");
  };
  const svgEl = Plot.plot({
    width,
    height: CHART_HEIGHT,
    marginLeft,
    marginRight,
    marginTop: 18,
    marginBottom: 22,
    style: { background: "transparent", color: ink2, fontFamily: "inherit", fontSize: "11px" },
    x: {
      type: "point", domain, ticks, padding: 0.4, label: null, tickSize: 0,
      // A month's name, and the year at January.
      tickFormat: (m) => {
        const [y, mo] = monthParts(m);
        return mo === 1 ? String(y) : MONTH_NAMES[mo - 1];
      },
    },
    y: {
      domain: [0, (maxHours / unit.per) * 1.15 || 1], ticks: 3, label: null, tickSize: 0,
      tickFormat: (v) => `${+v.toFixed(1)}${unit.suffix}`,
    },
    marks: [
      Plot.gridY({ ticks: 3, stroke: cssVar("--grid"), strokeOpacity: 1 }),
      Plot.ruleY([0], { stroke: cssVar("--axis") }),
      // A month with no median breaks the line rather than dropping to zero.
      Plot.lineY(points, { x: "month", y: "y", stroke: series, strokeWidth: 2, strokeLinejoin: "round", strokeLinecap: "round" }),
      Plot.dot(points.filter((d) => d.s.median !== null), { x: "month", y: "y", r: 4, fill: series, stroke: surface, strokeWidth: 2 }),
      // Counts too small for a median sit above the plot, away from the values.
      Plot.text(points.filter((d) => d.s.median === null && d.s.n > 0), {
        x: "month", frameAnchor: "top", lineAnchor: "bottom", dy: -4, text: (d) => `n ${d.s.n}`, fill: ink2, fontSize: 10,
      }),
      // The pointer snaps to the nearest month, so it need not land on a dot.
      Plot.ruleX(points, Plot.pointerX({ x: "month", stroke: cssVar("--axis") })),
      Plot.tip(points, Plot.pointerX({ x: "month", y: (d) => (Number.isFinite(d.y) ? d.y : 0), title: tipText, fontSize: 12 })),
    ],
  });
  svgEl.setAttribute("role", "img");
  svgEl.setAttribute("aria-label", `${measure.title} by month; the table below has every month.`);
  return svgEl;
}
