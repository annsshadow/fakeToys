#!/bin/bash
set -u
S=r97trial768
D="docs/laiyipao/audits/round97-copy-failure-feedback/data"
SHOT="docs/laiyipao/audits/round97-copy-failure-feedback/screenshots"
mkdir -p "$SHOT"
agent-browser close --all --session $S >/dev/null 2>&1
agent-browser open --session $S >/dev/null 2>&1
agent-browser set viewport 768 1024 --session $S >/dev/null 2>&1
agent-browser goto "http://127.0.0.1:5174/#/pages/me/me" --session $S >/dev/null 2>&1
sleep 4
echo "=== PROBE ==="
agent-browser eval --stdin --session $S < "$D/r97-probe.js" 2>&1 | head -3
echo "=== INSTALL ==="
agent-browser eval --stdin --session $S < "$D/r97-install.js" 2>&1 | head -3
echo "=== SUCCESS ==="
agent-browser eval --stdin --session $S < "$D/r97-run-success.js" 2>&1 | head -4
agent-browser screenshot "$(pwd)/$SHOT/trial-success-768.png" --session $S 2>&1 | head -1
echo "=== FAIL ==="
agent-browser eval --stdin --session $S < "$D/r97-run-fail.js" 2>&1 | head -4
agent-browser screenshot "$(pwd)/$SHOT/trial-fail-768.png" --session $S 2>&1 | head -1
echo TRIAL_DONE
