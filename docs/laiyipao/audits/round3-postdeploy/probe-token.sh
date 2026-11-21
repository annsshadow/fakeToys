#!/bin/bash
# 通过已登录会话申请战斗 token（location.hostname 自动取域名，无需真值变量）
S="lyp-posth5b"
AB="agent-browser --session $S"
echo "=== battle/token via auth ==="
$AB eval --stdin < docs/laiyipao/audits/round3-postdeploy/probe-token.js 2>&1
echo "=== done ==="
