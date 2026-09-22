# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""数据分析模块测试"""

import pytest
from augmentor.analytics import (
    DatasetAnalyzer, AnalysisReport, DataInsight, TextStatistics,
    analyze_dataset, get_dataset_insights
)


@pytest.fixture
def sample_dataset():
    """创建测试数据集"""
    return [
        {"instruction": "如何申请租房？", "input": "", "output": "请登录官网申请租房服务"},
        {"instruction": "租房需要什么材料？", "input": "", "output": "需要身份证、工作证明、收入证明等材料"},
        {"instruction": "租房流程是什么？", "input": "", "output": "选房、签约、付款、入住"},
        {"instruction": "如何申请买房？", "input": "", "output": "请咨询销售顾问"},
        {"instruction": "买房需要什么材料？", "input": "", "output": "需要身份证、户口本、收入证明、征信报告等"},
    ]


@pytest.fixture
def empty_dataset():
    """创建空数据集"""
    return []


@pytest.fixture
def minimal_dataset():
    """创建最小数据集"""
    return [
        {"instruction": "问题", "input": "", "output": "回答"},
    ]


class TestDatasetAnalyzer:
    """DatasetAnalyzer 测试"""
    
    def test_init(self):
        """测试初始化"""
        analyzer = DatasetAnalyzer()
        assert len(analyzer._items) == 0
    
    def test_init_with_data(self, sample_dataset):
        """测试初始化带数据"""
        analyzer = DatasetAnalyzer(sample_dataset)
        assert len(analyzer._items) == 5
    
    def test_load(self, sample_dataset):
        """测试加载数据"""
        analyzer = DatasetAnalyzer()
        analyzer.load(sample_dataset)
        assert len(analyzer._items) == 5
    
    def test_analyze(self, sample_dataset):
        """测试分析数据集"""
        analyzer = DatasetAnalyzer(sample_dataset)
        report = analyzer.analyze()
        
        assert isinstance(report, AnalysisReport)
        assert report.dataset_size == 5
        assert "instruction" in report.field_statistics
        assert report.quality_score >= 0
        assert report.diversity_score >= 0
        assert report.completeness_score >= 0
    
    def test_analyze_empty_dataset(self, empty_dataset):
        """测试分析空数据集"""
        analyzer = DatasetAnalyzer(empty_dataset)
        report = analyzer.analyze()
        
        assert report.dataset_size == 0
        assert report.quality_score == 0
    
    def test_calculate_text_statistics(self, sample_dataset):
        """测试计算文本统计信息"""
        analyzer = DatasetAnalyzer(sample_dataset)
        texts = [item["instruction"] for item in sample_dataset]
        stats = analyzer._calculate_text_statistics(texts)
        
        assert isinstance(stats, TextStatistics)
        assert stats.total_chars > 0
        assert stats.avg_chars > 0
        assert stats.vocabulary_size > 0
    
    def test_generate_insights(self, sample_dataset):
        """测试生成洞察"""
        analyzer = DatasetAnalyzer(sample_dataset)
        report = analyzer.analyze()
        
        assert isinstance(report.insights, list)
        assert len(report.insights) >= 0
    
    def test_get_duplicate_candidates(self, sample_dataset):
        """测试获取重复候选"""
        analyzer = DatasetAnalyzer(sample_dataset)
        candidates = analyzer.get_duplicate_candidates(threshold=0.5)
        
        assert isinstance(candidates, list)
    
    def test_get_quality_distribution(self, sample_dataset):
        """测试获取质量分布"""
        analyzer = DatasetAnalyzer(sample_dataset)
        distribution = analyzer.get_quality_distribution()
        
        assert isinstance(distribution, dict)
        assert sum(distribution.values()) == len(sample_dataset)
    
    def test_get_category_distribution(self, sample_dataset):
        """测试获取类别分布"""
        analyzer = DatasetAnalyzer(sample_dataset)
        distribution = analyzer.get_category_distribution()
        
        assert isinstance(distribution, dict)
        assert sum(distribution.values()) == len(sample_dataset)


