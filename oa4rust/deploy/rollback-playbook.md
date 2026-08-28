# OA4Rust U9 灰度迁移 — 回滚程序手册

> RTO 目标：**5 分钟**（从触发回滚到流量全部切回 Java 完成）

---

## 1. 触发条件

满足以下任一条件时，立即启动回滚流程：

| 级别 | 条件 |
|------|------|
| P1 | Rust 服务不可用（进程崩溃 / 端口未监听） |
| P1 | 关键模块错误率 > 5% 且持续 2 分钟 |
| P1 | 数据库连接池耗尽（deadpool 报 `Timeout` 或 `PoolExhausted`） |
| P1 | 数据不一致：同一事务在 Rust 与 Java 产生冲突写入 |
| P2 | 接口 P99 延迟突增 3 倍以上（由 Grafana / Loki 告警） |
| P2 | 内存 / CPU 持续打满（容器 OOM / cgroup throttle） |

**触发方式**：
- 自动化：Prometheus / Loki 告警 → PagerDuty → 值班工程师确认后执行
- 人工：业务方 / QA 反馈异常 → 值班工程师确认后执行

---

## 2. 回滚策略

### 2.1 策略矩阵

| 场景 | 策略 | RTO |
|------|------|-----|
| Rust 单节点故障 | nginx 摘除该节点（已有 `max_fails=3`） | < 30s |
| 整仓故障 / 数据错误 | nginx 将模块路由切回 `java_backend` | **5 min** |
| 配置错误（如 MODULE_ROUTING 配错） | 重启 Rust 服务 + 修正环境变量 | 2-3 min |
| 数据库迁移失败 | 停止 Rust 服务，全量切回 Java | 5 min |

### 2.2 核心原则

1. **优先切流，再查根因**：先恢复业务，再复盘。
2. **一次只做一件事**：避免在回滚同时提交新配置或代码。
3. **保留现场**：回滚前导出最近 15 分钟日志（`journalctl` / `docker logs`）供复盘。

---

## 3. 回滚流程

### 3.1 全量切回 Java（5 分钟目标）

**假设**：当前有模块已通过 nginx 路由到 Rust。

#### Step 0：确认与准备（30s）

```bash
# 1. 确认告警真实性
curl -s http://localhost:3000/health | jq .
curl -s http://localhost:8080/health | jq .

# 2. 通知相关方（#ops-oncall / 业务负责人）
echo "[U9-ROLLBACK] Starting rollback at $(date -u +%Y-%m-%dT%H:%M:%SZ)" | tee -a /var/log/rollback.log
```

#### Step 1：nginx 切流（1-2 min）

**方法 A：热更新 nginx 配置（推荐）**

```bash
# 1. 编辑路由配置（将 Rust 模块移到 java_backend）
#    详见 deploy/nginx.conf 注释，或直接包含 nginx-auth-routes.conf 兜底

# 2. 测试配置语法
nginx -t

# 3. 热重载
nginx -s reload

# 4. 验证
curl -s http://localhost/jaxrs/attendance/admin/list/all -H "Authorization: Bearer $TOKEN" -w "\nHTTP %{http_code}\n"
# 期望：HTTP 200（由 Java 返回）
```

**方法 B：降级 MODULE_ROUTING 环境变量（备用）**

```bash
# 1. 将 Rust 模块改为 java
export MODULE_ROUTING=attendance:java,control:java

# 2. 重启 Rust 服务（使其注册为 404 或不注册路由）
systemctl restart oa4rust

# 3. 配合 nginx fallback 到 java_backend 生效
#    注意：此方法依赖 nginx 已配置 proxy_intercept_errors on;
#         且 Rust 对 java 模块返回 404
```

#### Step 2：验证 Java 服务健康（1 min）

```bash
# 抽样检查关键接口
MODULES=("attendance" "control" "express" "meeting")
for mod in "${MODULES[@]}"; do
    echo "=== Checking $mod ==="
    # 根据实际接口调整，此处仅为示例
    curl -s "http://localhost/jaxrs/$mod/admin/list/all" \
         -H "Authorization: Bearer $TOKEN" \
         -w "HTTP %{http_code}\n"
done
```

#### Step 3：监控止血（1 min）

```bash
# 确认以下指标恢复正常：
# 1. 错误率 < 1%
# 2. P99 延迟恢复基线
# 3. 无新的 5xx 告警

# 持续观察 3 个采样周期（约 1 min）
watch -n 10 'curl -s http://localhost:3000/health'
```

#### Step 4：记录与通知（30s）

