#!/usr/bin/env python3
# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later
"""L-REST 无界 SELECT 静态普查 + 基线棘轮。

## 为什么有这个脚本

10-26 第四轮的 L 批次只治理了 8 处热点列表，commit message 里另称「跳过 14 处」，
但逐条名册没有落盘，且分类计数相加为 16 ≠ 14（见 docs/plans/2026-10-27-gap-closure-round.md §3）。
本轮不再补写那份名册，改用**全量静态普查 + 基线棘轮**回答同一个问题：

- 当前产品码里还有多少条**没有任何上界**的 SELECT？
- 新增一条时，CI 能不能当场红？
- 修掉一条时，基线能不能收敛而不是装作没看见？

## 口径（与 FI-002 同源）

扫描面复用 `fi002_column_scan.py` 的 `SRC_ROOTS` 与 `rust_literals()`：

- 只扫 `crates/*/src` 与 `src/` 的 Rust 字符串字面量；
- `#[cfg(test)]` 模块、`tests/`、`*_test.rs` / `*_tests.rs` 不入账（测试夹具不是产品 SQL）；
- 多条 SQL 拼在一个字面量里时按整段判型，不拆成「每段都算一条」。

按顶层形状分桶：

| 桶 | 判据 | 是否治理对象 |
|---|---|---|
| bounded | 有 `LIMIT <n|$n|?|{…}>` 或 `FETCH FIRST n ROWS` | 否 |
| aggregate | 顶层投影只有 COUNT/EXISTS/SUM/AVG/MIN/MAX（含 `COALESCE(MAX(...))`） | 否 |
| single_row | WHERE 对身份键 `id` / `xid` / `unique_id` 做等值绑定，或同时绑定 ≥2 个 `*_id`（复合身份） | 否 |
| parent_key | WHERE 只对**一个**非身份 `*_id`（外键/父键）做等值绑定 | **是**（逐端点决策） |
| key_set | WHERE id IN (...) / = ANY(...) | 否 |
| scalar | 无 FROM 的标量查询，或 `SELECT 1` 存在性探针 | 否 |
| subquery | INSERT/UPDATE/DELETE ... SELECT | 否 |
| unbounded | 其余 | **是** |

### 已知边界（不藏着）

1. `single_row` 只认**身份键约定**：`id` / `xid` / `unique_id`，或同时绑定 ≥2 个
   `*_id`（复合身份，如 `doc_id + person_id`）。`WHERE work_id = $1` / `person_id = $1`
   这类「按父键取子列表」归 `parent_key`，与 `unbounded` 一样进基线棘轮。
   为什么不按数据库主键/唯一键判：历史表大量**没有主键/唯一约束**（例如
   `x_attendance_config` 只有普通索引），schema 无法替静态扫描背书；身份键是应用层
   的稳定约定，复合身份用「同时绑定 ≥2 个 `*_id`」的形状近似。两者都不是数据库
   唯一性证明，真正的唯一性仍需数据治理（见 OA-P1-3 备注）。
2. `parent_key` 不等于「已批准无界」：它只表示「形状是父键取子列表，需要逐端点
   决策」。新增一条 `parent_key` 同样会让 CI 红，必须在基线里显式登记理由。
3. `format!` 里的动态表名/列名只解析静态部分；无法判定的整条 SQL 不会静默消失，
   而是按 `other` / 其余桶如实计数。
4. `LIMIT {}` / `LIMIT {n}` 是 Rust 格式化占位符，运行期必然有界，归 `bounded`。
5. `LIMIT $1` 也归 `bounded`：参数由调用方给，脚本不猜它会不会是 10^9；这是
   「SQL 形状有上界」与「上界取值是否合理」两件事，后者不在静态扫描能力内。
6. 带 `GROUP BY` 的投影（如 `SELECT status, COUNT(*) ... GROUP BY status`）目前不算
   `aggregate`，因为投影不只有聚合函数；它若同时带父键等值会被归 `parent_key`，
   否则归 `unbounded`。这是保守方向：宁可多登记，不静默放行。

## 基线

`docs/audits/artifacts/limit_unbounded.tsv` 登记 `unbounded` 与 `parent_key` 的计数：

    file<TAB>bucket<TAB>count<TAB>lines<TAB>sample_sql

**判定口径（端点级）**：同一文件内按 `端点标识 + 限额键` 去重后再计数——

- 端点标识：SQL 字面量所属的 `fn` 名；模块级 `const` 里的 SQL（如抽出去的
  `CREATOR_OWNER_PREDICATE`）先解析它的引用点，归到引用它的那个 `fn`；
- 限额键：`unbounded` / `parent_key`（两者都没有 LIMIT，按桶区分）。
- 分类前先内联 `format!` 里的谓词常量（`WHERE {CREATOR_OWNER_PREDICATE}`、
  `WHERE {}` + `u2_helpers::CREATOR_OWNER_PREDICATE`）：否则「把归属谓词抽成
  const」会把 `id = $1` 藏起来，单行查询被误判成 unbounded。

这样「把同一段谓词抽成 const」「文件里插入/删除行」都不会改变计数；
`lines` 列只作定位提示，**不参与判定**。

- 新文件出现 `unbounded` / `parent_key`，或既有文件端点数增加 → 退出 1；
- 端点数减少 → 打印收敛提示，退出 0；人工复核后 `--update-baseline` 收紧；
- 总字面量 / 文件数 / 各桶低于地板 → 退出 1，防止「扫描器坏了但没人发现」。
- 存量基线是旧的字面量口径，仅作**上限**使用：端点口径只会更少，不会因此误红。

## 用法

    python scripts/limit_scan.py                 # 棘轮模式（CI 用）
    python scripts/limit_scan.py --report        # 打印全部 unbounded（含基线内）
    python scripts/limit_scan.py --update-baseline  # 人工复核后重写基线
"""

