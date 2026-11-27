#!/usr/bin/env bash
set -u
ROOT="D:/WORKSPACE/fakeToys/.agents/worktrees/dsh-d0bc4721-3248"
OUT="$ROOT/docs/laiyipao/audits/round98-form-enter/data/static-out.txt"
> "$OUT"
cd "$ROOT" || exit 1
echo '=== TOKENS ===' | tee -a "$OUT"
grep -nE 'lyp-token-version|lyp-r3-version|lyp-r6-version' laiyipao/miniapp/src/styles/tokens.css | tee -a "$OUT"
echo '=== CONFIRM POINTS ===' | tee -a "$OUT"
grep -nE '@confirm="(doRedeem|fetchReplay|doSavePreset)"|@click="(doRedeem|fetchReplay|doSavePreset)"' laiyipao/miniapp/src/pages/{signin/signin.vue,verify/verify.vue,bag/bag.vue} | tee -a "$OUT"
echo '=== PARADIGM ===' | tee -a "$OUT"
grep -nE '@confirm="submit"' laiyipao/miniapp/src/pages/login/login.vue | tee -a "$OUT"
grep -nE '@confirm="submitNickname"|@confirm="submitPassword"|@confirm="submitUpgrade"' laiyipao/miniapp/src/pages/me/me.vue | tee -a "$OUT"
echo '=== FETCHING GUARD ===' | tee -a "$OUT"
grep -nE 'if \(fetching\.value\) return|fetching\.value = true|fetching\.value = false' laiyipao/miniapp/src/pages/verify/verify.vue | tee -a "$OUT"
echo '=== ZERO r98 ===' | tee -a "$OUT"
grep -rn '\-\-r98-' laiyipao/miniapp/src/pages/ laiyipao/miniapp/src/styles/ 2>/dev/null | tee -a "$OUT"; echo "R98_COUNT=$(grep -rn '\-\-r98-' laiyipao/miniapp/src/pages/ laiyipao/miniapp/src/styles/ 2>/dev/null | wc -l)" | tee -a "$OUT"
echo '=== STATIC_DONE ===' | tee -a "$OUT"
