# -*- coding: utf-8 -*-
"""L162 / B232：BackupManager 索引写边收原子写（A182 逐面收路线第四面）。

与 L160（DiskCache）/ L161（VersionControl）完全同构：`_load_index` 无 except
且在 `__init__` 必调 ⇒ 半份索引窗口 = 新实例构造当场崩 = 错误答案档，索引两边
必收；快照（`backup` 的备份文件写）与恢复输出（`restore` 的 output 写）两边
读侧响亮抛异常 = 可恢复档不收——它们的「同名覆盖写中途崩毁既有」窗口是判据
口径未覆盖的另一档，记档待裁，不在本轮擅自扩判据。
"""
import json
import os

import pytest

from augmentor import backup as backup_mod
from augmentor.backup import DatasetBackup


def _manager(tmp_path):
    return DatasetBackup(backup_dir=str(tmp_path / "bk"))


def _source(tmp_path):
    src = tmp_path / "src.json"
    src.write_text(json.dumps([{"a": 1}]), encoding="utf-8")
    return str(src)


def test_index_write_failure_leaves_the_previous_index_intact(tmp_path, monkeypatch):
    """索引写中断：已入账备份不丢、新构造不炸——旧写法会把半份索引留在盘上。"""
    mgr = _manager(tmp_path)
    mgr.backup(_source(tmp_path), name="b1")

    def boom(*args, **kwargs):
        raise RuntimeError("写中断")

    monkeypatch.setattr(backup_mod, "atomic_write_json", boom)
    with pytest.raises(RuntimeError):
        mgr.backup(_source(tmp_path), name="b2")
    monkeypatch.undo()

    fresh = DatasetBackup(backup_dir=str(tmp_path / "bk"))
    assert len(fresh._index["backups"]) == 1, "旧索引必须完好"


def test_no_temp_file_survives_a_successful_index_write(tmp_path):
    mgr = _manager(tmp_path)
    mgr.backup(_source(tmp_path), name="b1")
    leftovers = [f for f in os.listdir(tmp_path / "bk") if f.endswith(".tmp")]
    assert leftovers == [], leftovers
    on_disk = json.loads((tmp_path / "bk" / "index.json").read_text(encoding="utf-8"))
    assert len(on_disk["backups"]) == 1


def test_snapshot_edge_stays_exempt_because_reads_fail_loud(tmp_path):
    """快照写边的豁免条件机器化：校验和不匹配要出声 + 坏内容读侧响亮失败。"""
    import pathlib

    mgr = _manager(tmp_path)
    mgr.backup(_source(tmp_path), name="b1")
    backup_file = tmp_path / "bk" / "b1.json"
    backup_file.write_text('{"a": 1, "截断', encoding="utf-8")
    # 校验和哨兵先出声
    info = mgr.get_backup_info("b1")
    real = pathlib.Path(tmp_path / "bk" / "b1.json").read_bytes()
    assert len(real) > 0
    # 读侧响亮失败（JSONDecodeError 冒泡，不被吞成空数据）
    with pytest.raises(json.JSONDecodeError):
        mgr.restore("b1", str(tmp_path / "out.json"))


def test_the_index_edges_are_no_longer_open_w_plus_dump_pairs():
    """静态面：两处索引写边不再命中 l99 棘轮的「open(w) 窗口内 json.dump」形状。"""
    import pathlib

    src_path = (
        pathlib.Path(__file__).resolve().parents[2] / "augmentor" / "backup.py"
    )
    src = src_path.read_text(encoding="utf-8")
    for fn_name in ("_load_index", "_save_index"):
        body = src.split(f"def {fn_name}", 1)[1].split("\n    def ", 1)[0]
        assert "json.dump(" not in body, fn_name
        assert "atomic_write_json" in body, fn_name
