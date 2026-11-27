#!/bin/bash
S=r97btrial768c
D="docs/laiyipao/audits/round97-copy-failure-feedback/data"
agent-browser close --all --session "$S" >/dev/null 2>&1
agent-browser open --init-script "$(pwd)/$D/r97-battles-init.js" --session "$S" >/dev/null 2>&1
agent-browser set viewport 768 1024 --session "$S" >/dev/null 2>&1
agent-browser navigate "http://127.0.0.1:5174/#/pages/battles/battles" --session "$S" >/dev/null 2>&1
sleep 5
agent-browser eval --stdin --session "$S" < "$D/r97-expand-row.js" 2>&1 | head -1
sleep 2
echo -n "SUCCESS:"; agent-browser eval --stdin --session "$S" < "$D/r97-run-success-hash.js" 2>&1 | head -1
sleep 1
echo -n "FAIL:"; agent-browser eval --stdin --session "$S" < "$D/r97-run-fail-hash.js" 2>&1 | head -1
echo BATTLES_TRIAL3_DONE
