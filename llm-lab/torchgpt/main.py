# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""torchgpt CLI —— 实战轨的命令行入口（需要 torch）。

训练与生成各一个子命令，串起 data → train → sample 的完整流程::

    python -m torchgpt.main train --data data/tiny_corpus.txt --steps 500
    python -m torchgpt.main sample --ckpt checkpoints/ckpt.pt --prompt "The " --tokens 200

训练时会把词表（stoi/itos）一并存进 checkpoint（``meta`` 字段），生成时据此
解码，无需重新读语料。
"""

from __future__ import annotations

import argparse

import torch

from .config import GPTConfig, TrainConfig
from .data import CharDataset
from .model import GPT
from .sample import load_model, sample


def cmd_train(args):
    with open(args.data, encoding="utf-8") as f:
        text = f.read()
    ds = CharDataset(text)
    device = args.device or ("cuda" if torch.cuda.is_available() else "cpu")
    cfg = GPTConfig(
        vocab_size=ds.vocab_size,
        block_size=args.block_size,
        n_layer=args.n_layer,
        n_head=4,
        n_embd=args.n_embd,
    )
    tc = TrainConfig(max_steps=args.steps, batch_size=args.batch_size, out_dir=args.out_dir)
    print(f"vocab={ds.vocab_size} device={device} params≈"
          f"{sum(p.numel() for p in GPT(cfg).parameters()):,}")
    from .train import train as train_loop

    train_loop(ds.train_ids, ds.val_ids, cfg, tc, device=device,
               meta={"stoi": ds.stoi, "itos": ds.itos})


def cmd_sample(args):
    device = args.device or ("cuda" if torch.cuda.is_available() else "cpu")
    ckpt = torch.load(args.ckpt, map_location=device, weights_only=False)
    meta = ckpt.get("meta")
    assert meta is not None, "checkpoint 缺少词表 meta；请用本 CLI 重新训练"
    ds = CharDataset.from_vocab(meta["stoi"])
    model = GPT(ckpt["cfg"]).to(device)
    model.load_state_dict(ckpt["model"])
    model.eval()
    print(sample(model, ds, prompt=args.prompt, max_new_tokens=args.tokens,
                 temperature=args.temperature, top_k=args.top_k, device=device))


def main():
    parser = argparse.ArgumentParser(prog="torchgpt", description="迷你 GPT 实战轨")
    sub = parser.add_subparsers(dest="cmd", required=True)

    p_train = sub.add_parser("train", help="在文本语料上训练")
    p_train.add_argument("--data", required=True, help="UTF-8 文本文件")
    p_train.add_argument("--steps", type=int, default=1000)
    p_train.add_argument("--batch-size", type=int, default=32)
    p_train.add_argument("--block-size", type=int, default=128)
    p_train.add_argument("--n-layer", type=int, default=4)
    p_train.add_argument("--n-embd", type=int, default=128)
    p_train.add_argument("--out-dir", default="checkpoints")
    p_train.add_argument("--device", default=None)
    p_train.set_defaults(func=cmd_train)

    p_sample = sub.add_parser("sample", help="从 checkpoint 生成文本")
    p_sample.add_argument("--ckpt", required=True)
    p_sample.add_argument("--prompt", default="\n")
    p_sample.add_argument("--tokens", type=int, default=200)
    p_sample.add_argument("--temperature", type=float, default=0.8)
    p_sample.add_argument("--top-k", type=int, default=20)
    p_sample.add_argument("--device", default=None)
    p_sample.set_defaults(func=cmd_sample)

    args = parser.parse_args()
    args.func(args)


if __name__ == "__main__":
    main()
