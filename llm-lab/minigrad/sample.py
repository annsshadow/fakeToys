# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""minigrad.sample —— 自回归文本生成。

从一个起始上下文出发，每一步把当前序列喂给模型，取最后一个位置的 logits，
按温度/top-k 采样出下一个 token，拼接后继续，直到生成足够长度。
"""

from __future__ import annotations

import numpy as np


def softmax_np(logits):
    logits = logits - logits.max()
    e = np.exp(logits)
    return e / e.sum()


def generate(model, idx, max_new_tokens, temperature=1.0, top_k=None, rng=None):
    """idx: 起始 token 列表/数组 (T,)。返回包含新 token 的完整序列 (T+max_new_tokens,)。"""
    rng = rng or np.random
    block_size = model.cfg.block_size
    ids = list(np.asarray(idx).reshape(-1))
    for _ in range(max_new_tokens):
        context = ids[-block_size:]
        logits = model.forward(np.array([context]))  # (1, T, V)
        last = logits.data[0, -1] / max(temperature, 1e-6)
        if top_k is not None:
            kth = np.sort(last)[-top_k]
            last = np.where(last < kth, -np.inf, last)
        probs = softmax_np(last)
        next_id = int(rng.choice(len(probs), p=probs))
        ids.append(next_id)
    return np.array(ids)
