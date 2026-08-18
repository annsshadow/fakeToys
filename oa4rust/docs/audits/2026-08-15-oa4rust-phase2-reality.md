# OA4RUST Phase 2 Reality Audit — 2026-08-15

## Routed-endpoint DB-touch rate

Per-crate takeover metric for **routed** endpoints (axum `.route(...)` registrations),
classifying each referenced handler by whether its body touches the database. This
is the authoritative metric for Phase 2: it measures DB-backed coverage of the
*actually-wired* surface, not merely defined handlers.

Methodology: parse each crate's `router()`/`routes.rs` for
`.route("...", METHOD(handler))` (axum method chaining; chained
`put(a).delete(b)` counts as two endpoints). For each handler identifier, locate
`fn <ident>` in the crate's `src/**/*.rs` and test its body against DB-access
patterns: `pool.get(`, `client.query(`, `client.execute(`, `.query(`, `.execute(`,
`Entity::find`, `fetch_`, `sqlx`, `row_to_json`,
`Model::find`/`insert`/`update`/`delete`, `deadpool`. Handlers that cannot be
resolved to a `fn` in the same crate are reported as `unknown` (variables,
external `::`-paths, closures).

| crate | routed_endpoints | routed_touch_db | routed_shell | unknown | rate% |
|---|---:|---:|---:|---:|---:|
| organization_assemble_control | 105 | 99 | 6 | 0 | 94.3% |
| attendance_assemble_control | 87 | 86 | 1 | 0 | 98.9% |
| general_assemble_control | 61 | 61 | 0 | 0 | 100.0% |
| message_assemble_communicate | 58 | 58 | 0 | 0 | 100.0% |
| meeting_assemble_control | 42 | 42 | 0 | 0 | 100.0% |
| auth | 41 | 0 | 41 | 0 | 0.0% |
| ai_assemble_control | 37 | 36 | 1 | 0 | 97.3% |
| hotpic_assemble_control | 36 | 36 | 0 | 0 | 100.0% |
| jpush_assemble_control | 32 | 32 | 0 | 0 | 100.0% |
| program_center_core_entity | 25 | 15 | 5 | 5 | 60.0% |
| control | 25 | 13 | 12 | 0 | 52.0% |
| organization_core_entity | 24 | 18 | 6 | 0 | 75.0% |
| ai | 21 | 16 | 5 | 0 | 76.2% |
| general_core_entity | 21 | 9 | 12 | 0 | 42.9% |
| correlation_service_processing | 20 | 19 | 1 | 0 | 95.0% |
| personal | 19 | 6 | 13 | 0 | 31.6% |
| bbs | 18 | 18 | 0 | 0 | 100.0% |
| bbs_assemble_control | 17 | 15 | 2 | 0 | 88.2% |
| bbs_core_entity | 15 | 10 | 5 | 0 | 66.7% |
| empower | 14 | 3 | 9 | 2 | 21.4% |
| processplatform_service_processing | 12 | 11 | 1 | 0 | 91.7% |
| cms_assemble_control | 12 | 10 | 2 | 0 | 83.3% |
| file_assemble_control | 12 | 9 | 3 | 0 | 75.0% |
| portal_assemble_designer | 12 | 9 | 3 | 0 | 75.0% |
| meeting_core_entity | 11 | 9 | 2 | 0 | 81.8% |
| shared | 11 | 0 | 0 | 11 | 0.0% |
| calendar | 10 | 10 | 0 | 0 | 100.0% |
| meeting | 10 | 10 | 0 | 0 | 100.0% |
| mind | 10 | 10 | 0 | 0 | 100.0% |
| component_assemble_control | 10 | 9 | 1 | 0 | 90.0% |
| mind_core_entity | 10 | 5 | 5 | 0 | 50.0% |
| calendar_core_entity | 10 | 4 | 6 | 0 | 40.0% |
| express | 10 | 1 | 9 | 0 | 10.0% |
| attendance | 9 | 9 | 0 | 0 | 100.0% |
| query_assemble_surface | 9 | 9 | 0 | 0 | 100.0% |
| file | 9 | 6 | 3 | 0 | 66.7% |
| mind_assemble_control | 8 | 8 | 0 | 0 | 100.0% |
| portal | 8 | 7 | 1 | 0 | 87.5% |
| portal_assemble_surface | 8 | 7 | 1 | 0 | 87.5% |
| attendance_core_entity | 8 | 2 | 6 | 0 | 25.0% |
| processplatform_assemble_surface | 7 | 7 | 0 | 0 | 100.0% |
| program_center | 7 | 7 | 0 | 0 | 100.0% |
| process_designer | 7 | 6 | 1 | 0 | 85.7% |
| file_core_entity | 7 | 5 | 2 | 0 | 71.4% |
| console | 7 | 4 | 3 | 0 | 57.1% |
| processplatform_core_entity | 6 | 6 | 0 | 0 | 100.0% |
| processplatform_core_express | 6 | 6 | 0 | 0 | 100.0% |
| jpush | 6 | 5 | 1 | 0 | 83.3% |
| processplatform_assemble_designer | 6 | 5 | 1 | 0 | 83.3% |
| cms_core_entity | 6 | 4 | 2 | 0 | 66.7% |
| personal_extend | 6 | 0 | 6 | 0 | 0.0% |
| message | 5 | 5 | 0 | 0 | 100.0% |
| processplatform_assemble_bam | 5 | 5 | 0 | 0 | 100.0% |
| query_assemble_designer | 5 | 5 | 0 | 0 | 100.0% |
| query_core_entity | 5 | 5 | 0 | 0 | 100.0% |
| hotpic_core_entity | 5 | 4 | 1 | 0 | 80.0% |
| jpush_core_entity | 5 | 4 | 1 | 0 | 80.0% |
| cms_express | 5 | 2 | 3 | 0 | 40.0% |
| calendar_assemble_control | 4 | 4 | 0 | 0 | 100.0% |
| query_service_processing | 4 | 4 | 0 | 0 | 100.0% |
| correlation_core_entity | 4 | 3 | 1 | 0 | 75.0% |
| organization_assemble_express | 4 | 3 | 1 | 0 | 75.0% |
| query_core_express | 4 | 3 | 1 | 0 | 75.0% |
| ai_core_entity | 3 | 3 | 0 | 0 | 100.0% |
| component | 3 | 3 | 0 | 0 | 100.0% |
| component_core_entity | 3 | 3 | 0 | 0 | 100.0% |
| correlation | 3 | 3 | 0 | 0 | 100.0% |
| message_core_entity | 3 | 3 | 0 | 0 | 100.0% |
| organization_core_express | 3 | 3 | 0 | 0 | 100.0% |
| portal_core_entity | 3 | 3 | 0 | 0 | 100.0% |
| process_bam | 3 | 3 | 0 | 0 | 100.0% |
| process_express | 3 | 3 | 0 | 0 | 100.0% |
| process_surface | 3 | 3 | 0 | 0 | 100.0% |
| query_service | 3 | 3 | 0 | 0 | 100.0% |
| general | 3 | 2 | 1 | 0 | 66.7% |
| hotpic | 3 | 2 | 1 | 0 | 66.7% |
| program_init | 3 | 2 | 1 | 0 | 66.7% |
| base | 3 | 1 | 2 | 0 | 33.3% |
| cms_control | 2 | 2 | 0 | 0 | 100.0% |
| cms_core_express | 2 | 2 | 0 | 0 | 100.0% |
| correlation_core_express | 2 | 2 | 0 | 0 | 100.0% |
| organization_assemble_authentication | 2 | 2 | 0 | 0 | 100.0% |
| organization_assemble_personal | 2 | 2 | 0 | 0 | 100.0% |
| query_express | 2 | 1 | 1 | 0 | 50.0% |
| mcp_server | 1 | 0 | 1 | 0 | 0.0% |
| **TOTAL** | **1101** | **890** | **193** | **18** | **80.8%** |

