# Rollback Plan — OA4Rust Migration

## 1. 触发回滚的条件

以下任一条件触发回滚：

| 条件 | 阈值 | 检测方式 |
|------|------|----------|
| 数据损坏 | 出现脏数据、丢失更新、违反约束 | 数据校验脚本 / 应用层校验失败 |
| 错误率突增 | 5xx 错误率 > 5% 或 401/403 错误率 > 20% | nginx access_log + Prometheus |
| 响应延迟超标 | P95 延迟 > 2s（对比 Java 基线 P95 的 150%） | APM / nginx `$request_time` |
| 认证失败率突增 | 登录接口 401 率 > 30% | auth crate 日志统计 |
| 数据库连接异常 | 连接池耗尽、查询超时 > 5s | deadpool 指标 |
| 手动触发 | 开发/运维人员在 5 分钟内无法定位问题 | 运维决策 |

## 2. 回滚流程

```
触发 → 确认 → 切换 → 验证 → 复盘
```

### 步骤 1：触发（< 1 分钟）
- 自动监控告警或人工判断触发回滚
- 记录触发时间、触发条件、影响范围

### 步骤 2：确认（< 2 分钟）
- 确认回滚条件满足
- 通知相关开发人员
- 若为误报，可中止回滚

### 步骤 3：切换（< 5 分钟）
- **方案 A（推荐）：nginx 路由切换**
  ```bash
  # 将模块路由切回 Java
  export MODULE_ROUTING="<module_name>:java"
  # 重载 nginx（或发送 SIGHUP）
  nginx -s reload
  ```
- **方案 B：特性开关**
  ```bash
  # 在 Rust 服务中设置环境变量
  export RUST_FEATURE_ROLLBACK=true
  # 或通过管理端点
  curl -X POST http://localhost:8080/jaxrs/admin/rollback \
    -H "Authorization: Bearer <admin_token>" \
    -H "Content-Type: application/json" \
    -d '{"module": "<module_name>", "target": "java"}'
  ```

### 步骤 4：验证（< 3 分钟）
- 确认流量已切回 Java
- 验证核心业务流程恢复正常
- 监控错误率和延迟是否回落

### 步骤 5：复盘（24 小时内）
- 分析根本原因
- 修复问题后在测试环境验证
- 更新回滚计划

## 3. 特性开关设计

### nginx MODULE_ROUTING（已实现）
- 格式：`MODULE_ROUTING=module1:rust,module2:java`
- 默认值：未设置时全部路由到 Rust
- 热加载：修改环境变量后重载 nginx 生效

### Rust 特性开关（待实现）
- 环境变量：`RUST_FEATURE_ROLLBACK=true/false`
- 管理端点：`POST /jaxrs/admin/rollback`
- 返回当前路由状态：`GET /jaxrs/admin/routing-status`

## 4. 回滚演练计划

- 每波次完成后执行一次回滚演练
- 演练内容：模拟触发 → 切换 → 验证 → 切回
- 记录演练时间，目标：5 分钟内完成切换

## 5. 数据一致性保障

- 双轨运行期间，Java 保持写入能力
- Rust 侧仅读取或通过数据校验后写入
- 回滚时直接切回 Java，数据不丢失
