# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""第 76 轮：torchgpt.get_batch 宽窗取样必须与旧实现逐元素等价

旧实现对每个样本切两次窗、每窗 astype(int64)（默认复制）再 torch.stack。
改为 numpy 宽窗 + 一次类型转换 + from_numpy 切视图后，判据是：同一 torch
随机种子下，采样起点与 x/y 内容都和旧实现完全一致（含 dtype / 形状 / 边界）。
"""

import pytest

torch = pytest.importorskip("torch")  # 没装 torch 就跳过本文件全部用例

import numpy as np  # noqa: E402

from torchgpt.train import get_batch  # noqa: E402


def reference_get_batch(data, batch_size, block_size, device):
    """第 76 轮之前的实现（仅用于等价性对照）"""
    ix = torch.randint(len(data) - block_size - 1, (batch_size,))
    x = torch.stack([torch.from_numpy(data[i : i + block_size].astype(np.int64)) for i in ix])
    y = torch.stack([torch.from_numpy(data[i + 1 : i + 1 + block_size].astype(np.int64)) for i in ix])
    return x.to(device), y.to(device)


def test_matches_previous_implementation_for_same_seed():
    data = np.arange(128, dtype=np.int64)
    for seed in (0, 1, 7):
        torch.manual_seed(seed)
        got = get_batch(data, batch_size=8, block_size=16, device="cpu")
        torch.manual_seed(seed)
        want = reference_get_batch(data, batch_size=8, block_size=16, device="cpu")

        assert got[0].shape == want[0].shape == (8, 16)
        assert got[0].dtype == want[0].dtype == torch.long
        assert got[1].dtype == want[1].dtype == torch.long
        # 下游模型用 view() 改形状，输入必须连续（旧实现 stack 出来的就是连续的）
        assert got[0].is_contiguous() and got[1].is_contiguous()
        assert torch.equal(got[0], want[0])
        assert torch.equal(got[1], want[1])


def test_target_is_input_shifted_by_one_and_inside_corpus():
    data = np.arange(64, dtype=np.int64)
    for _ in range(20):  # 随机采样，多跑几次覆盖起点边界
        x, y = get_batch(data, batch_size=8, block_size=8, device="cpu")
        assert int(x.min()) >= 0 and int(y.max()) <= len(data) - 1
        assert torch.equal(y, x + 1)


def test_accepts_non_int64_token_stream():
    """旧实现无条件 astype(int64)；宽窗路径也必须把 uint16 等输入转成 long"""
    data = np.arange(100, dtype=np.uint16)
    x, y = get_batch(data, batch_size=4, block_size=8, device="cpu")
    assert x.dtype == torch.long and y.dtype == torch.long
    assert torch.equal(y, x + 1)
