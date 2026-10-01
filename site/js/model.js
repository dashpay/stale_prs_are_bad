// What the data means: stage and owner names, links, durations, and the
// indexes the views share. The producer decides stages, owners, lateness,
// idleness and limits; this file only names them and joins records.

export const SCHEMA_VERSION = 1;
export const STALE_HOURS = 12;
export const DAY_MS = 86_400_000;
// The review engine's own default, used only for a repository the data gives no limit for.
const DEFAULT_SLOT_LIMIT = 5;

// The page's wording for each stage the producer may send. Order, owner,
// thresholds and whether it is in the review flow come from the data.
const STAGE_WORDS = {
  draft: ["Draft"],
  bots: ["Bots"],
  "self-review": ["Self-review"],
  ci: ["CI running"],
  queued: ["Queued", "over the repository's review-slot limit; waits for the author's other PRs"],
  review: ["Review"],
  mergeable: ["Mergeable"],
  blocked: ["Blocked", "configuration error"],
  "not-governed": ["Not governed", "branch outside the policy"],
  unknown: ["Unknown", "no engine verdict"],
};

// The page's names for the owners the producer sends, in tile order. An owner
// it does not know yet still gets a tile, named by its key. Work outside the
// review flow is "parked", whoever owns its stage.
const OWNER_WORDS = {
  author: "Author",
  reviewers: "Reviewers",
  bots: "Bots",
  maintainers: "Maintainers",
  nobody: "Nobody in particular",
  parked: "Parked",
};
export const PARKED = "parked";
// Stages whose owner is a person other than the author: a bot's PR there
// still waits on people.
const PEOPLE_MOVES = new Set(["reviewers", "maintainers"]);
const OWNER_KEY_RE = /^[a-z][a-z-]{0,30}$/;

export function ownerLabel(key) {
  return has(OWNER_WORDS, key) ? OWNER_WORDS[key] : String(key);
}

export const LATENESS = {
  "very-late": { label: "very late", rank: 3 },
  late: { label: "late", rank: 2 },
  ok: { label: "on time", rank: 1 },
};

export const LOGIN_RE = /^[A-Za-z0-9-]{1,39}(\[bot\])?$/;
const AVATAR_LOGIN_RE = /^[A-Za-z0-9-]{1,39}$/;
const REPO_RE = /^[A-Za-z0-9-]{1,39}\/[A-Za-z0-9_.-]{1,100}$/;
const STAGE_KEY_RE = /^[a-z][a-z-]{0,30}$/;

/** The area the policy gives no owner is called "fallback" in the data. */
export function areaLabel(area) {
  return area === "fallback" ? "files with no dedicated owner" : String(area);
}

/** The PR's GitHub page, built only from a known repository and a positive integer. */
export function prUrl(data, repo, number) {
  if (!data.repoSet.has(repo) || !REPO_RE.test(repo)) return null;
  if (!Number.isSafeInteger(number) || number < 1) return null;
  return `https://github.com/${repo}/pull/${number}`;
}

export function avatarUrl(login) {
  return typeof login === "string" && AVATAR_LOGIN_RE.test(login)
    ? `https://avatars.githubusercontent.com/${login}?s=40`
    : null;
}

/** The internal person page, for logins that look like GitHub logins. */
export function personHref(login) {
  return typeof login === "string" && LOGIN_RE.test(login)
    ? `#/people/${encodeURIComponent(login)}`
    : null;
}

