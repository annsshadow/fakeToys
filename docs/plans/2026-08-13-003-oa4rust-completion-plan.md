---
title: OA4Rust 补全 oa/o2server 的可落地分阶段实施方案
type: plan
status: active
date: 2026-08-13
depends_on: docs/plans/2026-08-13-002-oa4rust-o2server-parity-analysis.md
scope: 将 oa4rust 从"70% 壳"推进到可生产接管 oa/o2server 的分阶段路线
---

# OA4Rust 补全 oa/o2server 的可落地分阶段实施方案

## 0. 目标与判定口径

**目标**：使 oa4rust 在功能、可靠性、运维三个维度达到可生产接管 oa/o2server 的程度。

**"100% 替代"的定义（DoD）**：
1. `cargo build --workspace` 与 `cargo test --workspace` 在 CI 全绿（含**真实 PostgreSQL** 集成测试）。
2. 覆盖率：核心模块（auth/org/processplatform/cms/portal/query）handler 级**功能**测试 ≥ 95%，且测试连真实库验证数据（非仅路由可达）。
3. 缺口清单 P0–P1 全部关闭；P2 按客户部署需求裁剪（PG-only 中小规模可先行）。
4. 提供与 o2server 对等的认证生态（至少 LDAP + 企业微信 + 钉钉 + 验证码 + OAuth2/SSO）、CMS、BPMN 工作流执行、查询/报表。
5. 具备一键部署 + 回滚 + 健康监控，并通过一次生产级影子流量/灰度验证。

**核心原则**：
- **先真后多**：1 个真实 handler > 10 个假成功壳。关闭 `Value::Null` 优先于新增路由。
- **先编译可运行，再补功能**：P0 是地基。
- **可验证**：每个阶段以"能连库的集成测试通过"为关卡，杜绝"无库测试"虚高覆盖率。
- **不破坏既有契约**：`ActionResult<T>` 9 字段、RBAC、会话、双池架构保持不变（沿用 `docs/solutions` 既有约定）。

---

## Phase 0 —— 可构建 + 可诚实（P0，预计 1–2 周）

**目标**：让工作区干净编译、进度口径真实、迁移可自举。

### 0.1 修复编译错误
- **单元 U0.1**：修复 `crates/file/src/lib.rs` 的 10 个错误（E0308 `&str`/`String` 匹配 arm、E0631 `.map()` 闭包类型、E0277 `serde_json::Value` 未实现 `ToSql` → 改为 `json::to_string()` 后传 `&str` 或 `PgType`）。
- **单元 U0.2**：修复 `src/lib.rs:21/63/79/96` 4 个错误（`cms_control::cms_control_router()` 缺参；processplatform 三处 `Some(pool.clone())` 应改为 `pool.clone()`，与 `router(pool: Pool)` 签名一致）。
- **验证**：`cargo build --workspace` 零 error（warning 可接受）。在 CI 加 `cargo build --workspace --deny warnings` 逐步收紧。

### 0.2 内置迁移运行器
- **单元 U0.3**：引入 `sqlx-cli`/`refinery` 或自研 `migrate run` 子命令，启动时自动按序号执行 `migrations/001..025` 及后续，并落 `schema_migrations` 记录表。
- **理由**：避免漏跑 017/018/019 导致登录/safe_logout 500（历史 P0）。
- **验证**：空库启动 → 自动建表 → `auth_person` 含 `change_password_time`/`password_expired_time`/`job`/`department`/`unit`/`position`，`auth_token_threshold`/`auth_identity`/`auth_person_identity` 存在。

### 0.3 进度口径修正
- **单元 U0.4**：修正 `docs/brainstorms/oa4rust-migration-status.md` 与 `oa4rust-endpoint-inventory.md`，将"仅 3 个 Value::Null"改为真实 201，并新增"真实化率"指标（查库 handler / 总 handler）。
- **验证**：`scripts/gen_inventory.py` 输出包含 `value_null` 真实计数与 `db_touch_rate`。

**Phase 0 出关判定**：`cargo build` 绿 + 迁移自举绿 + 进度文档真实。

---

## Phase 1 —— 数据正确性 + 真实测试基座（P0/P1 基础，预计 3–4 周）

**目标**：消除 201 处静默空数据，建立"连库可验证"的测试基座。

### 1.1 建立真实 DB 测试环境
- **单元 U1.1**：提供 `docker-compose.yml`（PostgreSQL 14）+ `scripts/setup_test_db.sh`，CI 与本地均可一键起库。`DATABASE_URL` 守卫改为"有库则跑集成测试，无库则 skip"，**替换**当前"无库即 mock 返回 500"的伪测试。
- **单元 U1.2**：将现有 `mock_pool()` 测试迁移为连库测试，至少覆盖 auth 登录/登出/2FA、org 查询、processplatform 发起/流转。

