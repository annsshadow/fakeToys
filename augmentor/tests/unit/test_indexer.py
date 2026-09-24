# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""数据集索引模块测试"""

import threading
import tracemalloc

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


def _items(count):
    """造确定性数据集：字段值刻意重复，让 exact/ngram 查询都能真命中"""
    return [
        {
            "instruction": f"如何办理第 {i % 13} 项事宜",
            "input": f"编号 {i % 7}",
            "output": f"请到窗口 {i % 5} 办理第 {i % 13} 项",
        }
        for i in range(count)
    ]


@pytest.fixture
def build_log(monkeypatch):
    """记录「建了哪几份索引」的确定性预言机

    为什么数次数而不是量时间：墙钟预算用例在全量套件里受覆盖率 trace 与
    同机并行进程影响，红得不稳定也不红得可信（见 OPTIMIZATION_LOOP 的纪律段）。
    两个被替换的方法在改动前后都存在，且转发用 `*args, **kwargs`，所以缺陷态
    会红在「真的白建了 4 份索引」上，而不是红在 AttributeError 上。
    """
    log = []
    original_field = DatasetIndexer._build_field_index
    original_ngram = DatasetIndexer._build_ngram_index

    def spy_field(self, *args, **kwargs):
        log.append(f"field:{args[0] if args else kwargs['field']}")
        return original_field(self, *args, **kwargs)

    def spy_ngram(self, *args, **kwargs):
        field = args[0] if args else kwargs["field"]
        n = args[1] if len(args) > 1 else kwargs.get("n", 2)
        log.append(f"ngram:{field}_{n}")
        return original_ngram(self, *args, **kwargs)

    monkeypatch.setattr(DatasetIndexer, "_build_field_index", spy_field)
    monkeypatch.setattr(DatasetIndexer, "_build_ngram_index", spy_ngram)
    return log


def assert_probe_alive(build_log):
    """先做一次「两版实现都必然建索引」的操作，证明计数器在响

    纯「0 次」断言最怕的是探针根本没生效：白建与否都报 0。这里用
    `list_indexes()` 触发一次完整建索引，再清空，之后才允许断言 0。
    """
    DatasetIndexer(_items(20)).list_indexes()
    assert len(build_log) == 4, (
        f"计数探针未生效：list_indexes() 应建满 4 份默认索引，实际 {build_log}")
    build_log.clear()


