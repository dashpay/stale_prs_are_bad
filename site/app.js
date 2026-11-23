// PR Hygiene dashboard: loads dashboard.json from this directory and renders
// the view the URL hash names. No build step; d3 and Plot are vendored globals.

import { h, openPr } from "./js/dom.js";
import { prepare, STALE_HOURS, findPerson, formatDuration, shortRepo, LOGIN_RE } from "./js/model.js";
import { parseHash, buildHash, setFilters } from "./js/state.js";
import { filterBar } from "./js/filters.js";
import { teamView } from "./js/team.js";
import { peopleView, personView, meView } from "./js/people.js";
import { meRoute, navigated, knownMe } from "./js/account.js";
import { privacyView } from "./js/privacy.js";
import { installKeys, openPalette, openHelp, focusTarget, JUMP_KEY } from "./js/keys.js";
import { toggleDetail } from "./js/rows.js";

const root = document.getElementById("app");
let data = null;
let pickedMe = null;
// Whether the phone's folded filters are open; kept across re-renders.
let filtersOpen = false;
// A PR picked in the jump palette: its row opens once Team shows it.
let pendingPr = null;

// The theme: the system's, or one picked here and kept in this browser.
const THEME_KEY = "pr-hygiene:dashboard:theme";
const THEMES = ["", "dark", "light"];
let theme = "";
try {
  const t = localStorage.getItem(THEME_KEY);
  theme = THEMES.includes(t) ? t : "";
} catch {
  theme = "";
}

function applyTheme() {
  if (theme) document.documentElement.dataset.theme = theme;
  else delete document.documentElement.dataset.theme;
}
applyTheme();

function cycleTheme() {
  theme = THEMES[(THEMES.indexOf(theme) + 1) % THEMES.length];
  try {
    if (theme) localStorage.setItem(THEME_KEY, theme);
    else localStorage.removeItem(THEME_KEY);
  } catch {
    // Storage blocked: the choice lasts until the page reloads.
  }
  applyTheme();
  // Plot draws with resolved colours, so the charts are drawn again.
  render();
}

async function load() {
  let raw;
  try {
    const res = await fetch("dashboard.json", { cache: "no-cache" });
    if (!res.ok) throw new Error(`HTTP ${res.status}`);
    raw = await res.json();
  } catch (e) {
    fail(e instanceof SyntaxError
      ? "dashboard.json is not valid JSON."
      : `dashboard.json could not be loaded (${e.message}). The data is published beside this page; try again shortly.`);
    return;
  }
  try {
    data = prepare(raw);
  } catch (e) {
    fail(e.message);
    return;
  }
  render();
  addEventListener("hashchange", () => {
    pickedMe = null;
    navigated();
    render();
  });
  // Plot is drawn with resolved colours, so a scheme change redraws the page.
  matchMedia("(prefers-color-scheme: dark)").addEventListener("change", render);
  // A tab left open for hours learns that its data went stale when it is next looked at.
  document.addEventListener("visibilitychange", () => {
    if (!document.hidden && isStale() !== shownStale) render();
  });
  installKeys(keyApi);
}

const keyApi = {
  go: (hash) => {
    location.hash = hash;
  },
  rows: () => [...root.querySelectorAll("main [data-row]")].filter((el) => el.offsetParent !== null),
  openOnGitHub: (row) => {
    const pr = data.prByKey.get(row.dataset.key);
    if (pr && pr.url) openPr(pr.url);
  },
  toggleLate: () => {
    const { route, filters: f } = parseHash(location.hash, data);
    if (route.view === "team") setFilters("/", f, { late: !f.late });
  },
  items: () => paletteItems(),
};

/** Everything the jump palette can reach: views, people, repositories and PRs. */
function paletteItems() {
  const items = [
    { kind: "view", label: "Team", hint: "whose move, and every open PR", run: () => keyApi.go("#/") },
    { kind: "view", label: "Late PRs", hint: "Team, late only", run: () => keyApi.go(buildHash("/", { late: true })) },
    { kind: "view", label: "People", hint: "what everyone owes and holds", run: () => keyApi.go("#/people") },
    { kind: "view", label: "Me", hint: "your reviews and PRs", run: () => keyApi.go("#/me") },
    { kind: "view", label: "Privacy", hint: "what is kept, and for how long", run: () => keyApi.go("#/privacy") },
  ];
  const people = data.people
    .filter((p) => LOGIN_RE.test(p.login) && findPerson(data, p.login) === p)
    .sort((a, b) => a.isBot - b.isBot || a.login.localeCompare(b.login));
  for (const p of people) {
    items.push({ kind: "person", label: p.login, hint: p.isBot ? "bot" : "person", run: () => keyApi.go(`#/people/${encodeURIComponent(p.login)}`) });
  }
  for (const r of data.repoSet) {
    items.push({ kind: "repo", label: shortRepo(r), hint: "Team, this repository only", run: () => keyApi.go(buildHash("/", { repo: r })) });
  }
  for (const p of data.prs) {
    if (!p.url) continue;
    items.push({
      kind: "pr", label: `${shortRepo(p.repo)}#${p.number}`, hint: p.title,
      run: () => {
        // Show it on the board, everyone's PRs included, with its details open.
        pendingPr = p.key;
        const hash = buildHash("/", { q: p.key, who: "all" });
        if (location.hash === hash) render();
        else location.hash = hash;
      },
    });
  }
  return items;
}

