# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""L240 / A325：泄漏检测近似计数的**稠密路线**守卫（代价判据 + 逐条等值 + 计数牙齿）

立账出处：`LeakageDetector.detect` 在 posting 塌缩档把整份倒排按每条测试样本摊平重数一遍，
L239 实测 10k×10k **45.4 s**（全项目当时最大的一格热点）。本轮按 dedup 同款代价预言机
加一条 numpy 0/1 矩阵乘法路线，并**逐条判决不变**。

守卫口径（与 A286 同一条纪律：**判据一律计数，不用墙钟比值**）：
- 判据函数 `_dense_route_wins` / `_dense_block_rows` 是纯整数运算 ⇒ 逐档精确断言；
- 等值性按「同一份输入、两条路线的报告 `to_dict()` 逐字节相同」断言，示例顺序也在内；
- 稠密那条**不许引入第二套相似度算式** ⇒ 用 `_jaccard_from_counts` 的**调用次数**钉住；
- 「比较次数不随训练规模增长」用调用计数钉，不用时间比。

实测档位（`Temp/l240_leakprobe8.py` 内核复刻 + `Temp/l240_leakprobe9.py` 产品面，
uv 托管 cpython 3.12.14，同一台机器；比值列 = 乘加格数 ÷ 摊平次数，越小越该走稠密）：

| 形状 | n_tr | vocab | 摊平/条 | 乘加÷摊平 | 判负/判赢 |
|------|------|-------|---------|-----------|-----------|
| templated 24 词 4000×4000 | 3919 | 682 | 37,293.5 | 71.6 | 稠密（21×） |
| templated 24 词 10000×10000 | 9791 | 682 | 93,582.7 | 71.3 | 稠密（51.8×） |
| templated60 48 词 4000×4000 | 4000 | 614 | 90,256.9 | 27.2 | 稠密（77×） |
| templated60 48 词 10000×10000 | 10000 | 614 | 226,361.4 | 27.1 | 稠密（171×） |
| random5 铺开 4000×4000 | 4000 | 17576 | 828.4 | 84,905 | 倒排（1.4×） |
| zhshort 塌缩 1500×1500 | 12 | 68 | 0.0 | — | 无非精确条目，不进判据 |

