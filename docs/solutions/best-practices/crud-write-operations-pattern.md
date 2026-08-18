---
title: "CRUD Write Operations Pattern for core_entity Crates"
date: 2026-08-10
category: best-practices
module: oa4rust/crates/*_core_entity
problem_type: best_practice
component: database
severity: medium
symptoms:
  - "core_entity crates initially had only list/get read handlers, no write operations"
  - "Inconsistent soft delete handling: some entities have deleted_at, others don't"
  - "Write handlers missing parameter validation and ownership checks"
root_cause: incomplete_setup
resolution_type: code_fix
tags: [crud, write-operations, soft-delete, core-entity, program-center]
related_components:
  - oa4rust/crates/program_center_core_entity
  - oa4rust/crates/organization_core_entity
  - oa4rust/crates/file_core_entity
applies_when:
  - "Implementing write operations for a new core_entity crate"
  - "Adding CRUD endpoints to an existing read-only core_entity crate"
  - "Reviewing core_entity crates for missing write operations"
---

# CRUD Write Operations Pattern for core_entity Crates

## Context

The oa4rust migration initially focused on read operations (list/get) for all 81 crates. Write operations (create/update/delete) were deferred and then implemented in the production readiness phase. The `program_center_core_entity` crate (39 routes) was the last to receive full write operations, revealing patterns and gotchas applicable to all core_entity crates.

## Guidance

### 1. Check if the entity has deleted_at before implementing delete

```rust
// Entities WITH deleted_at (soft delete):
// invoke, agent, structure in program_center_core_entity
pub async fn invoke_delete(
    db: Extension<DatabaseConnection>,
    path: Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let id = path.into_inner();
    let mut active = entities::invoke::Entity::find_by_id(&id)
        .one(&db).await.map_err(|_| AppError::Internal)?
        .ok_or(AppError::NotFound)?;
    
    active.deleted_at = Set(Some(chrono::Utc::now().naive_utc()));
    active.update(&db).await.map_err(|_| AppError::Internal)?;
    
    Ok(Json(ActionResult::success(serde_json::json!({"deleted": true}))))
}

// Entities WITHOUT deleted_at (no soft delete support):
// application, script in program_center_core_entity
// Return error noting the limitation rather than attempting physical delete
pub async fn application_delete(
    db: Extension<DatabaseConnection>,
    path: Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    // Check ownership first (IDOR prevention)
    require_owner(&pool, &session, &model.creator_person).await?;
    
    // Return error — this entity doesn't support deletion
    Ok(Json(ActionResult::error("delete not supported for this entity")))
}
```

### 2. Always inject Session for creator_person

```rust
pub async fn application_create(
    db: Extension<DatabaseConnection>,
    session: Extension<Session>,  // REQUIRED: from auth_middleware
    Json(req): Json<ApplicationCreateRequest>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    // ...
    let now = chrono::Utc::now().naive_utc();
    let active = entities::application::ActiveModel {
        id: Set(uuid::Uuid::new_v4().to_string()),
        name: Set(req.name),
        creator_person: Set(session.person_unique),  // NEVER trust request body
        created_at: Set(now),
        updated_at: Set(now),
        deleted_at: Set(None),
        // ...
    };
    // ...
}
```

### 3. Use require_owner for update/delete on user-owned resources

```rust
use shared::middleware::require_owner;

// In update handler:
let model = entities::application::Entity::find_by_id(&id)
    .one(&db).await.map_err(|_| AppError::Internal)?
    .ok_or(AppError::NotFound)?;
require_owner(&pool, &session, &model.creator_person).await?;

// In delete handler: same pattern
```

### 4. Consistent validation constants

```rust
const MAX_NAME_LEN: usize = 200;
const MAX_TEXT_LEN: usize = 500;
const MAX_LONG_TEXT_LEN: usize = 2000;
```

Apply these consistently across all entities in a crate.

### 5. Router registration pattern

```rust
pub fn router(pool: deadpool_postgres::Pool) -> axum::Router {
    // Graceful DB connection handling (see nested-tokio-runtime-panic doc)
    let db = std::panic::catch_unwind(|| {
        tokio::runtime::Handle::current()
            .block_on(shared::db::create_sea_orm_pool())
    })
    .ok()
    .and_then(|r| r.ok());

    let router = Router::new()
        .route("/jaxrs/program_center/application", 
            post(application_create)
            .get(application_list))
        .route("/jaxrs/program_center/application/{id}",
            put(application_update).delete(application_delete))
        // ... other routes
        .layer(Extension(pool));

    match db {
        Some(conn) => router.layer(Extension(conn)),
        None => router,
    }
}
```

## Why This Works

- Soft delete is the standard pattern for most entities; entities without `deleted_at` are historical exceptions
- `creator_person` from `Session` is the only trusted source of owner identity
- `require_owner` prevents IDOR by checking resource-level ownership
- The `catch_unwind` pattern ensures tests don't crash when no Tokio runtime is active

## Prevention

- When creating a new core_entity crate, implement write operations in the same PR as read operations
- Add integration tests for create → list → update → delete flow
- Verify `creator_person` is set correctly in create handler tests
- Verify `require_owner` blocks cross-user update/delete in tests

## Related

- [IDOR Vulnerability in Write Handlers](security-issues/idor-vulnerability-write-handlers.md)
- [Input Validation Pattern](best-practices/input-validation-pattern.md)
- [Nested Tokio Runtime Panic](integration-issues/nested-tokio-runtime-panic.md)
- **Impl:** `oa4rust/crates/program_center_core_entity/src/lib.rs`
- **Migration:** `oa4rust/migrations/012_add_creator_person.sql`
- **Plan:** `docs/plans/2026-08-10-001-prod-readiness-plan.md` (U2, U3)
