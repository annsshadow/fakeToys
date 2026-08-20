---
date: 2026-08-20
topic: oa4rust-remaining-gap-closure
---

# OA4Rust 剩余缺口补全 — 行为对比测试、crate 挂载、文档、handler 测试、端点对齐

## Summary

对 oa4rust 与 o2server 的"完全可代替"目标执行最后一轮缺口审计，识别出 5 个残余缺口：行为对比测试覆盖率不足、新功能 crate 挂载不完整、handler 级直接调用测试覆盖率极低、Java-Rust 端点对齐度待提升、文档 REST Endpoints 字段空白。通过补全测试清单、完成 crate 挂载、填充文档，使 oa4rust 在生产信心和功能可达性上达到全面可代替 o2server 的标准。

---

## Problem Frame

oa4rust 已完成 95 个 crate 的真实化、7624+ 条路由注册、`cargo test --workspace --lib` 全部通过。但深度审计揭示五个结构性缺口阻碍"完全代替 o2server"：

**行为对比测试几乎不执行。** `behavior_comparison/endpoints.rs` 已自动生成 1012 个端点定义，但 `behavior_compare.rs` 仅硬编码 77 个端点，且该测试被 `#[ignore]` 标记，CI 中完全静默跳过。772 个自动生成端点中 97.9% 未纳入实际对比，298 个需要认证的敏感端点（删除、更新、审批）缺乏行为回归保护。

**新功能 crate 可达性不完整。** `empower`（14 个 handler）完全未挂载到 `main.rs`，HTTP 不可达；`preview`、`realtime`、`signature` 部分挂载（功能可达但导入不规范）；`realtime` 的 `ws_stats` 路由完全缺失。

**Handler 级直接调用测试覆盖率极低。** 2690 个 `pub async fn` handler 中仅 17 个（0.6%）被直接调用测试覆盖；72% 的现有测试为路由存在性测试（仅检查非 404），不验证业务逻辑；85% 的 `tests_generated.rs` 测试因架构限制被 SKIPPED。

**Java-Rust 端点对齐度约 37%。** Rust 侧 1168 条 `.route()` 注册 vs Java 侧约 3152 个 HTTP 方法注解（@GET+@POST+@PUT+@DELETE），对齐度不足 40%。MCP 工具桥接覆盖 Rust 路由的 87%，但行为对比测试仅覆盖 6.6%。

**文档 REST Endpoints 字段全部空白。** `docs/oa/modules/o2server/` 55 张模块卡片的 "Responsibility" 字段已填充，但 "REST Endpoints" 字段全部为占位符，新成员无法通过文档快速定位 API。

这些缺口不是"有没有 crate"的问题，而是"测试保护是否充分"、"功能是否可达"、"文档是否可用"的问题，直接关系到 oa4rust 能否在质量信心上完全代替 o2server。

---

## Actors

- A1. **开发者**：执行测试补全、crate 挂载修复、文档填充
- A2. **CI 流水线**：`cargo test --workspace --lib` 必须通过，覆盖率门禁达标
- A3. **前端 o2web**：依赖一致的 API 契约和错误响应结构
- A4. **新加入开发者**：通过 `docs/oa/` 快速建立系统心智模型

---

## Key Flows

- **F1. 行为对比测试扩展流**
  - **Trigger：** 识别行为对比测试覆盖率不足后启动补全
  - **Actors：** A1, A2
  - **Steps：** 1. 将 `behavior_compare.rs` 的硬编码端点清单扩展至覆盖全部 1012 个自动生成端点 2. 移除 `#[ignore]` 标记或改为条件执行（Java 不可达时自动 SKIP）3. 确保测试在 CI 中可执行且不因 Java 不可达而 FAIL
  - **Outcome：** 行为对比测试覆盖率从 6.6% 提升至覆盖全部已实现端点，CI 中具备执行保证
  - **Covered by：** R1, R2, R3

