#!/bin/bash
KEYS="D:/WORKSPACE/fakeToys/laiyipao/deploy/.local.keys"
DOMAIN="$(grep -E "^DOMAIN=" "$KEYS"|head -1|sed "s/^DOMAIN=//;s/[[:space:]]*$//"|tr -d "\r")"
ADMIN_PASS="$(grep -E "^ADMIN_PASS=" "$KEYS"|head -1|sed "s/^ADMIN_PASS=//;s/[[:space:]]*$//"|tr -d "\r")"
DIR="docs/laiyipao/audits/round6-online"; AB="agent-browser --session lyp-r6online-h5"
mask(){ sed -e "s|$DOMAIN|<YOUR_TEST_DOMAIN>|g" -e "s|$ADMIN_PASS|<ADMIN_PASS>|g"; }
echo RESET_PROBE_AFTER_LOGIN
cat <<EOF | $AB eval --stdin 2>&1 | mask | sed "s/^/RESET: /"
(function(){var p=window.__LYP_PROBE__||{};["pageErrors","consoleErrors","failedRequests","apiFailures","recResponses"].forEach(function(k){if(Array.isArray(p[k]))p[k].length=0});return JSON.stringify({hasToken:!!localStorage.getItem("lyp_token"),reset:true})})()
EOF
for P in equipment reactions skills; do
  $AB open "https://$DOMAIN/laiyipao-h5/#/pages/$P/$P" >/dev/null 2>&1
  sleep 5
  echo "--- $P ---"
  cat <<EOF | $AB eval --stdin 2>&1 | mask | sed "s/^/DIFF: /"
JSON.stringify({url:location.href,pe:(window.__LYP_PROBE__.pageErrors||[]).length,ce:(window.__LYP_PROBE__.consoleErrors||[]).length,af:(window.__LYP_PROBE__.apiFailures||[]).length,fr:(window.__LYP_PROBE__.failedRequests||[]).length,recs:(window.__LYP_PROBE__.recResponses||[]).map(function(r){return r.status})})
EOF
done
echo DIFF_DONE