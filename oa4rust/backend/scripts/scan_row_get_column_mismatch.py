#!/usr/bin/env python3
# SPDX-License-Identifier: AGPL-3.0-or-later
"""O18/O36-D 静态扫描：tokio-postgres Row 取值类型 + 可空性 vs 实库列。

与 scan_entity_column_mismatch.py（sea-orm 实体侧）互补。本脚本不只看
migrations 的列类型，还要求取值点能关联到**最近的 SELECT 文本**，因此能区分：

  - 裸列 `SELECT level FROM x_org_unit` → 按 migration 列类型判定
  - 聚合别名 `SELECT COUNT(*) AS cnt` → 期望 i64（不是同名列的 VARCHAR）
  - 显式转型 `SELECT change_password_time::text` → 期望 String，可空性继承源列
  - `to_char(col, fmt)` → 期望 String，可空性继承源列（NULL 仍返回 NULL）
  - `COALESCE(col, ...)` → 非空
  - 其他表达式别名 → 跳过（无法静态推断）

分类：
  FAIL     运行期 tokio-postgres FromSql 必然失败（类型不匹配，500）
  NULLRISK 列可空 + Rust 侧非 Option：**该列出现 NULL 行时必然 panic → 500**
  LOOSE    同族加宽（用户口径：INTEGER 读 i64 算宽松），登记不修
  SILENT   try_get + 非 Option：失败被吞，字段静默为空
  OK       匹配
（无法关联到 SELECT 的取值点不输出，避免把动态 SQL/辅助函数误报成缺陷。）

可空性来源：migrations 正向累积（跳过 *_rollback.sql）——
CREATE TABLE 列（无 NOT NULL / PRIMARY KEY / SERIAL 视为可空）、
ALTER TABLE ADD COLUMN（无 NOT NULL 视为可空）、
ALTER COLUMN SET/DROP NOT NULL、ALTER TABLE RENAME TO（011 的 FILE_FILE→file_file）。

静态判定不了、因此不输出（各自由什么兜底写在 docs/plans 的 O36-D 结论里）：
  - `SELECT *`（列清单不在代码里）
  - `SELECT {var}` / format! 插值列（运行期才拼出）
  - CTE / 视图 / information_schema 动态列（FROM 不是基表）
  - 一条 SQL 里同名列多表歧义（本脚本只取最近一个 FROM）
  - try_get 吞错（归 SILENT，不列入 NULLRISK 必修）
  - SeaORM 实体（走 scan_entity_column_mismatch.py）
  - 裸 `Value::String(row.get("col"))` 已覆盖；`row.get::<_, T>` 的 T 才能拿到

NULLRISK 与基线 `docs/audits/artifacts/column_nullability_baseline.tsv` 做棘轮：
新增条目 → 红；修掉的条目 → 提示收敛（不阻断，避免与并发修复 PR 冲突）。

用法：
  python scripts/scan_row_get_column_mismatch.py                 # TSV 明细
  python scripts/scan_row_get_column_mismatch.py --summary       # 按 crate 汇总
  python scripts/scan_row_get_column_mismatch.py --update-baseline
  python scripts/scan_row_get_column_mismatch.py --self-test     # 注入用例自检
退出码：存在 FAIL、或存在基线外 NULLRISK 时为 1（可直接当门禁）。
"""
import re
import sys
from collections import Counter
from pathlib import Path

BACKEND = Path(__file__).resolve().parent.parent
MIGRATIONS = BACKEND / "migrations"
CRATES = BACKEND / "crates"
BASELINE = BACKEND.parent / "docs/audits/artifacts/column_nullability_baseline.tsv"

