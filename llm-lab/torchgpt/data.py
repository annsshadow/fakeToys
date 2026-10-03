# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""torchgpt.data —— 字符级数据准备（PyTorch 轨）。

把文本编码成一维 ``int64`` numpy 数组，并保存一个 ``meta`` 记录字符↔id 映射。
真实预训练会先把语料 tokenize 成二进制 ``.bin`` 用内存映射（memmap）流式读取，
这里为教学从简，直接驻留内存。
"""

from __future__ import annotations

import numpy as np


class CharDataset:
    def __init__(self, text: str):
        chars = sorted(set(text))
        self.stoi = {c: i for i, c in enumerate(chars)}
        self.itos = {i: c for i, c in enumerate(chars)}
        self.vocab_size = len(chars)
        ids = np.array([self.stoi[c] for c in text], dtype=np.int64)
        n_val = max(1, int(len(ids) * 0.1))
        self.train_ids = ids[:-n_val]
        self.val_ids = ids[-n_val:]

    def encode(self, s):
        return np.array([self.stoi[c] for c in s], dtype=np.int64)

    def decode(self, ids):
        return "".join(self.itos[int(i)] for i in ids)