- **F2. 新功能 crate 挂载修复流**
  - **Trigger：** 识别 `empower` 未挂载、`preview/realtime/signature` 部分挂载后启动修复
  - **Actors：** A1, A2
  - **Steps：** 1. 为 `empower` 添加 `use empower` 和 `.merge(empower::router(...))` 2. 为 `preview/signature` 补充 `use` 声明 3. 为 `realtime` 补充 `use` 声明并挂载 `ws_stats` 路由 4. 验证 `cargo test --workspace --lib` 通过
  - **Outcome：** 所有有业务代码的 crate 全部 HTTP 可达，导入规范统一
  - **Covered by：** R4, R5, R6

- **F3. Handler 直接调用测试补全流**
  - **Trigger：** 识别 handler 级测试覆盖率仅 0.6% 后启动补全
  - **Actors：** A1, A2
  - **Steps：** 1. 对 `processplatform_assemble_surface`（487 handlers）、`cms_assemble_control`（312 handlers）等大模块优先补全直接调用测试 2. 对无 DB 依赖的 handler 使用 stub mock pool 3. 对有 DB 依赖的 handler 使用真实 PostgreSQL 连接 4. 通过 Python 脚本批量生成测试桩，人工填充断言
  - **Outcome：** 业务 handler 直接调用测试覆盖率从 0.6% 提升至 ≥ 50%
  - **Covered by：** R7, R8, R9, R10

- **F4. 端点对齐度提升流**
  - **Trigger：** 识别 Java-Rust 端点对齐度约 37% 后启动补全
  - **Actors：** A1, A3
  - **Steps：** 1. 对照 Java `@Path` 注解（365 个文件、3152 个 HTTP 方法），识别 Rust 侧缺失的端点 2. 对高优先级缺口（如 `x_organization_assemble_authentication`、`x_organization_assemble_personal` 的剩余端点）实现 Rust handler 3. 保持与现有 `ActionResult<T>` 9 字段结构兼容 4. 运行行为对比测试验证新端点
  - **Outcome：** Java-Rust 端点对齐度从 37% 提升至 ≥ 70%
  - **Covered by：** R11, R12, R13

- **F5. 文档 REST Endpoints 填充流**
  - **Trigger：** 识别文档 REST Endpoints 字段全部空白后启动填充
  - **Actors：** A1, A4
  - **Steps：** 1. 基于 `oa4rust/tests/behavior_comparison/endpoints.rs` 的 1012 个端点定义，按 crate 分组 2. 为 `docs/oa/modules/o2server/` 下 55 张模块卡片的 "REST Endpoints" 字段填充端点列表 3. 验证 `docs/oa/README.md` 中所有链接可正常解析
  - **Outcome：** 55 张模块卡片的 REST Endpoints 字段全部填充，新成员可在 10 分钟内建立系统整体心智模型
  - **Covered by：** R14, R15

---

## Requirements

**行为对比测试扩展**
- R1. 将 `oa4rust/tests/behavior_compare.rs` 的硬编码端点清单从 77 个扩展至覆盖全部 1012 个自动生成端点（去重后约 992 个 unique rust_path）
- R2. 移除 `behavior_compare_rust_vs_java` 测试函数的 `#[ignore]` 标记，或改为条件执行（如通过环境变量 `BEHAVIOR_COMPARE=1` 控制），确保 CI 中可根据配置执行
- R3. 保持 Java 服务不可达时的 SKIP 降级机制：Java 不可达时全部端点标记为 SKIP 而非 FAIL，测试套件整体通过；Rust 不可达时仍 FAIL

**新功能 crate 挂载修复**
- R4. 为 `empower` crate 在 `oa4rust/src/main.rs` 中添加 `use empower` 声明和 `.merge(empower::router(pool.clone()))` 调用，使其 14 个 handler HTTP 可达
- R5. 为 `preview`、`signature` crate 在 `main.rs` 中补充 `use preview` / `use oa4rust_signature` 声明（已有 `.merge()` 调用，仅缺导入）
- R6. 为 `realtime` crate 在 `main.rs` 中补充 `use realtime` 声明，并将 `ws_stats` 路由挂载到现有 WebSocket 子应用

