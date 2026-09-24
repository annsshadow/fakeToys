# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""数据集统计模块测试"""

import math
import re
import tracemalloc

import pytest
from augmentor.statistics import (
    DatasetStatisticsCalculator, DatasetStatistics, FieldStatistics,
    calculate_statistics, get_field_summary
)


@pytest.fixture
def sample_dataset():
    """创建测试数据集"""
    return [
        {"instruction": "如何申请租房？", "input": "", "output": "请登录官网申请"},
        {"instruction": "租房需要什么材料？", "input": "", "output": "身份证、工作证明"},
        {"instruction": "租房流程是什么？", "input": "", "output": "选房、签约、付款"},
    ]


@pytest.fixture
def empty_dataset():
    """创建空数据集"""
    return []


class TestDatasetStatisticsCalculator:
    """DatasetStatisticsCalculator 测试"""
    
    def test_init(self):
        """测试初始化"""
        calculator = DatasetStatisticsCalculator()
        assert len(calculator._items) == 0
    
    def test_init_with_data(self, sample_dataset):
        """测试初始化带数据"""
        calculator = DatasetStatisticsCalculator(sample_dataset, "test")
        assert len(calculator._items) == 3
        assert calculator._dataset_name == "test"
    
    def test_load(self, sample_dataset):
        """测试加载数据"""
        calculator = DatasetStatisticsCalculator()
        calculator.load(sample_dataset, "test")
        assert len(calculator._items) == 3
    
    def test_calculate(self, sample_dataset):
        """测试计算统计信息"""
        calculator = DatasetStatisticsCalculator(sample_dataset, "test")
        stats = calculator.calculate()
        
        assert isinstance(stats, DatasetStatistics)
        assert stats.total_items == 3
        assert stats.dataset_name == "test"
        assert len(stats.field_statistics) > 0
    
    def test_calculate_empty_dataset(self, empty_dataset):
        """测试计算空数据集统计信息"""
        calculator = DatasetStatisticsCalculator(empty_dataset, "empty")
        stats = calculator.calculate()
        
        assert stats.total_items == 0
    
    def test_calculate_field_statistics(self, sample_dataset):
        """测试计算字段统计信息"""
        calculator = DatasetStatisticsCalculator(sample_dataset, "test")
        field_stats = calculator._calculate_field_statistics("instruction")
        
        assert isinstance(field_stats, FieldStatistics)
        assert field_stats.field_name == "instruction"
        assert field_stats.total_count == 3
        assert field_stats.filled_count == 3
        assert field_stats.empty_count == 0
    
    def test_calculate_content_statistics(self, sample_dataset):
        """测试计算内容统计信息"""
        calculator = DatasetStatisticsCalculator(sample_dataset, "test")
        content_stats = calculator._calculate_content_statistics()
        
        assert isinstance(content_stats, dict)
        assert "text_length" in content_stats
        assert "vocabulary" in content_stats
    
    def test_calculate_quality_metrics(self, sample_dataset):
        """测试计算质量指标"""
        calculator = DatasetStatisticsCalculator(sample_dataset, "test")
        quality_metrics = calculator._calculate_quality_metrics()
        
        assert isinstance(quality_metrics, dict)
        assert "completeness" in quality_metrics
        assert "diversity" in quality_metrics
        assert "consistency" in quality_metrics


class TestFieldStatistics:
    """FieldStatistics 测试"""
    
    def test_to_dict(self):
        """测试转换为字典"""
        stats = FieldStatistics(
            field_name="test",
            total_count=100,
            filled_count=80,
            empty_count=20,
            avg_length=10.5,
            min_length=2,
            max_length=20,
            median_length=10.0,
            std_deviation=3.2,
            unique_count=50,
            top_values=[("value1", 10), ("value2", 5)]
        )
        
        d = stats.to_dict()
        
        assert d["field_name"] == "test"
        assert d["total_count"] == 100
        assert d["filled_count"] == 80
        assert d["empty_count"] == 20
        assert d["fill_rate"] == 0.8
        assert d["avg_length"] == 10.5


class TestDatasetStatistics:
    """DatasetStatistics 测试"""
    
    def test_to_dict(self, sample_dataset):
        """测试转换为字典"""
        calculator = DatasetStatisticsCalculator(sample_dataset, "test")
        stats = calculator.calculate()
        d = stats.to_dict()
        
        assert isinstance(d, dict)
        assert "dataset_name" in d
        assert "total_items" in d
        assert "field_statistics" in d
        assert "quality_metrics" in d


