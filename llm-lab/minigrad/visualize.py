# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""minigrad.visualize —— 注意力权重可视化（零依赖，输出独立 HTML）。

注意力是 Transformer 的心脏，"看见"它是最直观的学习方式。``CausalSelfAttention``
在前向时会把权重存到 ``self.last_att``，本模块据此生成一个无任何外部依赖、
双击即可在浏览器打开的 HTML：每层一行、每个头一格的注意力热力图，
行是 query（正在生成的词），列是 key（它看向的词），颜色越深权重越大。

用法::

    python -m minigrad.visualize            # 训练一个小模型并生成 data/attention_demo.html
    python -m minigrad.visualize --ckpt x.npz --text "attention lets each position look back"
"""

from __future__ import annotations

import argparse
import html
import os

import numpy as np


def attention_maps(model, idx):
    """对一段 token 跑一次前向，返回注意力图列表。

    返回 ``maps[layer]`` 形状 ``(n_head, T, T)``；每行是一个 query 对所有
    key 的注意力分布（因果 mask 下右上方全为 0）。
    """
    idx = np.asarray(idx)
    assert idx.ndim == 1, "请传一维 token 序列"
    model.forward(idx[None])
    return [block.attn.last_att[0] for block in model.blocks]


def _display(tok):
    tok = html.escape(tok)
    return tok.replace("\n", "⏎") or "␠"


def to_html(maps, tokens, path, title="Attention Maps"):
    """把注意力图写成独立 HTML（无 JS 依赖）。tokens 为对应文本片段列表。"""
    T = len(tokens)
    labels = [_display(t) for t in tokens]
    parts = [
        "<!doctype html><html><head><meta charset='utf-8'>",
        f"<title>{html.escape(title)}</title><style>",
        "body{font-family:ui-monospace,Consolas,monospace;background:#0d1117;color:#c9d1d9;margin:24px}",
        "h1{font-size:18px}h2{font-size:14px;margin:18px 0 6px}",
        "table{border-collapse:collapse;margin-bottom:8px}",
        "td{width:26px;height:26px;min-width:26px;text-align:center;font-size:9px;"
        "border:1px solid #21262d;padding:0}",
        "td.lbl{background:#161b22;color:#8b949e;font-size:10px;white-space:nowrap}",
        ".wrap{overflow-x:auto}",
        "</style></head><body>",
        f"<h1>{html.escape(title)}</h1>",
        f"<p>{T} tokens · {len(maps)} layers × {maps[0].shape[0]} heads · "
        "行=query　列=key　颜色越深=权重越大（因果 mask：上半三角恒为 0）</p>",
    ]
    for li, layer in enumerate(maps):
        parts.append(f"<h2>layer {li}</h2><div class='wrap'>")
        for hi, head in enumerate(layer):
            parts.append(f"<p style='margin:4px 0 2px;font-size:12px'>head {hi}</p><table>")
            parts.append("<tr><td class='lbl'></td>" + "".join(f"<td class='lbl'>{l}</td>" for l in labels) + "</tr>")
            for qi in range(T):
                row = [f"<tr><td class='lbl'>{labels[qi]}</td>"]
                for ki in range(T):
                    a = float(head[qi, ki])
                    tip = f" title='q={labels[qi]} k={labels[ki]} w={a:.3f}'" if T <= 16 else ""
                    cell = f"<td{tip} style='background:rgba(56,139,253,{a:.3f})'>"
                    cell += f"{a:.2f}</td>" if T <= 16 else "</td>"
                    row.append(cell)
                row.append("</tr>")
                parts.append("".join(row))
            parts.append("</table>")
        parts.append("</div>")
    parts.append("</body></html>")
    with open(path, "w", encoding="utf-8") as f:
        f.write("".join(parts))
    return path


def demo(text=None, ckpt=None, out_path=None, sample_text=None):
    """训练（或加载）一个小模型，生成注意力可视化 HTML，返回文件路径。"""
    from .checkpoint import load, make_tokenizer
    from .train import train
    from .tokenizer import CharTokenizer

    if out_path is None:
        out_path = os.path.join(os.path.dirname(__file__), "..", "data", "attention_demo.html")
    out_path = os.path.normpath(out_path)

    if ckpt:
        model, stoi = load(ckpt)
        if stoi is None:
            raise ValueError("checkpoint 未含词表，无法解码 token；请用 train(out_ckpt=...) 重新保存")
        tok = make_tokenizer(stoi)
    else:
        model, tok = train(text=text, steps=150, block_size=48, n_embd=48,
                           n_layer=3, n_head=3, verbose=False)
    corpus = text
    if corpus is None:
        from .train import DEFAULT_CORPUS

        with open(os.path.normpath(DEFAULT_CORPUS), encoding="utf-8") as f:
            corpus = f.read()
    phrase = sample_text or corpus.split("\n")[0]
    ids = tok.encode(phrase)[: model.cfg.block_size]
    maps = attention_maps(model, ids)
    tokens = [tok.itos[int(i)] for i in ids]
    to_html(maps, tokens, out_path, title=f"attention · {phrase[:40]!r}")
    return out_path


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="可视化一个迷你 GPT 的注意力")
    parser.add_argument("--ckpt", default=None, help="checkpoint 路径（缺省则现场训练一个小模型）")
    parser.add_argument("--text", default=None, help="自定义语料文件（与 --ckpt 配套）")
    parser.add_argument("--phrase", default=None, help="要可视化的句子（缺省取语料首行）")
    parser.add_argument("--out", default=None, help="输出 HTML 路径")
    args = parser.parse_args()
    corpus = None
    if args.text:
        with open(args.text, encoding="utf-8") as f:
            corpus = f.read()
    path = demo(text=corpus, ckpt=args.ckpt, out_path=args.out, sample_text=args.phrase)
    print(f"注意力可视化已生成: {path}")
