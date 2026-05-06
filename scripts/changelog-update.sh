#!/usr/bin/env bash
# Usage: scripts/changelog-update.sh <version-tag>
# cargo-release pre-release-hook. Dry mode (DRY_RUN=true): print
# preview, leave file untouched. Execute: prepend new entry below
# the existing "# Changelog" header, keeping historic entries verbatim.

set -euo pipefail
cd "$(git rev-parse --show-toplevel)"

version="${1:?usage: changelog-update.sh <version-tag, e.g. v0.6.0>}"
new_entry=$(git cliff --tag "$version" --unreleased --strip header)

if [[ "${DRY_RUN:-true}" == "true" ]]; then
    cat <<EOF
=== changelog-update.sh: DRY_RUN — CHANGELOG.md not modified ===
=== Preview of the entry that would be inserted for $version ===
$new_entry
=== end of preview ===
EOF
    exit 0
fi

first_entry_line=$(grep -n -m1 -E '^(### \[|## [0-9])' CHANGELOG.md | cut -d: -f1 || true)

mkdir -p .tmp
tmpfile=$(mktemp .tmp/CHANGELOG.update.XXXXXX)
trap 'rm -f "$tmpfile"' EXIT

if [[ -z "$first_entry_line" ]]; then
    cat CHANGELOG.md > "$tmpfile"
    printf '%s\n' "$new_entry" >> "$tmpfile"
else
    head -n $((first_entry_line - 1)) CHANGELOG.md > "$tmpfile"
    printf '%s\n\n' "$new_entry" >> "$tmpfile"
    tail -n +"$first_entry_line" CHANGELOG.md >> "$tmpfile"
fi

mv "$tmpfile" CHANGELOG.md
trap - EXIT
