# SQL Dialect Abstraction Layer (Phase 3 U3.1)

**Date:** 2026-08-18  
**Module:** `shared::db::dialect`  
**Status:** implemented — 5 handlers updated as examples

## 1. Problem

The project uses raw PostgreSQL SQL throughout (~75 files). Dialect-specific features
make the codebase non-portable:

| Feature | PostgreSQL | MySQL |
|---|---|---|
| Parameter placeholder | `$1`, `$2` | `?` |
| Type cast | `expr::text`, `expr::bigint` | `CAST(expr AS CHAR)`, `CAST(expr AS BIGINT)` |
| JSON column type | `jsonb` | `json` |
| Identifier quote | `"name"` | `` `name` `` |
| Case-insensitive LIKE | `ILIKE` | `LIKE` (default collation) |
| Current timestamp | `NOW()` | `NOW()` (identical) |

## 2. Design

### 2.1 `SqlDialect` trait (`crates/shared/src/db/dialect.rs`)

Minimal trait covering only the dialect differences actually used by current queries:

```rust
pub trait SqlDialect: Send + Sync {
    fn name(&self) -> &'static str;
    fn quote_ident(&self, name: &str) -> String;
    fn param(&self, n: usize) -> String;
    fn now(&self) -> &'static str;
    fn json_type(&self) -> &'static str;
    fn cast_text(&self, expr: &str) -> String;
    fn cast_bigint(&self, expr: &str) -> String;
    fn ilike_op(&self) -> &'static str;
    fn format_sql(&self, sql: &str) -> String;
    fn cast_text_param(&self, n: usize) -> String { ... }
    fn cast_bigint_param(&self, n: usize) -> String { ... }
}
```

Default `cast_*_param` implementations combine `param()` + `cast_*()`.

### 2.2 Two implementations

- **`PostgresDialect`** — identity for `format_sql()`; `$N` unchanged; `::text` / `::bigint` unchanged.
- **`MySQLDialect`** — `format_sql()` replaces `$N` with `?` (hand-written scanner, no `regex` dep); casts use `CAST(...AS CHAR/BIGINT)`; identifier quote is `` ` ``.

### 2.3 Global dialect accessor

```rust
pub fn dialect() -> &'static dyn SqlDialect
```

Uses `std::sync::OnceLock` to read `DATABASE_DIALECT` env var once:

- `DATABASE_DIALECT=postgres` (default) → `PostgresDialect`
- `DATABASE_DIALECT=mysql` → `MySQLDialect`

No change to any pool/connection code. Handlers call `dialect()` to get the current dialect and build dialect-aware SQL.

### 2.4 `format_sql()` — safe `$N` → `?` replacement

Hand-written character scanner. Only replaces `$` followed by ASCII digits. Does **not** touch `$` inside string literals (e.g., `'price $100'` is preserved).

### 2.5 File layout

```
crates/shared/src/db/
  dialect.rs    ← SqlDialect trait + PostgresDialect + MySQLDialect + dialect()
  rs            ← existing pool code; re-exports dialect items
```

`db.rs` exports: `pub use dialect::{dialect, MySQLDialect, PostgresDialect, SqlDialect};`  
Handlers import via: `use shared::db::dialect;`

## 3. Handlers Updated (example set)

| Crate | Handler | SQL pattern demonstrated |
|---|---|---|
| `file_assemble_control` | `get_control_config` | `$N` params via `format_sql()` |
| `file_assemble_control` | `list_storage_pools` | no-param query via `format_sql()` |
| `file_assemble_control` | `update_control_config` | `$1, $2, $3` via `format_sql()` |
| `file_assemble_control` | `list_control_categories` | no-param query via `format_sql()` |
| `auth` | `login` | `::text` casts + `$1` — uses `cast_text()` + `param()` |
| `auth` | `whoami` | `$1` via `format_sql()` |
| `ai_assemble_control` | `config_list_mcp_paging` | `::bigint` casts + `$1, $2` — uses `cast_bigint_param()` |
| `ai_assemble_control` | `config_list_model_paging` | same `::bigint` pattern |
| `ai_assemble_control` | `list_ai_models` | `$N` via `format_sql()` |
| `attendance_assemble_control` | `toggle_control_rule` | `$1, $2` via `format_sql()` |
| `attendance_assemble_control` | `attendanceadmin_id` | `$1` via `format_sql()` |
| `attendance_assemble_control` | `attendanceappealInfo_filter_list_id_next_count` | `::bigint` cast via `cast_bigint_param()` |
| `attendance_assemble_control` | `attendanceappealInfo_filter_list_id_prev_count` | same `::bigint` pattern |

**Total: 4 crates, 7+ distinct handler functions touched.**

## 4. Usage patterns in handlers

### Pattern A: Simple `$N` queries (no casts)

```rust
use shared::db::dialect;

