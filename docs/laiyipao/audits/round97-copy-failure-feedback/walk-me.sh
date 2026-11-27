#!/bin/bash
D="docs/laiyipao/audits/round97-copy-failure-feedback/data"
SHOT="docs/laiyipao/audits/round97-copy-failure-feedback/screenshots"
mkdir -p "$SHOT"
run_me(){
  local VV="$1"; local WW="$2"; local HH="$3"
  local SS="r97meVV"
  SS="r97me${VV}"
  agent-browser close --all --session "$SS" >/dev/null 2>&1
  agent-browser open --session "$SS" >/dev/null 2>&1
  agent-browser set viewport "$WW" "$HH" --session "$SS" >/dev/null 2>&1
  agent-browser goto "http://127.0.0.1:5174/#/pages/me/me" --session "$SS" >/dev/null 2>&1
  sleep 4
  { echo "=== ME $VV PROBE ==="; agent-browser eval --stdin --session "$SS" < "$D/r97-probe.js" 2>&1 | head -3; echo "INSTALL:"; agent-browser eval --stdin --session "$SS" < "$D/r97-install.js" 2>&1 | head -1; echo "=== SUCCESS $VV ==="; agent-browser eval --stdin --session "$SS" < "$D/r97-run-success.js" 2>&1 | head -4; agent-browser screenshot "$(pwd)/$SHOT/r97-me-$VV-success.png" --session "$SS" 2>&1 | head -1; echo "=== FAIL $VV ==="; agent-browser eval --stdin --session "$SS" < "$D/r97-run-fail.js" 2>&1 | head -4; agent-browser screenshot "$(pwd)/$SHOT/r97-me-$VV-fail.png" --session "$SS" 2>&1 | head -1; } > "$D/w-me-$VV.log" 2>&1
  echo "ME $VV DONE"
}
run_me 390 390 844
run_me 768 768 1024
run_me 1280 1280 800
echo ME_WALK_DONE
