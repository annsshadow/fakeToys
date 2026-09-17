"""主动学习循环单元测试

选样策略决定标注预算的投放方向，策略校验与迭代收敛必须可靠。
"""

import pytest

from augmentor.active_learning import (
    ActiveLearningLoop,
    IterationRecord,
    SUPPORTED_STRATEGIES,
)


@pytest.fixture
def candidates():
    """候选样本

    Returns:
        数据列表
    """
    return [
        {"instruction": "如何申请入住安居乐寓？", "output": "通过 App 申请"},
        {"instruction": "退租需要提前多久？", "output": "提前 30 天"},
        {"instruction": "租金如何计算？", "output": "按房型定价"},
        {"instruction": "可以月付吗？", "output": "支持月付与季付"},
        {"instruction": "如何申请入住安居乐寓呢？", "output": "通过 App 申请"},
    ]


class TestInit:
    """初始化校验"""

    def test_rejects_unknown_strategy(self):
        """未知策略必须立即报错，否则运行到一半才失败"""
        with pytest.raises(ValueError):
            ActiveLearningLoop(strategy="magic")

    def test_rejects_non_positive_batch_size(self):
        """批量必须为正整数"""
        with pytest.raises(ValueError):
            ActiveLearningLoop(batch_size=0)

    def test_supported_strategies_exposed(self):
        """策略清单需包含计划约定的四种"""
        assert set(SUPPORTED_STRATEGIES) == {"uncertainty", "diversity", "hybrid", "random"}


class TestSelectSamples:
    """样本选择"""

    def test_returns_requested_batch_size(self, candidates):
        """选出数量需受 batch_size 限制"""
        loop = ActiveLearningLoop(batch_size=2, score_fn=lambda item: 0.5)
        selected = loop.select_samples(candidates)

        assert len(selected) == 2

    def test_batch_size_capped_by_data(self, candidates):
        """batch_size 超过数据量时应自动收敛"""
        loop = ActiveLearningLoop(batch_size=100, score_fn=lambda item: 0.5)
        assert len(loop.select_samples(candidates)) == len(candidates)

    def test_empty_dataset(self):
        """空数据集返回空列表"""
        loop = ActiveLearningLoop(score_fn=lambda item: 0.5)
        assert loop.select_samples([]) == []

    def test_uncertainty_prefers_near_threshold(self):
        """不确定性策略应优先选择接近阈值的样本，这是主动学习的核心假设"""
        loop = ActiveLearningLoop(strategy="uncertainty", batch_size=1, threshold=0.6)

        def score_fn(item):
            return item["score"]

        loop._score_fn = score_fn
        data = [
            {"instruction": "a", "score": 0.99},
            {"instruction": "b", "score": 0.6},
            {"instruction": "c", "score": 0.05},
        ]

        selected = loop.select_samples(data, strategy="uncertainty")

        assert selected[0]["score"] == 0.6

    def test_diversity_prefers_dissimilar(self):
        """多样性策略应优先选择与其余样本差异最大的样本"""
        loop = ActiveLearningLoop(strategy="diversity", batch_size=1)
        data = [
            {"instruction": "完全一样的问题内容"},
            {"instruction": "完全一样的问题内容"},
            {"instruction": "毫不相干的另一主题"},
        ]

        selected = loop.select_samples(data, strategy="diversity")

        assert selected[0]["instruction"] == "毫不相干的另一主题"

    def test_selected_items_carry_score_metadata(self, candidates):
        """选中样本需附带分值与索引，便于回溯选择依据"""
        loop = ActiveLearningLoop(batch_size=2, score_fn=lambda item: 0.7)
        selected = loop.select_samples(candidates)

        for item in selected:
            assert "_al_score" in item
            assert "_al_index" in item

    def test_unknown_strategy_argument_raises(self, candidates):
        """调用期传入未知策略同样必须报错"""
        loop = ActiveLearningLoop()
        with pytest.raises(ValueError):
            loop.select_samples(candidates, strategy="unknown")

    def test_random_strategy_is_reproducible_with_seed(self, candidates):
        """指定随机种子后结果应可复现，便于回归对比"""
        first = ActiveLearningLoop(
            strategy="random", batch_size=3, random_state=42
        ).select_samples(candidates)
        second = ActiveLearningLoop(
            strategy="random", batch_size=3, random_state=42
        ).select_samples(candidates)

        assert [i["_al_index"] for i in first] == [i["_al_index"] for i in second]


