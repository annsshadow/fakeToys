---
title: feat: Multi-Axis Quality & Observability Plan
type: feat
status: active
date: 2026-08-08
---

# Multi-Axis Quality & Observability Plan

## Summary

This plan upgrades the oa4rust codebase across four parallel axes: (1) replace 200+ stub tests with behavior-verifying tests using a shared mock pool framework, (2) fill remaining gaps between Rust handlers and the Java reference endpoint inventory, (3) operationalize the existing but dormant Rust-vs-Java behavior comparison framework, (4) expose an MCP tool layer and/or OpenAPI auto-discovery so the API surface becomes agent-accessible, and (6) stand up a real integration-test pipeline with PostgreSQL, CI, and cross-layer scenario coverage.

---

## Assumptions

*This plan was authored in direct response to a multi-direction request without a prior requirements document. The items below are agent inferences that fill gaps — un-validated bets that should be reviewed before implementation proceeds.*

- Java endpoint alignment means "cover the same paths and HTTP methods" rather than byte-identical response bodies, because the Java reference returns JSON with legacy field naming that the Rust side has already normalized.
- Agent tooling priority is MCP over OpenAPI, because the user explicitly named "agent tooling layer" and MCP is the agent-native protocol; OpenAPI is included as a complementary discovery surface.
- The dormant behavior comparison framework (`tests/behavior_compare.rs`) is salvageable and worth operationalizing rather than rewriting, because it already contains the comparator and reporter logic.
- Integration tests will use a disposable test database (created/migrated per test run) rather than a shared `oa4rust` database, because parallel CI jobs would otherwise collide.
- The shared `crates/shared/src/tests.rs` module is the right place to land a new `MockPool` / `MockClient` framework, because it already holds `lazy_pool()`, `test_app()`, and `send()`.

---

## Requirements

- R1. Replace at least 150 `INTERNAL_SERVER_ERROR` stub assertions with behavior-verifying tests that exercise handler logic through a mock database client.
- R2. Achieve ≥80% route coverage by endpoint inventory: every crate in `docs/brainstorms/oa4rust-endpoint-inventory.md` marked "partial" or "stub" must have all routes registered and returning non-500 on a valid request.
- R3. The behavior comparison suite (`tests/behavior_compare.rs`) must run end-to-end against both Rust and Java services, produce a Markdown diff report, and fail when response status or body structure diverges beyond an allowlist.
- R4. An MCP server process must expose the existing `/jaxrs/*` routes as `list_tools` / `call_tool` operations, with auth forwarded from the current session model.
- R5. An OpenAPI JSON spec must be generated at build time and served at `/openapi.json`, covering ≥90% of registered routes.
- R6. A PostgreSQL-backed integration test suite must exist that runs in CI, migrates a disposable schema per job, and verifies ≥5 cross-crate happy paths (e.g., create org → add person → schedule meeting → post in bbs).
- R7. All unit and integration tests must pass in a single `cargo test --workspace` invocation on a clean checkout, without requiring manual database setup.

---

## Scope Boundaries

- Java response-body byte-identical parity is out of scope; Rust JSON field naming normalization is intentional and preserved.
- MCP tool parameter schemas will be auto-derived from handler signatures rather than hand-authored; perfect IDE autocompletion is deferred.
- Performance / load testing is out of scope; this plan targets correctness and observability only.
- The existing Java service in `oa/o2server/` is treated as a black-box reference endpoint; no Java code changes are in scope.

---

## Key Technical Decisions

