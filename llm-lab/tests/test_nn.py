# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""网络层与 GPT 的形状、因果性、过拟合能力测试。"""

import numpy as np
import pytest

from minigrad import engine
from minigrad.nn import GPT, CausalSelfAttention, GPTConfig, Linear
from minigrad.optim import AdamW


@pytest.fixture(autouse=True)
def _seed():
    np.random.seed(0)


def test_linear_shape():
    lin = Linear(4, 7)
    y = lin(engine.Tensor(np.random.randn(2, 3, 4)))
    assert y.shape == (2, 3, 7)


def test_gpt_forward_shape():
    cfg = GPTConfig(vocab_size=11, block_size=8, n_layer=2, n_head=2, n_embd=16)
    model = GPT(cfg)
    idx = np.random.randint(0, 11, size=(3, 8))
    logits = model.forward(idx)
    assert logits.shape == (3, 8, 11)


def test_attention_is_causal():
    """位置 t 的输出不能依赖未来 token：改动序列尾部不应影响靠前位置的输出。"""
    np.random.seed(1)
    attn = CausalSelfAttention(n_embd=8, n_head=2)
    x1 = np.random.randn(1, 5, 8)
    x2 = x1.copy()
    x2[0, 4, :] += 10.0  # 只改最后一个位置
    y1 = attn(engine.Tensor(x1)).data
    y2 = attn(engine.Tensor(x2)).data
    assert np.allclose(y1[0, :4], y2[0, :4], atol=1e-9)
    assert not np.allclose(y1[0, 4], y2[0, 4])


def test_gpt_can_overfit_tiny_batch():
    """在单个固定小批上反复训练，loss 应显著下降——证明端到端可学习。"""
    cfg = GPTConfig(vocab_size=7, block_size=6, n_layer=2, n_head=2, n_embd=24)
    model = GPT(cfg)
    opt = AdamW(model.parameters(), lr=5e-3, weight_decay=0.0)
    idx = np.random.randint(0, 7, size=(4, 6))
    tgt = np.random.randint(0, 7, size=(4, 6))

    first = model.loss(idx, tgt).data.item()
    for _ in range(60):
        loss = model.loss(idx, tgt)
        opt.zero_grad()
        loss.backward()
        opt.step()
    last = model.loss(idx, tgt).data.item()
    assert last < first * 0.5, f"loss 未下降: {first:.3f} -> {last:.3f}"
