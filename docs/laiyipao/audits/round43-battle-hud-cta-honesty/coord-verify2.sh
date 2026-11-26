#!/usr/bin/env bash
set -u
ROOT=$(git rev-parse --show-toplevel)
D="$ROOT/docs/laiyipao/audits/round43-battle-hud-cta-honesty"
AB=$(command -v agent-browser)
S=lyp-r43-coord2
P="$D/data"
X="$D/screenshots"
"$AB" --session "$S" --init-script "$P/auth-init.js" open "http://127.0.0.1:5174/#/pages/battle/battle?level=5" > "$P/v2-open.txt" 2>&1
"$AB" --session "$S" set viewport 390 844 >> "$P/v2-open.txt" 2>&1
sleep 6
click_at() { local px="$1" py="$2" out="$3"; { echo "MOVE=$px,$py"; "$AB" --session "$S" mouse move "$px" "$py"; "$AB" --session "$S" mouse down left; "$AB" --session "$S" mouse up left; } > "$P/$out" 2>&1; }
num() { node "$P/one-num.js" "$ROOT/docs/laiyipao/audits/round43-battle-hud-cta-honesty" "$1" "$2" "$3"; }
for stage in r27 r24; do
  "$AB" --session "$S" eval --stdin < "$P/coord-probe.js" > "$P/v2-${stage}-probe.json" 2>&1
  "$AB" --session "$S" eval --stdin < "$P/coord-state.js" > "$P/v2-${stage}-before.json" 2>&1
  px=$(num "v2-${stage}-probe.json" "$stage" cx); py=$(num "v2-${stage}-probe.json" "$stage" cy)
  click_at "$px" "$py" "v2-${stage}-click.txt"
  sleep 1
  "$AB" --session "$S" eval --stdin < "$P/coord-state.js" > "$P/v2-${stage}-after.json" 2>&1
  "$AB" --session "$S" screenshot "$X/r43-v2-${stage}-after.png" > "$P/v2-${stage}-shot.txt" 2>&1
done
"$AB" --session "$S" eval --stdin < "$P/coord-state.js" > "$P/v2-help-before.json" 2>&1
"$AB" --session "$S" eval --stdin < "$P/coord-probe.js" > "$P/v2-help-probe.json" 2>&1
px=$(num v2-help-probe.json help cx); py=$(num v2-help-probe.json help cy)
click_at "$px" "$py" v2-help-click.txt
sleep 2
"$AB" --session "$S" eval --stdin < "$P/coord-state.js" > "$P/v2-help-open.json" 2>&1
"$AB" --session "$S" screenshot "$X/r43-v2-help-open.png" > "$P/v2-help-shot.txt" 2>&1
"$AB" --session "$S" eval --stdin < "$P/coord-probe.js" > "$P/v2-close-probe.json" 2>&1
px=$(num v2-close-probe.json legend cx); py=$(num v2-close-probe.json legend cy)
click_at "$px" "$py" v2-close-click.txt
sleep 1
"$AB" --session "$S" eval --stdin < "$P/coord-state.js" > "$P/v2-help-closed.json" 2>&1
"$AB" --session "$S" screenshot "$X/r43-v2-help-closed.png" > "$P/v2-closed-shot.txt" 2>&1
echo V2_DONE