#!/bin/bash
cd "$(git rev-parse --show-toplevel)" || exit 1
AB=agent-browser; D=docs/laiyipao/audits/round32-postfix-recheck; E="$PWD/$D"; DATA="$E/data"
S=lyp-r32p-deep5; URL='http://127.0.0.1:5175/laiyipao-admin/#/wiki?tab=enemies'
"$AB" --session "$S" close >/dev/null 2>&1 || true
"$AB" --session "$S" open about:blank >/dev/null 2>&1; sleep 1
"$AB" --session "$S" set viewport 390 844 >/dev/null 2>&1
"$AB" --session "$S" --init-script "$E/data/admin-auth-init.js" open "$URL" >/dev/null 2>&1; sleep 7
"$AB" --session "$S" set viewport 390 844 >/dev/null 2>&1; "$AB" --session "$S" reload >/dev/null 2>&1; sleep 7
printf "%s" 'JSON.stringify({hash:location.hash,visibleSections:[...document.querySelectorAll("section.wiki-section")].filter(x=>x.offsetParent!==null).map(x=>x.innerText.slice(0,60)),tocActive:[...document.querySelectorAll(".toc-on,.toc-item.on,.toc a.on")].map(x=>x.textContent.trim()),enemySectionVisible:[...document.querySelectorAll("section.wiki-section")].some(x=>x.offsetParent!==null&&x.innerText.includes("敌人图鉴")),bodyHasEnemy:document.body.innerText.includes("敌人图鉴"),bodyHasSkills:document.body.innerText.includes("技能表"),tableRows:document.querySelectorAll("section.wiki-section .el-table__body tbody tr").length})' > "$DATA/deep5-probe.txt"
"$AB" --session "$S" eval --stdin < "$DATA/deep5-probe.txt" > "$DATA/deep5-probe.json" 2>&1; echo PROBE_RC=$?; cat "$DATA/deep5-probe.json"
"$AB" --session "$S" screenshot "$E/screenshots/r32p-10-deeplink.png" >/dev/null 2>&1
echo DEEP5_DONE=YES