⇒ `_DENSE_ADD_COST = 1000` 落在「判赢最差档 71.6」的 14 倍之上、「判负档 84,905」的 85 倍之下，
两侧都有整数量级的余量；random5 那一档同时还撞 `_DENSE_MAX_BYTES`（视图 281 MB > 64 MiB），
两条账各自都足以判它负。
"""

from __future__ import annotations

import json
import random
from typing import Dict, List, Set

import numpy as np
import pytest

from augmentor import leakage
from augmentor.leakage import (
    _DENSE_ADD_COST,
    _DENSE_MAX_BYTES,
    _DENSE_MAX_CELLS,
    _DENSE_MAX_ROWS,
    LeakageDetector,
    _char_trigrams,
    _dense_block_rows,
    _dense_route_wins,
    _jaccard_from_counts,
)

WORDS24 = ("instruction answer context sample prompt token train test list dict build "
           "check merge split filter map reduce count score rank select sample augment").split()
WORDS48 = [w + str(k) for k in range(4) for w in WORDS24][:48]
TEMPLATES = [
    "请把下面这段文本改写成更礼貌的请求：{a}",
    "列出 {a} 的三个近义词，并给出例句。",
    "判断这段回复是否忠实于原文：{a}",
    "把 {a} 翻译成英文，保持术语一致。",
    "为下面的代码写出单元测试：{a}",
]

THRESHOLD = 0.8


def templated(n: int, words: List[str], seed: int = 20261012) -> List[Dict]:
    rnd = random.Random(seed)
    items: List[Dict] = []
    for i in range(n):
        a = " ".join(rnd.choice(words) for _ in range(rnd.randint(3, 12)))
        instr = rnd.choice(TEMPLATES).format(a=a)
        if i % 50 == 0:
            instr = items[i - 1]["instruction"] if i else instr
        items.append({"instruction": instr, "output": "x"})
    return items


def spread(n: int, seed: int = 7) -> List[Dict]:
    """铺开词表：每条指令都是随机五字母词 ⇒ posting 窄，倒排本来就快"""
    import string

    rnd = random.Random(seed)
    return [{"instruction": " ".join(
        "".join(rnd.choices(string.ascii_lowercase, k=5)) for _ in range(rnd.randint(1, 24))),
        "output": "a"} for _ in range(n)]


HEADS = [
    "请把下面这段文本改写成更礼貌的请求，并保持术语一致：",
    "判断这段回复是否忠实于原文，并给出一句理由：",
    "为下面的代码写出单元测试，覆盖边界条件：",
]
TAILS = [
    "用户登录接口返回 500 的原因",
    "订单列表分页参数的默认值",
    "缓存失效后的重试策略",
]
VARIANTS = ["", "呀", "呢", "吧", "哦"]


def near_dup(n: int, seed: int = 20261012) -> List[Dict]:
    """塌缩且**真造得出近似泄漏**的语料：三条长头部 × 三条尾部 × 五只句末语气词

    为什么不能用 `templated`：那一族只有共享模板、正文全是 24 个英文词的随机串，
    4000×4000 也才 26 条近似泄漏（probe8 现量），70×70 的规模上一条都造不出 ⇒
    「两条路线逐字节相同」会退化成「两个 0 相等」的空转绿。这里长头部人人共享（posting
    宽 ⇒ 判据走稠密），测试侧又只比训练侧多一只语气词（J 远在 0.8 之上 ⇒ 判得出 fuzzy）。
    """
    rnd = random.Random(seed)
    items = []
    for i in range(n):
        head = HEADS[i % len(HEADS)]
        tail = rnd.choice(TAILS)
        variant = VARIANTS[(i + rnd.randrange(len(VARIANTS))) % len(VARIANTS)]
        items.append({"instruction": f"{head}{tail}{variant}", "output": "x"})
    return items


PADS = ("好的 收到 明白 了解 行吧 可以 没问题 就这样 多谢 麻烦了 "
        "谢谢 拜托 劳驾 尽快 立刻").split()


def padded_dup(test: List[Dict], n_pads: int = len(PADS)) -> List[Dict]:
    """把测试侧每条尾巴接上一只短后缀 ⇒ 与测试样本**近邻但不精确**的训练条目

    `near_dup` 那一族只有 3×3×5=45 只签名，训练侧一条 450 就把签名空间填满，
    测试侧全部转成 exact 命中、非精确行数为 0 —— 尺子当场空转。这里给每条测试
    样本尾部追加 2~3 个中文字：`b` 以 `a` 为前缀 ⇒ `a` 的三元组全在 `b` 里，
    I=|a| 远过 `need` 剪枝（倒排侧因此真比得多），同时 J=|a|/|b| < 1（不是 exact，
    且仍在 0.8 之上 ⇒ 判得出 fuzzy）。
    """
    out: List[Dict] = []
    for item in test:
        for pad in PADS[:n_pads]:
            out.append({"instruction": item["instruction"] + pad, "output": "x"})
    return out


def collapsed_train_test(words: List[str], n: int = 70, seed: int = 20261012):
    data = templated(n * 2, words, seed)
    return data[:n], data[n:]


def leaky_train_test(n: int = 60, extra_seed: int = 5):
    """训练侧 n 条、测试侧 n 条，且两侧同源 ⇒ 既有 exact 也有 fuzzy，两种交错在一起"""
    train = near_dup(n, seed=20261012)
    test = near_dup(n, seed=extra_seed)
    return train, test


def non_exact_count(train: List[Dict], test: List[Dict]) -> int:
    """非空且非精确命中的测试条数 —— 多条尺子共同的期望调用数"""
    sig = LeakageDetector()._signature
    train_uniq = {sig(i) for i in train} - {""}
    return len(train_uniq), sum(1 for i in test if sig(i) and sig(i) not in train_uniq)


class RouteSpy:
    """包住真判据、把每次判决记下来，行为一字不改"""

    def __init__(self) -> None:
        self.real = leakage._dense_route_wins
        self.calls: List[dict] = []

    def __enter__(self) -> "RouteSpy":
        outer = self

        def wrapper(block_rows, n_train, vocab_size, flatten_adds):
            verdict = outer.real(block_rows, n_train, vocab_size, flatten_adds)
            outer.calls.append({
                "block_rows": block_rows,
                "n_train": n_train,
                "vocab": vocab_size,
                "flatten_adds": flatten_adds,
                "dense": verdict,
            })
            return verdict

        leakage._dense_route_wins = wrapper
        return self

    def __exit__(self, *exc) -> bool:
        leakage._dense_route_wins = self.real
        return False

    @property
    def dense_blocks(self) -> int:
        return sum(1 for c in self.calls if c["dense"])


def force_route(monkeypatch, dense: bool) -> None:
    """把选路焊死成指定那条，用来做「同一份输入两条路」的逐字节对账"""
    monkeypatch.setattr(leakage, "_dense_route_wins",
                        lambda block_rows, n_train, vocab_size, flatten_adds: dense)


def detect_both_routes(train: List[Dict], test: List[Dict], monkeypatch) -> tuple:
    """同一份输入分别强制走稠密与倒排，返回两份报告（顺序、示例都在内）"""
    force_route(monkeypatch, True)
    dense = LeakageDetector(fuzzy_threshold=THRESHOLD).detect(train, test)
    force_route(monkeypatch, False)
    inverted = LeakageDetector(fuzzy_threshold=THRESHOLD).detect(train, test)
    return dense, inverted


def _spy_jaccard(monkeypatch) -> Dict[str, int]:
    """给权威算式装计数器：返回 `{"n": 0}`，调用次数累加在里面"""
    counter = {"n": 0}
    real = _jaccard_from_counts

    def spy(inter, sa, sb):
        counter["n"] += 1
        return real(inter, sa, sb)

    monkeypatch.setattr(leakage, "_jaccard_from_counts", spy)
    return counter


def _spy_trigrams(monkeypatch) -> Dict[str, int]:
    counter = {"n": 0}
    real = _char_trigrams

    def spy(text):
        counter["n"] += 1
        return real(text)

    monkeypatch.setattr(leakage, "_char_trigrams", spy)
    return counter


def build_index(train: List[Dict]):
    """按产品口径重建倒排：三元组 → 训练序号集合，序号 → 三元组个数"""
    det = LeakageDetector(fuzzy_threshold=THRESHOLD)
    gram_index: Dict[str, Set[int]] = {}
    counts: List[int] = []
    seen: Set[str] = set()
    for item in train:
        sig = det._signature(item)
        if not sig or sig in seen:
            continue
        seen.add(sig)
        grams = _char_trigrams(sig)
        counts.append(len(grams))
        for gram in grams:
            gram_index.setdefault(gram, set()).add(len(counts) - 1)
    return gram_index, counts


class TestPredicateIsPureCounts:
    """`_dense_route_wins` 只吃整数、只查两条账 —— 逐档精确断言，不留浮点余地"""

    def test_measured_winner_shapes_route_dense(self):
        """实测四档判赢形状（probe8 现量）逐档验判据：单价 1000 都收得住"""
        cases = [
            # templated 24 词 4000×4000：摊平/条 37,293.5 ⇒ 整块 1.49 亿次，乘加 107 亿格
            (4000, 3919, 682, 149_047_535),
            # templated 24 词 10000×10000：摊平/条 93,582.7 ⇒ 整块 9.34 亿次，乘加 666 亿格
            (9976, 9791, 682, 933_581_579),
            # templated60 48 词 4000×4000：摊平/条 90,256.9 ⇒ 整块 3.61 亿次，乘加 98.2 亿格
            (4000, 4000, 614, 361_027_600),
        ]
        for rows, n_tr, vocab, adds in cases:
            madds = rows * n_tr * vocab
            assert n_tr * vocab * 4 <= _DENSE_MAX_BYTES, "赢档必须连内存账也过，否则读数不可比"
            assert adds * _DENSE_ADD_COST >= madds, f"{rows}×{n_tr}×{vocab}：判据漏收实测赢档"
            assert _dense_route_wins(rows, n_tr, vocab, adds) is True

    def test_measured_loser_shape_routes_inverted(self):
        """random5 铺开档（乘加÷摊平 = 84,905）判负，且视图字节也越线"""
        rows, n_tr, vocab, adds = 4000, 4000, 17576, 3_313_600
        assert _dense_route_wins(rows, n_tr, vocab, adds) is False
        # 两条账各自都足以判它负 —— 时间账独立成立，内存账独立成立
        assert adds * _DENSE_ADD_COST < rows * n_tr * vocab
        assert n_tr * vocab * 4 > _DENSE_MAX_BYTES

    def test_time_account_boundary_is_inclusive(self):
        """`摊平次数 × 单价 == 乘加格数` 恰好判赢（闭区间），少一次累加就判负"""
        rows, n_tr, vocab = 10, 50, 40
        madds = rows * n_tr * vocab
        assert madds == 20_000 and 20 * _DENSE_ADD_COST == madds
        assert _dense_route_wins(rows, n_tr, vocab, madds // _DENSE_ADD_COST) is True
        assert _dense_route_wins(rows, n_tr, vocab, madds // _DENSE_ADD_COST - 1) is False

    def test_memory_line_boundary_is_inclusive(self):
        """视图字节恰好等于上限判赢、多一列判负（时间账两边都放行）"""
        n_tr = 1000
        vocab_at_cap = _DENSE_MAX_BYTES // 4 // n_tr
        huge_adds = n_tr * vocab_at_cap * 10 * _DENSE_ADD_COST
        assert n_tr * vocab_at_cap * 4 <= _DENSE_MAX_BYTES
        assert _dense_route_wins(1, n_tr, vocab_at_cap, huge_adds) is True
        assert _dense_route_wins(1, n_tr, vocab_at_cap + 1, huge_adds) is False

    @pytest.mark.parametrize("rows,n_tr,vocab,adds", [
        (0, 100, 50, 10_000),       # 块里没有非精确条目：不进判据，也不许建视图
        (10, 0, 50, 10_000),        # 训练集空：稠密那侧压根选不出 argmax
        (10, 100, 0, 10_000),       # 词表空（训练签名全没三元组）
        (10, 100, 50, 0),           # 摊平次数为零：倒排那条一次都不比，稠密纯亏
        (-1, 100, 50, 10_000),      # 负数按「不进稠密」处理，判据不许把它读成大数
    ])
    def test_zero_and_negative_sides_stay_inverted(self, rows, n_tr, vocab, adds):
        assert _dense_route_wins(rows, n_tr, vocab, adds) is False

    def test_block_rows_is_integer_inverse_of_cell_cap(self):
        """块大小 = 格数上限 ÷ 训练条数，夹在 [1, _DENSE_MAX_ROWS]，纯整数"""
        assert _dense_block_rows(0) == _DENSE_MAX_ROWS
        assert _dense_block_rows(1) == _DENSE_MAX_ROWS
        assert _dense_block_rows(_DENSE_MAX_CELLS) == 1
        assert _dense_block_rows(_DENSE_MAX_CELLS * 10) == 1
        # 10k×10k 实测档：9791 条训练 ⇒ 102 行一块（1,000,000 ÷ 9791 向零截断）
        assert _dense_block_rows(9791) == 102
        assert _dense_block_rows(9791) * 9791 <= _DENSE_MAX_CELLS
        assert _dense_block_rows(_DENSE_MAX_CELLS // _DENSE_MAX_ROWS) == _DENSE_MAX_ROWS


class TestRoutesAreVerdictIdentical:
    """两条路线**逐条判决相同**：报告 `to_dict()` 逐字节对账（含示例顺序与计数）"""

    def test_collapsed_corpus_byte_identical_reports(self, monkeypatch):
        train, test = leaky_train_test()
        dense, inverted = detect_both_routes(train, test, monkeypatch)
        assert json.dumps(dense.to_dict(), sort_keys=True, ensure_ascii=False) == \
            json.dumps(inverted.to_dict(), sort_keys=True, ensure_ascii=False)
        assert dense.fuzzy_leaks > 0, "语料没造出近似泄漏 ⇒ 这支对账是空转的绿"
        assert dense.exact_leaks > 0, "语料没造出精确泄漏 ⇒ exact/非精确的错位看不见"

    def test_spread_corpus_byte_identical_reports(self, monkeypatch):
        """随机词表档（连一条泄漏都没有）也要逐字节相同 —— 「全 0」也是一种判决"""
        data = spread(140)
        dense, inverted = detect_both_routes(data[:70], data[70:], monkeypatch)
        assert dense.total_leaks == 0 == inverted.total_leaks
        assert dense.to_dict() == inverted.to_dict()

    def test_48word_corpus_byte_identical_reports(self, monkeypatch):
        train, test = collapsed_train_test(WORDS48, seed=5)
        dense, inverted = detect_both_routes(train, test, monkeypatch)
        assert dense.to_dict() == inverted.to_dict()

    @pytest.mark.parametrize("threshold,min_examples", [
        (1.0, 3),      # need = |a|：剪枝收到最紧，双临界平局口径必须一致
        (0.3, 0),      # 阈值放到很宽 + 「报告不带示例」那一档
        (0.999, 5),
    ])
    def test_threshold_and_examples_corners(self, monkeypatch, threshold, min_examples):
        train, test = leaky_train_test(n=40)
        force_route(monkeypatch, True)
        d = LeakageDetector(fuzzy_threshold=threshold,
                            min_examples=min_examples).detect(train, test)
        force_route(monkeypatch, False)
        i = LeakageDetector(fuzzy_threshold=threshold,
                            min_examples=min_examples).detect(train, test)
        assert d.to_dict() == i.to_dict()

    def test_blank_and_empty_train_sides(self, monkeypatch):
        """空签名不进 rows；训练侧全空 ⇒ 判据的零档把稠密挡住，倒排那条空转也不炸

        这一支同时把「强制稠密」这条缝的量程说清楚：训练侧一条签名都没有时，
        `_dense_route_wins` 判负，所以 `_dense_flags` 压根不会被调用（视图是 0 列，
        argmax 无从选起）。判据的零档另有 `test_zero_and_negative_sides_stay_inverted` 钉。
        """
        train = [{"instruction": ""}, {"instruction": "   "}]
        test = [{"instruction": "请解释一下过拟合"}, {"instruction": ""}]
        with RouteSpy() as spy:
            report = LeakageDetector(fuzzy_threshold=THRESHOLD).detect(train, test)
        assert spy.dense_blocks == 0, "训练侧为空还走稠密 ⇒ 判据的零档失守"
        assert report.total_leaks == 0
        # 有真训练条目时：空签名那条既不精确也不近似，两条路线同判
        train2 = [{"instruction": "请解释一下什么是过拟合现象"}] + train
        dense, inverted = detect_both_routes(train2, test, monkeypatch)
        assert dense.to_dict() == inverted.to_dict()
        assert dense.exact_leaks == inverted.exact_leaks

    def test_verdicts_match_brute_force_on_both_routes(self, monkeypatch):
        """独立预言机（真集合交并、无剪枝、无 numpy）逐条对账两条路线"""
        train, test = leaky_train_test(n=45)
        sig = LeakageDetector()._signature
        tr_grams: List[Set[str]] = []
        seen = set()
        for item in train:
            s = sig(item)
            if s and s not in seen:
                seen.add(s)
                tr_grams.append(_char_trigrams(s))
        flags = []
        for item in test:
            s = sig(item)
            if not s or s in seen:
                continue
            grams = _char_trigrams(s)
            best = max((len(grams & b) / len(grams | b) for b in tr_grams if grams | b),
                       default=0.0)
            flags.append(best >= THRESHOLD)
        assert sum(flags) > 0, "预言机这边就没近似泄漏 ⇒ 这支对账空转"
        dense, inverted = detect_both_routes(train, test, monkeypatch)
        for name, report in (("稠密", dense), ("倒排", inverted)):
            assert report.fuzzy_leaks == sum(flags), f"{name}那条与暴力预言机对不上"


class TestCountingTeeth:
    """计数牙齿：A286 之后的口径 —— 规模不变性用调用次数钉，不用墙钟比值"""

    def test_dense_uses_the_authoritative_formula_once_per_row(self, monkeypatch):
        """稠密那条不许自带第二套相似度算式：每条非精确样本恰好一次 `_jaccard_from_counts`"""
        train, test = leaky_train_test()
        _, non_exact = non_exact_count(train, test)
        force_route(monkeypatch, False)
        exact_n = LeakageDetector(fuzzy_threshold=THRESHOLD).detect(train, test).exact_leaks
        counter = _spy_jaccard(monkeypatch)
        force_route(monkeypatch, True)
        LeakageDetector(fuzzy_threshold=THRESHOLD).detect(train, test)
        assert counter["n"] > 0, "一次都没比过 ⇒ 这把尺子在空转"
        assert counter["n"] == non_exact == len(test) - exact_n

    def test_candidate_count_does_not_grow_with_train_size(self, monkeypatch):
        """训练侧涨一个量级：稠密那条比较次数**恒定**，倒排那条随规模涨

        这是 A325 性能主张的计数形式（A286 之后的口径：不许用墙钟比值当判据）。
        两侧共用同一份测试清单，训练侧只在「额外条目」上 differ，所以期望比较次数
        在两侧同值 —— 不然「恒定」会被「测试条数变了」混淆。
        """
        test = near_dup(80, seed=11)
        base_train = near_dup(50, seed=20261012)
        # 追加语料必须与测试侧**近邻但不精确**，否则倒排那条压根不比更多：
        # - 第一版用完全无关的中文句子 ⇒ 过不了 `need` 剪枝，读数 (71) 对 (71) 一格未动；
        # - 第二版改用 `near_dup(450, seed=99)` ⇒ 该族只有 45 只签名，450 条把空间填满，
        #   测试侧 22 条非精确全转 exact ⇒ non_exact=0、两侧调用数一起变 0，空转绿。
        # 现在用 `padded_dup`：每条测试样本 ×15 只短后缀 = 1200 条近邻训练条目。
        extra_train = padded_dup(test)
        readings = []
        for name, train in (("小训练集", base_train),
                            ("大训练集", base_train + extra_train)):
            _, non_exact = non_exact_count(train, test)
            counter = _spy_jaccard(monkeypatch)
            force_route(monkeypatch, True)
            LeakageDetector(fuzzy_threshold=THRESHOLD).detect(train, test)
            dense_calls = counter["n"]
            counter = _spy_jaccard(monkeypatch)
            force_route(monkeypatch, False)
            LeakageDetector(fuzzy_threshold=THRESHOLD).detect(train, test)
            inverted_calls = counter["n"]
            assert dense_calls == non_exact, f"{name}：稠密那条比的不再是「每行一次」"
            assert inverted_calls > dense_calls, f"{name}：倒排那条没比更多 —— 尺子看不见这一维"
            readings.append((len(train), non_exact, dense_calls, inverted_calls))
        for n_train, rows, dense_calls, _ in readings:
            assert dense_calls == rows, f"n_train={n_train} 那档的比较次数不等于行数"
        # 现量：小训练集 32 只唯一签名 / 71 次比较，大训练集 602 只 / 1466 次（20.6×），
        # 而两档的 dense 侧都钉在 22（= 行数）。
        assert readings[0][1] == readings[1][1] == 22, "训练规模涨把非精确行数也改了，对账失效"
        assert readings[1][3] > readings[0][3] * 4, \
            f"倒排侧的比较次数几乎不随训练规模涨：{readings}"

    @pytest.mark.parametrize("dense", [True, False])
    def test_trigrams_computed_once_per_signature(self, monkeypatch, dense):
        """块化之后 `_char_trigrams` 依旧每签名一次：摊平记账与两条路线都不重算"""
        train, test = leaky_train_test()
        uniq_n, non_exact = non_exact_count(train, test)
        counter = _spy_trigrams(monkeypatch)
        force_route(monkeypatch, dense)
        LeakageDetector(fuzzy_threshold=THRESHOLD).detect(train, test)
        assert counter["n"] == uniq_n + non_exact

    def test_block_alignment_survives_row_block_of_one(self, monkeypatch):
        """块大小压到 1（每行一次选路 + 一次回填）—— 判决序列必须与「整份一块」逐字节相同

        这一支钉的是本轮踩过的坑：探针 v3 把 exact 即时追加、非精确项攒到 flush 才追加，
        两条 kind 序列整体错位，读出现象「计数相同但序列在下标 1025/1030/1031 不同」。
        产品里回填位置写错，同一份错位会直接写进 `leaked_examples` 的顺序。
        """
        train, test = leaky_train_test()
        monkeypatch.setattr(leakage, "_dense_block_rows", lambda n_train: 1)
        force_route(monkeypatch, True)
        tiny = LeakageDetector(fuzzy_threshold=THRESHOLD).detect(train, test)
        monkeypatch.setattr(leakage, "_dense_block_rows", lambda n_train: _DENSE_MAX_ROWS)
        force_route(monkeypatch, True)
        whole = LeakageDetector(fuzzy_threshold=THRESHOLD).detect(train, test)
        assert tiny.to_dict() == whole.to_dict()
        assert whole.fuzzy_leaks > 0 and whole.exact_leaks > 0, \
            "两种判决都没造出 ⇒ 这支对齐钉子不成立"
        force_route(monkeypatch, False)
        inverted = LeakageDetector(fuzzy_threshold=THRESHOLD).detect(train, test)
        assert tiny.to_dict() == inverted.to_dict()

    def test_dense_view_rows_match_their_postings(self):
        """视图的行列必须与 `gram_ids`、`sig_gram_counts` 同源：把行序打乱就是错位"""
        train, _ = collapsed_train_test(WORDS24)
        gram_index, counts = build_index(train)
        gram_ids, mat, size_b = leakage._dense_view(gram_index, counts)
        assert mat.shape == (len(gram_ids), len(counts))
        assert size_b.shape == (len(counts),)
        assert sorted(gram_ids.values()) == list(range(mat.shape[0])), "行号不是连续满射"
        for gram, row in gram_ids.items():
            assert set(np.nonzero(mat[row])[0]) == gram_index[gram]
        assert (size_b.astype(int) == np.asarray(counts)).all()
        assert gram_ids.get("zzz") is None, "查不到的三元组必须落在视图之外"

    def test_route_is_recorded_per_measured_shape(self):
        """形状→路线点名：塌缩档走稠密、铺开档走倒排（语料漂出量程时这里先红）"""
        for tag, data, want_dense in (
                ("templated 24 词", templated(140, WORDS24), True),
                ("templated60 48 词", templated(140, WORDS48, seed=5), True),
                ("random5 铺开", spread(140), False)):
            train, test = data[:70], data[70:]
            with RouteSpy() as spy:
                LeakageDetector(fuzzy_threshold=THRESHOLD).detect(train, test)
            assert spy.calls, f"{tag}：判据没被调用，路线断言是空的"
            assert (spy.dense_blocks > 0) is want_dense, \
                f"{tag} 走了 {spy.dense_blocks}/{len(spy.calls)} 块稠密，期望 {want_dense}"


class TestViewBuiltOnlyWhenRouted:
    """稠密视图是「按代价换路」才物化，不是无条件物化"""

    def test_inverted_route_builds_no_dense_view(self, monkeypatch):
        builds: List[int] = []
        real = leakage._dense_view

        def spy(gram_index, sig_gram_counts):
            builds.append(len(gram_index))
            return real(gram_index, sig_gram_counts)

        monkeypatch.setattr(leakage, "_dense_view", spy)
        data = spread(140)
        force_route(monkeypatch, False)
        LeakageDetector(fuzzy_threshold=THRESHOLD).detect(data[:70], data[70:])
        assert builds == [], "强制走倒排还建了稠密视图 ⇒ 视图不再由判据把关"
        force_route(monkeypatch, True)
        LeakageDetector(fuzzy_threshold=THRESHOLD).detect(data[:70], data[70:])
        assert len(builds) == 1, "走稠密却不建视图 ⇒ 上面那笔「0 次」是假绿"

    def test_view_is_built_once_per_detect_call(self, monkeypatch):
        builds: List[int] = []
        real = leakage._dense_view

        def spy(gram_index, sig_gram_counts):
            builds.append(len(gram_index))
            return real(gram_index, sig_gram_counts)

        monkeypatch.setattr(leakage, "_dense_view", spy)
        monkeypatch.setattr(leakage, "_dense_block_rows", lambda n_train: 2)
        force_route(monkeypatch, True)
        train, test = collapsed_train_test(WORDS24)
        LeakageDetector(fuzzy_threshold=THRESHOLD).detect(train, test)
        assert len(builds) == 1, f"视图建了 {len(builds)} 次：一次性物化被写成每块一次"


class TestPeakMemoryAccounts:
    """两条账各留一支内存护栏：稠密那份既要算得出来，也要量得出读数"""

    def test_dense_view_bytes_and_block_cells_are_within_caps(self):
        """稠密侧峰值是**算出来**的，不是等 tracemalloc 撞上才发现问题"""
        n_tr, vocab = 9791, 682          # 10k×10k 实测档
        assert n_tr * vocab * 4 <= _DENSE_MAX_BYTES
        rows = _dense_block_rows(n_tr)
        assert rows * n_tr <= _DENSE_MAX_CELLS
        # 一格最多活三次（float32 累加、float64 商、float64 并集）⇒ 24 B/格
        peak = n_tr * vocab * 4 + rows * n_tr * 24
        assert peak <= _DENSE_MAX_BYTES + _DENSE_MAX_CELLS * 24
        assert peak / 1048576 < 95, f"10k×10k 档稠密侧峰值 {peak / 1048576:.1f} MiB 超预算"

    def test_dense_route_tracemalloc_peak_holds_on_collapsed_corpus(self, monkeypatch):
        """tracemalloc 看得见 numpy 的分配（本轮实测：1200×300 档稠密 16.4 MiB / 倒排 1.2 MiB）

        这一支的意义不是「省内存」，而是把**换路的内存代价钉成可核对的读数**：
        上界取 24 MiB（实测 16.4 的 1.5 倍余量），并同时断言稠密那份确实更贵，
        否则「1.2 MiB 的倒排」与「16.4 MiB 的稠密」哪条在被测就说不清。
        """
        import tracemalloc

        train = [{"instruction": f"训练样本编号 {i:05d} 号的问题描述文本"} for i in range(1200)]
        test = [{"instruction": f"测试样本编号 {j:05d} 号的问题描述文本"} for j in range(300)]
        peaks = {}
        for name, dense in (("dense", True), ("inverted", False)):
            force_route(monkeypatch, dense)
            tracemalloc.start()
            try:
                LeakageDetector(fuzzy_threshold=THRESHOLD).detect(train, test)
                peaks[name] = tracemalloc.get_traced_memory()[1]
            finally:
                tracemalloc.stop()
        assert peaks["dense"] > peaks["inverted"], "换路不费内存 ⇒ 上面那笔字节账是白算的"
        assert peaks["dense"] / 1048576 < 24.0
        assert peaks["inverted"] / 1048576 < 2.5
