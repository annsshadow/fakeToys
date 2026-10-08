# -*- coding: utf-8 -*-
"""L175 / B245：搜索方法封闭清单下沉 SDK 层（未知 method 不再静默回落 contains）。

改前形状：清单只住在 API 面（`dataset_tools.SEARCH_METHODS` 的 400 判据），
CLI 面靠 argparse choices；但 **SDK 直构**的调用方（`search_dataset(...,
method='regex_typo')`）会**静默回落 contains**——请求 regex 检索的得到
contains 结果与分数，不出任何声（checkpoint 症状族的语义漂移档）。A77 把
清单权威提到 `search_enhanced.SEARCH_METHODS`，API 面共引同一份（`is` 钉死），
`search` 入口 `require_choice` 判封闭清单、None 单独判（必填参数不适用
「未传回落默认」语义）、空串由 require_string 那刀判。
"""
import pathlib

import pytest

from augmentor.exceptions import DataValidationError
from augmentor.search_enhanced import SEARCH_METHODS, EnhancedSearcher, search_dataset

_ITEMS = [{"instruction": "apple pie", "output": "o"}, {"instruction": "banana", "output": "o"}]


def test_unknown_method_is_rejected_not_silently_downgraded():
    """未知方法名：拒并列出全部合法取值（改前是静默回落 contains）。"""
    with pytest.raises(DataValidationError) as ei:
        search_dataset(_ITEMS, "apple", method="typo")
    msg = str(ei.value)
    for m in SEARCH_METHODS:
        assert m in msg, f"报错文案必须列出全部合法取值，缺 {m!r}：{msg}"
    assert "typo" in msg


@pytest.mark.parametrize("bad", ["", "  ", "EXACT", "Ngram", 5, ["fuzzy"]])
def test_bad_shape_methods_are_rejected(bad):
    """形状错档（空/空白/大小写/非 str）各出各的准确错，不落到「回落 contains」。"""
    with pytest.raises(DataValidationError):
        search_dataset(_ITEMS, "apple", method=bad)


def test_null_method_is_rejected():
    """显式传 None = 用户错误，不是「未传」——必填参数拒。"""
    with pytest.raises(DataValidationError) as ei:
        search_dataset(_ITEMS, "apple", method=None)
    assert "null" in str(ei.value).lower()


# 字面清单（l97 判据：parametrize 值位必须是字面——list(SEARCH_METHODS) 是动态
# 调用会进盲面桶；清单与 SEARCH_METHODS 的同一性由共引用例另行钉住）
@pytest.mark.parametrize("m", ["exact", "contains", "ngram", "fuzzy", "regex"])
def test_every_listed_method_still_passes(m):
    """五合法值各跑通（ngram/fuzzy 需要对应参数，其余按默认档即可）。"""
    from augmentor.search_enhanced import SearchResult

    kw = {"ngram": {"ngram_n": 2}, "fuzzy": {"fuzzy_threshold": 0.6}}.get(m, {})
    result = search_dataset(_ITEMS, "apple", method=m, **kw)
    assert isinstance(result, SearchResult)
    assert result.total_matches >= 0


def test_the_api_face_shares_the_sdk_producer():
    """A77 共引钉：API 路由的清单 `is` SDK 层那一份（不是抄一份）。"""
    import api.routes.dataset_tools as tools
    assert tools.SEARCH_METHODS is SEARCH_METHODS
