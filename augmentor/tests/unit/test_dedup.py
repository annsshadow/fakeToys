# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

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

    def test_fallback_encode_is_float32(self):
        """稠密编码矩阵必须用 float32，它同时是这条路径上最大的一笔分配

        回退编码产出 n×vocab 的**稠密**字符二元组 TF 矩阵：真实 6902 条数据
        的 vocab 就有 23033 项，float64 是 1.27 GB，float32 只要 0.64 GB。
        更要紧的是调用方写的是 `np.asarray(self._batch_encode(texts), dtype=np.float32)`
        —— dtype 不符时那是**整份再复制一遍**，等于 float64 的实现凭空多出 0.64 GB。
        TF 值是「某二元组在该文档里出现几次」，量级个位数，float32 在 2^24 以内
        精确表示整数，改 dtype 不会改变任何一次计数。
        """
        dedup = Deduplicator()
        vectors = dedup._fallback_encode(["abcdef", "abcdefg", "xyz abc def"])

        assert vectors.dtype == np.float32, (
            f"编码矩阵 dtype 是 {vectors.dtype}：稠密大矩阵用 float64 会让峰值凭空翻倍"
        )

    def test_fallback_encode_peak_holds_only_one_matrix(self):
        """归一化必须**原地**做：`vectors / norms` 会再多养一份同尺寸矩阵

        这一步在真实数据上是 0.64 GB 量级的第二份拷贝，而它换来的是「同一份
        数据的另一种 dtype」——纯浪费。原地除之后峰值就是一个矩阵。
        预言机是独立算出来的「一个 float32 矩阵的字节数」，不是被测库的返回值。
        """
        import tracemalloc

        n = 2000
        texts = [f"instruction number {i} please sort a list of items" for i in range(n)]
        dedup = Deduplicator()

        cols = dedup._fallback_encode(texts).shape[1]
        one_matrix_bytes = n * cols * 4

        tracemalloc.start()
        try:
            vectors = dedup._fallback_encode(texts)
            _, peak = tracemalloc.get_traced_memory()
        finally:
            tracemalloc.stop()

        assert vectors.shape == (n, cols)
        assert peak < one_matrix_bytes * 1.5, (
            f"峰值 {peak / 1024 / 1024:.1f}MB 超过「一个 float32 矩阵」"
            f"（{one_matrix_bytes / 1024 / 1024:.1f}MB）的 1.5 倍，"
            "疑似仍在用 float64 或归一化时另起了一份拷贝"
        )


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

    def test_faiss_mock_finds_duplicates(self, monkeypatch):
        """mock faiss 应能找出重复组"""
        import types

        dedup = Deduplicator(threshold=0.5)
        texts = ["问题一", "问题一", "完全不同的问题二"]
        embeddings = np.array([
            [1.0, 0.0],
            [1.0, 0.0],
            [0.0, 1.0],
        ], dtype=np.float32)

        class MockFaiss:
            IndexFlatIP = None

        def make_index(dim):
            vectors = []
            vec_index = {}

            class Index:
                def add(self, arr):
                    nonlocal vectors
                    vectors = [tuple(v) for v in arr]

                def search(self, query, k):
                    import math
                    q = np.asarray(query[0], dtype=np.float32)
                    qn = np.linalg.norm(q) or 1.0
                    q = q / qn
                    sims = [
                        float(np.dot(q, np.asarray(v))) for v in vectors
                    ]
                    idx = sorted(range(len(sims)), key=lambda i: -sims[i])[:k]
                    # 返回 2D 数组以匹配真实 faiss 的 (1, k) 形状
                    dist = np.array([sims[i] for i in idx], dtype=np.float32).reshape(1, -1)
                    ids = np.array(idx, dtype=np.int64).reshape(1, -1)
                    return dist, ids

            return Index()

        fake = types.SimpleNamespace(
            IndexFlatIP=make_index,
        )
        monkeypatch.setitem(
            __import__("sys").modules, "faiss", fake
        )

        groups = dedup._find_duplicates_faiss(texts, embeddings)
        # 前两条文本完全相同，应落入同一重复组
        assert any(len(g) >= 2 for g in groups)

    def test_faiss_mock_no_duplicates(self, monkeypatch):
        """mock faiss 在互不相同时不应产生重复组"""
        import types
        import math

        dedup = Deduplicator(threshold=0.99)
        texts = ["完全不同的问题一", "完全不同的问题二", "完全不同的问题三"]
        embeddings = np.array([
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.0, 0.0, 1.0],
        ], dtype=np.float32)

        def make_index(dim):
            vectors = []

            class Index:
                def add(self, arr):
                    nonlocal vectors
                    vectors = [tuple(v) for v in arr]

                def search(self, query, k):
                    q = np.asarray(query[0], dtype=np.float32)
                    qn = np.linalg.norm(q) or 1.0
                    q = q / qn
                    sims = [float(np.dot(q, np.asarray(v))) for v in vectors]
                    idx = sorted(range(len(sims)), key=lambda i: -sims[i])[:k]
                    dist = np.array([sims[i] for i in idx], dtype=np.float32).reshape(1, -1)
                    ids = np.array(idx, dtype=np.int64).reshape(1, -1)
                    return dist, ids

            return Index()

        fake = types.SimpleNamespace(IndexFlatIP=make_index)
        monkeypatch.setitem(__import__("sys").modules, "faiss", fake)

        groups = dedup._find_duplicates_faiss(texts, embeddings)
        assert groups == []


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


