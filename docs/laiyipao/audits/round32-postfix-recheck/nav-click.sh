#!/bin/bash
cd "$(git rev-parse --show-toplevel)" || exit 1
AB=agent-browser; D=docs/laiyipao/audits/round32-postfix-recheck; E="$PWD/$D"; DATA="$E/data"
S=lyp-r32p-nav; URL='http://127.0.0.1:5175/laiyipao-admin/#/wiki?tab=skills'
"$AB" --session "$S" close >/dev/null 2>&1 || true
"$AB" --session "$S" open about:blank >/dev/null 2>&1; sleep 1
"$AB" --session "$S" set viewport 1280 800 >/dev/null 2>&1
"$AB" --session "$S" --init-script "$E/data/admin-auth-init.js" open "$URL" >/dev/null 2>&1; sleep 7
"$AB" --session "$S" set viewport 1280 800 >/dev/null 2>&1; "$AB" --session "$S" reload >/dev/null 2>&1; sleep 7
printf "%s" 'JSON.stringify({hash:location.hash,navActive:[...document.querySelectorAll(".nav-item.on")].map(x=>x.textContent.trim()),skillVisible:[...document.querySelectorAll("section.wiki-section")].some(x=>x.offsetParent!==null&&x.innerText.includes("技能机制"))})' > "$DATA/nav-pre.txt"
"$AB" --session "$S" eval --stdin < "$DATA/nav-pre.txt" > "$DATA/nav-pre.json" 2>&1; echo PRE_RC=$?; cat "$DATA/nav-pre.json"
"$AB" --session "$S" click '.nav-item:has-text("敌人抗性图鉴")' > "$DATA/nav-click.txt" 2>&1; echo CLICK_RC=$?; cat "$DATA/nav-click.txt"
sleep 2
printf "%s" 'JSON.stringify({hash:location.hash,navActive:[...document.querySelectorAll(".nav-item.on")].map(x=>x.textContent.trim()),enemyVisible:[...document.querySelectorAll("section.wiki-section")].some(x=>x.offsetParent!==null&&x.innerText.includes("绿色为弱点")),skillVisible:[...document.querySelectorAll("section.wiki-section")].some(x=>x.offsetParent!==null&&x.innerText.includes("技能机制"))})' > "$DATA/nav-post.txt"
"$AB" --session "$S" eval --stdin < "$DATA/nav-post.txt" > "$DATA/nav-post.json" 2>&1; echo POST_RC=$?; cat "$DATA/nav-post.json"
echo NAV_DONE=YES
