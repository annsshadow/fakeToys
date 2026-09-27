# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""L94：A171 的机械答案 —— 「计数尺子必须先自证」从 `Temp/` 搬进仓库

**为什么这一支该住在仓库里**：L93 那轮量路径解析，顺路把**量它的那几把尺子**一起量出
四个洞（账本 A171）：① 填数脚本那道「有失败项就不该填数」的闸读的 `failed` 来自一条
**按顺序**写的正则（`passed … skipped … failed`），而 pytest 的求和行在真有失败时是
`5 failed, 7032 passed, …` —— `failed` 在 `passed` **前面**，那半个可选组向后永远找不到
⇒ `failed` **恒为 0**，闸从写下那天起就不可能失败；② 数「本轮新增守卫」时按空格切
collect 行的节点 id，把带空格的参数化 id 劈成前缀 ⇒ 集合凭空少 45 条；③ 数调用点的 AST
尺子把 `ast.Name` 的字段读成 `.name`（真名 `id`）⇒ 几十处裸调用静默漏光、返回 `0 / 0`，
而这两个 0 **已经填进了账本正文**；④ 逐文件覆盖差按 `endswith("api/deps.py")` 匹配，而
键的真实形状是 `deps.py` ⇒ 读不到以后走了 fallback 分支，把一次真实的 +7 语句 / +4 分支
出口讲成「一格未动」。四格同族，且与 A165、A166 同族：**尺子读不出东西时返回的是合法
形状的 0 / None，于是判据不红、账本会**。

L93 的处置只改了自己那一支脚本 —— 而脚本住在 `Temp/`，永不入库，下一轮「抄上一轮的填数
脚本」就会把同一个恒真闸原样抄回去。本轮改机制：尺子与它的自检一起进仓库，并且**自检写在
每一次读数里**（每个公开读数函数先拿已知答案的样本跑一遍，不对就抛 `RulerNotProven`），
所以「没自证过就用它读数」这种形状在代码层面不可表达；缓存自检结果会被判为缺陷
（`test_no_public_ruler_skips_the_self_proof` 就是钉这一格的）。

