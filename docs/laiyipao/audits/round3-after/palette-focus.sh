#!/bin/bash
# R3 命令面板选中项后焦点归还专项（真浏览器键盘/焦点探针）
# 补 t16 终审建议②：验证 closePalette/navigateTo/runAiCommand 的焦点归还
S="lyp-admin-r3"
BASE="http://127.0.0.1:5175/laiyipao-admin/#"
DIR="docs/laiyipao/audits/round3-after"
AB="agent-browser --session $S"
echo "=== 回到 dashboard ==="
$AB open "$BASE/admin/dashboard" >/dev/null 2>&1
sleep 2
echo "=== CASE A: 导航项选中后焦点归还 trigger ==="
cat <<'EOF' | $AB eval --stdin 2>&1 | sed 's/^/PRE: /'
(function(){var t=document.querySelector('.command-trigger');if(t)t.focus();return JSON.stringify({trigger:!!t,focused:(document.activeElement?.className||'').slice(0,60)});})()
EOF
$AB press Control+k >/dev/null 2>&1
sleep 1
cat <<'EOF' | $AB eval --stdin 2>&1 | sed 's/^/OPEN: /'
JSON.stringify({panel:document.querySelectorAll('.cmd-panel').length,focusInInput:document.activeElement?.tagName==='INPUT',focusClass:(document.activeElement?.className||'').slice(0,60),opts:document.querySelectorAll('.cmd-option').length})
EOF
# Enter 选中当前高亮导航项
$AB press Enter >/dev/null 2>&1
sleep 2
cat <<'EOF' | $AB eval --stdin 2>&1 | sed 's/^/AFTER_ENTER: /'
JSON.stringify({url:location.href,panel:document.querySelectorAll('.cmd-panel').length,activeTag:document.activeElement?.tagName,activeClass:(document.activeElement?.className||'').slice(0,80),activeText:(document.activeElement?.innerText||'').trim().slice(0,40),isBody:document.activeElement===document.body})
EOF
echo "=== CASE B: AI 命令选中后焦点落到目标 CTA ==="
$AB open "$BASE/admin/dashboard" >/dev/null 2>&1
sleep 2
$AB press Control+k >/dev/null 2>&1
sleep 1
# 输入关键词筛选到 AI 洞察命令
$AB fill ".cmd-panel input" "运营洞察" >/dev/null 2>&1
sleep 1
cat <<'EOF' | $AB eval --stdin 2>&1 | sed 's/^/FILTER: /'
JSON.stringify({aiOpts:document.querySelectorAll('.cmd-ai-option').length,firstAi:(document.querySelector('.cmd-ai-option')?.innerText||'').trim().slice(0,40)})
EOF
# 点击 AI 选项（trusted via CDP click on element center not available; use dispatch + Enter fallback）
cat <<'EOF' | $AB eval --stdin 2>&1 | sed 's/^/CLICK_AI: /'
(function(){var a=document.querySelector('.cmd-ai-option');if(!a)return 'no ai option';a.click();return 'clicked';})()
EOF
sleep 2
cat <<'EOF' | $AB eval --stdin 2>&1 | sed 's/^/AFTER_AI: /'
(function(){var c=document.getElementById('ai-insight-card');var a=c?.querySelector('.ai-cta');return JSON.stringify({url:location.href,panel:document.querySelectorAll('.cmd-panel').length,card:!!c,cta:!!a,focusedCTA:document.activeElement===a,activeClass:(document.activeElement?.className||'').slice(0,60),activeText:(document.activeElement?.innerText||'').trim().slice(0,40)});})()
EOF
echo "=== DONE ==="
