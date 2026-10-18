# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""L278：`method="regex"` 的模式串静态闸（A345 收口）

缺陷形状：`SearchRequest.query` 是无约束 `str`，`EnhancedSearcher._search_regex`
拿到就 `re.compile`，然后对**每条候选字段值**跑一次 `pattern.search`；haystack 来自
调用方自己上传的数据文件，模式串也来自同一个请求体 ⇒ 两个方向都可控，而
`run_in_thread` 既无超时也无取消。L267 那轮只把「编译不过」接成了 400，
「编译得过但形状坏」与「长得没边」两维原样放过：实测 `(a+)+b` 在 `"a"` 重复串上
n=14 / 16 / 18 / 20 / 22 的耗时 0.0008 / 0.00316 / 0.01324 / 0.05305 / 0.20834 秒
（每 +2 字 ×4），200,000 字的模式串模型层照收。

本模块的四把尺子：

1. **真值表**（现量，逐格点名）：`find_repeated_ambiguous_group` 对 26 条危险形状
   **全部**判中、对 20 条常用合法形状**全部**放行；另有 2 格**已知误伤**
   （`[a](\\[[^\\]]*\\])+` 与 `[b](?:[^,]+,){3}[^,]+`，见 `KNOWN_FALSE_POSITIVES`）
   和 6 格**本闸不判**的族（`[c]` 交替公共前缀 / 可选量词 / `(?#…)`），
   两张贴在表里钉住 ⇒ 判据一旦变宽或变窄，这里先红，不许静默漂移。
2. **单一权威**：非空 / 长度 / 形状三道闸只有一个产地 `require_search_pattern`，
   产品码里除它以外没有第二处给模式串定长度；`_search_regex` 那一格必须**现读**到
   它对 `query` 的调用（源码锚点，不是行号）。
3. **行为差**：危险形状与超限长度 ⇒ `DataValidationError`（⇒ HTTP 400、CLI 出声），
   且**判在语料之前**（空语料也拒，与 L267 同一口径）；合法模式、其他方法、
   以及「超长但非 regex 方法」的 `query` 一概不受影响 ⇒ 刀口没有扩宽。
4. **尺子自证**（三支注入臂，零写盘）：摘掉 `_search_regex` 的接线、把静态闸打成
   恒真、把长度上限打到无限，各自都要让对应那格读数变红 ⇒ 本守卫不是恒真。

**解析器自身的支路**（`TestParserBranches`）：上面那把静态闸要读模式串，所以它自己
也有一族「认不出就不下判断」的支路。补覆盖时在这族支路里挖出四处真缺陷，全部按
`re` 的口径改正并把读数钉死：

- **域外异常逃出产品入口**：`_read_repeat_bound` 用 `int(body)` 收尾，而 `body` 只验过
  「字符是数字或 `, `」⇒ `a{1,2,3}` 触发 `int("2,3")`、`a{ }` 触发 `int(" ")`、
  `a{²}` 触发 `int("²")`（`str.isdigit()` 认它，`int()` 不认）当场炸 `ValueError`。
  它既不是 `DataValidationError` 也不是 `re.error`，`_search_regex` 的
  `except re.error` 接不住 ⇒ 改前 `EnhancedSearcher.search("a{1,2,3}", method="regex")`
  直接抛库外异常，HTTP 面 500。改前三者读数逐字记为
  `ValueError: invalid literal for int() with base 10: '2,3'` / `: ' '`。
- **漏判（实测指数）**：开区间量词的「能重复到 ≥2 次」额外要求了下界非 0
  （`hi is None and lo != 0`），于是等价于 `*` 的 `{0,}` / `{,}` 被打成不可重复 ⇒
  `((a+)){0,}b`、`(a+){0,}b` 漏判。实测 `((a+)){0,}b` 在必败语料上的耗时
  0.00074 / 0.00242 / 0.00864 / 0.03659 / 0.14244 秒（n=14/16/18/20/22，每 +2 字 ×4），
  与基准 `(a+)+b` 同曲线。
