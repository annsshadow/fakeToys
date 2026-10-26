#!/usr/bin/env python3
# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later
"""FI-002 静态列扫描：handler SQL 字面量引用的 (table, column) 对撞 migrations 的 schema。

## 为什么有这个脚本

FI-002（docs/audits/found-issues.md）用「代码 SQL 字面量 vs live PG information_schema」
对撞，一次挖出约 50 个运行时必然 500 的 handler——且全部被既有测试漏过。那轮的方法
有效但只存在于文档里，没有固化；本轮把它做成**无需活库**的静态版并接入 CI：

- **schema 宇宙**：解析 `migrations/*.sql`（运行器从盘读、按文件名序应用）的
  CREATE TABLE / ALTER TABLE ADD|DROP COLUMN / RENAME，得到 (table, column) 全集；
- **代码引用**：扫 `crates/*/src`、`src/` 的 Rust 字符串字面量，抽取表引用
  （FROM/JOIN/UPDATE/INSERT INTO/DELETE FROM）与列引用（限定列、INSERT 列清单、
  UPDATE SET、单表 SELECT 裸列、**WHERE/FILTER 裸列、ORDER BY/GROUP BY 裸列**——
  后三者为 2026-10-26 P1-1 补抽，补前 F-1/F-5 两个 fresh-DB 500 从盲区漏过），
  用同字面量内的别名映射解析；
- **棘轮**：与 `docs/audits/artifacts/fi002_unknown_columns.tsv` 基线对比，
  **新增**未知 (table, column) 对即退出 1；基线只能人工复核后 `--update-baseline` 收敛。

## 口径与已知边界（fail-loud）

- 只认**静态字面量**；`format!` 里的 `{}` 动态表名/列名会被跳过（无法静态判定），
  跳过数在输出里明示，不静默；
- 动态表名（`FROM {table}`）的整条 SQL 仍会解析静态部分，表名侧记入 skipped；
- `information_schema` / `pg_catalog` 等系统表不在 migrations 里，作为已知外部前缀跳过；
- CTE（`WITH x AS (...)`）的别名不映射回基表，其限定列按「未解析限定符」计数
  并明示（不计入违规）；
- 多表查询的裸列（无别名限定）无法归属，跳过并计数；WHERE/ORDER BY/GROUP BY
  的新抽取沿用同一「仅单表」口径；
- WHERE 抽取前先剥掉含 SELECT 的子查询（否则 `EXISTS (SELECT 1 FROM
  information_schema.columns WHERE table_name=...)` 的内层裸列会归属到外层表，
  实测 attendance:1114 假阳性）；`$N::type` 铸造、函数调用、关键字均已排除；
- SELECT 清单/UPDATE SET/ORDER BY/GROUP BY 的表达式项（含 `(`）整项跳过——
  表达式内部的列不被抽取（如 `ORDER BY to_char(x, ...)` 的 x）；
- **仍未覆盖**：`format!` 模板 `{}` 插值内容（F-6 的 ref_col 家族，靠 P1-3
  白名单守卫）；SELECT 别名被 ORDER BY/GROUP BY 引用时按别名豁免（P1-1 实测
  收口：search 三处 `ts_rank(...) AS rank`、cms `field_name AS name`）。
- **已收口（2026-10-26）**：视图（CREATE VIEW）的投影列——`build_universe`
  增认 `CREATE VIEW ... AS SELECT ... FROM base`，把投影列（裸列 + `AS alias`
  别名）并入折叠小写表名宇宙（B-7）。`"PP_E_APPLICATION"` 的 xkey/xterminal
  因此进 `pp_e_application` 宇宙，不再登记基线。反向注入自证：
  `SELECT xkey_typo FROM "PP_E_APPLICATION"` 被抓。
- **已收口（2026-10-26，B-2）**：关系名大小写折叠——`build_universe` 对带引号
  含大写的关系名保留原始大小写独立建模（小写表名走常规折叠路径不变），
  `analyze_sql` 的 TABLE_REF 对含大写的引号名按原始大小写查宇宙。全仓 5 个
  带引号含大写关系名（FILE_FOLDER/FILE_FILE/FILE_PERMISSION/FILE_SHARE/
  PP_E_APPLICATION）补进大小写键。双向验证：查 "PP_E_APPLICATION" 的 xkey
  命中、查 pp_e_application 的 xid 走小写折叠、大写幽灵列注入被点名；全仓
  基线态 exit 0、零新增违规。**至此 B-1~B-7 全收口，无留待项**。

## 用法

    python scripts/fi002_column_scan.py                 # 棘轮模式（CI 用）
    python scripts/fi002_column_scan.py --report        # 打印全部未知对（含基线内）
    python scripts/fi002_column_scan.py --update-baseline  # 人工复核后重写基线
"""

from __future__ import annotations

import argparse
import re
import sys
from pathlib import Path

BACKEND = Path(__file__).resolve().parents[1]
MIGRATIONS = BACKEND / "migrations"
BASELINE = BACKEND / "docs" / "audits" / "artifacts" / "fi002_unknown_columns.tsv"
SRC_ROOTS = [BACKEND / "src", BACKEND / "crates"]

