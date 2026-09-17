"""数据集统计模块测试"""

import pytest
from augmentor.statistics import (
    DatasetStatisticsCalculator, DatasetStatistics, FieldStatistics,
    calculate_statistics, get_field_summary
)


@pytest.fixture
def sample_dataset():
    """创建测试数据集"""
    return [
        {"instruction": "如何申请租房？", "input": "", "output": "请登录官网申请"},
        {"instruction": "租房需要什么材料？", "input": "", "output": "身份证、工作证明"},
        {"instruction": "租房流程是什么？", "input": "", "output": "选房、签约、付款"},
    ]


@pytest.fixture
def empty_dataset():
    """创建空数据集"""
    return []


class TestDatasetStatisticsCalculator:
    """DatasetStatisticsCalculator 测试"""
    
    def test_init(self):
        """测试初始化"""
        calculator = DatasetStatisticsCalculator()
        assert len(calculator._items) == 0
    
    def test_init_with_data(self, sample_dataset):
        """测试初始化带数据"""
        calculator = DatasetStatisticsCalculator(sample_dataset, "test")
        assert len(calculator._items) == 3
        assert calculator._dataset_name == "test"
    
    def test_load(self, sample_dataset):
        """测试加载数据"""
        calculator = DatasetStatisticsCalculator()
        calculator.load(sample_dataset, "test")
        assert len(calculator._items) == 3
    
    def test_calculate(self, sample_dataset):
        """测试计算统计信息"""
        calculator = DatasetStatisticsCalculator(sample_dataset, "test")
        stats = calculator.calculate()
        
        assert isinstance(stats, DatasetStatistics)
        assert stats.total_items == 3
        assert stats.dataset_name == "test"
        assert len(stats.field_statistics) > 0
    
    def test_calculate_empty_dataset(self, empty_dataset):
        """测试计算空数据集统计信息"""
        calculator = DatasetStatisticsCalculator(empty_dataset, "empty")
        stats = calculator.calculate()
        
        assert stats.total_items == 0
    
    def test_calculate_field_statistics(self, sample_dataset):
        """测试计算字段统计信息"""
        calculator = DatasetStatisticsCalculator(sample_dataset, "test")
        field_stats = calculator._calculate_field_statistics("instruction")
        
        assert isinstance(field_stats, FieldStatistics)
        assert field_stats.field_name == "instruction"
        assert field_stats.total_count == 3
        assert field_stats.filled_count == 3
        assert field_stats.empty_count == 0
    
    def test_calculate_content_statistics(self, sample_dataset):
        """测试计算内容统计信息"""
        calculator = DatasetStatisticsCalculator(sample_dataset, "test")
        content_stats = calculator._calculate_content_statistics()
        
        assert isinstance(content_stats, dict)
        assert "text_length" in content_stats
        assert "vocabulary" in content_stats
    
    def test_calculate_quality_metrics(self, sample_dataset):
        """测试计算质量指标"""
        calculator = DatasetStatisticsCalculator(sample_dataset, "test")
        quality_metrics = calculator._calculate_quality_metrics()
        
        assert isinstance(quality_metrics, dict)
        assert "completeness" in quality_metrics
        assert "diversity" in quality_metrics
        assert "consistency" in quality_metrics


class TestFieldStatistics:
    """FieldStatistics 测试"""
    
    def test_to_dict(self):
        """测试转换为字典"""
        stats = FieldStatistics(
            field_name="test",
            total_count=100,
            filled_count=80,
            empty_count=20,
            avg_length=10.5,
            min_length=2,
            max_length=20,
            median_length=10.0,
            std_deviation=3.2,
            unique_count=50,
            top_values=[("value1", 10), ("value2", 5)]
        )
        
        d = stats.to_dict()
        
        assert d["field_name"] == "test"
        assert d["total_count"] == 100
        assert d["filled_count"] == 80
        assert d["empty_count"] == 20
        assert d["fill_rate"] == 0.8
        assert d["avg_length"] == 10.5


class TestDatasetStatistics:
    """DatasetStatistics 测试"""
    
    def test_to_dict(self, sample_dataset):
        """测试转换为字典"""
        calculator = DatasetStatisticsCalculator(sample_dataset, "test")
        stats = calculator.calculate()
        d = stats.to_dict()
        
        assert isinstance(d, dict)
        assert "dataset_name" in d
        assert "total_items" in d
        assert "field_statistics" in d
        assert "quality_metrics" in d


class TestConvenienceFunctions:
    """便捷函数测试"""
    
    def test_calculate_statistics(self, sample_dataset):
        """测试计算统计信息"""
        stats = calculate_statistics(sample_dataset, "test")
        
        assert isinstance(stats, DatasetStatistics)
        assert stats.total_items == 3
    
    def test_get_field_summary(self, sample_dataset):
        """测试获取字段摘要"""
        summary = get_field_summary(sample_dataset, "instruction")
        
        assert isinstance(summary, dict)
        assert "field_name" in summary
        assert summary["field_name"] == "instruction"
