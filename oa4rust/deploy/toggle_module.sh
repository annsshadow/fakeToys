#!/usr/bin/env bash
# ──────────────────────────────────────────────────────────────────────────────
# OA4Rust U9 — 模块路由切换脚本 (toggle_module.sh)
#
# 通过修改 MODULE_ROUTING 环境变量实现按模块在 Rust / Java 之间灰度切换，
# 并生成 nginx include 文件实现流量比例调整（10% → 50% → 100%）。
#
# 用法:
#   ./toggle_module.sh status                            # 查看当前路由分配
#   ./toggle_module.sh set rust attendance control        # 模块全量切到 Rust
#   ./toggle_module.sh set java  message calendar         # 模块全量切到 Java
#   ./toggle_module.sh gray attendance 10 control 50      # 按比例灰度（模块取值见 DEFAULT_GRAY_MODULES）
#   ./toggle_module.sh rollback                           # 回滚到上一状态
#   ./toggle_module.sh reset                              # 清空（全部默认 Rust）
#
# 输出文件:
#   .module_routing.env  — MODULE_ROUTING 环境变量（source 后重启生效）
#   gray-routes.conf     — nginx include 文件（存放灰度 location 块）
#   .gray_state          — 灰度状态快照（用于回滚）
#
# 注意: 流量比例调整通过修改 nginx include 文件实现，需 nginx -s reload 生效。
#       配合 nginx.conf 中的 o2server_backend fallback 实现 5 分钟 RTO 回滚。
# ──────────────────────────────────────────────────────────────────────────────
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
PROJECT_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"
ENV_FILE="$PROJECT_DIR/.module_routing.env"
STATE_FILE="$PROJECT_DIR/.gray_state"
GRAY_CONF="$SCRIPT_DIR/gray-routes.conf"
BACKUP_DIR="$SCRIPT_DIR/.backups"
NGINX_CONF="${NGINX_CONF:-/etc/nginx/nginx.conf}"
NGINX_INCLUDE_LINE="        # include /etc/nginx/conf.d/gray-routes.conf;"

# 默认灰度模块
DEFAULT_GRAY_MODULES=(attendance control express meeting processplatform bam)

