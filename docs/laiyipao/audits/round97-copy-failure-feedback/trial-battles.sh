#!/bin/bash
S=r97btrial768
D="docs/laiyipao/audits/round97-copy-failure-feedback/data"
SHOT="docs/laiyipao/audits/round97-copy-failure-feedback/screenshots"
agent-browser close --all --session "$S" >/dev/null 2>&1
agent-browser open --session "$S" >/dev/null 2>&1
agent-browser set viewport 768 1024 --session "$S" >/dev/null 2>&1
agent-browser eval --stdin --session "$S" < "$D/r97-battles-init.js" >/dev/null 2>&1
agent-browser goto "http://127.0.0.1:5174/#/pages/battles/battles" --session "$S" >/dev/null 2>&1
sleep 5
echo "=== PROBE1 ==="
agent-browser eval --stdin --session "$S" < "$D/r97-battles-probe.js" 2>&1 | head -3
echo "=== EXPAND ==="
agent-browser eval --stdin --session "$S" < "$D/r97-expand-row.js" 2>&1 | head -3
sleep 2
echo "=== PROBE2 ==="
agent-browser eval --stdin --session "$S" < "$D/r97-battles-probe.js" 2>&1 | head -3
echo "=== SUCCESS ==="
agent-browser eval --stdin --session "$S" < "$D/r97-run-success-hash.js" 2>&1 | head -4
agent-browser screenshot "$(pwd)/$SHOT/trial-battles-success-768.png" --session "$S" 2>&1 | head -1
echo "=== FAIL ==="
agent-browser eval --stdin --session "$S" < "$D/r97-run-fail-hash.js" 2>&1 | head -4
agent-browser screenshot "$(pwd)/$SHOT/trial-battles-fail-768.png" --session "$S" 2>&1 | head -1
echo BATTLES_TRIAL_DONE
