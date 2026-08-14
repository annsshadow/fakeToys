---
date: 2026-08-12
topic: oa4rust-interface-audit-and-test-coverage
---

# OA4Rust 接口对齐审计与测试覆盖率提升

## Summary

系统性地审计 oa4rust 与 Java o2server 在接口、请求响应和业务逻辑层面的差距，补全无测试 crate 和薄弱模块的单元测试，使 oa4rust 达到单元测试级别的 o2server 等价性验证状态，并将 handler 级测试覆盖率提升至 95% 以上。

---

## Problem Frame

oa4rust 已完成 86 个 crate 的路由注册和 2,623 个 handler 函数的实现，`cargo test --workspace --lib` 当前通过 735 个测试。但扫描揭示两个结构性缺口：其一，oa4rust 的 handler 与 Java o2server 的端点在请求参数、响应结构、业务规则上存在未验证的对齐差距；其二，测试覆盖率严重不均衡——多个 crate 测试不足（ldap、organization_assemble_authentication、organization_assemble_personal 等完全无测试；jpush_assemble_control 等有 tests.rs 但测试数为 0），15+ 个大模块（program_center 205 个 handler 实际 5 tests、organization_assemble_control 105 个 handler 实际 8 tests 等）的 handler 测试覆盖率低于 10%，整体 handler 级测试覆盖率约 15%。距离"完全代替 o2server"的生产标准仍有显著差距。

---

## Actors

- A1. **开发者**：执行接口审计和测试补全
- A2. **前端 o2web**：依赖 oa4rust 的 API 契约，任何接口偏差会导致联调失败
- A3. **AI Agent / MCP 客户端**：通过 MCP 工具调用 oa4rust，需要一致的请求/响应行为
- A4. **CI 流水线**：`cargo test --workspace` 必须通过，覆盖率门禁达标

---

## Key Flows

- **F1. 接口对齐审计流**
  - **Trigger：** 需确认 oa4rust 能否完全代替 o2server
  - **Actors：** A1
  - **Steps：** 1. 提取 Java o2server 全部端点清单 2. 提取 oa4rust 全部端点清单 3. 逐项对比路径、方法、参数、响应结构 4. 标记差距（缺失/结构不符/逻辑不对齐）
  - **Outcome：** 一份完整的接口差距报告
  - **Covered by:** R1-R3

- **F2. 无测试 crate 补测流**（里程碑 M2）
  - **Trigger：** ldap、organization_assemble_authentication、organization_assemble_personal 无测试
  - **Actors：** A1
  - **Steps：** 1. 识别各 crate 的 handler 函数 2. 为每个 handler 编写基础路由存在性测试和错误路径测试
  - **Outcome：** 4 个零测试 crate 全部覆盖（mcp_server/openapi 仅编译验证）
  - **Covered by:** R4

- **F3. 大模块补测流**（里程碑 M2）
  - **Trigger：** program_center、organization_assemble_control 等大模块 handler 覆盖率 <10%
  - **Actors：** A1
  - **Steps：** 1. 按 handler 数排序确定优先级 2. 为低覆盖模块编写 handler 级测试 3. 每测试覆盖至少一个 handler 的 happy path + error path
  - **Outcome：** 所有业务模块 handler 测试覆盖率 ≥95%（排除自动生成代码 stub）
  - **Covered by:** R5-R7

---

## Requirements

**接口对齐审计**（里程碑 M1）
- R1. 生成 oa4rust 与 Java o2server 的完整端点对照表，包含：Java 端点路径、Rust 端点路径、HTTP 方法、handler 函数名、实现状态（已实现/缺失/部分实现）、响应结构差异
- R2. 对"部分实现"的端点，记录具体的业务逻辑差距（如：缺少 locked 检查、passwordExpired 逻辑不完整、响应字段缺失等），并分类为 P0（阻断替换）/P1（功能偏差）/P2（可选优化）
- R3. 审计结果以结构化文档形式交付，包含所有端点的对比结论和差距说明，供后续测试补全参考；P0 项必须在本次工作内修复

**无测试 crate 补测 + 大模块补测**（里程碑 M2）
- R4. 为以下 4 个零测试 crate 添加基础测试（路由存在性 + 错误路径）：ldap、organization_assemble_authentication、organization_assemble_personal。openapi 为自动生成代码，仅需验证编译通过

**大模块补测（按 handler 密度排序）**
- R5. program_center（205 handlers，5 tests，覆盖率 2.4%）：补全核心 CRUD handler 测试，目标覆盖率 ≥60%
- R6. organization_assemble_control（105 handlers，8 tests，覆盖率 7.6%）：补全 person/group/role/unit 核心操作测试，目标覆盖率 ≥60%
- R7. 明确列出所有覆盖率 <20% 的 crate 清单（当前已知：program_center、organization_assemble_control、general_assemble_control、file_assemble_control、attendance_assemble_control、cms_assemble_control、processplatform_service_processing、processplatform_assemble_surface、message_assemble_communicate 等），按 handler 数降序依次补测，直至整体覆盖率达标

