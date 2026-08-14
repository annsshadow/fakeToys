---
title: OA4Rust 能否 100% 替代 oa/o2server —— 现状对比与功能缺口分析
type: analysis
status: completed
date: 2026-08-13
scope: oa/o2server (Java 参考实现) vs oa4rust (Rust 重写)
evidence: docs/audits/o2server-parity-report.{md,json}, docs/brainstorms/oa4rust-endpoint-inventory.md, docs/brainstorms/oa4rust-migration-status.md, findings_correctness.json, migrations-review-result.json, cargo_err.txt, build_errors.txt, crates/** 源码静态核查
---

# OA4Rust 能否 100% 替代 oa/o2server —— 现状对比与功能缺口分析

## 0. 结论（先行）

**结论：以"能否端到端承担 o2server 的生产职责"为替代标准，当前 oa4rust 不能 100% 替代 oa/o2server。差距不是"差几个接口"，而是结构性、多维度、量级上的差距。** 同名路由 ≠ 同等功能，这是本次分析的核心判定口径。

三条硬阻断证据（均在源码/构建产物中可直接复现）：

1. **工作区无法干净编译。** `cargo_err.txt` 记录 `crates/file` 因 E0308/E0631/E0277 等 **10 个编译错误**无法编译；`build_errors.txt` 尾部 `src/lib.rs:21/63/79/96` 另有 4 个错误（`cms_control` 路由缺参、processplatform 三处 `Some(pool)` 类型不匹配）。**代码编不过，服务根本起不来。**
2. **"已完成"的口径不可信。** 项目自述"82/83 crate done、仅 3 个 Value::Null"，但源码静态核查显示 **`Value::Null` 实有 201 处，分布于 42 个 crate**；`crates/cms_assemble_control/src/lib.rs` 内 311 个 handler 仅 **11 个**真正调用 `pool.get()` 查库（约 3.5%），并保留 `// ─── appinfo_* stubs ───` 等注释，约 96% 为不查库的"假成功壳"。
3. **缺失整个能力类别。** Java 平台具备、Rust 完全没有或严重不足的：多数据库适配（含国产达梦/金仓/DB2/Informix）、分布式架构（Redis/Kafka/ActiveMQ/Quartz/中心-应用-节点）、Lucene 全文检索、IM 绑定（企业微信/钉钉/政务钉钉/飞书/公众号/andfx）、短信、验证码、Office 预览、电子签章、以及 BPMN 工作流的执行语义。

---

## 1. 现状对比（量化）

### 1.1 规模与工程基线

| 维度 | oa/o2server（Java） | oa4rust（Rust） | 差距 |
|------|--------------------|----------------|------|
| 构建系统 | Maven（55 个 `x_*` 模块） | Cargo workspace（88 crate） | — |
| 源码规模 | 12,834 个 `.java` 文件（src 内 7,851，~756,263 LOC） | 88 crate，handler 约 2,593–2,625 | Java 功能密度约为 Rust 当前实现的 **10–20 倍** |
| REST 端点 | **13,560** 个 `@Path`（全树）/ **5,858** 唯一（src 内 3,375） | **2,625** handler / 1,012 路由 | 数量约 **45%**（唯一端点口径，含大量浅层） |
| 编译状态 | 生产可用（持续交付） | **不干净**（file crate 10 错 + 根 4 错） | Rust 当前不可运行 |
| 测试 | 生产级（量级未量化） | 787 个，**约 15% handler 覆盖**；且**无库可测**（见 §1.5） | Rust 测试不验证功能 |
| 内置迁移器 | JPA/Hibernate 自动 DDL | **无 in-app 迁移运行器**（依赖手工跑 `migrations/*.sql`，已至 025） | Rust 部署易因漏跑 SQL 失败 |

### 1.2 运行架构与基础设施

| 能力 | oa/o2server | oa4rust | 备注 |
|------|------------|---------|------|
| HTTP 容器 | 内嵌 Jetty | axum (tokio) | 对等 |
| 内置库 | H2（开发/演示） | 无（直连 PG） | Rust 无默认库，需外部 PG |
| 数据库适配 | H2/MySQL/Oracle/PG/SQLServer/DB2/**达梦/金仓/Informix** | **仅 PostgreSQL** | 国产库缺失 |
| 分布式 | Center+Application+Node 三态；Redis 缓存；Kafka+ActiveMQ；Quartz | 单节点；内存滑动窗口限流；`auth_token_threshold` 表轮询失效 | 无 Redis/MQ/集群 |
| 全文检索 | Lucene（221 处引用）+ ES 包装 | **0 处引用（grep `lucene` 命中 0）** | 检索能力缺失 |
| 调度 | Quartz cron（258 处引用） | **无** | 定时任务缺失 |
| 文件存储 | webdav/sftp/s3/ftp/local 抽象（`externalStorageSources.json`） | file crate 有编译错误，存储抽象未证实 | 高危 |
| 信创认证 | 麒麟/UOS/ARM/MIPS 已认证 | 可交叉编译但未认证、未适配 | 合规缺口 |

### 1.3 认证与单点登录

| 能力 | oa/o2server | oa4rust | 状态 |
|------|------------|---------|------|
| 账号/密码/token | ✅ 完整 | auth crate ~45 handler（但 7 处 Value::Null） | 部分可用 |
| LDAP / AD | ✅ | ldap crate **1 handler、0 测试** | 基本空白 |
| OAuth2 | ✅ | 有 013_add_oauth_fields 迁移 | 部分 |
| SSO 客户端 | ✅ | 有 014_add_sso_client 迁移 | 部分 |
| 企业微信绑定 | ✅ `qiyeweixin/` | **无对应实现** | 缺失 |
| 钉钉 / 政务钉钉 | ✅ `dingding/` `zhengwudingding/` | **无** | 缺失 |
| 飞书(华为 welink) | ✅ `welink/` | **无** | 缺失 |
| 公众号 / 移动 andfx | ✅ `mpweixin/` `andfx/` | **无** | 缺失 |
| 短信 | ✅ `x_message_*` | **无证据** | 缺失 |
| 验证码 | ✅ `CaptchaAction` | **无证据** | 缺失 |
| 令牌阈值(异地踢出) | ✅ `tokenThreshold` | 有 017 迁移 + 表轮询 hack | 弱实现 |

### 1.4 高级引擎与业务模块

| 模块 | oa/o2server | oa4rust | 判定 |
|------|------------|---------|------|
| BPMN 工作流 + 表单设计器 | processplatform（work/task/workCompleted 执行引擎） | 有 487 handler 但多为 CRUD 薄壳；**工作流执行语义未证实** | 高风险 |
| 门户/页面设计器 | portal designer + surface | 56+48 handler | 待验证深度 |
| 自定义查询/报表/统计 | query（StatementAction/StatAction） | 67+59 handler（覆盖偏低） | 待验证 |
| 内容管理 CMS | cms（1,515 @Path） | 311 handler，**仅 3.5% 查库** | 实质空白 |
| 论坛 BBS | ✅ 354 @Path | 57 handler（7 处 Value::Null） | 部分 |
| 会议/考勤/日历 | ✅ | 有 handler，逻辑偏薄 | 部分 |
| 即时通讯 WebSocket | ✅ `ImAction`×33 | message crate 存在，实时未证实 | 待验证 |
| 移动推送 | ✅ 极光 jpush | jpush crate 16 handler | 待验证 |
| AI | ✅ x_ai | ai crate 21 handler | 待验证 |
| Office 预览 / 电子签章 | ✅（70+ 文件签名/PDF 引用） | **无证据** | 缺失 |

### 1.5 测试现实（"通过"≠"正确"）

`docs/REALIZE_RUNBOOK.md:19-20` 明确写明：**本环境无 PostgreSQL，测试必须无库也能编译并通过**。`crates/attendance_assemble_control/src/tests.rs` 用 `mock_pool()`（空配置，不真连库），断言 `==500` 或 `!=404` —— **只验证路由已注册，不验证任何数据与逻辑**。

因此：
- 项目"测试通过"= 编译通过 + 路由接线存在，**零功能校验**。
- `ldap`、`organization_assemble_authentication`、`organization_assemble_personal` 等 crate **0 测试**（parity report）。
- 2026-08-13 的"handler 测试 99% 覆盖"计划，即使达成，也是**无库覆盖**，不消除功能风险。

---

## 2. 功能缺口清单（尚不可用或缺失）

按"是否阻断替换"分级。括号标注证据来源。

### P0 — 阻断级（不解决则连替代的前提都不成立）

1. **编译不通**：`file` crate 10 错误（E0308 `&str`/`String` 匹配、E0631 `.map()` 闭包类型、E0277 `serde_json::Value` 未实现 `ToSql`）；`src/lib.rs:21/63/79/96` 4 错误（cms_control 路由缺参、processplatform 3 处 `Some(pool)` 类型）。【cargo_err.txt / build_errors.txt】
2. **无 in-app 迁移运行器**：schema 依赖手工执行 `migrations/*.sql`（已至 025）；已补齐的 017/018/019（token_threshold、auth_person 列、identity 表）若漏跑，登录/safe_logout 直接 500。【grep 命中 0】
3. **201 处 `Value::Null` 静默数据缺口**：覆盖 auth(7)、organization_core_entity(12)、express(16)、meeting_core_entity(14)、calendar_core_entity(13)、portal_assemble_designer(10)、cms_core_entity(7)、bbs_assemble_control(7) 等 **42 个 crate**。这些接口"成功返回"但数据为空，比 500 更危险（调用方无感知）。【源码静态核查】
4. **进度追踪失真**：`oa4rust-migration-status.md` 称"仅 3 个 Value::Null"，与源码 201 严重不符，导致决策依据错误。

### P1 — 重度缺口（核心能力不可用，生产无法接管）

5. **CMS 实质空白**：311 handler 仅 11 个查库（3.5%），约 96% 假成功壳（源码含 `// ─── appinfo_* stubs ───` 注释）。【cms_assemble_control】
6. **BPMN 工作流执行语义未证实**：processplatform 是 o2server 核心卖点；Rust 侧 487 handler 中大部分为 CRUD 薄壳（`attendancedetail_analyse` 仅 `UPDATE ... SET analysed=true`，无计算），会签/退回/并行网关/定时器流转未证实。
7. **认证生态缺失**：LDAP（0 测试）、企业微信、钉钉、政务钉钉、飞书、公众号、andfx、短信、验证码全部缺失或空白。
8. **全文检索缺失**：Lucene 221 引用对应的检索能力在 Rust 侧零对等实现（grep `lucene` 命中 0）。

### P2 — 中度缺口（影响可用性/可运维性）

9. **多数据库适配缺失**：仅 PostgreSQL；国产达梦/金仓、Oracle、SQLServer、DB2、Informix 均未支持，迁移可行性依赖 SQL 方言抽象层。
10. **分布式/集群缺失**：无 Redis 缓存、无 MQ（Kafka/ActiveMQ）、无 Quartz 调度、无 Center/Application/Node 拓扑；多实例仅靠 `auth_token_threshold` 表轮询失效（DB 轮询 hack，非真正分布式缓存）。
11. **Office 预览 / 电子签章缺失**：o2server 有 70+ 文件签名/PDF 与 OfficeAction；Rust 侧无证据。
12. **IM/WebSocket 实时能力未证实**：`x_message` 有 `ImAction`×33；Rust message crate 实时推送链路未验证。
13. **高频模块覆盖偏低**：attendance_assemble_control 89 handler 仅 5 测试(6%)、bbs 57 handler、general 61 handler(7%)、meeting 59(20%)，且均为无库测试。

### P3 — 待验证/低风险缺口

14. **mcp_server / openapi 为壳**：openapi crate 由脚本生成 767 个空 `async fn {}`（设计如此，但属占位）；mcp_server 有 `generated_routes.rs`（8176 行、0 handler，路由表）+ `tool_bridge.rs`（1821 行、2 函数，真实桥接） —— 较完整但需验证实际调用链路。
15. **信创适配/合规**：未认证麒麟/UOS/ARM/MIPS，生产信创环境无法直接替换。
16. **可观测性/运维**：健康检查、指标、日志聚合、灰度/回滚（deploy/ 有 nginx/rollback 脚手架）已初步具备，但缺乏生产级监控。

---

## 3. 优先级排序（决策矩阵）

| 缺口 | 阻断替代? | 工作量 | 业务影响 | 优先级 |
|------|----------|--------|----------|--------|
| P0-1 编译修复 | 是 | 低（≤1 周） | 致命 | **P0** |
| P0-2 in-app 迁移器 | 是 | 低（≤3 天） | 致命 | **P0** |
| P0-3 201 处 Value::Null 真实化 | 是 | 高（跨 42 crate） | 致命（静默空数据） | **P0** |
| P0-4 进度追踪修正 | 是 | 低 | 决策正确 | **P0** |
| P1-5 CMS 真实化 | 是(核心模块) | 高 | 高 | **P1** |
| P1-6 工作流执行语义 | 是(核心卖点) | 极高 | 高 | **P1** |
| P1-7 认证生态 | 是(多数部署) | 高 | 高 | **P1** |
| P1-8 全文检索 | 否(可暂代) | 中 | 中 | **P2** |
| P2-9 多数据库适配 | 否(可 PG 先行) | 高 | 依客户 | **P2** |
| P2-10 分布式/集群 | 否(中小规模) | 极高 | 依规模 | **P2** |
| P2-11 预览/签章 | 否 | 中 | 中 | **P2** |
| P2-12 实时 IM | 否 | 中 | 中 | **P2** |

**排序逻辑**：先消除"不可运行/静默错误"（P0），再补齐核心业务模块的真实功能（P1：CMS、工作流、认证），最后才是规模化与高级特性（P2）。P0 工作量小但收益巨大，应最先完成。

---

## 4. 关键不确定性 / 分析局限

- **"handler 数"不能直接等同功能**：o2server 一个 `@Path` 背后可能是复杂工作流；oa4rust 一个 `pub async fn` 可能是 `SELECT *` 薄壳。本分析已用 `pool.get()` 调用数与 `Value::Null` 数交叉验证浅层程度。
- **部分 P1/P2 模块的"真实度"需逐 handler 审计**：本次为抽样核查（重点 cms/processplatform/attendance），未穷举 2,625 个 handler。
- **运行态未实测**：本环境无 PostgreSQL，无法实际启动 oa4rust 验证登录/检索/工作流端到端行为；以上结论基于静态代码+构建产物+已有审计报告。
- **Java 侧全量功能以 `@Path` 计数近似**：13,560 含 build 重复，唯一 5,858；实际业务逻辑量更大（JPA 实体、服务层、引擎类未计入）。

> 下一文档 `2026-08-13-003-oa4rust-completion-plan.md` 给出将上述缺口落地为可执行的分阶段实施方案。
