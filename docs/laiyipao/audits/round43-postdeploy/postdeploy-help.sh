#!/usr/bin/env bash
set -u
ROOT=$(git rev-parse --show-toplevel)
SRC="$ROOT/docs/laiyipao/audits/round43-battle-hud-cta-honesty/data"
D="$ROOT/docs/laiyipao/audits/round43-postdeploy"
AB=$(command -v agent-browser)
S=lyp-r43-pd4
P="$D/data"; X="$D/screenshots"
num() { node "$SRC/one-num.js" "$D" "$1" "$2" "$3"; }
hx=$(num "pd-help3-openprobe.json" help cx); hy=$(num "pd-help3-openprobe.json" help cy)
"$AB" --session "$S" mouse move "$hx" "$hy" > "$P/pd-help4-click.txt" 2>&1
"$AB" --session "$S" mouse down left >> "$P/pd-help4-click.txt" 2>&1
"$AB" --session "$S" mouse up left >> "$P/pd-help4-click.txt" 2>&1
sleep 2
"$AB" --session "$S" eval --stdin < "$SRC/coord-state.js" > "$P/pd-help4-open.json" 2>&1
"$AB" --session "$S" screenshot "$X/pd-help4-open.png" > "$P/pd-help4-shot.txt" 2>&1
"$AB" --session "$S" eval --stdin < "$SRC/coord-probe.js" > "$P/pd-help4-probe.json" 2>&1
echo HELP4_DONE HX=$hx HY=$hy