function fail(message) {
  root.replaceChildren(h("div", { class: "banner banner-error", role: "alert" }, message));
}

function nav(route, f) {
  // Moving between views keeps the filters that mean the same on both and
  // drops the view's sort and search: the search matches titles on one view
  // and logins on the other.
  const links = [
    ["team", "/", "Team"],
    ["people", "/people", "People"],
    ["me", "/me", "Me"],
  ];
  return h("nav", { "aria-label": "Views" }, links.map(([view, path, label]) => {
    const current = route.view === view || (view === "people" && route.view === "person");
    const href = view === "me" ? buildHash(path, { milestone: f.milestone }) : buildHash(path, { ...f, q: "" }, { keepSort: false });
    return h("a", { href, "aria-current": current ? "page" : null }, label);
  }));
}

let shownStale = false;

function isStale() {
  return Date.now() - data.asOf > STALE_HOURS * 3_600_000;
}

function banners() {
  const out = [];
  shownStale = isStale();
  if (shownStale) {
    out.push(h("div", { class: "banner banner-warn", role: "status" },
      `⚠ This data is ${formatDuration(Date.now() - data.asOf)} old; the update job may have stopped. Times below are as of the snapshot.`));
  }
  for (const r of data.repos) {
    if (r.fetch_error) {
      out.push(h("div", { class: "banner banner-warn", role: "status" },
        `⚠ ${r.repo} could not be fetched: `, h("span", { class: "err" }, r.fetch_error), ". Its PRs may be missing."));
    }
    if (r.stage_times_error) {
      out.push(h("div", { class: "banner banner-warn", role: "status" },
        `⚠ For some ${r.repo} PRs, the time they entered their stage could not be read (`,
        h("span", { class: "err" }, r.stage_times_error),
        "). Those show their age instead and are not marked late, except a review, which keeps the review engine's own start."));
    }
    if (!r.engine_state_available) {
      out.push(h("div", { class: "banner banner-warn", role: "status" },
        `⚠ No review-engine verdicts for ${r.repo}: its governed PRs show as Unknown, and its reviews owed are missing.`));
    }
  }
  return out;
}

function header(route, f) {
  const at = new Date(data.asOf).toISOString();
  const commit = data.commit && /^[0-9a-f]{7,40}$/.test(data.commit) ? data.commit.slice(0, 7) : null;
  const themeName = theme || "system";
  return h("header", { class: "top" },
    h("div", { class: "brand" },
      h("span", { class: "logo", "aria-hidden": "true" }),
      h("h1", {}, "PR Hygiene"),
      h("p", { class: "tagline" }, "Where are we late, and whose move is it?")),
    nav(route, f),
    h("div", { class: "top-tools" },
      h("button", {
        type: "button", class: "jump", id: "jump", "aria-label": `Jump to (${JUMP_KEY})`, "aria-keyshortcuts": "Meta+K Control+K",
        on: { click: () => openPalette(keyApi) },
      }, h("span", { class: "jump-text", "aria-hidden": "true" }, "Jump to…"), h("kbd", { "aria-hidden": "true" }, JUMP_KEY)),
      h("p", { class: "asof", title: `Data as of ${at.slice(0, 16).replace("T", " ")} UTC${commit ? `, analyzer commit ${commit}` : ""}` },
        h("span", { class: "asof-label" }, "Data as of "),
        h("span", { class: "asof-date" }, `${at.slice(0, 10)} `),
        h("span", { class: "num" }, `${at.slice(11, 16)} UTC`),
        commit ? h("span", { class: "asof-commit" }, ` · ${commit}`) : null),
      h("button", {
        type: "button", class: "icon-button", id: "theme", title: `Theme: ${themeName}`,
        "aria-label": `Theme: ${themeName}. Select to change.`, on: { click: cycleTheme },
      }, h("span", { "aria-hidden": "true" }, { system: "◐", dark: "☾", light: "☀" }[themeName])),
      h("button", {
        type: "button", class: "icon-button", id: "help", title: "Keyboard shortcuts (?)",
        "aria-label": "Keyboard shortcuts", "aria-keyshortcuts": "Shift+?", on: { click: openHelp },
      }, h("span", { "aria-hidden": "true" }, "?"))));
}

/**
 * The view's filter row. On a phone only the search and "Late only" show
 * until "Filters" opens the rest; the button counts the filters in use.
 */