- **Mock strategy**: Build a `MockControlClient` trait implementation per aggregate root (org, person, meeting, bbs, file, etc.) and parameterize handlers over `dyn ControlClient` rather than `deadpool_postgres::Pool` in test builds. This avoids per-crate pool duplication and lets tests return deterministic rows without a database. (See `file_assemble_control` and `program_center_core_entity` for existing precedent.)
- **Behavior comparison runner**: Reuse `tests/behavior_compare.rs` and `tests/behavior_comparison/`; add a `--target rust|java|both` flag and a configurable diff allowlist (fields like `updatedAt` / `create_time` that differ by convention). Do not rewrite.
- **MCP protocol library**: Use `rmcp` (Rust MCP SDK) over stdio, with a thin shim that converts MCP `CallToolRequest` into axum `Request` and serializes the axum `Response` back into MCP `CallToolResult`. This keeps one handler implementation and avoids duplicating business logic.
- **OpenAPI generation**: Use `utoipa` with `axum_extras` because it decorates existing handlers with attributes rather than requiring a separate spec DSL. Annotate in batches per crate during the endpoint-alignment unit, not all at once.
- **Integration database lifecycle**: Use `cargo test` with `#[sqlx::test]`-style fixtures or a custom `TestContext` that runs `migrations/*.sql` against a `test_oa4rust_<pid>` database created at startup and dropped at teardown. Avoid Docker Compose in CI because the GitHub Actions runner already has a service container pattern.

---

## Implementation Units

### U1. Shared Mock Framework for Unit Tests

**Goal:** Replace 200+ `INTERNAL_SERVER_ERROR` stubs with deterministic behavior tests by providing a shared `MockPool` / `MockClient` framework in `crates/shared/src/tests.rs`.

**Requirements:** R1

**Dependencies:** None

**Files:**
- Create: `crates/shared/src/mock_client.rs` (trait + default impl)
- Modify: `crates/shared/src/lib.rs` (add `pub mod mock_client;`, move `RowGet`/`ControlClient`/`ControlPool`/`DynControlPool` traits here)
- Modify: `crates/file_assemble_control/src/lib.rs` (re-export traits from `shared`)
- Modify: `crates/program_center_core_entity/src/lib.rs` (import traits from `shared`)
- Modify: `crates/*/src/tests.rs` (crates using `ControlPool` pattern)

**Approach:**
1. Move `RowGet`, `ControlClient`, `ControlPool`, `DynControlPool` trait definitions from `file_assemble_control/src/lib.rs` to `shared/src/lib.rs`.
2. Create `MockControlClient`, `MockControlPool`, `MockRow` in `shared/src/mock_client.rs`.
3. Re-export traits from `file_assemble_control` for backward compatibility.
4. Convert `file_assemble_control` and `program_center_core_entity` tests to use shared mocks.

**Patterns to follow:**
- `file_assemble_control/src/tests.rs` (existing `MockControlClient` / `MockControlPool`)
- `program_center_core_entity/src/tests.rs` (same pattern)

**Test scenarios:**
- Happy path: handler receives valid JSON body → mock client returns preset row → response status is `OK` and body matches expected DTO.
- Edge case: handler receives empty body → mock client is never called → response is `BAD_REQUEST` before pool checkout.
- Error path: mock client returns `SqlxError` → handler translates to `Internal` with structured error body (not bare 500).
- Verification: running `cargo test --workspace` shows zero `INTERNAL_SERVER_ERROR` assertions remaining across crates using `ControlPool`.

**Verification:**
- `cargo test --workspace --lib` passes with zero `INTERNAL_SERVER_ERROR` assertions in `file_assemble_control` and `program_center_core_entity` tests.
- Each of these crates has at least one test that asserts a specific `ActionResult` field or response JSON key.

---

### U2. Java Endpoint Alignment — Missing Routes

**Goal:** Fill the remaining gaps between the Rust route registry and the Java endpoint inventory so that every Java `@Path` + HTTP method combination has a corresponding Rust handler registered.

**Requirements:** R2

**Dependencies:** U1 (mock framework lets us verify routes without DB)

