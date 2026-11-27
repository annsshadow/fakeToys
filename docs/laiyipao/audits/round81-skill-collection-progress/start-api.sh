#!/bin/bash
cd laiyipao/server
export DATABASE_URL="postgres://postgres@127.0.0.1:15440/laiyipao?sslmode=disable"
export BOOTSTRAP_ADMIN_PASS="admin12345"
export APP_ENV="local"
echo "=== R81 STARTING API ==="
exec go run ./cmd/api
