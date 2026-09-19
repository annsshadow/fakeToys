"""faiss 向量库（含 faiss 可用/缓存/删除重建）与 performance_benchmark 测试

faiss 未安装，通过向 sys.modules 注入伪 faiss 模块覆盖「faiss 可用」
分支；再覆盖 numpy 精确检索的查询缓存、删除后缓存过滤、持久化往返。
performance_benchmark 覆盖空阶段/指定阶段/计时相位/报告与内存峰值。
"""

import json
import sys
import types

import numpy as np
import pytest

from augmentor.vector.faiss import FAISSDB


class _FakeIndexFlatIP:
    """最小化的 IndexFlatIP 替身（numpy 内积检索）"""

    def __init__(self, dimension):
        self.dimension = dimension
        self._vectors = []

    def add(self, vectors):
        vectors = np.asarray(vectors, dtype=np.float32)
        if len(vectors) == 0:
            return
        if not self._vectors:
            self._vectors = [vectors]
        else:
            self._vectors.append(vectors)

    @property
    def _all(self):
        if not self._vectors:
            return np.zeros((0, self.dimension), dtype=np.float32)
        return np.vstack(self._vectors)

    def search(self, query, k):
        all_vecs = self._all
        if len(all_vecs) == 0:
            distances = np.zeros((1, 0), dtype=np.float32)
            indices = np.zeros((1, 0), dtype=np.int64)
            return distances, indices
        sims = all_vecs @ np.asarray(query[0], dtype=np.float32)
        top = np.argsort(-sims)[:k]
        indices = top.reshape(1, -1).astype(np.int64)
        distances = sims[top].reshape(1, -1).astype(np.float32)
        return distances, indices


def _install_fake_faiss(monkeypatch):
    """注入伪 faiss 模块使 FAISSDB 走 faiss 分支"""
    fake = types.ModuleType("faiss")
    fake.IndexFlatIP = _FakeIndexFlatIP
    monkeypatch.setitem(sys.modules, "faiss", fake)
    return fake


class TestFAISSAvailablePath:
    def test_backend_reports_faiss_when_available(self, monkeypatch):
        _install_fake_faiss(monkeypatch)
        db = FAISSDB(dimension=8)
        assert db.backend == "faiss"
        assert db._index is not None

    def test_faiss_search_ranks_correctly(self, monkeypatch):
        _install_fake_faiss(monkeypatch)
        db = FAISSDB(dimension=4)
        vecs = [
            np.array([1, 0, 0, 0], dtype=np.float32),
            np.array([0, 1, 0, 0], dtype=np.float32),
            np.array([0, 0, 1, 0], dtype=np.float32),
        ]
        db.add_vectors(vecs, [{"i": 0}, {"i": 1}, {"i": 2}], ids=["a", "b", "c"])
        results = db.search(np.array([1, 0, 0, 0], dtype=np.float32), top_k=2)
        assert results[0]["id"] == "a"
        assert results[0]["score"] > 0.9

    def test_faiss_delete_rebuilds_index(self, monkeypatch):
        _install_fake_faiss(monkeypatch)
        db = FAISSDB(dimension=4)
        db.add_vectors(
            [np.array([1, 0, 0, 0], dtype=np.float32),
             np.array([0, 1, 0, 0], dtype=np.float32)],
            [{"i": 0}, {"i": 1}], ids=["a", "b"]
        )
        removed = db.delete(["a"])
        assert removed == 1
        results = db.search(np.array([1, 0, 0, 0], dtype=np.float32), top_k=5)
        assert [r["id"] for r in results] == ["b"]


class TestFAISSCache:
    def test_query_cache_hit(self, monkeypatch):
        db = FAISSDB(dimension=4)
        db.add_vectors(
            [np.array([1, 0, 0, 0], dtype=np.float32),
             np.array([0, 1, 0, 0], dtype=np.float32)],
            [{"i": 0}, {"i": 1}], ids=["a", "b"]
        )
        q = np.array([1, 0, 0, 0], dtype=np.float32)
        first = db.search(q, top_k=2)
        second = db.search(q, top_k=2)
        assert first == second
        assert hasattr(db, "_query_cache")

    def test_cache_filtered_after_delete(self, monkeypatch):
        db = FAISSDB(dimension=4)
        db.add_vectors(
            [np.array([1, 0, 0, 0], dtype=np.float32),
             np.array([0, 1, 0, 0], dtype=np.float32)],
            [{"i": 0}, {"i": 1}], ids=["a", "b"]
        )
        q = np.array([1, 0, 0, 0], dtype=np.float32)
        db.search(q, top_k=2)  # 填充缓存
        db.delete(["a"])
        results = db.search(q, top_k=2)
        assert all(r["id"] != "a" for r in results)


