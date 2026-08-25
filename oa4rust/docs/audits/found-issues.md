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
