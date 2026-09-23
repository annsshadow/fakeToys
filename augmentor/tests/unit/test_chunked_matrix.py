# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""向量矩阵分块处理测试 - 验证无 FAISS 时的 numpy 分块检索

原文件的 `test_search_uses_chunked_fallback_for_large_vectors` 用
`inspect.getsource(FAISSDB.search)` 断言源码里出现 "chunk" 字样——它无法因
业务逻辑变化而失败（把分块改成一次性全量计算，只要注释里还留着 "chunk"
就依然通过）。下面改为真正驱动 numpy 路径的行为测试。
"""

import numpy as np
import pytest

from augmentor.vector.faiss import FAISSDB

DIM = 8
BIG_N = 5000


@pytest.fixture
def sample_vectors():
    """创建测试向量"""
    return [
        np.random.rand(384).astype(np.float32) for _ in range(50)
    ]


def _db_without_faiss(n=BIG_N, collection="test_no_faiss"):
    """构造一个强制走 numpy 分块路径的库（不依赖本机是否装了 faiss）"""
    db = FAISSDB(dimension=DIM, collection=collection)
    rng = np.random.RandomState(7)
    vectors = rng.rand(n, DIM).astype(np.float32)
    db.add_vectors(list(vectors), [{"i": i} for i in range(n)],
                   ids=[f"v{i}" for i in range(n)])
    db._index = None
    return db


class TestChunkedMatrixCalculation:
    """分块矩阵计算测试"""

    def test_chunked_search_on_large_dataset(self, sample_vectors):
        """大数据集搜索应返回有效结果"""
        db = FAISSDB(dimension=384, collection="test_chunk")
        db.add_vectors(sample_vectors, [{"text": f"item{i}"} for i in range(len(sample_vectors))])

        query = np.random.rand(384).astype(np.float32)
        results = db.search(query, top_k=3)

        assert len(results) == 3
        assert all("id" in r for r in results)
        assert all("score" in r for r in results)

    def test_chunk_size_larger_than_data(self, sample_vectors):
        """数据量小于分块大小时应正常处理"""
        db = FAISSDB(dimension=384, collection="test_small")
        db.add_vectors(sample_vectors, [{"text": f"item{i}"} for i in range(len(sample_vectors))])

        query = np.random.rand(384).astype(np.float32)
        results = db.search(query, top_k=5)
        assert len(results) <= 5


class TestChunkedMemoryOptimization:
    """numpy 分块路径的行为测试"""

    def test_numpy_fallback_matches_bruteforce(self):
        """分块检索的结果必须与暴力全量计算完全一致"""
        db = _db_without_faiss()
        query = np.random.RandomState(11).rand(DIM).astype(np.float32)

        results = db.search(query, top_k=5)

        q = query / np.linalg.norm(query)
        expected = np.argsort(-(db._vectors @ q))[:5]
        assert [r["id"] for r in results] == [f"v{int(i)}" for i in expected]

    def test_numpy_fallback_processes_in_chunks(self, monkeypatch):
        """必须分块计算，而不是一次性对全量相似度排序"""
        db = _db_without_faiss()
        query = np.random.RandomState(12).rand(DIM).astype(np.float32)

        shapes = []
        real_argsort = np.argsort

        def spy(a, *args, **kwargs):
            shapes.append(np.asarray(a).shape)
            return real_argsort(a, *args, **kwargs)

        monkeypatch.setattr(np, "argsort", spy)
        db.search(query, top_k=3)

        assert len(shapes) > 1, "未分块：一次性对全量相似度做排序"
        chunk_size = max(1000, BIG_N // 4 + 1)
        assert all(shape[0] <= chunk_size for shape in shapes), \
            f"存在超过 chunk_size={chunk_size} 的整块排序: {shapes}"
