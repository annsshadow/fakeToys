# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""第 84 轮：因果 attention mask 按形状缓存。

mask 只由 (T, T_total) 决定，旧实现每层/每次前向都重新分配并 `triu`。
缓存必须保持数值语义、复用同一只只读数组，且容量有界。
"""

import numpy as np
import pytest

from minigrad import nn
from minigrad.engine import Tensor


@pytest.fixture(autouse=True)
def clear_mask_cache():
    nn._MASK_CACHE.clear()
    yield
    nn._MASK_CACHE.clear()


def test_mask_values_are_causal():
    mask = nn._causal_mask(4, 4)
    assert mask.shape == (4, 4)
    assert np.all(mask[np.tril_indices(4)] == 0.0)
    assert np.all(mask[np.triu_indices(4, k=1)] == -1e9)


def test_single_query_with_cache_can_see_all_past():
    mask = nn._causal_mask(1, 5)
    assert mask.shape == (1, 5)
    assert np.all(mask == 0.0), "缓存解码时新 token 应能 attend 全部过去"


def test_same_shape_reuses_same_array():
    first = nn._causal_mask(8, 8)
    second = nn._causal_mask(8, 8)
    assert first is second
    assert first.flags.writeable is False, "共享数组必须只读"


def test_cache_is_bounded():
    for shape in range(1, nn._MASK_CACHE_MAX + 10):
        nn._causal_mask(shape, shape)
    assert len(nn._MASK_CACHE) <= nn._MASK_CACHE_MAX


def test_attention_forward_is_deterministic_and_uses_cache():
    np.random.seed(0)
    layer = nn.CausalSelfAttention(n_embd=8, n_head=2)
    x = Tensor(np.random.randn(2, 4, 8))
    y1 = layer.forward(x).data.copy()
    y2 = layer.forward(x).data.copy()
    assert np.allclose(y1, y2), "同一输入两次前向结果必须一致"
    assert layer.last_att.shape == (2, 2, 4, 4)
    # 第二次前向命中同一形状缓存
    assert (4, 4) in nn._MASK_CACHE
