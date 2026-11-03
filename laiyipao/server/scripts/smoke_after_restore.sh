#!/usr/bin/env bash
# smoke_after_restore.sh — 恢复库的「真玩一遍」入口（R71 / B 类：灾备验证能力增量）
#
# 为什么需要它
# ------------
# `/readyz` 只证明「能连上库」。一份恢复库可以做到：
#   - 行数、外键、索引、goose 版本全部一致；
#   - /readyz 200、游客登录、结算也都成功；
# 但**序列当前值没回来**（下一次写入撞已有 id）、
# **触发器没随 dump 恢复**（看板计数表静默停在旧值）、
# **时区不同**（每日任务/看板 Today 口径漂移）。
#
# 这些都属于「线上不报错、数字慢慢变旧」的隐性故障，只能在恢复当天用
# 真实业务路径去撞出来。本脚本调用 `cmd/smoke_restore` 做这件事。
#
# 用法
# ----
#   BASE_URL=http://127.0.0.1:18080 \
#   DATABASE_URL=postgres://postgres@127.0.0.1:5432/laiyipao_restore_drill?sslmode=disable \
#   EXPECTED_TZ=Etc/UTC \
#     ./scripts/smoke_after_restore.sh
#
# 环境变量
#   BASE_URL        必填，指向「使用恢复库」的 API 实例
#   DATABASE_URL    必填，恢复库（库名必须含 drill/smoke，安全阀）
#   EXPECTED_TZ     可选，源库时区；给了就要求恢复库一致
#   MIGRATIONS_DIR  可选，默认 <server>/migrations
#   ALLOW_NON_DRILL 可选，=1 时允许对非演练库执行（会写数据，慎用）
#
# 退出码：0 = 全部通过；非 0 = 失败（smoke 会打印 `[FAIL] <步骤名>`）。
#
# 演练流程已经串联：scripts/db_restore_drill.sh 的第 6 步会自动调用本脚本。

set -euo pipefail

SERVER_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$SERVER_DIR"

: "${BASE_URL:?必须设置 BASE_URL（指向使用恢复库的 API 实例）}"
: "${DATABASE_URL:?必须设置 DATABASE_URL（恢复库，库名需含 drill/smoke）}"

export BASE_URL
export DATABASE_URL
export MIGRATIONS_DIR="${MIGRATIONS_DIR:-$SERVER_DIR/migrations}"

exec go run ./cmd/smoke_restore
