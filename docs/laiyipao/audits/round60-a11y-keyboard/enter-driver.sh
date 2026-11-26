#!/usr/bin/env bash
set -euo pipefail
ROOT=$(git rev-parse --show-toplevel)
D="$ROOT/docs/laiyipao/audits/round60-a11y-keyboard"
SESS=lyp-r60-enter
BASE="http://127.0.0.1:5201/#/pages"
AB(){ agent-browser --session "$1" "${@:2}"; }
AB "$SESS" open >/dev/null
enter_case(){
  local NAME="$1" PAGE="$2" SEL="$3"
  AB "$SESS" open --init-script "$D/data/auth-init.js" "$BASE/$PAGE/$PAGE" >/dev/null
  AB "$SESS" set viewport 768 1024 >/dev/null
  sleep 2
  AB "$SESS" focus "$SEL" >/dev/null 2>&1 || echo "FOCUS_FAIL $NAME"
  AB "$SESS" eval --stdin < "$D/r60-state.js" > "$D/data/enter-$NAME-before.json" 2>"$D/data/enter-$NAME-before.err"
  AB "$SESS" press Enter >/dev/null
  sleep 1
  AB "$SESS" eval --stdin < "$D/r60-state.js" > "$D/data/enter-$NAME-after.json" 2>"$D/data/enter-$NAME-after.err"
  AB "$SESS" screenshot "$D/screenshots/r60-enter-$NAME.png" >/dev/null
  echo "ENTER $NAME done"
}
enter_case announcements announcements ".r3-ai-cta"
enter_case reactions reactions ".r6-why-toggle"
enter_case equipment equipment ".r6-why-toggle"
enter_case skills skills ".r6-why-toggle"
enter_case shop shop ".r3-ai-cta"
enter_case mastery mastery ".r3-ai-cta"
AB "$SESS" close >/dev/null 2>&1 || true
echo ENTER_FINISH
