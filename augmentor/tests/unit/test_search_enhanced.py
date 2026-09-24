# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""数据集搜索增强模块测试"""

import re
import pytest
from augmentor.exceptions import DataValidationError
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
        """非字符串值不应被索引

        `_ensure_indexes()` 是索引的按需入口（L13 前构造函数会无条件建好），
        所以这里显式触发一次再断言——要验的始终是「哪些值进得了索引」。
        """
        items = [{"count": 123, "name": "test"}]
        searcher = EnhancedSearcher(items)
        searcher._ensure_indexes()
        
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


class TestIndexBuiltOnDemand:
    """倒排索引只在真需要时建 —— 五种方法里只有 exact 读它

    实测：真实 6902 条数据建一份索引要 64.5 ms、常驻 6.17 MB，而 `search_dataset()`
    （CLI 与 `/api/dataset/search` 的唯一入口）**每次调用都新建一个搜索器**，于是
    默认的 contains 查询也要先付这 64.5 ms：端到端 80.85 ms 里查询本身只占 7.5 ms。
    预言机用「构建次数」而不是计时——墙钟在这台机器上不可复现（L12 已坐实）。
    """

    @pytest.fixture
    def build_counter(self, monkeypatch):
        """返回「每次 `_build_indexes` 记录一条条目数」的列表（装在类上，构造期也计得到）"""
        calls: list = []
        real = EnhancedSearcher._build_indexes

        def counting(self):
            calls.append(len(self._items))
            real(self)

        monkeypatch.setattr(EnhancedSearcher, "_build_indexes", counting)
        return calls

    def test_construction_does_not_build(self, sample_dataset, build_counter):
        """光构造搜索器不该动索引：缺陷态在这里就红（构造即建）"""
        EnhancedSearcher(sample_dataset)

        assert build_counter == []

    @pytest.mark.parametrize("method", ["contains", "ngram", "fuzzy", "regex"])
    def test_index_free_methods_never_build(self, sample_dataset, build_counter, method):
        """不读索引的四种方法，一条查询都不该触发构建"""
        searcher = EnhancedSearcher(sample_dataset)
        result = searcher.search("租房", method=method, limit=3)

        assert build_counter == [], f"{method} 查询白建了 {len(build_counter)} 次索引"
        assert result.total_matches >= 0

    def test_search_dataset_entry_point_skips_index_for_default_method(self, sample_dataset, build_counter):
        """真实入口 `search_dataset()` 走默认 contains 时零构建

        这条盯的是「每次调用新建搜索器」这个用法本身：只要构建回到构造函数，
        每条 API 搜索请求就重新付一次全表扫描。
        """
        result = search_dataset(sample_dataset, "租房")

        assert build_counter == []
        assert result.method == "contains"
        assert result.total_matches >= 2

    def test_exact_builds_once_and_reuses_across_queries(self, sample_dataset, build_counter):
        """exact 才建，且同实例连查三次只建一次"""
        searcher = EnhancedSearcher(sample_dataset)
        for _ in range(3):
            searcher.search("如何申请租房？", method="exact")

        assert build_counter == [5]

    def test_exact_matches_a_naive_scan(self, sample_dataset):
        """惰性/复用都不许改变结果：与独立写的全表扫描逐条对照

        预言机是朴素 `lower()` 相等判断，不走被测库的索引，所以「索引建晚了、
        建错了、复用了旧的」都会在这里露出来。
        """
        query = "如何申请租房？"
        expected = [
            i for i, it in enumerate(sample_dataset)
            if it.get("instruction", "").lower() == query.lower()
        ]

        searcher = EnhancedSearcher(sample_dataset)
        result = searcher.search(query, method="exact", fields=["instruction"])

        assert result.total_matches == len(expected)
        assert result.items == [sample_dataset[i] for i in expected]

    def test_load_invalidates_a_reused_searcher(self, build_counter):
        """换数据必须作废索引，否则新数据会被旧索引答成「查无此项」"""
        searcher = EnhancedSearcher([{"instruction": "只有租房"}])
        assert searcher.search("只有租房", method="exact", fields=["instruction"]).total_matches == 1

        searcher.load([{"instruction": "只有买房"}])
        assert searcher.search("只有租房", method="exact", fields=["instruction"]).total_matches == 0
        assert searcher.search("只有买房", method="exact", fields=["instruction"]).total_matches == 1
        assert build_counter == [1, 1]

    def test_get_statistics_reports_the_same_index_as_before(self, build_counter):
        """`get_statistics()` 是索引的第二个读者：统计口径不能因惰性而变空"""
        searcher = EnhancedSearcher([
            {"instruction": "how to rent", "output": "登录官网"},
            {"instruction": "how to buy", "output": "咨询顾问"},
        ])
        stats = searcher.get_statistics()

        # 手算：instruction 的键 = 两条整串 + 4 个去重词（how/to/rent/buy）；
        # output 的中文串整段成词，与整串同键，所以各 1。
        assert stats["indexed_fields"] == ["instruction", "output"]
        assert stats["field_counts"] == {"instruction": 6, "output": 2}
        assert build_counter == [2]


