#!/usr/bin/env python3
# SPDX-License-Identifier: AGPL-3.0-or-later
"""O18 静态扫描：tokio-postgres Row 取值类型 vs SELECT 列的真实来源类型。

与 scan_entity_column_mismatch.py（sea-orm 实体侧）互补。本脚本不只看
migrations 的列类型，还要求取值点能关联到**最近的 SELECT 文本**，因此能区分：

  - 裸列 `SELECT level FROM x_org_unit` → 按 migration 列类型判定
  - 聚合别名 `SELECT COUNT(*) AS cnt` → 期望 i64（不是同名列的 VARCHAR）
  - 显式转型 `SELECT change_password_time::text` → 期望 String
  - 其他表达式别名 → 跳过（无法静态推断）

分类：
  FAIL   运行期 tokio-postgres FromSql 必然失败（500）
  LOOSE  同族加宽（用户口径：INTEGER 读 i64 算宽松），登记不修
  OK     匹配
（无法关联到 SELECT 的取值点不输出，避免把动态 SQL/辅助函数误报成缺陷。）

用法：
  python scripts/scan_row_get_column_mismatch.py            # TSV 明细
  python scripts/scan_row_get_column_mismatch.py --summary  # 按 crate 汇总
退出码：存在 FAIL 时为 1（可直接当门禁）。
"""
import re
import sys
from collections import Counter
from pathlib import Path

BACKEND = Path(__file__).resolve().parent.parent
MIGRATIONS = BACKEND / "migrations"
CRATES = BACKEND / "crates"

# ── 迁移侧：列类型（CREATE TABLE + ALTER ADD COLUMN，按表合并）──
COL_TYPE = {}
create_re = re.compile(r'CREATE TABLE (?:IF NOT EXISTS )?"?([A-Za-z0-9_]+)"?\s*\(', re.I)
alter_add_re = re.compile(
    r'ALTER TABLE\s+(?:IF EXISTS\s+)?"?([A-Za-z0-9_]+)"?\s+ADD COLUMN\s+'
    r'(?:IF NOT EXISTS\s+)?"?([A-Za-z0-9_]+)"?\s+([A-Z]+)',
    re.I,
)
# 后续类型变更（044 这类修复迁移会把列从 TEXT 改回 BOOLEAN），按迁移顺序覆盖。
alter_type_re = re.compile(
    r'ALTER TABLE\s+(?:IF EXISTS\s+)?"?([A-Za-z0-9_]+)"?\s+ALTER\s+COLUMN\s+'
    r'"?([A-Za-z0-9_]+)"?\s+(?:SET\s+DATA\s+)?TYPE\s+([A-Z]+)',
    re.I,
)

for sql_file in sorted(MIGRATIONS.glob("*.sql")):
    # *_rollback.sql 是回滚脚本，不参与正向迁移的类型累积（044 的回滚会把
    # cal_calendar.is_public 改回 TEXT，若纳入会得出与实际库相反的结论）。
    if "rollback" in sql_file.name.lower():
        continue
    text = sql_file.read_text(encoding="utf-8", errors="replace")
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
            if not line or line.startswith("--"):
                continue
            first = line.split()[0].strip('"').upper()
            if first in ("PRIMARY", "UNIQUE", "CONSTRAINT", "FOREIGN", "CHECK", ")"):
                continue
            lm = re.match(r'"?([A-Za-z0-9_]+)"?\s+(.+)$', line)
            if not lm:
                continue
            col = lm.group(1).lower()
            up = lm.group(2).strip().upper()
            typ = None
            for two in ("DOUBLE PRECISION", "CHARACTER VARYING",
                        "TIMESTAMP WITH TIME ZONE", "TIMESTAMP WITHOUT TIME ZONE"):
                if up.startswith(two):
                    typ = two
                    break
            if typ is None:
                typ = up.split()[0].split("(")[0]
            COL_TYPE[(table, col)] = typ
    for m in alter_add_re.finditer(text):
        COL_TYPE[(m.group(1).lower(), m.group(2).lower())] = m.group(3).upper()
    for m in alter_type_re.finditer(text):
        COL_TYPE[(m.group(1).lower(), m.group(2).lower())] = m.group(3).upper()

