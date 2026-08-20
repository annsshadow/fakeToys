# OA4Rust U9 灰度迁移 — 灰度切换 Playbook

> **目标**：将 attendance / control / express / meeting 四个模块从 Java（o2server）逐步迁移到 Rust（oa4rust），通过模块级灰度 + 影子流量比对，确保 ≥ 2 周观察期无差异后全量切流。
>
> **RTO 目标**：5 分钟（从触发回滚到流量全部切回 Java 完成）
> **观察期**：每个阶段 ≥ 2 周，共 ≥ 10 周

---

## 目录

1. [灰度阶段总览](#1-灰度阶段总览)
2. [前置准备](#2-前置准备)
3. [阶段一：影子流量（Shadow Traffic）](#3-阶段一影子流量-shadow-traffic)
4. [阶段二：10% 灰度](#4-阶段二10-灰度)
5. [阶段三：50% 灰度](#5-阶段三50-灰度)
6. [阶段四：90% 灰度](#6-阶段四90-灰度)
7. [阶段五：100% 全量切流](#7-阶段五100-全量切流)
8. [观察指标](#8-观察指标)
9. [回滚条件与流程](#9-回滚条件与流程)
10. [2 周观察期 Checklist](#10-2-周观察期-checklist)
11. [常用命令速查](#11-常用命令速查)
12. [常见问题](#12-常见问题)
13. [角色与职责](#13-角色与职责)
14. [沟通计划](#14-沟通计划)

---

## 1. 灰度阶段总览

| 阶段 | 名称 | Rust 流量 | 目标 | 观察期 | 入口标准 |
|------|------|-----------|------|--------|----------|
| 0 | 影子流量 | 100%（复制到 Java） | 验证响应一致性 | 1 周 | Java 服务健康 |
| 1 | 10% 灰度 | 10% | 验证基础功能 | ≥ 2 周 | 影子流量阶段无差异 |
| 2 | 50% 灰度 | 50% | 验证性能与稳定性 | ≥ 2 周 | 10% 阶段指标正常 |
| 3 | 90% 灰度 | 90% | 验证容量边界 | ≥ 2 周 | 50% 阶段指标正常 |
| 4 | 100% 全量 | 100% | 完成迁移 | 持续观察 | 90% 阶段指标正常 |

> **关键原则**：每个阶段必须通过观察期 Checklist 才能进入下一阶段。未达标则回滚或延长观察期。

---

## 2. 前置准备

### 2.1 环境检查清单

- [ ] Rust 服务（oa4rust）已部署，监听 `127.0.0.1:3000`
- [ ] Java 服务（o2server）已部署，监听 `127.0.0.1:8080`
- [ ] nginx >= 1.13.4（支持 `mirror` 模块）
- [ ] 数据库连接正常（PostgreSQL 14）
- [ ] 监控系统已就绪（Grafana / Loki / Prometheus）
- [ ] 告警通道已配置（PagerDuty / 企业微信 / Slack）
- [ ] 回滚手册已同步给值班团队（见 `deploy/rollback-playbook.md`）

### 2.2 配置准备

```bash
# 1. 确认 nginx.conf 已就位
ls -la deploy/nginx.conf

# 2. 确认 toggle_module.sh 可执行
chmod +x deploy/toggle_module.sh

# 3. 确认 shadow-traffic.sh 可执行
chmod +x deploy/shadow-traffic.sh

# 4. 准备测试账号（用于影子流量测试）
export OA4RUST_TOKEN="your-test-token"
```

### 2.3 数据基线

在开始灰度前，记录当前 Java 服务的基线指标：

| 指标 | 基线值 | 测量时间 |
|------|--------|----------|
| 错误率（5xx） | < 0.1% | 最近 7 天平均 |
| P99 延迟 | ___ ms | 最近 7 天平均 |
| QPS | ___ | 最近 7 天平均 |
| 数据库连接池使用率 | < 70% | 最近 7 天平均 |

---

## 3. 阶段一：影子流量（Shadow Traffic）

**目标**：在不影响线上流量的前提下，复制请求到 Rust 和 Java，对比响应一致性。

### 3.1 启用影子流量

```bash
# 1. 确认 Java 服务健康
curl -s http://localhost:8080/health | jq .

# 2. 启用影子流量
cd deploy
./shadow-traffic.sh enable

# 3. 验证 nginx 配置
sudo nginx -t

# 4. 重载 nginx
sudo nginx -s reload
```

### 3.2 发送测试请求

```bash
# 发送带行为对比标记的请求
./shadow-traffic.sh run

# 或手动发送
curl -s http://localhost/jaxrs/attendance/admin/list/all \
     -H "X-Behavior-Comparison: true" \
     -H "Authorization: Bearer $OA4RUST_TOKEN"
```

### 3.3 对比响应一致性

```bash
# 运行一致性对比
./shadow-traffic.sh compare

# 生成报告
./shadow-traffic.sh report
```

### 3.4 观察指标

| 指标 | 验收标准 | 不达标处理 |
|------|----------|------------|
| 状态码一致率 | 100% | 排查业务逻辑差异 |
| 响应体大小差异 | < 5% | 检查字段顺序、时间戳等 |
| 错误率 | < 0.1% | 回滚，排查根因 |
| P99 延迟 | 与 Java 基线相当 | 优化 Rust 服务 |

### 3.5 阶段输出

- [ ] `shadow-comparison.log` 报告已生成
- [ ] 所有测试用例通过（状态码一致、响应体大小差异 < 5%）
- [ ] 无新的 5xx 告警
- [ ] 团队评审通过

**进入下一阶段条件**：影子流量阶段无差异，且观察 ≥ 1 周。

---

## 4. 阶段二：10% 灰度

**目标**：将 10% 的线上流量切到 Rust，验证基础功能。

### 4.1 配置灰度

```bash
# 1. 设置 10% 灰度（attendance, control, express, meeting）
cd deploy
./toggle_module.sh gray attendance 10 control 10 express 10 meeting 10

# 2. 确保 nginx.conf 中已启用 include
#    编辑 deploy/nginx.conf，取消注释：
#       include /etc/nginx/conf.d/gray-routes.conf;

# 3. 验证并重载 nginx
sudo nginx -t && sudo nginx -s reload

# 4. 验证灰度生效
curl -s http://localhost/jaxrs/attendance/admin/list/all \
     -H "Authorization: Bearer $OA4RUST_TOKEN" \
     -w "\nHTTP %{http_code}\n"
```

### 4.2 观察指标

| 指标 | 目标 | 不达标处理 |
|------|------|------------|
| 错误率（5xx） | < 0.5% | 立即回滚 |
| P99 延迟 | < Java 基线 1.5 倍 | 延长观察期 |
| 数据库连接池使用率 | < 80% | 扩容或回滚 |
| 业务方反馈 | 无异常 | 继续观察 |

### 4.3 阶段输出

- [ ] 10% 灰度配置已生效
- [ ] 监控仪表盘显示指标正常
- [ ] 业务方确认无异常
- [ ] 观察期 ≥ 2 周

**进入下一阶段条件**：10% 灰度阶段观察 ≥ 2 周，所有指标达标。

---

## 5. 阶段三：50% 灰度

**目标**：将 50% 的线上流量切到 Rust，验证性能与稳定性。

### 5.1 调整流量比例

```bash
# 1. 调整灰度比例到 50%
./toggle_module.sh gray attendance 50 control 50 express 50 meeting 50

# 2. 重载 nginx
sudo nginx -s reload

# 3. 验证比例生效
curl -s http://localhost/jaxrs/control/group/list \
     -H "Authorization: Bearer $OA4RUST_TOKEN" \
     -w "\nHTTP %{http_code}\n"
```

### 5.2 观察指标

| 指标 | 目标 | 不达标处理 |
|------|------|------------|
| 错误率（5xx） | < 0.3% | 回滚到 10% |
| P99 延迟 | < Java 基线 1.2 倍 | 延长观察期 |
| 数据库连接池使用率 | < 85% | 扩容 |
| 内存 / CPU | < 80% | 扩容或优化 |
| 业务方反馈 | 无异常 | 继续观察 |

### 5.3 阶段输出

- [ ] 50% 灰度配置已生效
- [ ] 监控仪表盘显示指标正常
- [ ] 观察期 ≥ 2 周

**进入下一阶段条件**：50% 灰度阶段观察 ≥ 2 周，所有指标达标。

---

## 6. 阶段四：90% 灰度

**目标**：将 90% 的线上流量切到 Rust，验证容量边界。

### 6.1 调整流量比例

```bash
# 1. 调整灰度比例到 90%
./toggle_module.sh gray attendance 90 control 90 express 90 meeting 90

# 2. 重载 nginx
sudo nginx -s reload
```

### 6.2 观察指标

| 指标 | 目标 | 不达标处理 |
|------|------|------------|
| 错误率（5xx） | < 0.2% | 回滚到 50% |
| P99 延迟 | < Java 基线 1.1 倍 | 延长观察期 |
| 数据库连接池使用率 | < 90% | 扩容 |
| 内存 / CPU | < 85% | 扩容或优化 |
| 业务方反馈 | 无异常 | 继续观察 |

### 6.3 阶段输出

- [ ] 90% 灰度配置已生效
- [ ] 监控仪表盘显示指标正常
- [ ] 观察期 ≥ 2 周

**进入下一阶段条件**：90% 灰度阶段观察 ≥ 2 周，所有指标达标。

---

## 7. 阶段五：100% 全量切流

**目标**：将所有流量切到 Rust，完成迁移。

### 7.1 全量切流

```bash
# 1. 将模块全量切到 Rust
./toggle_module.sh set rust attendance control express meeting

# 2. 关闭灰度 include（nginx.conf 中注释 include 行）
#    或删除 gray-routes.conf

# 3. 重载 nginx
sudo nginx -s reload

# 4. 验证全量切流成功
for mod in attendance control express meeting; do
    echo "=== Checking $mod ==="
    curl -s "http://localhost/jaxrs/$mod/admin/list/all" \
         -H "Authorization: Bearer $OA4RUST_TOKEN" \
         -w "\nHTTP %{http_code}\n"
done
```

### 7.2 观察指标

| 指标 | 目标 | 不达标处理 |
|------|------|------------|
| 错误率（5xx） | < 0.1% | 立即回滚 |
| P99 延迟 | < Java 基线 | 优化 Rust 服务 |
| 数据库连接池使用率 | < 80% | 扩容 |
| 业务方反馈 | 无异常 | 持续监控 |

### 7.3 阶段输出

- [ ] 全量切流成功
- [ ] 监控仪表盘显示指标正常
- [ ] 业务方确认无异常
- [ ] 持续观察 ≥ 2 周

---

## 8. 观察指标

### 8.1 核心指标

| 指标 | 采集方式 | 告警阈值 | 目标值 |
|------|----------|----------|--------|
| HTTP 错误率（5xx） | Prometheus / nginx access log | > 1% | < 0.1% |
| P99 延迟 | Grafana / nginx rt | > Java 基线 2 倍 | < Java 基线 |
| 数据库连接池使用率 | Grafana / deadpool metrics | > 90% | < 80% |
| 内存使用率 | Grafana / cgroup | > 90% | < 80% |
| CPU 使用率 | Grafana | > 80% | < 60% |
| 请求队列长度 | nginx / 应用指标 | > 100 | < 10 |

### 8.2 业务指标

| 指标 | 采集方式 | 告警阈值 |
|------|----------|----------|
| 考勤打卡成功率 | 业务监控 | < 99% |
| 审批流超时率 | 业务监控 | > 0.5% |
| 快递查询错误率 | 业务监控 | > 0.5% |
| 会议预约失败率 | 业务监控 | > 0.5% |

### 8.3 日志指标

| 指标 | 采集方式 | 告警阈值 |
|------|----------|----------|
| 异常堆栈数量 | Loki | 5 分钟内 > 10 条 |
| 数据库死锁 | PostgreSQL log | 任何死锁 |
| 连接超时 | Rust log | 任何 timeout |

---

## 9. 回滚条件与流程

### 9.1 自动回滚条件（P1）

满足以下任一条件时，**立即自动回滚**：

| 条件 | 检测方式 | 动作 |
|------|----------|------|
| Rust 服务不可用 | Health check 失败 | nginx 摘除节点 |
| 关键模块错误率 > 5% 持续 2 分钟 | Prometheus 告警 | 全量切回 Java |
| 数据库连接池耗尽 | deadpool Timeout / PoolExhausted | 停止 Rust 服务 |
| 数据不一致 | 对比脚本检测到冲突写入 | 全量切回 Java |

### 9.2 手动回滚条件（P2）

| 条件 | 检测方式 | 动作 |
|------|----------|------|
| P99 延迟突增 3 倍以上 | Grafana 告警 | 回滚到上一阶段 |
| 内存 / CPU 持续打满 | Grafana 告警 | 回滚或扩容 |
| 业务方反馈异常 | 人工反馈 | 立即回滚 |

### 9.3 回滚流程

**RTO 目标：5 分钟**

```bash
# 方法 A：一键回滚（推荐）
./toggle_module.sh rollback

# 方法 B：手动回滚
# 1. 将灰度模块切回 Java
./toggle_module.sh set java attendance control express meeting

# 2. 关闭灰度 include
#    编辑 nginx.conf，注释 include 行

# 3. 重载 nginx
sudo nginx -s reload

# 4. 验证 Java 服务健康
for mod in attendance control express meeting; do
    curl -s "http://localhost/jaxrs/$mod/admin/list/all" \
         -H "Authorization: Bearer $OA4RUST_TOKEN" \
         -w "\nHTTP %{http_code}\n"
done
```

### 9.4 回滚后检查清单

- [ ] 所有灰度模块已切回 Java
- [ ] nginx 配置已更新
- [ ] Java 服务健康检查通过
- [ ] 错误率恢复基线
- [ ] P99 延迟恢复基线
- [ ] 业务方确认恢复正常
- [ ] 回滚事件已记录（时间、原因、操作人）

---

## 10. 2 周观察期 Checklist

每个阶段必须完成以下 Checklist 才能进入下一阶段。

### 每日检查

- [ ] 错误率 < 阶段目标
- [ ] P99 延迟 < 阶段目标
- [ ] 无新的 5xx 告警
- [ ] 数据库连接池使用率 < 阶段目标
- [ ] 业务方无异常反馈

### 每周检查

- [ ] 导出本周监控数据（Grafana screenshot）
- [ ] 检查异常日志（Loki 查询）
- [ ] 检查数据库慢查询
- [ ] 检查内存泄漏（对比周初和周内存使用）
- [ ] 团队周会评审

### 阶段结束检查

- [ ] 观察期 ≥ 2 周
- [ ] 所有每日检查项全部通过
- [ ] 业务方签字确认
- [ ] 监控数据归档
- [ ] 进入下一阶段或回滚

---

## 11. 常用命令速查

### toggle_module.sh

```bash
# 查看当前状态
./toggle_module.sh status

# 全量切到 Rust
./toggle_module.sh set rust attendance control express meeting

# 全量切到 Java（回滚）
./toggle_module.sh set java attendance control express meeting

# 按比例灰度
./toggle_module.sh gray attendance 10 control 10 express 10 meeting 10
./toggle_module.sh gray attendance 50 control 50 express 50 meeting 50
./toggle_module.sh gray attendance 90 control 90 express 90 meeting 90

# 一键回滚
./toggle_module.sh rollback

# 重置为默认状态
./toggle_module.sh reset
```

### shadow-traffic.sh

```bash
# 启用影子流量
./shadow-traffic.sh enable

# 禁用影子流量
./shadow-traffic.sh disable

# 发送测试请求
./shadow-traffic.sh run

# 对比响应一致性
./shadow-traffic.sh compare

# 生成报告
./shadow-traffic.sh report

# 查看状态
./shadow-traffic.sh status
```

### nginx 运维

```bash
# 验证配置
sudo nginx -t

# 重载配置
sudo nginx -s reload

# 查看错误日志
tail -f /var/log/nginx/error.log

# 查看访问日志
tail -f /var/log/nginx/access.log
```

### Rust 服务运维

```bash
# 查看健康状态
curl -s http://localhost:3000/health | jq .

# 查看日志
tail -f /var/log/oa4rust/oa4rust.log

# 重启服务
systemctl restart oa4rust
```

---

## 12. 常见问题

### Q1：灰度比例不生效？

**排查步骤**：
1. 确认 `gray-routes.conf` 已生成：`ls -la deploy/gray-routes.conf`
2. 确认 nginx.conf 中已取消注释 include 行
3. 确认 nginx 已重载：`sudo nginx -t && sudo nginx -s reload`
4. 确认 nginx access log 中有流量记录

### Q2：影子流量 mirror 不工作？

**排查步骤**：
1. 确认 nginx 版本 >= 1.13.4：`nginx -v`
2. 确认 mirror 配置已启用：`grep "mirror /mirror_backend" /etc/nginx/nginx.conf`
3. 确认 Java 服务健康：`curl -s http://localhost:8080/health`
4. 检查 nginx error log：`tail -f /var/log/nginx/error.log`

### Q3：Rust 与 Java 响应不一致？

**排查步骤**：
1. 查看 `shadow-comparison.log` 报告
2. 确认差异是业务逻辑差异还是数据差异
3. 检查数据库数据是否一致
4. 如为业务逻辑差异，记录为 Issue，不影响灰度进度
5. 如为数据差异，立即回滚并排查

### Q4：如何快速回滚？

```bash
# 一键回滚（推荐）
./toggle_module.sh rollback

# 或手动回滚
./toggle_module.sh set java attendance control express meeting
sudo nginx -s reload
```

### Q5：如何验证灰度比例？

```bash
# 使用 X-Gray-Traffic 头强制切流
curl -H "X-Gray-Traffic: 1" http://localhost/jaxrs/attendance/admin/list/all
curl -H "X-Gray-Traffic: 0" http://localhost/jaxrs/attendance/admin/list/all
```

---

## 13. 角色与职责

| 角色 | 职责 | 联系人 |
|------|------|--------|
| 灰度负责人 | 整体灰度进度把控，阶段决策 | ___ |
| 运维工程师 | nginx 配置、服务部署、回滚执行 | ___ |
| 后端工程师（Rust） | Rust 服务监控、问题排查 | ___ |
| 后端工程师（Java） | Java 服务监控、数据一致性检查 | ___ |
| 测试工程师 | 影子流量测试、一致性对比 | ___ |
| 业务代表 | 业务指标确认、异常反馈 | ___ |

---

## 14. 沟通计划

### 14.1 定期同步

| 会议 | 频率 | 参与者 | 内容 |
|------|------|--------|------|
| 灰度站会 | 每日 10:00 | 灰度负责人、运维、后端 | 昨日指标、今日计划 |
| 灰度周报 | 每周五 | 全体相关方 | 本周指标、下周计划、风险 |
| 阶段评审会 | 每个阶段结束 | 全体相关方 | 阶段总结、进入下一阶段决策 |

### 14.2 异常沟通

- **P1 告警**：立即通知灰度负责人 + 运维 oncall，15 分钟内响应
- **P2 告警**：通知灰度负责人，1 小时内响应
- **业务反馈**：记录到故障系统，2 小时内给出初步结论

### 14.3 文档归档

- 每个阶段的监控数据归档到 `docs/gray-release/`
- 回滚事件记录到 `docs/incidents/`
- 阶段评审记录存档

---

## 附录 A：灰度配置示例

### A.1 nginx.conf 关键配置

```nginx
# 灰度配置 include（由 toggle_module.sh 生成）
include /etc/nginx/conf.d/gray-routes.conf;

# 或手动配置 upstream
upstream oa4rust_gray {
    server 127.0.0.1:3000 weight=10;
    server 127.0.0.1:8080 weight=90;
}

location ^~ /jaxrs/attendance/ {
    proxy_pass http://oa4rust_gray;
    # ... 其他 proxy 配置
}
```

### A.2 toggle_module.sh 使用示例

```bash
# 查看当前状态
./toggle_module.sh status

# 10% 灰度
./toggle_module.sh gray attendance 10 control 10

# 50% 灰度
./toggle_module.sh gray attendance 50 control 50

# 全量切到 Rust
./toggle_module.sh set rust attendance control

# 回滚
./toggle_module.sh rollback
```

### A.3 shadow-traffic.sh 使用示例

```bash
# 启用影子流量
./shadow-traffic.sh enable

# 发送测试
./shadow-traffic.sh run

# 对比
./shadow-traffic.sh compare

# 生成报告
./shadow-traffic.sh report
```

---

## 附录 B：回滚快速参考

| 场景 | 回滚命令 | RTO |
|------|----------|-----|
| 一键回滚 | `./toggle_module.sh rollback` | < 1 min |
| 切回 Java | `./toggle_module.sh set java <模块>` | < 1 min |
| nginx 重载 | `sudo nginx -s reload` | < 30s |
| 服务重启 | `systemctl restart oa4rust` | 2-3 min |

---

> **最后更新**：2026-08-19
> **文档维护者**：OA4Rust 实施团队
> **版本**：v1.0