class TestNgramCostFollowsTheQuery:
    """ngram 打分的开销跟着**查询**走，不再跟着**文档**走

    旧实现为每条文档物化整串的 n-gram 集合（`len(query_ngrams & _ngrams(value, n))`），
    代价是每条 O(文档长度) 个字符串对象。真实 6902 条 × 3 字段实测一次
    `method="ngram"` 查询 **104~113 ms**，而同样的数据 contains 只要 **8.2 ms**——
    慢就慢在这个与查询无关的整档集合上。一个查询 gram 命中文档，定义上就等价于
    「它是文档的子串」，所以按查询侧那 k 个 gram 各做一次子串判定即可：结果逐条
    相同，单字段实测 22.2 → 3.2 ms（中位）。

    预言机用「`_ngrams` 被喂了哪些文本」这种确定性计数（L12/L17 的教训：墙钟与
    分配量都不如调用次数可复现），并用一份独立写的朴素 gram 集合对照语义。
    """

    CORPUS = [
        {"instruction": "如何申请租房？", "output": "请登录官网申请"},
        {"instruction": "租房租房租房", "output": "重复很多遍的租房"},
        {"instruction": "买房需要什么材料？", "output": "身份证、收入证明"},
        {"instruction": "AAAa", "output": "aaaa"},
        {"instruction": "", "output": "空串没有 gram"},
        {"instruction": 42, "output": None},
    ]

    @staticmethod
    def naive_gram_scores(items, field, query, n=2):
        """独立实现的参照：整档 gram 集合 + 交集（也就是被换掉的那套算法）"""
        def grams(text):
            lowered = text.lower()
            if len(lowered) < n:
                return set()
            return {lowered[i:i + n] for i in range(len(lowered) - n + 1)}

        query_grams = grams(query)
        if not query_grams:
            return {}
        scores = {}
        for index, item in enumerate(items):
            value = item.get(field, "")
            if not isinstance(value, str):
                continue
            hit = len(query_grams & grams(value))
            if hit:
                scores[index] = hit / len(query_grams)
        return scores

    @pytest.mark.parametrize("query", ["租房", "如何申请租房？", "aaa", "AAaa",
                                       "空串", "不会命中的查询"])
    @pytest.mark.parametrize("n", [2, 3])
    def test_scores_are_identical_to_a_naive_gram_set(self, query, n):
        """等价性逐条对照：索引与分数都要和旧算法一字不差

        这条是本轮的语义护栏——把子串判定换回集合交集之外的任何改动（漏掉 `lower()`、
        分母取错、跳过了非字符串分支）都会在这里红。
        """
        searcher = EnhancedSearcher(self.CORPUS)

        assert searcher._search_ngram("instruction", query, n) == \
            self.naive_gram_scores(self.CORPUS, "instruction", query, n)
        assert searcher._search_ngram("output", query, n) == \
            self.naive_gram_scores(self.CORPUS, "output", query, n)

    def test_gram_sets_are_built_for_the_query_only(self, monkeypatch):
        """计数预言机：`_ngrams` 只许见到查询串，一条文档都不许进去"""
        seen = []
        real = EnhancedSearcher._ngrams

        def spying(text, n):
            seen.append(text)
            return real(text, n)

        monkeypatch.setattr(EnhancedSearcher, "_ngrams", staticmethod(spying))
        searcher = EnhancedSearcher(self.CORPUS * 20)

        assert searcher._search_ngram("instruction", "租房")
        assert seen == ["租房"], f"文档也被建了 gram 集合：{seen[1:4]}"

    def test_a_repeated_gram_in_one_document_scores_once(self):
        """覆盖率是「命中了几个查询 gram」，不是「出现了几次」"""
        searcher = EnhancedSearcher(self.CORPUS)

        # 「租房租房租房」只有一个 distinct bigram `租房`，覆盖率仍是 1.0
        assert searcher._search_ngram("instruction", "租房") == {0: 1.0, 1: 1.0}

    def test_case_folding_reaches_both_sides(self):
        """查询与文档都小写化：大小写不敏感这件事不能只在一侧做"""
        searcher = EnhancedSearcher(self.CORPUS)

        assert searcher._search_ngram("instruction", "AAA") == {3: 1.0}
        assert searcher._search_ngram("output", "AAAA") == {3: 1.0}

    def test_a_query_shorter_than_n_matches_nothing(self):
        """单字查询（短于 n）：没有 gram 可算，返回空而不是整档扫一遍"""
        searcher = EnhancedSearcher(self.CORPUS)

        assert searcher._search_ngram("instruction", "租") == {}
        assert searcher._search_ngram("instruction", "") == {}

    def test_non_string_field_values_are_skipped(self):
        """`42` / `None` 这类字段值不参与打分，也不许把查询砸了"""
        searcher = EnhancedSearcher(self.CORPUS)

        scores = searcher._search_ngram("instruction", "身份")

        assert scores == {}
        assert searcher._search_ngram("output", "身份") == {2: 1.0}

    def test_ranking_through_the_public_entry_is_unchanged(self, sample_dataset):
        """端到端：`search(method="ngram")` 的命中数与排序口径保持原样

        换算法最坏的错误不是慢，而是把「覆盖率高」的文档排到后面去却仍然返回 200。
        """
        expected = self.naive_gram_scores(sample_dataset, "instruction", "租房流程")

        result = EnhancedSearcher(sample_dataset).search(
            "租房流程", method="ngram", fields=["instruction"])

        assert result.total_matches == len(expected)
        assert result.items == [sample_dataset[i] for i in sorted(
            expected, key=lambda i: expected[i], reverse=True)]

    def test_ngram_still_pays_no_index_build(self, monkeypatch):
        """本轮不许把倒排索引顺带建回来：ngram 与索引无关（L13 的结论）"""
        calls = []
        monkeypatch.setattr(EnhancedSearcher, "_build_indexes",
                            lambda self: calls.append(1))

        EnhancedSearcher(self.CORPUS).search("租房", method="ngram")

        assert calls == []

    def test_the_public_entry_point_accumulates_fields_identically(self):
        """真实入口（CLI 与 `/api/dataset/search` 共用）的多字段累加口径不变

        默认三字段各算一次覆盖率再**相加**，是 ngram 与其它方法能混用的前提；参照那份
        朴素实现按同样的顺序累加，绕开被测库。语料是刻意挑的：doc0 三个字段各 0.5
        （合计 1.5）、doc1 只在 instruction 上满覆盖（1.0），所以「按字段求和」排在
        doc0 在前，而「取最大字段分」或「只看第一个字段」都会把 doc1 提到前面——
        排序本身就把累加口径钉住了。
        """
        items = [{"instruction": "租房合同要点", "input": "租房补贴", "output": "签约前看清租房条款"},
                 {"instruction": "租房约定", "input": "", "output": "咨询销售"}]
        expected = {}
        for field in ("instruction", "input", "output"):
            for index, score in self.naive_gram_scores(items, field, "租房约").items():
                expected[index] = expected.get(index, 0) + score

        result = search_dataset(items, "租房约", method="ngram")

        assert result.total_matches == len(expected)
        assert result.items == [items[0], items[1]]


