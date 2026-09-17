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
