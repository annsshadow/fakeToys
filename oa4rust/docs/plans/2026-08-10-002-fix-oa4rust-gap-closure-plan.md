---
title: OA4Rust 遗留问题修复 — DB 超时、模块拆分、OpenAPI 生成
type: fix
status: completed
date: 2026-08-10
origin: docs/brainstorms/2026-08-10-002-oa4rust-gap-closure-requirements.md
---

# OA4Rust 遗留问题修复 — DB 超时、模块拆分、OpenAPI 生成

## Summary

修复 `feat/oa4rust-gap-audit` 分支的三个技术遗留：升级 deadpool 添加连接池超时保护、按实体拆分 `program_center_core_entity` 的 lib.rs、完善 OpenAPI 路径生成脚本（完整 schema + 原子写入）。按优先级分为三个独立 PR：P0 可靠性修复 → P1 工具链 → P2 重构。

---

## Problem Frame

`feat/oa4rust-gap-audit` 分支在 2026-08-10 完成主体功能后，code review 识别出三个技术遗留：DB 连接池无超时导致 PostgreSQL 不可达时请求无限挂起、`program_center_core_entity/lib.rs` 膨胀至 971 行影响可维护性、OpenAPI 路径生成脚本仅打印到 stdout 未实际集成。这三个问题均不影响现有功能正确性，但分别在可靠性、可维护性、工具链完整性维度存在明确风险。

---

## Requirements

- R1. `shared/src/db.rs` 的 `create_pool()` 必须配置等待超时（默认 10 秒），通过 `POOL_WAIT_TIMEOUT_MS` 环境变量可覆盖
- R2. deadpool 升级后 `cargo check --workspace` 必须通过，无新增编译错误
- R3. `program_center_core_entity/src/lib.rs` 不超过 100 行，handler 拆分到独立子模块
- R4. 拆分后 `cargo test --workspace` 全部通过，无回归
- R5. `gen_openapi_paths.py` 直接写入 `crates/openapi/src/lib.rs`，生成完整 `#[utoipa::path]` + `#[openapi(...)]`
- R6. 生成脚本兼容多方法路由（`.route("path", put(h1).delete(h2))`）
- R7. 每个 `#[utoipa::path]` 包含 `params`（path 参数）和 `responses`（200/400/401/500）
- R8. 脚本原子写入（临时文件 + rename）

---

**Origin actors:** A1 (开发者), A2 (下游 Agent/MCP 客户端), A3 (CI 流水线)
**Origin acceptance examples:** AE1 (DB 超时), AE2 (模块拆分), AE3 (OpenAPI 生成), AE4 (集成测试), AE5 (环境变量覆盖)

---

## Scope Boundaries

- 仅修复三个遗留问题，不引入新功能
- 不修改 `main.rs` 或其他 crate 的路由注册
- 不修改 `shared/src/middleware/` 下除 `db.rs` 外的任何文件
- 模块拆分仅针对 `program_center_core_entity`，不影响其他 core_entity crate
- 不升级 utoipa、axum 或其他框架版本

### Deferred to Follow-Up Work

- DB 池超时的 multi-instance 场景（分布式限流）
- OpenAPI 的 `securitySchemes`（JWT/OAuth 安全描述）
- 模块拆分后的性能基准测试
- 电子签名功能（R16）

---

## Context & Research

### Relevant Code and Patterns

- **DB 池创建**: `crates/shared/src/db.rs` — `Pool::builder(mgr).build()` 模式，746 处 `pool.get().await` 调用分散在各 crate
- **双池共存**: `docs/solutions/architecture-patterns/seaorm-dual-pool-coexistence.md` — deadpool-postgres Pool (SQLx) 与 SeaORM DatabaseConnection 并行，升级 deadpool 不影响 SeaORM 侧
- **Runtime panic 防护**: `docs/solutions/integration-issues/nested-tokio-runtime-panic.md` — router 工厂中 `catch_unwind + block_on` 模式，deadpool 升级不改变此模式
- **IDOR 防护**: `docs/solutions/security-issues/idor-vulnerability-write-handlers.md` — 模块拆分时不能遗漏 `require_owner` 调用
- **CRUD 模式**: `docs/solutions/best-practices/crud-write-operations-pattern.md` — 拆分时保持软删除、creator_person 注入等模式一致
- **Input validation**: `docs/solutions/best-practices/input-validation-pattern.md` — 验证助手提取到共享模块
- **utoipa 注解模式**: `crates/control/src/person.rs` — 使用 `params(...)`, `responses(...)`, `request_body = StructName` 语法
- **脚本生成模式**: `scripts/extract_endpoints.py` — 直接重写目标文件（非 stdout），可作为参考
- **MCP 生成模式**: `scripts/gen_mcp_tools.py` — 已修复多行路由正则，可复用路由扫描逻辑