# 系统 schema 前缀：不在 migrations 里，按设计跳过
SYSTEM_SCHEMAS = {
    "information_schema",
    "pg_catalog",
    "pg_class",
    "pg_namespace",
    "pg_index",
    "pg_attribute",
    "pg_constraint",
    # 迁移运行器自建的追踪表（shared/src/migrate.rs），不在 migrations 里
    "schema_migrations",
    # PG 系统视图：query_service_processing/src/lib.rs:215 的
    # `FROM pg_stat_activity WHERE datname = ...`——补 WHERE 抽取后它会被
    # 误报成「表不在 migrations」，须与 information_schema 同批入名单（P1-1）。
    "pg_stat_activity",
}

SQL_KEYWORDS = {
    "select", "insert", "update", "delete", "from", "join", "where", "set",
    "values", "into", "on", "and", "or", "not", "null", "order", "group",
    "by", "having", "limit", "offset", "as", "left", "right", "inner",
    "outer", "cross", "full", "union", "all", "distinct", "case", "when",
    "then", "else", "end", "is", "in", "exists", "between", "like", "ilike",
    "asc", "desc", "true", "false", "returning", "conflict", "do", "nothing",
    "with", "recursive", "interval", "now", "current_timestamp", "coalesce",
}

CONSTRAINT_STARTERS = {
    "constraint", "primary", "foreign", "unique", "check", "exclude", "like",
}


def strip_sql_comments(text: str) -> str:
    """去掉 -- 行注释与 /* */ 块注释（字符串字面量内的不动，迁移里无此情况）。"""
    text = re.sub(r"/\*.*?\*/", " ", text, flags=re.S)
    return re.sub(r"--[^\n]*", "", text)


def split_top_level(body: str, sep: str = ",") -> list[str]:
    """按顶层分隔符切分（忽略括号内的分隔符，如 NUMERIC(10,2)、CHECK (a,b)）。"""
    parts, depth, cur = [], 0, []
    for ch in body:
        if ch == "(":
            depth += 1
        elif ch == ")":
            depth -= 1
        if ch == sep and depth == 0:
            parts.append("".join(cur))
            cur = []
        else:
            cur.append(ch)
    if cur:
        parts.append("".join(cur))
    return parts


IDENT = r'(?:"([^"]+)"|([A-Za-z_][A-Za-z0-9_]*))'


def ident_of(m: re.Match) -> str:
    """第一个 IDENT 的值（CREATE TABLE 用）。"""
    return ident_at(m, 0)


def ident_at(m: re.Match, idx: int) -> str:
    """第 idx 个 IDENT 的值（0 起）。

    每个 IDENT 贡献两个捕获组（带引号 / 不带引号）。**不能**用
    `m.group(1) or m.group(2)` 取第二个 IDENT——那样 ALTER TABLE 的列名
    会读到表名，把表自己当列加进宇宙（2026-10-25 实测：auth_person 因此
    只剩 13 真列 + 1 个假列）。
    """
    quoted = m.group(2 * idx + 1)
    bare = m.group(2 * idx + 2)
    return (quoted or bare or "").strip()


def parse_create_table(body: str) -> list[str]:
    """从 CREATE TABLE 的括号体里提取列名（跳过表级约束）。"""
    cols: list[str] = []
    for item in split_top_level(body):
        item = item.strip()
        if not item:
            continue
        low = item.lower()
        first = low.split(None, 1)[0].rstrip("(")
        if first in CONSTRAINT_STARTERS:
            # PRIMARY KEY (a, b) / UNIQUE (a) 形式：括号内是列
            m = re.match(r"(?:primary\s+key|unique)\s*\(([^)]*)\)", low)
            if m:
                for c in m.group(1).split(","):
                    c = c.strip().strip('"')
                    if c:
                        cols.append(c)
            continue
        m = re.match(IDENT, item)
        if m:
            cols.append(ident_of(m).lower())
    return cols


