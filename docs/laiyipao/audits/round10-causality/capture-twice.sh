#!/usr/bin/env bash
AB=/c/Users/user/AppData/Local/Microsoft/WinGet/Packages/OpenJS.NodeJS.LTS_Microsoft.Winget.Source_8wekyb3d8bbwe/node-v24.20.0-win-x64/node_modules/agent-browser/bin/agent-browser-win32-x64.exe
S=lyp-r10twice
D=docs/laiyipao/audits/round10-causality/screenshots
ev() { "$AB" --session "$S" eval "$1" 2>/dev/null; }
play_one() {
  local N=$1
  for i in $(seq 1 70); do
    TXT=$(ev "document.body.innerText")
    if echo "$TXT" | grep -q "通关|失败|未能守住"; then echo "BATTLE$N_SETTLE_T$i"; return 0; fi
    if echo "$TXT" | grep -q "无法开始战斗"; then echo "START_ERROR"; return 1; fi
    if echo "$TXT" | grep -q "继续战斗"; then ev "[...document.querySelectorAll('view,text,button')].find(e=>e.textContent.trim()==='继续战斗')?.click()" >/dev/null 2>&1; fi
    if echo "$TXT" | grep -q "直接下一波"; then ev "[...document.querySelectorAll('view,text')].find(e=>e.textContent.includes('直接下一波'))?.click()" >/dev/null 2>&1; fi
    if echo "$TXT" | grep -q "这一波给你"; then ev "document.querySelectorAll('.hand-card')[0]?.click()" >/dev/null 2>&1; fi
    sleep 2
  done
  return 1
}
"$AB" --session "$S" open "http://127.0.0.1:5174/#/pages/battle/battle?level=1" >/dev/null 2>&1
"$AB" --session "$S" set viewport 390 844 >/dev/null 2>&1
sleep 2
ev "[...document.querySelectorAll('view,text')].find(e=>e.textContent.trim()==='8×')?.click()" >/dev/null 2>&1
play_one 1 || { echo BATTLE1_FAIL; exit 1; }
"$AB" --session "$S" screenshot "$D/after-07-settle-growth.png" >/dev/null 2>&1
echo SHOT_BATTLE1
# 点「再来一局」重开（同一账号、同一 session）
ev "[...document.querySelectorAll('view,text')].find(e=>e.textContent.trim()==='再来一局')?.click()" >/dev/null 2>&1
sleep 3
ev "[...document.querySelectorAll('view,text')].find(e=>e.textContent.trim()==='8×')?.click()" >/dev/null 2>&1
play_one 2 || { echo BATTLE2_FAIL; exit 1; }
"$AB" --session "$S" screenshot "$D/after-08-settle-second.png" >/dev/null 2>&1
echo SHOT_BATTLE2
# 进战报页展开第一条详情
ev "location.hash = '#/pages/battles/battles'" >/dev/null 2>&1
sleep 3
echo "HAS_TWO=$(ev "document.body.innerText.includes('共 2 场')")"
ev "document.querySelectorAll('.battle-row')[0]?.click()" >/dev/null 2>&1
sleep 3
echo "GROWTH_TEXT:"
ev "(()=>{const e=[...document.querySelectorAll('view')].find(x=>x.className&&x.className.includes&&x.className.includes('r10-growth'));return e?e.innerText.slice(0,600):'NONE'})()"
"$AB" --session "$S" screenshot "$D/after-09-battles-growth.png" >/dev/null 2>&1
echo SHOT_BATTLES_GROWTH
echo TWICE_DONE