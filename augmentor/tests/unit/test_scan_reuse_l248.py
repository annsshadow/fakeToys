# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""L248 / A326：稠密那条**只多扫一遍字符**这件事必须有尺子

产品改动住在 `augmentor/dedup.py:486-510`（`_similarity_blocks` 的 fallback 分支把
`_ngram_scan` 的列号表交给稠密那条）与 `augmentor/dedup.py:182-191`（`_fallback_encode`
的发现遍改成「没人给表才自己扫」）。

**为什么位相同的判决不足以自证**：本轮的改动性质是「省掉一趟计算」，复用失败时
分组产物照样逐字节正确，只是慢一点。承 A286 那条「性能判据一律计数、不量墙钟」
纪律，这里全部用结构判据——**趟数、对象身份、只读且会计数的列号表**——而不是时间。

**十组判据**：

1. 稠密那条整条链路上 `_ngram_scan` 恰好 **1** 次、`_ngram_entries` **0** 次：
   选路那一趟就是编码那一趟。判倒排赢时同样 1 次/0 次，且稠密入口 0 次。
2. `_batch_encode` 收到的 `vocab` 与 `_ngram_scan` 返回的表**是同一个对象**（`is`）：
   排除了「内容相同但当场又建了一份」这种假复用。
3. 把「写就炸、读会计数」的现成表喂给 `_fallback_encode`：**一次都不许写**（写了就是
   仍在自己扫发现遍），**读次数恰好等于每篇去重后的二元组数之和**（读少了就是另建了
   一份新表）。断言里带陷阱自证，防止这张表本身是恒真的。
4. 复用表与自己建表两条编码 `tobytes()` **逐字节相等**（含 `_ngram_scan` 的建表顺序
   与发现遍一致这一前提）。
5. `vocab={}` 的退化档：矩阵是 `(n, 0)`，不 KeyError、不凭空加列。
6. **预言机入参名册**：四档语料的 `(doc_count, vocab_size, pair_adds, block_rows, 判决)`
   逐个钉住——列号表复用之后选路依据一格都不许变，这排除了「少扫一遍顺带改选了路」。
7. 名册的判决形状本身（三负一正）再钉一支：第 6 条若退化成了两张空表互比，这支当场红。
8. `_ngram_entries` 仍是 `_ngram_scan` + 折叠的组合，拆函数没把它改坏。
9. 判倒排赢时，产出第一个块之前选路 scratch（列号表 + 逐趟数组）必须**已经松手**——
   `tracemalloc` 的 current 读数钉住。倒排那条的意义是不物化 n×vocab 矩阵，scratch
   陪着它活等于把省下来的内存又还回去。
10. 每条判据都用真语料跑，禁止空跑：非空断言与形状断言同时在场。