class TestConvenienceFunctions:
    """便捷函数测试"""
    
    def test_analyze_dataset(self, sample_dataset):
        """测试分析数据集"""
        report = analyze_dataset(sample_dataset)
        
        assert isinstance(report, AnalysisReport)
        assert report.dataset_size == 5
    
    def test_get_dataset_insights(self, sample_dataset):
        """测试获取数据集洞察"""
        insights = get_dataset_insights(sample_dataset)
        
        assert isinstance(insights, list)
    
    def test_analyze_empty_dataset(self, empty_dataset):
        """测试分析空数据集"""
        report = analyze_dataset(empty_dataset)
        
        assert report.dataset_size == 0


class TestAnalysisReport:
    """AnalysisReport 测试"""
    
    def test_to_dict(self, sample_dataset):
        """测试转换为字典"""
        report = analyze_dataset(sample_dataset)
        d = report.to_dict()
        
        assert isinstance(d, dict)
        assert "dataset_size" in d
        assert "field_statistics" in d
        assert "insights" in d
        assert "scores" in d


class TestTokenize:
    """分词测试"""

    def test_chinese_text(self):
        """中文文本分词"""
        analyzer = DatasetAnalyzer()
        tokens = analyzer._tokenize("如何申请租房")
        assert len(tokens) >= 1
        assert all(isinstance(t, str) for t in tokens)

    def test_english_text(self):
        """英文文本分词"""
        analyzer = DatasetAnalyzer()
        tokens = analyzer._tokenize("hello world")
        assert "hello" in tokens
        assert "world" in tokens

    def test_mixed_text(self):
        """中英混合分词"""
        analyzer = DatasetAnalyzer()
        tokens = analyzer._tokenize("申请apply租房")
        assert len(tokens) >= 2

    def test_numbers(self):
        """数字分词"""
        analyzer = DatasetAnalyzer()
        tokens = analyzer._tokenize("租金1200元")
        assert "1200" in tokens

    def test_empty_text(self):
        """空文本分词"""
        analyzer = DatasetAnalyzer()
        tokens = analyzer._tokenize("")
        assert tokens == []


class TestGetWords:
    """词汇提取测试"""

    def test_filters_single_char_numbers(self):
        """单字符数字应被过滤"""
        analyzer = DatasetAnalyzer()
        words = analyzer._get_words("a 1 b")
        assert "1" not in words

    def test_keeps_alpha_words(self):
        """字母词汇应保留"""
        analyzer = DatasetAnalyzer()
        words = analyzer._get_words("hello world")
        assert "hello" in words
        assert "world" in words

    def test_empty_text(self):
        """空文本应返回空列表"""
        analyzer = DatasetAnalyzer()
        words = analyzer._get_words("")
        assert words == []


class TestGetSentences:
    """句子分割测试"""

    def test_chinese_sentences(self):
        """中文句子分割"""
        analyzer = DatasetAnalyzer()
        sentences = analyzer._get_sentences("第一句。第二句！第三句？")
        assert len(sentences) == 3

    def test_english_sentences(self):
        """英文句子分割"""
        analyzer = DatasetAnalyzer()
        sentences = analyzer._get_sentences("First. Second! Third?")
        assert len(sentences) == 3

    def test_empty_text(self):
        """空文本应返回空列表"""
        analyzer = DatasetAnalyzer()
        sentences = analyzer._get_sentences("")
        assert sentences == []

    def test_no_punctuation(self):
        """无标点文本应作为单个句子返回"""
        analyzer = DatasetAnalyzer()
        sentences = analyzer._get_sentences("没有标点的文本")
        assert len(sentences) == 1


