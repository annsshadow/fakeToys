#!/usr/bin/env bash
# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later
#
# 可复现的发布构建基准：对比 [profile.release] thin-LTO/strip 优化「前 vs 后」的
# 二进制体积与构建耗时。两次构建取自同一份当前源码，仅 profile 参数不同 —— 受控 A/B。
#
# 基线（baseline）= 无 [profile.release] 时的 Cargo 默认发布档：
#   opt-level=3, codegen-units=16, lto=false, strip=false
# 优化（optimized）= 本仓提交的档：opt-level=3, lto="thin", codegen-units=1, strip="symbols"
#
# 用法：  bash scripts/bench_release_profile.sh
# 产物：  两个二进制分别落在 target-bench/{baseline,optimized}/release/ 下；
#         体积与耗时打印到 stdout，并写入 docs/audits/release-profile-benchmark-<date>.md 之外的
#         临时汇总（本脚本只测量与打印，不修改仓库文件）。
set -euo pipefail
cd "$(dirname "$0")/.."

BIN=oa4rust
BASE_DIR=target-bench/baseline
OPT_DIR=target-bench/optimized

bin_path() { # $1 = target dir
  if [[ -f "$1/release/${BIN}.exe" ]]; then echo "$1/release/${BIN}.exe";
  else echo "$1/release/${BIN}"; fi
}
size_bytes() { stat -c %s "$1" 2>/dev/null || echo 0; }

echo "== [1/2] 基线构建（默认发布档：codegen-units=16, lto=false, strip=false）=="
t0=$(date +%s)
CARGO_INCREMENTAL=0 cargo build --release --bin "$BIN" \
  --target-dir "$BASE_DIR" \
  --config 'profile.release.lto=false' \
  --config 'profile.release.codegen-units=16' \
  --config 'profile.release.strip=false' >/dev/null
base_secs=$(( $(date +%s) - t0 ))

echo "== [2/2] 优化构建（本仓提交档：thin-LTO, codegen-units=1, strip=symbols）=="
t0=$(date +%s)
CARGO_INCREMENTAL=0 cargo build --release --bin "$BIN" \
  --target-dir "$OPT_DIR" >/dev/null
opt_secs=$(( $(date +%s) - t0 ))

base_bin=$(bin_path "$BASE_DIR");  opt_bin=$(bin_path "$OPT_DIR")
base_sz=$(size_bytes "$base_bin"); opt_sz=$(size_bytes "$opt_bin")
# .pdb（仅 Windows/MSVC）
base_pdb=$(size_bytes "${BASE_DIR}/release/${BIN}.pdb")
opt_pdb=$(size_bytes "${OPT_DIR}/release/${BIN}.pdb")

echo
echo "================ 结果 ================"
printf '%-14s %-16s %-16s %-10s\n' "指标" "baseline" "optimized" "Δ"
printf '%-14s %-16s %-16s %-10s\n' "二进制字节" "$base_sz" "$opt_sz" \
  "$(awk -v a="$base_sz" -v b="$opt_sz" 'BEGIN{if(a>0)printf "%+.1f%%",(b-a)*100/a; else print "n/a"}')"
printf '%-14s %-16s %-16s\n' ".pdb 字节" "$base_pdb" "$opt_pdb"
printf '%-14s %-16s %-16s %-10s\n' "构建秒" "$base_secs" "$opt_secs" \
  "$(awk -v a="$base_secs" -v b="$opt_secs" 'BEGIN{if(a>0)printf "%+.1f%%",(b-a)*100/a; else print "n/a"}')"
echo "baseline bin : $base_bin"
echo "optimized bin: $opt_bin"
echo "====================================="