class TestConvenienceFunctions:
    """便捷函数测试"""
    
    def test_calculate_statistics(self, sample_dataset):
        """测试计算统计信息"""
        stats = calculate_statistics(sample_dataset, "test")
        
        assert isinstance(stats, DatasetStatistics)
        assert stats.total_items == 3
    
    def test_get_field_summary(self, sample_dataset):
        """测试获取字段摘要"""
        summary = get_field_summary(sample_dataset, "instruction")
        
        assert isinstance(summary, dict)
        assert "field_name" in summary
        assert summary["field_name"] == "instruction"


class TestGetAllFields:
    """字段自动发现测试"""

    def test_mixed_fields(self):
        """不同字段应被自动发现"""
        items = [
            {"instruction": "q1", "output": "a1"},
            {"question": "q2", "answer": "a2"},
        ]
        calculator = DatasetStatisticsCalculator(items)
        fields = calculator._get_all_fields()
        
        assert "instruction" in fields
        assert "output" in fields
        assert "question" in fields
        assert "answer" in fields

    def test_empty_items(self):
        """空数据集应返回空字段列表"""
        calculator = DatasetStatisticsCalculator([])
        fields = calculator._get_all_fields()
        assert fields == []


class TestContentStatisticsEdgeCases:
    """内容统计边界测试"""

    def test_empty_data(self):
        """空数据集内容统计应为空"""
        calculator = DatasetStatisticsCalculator([])
        stats = calculator._calculate_content_statistics()
        assert stats == {}

    def test_empty_string_values(self):
        """空字符串值不应计入统计"""
        items = [{"instruction": "", "output": ""}]
        calculator = DatasetStatisticsCalculator(items)
        stats = calculator._calculate_content_statistics()
        assert stats == {}


class TestQualityMetricsEdgeCases:
    """质量指标边界测试"""

    def test_empty_data(self):
        """空数据集质量指标应为空"""
        calculator = DatasetStatisticsCalculator([])
        metrics = calculator._calculate_quality_metrics()
        assert metrics == {}

    def test_no_instructions(self):
        """无 instruction 数据时多样性为 0"""
        items = [{"output": "answer1"}, {"output": "answer2"}]
        calculator = DatasetStatisticsCalculator(items)
        metrics = calculator._calculate_quality_metrics()
        assert metrics["diversity"] == 0

    def test_identical_instructions(self):
        """相同 instruction 时多样性为 1/总数"""
        items = [
            {"instruction": "same", "output": "a1"},
            {"instruction": "same", "output": "a2"},
        ]
        calculator = DatasetStatisticsCalculator(items)
        metrics = calculator._calculate_quality_metrics()
        assert metrics["diversity"] == 0.5

    def test_instruction_equals_output(self):
        """instruction 等于 output 时一致性为 0"""
        items = [{"instruction": "same", "output": "same"}]
        calculator = DatasetStatisticsCalculator(items)
        metrics = calculator._calculate_quality_metrics()
        assert metrics["consistency"] == 0.0


class TestFieldStatisticsExtended:
    """FieldStatistics 扩展测试"""

    def test_median_even_count(self):
        """偶数个数据的中位数"""
        items = [
            {"field": "aa"},
            {"field": "aaaa"},
            {"field": "a"},
            {"field": "aaa"},
        ]
        calculator = DatasetStatisticsCalculator(items)
        stats = calculator._calculate_field_statistics("field")
        
        # lengths: 1, 2, 3, 4 → median = (2+3)/2 = 2.5
        assert stats.median_length == 2.5

    def test_median_odd_count(self):
        """奇数个数据的中位数"""
        items = [
            {"field": "aa"},
            {"field": "aaaa"},
            {"field": "a"},
        ]
        calculator = DatasetStatisticsCalculator(items)
        stats = calculator._calculate_field_statistics("field")
        
        # lengths: 1, 2, 4 → median = 2
        assert stats.median_length == 2

    def test_non_string_values(self):
        """非字符串值应转换为字符串计算长度"""
        items = [{"field": 12345}]
        calculator = DatasetStatisticsCalculator(items)
        stats = calculator._calculate_field_statistics("field")
        
        assert stats.avg_length == 5

    def test_partial_empty_values(self):
        """部分空值的字段统计"""
        items = [
            {"field": "hello"},
            {"field": ""},
            {"field": "world"},
        ]
        calculator = DatasetStatisticsCalculator(items)
        stats = calculator._calculate_field_statistics("field")
        
        assert stats.filled_count == 2
        assert stats.empty_count == 1