# ── 合理类型映射 ──
TEXT_LIKE = {"TEXT", "VARCHAR", "CHAR", "CHARACTER", "BPCHAR", "NAME", "CHARACTER VARYING"}
INT4_LIKE = {"INT", "INTEGER", "INT4", "SERIAL"}
INT8_LIKE = {"BIGINT", "INT8", "BIGSERIAL"}
INT2_LIKE = {"SMALLINT", "INT2"}
BOOL_LIKE = {"BOOL", "BOOLEAN"}
F32_LIKE = {"REAL", "FLOAT4"}
F64_LIKE = {"DOUBLE PRECISION", "FLOAT8", "DOUBLE"}
UUID_LIKE = {"UUID"}
JSON_LIKE = {"JSON", "JSONB"}
BYTEA_LIKE = {"BYTEA"}


def normalize_rust_type(raw: str) -> str:
    t = raw.strip()
    while t.startswith("Option<") and t.endswith(">"):
        t = t[len("Option<") : -1].strip()
    t = t.replace("&", "").strip()
    t = t.replace("serde_json::Value", "Value").replace("uuid::Uuid", "Uuid")
    t = t.replace("chrono::", "")
    return t


def classify_by_sql_type(rust_type: str, sql_type: str):
    """返回 OK / LOOSE / FAIL / None（未知类型不判定）。"""
    r = normalize_rust_type(rust_type)
    s = sql_type.upper()
    if r == "String":
        return "OK" if s in TEXT_LIKE else "FAIL"
    if r == "i32":
        return "OK" if s in INT4_LIKE else ("LOOSE" if s in INT2_LIKE or s in INT8_LIKE else "FAIL")
    if r == "i64":
        # 用户口径：INTEGER 读 i64 属宽松，不列为必然失败。
        return "OK" if s in INT8_LIKE else ("LOOSE" if s in INT4_LIKE or s in INT2_LIKE else "FAIL")
    if r == "i16":
        return "OK" if s in INT2_LIKE else ("LOOSE" if s in INT4_LIKE or s in INT8_LIKE else "FAIL")
    if r == "bool":
        return "OK" if s in BOOL_LIKE else "FAIL"
    if r == "f32":
        return "OK" if s in F32_LIKE else "FAIL"
    if r == "f64":
        return "OK" if s in F64_LIKE else ("LOOSE" if s in F32_LIKE else "FAIL")
    if r == "Value":
        return "OK" if s in JSON_LIKE else "FAIL"
    if r == "Uuid":
        return "OK" if s in UUID_LIKE else "FAIL"
    if r == "Vec<u8>":
        return "OK" if s in BYTEA_LIKE else "FAIL"
    if r in ("NaiveDateTime", "DateTime", "DateTimeUtc", "DateTimeWithTimeZone", "Timestamp"):
        return "OK" if "TIMESTAMP" in s else "FAIL"
    if r in ("NaiveDate", "Date"):
        return "OK" if s == "DATE" else "FAIL"
    if r in ("NaiveTime", "Time"):
        return "OK" if s == "TIME" else "FAIL"
    return None


# ── Rust 取值点 ──
turbofish_re = re.compile(
    r'\.(get|try_get)\s*::<\s*(?:_\s*,\s*)?(Option<[^>]+>|[A-Za-z0-9_:<>, ]+?)\s*>\s*'
    r'\(\s*"([A-Za-z0-9_]+)"\s*\)'
)
typed_let_re = re.compile(
    r'let\s+[a-z0-9_]+\s*:\s*(Option<[^>]+>|[A-Za-z0-9_:<>, ]+?)\s*=\s*\w+\s*\.'
    r'(get|try_get)\s*\(\s*"([A-Za-z0-9_]+)"\s*\)'
)
select_re = re.compile(r'\bSELECT\b', re.I)
from_re = re.compile(r'\bFROM\s+(?:ONLY\s+)?"?([A-Za-z0-9_]+)"?', re.I)
unknown_alias_re = re.compile(r'\bAS\s+"?([A-Za-z0-9_]+)"?', re.I)
count_alias_re = re.compile(r'COUNT\s*\([^)]*\)\s*(?:AS\s+)?"?([A-Za-z0-9_]+)"?', re.I)


