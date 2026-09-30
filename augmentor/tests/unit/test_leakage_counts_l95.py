# -*- coding: utf-8 -*-
"""L95 守卫：泄漏检测的「按计数算 Jaccard」重写必须与旧的按集合算等价，且热路径零集合运算

立尺依据（同进程 HEAD vs 工作树实测，Temp/l95q/verify_semantics_l95.py）：
- 阈值 0<t<=1 六档、阈值 <=0 三档，两版报告逐字段相同；
- `_char_trigrams` 调用次数 HEAD 798 → new 399（旧版每个候选测试条算两遍三元组）；
- 热路径集合运算 HEAD 50,834 次 → new 0 次。
"""
import sys
from pathlib import Path
from typing import Set

import pytest

ROOT = Path(__file__).resolve().parents[2]
if str(ROOT) not in sys.path:
    sys.path.insert(0, str(ROOT))

from augmentor.leakage import (  # noqa: E402
    LeakageDetector,
    _char_trigrams,
    _jaccard,
    _jaccard_from_counts,
    detect_leakage,
)


def brute_force_jaccard(a: Set[str], b: Set[str]) -> float:
    """独立算式（真建交并集），不复用被测模块的任何 helper"""
    if not a or not b:
        return 0.0
    return float(len(a & b)) / float(len(a | b))


def legacy_detect(train_items, test_items, fields, fuzzy_threshold):
    """暴力近似匹配：对每条测试样本，与每条训练样本逐对算真集合 Jaccard 取最大

    这是重写前那段代码的**语义**（倒排索引只是加速，不改变判决），因此可以当预言机。
    """
    detector = LeakageDetector(fields=fields, fuzzy_threshold=fuzzy_threshold)

    def sig(item):
        return detector._signature(item)

    train_sigs = [sig(i) for i in train_items]
    exact, fuzzy = 0, 0
    for item in test_items:
        s = sig(item)
        if not s:
            continue
        if s in train_sigs:
            exact += 1
            continue
        a = _char_trigrams(s)
        best = 0.0
        for t in train_sigs:
            if not t:
                continue
            sim = brute_force_jaccard(a, _char_trigrams(t))
            if sim > best:
                best = sim
        if best >= fuzzy_threshold:
            fuzzy += 1
    return exact, fuzzy


# 混合语料：精确泄漏 / 近似泄漏（改后缀）/ 局部改写 / 完全不同 / 空串 / 短于三元组
CORPUS = [
    {"instruction": "请解释一下什么是过拟合现象"},
    {"instruction": "请解释一下什么是过拟合现象呀"},       # 近似（高相似）
    {"instruction": "请解释一下什么是过拟合"},             # 近似（截断）
    {"instruction": "写一个Python函数用来读取CSV文件"},
    {"instruction": "写一个Java类用来解析XML配置文件"},     # 共享片段但整体不同
    {"instruction": "今天天气很好适合出去散步"},
    {"instruction": ""},                                  # 空签名
    {"instruction": "ab"},                                # 短于三元组
    {"instruction": "简述梯度下降的推导过程并说明学习率的影响"},
    {"instruction": "简述梯度下降的推导过程并说明学习率的影响。"},  # 归一化后与上条同签名
]


def split(corpus, n_train):
    return corpus[:n_train], corpus[n_train:]