- **漏判（字符类里的 `[`）**：`_skip_char_class` 把 `[[:alpha:]]` 当 POSIX 嵌套类去找
  `]]`，而 `re` 没有 POSIX 类——实测 `findall("[[:alpha:]]", "a]:b[c")` 返回 `['a]']`，
  即类在**第一枚** `]` 处闭合、后面那枚是字面量。于是「类内含 `[`」的模式被扫到串尾
  并回 `None` ⇒ `([ab[]+)+b`（实测 0.00012 / 0.00013 / 0.00046 / 0.00184 秒，
  与基准同曲线）整族漏判。
- **误伤（与 `?` 同形）**：`{0,1}` / `{,1}` 就是 `?` 的另一写法，实测线性
  （`(a{0,1})+b` 与 `(a?)+b` 在 22 字上都是 0.00000 秒），改前却按「可变长」判死；
  含空格的 `{2, }` / `{1, 2}` `re` 当字面量，改前却当量词（`((a+)){2, }` 被误拒）。
- **一枚扩展记号废掉整把闸**：`_group_extension_end` 认不出某个 `(?` 扩展时回 `None`，
  而扫描主循环据此**放弃整条模式**——于是攻击者只要在危险形状前面挂一枚合法但不认识
  的记号就整条放行：实测 `(?i)(a+)+b`、`(?s)(a*)*c`、`()(?(1)a)(a+)+b` 三形都是**合法
  正则**且改前读数都是 `None`。现在纯开关指令 `(?flags)` 由 `_flag_directive_end` 认走
  （它不包住内容，不该推组帧），其余认不出的扩展按「一个普通组的开头」继续扫——吃掉的
  `(` 总有一个 `)` 来配，所以栈不会失衡，「认不出」只可能变成「这一形判得不准」，
  不可能再变成「整条不判」。顺带证伪了一条旧口径：条件组 `()(?(1)a+|b)+c` 在必败语料上
  与基准 `(a+)+c` 同曲线（0.00022 / 0.00051 / 0.00207 / 0.00795 对 0.00021 / 0.00050 /
  0.00188 / 0.00759），它本来就该在危险面而不是「不判」面。

