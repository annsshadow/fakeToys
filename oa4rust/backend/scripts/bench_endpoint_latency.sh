#!/usr/bin/env bash
# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later
#
# 关键接口延迟基准：对同一台后端二进制起服务后，对某无鉴权端点采样 N 次
# curl 往返耗时，输出 p50/p95/p99。传入 baseline 与 optimized 两个二进制路径即可
# 得到发布档（thin-LTO）优化「前 vs 后」的运行时延迟对比。
#
# 依赖：本机 Postgres 监听 5432（DATABASE_URL 默认
#       postgres://o2server:password@localhost:5432/oa4rust，服务启动即迁移）。
# 用法：bash scripts/bench_endpoint_latency.sh <baseline_bin> <optimized_bin> [N] [endpoint]
set -uo pipefail
cd "$(dirname "$0")/.."

BASE_BIN="${1:?need baseline binary path}"
OPT_BIN="${2:?need optimized binary path}"
N="${3:-500}"
ENDPOINT="${4:-/api/authentication/captcha}"   # 无鉴权、CPU 密集（验证码图像生成），LTO 敏感
PORT=3000
BASE_URL="http://127.0.0.1:${PORT}"

kill_port() { # 杀掉占用 PORT 的残留进程
  local pids
  pids=$(netstat -ano 2>/dev/null | grep ":${PORT} " | grep LISTENING | awk '{print $NF}' | sort -u)
  for p in $pids; do taskkill //PID "$p" //F >/dev/null 2>&1 || true; done
  sleep 1
}

wait_ready() { # 轮询直到端点返回 HTTP 200，或 60s 超时
  for _ in $(seq 1 120); do
    local code
    code=$(curl -s -o /dev/null -w '%{http_code}' "${BASE_URL}${ENDPOINT}" 2>/dev/null || echo 000)
    [[ "$code" == "200" ]] && return 0
    sleep 0.5
  done
  return 1
}

percentiles() { # stdin: 每行一个耗时(秒)；输出 p50/p95/p99/mean(ms)
  sort -n | awk '{a[NR]=$1} END{
    if(NR==0){print "no-samples"; exit}
    s=0; for(i=1;i<=NR;i++) s+=a[i];
    printf "n=%d  mean=%.2fms  p50=%.2fms  p95=%.2fms  p99=%.2fms",
      NR, s/NR*1000, a[int(NR*0.50)]*1000, a[int(NR*0.95)]*1000, a[int(NR*0.99)]*1000
  }'
}

run_one() { # $1=label $2=bin
  local label="$1" bin="$2"
  kill_port
  echo "[$label] 启动 $bin"
  "$bin" >"target-bench/server-${label}.log" 2>&1 &
  local srv=$!
  if ! wait_ready; then
    echo "[$label] 服务未就绪（见 target-bench/server-${label}.log）"; kill_port; return 1
  fi
  # 预热
  for _ in $(seq 1 50); do curl -s -o /dev/null "${BASE_URL}${ENDPOINT}" || true; done
  # 采样
  local tmp="target-bench/lat-${label}.txt"; : >"$tmp"
  for _ in $(seq 1 "$N"); do
    curl -s -o /dev/null -w '%{time_total}\n' "${BASE_URL}${ENDPOINT}" >>"$tmp" 2>/dev/null || true
  done
  printf '[%s] %s\n' "$label" "$(percentiles <"$tmp")"
  kill_port
}

mkdir -p target-bench
echo "== 端点：${ENDPOINT}  采样 N=${N} =="
run_one baseline  "$BASE_BIN"
run_one optimized "$OPT_BIN"
echo "日志：target-bench/server-*.log  原始耗时：target-bench/lat-*.txt"
