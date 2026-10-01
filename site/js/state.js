// The URL hash is the page's whole state, so every view and filter is a link
// someone can share: "#/people/alice", "#/?repo=dashpay/platform&late=1".

import { LOGIN_RE } from "./model.js";

const WHO = new Set(["human", "all", "bot"]);
const DIRS = new Set(["asc", "desc"]);
const FILTER_KEYS = ["repo", "stage", "owner", "who", "late", "q"];

export function parseHash(hash, data) {
  const raw = hash.replace(/^#/, "");
  const cut = raw.indexOf("?");
  const path = cut < 0 ? raw : raw.slice(0, cut);
  const query = cut < 0 ? "" : raw.slice(cut + 1);
  const parts = path.split("/").filter(Boolean);
  let route = { view: "team" };
  if (parts[0] === "people" && parts.length === 1) route = { view: "people" };
  else if (parts[0] === "people" && parts.length === 2) {
    let login = null;
    try {
      login = decodeURIComponent(parts[1]);
    } catch {
      login = null;
    }
    route = { view: "person", login: login !== null && LOGIN_RE.test(login) ? login : null };
  } else if (parts[0] === "me") route = { view: "me" };

  const q = new URLSearchParams(query);
  const get = (k) => q.get(k) ?? "";
  const f = {
    repo: data.repoSet.has(get("repo")) ? get("repo") : "",
    stage: Object.hasOwn(data.stage, get("stage")) ? get("stage") : "",
    owner: data.owners.some((o) => o.key === get("owner")) ? get("owner") : "",
    who: WHO.has(get("who")) ? get("who") : "",
    late: get("late") === "1",
    q: get("q").slice(0, 100),
    sort: /^[a-z-]{1,20}$/.test(get("sort")) ? get("sort") : "",
    dir: DIRS.has(get("dir")) ? get("dir") : "",
  };
  return { route, filters: f };
}

/** The hash for a view with these filters; empty values are left out. */
export function buildHash(path, filters, { keepSort = true } = {}) {
  const q = new URLSearchParams();
  for (const k of FILTER_KEYS) {
    const v = filters[k];
    if (v === true) q.set(k, "1");
    else if (v) q.set(k, v);
  }
  if (keepSort && filters.sort) {
    q.set("sort", filters.sort);
    if (filters.dir) q.set("dir", filters.dir);
  }
  const s = q.toString();
  return `#${path}${s ? `?${s}` : ""}`;
}

/** Replace some filters and navigate; history keeps one entry per change. */
export function setFilters(path, filters, change, opts) {
  const next = { ...filters, ...change };
  location.hash = buildHash(path, next, opts);
}