function filters(data, path, f, opts) {
  const form = filterBar(data, path, f, { ...opts, onText: render });
  if (!form) return null;
  const used = ["repo", "milestone", "area", "stage", "who"].filter((k) => opts.show.includes(k) && f[k]).length;
  const button = h("button", {
    type: "button", class: "filters-toggle", id: "filters-toggle", "aria-expanded": String(filtersOpen), "aria-controls": "filters",
  }, "Filters", used ? h("span", { class: "count" }, String(used)) : null);
  form.id = "filters";
  const wrap = h("div", { class: `filter-row${filtersOpen ? " open" : ""}` }, button, form);
  button.addEventListener("click", () => {
    filtersOpen = !filtersOpen;
    wrap.classList.toggle("open", filtersOpen);
    button.setAttribute("aria-expanded", String(filtersOpen));
  });
  return wrap;
}

function render() {
  try {
    draw();
  } catch (e) {
    fail(`This page could not show the data: ${e.message}`);
  }
}

function draw() {
  const { route, filters: f } = parseHash(location.hash, data);
  // Keep focus, and the caret, where they were when a control re-renders the page.
  const active = document.activeElement;
  const hadFocus = active && active !== document.body && root.contains(active);
  const focusId = active && active.id ? active.id : null;
  // A PR can be listed twice (Open PRs, Close or revive): remember which list.
  const activeRow = active?.closest?.("[data-row]") || null;
  const rowKey = activeRow ? activeRow.dataset.key : null;
  const rowList = activeRow ? activeRow.closest(".rows")?.getAttribute("aria-label") : null;
  const caret = active && typeof active.selectionStart === "number" ? [active.selectionStart, active.selectionEnd] : null;

  let bar = null;
  let content;
  if (route.view === "team") {
    bar = filters(data, "/", f, {
      show: ["repo", "milestone", "area", "stage", "who", "late", "q", "owner"], whoDefault: "people",
      searchHint: "author, reviewer or title",
    });
    content = teamView(data, f, knownMe(data, render));
  } else if (route.view === "people") {
    bar = filters(data, "/people", f, { show: ["repo", "milestone", "who", "q"], whoDefault: "all", searchHint: "login" });
    content = peopleView(data, f);
  } else if (route.view === "person") {
    const person = findPerson(data, route.login);
    if (person) bar = filters(data, `/people/${encodeURIComponent(person.login)}`, f, { show: ["milestone"] });
    content = personView(data, route.login, { milestone: f.milestone });
  } else if (route.view === "privacy") {
    content = privacyView();
  } else {
    // Where there is no sign-in, Me is the login picker kept in this browser.
    bar = filters(data, "/me", f, { show: ["milestone"] });
    content = meRoute(data, route, f.milestone, render, () => meView(data, (p) => {
      pickedMe = p;
      render();
    }, pickedMe, f.milestone));
  }
  document.title = route.view === "person" && route.login ? `${route.login} · PR Hygiene`
    : route.view === "privacy" ? "Privacy · PR Hygiene" : "PR Hygiene";
  const footer = h("footer", { class: "foot" },
    h("a", { href: "#/privacy", "aria-current": route.view === "privacy" ? "page" : null }, "Privacy"),
    h("span", {}, "Press ", h("kbd", {}, "?"), " for keyboard shortcuts, ", h("kbd", {}, JUMP_KEY), " to jump"));
  root.replaceChildren(...[
    header(route, f), ...banners(), bar,
    h("main", { id: "main", tabindex: "-1", class: `view-${route.view}` }, content),
    footer,
  ].filter(Boolean));

  let el = focusId ? document.getElementById(focusId) : null;
  if (!el && rowKey) {
    const lists = [...root.querySelectorAll("main .rows")];
    const list = lists.find((l) => l.getAttribute("aria-label") === rowList) || null;
    const rows = [...(list || root).querySelectorAll("[data-row]")];
    const row = rows.find((r) => r.dataset.key === rowKey);
    el = row ? focusTarget(row) : null;
  }
  let reveal = false;
  if (pendingPr) {
    const row = [...root.querySelectorAll("main [data-row]")].find((r) => r.dataset.key === pendingPr);
    pendingPr = null;
    if (row) {
      toggleDetail(data, row);
      el = focusTarget(row);
      reveal = true;
    }
  }
  if (el) {
    el.focus({ preventScroll: reveal });
    if (reveal) el.scrollIntoView({ block: "center" });
    if (caret && typeof el.setSelectionRange === "function") {
      try {
        el.setSelectionRange(caret[0], caret[1]);
      } catch {
        // Some input types have no caret.
      }
    }
  } else if (hadFocus) {
    // The control that was used is gone (a picked login, a cleared filter):
    // continue from the new content rather than the top of the document.
    const target = document.getElementById("person-name") || document.getElementById("main");
    target.focus({ preventScroll: true });
  }
}

load();
