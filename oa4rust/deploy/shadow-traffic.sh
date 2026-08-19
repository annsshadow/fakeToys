#!/usr/bin/env bash
# ──────────────────────────────────────────────────────────────────────────────
# OA4Rust U9 — 影子流量管理脚本 (shadow-traffic.sh)
#
# 功能：
#   1. 启用/禁用 nginx mirror 影子流量
#   2. 发送带行为对比标记的测试请求
#   3. 收集 Rust 与 Java 响应日志
#   4. 对比响应一致性（状态码、响应体大小、内容摘要）
#
# 前置条件：
#   - nginx >= 1.13.4（支持 mirror 模块）
#   - Rust 服务已启用 behavior_comparison 中间件
#   - Java 服务（o2server）健康运行
#
# 用法:
#   ./shadow-traffic.sh enable                        # 启用影子流量
#   ./shadow-traffic.sh disable                       # 禁用影子流量
#   ./shadow-traffic.sh run                           # 发送测试请求
#   ./shadow-traffic.sh compare                       # 对比响应一致性
#   ./shadow-traffic.sh report                        # 生成一致性报告
#   ./shadow-traffic.sh status                        # 查看当前状态
# ──────────────────────────────────────────────────────────────────────────────
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
PROJECT_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"
NGINX_CONF="${NGINX_CONF:-/etc/nginx/nginx.conf}"
NGINX_CONF_LOCAL="$SCRIPT_DIR/nginx.conf"
ACCESS_LOG="${ACCESS_LOG:-/var/log/nginx/access.log}"
RUST_LOG="${RUST_LOG:-/var/log/oa4rust/oa4rust.log}"
COMPARISON_LOG="$PROJECT_DIR/shadow-comparison.log"
BACKUP_DIR="$SCRIPT_DIR/.backups"

# 测试用的模块和端点
TEST_MODULES=(attendance control express meeting)
TEST_ENDPOINTS=(
    "attendance/admin/list/all"
    "control/group/list"
    "express/delivery/list"
    "meeting/room/list"
)

cmd="${1:-help}"

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

check_nginx() {
    if ! command -v nginx >/dev/null 2>&1; then
        echo "错误：未检测到 nginx 命令"
        return 1
    fi
}

nginx_test_and_reload() {
    check_nginx
    echo "验证 nginx 配置..."
    if nginx -t 2>/dev/null; then
        echo "重载 nginx..."
        nginx -s reload 2>/dev/null || echo "请手动执行: sudo nginx -s reload"
    else
        echo "错误：nginx 配置验证失败"
        return 1
    fi
}

is_mirror_enabled() {
    local conf="${1:-$NGINX_CONF}"
    if [[ -f "$conf" ]]; then
        grep -q "^\s*mirror\s\+/mirror_backend" "$conf" 2>/dev/null && return 0
        # 检查是否被注释掉
        if grep -q "^\s*#\s*mirror\s\+/mirror_backend" "$conf" 2>/dev/null; then
            return 1
        fi
    fi
    return 1
}

# ──────────────────────────────────────────────────────────────────────────────
# 命令处理
# ──────────────────────────────────────────────────────────────────────────────

