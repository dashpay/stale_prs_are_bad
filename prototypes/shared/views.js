// The views both prototypes render: Team, People, a person, Me. The data,
// filters, routes and wording are the live page's (site/js); a "skin" from
// each prototype supplies only its marks, glyphs and chrome, and its own CSS
// does the rest.

import { h, openPr } from "../../site/js/dom.js";
import {
  LATENESS, formatDuration, formatHours, shortRepo, personHref, findPerson, has,
  closeOrRevive, idleWords, milestoneWords, areaLabel,
} from "../../site/js/model.js";
import { parseHash, buildHash, setFilters } from "../../site/js/state.js";
import { filterPrs, filterBar, effectiveWho } from "../../site/js/filters.js";
import { avatar, asksList, areaName, personLink } from "../../site/js/ui.js";
import { loginSearch } from "../../site/js/people.js";
import { privacyView } from "../../site/js/privacy.js";
import * as core from "./core.js";
import { installKeys, openPalette, openHelp, focusTarget } from "./keys.js";

const THEME_KEY = "pr-hygiene:proto-theme";
const ROLE_LABEL = { "author-bot": "opens PRs", "review-bot": "reviews PRs" };

export function start(skin) {
  const root = document.getElementById("app");
  let data = null;
  let me = null;
  let filtersOpen = false;
  // A PR picked in the palette: its row opens once the board shows it.
  let pendingExpand = null;

  // ---- theme: system, or a choice kept in this browser ----
  const themes = ["", "dark", "light"];
  let theme = "";
  try {
    theme = themes.includes(localStorage.getItem(THEME_KEY)) ? localStorage.getItem(THEME_KEY) : "";
  } catch {
    theme = "";
  }
  const applyTheme = () => {
    if (theme) document.documentElement.dataset.theme = theme;
    else delete document.documentElement.dataset.theme;
  };
  applyTheme();
  const cycleTheme = () => {
    theme = themes[(themes.indexOf(theme) + 1) % themes.length];
    try {
      localStorage.setItem(THEME_KEY, theme);
    } catch {
      // Kept for this page only.
    }
    applyTheme();
    render();
  };

  // ---- keyboard ----
  const api = {
    go: (hash) => {
      location.hash = hash;
    },
    rows: () => [...document.querySelectorAll("main [data-row]")].filter((el) => el.offsetParent !== null),
    toggle: (row) => {
      if (row.dataset.href) location.hash = row.dataset.href;
      else toggleDetail(row);
    },
    open: (row) => {
      const p = data.prByKey.get(row.dataset.key);
      if (p && p.url) openPr(p.url);
    },
    toggleLate: () => {
      const { route, filters: f } = parseHash(location.hash, data);
      if (route.view === "team") setFilters("/", f, { late: !f.late });
    },
    items: () => core.paletteItems(data, {
      go: api.go, openPr, me,
      showPr: (p) => {
        pendingExpand = p.key;
        const hash = buildHash("/", { q: p.key, who: "all" });
        if (location.hash === hash) render();
        else location.hash = hash;
      },
      setRepo: (r) => {
        location.hash = buildHash("/", { repo: r });
      },
    }),
  };

  // ---- shared pieces ----
  const sr = core.sr;

  function ageKind(p) {
    if (p.since_basis !== "engine") return "opened";
    return p.lateness || "untimed";
  }

  /** Time in stage: the mark, the duration, and in words what the mark means. */
  function age(p) {
    if (p.ageMs === null) return h("span", { class: "age age-none" }, "—", sr("no start time"));
    const kind = ageKind(p);
    // The words ride with the mark, as on the live page; "on time" stays in
    // the legend and for screen readers, so the actionable words stand out.
    const word = kind === "opened" ? "since opened" : has(LATENESS, kind) ? LATENESS[kind].label : "";
    const shown = kind === "ok" ? null : word;
    return h("span", { class: `age age-${kind}` }, skin.mark(kind), h("span", { class: "num" }, formatDuration(p.ageMs)),
      shown ? h("span", { class: "lw" }, shown) : word ? sr(`, ${word}`) : null);
  }

  /** The marks and each timed stage's thresholds, in the live chart legend's words. */
  function legend(prs) {
    const keys = [["ok", "on time"], ["late", "late"], ["very-late", "very late"]];
    if (prs.some((p) => ageKind(p) === "untimed")) keys.push(["untimed", "never late in this stage"]);
    keys.push(["opened", "since opened; stage start not recorded yet"]);
    const rules = data.stages
      .filter((s) => s.entryRecorded && s.lateHours)
      .map((s) => `${s.label} late after ${formatHours(s.lateHours[0])}, very late after ${formatHours(s.lateHours[1])}`);
    return h("div", { class: "legend" },
      h("ul", { "aria-label": "Legend" }, keys.map(([k, t]) => h("li", {}, skin.mark(k), t))),
      rules.length ? h("p", { class: "caption" }, `${rules.join(" · ")}. Only these stages record when a PR entered them.`) : null);
  }

  /** What the row's second line says: the next action, then who the PR waits on, in the engine's words. */
  function waitsText(p) {
    const asks = p.asks.map((a) => `${areaLabel(a.area)}: ${a.approvers.length ? a.approvers.join(" or ") : "nobody may approve"}`);
    if (p.objectors.length) asks.push(`re-review or resolve: ${p.objectors.join(", ")}`);
    return [p.next_action, asks.length ? `waits on ${asks.join("; ")}` : null].filter(Boolean).join(" — ");
  }

  function timeText(p) {
    if (p.ageMs === null) return "no start time";
    const s = data.stage[p.stage];
    if (p.since_basis !== "engine") return `${formatDuration(p.ageMs)} since opened; this stage's start is not recorded yet`;
    const late = p.lateness ? `, ${LATENESS[p.lateness].label}` : "";
    const rule = s.entryRecorded && s.lateHours
      ? ` (late after ${formatHours(s.lateHours[0])}, very late after ${formatHours(s.lateHours[1])})` : " (never late in this stage)";
    return `${formatDuration(p.ageMs)} in this stage${late}${rule}`;
  }

  function flags(p) {
    const g = skin.glyph;
    const out = [];
    const flag = (cls, text, label) => h("span", { class: `flag ${cls}`, title: label }, h("span", { "aria-hidden": "true" }, text), sr(label));
    if (p.unresolved_comments > 0) out.push(flag("cmt", `${g.comments}${p.unresolved_comments}`, `${p.unresolved_comments} unresolved review comments`));
    if (p.ci_failing) out.push(flag("bad", g.ci, "CI failing"));
    if (p.merge_conflict) out.push(flag("bad", g.conflict, "merge conflict"));
    if (p.changes_requested) out.push(flag("warn", g.cr, "changes requested"));
    if (p.idle) out.push(flag("dim", g.idle, "idle: no update for the idle period"));
    return out;
  }

  function prRef(p) {
    const label = `${shortRepo(p.repo)}#${p.number}`;
    return p.url
      ? h("a", { href: p.url, target: "_blank", rel: "noopener noreferrer", class: "ref" }, label)
      : h("span", { class: "ref" }, label);
  }

  function prRow(p, f, { part = null } = {}) {
    const li = h("li", { class: `row prrow age-${ageKind(p)}`, "data-row": "", "data-key": p.key });
    li.append(
      h("span", { class: "c-age" }, age(p)),
      h("span", { class: "c-ref" }, prRef(p)),
      h("span", { class: "c-main" },
        h("button", { type: "button", class: "title", "aria-expanded": "false", on: { click: () => toggleDetail(li) } }, p.title || "(no title)"),
        part ? h("span", { class: "sub part", title: part }, part)
          : waitsText(p) ? h("span", { class: "sub next", title: waitsText(p) }, waitsText(p)) : null),
      h("span", { class: "c-stage" }, skin.stage(data, p.stage)),
      h("span", { class: "c-author" }, personLink(p.author, p.isBot, { withAvatar: skin.avatars, milestone: f.milestone })),
      h("span", { class: "c-flags" }, flags(p)));
    li.addEventListener("click", (e) => {
      if (e.target.closest("a, button")) return;
      toggleDetail(li);
    });
    return li;
  }

  function detail(p) {
    const rest = p.blockers.filter((b) => b !== p.next_action);
    const kv = (k, ...v) => [h("dt", {}, k), h("dd", {}, v)];
    const s = data.stage[p.stage];
    const asks = asksList(p);
    const longFlags = [
      p.unresolved_comments ? `${p.unresolved_comments} unresolved review comments` : null,
      p.ci_failing ? "CI failing" : null, p.merge_conflict ? "merge conflict" : null,
      p.changes_requested ? "changes requested" : null, p.idle ? idleWords(data) : null,
    ].filter(Boolean);
    return h("li", { class: "detail" },
      h("dl", {},
        kv("Stage", skin.stage(data, p.stage), s.note ? ` ${s.note}` : ""),
        kv("Time", timeText(p)),
        p.next_action ? kv("Next", p.next_action) : null,
        rest.length ? kv("Blockers", h("ul", {}, rest.map((b) => h("li", {}, b)))) : null,
        asks ? kv("Waiting for", asks) : null,
        longFlags.length ? kv("Flags", longFlags.join(" · ")) : null,
        p.areas.length ? kv("Areas", p.areas.map((a, i) => [i ? ", " : "", areaName(a)])) : null,
        p.milestone ? kv("Milestone", p.milestone) : null,
        p.updatedAgoMs !== null ? kv("Updated", `${formatDuration(p.updatedAgoMs)} ago`) : null,
        p.tracked ? null : kv("Note", "Known only from the review engine; the board's own filters left it out")),
      p.url ? h("a", { href: p.url, target: "_blank", rel: "noopener noreferrer", class: "gh" }, "Open on GitHub ↗") : null);
  }

  function toggleDetail(li) {
    const btn = li.querySelector("button.title");
    const next = li.nextElementSibling;
    if (next && next.classList.contains("detail")) {
      next.remove();
      btn?.setAttribute("aria-expanded", "false");
      li.classList.remove("open");
      return;
    }
    const p = data.prByKey.get(li.dataset.key);
    if (!p) return;
    li.after(detail(p));
    btn?.setAttribute("aria-expanded", "true");
    li.classList.add("open");
  }

  function sortControl(path, f, sorts, def) {
    const cur = has(sorts, f.sort) ? f.sort : def;
    const dir = f.dir || sorts[cur].firstDir;
    return h("div", { class: "sort" },
      h("label", { for: "sort", class: "sort-label" }, "sort"),
      h("select", { id: "sort", on: { change: (e) => setFilters(path, f, { sort: e.target.value, dir: "" }) } },
        Object.entries(sorts).map(([k, s]) => h("option", { value: k, selected: k === cur }, s.label))),
      h("button", {
        type: "button", id: "sort-dir", class: "icon-btn",
        "aria-label": dir === "desc" ? "Descending; select to reverse" : "Ascending; select to reverse",
        on: { click: () => setFilters(path, f, { sort: cur, dir: dir === "desc" ? "asc" : "desc" }) },
      }, dir === "desc" ? "↓" : "↑"));
  }

  function panel(id, title, count, tools, ...body) {
    return h("section", { class: "panel", "aria-labelledby": id },
      h("div", { class: "panel-head" },
        h("h2", { id }, title, count === null ? null : h("span", { class: "count" }, String(count))), tools),
      body);
  }

  function fold(id, title, count, ...body) {
    return h("details", { class: "panel fold" },
      h("summary", { id }, h("span", { class: "fold-title" }, title), count === null ? null : h("span", { class: "count" }, String(count))),
      body);
  }

  function rows(list, make, empty) {
    return list.length ? h("ul", { class: "rows" }, list.map(make)) : h("p", { class: "empty" }, empty);
  }

  // ---- chrome ----
  function top(route, f) {
    const links = [["team", "/", "Team"], ["people", "/people", "People"], ["me", "/me", "Me"]];
    const nav = h("nav", { class: "tabs", "aria-label": "Views" }, links.map(([view, path, label]) => {
      const current = route.view === view || (view === "people" && route.view === "person");
      const href = view === "me" ? buildHash(path, { milestone: f.milestone }) : buildHash(path, { ...f, q: "" }, { keepSort: false });
      return h("a", { href, class: "tab", "aria-current": current ? "page" : null }, skin.tab(label));
    }));
    const { time, commit } = core.asOfText(data);
    const themeName = theme || "system";
    return h("header", { class: "top" },
      h("a", { class: "brand", href: "#/" }, skin.brand),
      nav,
      h("button", { type: "button", class: "jump", id: "jump", "aria-label": "Jump to (⌘K)", on: { click: () => openPalette(api) } },
        h("span", { class: "jump-text" }, skin.jumpLabel), h("kbd", {}, "⌘K")),
      h("span", { class: "asof", title: "When this data was taken (UTC), and the analyzer's commit" },
        core.isStale(data) ? h("span", { class: "stale" }, "stale · ") : null,
        h("span", { class: "asof-date" }, `${time.slice(0, 10)} `), `${time.slice(11)}Z`,
        commit ? h("span", { class: "asof-commit" }, ` · ${commit}`) : null),
      h("button", { type: "button", class: "icon-btn", id: "theme", title: `Theme: ${themeName}`, "aria-label": `Theme: ${themeName}; select to change`, on: { click: cycleTheme } },
        skin.themeGlyph(themeName)),
      h("button", { type: "button", class: "icon-btn", "aria-label": "Keyboard shortcuts", title: "Keyboard shortcuts (?)", on: { click: openHelp } }, "?"));
  }

  function banners() {
    return core.warnings(data).map((w) => h("div", { class: "banner", role: "status" }, skin.warnGlyph, w));
  }

  function filters(path, f, opts) {
    const form = filterBar(data, path, f, { ...opts, onText: render });
    if (!form) return null;
    const keys = ["repo", "milestone", "area", "stage", "late"].filter((k) => opts.show.includes(k) && f[k]);
    if (opts.show.includes("who") && f.who) keys.push("who");
    const btn = h("button", {
      type: "button", class: "fbar-toggle", "aria-expanded": String(filtersOpen), "aria-controls": "fbar-form",
    }, "Filters", keys.length ? h("span", { class: "count" }, String(keys.length)) : null);
    form.id = "fbar-form";
    const wrap = h("div", { class: `fbar${filtersOpen ? " open" : ""}${opts.show.includes("q") ? " has-q" : ""}` }, btn, form);
    btn.addEventListener("click", () => {
      filtersOpen = !filtersOpen;
      wrap.classList.toggle("open", filtersOpen);
      btn.setAttribute("aria-expanded", String(filtersOpen));
    });
    return wrap;
  }

  // ---- Team ----
  function emptyText(f) {
    if (!f.late) return "No PRs match the filters.";
    const timed = data.stages.filter((s) => s.entryRecorded && s.lateHours).map((s) => s.label);
    return timed.length
      ? `No late PRs match the filters. Only ${timed.join(", ")} records when a PR entered it, so only PRs there can be late.`
      : "No stage records when a PR entered it yet, so no PR can be late.";
  }

  function youStrip(f) {
    const person = me && findPerson(data, me);
    if (!person) {
      return h("p", { class: "you you-none" }, "Pick who you are on ", h("a", { href: "#/me" }, "Me"),
        " (", h("kbd", {}, "g"), " ", h("kbd", {}, "m"), ") and what you owe shows here first.");
    }
    const q = core.queueOf(data, person, f.milestone);
    const meHref = buildHash("/me", { milestone: f.milestone });
    return h("section", { class: "you", "aria-label": "Your move" },
      h("span", { class: "you-who" }, avatar(person.login), h("span", { class: "you-name" }, person.login)),
      h("a", { href: meHref, class: `you-stat${q.owes.length ? " hot" : ""}` },
        h("b", {}, String(q.owes.length)), ` review${q.owes.length === 1 ? "" : "s"} you owe`,
        q.oldestMs !== null ? h("span", { class: "you-dim" }, ` · oldest ${formatDuration(q.oldestMs)}${q.oldestOpened ? " since opened" : ""}`) : null),
      h("a", { href: meHref, class: `you-stat${q.yourMove.length ? " hot" : ""}` },
        h("b", {}, String(q.yourMove.length)), ` of your PRs on your move`),
      h("a", { href: meHref, class: "you-stat" }, h("b", {}, String(q.waiting.length)), " waiting on others"),
      h("a", { href: meHref, class: "you-go" }, "Open Me →"));
  }

  function moves(f) {
    const list = core.whoseMove(data, f);
    const who = f.who || "people";
    const scope = who === "people" ? "Waiting on people (humans' PRs, and bots' PRs waiting on reviewers or maintainers)"
      : who === "bot" ? "Bots' PRs only" : "All open PRs, bots' own included";
    const toggle = who === "people"
      ? h("a", { href: buildHash("/", { ...f, who: "all" }) }, "include bots' own")
      : h("a", { href: buildHash("/", { ...f, who: "" }) }, "only what waits on people");
    const item = (m) => {
      const active = f.owner === m.key;
      const parts = m.parts.length ? m.parts.map(([s, n]) => `${s.label} ${n}`).join(" · ") : "none open";
      return h("li", { class: m.parked ? "parked-li" : null },
        h("a", {
          href: buildHash("/", { ...f, owner: active ? "" : m.key }),
          class: `move${active ? " active" : ""}${m.n ? "" : " zero"}${m.parked ? " parked" : ""}`,
          "aria-current": active ? "true" : null,
          title: m.parked ? "Drafts, PRs off the governed branches and PRs with no engine verdict: nobody is asked to move them today." : null,
        },
        skin.moveCursor ? h("span", { class: "cursor", "aria-hidden": "true" }, active ? ">" : " ") : null,
        h("span", { class: "move-label" }, m.label),
        h("span", { class: "move-n" }, String(m.n)),
        m.late
          ? h("span", { class: `move-late ${m.veryLate ? "vl" : "l"}` }, skin.mark(m.veryLate ? "very-late" : "late"),
            `${m.late} late`, m.veryLate ? h("span", { class: "move-vl" }, ` · ${m.veryLate} very late`) : null)
          : h("span", { class: "move-late none" }),
        h("span", { class: "move-parts" }, parts)));
    };
    return h("section", { class: "moves-wrap", "aria-labelledby": "moves-h" },
      h("div", { class: "panel-head" }, h("h2", { id: "moves-h" }, "Whose move")),
      h("ul", { class: "moves" }, list.map(item)),
      h("p", { class: "caption moves-caption" }, scope, " · ", toggle));
  }

  function team(f) {
    const visible = filterPrs(data.prs, f);
    const sorted = core.sortPrs(data, visible, f.sort || "time", f.dir);
    const revive = closeOrRevive(visible);
    const period = data.idleDays ? `for ${data.idleDays} days or more` : "for the idle period";
    const idle = idleWords(data);
    return [
      youStrip(f),
      moves(f),
      panel("prs-h", "Open PRs", visible.length, sortControl("/", f, core.PR_SORTS, "time"),
        legend(visible),
        skin.listHead ? skin.listHead() : null,
        rows(sorted, (p) => prRow(p, f), emptyText(f))),
      fold("revive-h", "Close or revive", revive.length,
        h("p", { class: "caption" }, `Drafts and PRs off the governed branches that nobody has touched ${period}, and PRs with a merge conflict.`),
        rows(revive, (r) => prRow(r.pr, f, {
          part: [r.idle ? idle : null, r.conflict ? "merge conflict" : null].filter(Boolean).join(" · ")
            + (r.pr.updatedAgoMs !== null ? ` · last update ${formatDuration(r.pr.updatedAgoMs)} ago` : ""),
        }), "Nothing idle or conflicting.")),
    ];
  }

  // ---- People ----
  function stageBar(prs, max) {
    if (!prs.length) return h("span", { class: "muted" }, "none");
    const counts = data.stages.map((s) => [s, prs.filter((p) => p.stage === s.key).length]).filter(([, n]) => n);
    const bar = h("span", { class: "stack", "aria-hidden": "true" });
    bar.style.width = `${Math.max(8, (prs.length / max) * 100)}%`;
    for (const [s, n] of counts) {
      const seg = h("span", { class: `seg s-${s.key}`, title: `${s.label}: ${n}` });
      seg.style.flexGrow = String(n);
      bar.append(seg);
    }
    return h("span", { class: "stack-cell" }, bar,
      h("span", { class: "stack-text" }, counts.map(([s, n]) => `${s.label} ${n}`).join(" · ")));
  }

  function wipChips(wip) {
    if (!wip.length) return h("span", { class: "muted" }, "—");
    return wip.map(([repo, n, limit]) =>
      h("span", { class: `chip${n > limit ? " over" : ""}`, title: `${n} of ${limit} review slots${n > limit ? ", over the limit" : ""}` },
        `${shortRepo(repo)} ${n}`, n > limit ? [h("span", { "aria-hidden": "true" }, " ▲"), sr(", over the limit")] : null));
  }

  function roles(person) {
    return person.roles.map((r) => (has(ROLE_LABEL, r) ? ROLE_LABEL[r] : String(r))).join(", ");
  }

  function personRow(r, maxPrs, bots, f) {
    const href = personHref(r.person.login, f.milestone);
    const li = h("li", { class: "row prow", "data-row": "", "data-href": href, "data-key": r.person.login },
      h("span", { class: "c-who" }, personLink(r.person.login, r.person.isBot, { withAvatar: true, milestone: f.milestone }),
        bots && r.person.roles.length ? h("span", { class: "sub" }, roles(r.person)) : null),
      h("span", { class: "c-owed" }, sr("Reviews owed: "),
        r.owes.length ? h("b", { class: "num" }, String(r.owes.length)) : h("span", { class: "muted num" }, "0"),
        r.oldestMs !== null ? h("span", { class: "sub" }, ` oldest ${formatDuration(r.oldestMs)}${r.oldestOpened ? " since opened" : ""}`) : null),
      h("span", { class: "c-bar" }, sr("Own PRs by stage: "), stageBar(r.prs, maxPrs)),
      h("span", { class: "c-wip" }, sr("WIP per repository: "), wipChips(r.wip)),
      h("span", { class: "c-unres num", title: "Unresolved review comments on their PRs" }, sr("Unresolved comments: "), String(r.unresolved)));
    li.addEventListener("click", (e) => {
      if (e.target.closest("a, button")) return;
      location.hash = href;
    });
    return li;
  }

  function peopleHead() {
    return h("div", { class: "rows-head prow-head", "aria-hidden": "true" },
      h("span", {}, "who"), h("span", {}, "reviews owed"), h("span", {}, "own PRs by stage"), h("span", {}, "WIP / slots"), h("span", { class: "n" }, "unresolved"));
  }

  function people(f) {
    const who = effectiveWho(f, "all");
    const q = f.q.trim().toLowerCase();
    const all = data.people.filter((p) => !q || p.login.toLowerCase().includes(q)).map((p) => core.summarize(data, p, f));
    const maxPrs = Math.max(1, ...all.map((r) => r.prs.length));
    const sort = f.sort || "name";
    const out = [h("div", { class: "toolbar" }, sortControl("/people", f, core.PEOPLE_SORTS, "name"))];
    if (who !== "bot") {
      const humans = core.sortPeople(all.filter((r) => !r.person.isBot), sort, f.dir);
      out.push(panel("people-h", "People", humans.length, null, peopleHead(),
        rows(humans, (r) => personRow(r, maxPrs, false, f), "Nobody matches the filters.")));
    }
    if (who !== "human") {
      const bots = core.sortPeople(all.filter((r) => r.person.isBot), sort, f.dir);
      out.push(panel("bots-h", "Bots", bots.length, null, peopleHead(),
        rows(bots, (r) => personRow(r, maxPrs, true, f), "Nobody matches the filters.")));
    }
    out.push(h("p", { class: "caption foot-note" },
      "Reviews owed: PRs ready for review that wait on the person, with how long the oldest has waited. ",
      "WIP: open, non-draft PRs on governed branches per repository, against that repository's review-slot limit; ▲ marks one over it.",
      f.milestone ? " WIP counts every milestone." : ""));
    return out;
  }

  // ---- a person, and Me ----
  function yourPart(o, login, isMe) {
    const you = isMe ? "you" : login;
    const items = o.areas.map((a) => `${areaLabel(a.area)}${a.others.length ? ` (${[you, ...a.others].join(" or ")})` : ""}`);
    if (o.rereview) items.push(`re-review or resolve ${isMe ? "your" : "their"} objection`);
    return items.length ? items.join(" · ") : "listed as a reviewer; no open area";
  }

  function wipList(wip) {
    if (!wip.length) return h("p", { class: "empty" }, "No open PRs on governed branches.");
    return h("ul", { class: "wip" }, wip.map(([repo, n, limit]) => {
      const meter = h("span", { class: "meter", "aria-hidden": "true" });
      for (let i = 0; i < Math.min(Math.max(limit, n), limit + 10); i++) {
        meter.append(h("span", { class: `slot${i < n ? " used" : ""}${i >= limit ? " over" : ""}` }, skin.slotGlyph ? skin.slotGlyph(i < n) : null));
      }
      return h("li", {}, h("span", { class: "wip-repo" }, shortRepo(repo)), meter,
        h("span", { class: `num${n > limit ? " over-text" : ""}` }, n > limit ? `${n} of ${limit}, over the limit` : `${n} of ${limit}`));
    }));
  }

  function stat(label, n, sub, hot) {
    return h("div", { class: `stat${hot && n ? " hot" : ""}` },
      h("span", { class: "stat-n num" }, String(n)), h("span", { class: "stat-label" }, label), h("span", { class: "stat-sub" }, sub));
  }

  function personPage(login, f, isMe, note) {
    const person = findPerson(data, login);
    if (!person) {
      return [panel("nf-h", "Person not found", null, null,
        h("p", { class: "empty" }, "Nobody with that login is in this data. ", h("a", { href: "#/people" }, "See everyone"), "."))];
    }
    const q = core.queueOf(data, person, f.milestone);
    const scope = f.milestone ? ` ${milestoneWords(f.milestone)}` : "";
    const your = isMe ? "Your" : "Their";
    const oldest = q.oldestMs !== null ? `oldest ${formatDuration(q.oldestMs)}${q.oldestOpened ? " since opened" : ""}` : "none waiting";
    return [
      h("section", { class: "phead" },
        avatar(person.login),
        h("h1", { id: "person-name", tabindex: "-1" }, person.login),
        h("span", { class: "kind" }, person.isBot ? "bot" : "human", person.roles.length ? ` · ${roles(person)}` : ""),
        note),
      h("div", { class: "stats" },
        stat(isMe ? "reviews you owe" : "reviews owed", q.owes.length, oldest, true),
        stat(isMe ? "PRs on your move" : "PRs on their move", q.yourMove.length, "self-review, mergeable, or another author stage", true),
        stat("waiting on others", q.waiting.length, "reviewers, bots, CI, maintainers"),
        stat("parked", q.parked.length, "drafts and PRs off the policy")),
      panel("owes-h", `${isMe ? "Reviews you owe" : "Reviews owed"}${scope}`, q.owes.length, null,
        rows(q.owes, (o) => o.prData
          ? prRow(o.prData, f, { part: yourPart(o, person.login, isMe) })
          : h("li", { class: "row" }, String(o.pr)),
        f.milestone ? `Nothing to review ${milestoneWords(f.milestone)}.` : "Nothing to review right now.")),
      panel("mine-h", `${your} PRs on ${isMe ? "your" : "their"} move${scope}`, q.yourMove.length, null,
        rows(q.yourMove, (p) => prRow(p, f), "Nothing waits on the author.")),
      panel("wait-h", `${your} PRs waiting on others${scope}`, q.waiting.length, null,
        rows(q.waiting, (p) => prRow(p, f), "None.")),
      fold("parked-h", `${your} parked PRs${scope}`, q.parked.length, rows(q.parked, (p) => prRow(p, f), "None.")),
      panel("slots-h", "Review slots per repository", null, null, wipList(q.wip),
        f.milestone ? h("p", { class: "caption" }, "Slots count every open PR on governed branches, whatever its milestone.") : null),
      person.areaList.length
        ? fold("areas-h", isMe ? "Areas you own or review" : "Areas they own or review", null,
          h("ul", { class: "areas" }, person.areaList.map(([repo, ids]) =>
            h("li", {}, h("strong", {}, `${shortRepo(repo)}: `), ids.map((a, i) => [i ? ", " : "", areaName(a)])))))
        : null,
    ];
  }

  function mePage(f) {
    const person = me && findPerson(data, me);
    if (person) {
      const fromParam = new URLSearchParams(location.search).get("user");
      const note = h("p", { class: "me-note" },
        fromParam ? "Shown for ?user= in the address. " : "Kept in this browser only. ",
        h("button", {
          type: "button", class: "linkish", id: "me-change",
          on: {
            click: () => {
              core.forgetMe();
              me = null;
              if (fromParam) history.replaceState(null, "", location.pathname + location.hash);
              render();
            },
          },
        }, "Not you? Change"),
        h("span", { class: "muted" }, " · Sign-in and your own speed work as on the live page."));
      return personPage(person.login, f, true, note);
    }
    return [panel("who-h", "Who are you?", null, null,
      h("p", { class: "caption" }, "Pick your GitHub login to see the reviews you owe and your PRs. It is kept in this browser only."),
      loginSearch(data, "me-q", "your GitHub login", (p) => {
        core.rememberMe(p.login);
        me = p.login;
        render();
      }))];
  }

  // ---- render ----
  function render() {
    const { route, filters: f } = parseHash(location.hash, data);
    const active = document.activeElement;
    const hadFocus = active && active !== document.body && root.contains(active);
    const focusId = active && active.id ? active.id : null;
    const rowKey = active?.closest?.("[data-row]")?.dataset.key || null;
    const caret = active && typeof active.selectionStart === "number" ? [active.selectionStart, active.selectionEnd] : null;

    let bar = null;
    let content;
    if (route.view === "team") {
      bar = filters("/", f, { show: ["repo", "milestone", "area", "stage", "who", "late", "q", "owner"], whoDefault: "people", searchHint: "author, reviewer or title" });
      content = team(f);
    } else if (route.view === "people") {
      bar = filters("/people", f, { show: ["repo", "milestone", "who", "q"], whoDefault: "all", searchHint: "login" });
      content = people(f);
    } else if (route.view === "person") {
      const person = findPerson(data, route.login);
      if (person) bar = filters(`/people/${encodeURIComponent(person.login)}`, f, { show: ["milestone"] });
      content = personPage(route.login, f, person && person.login === me, null);
    } else if (route.view === "privacy") {
      content = privacyView();
    } else {
      bar = filters("/me", f, { show: ["milestone"] });
      content = mePage(f);
    }
    document.title = route.view === "person" && route.login ? `${route.login} · PR Hygiene` : "PR Hygiene";
    root.replaceChildren(...[
      top(route, f), ...banners(), bar,
      h("main", { id: "main", tabindex: "-1" }, content),
      skin.footer(route),
    ].filter(Boolean));

    let el = focusId ? document.getElementById(focusId) : null;
    let reveal = false;
    if (!el && rowKey) {
      const row = [...document.querySelectorAll("main [data-row]")].find((r) => r.dataset.key === rowKey);
      el = row ? focusTarget(row) : null;
    }
    if (pendingExpand) {
      const row = [...document.querySelectorAll("main [data-row]")].find((r) => r.dataset.key === pendingExpand);
      pendingExpand = null;
      if (row) {
        toggleDetail(row);
        el = focusTarget(row);
        reveal = true;
      }
    }
    if (el) {
      el.focus({ preventScroll: true });
      if (reveal) el.scrollIntoView({ block: "center" });
      if (caret && typeof el.setSelectionRange === "function") {
        try {
          el.setSelectionRange(caret[0], caret[1]);
        } catch {
          // Some inputs have no caret.
        }
      }
    } else if (hadFocus) {
      (document.getElementById("person-name") || document.getElementById("main")).focus({ preventScroll: true });
    }
  }

  core.loadData().then((d) => {
    data = d;
    me = core.meLogin(d);
    render();
    addEventListener("hashchange", render);
    installKeys(api);
  }, (e) => {
    root.replaceChildren(h("div", { class: "banner banner-error", role: "alert" }, e.message));
  });
}
