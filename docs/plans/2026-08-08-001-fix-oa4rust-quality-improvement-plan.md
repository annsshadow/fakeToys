---
title: OA4Rust 质量改进与合并准入
type: fix
status: active
date: 2026-08-08
origin: docs/brainstorms/2026-08-08-oa4rust-quality-improvement-requirements.md
---

# OA4Rust 质量改进与合并准入

## Summary

修复代码审查发现的 67 个质量问题，其中 7 个 P0 阻断级问题全部修复作为合并准入硬条件，同时补齐测试覆盖、修复安全漏洞、消除虚假信心断言，确保 oa4rust 达到可合并、可上线标准。

---

## Problem Frame

O2OA Java 后端迁移到 Rust（oa4rust）已完成 81 个 crate 的真实化，但 2026-08-08 全量代码审查暴露了 67 个问题。当前状态是"功能完成但质量未达标"——带着这些缺陷合并将面临安全漏洞、部署失败、测试虚假信心等风险。

---

## Requirements

> 完整需求列表（R1-R60）见 origin: [docs/brainstorms/2026-08-08-oa4rust-quality-improvement-requirements.md](docs/brainstorms/2026-08-08-oa4rust-quality-improvement-requirements.md)

- R1. 修复 `query_service` ON CONFLICT 部分唯一索引缺失（`crates/query_service/src/lib.rs:116`）
- R2. 为 `chat_delete` 添加事务保护（`crates/ai/src/lib.rs:305`）—— **已修复**
- R3. 修复 AI 模型 API key 明文泄露（`crates/ai/src/lib.rs:116`）—— **已修复**
- R4. 修复 `create_file_entity`/`upload_file`/`create_file` 的 creator 字段伪造漏洞
- R5. 修复 `processing_execute` 的 SQL 存储风险（`crates/query_service/src/lib.rs:100`）
- R6. 修复迁移编号冲突（`migrations/` 目录）—— 部分已修复（文件已重命名为 009，但仍需清理 008 冲突）
- R7. 修复 `ADMIN_WRITE_PREFIXES` 静默扩展：回滚到 4 个核心前缀
- R8-R60. 其余 P1/P2/P3 问题（见 requirements doc 完整列表）

**Origin acceptance examples:** AE1-AE6（见 origin doc，AE = Acceptance Example，验收示例）

---

## Glossary

- **AE (Acceptance Example):** 验收示例，用于定义需求的行为边界。AE1-AE6 在 origin requirements doc 中定义。
- **R (Requirement):** 需求编号，R1-R60 对应 code review 发现的 60 个可追溯问题。
- **P0-P3:** 优先级层级，P0 阻断级（合并硬准入），P1 高优先级，P2 中优先级，P3 低优先级。

---

## Scope Boundaries

- **包含：** 代码审查 67 个发现的所有修复；测试覆盖补齐；API 契约一致性修复
- **排除在外：** Java 后端修改；前端 o2web 修改；新业务功能开发；架构重设计；数据库 schema 重新设计

### Deferred to Follow-Up Work

- R45-R60 中涉及大文件拆分（ai 500+ 行、program_center 3300+ 行、middleware 800+ 行）的工作
- 授权中间件缓存策略（Redis vs 内存 LRU）—— 需研究后决定
- 文件路径遍历攻击的实际风险评估
- API 版本前缀添加（R60）

---

## Context & Research

### Relevant Code and Patterns

- **测试模式：** `crates/*/src/tests.rs` 中使用 mock-based 测试（`MockControlClient`/`MockRow`）和集成测试（`tests/integration_tests/db.rs`）
- **中间件模式：** `crates/shared/src/middleware.rs` 包含 auth、authorize、rate_limit、CORS 中间件
- **权限模型：** `PermissionRegistry` + `requires_admin` + `check_permission` 三层权限控制
- **响应格式：** `ActionResult<T>` 9 字段 JSON 结构（前端 o2web action.js 强依赖）
- **迁移模式：** `migrations/NNN_*.sql` 文件，通过 `tests/integration_tests/db.rs` 的 `initialize_test_db` 应用

