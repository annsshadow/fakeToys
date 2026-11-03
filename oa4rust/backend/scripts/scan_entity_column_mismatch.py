#!/usr/bin/env python3
# SPDX-License-Identifier: AGPL-3.0-or-later
"""O10 族①静态扫描：sea-orm 实体字段类型 vs migration 列类型。

解析 crates/*/src/entities/*.rs 的 DeriveEntityModel（table_name + 字段类型），
对比 migrations/*.sql 的 CREATE TABLE 列类型，列出会导致 insert RETURNING /
select 解码失败的不匹配点：
  - 实体 DateTime/DateTimeUtc/NaiveDateTime ↔ 列 TEXT（必炸：类型不兼容）
  - 实体 i32 ↔ 列 BIGINT（PG INT8 → i32 解码必炸）
  - 实体 String ↔ 列 BYTEA 等（其他明确不兼容）
用法：python scripts/scan_entity_column_mismatch.py  （输出 TSV：表 列 实体类型 列类型 实体文件）
"""
import re
import sys
from pathlib import Path

BACKEND = Path(__file__).resolve().parent.parent
MIGRATIONS = BACKEND / "migrations"

# ── 迁移侧：CREATE TABLE 列类型（含 ALTER ADD COLUMN；按表合并，后者覆盖）──
COL_TYPE = {}  # (table_lower, column_lower) -> sql type lower
create_re = re.compile(r'CREATE TABLE (?:IF NOT EXISTS )?"?([A-Za-z0-9_]+)"?\s*\(', re.I)
alter_add_re = re.compile(
    r'ALTER TABLE\s+(?:IF EXISTS\s+)?"?([A-Za-z0-9_]+)"?\s+ADD COLUMN\s+(?:IF NOT EXISTS\s+)?"?([A-Za-z0-9_]+)"?\s+([A-Z]+)',
    re.I)

for sql_file in sorted(MIGRATIONS.glob("*.sql")):
    text = sql_file.read_text(encoding="utf-8", errors="replace")
    # CREATE TABLE 块
    for m in create_re.finditer(text):
        table = m.group(1).lower()
        depth = 1
        i = m.end()
        while i < len(text) and depth > 0:
            ch = text[i]
            if ch == "(":
                depth += 1
            elif ch == ")":
                depth -= 1
            i += 1
        body = text[m.end():i - 1]
        for line in body.split("\n"):
            line = line.strip().rstrip(",")
            if not line:
                continue
            # 表约束行跳过（注意：不能用 startswith('CREATE')——会误杀
            # create_time/createor 列名（'CREATE_TIME...' 以 'CREATE' 开头），
            # 已实证漏报 file_file.create_time 的不匹配）
            first = line.split()[0].strip('"').upper()
            if first in ("PRIMARY", "UNIQUE", "CONSTRAINT", "FOREIGN", "CHECK", ")", "INDEX"):
                continue
            lm = re.match(r'"?([A-Za-z0-9_]+)"?\s+(.+)$', line)
            if not lm:
                continue
            col = lm.group(1).lower()
            rest = lm.group(2).strip()
            # 类型短语：双词类型合并（DOUBLE PRECISION / CHARACTER VARYING /
            # TIMESTAMP [WITH|WITHOUT] TIME ZONE），其余取第一个 token（去括号）
            up = rest.upper()
            for two in ("DOUBLE PRECISION", "CHARACTER VARYING",
                        "TIMESTAMP WITH TIME ZONE", "TIMESTAMP WITHOUT TIME ZONE"):
                if up.startswith(two):
                    typ = two
                    break
            else:
                typ = up.split()[0].split("(")[0]
            COL_TYPE[(table, col)] = typ
    for m in alter_add_re.finditer(text):
        COL_TYPE[(m.group(1).lower(), m.group(2).lower())] = m.group(3).upper()

