---
title: feat: oa4rust remaining gap closure
type: feat
status: active
date: 2026-08-20
origin: docs/brainstorms/2026-08-20-oa4rust-remaining-gap-closure-requirements.md
---

# OA4Rust 剩余缺口补全计划

## Summary

基于 `docs/brainstorms/2026-08-20-oa4rust-remaining-gap-closure-requirements.md` 的审计结论，分 5 个实施单元完成剩余缺口补全：行为对比测试清单扩展与 `#[ignore]` 处理、新功能 crate 挂载修复、handler 直接调用测试补全、Java-Rust 端点对齐度提升、文档 REST Endpoints 字段填充。目标是在测试保护、功能可达性和文档可用性上达到全面可代替 o2server 的标准。

---

## Problem Frame

oa4rust 已完成 95 个 crate 的真实化和 7624+ 条路由注册，`cargo test --workspace --lib` 全部通过。但深度审计揭示五个结构性缺口：行为对比测试覆盖率仅 6.6% 且被 `#[ignore]` 跳过；`empower` 完全未挂载、`preview/realtime/signature` 部分挂载；handler 级直接调用测试覆盖率仅 0.6%；Java-Rust 端点对齐度约 37%；文档 REST Endpoints 字段全部空白。这些缺口直接关系到 oa4rust 能否在质量信心上完全代替 o2server。

---

## Requirements

- R1. 将 `tests/behavior_compare.rs` 的硬编码端点清单从 77 个扩展至覆盖全部 1012 个自动生成端点（去重后约 992 个 unique rust_path）
- R2. 移除 `behavior_compare_rust_vs_java` 的 `#[ignore]` 标记，改为条件执行（`BEHAVIOR_COMPARE=1`）
- R3. 保持 Java 不可达时的 SKIP 降级机制
- R4. 为 `empower` 添加 `use empower` 和 `.merge(empower::router(pool.clone(), session_manager.clone()))`
- R5. 为 `preview`、`signature` 补充 `use` 声明
- R6. 为 `realtime` 补充 `use` 声明并挂载 `ws_stats`
- R7. 对 `processplatform_assemble_surface`、`cms_assemble_control`、`organization_assemble_control` 各补全 ≥ 50 个 handler 直接调用测试
- R8. 无 DB 依赖 handler 使用 stub mock pool
- R9. 有 DB 依赖 handler 使用真实 PostgreSQL
- R10. 通过 Python 脚本批量生成测试桩
- R11. 对照 Java `@Path` 注解识别 Rust 缺失端点
- R12. 实现缺失端点的 Rust handler
- R13. 新增端点通过行为对比测试验证
- R14. 为 55 张模块卡片的 REST Endpoints 字段填充端点列表
- R15. 验证 `docs/oa/README.md` 链接可解析

**Origin actors:** A1（开发者）、A2（CI 流水线）、A3（前端 o2web）、A4（新加入开发者）
**Origin flows:** F1（行为对比测试扩展）、F2（crate 挂载修复）、F3（handler 测试补全）、F4（端点对齐度提升）、F5（文档填充）
**Origin acceptance examples:** AE1（行为对比测试）、AE2（empower 挂载）、AE3（handler 测试）、AE4（端点对齐）、AE5（文档填充）

---

## Scope Boundaries

- **包含：** 行为对比测试清单扩展与 `#[ignore]` 处理；`empower/preview/realtime/signature` 的 main.rs 挂载修复；handler 直接调用测试补全；Java-Rust 端点对齐度提升；文档 REST Endpoints 字段填充
- **排除在外：** 前端 o2web 代码修改；Java 后端代码修改；数据库迁移或 schema 变更；新增 Java 不存在的新功能；修改现有业务逻辑代码（仅新增测试/挂载/文档）

### Deferred to Follow-Up Work

- 行为对比测试的 Java 侧服务容器化
- handler 级测试覆盖率提升至 99%（本轮目标 ≥ 50%）
- 文档模块卡片的 Key Flows、Dependencies 等字段深度填充
- `docs/oa/modules/o2web/` 下组件卡片的填充
- SQLx 完全移除

---

## Context & Research

### Relevant Code and Patterns

- **Crate 挂载模式：** `src/main.rs` 中统一使用 `.merge(crate::router(pool.clone(), session_manager.clone()))`，`empower::router(pool, session_manager)` 签名兼容
- **测试基础设施：** `crates/shared/src/testing.rs` 提供 `test_pool()`、`mock_pool()`、`is_db_available()`、`send_request()` 等工具
- **行为对比测试框架：** `tests/behavior_compare.rs`（硬编码清单）、`tests/behavior_comparison/`（自动生成清单 + 比较器 + 允许列表）
- **MCP 工具生成：** `scripts/gen_mcp_tools.py` → `crates/mcp_server/src/generated_routes.rs`
- **OpenAPI 生成：** `scripts/gen_openapi_paths.py` → `crates/openapi/src/lib.rs`
- **模块卡片文档：** `docs/oa/modules/o2server/*.md`（55 张，Responsibility 已填充，REST Endpoints 空白）

