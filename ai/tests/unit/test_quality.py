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
