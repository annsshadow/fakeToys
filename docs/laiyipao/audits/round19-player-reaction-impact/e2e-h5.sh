#!/bin/bash
AB=agent-browser
D=docs/laiyipao/audits/round19-player-reaction-impact
S=lyp-r19-e2e
SH="$PWD/$D/screenshots"
mkdir -p "$SH"
echo "=== RESET ==="
"$AB" --session $S close >/dev/null 2>&1
"$AB" --session $S open about:blank >/dev/null 2>&1
sleep 1
"$AB" --session $S set viewport 390 844 >/dev/null 2>&1
"$AB" --session $S --init-script "$PWD/$D/auth-init.js" open "http://127.0.0.1:5174/#/pages/reactions/reactions" >/dev/null 2>&1
sleep 5
"$AB" --session $S set viewport 390 844 >/dev/null 2>&1
"$AB" --session $S reload >/dev/null 2>&1
sleep 5
echo "=== PAGE STATE ==="
"$AB" --session $S eval --stdin <<'EOF'
(() => { const body=(document.body.innerText||"").replace(/\s+/g," ").slice(0,260); return JSON.stringify({hash:location.hash,title:document.title,body:body,hasCard:!!document.querySelector(".r19-my-reaction-impact")}); })()
EOF
echo "=== COUNT VIEW DOM ==="
"$AB" --session $S eval --stdin <<'EOF'
(() => { const p=window.__R19_PROBE__||{}; const q=function(s){var e=document.querySelector(s);return e?e.textContent.replace(/\s+/g," ").trim():""}; var active=[].slice.call(document.querySelectorAll(".r19-toggle-btn")).filter(function(b){return b.classList.contains("active")}).map(function(b){return b.textContent.trim()}); var rows=[].slice.call(document.querySelectorAll(".r19-rdi-row")).map(function(r){return {key:r.getAttribute("data-r19-reaction"),text:r.textContent.replace(/\s+/g," ").trim()}}); return JSON.stringify({hash:location.hash,title:document.title,loginPage:/登录|请登录/.test((document.body.innerText||"").slice(0,300)),hasCard:!!document.querySelector(".r19-my-reaction-impact"),active:active,window:q(".r19-rdi-window"),empty:q(".r19-rdi-empty"),meta:q(".r19-rdi-meta"),rows:rows,aiTitle:q(".r19-ai-title"),aiConclusion:q(".r19-ai-conclusion"),aiBasis:q(".r19-ai-basis"),aiCta:q(".r19-ai-cta"),honesty:q(".r19-rdi-honesty"),impacts:(p.impactResponses||[]).slice(-2),pe:(p.pageErrors||[]).length,ce:(p.consoleErrors||[]).length,af:(p.apiFailures||[]).length,fr:(p.failedRequests||[]).length}); })()
EOF
"$AB" --session $S screenshot "$SH/r19-01-count-view.png" >/dev/null 2>&1
echo SHOT1_DONE
echo "=== CLICK DAMAGE ==="
"$AB" --session $S eval --stdin <<'EOF'
(() => { var b=[].slice.call(document.querySelectorAll(".r19-toggle-btn")).find(function(x){return x.textContent.trim()==="伤害"}); if(b){b.click();return "CLICKED_DAMAGE"} return "NO_BTN" })()
EOF
sleep 1
echo "=== DAMAGE VIEW DOM ==="
"$AB" --session $S eval --stdin <<'EOF'
(() => { var q=function(s){var e=document.querySelector(s);return e?e.textContent.replace(/\s+/g," ").trim():""}; var active=[].slice.call(document.querySelectorAll(".r19-toggle-btn")).filter(function(b){return b.classList.contains("active")}).map(function(b){return b.textContent.trim()}); var rows=[].slice.call(document.querySelectorAll(".r19-rdi-row")).map(function(r){return {key:r.getAttribute("data-r19-reaction"),text:r.textContent.replace(/\s+/g," ").trim()}}); return JSON.stringify({active:active,rows:rows,aiConclusion:q(".r19-ai-conclusion"),honesty:q(".r19-rdi-honesty")}); })()
EOF
"$AB" --session $S screenshot "$SH/r19-02-damage-view.png" >/dev/null 2>&1
echo SHOT2_DONE
"$AB" --session $S set viewport 390 1100 >/dev/null 2>&1
sleep 1
"$AB" --session $S screenshot "$SH/r19-03-damage-tall.png" >/dev/null 2>&1
echo SHOT3_DONE
echo "=== SHA256 ==="
sha256sum "$SH"/r19-0*.png
echo "=== DONE ==="