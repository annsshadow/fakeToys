---
title: "Plan Document Lifecycle Management"
date: 2026-08-10
category: development-workflow
module: docs/plans
problem_type: workflow_issue
component: development_workflow
severity: medium
symptoms:
  - "Completed plans still marked status: active, creating confusion about which plan is current"
  - "Superseded plans not annotated with what replaced them"
  - "No status field on some completed plans (e.g., zero-secret-migration-plan)"
root_cause: inadequate_documentation
resolution_type: documentation_update
tags: [plan-lifecycle, status, documentation, frontmatter]
applies_when:
  - "Managing multiple sequential implementation plans in docs/plans/"
  - "Plans are replaced by newer, more detailed plans"
  - "Need to distinguish active work from completed or obsolete plans"
---

# Plan Document Lifecycle Management

## Context

The `docs/plans/` directory accumulated 11 plan documents over 10 days of sprint work. By 2026-08-10, 5 plans had stale status markers: 3 still marked `active` despite being fully replaced by later plans, and 2 completed plans not updated. This created confusion about which plan was the source of truth.

## Guidance

### Status Values

| Status | Meaning | When to Use |
|--------|---------|-------------|
| `active` | Currently in progress or the authoritative plan | New plans start here |
| `completed` | All implementation units finished, verified | Work is done, tests pass |
| `superseded` | Replaced by a newer plan | A later plan covers the same scope with more detail |

### Superseded Plans

When a plan is superseded, add an HTML comment in the frontmatter:

```yaml
status: superseded
<!-- Superseded by: docs/plans/2026-08-07-001-feat-oa4rust-4wave-realization-plan.md -->
```

### Rules

1. **Every new plan starts with `status: active`**
2. **When a plan's work is done, update to `status: completed` in the same commit that finishes the work**
3. **When a plan is replaced by a newer one, update to `superseded` with the superseding plan's path**
4. **Never delete plan files** — keep them for historical context and audit trail
5. **Periodic audit**: `grep -r "status: active" docs/plans/` should only match currently-in-progress plans

### Audit Command

```bash
# Check for stale active plans
grep -r "status: active" docs/plans/

# Should only return plans that are actually in progress
# If a plan is marked active but all U1-Un are verified complete, update it
```

## Why This Matters

- Prevents confusion about which plan is the current source of truth
- Creates an auditable chain of custody for planning decisions
- Enables automated tracking of plan state (e.g., CI checks for stale active plans)
- Preserves historical context without cluttering the active plan set

## When to Apply

- At the end of each sprint, audit plan statuses
- When creating a new plan that covers the same scope as an existing one
- When a plan's implementation is verified complete

## Examples

**Completed plan:**
```yaml
---
title: prod-readiness: plan cleanup, write ops, and docs completion
type: refactor
status: completed
date: 2026-08-10
---
```

**Superseded plan:**
```yaml
---
title: feat: OA4Rust full realization (80 crates to production)
type: feat
status: superseded
date: 2026-08-06
<!-- Superseded by: docs/plans/2026-08-07-001-feat-oa4rust-4wave-realization-plan.md -->
---
```

## Related

- [Single Source of Truth: Migration Status](best-practices/single-source-of-truth-migration-status.md)
- [Strangler Fig Migration Pattern](architecture-patterns/strangler-fig-migration.md)
- **Audit command:** `grep -r "status: active" docs/plans/`
- **Fix commit:** `46c4f51e` — refactor(plans): mark completed/superseded plans
- **Done in:** `docs/plans/2026-08-10-001-prod-readiness-plan.md` (U1)
