# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""minigrad.data —— 把一长串 token 切成 (输入, 目标) 批次。

语言模型的训练目标是"预测下一个 token"：给定位置 i 的输入，目标就是位置 i+1
的 token。因此一条样本是连续的 ``block_size`` 个 token 作为输入 x，右移一位作为
目标 y。``get_batch`` 随机采样若干这样的窗口拼成一个 batch。
"""

from __future__ import annotations

import numpy as np


def get_batch(data, batch_size, block_size, rng=None):
    """从一维 token 数组 ``data`` 随机采样一个批次。

    返回 ``(x, y)``，形状均为 (batch_size, block_size)，y 是 x 右移一位。
    """
    rng = rng or np.random
    max_start = len(data) - block_size - 1
    assert max_start > 0, "语料太短，装不下一个 block"
    starts = rng.randint(0, max_start, size=batch_size)
    x = np.stack([data[s : s + block_size] for s in starts])
    y = np.stack([data[s + 1 : s + 1 + block_size] for s in starts])
    return x, y


def train_val_split(ids, val_ratio=0.1):
    """按时间顺序切分训练/验证集（语言建模不打乱，保留上下文连续性）。"""
    ids = np.asarray(ids, dtype=np.int64)
    n_val = max(1, int(len(ids) * val_ratio))
    return ids[:-n_val], ids[-n_val:]
