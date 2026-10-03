# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""尺度实验：模型变大，loss 如何变化？

Scaling laws（Kaplan et al. 2020 / Hoffmann et al. 2022 "Chinchilla"）是现代大
模型训练的指南针。这里用一个玩具版实验建立直觉：在**同一语料、同一训练步数**
下训练几个不同大小的 GPT，记录参数量与最终 loss，输出 CSV + Markdown 表。

用法::

    python -m experiments.scaling_law                 # 默认 3 档大小，每档 300 步
    python -m experiments.scaling_law --steps 600 --out scaling_results.csv

注意这是"同步数对比"的玩具实验，真实 scaling 研究要控制算力、数据量并拟合
幂律；但它足以让你亲手看到"更大的模型 loss 更低、且开始过拟合"的现象。
"""

from __future__ import annotations

import argparse
import csv
import os
import sys

sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))

from minigrad.train import DEFAULT_CORPUS, train  # noqa: E402

SIZES = [
    ("S", dict(n_layer=2, n_head=2, n_embd=32)),
    ("M", dict(n_layer=3, n_head=4, n_embd=64)),
    ("L", dict(n_layer=4, n_head=4, n_embd=96)),
]


def run(steps=300, batch_size=16, block_size=32, out_csv=None, seed=1337, verbose=True):
    """训练各档模型，返回统计行列表（并可写出 CSV）。"""
    if out_csv is None:
        out_csv = os.path.join(os.path.dirname(os.path.abspath(__file__)), "scaling_results.csv")
    with open(os.path.normpath(DEFAULT_CORPUS), encoding="utf-8") as f:
        text = f.read()

    rows = []
    for name, kw in SIZES:
        model, tok, stats = train(
            text=text, steps=steps, batch_size=batch_size, block_size=block_size,
            seed=seed, return_stats=True, verbose=False, **kw
        )
        params = sum(p.data.size for p in model.parameters())
        rows.append({
            "size": name,
            "params": params,
            "final_train_loss": round(stats["final_train_loss"], 4),
            "final_val_loss": round(stats["final_val_loss"], 4),
            "vocab_size": stats["vocab_size"],
        })
        if verbose:
            print(f"[{name}] params={params:>8,}  train={stats['final_train_loss']:.4f}  "
                  f"val={stats['final_val_loss']:.4f}")

    with open(out_csv, "w", newline="", encoding="utf-8") as f:
        writer = csv.DictWriter(f, fieldnames=list(rows[0].keys()))
        writer.writeheader()
        writer.writerows(rows)
    if verbose:
        print(f"\n已写出 {out_csv}")
        print("| size | params | train loss | val loss |")
        print("| --- | --- | --- | --- |")
        for r in rows:
            print(f"| {r['size']} | {r['params']:,} | {r['final_train_loss']} | {r['final_val_loss']} |")
    return rows


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="玩具版 scaling 实验")
    parser.add_argument("--steps", type=int, default=300)
    parser.add_argument("--out", default=None, help="CSV 输出路径")
    args = parser.parse_args()
    run(steps=args.steps, out_csv=args.out)
