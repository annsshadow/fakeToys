# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""L334 批次 3a 守卫：A290 合并四档/六键、A293 聚合三档/一份权威回执。

刻意零 `parametrize` —— 这一轮不扰动 l97 / l98 的量面尺子；判据用循环与
行为面写，把产品码改回旧形状这里就红（等价于注入臂）。
"""

import ast
import json
import pathlib

ROOT = pathlib.Path(__file__).resolve().parents[2]
TOOLS = ROOT / "api" / "routes" / "dataset_tools.py"


def _write(path, items):
    path.write_text(json.dumps(items, ensure_ascii=False), encoding="utf-8")


# ------------------------------------------------------------------ A290
def test_merge_response_model_matches_the_six_sdk_keys(tmp_path):
    """六键一一对应：docstring 那句话现在必须是真的"""
    from api.routes.dataset_tools import MergeResponse
    from augmentor.dataset_ops import DatasetOperations, MergeConfig

    src = tmp_path / "a.json"
    _write(src, [{"instruction": "甲"}, {"instruction": "乙"}, {"instruction": "丙"}])
    receipt = DatasetOperations().merge_files(
        [str(src)], str(tmp_path / "out.json"), MergeConfig(deduplicate=False))
    assert set(MergeResponse.model_fields) == set(receipt)


def test_max_items_truncation_is_reported_separately_from_dedup(tmp_path):
    """截断与去重删除分开报：只截断时 removed_duplicates 必须是 0"""
    from augmentor.dataset_ops import DatasetOperations, MergeConfig

    src = tmp_path / "a.json"
    _write(src, [{"instruction": "甲"}, {"instruction": "乙"}, {"instruction": "丙"}])
    receipt = DatasetOperations().merge_files(
        [str(src)], str(tmp_path / "out.json"),
        MergeConfig(deduplicate=False, max_items=2))
    assert receipt["total_output"] == 2
    assert receipt["removed_duplicates"] == 0
    assert receipt["truncated_by_max_items"] == 1


def test_merge_endpoint_forwards_all_four_knobs():
    """端点构造 MergeConfig 时四档都要真传，少一档这里就红"""
    src = TOOLS.read_text(encoding="utf-8")
    tree = ast.parse(src)
    calls = [n for n in ast.walk(tree)
             if isinstance(n, ast.Call) and isinstance(n.func, ast.Name)
             and n.func.id == "MergeConfig"]
    assert len(calls) == 1
    names = {kw.arg for kw in calls[0].keywords}
    assert names == {"deduplicate", "dedup_threshold", "preserve_order", "max_items"}


def test_merge_request_exposes_all_four_knobs_with_sdk_defaults():
    from api.routes.dataset_tools import MergeRequest

    req = MergeRequest(inputs=["a.json"], output_file="out.json")
    assert (req.deduplicate, req.dedup_threshold, req.preserve_order, req.max_items) == (
        True, 0.9, True, None)


# ------------------------------------------------------------------ A293
def test_aggregate_request_exposes_three_knobs():
    from api.routes.dataset_tools import AggregateRequest

    req = AggregateRequest(datasets={"甲": "a.json"}, output_file="out.json")
    assert req.key_fields is None and req.value_fields is None and req.random_state is None


def test_aggregate_response_exposes_the_two_counters():
    from api.routes.dataset_tools import AggregateResponse

    keys = set(AggregateResponse.model_fields)
    assert {"conflict_count", "conflicts_truncated"} <= keys
    assert "conflicts" in keys, "明细仍在，只是被收窄"


def test_aggregate_receipt_is_built_from_to_dict_not_hand_written():
    """回执必须真的调 to_dict()，且截断与计数在同一支里算出来"""
    src = TOOLS.read_text(encoding="utf-8")
    _, _, tail = src.partition("async def dataset_aggregate")
    assert tail, "聚合端点没了 ⇒ 判据要跟着换形状"
    branch = tail.split("@router.post", 1)[0]
    assert "result.to_dict()" in branch
    assert "[:20]" in branch and "conflicts_truncated" in branch


def test_aggregation_result_to_dict_is_the_single_authority():
    """SDK 那五个键是权威：端点的回执键集必须由它加端点自有的几格组成"""
    from augmentor.aggregator import AggregationResult
    from api.routes.dataset_tools import AggregateResponse

    result = AggregationResult(aggregated=[], source_counts={},
                               conflicts=[{"key": "k"}])
    sdk = result.to_dict()
    assert sdk["conflict_count"] == 1 and sdk["conflicts"] == [{"key": "k"}]
    online = set(AggregateResponse.model_fields)
    assert set(sdk) - online == {"total_count"}, "只剩改名那一格是真差异"


def test_weighted_is_reproducible_when_random_state_is_given():
    """A293 的可复现那一半：同一 seed 两次聚合逐字相同"""
    from augmentor.aggregator import DataAggregator

    data = {"甲": [{"instruction": f"q{i}", "output": f"a{i}"} for i in range(20)]}
    first = DataAggregator(random_state=7).aggregate(data, "weighted", target_size=5)
    second = DataAggregator(random_state=7).aggregate(data, "weighted", target_size=5)
    assert [json.dumps(x, sort_keys=True) for x in first.aggregated] == [
        json.dumps(x, sort_keys=True) for x in second.aggregated]
