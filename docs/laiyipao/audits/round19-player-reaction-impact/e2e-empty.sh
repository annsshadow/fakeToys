#!/bin/bash
AB=agent-browser
D=docs/laiyipao/audits/round19-player-reaction-impact
S=lyp-r19-empty
SH="$PWD/$D/screenshots"
bash "$PWD/$D/mint-token.sh" 11 >/dev/null 2>&1
echo "=== TOKEN=USER11 ==="
"$AB" --session $S close >/dev/null 2>&1
"$AB" --session $S open about:blank >/dev/null 2>&1
sleep 1
"$AB" --session $S set viewport 390 844 >/dev/null 2>&1
"$AB" --session $S --init-script "$PWD/$D/auth-init.js" open "http://127.0.0.1:5174/#/pages/reactions/reactions" >/dev/null 2>&1
sleep 5
"$AB" --session $S set viewport 390 844 >/dev/null 2>&1
"$AB" --session $S reload >/dev/null 2>&1
sleep 5
echo "=== EMPTY DOM ==="
"$AB" --session $S eval --stdin <<'EOF'
(() => { var q=function(s){var e=document.querySelector(s);return e?e.textContent.replace(/\s+/g," ").trim():""}; var p=window.__R19_PROBE__||{}; return JSON.stringify({hash:location.hash,title:document.title,loginPage:/登录|请登录/.test((document.body.innerText||"").slice(0,300)),hasCard:!!document.querySelector(".r19-my-reaction-impact"),empty:q(".r19-rdi-empty"),rows:document.querySelectorAll(".r19-rdi-row").length,meta:q(".r19-rdi-meta"),aiConclusion:q(".r19-ai-conclusion"),aiBasis:q(".r19-ai-basis"),honesty:q(".r19-rdi-honesty"),impacts:(p.impactResponses||[]).slice(-1),pe:(p.pageErrors||[]).length,ce:(p.consoleErrors||[]).length,af:(p.apiFailures||[]).length,fr:(p.failedRequests||[]).length}); })()
EOF
"$AB" --session $S screenshot "$SH/r19-04-empty-user11.png" >/dev/null 2>&1
echo SHOT_EMPTY_DONE
sha256sum "$SH/r19-04-empty-user11.png"
echo "=== DONE ==="