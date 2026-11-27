#!/usr/bin/env bash
export AGENT_BROWSER_SESSION=r98-bag
AB="agent-browser"
$AB open "http://127.0.0.1:5174/#/pages/bag/bag" 2>&1 | tail -2
for V in "390 844" "768 1024" "1280 900"; do
  set -- $V; W=$1; H=$2
  $AB set viewport $W $H >/dev/null 2>&1
  $AB wait 2500 >/dev/null 2>&1
  echo "===BAG_$W_PROBE==="
  $AB eval --stdin < docs/laiyipao/audits/round98-a-link-keyboard/data/probe-links.js 2>&1 | tail -1
  echo "===BAG_$W_AX==="
  $AB snapshot -i 2>&1 | grep -iE "button|generic" | head -8
  echo "===BAG_$W_SHOT==="
  $AB screenshot docs/laiyipao/audits/round98-a-link-keyboard/screenshots/bag-link-$W.png 2>&1 | tail -1
done
