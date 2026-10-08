# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""torchgpt 实战轨测试。未安装 torch 时整文件自动跳过。"""

import numpy as np
import pytest

torch = pytest.importorskip("torch")  # 没装 torch 就跳过本文件全部用例


def _tiny_cfg():
    from torchgpt.config import GPTConfig

    return GPTConfig(vocab_size=13, block_size=8, n_layer=2, n_head=2, n_embd=16, dropout=0.0)


def test_forward_and_loss_shapes():
    from torchgpt.model import GPT

    model = GPT(_tiny_cfg())
    idx = torch.randint(0, 13, (3, 8))
    logits, loss = model(idx, idx)
    assert logits.shape == (3, 8, 13)
    assert loss.ndim == 0 and loss.item() > 0


def test_generate_appends_tokens():
    from torchgpt.model import GPT

    model = GPT(_tiny_cfg())
    idx = torch.zeros((1, 1), dtype=torch.long)
    out = model.generate(idx, max_new_tokens=10, top_k=5)
    assert out.shape == (1, 11)


def test_can_overfit_tiny_batch():
    from torchgpt.model import GPT

    torch.manual_seed(0)
    model = GPT(_tiny_cfg())
    opt = torch.optim.AdamW(model.parameters(), lr=5e-3)
    x = torch.randint(0, 13, (4, 8))
    y = torch.randint(0, 13, (4, 8))
    first = model(x, y)[1].item()
    for _ in range(50):
        opt.zero_grad()
        loss = model(x, y)[1]
        loss.backward()
        opt.step()
    assert loss.item() < first * 0.6


def test_lora_reduces_trainable_params():
    from torchgpt.lora import apply_lora, mark_only_lora_trainable, trainable_parameter_ratio
    from torchgpt.model import GPT

    model = GPT(_tiny_cfg())
    apply_lora(model, r=4, alpha=8)
    mark_only_lora_trainable(model)
    trainable, total, ratio = trainable_parameter_ratio(model)
    assert 0 < trainable < total
    assert ratio < 0.2  # LoRA 可训练参数应远小于全参


def test_dpo_prefers_chosen():
    from torchgpt.dpo import dpo_loss

    # chosen 对数似然更高 → loss 应较小
    good = dpo_loss(torch.tensor([2.0]), torch.tensor([0.0]), torch.tensor([0.0]), torch.tensor([0.0]))[0]
    bad = dpo_loss(torch.tensor([0.0]), torch.tensor([2.0]), torch.tensor([0.0]), torch.tensor([0.0]))[0]
    assert good.item() < bad.item()


def test_perplexity_positive():
    from torchgpt.eval import perplexity
    from torchgpt.model import GPT

    model = GPT(_tiny_cfg())
    data = np.random.randint(0, 13, size=200)
    ppl = perplexity(model, data, block_size=8, max_blocks=5)
    assert ppl > 1.0
