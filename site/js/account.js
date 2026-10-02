// Me with sign-in, where the PR Hygiene service serves this page. Whether it
// does is learned by asking `api/v1/me`: 200 is signed in, 401 is signed out
// with sign-in offered, and anything else (GitHub Pages answers 404) means
// there is no sign-in here, so Me keeps the login picker kept in this browser.
//
// Every request this page makes to the service is in this file, to a fixed
// path beside the page. Same-origin requests carry the session cookie;
// the cookie itself is HttpOnly, so no script here ever sees it.

import { h } from "./dom.js";
import { LOGIN_RE, findPerson, personHref } from "./model.js";
import { personView, loginSearch } from "./people.js";
import { avatar, section } from "./ui.js";
import { readSpeed, speedBody } from "./speed.js";

// The request mode stays the default, "cors": in mode "same-origin", under
// this page's no-referrer policy, browsers send `Origin: null` on a POST or
// DELETE, and the service refuses any change without its own exact Origin.
const SAME_ORIGIN = { credentials: "same-origin", cache: "no-store" };

const SPEED_WORDS = "Only shown to you here. Built from public GitHub activity, so not secret; we don't publish or rank it.";

// What the service said about this browser: "unknown" (not asked yet),
// "checking", "none" (no sign-in here; `why` says so when it is not plain
// absence), "out", or "in" with `me` = {login, optedOut}.
let account = { state: "unknown" };
// The speed answer for whoever is signed in: "unknown", "loading", "ok" with
// `speed`, "absent" (the service has no speed yet), or "error" with `why`.
let speed = { state: "unknown" };
// What the last action did, said once at the top of Me until the next navigation.
let notice = null;
// Answers that arrive after a newer question was asked are dropped.
let asked = 0;

/** Forget the one-time notice when the reader moves to another view. */
export function navigated() {
  notice = null;
}

async function checkSession(rerender) {
  const ticket = ++asked;
  account = { state: "checking" };
  speed = { state: "unknown" };
  let next;
  try {
    const res = await fetch("api/v1/me", SAME_ORIGIN);
    if (res.status === 200) next = readMe(await res.json());
    else if (res.status === 401) next = { state: "out" };
    else if (res.status === 404) next = { state: "none" };
    else next = { state: "none", why: `Sign-in could not be checked (the server answered ${res.status}), so this is the picker kept in this browser.` };
  } catch {
    next = { state: "none", why: "Sign-in could not be checked (no answer, or not one this page understands), so this is the picker kept in this browser." };
  }
  if (ticket !== asked) return;
  account = next;
  rerender();
}

/** The signed-in person, or "none" when the answer is not one this page can trust. */
function readMe(raw) {
  const o = raw && typeof raw === "object" ? raw : {};
  if (typeof o.login !== "string" || !LOGIN_RE.test(o.login) || !Number.isSafeInteger(o.id) || o.id < 1) {
    return { state: "none", why: "Sign-in answered in a form this page does not know, so this is the picker kept in this browser." };
  }
  return { state: "in", me: { login: o.login, optedOut: o.opted_out === true } };
}

// A speed answer that arrives after the session changed is dropped; the
// change reset `speed`, so the next signed-in render asks again.
async function loadSpeed(rerender) {
  const ticket = asked;
  speed = { state: "loading" };
  let next;
  try {
    const res = await fetch("api/v1/me/speed", SAME_ORIGIN);
    if (res.status === 200) next = { state: "ok", speed: readSpeed(await res.json()) };
    else if (res.status === 401) next = { state: "ended" };
    else if (res.status === 404) next = { state: "absent" };
    else next = { state: "error", why: `Your speed could not be loaded (the server answered ${res.status}).` };
  } catch {
    next = { state: "error", why: "Your speed could not be loaded (no answer, or not one this page understands)." };
  }
  if (ticket !== asked || account.state !== "in") return;
  if (next.state === "ended") {
    signedOutBy("Your sign-in has ended. Sign in again to see your speed.");
  } else {
    speed = next;
    if (next.state === "ok" && next.speed.optedOut) account.me.optedOut = true;
  }
  rerender();
}

