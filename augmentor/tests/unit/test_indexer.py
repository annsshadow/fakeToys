# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""数据集索引模块测试"""

import pytest
from augmentor.indexer import (
    DatasetIndexer, DatasetView, QueryResult,
    create_indexer, create_view, IndexType
)


@pytest.fixture
def sample_dataset():
    """创建测试数据集"""
    return [
        {"instruction": "如何申请租房？", "input": "", "output": "请登录官网申请"},
        {"instruction": "租房需要什么材料？", "input": "", "output": "身份证、工作证明"},
        {"instruction": "租房流程是什么？", "input": "", "output": "选房、签约、付款"},
        {"instruction": "如何申请买房？", "input": "", "output": "请咨询销售"},
        {"instruction": "买房需要什么材料？", "input": "", "output": "身份证、收入证明"},
    ]


class TestDatasetIndexer:
    """DatasetIndexer 测试"""
    
    def test_init(self, sample_dataset):
        """测试初始化"""
        indexer = DatasetIndexer(sample_dataset)
        assert len(indexer._items) == 5
    
    def test_load(self, sample_dataset):
        """测试加载数据"""
        indexer = DatasetIndexer()
        indexer.load(sample_dataset)
        assert len(indexer._items) == 5
    
    def test_search_exact(self, sample_dataset):
        """测试精确搜索"""
        indexer = DatasetIndexer(sample_dataset)
        results = indexer.search_exact("instruction", "如何申请租房？")
        assert results == [0]  # 中文值小写不变，去重后每个索引只记录一次
    
    def test_search_contains(self, sample_dataset):
        """测试包含搜索"""
        indexer = DatasetIndexer(sample_dataset)
        results = indexer.search_contains("instruction", "租房")
        assert len(results) == 3
    
    def test_search_ngram(self, sample_dataset):
        """测试n-gram搜索"""
        indexer = DatasetIndexer(sample_dataset)
        results = indexer.search_ngram("instruction", "申请", n=2)
        assert len(results) >= 2
    
    def test_search(self, sample_dataset):
        """测试通用搜索"""
        indexer = DatasetIndexer(sample_dataset)
        result = indexer.search("租房", method="contains")
        assert result.total_matches >= 2
        assert result.query_time_ms >= 0
    
    def test_get_item(self, sample_dataset):
        """测试获取单条数据"""
        indexer = DatasetIndexer(sample_dataset)
        item = indexer.get_item(0)
        assert item == sample_dataset[0]
        
        # 测试无效索引
        item = indexer.get_item(100)
        assert item is None
    
    def test_get_batch(self, sample_dataset):
        """测试获取批量数据"""
        indexer = DatasetIndexer(sample_dataset)
        items = indexer.get_batch([0, 2, 4])
        assert len(items) == 3
        assert items[0] == sample_dataset[0]
    
    def test_filter(self, sample_dataset):
        """测试过滤数据"""
        indexer = DatasetIndexer(sample_dataset)
        results = indexer.filter(lambda x: "申请" in x["instruction"])
        assert len(results) == 2
    
    def test_get_statistics(self, sample_dataset):
        """测试获取统计信息"""
        indexer = DatasetIndexer(sample_dataset)
        stats = indexer.get_statistics()
        
        assert stats["total_items"] == 5
        assert "instruction" in stats["field_statistics"]
    
    def test_list_indexes(self, sample_dataset):
        """测试列出索引"""
        indexer = DatasetIndexer(sample_dataset)
        indexes = indexer.list_indexes()
        
        assert len(indexes) > 0
        assert any(idx.index_type == IndexType.INVERTED for idx in indexes)


class TestDatasetView:
    """DatasetView 测试"""
    
    def test_init(self, sample_dataset):
        """测试初始化"""
        view = DatasetView(sample_dataset)
        assert len(view) == 5
    
    def test_getitem(self, sample_dataset):
        """测试索引访问"""
        view = DatasetView(sample_dataset)
        assert view[0] == sample_dataset[0]
        
        # 测试切片
        sliced = view[1:3]
        assert len(sliced) == 2
    
    def test_iter(self, sample_dataset):
        """测试迭代"""
        view = DatasetView(sample_dataset)
        items = list(view)
        assert len(items) == 5
    
    def test_head(self, sample_dataset):
        """测试获取前N条"""
        view = DatasetView(sample_dataset)
        head = view.head(2)
        assert len(head) == 2
        assert head.name == "default_head"
    
    def test_tail(self, sample_dataset):
        """测试获取后N条"""
        view = DatasetView(sample_dataset)
        tail = view.tail(2)
        assert len(tail) == 2
    
    def test_sample(self, sample_dataset):
        """测试随机采样"""
        view = DatasetView(sample_dataset)
        sampled = view.sample(2, seed=42)
        assert len(sampled) == 2
    
    def test_filter(self, sample_dataset):
        """测试过滤"""
        view = DatasetView(sample_dataset)
        filtered = view.filter(lambda x: "买房" in x["instruction"])
        assert len(filtered) == 2
    
    def test_search(self, sample_dataset):
        """测试搜索"""
        view = DatasetView(sample_dataset)
        result = view.search("租房")
        assert result.total_matches >= 2
    
    def test_to_list(self, sample_dataset):
        """测试转换为列表"""
        view = DatasetView(sample_dataset)
        items = view.to_list()
        assert items == sample_dataset
    
    def test_to_file(self, sample_dataset, tmp_path):
        """测试保存到文件"""
        view = DatasetView(sample_dataset)
        output_file = tmp_path / "output.json"
        view.to_file(str(output_file))
        
        assert output_file.exists()
        
        import json
        with open(output_file, 'r', encoding='utf-8') as f:
            data = json.load(f)
        assert len(data) == 5