class TestEquivalenceToBruteForce:
    """重写后的判决必须与「逐对真集合 Jaccard」完全一致"""

    @pytest.mark.parametrize("n_train", [1, 3, 5, 7])
    @pytest.mark.parametrize("threshold", [0.3, 0.7, 0.9])
    def test_verdicts_match_brute_force(self, n_train, threshold):
        train, test = split(CORPUS, n_train)
        report = LeakageDetector(fuzzy_threshold=threshold).detect(train, test)
        assert (report.exact_leaks, report.fuzzy_leaks) == legacy_detect(
            train, test, ["instruction"], threshold)

    @pytest.mark.parametrize("threshold", [0.0, -1.0])
    def test_non_positive_threshold_keeps_legacy_verdict(self, threshold):
        """阈值 <= 0 时旧实现把每条非精确样本都判为 fuzzy（best 从 0.0 起算）

        这个判决在数学上没意义（任何相似度都 >= 0），但 API/CLI 的 fuzzy_threshold
        没有范围校验，所以 0 与负数是可到达的输入；重写必须保持而不是顺手改掉。
        注意「阈值极小但为正」不在此列 —— 两版都要求 best >= t，与一条元组都不共享
        的样本本来就报不出来，这是旧行为的一部分（Temp/l95q 探针在 1e-9 档同样两版一致）。
        """
        train, test = split(CORPUS, 4)
        detector = LeakageDetector(fuzzy_threshold=threshold)
        report = detector.detect(train, test)
        # 空签名条目不参与比较（两版都跳过），其余每条必被判为 exact 或 fuzzy
        comparable = sum(1 for i in test if detector._signature(i))
        assert report.total_leaks == comparable


class TestPruneIsAdmissible:
    """剪枝 `I >= int(t*|a|)` 只许「少剪」，不许误剪"""

    def test_boundary_pair_below_need_is_still_correct(self):
        """构造 I 恰好等于 need-1 的一对：此时 J 必然 < t，剪掉它与不剪它同判"""
        a = set("abcdefghij")          # |a| = 10
        b = set("abcdefgh") | set("xyz")  # I = 8, |b| = 11
        assert len(a & b) == 8
        threshold = 0.9
        need = int(threshold * len(a))  # 9
        assert len(a & b) == need - 1
        # 真 Jaccard = 8/13 ≈ 0.615 < 0.9 —— 被剪掉是正确判决
        assert brute_force_jaccard(a, b) < threshold

    def test_boundary_pair_at_need_is_not_missed(self):
        """构造 I 恰好等于 need 的一对：即便真相似度低于阈值（不该报），也必须走完比较"""
        a = set("abcdefghij")          # |a| = 10
        b = set("abcdefghij") | set("klmnopqrst")  # |b| = 20，I = 10
        threshold = 1.0
        need = int(threshold * len(a))
        assert len(a & b) == need == 10
        assert len(a | b) == 20
        # 真 Jaccard = 10/20 = 0.5 < 1.0 —— 过了剪枝仍要被比较表否掉
        assert brute_force_jaccard(a, b) < threshold
        assert _jaccard_from_counts(len(a & b), len(a), len(b)) == 0.5

    def test_exact_tie_pair_is_not_pruned(self):
        """I 恰等于 need 且真相似度恰等于阈值的「双临界对」必须照报

        这是 L30 浮点平局教训的正例：训练串是测试串的子串（10 个 vs 8 个三元组），
        |a|=10、t=0.8 ⇒ need=int(0.8*10)=8=I，且 J=8/10=0.8=t。任何把剪枝写严
        （`>`）或把取整写成 ceil 的改动都会在这里判红。
        """
        train = [{"instruction": "abcdefghij"}]        # 8 个三元组
        test = [{"instruction": "abcdefghijkl"}]        # 10 个三元组，含训练的全部
        report = LeakageDetector(fuzzy_threshold=0.8).detect(train, test)
        assert report.fuzzy_leaks == 1
        assert brute_force_jaccard(_char_trigrams("abcdefghijkl"),
                                   _char_trigrams("abcdefghij")) == 0.8

    def test_prune_skips_something_it_should(self, monkeypatch):
        """剪枝必须**真的在剪**：候选全比的写法与不剪同义，那这条优化就没有守卫"""
        calls = {"n": 0}
        real = _jaccard_from_counts

        def spy(inter, sa, sb):
            calls["n"] += 1
            return real(inter, sa, sb)

        monkeypatch.setattr("augmentor.leakage._jaccard_from_counts", spy)
        train = [{"instruction": f"共享前缀 与各自不同的尾部 {i:03d} 号"} for i in range(60)]
        test = [{"instruction": "共享前缀 与各自不同的尾部 007 号呀"}]
        detector = LeakageDetector(fuzzy_threshold=0.8)
        n_pairs = sum(1 for _ in test) * len(train)
        detector.detect(train, test)
        assert calls["n"] > 0, "对照：一次都没比过说明这条尺子看不见热路径"
        assert calls["n"] < n_pairs, "剪枝没生效：候选被全量逐对比较了"

    def test_float_tie_never_under_counts(self):
        """t*|a| 落在浮点边界上时，int() 向零截断只会让 need 偏小（少剪），不会偏大

        业务含义：J = I/U 且 U >= |a| ⇒ J <= I/|a|。所以只要 I < need，就有
        I/|a| <= (need-1)/|a| < t ⇒ 真相似度必低于阈值，剪掉它不会误杀。
        """
        for size_a in range(1, 200):
            for t in (0.1, 0.3, 0.7, 0.8, 0.9, 0.99):
                need = int(t * size_a)
                assert need <= t * size_a < need + 1
                if need > 0:
                    assert (need - 1) / size_a < t