class TestFAISSPersistence:
    def test_persist_and_load_roundtrip(self, tmp_path):
        db = FAISSDB(dimension=4, collection="c1", storage_dir=str(tmp_path))
        db.add_vectors(
            [np.array([1, 0, 0, 0], dtype=np.float32),
             np.array([0, 1, 0, 0], dtype=np.float32)],
            [{"i": 0}, {"i": 1}], ids=["a", "b"]
        )
        db.persist()

        db2 = FAISSDB(dimension=4, collection="c1", storage_dir=str(tmp_path))
        assert db2.load() is True
        assert db2._ids == ["a", "b"]

    def test_load_missing_returns_false(self, tmp_path):
        db = FAISSDB(dimension=4, collection="none", storage_dir=str(tmp_path))
        assert db.load() is False

    def test_persist_without_storage_dir_raises(self):
        db = FAISSDB(dimension=4)
        with pytest.raises(ValueError, match="storage_dir"):
            db.persist()

    def test_clear_resets_state(self, monkeypatch):
        _install_fake_faiss(monkeypatch)
        db = FAISSDB(dimension=4)
        db.add_vectors([np.array([1, 0, 0, 0], dtype=np.float32)], [{"i": 0}], ids=["a"])
        db.clear()
        assert db._ids == []
        assert db.search(np.array([1, 0, 0, 0], dtype=np.float32)) == []


class TestFAISSValidation:
    def test_dimension_mismatch_raises(self):
        db = FAISSDB(dimension=4)
        db.add_vectors([np.zeros(4, dtype=np.float32)], [{}], ids=["a"])
        with pytest.raises(ValueError, match="维度不匹配"):
            db.search(np.zeros(8, dtype=np.float32))

    def test_duplicate_ids_rejected(self):
        db = FAISSDB(dimension=4)
        v = np.zeros(4, dtype=np.float32)
        db.add_vectors([v], [{}], ids=["x"])
        with pytest.raises(ValueError, match="已存在"):
            db.add_vectors([v], [{}], ids=["x"])

    def test_add_ignores_empty(self):
        db = FAISSDB(dimension=4)
        assert db.add_vectors([], [], []) == []


class TestPerformanceBenchmark:
    def _new_bench(self):
        from augmentor.performance_benchmark import PerformanceBenchmark
        return PerformanceBenchmark("lbl")

    def test_end_stage_no_stages_noop(self):
        bench = self._new_bench()
        bench.end_stage()  # 空阶段不抛错
        assert bench.stages == []

    def test_end_stage_by_name_non_last(self):
        bench = self._new_bench()
        bench.start_stage("s1")
        bench.start_stage("s2")
        bench.end_stage("s1")  # 指定非末尾阶段 → 走查找循环
        s1 = next(s for s in bench.stages if s["stage"] == "s1")
        assert s1["duration_ms"] is not None

    def test_end_stage_mismatched_name_noop(self):
        bench = self._new_bench()
        bench.start_stage("s1")
        bench.end_stage("nonexistent")  # 无匹配 → 不修改
        assert bench.stages[-1]["end_time"] is None

    def test_run_timed_phase(self):
        bench = self._new_bench()
        result = bench.run_timed_phase("phase", lambda x, y: x + y, 2, 3)
        assert result == 5
        assert bench.stages[0]["duration_ms"] is not None

    def test_report_and_memory_peak(self):
        bench = self._new_bench()
        bench.start_stage("p")
        bench.end_stage("p")
        peak = bench.measure_memory_peak()
        assert peak >= 0.0
        report = bench.generate_benchmark_report()
        assert report["label"] == "lbl"
        assert report["stage_count"] == 1
        assert report["stages"][0]["stage"] == "p"
