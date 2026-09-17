"""智能去重单元测试

去重直接影响训练数据规模，阈值校验、索引保留与相似对查询必须正确。
"""

import pytest
import numpy as np
from unittest.mock import MagicMock

from augmentor.dedup import Deduplicator, DedupResult


class TestInit:
    """初始化校验"""

    def test_rejects_out_of_range_threshold(self):
        """阈值必须在 0-1 之间，否则相似度比较失去意义"""
        with pytest.raises(ValueError):
            Deduplicator(threshold=1.5)
        with pytest.raises(ValueError):
            Deduplicator(threshold=-0.1)

    def test_default_threshold(self):
        """默认阈值应为 0.9"""
        dedup = Deduplicator()
        assert dedup.threshold == 0.9

    def test_custom_threshold(self):
        """自定义阈值应正确设置"""
        dedup = Deduplicator(threshold=0.8)
        assert dedup.threshold == 0.8


class TestDeduplicate:
    """去重主流程"""

    def test_removes_duplicates_and_keeps_first(self):
        """重复组应保留首个样本，保证结果稳定可复现"""
        dedup = Deduplicator(threshold=0.9)
        items = [
            {"instruction": "如何申请入住安居乐寓？"},
            {"instruction": "如何申请入住安居乐寓？"},
            {"instruction": "退租流程是什么？"},
        ]
        result = dedup.deduplicate(items)

        assert isinstance(result, DedupResult)
        assert result.original_count == 3
        assert result.removed_count >= 1
        assert 0 in result.kept_indices
        assert len(result.kept_indices) == result.deduplicated_count

    def test_empty_dataset(self):
        """空数据集应返回零值结果而非异常"""
        dedup = Deduplicator()
        result = dedup.deduplicate([])

        assert result.original_count == 0
        assert result.deduplicated_count == 0
        assert result.kept_indices == []

    def test_deduplicate_and_filter_returns_items(self):
        """过滤后的列表长度必须与 kept_indices 一致"""
        dedup = Deduplicator(threshold=0.9)
        items = [{"instruction": f"问题 {i}"} for i in range(5)]
        filtered = dedup.deduplicate_and_filter(items)

        assert len(filtered) <= len(items)
        assert all(isinstance(item, dict) for item in filtered)

    def test_custom_text_key(self):
        """支持自定义去重字段，适配不同数据结构"""
        dedup = Deduplicator(threshold=0.9)
        items = [{"question": "相同问题"}, {"question": "相同问题"}]
        result = dedup.deduplicate(items, text_key="question")

        assert result.original_count == 2
        assert result.deduplicated_count <= 2

    def test_no_duplicates(self):
        """没有重复时应保留所有数据"""
        dedup = Deduplicator(threshold=0.9)
        items = [
            {"instruction": "完全不同的问题1"},
            {"instruction": "完全不同的问题2"},
            {"instruction": "完全不同的问题3"},
        ]
        result = dedup.deduplicate(items)

        assert result.original_count == 3
        assert result.removed_count == 0
        assert result.deduplicated_count == 3

    def test_all_duplicates(self):
        """全部相同时应只保留一个"""
        dedup = Deduplicator(threshold=0.9)
        items = [
            {"instruction": "相同问题"},
            {"instruction": "相同问题"},
            {"instruction": "相同问题"},
        ]
        result = dedup.deduplicate(items)

        assert result.original_count == 3
        assert result.removed_count >= 2
        assert result.deduplicated_count == 1

    def test_dedup_result_fields(self):
        """DedupResult 应包含所有必要字段"""
        dedup = Deduplicator(threshold=0.9)
        items = [{"instruction": "测试"}]
        result = dedup.deduplicate(items)

        assert hasattr(result, 'original_count')
        assert hasattr(result, 'deduplicated_count')
        assert hasattr(result, 'removed_count')
        assert hasattr(result, 'duplicate_groups')
        assert hasattr(result, 'kept_indices')

    def test_duplicate_groups_structure(self):
        """重复组应为索引列表的列表"""
        dedup = Deduplicator(threshold=0.9)
        items = [
            {"instruction": "相同问题1"},
            {"instruction": "相同问题1"},
            {"instruction": "不同问题"},
        ]
        result = dedup.deduplicate(items)

        assert isinstance(result.duplicate_groups, list)
        for group in result.duplicate_groups:
            assert isinstance(group, list)
            assert len(group) >= 2


