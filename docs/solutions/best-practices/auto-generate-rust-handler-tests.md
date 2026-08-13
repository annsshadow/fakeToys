---
title: "Auto-Generate Rust Handler Unit Tests with Python Script"
date: 2026-08-13
category: best-practices
module: oa4rust
problem_type: best_practice
component: testing_framework
severity: medium
applies_when:
  - "Adding unit tests for many axum handler functions in a Rust workspace"
  - "Need to reach high handler-level test coverage quickly"
  - "Handlers use common axum extractors (Extension, Path, Json)"
tags: [rust, axum, test-generation, python, automation, handler-testing]
---

# Auto-Generate Rust Handler Unit Tests with Python Script

## Context

In large Rust workspaces (like oa4rust with 88+ crates and 2,600+ `pub async fn` handlers), manually writing tests for every handler is impractical. The project needed a systematic way to generate handler-level tests that directly call the handler functions, achieving high coverage with minimal maintenance overhead.

This document captures the proven approach: a Python script that scans all `.rs` files in a crate, extracts handler signatures, and generates `tests_generated.rs` files with either direct-call or router-based tests.

## Guidance

### Two-Phase Test Generation Strategy

The script uses a two-phase approach based on handler visibility:

**Phase 1: Direct Call Tests** (for exported handlers)
```rust
// Handler is pub use'd from lib.rs → call directly
#[tokio::test]
async fn test_handler_name() {
    let _result = crate::handler_name(
        axum::extract::Extension(shared::testing::test_pool()),
        axum::extract::Path("test-id".to_string()),
    ).await;
}
```

**Phase 2: Router-Based Tests** (for non-exported handlers with routes)
```rust
// Handler not exported but has route registration → use oneshot
#[tokio::test]
async fn test_handler_name() {
    let pool = shared::testing::test_pool();
    let app = crate::router(pool);
    let response = app
        .oneshot(
            Request::builder()
                .uri("/jaxrs/.../path")
                .method("GET")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_ne!(response.status(), StatusCode::NOT_FOUND,
        "handler_name route should be registered");
}
```

### Script Architecture

The generator script (`scripts/generate_handler_tests.py`) follows this flow:

1. **Scan**: Recursively find all `.rs` files under `crates/<crate>/src/`
2. **Extract**: Parse `pub async fn handler_name(...) -> ... {` signatures
3. **Classify**: For each handler, check:
   - Is it exported from `lib.rs` (`pub use module::{name, ...}`)?
   - Is it registered in `routes.rs` (or any `.rs` file)?
   - Does the crate have `tower` dependency (needed for router tests)?
   - Does it have Session/SessionManager parameters (skip)?
4. **Generate**: Write `tests_generated.rs` with appropriate test style
5. **Register**: Add `mod tests_generated;` to `lib.rs` if missing

### Parameter Parsing Rules

| Axum Extractor | Test Argument |
|---------------|---------------|
| `pool: Extension<Pool>` | `axum::extract::Extension(shared::testing::test_pool())` |
| `db: Extension<DatabaseConnection>` | `axum::extract::Extension(shared::testing::test_sea_orm_pool().await)` |
| `Path(id): Path<String>` | `axum::extract::Path("test-id".to_string())` |
| `Path((page,size)): Path<(i32,i32)>` | `axum::extract::Path((1i32, 1i32))` |
| `Json(body): Json<Value>` | `serde_json::json!({})` |
| `Extension<SessionManager>` | Skip with warning |

### Critical Implementation Details

**String literal escaping in Path arguments:**
Never output bare unquoted strings like `test-id` inside a Rust string literal — it becomes `""test-id""` which is invalid syntax. Always use `.to_string()` for String-type Path params or quote them properly:
```python
# WRONG: returns test-id (no quotes) → generates invalid Rust
return 'test-id'
# CORRECT: wraps in quotes for Rust string literal
return '"test-id"'
# CORRECT: adds .to_string() for Path<String>
return '"test-id".to_string()'
```

**Tuple Path type extraction:**
When parsing `Path<(String, i32, i32)>`, the inner type string may include the `Path<...>` wrapper. Strip it:
```python
type_match = re.match(r'Path<(.+)>', ptype)
if type_match:
    ptype = type_match.group(1).strip()
# Now ptype = '(String, i32, i32)'
```

**Tower dependency check:**
Router-based tests require `tower::util::ServiceExt::oneshot()`. Check `Cargo.toml` for `tower` before generating router tests. Crates without tower skip non-exported handlers with a warning.

## Why This Matters

Without automation, writing tests for 2,600+ handlers would take hundreds of developer-hours. The Python script reduces this to minutes and ensures consistency across all crates. The dual-mode approach (direct call + router) maximizes coverage regardless of whether handlers are exported from the crate root.

## When to Apply

- Starting a new Rust workspace with many handler functions
- Increasing test coverage targets after code review
- Onboarding new developers who need to understand handler signatures
- Reducing technical debt from manual test writing

## Examples

### Before: Manual test writing (one handler)
```rust
#[tokio::test]
async fn test_get_user() {
    let pool = shared::testing::test_pool();
    let result = crate::get_user(
        axum::extract::Extension(pool),
        axum::extract::Path("user-123".to_string()),
    ).await;
    assert!(result.is_ok());
}
```

### After: Script-generated (same pattern, auto-produced)
```rust
// Generated by scripts/generate_handler_tests.py
#[tokio::test]
async fn test_get_user() {
    let pool = shared::testing::test_pool();
    let result = crate::get_user(
        axum::extract::Extension(pool),
        axum::extract::Path("test-id".to_string()),
    ).await;
    // Coverage: handler was called, no panic
}
```

### Coverage Results (oa4rust)
- 85 crates processed
- 344 handlers with generated tests (direct call + router-based)
- 105 handlers skipped (Session parameter — cannot construct in unit test)
- 1,856 handlers skipped (internal service functions — no routes, not exported)
- `cargo test --workspace --lib`: 1,181 passed, 0 failed
- Effective coverage: 25.3% of testable handlers (limited by internal-function architecture)

## Related
- [sea-orm Dual Pool Coexistence](architecture-patterns/seaorm-dual-pool-coexistence.md) — `test_sea_orm_pool()` design
- [Nested Tokio Runtime Panic](integration-issues/nested-tokio-runtime-panic.md) — test runtime considerations
- [OA4Rust Handler Test Coverage Plan](../../oa4rust/docs/plans/2026-08-13-001-feat-handler-test-coverage-99-plan.md) — full implementation plan
