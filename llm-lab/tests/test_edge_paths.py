# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""补齐 minigrad / torchgpt 的边角路径：训练入口分支、反射算子、非 FlashAttention 回退。

这一组测的不是「主流程」，而是**分支与回退**——正是覆盖率报表上最后那 5%~20%
的构成，也正是真实缺陷最爱待的地方：
* ``minigrad.train`` 的 ``tokenizer`` 分支写错会让人以为「BPE 训了个模型」，
  实际跑的是字符级；
* ``Tensor`` 的反射算子（``2 - t``）方向写反，梯度符号就错，loss 照样能降；
* torchgpt 在没有 ``scaled_dot_product_attention`` 的环境（老 torch / 其他后端）
  走手写注意力回退——这条路几乎没人跑，但它是唯一保证可移植性的路径。
"""

from __future__ import annotations

import sys

import numpy as np
import pytest

from minigrad import train as mg_train
from minigrad.engine import Tensor
from minigrad.nn import GPT as MgGPT
from minigrad.nn import GPTConfig as MgConfig
from minigrad.nn import Linear, Module
from torchgpt.config import GPTConfig

try:  # torch 是可选依赖：未装时只跳过 torchgpt 用例，minigrad 轨仍可跑
    import torch
    import torch.nn.functional as F
except ModuleNotFoundError:  # pragma: no cover - 只在无 torch 环境触发
    # 只吞「模块不存在」；装了但加载失败（DLL/依赖损坏）必须继续抛，不能假装跳过。
    torch = None
    F = None

requires_torch = pytest.mark.skipif(
    torch is None, reason="torch 未安装（见 requirements-torch.txt）"
)

TINY_TEXT = ("the quick brown fox jumps over the lazy dog. " * 6).strip()


# ─────────────────────────────────────────────── Tensor 基础属性与反射算子


def test_tensor_shape_ndim_and_repr():
    """属性/__repr__ 是调试与日志的唯一窗口，返回错形状会误导所有下游判断。"""
    t = Tensor([[1.0, 2.0, 3.0]])
    assert t.shape == (1, 3)
    assert t.ndim == 2
    assert repr(t) == "Tensor(shape=(1, 3), op='')"


@pytest.mark.parametrize(
    "op,expected,grad",
    [
        (lambda t: 2.0 + t, [3.0, 4.0], [1.0, 1.0]),
        (lambda t: 2.0 * t, [2.0, 4.0], [2.0, 2.0]),
        (lambda t: 2.0 - t, [1.0, 0.0], [-1.0, -1.0]),
    ],
)
def test_reflected_operators_forward_and_grad(op, expected, grad):
    """反射算子必须把**常量放在左边**仍能得到正确前向值与梯度方向。

    ``2 - t`` 的梯度是 -1 而不是 +1；写反了训练仍然收敛，只是方向相反，
    属于最难靠肉眼发现的错误。
    """
    t = Tensor([1.0, 2.0])
    out = op(t)
    out.backward()

    assert np.allclose(out.data, expected)
    assert np.allclose(t.grad, grad)


def test_module_parameters_walks_lists_and_tuples():
    """``Module.parameters()`` 要能穿过 list / tuple 容器。

    容器里的参数漏收 = 那些参数永远不更新（静默「部分冻结」），
    而症状只是「某一层学不动」，很难归因。
    """
    class Block(Module):
        def __init__(self):
            self.w = Tensor(np.zeros((2, 2)))
            self.layers = [Linear(2, 2), Linear(2, 2)]
            self.head = (Linear(2, 1),)  # 元组容器同样要被穿透
            self.scalar = 1.0  # 非参数，必须被忽略

    params = Block().parameters()

    # w + 两个 Linear(2,2) 各 (w,b) + Linear(2,1) 的 (w,b)
    assert len(params) == 1 + 2 * 2 + 2
    assert all(isinstance(p, Tensor) for p in params)


# ────────────────────────────────────────────────── minigrad.train 分支


def _mg_kwargs(**kw):
    base = dict(
        text=TINY_TEXT,
        steps=2,
        batch_size=2,
        block_size=8,
        n_layer=1,
        n_head=1,
        n_embd=8,
        eval_every=10,
        verbose=False,
    )
    base.update(kw)
    return base


def test_train_without_text_reads_default_corpus():
    """``text=None`` 时读内置语料 —— 这是文档里 ``python -m minigrad.train`` 的路径。"""
    model, tok = mg_train.train(**_mg_kwargs(text=None))

    assert tok.vocab_size > 10  # 内置语料是真实英文，字符表远大于测试串
    assert model.cfg.block_size == 8


def test_train_with_bpe_tokenizer_uses_merged_vocab():
    """``tokenizer="bpe"`` 必须真的走 BPE（词表下限 256 字节），而不是回落到字符级。"""
    model, tok = mg_train.train(**_mg_kwargs(tokenizer="bpe", bpe_vocab=260))

    assert tok.vocab_size >= 256
    assert model.cfg.vocab_size == tok.vocab_size


def test_train_rejects_unknown_tokenizer():
    """未知 tokenizer 要**显式报错**。静默回落到 char 会让人误以为 BPE 跑过了。"""
    with pytest.raises(ValueError, match="未知 tokenizer"):
        mg_train.train(**_mg_kwargs(tokenizer="wordpiece"))


def test_train_saves_checkpoint_and_returns_stats(tmp_path, capsys):
    """``out_ckpt`` + ``return_stats`` 的组合是「可复现实验」的接口。"""
    ckpt = tmp_path / "mini.npz"
    model, tok, stats = mg_train.train(
        **_mg_kwargs(out_ckpt=str(ckpt), return_stats=True, verbose=True)
    )

    assert ckpt.exists()
    assert "checkpoint 已保存" in capsys.readouterr().out
    assert set(stats) == {"steps", "final_train_loss", "final_val_loss", "vocab_size"}
    assert stats["steps"] == 2
    assert stats["vocab_size"] == tok.vocab_size
    assert np.isfinite(stats["final_train_loss"])
    assert np.isfinite(stats["final_val_loss"])

    # 存的 checkpoint 必须能原样载回（训练 → 保存 → 恢复 闭环）
    from minigrad.checkpoint import load

    restored, stoi = load(str(ckpt))
    assert stoi == {k: int(v) for k, v in tok.stoi.items()}
    for a, b in zip(restored.parameters(), model.parameters()):
        assert np.allclose(a.data, b.data)


# ─────────────────────────────────────────────── torchgpt.dpo.sequence_logprob


def _tiny_torch_cfg(vocab=11, block=8):
    return GPTConfig(vocab_size=vocab, block_size=block, n_layer=1, n_head=2, n_embd=8)


@requires_torch
def test_sequence_logprob_matches_manual_log_softmax_sum():
    """逐 token 对数似然之和必须与手算一致（DPO 的整个分母都建立在这个量上）。"""
    from torchgpt.dpo import sequence_logprob
    from torchgpt.model import GPT

    torch.manual_seed(0)
    model = GPT(_tiny_torch_cfg())
    input_ids = torch.randint(0, 11, (1, 6))
    labels = torch.randint(0, 11, (1, 6))

    got = sequence_logprob(model, input_ids, labels)

    with torch.no_grad():
        logits, _ = model(input_ids)
        logp = F.log_softmax(logits[:, :-1, :], dim=-1)
        expected = logp.gather(-1, labels[:, 1:].unsqueeze(-1)).squeeze(-1).sum(dim=-1)

    assert got.shape == (1,)
    assert torch.allclose(got, expected, atol=1e-5)
    assert got.item() <= 0  # 概率的乘积不会超过 1


@requires_torch
def test_sequence_logprob_ignores_minus_100_labels():
    """``-100`` 位置必须完全不计入（padding 位的 logits 是噪声，算进去会污染 DPO）。"""
    from torchgpt.dpo import sequence_logprob
    from torchgpt.model import GPT

    model = GPT(_tiny_torch_cfg())
    input_ids = torch.randint(0, 11, (1, 6))
    labels = torch.full((1, 6), -100)

    got = sequence_logprob(model, input_ids, labels)

    assert got.item() == pytest.approx(0.0, abs=1e-6)


@requires_torch
def test_sequence_logprob_ignores_first_label_position():
    """第一个 label 位不参与（logits 与 labels 各右移一位）—— 改它不能影响结果。

    这条钉的是 shift 的方向：写反了（用 labels[:, :-1] 配 logits[:, 1:]）数值
    仍然「是个负数」，但语义变成「用未来的 logits 预测过去的 token」，DPO 全错。
    """
    from torchgpt.dpo import sequence_logprob
    from torchgpt.model import GPT

    torch.manual_seed(0)
    model = GPT(_tiny_torch_cfg())
    input_ids = torch.randint(0, 11, (1, 6))
    labels = torch.randint(0, 11, (1, 6))

    base = sequence_logprob(model, input_ids, labels)
    perturbed = labels.clone()
    perturbed[0, 0] = (perturbed[0, 0] + 5) % 11
    got = sequence_logprob(model, input_ids, perturbed)

    assert torch.allclose(base, got, atol=1e-6)


# ────────────────────────────── torchgpt 非 FlashAttention 回退路径一致性


@requires_torch
def test_manual_attention_fallback_matches_flash_path(monkeypatch):
    """没有 ``scaled_dot_product_attention`` 时走手写注意力，结果必须与快路径一致。

    这条回退路径在主流环境里几乎不会被触发，因此最容易腐烂；
    但它是有 GPU/老 torch/第三方后端上唯一的正确性保证。
    """
    from torchgpt.model import GPT

    cfg = _tiny_torch_cfg()
    torch.manual_seed(1234)
    flash_model = GPT(cfg)
    idx = torch.randint(0, cfg.vocab_size, (1, 6))

    with torch.no_grad():
        expected, _ = flash_model(idx)

    monkeypatch.delattr(F, "scaled_dot_product_attention", raising=False)
    torch.manual_seed(1234)
    fallback = GPT(cfg)
    assert fallback.transformer.h[0].attn.flash is False
    # strict=False：回退路径多一个常量 buffer（因果 mask），它不是可训练参数
    fallback.load_state_dict(flash_model.state_dict(), strict=False)
    with torch.no_grad():
        got, _ = fallback(idx)

    assert torch.allclose(got, expected, atol=1e-5), "手写注意力与 SDPA 结果不一致"


@requires_torch
def test_manual_attention_is_causal(monkeypatch):
    """手写注意力必须真的加了下三角 mask：改最后一个 token 不能影响前面的输出。"""
    from torchgpt.model import GPT

    monkeypatch.delattr(F, "scaled_dot_product_attention", raising=False)
    torch.manual_seed(7)
    model = GPT(_tiny_torch_cfg())
    model.eval()

    a = torch.randint(0, 11, (1, 5))
    b = a.clone()
    b[0, -1] = (b[0, -1] + 1) % 11

    with torch.no_grad():
        out_a, _ = model(a)
        out_b, _ = model(b)

    assert torch.allclose(out_a[:, :-1], out_b[:, :-1], atol=1e-6)
    assert not torch.allclose(out_a[:, -1], out_b[:, -1], atol=1e-6)


# ─────────────────────────────────────────── 残余分支（覆盖率报表最后几行）


def test_module_parameters_collects_tensor_inside_container():
    """容器里的裸 ``Tensor`` 也要收进来（嵌入表常这样挂）。"""
    class Holder(Module):
        def __init__(self):
            self.tensors = [Tensor(np.zeros(2)), Tensor(np.zeros(3))]

    assert len(Holder().parameters()) == 2


@requires_torch
def test_lora_linear_starts_as_identity_and_applies_scaled_delta():
    """LoRA 的两个硬约束：初始等价原模型；启用后增量按 alpha/r 缩放。

    初始不为 0 会让「微调」变成「破坏」——一开始就把预训练权重推离原位。
    """
    import torch.nn as nn

    from torchgpt.lora import LoRALinear

    base = nn.Linear(4, 3)
    lora = LoRALinear(base, r=2, alpha=8)
    x = torch.randn(2, 4)

    with torch.no_grad():
        assert torch.allclose(lora(x), base(x), atol=1e-6)  # lora_b 初始为 0

        lora.lora_b.fill_(1.0)
        delta = (x @ lora.lora_a.t()) @ lora.lora_b.t()
        assert torch.allclose(lora(x), base(x) + delta * 4.0, atol=1e-5)  # alpha/r = 4


def test_minigrad_generate_with_zero_temperature_is_deterministic():
    """``temperature=0`` 是贪心解码：同一 prompt 两次生成必须逐 token 相同。

    随机采样与贪心的分叉点就在这里；写错的话「可复现生成」这个承诺失效。
    """
    from minigrad.sample import generate

    model = MgGPT(MgConfig(vocab_size=8, block_size=8, n_layer=1, n_head=1, n_embd=8))
    a = generate(model, [1, 2], max_new_tokens=5, temperature=0.0)
    b = generate(model, [1, 2], max_new_tokens=5, temperature=0.0)

    assert list(a) == list(b)
    assert len(list(a)) == 2 + 5


@requires_torch
def test_perplexity_without_max_blocks_walks_entire_sequence():
    """不给 ``max_blocks`` 时必须走完所有窗口（不提前 break）。"""
    from torchgpt.eval import perplexity
    from torchgpt.model import GPT

    torch.manual_seed(0)
    model = GPT(_tiny_torch_cfg())
    data = np.random.randint(0, 11, size=40).astype(np.int64)

    full = perplexity(model, data, block_size=8, max_blocks=None)
    capped = perplexity(model, data, block_size=8, max_blocks=1)

    assert np.isfinite(full) and full > 1.0
    assert full != capped  # 单窗口与全序列不是同一个估计


@requires_torch
def test_torchgpt_main_module_entrypoint(tmp_path, monkeypatch):
    """``python -m torchgpt.main`` 的 ``__main__`` 分支必须可用。

    这条不测训练质量，只测「模块能被当作脚本执行」——打包/部署脚本依赖它。
    """
    import runpy

    corpus = tmp_path / "corpus.txt"
    corpus.write_text("abab" * 40, encoding="utf-8")
    out_dir = tmp_path / "ckpt"
    monkeypatch.setattr(
        sys,
        "argv",
        [
            "torchgpt.main",
            "train",
            "--data",
            str(corpus),
            "--steps",
            "1",
            "--batch-size",
            "2",
            "--block-size",
            "8",
            "--n-layer",
            "1",
            "--n-embd",
            "8",
            "--out-dir",
            str(out_dir),
            "--device",
            "cpu",
        ],
    )

    runpy.run_module("torchgpt.main", run_name="__main__")

    assert (out_dir / "ckpt.pt").exists()


def test_visualize_module_entrypoint(tmp_path, monkeypatch, capsys):
    """``python -m minigrad.visualize`` 的 ``__main__`` 分支（argparse 接线）。

    与上面 torchgpt 同理：这条是文档里写给用户的第一条命令。
    """
    import runpy

    from minigrad import checkpoint
    from minigrad.tokenizer import CharTokenizer

    tok = CharTokenizer(TINY_TEXT)
    model = MgGPT(MgConfig(vocab_size=tok.vocab_size, block_size=16, n_layer=1, n_head=2, n_embd=8))
    ckpt = tmp_path / "m.npz"
    checkpoint.save(model, ckpt, stoi=tok.stoi)

    corpus = tmp_path / "corpus.txt"
    corpus.write_text(TINY_TEXT, encoding="utf-8")
    out = tmp_path / "entry.html"
    monkeypatch.setattr(
        sys,
        "argv",
        ["minigrad.visualize", "--ckpt", str(ckpt), "--text", str(corpus),
         "--phrase", "the", "--out", str(out)],
    )

    # runpy 会警告「模块已在 sys.modules 里」——前面几个用例已经 import 过它。
    # 先摘掉再执行，等价于干净进程里 `python -m minigrad.visualize`。
    sys.modules.pop("minigrad.visualize", None)
    runpy.run_module("minigrad.visualize", run_name="__main__")

    assert out.exists()
    assert "注意力可视化已生成" in capsys.readouterr().out