### Institutional Learnings

- **路由重复注册导致 axum panic** — 合并前需验证无重复注册
- **前端强依赖 ActionResult<T> 9 字段 JSON 结构** — 业务错误必须返回 HTTP 200 + type=error
- **认证绕过历史教训** — 所有写操作端点必须显式校验权限

---

## Key Technical Decisions

- **回滚 ADMIN_WRITE_PREFIXES 到 4 个：** 用户决策，避免破坏性变更影响集成方
- **迁移文件清理：** 将重复编号的 008/009 文件归档到 `migrations/archive/`，保留唯一编号
- **creator 字段安全：** 所有写操作端点从认证上下文获取 creator，而非请求体
- **测试修复策略：** 将所有 `INTERNAL_SERVER_ERROR` 断言替换为 mock-based 成功断言

---

## Open Questions

### Resolved During Planning

- **R6 迁移冲突：** 将 `008_cleanup_duplicates.sql` 和 `009_correlation_tables.sql` 归档到 `migrations/archive/`，保留 `008_file_tables.sql` 和 `009_person_group_tables.sql` 作为正式迁移文件
- **R2/R3 已修复状态：** 需在 U1 之前添加 U0 验收节点，确认已修复状态
- **67 vs 60 数量差异：** 67 = R1-R60（60 项需求）+ AE1-AE7（7 项验收示例），编号体系完整

### Deferred to Implementation

- 授权中间件缓存策略（Redis vs 内存 LRU，TTL 设置）—— R19/R20
- 大文件拆分边界（按 domain 还是按 concern）—— R45/R50/R51
- SQL 存储的详细风险评估：方案 A（仅存储审计）vs 方案 B（预定义模板）—— 已决策采用方案 B
- ADMIN_WRITE_PREFIXES 过渡期 deprecation 策略细节
- 路径遍历白名单目录列表定义
- DB 宕机权限降级缓存策略（TTL 30s，缓存失效后 fail-closed）—— R16/R17

---

## Implementation Units

### U1. 迁移文件冲突清理

**Goal:** 清理 migrations/ 目录中的重复编号冲突，确保部署时迁移按序执行不跳过。

**Requirements:** R6

**Dependencies:** None

**Files:**
- Modify: `migrations/008_cleanup_duplicates.sql` → 归档到 `migrations/archive/008_cleanup_duplicates.sql`
- Modify: `migrations/009_correlation_tables.sql` → 归档到 `migrations/archive/009_correlation_tables.sql`
- Create: `migrations/archive/README.md`（说明归档原因）

**Approach:**
- 将重复编号的迁移文件移动到 `migrations/archive/` 目录
- 保留 `008_file_tables.sql`（FILE_FOLDER/FILE_FILE/FILE_PERMISSION 表）
- 保留 `009_person_group_tables.sql`（auth_person_group 表）
- 在 archive/README.md 中说明归档原因：编号冲突，原始文件内容已由保留文件替代

**Patterns to follow:** 现有 `migrations/archive/` 目录模式

**Test scenarios:**
- Happy path: `ls migrations/` 无重复编号
- Edge case: 归档文件保留完整内容可追溯

**Verification:**
- `migrations/` 目录下无重复编号文件
- `cargo test --workspace --lib` 通过

---

### U2. ON CONFLICT 部分唯一索引修复

**Goal:** 为 `x_query_processing` 表添加 `model_flag` 的唯一约束，确保 `ON CONFLICT (model_flag)` 可正常工作。

**Requirements:** R1, AE1

**Dependencies:** U1

**Files:**
- Create: `migrations/010_query_processing_unique_index.sql`

