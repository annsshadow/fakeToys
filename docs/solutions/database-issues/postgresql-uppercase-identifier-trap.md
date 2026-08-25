---
title: "PostgreSQL Uppercase Identifier Trap in SeaORM"
date: 2026-08-10
last_refreshed: 2026-08-25
category: database-issues
module: oa4rust/crates/orm
problem_type: database_issue
component: database
severity: high
symptoms:
  - "SeaORM entity queries fail with 'relation does not exist' for tables with uppercase names"
  - "PostgreSQL automatically lowercases unquoted identifiers"
  - "ORM entity definitions without explicit table_name produce wrong SQL"
root_cause: missing_include
resolution_type: migration
tags: [postgresql, seaorm, identifier, case-sensitivity, migration]
related_components:
  - oa4rust/migrations/011_normalize_schema.sql
  - oa4rust/crates/orm/src/entity.rs
---

# PostgreSQL Uppercase Identifier Trap in SeaORM

## Problem

PostgreSQL automatically lowercases unquoted identifiers. When the Java backend created tables with names like `FILE_FOLDER`, `AUTH_PERSON`, `PROCESS_WORK`, these names were stored in the database as lowercase `file_folder`, `auth_person`, `process_work`. SeaORM entity definitions that don't explicitly specify `table_name` will generate queries referencing the Rust struct name in snake_case — which may not match the actual table name.

## Symptoms

- Queries fail with `relation "file_folder" does not exist` when the entity is defined as `FileFolder` without explicit `table_name`
- Migration scripts reference uppercase names (`FILE_FOLDER`) but the actual table is lowercase (`file_folder`)
- The `#[sea_orm(table_name = "...")]` attribute is required on every entity

## Solution

### Step 1: Normalize schema with migration 011

Create a migration that renames all uppercase tables to lowercase snake_case:

```sql
-- migrations/011_normalize_schema.sql
-- Use DO $$ blocks for idempotent renaming (ALTER TABLE RENAME doesn't support IF EXISTS)
DO $$ BEGIN
  IF EXISTS (SELECT 1 FROM pg_class WHERE relname = 'FILE_FOLDER') THEN
    ALTER TABLE "FILE_FOLDER" RENAME TO "file_folder";
  END IF;
END $$;

DO $$ BEGIN
  IF EXISTS (SELECT 1 FROM pg_class WHERE relname = 'AUTH_PERSON') THEN
    ALTER TABLE "AUTH_PERSON" RENAME TO "auth_person";
  END IF;
END $$;

-- Index and constraint renaming
ALTER INDEX IF EXISTS "idx_file_folder_superior" RENAME TO "idx_file_folder_superior";
-- (all indexes and constraints follow same pattern)
```

### Step 2: Explicitly specify table_name in SeaORM entities

```rust
use sea_orm::entity::prelude::*;

#[derive(EntityTrait, DeriveEntity, Debug, Clone, DerefModel)]
#[sea_orm(table_name = "file_folder")]  // MUST match the actual PostgreSQL table name
pub struct Entity;

#[derive(ColTrait, EnumIter, DeriveColumn)]
pub enum Column {
    #[sea_orm(column_name = "id")]  // explicit column_name for snake_case mapping
    Id,
    #[sea_orm(column_name = "superior")]
    Superior,
}
```

### Step 3: Use the `orm_entity!` macro for consistency

The project defines an `orm_entity!` macro in `crates/orm/src/entity.rs` that encapsulates the common pattern:

```rust
// In each core_entity crate's entities/mod.rs
orm_entity! {
    name = FileFolder,
    table = "file_folder",
    columns {
        id: String,
        superior: Option<String>,
        name: String,
        deleted_at: Option<NaiveDateTime>,
    }
}
```

## Why This Works

- PostgreSQL's identifier case folding means `"FILE_FOLDER"` and `FILE_FOLDER` are the same table
- SeaORM's `table_name` attribute tells it the exact SQL to generate
- The `column_name` attribute handles the snake_case ↔ PascalCase mapping
- Migration 011 ensures the database schema matches what SeaORM expects

## Prevention

- Always run `SELECT tablename FROM pg_tables WHERE schemaname = 'public' ORDER BY tablename` after migrations to verify table names
- In new entities, always specify both `table_name` and `column_name` attributes
- The `audit_uppercase_tables.py` script scans all migration files and source code for uppercase table references
- Run `cargo test --workspace --lib` after any schema change to catch mismatches early

## Related

- [SeaORM Dual-Pool Coexistence](architecture-patterns/seaorm-dual-pool-coexistence.md)
- [Nested Tokio Runtime Panic](integration-issues/nested-tokio-runtime-panic.md)
- **Migration:** `oa4rust/migrations/011_normalize_schema.sql`
- **Audit script:** `oa4rust/scripts/audit_uppercase_tables.py`
- **Plan:** `docs/plans/2026-08-09-001-refact-oa4rust-orm-migration-plan.md` (U2)