### Institutional Learnings

- **SeaORM Dual-Pool Coexistence**: deadpool 升级不影响 SeaORM 侧的 `DatabaseConnection`，两个池独立
- **Nested Tokio Runtime Panic**: router 工厂必须是同步函数，`catch_unwind` 模式不受 deadpool 版本影响
- **IDOR Vulnerability**: 拆分 handler 时必须保留 `require_owner` 调用，尤其是 application/script/invoke 的 update/delete
- **CRUD Write Operations**: 软删除、creator_person 注入、验证常量统一 — 拆分后每个子模块独立遵循
- **Input Validation**: 验证失败必须返回 `AppError::BadRequest`（非 `ActionResult::error`），拆分后保持此约定

### External References

- deadpool 0.12 changelog: `.timeouts(Timeouts)` API 引入
- deadpool-postgres 0.14 依赖 deadpool 0.12
- utoipa 4.x: `params(...)` 用于路径参数，`responses(...)` 用于响应定义，不支持 `summary` 字段

---

## Key Technical Decisions

- **deadpool 升级路径**: 将 `deadpool-postgres` 从 `"0.12"` 升级至 `"0.14"`，显式添加 `deadpool = "0.12"` 到 workspace dependencies — 这是唯一能使用 `.timeouts()` API 的路径，deadpool 0.13+ 要求 Rust 1.85 不兼容当前 1.75
- **模块拆分结构**: 在 `crates/program_center_core_entity/src/handlers/` 下创建 5 个子模块（`application.rs`、`script.rs`、`invoke.rs`、`agent.rs`、`structure.rs`），每个模块包含自身的请求结构体、验证和 CRUD handler；`lib.rs` 仅保留 router 组装、公共常量和 `pub use` 重导出；`pub use` 保持外部 API 不变
- **OpenAPI 生成策略**: 脚本直接生成完整 `lib.rs` 文件（非 append 模式）— 因为 `#[openapi(paths(...))]` 需要显式函数名列表，无法通过 `include!()` 注入；同时生成 `#[openapi(tags(...), info(...))]` 段
- **utoipa 4.x 语法**: 使用 `params(...)`（非 `parameters`）表示路径参数，`responses(...)` 表示响应，`request_body = StructName` 表示请求体；不使用 `summary` 字段（utoipa 4.x 不支持）
- **分级交付**: P0（DB 超时）独立 PR → P1（OpenAPI）独立 PR → P2（模块拆分）独立 PR

---

## Open Questions

### Resolved During Planning

- **deadpool 版本选择**: deadpool 0.12 是唯一兼容 Rust 1.75 且提供 `Timeouts` API 的版本；deadpool-postgres 0.14.1 依赖 deadpool 0.12，升级路径明确
- **utoipa 语法确认**: 通过检查 `crates/control/src/person.rs` 中的实际用法确认 — `params(...)` + `responses(...)` + `request_body = StructName`，无 `summary`
- **模块拆分可见性**: 所有 handler 保持 `pub async fn`（非 `pub(crate)`），通过 `pub use` 从 `lib.rs` 重导出，外部 crate（如 `routes.rs`）的引用不受影响
- **OpenAPI 生成原子性**: 脚本先生成到 `lib.rs.tmp`，验证文件大小和关键字段存在后 `std::fs::rename` 替换 — rename 在 same-filesystem 上是原子操作

### Deferred to Implementation

- **体参数推断精度**: 脚本从 `struct XxxCreateRequest { ... }` 正则匹配推断 body_params，对复杂嵌套结构可能不精确 — 可接受，OpenAPI 为占位用途
- **746 个路径的编译时间**: 实际编译时间需运行后测量，若超过 5 分钟则启用分批生成降级

---

## Implementation Units

