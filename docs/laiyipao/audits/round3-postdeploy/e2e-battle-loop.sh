#!/bin/bash
# E2E battle loop; all output masked
KEYS="D:/WORKSPACE/fakeToys/laiyipao/deploy/.local.keys"
DOMAIN=$(grep -E '^DOMAIN=' "$KEYS"|head -1|sed 's/^DOMAIN=//;s/[[:space:]]*$//'|tr -d '\r')
S="lyp-posth5b"
AB="agent-browser --session $S"
mask(){ sed -e "s|$DOMAIN|<YOUR_TEST_DOMAIN>|g" | sed -E 's#https://[^ ]*<YOUR_TEST_DOMAIN>#https://<YOUR_TEST_DOMAIN>#g'; }
echo "=== E2E battle loop ==="
for i in $(seq 1 40); do
  ST=$($AB eval --stdin < docs/laiyipao/audits/round3-postdeploy/e2e-battle-check.js 2>&1 | mask)
  echo "t=${i}x3s $ST"
  echo "$ST" | grep -q '"hasResult":true' && { echo "=== RESULT REACHED ==="; break; }
  sleep 3
done
echo "=== final url ==="
$AB get url 2>&1 | mask
$AB screenshot docs/laiyipao/audits/round3-postdeploy/screenshots/post-e2e-battle-settle.png >/dev/null 2>&1
echo "settle shot saved"
echo "=== DONE ==="