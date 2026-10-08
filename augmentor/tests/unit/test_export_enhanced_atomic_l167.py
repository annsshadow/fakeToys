# -*- coding: utf-8 -*-
"""L167 / B237：增强导出的九格式共用落盘口收原子写（A182 逐面收路线第九面）。

`EnhancedExporter._export_json` 是 alpaca/sharegpt/chatml/llama/vicuna/belle/
openai/hf/json 九个格式分支共用的落盘 helper——导出物交付面（同 L163 判据）。
与 L163 不同的一点：本边的 `ensure_ascii` / `indent` 是**活选项**（ExportOptions
契约，测试面断言过 indent=4 与 ensure_ascii=False 默认），不能像 L163 那样
直接换紧凑工具——`atomic_write_json` 参数化（可选 `ensure_ascii` / `indent`，
默认紧凑与既有调用零变化），选项原样透传。
"""
import json
import os
import pathlib

import pytest

from augmentor import atomic_write as aw_mod
from augmentor import export_enhanced as ee_mod
from augmentor.atomic_write import atomic_write_json
from augmentor.export_enhanced import EnhancedExporter, ExportOptions


def test_default_write_stays_compact(tmp_path):
    """默认档零变化：紧凑序列化（既有调用面字节兼容）。"""
    p = tmp_path / "a.json"
    atomic_write_json(p, [{"instruction": "q", "output": "a"}])
    assert p.read_bytes() == b'[{"instruction":"q","output":"a"}]'


def test_write_failure_leaves_the_previous_export_intact(tmp_path, monkeypatch):
    """导出写中断：旧文件完好、临时件清掉——旧写法在第一行就截断交付物。"""
    out = tmp_path / "exp.json"
    out.write_text(json.dumps([{"instruction": "旧数据"}]), encoding="utf-8")
    before = out.read_bytes()

    def boom(*args, **kwargs):
        raise RuntimeError("写中断")

    monkeypatch.setattr(ee_mod, "atomic_write_json", boom)
    with pytest.raises(RuntimeError):
        EnhancedExporter().export([{"x": 1}], str(out), ExportOptions())
    monkeypatch.undo()
    assert out.read_bytes() == before
    assert not [p for p in tmp_path.iterdir() if ".tmp-" in p.name]


def test_options_are_passed_through_verbatim(tmp_path):
    """ensure_ascii / indent 是活选项，透传原样（ExportOptions 契约）。"""
    items = [{"instruction": "中"}]
    out4 = tmp_path / "i4.json"
    EnhancedExporter().export(items, str(out4), ExportOptions(indent=4))
    assert '    "instruction"' in out4.read_text(encoding="utf-8")
    out_a = tmp_path / "a.json"
    EnhancedExporter().export(items, str(out_a), ExportOptions(ensure_ascii=True))
    assert "\\u4e2d" in out_a.read_text(encoding="utf-8")


def test_the_export_edge_is_no_longer_an_open_w_plus_dump_pair():
    """静态面：九格式共用落盘口不再命中 l99 棘轮形状。"""
    src = (
        pathlib.Path(__file__).resolve().parents[2] / "augmentor" / "export_enhanced.py"
    ).read_text(encoding="utf-8")
    body = src.split("def _export_json(", 1)[1].split("\n    def ", 1)[0]
    assert "json.dump(" not in body
    assert "atomic_write_json" in body