export function shortRepo(repo) {
  return String(repo).replace(/^dashpay\//, "");
}

const list = (x) => (Array.isArray(x) ? x : []);
// Own keys only, so a value like "constructor" from the data is never a match.
export const has = (obj, key) => typeof key === "string" && Object.hasOwn(obj, key);
const dict = (x) => (x && typeof x === "object" && !Array.isArray(x) ? x : {});

function time(s) {
  if (typeof s !== "string") return null;
  const t = Date.parse(s);
  return Number.isFinite(t) ? t : null;
}

/** "45m", "7h", "12d": compact durations for table cells and tooltips. */
export function formatDuration(ms) {
  if (ms === null || ms === undefined || !Number.isFinite(ms)) return "—";
  // Rounded down, so "24h" never appears for a PR that is not yet a day late.
  const m = Math.max(0, ms) / 60_000;
  if (m < 60) return `${Math.floor(m)}m`;
  if (m < 60 * 24) return `${Math.floor(m / 60)}h`;
  return `${Math.floor(m / 1440)}d`;
}

export function formatHours(h) {
  return h < 24 ? `${h}h` : `${+(h / 24).toFixed(1)}d`;
}

/** The stage list from the data, in its display order, with the page's names. */
function readStages(rawStages) {
  const stages = [];
  for (const s of list(rawStages)) {
    const key = s && typeof s.stage === "string" && STAGE_KEY_RE.test(s.stage) ? s.stage : null;
    if (!key || stages.some((x) => x.key === key)) continue;
    const [label, note] = has(STAGE_WORDS, key) ? STAGE_WORDS[key] : [key];
    const late = Array.isArray(s.late_hours) && s.late_hours.length === 2 && s.late_hours.every(Number.isFinite)
      ? s.late_hours : null;
    stages.push({
      key, label, note: note || null,
      owner: typeof s.owner === "string" && OWNER_KEY_RE.test(s.owner) && s.owner !== PARKED ? s.owner : "nobody",
      lateHours: late,
      entryRecorded: s.entry_recorded === true,
      inFlow: s.in_flow !== false,
      order: stages.length,
    });
  }
  if (!stages.some((s) => s.key === "unknown")) {
    stages.push({
      key: "unknown", label: "Unknown", note: STAGE_WORDS.unknown[1], owner: "nobody",
      lateHours: null, entryRecorded: false, inFlow: false, order: stages.length,
    });
  }
  return stages;
}

/**
 * Check the shape the views rely on and index it. Throws a readable message
 * for anything this page cannot render faithfully.
 */
export function prepare(raw) {
  if (!raw || typeof raw !== "object") throw new Error("dashboard.json is not a JSON object.");
  if (raw.schema_version !== SCHEMA_VERSION) {
    throw new Error(
      `This page reads data version ${SCHEMA_VERSION}, but dashboard.json is version ` +
        `${JSON.stringify(raw.schema_version)}. It needs a matching page; nothing is shown rather than something wrong.`,
    );
  }
  for (const k of ["repos", "stages", "prs", "people"]) {
    if (!Array.isArray(raw[k])) throw new Error(`dashboard.json has no "${k}" list.`);
  }
  // Durations are "as of" the snapshot, the same instant the producer judged
  // lateness at, so a stale page never shows a time that disagrees with its colour.
  const asOf = time(raw.generated_at);
  if (asOf === null) throw new Error("dashboard.json has no valid generated_at.");

  // Every record is rebuilt field by field with the type the views expect, so
  // a value of the wrong type renders as empty instead of stopping the page.
  const repos = raw.repos.filter(isObj).map((r) => ({
    repo: str(r.repo),
    fetch_error: optStr(r.fetch_error),
    stage_times_error: optStr(r.stage_times_error),
    engine_state_available: r.engine_state_available !== false,
    slotLimit: Number.isSafeInteger(r.slot_limit) && r.slot_limit > 0 ? r.slot_limit : DEFAULT_SLOT_LIMIT,
  }));
  const repoSet = new Set(repos.map((r) => r.repo).filter(Boolean));
  const slotLimits = new Map(repos.map((r) => [r.repo, r.slotLimit]));
  const stages = readStages(raw.stages);
  const known = Object.keys(OWNER_WORDS);
  // Whose-move groups: owners of stages in the flow, then parked work.
  const owners = [...new Set(stages.filter((s) => s.inFlow).map((s) => s.owner))]
    .sort((a, b) => (known.indexOf(a) + 1 || 99) - (known.indexOf(b) + 1 || 99))
    .map((key) => ({ key, label: ownerLabel(key) }));
  if (stages.some((s) => !s.inFlow)) owners.push({ key: PARKED, label: ownerLabel(PARKED) });
  const data = {
    asOf,
    commit: optStr(raw.commit),
    idleDays: Number.isSafeInteger(raw.idle_days) && raw.idle_days > 0 ? raw.idle_days : null,
    repoSet, repos, stages, owners,
    stage: Object.fromEntries(stages.map((s) => [s.key, s])),
    slotLimit: (repo) => slotLimits.get(repo) ?? DEFAULT_SLOT_LIMIT,
  };

  data.prs = raw.prs.filter(isObj).map((p) => {
    const since = time(p.since);
    const updated = time(p.updated_at);
    const stage = has(data.stage, p.stage) ? p.stage : "unknown";
    const info = data.stage[stage];
    const sinceBasis = p.since_basis === "engine" || p.since_basis === "opened" ? p.since_basis : null;
    // Only a recorded stage start can make a PR late; an age never does.
    const lateness = sinceBasis === "engine" && has(LATENESS, p.lateness) ? p.lateness : null;
    const asks = list(p.asks).filter(isObj).map((a) => ({ area: str(a.area), approvers: strs(a.approvers) }));
    const objectors = strs(p.objectors);
    const repo = str(p.repo);
    const number = Number.isSafeInteger(p.number) ? p.number : null;
    const isBot = p.author_kind === "bot";
    return {
      key: str(p.key),
      repo,
      number,
      title: str(p.title),
      author: optStr(p.author),
      isBot,
      stage,
      inFlow: info.inFlow,
      owner: info.inFlow ? info.owner : PARKED,
      // A bot's PR still waits on people when reviewers or maintainers must act.
      needsPeople: !isBot || (info.inFlow && PEOPLE_MOVES.has(info.owner)),
      next_action: optStr(p.next_action),
      blockers: strs(p.blockers),
      since_basis: sinceBasis,
      ageMs: since === null ? null : asOf - since,
      updatedAgoMs: updated === null ? null : asOf - updated,
      lateness,
      lateRank: lateness ? LATENESS[lateness].rank : 0,
      asks,
      objectors,
      areas: strs(p.areas),
      reviewers: [...new Set([...asks.flatMap((a) => a.approvers), ...objectors])],
      unresolved_comments: count(p.unresolved_comments),
      ci_failing: p.ci_failing === true,
      merge_conflict: p.merge_conflict === true,
      changes_requested: p.changes_requested === true,
      tracked: p.tracked !== false,
      idle: p.idle === true,
      url: prUrl(data, repo, number),
    };
  });
  data.prByKey = new Map(data.prs.map((p) => [p.key, p]));
  data.areas = [...new Set(data.prs.flatMap((p) => p.areas))]
    .sort((a, b) => (a === "fallback") - (b === "fallback") || a.localeCompare(b));

  data.people = raw.people.filter(isObj).map((person) => {
    const owes = list(person.owes).filter(isObj).map((o) => ({
      pr: str(o.pr),
      areas: list(o.areas).filter(isObj).map((a) => ({ area: str(a.area), others: strs(a.others) })),
      rereview: o.rereview === true,
      prData: data.prByKey.get(str(o.pr)) || null,
    }));
    return {
      login: str(person.login),
      isBot: person.kind === "bot",
      roles: strs(person.roles),
      owesList: owes,
      authoredPrs: strs(person.authored).map((k) => data.prByKey.get(k)).filter(Boolean),
      wip: Object.entries(dict(person.wip)).map(([r, n]) => [r, count(n)]),
      areaList: Object.entries(dict(person.areas)).map(([r, ids]) => [r, strs(ids)]),
    };
  });
  data.personByLogin = new Map(data.people.map((p) => [p.login.toLowerCase(), p]));
  return data;
}

function isObj(x) {
  return x !== null && typeof x === "object" && !Array.isArray(x);
}

function str(x) {
  return typeof x === "string" ? x : "";
}

function optStr(x) {
  return typeof x === "string" ? x : null;
}

function strs(x) {
  return list(x).filter((s) => typeof s === "string");
}

/** A small non-negative integer; anything else, or anything absurd, is capped. */
function count(x) {
  return Number.isFinite(x) && x > 0 ? Math.min(Math.floor(x), 999) : 0;
}

/** A person from a login typed in a URL or kept in storage, or null. */
export function findPerson(data, login) {
  if (typeof login !== "string" || !LOGIN_RE.test(login)) return null;
  return data.personByLogin.get(login.toLowerCase()) || null;
}

/** Idle drafts and idle PRs off the governed branches, and PRs with a merge conflict. */
export function closeOrRevive(prs) {
  return prs
    .map((p) => ({ pr: p, idle: p.idle && (p.stage === "draft" || p.stage === "not-governed"), conflict: p.merge_conflict }))
    .filter((r) => r.idle || r.conflict)
    .sort((a, b) => (b.pr.updatedAgoMs ?? -1) - (a.pr.updatedAgoMs ?? -1));
}

/** "idle for 14+ days" when the data says how long idle is, else "idle". */
export function idleWords(data) {
  return data.idleDays ? `idle ${data.idleDays}+ days` : "idle";
}
