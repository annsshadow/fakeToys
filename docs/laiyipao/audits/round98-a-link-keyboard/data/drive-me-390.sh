#!/usr/bin/env bash
export AGENT_BROWSER_SESSION=r98-me2-390
mkdir -p docs/laiyipao/audits/round98-a-link-keyboard/screenshots
AB="agent-browser --idle-timeout 1h"
$AB open "http://127.0.0.1:5174/#/pages/me/me" 2>&1 | tail -2
$AB set viewport 390 844 2>&1 | tail -1
$AB wait 3000 2>&1 | tail -1
echo "===ME_PROBE==="
$AB eval --stdin < docs/laiyipao/audits/round98-a-link-keyboard/data/probe-me.js 2>&1 | tail -2
echo "===ME_SNAP==="
$AB snapshot -i 2>&1 | grep -iE "复制|button|generic|link" | head -20
echo "===ME_TABS==="
for i in 1 2 3 4 5 6; do echo -n "T$i "; $AB eval 'JSON.stringify({tag:document.activeElement.tagName,cls:document.activeElement.className,txt:(document.activeElement.textContent||"").trim().slice(0,16)})' 2>&1 | tail -1; $AB press Tab >/dev/null 2>&1; done
echo "===ME_SHOT==="
$AB screenshot docs/laiyipao/audits/round98-a-link-keyboard/screenshots/me-link-390.png 2>&1 | tail -1
