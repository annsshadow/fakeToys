# Phase 3 可行性评估：PostgreSQL 全文检索（Full-Text Search）

- 日期：2026-08-15
- 范围：`crates/cms_assemble_control`、`crates/query_assemble_designer`、`crates/query_assemble_surface`
- 结论：**全文检索可行**，已在 `cms_assemble_control` 中实现只读 `GET /jaxrs/cms_assemble_control/search?q=` 端点（含优雅降级 + 单测）。`cargo check -p cms_assemble_control` 通过。

---

## 1. 当前搜索状态（现状调研）

全局 grep `LIKE|ILIKE|to_tsvector|plainto_tsquery|ts_query|\bsearch\b` 结果：

| 位置 | 用法 | 说明 |
| --- | --- | --- |
| `crates/bbs/src/subject.rs:264` | `WHERE title ILIKE $1` | 真实搜索端点 `GET /jaxrs/bbs/subject/search?keyword=`，仅按 `title` 前缀/子串匹配 |
| `crates/file_assemble_control/src/lib.rs:967` | `FROM FILE_FILE WHERE name ILIKE $1` | 按文件名子串 |
| `crates/query_assemble_surface/src/lib.rs:631` | `x_query_design WHERE name ILIKE $1` | 按设计名子串 |
| `crates/query_assemble_surface/src/lib.rs:857,1036` | `x_query_table_data WHERE table_flag=$1 AND data ILIKE $2` | 在 JSON 文本列上做子串匹配 |
| `crates/query_assemble_designer/src/lib.rs:1367,1713` | 同上 `x_query_table_data ... ILIKE` | 同上 |
| `crates/correlation_service_processing/src/lib.rs:481,502` | `x_correlation WHERE "type" LIKE 'cms/%'` | 前缀分类匹配（非用户搜索） |
| `crates/base/src/lib.rs:68` | `pg_class WHERE relname LIKE 'cache_%'` | 内部元数据探测 |

**关键发现**：
- `crates/cms_assemble_control` 与 `crates/query_assemble_*` 中**均无 `to_tsvector`/`plainto_tsquery`**，全文检索尚未启用。
- 现有"搜索"全部是 `ILIKE '%kw%'` 子串匹配，缺点：无相关性排序、大表扫描慢（只能走顺序扫描或前缀 B-tree）、不支持分词/词干、中文按字匹配效果差、无排名（rank）。
- `x_cms_data_document`（`migrations/023_create_cms_assemble_tables.sql:211`）列：`id, app_id, category_id, title VARCHAR(255), content TEXT, author_id, status, publish_time, creator, create_time, deleted_at`。已为 `app_id`、`category_id` 建 B-tree 索引，但**无全文检索索引**。

---

## 2. 推荐方案 + 示例 SQL

### 2.1 主方案：`to_tsvector` / `plainto_tsquery` + GIN 索引

为 `x_cms_data_document` 增加生成列 + GIN 索引（**迁移 DDL**）：

```sql
-- 生成式 tsvector 列（title 权重 A，content 权重 B）
ALTER TABLE x_cms_data_document
  ADD COLUMN IF NOT EXISTS search_tsv tsvector
  GENERATED ALWAYS AS (
    setweight(to_tsvector('simple', COALESCE(title, '')), 'A') ||
    setweight(to_tsvector('simple', COALESCE(content, '')), 'B')
  ) STORED;

-- GIN 索引，检索走索引而非全表扫描
CREATE INDEX IF NOT EXISTS idx_x_cms_data_doc_tsv
  ON x_cms_data_document USING gin(search_tsv);
```

**查询（带相关性排名）**：

```sql
SELECT id, title, author_id, status,
       ts_rank(search_tsv, plainto_tsquery('simple', $1)) AS rank
FROM x_cms_data_document
WHERE deleted_at IS NULL
  AND search_tsv @@ plainto_tsquery('simple', $1)
ORDER BY rank DESC, COALESCE(publish_time, create_time) DESC NULLS LAST
LIMIT 50;
```

> 注：本次 Phase 3 仅实现**可行性端点**，未改动表结构。线上落地需先执行上述 DDL（`search_tsv` 列 + GIN 索引）。若暂不建索引，现有 `to_tsvector(...)` 表达式查询仍可正确执行，只是退化为顺序扫描。

### 2.2 备选方案：`pg_trgm` 三元组相似度（**强烈推荐用于中文/CJK 内容**）

PostgreSQL 内置 `pg_trgm` 按字符三元组切分，**对中文同样有效**，且支持 `%` / `similarity()` / `word_similarity()` 模糊匹配；配合 GIN/GiST 索引可加速子串与近似匹配。对 CJK 密集内容它比 `to_tsvector('simple')` 更实用（`simple` 配置不做中文分词，只能逐词精确匹配）。

