# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""L267：「调用方给的字符串被编译成正则模式」那一族的普查与出声契约

缺陷形状：`EnhancedSearcher._search_regex` 把 `re.compile(query)` 包在遍历
所有字段的 `try` 里，`except re.error` 只打一行 `logger.warning` 就返回
`matches`（空）。于是 `POST /api/dataset/search` 配 `method="regex"` 传一个
坏模式（`[invalid`）读到的是 **200 + `total_matches: 0`**——用户看到的是
「语料里没有」，真相是参数写错了。这与 `normalize_filters` 的 docstring 里
已经写明的 A27/A28④ 同一形状，而那一格项目已经判过：形状不对就地报错。

本模块的三把尺子：

1. **普查名册**（AST，钉在族上而非全仓扫）：出厂面（`augmentor/`、`api/`、
   `cli.py`）里第一实参**不是字符串字面量**的 `re.<fn>` 调用，实测恰好两格，
   逐格点名其权威面——`search_enhanced.py` 那格的模式来自调用方（HTTP 请求体
   / CLI `--query`），`validation.py` 那格来自出厂预置规则且 HTTP/CLI 只传
   `preset=`、传不进模式串。
2. **吞异常形状检测器**：出厂面里 `except re.error`（含 `re.PatternError`）
   的处理器，若其体内没有任何 `raise` 就计数。实测 0。旧实现落在这把尺子上。
3. **行为差**：坏模式 ⇒ `DataValidationError`（⇒ HTTP 400、CLI `错误: …` + 退出码 1）
   点名模式串与位置；合法模式与其他方法、以及其他端点的坏输入语义都不受影响。

