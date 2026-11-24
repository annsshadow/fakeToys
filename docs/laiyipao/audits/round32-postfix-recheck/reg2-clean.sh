#!/bin/bash
cd "$(git rev-parse --show-toplevel)" || exit 1
AB=agent-browser; D=docs/laiyipao/audits/round32-postfix-recheck; E="$PWD/$D"; DATA="$E/data"
S=lyp-r32p-search2; URL='http://127.0.0.1:5175/laiyipao-admin/#/wiki?tab=skills'
"$AB" --session "$S" close >/dev/null 2>&1 || true
"$AB" --session "$S" open about:blank >/dev/null 2>&1; sleep 1
"$AB" --session "$S" set viewport 390 844 >/dev/null 2>&1
"$AB" --session "$S" --init-script "$E/data/admin-auth-init.js" open "$URL" >/dev/null 2>&1; sleep 6
"$AB" --session "$S" set viewport 390 844 >/dev/null 2>&1; "$AB" --session "$S" reload >/dev/null 2>&1; sleep 6
echo SEARCH_READY=$("$AB" --session "$S" eval --stdin < "$DATA/search-pre.txt" 2>&1)
"$AB" --session "$S" click 'input[placeholder*="搜索关卡"]' > "$DATA/search2-click.txt" 2>&1; echo CLICK_RC=$?; cat "$DATA/search2-click.txt"
"$AB" --session "$S" keyboard type '温压弹' > "$DATA/search2-type.txt" 2>&1; echo TYPE_RC=$?; cat "$DATA/search2-type.txt"
"$AB" --session "$S" press Enter > "$DATA/search2-enter.txt" 2>&1; echo ENTER_RC=$?; cat "$DATA/search2-enter.txt"
sleep 2
printf "%s" 'JSON.stringify({hash:location.hash,scrollY:Math.round(scrollY),focused:[...document.querySelectorAll("[data-focus]")].map(x=>x.getAttribute("data-focus")).filter(Boolean).slice(-4),inputValue:document.querySelector("input[placeholder]")?.value,hasWenya:document.body.innerText.includes("温压弹"),skillTable:!!document.querySelector("#skill-table")})' > "$DATA/search2-post.txt"
echo SEARCH_POST=$("$AB" --session "$S" eval --stdin < "$DATA/search2-post.txt" 2>&1)
"$AB" --session "$S" screenshot "$E/screenshots/r32p-07-search.png" >/dev/null 2>&1
S=lyp-r32p-deep2; URL='http://127.0.0.1:5175/laiyipao-admin/#/wiki?tab=enemies'
"$AB" --session "$S" close >/dev/null 2>&1 || true
"$AB" --session "$S" open about:blank >/dev/null 2>&1; sleep 1
"$AB" --session "$S" set viewport 390 844 >/dev/null 2>&1
"$AB" --session "$S" --init-script "$E/data/admin-auth-init.js" open "$URL" >/dev/null 2>&1; sleep 6
"$AB" --session "$S" set viewport 390 844 >/dev/null 2>&1; "$AB" --session "$S" reload >/dev/null 2>&1; sleep 6
printf "%s" 'JSON.stringify({hash:location.hash,activeTabs:[...document.querySelectorAll(".el-tabs__item")].filter(x=>x.classList.contains("is-active")).map(x=>x.textContent.trim()),enemyTable:!!document.querySelector("#enemy-table"),skillTable:!!document.querySelector("#skill-table"),bodyHasEnemy:document.body.innerText.includes("敌人图鉴")})' > "$DATA/deep2-probe.txt"
echo DEEP_PROBE=$("$AB" --session "$S" eval --stdin < "$DATA/deep2-probe.txt" 2>&1)
"$AB" --session "$S" screenshot "$E/screenshots/r32p-08-deeplink.png" >/dev/null 2>&1
echo REG2_DONE=YES
