---
date: 2026-08-09
topic: oa4rust-orm-migration-and-write-ops
---

# OA4Rust 全量 ORM 迁移与写操作补齐

## Summary

以 SeaORM 为统一数据访问层渐进替换全量 81 个 crate 的 SQLx 原生查询，同步规范化 PostgreSQL schema 命名，为此前仅有 GET 查询的 8 个 core_entity crate 及相关模块补全 POST/PUT/DELETE 写操作，并建立全量行为对比测试框架（含 Java 服务不可用时的降级策略），覆盖全部 7,624 个端点。

---

## Problem Frame

OA4Rust 项目在 2026-08-08 已完成全部 81 个 crate 的真实化，但存在三个结构性缺陷：

**数据访问层零散且无类型安全。** 73 个真实化 crate 使用 SQLx 原生 SQL + `query!` 宏，8 个 core_entity crate 大量使用 `serde_json::Value` 手动映射行字段，缺乏编译期类型检查、实体关系表达和迁移感知能力。不同 crate 的数据访问模式不一致，增加了维护成本和错误风险。

**写操作严重缺失。** 8 个 core_entity crate（organization_core_entity、file_core_entity、program_center_core_entity、ai_core_entity、cms_express、correlation_core_entity、query_express 等）仅有 GET 查询端点，无 POST/PUT/DELETE 写操作。这意味着这些模块的完整 CRUD 能力未实现，前端无法进行数据修改操作。

**行为对比测试覆盖不足。** 目前仅 75 个端点有 Rust vs Java 行为对比测试，占全部 7,624 个端点的约 1%。随着写操作的引入，需要一个可扩展的对比测试框架来确保功能等效性，但 Java 服务可能已下线，要求框架具备降级能力。

此外，PostgreSQL schema 存在命名风格不一致问题（`FILE_FOLDER` 大写表名与 `auth_person` 小写表名混用），需要在 ORM 迁移过程中规范化。

---

## Actors

- **A1（开发者，单人）：** 负责 ORM 迁移、写操作实现、测试框架扩展
- **A2（现有 Java 后端）：** 作为行为对比基准，在可用时提供响应参考
- **A3（前端 o2web）：** 依赖 `ActionResult<T>` 9 字段 JSON 结构，任何修改必须保持兼容
- **A4（下游规划 Agent）：** 读取本需求文档后执行 ce-plan 进行详细规划

---

## Key Flows

- **F1. ORM 波次迁移流**
  - **Trigger：** 启动新一波 crate ORM 迁移
  - **Actors：** A1
  - **Steps：**
    1. 在 `crates/orm` 中定义 SeaORM 实体和通用 CRUD 助手
    2. 将目标 crate 的 SQLx 查询替换为 ORM 调用
    3. 更新迁移文件（如需要 schema 规范化）
    4. 运行 `cargo test --workspace --lib` 验证无回归
    5. 更新 `docs/brainstorms/oa4rust-migration-status-2026-08-08.md`
  - **Outcome：** 目标 crate 完成 ORM 化，状态从"SQLx 原生"更新为"SeaORM"
  - **Covered by：** R1-R6

- **F2. 写操作补全流程**
  - **Trigger：** ORM 迁移完成后，为特定 crate 补充写操作
  - **Actors：** A1, A3
  - **Steps：**
    1. 基于 ORM 实体定义请求 DTO（serde derive）
    2. 实现 POST/PUT/DELETE handler，包含参数验证和权限校验
    3. 添加单元测试和集成测试
    4. 更新行为对比测试端点清单（含 Java 端点映射）
  - **Outcome：** crate 具备完整 CRUD 能力，测试覆盖新增端点
  - **Covered by：** R7-R12

- **F3. 行为对比测试扩展流**
  - **Trigger：** 新增或修改端点后
  - **Actors：** A1, A2, A3
  - **Steps：**
    1. 在 `tests/behavior_compare.rs` 的 `ENDPOINTS` 数组中添加新端点
    2. 配置 allowlist.yaml 中的字段命名差异规则
    3. 运行对比测试；Java 不可用时自动降级为 mock 响应模式
    4. 生成并审核对比报告
  - **Outcome：** 所有端点均有行为对比测试覆盖或明确标记为已跳过
  - **Covered by：** R13-R16

---

## Requirements

**ORM 基础层构建**
- R1. 在 `crates/orm` 中构建 SeaORM 共享层，包含：实体派生宏封装、通用 CRUD 助手（list/create/update/delete）、软删除支持、分页查询助手
- R2. ORM 层统一处理 `ActionResult<T>` 响应包装，消除各 crate 中重复的 `Value::Object(serde_json::Map::from_iter(...))` 手动映射模式
- R3. ORM 层需与现有 `deadpool_postgres::Pool` 兼容，通过 `sea_orm::DatabaseConnection` 包装