from __future__ import annotations

import argparse
import collections
import re
import sys
from pathlib import Path

SCRIPT_DIR = Path(__file__).resolve().parent
if str(SCRIPT_DIR) not in sys.path:
    sys.path.insert(0, str(SCRIPT_DIR))

import fi002_column_scan as fi002  # noqa: E402

BACKEND = fi002.BACKEND
BASELINE = BACKEND / "docs" / "audits" / "artifacts" / "limit_unbounded.tsv"

SELECT_RE = re.compile(r"^\s*(?:/\*.*?\*/\s*)*(select|with)\b", re.I | re.S)
DML_SELECT_RE = re.compile(r"^\s*(insert|update|delete)\b.*\bselect\b", re.I | re.S)
LIMIT_RE = re.compile(
    r"\blimit\s+(?:\$\d+|\d+|\?|\{\s*\w*\s*\}|[a-z_][a-z0-9_]*\s*\()", re.I
)
FETCH_RE = re.compile(r"\bfetch\s+first\s+(?:\d+|\$\d+|\?)\s+rows?", re.I)
AGG_ONLY_RE = re.compile(
    r"^\s*select\s+(?:distinct\s+)?(?:coalesce\s*\(\s*)?(?:count|exists|sum|avg|min|max)\s*\(",
    re.I | re.S,
)
IDENTITY_EQ_RE = re.compile(r"\b(?:id|xid|unique_id)\s*=\s*(\$\d+|'[^']*'|\d+)", re.I)
EQ_KEY_RE = re.compile(r"\b(?:[a-z_]*id)\s*=\s*(\$\d+|'[^']*'|\d+)", re.I)
KEY_SET_RE = re.compile(r"\b(?:[a-z_]*id)\s+(?:in\s*\(|= any\s*\()", re.I)
#: `SELECT 1 FROM ... WHERE name = $1` 是存在性探针，不是列表；它返回几行取决于
#: 唯一约束，但调用方只读「有没有」，加 LIMIT 1 不改变语义。
PROBE_RE = re.compile(r"^\s*select\s+1\b", re.I)
#: 端点归因用：SQL 字面量属于哪个 fn / const。
FN_DECL_RE = re.compile(r"\bfn\s+([a-z_][a-z0-9_]*)", re.I)
CONST_DECL_RE = re.compile(r"\bconst\s+([A-Z_][A-Z0-9_]*)\s*:", re.I)
#: `format!("... WHERE {PRED}", ...)` 里的谓词常量：分类前先内联，否则
#: `WHERE {CREATOR_OWNER_PREDICATE}` 会被当成「没有谓词」误判为 unbounded。
CONST_STR_DEF_RE = re.compile(
    r'\bconst\s+([A-Z_][A-Z0-9_]*)\s*:\s*&str\s*=\s*"((?:[^"\\]|\\.)*)"', re.S
)
FORMAT_NAMED_RE = re.compile(r"\{([A-Za-z_][A-Za-z0-9_]*)\}")
#: 位置参数 `{}` 的实参：附近出现的全大写常量名（如 `u2_helpers::CREATOR_OWNER_PREDICATE`）。
CONST_REF_RE = re.compile(r"\b[A-Z][A-Z0-9_]{3,}\b")