**Approach:**
- 创建新迁移文件 `010_query_processing_unique_index.sql`
- 添加 `CREATE UNIQUE INDEX IF NOT EXISTS ... ON x_query_processing (model_flag) WHERE model_flag IS NOT NULL`
- 前置校验：迁移前先检查约束是否已存在（`SELECT 1 FROM pg_constraint WHERE conname = 'uq_query_processing_model_flag'`），避免生产环境重复创建失败
- 同时处理 `ON CONFLICT DO UPDATE` 返回 UUID 的语义问题（R18）

**Technical design:**
```sql
-- 部分唯一索引：仅当 model_flag 非空时才 enforce 唯一性
CREATE UNIQUE INDEX IF NOT EXISTS uq_query_processing_model_flag
    ON x_query_processing (model_flag)
    WHERE model_flag IS NOT NULL;
```

**Test scenarios:**
- Happy path: `ON CONFLICT (model_flag)` 成功更新而非插入重复
- Edge case: `model_flag` 为 NULL 时允许多行（部分索引特性）
- Error path: 无索引时 ON CONFLICT 报错（回归测试）

**Verification:**
- 迁移文件存在且语法正确
- 集成测试验证 ON CONFLICT 行为

---

### U3. Creator 字段伪造修复

**Goal:** 修复 `upload_file` 和 `create_file` 端点接受用户可控 creator 字段的安全漏洞。

**Requirements:** R4, AE2

**Dependencies:** None

**Files:**
- Modify: `crates/file_assemble_control/src/lib.rs`

**Approach:**
- `upload_file`（line 242）：将 `creator` 从请求体改为从认证上下文获取
- `create_file`（line 278）：同上
- `create_file_entity`（line 336）：已使用 `"system"`，保持不变（这是内部实体创建端点）
- 需要添加 `Extension(Session)` 参数获取当前用户

**Patterns to follow:** 参考 `delete_file_entity`（line 476-506）已有 session 参数模式

**Test scenarios:**
- Happy path: 上传文件时 creator 自动设置为当前登录用户
- Error path: 未认证用户调用返回 401
- Integration: 用户 A 上传文件，creator 字段为用户 A 的 ID，非请求体中的值

**Verification:**
- `upload_file` 和 `create_file` 的 handler 签名包含 `Extension(Session)`
- 测试验证 creator 字段来自 session 而非请求体

---

### U4. SQL 存储风险修复

**Goal:** 修复 `processing_execute` 将用户原始 SQL 存储在数据库中的风险。

**Requirements:** R5, AE1

**Dependencies:** None

**Files:**
- Modify: `crates/query_service/src/lib.rs`

**Approach:**
- 当前实现将用户提供的 `query` 字符串直接存入 `x_query_processing.query` 列
- 风险：存储的 SQL 可能被后续执行，存在注入风险
- **修复方案：** 改用预定义查询模板 + 参数绑定，不存储用户原始 SQL。若需审计，单独存储模板 ID 和执行参数（JSON 序列化）。
- 最小修复：在存储前验证 query 不包含危险模式（SELECT/INSERT/UPDATE/DELETE/DROP 等），或要求 query 必须是预定义的模板 ID

**Technical design:**
```rust
// 方案 A：验证 query 不包含危险 SQL 关键字
fn validate_query(query: &str) -> bool {
    let dangerous = ["SELECT", "INSERT", "UPDATE", "DELETE", "DROP", "ALTER", "CREATE"];
    !dangerous.iter().any(|d| query.to_uppercase().contains(d))
}

// 方案 B：仅允许模板 ID，不存储原始 SQL
// 将 query 参数改为 template_id，从模板表获取实际 SQL
```

**Test scenarios:**
- Happy path: 合法 query 正常存储
- Error path: 包含危险关键字的 query 返回 400
- Edge case: 空 query 返回 400（已有）

**Verification:**
- 危险 SQL 模式被拒绝
- 合法查询正常执行

---

### U5. ADMIN_WRITE_PREFIXES 回滚

