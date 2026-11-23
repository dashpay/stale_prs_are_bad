// People: what everyone owes and holds, a page per person, and "me". Counts
// describe current state; lateness colour stays on PRs and never ranks people.

import { h } from "./dom.js";
import { LOGIN_RE, PARKED, has, findPerson, formatDuration, milestoneWords, personHref, shortRepo } from "./model.js";
import { effectiveWho, matchesMilestone } from "./filters.js";
import { personLink, avatar, botMark, section, areaName } from "./ui.js";
import { prRow, rowList, sortControl, sorted, sr } from "./rows.js";

const ME_KEY = "pr-hygiene:dashboard:me";

const ROLE_LABEL = { "author-bot": "opens PRs", "review-bot": "reviews PRs" };

/**
 * One person's numbers, limited to a repository and a milestone when the
 * filters name them. Review slots are per repository whatever the milestone,
 * so a milestone leaves them as they are.
 */
export function summarize(data, person, { repo = "", milestone = "" } = {}) {
  const inRepo = (r) => !repo || r === repo;
  const owes = person.owesList.filter((o) => inRepo(o.prData ? o.prData.repo : o.pr.split("#")[0]) &&
    (!milestone || (o.prData && matchesMilestone(o.prData, milestone))));
  // The longest wait among owed PRs, and whether that is a recorded wait or
  // only the PR's age.
  const oldest = owes
    .map((o) => o.prData)
    .filter((p) => p && p.ageMs !== null)
    .reduce((a, p) => (!a || p.ageMs > a.ageMs ? p : a), null);
  const prs = person.authoredPrs.filter((p) => inRepo(p.repo) && matchesMilestone(p, milestone));
  // [repo, open PRs, that repository's slot limit]
  const wip = person.wip.filter(([r]) => inRepo(r)).map(([r, n]) => [r, n, data.slotLimit(r)]);
  return {
    person,
    owes,
    oldestMs: oldest ? oldest.ageMs : null,
    oldestOpened: oldest ? oldest.since_basis !== "engine" : false,
    prs,
    wip,
    wipMax: wip.reduce((m, [, n]) => Math.max(m, n), 0),
    unresolved: prs.reduce((s, p) => s + (Number(p.unresolved_comments) || 0), 0),
  };
}

/**
 * Someone's queue split by whose move it is: the reviews they owe, oldest
 * first; their PRs whose stage the author owns; their PRs waiting on anyone
 * else; and their parked PRs. Within each, the longest wait first.
 */
export function queueOf(data, person, milestone = "") {
  const s = summarize(data, person, { milestone });
  const byAge = (a, b) => (b.ageMs ?? -1) - (a.ageMs ?? -1);
  return {
    ...s,
    owes: [...s.owes].sort((a, b) => (b.prData?.ageMs ?? -1) - (a.prData?.ageMs ?? -1)),
    yourMove: s.prs.filter((p) => p.owner === "author").sort(byAge),
    waiting: s.prs.filter((p) => p.owner !== "author" && p.owner !== PARKED).sort(byAge),
    parked: s.prs.filter((p) => p.owner === PARKED).sort(byAge),
  };
}


// The People table's columns as sort keys, with the same names in the URL.
const PEOPLE_SORTS = {
  name: { label: "Name", sort: (r) => [String(r.person.login)] },
  owed: { label: "Reviews owed", firstDir: "desc", sort: (r) => [r.owes.length, r.oldestMs ?? -1] },
  prs: { label: "Own PRs", firstDir: "desc", sort: (r) => [r.prs.length] },
  wip: { label: "WIP per repo", firstDir: "desc", sort: (r) => [r.wipMax] },
  unresolved: { label: "Unresolved comments", firstDir: "desc", sort: (r) => [r.unresolved] },
};

export function peopleView(data, f) {
  const who = effectiveWho(f, "all");
  const q = f.q.trim().toLowerCase();
  const rows = data.people
    .filter((p) => !q || String(p.login).toLowerCase().includes(q))
    .map((p) => summarize(data, p, f));
  const maxPrs = Math.max(1, ...rows.map((r) => r.prs.length));
  const out = [h("div", { class: "toolbar" }, sortControl("/people", f, PEOPLE_SORTS, "name"))];
  if (who !== "bot") {
    const humans = rows.filter((r) => !r.person.isBot);
    out.push(peopleList(data, "people-h", `People (${humans.length})`, humans, f, maxPrs, false));
  }
  if (who !== "human") {
    const bots = rows.filter((r) => r.person.isBot);
    out.push(peopleList(data, "bots-h", `Bots (${bots.length})`, bots, f, maxPrs, true));
  }
  out.push(h("p", { class: "sub" },
    "Reviews owed: PRs ready for review that wait on the person, with how long the oldest has waited. ",
    "WIP: open, non-draft PRs on governed branches per repository, against that repository's review-slot limit; ▲ marks one over it.",
    f.milestone ? " WIP counts every milestone." : ""));
  return out;
}