class TestCountingFormEquivalence:
    """按计数算式与集合外壳必须逐值相同 —— 前者是热路径唯一权威"""

    @pytest.mark.parametrize("a,b", [
        (set(), {"x"}),
        ({"x"}, set()),
        ({"x"}, {"x"}),
        ({"x"}, {"y"}),
        (set("abc"), set("bcd")),
        (set("abcdef"), set("fedcba")),
    ])
    def test_shell_matches_brute_force(self, a, b):
        assert _jaccard(a, b) == brute_force_jaccard(a, b)

    @pytest.mark.parametrize("inter,sa,sb", [
        (0, 3, 4), (3, 3, 3), (1, 5, 5), (2, 2, 8), (0, 0, 0),
    ])
    def test_counts_matches_brute_force(self, inter, sa, sb):
        a = set(range(sa))
        b = set(range(inter)) | set(range(sa, sa + sb - inter))
        assert len(a & b) == inter and len(a | b) == sa + sb - inter
        assert _jaccard_from_counts(inter, sa, sb) == brute_force_jaccard(a, b)


# 专供「热路径」两支护卫的语料：训练侧**不含**变体，测试侧全是变体 ⇒ 一定有候选活到比较
# 阶段。上一版直接复用 CORPUS*4，训练侧把变体也收了进去，于是 32 条测试样本全是 exact、
# 一次比较都没发生 —— 「0 次集合运算」是空转出来的绿（本轮变异取证 M1 抓出）。
FUZZY_TRAIN = [
    {"instruction": "请解释一下什么是过拟合现象"},
    {"instruction": "简述梯度下降的推导过程并说明学习率的影响"},
]
FUZZY_TEST = [
    {"instruction": "请解释一下什么是过拟合现象呀"},
    {"instruction": "请解释一下什么是过拟合"},
    {"instruction": "简述梯度下降的推导过程并说明学习率"},
    {"instruction": "简述梯度下降的推导过程并说明学习率的影响呀"},
]