cmd="${1:-status}"
if [[ $# -gt 0 ]]; then shift; fi

# ──────────────────────────────────────────────────────────────────────────────
# 工具函数
# ──────────────────────────────────────────────────────────────────────────────

ensure_backup_dir() {
    mkdir -p "$BACKUP_DIR"
}

backup_file() {
    local file="$1"
    if [[ -f "$file" ]]; then
        local ts
        ts=$(date -u +%Y%m%d%H%M%S)
        cp "$file" "$BACKUP_DIR/$(basename "$file").$ts"
    fi
}

save_state() {
    local action="$1"
    local detail="$2"
    cat > "$STATE_FILE" <<EOF
action=$action
detail=$detail
timestamp=$(date -u +%Y-%m-%dT%H:%M:%SZ)
EOF
}

load_state() {
    if [[ -f "$STATE_FILE" ]]; then
        cat "$STATE_FILE"
    else
        echo "无历史状态记录"
    fi
}

nginx_test_and_reload() {
    echo "验证 nginx 配置..."
    if command -v nginx >/dev/null 2>&1; then
        if nginx -t 2>/dev/null; then
            echo "重载 nginx..."
            nginx -s reload 2>/dev/null || echo "请手动执行: sudo nginx -s reload"
        else
            echo "警告：nginx 配置验证失败，请检查 $NGINX_CONF"
            return 1
        fi
    else
        echo "未检测到 nginx 命令，请手动验证并重载配置。"
    fi
}

# ──────────────────────────────────────────────────────────────────────────────
# 命令处理
# ──────────────────────────────────────────────────────────────────────────────

case "$cmd" in
    status)
        echo "=== MODULE_ROUTING ==="
        if [[ -f "$ENV_FILE" ]]; then
            grep -E '^MODULE_ROUTING=' "$ENV_FILE" || echo "(未设置，全部默认 Rust)"
        else
            echo "未找到 $ENV_FILE，全部模块默认路由到 Rust。"
        fi

        echo ""
        echo "=== 灰度状态 ==="
        if [[ -f "$STATE_FILE" ]]; then
            load_state
        else
            echo "无灰度状态记录"
        fi

        echo ""
        echo "=== 灰度配置 ==="
        if [[ -f "$GRAY_CONF" ]]; then
            echo "灰度配置已生成: $GRAY_CONF"
            echo "--- upstream weight ---"
            grep -E "weight=" "$GRAY_CONF" | head -4 || true
        else
            echo "未生成灰度配置（默认 100% Rust）"
        fi
        ;;

    set)
        direction="${1:?需要 rust 或 java}"
        shift
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

        # 备份并写入
        backup_file "$ENV_FILE"
        echo "MODULE_ROUTING=$out" > "$ENV_FILE"

        # 清空灰度状态（全量切流时清除灰度配置）
        if [[ -f "$GRAY_CONF" ]]; then
            backup_file "$GRAY_CONF"
            rm -f "$GRAY_CONF"
        fi
        rm -f "$STATE_FILE"

        echo "已写入 $ENV_FILE: MODULE_ROUTING=$out"
        echo "请 source 该文件并重启 oa4rust 进程使切换生效。"
        ;;

    gray)
        # 按比例灰度
        # 用法: ./toggle_module.sh gray <module> <ratio> [module2 ratio2 ...]
        # 例: ./toggle_module.sh gray attendance 10 control 50
        # 模块取值参考 DEFAULT_GRAY_MODULES（缺参时会在报错中列出）

        # 解析参数（模块和比例交替）
        declare -A ratios
        declare -a gray_modules=()
        while [[ $# -gt 0 ]]; do
            module="$1"
            ratio="$2"
            if [[ -z "$ratio" ]]; then
                echo "错误：模块 '$module' 缺少比例参数"
                exit 1
            fi
            if [[ ! "$ratio" =~ ^[0-9]+$ ]] || (( ratio < 1 || ratio > 100 )); then
                echo "错误：比例 '$ratio' 必须是 1-100 的整数"
                exit 1
            fi
            ratios["$module"]="$ratio"
            gray_modules+=("$module")
            shift 2
        done
        [[ ${#gray_modules[@]} -eq 0 ]] && { echo "请指定至少一个模块和比例（可用模块: ${DEFAULT_GRAY_MODULES[*]}）"; exit 1; }

        ensure_backup_dir

        # 备份当前灰度配置
        backup_file "$GRAY_CONF"

        # 更新 MODULE_ROUTING（灰度模块设为 rust）
        declare -A map
        if [[ -f "$ENV_FILE" ]]; then
          cur=$(grep -E '^MODULE_ROUTING=' "$ENV_FILE" | tail -1 | cut -d= -f2-)
          IFS=',' read -ra parts <<< "$cur"
          for p in "${parts[@]}"; do
            m="${p%%:*}"; t="${p##*:}"
            map["$m"]="$t"
          done
        fi
        for m in "${gray_modules[@]}"; do
          map["$m"]="rust"
        done
        out=""
        for k in "${!map[@]}"; do
          [[ -n "$out" ]] && out="$out,"
          out="${out}${k}:${map[$k]}"
        done
        echo "MODULE_ROUTING=$out" > "$ENV_FILE"

        # 生成灰度 nginx include 文件
        # 使用统一 upstream，weight 由脚本动态调整
        {
            echo "# 自动生成的灰度比例配置（由 toggle_module.sh 管理）"
            echo "# 生成时间：$(date -u +%Y-%m-%dT%H:%M:%SZ)"
            echo "# 生效方式：在 nginx.conf 中取消注释 include 此文件，然后 nginx -s reload"
            echo ""
            echo "upstream oa4rust_gray {"
            for m in "${gray_modules[@]}"; do
                ratio="${ratios[$m]}"
                echo "    # 模块 $m: ${ratio}% Rust / $((100 - ratio))% Java"
                echo "    server 127.0.0.1:3000 weight=${ratio};"
                echo "    server 127.0.0.1:8080 weight=$((100 - ratio));"
            done
            echo "}"
            echo ""
            for m in "${gray_modules[@]}"; do
                ratio="${ratios[$m]}"
                echo "# 模块 $m（灰度比例：${ratio}% Rust）"
                echo "location ^~ /jaxrs/${m}/ {"
                echo "    limit_req zone=api burst=20 nodelay;"
                echo "    proxy_pass http://oa4rust_gray;"
                echo "    proxy_set_header Host \$host;"
                echo "    proxy_set_header X-Real-IP \$remote_addr;"
                echo "    proxy_set_header X-Forwarded-For \$proxy_add_x_forwarded_for;"
                echo "    proxy_set_header X-Forwarded-Proto \$scheme;"
                echo "    proxy_next_upstream error timeout http_500 http_502 http_503;"
                echo "    proxy_connect_timeout 5s;"
                echo "    proxy_send_timeout 10s;"
                echo "    proxy_read_timeout 10s;"
                echo "}"
                echo ""
            done
        } > "$GRAY_CONF"

        # 保存状态
        ratios_str=""
        for m in "${gray_modules[@]}"; do
            [[ -n "$ratios_str" ]] && ratios_str="$ratios_str,"
            ratios_str="${ratios_str}${m}=${ratios[$m]}"
        done
        save_state "gray" "$ratios_str"

        echo "灰度配置已生成：$GRAY_CONF"
        echo ""
        echo "下一步操作："
        echo "  1. 确保 nginx.conf 中已取消注释以下行："
        echo "     $NGINX_INCLUDE_LINE"
        echo "  2. 验证并重载 nginx："
        echo "     sudo nginx -t && sudo nginx -s reload"
        echo ""
        echo "当前灰度比例："
        for m in "${gray_modules[@]}"; do
            ratio="${ratios[$m]}"
            echo "  $m: ${ratio}% Rust / $((100 - ratio))% Java"
        done
        ;;

    rollback)
        if [[ ! -f "$STATE_FILE" ]]; then
            echo "错误：未找到状态文件 $STATE_FILE，无法回滚。"
            exit 1
        fi

        ensure_backup_dir

        # 读取上一状态
        last_action=$(grep '^action=' "$STATE_FILE" | cut -d= -f2-)
        last_detail=$(grep '^detail=' "$STATE_FILE" | cut -d= -f2-)
        last_ts=$(grep '^timestamp=' "$STATE_FILE" | cut -d= -f2-)

        echo "回滚到上一状态："
        echo "  操作: $last_action"
        echo "  详情: $last_detail"
        echo "  时间: $last_ts"
        echo ""

        # 恢复 MODULE_ROUTING
        if [[ -f "$ENV_FILE" ]]; then
            backup_file "$ENV_FILE"
        fi
        # 全量切回 Rust（最安全的回滚）
        echo "MODULE_ROUTING=" > "$ENV_FILE"

        # 恢复灰度配置（删除）
        if [[ -f "$GRAY_CONF" ]]; then
            backup_file "$GRAY_CONF"
            rm -f "$GRAY_CONF"
        fi

        # 清空状态
        rm -f "$STATE_FILE"

        echo "已回滚到安全状态：全部模块路由到 Rust。"
        echo ""
        echo "下一步操作："
        echo "  1. source $ENV_FILE 并重启 oa4rust 进程"
        echo "  2. 验证 nginx.conf 中已注释或移除 include 行"
        echo "  3. 执行：sudo nginx -s reload"
        ;;

    reset)
        if [[ -f "$ENV_FILE" ]]; then
            backup_file "$ENV_FILE"
            rm -f "$ENV_FILE"
        fi
        if [[ -f "$GRAY_CONF" ]]; then
            backup_file "$GRAY_CONF"
            rm -f "$GRAY_CONF"
        fi
        rm -f "$STATE_FILE"
        echo "已重置：移除 $ENV_FILE 和 $GRAY_CONF，全部模块默认路由到 Rust。"
        ;;

    *)
        echo "未知命令: $cmd"
        echo ""
        echo "可用命令:"
        echo "  status                   查看当前路由分配"
        echo "  set <rust|java> <模块...> 设置模块路由方向"
        echo "  gray <模块> <比例> [...]  按比例灰度（比例: 1-100，可用模块见 DEFAULT_GRAY_MODULES）"
        echo "  rollback                 回滚到上一状态"
        echo "  reset                    重置为默认状态"
        exit 1 ;;
esac
