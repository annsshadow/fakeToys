"""数据集操作工具模块测试"""

import json
import pytest
from pathlib import Path
from augmentor.dataset_ops import (
    DatasetOperations, MergeConfig, SampleConfig, SplitConfig,
    merge_datasets, sample_dataset as sample_dataset_func, split_dataset
)


@pytest.fixture
def test_data():
    """创建测试数据集"""
    return [
        {"instruction": "如何申请租房？", "input": "", "output": "请登录官网申请"},
        {"instruction": "租房需要什么材料？", "input": "", "output": "身份证、工作证明"},
        {"instruction": "租房流程是什么？", "input": "", "output": "选房、签约、付款"},
        {"instruction": "如何支付房租？", "input": "", "output": "支持多种支付方式"},
        {"instruction": "租房合同怎么签？", "input": "", "output": "在线签署电子合同"},
    ]


@pytest.fixture
def temp_dataset_file(tmp_path, test_data):
    """创建临时数据集文件"""
    file_path = tmp_path / "dataset.json"
    with open(file_path, 'w', encoding='utf-8') as f:
        json.dump(test_data, f, ensure_ascii=False)
    return str(file_path)


class TestDatasetOperations:
    """DatasetOperations 测试"""
    
    def test_merge_basic(self, test_data):
        """测试基本合并功能"""
        ops = DatasetOperations()
        dataset_b = [
            {"instruction": "问题6", "input": "", "output": "回答6"},
            {"instruction": "问题7", "input": "", "output": "回答7"},
        ]
        
        merged = ops.merge([test_data, dataset_b])
        assert len(merged) == 7
    
    def test_merge_with_dedup(self, test_data):
        """测试带去重的合并"""
        ops = DatasetOperations()
        dataset_b = [
            {"instruction": "如何申请租房？", "input": "", "output": "请登录官网申请"},  # 重复
            {"instruction": "问题7", "input": "", "output": "回答7"},
        ]
        
        config = MergeConfig(deduplicate=True)
        merged = ops.merge([test_data, dataset_b], config)
        
        # 去重后应该只有6条（原5条 + 1条新数据）
        assert len(merged) == 6
    
    def test_merge_max_items(self, test_data):
        """测试限制最大条数"""
        ops = DatasetOperations()
        dataset_b = [
            {"instruction": "问题6", "input": "", "output": "回答6"},
            {"instruction": "问题7", "input": "", "output": "回答7"},
        ]
        
        config = MergeConfig(max_items=5)
        merged = ops.merge([test_data, dataset_b], config)
        
        assert len(merged) == 5
    
    def test_sample_random(self, test_data):
        """测试随机采样"""
        ops = DatasetOperations()
        config = SampleConfig(method="random", size=3, seed=42)
        
        sampled = ops.sample(test_data, config)
        assert len(sampled) == 3
    
    def test_sample_by_ratio(self, test_data):
        """测试按比例采样"""
        ops = DatasetOperations()
        config = SampleConfig(ratio=0.6, seed=42)
        
        sampled = ops.sample(test_data, config)
        assert len(sampled) == 3  # 5 * 0.6 = 3
    
    def test_sample_systematic(self, test_data):
        """测试系统采样"""
        ops = DatasetOperations()
        config = SampleConfig(method="systematic", size=3)
        
        sampled = ops.sample(test_data, config)
        assert len(sampled) == 3
    
    def test_split_basic(self, test_data):
        """测试基本分割功能"""
        ops = DatasetOperations()
        config = SplitConfig(ratios=(0.6, 0.2, 0.2), shuffle=False)
        
        train, val, test = ops.split(test_data, config)
        
        assert len(train) == 3
        assert len(val) == 1
        assert len(test) == 1
    
    def test_split_with_shuffle(self, test_data):
        """测试打乱分割"""
        ops = DatasetOperations()
        config = SplitConfig(ratios=(0.6, 0.2, 0.2), shuffle=True, seed=42)
        
        train, val, test = ops.split(test_data, config)
        
        assert len(train) + len(val) + len(test) == len(test_data)
    
    def test_split_invalid_ratios(self, test_data):
        """测试无效分割比例"""
        ops = DatasetOperations()
        config = SplitConfig(ratios=(0.5, 0.3, 0.3))  # 总和 > 1
        
        with pytest.raises(ValueError):
            ops.split(test_data, config)
    
    def test_shuffle(self, test_data):
        """测试打乱顺序"""
        ops = DatasetOperations()
        shuffled = ops.shuffle(test_data, seed=42)
        
        assert len(shuffled) == len(test_data)
        # 内容相同但顺序可能不同
        assert set(item["instruction"] for item in shuffled) == set(item["instruction"] for item in test_data)
    
    def test_head(self, test_data):
        """测试获取前N条"""
        ops = DatasetOperations()
        head = ops.head(test_data, n=3)
        
        assert len(head) == 3
        assert head == test_data[:3]
    
    def test_tail(self, test_data):
        """测试获取后N条"""
        ops = DatasetOperations()
        tail = ops.tail(test_data, n=3)
        
        assert len(tail) == 3
        assert tail == test_data[-3:]
    
    def test_filter_by_length(self, test_data):
        """测试按长度过滤"""
        ops = DatasetOperations()
        filtered = ops.filter_by_length(test_data, min_length=5, max_length=10)
        
        for item in filtered:
            assert 5 <= len(item["instruction"]) <= 10
    
    def test_filter_by_keyword(self, test_data):
        """测试按关键词过滤"""
        ops = DatasetOperations()
        filtered = ops.filter_by_keyword(test_data, ["租房"], mode="include")
        
        for item in filtered:
            assert "租房" in item["instruction"]
    
    def test_get_statistics(self, test_data):
        """测试获取统计信息"""
        ops = DatasetOperations()
        stats = ops.get_statistics(test_data)
        
        assert stats["total"] == 5
        assert "avg_length" in stats
        assert "min_length" in stats
        assert "max_length" in stats
        assert "unique_instructions" in stats