def build_universe() -> tuple[dict[str, set[str]], list[str]]:
    """解析全部正向迁移，返回 (table -> columns, 解析告警)。"""
    universe: dict[str, set[str]] = {}
    warnings: list[str] = []
    files = sorted(
        p for p in MIGRATIONS.glob("*.sql") if not p.name.endswith("_rollback.sql")
    )
    if not files:
        raise SystemExit(f"未找到迁移文件：{MIGRATIONS}")

    for f in files:
        text = strip_sql_comments(f.read_text(encoding="utf-8"))

        # CREATE TABLE [IF NOT EXISTS] name ( body )
        for m in re.finditer(
            r"CREATE\s+TABLE\s+(?:IF\s+NOT\s+EXISTS\s+)?" + IDENT + r"\s*\(",
            text,
            re.I,
        ):
            table = ident_of(m).lower()
            start = m.end()
            depth, i = 1, start
            while i < len(text) and depth > 0:
                if text[i] == "(":
                    depth += 1
                elif text[i] == ")":
                    depth -= 1
                i += 1
            body = text[start : i - 1]
            cols = parse_create_table(body)
            if not cols:
                warnings.append(f"{f.name}: CREATE TABLE {table} 未解析出列")
            universe.setdefault(table, set()).update(cols)
            # B-2 收口（2026-10-26）：带引号且含大写字母的关系名保留原始大小写
            # 独立建模。PostgreSQL 对引号标识符大小写敏感——"X_PP_C_TASK" 与
            # x_pp_c_task 是不同关系。折叠小写会让「引号大写表引用」误查小写表的列。
            # 仅对含大写的引号名建镜像键（小写表名走常规折叠路径，不受影响）。
            raw_ident = ident_of(m)
            if raw_ident != raw_ident.lower():
                universe.setdefault(raw_ident, set()).update(cols)

        # CREATE VIEW [IF NOT EXISTS] name AS SELECT ... FROM base_table
        # B-7 收口（2026-10-26）：视图投影列（含 AS alias 的别名）并入折叠小写的
        # 表名宇宙。112 建的 "PP_E_APPLICATION" 视图投影 xid AS xkey / xid AS
        # xterminal——这两个是**视图层别名**，不是基表 pp_e_application 的物理列。
        # 扫描器按 B-2 把 "PP_E_APPLICATION" 折叠到小写 pp_e_application 后查列，
        # 表宇宙里没有 xkey/xterminal 故登记基线。此处把视图 SELECT 清单的投影列
        # （裸列 + AS 别名）并入同名表的宇宙，让扫描侧命中而不再误登记。
        for vm in re.finditer(
            r"CREATE\s+VIEW\s+(?:IF\s+NOT\s+EXISTS\s+)?" + IDENT + r"\s+AS\b",
            text,
            re.I,
        ):
            vname = ident_of(vm).lower()
            rest = text[vm.end():]
            sm = re.search(r"\bselect\b(.*?)\bfrom\b", rest, re.I | re.S)
            if not sm:
                continue
            proj: set[str] = set()
            for item in split_top_level(sm.group(1)):
                s = item.strip()
                if not s:
                    continue
                am = re.search(r"\bas\s+\"?([A-Za-z_][A-Za-z0-9_]*)\"?\s*$", s, re.I)
                if am:
                    proj.add(am.group(1).lower())
                else:
                    mm = re.match(r"\"?([A-Za-z_][A-Za-z0-9_]*)\"?", s)
                    if mm and mm.group(1).lower() not in SQL_KEYWORDS:
                        proj.add(mm.group(1).lower())
            fm = re.search(r"\bfrom\s+(\"[^\"]+\"|[A-Za-z_][A-Za-z0-9_]*)", rest, re.I)
            if fm:
                base = fm.group(1).strip('"').lower()
                universe.setdefault(base, set()).update(proj)
                universe.setdefault(vname, set()).update(proj)
                # B-2 收口：带引号含大写的视图名保留原始大小写独立建模
                vname_raw = ident_of(vm)
                if vname_raw != vname_raw.lower():
                    universe.setdefault(vname_raw, set()).update(proj)

        # ALTER TABLE 语句级扫描：一条 ALTER 可带**多个** ADD/DROP COLUMN
        # （逗号分隔，如 018 一次加 7 列）。按 `;` 切语句，逐语句处理。
        for stmt_m in re.finditer(
            r"ALTER\s+TABLE\s+(?:IF\s+EXISTS\s+)?" + IDENT + r"([^;]*);",
            text,
            re.I,
        ):
            table = ident_at(stmt_m, 0).lower()
            body = stmt_m.group(3)

            # ADD COLUMN [IF NOT EXISTS] col
            for m in re.finditer(
                r"\bADD\s+COLUMN\s+(?:IF\s+NOT\s+EXISTS\s+)?" + IDENT, body, re.I
            ):
                universe.setdefault(table, set()).add(ident_at(m, 0).lower())

            # DROP COLUMN [IF EXISTS] col
            for m in re.finditer(
                r"\bDROP\s+COLUMN\s+(?:IF\s+EXISTS\s+)?" + IDENT, body, re.I
            ):
                if table in universe:
                    universe[table].discard(ident_at(m, 0).lower())

            # RENAME TO new（表重命名）
            m = re.match(r"\s*RENAME\s+TO\s+" + IDENT, body, re.I)
            if m:
                new = ident_at(m, 0).lower()
                if table == new:
                    # 011 的「大写→小写」重命名：大小写不敏感建模下是 no-op
                    continue
                if table in universe:
                    universe[new] = universe.pop(table)
                # 旧表不在宇宙内：表不是 migrations 建的（遗留库），
                # 其引用会以「表不在 migrations」出现在违规里，由基线登记

    return universe, warnings


# ── Rust 字面量提取 ──────────────────────────────────────────────────────────

