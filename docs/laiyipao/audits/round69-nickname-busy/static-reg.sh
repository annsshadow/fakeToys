#!/usr/bin/env bash
set -uo pipefail
ROOT="$(pwd)"; M="$ROOT/laiyipao/miniapp/src"
echo "=== TOKENS ==="
grep -n -- "--lyp-token-version\|--lyp-r3-version\|--lyp-r6-version" "$M/styles/tokens.css" || true
echo "=== R69 TOKEN IN SRC (expect only contract) ==="
grep -rn -- "--r69-" "$M" | head -20 || true
echo "=== NICK INPUT A11Y ==="
grep -n "v-aria-native.*新昵称\|aria-label.*新昵称\|placeholder-class=\"ph\"" "$M/pages/me/me.vue" | head -10 || true
echo "=== SUBMIT PASSWORD ==="
grep -n "passwordSubmitting\|submitPassword" "$M/pages/me/me.vue" | head -12 || true
echo "=== CTA role=button count ==="
echo -n "role=button="; grep -rn -- "role=\"button\"" "$M/pages" | wc -l
echo -n "tabindex0="; grep -rn -- "tabindex=\"0\"" "$M/pages" | wc -l
echo -n "keydownenter="; grep -rn -- "@keydown.enter" "$M/pages" | wc -l
echo "=== DIFF NAMES ==="
cd "$ROOT" && git diff --name-only
echo STATIC_DONE
