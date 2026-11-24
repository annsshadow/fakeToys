#!/bin/bash
cd "$(git rev-parse --show-toplevel)" || exit 1
AB=agent-browser; S=lyp-r32-reg2; E="$PWD/docs/laiyipao/audits/round32-postfix-recheck"; D="$E/data"
"$AB" --session "$S" close >/dev/null 2>&1 || true
"$AB" --session "$S" open about:blank >/dev/null 2>&1; sleep 1
"$AB" --session "$S" set viewport 1280 800 >/dev/null 2>&1
"$AB" --session "$S" --init-script "$D/admin-auth-init.js" open 'http://127.0.0.1:5175/laiyipao-admin/#/wiki?tab=enemies' >/dev/null 2>&1; sleep 6
printf '%s' 'JSON.stringify({hash:location.hash,enemyTab:[...document.querySelectorAll(".el-tabs__item")].find(x=>x.textContent.includes("敌人图鉴"))?.getAttribute("aria-selected"),enemyTable:!!document.querySelector("#enemy-table"),skillTable:!!document.querySelector("#skill-table")})' > "$D/reg2-deeplink.txt"
"$AB" --session "$S" eval --stdin < "$D/reg2-deeplink.txt" > "$D/reg2-deeplink.json" 2>&1
echo DEEPLINK; cat "$D/reg2-deeplink.json"
timeout 15 "$AB" --session "$S" fill 'input[placeholder^="搜索关卡"]' '腐蚀扩散' > "$D/reg2-fill.txt" 2>&1; echo FILL_RC=$?
timeout 15 "$AB" --session "$S" press 'Enter' > "$D/reg2-enter.txt" 2>&1; echo ENTER_RC=$?
sleep 2
printf '%s' 'JSON.stringify({hash:location.hash,scrollY:Math.round(scrollY),focusKey:document.querySelector(".wiki-body")?.getAttribute("data-focus"),hasCorrosion:document.body.innerText.includes("腐蚀扩散"),reactionHeaders:[...document.querySelectorAll("#reaction-table .el-table__header th .cell")].map(x=>x.textContent.trim()).slice(0,8)})' > "$D/reg2-search.txt"
"$AB" --session "$S" eval --stdin < "$D/reg2-search.txt" > "$D/reg2-search.json" 2>&1
echo SEARCH; cat "$D/reg2-search.json"