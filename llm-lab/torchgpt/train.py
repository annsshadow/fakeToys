# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""torchgpt.train —— 工业级训练循环的最小可运行版。

演示的"真·训练"要点：
* AMP 自动混合精度（``torch.autocast`` + ``GradScaler``）省显存、提速；
* 梯度累积（grad accumulation）用小 batch 等效大 batch；
* 梯度裁剪（grad clipping）防梯度爆炸；
* 带 warmup 的余弦学习率；
* 参数分组权重衰减（只对 2D 权重做 weight decay，不衰减 bias/LayerNorm）；
* 周期性验证与 checkpoint。

单机多卡请用 ``torchrun --nproc_per_node=N`` 启动并包一层 DDP，详见
docs/06-规模化训练.md。

注意：本模块使用 torch 2.4+ 的统一 AMP API（``torch.amp.GradScaler`` /
``torch.autocast``）；在 torch>=2.14（Python 3.14 可用的首个完整版本）上实测通过。
"""

from __future__ import annotations

import math
import os

import numpy as np
import torch

from .config import GPTConfig, TrainConfig
from .model import GPT


def get_batch(data, batch_size, block_size, device):
    ix = torch.randint(len(data) - block_size - 1, (batch_size,))
    x = torch.stack([torch.from_numpy(data[i : i + block_size].astype(np.int64)) for i in ix])
    y = torch.stack([torch.from_numpy(data[i + 1 : i + 1 + block_size].astype(np.int64)) for i in ix])
    return x.to(device), y.to(device)


def cosine_lr(step, tc: TrainConfig):
    if step < tc.warmup_steps:
        return tc.lr * (step + 1) / tc.warmup_steps
    if step >= tc.max_steps:
        return tc.min_lr
    ratio = (step - tc.warmup_steps) / max(1, tc.max_steps - tc.warmup_steps)
    return tc.min_lr + 0.5 * (tc.lr - tc.min_lr) * (1 + math.cos(math.pi * ratio))


def configure_optimizer(model, tc: TrainConfig):
    """2D 以上参数做权重衰减，bias/LayerNorm 不衰减——GPT 训练惯例。"""
    decay, no_decay = [], []
    for p in model.parameters():
        if not p.requires_grad:
            continue
        (decay if p.dim() >= 2 else no_decay).append(p)
    groups = [
        {"params": decay, "weight_decay": tc.weight_decay},
        {"params": no_decay, "weight_decay": 0.0},
    ]
    return torch.optim.AdamW(groups, lr=tc.lr, betas=(tc.beta1, tc.beta2))


@torch.no_grad()
def estimate_loss(model, data, tc, block_size, device):
    model.eval()
    losses = torch.zeros(tc.eval_iters)
    for i in range(tc.eval_iters):
        x, y = get_batch(data, tc.batch_size, block_size, device)
        _, loss = model(x, y)
        losses[i] = loss.item()
    model.train()
    return losses.mean().item()


def train(train_data, val_data, cfg: GPTConfig, tc: TrainConfig, device="cpu", meta=None):
    torch.manual_seed(tc.seed)
    model = GPT(cfg).to(device)
    opt = configure_optimizer(model, tc)
    use_amp = tc.amp and device.startswith("cuda")
    # torch 2.4+ 统一 AMP API；enabled=False 时是完全直通（no-op）
    scaler = torch.amp.GradScaler("cuda" if use_amp else "cpu", enabled=use_amp)

    for step in range(tc.max_steps):
        for g in opt.param_groups:
            g["lr"] = cosine_lr(step, tc)

        opt.zero_grad(set_to_none=True)
        for _ in range(tc.grad_accum_steps):  # 梯度累积
            x, y = get_batch(train_data, tc.batch_size, cfg.block_size, device)
            with torch.autocast(device_type="cuda" if use_amp else "cpu", enabled=use_amp):
                _, loss = model(x, y)
                loss = loss / tc.grad_accum_steps
            scaler.scale(loss).backward()
        scaler.unscale_(opt)
        torch.nn.utils.clip_grad_norm_(model.parameters(), tc.grad_clip)  # 梯度裁剪
        scaler.step(opt)
        scaler.update()

        if step % tc.eval_every == 0 or step == tc.max_steps - 1:
            val = estimate_loss(model, val_data, tc, cfg.block_size, device)
            print(f"step {step:5d} | val loss {val:.4f}")
            os.makedirs(tc.out_dir, exist_ok=True)
            torch.save(
                {"model": model.state_dict(), "cfg": cfg, "step": step, "meta": meta},
                os.path.join(tc.out_dir, "ckpt.pt"),
            )
    return model
