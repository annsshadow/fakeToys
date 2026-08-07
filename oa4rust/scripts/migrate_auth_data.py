#!/usr/bin/env python3
"""
Migrate auth data from MySQL (Java side) to PostgreSQL (Rust side).
Uses INSERT ON CONFLICT for idempotent re-runs.
"""

import os
import sys
import logging
from typing import Dict, List, Any

try:
    import pymysql
    import psycopg2
    from psycopg2.extras import execute_values
except ImportError:
    print("Missing dependencies. Install with:")
    print("  pip install pymysql psycopg2-binary")
    sys.exit(1)

logging.basicConfig(level=logging.INFO)
logger = logging.getLogger(__name__)


class MigrationError(Exception):
    pass


def get_mysql_connection() -> pymysql.Connection:
    url = os.getenv("MYSQL_URL", "mysql://o2server:password@localhost:3306/o2server")
    # Parse URL (simple parsing)
    if url.startswith("mysql://"):
        url = url[8:]
    user_pass, host_port_db = url.split("@")
    user, password = user_pass.split(":")
    host_port, database = host_port_db.split("/")
    if ":" in host_port:
        host, port = host_port.split(":")
        port = int(port)
    else:
        host = host_port
        port = 3306

    return pymysql.connect(
        host=host,
        port=port,
        user=user,
        password=password,
        database=database,
        charset="utf8mb4",
    )


def get_postgres_connection() -> psycopg2.Connection:
    url = os.getenv("POSTGRES_URL", "postgres://o2server:password@localhost:5432/o2server_rust")
    return psycopg2.connect(url)


def migrate_persons(mysql_conn, pg_conn) -> int:
    """Migrate person data from MySQL to PostgreSQL."""
    logger.info("Migrating persons...")

    with mysql_conn.cursor() as cur:
        cur.execute("""
            SELECT id, unique_id, name, mobile, email, password, locked, failed_attempts
            FROM organization_person
            WHERE disable = FALSE
        """)
        persons = cur.fetchall()

    if not persons:
        logger.info("No persons found to migrate")
        return 0

    with pg_conn.cursor() as cur:
        execute_values(
            cur,
            """
            INSERT INTO auth_person (id, unique_id, name, mobile, password_hash, locked, failed_attempts)
            VALUES %s
            ON CONFLICT (id) DO UPDATE SET
                unique_id = EXCLUDED.unique_id,
                name = EXCLUDED.name,
                mobile = EXCLUDED.mobile,
                locked = EXCLUDED.locked,
                failed_attempts = EXCLUDED.failed_attempts
            """,
            [(p[0], p[1], p[2], p[3], p[4], p[6], p[7]) for p in persons],
        )

    pg_conn.commit()
    logger.info(f"Migrated {len(persons)} persons")
    return len(persons)


def migrate_roles(mysql_conn, pg_conn) -> int:
    """Migrate role data from MySQL to PostgreSQL."""
    logger.info("Migrating roles...")

    with mysql_conn.cursor() as cur:
        cur.execute("SELECT id, name, description FROM organization_role WHERE disable = FALSE")
        roles = cur.fetchall()

    if not roles:
        logger.info("No roles found to migrate")
        return 0

    with pg_conn.cursor() as cur:
        execute_values(
            cur,
            """
            INSERT INTO auth_role (id, name, description)
            VALUES %s
            ON CONFLICT (id) DO UPDATE SET
                name = EXCLUDED.name,
                description = EXCLUDED.description
            """,
            roles,
        )

    pg_conn.commit()
    logger.info(f"Migrated {len(roles)} roles")
    return len(roles)


def migrate_units(mysql_conn, pg_conn) -> int:
    """Migrate unit data from MySQL to PostgreSQL."""
    logger.info("Migrating units...")

    with mysql_conn.cursor() as cur:
        cur.execute("SELECT id, name, parent_id, level FROM organization_unit WHERE disable = FALSE")
        units = cur.fetchall()

    if not units:
        logger.info("No units found to migrate")
        return 0

    with pg_conn.cursor() as cur:
        execute_values(
            cur,
            """
            INSERT INTO auth_unit (id, name, parent_id, level)
            VALUES %s
            ON CONFLICT (id) DO UPDATE SET
                name = EXCLUDED.name,
                parent_id = EXCLUDED.parent_id,
                level = EXCLUDED.level
            """,
            units,
        )

    pg_conn.commit()
    logger.info(f"Migrated {len(units)} units")
    return len(units)


def main():
    logger.info("Starting auth data migration...")

    try:
        mysql_conn = get_mysql_connection()
        pg_conn = get_postgres_connection()
    except Exception as e:
        logger.error(f"Failed to connect to database: {e}")
        sys.exit(1)

    try:
        total = 0
        total += migrate_persons(mysql_conn, pg_conn)
        total += migrate_roles(mysql_conn, pg_conn)
        total += migrate_units(mysql_conn, pg_conn)

        logger.info(f"Migration completed: {total} total records migrated")
    except MigrationError as e:
        logger.error(f"Migration failed: {e}")
        pg_conn.rollback()
        sys.exit(1)
    except Exception as e:
        logger.error(f"Unexpected error: {e}")
        pg_conn.rollback()
        sys.exit(1)
    finally:
        mysql_conn.close()
        pg_conn.close()


if __name__ == "__main__":
    main()
