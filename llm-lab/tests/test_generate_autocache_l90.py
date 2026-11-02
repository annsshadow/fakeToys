# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""第 90 轮：``generate`` 在窗口内自动复用 KV cache。

朴素 ``generate`` 每步都把整段上下文重算一遍；``generate_cached`` 早已实现
「prefill 一次 + 每步只喂 1 token」，但调用方仍默认走朴素版。本轮让
``generate`` 在 ``len(prompt)+max_new_tokens <= block_size`` 时直接委托缓存版，
并保证：

* 同一随机源下逐 token 输出与朴素版完全一致（分布等价）；
* 真的走缓存（前向输入长度必须是 ``[prompt_len, 1, 1, ...]``）；
* 超出窗口时仍回退朴素滑窗（缓存不可复用，行为等价）；
* 窗口内的实际耗时更低。
"""

import time

import numpy as np
import pytest

from minigrad.nn import GPT, GPTConfig
from minigrad.sample import generate


@pytest.fixture(autouse=True)
def _seed():
    np.random.seed(11)


def _tiny_model(block_size: int = 20):
    cfg = GPTConfig(vocab_size=13, block_size=block_size, n_layer=2, n_head=2, n_embd=16)
    return GPT(cfg)


def _naive_generate(model, idx, max_new_tokens, temperature=1.0, top_k=None, rng=None):
    """第 90 轮前的朴素实现，作为逐 token 等价的参照物。"""
    rng = rng or np.random
    block_size = model.cfg.block_size
    ids = list(np.asarray(idx).reshape(-1))
    for _ in range(max_new_tokens):
        context = ids[-block_size:]
        logits = model.forward(np.array([context]))
        last = logits.data[0, -1]
        if temperature <= 0:
            nxt = int(np.argmax(last))
        else:
            scaled = last / max(temperature, 1e-6)
            if top_k is not None:
                kth = np.sort(scaled)[-top_k]
                scaled = np.where(scaled < kth, -np.inf, scaled)
            e = np.exp(scaled - scaled.max())
            nxt = int(rng.choice(len(e), p=e / e.sum()))
        ids.append(nxt)
    return np.array(ids)


def _spy_forward_lengths(model):
    """记录每次前向的输入 token 数，用来证明实际走了缓存路径。"""
    lengths: list[int] = []
    real_forward = model.forward

    def spy(idx, caches=None):
        lengths.append(int(np.asarray(idx).shape[1]))
        return real_forward(idx, caches=caches)

    model.forward = spy  # 实例属性遮蔽类方法
    return lengths


def test_delegates_to_cached_within_window():
    model = _tiny_model()
    prompt = np.array([1, 2, 3, 4])
    lengths = _spy_forward_lengths(model)
    out = generate(model, prompt, max_new_tokens=8, temperature=0.0)
    # 缓存路径：prefill 整段 prompt 一次，之后每步只喂 1 个 token
    # （首个新 token 用 prefill 的最后一个 logits，故前向次数为 1 + (n-1)）
    assert lengths == [4] + [1] * 7, f"未走 KV cache，前向输入长度为 {lengths}"
    assert len(out) == 4 + 8


def test_within_window_matches_naive_sampling():
    model = _tiny_model()
    prompt = np.array([5, 6, 7, 8])
    # 采样路径每步消耗随机数，必须给两条路径同一随机流（而非共享全局状态）
    ref = _naive_generate(
        model, prompt, max_new_tokens=10, temperature=0.8, top_k=5,
        rng=np.random.default_rng(2026),
    )
    got = generate(
        model, prompt, max_new_tokens=10, temperature=0.8, top_k=5,
        rng=np.random.default_rng(2026),
    )
    assert np.array_equal(ref, got), "窗口内缓存版必须与朴素版逐 token 相同"


def test_falls_back_to_naive_beyond_window():
    model = _tiny_model(block_size=20)
    prompt = np.array([2, 3, 4])
    lengths = _spy_forward_lengths(model)
    # 3 + 20 = 23 > 20 ⇒ 必须走朴素滑窗（每次前向的输入长度随上下文增长，最终钉在 20）
    out = generate(model, prompt, max_new_tokens=20, temperature=0.0)
    assert lengths[0] == 3
    assert lengths[-1] == 20, f"滑窗路径最后一次前向应喂满窗口，实际 {lengths[-1]}"
    assert lengths == sorted(lengths), "朴素路径上下文只增长不缩短"

    ref = _naive_generate(model, prompt, max_new_tokens=20, temperature=0.0)
    assert np.array_equal(ref, out), "回退路径行为必须与旧实现一致"


def test_benchmark_cached_faster_within_window():
    model = _tiny_model(block_size=64)
    prompt = np.array([1] * 16)
    n_new = 48  # 16 + 48 = 64，恰好用满窗口

    def timed(fn):
        best = float("inf")
        for _ in range(3):
            t0 = time.perf_counter()
            fn()
            best = min(best, time.perf_counter() - t0)
        return best

    naive = timed(lambda: _naive_generate(model, prompt, n_new, temperature=0.0))
    cached = timed(lambda: generate(model, prompt, n_new, temperature=0.0))
    assert cached < naive, f"窗口内缓存版未变快：naive={naive:.4f}s cached={cached:.4f}s"
