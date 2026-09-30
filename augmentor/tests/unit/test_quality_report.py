# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""数据集质量报告模块测试"""

import json
import pytest
from pathlib import Path
from augmentor.quality_report import (
    QualityReporter, QualityReport, QualityMetric,
    generate_quality_report, save_quality_report
)


@pytest.fixture
def sample_dataset():
    """创建测试数据集"""
    return [
        {"instruction": "如何申请租房？", "input": "", "output": "请登录官网申请"},
        {"instruction": "租房需要什么材料？", "input": "", "output": "身份证、工作证明"},
        {"instruction": "租房流程是什么？", "input": "", "output": "选房、签约、付款"},
        {"instruction": "如何申请买房？", "input": "", "output": "请咨询销售顾问"},
        {"instruction": "买房需要什么材料？", "input": "", "output": "身份证、收入证明、征信报告"},
    ]


@pytest.fixture
def empty_dataset():
    """创建空数据集"""
    return []


class TestQualityReporter:
    """QualityReporter 测试"""
    
    def test_init(self):
        """测试初始化"""
        reporter = QualityReporter(threshold=0.7)
        assert reporter._threshold == 0.7
    
    def test_generate_report(self, sample_dataset):
        """测试生成报告"""
        reporter = QualityReporter()
        report = reporter.generate_report(sample_dataset, "test_dataset")
        
        assert isinstance(report, QualityReport)
        assert report.dataset_name == "test_dataset"
        assert report.total_items == 5
        assert len(report.metrics) > 0
        assert report.overall_score >= 0
        assert report.overall_score <= 1
    
    def test_generate_report_empty_dataset(self, empty_dataset):
        """测试生成空数据集报告"""
        reporter = QualityReporter()
        report = reporter.generate_report(empty_dataset, "empty_dataset")
        
        assert report.total_items == 0
        assert len(report.metrics) > 0

    def test_diversity_metric_duplicated_items(self):
        """全部重复问题应拉低多样性指标"""
        reporter = QualityReporter()
        items = [{"instruction": "同一个问题"}] * 4
        metric = reporter._calculate_diversity(items)
        assert metric.value == pytest.approx(0.25)
        assert metric.name == "多样性"

    def test_diversity_metric_no_instructions(self):
        """无有效问题时应返回未通过"""
        reporter = QualityReporter()
        metric = reporter._calculate_diversity([{"output": "x"}])
        assert metric.passed is False
        assert "没有有效问题" in metric.description

    def test_diversity_metric_empty(self):
        """空数据多样性应返回 0"""
        reporter = QualityReporter()
        metric = reporter._calculate_diversity([])
        assert metric.value == 0
        assert metric.passed is False

    def test_length_distribution_metric(self, sample_dataset):
        """长度分布指标应计算标准差分数"""
        reporter = QualityReporter()
        metric = reporter._calculate_length_distribution(sample_dataset)
        assert metric.name == "长度分布"
        assert 0.0 <= metric.value <= 1.0

    def test_length_distribution_metric_uniform_lengths(self):
        """长度完全一致时多样性分数应为 1"""
        reporter = QualityReporter()
        items = [{"instruction": "固定长度文本"}] * 3
        metric = reporter._calculate_length_distribution(items)
        assert metric.value == pytest.approx(1.0)
        assert metric.passed is True

    def test_duplication_rate_metric_duplicates(self):
        """有重复时重复率指标应反映唯一比例"""
        reporter = QualityReporter()
        items = [
            {"instruction": "问题一"},
            {"instruction": "问题一"},
            {"instruction": "问题二"},
        ]
        metric = reporter._calculate_duplication_rate(items)
        assert metric.value == pytest.approx(2 / 3)

    def test_duplication_rate_metric_empty(self):
        """空数据重复率默认通过"""
        reporter = QualityReporter()
        metric = reporter._calculate_duplication_rate([])
        assert metric.value == 1.0
        assert metric.passed is True

    def test_quality_metric_to_dict(self):
        """QualityMetric.to_dict 应包含全部字段"""
        metric = QualityMetric(
            name="完整性", value=0.8, threshold=0.7, passed=True,
            description="ok", recommendation="保持"
        )
        d = metric.to_dict()
        assert d["name"] == "完整性"
        assert d["value"] == 0.8
        assert d["passed"] is True
    
    def test_calculate_completeness(self, sample_dataset):
        """测试计算完整性指标"""
        reporter = QualityReporter()
        metric = reporter._calculate_completeness(sample_dataset)
        
        assert isinstance(metric, QualityMetric)
        assert metric.name == "完整性"
        assert metric.value >= 0
        assert metric.value <= 1
    
    def test_calculate_consistency(self, sample_dataset):
        """测试计算一致性指标"""
        reporter = QualityReporter()
        metric = reporter._calculate_consistency(sample_dataset)
        
        assert isinstance(metric, QualityMetric)
        assert metric.name == "一致性"
        assert metric.value >= 0
        assert metric.value <= 1
    
    def test_calculate_diversity(self, sample_dataset):
        """测试计算多样性指标"""
        reporter = QualityReporter()
        metric = reporter._calculate_diversity(sample_dataset)
        
        assert isinstance(metric, QualityMetric)
        assert metric.name == "多样性"
        assert metric.value >= 0
        assert metric.value <= 1
    
    def test_calculate_length_distribution(self, sample_dataset):
        """测试计算长度分布指标"""
        reporter = QualityReporter()
        metric = reporter._calculate_length_distribution(sample_dataset)
        
        assert isinstance(metric, QualityMetric)
        assert metric.name == "长度分布"
        assert metric.value >= 0
        assert metric.value <= 1
    
    def test_calculate_duplication_rate(self, sample_dataset):
        """测试计算重复率指标"""
        reporter = QualityReporter()
        metric = reporter._calculate_duplication_rate(sample_dataset)
        
        assert isinstance(metric, QualityMetric)
        assert metric.name == "重复率"
        assert metric.value >= 0
        assert metric.value <= 1


