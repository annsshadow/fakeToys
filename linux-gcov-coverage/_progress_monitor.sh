#!/usr/bin/env bash
# 修正版进度监视器：监控 x86_64 gcov 构建容器，每 5 分钟把 docker 日志尾部写入 progress_feed.log。
# 用法: _progress_monitor.sh [container_name]
set -uo pipefail

CONTAINER="${1:-x86-gcov-run4}"
FEED=/d/WORKSPACE/linux-7.1.3/tools/testing/coverage/progress_feed.log
: > "$FEED"

echo "$(date '+%H:%M:%S') [monitor-start] 容器=$CONTAINER" >> "$FEED"

# Grace period: 容器启动后前 ~15s 可能尚未产生日志。
sleep 15

for i in $(seq 1 120); do
  TS=$(date '+%H:%M:%S')
  if docker ps --format '{{.Names}}' 2>/dev/null | grep -qx "$CONTAINER"; then
    RUNNING=1
  else
    RUNNING=0
  fi
  LAST=$(docker logs --tail 3 "$CONTAINER" 2>&1 | tr '\n' '|')
  if [ "$RUNNING" -eq 0 ]; then
    echo "$TS [容器已退出] $LAST" >> "$FEED"
    break
  fi
  echo "$TS [运行中] $LAST" >> "$FEED"
  sleep 300
done
echo "$(date '+%H:%M:%S') [monitor-done]" >> "$FEED"
