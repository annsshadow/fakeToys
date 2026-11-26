#!/bin/bash
cd "$(git rev-parse --show-toplevel)" || exit 1
D=docs/laiyipao/audits/round35-postdeploy
BAD=$(grep -RInE 'Bearer[[:space:]]+eyJ|eyJ[A-Za-z0-9_-]{10,}|[A-Za-z]:[/\\](Users|WORKSPACE|home)' "$D" --exclude-dir=data 2>/dev/null || true)
echo BAD_COUNT=$(echo -n "$BAD" | grep -c . || true)
for f in "$D"/*.sh; do bash -n "$f" || exit 1; done
echo SYNTAX_RC=0
