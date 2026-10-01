#!/usr/bin/env bash
# Fails when the page's own scripts use an API that parses a string as HTML or
# code, when the HTML carries inline script, or when a vendored library no
# longer matches its recorded checksum. Everything this page renders comes from
# PR titles, blockers and areas anyone can write, so text goes in through
# textContent and createElement only, and links through js/dom.js's checks.
set -uo pipefail
cd "$(dirname "$0")" || exit 2
fail=0

js_pattern='innerHTML|outerHTML|insertAdjacentHTML|document\.write|\beval\b|\bFunction[[:space:]]*\(|setHTMLUnsafe|parseHTMLUnsafe|createContextualFragment|parseFromString|srcdoc|\.html[[:space:]]*\(|set(Timeout|Interval)[[:space:]]*\([[:space:]]*["'"'"'`]|\[[[:space:]]*["'"'"'`](inner|outer)|\.(href|src)[[:space:]]*=[^=]|setAttribute(NS)?\([^)]*["'"'"'`](href|src|style|on[a-z]+)["'"'"'`]'

files=()
while IFS= read -r f; do files+=("$f"); done < <(find . \( -name '*.js' -o -name '*.mjs' -o -name '*.cjs' \) -not -path './vendor/*' | sort)
if [ "${#files[@]}" -eq 0 ]; then
  echo "check.sh: no scripts found under $(pwd)" >&2
  exit 2
fi

# Links and image sources are set only through js/dom.js, which validates them.
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
hits=$(grep -nE "$js_pattern" "${files[@]}")
report "HTML, code or a URL taken from a string; use textContent / createElement / js/dom.js instead" $? "$hits"

# No inline script or event-handler attributes; the CSP forbids them too.
hits=$(grep -nEi '<script([[:space:]][^>]*)?>[[:space:]]*[^<[:space:]]|<script>[[:space:]]*$|[[:space:]]on[a-z]+[[:space:]]*=' ./*.html)
report "inline script in HTML" $? "$hits"

# Vendored files must match the checksums in vendor/VERSIONS.
while read -r file _pkg _ver _lic sum; do
  case "$file" in ''|'#'*) continue ;; esac
  actual=$(shasum -a 256 "vendor/$file" 2>/dev/null | cut -d' ' -f1)
  if [ "$actual" != "$sum" ]; then
    echo "check.sh: vendor/$file does not match its sha256 in vendor/VERSIONS" >&2
    fail=1
  fi
done < vendor/VERSIONS

[ "$fail" -eq 0 ] && echo "check.sh: ok (${#files[@]} scripts, no HTML or code from strings; vendor checksums match)"
exit "$fail"
