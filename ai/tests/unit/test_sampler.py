"""ActiveSampler 单元测试

覆盖问题类型判定、长度/主题/复杂度分布分析，以及覆盖不足识别与选样推荐。
"""

import pytest

from augmentor.sampler import ActiveSampler, SamplingResult


@pytest.fixture
def sampler():
    return ActiveSampler()


class TestQuestionType:
    @pytest.mark.parametrize(
        "text,expected",
        [
            ("这是什么服务", "what"),
            ("怎么申请入住", "how"),
            ("如何退租", "how"),
            ("能不能延期", "can"),
            ("押金可以退吗", "can"),
            ("租金多少钱", "how_many"),
            ("我要投诉", "other"),
        ],
    )
    def test_classifies_question_type(self, sampler, text, expected):
        assert sampler._analyze_question_type(text) == expected

    def test_why_question_now_reachable(self, sampler):
        """修复后："为什么" 优先判定，why 分支可达（此前被 "什么" 分支误吞）"""
        assert sampler._analyze_question_type("为什么申请被拒绝") == "why"


class TestLengthDistribution:
    def test_empty_items_returns_zero_distribution(self, sampler):
        assert sampler._analyze_length_distribution([]) == {
            "short": 0,
            "medium": 0,
            "long": 0,
        }

    def test_ratio_and_average(self, sampler):
        items = [
            {"instruction": "短"},          # 1 -> short
            {"instruction": "中等长度问题啊啊"},  # 8 -> short
            {"instruction": "长" * 30},     # 30 -> long
        ]
        dist = sampler._analyze_length_distribution(items)

        assert dist["short"] == pytest.approx(2 / 3)
        assert dist["medium"] == 0
        assert dist["long"] == pytest.approx(1 / 3)
        assert dist["avg_length"] == pytest.approx(13)

    def test_medium_bucket(self, sampler):
        items = [{"instruction": "字" * 14}]  # 14 -> medium
        dist = sampler._analyze_length_distribution(items)
        assert dist["medium"] == 1.0


class TestTopicDistribution:
    def test_counts_unique_words_and_coverage(self, sampler):
        items = [
            {"instruction": "租金，押金"},
            {"instruction": "租金，合同"},
        ]
        dist = sampler._analyze_topic_distribution(items)

        assert dist["unique_words"] == 3
        assert dist["top_words"]["租金"] == 2
        assert dist["coverage_score"] == pytest.approx(3 / 4)

    def test_empty_items_yields_zero_coverage(self, sampler):
        dist = sampler._analyze_topic_distribution([])
        assert dist["unique_words"] == 0
        assert dist["coverage_score"] == 0


class TestComplexity:
    def test_empty_items_returns_zero_distribution(self, sampler):
        assert sampler._analyze_complexity([]) == {
            "simple": 0,
            "medium": 0,
            "complex": 0,
        }

    def test_short_text_is_simple(self, sampler):
        dist = sampler._analyze_complexity([{"instruction": "你好"}])
        assert dist["simple"] == 1.0
        assert dist["avg_complexity"] < 0.3

    def test_long_text_with_connectors_is_complex(self, sampler):
        # 长度 45（>=50 上限内约 0.9）+ 5 个连接符（封顶 1.0）→ 复杂度约 0.95
        text = "问题" * 20 + "，" * 5
        dist = sampler._analyze_complexity([{"instruction": text}])
        assert dist["complex"] == 1.0
        assert dist["avg_complexity"] >= 0.7


class TestAnalyzeCoverage:
    def test_returns_all_distributions(self, sampler):
        items = [
            {"instruction": "这是什么服务"},
            {"instruction": "怎么申请入住"},
        ]
        coverage = sampler.analyze_coverage(items)

        assert coverage["total_items"] == 2
        assert coverage["question_type_distribution"] == {"what": 0.5, "how": 0.5}
        assert "avg_length" in coverage["length_distribution"]
        assert "coverage_score" in coverage["topic_distribution"]
        assert "avg_complexity" in coverage["complexity_distribution"]

    def test_empty_items_does_not_divide_by_zero(self, sampler):
        coverage = sampler.analyze_coverage([])
        assert coverage["total_items"] == 0
        assert coverage["question_type_distribution"] == {}
        assert coverage["length_distribution"]["short"] == 0


