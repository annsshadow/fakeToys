---
date: 2026-08-13
topic: oa4rust-handler-test-coverage-99
---

# OA4Rust 业务 Handler 单元测试覆盖率提升至 99%

## Summary

对 oa4rust 所有业务 crate（排除 mcp_server、openapi）中的 handler 函数，通过脚本化方式生成并补充单元测试，使至少 99% 的 `pub` handler 被一个测试直接调用，测试借助 Docker PostgreSQL 支撑有 DB 依赖的 handler 路径。

---

## Problem Frame

oa4rust 已完成约 2,592 个 handler 函数的实现（88 个 crate），但 handler 级测试覆盖率极低：现有 840 个测试中大量是路由存在性验证或数据序列化测试，而非真正调用各 handler 业务逻辑。约 1,765 个 handler 目前零测试覆盖，导致任何业务逻辑回归都无法通过单元测试检出。距离"完全代替 o2server"的生产标准有显著差距。

---

## Actors

- A1. **开发者**：执行测试补全，维护测试基础设施
- A2. **CI 流水线**：`cargo test --workspace --lib` 必须通过，覆盖率门禁达标

---

## Requirements

**覆盖率目标**
- R1. 除 mcp_server、openapi 之外的所有业务 crate，其 `pub` handler 函数中至少 99% 被一个单元测试直接调用（直接调用指在 `#[tokio::test]` 或 `#[test]` 函数体内显式调用 `crate::handler_name(...)` 或 `super::handler_name(...)`，不要求通过 router/HTTP 转发）
- R2. 计算分母时仅统计 `pub` handler 函数，不含内部辅助函数（`fn helper`、`async fn private_fn` 等）；mcp_server 和 openapi 两个 crate 完全不纳入统计
- R3. `cargo test --workspace --lib` 全部通过，无新增编译错误或警告

**测试基础设施**
- R4. 使用本机 Docker PostgreSQL（已有容器映射到 5433 端口）作为测试数据库，测试启动时自动创建临时测试库并应用 migrations，测试结束后删除该库
- R5. 复用 `tests/integration_tests/db.rs` 中的 `init_test_database()` / `drop_database()` 模式建立测试 PG 生命周期管理
- R6. 测试连接字符串通过环境变量或 `build_test_pool()` 辅助函数配置，默认指向 `localhost:5433`，用户密码为 `password`，用户名为 `postgres`

**测试生成策略**
- R7. 对无 DB 依赖的纯逻辑 handler（参数仅为 `Path`、`Json`、`Extension<Pool>` 中不含实际查询的部分），使用 stub mock pool（不调用真实 DB）即可满足覆盖要求
- R8. 对有 DB 依赖的 handler，测试需使用真实 PG 连接，handler 执行到实际 SQL 查询，断言返回 `Ok(Json(ActionResult<...>))` 且状态为成功（`type == "success"` 或 `type == "error"` 均视为覆盖）
- R9. 通过 Python 脚本批量解析各 crate 的 `src/lib.rs`，提取所有 `pub async fn` 名称及其签名，自动生成 `tests_generated.rs` 测试桩（包含基本调用结构），再由人工或脚本填充具体断言
- R10. 对 `tests.rs` 已存在但测试数为 0 的 crate，直接在现有 `tests.rs` 中补充 handler 调用测试，不新建文件

**覆盖缺失重点处理**
- R11. 优先处理 handler 数 > 50 的大模块：processplatform_assemble_surface（488 handlers）、cms_assemble_control（313 handlers）、organization_assemble_control（106 handlers）、processplatform_service_processing（100 handlers）、file_assemble_control（96 handlers）、processplatform_assemble_designer（97 handlers）、general_assemble_control（63 handlers）、meeting_assemble_control（61 handlers）、portal_assemble_designer（58 handlers）、message_assemble_communicate（59 handlers）
- R12. 零测试 crate（ldap、organization_assemble_authentication、organization_assemble_personal）必须补全全部 handler 的直接调用测试
- R13. 对已有 `tests_generated.rs` 的 10 个 crate，检查现有 stub 是否覆盖了所有 handler；未覆盖的补齐

