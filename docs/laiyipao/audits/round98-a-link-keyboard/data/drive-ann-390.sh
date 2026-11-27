#!/usr/bin/env bash
export AGENT_BROWSER_SESSION=r98-ann-390
AB="agent-browser"
$AB open "http://127.0.0.1:5174/#/pages/announcements/announcements" 2>&1 | tail -2
$AB set viewport 390 844 2>&1 | tail -1
$AB wait 2500 2>&1 | tail -1
echo "===BEFORE_FILL==="
$AB eval --stdin < docs/laiyipao/audits/round98-a-link-keyboard/data/probe-links.js 2>&1 | tail -2
$AB fill "input.search-inp" "zzzzNOTFOUNDzzzz" 2>&1 | tail -2
$AB wait 800 2>&1 | tail -1
echo "===AFTER_FILL==="
$AB eval --stdin < docs/laiyipao/audits/round98-a-link-keyboard/data/probe-links.js 2>&1 | tail -2
echo "===SNAP==="
$AB snapshot -i 2>&1 | grep -iE "清除|link|button|generic" | head -20
$AB screenshot docs/laiyipao/audits/round98-a-link-keyboard/screenshots/ann-link-390.png 2>&1 | tail -2
