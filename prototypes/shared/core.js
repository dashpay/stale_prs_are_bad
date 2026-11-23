// Shared plumbing for the UI prototypes. The data goes through the site's own
// prepare(), filters through the site's own filterPrs(), so both prototypes
// show exactly what the live page shows; only the presentation differs.

import { h } from "../../site/js/dom.js";
import {
  prepare, STALE_HOURS, LOGIN_RE, PARKED, findPerson, formatDuration, shortRepo,
} from "../../site/js/model.js";
import { filterPrs, matchesMilestone } from "../../site/js/filters.js";
import { flagCount } from "../../site/js/ui.js";

// Served from a laptop, the prototypes read the live service, which answers
// any origin. Served beside the real page, they read its own copy.
const LIVE = "https://pr-hygiene.services.dash.org/dashboard.json";
export const DATA_URL = /^(localhost|127\.0\.0\.1)$/.test(location.hostname) ? LIVE : "/dashboard.json";

export async function loadData() {
  let raw;
  try {
    const res = await fetch(DATA_URL, { cache: "no-cache" });
    if (!res.ok) throw new Error(`HTTP ${res.status}`);
    raw = await res.json();
  } catch (e) {
    throw new Error(`dashboard.json could not be loaded (${e.message}).`);
  }
  return prepare(raw);
}

export function isStale(data) {
  return Date.now() - data.asOf > STALE_HOURS * 3_600_000;
}

/** The live page's warnings, word for word: stale data, and per repository what could not be read. */
export function warnings(data) {
  const out = [];
  if (isStale(data)) {
    out.push(`This data is ${formatDuration(Date.now() - data.asOf)} old; the update job may have stopped. Times below are as of the snapshot.`);
  }
  for (const r of data.repos) {
    if (r.fetch_error) out.push(`${r.repo} could not be fetched: ${r.fetch_error}. Its PRs may be missing.`);
    if (r.stage_times_error) {
      out.push(`For some ${r.repo} PRs, the time they entered their stage could not be read (${r.stage_times_error}). ` +
        "Those show their age instead and are not marked late, except a review, which keeps the review engine's own start.");
    }
    if (!r.engine_state_available) {
      out.push(`No review-engine verdicts for ${r.repo}: its governed PRs show as Unknown, and its reviews owed are missing.`);
    }
  }
  return out;
}

export function asOfText(data) {
  const commit = data.commit && /^[0-9a-f]{7,40}$/.test(data.commit) ? data.commit.slice(0, 7) : null;
  return { time: new Date(data.asOf).toISOString().slice(0, 16).replace("T", " "), commit };
}

/**
 * The whose-move groups for the PRs the filters leave (whose-move itself
 * ignored), counted as the live tiles count them.
 */
export function whoseMove(data, f) {
  const prs = filterPrs(data.prs, f, { ignoreOwner: true });
  return data.owners.map((o) => {
    const items = prs.filter((p) => p.owner === o.key);
    const parts = data.stages
      .filter((s) => (o.key === PARKED ? !s.inFlow : s.inFlow && s.owner === o.key))
      .map((s) => [s, items.filter((p) => p.stage === s.key).length])
      .filter(([, n]) => n);
    return {
      key: o.key,
      label: o.label,
      parked: o.key === PARKED,
      n: items.length,
      late: items.filter((p) => p.lateRank >= 2).length,
      veryLate: items.filter((p) => p.lateness === "very-late").length,
      parts,
    };
  });
}

// The live table's sort keys, so a sorted link means the same on every page.
export const PR_SORTS = {
  time: { label: "Late first", firstDir: "desc", key: (p) => [Number(p.inFlow), p.lateRank, p.ageMs ?? -1] },
  repo: { label: "Repo", firstDir: "asc", key: (p) => [p.repo, p.number ?? 0] },
  author: { label: "Author", firstDir: "asc", key: (p) => [p.author ?? ""] },
  stage: { label: "Stage", firstDir: "asc", key: (p, data) => [data.stage[p.stage].order, -(p.ageMs ?? -1)] },
  flags: { label: "Flags", firstDir: "desc", key: (p) => [flagCount(p), p.unresolved_comments] },
};

export function sortPrs(data, prs, sort, dir) {
  const s = PR_SORTS[sort] || PR_SORTS.time;
  const sign = (dir || s.firstDir) === "desc" ? -1 : 1;
  return [...prs].sort((a, b) => sign * compare(s.key(a, data), s.key(b, data)));
}