### Institutional Learnings

- `docs/solutions/` 中暂无直接相关的方案文档
- 历史计划显示 2026-08-10 已完成认证安全模块补全、MCP 工具桥接、program_center CRUD 写操作
- 2026-08-12 已完成 portal 测试编译错误修复、微信模板消息补全、响应格式统一

### External References

- 无外部依赖，全部工作基于本地代码库

---

## Key Technical Decisions

- **行为对比测试从 `#[ignore]` 改为条件执行：** 保留 Java 不可达时的 SKIP 降级机制，通过环境变量 `BEHAVIOR_COMPARE=1` 控制是否执行，避免 CI 中无条件跳过
- **`empower` 挂载沿用现有 `.merge()` 模式：** `empower::router(pool, session_manager)` 签名与 main.rs 中其他 crate 一致，直接添加 `use empower` 和 `.merge(empower::router(pool.clone(), session_manager.clone()))`
- **Handler 测试优先覆盖大模块：** `processplatform_assemble_surface`（487 handlers）、`cms_assemble_control`（312 handlers）、`organization_assemble_control`（105 handlers）占总 handler 数的 33%，优先补全可最大化覆盖率提升
- **文档填充基于现有端点清单：** `tests/behavior_comparison/endpoints.rs` 的 1012 个端点定义作为 REST Endpoints 字段的数据源
- **Java-Rust 端点对齐度以 HTTP 方法注解为统计口径：** 以 Java 侧的 @GET/@POST/@PUT/@DELETE 总数为分母，Rust 侧的 `.route()` 注册数为分子

---

## Open Questions

### Resolved During Planning

- [Affects R4] `empower::router()` 签名是否为 `router(pool: Pool, session_manager: SessionManager) -> Router`？**已解决：** 读取 `crates/empower/src/router.rs` 确认签名兼容，与 `personal::router(pool, session_manager)` 模式一致

### Deferred to Implementation

- [Affects R7][Needs research] 大模块的 50 个 handler 直接调用测试中，有多少 handler 因架构限制（内部 service 函数未导出）无法直接调用
- [Affects R11][Needs research] Java 侧 3152 个 HTTP 方法注解中，有多少对应 Rust 侧已实现但未注册行为对比测试的端点
- [Affects R2][Technical] 行为对比测试的条件执行机制具体实现：在 `behavior_compare.rs` 中添加 `#[cfg_attr(not(test), ignore)]` 还是使用 `#[tokio::test(flavor = "multi_thread")]` 配合环境变量

---

## Implementation Units

### U1. 行为对比测试清单扩展与条件执行

**Goal:** 将行为对比测试从 77 个端点扩展至覆盖全部 992 个 unique 端点，并移除无条件 `#[ignore]` 跳过

**Requirements:** R1, R2, R3

**Dependencies:** None

**Files:**
- Modify: `tests/behavior_compare.rs`
- Test: `tests/behavior_compare.rs`（自身）

**Approach:**
- 将 `tests/behavior_comparison/endpoints.rs` 中的 1012 个 EndpointDef 合并到 `tests/behavior_compare.rs` 的 `ENDPOINTS` 常量中
- 去重（按 `rust_path`），确保每个 unique 路径只出现一次
- 移除 `#[ignore]` 标记，改为条件执行：当环境变量 `BEHAVIOR_COMPARE=1` 时执行完整对比，否则跳过
- 保持现有 SKIP 降级机制：Java 不可达时全部 SKIP，Rust 不可达时 FAIL

**Patterns to follow:**
- `tests/behavior_compare.rs` 现有的 `all_endpoints()` 去重逻辑
- `tests/behavior_comparison/comparator.rs` 的 `is_service_reachable()` 和 SKIP 机制

**Test scenarios:**
- Happy path: `BEHAVIOR_COMPARE=1 cargo test --test behavior_compare` 执行全部 992 个端点对比，Java 可达时全部 PASS
- Java unreachable: Java 服务不可达时全部端点标记为 SKIP，测试套件通过
- Rust unreachable: Rust 服务不可达时测试 FAIL
- Default skip: 无 `BEHAVIOR_COMPARE` 环境变量时测试跳过