class TestLazyIndexConstruction:
    """索引按需构建（A11：视图操作白建索引）"""

    def test_view_operations_build_no_indexes(self, build_log):
        """切片/过滤/采样一类视图操作一个都不读索引，就不该为之建索引"""
        assert_probe_alive(build_log)
        items = _items(300)

        view = DatasetView(items)
        assert len(view) == 300
        assert view.to_list() == items
        view.head(10)
        view.tail(10)
        view.sample(20, seed=1)
        view[10:20]
        view.filter(lambda it: it["input"] != "").filter(
            lambda it: len(it["output"]) > 3).head(5)
        DatasetIndexer(items).filter(lambda it: True)
        DatasetIndexer(items).get_item(3)
        DatasetIndexer(items).get_batch([0, 1])

        assert build_log == [], f"视图操作白建了索引：{build_log}"

    def test_constructing_a_view_does_not_allocate_the_indexes(self, build_log):
        """构造视图的驻留内存必须是常数级：4000 条的 4 份索引实测 11.7 MB

        预算取 64 KB：修复后实测约 1 KB，缺陷态 11.7 MB（红），中间有足够余量，
        不会被这点数据集的正常开销顶穿。
        """
        assert_probe_alive(build_log)
        items = _items(4000)

        tracemalloc.start()
        view = DatasetView(items)
        assert len(view) == 4000
        _peak, current = tracemalloc.get_traced_memory()
        tracemalloc.stop()

        assert current < 64 * 1024, (
            f"构造视图 + 取长度后仍驻留 {current / 1024:.1f} KB，索引没改成按需构建")

    def test_contains_query_never_builds_an_index(self, build_log):
        """默认搜索方法是 contains，它扫全表、不读任何索引"""
        assert_probe_alive(build_log)
        items = _items(300)

        result = DatasetView(items).search("窗口 3")

        assert build_log == [], f"contains 查询建了没人用的索引：{build_log}"
        # 「没建索引」不能靠「干脆没查」蒙过去：结果要和对全表的朴素扫描一致
        expected = {i for i, it in enumerate(items)
                    if "窗口 3" in it["instruction"].lower()
                    or "窗口 3" in it["output"].lower()}
        assert result.total_matches == len(expected) > 0

    def test_exact_query_builds_only_the_fields_it_reads(self, build_log):
        """exact 只读它要查的那两个字段，第三份倒排与 n-gram 索引不该建"""
        assert_probe_alive(build_log)
        items = _items(300)

        result = DatasetIndexer(items).search("如何办理第 3 项事宜", method="exact")

        assert build_log == ["field:instruction", "field:output"], build_log
        assert result.total_matches > 0

    def test_ngram_query_builds_only_its_own_ngram_index(self, build_log):
        """ngram 查询只用 n-gram 索引，三份倒排索引不该陪跑"""
        assert_probe_alive(build_log)
        items = _items(300)

        result = DatasetIndexer(items).search("办理", method="ngram")

        assert build_log == ["ngram:instruction_2"], build_log
        assert result.total_matches > 0

    def test_each_index_is_built_once_and_reused(self, build_log):
        """同一索引器上重复查询不得重复建：按需 ≠ 每次查都建"""
        assert_probe_alive(build_log)
        indexer = DatasetIndexer(_items(300))

        for _ in range(3):
            indexer.search("如何办理第 3 项事宜", method="exact")
        for _ in range(2):
            indexer.search("办理", method="ngram")

        assert sorted(build_log) == sorted(
            ["field:instruction", "field:output", "ngram:instruction_2"]), build_log

    def test_statistics_and_list_indexes_still_report_all_defaults(self, build_log):
        """对外可见的「建了哪些索引」口径不能因为按需构建而少报"""
        assert_probe_alive(build_log)
        indexer = DatasetIndexer(_items(120))

        names = [i.name for i in indexer.list_indexes()]
        assert names == ["field_instruction", "field_output", "field_input",
                         "ngram_instruction_2"], names
        assert len(build_log) == 4, build_log

        build_log.clear()
        stats = indexer.get_statistics()
        assert stats["indexes"] == ["instruction", "output", "input"]
        assert stats["ngram_indexes"] == ["instruction_2"]
        # 已经建齐了，第二个报告者不该再触发一次重建
        assert build_log == [], f"重复建索引：{build_log}"

    def test_empty_dataset_keeps_its_old_report(self):
        """空数据集：构造出来的索引器仍然报「零份索引」

        旧实现靠 `if self._items` 跳过建索引，`load([])` 却会建出 4 份空索引。
        按需构建要原样保住这两种口径的差别，所以「欠哪几份」与「数据是否为空」
        是分开记录的（见 `__init__` / `load`）。
        """
        assert DatasetIndexer([]).list_indexes() == []
        assert DatasetIndexer([]).get_statistics()["indexes"] == []

        loaded = DatasetIndexer()
        loaded.load([])
        assert [i.name for i in loaded.list_indexes()] == [
            "field_instruction", "field_output", "field_input", "ngram_instruction_2"]

    def test_load_invalidates_indexes_built_before_it(self, build_log):
        """换数据必须作废已建索引——旧实现靠重建，新实现靠重新欠账"""
        indexer = DatasetIndexer([{"instruction": "只有旧数据有这个词", "output": "x"}])
        assert indexer.search_exact("instruction", "只有旧数据有这个词") == [0]
        build_log.clear()

        indexer.load([{"instruction": "只有新数据有这个词", "output": "y"}])
        assert indexer.search_exact("instruction", "只有新数据有这个词") == [0]
        assert indexer.search_exact("instruction", "只有旧数据有这个词") == []
        # 查询只读 instruction，所以 load 之后也只该重建这一份
        assert build_log == ["field:instruction"], build_log

    def test_slice_view_answers_from_its_own_items(self, build_log):
        """子视图的索引建在自己的切片上，不能拿父视图的索引回答"""
        items = _items(50)
        view = DatasetView(items)
        window = view[10:20]
        build_log.clear()

        hits = window.search(items[15]["instruction"], method="exact").items

        assert hits, "切片视图的 exact 查询查不到本切片内的数据"
        assert all(h["instruction"] == items[15]["instruction"] for h in hits)
        # 父视图在此之前一份索引都没建；这次查询只为切片建了它读的那两个字段
        assert build_log == ["field:instruction", "field:output"], build_log
        # 第 5 条也在父视图里满足同一条查询，但它不在切片内
        assert items[5]["instruction"] != items[15]["instruction"]

    def test_non_default_field_query_stays_empty_without_building(self, build_log):
        """默认清单之外的字段：既不多建一份索引，也不改变「查无此项」的语义

        「exact 查自定义字段永远返回空」本身是个坑（已另立 A23 记录），但那是
        改语义的另一件事，性能轮不顺手改。
        """
        assert_probe_alive(build_log)
        indexer = DatasetIndexer([{"instruction": "a", "answer": "b"}])

        assert indexer.search_exact("answer", "b") == []
        assert indexer.search_ngram("answer", "b") == []
        assert build_log == [], f"为清单外的字段建了索引：{build_log}"

    def test_concurrent_first_queries_all_get_the_same_answer(self, build_log):
        """多线程同时发来第一条查询：只能有一个建索引，其余复用

        这条是语义护栏（缺陷态也绿）：它钉住的是「销账必须发生在建完之后」，
        防止后人把 `_pending_fields.discard()` 挪到构建前面——那样第二个线程会
        以为索引已就绪、读到没有这个键的字典而把结果误报成「无匹配」。
        """
        items = _items(400)
        indexer = DatasetIndexer(items)
        value = items[0]["instruction"]
        results = []
        barrier = threading.Barrier(8)

        def worker():
            barrier.wait()
            results.append(indexer.search_exact("instruction", value))

        threads = [threading.Thread(target=worker) for _ in range(8)]
        for t in threads:
            t.start()
        for t in threads:
            t.join()

        expected = [i for i, it in enumerate(items) if it["instruction"] == value]
        assert expected
        assert len(results) == 8
        assert all(r == expected for r in results), "并发首查询读到了半成品索引"
        # 8 个线程抢第一条查询，也只能建 1 份索引
        assert build_log == ["field:instruction"], build_log

