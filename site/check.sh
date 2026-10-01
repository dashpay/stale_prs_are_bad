#!/usr/bin/env bash
# Fails when the page could start treating data as markup, code or a link:
# its own scripts use an API that parses a string as HTML or code, open or
# assign a URL, or set an attribute outside js/dom.js's checks; the HTML loses
# its Content-Security-Policy or noindex, or carries inline script; or a
# vendored file is unlisted or no longer matches its recorded checksum.
# Everything the page renders comes from PR titles, blockers and areas anyone
# can write, so text goes in through textContent and createElement only.
set -uo pipefail
cd "$(dirname "$0")" || exit 2
fail=0

report() { # label, grep status, hits
  if [ "$2" -eq 0 ]; then
    echo "check.sh: $1:" >&2
    echo "$3" >&2
    fail=1
  elif [ "$2" -ne 1 ]; then
    echo "check.sh: grep failed (status $2)" >&2
    fail=1
  fi
}

q="[\"'\`]"
js_pattern="innerHTML|outerHTML|insertAdjacentHTML|document\\.write|\\beval\\b|\\bFunction[[:space:]]*\\(|setHTMLUnsafe|parseHTMLUnsafe|createContextualFragment|parseFromString|srcdoc"
js_pattern+="|\\.html[[:space:]]*\\(|\\.attr[[:space:]]*\\("                        # d3 sinks
js_pattern+="|set(Timeout|Interval)[[:space:]]*\\([[:space:]]*$q"                    # string timers
js_pattern+="|\\[[[:space:]]*$q(inner|outer)"                                         # el[\"innerHTML\"]
js_pattern+="|\\.(href|src)[[:space:]]*(=[^=]|=?[[:space:]]*$)"                       # .href = x, also split across lines
js_pattern+="|setAttribute(NS)?\\([^)]*$q(href|src|style|on[a-z]+)$q"                # literal dangerous names
js_pattern+="|setAttribute(NS)?\\([[:space:]]*([^\"'\`[:space:]]|$)"                 # a name not written literally
js_pattern+="|\\b(open|assign)[[:space:]]*\\("                                        # window.open, location.assign

files=()
while IFS= read -r f; do files+=("$f"); done < <(find . \( -name '*.js' -o -name '*.mjs' -o -name '*.cjs' \) -not -path './vendor/*' | sort)
if [ "${#files[@]}" -eq 0 ]; then
  echo "check.sh: no scripts found under $(pwd)" >&2
  exit 2
fi

# js/dom.js holds the few vetted sinks (allowlisted attribute names, validated
# links); each is marked on its own line and exempt only there.
hits=$(grep -nE "$js_pattern" "${files[@]}")
status=$?
if [ "$status" -eq 0 ]; then
  hits=$(printf '%s\n' "$hits" | grep -vE '^\./js/dom\.js:[0-9]+:.*// vetted-sink: ')
  [ -n "$hits" ] || status=1
fi
report "HTML, code or a URL taken from a string; use textContent / createElement / js/dom.js instead" "$status" "$hits"

# No inline script or event-handler attributes; the CSP forbids them too.
hits=$(grep -nEi '<script([[:space:]][^>]*)?>[[:space:]]*[^<[:space:]]|<script>[[:space:]]*$|[[:space:]]on[a-z]+[[:space:]]*=' ./*.html)
report "inline script in HTML" $? "$hits"

# The policy and noindex the page ships with, exactly.
csp='<meta http-equiv="Content-Security-Policy" content="default-src '"'none'"'; script-src '"'self'"'; style-src '"'self'"' '"'unsafe-inline'"'; img-src '"'self'"' https://avatars.githubusercontent.com; connect-src '"'self'"'; base-uri '"'none'"'; object-src '"'none'"'; form-action '"'none'"'; require-trusted-types-for '"'script'"'; trusted-types '"'none'"'">'
for want in "$csp" '<meta name="robots" content="noindex">'; do
  if ! grep -qxF "$want" index.html; then
    echo "check.sh: index.html lacks, or has changed: $want" >&2
    fail=1
  fi
done

# Vendored files: each one listed in vendor/VERSIONS, and matching its sha256.
listed=()
while read -r file _pkg _ver _lic sum; do
  case "$file" in ''|'#'*) continue ;; esac
  listed+=("$file")
  actual=$(shasum -a 256 "vendor/$file" 2>/dev/null | cut -d' ' -f1)
  if [ "$actual" != "$sum" ]; then
    echo "check.sh: vendor/$file does not match its sha256 in vendor/VERSIONS" >&2
    fail=1
  fi
done < vendor/VERSIONS
while IFS= read -r path; do
  name=${path#./vendor/}
  [ "$name" = VERSIONS ] && continue
  if ! printf '%s\n' "${listed[@]}" | grep -qxF "$name"; then
    echo "check.sh: vendor/$name is not listed in vendor/VERSIONS" >&2
    fail=1
  fi
done < <(find ./vendor -type f | sort)

[ "$fail" -eq 0 ] && echo "check.sh: ok (${#files[@]} scripts, no HTML, code or URLs from strings; CSP and noindex intact; vendor listed and checksums match)"
exit "$fail"
