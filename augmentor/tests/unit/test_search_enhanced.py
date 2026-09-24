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
