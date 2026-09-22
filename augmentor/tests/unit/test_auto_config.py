# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""AutoConfig 单元测试

配置推荐规则必须可解释：每个推荐值都要能从输入统计推导出来。
"""

import pytest

from augmentor.auto_config import (
    AutoConfig,
    AutoConfigRecommendation,
    auto_recommend,
    DEDUP_THRESHOLD_RANGE,
    QUALITY_THRESHOLD_RANGE,
)


@pytest.fixture
def healthy_stats():
    return {
        "duplicate_rate": 0.1,
        "quality_pass_rate": 0.9,
        "length_stats": {"avg": 20, "max": 30},
        "language_distribution": {"zh": 10},
    }


class TestInit:
    def test_rejects_inverted_dedup_range(self):
        with pytest.raises(ValueError, match="dedup_range"):
            AutoConfig(dedup_range=(0.9, 0.8))

    def test_rejects_inverted_quality_range(self):
        with pytest.raises(ValueError, match="quality_range"):
            AutoConfig(quality_range=(0.8, 0.5))

    def test_default_ranges(self):
        config = AutoConfig()
        assert config.dedup_range == DEDUP_THRESHOLD_RANGE
        assert config.quality_range == QUALITY_THRESHOLD_RANGE


class TestRecommend:
    def test_rejects_non_positive_total(self):
        with pytest.raises(ValueError, match="total_items"):
            AutoConfig().recommend({}, 0)

    def test_high_duplicate_rate_raises_dedup_threshold(self, healthy_stats):
        stats = dict(healthy_stats, duplicate_rate=0.5)
        rec = AutoConfig().recommend(stats, 100)
        assert rec.dedup_threshold == DEDUP_THRESHOLD_RANGE[1]
        assert any("重复率" in r for r in rec.reasoning)

    def test_low_duplicate_rate_lowers_dedup_threshold(self, healthy_stats):
        stats = dict(healthy_stats, duplicate_rate=0.01)
        rec = AutoConfig().recommend(stats, 100)
        assert rec.dedup_threshold == DEDUP_THRESHOLD_RANGE[0]
        assert any("重复率" in r for r in rec.reasoning)

    def test_moderate_duplicate_rate_linear_interpolation(self, healthy_stats):
        stats = dict(healthy_stats, duplicate_rate=0.175)
        rec = AutoConfig().recommend(stats, 100)
        lo, hi = DEDUP_THRESHOLD_RANGE
        assert lo < rec.dedup_threshold < hi

    def test_low_pass_rate_relaxes_quality_gate(self, healthy_stats):
        stats = dict(healthy_stats, quality_pass_rate=0.2)
        rec = AutoConfig().recommend(stats, 100)
        assert rec.quality_threshold == QUALITY_THRESHOLD_RANGE[0]
        assert any("质量通过率" in r for r in rec.reasoning)

    def test_high_pass_rate_tightens_quality_gate(self, healthy_stats):
        stats = dict(healthy_stats, quality_pass_rate=0.95)
        rec = AutoConfig().recommend(stats, 100)
        assert rec.quality_threshold == QUALITY_THRESHOLD_RANGE[1]

    def test_recommendation_fields(self, healthy_stats):
        rec = AutoConfig().recommend(healthy_stats, 100)
        assert rec.recommended_sample_size >= 1
        assert "duplicate_rate" in rec.source_stats
        d = rec.to_dict()
        assert d["dedup_threshold"] == rec.dedup_threshold
        assert "reasoning" in d

    def test_missing_stats_use_defaults(self):
        rec = AutoConfig().recommend({}, 50)
        assert rec.dedup_threshold in DEDUP_THRESHOLD_RANGE
        assert rec.quality_threshold in QUALITY_THRESHOLD_RANGE


class TestSampleSize:
    def test_uniform_lengths_min_ratio(self):
        stats = {"length_stats": {"avg": 10, "max": 11}}
        min_ratio, max_ratio = AutoConfig().sample_ratio_range
        rec = AutoConfig().recommend(stats, 100)
        # 无语言多样性加分时
        assert rec.recommended_sample_size == int(100 * min_ratio)

    def test_extreme_length_spread_max_ratio(self):
        stats = {"length_stats": {"avg": 10, "max": 200}}
        _, max_ratio = AutoConfig().sample_ratio_range
        rec = AutoConfig().recommend(stats, 100)
        assert rec.recommended_sample_size == int(100 * max_ratio)

    def test_multi_language_bonus(self):
        stats = {
            "length_stats": {"avg": 10, "max": 15},
            "language_distribution": {"zh": 5, "en": 5, "mixed": 2},
        }
        min_ratio, max_ratio = AutoConfig().sample_ratio_range
        rec = AutoConfig().recommend(stats, 200)
        expected = int(200 * min(min_ratio + 0.05, max_ratio))
        assert rec.recommended_sample_size == expected

    def test_sample_size_never_below_one(self):
        rec = AutoConfig().recommend({"length_stats": {}}, 1)
        assert rec.recommended_sample_size >= 1

    def test_zero_avg_length_no_division_error(self):
        rec = AutoConfig().recommend({"length_stats": {"avg": 0, "max": 0}}, 10)
        assert rec.recommended_sample_size >= 1


class TestValidation:
    def test_valid_recommendation_has_no_problems(self, healthy_stats):
        config = AutoConfig()
        rec = config.recommend(healthy_stats, 100)
        assert config.validate_recommendation(rec) == []

    def test_out_of_range_dedup_flagged(self):
        config = AutoConfig()
        rec = AutoConfigRecommendation(
            dedup_threshold=0.99,
            quality_threshold=0.6,
            recommended_sample_size=10,
        )
        problems = config.validate_recommendation(rec)
        assert any("去重阈值" in p for p in problems)

    def test_out_of_range_quality_flagged(self):
        config = AutoConfig()
        rec = AutoConfigRecommendation(
            dedup_threshold=0.9,
            quality_threshold=0.3,
            recommended_sample_size=10,
        )
        problems = config.validate_recommendation(rec)
        assert any("质量阈值" in p for p in problems)

    def test_zero_sample_size_flagged(self):
        config = AutoConfig()
        rec = AutoConfigRecommendation(
            dedup_threshold=0.9,
            quality_threshold=0.6,
            recommended_sample_size=0,
        )
        problems = config.validate_recommendation(rec)
        assert any("采样数量" in p for p in problems)


class TestConvenience:
    def test_auto_recommend_returns_dict(self, healthy_stats):
        result = auto_recommend(healthy_stats, 100)
        assert isinstance(result, dict)
        assert "dedup_threshold" in result
        assert "quality_threshold" in result

    def test_auto_recommend_minimal_input(self):
        result = auto_recommend({}, 10)
        assert result["recommended_sample_size"] >= 1