def unescape_rust(body: str) -> str:
    """把 Rust 字面量的转义还原成运行期字符串。

    **必须做**：源码里的行续接（反斜杠 + 换行）在运行期被吃掉，不还原就会
    让 `WITH RECURSIVE chain AS (...),\n seeds AS (...)` 这种多行 CTE 的
    正则失配——实测把 seeds/sub_groups/tree 当成幽灵表。
    """
    body = body.replace("\\\n", "")  # 行续接：反斜杠 + 换行
    body = body.replace('\\"', '"')
    body = body.replace("\\n", " ").replace("\\t", " ").replace("\\r", " ")
    return body


LITERAL_RE = re.compile(
    r'r#"(?P<raw>.*?)"#|r"(?P<raw2>[^"]*)"|"(?P<norm>(?:[^"\\]|\\.)*)"',
    re.S,
)


def cfg_test_ranges(text: str) -> list[tuple[int, int]]:
    """`#[cfg(test)] mod ... { ... }` 的字节区间。

    文件内测试模块里的 SQL 是夹具（常打 users/t/x_probe 这类虚构表），
    不该进棘轮。按花括号配平取块区间——比「见到 cfg(test) 就切掉后半文件」
    精确，避免误伤测试模块之后的产码。
    """
    ranges = []
    for m in re.finditer(r"#\[cfg\(test\)\]\s*(?:\n\s*)*mod\s+[A-Za-z0-9_]+\s*\{", text):
        start = m.start()
        i = m.end() - 1  # 指向 '{'
        depth = 0
        while i < len(text):
            if text[i] == "{":
                depth += 1
            elif text[i] == "}":
                depth -= 1
                if depth == 0:
                    break
            i += 1
        ranges.append((start, i + 1))
    return ranges


def rust_literals(path: Path):
    """产出 (行号, 字面量内容)。只取含 SQL 关键词、且不在 #[cfg(test)] 模块内的。"""
    try:
        text = path.read_text(encoding="utf-8", errors="replace")
    except OSError:
        return
    skip = cfg_test_ranges(text)
    for m in LITERAL_RE.finditer(text):
        if any(a <= m.start() < b for a, b in skip):
            continue
        body = m.group("raw") or m.group("raw2") or m.group("norm") or ""
        if not body:
            continue
        low = body.lower()
        if not re.search(r"\b(select|insert|update|delete)\b", low):
            continue
        if not re.search(r"\b(from|into|set|join|table)\b", low):
            continue
        line = text.count("\n", 0, m.start()) + 1
        yield line, unescape_rust(body)


QUALIFIED_RE = re.compile(r"\b([A-Za-z_][A-Za-z0-9_]*)\.([A-Za-z_][A-Za-z0-9_]*)\b")
TABLE_REF_RE = re.compile(
    r"\b(?:from|join|update|insert\s+into|delete\s+from)\s+"
    r'("([^"]+)"|([A-Za-z_][A-Za-z0-9_]*))'
    r"(?:\s+(?:as\s+)?([A-Za-z_][A-Za-z0-9_]*))?",
    re.I,
)
INSERT_COLS_RE = re.compile(
    r"\binsert\s+into\s+(\"[^\"]+\"|[A-Za-z_][A-Za-z0-9_]*)\s*\(([^)]*)\)", re.I
)
SET_CLAUSE_RE = re.compile(r"\bset\s+(.*?)(?:\bwhere\b|\breturning\b|$)", re.I | re.S)
SELECT_LIST_RE = re.compile(r"\bselect\s+(.*?)\s+\bfrom\b", re.I | re.S)
# B-6 修复（2026-10-26）：`SELECT DISTINCT col, ...` 的 DISTINCT 原先落在 group(1)
# 的第一个 split 项里，被裸列分支按「关键字」静默跳过 ⇒ 首列 col 永不被抽取
# （P0-6 反向注入 `SELECT DISTINCT process_key_typo, ...` 实证：改首列漏检）。
# 修法不改正则（惰性 `*?` 量词会先取 0 次、让 `.*?` 吸收 DISTINCT，正则改不动），
# 改为**抽取后**剥掉列表体开头的 DISTINCT / ALL 修饰词。
SELECT_MODIFIERS = re.compile(r"^(?:distinct|all)\s+", re.I)


def strip_select_modifiers(list_body: str) -> str:
    """剥掉 SELECT 列表体开头的 DISTINCT / ALL（可重复），让首列真正参与抽取。"""
    s = list_body
    while True:
        s2 = SELECT_MODIFIERS.sub("", s, count=1)
        if s2 == s:
            break
        s = s2
    return s
# WHERE 片段：从首个 where 到 returning/order/group/having/limit/window 或结尾
WHERE_TAIL_RE = re.compile(
    r"\bwhere\b(.*?)(?:\breturning\b|\border\s+by\b|\bgroup\s+by\b|\bhaving\b"
    r"|\blimit\b|\bwindow\b|\bfetch\b|$)",
    re.I | re.S,
)
# ORDER BY / GROUP BY 的列表段（裸列按逗号切分后取首标识符）
ORDER_GROUP_RE = re.compile(
    r"\b(?:order|group)\s+by\b(.*?)(?:\blimit\b|\boffset\b|\bfor\b|\bwindow\b|$)",
    re.I | re.S,
)
# 裸列判定：标识符后跟比较/成员/模式运算符。
# 负向断言排除：`.col`（限定列，QUALIFIED_RE 负责）、`$1`（参数）、
# `::type`（强制转换——实测 `($1::text IS NULL OR ...)` 会被误判成裸列 text）。
BARE_COL_RE = re.compile(
    r"(?<![.\w$:])([A-Za-z_][A-Za-z0-9_]*)\s*"
    r"(?:=|<>|!=|<=|>=|<|>|\bis\b|\bin\b|\blike\b|\bilike\b|\bbetween\b)",
    re.I,
)