export function compare(a, b) {
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

/**
 * One person's numbers, as the live People table computes them (a copy of
 * people.js's summarize, which that module does not export).
 */
export function summarize(data, person, { repo = "", milestone = "" } = {}) {
  const inRepo = (r) => !repo || r === repo;
  const owes = person.owesList.filter((o) => inRepo(o.prData ? o.prData.repo : o.pr.split("#")[0]) &&
    (!milestone || (o.prData && matchesMilestone(o.prData, milestone))));
  const oldest = owes
    .map((o) => o.prData)
    .filter((p) => p && p.ageMs !== null)
    .reduce((a, p) => (!a || p.ageMs > a.ageMs ? p : a), null);
  const prs = person.authoredPrs.filter((p) => inRepo(p.repo) && matchesMilestone(p, milestone));
  const wip = person.wip.filter(([r]) => inRepo(r)).map(([r, n]) => [r, n, data.slotLimit(r)]);
  return {
    person,
    owes,
    oldest,
    oldestMs: oldest ? oldest.ageMs : null,
    oldestOpened: oldest ? oldest.since_basis !== "engine" : false,
    prs,
    wip,
    wipMax: wip.reduce((m, [, n]) => Math.max(m, n), 0),
    unresolved: prs.reduce((s, p) => s + (Number(p.unresolved_comments) || 0), 0),
  };
}

// The live People table's sort keys.
export const PEOPLE_SORTS = {
  name: { label: "Name", firstDir: "asc", key: (r) => [String(r.person.login)] },
  owed: { label: "Reviews owed", firstDir: "desc", key: (r) => [r.owes.length, r.oldestMs ?? -1] },
  prs: { label: "Own PRs", firstDir: "desc", key: (r) => [r.prs.length] },
  wip: { label: "WIP", firstDir: "desc", key: (r) => [r.wipMax] },
  unresolved: { label: "Unresolved", firstDir: "desc", key: (r) => [r.unresolved] },
};

export function sortPeople(rows, sort, dir) {
  const s = PEOPLE_SORTS[sort] || PEOPLE_SORTS.name;
  const sign = (dir || s.firstDir) === "desc" ? -1 : 1;
  return [...rows].sort((a, b) => sign * compare(s.key(a), s.key(b)));
}

/**
 * Someone's queue split by whose move it is: the reviews they owe (oldest
 * first, as on their live page), their own PRs whose stage the author owns,
 * the ones waiting on someone else, and parked ones.
 */
export function queueOf(data, person, milestone = "") {
  const s = summarize(data, person, { milestone });
  const byAge = (a, b) => (b.ageMs ?? -1) - (a.ageMs ?? -1);
  const owes = [...s.owes].sort((a, b) => (b.prData?.ageMs ?? -1) - (a.prData?.ageMs ?? -1));
  return {
    ...s,
    owes,
    yourMove: s.prs.filter((p) => p.owner === "author").sort(byAge),
    waiting: s.prs.filter((p) => p.owner !== "author" && p.owner !== PARKED).sort(byAge),
    parked: s.prs.filter((p) => p.owner === PARKED).sort(byAge),
  };
}

const ME_KEY = "pr-hygiene:dashboard:me";

/** Who "me" is in a prototype: `?user=` first, else the login the live page keeps in this browser. */
export function meLogin(data) {
  const q = new URLSearchParams(location.search).get("user");
  if (q && findPerson(data, q)) return findPerson(data, q).login;
  try {
    const v = localStorage.getItem(ME_KEY);
    if (v && LOGIN_RE.test(v) && findPerson(data, v)) return findPerson(data, v).login;
  } catch {
    // Storage blocked: nobody is "me".
  }
  return null;
}

export function rememberMe(login) {
  try {
    localStorage.setItem(ME_KEY, login);
  } catch {
    // Private mode: the choice lasts this page only.
  }
}

export function forgetMe() {
  try {
    localStorage.removeItem(ME_KEY);
  } catch {
    // Nothing stored.
  }
}

/** Every jump target for the command palette: views, people, repositories and PRs. */
export function paletteItems(data, { go, openPr, setRepo, me }) {
  const items = [
    { kind: "view", label: "Team", hint: "whose move, every open PR", run: () => go("#/") },
    { kind: "view", label: "People", hint: "who owes what", run: () => go("#/people") },
    { kind: "view", label: "Me", hint: me ? `as ${me}` : "pick your login", run: () => go("#/me") },
    { kind: "view", label: "Late PRs", hint: "Team, late only", run: () => go("#/?late=1") },
  ];
  for (const p of [...data.people].sort((a, b) => a.isBot - b.isBot || a.login.localeCompare(b.login))) {
    if (!findPerson(data, p.login)) continue;
    items.push({ kind: "person", label: p.login, hint: p.isBot ? "bot" : "person", run: () => go(`#/people/${encodeURIComponent(p.login)}`) });
  }
  for (const r of data.repoSet) items.push({ kind: "repo", label: shortRepo(r), hint: "filter Team to this repository", run: () => setRepo(r) });
  for (const p of data.prs) {
    if (!p.url) continue;
    items.push({ kind: "pr", label: `${shortRepo(p.repo)}#${p.number}`, hint: p.title, run: () => openPr(p.url) });
  }
  return items;
}

/**
 * Items whose words contain every word typed: those matched on their label
 * first ("platform 49" finds platform#49xx before titles with a 49), then
 * the rest, each in list order (views, people, repos, PRs).
 */
export function matchPalette(items, q, limit = 40) {
  const words = q.trim().toLowerCase().split(/\s+/).filter(Boolean);
  const onLabel = [];
  const elsewhere = [];
  for (const it of items) {
    const label = `${it.label} ${it.kind}`.toLowerCase();
    const hay = `${label} ${it.hint}`.toLowerCase();
    if (words.every((w) => label.includes(w))) onLabel.push(it);
    else if (words.every((w) => hay.includes(w))) elsewhere.push(it);
  }
  return [...onLabel, ...elsewhere].slice(0, limit);
}

/** "a", "b": a short visually-hidden text for assistive technology. */
export function sr(text) {
  return h("span", { class: "sr" }, text);
}
