#!/usr/bin/env bash
AB=/c/Users/user/AppData/Local/Microsoft/WinGet/Packages/OpenJS.NodeJS.LTS_Microsoft.Winget.Source_8wekyb3d8bbwe/node-v24.20.0-win-x64/node_modules/agent-browser/bin/agent-browser-win32-x64.exe
S=lyp-r10settle2
D=docs/laiyipao/audits/round10-causality/screenshots
ev() { "$AB" --session "$S" eval "$1" 2>/dev/null; }
ev "location.hash = '#/pages/battle/battle?level=1'" >/dev/null 2>&1
"$AB" --session "$S" open "http://127.0.0.1:5174/#/pages/battle/battle?level=1" >/dev/null 2>&1
"$AB" --session "$S" set viewport 390 844 >/dev/null 2>&1
sleep 2
# 选 8× 倍速
ev "[...document.querySelectorAll('view,text')].find(e=>e.textContent.trim()==='8×')?.click()" >/dev/null 2>&1
echo "SPEED_SET"
RESOLVED=0
for i in $(seq 1 70); do
  TXT=$(ev "document.body.innerText")
  if echo "$TXT" | grep -q "养成前后"; then echo "BATTLE1_SETTLE at T$i"; RESOLVED=1; break; fi
  if echo "$TXT" | grep -q "无法开始战斗"; then echo "START_ERROR at T$i"; break; fi
  if echo "$TXT" | grep -q "继续战斗"; then ev "[...document.querySelectorAll('view,text,button')].find(e=>e.textContent.trim()==='继续战斗')?.click()" >/dev/null 2>&1; fi
  if echo "$TXT" | grep -q "直接下一波"; then ev "[...document.querySelectorAll('view,text')].find(e=>e.textContent.includes('直接下一波'))?.click()" >/dev/null 2>&1; fi
  if echo "$TXT" | grep -q "这一波给你"; then ev "document.querySelectorAll('.hand-card')[0]?.click()" >/dev/null 2>&1; fi
  sleep 2
done
if [ "$RESOLVED" = "1" ]; then
  "$AB" --session "$S" screenshot "$D/after-07-settle-growth.png" >/dev/null 2>&1
  echo "SHOT after-07-settle-growth.png"
fi
echo "BATTLE1_DONE resolved=$RESOLVED"