function peopleList(data, id, title, rows, f, maxPrs, bots) {
  const head = h("div", { class: "rows-head prow", "aria-hidden": "true" },
    h("span", {}, bots ? "Bot" : "Person"), h("span", {}, "Reviews owed"), h("span", {}, "Own PRs by stage"),
    h("span", {}, "WIP per repo"), h("span", { class: "n" }, "Unresolved"));
  return h("section", { class: "card", "aria-labelledby": id },
    h("h2", { id }, title),
    rows.length ? head : null,
    rowList(sorted(rows, PEOPLE_SORTS, f, "name"), (r) => personRow(data, r, f, maxPrs, bots), "Nobody matches the filters.", title));
}

function personRow(data, r, f, maxPrs, bots) {
  const href = personHref(r.person.login, f.milestone);
  const li = h("li", { class: "row prow", "data-row": "", "data-key": r.person.login },
    h("span", { class: "c-who" }, personLink(r.person.login, r.person.isBot, { withAvatar: true, milestone: f.milestone }),
      bots ? h("span", { class: "row-sub" }, sr("Roles: "), roles(r.person)) : null),
    h("span", { class: "c-owed" }, sr("Reviews owed: "),
      r.owes.length
        ? [h("strong", { class: "num" }, String(r.owes.length)),
          r.oldestMs === null ? null
            : h("span", { class: "basis" }, ` oldest ${formatDuration(r.oldestMs)}${r.oldestOpened ? " since opened" : ""}`)]
        : h("span", { class: "muted num" }, "0")),
    h("span", { class: "c-bar" }, sr("Own PRs by stage: "), stageBar(data, r.prs, maxPrs)),
    h("span", { class: "c-wip" }, sr("WIP per repo: "), wipChips(r.wip)),
    h("span", { class: "c-unres num" }, sr("Unresolved comments: "), String(r.unresolved)));
  // A click anywhere on the row goes where the name links.
  li.addEventListener("click", (e) => {
    if (e.target.closest("a, button") || !href) return;
    location.hash = href.slice(1);
  });
  return li;
}

function roles(person) {
  return person.roles.length
    ? person.roles.map((r) => (has(ROLE_LABEL, r) ? ROLE_LABEL[r] : String(r))).join(", ")
    : h("span", { class: "muted" }, "—");
}

/** A small stacked bar of a person's PRs by stage, on a scale shared by every row, with the counts as text. */
function stageBar(data, prs, max) {
  if (!prs.length) return h("span", { class: "muted" }, "none");
  const counts = data.stages.map((s) => [s, prs.filter((p) => p.stage === s.key).length]).filter(([, n]) => n);
  const bar = h("span", { class: "stack", "aria-hidden": "true" });
  bar.style.width = `${Math.max(8, (prs.length / max) * 100)}%`;
  for (const [s, n] of counts) {
    const seg = h("span", { class: `seg seg-${s.key}`, title: `${s.label}: ${n}` });
    seg.style.flexGrow = String(n);
    bar.append(seg);
  }
  return h("span", { class: "stack-cell" }, bar,
    h("span", { class: "stack-text" }, counts.map(([s, n]) => `${s.label} ${n}`).join(" · ")));
}

function wipChips(wip) {
  if (!wip.length) return h("span", { class: "muted" }, "—");
  return h("span", { class: "chips" }, wip.map(([repo, n, limit]) =>
    h("span", { class: `chip${n > limit ? " chip-over" : ""}`, title: `${n} of ${limit} review slots${n > limit ? ", over the limit" : ""}` },
      `${shortRepo(repo)} ${n}${n > limit ? " ▲" : ""}`)));
}

/**
 * One person's page; on Me, `note` says how the page knows it is you. With
 * `milestone`, the reviews and PRs are only those in it.
 */