def select_expectation(sql_select: str, col: str, table: str):
    """根据 SELECT 列表判断该列传给 Row 的期望类型。

    返回 (kind, value)：kind ∈ {"sqltype","rust","skip"}。
    """
    low = sql_select.lower()
    # COUNT(...) AS col（或 COUNT(*) 的隐式列名 count）→ i64
    m = count_alias_re.search(sql_select)
    if m and m.group(1).lower() == col:
        return ("rust", "i64")
    if col == "count" and "count(" in low:
        return ("rust", "i64")
    # col::text / CAST(col AS TEXT) → String
    cast_text = re.search(
        rf'\b{col}\b\s*::\s*(text|varchar|character varying)|CAST\s*\(\s*\b{col}\b\s+AS\s+(text|varchar)',
        low, re.I)
    if cast_text:
        return ("rust", "String")
    # 其他 AS col 别名（表达式结果）→ 静态无法判定
    for m in unknown_alias_re.finditer(sql_select):
        if m.group(1).lower() == col:
            return ("skip", None)
    # 裸列：选择列表中出现独立 word
    if re.search(rf'(?<![\w.])"?{col}"?(?![\w])', low):
        sql_type = COL_TYPE.get((table, col))
        if sql_type is None:
            return ("skip", None)
        return ("sqltype", sql_type)
    return ("skip", None)


rows = []
for rs in sorted(CRATES.glob("*/src/**/*.rs")):
    rel = rs.relative_to(BACKEND).as_posix()
    if "/tests/" in rel or rel.endswith("/tests.rs") or rel.endswith("_tests.rs"):
        continue
    text = rs.read_text(encoding="utf-8", errors="replace")
    line_of = lambda off: text.count("\n", 0, off) + 1  # noqa: E731

    calls = []
    for m in turbofish_re.finditer(text):
        calls.append((m.start(), m.group(2).strip(), m.group(3).lower(), m.group(1)))
    for m in typed_let_re.finditer(text):
        calls.append((m.start(), m.group(1).strip(), m.group(3).lower(), m.group(2)))

    # 每个 SELECT：找匹配的 FROM，以及 FROM 后的表名
    selects = []
    for m in select_re.finditer(text):
        f = from_re.search(text, m.end(), m.end() + 3000)
        if not f:
            continue
        selects.append((m.start(), f.start(), f.group(1).lower(), text[m.end():f.start()]))

    for pos, rust_type, col, method in calls:
        chosen = None
        for s_start, f_start, table, fields in selects:
            if s_start < pos and pos - f_start < 4000:
                chosen = (s_start, table, fields)
            elif s_start > pos:
                break
        if chosen is None:
            continue
        _, table, fields = chosen
        # 跳过 SELECT *
        if re.fullmatch(r'\s*\*\s*', fields, re.I):
            continue
        kind, value = select_expectation(fields, col, table)
        if kind == "skip":
            continue
        if kind == "rust":
            verdict = classify_by_sql_type(rust_type, "BIGINT" if value == "i64" else "TEXT")
            sql_type = value
        else:
            verdict = classify_by_sql_type(rust_type, value)
            sql_type = value
        if verdict is None:
            continue
        # try_get 的失败通常被 .ok()/unwrap_or 吞掉，不会 panic；单列为 SILENT，
        # 表示“字段恒为空/静默丢失”，不进入“必然失败”必修集。
        if verdict == "FAIL" and method == "try_get":
            verdict = "SILENT"
        rows.append((verdict, rel, str(line_of(pos)), col, rust_type, table, sql_type))

fails = [r for r in rows if r[0] == "FAIL"]
silents = [r for r in rows if r[0] == "SILENT"]
looses = [r for r in rows if r[0] == "LOOSE"]

if "--summary" in sys.argv:
    print(f"FAIL={len(fails)} SILENT={len(silents)} LOOSE={len(looses)}")
    for crate, n in Counter(r[1].split("/")[1] for r in fails).most_common():
        print(f"  {crate}: {n}")
else:
    print("VERDICT\tFILE\tLINE\tCOLUMN\tRUST_TYPE\tTABLE\tSQL_TYPE")
    for r in rows:
        print("\t".join(r))

sys.exit(1 if fails else 0)