**Goal:** 将 `ADMIN_WRITE_PREFIXES` 从 15 个前缀回滚到 4 个核心前缀，消除破坏性变更。

**Requirements:** R7, AE4

**Dependencies:** None

**Files:**
- Modify: `crates/shared/src/middleware.rs`

**Approach:**
- 将 `ADMIN_WRITE_PREFIXES` 从 15 个前缀回滚到 4 个：`/jaxrs/person`、`/jaxrs/unit`、`/jaxrs/role`、`/jaxrs/group`
- 更新顶部的迁移注释，记录回滚决策
- **同步更新** `PermissionRegistry::with_defaults()` 中新模块的权限级别（从 Admin 改为 Authenticated）
- 原子性要求：`ADMIN_WRITE_PREFIXES` 常量与 `PermissionRegistry` 注册必须在一个 PR 内同时提交，避免部署窗口期出现权限不一致
- **过渡期策略**：保留 15 个前缀的完整列表作为注释存档（`ADMIN_WRITE_DEPRECATED_PREFIXES`），供集成方参考；提供 30 天 deprecation 警告期

**Patterns to follow:** 现有 `ADMIN_WRITE_PREFIXES` 常量定义模式

**Test scenarios:**
- Happy path: 非 admin 用户可写 /jaxrs/ai、/jaxrs/file 等模块
- Error path: 非 admin 用户写 /jaxrs/person 仍返回 403
- Integration: 前端 o2web 写操作恢复正常

**Verification:**
- `ADMIN_WRITE_PREFIXES` 只包含 4 个核心前缀
- `cargo test --workspace --lib` 通过
- 集成测试验证权限降级

---

### U6. 测试虚假信心修复

**Goal:** 修复所有断言 `INTERNAL_SERVER_ERROR` 的路由存在性测试，替换为 mock-based 成功断言。

**Requirements:** R14, R15, AE3

**Files:**
- Modify: `crates/ai/src/tests.rs`
- Modify: `crates/file_assemble_control/src/tests.rs`
- Modify: `crates/query_service/src/tests.rs`
- Modify: 其他 6+ 个 crate 的 tests.rs

**Approach:**
- 识别所有 `assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR)` 断言
- 替换为 mock-based 测试，使用 `MockControlClient` 返回预期数据
- 对于需要真实 DB 的测试，使用 `tests/integration_tests/db.rs` 的 `TestContext`
- **保留 20% 测试作为 live-DB 集成测试**（标记 `#[cfg(feature = "integration")]`），确保 mock 不掩盖真实 DB 行为差异

**Patterns to follow:** `crates/file_assemble_control/src/tests.rs` 中的 mock 模式

**Test scenarios:**
- Happy path: 所有路由存在性测试返回 200 + type=success
- Edge case: mock 返回空数据时端点正常响应

**Verification:**
- `cargo test --workspace --lib` 全部通过
- 无 `INTERNAL_SERVER_ERROR` 断言残留

---

### U7. P1 安全与正确性修复

**Goal:** 修复 P1 级别的安全漏洞和正确性问题。

**Requirements:** R8-R24（除已修复的 R2、R3）

**Dependencies:** U3, U5

**Files:**
- Modify: `crates/file_assemble_control/src/lib.rs`（R8、R27、R28）
- Modify: `crates/ai/src/lib.rs`（R9、R11、R22）
- Modify: `crates/program_center/src/lib.rs`（R10、R21）
- Modify: `crates/shared/src/middleware.rs`（R12、R13、R16、R17、R19、R20、R24）
- Modify: `crates/query_service/src/lib.rs`（R18）

