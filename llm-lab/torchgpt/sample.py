# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""torchgpt.sample —— 从 checkpoint 加载模型并生成文本（PyTorch 轨）。"""

from __future__ import annotations

import torch

from .data import CharDataset
from .model import GPT


def load_model(ckpt_path, device="cpu"):
    ckpt = torch.load(ckpt_path, map_location=device, weights_only=False)
    model = GPT(ckpt["cfg"]).to(device)
    model.load_state_dict(ckpt["model"])
    model.eval()
    return model


def sample(model, dataset: CharDataset, prompt="\n", max_new_tokens=200,
           temperature=0.8, top_k=20, device="cpu"):
    idx = torch.from_numpy(dataset.encode(prompt))[None].to(device)
    out = model.generate(idx, max_new_tokens, temperature=temperature, top_k=top_k)
    return dataset.decode(out[0].tolist())
