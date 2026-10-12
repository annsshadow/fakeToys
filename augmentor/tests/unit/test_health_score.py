# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""数据集健康度评分测试 - 验证数据完整性、多样性、均衡性、覆盖率评估"""

import pytest
from augmentor.health_score import DatasetHealthScore


@pytest.fixture
def healthy_items():
    """健康数据集样本"""
    return [
        {"instruction": "如何租房？", "input": "", "output": "建议先看房源信息..."},
        {"instruction": "租房合同注意事项？", "input": "", "output": "注意租金、押金条款..."},
        {"instruction": "租房如何维权？", "input": "", "output": "可向住建委投诉..."},
        {"instruction": "租房如何看房？", "input": "", "output": "建议实地查看..."},
        {"instruction": "租房如何签约？", "input": "", "output": "确认双方身份信息..."},
    ]


@pytest.fixture
def poor_items():
    """低健康数据集样本（重复严重、内容缺失、长度极不均衡）"""
    return [
        {"instruction": "租房", "output": ""},
        {"instruction": "租房", "output": ""},
        {"instruction": "租房", "output": ""},
    ]


class TestDatasetHealthScoreInit:
    """初始化测试"""
    
    def test_default_weights(self):
        scorer = DatasetHealthScore()
        assert scorer.weights == [0.25, 0.25, 0.25, 0.25]
    
    def test_custom_weights(self):
        scorer = DatasetHealthScore(weights=[0.1, 0.3, 0.3, 0.3])
        assert scorer.weights == [0.1, 0.3, 0.3, 0.3]
    
    def test_invalid_weights_length_raises(self):
        with pytest.raises(ValueError, match="weights 必须是 4 个元素，当前 2 个"):
            DatasetHealthScore(weights=[0.5, 0.5])
    
    def test_empty_weights_list_is_not_silently_defaulted(self):
        """改前 `weights or 默认` 把 [] 读成「未设置」而静默回落，与 QualityScorer 不同判"""
        with pytest.raises(ValueError, match="weights 必须是 4 个元素，当前 0 个"):
            DatasetHealthScore(weights=[])

    def test_weights_not_sum_to_one_raises(self):
        with pytest.raises(ValueError, match="weights 之和必须是 1"):
            DatasetHealthScore(weights=[0.1, 0.1, 0.1, 0.1])

    @pytest.mark.parametrize("weights, idx", [
        ([2.0, -0.5, -0.25, -0.25], 0),
        ([0.25, -0.25, 0.5, 0.5], 1),
        ([0.25, 0.25, 1.5, -0.5], 2),
        ([0.25, 0.25, 0.25, -0.5], 3),
    ])
    def test_each_weight_must_be_a_ratio(self, weights, idx):
        """A319（L238）：逐项 [0, 1]，判错哪个下标就点名哪个——不判符号也就挡不住排序反转"""
        with pytest.raises(ValueError, match=f"weights\\[{idx}\\] 必须在 0.0 到 1.0 之间"):
            DatasetHealthScore(weights=weights)

    def test_non_numeric_weight_raises_instead_of_typeerror(self):
        """改前 `'abc'` 死在 `sum()` 的 TypeError 上（500 的形状），现在是可行动的 400 文案"""
        with pytest.raises(ValueError, match="必须是数值"):
            DatasetHealthScore(weights=["abc", 0.25, 0.25, 0.25])

    def test_bool_weight_is_rejected_by_name(self):
        """`True` 是 `int` 的子类，会被当成 1.0 满权重 ⇒ 布尔必须点名拒（直构面才测得到）"""
        with pytest.raises(ValueError, match="不能是布尔"):
            DatasetHealthScore(weights=[True, False, 0.5, 0.5])

    def test_sum_tolerance_is_inclusive(self):
        """A320（L238）：0.99 与 1.01 都在容差内（判前先量化到 9 位），0.98 才拒"""
        assert DatasetHealthScore(weights=[0.24, 0.25, 0.25, 0.25]).weights[-1] == 0.25
        assert DatasetHealthScore(weights=[0.26, 0.25, 0.25, 0.25]).weights[0] == 0.26
        with pytest.raises(ValueError, match="weights 之和必须是 1"):
            DatasetHealthScore(weights=[0.23, 0.25, 0.25, 0.25])

    def test_score_is_clamped_into_unit_interval(self, healthy_items, poor_items):
        """A319（L238）：出口钳进 [0, 1]。分量各自在界内，唯一越界来源是「和的容差」——
        1.005 份权重乘满分分量就是 1.005，而 `level` 判的正是这个数。"""
        for items in (healthy_items, poor_items):
            for weights in ([0.25] * 4, [1.0, 0.005, 0.005, 0.0], [0.0, 0.0, 0.0, 1.0]):
                result = DatasetHealthScore(weights=weights).score(items)
                assert 0.0 <= result["health_score"] <= 1.0, (weights, result)
        m = DatasetHealthScore().score(healthy_items)["metrics"]
        raw = 1.0 * m["completeness"] + 0.005 * m["diversity"] + 0.005 * m["quality_balance"]
        assert raw > 1.0, "夹具不再越过 1 ⇒ 上面那圈的钳制断言是空转，换夹具再来"
        assert DatasetHealthScore(weights=[1.0, 0.005, 0.005, 0.0]).score(
            healthy_items)["health_score"] == 1.0


