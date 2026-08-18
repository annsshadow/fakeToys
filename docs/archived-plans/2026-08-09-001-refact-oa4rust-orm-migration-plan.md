---
title: refact: OA4Rust 全量 SeaORM 迁移与写操作补齐
type: refactor
status: completed
date: 2026-08-09
origin: docs/brainstorms/2026-08-09-oa4rust-orm-migration-and-write-ops-requirements.md
---

# OA4Rust 全量 SeaORM 迁移与写操作补齐

## Summary

构建 `crates/orm` SeaORM 共享层，分三波（Wave 1: core_entity → Wave 2: assemble_control → Wave 3: 其余）将 81 个 crate 从 SQLx 原生查询迁移至 SeaORM，同步执行全量 schema 规范化（大写字母表名 → 小写 snake_case），为 8 个 core_entity crate 补全 POST/PUT/DELETE 写操作，并扩展行为对比测试框架覆盖全部 7,624 个端点（含静态 mock 降级）。

---

## Problem Frame

OA4Rust 已完成 81 个 crate 的真实化（7,624 个路由），但数据访问层存在三个结构性缺陷：8 个 core_entity crate 仅有 GET 查询无写操作；schema 命名风格不一致（FILE_FOLDER 大写 vs auth_person 小写混用）；行为对比测试仅覆盖 ~1% 端点。SQLx 原生查询缺乏编译期类型安全和实体关系表达，维护成本高。

---

## Requirements

- R1. 构建 `crates/orm` SeaORM 共享层（实体派生、CRUD 助手、软删除、分页）
- R2. ORM 层统一处理 ActionResult<T> 响应包装
- R3. ORM 层与现有 deadpool-postgres Pool 共存，通过 DatabaseConnection 包装
- R4. 规范化 PostgreSQL 表名/列名（UPPERCASE → snake_case），编写数据迁移脚本
- R5. 为新表/列添加外键约束和级联删除
- R6. 迁移文件幂等执行
- R7. Wave 1（核心实体层）：将 19 个 core_entity crate 从 SQLx 迁移至 SeaORM（organization_core_entity、file_core_entity、program_center_core_entity、ai_core_entity、cms_core_entity、bbs_core_entity、calendar_core_entity、meeting_core_entity、attendance_core_entity、message_core_entity、mind_core_entity、portal_core_entity、processplatform_core_entity、general_core_entity、hotpic_core_entity、jpush_core_entity 等）
- R10. 每波完成后全量回归测试
- R11. ORM 与 SQLx 并存（复杂查询保留 SQLx）
- R12. 为 8 个 core_entity crate 补全 POST/PUT/DELETE
- R13. 写操作包含参数验证和权限校验
- R14. 写操作响应与 Java 等效
- R15-R18. 行为对比测试扩展至 7,624 端点 + 静态 mock 降级
- R19-R21. 文档更新
- R22-R23. 前端兼容性保障

**Origin actors:** A1（开发者）, A2（Java 后端）, A3（前端 o2web）
**Origin flows:** F1（ORM 波次迁移）, F2（写操作补齐）, F3（行为对比测试扩展）
**Origin acceptance examples:** AE1-AE6

---

## Scope Boundaries

- **包含：** SeaORM 共享层构建；81 个 crate 全量 ORM 迁移；PostgreSQL schema 全量规范化；8 个 core_entity crate 写操作补齐；全量行为对比测试框架扩展；前端兼容性保障
- **排除在外：** Java 后端代码修改；前端 o2web 代码修改；文件存储层迁移；性能压测；Java 服务永久下线；微服务拆分；定时任务框架迁移

### Deferred to Follow-Up Work

- SQLx 完全移除（ORM 为默认路径，复杂查询可保留 SQLx 并存）
- 数据库连接池优化（Deadpool 参数调优）
- ORM 层支持多数据库后端
- 迁移状态文档的历史版本归档

---

## Context & Research

### Relevant Code and Patterns

- **现有数据访问模式：** `crates/*/src/*.rs` 中所有 handler 使用 `pool: Extension<Pool>` + `client.query()` + `serde_json::Value` 手动映射（参考 `crates/organization_core_entity/src/lib.rs`）
- **分页模式：** `crates/control/src/pagination.rs` 中的 `page_result(total, data, is_next)` 函数，返回带 `count`/`size`/`position` 字段的 ActionResult
- **软删除模式：** 所有查询添加 `AND deleted_at IS NULL`，删除操作执行 `UPDATE ... SET deleted_at = NOW()`
- ** ActionResult 包装：** `shared/src/response.rs` 中的 `ActionResult<T>` 9 字段结构，前端 action.js 强依赖
- **ControlClient trait：** `shared/src/lib.rs` 中的抽象层，用于测试注入 mock 数据
- **MockControlClient：** `shared/src/mock_client.rs` 中的测试 mock，返回预定义行数据
- **集成测试模式：** `tests/integration_tests/db.rs` 中的 `init_test_database()` + `tests/integration_tests/helpers.rs` 中的 `setup_test_server()`
- **行为对比测试：** `tests/behavior_compare.rs` 中手动维护的 ENDPOINTS 数组 + `tests/behavior_comparison/comparator.rs`

