#!/bin/bash
# 精确验证命令面板焦点归还：页面内 focusin 事件日志，排除 eval 干扰
S="lyp-admin-r3"
BASE="http://127.0.0.1:5175/laiyipao-admin/#"
AB="agent-browser --session $S"
$AB open "$BASE/admin/dashboard" >/dev/null 2>&1
sleep 2
# 安装 focusin 日志探针
cat <<'EOF' | $AB eval --stdin >/dev/null 2>&1
(function(){window.__FL=[];document.addEventListener('focusin',function(e){var t=e.target;window.__FL.push((t.tagName||'?')+'.'+((typeof t.className==='string'?t.className:'')||'').slice(0,30));},true);return 'installed';})()
EOF
# 用真实 CDP 点击 command-trigger 打开面板（trusted）
cat <<'EOF' | $AB eval --stdin 2>&1 | sed 's/^/TRIGGER: /'
(function(){var t=document.querySelector('.command-trigger');if(!t)return 'no trigger';t.focus();return JSON.stringify({focused:(document.activeElement?.className||'')});})()
EOF
$AB press Control+k >/dev/null 2>&1
sleep 1
cat <<'EOF' | $AB eval --stdin 2>&1 | sed 's/^/OPEN: /'
JSON.stringify({panel:document.querySelectorAll('.cmd-panel').length,focus:(document.activeElement?.className||''),log:window.__FL.slice()})
EOF
# 选中第一导航项（dashboard）
$AB press Enter >/dev/null 2>&1
sleep 2
cat <<'EOF' | $AB eval --stdin 2>&1 | sed 's/^/RESULT: /'
JSON.stringify({url:location.href,panel:document.querySelectorAll('.cmd-panel').length,focus:(document.activeElement?.className||''),focusTag:document.activeElement?.tagName,log:window.__FL.slice()})
EOF
echo DONE
