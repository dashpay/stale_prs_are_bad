// PR Hygiene dashboard: loads dashboard.json from this directory and renders
// the view the URL hash names. No build step; d3 and Plot are vendored globals.

import { h } from "./js/dom.js";
import { prepare, STALE_HOURS, formatDuration } from "./js/model.js";
import { parseHash, buildHash } from "./js/state.js";
import { filterBar } from "./js/filters.js";
import { teamView } from "./js/team.js";
import { peopleView, personView, meView } from "./js/people.js";

const root = document.getElementById("app");
let data = null;
let pickedMe = null;

async function load() {
  let raw;
  try {
    const res = await fetch("dashboard.json", { cache: "no-cache" });
    if (!res.ok) throw new Error(`dashboard.json could not be loaded (HTTP ${res.status}).`);
    raw = await res.json();
  } catch (e) {
    fail(e instanceof SyntaxError ? "dashboard.json is not valid JSON." : e.message);
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
    render();
  });
  // Plot is drawn with resolved colours, so a scheme change redraws the page.
  matchMedia("(prefers-color-scheme: dark)").addEventListener("change", render);
  // A tab left open for hours learns that its data went stale when it is next looked at.
  document.addEventListener("visibilitychange", () => {
    if (!document.hidden && isStale() !== shownStale) render();
  });
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
    const href = view === "me" ? "#/me" : buildHash(path, { ...f, q: "" }, { keepSort: false });
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
    if (!r.engine_state_available) {
      out.push(h("div", { class: "banner banner-warn", role: "status" },
        `⚠ No review-engine verdicts for ${r.repo}: its governed PRs show as Unknown, and its reviews owed are missing.`));
    }
  }
  return out;
}

function header(route, f) {
  const asOf = new Date(data.asOf);
  const commit = data.commit && /^[0-9a-f]{7,40}$/.test(data.commit) ? data.commit.slice(0, 7) : null;
  return h("header", { class: "top" },
    h("div", { class: "brand" },
      h("h1", {}, "PR Hygiene"),
      h("p", { class: "sub" }, "Where are we late, and whose move is it? ",
        h("span", { class: "asof" }, `Data as of ${asOf.toISOString().slice(0, 16).replace("T", " ")} UTC`,
          commit ? ` · ${commit}` : ""))),
    nav(route, f));
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
  const caret = active && typeof active.selectionStart === "number" ? [active.selectionStart, active.selectionEnd] : null;

  let bar = null;
  let content;
  if (route.view === "team") {
    bar = filterBar(data, "/", f, {
      show: ["repo", "stage", "who", "late", "q", "owner"], whoDefault: "human", onText: render, searchHint: "author, reviewer or title",
    });
    content = teamView(data, f);
  } else if (route.view === "people") {
    bar = filterBar(data, "/people", f, { show: ["repo", "who", "q"], whoDefault: "all", onText: render, searchHint: "login" });
    content = peopleView(data, f);
  } else if (route.view === "person") {
    content = personView(data, route.login);
  } else {
    content = meView(data, (p) => {
      pickedMe = p;
      render();
    }, pickedMe);
  }
  document.title = route.view === "person" && route.login ? `${route.login} · PR Hygiene` : "PR Hygiene";
  root.replaceChildren(...[header(route, f), ...banners(), bar, h("main", { id: "main", tabindex: "-1" }, content)].filter(Boolean));

  const el = focusId ? document.getElementById(focusId) : null;
  if (el) {
    el.focus();
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
