# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""第 83 轮：`_topo` 改为迭代式后序 DFS。

旧实现递归遍历计算图；图深度超过 Python 递归上限（默认 1000）时
`backward()` 在拓扑排序阶段就 `RecursionError`。本用例锁两件事：

1. 深链（1500 步）必须能正常反向，且梯度正确；
2. 拓扑序语义不变：每个节点都排在它的全部操作数之后（含菱形/共享子图）。
"""

import numpy as np
import pytest

from minigrad.engine import Tensor, _topo


def _old_topo(root):
    """第 83 轮之前的递归实现（仅用于等价性对照）"""
    topo, visited = [], set()

    def build(v):
        if v not in visited:
            visited.add(v)
            for child in v._prev:
                build(child)
            topo.append(v)

    build(root)
    return topo


def test_deep_chain_backward_does_not_recurse():
    base = Tensor(0.0)
    x = base
    for _ in range(1500):
        x = x + 1.0
    loss = x.sum()
    loss.backward()
    assert float(base.grad) == pytest.approx(1.0)


def test_shared_subgraph_gradient_is_summed():
    base = Tensor(2.0)
    shared = base * 3.0
    out = shared + shared          # 同一条子表达式被两条边引用
    out.backward()
    assert float(base.grad) == pytest.approx(6.0)


def test_topological_order_respects_all_operands():
    a = Tensor(1.0)
    b = Tensor(2.0)
    c = a + b
    d = a * c
    e = c + d
    order = _topo(e)
    pos = {id(n): i for i, n in enumerate(order)}
    for node in order:
        for child in node._prev:
            assert pos[id(child)] < pos[id(node)], "操作数必须排在使用它的节点之前"


def test_visits_same_node_set_as_previous_implementation():
    a = Tensor(1.0)
    b = a * 2.0
    c = a + b
    d = b * c + a
    new = {id(n) for n in _topo(d)}
    old = {id(n) for n in _old_topo(d)}
    assert new == old
