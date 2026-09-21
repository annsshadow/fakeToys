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

    def test_empty_data_returns_empty(self, candidates):
        """空候选列表应返回空而不是抛异常"""
        loop = ActiveLearningLoop(batch_size=2, score_fn=lambda item: 0.5)
        assert loop.select_samples([]) == []

    def test_invalid_strategy_rejected(self, candidates):
        """select_samples 传未知策略应立即报错"""
        loop = ActiveLearningLoop(batch_size=2, score_fn=lambda item: 0.5)
        with pytest.raises(ValueError, match="不支持"):
            loop.select_samples(candidates, strategy="magic")

    def test_selected_items_carry_score_and_index(self, candidates):
        """选中样本应附带 _al_score 与 _al_index，便于追溯"""
        loop = ActiveLearningLoop(batch_size=2, score_fn=lambda item: 0.7)
        selected = loop.select_samples(candidates)
        for item in selected:
            assert "_al_score" in item
            assert "_al_index" in item

    def test_selected_items_are_copies(self, candidates):
        """选择不得修改调用方传入的原始数据"""
        loop = ActiveLearningLoop(batch_size=1, score_fn=lambda item: 0.5)
        before = dict(candidates[0])
        loop.select_samples(candidates)
        assert candidates[0] == before


class TestEvaluatePerformance:
    """性能评估"""

    def test_empty_loop_report(self):
        """无数据时性能报告仅含基础字段"""
        loop = ActiveLearningLoop(score_fn=lambda item: 0.5)
        report = loop.evaluate_performance()
        assert report["iteration"] == 0
        assert report["labeled_count"] == 0
        assert "avg_selection_score" not in report

    def test_labeled_scores_reported(self):
        """有标注数据时应统计选择分数的均值/极值"""
        loop = ActiveLearningLoop(batch_size=2, score_fn=lambda item: 0.5)
        items = [{"instruction": f"q{i}"} for i in range(3)]
        loop.update_model(items)
        for item in loop._labeled:
            item["_al_score"] = 0.5
        report = loop.evaluate_performance()
        assert report["avg_selection_score"] == 0.5
        assert report["max_selection_score"] == 0.5
        assert report["min_selection_score"] == 0.5

    def test_candidate_ratio_reported(self):
        """候选数据存在时应计算标注占比"""
        loop = ActiveLearningLoop(batch_size=1, score_fn=lambda item: 0.5)
        data = [{"instruction": "q1"}]
        loop.update_model([{"instruction": "labeled", "_al_score": 0.5}])
        report = loop.evaluate_performance(data)
        assert report["candidate_count"] == 1
        assert report["labeled_ratio"] == pytest.approx(1.0)

    def test_history_stats_in_report(self, candidates):
        """有迭代历史时应统计平均批量大小"""
        loop = ActiveLearningLoop(batch_size=2, score_fn=lambda item: 0.5)
        loop.run(candidates, iterations=2)
        report = loop.evaluate_performance()
        assert report["iterations"] == 2
        assert report["avg_batch_size"] == pytest.approx(2.0)


class TestRunLoop:
    """循环执行"""

    def test_multiple_iterations_reduce_remaining(self, candidates):
        """多轮运行后剩余候选应递减直至耗尽"""
        loop = ActiveLearningLoop(batch_size=2, score_fn=lambda item: 0.5)
        report = loop.run(candidates, iterations=3)
        assert report["total_selected"] == 5
        assert report["remaining_count"] == 0
        assert report["iterations"] == 3

    def test_indices_map_back_to_original_data(self, candidates):
        """第二轮选出的 _al_index 必须指向原始数据的下标"""
        loop = ActiveLearningLoop(batch_size=2, score_fn=lambda item: 0.5)
        report = loop.run(candidates, iterations=2)
        selected_indices = [
            item["_al_index"]
            for batch in report["batches"]
            for item in batch
        ]
        assert selected_indices == sorted(selected_indices)
        assert len(set(selected_indices)) == len(selected_indices)
        for index in selected_indices:
            assert candidates[index]["instruction"] in [
                item["instruction"] for batch in report["batches"] for item in batch
            ]

    def test_zero_iterations_rejected(self, candidates):
        """iterations=0 必须报错"""
        loop = ActiveLearningLoop(batch_size=1, score_fn=lambda item: 0.5)
        with pytest.raises(ValueError, match="iterations"):
            loop.run(candidates, iterations=0)

    def test_random_strategy_is_reproducible(self, candidates):
        """随机策略在相同种子下必须可复现"""
        loop_a = ActiveLearningLoop(
            batch_size=2, strategy="random", score_fn=lambda item: 0.5, random_state=42
        )
        loop_b = ActiveLearningLoop(
            batch_size=2, strategy="random", score_fn=lambda item: 0.5, random_state=42
        )
        report_a = loop_a.run(candidates, iterations=1)
        report_b = loop_b.run(candidates, iterations=1)
        assert [i["_al_index"] for i in report_a["batches"][0]] == \
               [i["_al_index"] for i in report_b["batches"][0]]

    def test_diversity_strategy_prefers_dissimilar(self, candidates):
        """多样性策略应优先选择彼此差异大的样本"""
        data = [
            {"instruction": "如何申请入住安居乐寓？"},
            {"instruction": "如何申请入住安居乐寓呢？"},
            {"instruction": "押金什么时候退还？"},
        ]
        loop = ActiveLearningLoop(
            batch_size=2, strategy="diversity", score_fn=lambda item: 0.5
        )
        selected = loop.select_samples(data)
        instructions = [item["instruction"] for item in selected]
        # 两条相似问题不应同时被选中
        assert not all("申请入住安居乐寓" in i for i in instructions)


