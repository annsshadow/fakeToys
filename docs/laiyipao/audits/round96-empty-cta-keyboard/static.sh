#!/usr/bin/env bash
set -u
ROOT="$(pwd)"
OUT="$ROOT/docs/laiyipao/audits/round96-empty-cta-keyboard/data/static-out.txt"
{
 cd "$ROOT" || exit 1
 echo "=== 令牌三锚点 tokens.css:14-16 ==="
 sed -n "12,17p" laiyipao/miniapp/src/styles/tokens.css
 echo "=== 业务零--r96- (排除.test.) ==="
 grep -rn -- "--r96-" laiyipao/miniapp/src --include="*.vue" --include="*.css" --include="*.ts" | grep -v ".test." || echo "NO_MATCH"
 echo "=== 六页CTA四属性 ==="
 for f in bag/bag battles/battles enemies/enemies me/me rank/rank skins/skins; do
   echo "--- $f ---"
   grep -n "r3-empty-action" "laiyipao/miniapp/src/pages/$f.vue"
 done
 echo "=== 焦点环CSS ==="
 grep -n "focus-visible" laiyipao/miniapp/src/styles/common.css
 echo STATIC_ALL_DONE
} > "$OUT" 2>&1
echo STATIC_DONE