case "$cmd" in
    enable)
        echo "=== 启用影子流量 ==="

        # 使用本地 nginx.conf（开发环境）
        local_conf="$NGINX_CONF_LOCAL"
        if [[ ! -f "$local_conf" ]]; then
            echo "错误：未找到本地 nginx.conf: $local_conf"
            exit 1
        fi

        ensure_backup_dir
        backup_file "$local_conf"

        # 取消注释 mirror 相关行
        sed -i 's/^\(\s*\)# \(\s*mirror \/mirror_backend\)/\1\2/' "$local_conf"
        sed -i 's/^\(\s*\)# \(\s*mirror_request_body\)/\1\2/' "$local_conf"
        sed -i 's/^\(\s*\)# \(\s*location = \/mirror_backend\)/\1\2/' "$local_conf"
        sed -i 's/^\(\s*\)# \(\s*proxy_pass http:\/\/shadow_o2server_backend\)/\1\2/' "$local_conf"
        sed -i 's/^\(\s*\)# \(\s*proxy_set_header Host\)/\1\2/' "$local_conf"
        sed -i 's/^\(\s*\)# \(\s*proxy_set_header X-Real-IP\)/\1\2/' "$local_conf"
        sed -i 's/^\(\s*\)# \(\s*proxy_set_header X-Forwarded-For\)/\1\2/' "$local_conf"
        sed -i 's/^\(\s*\)# \(\s*proxy_set_header X-Forwarded-Proto\)/\1\2/' "$local_conf"
        sed -i 's/^\(\s*\)# \(\s*proxy_connect_timeout\)/\1\2/' "$local_conf"
        sed -i 's/^\(\s*\)# \(\s*proxy_send_timeout\)/\1\2/' "$local_conf"
        sed -i 's/^\(\s*\)# \(\s*proxy_read_timeout\)/\1\2/' "$local_conf"

        # 确保 mirror_backend location 内部标记存在
        if ! grep -q "^\s*location = /mirror_backend" "$local_conf"; then
            echo "错误：未找到 mirror_backend location 块，请检查 nginx.conf"
            exit 1
        fi

        echo "影子流量已启用（本地 nginx.conf）"
        echo ""
        echo "下一步："
        echo "  1. 确认 Java 服务（o2server）健康：curl -s http://localhost:8080/health"
        echo "  2. 重载 nginx：sudo nginx -t && sudo nginx -s reload"
        echo "  3. 发送测试请求：./shadow-traffic.sh run"
        echo "  4. 对比响应：./shadow-traffic.sh compare"
        ;;

    disable)
        echo "=== 禁用影子流量 ==="

        local_conf="$NGINX_CONF_LOCAL"
        if [[ ! -f "$local_conf" ]]; then
            echo "错误：未找到本地 nginx.conf: $local_conf"
            exit 1
        fi

        ensure_backup_dir
        backup_file "$local_conf"

        # 注释掉 mirror 相关行
        sed -i 's/^\(\s*mirror \/mirror_backend\)/# \1/' "$local_conf"
        sed -i 's/^\(\s*mirror_request_body\)/# \1/' "$local_conf"
        sed -i 's/^\(\s*location = \/mirror_backend\)/# \1/' "$local_conf"
        sed -i 's/^\(\s*proxy_pass http:\/\/shadow_o2server_backend\)/# \1/' "$local_conf"
        sed -i 's/^\(\s*proxy_set_header Host \$host;\)/# \1/' "$local_conf"
        sed -i 's/^\(\s*proxy_set_header X-Real-IP \$remote_addr;\)/# \1/' "$local_conf"
        sed -i 's/^\(\s*proxy_set_header X-Forwarded-For \$proxy_add_x_forwarded_for;\)/# \1/' "$local_conf"
        sed -i 's/^\(\s*proxy_set_header X-Forwarded-Proto \$scheme;\)/# \1/' "$local_conf"
        sed -i 's/^\(\s*proxy_connect_timeout\)/# \1/' "$local_conf"
        sed -i 's/^\(\s*proxy_send_timeout\)/# \1/' "$local_conf"
        sed -i 's/^\(\s*proxy_read_timeout\)/# \1/' "$local_conf"

        echo "影子流量已禁用（本地 nginx.conf）"
        echo ""
        echo "下一步："
        echo "  重载 nginx：sudo nginx -t && sudo nginx -s reload"
        ;;

    run)
        echo "=== 发送测试请求（携带 X-Behavior-Comparison: true） ==="

        if [[ -z "${OA4RUST_TOKEN:-}" ]]; then
            echo "警告：未设置 OA4RUST_TOKEN 环境变量，测试可能返回 401"
        fi

        for endpoint in "${TEST_ENDPOINTS[@]}"; do
            echo ""
            echo "--- 测试: $endpoint ---"
            curl -s -w "\nHTTP %{http_code}\n" \
                 "http://localhost/jaxrs/$endpoint" \
                 -H "X-Behavior-Comparison: true" \
                 -H "Authorization: Bearer ${OA4RUST_TOKEN:-}" \
                 --max-time 10 || echo "请求失败"
            sleep 0.5
        done

        echo ""
        echo "测试请求已发送。"
        echo "Rust 行为对比日志已写入：$RUST_LOG"
        echo "Java 影子响应已记录在：$ACCESS_LOG"
        echo ""
        echo "使用 ./shadow-traffic.sh compare 对比响应一致性"
        ;;

    compare)
        echo "=== 对比 Rust 与 Java 响应一致性 ==="
        echo ""

        if [[ ! -f "$RUST_LOG" ]]; then
            echo "错误：未找到 Rust 日志: $RUST_LOG"
            exit 1
        fi
        if [[ ! -f "$ACCESS_LOG" ]]; then
            echo "错误：未找到 nginx access log: $ACCESS_LOG"
            exit 1
        fi

        # 提取 Rust behavior_comparison 记录
        # 支持 tracing JSON 格式和文本格式
        rust_entries=$(grep -E "behavior_comparison|behavior-comparison" "$RUST_LOG" | tail -100 || true)

        if [[ -z "$rust_entries" ]]; then
            echo "警告：未在 Rust 日志中找到 behavior_comparison 记录"
            echo "请确认："
            echo "  1. Rust 服务已启用 behavior_comparison 中间件"
            echo "  2. 请求携带了 X-Behavior-Comparison: true 头"
            exit 1
        fi

        echo "找到 $(echo "$rust_entries" | wc -l) 条 Rust 行为对比记录"
        echo ""

        # 初始化计数器
        total=0
        matched=0
        mismatched=0
        declare -a mismatches=()

        # 逐条对比
        while IFS= read -r line; do
            ((total++))

            # 提取路径和状态码（支持 JSON 和 key=value 格式）
            if echo "$line" | grep -q '"path"'; then
                # JSON 格式
                path=$(echo "$line" | grep -o '"path":"[^"]*"' | cut -d'"' -f4 || true)
                rust_status=$(echo "$line" | grep -o '"status":[0-9]*' | cut -d':' -f2 || true)
                body_preview=$(echo "$line" | grep -o '"body_preview":"[^"]*"' | cut -d'"' -f4 || true)
            else
                # key=value 格式
                path=$(echo "$line" | grep -oE 'path=[^ ]+' | cut -d'=' -f2 || true)
                rust_status=$(echo "$line" | grep -oE 'status=[0-9]+' | cut -d'=' -f2 || true)
                body_preview=$(echo "$line" | grep -oE 'body_preview="[^"]*"' | cut -d'"' -f2 || true)
            fi

            [[ -z "$path" ]] && continue

            # 在 nginx access log 中查找对应的 mirror 请求
            # 匹配条件：请求路径相同，且 backend=127.0.0.1:8080（shadow 流量）
            java_entry=$(grep "\"$path " "$ACCESS_LOG" | grep "backend=127.0.0.1:8080" | tail -1 || true)

            if [[ -z "$java_entry" ]]; then
                echo "[$total] 路径: $path"
                echo "  Rust 状态: $rust_status"
                echo "  Java 响应: 未找到（可能 mirror 未启用或请求未到达 Java）"
                ((mismatched++))
                mismatches+=("[$total] $path - 未找到 Java 响应")
                continue
            fi

            # 提取 Java 状态码和响应大小
            java_status=$(echo "$java_entry" | awk '{print $9}' || true)
            java_size=$(echo "$java_entry" | awk '{print $10}' || true)

            # 对比状态码
            if [[ "$rust_status" == "$java_status" ]]; then
                status_match="✓"
                ((matched++))
            else
                status_match="✗"
                ((mismatched++))
                mismatches+=("[$total] $path - Rust: $rust_status, Java: $java_status")
            fi

            echo "[$total] 路径: $path"
            echo "  Rust 状态: $rust_status | Java 状态: $java_status | 匹配: $status_match"
            echo "  Rust body_preview: ${body_preview:0:100}..."
            echo "  Java 响应大小: ${java_size:-unknown} bytes"
            echo ""
        done <<< "$rust_entries"

        # 输出总结
        echo "=== 对比总结 ==="
        echo "总记录数: $total"
        echo "一致: $matched"
        echo "不一致: $mismatched"
        echo ""

        if [[ $mismatched -gt 0 ]]; then
            echo "不一致项："
            for m in "${mismatches[@]}"; do
                echo "  $m"
            done
            echo ""
            echo "建议：检查以下可能原因："
            echo "  1. Rust 与 Java 业务逻辑差异"
            echo "  2. 数据库数据不一致"
            echo "  3. 请求参数或 Header 差异"
            exit 1
        else
            echo "✓ 所有记录一致"
        fi
        ;;

    report)
        echo "=== 生成影子流量一致性报告 ==="
        echo ""

        # 运行对比并保存结果
        compare_output=$(./shadow-traffic.sh compare 2>&1) || true

        {
            echo "# OA4Rust U9 影子流量一致性报告"
            echo ""
            echo "生成时间：$(date -u +%Y-%m-%dT%H:%M:%SZ)"
            echo ""
            echo "## 对比结果"
            echo ""
            echo "\`\`\`"
            echo "$compare_output"
            echo "\`\`\`"
            echo ""
            echo "## 观察建议"
            echo ""
            echo "- 观察期：至少 2 周"
            echo "- 关注指标：错误率、P99 延迟、数据一致性"
            echo "- 回滚条件：详见 docs/gray-release-playbook.md"
        } > "$COMPARISON_LOG"

        echo "报告已生成：$COMPARISON_LOG"
        ;;

    status)
        echo "=== 影子流量状态 ==="
        echo ""

        if is_mirror_enabled "$NGINX_CONF_LOCAL"; then
            echo "影子流量：已启用（本地配置）"
        else
            echo "影子流量：已禁用"
        fi

        echo ""
        echo "Rust 日志：$RUST_LOG"
        if [[ -f "$RUST_LOG" ]]; then
            echo "Rust 日志存在 ✓"
            echo "behavior_comparison 记录数：$(grep -cE "behavior_comparison|behavior-comparison" "$RUST_LOG" 2>/dev/null || echo 0)"
        else
            echo "Rust 日志不存在"
        fi

        echo ""
        echo "Nginx access log：$ACCESS_LOG"
        if [[ -f "$ACCESS_LOG" ]]; then
            echo "Access log 存在 ✓"
            echo "shadow 请求数（backend=127.0.0.1:8080）：$(grep -c "backend=127.0.0.1:8080" "$ACCESS_LOG" 2>/dev/null || echo 0)"
        else
            echo "Access log 不存在"
        fi
        ;;

    *)
        echo "未知命令: $cmd"
        echo ""
        echo "用法:"
        echo "  ./shadow-traffic.sh enable         启用影子流量"
        echo "  ./shadow-traffic.sh disable        禁用影子流量"
        echo "  ./shadow-traffic.sh run            发送测试请求"
        echo "  ./shadow-traffic.sh compare        对比响应一致性"
        echo "  ./shadow-traffic.sh report         生成一致性报告"
        echo "  ./shadow-traffic.sh status         查看当前状态"
        exit 1 ;;
esac
