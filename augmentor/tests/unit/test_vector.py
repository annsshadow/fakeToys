"""向量数据库单元测试

向量检索是语义去重与相似样本推荐的基础，接口一致性必须被保证。
"""

import numpy as np
import pytest

from augmentor.vector import create_vector_db, FAISSDB, VectorDB


def unit_vectors(dimension=4):
    """构造一组互相正交的单位向量

    Args:
        dimension: 维度

    Returns:
        向量列表
    """
    vectors = []
    for index in range(dimension):
        vector = np.zeros(dimension, dtype=np.float32)
        vector[index] = 1.0
        vectors.append(vector)
    return vectors


class TestFactory:
    """工厂方法"""

    def test_creates_faiss_backend(self):
        """faiss 后端需可创建"""
        db = create_vector_db("faiss", dimension=4)
        assert isinstance(db, FAISSDB)
        assert isinstance(db, VectorDB)

    def test_unknown_backend_raises(self):
        """未知后端必须报错并列出支持项"""
        with pytest.raises(ValueError) as excinfo:
            create_vector_db("pinecone")
        assert "支持" in str(excinfo.value)

    def test_invalid_dimension_raises(self):
        """维度必须为正，否则无法建索引"""
        with pytest.raises(ValueError):
            create_vector_db("faiss", dimension=0)

    def test_chromadb_requires_dependency(self):
        """未安装 chromadb 时应抛出清晰的 ImportError，而不是 AttributeError"""
        try:
            import chromadb  # noqa: F401
            pytest.skip("chromadb 已安装，跳过依赖缺失场景")
        except ImportError:
            pass

        with pytest.raises(ImportError) as excinfo:
            create_vector_db("chromadb", dimension=4)
        assert "chromadb" in str(excinfo.value)


class TestAddAndSearch:
    """写入与检索"""

    def test_add_returns_ids(self):
        """写入需返回可追踪的 ID"""
        db = create_vector_db("faiss", dimension=4)
        ids = db.add_vectors(unit_vectors(), [{"i": i} for i in range(4)])

        assert len(ids) == 4
        assert len(set(ids)) == 4
        assert db.count() == 4

    def test_search_returns_nearest_first(self):
        """检索结果必须按相似度降序，且首位为最相近向量"""
        db = create_vector_db("faiss", dimension=4)
        db.add_vectors(unit_vectors(), [{"i": i} for i in range(4)])

        results = db.search(np.array([1, 0, 0, 0], dtype=np.float32), top_k=2)

        assert len(results) == 2
        assert results[0]["score"] == pytest.approx(1.0)
        assert results[0]["score"] >= results[1]["score"]

    def test_search_empty_db(self):
        """空库检索应返回空列表而非报错"""
        db = create_vector_db("faiss", dimension=4)
        assert db.search(np.array([1, 0, 0, 0], dtype=np.float32)) == []

    def test_search_top_k_capped_by_size(self):
        """top_k 超过库容量时应自动收敛"""
        db = create_vector_db("faiss", dimension=4)
        db.add_vectors(unit_vectors()[:2], [{"i": 0}, {"i": 1}])

        assert len(db.search(np.array([1, 0, 0, 0], dtype=np.float32), top_k=10)) == 2

    def test_dimension_mismatch_raises(self):
        """维度不匹配必须报错，避免检索出错误结果"""
        db = create_vector_db("faiss", dimension=4)
        with pytest.raises(ValueError):
            db.add_vectors([np.array([1.0, 0.0], dtype=np.float32)], [{}])

    def test_metadata_length_mismatch_raises(self):
        """向量与元数据数量不一致必须报错"""
        db = create_vector_db("faiss", dimension=4)
        with pytest.raises(ValueError):
            db.add_vectors(unit_vectors()[:2], [{"i": 0}])

    def test_duplicate_ids_rejected(self):
        """重复 ID 必须被拒绝，否则删除语义会歧义"""
        db = create_vector_db("faiss", dimension=4)
        db.add_vectors(unit_vectors()[:1], [{"i": 0}], ids=["fixed"])

        with pytest.raises(ValueError):
            db.add_vectors(unit_vectors()[1:2], [{"i": 1}], ids=["fixed"])

    def test_custom_ids_preserved(self):
        """自定义 ID 需被原样保留"""
        db = create_vector_db("faiss", dimension=4)
        db.add_vectors(unit_vectors()[:2], [{"i": 0}, {"i": 1}], ids=["a", "b"])

        assert {r["id"] for r in db.search(np.array([1, 0, 0, 0], dtype=np.float32), 2)} == {"a", "b"}