# ── 迁移侧：列类型 + 可空性（CREATE TABLE + ALTER ADD COLUMN，按表合并）──
COL_TYPE = {}
COL_NULLABLE = {}
TABLE_ALIAS = {}
CREATED_TABLES = set()
create_re = re.compile(r'CREATE TABLE (?:IF NOT EXISTS )?"?([A-Za-z0-9_]+)"?\s*\(', re.I)
alter_add_re = re.compile(
    r'ALTER TABLE\s+(?:IF EXISTS\s+)?"?([A-Za-z0-9_]+)"?\s+ADD COLUMN\s+'
    r'(IF NOT EXISTS\s+)?"?([A-Za-z0-9_]+)"?\s+([^;\n]+)',
    re.I,
)
# 后续类型变更（044 这类修复迁移会把列从 TEXT 改回 BOOLEAN），按迁移顺序覆盖。
alter_type_re = re.compile(
    r'ALTER TABLE\s+(?:IF EXISTS\s+)?"?([A-Za-z0-9_]+)"?\s+ALTER\s+COLUMN\s+'
    r'"?([A-Za-z0-9_]+)"?\s+(?:SET\s+DATA\s+)?TYPE\s+([A-Z]+)',
    re.I,
)
alter_notnull_re = re.compile(
    r'ALTER TABLE\s+(?:IF EXISTS\s+)?"?([A-Za-z0-9_]+)"?\s+ALTER\s+COLUMN\s+'
    r'"?([A-Za-z0-9_]+)"?\s+(SET|DROP)\s+NOT\s+NULL',
    re.I,
)
rename_re = re.compile(
    r'ALTER TABLE\s+(?:IF EXISTS\s+)?"?([A-Za-z0-9_]+)"?\s+RENAME\s+TO\s+"?([A-Za-z0-9_]+)"?',
    re.I,
)
SERIAL_LIKE = {"SERIAL", "BIGSERIAL", "SMALLSERIAL", "SERIAL4", "SERIAL8", "SERIAL2"}


def resolve_table(name: str) -> str:
    seen = set()
    while name in TABLE_ALIAS and name not in seen:
        seen.add(name)
        name = TABLE_ALIAS[name]
    return name


def column_nullable(table: str, col: str):
    table = resolve_table(table)
    return COL_NULLABLE.get((table, col))


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
        # 同一张表的后续 `CREATE TABLE IF NOT EXISTS` 在真实库里是 no-op，
        # 不能覆盖最早建表时的列定义（032 与 034 都重声明过 x_ai_file 等）。
        if table in CREATED_TABLES:
            continue
        CREATED_TABLES.add(table)
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
                typ = up.split()[0].split("(")[0].rstrip(",").rstrip(")")
            COL_TYPE[(table, col)] = typ
            # 可空性：显式 NOT NULL / 列级 PRIMARY KEY / SERIAL 视为非空。
            not_null = (
                "NOT NULL" in up
                or "PRIMARY KEY" in up
                or typ in SERIAL_LIKE
            )
            COL_NULLABLE[(table, col)] = not not_null
    for m in alter_add_re.finditer(text):
        table = resolve_table(m.group(1).lower())
        col = m.group(3).lower()
        # `ADD COLUMN IF NOT EXISTS` 在列已存在时是 no-op（真实库不会因此变成
        # NOT NULL）。041 的 reply_count / 067 的 locked 都属这一类：原始 CREATE
        # 里是可空，后续 IF NOT EXISTS 的 NOT NULL 从未生效，故不能覆盖。
        if m.group(2) and (table, col) in COL_TYPE:
            continue
        rest = m.group(4).upper()
        typ = None
        for two in ("DOUBLE PRECISION", "CHARACTER VARYING",
                    "TIMESTAMP WITH TIME ZONE", "TIMESTAMP WITHOUT TIME ZONE"):
            if rest.startswith(two):
                typ = two
                break
        if typ is None:
            typ = rest.split()[0].split("(")[0].rstrip(",").rstrip(")")
        COL_TYPE[(table, col)] = typ
        COL_NULLABLE[(table, col)] = "NOT NULL" not in rest
    for m in alter_type_re.finditer(text):
        COL_TYPE[(resolve_table(m.group(1).lower()), m.group(2).lower())] = m.group(3).upper()
    for m in alter_notnull_re.finditer(text):
        COL_NULLABLE[(resolve_table(m.group(1).lower()), m.group(2).lower())] = (
            m.group(3).upper() == "DROP"
        )
    for m in rename_re.finditer(text):
        TABLE_ALIAS[m.group(1).lower()] = m.group(2).lower()
        for (t, c), v in list(COL_TYPE.items()):
            if t == m.group(1).lower():
                COL_TYPE[(m.group(2).lower(), c)] = v
        for (t, c), v in list(COL_NULLABLE.items()):
            if t == m.group(1).lower():
                COL_NULLABLE[(m.group(2).lower(), c)] = v

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