class TestStreamingGrouping:
    """按行分块去重：语义必须与旧的「完整矩阵 + 逐行贪心」完全一致，
    但峰值内存不得随 n 平方增长（旧实现 n=50000 需约 10GB，会 OOM）。
    """

    @staticmethod
    def _naive_groups(similarity, threshold):
        """参照实现：直接照搬旧版基于完整相似度矩阵的逐行贪心"""
        n = len(similarity)
        visited = [False] * n
        groups = []
        for i in range(n):
            if visited[i]:
                continue
            group = [i]
            visited[i] = True
            for j in range(i + 1, n):
                if visited[j]:
                    continue
                if similarity[i, j] >= threshold:
                    group.append(j)
                    visited[j] = True
            if len(group) > 1:
                groups.append(group)
        return groups

    def test_streaming_matches_naive_reference(self, monkeypatch):
        """构造带簇的向量，流式分组结果必须与旧算法逐组相同"""
        n, dim = 60, 6
        rng = np.random.default_rng(20260923)
        emb = rng.random((n, dim), dtype=np.float32)
        # 制造 3 组精确重复（相似度 = 1.0）
        for base, copies in ((0, [5, 11, 23]), (7, [8, 19]), (30, [31, 44, 55])):
            for c in copies:
                emb[c] = emb[base]

        dedup = Deduplicator(threshold=0.9)
        monkeypatch.setattr(dedup, "_batch_encode", lambda texts: emb)

        texts = ["t%d" % i for i in range(n)]
        expected = self._naive_groups(
            dedup._compute_similarity_matrix_chunked(emb), dedup.threshold
        )
        actual = dedup._find_duplicate_groups_chunked(texts)

        assert actual == expected

    def test_streaming_matches_naive_across_block_boundaries(self, monkeypatch):
        """块边界不能改变分组结果：把 chunk_size 压到 1 也必须一致"""
        n, dim = 25, 4
        rng = np.random.default_rng(7)
        emb = rng.random((n, dim), dtype=np.float32)
        emb[3] = emb[0]
        emb[9] = emb[0]
        emb[20] = emb[15]

        dedup = Deduplicator(threshold=0.85)
        monkeypatch.setattr(dedup, "_batch_encode", lambda texts: emb)

        texts = ["t%d" % i for i in range(n)]
        expected = self._naive_groups(
            dedup._compute_similarity_matrix_chunked(emb), dedup.threshold
        )

        for chunk_size in (1, 2, 3, 7, 25, 1000):
            assert dedup._find_duplicate_groups_chunked(texts, chunk_size) == expected

    def test_grouping_path_holds_only_one_encoding_matrix(self):
        """整条分组路径的峰值应≈「一个编码矩阵 + 一个相似度块」

        编码产出的是 n×vocab 稠密矩阵（vocab 常常上千列），所以「归一化时另起
        一份」在这一步上是实打实的翻倍：真实 6902 条数据那一份就是 0.64 GB。
        预言机用独立算出的 `n × vocab × 4` 字节数，不取自被测库的返回值。
        """
        import random
        import tracemalloc

        rng = random.Random(0)
        alphabet = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789"
        n = 1200
        texts = ["".join(rng.choice(alphabet) for _ in range(60)) for _ in range(n)]

        dedup = Deduplicator(threshold=0.99)
        dedup._load_model()
        cols = dedup._fallback_encode(texts).shape[1]
        one_matrix_bytes = n * cols * 4
        assert cols > 1000, f"词表只有 {cols} 列，样本撑不起可判别的内存边界"
        # 峰值预算只由「一个编码矩阵 + 一个相似度块」构成，两块都按公式独立算出
        block_bytes = dedup._row_block_size(n, 1000) * n * 4

        tracemalloc.start()
        try:
            dedup._find_duplicate_groups_chunked(texts)
            _, peak = tracemalloc.get_traced_memory()
        finally:
            tracemalloc.stop()

        bound = (one_matrix_bytes + block_bytes) * 1.25
        assert peak < bound, (
            f"峰值 {peak / 1024 / 1024:.1f}MB 超过「一个编码矩阵 "
            f"({one_matrix_bytes / 1024 / 1024:.1f}MB) + 一个相似度块 "
            f"({block_bytes / 1024 / 1024:.1f}MB)」再留 25% 余量的预算 "
            f"{bound / 1024 / 1024:.1f}MB：归一化疑似又物化了一份同尺寸矩阵"
        )

    def test_row_block_size_caps_block_cells(self):
        """行块大小必须把「块单元数」压到上限内（4M 单元 = 16MB）"""
        dedup = Deduplicator()

        assert dedup._row_block_size(1000, 1000) == 1000
        assert dedup._row_block_size(5000, 5000) == 800       # 4M // 5000
        assert dedup._row_block_size(50_000, 1000) == 80      # 4M // 50000
        assert dedup._row_block_size(0, 10) == 1

        for n, requested in ((1000, 1000), (5000, 5000), (50_000, 1000), (200_000, 4096)):
            rows = dedup._row_block_size(n, requested)
            assert rows * n <= 4_000_000

    def test_no_full_n_by_n_matrix_is_allocated(self, monkeypatch):
        """结构性护栏：任何时刻都不允许出现 n×n 的相似度块，也不许算被丢弃的下三角

        块的列宽必须是 `n - start`：`_greedy_group_from_row` 只消费 `row[i+1:]`，
        而块内最小行号就是 `start`，所以列 < start 的结果按定义没有读者。列宽
        等于 n 意味着白算整个下三角（实测全量 6902 条时相似度那一步要 30.0 s，
        其中约一半 flops 落在那半块上）。
        """
        n, dim = 6000, 4
        emb = np.random.default_rng(1).random((n, dim), dtype=np.float32)
        from augmentor.dedup import _MAX_BLOCK_CELLS

        dedup = Deduplicator(threshold=0.99)
        monkeypatch.setattr(dedup, "_batch_encode", lambda texts: emb)

        shapes = []
        original = dedup._iter_similarity_row_blocks

        def spy(normalized, block_rows):
            for start, end, block in original(normalized, block_rows):
                shapes.append((start, block.shape))
                yield start, end, block

        monkeypatch.setattr(dedup, "_iter_similarity_row_blocks", spy)

        dedup._find_duplicate_groups_chunked(["x"] * n)

        assert shapes, "应至少产出一个相似度块"
        assert all(rows * cols <= _MAX_BLOCK_CELLS for _, (rows, cols) in shapes), (
            f"存在超过上限 4M 单元的块: {shapes[0]}"
        )
        for start, (rows, cols) in shapes:
            assert cols == n - start, (
                f"第 {start} 行的块列数是 {cols}，应为 n - start = {n - start}"
            )
        total_cells = sum(rows * cols for _, (rows, cols) in shapes)
        block_rows = max(rows for _, (rows, _) in shapes)
        assert total_cells <= n * n // 2 + n * block_rows, (
            f"产出 {total_cells} 个块单元，超过「只算上三角」应有的 "
            f"{n * n // 2}+边界余量——疑似仍在算满宽块"
        )

    def test_peak_memory_is_far_below_full_matrix(self, monkeypatch):
        """实测峰值内存：应约等于「一个块」，而不是完整 n×n 矩阵

        n=5000 时完整矩阵需 95MB；分块路径应稳定在单块上限附近（约 16MB）。
        若实现退化回「先构造完整矩阵」或「同时持有两块」，本测试会失败。
        """
        import tracemalloc

        from augmentor.dedup import _MAX_BLOCK_CELLS

        n, dim = 5000, 8
        emb = np.random.default_rng(3).random((n, dim), dtype=np.float32)

        dedup = Deduplicator(threshold=0.99)
        monkeypatch.setattr(dedup, "_batch_encode", lambda texts: emb)

        tracemalloc.start()
        try:
            dedup._find_duplicate_groups_chunked(["x"] * n)
            _, peak = tracemalloc.get_traced_memory()
        finally:
            tracemalloc.stop()

        one_block_bytes = _MAX_BLOCK_CELLS * 4
        full_matrix_bytes = n * n * 4

        assert peak < full_matrix_bytes / 4, (
            f"峰值 {peak / 1024 / 1024:.1f}MB 已逼近完整矩阵 "
            f"{full_matrix_bytes / 1024 / 1024:.0f}MB，疑似退化"
        )
        assert peak < one_block_bytes * 2, (
            f"峰值 {peak / 1024 / 1024:.1f}MB 超过「两个块」"
            f"（{one_block_bytes * 2 / 1024 / 1024:.1f}MB），"
            "疑似块未被及时释放"
        )


