# -*- coding: utf-8 -*-
"""L166 / B236：数据集工具路由的落盘口收原子写（A182 逐面收路线第八面）。

`api/routes/dataset_tools.py` 的 `_dump` 是「写盘变换类」端点（convert/merge/
sample/split/aggregate/rag）共用的落盘 helper——产物经白名单校验落进数据根，
`list_data_files` 按 `*.json` 扫同一目录，写窗口内读者拿半份。与 L163/L164
同一判据必收。`data_ops` 命令的 --output 报告边同轮分档：CLI 报告一次性写
（同步等待、任意路径、无扫描链）按 L165 口径豁免。
"""
import json
import os
import pathlib

import pytest

from api.routes.dataset_tools import _dump


def test_dump_survives_a_serialization_failure_with_the_old_file_intact(tmp_path, monkeypatch):
    """落盘失败：既有文件保持原样、临时件清掉——旧写法第一行就截断目标。"""
    out = tmp_path / "out.json"
    out.write_text(json.dumps([{"id": 1}]), encoding="utf-8")
    before = out.read_bytes()

    import api.routes.dataset_tools as tools

    def boom(*args, **kwargs):
        raise RuntimeError("写中断")

    monkeypatch.setattr(tools, "atomic_write_json", boom)
    with pytest.raises(RuntimeError):
        _dump([{"id": 2}], out)
    monkeypatch.undo()
    assert out.read_bytes() == before, "旧内容必须完好"
    assert not [p for p in tmp_path.iterdir() if ".tmp-" in p.name]


def test_dump_round_trips_and_creates_missing_parents(tmp_path):
    out = tmp_path / "nested" / "out.json"
    _dump([{"id": 1}], out)
    assert json.loads(out.read_text(encoding="utf-8")) == [{"id": 1}]
    assert not [p for p in (tmp_path / "nested").iterdir() if ".tmp-" in p.name]


def test_the_dump_helper_is_no_longer_an_open_w_plus_dump_pair():
    """静态面：_dump 不再命中 l99 棘轮形状。"""
    src = (
        pathlib.Path(__file__).resolve().parents[2] / "api" / "routes" / "dataset_tools.py"
    ).read_text(encoding="utf-8")
    body = src.split("def _dump(", 1)[1].split("\ndef ", 1)[0]
    assert "json.dump(" not in body
    assert "atomic_write_json" in body
