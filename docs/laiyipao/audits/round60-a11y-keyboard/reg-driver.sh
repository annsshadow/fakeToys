#!/usr/bin/env bash
set -euo pipefail
ROOT=$(git rev-parse --show-toplevel)
D="$ROOT/docs/laiyipao/audits/round60-a11y-keyboard"
SESS=lyp-r60-reg
BASE="http://127.0.0.1:5201/#/pages"
AB(){ agent-browser --session "$1" "${@:2}"; }
AB "$SESS" open >/dev/null
for P in me me index index reactions reactions equipment equipment; do
  AB "$SESS" open --init-script "$D/data/auth-init.js" "$BASE/$P/$P" >/dev/null
  AB "$SESS" set viewport 768 1024 >/dev/null
  sleep 2
  AB "$SESS" eval --stdin < "$D/r60-reg-probe.js" > "$D/data/reg-$P.json" 2>"$D/data/reg-$P.err"
  echo "REG $P done"
done
# mouse positive control on announcements CTA
AB "$SESS" open --init-script "$D/data/auth-init.js" "$BASE/announcements/announcements" >/dev/null
AB "$SESS" set viewport 768 1024 >/dev/null
sleep 2
AB "$SESS" eval "location.hash" > "$D/data/mouse-before.txt" 2>&1
AB "$SESS" click ".r3-ai-cta" > "$D/data/mouse-click.txt" 2>&1
sleep 1
AB "$SESS" eval "location.hash" > "$D/data/mouse-after.txt" 2>&1
AB "$SESS" screenshot "$D/screenshots/r60-mouse-announcements.png" >/dev/null
echo MOUSE_DONE
AB "$SESS" close >/dev/null 2>&1 || true
echo REG_FINISH
