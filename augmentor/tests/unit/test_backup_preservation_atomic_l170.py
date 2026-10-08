# -*- coding: utf-8 -*-
"""L170 / B240：备份快照与恢复输出两边收原子写（A182 逐面收的待裁档裁定）。

L162 记档的「同名覆盖写中途崩毁既有」档在本轮裁定并收边。裁定理由不是扩
判据，而是**语义自洽**：备份/恢复的语义就是数据保全——「备份操作毁掉旧备
份」「恢复动作毁掉用户输出路径上的既有文件」自相矛盾；`os.replace` 前旧内
容完好的性质恰好就是这两边要的保全，工具零成本覆盖。

同轮的 API 裸数值字段普查（18 处）结论：全部链路完整或已有契约——比例三键
（DataSplitter 判据 → DataValidationError → ValueError 子类 → 400 可行动）、
quality/dedup 三 threshold（组件层 L48/L76 判据同样 400 形态）、leakage
fuzzy_threshold 属 A172（待用户拍，不动）、分页/size 杂项无错误答案路径。
"""
import json
import os
import pathlib

import pytest

from augmentor import backup as backup_mod
from augmentor.backup import DatasetBackup

_BK = pathlib.Path(__file__).resolve().parents[2] / "augmentor" / "backup.py"


def _manager(tmp_path):
    src = tmp_path / "src.json"
    src.write_text(json.dumps([{"a": 1}]), encoding="utf-8")
    mgr = DatasetBackup(backup_dir=str(tmp_path / "bk"))
    mgr.backup(str(src), name="b1")
    return mgr, src


def test_backup_same_name_overwrite_failure_keeps_the_old_backup(tmp_path, monkeypatch):
    """同名覆盖写中断：旧备份完好——旧写法在第一行就把它截断。"""
    mgr, src = _manager(tmp_path)
    bk_file = tmp_path / "bk" / "b1.json"
    before = bk_file.read_bytes()

    def boom(*args, **kwargs):
        raise RuntimeError("写中断")

    monkeypatch.setattr(backup_mod, "atomic_write_json", boom)
    with pytest.raises(RuntimeError):
        mgr.backup(str(src), name="b1")
    monkeypatch.undo()
    assert bk_file.read_bytes() == before, "旧备份必须完好"
    assert json.loads(bk_file.read_text(encoding="utf-8")) == [{"a": 1}]


def test_restore_failure_keeps_the_user_output_intact(tmp_path, monkeypatch):
    """恢复写中断：用户输出路径上的既有文件完好（恢复不毁数据）。"""
    mgr, _ = _manager(tmp_path)
    out = tmp_path / "out.json"
    out.write_text("用户既有内容", encoding="utf-8")

    def boom(*args, **kwargs):
        raise RuntimeError("写中断")

    monkeypatch.setattr(backup_mod, "atomic_write_json", boom)
    with pytest.raises(RuntimeError):
        mgr.restore("b1", str(out))
    monkeypatch.undo()
    assert out.read_text(encoding="utf-8") == "用户既有内容"
    assert not [p for p in tmp_path.iterdir() if ".tmp-" in p.name]


def test_both_edges_round_trip(tmp_path):
    mgr, _ = _manager(tmp_path)
    out = tmp_path / "out.json"
    mgr.restore("b1", str(out))
    assert json.loads(out.read_text(encoding="utf-8")) == [{"a": 1}]


def test_the_two_edges_are_no_longer_open_w_plus_dump_pairs():
    """静态面：快照与恢复输出两边不再命中 l99 棘轮形状。"""
    src = _BK.read_text(encoding="utf-8")
    assert src.count("atomic_write_json(") >= 3  # 索引两边（L162）+ 本轮两边
    body_backup = src.split("def backup(", 1)[1].split("\n    def ", 1)[0]
    body_restore = src.split("def restore(", 1)[1].split("\n    def ", 1)[0]
    assert "json.dump(items" not in body_backup
    assert "json.dump(items" not in body_restore