### Institutional Learnings

- **ActionResult<T> 9 字段结构不可变：** 任何 ORM 迁移不得改变响应 JSON 结构
- **SQLx 与 SeaORM 版本隔离：** SeaORM 2.0 内部依赖 sqlx 0.9，项目使用 sqlx 0.8，需通过 feature 隔离避免冲突
- **大写字段名陷阱：** PostgreSQL 未加引号的标识符自动转小写，ORM entity 必须用 `#[sea_orm(table_name = "...")]` 显式指定
- **person_flag_clause 动态 SQL：** `control/src/person.rs` 中的多字段 OR 匹配模式不适合 SeaORM，应保留为 SQLx 原生查询

### External References

- SeaORM 2.0 文档：https://docs.rs/sea-orm/2.0.1/sea_orm/
- SeaORM Axum 示例：https://github.com/SeaQL/sea-orm/tree/master/examples/axum_example
- SeaQuery（SeaORM 查询构建器）：https://docs.rs/sea-query/

---

## Key Technical Decisions

- **双池并存策略：** 引入 `DatabaseConnection`（SeaORM）与现有 `Pool`（deadpool-postgres）并存。迁移后的 crate 使用 `Extension<DatabaseConnection>`，未迁移 crate 继续使用 `Extension<Pool>`。main.rs 中同时注册两种 Extension。**退出标准：** Wave 3 完成后 30 天内审计所有保留 SQLx 的复杂查询，逐一评估是否可用 SeaORM 替代；超出期限强制统一迁移或明确标注保留理由
- **Schema 规范化策略：** 先创建 migration 011 执行全量表名重命名（FILE_FOLDER → file_folder 等），再开始 ORM 迁移。ORM entity 使用 snake_case 表名，与规范化后的 schema 一致
- **ActionResult 保持 Value 格式：** ORM 迁移初期保持 `ActionResult<Value>` 不变，通过 `#[serde(rename = "...")]` 在 entity 或转换函数中实现 camelCase，不引入 DTO 层
- **动态 SQL 保留 SQLx：** `person_flag_clause` 等动态拼接 SQL 的模式保留原生 SQLx，其余标准 CRUD 迁移到 SeaORM
- **SessionManager/RBAC 边界：** 认证路径（SessionManager、is_admin、person_has_role）继续使用原始 `Pool`，不纳入 ORM 迁移范围。此边界在 orm 层文档中明确标注
- **软删除过滤审查：** `list_active` 助手自动注入 `deleted_at IS NULL`；迁移前需审查所有 core_entity crate 的现有查询，标记缺少软删除过滤的端点作为已知差异
- **测试隔离：** 迁移过程中每个 crate 独立测试，`cargo test --workspace --lib` 全量回归验证无破坏性变更

---

## Open Questions

### Resolved During Planning

- **池管理策略：** 保留 deadpool-postgres 不替换——SeaORM 内置池功能等价但迁移成本更高，且现有代码已稳定运行。改为双池并存，逐步迁移
- **sqlx 版本冲突：** SeaORM 2.0 使用 sqlx 0.9，项目使用 sqlx 0.8。**前置验证（U1 开始前）：** 在独立分支上运行 `cargo build --workspace` 验证双版本共存编译通过；运行 `cargo tree -i sqlx` 确认依赖关系；运行全量测试验证无运行时冲突。验证失败则升级为升级 workspace sqlx 到 0.9
- **表名保留大写 vs 规范化：** 用户已决策全量规范化（大写→小写），创建 migration 011 执行 RENAME TABLE

### Deferred to Implementation

- SeaORM entity 的时间字段类型（`DateTime<Utc>` vs `naive_datetime`）—— 需在实际编写 entity 时根据迁移脚本中的列类型决定
- ORM 层 CRUD 助手的泛型签名细节（是否需要 `EntityTrait` 约束）—— 需在实现时根据实际使用场景确定
- 行为对比测试的 7,624 个端点自动生成策略—— 需在实现时评估脚本方案 vs 手动维护的性价比
- mock 响应文件的具体内容（需对照 Java 端点响应格式）—— 需在 Java 端点文档可用时填充

---

## Implementation Units

### U1. ORM 共享层基础构建

