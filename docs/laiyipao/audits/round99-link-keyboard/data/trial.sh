#!/bin/bash
S=r99trial; AB(){ agent-browser --session "$S" "$@"; }
D="docs/laiyipao/audits/round99-link-keyboard"
AB close --all >/dev/null 2>&1
AB set viewport 768 1024 >/dev/null 2>&1
AB open --init-script "$(pwd)/$D/data/r99-init.js" "http://127.0.0.1:5174/#/pages/bag/bag" 2>&1 | tail -2
sleep 4
echo '--- URL ---'; AB get url 2>&1 | tail -1
echo '--- PROBE ---'; AB eval --stdin < "$D/data/r99-probe.js" 2>&1 | tail -1
echo '--- HITS ---'; AB eval --stdin < "$D/data/r99-hits.js" 2>&1 | tail -1
echo '--- AX ---'; AB snapshot -i 2>&1 | grep -i -E 'button|link|generic' | head -40
echo '--- TRIAL_DONE ---'
