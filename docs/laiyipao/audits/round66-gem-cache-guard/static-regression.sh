#!/usr/bin/env bash
set -uo pipefail
ROOT="$(pwd)"
M="$ROOT/laiyipao/miniapp/src"
echo "=== TOKEN ANCHORS ==="
grep -n -- '--lyp-token-version: lyp-tokens-v1\|--lyp-r3-version: lyp-tokens-v3-r3\|--lyp-r6-version: lyp-tokens-r6' "$M/styles/tokens.css"
echo "=== R66 TOKEN COUNT (business should be 0) ==="
grep -rn -- '--r66-' "$M" --include='*.vue' --include='*.ts' --include='*.css' | grep -v 'contract.test.ts' | wc -l
echo "=== R60 keyboard tabindex/keydown counts ==="
grep -rn 'tabindex="0"' "$M/pages" --include='*.vue' | wc -l
grep -rn '@keydown.enter' "$M/pages" --include='*.vue' | wc -l
echo "=== R55 placeholder-class=ph count ==="
grep -rn 'placeholder-class="ph"' "$M" --include='*.vue' | wc -l
echo "=== R61 v-aria-native count ==="
grep -rn 'v-aria-native' "$M/pages" --include='*.vue' | wc -l
echo "=== R10 gems CTA anchors ==="
grep -n '去背包镶嵌\|去商城找宝石\|服务端属性差分\|r10GemGainCacheKey' "$M/pages/gems/gems.vue"
echo "=== R64 battles race guards (should be untouched this round) ==="
grep -n 'detailPanelSeq' "$M/pages/battles/battles.vue" | head -10
echo "=== DIFF FILES THIS ROUND ==="
cd "$ROOT" && git diff --name-only
echo STATIC_DONE