**Files:**
- Modify: `crates/*/src/routes.rs` (crates with missing routes)
- Reference: `docs/brainstorms/oa4rust-endpoint-inventory.md`
- Reference: `D:\WORKSPACE\fakeToys\oa\o2server\` (Java source)

**Approach:**
1. Parse `docs/brainstorms/oa4rust-endpoint-inventory.md` to extract crates with "partial" or "stub" status.
2. For each missing route, read the corresponding Java `*Action.java` to determine path, method, params, and expected `ActionResult` shape.
3. Register the route in the crate's `routes.rs` following the existing `.route(path, get(...))` chain pattern.
4. Add a minimal stub test asserting the route exists and returns `OK` on a valid mock request; deepen to behavior test in U1's batch pass.

**Patterns to follow:**
- `crates/meeting/src/routes.rs` (multi-method route registration)
- `crates/portal/src/routes.rs` (path-param patterns)

**Test scenarios:**
- Happy path: `GET /jaxrs/{module}/{id}` with valid mock → `OK` with expected JSON keys.
- Edge case: `GET /jaxrs/{module}/{id}` with nonexistent ID → `NOT_FOUND` or empty `ActionResult` (matching Java behavior).
- Error path: `POST /jaxrs/{module}` with malformed body → `BAD_REQUEST` before handler executes.
- Verification: route count in `src/main.rs` `create_app()` matches the Java `@Path` count for that module.

**Verification:**
- `scripts/gen_inventory.py` reports zero `TODO` / `stub` entries for targeted crates.
- Manual spot-check: every Java `@GET @Path("{id}")` pattern in `o2server/x_*_assemble_control/src/main/java/` has a Rust counterpart.

---

### U3. Behavior Comparison Suite — Operationalize

**Goal:** Turn the dormant `tests/behavior_compare.rs` and `tests/behavior_comparison/` into a runnable, CI-friendly test that compares Rust and Java responses and reports diffs.

**Requirements:** R3

**Dependencies:** U2 (routes must exist in both systems)

**Files:**
- Modify: `tests/behavior_compare.rs`
- Modify: `tests/behavior_comparison/comparator.rs`
- Modify: `tests/behavior_comparison/reporter.rs`
- Create: `tests/behavior_comparison/allowlist.yaml` (diff allowlist)

**Approach:**
1. Refactor `behavior_compare.rs` from `#[ignore]` / `unimplemented!()` into a parameterized test harness.
2. Add a `--java-url` flag (env var `JAVA_SERVICE_URL`) so the comparator can target the running Java service; default to `http://localhost:8080`.
3. In `comparator.rs`, for each endpoint in the inventory, fire identical requests to Rust and Java, compare status and JSON body keys (allow configured fields to differ, e.g., timestamps).
4. In `reporter.rs`, emit Markdown grouped by crate with per-route pass/fail/diff status.
5. Gate CI: if any route fails comparison, exit non-zero.

**Patterns to follow:**
- `tests/integration_tests.rs` (existing workspace test structure)
- `tests/behavior_comparison/reporter.rs` (existing Markdown generation)

**Test scenarios:**
- Happy path: identical request to Rust and Java → both return `OK` with matching body keys → reporter marks `PASS`.
- Allowlist path: Rust returns `updatedAt` (epoch ms), Java returns `update_time` (ISO 8601) → reporter marks `PASS` via allowlist.
- Divergence path: Rust returns 404, Java returns 200 on a legacy route → reporter marks `FAIL` with diff body.
- Network failure: `JAVA_SERVICE_URL` unreachable → comparator skips Java side and reports `SKIP` rather than panic.

**Verification:**
- `JAVA_SERVICE_URL=http://localhost:8080 cargo test --test behavior_compare` runs to completion and produces `target/debug/behavior-report.md`.
- Report contains ≤5 `FAIL` entries after U2 alignment is complete.

---

### U4. MCP Tool Layer

**Goal:** Expose the existing `/jaxrs/*` routes as MCP `tools` so AI agents can discover and invoke the full OA API surface without raw HTTP knowledge.

**Requirements:** R4

**Dependencies:** R5 (OpenAPI spec provides tool-input schema)

**Files:**
- Create: `crates/mcp_server/src/lib.rs`
- Create: `crates/mcp_server/src/tool_bridge.rs`
- Create: `crates/mcp_server/src/main.rs` (stdio entrypoint)
- Modify: `Cargo.toml` (workspace root, add `mcp_server` member)
- Modify: `src/main.rs` (optional: embed MCP endpoint alongside axum)

**Approach:**
1. Add `rmcp` (or equivalent) as workspace dependency.
2. Build `ToolBridge` that introspects the axum `Router` (or the generated OpenAPI spec from R5) and maps each route to an MCP `Tool` with:
   - `name`: `jaxrs_{crate}_{action}` (e.g., `jaxrs_meeting_get_by_id`)
   - `description`: derived from route path and method
   - `inputSchema`: derived from handler's JSON body / query-param structs
