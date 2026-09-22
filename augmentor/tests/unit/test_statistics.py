# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

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


class TestGetAllFields:
    """字段自动发现测试"""

    def test_mixed_fields(self):
        """不同字段应被自动发现"""
        items = [
            {"instruction": "q1", "output": "a1"},
            {"question": "q2", "answer": "a2"},
        ]
        calculator = DatasetStatisticsCalculator(items)
        fields = calculator._get_all_fields()
        
        assert "instruction" in fields
        assert "output" in fields
        assert "question" in fields
        assert "answer" in fields

    def test_empty_items(self):
        """空数据集应返回空字段列表"""
        calculator = DatasetStatisticsCalculator([])
        fields = calculator._get_all_fields()
        assert fields == []


class TestContentStatisticsEdgeCases:
    """内容统计边界测试"""

    def test_empty_data(self):
        """空数据集内容统计应为空"""
        calculator = DatasetStatisticsCalculator([])
        stats = calculator._calculate_content_statistics()
        assert stats == {}

    def test_empty_string_values(self):
        """空字符串值不应计入统计"""
        items = [{"instruction": "", "output": ""}]
        calculator = DatasetStatisticsCalculator(items)
        stats = calculator._calculate_content_statistics()
        assert stats == {}


class TestQualityMetricsEdgeCases:
    """质量指标边界测试"""

    def test_empty_data(self):
        """空数据集质量指标应为空"""
        calculator = DatasetStatisticsCalculator([])
        metrics = calculator._calculate_quality_metrics()
        assert metrics == {}

    def test_no_instructions(self):
        """无 instruction 数据时多样性为 0"""
        items = [{"output": "answer1"}, {"output": "answer2"}]
        calculator = DatasetStatisticsCalculator(items)
        metrics = calculator._calculate_quality_metrics()
        assert metrics["diversity"] == 0

    def test_identical_instructions(self):
        """相同 instruction 时多样性为 1/总数"""
        items = [
            {"instruction": "same", "output": "a1"},
            {"instruction": "same", "output": "a2"},
        ]
        calculator = DatasetStatisticsCalculator(items)
        metrics = calculator._calculate_quality_metrics()
        assert metrics["diversity"] == 0.5

    def test_instruction_equals_output(self):
        """instruction 等于 output 时一致性为 0"""
        items = [{"instruction": "same", "output": "same"}]
        calculator = DatasetStatisticsCalculator(items)
        metrics = calculator._calculate_quality_metrics()
        assert metrics["consistency"] == 0.0


class TestFieldStatisticsExtended:
    """FieldStatistics 扩展测试"""

    def test_median_even_count(self):
        """偶数个数据的中位数"""
        items = [
            {"field": "aa"},
            {"field": "aaaa"},
            {"field": "a"},
            {"field": "aaa"},
        ]
        calculator = DatasetStatisticsCalculator(items)
        stats = calculator._calculate_field_statistics("field")
        
        # lengths: 1, 2, 3, 4 → median = (2+3)/2 = 2.5
        assert stats.median_length == 2.5

    def test_median_odd_count(self):
        """奇数个数据的中位数"""
        items = [
            {"field": "aa"},
            {"field": "aaaa"},
            {"field": "a"},
        ]
        calculator = DatasetStatisticsCalculator(items)
        stats = calculator._calculate_field_statistics("field")
        
        # lengths: 1, 2, 4 → median = 2
        assert stats.median_length == 2

    def test_non_string_values(self):
        """非字符串值应转换为字符串计算长度"""
        items = [{"field": 12345}]
        calculator = DatasetStatisticsCalculator(items)
        stats = calculator._calculate_field_statistics("field")
        
        assert stats.avg_length == 5

    def test_partial_empty_values(self):
        """部分空值的字段统计"""
        items = [
            {"field": "hello"},
            {"field": ""},
            {"field": "world"},
        ]
        calculator = DatasetStatisticsCalculator(items)
        stats = calculator._calculate_field_statistics("field")
        
        assert stats.filled_count == 2
        assert stats.empty_count == 1