def strip_subqueries(text: str) -> str:
    """删掉含 SELECT 的括号子查询（保留其余括号内容）。

    为什么必须做：`... AND EXISTS (SELECT 1 FROM information_schema.columns
    WHERE table_name = 'x' AND column_name = 'y')` 若不剥子查询，子查询里的
    裸列（table_name/column_name）会被归属到外层表——实测
    attendance_assemble_control/src/lib.rs:1114 就是这个形状的假阳性。
    """
    out: list[str] = []
    i = 0
    n = len(text)
    while i < n:
        ch = text[i]
        if ch == "(":
            depth = 0
            j = i
            while j < n:
                if text[j] == "(":
                    depth += 1
                elif text[j] == ")":
                    depth -= 1
                    if depth == 0:
                        break
                j += 1
            inner = text[i + 1 : j]
            if re.search(r"\bselect\b", inner, re.I):
                i = j + 1  # 整段子查询丢弃
                continue
        out.append(ch)
        i += 1
    return "".join(out)


def bare_cols_in_clause(clause: str) -> set[str]:
    """从一个子句片段抽裸列名（小写）。函数调用/关键字/参数/ cast 均已排除。"""
    cols: set[str] = set()
    for m in BARE_COL_RE.finditer(clause):
        name = m.group(1).lower()
        if name in SQL_KEYWORDS:
            continue
        # 标识符后紧跟 '(' 的是函数调用（count(/ now()），不是列
        tail = clause[m.end(1) :].lstrip()
        if tail.startswith("("):
            continue
        cols.add(name)
    return cols


def bare_cols_in_list(segment: str) -> set[str]:
    """ORDER BY / GROUP BY 列表段：按逗号切，每项取首标识符。

    与 SELECT 清单分支同口径：含 '(' 的项（函数/聚合表达式）整项跳过——
    表达式内部的列不被抽取（既有 SELECT 分支的同款边界，保持一致）。
    带引号的裸列（"xcreateTime"）去引号后参与判定（B-4 同款修复）。
    """
    cols: set[str] = set()
    for item in split_top_level(segment):
        item = item.strip().strip('"')
        if not item or item == "*" or item.endswith("*"):
            continue
        if "(" in item or "." in item:
            continue
        mm = re.match(r"([A-Za-z_][A-Za-z0-9_]*)", item)
        if not mm:
            continue
        name = mm.group(1).lower()
        if name in SQL_KEYWORDS:
            continue
        cols.add(name)
    return cols