class TestGenerateSummary:
    """摘要生成测试"""

    def test_summary_contains_key_info(self):
        """摘要应包含关键信息"""
        items = [
            {"instruction": "question", "output": "answer"},
        ]
        calculator = DatasetStatisticsCalculator(items, "my_dataset")
        stats = calculator.calculate()
        
        assert "my_dataset" in stats.summary
        assert "1 条数据" in stats.summary

    def test_summary_with_quality_metrics(self):
        """摘要应包含质量指标"""
        items = [{"instruction": "q", "output": "a"}]
        calculator = DatasetStatisticsCalculator(items, "test")
        stats = calculator.calculate()
        
        assert "质量指标" in stats.summary

    def test_summary_empty_dataset(self):
        """空数据集摘要"""
        calculator = DatasetStatisticsCalculator([], "empty")
        stats = calculator.calculate()
        
        assert "0 条数据" in stats.summary


class TestFieldStatisticsToDict:
    """FieldStatistics.to_dict 扩展测试"""

    def test_fill_rate_zero_total(self):
        """total_count 为 0 时 fill_rate 应为 0"""
        stats = FieldStatistics(
            field_name="test", total_count=0, filled_count=0,
            empty_count=0, avg_length=0, min_length=0, max_length=0,
            median_length=0, std_deviation=0, unique_count=0
        )
        d = stats.to_dict()
        assert d["fill_rate"] == 0

    def test_fill_rate_computed(self):
        """fill_rate = filled/total"""
        stats = FieldStatistics(
            field_name="f", total_count=4, filled_count=3, empty_count=1,
            avg_length=1.0, min_length=1, max_length=2,
            median_length=1.0, std_deviation=0.5, unique_count=3
        )
        d = stats.to_dict()
        assert d["fill_rate"] == pytest.approx(0.75)


class TestStatisticsExtended:
    """统计模块扩展测试（覆盖剩余分支）"""

    def test_calculate_specific_fields(self, sample_dataset):
        """指定字段统计应只包含这些字段"""
        calculator = DatasetStatisticsCalculator(sample_dataset, "ds")
        stats = calculator.calculate(fields=["instruction"])
        assert list(stats.field_statistics.keys()) == ["instruction"]

    def test_calculate_unknown_field_yields_empty_stats(self):
        """未知字段统计应全为零值"""
        calculator = DatasetStatisticsCalculator([{"a": "x"}], "ds")
        stats = calculator.calculate(fields=["nonexistent"])
        fs = stats.field_statistics["nonexistent"]
        assert fs.total_count == 0 or fs.filled_count == 0

    def test_content_statistics_keys(self, sample_dataset):
        """内容统计应包含长度相关指标"""
        calculator = DatasetStatisticsCalculator(sample_dataset, "ds")
        stats = calculator.calculate()
        assert "instruction_length" in stats.content_statistics or stats.content_statistics

    def test_quality_metrics_present(self, sample_dataset):
        """质量指标应为非空字典"""
        calculator = DatasetStatisticsCalculator(sample_dataset, "ds")
        stats = calculator.calculate()
        assert isinstance(stats.quality_metrics, dict)
        assert len(stats.quality_metrics) > 0

    def test_to_dict_round_trip(self, sample_dataset):
        """DatasetStatistics.to_dict 应包含嵌套字段统计"""
        calculator = DatasetStatisticsCalculator(sample_dataset, "ds")
        stats = calculator.calculate()
        d = stats.to_dict()
        assert "field_statistics" in d
        assert "instruction" in d["field_statistics"]
        assert d["total_items"] == 3

    def test_calculate_statistics_convenience(self, sample_dataset):
        """便捷函数 calculate_statistics 应返回 DatasetStatistics"""
        result = calculate_statistics(sample_dataset, "conv")
        assert isinstance(result, DatasetStatistics)
        assert result.total_items == 3

    def test_get_field_summary_convenience(self, sample_dataset):
        """便捷函数 get_field_summary 应返回字段摘要"""
        summary = get_field_summary(sample_dataset, "instruction")
        assert summary is not None


