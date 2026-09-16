"""ChromaDB 后端单元测试

通过向 sys.modules 注入假 chromadb 模块，在未安装 chromadb 的环境下
仍可验证包装层的全部行为（写入、检索、删除、清空、元数据清洗）。
"""

import importlib.util
import json
import sys
import types

import numpy as np
import pytest

from augmentor.vector.chromadb import ChromaDB, sanitize_metadata


class _FakeCollection:
    """内存版集合，使用真实余弦相似度（向量入库前已 L2 归一化）"""

    def __init__(self, name, metadata=None):
        self.name = name
        self.metadata = metadata
        self.store = {}

    def count(self):
        return len(self.store)

    def get(self, ids):
        return {"ids": [i for i in ids if i in self.store]}

    def add(self, ids, embeddings, metadatas):
        for vid, embedding, meta in zip(ids, embeddings, metadatas):
            self.store[vid] = (list(embedding), meta)

    def delete(self, ids):
        for vid in ids:
            self.store.pop(vid, None)

    def query(self, query_embeddings, n_results, include):
        query = query_embeddings[0]
        scored = []
        for vid, (embedding, meta) in self.store.items():
            similarity = float(sum(a * b for a, b in zip(query, embedding)))
            scored.append((similarity, vid, meta))
        scored.sort(key=lambda item: item[0], reverse=True)

        top = scored[:n_results]
        return {
            "ids": [[item[1] for item in top]],
            "metadatas": [[item[2] for item in top]],
            "distances": [[1.0 - item[0] for item in top]],
        }


class _FakeClient:
    def __init__(self, path=None):
        self.path = path
        self.collections = {}

    def get_or_create_collection(self, name, metadata=None):
        if name not in self.collections:
            self.collections[name] = _FakeCollection(name, metadata)
        return self.collections[name]

    def delete_collection(self, name):
        self.collections.pop(name, None)


class _FakeChromaModule(types.ModuleType):
    def __init__(self):
        super().__init__("chromadb")
        self.persistent_paths = []
        self.client_calls = 0

    def PersistentClient(self, path):
        self.persistent_paths.append(path)
        return _FakeClient(path)

    def Client(self):
        self.client_calls += 1
        return _FakeClient()


@pytest.fixture
def fake_chromadb(monkeypatch):
    module = _FakeChromaModule()
    monkeypatch.setitem(sys.modules, "chromadb", module)
    return module


def make_db(tmp_path, dimension=4, collection="test", **kwargs):
    return ChromaDB(
        dimension=dimension,
        collection=collection,
        storage_dir=str(tmp_path / "chroma"),
        **kwargs,
    )


class TestMissingDependency:
    def test_raises_import_error_when_chromadb_absent(self):
        if importlib.util.find_spec("chromadb") is not None:
            pytest.skip("chromadb 已安装，无法测试缺失降级路径")

        with pytest.raises(ImportError, match="pip install chromadb"):
            ChromaDB(dimension=4)

    def test_invalid_dimension_rejected(self, fake_chromadb):
        with pytest.raises(ValueError, match="向量维度必须为正整数"):
            ChromaDB(dimension=0)


class TestConstruction:
    def test_persistent_client_creates_storage_dir(self, tmp_path, fake_chromadb):
        make_db(tmp_path)
        assert (tmp_path / "chroma").is_dir()
        assert fake_chromadb.persistent_paths == [str(tmp_path / "chroma")]

    def test_non_persistent_uses_in_memory_client(self, tmp_path, fake_chromadb):
        ChromaDB(dimension=4, collection="mem", persist=False)
        assert fake_chromadb.client_calls == 1

    def test_persist_without_storage_dir_uses_in_memory_client(self, tmp_path, fake_chromadb):
        ChromaDB(dimension=4, collection="mem", storage_dir=None, persist=True)
        assert fake_chromadb.client_calls == 1

    def test_collection_uses_cosine_space(self, tmp_path, fake_chromadb):
        db = make_db(tmp_path)
        assert db._collection.metadata == {"hnsw:space": "cosine"}


