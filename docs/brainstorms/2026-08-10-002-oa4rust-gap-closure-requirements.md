---
date: 2026-08-10
topic: oa4rust-gap-closure
---

# OA4Rust 差距补全 — 遗留问题修复

## Summary

为 `feat/oa4rust-gap-audit` 分支实现三个遗留问题修复：DB 连接池超时保护（P0）、`program_center_core_entity` 模块按实体拆分（P2）、OpenAPI 路径自动生成脚本完善（P1）。**推荐三级独立 PR 交付**（P0 可靠性 → P1 工具链 → P2 重构），若合并为单一 PR 需保证每项可独立回滚。

---

## Problem Frame

`feat/oa4rust-gap-audit` 分支已在 2026-08-10 完成主体实现（认证安全模块扩展、MCP 工具桥接、用户注册、program_center CRUD 写操作），并通过 code review 修复了 P0 IDOR、P1 注册限流、switch_user 审计日志等安全问题。但在 review 收尾阶段，三个技术遗留问题未能在同一次迭代中解决：

1. **DB 连接池无超时**：`shared/src/db.rs` 的 `Pool::builder(mgr).build()` 未配置超时，deadpool 0.10 不直接暴露 `Timeouts` API，导致 PostgreSQL 不可达时所有请求无限挂起。
2. **`program_center_core_entity/lib.rs` 膨胀**：862 行包含全部 CRUD handler，与已有的 `entities/` 子模块结构不一致，维护成本持续增加。
3. **OpenAPI 路径未集成**：`gen_openapi_paths.py` 生成 746 条路径但仅打印到 stdout，`crates/openapi/src/lib.rs` 仍只有 14 个占位函数，`/openapi.json` 端点信息严重缺失。

---

## Actors

- A1. **开发者**：执行拆分和脚本修改
- A2. **下游 Agent（MCP 客户端）**：依赖 OpenAPI 规范发现可用端点
- A3. **CI 流水线**：`cargo check --workspace` 和 `cargo test --workspace` 验证无回归

---

## Requirements

**DB 连接池超时**
- R1. `shared/src/db.rs` 的 `create_pool()` 必须配置等待超时（默认 10 秒），当连接池耗尽时 `pool.get().await` 应返回 `PoolError` 而非无限挂起
- R2. 需评估 `deadpool` 从 0.10 升级到 0.12 的跨 crate 兼容性影响——当前 83 个 crate 依赖 `deadpool-postgres`，需确认升级不会引入 breaking change
- R3. 超时配置值应可通过环境变量或配置覆盖（不硬编码）

**模块拆分**
- R4. `program_center_core_entity/src/lib.rs` 按实体拆分为独立子模块：`application.rs`、`script.rs`、`invoke.rs`、`agent.rs`、`structure.rs`，每个模块包含自身的请求结构体、验证函数和 CRUD handler
- R5. `lib.rs` 仅保留 router 组装逻辑、公共常量（`MAX_NAME_LEN` 等）和 `pub use` 重导出
- R6. 测试文件同步拆分：每个实体模块配套独立的测试文件（`tests/application.rs`、`tests/script.rs` 等），或在现有 `tests.rs` 中按实体分组
- R7. 拆分后 `cargo test --workspace` 必须全部通过，无回归

**OpenAPI 自动生成**
- R8. `gen_openapi_paths.py` 必须直接重写 `crates/openapi/src/lib.rs`（而非打印到 stdout），生成完整的 `#[utoipa::path]` 占位函数，并**同时重写整个 `#[openapi(paths(...), tags(...), info(...))]` 属性段**（而非仅更新 paths 列表），确保 cargo check 不失败；脚本需覆盖 lib.rs 中从 `#[openapi(...)]` 开始到对应闭合括号为止的完整属性段，或生成完整 lib.rs 文件；**写入必须原子化**：先写临时文件，验证内容无误后 `std::fs::rename` 替换原文件，防止写入中断损坏源文件
- R9. **Spike 阶段**：先验证脚本获取请求结构体字段的技术路径（扫描 `*.rs` 中的 `struct CreateRequest`/`struct UpdateRequest` 定义，或从 handler 函数签名推断），输出 spike 结论文档；**实现阶段**：基于 spike 结论，脚本从 Rust 请求结构体自动推断 `body_params`，从 URL 路径自动推断 `path_params`。R9 实现依赖 spike 结论，spike 未验证前不得开始 R9 实现。
- R10. 每个 `#[utoipa::path]` 注解必须包含完整的 `responses` 字段：成功（200）、参数错误（400）、未认证（401）、内部错误（500）；生成的 stub 占位函数需指定返回类型 `-> Result<Json<ActionResult<Value>>, AppError>`（而非无返回类型），确保 utoipa 能正确推断 200 响应 schema
- R11. 生成脚本需兼容多方法路由（`.route("path", put(h1).delete(h2))`），为每个 HTTP 方法生成独立路径项
- R12. 生成后 `cargo check --workspace` 必须通过；**条件式成功标准**：若 utoipa derive 宏可正常处理 ~746 个路径则视为通过，若编译时间超过 5 分钟则降级为分批生成（每次处理 ≤200 个路径）后重新验证

---

## Acceptance Examples

