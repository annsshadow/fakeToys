#!/bin/bash
cd "$(git rev-parse --show-toplevel)" || exit 1
D=docs/laiyipao/audits/round36-postdeploy
mkdir -p "$D/data"
curl -sS http://127.0.0.1:8080/api/v1/config --max-time 30 -o "$D/data/config.json" || exit 1
echo "CONFIG_BYTES=$(wc -c < "$D/data/config.json")"
