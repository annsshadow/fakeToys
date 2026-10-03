# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""逐算子的数值梯度校验。

对每个算子，用中心差分 (f(x+h)-f(x-h))/2h 估计梯度，再与引擎反向传播算出的
解析梯度比较。能通过，说明该算子的 ``_backward`` 实现正确——这是整个
minigrad 训练栈可信的地基。
"""

import numpy as np
import pytest

from minigrad import engine
from minigrad.engine import Tensor


def numeric_grad(f, t, h=1e-5):
    g = np.zeros_like(t.data)
    it = np.nditer(t.data, flags=["multi_index"])
    while not it.finished:
        idx = it.multi_index
        orig = t.data[idx]
        t.data[idx] = orig + h
        lp = float(f().data)
        t.data[idx] = orig - h
        lm = float(f().data)
        t.data[idx] = orig
        g[idx] = (lp - lm) / (2 * h)
        it.iternext()
    return g


def check(f, inputs, tol=1e-5):
    for t in inputs:
        t.grad = np.zeros_like(t.data)
    out = f()
    out.backward()
    for t in inputs:
        ng = numeric_grad(f, t)
        assert np.allclose(t.grad, ng, atol=tol, rtol=1e-3), (
            f"grad mismatch: max |diff|={np.abs(t.grad - ng).max():.2e}"
        )


@pytest.fixture(autouse=True)
def _seed():
    np.random.seed(0)


def test_add_mul_pow_broadcast():
    a = Tensor(np.random.randn(3, 4))
    b = Tensor(np.random.randn(4))  # 广播
    check(lambda: ((a + b) * (a ** 2)).sum(), [a, b])


def test_matmul_2d_and_batched():
    a = Tensor(np.random.randn(2, 3, 4))
    w = Tensor(np.random.randn(4, 5))
    check(lambda: (a @ w).sum(), [a, w])


def test_sum_mean_axis():
    a = Tensor(np.random.randn(3, 5))
    check(lambda: a.sum(axis=1).mean(), [a])


def test_reshape_transpose():
    a = Tensor(np.random.randn(2, 3, 4))
    check(lambda: a.reshape(6, 4).transpose((1, 0)).sum(), [a])


def test_relu_tanh_exp_log():
    a = Tensor(np.abs(np.random.randn(3, 4)) + 0.1)  # 保证 log 的输入为正
    check(lambda: a.relu().tanh().sum(), [a])
    check(lambda: a.exp().log().sum(), [a])


def test_softmax():
    a = Tensor(np.random.randn(3, 5))
    coef = Tensor(np.random.randn(3, 5))  # 固定系数，否则数值梯度每次重采样会乱
    check(lambda: (engine.softmax(a, axis=-1) * coef).sum(), [a])


def test_layernorm():
    x = Tensor(np.random.randn(2, 6))
    g = Tensor(np.random.randn(6))
    b = Tensor(np.random.randn(6))
    check(lambda: engine.layernorm(x, g, b).sum(), [x, g, b])


def test_gelu():
    a = Tensor(np.random.randn(3, 4))
    check(lambda: engine.gelu(a).sum(), [a])


def test_embedding():
    w = Tensor(np.random.randn(5, 3))
    idx = np.array([[0, 2], [4, 1]])
    check(lambda: engine.embedding(w, idx).sum(), [w])


def test_cross_entropy():
    logits = Tensor(np.random.randn(4, 6))
    targets = np.array([0, 3, 5, 1])
    check(lambda: engine.cross_entropy(logits, targets), [logits], tol=1e-4)
