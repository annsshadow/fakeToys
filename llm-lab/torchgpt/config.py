# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""torchgpt.config —— 模型与训练超参配置。

torchgpt 是"实战轨"：用 PyTorch 复刻 nanoGPT 级别的 GPT，演示工业界真正会用到
的东西（AMP 混合精度、梯度累积、梯度裁剪、checkpoint、LoRA、DPO）。它需要
``torch``（见 llm-lab/requirements-torch.txt）；本仓库若未装 torch，相关测试会
自动跳过，而纯 NumPy 的 ``minigrad`` 轨仍可运行。
"""

from __future__ import annotations

from dataclasses import dataclass


@dataclass
class GPTConfig:
    vocab_size: int = 65
    block_size: int = 128
    n_layer: int = 4
    n_head: int = 4
    n_embd: int = 128
    dropout: float = 0.0
    bias: bool = True


@dataclass
class TrainConfig:
    lr: float = 3e-4
    min_lr: float = 3e-5
    warmup_steps: int = 100
    max_steps: int = 2000
    weight_decay: float = 0.1
    beta1: float = 0.9
    beta2: float = 0.95
    grad_clip: float = 1.0
    batch_size: int = 32
    grad_accum_steps: int = 1  # 显存不够时放大它，等效于更大 batch
    eval_every: int = 200
    eval_iters: int = 50
    amp: bool = True           # 自动混合精度（float16/bf16）
    seed: int = 1337
    out_dir: str = "checkpoints"