**Verification:**
- `cargo test --test behavior_compare` 在默认情况下跳过
- `BEHAVIOR_COMPARE=1 cargo test --test behavior_compare` 执行全部端点
- 测试报告输出到 `target/debug/behavior-report.md`

---

### U2. 新功能 crate 挂载修复

**Goal:** 完成 `empower`、`preview`、`realtime`、`signature` 在 `main.rs` 中的挂载和导入修复

**Requirements:** R4, R5, R6

**Dependencies:** None

**Files:**
- Modify: `src/main.rs`
- Test: `src/main.rs`（编译验证）

**Approach:**
- `empower`：添加 `use empower;` 声明，在 `create_app()` 中添加 `.merge(empower::router(pool.clone(), session_manager.clone()))`
- `preview`：添加 `use preview;` 声明（已有 `.merge(preview::preview_route(...))`）
- `signature`：添加 `use oa4rust_signature;` 声明（已有 `.merge(oa4rust_signature::signature_route(...))`）
- `realtime`：添加 `use realtime;` 声明，将 `ws_stats` 路由挂载到现有 WebSocket 子应用

**Patterns to follow:**
- `src/main.rs` 中 existing `.merge(crate::router(pool.clone()))` 模式
- `realtime` 的 WebSocket 子应用构建模式（`Router::new().route("/ws", get(...)).route("/ws/{room_id}", get(...))`）

**Test scenarios:**
- Compile: `cargo check --workspace` 通过，无新增错误
- Mount: 向 `empower` 的 `/jaxrs/person/empower` 发送请求，返回非 404
- Mount: 向 `realtime` 的 `/ws/stats` 发送请求（如果 ws_stats 存在），返回非 404

**Verification:**
- `cargo test --workspace --lib` 全部通过
- `empower`、`preview`、`realtime`、`signature` 的 handler 在 HTTP 层可达

---

### U3. Handler 直接调用测试补全

**Goal:** 对三大模块补全 handler 直接调用测试，并建立批量生成脚本

**Requirements:** R7, R8, R9, R10

**Dependencies:** U2（ crate 挂载完成后，集成测试更完整）

**Files:**
- Create: `scripts/gen_handler_tests.py`
- Modify: `crates/processplatform_assemble_surface/src/tests.rs` 或 `tests_generated.rs`
- Modify: `crates/cms_assemble_control/src/tests.rs` 或 `tests_generated.rs`
- Modify: `crates/organization_assemble_control/src/tests.rs` 或 `tests_generated.rs`

**Approach:**
- 开发 Python 脚本 `scripts/gen_handler_tests.py`：解析各 crate 的 `src/lib.rs`，提取 `pub async fn` 名称和签名，生成 `tests_generated.rs` 测试桩
- 对 `processplatform_assemble_surface`、`cms_assemble_control`、`organization_assemble_control` 各生成 ≥ 50 个直接调用测试
- 无 DB 依赖 handler：使用 `shared::testing::mock_pool()` 构造 stub pool
- 有 DB 依赖 handler：使用 `shared::testing::test_pool()` + `is_db_available()` 守卫

**Patterns to follow:**
- `crates/ai/src/tests_generated.rs`（17 个直接 handler 调用测试）
- `crates/file_assemble_control/src/tests.rs`（4 个直接 handler 调用测试 + mock pool）
- `tests/integration_tests/db.rs` 的 `init_test_database()` / `drop_database()` 模式

**Test scenarios:**
- Happy path: 无 DB handler 使用 mock pool，直接调用返回 `ActionResult::success`
- Integration: 有 DB handler 使用真实 PostgreSQL，执行实际 SQL 查询
- Script: `python scripts/gen_handler_tests.py processplatform_assemble_surface` 生成 50+ 个测试桩

**Verification:**
- `cargo test -p processplatform_assemble_surface` 通过，≥ 50 个直接调用测试
- `cargo test -p cms_assemble_control` 通过，≥ 50 个直接调用测试
- `cargo test -p organization_assemble_control` 通过，≥ 50 个直接调用测试

---

### U4. Java-Rust 端点对齐度提升

**Goal:** 对照 Java `@Path` 注解识别并实现 Rust 侧缺失的高优先级端点

**Requirements:** R11, R12, R13

**Dependencies:** U1（行为对比测试就绪后验证新端点）

**Files:**
- Modify: 各缺失端点对应的 crate `src/lib.rs` 和 `src/routes.rs`
- Test: `tests/behavior_compare.rs`（新增端点需加入对比清单）

