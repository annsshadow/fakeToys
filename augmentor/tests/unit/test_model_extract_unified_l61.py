# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""A111：六个模型后端的 extract_json_from_response 必须等价于基类唯一解析器

L59(A108)/L60(A110) 修完 pipeline 与 expander 后，本仓还剩最后一族同构缺陷：
openai/ollama/ernie 三个后端各自手搓了一份 JSON 数组解析器，与基类
`models/base.py:extract_json_array` 逐字或近似重复，且三份彼此口径不一：

- openai 少了 `isinstance(result, list)` 守卫 ⇒ 纯 JSON 对象响应会违反 `-> list`
  契约、把 dict 当数组返回；
- ernie 只做了括号切片那一半、漏了「先直接解析」那一半；
- claude / gemini 早已 `return extract_json_array(response)`。

L61 把 openai/ollama/ernie 也接上同一个 `extract_json_array`。本文件用「行为等价
表」把六份实现钉死在唯一解析器上——将来谁再把逻辑抄回去并改动，对应形状会红。
"""

import pytest

from augmentor.config import ModelConfig
from augmentor.exceptions import ModelResponseError
from augmentor.models.base import extract_json_array
from augmentor.models.claude import ClaudeBackend
from augmentor.models.ernie import ERNIEBackend
from augmentor.models.gemini import GeminiBackend
from augmentor.models.ollama import OllamaBackend
from augmentor.models.openai_model import OpenAIBackend


def _openai():
    return OpenAIBackend(ModelConfig(type="openai", api_key="k", model="m"))


def _ollama():
    return OllamaBackend(ModelConfig(type="ollama", base_url="http://localhost:11434", model="qwen"))


def _ernie():
    return ERNIEBackend(ModelConfig(type="baidu", api_key="ak", secret_key="sk", model="ernie"))


def _claude():
    return ClaudeBackend(ModelConfig(type="claude", api_key="k", model="m"))


def _gemini():
    return GeminiBackend(ModelConfig(type="gemini", api_key="k", model="m"))


BACKENDS = {
    "openai": _openai,
    "ollama": _ollama,
    "ernie": _ernie,
    "claude": _claude,
    "gemini": _gemini,
}

# 覆盖直接解析 / 括号切片 / 围栏 / 空数组 / 对象+数组 / 无数组各形状
EQUIV_CASES = [
    '[{"instruction": "a"}]',
    '前缀 [{"a": 1}] 后缀',
    '```json\n[1, 2]\n```',
    '[]',
    '{"a": 1} [1, 2]',
    "no array here",
    '{"a": 1}',  # 纯对象：唯一解析器应抛错（openai 旧版在此返回 dict）
]


@pytest.mark.parametrize("name", list(BACKENDS))
@pytest.mark.parametrize("response", EQUIV_CASES)
def test_adapter_matches_canonical_parser(name, response):
    """每个后端的 extract_json_from_response 与 extract_json_array 行为完全一致。"""
    backend = BACKENDS[name]()

    try:
        expected = extract_json_array(response)
    except ModelResponseError:
        with pytest.raises(ModelResponseError):
            backend.extract_json_from_response(response)
    else:
        assert backend.extract_json_from_response(response) == expected


def test_openai_no_longer_leaks_dict_on_pure_object():
    """A111 的契约修复面：openai 旧版把纯 JSON 对象当数组返回 dict，现必须抛错。

    这条是本轮唯一改变既有（未测）行为的用例，单独立出来写明意图，防止有人误当成
    「回归」又给退回去。
    """
    backend = _openai()
    with pytest.raises(ModelResponseError):
        backend.extract_json_from_response('{"a": 1}')


def test_all_adapters_route_through_shared_helper_source():
    """结构守卫：五份实现里不再有 `json.loads(response)` 手搓解析，只剩一行委派。"""
    import inspect

    for name, factory in BACKENDS.items():
        src = inspect.getsource(factory().extract_json_from_response)
        assert "extract_json_array(response)" in src, f"{name} 未复用共享解析器"
        assert "json.loads" not in src, f"{name} 仍在方法体内手搓 json.loads"