### 1.2 关闭 201 处 Value::Null（按 crate 分批）
- **单元 U1.3**：脚本定位全部 `Value::Null`（`grep -rn "Value::Null" crates/`），按出现频次最高的 42 个 crate 分批：每批一个 PR，替换为真实查询或明确的不支持响应（返回 404/501 + 日志，而非空成功）。
- **优先级批次**：auth(7) → organization_core_entity(12)/express(16)/meeting_core_entity(14)/calendar_core_entity(13) → portal_assemble_designer(10) → cms_core_entity(7)/bbs_assemble_control(7) → 其余 33 crate。
- **验证**：每批 `cargo test --workspace` 连库通过；`grep -c "Value::Null" crates/` 归零。

### 1.3 零测试 crate 补测
- **单元 U1.4**：`ldap`、`organization_assemble_authentication`、`organization_assemble_personal` 至少各补 3 个连库测试（含 LDAP 失败回退 DB 路径、匿名 icon 公开访问、personal 读写）。

**Phase 1 出关判定**：0 个 Value::Null + 真实 DB 测试基座就绪 + 零测试 crate 清零。

---

## Phase 2 —— 核心模块真实化（P1，预计 8–12 周）

**目标**：让核心业务模块从"壳"变为"可用"，优先 CMS、工作流、认证生态。

### 2.1 CMS 真实化（缺口 P1-5）
- **单元 U2.1**：对 `crates/cms_assemble_control` 311 handler 逐组真实化。优先 `appinfo_*`/`categoryinfo_*`/`anonymous_*` 的列表/详情/发布，复用 `shared::response::row_to_json` 仅作过渡，最终落地参数化 SELECT + 业务校验 + 软删除。
- **验证**：每个 CMS 接口连库返回真实数据；文档 CRUD 链路端到端通过。

### 2.2 BPMN 工作流执行语义（缺口 P1-6，最高优先级模块）
- **单元 U2.2**：建立工作流引擎最小可用集：流程定义加载、work 创建、task 分派/认领、同意/退回/转办、并行/包容网关、定时器（用 tokio 调度暂代 Quartz）、workCompleted 归档。对照 `x_processplatform_service_processing` 的 144 @Path 行为定义验收用例。
- **验证**：一条含会签+退回+并行网关的流程端到端跑通（集成测试覆盖）。

### 2.3 认证生态（缺口 P1-7）
- **单元 U2.3 LDAP/AD**：补全 `crates/ldap` 真实绑定/搜索/校验 + 测试（修复 findings 中 two_factor 绕过 LDAP 的问题）。
- **单元 U2.4 企业微信/钉钉/政务钉钉**：新增 `organization_assemble_* ` 下 `qiyeweixin`/`dingding`/`zhengwudingding` 绑定/扫码登录 handler，对齐 o2server `AuthenticationAction` 的 24 @Path。
- **单元 U2.5 验证码 + 短信**：新增 captcha 生成/校验（复用 `captcha` crate 依赖）、短信网关抽象（可后接 mock）。
- **验证**：LDAP 登录、企微扫码登录、验证码登录各一条集成测试通过。

### 2.4 查询/报表/门户 深度验证
- **单元 U2.6**：对 query(67+59)、portal(56+48) 抽样审计真实度，补齐明显浅层 handler（如同 Phase 1 的 Value::Null 处理），确保核心 statement/stat/view 可用。

**Phase 2 出关判定**：CMS/工作流/认证三类核心场景端到端可用；P1 全部关闭。

---

## Phase 3 —— 规模化与高级特性（P2，按需求裁剪，预计 6–10 周）

### 3.1 多数据库适配（缺口 P2-9）
- **单元 U3.1**：抽象 SQL 方言层（或限定支持 PG + MySQL + 国产达梦/金仓），将死 SQL 参数化；提供方言开关。先做 MySQL，再评国产库。
- **验证**：同一套 handler 在 PG 与 MySQL 测试库均通过。

### 3.2 分布式/集群（缺口 P2-10）
- **单元 U3.2**：引入 Redis 作为会话/缓存层，替代 `auth_token_threshold` 表轮询；消息总线（NATS/Kafka 客户端）承载异步事件；可选 Quartz 等价调度（tokio-cron-scheduler）。
- **验证**：多实例部署下会话失效、事件广播一致。

### 3.3 全文检索（缺口 P1-8 / P2）
- **单元 U3.3**：集成 Tantivy（Rust Lucene 等价）或 PG 全文索引，提供 o2server Lucene 检索对等的文档/内容搜索接口。

### 3.4 IM/实时 + 预览/签章（缺口 P2-11/12）
- **单元 U3.4**：WebSocket 实时消息（对标 `ImAction`×33）；Office 预览（可接 OnlyOffice/LibreOffice 服务）、电子签章（PDF 签名）。

**Phase 3 出关判定**：按目标客户所需 P2 项完成并通过验收；非必选 P2 项标记为"路线图未覆盖"。

---

## Phase 4 —— 验证与切换（收尾，预计 2–3 周）

### 4.1  parity 回归套件
- **单元 U4.1**：基于 `docs/audits/o2server-parity-report.json` 的 55 模块映射，生成自动 parity 测试（对每个 o2server `@Path` 找到 Rust 等价并跑契约测试）。将"handler 数对比"升级为"行为契约对比"。