**Approach:**
- 静态分析 `oa/o2server/` 下 365 个含 `@Path` 注解的 Java 文件，提取 `@GET`/`@POST`/`@PUT`/`@DELETE` 方法
- 与 `tests/behavior_comparison/endpoints.rs` 的 1012 个 Rust 端点进行交叉比对
- 识别高优先级缺口（认证安全、个人中心、流程平台等核心模块）
- 对缺口实现 Rust handler，保持 `ActionResult<T>` 9 字段结构兼容
- 新增端点自动加入行为对比测试清单

**Patterns to follow:**
- 现有 `auth/src/two_factor.rs`、`auth/src/sso.rs` 等认证端点的实现模式
- `organization_assemble_control/src/lib.rs` 的大模块 handler 组织模式
- `shared/src/response.rs` 的 `ActionResult` 构造模式

**Test scenarios:**
- Behavior compare: 新增端点通过 `BEHAVIOR_COMPARE=1 cargo test --test behavior_compare` 验证
- Manual: 向新增端点发送 HTTP 请求，返回 `ActionResult` 结构正确

**Verification:**
- Java-Rust 端点对齐度从 37% 提升至 ≥ 70%
- 新增端点全部通过行为对比测试（Java 可达时）

---

### U5. 文档 REST Endpoints 字段填充

**Goal:** 为 55 张模块卡片的 REST Endpoints 字段填充实际端点列表

**Requirements:** R14, R15

**Dependencies:** U1（behavior_comparison/endpoints.rs 作为数据源）

**Files:**
- Modify: `docs/oa/modules/o2server/*.md`（55 张卡片）
- Test: 链接可解析性验证

**Approach:**
- 基于 `tests/behavior_comparison/endpoints.rs` 的 1012 个端点定义，按 crate 分组
- 为每个模块卡片的 "REST Endpoints" 字段填充端点路径、HTTP 方法、简要描述
- 验证 `docs/oa/README.md` 中所有链接可正常解析

**Patterns to follow:**
- `docs/oa/modules/o2server/*.md` 现有字段格式（Core Classes and Interfaces、Dependencies 等）

**Test scenarios:**
- Content: 随机抽查 5 张模块卡片，REST Endpoints 字段非空且包含实际端点
- Link: `docs/oa/README.md` 中所有链接可正常解析

**Verification:**
- 55 张模块卡片的 REST Endpoints 字段全部填充
- `docs/oa/README.md` 链接可解析

---

## System-Wide Impact

- **Interaction graph:** U1 和 U4 共享 `tests/behavior_compare.rs`，需协调端点清单维护；U2 修改 `src/main.rs`，影响所有 crate 的路由挂载顺序
- **Error propagation:** 行为对比测试的 SKIP 机制需保持稳定，避免新端点引入 FAIL
- **State lifecycle risks:** `empower` 挂载后需验证 session 中间件正确注入
- **API surface parity:** U4 新增端点需保持 `ActionResult<T>` 9 字段结构兼容
- **Integration coverage:** U1 的行为对比测试是 U4 的验证基础设施
- **Unchanged invariants:** 现有 1012 个端点的行为和响应结构不因本计划改变

---

## Risks & Dependencies

| Risk | Likelihood | Impact | Mitigation |
|------|-----------|--------|------------|
| 大模块 handler 因架构限制无法直接调用 | Medium | Medium | 对未导出 handler 调整 `pub` 修饰或改用集成测试 |
| Java 侧端点清单不完整或口径不一致 | Medium | Medium | 以 Rust 侧 `endpoints.rs` 为主，Java 侧作为参考 |
| 行为对比测试执行时间过长 | Medium | Medium | 保持现有分页/超时机制，必要时分批执行 |
| 文档填充工作量过大 | Low | Low | 基于现有 endpoints.rs 自动生成，人工复核 |

---

## Documentation / Operational Notes

- 行为对比测试需在 CI 中配置 `BEHAVIOR_COMPARE=1` 环境变量才能执行
- `docs/oa/modules/o2server/` 的 REST Endpoints 字段填充后，建议更新 `docs/oa/README.md` 指向
- Java 服务容器化（Deferred）后将显著提升行为对比测试的有效性

---

## Sources & References

- **Origin document:** [docs/brainstorms/2026-08-20-oa4rust-remaining-gap-closure-requirements.md](../brainstorms/2026-08-20-oa4rust-remaining-gap-closure-requirements.md)
- Related code: `src/main.rs`, `tests/behavior_compare.rs`, `tests/behavior_comparison/endpoints.rs`, `crates/empower/src/router.rs`
- Related plans: `docs/plans/2026-08-10-002-fix-oa4rust-gap-closure-plan.md`, `docs/plans/2026-08-12-001-fix-oa4rust-final-gap-closure-plan.md`
- External docs: 无
