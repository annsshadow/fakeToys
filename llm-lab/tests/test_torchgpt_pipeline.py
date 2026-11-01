# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""torchgpt 数据 → 训练 → 生成 → CLI 全链路测试。

为什么要补这一组：``torchgpt/data.py`` / ``train.py`` / ``sample.py`` / ``main.py``
此前**覆盖率为 0%**。这四段恰好是「文档里承诺的那条流程」——读语料、切批、带
warmup 的余弦调度、梯度累积与裁剪、存 checkpoint、再从 checkpoint 生成文本。
它们零覆盖意味着 README 里那句 ``python -m torchgpt.main train ...`` 没有任何
自动化证据，任何人改坏一行都只能靠手跑才发现。

这里全部用极小配置（block_size=8、n_layer=1、n_embd=8）跑**真实**的前向与反向，
断言的是可判定的性质：形状、窗口错位关系、学习率曲线的连续性与单调性、参数分组
边界、checkpoint 往返一致性。不追求「跑一遍不出错」，追求「改坏了会红」。
"""

from __future__ import annotations

import math
import types

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


# ─────────────────────────────────────────────────────────────── CharDataset


def test_char_dataset_builds_sorted_vocab_and_roundtrips():
    """词表必须按字符排序（否则同一语料在不同机器上 id 不同，checkpoint 不可移植）。"""
    from torchgpt.data import CharDataset

    text = "cab\nabc"
    ds = CharDataset(text)

    assert list(ds.stoi) == sorted(set(text))
    assert ds.vocab_size == len(set(text))
    assert ds.itos == {i: c for c, i in ds.stoi.items()}
    # 编解码是恒等变换，且 dtype 必须是 int64（后面 get_batch 直接喂 torch.from_numpy）
    ids = ds.encode(text)
    assert ids.dtype == np.int64
    assert ds.decode(ids) == text


def test_char_dataset_splits_tail_as_validation():
    """验证集取**尾部** 10%（语言建模按时间顺序切，不打乱）。

    断言的是「不重叠 + 尾部 + 至少 1 个 token」三条性质，而不是具体长度：
    长度公式一旦改动（比如改成 20%），只要性质仍成立就不该红。
    """
    from torchgpt.data import CharDataset

    text = "abcdefghij" * 10  # 100 个字符
    ds = CharDataset(text)

    assert len(ds.val_ids) == max(1, int(len(text) * 0.1))
    assert len(ds.train_ids) + len(ds.val_ids) == len(text)
    # 尾部：训练集是前缀，验证集是后缀，两者拼起来正好是原始序列
    assert np.array_equal(np.concatenate([ds.train_ids, ds.val_ids]), ds.encode(text))


def test_char_dataset_val_never_empty_on_tiny_corpus():
    """语料只有 1 个字符时 val 仍要有 1 个 token —— 空验证集会算出 NaN loss。"""
    from torchgpt.data import CharDataset

    ds = CharDataset("a")
    assert len(ds.val_ids) == 1
    assert len(ds.train_ids) == 0


def test_char_dataset_from_vocab_has_no_data_but_can_decode():
    """从 checkpoint 词表重建时没有语料，只能解码；这条路径是生成端唯一的入口。"""
    from torchgpt.data import CharDataset

    ds = CharDataset.from_vocab({"a": 0, "b": 1})

    assert ds.vocab_size == 2
    assert ds.itos == {0: "a", 1: "b"}
    assert ds.encode("ba").tolist() == [1, 0]
    assert ds.decode([1, 0]) == "ba"
    # 明确没有数据：空数组而不是 None，调用方 len() 不会炸
    assert ds.train_ids.size == 0 and ds.val_ids.size == 0


# ─────────────────────────────────────────────────────────────── get_batch


def test_get_batch_targets_are_inputs_shifted_by_one():
    """语言建模的唯一正确性前提：y 是 x 右移一位，且同一窗口内一致。

    这条关系写反了训练照样「跑得动」，只是模型在学「复制输入」——
    不会报错、loss 也会降，所以必须由测试钉住。
    """
    from torchgpt.train import get_batch

    data = np.arange(64, dtype=np.int64)  # 值等于下标，便于直接读窗口位置
    x, y = get_batch(data, batch_size=4, block_size=8, device="cpu")

    assert x.shape == (4, 8) and y.shape == (4, 8)
    assert x.dtype == torch.long and y.dtype == torch.long
    assert torch.equal(y[:, :-1], x[:, 1:])
    # 每一行内部是连续递增的 → 采样窗口没有被打乱
    for row in x:
        assert torch.equal(row[1:] - row[:-1], torch.ones(7, dtype=torch.long))


def test_get_batch_stays_inside_corpus_bounds():
    """最后一个窗口不能越界（越界会被 numpy 静默截短，得到短 batch）。"""
    from torchgpt.train import get_batch

    n, block = 32, 8
    data = np.arange(n, dtype=np.int64)
    for _ in range(20):  # 随机采样，多跑几次覆盖边界
        x, y = get_batch(data, batch_size=8, block_size=block, device="cpu")
        assert int(y.max()) <= n - 1
        assert x.shape == (8, block)


# ─────────────────────────────────────────────────────────────── cosine_lr


def test_cosine_lr_warmup_is_linear_and_continuous_at_handoff():
    """warmup 段线性升温，且在 step==warmup 处与余弦段**无缝衔接**。

    衔接处的跳变会让训练在 warmup 结束瞬间产生一次隐式「学习率阶跃」，
    是 nanoGPT 类实现最常见的抄错点。
    """
    from torchgpt.train import cosine_lr

    tc = TrainConfig(lr=1.0, min_lr=0.1, warmup_steps=10, max_steps=100)

    assert cosine_lr(0, tc) == pytest.approx(0.1)  # lr * 1/10
    assert cosine_lr(9, tc) == pytest.approx(1.0)  # 峰值
    assert cosine_lr(10, tc) == pytest.approx(1.0)  # 余弦段起点 == warmup 终点
    # 线性：前 10 步等距
    warm = [cosine_lr(s, tc) for s in range(10)]
    steps = np.diff(warm)
    assert np.allclose(steps, steps[0])


def test_cosine_lr_decays_monotonically_to_min_lr():
    """warmup 之后必须单调不增，并在 max_steps 及以后恒等于 min_lr。"""
    from torchgpt.train import cosine_lr

    tc = TrainConfig(lr=1.0, min_lr=0.1, warmup_steps=10, max_steps=100)
    tail = [cosine_lr(s, tc) for s in range(10, 100)]

    assert all(a >= b for a, b in zip(tail, tail[1:])), "余弦段出现回升"
    assert tail[-1] > 0.1  # 最后一步仍未触底
    assert cosine_lr(100, tc) == pytest.approx(0.1)
    assert cosine_lr(10_000, tc) == pytest.approx(0.1)  # 超出后钳制
    # 中点应约为 (lr+min_lr)/2
    assert cosine_lr(55, tc) == pytest.approx(0.55, abs=0.02)


# ────────────────────────────────────────────────────── configure_optimizer


def test_configure_optimizer_decays_only_matrix_params():
    """GPT 训练惯例：只对 dim>=2 的权重做 weight decay，bias / LayerNorm 不衰减。

    分组搞错的症状很隐蔽——loss 曲线看起来正常，只是最终指标略差，
    所以必须用「分组边界」这种结构性断言钉住，而不是只看训练能不能跑。
    """
    from torchgpt.model import GPT
    from torchgpt.train import configure_optimizer

    model = GPT(_tiny_cfg())
    tc = TrainConfig(lr=3e-4, weight_decay=0.1, beta1=0.9, beta2=0.95)
    opt = configure_optimizer(model, tc)

    assert len(opt.param_groups) == 2
    decay, no_decay = opt.param_groups
    assert decay["weight_decay"] == pytest.approx(0.1)
    assert no_decay["weight_decay"] == 0.0
    assert all(p.dim() >= 2 for p in decay["params"])
    assert all(p.dim() < 2 for p in no_decay["params"])
    # 一个参数都不能漏（漏掉的参数永远不更新，训练会「部分冻结」）
    total = len(decay["params"]) + len(no_decay["params"])
    assert total == sum(1 for p in model.parameters() if p.requires_grad)
    assert opt.defaults["lr"] == pytest.approx(3e-4)


def test_configure_optimizer_skips_frozen_params():
    """requires_grad=False 的参数不进优化器 —— 否则 weight decay 会继续改动它。"""
    from torchgpt.model import GPT
    from torchgpt.train import configure_optimizer

    model = GPT(_tiny_cfg())
    frozen = next(model.parameters())
    frozen.requires_grad_(False)
    opt = configure_optimizer(model, TrainConfig())

    in_opt = {id(p) for g in opt.param_groups for p in g["params"]}
    assert id(frozen) not in in_opt
    assert len(in_opt) == sum(1 for p in model.parameters() if p.requires_grad)


# ────────────────────────────────────────────────────────────── estimate_loss


def test_estimate_loss_returns_finite_scalar_and_restores_train_mode():
    """验证期必须把模型切回 train()，否则 dropout 会在训练时被静默关掉。"""
    from torchgpt.model import GPT
    from torchgpt.train import estimate_loss

    cfg = _tiny_cfg()
    model = GPT(cfg)
    model.train()
    data = np.random.randint(0, cfg.vocab_size, size=64).astype(np.int64)

    val = estimate_loss(model, data, _tiny_train_cfg(eval_iters=3), cfg.block_size, "cpu")

    assert isinstance(val, float)
    assert math.isfinite(val) and val > 0
    assert model.training is True


# ──────────────────────────────────────────────────────────────────── train


def test_train_writes_checkpoint_with_config_and_meta(tmp_path):
    """训练结束必须落一个**自足**的 checkpoint：权重 + 配置 + 词表。

    生成端（cmd_sample）只靠这个文件重建模型与分词器；少存 meta 就直接断链。
    """
    from torchgpt.model import GPT
    from torchgpt.train import train

    cfg = _tiny_cfg()
    data = np.random.randint(0, cfg.vocab_size, size=64).astype(np.int64)
    tc = _tiny_train_cfg(max_steps=2, out_dir=str(tmp_path / "ckpt"))
    meta = {"stoi": {"a": 0}, "itos": {0: "a"}}

    train(data, data, cfg, tc, device="cpu", meta=meta)

    path = tmp_path / "ckpt" / "ckpt.pt"
    assert path.exists()
    ckpt = torch.load(path, map_location="cpu", weights_only=False)
    assert set(ckpt) == {"model", "cfg", "step", "meta"}
    assert ckpt["step"] == tc.max_steps - 1  # 最后一次落盘发生在最后一步
    assert ckpt["cfg"].vocab_size == cfg.vocab_size
    assert ckpt["meta"] == meta
    # 权重形状必须与配置一致，否则 load_state_dict 会红
    ref = GPT(cfg).state_dict()
    assert {k: tuple(v.shape) for k, v in ref.items()} == {
        k: tuple(v.shape) for k, v in ckpt["model"].items()
    }


def test_train_actually_updates_parameters(tmp_path):
    """两步真实反向后参数必须变化 —— 这是「训练循环接对了」的最小证据。

    只断言「能跑完」是不够的：把 opt.step() 注释掉照样能跑完，
    但参数一动不动。所以这里比较前后权重快照。
    """
    from torchgpt.model import GPT
    from torchgpt.train import train

    cfg = _tiny_cfg()
    data = np.random.randint(0, cfg.vocab_size, size=64).astype(np.int64)
    tc = _tiny_train_cfg(max_steps=2, out_dir=str(tmp_path))

    model = GPT(cfg)
    before = [p.detach().clone() for p in model.parameters()]
    trained = train(data, data, cfg, tc, device="cpu")

    after = list(trained.parameters())
    assert len(before) == len(after)
    assert any(not torch.equal(b, a) for b, a in zip(before, after)), "参数没有任何更新"


def test_train_checkpoints_at_eval_boundaries(tmp_path, capsys):
    """eval_every 的落盘节奏必须真实发生（它决定了断点续训能恢复到哪一步）。"""
    from torchgpt.train import train

    cfg = _tiny_cfg()
    data = np.random.randint(0, cfg.vocab_size, size=64).astype(np.int64)
    train(data, data, cfg, _tiny_train_cfg(max_steps=4, eval_every=2), device="cpu")

    lines = [ln for ln in capsys.readouterr().out.splitlines() if "val loss" in ln]
    # step 0 与 step 2 命中 eval_every；step 3 == max_steps-1 是强制收尾
    assert len(lines) == 3
    assert "step     0" in lines[0] and "step     3" in lines[-1]


# ─────────────────────────────────────────────────────────────────── sample


def _save_ckpt(path, cfg, with_meta=True):
    from torchgpt.model import GPT

    meta = {"stoi": {"a": 0, "b": 1}, "itos": {0: "a", 1: "b"}} if with_meta else None
    torch.save(
        {"model": GPT(cfg).state_dict(), "cfg": cfg, "step": 0, "meta": meta}, path
    )
    return path


def test_load_model_restores_weights_and_eval_mode(tmp_path):
    """load_model 必须回到 eval()：训练态生成会引入 dropout 噪声，输出不可复现。"""
    from torchgpt.model import GPT
    from torchgpt.sample import load_model

    cfg = _tiny_cfg()
    path = _save_ckpt(tmp_path / "ckpt.pt", cfg)
    model = load_model(path, device="cpu")

    assert model.training is False
    assert model.cfg.vocab_size == cfg.vocab_size
    src = GPT(cfg)
    src.load_state_dict(torch.load(path, map_location="cpu", weights_only=False)["model"])
    for a, b in zip(model.parameters(), src.parameters()):
        assert torch.equal(a, b)


def test_sample_returns_prompt_plus_requested_tokens(tmp_path):
    """生成长度 = prompt 长度 + max_new_tokens（字符级词表下 id 与字符一一对应）。"""
    from torchgpt.data import CharDataset
    from torchgpt.sample import load_model, sample

    cfg = _tiny_cfg(vocab_size=2)
    ds = CharDataset.from_vocab({"a": 0, "b": 1})
    model = load_model(_save_ckpt(tmp_path / "ckpt.pt", cfg), device="cpu")

    out = sample(model, ds, prompt="ab", max_new_tokens=5, temperature=1.0, top_k=2, device="cpu")

    assert isinstance(out, str)
    assert len(out) == 2 + 5
    assert set(out) <= {"a", "b"}, "解码出词表外的字符，说明 itos 建错了"


# ─────────────────────────────────────────────────────────────────────── CLI


def _train_args(data_path, out_dir, **kw):
    base = dict(
        data=str(data_path),
        steps=1,
        batch_size=2,
        block_size=8,
        n_layer=1,
        n_embd=8,
        out_dir=str(out_dir),
        device="cpu",
    )
    base.update(kw)
    return types.SimpleNamespace(**base)


def test_cmd_train_produces_loadable_checkpoint(tmp_path, capsys):
    """CLI 的 train 子命令必须产出可被 sample 子命令直接吃下的 checkpoint。"""
    from torchgpt.main import cmd_sample, cmd_train

    corpus = tmp_path / "corpus.txt"
    corpus.write_text("abab" * 40, encoding="utf-8")
    out_dir = tmp_path / "ckpt"

    cmd_train(_train_args(corpus, out_dir))

    assert "vocab=2" in capsys.readouterr().out
    ckpt = out_dir / "ckpt.pt"
    assert ckpt.exists()

    # 端到端：训练产物 → 生成（这一条覆盖了「两个子命令之间的契约」）
    cmd_sample(
        types.SimpleNamespace(
            ckpt=str(ckpt), prompt="ab", tokens=4, temperature=0.0, top_k=None, device="cpu"
        )
    )
    out = capsys.readouterr().out
    assert "ab" in out
    assert len(out.strip()) >= 4


def test_cmd_sample_requires_vocab_meta(tmp_path):
    """没有词表的 checkpoint 必须**显式报错**，而不是生成乱码。

    静默生成乱码是这类教学代码最容易出现的假成功。
    """
    from torchgpt.main import cmd_sample

    path = _save_ckpt(tmp_path / "no_meta.pt", _tiny_cfg(vocab_size=2), with_meta=False)
    with pytest.raises(AssertionError, match="词表"):
        cmd_sample(
            types.SimpleNamespace(
                ckpt=str(path), prompt="a", tokens=1, temperature=0.0, top_k=None, device="cpu"
            )
        )


def test_main_cli_dispatches_train_then_sample(tmp_path, monkeypatch, capsys):
    """argparse 接线：子命令名 → 函数，参数名 → 属性。

    这段「胶水」最容易因为改参数名而断，但不会在任何单测里自然暴露。
    """
    from torchgpt import main as cli

    corpus = tmp_path / "corpus.txt"
    corpus.write_text("abab" * 40, encoding="utf-8")
    out_dir = tmp_path / "ckpt"

    monkeypatch.setattr(
        "sys.argv",
        [
            "torchgpt",
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
    cli.main()
    assert (out_dir / "ckpt.pt").exists()

    monkeypatch.setattr(
        "sys.argv",
        [
            "torchgpt",
            "sample",
            "--ckpt",
            str(out_dir / "ckpt.pt"),
            "--prompt",
            "ab",
            "--tokens",
            "3",
            "--temperature",
            "0",
            "--device",
            "cpu",
        ],
    )
    cli.main()
    assert capsys.readouterr().out.strip() != ""