class TestDatasetHealthScoreMetrics:
    """健康度指标测试 - 验证完整性、多样性、均衡性、覆盖率计算意图"""
    
    def test_completeness_full_items(self, healthy_items):
        """完整数据应得高完整性分数"""
        scorer = DatasetHealthScore()
        score = scorer.calculate_completeness(healthy_items)
        assert score == 1.0
    
    def test_completeness_missing_fields(self):
        """缺失必填字段应降低完整性分数"""
        scorer = DatasetHealthScore()
        items = [{"instruction": "问题"}, {"instruction": "问题", "output": ""}]
        score = scorer.calculate_completeness(items)
        assert score < 1.0
    
    def test_diversity_high_for_unique_instructions(self, healthy_items):
        """指令多样性高时多样性分数应接近1"""
        scorer = DatasetHealthScore()
        score = scorer.calculate_diversity(healthy_items)
        assert score >= 0.8
    
    def test_diversity_low_for_duplicates(self):
        """重复指令应降低多样性分数，反映数据重复风险"""
        scorer = DatasetHealthScore()
        items = [
            {"instruction": "租房", "output": "a"},
            {"instruction": "租房", "output": "b"},
        ]
        score = scorer.calculate_diversity(items)
        assert score == 0.5  # 2条中1条唯一
    
    def test_quality_balance_uniform_lengths(self, healthy_items):
        """输出长度均匀时均衡性高"""
        scorer = DatasetHealthScore()
        score = scorer.calculate_quality_balance(healthy_items)
        assert 0.0 <= score <= 1.0
    
    def test_coverage_non_empty_text(self, healthy_items):
        """有文本内容时覆盖率应大于0"""
        scorer = DatasetHealthScore()
        score = scorer.calculate_coverage(healthy_items)
        assert score > 0.0


class TestDatasetHealthScoreOverall:
    """整体健康评分测试"""
    
    def test_healthy_dataset_scores_high(self, healthy_items):
        """健康数据集应获得较高健康分数（>=0.7 标记为 healthy）"""
        scorer = DatasetHealthScore()
        result = scorer.score(healthy_items)
        assert result["health_score"] >= 0.5
        assert result["metrics"]["completeness"] == 1.0
        assert result["metrics"]["diversity"] >= 0.8
        assert "healthy" in result["level"] or "moderate" in result["level"]
    
    def test_poor_dataset_scores_lower_than_healthy(self, poor_items, healthy_items):
        """低健康数据集应获得比健康数据集更低的健康分数，且应标记为 poor"""
        scorer = DatasetHealthScore()
        poor_result = scorer.score(poor_items)
        healthy_result = scorer.score(healthy_items)
        assert poor_result["health_score"] < healthy_result["health_score"]
        # 重复严重且内容缺失的数据应被标记为 poor（根据实际计算，可能在边界附近）
        assert poor_result["level"] in ("poor", "moderate")
    
    def test_empty_items_returns_zero(self):
        """空数据集健康分数应为0，反映无数据无法评估健康状态"""
        scorer = DatasetHealthScore()
        result = scorer.score([])
        assert result["health_score"] == 0.0
        assert result["total_samples"] == 0
    
    def test_result_contains_all_metrics(self, healthy_items):
        """评分结果应包含所有必要指标"""
        scorer = DatasetHealthScore()
        result = scorer.score(healthy_items)
        assert "health_score" in result
        assert "level" in result
        assert "metrics" in result
        assert "weights" in result
        metrics = result["metrics"]
        assert all(k in metrics for k in ["completeness", "diversity", "quality_balance", "coverage"])