class TestReport:
    """去重报告"""

    def test_report_fields(self):
        """报告需给出移除比例，供质量报告聚合"""
        dedup = Deduplicator(threshold=0.9)
        items = [{"instruction": "一样的问题"}, {"instruction": "一样的问题"}]
        report = dedup.generate_report(items)

        assert report["original_count"] == 2
        assert "removal_rate" in report
        assert 0.0 <= report["removal_rate"] <= 1.0
        assert report["threshold"] == 0.9

    def test_report_on_empty(self):
        """空数据集移除比例应为 0"""
        report = Deduplicator().generate_report([])
        assert report["removal_rate"] == 0

    def test_report_contains_all_fields(self):
        """报告应包含所有必要字段"""
        dedup = Deduplicator(threshold=0.9)
        items = [{"instruction": "测试"}]
        report = dedup.generate_report(items)

        assert "original_count" in report
        assert "deduplicated_count" in report
        assert "removed_count" in report
        assert "removal_rate" in report
        assert "duplicate_groups_count" in report
        assert "avg_group_size" in report
        assert "threshold" in report


class TestFindSimilarPairs:
    """相似对查询（回归测试）

    该方法此前调用了不存在的 _compute_similarity_matrix，会在运行期抛
    AttributeError，导致 Web UI 的相似样本视图直接 500。
    """

    def test_returns_pairs_without_attribute_error(self):
        """必须能正常返回相似对，不能再抛 AttributeError"""
        dedup = Deduplicator(threshold=0.5)
        items = [
            {"instruction": "如何申请入住安居乐寓？"},
            {"instruction": "如何申请入住安居乐寓呢？"},
            {"instruction": "完全不相关的问题内容"},
        ]
        pairs = dedup.find_similar_pairs(items, top_k=5)

        assert isinstance(pairs, list)
        for first, second, similarity in pairs:
            assert isinstance(first, int) and isinstance(second, int)
            assert 0.0 <= similarity <= 1.0

    def test_returns_empty_for_single_item(self):
        """少于两条数据时无相似对可言"""
        dedup = Deduplicator()
        assert dedup.find_similar_pairs([{"instruction": "唯一"}]) == []

    def test_returns_empty_for_empty_dataset(self):
        """空数据集应返回空列表"""
        dedup = Deduplicator()
        assert dedup.find_similar_pairs([]) == []

    def test_top_k_limits_results(self):
        """top_k 应限制返回的相似对数量"""
        dedup = Deduplicator(threshold=0.5)
        items = [{"instruction": f"问题{i}"} for i in range(10)]
        pairs = dedup.find_similar_pairs(items, top_k=3)

        assert len(pairs) <= 3

    def test_similarity_range(self):
        """相似度应在 0-1 之间"""
        dedup = Deduplicator(threshold=0.5)
        items = [
            {"instruction": "如何申请租房？"},
            {"instruction": "如何申请租房呢？"},
        ]
        pairs = dedup.find_similar_pairs(items, top_k=5)

        for _, _, similarity in pairs:
            assert 0.0 <= similarity <= 1.0