class TestCalculateQualityScore:
    """质量分数计算测试"""

    def test_empty_dataset(self):
        """空数据集质量分数应为 0"""
        analyzer = DatasetAnalyzer([])
        assert analyzer._calculate_quality_score() == 0.0

    def test_perfect_quality(self):
        """完全匹配的数据质量分数应较高"""
        items = [
            {"instruction": "问题A", "output": "这是关于完全不同内容的回答"},
            {"instruction": "问题B", "output": "这是另一个完全不同的回答"},
        ]
        analyzer = DatasetAnalyzer(items)
        score = analyzer._calculate_quality_score()
        assert 0.0 <= score <= 1.0

    def test_incomplete_items_lower_score(self):
        """缺少字段的数据应降低质量分数"""
        items = [
            {"instruction": "问题", "output": "回答"},
            {"instruction": "", "output": ""},  # 缺少字段
        ]
        analyzer = DatasetAnalyzer(items)
        score = analyzer._calculate_quality_score()
        assert score < 1.0

    def test_missing_output(self):
        """缺少 output 的数据"""
        items = [
            {"instruction": "问题", "output": ""},
            {"instruction": "问题2", "output": ""},
        ]
        analyzer = DatasetAnalyzer(items)
        score = analyzer._calculate_quality_score()
        assert score < 1.0


class TestCalculateDiversityScore:
    """多样性分数计算测试"""

    def test_empty_dataset(self):
        """空数据集多样性分数应为 0"""
        analyzer = DatasetAnalyzer([])
        assert analyzer._calculate_diversity_score() == 0.0

    def test_no_instructions(self):
        """无 instruction 数据多样性分数应为 0"""
        items = [{"output": "回答"}]
        analyzer = DatasetAnalyzer(items)
        assert analyzer._calculate_diversity_score() == 0.0

    def test_identical_instructions(self):
        """相同 instruction 多样性应较低"""
        items = [
            {"instruction": "相同问题"},
            {"instruction": "相同问题"},
            {"instruction": "相同问题"},
        ]
        analyzer = DatasetAnalyzer(items)
        score = analyzer._calculate_diversity_score()
        assert 0.0 <= score <= 1.0

    def test_diverse_instructions(self):
        """不同 instruction 多样性应较高"""
        items = [
            {"instruction": "如何申请租房？"},
            {"instruction": "买房需要什么条件？"},
            {"instruction": "物业费怎么计算？"},
        ]
        analyzer = DatasetAnalyzer(items)
        score = analyzer._calculate_diversity_score()
        assert 0.0 <= score <= 1.0

    def test_no_words_in_instructions(self):
        """instruction 中无有效词汇时应返回 0"""
        items = [{"instruction": "1"}, {"instruction": "2"}]
        analyzer = DatasetAnalyzer(items)
        score = analyzer._calculate_diversity_score()
        assert score == 0.0


class TestCalculateCompletenessScore:
    """完整性分数计算测试"""

    def test_empty_dataset(self):
        """空数据集完整性分数应为 0"""
        analyzer = DatasetAnalyzer([])
        assert analyzer._calculate_completeness_score(["instruction", "output"]) == 0.0

    def test_all_complete(self):
        """所有字段都完整的数据完整性应为 1"""
        items = [
            {"instruction": "问题", "output": "回答"},
            {"instruction": "问题2", "output": "回答2"},
        ]
        analyzer = DatasetAnalyzer(items)
        score = analyzer._calculate_completeness_score(["instruction", "output"])
        assert score == pytest.approx(1.0)

    def test_partial_complete(self):
        """部分字段完整时完整性应在 0-1 之间"""
        items = [
            {"instruction": "问题", "output": "回答"},
            {"instruction": "问题2", "output": ""},
        ]
        analyzer = DatasetAnalyzer(items)
        score = analyzer._calculate_completeness_score(["instruction", "output"])
        assert 0.0 < score < 1.0

    def test_no_required_fields(self):
        """不需要的字段不影响完整性"""
        items = [{"other_field": "值"}]
        analyzer = DatasetAnalyzer(items)
        score = analyzer._calculate_completeness_score(["instruction", "output"])
        assert score == 0.0


class TestGetDuplicateCandidates:
    """重复候选测试"""

    def test_no_duplicates(self):
        """无重复时应返回空列表"""
        items = [
            {"instruction": "完全不同的问题1"},
            {"instruction": "完全不同的问题2"},
        ]
        analyzer = DatasetAnalyzer(items)
        candidates = analyzer.get_duplicate_candidates(threshold=0.9)
        assert candidates == []

    def test_high_threshold_fewer_matches(self):
        """高阈值应返回较少匹配"""
        items = [
            {"instruction": "如何申请租房"},
            {"instruction": "如何申请租房呢"},
        ]
        analyzer = DatasetAnalyzer(items)
        high = analyzer.get_duplicate_candidates(threshold=0.9)
        low = analyzer.get_duplicate_candidates(threshold=0.3)
        assert len(high) <= len(low)

    def test_empty_dataset(self):
        """空数据集应返回空列表"""
        analyzer = DatasetAnalyzer([])
        assert analyzer.get_duplicate_candidates() == []