function signedOutBy(message) {
  asked++;
  account = { state: "out" };
  speed = { state: "unknown" };
  notice = message;
}

// The three changes Me can make, each a fixed path and method. Each answers
// 204; a 401 means the session had already ended.
const ACTIONS = {
  logout: {
    request: () => fetch("auth/logout", { method: "POST", ...SAME_ORIGIN }),
    done: () => signedOutBy("You are signed out."),
  },
  optOut: {
    request: () => fetch("api/v1/me/opt-out", { method: "POST", ...SAME_ORIGIN }),
    done: () => {
      if (account.state === "in") account.me.optedOut = true;
      speed = { state: "unknown" };
      notice = "You have opted out. Your speed inputs are deleted and no longer recorded.";
    },
  },
  remove: {
    request: () => fetch("api/v1/me", { method: "DELETE", ...SAME_ORIGIN }),
    done: () => signedOutBy("Your data is deleted and you are signed out, in every browser."),
  },
};

/**
 * Run an action; null when it went through, else what went wrong. On
 * success the page re-renders and focus moves to the notice saying what
 * happened, as the control that was used is gone.
 */
async function run(key, rerender) {
  let res;
  try {
    res = await ACTIONS[key].request();
  } catch {
    return "No answer from the server; nothing changed. Try again.";
  }
  if (res.ok) {
    ACTIONS[key].done();
  } else if (res.status === 401) {
    signedOutBy("Your sign-in had already ended. Sign in again to do that.");
  } else {
    return `That did not go through (the server answered ${res.status}). Try again later.`;
  }
  rerender();
  document.getElementById("me-notice")?.focus();
  return null;
}

/**
 * A button that asks before it acts: a click shows what will happen with
 * a confirm and a cancel button, and Escape or Cancel goes back. While the
 * request runs the buttons are gone; a failure is said where the button was.
 */
function confirmButton({ id, label, question, yes, working, action, rerender, danger = false }) {
  const box = h("span", { class: "confirm" });
  const cls = `action${danger ? " action-danger" : ""}`;
  const idle = (error) => {
    const button = h("button", { type: "button", class: cls, id, on: { click: ask } }, label);
    box.replaceChildren(button, ...(error ? [h("span", { class: "action-error", role: "alert" }, error)] : []));
    return button;
  };
  const ask = () => {
    const confirm = h("button", { type: "button", class: cls, id: `${id}-yes`, "aria-describedby": `${id}-q`, on: { click: go } }, yes);
    box.replaceChildren(
      h("span", { class: "confirm-q", id: `${id}-q` }, question),
      confirm,
      h("button", { type: "button", class: "action", on: { click: () => idle().focus() } }, "Cancel"));
    confirm.focus();
  };
  const go = async () => {
    // Focus stays in the box while the request runs.
    const wait = h("span", { class: "muted", role: "status", tabindex: "-1" }, working);
    box.replaceChildren(wait);
    wait.focus();
    const error = await run(action, rerender);
    if (error && box.isConnected) idle(error).focus();
  };
  box.addEventListener("keydown", (e) => {
    if (e.key === "Escape" && box.querySelector(`#${id}-yes`)) idle().focus();
  });
  idle();
  return box;
}

/** Me: the picker where there is no sign-in, else signed out or in. */
export function meRoute(data, route, rerender, picker) {
  if (account.state === "unknown") checkSession(rerender);
  const top = [];
  if (notice) top.push(h("p", { class: "notice", id: "me-notice", role: "status", tabindex: "-1" }, notice));
  if (route.signinFailed && (account.state === "out" || account.state === "in")) {
    top.push(h("p", { class: "notice", role: "status" },
      "Signing in with GitHub did not finish, so nothing changed. You can try again. ",
      h("a", { href: "#/me" }, "Dismiss")));
  }
  switch (account.state) {
    case "in": return [...top, ...signedIn(data, account.me, rerender)];
    case "out": return [...top, ...signedOut(data)];
    case "none": return [...top, account.why ? h("p", { class: "notice" }, account.why) : null, ...picker()];
    default: return [section("Me", h("p", { class: "muted", role: "status" }, "Checking whether you are signed in…"))];
  }
}

