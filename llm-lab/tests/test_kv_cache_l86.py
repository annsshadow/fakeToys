# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""第 86 轮：torchgpt.generate 增量 KV cache 必须与整窗重算严格等价。

旧实现每生成一个 token 都把 ``idx[:, -block_size:]`` 整窗前向一遍（O(n²)）；
新实现 prefill 一次后每步只喂最后 1 个 token，各注意力层复用已算好的 K/V。
判据是「逐元素等价」而不是「看起来差不多」：同一组权重下，增量路径每一步的
末位 logits 必须与整窗重算一致；同一 torch 随机种子下生成序列必须逐 token
相同（含窗口滑出 block_size 后的重置路径）。
"""

import time

import torch
import torch.nn.functional as F

from torchgpt.config import GPTConfig
from torchgpt.model import GPT


def _tiny(block_size=16, n_layer=2, n_head=2, n_embd=32, vocab=13):
    torch.manual_seed(1234)
    cfg = GPTConfig(
        vocab_size=vocab,
        block_size=block_size,
        n_layer=n_layer,
        n_head=n_head,
        n_embd=n_embd,
    )
    model = GPT(cfg)
    model.eval()
    return model


def reference_generate(model, idx, max_new_tokens, temperature=1.0, top_k=None):
    """第 86 轮之前的实现（仅用于等价性对照）。"""
    for _ in range(max_new_tokens):
        idx_cond = idx[:, -model.cfg.block_size :]
        logits, _ = model(idx_cond)
        logits = logits[:, -1, :] / max(temperature, 1e-6)
        if top_k is not None:
            v, _ = torch.topk(logits, min(top_k, logits.size(-1)))
            logits[logits < v[:, [-1]]] = -float("inf")
        probs = F.softmax(logits, dim=-1)
        idx_next = torch.multinomial(probs, num_samples=1)
        idx = torch.cat((idx, idx_next), dim=1)
    return idx


def _force_non_flash(model):
    """把每个注意力层切到非 SDPA 分支（复用注册的因果 mask buffer）。"""
    for block in model.transformer.h:
        attn = block.attn
        attn.flash = False
        if not hasattr(attn, "mask"):
            attn.register_buffer(
                "mask",
                torch.tril(torch.ones(model.cfg.block_size, model.cfg.block_size)).view(
                    1, 1, model.cfg.block_size, model.cfg.block_size
                ),
            )


def test_incremental_logits_match_full_recompute():
    model = _tiny(block_size=16)
    seq = torch.arange(16).remainder(13).reshape(1, 16)
    caches = [{} for _ in range(model.cfg.n_layer)]

    logits_full, _ = model(seq[:, :5])
    logits_prefill, _ = model(seq[:, :5], kv_caches=caches, pos_offset=0)
    assert torch.allclose(logits_prefill, logits_full, atol=1e-6)

    for i in range(5, 16):
        step, _ = model(seq[:, i : i + 1], kv_caches=caches, pos_offset=i)
        full, _ = model(seq[:, : i + 1])
        assert step.shape == (1, 1, 13)
        assert torch.allclose(step[:, -1], full[:, -1], atol=1e-6)
        # 缓存长度随步进增长，且每层都持有自己的 K/V
        assert all(c["k"].size(2) == i + 1 for c in caches)


def test_non_flash_attention_path_supports_incremental_decode():
    model = _tiny(block_size=8)
    _force_non_flash(model)
    seq = torch.arange(7).remainder(13).reshape(1, 7)
    caches = [{} for _ in range(model.cfg.n_layer)]

    logits_full, _ = model(seq)
    logits_prefill, _ = model(seq, kv_caches=caches, pos_offset=0)
    assert torch.allclose(logits_prefill, logits_full, atol=1e-6)

    # 单 token 增量走 mask 切片分支，必须放行全部历史 key
    extra = torch.tensor([[5]])
    step, _ = model(extra, kv_caches=caches, pos_offset=7)
    full, _ = model(torch.cat((seq, extra), dim=1))
    assert torch.allclose(step[:, -1], full[:, -1], atol=1e-6)


def test_generate_matches_full_recompute_reference():
    model = _tiny(block_size=8)
    prompt = torch.randint(0, 13, (1, 5))
    for kwargs in ({"temperature": 0.7, "top_k": None}, {"temperature": 1.0, "top_k": 5}):
        torch.manual_seed(99)
        got = model.generate(prompt.clone(), max_new_tokens=20, **kwargs)
        torch.manual_seed(99)
        want = reference_generate(model, prompt.clone(), max_new_tokens=20, **kwargs)
        # 20 > block_size(8)，覆盖「窗口滑出后重置缓存整窗重算」分支
        assert torch.equal(got, want)


def test_generate_matches_reference_without_flash():
    model = _tiny(block_size=8)
    _force_non_flash(model)
    prompt = torch.randint(0, 13, (1, 5))
    torch.manual_seed(42)
    got = model.generate(prompt.clone(), max_new_tokens=20, temperature=1.0, top_k=None)
    torch.manual_seed(42)
    want = reference_generate(model, prompt.clone(), 20, temperature=1.0, top_k=None)
    assert torch.equal(got, want)


def test_incremental_generation_is_faster_on_cpu():
    model = _tiny(block_size=64, n_layer=2, n_head=2, n_embd=32)
    prompt = torch.randint(0, 13, (1, 32))

    def timed(fn, repeats=3):
        # 预热，避开首次调用的算子选择/缓存开销
        fn()
        samples = []
        for _ in range(repeats):
            start = time.perf_counter()
            fn()
            samples.append(time.perf_counter() - start)
        return sorted(samples)[len(samples) // 2]

    torch.manual_seed(7)
    old = timed(lambda: reference_generate(model, prompt.clone(), 64, temperature=0.8, top_k=10))
    torch.manual_seed(7)
    new = timed(lambda: model.generate(prompt.clone(), 64, temperature=0.8, top_k=10))
    assert new < old, f"增量缓存没有更快：new={new:.4f}s old={old:.4f}s"
