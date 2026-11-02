# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""第 75 轮：get_batch 向量化取窗必须与旧实现逐元素等价

旧实现每个样本一次 Python 切片 + np.stack；训练每步都调用，batch_size 越大
解释器开销越线性。改成索引矩阵后，正确性判据只有一条：同一随机种子下，
x/y 与旧实现逐元素一致（采样起点、窗口边界、目标右移一位都不能变）。
"""

import numpy as np
import pytest

from minigrad.data import get_batch


def reference_get_batch(data, batch_size, block_size, rng):
    """第 75 轮之前的实现（仅用于等价性对照）"""
    max_start = len(data) - block_size - 1
    starts = rng.randint(0, max_start, size=batch_size)
    x = np.stack([data[s : s + block_size] for s in starts])
    y = np.stack([data[s + 1 : s + 1 + block_size] for s in starts])
    return x, y


@pytest.mark.parametrize(
    "batch_size,block_size,seed",
    [(1, 1, 0), (4, 8, 1), (16, 32, 2), (128, 16, 3)],
)
def test_vectorized_matches_previous_implementation(batch_size, block_size, seed):
    data = np.arange(1000, dtype=np.int64)
    got = get_batch(data, batch_size, block_size, np.random.RandomState(seed))
    want = reference_get_batch(data, batch_size, block_size, np.random.RandomState(seed))

    assert got[0].shape == (batch_size, block_size)
    assert got[1].shape == (batch_size, block_size)
    np.testing.assert_array_equal(got[0], want[0])
    np.testing.assert_array_equal(got[1], want[1])


def test_target_stays_inside_corpus_and_shifted_by_one():
    data = np.arange(64, dtype=np.int64)
    for _ in range(20):  # 随机采样，多跑几次覆盖起点边界
        x, y = get_batch(data, batch_size=8, block_size=8)
        assert x.min() >= 0 and y.max() <= len(data) - 1
        np.testing.assert_array_equal(y, x + 1)


def test_short_corpus_still_fails_loud():
    with pytest.raises(AssertionError, match="语料太短"):
        get_batch(np.arange(9), batch_size=2, block_size=8)


def test_accepts_plain_python_list():
    """旧实现能接受 list（切片 + np.stack）；向量化后仍须兼容"""
    x, y = get_batch(list(range(100)), 2, 4, np.random.RandomState(5))
    assert x.shape == (2, 4)
    np.testing.assert_array_equal(y, x + 1)
