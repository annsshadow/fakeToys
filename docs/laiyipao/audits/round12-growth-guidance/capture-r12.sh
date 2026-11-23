#!/usr/bin/env bash
AB=/c/Users/user/AppData/Local/Microsoft/WinGet/Packages/OpenJS.NodeJS.LTS_Microsoft.Winget.Source_8wekyb3d8bbwe/node-v24.20.0-win-x64/node_modules/agent-browser/bin/agent-browser-win32-x64.exe
S=lyp-r12g
D=docs/laiyipao/audits/round12-growth-guidance/screenshots
mkdir -p "$D"
"$AB" --session "$S" open "http://127.0.0.1:5174/#/pages/index/index" >/dev/null 2>&1
"$AB" --session "$S" set viewport 390 844 >/dev/null 2>&1
sleep 4
echo HOME_TXT=$("$AB" --session "$S" eval "(document.querySelector('.ai-insight-conclusion')||document.querySelector('.r3-ai-conclusion')||document.body).innerText.slice(0,160)" 2>/dev/null)
"$AB" --session "$S" screenshot "$D/after-00-home.png" >/dev/null 2>&1
capture() {
  pg=$1; name=$2
  "$AB" --session "$S" open "http://127.0.0.1:5174/#/pages/$pg/$pg" >/dev/null 2>&1
  "$AB" --session "$S" set viewport 390 844 >/dev/null 2>&1
  sleep 3
  T=$("$AB" --session "$S" eval "document.querySelector('.r12-priority-text')?document.querySelector('.r12-priority-text').innerText:'NONE'" 2>/dev/null)
  C=$("$AB" --session "$S" eval "document.querySelector('.r12-priority-cta')?document.querySelector('.r12-priority-cta').innerText:'NONE'" 2>/dev/null)
  echo "$name PRIORITY=[$T] CTA=[$C]"
  "$AB" --session "$S" screenshot "$D/after-$name.png" >/dev/null 2>&1
}
capture skills 01-skills
capture equipment 02-equipment
capture gems 03-gems
capture bag 04-bag
echo CAPTURE_R12_DONE