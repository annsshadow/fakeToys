---
title: "Rust Axum Handler Test Pool Pattern with Docker PostgreSQL"
date: 2026-08-13
category: best-practices
module: oa4rust
problem_type: best_practice
component: testing_framework
severity: medium
applies_when:
  - "Writing unit tests for axum handlers that need database access"
  - "Testing handlers with Extension<Pool> or Extension<DatabaseConnection>"
  - "Setting up test infrastructure for a Rust workspace with PostgreSQL"
tags: [rust, axum, testing, postgresql, docker, deadpool, sea-orm, pool]
---

# Rust Axum Handler Test Pool Pattern with Docker PostgreSQL

## Context

Unit testing axum handlers that depend on a database requires a test database connection pool. In the oa4rust project, handlers use two different pool types:
- `deadpool_postgres::Pool` (via `Extension<Pool>`)
- `sea_orm::DatabaseConnection` (via `Extension<DatabaseConnection>`)

Both need to connect to the same Docker PostgreSQL instance for tests, but they have incompatible types and connection patterns.

## Guidance

### Dual Pool Pattern

Create two pool factory functions in `crates/shared/src/testing.rs`:

**deadpool_postgres Pool (for Extension<Pool> handlers):**
```rust
/// Connects to Docker PG at localhost:5433.
/// Pool is built eagerly but connections are lazy (first .get() triggers connect).
pub fn test_pool() -> Pool {
    let mut cfg = deadpool_postgres::tokio_postgres::Config::new();
    cfg.host("localhost")
        .port(5433)
        .user("postgres")
        .dbname("postgres");
    let mgr = deadpool_postgres::Manager::new(cfg, deadpool_postgres::tokio_postgres::NoTls);
    Pool::builder(mgr).max_size(5).build().unwrap()
}
```

**sea_orm DatabaseConnection (for Extension<DatabaseConnection> handlers):**
```rust
/// Returns Result so PG unavailability doesn't panic the test runner.
pub async fn test_sea_orm_pool() -> Result<DatabaseConnection, String> {
    let mut options = ConnectOptions::new("postgres://postgres@localhost:5433/postgres");
    options.max_connections(5).sqlx_logging(false);
    Database::connect(options).await.map_err(|e| e.to_string())
}
```

### Key Design Decisions

1. **Lazy connection**: `test_pool()` builds the pool immediately (no network), but `.get().await` lazily establishes connections. This means tests start fast even if PG is down.

2. **Result return for sea_orm**: `test_sea_orm_pool()` returns `Result` instead of panicking. This prevents the entire test suite from failing when the Docker PG container is not running.

3. **Same Docker container**: Both pools connect to `yhmbs_pg_test` on `localhost:5433` with `trust` auth. Ensure the container is running before tests:
   ```bash
   docker exec yhmbs_pg_test psql -U postgres -c "SELECT 1"
   ```

4. **No separate test database**: Use the default `postgres` database. The migrations in `migrations/` are applied by the integration test infrastructure (`tests/integration_tests/db.rs::init_test_database()`).

### Testing the Pool Functions

Add tests in `crates/shared/src/tests.rs`:
```rust
#[test]
fn test_test_pool_builds_without_connecting() {
    // Pool construction is local — no network required
    let _pool = crate::testing::test_pool();
}

#[tokio::test]
async fn test_test_sea_orm_pool_connects() {
    // Returns Err if PG not reachable — don't panic
    let result = crate::testing::test_sea_orm_pool().await;
    let _ = result;
}
```

### Using in Generated Tests

The test generator script (`scripts/generate_handler_tests.py`) automatically selects the right pool:
- Handlers with `Extension<Pool>` → use `test_pool()` (synchronous, no `.await`)
- Handlers with `Extension<DatabaseConnection>` → use `test_sea_orm_pool().await`
- Handlers with `Extension<SessionManager>` → skip (cannot construct in unit test)

## Why This Matters

Without a consistent test pool pattern, each crate would need its own connection configuration, leading to:
- Duplicate connection strings scattered across crates
- Inconsistent test behavior when PG is unavailable
- More maintenance burden when changing test database configuration

The shared `shared::testing` module centralizes this concern and is already a dependency of all business crates.

## When to Apply

- Adding new crates that need handler-level tests
- Setting up CI test infrastructure for a Rust + PostgreSQL workspace
- Migrating from SQLx to SeaORM (or vice versa) and needing both pool types

## Examples

### Direct-call test (handler is exported from crate root)
```rust
#[tokio::test]
async fn test_applications() {
    let _result = crate::applications(
        axum::extract::Extension(shared::testing::test_pool()),
    ).await;
}
```

### Router-based test (handler not exported, accessed via oneshot)
```rust
#[tokio::test]
async fn test_get_user() {
    let pool = shared::testing::test_pool();
    let app = crate::router(pool);
    let response = app
        .oneshot(Request::builder()
            .uri("/jaxrs/user/get/123")
            .method("GET")
            .body(Body::empty()).unwrap())
        .await.unwrap();
    assert_ne!(response.status(), StatusCode::NOT_FOUND);
}
```

### SeaORM handler test
```rust
#[tokio::test]
async fn test_entity_handler() {
    let conn = shared::testing::test_sea_orm_pool().await.unwrap();
    let _result = crate::handler(
        axum::extract::Extension(conn),
    ).await;
}
```

## Related
- [Auto-Generate Rust Handler Unit Tests](auto-generate-rust-handler-tests.md) — the full test generation approach
- [sea-orm Dual Pool Coexistence](architecture-patterns/seaorm-dual-pool-coexistence.md) — production dual-pool design