export function personView(data, login, { isMe = false, note = null, milestone = "" } = {}) {
  const person = findPerson(data, login);
  if (!person) {
    return [section("Person not found",
      h("p", {}, "Nobody with that login is in this data. ", h("a", { href: "#/people" }, "See everyone"), "."))];
  }
  const q = queueOf(data, person, milestone);
  const scope = milestone ? ` ${milestoneWords(milestone)}` : "";
  const your = isMe ? "Your" : "Their";
  const header = h("section", { class: "card person-head" },
    h("div", { class: "person-title" }, avatar(person.login),
      h("h2", { id: "person-name", tabindex: "-1" }, person.login),
      h("span", { class: "kind" }, person.isBot ? ["bot ", botMark()] : "human"),
      person.roles.length ? h("span", { class: "kind" }, ["· ", roles(person)]) : null),
    note,
    areasList(person, isMe));
  const stat = (n, label, sub, hot) => h("li", { class: `stat${hot && n ? " hot" : ""}` },
    h("span", { class: "stat-n num" }, String(n)), h("span", { class: "stat-label" }, label), h("span", { class: "stat-sub" }, sub));
  const oldest = q.oldestMs !== null ? `oldest ${formatDuration(q.oldestMs)}${q.oldestOpened ? " since opened" : ""}` : "none waiting";
  const prs = (list, id, title, empty) => h("section", { class: "card", "aria-labelledby": id },
    h("h2", { id }, `${title}${scope} (${list.length})`),
    rowList(list, (p) => prRow(data, p, { milestone, blockers: true }), empty, title));
  const none = milestone ? `None ${milestoneWords(milestone)}.` : "None.";
  return [
    header,
    h("ul", { class: "stats", "aria-label": "Summary" },
      stat(q.owes.length, isMe ? "reviews you owe" : "reviews owed", oldest, true),
      stat(q.yourMove.length, `${your.toLowerCase()} PRs on ${isMe ? "your" : "their"} move`, "self-review, mergeable, or another author stage", true),
      stat(q.waiting.length, `${your.toLowerCase()} PRs waiting on others`, "reviewers, bots, CI, maintainers", false),
      stat(q.parked.length, `${your.toLowerCase()} parked PRs`, "drafts, and PRs off the governed branches", false)),
    h("section", { class: "card", "aria-labelledby": "owes-h" },
      h("h2", { id: "owes-h" }, `${isMe ? "Reviews you owe" : "Reviews owed"}${scope} (${q.owes.length})`),
      rowList(q.owes, (o) => o.prData
        ? prRow(data, o.prData, { milestone, part: yourPart(o, person.login, isMe) })
        : h("li", { class: "row" }, String(o.pr)),
      milestone ? `Nothing to review ${milestoneWords(milestone)}.` : "Nothing to review right now.", "Reviews owed")),
    prs(q.yourMove, "mine-h", `${your} PRs on ${isMe ? "your" : "their"} move`, milestone ? `No open PRs on ${isMe ? "your" : "their"} move ${milestoneWords(milestone)}.` : `No open PRs on ${isMe ? "your" : "their"} move.`),
    prs(q.waiting, "waiting-h", `${your} PRs waiting on others`, none),
    prs(q.parked, "parked-h", `${your} parked PRs`, none),
    section("Review slots per repository", wipList(q.wip),
      milestone ? h("p", { class: "sub" }, "Slots count every open PR on governed branches, whatever its milestone.") : null),
  ];
}

function areasList(person, isMe) {
  if (!person.areaList.length) return null;
  // Folded: the list runs long, and it is reference rather than the queue.
  const n = person.areaList.reduce((s, [, ids]) => s + ids.length, 0);
  return h("details", { class: "areas" },
    h("summary", {}, isMe ? "Areas you own or review" : "Areas they own or review",
      h("span", { class: "muted" }, ` (${n} in ${person.areaList.length} ${person.areaList.length === 1 ? "repository" : "repositories"})`)),
    h("ul", {}, person.areaList.map(([repo, ids]) =>
      h("li", {}, h("strong", {}, `${shortRepo(repo)}:`), " ",
        ids.map((a, i) => [i ? ", " : "", areaName(a)])))));
}

/** The engine's own "your part" wording: "`dpp` (you or bob or carol)", or the area alone when no one else may. */
function yourPart(o, login, isMe) {
  const you = isMe ? "you" : login;
  const items = o.areas.map((a) => h("li", {}, areaName(a.area),
    a.others.length ? ` (${[you, ...a.others].join(" or ")})` : null));
  if (o.rereview) items.push(h("li", {}, `re-review or resolve ${isMe ? "your" : "their"} objection`));
  if (!items.length) items.push(h("li", { class: "muted" }, "listed as a reviewer; no open area"));
  return h("ul", { class: "part" }, items);
}