#: 文件级声明表与 const 引用点解析的缓存，避免每个字面量重读整文件。
_DECL_CACHE: dict[Path, tuple[list[tuple[int, str]], dict[str, int]]] = {}
_CONST_FN_CACHE: dict[tuple[Path, str], str] = {}
_FILE_LINES_CACHE: dict[Path, list[str]] = {}
_PREDICATE_CONSTS: dict[str, str] = {}

#: 扫描器自证地板：低于这些值说明正则/扫描面坏了，不是「代码变干净了」。
TOTAL_FLOOR = 2600
FILE_FLOOR = 120
BUCKET_FLOORS = {
    "bounded": 450,
    "aggregate": 200,
    "single_row": 900,
    "parent_key": 65,
    "key_set": 80,
    "scalar": 15,
    "subquery": 20,
    "unbounded": 550,
}

#: 进基线棘轮的桶。`parent_key` 与 `unbounded` 都是「需要逐端点决策」，
#: 区别只在形状：前者是父键取子列表，后者连父键都没有。
TRACKED_BUCKETS = ("parent_key", "unbounded")


def classify(sql: str) -> str:
    """一条 SQL 字面量 → 桶名。判序见模块 docstring。"""
    s = " ".join(sql.split())
    if DML_SELECT_RE.search(s):
        return "subquery"
    if not SELECT_RE.search(s):
        return "other"
    if LIMIT_RE.search(s) or FETCH_RE.search(s):
        return "bounded"
    if AGG_ONLY_RE.search(s):
        return "aggregate"
    if PROBE_RE.search(s) or not re.search(r"\bfrom\b", s, re.I):
        return "scalar"
    where = re.split(r"\b(?:group\s+by|order\s+by|having)\b", s, maxsplit=1, flags=re.I)[0]
    if IDENTITY_EQ_RE.search(where):
        return "single_row"
    # 复合身份：同时绑定两个及以上 *_id（如 doc_id + person_id）。
    # 这是形状近似，不是数据库唯一性证明；边界写在模块 docstring。
    parent_cols = {
        m.lower()
        for m in re.findall(r"\b([a-z_]*id)\s*=\s*(?:\$\d+|'[^']*'|\d+)", where, re.I)
    }
    if len(parent_cols) >= 2:
        return "single_row"
    if EQ_KEY_RE.search(where):
        return "parent_key"
    if KEY_SET_RE.search(where):
        return "key_set"
    return "unbounded"


def _file_lines(path: Path) -> list[str]:
    cached = _FILE_LINES_CACHE.get(path)
    if cached is None:
        cached = path.read_text(encoding="utf-8", errors="ignore").splitlines()
        _FILE_LINES_CACHE[path] = cached
    return cached


def _predicate_consts() -> dict[str, str]:
    """工作区内 `const X: &str = "..."` 的常量体；按名首次出现为准。"""
    if _PREDICATE_CONSTS:
        return _PREDICATE_CONSTS
    for path in sorted(BACKEND.glob("crates/**/*.rs")):
        for m in CONST_STR_DEF_RE.finditer(
            path.read_text(encoding="utf-8", errors="ignore")
        ):
            name, body = m.group(1), m.group(2)
            # Rust 的 `\` 行续接会留在字符串体里，归一成空格再分类。
            body = re.sub(r"\\\s*\n\s*", " ", body)
            _PREDICATE_CONSTS.setdefault(name, body)
    return _PREDICATE_CONSTS


def _looks_like_predicate(body: str) -> bool:
    """只内联「像 WHERE 谓词」的常量，避免把表名/整段 SQL 常量塞进来误判。"""
    return "=" in body and bool(
        re.search(r"\b(where|and|exists|creator|owner)\b", body, re.I)
    )


def _resolve_placeholders(sql: str, path: Path, line: int) -> str:
    """把 `format!` 里的谓词常量内联回 SQL，供分类使用（不改写原始字面量）。"""
    if "{" not in sql:
        return sql
    consts = _predicate_consts()
    sql = FORMAT_NAMED_RE.sub(
        lambda m: consts[m.group(1)]
        if m.group(1) in consts and _looks_like_predicate(consts[m.group(1)])
        else m.group(0),
        sql,
    )
    if "{}" not in sql:
        return sql
    lines = _file_lines(path)
    for offset in range(0, 18):
        idx = line - 1 + offset
        if idx >= len(lines):
            break
        for cand in CONST_REF_RE.findall(lines[idx]):
            body = consts.get(cand)
            if body and _looks_like_predicate(body):
                return sql.replace("{}", body, 1)
    return sql