**Goal:** 在 `crates/orm` 中构建 SeaORM 共享层，提供实体基础、通用 CRUD 助手、软删除、分页工具，作为所有后续迁移的基础。

**Requirements:** R1, R2, R3, R6

**Dependencies:** None

**Files:**
- Create: `oa4rust/crates/orm/Cargo.toml`
- Create: `oa4rust/crates/orm/src/lib.rs`
- Create: `oa4rust/crates/orm/src/entity.rs`
- Create: `oa4rust/crates/orm/src/helpers.rs`
- Create: `oa4rust/crates/orm/src/pagination.rs`
- Create: `oa4rust/crates/orm/src/soft_delete.rs`
- Modify: `oa4rust/Cargo.toml`（添加 orm workspace member 和 sea-orm dependency）
- Modify: `oa4rust/crates/shared/Cargo.toml`（添加 orm 依赖，导出 `get_sea_orm_pool`）
- Modify: `oa4rust/crates/shared/src/db.rs`（添加 SeaORM DatabaseConnection 创建函数）
- Test: `oa4rust/crates/orm/src/tests.rs`

**Approach:**
- `entity.rs`：定义 `orm_entity!` 宏，封装 SeaORM 实体派生的常见模式（主键、索引、soft_delete、table_name 属性）
- `helpers.rs`：提供 `list_active`, `get_by_id`, `create`, `update`, `soft_delete` 通用 CRUD 助手，返回 `ActionResult<Value>`
- `pagination.rs`：提供 `cursor_list` 助手，替代现有的 `query_page` 模式，支持 next/prev 双向分页
- `soft_delete.rs`：提供 `ActiveModel` 派生，自动处理 `deleted_at` 字段
- `shared/src/db.rs`：新增 `create_sea_orm_pool()` 函数，与现有 `create_pool()` 并行
- `shared/src/lib.rs`：导出 `get_sea_orm_pool` 辅助函数
- **边界声明：** SessionManager 和 RBAC 中间件继续使用原始 `Pool`，不纳入 ORM 迁移范围（认证路径与数据访问层正交）

**Patterns to follow:**
- 现有 `shared/src/response.rs` 中的 `ActionResult<T>` 结构
- 现有 `control/src/pagination.rs` 中的 `page_result()` 模式

**Test scenarios:**
- Happy: `orm_entity!` 宏正确派生 SeaORM Entity，编译通过
- Happy: `list_active` 助手返回正确数量的记录，包含 `deleted_at IS NULL` 过滤
- Happy: `soft_delete` 助手执行 UPDATE SET deleted_at = NOW()
- Happy: `cursor_list` 助手返回正确分页结果，包含 count/size/position
- Edge: 空表查询返回空数组 + count=0
- Edge: 所有记录已软删除时 list_active 返回空
- Error: 数据库连接失败时返回 AppError::Internal

**Verification:**
- `cargo test -p orm --lib` 全部通过
- `orm` crate 成功编译，无 warning
- `shared` crate 成功 re-export orm 模块
- `cargo build --workspace` 验证 sqlx 0.8 + SeaORM（sqlx 0.9 传递依赖）双版本共存编译通过

---

### U2. Schema 规范化迁移脚本

**Goal:** 创建 migration 011，将数据库中所有大写字母表名和列名重命名为小写 snake_case，并确保幂等执行。

**Requirements:** R4, R5, R6

**Dependencies:** None

**Files:**
- Create: `oa4rust/migrations/011_normalize_schema.sql`
- Create: `oa4rust/migrations/011_normalize_schema_rollback.sql`
- Create: `oa4rust/scripts/audit_uppercase_tables.py`
- Modify: `oa4rust/migrations/008_file_tables.sql`（更新为大写转小写后的版本，作为参考）

**Approach:**
- 编写 Python 脚本 `scripts/audit_uppercase_tables.py`，扫描所有 migration 文件和大写表名引用，生成待重命名的表/列清单
- migration 011 使用 `DO $$ BEGIN ... END $$` 块实现幂等重命名（PostgreSQL `ALTER TABLE ... RENAME TO` 不支持 `IF EXISTS`）：
  ```sql
  DO $$ BEGIN
    IF EXISTS (SELECT 1 FROM pg_class WHERE relname = 'FILE_FOLDER') THEN
      ALTER TABLE "FILE_FOLDER" RENAME TO "file_folder";
    END IF;
  END $$;
  ```