### 4.2 性能与容量
- **单元 U4.2**：压测核心链路（登录、流程发起、CMS 发布），确立基线；对比 o2server 同等场景。

### 4.3 灰度切换
- **单元 U4.3**：复用 `deploy/nginx.conf` + `toggle_module.sh` 做模块级灰度；影子流量比对 Rust 与 Java 响应一致性；观察期 ≥ 2 周无差异后切流。

**Phase 4 出关判定**：parity 套件绿 + 性能基线达标 + 灰度无回归 → 宣布"可替代"。

---

## 5. 风险登记

| 风险 | 影响 | 缓解 |
|------|------|------|
| CMS/工作流真实化工作量被低估 | Phase 2 延期 | 先小批试点（1 个 CMS 接口组 + 1 条流程）验证模式再铺开 |
| 无库测试文化导致"假绿" | 功能回归无感知 | Phase 1 强制连库测试基座，CI 阻断无库 mock |
| 共享层改动面大（db.rs/middleware） | 全仓回归 | 变更走独立 PR + `cargo test --workspace` 门禁 |
| 国产库/信创适配缺环境 | 无法验证 | 早期申请达梦/金仓测试实例，否则 Phase 3 标注为"未验证" |
| 进度口径失真反复 | 决策误判 | Phase 0 修正后，CI 自动校验 inventory 与源码一致 |

---

## 6. 里程碑与时间盒（估算，单开发视角）

| 里程碑 | 内容 | 时间盒 |
|--------|------|--------|
| M0 | Phase 0 完成（可构建 + 迁移自举 + 口径真实） | 1–2 周 |
| M1 | Phase 1 完成（0 Null + 连库测试基座） | +3–4 周 |
| M2 | Phase 2 完成（CMS/工作流/认证真实化） | +8–12 周 |
| M3 | Phase 3 按需完成（PG-only 中小规模可止步于此） | +6–10 周 |
| M4 | Phase 4 完成（parity + 灰度切换） | +2–3 周 |

> 若仅需"PG-only、中小规模、核心 OA 场景"的替代，可在 M2 后评估裁剪 P2，提前进入 M4 灰度 —— 即**务实的最低可行替代点**。

---

## 7. 与既有计划的关系

- 复用 `2026-08-13-001-feat-handler-test-coverage-99-plan.md` 的测试框架，但**要求**其测试改为连库（消除"无库 99%"的虚假覆盖）。
- 复用 `REALIZE_RUNBOOK.md` 的 handler 真实化模式与 `docs/solutions` 的 CRUD/RBAC/IDOR 约定。
- 不冲突 `2026-08-10-002-fix-oa4rust-gap-closure-plan.md`（已完成），本计划在其之上推进深度真实化。

---

## 实现情况（2026-08-21 审计）

**审计基准：** 工作树 HEAD 314c7a75；判定状态：active（总路线图，Phase 4 出关判定未达成）

### 分阶段落地状态

- **Phase 0 可构建+可诚实：已完成** —— 编译错误修复（提交 1d866b66 等）；迁移运行器读盘机制（`shared/src/migrate.rs`，启动按序应用 migrations/001..059）；进度口径修正
- **Phase 1 数据正确性+测试基座：大部分完成** —— docker-compose.yml + setup_test_db.sh + integration_tests（13 场景）在档；mock_pool→真实 DB 测试迁移系列提交在档；Value::Null 由 201 降至实测 15 处（未完全归零）；零测试 crate 已补测（ldap/organization_assemble_authentication/organization_assemble_personal 均有 tests）
- **Phase 2 核心模块真实化：大部分完成** —— CMS 新增 310 路由（fc937a40）+ 种子数据（c6b53715）；BPMN 事务支持（3ac82662）+ bpmn_process 集成场景；认证生态全量落地（LDAP/企微/钉钉/政务钉钉/验证码/短信）；U2.6 查询/门户深度验证未逐项核验
- **Phase 3 规模化与高级特性：大部分完成** —— MySQL 集成测试执行修复（004e02d9/d972e010）；Redis session 默认（7710d8af）；全文检索 migration 058；IM 端点（d972e010）、realtime crate、preview crate、PDF 签章链（ae911482）
- **Phase 4 验证与切换：部分完成** —— 行为契约对比套件（tests/behavior_comparison/）即 U4.1 的升级实现；性能基线 `oa4rust/docs/performance-baseline.md` 在档（U4.2）；灰度脚手架（deploy/nginx.conf、toggle_module.sh、shadow-traffic.sh、rollback-playbook.md、gray-release-playbook.md）全在档，但影子流量观察与切流（U4.3）未实际执行

### 未完成 / 遗留 → 待汇入剩余工作汇总计划

- Value::Null 残留 15 处清理（Phase 1 出关条件未完全满足）
- U2.6 query/portal 抽样深度审计与浅层 handler 补齐
- U4.3 影子流量灰度验证与切流执行（脚本就绪，流程未跑）
- DoD 第 3 条：Java-Rust 端点对齐度提升（当前约 36.6%，见 2026-08-20-001 U4）
