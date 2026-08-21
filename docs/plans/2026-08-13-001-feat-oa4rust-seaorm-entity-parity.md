---
title: OA4Rust SeaORM 实体↔表 对账闭环
type: feat
status: completed
date: 2026-08-13
origin: 延续 oa4rust block_on 反模式清除与 AI/BBS/CMS 实体对账工作
---

# OA4Rust SeaORM 实体↔表 对账闭环

## Summary

延续 oa4rust 对 O2OA（oa/o2server）的 Rust 重写工作。在清除 `tokio::runtime::Handle::current().block_on(shared::db::create_sea_orm_pool())` 反模式（此前已完成，但遗留 8 个路由被错误 transform 清空、AI/BBS 实体映射偏差）的基础上，本阶段系统性消除了 **SeaORM 实体 `table_name` 与已部署 PostgreSQL 表之间的全部对账缺口**，使所有已注册的 SeaORM `list` 端点在运行时不再 500。

## Problem Frame

oa4rust 使用 `*_core_entity` crate 定义 SeaORM 实体（`#[sea_orm(table_name = "...")]`），但大量实体的目标表从未被任何迁移创建，或列定义与已部署表不匹配。任一查询缺失表/缺失列的端点都会以 `internal server error`（HTTP 200 + `type:"error"`）静默失败。这类问题无法在编译期发现，只能运行时逐端点对账。

## Root Cause Classes（已修复）

1. **缺失表（占多数）**：实体引用的表从未迁移。例：`x_ai_app`/`x_ai_conversation`/`x_ai_model`、CMS 的 `x_cms_category`/`x_cms_article`、以及 33 个 `*_core_entity` 表（program_center / attendance / calendar / correlation / general / hotpic / jpush / mind / message / organization / portal / processplatform / query_express）。
2. **缺失列**：迁移 029 建表时漏列，或已部署表（迁移 022）与实体列集不一致。例：`x_script` 缺 `creator_person`/`deleted_at`；`x_org_group` 缺 `parent_id`/`level`；`x_org_identity` 缺 `person_id`/`type_`。
3. **SeaORM 字段名归一化**：实体字段 `type_` 被 SeaORM 映射为 SQL 列 `type`（剥离尾部下划线），而数据库列命名为 `type_`。修复方式：在实体字段加 `#[sea_orm(column_name = "type_")]`（cal_calendar / org_definition / org_identity），保留干净的 `type_` 列名、规避保留字 `type`。
4. **实体 `table_name` 与部署表名不符（BBS，前期已修）**：`bbs_forum_info`→`x_bbs_forum`、`bbs_section_info`→`x_bbs_section`、`bbs_subject_info`→`x_bbs_topic`。
5. **种子凭据漂移**：`testuser` 的 bcrypt 哈希在会话间漂移，导致登录 `invalid credentials`。已用标准 bcrypt 重新生成并写入 `auth_person.password_hash`。

## Key Technical Decisions

- **建表优先于改实体**：对实体引用的缺失表，采用与 AI/BBS 一致的策略——按实体 `Model` 列定义建表（migration 028/029），而非反过来改实体映射。这样 `*_core_entity` 自洽，不污染 `cms_assemble_control` 等使用不同表名（如 `x_cms_categoryinfo`）的独立模块。
- **迁移幂等**：所有正向迁移用 `CREATE TABLE IF NOT EXISTS`；列补齐用 `ALTER TABLE ... ADD COLUMN IF NOT EXISTS`（PG 9.6+）。回滚脚本一一对应。
- **迁移运行器读盘**：`shared/src/migrate.rs` 在启动时从文件系统 `migrations/` 按序应用正向 SQL，因此新增迁移只需重启服务即可生效，无需重新编译（仅代码改动才需 `cargo build`）。
- **诊断方法**：SeaORM 端点一律吞掉底层错误返回通用 `AppError::Internal`。定位时临时在 handler 的 `.map_err(|_| ...)` 注入 `tracing::error!`，以 `RUST_LOG=debug` 重启抓取真实 PG 错误码（如 `42703 column X does not exist`），确认后回退注入。