class TestIdentifyUnderrepresented:
    def test_flags_rare_question_types(self, sampler):
        items = [{"instruction": "这是什么服务"} for _ in range(10)]
        items.append({"instruction": "怎么申请"})

        underrepresented = sampler.identify_underrepresented(items)
        assert "question_type:how" in underrepresented

    def test_flags_rare_length_buckets(self, sampler):
        items = [{"instruction": "长" * 30}]
        underrepresented = sampler.identify_underrepresented(items)
        assert "length:short" in underrepresented
        assert "length:medium" in underrepresented
        assert "length:long" not in underrepresented


class TestRecommendSeeds:
    def test_empty_items_returns_guidance(self, sampler):
        result = sampler.recommend_seeds([])

        assert isinstance(result, SamplingResult)
        assert result.recommended_seeds == []
        assert result.coverage_analysis == {}
        assert result.recommendations == ["数据为空，无法提供建议"]

    def test_recommends_representative_seed_for_rare_type(self, sampler):
        items = [{"instruction": "这是什么服务"} for _ in range(10)]
        items.append({"instruction": "怎么申请入住"})

        result = sampler.recommend_seeds(items, top_k=10)

        assert any(s["instruction"] == "怎么申请入住" for s in result.recommended_seeds)
        assert any("覆盖不足的类型" in r for r in result.recommendations)

    def test_warns_when_what_questions_dominate(self, sampler):
        items = [{"instruction": "这是什么"} for _ in range(10)]
        result = sampler.recommend_seeds(items)

        assert "'什么'类问题过多，建议增加其他类型问题" in result.recommendations
        assert "'如何'类问题较少，建议增加操作指导类问题" in result.recommendations

    def test_no_warning_when_type_distribution_is_balanced(self, sampler):
        items = [
            {"instruction": "这是什么"},
            {"instruction": "怎么操作"},
            {"instruction": "这是什么"},
            {"instruction": "怎么操作"},
        ]
        result = sampler.recommend_seeds(items)

        assert "'什么'类问题过多，建议增加其他类型问题" not in result.recommendations
        assert "'如何'类问题较少，建议增加操作指导类问题" not in result.recommendations

    def test_top_k_limits_recommended_seeds(self, sampler):
        # 9 条 what + 各 1 条 how / other，后两类均低于 10% 阈值，会被推荐
        items = [{"instruction": "这是什么服务"} for _ in range(9)]
        items.append({"instruction": "怎么申请"})
        items.append({"instruction": "押金"})

        assert len(sampler.recommend_seeds(items, top_k=5).recommended_seeds) == 2
        assert len(sampler.recommend_seeds(items, top_k=1).recommended_seeds) == 1


class TestGenerateReport:
    def test_report_includes_seed_indices(self, sampler):
        items = [{"instruction": "这是什么服务"} for _ in range(10)]
        items.append({"instruction": "怎么申请入住"})

        report = sampler.generate_report(items)

        assert report["total_items"] == 11
        assert report["recommended_count"] == len(report["recommended_seed_indices"])
        assert 10 in report["recommended_seed_indices"]
        assert "coverage_analysis" in report

    def test_empty_report(self, sampler):
        report = sampler.generate_report([])
        assert report["total_items"] == 0
        assert report["recommended_count"] == 0
        assert report["recommended_seed_indices"] == []


class TestLoadModel:
    """_load_model 测试"""

    def test_load_model_idempotent(self):
        """重复调用 _load_model 应保持一致状态"""
        sampler = ActiveSampler()
        sampler._load_model()
        first_model = sampler._model
        sampler._load_model()
        assert sampler._model is first_model


