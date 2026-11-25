#!/usr/bin/env bash
set -euo pipefail
ROOT=$(git rev-parse --show-toplevel)
D="$ROOT/docs/laiyipao/audits/round44-r22-cta-honesty"
P="$D/data"; X="$D/screenshots"
AB=$(command -v agent-browser)
S=lyp-r44-alias
open_url="http://127.0.0.1:5174/oa4rust-h5/#/pages/battle/battle?level=5"
mkdir -p "$P" "$X"
"$AB" --session "$S" --init-script "$P/auth-init.js" open "$open_url" > "$P/alias-open.txt" 2>&1
"$AB" --session "$S" set viewport 390 844 > "$P/alias-viewport.txt" 2>&1
"$AB" --session "$S" eval "$(cat "$P/card-state.js")" > "$P/alias-initial.json" 2>&1
"$AB" --session "$S" screenshot "$X/r44-alias-initial.png" > "$P/alias-initial-shot.txt" 2>&1
# 真实等待波次结束进入选牌面板；若 8× 未生效则最多等 90s 并如实记录。
"$AB" --session "$S" wait --fn "() => !!document.querySelector('.sheet') && document.querySelectorAll('.hand-card').length > 0" --timeout 90000 > "$P/alias-wait-card.txt" 2>&1 || true
"$AB" --session "$S" eval "$(cat "$P/dialog-probe.js")" > "$P/alias-card.json" 2>&1
"$AB" --session "$S" screenshot "$X/r44-alias-card.png" > "$P/alias-card-shot.txt" 2>&1
# 记录 R43 r27/r24 只读 CTA 在战斗中的状态（card_select 时不渲染，回到战斗后复查）。
"$AB" --session "$S" eval "$(cat "$P/card-state.js")" > "$P/alias-card2.json" 2>&1
echo ALIAS_DRIVER_DONE
