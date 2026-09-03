# OA4Rust Routing Completion Plan
## Date: 2026-09-03

## Current Status Summary

| Module | Java Handlers | Rust Routes | Status |
|--------|--------------|-------------|--------|
| x_organization_assemble_authentication | 53 | 154 | OK 100% |
| x_file_assemble_control | 105 | 565 | OK 100% |
| x_jpush_assemble_control | 9 | 75 | OK 100% |
| x_component_assemble_control | 7 | 45 | OK 100% |
| x_calendar_assemble_control | 31 | 102 | OK 100% |
| x_portal_assemble_designer | 64 | 74 | NEEDS ROUTES |
| x_query_service_processing | 24 | 28 | NEEDS JAVA_BASE |
| x_processplatform_assemble_designer | 118 | 114 | NEEDS JAVA_BASE |
| x_processplatform_service_processing | 127 | 200 | NEEDS JAVA_BASE |
| x_organization_assemble_personal | 76 | 3 | NEEDS 73 ROUTES |

**Total**: Rust=2064 routes, Java=614 handlers, Coverage=80.3%

---

## P0: organization_assemble_personal (73 routes missing)

**File**: crates/organization_assemble_personal/src/lib.rs

**Current**: Only 3 routes implemented

### Missing Routes by Category

1. custom (8 routes)
2. definition (6 routes)  
3. empower (21 routes)
4. empowerlog (8 routes)
5. exmail (6 routes)
6. person (9 routes)
7. icon (1 route)
8. regist (7 routes)
9. reset (6 routes)
10. signature (4 routes)

See scripts/all_java_handlers.json for full list.

---

## P1: Add JAVA_BASE Constants (2 modules)

**Problem**: audit_v52.js needs pub const JAVA_BASE to calculate coverage

### 1. processplatform_service_processing
**File**: crates/processplatform_service_processing/src/lib.rs
**Add**: pub const JAVA_BASE: &str = /jaxrs/processplatform/service/processing;
**Routes**: 200 total, **Java**: 127 handlers

### 2. processplatform_assemble_designer
**File**: crates/processplatform_assemble_designer/src/lib.rs
**Add**: pub const JAVA_BASE: &str = /jaxrs/processplatform/assemble/designer;
**Routes**: 114 total, **Java**: 118 handlers

---

## P2: Fix audit_v52.js Multi-line format!() Parsing

**File**: scripts/audit_v52.js
**Problem**: extractFormatRoutes() only matches single-line format!() calls
**Impact**: query_service_processing has 14 format!() routes not counted

---

## Implementation Steps

1. P0: organization_assemble_personal (1-2 hours)
2. P1: Add JAVA_BASE constants (5 minutes)
3. P2: Fix audit script (optional, 30 minutes)

---

## Verification

1. cargo check --workspace
2. cargo test --workspace  
3. node scripts/audit_v52.js shows 100%

---

*Generated: 2026-09-03*