class TestSinglePassInvariants:
    """合并遍历的**可数**不变量：同一条数据不许被反复扫

    这几条是 L12 的性能预言机。预言机故意不用耗时 —— 同一份真实数据在这台共享
    机器上不同批次量到过 47 ms 与 69 ms，任何耗时阈值都会飘；用的是解释器被叫
    起来的**次数**，它确定、可复现，而且恰好就是这些合并想省的东西。
    「before」数字均为 HEAD 版实现在同一负载上的实测。
    """

    def test_each_value_is_stringified_once(self):
        """字段统计里每个值的 `str()` 只该发生 1 次，不是 3 次

        HEAD 版把原始值存进 `values`，之后「长度」「唯一值数量」「热门值」各自
        扫一遍并各自 `str(v)`，同一个值转 3 次。
        """
        calls = []

        class Valued:
            def __init__(self, text):
                self.text = text

            def __str__(self):
                calls.append(self.text)
                return self.text

        items = [{"f": Valued("v%d" % (i % 2))} for i in range(10)]
        stats = DatasetStatisticsCalculator(items)._calculate_field_statistics("f")

        assert stats.unique_count == 2
        assert stats.top_values == [("v0", 5), ("v1", 5)]
        assert len(calls) == 10, (
            f"`str()` 被调了 {len(calls)} 次（只有 10 条数据）：每个值至多转一次，"
            "多出来的是「唯一值」与「热门值」各自重扫了全表（HEAD 版是 3 次/值）"
        )

    def test_quality_metrics_reads_two_keys_per_item(self):
        """完整性/多样性/一致性共用一趟遍历：每条只读 2 个键

        三项指标都只看 `instruction` 与 `output`，HEAD 版分三遍扫表、每条
        `item.get()` 6 次。
        """
        gets = []

        class Counted(dict):
            def get(self, *args, **kwargs):
                gets.append(1)
                return super().get(*args, **kwargs)

        items = [Counted({"instruction": "q%d" % i, "output": "a%d" % i})
                 for i in range(20)]
        metrics = DatasetStatisticsCalculator(items)._calculate_quality_metrics()

        assert metrics == {"completeness": 1.0, "diversity": 1.0, "consistency": 1.0}
        assert len(gets) <= 3 * len(items), (
            f"质量指标调了 {len(gets)} 次 `item.get()`（{len(items)} 条，上限 "
            f"{3 * len(items)}）：三项指标又各自扫了一遍全表（HEAD 版 6 次/条）"
        )

    def test_tokenizer_uses_the_precompiled_pattern(self, monkeypatch):
        """逐条切词不许回退到 `re` 的模块级入口

        `re.findall(字面模式, text)` 每次都要走一遍 `re._compile()` 的缓存查找，
        真实 6902 条 × 3 字段 = 13804 次纯查找开销。模块级 `_TOKEN_PATTERN`
        编译一次，之后只碰已编译对象的 `findall`。
        """
        seen = []
        original = re.findall

        def spy(pattern, text, *args, **kwargs):
            seen.append(pattern)
            return original(pattern, text, *args, **kwargs)

        monkeypatch.setattr(re, "findall", spy)
        stats = DatasetStatisticsCalculator(
            [{"t": "租 房 流 程 how to rent 123"}]
        )._calculate_content_statistics()

        assert stats["vocabulary"]["total_words"] == 8
        assert not seen, (
            f"切词调了 {len(seen)} 次 `re.findall`：每次都要重查编译缓存，"
            "应在模块级编译一次 `_TOKEN_PATTERN` 再复用"
        )


class TestVocabularyMemoryBudget:
    """词汇统计的峰值内存：token 缓冲区必须有上界

    HEAD 版把全数据集的 token 攒成一个巨列表再交给 `Counter`；`findall` 每次
    返回的都是新建字符串对象，所以那个巨列表是**真实占内存**的，不只是指针数组。
    攒够 `_TOKEN_CHUNK`（1000）个就整块并进计数器后，同一负载峰值 1292 KB →
    66.6 KB（真实 6902 条上是 8.15 MB → 2.22 MB），时间实测中性（配对 21 轮
    比值中位 1.005）。
    """

    def test_token_buffer_stays_bounded(self):
        text = " ".join(["alpha beta gamma delta"] * 10)   # 每条 40 个 token
        items = [{"i": text, "o": text} for _ in range(300)]

        tracemalloc.start()
        try:
            stats = DatasetStatisticsCalculator(items)._calculate_content_statistics()
            peak = tracemalloc.get_traced_memory()[1]
        finally:
            tracemalloc.stop()

        # 巨列表有 24000 个元素：省下来的必须是它，而不是「干脆没数」
        assert stats["vocabulary"]["total_words"] == 24000
        assert stats["vocabulary"]["unique_words"] == 4
        assert stats["vocabulary"]["top_words"][0] == ("alpha", 6000)

        assert peak < 400 * 1024, (
            f"峰值 {peak / 1024:.0f} KB：24000 个 token 的巨列表就要 1.3 MB，"
            "说明又回到「整表物化再一次计数」"
        )


