#!/bin/bash
cd "$(git rev-parse --show-toplevel)" || exit 1
AB=agent-browser; S=lyp-r32-reg; E="$PWD/docs/laiyipao/audits/round32-postfix-recheck"; D="$E/data"
"$AB" --session "$S" close >/dev/null 2>&1 || true
"$AB" --session "$S" open about:blank >/dev/null 2>&1; sleep 1
"$AB" --session "$S" set viewport 1280 800 >/dev/null 2>&1
"$AB" --session "$S" --init-script "$D/admin-auth-init.js" open 'http://127.0.0.1:5175/laiyipao-admin/#/wiki?tab=enemies' >/dev/null 2>&1; sleep 6
printf '%s' 'JSON.stringify({hash:location.hash,enemyTabVisible:[...document.querySelectorAll(".el-tabs__item")].some(x=>x.textContent.includes("敌人图鉴")&&x.getAttribute("aria-selected")==="true"),enemyTable:!!document.querySelector("#enemy-table"),skillTable:!!document.querySelector("#skill-table")})' > "$D/reg-deeplink.txt"
"$AB" --session "$S" eval --stdin < "$D/reg-deeplink.txt" > "$D/reg-deeplink.json" 2>&1
echo DEEPLINK; cat "$D/reg-deeplink.json"
"$AB" --session "$S" fill 'input[placeholder*="搜索关卡"]' '腐蚀扩散' >/dev/null 2>&1; sleep 2
"$AB" --session "$S" click 'button:has-text("搜索")' > "$D/reg-search-click.txt" 2>&1; echo SEARCH_CLICK_RC=$?
sleep 2
printf '%s' 'JSON.stringify({hash:location.hash,scrollY:Math.round(scrollY),focused:[...document.querySelectorAll("[data-focus]")].map(x=>x.getAttribute("data-focus")).filter(Boolean).slice(-4),hasCorrosion:document.body.innerText.includes("腐蚀扩散"),r31ReactionTable:!!document.querySelector("#reaction-table"),reactionHeaders:[...document.querySelectorAll("#reaction-table .el-table__header th .cell")].map(x=>x.textContent.trim())})' > "$D/reg-search.txt"
"$AB" --session "$S" eval --stdin < "$D/reg-search.txt" > "$D/reg-search.json" 2>&1
echo SEARCH; cat "$D/reg-search.json"