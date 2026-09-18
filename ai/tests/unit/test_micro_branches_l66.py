"""微型分支收尾 L66：vector 族 + auto_config/auto_test 残留分支

- vector 工厂: chromadb 后端创建
- normalize: 一维输入自动 reshape
- chromadb: 依赖缺失时的 ImportError 提示
- faiss: 无效索引 -1 跳过、load 重建 faiss 索引、一维向量 reshape
- auto_config: 中等长度不均衡的采样比例中间档
- auto_test: TestCase.to_dict
"""

import json
import sys
from pathlib import Path
from types import SimpleNamespace

import numpy as np
import pytest

from augmentor.auto_config import AutoConfig
from augmentor.auto_test import TestCase
from augmentor.vector import create_vector_db
from augmentor.vector.base import normalize_vectors
from augmentor.vector.chromadb import ChromaDB
from augmentor.vector.faiss import FAISSDB


class TestVectorFactory:
    def test_create_chromadb_backend(self):
        db = create_vector_db(backend="chromadb", dimension=4)
        assert db is not None
        assert db.dimension == 4


class TestNormalize1D:
    def test_1d_input_reshaped(self):
        out = normalize_vectors(np.array([3.0, 4.0]))
        assert out.shape == (1, 2)
        assert float(np.linalg.norm(out[0])) == pytest.approx(1.0)


class TestChromaImportError:
    def test_missing_dependency_raises(self, monkeypatch):
        monkeypatch.setitem(sys.modules, "chromadb", None)
        with pytest.raises(ImportError, match="chromadb"):
            ChromaDB(dimension=4)


def _mk_faiss_db(storage: str | None = None, collection: str = "default") -> FAISSDB:
    db = FAISSDB(dimension=3, collection=collection, storage_dir=storage)
    db.add_vectors(
        [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0]],
        metadata=[{"x": 1}, {}],
        ids=["a", "b"],
    )
    return db


class TestFaissBranches:
    def test_search_skips_invalid_indices(self):
        db = _mk_faiss_db()
        fake_index = SimpleNamespace(
            search=lambda q, k: (
                np.array([[0.9, 0.1]]),
                np.array([[0, -1]]),
            )
        )
        db._index = fake_index
        results = db.search(np.array([1.0, 0.0, 0.0]), top_k=2)
        assert [r["id"] for r in results] == ["a"]

    def test_load_rebuilds_faiss_index(self, tmp_path):
        storage = tmp_path / "vecs"
        src = _mk_faiss_db(str(storage), collection="default")
        src.persist()

        class _FakeIndex:
            def __init__(self, dim):
                self.added = []

            def add(self, vecs):
                self.added.append(vecs)

        class _FakeFaiss:
            @staticmethod
            def IndexFlatIP(dim):
                return _FakeIndex(dim)

        loaded = FAISSDB(dimension=3, storage_dir=str(storage))
        loaded._faiss = _FakeFaiss
        assert loaded.load() is True
        assert isinstance(loaded._index, _FakeIndex)
        assert len(loaded._index.added) == 1

    def test_load_reshapes_1d_vectors(self, tmp_path):
        storage = tmp_path / "v1d"
        storage.mkdir()
        np.save(storage / "c_vectors.npy", np.array([1.0, 0.0, 0.0], dtype=np.float32))
        (storage / "c_meta.json").write_text(
            json.dumps({"ids": ["a"], "metadata": [{}], "dimension": 3}),
            encoding="utf-8",
        )
        loaded = FAISSDB(dimension=3, collection="c", storage_dir=str(storage))
        assert loaded.load() is True
        assert loaded._vectors.shape == (1, 3)


class TestAutoConfigMidSpread:
    def test_sample_ratio_middle_tier(self):
        cfg = AutoConfig()
        lo, hi = cfg.sample_ratio_range
        # spread = 50 / 10 = 5 → 中间档 ratio=(lo+hi)/2
        n = cfg._recommend_sample_size(
            {"length_stats": {"avg": 10, "max": 50}}, 1000
        )
        assert n == int(1000 * (lo + hi) / 2)


class TestAutoTestDict:
    def test_test_case_to_dict(self):
        case = TestCase(
            test_id="t1",
            name="case",
            description="d",
            test_func=lambda: 1,
            expected_result=1,
            tags=["unit"],
        )
        d = case.to_dict()
        assert d["test_id"] == "t1"
        assert d["tags"] == ["unit"]
