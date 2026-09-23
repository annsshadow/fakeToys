# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""质量评分计算优化测试 - 批量评分缓存验证"""

import pytest
from unittest.mock import MagicMock
from augmentor.quality import QualityScorer


@pytest.fixture
def scorer():
    return QualityScorer(threshold=0.6)


class TestQualityScoreOptimization:
    """质量评分优化测试"""
    
    def test_batch_score_returns_list(self, scorer):
        """批量评分应返回评分列表"""
        items = [
            {"original": "如何租房？", "generated": "如何租房？", "output": "建议查看房源"},
            {"original": "租金多少？", "generated": "租金多少？", "output": "根据房型不同"},
        ]
        scores = scorer.batch_score(items)
        assert isinstance(scores, list)
        assert len(scores) == len(items)
    
    def test_embedding_cache_exists_after_score(self, scorer):
        """同一批数据重复评分不得重复整批编码（缓存必须真正命中）

        原实现用 `inspect.getsource` 检查源码里是否出现 "_embedding_cache"
        字样——无论缓存是否真的生效都会通过，无法在业务逻辑变化时失败。
        """
        import numpy as np

        class _RecordingModel:
            def __init__(self):
                self.sizes = []

            def encode(self, texts, show_progress_bar=False, batch_size=32):
                self.sizes.append(len(texts))
                return np.ones((len(texts), 4), dtype=np.float32)

        model = _RecordingModel()
        scorer._model = model
        scorer._cross_encoder = "fallback"

        items = [
            {"original": f"问题{i}", "generated": f"生成{i}", "output": f"回答{i}"}
            for i in range(4)
        ]

        scorer.batch_score(items, include_semantic=True)
        assert 4 in model.sizes, "首轮应对整批做一次编码"

        model.sizes.clear()
        scorer.batch_score(items, include_semantic=True)
        assert 4 not in model.sizes, "第二轮仍整批编码，嵌入缓存未命中"
    
    def test_score_result_has_required_fields(self, scorer):
        """评分结果应包含所有必要字段"""
        result = scorer.score("问题", "生成问题", "回答")
        assert "semantic_similarity" in result.__dict__ or hasattr(result, 'semantic_similarity')
        assert "relevance" in result.__dict__ or hasattr(result, 'relevance')
        assert "diversity" in result.__dict__ or hasattr(result, 'diversity')
        assert "total_score" in result.__dict__ or hasattr(result, 'total_score')
        assert "passed" in result.__dict__ or hasattr(result, 'passed')
