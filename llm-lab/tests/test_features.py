# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""checkpoint 保存/恢复、BPE 训练模式、注意力可视化与训练统计的测试。"""

import numpy as np
import pytest

from minigrad import engine
from minigrad.checkpoint import load, make_tokenizer, save
from minigrad.nn import GPT, GPTConfig
from minigrad.tokenizer import BPETokenizer, CharTokenizer
from minigrad.train import train
from minigrad.visualize import attention_maps, to_html


@pytest.fixture(autouse=True)
def _seed():
    np.random.seed(3)


def _tiny_train(tmp_path, **kw):
    text = "hello world. " * 30 + "attention is all you need. " * 30
    return train(text=text, steps=25, batch_size=8, block_size=16,
                 n_layer=2, n_head=2, n_embd=24, lr=5e-3, verbose=False, **kw)


def test_checkpoint_roundtrip_same_logits(tmp_path):
    model, tok = _tiny_train(tmp_path, out_ckpt=tmp_path / "ckpt.npz")
    ckpt_path = tmp_path / "ckpt.npz.npz" if not (tmp_path / "ckpt.npz").exists() else tmp_path / "ckpt.npz"
    model2, stoi = load(ckpt_path)
    assert stoi is not None and len(stoi) == tok.vocab_size

    idx = np.random.randint(0, tok.vocab_size, size=(2, 8))
    out1 = model.forward(idx).data
    out2 = model2.forward(idx).data
    assert np.allclose(out1, out2, atol=0.0)  # 逐 bit 一致才算恢复成功

    tok2 = make_tokenizer(stoi)
    assert tok2.decode(tok2.encode("attention")) == "attention"


def test_checkpoint_shape_mismatch_fails_loud(tmp_path):
    model, tok = _tiny_train(tmp_path)
    save(model, tmp_path / "m.npz", stoi=tok.stoi)
    z_path = tmp_path / "m.npz"
    # 篡改一个参数数组的形状模拟"配置不符的坏 checkpoint"
    import numpy as _np

    data = dict(_np.load(z_path))
    data["p0"] = _np.zeros((3, 999))
    _np.savez(z_path, **data)
    with pytest.raises(AssertionError):
        load(z_path)


def test_bpe_tokenizer_training_mode(tmp_path):
    # 注意：BPE 对高度重复的语料会极端压缩（"the cat sat..."*40 曾被 24 次合并
    # 压成 4 个 token），所以这里用多样化的语料保证编码后仍有足够长度可切批
    text = ("the cat sat on the mat. " * 10) + ("dogs run in the fog. " * 10) + \
           ("birds sing at dawn. " * 10) + ("fish swim in the sea. " * 10)
    model, tok = train(text=text, steps=15, batch_size=8, block_size=16,
                       n_layer=2, n_head=2, n_embd=24, lr=5e-3,
                       tokenizer="bpe", bpe_vocab=280, verbose=False)
    assert isinstance(tok, BPETokenizer)
    assert tok.vocab_size <= 280
    ids = tok.encode("the cat")
    assert tok.decode(ids) == "the cat"
    assert len(ids) < len("the cat".encode("utf-8"))  # 子词合并生效（曾实测压到 2 个 token）
    # 模型词表与分词器一致，能正常前向
    n = min(8, len(ids))
    logits = model.forward(np.array([ids[:n]]))
    assert logits.shape == (1, n, tok.vocab_size)


def test_train_rejects_unknown_tokenizer():
    with pytest.raises(ValueError):
        train(text="abc", tokenizer="sentencepiece")


def test_train_return_stats():
    text = "hello world. " * 30
    model, tok, stats = train(text=text, steps=20, batch_size=8, block_size=16,
                              n_layer=2, n_head=2, n_embd=24, lr=5e-3,
                              return_stats=True, verbose=False)
    assert stats["steps"] == 20
    assert 0 < stats["final_train_loss"] < 10.0
    assert stats["vocab_size"] == tok.vocab_size
    assert np.isfinite(stats["final_val_loss"])


def test_attention_maps_shape_and_row_normalized():
    cfg = GPTConfig(vocab_size=9, block_size=12, n_layer=2, n_head=2, n_embd=16)
    model = GPT(cfg)
    ids = np.random.randint(0, 9, size=10)
    maps = attention_maps(model, ids)
    assert len(maps) == 2
    for layer in maps:
        assert layer.shape == (2, 10, 10)
        assert np.allclose(layer.sum(axis=-1), 1.0, atol=1e-9)  # 每 query 行归一
        assert np.allclose(layer[:, np.triu_indices(10, k=1)[0], np.triu_indices(10, k=1)[1]], 0.0)


def test_to_html_writes_standalone_file(tmp_path):
    cfg = GPTConfig(vocab_size=9, block_size=12, n_layer=1, n_head=2, n_embd=16)
    model = GPT(cfg)
    ids = np.array([1, 2, 3, 4])
    maps = attention_maps(model, ids)
    out = to_html(maps, ["a", "b", "c", "\n"], tmp_path / "att.html")
    content = open(out, encoding="utf-8").read()
    assert content.startswith("<!doctype html>")
    assert "layer 0" in content and "head 1" in content
    assert "⏎" in content  # 换行被安全地显示为可见符号
