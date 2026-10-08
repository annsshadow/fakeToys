# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""torchgpt.lora —— LoRA 低秩微调。

LoRA（Low-Rank Adaptation）冻结原始权重 W，只训练两个小矩阵 A(r×in)、B(out×r)，
用 ``W x + (BA) x * (alpha/r)`` 近似权重更新。可训练参数通常只有全参微调的
0.1%~1%，是单卡微调大模型的主流做法。QLoRA 则在此基础上把 base 权重量化到 4bit，
进一步省显存（见 docs/07-微调-SFT-LoRA-QLoRA.md）。
"""

from __future__ import annotations

import torch
import torch.nn as nn


class LoRALinear(nn.Module):
    def __init__(self, base: nn.Linear, r: int = 8, alpha: int = 16, dropout: float = 0.0):
        super().__init__()
        self.base = base
        for p in self.base.parameters():
            p.requires_grad = False  # 冻结原始权重
        self.r = r
        self.scaling = alpha / r
        self.lora_a = nn.Parameter(torch.randn(r, base.in_features) * 0.01)
        self.lora_b = nn.Parameter(torch.zeros(base.out_features, r))  # 初始为 0，训练起点等于原模型
        self.dropout = nn.Dropout(dropout)

    def forward(self, x):
        delta = (self.dropout(x) @ self.lora_a.t()) @ self.lora_b.t()
        return self.base(x) + delta * self.scaling


def apply_lora(model: nn.Module, r: int = 8, alpha: int = 16, targets=("c_attn", "c_proj")):
    """把模型中名字匹配 ``targets`` 的 ``nn.Linear`` 原地替换为 ``LoRALinear``。"""
    for name, module in model.named_modules():
        for child_name, child in list(module.named_children()):
            if isinstance(child, nn.Linear) and child_name in targets:
                setattr(module, child_name, LoRALinear(child, r=r, alpha=alpha))
    return model


def mark_only_lora_trainable(model: nn.Module):
    for name, p in model.named_parameters():
        p.requires_grad = "lora_" in name


def trainable_parameter_ratio(model: nn.Module):
    trainable = sum(p.numel() for p in model.parameters() if p.requires_grad)
    total = sum(p.numel() for p in model.parameters())
    return trainable, total, trainable / total