class TestUpdateModel:
    """标注更新"""

    def test_update_increments_iteration(self):
        loop = ActiveLearningLoop(score_fn=lambda item: 0.5)
        assert loop._iteration == 0
        loop.update_model([{"instruction": "a"}])
        assert loop._iteration == 1
        assert len(loop._labeled) == 1

    def test_update_with_empty_data_only_increments(self):
        loop = ActiveLearningLoop(score_fn=lambda item: 0.5)
        loop.update_model([])
        assert loop._iteration == 1
        assert len(loop._labeled) == 0


class TestIterationRecord:
    """迭代记录"""

    def test_to_dict_fields(self):
        record = IterationRecord(
            iteration=1, strategy="uncertainty",
            selected_count=2, selected_indices=[0, 1],
            performance={"avg": 0.5}
        )
        d = record.to_dict()
        assert d["iteration"] == 1
        assert d["selected_indices"] == [0, 1]
        assert d["performance"] == {"avg": 0.5}

    def test_default_fields(self):
        record = IterationRecord(iteration=1, strategy="random", selected_count=0)
        assert record.selected_indices == []
        assert record.performance == {}


class TestUncertaintyScores:
    """不确定性打分"""

    def test_score_near_threshold_is_higher(self):
        loop = ActiveLearningLoop(threshold=0.6, score_fn=lambda item: 0.6)
        far = ActiveLearningLoop(threshold=0.6, score_fn=lambda item: 0.1)
        items = [{"instruction": "q"}]
        assert loop._uncertainty_scores(items)[0] == pytest.approx(1.0)
        assert far._uncertainty_scores(items)[0] == pytest.approx(0.5)

    def test_scores_clamped_to_unit_range(self):
        loop = ActiveLearningLoop(threshold=0.6, score_fn=lambda item: 0.9)
        result = loop._uncertainty_scores([{"instruction": "q"}])
        assert 0.0 <= result[0] <= 1.0


class TestHybridScores:
    """混合策略打分"""

    def test_hybrid_averages_uncertainty_and_diversity(self):
        items = [{"instruction": f"不同的问题{i}啊哈"} for i in range(3)]
        loop = ActiveLearningLoop(batch_size=3, score_fn=lambda item: 0.5)
        hybrid = loop._compute_scores(items, "hybrid")
        uncertainty = loop._uncertainty_scores(items)
        diversity = loop._diversity_scores(items)
        for i in range(len(items)):
            assert hybrid[i] == pytest.approx((uncertainty[i] + diversity[i]) / 2.0)

    def test_diversity_scores_empty(self):
        loop = ActiveLearningLoop(score_fn=lambda item: 0.5)
        assert loop._diversity_scores([]) == []


class TestScoreFn:
    """自定义打分函数"""

    def test_score_fn_takes_priority(self):
        loop = ActiveLearningLoop(score_fn=lambda item: float(item["value"]))
        items = [
            {"instruction": "a", "value": 0.1},
            {"instruction": "b", "value": 0.9},
        ]
        selected = loop.select_samples(items, strategy="uncertainty")
        # 越接近阈值 0.6 的 uncertainty 越高；0.9 距 0.3，0.1 距 0.5 -> 0.9 分更高
        assert selected[0]["instruction"] == "b"

    def test_default_scorer_lazy_creation(self):
        loop = ActiveLearningLoop()
        assert loop._get_scorer() is not None
        # 二次获取应复用同一实例
        assert loop._get_scorer() is loop._scorer


class TestSelectSamplesEdge:
    """选择边界补充"""

    def test_batch_size_exceeds_data_capped(self, candidates):
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