```sql
CREATE EXTENSION IF NOT EXISTS pg_trgm;
CREATE INDEX IF NOT EXISTS idx_x_cms_data_doc_trgm
  ON x_cms_data_document USING gin (title gin_trgm_ops, content gin_trgm_ops);

SELECT id, title, author_id,
       similarity(title || ' ' || COALESCE(content, ''), $1) AS sim
FROM x_cms_data_document
WHERE deleted_at IS NULL
  AND (title % $1 OR content % $1)
ORDER BY sim DESC
LIMIT 50;
```

**建议**：最终落地可采用"混合"——`to_tsvector` 负责拉丁词/词干匹配与排名，`pg_trgm` 兜底 CJK 子串；或对于中文为主的 OA 场景直接以 `pg_trgm` 为主。

---

## 3. 本次新增的端点

**文件**：`crates/cms_assemble_control/src/lib.rs`、`crates/cms_assemble_control/src/routes.rs`

- `GET /jaxrs/cms_assemble_control/search?q=<term>`
  - 处理器 `search()`（lib.rs 内 `DocumentSearchQuery { q: Option<String> }`）。
  - 使用 `build_document_search_sql()` 构造只读查询：`to_tsvector('simple', ...)` 组合 `title`+`content`，`@@ plainto_tsquery('simple', $1)` 匹配，`LIMIT 50`，按 `publish_time/create_time` 倒序。
  - **优雅降级**：`q` 为空立即返回 `{"count":0,"data":[]}`；DB 不可达/查询失败时同样返回 `200 {"count":0,"data":[]}`（遵循本 crate LIST 处理器约定）。
  - **只读、无注入面**：查询串由固定模板生成，参数走 `$1` 占位符，未做字符串拼接。
  - 路由在 `routes.rs` 中注册，import `search`。
- 单元测试（`#[cfg(test)] mod search_tests`，无需实时 DB）：验证 SQL 含 `to_tsvector`/`plainto_tsquery`/`deleted_at IS NULL`/`@@` 且仅一个 `$1` 参数；验证 `document_search_result` 的返回结构。

> 注意：`utoipa` 并非本 crate 依赖，故新增端点未加 `#[utoipa::path]` 注解（避免引入未声明依赖）。如需纳入 OpenAPI 文档，可后续在 `Cargo.toml` 引入 `utoipa` 并补注解。

---

## 4. 风险与注意事项

1. **中文分词（最高优先级）**：`to_tsvector('simple', ...)` 不切分中文，仅能整词精确匹配，对 CJK 正文检索几乎无效。生产化应改用 `pg_trgm`（推荐）或安装 `zhparser`/`pg_jieba` 自定配置（如 `CREATE TEXT SEARCH CONFIGURATION jiebacfg ...`）。详见 2.2。
2. **索引/迁移缺失**：当前表无 `search_tsv` 列与 GIN 索引。未建索引时 `to_tsvector` 表达式查询退化为全表顺序扫描，`x_cms_data_document` 行数增长后会变慢。落地需补充迁移（2.1 DDL）。
3. **生成列 vs 表达式索引**：示例 2.1 用 `GENERATED ALWAYS AS ... STORED` 列 + GIN，最省心；也可改用表达式索引 `USING gin(to_tsvector('simple', ...))`，但混合权重（`setweight`）更适合生成列写法。
4. **排序与 NULL**：`COALESCE(publish_time, create_time) DESC NULLS LAST` 兼容两列为 NULL 的草稿；`simple` 配置无词干，意味着英文 "running" 不会匹配 "run"，如需词干可换 `english` 配置（但中文仍无效）。
5. **权限/多租户**：当前查询未过滤 `app_id`/可见性。若文档按应用隔离，应追加 `AND app_id = $2` 参数。本次可行性端点按"全库可见"实现，生产需结合鉴权。
6. **范围之外**：本次未改动 `query_assemble_*`（`x_query_table_data.data` 是 JSON 文本，全文检索更适合用 `jsonb_to_tsvector` 或先 `->>` 提取后建索引）。未触碰 `tests/integration_tests/` 与 `tests/integration_runner.rs`。
7. **测试覆盖**：仅做了 SQL 构造与返回结构的单测；端到端的真实检索需实时 PG（建议后续补 `#[ignore]` 集成测试，连测试库 seed 一行后断言命中）。

---

## 5. 验证

- `cargo check -p cms_assemble_control`：**通过（0 error，仅既有 warning）**。
- `cargo check --tests -p cms_assemble_control`：**通过**（测试模块可编译）。
- 未运行 `cargo test --workspace` / `cargo build --workspace` / `cargo test --test integration_runner`（按任务约束）。
