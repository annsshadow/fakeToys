# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""minigrad.optim —— 教学用优化器：SGD(+momentum) 与 AdamW。

AdamW 是训练 Transformer 的事实标准：Adam 的自适应学习率 + 解耦权重衰减
（weight decay 不混进梯度，而是直接作用在参数上）。
"""

from __future__ import annotations

import numpy as np


class SGD:
    def __init__(self, params, lr=1e-2, momentum=0.0):
        self.params = list(params)
        self.lr = lr
        self.momentum = momentum
        self.velocity = [np.zeros_like(p.data) for p in self.params]

    def zero_grad(self):
        for p in self.params:
            p.grad = np.zeros_like(p.data)

    def step(self):
        for i, p in enumerate(self.params):
            self.velocity[i] = self.momentum * self.velocity[i] + p.grad
            p.data -= self.lr * self.velocity[i]


class AdamW:
    def __init__(self, params, lr=3e-4, betas=(0.9, 0.95), eps=1e-8, weight_decay=0.1):
        self.params = list(params)
        self.lr = lr
        self.b1, self.b2 = betas
        self.eps = eps
        self.wd = weight_decay
        self.m = [np.zeros_like(p.data) for p in self.params]
        self.v = [np.zeros_like(p.data) for p in self.params]
        self.t = 0

    def zero_grad(self):
        for p in self.params:
            p.grad = np.zeros_like(p.data)

    def step(self):
        self.t += 1
        for i, p in enumerate(self.params):
            g = p.grad
            self.m[i] = self.b1 * self.m[i] + (1 - self.b1) * g
            self.v[i] = self.b2 * self.v[i] + (1 - self.b2) * (g * g)
            m_hat = self.m[i] / (1 - self.b1 ** self.t)
            v_hat = self.v[i] / (1 - self.b2 ** self.t)
            # 解耦权重衰减：直接作用在参数上，不进入梯度动量
            p.data -= self.lr * (m_hat / (np.sqrt(v_hat) + self.eps) + self.wd * p.data)
