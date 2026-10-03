# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""torchgpt.eval —— 困惑度（perplexity）评估。

困惑度 = exp(平均交叉熵)，是语言模型最常用的内在指标：数值越低，模型对真实
文本"越不意外"。它只衡量预测分布的质量，不等于下游任务好坏——下游还需 MMLU、
GSM8K 等基准（见 docs/09-评估.md）。
"""

from __future__ import annotations

import math

import torch


@torch.no_grad()
def perplexity(model, data, block_size, device="cpu", stride=None, max_blocks=None):
    """在一段连续 token 上以滑动窗口计算困惑度。"""
    model.eval()
    stride = stride or block_size
    nll_sum, token_count, blocks = 0.0, 0, 0
    for start in range(0, len(data) - block_size - 1, stride):
        x = torch.from_numpy(data[start : start + block_size].astype("int64"))[None].to(device)
        y = torch.from_numpy(data[start + 1 : start + 1 + block_size].astype("int64"))[None].to(device)
        _, loss = model(x, y)
        nll_sum += loss.item() * block_size
        token_count += block_size
        blocks += 1
        if max_blocks and blocks >= max_blocks:
            break
    mean_nll = nll_sum / max(1, token_count)
    return math.exp(mean_nll)
