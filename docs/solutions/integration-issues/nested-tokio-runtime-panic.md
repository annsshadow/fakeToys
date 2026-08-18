---
title: "Nested Tokio Runtime Panic in SeaORM Routers"
date: 2026-08-10
category: integration-issues
module: oa4rust/crates/*/core_entity
problem_type: runtime_error
component: database
severity: critical
symptoms:
  - "Router factory functions calling tokio::runtime::Handle::current().block_on() panic when no runtime exists"
  - "Tests crash with 'no current runtime found' in core_entity crates"
  - "Multiple sequential fix attempts before finding the correct pattern"
root_cause: async_timing
resolution_type: code_fix
tags: [tokio, seaorm, panic, router, catch-unwind]
related_components:
  - oa4rust/crates/shared
  - oa4rust/src/main.rs
---

# Nested Tokio Runtime Panic in SeaORM Routers

## Problem

When migrating core_entity crates from SQLx to SeaORM, router factory functions (sync `fn` returning `Router`) needed to create a `DatabaseConnection` asynchronously. Calling `tokio::runtime::Handle::current().block_on()` inside these sync functions panicked when no Tokio runtime was active — particularly in tests and during Axum router construction.

## Symptoms

- Tests for core_entity crates crashed with `"no current runtime found"` or nested runtime panics
- Router registration in `main.rs` panicked during startup if called outside an async context
- Multiple commits were needed before finding the correct fix:
  - `2c47b3a0` → first attempt at fixing nested runtime panic
  - `a01d9832` → rollback of async router approach
  - `6fcf5646` → fix in `attendance_core_entity` only
  - `3cbb5fd1` → added catch_unwind in attendance router
  - `ba8d1368` → final fix applied to all 17 core_entity crates

## What Didn't Work

1. **Making router functions async** — Axum router factory functions must be sync (`fn`, not `async fn`), so this breaks the API contract
2. **Creating a new `tokio::runtime::Runtime` inside the router** — nested runtime creation also panics in this context
3. **Fixing one crate at a time** — the pattern was needed in all 17 core_entity crates; partial fixes left the rest broken
4. **`.expect("failed to create sea-orm connection")`** — hard panics are unacceptable in router factories; tests must not crash the process

## Solution

Wrap the `block_on` call in `std::panic::catch_unwind` and gracefully degrade when the DB connection cannot be created:

```rust
pub fn router(pool: deadpool_postgres::Pool) -> axum::Router {
    // Gracefully handle cases where no Tokio runtime is active
    let db = std::panic::catch_unwind(|| {
        tokio::runtime::Handle::current()
            .block_on(shared::db::create_sea_orm_pool())
    })
    .ok()
    .and_then(|r| r.ok());

    let router = Router::new()
        .route("/jaxrs/...", get(list_handler))
        // ... other routes
        .layer(Extension(pool));

    match db {
        Some(conn) => router.layer(Extension(conn)),
        None => router, // graceful degradation in test/sync contexts
    }
}
```

This pattern was applied atomically across all 17 core_entity crates in a single commit (`ba8d1368`).

## Why This Works

- `tokio::runtime::Handle::current()` panics if no runtime is in the current thread's context
- `catch_unwind` catches the panic at the Rust level, converting it to `Err`
- The double `.ok().and_then()` flattens `Result<Pool, _>` → `Option<Pool>`
- The router still works without the DB connection (list queries return 500, which is acceptable in tests)
- In production, a Tokio runtime is always active, so the DB connection is created successfully

## Prevention

- Always use `catch_unwind` around `block_on` calls in sync factory functions
- Never `.expect()` on pool creation in router factories — tests will fail
- Apply the pattern consistently across all crates in a single commit (don't fix one at a time)
- The pattern is documented in `docs/solutions/architecture-patterns/seaorm-dual-pool-coexistence.md`

## Related

- [SeaORM Dual-Pool Coexistence](architecture-patterns/seaorm-dual-pool-coexistence.md)
- [Plan Document Lifecycle Management](development-workflow/plan-status-lifecycle.md)
- **Fix commit:** `ba8d1368` — fix: catch panic in all core_entity routers with catch_unwind
- **Prior attempts:** `2c47b3a0` → `a01d9832` → `6fcf5646` → `3cbb5fd1` → `ba8d1368`
- **Plan:** `docs/plans/2026-08-09-001-refact-oa4rust-orm-migration-plan.md` (U1, Wave 1)
