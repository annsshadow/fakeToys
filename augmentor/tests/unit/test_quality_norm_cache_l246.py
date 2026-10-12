# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

r"""L246 守卫：A327 —— 多样性向量分支的行范数随向量同批刷新，不逐候选重算

**为什么立这一条**：`_calculate_diversity` 的向量分支原先把
`np.linalg.norm(_existing_embeddings, axis=1)` 写在「每候选一次」的那条除法里，于是
n 条候选就把 sample_size×dim 的平方和付 n 遍。L246 改前现量（仓外探针
FakeModel 顶掉 sentence-transformers，n=4000，`include_semantic=False`，相关性走
n-gram fallback）：

| 档 | 窗口 / 维度 | 改前 axis=1 次数 | 改后 | 单次 wall 改前→改后 |
|----|-------------|------------------|------|--------------------|
| 参照集稳定 | 30 / 64 | 4000 | **1** | 407.7 → 376.6 ms（−7.6%） |
| 参照集逐条达标 | 30 / 64 | 4000 | **1** | 406.8 → 359.8 ms（−11.6%） |
| 生产第一批（参照从 1 涨到窗口） | 30 / 64 | 4000 | **30** | 430.4 → 404.8 ms（−6.0%） |
| 真实语义模型量级 | 100 / 384 | 4000 | **1** | 459.4 → 396.6 ms（−13.7%） |

wall 是单次读数（含解释器噪声），**计数那一列是精确的**：改后 axis=1 的次数恒等于
「参照切片的编码次数」，改前恒等于候选数。四档的判决产物（每条样本的三个分与两个
布尔量，共 4×4000 格，`repr` 落盘）**逐字节相同**（sha256 前后同为
`798206eed79354a5…`），本文件的 `test_cached_path_reproduces_the_inline_formula_bit_for_bit`
与 `test_warm_cache_matches_a_cold_scorer_per_candidate` 把这条位相同收进仓内，不靠仓外探针。

**为什么没有守卫会出声**：数值一路正确、缓存命中与否是纯内部事实，「有人把范数挪回
那条除法里」在本仓今天仍然是静默通过的 —— 所以这里钉的是**次数**而不是数值。

**口径**：
1. 两格缓存的寿命**不同轴**，本轮把它们分清楚（A327 原格点名要的正是这句话）：
   `_embedding_cache` 是**实例级、跨 `batch_score` 调用**的整批向量缓存，键为
   `(batch_key, tuple(文本))`，`reset_cache()` 不清它；`_existing_embeddings` 与它的
   行范数是**参照集那一族**，失效判据是参照切片本身，`reset_cache()` 同清两格。
   行范数不另设判据：它是那份矩阵的纯函数，只随那一次编码同处刷新。
   `test_the_counter_is_not_vacuous_when_the_window_churns` 现场证明这条边界跟着判据走，
   顺带证明计数器不是恒 1 的空尺子。
2. 逐候选那个**向量自身**的范数（`axis=None`）仍在，它是每条候选的合法成本，不许被
   一起省掉 ⇒ `test_…` 同时钉 `other == 候选数`。
3. 向量矩阵这一格的注入接缝从「填一格」变成「填两格」：只喂 `_existing_embeddings`
   而不喂 `_existing_norms` 时**当场抛**，不静默用一份和矩阵不同源的范数算出一个数。
"""

import zlib

import numpy as np
import pytest

from augmentor.quality import QualityScorer


def _vec(text, dim=8):
    """由文本唯一决定的伪向量：同一文本永远同一向量，跨 scorer、跨进程可比"""
    rng = np.random.RandomState(zlib.crc32(text.encode("utf-8")) % (2 ** 31))
    return rng.rand(dim).astype(np.float32) + 0.1


class FakeModel:
    """顶掉 sentence-transformers，记录每次 encode 的条数供计数判据对照"""

    def __init__(self, dim=8):
        self.dim = dim
        self.sizes = []

    def encode(self, texts, **kwargs):
        self.sizes.append(len(texts))
        return np.array([_vec(t, self.dim) for t in texts], dtype=np.float32)