def is_option(raw: str) -> bool:
    return raw.strip().startswith("Option<")


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
# `Value::String(row.get("col"))`：隐式 String，是 O31/O35/O36 连续踩雷的形态。
implicit_string_re = re.compile(
    r'Value::String\s*\(\s*row\.get\s*\(\s*"([A-Za-z0-9_]+)"\s*\)\s*\)'
)
select_re = re.compile(r'\bSELECT\b', re.I)
from_re = re.compile(r'\bFROM\s+(?:ONLY\s+)?"?([A-Za-z0-9_]+)"?', re.I)
unknown_alias_re = re.compile(r'\bAS\s+"?([A-Za-z0-9_]+)"?', re.I)
count_alias_re = re.compile(r'COUNT\s*\([^)]*\)\s*(?:AS\s+)?"?([A-Za-z0-9_]+)"?', re.I)


def select_expectation(sql_select: str, col: str, table: str):
    """根据 SELECT 列表判断该列传给 Row 的期望类型与可空性。

    返回 (kind, value, nullable, skip_reason)：kind ∈ {"sqltype","rust","skip"}。
    """
    low = sql_select.lower()
    fields = sql_select

    def source_nullable(expr_col: str):
        return column_nullable(table, expr_col)

    # COUNT(...) AS col（或 COUNT(*) 的隐式列名 count）→ i64，聚合结果非空
    m = count_alias_re.search(fields)
    if m and m.group(1).lower() == col:
        return ("rust", "i64", False, None)
    if col == "count" and "count(" in low:
        return ("rust", "i64", False, None)

    # COALESCE(col, ...) → 非空（兜底表达式）
    if re.search(rf'COALESCE\s*\(\s*"?{col}"?\s*,', fields, re.I):
        sql_type = COL_TYPE.get((resolve_table(table), col))
        if sql_type is not None:
            return ("sqltype", sql_type, False, None)

    # col::text / CAST(col AS TEXT) → String，可空性继承源列
    cast_text = re.search(
        rf'\b{col}\b\s*::\s*(text|varchar|character varying)|CAST\s*\(\s*\b{col}\b\s+AS\s+(text|varchar)',
        low, re.I)
    if cast_text:
        return ("rust", "String", source_nullable(col), None)

    # to_char(col, ...) → String，NULL 仍返回 NULL
    if re.search(rf'to_char\s*\(\s*"?{col}"?', low, re.I):
        return ("rust", "String", source_nullable(col), None)

    # 其他 AS col 别名（表达式结果）→ 静态无法判定
    for m in unknown_alias_re.finditer(fields):
        if m.group(1).lower() == col:
            return ("skip", None, None, "expr_alias")

    # 裸列：选择列表中出现独立 word
    if re.search(rf'(?<![\w.])"?{col}"?(?![\w])', low):
        sql_type = COL_TYPE.get((resolve_table(table), col))
        if sql_type is None:
            return ("skip", None, None, "unknown_column")
        return ("sqltype", sql_type, source_nullable(col), None)
    return ("skip", None, None, "not_in_select")


