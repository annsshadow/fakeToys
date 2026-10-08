# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""minigrad.sample —— 自回归文本生成（两种实现互为印证）。

* ``generate``：朴素做法，每步把**整段**序列重新喂给模型，只取最后一个位置的
  logits。简单直观，但同一段前缀被反复计算，总开销 O(T^3)。
* ``generate_cached``：KV cache 版，prompt 整段算一次并把每层的 K/V 缓存下来，
  之后每步只喂**一个**新 token，开销降到 O(T^2)。这是所有推理引擎的基本功，
  原理见 docs/10-推理优化.md。

两种实现产出数学上完全相同的分布——``tests/test_kvcache.py`` 会逐 logit 验证。
"""

from __future__ import annotations

import numpy as np


def softmax_np(logits):
    logits = logits - logits.max()
    e = np.exp(logits)
    return e / e.sum()


def _pick_next(last_logits, temperature, top_k, rng):
    """从一维 logits 里采出下一个 token id。temperature<=0 为贪心 argmax。"""
    if temperature <= 0:
        return int(np.argmax(last_logits))
    last = last_logits / max(temperature, 1e-6)
    if top_k is not None:
        kth = np.sort(last)[-top_k]
        last = np.where(last < kth, -np.inf, last)
    probs = softmax_np(last)
    return int(rng.choice(len(probs), p=probs))


def generate(model, idx, max_new_tokens, temperature=1.0, top_k=None, rng=None):
    """朴素生成：每步重算整段上下文。idx: 起始 token 列表/数组 (T,)。"""
    rng = rng or np.random
    block_size = model.cfg.block_size
    ids = list(np.asarray(idx).reshape(-1))
    for _ in range(max_new_tokens):
        context = ids[-block_size:]
        logits = model.forward(np.array([context]))  # (1, T, V)
        ids.append(_pick_next(logits.data[0, -1], temperature, top_k, rng))
    return np.array(ids)


def generate_cached(model, idx, max_new_tokens, temperature=1.0, top_k=None, rng=None):
    """KV cache 生成：prompt 前向一次，之后每步只前向一个 token。

    要求 ``len(prompt) + max_new_tokens <= block_size``（位置编码总数上限）。
    """
    rng = rng or np.random
    block_size = model.cfg.block_size
    ids = list(np.asarray(idx).reshape(-1))
    assert len(ids) + max_new_tokens <= block_size, (
        "generate_cached 要求 prompt+生成长度不超过 block_size；"
        "更长文本请用朴素 generate（它每步滑窗）"
    )
    caches = [{} for _ in model.blocks]
    logits = model.forward(np.array([ids]), caches=caches)  # prefill
    for step in range(max_new_tokens):
        if step == 0:
            last_logits = logits.data[0, -1]
        else:
            logits = model.forward(np.array([[ids[-1]]]), caches=caches)
            last_logits = logits.data[0, -1]
        ids.append(_pick_next(last_logits, temperature, top_k, rng))
    return np.array(ids)
