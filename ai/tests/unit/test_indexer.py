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
        assert len(results) == 2  # 原始值和小写版本
        assert 0 in results
    
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