### U1. DB 连接池超时保护

**Goal:** 为 `shared/src/db.rs` 的 `create_pool()` 添加连接池等待超时，防止 PostgreSQL 不可达时请求无限挂起。

**Requirements:** R1, R2

**Dependencies:** None（独立变更）

**Files:**
- Modify: `crates/shared/Cargo.toml`（新增 `deadpool` 依赖）
- Modify: `Cargo.toml` workspace.dependencies（升级 `deadpool-postgres` 至 0.14，新增 `deadpool = "0.12"`）
- Modify: `crates/shared/src/db.rs`（添加 `.timeouts()` 配置、环境变量读取、`DbError` 扩展）
- Modify: `crates/shared/src/lib.rs`（如有新错误类型导出）

**Approach:**
1. 升级 `deadpool-postgres` 至 `"0.14"`，显式添加 `deadpool = "0.12"` 到 workspace.dependencies
2. 在 `create_pool()` 中读取 `POOL_WAIT_TIMEOUT_MS` 环境变量（默认 10000ms），通过 `Timeouts::default().wait(Some(duration))` 配置
3. `DbError` 新增 `PoolTimeout` 变体（区分超时与其他池错误）
4. 确保 `From<deadpool::TimeoutError>` 或等效转换存在

**Patterns to follow:**
- `docs/solutions/architecture-patterns/seaorm-dual-pool-coexistence.md` — deadpool 升级不影响 SeaORM 侧
- `docs/solutions/integration-issues/nested-tokio-runtime-panic.md` — router 工厂模式不受影响

**Test scenarios:**
- Happy path: 正常启动，`create_pool()` 成功，超时配置为默认 10 秒
- Happy path: `POOL_WAIT_TIMEOUT_MS=5000` 环境变量设置后，超时配置为 5 秒
- Error path: PostgreSQL 不可达且连接池耗尽，`pool.get().await` 在 10 秒内返回错误（非无限挂起）
- Edge case: `POOL_WAIT_TIMEOUT_MS` 设置为 0，超时立即触发

**Verification:**
- `cargo check -p shared` 通过
- `cargo check --workspace` 通过（确认 83 个依赖 crate 无编译错误）
- `cargo test -p shared --lib` 通过
- `create_pool()` 中显式调用 `.timeouts()` 且在代码中可查到

---

### U2. OpenAPI 路径自动生成

**Goal:** 完善 `gen_openapi_paths.py` 使其直接生成完整、可编译的 `crates/openapi/src/lib.rs`。

**Requirements:** R5, R6, R7, R8

**Dependencies:** U1（建议先完成 U1 确保编译环境稳定，但可并行执行）

**Files:**
- Modify: `scripts/gen_openapi_paths.py`（重写主体逻辑）
- Create: `crates/openapi/src/lib.rs`（由脚本生成，非手动编写）
- Test: `scripts/test_gen_openapi_paths.py`（验证生成代码可编译）

**Approach:**
1. 脚本直接写入 `crates/openapi/src/lib.rs`（替换当前 stdout 输出）
2. 生成完整的 `#[utoipa::path(...)]` 注解 — 使用 utoipa 4.x 正确语法：`params(...)` 而非 `parameters`，无 `summary` 字段
3. 多方法路由处理：`.route("path", put(h1).delete(h2))` 为每个方法生成独立函数和路径项
4. `body_params` 推断：扫描 `struct XxxCreateRequest { ... }` 和 `struct XxxUpdateRequest { ... }` 定义，提取字段名生成 `request_body = XxxCreateRequest`
5. `responses` 生成：每个路径包含 200（body = serde_json::Value）、400、401、500
6. 原子写入：先写 `lib.rs.tmp`，验证成功后 `std::fs::rename`
7. 生成完整的 `#[openapi(paths(...), tags(...), info(...))]` 属性段

**Technical design:**
```
脚本输出结构：
1. 文件头注释（AUTO-GENERATED）
2. use 语句（utoipa::OpenApi, serde_json::Value 等）
3. 所有 #[utoipa::path(...)] 占位函数（~746 个）
4. #[derive(OpenApi)] + #[openapi(...)] 属性段（paths 列表 + tags 列表 + info）
5. pub struct ApiDoc;
```

