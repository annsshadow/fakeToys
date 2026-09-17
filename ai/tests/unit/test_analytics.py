"""数据分析模块测试"""

import pytest
from augmentor.analytics import (
    DatasetAnalyzer, AnalysisReport, DataInsight, TextStatistics,
    analyze_dataset, get_dataset_insights
)


@pytest.fixture
def sample_dataset():
    """创建测试数据集"""
    return [
        {"instruction": "如何申请租房？", "input": "", "output": "请登录官网申请租房服务"},
        {"instruction": "租房需要什么材料？", "input": "", "output": "需要身份证、工作证明、收入证明等材料"},
        {"instruction": "租房流程是什么？", "input": "", "output": "选房、签约、付款、入住"},
        {"instruction": "如何申请买房？", "input": "", "output": "请咨询销售顾问"},
        {"instruction": "买房需要什么材料？", "input": "", "output": "需要身份证、户口本、收入证明、征信报告等"},
    ]


@pytest.fixture
def empty_dataset():
    """创建空数据集"""
    return []


@pytest.fixture
def minimal_dataset():
    """创建最小数据集"""
    return [
        {"instruction": "问题", "input": "", "output": "回答"},
    ]


class TestDatasetAnalyzer:
    """DatasetAnalyzer 测试"""
    
    def test_init(self):
        """测试初始化"""
        analyzer = DatasetAnalyzer()
        assert len(analyzer._items) == 0
    
    def test_init_with_data(self, sample_dataset):
        """测试初始化带数据"""
        analyzer = DatasetAnalyzer(sample_dataset)
        assert len(analyzer._items) == 5
    
    def test_load(self, sample_dataset):
        """测试加载数据"""
        analyzer = DatasetAnalyzer()
        analyzer.load(sample_dataset)
        assert len(analyzer._items) == 5
    
    def test_analyze(self, sample_dataset):
        """测试分析数据集"""
        analyzer = DatasetAnalyzer(sample_dataset)
        report = analyzer.analyze()
        
        assert isinstance(report, AnalysisReport)
        assert report.dataset_size == 5
        assert "instruction" in report.field_statistics
        assert report.quality_score >= 0
        assert report.diversity_score >= 0
        assert report.completeness_score >= 0
    
    def test_analyze_empty_dataset(self, empty_dataset):
        """测试分析空数据集"""
        analyzer = DatasetAnalyzer(empty_dataset)
        report = analyzer.analyze()
        
        assert report.dataset_size == 0
        assert report.quality_score == 0
    
    def test_calculate_text_statistics(self, sample_dataset):
        """测试计算文本统计信息"""
        analyzer = DatasetAnalyzer(sample_dataset)
        texts = [item["instruction"] for item in sample_dataset]
        stats = analyzer._calculate_text_statistics(texts)
        
        assert isinstance(stats, TextStatistics)
        assert stats.total_chars > 0
        assert stats.avg_chars > 0
        assert stats.vocabulary_size > 0
    
    def test_generate_insights(self, sample_dataset):
        """测试生成洞察"""
        analyzer = DatasetAnalyzer(sample_dataset)
        report = analyzer.analyze()
        
        assert isinstance(report.insights, list)
        assert len(report.insights) >= 0
    
    def test_get_duplicate_candidates(self, sample_dataset):
        """测试获取重复候选"""
        analyzer = DatasetAnalyzer(sample_dataset)
        candidates = analyzer.get_duplicate_candidates(threshold=0.5)
        
        assert isinstance(candidates, list)
    
    def test_get_quality_distribution(self, sample_dataset):
        """测试获取质量分布"""
        analyzer = DatasetAnalyzer(sample_dataset)
        distribution = analyzer.get_quality_distribution()
        
        assert isinstance(distribution, dict)
        assert sum(distribution.values()) == len(sample_dataset)
    
    def test_get_category_distribution(self, sample_dataset):
        """测试获取类别分布"""
        analyzer = DatasetAnalyzer(sample_dataset)
        distribution = analyzer.get_category_distribution()
        
        assert isinstance(distribution, dict)
        assert sum(distribution.values()) == len(sample_dataset)


class TestConvenienceFunctions:
    """便捷函数测试"""
    
    def test_analyze_dataset(self, sample_dataset):
        """测试分析数据集"""
        report = analyze_dataset(sample_dataset)
        
        assert isinstance(report, AnalysisReport)
        assert report.dataset_size == 5
    
    def test_get_dataset_insights(self, sample_dataset):
        """测试获取数据集洞察"""
        insights = get_dataset_insights(sample_dataset)
        
        assert isinstance(insights, list)
    
    def test_analyze_empty_dataset(self, empty_dataset):
        """测试分析空数据集"""
        report = analyze_dataset(empty_dataset)
        
        assert report.dataset_size == 0


class TestAnalysisReport:
    """AnalysisReport 测试"""
    
    def test_to_dict(self, sample_dataset):
        """测试转换为字典"""
        report = analyze_dataset(sample_dataset)
        d = report.to_dict()
        
        assert isinstance(d, dict)
        assert "dataset_size" in d
        assert "field_statistics" in d
        assert "insights" in d
        assert "scores" in d
