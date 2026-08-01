#!/bin/bash
# Mechanizes CHANGELOG.md's footer-link discipline: every `## [X.Y.Z]` version heading must have
# a matching `[X.Y.Z]: <url>` footer link, so a reader (or a CI job) can always click through from
# a version to its release. Modeled on pacta's own `pacta-governance` changelog-footer-link
# reaction, kept dependency-free here since this template has no governance crate of its own yet.
set -euo pipefail

file="CHANGELOG.md"
if [ ! -f "$file" ]; then
  echo "changelog-guard: no CHANGELOG.md found, nothing to check"
  exit 0
fi

headings=$(grep -oE '^## \[[0-9]+\.[0-9]+\.[0-9]+\]' "$file" | sed -E 's/^## \[(.*)\]$/\1/' || true)

if [ -z "$headings" ]; then
  echo "changelog-guard: no version headings yet, nothing to check"
  exit 0
fi

missing=0
while IFS= read -r version; do
  if ! grep -qE "^\[${version//./\\.}\]: " "$file"; then
    echo "changelog-guard: version heading [$version] has no matching footer link"
    missing=1
  fi
done <<< "$headings"

if [ "$missing" -ne 0 ]; then
  echo "changelog-guard: FAILED — add a [X.Y.Z]: <url> footer line for every version heading"
  exit 1
fi

echo "changelog-guard: clean — every version heading has a matching footer link"
