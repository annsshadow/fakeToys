# -*- coding: utf-8 -*-
"""L163 / B233：导出五边收原子写（A182 逐面收路线第五面）。

`Exporter.export` 的五个写边（JSONL 文本 + LLAMA_FACTORY / ALPACA / SHAREGPT /
CHATML 四个 JSON）是**导出物交付面**：输出落在 `web.data_roots` 白名单目录，
而数据管理端点 `list_data_files` 按 `*.json` 扫同一目录（`sorted(root.glob(...))`）
——文件在写窗口一开始就出现在列表里，预览 / 下载在窗口内读到的就是半份
（`atomic_write` 模块 docstring 点名的读者形状）。读侧把坏 JSON 变成
「文件不是合法 JSON」的错误答案 ⇒ 必收。

工具面：`atomic_write` 新增 `atomic_write_text`（JSONL 是文本写，不能走
JSON 序列化；替换语义 / 临时件命名与 JSON 版逐字同口径——临时件名不匹配
`*.json` 也不匹配 `*.jsonl`，目录扫描不会把半份临时件列出来）。

写盘字节变化记档：四条 JSON 边从 `indent=2` 变为工具的紧凑序列化——
导出物是机器消费的训练数据，读侧只解析不读格式；既有测试面全部用
`json.load` / `json.loads` 解析断言，无格式断言（实证于收边后的全量门禁）。
"""
import json
import os
import pathlib

import pytest

from augmentor import export as export_mod
from augmentor.export import Exporter

_EXPORT = pathlib.Path(__file__).resolve().parents[2] / "augmentor" / "export.py"
_ATOMIC = pathlib.Path(__file__).resolve().parents[2] / "augmentor" / "atomic_write.py"


def _write_out(tmp_path, fmt):
    items = [{"instruction": "q", "output": "a", "input": ""}]
    out = tmp_path / f"out_{fmt}.jsonl" if fmt == "jsonl" else tmp_path / f"out_{fmt}.json"
    Exporter().export(items, str(out), fmt)
    return out, items


@pytest.mark.parametrize("fmt", ["jsonl", "llama_factory", "alpaca", "sharegpt", "chatml"])
def test_export_survives_a_serialization_failure_with_the_old_file_intact(tmp_path, monkeypatch, fmt):
    """写中断（序列化在临时件上抛）：目标保持原样、临时件清掉、异常上抛。

    旧写法 `open('w')` 在第一行就截断目标——这就是导出物交付面的缺陷本体。
    """
    out, _ = _write_out(tmp_path, fmt)
    before = out.read_bytes()

    if fmt == "jsonl":
        def boom(*args, **kwargs):
            raise RuntimeError("写中断")
        monkeypatch.setattr(export_mod, "atomic_write_text", boom)
    else:
        def boom(*args, **kwargs):
            raise RuntimeError("写中断")
        monkeypatch.setattr(export_mod, "atomic_write_json", boom)

    with pytest.raises(RuntimeError):
        Exporter().export([{"x": 1}], str(out), fmt)
    monkeypatch.undo()
    assert out.read_bytes() == before, "旧内容必须完好"
    assert not [p for p in tmp_path.iterdir() if ".tmp-" in p.name], "临时件必须清掉"


@pytest.mark.parametrize("fmt", ["jsonl", "llama_factory", "alpaca", "sharegpt", "chatml"])
def test_export_still_round_trips_and_leaves_no_temp_file(tmp_path, fmt):
    out, _ = _write_out(tmp_path, fmt)
    assert out.exists()
    if fmt == "jsonl":
        rows = [json.loads(l) for l in out.read_text(encoding="utf-8").split("\n") if l.strip()]
        assert rows and all("instruction" in r for r in rows)
    else:
        data = json.loads(out.read_text(encoding="utf-8"))
        assert isinstance(data, list) and data
    assert not [p for p in tmp_path.iterdir() if ".tmp-" in p.name]


def test_atomic_write_text_exists_and_never_names_a_json_like_temp():
    """文本版工具与 JSON 版同语义；临时件名避开 *.json 与 *.jsonl 两种扫描。"""
    src = _ATOMIC.read_text(encoding="utf-8")
    assert "def atomic_write_text" in src
    marker = 'with_name(f".{file_path.name}.tmp-{secrets.token_hex(4)}")'
    assert src.count(marker) == 2, "两个写工具必须共用同一临时件命名口径"


def test_the_export_edges_are_no_longer_open_w_plus_dump_pairs():
    """静态面：export() 的五个写边不再命中 l99 棘轮与截断写形状。"""
    src = _EXPORT.read_text(encoding="utf-8")
    body = src.split("def export(", 1)[1].split("def export_all_formats", 1)[0]
    assert "json.dump(" not in body
    assert body.count("atomic_write_json(output_path, data)") == 4
    assert "atomic_write_text(output_path" in body