**覆盖率目标**
- R8. 整体 handler 级测试覆盖率 ≥95%（计算口径：排除 mcp_server、openapi 等自动生成代码中的 stub handler，仅统计实际业务 handler）
- R9. 每个 handler 至少有一个测试用例覆盖 happy path，关键 handler（认证、写操作）需覆盖 error path
- R10. `cargo test --workspace --lib` 全部通过，无新增编译警告

---

## Acceptance Examples

- AE1. **Covers R1, R2.** Given 运行接口审计脚本，when 输出差距报告，then 包含全部业务 handler 的状态标记和响应结构差异，所有"部分实现"项有具体的业务逻辑差距说明，P0 项数为 0
- AE2. **Covers R4.** Given 运行 `cargo test -p ldap`，when 测试执行时，then 至少通过 1 个测试用例
- AE3. **Covers R5.** Given program_center 的测试文件，when 运行 `cargo test -p program_center`，then 测试数从 5 增至 ≥124 个
- AE4. **Covers R8.** Given 所有补测完成后，when 运行覆盖率统计（排除自动生成代码 stub handler），then handler 级覆盖率 ≥95%
- AE5. **Covers R10.** Given 所有更改提交后，when 运行 `cargo test --workspace --lib`，then 735+ 测试全部通过，无编译错误

---

## Success Criteria

**里程碑 M1（接口审计）**
- 接口差距报告完整记录所有端点对比结论，P0 项数为 0
- 报告中明确列出各模块覆盖率基线数据

**里程碑 M2（测试补全）**
- 4 个零测试 crate 全部有测试覆盖
- program_center 和 organization_assemble_control 覆盖率 ≥60%
- 整体 handler 级测试覆盖率 ≥95%（排除自动生成代码 stub handler）
- `cargo test --workspace --lib` 全部通过
- `cargo check --workspace` 无新增警告

---

## Scope Boundaries

- 仅审计和补测，不修改业务逻辑代码（除非发现 P0 级差距需修复）
- 自动生成的代码（mcp_server、openapi）仅验证编译通过，不强制添加测试
- 覆盖率计算排除自动生成代码中的 stub handler
- 不修改 Java o2server 代码
- 不修改前端 o2web 代码
- 覆盖率计算基于 handler 函数粒度，非代码行粒度
- 测试采用参数化+数据驱动模式编写，复用度 ≥80%，以降低长期维护成本

### Deferred for later

- 集成测试（integration tests）的全面覆盖
- 性能测试和压力测试
- 使用 grcov/tarpaulin 等工具进行精确代码行覆盖率测量
- 边界条件测试的深度覆盖（并发、超时、数据完整性）

### Outside this product's identity

- 前端 o2web 的重写或现代化改造
- Java o2server 的维护或增强
- CI/CD 流水线的重构

---

## Key Decisions

- **handler 级覆盖率而非代码行覆盖率**：每个 handler 函数至少有一个测试，比行覆盖率更能反映业务逻辑覆盖；计算时排除 mcp_server、openapi 等自动生成代码中的 stub handler
- **分阶段交付**：M1 完成接口审计并修复 P0 差距，M2 基于审计结果针对性补测
- **优先补全零测试和大模块**：4 个零测试 crate 成本低、收益明确；大模块 handler 数多，补测后覆盖率提升显著
- **接口审计先行**：先明确差距再针对性补测，避免盲目添加测试
- **测试风格复用现有模式**：复用 `build_test_pool()` + `oneshot()` 的集成测试模式，与现有测试保持一致

---

## Dependencies / Assumptions

- Java o2server 代码可作为接口参照契约
- 现有测试模式（`build_test_pool` + `oneshot`）可扩展到新模块
- `docs/brainstorms/oa4rust-endpoint-inventory.md` 中的端点清单可作为审计基础
- 不需要引入新的测试框架或覆盖率工具

---

## Outstanding Questions

### Resolve Before Planning

（无阻塞问题，所有方向已在对话中明确）

### Deferred to Planning

- [Affects R1][Needs research] Java o2server 端点的精确提取方式——需确认是从 Java 源码提取还是从已有文档提取
- [Affects R8][Technical] 95% 覆盖率的精确计算口径——排除 mcp_server、openapi 自动生成代码中的 stub handler
- [Affects R5-R7][Needs research] 大模块测试的优先级排序——基于 o2web 实际调用频率还是 handler 数量
