---
title: "Single Source of Truth: Migration Status Tracking"
date: 2026-08-10
category: best-practices
module: docs/brainstorms
problem_type: best_practice
component: documentation
severity: medium
symptoms:
  - "Multiple migration status files with conflicting information"
  - "Team members unsure which crate status is current"
  - "Plan documents reference stale status counts"
root_cause: inadequate_documentation
resolution_type: documentation_update
tags: [migration-status, single-source-of-truth, documentation, tracking]
related_components:
  - docs/brainstorms/oa4rust-migration-status-2026-08-08.md
  - docs/plans/
applies_when:
  - "Managing a large-scale technology migration (Java → Rust, SQLx → SeaORM)"
  - "Multiple team members or sessions contribute to migration progress"
  - "Status needs to be queryable and machine-parseable"
---

# Single Source of Truth: Migration Status Tracking

## Context

The oa4rust migration involved 81 crates transitioning from SQLx to SeaORM over multiple sprint days. Without a single authoritative status document, team members (and AI agents) would need to inspect each crate's source code to determine its current state — a brittle and error-prone process.

## Guidance

### The Migration Status Document

`docs/brainstorms/oa4rust-migration-status-2026-08-08.md` serves as the single source of truth for migration progress. It contains:

1. **Header with metadata**: update date, total crate count, reference to the origin requirements
2. **Status legend**: defines what each status means
3. **Summary table**: counts by status category
4. **Per-crate table**: each crate's Java module mapping, router count, and core endpoint description

### Status Values

| Status | Meaning |
|--------|---------|
| 已完成（真实化） | All endpoints connected to PostgreSQL with real business logic, no stubs |
| 部分真实化 | Contains real PostgreSQL queries, but some endpoints are stubs or unmounted |
| 已接入（桩代码） | Routes registered, but endpoints return empty lists / `Value::Null` / mock data |
| 无数据库查询 | Routes registered with handler framework, but no PostgreSQL query calls in handlers |

### Update Procedure

After each sprint, update the status document:

1. **Update the header date** to today
2. **Verify crate count**: run `Get-ChildItem crates -Directory` in the `oa4rust` directory and compare with the table
3. **Update status columns**: only change status when actually verified complete (stubs → real implementation → verified via `cargo test`)
4. **Update route counts**: periodically re-count registered routes per crate
5. **If code and document disagree**: update the document first, then investigate the code

### Verification Commands

```bash
# Check crate count matches the status document
cd oa4rust && Get-ChildItem crates -Directory | Measure-Object | Select-Object -ExpandProperty Count

# Verify all tests pass (proof of real implementation)
cargo test --workspace --lib

# Check for remaining stub patterns
Select-String -Path "crates/*/src/*.rs" -Pattern "ActionResult::success\(Value::Null\)" | Select-Object -First 10
Select-String -Path "crates/*/src/*.rs" -Pattern "assert!\(true\)" | Select-Object -First 10
```

## Why This Matters

- A single document eliminates confusion about which plan or status is current
- Machine-parseable format (Markdown table) enables automated checks
- The document references the origin requirements, creating a traceability chain
- When the document says "81 crates completed" but code disagrees, the document wins — this forces investigation rather than silent drift

## When to Apply

- Any multi-crate or multi-module migration project
- Projects where progress is tracked across multiple sessions or sprints
- When AI agents or multiple contributors need to understand current state

## Examples

**Header:**
```markdown
# OA4Rust 迁移进度跟踪清单

**更新时间：** 2026-08-10
**Workspace crate 总数：** 81（含 shared 基础设施 crate）
**参照需求：** docs/brainstorms/2026-08-05-oa4rust-comprehensive-advancement-requirements.md
```

**Summary table:**
```markdown
| 维度 | 数量 |
|------|------|
| 已完成（真实化） | 81 个 |
| 无数据库查询 | 0 个 |
| 已接入（桩代码） | 0 个 |
| 路由注册总数 | 7,624 个 |
| 测试状态 | cargo test --workspace --lib 全部通过 |
```

## Related

- [Plan Document Lifecycle Management](development-workflow/plan-status-lifecycle.md)
- [SeaORM Dual-Pool Coexistence](architecture-patterns/seaorm-dual-pool-coexistence.md)
- **Current doc:** `docs/brainstorms/oa4rust-migration-status-2026-08-08.md`
- **Superseded:** `docs/brainstorms/oa4rust-migration-status.md` (2026-08-07)
- **Plan:** `docs/plans/2026-08-07-001-feat-oa4rust-4wave-realization-plan.md` (R50)