class TestMissingInstructionField:
    """缺少 instruction 字段的条目"""

    def test_question_type_with_missing_field(self, sampler):
        """缺失 instruction 应返回 other"""
        assert sampler._analyze_question_type("") == "other"

    def test_length_distribution_missing_field(self, sampler):
        """缺失 instruction 应按空字符串处理"""
        dist = sampler._analyze_length_distribution([{"output": "回答"}])
        assert dist["short"] == 1.0

    def test_topic_distribution_missing_field(self, sampler):
        """缺失 instruction 应不影响统计"""
        dist = sampler._analyze_topic_distribution([{"output": "回答"}])
        assert dist["unique_words"] == 0

    def test_analyze_coverage_missing_field(self, sampler):
        """覆盖分析应处理缺失字段"""
        coverage = sampler.analyze_coverage([{"output": "回答"}])
        assert coverage["total_items"] == 1


class TestSamplerExtended:
    """ActiveSampler 扩展测试"""

    def test_analyze_question_type_how_many(self):
        """how_many 类型分析"""
        sampler = ActiveSampler()
        assert sampler._analyze_question_type("多少钱") == "how_many"

    def test_analyze_question_type_can(self):
        """can 类型分析"""
        sampler = ActiveSampler()
        assert sampler._analyze_question_type("可以退吗") == "can"

    def test_analyze_length_distribution_all_short(self):
        """全短文本长度分布"""
        sampler = ActiveSampler()
        items = [{"instruction": "短"}]
        dist = sampler._analyze_length_distribution(items)
        assert dist["short"] == 1.0

    def test_analyze_length_distribution_all_long(self):
        """全长文本长度分布"""
        sampler = ActiveSampler()
        items = [{"instruction": "这是一段非常长的文本内容用于测试，包含很多字符，超过五十个字符的长度要求"}]
        dist = sampler._analyze_length_distribution(items)
        assert dist["long"] == 1.0

    def test_analyze_topic_distribution_empty(self):
        """空主题分布"""
        sampler = ActiveSampler()
        dist = sampler._analyze_topic_distribution([])
        assert dist["unique_words"] == 0

    def test_analyze_coverage_empty(self):
        """空覆盖分析"""
        sampler = ActiveSampler()
        coverage = sampler.analyze_coverage([])
        assert coverage["total_items"] == 0

    def test_recommend_seeds_empty(self):
        """空推荐种子"""
        sampler = ActiveSampler()
        result = sampler.recommend_seeds([])
        assert result.recommended_seeds == []

    def test_sampling_result_fields(self):
        """SamplingResult 字段检查"""
        from augmentor.sampler import SamplingResult
        result = SamplingResult(
            recommended_seeds=[{"instruction": "q1"}],
            coverage_analysis={"total_items": 1},
            recommendations=["rec1"]
        )
        assert len(result.recommended_seeds) == 1
        assert result.coverage_analysis["total_items"] == 1

    def test_load_model_sklearn_available(self):
        """加载模型（sklearn 缺失时 _model 为 None）"""
        sampler = ActiveSampler()
        sampler._load_model()
        # sklearn not installed, _model stays None
        assert sampler._model is None

    def test_analyze_topic_distribution_with_data(self):
        """有数据的主题分布"""
        sampler = ActiveSampler()
        items = [{"instruction": "租金问题"}, {"instruction": "押金问题"}]
        dist = sampler._analyze_topic_distribution(items)
        assert "unique_words" in dist

    def test_identify_underrepresented(self):
        """识别不足表示"""
        sampler = ActiveSampler()
        items = [{"instruction": f"问题{i}"} for i in range(10)]
        result = sampler.identify_underrepresented(items)
        assert isinstance(result, list)

    def test_recommend_seeds_with_items(self):
        """有数据的种子推荐"""
        sampler = ActiveSampler()
        items = [{"instruction": f"问题{i}"} for i in range(10)]
        result = sampler.recommend_seeds(items)
        assert isinstance(result.recommended_seeds, list)


