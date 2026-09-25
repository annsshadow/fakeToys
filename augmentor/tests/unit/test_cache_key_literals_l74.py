# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""L74 / A116：缓存键必须「同一份语义只有一个键」。

`require_ratio` 收 int 也收 float，所以 `temperature: 1` 与 `temperature: 1.0`
是同一份配置。改前 `_cache_key` 直接 `repr()` 这两个值 ⇒ 它们劈成两个键，
症状不是崩而是**命中率被字面量写法切开**（内存与磁盘两路都劈）。

这批用例锁三件事：
1. 同语义的两种字面量必须同键（正向）；
2. 不同语义必须仍然异键（对照组 —— 少了这一组，「键恒为常量」也能过第 1 条）；
3. **float 写法的键逐字节等于改前的键**（缓存失效面：本轮改动不许把既有磁盘缓存整体作废）。
"""
import hashlib
import sys
from pathlib import Path

import pytest

AI_DIR = Path(__file__).resolve().parents[2]
if str(AI_DIR) not in sys.path:
    sys.path.insert(0, str(AI_DIR))

from augmentor.config import ModelConfig  # noqa: E402
from augmentor.exceptions import DataValidationError  # noqa: E402
from augmentor.models.openai_model import OpenAIBackend  # noqa: E402


def _backend(temperature=0.7, top_p=0.95, max_output_tokens=2048, model="m",
             **kwargs):
    return OpenAIBackend(ModelConfig(type="openai", api_key="k", model=model,
                                     temperature=temperature, top_p=top_p,
                                     max_output_tokens=max_output_tokens),
                         **kwargs)


def _payload(temperature, top_p, max_output_tokens, prompt="p",
             model="m", type_="openai"):
    """改前那份逐字节口径的载荷（float 写法下与本轮实现同形）"""
    return "\x00".join([str(type_ or ""), str(model or ""),
                        repr(float(temperature)), repr(float(top_p)),
                        repr(max_output_tokens), prompt])


class TestLiteralsOfOneMeaningShareOneKey:
    """同一份语义的 int 与 float 写法必须落在同一个键上"""

    @pytest.mark.parametrize("integer,float_literal", [
        (1, 1.0),
        (0, 0.0),
        (2, 2.0),
    ])
    def test_temperature_literals_do_not_split(self, integer, float_literal):
        assert (_backend(temperature=integer)._cache_key("p")
                == _backend(temperature=float_literal)._cache_key("p")), (
            "temperature 的 %r 与 %r 是同一份配置，却拿到两个键"
            % (integer, float_literal))

    @pytest.mark.parametrize("integer,float_literal", [(1, 1.0), (0, 0.0)])
    def test_top_p_literals_do_not_split(self, integer, float_literal):
        assert (_backend(top_p=integer)._cache_key("p")
                == _backend(top_p=float_literal)._cache_key("p")), (
            "top_p 的 %r 与 %r 是同一份配置，却拿到两个键"
            % (integer, float_literal))

    def test_disk_cache_hits_across_the_two_literal_spellings(self, tmp_path):
        """端到端物证：用 `temperature: 1` 写进磁盘的条目，必须被 `1.0` 读到

        只比键字符串的用例证明不了「这条键真的穿过磁盘那一层」，而用户看到的
        症状是「换了个写法，缓存全部重来」。
        """
        cache_dir = tmp_path / "resp"
        first = _backend(temperature=1, response_cache_dir=str(cache_dir))
        first._call_api = lambda prompt: "变体"
        assert first.generate("p") == "变体"

        second = _backend(temperature=1.0, response_cache_dir=str(cache_dir))
        second._call_api = lambda prompt: pytest.fail("应命中磁盘缓存，不该再打 API")
        assert second.generate("p") == "变体"
        assert second.request_count == 1


class TestDifferentMeaningsStillSplit:
    """对照组：归一化不许把不同值并成同一条

    少了这一组，`_cache_key` 直接返回常量也能过上一类用例（L72 的「只测一族
    单成员的守卫等于没守卫」在这里的镜像）。
    """

    @pytest.mark.parametrize("left,right", [
        (0.5, 0.6),
        (1, 1.1),
        (0.99, 1),
        (0, 1),
    ])
    def test_temperature_still_distinguishes(self, left, right):
        assert (_backend(temperature=left)._cache_key("p")
                != _backend(temperature=right)._cache_key("p"))

    @pytest.mark.parametrize("left,right", [(0.9, 0.95), (1, 0.5)])
    def test_top_p_still_distinguishes(self, left, right):
        assert (_backend(top_p=left)._cache_key("p")
                != _backend(top_p=right)._cache_key("p"))

    def test_model_and_type_still_distinguish(self):
        base = _backend()
        assert base._cache_key("p") != _backend(model="m2")._cache_key("p")
        baidu_typed = OpenAIBackend(ModelConfig(type="baidu", api_key="k",
                                                secret_key="s", model="m",
                                                temperature=0.7, top_p=0.95,
                                                max_output_tokens=2048))
        assert base._cache_key("p") != baidu_typed._cache_key("p"), "键未区分 type"

    def test_prompt_still_distinguishes(self):
        backend = _backend()
        assert backend._cache_key("p") != backend._cache_key("q")


class TestKeyFormatDidNotDrift:
    """本轮只并档，不改口径：float 写法的键必须与改前逐字节相同"""

    def test_float_spellings_keep_the_pre_change_digest(self):
        backend = _backend(temperature=0.7, top_p=0.95, max_output_tokens=2048)
        expected = hashlib.sha256(
            _payload(0.7, 0.95, 2048).encode("utf-8")).hexdigest()
        assert backend._cache_key("p") == expected, (
            "键的载荷格式变了 ⇒ 既有磁盘缓存条目整体失效，这不在本轮的拍定里")

    def test_int_spellings_move_onto_the_float_key(self):
        """int 写法的**新**键必须等于按 float 档拼出的载荷（换到哪一档要说清）"""
        expected = hashlib.sha256(
            _payload(1.0, 0.95, 2048).encode("utf-8")).hexdigest()
        assert _backend(temperature=1)._cache_key("p") == expected


class TestMaxOutputTokensNeedsNoNormalization:
    """A116 行里那条「别去修」的分支要能被反证

    `max_output_tokens` 走 `require_count`（拒非整数），所以 `2048` 与 `2048.0`
    根本不可能同时存在 ⇒ 不需要归一化。这条判据一旦被人放宽，本组用例就会红。
    """

    def test_float_token_count_is_rejected_by_the_runtime_face(self):
        with pytest.raises(DataValidationError):
            ModelConfig(type="openai", api_key="k", max_output_tokens=2048.0)

    def test_float_token_count_is_rejected_by_the_static_face(self):
        from augmentor.config_validator import validate_config

        result = validate_config(
            {"models": {"default": "ernie",
                        "m": {"type": "openai", "max_output_tokens": 2048.0}}})
        assert not result.is_valid, "静态面放宽了 ⇒ 缓存键的 int/float 劈叉会复活"

    def test_equal_int_literals_keep_one_key(self):
        assert (_backend(max_output_tokens=2048)._cache_key("p")
                == _backend(max_output_tokens=2048)._cache_key("p"))
        assert (_backend(max_output_tokens=2048)._cache_key("p")
                != _backend(max_output_tokens=2049)._cache_key("p"))
