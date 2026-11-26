#!/usr/bin/env bash
AB=/c/Users/user/AppData/Local/Microsoft/WinGet/Packages/OpenJS.NodeJS.LTS_Microsoft.Winget.Source_8wekyb3d8bbwe/node-v24.20.0-win-x64/node_modules/agent-browser/bin/agent-browser-win32-x64.exe
S=lyp-r10settle3
D=docs/laiyipao/audits/round10-causality/screenshots
ev() { "$AB" --session "$S" eval "$1" 2>/dev/null; }
"$AB" --session "$S" open "http://127.0.0.1:5174/#/pages/battle/battle?level=1" >/dev/null 2>&1
"$AB" --session "$S" set viewport 390 844 >/dev/null 2>&1
sleep 2
ev "[...document.querySelectorAll('view,text')].find(e=>e.textContent.trim()==='8×')?.click()" >/dev/null 2>&1
RESOLVED=0
for i in $(seq 1 70); do
  TXT=$(ev "document.body.innerText")
  if echo "$TXT" | grep -q "养成前后"; then echo "BATTLE2_SETTLE at T$i"; RESOLVED=1; break; fi
  if echo "$TXT" | grep -q "无法开始战斗"; then echo "START_ERROR"; break; fi
  if echo "$TXT" | grep -q "继续战斗"; then ev "[...document.querySelectorAll('view,text,button')].find(e=>e.textContent.trim()==='继续战斗')?.click()" >/dev/null 2>&1; fi
  if echo "$TXT" | grep -q "直接下一波"; then ev "[...document.querySelectorAll('view,text')].find(e=>e.textContent.includes('直接下一波'))?.click()" >/dev/null 2>&1; fi
  if echo "$TXT" | grep -q "这一波给你"; then ev "document.querySelectorAll('.hand-card')[0]?.click()" >/dev/null 2>&1; fi
  sleep 2
done
if [ "$RESOLVED" = "1" ]; then
  "$AB" --session "$S" screenshot "$D/after-08-settle-second.png" >/dev/null 2>&1
  echo "SHOT after-08-settle-second.png"
  ev "location.hash = '#/pages/battles/battles'" >/dev/null 2>&1
  sleep 3
  ev "document.querySelectorAll('.battle-row')[0]?.click()" >/dev/null 2>&1
  sleep 3
  echo "DETAIL_TEXT:"
  ev "document.body.innerText.includes('可信差值')" 
  "$AB" --session "$S" screenshot "$D/after-09-battles-growth.png" >/dev/null 2>&1
  echo "SHOT after-09-battles-growth.png"
fi
echo "BATTLE2_DONE resolved=$RESOLVED"