---
title: "Strangler Fig Migration Pattern for OA4Rust"
date: 2026-08-10
category: architecture-patterns
module: oa4rust/
problem_type: architecture_pattern
component: development_workflow
severity: high
symptoms:
  - "Large Java monolith needs gradual replacement without downtime"
  - "Frontend must not break during backend migration"
  - "Need ability to roll back quickly if Rust implementation has issues"
root_cause: config_error
resolution_type: workflow_improvement
tags: [strangler-fig, migration, nginx, java-rust, gradual-migration]
related_components:
  - oa4rust/src/main.rs
  - oa/o2server/
  - oa/o2web/
applies_when:
  - "Migrating a large Java backend to Rust incrementally"
  - "Frontend must remain functional throughout migration"
  - "Need rollback capability at module granularity"
---

# Strangler Fig Migration Pattern for OA4Rust

## Context

The O2OA platform's Java backend (`o2server`, 57+ Maven modules) is being gradually replaced by a Rust monolith (`oa4rust`, 81 crates). The Strangler Fig pattern enables this migration without downtime: Rust and Java run in parallel, nginx routes requests to the appropriate backend based on URL prefix, and modules are switched one by one.

## Guidance

### Architecture

```
                    ┌─────────────┐
                    │    nginx    │
                    │ (router)    │
                    └──────┬──────┘
                           │
              ┌────────────┼────────────┐
              ▼            ▼            ▼
        /jaxrs/control  /jaxrs/auth   /jaxrs/cms
          → Rust         → Rust       → Java
        (port 3000)   (port 3000)   (port 20020)
```

### Step-by-Step Migration Flow

Each module follows a 4-step process, with rollback possible at each step:

1. **Data Migration** — Migrate data from MySQL (Java) to PostgreSQL (Rust)
2. **Rust Deployment** — Deploy and verify the Rust implementation
3. **Traffic Switch** — Update nginx config to route the module to Rust
4. **Observation** — Monitor for 30+ minutes, verify correctness

### Nginx Configuration

```nginx
# Rust-handled modules
location ~ ^/jaxrs/(control|auth|personal|program_init)/ {
    proxy_pass http://127.0.0.1:3000;
    proxy_set_header Host $host;
    proxy_set_header X-Real-IP $remote_addr;
}

# Java-handled modules (default)
location /jaxrs/ {
    proxy_pass http://127.0.0.1:20020;
    proxy_set_header Host $host;
}
```

### Rollback Procedure

Define clear rollback triggers and procedure:

| Trigger | Action | RTO |
|---------|--------|-----|
| Data corruption detected | Cut nginx back to Java | 5 minutes |
| Error rate > threshold | Cut nginx back to Java | 5 minutes |
| Performance degradation | Cut nginx back to Java | 5 minutes |
| Frontend incompatibility | Cut nginx back to Java | 5 minutes |

```bash
# Rollback script: switch all routes back to Java
# (modify nginx config to remove Rust location blocks)
nginx -s reload
```

### Key Decisions

1. **Session token interoperability** — Rust-issued sessions must be valid for Java modules and vice versa. Decision: share JWT signing key or keep login on Java side until auth module is fully migrated
2. **Concurrent write prevention** — During migration, disable Java writes to tables that Rust has taken over, using application-level feature flags or DB triggers
3. **Frontend contract preservation** — Rust must return the exact same JSON shape (`ActionResult<T>` 9 fields) as Java to avoid frontend changes

## Why This Matters

- Enables zero-downtime migration of a 57-module Java system
- Module-by-module switching limits blast radius of any single module's issues
- Rollback in 5 minutes provides safety net for production issues
- Frontend remains untouched — the contract is preserved at the API level

## When to Apply

- Migrating any large monolith to a new technology stack
- When zero-downtime is a hard requirement
- When the new implementation needs to coexist with the old during verification

## Examples

**Wave-based migration order (oa4rust):**
- Wave 0: Migration safety preparation (behavior comparison framework, rollback plan)
- Wave 1: Security hardening + 6 complete crates (auth, control, personal, personal_extend, message, program_init)
- Wave 2: File, calendar, attendance, general
- Wave 3: Meeting, portal, process, query, cms
- Wave 4: Remaining 8 crates with no database queries

## Related

- [Plan Document Lifecycle Management](development-workflow/plan-status-lifecycle.md)
- [SeaORM Dual-Pool Coexistence](architecture-patterns/seaorm-dual-pool-coexistence.md)
- [Single Source of Truth: Migration Status](best-practices/single-source-of-truth-migration-status.md)
- **Origin:** `docs/brainstorms/2026-08-05-oa4rust-comprehensive-advancement-requirements.md`
- **Plan:** `docs/plans/2026-08-07-001-feat-oa4rust-4wave-realization-plan.md`
- **Rollback plan:** `docs/plans/2026-08-10-001-prod-readiness-plan.md` (deferred: rollback playbook)