def evaluate(fields: str, col: str, table: str, rust_type: str, method: str):
    """返回 (verdict, sql_type, null_risk)。"""
    kind, value, nullable, _reason = select_expectation(fields, col, table)
    if kind == "skip":
        return None, None, False
    if kind == "rust":
        sql_type = "BIGINT" if value == "i64" else "TEXT"
    else:
        sql_type = value
    verdict = classify_by_sql_type(rust_type, sql_type)
    if verdict is None:
        return None, sql_type, False
    # try_get 的失败通常被 .ok()/unwrap_or 吞掉，不会 panic；单列为 SILENT，
    # 表示“字段恒为空/静默丢失”，不进入“必然失败”必修集。
    if verdict == "FAIL" and method == "try_get":
        return "SILENT", sql_type, False
    # 类型匹配（或宽松）但列可空、Rust 非 Option → NULL 行必然 panic。
    null_risk = (
        verdict in ("OK", "LOOSE")
        and nullable is True
        and not is_option(rust_type)
        and method == "get"
    )
    return verdict, sql_type, null_risk


def scan():
    rows = []
    skip_stats = Counter()
    for rs in sorted(CRATES.glob("*/src/**/*.rs")):
        rel = rs.relative_to(BACKEND).as_posix()
        name = rs.name
        if ("/tests/" in rel or name == "tests.rs" or name.endswith("_tests.rs")
                or name.startswith("tests_")):
            continue
        text = rs.read_text(encoding="utf-8", errors="replace")
        line_of = lambda off: text.count("\n", 0, off) + 1  # noqa: E731

        calls = []
        for m in turbofish_re.finditer(text):
            calls.append((m.start(), m.group(2).strip(), m.group(3).lower(), m.group(1)))
        for m in typed_let_re.finditer(text):
            calls.append((m.start(), m.group(1).strip(), m.group(3).lower(), m.group(2)))
        for m in implicit_string_re.finditer(text):
            calls.append((m.start(), "String", m.group(1).lower(), "get"))

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
                skip_stats["no_select"] += 1
                continue
            _, table, fields = chosen
            # 跳过 SELECT *
            if re.fullmatch(r'\s*\*\s*', fields, re.I):
                skip_stats["select_star"] += 1
                continue
            if "{" in fields:
                skip_stats["format_interpolated"] += 1
                continue
            kind, _value, _nullable, reason = select_expectation(fields, col, table)
            if kind == "skip":
                skip_stats[reason] += 1
                continue
            verdict, sql_type, null_risk = evaluate(fields, col, table, rust_type, method)
            if verdict is None:
                continue
            if null_risk:
                verdict = "NULLRISK"
            rows.append((verdict, rel, str(line_of(pos)), col, rust_type,
                         table, sql_type))
    return rows, skip_stats


def baseline_key(row) -> str:
    verdict, rel, _line, col, _rust, table, _sql = row
    return f"{verdict}\t{rel}\t{resolve_table(table)}\t{col}"


def read_baseline() -> list:
    if not BASELINE.exists():
        return []
    return [ln.rstrip("\n") for ln in BASELINE.read_text(encoding="utf-8").splitlines()
            if ln.strip() and not ln.startswith("#")]


def write_baseline(keys: list):
    BASELINE.parent.mkdir(parents=True, exist_ok=True)
    header = ("# O36-D 类型一致性守卫基线（FAIL + NULLRISK）\n"
              "# 新增条目会让 column-nullability-scan 变红；修复后执行 --update-baseline 收敛。\n"
              "# 列：verdict\tfile\ttable\tcolumn\n")
    BASELINE.write_text(header + "\n".join(sorted(keys)) + "\n", encoding="utf-8")