- 索引重命名使用 `ALTER INDEX old_name RENAME TO new_name`（支持 IF EXISTS）
- 外键/约束重命名使用 `ALTER TABLE ... RENAME CONSTRAINT old_name TO new_name`
- 同时处理列名重命名（如需要）
- 为每个 RENAME 添加存在性检查确保幂等
- 回滚脚本记录所有重命名操作的逆向操作
- **生产迁移要求：** 迁移前完整数据库备份；明确维护窗口和回滚触发条件；迁移后健康检查（验证所有查询正常）
- **全量审计：** 脚本需扫描所有 crate 源码中的表名引用（不仅限于 migrations/ 目录），确保迁移 011 覆盖所有实际使用的表

**Technical design:**
```sql
-- migration 011 幂等重命名表（使用 DO 块，因 ALTER TABLE RENAME 不支持 IF EXISTS）
DO $$ BEGIN
  IF EXISTS (SELECT 1 FROM pg_class WHERE relname = 'FILE_FOLDER') THEN
    ALTER TABLE "FILE_FOLDER" RENAME TO "file_folder";
  END IF;
END $$;

DO $$ BEGIN
  IF EXISTS (SELECT 1 FROM pg_class WHERE relname = 'FILE_FILE') THEN
    ALTER TABLE "FILE_FILE" RENAME TO "file_file";
  END IF;
END $$;
-- ... 所有大写表名

-- 索引重命名（ALTER INDEX 支持 IF EXISTS）
ALTER INDEX IF EXISTS "idx_file_folder_superior" RENAME TO "idx_file_folder_superior";

-- 约束重命名
ALTER TABLE "file_file" RENAME CONSTRAINT "file_file_superior_fkey" TO "fk_file_file_folder";
-- ... 通过 pg_constraint 查询并重命名外键
```

**Patterns to follow:**
- 现有迁移文件的 `CREATE TABLE IF NOT EXISTS` 幂等模式

**Test scenarios:**
- Happy: migration 011 在干净数据库上执行成功，所有表名变为小写
- Happy: migration 011 在已有小写表名的数据库上幂等执行（IF EXISTS 跳过）
- Happy: 回滚脚本可逆执行（表名恢复为大写）
- Edge: 表中无数据时重命名不丢失数据
- Error: 外键约束重命名时正确处理依赖顺序

**Verification:**
- `SELECT tablename FROM pg_tables WHERE schemaname = 'public' ORDER BY tablename` 所有表名为小写
- `cargo test --workspace --lib` 通过（验证迁移后查询正常工作）
- 集成测试 `init_test_database()` 成功执行 migration 011

---

### U3. Wave 1：core_entity crate ORM 迁移

**Goal:** 将 19 个 core_entity crate 从 SQLx 迁移到 SeaORM，保持现有 GET 端点行为不变。

**Requirements:** R7, R10, R22, R23

**Dependencies:** U1, U2

**Files:**
- Modify: `oa4rust/crates/organization_core_entity/src/lib.rs`
- Modify: `oa4rust/crates/file_core_entity/src/lib.rs`
- Modify: `oa4rust/crates/program_center_core_entity/src/lib.rs`
- Modify: `oa4rust/crates/ai_core_entity/src/lib.rs`
- Modify: `oa4rust/crates/cms_core_entity/src/lib.rs`
- Modify: `oa4rust/crates/bbs_core_entity/src/lib.rs`
- Modify: `oa4rust/crates/calendar_core_entity/src/lib.rs`
- Modify: `oa4rust/crates/meeting_core_entity/src/lib.rs`
- Modify: `oa4rust/crates/attendance_core_entity/src/lib.rs`
- Modify: `oa4rust/crates/message_core_entity/src/lib.rs`
- Modify: `oa4rust/crates/mind_core_entity/src/lib.rs`
- Modify: `oa4rust/crates/portal_core_entity/src/lib.rs`
- Modify: `oa4rust/crates/processplatform_core_entity/src/lib.rs`
- Modify: `oa4rust/crates/general_core_entity/src/lib.rs`
- Modify: `oa4rust/crates/hotpic_core_entity/src/lib.rs`
- Modify: `oa4rust/crates/jpush_core_entity/src/lib.rs`
- Modify: `oa4rust/crates/correlation_core_entity/src/lib.rs`
- Modify: `oa4rust/crates/cms_express/src/lib.rs`
- Modify: `oa4rust/crates/query_express/src/lib.rs`
- Test: 各 crate 的 `src/tests.rs`

**Approach:**
- 为每个 crate 创建 `entities/` 子模块，使用 `orm_entity!` 宏定义 SeaORM 实体
- 表名使用小写 snake_case（已随 U2 规范化）
- 将所有 GET handler 从 `pool: Extension<Pool>` + `client.query()` 改为 `db: Extension<DatabaseConnection>` + SeaORM 查询
- 响应转换：使用 `#[serde(rename = "camelCase")]` 属性或转换函数映射到 ActionResult<Value>
- `person_flag_clause` 等动态 SQL 模式保留 SQLx 原生查询（通过 `Extension<Pool>` 获取）
- 保持 `router(pool)` 函数签名不变，内部创建 DatabaseConnection 并 layer Extension