**这是缓解不是根除**：静态闸只看模式串的形状，不看语料，所以它判的是「这条模式在
部分匹配的输入上会不会指数级回溯」，而不是「这次匹配实际跑了多久」。请求级超时与
换线性引擎（`re2`）那一半仍待拍（引入非确定性结局 / 改 `re.IGNORECASE` 语义），
见账本 A345 与 L278。
"""

import json
import re
import tempfile
from pathlib import Path

import pytest
from fastapi.testclient import TestClient

from augmentor.exceptions import DataValidationError
from augmentor.search_enhanced import EnhancedSearcher
from augmentor import validation
from augmentor.validation import (
    MAX_PATTERN_LENGTH,
    find_repeated_ambiguous_group,
    require_search_pattern,
)

AI_DIR = Path(__file__).resolve().parents[2]

ITEMS = [
    {"instruction": "如何申请公租房？", "input": "", "output": "请登录官网申请"},
    {"instruction": "公租房需要什么材料？", "input": "", "output": "身份证"},
]

#: 危险形状（现量 21 格，逐格都要判中）：可变长的组被重复到 ≥2 次。
DANGER_PATTERNS = (
    "(a+)+b",
    "(a*)*c",
    "(.+)+$",
    "(a+)*",
    "(a*){2,}",
    "([0-9]+)+x",
    "(a|a+)+b",
    r"(?:\s+\w+)+$",
    "((a+))+",
    "(.*)*",
    r"(\w+\s?)*",
    "(a{1,2})+",
    r"((\d+)+)",
    "(x+y+)+",
    "(.+)+",
    r"(\d{1,}){2}",
    r"([\w.]+)+@",
    "((a)(b+))+",
    r"(?:(?:\w)+)+",
    "(a*a)+",
    "((((((((a+))))))))+",
    # 以下三形改前漏判：开区间量词被额外要求「下界非 0」，以及类内含 `[` 时扫到串尾
    r"((a+)){0,}b",
    r"(a+){0,}b",
    r"([ab[]+)+b",
    # 带冒号的开关组 `(?i:…)`：新加的「纯开关指令」那一刀不能把这一刀也带走
    r"(?i:((a+)))+b",
    # 改前「挂一枚认不出的扩展记号就废掉整把闸」：这一形合法、实测指数、改前读 None
    "()(?(1)a)(a+)+b",
)

#: 合法形状（现量 20 格，逐格都要放行）：数据集检索里真正在写的模式。
ALLOWED_PATTERNS = (
    "(foo|bar)+",
    "(a+b+)",
    "(abc){3}",
    r"\d{1,3}",
    r"(?:\d{2}){4}",
    "(a|b)*",
    r"(https?|ftp)://",
    r"[0-9]{2,4}",
    r"(\w+)",
    r"(?P<year>\d{4})-(?P<month>\d{2})",
    r"[^a-z]+",
    r"(?i)(foo|bar)+",
    r"(\d{3})\.(\d{3})",
    "公.*租房",
    r"^\s*(\d+)\s*[年月日]$",
    "(colou?r)+",
    # 以下四形改前误伤或炸域外异常：`{0,1}` 与 `?` 同形、含空格/多逗号的计数 `re` 当字面量
    r"(a{0,1})+b",
    r"(a{1, 2})+",
    "a{1,2,3}",
    "a{ }",
)

#: 已知误伤（现量 2 格）：形状上「组内含可变长片段」但每段被必现字面量钉住，
#: 实际不指数级。要消掉它得做字符类不相交判定（本闸不做，见 A3xx 新的一格），
#: 所以先把误伤**钉在这一格里点名**：哪天判定变准了，这一格必须跟着改，
#: 而不是让「合法模式被拒」这件事从散文里飘走。
KNOWN_FALSE_POSITIVES = (
    r"(\[[^\]]*\])+",
    r"(?:[^,]+,){3}[^,]+",
)

#: 本闸**不下判断**的族（现量 6 格，逐格读数为 `None`）：交替分支公共前缀
#: （`(a|aa)+b`）、可选量词套可空匹配（`(a?)+`，实测 CPython 的空匹配推进护栏下
#: 线性：`(a?)+b` 在 24 字上 0.0001 秒）、`(?#…)` 内联注释。
#: 条件组 `(?(id)yes|no)` 原先也在这一族，读数证明它**不该**在：`()(?(1)a+|b)+c`
#: 在必败语料上 0.00022 / 0.00051 / 0.00207 / 0.00795 秒，与基准 `(a+)+c` 的
#: 0.00021 / 0.00050 / 0.00188 / 0.00759 秒同曲线 ⇒ 真指数，已挪进危险面。
UNJUDGED_FAMILIES = (
    "(a|aa)+b",
    "(a|ab)+",
    "(a?)+",
    "((a?)?)+b",
    "([a-z]|[a-z][a-z])+",
    "(?#((a+))+)",
)


@pytest.fixture
def search_env(tmp_path, monkeypatch):
    """HTTP 面：`POST /api/dataset/search` 的最小可用环境（与 L267 同形）。"""
    import os
    from types import SimpleNamespace
    from api.main import app

    monkeypatch.setenv("AUGMENTOR_DATA_ROOTS",
                       os.pathsep.join([str(tmp_path), tempfile.gettempdir()]))
    monkeypatch.delenv("AUGMENTOR_API_KEY", raising=False)
    data = tmp_path / "items.json"
    data.write_text(json.dumps(ITEMS, ensure_ascii=False), encoding="utf-8")
    return SimpleNamespace(client=TestClient(app), data=data)


def post(search_env, **body):
    """发一次搜索请求，返回响应对象。"""
    payload = {"input_file": str(search_env.data), "query": "公租房", "method": "regex"}
    payload.update(body)
    return search_env.client.post("/api/dataset/search", json=payload)


class TestTruthTable:
    """静态闸的真值表：判中多少、放行多少、误伤与不判各几格"""

    def test_every_danger_shape_is_caught(self):
        missed = [p for p in DANGER_PATTERNS if find_repeated_ambiguous_group(p) is None]
        assert missed == []

    def test_the_danger_face_is_the_measured_size(self):
        assert len(DANGER_PATTERNS) == 26

    def test_every_allowed_shape_passes_except_the_named_two(self):
        hits = [p for p in ALLOWED_PATTERNS if find_repeated_ambiguous_group(p) is not None]
        assert hits == []

    def test_the_allowed_face_is_the_measured_size(self):
        assert len(ALLOWED_PATTERNS) == 20

    def test_the_known_false_positive_grid_is_exactly_two(self):
        """误伤只有这两格，且**这两格确实被拒**——判据变准了就要在这里红一次。"""
        assert len(KNOWN_FALSE_POSITIVES) == 2
        assert [find_repeated_ambiguous_group(p) is not None
                for p in KNOWN_FALSE_POSITIVES] == [True, True]
        for pattern in KNOWN_FALSE_POSITIVES:
            with pytest.raises(DataValidationError):
                require_search_pattern("query", pattern)

    def test_the_unjudged_families_stay_unjudged(self):
        """不判的族逐格读 `None`：这一格防的是「把尺子偷偷收进不认识的形状里」。"""
        assert len(UNJUDGED_FAMILIES) == 6
        assert [find_repeated_ambiguous_group(p) for p in UNJUDGED_FAMILIES] == [None] * 6

    def test_hit_names_column_inner_fragment_and_quantifier(self):
        """命中要能指出是哪一段：报错的可用性靠这三样，缺一样调用方就得自己试。"""
        assert find_repeated_ambiguous_group("(a+)+b")[:1] == (0,)
        col, inner, repeated, quant = find_repeated_ambiguous_group("(a+)+b")
        assert (inner, repeated, quant) == ("a+", "(a+)", "+")
        assert col == 0
        # 嵌套一层时凭「继承子组的歧义源」判中，报的是最外面那把量词
        assert find_repeated_ambiguous_group("((a+))+") == (0, "(a+)", "((a+))", "+")

    def test_the_screen_itself_is_one_pass_over_the_pattern(self):
        """静态闸自己不能是新 DoS 面：最深嵌套与最长合法模式都只走单遍 ⇒ 用返回形状验
        「它没有回头重扫」这件事做不到，这里钉住的是**最坏输入也给出判决**而不是抛错。"""
        deep = "(" * 100 + "a+" + ")" * 100 + "+"
        assert find_repeated_ambiguous_group(deep) is not None
        assert find_repeated_ambiguous_group("x" * MAX_PATTERN_LENGTH) is None
        assert find_repeated_ambiguous_group("[" + "x" * 1000) is None
        assert find_repeated_ambiguous_group("(?P<") is None


class TestParserBranches:
    """静态闸**自己**的支路：读不出形状时要「不下判断」，而不是漏判、误伤或炸域外异常

    改前这一族没有一格被测（覆盖率把 26 条语句、15 条支路点名成缺数），而其中三条
    根本不是「不下判断」：`int("2,3")` / `int(" ")` / `int("²")` 当场抛 `ValueError`，
    它既不是 `DataValidationError` 也不是 `re.error`，`_search_regex` 那一层接不住。
    """

    #: 现量 21 格：每格点名一条「认不出 ⇒ 读 `None`」的支路
    UNRECOGNISED = (
        ("字符类未闭合", "[a"),
        ("(?#…) 没有闭合的 )", "(?#abc"),
        ("(?< 之后找不到 >", "(?<x"),
        ("(?P< 之后找不到 >", "(?P<"),
        ("(?P 之后既不是 < 也不是 =", "(?Pxy)"),
        ("括号不配对（少一个 )）", "(a+"),
        ("结尾裸反斜杠", "(a+)" + chr(92)),
        ("量词前面没有可重复的单元", "(+a)"),
        ("| 之后紧跟量词", "(?:a|+b)"),
        ("{ 之后没有 }", "a{100"),
        ("计数跨度超过 21", "a{" + "1" * 25 + "}"),
        ("计数体含字母", "a{x}"),
        ("计数体只有空格", "a{ }"),
        ("计数体 isdigit 但 int 不认", "a{" + chr(0xb2) + "}"),
        ("上下界写反", "(a{3,2})+"),
        ("(?<name> 命名组", r"(?<year>\d{4})"),
        ("前瞻 (?=", "(a+(?=b))"),
        ("后顾 (?<=", "(a+(?<=b))"),
        ("后顾 (?<!", "(a+(?<!b))"),
        ("反向引用 (?P=", "(?P<x>a)(?P=x)"),
        ("首位置 ] 之后类未闭合", "[^]]a"),
    )

    #: 现量 8 格：改前漏判（指数形却读 `None`），改后按 `re` 口径判中，读数逐格钉死
    NOW_JUDGED = (
        ("开区间 {0,} 套可变长组", r"((a+)){0,}b", (0, "(a+)", "((a+))", "{0,}")),
        ("开区间 {,} 套可变长组", r"((a+)){,}b", (0, "(a+)", "((a+))", "{,}")),
        ("开区间直接套可变长单元", r"(a+){0,}b", (0, "a+", "(a+)", "{0,}")),
        ("类内字面 [ 的指数形", r"([ab[]+)+b", (0, "[ab[]+", "([ab[]+)", "+")),
        ("类首位置 ] 的指数形", r"([]a]+)+b", (0, "[]a]+", "([]a]+)", "+")),
        ("纯开关指令 (?i) 之后", r"(?i)(a+)+b", (4, "a+", "(a+)", "+")),
        ("纯开关指令 (?s) 之后", r"(?s)(a*)*c", (4, "a*", "(a*)", "*")),
        ("条件组之后（改前一枚扩展记号废掉整把闸）",
         "()(?(1)a)(a+)+b", (9, "a+", "(a+)", "+")),
    )

    def test_the_branch_face_is_the_measured_size(self):
        assert (len(self.UNRECOGNISED), len(self.NOW_JUDGED)) == (21, 8)

    def test_unrecognised_shapes_give_no_verdict(self):
        for label, source in self.UNRECOGNISED:
            assert find_repeated_ambiguous_group(source) is None, label

    def test_the_five_shapes_now_get_a_verdict(self):
        for label, source, expected in self.NOW_JUDGED:
            assert find_repeated_ambiguous_group(source) == expected, label

    def test_the_optional_and_starred_forms_are_read_like_re(self):
        """两对「同形必须同判」：`{0,1}` ≡ `?`（实测线性 ⇒ 放行）、`{0,}` ≡ `*`（判中）"""
        assert find_repeated_ambiguous_group("(a?)+b") is None
        assert find_repeated_ambiguous_group("(a{0,1})+b") is None
        assert find_repeated_ambiguous_group("(a{,1})+b") is None
        assert find_repeated_ambiguous_group("(a*)*c") is not None
        assert find_repeated_ambiguous_group("(a+){0,}b") is not None
        assert find_repeated_ambiguous_group("(a+){,}b") is not None

    def test_no_foreign_exception_escapes_the_parser(self):
        """`int()` 那一族的回归守卫：只允许「判中」或「放行」，出现第三种异常就红。"""
        for source in ("a{1,2,3}", "a{ }", "a{" + chr(0xb2) + "}", "a{1, 2}",
                       "a{,}", "a{ 1,2}", "((a+)){2, }"):
            try:
                find_repeated_ambiguous_group(source)
            except Exception as exc:                      # noqa: BLE001 - 这里要的就是「什么都不能抛」
                raise AssertionError("%r 抛出域外异常 %s" % (source, type(exc).__name__))

    def test_literal_brace_patterns_stay_allowed_by_the_authority(self):
        """`re` 把这三形当**字面量**，所以权威面必须放行而不是出声：改前它们抛 ValueError。"""
        for source in ("a{1,2,3}", "a{ }", "a{" + chr(0xb2) + "}"):
            assert require_search_pattern("query", source) == source

    def test_library_entry_returns_a_result_not_a_foreign_exception(self):
        searcher = EnhancedSearcher(ITEMS)
        for source in ("a{1,2,3}", "a{ }"):
            assert searcher.search(source, method="regex").total_matches == 0

    def test_http_face_no_longer_leaks_a_500_on_a_literal_brace(self, search_env):
        """`a{1,2,3}` 是合法正则（字面量）⇒ 200 空结果，而不是 500 固定文案。"""
        response = post(search_env, query="a{1,2,3}")
        assert response.status_code == 200, response.text


class TestSingleAuthority:
    """三道闸只有一个产地，且接线点现读得到"""

    def test_the_wiring_is_where_the_verdict_says(self):
        text = (AI_DIR / "augmentor" / "search_enhanced.py").read_text(encoding="utf-8")
        assert text.count(
            "re.compile(require_search_pattern(\"query\", query), re.IGNORECASE)"
        ) == 1

    def test_no_second_length_authority_in_shipped_code(self):
        """产品码里除 `validation.py` 之外没有第二处给模式串定长度（判的是 `MAX_PATTERN_LENGTH`
        这个符号本身，不是某个具体消息）。"""
        readers = []
        for path in sorted((AI_DIR / "augmentor").rglob("*.py")) + sorted(
                (AI_DIR / "api").rglob("*.py")):
            if "__pycache__" in path.parts or path.name == "validation.py":
                continue
            if "MAX_PATTERN_LENGTH" in path.read_text(encoding="utf-8"):
                readers.append(path.name)
        assert readers == []

    def test_authority_reuses_the_scalar_family_entry_check(self):
        """非空那一刀共用 `require_string`：`None` 走的是「字段值是 None 却被按字符串用」
        那条既有判据，不在模式串这里另立一套。"""
        for bad in (None, "", "   ", 123, True, ["a+"]):
            with pytest.raises(DataValidationError):
                require_search_pattern("query", bad)

    def test_type_errors_do_not_escape_the_authority(self):
        """改前 `query=None` 让 `re.compile` 抛 `TypeError`——`except re.error` 接不住，
        HTTP 面降成 500 固定文案；改后同样本走域错误 ⇒ 400。"""
        with pytest.raises(DataValidationError):
            EnhancedSearcher(ITEMS).search(None, method="regex")
        with pytest.raises(TypeError):
            re.compile(None)


class TestLengthCap:
    """长度那一维：上限两侧都要有读数"""

    def test_max_length_pattern_is_accepted(self):
        assert require_search_pattern("query", "a" * MAX_PATTERN_LENGTH) == "a" * MAX_PATTERN_LENGTH

    def test_one_over_max_is_named_in_the_error(self):
        long_pattern = "a" * (MAX_PATTERN_LENGTH + 1)
        with pytest.raises(DataValidationError) as caught:
            require_search_pattern("query", long_pattern)
        message = str(caught.value)
        assert str(MAX_PATTERN_LENGTH + 1) in message and "上限" in message

    def test_the_cap_value_is_the_measured_one(self):
        """1024 是拍下来的数，不是随手写的：改它要连这条断言与账本 A345 的判决一起改。"""
        assert MAX_PATTERN_LENGTH == 1024


class TestBehaviorDifference:
    """接线之后的对外形状：拒在语料之前，合法面一格未动"""

    def test_danger_shape_raises_before_any_data_is_read(self):
        """空语料也要拒 ⇒ 参数校验先于数据面（与 L267 同一口径），
        也证明指数级回溯**没有机会**跑起来。"""
        searcher = EnhancedSearcher([])
        for pattern in ("(a+)+b", "(.*)*c", r"(\w+\s?)*", "(?i)(a+)+b"):
            with pytest.raises(DataValidationError) as caught:
                searcher.search(pattern, method="regex")
            assert "指数级回溯" in str(caught.value)

    def test_message_names_the_repeated_group_and_the_inner_quantifier(self):
        with pytest.raises(DataValidationError) as caught:
            require_search_pattern("query", "(a+)+b")
        message = str(caught.value)
        assert "'(a+)'" in message and "'a+'" in message and "'+'" in message

    def test_legit_patterns_and_other_methods_are_untouched(self):
        searcher = EnhancedSearcher(ITEMS)
        assert searcher.search("公.*租房", method="regex").total_matches == 2
        # 放行的形状照样跑到匹配那一步：样本里没有 foo/bar ⇒ 0 条，而不是 400
        assert searcher.search("(foo|bar)+", method="regex").total_matches == 0
        # 超长的 query 只在 regex 这一支判：contains 仍按普通字符串处理，刀口没扩宽
        huge = "公租房" * (MAX_PATTERN_LENGTH)
        assert searcher.search(huge, method="contains").total_matches == 0
        assert searcher.search(huge, method="exact").total_matches == 0

    def test_http_face_returns_400_with_the_reason(self, search_env):
        response = post(search_env, query="(a+)+b")
        assert response.status_code == 400, response.text
        assert "指数级回溯" in response.json()["detail"]

    def test_http_face_returns_400_for_overlong_query(self, search_env):
        response = post(search_env, query="a" * (MAX_PATTERN_LENGTH + 1))
        assert response.status_code == 400, response.text
        assert "上限" in response.json()["detail"]

    def test_http_face_still_answers_for_a_normal_pattern(self, search_env):
        response = post(search_env, query="公.*租房")
        assert response.status_code == 200, response.text
        assert response.json()["total_matches"] == 2

    def test_http_face_still_answers_for_contains_with_a_dangerous_string(self, search_env):
        """`contains` 把那串当字面量：不判形状，否则会把「搜一个含括号的串」判成 400。"""
        response = post(search_env, query="(a+)+b", method="contains")
        assert response.status_code == 200, response.text
        assert response.json()["total_matches"] == 0


class TestRulerFalsifiability:
    """三支注入臂：每条判据都要在它被拆掉时红一次"""

    def test_arm_unplugging_the_wiring_lets_the_shape_through(self, monkeypatch):
        """臂 1 摘掉 `_search_regex` 的接线 ⇒ 危险模式重新被照收（本守卫的行为差那几格
        就是靠这句成立才有牙齿）。"""
        monkeypatch.setattr("augmentor.search_enhanced.require_search_pattern",
                            lambda name, value: value)
        result = EnhancedSearcher([]).search("(a+)+b", method="regex")
        assert result.total_matches == 0

    def test_arm_neutralizing_the_screen_lets_the_shape_through(self, monkeypatch):
        """臂 2 把静态闸打成恒真（`None`）⇒ 形状那一刀不再出声。"""
        monkeypatch.setattr(validation, "find_repeated_ambiguous_group",
                            lambda source: None)
        assert require_search_pattern("query", "(a+)+b") == "(a+)+b"

    def test_arm_raising_the_length_cap_stops_naming_the_length(self, monkeypatch):
        """臂 3 把上限打到无限 ⇒ 超长那一刀消失，`TestLengthCap` 的读数必然对不上。"""
        monkeypatch.setattr(validation, "MAX_PATTERN_LENGTH", 10 ** 9)
        assert require_search_pattern("query", "a" * 100000) == "a" * 100000
        with pytest.raises(AssertionError):
            assert len("a" * (validation.MAX_PATTERN_LENGTH + 1)) == MAX_PATTERN_LENGTH + 1
