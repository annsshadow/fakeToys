# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""向量数据库查询缓存测试

原文件只断言 `hasattr(db, '_query_cache')` 和 `isinstance(..., dict)`——
它测的是「有个 dict 叫这个名字」，不是「缓存行为正确」。下面换成行为断言，
覆盖原实现真实存在的三个缺陷：

1. 缓存键只有查询向量、不含 top_k → 先 top_k=2 再 top_k=6 会返回被截断的 2 条
2. add_vectors / delete / clear / load 后不失效缓存 → 新向量永远查不出来
3. 缓存无容量上限 → 每个新查询都永久驻留内存
"""

import numpy as np
import pytest

from augmentor.vector.faiss import FAISSDB

DIM = 8


def _db(n=10, collection="test_cache"):
    db = FAISSDB(dimension=DIM, collection=collection)
    db.add_vectors(
        [np.random.rand(DIM).astype(np.float32) for _ in range(n)],
        [{"text": f"item{i}"} for i in range(n)],
        ids=[f"v{i}" for i in range(n)],
    )
    return db


def _query(seed=0):
    rng = np.random.RandomState(seed)
    return rng.rand(DIM).astype(np.float32)


class TestCacheKeyCoversTopK:
    """缺陷 1：缓存键必须覆盖 top_k"""

    def test_larger_top_k_after_smaller_is_not_truncated(self):
        """先 top_k=2 再 top_k=6，第二次必须真的返回 6 条

        原实现用查询向量做唯一键，第二次直接命中那条 2 条的缓存，
        而 `valid_results[:k]` 对不足 k 条的结果无能为力。
        """
        db = _db(n=10)
        q = _query()

        assert len(db.search(q, top_k=2)) == 2
        assert len(db.search(q, top_k=6)) == 6

    def test_smaller_top_k_after_larger_is_truncated(self):
        """先 top_k=6 再 top_k=2，第二次必须只返回 2 条"""
        db = _db(n=10)
        q = _query()

        assert len(db.search(q, top_k=6)) == 6
        assert len(db.search(q, top_k=2)) == 2

    def test_same_query_and_top_k_hits_cache(self):
        """键相同才应命中：同 query、同 top_k 的结果应完全一致"""
        db = _db(n=10)
        q = _query()

        first = db.search(q, top_k=3)
        second = db.search(q, top_k=3)

        assert first == second
        assert db._query_cache.stats["hits"] >= 1, "相同查询没有走缓存"


class TestCacheInvalidationOnWrite:
    """缺陷 2：写操作必须失效缓存"""

    def test_add_vectors_makes_new_vector_searchable(self):
        """写入后必须能查到新向量

        缓存的是「某个索引状态下的 top-k」。新向量与已有向量得分相同
        （都用查询本身作为向量），只要缓存未失效，它就永远不出现在结果里。
        """
        db = _db(n=3)
        q = _query()

        assert "new" not in {r["id"] for r in db.search(q, top_k=3)}

        db.add_vectors([q], [{"text": "new"}], ids=["new"])

        ids = [r["id"] for r in db.search(q, top_k=3)]
        assert "new" in ids, "新增向量后仍命中旧缓存，新向量查不出来"

    def test_delete_is_reflected_in_results(self):
        """删除后结果中不得再出现被删 ID"""
        db = _db(n=5)
        q = _query()

        db.search(q, top_k=5)          # 填充缓存
        db.delete(["v0"])

        ids = {r["id"] for r in db.search(q, top_k=5)}
        assert "v0" not in ids

    def test_clear_is_reflected_in_results(self):
        """清空后重新写入的数据必须能查到"""
        db = _db(n=5)
        q = _query()

        db.search(q, top_k=3)
        db.clear()
        assert db.search(q, top_k=3) == []

        db.add_vectors([q], [{"text": "after-clear"}], ids=["after-clear"])
        ids = [r["id"] for r in db.search(q, top_k=3)]
        assert ids == ["after-clear"]

    def test_load_replaces_cache(self, tmp_path):
        """load 覆盖整个索引后，旧缓存不得继续生效"""
        db = _db(n=4, collection="c_load")
        q = _query()
        db.storage_dir = tmp_path
        db.persist()

        other = FAISSDB(dimension=DIM, collection="c_load", storage_dir=str(tmp_path))
        # 先让 other 有内容且缓存已填充（top_k=1 使 k 在 load 前后同为 1，
        # 从而隔离出「缓存未失效」而不是「键变了」）
        other.add_vectors([_query(99)], [{"text": "stale"}], ids=["stale"])
        assert [r["id"] for r in other.search(q, top_k=1)] == ["stale"]

        assert other.load() is True

        ids = [r["id"] for r in other.search(q, top_k=1)]
        assert ids[0] in {"v0", "v1", "v2", "v3"}, "load 后仍命中旧索引的缓存"


class TestCacheIsBounded:
    """缺陷 3：缓存必须有容量上限"""

    def test_default_cache_limit_is_finite(self):
        """默认上限必须是一个有限且合理的值"""
        assert 0 < FAISSDB._QUERY_CACHE_MAX <= 10_000

    def test_cache_size_is_bounded(self, monkeypatch):
        """缓存条目数不得随查询次数增长

        上限通过 monkeypatch 固定为 20，断言也写死 20——若用
        `db._QUERY_CACHE_MAX` 做断言，把上限调大就能让测试空转通过。
        """
        monkeypatch.setattr(FAISSDB, "_QUERY_CACHE_MAX", 20)
        db = _db(n=3)
        rng = np.random.RandomState(42)

        for _ in range(100):
            db.search(rng.rand(DIM).astype(np.float32), top_k=2)

        assert len(db._query_cache) <= 20, "缓存未按上限淘汰，条目数随查询次数增长"

    def test_distinct_queries_are_cached_separately(self):
        db = _db(n=3)

        db.search(_query(1), top_k=2)
        db.search(_query(2), top_k=2)

        assert len(db._query_cache) == 2
        assert db._query_cache.stats["misses"] == 2


class TestResultStructure:
    """结果结构契约"""

    def test_search_returns_valid_structure_with_cache(self):
        db = _db(n=5)
        q = _query()

        for _ in range(2):  # 第一次未命中，第二次命中缓存
            results = db.search(q, top_k=2)
            assert isinstance(results, list)
            assert len(results) == 2
            for r in results:
                assert set(r) == {"id", "score", "metadata"}
                assert isinstance(r["score"], float)

    def test_cache_hit_returns_metadata_of_current_index(self):
        """命中缓存返回的元数据必须来自当前索引，而不是缓存里的旧引用"""
        db = _db(n=3)
        q = db._vectors[0]        # 用第 0 条向量做查询，保证 top-1 就是 v0

        assert db.search(q, top_k=1)[0]["id"] == "v0"
        db._metadata[0] = {"text": "被改过"}

        results = db.search(q, top_k=1)
        assert results[0]["id"] == "v0"
        assert results[0]["metadata"] == {"text": "被改过"}


class TestDimensionValidation:
    def test_wrong_dimension_raises_before_caching(self):
        from augmentor.exceptions import VectorError

        db = _db(n=3)
        with pytest.raises(VectorError):
            db.search(np.zeros(DIM + 1, dtype=np.float32), top_k=2)
        assert len(db._query_cache) == 0

    def test_empty_db_returns_empty_and_caches_nothing(self):
        db = FAISSDB(dimension=DIM)
        assert db.search(_query(), top_k=3) == []
        assert len(db._query_cache) == 0