def _declarations(path: Path) -> tuple[list[tuple[int, str]], dict[str, int]]:
    """`([(行号, fn 名)], {const 名: 行号})`；按文件缓存。"""
    cached = _DECL_CACHE.get(path)
    if cached is not None:
        return cached
    fns: list[tuple[int, str]] = []
    consts: dict[str, int] = {}
    for lineno, line in enumerate(
        path.read_text(encoding="utf-8", errors="ignore").splitlines(), 1
    ):
        m = FN_DECL_RE.search(line)
        if m:
            fns.append((lineno, m.group(1)))
        m = CONST_DECL_RE.search(line)
        if m:
            consts.setdefault(m.group(1), lineno)
    _DECL_CACHE[path] = (fns, consts)
    return fns, consts


def _nearest_fn(fns: list[tuple[int, str]], line: int) -> str:
    name = ""
    for decl_line, fn_name in fns:
        if decl_line > line:
            break
        name = fn_name
    return name


def _const_endpoint(path: Path, const_name: str, const_line: int, fns: list[tuple[int, str]]) -> str:
    """模块级 const 里的 SQL 归到引用它的 fn；找不到引用就记 const 名。"""
    key = (path, const_name)
    cached = _CONST_FN_CACHE.get(key)
    if cached is not None:
        return cached
    endpoint = f"const:{const_name}"
    for lineno, line in enumerate(
        path.read_text(encoding="utf-8", errors="ignore").splitlines(), 1
    ):
        if lineno == const_line:
            continue
        if re.search(rf"\b{re.escape(const_name)}\b", line):
            fn = _nearest_fn(fns, lineno)
            if fn:
                endpoint = fn
                break
    _CONST_FN_CACHE[key] = endpoint
    return endpoint


def endpoint_of(path: Path, line: int) -> str:
    """字面量 → 端点标识：就近的 fn；若最近声明是 const，则解析 const 的引用点。"""
    fns, consts = _declarations(path)
    fn_line, fn_name = -1, ""
    for decl_line, name in fns:
        if decl_line > line:
            break
        fn_line, fn_name = decl_line, name
    const_line, const_name = -1, ""
    for name, decl_line in consts.items():
        if decl_line <= line and decl_line > const_line:
            const_line, const_name = decl_line, name
    if const_line > fn_line:
        return _const_endpoint(path, const_name, const_line, fns)
    return fn_name or f"literal@{line}"


def collect() -> list[tuple[str, int, str, str, str]]:
    """`[(相对路径, 行号, 桶, 规整后 SQL, 端点标识), ...]`。"""
    rows = []
    for root in fi002.SRC_ROOTS:
        if not root.is_dir():
            continue
        for path in sorted(root.rglob("*.rs")):
            rel = str(path.relative_to(BACKEND)).replace("\\", "/")
            if (
                path.name == "tests.rs"
                or "/tests/" in rel
                or path.name.endswith("_test.rs")
                or path.name.endswith("_tests.rs")
            ):
                continue
            for line, body in fi002.rust_literals(path):
                if not SELECT_RE.search(body) and not DML_SELECT_RE.search(body):
                    continue
                rows.append(
                    (
                        rel,
                        line,
                        classify(_resolve_placeholders(body, path, line)),
                        " ".join(body.split()),
                        endpoint_of(path, line),
                    )
                )
    return rows


def load_baseline() -> dict[tuple[str, str], int]:
    """基线 → `{(file, bucket): count}`；注释与空行忽略。"""
    if not BASELINE.is_file():
        return {}
    expected = {}
    for raw in BASELINE.read_text(encoding="utf-8").splitlines():
        if not raw or raw.startswith("#"):
            continue
        parts = raw.split("\t")
        if len(parts) < 3:
            raise SystemExit(f"基线行格式不对（需要至少 3 列）：{raw!r}")
        expected[(parts[0], parts[1])] = int(parts[2])
    return expected


def summarize(rows: list[tuple[str, int, str, str, str]]):
    """按 `(文件, 端点标识, 桶)` 去重；同一端点的多段 SQL 只算一个风险点。"""
    by_bucket = collections.Counter(row[2] for row in rows)
    tracked: dict[tuple[str, str], dict[str, tuple[int, str]]] = collections.defaultdict(dict)
    for rel, line, bucket, sql, endpoint in rows:
        if bucket in TRACKED_BUCKETS:
            tracked[(rel, bucket)].setdefault(endpoint, (line, sql))
    return by_bucket, tracked


