#!/usr/bin/env bash
set -u
ROOT=$(git rev-parse --show-toplevel)
D="$ROOT/docs/laiyipao/audits/round43-battle-hud-cta-honesty"
AB=$(command -v agent-browser)
S=lyp-r43-coord
"$AB" --session "$S" --init-script "$D/data/auth-init.js" open "http://127.0.0.1:5174/#/pages/battle/battle?level=5" > "$D/data/coord-open.txt" 2>&1
"$AB" --session "$S" set viewport 390 844 >> "$D/data/coord-open.txt" 2>&1
sleep 6
"$AB" --session "$S" eval --stdin < "$D/data/coord-probe.js" > "$D/data/coord-probe.json" 2>&1
read R27X R27Y R24X R24Y HELPX HELPY < <(node "$D/data/coord-parse.js" "$D/data/coord-probe.json" r27 r24 help)
for stage in r27 r24; do
  eval X=$${stage}X Y=$${stage}Y
  "$AB" --session "$S" eval --stdin < "$D/data/coord-state.js" > "$D/data/coord-${stage}-before.json" 2>&1
  { echo "MOVE=$X,$Y"; "$AB" --session "$S" mouse move "$X" "$Y"; "$AB" --session "$S" mouse down left; "$AB" --session "$S" mouse up left; } > "$D/data/coord-${stage}-click.txt" 2>&1
  sleep 1
  "$AB" --session "$S" eval --stdin < "$D/data/coord-state.js" > "$D/data/coord-${stage}-after.json" 2>&1
  "$AB" --session "$S" screenshot "$D/screenshots/r43-coord-${stage}-after.png" > "$D/data/coord-${stage}-shot.txt" 2>&1
done
"$AB" --session "$S" eval --stdin < "$D/data/coord-state.js" > "$D/data/coord-help-before.json" 2>&1
"$AB" --session "$S" eval --stdin < "$D/data/coord-probe.js" > "$D/data/coord-help-probe.json" 2>&1
read HELPX2 HELPY2 < <(node "$D/data/coord-parse.js" "$D/data/coord-help-probe.json" help)
{ echo "MOVE=$HELPX2,$HELPY2"; "$AB" --session "$S" mouse move "$HELPX2" "$HELPY2"; "$AB" --session "$S" mouse down left; "$AB" --session "$S" mouse up left; } > "$D/data/coord-help-click.txt" 2>&1
sleep 1
"$AB" --session "$S" eval --stdin < "$D/data/coord-state.js" > "$D/data/coord-help-open.json" 2>&1
"$AB" --session "$S" screenshot "$D/screenshots/r43-coord-help-open.png" > "$D/data/coord-help-shot.txt" 2>&1
"$AB" --session "$S" eval --stdin < "$D/data/coord-probe.js" > "$D/data/coord-close-probe.json" 2>&1
read CLOSEX CLOSEY < <(node "$D/data/coord-parse.js" "$D/data/coord-close-probe.json" legend)
{ echo "MOVE=$CLOSEX,$CLOSEY"; "$AB" --session "$S" mouse move "$CLOSEX" "$CLOSEY"; "$AB" --session "$S" mouse down left; "$AB" --session "$S" mouse up left; } > "$D/data/coord-close-click.txt" 2>&1
sleep 1
"$AB" --session "$S" eval --stdin < "$D/data/coord-state.js" > "$D/data/coord-help-closed.json" 2>&1
"$AB" --session "$S" screenshot "$D/screenshots/r43-coord-help-closed.png" > "$D/data/coord-closed-shot.txt" 2>&1
echo COORD_DONE