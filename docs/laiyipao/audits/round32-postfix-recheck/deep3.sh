#!/bin/bash
cd "$(git rev-parse --show-toplevel)" || exit 1
AB=agent-browser; D=docs/laiyipao/audits/round32-postfix-recheck; E="$PWD/$D"; DATA="$E/data"
S=lyp-r32p-deep3; URL='http://127.0.0.1:5175/laiyipao-admin/#/wiki?tab=enemies'
"$AB" --session "$S" close >/dev/null 2>&1 || true
"$AB" --session "$S" open about:blank >/dev/null 2>&1; sleep 1
"$AB" --session "$S" set viewport 390 844 >/dev/null 2>&1
"$AB" --session "$S" --init-script "$E/data/admin-auth-init.js" open "$URL" >/dev/null 2>&1; sleep 7
printf "%s" 'JSON.stringify({hash:location.hash,activeTab:[...document.querySelectorAll(".el-tabs__item")].filter(x=>x.getAttribute("aria-selected")==="true"||x.classList.contains("is-active")).map(x=>x.textContent.trim()),enemyTable:!!document.querySelector("#enemy-table"),skillTable:!!document.querySelector("#skill-table"),bodyHasEnemy:document.body.innerText.includes("敌人图鉴"),bodyHasSkills:document.body.innerText.includes("技能机制")})' > "$DATA/deep3-probe.txt"
echo DEEP3=$("$AB" --session "$S" eval --stdin < "$DATA/deep3-probe.txt" 2>&1)
echo "=== real CLI click enemy tab ==="
"$AB" --session "$S" click '.el-tabs__item:has-text("敌人图鉴")' > "$DATA/deep3-click.txt" 2>&1; echo CLICK_RC=$?; cat "$DATA/deep3-click.txt"
sleep 1
echo DEEP3_AFTER=$("$AB" --session "$S" eval --stdin < "$DATA/deep3-probe.txt" 2>&1)
"$AB" --session "$S" screenshot "$E/screenshots/r32p-08-deeplink.png" >/dev/null 2>&1
echo DEEP3_DONE=YES