let rows = client
    .query(dialect().format_sql("SELECT id FROM t WHERE id = $1"), &[&id])
    .await?;
```

`format_sql()` returns a `String`; pass `&sql` to the query method.

### Pattern B: Casts (`::text`, `::bigint`)

```rust
let d = dialect();
let sql = format!(
    "SELECT {}, {} FROM auth_person WHERE unique_id = {}",
    d.cast_text("change_password_time"),
    d.cast_text("password_expired_time"),
    d.param(1),
);
let row = client.query_one(&sql, &[&credential]).await?;
```

### Pattern C: Pagination with `LIMIT ...::bigint OFFSET ...::bigint`

```rust
let d = dialect();
let sql = format!(
    "SELECT ... LIMIT {} OFFSET {}",
    d.cast_bigint_param(2),
    d.cast_bigint_param(1),
);
let rows = client.query(&sql, &[&offset, &size]).await?;
```

## 5. Test results

### 5.1 Dialect unit tests (`crates/shared/src/db/dialect.rs`)

All 14 dialect-specific tests pass:

```
test tests::mysql_format_sql_handles_multi_digit ... ok
test tests::mysql_format_sql_preserves_dollar_in_string ... ok
test tests::mysql_ilike ... ok
test tests::mysql_json_type ... ok
test tests::mysql_param_and_cast ... ok
test tests::mysql_quote_ident ... ok
test tests::postgres_cast_text ... ok
test tests::postgres_format_sql_is_identity ... ok
test tests::postgres_ilike ... ok
test tests::postgres_json_type ... ok
test tests::postgres_param_and_cast ... ok
test tests::postgres_quote_ident ... ok
test tests::mysql_dialect_name ... ok
test tests::postgres_dialect_name ... ok
```

### 5.2 CI matrix

`.github/workflows/ci.yml` updated with dialect matrix:

```yaml
unit-tests:
  strategy:
    matrix:
      dialect: [postgres, mysql]
  steps:
    - name: Run unit tests (DATABASE_DIALECT=${{ matrix.dialect }})
      run: cargo test --workspace --lib
      env:
        DATABASE_DIALECT: ${{ matrix.dialect }}
```

Both `postgres` and `mysql` jobs run `cargo test --workspace --lib` (no real DB needed for dialect tests).

### 5.3 Local verification

```bash
# PostgreSQL dialect (default)
cargo test --workspace --lib -- dialect::
# passes: 14 dialect tests

# MySQL dialect
DATABASE_DIALECT=mysql cargo test --workspace --lib -- dialect::
# passes: 14 dialect tests (MySQLDialect path)
```

## 6. Constraints and decisions

| Decision | Rationale |
|---|---|
| Minimal trait (8 methods + 2 defaults) | Only covers differences actually present in queries. No speculative abstractions. |
| `format_sql()` only does `$N` → `?` | Casts need context (the expression being cast), which a blind regex can't provide. Explicit `cast_*()` methods are safer. |
| `OnceLock` for global dialect | No new deps; thread-safe; matches existing `dotenvy::dotenv()` pattern. |
| No changes to pool/connection code | Dialect is purely about SQL text generation. Connection plumbing is untouched. |
| 7+ handlers updated across 4 crates | Representative of ControlPool, raw Pool, `$N`, `::text`, `::bigint` patterns. Not a full migration. |
| No `regex` dep added | Hand-written scanner is 15 lines, zero deps, no transitive bloat. |
| CI matrix on `unit-tests` only | Dialect layer is pure logic; no DB required. Integration tests still run on PostgreSQL only. |

## 7. Follow-up tasks (not in this phase)

1. Migrate remaining ~70 raw-SQL crates to use `dialect().format_sql()` or dialect-aware query builders.
2. Update `RowGet` implementations for MySQL row types (when MySQL connection pool is added).
3. Update `ControlClient` mock tests to verify dialect-transformed SQL strings.
4. Add MySQL integration test service to CI (requires MySQL container).
5. Handle dialect-specific SQL that can't be expressed via current trait methods (e.g., `RETURNING`, `ON CONFLICT`, `ILIKE` in complex queries).
