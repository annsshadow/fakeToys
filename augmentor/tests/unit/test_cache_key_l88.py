# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""第 88 轮：CachedProcessor 缓存键结构化归一。

旧键是 `str(args)` + `str(sorted(kwargs.items()))`：嵌套 dict 的插入顺序直接决定
字符串形态，同一份逻辑请求换序后必然 miss。新实现递归归一（dict 按稳定键序、
list/tuple/set 带类型标记），判据是「逻辑等价的调用命中同一缓存，不同调用不串」。
"""

from augmentor.cache import CachedProcessor, _stable_value


def _legacy_key(*args, **kwargs):
    """第 88 轮之前的键（仅用于证明用例非空转）。"""
    import hashlib
    import json

    key_data = {"args": str(args), "kwargs": str(sorted(kwargs.items()))}
    return hashlib.md5(json.dumps(key_data, default=str).encode()).hexdigest()


def test_nested_dict_insertion_order_hits_cache():
    calls = []

    def proc(payload):
        calls.append(payload)
        return payload["a"] + payload["b"]["x"]

    wrapped = CachedProcessor(proc)
    first = {"a": 1, "b": {"x": 2, "y": 3}}
    second = {"b": {"y": 3, "x": 2}, "a": 1}

    assert _legacy_key(first) != _legacy_key(second), "旧键对换序确实不同，用例非空转"
    assert wrapped.process(first) == 3
    assert wrapped.process(second) == 3
    assert len(calls) == 1, "逻辑等价的换序请求必须命中缓存"


def test_kwargs_order_hits_cache():
    calls = []

    def proc(payload, alpha=0, beta=0):
        calls.append((payload, alpha, beta))
        return alpha + beta

    wrapped = CachedProcessor(proc)
    assert wrapped.process({"k": 1}, alpha=1, beta=2) == 3
    assert wrapped.process({"k": 1}, beta=2, alpha=1) == 3
    assert len(calls) == 1


def test_distinct_payloads_do_not_share_cache():
    calls = []

    def proc(payload):
        calls.append(tuple(payload))
        return len(payload)

    wrapped = CachedProcessor(proc)
    assert wrapped.process([1, 2]) == 2
    assert wrapped.process([1, 2, 3]) == 3
    assert len(calls) == 2, "不同参数不得复用同一条缓存"


def test_stable_value_marks_container_types():
    # dict / list / tuple / set 各自带类型标记，避免形状不同却压成同一串
    assert _stable_value({"a": 1}) != _stable_value([["a", 1]])
    assert _stable_value((1, 2)) != _stable_value([1, 2])
    assert _stable_value(1) != _stable_value("1")
    # 同一个 dict 两种插入顺序：归一后逐字节一致
    assert _stable_value({"a": 1, "b": 2}) == _stable_value({"b": 2, "a": 1})
