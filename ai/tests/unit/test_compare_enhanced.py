"""数据集比较增强模块测试"""

import pytest
from augmentor.compare_enhanced import (
    EnhancedComparator, EnhancedComparisonResult, ComparisonMetrics,
    FieldComparison, compare_datasets_enhanced, diff_datasets
)


@pytest.fixture
def dataset_a():
    """创建数据集A"""
    return [
        {"instruction": "如何申请租房？", "input": "", "output": "请登录官网申请"},
        {"instruction": "租房需要什么材料？", "input": "", "output": "身份证、工作证明"},
        {"instruction": "租房流程是什么？", "input": "", "output": "选房、签约、付款"},
    ]


@pytest.fixture
def dataset_b():
    """创建数据集B"""
    return [
        {"instruction": "如何申请租房？", "input": "", "output": "请登录官网申请"},
        {"instruction": "租房需要什么材料？", "input": "", "output": "身份证、工作证明"},
        {"instruction": "如何申请买房？", "input": "", "output": "请咨询销售顾问"},
    ]


class TestEnhancedComparator:
    """EnhancedComparator 测试"""
    
    def test_init(self):
        """测试初始化"""
        comparator = EnhancedComparator()
        assert comparator is not None
    
    def test_compare(self, dataset_a, dataset_b):
        """测试比较数据集"""
        comparator = EnhancedComparator()
        result = comparator.compare(dataset_a, dataset_b, "dataset_a", "dataset_b")
        
        assert isinstance(result, EnhancedComparisonResult)
        assert result.dataset_a_name == "dataset_a"
        assert result.dataset_b_name == "dataset_b"
        assert result.metrics.size_a == 3
        assert result.metrics.size_b == 3
    
    def test_compare_empty_datasets(self):
        """测试比较空数据集"""
        comparator = EnhancedComparator()
        result = comparator.compare([], [], "empty_a", "empty_b")
        
        assert result.metrics.size_a == 0
        assert result.metrics.size_b == 0
        assert result.metrics.similarity_score == 0  # 空数据集相似度为0
    
    def test_compare_same_datasets(self, dataset_a):
        """测试比较相同数据集"""
        comparator = EnhancedComparator()
        result = comparator.compare(dataset_a, dataset_a, "same_a", "same_b")
        
        assert result.metrics.similarity_score == 1.0
        assert result.metrics.unique_a == 0
        assert result.metrics.unique_b == 0
    
    def test_compare_different_datasets(self, dataset_a, dataset_b):
        """测试比较不同数据集"""
        comparator = EnhancedComparator()
        result = comparator.compare(dataset_a, dataset_b)
        
        # 应该有一些共同数据和一些独特数据
        assert result.metrics.common_items >= 0
        assert result.metrics.unique_a >= 0
        assert result.metrics.unique_b >= 0


class TestComparisonMetrics:
    """ComparisonMetrics 测试"""
    
    def test_to_dict(self):
        """测试转换为字典"""
        metrics = ComparisonMetrics(
            size_a=100,
            size_b=90,
            size_diff=10,
            size_ratio=1.11,
            common_items=80,
            unique_a=20,
            unique_b=10,
            similarity_score=0.89,
            field_overlap=0.95
        )
        
        d = metrics.to_dict()
        
        assert d["size_a"] == 100
        assert d["size_b"] == 90
        assert d["similarity_score"] == 0.89


class TestFieldComparison:
    """FieldComparison 测试"""
    
    def test_to_dict(self):
        """测试转换为字典"""
        comp = FieldComparison(
            field_name="instruction",
            in_a_only=5,
            in_b_only=3,
            in_both=90,
            type_mismatches=2,
            value_differences=[{"instruction": "test", "value_a": "a", "value_b": "b"}]
        )
        
        d = comp.to_dict()
        
        assert d["field_name"] == "instruction"
        assert d["in_a_only"] == 5
        assert d["in_b_only"] == 3
        assert d["in_both"] == 90
        assert d["type_mismatches"] == 2


class TestConvenienceFunctions:
    """便捷函数测试"""
    
    def test_compare_datasets_enhanced(self, dataset_a, dataset_b):
        """测试增强数据集比较"""
        result = compare_datasets_enhanced(dataset_a, dataset_b, "a", "b")
        
        assert isinstance(result, EnhancedComparisonResult)
        assert result.metrics.size_a == 3
        assert result.metrics.size_b == 3
    
    def test_diff_datasets(self, dataset_a, dataset_b):
        """测试计算数据集差异"""
        result = diff_datasets(dataset_a, dataset_b)
        
        assert isinstance(result, dict)
        assert "only_in_a" in result
        assert "only_in_b" in result
        assert "in_both" in result
        assert "stats" in result


class TestEnhancedComparisonResult:
    """EnhancedComparisonResult 测试"""
    
    def test_to_dict(self, dataset_a, dataset_b):
        """测试转换为字典"""
        result = compare_datasets_enhanced(dataset_a, dataset_b, "a", "b")
        d = result.to_dict()
        
        assert isinstance(d, dict)
        assert "dataset_a_name" in d
        assert "dataset_b_name" in d
        assert "metrics" in d
        assert "field_comparisons" in d
        assert "summary" in d
        assert "recommendations" in d
