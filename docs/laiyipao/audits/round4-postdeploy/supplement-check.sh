#!/bin/bash
# t44 补验：admin 五视图精确 AI 卡选择器 + 线上 config AoE 25% 文案
KEYS="D:/WORKSPACE/fakeToys/laiyipao/deploy/.local.keys"
DOMAIN="$(grep -E "^DOMAIN=" "$KEYS" | head -1 | sed "s/^DOMAIN=//;s/[[:space:]]*$//" | tr -d "\r")"
ADMIN_PASS="$(grep -E "^ADMIN_PASS=" "$KEYS" | head -1 | sed "s/^ADMIN_PASS=//;s/[[:space:]]*$//" | tr -d "\r")"
AB="agent-browser --session lyp-r4online-admin"
BASE="https://$DOMAIN/laiyipao-admin/#"
DIR="docs/laiyipao/audits/round4-postdeploy"
mask(){ sed -e "s|$DOMAIN|<YOUR_TEST_DOMAIN>|g" -e "s|$ADMIN_PASS|<ADMIN_PASS>|g"; }
echo "=== admin AI card precise re-check ==="
for V in dashboard content economy defenses equipment; do
  $AB open "$BASE/admin/$V" >/dev/null 2>&1
  sleep 3
  echo "--- $V ---"
  cat <<EOF | $AB eval --stdin 2>&1 | mask | sed "s/^/AI: /"
JSON.stringify({aiInsightCard:document.querySelectorAll(".ai-insight-card").length,aiInsight:document.querySelectorAll(".ai-insight").length,badge:(document.querySelector(".ai-badge,[id='ai-insight-card'] .ai-badge")||{}).innerText||"",title:(document.querySelector(".ai-title,.ai-insight-title")||{}).innerText||"",headline:(document.querySelector(".ai-headline,.ai-insight-headline")||{}).innerText||"",evidence:(document.querySelector(".ai-evidence,.ai-insight-evidence")||{}).innerText||"",cta:(document.querySelector(".ai-cta,.ai-insight-cta,.ai-insight-card button")||{}).innerText||""})
EOF
done
echo "=== online config AoE 25% text ==="
curl -s -k --max-time 15 "https://$DOMAIN/laiyipao-api/api/v1/config" 2>/dev/null | python -c "import sys,json;d=json.load(sys.stdin);s=json.dumps(d,ensure_ascii=False);import re;hits=[m for m in re.findall(r'[^\"]{0,40}25%[^\"]{0,40}',s)];print('AOE25_HITS='+str(len(hits)));print(chr(10).join(hits[:8]))" 2>&1 | mask
echo DONE