class TestGetQualityDistribution:
    """质量分布测试"""

    def test_all_excellent(self):
        """高质量数据应标记为 excellent"""
        items = [
            {"instruction": "这是一个详细的问题描述信息", "output": "这是一个非常详细和完整的回答内容，包含了大量有用的信息和详细的步骤说明，可以帮助用户更好地理解和解决问题"},
        ]
        analyzer = DatasetAnalyzer(items)
        dist = analyzer.get_quality_distribution()
        assert dist["excellent"] == 1

    def test_all_poor(self):
        """低质量数据应标记为 poor"""
        items = [
            {"instruction": "问", "output": "答"},
        ]
        analyzer = DatasetAnalyzer(items)
        dist = analyzer.get_quality_distribution()
        assert dist["poor"] == 1

    def test_empty_dataset(self):
        """空数据集分布应全为 0"""
        analyzer = DatasetAnalyzer([])
        dist = analyzer.get_quality_distribution()
        assert all(v == 0 for v in dist.values())


class TestGetCategoryDistribution:
    """类别分布测试"""

    def test_rental_category(self):
        """包含"租房"关键词应分类到租房"""
        items = [{"instruction": "如何申请租房？"}]
        analyzer = DatasetAnalyzer(items)
        dist = analyzer.get_category_distribution()
        assert dist.get("租房", 0) == 1

    def test_buy_category(self):
        """包含"买房"关键词应分类到买房"""
        items = [{"instruction": "买房需要什么条件？"}]
        analyzer = DatasetAnalyzer(items)
        dist = analyzer.get_category_distribution()
        assert dist.get("买房", 0) == 1

    def test_loan_category(self):
        """包含"贷款"关键词应分类到贷款"""
        items = [{"instruction": "贷款利率是多少？"}]
        analyzer = DatasetAnalyzer(items)
        dist = analyzer.get_category_distribution()
        assert dist.get("贷款", 0) == 1

    def test_other_category(self):
        """不匹配任何关键词应分类到其他"""
        items = [{"instruction": "今天天气真好"}]
        analyzer = DatasetAnalyzer(items)
        dist = analyzer.get_category_distribution()
        assert dist.get("其他", 0) == 1

    def test_multiple_items(self):
        """多个项目的类别分布"""
        items = [
            {"instruction": "租房流程"},
            {"instruction": "买房条件"},
            {"instruction": "贷款利率"},
        ]
        analyzer = DatasetAnalyzer(items)
        dist = analyzer.get_category_distribution()
        assert sum(dist.values()) == 3


class TestTextStatisticsEdgeCases:
    """文本统计边界测试"""

    def test_empty_texts(self):
        """空文本列表应返回零值统计"""
        analyzer = DatasetAnalyzer()
        stats = analyzer._calculate_text_statistics([])
        assert stats.total_chars == 0
        assert stats.avg_chars == 0
        assert stats.vocabulary_size == 0

    def test_single_text(self):
        """单条文本统计"""
        analyzer = DatasetAnalyzer()
        stats = analyzer._calculate_text_statistics(["hello"])
        assert stats.total_chars == 5
        assert stats.min_chars == 5
        assert stats.max_chars == 5