def analyze_sql(sql: str) -> tuple[set[tuple[str, str]], dict[str, int]]:
    """从一条 SQL 字面量抽取 (table, column) 引用，返回 (pairs, 统计)。"""
    stats = {"unresolved_qualifier": 0, "ambiguous_bare": 0, "dynamic": 0}
    pairs: set[tuple[str, str]] = set()

    # 含 {…} 的是 format! 模板：部分 SQL（常见是 {ANCESTOR_CTE} 这类 CTE 片段）
    # 不在本字面量里，CTE 名无从识别。此类字面量**只报能正向解析的引用**
    # （表在 migrations 宇宙内），不报「表不在 migrations」——否则 CTE 名会被
    # 当成幽灵表（实测：chain / events / seeds / tree / sub_groups 全是这种）。
    dynamic = "{" in sql and "}" in sql
    if dynamic:
        stats["dynamic"] += 1

    # CTE 名（WITH x AS (...)、WITH RECURSIVE a AS (...), b AS (...)）：
    # 它们是查询内临时关系，不是基表——当表用会报一堆假违规
    # （实测：base / chain / hit / events / gs 全是 CTE）。
    cte_names: set[str] = set()
    for m in re.finditer(
        r"(?:\bwith\b(?:\s+recursive)?|,)\s*([A-Za-z_][A-Za-z0-9_]*)"
        r"(?:\s*\([^()]*\))?\s+as\s*\(",
        sql,
        re.I,
    ):
        cte_names.add(m.group(1).lower())

    # 表引用 + 别名映射
    alias: dict[str, str] = {}
    tables: list[str] = []
    for m in TABLE_REF_RE.finditer(sql):
        raw = m.group(1)
        table = (m.group(2) or m.group(3) or "").strip('"').lower()
        if not table:
            continue
        # B-2 收口（2026-10-26）：带引号且含大写字母的关系名保留原始大小写——
        # PostgreSQL 引号标识符大小写敏感，"X_PP_C_TASK" 与 x_pp_c_task 不同。
        # 折叠小写会让这类引用误查小写表的列（B-2 盲区）。此处对含大写的引号
        # 名按原始大小写查宇宙，小写/不带引号名走常规折叠路径不变。
        quoted_name = m.group(2)  # 引号内原始文本（可能大写），None 表示裸名
        if quoted_name is not None and quoted_name != quoted_name.lower():
            table = quoted_name  # 保留原始大小写
        # 系统 schema（information_schema / pg_catalog / pg_class …）不是 migrations 建的表
        if table in SYSTEM_SCHEMAS:
            continue
        # CTE 名不是基表
        if table in cte_names:
            continue
        # schema 限定写法（information_schema.columns）：表名后紧跟 '.' 的是 schema
        tail = sql[m.end(1):m.end(1) + 1]
        if tail == ".":
            continue
        ali = (m.group(4) or "").lower()
        if ali and ali not in SQL_KEYWORDS:
            alias[ali] = table
        if table not in tables:
            tables.append(table)

    # 限定列 ref.col
    for m in QUALIFIED_RE.finditer(sql):
        left, right = m.group(1).lower(), m.group(2).lower()
        if left in SYSTEM_SCHEMAS:
            continue
        if left in cte_names:
            stats["unresolved_qualifier"] += 1
        elif left in alias:
            pairs.add((alias[left], right))
        elif left in tables:
            pairs.add((left, right))
        else:
            # 可能是 CTE 别名 / 函数命名空间 / schema 限定；不计违规
            stats["unresolved_qualifier"] += 1

    # INSERT 列清单
    for m in INSERT_COLS_RE.finditer(sql):
        table = m.group(1).strip('"').lower()
        if table in SYSTEM_SCHEMAS or table in cte_names:
            continue
        for c in m.group(2).split(","):
            c = c.strip().strip('"').lower()
            if c and re.fullmatch(r"[a-z_][a-z0-9_]*", c):
                pairs.add((table, c))

    # UPDATE SET 列（单表；多表 UPDATE 的 SET 列无法归属）
    if tables and len(tables) == 1:
        for m in SET_CLAUSE_RE.finditer(sql):
            for item in split_top_level(m.group(1)):
                # B-4 修复：SET "xupdateTime" = NOW() 的引号列原先被
                # re.match 的引号开头跳过——去引号后再匹配（INSERT 清单分支
                # 一直是这个口径，此处对齐）。
                mm = re.match(r"\s*\"?([A-Za-z_][A-Za-z0-9_]*)\"?\s*=", item)
                if mm:
                    pairs.add((tables[0], mm.group(1).lower()))

    # 单表 SELECT 裸列
    if tables and len(tables) == 1:
        for m in SELECT_LIST_RE.finditer(sql):
            for item in split_top_level(strip_select_modifiers(m.group(1))):
                item = item.strip()
                if not item or item == "*" or item.endswith("*"):
                    continue
                # B-4 修复：`"xcreateTime"` 这类带引号裸列原先以引号开头、
                # re.match 直接失配被静默跳过（F-7 的 pp_c_keylock."xcreateTime"
                # 因此没进过违规集）。去引号后再匹配——INSERT 清单分支一直是
                # 这个口径，此处对齐。
                mm = re.match(r"\"?([A-Za-z_][A-Za-z0-9_]*)\"?", item.strip('"'))
                if not mm:
                    continue
                name = mm.group(1).lower()
                # 函数调用（count( / coalesce( ...）或关键字不算裸列；
                # 限定列（u.id / 表名.列）由 QUALIFIED_RE 负责，这里必须跳过——
                # 否则别名会被当成列名（实测：x_org_unit.u2、x_work.w 一批假违规）。
                if "(" in item or "." in item or name in SQL_KEYWORDS:
                    continue
                pairs.add((tables[0], name))
    elif len(tables) > 1:
        stats["ambiguous_bare"] += 1

    # ── WHERE / FILTER / ORDER BY / GROUP BY 裸列（P1-1 补抽，2026-10-26）──
    # 原扫描器只抽 SELECT 清单/SET/INSERT 清单/限定列，WHERE 与 ORDER BY 里的
    # 裸列完全不可见——F-1（x_process_bam.status，WHERE/FILTER 里）与 F-5
    # （x_program_field.sort_order，ORDER BY 里）正是从这个盲区漏进 fresh DB 500 的。
    # 归属口径与上面一致：仅单表查询（多表无法归属，跳过并计数）。
    if tables and len(tables) == 1:
        table = tables[0]
        # SELECT 别名集合：`expr AS alias` / `expr alias`。ORDER BY/GROUP BY
        # 引用别名不是基表列——实测 search 三处 `ts_rank(...) AS rank` +
        # ORDER BY rank、cms `field_name AS name` + ORDER BY name 全是假阳性。
        aliases: set[str] = set()
        for sm in SELECT_LIST_RE.finditer(sql):
            for item in split_top_level(strip_select_modifiers(sm.group(1))):
                s = item.strip()
                am = re.search(r"\bas\s+\"?([A-Za-z_][A-Za-z0-9_]*)\"?\s*$", s, re.I) or \
                    re.search(r"\)\s+\"?([A-Za-z_][A-Za-z0-9_]*)\"?\s*$", s)
                if am:
                    aliases.add(am.group(1).lower())
        for wm in WHERE_TAIL_RE.finditer(sql):
            clause = strip_subqueries(wm.group(1))
            for col in bare_cols_in_clause(clause) - aliases:
                pairs.add((table, col))
        for om in ORDER_GROUP_RE.finditer(sql):
            for col in bare_cols_in_list(om.group(1)) - aliases:
                pairs.add((table, col))

    return pairs, stats