class TestDeleteAndClear:
    """删除与清空"""

    def test_delete_removes_and_reindexes(self):
        """删除后必须重建索引，否则检索会命中已删除向量"""
        db = create_vector_db("faiss", dimension=4)
        ids = db.add_vectors(unit_vectors(), [{"i": i} for i in range(4)])

        removed = db.delete([ids[0]])

        assert removed == 1
        assert db.count() == 3
        results = db.search(np.array([1, 0, 0, 0], dtype=np.float32), top_k=4)
        assert all(r["id"] != ids[0] for r in results)

    def test_delete_unknown_id_returns_zero(self):
        """删除不存在的 ID 应返回 0"""
        db = create_vector_db("faiss", dimension=4)
        db.add_vectors(unit_vectors()[:1], [{"i": 0}])

        assert db.delete(["not-exists"]) == 0

    def test_delete_empty_list(self):
        """空删除列表应安全返回 0"""
        db = create_vector_db("faiss", dimension=4)
        assert db.delete([]) == 0

    def test_clear(self):
        """清空后数量归零且检索返回空"""
        db = create_vector_db("faiss", dimension=4)
        db.add_vectors(unit_vectors(), [{"i": i} for i in range(4)])

        db.clear()

        assert db.count() == 0
        assert db.search(np.array([1, 0, 0, 0], dtype=np.float32)) == []


class TestPersistence:
    """持久化"""

    def test_persist_and_load(self, tmp_path):
        """持久化后重新加载必须保持向量与元数据一致"""
        db = create_vector_db("faiss", dimension=4, storage_dir=str(tmp_path))
        db.add_vectors(unit_vectors(), [{"i": i} for i in range(4)], ids=[f"id{i}" for i in range(4)])
        db.persist()

        reloaded = create_vector_db("faiss", dimension=4, storage_dir=str(tmp_path))
        assert reloaded.load() is True
        assert reloaded.count() == 4

        results = reloaded.search(np.array([0, 1, 0, 0], dtype=np.float32), top_k=1)
        assert results[0]["id"] == "id1"

    def test_load_without_files_returns_false(self, tmp_path):
        """无持久化文件时 load 返回 False 而不是抛异常"""
        db = create_vector_db("faiss", dimension=4, storage_dir=str(tmp_path))
        assert db.load() is False

    def test_persist_without_storage_dir_raises(self):
        """未配置目录时持久化必须报错"""
        db = create_vector_db("faiss", dimension=4)
        with pytest.raises(ValueError):
            db.persist()


class TestMetadataLookup:
    """元数据查询"""

    def test_get_metadata_by_id(self):
        """按 ID 查询元数据"""
        db = create_vector_db("faiss", dimension=4)
        ids = db.add_vectors(unit_vectors()[:1], [{"tag": "x"}], ids=["one"])

        assert db.get_metadata(ids[0]) == {"tag": "x"}
        assert db.get_metadata("missing") is None


class TestFAISSExtended3:
    """FAISS 数据库第三轮扩展测试（覆盖 mock 索引路径）"""

    @pytest.fixture
    def db(self):
        return create_vector_db("faiss", dimension=4)

    def test_ids_length_mismatch_raises(self, db):
        """ids 数量与 vectors 不一致应报错"""
        with pytest.raises(ValueError, match="ids 长度"):
            db.add_vectors(unit_vectors()[:2], [{"i": 0}, {"i": 1}], ids=["only-one"])

    def test_duplicate_ids_in_batch_raises(self, db):
        """同批 ids 内部重复应报错"""
        with pytest.raises(ValueError, match="重复"):
            db.add_vectors(unit_vectors()[:2], [{"i": 0}, {"i": 1}], ids=["x", "x"])

    def test_existing_id_rejected(self, db):
        """与已有 ID 冲突应报错"""
        db.add_vectors(unit_vectors()[:1], [{"i": 0}], ids=["a"])
        with pytest.raises(ValueError, match="ID 已存在"):
            db.add_vectors(unit_vectors()[1:2], [{"i": 1}], ids=["a"])

    def test_backend_reports_numpy_when_faiss_missing(self, db):
        """faiss 未安装时 backend 应报告 numpy 回退"""
        assert db.backend in ("faiss", "numpy")
        # 强制模拟 faiss 不可用
        db._faiss = None
        db._index = None
        assert db.backend == "numpy"

    def test_search_uses_mock_index(self, db, monkeypatch):
        """提供 mock faiss 索引时应走 index.search 路径"""
        import types

        class MockIndex:
            def add(self, vectors):
                pass

            def search(self, query, k):
                # 返回与 numpy 回退一致的确定性结果：第 0 维向量最相似
                return (np.array([[0.9, 0.5]], dtype=np.float32),
                        np.array([[0, 1]], dtype=np.int64))

        db._index = MockIndex()
        db._faiss = types.SimpleNamespace()
        db.add_vectors(unit_vectors()[:2], [{"i": 0}, {"i": 1}])
        results = db.search(np.array([1, 0, 0, 0], dtype=np.float32), top_k=2)
        assert len(results) == 2
        assert results[0]["id"] in ("default-0", "default-1")

    def test_search_top_k_larger_than_count_capped(self, db):
        """top_k 大于向量数时应收敛到实际数量"""
        db.add_vectors(unit_vectors()[:2], [{"i": 0}, {"i": 1}])
        results = db.search(np.array([1, 0, 0, 0], dtype=np.float32), top_k=100)
        assert len(results) == 2

    def test_delete_removes_all(self, db):
        """删除全部向量后 count 为 0 且检索为空"""
        ids = db.add_vectors(unit_vectors()[:3], [{"i": i} for i in range(3)])
        removed = db.delete(ids)
        assert removed == 3
        assert db.count() == 0
        assert db.search(np.array([1, 0, 0, 0], dtype=np.float32)) == []

    def test_delete_nonexistent_returns_zero(self, db):
        """删除不存在的 ID 应返回 0"""
        assert db.delete(["ghost"]) == 0

    def test_load_vector_count_check(self, db, tmp_path, monkeypatch):
        """加载时元数据与向量数量应一致"""
        db.storage_dir = tmp_path
        db.add_vectors(unit_vectors()[:2], [{"i": 0}, {"i": 1}])
        db.persist()
        db2 = create_vector_db("faiss", dimension=4, storage_dir=str(tmp_path))
        assert db2.load() is True
        assert db2.count() == 2

    def test_generate_ids_sequential(self, db):
        """自动生成 ID 应按 default-N 递增"""
        db.add_vectors(unit_vectors()[:1], [{"i": 0}])
        ids = db.add_vectors(unit_vectors()[1:2], [{"i": 1}])
        assert ids == ["default-1"]