3. Implement `list_tools` returning all mapped tools, and `call_tool` forwarding to axum via `axum::Server`'s `ServiceExt::oneshot` or by constructing an HTTP request internally.
4. Provide two transport modes:
   - **stdio** (default): spawn as subprocess; read JSON-RPC on stdin, write to stdout. Suitable for IDE/agent integration.
   - **HTTP** (optional, behind `--http` flag): mount at `/mcp` in the existing axum app, using the same auth middleware as `/jaxrs/*`.

**Patterns to follow:**
- `crates/ai_assemble_control/src/mcp_config/` (existing MCP config management patterns, for naming conventions only)
- `crates/shared/src/middleware/` (session extraction, to reuse in MCP auth)

**Test scenarios:**
- Happy path: `list_tools` returns ≥200 tools (matching route count).
- Happy path: `call_tool` with valid params for `jaxrs_base_echo` returns the same JSON as `GET /jaxrs/base/echo`.
- Auth path: `call_tool` without valid session → `UNAUTHORIZED` error tool result.
- Error path: `call_tool` with unknown tool name → `INVALID_REQUEST` error tool result.
- Verification: `cargo run --bin mcp_server -- stdio` responds to `{"jsonrpc":"2.0","id":1,"method":"tools/list"}` with a non-empty `tools` array.

**Verification:**
- MCP server binary builds and runs.
- `echo` test from Claude Desktop (or equivalent MCP client) can discover and call a simple route.

---

### U5. OpenAPI Auto-Discovery

**Goal:** Generate a live OpenAPI 3.0 JSON spec at startup and serve it at `/openapi.json`, covering ≥90% of registered routes with parameter schemas and response types.

**Requirements:** R5

**Dependencies:** None (can proceed in parallel with U4)

**Files:**
- Create: `crates/openapi/src/lib.rs`
- Modify: `src/main.rs` (mount `/openapi.json` route)
- Modify: `crates/*/src/routes.rs` (add `#[utoipa::path]` attributes, batch by crate)

**Approach:**
1. Add `utoipa = { version = "4", features = ["axum_extras"] }` and `utoipa-swagger-ui = { version = "6", features = ["axum"] }` to workspace dependencies.
2. Define a root `ApiDoc` struct in `crates/openapi/src/lib.rs` that aggregates sub-specs from each crate.
3. Annotate handlers in batches: start with `base`, `control`, `meeting`, `bbs`, `file`, `portal`, `correlation` (high-traffic / public-facing crates), then sweep the rest.
4. For each handler, add `#[utoipa::path(...)]` with `get`, `post`, `path`, `request_body`, `responses`, and `tag` matching the crate's Java module name.
5. Mount `axum::routing::get("/openapi.json", || async { ApiDoc::openapi().to_json() })` in `create_app()`.
6. Optional: mount `SwaggerUi` at `/swagger-ui` for interactive browsing.

**Patterns to follow:**
- `crates/shared/src/tests.rs` (existing `test_app()` pattern for building a Router with shared state)
- `crates/base/src/routes.rs` (simplest existing router to annotate first)

**Test scenarios:**
- Happy path: `GET /openapi.json` returns 200 with `openapi: "3.0.3"` and `paths` containing ≥350 entries (≥90% of 417 routes).
- Schema validation: every path entry has `requestBody` or `parameters` when the handler expects input, and `responses` includes `200` / `400` / `401` / `500`.
- Error path: unannotated handler still compiles (utoipa attributes are additive; missing annotations just omit the route from the spec).
- Verification: load the JSON into `swagger-cli bundle` (or equivalent) and it validates without errors.

**Verification:**
- `cargo test --workspace` still passes (utoipa macros are compile-time only).
- `curl http://localhost:3000/openapi.json | jq '.paths | length'` returns ≥350.

---

### U6. Integration Test Pipeline — Database & CI

**Goal:** Stand up a disposable PostgreSQL-backed integration test pipeline that runs in CI and verifies cross-crate happy paths against a real database.

