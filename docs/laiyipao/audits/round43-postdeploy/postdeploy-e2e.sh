#!/usr/bin/env bash
set -u
ROOT=$(git rev-parse --show-toplevel)
SRC="$ROOT/docs/laiyipao/audits/round43-battle-hud-cta-honesty/data"
D="$ROOT/docs/laiyipao/audits/round43-postdeploy"
AB=$(command -v agent-browser)
S=lyp-r43-pd3
P="$D/data"; X="$D/screenshots"
mkdir -p "$P" "$X"
open_url="http://127.0.0.1:5274/#/pages/battle/battle?level=5"
"$AB" --session "$S" --init-script "$P/auth-init.js" open "$open_url" > "$P/pd-open.txt" 2>&1
"$AB" --session "$S" set viewport 390 844 >> "$P/pd-open.txt" 2>&1
sleep 6
num() { node "$SRC/one-num.js" "$D" "$1" "$2" "$3"; }
click_at() { local px="$1" py="$2" out="$3"; { echo "MOVE=$px,$py"; "$AB" --session "$S" mouse move "$px" "$py"; "$AB" --session "$S" mouse down left; "$AB" --session "$S" mouse up left; } > "$P/$out" 2>&1; }
for stage in r27 r24; do
  "$AB" --session "$S" eval --stdin < "$SRC/coord-probe.js" > "$P/pd-${stage}-probe.json" 2>&1
  "$AB" --session "$S" eval --stdin < "$SRC/coord-state.js" > "$P/pd-${stage}-before.json" 2>&1
  px=$(num "pd-${stage}-probe.json" "$stage" cx); py=$(num "pd-${stage}-probe.json" "$stage" cy)
  click_at "$px" "$py" "pd-${stage}-click.txt"
  sleep 1
  "$AB" --session "$S" eval --stdin < "$SRC/coord-state.js" > "$P/pd-${stage}-after.json" 2>&1
  "$AB" --session "$S" screenshot "$X/pd-${stage}-after.png" > "$P/pd-${stage}-shot.txt" 2>&1
done
"$AB" --session "$S" eval --stdin < "$SRC/coord-state.js" > "$P/pd-help-before.json" 2>&1
"$AB" --session "$S" click .help-chip > "$P/pd-help-click.txt" 2>&1
sleep 2
"$AB" --session "$S" eval --stdin < "$SRC/coord-state.js" > "$P/pd-help-open.json" 2>&1
"$AB" --session "$S" screenshot "$X/pd-help-open.png" > "$P/pd-help-shot.txt" 2>&1
"$AB" --session "$S" click .legend-close > "$P/pd-help-close.txt" 2>&1
sleep 1
"$AB" --session "$S" eval --stdin < "$SRC/coord-state.js" > "$P/pd-help-closed.json" 2>&1
"$AB" --session "$S" screenshot "$X/pd-help-closed.png" > "$P/pd-help-closed-shot.txt" 2>&1
"$AB" --session "$S" eval --stdin < "$SRC/battle-probe.js" > "$P/pd-coexist.json" 2>&1
"$AB" --session "$S" screenshot "$X/pd-coexist.png" > "$P/pd-coexist-shot.txt" 2>&1
echo PD_E2E_DONE
