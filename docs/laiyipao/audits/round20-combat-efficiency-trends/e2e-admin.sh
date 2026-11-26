#!/bin/bash
AB=agent-browser
S=r20-admin
D=docs/laiyipao/audits/round20-combat-efficiency-trends
SH="$PWD/$D/screenshots"
echo STEP1_CLOSE
"$AB" --session $S close >/dev/null 2>&1
echo STEP2_OPEN
"$AB" --session $S set viewport 1440 900 >/dev/null 2>&1
"$AB" --session $S --init-script "$PWD/$D/admin-probe.js" open 'http://127.0.0.1:5175/laiyipao-admin/#/admin/dashboard' 2>&1
echo OPEN_RC=$?
sleep 8
echo STEP3_DOM
EXPR='(() => { var p=window.__R20_ADMIN_PROBE__||{}; var card=document.querySelector(".r20-efficiency-trend"); var row=card&&card.querySelector(".r20-efficiency-row"); var bars=[].slice.call(document.querySelectorAll(".r20-trend-bar")); var ai=card&&card.querySelector(".r20-ai-note"); var note=card&&card.querySelector(".r20-window-note"); var honesty=card&&card.querySelector(".r20-honesty-note"); return JSON.stringify({url:location.href,loginPage:/登录|login/i.test(document.body.innerText.slice(0,200)),hasCard:!!card,role:card&&card.getAttribute("role"),windowNote:note?note.textContent.trim():"",row:row?row.textContent.trim().replace(/\s+/g," "):"",bars:bars.map(function(b){return {bucket:b.getAttribute("data-r20-bucket"),text:b.textContent.trim().replace(/\s+/g," ")}}),aiTitle:ai&&ai.querySelector(".r20-ai-title")?ai.querySelector(".r20-ai-title").textContent.trim():"",aiConclusion:ai&&ai.querySelector(".r20-ai-conclusion")?ai.querySelector(".r20-ai-conclusion").textContent.trim():"",aiBasis:ai&&ai.querySelector(".r20-ai-basis")?ai.querySelector(".r20-ai-basis").textContent.trim():"",aiCta:ai&&ai.querySelector(".r20-ai-cta")?ai.querySelector(".r20-ai-cta").textContent.trim():"",honesty:honesty?honesty.textContent.trim():"",empty:card&&card.querySelector(".r20-efficiency-empty")?card.querySelector(".r20-efficiency-empty").textContent.trim():"",responses:(p.efficiencyResponses||[]).slice(-2),pe:(p.pageErrors||[]).length,ce:(p.consoleErrors||[]).length,af:(p.apiFailures||[]).length,fr:(p.failedRequests||[]).length}); })()'
B=$(printf %s "$EXPR" | base64 -w0)
"$AB" --session $S eval -b "$B" 2>&1 | head -c 6000
echo STEP4_CLICK_TREND
"$AB" --session $S click '.r20-view-trend' 2>&1
sleep 2
B2=$(printf %s "$EXPR" | base64 -w0)
"$AB" --session $S eval -b "$B2" 2>&1 | head -c 6000
echo STEP5_SHOT
"$AB" --session $S screenshot "$SH/r20-04-admin-summary.png" >/dev/null 2>&1
"$AB" --session $S screenshot "$SH/r20-05-admin-trend.png" >/dev/null 2>&1
echo SHOTS_DONE
