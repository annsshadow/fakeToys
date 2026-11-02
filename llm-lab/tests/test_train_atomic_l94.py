# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""第 94 轮：torchgpt checkpoint 原子写守卫。

训练里 checkpoint 是断点续训的命根子。旧实现 ``torch.save`` 直接写 ``ckpt.pt``：
崩溃 / Ctrl+C / 磁盘写满会留下半份文件，加载端拿到的是坏 checkpoint。这里钉住
三条性质：

* 正常落盘可完整往返，且不残留临时文件；
* 写入中途失败时**旧 checkpoint 完好**、半成品被清理；
* ``train`` 的落盘节奏走的是同一条原子路径（集成验证）。
"""

from __future__ import annotations

import numpy as np
import pytest

torch = pytest.importorskip("torch")  # 未安装 torch 时整文件跳过

from torchgpt.config import GPTConfig, TrainConfig  # noqa: E402


def _tiny_cfg(vocab_size=13, block_size=8):
    return GPTConfig(
        vocab_size=vocab_size,
        block_size=block_size,
        n_layer=1,
        n_head=2,
        n_embd=8,
        dropout=0.0,
    )


def _tiny_train_cfg(**kw):
    base = dict(
        max_steps=2,
        batch_size=2,
        eval_every=1,
        eval_iters=1,
        warmup_steps=1,
        amp=False,
        seed=0,
    )
    base.update(kw)
    return TrainConfig(**base)


def test_save_checkpoint_roundtrips_and_leaves_no_temp(tmp_path):
    """正常路径：产物可加载，目录里只剩目标文件（临时件已被替换掉）。"""
    from torchgpt.train import _save_checkpoint

    target = tmp_path / "ckpt.pt"
    payload = {"step": 7, "meta": {"stoi": {"a": 0}}}

    _save_checkpoint(payload, str(target))

    assert torch.load(target, map_location="cpu", weights_only=False) == payload
    assert [p.name for p in tmp_path.iterdir()] == ["ckpt.pt"]


def test_interrupted_save_keeps_old_checkpoint_and_cleans_temp(tmp_path, monkeypatch):
    """写入中途抛异常：旧 checkpoint 逐字节不变，半成品临时文件必须清掉。"""
    from torchgpt.train import _save_checkpoint

    target = tmp_path / "ckpt.pt"
    _save_checkpoint({"step": 1}, str(target))
    before = target.read_bytes()

    def boom(payload, path):
        # 模拟写到一半就失败（磁盘满 / 进程被杀的最坏形状）
        with open(path, "wb") as f:
            f.write(b"half-written")
        raise RuntimeError("disk full")

    monkeypatch.setattr(torch, "save", boom)
    with pytest.raises(RuntimeError, match="disk full"):
        _save_checkpoint({"step": 2}, str(target))

    assert target.read_bytes() == before, "中断后旧 checkpoint 必须完好"
    assert [p.name for p in tmp_path.iterdir()] == ["ckpt.pt"], "临时文件必须被清理"


def test_train_writes_checkpoint_atomically(tmp_path):
    """集成：train 的落盘必须走原子路径，训练结束后无临时文件残留。"""
    from torchgpt.train import train

    cfg = _tiny_cfg()
    data = np.random.randint(0, cfg.vocab_size, size=64).astype(np.int64)
    out = tmp_path / "ckpt"
    tc = _tiny_train_cfg(max_steps=1, eval_every=1, out_dir=str(out))

    train(data, data, cfg, tc, device="cpu")

    ckpt = torch.load(out / "ckpt.pt", map_location="cpu", weights_only=False)
    assert ckpt["step"] == 0
    assert [p.name for p in out.iterdir()] == ["ckpt.pt"]
