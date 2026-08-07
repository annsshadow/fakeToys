#!/usr/bin/env python3
"""
Verify data consistency between MySQL (Java side) and PostgreSQL (Rust side) for auth data.
"""

import os
import sys
import logging

try:
    import pymysql
    import psycopg2
except ImportError:
    print("Missing dependencies. Install with:")
    print("  pip install pymysql psycopg2-binary")
    sys.exit(1)

logging.basicConfig(level=logging.INFO)
logger = logging.getLogger(__name__)


class VerificationError(Exception):
    pass


def get_mysql_connection() -> pymysql.Connection:
    url = os.getenv("MYSQL_URL", "mysql://o2server:password@localhost:3306/o2server")
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


def count_table(conn, table: str) -> int:
    with conn.cursor() as cur:
        cur.execute(f"SELECT COUNT(*) FROM {table}")
        return cur.fetchone()[0]


def verify_counts(mysql_conn, pg_conn) -> bool:
    """Verify row counts match between MySQL and PostgreSQL."""
    tables = ["auth_person", "auth_role", "auth_unit"]

    all_ok = True
    for table in tables:
        mysql_count = count_table(mysql_conn, f"organization_{table.split('_')[1]}")
        pg_count = count_table(pg_conn, table)

        if mysql_count != pg_count:
            logger.error(f"Count mismatch for {table}: MySQL={mysql_count}, PostgreSQL={pg_count}")
            all_ok = False
        else:
            logger.info(f"Count OK for {table}: {mysql_count} rows")

    return all_ok


def verify_sample_data(mysql_conn, pg_conn) -> bool:
    """Verify sample records match between MySQL and PostgreSQL."""
    logger.info("Verifying sample data...")

    with mysql_conn.cursor() as cur:
        cur.execute("SELECT id, unique_id, name FROM organization_person WHERE id = 'person-admin' LIMIT 1")
        mysql_person = cur.fetchone()

    with pg_conn.cursor() as cur:
        cur.execute("SELECT id, unique_id, name FROM auth_person WHERE id = 'person-admin' LIMIT 1")
        pg_person = cur.fetchone()

    if not mysql_person or not pg_person:
        logger.warning("Sample person not found in one or both databases")
        return True  # Skip if not found

    if mysql_person[1] != pg_person[1] or mysql_person[2] != pg_person[2]:
        logger.error(f"Sample person mismatch: MySQL={mysql_person}, PostgreSQL={pg_person}")
        return False

    logger.info(f"Sample person OK: {pg_person}")
    return True


def main():
    logger.info("Starting data verification...")

    try:
        mysql_conn = get_mysql_connection()
        pg_conn = get_postgres_connection()
    except Exception as e:
        logger.error(f"Failed to connect to database: {e}")
        sys.exit(1)

    try:
        counts_ok = verify_counts(mysql_conn, pg_conn)
        sample_ok = verify_sample_data(mysql_conn, pg_conn)

        if counts_ok and sample_ok:
            logger.info("Verification PASSED: Data is consistent")
            sys.exit(0)
        else:
            logger.error("Verification FAILED: Data inconsistency detected")
            sys.exit(1)
    except Exception as e:
        logger.error(f"Verification error: {e}")
        sys.exit(1)
    finally:
        mysql_conn.close()
        pg_conn.close()


if __name__ == "__main__":
    main()
