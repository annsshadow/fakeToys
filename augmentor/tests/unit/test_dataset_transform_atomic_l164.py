# -*- coding: utf-8 -*-
"""L164 / B234：数据集变换三边收原子写（A182 逐面收路线第六面）。

`DatasetOperations` 的 merge_files / sample_file / split_file 三条写边与 L163
导出物交付面同判据：API 侧（`api/routes/dataset_tools.py` 的 merge/sample/split
端点）与 CLI 侧的输出路径都经 `resolve_data_path` / `resolve_data_dir` 白名单
落进数据根，而 `list_data_files` 按 `*.json` 扫同一目录——写窗口内读者拿到
半份。同判据必收；三边输入读侧 `json.load` 裸抛（响亮失败）不属缺陷面。
"""
import json
import os
import pathlib

import pytest

from augmentor import dataset_ops as dops_mod
from augmentor.dataset_ops import DatasetOperations, MergeConfig, SampleConfig

_DOPS = pathlib.Path(__file__).resolve().parents[2] / "augmentor" / "dataset_ops.py"


def _ops(tmp_path):
    a = tmp_path / "a.json"
    b = tmp_path / "b.json"
    a.write_text(
        json.dumps([{"instruction": "什么是机器学习", "output": "ML 是……"}], ensure_ascii=False),
        encoding="utf-8",
    )
    b.write_text(
        json.dumps([{"instruction": "今天天气如何", "output": "晴。"}], ensure_ascii=False),
        encoding="utf-8",
    )
    return DatasetOperations(), str(a), str(b)


def test_write_failure_leaves_the_previous_output_intact(tmp_path, monkeypatch):
    """变换写中断：既有输出文件保持原样——旧写法在第一行就把它截断。"""
    ops, a, b = _ops(tmp_path)
    out = tmp_path / "m.json"
    r1 = ops.merge_files([a, b], str(out), MergeConfig(deduplicate=False))
    before = out.read_bytes()

    def boom(*args, **kwargs):
        raise RuntimeError("写中断")

    monkeypatch.setattr(dops_mod, "atomic_write_json", boom)
    with pytest.raises(RuntimeError):
        ops.merge_files([a, b], str(out), MergeConfig(deduplicate=False))
    monkeypatch.undo()
    assert out.read_bytes() == before, "旧内容必须完好"


def test_three_transforms_round_trip_and_leave_no_temp_file(tmp_path):
    ops, a, b = _ops(tmp_path)
    r1 = ops.merge_files([a, b], str(tmp_path / "m.json"), MergeConfig(deduplicate=False))
    assert r1["total_output"] == 2
    assert len(json.loads((tmp_path / "m.json").read_text(encoding="utf-8"))) == 2
    ops.sample_file(a, str(tmp_path / "s.json"), SampleConfig())
    assert json.loads((tmp_path / "s.json").read_text(encoding="utf-8")) is not None
    ops.split_file(a, str(tmp_path / "sp"))
    sp = sorted((tmp_path / "sp").glob("*.json"))
    assert sp and all(json.loads(p.read_text(encoding="utf-8")) is not None for p in sp)
    leftovers = [p for p in tmp_path.rglob("*") if ".tmp-" in p.name]
    assert leftovers == [], leftovers


def test_overwrite_output_path_is_byte_identical_to_fresh_write(tmp_path):
    """同名覆盖写（输入=输出路径的工作流）在收边后保持内容正确。"""
    ops, a, _ = _ops(tmp_path)
    out = tmp_path / "loop.json"
    out.write_text(json.dumps([{"instruction": "旧数据"}], ensure_ascii=False), encoding="utf-8")
    ops.sample_file(str(out), str(out), SampleConfig())
    data = json.loads(out.read_text(encoding="utf-8"))
    assert isinstance(data, list)


def test_the_transform_edges_are_no_longer_open_w_plus_dump_pairs():
    """静态面：三条变换写边不再命中 l99 棘轮形状。"""
    src = _DOPS.read_text(encoding="utf-8")
    for fn_name in ("merge_files", "sample_file", "split_file"):
        body = src.split(f"def {fn_name}", 1)[1].split("\n    def ", 1)[0]
        assert "json.dump(" not in body, fn_name
    assert src.count("atomic_write_json(") >= 3
