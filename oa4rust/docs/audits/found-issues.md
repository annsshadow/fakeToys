# Found Issues（plan002 执行期缺陷记录）

## FI-001: document_id_view_count 端点引用不存在的 view_count 列

- **发现时间**: 2026-08-22（plan002 执行期，编写集成测试 scenarios/data_integrity.rs 时发现）
- **状态**: 已修复

### 问题

业务端点 `document_id_view_count`（`crates/cms_assemble_control/src/lib.rs:5747`）执行：

```sql
UPDATE x_cms_document SET view_count = view_count + 1 WHERE id = $1 RETURNING view_count AS new_count
```

但 `x_cms_document` 表由 migration 034 创建，从未包含 `view_count` 列。运行时该端点必然 500。

### 根因

端点代码与迁移 schema 脱节：`x_cms_document` 的建表迁移（034）只含 id/xid/时间/创建人等基础列，后续迁移（058）仅补了 title/content 搜索列，无人补 `view_count`。库中虽存在等价计数表 `x_cms_document_view_count`（migration 024），但端点并未使用它。

### 修复

选择方案 (a)：新增幂等 migration，而非改端点查计数表。理由：

1. **零源码改动**——端点 SQL 保持原样，破坏面最小；
2. **有先例**——migration 058 即以 `ADD COLUMN IF NOT EXISTS` 给 `x_cms_document` 补列；
3. 方案 (b) 破坏更大：计数表 `view_count` 为 INTEGER(i32)，端点按 i64 读取；且 upsert 写法会给不存在的文档也插入计数行，要保留 NotFound 语义必须加存在性检查，需改业务代码。

改动文件：

- `migrations/060_add_view_count.sql` — `ALTER TABLE "x_cms_document" ADD COLUMN IF NOT EXISTS "view_count" BIGINT NOT NULL DEFAULT 0;`
- `migrations/060_add_view_count_rollback.sql` — 对应回滚（DROP COLUMN IF EXISTS）

### 验证

- `cargo check -p cms_assemble_control`：通过（仅存量 warning）
- `cargo test --lib -p cms_assemble_control`：335 passed, 0 failed

## FI-001 补记（2026-09-27）：060 正向迁移曾被删除，「已修复」实为假修复

b7bfca566（2026-08-27）以「avoid checksum conflict on fresh DB / 列已手工加到已有库」为由删除了
`060_add_view_count.sql`，但**没有任何迁移接替它给 x_cms_document 补 view_count**：

- fresh DB：060 永不应用 → 端点 500；
- 本库（live PG）：schema_migrations 从未记账 060，手工加的列在重建库时丢失 → 端点 500。

即 FI-001 自 2026-08-27 起处于「文档称已修复、运行时仍 500」状态约一个月。已按原文恢复 060
（幂等，未记账的库自动应用）。教训：**归档迁移必须同时给出接替者**，否则 fresh DB 与存量库都会漏列。

## FI-002: 全仓「handler SQL 引用不存在的列/表」系统性核对（2026-09-27）

- **状态**: 已修复

### 问题

以「代码 SQL 字面引用的 (table, column)」对撞 live PG `information_schema`，发现约 50 个 handler
运行时必然 500（undefined column/table），且全部被既有测试漏过（无 live DB 覆盖这些路径）：

1. **裸表名**：`portal_page`（SeaORM entity 声明 x_portal_page，手写 SQL 脱节）、`GEN_ARA_DISTRICT`；
2. **幽灵列**：`FILE_FILE.folder_id`（表从未有此列，033 加的是 superior）——4 个 handler 恒 500；
3. **错表**：meeting 三个 building/list/start/... handler 查楼宇表的时段列（表是纯楼宇维度）——
   codegen 把「按时段列会议」Action 前缀误挂 building；
4. **迁移缺列**：42 个 (table, column) 对（考勤周期/回执、IM 会话操作、组织、门户、控制台、
   程序中心、PP 家族过滤列等）——各 crate 的 u2 闭合批次加 handler 时未同步迁移。

### 修复

- 恢复 060 + 新增 `migrations/101_add_missing_handler_columns.sql`（TO_REGCLASS 判表 + 幂等 ADD COLUMN）；
- 表名/错表/幽灵列按各自语义改 SQL（详见提交 fix(sql)、feat(file)）。

### 方法沉淀

扫描脚本核心正则（WHERE/SET 列引用提取）+ information_schema 对撞可复用于后续新增 handler 的
自检；误报来源是动态拼接 SQL 片段与 AS 别名，人工复核后剔除。