def write_baseline(tracked: dict[tuple[str, str], dict[str, tuple[int, str]]]) -> None:
    BASELINE.parent.mkdir(parents=True, exist_ok=True)
    lines = [
        "# L-REST 无界 / 父键 SELECT 基线（2026-10-27 建立，2026-10-28 拆出 parent_key）",
        "#",
        "# 语义：本文件登记的是**已人工复核、暂未加 LIMIT** 的 SELECT 端点级计数：",
        "#   unbounded  —— 顶层没有 LIMIT，也没有可用于单行/父键判定的谓词；",
        "#   parent_key —— WHERE 只绑定一个非身份 *_id（父键取子列表），需逐端点决策。",
        "# 两者都不是「已批准无界」：新增即红，必须在基线里显式登记并写明理由。",
        "# 计数口径：同文件内按「端点标识(fn 名) + bucket」去重；功能等价的 const 抽取不增计数。",
        "# 棘轮：新文件出现，或既有文件端点计数增加 → CI 红（scripts/limit_scan.py 退出 1）；",
        "#       计数减少 → 提示收敛（复核后 --update-baseline 收紧，不要把减少当成失败）。",
        "# 修完一条就从基线删一条；**不要**为了绿而往这里加新条目。",
        "#",
        "# 格式：file<TAB>bucket<TAB>count<TAB>lines<TAB>sample_sql",
        "# lines 列只作定位提示，不参与判定；由 scripts/limit_scan.py --update-baseline 生成。",
        "",
    ]
    total = 0
    for (rel, bucket), items in sorted(tracked.items()):
        reps = list(items.values())
        lines.append(
            "\t".join(
                [
                    rel,
                    bucket,
                    str(len(items)),
                    ",".join(str(line) for line, _ in reps),
                    reps[0][1][:200],
                ]
            )
        )
        total += len(items)
    lines.append("")
    lines.append(f"# 合计 {total} 条（unbounded + parent_key），{len(tracked)} 个文件")
    BASELINE.write_text("\n".join(lines) + "\n", encoding="utf-8")


def main() -> int:
    ap = argparse.ArgumentParser(description="L-REST 无界 SELECT 静态普查")
    ap.add_argument("--report", action="store_true", help="打印全部 unbounded（含基线内）")
    ap.add_argument("--update-baseline", action="store_true", help="重写基线（人工复核后）")
    args = ap.parse_args()

    rows = collect()
    by_bucket, tracked = summarize(rows)
    total = len(rows)
    files = len({row[0] for row in rows})
    print(f"SELECT 字面量 {total} 条 / {files} 个文件")
    print("分桶：" + "  ".join(f"{k}={v}" for k, v in sorted(by_bucket.items())))

    if total < TOTAL_FLOOR or files < FILE_FLOOR:
        print(
            f"::error::扫描面异常：total={total}（地板 {TOTAL_FLOOR}），files={files}（地板 {FILE_FLOOR}）"
        )
        return 1
    for bucket, floor in BUCKET_FLOORS.items():
        if by_bucket.get(bucket, 0) < floor:
            print(f"::error::桶 {bucket} 只剩 {by_bucket.get(bucket, 0)} 条（地板 {floor}），扫描器可能坏了")
            return 1

    if args.update_baseline:
        write_baseline(tracked)
        print(
            f"已重写基线：{BASELINE}（unbounded {by_bucket.get('unbounded', 0)} 条，"
            f"parent_key {by_bucket.get('parent_key', 0)} 条）"
        )
        return 0

    baseline = load_baseline()
    if not baseline:
        print(f"::error::基线不存在或为空：{BASELINE}")
        return 1

    problems = []
    for key, items in sorted(tracked.items()):
        expected = baseline.get(key, 0)
        if len(items) > expected:
            locs = ", ".join(f"{endpoint}@{line}" for endpoint, (line, _) in items.items())
            problems.append(
                f"{key[0]}：{key[1]} 端点 {len(items)} 个 > 基线 {expected} 条（旧字面量口径）；"
                f"定位（行号仅供参考）：{locs}"
            )
    for (rel, bucket), expected in sorted(baseline.items()):
        actual = len(tracked.get((rel, bucket), {}))
        if actual < expected:
            print(
                f"::notice::{rel} 的 {bucket} 从 {expected} 降到 {actual}（端点口径）；"
                "复核后 --update-baseline 收紧"
            )

    if problems:
        print("::error::L-REST 棘轮失败：")
        for item in problems:
            print("  " + item)
        return 1

    if args.report:
        for (rel, bucket), items in sorted(tracked.items()):
            for endpoint, (line, sql) in items.items():
                print(f"{rel}:{line}\t{bucket}\t{endpoint}\t{sql}")

    print(
        f"L-REST 基线态通过：unbounded {by_bucket.get('unbounded', 0)} 条 + "
        f"parent_key {by_bucket.get('parent_key', 0)} 条（基线 {sum(baseline.values())} 条）"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
