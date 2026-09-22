# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""json_extract LLM 响应 JSON 提取模块测试

覆盖直接解析、markdown 围栏、前后缀说明文字（平衡扫描）、
尾随逗号修复、字符串内括号感知、数组提取限定、失败路径。
"""

import json

import pytest

from augmentor import ExtractResult, extract_json, extract_json_list
from augmentor.json_extract import _balanced_span, _strip_trailing_commas


class TestExtractJson:
    def test_direct_object(self):
        result = extract_json('{"a": 1}')
        assert result.ok is True
        assert result.method == "direct"
        assert result.value == {"a": 1}

    def test_direct_array(self):
        result = extract_json("[1, 2, 3]")
        assert result.ok is True
        assert result.value == [1, 2, 3]

    def test_markdown_fence(self):
        text = '这是结果：\n```json\n{"ok": true}\n```\n谢谢'
        result = extract_json(text)
        assert result.ok is True
        assert result.method == "fence"
        assert result.value == {"ok": True}

    def test_prose_wrapped_balanced(self):
        text = '结果如下 {"name": "张三", "age": 30} 以上是答案'
        result = extract_json(text)
        assert result.ok is True
        assert result.method == "balanced"
        assert result.value == {"name": "张三", "age": 30}

    def test_trailing_comma_repaired(self):
        text = '{"a": 1, "b": [1, 2,],}'
        result = extract_json(text)
        assert result.ok is True
        assert result.value == {"a": 1, "b": [1, 2]}

    def test_brackets_inside_string_not_miscounted(self):
        text = '说明 {"note": "包含 } 和 [ 括号", "x": 1} 结束'
        result = extract_json(text)
        assert result.ok is True
        assert result.value["note"] == "包含 } 和 [ 括号"

    def test_nested_array_in_prose(self):
        text = '数据 [["a","b"],["c"]] 完毕'
        result = extract_json(text)
        assert result.ok is True
        assert result.value == [["a", "b"], ["c"]]

    def test_unparseable_returns_none(self):
        result = extract_json("完全没有 JSON 结构")
        assert result.ok is False
        assert result.method == "none"
        assert result.value is None

    def test_empty_and_non_str(self):
        assert extract_json("").ok is False
        assert extract_json(None).ok is False

    def test_result_to_dict(self):
        r = extract_json('{"k": "v"}')
        d = r.to_dict()
        assert d["ok"] is True and d["value"] == {"k": "v"}
        assert isinstance(r, ExtractResult)


class TestExtractJsonList:
    def test_list_returned(self):
        result = extract_json_list('前缀 ["x", "y"] 后缀')
        assert result.ok is True
        assert result.value == ["x", "y"]

    def test_object_rejected(self):
        result = extract_json_list('{"a": 1}')
        assert result.ok is False
        assert result.method == "not_list"

    def test_fence_list(self):
        result = extract_json_list('```json\n[{"i": 1}]\n```')
        assert result.ok is True
        assert result.value == [{"i": 1}]


class TestBalancedEscapes:
    def test_escaped_quote_inside_string(self):
        """字符串内转义引号不应提前终止平衡扫描"""
        # 实际数据含反斜杠转义的引号（raw 字符串保留 \\）
        text = r'结果 {"msg": "含 \" 引号", "n": 2} 完'
        result = extract_json(text)
        assert result.ok is True
        assert result.value == {"msg": '含 " 引号', "n": 2}

    def test_backslash_in_string(self):
        text = r'前 {"path": "a\\b"} 后'
        result = extract_json(text)
        assert result.ok is True
        assert result.value["path"] == "a\\b"


class TestFenceRepair:
    def test_fence_with_trailing_comma(self):
        text = '```json\n{"a": 1, "b": [1,],}\n```'
        result = extract_json(text)
        assert result.ok is True
        assert result.method == "fence"
        assert result.value == {"a": 1, "b": [1]}

    def test_fence_json_keyword(self):
        text = '```json\n[{"x": 1}]\n```'
        result = extract_json(text)
        assert result.ok is True
        assert result.value == [{"x": 1}]


class TestNoStructure:
    def test_plain_prose_none(self):
        assert extract_json("就一句话，没有任何结构").ok is False

    def test_partial_object_unclosed(self):
        assert extract_json('{"a": 1').ok is False
        assert extract_json('[1, 2').ok is False


class TestHelpers:
    def test_balanced_span_missing(self):
        assert _balanced_span("没有括号") is None

    def test_balanced_span_partial(self):
        assert _balanced_span('{"a": 1') is None

    def test_strip_trailing_commas_preserves_strings(self):
        text = '{"a": "x,]", "b": 1,}'
        cleaned = _strip_trailing_commas(text)
        assert json.loads(cleaned) == {"a": "x,]", "b": 1}

    def test_to_dict_keys(self):
        r = ExtractResult(ok=False, method="none")
        assert set(r.to_dict()) == {"ok", "value", "method"}