class TestHotPathIsSetOpFree:
    """热路径一次集合运算都不做：这是本轮性能主张的机械守卫，不是文档里的形容词"""

    def test_ruler_sees_real_set_ops(self):
        """对照组：尺子必须看得见它声称能看见的东西，否则下面的「0 次」是假的

        踩过的坑：`set.__and__` 返回**原生 set**，所以链式写 `(a&b)|(c|d)` 时第二个 `|`
        落在原生集合上、不计数 —— 每个算子必须单独在计数器实例上行使一次。
        """
        ops = _OpCounter.reset()
        a, b = _OpCounter("abc"), _OpCounter("bcd")
        _ = a & b
        _ = a | b
        _ = a.intersection(b)
        _ = a.union(b)
        assert ops == {"and": 1, "or": 1, "intersection": 1, "union": 1}

    def test_detect_performs_no_set_ops_on_the_hot_path(self, monkeypatch):
        _OpCounter.reset()
        real = _char_trigrams
        comparisons = {"n": 0}
        real_j = _jaccard_from_counts

        def counting_trigrams(text):
            return _OpCounter(real(text))

        def counting_jaccard(inter, sa, sb):
            comparisons["n"] += 1
            return real_j(inter, sa, sb)

        monkeypatch.setattr("augmentor.leakage._char_trigrams", counting_trigrams)
        monkeypatch.setattr("augmentor.leakage._jaccard_from_counts", counting_jaccard)
        LeakageDetector(fuzzy_threshold=0.8).detect(FUZZY_TRAIN, FUZZY_TEST)
        # 先证明这次跑真的走到了比较阶段，否则「0 次集合运算」可能只是什么都没做
        assert comparisons["n"] > 0, "语料里没有一个候选活过剪枝 —— 这把尺子在空转"
        # 剪枝与倒排索引只数个数、取长度；任何一次 & | 都说明「按计数」那条路被换掉了
        assert _OpCounter.ops == {"and": 0, "or": 0, "intersection": 0, "union": 0}

    def test_trigrams_computed_once_per_signature(self, monkeypatch):
        """旧实现每条候选测试样本算两遍三元组（一遍取候选、一遍取集合）——重写后一遍"""
        calls = {"n": 0}
        real = _char_trigrams

        def spy(text):
            calls["n"] += 1
            return real(text)

        monkeypatch.setattr("augmentor.leakage._char_trigrams", spy)
        train, test = split(CORPUS * 4, 16)
        sig = LeakageDetector()._signature
        train_uniq = {sig(i) for i in train} - {""}
        # 训练侧按去重后的签名各算一次；测试侧每条非精确样本各算一次（重复条目不共享）
        test_nonexact = [sig(i) for i in test if sig(i) and sig(i) not in train_uniq]
        LeakageDetector(fuzzy_threshold=0.7).detect(train, test)
        assert calls["n"] == len(train_uniq) + len(test_nonexact)


class _OpCounter(set):
    """记录实例上 & | intersection union 次数的集合子类（set 是内置类型，只能这样插表）"""

    ops: dict = {}

    @classmethod
    def reset(cls):
        cls.ops = {"and": 0, "or": 0, "intersection": 0, "union": 0}
        return cls.ops

    def __and__(self, other):
        type(self).ops["and"] += 1
        return set.__and__(self, other)

    def __or__(self, other):
        type(self).ops["or"] += 1
        return set.__or__(self, other)

    def intersection(self, other):
        type(self).ops["intersection"] += 1
        return set.intersection(self, other)

    def union(self, other):
        type(self).ops["union"] += 1
        return set.union(self, other)


class TestPeakMemoryIsBounded:
    """峰值内存：旧版为每个训练签名另存一份三元组集合，重写后只剩「个数」"""

    def test_peak_mib_under_ceiling(self):
        import tracemalloc

        train = [{"instruction": f"训练样本编号 {i:05d} 号的问题描述文本"}
                 for i in range(1200)]
        test = [{"instruction": f"测试样本编号 {j:05d} 号的问题描述文本"}
                for j in range(300)]
        tracemalloc.start()
        try:
            LeakageDetector(fuzzy_threshold=0.8).detect(train, test)
            peak = tracemalloc.get_traced_memory()[1]
        finally:
            tracemalloc.stop()
        # 同语料两版实测：HEAD 3.30 MiB / 工作树 1.23 MiB（Temp/l95q 探针）。
        # 上界 2.5 离工作树有一倍余量（容解释器抖动），离 HEAD 也有一倍余量（拦回退）。
        assert peak / 1048576 < 2.5


class TestPublicApiUnchanged:
    """模块级入口与报告形状不因内部重写而漂移"""

    def test_module_function_still_reports(self):
        train, test = split(CORPUS, 3)
        report = detect_leakage(train, test, fuzzy_threshold=0.7)
        assert report.train_size == 3 and report.test_size == len(test)
        assert set(report.to_dict()) == {
            "train_size", "test_size", "exact_leaks", "fuzzy_leaks",
            "total_leaks", "leak_rate", "is_clean", "leaked_examples",
        }

    def test_leaked_examples_still_truncated_by_min_examples(self):
        train = [{"instruction": f"共同前缀 {i} 号样本的详细问题描述"} for i in range(5)]
        test = [{"instruction": f"共同前缀 {i} 号样本的详细问题描述X"} for i in range(20)]
        report = LeakageDetector(fuzzy_threshold=0.7, min_examples=3).detect(train, test)
        assert len(report.leaked_examples) == 3
