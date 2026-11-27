#!/bin/bash
D="docs/laiyipao/audits/round97-copy-failure-feedback/data"
SHOT="docs/laiyipao/audits/round97-copy-failure-feedback/screenshots"
mkdir -p "$SHOT"
run_b(){
  local VV="$1"; local WW="$2"; local HH="$3"; local SS="r97b${VV}"
  agent-browser close --all --session "$SS" >/dev/null 2>&1
  agent-browser open --init-script "$(pwd)/$D/r97-battles-init.js" --session "$SS" >/dev/null 2>&1
  agent-browser set viewport "$WW" "$HH" --session "$SS" >/dev/null 2>&1
  agent-browser navigate "http://127.0.0.1:5174/#/pages/battles/battles" --session "$SS" >/dev/null 2>&1
  sleep 5
  { echo "=== BATTLES $VV PROBE ==="; agent-browser eval --stdin --session "$SS" < "$D/r97-battles-probe.js" 2>&1 | head -2;
    agent-browser eval --stdin --session "$SS" < "$D/r97-expand-row.js" 2>&1 | head -1; sleep 2;
    echo "PROBE_EXPANDED:"; agent-browser eval --stdin --session "$SS" < "$D/r97-battles-probe.js" 2>&1 | head -2;
    echo "=== SUCCESS $VV ==="; agent-browser eval --stdin --session "$SS" < "$D/r97-run-success-hash.js" 2>&1 | head -3;
    agent-browser screenshot "$(pwd)/$SHOT/r97-battles-$VV-success.png" --session "$SS" 2>&1 | head -1;
    agent-browser eval --stdin --session "$SS" < "$D/r97-expand-row.js" 2>&1 | head -1; sleep 2;
    echo "=== FAIL $VV ==="; agent-browser eval --stdin --session "$SS" < "$D/r97-run-fail-hash.js" 2>&1 | head -3;
    agent-browser screenshot "$(pwd)/$SHOT/r97-battles-$VV-fail.png" --session "$SS" 2>&1 | head -1;
  } > "$D/w-battles-$VV.log" 2>&1
  echo "BATTLES $VV DONE"
}
run_b 390 390 844
run_b 768 768 1024
run_b 1280 1280 800
echo BATTLES_WALK_DONE
