# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""L335 波 1a 守卫：A385 成功 200 回显安全化、A358 在场非字符串即拒。

零 `parametrize`，不扰动 l97 / l98 量面；判据都是行为面 —— 把产品码改回旧形状就红。
"""

import datetime
import json

import pytest
from fastapi import HTTPException


# ------------------------------------------------------------------ A385
def test_success_echo_is_utf8_safe():
    """孤立代理不再打穿成功 200 的序列化：回显经 UTF-8 安全化"""
    from augmentor.search_enhanced import SearchResult

    result = SearchResult(items=[], total_matches=0, query_time_ms=0.0,
                          query="找\ud800这个词", method="contains")
    receipt = result.to_dict()
    assert "\ud800" not in receipt["query"]
    assert "找" in receipt["query"] and "这个词" in receipt["query"]
    json.dumps(receipt, ensure_ascii=False).encode("utf-8")  # 这一步改前抛 UnicodeEncodeError


def test_success_echo_keeps_ordinary_text_and_emoji_untouched():
    """反证臂：合法文本与成对代理（emoji）一个字节都不许被改"""
    from augmentor.search_enhanced import SearchResult

    for query in ("正常查询", "\U0001F600 表情", "mixed 中英 123"):
        receipt = SearchResult(items=[], total_matches=0, query_time_ms=0.0,
                               query=query, method="contains").to_dict()
        assert receipt["query"] == query


def test_echo_is_sanitized_in_the_sdk_not_by_a_global_response_class():
    """静态面：安全化落在 SearchResult.to_dict()，不靠换 default_response_class
    （那条路会把服务端自己算出的 NaN 一起抹掉，与真值性判据相反）"""
    import pathlib

    root = pathlib.Path(__file__).resolve().parents[2]
    sdk = (root / "augmentor" / "search_enhanced.py").read_text(encoding="utf-8")
    main = (root / "api" / "main.py").read_text(encoding="utf-8")
    assert '"query": self.query.encode' in sdk
    assert "default_response_class=" not in main, "不许用全局响应类掩盖服务端自己算出的 NaN"


# ------------------------------------------------------------------ A358
def test_as_column_rejects_non_string_instead_of_normalizing():
    """改前 `str()` 归一后照常算分（200），现在与 merge 侧同口径回 400"""
    from api.routes.dataset_tools import _as_column
    from augmentor.exceptions import DatasetTextError

    with pytest.raises(DatasetTextError) as exc:
        _as_column([{"instruction": 1, "output": 2}], "instruction", "生成侧")
    assert "第 0 条" in str(exc.value) and "instruction" in str(exc.value)
    assert "int" in str(exc.value)


def test_as_column_keeps_the_missing_field_verdict():
    """反证臂：缺列的逐条点名文案没被新判据吞掉"""
    from api.routes.dataset_tools import _as_column

    with pytest.raises(HTTPException) as exc:
        _as_column([{"output": "a"}], "instruction", "生成侧")
    assert exc.value.status_code == 400
    assert "没有字段" in exc.value.detail


def test_as_column_returns_plain_text_unchanged():
    """反证臂：合法文本列原样返回，不做多余加工"""
    from api.routes.dataset_tools import _as_column

    assert _as_column([{"text": "甲"}, {"text": "乙"}], "text", "评估") == ["甲", "乙"]


def test_field_statistics_rejects_non_string_instead_of_measuring_its_length():
    """改前 CLI stats 把 int 读成「长度 1 的文本」，现在抛领域异常"""
    from augmentor.exceptions import DataValidationError
    from augmentor.statistics import calculate_statistics

    with pytest.raises(DataValidationError):
        calculate_statistics([{"instruction": 1, "output": 2}])


def test_read_side_keeps_its_explicit_normalization_exception():
    """读入侧的产出归一仍合法：日期这类非原生标量被转成文本再落库，不是「把值当文本用」"""
    from augmentor.converter import excel_cell

    assert excel_cell(3) == 3
    assert excel_cell(datetime.date(2026, 1, 1)) == "2026-01-01"