class TestFuzzySlidesAWindow:
    """fuzzy 从「整档 Jaccard」换成「查询作为等长窗口在文档上滑」

    旧的 `_search_fuzzy` 拿查询 token 集合与**整条文档** token 集合算 Jaccard，在真实
    6902 条中文语料上**任何查询都恒 0 命中**（A27）：`_tokenize` 把一整段连续中文当成
    一个 token，`{'租房'}` 与 `{'如何申请租房？'}` 永不相交；阈值从 0.6 一路降到 0.02
    仍然是 0，所以这不是阈值太严，是度量本身错——对称的整档相似度对「短查询 vs 长文档」
    根本不成立。新语义：`分数 = 最好的等长窗口的 1 - 错配数/查询长度`，门槛 `threshold`，
    于是 `contains` 的命中必然是 fuzzy 的子集（且得 1.0），多出来的是允许换字的近似命中。

    为了不退化成 O(文档长度 × 查询长度) 的裸扫（朴素窗口在 6902 条上实测 1.0~1.9 秒），
    实现里加了一层鸽笼过滤：错配 ≤ d 的窗口必然与查询切成 d+1 段中的某一段完全相同，
    先用 `in` 排除整条文档。**过滤是纯优化**，所以下面每一条都用一份不带过滤的朴素
    实现做等价对照——过滤掉错了就会在这里红，而不是在用户的搜索结果里静默少几条。
    """

    CORPUS = [
        {"instruction": "如何申请租房？", "output": "请登录官网申请"},
        {"instruction": "公租房补贴政策", "output": "租房补贴"},
        {"instruction": "AAAa", "output": "aaaa"},
        {"instruction": "", "output": "短"},
        {"instruction": 42, "output": None},
        {"instruction": "买房需要什么材料", "output": "身份证、收入证明"},
    ]

    @staticmethod
    def naive_window_scores(items, field, query, threshold=0.6):
        """朴素参照：把查询当窗口在文档上**逐位滑到底**，不用任何过滤

        这是被优化掉的那条直路，故意写得最笨：对每个起点数一遍错配字数，取最高分。
        与被测库唯一的差别就是「有没有鸽笼过滤」，所以两边必须逐条、逐分相同。
        """
        pattern = query.lower()
        size = len(pattern)
        scores = {}
        if not size or not 0 < threshold <= 1:
            return scores
        for index, item in enumerate(items):
            value = item.get(field, "")
            if not isinstance(value, str) or len(value) < size:
                continue
            text = value.lower()
            best = 0.0
            for start in range(len(text) - size + 1):
                window = text[start:start + size]
                mismatches = sum(1 for a, b in zip(pattern, window) if a != b)
                score = 1.0 - mismatches / size
                if score > best:
                    best = score
            if best >= threshold:
                scores[index] = best
        return scores

    @pytest.mark.parametrize("query", [
        "租房", "公租房", "租方房", "AAA", "aaaa", "身份", "不存在的东西",
    ])
    @pytest.mark.parametrize("threshold", [0.6, 1.0, 0.5])
    @pytest.mark.parametrize("field", ["instruction", "output"])
    def test_matches_a_naive_window_scan(self, query, threshold, field):
        """等价性主护栏：加过滤的实现要和朴素全扫**逐条逐分**相同

        覆盖 `threshold=1.0`（退化为 contains）、0.6（默认）、0.5（放宽）三档，
        以及中文/拉丁/大小写/空串/非字符串字段五类取值。
        """
        searcher = EnhancedSearcher(self.CORPUS)
        expected = self.naive_window_scores(self.CORPUS, field, query, threshold)

        assert searcher._search_fuzzy(field, query, threshold) == expected

    def test_contains_hits_are_a_subset_scoring_one(self):
        """`contains` 能搜到的，fuzzy 一定要能搜到且得满分

        这是新语义的**定义级**不变式：窗口完全对齐时错配数为 0。反过来不成立
        （fuzzy 多找到的就是允许换字的那批），所以两条一起才说明 fuzzy 没退化成 contains。
        """
        searcher = EnhancedSearcher(self.CORPUS)
        contains = searcher._search_contains("instruction", "公租房")
        fuzzy = searcher._search_fuzzy("instruction", "公租房")

        assert set(contains) <= set(fuzzy)
        for index in contains:
            assert fuzzy[index] == 1.0
        assert fuzzy[0] == pytest.approx(2 / 3)

    def test_a_typo_query_finds_the_record(self):
        """真实缺陷的正例：一个字不同也该命中，旧实现在这里返回空

        「公租房」查 `"如何申请租房？"`：最佳窗口是 `请租房`（3 字里 2 字相同 = 0.667），
        旧 Jaccard 因为整段中文算一个 token，交集为空 → 恒 0 命中。
        """
        searcher = EnhancedSearcher([{"instruction": "如何申请租房？"}])

        assert searcher._search_fuzzy("instruction", "公租房") == {0: pytest.approx(2 / 3)}
        assert searcher._search_fuzzy("instruction", "租房") == {0: 1.0}

    def test_the_whole_field_is_not_treated_as_one_token(self, monkeypatch):
        """新实现不许再走 `_tokenize`——那正是「整段中文当一个 token」的病根

        计数预言机：旧实现每条文档都要 `_tokenize` 两次（查询 + 文档），所以这里
        只要**一次都不许发生**。承 L17 的教训，被数的名字必须是旧路真走的那一个。
        """
        calls = []
        real = EnhancedSearcher._tokenize
        monkeypatch.setattr(EnhancedSearcher, "_tokenize",
                            lambda self, text: (calls.append(text), real(self, text))[1])
        searcher = EnhancedSearcher(self.CORPUS * 20)

        assert searcher._search_fuzzy("instruction", "租房")
        assert calls == [], f"fuzzy 又去整档切 token 了：{calls[:3]}"

    def test_threshold_outside_the_usable_range_matches_nothing(self):
        """阈值不是 `(0, 1]` 时返回空，而不是「全给 1.0」或抛异常

        `threshold=0` 旧语义等于不过滤，窗口分数会退化成整档相似度；越界的
        `1.5` / `-1` 更没有任何合理含义。三档都必须安静地返回空。
        """
        searcher = EnhancedSearcher(self.CORPUS)

        for threshold in (0, -1, 1.5):
            assert searcher._search_fuzzy("instruction", "租房", threshold) == {}

    def test_an_empty_query_matches_nothing(self):
        """空查询没有窗口可言，不许退化成「每行 0 错配 = 满分」"""
        searcher = EnhancedSearcher(self.CORPUS)

        assert searcher._search_fuzzy("instruction", "") == {}

    def test_a_query_longer_than_the_field_never_hits(self):
        """查询比字段还长：一个窗口都放不下，返回空而不是按可用长度打折"""
        searcher = EnhancedSearcher([{"instruction": "租房"}])

        assert searcher._search_fuzzy("instruction", "租房补贴政策") == {}

    def test_non_string_field_values_are_skipped(self):
        """`42` / `None` / 空串这类值不参与滑动窗口，也不许把整条查询砸了

        语料第 4 条是 `{"instruction": 42, "output": None}`、第 3 条 instruction 是空串，
        它们只能被跳过。留下的必须是真含窗口的：`如何申请租房？` 与 `公租房补贴政策`
        都含连续的「租房」（后者是「公**租房**」）→ 各 1.0，别的字段值都不许混进来。
        """
        searcher = EnhancedSearcher(self.CORPUS)

        assert searcher._search_fuzzy("instruction", "租房") == {0: 1.0, 1: 1.0}
        assert searcher._search_fuzzy("output", "租房") == {1: 1.0}

    def test_the_public_entry_reports_the_fuzzy_hits_it_found(self):
        """端到端：真实入口（CLI 与 `/api/dataset/search` 共用）不再交出 0 条

        症状就是本轮要修的：以前 fuzzy 无论查什么都返回 `total_matches=0`，
        用户以为语料里没有，其实是被度量吃掉了。
        """
        items = [{"instruction": "如何申请租房？", "output": "请登录官网申请"},
                 {"instruction": "公租房补贴政策", "output": "租房补贴"}]

        result = search_dataset(items, "公租房", method="fuzzy")

        assert result.total_matches == len(items)
        assert result.items == [items[1], items[0]]


