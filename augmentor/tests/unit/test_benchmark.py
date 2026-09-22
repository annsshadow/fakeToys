# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""数据质量基准单元测试

基准用于跨版本对比数据质量，存档与回归判定必须准确。
"""

import pytest

from augmentor.benchmark import QualityBenchmark, SUPPORTED_METRICS, METRIC_DIRECTIONS


class FakeScore:
    """模拟质量分"""

    def __init__(self, total_score, diversity, passed):
        self.total_score = total_score
        self.diversity = diversity
        self.passed = passed


class FakeScorer:
    """模拟评分器"""

    def __init__(self, values):
        self.values = values

    def batch_score(self, items):
        """返回预设分值

        Args:
            items: 输入数据

        Returns:
            分值列表
        """
        return [
            FakeScore(v, v, v >= 0.6)
            for v in self.values[:len(items)]
        ]


class FakeDeduplicator:
    """模拟去重器"""

    def __init__(self, removal_rate):
        self.removal_rate = removal_rate

    def generate_report(self, items, text_key="instruction"):
        """返回预设重复率

        Args:
            items: 输入数据
            text_key: 文本字段

        Returns:
            报告字典
        """
        return {"removal_rate": self.removal_rate, "original_count": len(items)}


def make_benchmark(values=(0.9, 0.8, 0.7, 0.5), removal_rate=0.1, **kwargs):
    """构造带假依赖的基准对象

    Args:
        values: 评分序列
        removal_rate: 重复率
        **kwargs: 其他参数

    Returns:
        QualityBenchmark 实例
    """
    return QualityBenchmark(
        scorer=FakeScorer(list(values)),
        deduplicator=FakeDeduplicator(removal_rate),
        **kwargs
    )


class TestInit:
    """初始化校验"""

    def test_default_metrics(self):
        """默认指标需覆盖计划约定的四项"""
        assert QualityBenchmark().metrics == list(SUPPORTED_METRICS)

    def test_rejects_unknown_metric(self):
        """未知指标必须报错"""
        with pytest.raises(ValueError):
            QualityBenchmark(metrics=["unknown"])

    def test_metric_directions(self):
        """重复率越小越好，其余越大越好，方向定义影响回归判定"""
        assert METRIC_DIRECTIONS["duplication_rate"] is False
        assert METRIC_DIRECTIONS["pass_rate"] is True


class TestRunBenchmark:
    """基准执行"""

    def test_computes_metrics(self):
        """基准结果需包含全部指标数值"""
        bench = make_benchmark(values=(0.9, 0.8, 0.7, 0.5), removal_rate=0.1)
        result = bench.run_benchmark([{"instruction": f"q{i}"} for i in range(4)])

        assert result["sample_count"] == 4
        assert result["metrics"]["pass_rate"] == pytest.approx(0.75)
        assert result["metrics"]["avg_total_score"] == pytest.approx(0.725)
        assert result["metrics"]["duplication_rate"] == pytest.approx(0.1)

    def test_restricted_metrics(self):
        """只请求部分指标时结果中不应出现其他指标"""
        bench = make_benchmark(metrics=["pass_rate"])
        result = bench.run_benchmark([{"instruction": "q"}])

        assert set(result["metrics"].keys()) == {"pass_rate"}
        # all_metrics 仍保留完整计算，便于排查
        assert "duplication_rate" in result["all_metrics"]

    def test_empty_dataset(self):
        """空数据集应返回零值结果而不是抛异常"""
        result = QualityBenchmark().run_benchmark([])

        assert result["sample_count"] == 0
        assert all(v == 0.0 for v in result["metrics"].values())

    def test_result_has_timestamp(self):
        """结果需带时间戳，供基准对比展示"""
        result = make_benchmark().run_benchmark([{"instruction": "q"}])
        assert result["timestamp"]


class TestBaselinePersistence:
    """基准存档"""

    def test_save_and_load(self, tmp_path):
        """保存后可原样加载，保证跨会话对比可行"""
        baseline_file = tmp_path / "baseline.json"
        bench = make_benchmark(baseline_file=str(baseline_file))
        results = bench.run_benchmark([{"instruction": f"q{i}"} for i in range(4)])

        bench.save_baseline(results)

        assert baseline_file.exists()
        assert bench.load_baseline()["metrics"]["pass_rate"] == pytest.approx(0.75)

    def test_save_without_path_raises(self):
        """未配置路径时必须报错"""
        with pytest.raises(ValueError):
            make_benchmark().save_baseline({"metrics": {}})

    def test_load_without_file_returns_none(self, tmp_path):
        """文件不存在时返回 None"""
        bench = make_benchmark(baseline_file=str(tmp_path / "missing.json"))
        assert bench.load_baseline() is None


class TestCompareWithBaseline:
    """基准对比"""

    def test_detects_improvement(self):
        """指标上升应判定为 improved"""
        bench = make_benchmark()
        baseline = {"metrics": {"pass_rate": 0.5, "avg_total_score": 0.5,
                               "diversity": 0.5, "duplication_rate": 0.2},
                    "timestamp": "t0"}
        current = {"metrics": {"pass_rate": 0.8, "avg_total_score": 0.5,
                              "diversity": 0.5, "duplication_rate": 0.2},
                   "timestamp": "t1"}

        comparison = bench.compare_with_baseline(current, baseline)

        assert "pass_rate" in comparison["improved"]
        assert comparison["comparisons"]["pass_rate"]["status"] == "improved"
        assert comparison["overall"] == "improved"

    def test_lower_duplication_rate_counts_as_improvement(self):
        """重复率下降应算改进（方向相反），这是最易写反的逻辑"""
        bench = make_benchmark()
        baseline = {"metrics": {"duplication_rate": 0.4}, "timestamp": "t0"}
        current = {"metrics": {"duplication_rate": 0.1}, "timestamp": "t1"}

        comparison = bench.compare_with_baseline(current, baseline)

        assert "duplication_rate" in comparison["improved"]

    def test_detects_regression(self):
        """指标下降应判定为 regressed"""
        bench = make_benchmark()
        baseline = {"metrics": {"pass_rate": 0.9}, "timestamp": "t0"}
        current = {"metrics": {"pass_rate": 0.3}, "timestamp": "t1"}

        comparison = bench.compare_with_baseline(current, baseline)

        assert comparison["overall"] == "regressed"
        assert comparison["comparisons"]["pass_rate"]["delta"] == pytest.approx(-0.6)

    def test_unchanged_metrics(self):
        """数值相同判定为 unchanged"""
        bench = make_benchmark()
        baseline = {"metrics": {"pass_rate": 0.5}, "timestamp": "t0"}
        current = {"metrics": {"pass_rate": 0.5}, "timestamp": "t1"}

        comparison = bench.compare_with_baseline(current, baseline)
        assert comparison["unchanged"] == ["pass_rate"]

    def test_missing_baseline_raises(self, tmp_path):
        """无基准时必须报错，提示先保存基准"""
        bench = make_benchmark(baseline_file=str(tmp_path / "missing.json"))
        with pytest.raises(ValueError):
            bench.compare_with_baseline({"metrics": {}})


class TestReport:
    """报告渲染"""

    def test_markdown_contains_metrics_table(self):
        """Markdown 报告需包含指标表格"""
        bench = make_benchmark(metrics=["pass_rate", "duplication_rate"])
        results = bench.run_benchmark([{"instruction": f"q{i}"} for i in range(4)])
        markdown = bench.generate_report(results)

        assert "# 数据质量基准报告" in markdown
        assert "| pass_rate |" in markdown
        assert "越小越好" in markdown

    def test_markdown_contains_comparison_when_present(self):
        """存在对比结果时应渲染对比表格"""
        bench = make_benchmark()
        results = bench.run_benchmark([{"instruction": "q"}])
        results["comparisons"] = {
            "pass_rate": {"current": 0.8, "baseline": 0.5, "delta": 0.3, "status": "improved"}
        }

        assert "## 与基准对比" in bench.generate_report(results)


class TestBenchmarkExtended:
    """QualityBenchmark 扩展测试"""

    def test_run_benchmark_empty(self):
        """空数据基准测试"""
        bench = make_benchmark()
        results = bench.run_benchmark([])
        assert results["sample_count"] == 0

    def test_run_benchmark_single_item(self):
        """单条数据基准测试"""
        bench = make_benchmark()
        results = bench.run_benchmark([{"instruction": "q1"}])
        assert results["sample_count"] == 1

    def test_save_and_load_baseline(self, tmp_path):
        """保存并加载基准"""
        bench = make_benchmark(baseline_file=str(tmp_path / "baseline.json"))
        results = bench.run_benchmark([{"instruction": "q1"}])
        bench.save_baseline(results)
        loaded = bench.load_baseline()
        assert loaded is not None
        assert "metrics" in loaded

    def test_supported_metrics(self):
        """支持的指标列表"""
        assert "pass_rate" in SUPPORTED_METRICS
        assert "duplication_rate" in SUPPORTED_METRICS

    def test_metric_directions(self):
        """指标方向"""
        assert METRIC_DIRECTIONS["pass_rate"] is True
        assert METRIC_DIRECTIONS["duplication_rate"] is False

    def test_compare_overall_improved(self):
        """整体改善判定"""
        bench = make_benchmark()
        baseline = {"metrics": {"pass_rate": 0.5}, "timestamp": "t0"}
        current = {"metrics": {"pass_rate": 0.9}, "timestamp": "t1"}
        comparison = bench.compare_with_baseline(current, baseline)
        assert comparison["overall"] == "improved"
