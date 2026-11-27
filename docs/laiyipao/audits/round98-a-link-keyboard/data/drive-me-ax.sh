#!/usr/bin/env bash
export AGENT_BROWSER_SESSION=r98-me6
AB="agent-browser"
$AB open "http://127.0.0.1:5174/#/pages/me/me" 2>&1 | tail -2
$AB set viewport 390 844 >/dev/null 2>&1
$AB wait 2500 >/dev/null 2>&1
echo "===AX390==="
$AB snapshot -i 2>&1 | grep -iE "button|generic|复制|改名" | head -15
echo "===SHOT390==="
$AB screenshot docs/laiyipao/audits/round98-a-link-keyboard/screenshots/me-link-390.png 2>&1 | tail -1
$AB set viewport 768 1024 >/dev/null 2>&1
$AB wait 1000 >/dev/null 2>&1
echo "===SHOT768==="
$AB screenshot docs/laiyipao/audits/round98-a-link-keyboard/screenshots/me-link-768.png 2>&1 | tail -1
$AB set viewport 1280 900 >/dev/null 2>&1
$AB wait 1000 >/dev/null 2>&1
echo "===SHOT1280==="
$AB screenshot docs/laiyipao/audits/round98-a-link-keyboard/screenshots/me-link-1280.png 2>&1 | tail -1
