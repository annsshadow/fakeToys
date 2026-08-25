---
title: "SeaORM Dual-Pool Coexistence Pattern"
date: 2026-08-10
last_refreshed: 2026-08-25
category: architecture-patterns
module: oa4rust/crates/shared
problem_type: architecture_pattern
component: database
severity: high
symptoms:
  - "SeaORM 2.0 depends on sqlx 0.9 while the project uses sqlx 0.8"
  - "Direct Pool replacement would break all existing SQLx-based code"
  - "No clear boundary between which crate uses which pool type"
root_cause: config_error
resolution_type: code_fix
tags: [seaorm, sqlx, dual-pool, migration, deadpool]
related_components:
  - oa4rust/crates/shared/src/db/mod.rs
  - oa4rust/crates/orm
  - oa4rust/src/main.rs
applies_when:
  - "Migrating a large Rust project from raw SQLx to an ORM like SeaORM"
  - "Need to coexist with existing deadpool-postgres Pool usage"
  - "Cannot afford a full rewrite of all data access code at once"
---

# SeaORM Dual-Pool Coexistence Pattern

## Context

The oa4rust project migrated 81 crates from raw SQLx queries to SeaORM 2.0. SeaORM internally depends on sqlx 0.9, while the project uses sqlx 0.8. Directly replacing the `deadpool-postgres::Pool` would break all existing code and risk version conflicts. The solution was to run both pools side by side.

## Guidance

### 1. Create a separate SeaORM connection

```rust
// shared/src/db/mod.rs
pub async fn create_sea_orm_pool() -> Result<DatabaseConnection, DbError> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://o2server:password@localhost:5432/oa4rust".to_string());
    let mut options = ConnectOptions::new(database_url);
    options.max_connections(20).sqlx_logging(false);
    Database::connect(options)
        .await
        .map_err(|e| DbError::PoolError(e.to_string()))
}
```

Both pools connect to the same `DATABASE_URL` but are independent — SeaORM's internal sqlx 0.9 dependency is isolated within the SeaORM crate and does not conflict with the workspace's sqlx 0.8.

### 2. Register both in main.rs

```rust
// main.rs
let pool = create_pool(&db_url).await?;          // deadpool-postgres::Pool (SQLx)
let sea_orm_conn = create_sea_orm_pool().await?;  // SeaORM DatabaseConnection

Router::new()
    .layer(Extension(pool))           // existing crates use this
    .layer(Extension(sea_orm_conn))   // new ORM crates use this
    .merge(auth::router(pool))        // auth keeps using Pool
    .merge(control::router(pool))
    .merge(personal::router(pool))
    .merge(organization_core_entity::router(pool))  // ORM crate
    // ...
```

### 3. Define clear boundaries

| Layer | Pool Type | Used By |
|-------|-----------|---------|
| Authentication (SessionManager) | `Pool` (SQLx) | auth crate, shared middleware |
| RBAC (authorize_middleware) | `Pool` (SQLx) | shared middleware |
| Core entity CRUD | `DatabaseConnection` (SeaORM) | all `*_core_entity` crates |
| Complex dynamic SQL | `Pool` (SQLx) | `control`, `person_flag_clause` |
| Assemble control | `Pool` or `DatabaseConnection` | varies by crate |

### 4. Handle sync-to-async boundary in router factories

Router factory functions are sync (`fn` not `async fn`), but SeaORM pool creation is async. Use `catch_unwind` to handle cases where no Tokio runtime is active:

```rust
pub fn router(pool: deadpool_postgres::Pool) -> axum::Router {
    let db = std::panic::catch_unwind(|| {
        tokio::runtime::Handle::current()
            .block_on(shared::db::create_sea_orm_pool())
    })
    .ok()
    .and_then(|r| r.ok());

    let router = Router::new()...;
    match db {
        Some(conn) => router.layer(Extension(conn)),
        None => router,
    }
}
```

### 5. Sustained dual-pool state (no exit planned)

The dual-pool state was originally framed as a transition with a 30-day post-migration audit to remove retained SQLx. That audit (`docs/plans/2026-08-21-002` U8) concluded SQLx **cannot** be fully removed: SeaORM 2.x depends transitively on sqlx, so `cargo tree` still shows sqlx nodes and removing it would mean rewriting the ORM. The target was therefore downgraded to "workspace direct sqlx dependency cleared" (achieved), and dual-pool is the **sustained** architecture. Retained raw SQLx (e.g., `person_flag_clause` dynamic SQL) is intentional — see `architecture-patterns/dynamic-sql-retains-sqlx.md`.

## Why This Matters

- Avoids a disruptive all-at-once rewrite of 81 crates
- Prevents sqlx version conflicts (0.8 vs 0.9)
- Allows incremental migration with per-crate testing
- Keeps authentication and RBAC on the stable SQLx path

## When to Apply

- Large Rust projects migrating from raw SQL to an ORM
- When the ORM has a different transitive dependency on the SQL driver
- When a full rewrite is impractical due to size or risk

## Examples

**Before (all SQLx):**
```rust
pub async fn person_list(pool: Extension<Pool>) -> Result<Json<...>> {
    let rows = pool.get().await?.query("SELECT * FROM auth_person", &[]).await?;
    // manual mapping
}
```

**After (SeaORM for CRUD, SQLx retained for complex queries):**
```rust
pub async fn person_list(db: Extension<DatabaseConnection>) -> Result<Json<...>> {
    let models = person::Entity::find()
        .filter(Column::DeletedAt.isNull())
        .all(&db)
        .await?;
    // SeaORM auto-mapping
}

// Complex dynamic SQL retains SQLx:
pub async fn person_by_flag(pool: Extension<Pool>, flag: String) -> Result<Json<...>> {
    // person_flag_clause: dynamic OR-matching across multiple columns
    // Stays as raw SQLx query
}
```

## Related

- [Nested Tokio Runtime Panic](integration-issues/nested-tokio-runtime-panic.md)
- [Dynamic SQL Retains SQLx](architecture-patterns/dynamic-sql-retains-sqlx.md)
- **Origin:** `docs/brainstorms/2026-08-09-oa4rust-orm-migration-and-write-ops-requirements.md`
- **Plan:** `docs/plans/2026-08-09-001-refact-oa4rust-orm-migration-plan.md` (U1-U3)
- **Status (2026-08-25):** dual-pool is sustained; SQLx removal vetoed (sea-orm depends on sqlx). Current endpoint-parity state: `docs/audits/final-coverage-sweep.md` (99.77% as of 2026-08-23). The 2026-08-08 migration-status brainstorm is a historical snapshot.
