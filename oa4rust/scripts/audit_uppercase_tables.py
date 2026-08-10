#!/usr/bin/env python3
"""扫描 oa4rust 代码库中的所有大写表名引用。"""
import os
import re
import sys

ROOT = os.path.dirname(os.path.abspath(__file__))
CRATES_DIR = os.path.join(ROOT, '..', 'crates')
MIGRATIONS_DIR = os.path.join(ROOT, '..', 'migrations')

# 匹配大写表名（全大写或混合大小写但包含大写）
UPPERCASE_TABLE_PATTERN = re.compile(r'(?:FROM|INTO|TABLE|UPDATE|JOIN|RENAME\s+TO)\s+["\']?([A-Z][A-Z0-9_]+)["\']?', re.IGNORECASE)
# 匹配完整的表名（大写字母+下划线+数字，至少2个字符）
TABLE_NAME_PATTERN = re.compile(r'\b([A-Z][A-Z0-9_]{2,})\b')

def scan_file(path):
    """扫描单个文件，返回大写表名列表。"""
    tables = set()
    try:
        with open(path, 'r', encoding='utf-8', errors='replace') as f:
            content = f.read()
            for match in TABLE_NAME_PATTERN.finditer(content):
                table = match.group(1)
                # 过滤掉明显的非表名（如 SQL 关键字、Rust 类型等）
                skip_patterns = [
                    'SELECT', 'INSERT', 'UPDATE', 'DELETE', 'FROM', 'WHERE',
                    'AND', 'OR', 'NOT', 'NULL', 'TRUE', 'FALSE',
                    'CREATE', 'ALTER', 'DROP', 'INDEX', 'TABLE', 'JOIN',
                    'ORDER', 'BY', 'GROUP', 'HAVING', 'LIMIT', 'OFFSET',
                    'INTEGER', 'VARCHAR', 'TEXT', 'BOOLEAN', 'TIMESTAMP',
                    'PRIMARY', 'KEY', 'FOREIGN', 'REFERENCES', 'DEFAULT',
                    'UNIQUE', 'CHECK', 'CONSTRAINT',
                    'IF', 'EXISTS', 'THEN', 'ELSE', 'END',
                    'async', 'await', 'use', 'pub', 'fn', 'let', 'mut',
                    'String', 'Vec', 'Option', 'Result', 'HashMap',
                    'serde', 'json', 'axum', 'tokio', 'sqlx', 'sea',
                    'Debug', 'Clone', 'Copy', 'PartialEq', 'Eq',
                    'Serialize', 'Deserialize', 'Into', 'From',
                ]
                if table not in skip_patterns and not table.startswith('_'):
                    tables.add(table)
    except Exception as e:
        print(f"Warning: Failed to read {path}: {e}", file=sys.stderr)
    return tables

def main():
    all_tables = set()
    file_counts = {}

    # 扫描 crates 目录
    for root, dirs, files in os.walk(CRATES_DIR):
        for f in files:
            if f.endswith('.rs'):
                path = os.path.join(root, f)
                tables = scan_file(path)
                if tables:
                    file_counts[path] = tables
                    all_tables.update(tables)

    # 扫描 migrations 目录
    for f in os.listdir(MIGRATIONS_DIR):
        if f.endswith('.sql'):
            path = os.path.join(MIGRATIONS_DIR, f)
            tables = scan_file(path)
            if tables:
                file_counts[path] = tables
                all_tables.update(tables)

    print(f"Found {len(all_tables)} unique uppercase table references:")
    for table in sorted(all_tables):
        print(f"  {table}")

    print(f"\nFiles with uppercase table references: {len(file_counts)}")
    for path, tables in sorted(file_counts.items()):
        print(f"  {path}: {sorted(tables)}")

    return all_tables

if __name__ == '__main__':
    tables = main()
    # 输出为 migration 脚本提供输入
    print(f"\n-- TABLES_TO_RENAME={','.join(sorted(tables))}")
