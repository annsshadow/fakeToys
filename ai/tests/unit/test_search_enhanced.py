"""数据集搜索增强模块测试"""

import pytest
from augmentor.search_enhanced import (
    EnhancedSearcher, SearchResult, SearchFilter,
    search_dataset, create_searcher
)


@pytest.fixture
def sample_dataset():
    """创建测试数据集"""
    return [
        {"instruction": "如何申请租房？", "input": "", "output": "请登录官网申请"},
        {"instruction": "租房需要什么材料？", "input": "", "output": "身份证、工作证明"},
        {"instruction": "租房流程是什么？", "input": "", "output": "选房、签约、付款"},
        {"instruction": "如何申请买房？", "input": "", "output": "请咨询销售顾问"},
        {"instruction": "买房需要什么材料？", "input": "", "output": "身份证、收入证明"},
    ]


class TestEnhancedSearcher:
    """EnhancedSearcher 测试"""
    
    def test_init(self):
        """测试初始化"""
        searcher = EnhancedSearcher()
        assert len(searcher._items) == 0
    
    def test_init_with_data(self, sample_dataset):
        """测试初始化带数据"""
        searcher = EnhancedSearcher(sample_dataset)
        assert len(searcher._items) == 5
    
    def test_load(self, sample_dataset):
        """测试加载数据"""
        searcher = EnhancedSearcher()
        searcher.load(sample_dataset)
        assert len(searcher._items) == 5
    
    def test_search_contains(self, sample_dataset):
        """测试包含搜索"""
        searcher = EnhancedSearcher(sample_dataset)
        result = searcher.search("租房", method="contains")
        
        assert isinstance(result, SearchResult)
        assert result.total_matches >= 2
        assert result.method == "contains"
    
    def test_search_exact(self, sample_dataset):
        """测试精确搜索"""
        searcher = EnhancedSearcher(sample_dataset)
        result = searcher.search("如何申请租房？", method="exact")
        
        assert isinstance(result, SearchResult)
    
    def test_search_fuzzy(self, sample_dataset):
        """测试模糊搜索"""
        searcher = EnhancedSearcher(sample_dataset)
        result = searcher.search("申请租房", method="fuzzy")
        
        assert isinstance(result, SearchResult)
    
    def test_search_regex(self, sample_dataset):
        """测试正则表达式搜索"""
        searcher = EnhancedSearcher(sample_dataset)
        result = searcher.search("申请.*租房", method="regex")
        
        assert isinstance(result, SearchResult)
        assert result.total_matches >= 1  # 至少匹配一个
    
    def test_search_with_limit(self, sample_dataset):
        """测试限制搜索结果"""
        searcher = EnhancedSearcher(sample_dataset)
        result = searcher.search("租房", limit=2)
        
        assert len(result.items) <= 2
    
    def test_search_with_offset(self, sample_dataset):
        """测试偏移搜索"""
        searcher = EnhancedSearcher(sample_dataset)
        result = searcher.search("租房", offset=1, limit=2)
        
        assert isinstance(result, SearchResult)
    
    def test_search_with_filter(self, sample_dataset):
        """测试带过滤器搜索"""
        searcher = EnhancedSearcher(sample_dataset)
        
        filters = [SearchFilter(field="instruction", operator="contains", value="申请")]
        result = searcher.search("租房", filters=filters)
        
        assert isinstance(result, SearchResult)
        # 应该只返回包含"申请"的结果
        for item in result.items:
            assert "申请" in item.get("instruction", "")
    
    def test_get_statistics(self, sample_dataset):
        """测试获取统计信息"""
        searcher = EnhancedSearcher(sample_dataset)
        stats = searcher.get_statistics()
        
        assert "total_items" in stats
        assert "indexed_fields" in stats
        assert stats["total_items"] == 5


class TestSearchFilter:
    """SearchFilter 测试"""
    
    def test_to_dict(self):
        """测试转换为字典"""
        filter_item = SearchFilter(
            field="instruction",
            operator="contains",
            value="租房"
        )
        
        d = filter_item.to_dict()
        
        assert d["field"] == "instruction"
        assert d["operator"] == "contains"
        assert d["value"] == "租房"


class TestConvenienceFunctions:
    """便捷函数测试"""
    
    def test_search_dataset(self, sample_dataset):
        """测试搜索数据集"""
        result = search_dataset(sample_dataset, "租房")
        
        assert isinstance(result, SearchResult)
        assert result.total_matches >= 2
    
    def test_create_searcher(self, sample_dataset):
        """测试创建搜索器"""
        searcher = create_searcher(sample_dataset)
        
        assert isinstance(searcher, EnhancedSearcher)
        assert len(searcher._items) == 5


class TestSearchResult:
    """SearchResult 测试"""
    
    def test_to_dict(self, sample_dataset):
        """测试转换为字典"""
        result = search_dataset(sample_dataset, "租房")
        d = result.to_dict()
        
        assert isinstance(d, dict)
        assert "items" in d
        assert "total_matches" in d
        assert "query" in d