**Requirements:** R6, R7

**Dependencies:** U1 (mock framework for unit tests), U2 (routes registered)

**Files:**
- Create: `tests/integration_runner.rs` (rewrite from `unimplemented!()`)
- Create: `tests/integration_tests/db.rs` (database lifecycle: create, migrate, drop)
- Create: `tests/integration_tests/scenarios/` (cross-crate happy-path tests)
- Create: `.env.test.example`
- Modify: `.cargo/config.toml` or `Cargo.toml` (test profile, if needed)
- Create: `.github/workflows/ci.yml`
- Modify: `migrations/` (ensure idempotent, or add a migration runner)

**Approach:**
1. Add `refinery = { version = "0.8", features = ["postgresql"] }` or a simple `sqlx::migrate!()` runner to execute `migrations/*.sql` against the test database.
2. Implement `tests/integration_tests/db.rs`:
   - Read `DATABASE_URL` from env (fallback to `postgres://o2server:password@localhost:5432/oa4rust_test`).
   - Create database `oa4rust_test_<pid>` if not exists.
   - Run migrations.
   - Yield `Pool` to test functions.
   - Drop database after all tests complete (use `once` / `lazy_static` guard).
3. Implement cross-crate scenarios in `tests/integration_tests/scenarios/`:
   - `org_person_meeting.rs`: create org → add person → create meeting → add attendee → verify.
   - `bbs_correlation.rs`: create bbs post → add comment → create correlation → verify link.
   - `file_upload.rs`: create file metadata → upload chunk → verify retrieval.
4. Add `.github/workflows/ci.yml` with two jobs:
   - `unit-tests`: `cargo test --workspace --lib` (no DB needed).
   - `integration-tests`: spin up `postgres:16` service container, run `cargo test --test integration_runner -- --ignored`.
5. Add `.env.test.example` documenting required env vars.

**Patterns to follow:**
- `tests/integration_tests.rs` (existing workspace-level test structure)
- `crates/shared/src/tests.rs` (`lazy_pool()` pattern for deferred connection)
- `crates/auth/src/tests.rs` (real behavior tests that hit auth logic)

**Test scenarios:**
- Happy path: full org → person → meeting flow completes with 200/201 statuses and persisted rows in the test database.
- Transaction rollback: scenario panics mid-flow → test database is still dropped cleanly in teardown.
- Parallel safety: two CI jobs run simultaneously → each gets its own `oa4rust_test_<pid>` database, no collisions.
- Migration idempotency: running migrations twice against an empty database does not error.
- Verification: `cargo test --test integration_runner -- --ignored` passes on a fresh `postgres:16` container with only `migrations/` applied.

**Verification:**
- GitHub Actions run (or local `act` run) shows `unit-tests` and `integration-tests` both green.
- `cargo test --workspace` (without `--ignored`) passes on a machine without PostgreSQL running, proving the default test profile is unit-only.

---

### U7. Shared Test Infrastructure Consolidation

**Goal:** Eliminate per-crate `build_test_pool()` duplication and centralize test helpers in `crates/shared/src/tests.rs` so every crate tests against the same `test_app()` + `send()` contract.

**Requirements:** R1, R7

**Dependencies:** U1 (mock framework design)

**Files:**
- Create: `crates/shared/src/testing.rs` (shared helpers)
- Modify: `crates/*/src/tests.rs` (replace local `build_test_pool()` with `shared::testing::mock_pool()`)

**Approach:**
1. Add `pub mod testing;` to `shared/src/lib.rs`.
2. Move `mock_pool()`, `test_app_with(pool)`, `send_request()` from `tests.rs` to `testing.rs`.
3. Audit all 81 `tests.rs` files for local pool / router builders.
4. For each unique pattern, replace with imports from `shared::testing`.

**Patterns to follow:**
- `crates/shared/src/tests.rs` (existing `test_app()`, `send()`, `lazy_pool()`)

**Test scenarios:**
- Refactor correctness: after replacing `build_test_pool()` in `crates/ai/src/tests.rs`, all tests still compile and pass with the shared mock.
- Regression guard: adding a new `build_test_pool()` to any crate triggers CI lint failure.
- Verification: `grep -r "fn build_test_pool" crates/*/src/tests.rs` returns zero hits after migration.

