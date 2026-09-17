"""质量报告单元测试

报告是质量门禁的产出物，统计口径与建议逻辑必须可靠。
"""

import pytest

from augmentor.quality import QualityScore
from augmentor.report import ReportGenerator, QualityReport


def make_scores(values):
    """构造 QualityScore 列表

    Args:
        values: total_score 列表

    Returns:
        QualityScore 列表
    """
    return [
        QualityScore(
            semantic_similarity=v,
            relevance=v,
            diversity=v,
            total_score=v,
            passed=v >= 0.6
        )
        for v in values
    ]


class TestGenerate:
    """报告生成"""

    def test_counts_and_pass_rate(self):
        """通过率必须与 passed 数量严格对应"""
        items = [{"instruction": f"问题{i}"} for i in range(4)]
        scores = make_scores([0.9, 0.8, 0.3, 0.2])

        report = ReportGenerator(threshold=0.6).generate(items, scores)

        assert isinstance(report, QualityReport)
        assert report.total_samples == 4
        assert report.passed_samples == 2
        assert report.filtered_samples == 2
        assert report.pass_rate == pytest.approx(0.5)

    def test_metric_summary_statistics(self):
        """均值/极值统计必须正确，供前端雷达图使用"""
        items = [{"instruction": f"问题{i}"} for i in range(3)]
        scores = make_scores([0.2, 0.5, 0.8])

        report = ReportGenerator().generate(items, scores)
        total = report.metric_summary["total_score"]

        assert total["mean"] == pytest.approx(0.5)
        assert total["min"] == pytest.approx(0.2)
        assert total["max"] == pytest.approx(0.8)

    def test_distribution_buckets_sum_to_one(self):
        """分桶占比之和应为 1，否则图表会失真"""
        items = [{"instruction": f"问题{i}"} for i in range(5)]
        scores = make_scores([0.1, 0.3, 0.5, 0.7, 0.9])

        report = ReportGenerator().generate(items, scores)
        assert sum(report.score_distribution.values()) == pytest.approx(1.0)

    def test_length_mismatch_raises(self):
        """长度不一致必须报错，避免统计错位"""
        with pytest.raises(ValueError):
            ReportGenerator().generate([{"instruction": "a"}], make_scores([0.5, 0.6]))

    def test_empty_dataset(self):
        """空数据集报告应全为 0 且不抛异常"""
        report = ReportGenerator().generate([], [])
        assert report.total_samples == 0
        assert report.pass_rate == 0.0

    def test_dedup_summary_passthrough(self):
        """去重统计需原样并入报告"""
        items = [{"instruction": "a"}]
        report = ReportGenerator().generate(
            items, make_scores([0.9]), {"removal_rate": 0.25}
        )
        assert report.dedup_summary["removal_rate"] == 0.25


class TestSuggestions:
    """改进建议规则"""

    def test_low_pass_rate_suggests_relaxing_threshold(self):
        """通过率过低时应建议放宽阈值"""
        items = [{"instruction": f"q{i}"} for i in range(4)]
        report = ReportGenerator().generate(items, make_scores([0.1, 0.1, 0.1, 0.1]))

        assert any("放宽" in s for s in report.improvement_suggestions)

    def test_high_pass_rate_suggests_tightening(self):
        """通过率过高说明阈值过松，应建议提高阈值"""
        items = [{"instruction": f"q{i}"} for i in range(4)]
        report = ReportGenerator().generate(items, make_scores([0.99] * 4))

        assert any("提高阈值" in s for s in report.improvement_suggestions)

    def test_healthy_data_reports_good_quality(self):
        """指标正常时应给出正向结论而不是硬凑建议"""
        items = [{"instruction": f"q{i}"} for i in range(4)]
        # 通过率 75% 且各分项均值均高于 0.7，属于健康区间
        report = ReportGenerator().generate(items, make_scores([0.9, 0.75, 0.5, 0.8]))

        assert report.improvement_suggestions == ["各项指标均在合理区间，数据质量良好"]

    def test_high_removal_rate_warns(self):
        """去重移除比例过高说明种子同质化，需提示"""
        items = [{"instruction": "q"}]
        report = ReportGenerator().generate(
            items, make_scores([0.9]), {"removal_rate": 0.6}
        )

        assert any("去重移除比例较高" in s for s in report.improvement_suggestions)

    def test_low_semantic_similarity_suggests(self):
        """语义相似度偏低时应建议收敛生成约束"""
        items = [{"instruction": f"q{i}"} for i in range(3)]
        scores = [
            QualityScore(semantic_similarity=0.3, relevance=0.9, diversity=0.9,
                         total_score=0.7, passed=True)
            for _ in range(3)
        ]
        report = ReportGenerator().generate(items, scores)
        assert any("语义相似度" in s for s in report.improvement_suggestions)

    def test_low_relevance_suggests(self):
        """回答相关性偏低时应建议检查问答匹配"""
        items = [{"instruction": f"q{i}"} for i in range(3)]
        scores = [
            QualityScore(semantic_similarity=0.9, relevance=0.2, diversity=0.9,
                         total_score=0.7, passed=True)
            for _ in range(3)
        ]
        report = ReportGenerator().generate(items, scores)
        assert any("相关性" in s for s in report.improvement_suggestions)

    def test_low_diversity_suggests(self):
        """多样性偏低时应建议提高 temperature"""
        items = [{"instruction": f"q{i}"} for i in range(3)]
        scores = [
            QualityScore(semantic_similarity=0.9, relevance=0.9, diversity=0.1,
                         total_score=0.63, passed=True)
            for _ in range(3)
        ]
        report = ReportGenerator().generate(items, scores)
        assert any("多样性" in s for s in report.improvement_suggestions)


class TestSerialization:
    """序列化"""

    def test_to_dict_contains_charts(self):
        """to_dict 需包含图表数据，供 Web UI 直接使用"""
        items = [{"instruction": f"q{i}"} for i in range(3)]
        report = ReportGenerator().generate(items, make_scores([0.2, 0.5, 0.9]))
        payload = report.to_dict()

        assert "score_histogram" in payload["charts"]
        assert "metric_radar" in payload["charts"]
        assert len(payload["charts"]["metric_radar"]["labels"]) > 0

    def test_to_markdown_has_sections(self):
        """Markdown 报告需包含概况与建议小节"""
        items = [{"instruction": f"q{i}"} for i in range(3)]
        report = ReportGenerator().generate(
            items, make_scores([0.2, 0.5, 0.9]), {"removal_rate": 0.1}
        )
        markdown = report.to_markdown()

        assert "# 数据质量报告" in markdown
        assert "## 总体概况" in markdown
        assert "## 改进建议" in markdown
