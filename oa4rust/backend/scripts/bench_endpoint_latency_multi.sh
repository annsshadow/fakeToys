#!/usr/bin/env bash
# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later
#
# 多轮 · 多端点 关键接口延迟基准（强化 bench_endpoint_latency.sh 的单轮版）。
# 对 baseline 与 optimized 两档二进制，各自对一组无鉴权端点做 R 轮 × N 次 curl 采样，
# 报告每端点每档的「各轮 p50/p95 的中位数」，降低单轮噪声。
#
# 依赖：本机 Postgres 5432（服务默认连 postgres://o2server:password@localhost:5432/oa4rust）。
# 用法：bash scripts/bench_endpoint_latency_multi.sh <baseline_bin> <optimized_bin> [N] [ROUNDS]
set -uo pipefail
cd "$(dirname "$0")/.."

BASE_BIN="${1:?need baseline binary path}"
OPT_BIN="${2:?need optimized binary path}"
N="${3:-300}"
ROUNDS="${4:-5}"
PORT=3000
BASE_URL="http://127.0.0.1:${PORT}"
CANDIDATES=(
  "/api/authentication/captcha"
  "/openapi.json"
  "/api/authentication/captcha/width/200/height/80"
)

kill_port() {
  local pids
  pids=$(netstat -ano 2>/dev/null | grep ":${PORT} " | grep LISTENING | awk '{print $NF}' | sort -u)
  for p in $pids; do taskkill //PID "$p" //F >/dev/null 2>&1 || true; done
  sleep 1
}
http_code() { curl -s -o /dev/null -w '%{http_code}' "${BASE_URL}$1" 2>/dev/null || echo 000; }
wait_ready() { for _ in $(seq 1 120); do [[ "$(http_code /api/authentication/captcha)" == "200" ]] && return 0; sleep 0.5; done; return 1; }
pctl() { # $1=file $2=frac -> ms
  sort -n "$1" | awk -v f="$2" '{a[NR]=$1} END{if(NR==0){print "-1";exit} printf "%.2f", a[int(NR*f)]*1000}'
}
median() { sort -n | awk '{a[NR]=$1} END{if(NR==0){print"-";exit} printf "%.2f", (NR%2? a[int(NR/2)+1] : (a[NR/2]+a[NR/2+1])/2)}'; }

run_bin() { # $1=label $2=bin ; sets global results
  local label="$1" bin="$2"
  kill_port
  "$bin" >"target-bench/srv-${label}.log" 2>&1 &
  if ! wait_ready; then echo "[$label] 未就绪"; kill_port; return 1; fi
  for _ in $(seq 1 50); do curl -s -o /dev/null "${BASE_URL}/api/authentication/captcha" || true; done
  for ep in "${ACTIVE_EPS[@]}"; do
    local p50s="" p95s=""
    for _ in $(seq 1 "$ROUNDS"); do
      local tmp; tmp=$(mktemp)
      for _ in $(seq 1 "$N"); do curl -s -o /dev/null -w '%{time_total}\n' "${BASE_URL}${ep}" >>"$tmp" 2>/dev/null || true; done
      p50s+="$(pctl "$tmp" 0.50)"$'\n'; p95s+="$(pctl "$tmp" 0.95)"$'\n'
      rm -f "$tmp"
    done
    printf '%-45s %-10s p50(中位)=%sms  p95(中位)=%sms\n' "$ep" "$label" "$(printf '%s' "$p50s" | median)" "$(printf '%s' "$p95s" | median)"
  done
  kill_port
}

mkdir -p target-bench
# 探测哪些候选端点在本机返回 200
kill_port
"$OPT_BIN" >"target-bench/srv-probe.log" 2>&1 &
wait_ready || { echo "探测服务未就绪"; kill_port; exit 1; }
ACTIVE_EPS=()
for ep in "${CANDIDATES[@]}"; do c=$(http_code "$ep"); echo "probe $ep -> $c"; [[ "$c" == "200" ]] && ACTIVE_EPS+=("$ep"); done
kill_port
echo "== 采样端点: ${ACTIVE_EPS[*]}  (N=${N} × ROUNDS=${ROUNDS}) =="
run_bin baseline  "$BASE_BIN"
run_bin optimized "$OPT_BIN"
