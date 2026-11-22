#!/usr/bin/env bash
AB=/c/Users/user/AppData/Local/Microsoft/WinGet/Packages/OpenJS.NodeJS.LTS_Microsoft.Winget.Source_8wekyb3d8bbwe/node-v24.20.0-win-x64/node_modules/agent-browser/bin/agent-browser-win32-x64.exe
S=lyp-r10twice
D=docs/laiyipao/audits/round10-causality/screenshots
ev() { "$AB" --session "$S" eval "$1" 2>/dev/null; }
# battle1 already settled in this session — capture it
ev "document.body.innerText.includes('养成前后')" >/dev/null 2>&1
T1=$(ev "document.body.innerText")
echo "B1_GROWTH=$(echo "$T1" | grep -o '回放哈希 [a-z0-9]*')"
"$AB" --session "$S" screenshot "$D/after-07-settle-growth.png" >/dev/null 2>&1
echo SHOT_B1
# click retry — same account/session, redirectTo keeps one page
ev "[...document.querySelectorAll('view,text')].find(e=>e.textContent.trim()==='再来一局')?.click()" >/dev/null 2>&1
sleep 4
ev "[...document.querySelectorAll('view,text')].find(e=>e.textContent.trim()==='8×')?.click()" >/dev/null 2>&1
RESOLVED=0
for i in $(seq 1 70); do
  TXT=$(ev "document.body.innerText")
  if echo "$TXT" | grep -q "养成前后"; then echo "BATTLE2_SETTLE_T$i"; RESOLVED=1; break; fi
  if echo "$TXT" | grep -q "无法开始战斗"; then echo "START_ERROR"; break; fi
  if echo "$TXT" | grep -q "继续战斗"; then ev "[...document.querySelectorAll('view,text,button')].find(e=>e.textContent.trim()==='继续战斗')?.click()" >/dev/null 2>&1; fi
  if echo "$TXT" | grep -q "直接下一波"; then ev "[...document.querySelectorAll('view,text')].find(e=>e.textContent.includes('直接下一波'))?.click()" >/dev/null 2>&1; fi
  if echo "$TXT" | grep -q "这一波给你"; then ev "document.querySelectorAll('.hand-card')[0]?.click()" >/dev/null 2>&1; fi
  sleep 2
done
if [ "$RESOLVED" = "1" ]; then
  echo "B2_HASH=$(ev "document.body.innerText" | grep -o '回放哈希 [a-z0-9]*')"
  "$AB" --session "$S" screenshot "$D/after-08-settle-second.png" >/dev/null 2>&1
  echo SHOT_B2
  ev "location.hash = '#/pages/battles/battles'" >/dev/null 2>&1
  sleep 3
  echo "HAS_TWO=$(ev "document.body.innerText.includes('共 2 场')")"
  ev "document.querySelectorAll('.battle-row')[0]?.click()" >/dev/null 2>&1
  sleep 4
  echo "GROWTH:"
  ev "(()=>{const e=[...document.querySelectorAll('view')].find(x=>x.className&&x.className.includes&&x.className.includes('r10-growth'));return e?e.innerText.slice(0,700):'NONE'})()" > docs/laiyipao/audits/round10-causality/growth-text.txt 2>&1
  cat docs/laiyipao/audits/round10-causality/growth-text.txt
  "$AB" --session "$S" screenshot "$D/after-09-battles-growth.png" >/dev/null 2>&1
  echo SHOT_BATTLES
fi
echo RESOLVED=$RESOLVED