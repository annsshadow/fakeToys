# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""L329：A370 —— 静态闸判据与 `re` 权威之间的三处口径差，逐格收口

账本 A370 原文记的是三处差：

① `a{+1}` 那一族 —— 重复计数读取器按字面量读，而 `re` 把它当量词；
② 两格**已知误伤**（合法模式被闸拒）；
③ 括号不配对 / 字符类未闭合 / 组名没写完 / 结尾裸反斜杠这四格 —— 闸不下判断，
   而它们同时是 `re.compile` 的语法错 ⇒ 由编译那一步折成 400，用户看到的回执形状
   与闸拦下时不同。

本轮**逐格量过**（读数见 `Temp/l329q/a370_probe.py` 与 `Temp/l329q/a370_after.py`）
并按量到的结果拍板：

**① 不是差 —— 闸与 `re` 同口径。**
`re` 把 `a{+1}` 读成「字面 `a` + 一个或多个 `{` + 字面 `1` + 字面 `}`」
（`re._parser.parse` 的读数逐字是 `[(LITERAL, 97), (MAX_REPEAT, (1, MAXREPEAT,
[(LITERAL, 123)])), (LITERAL, 49), (LITERAL, 125)]`），而本闸读同一串时**先把 `{`
当字面量、再把后面的 `+` 照常当量词** ⇒ 两边模型一致，**判决也一致**
（实测 `(a{+1})+b` 与 `(\\{{+1})+b` 两形都被闸判中）。
⇒ 处置：**写实**（`_read_repeat_bound` 的 docstring 补上这一段）+ **本文件把这一族
钉成一条守卫**（闸的判决必须与 `re` 的解析形状同向）。

**② 误伤是闸的缺陷，不是对外契约。**
那两格（`(\\[[^\\]]*\\])+` 与 `(?:[^,]+,){3}[^,]+`）**实测不是指数级**：在
n=14/16/18/20/22 的必败语料上都是 ~0.0003~0.002 ms 的**平坦**曲线，而基准
`(a+)+b` 是 0.0007 → 0.0032 → 0.0132 → 0.0531 → 0.2083 秒（每 +2 字 ×4）。
根因：闸的判据是「组体里含可变长片段 ⇒ 该组被重复 ⇒ 危险」，而这两族的**片段长度
被组体末尾那个必现字符钉死了**（`[^\\]]*` 吃不到 `]`、`[^,]+` 吃不到 `,`）
⇒ 组体对同一条输入只有一种切法 ⇒ 重复它不炸。
⇒ 处置：闸加了一档「**锚点不相交 ⇒ 豁免**」（`validation._pattern_group_is_exempt`）。
判据只会让闸**比 `re` 宽**、不会比 `re` 窄：算不出字符集 / 有分支 / 锚点不是组体
最后一个单元 / 锚点可选 / 片段来自子组 ⇒ 一律**不豁免**。

**③ 两种 400，各自说清自己那一种 —— 不是缺陷。**
闸拦下的是「形状危险」，编译拦下的是「语法错」：这是**两个不同的真原因**，
用户看到哪一档取决于他的模式串到底有什么毛病 ⇒ 两种回执都是诚实的信息。
⇒ 处置：**写实**（本文件钉住「语法错那一档回的是编译的判决，不是闸的判决」）。

