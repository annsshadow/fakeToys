# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""DataAggregator 单元测试

聚合策略是数据融合的核心，去重/交集/一致性口径必须稳定可追溯。
"""

import pytest

from augmentor.aggregator import (
    DataAggregator,
    AggregationResult,
    aggregate_datasets,
)


@pytest.fixture
def datasets():
    return {
        "a": [
            {"instruction": "q1", "output": "a1"},
            {"instruction": "q2", "output": "a2"},
        ],
        "b": [
            {"instruction": "q2", "output": "a2"},
            {"instruction": "q3", "output": "a3"},
        ],
    }


class TestUnion:
    def test_union_merges_and_dedups(self, datasets):
        result = DataAggregator().aggregate(datasets, "union")
        instructions = [item["instruction"] for item in result.aggregated]
        assert instructions == ["q1", "q2", "q3"]
        assert result.removed_duplicates == 1
        assert result.total_count == 3

    def test_union_source_counts(self, datasets):
        result = DataAggregator().aggregate(datasets, "union")
        assert result.source_counts == {"a": 2, "b": 2}

    def test_union_first_source_wins(self, datasets):
        result = DataAggregator().aggregate(datasets, "union")
        q2 = [item for item in result.aggregated if item["instruction"] == "q2"]
        assert len(q2) == 1

    def test_union_empty_datasets(self):
        result = DataAggregator().aggregate({}, "union")
        assert result.aggregated == []
        assert result.source_counts == {}


class TestIntersection:
    def test_intersection_only_common_keys(self, datasets):
        result = DataAggregator().aggregate(datasets, "intersection")
        assert [item["instruction"] for item in result.aggregated] == ["q2"]

    def test_intersection_no_overlap(self):
        data = {
            "a": [{"instruction": "q1"}],
            "b": [{"instruction": "q2"}],
        }
        result = DataAggregator().aggregate(data, "intersection")
        assert result.aggregated == []

    def test_intersection_empty_sources(self):
        result = DataAggregator().aggregate({"a": []}, "intersection")
        assert result.aggregated == []

    def test_intersection_no_datasets(self):
        result = DataAggregator().aggregate({}, "intersection")
        assert result.aggregated == []


class TestWeighted:
    def test_weighted_quota_distribution(self):
        data = {
            "a": [{"instruction": f"a{i}"} for i in range(20)],
            "b": [{"instruction": f"b{i}"} for i in range(20)],
        }
        aggregator = DataAggregator(random_state=42)
        result = aggregator.aggregate(
            data, "weighted",
            weights={"a": 3.0, "b": 1.0},
            target_size=20,
        )
        # a 占 75% -> 15, b 占 25% -> 5
        a_count = sum(1 for item in result.aggregated if item["instruction"].startswith("a"))
        b_count = sum(1 for item in result.aggregated if item["instruction"].startswith("b"))
        assert a_count == 15
        assert b_count == 5

    def test_weighted_seed_reproducibility(self):
        data = {
            "s": [{"instruction": f"q{i}"} for i in range(20)],
        }
        r1 = DataAggregator(random_state=7).aggregate(
            data, "weighted", target_size=5)
        r2 = DataAggregator(random_state=7).aggregate(
            data, "weighted", target_size=5)
        assert [i["instruction"] for i in r1.aggregated] == \
               [i["instruction"] for i in r2.aggregated]

    def test_weighted_cross_source_dedup(self, datasets):
        result = DataAggregator(random_state=1).aggregate(
            datasets, "weighted", target_size=10)
        # q2 在 a/b 中都出现，只能保留 1 条
        q2 = [item for item in result.aggregated if item["instruction"] == "q2"]
        assert len(q2) <= 1

    def test_weighted_equal_default_weights(self):
        data = {
            "a": [{"instruction": "x"}],
            "b": [{"instruction": "y"}],
        }
        aggregator = DataAggregator(random_state=0)
        result = aggregator.aggregate(data, "weighted", target_size=100)
        assert len(result.aggregated) == 2


class TestConsistent:
    def test_consistent_keeps_matching_answers(self, datasets):
        result = DataAggregator().aggregate(datasets, "consistent")
        # q2 在两个源中答案一致 -> 保留
        q2 = [item for item in result.aggregated if item["instruction"] == "q2"]
        assert len(q2) == 1
        assert q2[0]["_sources"] == ["a", "b"]

    def test_conflicting_answers_recorded(self):
        data = {
            "a": [{"instruction": "q1", "output": "答案A"}],
            "b": [{"instruction": "q1", "output": "答案B"}],
        }
        result = DataAggregator().aggregate(data, "consistent")
        assert result.aggregated == []
        assert len(result.conflicts) == 1
        assert result.conflicts[0]["values"] == ["答案A", "答案B"]
        assert result.conflicts[0]["sources"] == ["a", "b"]

    def test_unique_keys_always_consistent(self, datasets):
        result = DataAggregator().aggregate(datasets, "consistent")
        # q1 只在 a 中，q3 只在 b 中 -> 都保留
        instructions = {item["instruction"] for item in result.aggregated}
        assert "q1" in instructions
        assert "q3" in instructions


class TestStrategyDispatch:
    def test_unknown_strategy_raises(self, datasets):
        with pytest.raises(ValueError, match="不支持"):
            DataAggregator().aggregate(datasets, "mad")

    def test_aggregation_result_to_dict(self, datasets):
        result = DataAggregator().aggregate(datasets, "union")
        d = result.to_dict()
        assert d["total_count"] == result.total_count
        assert d["removed_duplicates"] == 1
        assert "conflicts" in d

    def test_convenience_function(self, datasets):
        result = aggregate_datasets(datasets, "union")
        assert isinstance(result, AggregationResult)
        assert result.total_count == 3

    def test_custom_key_and_value_fields(self):
        data = {
            "a": [{"question": "q", "answer": "x"}],
            "b": [{"question": "q", "answer": "x"}],
        }
        aggregator = DataAggregator(
            key_fields=["question"], value_fields=["answer"]
        )
        result = aggregator.aggregate(data, "consistent")
        assert len(result.aggregated) == 1
        assert result.conflicts == []