class TestTuningKnobsReachTheScoring:
    """fuzzy 的阈值与 ngram 的 gram 长度必须是**公开可调**的

    L25 把 fuzzy 修活之后留下的口子（A28④）：`threshold` 只活在私有方法
    `_search_fuzzy` 的默认参数上，`search()` 调它时不传，`search_dataset()` 也不收，
    CLI 没有 `--fuzzy-threshold`，API 的 `SearchRequest` 没有对应字段。于是「松紧」这个
    决定结果长什么样的旋钮，用户只能改库源码——而它的影响是**成倍**的：真实 6902 条上
    「租房合同」阈值 0.5 → 1272 命中、0.6 → 159、0.8 → 38；「公租屋」0.6 → 63、0.7 → **0**
    （错 1 字的得分正好是 0.667）。这跟 A23/A27 是同一个形状：能力在库里，入口摸不到。

    语料沿用 CLI/API 集成用例那 5 条租房问答，所以本轮三个数在三种端上必须一致：
    「租房」在阈值 0.5 下 4 条、0.6 下 2 条；「腿租押金」在 0.75 下 1 条、0.8 下 0 条；
    「租房」ngram n=1 4 条、n=2 2 条、n=3 0 条。
    """

    ITEMS = [
        {"instruction": "如何申请租房？", "input": "", "output": "登录官网申请"},
        {"instruction": "租房多少钱？", "input": "", "output": "按房型定价"},
        {"instruction": "如何退租押金？", "input": "", "output": "满一年后退还"},
        {"instruction": "可以申请月付吗？", "input": "", "output": "支持月付"},
        {"instruction": "租期最短多久？", "input": "", "output": "一个月起租"},
    ]
    FIELDS = ["instruction", "output"]

    @staticmethod
    def oracle_hits(items, field, query, oracle, **kwargs):
        """把朴素 oracle 套到每条记录上，得到「该字段有命中」的下标集合

        故意不写第二份打分实现：oracle 用前两类用例各自钉过的那份（gram 集合 /
        朴素窗口），这里只负责逐条调用，免得同一条口径在三处各写一遍、
        下一轮改口径时漏改一处。
        """
        return {index for index, item in enumerate(items)
                if oracle([item], field, query, **kwargs)}

    def fuzzy_hits(self, threshold):
        """库口径：两字段并起来数命中条目"""
        searcher = EnhancedSearcher(self.ITEMS)
        result = searcher.search("腿租押金", fields=self.FIELDS, method="fuzzy",
                                 fuzzy_threshold=threshold, limit=99)
        return {item["instruction"] for item in result.items}

    def test_the_threshold_moves_the_hit_set(self):
        """阈值不是摆设：调松一档必须真的多出命中，且多出来的那些与朴素窗口一致

        「租房」在 0.6 下 2 条、0.5 下 4 条。两边都跟朴素窗口 oracle（L25 那份）对齐，
        所以这里同时钉住了三件事：旋钮走到了打分代码、走的是那份 oracle 的语义、
        以及 0.6 的命中集合是 0.5 的子集（阈值越松命中越多的单调方向不能反）。
        """
        searcher = EnhancedSearcher(self.ITEMS)
        loose = searcher.search("租房", fields=self.FIELDS, method="fuzzy",
                                fuzzy_threshold=0.5, limit=99)
        strict = searcher.search("租房", fields=self.FIELDS, method="fuzzy",
                                 fuzzy_threshold=0.6, limit=99)
        loose_set = self.oracle_hits(self.ITEMS, "instruction", "租房",
                                     TestFuzzySlidesAWindow.naive_window_scores, threshold=0.5) \
            | self.oracle_hits(self.ITEMS, "output", "租房",
                               TestFuzzySlidesAWindow.naive_window_scores, threshold=0.5)
        strict_set = self.oracle_hits(self.ITEMS, "instruction", "租房",
                                      TestFuzzySlidesAWindow.naive_window_scores, threshold=0.6) \
            | self.oracle_hits(self.ITEMS, "output", "租房",
                               TestFuzzySlidesAWindow.naive_window_scores, threshold=0.6)

        assert loose.total_matches == len(loose_set)
        assert strict.total_matches == len(strict_set)
        assert len(strict.items) < len(loose.items), "阈值调松了命中数却没变——旋钮没生效"
        assert {i["instruction"] for i in strict.items} <= {i["instruction"] for i in loose.items}

    def test_the_threshold_boundary_is_inclusive(self):
        """`分数 >= 阈值` 才算命中，等号这一侧要钉住

        「腿租押金」对「如何退租押金？」的最优窗口是「退租押金」，错 1 字 / 4 字 = 0.75。
        所以阈值取 0.75 必须**还在**、取 0.8 才走。写成 `>` 的话 0.75 会掉成 0 条，
        而这个边界恰恰是用户最常自己设的值（「允许错一个字」= 1 - 1/字数）。
        """
        assert self.fuzzy_hits(0.75) == {"如何退租押金？"}
        assert self.fuzzy_hits(0.8) == set()

    @pytest.mark.parametrize("query, method, knob", [
        ("租房", "fuzzy", "fuzzy_threshold"),
        ("月付", "fuzzy", "fuzzy_threshold"),
        ("腿租押金", "fuzzy", "fuzzy_threshold"),
        ("租房", "ngram", "ngram_n"),
        ("可以申请", "ngram", "ngram_n"),
    ])
    def test_the_default_stays_the_shipped_semantics(self, query, method, knob):
        """不传旋钮 = 显式传默认值（0.6 / 2）——新参数不得改动既有口径

        三端默认值一致是回归的前提：CLI 的 `--fuzzy-threshold`、API 的
        `SearchRequest.fuzzy_threshold` 都以这里的默认值为准。
        """
        searcher = EnhancedSearcher(self.ITEMS)
        defaults = {"fuzzy_threshold": 0.6, "ngram_n": 2}

        implicit = searcher.search(query, fields=self.FIELDS, method=method, limit=99)
        explicit = searcher.search(query, fields=self.FIELDS, method=method, limit=99,
                                   **{knob: defaults[knob]})

        assert explicit.items == implicit.items
        assert explicit.total_matches == implicit.total_matches

    def test_the_ngram_length_moves_the_hit_set(self):
        """`ngram_n` 同样要走到打分代码：n=1 逐字覆盖、n=2 二元、n=3 对 2 字查询无解

        三个数都跟 L24 那份 gram 集合 oracle 逐条对齐（不是字面量），顺带钉住
        「n 比查询还长 → gram 集合为空 → 零命中」是**定义内**的结果而非崩溃。
        """
        searcher = EnhancedSearcher(self.ITEMS)
        for n in (1, 2, 3):
            result = searcher.search("租房", fields=self.FIELDS, method="ngram",
                                     ngram_n=n, limit=99)
            expected = self.oracle_hits(self.ITEMS, "instruction", "租房",
                                        TestNgramCostFollowsTheQuery.naive_gram_scores, n=n) \
                | self.oracle_hits(self.ITEMS, "output", "租房",
                                   TestNgramCostFollowsTheQuery.naive_gram_scores, n=n)
            assert result.total_matches == len(expected), f"n={n} 与 gram 集合 oracle 不一致"
            assert {i["instruction"] for i in result.items} == \
                {self.ITEMS[index]["instruction"] for index in expected}
        assert searcher.search("租房", fields=self.FIELDS, method="ngram",
                               ngram_n=3, limit=99).total_matches == 0

    @pytest.mark.parametrize("method, knob", [
        ("contains", "fuzzy_threshold"),
        ("ngram", "fuzzy_threshold"),
        ("fuzzy", "ngram_n"),
        ("regex", "ngram_n"),
    ])
    def test_a_knob_only_bends_its_own_method(self, method, knob):
        """旋钮不许串台：fuzzy 的阈值不影响 contains/ngram，n 也不影响 fuzzy/regex

        `regex` 用「租」这种字面式（语料里含「租」的条目都算命中），否则模式里的元字符
        会让「两个结果一样」这条判断变成在测正则引擎。
        """
        searcher = EnhancedSearcher(self.ITEMS)
        query = "租" if method == "regex" else "租房"
        baseline = searcher.search(query, fields=self.FIELDS, method=method, limit=99)
        bent = searcher.search(query, fields=self.FIELDS, method=method, limit=99,
                               **{knob: 1.0 if knob == "fuzzy_threshold" else 4})

        assert bent.items == baseline.items
        assert bent.total_matches == baseline.total_matches

    @pytest.mark.parametrize("value", [0, -1, 1.5, 1.0000001])
    def test_an_out_of_range_threshold_fails_loudly(self, value):
        """阈值越界要报错，不许静默交出 0 条

        这正是 A27 的形状：`_search_fuzzy` 内部对越界值返回 `{}`，作为私有护栏没问题，
        但公开入口若跟着返回空，用户读到的是「语料里没有」，而真相是「参数写错了」。
        异常用 `DataValidationError`（与 `batch_size`/聚合策略那几处入参校验同族），
        它同时是 `ValueError` —— API 的 400 映射与 `except ValueError` 的老调用方都不断。
        """
        searcher = EnhancedSearcher(self.ITEMS)

        with pytest.raises(DataValidationError) as excinfo:
            searcher.search("租房", method="fuzzy", fuzzy_threshold=value)

        assert isinstance(excinfo.value, ValueError)
        assert "模糊阈值" in str(excinfo.value)

    @pytest.mark.parametrize("value", [0, -2, 2.5, True])
    def test_an_out_of_range_gram_length_fails_loudly(self, value):
        """`ngram_n` 必须是 >=1 的整数；`True` 这种「看起来像 1」的也一并拒掉

        允许 float 会静默走进 `text[i:i+2.5]` 的 TypeError（等于把校验推给解释器），
        允许 bool 则 `True` 悄悄当 1 用——两个都是把坏参数往下传。
        """
        searcher = EnhancedSearcher(self.ITEMS)

        with pytest.raises(DataValidationError) as excinfo:
            searcher.search("租房", method="ngram", ngram_n=value)

        assert isinstance(excinfo.value, ValueError)
        assert "n-gram 长度" in str(excinfo.value)

    def test_validation_happens_before_the_corpus_is_touched(self):
        """报错发生在打分之前：一个字段都不许扫

        计数探针把五种私有方法全部替换成记账版，坏参数若先走到分发口就会留下次数。
        探针自己也要证明是响的——所以同一处再用**合法**参数跑一遍，断言次数 > 0，
        否则「0 次」可能只是探针根本没生效（L14 的教训）。
        """
        calls = []

        class Counting(EnhancedSearcher):
            def _search_exact(self, *args, **kwargs):
                calls.append("exact")
                return {}

            def _search_contains(self, *args, **kwargs):
                calls.append("contains")
                return {}

            def _search_ngram(self, *args, **kwargs):
                calls.append("ngram")
                return {}

            def _search_fuzzy(self, *args, **kwargs):
                calls.append("fuzzy")
                return {}

            def _search_regex(self, *args, **kwargs):
                calls.append("regex")
                return {}

        searcher = Counting(self.ITEMS)
        with pytest.raises(DataValidationError):
            searcher.search("租房", fields=self.FIELDS, method="contains",
                            fuzzy_threshold=0)
        with pytest.raises(DataValidationError):
            searcher.search("租房", fields=self.FIELDS, method="ngram", ngram_n=0)
        assert calls == []

        searcher.search("租房", fields=self.FIELDS, method="contains", limit=99)
        assert calls, "计数探针没生效，上面那两条断言不算证据"

    def test_the_shared_entry_point_forwards_both_knobs(self):
        """真实入口（CLI 与 `/api/dataset/search` 共用 `search_dataset`）两个旋钮都收

        钉的是「转发」这一步：`search()` 有了参数而 `search_dataset()` 忘了传，
        上面所有用例照样绿，用户却仍然只能拿默认值。所以这里只走公开入口，
        并且拿同一份语料上「0.75 有 1 条 / 0.8 有 0 条」「n=1 有 4 条 / n=2 有 2 条」
        两组**彼此不同**的结果来证明参数真的穿过去了。
        """
        loose = search_dataset(self.ITEMS, "腿租押金", fields=self.FIELDS,
                               method="fuzzy", fuzzy_threshold=0.75)
        strict = search_dataset(self.ITEMS, "腿租押金", fields=self.FIELDS,
                                method="fuzzy", fuzzy_threshold=0.8)
        unigram = search_dataset(self.ITEMS, "租房", fields=self.FIELDS,
                                 method="ngram", ngram_n=1)
        bigram = search_dataset(self.ITEMS, "租房", fields=self.FIELDS,
                                method="ngram", ngram_n=2)

        assert loose.total_matches == 1
        assert strict.total_matches == 0
        assert unigram.total_matches > bigram.total_matches


