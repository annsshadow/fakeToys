#!/usr/bin/env bash
# Adds SPDX AGPL-3.0 license headers to source files across the monorepo.
# Idempotent: files that already contain an SPDX identifier are skipped.
set -euo pipefail

cd "$(dirname "$0")/.."

YEAR=2026
HOLDER="annsshadow"
LINE1="Copyright (C) ${YEAR} ${HOLDER}"
LINE2="SPDX-License-Identifier: AGPL-3.0-or-later"

added=0
skipped=0

has_header() { grep -q 'SPDX-License-Identifier' "$1"; }

is_generated() {
  head -20 "$1" | grep -qiE '@generated|AUTO-?GENERATED|This file was generated|DO NOT EDIT'
}

# Prepend a block, preserving the file's trailing newline behaviour.
prepend() {
  local file="$1" block="$2"
  local tmp; tmp="$(mktemp)"
  printf '%s' "$block" > "$tmp"
  cat "$file" >> "$tmp"
  mv "$tmp" "$file"
}

process() {
  local file="$1" style="$2"
  if has_header "$file"; then skipped=$((skipped+1)); return; fi
  if is_generated "$file"; then skipped=$((skipped+1)); return; fi

  case "$style" in
    slash)  block="// ${LINE1}"$'\n'"// ${LINE2}"$'\n\n' ;;
    hash)   block="# ${LINE1}"$'\n'"# ${LINE2}"$'\n\n' ;;
    vue)    block="<!-- ${LINE1} -->"$'\n'"<!-- ${LINE2} -->"$'\n\n' ;;
  esac
  prepend "$file" "$block"
  added=$((added+1))
}

# Rust
while IFS= read -r f; do process "$f" slash; done < <(
  find oa4rust -path '*/target' -prune -o -path '*/target-ci' -prune -o -name '*.rs' -type f -print)

# Vue
while IFS= read -r f; do process "$f" vue; done < <(
  find oa4rust-web -path '*/node_modules' -prune -o -path '*/dist' -prune -o -name '*.vue' -type f -print)

# TypeScript (web + auto-checkin)
while IFS= read -r f; do process "$f" slash; done < <(
  find oa4rust-web auto-checkin -path '*/node_modules' -prune -o -path '*/dist' -prune -o -name '*.ts' -type f -print)

# Python
while IFS= read -r f; do process "$f" hash; done < <(
  find augmentor -path '*/__pycache__' -prune -o -path '*/.pytest_cache' -prune -o \
    -path '*/node_modules' -prune -o -path '*/bak' -prune -o -path '*/archive' -prune \
    -o -name '*.py' -type f -print)

# Java
while IFS= read -r f; do process "$f" slash; done < <(
  find cool -name '*.java' -type f -print)

echo "added=${added} skipped=${skipped}"
