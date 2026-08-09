---
title: refact: SeaORM 迁移后续验收工作
type: refactor
status: active
date: 2026-08-09
origin: docs/plans/2026-08-09-001-refact-oa4rust-orm-migration-plan.md
---

# SeaORM 迁移后续验收工作

## Summary

为 SeaORM 全量迁移项目（`feat/seaorm-migration` 分支）补全后续验收工作：迁移脚本数据库验证、写操作端点集成测试、端到端功能等效性测试、行为对比测试端点自动生成。

---

## 背景

**已完成工作（`feat/seaorm-migration` 分支，7 个 commit）：**
- U1-U8 全部完成：81 个 crate 全部迁移至 SeaORM 1.0
- 创建 `crates/orm` 共享层、migration 011 schema 规范化脚本
- 19 个 core_entity crate 补全写操作（~76 个 POST/PUT/DELETE handler）
- 修复 17 个 crate 的嵌套 tokio runtime panic 问题
- `cargo test --workspace --lib`: 237 passed, 0 failed

**当前状态：**
- 代码编译通过，单元测试全部通过
- 行为对比测试覆盖 ~79 个端点（目标 7,624 个）
- migration 011 脚本已创建但未在测试数据库验证
- 写操作端点缺少集成测试

---

## 实施单元

### U9. Schema 规范化数据库验证

**Goal:** 在测试数据库上验证 migration 011 可幂等执行，表名全部变为小写 snake_case。

**Requirements:** R4, R6

**Dependencies:** None

**Files:**
- Create: `oa4rust/scripts/verify_migration_011.py`

**Approach:**
- 编写 Python 脚本连接到测试数据库
- 执行 migration 011（幂等，可在已小写化数据库上再次执行）
- 验证所有表名为小写 snake_case
- 验证索引和约束已正确重命名
- 回滚验证：执行 rollback 脚本后恢复大写表名
- 再次执行 migration 011 验证幂等性

**Verification:**
- 脚本执行成功，所有表名为小写
- 回滚后表名恢复大写
- 再次执行 migration 011 无报错

---

### U10. 写操作端点集成测试

**Goal:** 为所有新增的写操作端点添加集成测试，验证创建/更新/删除行为。

**Requirements:** R12, R13, R14

**Dependencies:** None

**Files:**
- Modify: 各 core_entity crate 的 `src/tests.rs`

**Approach:**
- 为每个写操作端点添加集成测试
- 测试覆盖：
  - Happy: 创建记录 → 查询验证
  - Happy: 更新记录 → 查询验证
  - Happy: 软删除 → 查询验证（deleted_at IS NULL 过滤生效）
  - Error: 缺少必填字段 → 400
  - Error: 更新不存在的记录 → 404
- 使用 `DatabaseConnection` mock 或直接连接测试数据库
- 遵循现有测试模式（`#[test]` + tokio runtime）

**Test scenarios:**
- Happy: organization 创建 person → 查询列表验证
- Happy: file 创建文件夹 → 查询列表验证
- Happy: cms 创建文章 → 查询验证
- Happy: 软删除 → deleted_at IS NULL 过滤生效
- Error: 缺少 name 字段 → BadRequest
- Error: 更新不存在 id → NotFound

**Verification:**
- 所有写操作端点有对应集成测试
- `cargo test --workspace --lib` 全部通过

---

### U11. 行为对比测试端点自动生成

**Goal:** 创建 `extract_endpoints.py` 脚本，从代码自动提取全部 7,624 个端点，生成行为对比测试端点清单。

**Requirements:** R15, R16, R17, R18

**Dependencies:** None

**Files:**
- Create: `oa4rust/scripts/extract_endpoints.py`
- Modify: `oa4rust/tests/behavior_compare.rs`（使用生成的端点清单）
- Modify: `oa4rust/tests/behavior_comparison/allowlist.yaml`（扩展字段映射）

**Approach:**
- 编写 Python 脚本扫描所有 crate 的 router 注册代码
- 提取端点信息：crate_name, method, path, java_war, java_action
- Java 映射基于 `oa4rust-migration-status-2026-08-08.md` 中的 Java 模块映射
- 输出 CSV/JSON 格式的端点清单
- 扩展 allowlist.yaml 覆盖更多字段命名差异

**Verification:**
- `extract_endpoints.py` 输出 7,000+ 行端点清单
- `cargo test --test behavior_compare` 通过（Java 不可用时全部 SKIP）

---

### U12. 端到端功能等效性测试

**Goal:** 验证 ORM 迁移后的功能与迁移前完全等效。

**Requirements:** R10, R22, R23

**Dependencies:** U9, U10

**Files:**
- Create: `oa4rust/tests/integration/` 目录
- Modify: 现有集成测试

**Approach:**
- 创建测试数据库，执行所有 migrations（包括 011）
- 运行全量集成测试，验证：
  - 所有 GET 端点返回正确数据
  - 所有写操作端点正确持久化
  - 软删除过滤生效
  - 分页功能正常
- 对比 Java 端点响应（如果 Java 服务可用）

**Verification:**
- 集成测试全部通过
- 功能等效性验证通过

---

## 验证标准

- `cargo check --workspace`: 通过
- `cargo test --workspace --lib`: 全部通过
- `cargo test --test behavior_compare`: 通过（Java 不可用时 SKIP）
- Schema 规范化验证脚本执行成功
- 写操作集成测试覆盖率 ≥ 90%

---

## 风险与依赖

| 风险 | 可能性 | 影响 | 缓解措施 |
|------|--------|------|----------|
| 测试数据库连接失败 | 中 | 低 | 脚本 graceful 处理连接失败，标记为 SKIP |
| 端点生成脚本遗漏部分端点 | 高 | 中 | 手动补充遗漏端点，对比 git diff 验证覆盖率 |
| Java 服务不可用导致对比测试全部 SKIP | 高 | 低 | SKIP 是预期行为，不影响测试通过 |
