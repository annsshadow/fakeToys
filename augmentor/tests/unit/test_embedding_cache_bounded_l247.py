# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

r"""L247 守卫：A334 —— `_embedding_cache` 只留最近一批，逐 chunk 流式不再终身驻留

**为什么立这一条**：`batch_score` 里的整批向量缓存 `_embedding_cache`（键
`(batch_key, tuple(文本))`）原本是实例级、跨调用只增不减。生产侧两个真读者是「逐
chunk 调用、每个批次只评一次」的流式形状（`augmentor/cli/commands/pipeline.py:43`、
`augmentor/pipeline.py:557` 的 filter_by_quality）——它们**一次都命中不了**，却把历史批次的向量
一直挂在 scorer 实例上；驻留是纯泄漏，不是收益。API 侧（`api/routes/quality.py:150`
等）每请求新建 scorer，本来就无害。

**改前 / 改后现量**（仓外探针 FakeModel 顶掉 sentence-transformers，dim=64，
判决产物为每条样本的三个分与两个布尔量）：

| 形状 | 改前条目 / 驻留字节 | 改后 | 编码次数 改前→改后 |
|------|---------------------|------|--------------------|
| ① 逐 chunk 流式 4×1000 | 8 / 2 048 000 | **2 / 512 000** | 128 → 128（零代价） |
| ② 同一批连评两次（200） | 2 / 512 000 | 2 / 512 000 | 32 + 30 → 32 + 30（命中保住） |
| ③ a_b_a_b 交错（200） | 4 / 1 024 000 | 2 / 512 000 | 124 → 128（+4 次整批编码） |
| ④ a、b 各一次（comparison） | 4 / 1 024 000 | **2 / 512 000** | 64 → 64 |

①那一档按真实模型维度 384 折算是 **12 MB → 3 MB**（`diversity_sample_size` 与文本
数无关的那份参照切片另计）。③是本轮唯一付出的代价，而产品侧没有这种形状：
`augmentor/comparison.py:237-238` 是同一 scorer 上 a、b **各一次**（=④），`augmentor/benchmark.py:124` 每次
`run_benchmark` 一批，两边改前后编码次数都不变。四档的判决产物 `repr` 落盘
**逐字节相同**（sha256 前后同为 `6f405bf0099ef1e9…`，487 191 bytes）——有界化只改
「编码几次」，不改任何一个数。

**口径**：
1. 上限写死 2，等价于「只留最近一批」，耦合的是「一次 `batch_score` 最多插 orig/gen
   两个键」这个事实。将来谁加第三个键，同批连评两次就会退化成全 miss，
   `test_same_batch_scored_twice_skips_both_batch_encodes` 当场红——所以不需要再去
   静态数源码里的调用点。
2. 逐出的方向是**最旧**，不是最新：留下的必须恰好是最近一批的两个键，
   `test_the_surviving_pair_is_exactly_the_most_recent_batch` 钉这一格。
3. 本文件数的是**条目数与文本引用数**（驻留是纯内部事实，数值一路正确时没有任何
   别的尺子会出声），命中与 miss 各有一支点名用例，两维分开钉。
"""

import zlib

import numpy as np

from augmentor.quality import QualityScorer

DIM = 8


def _vec(text, dim=DIM):
    """由文本唯一决定的伪向量：同一文本永远同一向量，跨 scorer、跨进程可比"""
    rng = np.random.RandomState(zlib.crc32(text.encode("utf-8")) % (2 ** 31))
    return rng.rand(dim).astype(np.float32) + 0.1


class FakeModel:
    """顶掉 sentence-transformers，记录每次 encode 的条数与文本"""

    def __init__(self):
        self.sizes = []

    def encode(self, texts, **kwargs):
        self.sizes.append(len(texts))
        return np.array([_vec(t) for t in texts], dtype=np.float32)


def _scorer():
    """threshold=0.999 ⇒ 没有样本过得了门槛 ⇒ 多样性参照集恒空、不产生额外编码"""
    scorer = QualityScorer(threshold=0.999)
    scorer._cross_encoder = "fallback"
    scorer._model = FakeModel()
    return scorer


def _batch(idx, n):
    return [{"original": "种子问题 %d_%d" % (idx, i),
             "generated": "生成回答 %d_%d 的内容，这一段要够长才能彼此不同" % (idx, i),
             "output": "参考答案 %d_%d" % (idx, i)} for i in range(n)]


def _digest(scores):
    return [repr(s.semantic_similarity) for s in scores] + \
           [repr(s.total_score) for s in scores]


