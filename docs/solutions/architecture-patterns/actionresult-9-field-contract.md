---
title: "ActionResult<T> 9-Field Contract Preservation"
date: 2026-08-10
category: architecture-patterns
module: oa4rust/crates/shared/src/response.rs
problem_type: architecture_pattern
component: authentication
severity: high
symptoms:
  - "Frontend action.js strongly depends on a 9-field JSON structure from ActionResult<T>"
  - "Any ORM migration or new endpoint risks changing the response shape"
  - "Business errors must return HTTP 200 + type=error, not HTTP error codes"
root_cause: config_error
resolution_type: code_fix
tags: [actionresult, frontend-contract, serialization, oa4rust]
related_components:
  - oa4rust/crates/shared/src/response.rs
  - oa/o2web/source/x_init/src/common/action.js
applies_when:
  - "Adding new API endpoints to oa4rust"
  - "Migrating data access layers (SQLx → SeaORM)"
  - "Any change that touches handler response construction"
---

# ActionResult<T> 9-Field Contract Preservation

## Context

The frontend `o2web` uses `action.js` which extracts `json.data` from every API response. The Rust backend must return exactly 9 fields in the `ActionResult<T>` envelope, or the frontend breaks silently. This contract is non-negotiable — it is the primary API surface between Rust and the existing Java-agnostic frontend.

## Guidance

### The 9-Field Structure

```rust
// shared/src/response.rs
pub struct ActionResult<T> {
    pub data: T,           // The actual response payload
    pub r#type: Option<String>,  // "success" or "error"
    pub message: Option<String>, // Human-readable message
    pub date: Option<String>,    // ISO timestamp
    pub spent: Option<f64>,      // Request duration in ms
    pub size: Option<i64>,       // Current page size
    pub count: Option<i64>,      // Total count (for pagination)
    pub position: Option<String>,// "next" or "prev" cursor
    pub prompt: Option<String>,  // Additional prompt text
}
```

### Key Rules

1. **Always wrap responses in `ActionResult`** — never return raw `Json(model)`
2. **Business errors return HTTP 200 + `type=error`** — HTTP status codes are only for transport errors (401, 403, 429)
3. **Use `ActionResult<Value>` during ORM migration** — keep the generic as `serde_json::Value` to preserve flexibility; don't introduce DTOs yet
4. **Field naming** — use `#[serde(rename = "camelCase")]` on entity fields or manual mapping in handlers to match what the frontend expects

### Example: Correct Response Pattern

```rust
use shared::response::ActionResult;
use serde_json::Value;

pub async fn person_list(
    db: Extension<DatabaseConnection>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let models = person::Entity::find()
        .filter(Column::DeletedAt.isNull())
        .limit(20)
        .all(&db)
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = models.iter().map(|m| {
        serde_json::json!({
            "id": m.id,
            "name": m.name,
            "mobile": m.mobile.as_ref().unwrap_or(&String::new()).clone(),
        })
    }).collect();

    Ok(Json(ActionResult {
        data: serde_json::json!({ "count": data.len(), "data": data }),
        r#type: Some("success".to_string()),
        message: None,
        date: None,
        spent: None,
        size: Some(20),
        count: Some(data.len() as i64),
        position: None,
        prompt: None,
    }))
}
```

### Example: Business Error (HTTP 200 + type=error)

```rust
// WRONG: returning HTTP 400 for business validation errors
return Err(AppError::BadRequest("name is required".to_string()));

// CORRECT: the shared error handler converts AppError::BadRequest to
// HTTP 200 + ActionResult with type=error (via the error middleware)
```

## Why This Matters

- The frontend `action.js` does `const result = await fetch(...); const data = result.json().data;`
- If the response shape changes, the frontend silently gets `undefined` instead of data
- This caused the "fakeToys" project name — the initial migration produced responses that the frontend couldn't parse
- The 9-field structure is a contract with the entire o2web frontend ecosystem

## When to Apply

- Every new handler must follow this pattern
- During ORM migration, verify that the new SeaORM-based handler returns the same JSON shape as the old SQLx handler
- The behavior comparison test framework (7,624 endpoints) serves as an automated safety net for contract drift

## Prevention

- Add a response shape test in each crate's `tests.rs` that verifies the 9-field structure
- The behavior comparison tests (`tests/behavior_compare.rs`) automatically check field names and types against Java responses
- Never add top-level fields outside the `ActionResult` envelope

## Related

- [SeaORM Dual-Pool Coexistence](architecture-patterns/seaorm-dual-pool-coexistence.md)
- [Input Validation Pattern](best-practices/input-validation-pattern.md)
- **Source:** `oa/o2web/source/x_init/src/common/action.js` (frontend consumer)
- **Struct:** `oa4rust/crates/shared/src/response.rs`
- **Behavior compare:** `oa4rust/tests/behavior_compare.rs` (7,624 endpoints)
