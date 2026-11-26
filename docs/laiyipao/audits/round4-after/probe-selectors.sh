#!/bin/bash
S="lyp-r4probe"
AB="agent-browser --session $S"
$AB --init-script docs/laiyipao/audits/round4-after/probe-init.js open "http://127.0.0.1:5174/#/pages/bag/bag" >/dev/null 2>&1; sleep 4
$AB set viewport 390 844 >/dev/null 2>&1
echo "=== bag selectors ==="
$AB eval --stdin < docs/laiyipao/audits/round4-after/probe-selectors.js 2>&1
echo done