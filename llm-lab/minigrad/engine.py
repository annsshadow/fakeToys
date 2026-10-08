# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""minigrad.engine —— 纯 NumPy 的反向模式自动微分引擎。

教学目标：用一个张量级 ``Tensor`` 类（迷你 PyTorch）演示自动微分的全部要点：

* 每个算子在前向时记录计算图（``_prev`` / ``_backward``）；
* ``backward()`` 用拓扑排序一次性回传梯度；
* 广播（broadcasting）在反向时需要把梯度"求和折叠"回原形状（``_unbroadcast``）。

引擎刻意用 ``float64``，这样 ``tests/test_engine.py`` 里的数值梯度校验
（finite difference）能逼到 1e-6 量级，确保每个算子的反向实现都是对的。
真正的 GPT（见 ``minigrad/nn.py``）完全建立在这些算子之上。
"""

from __future__ import annotations

import numpy as np


class Tensor:
    """包装一个 ``np.ndarray``，并记录生成它的运算，用于反向传播。"""

    def __init__(self, data, _children=(), _op=""):
        self.data = np.asarray(data, dtype=np.float64)
        self.grad = np.zeros_like(self.data)
        self._backward = lambda: None
        self._prev = set(_children)
        self._op = _op

    # ---------------------------------------------------------------- 基础属性
    @property
    def shape(self):
        return self.data.shape

    @property
    def ndim(self):
        return self.data.ndim

    def __repr__(self):
        return f"Tensor(shape={self.data.shape}, op={self._op!r})"

    @staticmethod
    def _unbroadcast(grad, shape):
        """把 ``grad`` 沿广播出来的维度求和，折叠回 ``shape``。

        广播让 (C,) 的偏置能加到 (B,T,C) 上；反向时这些"复制"出来的梯度
        必须加总回原来的形状，否则梯度维度对不上。
        """
        while grad.ndim > len(shape):
            grad = grad.sum(axis=0)
        for i, dim in enumerate(shape):
            if dim == 1 and grad.shape[i] != 1:
                grad = grad.sum(axis=i, keepdims=True)
        return grad

    # ---------------------------------------------------------------- 逐元素运算
    def __add__(self, other):
        other = other if isinstance(other, Tensor) else Tensor(other)
        out = Tensor(self.data + other.data, (self, other), "+")

        def _backward():
            self.grad += self._unbroadcast(out.grad, self.data.shape)
            other.grad += self._unbroadcast(out.grad, other.data.shape)

        out._backward = _backward
        return out

    def __mul__(self, other):
        other = other if isinstance(other, Tensor) else Tensor(other)
        out = Tensor(self.data * other.data, (self, other), "*")

        def _backward():
            self.grad += self._unbroadcast(other.data * out.grad, self.data.shape)
            other.grad += self._unbroadcast(self.data * out.grad, other.data.shape)

        out._backward = _backward
        return out

    def __pow__(self, p):
        assert isinstance(p, (int, float)), "只支持常数次幂"
        out = Tensor(self.data ** p, (self,), f"**{p}")

        def _backward():
            self.grad += (p * self.data ** (p - 1)) * out.grad

        out._backward = _backward
        return out

    def __matmul__(self, other):
        out = Tensor(self.data @ other.data, (self, other), "@")

        def _backward():
            self.grad += self._unbroadcast(
                out.grad @ np.swapaxes(other.data, -1, -2), self.data.shape
            )
            other.grad += self._unbroadcast(
                np.swapaxes(self.data, -1, -2) @ out.grad, other.data.shape
            )

        out._backward = _backward
        return out

    # 由基础运算组合出来的便捷运算
    def __neg__(self):
        return self * -1.0

    def __sub__(self, other):
        other = other if isinstance(other, Tensor) else Tensor(other)
        return self + (-other)

    def __truediv__(self, other):
        other = other if isinstance(other, Tensor) else Tensor(other)
        return self * other ** -1.0

    def __radd__(self, other):
        return self + other

    def __rmul__(self, other):
        return self * other

    def __rsub__(self, other):
        return (-self) + other

    # ---------------------------------------------------------------- 规约 / 变形
    def sum(self, axis=None, keepdims=False):
        out = Tensor(self.data.sum(axis=axis, keepdims=keepdims), (self,), "sum")

        def _backward():
            g = out.grad
            if axis is not None and not keepdims:
                g = np.expand_dims(g, axis)
            self.grad += np.broadcast_to(g, self.data.shape)

        out._backward = _backward
        return out

    def mean(self, axis=None, keepdims=False):
        n = self.data.size if axis is None else self.data.shape[axis]
        return self.sum(axis=axis, keepdims=keepdims) * (1.0 / n)

    def reshape(self, *shape):
        out = Tensor(self.data.reshape(*shape), (self,), "reshape")

        def _backward():
            self.grad += out.grad.reshape(self.data.shape)

        out._backward = _backward
        return out

    def transpose(self, axes):
        out = Tensor(np.transpose(self.data, axes), (self,), "transpose")

        def _backward():
            self.grad += np.transpose(out.grad, np.argsort(axes))

        out._backward = _backward
        return out

    # ---------------------------------------------------------------- 激活函数
    def relu(self):
        out = Tensor(np.maximum(self.data, 0.0), (self,), "relu")

        def _backward():
            self.grad += (self.data > 0.0) * out.grad

        out._backward = _backward
        return out

    def tanh(self):
        t = np.tanh(self.data)
        out = Tensor(t, (self,), "tanh")

        def _backward():
            self.grad += (1.0 - t * t) * out.grad

        out._backward = _backward
        return out

    def exp(self):
        e = np.exp(self.data)
        out = Tensor(e, (self,), "exp")

        def _backward():
            self.grad += e * out.grad

        out._backward = _backward
        return out

    def log(self):
        out = Tensor(np.log(self.data), (self,), "log")

        def _backward():
            self.grad += (1.0 / self.data) * out.grad

        out._backward = _backward
        return out

    # ---------------------------------------------------------------- 反向传播
    def backward(self):
        """从本张量（通常是标量 loss）出发，反向回传梯度到所有叶子。"""
        topo = _topo(self)
        self.grad = np.ones_like(self.data)
        for node in reversed(topo):
            node._backward()


def _topo(root):
    topo, visited = [], set()

    def build(v):
        if v not in visited:
            visited.add(v)
            for child in v._prev:
                build(child)
            topo.append(v)

    build(root)
    return topo


# ====================================================================== 复合函数
# 下面这些函数全部由上面的基础算子组合而成，因此梯度"自动"正确——
# 这正是 autograd 的威力：实现一次基础算子，复杂函数的反向传播就是免费的。


def softmax(x, axis=-1):
    """数值稳定的 softmax：先减去最大值（常数，不影响梯度）再做 exp。"""
    shift = x.data.max(axis=axis, keepdims=True)
    e = (x - Tensor(shift)).exp()
    return e / e.sum(axis=axis, keepdims=True)


def gelu(x):
    """GELU 的 tanh 近似，GPT-2/NanoGPT 用的就是它。"""
    c = np.sqrt(2.0 / np.pi)
    inner = (x + x ** 3 * 0.044715) * c
    return x * (inner.tanh() + 1.0) * 0.5


def layernorm(x, gamma, beta, eps=1e-5):
    """沿最后一维做层归一化，全程用基础算子，便于观察其反向传播。"""
    mu = x.mean(axis=-1, keepdims=True)
    xc = x - mu
    var = (xc * xc).mean(axis=-1, keepdims=True)
    std = (var + eps) ** 0.5
    return xc / std * gamma + beta


def embedding(weight, idx):
    """按整数索引 ``idx`` 从 ``weight`` (V, C) 中取行；反向时做 scatter-add。"""
    idx = np.asarray(idx)
    out = Tensor(weight.data[idx], (weight,), "embedding")

    def _backward():
        np.add.at(weight.grad, idx, out.grad)

    out._backward = _backward
    return out


def cross_entropy(logits, targets):
    """交叉熵损失（logits 版）。

    手写反向：对 logits 的梯度正好是 ``softmax(logits) - onehot(target)``，
    这是深度学习里最该牢记的一条推导，所以这里不用 autograd 组合、而是直接写死，
    既高效又便于教学对照。``logits`` 形状 (N, V)，``targets`` 为长度 N 的整型索引。
    """
    data = logits.data
    shift = data - data.max(axis=-1, keepdims=True)
    e = np.exp(shift)
    probs = e / e.sum(axis=-1, keepdims=True)
    n = data.shape[0]
    rows = np.arange(n)
    loss_val = -np.log(probs[rows, targets] + 1e-12).mean()
    out = Tensor(loss_val, (logits,), "cross_entropy")

    def _backward():
        dlogits = probs.copy()
        dlogits[rows, targets] -= 1.0
        dlogits /= n
        logits.grad += dlogits * out.grad

    out._backward = _backward
    return out

