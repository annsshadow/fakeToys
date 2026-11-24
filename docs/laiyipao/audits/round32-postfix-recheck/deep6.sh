#!/bin/bash
cd "$(git rev-parse --show-toplevel)" || exit 1
AB=agent-browser; D=docs/laiyipao/audits/round32-postfix-recheck; E="$PWD/$D"; DATA="$E/data"
S=lyp-r32p-deep5; URL='http://127.0.0.1:5175/laiyipao-admin/#/wiki?tab=enemies'
"$AB" --session "$S" set viewport 1280 800 >/dev/null 2>&1; sleep 1
printf "%s" 'JSON.stringify({hash:location.hash,navActive:[...document.querySelectorAll(".nav-item.on")].map(x=>x.textContent.trim()),enemySectionVisible:[...document.querySelectorAll("section.wiki-section")].some(x=>x.offsetParent!==null&&x.innerText.includes("绿色为弱点")),skillSectionVisible:[...document.querySelectorAll("section.wiki-section")].some(x=>x.offsetParent!==null&&x.innerText.includes("技能机制")),enemyRows:[...document.querySelectorAll("section.wiki-section .el-table__body tbody tr")].slice(0,3).map(x=>x.innerText.replace(/\s+/g," ").slice(0,60)),bodyHasEnemyNav:document.body.innerText.includes("敌人抗性图鉴"),docOverflow:document.documentElement.scrollWidth-window.innerWidth})' > "$DATA/deep6-probe.txt"
"$AB" --session "$S" eval --stdin < "$DATA/deep6-probe.txt" > "$DATA/deep6-probe.json" 2>&1; echo PROBE_RC=$?; cat "$DATA/deep6-probe.json"
"$AB" --session "$S" screenshot "$E/screenshots/r32p-11-deeplink-final.png" >/dev/null 2>&1
echo DEEP6_DONE=YES
