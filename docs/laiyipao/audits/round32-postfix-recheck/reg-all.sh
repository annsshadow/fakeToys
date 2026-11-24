#!/bin/bash
cd "$(git rev-parse --show-toplevel)" || exit 1
AB=agent-browser; S=lyp-r32p-reg-all
D=docs/laiyipao/audits/round32-postfix-recheck; E="$PWD/$D"; DATA="$E/data"
URL='http://127.0.0.1:5175/laiyipao-admin/#/wiki?tab=skills'
"$AB" --session "$S" close >/dev/null 2>&1 || true
"$AB" --session "$S" open about:blank >/dev/null 2>&1; sleep 1
"$AB" --session "$S" set viewport 390 844 >/dev/null 2>&1
"$AB" --session "$S" --init-script "$E/data/admin-auth-init.js" open "$URL" >/dev/null 2>&1; sleep 6
"$AB" --session "$S" set viewport 390 844 >/dev/null 2>&1; "$AB" --session "$S" reload >/dev/null 2>&1; sleep 6
"$AB" --session "$S" eval --stdin < "$E/scroll-probe.js" > "$DATA/scroll-lastcol.json" 2>&1; echo SCROLL_RC=$?
"$AB" --session "$S" eval --stdin < "$E/../round31-wiki-reaction-mechanics/wiki-probe.js" > "$DATA/reg-r31-reaction.json" 2>&1; echo R31_RC=$?
printf "%s" 'JSON.stringify({hash:location.hash,tab:[...document.querySelectorAll(".el-tabs__item")].find(x=>x.getAttribute("aria-selected")==="true")?.textContent.trim(),searchPlaceholder:document.querySelector("input[placeholder]")?.getAttribute("placeholder"),skillTable:!!document.querySelector("#skill-table")})' > "$DATA/reg-pre.txt"
"$AB" --session "$S" eval --stdin < "$DATA/reg-pre.txt" > "$DATA/reg-pre.json" 2>&1; echo PRE_RC=$?
timeout 20 "$AB" --session "$S" fill 'input[placeholder*="搜索关卡"]' '温压弹' > "$DATA/reg-fill.txt" 2>&1; echo FILL_RC=$?
timeout 20 "$AB" --session "$S" press 'Enter' > "$DATA/reg-enter.txt" 2>&1; echo ENTER_RC=$?
sleep 2
printf "%s" 'JSON.stringify({hash:location.hash,scrollY:Math.round(scrollY),focused:[...document.querySelectorAll("[data-focus]")].map(x=>x.getAttribute("data-focus")).filter(Boolean).slice(-4),hasWenya:document.body.innerText.includes("温压弹"),skillTable:!!document.querySelector("#skill-table")})' > "$DATA/reg-post.txt"
"$AB" --session "$S" eval --stdin < "$DATA/reg-post.txt" > "$DATA/reg-post.json" 2>&1; echo POST_RC=$?
printf "%s" 'JSON.stringify({from:location.hash})' > "$DATA/reg-nav-pre.txt"
"$AB" --session "$S" eval --stdin < "$DATA/reg-nav-pre.txt" > "$DATA/reg-nav-pre.json" 2>&1
"$AB" --session "$S" --init-script "$E/data/admin-auth-init.js" open 'http://127.0.0.1:5175/laiyipao-admin/#/wiki?tab=enemies' >/dev/null 2>&1; sleep 6
printf "%s" 'JSON.stringify({hash:location.hash,activeTab:[...document.querySelectorAll(".el-tabs__item")].find(x=>x.getAttribute("aria-selected")==="true")?.textContent.trim(),enemyTable:!!document.querySelector("#enemy-table"),skillTable:!!document.querySelector("#skill-table")})' > "$DATA/reg-deeplink.txt"
"$AB" --session "$S" eval --stdin < "$DATA/reg-deeplink.txt" > "$DATA/reg-deeplink.json" 2>&1; echo DEEPLINK_RC=$?
"$AB" --session "$S" screenshot "$E/screenshots/r32p-06-regression.png" >/dev/null 2>&1; echo REG_DONE=YES