```bash
# 记录回滚完成时间
echo "[U9-ROLLBACK] Completed at $(date -u +%Y-%m-%dT%H:%M:%SZ)" | tee -a /var/log/rollback.log

# 通知业务方与值班经理
# 渠道：#ops-oncall + 邮件 / 企业微信
```

---

## 4. 灰度模块回滚清单

以下为当前可灰度模块，回滚时按需选择：

| 模块 | 默认路由 | nginx location 前缀 | 是否可独立回滚 |
|------|----------|---------------------|----------------|
| attendance | Rust | `/jaxrs/attendance/*` | 是 |
| control | Rust | `/jaxrs/(person|group|role|unit)/*` | 是 |
| express | Rust | `/jaxrs/express/*` | 是 |
| meeting | Rust | `/jaxrs/meeting/*` | 是 |
| message | Java | `/jaxrs/message/*` | 是 |
| calendar | Java | `/jaxrs/calendar/*` | 是 |
| portal | Java | `/jaxrs/portal/*` | 是 |
| bbs | Java | `/jaxrs/bbs/*` | 是 |

> 注：独立回滚时，仅修改对应模块的 nginx location 块，避免影响其他模块。

---

## 5. 常见故障与快速处置

| 现象 | 根因 | 快速处置 |
|------|------|----------|
| Rust 健康检查失败 | 进程崩溃 / 端口冲突 | `systemctl restart oa4rust` + 观察日志 |
| nginx 重载失败 | 配置语法错误 | `nginx -t` 定位错误行，修正后重载 |
| Java 服务也异常 | 共用 DB 或网络 | 切换前先确认 Java 服务本身健康 |
| 切流后仍有 5xx | nginx upstream 未刷新 | 确认 `proxy_next_upstream` 配置，或强制关闭长连接 |

---

## 6. 复盘要求

回滚完成后 24 小时内输出：

1. **时间线**：告警时间 → 确认时间 → 回滚开始 → 回滚完成 → 业务恢复
2. **根因**：代码缺陷 / 配置错误 / 容量不足 / 依赖故障
3. **改进项**：对应到具体 Action Item，进入 sprint backlog

---

## 7. 本地演练记录（U5 · 2026-08-26）

### 演练环境

- 宿主机 Windows，Docker 容器 `bash-runner`（bash:latest，Alpine）
- 目标脚本：`toggle_module.sh`、`shadow-traffic.sh`
- 无 nginx（本地演练跳过 nginx 重载步骤）

### 演练步骤与结果

| 步骤 | 命令 | 结果 | 备注 |
|------|------|------|------|
| 1. 初始状态 | `toggle_module.sh status` | ✓ 全部默认 Rust | 无 .module_routing.env |
| 2. 灰度10% | `toggle_module.sh gray attendance 10` | ✓ 生成 gray-routes.conf | attendance: 10% Rust / 90% Java |
| 3. 验证状态 | `toggle_module.sh status` | ✓ MODULE_ROUTING=attendance:rust | 状态文件记录正确 |
| 4. 回滚 | `toggle_module.sh rollback` | ✓ 恢复全部 Rust | .module_routing.env 清空 |
| 5. 重置 | `toggle_module.sh reset` | ✓ 清理所有产物 | .module_routing.env + gray-routes.conf 已删除 |
| 6. 影子流量 | `shadow-traffic.sh status` | ✓ 显示已禁用 | 日志文件不存在（无 nginx，符合预期） |

### 发现的缺陷与修复

**缺陷 #1：toggle_module.sh 参数解析偏移**
- **现象**：`gray attendance 10` 报错 "比例 'attendance' 必须是1-100 的整数"
- **根因**：`cmd="${1:-status}"` 取走命令后未 shift，while 循环的 `$1` 仍指向命令名而非模块名
- **修复**：在 `cmd=...` 后添加 `if [[ $# -gt 0 ]]; then shift; fi`；同步修正 `set` 分支的参数引用（`$2`→`$1`）
- **影响**：所有带参数的子命令（set/gray）均受影响
- **状态**：已修复并验证

### 已知限制

- nginx mirror 配置未在本地验证（容器内无 nginx）
- shadow-traffic.sh 的 compare/report 功能依赖 nginx access log，本地无法演练
- 演练仅验证脚本逻辑正确性，未涉及实际流量切换

### 运交注意事项

1. 生产环境执行灰度前，确保 nginx.conf 中已取消注释 `include /etc/nginx/conf.d/gray-routes.conf;`
2. rollback 后必须执行 `nginx -s reload` 使配置生效
3. 建议在非高峰时段执行灰度操作
4. 演练期间发现的 shift bug 已修复，部署前请确认脚本版本
