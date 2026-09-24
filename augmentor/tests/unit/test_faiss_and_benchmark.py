# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

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


class TestIncrementalAdditions:
    """逐条写入不得每次重抄整份矩阵，也不得每次重建 ID 集合

    `add_vectors` 原本每次都 `np.vstack` 全量矩阵、每次都 `set(self._ids)` 重建镜像，
    「一条一批」的写入因此是 O(n²)。真实规模实测（同进程新旧交替，dimension=384）：
    逐条写入 750/1500/3000 条 **58.2 / 386.1 / 1671.3 ms**（输入翻倍 → 时间 ×6.6、×4.3
    即平方）→ **6.5 / 13.1 / 31.7 ms**（×2.0、×2.4 即线性，8.94×/29.55×/52.72×，
    全部轮次都更快）；重复 ID 探测 200 次 **7.8 → 1.5 ms**。预言机用**复制次数与
    复制行数**，不用计时。
    """

    @staticmethod
    def _vectors(n, dimension=8, seed=11):
        rng = np.random.default_rng(seed)
        return rng.normal(size=(n, dimension)).astype(np.float32)

    def test_incremental_add_never_recopies_the_matrix(self, monkeypatch):
        """逐条 add 一次都不该出现「整份矩阵重抄」

        探针挂在 `numpy.vstack` 上：两版都存在的名字，所以缺陷态它同样能数到。
        断言之后还要核对行数与内容，免得「零次」是靠悄悄丢写入换来的。
        """
        calls = []
        real = np.vstack

        def spy(*args, **kwargs):
            calls.append(args)
            return real(*args, **kwargs)

        monkeypatch.setattr(np, "vstack", spy)
        vecs = self._vectors(30)
        db = FAISSDB(dimension=8)
        for k in range(30):
            db.add_vectors([vecs[k]], [{"i": k}], ["v%d" % k])

        assert calls == [], f"逐条写入期间整份矩阵被重抄了 {len(calls)} 次"
        assert db.count() == 30
        assert np.allclose(db._vectors, _normalize(vecs))

    def test_growth_copies_are_amortized(self, monkeypatch):
        """400 次逐条写入的复制总量必须是 O(n)：几何扩容只发生 log 次"""
        grows = []
        real = FAISSDB._grow

        def spy(self, rows_needed):
            grows.append((self._size, rows_needed))
            return real(self, rows_needed)

        monkeypatch.setattr(FAISSDB, "_grow", spy)
        vecs = self._vectors(400)
        db = FAISSDB(dimension=8)
        for k in range(400):
            db.add_vectors([vecs[k]], [{"i": k}], ["v%d" % k])

        copied_rows = sum(size for size, _ in grows)
        assert len(grows) <= 12, f"扩容 {len(grows)} 次，没做到几何增长"
        assert copied_rows <= 2 * 400, f"复制了 {copied_rows} 行，超过 2n 的摊销上界"
        assert db.count() == 400

    def test_incremental_and_batch_agree_row_for_row(self):
        """逐条写与一次写满必须给出同一份行对齐结果（写入方式不该影响检索）"""
        vecs = self._vectors(100, seed=23)
        each = FAISSDB(dimension=8)
        for k in range(100):
            each.add_vectors([vecs[k]], [{"i": k}], ["v%d" % k])
        batch = FAISSDB(dimension=8)
        batch.add_vectors(list(vecs), [{"i": k} for k in range(100)],
                          ["v%d" % k for k in range(100)])

        assert each._ids == batch._ids
        assert np.array_equal(each._vectors, batch._vectors)
        query = vecs[3]
        assert [(r["id"], r["score"]) for r in each.search(query, top_k=10)] == \
               [(r["id"], r["score"]) for r in batch.search(query, top_k=10)]

    def test_spare_capacity_does_not_leak_into_persistence(self, tmp_path):
        """落盘的必须只有有效行：几何扩容留下的空槽不属于数据

        这条护的是新设计自己引入的风险（写缓冲比有效行更长），缺陷态同样满足它。
        """
        vecs = self._vectors(300)
        db = FAISSDB(dimension=8, collection="geo", storage_dir=str(tmp_path))
        for k in range(300):
            db.add_vectors([vecs[k]], [{"i": k}], ["v%d" % k])
        assert db._buffer.shape[0] > 300  # 确实攒下了空槽
        db.persist()

        fresh = FAISSDB(dimension=8, collection="geo", storage_dir=str(tmp_path))
        assert fresh.load() is True
        assert fresh._vectors.shape == (300, 8)
        assert np.array_equal(fresh._vectors, _normalize(vecs))

    def test_duplicate_detection_survives_growth_and_delete(self):
        """镜像 ID 集合必须跟着增删走：漏检会让同一 ID 占两行"""
        vecs = self._vectors(3)
        db = FAISSDB(dimension=8)
        for k in range(3):
            db.add_vectors([vecs[k]], [{"i": k}], ["v%d" % k])
        with pytest.raises(ValueError, match="已存在"):
            db.add_vectors([vecs[0]], [{"i": 0}], ["v0"])

        assert db.delete(["v1"]) == 1
        assert db.add_vectors([vecs[1]], [{"i": 1}], ["v1"]) == ["v1"]
        assert db._id_set == set(db._ids)
        with pytest.raises(ValueError, match="已存在"):
            db.add_vectors([vecs[2]], [{"i": 2}], ["v2"])

    def test_id_set_rebuilds_when_ids_are_replaced_behind_its_back(self):
        """绕过写接口换掉 `_ids` 时，长度判据要能发现并整份重建"""
        vecs = self._vectors(2)
        db = FAISSDB(dimension=8)
        db.add_vectors(list(vecs), [{"i": 0}, {"i": 1}], ["v0", "v1"])

        db._ids = ["z"]
        db.add_vectors([vecs[0]], [{"i": 9}], ["v9"])
        assert db._id_set == {"z", "v9"}
        with pytest.raises(ValueError, match="已存在"):
            db.add_vectors([vecs[1]], [{"i": 8}], ["z"])

    def test_rows_stay_aligned_after_deleting_from_incremental_writes(self):
        """删除后剩余行仍与 `_ids` 逐行对齐，检索返回的元数据不能串位"""
        vecs = self._vectors(40, seed=31)
        db = FAISSDB(dimension=8)
        for k in range(40):
            db.add_vectors([vecs[k]], [{"i": k}], ["v%d" % k])
        db.delete(["v%d" % k for k in range(0, 40, 2)])

        assert db.count() == 20
        assert db._vectors.shape == (20, 8)
        rows = db.search(vecs[3], top_k=1)
        assert rows[0]["id"] == "v3"
        assert rows[0]["metadata"] == {"i": 3}


def _normalize(vectors):
    from augmentor.vector.base import normalize_vectors
    return normalize_vectors(np.array(vectors, dtype=np.float32))


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
