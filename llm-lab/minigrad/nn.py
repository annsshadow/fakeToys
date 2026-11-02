# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""minigrad.nn —— 建立在 ``engine.Tensor`` 之上的神经网络层与一个完整 GPT。

这里实现的是一个 decoder-only Transformer（GPT）：token/位置嵌入 → N 个
``Block``（因果自注意力 + MLP，均为 Pre-LN 残差）→ 终层 LayerNorm → 输出头。
所有层只依赖 ``engine`` 的基础算子，因此反向传播全自动。
"""

from __future__ import annotations

from collections import OrderedDict
from dataclasses import dataclass

import numpy as np

from . import engine
from .engine import Tensor

# 第 84 轮：因果 mask 缓存。mask 只取决于 (T, T_total) 两个形状参数，
# 旧实现每层、每次前向都重新 `np.ones((T,T_total))` + `np.triu` + 乘法，
# 训练里同一形状会重复上千次。缓存只存**只读 ndarray**，每次前向仍新建
# `Tensor` 包装（`.grad` 按调用隔离，不会跨步累加）。
_MASK_CACHE: "OrderedDict[tuple[int, int], np.ndarray]" = OrderedDict()
_MASK_CACHE_MAX = 32


def _causal_mask(T: int, T_total: int) -> np.ndarray:
    """返回 (T, T_total) 的上三角 -1e9 mask（同形状复用同一只只读数组）。"""
    key = (T, T_total)
    cached = _MASK_CACHE.get(key)
    if cached is not None:
        _MASK_CACHE.move_to_end(key)
        return cached
    mask = np.triu(np.ones((T, T_total), dtype=np.float64),
                   k=T_total - T + 1) * -1e9
    mask.flags.writeable = False
    _MASK_CACHE[key] = mask
    if len(_MASK_CACHE) > _MASK_CACHE_MAX:
        _MASK_CACHE.popitem(last=False)
    return mask


class Module:
    """最小模块基类：自动收集 ``__dict__`` 里的参数张量和子模块。"""

    def parameters(self):
        params = []
        for value in self.__dict__.values():
            if isinstance(value, Tensor):
                params.append(value)
            elif isinstance(value, Module):
                params.extend(value.parameters())
            elif isinstance(value, (list, tuple)):
                for item in value:
                    if isinstance(item, Module):
                        params.extend(item.parameters())
                    elif isinstance(item, Tensor):
                        params.append(item)
        return params

    def __call__(self, *args, **kwargs):
        return self.forward(*args, **kwargs)


class Linear(Module):
    """全连接层 ``y = x @ W + b``，权重用 1/sqrt(fan_in) 缩放初始化。"""

    def __init__(self, in_features, out_features, bias=True):
        self.w = Tensor(np.random.randn(in_features, out_features) / np.sqrt(in_features))
        self.b = Tensor(np.zeros(out_features)) if bias else None

    def forward(self, x):
        out = x @ self.w
        if self.b is not None:
            out = out + self.b
        return out


class Embedding(Module):
    """查表嵌入，权重小随机初始化（std=0.02，GPT-2 惯例）。"""

    def __init__(self, num_embeddings, dim):
        self.weight = Tensor(np.random.randn(num_embeddings, dim) * 0.02)

    def forward(self, idx):
        return engine.embedding(self.weight, idx)


class LayerNorm(Module):
    def __init__(self, dim, eps=1e-5):
        self.gamma = Tensor(np.ones(dim))
        self.beta = Tensor(np.zeros(dim))
        self.eps = eps

    def forward(self, x):
        return engine.layernorm(x, self.gamma, self.beta, self.eps)


class CausalSelfAttention(Module):
    """多头因果自注意力。

    用三个独立的 ``Linear`` 产生 Q/K/V（比把一个大矩阵切片更易读），
    reshape 成多头后做缩放点积注意力，并用上三角 mask 保证"只能看过去"。

    ``cache`` 用于增量解码（KV cache，见 ``sample.generate_cached``）：传入一个
    ``{"k": ..., "v": ...}`` 字典（可为空），本层会把新算出的 K/V 追加进去，
    并让新 token 只 attend 到"全部过去 + 自己"。前向会把注意力权重存到
    ``self.last_att``，供 ``visualize`` 模块读取。
    """

    def __init__(self, n_embd, n_head):
        assert n_embd % n_head == 0, "n_embd 必须能被 n_head 整除"
        self.n_head = n_head
        self.head_dim = n_embd // n_head
        self.q = Linear(n_embd, n_embd)
        self.k = Linear(n_embd, n_embd)
        self.v = Linear(n_embd, n_embd)
        self.proj = Linear(n_embd, n_embd)
        self.last_att = None

    def forward(self, x, cache=None):
        B, T, C = x.shape
        H, d = self.n_head, self.head_dim

        def split_heads(t):
            # (B, T, C) -> (B, H, T, d)
            return t.reshape(B, T, H, d).transpose((0, 2, 1, 3))

        q, k, v = split_heads(self.q(x)), split_heads(self.k(x)), split_heads(self.v(x))

        if cache is not None:
            # 第 99 轮：旧实现每步 `np.concatenate([cache, new], axis=2)` ——
            # 生成长度 T 时第 t 步要复制 t×d 个元素，整个解码在 KV 上做 O(T²) 拷贝。
            # 改为按需翻倍的预分配缓冲：`k_buf`/`v_buf` 存容量，`k`/`v` 仍是对
            # 「已用长度」的视图（外部可见形状不变），每步只写入新 token 的 K/V。
            past = 0 if cache.get("k") is None else cache["k"].shape[2]
            need = past + T
            for name, new in (("k", k), ("v", v)):
                buf = cache.get(name + "_buf")
                if buf is None or buf.shape[2] < need:
                    cap = max(need, 2 * past)
                    grown = np.empty((B, H, cap, d), dtype=new.data.dtype)
                    if past:
                        grown[:, :, :past] = cache[name]
                    cache[name + "_buf"] = grown
                    buf = grown
                buf[:, :, past:need] = new.data
                cache[name] = buf[:, :, :need]
            k = Tensor(cache["k"])
            v = Tensor(cache["v"])

        T_total = k.shape[2]
        # 查询 i（绝对位置 T_past+i）允许看到键 j <= T_past+i。统一写成
        # 上三角 mask：禁掉 j - i >= T_total - T + 1 的位置；无 cache 时
        # T_total == T，恰好退化为普通的 k=1 因果 mask。
        mask = _causal_mask(T, T_total)
        att = engine.softmax((q @ k.transpose((0, 1, 3, 2))) * (1.0 / np.sqrt(d)) + Tensor(mask), axis=-1)
        self.last_att = att.data  # (B, H, T, T_total)，可视化用
        y = att @ v  # (B,H,T,d)
        y = y.transpose((0, 2, 1, 3)).reshape(B, T, C)  # 合并多头
        return self.proj(y)


class MLP(Module):
    """逐位置前馈网络，隐藏层扩张 4 倍（Transformer 标配）。"""

    def __init__(self, n_embd):
        self.fc = Linear(n_embd, 4 * n_embd)
        self.proj = Linear(4 * n_embd, n_embd)

    def forward(self, x):
        return self.proj(engine.gelu(self.fc(x)))


class Block(Module):
    """Pre-LN Transformer 块：先归一化再进子层，两处残差连接。"""

    def __init__(self, n_embd, n_head):
        self.ln1 = LayerNorm(n_embd)
        self.attn = CausalSelfAttention(n_embd, n_head)
        self.ln2 = LayerNorm(n_embd)
        self.mlp = MLP(n_embd)

    def forward(self, x, cache=None):
        x = x + self.attn(self.ln1(x), cache)
        x = x + self.mlp(self.ln2(x))
        return x


@dataclass
class GPTConfig:
    vocab_size: int
    block_size: int = 64
    n_layer: int = 3
    n_head: int = 4
    n_embd: int = 64


class GPT(Module):
    def __init__(self, cfg: GPTConfig):
        self.cfg = cfg
        self.wte = Embedding(cfg.vocab_size, cfg.n_embd)
        self.wpe = Embedding(cfg.block_size, cfg.n_embd)
        self.blocks = [Block(cfg.n_embd, cfg.n_head) for _ in range(cfg.n_layer)]
        self.ln_f = LayerNorm(cfg.n_embd)
        self.head = Linear(cfg.n_embd, cfg.vocab_size, bias=False)

    def forward(self, idx, caches=None):
        """idx: 整型数组 (B, T)，返回 logits (B, T, vocab)。

        ``caches`` 是与 ``self.blocks`` 等长的字典列表（元素可为空字典），
        传入后做增量前向：新 token 的位置从已有 KV 长度接续算起，各注意力层
        把 K/V 追加进对应缓存。训练/整段前向传 None 即可，行为不变。
        """
        idx = np.asarray(idx)
        B, T = idx.shape
        T_past = caches[0]["k"].shape[2] if caches and caches[0].get("k") is not None else 0
        assert T_past + T <= self.cfg.block_size, "序列超过 block_size"
        x = self.wte(idx) + self.wpe(np.arange(T_past, T_past + T))  # token + 位置嵌入
        for i, block in enumerate(self.blocks):
            x = block(x, caches[i] if caches is not None else None)
        x = self.ln_f(x)
        return self.head(x)

    def loss(self, idx, targets):
        """交叉熵语言建模损失；targets 为右移一位的下一个 token。"""
        logits = self.forward(idx)
        B, T, V = logits.shape
        return engine.cross_entropy(
            logits.reshape(B * T, V), np.asarray(targets).reshape(-1)
        )