**Approach:**
- R8: 修复 `update_file_entity` 的 COALESCE 逻辑
- R9: 修复分页 offset 计算的 i32→i64 溢出
- R10: 恢复 `modules_all` 的 entityCount 原语义
- R11: 为 `chat_delete`/`file_delete` 添加所有权检查（chat_delete 已有，file_delete 需添加）
- R12: 实现 `PermissionLevel::Owner` 分支
- R13: 为 `person_has_group` 添加软删/禁用过滤
- R16/R17: 修复 DB 宕机和模块未注册时的权限降级 — 添加本地缓存权限快照（TTL 30s），DB 不可用时使用缓存而非直接拒绝或放行；缓存失效后 fail-closed 并触发告警
- R18: 修复 `ON CONFLICT DO UPDATE` 返回新生成 UUID 的语义错误
- R19/R20: 优化授权中间件性能（添加请求级 is_admin 缓存）
- R21: 统一 `ApplicationCreateRequest` 字段命名
- R22: 修复 `list_enable_model` 响应语义
- R24: 修复 403 响应提供缺失角色信息

**Test scenarios:**
- Happy path: 所有权检查正确拒绝跨用户删除
- Error path: DB 宕机时授权检查 fail-open 而非 fail-closed
- Edge case: 分页参数超出范围时正确拒绝

**Verification:**
- 所有 P1 问题修复并通过测试
- `cargo test --workspace --lib` 通过

---

### U8. P2 中优先级修复

**Goal:** 修复 P2 级别的安全、性能和测试覆盖问题。

**Requirements:** R25-R44

**Dependencies:** U7

**Files:**
- Modify: 多个 crate 的 src/lib.rs 和 src/tests.rs
- Modify: `migrations/009_person_group_tables.sql`（R43、R44）

**Approach:**
- R25-R32: 修复类型转换、stub 端点、删除策略一致性、路径遍历等（路径遍历使用白名单校验：解析用户路径经 `std::path::Path::canonicalize` 后校验是否在允许的根目录内）
- R33-R36: 补充测试覆盖（新增 POST 端点、空行分支、错误路径、中间件单元测试）
- R37-R41: 添加 LIMIT、移除冗余代码、优化查询
- R42: 添加 config_save key 白名单验证（精确匹配，非前缀匹配）
- R43-R44: 为 migration 009 添加外键约束和级联删除
- R16/R17: DB 宕机权限降级策略 — 添加本地缓存权限快照（TTL 30s），DB 不可用时使用缓存而非直接拒绝或放行

**Test scenarios:**
- Happy path: 所有新增测试通过
- Error path: 路径遍历被拒绝、大分页被拒绝
- Integration: 外键约束防止孤立记录

**Verification:**
- 测试覆盖率提升至 60%+
- `cargo test --workspace` 全部通过

---

### U9. P3 低优先级改进

**Goal:** 修复 P3 级别的可维护性和代码质量问题。

**Requirements:** R45-R60（排除 Deferred to Follow-Up Work 中的大文件拆分 R45/R50/R51，这些不在 U9 范围内）

**Dependencies:** U8

**Files:**
- Modify: 多个 crate 的 src/lib.rs 和 src/tests.rs
- Modify: `crates/shared/src/middleware.rs`

**Approach:**
- R46: 提取分页响应包装器为 helper
- R47: 简化 file_assemble_control 的三层 trait 抽象
- R48: 消除未使用的导入和变量
- R49: 统一 ADMIN_WRITE_PREFIXES 与 PermissionRegistry 的数据源
- R52: 接入 behavior_comparison_middleware
- R53: 修复 config_list_mcp_paging 返回空桩数据
- R54: 修复 index_delete 不执行删除的问题
- R55: 修复 file_download 返回 JSON 而非文件流
- R56: 统一列表端点响应形状
- R57: 为 migration 009 添加回滚脚本
- R58: 移除冗余索引
- R59: 修复前缀匹配过宽问题

**Test scenarios:**
- Happy path: 所有 P3 修复通过测试
- Integration: 行为对比中间件正确记录测试数据

**Verification:**
- `cargo test --workspace` 全部通过
- 代码质量指标改善

---

## System-Wide Impact

