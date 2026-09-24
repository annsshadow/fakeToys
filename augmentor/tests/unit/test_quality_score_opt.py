# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""质量评分计算优化测试 - 批量评分缓存验证"""

import hashlib

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

    def test_embedding_cache_key_is_the_batch_itself_not_a_join_fingerprint(self):
        """批量编码缓存的键必须忠实于「被缓存的那份输入」

        原实现用 `md5("\\x00".join(texts))`：`["a", "b\\x00c"]` 与
        `["a\\x00b", "c"]` 拼出同一个串。等长时新批次静默拿到旧批次的向量
        （分数被答成别人的），变长时缓存向量与条数不匹配直接 IndexError。
        真实语料 13804 个字符串字段里 NUL 出现 0 次，所以这是**公共边界的潜在
        缺陷**（含 NUL 的输入可达：JSON 允许 `\\u0000`），不是当前语料上的活故障。
        """
        import numpy as np

        class _HashModel:
            def __init__(self):
                self.calls = 0

            def encode(self, texts, show_progress_bar=False, batch_size=32):
                self.calls += 1
                rows = []
                for t in texts:
                    digest = hashlib.sha256(t.encode("utf-8")).digest()
                    v = np.array([b / 255.0 for b in digest[:8]], dtype=np.float32)
                    rows.append(v / np.linalg.norm(v))
                return np.array(rows)

        nul = chr(0)
        batch_a = [
            {"original": "a", "generated": "生成甲", "output": "回答甲"},
            {"original": "b" + nul + "c", "generated": "生成乙", "output": "回答乙"},
        ]
        batch_b = [
            {"original": "a" + nul + "b", "generated": "生成甲", "output": "回答甲"},
            {"original": "c", "generated": "生成乙", "output": "回答乙"},
        ]
        assert nul.join(["a", "b" + nul + "c"]) == nul.join(["a" + nul + "b", "c"]), (
            "语料前提：两组 original 拼接后逐字相同，否则本用例测不到非单射"
        )

        scorer = QualityScorer(threshold=0.0)
        scorer._model = _HashModel()
        scorer._cross_encoder = "fallback"

        scorer.batch_score(batch_a)
        calls_after_a = scorer._model.calls
        scores_b = scorer.batch_score(batch_b)
        assert scorer._model.calls > calls_after_a, "batch_b 复用了 batch_a 的向量"

        fresh = QualityScorer(threshold=0.0)
        fresh._model = _HashModel()
        fresh._cross_encoder = "fallback"
        oracle_b = fresh.batch_score(batch_b)
        assert [round(s.semantic_similarity, 12) for s in scores_b] == [
            round(s.semantic_similarity, 12) for s in oracle_b
        ]

    def test_embedding_cache_key_survives_batch_length_change(self):
        """变长碰撞不得让缓存向量与批次条数错位（原实现抛 IndexError）"""
        import numpy as np

        class _OneModel:
            def encode(self, texts, show_progress_bar=False, batch_size=32):
                return np.ones((len(texts), 4), dtype=np.float32)

        nul = chr(0)
        scorer = QualityScorer(threshold=0.0)
        scorer._model = _OneModel()
        scorer._cross_encoder = "fallback"

        # 两条 "a"+NUL+"b" 拼出的串与单条 ... 相同，但条数不同
        scorer.batch_score([{"original": "a" + nul + "b", "generated": "生成甲", "output": "回答甲"}])
        scores = scorer.batch_score([
            {"original": "a", "generated": "生成甲", "output": "回答甲"},
            {"original": "b", "generated": "生成乙", "output": "回答乙"},
        ])
        assert len(scores) == 2
    
    def test_score_result_has_required_fields(self, scorer):
        """评分结果应包含所有必要字段"""
        result = scorer.score("问题", "生成问题", "回答")
        assert "semantic_similarity" in result.__dict__ or hasattr(result, 'semantic_similarity')
        assert "relevance" in result.__dict__ or hasattr(result, 'relevance')
        assert "diversity" in result.__dict__ or hasattr(result, 'diversity')
        assert "total_score" in result.__dict__ or hasattr(result, 'total_score')
        assert "passed" in result.__dict__ or hasattr(result, 'passed')
