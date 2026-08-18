---
title: "Input Validation Pattern for Write Handlers"
date: 2026-08-10
category: best-practices
module: oa4rust/crates/*/src/lib.rs
problem_type: best_practice
component: authentication
severity: medium
symptoms:
  - "Inconsistent validation across handlers: some check name non-empty, others don't"
  - "No length limits on text fields, risking DB column overflow"
  - "Validation errors returned as ActionResult::error instead of AppError::BadRequest"
root_cause: missing_validation
resolution_type: code_fix
tags: [validation, input-sanitization, app-error, bad-request]
related_components:
  - oa4rust/crates/program_center_core_entity/src/lib.rs
  - oa4rust/crates/shared/src/error.rs
applies_when:
  - "Implementing create or update handlers for any entity"
  - "Adding write operations to a core_entity crate"
  - "Reviewing handlers for security or data integrity issues"
---

# Input Validation Pattern for Write Handlers

## Context

During the program_center_core_entity write operations implementation, code review found inconsistent input validation across handlers. Some checked `name.trim().is_empty()` inline, others had no validation. There were no length limits on text fields, and validation errors were returned in inconsistent formats.

## Guidance

### 1. Define centralized validation constants and helpers

```rust
// In each crate's lib.rs or a shared validation module
const MAX_NAME_LEN: usize = 200;
const MAX_TEXT_LEN: usize = 500;
const MAX_LONG_TEXT_LEN: usize = 2000;

fn validate_name(name: &str) -> Result<(), AppError> {
    if name.trim().is_empty() {
        return Err(AppError::BadRequest("name is required".to_string()));
    }
    if name.len() > MAX_NAME_LEN {
        return Err(AppError::BadRequest(format!(
            "name must be at most {MAX_NAME_LEN} characters"
        )));
    }
    Ok(())
}

fn validate_text(text: &str, max: usize, field: &str) -> Result<(), AppError> {
    if text.len() > max {
        return Err(AppError::BadRequest(format!(
            "{field} must be at most {max} characters"
        )));
    }
    Ok(())
}
```

### 2. Call validation at the start of create/update handlers

```rust
pub async fn application_create(
    db: Extension<DatabaseConnection>,
    session: Extension<Session>,
    Json(req): Json<ApplicationCreateRequest>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    // Validate input first
    validate_name(&req.name)?;
    if let Some(ref cat) = req.category {
        validate_text(cat, MAX_TEXT_LEN, "category")?;
    }
    // ... proceed with DB operations
}
```

### 3. Always use AppError::BadRequest for validation failures

```rust
// CORRECT: the shared error handler converts this to HTTP 200 + type=error
return Err(AppError::BadRequest("name is required".to_string()));

// WRONG: returning ActionResult::error directly from the handler
// This bypasses the shared error middleware and may produce wrong HTTP status
return Ok(Json(ActionResult::error("name is required")));
```

### 4. Request struct validation with serde

For structs, use serde's `#[validate]` or manual validation in a `validate()` method:

```rust
#[derive(Debug, Deserialize)]
pub struct ApplicationCreateRequest {
    pub name: String,
    pub category: Option<String>,
    pub sub_category: Option<String>,
    pub version: Option<String>,
    pub publisher: Option<String>,
}

impl ApplicationCreateRequest {
    pub fn validate(&self) -> Result<(), AppError> {
        validate_name(&self.name)?;
        if let Some(ref cat) = self.category {
            validate_text(cat, MAX_TEXT_LEN, "category")?;
        }
        Ok(())
    }
}
```

## Why This Works

- Centralized validation helpers eliminate inconsistency across handlers
- `AppError::BadRequest` is properly handled by the shared error middleware to return HTTP 200 + `type=error` (preserving the ActionResult contract)
- Length limits prevent DB column overflow (PostgreSQL TEXT columns have no hard limit but application-level limits prevent abuse)
- Validation at the handler entry point (before DB access) fails fast and avoids unnecessary database operations

## Prevention

- Extract validation into shared helpers in the crate, not inline in handlers
- Always use `AppError::BadRequest` for client-side validation failures
- Apply length limits consistently: name ≤ 200, text ≤ 500, long text ≤ 2000
- Add integration tests that verify validation error responses for missing and oversized fields
- Code review checklist: every create/update handler must call validation helpers before DB operations

## Related

- [IDOR Vulnerability in Write Handlers](security-issues/idor-vulnerability-write-handlers.md)
- [CRUD Write Operations Pattern](best-practices/crud-write-operations-pattern.md)
- [ActionResult<T> 9-Field Contract](architecture-patterns/actionresult-9-field-contract.md)
- **Fix commits:** `869188d9`, `0f66c101`
- **Plan:** `docs/plans/2026-08-10-001-prod-readiness-plan.md` (U2, R4, R5)
