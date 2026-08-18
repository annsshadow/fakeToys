# Migration Archive

This directory contains migration files that were archived due to numbering conflicts.

## Archived Files

### 008_cleanup_duplicates.sql
- **Reason:** Numbering conflict with 008_file_tables.sql
- **Content preserved:** Documents that 003 and 004 were duplicates moved to archive
- **Replacement:** N/A (this was a no-op migration)

### 009_correlation_tables.sql
- **Reason:** Numbering conflict with 009_person_group_tables.sql
- **Content preserved:** Creates x_correlation and CORR_C_CORRELATION tables
- **Replacement:** 009_person_group_tables.sql handles auth_person_group; correlation tables may need re-numbering

## Current Migration Order

1. 001_create_auth_tables.sql
2. 002_seed_auth_data.sql
3. 005_org_tables.sql
4. 006_meeting_tables.sql
5. 006_org_updated_at.sql
6. 007_admin_seed.sql
7. 007_bbs_tables.sql
8. 007_secret_config.sql
9. 008_file_tables.sql
10. 009_person_group_tables.sql

## Notes

- Archive files are kept for historical reference and rollback purposes
- New migrations should use the next available number (010, 011, etc.)
- When applying migrations to new environments, only use files in the root migrations/ directory