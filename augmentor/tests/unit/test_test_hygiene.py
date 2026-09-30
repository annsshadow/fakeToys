# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""测试套件自身的卫生门禁

F6 的核心诊断是"覆盖率刷分"：套件里混入大量**恒真断言**（`assert True`、
`assert X or True`、局部字面量互比）与**空壳测试**（函数体只有一行 docstring，
或用 `try/except: pass` 吞掉一切）。它们不可能因业务逻辑变化而失败，
却把覆盖率数字推高，让"通过测试"不再等价于"行为正确"。

2026-09-21 的清理结果：删除 29 个占位/冗余文件、改写 7 处空洞断言、修复 1 处空壳测试。
测试数 3108 → 3036，而覆盖率**反而从 99.41% 升到 99.50%**——
删掉假测试比留着它们更能反映真实覆盖。

本门禁确保它们不会重新长回来。判据由 `scripts/triage_tests.py` 提供，
与清理时使用的是同一套逻辑，避免"门禁标准与清理标准不一致"。

L84 又补了一族同一病根但判据完全不同的问题：**同作用域重复定义**
（见 `TestNoShadowedDefinitions`）。它不是"断言会不会失败"，而是"这段代码会不会被执行"。
"""

import ast
from pathlib import Path

import pytest

from scripts.triage_tests import (
    _collect_consts,
    _is_vacuous,
    analyse_all,
    duplicate_definitions,
)

REPO = Path(__file__).resolve().parents[2]

# 扫描噪音：`Temp/` 下是一次性探针，副本式目录（`.backups`）天生装重复内容
NOISE_DIRS = {"Temp", "__pycache__", ".venv", "node_modules", "build", "dist",
              ".backups", ".pytest_cache", ".ruff_cache", ".mypy_cache"}


@pytest.fixture(scope="module")
def report():
    """全量测试文件分析结果（模块级缓存，避免每个用例重扫一遍）"""
    return analyse_all()


@pytest.fixture(scope="module")
def shadow_scan():
    """全仓（产品码 + API + CLI + 测试）同作用域重复定义扫描，模块级缓存

    返回值同时带 `files`（扫到了哪些文件），因为"零违例"这个判决的可信度取决于
    扫描面真的覆盖到了 —— 只报 offenders 的话，一次改坏 glob 就能让门禁永远绿。
    """
    files = []
    offenders = []
    for path in sorted(REPO.rglob("*.py")):
        if NOISE_DIRS & set(path.parts):
            continue
        rel = path.relative_to(REPO).as_posix()
        files.append(rel)
        try:
            dupes = duplicate_definitions(path)
        except (SyntaxError, UnicodeDecodeError):
            continue
        for d in dupes:
            offenders.append(
                f"{rel}::{d['scope']}::{d['name']}:{d['lineno']}"
                f"（顶掉了 :{d['shadowed_lineno']}）"
            )
    return {"files": files, "offenders": offenders}


class TestSuiteHygiene:
    """测试卫生门禁"""

    def test_no_placeholder_files(self, report):
        """不得存在不含任何有效检查点的测试文件"""
        offenders = [r["file"] for r in report if r.get("placeholder")]
        assert not offenders, (
            "以下测试文件不含任何有效检查点（只有恒真断言），"
            f"应删除或补成真断言: {offenders}"
        )

    def test_no_vacuous_asserts(self, report):
        """不得存在恒真断言

        典型形式：`assert True`、`assert 94.17 >= 80.0`（字面量互比）、
        `assert hasattr(x, 'y') or True`、`n = 5; assert n == 5`（局部常量恒真）。
        """
        offenders = [
            (r["file"], r["vacuous_asserts"])
            for r in report
            if r.get("vacuous_asserts")
        ]
        assert not offenders, (
            f"发现恒真断言（不可能因业务逻辑变化而失败）: {offenders}"
        )

    def test_no_test_that_cannot_fail(self, report):
        """不得存在不可能失败的测试

        判据是"能否失败"而不是"有没有 assert"——`writer.close()` 这种
        "不抛异常"测试是合法的（`close` 若开始抛异常，测试就会失败），
        不应被误报。
        """
        offenders = [
            f"{r['file']}::{d['name']}:{d['lineno']}"
            for r in report
            for d in r.get("tests_detail", [])
            if not d["can_fail"]
        ]
        assert not offenders, (
            "以下测试无论被测代码如何改动都会通过，等于没有测试: "
            f"{offenders}"
        )

    def test_suite_size_is_reported(self, report):
        """把套件规模与检查点密度纳入门禁，防止用空测试灌水

        这条不设硬阈值（避免为了凑数字而写测试），只在密度异常时提示：
        平均每个测试至少要有 1 个有效检查点。
        """
        total_tests = sum(r["tests"] for r in report)
        total_checks = sum(r["real_checks"] for r in report)

        assert total_tests > 0
        assert total_checks >= total_tests, (
            f"有效检查点密度过低：{total_checks} 个检查点 / {total_tests} 个测试"
        )


class TestNoShadowedDefinitions:
    """同作用域重复定义门禁：抓「后一份静默覆盖前一份」

    **为什么单独立一档**：这一族和空洞断言同病根（测试看着在跑其实没跑），但判据完全
    不同 —— 上面几档问的是"断言会不会失败"，这一档问的是"这段代码会不会被执行"。
    立项现场（2026-09-26，L84）：改 `tests/unit/test_excel_write_edge_l84.py` 的辅助函数
    时 Edit 留下了一份陈旧副本，两份 `def product_call_sites` 并排存在，跑起来只有后写的
    那份生效；测试全绿，本门禁一条也没红，因为生效那份确实是真测试。是同批的 AST 复查
    撞破的，不是任何既有守卫。

    首跑在 4 个旧测试文件里抓到 5 处覆盖事件（`TestCleanerExtended`、
    `TestMigrationExtended`、`TestValidationExtended` 三连、`TestVersioningExtended`），
    即**约 40 个测试用例长期根本没被执行**。修法是把**后面**那几份改名（它们本来就在跑，
    改名不改变行为），于是前面那几份复活：142 passed → 182 passed，全绿。
    """

    def test_repo_has_no_shadowed_definitions(self, shadow_scan):
        """任何作用域里都不许有两份同名 def/class 互相顶掉"""
        assert shadow_scan["offenders"] == [], shadow_scan["offenders"]

    def test_gate_reaches_product_code_not_only_tests(self, shadow_scan):
        """扫描面必须真覆盖产品码

        这一档的"零违例"判决只有在看得到产品码时才有意义（覆盖产品码才是它的重点：
        那里被顶掉的是**业务行为**，不是几条测试）。glob 改坏时上一条会永远绿。
        """
        files = shadow_scan["files"]
        assert len(files) > 300, len(files)
        for expected in ("augmentor/converter.py", "augmentor/cli/parser.py",
                         "api/routes/dataset_tools.py", "scripts/triage_tests.py"):
            assert expected in files, expected


class TestAnalyzerTaintRules:
    """分析器自身的判据：既不许把有效断言误报成空洞，也不许放过真空洞

    **为什么本轮加这一格**：L79 的文档引用守卫写了 `bad = {}` → 循环里 `bad[doc] = dead`
    → `assert bad == {}`，被本门禁报成恒真断言。撒谎的不是那条断言而是判据 —— 它认得
    `d.append(...)` 这类方法接收者，却认不得 `d[k] = v`（没人会写成 `d.__setitem__(k, v)`）。
    误报的代价不是「多一条红」而是**下一轮会把有效断言改成没判断力的形状去绕开门禁**。

    这里直接测那两个内部函数而不是 `analyse_file`：后者把路径按仓库根做 `relative_to`，
    `tmp_path` 在它外面（实测抛 `ValueError: ... is not in the subpath of ...`）。
    端到端那一半由 `test_no_vacuous_asserts` 对全仓扫的结果自己兑现。
    """

    @staticmethod
    def _vacuous(body):
        fn = ast.parse("def test_a():\n" + body + "\n").body[0]
        consts = _collect_consts(fn)
        return [ast.unparse(n) for n in ast.walk(fn)
                if isinstance(n, ast.Assert) and _is_vacuous(n, consts)]

    def test_item_assignment_taints_the_container(self):
        """下标写入过的字典不是常量：这条断言的真值要靠被测逻辑决定"""
        assert self._vacuous(
            "    bad = {}\n"
            "    for doc in ('a', 'b'):\n"
            "        bad[doc] = len(doc)\n"
            "    assert bad == {}\n") == []

    def test_item_deletion_and_augmented_item_assignment_also_taint(self):
        assert self._vacuous(
            "    acc = {'n': 0}\n"
            "    acc['n'] += 1\n"
            "    del acc['n']\n"
            "    assert acc == {}\n") == []

    def test_a_genuinely_vacuous_assert_still_offends(self):
        """反向档：上面两条不许是把判据整体关掉换来的"""
        assert self._vacuous("    n = 5\n    assert n == 5\n") == ["assert n == 5"]

    def test_the_repo_wide_gate_sees_no_offender_in_the_l79_guard(self, report):
        row = [r for r in report
               if r["file"].replace("\\", "/").endswith("test_doc_line_refs_l79.py")]
        assert len(row) == 1, row
        assert row[0]["vacuous_asserts"] == 0, row[0]


class TestShadowRuleJudgements:
    """`duplicate_definitions` 自身的双向判据

    纪律 (f)：新判据必须带双向注入用例 —— 只测"能抓到"的判据往往是靠把合法形状一起抓掉
    达成的，那种门禁下一轮就得改松或绕开。所以真覆盖那一半必须红，四种合法同名（条件定义 /
    property 对儿 / @overload / 跨作用域）必须不红。
    """

    @staticmethod
    def _dupes(tmp_path, source):
        path = tmp_path / "sample.py"
        path.write_text(source, encoding="utf-8")
        return duplicate_definitions(path)

    def test_repeated_function_offends_and_points_at_both(self, tmp_path):
        dupes = self._dupes(tmp_path, "def f():\n    pass\n\n\ndef f():\n    pass\n")
        assert [(d["name"], d["shadowed_lineno"], d["lineno"], d["scope"])
                for d in dupes] == [("f", 1, 5, "<module>")]

    def test_three_declarations_report_a_chain_not_one(self, tmp_path):
        """三连同名要给出两次覆盖：只报一次会把中间那份也当成活着的"""
        dupes = self._dupes(
            tmp_path,
            "class C:\n    pass\n\n\nclass C:\n    pass\n\n\nclass C:\n    pass\n")
        assert [(d["shadowed_lineno"], d["lineno"]) for d in dupes] == [(1, 5), (5, 9)]

    def test_repeated_method_inside_a_class_offends(self, tmp_path):
        dupes = self._dupes(
            tmp_path,
            "class C:\n"
            "    def m(self):\n        return 1\n"
            "    def m(self):\n        return 2\n")
        assert [(d["scope"], d["name"]) for d in dupes] == [("C", "m")]

    def test_conditional_fallback_does_not_offend(self, tmp_path):
        """`try: import x` → `except ImportError:` 里的替身是合法回落，不报"""
        dupes = self._dupes(
            tmp_path,
            "try:\n    import ujson as json\nexcept ImportError:\n"
            "    import json\n\n\ntry:\n    pass\nexcept ImportError:\n"
            "    def load(s):\n        return s\n")
        assert dupes == []

    def test_property_and_overload_pairs_do_not_offend(self, tmp_path):
        dupes = self._dupes(
            tmp_path,
            "from typing import overload\n\n\n"
            "class C:\n"
            "    @property\n"
            "    def x(self):\n        return 1\n"
            "    @x.setter\n"
            "    def x(self, v):\n        pass\n"
            "    @x.deleter\n"
            "    def x(self):\n        pass\n\n\n"
            "@overload\n"
            "def g(a: int) -> int: ...\n"
            "@overload\n"
            "def g(a: str) -> str: ...\n"
            "def g(a):\n    return a\n")
        assert dupes == []

    def test_cross_scope_same_name_does_not_offend(self, tmp_path):
        """类方法 `convert_file` + 模块函数 `convert_file` 是两个作用域，`converter.py` 就这么写"""
        dupes = self._dupes(
            tmp_path,
            "class C:\n"
            "    def convert_file(self):\n        pass\n\n\n"
            "def convert_file():\n    pass\n")
        assert dupes == []