class TestQualityReport:
    """QualityReport 测试"""
    
    def test_to_dict(self, sample_dataset):
        """测试转换为字典"""
        report = generate_quality_report(sample_dataset, "test")
        d = report.to_dict()
        
        assert isinstance(d, dict)
        assert "dataset_name" in d
        assert "metrics" in d
        assert "overall_score" in d
    
    def test_to_markdown(self, sample_dataset):
        """测试转换为Markdown"""
        report = generate_quality_report(sample_dataset, "test")
        md = report.to_markdown()
        
        assert isinstance(md, str)
        assert "数据集质量报告" in md
        assert "test" in md


class TestConvenienceFunctions:
    """便捷函数测试"""
    
    def test_generate_quality_report(self, sample_dataset):
        """测试生成质量报告"""
        report = generate_quality_report(sample_dataset, "test")
        
        assert isinstance(report, QualityReport)
        assert report.dataset_name == "test"
    
    def test_save_quality_report_json(self, sample_dataset, tmp_path):
        """测试保存JSON格式报告"""
        report = generate_quality_report(sample_dataset, "test")
        output_path = tmp_path / "report.json"
        
        save_quality_report(report, str(output_path), "json")
        
        assert output_path.exists()
        
        with open(output_path, 'r', encoding='utf-8') as f:
            data = json.load(f)
        
        assert data["dataset_name"] == "test"
    
    def test_save_quality_report_markdown(self, sample_dataset, tmp_path):
        """测试保存Markdown格式报告"""
        report = generate_quality_report(sample_dataset, "test")
        output_path = tmp_path / "report.md"
        
        save_quality_report(report, str(output_path), "markdown")
        
        assert output_path.exists()
        
        with open(output_path, 'r', encoding='utf-8') as f:
            content = f.read()
        
        assert "数据集质量报告" in content