**Schema 规范化**
- R4. 规范化 PostgreSQL 表名和列名命名风格：统一使用小写 + 下划线（snake_case），编写数据迁移脚本将现有大写表名（如 FILE_FOLDER → file_folder）同步重命名，所有引用该表的查询和 ORM 实体同步更新
- R5. 为所有新表/列添加外键约束和级联删除策略，消除孤立记录风险
- R6. 迁移文件需支持幂等执行（`IF NOT EXISTS` + 条件检查），确保现有数据库可安全升级

**ORM 波次迁移**
- R7. Wave 1（核心实体层）：将 8 个 core_entity crate（organization_core_entity、file_core_entity、program_center_core_entity、ai_core_entity、cms_core_entity、bbs_core_entity、calendar_core_entity、meeting_core_entity）从 SQLx 迁移至 SeaORM
- R8. Wave 2（组装控制层）：将 15+ 个 assemble_control crate 从 SQLx 迁移至 SeaORM
- R9. Wave 3（其余模块）：将剩余所有 crate 从 SQLx 迁移至 SeaORM
- R10. 每波迁移完成后必须通过 `cargo test --workspace --lib` 全量回归测试，确保不影响其他 crate
- R11. 迁移过程中保留 SQLx 用于极少数复杂查询/性能热点（如动态 SQL 执行、大批量导入），ORM 与 SQLx 可并存但 ORM 为默认路径

**写操作补齐**
- R12. 为全部 8 个 core_entity crate 补全 POST/PUT/DELETE 写操作，实现完整的 CRUD 能力
- R13. 所有写操作端点必须包含输入参数验证（类型、长度、格式）和权限校验（与现有 `authorize_middleware` 对齐）
- R14. 写操作响应格式必须与 Java 端点行为等效（字段名、类型、分页语义）

**行为对比测试**
- R15. 行为对比测试框架需扩展至覆盖全部 7,624 个端点（含新增写操作端点）
- R16. 框架需支持 Java 服务不可用时的降级策略：自动切换为静态 mock 响应文件模式，记录 SKIP 状态而非 FAIL，并在报告中标注降级原因；mock 响应文件需覆盖所有端点的关键字段结构
- R17. 新增端点的 Java 映射信息需纳入 `tests/behavior_compare.rs` 的 `ENDPOINTS` 数组，包含 rust_path、java_war、java_action 字段
- R18. allowlist.yaml 需扩展以覆盖 Rust camelCase 与 Java snake_case 的所有命名差异（当前仅覆盖时间戳字段）

**文档与维护**
- R19. 每波迁移完成后立即更新 `docs/brainstorms/oa4rust-migration-status-2026-08-08.md`，记录各 crate 的 ORM 状态（SQLx → SeaORM）
- R20. 更新 `docs/brainstorms/oa4rust-endpoint-inventory.md`，标记新增写操作端点的实现状态
- R21. ORM 迁移完成后，在 `docs/brainstorms/` 中创建 `oa4rust-orm-migration-guide.md`，记录迁移模式、常见陷阱和最佳实践

**前端兼容性**
- R22. 所有 ORM 迁移和写操作实现必须保持 `ActionResult<T>` 9 字段 JSON 结构不变（data/type/message/date/spent/size/count/position/prompt）
- R23. ORM 迁移不得改变任何已有端点的响应字段名称、类型或分页语义，确保前端 o2web 无需适配

---

## Acceptance Examples

- **AE1. Covers R1, R2, R7.** 给定 organization_core_entity crate 中的人员列表端点，使用 SeaORM 实体替代 SQLx 原生查询后，`cargo test` 全部通过，响应格式与迁移前完全一致。
- **AE2. Covers R4, R5.** 给定 FILE_FOLDER 表，规范化为 file_folder 小写命名后，所有引用该表的查询和 ORM 实体同步更新，迁移文件幂等执行成功，无孤立记录。
- **AE3. Covers R12, R13.** 给定 organization_core_entity crate，补充 POST /jaxrs/organization/person 端点后，传入有效请求体成功创建人员并返回 200 + type=success；传入缺少必填字段的请求体返回 400 + type=error；未认证用户调用返回 401。
- **AE4. Covers R16, R18.** 给定 Java 服务不可用，运行行为对比测试时所有端点标记为 SKIP 并在报告中注明降级原因，测试通过而非失败；allowlist 覆盖全部已知的字段命名差异。
- **AE5. Covers R22, R23.** 给定任意已完成 ORM 迁移的端点，前端 o2web 的 action.js 能正常解析响应 JSON，无需任何前端代码修改。
- **AE6. Covers R15.** 给定全部 7,624 个端点，行为对比测试框架能枚举并对比每一个端点（Java 可用时为 Pass/Fail，Java 不可用时为 SKIP），报告无遗漏。

