#!/usr/bin/env bash
AB=/c/Users/user/AppData/Local/Microsoft/WinGet/Packages/OpenJS.NodeJS.LTS_Microsoft.Winget.Source_8wekyb3d8bbwe/node-v24.20.0-win-x64/node_modules/agent-browser/bin/agent-browser-win32-x64.exe
S=lyp-r10final
D=docs/laiyipao/audits/round10-causality/screenshots
ev() { "$AB" --session "$S" eval "$1" 2>/dev/null; }
get_hash() { ev "document.body.innerText" | grep -o '回放哈希 [a-z0-9]*' | head -1; }
play_one() {
  local N=$1
  ev "[...document.querySelectorAll('view,text')].find(e=>e.textContent.trim()==='8×')?.click()" >/dev/null 2>&1
  for i in $(seq 1 70); do
    TXT=$(ev "document.body.innerText")
    if echo "$TXT" | grep -q "养成前后" && echo "$TXT" | grep -q "回放哈希"; then echo "BATTLE${N}_SETTLE_T$i"; return 0; fi
    if echo "$TXT" | grep -q "无法开始战斗"; then echo "START_ERROR_$N"; return 1; fi
    if echo "$TXT" | grep -q "继续战斗"; then ev "[...document.querySelectorAll('view,text,button')].find(e=>e.textContent.trim()==='继续战斗')?.click()" >/dev/null 2>&1; fi
    if echo "$TXT" | grep -q "直接下一波"; then ev "[...document.querySelectorAll('view,text')].find(e=>e.textContent.includes('直接下一波'))?.click()" >/dev/null 2>&1; fi
    if echo "$TXT" | grep -q "这一波给你"; then ev "document.querySelectorAll('.hand-card')[0]?.click()" >/dev/null 2>&1; fi
    sleep 2
  done
  return 1
}
"$AB" --session "$S" open "http://127.0.0.1:5174/#/pages/battle/battle?level=1" >/dev/null 2>&1
"$AB" --session "$S" set viewport 390 844 >/dev/null 2>&1
sleep 3
play_one 1 || { echo B1_FAIL; exit 1; }
H1=$(get_hash)
echo "HASH1=$H1"
"$AB" --session "$S" screenshot "$D/after-07-settle-growth.png" >/dev/null 2>&1
echo SHOT_B1
# reload forces a brand-new battle instance in the SAME session (same account)
"$AB" --session "$S" reload >/dev/null 2>&1
sleep 4
echo "AFTER_RELOAD_HAS_SETTLE=$(ev "document.body.innerText.includes('养成前后')")"
ev "[...document.querySelectorAll('view,text')].find(e=>e.textContent.trim()==='8×')?.click()" >/dev/null 2>&1
play_one 2 || { echo B2_FAIL; exit 1; }
H2=$(get_hash)
echo "HASH2=$H2"
if [ "$H1" = "$H2" ]; then echo DUP_HASH_WARNING; else echo DISTINCT_HASHES_OK; fi
"$AB" --session "$S" screenshot "$D/after-08-settle-second.png" >/dev/null 2>&1
echo SHOT_B2
"$AB" --session "$S" open "http://127.0.0.1:5174/#/pages/battles/battles" >/dev/null 2>&1
sleep 3
echo "HAS_TWO=$(ev "document.body.innerText.includes('共 2 场')")"
ev "document.querySelectorAll('.battle-row')[0]?.click()" >/dev/null 2>&1
sleep 4
ev "(()=>{const e=[...document.querySelectorAll('view')].find(x=>x.className&&x.className.includes&&x.className.includes('r10-growth'));return e?e.innerText.slice(0,800):'NONE'})()" > docs/laiyipao/audits/round10-causality/growth-text3.txt 2>&1
echo "GROWTH:"; cat docs/laiyipao/audits/round10-causality/growth-text3.txt
"$AB" --session "$S" screenshot "$D/after-09-battles-growth.png" >/dev/null 2>&1
echo SHOT_BATTLES
echo FINAL_DONE H1=$H1 H2=$H2