**Technical design:**
```rust
// 示例：organization_core_entity 迁移模式
use orm::{get_db, list_active, to_json_array};

pub async fn person_list(
    db: Extension<DatabaseConnection>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let models = list_active::<person::Entity>(&db, 20)
        .await
        .map_err(|_| AppError::Internal)?;
    let data: Vec<Value> = to_json_array(&models, &[
        ("id", |m| m.id.clone()),
        ("name", |m| m.name.clone()),
        ("mobile", |m| m.mobile.as_ref().map(|s| s.clone()).unwrap_or_default()),
        ("email", |m| m.email.as_ref().map(|s| s.clone()).unwrap_or_default()),
    ]);
    Ok(Json(ActionResult::success(serde_json::json!({
        "count": data.len(),
        "data": data,
    }))))
}
```

**Patterns to follow:**
- 现有 `organization_core_entity/src/lib.rs` 中的 `definition_list` 等 handler
- 现有 `control/src/person.rs` 中的 `query_page` 模式（保留为 SQLx）

**Test scenarios:**
- Happy: 所有 8 个 crate 的 GET 端点返回与迁移前相同的响应格式
- Happy: `cargo test --workspace --lib` 全部通过
- Happy: 集成测试验证数据库查询返回真实数据
- Edge: 空表时返回空数组 + count=0
- Edge: 软删除记录不参与查询（deleted_at IS NULL 过滤生效）

**Verification:**
- `cargo test --workspace --lib` 全部通过
- 8 个 crate 的所有 GET 端点响应与迁移前完全一致
- `docs/brainstorms/oa4rust-migration-status-2026-08-08.md` 更新：8 个 core_entity crate 状态从"SQLx 原生"→"SeaORM"

---

### U4. Wave 1：core_entity crate 写操作补齐

**Goal:** 为 19 个 core_entity crate 补全 POST/PUT/DELETE 写操作，实现完整 CRUD。

**Requirements:** R12, R13, R14

**Dependencies:** U1, U2

**Files:**
- Modify: `oa4rust/crates/organization_core_entity/src/lib.rs`（添加 create/update/delete handler）
- Modify: `oa4rust/crates/file_core_entity/src/lib.rs`（添加 create/delete handler）
- Modify: `oa4rust/crates/program_center_core_entity/src/lib.rs`（添加 create/update/delete handler）
- Modify: `oa4rust/crates/ai_core_entity/src/lib.rs`（添加 create/update/delete handler）
- Modify: `oa4rust/crates/cms_core_entity/src/lib.rs`（添加 CRUD handler）
- Modify: `oa4rust/crates/bbs_core_entity/src/lib.rs`（添加 CRUD handler）
- Modify: `oa4rust/crates/calendar_core_entity/src/lib.rs`（添加 CRUD handler）
- Modify: `oa4rust/crates/meeting_core_entity/src/lib.rs`（添加 CRUD handler）
- Modify: `oa4rust/crates/attendance_core_entity/src/lib.rs`（添加 CRUD handler）
- Modify: `oa4rust/crates/message_core_entity/src/lib.rs`（添加 CRUD handler）
- Modify: `oa4rust/crates/mind_core_entity/src/lib.rs`（添加 CRUD handler）
- Modify: `oa4rust/crates/portal_core_entity/src/lib.rs`（添加 CRUD handler）
- Modify: `oa4rust/crates/processplatform_core_entity/src/lib.rs`（添加 CRUD handler）
- Modify: `oa4rust/crates/general_core_entity/src/lib.rs`（添加 CRUD handler）
- Modify: `oa4rust/crates/hotpic_core_entity/src/lib.rs`（添加 CRUD handler）
- Modify: `oa4rust/crates/jpush_core_entity/src/lib.rs`（添加 CRUD handler）
- Modify: `oa4rust/crates/correlation_core_entity/src/lib.rs`（添加 CRUD handler）
- Modify: `oa4rust/crates/cms_express/src/lib.rs`（添加 publish/unpublish handler）
- Modify: `oa4rust/crates/query_express/src/lib.rs`（添加 create/update handler）
- Test: 各 crate 的 `src/tests.rs`

**Approach:**
- 参照 `crates/control/src/person.rs` 中的 create/update/delete 模式实现写操作
- 所有写操作 handler 包含：
  1. 参数验证（非空检查、长度限制、格式校验）
  2. 权限校验（通过 Extension<Session> 获取当前用户）
  3. **PermissionLevel 映射：** 在 `PermissionRegistry` 中为每个新端点明确声明权限级别（敏感管理端点如 person/unit/role/group 使用 Admin，普通实体写操作使用 Authenticated）
  4. ORM 写入操作
  5. ActionResult 响应包装