四支尺子与四个洞一一对应：`footer_counts` + `failed_node_ids` + `assert_meshed`（①，按
标签读数并与短表点名互相咬合）、`collect_node_ids`（②，整行收，不按空白切）、
`count_calls` / `count_calls_in_dir`（③，AST 且带「全仓读不到任何调用点即抛」的下限）、
`file_stats` + `by_basename`（④，按 coverage XML 自己的 `filename` 建键，读不到抛
`NoMatch`、裸名撞多支抛 `AmbiguousName`，**没有返回 None 让调用方走 fallback 的那一支**）。
"""

import ast
import inspect
import re
import xml.etree.ElementTree as ET
from pathlib import Path
from typing import Dict, List, Tuple

import pytest


class RulerNotProven(RuntimeError):
    """尺子的已知答案样本对不上：这把尺子此刻不可信，禁止用它读数"""


class NoMatch(LookupError):
    """尺子读不到那一格 —— 这与「读到了 0」是两件不同的事（A171 的本体）"""


class AmbiguousName(LookupError):
    """裸名命中多支：与 `test_doc_line_refs_l79.py` 的 `ambiguous_file` 桶同一条口径"""

#: 求和行里可能出现的计数标签。写全是为了让「没被认出的标签」在闭合式里露出来。
COUNT_LABELS = ("passed", "skipped", "failed", "error", "errors",
                "deselected", "xfailed", "xpassed", "warnings")

#: 短表点名行：只按第一个空格切，节点 id 本身可以带空格。
FAILED_LINE = re.compile(r"^(?:FAILED|ERROR) (\S+)", re.MULTILINE)


# ------------------------------------------------------------------ ① 求和行
def _parse_footer(text: str) -> Dict[str, object]:
    lines = [l for l in text.splitlines() if " passed" in l and " in " in l]
    if not lines:
        raise NoMatch("读不到求和行：这份输出没跑完，「读不到」不是「零失败」")
    line = lines[-1]
    out: Dict[str, object] = {}
    for label in COUNT_LABELS:
        m = re.search(r"(\d[\d,]*) " + label + r"\b", line)
        if m:
            out[label] = int(m.group(1).replace(",", ""))
    m = re.search(r" in ([\d.]+)s", line)
    if not m:
        raise NoMatch(f"求和行读不到耗时：{line!r}")
    out["duration"] = m.group(1)
    out["line"] = line
    return out


def footer_counts(text: str) -> Dict[str, object]:
    """从 pytest 的求和行**按标签**读数，绝不按出现顺序

    Args:
        text: 一份完整的 pytest 输出

    Returns:
        出现过的计数标签（`passed` / `skipped` / `failed` …）加 `duration` 与 `line`；
        全绿时 `failed` 这个键**不存在**（不是 0），所以读它要带 `.get(k, 0)`

    Raises:
        RulerNotProven: 已知答案的样本读不对
        NoMatch: 输出里没有求和行
    """
    got = _parse_footer(FOOTER_SAMPLE)
    for label, want in FOOTER_ANSWER.items():
        if got.get(label) != want:
            raise RulerNotProven(f"求和行尺子读错 {label}：{got.get(label)} != {want}")
    if len(failed_node_ids(FOOTER_SAMPLE)) != 3:
        raise RulerNotProven("短表点名尺子读错：应点名 3 条")
    return _parse_footer(text)


def failed_node_ids(text: str) -> set:
    """短表里被点名的失败/错误项（`FAILED path::name` / `ERROR path::name`）

    Args:
        text: 一份完整的 pytest 输出

    Returns:
        节点 id 集合
    """
    return set(FAILED_LINE.findall(text))


def assert_meshed(counts: Dict[str, object], ids: set, collected: int = None) -> None:
    """两路**独立**读数必须互相咬合：求和行的失败数 == 短表点名的条数

    这不是装饰。L93 抓到①号洞靠的就是它：同一份 l92 输出，求和行那路读 `0`、短表那路
    点名 5 条 ⇒ 当场报 `0 vs 5`。任何单一读数都能被自己的正则骗过去，两路同时骗过去的
    概率低得多。给了 `collected` 就顺带验闭合式，于是「出现了本尺子不认识的标签」也无法
    被当成干净读数。

    Args:
        counts: `footer_counts` 的返回
        ids: `failed_node_ids` 的返回
        collected: 可选的 `--collect-only` 出口数

    Raises:
        AssertionError: 两路不咬合，或闭合式对不上
    """
    n_fail = sum(int(counts.get(k, 0)) for k in ("failed", "error", "errors"))
    assert n_fail == len(ids), f"求和行失败数 {n_fail} 与短表点名 {len(ids)} 不符"
    if collected is not None:
        total = sum(int(counts.get(k, 0))
                    for k in ("passed", "skipped", "failed", "error", "errors"))
        assert total == collected, f"{total} != collected {collected}"


# ---------------------------------------------------------------- ② collect
def _parse_collect(text: str) -> set:
    return {l.strip() for l in text.splitlines() if "::" in l}


def collect_node_ids(text: str) -> set:
    """`pytest --collect-only -q` 的快照按**整行**收成节点 id 集合

    参数化 id 里带空格的到处都是（`test_x[   ]`、`test_y[n-gram 长度]`），所以「按空白切
    一行取第一段」会把不同用例劈成同一个前缀 —— L93 实测凭空少 45 条，发现它的唯一原因
    是 `len(集合) == 出口数` 这一格。用法：拿到集合后**必须**与 collected 对数。

    Args:
        text: collect 快照原文

    Returns:
        含 `::` 的行的去重集合

    Raises:
        RulerNotProven: 已知答案的样本读不对
    """
    if len(_parse_collect(COLLECT_SAMPLE)) != COLLECT_ANSWER:
        raise RulerNotProven("collect 尺子按行收不干净：带空格的参数化 id 被切了")
    return _parse_collect(text)


# ---------------------------------------------------------------- ③ AST 调用
def _call_name(node: ast.Call):
    """取调用名：`ast.Name` 的字段是 `id`，`ast.Attribute` 才是 `attr`

    单列成函数是因为 `.name` 那个错误形状对两种节点都返回 `None` —— 它不报错，只是安静
    地数出 0。
    """
    f = node.func
    if isinstance(f, ast.Name):
        return f.id
    if isinstance(f, ast.Attribute):
        return f.attr
    return None


def _tally(src: str, name: str, keyword: str, nonbool: List[str]) -> Dict[str, int]:
    out = {"true": 0, "other": 0}
    for node in ast.walk(ast.parse(src)):
        if not isinstance(node, ast.Call) or _call_name(node) != name:
            continue
        kw = next((k for k in node.keywords if k.arg == keyword), None)
        val = getattr(kw.value, "value", "«not-a-literal»") if kw is not None else None
        out["true" if val is True else "other"] += 1
        if kw is not None and val == "«not-a-literal»":
            nonbool.append(ast.unparse(node))
    return out


def count_calls(src: str, name: str = "resolve_data_path",
                keyword: str = "for_write") -> Dict[str, int]:
    """按 AST 数某个函数的调用点，带 `keyword=True` **字面量**者单列

    文本 grep 不行：`api/deps.py` 的 docstring 自己就在引用那行代码，按文本数会把注释数成
    调用点（A165 那一族）。

    Args:
        src: 一段 Python 源码
        name: 目标函数名（裸调用与 `obj.name(...)` 都算）
        keyword: 布尔旋钮的关键字名

    Returns:
        `{"true": 带 keyword=True 字面量的, "other": 其余}`

    Raises:
        RulerNotProven: 已知答案的样本读不对
    """
    if _tally(CALLS_SAMPLE, "resolve_data_path", "for_write", []) != CALLS_ANSWER:
        raise RulerNotProven(
            f"AST 调用点尺子读错：{_tally(CALLS_SAMPLE, 'resolve_data_path', 'for_write', [])}"
            f" != {CALLS_ANSWER}")
    return _tally(src, name, keyword, [])


def count_calls_in_dir(root, name: str = "resolve_data_path",
                       keyword: str = "for_write", pattern: str = "*.py"):
    """递归数一个目录里的调用点，并回报哪些是变量而非字面量

    空目录不跑循环，所以读数前先经 `count_calls` 走一次自检：自检挂在读数上，就不存在
    「忘了自证」这一格。

    Args:
        root: 目录
        name: 目标函数名
        keyword: 布尔旋钮的关键字名
        pattern: 文件通配

    Returns:
        `(写点数, 读点数, 非字面量旋钮的调用原文列表)`

    Raises:
        RulerNotProven: 尺子自检不过
        NoMatch: 一支调用点都没数到 —— 产品码里不可能一个都没有，那是探针坏了（A171 ③）
    """
    count_calls("")
    write = read = 0
    nonbool: List[str] = []
    for path in sorted(Path(root).rglob(pattern)):
        got = count_calls(path.read_text(encoding="utf-8", errors="ignore"), name, keyword)
        write += got["true"]
        read += got["other"]
    if write == 0 and read == 0:
        raise NoMatch(f"{root} 里一个 {name}(...) 调用点都没数到：先怀疑探针，别把 0 填进账本")
    return write, read, nonbool


# -------------------------------------------------------------- ④ 覆盖 XML
def _parse_coverage(xml_text: str) -> Dict[str, Tuple[int, int, int, int]]:
    out: Dict[str, Tuple[int, int, int, int]] = {}
    for cls in ET.fromstring(xml_text).iter("class"):
        els = cls.findall("lines/line")
        if not els:
            continue
        if cls.get("filename") in out:
            raise AmbiguousName(f"XML 里 {cls.get('filename')} 出现两次：键会互相覆盖")
        stmts = len(els)
        missing = sum(1 for e in els if int(e.get("hits", "0")) == 0)
        exits = partial = 0
        for e in els:
            cc = e.get("condition-coverage")
            if not cc:
                continue
            got, _, total = cc.split("(")[1].partition("/")
            total = total.rstrip(")")
            exits += int(total)
            if int(got) < int(total):
                partial += 1
        out[cls.get("filename")] = (stmts, missing, exits, partial)
    return out


def file_stats(xml_path) -> Dict[str, Tuple[int, int, int, int]]:
    """coverage.py 的 XML 逐文件数出四元组 `(语句, 缺数, 分支出口, 偏支)`

    口径来自 `cov_diff_l91`（本仓已自证过）：TOTAL 从根属性读，**分支出口从
    `condition-coverage` 的分母读** —— 按 `branches=` 数的是分支行而不是出口，未触达行
    也根本没有 `missing` 属性，那样读数看着像满分。键用 XML 自己的 `filename`：实测本仓
    127 个 `class` 节点的 `filename` **互不相同**，而按**裸名**查会撞上 13 个名字覆盖
    35 支文件（`quality.py`×3、`export.py`×3……），所以裸名只许当查询条件、不许当键。

    Args:
        xml_path: coverage XML 的路径

    Returns:
        `filename → 四元组`

    Raises:
        RulerNotProven: 已知答案的样本读不对
        AmbiguousName: 同一个 `filename` 出现两次（合并即丢数据）
    """
    stats = _parse_coverage(COVERAGE_SAMPLE)
    if stats.get("api/deps.py") != (3, 1, 2, 1):
        raise RulerNotProven(f"覆盖 XML 尺子读错 api/deps.py：{stats.get('api/deps.py')}")
    if len([fn for fn in stats if Path(fn).name == "quality.py"]) != 2:
        raise RulerNotProven("两支 quality.py 没并存：按裸名建键会把它们合成一支")
    return _parse_coverage(Path(xml_path).read_text(encoding="utf-8", errors="ignore"))


def by_basename(stats: Dict[str, Tuple[int, int, int, int]], name: str):
    """在 `file_stats` 的结果里按裸名查一支

    Args:
        stats: `file_stats` 的返回
        name: 裸文件名，如 `deps.py`

    Returns:
        `(filename, 四元组)`

    Raises:
        NoMatch: 一支都没有 —— 这不是「没有差」，调用方必须停下来而不是走 fallback
        AmbiguousName: 多支同名，裸名不足以定位
    """
    hits = [fn for fn in stats if Path(fn).name == name]
    if not hits:
        raise NoMatch(f"覆盖差里没有 {name}：读不到 ≠ 没变化")
    if len(hits) > 1:
        raise AmbiguousName(f"{name} 命中 {len(hits)} 支：{sorted(hits)}")
    return hits[0], stats[hits[0]]


# ---------------------------------------------------------------- 已知答案
#: 求和行的形状取自 L92 的真实输出（`failed` 印在 `passed` 前面），答案是那轮的读数。
FOOTER_SAMPLE = ("=============== 5 failed, 7,032 passed, 3 skipped, 7 warnings in 91.97s =============\n"
                 "FAILED tests/a.py::x - assert 0\nFAILED tests/b.py::y - boom\nERROR tests/c.py::z\n")
FOOTER_ANSWER = {"passed": 7032, "skipped": 3, "failed": 5}

#: 两条带空格的参数化 id + 一条普通 id，按空白切就会少条。
COLLECT_SAMPLE = ("tests/a.py::test_empty_path_rejected_with_400[   ]\n"
                  "tests/a.py::test_ngram[n-gram 长度]\n"
                  "tests/b.py::test_plain\n")
COLLECT_ANSWER = 3

#: 1 写（字面量 True）+ 1 属性调用读 + 1 显式 False 读，外加 1 支别的函数。
CALLS_SAMPLE = ('resolve_data_path("a", for_write=True)\n'
                'deps.resolve_data_path("b")\n'
                'resolve_data_path("c", for_write=False)\n'
                'resolve_within_roots("d")\n')
CALLS_ANSWER = {"true": 1, "other": 2}

COVERAGE_SAMPLE = """<?xml version="1.0" ?>
<coverage lines-valid="5" lines-covered="3" branches-valid="2" timestamp="1">
  <packages><package name="api"><classes>
    <class name="deps" filename="api/deps.py"><lines>
      <line number="1" hits="1"/><line number="2" hits="0"/>
      <line number="3" hits="2" branch="true" condition-coverage="50% (1/2)"/>
    </lines></class>
    <class name="q1" filename="routes/quality.py"><lines><line number="1" hits="1"/></lines></class>
    <class name="q2" filename="quality.py"><lines><line number="1" hits="1"/></lines></class>
  </classes></package></packages>
