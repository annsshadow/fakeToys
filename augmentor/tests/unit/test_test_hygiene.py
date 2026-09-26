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
"""

import ast

import pytest

from scripts.triage_tests import _collect_consts, _is_vacuous, analyse_all


@pytest.fixture(scope="module")
def report():
    """全量测试文件分析结果（模块级缓存，避免每个用例重扫一遍）"""
    return analyse_all()


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
