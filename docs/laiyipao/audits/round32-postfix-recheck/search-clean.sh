#!/bin/bash
cd "$(git rev-parse --show-toplevel)" || exit 1
AB=agent-browser; S=lyp-r32p-search; D=docs/laiyipao/audits/round32-postfix-recheck; E="$PWD/$D"; DATA="$E/data"
URL='http://127.0.0.1:5175/laiyipao-admin/#/wiki?tab=skills'
"$AB" --session "$S" close >/dev/null 2>&1 || true
"$AB" --session "$S" open about:blank >/dev/null 2>&1; sleep 1
"$AB" --session "$S" set viewport 390 844 >/dev/null 2>&1
"$AB" --session "$S" --init-script "$E/data/admin-auth-init.js" open "$URL" >/dev/null 2>&1; sleep 6
"$AB" --session "$S" set viewport 390 844 >/dev/null 2>&1; "$AB" --session "$S" reload >/dev/null 2>&1; sleep 6
printf "%s" 'JSON.stringify({hash:location.hash,ready:!!document.querySelector(".wiki-body"),searchPlaceholder:document.querySelector("input[placeholder]")?.getAttribute("placeholder"),skillTable:!!document.querySelector("#skill-table")})' > "$DATA/search-pre.txt"
"$AB" --session "$S" eval --stdin < "$DATA/search-pre.txt" > "$DATA/search-pre.json" 2>&1; echo PRE_RC=$?; cat "$DATA/search-pre.json"
timeout 25 "$AB" --session "$S" click 'input[placeholder*="搜索关卡"]' > "$DATA/search-click.txt" 2>&1; echo CLICK_RC=$?
timeout 25 "$AB" --session "$S" keyboard type '温压弹' > "$DATA/search-type.txt" 2>&1; echo TYPE_RC=$?
timeout 25 "$AB" --session "$S" press Enter > "$DATA/search-enter.txt" 2>&1; echo ENTER_RC=$?
sleep 2
printf "%s" 'JSON.stringify({hash:location.hash,scrollY:Math.round(scrollY),focused:[...document.querySelectorAll("[data-focus]")].map(x=>x.getAttribute("data-focus")).filter(Boolean).slice(-4),inputValue:document.querySelector("input[placeholder]")?.value,hasWenya:document.body.innerText.includes("温压弹"),skillTable:!!document.querySelector("#skill-table")})' > "$DATA/search-post.txt"
"$AB" --session "$S" eval --stdin < "$DATA/search-post.txt" > "$DATA/search-post.json" 2>&1; echo POST_RC=$?; cat "$DATA/search-post.json"
"$AB" --session "$S" screenshot "$E/screenshots/r32p-07-search.png" >/dev/null 2>&1
echo SEARCH_DONE=YES
