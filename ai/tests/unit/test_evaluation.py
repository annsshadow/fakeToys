"""模型评估单元测试

评估指标用于模型选型与回归对比，指标定义必须可解释、可复现。
"""

import pytest

from augmentor.evaluation import (
    ModelEvaluator,
    compute_bleu,
    compute_rouge_l,
    compute_similarity,
    tokenize,
    _ngrams,
    _lcs_length,
    EvaluationResult,
    SUPPORTED_METRICS,
)


class TestTokenizer:
    """分词"""

    def test_splits_chinese_by_character(self):
        """中文按字切分，保证无需分词词典即可计算"""
        assert tokenize("你好") == ["你", "好"]

    def test_splits_english_by_word(self):
        """英文按词切分"""
        assert tokenize("hello world") == ["hello", "world"]

    def test_mixed_text(self):
        """中英混合需同时处理两类词元"""
        assert tokenize("你好world") == ["你", "好", "world"]

    def test_empty(self):
        """空文本返回空列表"""
        assert tokenize("") == []

    def test_punctuation_dropped(self):
        """标点不作为词元"""
        assert tokenize("你好，世界！") == ["你", "好", "世", "界"]


class TestBleu:
    """BLEU"""

    def test_identical_text_scores_one(self):
        """完全一致的文本 BLEU 应为 1，这是指标自洽的基本要求"""
        assert compute_bleu("你好世界", "你好世界") == pytest.approx(1.0)

    def test_completely_different_scores_zero(self):
        """毫无重叠的文本应得 0 分"""
        assert compute_bleu("你好世界", "abcd") == pytest.approx(0.0)

    def test_partial_overlap_between_zero_and_one(self):
        """部分重叠应落在开区间内"""
        score = compute_bleu("你好世界测试", "你好世界")
        assert 0.0 < score < 1.0

    def test_empty_inputs(self):
        """空输入返回 0 而不是抛异常"""
        assert compute_bleu("", "abc") == 0.0
        assert compute_bleu("abc", "") == 0.0

    def test_brevity_penalty_applies(self):
        """过短生成应被简短惩罚压低分数"""
        short = compute_bleu("你好", "你好世界测试内容")
        longer = compute_bleu("你好世界测试内容", "你好世界测试内容")
        assert short < longer


class TestRougeL:
    """ROUGE-L"""

    def test_identical_scores_one(self):
        """完全一致应为满分"""
        assert compute_rouge_l("你好世界", "你好世界") == pytest.approx(1.0)

    def test_no_overlap_zero(self):
        """无公共子序列为 0"""
        assert compute_rouge_l("abcd", "wxyz") == pytest.approx(0.0)

    def test_subset_scores_between(self):
        """子集关系应给出中间分数"""
        score = compute_rouge_l("你好世界", "你好世界测试")
        assert 0.0 < score < 1.0

    def test_empty_inputs(self):
        """空输入返回 0"""
        assert compute_rouge_l("", "abc") == 0.0

    def test_non_string_input(self):
        """非字符串输入"""
        assert compute_rouge_l(123, "abc") == 0.0


class TestSimilarity:
    """Jaccard 相似度"""

    def test_identical(self):
        """完全一致为 1"""
        assert compute_similarity("你好世界", "你好世界") == pytest.approx(1.0)

    def test_disjoint(self):
        """完全不同为 0"""
        assert compute_similarity("abcd", "wxyz") == pytest.approx(0.0)

    def test_empty(self):
        """空输入为 0"""
        assert compute_similarity("", "abc") == 0.0


class TestEvaluatorInit:
    """评估器初始化"""

    def test_default_metrics(self):
        """默认使用三种指标"""
        assert ModelEvaluator().metrics == ["bleu", "rouge_l", "similarity"]

    def test_rejects_unknown_metric(self):
        """未知指标必须报错并列出支持项"""
        with pytest.raises(ValueError) as excinfo:
            ModelEvaluator(metrics=["perplexity"])
        assert "不支持的评估指标" in str(excinfo.value)

    def test_supported_metrics_constant(self):
        """支持指标清单需包含计划约定的三项"""
        assert set(SUPPORTED_METRICS) == {"bleu", "rouge_l", "similarity"}


class TestEvaluation:
    """评估主流程"""

    def test_evaluate_quality_averages_metrics(self):
        """综合分应为各指标均值，且落在 0-1"""
        score = ModelEvaluator().evaluate_quality("你好世界", "你好世界")
        assert score == pytest.approx(1.0)

    def test_evaluate_pair_returns_all_metrics(self):
        """单条评估需给出全部指标明细"""
        scores = ModelEvaluator().evaluate_pair("你好", "你好")
        assert set(scores.keys()) == {"bleu", "rouge_l", "similarity"}

    def test_batch_evaluation(self):
        """批量评估需返回平均值与样本数"""
        result = ModelEvaluator().evaluate_batch(
            ["你好世界", "abc"], ["你好世界", "abc"]
        )

        assert result.sample_count == 2
        assert result.metrics["bleu"] == pytest.approx(1.0)

    def test_batch_length_mismatch_raises(self):
        """长度不一致必须报错，避免错位比较"""
        with pytest.raises(ValueError):
            ModelEvaluator().evaluate_batch(["a"], ["a", "b"])

    def test_batch_empty(self):
        """空批量应返回零值结果"""
        result = ModelEvaluator().evaluate_batch([], [])
        assert result.sample_count == 0
        assert result.metrics["bleu"] == 0.0


