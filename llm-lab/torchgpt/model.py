# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""torchgpt.model —— nanoGPT 风格的 GPT（PyTorch）。

与 minigrad 的 GPT 结构一致（Pre-LN、因果自注意力、4x MLP、权重共享），
但用 ``torch.nn`` 实现，并在可用时走 ``F.scaled_dot_product_attention``
（PyTorch 2 的 FlashAttention 快路径）。对照两份实现，能把"从零手写"和
"工程实现"接起来。
"""

from __future__ import annotations

import math

import torch
import torch.nn as nn
import torch.nn.functional as F

from .config import GPTConfig


class CausalSelfAttention(nn.Module):
    def __init__(self, cfg: GPTConfig):
        super().__init__()
        assert cfg.n_embd % cfg.n_head == 0
        self.n_head = cfg.n_head
        self.n_embd = cfg.n_embd
        self.c_attn = nn.Linear(cfg.n_embd, 3 * cfg.n_embd, bias=cfg.bias)
        self.c_proj = nn.Linear(cfg.n_embd, cfg.n_embd, bias=cfg.bias)
        self.attn_dropout = nn.Dropout(cfg.dropout)
        self.resid_dropout = nn.Dropout(cfg.dropout)
        self.dropout = cfg.dropout
        self.flash = hasattr(F, "scaled_dot_product_attention")
        if not self.flash:
            self.register_buffer(
                "mask",
                torch.tril(torch.ones(cfg.block_size, cfg.block_size)).view(
                    1, 1, cfg.block_size, cfg.block_size
                ),
            )

    def forward(self, x, kv_cache=None):
        """``kv_cache``：增量解码用的 ``{"k": ..., "v": ...}`` 字典（可为空）。

        传入后本层把新算的 K/V 追加进缓存，新 token 只对「全部过去 + 自己」
        做注意力。``T_total == T`` 是整窗前向（训练/prefill），走普通因果 mask；
        否则只允许单 token 增量步进（``T == 1``），此时该 token 可以看到全部
        缓存，等价于旧实现「整窗重算后取最后一行」。
        """
        B, T, C = x.size()
        q, k, v = self.c_attn(x).split(self.n_embd, dim=2)
        q = q.view(B, T, self.n_head, C // self.n_head).transpose(1, 2)
        k = k.view(B, T, self.n_head, C // self.n_head).transpose(1, 2)
        v = v.view(B, T, self.n_head, C // self.n_head).transpose(1, 2)
        if kv_cache is not None:
            if kv_cache.get("k") is not None:
                k = torch.cat((kv_cache["k"], k), dim=2)
                v = torch.cat((kv_cache["v"], v), dim=2)
            kv_cache["k"], kv_cache["v"] = k, v
        T_total = k.size(2)
        if T_total == T:
            if self.flash:
                y = F.scaled_dot_product_attention(
                    q, k, v, dropout_p=self.dropout if self.training else 0.0, is_causal=True
                )
            else:
                att = (q @ k.transpose(-2, -1)) / math.sqrt(k.size(-1))
                att = att.masked_fill(self.mask[:, :, :T, :T] == 0, float("-inf"))
                att = self.attn_dropout(F.softmax(att, dim=-1))
                y = att @ v
        else:
            assert T == 1, "增量解码每次只喂 1 个 token"
            if self.flash:
                # 单 query 对全部 key：SDPA 的 is_causal 是按左上角对齐的，
                # 1×T_total 用它会错误地只放行第 0 个 key，必须显式关闭。
                y = F.scaled_dot_product_attention(
                    q, k, v, dropout_p=self.dropout if self.training else 0.0, is_causal=False
                )
            else:
                att = (q @ k.transpose(-2, -1)) / math.sqrt(k.size(-1))
                att = att.masked_fill(
                    self.mask[:, :, T_total - T : T_total, :T_total] == 0, float("-inf")
                )
                att = self.attn_dropout(F.softmax(att, dim=-1))
                y = att @ v
        y = y.transpose(1, 2).contiguous().view(B, T, C)
        return self.resid_dropout(self.c_proj(y))


class MLP(nn.Module):
    def __init__(self, cfg: GPTConfig):
        super().__init__()
        self.c_fc = nn.Linear(cfg.n_embd, 4 * cfg.n_embd, bias=cfg.bias)
        self.c_proj = nn.Linear(4 * cfg.n_embd, cfg.n_embd, bias=cfg.bias)
        self.dropout = nn.Dropout(cfg.dropout)

    def forward(self, x):
        return self.dropout(self.c_proj(F.gelu(self.c_fc(x))))


class Block(nn.Module):
    def __init__(self, cfg: GPTConfig):
        super().__init__()
        self.ln_1 = nn.LayerNorm(cfg.n_embd)
        self.attn = CausalSelfAttention(cfg)
        self.ln_2 = nn.LayerNorm(cfg.n_embd)
        self.mlp = MLP(cfg)

    def forward(self, x, kv_cache=None):
        x = x + self.attn(self.ln_1(x), kv_cache)
        x = x + self.mlp(self.ln_2(x))
        return x


class GPT(nn.Module):
    def __init__(self, cfg: GPTConfig):
        super().__init__()
        self.cfg = cfg
        self.transformer = nn.ModuleDict(
            dict(
                wte=nn.Embedding(cfg.vocab_size, cfg.n_embd),
                wpe=nn.Embedding(cfg.block_size, cfg.n_embd),
                drop=nn.Dropout(cfg.dropout),
                h=nn.ModuleList([Block(cfg) for _ in range(cfg.n_layer)]),
                ln_f=nn.LayerNorm(cfg.n_embd),
            )
        )
        self.lm_head = nn.Linear(cfg.n_embd, cfg.vocab_size, bias=False)
        # 权重共享：输入嵌入与输出投影同一张表（GPT-2 的做法，省参数、更稳）
        self.transformer.wte.weight = self.lm_head.weight
        self.apply(self._init_weights)

    def _init_weights(self, module):
        if isinstance(module, nn.Linear):
            nn.init.normal_(module.weight, mean=0.0, std=0.02)
            if module.bias is not None:
                nn.init.zeros_(module.bias)
        elif isinstance(module, nn.Embedding):
            nn.init.normal_(module.weight, mean=0.0, std=0.02)

    def forward(self, idx, targets=None, kv_caches=None, pos_offset=0):
        """``kv_caches``：与 ``self.transformer.h`` 等长的字典列表（增量解码用）。

        传入后各层把 K/V 追加进自己的缓存；``pos_offset`` 是 ``idx`` 首个 token
        的绝对位置（无缓存时 0，增量步进时 = 已有 token 数）。
        """
        B, T = idx.size()
        assert pos_offset + T <= self.cfg.block_size
        pos = torch.arange(pos_offset, pos_offset + T, device=idx.device)
        x = self.transformer.drop(self.transformer.wte(idx) + self.transformer.wpe(pos))
        for layer, block in enumerate(self.transformer.h):
            cache = None if kv_caches is None else kv_caches[layer]
            x = block(x, cache)
        x = self.transformer.ln_f(x)
        logits = self.lm_head(x)
        loss = None
        if targets is not None:
            loss = F.cross_entropy(
                logits.view(-1, logits.size(-1)), targets.view(-1), ignore_index=-100
            )
        return logits, loss

    @torch.no_grad()
    def generate(self, idx, max_new_tokens, temperature=1.0, top_k=None):
        # KV cache：窗口内生成时首个 token 之后每步只喂最后 1 个 token，避免
        # 每步整窗重算（旧实现 O(n²)，n = 生成长度）。窗口滑出 block_size 后，
        # 旧实现的位置编码相对当前窗口重排（每步左移一位），已缓存的 K/V 无法
        # 复用 —— 此时退回整窗重算，保证与旧实现逐 token 等价。
        caches = [{} for _ in range(len(self.transformer.h))]
        for _ in range(max_new_tokens):
            if idx.size(1) > self.cfg.block_size:
                for cache in caches:
                    cache.clear()
            if caches[0].get("k") is None:
                inp = idx[:, -self.cfg.block_size :]
                pos_offset = 0
            else:
                inp = idx[:, -1:]
                pos_offset = caches[0]["k"].size(2)
            logits, _ = self(inp, kv_caches=caches, pos_offset=pos_offset)
            # 温度下限钳制：temperature=0（贪心解码）时直接相除会得到 inf，
            # softmax 变 NaN，torch.multinomial 抛
            # "probability tensor contains either inf, nan or element < 0"。
            # minigrad 轨（minigrad/sample.py::_pick_next）一直用同样的 1e-6 钳制，
            # 这里补齐，否则两条轨的 `--temperature 0` 行为不一致。
            logits = logits[:, -1, :] / max(temperature, 1e-6)
            if top_k is not None:
                v, _ = torch.topk(logits, min(top_k, logits.size(-1)))
                logits[logits < v[:, [-1]]] = -float("inf")
            probs = F.softmax(logits, dim=-1)
            idx_next = torch.multinomial(probs, num_samples=1)
            idx = torch.cat((idx, idx_next), dim=1)
        return idx