class TestLoopLifecycle:
    """循环生命周期"""

    def test_update_model_accumulates_and_increments(self):
        """标注数据应被累计，迭代计数递增"""
        loop = ActiveLearningLoop()
        loop.update_model([{"instruction": "a"}])
        loop.update_model([{"instruction": "b"}])

        assert len(loop._labeled) == 2
        assert loop._iteration == 2

    def test_update_model_with_empty_does_not_crash(self):
        """空标注数据不应导致异常"""
        loop = ActiveLearningLoop()
        loop.update_model([])
        assert loop._iteration == 1

    def test_evaluate_performance_before_any_selection(self):
        """未选样时评估应给出零值报告而不是报错"""
        report = ActiveLearningLoop().evaluate_performance()

        assert report["iteration"] == 0
        assert report["labeled_count"] == 0

    def test_run_single_iteration(self, candidates):
        """单轮运行需返回批次与历史"""
        loop = ActiveLearningLoop(batch_size=2, score_fn=lambda item: 0.5)
        result = loop.run(candidates)

        assert result["iterations"] == 1
        assert result["total_selected"] == 2
        assert result["remaining_count"] == len(candidates) - 2
        assert len(result["history"]) == 1

    def test_run_multiple_iterations_consumes_data(self, candidates):
        """多轮运行不应重复选择同一样本"""
        loop = ActiveLearningLoop(batch_size=2, score_fn=lambda item: 0.5)
        result = loop.run(candidates, iterations=2)

        assert result["iterations"] == 2
        assert result["total_selected"] == 4
        assert result["remaining_count"] == 1

        all_indices = [
            index for record in loop.history for index in record.selected_indices
        ]
        assert len(all_indices) == len(set(all_indices))

    def test_run_stops_when_data_exhausted(self, candidates):
        """数据耗尽时应提前结束而不是无限循环"""
        loop = ActiveLearningLoop(batch_size=2, score_fn=lambda item: 0.5)
        result = loop.run(candidates, iterations=10)

        assert result["iterations"] == 3
        assert result["remaining_count"] == 0

    def test_run_rejects_zero_iterations(self, candidates):
        """迭代次数必须为正"""
        loop = ActiveLearningLoop()
        with pytest.raises(ValueError):
            loop.run(candidates, iterations=0)

    def test_performance_report_includes_metrics(self, candidates):
        """报告需包含平均分与覆盖比例"""
        loop = ActiveLearningLoop(batch_size=2, score_fn=lambda item: 0.8)
        result = loop.run(candidates)

        performance = result["performance"]
        assert performance["labeled_count"] == 2
        assert performance["avg_selection_score"] == pytest.approx(0.8)
        assert performance["candidate_count"] == len(candidates)

    def test_iteration_record_serialization(self):
        """迭代记录需可序列化，供 API 返回"""
        record = IterationRecord(iteration=1, strategy="random", selected_count=2)
        payload = record.to_dict()

        assert payload["iteration"] == 1
        assert payload["selected_indices"] == []


class TestActiveLearningExtended:
    """ActiveLearningLoop 扩展测试"""

    def test_hybrid_strategy(self, candidates):
        """混合策略选择"""
        loop = ActiveLearningLoop(strategy="hybrid", batch_size=2, score_fn=lambda item: 0.5)
        selected = loop.select_samples(candidates)
        assert len(selected) == 2

    def test_score_fn_none_uses_random(self):
        """无评分函数时使用随机策略"""
        loop = ActiveLearningLoop(strategy="uncertainty", batch_size=2)
        data = [{"instruction": f"q{i}"} for i in range(5)]
        selected = loop.select_samples(data)
        assert len(selected) == 2

    def test_history_property(self, candidates):
        """history 属性"""
        loop = ActiveLearningLoop(batch_size=2, score_fn=lambda item: 0.5)
        loop.run(candidates)
        assert len(loop.history) == 1

    def test_select_samples_override_strategy(self, candidates):
        """运行时覆盖策略"""
        loop = ActiveLearningLoop(strategy="uncertainty", batch_size=2, score_fn=lambda item: 0.5)
        selected = loop.select_samples(candidates, strategy="random")
        assert len(selected) == 2

    def test_run_default_iterations(self, candidates):
        """默认迭代次数"""
        loop = ActiveLearningLoop(batch_size=5, score_fn=lambda item: 0.5)
        result = loop.run(candidates, iterations=1)
        assert result["iterations"] == 1