class TestNgramSimilarity:
    """N-gram 相似度测试"""

    def test_identical_texts(self):
        """相同文本相似度应为 1"""
        dedup = Deduplicator()
        sim = dedup._ngram_similarity("hello", "hello")
        assert sim == pytest.approx(1.0)

    def test_disjoint_texts(self):
        """无公共字符的文本相似度应为 0"""
        dedup = Deduplicator()
        sim = dedup._ngram_similarity("abcd", "wxyz")
        assert sim == pytest.approx(0.0)

    def test_empty_texts(self):
        """空文本相似度应为 0"""
        dedup = Deduplicator()
        sim = dedup._ngram_similarity("", "abc")
        assert sim == 0.0

    def test_one_empty_text(self):
        """一个空文本相似度应为 0"""
        dedup = Deduplicator()
        sim = dedup._ngram_similarity("abc", "")
        assert sim == 0.0

    def test_both_empty(self):
        """两个空文本相似度应为 0"""
        dedup = Deduplicator()
        sim = dedup._ngram_similarity("", "")
        assert sim == 0.0


class TestFallbackEncode:
    """Fallback 编码测试"""

    def test_fallback_encode_returns_matrix(self):
        """Fallback 编码应返回矩阵"""
        dedup = Deduplicator()
        dedup._model = "fallback"
        
        texts = ["hello world", "test text"]
        embeddings = dedup._fallback_encode(texts)
        
        assert embeddings.shape[0] == 2
        assert embeddings.shape[1] > 0

    def test_fallback_encode_normalization(self):
        """Fallback 编码应进行 L2 归一化"""
        dedup = Deduplicator()
        dedup._model = "fallback"
        
        texts = ["hello", "world"]
        embeddings = dedup._fallback_encode(texts)
        
        # 检查归一化
        norms = np.linalg.norm(embeddings, axis=1)
        for norm in norms:
            assert abs(norm - 1.0) < 1e-6


class TestComputeSimilarityMatrixChunked:
    """分块相似度矩阵测试"""

    def test_chunked_matrix_shape(self):
        """输出矩阵应为 n x n"""
        dedup = Deduplicator()
        embeddings = np.random.rand(5, 10)
        matrix = dedup._compute_similarity_matrix_chunked(embeddings, chunk_size=2)
        assert matrix.shape == (5, 5)

    def test_chunked_matrix_symmetric(self):
        """相似度矩阵应对称"""
        dedup = Deduplicator()
        embeddings = np.random.rand(6, 10)
        matrix = dedup._compute_similarity_matrix_chunked(embeddings, chunk_size=3)
        np.testing.assert_allclose(matrix, matrix.T, atol=1e-6)

    def test_chunked_matrix_diagonal_positive(self):
        """对角线元素应为 1（自相似）"""
        dedup = Deduplicator()
        embeddings = np.random.rand(4, 10)
        matrix = dedup._compute_similarity_matrix_chunked(embeddings, chunk_size=1000)
        for i in range(4):
            assert matrix[i, i] >= 0.99

    def test_zero_embeddings_handled(self):
        """零向量不应导致除零错误"""
        dedup = Deduplicator()
        embeddings = np.array([[0, 0, 0], [1, 0, 0], [0, 1, 0]], dtype=np.float32)
        matrix = dedup._compute_similarity_matrix_chunked(embeddings)
        assert matrix.shape == (3, 3)


class TestFindDuplicatesFaiss:
    """FAISS 去重测试（ImportError 分支）"""

    def test_faiss_not_installed_raises(self):
        """未安装 faiss 时应抛出 ImportError"""
        dedup = Deduplicator()
        embeddings = np.random.rand(3, 10)
        with pytest.raises(ImportError, match="FAISS"):
            dedup._find_duplicates_faiss(["a", "b", "c"], embeddings)