function wipList(wip) {
  if (!wip.length) return h("p", { class: "muted" }, "No open PRs on governed branches.");
  return h("ul", { class: "wip" }, wip.map(([repo, n, limit]) => {
    const meter = h("span", { class: "meter", "aria-hidden": "true" });
    // Capped so a huge count cannot widen the page; the text says the number.
    for (let i = 0; i < Math.min(Math.max(limit, n), limit + 10); i++) {
      meter.append(h("span", { class: `slot${i < n ? " slot-used" : ""}${i >= limit ? " slot-over" : ""}` }));
    }
    const text = n > limit ? `${n} of ${limit}, over the limit` : `${n} of ${limit}`;
    return h("li", {}, h("span", { class: "wip-repo" }, shortRepo(repo)), meter, h("span", { class: n > limit ? "over" : "" }, text));
  }));
}

// "Me" is a login kept in this browser. The origin is shared with other
// sites, so only a validated login is ever stored, nothing else. A stored
// value that is not a login is removed; a login missing from today's data is
// kept for when it returns, and not used meanwhile.
export function storedMe(data) {
  let v = null;
  try {
    v = localStorage.getItem(ME_KEY);
  } catch {
    return { person: null, missing: null };
  }
  if (v !== null && !LOGIN_RE.test(v)) {
    forgetMe();
    return { person: null, missing: null };
  }
  const person = findPerson(data, v);
  return { person, missing: v !== null && !person ? v : null };
}

function rememberMe(person) {
  try {
    localStorage.setItem(ME_KEY, person.login);
  } catch {
    // Private mode or blocked storage: the choice lasts until the next navigation.
  }
}

function forgetMe() {
  try {
    localStorage.removeItem(ME_KEY);
  } catch {
    // Nothing stored, nothing to forget.
  }
}

export function meView(data, rerender, picked, milestone) {
  const stored = picked ? { person: picked, missing: null } : storedMe(data);
  const me = stored.person;
  if (me) {
    const change = () => {
      forgetMe();
      rerender(null);
    };
    return personView(data, me.login, {
      isMe: true,
      milestone,
      note: h("p", { class: "sub" }, "Kept in this browser only. ",
        h("button", { type: "button", class: "linkish", on: { click: change } }, "Not you? Change")),
    });
  }
  const pick = (p) => {
    rememberMe(p);
    rerender(p);
  };
  return [h("section", { class: "card" },
    h("h2", {}, "Who are you?"),
    h("p", { class: "sub" }, "Pick your GitHub login to see the reviews you owe and your PRs. It is kept in this browser only."),
    stored.missing ? h("p", { class: "sub" }, `${stored.missing} is not in this data, so there is nothing to show for them yet.`) : null,
    loginSearch(data, "me-q", "your GitHub login", pick))];
}

/**
 * A search over everyone in the data that lists matches as you type. Each
 * match is a button that calls `choose`, or with `links` a link to their
 * page; Enter chooses the only match, or an exact one.
 */
export function loginSearch(data, id, placeholder, choose, { links = false } = {}) {
  const input = h("input", { type: "search", id, placeholder, autocomplete: "off", spellcheck: "false" });
  const list = h("ul", { class: "picker" });
  // Only people whose login passes validation are listed, so only those can be chosen.
  const people = data.people
    .filter((p) => findPerson(data, p.login) === p)
    .sort((a, b) => a.isBot - b.isBot || String(a.login).localeCompare(String(b.login)));
  const matching = () => {
    const q = input.value.trim().toLowerCase();
    return people.filter((p) => !q || p.login.toLowerCase().includes(q));
  };
  const entry = (p) => [avatar(p.login), p.login, p.isBot ? [" ", botMark()] : null];
  const fill = () => {
    const shown = matching();
    list.replaceChildren(...shown.map((p) => h("li", {}, links
      ? h("a", { class: "pick", href: personHref(p.login) }, entry(p))
      : h("button", { type: "button", class: "pick", on: { click: () => choose(p) } }, entry(p)))));
    if (!shown.length) list.append(h("li", { class: "muted" }, "No one by that name."));
  };
  input.addEventListener("input", fill);
  input.addEventListener("keydown", (e) => {
    if (e.key !== "Enter") return;
    const shown = matching();
    const exact = shown.find((p) => p.login.toLowerCase() === input.value.trim().toLowerCase());
    if (shown.length === 1 || exact) {
      e.preventDefault();
      choose(exact || shown[0]);
    }
  });
  fill();
  return [h("div", { class: "field" }, h("label", { for: id }, "Search"), input), list];
}