class TestConvenienceFunctions:
    """便捷函数测试"""
    
    def test_merge_files(self, tmp_path, test_data):
        """测试合并文件"""
        # 创建两个数据集文件
        file_a = tmp_path / "a.json"
        file_b = tmp_path / "b.json"
        
        with open(file_a, 'w', encoding='utf-8') as f:
            json.dump(test_data[:3], f, ensure_ascii=False)
        
        with open(file_b, 'w', encoding='utf-8') as f:
            json.dump(test_data[3:], f, ensure_ascii=False)
        
        output = tmp_path / "merged.json"
        result = merge_datasets([str(file_a), str(file_b)], str(output))
        
        assert result["input_files"] == 2
        assert result["total_output"] == 5
        assert output.exists()
    
    def test_sample_file(self, tmp_path, test_data):
        """测试采样文件"""
        input_file = tmp_path / "input.json"
        output_file = tmp_path / "output.json"
        
        with open(input_file, 'w', encoding='utf-8') as f:
            json.dump(test_data, f, ensure_ascii=False)
        
        result = sample_dataset_func(str(input_file), str(output_file), size=3, seed=42)
        
        assert result["input_count"] == 5
        assert result["output_count"] == 3
        assert output_file.exists()
    
    def test_split_file(self, tmp_path, test_data):
        """测试分割文件"""
        input_file = tmp_path / "input.json"
        output_dir = tmp_path / "splits"
        
        with open(input_file, 'w', encoding='utf-8') as f:
            json.dump(test_data, f, ensure_ascii=False)
        
        result = split_dataset(str(input_file), str(output_dir), seed=42)
        
        assert "splits" in result
        assert (output_dir / "train.json").exists()
        assert (output_dir / "val.json").exists()
        assert (output_dir / "test.json").exists()


class TestDatasetOperationsEdgeCases:
    """边界情况测试"""
    
    def test_merge_empty_datasets(self):
        """测试合并空数据集"""
        ops = DatasetOperations()
        merged = ops.merge([[], []])
        assert len(merged) == 0
    
    def test_sample_larger_than_dataset(self, test_data):
        """测试采样数量大于数据集"""
        ops = DatasetOperations()
        config = SampleConfig(size=100)
        
        sampled = ops.sample(test_data, config)
        assert len(sampled) == len(test_data)
    
    def test_get_statistics_empty(self):
        """测试空数据集统计"""
        ops = DatasetOperations()
        stats = ops.get_statistics([])
        
        assert stats["total"] == 0


class TestDatasetOperationsExtended:
    """DatasetOperations 扩展测试"""

    def test_merge_single_dataset(self):
        """合并单个数据集"""
        ops = DatasetOperations()
        data = [{"instruction": "q1"}, {"instruction": "q2"}]
        merged = ops.merge([data])
        assert len(merged) == 2

    def test_sample_random_with_seed(self, test_data):
        """随机采样可复现性"""
        ops = DatasetOperations()
        config = SampleConfig(method="random", size=3, seed=42)
        sampled1 = ops.sample(test_data, config)
        sampled2 = ops.sample(test_data, config)
        assert [item["instruction"] for item in sampled1] == [item["instruction"] for item in sampled2]

    def test_sample_stratified(self, test_data):
        """分层采样"""
        ops = DatasetOperations()
        config = SampleConfig(method="stratified", size=3, stratify_key="instruction")
        sampled = ops.sample(test_data, config)
        assert len(sampled) == 3

    def test_split_all_to_train(self, test_data):
        """全部分到训练集"""
        ops = DatasetOperations()
        config = SplitConfig(ratios=(1.0, 0.0, 0.0))
        train, val, test = ops.split(test_data, config)
        assert len(train) == len(test_data)
        assert len(val) == 0
        assert len(test) == 0

    def test_filter_by_keyword_exclude(self, test_data):
        """排除关键词过滤"""
        ops = DatasetOperations()
        filtered = ops.filter_by_keyword(test_data, ["租房"], mode="exclude")
        for item in filtered:
            assert "租房" not in item["instruction"]

    def test_filter_by_keyword_empty(self, test_data):
        """空关键词列表过滤应返回全部"""
        ops = DatasetOperations()
        filtered = ops.filter_by_keyword(test_data, [], mode="include")
        assert len(filtered) == 0

    def test_merge_with_max_items_and_dedup(self, test_data):
        """同时使用 max_items 和 dedup"""
        ops = DatasetOperations()
        dataset_b = [{"instruction": "如何申请租房？"}]  # duplicate
        config = MergeConfig(max_items=3, deduplicate=True)
        merged = ops.merge([test_data, dataset_b], config)
        assert len(merged) <= 3

    def test_merge_files_empty(self, tmp_path):
        """合并空文件"""
        file_a = tmp_path / "empty.json"
        file_a.write_text("[]", encoding="utf-8")
        output = tmp_path / "merged.json"
        result = merge_datasets([str(file_a)], str(output))
        assert result["total_output"] == 0
