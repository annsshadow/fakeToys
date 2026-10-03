# -*- coding: utf-8 -*-
"""L165 / B235：依赖登记两边收原子写（A182 逐面收路线第七面）。

`DependencyManager` 的 `_save_datasets` / `_save_dependencies` 与 L160–L162
同构：读侧 `_load_file` 无 except 且 `__init__` 必调——半份登记表的写窗口里，
任何新构造的管理器实例被 `JSONDecodeError` 当场炸掉 = 错误答案档。登记表被
截断的另一症状是「静默回落 default」（若有人顺手给 `_load_file` 加 except）：
已登记的数据集/依赖凭空消失，与 L99 checkpoint 那格同款——两条路都封死，
修法就是把写边收原子。
"""
import json
import os
import pathlib

import pytest

from augmentor import dependency as dep_mod
from augmentor.dependency import DependencyManager

_DEP = pathlib.Path(__file__).resolve().parents[2] / "augmentor" / "dependency.py"


def _manager(tmp_path):
    return DependencyManager(registry_path=str(tmp_path / "reg"))


def test_registry_write_failure_leaves_previous_entries_intact(tmp_path, monkeypatch):
    """登记写中断：已登记条目不丢、新构造不炸——旧写法会把半份登记表留在盘上。"""
    mgr = _manager(tmp_path)
    mgr.register_dataset("ds1", "/fake/path", 10, description="t")

    def boom(*args, **kwargs):
        raise RuntimeError("写中断")

    monkeypatch.setattr(dep_mod, "atomic_write_json", boom)
    with pytest.raises(RuntimeError):
        mgr.register_dataset("ds2", "/fake/path2", 20, description="t2")
    monkeypatch.undo()

    fresh = DependencyManager(registry_path=str(tmp_path / "reg"))
    names = {d.name for d in fresh.list_datasets()} if hasattr(fresh, "list_datasets") else set(fresh._datasets)
    assert "ds1" in names, "旧登记必须完好"
    assert "ds2" not in names, "写失败的登记不得以半份状态入账"


def test_registry_round_trips_and_leaves_no_temp_file(tmp_path):
    mgr = _manager(tmp_path)
    mgr.register_dataset("ds1", "/fake/path", 10, description="t")
    reg_dir = tmp_path / "reg"
    files = sorted(reg_dir.glob("*.json"))
    assert files and all(json.loads(p.read_text(encoding="utf-8")) is not None for p in files)
    leftovers = [p for p in reg_dir.iterdir() if ".tmp-" in p.name]
    assert leftovers == [], leftovers


def test_both_registry_edges_are_no_longer_open_w_plus_dump_pairs():
    """静态面：两条登记写边不再命中 l99 棘轮形状。"""
    src = _DEP.read_text(encoding="utf-8")
    for fn_name in ("_save_datasets", "_save_dependencies"):
        body = src.split(f"def {fn_name}", 1)[1].split("\n    def ", 1)[0]
        assert "json.dump(" not in body, fn_name
        assert "atomic_write_json" in body, fn_name
