#!/usr/bin/env bash
# db_restore_drill.sh — 备份恢复演练（R58 / B 类：灾备能力增量）
#
# 为什么需要它
# ------------
# 「有 pg_dump」不等于「能恢复」。真正会出问题的是：
#   1. 恢复到空库时报错（owner / 权限 / 扩展缺失）；
#   2. 恢复成功但**少行**（增量 dump、过滤条件、并发写入）；
#   3. 恢复成功但**约束没回来**（外键 / 唯一索引丢失，之后靠应用层兜）。
# 这些只有「真 dump → 真空库 → 逐表比对」才看得出来。
#
# 做法
# ----
#   1. pg_dump --no-owner --no-privileges（生产恢复通常不是同一个 DB 角色）
#   2. 建一个空库（名字必须含 "drill"，防止误删业务库）
#   3. 恢复、计时、统计 dump 字节数
#   4. 逐表比对行数；比对外键数、公共索引数、goose 版本、服务器时区
#   5. 对恢复库启动一个临时 API（MIGRATIONS_ON_BOOT=false），
#      用 scripts/smoke_after_restore.sh 真玩一遍：
#      游客登录 → 配置 → 开局 → 结算 → 战报 → 进度 → 领任务 → 钱包流水，
#      再验管理端（登录/看板/玩家列表/发公告），并断言写侧触发器真的更新计数表。
#   6. 任一不一致/任一步失败 → 非零退出，并指出是哪一步。
#
# 用法
# ----
#   本机有 pg_dump/psql：
#     SRC_DB=laiyipao ./scripts/db_restore_drill.sh
#   只在本机 Docker 里有客户端（本仓开发环境）：
#     PG_CONTAINER=lyp-pg SRC_DB=laiyipao ./scripts/db_restore_drill.sh
#
# 环境变量（全部有安全缺省）：
#   SRC_DB        源库，默认 laiyipao
#   DST_DB        恢复目标库，默认 <SRC_DB>_restore_drill（必须含 drill）
#   PGHOST/PGPORT/PGUSER  默认 127.0.0.1 / 5432 / postgres
#   PG_CONTAINER  非空时所有 pg_dump/psql 都走 docker exec -i <容器>
#   KEEP_DUMP     非空时保留 dump 文件（默认完成后删除）
#   DRILL_API_PORT  临时 API 端口，默认随机（18080 + PID%500）
#   DRILL_PGHOST    宿主机访问 Postgres 的地址，默认 127.0.0.1
#                   （PG_CONTAINER 模式下容器内看到的 PGHOST 不一定能从宿主机访问）

set -euo pipefail

SERVER_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$SERVER_DIR"

SRC_DB="${SRC_DB:-laiyipao}"
DST_DB="${DST_DB:-${SRC_DB}_restore_drill}"
PGHOST="${PGHOST:-127.0.0.1}"
PGPORT="${PGPORT:-5432}"
PGUSER="${PGUSER:-postgres}"
PG_CONTAINER="${PG_CONTAINER:-}"
KEEP_DUMP="${KEEP_DUMP:-}"

# 安全阀：只允许对「名字里带 drill」的库执行 DROP/CREATE。
case "$DST_DB" in
  *drill*) ;;
  *) echo "拒绝执行：DST_DB 必须包含 'drill'（当前：$DST_DB）" >&2; exit 2 ;;
esac

psql_run() {
  if [ -n "$PG_CONTAINER" ]; then
    docker exec -i "$PG_CONTAINER" psql -U "$PGUSER" -h "$PGHOST" -p "$PGPORT" "$@"
  else
    psql -U "$PGUSER" -h "$PGHOST" -p "$PGPORT" "$@"
  fi
}

dump_run() {
  if [ -n "$PG_CONTAINER" ]; then
    docker exec -i "$PG_CONTAINER" pg_dump -U "$PGUSER" -h "$PGHOST" -p "$PGPORT" "$@"
  else
    pg_dump -U "$PGUSER" -h "$PGHOST" -p "$PGPORT" "$@"
  fi
}

TMP_DIR="$(mktemp -d)"
DUMP_FILE="$TMP_DIR/${SRC_DB}.sql"
API_PID=""
stop_api() {
  if [ -n "$API_PID" ]; then
    kill "$API_PID" 2>/dev/null || true
    wait "$API_PID" 2>/dev/null || true
    API_PID=""
  fi
}
cleanup() {
  stop_api
  if [ -z "$KEEP_DUMP" ]; then rm -rf "$TMP_DIR"; else echo "dump 保留在：$DUMP_FILE"; fi
}
trap cleanup EXIT

echo "[drill] 源库=$SRC_DB 目标库=$DST_DB 容器=${PG_CONTAINER:-<本机>}"

echo "[1/6] pg_dump 源库..."
START=$(date +%s)
dump_run --no-owner --no-privileges --format=plain "$SRC_DB" > "$DUMP_FILE"
DUMP_SECS=$(( $(date +%s) - START ))
DUMP_BYTES=$(wc -c < "$DUMP_FILE" | tr -d ' ')
echo "      dump 用时=${DUMP_SECS}s 大小=${DUMP_BYTES}B"

echo "[2/6] 重建空库..."
# FORCE：上一次演练的服务/连接可能还挂着（例如刚停掉的 API 池），
# 普通 DROP 会因「being accessed by other users」直接失败。PG13+ 支持。
psql_run -d postgres -v ON_ERROR_STOP=1 -q \
  -c "DROP DATABASE IF EXISTS \"$DST_DB\" WITH (FORCE)" \
  -c "CREATE DATABASE \"$DST_DB\""