def _count_norms(monkeypatch):
    """按 axis 分桶数 `np.linalg.norm`：`axis1` 是被缓存的那一路，`other` 是逐候选的向量范数"""
    stats = {"axis1": 0, "other": 0}
    orig = np.linalg.norm

    def wrapper(x, *a, **kw):
        if kw.get("axis") == 1 or (a and a[0] == 1):
            stats["axis1"] += 1
        else:
            stats["other"] += 1
        return orig(x, *a, **kw)

    monkeypatch.setattr(np.linalg, "norm", wrapper)
    return stats


def _scorer(threshold, sample_size):
    scorer = QualityScorer(threshold=threshold, diversity_sample_size=sample_size)
    # 相关性走 n-gram：本文件只管向量侧的行范数，不把 cross-encoder 拖进来
    scorer._cross_encoder = "fallback"
    scorer._model = FakeModel()
    return scorer


def _refs(n):
    return ["参照文本 %d 号，长度足够切出 n-gram" % i for i in range(n)]


def _items(n):
    return [{"original": "问题 %d" % (i % 7),
             "generated": "回答 %d 的内容，这一段要够长才能彼此不同" % i,
             "output": "输出 %d" % (i % 5)} for i in range(n)]


class TestRowNormIsComputedOnce:
    """计数判据：axis=1 的次数 == 参照切片的编码次数，而不是候选数"""

    def test_full_window_recomputes_row_norms_once_for_the_whole_batch(self, monkeypatch):
        scorer = _scorer(0.999, 8)
        stats = _count_norms(monkeypatch)
        scores = scorer.batch_score(_items(50), existing_generated=_refs(8),
                                    include_semantic=False)

        assert len(scores) == 50
        # 改前这里是 50：每候选把 8×dim 的平方和重算一遍
        assert stats["axis1"] == 1, stats
        # 该省的省、不该省的一格不许动：逐候选的向量自身范数照旧付 50 次
        assert stats["other"] == 50, stats
        # 整批生成向量惰性编码 1 次 + 参照切片编码 1 次
        assert scorer._model.sizes == [50, 8], scorer._model.sizes

    def test_norm_refresh_count_equals_reencode_count_while_window_grows(self, monkeypatch):
        """生产第一批的形状：参照集从 2 条逐条涨到窗口上限，每涨一次才重算一次范数"""
        scorer = _scorer(0.0, 6)
        refs = _refs(2)
        stats = _count_norms(monkeypatch)
        scorer.batch_score(_items(20), existing_generated=refs,
                           include_semantic=False)

        # 窗口 2→3→4→5→6 各编一次码，到上限后不再重编
        assert scorer._model.sizes[1:] == [2, 3, 4, 5, 6], scorer._model.sizes
        assert stats["axis1"] == len(scorer._model.sizes) - 1 == 5, stats
        assert stats["axis1"] < 20, stats


class TestBitIdentity:
    """缓存不许改变任何一个分"""

    def test_cached_path_reproduces_the_inline_formula_bit_for_bit(self):
        """位相同钉在「改前那条式子」本身：照旧式现算一遍，与走缓存的产物逐位比"""
        refs = _refs(4)
        items = _items(1)
        scorer = _scorer(0.999, 4)
        diversity = scorer.batch_score(items, existing_generated=refs,
                                       include_semantic=False)[0].diversity

        matrix = np.array([_vec(t) for t in refs], dtype=np.float32)
        vector = _vec(items[0]["generated"])
        old_shape = np.dot(matrix, vector) / (
            np.linalg.norm(matrix, axis=1) * np.linalg.norm(vector)
        )
        expected = float(max(0.0, 1.0 - float(np.max(old_shape))))
        assert repr(diversity) == repr(expected), (diversity, expected)

    def test_warm_cache_matches_a_cold_scorer_per_candidate(self):
        """整批走缓存的产物，与「每候选换新 scorer」（改前每候选必然重算的形状）逐位相同"""
        refs = _refs(8)
        items = _items(30)
        warm = _scorer(0.0, 8).batch_score(items, existing_generated=refs,
                                           include_semantic=False)
        cold = []
        for item in items:
            fresh = _scorer(0.0, 8)
            cold.append(fresh._calculate_diversity(
                item["generated"], refs, _vec(item["generated"])))
        assert [repr(s.diversity) for s in warm] == [repr(d) for d in cold]