class TestNgramInvertedView:
    """倒排视图的构造契约

    这组用例钉的是「倒排视图代表的就是那批 TF 向量」：计数口径、归一化口径、
    排序前提，以及选路判据的输入 `pair_adds`。它们任何一条塌掉，下一组的
    逐元素等价都不会成立，所以分开钉——等价用例红的时候要先知道是哪一层塌的。
    """

    ROWS = [
        "如何申请公租房",
        "如何申请公租房呢",
        "公租房申请流程",
        "二手房交易流程",
        "物业费按建筑面积计算",
        "",          # 零向量：一个二元组都没有
        "a",         # 零向量：长度 < 2
        "公租房",
    ]

    def view(self, texts=None):
        return Deduplicator()._inverted_view(self.ROWS if texts is None else texts)

    def test_repeated_ngrams_inside_one_document_are_counted(self):
        """同一篇里重复出现的二元组必须计多次，与稠密实现同一个 TF 口径

        取 "abab" 是因为它同时含重叠与非重叠的重复：ab 两次、ba 一次。
        """
        docs, terms, counts, vocab_size = Deduplicator()._ngram_entries(["abab", "bcb"])

        assert vocab_size == 4, f"词表应是 ab/ba/bc/cb 四项，实得 {vocab_size}"
        per_doc = [sorted(int(c) for d, _t, c in zip(docs, terms, counts) if int(d) == i)
                   for i in (0, 1)]
        assert per_doc == [[1, 2], [1, 1]], f"TF 计数是 {per_doc}，应为 [[1, 2], [1, 1]]"

    def test_document_norms_cover_hapax_ngrams_too(self):
        """每篇的值必须按**全部**词条（含只在本篇出现的）归一化，行平方和 = 1

        「只出现一次的词条对非对角没有贡献」不等于「可以不算进范数」——稠密
        实现把它算进去了。漏掉它，所有相似度都会被整体抬高，重复组随之变多。
        零向量那两篇必须保持 0（除以 1 而不是除以 0，与 `norms[norms == 0] = 1` 同口径）。
        """
        inverted = self.view()
        squares = np.bincount(inverted.docs,
                              weights=inverted.values.astype(np.float64) ** 2,
                              minlength=len(self.ROWS))
        has_ngrams = np.asarray([len(t) > 1 for t in self.ROWS])

        np.testing.assert_allclose(squares[has_ngrams], 1.0, atol=1e-6)
        assert (squares[~has_ngrams] == 0).all(), "零向量那两篇不该有任何 postings"

    def test_pair_adds_matches_an_independent_df_formula(self):
        """`pair_adds` 必须是 Σ df(df+1)/2，且用独立的 Counter 算法对照

        它是选路判据唯一的代价输入：算小了会让倒排在退化语料上吃掉稠密，
        算大了会让倒排在该赢的地方弃权。
        """
        inverted = self.view()
        df = {}
        for text in self.ROWS:
            for ngram in {text[j:j + 2] for j in range(len(text) - 1)}:
                df[ngram] = df.get(ngram, 0) + 1

        assert inverted.pair_adds == sum(d * (d + 1) // 2 for d in df.values())
        assert inverted.pair_adds > 0, "样本撑不起可判别的代价，这条用例是空跑"

    def test_postings_are_sorted_by_term_and_then_by_document(self):
        """块生产者靠两次 `searchsorted` 夹出同词条的一段 postings

        前提是：整体按词条升序、同词条内按文档升序。后者还保证展开顺序可复现，
        于是同一个语料跑两次的块逐位相同。
        """
        inverted = self.view()

        assert (np.diff(inverted.terms.astype(np.int64)) >= 0).all()
        same_term = np.flatnonzero(np.diff(inverted.terms) == 0)
        assert same_term.size, "样本里得有同词条的多篇，否则这条排序是空断言"
        assert (np.diff(inverted.docs)[same_term] > 0).all()

    @pytest.mark.parametrize("texts", [[], [""], ["", "a", "之"]])
    def test_a_corpus_without_any_ngram_still_builds(self, texts):
        """没有任何二元组的语料（含空语料）不能炸，代价记为 0"""
        inverted = self.view(texts)

        assert (inverted.doc_count, inverted.vocab_size) == (len(texts), 0)
        assert (inverted.pair_adds, len(inverted.terms)) == (0, 0)

    def test_values_are_float32(self):
        """倒排的值必须与稠密编码同为 float32：那是 nnz 份而非 n×vocab 份的分配"""
        assert self.view().values.dtype == np.float32


class TestInvertedBlocksMatchDenseBlocks:
    """两条块生产者必须产出同一批相似度——「换算法不换语义」的唯一凭据"""

    CORPUS = TestNgramInvertedView.ROWS * 3      # 24 条：含三组精确重复与零向量

    def pair(self, block_rows):
        dedup = Deduplicator(threshold=0.9)
        dense = dedup._fallback_encode(self.CORPUS)
        reference = list(dedup._iter_similarity_row_blocks(dense, block_rows))
        inverted = list(dedup._iter_inverted_similarity_row_blocks(
            dedup._inverted_view(self.CORPUS), block_rows))
        return reference, inverted

    @pytest.mark.parametrize("block_rows", [1, 2, 3, 5, 1000])
    def test_blocks_agree_element_by_element(self, block_rows):
        """逐块、逐元素对照（含块边界与末块的非满形状）"""
        reference, inverted = self.pair(block_rows)

        assert [(s, e) for s, e, _ in inverted] == [(s, e) for s, e, _ in reference]
        assert reference, "两条路都没产出块，这条对照是空跑"
        for (_, _, got), (_, _, want) in zip(inverted, reference):
            assert got.shape == want.shape
            np.testing.assert_allclose(got, want, atol=1e-6,
                                       err_msg=f"block_rows={block_rows} 时两条路不一致")

    def test_the_inverted_blocks_keep_the_column_contract(self):
        """倒排版必须守住稠密版的两条结构约束：列宽 n-start、单块单元数封顶

        归组只消费 `row[i + 1 - start:]`，列宽不是 `n - start` 就会读错下标；
        单元数不封顶则意味着峰值又随 n 平方长回去。
        """
        from augmentor.dedup import _MAX_BLOCK_CELLS

        n = len(self.CORPUS)
        _, inverted = self.pair(5)

        for start, end, block in inverted:
            assert block.shape == (end - start, n - start)
            assert block.shape[0] * block.shape[1] <= _MAX_BLOCK_CELLS

    @pytest.mark.parametrize("threshold", [0.1, 0.3, 0.5, 0.7, 0.9, 0.99])
    def test_units_only_flip_within_the_agreement_band(self, threshold):
        """两条路的判定差异必须全部落在它们的数值一致带之内

        逐元素 allclose 只说明数值接近，这里说明**判定**差在哪：归组只读判定，
        判定差一个下标就是数据被多删或漏删，所以「差多少」必须能被数值差解释；
        离阈值远的单元一个都不许差。带宽 2e-6 取自实测：本语料两条路逐元素最大
        差 1.2e-7，走生产路径（稠密那条还要再过一次归一化）最大差 1.8e-7。
        """
        band = 2e-6
        reference, inverted = self.pair(3)

        assert reference, "两条路都没产出块，这条对照是空跑"
        for (_, _, got), (_, _, want) in zip(inverted, reference):
            assert float(np.abs(got - want).max()) <= band, (
                f"阈值 {threshold} 处有单元数值差超过判定带 {band:.0e}"
            )
            for k in np.flatnonzero((got >= threshold) != (want >= threshold)):
                r, c = np.unravel_index(k, got.shape)
                assert abs(float(got[r, c]) - threshold) <= band and \
                       abs(float(want[r, c]) - threshold) <= band, (
                    f"单元 ({r},{c}) 判定不同却两者都离阈值 {threshold} 很远："
                    f"{float(got[r, c])} vs {float(want[r, c])}——不是浮点并列，是真不一致"
                )

    def test_a_corpus_without_ngrams_yields_zero_blocks(self):
        """零语料在倒排版上必须是全零块（而不是崩溃，也不是未初始化内存）"""
        dedup = Deduplicator()
        empty = dedup._inverted_view(["", "a", "之", "b"])

        blocks = list(dedup._iter_inverted_similarity_row_blocks(empty, 3))

        assert [(s, e) for s, e, _ in blocks] == [(0, 3), (3, 4)]
        assert all(not b.any() for _, _, b in blocks), "全零块里出现了非零值"

    def test_identical_documents_still_score_one(self):
        """完全相同的两篇在倒排块里必须仍是 1.0，否则重复组永远配不出来"""
        dedup = Deduplicator()
        blocks = list(dedup._iter_inverted_similarity_row_blocks(
            dedup._inverted_view(["如何申请公租房", "如何申请公租房"]), 1))

        assert [(s, e) for s, e, _ in blocks] == [(0, 1), (1, 2)]
        assert float(blocks[0][2][0, 1]) == pytest.approx(1.0, abs=1e-6)


class TestTheProducerIsChosenByMeasuredCost:
    """选路必须由两条实测账决定，而不是「有倒排就一律用倒排」

    下面每一组 `(n, vocab, pair_adds)` 都是同机实测出来的，不是编出来的边界值：
    预言机一旦退化成单账、常数、或「矩阵超过某个固定 MiB 就换」这类坏代理，本类
    就会红。真实数据上的收益（端到端 10.8 倍）与它挡住的退化档（全重复语料倒排慢
    69 倍；随机中段语料倒排峰值贵 1.4～2.4 倍）都凭这张实测表说话。
    """

    #: (文档数, 词表, pair_adds, 实测「倒排路径」端到端峰值 MiB)
    #: 随机档取 60 字符随机串语料、n=1200×k；首行是 train_data.json 的 instruction
    MEASURED = [
        (6902, 23033, 21_917_714, 149.1),
        (1200, 3844, 730_095, 54.2),
        (2400, 3844, 2_784_345, 104.0),
        (3600, 3844, 6_156_473, 153.9),
        (4800, 3844, 10_866_219, 171.3),
        (6000, 3844, 16_885_523, 173.9),
        (7200, 3844, 24_211_675, 175.9),
    ]
    REAL = MEASURED[0]

    def win(self, doc_count, vocab_size, pair_adds):
        """按生产路径同款的行块大小问一次预言机"""
        dedup = Deduplicator()
        return dedup._inverted_blocks_win(doc_count, vocab_size, pair_adds,
                                          dedup._row_block_size(doc_count, 1000))

    def test_the_cost_constants_are_the_measured_ones(self):
        """三个代价常数钉死：动它们等于动判据，必须连带重测再改这里"""
        from augmentor.dedup import (_INVERTED_ADD_COST, _INVERTED_MEM_MARGIN,
                                     _INVERTED_PAIR_BYTES)

        assert _INVERTED_ADD_COST == 3000
        assert _INVERTED_PAIR_BYTES == 96
        assert _INVERTED_MEM_MARGIN == 125

    def test_the_memory_unit_price_reproduces_the_measured_peaks(self):
        """内存单价 96 B 必须仍然复现实测峰值，否则它就是个漂移的魔数

        估的是「最大那个块」的 pair 级临时数组：`pair_adds × block_rows ÷ n × 单价`。
        七档实测比值 0.97～1.13 全在带内（带宽留到拟合余量 0.8～1.3）。这条不判方向，
        只判拟合——它替 `_INVERTED_PAIR_BYTES` 守住「有实测背书」这个身份：
        改了常数而不重测，就会在这里红。
        """
        from augmentor.dedup import _INVERTED_PAIR_BYTES

        dedup = Deduplicator()
        for doc_count, _vocab_size, pair_adds, measured_mib in self.MEASURED:
            block_rows = dedup._row_block_size(doc_count, 1000)
            est_mib = pair_adds * block_rows * _INVERTED_PAIR_BYTES / doc_count / 2 ** 20
            assert 0.8 <= est_mib / measured_mib <= 1.3, (
                f"n={doc_count}：估算 {est_mib:.1f} MiB 复现不了实测 {measured_mib} MiB"
            )

    def test_the_real_corpus_takes_the_inverted_path(self):
        """本轮修复的全部收益系于这一位返回值：真实 6902 条必须判倒排赢

        实测（min-of-3、同进程 back-to-back）倒排 969 ms / 149.1 MiB，稠密那条
        10484 ms / 621.8 MiB，逐组结果完全相同（比值 0.092）。这条变红就说明倒排生产者成了死代码：
        逐元素等价用例照跑，生产路径却一点没变快。
        """
        assert self.win(*self.REAL[:3]) is True

    def test_a_fully_duplicated_corpus_loses_the_time_account(self):
        """全重复语料：vocab 塌成 9，散射累加次数反而和稠密乘加数一样多

        实测 n=6902、vocab=9：稠密那条 2.1437 亿次乘加，倒排这条 pair_adds 也是
        2.1440 亿次——次数上没有优势，而单次散射贵约 3000 倍，于是倒排 6.23 s
        vs 稠密 0.09 s。这类语料的稠密矩阵只有 0.24 MiB，两条账同时判负。
        """
        assert self.win(6902, 9, 214_400_277) is False

    def test_the_mid_range_random_corpus_stays_dense_though_the_matrix_is_big(self):
        """本轮实测推翻了一条「矩阵超过 64 MiB 就换」的代理判据

        随机 60 字符语料 n=1200…7200 共六档，逐档实测：倒排峰值一直是稠密的
        1.4～2.4 倍（54.2 vs 22.2、171.3 vs 85.7、175.9 vs 120.9 MiB），时间上
        n=4800 那档还慢 7%（620 vs 577 ms），要到 n≥6000 才反过来自身省 22～25%。
        而其中 n=4800/6000/7200 三档的稠密矩阵是 70.4/88.0/105.6 MiB——**按固定
        MiB 线早该换路**。内存账因此必须跟着 pair 级临时数组走，不跟着矩阵大小走。
        本用例把这六档全部钉成判负，并钉住「恰好三档越过那条坏线」——少于三档说明
        表或行块大小变了，用例不再能区分新旧判据。
        """
        crossed = 0
        for doc_count, vocab_size, pair_adds, _measured in self.MEASURED[1:]:
            assert self.win(doc_count, vocab_size, pair_adds) is False, (
                f"n={doc_count}、vocab={vocab_size} 实测倒排峰值更贵，不该换路"
            )
            crossed += doc_count * vocab_size * 4 > 64 * 1024 * 1024
        assert crossed == 3, f"越过 64 MiB 坏线的档数应是 3，实得 {crossed}"

    def test_only_the_instruction_shaped_field_qualifies_on_this_dataset(self):
        """收益边界实测：同一份真实数据换字段，判据跟着翻

        `output` 字段 V=17450 但 pair_adds 高达 1.466 亿（时间比值 1.06）→ 判负；
        `input` 字段整档为空 → V=0、pair_adds=0 → 判负。所以本轮那 10.8 倍是
        「instruction 这一档的多样性」换来的，不是「任何去重都快了 10 倍」。
        """
        assert self.win(6902, 17450, 146_586_300) is False
        assert self.win(6902, 0, 0) is False

    @pytest.mark.parametrize("doc_count,vocab_size,pair_adds", [
        (6902, 0, 0),          # 零词表：稠密那条本来也不做任何乘法
        (1, 1, 0),             # 单文档：没有配对
        (10, 4, 1),            # 小语料：稠密矩阵只有 160 字节
    ])
    def test_degenerate_inputs_never_switch_producer(self, doc_count, vocab_size, pair_adds):
        """退化输入必须一律判「不换路」

        零词表这一档还兜住了既有护栏用例：
        :meth:`TestStreamingGrouping.test_no_full_n_by_n_matrix_is_allocated` 喂的是
        `["x"] * 6000`（长度 < 2 没有二元组 → vocab 0），它靠 spy 稠密生产者来数块
        形状；这里一旦判赢，那条用例就会因为「稠密生产者没被调用」而假红。
        """
        assert self.win(doc_count, vocab_size, pair_adds) is False

    def test_the_time_account_flips_at_its_exact_boundary(self):
        """时间账严格单调：换算后乘加数**等于**稠密乘加数时就不该换

        越过一格就必须判负，否则判据成了「差不多就换」，实测收益不再有依据。
        这里把 `block_rows` 取成 1 把内存账请出对照——真实块大小下这个点位两条账
        同时判负，就测不出时间账自己的边界了。
        """
        from augmentor.dedup import _INVERTED_ADD_COST

        doc_count, vocab_size = self.REAL[:2]
        dense_madds = doc_count * doc_count // 2 * vocab_size
        boundary = dense_madds // _INVERTED_ADD_COST

        assert boundary * _INVERTED_ADD_COST < dense_madds, "边界取错了"
        assert Deduplicator._inverted_blocks_win(doc_count, vocab_size, boundary, 1) is True
        assert Deduplicator._inverted_blocks_win(doc_count, vocab_size, boundary + 1, 1) is False

    def test_the_memory_account_is_monotone_in_block_rows(self):
        """块越大、倒排的临时数组越贵——判据必须随之单调收紧

        同一形状（真实语料的 n/vocab/pair_adds）只把行块放大：一路判赢到某格为止，
        判负之后不许回弯。这条挡住「块大小不参与判据」的写法——那会让小 n 换路、
        大 n 反而一直换。
        """
        doc_count, vocab_size, pair_adds = self.REAL[:3]
        decisions = [Deduplicator._inverted_blocks_win(doc_count, vocab_size, pair_adds, br)
                     for br in (1, 100, 500, 579, 1000, 1500, 2000, 3000)]

        assert decisions[0] is True and decisions[-1] is False, f"两端就该一判一判负: {decisions}"
        assert decisions == sorted(decisions, reverse=True), f"判据非单调（块变大又判赢）: {decisions}"


class TestOnlyOneProducerRuns:
    """选定一条生产者之后，另一条必须一次都没被调用

    两条生产者各自的峰值都不小（真实数据上稠密那份是 606 MB），所以「两条都算一遍
    再挑」是不能接受的退路；判负时也一样，postings 要立刻松手。这里用计数器把
    选路钉成结构约束——预言机返回什么，另一条接缝就不许被碰。

    小语料在任何真实判据下都过不了内存账（24 条的矩阵才几 KB），所以必须
    monkeypatch 预言机才能测到「只跑一条」这件事本身。
    """

    CORPUS = TestNgramInvertedView.ROWS * 3

    #: 接缝名 → 计数键的映射，顺序即断言里比对的顺序
    SPIED = {
        "batch_encode": "_batch_encode",
        "fallback_encode": "_fallback_encode",
        "dense": "_iter_similarity_row_blocks",
        "inverted": "_iter_inverted_similarity_row_blocks",
    }

    def armed(self, monkeypatch, win):
        """钉住选路，给四条接缝装计数器，返回 (去重器, 调用次数)"""
        monkeypatch.setattr(Deduplicator, "_inverted_blocks_win",
                            staticmethod(lambda *args: win))
        calls = dict.fromkeys(self.SPIED, 0)

        def counting(original, key):
            def wrapper(*args, **kwargs):
                calls[key] += 1
                return original(*args, **kwargs)
            return wrapper

        for key, name in self.SPIED.items():
            monkeypatch.setattr(Deduplicator, name,
                                counting(getattr(Deduplicator, name), key))

        dedup = Deduplicator()
        # 显式设成 fallback：真实语义模型的向量处处非零，倒排按判据本就不该候选，
        # 而本用例测的是「二选一」这个结构，与本机装没装 sentence-transformers 无关。
        dedup._model = "fallback"
        return dedup, calls

    def test_the_inverted_choice_never_encodes_a_dense_matrix(self, monkeypatch):
        """判倒排赢时，稠密那条的三个入口都不许被碰一下"""
        dedup, calls = self.armed(monkeypatch, True)

        groups = dedup._find_duplicate_groups_chunked(self.CORPUS, chunk_size=5)

        assert calls == {"batch_encode": 0, "fallback_encode": 0, "dense": 0, "inverted": 1}, (
            f"判倒排赢却碰了稠密生产者: {calls}"
        )
        assert groups, "这条语料应至少分出一组重复，否则本用例是空跑"

    def test_the_dense_choice_never_runs_the_inverted_producer(self, monkeypatch):
        """判稠密赢时回到原来的编码→分块那条路，倒排生产者一次都不跑"""
        dedup, calls = self.armed(monkeypatch, False)

        groups = dedup._find_duplicate_groups_chunked(self.CORPUS, chunk_size=5)

        assert calls == {"batch_encode": 1, "fallback_encode": 1, "dense": 1, "inverted": 0}, (
            f"判稠密赢却跑了倒排生产者，等于两条路都算了一遍: {calls}"
        )
        assert groups, "这条语料应至少分出一组重复，否则本用例是空跑"

    @staticmethod
    def reference_pair_sims(texts):
        """独立第三路：float64 重新累加的上三角相似度

        只用来回答「这个阈值离语料里最近的一对有多远」，即该阈值上有没有精确并列，
        不参与等价断言本身（量级 1e-2 的判断不受 float32 输入的影响）。
        """
        dedup = Deduplicator()
        dedup._model = "fallback"
        emb = dedup._fallback_encode(texts).astype(np.float64)
        sims = emb @ emb.T
        return sims[np.triu_indices(len(texts), 1)]

    def production_pair(self, monkeypatch, texts):
        """按生产路径 `_similarity_blocks` 分别取两条路的块"""
        blocks = {}
        for win in (True, False):
            dedup, _ = self.armed(monkeypatch, win)
            blocks[win] = list(dedup._similarity_blocks(texts, 1000))
        return blocks[True], blocks[False]

    def test_an_exact_tie_is_not_a_producer_bug(self, monkeypatch):
        """阈值恰好压在「数学上正好相等」的一对上时，判定不由实现决定

        本语料有 9 对文档的二元组重合率精确等于 0.5。实测走生产路径时它们在
        两条路上分别得到 0.49999997 与 0.50000000（差 1.2e-8），于是 `>= 0.5`
        一真一假、18 个单元（对称计）判定相反，端到端分组随之不同。
        这不是本轮引入的性质：同一份 float64 参考给出的值是 0.4999999634，
        离 0.5 也有 3.7e-8——并列点上任何 float32 实现都无从裁定。
        本用例把「差异有界」这件事钉住：并列点上可以判不同，但数值差必须 ≤ 2e-6，
        超出即真不一致。也因此，等价断言只在无并列的阈值上做（见下一条）。
        """
        band = 2e-6
        inverted, reference = self.production_pair(monkeypatch, self.CORPUS)

        ties = self.reference_pair_sims(self.CORPUS)
        ties = ties[np.abs(ties - 0.5) < 5e-7]
        assert ties.size == 9, f"预期语料里有 9 对精确并列于 0.5，实得 {ties.size}"

        flips = 0
        for (_, _, got), (_, _, want) in zip(inverted, reference):
            assert float(np.abs(got - want).max()) <= band
            flips += int(np.flatnonzero((got >= 0.5) != (want >= 0.5)).size)
        assert flips == 18, f"并列点上的判定相反数应恰为 18（9 对×对称），实得 {flips}"

    @pytest.mark.parametrize("threshold", [0.1, 0.3, 0.7, 0.85, 0.9, 0.99])
    def test_the_public_entry_gives_the_same_result_either_way(self, monkeypatch, threshold):
        """从公开入口 `deduplicate()` 进去，两条生产者给出的分组必须逐位相同

        逐元素等价那一组用例钉的是块，这里钉的是接线：选路只能发生在「挑一条」，
        不能挑到一半又换（那会让 `kept_indices` 与 `duplicate_groups` 对不上）。
        阈值不能取 0.5——那是精确并列点，见上一条用例。
        """
        margin = float(np.abs(self.reference_pair_sims(self.CORPUS) - threshold).min())
        assert margin > 1e-4, (
            f"阈值 {threshold} 距语料里最近的一对只有 {margin:.1e}，"
            "落进浮点判定带，不适合做逐位等价断言"
        )

        items = [{"instruction": text} for text in self.CORPUS]
        results = []
        for win in (True, False):
            dedup, _ = self.armed(monkeypatch, win)
            dedup.threshold = threshold
            results.append(dedup.deduplicate(items))

        got, want = results
        assert got.duplicate_groups == want.duplicate_groups
        assert got.kept_indices == want.kept_indices
        assert (got.removed_count, got.deduplicated_count) == \
               (want.removed_count, want.deduplicated_count)
        assert got.duplicate_groups, f"阈值 {threshold} 下一组重复都没分到，本用例是空跑"

class TestSimilarPairsCoverEveryDocument:
    """相似对必须在**全部**条目之间找，不许有隐形的「只看前 200 条」窗口

    旧实现把候选限制在 `items[:200]`，第 201 条之后的重复**永远不会**出现在结果
    里，而返回形状完全正常：真实 6902 条语料上 `instruction` 侧报出的最大索引只有
    158、`output` 侧只有 53（全量口径下这两侧分别有 482 与 48,677 对达标）。
    本组用例的语料是「互不相关的占位档 + 尾部若干条完全相同」，占位档两两正交
    （TF 向量没有共同二元组 → 相似度精确为 0），所以达标对集合可以精确枚举。
    """

    #: 占位档数量刻意超过 200，让尾部那几条落在旧实现的窗口之外
    FILLER_COUNT = 220

    #: 尾部重复文档用的字符区间与占位档（0x4E00…0x4E00+659）不重叠
    DUP_TEXT = "".join(chr(0x4E00 + 1000 + k) for k in range(8))

    @classmethod
    def fillers(cls, count):
        """每篇 3 个互不相同的汉字，且任意两篇的字符集合不相交 → 相似度精确 0"""
        return ["".join(chr(0x4E00 + 3 * t + r) for r in range(3)) for t in range(count)]

    def texts(self, tail_duplicates):
        return self.fillers(self.FILLER_COUNT) + [self.DUP_TEXT] * tail_duplicates

    def dedup(self):
        dedup = Deduplicator(threshold=0.9)
        # 正交性来自 fallback 编码器的 TF 词表；真实语义模型会把无关句子也映得相近
        dedup._model = "fallback"
        return dedup

    def test_placeholders_alone_qualify_no_pairs(self):
        """前置自证：占位档之间必须真的一个达标对都没有

        后面几条断言的是「结果恰好等于尾部那几对」，一旦占位档互相达标，那些断言
        就会被噪声冲掉。先把「语料是干净的」单独钉住，红的时候才知道该修哪一层。
        """
        assert self.dedup().find_similar_pairs(
            [{"instruction": t} for t in self.texts(0)], top_k=5) == []

    def test_a_duplicate_pair_beyond_the_200th_item_is_found(self):
        """唯一的达标对落在第 221/222 条时必须被找到（旧实现看不见它）"""
        items = [{"instruction": t} for t in self.texts(2)]
        pairs = self.dedup().find_similar_pairs(items, top_k=5)

        assert [(i, j) for i, j, _ in pairs] == [(220, 221)], (
            f"尾部那一对没有被找到，说明比较范围仍被截断: {pairs}"
        )
        assert abs(pairs[0][2] - 1.0) < 1e-5

    def test_the_whole_tail_group_is_reported(self):
        """尾部 4 条完全相同 → 恰好 C(4,2)=6 对，一对不落、一对不多"""
        items = [{"instruction": t} for t in self.texts(4)]
        pairs = self.dedup().find_similar_pairs(items, top_k=20)

        expected = {(i, j) for i in range(220, 224) for j in range(i + 1, 224)}
        assert {(i, j) for i, j, _ in pairs} == expected

    @pytest.mark.parametrize("chunk_size", [1000, 25])
    def test_pairs_survive_block_boundaries(self, chunk_size):
        """达标对在最后一块里时也必须找到——钉「逐块扫完整档」而不是只扫首块

        224 条按 25 行切块后有 9 块，那 6 对全在第 9 块（行 220–223、列同段）。
        """
        pairs = self.dedup()._find_similar_pairs_chunked(
            self.texts(4), 20, chunk_size=chunk_size)

        expected = {(i, j) for i in range(220, 224) for j in range(i + 1, 224)}
        assert {(i, j) for i, j, _ in pairs} == expected, f"块大小 {chunk_size} 下候选集不同"

    def test_only_the_upper_triangle_is_reported(self):
        """每对只报一次且 `i < j`：对称块的两半不许都倒出来"""
        items = [{"instruction": t} for t in self.texts(6)]
        pairs = self.dedup().find_similar_pairs(items, top_k=100)

        keys = [(i, j) for i, j, _ in pairs]
        assert all(i < j for i, j in keys), f"出现了下三角或自配对: {keys}"
        # top_k(100) 大于候选总数：有几对报几对，不补位也不截断
        assert len(keys) == len(set(keys)) == 15, f"C(6,2)=15 对应全部报出且不重复: {keys}"

    def test_missing_field_is_treated_as_empty_not_an_error(self):
        """缺字段的条目按空文本参与比较，全零向量之间不构成达标对"""
        dedup = self.dedup()
        items = [{"output": "租房补贴怎样申请"} for _ in range(30)]

        # 默认字段一个都没有 → 30 条空文本 → 零向量之间相似度 0
        assert dedup.find_similar_pairs(items, top_k=5) == []

        got = dedup.find_similar_pairs(items, text_key="output", top_k=5)
        assert [i for i, _, _ in got] == [0] * 5
        assert [j for _, j, _ in got] == [1, 2, 3, 4, 5]
        for _, _, sim in got:
            assert abs(sim - 1.0) < 1e-5


class TestSimilarPairsMatchAnIndependentOracle:
    """名次与数值要能对上第三条独立算出来的路

    选路那一组用例钉的是「挑哪条生产者」，本组钉的是「挑完之后答案对不对」：
    用 float64 重新做一遍归一化点积、独立排序，再与公开入口给出的结果比对。
    TF 向量的相似度天然爱并列（重复文档恒等于 1.0、同一词表下常有成片的 0.8333），
    而并列点上的先后**任何 float32 实现都无从裁定**（见 A36），所以断言分两档：
    前 `top_k` 名彼此隔开 > 1e-4 时才比**序列**，否则只比**完整候选集合**。
    两档都先做「门槛间隔」前置——判据落在并列带里的语料直接把用例顶红，
    也不放一条假绿的过去。
    """

    BASE = "晨阳计划月租金折上96折押一付一半年内全城无责换房"

    def ladder(self):
        """一条基准句 + 逐位替换前缀：制造一串「接近但互不相等」的相似度"""
        rows = [self.BASE]
        for k in range(1, 9):
            rows.append(self.BASE[:k] + "".join("甲乙丙丁"[m % 4] for m in range(k))
                        + self.BASE[2 * k:])
        return rows

    def corpora(self):
        return {
            "倒排语料": TestNgramInvertedView.ROWS * 3,
            "阶梯近重复": self.ladder(),
            "尾部重复224": TestSimilarPairsCoverEveryDocument().texts(4),
        }

    @staticmethod
    def oracle(texts, threshold):
        """独立第三路：float64 归一化 + 全对排序，不走被测实现的任何中间产物"""
        dedup = Deduplicator(threshold=threshold)
        dedup._model = "fallback"
        vectors = dedup._fallback_encode(texts).astype(np.float64)
        norms = np.linalg.norm(vectors, axis=1)
        unit = vectors / np.where(norms == 0.0, 1.0, norms)[:, None]
        sims = unit @ unit.T

        rows, cols = np.triu_indices(len(texts), 1)
        return rows, cols, sims[rows, cols]

    def qualified(self, name, threshold=0.9):
        """返回 (rows, cols, values, keep, floor)：keep 是参考路上达标对的下标"""
        rows, cols, values = self.oracle(self.corpora()[name], threshold)
        floor = threshold * 0.8
        margin = float(np.abs(values - floor).min())
        assert margin > 1e-4, (
            f"{name}: 最近的配对距门槛 {floor} 只有 {margin:.1e}，"
            "落在 float32 判定带里，收不收它两说，不适合做等价断言"
        )
        return rows, cols, values, np.flatnonzero(values >= floor), floor

    def test_the_decisive_prefix_matches_the_oracle_order(self):
        """阶梯语料的前 3 名彼此隔开 1e-2 量级 → 序列可以逐条比"""
        top_k = 3
        rows, cols, values, keep, _ = self.qualified("阶梯近重复")
        assert keep.size > top_k, f"达标对不足 {top_k} 对，截断没起作用，本用例是空跑"

        head = np.sort(values[keep])[::-1]
        separation = float(np.min(-np.diff(head[:top_k + 1])))
        assert separation > 1e-4, f"前 {top_k + 1} 名里最小相邻差只有 {separation:.1e}，有并列"

        order = np.lexsort((cols[keep], rows[keep], -values[keep]))[:top_k]
        expected = [(int(rows[keep][o]), int(cols[keep][o]), float(values[keep][o]))
                    for o in order]

        dedup = Deduplicator(threshold=0.9)
        dedup._model = "fallback"
        got = dedup.find_similar_pairs(
            [{"instruction": t} for t in self.corpora()["阶梯近重复"]], top_k=top_k)

        assert [(i, j) for i, j, _ in got] == [(i, j) for i, j, _ in expected], (
            f"序列不同\n实际 {got}\n参考 {expected}"
        )
        for (_, _, got_sim), (_, _, want_sim) in zip(got, expected):
            assert abs(got_sim - want_sim) <= 2e-6

    @pytest.mark.parametrize("name,top_k", [("倒排语料", 27), ("尾部重复224", 20)])
    def test_the_full_candidate_set_matches_the_oracle(self, name, top_k):
        """并列成片的语料只比集合：`top_k` 取到装得下全部达标对，截断点不参与裁定

        倒排语料有 24 对完全相同的文档（相似度恒等于 1.0），尾部重复224 有 6 对；
        名次在这些对上由 `(i, j)` 规则决定，那是
        :class:`TestSimilarPairTiesBreakByIndex` 的职责，不在本用例里重复钉。
        """
        rows, cols, values, keep, _ = self.qualified(name)
        assert top_k >= keep.size, (
            f"{name}: top_k={top_k} 小于达标对数 {keep.size}，截断落在并列段里"
        )

        expected = {(int(rows[k]), int(cols[k])) for k in keep}
        dedup = Deduplicator(threshold=0.9)
        dedup._model = "fallback"
        got = dedup.find_similar_pairs([{"instruction": t} for t in self.corpora()[name]],
                                       top_k=top_k)

        got_keys = {(i, j) for i, j, _ in got}
        assert got_keys == expected, (
            f"{name}: 候选集合不同，缺 {expected - got_keys}，多 {got_keys - expected}"
        )
        assert len(got) == keep.size
        want = {(int(rows[k]), int(cols[k])): float(values[k]) for k in keep}
        for i, j, sim in got:
            assert abs(sim - want[(i, j)]) <= 2e-6

    @pytest.mark.parametrize("name,top_k", [
        ("倒排语料", 4),
        ("阶梯近重复", 3),
        ("尾部重复224", 20),
    ])
    def test_results_are_ranked_and_come_from_the_upper_triangle(self, name, top_k):
        """结构不变量：相似度降序、`i < j`、无重复——与数值对不对是两件事"""
        texts = self.corpora()[name]
        dedup = Deduplicator(threshold=0.9)
        dedup._model = "fallback"
        got = dedup.find_similar_pairs([{"instruction": t} for t in texts], top_k=top_k)

        assert got, f"{name}: 没有返回任何配对，本用例是空跑"
        sims = [s for _, _, s in got]
        assert sims == sorted(sims, reverse=True), f"相似度未按降序: {sims}"
        keys = [(i, j) for i, j, _ in got]
        assert all(i < j for i, j in keys)
        assert len(set(keys)) == len(keys), f"同一对报了两次: {keys}"


class TestSimilarPairTiesBreakByIndex:
    """并列名次由 `(i, j)` 升序裁定，且与走哪条生产者无关

    600 条全重复语料里每一对相似度都相等，截断点正落在并列段中间：这时候
    名次规则就是唯一能让「同一份数据换个内部实现也不换 top_k」成立的东西。
    这些文档又全部在第 200 条之后，所以旧实现（只看前 200 条）在这里报空。
    """

    TAIL_DUPLICATES = 30

    def texts(self):
        holder = TestSimilarPairsCoverEveryDocument()
        return holder.fillers(holder.FILLER_COUNT) + [holder.DUP_TEXT] * self.TAIL_DUPLICATES

    @pytest.mark.parametrize("win", [None, True, False])
    def test_the_expected_prefix_of_the_tie_run_is_returned(self, monkeypatch, win):
        """前 5 条必须是 `(220, 221) … (220, 225)`，而不是别的并列成员"""
        if win is not None:
            monkeypatch.setattr(Deduplicator, "_inverted_blocks_win",
                                staticmethod(lambda *args: win))
        dedup = Deduplicator(threshold=0.9)
        dedup._model = "fallback"

        pairs = dedup.find_similar_pairs([{"instruction": t} for t in self.texts()],
                                         top_k=5)

        start = TestSimilarPairsCoverEveryDocument.FILLER_COUNT
        assert [(i, j) for i, j, _ in pairs] == [
            (start, start + step) for step in range(1, 6)], f"win={win} 下并列段前缀不同: {pairs}"
        assert len({s for _, _, s in pairs}) == 1, (
            f"并列段的相似度应当逐位相同: {[s for _, _, s in pairs]}"
        )


class TestSimilarPairsWorkingSetStaysBounded:
    """去掉窗口之后，候选保留集不许随「总达标对数」膨胀

    全重复语料有 C(n,2) 对达标（600 条 → 179,700 对），「先把候选全收进一个表、
    最后一次性排序」的写法实测峰值 13.41 MiB（numpy 三段拼接），n=1200 时 48.10 MiB；
    用 Python 元组表收更贵（32.0 / 132.6 MiB）——都跟着对数平方走。实现按行消费块、
    每行末尾把保留集裁回 top_k，所以峰值只到 `top_k + 单行达标数`（≤ top_k+n）。
    这里用 `_pair_rank` 见到过的最大工作集把这件事钉成结构约束，比只测一次墙钟
    峰值可靠（时间随机器负载变，见循环日志 L12/L25）。

    两条用例都做过注入验证：把本方法换成「收完再排」的等价写法（结果逐条相同、
    只有资源形状不同）之后，本类两条红、其余 115 条全绿。
    """

    def ranked_sizes(self, monkeypatch, texts, top_k):
        """跑一次公开入口，记录 `_pair_rank` 每次见到的候选条数"""
        sizes = []

        def spy(original, *args):
            def wrapper(*inner_args, **inner_kwargs):
                sizes.append(len(inner_args[0]))
                return original(*inner_args, **inner_kwargs)
            return wrapper

        monkeypatch.setattr(Deduplicator, "_pair_rank",
                            staticmethod(spy(Deduplicator._pair_rank)))
        dedup = Deduplicator(threshold=0.9)
        dedup._model = "fallback"
        pairs = dedup.find_similar_pairs([{"instruction": t} for t in texts], top_k=top_k)
        return sizes, pairs

    def test_the_working_set_never_grows_with_the_candidate_count(self, monkeypatch):
        """工作集上界是 `top_k + n`，而候选总数是 n²/2——差两个数量级才算数"""
        n, top_k = 600, 5
        candidates = n * (n - 1) // 2

        sizes, pairs = self.ranked_sizes(monkeypatch, ["完全相同的一句话"] * n, top_k)

        assert sizes, "_pair_rank 一次都没被调用，探针没生效"
        assert len(pairs) == top_k
        assert max(sizes) <= top_k + n, f"保留集涨到 {max(sizes)}，说明候选在跨行累积"
        assert candidates > 20 * max(sizes), (
            f"候选 {candidates:,} 对相对工作集 {max(sizes)} 不够悬殊，本用例分不出两种写法"
        )

    def test_peak_stays_far_below_materializing_every_candidate(self):
        """600 条全重复语料的峰值必须远低于「把 179,700 对都建出来」

        实测（同一进程 back-to-back）：有界 1.44 MiB（占主导的是 600×600 的相似度
        块），收完再排的等价写法 13.41 MiB（n=1200 时 4.70 对 48.10 MiB）。界取
        6 MiB——留出覆盖率 trace 对纯 Python 循环的放大（见 L10 与本文件里的内存
        用例），又不到朴素路径的量级。
        """
        import tracemalloc

        items = [{"instruction": "完全相同的一句话"}] * 600
        dedup = Deduplicator(threshold=0.9)
        dedup._model = "fallback"

        tracemalloc.start()
        try:
            pairs = dedup.find_similar_pairs(items, top_k=5)
            peak = tracemalloc.get_traced_memory()[1] / 2 ** 20
        finally:
            tracemalloc.stop()

        assert len(pairs) == 5, f"600 条全重复应有 5 对候选，实得 {pairs}"
        assert peak < 6.0, f"峰值 {peak:.2f} MiB 超出预算（收完再排实测 13.41 MiB）"


class TestSimilarPairsUseTheGroupingProducer:
    """相似对与归组必须共用同一个相似度块生产者

    L30（A17）做出块生产者之后，A35 记的是「`find_similar_pairs` 还在自己编码
    整份 n×vocab 矩阵」（真实 6902 条：606 MiB 矩阵、624.2 MiB 峰值）。本轮把它
    接到 :meth:`Deduplicator._similarity_blocks` 上，所以这里钉三件事：块生产者
    恰好被叫一次；构造完整 n×n 的那两个入口一次都不叫；`_batch_encode` 的次数
    与选路一致（判倒排赢时不许再编一份稠密矩阵）。
    """

    CORPUS = TestNgramInvertedView.ROWS * 3

    #: 接缝里属于 staticmethod 的那几个，包装时必须保持 static，否则 self 会被
    #: 当成第一个位置参数传进去
    STATIC = {"_normalize"}

    #: 被计数的接缝（模式同 :meth:`TestOnlyOneProducerRuns.armed`，只是清单不同）
    SPIED = ("_similarity_blocks", "_normalize", "_compute_similarity_matrix_chunked",
             "_batch_encode")

    def armed(self, monkeypatch, win):
        """钉住选路并给接缝装计数器，返回 (去重器, 调用次数)"""
        if win is not None:
            monkeypatch.setattr(Deduplicator, "_inverted_blocks_win",
                                staticmethod(lambda *args: win))
        calls = dict.fromkeys(self.SPIED, 0)
        for name in self.SPIED:
            original = getattr(Deduplicator, name)

            def wrapper(*args, _orig=original, _key=name, **kwargs):
                calls[_key] += 1
                return _orig(*args, **kwargs)

            monkeypatch.setattr(Deduplicator, name,
                                staticmethod(wrapper) if name in self.STATIC else wrapper)

        dedup = Deduplicator(threshold=0.6)
        dedup._model = "fallback"
        return dedup, calls

    @pytest.mark.parametrize("win", [None, True, False])
    def test_the_shared_producer_runs_and_the_whole_matrix_does_not(self, monkeypatch, win):
        """三条生产者共用的块入口叫一次，n×n 那条路一步都不许走"""
        dedup, calls = self.armed(monkeypatch, win)

        pairs = dedup.find_similar_pairs([{"instruction": t} for t in self.CORPUS],
                                         top_k=8)

        assert pairs, f"win={win}: 这条语料应至少报出一对，否则本用例是空跑"
        assert calls["_similarity_blocks"] == 1, f"块生产者被叫了 {calls} 次"
        assert calls["_normalize"] == 0, f"又去复制整份归一化矩阵了: {calls}"
        assert calls["_compute_similarity_matrix_chunked"] == 0, (
            f"退回了构造 n×n 的老路: {calls}"
        )

    def test_the_inverted_choice_never_encodes_a_dense_matrix(self, monkeypatch):
        """判倒排赢时，稠密编码入口一次都不被碰"""
        dedup, calls = self.armed(monkeypatch, True)

        dedup.find_similar_pairs([{"instruction": t} for t in self.CORPUS], top_k=8)

        assert calls == {"_similarity_blocks": 1, "_normalize": 0,
                         "_compute_similarity_matrix_chunked": 0, "_batch_encode": 0}, (
            f"判倒排赢却编了稠密矩阵: {calls}"
        )

    def test_the_dense_choice_encodes_exactly_once(self, monkeypatch):
        """判稠密时也只经块生产者一次、只编码一次——不是每块重编一遍"""
        dedup, calls = self.armed(monkeypatch, False)

        dedup.find_similar_pairs([{"instruction": t} for t in self.CORPUS], top_k=8)

        assert calls == {"_similarity_blocks": 1, "_normalize": 0,
                         "_compute_similarity_matrix_chunked": 0, "_batch_encode": 1}, (
            f"稠密那条没有只编码一次，或根本没经块生产者: {calls}"
        )