echo "[3/6] 恢复..."
START=$(date +%s)
psql_run -d "$DST_DB" -v ON_ERROR_STOP=1 -q < "$DUMP_FILE" > /dev/null
RESTORE_SECS=$(( $(date +%s) - START ))
echo "      恢复用时=${RESTORE_SECS}s"

echo "[4/6] 逐表比对行数..."
TABLES=$(psql_run -At -d "$SRC_DB" -c \
  "SELECT tablename FROM pg_tables WHERE schemaname='public' ORDER BY 1")
COUNT_SQL=""
for t in $TABLES; do
  COUNT_SQL+="SELECT '$t', COUNT(*) FROM \"$t\" UNION ALL "
done
# UNION ALL 不保证输出顺序，必须显式 ORDER BY，否则两侧顺序不同会误报。
COUNT_SQL="${COUNT_SQL% UNION ALL } ORDER BY 1"

psql_run -At -d "$SRC_DB" -c "$COUNT_SQL" > "$TMP_DIR/src_counts.txt"
psql_run -At -d "$DST_DB" -c "$COUNT_SQL" > "$TMP_DIR/dst_counts.txt"

if ! diff -u "$TMP_DIR/src_counts.txt" "$TMP_DIR/dst_counts.txt" > "$TMP_DIR/counts.diff"; then
  echo "[FAIL] 行数不一致：" >&2
  sed 's/^/      /' "$TMP_DIR/counts.diff" >&2
  exit 1
fi
TABLE_COUNT=$(echo "$TABLES" | wc -l | tr -d ' ')
TOTAL_ROWS=$(awk -F'|' '{s+=$2} END {print s+0}' "$TMP_DIR/src_counts.txt")
echo "      表=${TABLE_COUNT} 总行数=${TOTAL_ROWS}"

echo "[5/6] 比对约束、索引、时区与迁移版本..."
METRICS_SQL="SELECT
  (SELECT COUNT(*) FROM pg_constraint WHERE contype='f') || '|' ||
  (SELECT COUNT(*) FROM pg_indexes WHERE schemaname='public') || '|' ||
  current_setting('TimeZone') || '|' ||
  (SELECT version_id FROM goose_db_version ORDER BY version_id DESC LIMIT 1)"
SRC_METRICS=$(psql_run -At -d "$SRC_DB" -c "$METRICS_SQL")
DST_METRICS=$(psql_run -At -d "$DST_DB" -c "$METRICS_SQL")
IFS='|' read -r SRC_FK SRC_IDX SRC_TZ SRC_VER <<< "$SRC_METRICS"
IFS='|' read -r DST_FK DST_IDX DST_TZ DST_VER <<< "$DST_METRICS"
if [ "$SRC_METRICS" != "$DST_METRICS" ]; then
  echo "[FAIL] 约束/索引/时区/版本不一致：源=$SRC_METRICS 恢复库=$DST_METRICS" >&2
  exit 1
fi

echo "[6/6] 恢复库真玩一遍（玩家 + 管理端 + 写侧触发器）..."
# 临时 API 指向恢复库；MIGRATIONS_ON_BOOT=false 很关键：
# 若让它在启动时自动迁移，一份「版本落后」的坏 dump 会被悄悄补好，
# 第 5 步的 goose 比对也就白做了。
DRILL_PGHOST="${DRILL_PGHOST:-127.0.0.1}"
DRILL_API_PORT="${DRILL_API_PORT:-$((18080 + ($$ % 500)))}"
DST_DSN="postgres://${PGUSER}@${DRILL_PGHOST}:${PGPORT}/${DST_DB}?sslmode=disable"
API_BIN="$TMP_DIR/api"
if ! go build -o "$API_BIN" ./cmd/api; then
  echo "[FAIL] 编译临时 API 失败" >&2
  exit 1
fi
DATABASE_URL="$DST_DSN" \
  MIGRATIONS_ON_BOOT=false \
  DASHBOARD_CACHE_TTL=0 \
  ADDR="127.0.0.1:$DRILL_API_PORT" \
  JWT_SECRET="drill-only-rotate-me" \
  "$API_BIN" > "$TMP_DIR/api.log" 2>&1 &
API_PID=$!

READY=""
for _ in $(seq 1 60); do
  if curl -fsS "http://127.0.0.1:$DRILL_API_PORT/readyz" >/dev/null 2>&1; then READY=1; break; fi
  sleep 0.5
done
if [ "$READY" != "1" ]; then
  echo "[FAIL] 恢复库上的临时 API 30s 内没通过 /readyz" >&2
  tail -n 40 "$TMP_DIR/api.log" >&2 || true
  exit 1
fi

BASE_URL="http://127.0.0.1:$DRILL_API_PORT" \
  DATABASE_URL="$DST_DSN" \
  EXPECTED_TZ="$SRC_TZ" \
  MIGRATIONS_DIR="$SERVER_DIR/migrations" \
  "$SERVER_DIR/scripts/smoke_after_restore.sh"
stop_api

echo "[PASS] 恢复演练通过"
echo "      外键=${DST_FK} 公共索引=${DST_IDX} 时区=${DST_TZ} goose版本=${DST_VER}"
echo "      dump=${DUMP_BYTES}B dump耗时=${DUMP_SECS}s 恢复耗时=${RESTORE_SECS}s"