class TestCacheLifetime:
    """失效边界：随向量同生同死，`reset_cache` 同清两格"""

    def test_reset_cache_clears_norms_along_with_embeddings(self, monkeypatch):
        scorer = _scorer(0.999, 4)
        first = scorer.batch_score(_items(30), existing_generated=_refs(4),
                                   include_semantic=False)
        assert len(scorer._existing_norms) == 4
        assert scorer._existing_norms.tolist() == np.linalg.norm(
            scorer._existing_embeddings, axis=1).tolist()

        scorer.reset_cache()
        assert scorer._existing_embeddings == []
        assert scorer._existing_norms == []
        assert scorer._existing_texts == []

        # 计数器在重置之后才装：这一趟必须自己重编码、重算一次范数（两格同寿命）
        stats = _count_norms(monkeypatch)
        second = scorer.batch_score(_items(30), existing_generated=_refs(4),
                                    include_semantic=False)
        # [30, 4]（第一趟）+ [4]（第二趟）：`reset_cache` 清的是参照集那一族，整批生成
        # 向量走 `_embedding_cache`（键含全文元组，实例级、`reset_cache` 不清）⇒ 第二趟
        # 命中不再重编。参照集与它的行范数同批刷新，正是本文件要钉的那条边界。
        assert scorer._model.sizes == [30, 4, 4], scorer._model.sizes
        assert stats["axis1"] == 1, stats
        assert [repr(s.diversity) for s in first] == [repr(s.diversity) for s in second]

    def test_the_counter_is_not_vacuous_when_the_window_churns(self, monkeypatch):
        """判据每轮不命中 ⇒ 每轮重编码 ⇒ 每轮重算范数：计数器跟着失效边界走，不是恒 1

        这一支同时是计数判据的注入取证：把「缓存命中」摘掉，axis=1 的读数必须回到
        候选数（= 改前的形状），否则 `== 1` 那条断言就是空尺子。
        """
        scorer = _scorer(0.0, 6)
        stats = _count_norms(monkeypatch)
        for item in _items(20):
            scorer._existing_texts = ["上一轮缓存的并不这一段"]
            scorer._calculate_diversity(item["generated"], _refs(2), None)

        assert stats["axis1"] == 20, stats
        assert len(scorer._model.sizes) == 40, scorer._model.sizes


class TestInjectionSeamNowNeedsBothCells:
    """`_existing_embeddings` 的注入接缝改成两格同填：只填矩阵要当场响"""

    def test_lone_matrix_injection_raises_instead_of_using_stale_norms(self):
        scorer = _scorer(0.0, 4)
        scorer._existing_embeddings = np.array(
            [[1.0, 0.0, 0.0, 0.0], [0.0, 1.0, 0.0, 0.0]], dtype=np.float32)
        scorer._existing_texts = ["a", "b"]
        scorer._model.sizes.clear()

        with pytest.raises(TypeError):
            scorer._calculate_diversity(
                "c", ["a", "b"], np.array([0.0, 0.0, 1.0, 0.0], dtype=np.float32))
        # 判据命中的那一条路没被走成重编码 ⇒ 响声来自范数那一格，而不是缓存失效
        assert scorer._model.sizes == [], scorer._model.sizes

    def test_filling_both_cells_uses_the_injected_matrix(self):
        scorer = _scorer(0.0, 4)
        matrix = np.array([[1.0, 0.0, 0.0, 0.0], [0.0, 1.0, 0.0, 0.0]], dtype=np.float32)
        scorer._existing_embeddings = matrix
        scorer._existing_texts = ["a", "b"]
        scorer._existing_norms = np.linalg.norm(matrix, axis=1)
        scorer._model.sizes.clear()

        diversity = scorer._calculate_diversity(
            "c", ["a", "b"], np.array([0.0, 1.0, 0.0, 0.0], dtype=np.float32))
        assert diversity == 0.0
        assert scorer._model.sizes == [], scorer._model.sizes
