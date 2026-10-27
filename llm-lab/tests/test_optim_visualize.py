# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""minigrad 优化器与注意力可视化的行为测试。

为什么补这两块：
* ``minigrad/optim.py`` 覆盖 71% —— 缺的正是 **momentum 与 AdamW 的更新公式本身**。
  这类代码错了不会抛异常，只会让训练慢一点、效果差一点，是最需要「数值断言」
  而不是「跑通断言」的地方。
* ``minigrad/visualize.py`` 覆盖 52% —— 它是文档里「看见注意力」的入口，
  但产出是 HTML 字符串，没有任何结构断言，改坏了只能靠人眼打开浏览器。

断言方式：用手算得到的闭式结果对拍，而不是「更新后参数变了」。
"""

from __future__ import annotations

import numpy as np
import pytest

from minigrad import checkpoint
from minigrad.engine import Tensor
from minigrad.nn import GPT, GPTConfig
from minigrad.optim import SGD, AdamW
from minigrad.tokenizer import CharTokenizer


# ───────────────────────────────────────────────────────────────────── SGD


def test_sgd_without_momentum_is_plain_gradient_descent():
    """momentum=0 时更新必须是 p -= lr * grad（逐元素、符号正确）。"""
    p = Tensor(np.array([1.0, -2.0, 0.0]))
    p.grad = np.array([0.5, -0.25, 1.0])
    opt = SGD([p], lr=0.1)

    opt.step()

    assert np.allclose(p.data, [1.0 - 0.05, -2.0 + 0.025, -0.1])


def test_sgd_momentum_accumulates_velocity_across_steps():
    """带动量时第二步的步长必须是 (m*g + g)，而不是 g —— 这是动量的全部意义。"""
    p = Tensor(np.array([0.0]))
    p.grad = np.array([1.0])
    opt = SGD([p], lr=0.1, momentum=0.9)

    opt.step()  # v = 1        → p = -0.1
    assert p.data[0] == pytest.approx(-0.1)
    opt.step()  # v = 0.9*1+1  → p = -0.1 - 0.19
    assert p.data[0] == pytest.approx(-0.29)
    opt.step()  # v = 0.9*1.9+1 = 2.71 → p = -0.29 - 0.271
    assert p.data[0] == pytest.approx(-0.561)


def test_sgd_zero_grad_clears_every_parameter():
    """zero_grad 必须把所有参数梯度清零，否则梯度会跨 step 静默累积。"""
    params = [Tensor(np.zeros(2)), Tensor(np.zeros(3))]
    for p in params:
        p.grad = np.ones_like(p.data)
    opt = SGD(params, lr=0.1)

    opt.zero_grad()

    assert all(np.all(p.grad == 0) for p in params)
    assert all(p.grad.shape == p.data.shape for p in params)


def test_sgd_does_not_share_velocity_between_params():
    """每个参数要有独立的动量槽（用同一个数组会让所有参数同步，训练发散）。"""
    a, b = Tensor(np.zeros(1)), Tensor(np.zeros(1))
    a.grad, b.grad = np.array([1.0]), np.array([1.0])
    opt = SGD([a, b], lr=0.1, momentum=0.5)

    opt.step()
    opt.step()

    assert opt.velocity[0] is not opt.velocity[1]
    assert np.allclose(a.data, b.data)  # 同梯度同更新，但槽是两份


# ─────────────────────────────────────────────────────────────────── AdamW


def test_adamw_first_step_is_about_lr_when_decay_disabled():
    """AdamW 第一步的更新量 ≈ lr * sign(g)（偏差校正把 m/v 归一到 ±1）。

    这条是 Adam 系优化器最容易写错的地方：漏掉 ``1 - beta**t`` 校正后，
    第一步步长会只有 lr 的 1/10，训练「看起来能跑」但慢十倍。
    """
    p = Tensor(np.array([5.0]))
    p.grad = np.array([1.0])
    opt = AdamW([p], lr=0.1, weight_decay=0.0)

    opt.step()

    assert p.data[0] == pytest.approx(5.0 - 0.1, abs=1e-6)
    assert opt.t == 1
    # 内部状态也要符合定义，否则「第二步开始」的校正会错
    assert opt.m[0][0] == pytest.approx(0.1)  # (1-0.9)*1
    assert opt.v[0][0] == pytest.approx(0.05)  # (1-0.95)*1


def test_adamw_weight_decay_is_decoupled_from_gradient():
    """零梯度时权重仍按 p *= (1 - lr*wd) 衰减 —— 解耦衰减的直接判据。

    如果实现把 wd 混进梯度（L2 正则），零梯度下参数不会动，这条就会红。
    """
    p = Tensor(np.array([2.0]))
    p.grad = np.array([0.0])
    opt = AdamW([p], lr=0.1, weight_decay=0.5)

    opt.step()

    assert p.data[0] == pytest.approx(2.0 * (1 - 0.1 * 0.5))


def test_adamw_direction_follows_gradient_sign():
    """更新方向必须与梯度反号（优化器不是随机游走）。"""
    up, down = Tensor(np.array([0.0])), Tensor(np.array([0.0]))
    up.grad, down.grad = np.array([1.0]), np.array([-1.0])
    opt = AdamW([up, down], lr=0.01, weight_decay=0.0)

    opt.step()

    assert up.data[0] < 0 < down.data[0]
    assert up.data[0] == pytest.approx(-down.data[0], abs=1e-9)


def test_adamw_zero_grad_and_step_counter():
    """zero_grad 清梯度；t 单调递增（偏差校正依赖它）。"""
    p = Tensor(np.array([1.0]))
    p.grad = np.array([1.0])
    opt = AdamW([p], lr=0.1)

    opt.zero_grad()
    assert np.all(p.grad == 0)
    for _ in range(3):
        opt.step()
    assert opt.t == 3


# ─────────────────────────────────────────────────────────────── visualize


def _tiny_model(vocab=8, block=12, n_layer=2, n_head=2):
    return GPT(GPTConfig(vocab_size=vocab, block_size=block, n_layer=n_layer, n_head=n_head, n_embd=8))


def test_attention_maps_shape_and_causal_mask():
    """每层一张 (n_head, T, T) 图，且因果 mask 保证上三角恒为 0。

    上三角非零意味着「看到了未来 token」——训练 loss 会异常漂亮，
    上线却完全不能自回归生成。这是必须结构性钉住的约束。
    """
    from minigrad.visualize import attention_maps

    model = _tiny_model()
    ids = np.arange(6) % 8
    maps = attention_maps(model, ids)

    assert len(maps) == model.cfg.n_layer
    for m in maps:
        assert m.shape == (model.cfg.n_head, 6, 6)
        assert np.allclose(m.sum(axis=-1), 1.0, atol=1e-6), "每行不是概率分布"
        assert np.allclose(np.triu(m, k=1), 0.0), "因果 mask 泄漏了未来 token"


def test_attention_maps_rejects_two_dimensional_input():
    """接口约定：只接受一维 token 序列，二维要**显式**失败而不是猜。"""
    from minigrad.visualize import attention_maps

    with pytest.raises(AssertionError, match="一维"):
        attention_maps(_tiny_model(), np.zeros((2, 3), dtype=int))


def test_display_marks_whitespace_and_escapes_html():
    """可视化标签的三种特殊处理：换行、空串、HTML 转义。

    不转义的话，语料里的 ``<`` 会把表格结构冲掉，HTML 直接坏掉。
    """
    from minigrad.visualize import _display

    assert _display("\n") == "⏎"
    assert _display("") == "␠"
    assert _display("<b>") == "&lt;b&gt;"
    assert _display("a") == "a"


def _maps(n_layer=1, n_head=1, T=4):
    return [np.full((n_head, T, T), 1.0 / T) for _ in range(n_layer)]


def test_to_html_small_t_embeds_weight_tooltips(tmp_path):
    """T<=16 时每个格子带 title 提示（可交互阅读），且文件自包含无外部依赖。"""
    from minigrad.visualize import to_html

    path = tmp_path / "att.html"
    to_html(_maps(T=4), list("abcd"), path, title="demo")

    html = path.read_text(encoding="utf-8")
    assert html.startswith("<!doctype html>")
    assert "<title>demo</title>" in html
    assert "title='q=a k=b w=0.250'" in html
    assert "layer 0" in html and "head 0" in html
    # 自包含：没有 <script src> / <link rel=stylesheet>
    assert "<script" not in html and "<link" not in html


def test_to_html_large_t_drops_tooltips_but_keeps_cells(tmp_path):
    """T>16 时省掉 title（否则 HTML 体积按 T² 爆炸），但格子数不变。"""
    from minigrad.visualize import to_html

    tokens = [chr(ord("a") + i % 26) for i in range(20)]
    path = tmp_path / "big.html"
    to_html(_maps(T=20), tokens, path)

    html = path.read_text(encoding="utf-8")
    assert "title='q=" not in html
    assert html.count("<td") >= 20 * 20  # 每个 (q,k) 一个格子


def test_to_html_escapes_title(tmp_path):
    """标题来自用户输入，必须转义，否则可以注入标签。"""
    from minigrad.visualize import to_html

    path = tmp_path / "esc.html"
    to_html(_maps(T=2), list("ab"), path, title="<script>x</script>")

    html = path.read_text(encoding="utf-8")
    assert "<script>x</script>" not in html
    assert "&lt;script&gt;" in html


def test_demo_trains_on_given_text_and_writes_html(tmp_path, monkeypatch):
    """demo 的无 checkpoint 分支：现场训练一个小模型再出图。

    训练本身很慢（150 步），这里替换 ``minigrad.train.train`` 只保留
    demo 自己的职责：选句子 → 编码 → 取注意力 → 落 HTML。
    """
    from minigrad import visualize

    model = _tiny_model()
    tok = CharTokenizer("hello\nworld")
    monkeypatch.setattr("minigrad.train.train", lambda **kw: (model, tok))

    out = tmp_path / "demo.html"
    path = visualize.demo(text="hello\nworld", out_path=str(out))

    assert path == str(out.resolve()) or path == str(out)
    assert out.exists()
    html = out.read_text(encoding="utf-8")
    # 标题里的引号会被 html.escape 转成 &#x27;，所以只断言可读前缀与句子本身
    assert "attention ·" in html and "hello" in html


def test_demo_from_checkpoint_uses_stored_vocab(tmp_path):
    """有 checkpoint 时不训练，直接用存的词表解码 —— 生成可复现的前提。"""
    from minigrad import visualize

    model = _tiny_model()
    tok = CharTokenizer("hello\nworld")
    ckpt = tmp_path / "m.npz"
    checkpoint.save(model, ckpt, stoi=tok.stoi)

    out = tmp_path / "from_ckpt.html"
    visualize.demo(ckpt=str(ckpt), text="hello\nworld", out_path=str(out), sample_text="he")

    html = out.read_text(encoding="utf-8")
    assert out.exists()
    assert "attention ·" in html and "he" in html


def test_demo_rejects_checkpoint_without_vocab(tmp_path):
    """没有词表的 checkpoint 无法解码 token，必须报错而不是画一张乱码图。"""
    from minigrad import visualize

    ckpt = tmp_path / "no_vocab.npz"
    checkpoint.save(_tiny_model(), ckpt, stoi=None)

    with pytest.raises(ValueError, match="词表"):
        visualize.demo(ckpt=str(ckpt), text="hello", out_path=str(tmp_path / "x.html"))