CTE_DEF_RE = re.compile(
    r"(?:\bwith\b(?:\s+recursive)?|,)\s*([A-Za-z_][A-Za-z0-9_]*)"
    r"(?:\s*\([^()]*\))?\s+as\s*\(",
    re.I,
)


def collect_cte_names() -> set[str]:
    """全仓 CTE 名集合。

    单个 SQL 片段（如 `UNION SELECT ... JOIN sub_groups sg`）里的 CTE 名
    在其外层模板里定义——只看单条字面量认不出。CTE 名是**查询内**临时关系，
    但本仓按约定跨片段复用，故全仓收集后统一当非基表处理。
    """
    names: set[str] = set()
    for root in SRC_ROOTS:
        if not root.exists():
            continue
        for path in sorted(root.rglob("*.rs")):
            for _, sql in rust_literals(path):
                for m in CTE_DEF_RE.finditer(sql):
                    names.add(m.group(1).lower())
    return names


def scan_code(universe: dict[str, set[str]]):
    """返回 (violations, 统计)。violations: {(table, column): [evidence...]}"""
    violations: dict[tuple[str, str], list[str]] = {}
    global_cte = collect_cte_names()
    stats = {
        "files": 0,
        "literals": 0,
        "pairs": 0,
        "unresolved": 0,
        "ambiguous": 0,
        "dynamic": 0,
        "test_refs": 0,
        "dynamic_unknown_table": 0,
        "cte_refs": 0,
    }

    for root in SRC_ROOTS:
        if not root.exists():
            continue
        for path in sorted(root.rglob("*.rs")):
            rel = path.relative_to(BACKEND).as_posix()
            stats["files"] += 1
            for line, sql in rust_literals(path):
                stats["literals"] += 1
                pairs, st = analyze_sql(sql)
                stats["unresolved"] += st["unresolved_qualifier"]
                stats["ambiguous"] += st["ambiguous_bare"]
                stats["dynamic"] += st["dynamic"]
                is_test_file = "test" in path.name.lower()
                for table, col in pairs:
                    stats["pairs"] += 1
                    if is_test_file:
                        # 测试文件的 SQL 常打虚构表（users/t/x_probe）——不计违规
                        stats["test_refs"] += 1
                        continue
                    if table not in universe:
                        if table in global_cte:
                            # 别处定义的 CTE 名（片段引用），不是基表
                            stats["cte_refs"] += 1
                            continue
                        if st["dynamic"]:
                            # 动态模板里的未知表多半是占位符展开的 CTE/临时表
                            stats["dynamic_unknown_table"] += 1
                            continue
                        violations.setdefault((table, col), []).append(
                            f"{rel}:{line} 表 {table} 不在 migrations"
                        )
                    elif col not in universe[table]:
                        violations.setdefault((table, col), []).append(
                            f"{rel}:{line}"
                        )
    return violations, stats


# ── 基线棘轮 ────────────────────────────────────────────────────────────────


def load_baseline() -> set[tuple[str, str]]:
    if not BASELINE.exists():
        return set()
    known = set()
    for raw in BASELINE.read_text(encoding="utf-8").splitlines():
        if not raw or raw.startswith("#"):
            continue
        parts = raw.split("\t")
        if len(parts) >= 2:
            known.add((parts[0].strip().lower(), parts[1].strip().lower()))
    return known


# 基线条目分类理由（人工复核结论，2026-10-25）
REASON_LEGACY_TABLE = (
    "遗留表：Java 时代建的表，migrations 从不创建（011 只做大小写重命名）。"
    "引用它的 handler 在 fresh DB 上会 500；修法需先决定这些表归谁建，"
    "待有库轮次（本机无 PG，盲建表风险 > 收益）"
)
REASON_MISSING_COLUMN = (
    "迁移缺列：handler 引用但 migrations 未建（与 FI-002 同类）。"
    "fresh DB 上该 SQL 必 500；修法按 101 的范式补幂等 ADD COLUMN，"
    "但列类型须对 live DB 复核，待有库轮次"
)
# 视图投影列（P1-1 登记，2026-10-26）："PP_E_APPLICATION"（112 建的视图）
# 投影 xid AS xkey / xid AS xterminal，handler（surface:887/975）经大小写
# 折叠把引用归属到 pp_e_application 表（表无这两列）。扫描器宇宙只建模表、
# 不建模视图的关系名大小写（B-2），故这两对按「视图投影列」登记基线；
# 视图若被删/改，由 P1-2「引号大写标识符必须在 migrations 有对应 CREATE」
# 守卫负责——两把尺子分工，不互相依赖。
VIEW_PROJECTED = {
    ("pp_e_application", "xkey"),
    ("pp_e_application", "xterminal"),
}
REASON_VIEW_COLUMN = (
    "视图投影列：migrations/112 建的 \"PP_E_APPLICATION\" 视图投影 "
    "xid AS xkey / xid AS xterminal；handler 引用经大小写折叠归属到 "
    "pp_e_application 表（表无此列，属视图独有）。表的列检查不受影响；"
    "视图存在性由 P1-2 引号大写标识符守卫把守"
)