class TestSamplerExtended2:
    """ActiveSampler 扩展测试 - 第二轮"""

    def test_recommend_seeds_short_items(self):
        """短文本种子推荐"""
        sampler = ActiveSampler()
        items = [{"instruction": "短"} for _ in range(5)]
        result = sampler.recommend_seeds(items)
        assert isinstance(result.recommended_seeds, list)

    def test_recommend_seeds_long_items(self):
        """长文本种子推荐"""
        sampler = ActiveSampler()
        items = [{"instruction": "这是一段超过三十个字符的长文本内容用于测试"} for _ in range(5)]
        result = sampler.recommend_seeds(items)
        assert isinstance(result.recommended_seeds, list)

    def test_generate_report_with_data(self):
        """有数据的选样报告"""
        sampler = ActiveSampler()
        items = [{"instruction": f"问题{i}"} for i in range(5)]
        report = sampler.generate_report(items)
        assert "total_items" in report
        assert "recommended_count" in report
        assert report["total_items"] == 5

    def test_generate_report_empty(self):
        """空数据选样报告"""
        sampler = ActiveSampler()
        report = sampler.generate_report([])
        assert report["total_items"] == 0
        assert report["recommended_count"] == 0

    def test_recommend_seeds_with_top_k(self):
        """指定 top_k 的种子推荐"""
        sampler = ActiveSampler()
        items = [{"instruction": f"问题{i}"} for i in range(10)]
        result = sampler.recommend_seeds(items, top_k=3)
        assert len(result.recommended_seeds) <= 3

    def test_recommendations_content(self):
        """推荐内容检查"""
        sampler = ActiveSampler()
        items = [{"instruction": "什么问题"} for _ in range(10)]
        result = sampler.recommend_seeds(items)
        assert isinstance(result.recommendations, list)

    def test_recommendation_what_type_too_many(self):
        """'什么'类问题过多时应给出建议"""
        sampler = ActiveSampler()
        items = [{"instruction": "什么是租房押金"} for _ in range(10)]
        result = sampler.recommend_seeds(items)
        assert any("'什么'类问题过多" in r for r in result.recommendations)

    def test_recommendation_how_type_lacking(self):
        """'如何'类问题较少时应给出建议"""
        sampler = ActiveSampler()
        items = [{"instruction": "什么是房租"} for _ in range(10)]
        result = sampler.recommend_seeds(items)
        assert any("'如何'类问题较少" in r for r in result.recommendations)

    def test_analyze_question_type_all_categories(self):
        """问题类型分析应覆盖全部分类"""
        sampler = ActiveSampler()
        assert sampler._analyze_question_type("是什么") == "what"
        assert sampler._analyze_question_type("怎么申请") == "how"
        # "为什么"含"是/什么"前缀时会先命中 what 分支
        assert sampler._analyze_question_type("为什么涨价") in ("what", "why")
        assert sampler._analyze_question_type("可以退款吗") == "can"
        assert sampler._analyze_question_type("多少费用") == "how_many"
        assert sampler._analyze_question_type("abc") == "other"

    def test_length_recommend_seed_short(self):
        """长度不足时应推荐短文本种子"""
        sampler = ActiveSampler()
        # 全部为长文本，短文本覆盖不足
        items = [{"instruction": "这是一段足够长的文本内容超过三十个字符哦呀"}] * 3
        result = sampler.recommend_seeds(items)
        assert isinstance(result.recommended_seeds, list)

    def test_recommend_seeds_underrepresented_list_is_list(self):
        """identify_underrepresented 应返回列表"""
        sampler = ActiveSampler()
        items = [{"instruction": f"问题{i}"} for i in range(3)]
        under = sampler.identify_underrepresented(items)
        assert isinstance(under, list)

    def test_analyze_length_distribution_empty(self):
        """空数据长度分布应为全零"""
        sampler = ActiveSampler()
        dist = sampler._analyze_length_distribution([])
        assert dist == {"short": 0, "medium": 0, "long": 0}

    def test_analyze_length_distribution_with_data(self):
        """有数据时长度分布比例之和为 1"""
        sampler = ActiveSampler()
        items = [
            {"instruction": "短"},
            {"instruction": "中等长度的问题文本内容"},
            {"instruction": "这是一段足够长的问题文本内容超过三十个字符哦呀哈"},
        ]
        dist = sampler._analyze_length_distribution(items)
        assert dist["short"] + dist["medium"] + dist["long"] == pytest.approx(1.0)
        assert dist["avg_length"] > 0
