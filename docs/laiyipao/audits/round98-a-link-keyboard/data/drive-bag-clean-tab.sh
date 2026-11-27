#!/usr/bin/env bash
export AGENT_BROWSER_SESSION=r98-bagClean
AB="agent-browser"
$AB open "http://127.0.0.1:5174/#/pages/bag/bag" 2>&1 | tail -1
$AB set viewport 390 844 >/dev/null 2>&1
$AB wait 2500 >/dev/null 2>&1
echo "===BAG_CLEAN_TAB==="
for i in 1 2 3 4 5 6 7 8; do $AB press Tab >/dev/null 2>&1; printf "T%s " "$i"; $AB eval --stdin < docs/laiyipao/audits/round98-a-link-keyboard/data/probe-active.js 2>&1 | tail -1; done
