---
title: "Dynamic SQL Retains SQLx: When Not to Migrate"
date: 2026-08-10
category: architecture-patterns
module: oa4rust/crates/control
problem_type: architecture_pattern
component: database
severity: medium
symptoms:
  - "SeaORM cannot express dynamic multi-column OR matching patterns"
  - "person_flag_clause builds SQL with runtime-determined column lists"
  - "Forcing SeaORM on dynamic SQL produces uglier, less maintainable code"
root_cause: wrong_api
resolution_type: code_fix
tags: [seaorm, sqlx, dynamic-sql, person-flag-clause, migration-strategy]
related_components:
  - oa4rust/crates/control/src/person.rs
  - oa4rust/crates/shared/src/middleware.rs
applies_when:
  - "Deciding which queries to migrate from SQLx to SeaORM"
  - "Queries involve dynamic WHERE clause construction"
  - "Multi-field OR matching or conditional column selection"
---

# Dynamic SQL Retains SQLx: When Not to Migrate

## Context

The oa4rust SeaORM migration plan explicitly states that not all queries should be migrated. Dynamic SQL patterns — particularly `person_flag_clause` in `control/src/person.rs` — use runtime-determined column lists and multi-field OR matching that SeaORM's query builder cannot express cleanly.

## Guidance

### The person_flag_clause Pattern

```rust
// In control/src/person.rs — this pattern is deliberately kept as raw SQLx
fn person_flag_clause(flag: &str) -> String {
    // Dynamic SQL: matches flag against multiple columns with OR
    // SeaORM cannot express this without losing type safety or readability
    format!(
        "({} = $1) OR ({} = $1) OR ({} = $1)",
        "unique_id", "person_flag", "email"
    )
}

pub async fn person_get(
    pool: Extension<Pool>,
    flag: Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let flag = flag.into_inner();
    let clause = person_flag_clause(&flag);
    
    // Raw SQLx query — kept because SeaORM cannot express this pattern
    let sql = format!(
        "SELECT * FROM auth_person WHERE {} AND deleted_at IS NULL LIMIT 1",
        clause
    );
    
    let row = pool.get().await?.query_one(&sql, &[&flag]).await?;
    // ... map to ActionResult
}
```

### Decision Framework: Migrate or Keep SQLx?

| Query Pattern | Migrate to SeaORM? | Reason |
|--------------|-------------------|--------|
| Simple CRUD (INSERT/UPDATE/DELETE by ID) | ✅ Yes | SeaORM excels here |
| List with static WHERE + pagination | ✅ Yes | `Entity::find().filter(...).paginate(...)` |
| List with dynamic OR across columns | ❌ No | `person_flag_clause` pattern |
| Complex JOIN across multiple tables | ⚠️ Evaluate | If join is static, yes; if dynamic, no |
| Aggregation (COUNT, SUM, GROUP BY) | ✅ Yes | SeaORM's `into_json()` + group_by |
| Subquery or CTE | ❌ No | Limited SeaORM support |
| DDL/DML operations | ❌ No | Use SQLx or SeaQuery directly |

### Boundary Declaration

In each migrated crate's documentation or code comments, explicitly declare which queries retained SQLx and why:

```rust
// NOTE: The following queries retain raw SQLx because SeaORM cannot express
// the dynamic multi-column matching pattern. All other queries in this crate
// use SeaORM DatabaseConnection.
//
// Retained SQLx queries:
// - person_flag_clause: dynamic OR across unique_id, person_flag, email
// - unit_hierarchy: recursive CTE for organizational unit tree
```

## Why This Works

- Mixing SQLx and SeaORM in the same project is supported by the dual-pool pattern
- The `Pool` (deadpool-postgres) remains available via `Extension<Pool>` for queries that need it
- `DatabaseConnection` (SeaORM) is used via `Extension<DatabaseConnection>` for standard CRUD
- This avoids the temptation to force SeaORM onto queries it handles poorly

## Prevention

- During migration, audit each query in the crate and classify it as "SeaORM-friendly" or "SQLx-retained"
- Don't migrate a query just for the sake of consistency — evaluate each one
- Document retained SQLx queries with comments explaining why
- The 30-day post-migration audit should revisit retained SQLx queries to see if SeaORM has gained capabilities

## Related

- [SeaORM Dual-Pool Coexistence](architecture-patterns/seaorm-dual-pool-coexistence.md)
- [Nested Tokio Runtime Panic](integration-issues/nested-tokio-runtime-panic.md)
- **Reference:** `oa4rust/crates/control/src/person.rs` (person_flag_clause)
- **Decision:** `docs/plans/2026-08-09-001-refact-oa4rust-orm-migration-plan.md` (Key Technical Decisions)