- AE1. **Covers R1, R2.** Given PostgreSQL 不可达且所有连接被占用，当应用发起新数据库请求时，应在 10 秒内返回 `PoolError` 而非无限挂起。
- AE2. **Covers R4, R5, R7.** Given `program_center_core_entity` crate，拆分后 `lib.rs` 不超过 100 行，每个实体模块有独立的 CRUD handler 定义，`cargo test -p program_center_core_entity` 全部通过。
- AE3. **Covers R8, R9, R10, R11, R12.** Given 运行 `python scripts/gen_openapi_paths.py`，输出文件 `crates/openapi/src/lib.rs` 包含 ~746 个 `#[utoipa::path]` 函数（每个函数有 `tag`、`summary`、`parameters`（path/body）、`responses`（200/400/401/500）），多方法路由（`.route("path", put(h1).delete(h2))`）为每个 HTTP 方法生成独立路径项，`#[openapi(paths(...), tags(...), info(...))]` 属性段被完整重写而非仅更新 paths 列表，`cargo check -p openapi` 通过；若 utoipa derive 宏处理 746 个路径导致编译超时，则脚本降级为分批生成模式。
- AE4. **Covers R6, R7.** Given拆分后运行 `cargo test --workspace`，所有测试通过，无新增编译警告（除已有的预存警告外）。
- AE5. **Covers R3.** Given `POOL_WAIT_TIMEOUT_MS=5000` 环境变量已设置，应用启动后 `create_pool()` 使用的等待超时为 5 秒（非默认 10 秒）；PostgreSQL 不可达且连接池耗尽时，`pool.get().await` 在 5 秒内返回 `PoolError`。

---

## Success Criteria

- `cargo check --workspace` 通过，无新增 error
- `cargo test --workspace` 通过，无新增失败
- `crates/openapi/src/lib.rs` 包含全部 ~746 个端点的 `#[utoipa::path]` 注解
- `crates/program_center_core_entity/src/lib.rs` 不超过 100 行
- DB 连接池超时配置在 `shared/src/db.rs` 中显式声明，可通过环境变量覆盖
- 无新增跨 crate 编译错误或 warning（除已有的预存 warning）

---

## Scope Boundaries

- 仅修复 `feat/oa4rust-gap-audit` 分支的遗留问题，不引入新功能
- DB 池超时的 multi-instance 场景（分布式限流）不在范围内
- OpenAPI 的 `securitySchemes`（JWT/OAuth 安全描述）不在范围内
- 电子签名（signature.rs）相关功能不在范围内
- lib.rs 拆分后的性能基准测试不在范围内

---

## Key Decisions

- **分级交付策略**：三个问题按风险等级分为独立 PR 交付——DB 连接池超时（P0，可靠性优先，独立 PR）；OpenAPI 自动生成（P1，工具链，独立 PR）；模块拆分（P2，重构，独立 PR）。若合并为单一 PR，需保证每项功能可独立回滚（通过 feature flag 或模块边界清晰）。
- **deadpool 版本策略**：`deadpool-postgres 0.12.1` 实际依赖 `deadpool 0.10.0`（非 0.12），当前 Cargo.lock 中解析版本为 `deadpool 0.10.0`。若需使用 `Timeouts` API，必须显式将 `deadpool-postgres` 升级至 `0.14.1` 并新增 `deadpool = "0.12"` 依赖；在升级前，超时实现需在 `deadpool 0.10` 约束下寻找替代方案（如 tokio::time::timeout 包装 `pool.get().await`）
- **OpenAPI 写入策略**：脚本直接覆盖 `lib.rs`（类似 `extract_endpoints.py` 覆盖 `behavior_compare_endpoints.rs`），而非 `include!()` 模式——因为 `#[openapi(paths(...))]` 需要显式函数名列表，无法通过宏包含；**脚本必须原子写入**：先写临时文件，验证无误后 `std::fs::rename` 替换原文件，防止写入中断损坏源文件
- **模块拆分策略**：按实体拆分为子模块而非平铺——与现有 `entities/` 子模块结构保持一致，每个子模块可独立测试

---

## Dependencies / Assumptions

- `deadpool-postgres 0.12.1` 在 Cargo.lock 中已存在（Cargo.toml 声明 `"0.12"`），但实际解析为 `deadpool 0.10.0`（非 0.12）；`deadpool 0.10` 不直接暴露 `Timeouts` API，需确认升级至 `deadpool-postgres 0.14.1` 后是否可用，或采用 tokio::time::timeout 包装方案
- utoipa 4.x 的 `#[utoipa::path]` 宏支持 `responses` 字段的 OpenAPI 3.0 schema 格式
- `program_center_core_entity` 的 handler 间无跨模块引用，拆分后 `pub use` 重导出可保持公共 API 不变
- gen_openapi_paths.py 的当前路由扫描正则已修复（多行路由支持），可直接复用

---

## Outstanding Questions

### Resolve Before Planning

- [Affects R2][Technical] `deadpool-postgres 0.12.1` 实际依赖 `deadpool 0.10.0`（非 0.12），`deadpool 0.10` 无 `Timeouts` API——需确认：升级 `deadpool-postgres` 至 `0.14.1` 后是否可直接使用 `Timeouts`，或采用 `tokio::time::timeout` 包装方案。
- [Affects R9][Resolve Before Planning] 脚本如何获取请求结构体字段？当前 gen_openapi_paths.py 仅从路由路径推断——需确认是扫描 `*.rs` 中的 `struct CreateRequest` 定义，还是从 handler 签名推断。**此问题为 R9 实现的前置 spike，必须在规划阶段完成验证后再开始实现。**
- [Affects R8][Technical] 脚本生成策略需同时覆盖 `#[openapi(paths(...), tags(...), info(...))]` 整个属性段的重写——当前仅更新 paths 列表会导致 cargo check 失败，需明确是逐段替换还是生成完整 lib.rs 文件。
- [Affects R12][Resolve Before Planning] utoipa derive 宏处理 746 个路径时是否会导致编译超时——需实际验证。若超时，脚本需支持分批生成（≤200 路径/批）降级策略。

### Deferred to Planning

- [Affects R3][Technical] 超时配置值的暴露方式：环境变量（如 `POOL_WAIT_TIMEOUT_MS`）还是 `shared` crate 的配置模块？
