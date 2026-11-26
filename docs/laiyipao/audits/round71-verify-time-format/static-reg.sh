#!/usr/bin/env bash
set -uo pipefail
ROOT="$(cd /d/WORKSPACE/fakeToys/.agents/worktrees/dsh-d0bc4721-3248 && pwd)"
cd "$ROOT"
echo TOKENS
grep -nE "--lyp-token-version:|--lyp-r3-version:|--lyp-r6-version:" laiyipao/miniapp/src/styles/tokens.css | grep -v "://"
echo R71_COUNT
grep -rn -- "--r71-" laiyipao/miniapp/src | wc -l
echo KBD
echo -n "tabindex=";grep -rho 'tabindex="0"' laiyipao/miniapp/src/pages | wc -l
echo -n "keydown=";grep -rho '@keydown.enter' laiyipao/miniapp/src/pages | wc -l
echo -n "ph=";grep -rho 'placeholder-class="ph"' laiyipao/miniapp/src/pages | wc -l
echo VERIFY_INV
grep -nE "r5AiVerify.conclusion|buildSnapshotCard|pretty\(replayInfo.expected_hash\)|v-aria-native|placeholder-class" laiyipao/miniapp/src/pages/verify/verify.vue
echo DIFF_NAMES
git diff --name-only HEAD 2>/dev/null
echo STAT_DONE