"""indexer 搜索方法与视图/统计分支测试

覆盖 exact/contains/ngram 三种搜索、get_item/get_batch、filter、
get_statistics、DatasetView 链式操作与模块级工厂。
"""

import pytest

from augmentor.indexer import (
    DatasetIndexer,
    DatasetView,
    create_indexer,
    create_view,
)


@pytest.fixture
def items():
    return [
        {"instruction": "如何申请租房？", "output": "登录官网"},
        {"instruction": "租金多少钱？", "output": "按月计价"},
        {"instruction": "押金怎么退？", "output": "满一年退还"},
        {"instruction": "如何换房？", "output": "提交申请"},
    ]


class TestSearchMethods:
    def test_exact_match(self, items):
        indexer = DatasetIndexer(items)
        matches = indexer.search_exact("instruction", "租金多少钱？")
        assert matches == [1]

    def test_exact_match_no_duplicate_for_lowercase_keys(self):
        # 回归：同一索引不得因 value 与 value.lower() 同 key 而被记录两次
        data = [{"instruction": "Hello", "output": "x"}, {"instruction": "world", "output": "y"}]
        indexer = DatasetIndexer(data)
        assert indexer.search_exact("instruction", "Hello") == [0]
        # 小写别名仍可检索
        assert indexer.search_exact("instruction", "hello") == [0]
        # 不同大小写的独立值不互相污染
        assert indexer.search_exact("instruction", "world") == [1]

    def test_contains_match(self, items):
        indexer = DatasetIndexer(items)
        matches = indexer.search_contains("instruction", "如何")
        assert 0 in matches and 3 in matches

    def test_ngram_match(self, items):
        indexer = DatasetIndexer(items)
        matches = indexer.search_ngram("instruction", "押金")
        assert 2 in matches

    def test_generic_search_contains(self, items):
        indexer = DatasetIndexer(items)
        result = indexer.search("退", method="contains")
        assert result.total_matches >= 1
        assert "contains" in result.index_used

    def test_generic_search_ngram_and_exact(self, items):
        indexer = DatasetIndexer(items)
        assert indexer.search("押金怎么退？", method="exact").total_matches >= 0
        assert indexer.search("租金", method="ngram").total_matches >= 1


class TestAccessors:
    def test_get_item(self, items):
        indexer = DatasetIndexer(items)
        assert indexer.get_item(0)["instruction"] == "如何申请租房？"
        assert indexer.get_item(99) is None

    def test_get_batch(self, items):
        indexer = DatasetIndexer(items)
        batch = indexer.get_batch([0, 2])
        assert len(batch) == 2
        assert batch[1]["instruction"] == "押金怎么退？"

    def test_filter(self, items):
        indexer = DatasetIndexer(items)
        result = indexer.filter(lambda x: "如何" in x.get("instruction", ""))
        assert len(result) == 2


class TestStatistics:
    def test_statistics_shape(self, items):
        indexer = DatasetIndexer(items)
        stats = indexer.get_statistics()
        assert stats["total_items"] == 4
        assert "field_statistics" in stats
        assert "instruction" in stats["field_statistics"]

    def test_list_indexes(self, items):
        indexer = DatasetIndexer(items)
        assert isinstance(indexer.list_indexes(), list)


class TestDatasetView:
    def test_view_len_and_item(self, items):
        view = DatasetView(items)
        assert len(view) == 4
        assert view[0]["instruction"] == "如何申请租房？"
        assert [i["output"] for i in view][:1] == ["登录官网"]

    def test_view_head_tail(self, items):
        view = create_view(items)
        assert len(view.head(2)) == 2
        assert len(view.tail(2)) == 2
        assert view.head(2).to_list()[0] == items[0]

    def test_view_sample_deterministic(self, items):
        view = create_view(items)
        s1 = view.sample(2, seed=7).to_list()
        s2 = view.sample(2, seed=7).to_list()
        assert s1 == s2
        assert len(s1) == 2

    def test_view_filter_and_search(self, items):
        view = create_view(items)
        filtered = view.filter(lambda x: "如何" in x.get("instruction", ""))
        assert len(filtered) == 2
        result = view.search("退")
        assert result.total_matches >= 1

    def test_view_out_of_range(self, items):
        view = DatasetView(items)
        with pytest.raises(IndexError):
            _ = view[99]


class TestFactories:
    def test_create_indexer(self, items):
        indexer = create_indexer(items)
        assert isinstance(indexer, DatasetIndexer)

    def test_create_view(self, items):
        view = create_view(items, name="v")
        assert isinstance(view, DatasetView)
