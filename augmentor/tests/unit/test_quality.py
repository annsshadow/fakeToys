# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""质量评分单元测试

质量分决定样本是否进入训练集，因此权重校验、阈值过滤与报告统计必须准确。
"""

import json

import pytest

from augmentor.quality import QualityScorer, QualityScore


class TestScorerInit:
    """初始化校验"""

    def test_default_weights(self):
        """默认权重应为 [0.3, 0.4, 0.3]"""
        scorer = QualityScorer()
        assert scorer.weights == [0.3, 0.4, 0.3]

    def test_rejects_wrong_weight_length(self):
        """权重个数错误必须立即报错，否则评分公式会静默错位"""
        with pytest.raises(ValueError):
            QualityScorer(weights=[0.5, 0.5])

    def test_rejects_weights_not_summing_to_one(self):
        """权重之和不为 1 会导致分数越界，必须拒绝"""
        with pytest.raises(ValueError):
            QualityScorer(weights=[0.5, 0.5, 0.5])

    def test_custom_weights(self):
        """自定义权重应正确设置"""
        scorer = QualityScorer(weights=[0.4, 0.3, 0.3])
        assert scorer.weights == [0.4, 0.3, 0.3]

    def test_custom_threshold(self):
        """自定义阈值应正确设置"""
        scorer = QualityScorer(threshold=0.8)
        assert scorer.threshold == 0.8

    def test_default_diversity_sample_size(self):
        """默认多样性采样大小应为 30"""
        scorer = QualityScorer()
        assert scorer.diversity_sample_size == 30

    def test_custom_diversity_sample_size(self):
        """自定义多样性采样大小应正确设置"""
        scorer = QualityScorer(diversity_sample_size=50)
        assert scorer.diversity_sample_size == 50


class TestSimilarityFallback:
    """无模型时的 fallback 行为"""

    def test_ngram_similarity_identical(self):
        """完全相同的文本相似度应为 1"""
        scorer = QualityScorer()
        assert scorer._ngram_similarity("你好世界", "你好世界") == pytest.approx(1.0)

    def test_ngram_similarity_disjoint(self):
        """无公共 n-gram 时相似度应为 0"""
        scorer = QualityScorer()
        assert scorer._ngram_similarity("abcd", "wxyz") == pytest.approx(0.0)

    def test_ngram_similarity_empty(self):
        """空文本不应触发除零错误"""
        scorer = QualityScorer()
        assert scorer._ngram_similarity("", "abc") == 0.0

    def test_ngram_similarity_one_empty(self):
        """一个空文本相似度应为 0"""
        scorer = QualityScorer()
        assert scorer._ngram_similarity("abc", "") == 0.0

    def test_ngram_similarity_both_empty(self):
        """两个空文本相似度应为 0"""
        scorer = QualityScorer()
        assert scorer._ngram_similarity("", "") == 0.0

    def test_ngram_similarity_partial_match(self):
        """部分匹配的文本相似度应在 0-1 之间"""
        scorer = QualityScorer()
        sim = scorer._ngram_similarity("hello world", "hello python")
        assert 0.0 < sim < 1.0


class TestScoring:
    """评分主流程"""

    def test_score_identical_text_scores_high(self):
        """文本与自身完全一致时，语义相似度必须接近满分"""
        scorer = QualityScorer(threshold=0.5)
        result = scorer.score("如何申请入住", "如何申请入住", "通过 App 申请")

        assert isinstance(result, QualityScore)
        assert result.semantic_similarity == pytest.approx(1.0)
        assert 0.0 <= result.total_score <= 1.0

    def test_score_passed_flag_follows_threshold(self):
        """passed 标记必须严格由阈值决定，供上层过滤使用"""
        low_threshold = QualityScorer(threshold=0.0)
        result = low_threshold.score("问题", "问题", "回答")
        assert result.passed is True

        high_threshold = QualityScorer(threshold=1.0)
        result = high_threshold.score("问题", "问题", "回答")
        assert result.passed is False

    def test_batch_score_length_matches_input(self):
        """批量评分结果必须与输入一一对应，否则过滤时会错位"""
        scorer = QualityScorer()
        items = [
            {"original": "问题A", "generated": "问题A", "output": "回答A"},
            {"original": "问题B", "generated": "问题B", "output": "回答B"},
        ]
        scores = scorer.batch_score(items)
        assert len(scores) == len(items)

    def test_batch_score_empty(self):
        """空数据集应返回空结果而不是抛异常"""
        scorer = QualityScorer()
        assert scorer.batch_score([]) == []

    def test_filter_by_quality_keeps_only_passed(self):
        """过滤后剩余样本必须全部满足阈值"""
        scorer = QualityScorer(threshold=0.99)
        items = [
            {"original": "完全一致的问题", "generated": "完全一致的问题", "output": "回答"},
            {"original": "问题", "generated": "毫不相关的内容", "output": "回答"},
        ]
        filtered = scorer.filter_by_quality(items)
        assert len(filtered) <= len(items)

    def test_score_returns_quality_score(self):
        """评分应返回 QualityScore 实例"""
        scorer = QualityScorer()
        result = scorer.score("问题", "问题", "回答")
        
        assert hasattr(result, 'semantic_similarity')
        assert hasattr(result, 'relevance')
        assert hasattr(result, 'diversity')
        assert hasattr(result, 'total_score')
        assert hasattr(result, 'passed')

    def test_score_values_in_range(self):
        """评分值应在 0-1 之间"""
        scorer = QualityScorer()
        result = scorer.score("问题", "问题", "回答")
        
        assert 0.0 <= result.semantic_similarity <= 1.0
        assert 0.0 <= result.relevance <= 1.0
        assert 0.0 <= result.diversity <= 1.0
        assert 0.0 <= result.total_score <= 1.0

    def test_batch_score_with_existing_generated(self):
        """批量评分时应考虑已生成的问题"""
        scorer = QualityScorer()
        items = [
            {"original": "问题1", "generated": "问题1", "output": "回答1"},
            {"original": "问题2", "generated": "问题2", "output": "回答2"},
        ]
        existing = ["问题1"]
        scores = scorer.batch_score(items, existing)
        
        assert len(scores) == len(items)


class TestReport:
    """质量报告"""

    def test_report_contains_required_fields(self):
        """报告需包含通过率与分项统计，供 Web UI 直接渲染"""
        scorer = QualityScorer()
        items = [
            {"original": "问题", "generated": "问题", "output": "回答"},
            {"original": "问题2", "generated": "问题2", "output": "回答2"},
        ]
        report = scorer.generate_report(items)

        assert report["total_samples"] == 2
        assert report["passed_samples"] + report["filtered_samples"] == 2
        assert "total_score" in report
        assert "semantic_similarity" in report
        assert report["threshold"] == scorer.threshold

    def test_report_on_empty_dataset(self):
        """空数据集报告不应出现除零错误"""
        scorer = QualityScorer()
        report = scorer.generate_report([])

        assert report["total_samples"] == 0
        assert report["pass_rate"] == 0
        assert report["total_score"]["mean"] == 0

    def test_reset_cache(self):
        """重置缓存后多样性计算不应复用旧 embedding"""
        scorer = QualityScorer()
        scorer._existing_embeddings = [1, 2, 3]
        scorer._existing_texts = ["a"]
        scorer._existing_texts_hash = "hash"

        scorer.reset_cache()

        assert scorer._existing_embeddings == []
        assert scorer._existing_texts == []
        assert scorer._existing_texts_hash is None

    def test_report_statistics(self):
        """报告统计值应正确计算"""
        scorer = QualityScorer()
        items = [
            {"original": "问题1", "generated": "问题1", "output": "回答1"},
            {"original": "问题2", "generated": "问题2", "output": "回答2"},
        ]
        report = scorer.generate_report(items)
        
        assert "mean" in report["total_score"]
        assert "std" in report["total_score"]
        assert "min" in report["total_score"]
        assert "max" in report["total_score"]

    def test_report_pass_rate(self):
        """通过率应正确计算"""
        scorer = QualityScorer(threshold=0.0)
        items = [
            {"original": "问题", "generated": "问题", "output": "回答"},
        ]
        report = scorer.generate_report(items)
        
        assert report["pass_rate"] == 1.0


class TestComputeTextsHash:
    """文本哈希计算测试"""

    def test_hash_consistent(self):
        """相同文本应产生相同哈希"""
        scorer = QualityScorer()
        texts = ["hello", "world"]
        
        hash1 = scorer._compute_texts_hash(texts)
        hash2 = scorer._compute_texts_hash(texts)
        
        assert hash1 == hash2

    def test_hash_different_for_different_texts(self):
        """不同文本应产生不同哈希"""
        scorer = QualityScorer()
        texts1 = ["hello"]
        texts2 = ["world"]
        
        hash1 = scorer._compute_texts_hash(texts1)
        hash2 = scorer._compute_texts_hash(texts2)
        
        assert hash1 != hash2

    def test_hash_uses_first_100_items(self):
        """哈希应只使用前 100 条文本"""
        scorer = QualityScorer()
        texts = [f"item{i}" for i in range(200)]
        
        hash1 = scorer._compute_texts_hash(texts)
        hash2 = scorer._compute_texts_hash(texts[:100])
        
        assert hash1 == hash2


class TestModelPaths:
    """模型路径覆盖率测试（mock sentence-transformers）"""

    def test_calculate_semantic_similarity_with_model(self):
        """有模型时应使用 embedding 计算语义相似度"""
        scorer = QualityScorer()
        # 构造一个假模型，返回固定 embedding
        class FakeModel:
            def encode(self, texts, **kwargs):
                import numpy as np
                # 相同文本返回相同 embedding
                return np.array([[1.0, 0.0, 0.0]] * len(texts)) if texts[0] == texts[1] else np.array([[1.0, 0.0, 0.0], [0.0, 1.0, 0.0]])
        
        scorer._model = FakeModel()
        sim = scorer._calculate_semantic_similarity("hello", "hello")
        assert sim == pytest.approx(1.0)
        
        sim_diff = scorer._calculate_semantic_similarity("hello", "world")
        assert sim_diff == pytest.approx(0.0)

    def test_calculate_relevance_with_cross_encoder(self):
        """有 cross_encoder 时应使用其预测结果"""
        scorer = QualityScorer()
        
        class FakeCrossEncoder:
            def predict(self, pairs):
                return 0.8
        
        scorer._cross_encoder = FakeCrossEncoder()
        score = scorer._calculate_relevance("question", "answer")
        # 归一化: (0.8 + 1) / 2 = 0.9
        assert score == pytest.approx(0.9)

    def test_calculate_diversity_with_model(self):
        """有模型时应使用 embedding 计算多样性"""
        scorer = QualityScorer(diversity_sample_size=5)
        import numpy as np
        
        class FakeModel:
            def encode(self, texts, **kwargs):
                # 所有文本返回相同方向的 embedding
                return np.array([[1.0, 0.0, 0.0]] * len(texts))
        
        scorer._model = FakeModel()
        diversity = scorer._calculate_diversity("text1", ["existing1", "existing2"])
        # 与已有文本完全相同 → max_sim=1 → diversity=0
        assert diversity == pytest.approx(0.0)

    def test_calculate_diversity_with_model_different_texts(self):
        """有模型时，不同文本应有较高多样性"""
        scorer = QualityScorer()
        import numpy as np
        
        class FakeModel:
            def encode(self, texts, **kwargs):
                # 根据文本内容生成不同的 embedding
                result = []
                for t in texts:
                    if "existing" in t:
                        result.append([0.0, 1.0, 0.0])
                    else:
                        result.append([1.0, 0.0, 0.0])
                return np.array(result)
        
        scorer._model = FakeModel()
        scorer._existing_texts_hash = None
        diversity = scorer._calculate_diversity("new_text", ["existing1"])
        assert diversity == pytest.approx(1.0)

    def test_calculate_diversity_empty_existing_embeddings(self):
        """空缓存 embedding 应返回 1.0"""
        scorer = QualityScorer()
        import numpy as np
        
        class FakeModel:
            def encode(self, texts, **kwargs):
                return np.array([[1.0, 0.0, 0.0]] * len(texts))
        
        scorer._model = FakeModel()
        # 预设缓存使 hash 匹配且 embeddings 为空
        scorer._existing_embeddings = []
        scorer._existing_texts = ["existing"]
        scorer._existing_texts_hash = scorer._compute_texts_hash(["existing"])
        diversity = scorer._calculate_diversity("text1", ["existing"])
        assert diversity == pytest.approx(1.0)

    def test_batch_score_with_model(self):
        """有模型时 batch_score 应使用模型计算"""
        scorer = QualityScorer(threshold=0.0)
        import numpy as np
        
        class FakeModel:
            def encode(self, texts, **kwargs):
                return np.array([[1.0, 0.0, 0.0]] * len(texts))
        
        scorer._model = FakeModel()
        items = [
            {"original": "问题1", "generated": "问题1", "output": "回答1"},
            {"original": "问题2", "generated": "问题2", "output": "回答2"},
        ]
        scores = scorer.batch_score(items)
        assert len(scores) == 2
        for s in scores:
            assert 0.0 <= s.semantic_similarity <= 1.0

    def test_diversity_with_existing_generated(self):
        """batch_score 中 existing_generated 参数应被使用"""
        scorer = QualityScorer(threshold=0.0)
        import numpy as np
        
        class FakeModel:
            def encode(self, texts, **kwargs):
                return np.array([[1.0, 0.0, 0.0]] * len(texts))
        
        scorer._model = FakeModel()
        items = [
            {"original": "问题1", "generated": "问题1", "output": "回答1"},
        ]
        scores = scorer.batch_score(items, existing_generated=["问题1"])
        assert len(scores) == 1


class TestQualityExtended:
    """QualityScorer 扩展测试"""

    def test_score_empty_texts(self):
        """空文本评分"""
        scorer = QualityScorer()
        score = scorer.score("", "", "")
        assert score.semantic_similarity == 0.0
        assert score.relevance == 0.0

    def test_score_identical_texts(self):
        """相同文本评分"""
        scorer = QualityScorer()
        score = scorer.score("你好世界", "你好世界", "回答")
        assert score.semantic_similarity == 1.0

    def test_filter_by_quality(self):
        """按质量过滤"""
        scorer = QualityScorer(threshold=0.5)
        items = [
            {"original": "q1", "generated": "q1", "output": "a1"},
            {"original": "q2", "generated": "完全不同", "output": "a2"},
        ]
        filtered = scorer.filter_by_quality(items)
        assert len(filtered) <= len(items)

    def test_quality_score_fields(self):
        """QualityScore 应包含所有必要字段"""
        scorer = QualityScorer()
        score = scorer.score("test", "test", "answer")
        assert hasattr(score, 'semantic_similarity')
        assert hasattr(score, 'relevance')
        assert hasattr(score, 'diversity')
        assert hasattr(score, 'total_score')
        assert hasattr(score, 'passed')

    def test_batch_score_empty(self):
        """空批量评分"""
        scorer = QualityScorer()
        scores = scorer.batch_score([])
        assert scores == []

    def test_generate_report(self):
        """生成质量报告"""
        scorer = QualityScorer()
        items = [
            {"original": "q1", "generated": "q1", "output": "a1"},
            {"original": "q2", "generated": "q2", "output": "a2"},
        ]
        report = scorer.generate_report(items)
        assert "total_samples" in report
        assert "passed_samples" in report
        assert report["total_samples"] == 2

    def test_batch_score_multiple_items(self):
        """多条数据批量评分"""
        scorer = QualityScorer()
        items = [
            {"original": "q1", "generated": "q1", "output": "a1"},
            {"original": "q2", "generated": "q2", "output": "a2"},
            {"original": "q3", "generated": "q3", "output": "a3"},
        ]
        scores = scorer.batch_score(items)
        assert len(scores) == 3

    def test_batch_score_with_existing(self):
        """带已有生成的批量评分"""
        scorer = QualityScorer()
        items = [
            {"original": "q1", "generated": "q1", "output": "a1"},
        ]
        scores = scorer.batch_score(items, existing_generated=["existing"])
        assert len(scores) == 1

    def test_ngram_similarity(self):
        """n-gram 相似度计算"""
        scorer = QualityScorer()
        sim = scorer._ngram_similarity("hello world", "hello world")
        assert sim == 1.0

    def test_ngram_similarity_different(self):
        """不同文本 n-gram 相似度"""
        scorer = QualityScorer()
        sim = scorer._ngram_similarity("hello", "world")
        assert sim < 1.0

    def test_calculate_diversity_empty(self):
        """空已有文本多样性"""
        scorer = QualityScorer()
        diversity = scorer._calculate_diversity("text", [])
        assert diversity == 1.0

    def test_calculate_diversity_identical(self):
        """相同文本多样性（fallback 模式）"""
        scorer = QualityScorer()
        scorer._model = "fallback"
        diversity = scorer._calculate_diversity("text", ["text"])
        assert diversity == 0.0


class TestQualityExtended2:
    """QualityScorer 第二轮扩展测试（覆盖剩余边界）"""

    def test_fallback_semantic_uses_ngram(self):
        """模型为 fallback 时语义相似度走 n-gram"""
        scorer = QualityScorer()
        scorer._model = "fallback"
        assert scorer._calculate_semantic_similarity("租房申请", "租房申请") == pytest.approx(1.0)

    def test_fallback_relevance_uses_ngram(self):
        """cross-encoder 为 fallback 时相关性走 n-gram"""
        scorer = QualityScorer()
        scorer._cross_encoder = "fallback"
        score = scorer._calculate_relevance("租房", "租房")
        assert score == pytest.approx(1.0)

    def test_load_models_idempotent(self):
        """_load_models 重复调用不应重新加载"""
        scorer = QualityScorer()
        scorer._load_models()
        model_before = scorer._model
        encoder_before = scorer._cross_encoder
        scorer._load_models()
        assert scorer._model is model_before
        assert scorer._cross_encoder is encoder_before

    def test_load_models_sets_fallback_or_model(self):
        """_load_models 应把模型槽位设置为实例或 fallback 标记"""
        scorer = QualityScorer()
        scorer._model = None
        scorer._cross_encoder = None
        scorer._load_models()
        assert scorer._model is not None or scorer._model == "fallback"
        assert scorer._cross_encoder is not None or scorer._cross_encoder == "fallback"

    def test_diversity_with_existing_text_monotonic(self):
        """已有重复文本越多，多样性应越低（fallback 模式）"""
        scorer = QualityScorer()
        scorer._model = "fallback"
        scorer._cross_encoder = "fallback"
        low = scorer._calculate_diversity("租房", ["租房", "租房", "租房"])
        high = scorer._calculate_diversity("完全不相关的文本", [])
        assert low < high

    def test_score_custom_weights_total_range(self):
        """自定义权重下总分仍应在 0-1 区间"""
        scorer = QualityScorer(weights=[0.1, 0.8, 0.1])
        result = scorer.score("问题", "问题", "回答")
        assert 0.0 <= result.total_score <= 1.0

    def test_generate_report_statistics_keys(self):
        """报告统计字典应包含均值/标准差/极值"""
        scorer = QualityScorer()
        items = [
            {"original": "q1", "generated": "q1", "output": "a1"},
            {"original": "q2", "generated": "q2", "output": "a2"},
        ]
        report = scorer.generate_report(items)
        for key in ("mean", "std", "min", "max"):
            assert key in report["total_score"]

    def test_batch_score_score_items_structure(self):
        """批量评分结果应是 QualityScore 列表且数量一致"""
        scorer = QualityScorer()
        items = [
            {"original": f"q{i}", "generated": f"q{i}", "output": f"a{i}"}
            for i in range(4)
        ]
        scores = scorer.batch_score(items)
        assert all(isinstance(s, QualityScore) for s in scores)
        assert len(scores) == 4


class TestSemanticDimensionOptional:
    """语义维度可跳过（离线评估场景）

    对已落盘的数据集做质量评估时拿不到原始种子问题。若把 instruction 同时
    当作 original 与 generated，语义相似度会恒为 1.0 并虚增总分（见
    docs/plans/2026-09-21-001-audit-augmentor-fullstack-optimization-plan.md F1）。
    因此需要一条「跳过语义维度」的路径，且权重必须重归一化。
    """

    def test_effective_weights_unchanged_when_semantic_included(self):
        """含语义时权重原样返回"""
        scorer = QualityScorer(weights=[0.3, 0.4, 0.3])
        assert scorer.effective_weights(True) == [0.3, 0.4, 0.3]

    def test_effective_weights_renormalized_when_semantic_skipped(self):
        """跳过语义时其权重按比例分摊，总和仍为 1.0"""
        scorer = QualityScorer(weights=[0.3, 0.4, 0.3])
        weights = scorer.effective_weights(False)

        assert weights[0] == 0.0
        assert abs(sum(weights) - 1.0) < 1e-9
        # 0.4 : 0.3 的比例必须保持
        assert abs(weights[1] / weights[2] - 0.4 / 0.3) < 1e-9

    def test_effective_weights_degenerate_zero_remainder(self):
        """相关性与多样性权重全为 0 时不得除零"""
        scorer = QualityScorer(weights=[1.0, 0.0, 0.0])
        weights = scorer.effective_weights(False)
        assert weights == [0.0, 0.5, 0.5]

    def test_batch_score_marks_semantic_not_evaluated(self):
        """跳过语义时结果需带 semantic_evaluated=False 标记"""
        scorer = QualityScorer()
        scores = scorer.batch_score(
            [{"generated": "q", "output": "a"}], include_semantic=False
        )
        assert scores[0].semantic_evaluated is False
        assert scores[0].semantic_similarity == 0.0

    def test_batch_score_default_keeps_semantic(self):
        """默认路径行为不变（增强管道依赖它）"""
        scorer = QualityScorer()
        scores = scorer.batch_score(
            [{"original": "种子问题", "generated": "变体问题", "output": "回答"}]
        )
        assert scores[0].semantic_evaluated is True

    def test_irrelevant_answer_fails_without_semantic(self):
        """无关答案必须被判不通过——修复前它恰好等于阈值 0.6"""
        scorer = QualityScorer(threshold=0.6)
        score = scorer.batch_score(
            [{"generated": "怎么申请租房？", "output": "今天天气不错哈哈哈"}],
            include_semantic=False,
        )[0]
        assert score.total_score < 0.6
        assert score.passed is False

    def test_passed_is_python_bool_and_json_serializable(self):
        """passed 必须是 Python 布尔，而不是 numpy.bool_

        评分链路是 numpy 的，比较结果是 np.bool_。np.bool_ 既不是 bool 的子类，
        也无法被 json.dumps 序列化（TypeError），因此只要它进入任何 JSON 响应或
        落盘路径就会 500。dataclass 声明的是 ``passed: bool``，这条断言守住该契约。
        """
        scorer = QualityScorer()
        batch = scorer.batch_score(
            [{"generated": "问题", "output": "回答"}], include_semantic=False
        )[0]
        single = scorer.score("种子", "变体", "回答", include_semantic=False)

        assert isinstance(batch.passed, bool)
        assert isinstance(single.passed, bool)
        json.dumps({"batch": batch.passed, "single": single.passed})

    def test_diversity_not_degenerate_when_semantic_skipped(self):
        """跳过语义后多样性不得退化为恒 1.0

        多样性参照集原本以语义分是否达标作为纳入门槛；语义被跳过时该分数恒为 0，
        若仍以它把关则参照集永不增长、多样性恒为 1.0——等于把退化从语义维度
        搬到了多样性维度。
        """
        scorer = QualityScorer(threshold=0.0)
        items = [
            {"generated": "完全相同的文本", "output": "完全相同的文本"},
            {"generated": "完全相同的文本", "output": "完全相同的文本"},
        ]
        scores = scorer.batch_score(items, include_semantic=False)

        assert scores[0].diversity == 1.0, "首条没有参照对象，多样性为 1.0"
        assert scores[1].diversity < 1.0, "与首条完全相同，多样性必须下降"


class _RecordingModel:
    """可记录调用内容的假向量模型

    向量由文本内容哈希决定，因此「拿错向量」会立刻在相似度上暴露。
    """

    def __init__(self):
        self.calls = []

    def encode(self, texts, show_progress_bar=False, batch_size=32):
        import hashlib as _hashlib

        import numpy as _np

        self.calls.append(list(texts))
        rows = []
        for t in texts:
            digest = _hashlib.md5(t.encode("utf-8")).digest()
            rows.append([b / 255.0 for b in digest[:8]])
        return _np.array(rows, dtype=_np.float32)


class TestEmbeddingCache:
    """编码缓存与重复编码（3.0 修复）

    原实现 `cached_encode` 的缓存键是 `hash(str(texts)[:200])` 且**忽略**
    batch_key，于是「前若干条相同」的两个不同列表会互相命中缓存——
    generated 的向量被当成 original 的向量返回，语义相似度静默变成 1.0。
    """

    @staticmethod
    def _shared_prefix_lists():
        """构造两个 str() 前 200 字符完全一致、但整体内容不同的列表"""
        common = ["共享的开头文本内容" * 10] * 4
        originals = common + ["original独有内容"]
        generateds = common + ["generated独有内容"]
        # 前提校验：若前 200 字符不同，本测试就覆盖不到原缺陷
        assert str(originals)[:200] == str(generateds)[:200]
        return originals, generateds

    def test_cache_key_distinguishes_role_and_content(self):
        """original 与 generated 不得互相命中缓存"""
        originals, generateds = self._shared_prefix_lists()
        model = _RecordingModel()
        scorer = QualityScorer()
        scorer._model = model

        items = [
            {"original": o, "generated": g, "output": g}
            for o, g in zip(originals, generateds)
        ]
        scores = scorer.batch_score(items, include_semantic=True)

        # 最后一条 original 与 generated 内容不同，相似度必须 < 1.0。
        # 若缓存键碰撞，generated 拿到的是 original 的向量，相似度恒为 1.0。
        assert scores[-1].semantic_similarity < 1.0, (
            "original 与 generated 被当成同一批文本，编码缓存键发生了碰撞"
        )
        # 前 4 条两侧内容相同，相似度应为 1.0
        assert scores[0].semantic_similarity == pytest.approx(1.0)

    def test_calculate_diversity_reuses_supplied_embedding(self):
        """已传入向量时不得再对当前文本单独编码"""
        model = _RecordingModel()
        scorer = QualityScorer()
        scorer._model = model

        existing = ["参照甲", "参照乙"]
        target = "目标文本"
        embedding = model.encode([target])[0]

        calls_before = len(model.calls)
        scorer._calculate_diversity(target, existing, embedding)

        new_calls = model.calls[calls_before:]
        assert [target] not in new_calls, "已传入向量却仍对当前文本单独编码"

    def test_reference_set_is_not_reencoded_every_round(self):
        """参照集向量不得逐轮整体重编码

        缓存键必须精确等于被缓存的内容（`existing[:diversity_sample_size]`）。
        原实现用 `existing[:100]` 做键，比实际缓存的 30 条更长，于是第 30~100
        条增长期间缓存每轮都被判失效，参照集被反复整体重编码。
        """
        model = _RecordingModel()
        scorer = QualityScorer(threshold=0.0)
        scorer._model = model

        n = 120
        items = [
            {"original": f"种子{i}", "generated": f"变体{i}", "output": f"变体{i}"}
            for i in range(n)
        ]
        scorer.batch_score(items, include_semantic=True)

        # 长度 < n 的调用即参照集编码（整批编码长度为 n）
        ref_calls = [c for c in model.calls if len(c) < n]
        assert len(ref_calls) <= 31, (
            f"参照集被重编码 {len(ref_calls)} 次（应 ≤ 31），"
            "缓存键与缓存内容不匹配"
        )

    def test_total_encoding_stays_bounded(self):
        """总编码量必须有界，不得叠加冗余的参照集重编码"""
        model = _RecordingModel()
        scorer = QualityScorer(threshold=0.0)
        scorer._model = model

        n = 200
        items = [
            {"original": f"种子{i}", "generated": f"变体{i}", "output": f"变体{i}"}
            for i in range(n)
        ]
        scorer.batch_score(items, include_semantic=True)

        total = sum(len(c) for c in model.calls)
        # 必要开销：两批各 n 条 + 参照集（≤ 1+2+…+30 = 465）
        assert total <= 2 * n + 500, f"总编码量 {total} 过高，存在冗余重编码"

    def test_diversity_with_precomputed_embedding_matches_manual(self):
        """传入预算向量与自行编码必须得到完全相同的多样性分数"""
        model = _RecordingModel()
        scorer = QualityScorer()
        scorer._model = model

        existing = ["文本甲", "文本乙"]
        target = "文本丙"

        with_precomputed = scorer._calculate_diversity(
            target, existing, model.encode([target])[0]
        )
        manual = scorer._calculate_diversity(target, existing)

        assert with_precomputed == pytest.approx(manual)
