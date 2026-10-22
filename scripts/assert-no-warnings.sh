#!/usr/bin/env bash
# 构建日志的「0 告警」断言。
#
# 用法：assert-no-warnings.sh <build.log> [标签]
#
# 为什么需要它：Vite / Rollup / lightningcss 的告警**不改变退出码**，
# 构建照样 exit 0。只检查退出码的门禁看不见告警，告警就会一路堆到没人看。
#
# 判据是日志里的告警行形态，覆盖本仓库实际出现过的三类：
#   1. `(!) ...`                        —— Rollup / Rolldown 的告警前缀
#   2. `[lightningcss minify] ...`      —— CSS 压缩阶段的告警
#   3. `warning: ...`                   —— 其他工具（tsc 之外）的告警行
#
# 注意：不要用 VITE_CONFIG_NATIVE_IGNORE_WARNING 之类的开关压掉告警，
# 那是把门禁关掉，不是修问题。
set -euo pipefail

log="${1:?用法: assert-no-warnings.sh <build.log> [标签]}"
label="${2:-build}"

if [ ! -f "$log" ]; then
  echo "::error::日志文件不存在：$log"
  exit 2
fi

# 必须先剥掉 ANSI 颜色码：Vite/Rollup 的告警行在终端里是 "\033[33m(!) ..."，
# 直接按 ^(!) 匹配会全部漏掉（实测就是这样漏的）。
# 用管道而不是临时文件：临时文件在受管环境里删除会失败，反而把退出码带脏。
hits="$(sed $'s/\x1b\\[[0-9;]*m//g' "$log" | grep -inE '^[(]!|\[lightningcss|(^|[[:space:]])warning:' || true)"
if [ -n "$hits" ]; then
  count="$(printf '%s\n' "$hits" | wc -l | tr -d ' ')"
  echo "::error::${label} 产生了 ${count} 条编译告警（门禁是 0）："
  printf '%s\n' "$hits"
  echo "::error::请修掉上述告警本身；不要用 ignore 开关把它压掉。"
  exit 1
fi

echo "${label}: 0 告警"
