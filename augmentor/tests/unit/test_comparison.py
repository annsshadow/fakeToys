# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""数据集对比模块测试"""

import json
import pytest
from pathlib import Path
from augmentor.comparison import DatasetComparator, ComparisonResult, compare_datasets


@pytest.fixture
def dataset_a():
    """创建测试数据集A"""
    return [
        {"instruction": "如何申请租房？", "input": "", "output": "请登录官网申请"},
        {"instruction": "租房需要什么材料？", "input": "", "output": "身份证、工作证明"},
        {"instruction": "租房流程是什么？", "input": "", "output": "选房、签约、付款"},
    ]


@pytest.fixture
def dataset_b():
    """创建测试数据集B"""
    return [
        {"instruction": "怎么申请租房呢？", "input": "", "output": "请登录官网申请"},
        {"instruction": "租房需要哪些材料？", "input": "", "output": "身份证、工作证明"},
        {"instruction": "租房的流程是怎样的？", "input": "", "output": "选房、签约、付款"},
        {"instruction": "如何支付房租？", "input": "", "output": "支持多种支付方式"},
    ]


@pytest.fixture
def temp_dataset_files(tmp_path, dataset_a, dataset_b):
    """创建临时数据集文件"""
    file_a = tmp_path / "dataset_a.json"
    file_b = tmp_path / "dataset_b.json"
    
    with open(file_a, 'w', encoding='utf-8') as f:
        json.dump(dataset_a, f, ensure_ascii=False)
    
    with open(file_b, 'w', encoding='utf-8') as f:
        json.dump(dataset_b, f, ensure_ascii=False)
    
    return str(file_a), str(file_b)


class TestDatasetComparator:
    """DatasetComparator 测试"""
    
    def test_init(self):
        """测试初始化"""
        comparator = DatasetComparator(
            quality_threshold=0.7,
            dedup_threshold=0.85
        )
        assert comparator.quality_scorer.threshold == 0.7
        assert comparator.deduplicator.threshold == 0.85
    
    def test_compare_basic(self, dataset_a, dataset_b):
        """测试基本对比功能"""
        comparator = DatasetComparator()
        result = comparator.compare(dataset_a, dataset_b, "A", "B")
        
        assert isinstance(result, ComparisonResult)
        assert result.dataset_a_name == "A"
        assert result.dataset_b_name == "B"
        assert result.dataset_a_count == 3
        assert result.dataset_b_count == 4
    
    def test_compare_quality_metrics(self, dataset_a, dataset_b):
        """测试质量指标计算"""
        comparator = DatasetComparator()
        result = comparator.compare(dataset_a, dataset_b)
        
        # 验证质量指标存在
        assert "avg_score" in result.quality_a
        assert "pass_rate" in result.quality_a
        assert "avg_score" in result.quality_b
        assert "pass_rate" in result.quality_b
        
        # 验证质量差异计算
        assert "avg_score" in result.quality_diff
    
    def test_compare_length_metrics(self, dataset_a, dataset_b):
        """测试长度指标计算"""
        comparator = DatasetComparator()
        result = comparator.compare(dataset_a, dataset_b)
        
        # 验证长度指标存在
        assert "avg" in result.length_a
        assert "min" in result.length_a
        assert "max" in result.length_a
        
        assert "avg" in result.length_b
        assert "min" in result.length_b
        assert "max" in result.length_b
    
    def test_compare_dedup_metrics(self, dataset_a, dataset_b):
        """测试去重指标计算"""
        comparator = DatasetComparator()
        result = comparator.compare(dataset_a, dataset_b)
        
        # 验证去重指标存在
        assert "duplication_rate" in result.dedup_a
        assert "unique_count" in result.dedup_a
        assert "duplication_rate" in result.dedup_b
        assert "unique_count" in result.dedup_b
    
    def test_compare_vocabulary(self, dataset_a, dataset_b):
        """测试词汇量计算"""
        comparator = DatasetComparator()
        result = comparator.compare(dataset_a, dataset_b)
        
        # 验证词汇量指标存在
        assert "unique_chars" in result.vocabulary_a
        assert "total_chars" in result.vocabulary_a
        assert "unique_chars" in result.vocabulary_b
        assert "total_chars" in result.vocabulary_b
    
    def test_compare_winner(self, dataset_a, dataset_b):
        """测试获胜者判断"""
        comparator = DatasetComparator()
        result = comparator.compare(dataset_a, dataset_b)
        
        # 验证获胜者存在
        assert result.winner in ["a", "b", "tie"]
    
    def test_compare_summary(self, dataset_a, dataset_b):
        """测试摘要生成"""
        comparator = DatasetComparator()
        result = comparator.compare(dataset_a, dataset_b)
        
        # 验证摘要存在
        assert isinstance(result.summary, str)
        assert len(result.summary) > 0
        assert "数据集对比" in result.summary
    
    def test_save_comparison(self, tmp_path, dataset_a, dataset_b):
        """测试保存对比结果"""
        comparator = DatasetComparator()
        result = comparator.compare(dataset_a, dataset_b)
        
        output_path = tmp_path / "comparison_result.json"
        comparator.save_comparison(result, str(output_path))
        
        # 验证文件存在
        assert output_path.exists()
        
        # 验证文件内容
        with open(output_path, 'r', encoding='utf-8') as f:
            saved_data = json.load(f)
        
        assert saved_data["dataset_a_name"] == result.dataset_a_name
        assert saved_data["dataset_b_name"] == result.dataset_b_name
        assert saved_data["winner"] == result.winner