**边界**：本守卫不测「两条路数值一致」（`tests/unit/test_dedup.py` 的
`TestInvertedBlocksMatchDenseBlocks` 已焊）与「判一条另一条不许碰」（同文件
`TestOnlyOneProducerRuns`），它测的是这两条都没覆盖的**扫描趟数**。
"""
from typing import Dict, List, Tuple

import numpy as np
import pytest

from augmentor.dedup import Deduplicator

#: 生产路径默认的行块请求量（`_find_duplicate_groups_chunked` 的 chunk_size 默认值）
DEFAULT_CHUNK = 1000


def _fallback() -> Deduplicator:
    """显式 fallback 编码器：本文件测的是字符扫描，与本机装没装模型无关"""
    dedup = Deduplicator()
    dedup._model = "fallback"
    return dedup


class _ReadOnlyTable(dict):
    """只许读、一读就计数的列号表

    `_fallback_encode` 若真接过了这张表，填矩阵那一趟的每一次 `vocab[ngram]` 都落在
    这里，计数应当**恰好等于「每篇去重后的词条数之和」**；反之它若当场另建一份新表，
    这张表一次都不会被读。写则当场响——发现遍的唯一动作就是 `vocab[ngram] = len(vocab)`。
    这两半合起来才是「复用」的完整机器定义：内容相同但重建一份（假复用）过不了计数，
    收到现成表还自己扫（没接上）过不了零写。
    """

    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.reads = 0

    def __getitem__(self, key):
        self.reads += 1
        return dict.__getitem__(self, key)

    def __setitem__(self, key, value):
        raise AssertionError("收到现成列号表却仍往里写：发现遍没有被跳过")

    def update(self, *args, **kwargs):
        raise AssertionError("收到现成列号表却仍往里写：发现遍没有被跳过")

    def setdefault(self, *args, **kwargs):
        raise AssertionError("收到现成列号表却仍往里写：发现遍没有被跳过")


def _readonly(vocab: Dict[str, int]) -> _ReadOnlyTable:
    """用 `dict.__setitem__` 绕过子类钩子灌进去，得到一张只读表"""
    table = _ReadOnlyTable()
    for key, value in vocab.items():
        dict.__setitem__(table, key, value)
    return table


# ---------------------------------------------------------------------------
# 四档语料：三档判稠密赢、一档判倒排赢，形状全部由下标算出，不用随机数
# ---------------------------------------------------------------------------

def shape_dup(n: int = 400) -> List[str]:
    """小词表 + 大量重复：稠密乘加便宜，倒排的散射次数爆炸 ⇒ 判稠密赢"""
    return ["sort a list of items number %d please" % (i % 7) for i in range(n)]


def shape_alpha(n: int = 600) -> List[str]:
    """4 字符字母表上 14 字符短文本：词表只有十几项 ⇒ 判稠密赢"""
    alphabet = "abcd"
    return ["".join(alphabet[(i * 7 + j * 3) % 4] for j in range(14)) for i in range(n)]


def shape_mixed(n: int = 700) -> List[str]:
    """64 字符字母表上 60 字符：词表上千项，仍未越过倒排的判据线 ⇒ 判稠密赢"""
    alphabet = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789"
    return ["".join(alphabet[(i * 11 + j * 17) % 62] for j in range(60)) for i in range(n)]


def shape_long(n: int = 1200, length: int = 120) -> List[str]:
    """4000 个汉字上 120 字符长文：词表≈出现次数 ⇒ 倒排判赢（真实语料的形状）

    下标里必须带**非线性项**：纯线性 `a*i + b*j` 会让二元组完全由起点决定，词表塌到
    4000 项、每个词条的 df 涨到几十，倒排的散射次数反而爆炸（L248 写这档时第一版就
    踩了这个坑，实测 `(4000, 2623572, False)`）。二次项让相邻差值随 `j` 变，
    二元组几乎篇篇互异，才是真实语料「词表大、df 小」的那一侧。
    """
    return ["".join(chr(0x4E00 + (31 * i + 17 * j + 29 * j * j) % 4000) for j in range(length))
            for i in range(n)]


SHAPES = {
    "dup": shape_dup,
    "alpha": shape_alpha,
    "mixed": shape_mixed,
    "long": shape_long,
}


def route_of(dedup: Deduplicator, texts: List[str]) -> Tuple[int, int, int, int, bool]:
    """复现选路那一趟，返回预言机入参与判决（不碰任何一条生产者）"""
    block_rows = dedup._row_block_size(len(texts), DEFAULT_CHUNK)
    doc_arr, term_arr, vocab = dedup._ngram_scan(texts)
    inverted = dedup._inverted_view(texts, scan=(doc_arr, term_arr, vocab))
    win = dedup._inverted_blocks_win(inverted.doc_count, inverted.vocab_size,
                                     inverted.pair_adds, block_rows)
    return (inverted.doc_count, inverted.vocab_size, inverted.pair_adds, block_rows, win)


#: 被计数的四个接缝。名字全列在这里，计数断言才不会把「一次都没跑」写成「缺键」。
SPIED = ("_ngram_scan", "_ngram_entries", "_fallback_encode", "_batch_encode")


class TestScanCountOnEachRoute:
    """复用列号表之后，稠密那条总共只扫两趟字符：选路一趟 + 填矩阵一趟"""

    @staticmethod
    def spy(monkeypatch, name, log):
        original = getattr(Deduplicator, name)

        def wrapper(*args, **kwargs):
            result = original(*args, **kwargs)
            log.append((name, result))
            return result

        monkeypatch.setattr(Deduplicator, name, wrapper)

    @staticmethod
    def counts(log):
        out = dict.fromkeys(SPIED, 0)
        for name, _ in log:
            out[name] += 1
        return out

    def test_dense_route_scans_the_corpus_once(self, monkeypatch):
        """判稠密赢时：`_ngram_scan` 恰好 1 次，`_ngram_entries` 一次都不许跑

        `_ngram_entries` 是「自己从头再扫一遍」的那个入口。它一旦被碰上，就说明列号表
        没接过来，本轮的改动当场失效——而分组产物不会有任何可见差异。
        """
        log = []
        for name in ("_ngram_scan", "_ngram_entries", "_fallback_encode", "_batch_encode"):
            self.spy(monkeypatch, name, log)

        dedup = _fallback()
        texts = shape_dup()
        blocks = list(dedup._similarity_blocks(texts, DEFAULT_CHUNK))

        assert self.counts(log) == {"_ngram_scan": 1, "_ngram_entries": 0,
                                    "_fallback_encode": 1, "_batch_encode": 1}, (
            f"稠密那条的扫描趟数不对: {self.counts(log)}"
        )
        assert blocks and blocks[0][2].shape[1] > 0, "本用例空跑了"

    def test_inverted_route_scans_the_corpus_once(self, monkeypatch):
        """判倒排赢时也只剩一趟扫描，且稠密那条一个入口都不碰"""
        log = []
        for name in ("_ngram_scan", "_ngram_entries", "_fallback_encode", "_batch_encode"):
            self.spy(monkeypatch, name, log)
        monkeypatch.setattr(Deduplicator, "_inverted_blocks_win",
                            staticmethod(lambda *args: True))

        dedup = _fallback()
        blocks = list(dedup._similarity_blocks(shape_long(200, 60), DEFAULT_CHUNK))

        assert self.counts(log) == {"_ngram_scan": 1, "_ngram_entries": 0,
                                    "_fallback_encode": 0, "_batch_encode": 0}, (
            f"倒排那条的扫描趟数不对: {self.counts(log)}"
        )
        assert blocks, "本用例空跑了"

    def test_dense_route_encodes_with_the_routing_table(self, monkeypatch):
        """`_batch_encode` 收到的必须是 `_ngram_scan` 返回的**同一个对象**

        只比内容会放过「拿到表之后当场又建一份」这种假复用；`is` 才钉得住。
        """
        log = []
        self.spy(monkeypatch, "_ngram_scan", log)
        self.spy(monkeypatch, "_batch_encode", log)

        dedup = _fallback()
        texts = shape_dup()
        list(dedup._similarity_blocks(texts, DEFAULT_CHUNK))

        scanned = [r for n, r in log if n == "_ngram_scan"]
        handed = [r for n, r in log if n == "_batch_encode"]
        assert len(scanned) == 1 and len(handed) == 1, f"接缝调用次数不对: {len(log)}"
        table = scanned[0][2]
        assert isinstance(table, dict) and table, "选路那一趟没建出列号表"
        assert handed[0].shape == (len(texts), len(table)), (
            f"稠密矩阵列宽 {handed[0].shape[1]} 与列号表长 {len(table)} 不符："
            "编码用的不是选路那张表"
        )

    def test_the_table_handed_to_batch_encode_is_the_scanned_one(self, monkeypatch):
        """直接抓住 `_batch_encode` 的 `vocab` 实参，与扫描返回的表比身份"""
        seen = {}

        original_scan = Deduplicator._ngram_scan

        def scan(self, texts):
            result = original_scan(self, texts)
            seen["table"] = result[2]
            return result

        monkeypatch.setattr(Deduplicator, "_ngram_scan", scan)

        original_encode = Deduplicator._batch_encode

        def encode(self, texts, vocab=None):
            seen["handed"] = vocab
            return original_encode(self, texts, vocab)

        monkeypatch.setattr(Deduplicator, "_batch_encode", encode)

        dedup = _fallback()
        list(dedup._similarity_blocks(shape_alpha(), DEFAULT_CHUNK))

        assert seen["handed"] is seen["table"], (
            "稠密那条拿到的列号表不是选路那一趟建出来的（假复用：内容相同但重建了一份）"
        )


class TestDiscoveryPassIsConditional:
    """`_fallback_encode` 的「自己建表」那一遍只在没人给表时才跑"""

    def test_given_table_is_read_exactly_once_per_term_and_never_written(self):
        """喂一张「只读 + 计数」的现成表：必须正常返回矩阵、读够数、一次都不写

        三段各有分工：
        - **零写**否证「收到现成表仍然自己扫第一遍」；
        - **读够数**否证「另建一份新表来填」（那种假复用一次都不会读这张表）；
        - 陷阱自证（先证明这张表确实会响、确实会计数）防止本用例是一把恒真的尺子。
        """
        dedup = _fallback()
        texts = ["abcdef", "abcdefg", "xyz abc def"]
        table = _readonly(dedup._ngram_scan(texts)[2])

        with pytest.raises(AssertionError):
            table["zz"] = 1  # 陷阱必须是活的
        assert table.reads == 0

        vectors = dedup._fallback_encode(texts, vocab=table)

        expected_reads = sum(len({t[j:j + 2] for j in range(len(t) - 1)}) for t in texts)
        assert vectors.shape == (3, len(table))
        assert vectors.dtype == np.float32
        assert table.reads == expected_reads, (
            f"填矩阵那一趟读了 {table.reads} 次，独立算出来的应该是 {expected_reads} 次"
            "（每篇去重后的二元组数之和）：读少了说明它另建了一份表"
        )

    def test_reused_and_self_built_tables_encode_identically(self):
        """复用不改变任何数值：两条编码 `tobytes()` 逐字节相等

        列号按首次出现顺序编，而 `_ngram_scan` 扫的是同一批 `texts`、同一套
        `text[j:j+2]`、同一个顺序 ⇒ 两张表逐键相同。这条断言就是那段说明的机器版。
        """
        dedup = _fallback()
        texts = list(SHAPES["mixed"]()) + list(SHAPES["dup"]())

        table = dedup._ngram_scan(texts)[2]
        with_table = dedup._fallback_encode(texts, vocab=table)
        without = dedup._fallback_encode(texts, vocab=None)

        assert table == {ng: i for i, ng in enumerate(
            dict.fromkeys(t[j:j + 2] for t in texts for j in range(len(t) - 1)))}
        assert with_table.shape == without.shape == (len(texts), len(table))
        assert with_table.tobytes() == without.tobytes(), (
            "复用列号表改变了编码产物：两张表的列序不是一一对应"
        )

    def test_empty_table_gives_an_n_by_zero_matrix(self):
        """`vocab={}` 的退化档：全部文本短于 2 字符 ⇒ 矩阵 (n, 0)，不凭空加列

        这是 `augmentor/dedup.py:182` 那个 `if vocab is None` 的另一半：给了**空**表
        不等于没给表，实现不许把空表当成缺参再自己去建。
        """
        dedup = _fallback()
        vectors = dedup._fallback_encode(["a", "b", ""], vocab={})

        assert vectors.shape == (3, 0)
        assert vectors.dtype == np.float32

    def test_scan_and_entries_fold_still_agree(self):
        """拆出 `_ngram_scan` 之后，`_ngram_entries` 的产物必须逐元素不变

        倒排那条仍然走这个入口（`tests/unit/test_dedup.py` 的
        `TestNgramInvertedView` 靠它建视图），所以它既是老行为的回归钉，
        也证明「折叠」那一半没被复用改动带跑。
        """
        dedup = _fallback()
        texts = list(SHAPES["mixed"]())

        docs, terms, counts, vocab_size = dedup._ngram_entries(texts)
        doc_arr, term_arr, table = dedup._ngram_scan(texts)
        folded = Deduplicator._entries_from_scan(doc_arr, term_arr, table)

        assert vocab_size == len(table) == 62
        for got, want in zip(folded[:3], (docs, terms, counts)):
            np.testing.assert_array_equal(got, want)
        assert folded[3] == vocab_size


class TestRoutingDecisionIsUntouched:
    """列号表复用之后，选路依据一格都不许变

    本轮改的是「谁扫第几遍」，不是「走哪条路」。把四档语料的预言机入参与判决逐个钉住，
    就等于排除了「少扫一遍顺带把路选错了」这一类副作用——它不会改变分组正确性，
    只会把慢的那条路换成另一条，正因为看不见才需要尺子。
    """

    #: 现量（L248，本机）：`(doc_count, vocab_size, pair_adds, block_rows, 走倒排)`
    MEASURED_ROSTER = {
        "dup": (400, 46, 2589658, 1000, False),
        "alpha": (600, 4, 721200, 1000, False),
        "mixed": (700, 62, 13776218, 1000, False),
        "long": (1200, 142800, 142800, 1000, True),
    }

    def test_route_roster_is_the_measured_inputs(self):
        """四档语料的选路入参与判决 == 现量名册"""
        dedup = _fallback()

        got = {name: route_of(dedup, build()) for name, build in SHAPES.items()}

        assert got == self.MEASURED_ROSTER, (
            "选路依据变了。本轮只该省掉一趟字符扫描，不该改判："
            f"\n实得 {got}\n名册 {self.MEASURED_ROSTER}"
        )

    def test_three_shapes_go_dense_one_goes_inverted(self):
        """名册不是抄来的常数：判决本身必须三负一正，否则上面那条是在比两张空表"""
        decisions = [v[-1] for v in self.MEASURED_ROSTER.values()]

        assert decisions == [False, False, False, True], (
            f"四档语料的判决形状不对: {decisions}"
        )
        assert all(v[1] > 0 for v in self.MEASURED_ROSTER.values()), "有语料词表为空"


#: 判倒排赢时，产出第一个块之前 traced current 的上限（字节）。
#: 现量（`<OUTSIDE_REPO>/l248_peak_probe.py`，1200×120 那档，两次读数 2 858 048 /
#: 2 857 379）：真路 **2.86 MB**；手工复刻「判完路仍握着列号表与逐趟数组」那一档
#: **21.83 MB**（142 800 项的 dict ≈ 16 MB + 两份 int64 出现数组 2.3 MB）。
#: 8 MiB 这根线两侧各留 2.7～2.9 倍余量：pass 侧宽到不会被 numpy 临时抖动碰响，
#: fail 侧紧到「松手」那一行一旦删掉就当场红（L248 注入臂㈢实测红）。
SCRATCH_CEILING = 8 * 1024 * 1024


class TestScratchIsReleasedOnTheInvertedRoute:
    """倒排那条不物化稠密矩阵，选路 scratch 也不许陪它一起活"""

    def test_first_block_is_produced_after_the_scratch_is_dropped(self, monkeypatch):
        """在 `_iter_inverted_similarity_row_blocks` 入口处读 traced current"""
        import tracemalloc

        texts = shape_long()
        dedup = _fallback()
        monkeypatch.setattr(Deduplicator, "_inverted_blocks_win",
                            staticmethod(lambda *args: True))

        seen = {}
        original = Deduplicator._iter_inverted_similarity_row_blocks

        def probing(self, inverted, block_rows):
            seen["current"] = tracemalloc.get_traced_memory()[0]
            seen["terms"] = int(len(inverted.terms))
            return original(self, inverted, block_rows)

        monkeypatch.setattr(Deduplicator, "_iter_inverted_similarity_row_blocks", probing)

        tracemalloc.start()
        try:
            first = next(dedup._similarity_blocks(texts, DEFAULT_CHUNK))
        finally:
            tracemalloc.stop()

        assert seen["terms"] > 0 and first[2].size > 0, "本用例空跑了"
        assert seen["current"] < SCRATCH_CEILING, (
            f"第一个块产出前还养着 {seen['current'] / 2**20:.1f} MiB：选路 scratch"
            "（列号表 + 逐趟数组）没在判完路之后松手，倒排省下的矩阵又被补回去了"
        )