尺子自证走零写盘通道（`compile()` 内存里的旧形状喂给检测器），不改仓库文件。
"""

import ast
import json
import re
import tempfile
from pathlib import Path

import pytest
from fastapi.testclient import TestClient

from augmentor.exceptions import DataValidationError
from augmentor.search_enhanced import EnhancedSearcher, search_dataset
from augmentor.validation import DatasetValidator
from augmentor.cli.commands.data_ops import run_search
from augmentor.cli.parser import build_parser

AI_DIR = Path(__file__).resolve().parents[2]

#: `re` 模块里第一实参是模式串的函数名。
REGEX_FNS = frozenset({"compile", "search", "match", "fullmatch",
                       "sub", "subn", "findall", "finditer", "split"})

#: 出厂面 = 产品包 + API 层 + CLI 入口。`archive/` 是留档脚本（模式来自同文件
#: 源码字面量列表 `synonym_rules`，不在任何产品入口的调用图上，也不进覆盖率面），
#: `scripts/` 是开发期工具，两者都不算「调用方可注入模式」的面。
SHIPPED_DIRS = ("augmentor", "api")
SHIPPED_FILES = ("cli.py",)

#: 普查实测名册：(相对路径, 函数名)。改动它必须同时改动那一格的权威面定性。
NON_LITERAL_PATTERNS = (
    ("augmentor/search_enhanced.py", "compile"),
    ("augmentor/validation.py", "compile"),
)

#: 名册锚点（现读源码，逐格唯一命中）：把「格子还在原位」与定性绑在一起，
#: 免得名册绿而那一格已被搬走或改写。
ROSTER_ANCHORS = {
    "augmentor/search_enhanced.py":
        "pattern = re.compile(require_search_pattern(\"query\", query), re.IGNORECASE)",
    "augmentor/validation.py":
        "re.compile(p, re.IGNORECASE) for p in patterns",
}

ITEMS = [
    {"instruction": "如何申请公租房？", "input": "", "output": "请登录官网申请"},
    {"instruction": "公租房需要什么材料？", "input": "", "output": "身份证"},
]


def shipped_sources():
    """出厂面的全部 `.py` 源文件（排序、去 `__pycache__`）。

    Returns:
        路径列表
    """
    paths = []
    for name in SHIPPED_DIRS:
        paths.extend(p for p in (AI_DIR / name).rglob("*.py"))
    for name in SHIPPED_FILES:
        path = AI_DIR / name
        if path.exists():
            paths.append(path)
    return sorted(p for p in paths if "__pycache__" not in p.parts)


def trees(rel_paths):
    """[(相对路径, AST)]：按源码文本解析，不把模块导入第二遍。

    Args:
        rel_paths: 源文件路径列表

    Returns:
        路径与语法树的成对列表
    """
    out = []
    for path in rel_paths:
        rel = str(path.relative_to(AI_DIR)).replace("\\", "/")
        out.append((rel, ast.parse(path.read_text(encoding="utf-8"), filename=rel)))
    return out


def non_literal_pattern_sites():
    """出厂面里「第一实参不是字符串字面量」的 `re.<fn>` 调用点。

    Returns:
        排序后的 `(相对路径, 函数名)` 列表
    """
    hits = []
    for rel, tree in trees(shipped_sources()):
        for node in ast.walk(tree):
            if not isinstance(node, ast.Call):
                continue
            fn = node.func
            if not (isinstance(fn, ast.Attribute) and isinstance(fn.value, ast.Name)
                    and fn.value.id == "re" and fn.attr in REGEX_FNS):
                continue
            if not node.args:
                continue
            first = node.args[0]
            if isinstance(first, ast.Constant) and isinstance(first.value, (str, bytes)):
                continue
            hits.append((rel, fn.attr))
    return sorted(hits)


def re_error_swallowers(source):
    """数一段源码里「`except re.error` 且体内没有 raise」的处理器。

    Args:
        source: Python 源码文本

    Returns:
        `(行号, 体内语句数)` 列表——旧实现那种「一行 warning 就返回」的形状
    """
    found = []
    for node in ast.walk(ast.parse(source)):
        if not isinstance(node, ast.ExceptHandler):
            continue
        exc = node.type
        names = []
        if isinstance(exc, ast.Name):
            names = [exc.id]
        elif isinstance(exc, ast.Attribute):
            names = [f"{exc.value.id}.{exc.attr}"] if isinstance(exc.value, ast.Name) else []
        elif isinstance(exc, ast.Tuple):
            names = [e.id if isinstance(e, ast.Name) else ast.dump(e) for e in exc.elts]
        if not set(names) & {"re.error", "re.PatternError", "PatternError"}:
            continue
        if not any(isinstance(n, ast.Raise) for n in ast.walk(node) if n is not node):
            found.append((node.lineno, len(node.body)))
    return found


@pytest.fixture
def search_env(tmp_path, monkeypatch):
    """隔离数据白名单与 API key，准备一份可搜索的样本文件

    Returns:
        SimpleNamespace，含 client 与 data 路径
    """
    import os
    from types import SimpleNamespace
    from api.main import app

    monkeypatch.setenv("AUGMENTOR_DATA_ROOTS",
                       os.pathsep.join([str(tmp_path), tempfile.gettempdir()]))
    monkeypatch.delenv("AUGMENTOR_API_KEY", raising=False)
    data = tmp_path / "items.json"
    data.write_text(json.dumps(ITEMS, ensure_ascii=False), encoding="utf-8")
    return SimpleNamespace(client=TestClient(app), data=data)


class TestCensusRoster:
    """普查面：谁的模式串来自调用方"""

    def test_the_roster_is_exactly_the_two_measured_grids(self):
        assert non_literal_pattern_sites() == list(NON_LITERAL_PATTERNS)

    def test_each_grid_is_still_where_the_verdict_says(self):
        for rel, anchor in ROSTER_ANCHORS.items():
            text = (AI_DIR / rel).read_text(encoding="utf-8")
            assert text.count(anchor) == 1, f"{rel} 的锚点不再唯一命中 ⇒ 定性要重读"


class TestSwallowShapeRuler:
    """吞异常那一形：出厂面 0 处"""

    def test_no_re_error_handler_without_a_raise_in_shipped_code(self):
        offenders = []
        for path in shipped_sources():
            rel = str(path.relative_to(AI_DIR)).replace("\\", "/")
            hits = re_error_swallowers(path.read_text(encoding="utf-8"))
            if hits:
                offenders.append((rel, hits))
        assert offenders == []

    def test_the_ruler_still_catches_the_shape_it_replaces(self):
        """零写盘自证：把改前那一段（warning 后返回）在内存里喂给检测器"""
        old_shape = (
            "try:\n"
            "    pattern = re.compile(query)\n"
            "except re.error as e:\n"
            "    logger.warning(e)\n"
        )
        assert re_error_swallowers(old_shape) == [(3, 1)]
        # 同一把尺子对「改成出声」后的形状必须闭嘴
        assert re_error_swallowers(
            "try:\n    re.compile(query)\nexcept re.error as exc:\n    raise ValueError()\n"
        ) == []


class TestBehaviorDelta:
    """行为差：坏模式是参数错，不是零命中"""

    def test_bad_pattern_raises_domain_error_naming_query_and_position(self):
        with pytest.raises(DataValidationError) as caught:
            EnhancedSearcher(ITEMS).search("[invalid", method="regex")
        message = str(caught.value)
        assert "无效的正则表达式" in message
        assert "[invalid" in message, "不点名是哪个模式，调用方还得自己试"
        assert "unterminated character set" in message, "re 给的原因要原样带上"
        assert "位置 0" in message

    def test_rejects_even_with_no_data(self):
        """参数校验先于数据面：空语料也是坏参数，不能退成「没命中」"""
        with pytest.raises(DataValidationError):
            EnhancedSearcher([]).search("(", method="regex")

    def test_search_dataset_entry_also_raises(self):
        with pytest.raises(DataValidationError):
            search_dataset(ITEMS, "((", None, "regex", 10, 0)

    def test_valid_pattern_and_other_methods_keep_working(self):
        """判据只针对 regex 的模式编译，不扩宽到别处"""
        assert EnhancedSearcher(ITEMS).search("公.*租房", method="regex").total_matches == 2
        # contains 把 '[invalid' 当普通字符串：坏串不是坏模式
        assert EnhancedSearcher(ITEMS).search("[invalid", method="contains").total_matches == 0

    def test_http_face_returns_400_not_empty_results(self, search_env):
        """改前这里读成 200 + total_matches 0"""
        response = search_env.client.post(
            "/api/dataset/search",
            json={"input_file": str(search_env.data), "query": "[invalid", "method": "regex"},
        )
        assert response.status_code == 400, response.text
        detail = response.json()["detail"]
        assert "无效的正则表达式" in detail and "[invalid" in detail

        ok = search_env.client.post(
            "/api/dataset/search",
            json={"input_file": str(search_env.data), "query": "公.*租房", "method": "regex"},
        )
        assert ok.status_code == 200 and ok.json()["total_matches"] == 2

    def test_cli_face_raises_too(self, tmp_path):
        """`main()` 的 `except Exception` 把它变成 `错误: …` + 退出码 1"""
        data = tmp_path / "items.json"
        data.write_text(json.dumps(ITEMS, ensure_ascii=False), encoding="utf-8")
        args = build_parser().parse_args(
            ["search", "--input", str(data), "--query", "[invalid", "--method", "regex"]
        )
        with pytest.raises(DataValidationError):
            run_search(args, None)


class TestPresetAuthorityGrid:
    """名册第二格：`validation.py` 的模式来自出厂预置，传不进调用方的串"""

    def test_preset_rules_are_the_only_pattern_authority_reachable_from_http(self):
        from api.routes.dataset_tools import ValidateRequest

        # 校验端点的请求体只有 preset（与 input_file 等），没有模式字段 ⇒ 编译失败
        # 只可能是出厂规则坏了，那是「代码错」而不是「参数错」，保持原样传播。
        assert "rules" not in ValidateRequest.model_fields
        assert "forbidden_patterns" not in ValidateRequest.model_fields

    def test_bad_preset_pattern_propagates_rather_than_silently_skipping(self):
        validator = DatasetValidator(rules={
            "required_fields": ["instruction"], "forbidden_patterns": ["("],
        })
        validator.validate([])  # 空数据集不触发惰性编译
        with pytest.raises(re.error):
            validator.validate([{"instruction": "q"}])
