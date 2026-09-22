# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

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


class TestChromaDBExtended:
    """ChromaDB 后端扩展测试（覆盖剩余分支）"""

    def test_add_rejects_duplicate_ids(self, tmp_path, fake_chromadb):
        """重复 ID 必须被拒绝，否则删除语义会歧义"""
        db = make_db(tmp_path)
        db.add_vectors([np.array([1, 0, 0, 0])], [{"text": "a"}], ids=["dup"])
        with pytest.raises(ValueError, match="ID 已存在"):
            db.add_vectors([np.array([0, 1, 0, 0])], [{"text": "b"}], ids=["dup"])

    def test_add_rejects_ids_length_mismatch(self, tmp_path, fake_chromadb):
        """ids 数量与 vectors 不一致应报错"""
        db = make_db(tmp_path)
        with pytest.raises(ValueError, match="ids 长度"):
            db.add_vectors(
                [np.array([1, 0, 0, 0]), np.array([0, 1, 0, 0])],
                [{"text": "a"}, {"text": "b"}],
                ids=["only-one"],
            )

    def test_search_on_empty_returns_empty(self, tmp_path, fake_chromadb):
        """空集合检索应返回空列表"""
        db = make_db(tmp_path)
        assert db.search(np.array([1, 0, 0, 0])) == []

    def test_search_dimension_mismatch_raises(self, tmp_path, fake_chromadb):
        """维度不匹配必须报错"""
        db = make_db(tmp_path)
        db.add_vectors([np.array([1, 0, 0, 0])], [{"text": "a"}])
        with pytest.raises(ValueError, match="维度不匹配"):
            db.search(np.array([1, 0, 0]))

    def test_search_2d_query_matrix(self, tmp_path, fake_chromadb):
        """批量查询（2D 矩阵）应能正常工作"""
        db = make_db(tmp_path)
        db.add_vectors(
            [np.array([1, 0, 0, 0]), np.array([0, 1, 0, 0])],
            [{"i": 0}, {"i": 1}],
        )
        query = np.array([[1, 0, 0, 0], [0, 1, 0, 0]], dtype=np.float32)
        results = db.search(query, top_k=1)
        # 实现取首个查询向量的结果列表
        assert len(results) >= 1
        # 余弦相似度首位应命中自身
        assert results[0]["score"] > 0.9

    def test_search_top_k_capped_by_count(self, tmp_path, fake_chromadb):
        """top_k 超过集合大小应收敛"""
        db = make_db(tmp_path)
        db.add_vectors([np.array([1, 0, 0, 0])], [{"i": 0}])
        results = db.search(np.array([1, 0, 0, 0]), top_k=100)
        assert len(results) == 1

    def test_search_score_is_similarity(self, tmp_path, fake_chromadb):
        """距离转相似度后分数应在 0-1 区间"""
        db = make_db(tmp_path)
        db.add_vectors([np.array([1, 0, 0, 0])], [{"i": 0}])
        result = db.search(np.array([1, 0, 0, 0]))[0]
        assert 0.0 <= result["score"] <= 1.0
        assert "metadata" in result

    def test_generated_id_format(self, tmp_path, fake_chromadb):
        """自动生成 ID 应使用 集合名-N 格式"""
        db = make_db(tmp_path)
        ids = db.add_vectors([np.array([1, 0, 0, 0])], [{"i": 0}])
        assert ids == ["test-0"]

    def test_get_metadata_nonexistent(self, tmp_path, fake_chromadb):
        """获取不存在的 ID 元数据应返回 None"""
        db = make_db(tmp_path)
        assert db.get_metadata("ghost") is None

    def test_count_tracks_adds_and_deletes(self, tmp_path, fake_chromadb):
        """count 应随增删正确变化"""
        db = make_db(tmp_path)
        db.add_vectors([np.array([1, 0, 0, 0])], [{"i": 0}])
        db.add_vectors([np.array([0, 1, 0, 0])], [{"i": 1}])
        assert db.count() == 2
        db.delete(["test-0"])
        assert db.count() == 1

    def test_clear_then_add_fresh(self, tmp_path, fake_chromadb):
        """清空后重新添加应从零计数"""
        db = make_db(tmp_path)
        db.add_vectors([np.array([1, 0, 0, 0])], [{"i": 0}])
        db.clear()
        ids = db.add_vectors([np.array([1, 0, 0, 0])], [{"i": 1}])
        assert db.count() == 1
        assert ids == ["test-0"]


class TestSanitizeMetadataExtended:
    """sanitize_metadata 扩展测试"""

    def test_mixed_nested_and_scalar(self):
        result = sanitize_metadata({
            "a": 1,
            "b": {"nested": [1, 2]},
            "c": None,
            "d": True,
        })
        assert result["a"] == 1
        assert result["b"] == '{"nested": [1, 2]}'
        assert result["c"] == ""
        assert result["d"] is True

    def test_unicode_json_serialization(self):
        """嵌套中文结构应序列化为 JSON 字符串"""
        result = sanitize_metadata({"name": {"城市": "北京"}})
        assert result["name"] == '{"城市": "北京"}'
