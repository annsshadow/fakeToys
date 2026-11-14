# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""L336 波 1b-1 守卫：A296 全值分组 + 回执出声、A345/A371 请求级超时。

零 parametrize，不扰动量面尺子；改动回退到旧形状时下面每一支都会红。
"""

import json
import pathlib

from fastapi.testclient import TestClient


# ------------------------------------------------------------------ A296
def _stratify_receipt(tmp_path, size=3):
    from augmentor.dataset_ops import DatasetOperations, SampleConfig

    items = [{"instruction": "AAAAAAAAAA甲"},
             {"instruction": "AAAAAAAAAA乙"},
             {"instruction": "BBBBBBBBBB丙"}]
    src = tmp_path / "in.json"
    src.write_text(json.dumps(items, ensure_ascii=False), encoding="utf-8")
    out = tmp_path / "out.json"
    receipt = DatasetOperations().sample_file(
        str(src), str(out),
        SampleConfig(method="stratified", size=size, seed=7,
                     stratify_key="instruction"))
    return receipt, json.loads(out.read_text(encoding="utf-8"))


def test_full_value_keys_do_not_collapse_the_ten_char_prefix(tmp_path):
    """改前前 10 字符相同的甲乙塌成一组（2 组），现在是 3 组且回执报真数"""
    receipt, got = _stratify_receipt(tmp_path, size=3)
    assert receipt["stratify_group_count"] == 3, receipt
    assert len(got) == 3
    assert {x["instruction"] for x in got} == {
        "AAAAAAAAAA甲", "AAAAAAAAAA乙", "BBBBBBBBBB丙"}


def test_group_count_is_independent_of_sample_size(tmp_path):
    """反证臂：组数描述输入分布，不随 size 变小而缩水（不是随手回显 output_count）"""
    receipt, got = _stratify_receipt(tmp_path, size=2)
    assert receipt["stratify_group_count"] == 3
    assert receipt["output_count"] == len(got) == 2


def test_non_stratified_sample_reports_zero_groups(tmp_path):
    """反证臂：随机采样没有分组动作，组数必须是 0 而不是编一个数"""
    from augmentor.dataset_ops import DatasetOperations, SampleConfig

    src = tmp_path / "in.json"
    src.write_text(json.dumps([{"instruction": "甲"}, {"instruction": "乙"}]), encoding="utf-8")
    receipt = DatasetOperations().sample_file(
        str(src), str(tmp_path / "out.json"),
        SampleConfig(method="random", size=1, seed=1))
    assert receipt["stratify_group_count"] == 0


# ------------------------------------------------------------------ A345 / A371
def _patch_search(monkeypatch, *, slow, timeout):
    import time

    import api.routes.dataset_tools as tools
    import augmentor.search_enhanced as se

    async def _fake_read(_path):
        return [{"instruction": "甲"}]

    def _search(*_args, **_kwargs):
        if slow:
            time.sleep(0.4)
        return se.SearchResult(items=[], total_matches=0, query_time_ms=0.0,
                               query="甲", method="contains")

    monkeypatch.setattr(tools, "read_json_file", _fake_read)
    monkeypatch.setattr(tools, "resolve_data_path", lambda _n: pathlib.Path("x.json"))
    monkeypatch.setattr(se, "search_dataset", _search)
    monkeypatch.setattr(tools, "SEARCH_TIMEOUT_SECONDS", timeout)
    return tools


def test_slow_regex_search_returns_504_instead_of_holding_a_worker(monkeypatch):
    """请求级超时：慢搜索及时回 504，不再占住线程池到分钟级"""
    from api.main import app

    _patch_search(monkeypatch, slow=True, timeout=0.01)
    resp = TestClient(app).post("/api/dataset/search",
                                json={"input_file": "x.json", "query": "甲"})
    assert resp.status_code == 504
    assert "超时" in resp.json()["detail"]


def test_fast_search_keeps_the_200_receipt(monkeypatch):
    """反证臂：正常搜索仍是 200、键集不变，装超时没把成功面改坏"""
    from api.main import app

    _patch_search(monkeypatch, slow=False, timeout=5.0)
    resp = TestClient(app).post("/api/dataset/search",
                                json={"input_file": "x.json", "query": "甲"})
    assert resp.status_code == 200
    assert resp.json()["query"] == "甲"
    assert "total_matches" in resp.json()


def test_regex_compile_line_is_unchanged_so_re2_was_not_adopted():
    r"""题 4 代拍：不换线性引擎 —— `\d` / `(?i)` 语义不许漂移"""
    src = (pathlib.Path(__file__).resolve().parents[2]
           / "augmentor" / "search_enhanced.py").read_text(encoding="utf-8")
    assert 're.compile(require_search_pattern("query", query), re.IGNORECASE)' in src
    assert "import re2" not in src
