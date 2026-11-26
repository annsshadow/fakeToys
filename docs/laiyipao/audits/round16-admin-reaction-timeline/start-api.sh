#!/bin/bash
# QA测试环境启动后端（回环地址，真值不入库）
cd laiyipao/server || exit 1
export DATABASE_URL="postgres://postgres@127.0.0.1:15440/laiyipao?sslmode=disable"
export ADDR=":8080"
exec go run ./cmd/api
