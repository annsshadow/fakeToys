"""质量评分单元测试

质量分决定样本是否进入训练集，因此权重校验、阈值过滤与报告统计必须准确。
"""

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
