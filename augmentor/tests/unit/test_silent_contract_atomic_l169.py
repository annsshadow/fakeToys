# -*- coding: utf-8 -*-
"""L169 / B239：实验记录与趋势历史两写边收原子写（A182 逐面收路线第十一面）。

这两处与 L160–L168 的「必收」判定路径不同：它们的**读侧契约就是静默的**
（`load_experiment` 损坏 → None、`_save_experiment`「不应崩溃」、
`quality_trend` 走 L146 检疫）——test_tracker.py 三处测试把这套语义钉成
契约。收原子**不翻案**：半份窗口消失后，损坏只剩余磁盘故障一途，读侧
静默路径更难触达、写失败仍被 except 吞住（契约保持）；quality_trend 的
L146 检疫保留为纵深防御（原子写让检疫只留给真正的磁盘故障）。
"""
import json
import os
import pathlib

import pytest

from augmentor import quality_trend as qt_mod
from augmentor import tracker as tracker_mod
from augmentor.quality_trend import QualityTrendTracker
from augmentor.tracker import ExperimentTracker

_TRACKER = pathlib.Path(__file__).resolve().parents[2] / "augmentor" / "tracker.py"
_QT = pathlib.Path(__file__).resolve().parents[2] / "augmentor" / "quality_trend.py"


def test_tracker_write_failure_still_does_not_crash_and_old_file_intact(tmp_path, monkeypatch):
    """「不应崩溃」契约保持 + 原子语义：旧实验文件在写中断后完好。"""
    tracker = ExperimentTracker(storage_dir=str(tmp_path))
    tracker.start_experiment("e1", "v1", "ernie")
    path = tmp_path / "e1.json"
    before = path.read_bytes()

    def boom(*args, **kwargs):
        raise RuntimeError("写中断")

    monkeypatch.setattr(tracker_mod, "atomic_write_json", boom)
    tracker._save_experiment(tracker._current_experiment)  # 不得抛（契约）
    monkeypatch.undo()
    assert path.read_bytes() == before, "旧实验文件必须完好"
    assert not [p for p in tmp_path.iterdir() if ".tmp-" in p.name]


def test_trend_write_failure_still_does_not_crash_and_quarantine_untouched(tmp_path, monkeypatch):
    """趋势写失败不崩（契约）；正常路径不触发检疫标志。"""
    qt = QualityTrendTracker(storage_path=str(tmp_path / "trend.json"))
    qt._trend_history = [{"score": 0.9}]
    qt._save_history()
    assert qt._corrupt_unquarantined is False, "正常写不得触发检疫"
    before = (tmp_path / "trend.json").read_bytes()

    def boom(*args, **kwargs):
        raise RuntimeError("写中断")

    monkeypatch.setattr(qt_mod, "atomic_write_json", boom)
    qt._save_history()  # 不得抛（契约）
    monkeypatch.undo()
    assert (tmp_path / "trend.json").read_bytes() == before
    assert not [p for p in tmp_path.iterdir() if ".tmp-" in p.name]


def test_both_edges_are_no_longer_open_w_plus_dump_pairs():
    """静态面：两条写边不再命中 l99 棘轮形状。"""
    for path, fn in ((_TRACKER, "_save_experiment"), (_QT, "_save_history")):
        src = path.read_text(encoding="utf-8")
        body = src.split(f"def {fn}", 1)[1].split("\n    def ", 1)[0]
        assert "json.dump(" not in body, fn
        assert "atomic_write_json" in body, fn
