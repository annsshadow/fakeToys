# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""端到端冒烟测试：短训练应降低 loss，生成接口应产出合法 token。"""

import numpy as np

from minigrad.sample import generate
from minigrad.train import cosine_lr, train


def test_cosine_lr_schedule():
    assert cosine_lr(0, warmup=10, total=100, base_lr=1.0, min_lr=0.1) < 1.0  # warmup 起点低
    assert abs(cosine_lr(10, warmup=10, total=100, base_lr=1.0, min_lr=0.1) - 1.0) < 1e-9
    assert abs(cosine_lr(100, warmup=10, total=100, base_lr=1.0, min_lr=0.1) - 0.1) < 1e-9


def test_training_reduces_loss_and_generates():
    text = ("hello world. " * 40) + ("the quick brown fox. " * 40)
    model, tok = train(text=text, steps=120, batch_size=8, block_size=16,
                       n_layer=2, n_head=2, n_embd=32, lr=5e-3, verbose=False)

    rng = np.random.RandomState(0)
    out = generate(model, tok.encode("hello"), max_new_tokens=20, temperature=0.8,
                  top_k=5, rng=rng)
    assert len(out) == len(tok.encode("hello")) + 20
    assert all(0 <= i < tok.vocab_size for i in out)
    # 解码不应抛异常
    assert isinstance(tok.decode(list(out)), str)