**Workspace total routed endpoints:** 1101 (across 85 crates with at least one route).

- **routed_touch_db:** 890
- **routed_shell (no DB):** 193
- **unknown (unresolved handlers):** 18
- **Overall routed-DB-touch rate:** 80.8% (routed_touch_db / routed_endpoints).
- Excluding the 18 unresolved handlers, the rate is 82.2% (890 / 1083).

**Notable gaps (Phase 2 shell/suspect crates):**
- `auth` (41 routed, 0% DB) and `personal_extend` (6 routed, 0%) and `shared` (11 routed, all unknown) — these are routed but show no DB access; `shared` is entirely unresolved (likely a shared-router assembly crate, handlers live elsewhere). `auth`/`personal_extend` are prime candidates for shell handlers.
- Low-rate core-entity / control crates worth a closer look: `express` (10.0%), `empower` (21.4%), `attendance_core_entity` (25.0%), `personal` (31.6%), `base` (33.3%), `general_core_entity` (42.9%), `calendar_core_entity` (40.0%), `cms_express` (40.0%), `control` (52.0%), `mind_core_entity` (50.0%).

_Generated by `scripts/measure_routed_db_touch.py` (no Rust source modified)._

**Caveat — heuristic blind spots:** the DB-touch test scans only the *handler
function body* for the listed literals. Handlers that delegate persistence to a
helper (e.g. `auth` calls `session_manager.create_session(...)` /
`bind_store().confirm(...)`; many crates call into a `*_store` or service layer)
are counted as `routed_shell` even though the DB write happens inside that
helper. Thus the true DB-backed rate is almost certainly **higher** than 80.8%,
and the 0% crates (`auth`, `personal_extend`) are most likely *indirect* DB
consumers rather than confirmed empty shells. Treat `routed_shell` as
"no DB access *visible in the handler body*" rather than "confirmed stub".
To get a definitive rate, re-run with handler bodies inlined/recursed into their
callees, or grep the store/service layer directly.