## Implementation Units / Migrations

- `migrations/026_create_ai_app_conversation.sql` — 建 `x_ai_app`/`x_ai_conversation`
- `migrations/027_create_ai_model.sql` — 建 `x_ai_model`
- `migrations/028_create_cms_core_tables.sql` — 建 `x_cms_category`/`x_cms_article`
- `migrations/029_create_missing_core_entity_tables.sql` — 一次性补齐 33 个缺失实体表
- `migrations/030_add_missing_columns_core_entity.sql` — 补齐 `x_script`/`x_org_group`/`x_org_identity` 缺失列
- `migrations/031_add_type_column_org_identity.sql` — 补齐 `x_org_identity.type_`
- 实体改动：`crates/calendar_core_entity/src/entities/cal_calendar.rs`、`crates/organization_core_entity/src/entities/org_definition.rs`、`org_identity.rs` 增加 `column_name = "type_"`
- `scripts/parity_sweep.sh` — 可复用的端到端点对账回归闸门（登录 + 41 个 list 端点断言 `type=="success"`，任一失败非零退出）

## Verification

`bash scripts/parity_sweep.sh`（携带缓存 TOKEN）结果：**0 failure(s) / 41 endpoint(s) — PARITY OK**。

覆盖基线（前期阶段）：AI app/model/conversation、BBS forum/section/subject、MEETING room、CMS category/article。
覆盖核心实体模块（32 端点）：program_center(5)、correlation、general(3)、hotpic、jpush(2)、mind(2)、message、organization(5)、portal(4)、processplatform(3)、query、attendance(2)、calendar(2)。

构建：`cargo build --offline --bin oa4rust` 通过（仅既有 2 个 dead_code/unused_import 警告，无错误）。

## Open Questions / Remaining Work

本次仅闭环 **`_core_entity` 类实体的 list 端点**对账。距「100% 替换 oa/o2server」仍余大量工作，按价值排序：

1. **扩大对账面**：`*_assemble_control` / `*_express` / `*_service_processing` 等模块的路由同样可能 500（`parity_sweep.sh` 的端点清单需持续扩展）。
2. **消除 201 处 `Value::Null` 静默空标记**：部分响应用 JSON null 占位而非真实数据/错误，需查清是未实现还是数据缺失。
3. **真实业务逻辑的 list 之外**：list 返回 200 但多为空数据；create/update/复杂聚合（工作流、CMS 文档、组织递归导航）仍需真实实现与种子数据。
4. **认证生态**：LDAP / 企业微信 / 钉钉 认证 provider 接入。
5. **多数据库适配、分布式/缓存/搜索**：架构层能力。
6. **灰度和切换**：从 Java o2server 到 oa4rust 的灰度切换方案与回滚。
7. **真实 DB 集成测试基座**：将 `parity_sweep.sh` 固化为集成测试，并入 CI。

---

## 实现情况（2026-08-21 审计）

**审计基准：** 工作树 HEAD 314c7a75；判定状态：completed

### 已验证完成

- migrations 026-031 全部实测在档（含 rollback 配对）
- `scripts/parity_sweep.sh` 在档
- 实体 column_name 修正（cal_calendar / org_definition / org_identity）随实体层落地

### 原 Remaining Work 的去向

- #4 认证生态（LDAP/企微/钉钉）：已由后续认证提供方计划全量落地
- #5 多数据库/分布式/搜索：MySQL 集成测试修复、Redis session 默认（7710d8af）、全文检索 migration 058 均已落地
- #7 parity 固化为集成测试：已由 `tests/behavior_comparison/` 行为契约对比套件升级承接
- #2 Value::Null 清理：201 处降至实测 15 处，残留待清理
- #1 扩大对账面 / #3 list 之外的真实业务逻辑 / #6 灰度切换：持续性问题，汇入剩余工作汇总计划统一追踪
