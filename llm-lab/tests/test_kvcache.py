# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""KV cache 教学实现的正确性验证。

核心不变量：**增量（带 cache）前向与整段（无 cache）前向，逐 logit 完全一致**。
位置嵌入按绝对位置接续、注意力 mask 的下三角对齐，任何一处错了都会在这里现形。
"""

import numpy as np
import pytest

from minigrad.nn import GPT, GPTConfig
from minigrad.sample import generate, generate_cached


@pytest.fixture(autouse=True)
def _seed():
    np.random.seed(7)


def _tiny_model():
    cfg = GPTConfig(vocab_size=11, block_size=20, n_layer=2, n_head=2, n_embd=16)
    return GPT(cfg)


def test_cached_forward_matches_full_forward_chunked():
    model = _tiny_model()
    tokens = np.random.randint(0, 11, size=(1, 12))
    full = model.forward(tokens).data  # (1, 12, V)

    caches = [{} for _ in model.blocks]
    out1 = model.forward(tokens[:, :5], caches=caches).data
    out2 = model.forward(tokens[:, 5:], caches=caches).data  # 一次喂 7 个新 token
    incremental = np.concatenate([out1, out2], axis=1)
    assert np.allclose(full, incremental, atol=1e-10)


def test_cached_forward_matches_full_forward_token_by_token():
    model = _tiny_model()
    tokens = np.random.randint(0, 11, size=(1, 10))
    full = model.forward(tokens).data

    caches = [{} for _ in model.blocks]
    outs = [model.forward(tokens[:, :1], caches=caches).data]
    for t in range(1, 10):
        outs.append(model.forward(tokens[:, t : t + 1], caches=caches).data)
    incremental = np.concatenate(outs, axis=1)
    assert np.allclose(full, incremental, atol=1e-10)


def test_generate_cached_equals_naive_greedy():
    """贪心采样下，两种生成方式必须产出逐 token 相同的序列。"""
    model = _tiny_model()
    prompt = np.random.randint(0, 11, size=4)
    naive = generate(model, prompt, max_new_tokens=8, temperature=0.0)
    cached = generate_cached(model, prompt, max_new_tokens=8, temperature=0.0)
    assert np.array_equal(naive, cached)


def test_generate_cached_rejects_overflow():
    model = _tiny_model()  # block_size=20
    prompt = np.zeros(15, dtype=int)
    with pytest.raises(AssertionError):
        generate_cached(model, prompt, max_new_tokens=10, temperature=1.0)


def test_cached_prefill_touches_every_layer_cache():
    model = _tiny_model()
    caches = [{} for _ in model.blocks]
    model.forward(np.random.randint(0, 11, size=(1, 6)), caches=caches)
    for c in caches:
        assert c["k"].shape == (1, 2, 6, 8)  # (B, H, T_past, head_dim)
        assert c["v"].shape == (1, 2, 6, 8)