class TestTheResultEchoesTheKnobsItUsed:
    """结果必须回显**本次真正生效**的松紧旋钮（A29）

    L26 把两个旋钮接到了三端，但 `SearchResult` 里只有 `query` / `method`，于是
    「找到 0 条」依然有两种读不出区别的成因：语料里确实没有，和旋钮拧得太紧。
    真实 6902 条上这个区别值 63 条——「公租屋」阈值 0.6 → 63 命中、0.7 → **0** 命中，
    而两者的 `method` 都是 `fuzzy`。

    口径取「未消费即为 `None`」而不是「一律回显」：contains 旁边印一个 0.6 会让人以为
    阈值管得到它，那正是 A27/A28④ 那一类「用户以为参数起了作用」的静默。
    """

    ITEMS = TestTuningKnobsReachTheScoring.ITEMS
    FIELDS = TestTuningKnobsReachTheScoring.FIELDS

    def search(self, method, **kwargs):
        return EnhancedSearcher(self.ITEMS).search(
            "腿租押金" if method == "fuzzy" else "租房",
            fields=self.FIELDS, method=method, limit=99, **kwargs)

    def test_fuzzy_says_which_threshold_produced_it(self):
        result = self.search("fuzzy", fuzzy_threshold=0.75)
        assert result.fuzzy_threshold == 0.75

    def test_ngram_says_which_gram_length_produced_it(self):
        assert self.search("ngram", ngram_n=1).ngram_n == 1
        assert self.search("ngram", ngram_n=3).ngram_n == 3

    def test_the_echo_is_the_effective_value_including_defaults(self):
        """不传旋钮时回显的是**当时生效的默认值**，不是 `None`

        「回显生效值」如果只在显式传参时成立，用户拿到 0 条时仍看不出系统用了哪一档；
        而 CLI/API 恰恰大量走默认路径。
        """
        assert self.search("fuzzy").fuzzy_threshold == 0.6
        assert self.search("ngram").ngram_n == 2

    @pytest.mark.parametrize("method", ["exact", "contains", "regex"])
    def test_a_method_that_consumes_neither_knob_echoes_neither(self, method):
        result = EnhancedSearcher(self.ITEMS).search(
            "租房", fields=self.FIELDS, method=method, limit=99)
        assert (result.fuzzy_threshold, result.ngram_n) == (None, None)

    def test_each_knob_echoes_only_for_its_own_method(self):
        """交叉两格：fuzzy 不冒充 ngram 的 `n`，ngram 也不冒充 fuzzy 的阈值"""
        assert (self.search("fuzzy").ngram_n,
                self.search("ngram").fuzzy_threshold) == (None, None)

    def test_two_zero_hit_results_now_tell_apart_a_tight_knob(self):
        """本轮的全部意义：两条 0 命中的结果**可分辨**

        「腿租押金」阈值 0.8 是「拧太紧了」，contains「腿租押金」是「语料里真没有」
        （整串不存在）。两者的条数相同、方法不同，缺陷态除此之外没有任何信息差。
        """
        tight = self.search("fuzzy", fuzzy_threshold=0.8)
        absent = EnhancedSearcher(self.ITEMS).search(
            "腿租押金", fields=self.FIELDS, method="contains", limit=99)

        assert (tight.total_matches, absent.total_matches) == (0, 0)
        assert tight.fuzzy_threshold == 0.8
        assert absent.fuzzy_threshold is None

    def test_to_dict_exports_both_knobs_without_losing_the_old_keys(self):
        """落盘 / HTTP 响应都走 `to_dict()`，所以键集合本身就是契约

        这条是 API 契约用例 `test_api_openapi_contract.py` 里 `/api/dataset/search`
        那格的来源：FastAPI 按 `response_model` 过滤返回值，模型少写一键就会**静默**丢字段。
        """
        d = self.search("fuzzy", fuzzy_threshold=0.75).to_dict()

        assert set(d) == {"items", "total_matches", "query_time_ms", "query",
                          "method", "highlights", "fuzzy_threshold", "ngram_n"}
        assert d["fuzzy_threshold"] == 0.75
        assert d["ngram_n"] is None
        assert d["method"] == "fuzzy"
        assert d["total_matches"] == 1

    def test_a_result_built_without_the_new_fields_still_works(self):
        """`SearchResult` 是导出类型，外部按前 6 个字段构造必须照旧可用（新字段有默认值）"""
        legacy = SearchResult(items=[], total_matches=0, query_time_ms=0.0,
                              query="租房", method="fuzzy", highlights=[])

        assert legacy.fuzzy_threshold is None
        assert legacy.ngram_n is None

    def test_the_shared_entry_point_echoes_both_knobs(self):
        """公开入口的转发要一路到**结果**：`search_dataset()` 造的是同一个 `SearchResult`

        只钉 `search()` 的话，`search_dataset()` 换成「自己拼一份字典返回」就查不出来了，
        而 CLI 与 `/api/dataset/search` 走的全是这条路。
        """
        fuzzy = search_dataset(self.ITEMS, "腿租押金", fields=self.FIELDS,
                               method="fuzzy", fuzzy_threshold=0.75)
        ngram = search_dataset(self.ITEMS, "租房", fields=self.FIELDS,
                               method="ngram", ngram_n=1)

        assert (fuzzy.fuzzy_threshold, fuzzy.ngram_n) == (0.75, None)
        assert (ngram.fuzzy_threshold, ngram.ngram_n) == (None, 1)