class TestDeduplicateChunked:
    """分块去重流程测试"""

    def test_empty_texts_returns_empty(self):
        """空文本列表应返回空重复组"""
        dedup = Deduplicator()
        dedup._model = "fallback"
        groups = dedup._find_duplicate_groups_chunked([], chunk_size=1000)
        assert groups == []

    def test_small_dataset_single_chunk(self):
        """小数据集应使用单个分块"""
        dedup = Deduplicator(threshold=0.9)
        dedup._model = "fallback"
        texts = ["相同问题", "相同问题", "不同问题"]
        groups = dedup._find_duplicate_groups_chunked(texts, chunk_size=1000)
        assert isinstance(groups, list)

    def test_large_chunk_size(self):
        """chunk_size 大于数据量时应正常工作"""
        dedup = Deduplicator(threshold=0.9)
        dedup._model = "fallback"
        texts = ["问题A", "问题B"]
        groups = dedup._find_duplicate_groups_chunked(texts, chunk_size=10000)
        assert isinstance(groups, list)


class TestBatchEncodeFallback:
    """批量编码 fallback 测试"""

    def test_batch_encode_fallback(self):
        """fallback 模型应使用 n-gram 编码"""
        dedup = Deduplicator()
        dedup._model = "fallback"
        texts = ["hello", "world", "test"]
        embeddings = dedup._batch_encode(texts)
        assert embeddings.shape[0] == 3
        assert embeddings.shape[1] > 0

    def test_batch_encode_single_text(self):
        """单条文本也应正常编码"""
        dedup = Deduplicator()
        dedup._model = "fallback"
        embeddings = dedup._batch_encode(["hello"])
        assert embeddings.shape[0] == 1


class TestBatchEncodeRealModel:
    """批量编码真实模型路径测试"""

    def test_batch_encode_with_mock_model(self):
        """当模型为真实对象时应调用 encode 方法"""
        dedup = Deduplicator()
        mock_model = MagicMock()
        mock_model.encode.return_value = np.array([[0.5, 0.5], [0.5, 0.5]])
        dedup._model = mock_model
        dedup._batch_encode(["a", "b"])
        assert mock_model.encode.called
        # encode 应接收原始文本列表与固定参数
        args, kwargs = mock_model.encode.call_args
        assert list(args[0]) == ["a", "b"]
        assert kwargs.get("show_progress_bar") is False
        assert kwargs.get("batch_size") == 64

    def test_batch_encode_with_mock_model_no_call(self):
        """空文本列表时 encode 不被调用"""
        dedup = Deduplicator()
        mock_model = MagicMock()
        mock_model.encode.return_value = np.array([])
        dedup._model = mock_model
        dedup._batch_encode([])
        assert mock_model.encode.called

    def test_batch_encode_result_shape_matches_input(self):
        """编码结果行数应与输入一致"""
        dedup = Deduplicator()
        mock_model = MagicMock()
        mock_model.encode.return_value = np.array([[0.1, 0.9]] * 3)
        dedup._model = mock_model
        result = dedup._batch_encode(["x", "y", "z"])
        assert result.shape[0] == 3


class TestLoadModelExtended:
    """_load_model 扩展测试"""

    def test_load_model_idempotent(self):
        """重复调用 _load_model 不应重新初始化"""
        dedup = Deduplicator()
        dedup._load_model()
        first_model = dedup._model
        dedup._load_model()
        assert dedup._model is first_model

    def test_load_model_sets_model_type(self):
        """_load_model 应设置模型类型"""
        dedup = Deduplicator()
        dedup._load_model()
        assert dedup._model is not None


class TestDedupResultExtended:
    """DedupResult 扩展测试"""

    def test_dedup_result_equality(self):
        """DedupResult 应支持比较"""
        r1 = DedupResult(
            original_count=5, deduplicated_count=3,
            removed_count=2, duplicate_groups=[[0, 1]],
            kept_indices=[0, 2, 4]
        )
        r2 = DedupResult(
            original_count=5, deduplicated_count=3,
            removed_count=2, duplicate_groups=[[0, 1]],
            kept_indices=[0, 2, 4]
        )
        assert r1.original_count == r2.original_count
        assert r1.kept_indices == r2.kept_indices