class TestConvenienceFunctions:
    """便捷函数测试"""
    
    def test_create_indexer(self, sample_dataset):
        """测试创建索引器"""
        indexer = create_indexer(sample_dataset)
        assert isinstance(indexer, DatasetIndexer)
        assert len(indexer._items) == 5
    
    def test_create_view(self, sample_dataset):
        """测试创建视图"""
        view = create_view(sample_dataset, "test")
        assert isinstance(view, DatasetView)
        assert view.name == "test"


class TestIndexerEdgeCases:
    """索引器边界测试"""

    def test_init_empty(self):
        """空数据集初始化"""
        indexer = DatasetIndexer()
        assert len(indexer._items) == 0
        assert indexer.get_statistics()["total_items"] == 0

    def test_search_exact_nonexistent_field(self):
        """搜索不存在的字段"""
        indexer = DatasetIndexer([{"instruction": "test"}])
        results = indexer.search_exact("nonexistent", "test")
        assert results == []

    def test_search_exact_case_insensitive(self):
        """精确搜索应不区分大小写"""
        items = [{"instruction": "Hello World"}]
        indexer = DatasetIndexer(items)
        results = indexer.search_exact("instruction", "hello world")
        assert len(results) >= 1

    def test_search_ngram_no_index(self):
        """搜索不存在的 ngram 索引"""
        indexer = DatasetIndexer([{"instruction": "test"}])
        results = indexer.search_ngram("nonexistent", "test")
        assert results == []

    def test_search_ngram_short_text(self):
        """短文本 ngram 搜索"""
        items = [{"instruction": "ab"}]
        indexer = DatasetIndexer(items)
        results = indexer.search_ngram("instruction", "ab", n=2)
        assert len(results) == 1

    def test_get_batch_invalid_indices(self):
        """无效索引应被跳过"""
        items = [{"instruction": "a"}, {"instruction": "b"}]
        indexer = DatasetIndexer(items)
        results = indexer.get_batch([0, 100, 1])
        assert len(results) == 2

    def test_get_item_negative_index(self):
        """负索引应返回 None"""
        items = [{"instruction": "test"}]
        indexer = DatasetIndexer(items)
        assert indexer.get_item(-1) is None

    def test_statistics_empty(self):
        """空数据集统计"""
        indexer = DatasetIndexer([])
        stats = indexer.get_statistics()
        assert stats["total_items"] == 0
        assert stats["field_statistics"] == {}

    def test_list_indexes_empty(self):
        """空数据集索引列表"""
        indexer = DatasetIndexer([])
        indexes = indexer.list_indexes()
        assert len(indexes) == 0


class TestDatasetViewExtended:
    """DatasetView 扩展测试"""

    def test_len(self, sample_dataset):
        """__len__ 应返回数据数量"""
        view = DatasetView(sample_dataset)
        assert len(view) == 5

    def test_items_property(self, sample_dataset):
        """items 属性应返回数据列表"""
        view = DatasetView(sample_dataset)
        assert view.items == sample_dataset

    def test_tail_more_than_available(self):
        """tail 请求超过数据量时应返回全部"""
        items = [{"instruction": "a"}]
        view = DatasetView(items)
        tail = view.tail(10)
        assert len(tail) == 1

    def test_head_more_than_available(self):
        """head 请求超过数据量时应返回全部"""
        items = [{"instruction": "a"}]
        view = DatasetView(items)
        head = view.head(10)
        assert len(head) == 1

    def test_to_file_jsonl(self, sample_dataset, tmp_path):
        """保存为 JSONL 格式"""
        view = DatasetView(sample_dataset)
        output_file = tmp_path / "output.jsonl"
        view.to_file(str(output_file), format="jsonl")
        
        assert output_file.exists()
        with open(output_file, 'r', encoding='utf-8') as f:
            lines = f.readlines()
        assert len(lines) == 5

    def test_slice_returns_view(self, sample_dataset):
        """切片应返回 DatasetView"""
        view = DatasetView(sample_dataset)
        sliced = view[1:3]
        assert isinstance(sliced, DatasetView)
        assert sliced.name == "default_slice"

    def test_filter_returns_view(self, sample_dataset):
        """过滤应返回 DatasetView"""
        view = DatasetView(sample_dataset)
        filtered = view.filter(lambda x: True)
        assert isinstance(filtered, DatasetView)
        assert filtered.name == "default_filtered"

    def test_head_returns_view(self, sample_dataset):
        """head 应返回 DatasetView"""
        view = DatasetView(sample_dataset)
        head = view.head(2)
        assert isinstance(head, DatasetView)

    def test_tail_returns_view(self, sample_dataset):
        """tail 应返回 DatasetView"""
        view = DatasetView(sample_dataset)
        tail = view.tail(2)
        assert isinstance(tail, DatasetView)


class TestIndexerExtended2:
    """索引器第二轮扩展测试"""

    def test_indexer_empty_items(self):
        """空项目索引器"""
        indexer = DatasetIndexer([])
        stats = indexer.get_statistics()
        assert stats["total_items"] == 0

    def test_view_filter_all_items(self):
        """过滤全部项目"""
        view = DatasetView([{"a": 1}])
        filtered = view.filter(lambda x: True)
        assert len(filtered) == 1

    def test_view_filter_no_items(self):
        """过滤无项目"""
        view = DatasetView([{"a": 1}])
        filtered = view.filter(lambda x: False)
        assert len(filtered) == 0