class TestCompareOutputs:
    """输出对比"""

    def test_identifies_better_model(self):
        """更接近参考文本的一方应被判定为胜出"""
        references = ["你好世界测试", "abcd efgh"]
        outputs_a = ["你好世界测试", "abcd efgh"]
        outputs_b = ["完全无关内容", "另一个无关"]

        result = ModelEvaluator().compare_outputs(outputs_a, outputs_b, references)

        assert result["winner"] == "a"
        assert result["win_counts"]["a"] == 2

    def test_tie_detection(self):
        """完全相同的输出应判定为平局"""
        references = ["你好世界"]
        result = ModelEvaluator().compare_outputs(
            ["你好世界"], ["你好世界"], references
        )
        assert result["winner"] == "tie"

    def test_result_contains_both_models(self):
        """对比结果需同时给出双方指标，便于汇报"""
        result = ModelEvaluator().compare_outputs(["a"], ["b"], ["a"])
        assert "model_a" in result and "model_b" in result


class FakeModel:
    """模拟模型后端"""

    def __init__(self, response):
        self.response = response

    def generate(self, prompt: str) -> str:
        """返回固定响应

        Args:
            prompt: 提示词

        Returns:
            固定响应
        """
        return self.response


class TestCompareModels:
    """模型对比"""

    def test_compares_with_references(self):
        """提供参考文本时应给出胜负统计"""
        evaluator = ModelEvaluator()
        result = evaluator.compare_models(
            FakeModel("你好世界"),
            FakeModel("无关内容"),
            prompts=["p"],
            references=["你好世界"]
        )

        assert result["winner"] == "a"

    def test_without_references_returns_outputs(self):
        """未提供参考文本时只返回双方输出，不做打分"""
        evaluator = ModelEvaluator()
        result = evaluator.compare_models(
            FakeModel("A"), FakeModel("B"), prompts=["p1", "p2"]
        )

        assert result["reference_provided"] is False
        assert result["model_a_outputs"] == ["A", "A"]

    def test_string_model_requires_factory(self):
        """传入字符串模型名但无工厂时必须报错，避免静默失败"""
        evaluator = ModelEvaluator()
        with pytest.raises(ValueError):
            evaluator.compare_models("model-a", "model-b", prompts=["p"])

    def test_string_model_resolved_by_factory(self):
        """提供工厂时应能解析模型名"""
        backends = {"a": FakeModel("你好"), "b": FakeModel("你好")}
        evaluator = ModelEvaluator(model_factory=lambda name: backends[name])

        result = evaluator.compare_models("a", "b", prompts=["p"], references=["你好"])
        assert result["winner"] == "tie"


class TestNgrams:
    """n-gram 计算"""

    def test_unigrams(self):
        """1-gram 应返回每个 token 的计数"""
        result = _ngrams(["a", "b", "a"], 1)
        assert result[("a",)] == 2
        assert result[("b",)] == 1

    def test_bigrams(self):
        """2-gram 应返回相邻 token 对"""
        result = _ngrams(["a", "b", "c"], 2)
        assert result[("a", "b")] == 1
        assert result[("b", "c")] == 1

    def test_empty_tokens(self):
        """空 token 列表应返回空 Counter"""
        result = _ngrams([], 2)
        assert len(result) == 0

    def test_n_larger_than_tokens(self):
        """n 大于 token 数量时应返回空"""
        result = _ngrams(["a"], 3)
        assert len(result) == 0


class TestLCSLength:
    """最长公共子序列"""

    def test_identical(self):
        """相同序列 LCS 应等于长度"""
        assert _lcs_length(["a", "b", "c"], ["a", "b", "c"]) == 3

    def test_no_common(self):
        """无公共元素 LCS 应为 0"""
        assert _lcs_length(["a", "b"], ["c", "d"]) == 0

    def test_subsequence(self):
        """子序列场景"""
        assert _lcs_length(["a", "c"], ["a", "b", "c"]) == 2

    def test_empty_input(self):
        """空输入应返回 0"""
        assert _lcs_length([], ["a"]) == 0
        assert _lcs_length(["a"], []) == 0


class TestEvaluationResult:
    """EvaluationResult 数据类"""

    def test_to_dict(self):
        """to_dict 应返回完整结构"""
        result = EvaluationResult(
            metrics={"bleu": 0.8, "rouge_l": 0.9},
            sample_count=5,
            details=[{"text": "test"}]
        )
        d = result.to_dict()
        assert d["metrics"]["bleu"] == 0.8
        assert d["sample_count"] == 5
        assert len(d["details"]) == 1

    def test_default_values(self):
        """默认值应正确"""
        result = EvaluationResult()
        assert result.metrics == {}
        assert result.sample_count == 0
        assert result.details == []


class TestBleuExtended:
    """BLEU 扩展测试"""

    def test_max_n_1(self):
        """max_n=1 时应只使用 unigram 精度"""
        score = compute_bleu("你好世界", "你好世界", max_n=1)
        assert score == pytest.approx(1.0)

    def test_score_bounded(self):
        """分数应始终在 0-1 范围内"""
        score = compute_bleu("很长的生成文本用于测试边界情况", "短参考", max_n=2)
        assert 0.0 <= score <= 1.0


class TestRougeLExtended:
    """ROUGE-L 扩展测试"""

    def test_subset_direction(self):
        """子集关系 ROUGE-L 应为中间值"""
        score = compute_rouge_l("你好世界", "你好世界测试")
        assert 0.0 < score < 1.0


class TestSimilarityExtended:
    """相似度扩展测试"""

    def test_partial_overlap(self):
        """部分重叠应为中间值"""
        score = compute_similarity("你好世界", "你好大家")
        assert 0.0 < score < 1.0
