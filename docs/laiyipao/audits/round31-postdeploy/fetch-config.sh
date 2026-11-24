#!/bin/bash
cd "$(git rev-parse --show-toplevel)" || exit 1
curl -s http://127.0.0.1:8080/api/v1/config > docs/laiyipao/audits/round31-postdeploy/data/config.json
echo CONFIG_BYTES=$(wc -c < docs/laiyipao/audits/round31-postdeploy/data/config.json)
