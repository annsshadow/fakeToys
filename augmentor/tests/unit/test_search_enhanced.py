# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""数据集搜索增强模块测试"""

import re
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
        assert stats["total_items"] == 5

    def test_search_no_results(self, sample_dataset):
        """搜索无结果"""
        searcher = EnhancedSearcher(sample_dataset)
        result = searcher.search("不存在的关键词", method="contains")
        assert result.total_matches == 0

    def test_search_invalid_regex(self, sample_dataset):
        """无效正则表达式 - 返回空结果"""
        searcher = EnhancedSearcher(sample_dataset)
        result = searcher.search("[invalid", method="regex")
        assert result.total_matches == 0

    def test_search_with_empty_data(self):
        """空数据集搜索"""
        searcher = EnhancedSearcher()
        result = searcher.search("test")
        assert result.total_matches == 0

    def test_search_exact_no_match(self, sample_dataset):
        """精确搜索无匹配"""
        searcher = EnhancedSearcher(sample_dataset)
        result = searcher.search("不存在", method="exact")
        assert result.total_matches == 0

    def test_search_fuzzy_no_match(self, sample_dataset):
        """模糊搜索无匹配"""
        searcher = EnhancedSearcher(sample_dataset)
        result = searcher.search("xyz123", method="fuzzy")
        assert result.total_matches == 0

    def test_search_with_filter_no_match(self, sample_dataset):
        """带过滤器搜索无匹配"""
        searcher = EnhancedSearcher(sample_dataset)
        filters = [SearchFilter(field="instruction", operator="contains", value="不存在")]
        result = searcher.search("租房", filters=filters)
        assert result.total_matches == 0

    def test_create_searcher(self, sample_dataset):
        """create_searcher 工厂函数"""
        searcher = create_searcher(sample_dataset)
        assert isinstance(searcher, EnhancedSearcher)
        assert len(searcher._items) == 5
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


class TestEvaluateFilter:
    """过滤操作符测试"""

    def test_eq_operator(self):
        """eq 操作符"""
        searcher = EnhancedSearcher([{"status": "active"}])
        assert searcher._evaluate_filter("active", "eq", "active") is True
        assert searcher._evaluate_filter("active", "eq", "inactive") is False

    def test_ne_operator(self):
        """ne 操作符"""
        searcher = EnhancedSearcher([{"status": "active"}])
        assert searcher._evaluate_filter("active", "ne", "inactive") is True
        assert searcher._evaluate_filter("active", "ne", "active") is False

    def test_contains_operator(self):
        """contains 操作符"""
        searcher = EnhancedSearcher([{"text": "hello world"}])
        assert searcher._evaluate_filter("hello world", "contains", "hello") is True
        assert searcher._evaluate_filter("hello world", "contains", "xyz") is False
        assert searcher._evaluate_filter(123, "contains", "1") is False

    def test_gt_operator(self):
        """gt 操作符"""
        searcher = EnhancedSearcher([{"count": 10}])
        assert searcher._evaluate_filter(10, "gt", 5) is True
        assert searcher._evaluate_filter(10, "gt", 10) is False
        assert searcher._evaluate_filter("10", "gt", 5) is False

    def test_lt_operator(self):
        """lt 操作符"""
        searcher = EnhancedSearcher([{"count": 5}])
        assert searcher._evaluate_filter(5, "lt", 10) is True
        assert searcher._evaluate_filter(5, "lt", 5) is False
        assert searcher._evaluate_filter("5", "lt", 10) is False

    def test_gte_operator(self):
        """gte 操作符"""
        searcher = EnhancedSearcher([{"count": 10}])
        assert searcher._evaluate_filter(10, "gte", 10) is True
        assert searcher._evaluate_filter(10, "gte", 5) is True
        assert searcher._evaluate_filter(10, "gte", 15) is False

    def test_lte_operator(self):
        """lte 操作符"""
        searcher = EnhancedSearcher([{"count": 5}])
        assert searcher._evaluate_filter(5, "lte", 5) is True
        assert searcher._evaluate_filter(5, "lte", 10) is True
        assert searcher._evaluate_filter(5, "lte", 3) is False

    def test_in_operator(self):
        """in 操作符"""
        searcher = EnhancedSearcher([{"status": "active"}])
        assert searcher._evaluate_filter("active", "in", ["active", "pending"]) is True
        assert searcher._evaluate_filter("active", "in", ["inactive"]) is False
        assert searcher._evaluate_filter("active", "in", "not_a_list") is False

    def test_not_in_operator(self):
        """not_in 操作符"""
        searcher = EnhancedSearcher([{"status": "active"}])
        assert searcher._evaluate_filter("active", "not_in", ["inactive"]) is True
        assert searcher._evaluate_filter("active", "not_in", ["active"]) is False
        assert searcher._evaluate_filter("active", "not_in", "not_a_list") is True

    def test_unknown_operator(self):
        """未知操作符应返回 False"""
        searcher = EnhancedSearcher([{"field": "value"}])
        assert searcher._evaluate_filter("value", "unknown", "value") is False


