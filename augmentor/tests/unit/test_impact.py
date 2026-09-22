# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""ImpactEvaluator 单元测试

增强收益必须可量化：规模、多样性、去重、长度分布四个维度都要有增益口径。
"""

import pytest

from augmentor.impact import (
    AugmentationImpact,
    AugmentationMetrics,
    ImpactEvaluator,
    evaluate_augmentation,
)


@pytest.fixture
def before():
    return [
        {"instruction": "如何申请租房？", "output": "登录官网"},
        {"instruction": "租房多少钱？", "output": "按房型定价"},
    ]


@pytest.fixture
def after():
    return [
        {"instruction": "如何申请租房？", "output": "登录官网"},
        {"instruction": "租房多少钱？", "output": "按房型定价"},
        {"instruction": "如何退租押金？", "output": "满一年后退还"},
        {"instruction": "可以申请月付吗？", "output": "支持月付与季付"},
    ]


class TestMeasure:
    def test_empty_items(self):
        metrics = ImpactEvaluator().measure([])
        assert metrics.total_items == 0
        assert metrics.avg_length == 0.0
        assert metrics.duplicate_rate == 0.0

    def test_counts_and_uniques(self, before):
        metrics = ImpactEvaluator().measure(before)
        assert metrics.total_items == 2
        assert metrics.unique_instructions == 2

    def test_duplicate_rate(self):
        items = [
            {"instruction": "a"},
            {"instruction": "a"},
            {"instruction": "b"},
        ]
        metrics = ImpactEvaluator().measure(items)
        # 2 条 a 重复 + 0 条 b -> 2/3
        assert metrics.duplicate_rate == pytest.approx(2 / 3)

    def test_length_statistics(self):
        items = [
            {"instruction": "ab"},
            {"instruction": "abcdef"},
        ]
        metrics = ImpactEvaluator().measure(items)
        assert metrics.avg_length == pytest.approx(4.0)
        assert metrics.length_std == pytest.approx(2.0)

    def test_custom_text_field(self):
        items = [{"output": "abc", "instruction": "x" * 20}]
        metrics = ImpactEvaluator(text_field="output").measure(items)
        assert metrics.avg_length == pytest.approx(3.0)

    def test_metrics_to_dict(self, before):
        d = ImpactEvaluator().measure(before).to_dict()
        for key in ("total_items", "unique_instructions", "avg_length",
                    "length_std", "duplicate_rate", "extra"):
            assert key in d


class TestEvaluate:
    def test_scale_gain(self, before, after):
        impact = ImpactEvaluator().evaluate(before, after)
        # 2 -> 4 条，翻倍
        assert impact.gains["scale_gain"] == pytest.approx(1.0)

    def test_diversity_gain(self, before, after):
        impact = ImpactEvaluator().evaluate(before, after)
        assert impact.gains["diversity_gain"] == pytest.approx(1.0)

    def test_dedup_gain_positive_when_cleaner(self):
        before = [
            {"instruction": "a"},
            {"instruction": "a"},
            {"instruction": "b"},
        ]
        after = [
            {"instruction": "a"},
            {"instruction": "b"},
            {"instruction": "c"},
        ]
        impact = ImpactEvaluator().evaluate(before, after)
        assert impact.gains["dedup_gain"] > 0

    def test_empty_before_gains_zeroed(self, after):
        impact = ImpactEvaluator().evaluate([], after)
        assert impact.gains["scale_gain"] == 0.0
        assert impact.gains["diversity_gain"] == 0.0

    def test_impact_to_dict(self, before, after):
        d = ImpactEvaluator().evaluate(before, after).to_dict()
        assert "before" in d
        assert "after" in d
        assert "gains" in d

    def test_length_spread_gain(self):
        before = [{"instruction": "中等问题"}]
        after = [
            {"instruction": "短"},
            {"instruction": "这是一个非常长的问题文本内容啊哈"},
        ]
        impact = ImpactEvaluator().evaluate(before, after)
        assert impact.gains["length_spread_gain"] >= 0


class TestIsBeneficial:
    def test_positive_scale_is_beneficial(self, before, after):
        impact = ImpactEvaluator().evaluate(before, after)
        assert ImpactEvaluator().is_beneficial(impact) is True

    def test_shrinkage_not_beneficial(self):
        before = [
            {"instruction": "如何申请租房？"},
            {"instruction": "租房多少钱？"},
            {"instruction": "如何退租押金？"},
            {"instruction": "可以申请月付吗？"},
        ]
        after = [
            {"instruction": "如何申请租房？"},
            {"instruction": "租房多少钱？"},
        ]
        impact = ImpactEvaluator().evaluate(before, after)
        assert ImpactEvaluator().is_beneficial(impact) is False

    def test_min_scale_gain_enforced(self, before, after):
        impact = ImpactEvaluator().evaluate(before, after)
        assert ImpactEvaluator().is_beneficial(impact, min_scale_gain=2.0) is False

    def test_worse_duplicates_fail(self):
        before = [{"instruction": "unique text " + str(i)} for i in range(5)]
        after = [
            {"instruction": "dup"},
            {"instruction": "dup"},
            {"instruction": "dup"},
        ]
        impact = ImpactEvaluator().evaluate(before, after)
        assert ImpactEvaluator().is_beneficial(impact) is False


class TestConvenience:
    def test_evaluate_augmentation_returns_dict(self, before, after):
        result = evaluate_augmentation(before, after)
        assert "gains" in result
        assert result["before"]["total_items"] == 2
        assert result["after"]["total_items"] == 4

    def test_evaluate_augmentation_custom_field(self, before, after):
        result = evaluate_augmentation(before, after, text_field="output")
        assert result["after"]["total_items"] == 4