class TestAddVectors:
    def test_add_returns_generated_ids(self, tmp_path, fake_chromadb):
        db = make_db(tmp_path)
        ids = db.add_vectors(
            [np.array([1, 0, 0, 0]), np.array([0, 1, 0, 0])],
            [{"text": "a"}, {"text": "b"}],
        )

        assert ids == ["test-0", "test-1"]
        assert db.count() == 2
        assert db._ids == ids

    def test_add_with_explicit_ids(self, tmp_path, fake_chromadb):
        db = make_db(tmp_path)
        ids = db.add_vectors([np.array([1, 0, 0, 0])], [{"text": "a"}], ids=["custom-1"])

        assert ids == ["custom-1"]
        assert db.get_metadata("custom-1") == {"text": "a"}

    def test_add_empty_returns_empty(self, tmp_path, fake_chromadb):
        db = make_db(tmp_path)
        assert db.add_vectors([], []) == []
        assert db.count() == 0

    def test_add_rejects_length_mismatch(self, tmp_path, fake_chromadb):
        db = make_db(tmp_path)
        with pytest.raises(ValueError, match="vectors 与 metadata 长度不一致"):
            db.add_vectors([np.array([1, 0, 0, 0])], [])

    def test_add_rejects_dimension_mismatch(self, tmp_path, fake_chromadb):
        db = make_db(tmp_path)
        with pytest.raises(ValueError, match="向量维度不匹配"):
            db.add_vectors([np.array([1, 0, 0])], [{"text": "a"}])

    def test_add_rejects_ids_length_mismatch(self, tmp_path, fake_chromadb):
        db = make_db(tmp_path)
        with pytest.raises(ValueError, match="ids 长度与 vectors 不一致"):
            db.add_vectors([np.array([1, 0, 0, 0])], [{"text": "a"}], ids=["x", "y"])

    def test_add_rejects_duplicate_ids(self, tmp_path, fake_chromadb):
        db = make_db(tmp_path)
        db.add_vectors([np.array([1, 0, 0, 0])], [{"text": "a"}], ids=["dup"])

        with pytest.raises(ValueError, match="ID 已存在"):
            db.add_vectors([np.array([0, 1, 0, 0])], [{"text": "b"}], ids=["dup"])

    def test_metadata_is_sanitized_before_write(self, tmp_path, fake_chromadb):
        db = make_db(tmp_path)
        db.add_vectors(
            [np.array([1, 0, 0, 0])],
            [{"nested": {"a": 1}, "none": None, "tags": ["x"]}],
            ids=["m1"],
        )

        stored = db._collection.store["m1"][1]
        assert stored["nested"] == json.dumps({"a": 1}, ensure_ascii=False)
        assert stored["tags"] == json.dumps(["x"], ensure_ascii=False)
        assert stored["none"] == ""


class TestSearch:
    def _populated_db(self, tmp_path):
        db = make_db(tmp_path)
        db.add_vectors(
            [
                np.array([1.0, 0.0, 0.0, 0.0]),
                np.array([0.0, 1.0, 0.0, 0.0]),
                np.array([0.70710678, 0.70710678, 0.0, 0.0]),
            ],
            [{"text": "a"}, {"text": "b"}, {"text": "c"}],
            ids=["a", "b", "c"],
        )
        return db

    def test_search_on_empty_collection_returns_empty(self, tmp_path, fake_chromadb):
        assert make_db(tmp_path).search(np.array([1, 0, 0, 0])) == []

    def test_search_returns_similarity_desc(self, tmp_path, fake_chromadb):
        db = self._populated_db(tmp_path)
        results = db.search(np.array([1.0, 0.0, 0.0, 0.0]), top_k=2)

        assert [r["id"] for r in results] == ["a", "c"]
        assert results[0]["score"] == pytest.approx(1.0)
        assert results[1]["score"] == pytest.approx(0.70710678, abs=1e-5)
        assert results[0]["metadata"] == {"text": "a"}

    def test_search_caps_n_results_by_collection_size(self, tmp_path, fake_chromadb):
        db = self._populated_db(tmp_path)
        assert len(db.search(np.array([1.0, 0.0, 0.0, 0.0]), top_k=100)) == 3

    def test_search_rejects_dimension_mismatch(self, tmp_path, fake_chromadb):
        db = self._populated_db(tmp_path)
        with pytest.raises(ValueError, match="查询向量维度不匹配"):
            db.search(np.array([1.0, 0.0, 0.0]))

    def test_search_accepts_two_dimensional_query(self, tmp_path, fake_chromadb):
        db = self._populated_db(tmp_path)
        results = db.search(np.array([[1.0, 0.0, 0.0, 0.0]]), top_k=1)
        assert results[0]["id"] == "a"


class TestDeleteAndClear:
    def test_delete_returns_removed_count(self, tmp_path, fake_chromadb):
        db = make_db(tmp_path)
        db.add_vectors(
            [np.array([1, 0, 0, 0]), np.array([0, 1, 0, 0])],
            [{"text": "a"}, {"text": "b"}],
            ids=["a", "b"],
        )

        assert db.delete(["a"]) == 1
        assert db.count() == 1
        assert db._ids == ["b"]
        assert db._metadata == [{"text": "b"}]

    def test_delete_empty_list_returns_zero(self, tmp_path, fake_chromadb):
        db = make_db(tmp_path)
        assert db.delete([]) == 0

    def test_delete_unknown_ids_returns_zero(self, tmp_path, fake_chromadb):
        db = make_db(tmp_path)
        db.add_vectors([np.array([1, 0, 0, 0])], [{"text": "a"}], ids=["a"])
        assert db.delete(["nope"]) == 0
        assert db.count() == 1

    def test_clear_resets_collection(self, tmp_path, fake_chromadb):
        db = make_db(tmp_path)
        db.add_vectors([np.array([1, 0, 0, 0])], [{"text": "a"}], ids=["a"])

        db.clear()

        assert db.count() == 0
        assert db._ids == []
        assert db._metadata == []
        assert db._collection.metadata == {"hnsw:space": "cosine"}


class TestSanitizeMetadata:
    def test_scalars_are_preserved(self):
        result = sanitize_metadata({"s": "x", "i": 1, "f": 0.5, "b": True})
        assert result == {"s": "x", "i": 1, "f": 0.5, "b": True}

    def test_none_becomes_empty_string(self):
        assert sanitize_metadata({"x": None}) == {"x": ""}

    def test_nested_structures_become_json(self):
        result = sanitize_metadata({"d": {"a": 1}, "l": [1, 2]})
        assert result["d"] == '{"a": 1}'
        assert result["l"] == "[1, 2]"

    def test_empty_or_none_input(self):
        assert sanitize_metadata({}) == {}
        assert sanitize_metadata(None) == {}
