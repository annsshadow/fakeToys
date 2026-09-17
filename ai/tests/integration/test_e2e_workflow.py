"""端到端工作流集成测试

验证多个新模块（profiling/outlier/aggregator/impact/data_pipeline）
在真实数据上能协同完成 清洗→去重→画像→异常过滤→聚合→增益评估 流程。
"""

import json

import pytest


@pytest.fixture
def dataset_a():
    return [
        {"instruction": "如何申请租房？", "output": "登录官网申请"},
        {"instruction": "如何申请租房？", "output": "登录官网申请"},
        {"instruction": "租房多少钱？", "output": "按房型定价"},
        {"instruction": "退租押金怎么退？", "output": "满一年后退还"},
        {"instruction": "可以申请月付吗？", "output": "支持月付与季付"},
        {"instruction": "", "output": ""},
    ]


@pytest.fixture
def dataset_b():
    return [
        {"instruction": "租期最短多久？", "output": "一个月起租"},
        {"instruction": "可以申请月付吗？", "output": "支持月付与季付"},
    ]


class TestEndToEndWorkflow:
    def test_full_pipeline(self, dataset_a, dataset_b):
        from augmentor.data_pipeline import DataPipeline
        from augmentor.outlier import OutlierDetector
        from augmentor.aggregator import DataAggregator
        from augmentor.impact import ImpactEvaluator
        from augmentor.profiling import DataProfiler

        # 1. 并集聚合两个数据集
        aggregator = DataAggregator()
        aggregated = aggregator.aggregate(
            {"a": dataset_a, "b": dataset_b}, "union"
        ).aggregated

        # 2. 清洗：移除空 instruction
        def clean_stage(items, ctx):
            return [
                item for item in items
                if item.get("instruction", "").strip()
            ]

        pipeline = DataPipeline("e2e")
        pipeline.add_stage("clean", clean_stage, stop_on_failure=True)
        cleaned = pipeline.run(aggregated)

        assert all(item.get("instruction") for item in cleaned)
        assert pipeline.results[0].success

        # 3. 画像
        profile = DataProfiler().profile(cleaned)
        assert profile["total_items"] == len(cleaned)
        # 并集聚合已去重，清洗后的数据重复率应为 0
        assert profile["duplicate_rate"] == 0.0

        # 4. 异常检测
        detector = OutlierDetector(field="length", threshold=3.0)
        with_length = detector.attach_length_field(cleaned)
        outliers = detector.detect(with_length)
        assert outliers.total_items == len(cleaned)

        # 5. 增益评估（原始 vs 清洗后；去重可能使规模收缩，故只验证键存在）
        evaluator = ImpactEvaluator()
        impact = evaluator.evaluate(dataset_a, cleaned)
        assert "scale_gain" in impact.gains
        assert "dedup_gain" in impact.gains


class TestAggregatorWithRealData:
    def test_weighted_aggregation_reproducible(self, tmp_path, dataset_a, dataset_b):
        """加权聚合在相同种子下应可复现，且结果可序列化"""
        from augmentor.aggregator import DataAggregator

        paths = {}
        for name, data in (("a", dataset_a), ("b", dataset_b)):
            path = tmp_path / f"{name}.json"
            path.write_text(json.dumps(data, ensure_ascii=False), encoding="utf-8")
            paths[name] = str(path)

        r1 = DataAggregator(random_state=42).aggregate(
            {"a": dataset_a, "b": dataset_b},
            "weighted", weights={"a": 2.0, "b": 1.0}, target_size=5
        )
        r2 = DataAggregator(random_state=42).aggregate(
            {"a": dataset_a, "b": dataset_b},
            "weighted", weights={"a": 2.0, "b": 1.0}, target_size=5
        )

        assert [i["instruction"] for i in r1.aggregated] == \
               [i["instruction"] for i in r2.aggregated]
        assert len(r1.aggregated) <= 5

    def test_consistent_aggregation_detects_conflict(self, dataset_a, dataset_b):
        """两源相同问题不同答案时应记录冲突"""
        from augmentor.aggregator import DataAggregator

        conflicting_b = [
            {"instruction": "如何申请租房？", "output": "不同答案哦"},
        ]
        result = DataAggregator().aggregate(
            {"a": dataset_a, "b": conflicting_b}, "consistent"
        )
        assert len(result.conflicts) == 1
        assert result.conflicts[0]["key"] == "如何申请租房？"


class TestOutlierIntegration:
    def test_outlier_removal_reduces_dataset(self, dataset_a):
        """异常过滤应能缩小数据集"""
        from augmentor.outlier import OutlierDetector

        items = dataset_a + [{"instruction": "问" + "长" * 800}]
        detector = OutlierDetector(field="length", threshold=2.0)
        enriched = detector.attach_length_field(items)
        kept = detector.filter(enriched)
        assert len(kept) < len(items)


class TestImpactIntegration:
    def test_augmentation_gains_positive_for_growth(self, dataset_b):
        """合并后数据集规模增长，scale_gain 应为正且去重不恶化"""
        from augmentor.impact import ImpactEvaluator

        before = dataset_b
        after = dataset_b + [
            {"instruction": "全新问题一？", "output": "全新答案一"},
            {"instruction": "全新问题二？", "output": "全新答案二"},
        ]
        impact = ImpactEvaluator().evaluate(before, after)
        assert impact.gains["scale_gain"] > 0
        assert ImpactEvaluator().is_beneficial(impact)