**Handler 直接调用测试补全**
- R7. 对 `processplatform_assemble_surface`（487 handlers）、`cms_assemble_control`（312 handlers）、`organization_assemble_control`（105 handlers）三个大模块，每个模块至少补全 50 个 handler 的直接调用测试
- R8. 对无 DB 依赖的纯逻辑 handler（参数仅为 `Path`、`Json`、`Extension<Pool>` 中不含实际查询的部分），使用 stub mock pool 即可满足覆盖要求
- R9. 对有 DB 依赖的 handler，测试需使用真实 PostgreSQL 连接，handler 执行到实际 SQL 查询，断言返回 `Ok(Json(ActionResult<...>))` 且状态为成功（`type == "success"` 或 `type == "error"` 均视为覆盖）
- R10. 通过 Python 脚本批量解析各 crate 的 `src/lib.rs`，提取所有 `pub async fn` 名称及其签名，自动生成 `tests_generated.rs` 测试桩（包含基本调用结构），再由人工或脚本填充具体断言

**Java-Rust 端点对齐度提升**
- R11. 对照 Java `@Path` 注解（365 个文件、约 3152 个 HTTP 方法注解），识别 Rust 侧缺失的高优先级端点（认证安全、个人中心、流程平台等核心模块）
- R12. 对识别出的缺失端点，实现对应的 Rust handler，保持与现有 `ActionResult<T>` 9 字段结构兼容
- R13. 新增端点必须通过行为对比测试验证（当 Java 服务可达时），确保 Rust 与 Java 响应结构一致

**文档 REST Endpoints 填充**
- R14. 基于 `oa4rust/tests/behavior_comparison/endpoints.rs` 的 1012 个端点定义，为 `docs/oa/modules/o2server/` 下 55 张模块卡片的 "REST Endpoints" 字段填充实际端点列表
- R15. 验证 `docs/oa/README.md` 中所有链接可正常解析，确保文档导航完整

---

## Acceptance Examples

- AE1. **Covers R1, R2, R3.** Given 运行 `cargo test --test behavior_compare`，when 环境变量 `BEHAVIOR_COMPARE=1` 已设置，then 测试执行全部 1012 个端点（去重后约 992 个 unique paths）；Java 不可达时全部标记为 SKIP，测试套件通过；Rust 不可达时测试 FAIL
- AE2. **Covers R4.** Given 向 `empower` 的任一 handler 发送 HTTP 请求，when `main.rs` 已完成挂载，then 请求返回非 404 响应，handler 正常执行
- AE3. **Covers R7, R8.** Given `processplatform_assemble_surface` 的 487 个 handler，when 运行其测试套件，then 至少 50 个 handler 被直接调用测试覆盖；无 DB 依赖的 handler 使用 stub mock pool，有 DB 依赖的 handler 使用真实 PostgreSQL
- AE4. **Covers R11, R12, R13.** Given Java 服务可达，when 新增端点实现后运行行为对比测试，then 新端点通过对比验证，Rust 与 Java 响应结构一致；Java 不可达时标记为 SKIP
- AE5. **Covers R14, R15.** Given 随机抽查 5 张模块卡片，when 查看其 "REST Endpoints" 字段，then 每张卡片包含非空的端点列表；`docs/oa/README.md` 中所有链接可正常解析

---

## Success Criteria