---

## Success Criteria

- **业务结果：** 全部 81 个 crate 使用 SeaORM 作为主要数据访问层，8 个 core_entity crate 具备完整 CRUD 能力，前端 o2web 无需任何适配即可正常使用所有新写操作端点。
- **质量结果：** `cargo test --workspace` 全量通过，行为对比测试覆盖 7,624 个端点（Java 不可用时全部标记为 SKIP 并降级），无 PostgreSQL schema 命名不一致问题。
- **可维护性结果：** 新增端点实现时遵循统一的 ORM + 写操作模板，新开发者可参考 `oa4rust-orm-migration-guide.md` 快速上手，迁移状态文档与代码实际状态一致。

---

## Scope Boundaries

- **包含：** SeaORM 共享层构建；81 个 crate 的全量 ORM 迁移；PostgreSQL schema 规范化；8 个 core_entity crate 的写操作补齐；全量行为对比测试框架扩展；前端兼容性保障
- **排除在外：** Java 后端代码修改；前端 o2web 代码修改；文件存储层迁移（本地/NAS/对象存储）；性能压测与 Java 基准对比；Java 服务的永久下线；微服务拆分；定时任务/批处理框架的 Rust 迁移

### Deferred for later

- SQLx 完全移除（ORM 为默认路径，但复杂查询场景下可保留 SQLx 并存）
- 数据库连接池优化（Deadpool 参数调优、连接数策略）
- ORM 层支持多数据库后端（当前仅 PostgreSQL）
- 迁移状态文档的历史版本归档

### Outside this product's identity

- Java 代码的 Rust 重写（这是迁移的目标而非功能本身）
- 独立的 ORM 工具库发布（SeaORM 层为 oa4rust 内部共享 crate，不对外发布）
- 前端 o2web 的重写或现代化改造

---

## Key Decisions

- **SeaORM 作为 ORM 框架：** 现代、活跃开发、宏定义实体、支持动态查询，与 deadpool-postgres 兼容性好，适合本项目规模
- **分波次渐进迁移：** Wave 1（core_entity）→ Wave 2（assemble_control）→ Wave 3（其余），每波独立回归验证，风险可控；避免一次性全量替换导致的回归风险
- **ORM + SQLx 并存策略：** ORM 为默认路径，复杂查询/性能热点保留 SQLx；避免为追求一致而牺牲性能或可读性
- **全量 Schema 规范化迁移：** 同步重命名现有大写表名（FILE_FOLDER → file_folder 等）为小写，编写数据迁移脚本，所有 ORM 实体和查询同步更新
- **静态 mock 响应文件降级策略：** Java 不可用时，行为对比测试框架返回预设的 mock 响应文件，标记 SKIP 而非 FAIL，确保测试框架持续可用
- **全量行为对比测试覆盖：** 7,624 个端点全覆盖目标，含新增写操作端点

---

## Dependencies / Assumptions

- PostgreSQL 数据库已包含所有迁移文件中定义的表结构（`migrations/001-010`）
- 前端 o2web 的 `action.js` 对 `ActionResult<T>` 的 9 字段结构有隐式依赖，任何迁移不得改变此结构
- SeaORM 0.12+ 支持 deadpool-postgres 的 `Pool` 类型，无需自定义连接适配
- Java 服务可能已下线或不可用，行为对比测试需要兼容此场景
- 各 crate 的 `router(pool)` 入口函数签名不变，ORM 迁移对路由注册层透明

---

## Outstanding Questions

### Resolve Before Planning

（所有阻塞问题已解决）

- ~~Java 服务不可用时的降级策略~~ → **已决策：** 使用静态 mock 响应文件，Java 不可用时返回预设的 mock 数据
- ~~Schema 规范化的回退策略~~ → **已决策：** 全量规范化迁移，同步重命名表为小写，添加数据迁移脚本

### Deferred to Planning

- [Affects R1][Technical] SeaORM 实体的字段映射策略：是否使用 `#[sea_orm(table_name = "...")]` 保持现有大写表名，还是同步重命名表？
- [Affects R3][Needs research] ORM 层与现有 `ControlPool`/`ControlClient` trait 的集成方式——是否需要抽象统一的数据访问接口
- [Affects R7-R9][Needs research] 各 crate 的 SQLx 查询复杂度评估——哪些查询适合 ORM，哪些应保留原生 SQL
- [Affects R15][Technical] 7,624 个端点的行为对比测试枚举策略——是自动生成端点清单还是手动维护