- **Interaction graph:** 授权中间件修改影响所有端点的权限检查；迁移文件修改影响数据库 schema
- **Error propagation:** ActionResult 错误响应格式保持不变（前端强依赖）
- **State lifecycle risks:** 迁移文件归档不影响现有数据库；creator 字段修复影响已有文件的 creator 值（只影响新创建的文件）
- **API surface parity:** ADMIN_WRITE_PREFIXES 回滚恢复非 admin 用户的写权限，需通知集成方
- **Integration coverage:** 需验证所有 7624 个路由的测试覆盖
- **实现单元并行度：** U1-U6 可并行执行；U7 依赖 U3+U5；U8 依赖 U7；U9 依赖 U8

---

## Risks & Dependencies

| Risk | Mitigation |
|------|------------|
| ADMIN_WRITE_PREFIXES 回滚可能影响已依赖新权限的集成方 | 提前通知集成方，提供迁移时间窗口；保留废弃前缀列表供参考 |
| 迁移文件归档可能导致旧环境迁移跳过 | 归档文件保留完整内容，新环境不受影响 |
| 测试修复可能暴露潜在问题 | 分阶段修复，每阶段验证 `cargo test`；保留 20% live-DB 集成测试 |
| SQL 存储风险修复可能影响现有功能 | 与业务方确认 query 字段的使用场景 |
| 全量计划缺少回滚策略 | 每个 P0 单元需附独立回滚步骤；任一 P0 失败则中止整个部署 |
| 线性依赖链 U3→U7→U8→U9 | U3/U4/U5/U6 可并行；U7 依赖 U3+U5；U8 依赖 U7；U9 依赖 U8 |

---

## Documentation / Operational Notes

- 更新 `docs/brainstorms/oa4rust-migration-status-2026-08-08.md` 为"合并就绪"状态
- 更新 `docs/brainstorms/oa4rust-remaining-work-2026-08-08.md` 记录本次修复
- ADMIN_WRITE_PREFIXES 回滚需在 PR 描述中明确说明
- **回滚策略：** 每个 P0 单元独立可回滚；若任一 P0 失败则中止整个部署，不回滚部分单元
- **历史数据策略：** creator 字段修复后，历史记录的 creator 值保持不变，仅新创建的文件使用 Session 中的用户

---

## Rollback Strategy

每个 P0 单元失败时，按以下策略回滚：

| 单元 | 回滚操作 |
|------|----------|
| U1（迁移归档） | 将归档文件移回 `migrations/`，无需数据回滚 |
| U2（索引创建） | 若迁移失败，删除 `010_*.sql` 并重新运行；若索引已存在，`IF NOT EXISTS` 已防护 |
| U3（creator 修复） | 代码回滚；历史数据不受影响（只影响新创建文件） |
| U4（SQL 存储） | 代码回滚；需业务方确认 query 字段使用场景后再决定 |
| U5（ADMIN_WRITE 回滚） | 代码回滚；需通知集成方权限已恢复 |
| U6（测试修复） | 代码回滚；无数据影响 |
| U7-U9 | 依赖上游单元，上游回滚则整体回滚 |

**硬停止条件：** 任一 P0 单元测试失败或部署报错，中止整个部署，不回滚部分单元。

---

## Sources & References

- **Origin document:** [docs/brainstorms/2026-08-08-oa4rust-quality-improvement-requirements.md](docs/brainstorms/2026-08-08-oa4rust-quality-improvement-requirements.md)
- **Code review:** [docs/brainstorms/oa4rust-code-review-2026-08-08.md](docs/brainstorms/oa4rust-code-review-2026-08-08.md)
- **Migration status:** [docs/brainstorms/oa4rust-migration-status-2026-08-08.md](docs/brainstorms/oa4rust-migration-status-2026-08-08.md)
- Related code: `oa4rust/crates/shared/src/middleware.rs`, `oa4rust/crates/ai/src/lib.rs`, `oa4rust/crates/file_assemble_control/src/lib.rs`, `oa4rust/crates/query_service/src/lib.rs`