跑法：`python -m pytest tests/unit/test_pattern_disjoint_exempt_l329.py -q --no-cov`
"""

import json
import re

import pytest
from fastapi.testclient import TestClient

from augmentor import validation
from augmentor.exceptions import DataValidationError
from augmentor.search_enhanced import EnhancedSearcher
from augmentor.validation import (
    find_repeated_ambiguous_group,
    require_search_pattern,
)

ITEMS = [{"instruction": "如何申请公租房？", "input": "", "output": "请登录官网申请"}]

#: ① `{+n}` 那一族：`re` 与本闸必须**同判**（读数见模块 docstring）
BRACE_FAMILY_DANGER = (
    "(a{+1})+b",
    r"(\{{+1})+b",
    "(a{+1})+",
)

#: ② 豁免那一族（实测不是指数级）
EXEMPTED = (
    r"(\[[^\]]*\])+",
    r"(?:[^,]+,){3}[^,]+",
    "(a*b)+",
    r"(\d*b)+",
)

#: ②的孪生四格：长得极像，但片段的长度**没**被钉死 ⇒ 必须仍然被拦
TWINS = (
    "(a*b*)+",
    "(a*|b)+",
    "([a-z]*a)+",
    "(a*.)+",
)

#: ③ 闸读不出 ⇒ 不下判断，而它们同时是 `re.compile` 的语法错
SYNTAX_ERROR_SHAPES = (
    ("(a+", "missing )"),
    ("[a", "unterminated character set"),
    ("(?P<", "missing group name"),
    ("a" + chr(92), "bad escape"),
)


class TestTheBraceFamilyAgreesWithRe:
    """① `{+n}` 那一族：闸与 `re` 同口径（这不是一处「口径差」）"""

    def test_re_reads_the_brace_form_as_a_repeat_of_the_literal_brace(self):
        """`re` 的读数逐字：字面 `a` + 一个或多个 `{` + 字面 `1` + 字面 `}`"""
        parsed = re._parser.parse("a{+1}")
        assert (str(parsed[0][0]), parsed[0][1]) == ("LITERAL", ord("a"))
        repeat = parsed[1]
        assert str(repeat[0]) == "MAX_REPEAT", repeat
        assert repeat[1][0] == 1 and repeat[1][1] == re._constants.MAXREPEAT, repeat
        assert [(str(op), arg) for op, arg in repeat[1][2]] == \
            [("LITERAL", ord("{"))], repeat
        assert (str(parsed[2][0]), parsed[2][1]) == ("LITERAL", ord("1")), parsed
        assert (str(parsed[3][0]), parsed[3][1]) == ("LITERAL", ord("}")), parsed

    def test_the_gate_catches_the_dangerous_brace_forms(self):
        for pattern in BRACE_FAMILY_DANGER:
            assert find_repeated_ambiguous_group(pattern) is not None, pattern

    def test_the_gate_does_not_flag_the_bare_brace_form(self):
        """裸 `a{+1}` 自身不含可变长组 ⇒ 不出声（与 `re` 的模型一致）"""
        assert find_repeated_ambiguous_group("a{+1}") is None
        assert find_repeated_ambiguous_group("a{+2}b") is None


class TestTheExemptionIsSound:
    """② 豁免那一档：放行的是**实测不炸**的那几格，放行不了的那几格仍被拦"""

    def test_the_exempted_family_passes_the_authority(self):
        for pattern in EXEMPTED:
            assert require_search_pattern("query", pattern) == pattern

    def test_the_twins_are_still_caught(self):
        for pattern in TWINS:
            assert find_repeated_ambiguous_group(pattern) is not None, pattern
            with pytest.raises(DataValidationError):
                require_search_pattern("query", pattern)

    def test_the_exemption_is_off_when_the_charsets_cannot_be_computed(self, monkeypatch):
        """注入臂 1：把字符集计算打成「算不出」⇒ 豁免那一档整档失效

        ⇒ 证明「豁免」真的依赖字符集计算，而不是别的什么恒真条件。
        """
        monkeypatch.setattr(validation, "_unit_char_set", lambda unit: None)
        for pattern in EXEMPTED[:2]:
            assert find_repeated_ambiguous_group(pattern) is not None, pattern

    def test_the_exemption_is_load_bearing_on_the_disjointness_test(self, monkeypatch):
        """注入臂 2：把不相交判据打成恒真 ⇒ 那四格孪生**漏过去**

        ⇒ 证明「不相交」这一格是承重的（把闸放得太宽的后果是可测的）。
        """
        monkeypatch.setattr(validation, "_char_sets_disjoint",
                            lambda left, right: True)
        missed = [p for p in TWINS if find_repeated_ambiguous_group(p) is None]
        assert missed, "注入臂没生效 ⇒ 这一支的断言不是恒真的"

    def test_the_exempted_shapes_are_really_flat_in_time(self):
        """那两格在实测上是**平坦**的（不是指数曲线）—— 豁免的依据是读数，不是想象"""
        import time

        for pattern, unit in ((r"(\[[^\]]*\])+", "["),
                              (r"(?:[^,]+,){3}[^,]+", "a,")):
            compiled = re.compile(pattern)
            durations = []
            for n in (14, 16, 18, 20, 22):
                start = time.perf_counter()
                compiled.search(unit * n)
                durations.append(time.perf_counter() - start)
            # 平坦：最大格不到最小格的 50 倍（指数曲线每 +2 字 ×4 ⇒ 22 字那格是 14 字那格的 256 倍）
            assert max(durations) < max(min(durations) * 50, 1e-4), durations


class TestTheTwoFourHundreds:
    """③ 两种 400，各自说清自己那一种"""

    def test_the_gate_says_nothing_about_the_syntax_error_shapes(self):
        for source, _ in SYNTAX_ERROR_SHAPES:
            assert find_repeated_ambiguous_group(source) is None, source

    def test_re_is_the_one_that_speaks_on_those_shapes(self):
        for source, fragment in SYNTAX_ERROR_SHAPES:
            with pytest.raises(re.error) as caught:
                re.compile(source)
            assert fragment in str(caught.value), (source, str(caught.value))

    def test_the_library_face_says_the_compile_verdict_not_the_gate_one(self):
        """库面：语法错 ⇒ 报的是**编译**的判决（「无效的正则表达式 …」）"""
        for source, fragment in SYNTAX_ERROR_SHAPES:
            with pytest.raises(DataValidationError) as caught:
                EnhancedSearcher(ITEMS).search(source, method="regex")
            message = str(caught.value)
            assert "无效的正则表达式" in message, message
            assert "指数级回溯" not in message, message

    def test_the_http_face_answers_400_for_both_kinds(self, tmp_path, monkeypatch):
        """两种 400：形状危险的回「指数级回溯」，语法错的回「无效的正则表达式」"""
        import os
        import tempfile
        from types import SimpleNamespace

        from api.main import app

        monkeypatch.setenv("AUGMENTOR_DATA_ROOTS",
                           os.pathsep.join([str(tmp_path), tempfile.gettempdir()]))
        monkeypatch.delenv("AUGMENTOR_API_KEY", raising=False)
        data = tmp_path / "items.json"
        data.write_text(json.dumps(ITEMS, ensure_ascii=False), encoding="utf-8")
        client = TestClient(app)

        risky = client.post("/api/dataset/search", json={
            "input_file": str(data), "query": "(a+)+b", "method": "regex"})
        assert risky.status_code == 400, risky.text
        assert "指数级回溯" in risky.json()["detail"], risky.text

        broken = client.post("/api/dataset/search", json={
            "input_file": str(data), "query": "(a+", "method": "regex"})
        assert broken.status_code == 400, broken.text
        assert "无效的正则表达式" in broken.json()["detail"], broken.text
        assert "指数级回溯" not in broken.json()["detail"], broken.text
