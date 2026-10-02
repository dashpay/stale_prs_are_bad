// The only way this page builds DOM. Every string from dashboard.json is
// attacker-controlled (PR titles, blockers, areas, logins), so text always
// goes in through text nodes and attributes pass an allowlist. Links and
// image sources are accepted only in the shapes this page builds itself:
// a route in the hash, a PR on GitHub, or the service's sign-in, a fixed path
// beside the page.

const SAFE_ATTRS = new Set([
  "class", "id", "title", "alt", "role", "tabindex", "scope", "colspan",
  "type", "name", "value", "for", "placeholder", "autocomplete", "spellcheck",
  "width", "height", "loading", "decoding", "referrerpolicy", "rel", "target",
  "hidden", "checked", "selected", "disabled", "lang", "datetime",
]);

const SAFE_HREF = /^(#[^\s]*|auth\/login|https:\/\/github\.com\/[A-Za-z0-9_.-]+\/[A-Za-z0-9_.-]+\/pull\/[1-9][0-9]*)$/;
const SAFE_SRC = /^https:\/\/avatars\.githubusercontent\.com\/[A-Za-z0-9-]{1,39}\?s=40$/;

function setAttr(el, name, value) {
  if (value === false || value === null || value === undefined) return;
  const v = value === true ? "" : String(value);
  if (name === "href") {
    if (!SAFE_HREF.test(v)) throw new Error("refusing an href this page did not build");
  } else if (name === "src") {
    if (!SAFE_SRC.test(v)) throw new Error("refusing an image source this page did not build");
  } else if (!SAFE_ATTRS.has(name) && !/^(aria|data)-[a-z-]+$/.test(name)) {
    throw new Error(`attribute not allowed: ${name}`);
  }
  el.setAttribute(name, v); // vetted-sink: name allowlisted, href/src validated above
}

function append(el, child) {
  if (child === null || child === undefined || child === false) return;
  if (Array.isArray(child)) {
    for (const c of child) append(el, c);
  } else if (child instanceof Node) {
    el.appendChild(child);
  } else {
    el.appendChild(document.createTextNode(String(child)));
  }
}

/**
 * h("a", {href: "#/people", class: "x", on: {click: fn}}, "text", node, [more])
 * Strings become text nodes, never markup.
 */
export function h(tag, attrs, ...children) {
  const el = document.createElement(tag);
  for (const [name, value] of Object.entries(attrs || {})) {
    if (name === "on") {
      for (const [ev, fn] of Object.entries(value)) el.addEventListener(ev, fn);
    } else {
      setAttr(el, name, value);
    }
  }
  append(el, children);
  return el;
}

const SVG_NS = "http://www.w3.org/2000/svg";
const SVG_TAGS = new Set(["svg", "circle", "polygon"]);
const SVG_ATTRS = new Set(["class", "width", "height", "viewBox", "aria-hidden", "cx", "cy", "r", "points"]);

/** A small SVG element for legend keys and inline marks: known shapes and geometry only. */
export function svg(tag, attrs, ...children) {
  if (!SVG_TAGS.has(tag)) throw new Error(`svg element not allowed: ${tag}`);
  const el = document.createElementNS(SVG_NS, tag);
  for (const [name, value] of Object.entries(attrs || {})) {
    if (!SVG_ATTRS.has(name)) throw new Error(`svg attribute not allowed: ${name}`);
    el.setAttribute(name, String(value)); // vetted-sink: name allowlisted above
  }
  for (const c of children) el.appendChild(c);
  return el;
}

const PR_URL = /^https:\/\/github\.com\/[A-Za-z0-9_.-]+\/[A-Za-z0-9_.-]+\/pull\/[1-9][0-9]*$/;

/** Open a PR's GitHub page in a new tab; nothing but a PR link this page built. */
export function openPr(url) {
  if (!PR_URL.test(url)) throw new Error("refusing to open a link this page did not build");
  window.open(url, "_blank", "noopener,noreferrer"); // vetted-sink: URL validated above
}