- 使用 SeaORM 的 `ActiveModel` 进行写入，利用 soft_delete 模式处理删除
- 分页列表端点保持现有行为，新增写操作端点遵循相同响应格式
- 为每个写操作添加对应的集成测试

**Patterns to follow:**
- `crates/control/src/person.rs` 中的 create/update/delete 完整模式
- `crates/personal/src/password.rs` 中的输入验证模式
- 现有 `authorize_middleware` 的权限校验方式

**Test scenarios:**
- Happy: 创建人员 → 查询列表 → 验证新记录存在（Covers AE3）
- Happy: 更新人员 → 查询 → 验证字段已更新
- Happy: 软删除 → 查询 → 验证记录不可见（deleted_at IS NULL 过滤）
- Happy: 再次软删除已删除记录 → 返回 404
- Error: 缺少必填字段 → 返回 400 + type=error
- Error: 未认证用户调用写操作 → 返回 401
- Error: 无权限用户调用 → 返回 403
- Integration: 创建 → 查询 → 更新 → 软删除 → 验证最终状态（Covers AE3）

**Verification:**
- `cargo test --workspace --lib` 全部通过
- 所有新增写操作端点通过集成测试
- 新增端点纳入行为对比测试端点清单

---

### U5. Wave 2：assemble_control crate ORM 迁移

**Goal:** 将 15+ 个 assemble_control crate 从 SQLx 迁移到 SeaORM。

**Requirements:** R8, R10, R22, R23

**Dependencies:** U1, U2, U4

**Files:**
- Modify: `oa4rust/crates/organization_assemble_control/src/lib.rs`
- Modify: `oa4rust/crates/attendance_assemble_control/src/lib.rs`
- Modify: `oa4rust/crates/calendar_assemble_control/src/lib.rs`
- Modify: `oa4rust/crates/file_assemble_control/src/lib.rs`
- Modify: `oa4rust/crates/general_assemble_control/src/lib.rs`
- Modify: `oa4rust/crates/meeting_assemble_control/src/lib.rs`
- Modify: `oa4rust/crates/message_assemble_communicate/src/lib.rs`
- Modify: `oa4rust/crates/portal_assemble_designer/src/lib.rs`
- Modify: `oa4rust/crates/portal_assemble_surface/src/lib.rs`
- Modify: `oa4rust/crates/bbs_assemble_control/src/lib.rs`
- Modify: `oa4rust/crates/component_assemble_control/src/lib.rs`
- Modify: `oa4rust/crates/hotpic_assemble_control/src/lib.rs`
- Modify: `oa4rust/crates/jpush_assemble_control/src/lib.rs`
- Modify: `oa4rust/crates/mind_assemble_control/src/lib.rs`
- Modify: `oa4rust/crates/ai_assemble_control/src/lib.rs`
- Modify: `oa4rust/crates/cms_assemble_control/src/lib.rs`
- Test: 各 crate 的 `src/tests.rs`

**Approach:**
- 参照 U3 的迁移模式，逐个 crate 迁移
- assemble_control crate 通常包含复杂查询（多表 JOIN、聚合），优先迁移简单 CRUD 端点，复杂查询保留 SQLx
- 每个 crate 迁移后运行该 crate 的单元测试 + workspace 回归测试
- 对于使用 `ControlClient` trait 的 crate（如 `file_assemble_control`），适配 trait 以支持 SeaORM `DatabaseConnection`

**Patterns to follow:**
- U3 中已验证的迁移模式
- 现有 `file_assemble_control/src/lib.rs` 中的 ControlClient 模式

**Test scenarios:**
- Happy: 每个 crate 的现有 GET 端点返回与迁移前相同结果
- Happy: `cargo test --workspace --lib` 全部通过（验证无回归）
- Edge: 复杂查询端点（多表 JOIN）行为不变
- Integration: 端到端流程测试（如 attendance 的打卡流程）

**Verification:**
- `cargo test --workspace --lib` 全部通过
- `docs/brainstorms/oa4rust-migration-status-2026-08-08.md` 更新：15+ assemble_control crate 状态更新为"SeaORM"

---

### U6. Wave 3：其余 crate ORM 迁移

**Goal:** 将剩余所有 crate（express、portal、bbs、calendar、component、file、general、hotpic、jpush、meeting、mind、process_*、query_*、cms_* 等）从 SQLx 迁移到 SeaORM。

**Requirements:** R9, R10, R22, R23

**Dependencies:** U5

