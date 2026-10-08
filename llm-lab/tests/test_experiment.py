# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""experiments.scaling_law 的快速版测试（用小模型替身，秒级完成）。"""

import csv

import pytest

import experiments.scaling_law as sl


def test_scaling_run_produces_csv(tmp_path, monkeypatch):
    monkeypatch.setattr(sl, "SIZES", [
        ("T1", dict(n_layer=1, n_head=2, n_embd=24)),
        ("T2", dict(n_layer=2, n_head=2, n_embd=32)),
    ])
    out = tmp_path / "scaling.csv"
    rows = sl.run(steps=12, out_csv=str(out), verbose=False)

    assert len(rows) == 2
    assert rows[1]["params"] > rows[0]["params"]  # 更大的配置参数更多
    for r in rows:
        assert 0.0 < r["final_train_loss"] < 12.0
        assert 0.0 < r["final_val_loss"] < 12.0

    with open(out, encoding="utf-8") as f:
        lines = list(csv.reader(f))
    assert lines[0] == ["size", "params", "final_train_loss", "final_val_loss", "vocab_size"]
    assert len(lines) == 3  # header + 2 rows
