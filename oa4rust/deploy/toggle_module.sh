#!/usr/bin/env bash
# ──────────────────────────────────────────────────────────────────────────────
# OA4Rust U9 — 模块路由切换脚本 (toggle_module.sh)
#
# 通过修改 MODULE_ROUTING 环境变量实现按模块在 Rust / Java 之间灰度切换，
# 无需改动 nginx 配置即可即时切流（配合 shared::middleware::ModuleRouting）。
#
# 用法:
#   ./toggle_module.sh status            # 查看当前路由分配
#   ./toggle_module.sh set rust attendance control
#   ./toggle_module.sh set java  message calendar
#   ./toggle_module.sh reset              # 清空（全部默认 Rust）
#
# 注意: 该脚本仅修改环境变量文件（./.module_routing.env），需由部署流程
#       source 后重启 oa4rust 进程生效；配合 nginx.conf 的 java_backend fallback
#       实现 5 分钟 RTO 回滚。
# ──────────────────────────────────────────────────────────────────────────────
set -euo pipefail

ENV_FILE="$(cd "$(dirname "$0")/.." && pwd)/.module_routing.env"

cmd="${1:-status}"

case "$cmd" in
  status)
    if [[ -f "$ENV_FILE" ]]; then
      echo "当前 MODULE_ROUTING:"
      grep -E '^MODULE_ROUTING=' "$ENV_FILE" || echo "(未设置，全部默认 Rust)"
    else
      echo "未找到 $ENV_FILE，全部模块默认路由到 Rust。"
    fi
    ;;
  set)
    direction="${2:?需要 rust 或 java}"
    shift 2
    modules=("$@")
    [[ ${#modules[@]} -eq 0 ]] && { echo "请指定至少一个模块"; exit 1; }
    # 读取已有分配（除目标方向外的保留）
    declare -A map
    if [[ -f "$ENV_FILE" ]]; then
      cur=$(grep -E '^MODULE_ROUTING=' "$ENV_FILE" | tail -1 | cut -d= -f2-)
      IFS=',' read -ra parts <<< "$cur"
      for p in "${parts[@]}"; do
        m="${p%%:*}"; t="${p##*:}"
        map["$m"]="$t"
      done
    fi
    for m in "${modules[@]}"; do
      map["$m"]="$direction"
    done
    out=""
    for k in "${!map[@]}"; do
      [[ -n "$out" ]] && out="$out,"
      out="${out}${k}:${map[$k]}"
    done
    echo "MODULE_ROUTING=$out" > "$ENV_FILE"
    echo "已写入 $ENV_FILE: MODULE_ROUTING=$out"
    echo "请 source 该文件并重启 oa4rust 进程使切换生效。"
    ;;
  reset)
    rm -f "$ENV_FILE"
    echo "已重置：移除 $ENV_FILE，全部模块默认路由到 Rust。"
    ;;
  *)
    echo "未知命令: $cmd"; exit 1 ;;
esac
