// People: what everyone owes and holds, a page per person, and "me". Counts
// describe current state; lateness colour stays on PRs and never ranks people.

import { h } from "./dom.js";
import { LOGIN_RE, has, findPerson, formatDuration, personHref, shortRepo } from "./model.js";
import { setFilters } from "./state.js";
import { effectiveWho } from "./filters.js";
import {
  prLink, personLink, avatar, botMark, stageChip, timeCell, badges, section, sortableTable, areaName, asksList,
} from "./ui.js";

const ME_KEY = "pr-hygiene:dashboard:me";

const ROLE_LABEL = { "author-bot": "opens PRs", "review-bot": "reviews PRs" };

/** One person's numbers, limited to a repository when the filter names one. */
function summarize(data, person, repo) {
  const inRepo = (r) => !repo || r === repo;
  const owes = person.owesList.filter((o) => inRepo(o.prData ? o.prData.repo : o.pr.split("#")[0]));
  // The longest wait among owed PRs, and whether that is a recorded wait or
  // only the PR's age.
  const oldest = owes
    .map((o) => o.prData)
    .filter((p) => p && p.ageMs !== null)
    .reduce((a, p) => (!a || p.ageMs > a.ageMs ? p : a), null);
  const prs = person.authoredPrs.filter((p) => inRepo(p.repo));
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

export function peopleView(data, f) {
  const who = effectiveWho(f, "all");
  const q = f.q.trim().toLowerCase();
  const rows = data.people
    .filter((p) => !q || String(p.login).toLowerCase().includes(q))
    .map((p) => summarize(data, p, f.repo));
  const maxPrs = Math.max(1, ...rows.map((r) => r.prs.length));
  const out = [];
  if (who !== "bot") {
    const humans = rows.filter((r) => !r.person.isBot);
    out.push(section(`People (${humans.length})`, peopleTable(data, humans, f, maxPrs, false)));
  }
  if (who !== "human") {
    const bots = rows.filter((r) => r.person.isBot);
    out.push(section(`Bots (${bots.length})`, peopleTable(data, bots, f, maxPrs, true)));
  }
  out.push(h("p", { class: "sub" },
    "Reviews owed: PRs ready for review that wait on the person, with how long the oldest has waited. ",
    "WIP: open, non-draft PRs on governed branches per repository, against that repository's review-slot limit; ▲ marks one over it."));
  return out;
}

function peopleTable(data, rows, f, maxPrs, bots) {
  const columns = [
    {
      key: "name", label: bots ? "Bot" : "Person", sort: (r) => [String(r.person.login)],
      cell: (r) => personLink(r.person.login, r.person.isBot, { withAvatar: true }),
    },
    bots ? { key: "roles", label: "Roles", cell: (r) => roles(r.person) } : null,
    {
      key: "owed", label: "Reviews owed", firstDir: "desc", sort: (r) => [r.owes.length, r.oldestMs ?? -1],
      cell: (r) => r.owes.length
        ? [h("span", { class: "num" }, String(r.owes.length)),
          r.oldestMs === null ? null
            : h("span", { class: "basis" }, ` oldest ${formatDuration(r.oldestMs)}${r.oldestOpened ? " since opened" : ""}`)]
        : h("span", { class: "muted" }, "0"),
    },
    { key: "prs", label: "Own PRs by stage", firstDir: "desc", sort: (r) => [r.prs.length], cell: (r) => stageBar(data, r.prs, maxPrs) },
    { key: "wip", label: "WIP per repo", firstDir: "desc", sort: (r) => [r.wipMax], cell: (r) => wipChips(r.wip) },
    { key: "unresolved", label: "Unresolved comments", numeric: true, firstDir: "desc", sort: (r) => [r.unresolved], cell: (r) => String(r.unresolved) },
  ].filter(Boolean);
  return sortableTable({
    id: bots ? "bots" : "people", columns, rows, sort: f.sort || "name", dir: f.dir,
    onSort: (key, dir) => setFilters("/people", f, { sort: key, dir }),
    empty: "Nobody matches the filters.",
  });
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

/** One person's page; on Me, `note` says how the page knows it is you. */
export function personView(data, login, { isMe = false, note = null } = {}) {
  const person = findPerson(data, login);
  if (!person) {
    return [section("Person not found",
      h("p", {}, "Nobody with that login is in this data. ", h("a", { href: "#/people" }, "See everyone"), "."))];
  }
  const s = summarize(data, person, "");
  const header = h("section", { class: "card person-head" },
    h("div", { class: "person-title" }, avatar(person.login),
      h("h2", { id: "person-name", tabindex: "-1" }, person.login),
      h("span", { class: "kind" }, person.isBot ? ["bot ", botMark()] : "human"),
      person.roles.length ? h("span", { class: "kind" }, ["· ", roles(person)]) : null),
    note,
    areasList(person, isMe));
  return [
    header,
    section(`${isMe ? "Reviews you owe" : "Reviews owed"} (${s.owes.length})`, owedTable(data, s.owes, person.login, isMe)),
    section(`${isMe ? "Your PRs" : "Their PRs"} (${s.prs.length})`, ownTable(data, s.prs)),
    section("Review slots per repository", wipList(s.wip)),
  ];
}

function areasList(person, isMe) {
  if (!person.areaList.length) return null;
  return h("div", { class: "areas" },
    h("h3", {}, isMe ? "Areas you own or review" : "Areas they own or review"),
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

function owedTable(data, owes, login, isMe) {
  const rows = [...owes].sort((a, b) => (b.prData?.ageMs ?? -1) - (a.prData?.ageMs ?? -1));
  return sortableTable({
    columns: [
      { key: "pr", label: "PR", cell: (o) => o.prData ? prLink(o.prData) : String(o.pr) },
      { key: "author", label: "Author", cell: (o) => o.prData ? personLink(o.prData.author, o.prData.isBot) : "—" },
      { key: "part", label: isMe ? "Your part" : "Their part", cell: (o) => yourPart(o, login, isMe) },
      { key: "wait", label: "Waiting", cell: (o) => o.prData ? timeCell(o.prData) : "—" },
    ],
    rows, empty: "Nothing to review right now.",
  });
}

function ownTable(data, prs) {
  const rows = [...prs].sort((a, b) => data.stage[a.stage].order - data.stage[b.stage].order || (b.ageMs ?? -1) - (a.ageMs ?? -1));
  return sortableTable({
    columns: [
      { key: "pr", label: "PR", cell: (p) => prLink(p) },
      { key: "stage", label: "Stage", cell: (p) => stageChip(data, p.stage) },
      { key: "time", label: "Time in stage", cell: (p) => timeCell(p) },
      { key: "next", label: "Next action", cell: (p) => nextAndBlockers(p) },
      { key: "flags", label: "Flags", cell: (p) => badges(p) },
    ],
    rows, empty: "No open PRs.",
  });
}

function nextAndBlockers(p) {
  const asks = asksList(p);
  if (!p.next_action && !p.blockers.length && !asks) return h("span", { class: "muted" }, "—");
  const rest = p.blockers.filter((b) => b !== p.next_action);
  return h("div", { class: "next" },
    p.next_action ? h("span", {}, String(p.next_action)) : null,
    rest.length ? h("ul", { class: "blockers" }, rest.map((b) => h("li", {}, String(b)))) : null,
    asks);
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
function storedMe(data) {
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

export function meView(data, rerender, picked) {
  const stored = picked ? { person: picked, missing: null } : storedMe(data);
  const me = stored.person;
  if (me) {
    const change = () => {
      forgetMe();
      rerender(null);
    };
    return personView(data, me.login, {
      isMe: true,
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