def write_baseline(violations: dict[tuple[str, str], list[str]], universe: dict[str, set[str]]) -> None:
    BASELINE.parent.mkdir(parents=True, exist_ok=True)
    lines = [
        "# FI-002 静态列扫描基线（2026-10-25 建立）",
        "#",
        "# 语义：本文件登记的 (table, column) 对是**已人工复核、暂未修**的已知缺口。",
        "# 棘轮：新增未知对 → CI 红（scripts/fi002_column_scan.py 退出 1）；",
        "#       缺口被修掉 → 提示收敛（复核后 --update-baseline）。",
        "# 修完一条就从基线删一条；**不要**为了绿而往这里加新条目。",
        "#",
        "# 格式：table<TAB>column<TAB>首条证据<TAB>复核理由",
        "# 由 scripts/fi002_column_scan.py --update-baseline 生成，勿手改证据列。",
    ]
    legacy = missing = view = 0
    for (table, col) in sorted(violations):
        evidence = violations[(table, col)][0]
        if table not in universe:
            reason = REASON_LEGACY_TABLE
            legacy += 1
        elif (table, col) in VIEW_PROJECTED:
            reason = REASON_VIEW_COLUMN
            view += 1
        else:
            reason = REASON_MISSING_COLUMN
            missing += 1
        lines.append(f"{table}\t{col}\t{evidence}\t{reason}")
    lines.append("")
    lines.append(
        f"# 合计 {len(violations)} 对：遗留表 {legacy}、迁移缺列 {missing}、视图投影列 {view}"
    )
    BASELINE.write_text("\n".join(lines) + "\n", encoding="utf-8")


def main() -> int:
    ap = argparse.ArgumentParser(description="FI-002 静态列扫描")
    ap.add_argument("--report", action="store_true", help="打印全部未知对（含基线内）")
    ap.add_argument("--update-baseline", action="store_true", help="重写基线文件")
    args = ap.parse_args()

    universe, warnings = build_universe()
    total_cols = sum(len(v) for v in universe.values())
    violations, stats = scan_code(universe)

    print(
        f"[fi002] schema 宇宙：{len(universe)} 表 / {total_cols} 列"
        f"（{len(list(MIGRATIONS.glob('*.sql')))} 个迁移文件）"
    )
    print(
        f"[fi002] 扫描：{stats['files']} 文件 / {stats['literals']} SQL 字面量 / "
        f"{stats['pairs']} 列引用；跳过：未解析限定符 {stats['unresolved']}、"
        f"多表裸列 {stats['ambiguous']}、动态模板 {stats['dynamic']}、"
        f"测试文件引用 {stats['test_refs']}、动态模板内未知表 {stats['dynamic_unknown_table']}"
    )
    if warnings:
        for w in warnings[:10]:
            print(f"[fi002] 解析告警：{w}", file=sys.stderr)
        if len(warnings) > 10:
            print(f"[fi002] 另有 {len(warnings) - 10} 条解析告警（略）", file=sys.stderr)

    if args.update_baseline:
        write_baseline(violations, universe)
        print(f"[fi002] 基线已重写：{BASELINE.relative_to(BACKEND)}（{len(violations)} 对）")
        return 0

    baseline = load_baseline()
    current = set(violations)
    new = sorted(current - baseline)
    resolved = sorted(baseline - current)

    if args.report:
        for (table, col) in sorted(current):
            mark = "基线内" if (table, col) in baseline else "**新增**"
            print(f"  [{mark}] {table}.{col}  {violations[(table, col)][0]}")

    if resolved:
        print(
            f"[fi002] 基线可收敛：{len(resolved)} 对已不再出现"
            f"（如 {', '.join(f'{t}.{c}' for t, c in resolved[:5])}）——"
            "复核后用 --update-baseline 收敛",
            file=sys.stderr,
        )

    if new:
        print(f"\n[fi002] FAIL：{len(new)} 对新增未知 (table, column)", file=sys.stderr)
        for table, col in new:
            print(f"  {table}.{col}  {violations[(table, col)][0]}", file=sys.stderr)
        print(
            "\n修复：补迁移加列 / 改 SQL 拼写 / 若为扫描器误报则改进脚本或加基线并写明理由",
            file=sys.stderr,
        )
        return 1

    print(f"[fi002] OK：{len(current)} 对未知（均在基线内，无新增）")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
