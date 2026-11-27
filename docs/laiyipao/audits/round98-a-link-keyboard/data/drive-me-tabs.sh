#!/usr/bin/env bash
export AGENT_BROWSER_SESSION=r98-me3-390
AB="agent-browser --idle-timeout 1h"
mkdir -p docs/laiyipao/audits/round98-a-link-keyboard/screenshots
$AB open "http://127.0.0.1:5174/#/pages/me/me" 2>&1 | tail -2
$AB set viewport 390 844 2>&1 | tail -1
$AB wait 3000 2>&1 | tail -1
echo "===TAB_SEQ==="
for i in 1 2 3 4 5 6 7 8 9 10 11 12; do
  $AB press Tab >/dev/null 2>&1
  printf "T%s " "$i"
  $AB eval --stdin < docs/laiyipao/audits/round98-a-link-keyboard/data/probe-active.js 2>&1 | tail -1
done