class TestSearchMethodsExtended:
    """搜索方法扩展测试"""

    def test_search_exact_no_match(self):
        """精确搜索无匹配"""
        searcher = EnhancedSearcher([{"instruction": "hello"}])
        result = searcher.search("xyz", method="exact")
        assert result.total_matches == 0

    def test_search_contains_case_insensitive(self):
        """包含搜索应不区分大小写"""
        searcher = EnhancedSearcher([{"instruction": "Hello World"}])
        result = searcher.search("hello", method="contains")
        assert result.total_matches == 1

    def test_search_fuzzy_no_match(self):
        """模糊搜索无匹配"""
        searcher = EnhancedSearcher([{"instruction": "hello"}])
        result = searcher.search("xyz123", method="fuzzy")
        assert result.total_matches == 0

    def test_search_regex_invalid_pattern(self):
        """无效正则应返回空结果"""
        searcher = EnhancedSearcher([{"instruction": "hello"}])
        result = searcher.search("[invalid", method="regex")
        assert result.total_matches == 0

    def test_search_no_items(self):
        """空数据集搜索"""
        searcher = EnhancedSearcher([])
        result = searcher.search("test")
        assert result.total_matches == 0

    def test_search_highlights(self):
        """高亮应包含匹配信息"""
        items = [{"instruction": "如何申请租房"}]
        searcher = EnhancedSearcher(items)
        result = searcher.search("申请", method="contains")
        
        assert len(result.highlights) > 0
        assert result.highlights[0]["index"] == 0

    def test_search_with_multiple_filters(self):
        """多个过滤器应同时生效"""
        items = [
            {"instruction": "申请租房", "category": "rent"},
            {"instruction": "申请买房", "category": "buy"},
            {"instruction": "租房流程", "category": "rent"},
        ]
        searcher = EnhancedSearcher(items)
        
        filters = [
            SearchFilter(field="category", operator="eq", value="rent"),
            SearchFilter(field="instruction", operator="contains", value="申请"),
        ]
        result = searcher.search("租房", filters=filters)
        
        assert result.total_matches == 1
        assert result.items[0]["category"] == "rent"

    def test_build_indexes_non_string(self):
        """非字符串值不应被索引"""
        items = [{"count": 123, "name": "test"}]
        searcher = EnhancedSearcher(items)
        
        assert "name" in searcher._indexes
        assert "count" not in searcher._indexes

    def test_search_pagination(self):
        """分页应正确工作"""
        items = [{"instruction": f"question {i}"} for i in range(10)]
        searcher = EnhancedSearcher(items)
        
        result = searcher.search("question", limit=3, offset=5)
        assert len(result.items) <= 3

    def test_statistics_after_load(self):
        """加载数据后统计信息应更新"""
        searcher = EnhancedSearcher()
        assert searcher.get_statistics()["total_items"] == 0
        
        searcher.load([{"instruction": "test"}])
        assert searcher.get_statistics()["total_items"] == 1
