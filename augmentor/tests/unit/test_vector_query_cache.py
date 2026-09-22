# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""向量数据库查询优化测试 - 查询缓存复用验证"""

import pytest
import numpy as np
from augmentor.vector.faiss import FAISSDB


@pytest.fixture
def small_db():
    db = FAISSDB(dimension=384, collection="test_cache")
    db.add_vectors(
        [np.random.rand(384).astype(np.float32) for _ in range(10)],
        [{"text": f"item{i}"} for i in range(10)]
    )
    return db


class TestVectorQueryCache:
    """向量查询缓存优化测试"""
    
    def test_query_cache_exists_after_search(self, small_db):
        """搜索后应初始化查询缓存"""
        query = np.random.rand(384).astype(np.float32)
        small_db.search(query, top_k=3)
        assert hasattr(small_db, '_query_cache')
        assert isinstance(small_db._query_cache, dict)
    
    def test_cache_stores_search_results(self, small_db):
        """相同查询应产生缓存条目"""
        query = np.random.rand(384).astype(np.float32)
        small_db.search(query, top_k=3)
        
        # 由于使用随机向量，缓存条目应存在
        assert len(small_db._query_cache) >= 1
    
    def test_search_returns_valid_structure_with_cache(self, small_db):
        """带缓存的搜索应返回有效结果结构"""
        query = np.random.rand(384).astype(np.float32)
        results = small_db.search(query, top_k=2)
        
        assert isinstance(results, list)
        for r in results:
            assert "id" in r
            assert "score" in r
            assert "metadata" in r