function signedOut(data) {
  const go = (p) => {
    location.hash = personHref(p.login);
  };
  return [
    h("section", { class: "card" },
      h("h2", {}, "Sign in to see your own page"),
      h("p", {}, "The reviews you owe, your PRs, and your own speed, which only you see."),
      h("p", {}, h("a", { class: "button", href: "auth/login" }, "Sign in with GitHub")),
      h("p", { class: "sub" },
        "Signing in reads only your public GitHub profile: your user id and login. No email, and no access to any repository. ",
        h("a", { href: "#/privacy" }, "Privacy"))),
    h("section", { class: "card" },
      h("h2", {}, "Look someone up"),
      h("p", { class: "sub" }, "Everyone's page here is public and needs no sign-in. This search is not remembered."),
      loginSearch(data, "me-q", "a GitHub login", go, { links: true })),
  ];
}

function signedIn(data, me, rerender) {
  const note = h("div", { class: "sub me-note" }, "Signed in with GitHub. ",
    confirmButton({
      id: "act-logout", label: "Sign out", question: "Sign out of this browser?", yes: "Sign out",
      working: "Signing out…", action: "logout", rerender,
    }));
  const person = findPerson(data, me.login);
  const page = person
    ? personView(data, person.login, { isMe: true, note })
    : [h("section", { class: "card person-head" },
      h("div", { class: "person-title" }, avatar(me.login), h("h2", { id: "person-name", tabindex: "-1" }, me.login)),
      note,
      h("p", {}, "Nothing of yours is open in this data: no reviews owed and no open PRs."))];
  return [...page, speedSection(me, rerender)];
}

function speedSection(me, rerender) {
  let body;
  if (me.optedOut) {
    body = [h("p", {}, "You have opted out: no speed is computed for you, and what it was built from is deleted. ",
      "The public queue, a mirror of GitHub, is unchanged.")];
  } else {
    if (speed.state === "unknown") loadSpeed(rerender);
    if (speed.state === "ok" && !speed.speed.optedOut) body = speedBody(speed.speed);
    else if (speed.state === "absent") body = [h("p", { class: "muted" }, "This server does not compute speed yet.")];
    else if (speed.state === "error") {
      body = [h("p", {}, speed.why, " ",
        h("button", { type: "button", class: "linkish", on: { click: () => { speed = { state: "unknown" }; rerender(); } } }, "Try again"))];
    } else body = [h("p", { class: "muted", role: "status" }, "Loading your speed…")];
  }
  const actions = [];
  if (!me.optedOut) {
    actions.push(confirmButton({
      id: "act-optout", label: "Opt out",
      question: "Opt out? Your speed inputs are deleted and no longer recorded; there is no button to undo it. The public queue is unchanged.",
      yes: "Opt out", working: "Opting out…", action: "optOut", rerender,
    }));
  }
  actions.push(confirmButton({
    id: "act-delete", label: "Delete my data", danger: true,
    question: "Delete your data? This signs you out in every browser and deletes your speed inputs. An opt-out is kept, so it is still honoured.",
    yes: "Delete", working: "Deleting…", action: "remove", rerender,
  }));
  return h("section", { class: "card speed" },
    h("h2", {}, "Your speed"),
    h("p", { class: "sub" }, SPEED_WORDS),
    body,
    h("div", { class: "account" },
      h("h3", {}, "Your data"),
      h("p", { class: "sub" }, me.optedOut
        ? "Deleting your data also signs you out, in every browser. "
        : "Opting out stops your speed for good and keeps you signed in; deleting your data also signs you out. ",
      h("a", { href: "#/privacy" }, "What is kept, and for how long")),
      h("div", { class: "actions" }, actions)));
}