def _resident(scorer):
    """(条目数, 键里持有的文本引用个数, 向量字节数)：驻留面的三格读数"""
    cache = scorer._embedding_cache
    return (len(cache),
            sum(len(key[1]) for key in cache),
            sum(int(arr.nbytes) for arr in cache.values()))


class TestResidencyIsBounded:
    """逐 chunk 流式不再把历史批次的向量挂在实例上"""

    def test_only_the_last_batch_stays_resident_after_four_chunks(self):
        scorer = _scorer()
        batches = [_batch(i, 60) for i in range(4)]
        for b in batches:
            scorer.batch_score(b, include_semantic=True)

        assert len(scorer._embedding_cache) == 2, scorer._embedding_cache.keys()
        assert sorted(key[0] for key in scorer._embedding_cache) == ["gen", "orig"]
        last = {("orig", tuple(item["original"] for item in batches[3])),
                ("gen", tuple(item["generated"] for item in batches[3]))}
        assert set(scorer._embedding_cache) == last

    def test_residency_does_not_grow_from_the_first_chunk_to_the_last(self):
        """每批各测一次驻留三格：条目、文本引用、向量字节都不许随批次数往上涨"""
        scorer = _scorer()
        readings = []
        for i in range(5):
            scorer.batch_score(_batch(i, 40), include_semantic=True)
            readings.append(_resident(scorer))

        assert [r[0] for r in readings] == [2, 2, 2, 2, 2], readings
        assert [r[1] for r in readings] == [80] * 5, readings
        assert [r[2] for r in readings] == [2 * 40 * DIM * 4] * 5, readings


class TestEvictionIsOldestFirst:
    """逐出方向：丢最旧、留最新一批"""

    def test_the_surviving_pair_is_exactly_the_most_recent_batch(self):
        scorer = _scorer()
        scorer.batch_score(_batch(0, 30), include_semantic=True)
        scorer.batch_score(_batch(1, 30), include_semantic=True)

        texts = {key[1] for key in scorer._embedding_cache}
        assert texts == {tuple(item["original"] for item in _batch(1, 30)),
                         tuple(item["generated"] for item in _batch(1, 30))}
        stale = ("orig", tuple(item["original"] for item in _batch(0, 30)))
        assert stale not in scorer._embedding_cache


class TestHitShapeSurvivesTheCap:
    """唯一真读者（同一批连评两次）仍然白捡；交错批次是本轮认下的代价"""

    def test_same_batch_scored_twice_skips_both_batch_encodes(self):
        scorer = _scorer()
        b = _batch(0, 50)
        scorer.batch_score(b, include_semantic=True)
        assert scorer._model.sizes == [50, 50], scorer._model.sizes

        scorer._model.sizes.clear()
        scorer.batch_score(b, include_semantic=True)
        assert scorer._model.sizes == [], "上限 2 把「最近一批」挤掉了，同批重评不再命中"

    def test_interleaved_batches_lose_the_cross_batch_hit(self):
        """a_b_a_b 的每一轮都重编码：有界化的代价就钉在这一格，不写成散文"""
        scorer = _scorer()
        a, bb = _batch(0, 20), _batch(1, 20)
        for _ in range(2):
            scorer.batch_score(a, include_semantic=True)
            scorer.batch_score(bb, include_semantic=True)

        assert scorer._model.sizes == [20] * 8, scorer._model.sizes


class TestNumbersAreUntouchedByEviction:
    """位相同：逐出只改「编码几次」，不改任何一个数"""

    def test_scores_after_three_foreign_batches_equal_a_fresh_scorer(self):
        target = _batch(7, 40)
        scorer = _scorer()
        first = scorer.batch_score(target, include_semantic=True)
        for i in range(3):
            scorer.batch_score(_batch(i, 40), include_semantic=True)
        after_eviction = scorer.batch_score(target, include_semantic=True)

        fresh = _scorer().batch_score(target, include_semantic=True)
        assert _digest(after_eviction) == _digest(fresh)
        assert _digest(first) == _digest(after_eviction)

    def test_evicted_batch_is_reencoded_not_reused_from_a_stale_key(self):
        """命中判据没被顺手改松：同一 batch_key 换一批文本必须重编码"""
        scorer = _scorer()
        scorer.batch_score(_batch(0, 25), include_semantic=True)
        scorer._model.sizes.clear()
        scores = scorer.batch_score(_batch(1, 25), include_semantic=True)

        assert scorer._model.sizes == [25, 25], "换了文本还命中旧向量 = 分数被答成别人的"
        assert len(scores) == 25
