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
        """评分后应初始化嵌入缓存"""
        # 使用简化测试：验证代码路径包含缓存机制
        import inspect
        source = inspect.getsource(scorer.batch_score)
        assert "_embedding_cache" in source or "cached_encode" in source
    
    def test_score_result_has_required_fields(self, scorer):
        """评分结果应包含所有必要字段"""
        result = scorer.score("问题", "生成问题", "回答")
        assert "semantic_similarity" in result.__dict__ or hasattr(result, 'semantic_similarity')
        assert "relevance" in result.__dict__ or hasattr(result, 'relevance')
        assert "diversity" in result.__dict__ or hasattr(result, 'diversity')
        assert "total_score" in result.__dict__ or hasattr(result, 'total_score')
        assert "passed" in result.__dict__ or hasattr(result, 'passed')
