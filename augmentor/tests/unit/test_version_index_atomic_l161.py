# -*- coding: utf-8 -*-
"""L161 / B231：VersionControl 索引写边收原子写（A182 逐面收路线第三面）。

判据口径（L99 / atomic_write 模块明文）下 `DatasetVersionManager` 四处写边的
分判：

- `_load_index` 的默认索引写边与 `_save_index` **收**：读侧 `_load_index`
  无 except 且在 `__init__` 必调——旧写法的半份索引窗口里，任何新构造的
  管理器实例（同目录第二个实例、或写窗口内的下一次构造）被 `JSONDecodeError`
  当场炸掉 = 错误答案档。
- `create_version` 的 data.json / version.json 两边**刻意不收**：读侧
  `get_version_data` 对坏内容响亮抛异常（fail loud），按判据属可恢复代价；
  且版本快照写一次不再改、写后即算 checksum，窗口极小。豁免条件同样机器化
  ——读侧行为变了 ⇒ 本文件红 ⇒ 重新对账。
"""
import json
import os
import tempfile

import pytest

from augmentor import version_control as vc_mod
from augmentor.version_control import DatasetVersionManager


def _manager(tmp_path):
    return DatasetVersionManager(versions_dir=str(tmp_path))


def test_index_write_failure_leaves_the_previous_index_intact(tmp_path, monkeypatch):
    """索引写中断：已入账版本不丢、新构造不炸——旧写法会把半份索引留在盘上。"""
    mgr = _manager(tmp_path)
    mgr.create_version([{"a": 1}], description="t1")

    def boom(*args, **kwargs):
        raise RuntimeError("写中断")

    monkeypatch.setattr(vc_mod, "atomic_write_json", boom)
    with pytest.raises(RuntimeError):
        mgr.create_version([{"b": 2}], description="t2")
    monkeypatch.undo()

    fresh = DatasetVersionManager(versions_dir=str(tmp_path))
    assert len(fresh._index["versions"]) == 1, "旧索引必须完好"
    assert fresh._index["current_version"] == "v1.0.0"


def test_no_temp_file_survives_a_successful_index_write(tmp_path):
    mgr = _manager(tmp_path)
    mgr.create_version([{"a": 1}], description="t1")
    leftovers = [f for f in os.listdir(tmp_path) if f.endswith(".tmp")]
    assert leftovers == [], leftovers
    on_disk = json.loads((tmp_path / "index.json").read_text(encoding="utf-8"))
    assert len(on_disk["versions"]) == 1


def test_snapshot_edges_stay_exempt_because_reads_fail_loud(tmp_path):
    """快照写边的豁免条件机器化：读侧必须响亮失败，静默吞掉 ⇒ 重新对账。"""
    mgr = _manager(tmp_path)
    mgr.create_version([{"a": 1}], description="t1")
    data_file = tmp_path / "v1.0.0" / "data.json"
    data_file.write_text('{"a": 1, "截断', encoding="utf-8")
    with pytest.raises(json.JSONDecodeError):
        mgr.load_version("v1.0.0")


def test_the_index_edges_are_no_longer_open_w_plus_dump_pairs():
    """静态面：两处索引写边不再命中 l99 棘轮的「open(w) 窗口内 json.dump」形状。"""
    import pathlib

    src_path = (
        pathlib.Path(__file__).resolve().parents[2] / "augmentor" / "version_control.py"
    )
    src = src_path.read_text(encoding="utf-8")
    for fn_name in ("_load_index", "_save_index"):
        body = src.split(f"def {fn_name}", 1)[1].split("\n    def ", 1)[0]
        assert "json.dump(" not in body, fn_name
        assert "atomic_write_json" in body, fn_name
