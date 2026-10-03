# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""minigrad —— 纯 NumPy 的迷你 GPT 训练栈（本仓库任何环境都能跑）。

模块一览：
    engine     张量级自动微分引擎（autograd）
    nn         基于 engine 的网络层与 GPT 模型
    optim      SGD / AdamW 优化器
    tokenizer  字符级 / BPE 分词器
    data       批次采样
    train      训练循环（可直接 ``python -m minigrad.train`` 运行）
    sample     自回归文本生成
"""

from . import data, engine, nn, optim, tokenizer

__all__ = ["engine", "nn", "optim", "tokenizer", "data"]