**验收标准**
- R14. 执行覆盖率统计脚本后，输出报告包含每个 crate 的：总 handler 数、已覆盖 handler 数、覆盖率百分比；总体覆盖率 ≥ 99%
- R15. 测试执行时间可控：单个 crate 的测试套件不超过 30 秒（排除首次 PG 启动开销）

---

## Acceptance Examples

- AE1. **Covers R1, R14.** Given 运行覆盖率统计脚本，when 分析所有业务 crate，then 报告整体 handler 覆盖率 ≥ 99%，每个大模块覆盖率 ≥ 95%
- AE2. **Covers R3.** Given 所有测试补全后，when 运行 `cargo test --workspace --lib`，then 全部测试通过，无编译警告
- AE3. **Covers R11.** Given program_center 模块，when 运行其测试，then 205 个 handler 中至少 204 个被直接调用
- AE4. **Covers R12.** Given ldap crate，when 运行 `cargo test -p ldap`，then 5 个 handler 全部被测试调用

---

## Success Criteria

- `cargo test --workspace --lib` 全部通过
- 覆盖率报告显示业务 handler 覆盖率 ≥ 99%
- 无新增编译警告（`cargo check --workspace`）
- Docker PG 容器测试后保持可用状态（无残留临时库）

---

## Scope Boundaries

- mcp_server 和 openapi 两个 crate 的 handler 不计入覆盖率分母
- 不修改任何业务逻辑代码（仅新增测试）
- 不修改现有测试（仅在已有 tests.rs 中追加，或新建 tests_generated.rs）
- 不引入新的测试框架或第三方依赖
- 不覆盖集成测试（已有 `tests/integration_tests/` 基础设施）
- 不修改 Docker PG 容器的基础配置（仅创建/销毁临时数据库）
- 不保证每个 handler 都有 happy path + error path 双重覆盖（99% 只要求"至少被调用一次"）

---

## Key Decisions

- **直接调用 handler 而非通过 HTTP 路由**：每个 handler 函数签名可以直接传入，避免构造完整 HTTP 请求和中间件栈的复杂度；对无 DB 依赖的 handler 尤其高效
- **Python 脚本批量生成测试桩**：2,500+ handler 手动编写不可行；通过解析 lib.rs 提取函数名和参数模式自动生成骨架，再按需填充断言
- **复用集成测试 PG 基础设施**：已有 `tests/integration_tests/db.rs` 的容器生命周期管理可直接复用，避免引入新的测试数据库机制
- **分阶段推进大模块**：handler 密度最高的模块优先处理，每完成一个模块即运行测试验证，避免最后集中修复大量失败

---

## Dependencies / Assumptions

- Docker PostgreSQL 容器（yhmbs_pg_test，端口 5433）持续可用；若容器不存在，测试脚本需能启动新容器
- migrations 目录（`migrations/*.sql`）完整覆盖所有 handler 依赖的表结构
- 现有 `tests_generated.rs` 生成模式可扩展（当前 10 个 crate 已有此模式）
- `cargo test` 支持并发执行（`--jobs` 参数），各 crate 测试互不干扰

---

## Outstanding Questions

### Resolve Before Planning

（无阻塞问题）

### Deferred to Planning

- [Affects R4][Needs research] Docker PG 容器认证方式：当前容器使用 `POSTGRES_HOST_AUTH_METHOD=trust`，用户名/密码连接方式需确认可用性
- [Affects R7][Technical] 判断 handler 是否"无 DB 依赖"的自动规则——可通过静态分析检测函数体中是否包含 `.query()` / `.execute()` / `.query_opt()` / `.query_one()` 等调用
- [Affects R9][Needs research] Python 脚本解析 Rust 签名的精度——需处理 async、多个 extract 参数（Path、Json、Extension）的组合情况
