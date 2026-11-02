# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""第 69 轮：checkpoint 原子写 + 压缩保存。

判据：
1. round-trip：save 后 load 参数/配置/词表逐项一致（savez_compressed 兼容）；
2. 同目录无临时 .npz 残留（原子写成功路径）；
3. 写入中途中断：旧 checkpoint 完好、无半成品临时文件（fail-safe）；
4. 旧版 ``np.savez`` 写出的非压缩文件仍可读（加载兼容）。
"""

import numpy as np
import pytest

from minigrad.checkpoint import load, save
from minigrad.nn import GPT, GPTConfig


def _model():
    cfg = GPTConfig(vocab_size=11, block_size=8, n_layer=1, n_head=2, n_embd=8)
    return GPT(cfg)


def test_save_load_round_trip(tmp_path):
    m = _model()
    for p in m.parameters():
        p.data = np.linspace(0, 1, p.data.size).reshape(p.data.shape)
    stoi = {"a": 1, "b": 2, "c": 3}
    target = tmp_path / "ckpt"  # 不带 .npz，应自动补后缀
    save(m, target, stoi=stoi)

    m2, stoi2 = load(tmp_path / "ckpt.npz")
    for p1, p2 in zip(m.parameters(), m2.parameters()):
        assert p1.shape == p2.shape
        assert np.array_equal(p1.data, p2.data), "参数必须逐项还原"
    assert m2.cfg.n_embd == m.cfg.n_embd
    assert stoi2 == {str(k): int(v) for k, v in stoi.items()}
    # 目录里只有目标文件（原子写的临时文件已落位，不留残留）
    leftovers = [f for f in (tmp_path).iterdir() if f.suffix == ".npz" and f.name != "ckpt.npz"]
    assert leftovers == [], f"目录残留临时 .npz: {leftovers}"


def test_save_writes_compressed_archive(tmp_path):
    # savez_compressed 产出的 zip 成员应为 deflate 压缩（非 store）
    import zipfile

    m = _model()
    save(m, tmp_path / "z.npz")
    with zipfile.ZipFile(tmp_path / "z.npz") as z:
        infos = z.infolist()
        assert infos, "zip 内应有成员"
        # 大数组成员应是压缩存储；小 JSON 元数据可能退化为 store，取任一
        assert any(i.compress_type == zipfile.ZIP_DEFLATED for i in infos), (
            "至少一个成员应 deflate 压缩（savez_compressed 生效）"
        )


def test_interrupted_save_keeps_old_checkpoint(tmp_path, monkeypatch):
    old = _model()
    old_target = tmp_path / "ckpt.npz"
    save(old, old_target)

    def _boom(*args, **kwargs):
        raise RuntimeError("模拟写入中途中断（崩溃/OOM）")

    monkeypatch.setattr("minigrad.checkpoint.np.savez_compressed", _boom)
    with pytest.raises(RuntimeError, match="模拟写入中途中断"):
        save(_model(), tmp_path / "ckpt")  # 目标为 ckpt.npz（自动补后缀）

    # 旧 checkpoint 完好可读（没被半改状态毁掉）
    m2, _ = load(old_target)
    assert m2.cfg.n_embd == old.cfg.n_embd
    # 无半成品临时文件残留
    leftovers = [f for f in tmp_path.iterdir() if f.suffix == ".npz" and f.name != "ckpt.npz"]
    assert leftovers == [], f"中断后不应残留临时文件: {leftovers}"


def test_load_legacy_uncompressed_file(tmp_path):
    # 旧版 np.savez（未压缩）写出的文件必须仍可加载（np.load 透明兼容）
    m = _model()
    arrays = {f"p{i}": p.data for i, p in enumerate(m.parameters())}
    import json

    np.savez(
        tmp_path / "legacy.npz",
        **arrays,
        **{k: np.array(json.dumps(v)) for k, v in {"_cfg": vars(m.cfg)}.items()},
    )
    m2, _ = load(tmp_path / "legacy.npz")
    assert m2.cfg.n_embd == m.cfg.n_embd
