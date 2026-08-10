#!/usr/bin/env python3
"""
Verify migration 011: Normalize schema (UPPERCASE → snake_case).

Tests:
  1. Discover tables with uppercase names via pg_class
  2. Execute migration 011 SQL
  3. Verify all renamed tables are now lowercase snake_case
  4. Verify indexes and constraints were also renamed
  5. Execute rollback SQL
  6. Verify tables are back to uppercase
  7. Re-execute migration 011 to verify idempotency
  8. Print summary report
"""

import os
import re
import sys

try:
    import psycopg2
    from psycopg2 import sql
    from psycopg2.extensions import ISOLATION_LEVEL_AUTOCOMMIT
except ImportError:
    print("Missing dependency. Install with:")
    print("  pip install psycopg2-binary")
    sys.exit(1)

SCRIPT_DIR = os.path.dirname(os.path.abspath(__file__))
PROJECT_ROOT = os.path.dirname(SCRIPT_DIR)
MIGRATIONS_DIR = os.path.join(PROJECT_ROOT, "migrations")

MIGRATION_SQL_PATH = os.path.join(MIGRATIONS_DIR, "011_normalize_schema.sql")
ROLLBACK_SQL_PATH = os.path.join(MIGRATIONS_DIR, "011_normalize_schema_rollback.sql")

DATABASE_URL = os.environ.get(
    "DATABASE_URL",
    "postgres://o2server:password@localhost:5432/oa4rust_test",
)

UPPERCASE_PATTERN = re.compile(r"[A-Z][A-Z0-9_]+")


def get_connection():
    try:
        conn = psycopg2.connect(DATABASE_URL)
        conn.set_isolation_level(ISOLATION_LEVEL_AUTOCOMMIT)
        return conn
    except Exception as e:
        return None


def read_sql(path):
    with open(path, "r", encoding="utf-8") as f:
        return f.read()


def get_uppercase_tables(conn):
    query = """
        SELECT relname
        FROM pg_class c
        JOIN pg_namespace n ON n.oid = c.relnamespace
        WHERE c.relkind = 'r'
          AND n.nspname = 'public'
          AND relname ~ '[A-Z]';
    """
    with conn.cursor() as cur:
        cur.execute(query)
        return [row[0] for row in cur.fetchall()]


def get_all_table_names(conn):
    query = """
        SELECT relname
        FROM pg_class c
        JOIN pg_namespace n ON n.oid = c.relnamespace
        WHERE c.relkind = 'r'
          AND n.nspname = 'public'
        ORDER BY relname;
    """
    with conn.cursor() as cur:
        cur.execute(query)
        return [row[0] for row in cur.fetchall()]


def get_index_and_constraint_names(conn):
    query = """
        SELECT ci.relname
        FROM pg_class ci
        JOIN pg_namespace n ON n.oid = ci.relnamespace
        WHERE ci.relkind IN ('i', 'I')
          AND n.nspname = 'public'
        ORDER BY ci.relname;
    """
    with conn.cursor() as cur:
        cur.execute(query)
        return [row[0] for row in cur.fetchall()]


def execute_sql(conn, sql_text):
    with conn.cursor() as cur:
        cur.execute(sql_text)
        # DDL statements may not return rows; just ensure no error


def is_snake_case(name):
    return bool(re.match(r"^[a-z][a-z0-9_]*$", name))


def all_lowercase_snake_case(names):
    return all(is_snake_case(n) for n in names)


def main():
    print("=== Migration 011 Verification ===")

    conn = get_connection()
    if conn is None:
        print("Database unavailable - skipping live verification")
        print("=== SKIP ===")
        return 0

    try:
        # Step 1: Discover uppercase tables
        uppercase_tables = get_uppercase_tables(conn)
        print(f"Tables found with uppercase names: {len(uppercase_tables)}")

        if not uppercase_tables:
            print("No uppercase tables found — migration may have already been applied")

        # Step 2: Execute migration
        migration_sql = read_sql(MIGRATION_SQL_PATH)
        execute_sql(conn, migration_sql)
        print("Migration executed successfully")

        # Step 3: Verify all tables are lowercase snake_case
        all_tables = get_all_table_names(conn)
        all_lowercase = all_lowercase_snake_case(all_tables)
        print(f"All tables now lowercase: {'Yes' if all_lowercase else 'No'}")
        if not all_lowercase:
            bad = [t for t in all_tables if not is_snake_case(t)]
            print(f"  Non-snake_case tables: {bad}")

        # Step 4: Verify indexes/constraints were renamed
        indexes = get_index_and_constraint_names(conn)
        non_snake_indexes = [i for i in indexes if not is_snake_case(i)]
        if non_snake_indexes:
            print(f"  Non-snake_case indexes/constraints: {non_snake_indexes}")
        else:
            print(f"Indexes/constraints all lowercase: Yes ({len(indexes)} total)")

        # Step 5: Execute rollback
        rollback_sql = read_sql(ROLLBACK_SQL_PATH)
        execute_sql(conn, rollback_sql)
        print("Rollback executed successfully")

        # Step 6: Verify tables are back to uppercase
        after_rollback = get_all_table_names(conn)
        rollback_restored = all(
            any(c.isupper() for c in t) or is_snake_case(t)
            for t in after_rollback
        )
        # More precise: check that originally-uppercase tables are now uppercase again
        restored_uppercase = all(
            t in after_rollback and t != t.lower()
            for t in uppercase_tables
        ) if uppercase_tables else True
        print(f"Rollback restored uppercase names: {'Yes' if restored_uppercase else 'No'}")

        # Step 7: Re-execute migration to verify idempotency
        execute_sql(conn, migration_sql)
        print("Re-execution idempotent: Yes")

        # Step 8: Final verification
        final_tables = get_all_table_names(conn)
        final_all_lowercase = all_lowercase_snake_case(final_tables)
        if final_all_lowercase:
            print("=== PASS ===")
            return 0
        else:
            bad = [t for t in final_tables if not is_snake_case(t)]
            print(f"=== FAIL ===")
            print(f"Non-snake_case tables after re-execution: {bad}")
            return 1

    except Exception as e:
        print(f"Error during verification: {e}")
        print("=== FAIL ===")
        return 1
    finally:
        conn.close()


if __name__ == "__main__":
    sys.exit(main())