class TestInsightsGeneration:
    """洞察生成测试"""

    def test_small_dataset_warning(self):
        """小数据集应产生警告"""
        items = [{"instruction": "q", "output": "a"}] * 5
        analyzer = DatasetAnalyzer(items)
        report = analyzer.analyze()
        warnings = [i for i in report.insights if i.severity == "warning"]
        assert len(warnings) >= 1

    def test_large_dataset_info(self):
        """大数据集应产生信息"""
        items = [{"instruction": f"question {i}", "output": f"answer {i}"} for i in range(1000)]
        analyzer = DatasetAnalyzer(items)
        report = analyzer.analyze()
        infos = [i for i in report.insights if i.severity == "info"]
        assert any("数据量充足" in i.title for i in infos)

    def test_short_instructions_warning(self):
        """短问题应产生警告"""
        items = [{"instruction": "问", "output": "这是一个足够长的回答内容"}] * 5
        analyzer = DatasetAnalyzer(items)
        report = analyzer.analyze()
        warnings = [i for i in report.insights if "过短" in i.title]
        assert len(warnings) >= 1

    def test_short_outputs_warning(self):
        """短回答应产生警告"""
        items = [{"instruction": "这是一个足够长的问题", "output": "短"}] * 5
        analyzer = DatasetAnalyzer(items)
        report = analyzer.analyze()
        warnings = [i for i in report.insights if "回答过短" in i.title]
        assert len(warnings) >= 1


class TestAnalyticsExtended:
    """DatasetAnalyzer 扩展测试（覆盖剩余分支）"""

    def test_load_resets_items(self):
        """load 应替换内部数据"""
        analyzer = DatasetAnalyzer([{"instruction": "old"}])
        analyzer.load([{"instruction": "new"}])
        assert analyzer._items[0]["instruction"] == "new"

    def test_tokenize_mixed(self):
        """分词应处理中英数字"""
        analyzer = DatasetAnalyzer([])
        tokens = analyzer._tokenize("你好abc123")
        assert "你好" in tokens
        assert "abc" in tokens
        assert "123" in tokens

    def test_get_words_filters_single_char(self):
        """_get_words 应过滤单字符非字母词"""
        analyzer = DatasetAnalyzer([])
        words = analyzer._get_words("a b 中 好")
        assert all(len(w) > 1 or w.isalpha() for w in words)

    def test_get_sentences_splits_punctuation(self):
        """句子应按中英文标点切分"""
        analyzer = DatasetAnalyzer([])
        sentences = analyzer._get_sentences("第一句。第二句！第三句？")
        assert len(sentences) == 3

    def test_text_statistics_empty(self):
        """空文本统计应全为零值"""
        analyzer = DatasetAnalyzer([])
        stats = analyzer._calculate_text_statistics([])
        assert stats.total_chars == 0
        assert stats.vocabulary_size == 0
        assert stats.top_words == []

    def test_text_statistics_with_data(self):
        """有数据时统计应正确计算字数与词频"""
        analyzer = DatasetAnalyzer([])
        stats = analyzer._calculate_text_statistics(["如何申请", "如何退租"])
        assert stats.total_chars == 8
        assert stats.avg_chars == pytest.approx(4.0)
        assert stats.vocabulary_size > 0

    def test_quality_score_range(self):
        """质量分应在 0-1 之间"""
        items = [{"instruction": f"问题{i}", "output": f"回答{i}"} for i in range(10)]
        score = DatasetAnalyzer(items)._calculate_quality_score()
        assert 0.0 <= score <= 1.0

    def test_diversity_score_range(self):
        """多样性分应在 0-1 之间"""
        items = [{"instruction": f"不同的问题{i}啊"} for i in range(10)]
        score = DatasetAnalyzer(items)._calculate_diversity_score()
        assert 0.0 <= score <= 1.0

    def test_completeness_score_fields(self):
        """完整性分应基于指定字段计算"""
        items = [{"instruction": "q", "output": "a", "input": ""}] * 5
        score = DatasetAnalyzer(items)._calculate_completeness_score(["instruction", "output", "input"])
        assert 0.0 <= score <= 1.0

    def test_analyze_custom_fields(self):
        """analyze 应支持自定义字段列表"""
        items = [{"custom": "内容"}] * 3
        report = DatasetAnalyzer(items).analyze(fields=["custom"])
        assert "custom" in report.field_statistics

    def test_report_to_dict(self):
        """AnalysisReport.to_dict 应包含全部字段"""
        items = [{"instruction": "q", "output": "a"}]
        report = DatasetAnalyzer(items).analyze()
        d = report.to_dict()
        assert "dataset_size" in d
        assert "quality_score" in d or "scores" in d