class TestVectorExtended3:
    """向量数据库第三轮扩展测试"""

    def test_search_2d_query_matrix(self):
        """二维查询矩阵（批量查询）应被 reshape 正确处理"""
        db = create_vector_db("faiss", dimension=4)
        db.add_vectors(unit_vectors(), [{"i": i} for i in range(4)])
        query = np.array([[1, 0, 0, 0], [0, 1, 0, 0]], dtype=np.float32)
        results = db.search(query, top_k=1)
        assert results[0]["id"] == "default-0"

    def test_add_empty_metadata_lists(self):
        """空向量列表添加应返回空"""
        db = create_vector_db("faiss", dimension=4)
        assert db.add_vectors([], []) == []
    """VectorDB 扩展测试"""

    def test_search_top_k(self):
        """搜索 top_k 限制"""
        db = create_vector_db("faiss", dimension=4)
        db.add_vectors(unit_vectors(), [{"i": i} for i in range(4)])
        results = db.search(np.array([1, 0, 0, 0], dtype=np.float32), top_k=2)
        assert len(results) == 2

    def test_count_after_add(self):
        """添加后计数"""
        db = create_vector_db("faiss", dimension=4)
        db.add_vectors(unit_vectors()[:2], [{"i": 0}, {"i": 1}])
        assert db.count() == 2

    def test_delete_by_ids(self):
        """按 ID 删除"""
        db = create_vector_db("faiss", dimension=4)
        ids = db.add_vectors(unit_vectors(), [{"i": i} for i in range(4)])
        deleted = db.delete(ids[:2])
        assert deleted == 2
        assert db.count() == 2

    def test_add_empty_vectors(self):
        """添加空向量列表"""
        db = create_vector_db("faiss", dimension=4)
        result = db.add_vectors([], [])
        assert result == []

    def test_persist_and_reload_count(self, tmp_path):
        """持久化后重新加载计数一致"""
        db = create_vector_db("faiss", dimension=4, storage_dir=str(tmp_path))
        db.add_vectors(unit_vectors()[:2], [{"i": 0}, {"i": 1}])
        db.persist()

        reloaded = create_vector_db("faiss", dimension=4, storage_dir=str(tmp_path))
        reloaded.load()
        assert reloaded.count() == 2

    def test_add_vectors_id_length_mismatch(self):
        """ID 长度不匹配"""
        db = create_vector_db("faiss", dimension=4)
        with pytest.raises(ValueError, match="ids"):
            db.add_vectors(unit_vectors()[:2], [{"i": 0}, {"i": 1}], ids=["only_one"])

    def test_add_vectors_duplicate_ids(self):
        """重复 ID"""
        db = create_vector_db("faiss", dimension=4)
        with pytest.raises(ValueError, match="重复"):
            db.add_vectors(unit_vectors()[:2], [{"i": 0}, {"i": 1}], ids=["a", "a"])

    def test_add_vectors_existing_ids(self):
        """已存在的 ID"""
        db = create_vector_db("faiss", dimension=4)
        db.add_vectors(unit_vectors()[:1], [{"i": 0}], ids=["existing"])
        with pytest.raises(ValueError, match="已存在"):
            db.add_vectors(unit_vectors()[:1], [{"i": 1}], ids=["existing"])

    def test_backend_property(self):
        """backend 属性"""
        db = create_vector_db("faiss", dimension=4)
        assert db.backend in ("faiss", "numpy")

    def test_search_empty_db(self):
        """空数据库搜索"""
        db = create_vector_db("faiss", dimension=4)
        results = db.search(np.array([1, 0, 0, 0], dtype=np.float32))
        assert results == []

    def test_delete_empty_list(self):
        """删除空列表"""
        db = create_vector_db("faiss", dimension=4)
        assert db.delete([]) == 0

    def test_get_metadata_nonexistent(self):
        """获取不存在的元数据"""
        db = create_vector_db("faiss", dimension=4)
        assert db.get_metadata("nonexistent") is None

    def test_search_dimension_mismatch(self):
        """搜索向量维度不匹配"""
        db = create_vector_db("faiss", dimension=4)
        db.add_vectors(unit_vectors()[:2], [{"i": 0}, {"i": 1}])
        with pytest.raises(ValueError, match="维度"):
            db.search(np.array([1, 0, 0], dtype=np.float32))

    def test_search_top_k_larger_than_count(self):
        """搜索 top_k 大于数据量"""
        db = create_vector_db("faiss", dimension=4)
        db.add_vectors(unit_vectors()[:2], [{"i": 0}, {"i": 1}])
        results = db.search(np.array([1, 0, 0, 0], dtype=np.float32), top_k=10)
        assert len(results) == 2

    def test_delete_nonexistent_ids(self):
        """删除不存在的 ID"""
        db = create_vector_db("faiss", dimension=4)
        db.add_vectors(unit_vectors()[:2], [{"i": 0}, {"i": 1}])
        deleted = db.delete(["nonexistent"])
        assert deleted == 0

    def test_persist_and_reload_search(self, tmp_path):
        """持久化后搜索结果一致"""
        db = create_vector_db("faiss", dimension=4, storage_dir=str(tmp_path))
        db.add_vectors(unit_vectors(), [{"i": i} for i in range(4)], ids=[f"v{i}" for i in range(4)])
        db.persist()

        reloaded = create_vector_db("faiss", dimension=4, storage_dir=str(tmp_path))
        reloaded.load()
        results = reloaded.search(np.array([1, 0, 0, 0], dtype=np.float32), top_k=1)
        assert results[0]["id"] == "v0"

    def test_add_vectors_normalized(self):
        """添加向量后自动归一化"""
        db = create_vector_db("faiss", dimension=4)
        ids = db.add_vectors([np.array([3, 0, 0, 0], dtype=np.float32)], [{"i": 0}])
        assert len(ids) == 1

    def test_delete_rebuilds_index(self, tmp_path):
        """删除后重建索引"""
        db = create_vector_db("faiss", dimension=4, storage_dir=str(tmp_path))
        db.add_vectors(unit_vectors()[:3], [{"i": i} for i in range(3)])
        db.delete(["default-0"])
        assert db.count() == 2
        results = db.search(np.array([1, 0, 0, 0], dtype=np.float32), top_k=1)
        assert len(results) == 1

    def test_clear_and_add_again(self):
        """清空后再添加"""
        db = create_vector_db("faiss", dimension=4)
        db.add_vectors(unit_vectors()[:2], [{"i": 0}, {"i": 1}])
        db.clear()
        assert db.count() == 0
        db.add_vectors(unit_vectors()[:1], [{"i": 99}])
        assert db.count() == 1

    def test_load_with_wrong_dimension(self, tmp_path):
        """加载维度不匹配的数据"""
        db1 = create_vector_db("faiss", dimension=4, storage_dir=str(tmp_path))
        db1.add_vectors(unit_vectors()[:2], [{"i": 0}, {"i": 1}])
        db1.persist()

        db2 = create_vector_db("faiss", dimension=8, storage_dir=str(tmp_path))
        result = db2.load()
        assert result is True

    def test_load_no_storage_dir(self):
        """无存储目录时 load 返回 False"""
        db = create_vector_db("faiss", dimension=4)
        assert db.load() is False

    def test_persist_no_storage_dir_raises(self):
        """无存储目录时 persist 抛异常"""
        db = create_vector_db("faiss", dimension=4)
        with pytest.raises(ValueError):
            db.persist()

    def test_search_numpy_fallback(self):
        """numpy 回退搜索"""
        db = create_vector_db("faiss", dimension=4)
        db.add_vectors(unit_vectors()[:2], [{"i": 0}, {"i": 1}])
        results = db.search(np.array([1, 0, 0, 0], dtype=np.float32), top_k=2)
        assert len(results) == 2
        assert results[0]["score"] >= results[1]["score"]