class TestCompareDatasets:
    """compare_datasets 函数测试"""
    
    def test_compare_datasets_files(self, temp_dataset_files):
        """测试从文件对比数据集"""
        file_a, file_b = temp_dataset_files
        
        result = compare_datasets(file_a, file_b, "A", "B")
        
        assert isinstance(result, ComparisonResult)
        assert result.dataset_a_count == 3
        assert result.dataset_b_count == 4
    
    def test_compare_datasets_with_output(self, tmp_path, temp_dataset_files):
        """测试带输出保存的对比"""
        file_a, file_b = temp_dataset_files
        output_path = tmp_path / "result.json"
        
        result = compare_datasets(file_a, file_b, "A", "B", str(output_path))
        
        assert output_path.exists()
        assert isinstance(result, ComparisonResult)


class TestComparisonEdgeCases:
    """边界情况测试"""
    
    def test_empty_datasets(self):
        """测试空数据集对比"""
        comparator = DatasetComparator()
        result = comparator.compare([], [], "Empty A", "Empty B")
        
        assert result.dataset_a_count == 0
        assert result.dataset_b_count == 0
        assert result.quality_a["avg_score"] == 0
        assert result.quality_b["avg_score"] == 0
    
    def test_single_item_datasets(self):
        """测试单条数据对比"""
        data_a = [{"instruction": "问题A", "input": "", "output": "回答A"}]
        data_b = [{"instruction": "问题B", "input": "", "output": "回答B"}]
        
        comparator = DatasetComparator()
        result = comparator.compare(data_a, data_b)
        
        assert result.dataset_a_count == 1
        assert result.dataset_b_count == 1
    
    def test_identical_datasets(self):
        """测试相同数据集对比"""
        data = [
            {"instruction": "问题1", "input": "", "output": "回答1"},
            {"instruction": "问题2", "input": "", "output": "回答2"},
        ]
        
        comparator = DatasetComparator()
        result = comparator.compare(data, data, "Same", "Same")
        
        assert result.winner == "tie"
        assert result.quality_diff["avg_score"] == 0


class TestComputeMethods:
    """各计算方法单独测试"""

    def test_compute_quality_metrics_empty(self):
        """空数据集质量指标应返回零值"""
        comparator = DatasetComparator()
        metrics = comparator._compute_quality_metrics([])
        
        assert metrics["avg_score"] == 0.0
        assert metrics["pass_rate"] == 0.0
        assert metrics["min_score"] == 0.0
        assert metrics["max_score"] == 0.0

    def test_compute_length_metrics_empty(self):
        """空数据集长度指标应返回零值"""
        comparator = DatasetComparator()
        metrics = comparator._compute_length_metrics([])
        
        assert metrics["avg"] == 0
        assert metrics["min"] == 0
        assert metrics["max"] == 0
        assert metrics["std"] == 0

    def test_compute_length_metrics_custom_key(self):
        """自定义字段长度指标"""
        items = [{"question": "短"}, {"question": "这是一个较长的问题"}]
        comparator = DatasetComparator()
        metrics = comparator._compute_length_metrics(items, key="question")
        
        assert metrics["min"] == len("短")
        assert metrics["max"] == len("这是一个较长的问题")
        assert metrics["avg"] > 0

    def test_compute_vocabulary_size_empty(self):
        """空数据集词汇量应为 0"""
        comparator = DatasetComparator()
        vocab = comparator._compute_vocabulary_size([])
        
        assert vocab["unique_chars"] == 0
        assert vocab["total_chars"] == 0

    def test_compute_vocabulary_size(self):
        """词汇量计算应正确"""
        items = [{"instruction": "abc"}, {"instruction": "abd"}]
        comparator = DatasetComparator()
        vocab = comparator._compute_vocabulary_size(items)
        
        assert vocab["unique_chars"] == 4  # a, b, c, d
        assert vocab["total_chars"] == 6   # 3 + 3

    def test_compute_dedup_metrics_single_item(self):
        """单条数据去重指标"""
        comparator = DatasetComparator()
        metrics = comparator._compute_dedup_metrics([{"instruction": "test"}])
        
        assert metrics["duplication_rate"] == 0.0
        assert metrics["unique_count"] == 1

    def test_generate_summary_contains_sections(self):
        """摘要应包含所有部分"""
        comparator = DatasetComparator()
        data_a = [{"instruction": "问题A", "output": "回答A"}]
        data_b = [{"instruction": "问题B", "output": "回答B"}]
        result = comparator.compare(data_a, data_b, "DS-A", "DS-B")
        
        assert "基本信息" in result.summary
        assert "质量对比" in result.summary
        assert "长度对比" in result.summary
        assert "结论" in result.summary
        assert "DS-A" in result.summary
        assert "DS-B" in result.summary

    def test_save_comparison_creates_parent_dirs(self, tmp_path, dataset_a, dataset_b):
        """保存对比结果应自动创建父目录"""
        comparator = DatasetComparator()
        result = comparator.compare(dataset_a, dataset_b)
        
        output_path = tmp_path / "subdir" / "nested" / "result.json"
        comparator.save_comparison(result, str(output_path))
        
        assert output_path.exists()
