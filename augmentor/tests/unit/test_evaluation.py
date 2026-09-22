# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""evaluation 模块单元测试

覆盖 tokenize / BLEU / ROUGE-L / 相似度 / ModelEvaluator
（质量、成对、批量、输出对比、模型对比与后端解析）。
"""

import pytest

from augmentor.evaluation import (
    ModelEvaluator,
    compute_bleu,
    compute_rouge_l,
    compute_similarity,
    tokenize,
)


class TestTokenize:
    def test_chinese_char_split(self):
        assert tokenize("租房") == ["租", "房"]

    def test_latin_words(self):
        assert tokenize("hello world") == ["hello", "world"]

    def test_mixed(self):
        tokens = tokenize("AI模型")
        assert "AI" in tokens
        assert "模" in tokens and "型" in tokens

    def test_empty_and_non_str(self):
        assert tokenize("") == []
        assert tokenize(None) == []
        assert tokenize(123) == []

    def test_punctuation_splits_words(self):
        assert tokenize("a,b c-d") == ["a", "b", "c", "d"]


class TestBleu:
    def test_identical_texts_high_score(self):
        assert compute_bleu("租房多少钱", "租房多少钱") > 0.9

    def test_disjoint_texts_zero(self):
        assert compute_bleu("aaaa", "bbbb") == 0.0

    def test_empty_inputs(self):
        assert compute_bleu("", "x") == 0.0
        assert compute_bleu("x", "") == 0.0

    def test_short_generation_penalized(self):
        long_ref = "这是一个非常长的参考文本需要很多词来构成对比"
        assert compute_bleu("这", long_ref) < compute_bleu(long_ref, long_ref)


class TestRougeL:
    def test_identical(self):
        assert compute_rouge_l("abc", "abc") == pytest.approx(1.0)

    def test_partial_overlap(self):
        score = compute_rouge_l("租房申请流程", "租房退租押金")
        assert 0.0 < score < 1.0

    def test_no_overlap_zero(self):
        assert compute_rouge_l("aaa", "bbb") == 0.0

    def test_empty(self):
        assert compute_rouge_l("", "x") == 0.0


class TestSimilarity:
    def test_identical_sets(self):
        assert compute_similarity("hello world", "hello world") == pytest.approx(1.0)

    def test_partial(self):
        score = compute_similarity("a b c", "a b d")
        assert 0.0 < score < 1.0

    def test_disjoint_zero(self):
        assert compute_similarity("a b", "c d") == 0.0

    def test_empty_zero(self):
        assert compute_similarity("", "x") == 0.0


class TestModelEvaluatorValidation:
    def test_unsupported_metric_raises(self):
        with pytest.raises(ValueError, match="不支持"):
            ModelEvaluator(metrics=["nonsense"])

    def test_default_metrics(self):
        evaluator = ModelEvaluator()
        assert evaluator.metrics == ["bleu", "rouge_l", "similarity"]


class TestEvaluateQuality:
    def test_quality_identical(self):
        evaluator = ModelEvaluator(metrics=["similarity"])
        assert evaluator.evaluate_quality("租房", "租房") == pytest.approx(1.0)

    def test_quality_mean_of_metrics(self):
        evaluator = ModelEvaluator(metrics=["bleu", "rouge_l", "similarity"])
        score = evaluator.evaluate_quality("x", "y")
        assert 0.0 <= score <= 1.0


class TestEvaluatePair:
    def test_pair_returns_all_metrics(self):
        evaluator = ModelEvaluator()
        pair = evaluator.evaluate_pair("hello", "hello")
        assert set(pair) == {"bleu", "rouge_l", "similarity"}
        assert all(0.0 <= v <= 1.0 for v in pair.values())


class TestEvaluateBatch:
    def test_batch_averages_and_details(self):
        evaluator = ModelEvaluator(metrics=["similarity"])
        result = evaluator.evaluate_batch(["abc", "xyz"], ["abc", "xyz"])
        assert result.sample_count == 2
        assert result.metrics["similarity"] == pytest.approx(1.0)
        assert len(result.details) == 2
        assert result.details[0]["index"] == 0

    def test_batch_mismatched_lengths_raise(self):
        evaluator = ModelEvaluator()
        with pytest.raises(ValueError, match="长度不一致"):
            evaluator.evaluate_batch(["a"], ["a", "b"])

    def test_batch_empty(self):
        evaluator = ModelEvaluator()
        result = evaluator.evaluate_batch([], [])
        assert result.sample_count == 0
        assert all(v == 0.0 for v in result.metrics.values())

    def test_to_dict(self):
        evaluator = ModelEvaluator(metrics=["bleu"])
        result = evaluator.evaluate_batch(["a"], ["a"])
        payload = result.to_dict()
        assert payload["sample_count"] == 1
        assert "metrics" in payload and "details" in payload


class TestCompareOutputs:
    def test_winner_a_when_a_better(self):
        evaluator = ModelEvaluator(metrics=["similarity"])
        result = evaluator.compare_outputs(
            ["good", "good"], ["bad", "bad"], ["good", "good"]
        )
        assert result["winner"] == "a"
        assert result["win_counts"]["a"] == 2

    def test_winner_b_when_b_better(self):
        evaluator = ModelEvaluator(metrics=["similarity"])
        result = evaluator.compare_outputs(
            ["bad", "bad"], ["good", "good"], ["good", "good"]
        )
        assert result["winner"] == "b"

    def test_tie_when_identical(self):
        evaluator = ModelEvaluator(metrics=["similarity"])
        result = evaluator.compare_outputs(["same"], ["same"], ["same"])
        assert result["winner"] == "tie"
        assert result["win_counts"]["tie"] == 1

    def test_result_contains_both_reports(self):
        evaluator = ModelEvaluator(metrics=["bleu"])
        result = evaluator.compare_outputs(["a"], ["a"], ["a"])
        assert "model_a" in result and "model_b" in result
        assert result["model_a"]["sample_count"] == 1


class _FakeBackend:
    """固定输出的假模型后端"""

    def __init__(self, outputs):
        self._outputs = list(outputs)

    def generate(self, prompt):
        return self._outputs.pop(0)


class TestCompareModels:
    def test_with_references_compares_outputs(self):
        evaluator = ModelEvaluator(metrics=["similarity"])
        backend_a = _FakeBackend(["good", "good"])
        backend_b = _FakeBackend(["bad", "bad"])
        result = evaluator.compare_models(
            backend_a, backend_b, ["p1", "p2"], references=["good", "good"]
        )
        assert result["winner"] == "a"

    def test_without_references_returns_outputs(self):
        evaluator = ModelEvaluator()
        backend_a = _FakeBackend(["a1", "a2"])
        backend_b = _FakeBackend(["b1", "b2"])
        result = evaluator.compare_models(backend_a, backend_b, ["p1", "p2"])
        assert result["reference_provided"] is False
        assert result["model_a_outputs"] == ["a1", "a2"]
        assert result["model_b_outputs"] == ["b1", "b2"]

    def test_string_model_requires_factory(self):
        evaluator = ModelEvaluator()
        with pytest.raises(ValueError, match="model_factory"):
            evaluator.compare_models("ernie", "openai", ["p"])

    def test_string_model_resolved_by_factory(self):
        def factory(name):
            return {"ernie": _FakeBackend(["x"]), "openai": _FakeBackend(["y"])}[name]

        evaluator = ModelEvaluator(metrics=["similarity"], model_factory=factory)
        result = evaluator.compare_models("ernie", "openai", ["p"])
        assert result["reference_provided"] is False
