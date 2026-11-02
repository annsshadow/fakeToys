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
docs/llm-lab/06-规模化训练.md。

注意：本模块使用 torch 2.4+ 的统一 AMP API（``torch.amp.GradScaler`` /
``torch.autocast``）；在 torch>=2.14（Python 3.14 可用的首个完整版本）上实测通过。
"""

from __future__ import annotations

import math
import os
import tempfile

import numpy as np
import torch

from .config import GPTConfig, TrainConfig
from .model import GPT


def get_batch(data, batch_size, block_size, device):
    """从 token 流随机取一个 (x, y) 批次，y 是 x 右移一位。

    第 76 轮：旧实现每个样本切两次窗、每窗再 ``astype(np.int64)``（默认
    ``copy=True``）后 ``torch.stack``——一次 batch 最多 3 份 bs×blk 的整块拷贝，
    且每样本一次 Python 级 stack，训练循环每步都调用。改为 numpy 一次取
    bs×(blk+1) 宽窗、必要时只转换一次类型，再 ``from_numpy`` 切出错开一位的
    两块并 ``contiguous()``：拷贝保持 2×bs×blk（下游 ``view`` 依赖连续性，
    旧实现返回的也是连续张量），但整批只在 C 层做两次连续化，不再逐样本 stack。
    """
    n = len(data) - block_size - 1
    ix = torch.randint(n, (batch_size,)).numpy()
    # 第 100 轮：`np.stack([data[i : i + block_size + 1] for i in ix])` 仍是
    # 逐样本 Python 切片 —— sliding_window_view 一次建出零拷贝的窗口视图，
    # 再用 `[ix]` 在 C 层取整批（一次连续拷贝），与旧实现逐元素一致。
    windows = np.lib.stride_tricks.sliding_window_view(
        np.asarray(data), block_size + 1
    )[ix]
    windows = windows.astype(np.int64, copy=False)
    batch = torch.from_numpy(windows)
    return batch[:, :-1].contiguous().to(device), batch[:, 1:].contiguous().to(device)


def _save_checkpoint(payload, target):
    """原子写 checkpoint：同目录临时文件 + ``os.replace``。

    第 94 轮：旧实现 ``torch.save`` 直接写 ``ckpt.pt``，训练途中崩溃 / Ctrl+C / 磁盘
    写满会留下半份文件，断点续训加载到坏 checkpoint。改为先写目标同目录的临时文件，
    完整落盘后才原子替换；失败时删掉半成品并保留旧 checkpoint。
    """
    parent = os.path.dirname(os.path.abspath(target)) or "."
    fd, tmp = tempfile.mkstemp(
        prefix=os.path.basename(target) + ".", suffix=".tmp", dir=parent
    )
    os.close(fd)
    try:
        torch.save(payload, tmp)
        os.replace(tmp, target)
    except BaseException:
        try:
            os.remove(tmp)
        except OSError:
            pass
        raise


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
            _save_checkpoint(
                {"model": model.state_dict(), "cfg": cfg, "step": step, "meta": meta},
                os.path.join(tc.out_dir, "ckpt.pt"),
            )
    return model