- `behavior_compare.rs` 覆盖全部 992 个 unique 端点，`#[ignore]` 标记移除或改为条件执行
- `cargo test --workspace --lib` 全部通过，无新增编译错误或警告
- `empower`、`preview`、`realtime`、`signature` 的导入和挂载规范统一，所有 handler HTTP 可达
- 业务 handler 直接调用测试覆盖率从 0.6% 提升至 ≥ 50%（至少覆盖 1345 个 handler）
- Java-Rust 端点对齐度从 37% 提升至 ≥ 70%
- `docs/oa/modules/o2server/` 55 张模块卡片的 REST Endpoints 字段全部填充
- 无新增 `todo!()`、`unimplemented!()`、`stub` 标记

---

## Scope Boundaries

- **包含：** 行为对比测试清单扩展与 `#[ignore]` 处理；`empower/preview/realtime/signature` 的 main.rs 挂载修复；handler 直接调用测试补全；Java-Rust 端点对齐度提升；文档 REST Endpoints 字段填充
- **排除在外：** 前端 o2web 代码修改；Java 后端代码修改；数据库迁移或 schema 变更；新增 Java 不存在的新功能；修改现有业务逻辑代码（仅新增测试/挂载/文档）

### Deferred for later

- 行为对比测试的 Java 侧服务容器化（当前依赖外部 JAVA_SERVICE_URL）
- handler 级测试覆盖率提升至 99%（本轮目标 ≥ 50%）
- 文档模块卡片的 Key Flows、Dependencies 等字段深度填充
- `docs/oa/modules/o2web/` 下组件卡片的填充
- SQLx 完全移除（按已有策略保留并存）

### Outside this product's identity

- 前端 o2web 的重写或现代化改造
- 独立的 OAuth 提供商 SDK 发布
- Java 服务的永久下线决策
- 微服务拆分或架构重构

---

## Key Decisions

- **行为对比测试从 `#[ignore]` 改为条件执行：** 保留 Java 不可达时的 SKIP 降级机制，但移除无条件跳过，允许 CI 通过环境变量控制执行
- **`empower` 完全挂载而非仅部分功能暴露：** 14 个 handler 全部通过 `router()` 挂载，与现有 assemble_control crate 模式一致
- **Handler 测试优先覆盖大模块：** `processplatform_assemble_surface`、`cms_assemble_control`、`organization_assemble_control` 三个模块占总 handler 数的 33%，优先补全可最大化覆盖率提升
- **文档填充基于现有端点清单：** `behavior_comparison/endpoints.rs` 的 1012 个端点定义作为 REST Endpoints 字段的数据源，不依赖 Swagger 或手动整理
- **Java-Rust 端点对齐度以 HTTP 方法注解为统计口径：** 以 Java 侧的 @GET/@POST/@PUT/@DELETE 总数为分母，Rust 侧的 `.route()` 注册数为分子

---

## Dependencies / Assumptions

- Java o2server 的 `@Path` 注解（365 个文件、约 3152 个 HTTP 方法注解）可作为 Rust 实现的参考契约
- `behavior_comparison/endpoints.rs` 的 1012 个端点定义已覆盖全部已实现的 Rust 路由
- `empower` crate 的 `router()` 函数已完整实现 14 个 handler 的路由组装
- `realtime` crate 的 `ws_stats` 路由逻辑已实现，仅需挂载
- 前端 o2web 对 `ActionResult<T>` 的 9 字段结构有隐式依赖，新增端点必须保持兼容

---

## Outstanding Questions

### Resolve Before Planning

- [Affects R4] `empower` crate 的 router 函数签名是否与 main.rs 中其他 `.merge()` 调用兼容？需要确认 `empower::router(pool)` 的参数类型

### Deferred to Planning

- [Affects R7][Needs research] 大模块的 50 个 handler 直接调用测试中，有多少 handler 因架构限制（内部 service 函数未导出）无法直接调用，需要调整导出策略
- [Affects R11][Needs research] Java 侧 3152 个 HTTP 方法注解中，有多少对应 Rust 侧已实现但未注册行为对比测试的端点
- [Affects R2][Technical] 行为对比测试的条件执行机制：使用环境变量 `BEHAVIOR_COMPARE=1` 还是其他 CI 配置方式
