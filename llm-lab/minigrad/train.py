# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""minigrad.train —— 迷你 GPT 的完整训练循环。

直接运行即可在内置的小语料上训练一个字符级 GPT 并生成文本：

    python -m minigrad.train

它串起了前面所有模块：分词 → 切批 → 前向求 loss → 反向 → AdamW 更新，
并演示带 warmup 的余弦学习率调度与周期性验证。参数都很小，CPU 几十秒即可见
loss 明显下降，便于观察"训练到底在干什么"。
"""

from __future__ import annotations

import math
import os

import numpy as np

from .data import get_batch, train_val_split
from .nn import GPT, GPTConfig
from .optim import AdamW
from .sample import generate
from .tokenizer import CharTokenizer

DEFAULT_CORPUS = os.path.join(os.path.dirname(__file__), "..", "data", "tiny_corpus.txt")


def cosine_lr(step, warmup, total, base_lr, min_lr):
    """带 warmup 的余弦退火：先线性升温，再余弦降到 min_lr。"""
    if step < warmup:
        return base_lr * (step + 1) / warmup
    if step >= total:
        return min_lr
    ratio = (step - warmup) / max(1, total - warmup)
    return min_lr + 0.5 * (base_lr - min_lr) * (1 + math.cos(math.pi * ratio))


def estimate_loss(model, data, batch_size, block_size, iters, rng):
    losses = []
    for _ in range(iters):
        x, y = get_batch(data, batch_size, block_size, rng)
        losses.append(model.loss(x, y).data.item())
    return float(np.mean(losses))


def train(
    text=None,
    steps=400,
    batch_size=16,
    block_size=32,
    n_layer=3,
    n_head=4,
    n_embd=64,
    lr=3e-3,
    seed=1337,
    eval_every=100,
    tokenizer="char",
    bpe_vocab=400,
    out_ckpt=None,
    return_stats=False,
    verbose=True,
):
    """训练一个 GPT，返回 ``(model, tokenizer)``（``return_stats=True`` 时附带统计字典）。

    ``tokenizer`` 可选 ``"char"``（默认，每个字符一个 token）或 ``"bpe"``（先用
    ``tokenizer.BPETokenizer`` 在语料上学 ``bpe_vocab`` 大小的词表再训练）。
    ``out_ckpt`` 给定路径时，训练结束把模型与词表保存为 checkpoint
    （见 ``checkpoint.py``），可用 ``checkpoint.load`` 恢复。
    """
    rng = np.random.RandomState(seed)
    np.random.seed(seed)

    if text is None:
        with open(os.path.normpath(DEFAULT_CORPUS), encoding="utf-8") as f:
            text = f.read()

    if tokenizer == "char":
        tok = CharTokenizer(text)
    elif tokenizer == "bpe":
        from .tokenizer import BPETokenizer

        tok = BPETokenizer().train(text, vocab_size=bpe_vocab)
    else:
        raise ValueError(f"未知 tokenizer: {tokenizer!r}（可选 'char' / 'bpe'）")
    ids = tok.encode(text)
    train_ids, val_ids = train_val_split(ids, val_ratio=0.1)

    cfg = GPTConfig(
        vocab_size=tok.vocab_size,
        block_size=block_size,
        n_layer=n_layer,
        n_head=n_head,
        n_embd=n_embd,
    )
    model = GPT(cfg)
    opt = AdamW(model.parameters(), lr=lr)

    final_val = float("nan")
    final_train = float("nan")
    for step in range(steps):
        opt.lr = cosine_lr(step, warmup=steps // 20 + 1, total=steps, base_lr=lr, min_lr=lr * 0.1)
        x, y = get_batch(train_ids, batch_size, block_size, rng)
        loss = model.loss(x, y)
        final_train = loss.data.item()
        opt.zero_grad()
        loss.backward()
        opt.step()

        if step % eval_every == 0 or step == steps - 1:
            final_val = estimate_loss(model, val_ids, batch_size, block_size, 5, rng)
            if verbose:
                print(f"step {step:4d} | train {loss.data.item():.4f} | val {final_val:.4f} | lr {opt.lr:.2e}")

    if out_ckpt:
        from .checkpoint import save

        save(model, out_ckpt, stoi=tok.stoi)
        if verbose:
            print(f"checkpoint 已保存: {out_ckpt}")

    if return_stats:
        return model, tok, {
            "steps": steps,
            "final_train_loss": final_train,
            "final_val_loss": final_val,
            "vocab_size": tok.vocab_size,
        }
    return model, tok


def main():
    model, tok = train()
    start = tok.encode("\n")
    out = generate(model, start, max_new_tokens=200, temperature=0.8, top_k=10)
    print("\n===== 生成样例 =====")
    print(tok.decode(list(out)))


if __name__ == "__main__":
    main()
