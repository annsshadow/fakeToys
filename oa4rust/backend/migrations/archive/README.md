# Migration Archive

This directory contains migration files that were archived due to numbering conflicts.

## Archived Files

### 008_cleanup_duplicates.sql
- **Reason:** Numbering conflict with 008_file_tables.sql
- **Content preserved:** Documents that 003 and 004 were duplicates moved to archive
- **Replacement:** N/A (this was a no-op migration)

### 009_correlation_tables.sql
- **Reason:** Numbering conflict with 009_person_group_tables.sql
- **Content preserved:** Creates x_correlation and CORR_C_CORRELATION tables
- **Replacement:** 009_person_group_tables.sql handles auth_person_group; correlation tables may need re-numbering

### 060_add_view_count.sql.disabled（2026-10-2x 冷藏，`27335205f`）
- **Reason:** Causes checksum conflict on fresh DB
- **Content preserved:** Adds a view_count column (view/table 形态见原文件)
- **Replacement:** N/A —— 如需该列，按 101/112 的幂等范式（TO_REGCLASS 守卫 +
  ADD COLUMN IF NOT EXISTS + 出处行号注释）新建编号迁移；勿重启本文件

### 089_fix_500_endpoints.sql.disabled（同批冷藏，`27335205f`）
- **Reason:** Creates views over nonexistent uppercase tables —— 视图定义与当时的
  schema 现实不符，应用即错
- **Content preserved:** PP_E_APPLICATION 等大写视图映射 + 一批缺列 ALTER
- **当年处置:** commit 自述「Columns applied manually via SQL instead」——缺列被
  **手工**应用到各开发库，视图没有。后果：dev/live 库能跑，fresh DB 上引用这些
  视图的 handler 必 500（2026-10-26 第三轮 F-3 实证：surface:887/975 两个端点）
- **现状（修正版已落地）:** 其 PP_E_APPLICATION 视图定义的修正版在
  `migrations/112_fix_fresh_db_500_endpoints.sql`（幂等、pg_class 守卫不存在才建、
  定义与 032 DDL 逐列核对）。**⚠️ 不要重启用本文件**——要动视图定义，改 112 或
  新建编号迁移；直接重启 089 会把「views over nonexistent tables」的原始缺陷带
  回来。视图存在性由 `tests/quoted_relation_guard.rs` 把守（引号大写关系必须在
  生效迁移里有 CREATE）

## Current Migration Order

迁移运行器只应用本目录**之上** `migrations/*.sql` 顶层（排除 `archive/` 与
`*_rollback.sql`，见 `shared/src/migrate.rs:84`）。现行最新编号：112
（111 = FI-002 第二轮补列；112 = fresh-DB 500 收口，2026-10-26）。

## Notes

- Archive files are kept for historical reference and rollback purposes
- New migrations should use the next available number (currently 113+)
- When applying migrations to new environments, only use files in the root migrations/ directory
