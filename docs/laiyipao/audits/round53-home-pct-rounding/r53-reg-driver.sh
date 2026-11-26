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
  AB "$S" network route "http://127.0.0.1:8080/api/v1/me" --body "$BODY" --allow-origin http://127.0.0.1:5201 >/dev/null 2>&1
  AB "$S" network route "http://127.0.0.1:8080/api/v1/config" --body "$CFGBODY" --allow-origin http://127.0.0.1:5201 >/dev/null 2>&1
  AB "$S" reload >/dev/null 2>&1
  sleep 5
}
for PAGE in skills/ skills/equipment/equipment gems/gems bag/bag me/me; do
  S="lyp-r53-reg-$(echo $PAGE | tr '/' '-')"
  setup "$S" "$PAGE" case1
  OUT=$(echo $PAGE | tr '/' '-')
  cat "$D/r53-reg-probe.js" | AB "$S" eval --stdin > "$D/data/reg-$OUT.json" 2>&1
  AB "$S" screenshot "$D/screenshots/r53-reg-$OUT.png" >/dev/null 2>&1
  echo REG_${OUT}_DONE
  AB "$S" close >/dev/null 2>&1 || true
done
echo R53_REG_DRIVER_DONE=YES