**Patterns to follow:**
- `scripts/extract_endpoints.py` — 直接重写目标文件的模式
- `crates/control/src/person.rs` — utoipa 4.x 注解语法（`params`, `responses`, `request_body`）
- `docs/solutions/tooling-decisions/oa-component-card-generation.md` — 源码提取 → 生成骨架 → 验证完整性管线

**Test scenarios:**
- Happy path: 运行脚本后 `crates/openapi/src/lib.rs` 包含 ~746 个 `#[utoipa::path]` 函数
- Happy path: `cargo check -p openapi` 通过（无编译错误）
- Happy path: `#[openapi(paths(...))]` 包含所有生成的函数名
- Happy path: 多方法路由（如 `.route("path", put(h1).delete(h2))`）生成独立的 PUT 和 DELETE 路径项
- Happy path: `crates/openapi/src/lib.rs` 写入后原文件内容被完整替换（非追加）
- Error path: 脚本中断后原 `lib.rs` 未被损坏（原子写入保证）
- Edge case: 无路由的 crate 不产生任何输出

**Verification:**
- `python scripts/gen_openapi_paths.py` 执行成功
- `cargo check -p openapi` 通过
- `cargo check --workspace` 通过
- `crates/openapi/src/lib.rs` 包含 `#[openapi(paths(...))]` 且函数名数量与生成数量一致

---

### U3. program_center_core_entity 模块拆分

**Goal:** 将 `program_center_core_entity/src/lib.rs`（971 行）按实体拆分为独立子模块，`lib.rs` 缩减至 100 行以内。

**Requirements:** R3, R4

**Dependencies:** 无（独立变更）

**Files:**
- Create: `crates/program_center_core_entity/src/handlers/application.rs`
- Create: `crates/program_center_core_entity/src/handlers/script.rs`
- Create: `crates/program_center_core_entity/src/handlers/invoke.rs`
- Create: `crates/program_center_core_entity/src/handlers/agent.rs`
- Create: `crates/program_center_core_entity/src/handlers/structure.rs`
- Create: `crates/program_center_core_entity/src/handlers/mod.rs`
- Modify: `crates/program_center_core_entity/src/lib.rs`（缩减至 ~80 行：router + pub use + 常量）
- Modify: `crates/program_center_core_entity/src/tests.rs`（更新函数引用路径）
- Test: `crates/program_center_core_entity/src/handlers/tests.rs`（可选，或保留在 tests.rs 中按实体分组）

**Approach:**
1. 创建 `src/handlers/` 子目录，每个实体一个 `.rs` 文件
2. 每个 handler 模块包含：请求结构体（Create/Update）、验证函数（validate_name 等共享，或在模块内定义）、list/create/update/delete handler
3. `lib.rs` 精简为：use 声明、`pub mod handlers;`、`pub use handlers::*;`（或按需重导出）、常量定义、router 函数、tests 模块
4. 验证 `require_owner` 调用在所有 write handler 中保留（application/script/invoke 必须有，agent/structure 用 is_admin）
5. 验证 `creator_person` 从 `session.person_unique` 注入（非请求体）
6. 更新 `tests.rs` 中的函数引用路径（如 `crate::handlers::application::application_list`）
7. 运行 `cargo test -p program_center_core_entity` 验证

**Technical design:**
```
src/
  lib.rs          (~80 行: mod handlers, pub use, constants, router, tests)
  routes.rs       (不变，薄包装)
  tests.rs        (更新引用路径)
  handlers/
    mod.rs        (pub use 重导出)
    application.rs  (ApplicationCreateRequest, application_* handlers)
    script.rs       (ScriptCreateRequest, script_* handlers)
    invoke.rs       (InvokeCreateRequest, invoke_* handlers)
    agent.rs        (AgentCreateRequest, agent_* handlers)
    structure.rs    (StructureCreateRequest, structure_* handlers)
```

**Patterns to follow:**
- `docs/solutions/security-issues/idor-vulnerability-write-handlers.md` — `require_owner` 不可遗漏
- `docs/solutions/best-practices/crud-write-operations-pattern.md` — 软删除、creator_person 注入
- `docs/solutions/best-practices/input-validation-pattern.md` — 验证失败返回 `AppError::BadRequest`
- `docs/solutions/integration-issues/nested-tokio-runtime-panic.md` — router 工厂 `catch_unwind` 模式不变