**Verification:**
- `cargo test --workspace --lib` passes.
- `grep -r "fn build_test_pool" crates/` returns no matches.

---

## System-Wide Impact

- **Interaction graph:** U1 and U7 touch every crate's `tests.rs`; merges must be coordinated so PRs don't conflict on the same files. Recommend a single "test infrastructure" PR landing U7 first, then per-crate U1 conversion PRs in waves of ~10 crates.
- **Error propagation:** Mock framework must preserve the same `AppError` → HTTP status mapping as the real pool, so existing assertions on `StatusCode::INTERNAL_SERVER_ERROR` can be tightened rather than rewritten.
- **State lifecycle risks:** Integration tests (U6) must not leak databases between CI jobs; the `DROP DATABASE` teardown must run even on panic (use `Drop` guard).
- **API surface parity:** MCP layer (U4) and OpenAPI (U5) both depend on route registration; if U2 adds routes mid-flight, U4/U5 annotation must be applied to the new routes or they will be invisible to agents and docs.
- **Integration coverage:** U6's cross-crate scenarios verify that pool injection and middleware ordering work end-to-end; they will catch bugs that per-crate unit tests cannot (e.g., auth middleware stripping a header that a downstream handler expects).

---

## Risks & Dependencies

| Risk | Likelihood | Impact | Mitigation |
|------|-----------|--------|------------|
| Per-crate test conversion causes merge conflicts across 81 crates | High | Medium | Land U7 (shared helpers) first; convert crates in small waves behind feature flags or per-crate branches. |
| Java service version drift makes behavior comparison unreliable | Medium | High | Pin Java service to a released tag (e.g., `o2server@v2.4.1`) in CI; record the tag in `behavior_comparison/allowlist.yaml` header. |
| MCP stdio transport blocks on long-running handler | Medium | Medium | Set per-call timeout (e.g., 30s) and return `TIMEOUT` tool result; async handlers already use tokio so cancellation is safe. |
| OpenAPI annotation burden is large (417 routes) | High | Low | Annotate per-crate in batches; unannotated routes still compile, they just don't appear in the spec. Accept 90% coverage in v1. |
| Integration test database setup is brittle on CI | Medium | Medium | Use GitHub Actions `services: postgres` with a health-check script; fall back to `docker run` in a step if the service container is unavailable. |
| Mock framework cannot express every SQL pattern (COPY, CTE, advisory locks) | Low | Medium | Start with the 80% of handlers that use simple `query` / `execute`; defer complex SQL patterns to U1's follow-up wave. |

---

## Documentation / Operational Notes

- `docs/brainstorms/oa4rust-endpoint-inventory.md` must be kept in sync with U2 progress; update status badges after each crate wave.
- MCP server configuration (stdio vs HTTP, session forwarding) should be documented in a new `docs/operations/mcp-setup.md`.
- OpenAPI spec URL (`/openapi.json`) should be linked from the project README and from the existing `/jaxrs/base/openapi/info` endpoint (replace the hardcoded JSON with a redirect to `/openapi.json`).
- Integration test database name pattern (`oa4rust_test_<pid>`) should be documented in `.env.test.example` so developers can run the suite locally without clobbering their dev database.

---

## Sources & References

- **Origin document:** This plan was authored directly from a multi-direction request; no upstream requirements doc exists.
- **Java reference:** `D:\WORKSPACE\fakeToys\oa\o2server\` (Maven multi-module JAX-RS service)
- **Endpoint inventory:** `docs/brainstorms/oa4rust-endpoint-inventory.md`
- **Completion plan:** `docs/brainstorms/2026-08-04-oa4rust-completion-plan.md`
- **Existing test infra:** `crates/shared/src/tests.rs`, `tests/behavior_compare.rs`, `tests/behavior_comparison/`
- **Rollback plan:** `docs/ops/rollback-plan.md`
- **External refs:** `utoipa` (OpenAPI for Rust), `rmcp` (Rust MCP SDK), `refinery` (SQL migration runner)