def self_test() -> int:
    """注入用例：验证 NULLRISK/OK/SILENT 判定与可空性继承，不依赖真实仓库。"""
    global COL_TYPE, COL_NULLABLE, TABLE_ALIAS
    saved = (COL_TYPE, COL_NULLABLE, TABLE_ALIAS)
    COL_TYPE = {("t", "nullable_text"): "VARCHAR", ("t", "not_null_text"): "TEXT",
                ("t", "ts"): "TIMESTAMP", ("t", "cnt"): "BIGINT"}
    COL_NULLABLE = {("t", "nullable_text"): True, ("t", "not_null_text"): False,
                    ("t", "ts"): True, ("t", "cnt"): False}
    TABLE_ALIAS = {}
    cases = [
        # (fields, col, rust_type, method, expected_verdict)
        ('id, nullable_text FROM t', "nullable_text", "String", "get", "NULLRISK"),
        ('id, nullable_text FROM t', "nullable_text", "Option<String>", "get", "OK"),
        ('id, nullable_text::text AS nullable_text FROM t', "nullable_text", "String", "get", "NULLRISK"),
        ('id, nullable_text::text AS nullable_text FROM t', "nullable_text", "Option<String>", "get", "OK"),
        ('id, COALESCE(nullable_text, \'\') AS nullable_text FROM t', "nullable_text", "String", "get", "OK"),
        ('id, to_char(ts, \'YYYY\') AS ts FROM t', "ts", "String", "get", "NULLRISK"),
        ('id, count(*) AS cnt FROM t', "cnt", "i64", "get", "OK"),
        ('id, not_null_text FROM t', "not_null_text", "String", "get", "OK"),
        ('id, nullable_text FROM t', "nullable_text", "String", "try_get", "OK"),
        ('id, nullable_text FROM t', "nullable_text", "i64", "get", "FAIL"),
        ('id, nullable_text FROM t', "nullable_text", "Option<i64>", "get", "FAIL"),
    ]
    failed = 0
    for fields, col, rust, method, want in cases:
        verdict, _sql, risk = evaluate(fields, col, "t", rust, method)
        if risk and verdict == "OK":
            verdict = "NULLRISK"
        if verdict != want:
            failed += 1
            print(f"self-test FAIL: {fields!r} {col} {rust} {method} -> {verdict}, want {want}")
    COL_TYPE, COL_NULLABLE, TABLE_ALIAS = saved
    print(f"self-test: {len(cases) - failed}/{len(cases)} passed")
    return 0 if failed == 0 else 1


def main() -> int:
    if "--self-test" in sys.argv:
        return self_test()

    rows, skip_stats = scan()

    fails = [r for r in rows if r[0] == "FAIL"]
    nullrisks = [r for r in rows if r[0] == "NULLRISK"]
    silents = [r for r in rows if r[0] == "SILENT"]
    looses = [r for r in rows if r[0] == "LOOSE"]

    if "--update-baseline" in sys.argv:
        gate_rows = fails + nullrisks
        write_baseline([baseline_key(r) for r in gate_rows])
        print(f"baseline updated: {len(gate_rows)} entries -> {BASELINE}")

    # 棘轮：FAIL（类型不匹配必 500）与 NULLRISK（可空列非 Option，NULL 行必 500）
    # 都纳入基线；新增即红，修复后提示收敛。
    current = Counter(baseline_key(r) for r in fails + nullrisks)
    known = Counter(read_baseline())
    new_entries = sorted((current - known).elements())
    stale_entries = sorted((known - current).elements())

    if "--summary" in sys.argv:
        print(f"FAIL={len(fails)} NULLRISK={len(nullrisks)} SILENT={len(silents)} LOOSE={len(looses)}")
        for crate, n in Counter(r[1].split("/")[1] for r in fails).most_common():
            print(f"  FAIL {crate}: {n}")
        for crate, n in Counter(r[1].split("/")[1] for r in nullrisks).most_common():
            print(f"  NULLRISK {crate}: {n}")
        print("  skip: " + ", ".join(f"{k}={v}" for k, v in sorted(skip_stats.items())))
    else:
        print("VERDICT\tFILE\tLINE\tCOLUMN\tRUST_TYPE\tTABLE\tSQL_TYPE")
        for r in rows:
            print("\t".join(r))

    if new_entries:
        print(f"\n基线外新增 {len(new_entries)} 条（FAIL = 类型必失败 / NULLRISK = NULL 行必 500）：")
        for e in new_entries:
            print(f"  + {e}")
        print("修复后执行 --update-baseline。")
    if stale_entries:
        print(f"\n基线内已收敛 {len(stale_entries)} 条（提示，不阻断）：")
        for e in stale_entries:
            print(f"  - {e}")

    return 1 if new_entries else 0


if __name__ == "__main__":
    sys.exit(main())