# ── 实体侧：DeriveEntityModel 字段类型 ──
entity_re = re.compile(r'#\[sea_orm\(table_name\s*=\s*"([A-Za-z0-9_]+)"\)\]')
# 字段 = 可选的紧邻 sea_orm 属性块 + pub 字段（属性块与字段间只允许空白）
field_re = re.compile(
    r'(?:#\[sea_orm\(([^\]]*)\)\]\s*)?pub\s+([a-z0-9_]+)\s*:\s*([^,]+),')
colname_re = re.compile(r'column_name\s*=\s*"([A-Za-z0-9_]+)"')

# Rust 类型 → 兼容的 SQL 类型族
def compat(rust: str, sql: str) -> bool:
    r = rust.replace("Option<", "").replace(">", "").strip()
    s = sql.upper()
    if r in ("String", "Value", "Json", "serde_json::Value"):
        return s in ("TEXT", "VARCHAR", "CHAR", "CHARACTER", "CHARACTER VARYING",
                     "JSONB", "JSON", "UUID", "BYTEA") or s.startswith(("VARCHAR", "CHAR"))
    if r in ("i32",):
        return s in ("INT", "INTEGER", "INT4", "SMALLINT", "INT2")
    if r in ("i64",):
        return s in ("BIGINT", "INT8", "INT", "INTEGER", "INT4")
    if r in ("i16",):
        return s in ("SMALLINT", "INT2", "INT", "INTEGER", "INT4", "BIGINT", "INT8")
    if r in ("bool",):
        return s in ("BOOL", "BOOLEAN")
    if r in ("f32",):
        return s in ("REAL", "FLOAT4")
    if r in ("f64",):
        return s in ("DOUBLE PRECISION", "FLOAT8", "DOUBLE")
    if r in ("DateTime", "NaiveDateTime", "DateTimeUtc", "DateTimeWithTimeZone"):
        return "TIMESTAMP" in s or "TIMESTAMPTZ" in s
    if r in ("NaiveDate", "Date"):
        return s == "DATE"
    if r in ("NaiveTime", "Time"):
        return s == "TIME"
    if r in ("Uuid", "uuid::Uuid"):
        return s in ("UUID", "TEXT")
    if r in ("Vec<u8>",):
        return s == "BYTEA"
    if r in ("Decimal", "rust_decimal::Decimal", "BigDecimal"):
        return s in ("NUMERIC", "DECIMAL", "BIGINT", "INT8")
    return True  # 未知类型不报警（避免误报）

mismatches = []
for ent in sorted(BACKEND.glob("crates/*/src/entities/*.rs")):
    text = ent.read_text(encoding="utf-8", errors="replace")
    tm = entity_re.search(text)
    if not tm:
        continue
    table = tm.group(1).lower()
    # 逐字段扫：紧邻字段的 #[sea_orm(...)] 属性块里的 column_name 优先
    for fm in field_re.finditer(text):
        attr = fm.group(1) or ""
        col = fm.group(2).lower()
        rust = fm.group(3).strip()
        nm = colname_re.search(attr)
        if nm:
            col = nm.group(1).lower()
        sql = COL_TYPE.get((table, col))
        if sql is None:
            # 列不在任何迁移里——表缺失或列缺失（另一族问题）
            if "--debug" in sys.argv:
                print(f"DEBUG {table}.{col}: keys={sorted(k for k in COL_TYPE if k[0] == table)}", file=sys.stderr)
            mismatches.append((table, col, rust, "<列不在迁移中>", str(ent.relative_to(BACKEND))))
            continue
        if not compat(rust, sql):
            mismatches.append((table, col, rust, sql, str(ent.relative_to(BACKEND))))

print(f"{'TABLE'}\t{'COLUMN'}\t{'ENTITY_TYPE'}\t{'COLUMN_TYPE'}\tFILE")
for row in mismatches:
    print("\t".join(row))
print(f"# total mismatches: {len(mismatches)}", file=sys.stderr)
