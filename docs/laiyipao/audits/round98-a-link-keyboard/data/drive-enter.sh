#!/usr/bin/env bash
export AGENT_BROWSER_SESSION=r98-enter
AB="agent-browser"
$AB open "http://127.0.0.1:5174/#/pages/me/me" 2>&1 | tail -2
$AB set viewport 390 844 >/dev/null 2>&1
$AB wait 2500 >/dev/null 2>&1
echo "===BEFORE==="; $AB eval --stdin < docs/laiyipao/audits/round98-a-link-keyboard/data/check-nick.js 2>&1 | tail -1
echo "===INJECT==="; $AB eval --stdin < docs/laiyipao/audits/round98-a-link-keyboard/data/inject-nick.js 2>&1 | tail -1
echo "===PRESS_ENTER==="; $AB press Enter 2>&1 | tail -1
echo "===AFTER_ENTER==="; $AB eval --stdin < docs/laiyipao/audits/round98-a-link-keyboard/data/check-nick.js 2>&1 | tail -1
echo "===CLICK_CONTROL==="; $AB find text 改名 click 2>&1 | tail -2
echo "===AFTER_CLICK==="; $AB eval --stdin < docs/laiyipao/audits/round98-a-link-keyboard/data/check-nick.js 2>&1 | tail -1
