// What this page and the service behind it know about you, where it comes
// from, and how long it is kept. The same page is served where there is no
// sign-in (GitHub Pages), so every sentence about sign-in says "if".

import { h } from "./dom.js";

function part(title, ...children) {
  return h("section", {}, h("h3", {}, title), children);
}

export function privacyView() {
  return [h("section", { class: "card privacy" },
    h("h2", { id: "privacy", tabindex: "-1" }, "Privacy"),
    part("Where the data comes from",
      h("p", {}, "Everything here is read from public GitHub activity: pull requests, reviews and the review engine's ",
        "public status. None of it comes from you, and nothing here asks you for anything.")),
    part("Signing in",
      h("p", {}, "If you sign in with GitHub, the sign-in reads only your public profile, once: your GitHub user id and ",
        "login. No email is collected. The GitHub token it gets is revoked right after sign-in (should GitHub refuse, ",
        "the token expires on its own) and is never stored. The GitHub App behind it has no permissions at all.")),
    part("Speed inputs",
      h("p", {}, "Your speed is built from speed inputs: when you were asked to review a PR and how that ended, your ",
        "approvals and change requests, and when your PRs were ready for review and merged, keyed by your GitHub user id. ",
        "They are recorded from the public activity for everyone in it, whether or not they sign in, unless they opt out.")),
    part("What is kept, and for how long",
      h("ul", {},
        h("li", {}, "Sessions: 30 days from sign-in. A session holds your user id and login only. Signing out ends it at once."),
        h("li", {}, "Speed inputs, and the history of each PR's stages: 13 months."),
        h("li", {}, "Backups: no longer than that."),
        h("li", {}, "An opt-out: your user id and when you opted out, kept so it is honoured, even after you delete your data."),
        h("li", {}, "Request logs: the method, path, status and time taken; never your address or a cookie."))),
    part("Your speed",
      h("p", {}, "Your speed is shown only to you, on Me, while you are signed in. It is built from public GitHub activity, ",
        "so it is not secret, but it is not published or ranked.")),
    part("Opting out and deleting your data",
      h("p", {}, "On Me, while signed in, you can opt out: your speed inputs are deleted and no longer recorded, and you stay ",
        "signed in. Your PRs and reviews still show on Team and People: that public queue is a mirror of GitHub and is ",
        "unchanged. This page has no way to undo an opt-out."),
      h("p", {}, "You can also delete your data: every session of yours, in every browser, and your speed inputs. Unless ",
        "you opted out, speed inputs are recorded again from your next activity; if you opted out, the opt-out is kept ",
        "so it is still honoured.")),
    part("In your browser",
      h("p", {}, "Signing in sets two cookies that scripts on the page cannot read: one for the sign-in itself, kept 10 ",
        "minutes, and the session. Where there is no sign-in, Me can remember a login you pick, in this browser only. ",
        "Avatars load from GitHub with no referrer. There are no analytics."))),
  ];
}