**Files:**
- Modify: 所有未迁移的 crate 的 `src/lib.rs` 和子模块
- Test: 各 crate 的 `src/tests.rs`

**Approach:**
- 按依赖关系排序迁移：先迁移无依赖的基础 crate（base、express），再迁移业务 crate
- 对于 `process_*`、`query_*` 等复杂 crate，识别其中的动态 SQL 模式并保留为 SQLx
- 每个子集迁移后独立回归测试
- 完成所有 crate 迁移后执行全量 `cargo test --workspace`

**Patterns to follow:**
- U3-U5 中已验证的迁移模式

**Test scenarios:**
- Happy: 所有 crate 的现有端点行为不变
- Happy: `cargo test --workspace` 全部通过
- Edge: 动态 SQL 端点（如 query_service 的 processing_execute）行为不变
- Integration: 全量集成测试通过

**Verification:**
- `cargo test --workspace` 全部通过
- `docs/brainstorms/oa4rust-migration-status-2026-08-08.md` 更新：全部 81 个 crate 状态为"SeaORM"
- `docs/brainstorms/oa4rust-endpoint-inventory.md` 更新：所有端点 ORM 状态标记

---

### U7. 行为对比测试全量扩展

**Goal:** 将行为对比测试从 ~75 个端点扩展到覆盖全部 7,624 个端点，并实现 Java 不可用时的静态 mock 降级。

**Requirements:** R15, R16, R17, R18

**Dependencies:** U3, U6（需全量 crate 迁移完成后才能覆盖全部 7,624 个端点）

**Files:**
- Modify: `oa4rust/tests/behavior_compare.rs`
- Modify: `oa4rust/tests/behavior_comparison/allowlist.yaml`
- Create: `oa4rust/tests/behavior_comparison/mock_responses/`（静态 mock 响应目录）
- Create: `oa4rust/scripts/extract_endpoints.py`
- Modify: `oa4rust/tests/behavior_comparison/comparator.rs`（添加 mock 降级逻辑）
- Modify: `oa4rust/tests/behavior_comparison/reporter.rs`

**Approach:**
- 编写 `scripts/extract_endpoints.py` 脚本，从 `main.rs` 和所有 crate 的 router 注册代码中自动提取所有端点路径和方法，生成端点清单（Rust 侧）
- 端点清单与 Java 端点映射基于 `oa4rust-migration-status-2026-08-08.md` 中的 Java 模块映射信息手动填充 `java_war`/`java_action` 字段
- 对于无法映射 Java 端点的 crate，标记为 SKIP 并在行为对比报告中说明原因
- 为每个端点生成基础 mock 响应（基于现有 allowlist.yaml 中的字段命名规则）
- 修改 `comparator.rs`：当 Java 服务不可达时，加载对应端点的 mock 响应文件作为 Java 侧响应，标记为 SKIP
- 扩展 `allowlist.yaml` 覆盖所有已知的 camelCase/snake_case 字段命名差异
- 保留手动编写的 75 个端点作为高质量基准，自动生成剩余端点

**Technical design:**
```python
# scripts/extract_endpoints.py 输出示例
# 格式：crate_name, method, rust_path, java_war, java_action
auth, POST, /jaxrs/authentication/login, x_organization_assemble_authentication, jaxrs/authentication/login
auth, GET, /jaxrs/authentication/whoami, x_organization_assemble_authentication, jaxrs/authentication/whoami
# ... 7624 entries
```

**Test scenarios:**
- Happy: 脚本提取所有 7,624 个端点（Covers AE6）
- Happy: Java 不可用时所有端点标记为 SKIP，测试通过
- Happy: Java 可用时端点对比正常 Pass/Fail
- Edge: 未知端点的 mock 响应自动生成
- Integration: 新增写操作端点自动纳入对比测试

**Verification:**
- `extract_endpoints.py` 输出 7,624 行端点清单
- `cargo test --test behavior_compare` 通过（Java 不可用时全部 SKIP）
- `allowlist.yaml` 覆盖所有已知字段命名差异
- `docs/brainstorms/oa4rust-endpoint-inventory.md` 更新：测试覆盖状态标记

---

### U8. 文档与维护

**Goal:** 更新所有维护文档，创建 ORM 迁移指南。

**Requirements:** R19, R20, R21

**Dependencies:** U6, U7

**Files:**
- Modify: `docs/brainstorms/oa4rust-migration-status-2026-08-08.md`
- Modify: `docs/brainstorms/oa4rust-endpoint-inventory.md`
- Create: `docs/brainstorms/oa4rust-orm-migration-guide.md`
- Modify: `oa4rust/README.md`（添加 ORM 使用说明）
- Modify: `oa4rust/.env.test.example`（添加 SEAORM_DATABASE_URL 说明，明确双池配置）