**Test scenarios:**
- Happy path: `cargo test -p program_center_core_entity --lib` 全部通过
- Happy path: `cargo test --workspace` 全部通过（无回归）
- Happy path: `lib.rs` 不超过 100 行
- Happy path: 所有 handler 函数仍可通过 `crate::handlers::xxx::handler_name` 路径访问
- Error path: application_update 用非所有者 token 调用 → 返回 403 Forbidden（require_owner 生效）
- Error path: agent_update 用非 admin token 调用 → 返回 403 Forbidden（is_admin 检查生效）
- Edge case: 拆分后 `routes.rs` 的 `program_center_core_entity_router()` 调用不受影响

**Verification:**
- `lib.rs` ≤ 100 行
- `cargo test -p program_center_core_entity --lib` 100% 通过
- `cargo test --workspace` 100% 通过
- `git diff` 确认无意外文件变更

---

## System-Wide Impact

- **Interaction graph:** U1 仅影响 `shared/src/db.rs` 和 `Cargo.toml`，不影响任何 handler 调用链；U2 仅影响 `crates/openapi/src/lib.rs` 和脚本；U3 影响 `program_center_core_entity` crate 内部结构，外部通过 `pub use` 重导出保持 API 不变
- **Error propagation:** U1 新增 `PoolTimeout` 错误类型，上游 `map_err` 需适配；U3 不改变错误传播路径
- **State lifecycle risks:** 无状态变更风险 — 三个 unit 均为重构/配置类变更
- **API surface parity:** U3 拆分后公开 API 不变（`pub use` 重导出），`routes.rs` 和 `main.rs` 的 router 注册不受影响
- **Integration coverage:** U3 需确保 `tests/integration_tests/scenarios/program_center_core_entity.rs` 中的集成测试引用路径正确
- **Unchanged invariants:** `ActionResult<T>` 9 字段契约不变；`require_owner` 调用不变；`creator_person` 从 session 注入不变；双池架构不变

---

## Risks & Dependencies

| Risk | Mitigation |
|------|------------|
| deadpool 0.14 升级可能引入 breaking change 影响 83 个 crate | 先在 `shared` crate 单独验证 `cargo check`，再扩展至 workspace；保留原 Cargo.lock 备份 |
| 746 个 utoipa 路径导致编译超时 | R12 已设为条件式成功标准（超时则分批生成）；先在单个 crate 验证编译时间 |
| 模块拆分后 `pub use` 路径变化导致外部引用断裂 | 通过 `handlers/mod.rs` 的 `pub use` 重导出保持 `crate::handlers::application::*` 路径不变 |
| 脚本生成代码与现有 14 个占位函数风格不一致 | 脚本直接覆盖整个 lib.rs，旧占位函数一并替换 |
| 多方法路由正则匹配遗漏边界情况 | 复用 gen_mcp_tools.py 已修复的正则，额外添加多方法路由匹配逻辑 |

---

## Documentation / Operational Notes

- 更新 `docs/brainstorms/oa4rust-endpoint-inventory.md` 中的 program_center_core_entity 路由数（拆分后不变，仍为 20 路由）
- 更新 `docs/brainstorms/oa4rust-migration-status.md`（无变化，所有 crate 仍为 done）
- U2 完成后 `/openapi.json` 端点将返回完整 746 路径的 OpenAPI 规范
- 三个 PR 按 P0→P1→P2 顺序合并，确保每次合并后 `cargo test --workspace` 通过

---

## Sources & References

- **Origin document:** [docs/brainstorms/2026-08-10-002-oa4rust-gap-closure-requirements.md](../brainstorms/2026-08-10-002-oa4rust-gap-closure-requirements.md)
- **Related code:** `crates/shared/src/db.rs`, `crates/program_center_core_entity/src/lib.rs`, `scripts/gen_openapi_paths.py`, `crates/openapi/src/lib.rs`
- **Related solutions:** `docs/solutions/architecture-patterns/seaorm-dual-pool-coexistence.md`, `docs/solutions/security-issues/idor-vulnerability-write-handlers.md`, `docs/solutions/best-practices/crud-write-operations-pattern.md`
- **Related scripts:** `scripts/extract_endpoints.py`, `scripts/gen_mcp_tools.py`