</coverage>
"""

#: 公开读数口的名单——`test_no_public_ruler_skips_the_self_proof` 逐支检查它们带自检。
PUBLIC_RULERS = ("footer_counts", "collect_node_ids", "count_calls",
                 "count_calls_in_dir", "file_stats")


class TestTheRulersCanFail:
    """每支尺子都要能红：已知答案改一位就必须抛（A165/A171 的「恒真判据」家族）"""

    def test_footer_answer_mismatch_is_caught(self, monkeypatch):
        mod = inspect.getmodule(footer_counts)
        monkeypatch.setattr(mod, "FOOTER_ANSWER", {"passed": 7032, "skipped": 99, "failed": 5})
        with pytest.raises(RulerNotProven, match="skipped"):
            footer_counts("===== 3 passed, 3 skipped in 1.00s =====\n")

    def test_collect_answer_mismatch_is_caught(self, monkeypatch):
        mod = inspect.getmodule(collect_node_ids)
        monkeypatch.setattr(mod, "COLLECT_SAMPLE", "tests/a.py::test_plain\n")
        with pytest.raises(RulerNotProven, match="collect"):
            collect_node_ids("tests/b.py::x\n")

    def test_call_answer_mismatch_is_caught(self, monkeypatch):
        mod = inspect.getmodule(count_calls)
        monkeypatch.setattr(mod, "CALLS_ANSWER", {"true": 5, "other": 2})
        with pytest.raises(RulerNotProven, match="AST"):
            count_calls('resolve_data_path("z")\n')

    def test_coverage_answer_mismatch_is_caught(self, monkeypatch):
        mod = inspect.getmodule(file_stats)
        monkeypatch.setattr(mod, "COVERAGE_SAMPLE",
                            COVERAGE_SAMPLE.replace('condition-coverage="50% (1/2)"', ""))
        with pytest.raises(RulerNotProven, match="api/deps.py"):
            file_stats(Path(__file__))


class TestFooterRuler:
    """①号洞：求和行的 `failed` 印在 `passed` 前面，按顺序读的那半个组永远找不到"""

    def test_failed_is_read_when_it_is_printed_before_passed(self):
        got = footer_counts(FOOTER_SAMPLE)
        assert got["failed"] == 5
        assert got["passed"] == 7032
        assert got["skipped"] == 3

    def test_an_order_based_regex_is_the_defect_this_ruler_replaces(self):
        """把当年那条恒真正则留在这里当**反向证据**：它对同一份输入读 0"""
        ordered = re.search(r"(\d[\d,]*) passed.*?(\d[\d,]*) skipped.*?(\d[\d,]*) failed",
                            FOOTER_SAMPLE, re.DOTALL)
        assert not ordered, "顺序正则在这份样本上读不到 failed ⇒ 恒取默认 0，正是 A171 ①"

    def test_a_green_footer_has_no_failed_label_at_all(self):
        got = footer_counts("=========== 7037 passed, 3 skipped, 7 warnings in 220.15s ==========\n")
        assert "failed" not in got
        assert int(got.get("failed", 0)) == 0

    def test_thousands_separators_are_stripped(self):
        assert footer_counts("==== 12,345 passed, 3 skipped in 9.00s ====")["passed"] == 12345

    def test_a_run_without_a_summary_line_is_not_zero_failures(self):
        with pytest.raises(NoMatch):
            footer_counts("ERROR: usage __main__.py -c [options]\nno tests ran\n")

    def test_the_mesh_catches_a_footer_that_disagrees_with_the_short_table(self):
        counts = {"failed": 5, "passed": 3}
        with pytest.raises(AssertionError, match="短表点名"):
            assert_meshed(counts, {"tests/a.py::x"})

    def test_an_unrecognized_label_breaks_the_closure_instead_of_passing(self):
        """闭合式是「本尺子不认识的标签」的兜底：它们会让总数对不上 collected"""
        counts = footer_counts("==== 100 passed, 3 skipped, 1 rerun in 5.00s ====\n")
        with pytest.raises(AssertionError, match="collected"):
            assert_meshed(counts, set(), collected=104)


class TestCollectRuler:
    """②号洞：按空白切节点 id 会把参数化 id 劈成前缀"""

    def test_parametrized_ids_with_spaces_survive(self):
        ids = collect_node_ids(COLLECT_SAMPLE)
        assert len(ids) == COLLECT_ANSWER
        assert "tests/a.py::test_empty_path_rejected_with_400[   ]" in ids

    def test_splitting_on_whitespace_collapses_ids_that_differ_after_a_space(self):
        """反向证据：参数化 id 里空格之后的部分被切掉，两支并成一支"""
        text = ("tests/a.py::test_x[a b]\n"
                "tests/a.py::test_x[a c]\n"
                "tests/b.py::test_y\n")
        naive = {l.split()[0] for l in text.splitlines() if "::" in l}
        assert len(naive) == 2 and len(collect_node_ids(text)) == 3

    def test_non_node_lines_are_ignored(self):
        text = "7040 tests collected in 3.21s\n" + COLLECT_SAMPLE + "no tests ran.\n"
        assert len(collect_node_ids(text)) == 3


class TestCallRuler:
    """③号洞：`ast.Name` 的字段是 `id`，写成 `.name` 会静默数出 0"""

    def test_bare_and_attribute_calls_both_count(self):
        assert count_calls(CALLS_SAMPLE) == CALLS_ANSWER

    def test_a_non_literal_flag_is_not_counted_as_a_write(self):
        got = count_calls('resolve_data_path("a", for_write=flag)\n')
        assert got == {"true": 0, "other": 1}

    def test_the_ruler_is_not_blind_to_the_docstring_it_measures(self):
        """注释与字符串里的那行代码不算调用点（A165 那一族的文本扫描才会犯）"""
        src = '"""\n实测 resolve_data_path("x", for_write=True) 403\n"""\n'
        assert count_calls(src) == {"true": 0, "other": 0}

    def test_a_directory_with_no_call_site_raises_instead_of_reporting_zero(self, tmp_path):
        (tmp_path / "quiet.py").write_text("x = 1\n", encoding="utf-8")
        with pytest.raises(NoMatch):
            count_calls_in_dir(tmp_path)


class TestCoverageRuler:
    """④号洞：读不到就走 fallback，把真实的 +7 语句讲成「一格未动」"""

    def test_the_tuple_shape_on_a_known_xml(self):
        stats = _parse_coverage(COVERAGE_SAMPLE)
        assert stats["api/deps.py"] == (3, 1, 2, 1)
        assert stats["routes/quality.py"] == (1, 0, 0, 0)

    def test_no_match_is_not_the_same_as_zero(self):
        stats = _parse_coverage(COVERAGE_SAMPLE)
        with pytest.raises(NoMatch):
            by_basename(stats, "nope.py")

    def test_a_colliding_basename_raises_instead_of_picking_the_last(self):
        stats = _parse_coverage(COVERAGE_SAMPLE)
        with pytest.raises(AmbiguousName, match="quality.py"):
            by_basename(stats, "quality.py")

    def test_a_unique_basename_is_found_by_basename(self):
        stats = _parse_coverage(COVERAGE_SAMPLE)
        assert by_basename(stats, "deps.py") == ("api/deps.py", (3, 1, 2, 1))

    def test_the_sample_xml_is_not_the_repo_xml(self):
        """自检样本必须**含撞名**：本仓真实 XML 的裸名 13 处相撞，样本只留两支"""
        stats = _parse_coverage(COVERAGE_SAMPLE)
        assert len([fn for fn in stats if Path(fn).name == "quality.py"]) == 2


class TestTheRulersAreUsedNotDecorated:
    """自检不许是可选步骤：可选的自检等于没有自检（A171 的收口形状）"""

    @staticmethod
    def proves_inline(fn) -> bool:
        """函数体（跳过 docstring）的头两句里是否引用了已知答案样本或另一支读数口"""
        tree = ast.parse(inspect.getsource(fn).lstrip())
        body = tree.body[0].body
        if (body and isinstance(body[0], ast.Expr)
                and isinstance(getattr(body[0], "value", None), ast.Constant)
                and isinstance(body[0].value.value, str)):
            body = body[1:]
        snippet = "\n".join(ast.unparse(s) for s in body[:2])
        return any(mark in snippet for mark in ("_SAMPLE", "_ANSWER") + PUBLIC_RULERS)

    def test_no_public_ruler_skips_the_self_proof(self):
        for fname in PUBLIC_RULERS:
            assert self.proves_inline(globals()[fname]), f"{fname} 的读数口没带自检"

    def test_the_guard_fires_on_a_ruler_without_a_proof(self):
        def unproven(text):
            return len(text)

        assert not self.proves_inline(unproven)

    def test_every_named_ruler_exists(self):
        for fname in PUBLIC_RULERS:
            assert callable(globals().get(fname)), fname