**Approach:**
- 更新迁移状态文档：所有 81 个 crate 标注 SeaORM 状态，标注已迁移时间
- 更新端点清单：标记新增写操作端点，更新测试覆盖状态
- 创建迁移指南：记录 `orm_entity!` 宏用法、CRUD 助手使用模式、常见陷阱（大写字段名、软删除过滤、分页适配）
- 更新 README：说明双池配置和 ORM 使用方式

**Test scenarios:**
- Test expectation: none — 纯文档工作

**Verification:**
- 迁移状态文档与代码实际状态一致
- 迁移指南包含完整的编码示例
- README 更新反映新的依赖和配置要求

---

## System-Wide Impact

- **Interaction graph:** ORM 层变更影响所有 81 个 crate 的数据访问；`shared/src/db.rs` 新增 SeaORM 池创建函数；`main.rs` 需注册 `DatabaseConnection` Extension
- **Error propagation:** `AppError` 枚举不变；SeaORM 的 `DbErr` 通过 `map_err` 转换为 `AppError::Internal`
- **State lifecycle risks:** Schema 规范化（U2）影响所有查询——需在迁移前确认所有引用已更新；双池并存期间需确保测试使用正确的池
- **API surface parity:** 所有已有端点的响应格式不变（ActionResult<T> 9 字段）；新增写操作端点遵循相同格式
- **Integration coverage:** ORM 迁移后需全量回归测试；行为对比测试覆盖全部端点
- **Unchanged invariants:** `ActionResult<T>` 9 字段结构不变；`/health` 端点保持公开；`AUTH_EXEMPT_PATHS` 不变；中间件层不变

---

## Risks & Dependencies

| Risk | Likelihood | Impact | Mitigation |
|------|-----------|--------|------------|
| Schema 规范化（U2）导致现有查询失效 | 高 | 高 | U2 先执行规范化并在测试数据库验证；全量审计所有表名引用；集成测试作为安全网 |
| 双池并存增加复杂性 | 中 | 中 | 明确边界：已迁移 crate 用 DatabaseConnection，未迁移用 Pool；SessionManager/RBAC 不迁移；Wave 3 后 30 天退出审计 |
| sqlx 双版本共存编译风险 | 中 | 中 | U1 前置 gate：先在独立分支验证 `cargo build` 和全量测试；失败则升级为升级 workspace sqlx 到 0.9 |
| 行为对比测试 7,624 端点 Java 映射不完整 | 高 | 中 | 使用 `extract_endpoints.py` 生成 Rust 端点清单；Java 映射基于 migration status 文档手动填充；无法映射的标记 SKIP |
| ORM 迁移中途回滚 | 中 | 高 | 每个 wave 开始前创建 git tag；回滚时从 git 恢复修改的源文件；schema 已变更（表名小写化）在回滚后仍保持 |
| 新写操作端点权限配置遗漏 | 中 | 高 | U4 明确要求在 PermissionRegistry 中为每个新端点声明权限级别（Admin vs Authenticated） |

---

## Documentation / Operational Notes

- 每波迁移完成后立即更新 `docs/brainstorms/oa4rust-migration-status-2026-08-08.md`
- Schema 规范化（U2）是高风险操作——需在测试数据库上先验证再在生产环境执行；迁移前完整备份；明确维护窗口和回滚触发条件
- 双池并存的过渡期需明确标注哪些 crate 使用哪种池
- 行为对比测试的 mock 响应文件需随端点变化持续更新

---

## Sources & References

- **Origin document:** [docs/brainstorms/2026-08-09-oa4rust-orm-migration-and-write-ops-requirements.md](../brainstorms/2026-08-09-oa4rust-orm-migration-and-write-ops-requirements.md)
- **Migration status:** [docs/brainstorms/oa4rust-migration-status-2026-08-08.md](../brainstorms/oa4rust-migration-status-2026-08-08.md)
- **Endpoint inventory:** [docs/brainstorms/oa4rust-endpoint-inventory.md](../brainstorms/oa4rust-endpoint-inventory.md)
- **Code review:** [docs/brainstorms/oa4rust-code-review-2026-08-08.md](../brainstorms/oa4rust-code-review-2026-08-08.md)
- Related code: `oa4rust/crates/shared/src/response.rs`, `oa4rust/crates/shared/src/db.rs`, `oa4rust/crates/control/src/person.rs`, `oa4rust/crates/control/src/pagination.rs`
- Related plans: [docs/plans/2026-08-07-001-feat-oa4rust-4wave-realization-plan.md](2026-08-07-001-feat-oa4rust-4wave-realization-plan.md)
- External docs: SeaORM 2.0, SeaQuery, Deadpool-postgres 0.12