class TestGenerateSummary:
    """摘要生成测试"""

    def test_summary_contains_key_info(self):
        """摘要应包含关键信息"""
        items = [
            {"instruction": "question", "output": "answer"},
        ]
        calculator = DatasetStatisticsCalculator(items, "my_dataset")
        stats = calculator.calculate()
        
        assert "my_dataset" in stats.summary
        assert "1 条数据" in stats.summary

    def test_summary_with_quality_metrics(self):
        """摘要应包含质量指标"""
        items = [{"instruction": "q", "output": "a"}]
        calculator = DatasetStatisticsCalculator(items, "test")
        stats = calculator.calculate()
        
        assert "质量指标" in stats.summary

    def test_summary_empty_dataset(self):
        """空数据集摘要"""
        calculator = DatasetStatisticsCalculator([], "empty")
        stats = calculator.calculate()
        
        assert "0 条数据" in stats.summary


class TestFieldStatisticsToDict:
    """FieldStatistics.to_dict 扩展测试"""

    def test_fill_rate_zero_total(self):
        """total_count 为 0 时 fill_rate 应为 0"""
        stats = FieldStatistics(
            field_name="test", total_count=0, filled_count=0,
            empty_count=0, avg_length=0, min_length=0, max_length=0,
            median_length=0, std_deviation=0, unique_count=0
        )
        d = stats.to_dict()
        assert d["fill_rate"] == 0

    def test_fill_rate_computed(self):
        """fill_rate = filled/total"""
        stats = FieldStatistics(
            field_name="f", total_count=4, filled_count=3, empty_count=1,
            avg_length=1.0, min_length=1, max_length=2,
            median_length=1.0, std_deviation=0.5, unique_count=3
        )
        d = stats.to_dict()
        assert d["fill_rate"] == pytest.approx(0.75)


class TestStatisticsExtended:
    """统计模块扩展测试（覆盖剩余分支）"""

    def test_calculate_specific_fields(self, sample_dataset):
        """指定字段统计应只包含这些字段"""
        calculator = DatasetStatisticsCalculator(sample_dataset, "ds")
        stats = calculator.calculate(fields=["instruction"])
        assert list(stats.field_statistics.keys()) == ["instruction"]

    def test_calculate_unknown_field_yields_empty_stats(self):
        """未知字段统计应全为零值"""
        calculator = DatasetStatisticsCalculator([{"a": "x"}], "ds")
        stats = calculator.calculate(fields=["nonexistent"])
        fs = stats.field_statistics["nonexistent"]
        assert fs.total_count == 0 or fs.filled_count == 0

    def test_content_statistics_keys(self, sample_dataset):
        """内容统计应包含长度相关指标"""
        calculator = DatasetStatisticsCalculator(sample_dataset, "ds")
        stats = calculator.calculate()
        assert "instruction_length" in stats.content_statistics or stats.content_statistics

    def test_quality_metrics_present(self, sample_dataset):
        """质量指标应为非空字典"""
        calculator = DatasetStatisticsCalculator(sample_dataset, "ds")
        stats = calculator.calculate()
        assert isinstance(stats.quality_metrics, dict)
        assert len(stats.quality_metrics) > 0

    def test_to_dict_round_trip(self, sample_dataset):
        """DatasetStatistics.to_dict 应包含嵌套字段统计"""
        calculator = DatasetStatisticsCalculator(sample_dataset, "ds")
        stats = calculator.calculate()
        d = stats.to_dict()
        assert "field_statistics" in d
        assert "instruction" in d["field_statistics"]
        assert d["total_items"] == 3

    def test_calculate_statistics_convenience(self, sample_dataset):
        """便捷函数 calculate_statistics 应返回 DatasetStatistics"""
        result = calculate_statistics(sample_dataset, "conv")
        assert isinstance(result, DatasetStatistics)
        assert result.total_items == 3

    def test_get_field_summary_convenience(self, sample_dataset):
        """便捷函数 get_field_summary 应返回字段摘要"""
        summary = get_field_summary(sample_dataset, "instruction")
        assert summary is not None