class TestQualityMetric:
    """QualityMetric 测试"""
    
    def test_to_dict(self):
        """测试转换为字典"""
        metric = QualityMetric(
            name="test_metric",
            value=0.8,
            threshold=0.7,
            passed=True,
            description="测试指标",
            recommendation="测试建议"
        )
        
        d = metric.to_dict()
        
        assert d["name"] == "test_metric"
        assert d["value"] == 0.8
        assert d["passed"] is True


class TestQualityReportExtended:
    """QualityReport 扩展测试"""

    def test_report_to_dict(self, sample_dataset):
        """QualityReport.to_dict 完整性"""
        report = generate_quality_report(sample_dataset, "test")
        d = report.to_dict()
        assert "dataset_name" in d
        assert "total_items" in d
        assert "overall_score" in d
        assert "metrics" in d
        assert "timestamp" in d

    def test_report_markdown_header(self, sample_dataset):
        """报告 Markdown 格式包含标题"""
        report = generate_quality_report(sample_dataset, "test")
        md = report.to_markdown()
        assert "质量报告" in md or "quality" in md.lower()

    def test_empty_dataset_report(self):
        """空数据集报告"""
        report = generate_quality_report([], "empty")
        assert report.total_items == 0

    def test_single_item_report(self):
        """单条数据报告"""
        data = [{"instruction": "q1", "input": "", "output": "a1"}]
        report = generate_quality_report(data, "single")
        assert report.total_items == 1

    def test_quality_report_fields(self, sample_dataset):
        """QualityReport 字段检查"""
        report = generate_quality_report(sample_dataset, "test")
        assert hasattr(report, 'overall_score')
        assert hasattr(report, 'dataset_name')
        assert hasattr(report, 'total_items')
        assert hasattr(report, 'timestamp')

    def test_save_json(self, sample_dataset, tmp_path):
        """保存 JSON 格式"""
        report = generate_quality_report(sample_dataset, "test")
        path = tmp_path / "report.json"
        save_quality_report(report, str(path), "json")
        assert path.exists()

    def test_save_markdown(self, sample_dataset, tmp_path):
        """保存 Markdown 格式"""
        report = generate_quality_report(sample_dataset, "test")
        path = tmp_path / "report.md"
        save_quality_report(report, str(path), "markdown")
        assert path.exists()


class TestQualityReportPartialBranches:
    """质量报告的三条可达假支（L116，A205）

    to_markdown 无改进建议、一致性统计里半条记录（空 output）与自我重复条
    （instruction==output）——都是真实数据形态，此前只走了「有建议 / 两侧非空 /
    问答不同」那一侧（L100 终态偏支 87->96 / 217->213 / 219->213）。
    """

    def test_to_markdown_without_recommendations_skips_section(self):
        report = QualityReport(
            dataset_name="d", timestamp="t", total_items=1,
            metrics=[QualityMetric("完整性", 1.0, 0.7, True, "ok")],
            recommendations=[],
        )
        md = report.to_markdown()
        assert "## 改进建议" not in md
        assert "## 质量指标" in md

    def test_to_markdown_with_recommendations_shows_section(self):
        report = QualityReport(
            dataset_name="d", timestamp="t", total_items=1,
            recommendations=["多收集数据"],
        )
        assert "## 改进建议" in report.to_markdown()
        assert "- 多收集数据" in report.to_markdown()

    def test_consistency_counts_only_full_and_distinct_pairs(self):
        reporter = QualityReporter()
        items = [
            {"instruction": "问A", "input": "", "output": "答A"},   # 计入
            {"instruction": "问B", "input": "", "output": ""},       # output 空 ⇒ 不计（217 假支）
            {"instruction": "重复", "input": "", "output": "重复"},   # 问答相同 ⇒ 不计（219 假支）
        ]
        metric = reporter._calculate_consistency(items)
        # 3 条里只有 1 条既完整又问答不同
        assert abs(metric.value - 1 / 3) < 1e-9