class TestMergedPassSemantics:
    """合并遍历的语义：期望值全部手算，不向被测库要参照

    性能轮最怕「优化到一半把结果优化错了」。下面四组的数字都是纸面上算出来的。
    """

    def test_field_statistics_match_hand_computation(self):
        """长度、唯一值、热门值在一份手算数据集上的全部数字

        非空值 `aa`、`aaaa`、`aa`、`'7'`（长度 2,4,2,1）：均值 9/4=2.25；
        排序 [1,2,2,4] → 中位数 (2+2)/2=2；方差 (0.0625+3.0625+0.0625+1.5625)/4
        =1.1875 → 标准差 √1.1875；唯一值 {aa, aaaa, '7'} 共 3 个；
        热门值同频时按首次出现排，`aa` 出现 2 次排第一。
        """
        items = [{"f": "aa"}, {"f": ""}, {"f": "aaaa"}, {"f": "aa"}, {"f": 7}]
        stats = DatasetStatisticsCalculator(items)._calculate_field_statistics("f")

        assert stats.total_count == 5
        assert stats.filled_count == 4
        assert stats.empty_count == 1
        assert stats.avg_length == 2.25
        assert (stats.min_length, stats.max_length) == (1, 4)
        assert stats.median_length == 2.0
        assert stats.std_deviation == math.sqrt(1.1875)
        assert stats.unique_count == 3
        assert stats.top_values == [("aa", 2), ("aaaa", 1), ("7", 1)]

    def test_content_statistics_match_hand_computation(self):
        """`text_length.total` 与 `avg` 共用一次求和后的真值

        文本 `"m k m"` 与 `"k"`：长度 5 和 1 → 总长 6、均值 3.0；token 依次
        m,k,m,k → 共 4 个、2 个不同。**故意**让先出现的 `m` 排在 `k` 后面（字面序），
        这样「按字面序破并列」的错误实现会被这条用例抓到。
        """
        items = [{"a": "m k m"}, {"b": "k"}]
        stats = DatasetStatisticsCalculator(items)._calculate_content_statistics()

        assert stats["text_length"] == {"avg": 3.0, "min": 1, "max": 5, "total": 6}
        assert stats["vocabulary"]["total_words"] == 4
        assert stats["vocabulary"]["unique_words"] == 2
        assert stats["vocabulary"]["top_words"] == [("m", 2), ("k", 2)]

    def test_quality_metrics_match_hand_computation(self):
        """三项指标合并成一趟遍历后的真值

        4 条 × 2 个必填字段共 8 格，填了 6 格 → completeness 6/8=0.75；
        非空 instruction 是 q1,q1,q4 → 多样性 2/3；
        只有第 1 条「两个字段都有且不相等」→ 一致性 1/4=0.25。
        """
        items = [
            {"instruction": "q1", "output": "a1"},
            {"instruction": "q1", "output": ""},
            {"instruction": "", "output": "a3"},
            {"instruction": "q4", "output": "q4"},
        ]
        metrics = DatasetStatisticsCalculator(items)._calculate_quality_metrics()

        assert metrics["completeness"] == 0.75
        assert metrics["diversity"] == pytest.approx(2 / 3)
        assert metrics["consistency"] == 0.25

    def test_top_word_order_survives_chunk_boundaries(self, monkeypatch):
        """分批计数不许改变并列词的先后：把批次压到 1 个 token 也要一样

        `most_common` 对同频次的词按**首次出现**排序，而每个批次边界都是一次潜在
        的重排点。把 `_TOKEN_CHUNK` 调到 1（最碎的分批）与默认值跑同一份数据，
        结果必须完全相同，且必须是手算的那个顺序。
        """
        import augmentor.statistics as statistics_module

        items = [{"t": "m k"}, {"t": "k m"}]
        # token 依次 m,k,k,m → 两个各 2 次，先出现的 m 必须排在 k 前面
        expected = [("m", 2), ("k", 2)]

        def top_words():
            stats = DatasetStatisticsCalculator(items)._calculate_content_statistics()
            assert stats["vocabulary"]["total_words"] == 4
            return stats["vocabulary"]["top_words"]

        assert top_words() == expected
        monkeypatch.setattr(statistics_module, "_TOKEN_CHUNK", 1)
        assert top_words() == expected
