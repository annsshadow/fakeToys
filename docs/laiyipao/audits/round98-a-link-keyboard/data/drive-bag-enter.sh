#!/usr/bin/env bash
export AGENT_BROWSER_SESSION=r98-bagEnter
AB="agent-browser"
$AB open "http://127.0.0.1:5174/#/pages/bag/bag" 2>&1 | tail -2
$AB set viewport 390 844 >/dev/null 2>&1
$AB wait 2500 >/dev/null 2>&1
echo "===ENTER_BAG==="
$AB eval --stdin < docs/laiyipao/audits/round98-a-link-keyboard/data/enter-probe.js 2>&1 | tail -2
echo "===TAB_BAG==="
for i in 1 2 3 4 5 6; do $AB press Tab >/dev/null 2>&1; printf "T%s " "$i"; $AB eval --stdin < docs/laiyipao/audits/round98-a-link-keyboard/data/probe-active.js 2>&1 | tail -1; done
