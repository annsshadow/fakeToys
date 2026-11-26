#!/usr/bin/env bash
set -uo pipefail
ROOT=$(git rev-parse --show-toplevel)
D="$ROOT/docs/laiyipao/audits/round53-home-pct-rounding"
AB(){ agent-browser --session "$1" "${@:2}"; }
setup(){
  local S="$1" PAGE="$2" BODYNAME="$3"
  local BODY CFGBODY
  BODY=$(cat "$D/data/me-$BODYNAME.json")
  CFGBODY=$(cat "$D/data/config-$BODYNAME.json")
  AB "$S" close >/dev/null 2>&1 || true
  AB "$S" open >/dev/null 2>&1
  AB "$S" open --init-script "$D/data/auth-init.js" "http://127.0.0.1:5201/#/pages/$PAGE" >/dev/null 2>&1
  AB "$S" set viewport 390 844 >/dev/null 2>&1
  sleep 3
  AB "$S" network unroute >/dev/null 2>&1 || true
  AB "$S" network route "http://127.0.0.1:8080/api/v1/me" --body "$BODY" --allow-origin http://127.0.0.1:5201 > "$D/data/$4-route.txt" 2>&1
  AB "$S" network route "http://127.0.0.1:8080/api/v1/config" --body "$CFGBODY" --allow-origin http://127.0.0.1:5201 > "$D/data/$4-cfgroute.txt" 2>&1
  AB "$S" reload > "$D/data/$4-reload.txt" 2>&1
  sleep 5
  cat "$D/r53-reg-probe.js" | AB "$S" eval --stdin > "$D/data/reg-$4.json" 2>&1
  AB "$S" screenshot "$D/screenshots/r53-reg-$4.png" >/dev/null 2>&1
  echo REG_$4_DONE
  AB "$S" close >/dev/null 2>&1 || true
}
setup lyp-r53-r2-skills skills/skills case1 reg2-skills
setup lyp-r53-r2-eq equipment/equipment case1 reg2-equipment
setup lyp-r53-r2-gems gems/gems case1 reg2-gems
setup lyp-r53-r2-bag bag/bag case1 reg2-bag
setup lyp-r53-r2-me me/me case1 reg2-me
setup lyp-r53-r2-el skills/skills element reg2-element
echo R53_REG_DRIVER2_DONE=YES
