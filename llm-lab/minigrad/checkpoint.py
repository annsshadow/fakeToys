# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""minigrad.checkpoint —— 模型的保存与恢复。

真实训练里 checkpoint 是命根子：断点续训、离线评估、生成都靠它。这里用
``numpy.savez`` 把参数数组、模型配置与词表（``stoi``）打包进一个 ``.npz``：

* 参数按 ``Module.parameters()`` 的确定顺序存成 ``p0, p1, ...``，加载时按
  顺序回填并**逐一校验形状**（顺序由构建代码决定，配置一致则顺序一致）；
* 配置（GPTConfig 的字段）与词表以 JSON 字符串内嵌，一个文件即可自足地
  重建模型与分词器。

``train.train(..., out_ckpt="...")`` 已集成了训练结束自动保存。
"""

from __future__ import annotations

import json

import numpy as np

from .nn import GPT, GPTConfig


def save(model: GPT, path, stoi=None):
    """把模型（可选连同词表 stoi）保存为 ``.npz``。"""
    arrays = {f"p{i}": p.data for i, p in enumerate(model.parameters())}
    meta = {"_cfg": vars(model.cfg)}
    if stoi is not None:
        meta["_stoi"] = {str(k): int(v) for k, v in stoi.items()}
    np.savez(path, **arrays, **{k: np.array(json.dumps(v)) for k, v in meta.items()})


def load(path):
    """恢复模型；若存过词表则一并返回 ``(model, stoi|None)``。"""
    z = np.load(path)
    cfg = GPTConfig(**json.loads(z["_cfg"].item()))
    model = GPT(cfg)
    params = model.parameters()
    for i, p in enumerate(params):
        saved = z[f"p{i}"]
        assert saved.shape == p.data.shape, f"参数 {i} 形状不符: {saved.shape} vs {p.data.shape}"
        p.data = saved.copy()
    stoi = json.loads(z["_stoi"].item()) if "_stoi" in z.files else None
    return model, stoi


def make_tokenizer(stoi):
    """用保存下来的 ``stoi`` 重建一个 CharTokenizer。"""
    from .tokenizer import CharTokenizer

    tok = CharTokenizer("")
    tok.stoi = {k: int(v) for k, v in stoi.items()}
    tok.itos = {i: c for c, i in tok.stoi.items()}
    return tok
