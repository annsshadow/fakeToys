# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""分析模块热路径的复杂度门禁

`DatasetAnalyzer._calculate_diversity_score()` 曾把均值写在生成器里
（`sum((l - sum(lengths)/len(lengths))**2 ...)`），于是每比较一个长度就重算一次
全体长度的和 —— 整体退化成 O(n²)。

## 为什么判据从「绝对墙钟」换成「同进程翻倍比」（A157 的 ①）

旧版是一条 `assert elapsed_ms < 500`。它测的不是复杂度，是**这台机器此刻给不给 CPU**：
L85 那轮全量里它就假红过一次（当时另有一条口径把它排除在终态证据之外，A157 因此立案）。
换成相对判据之后，机器快慢、负载高低都同时进分子分母，只剩「翻倍的数据要花几倍时间」
这一件事 —— 而那正是本用例想锁的东西。

## 阈值 3.0 的两边都有实测（不是推理）

安静侧与争用侧来自 `Temp/l89q/true_bench.py`（min-of-3 与 median-of-3 都跑过，表里给
中位数）；**退化那一侧最强的一条不是复刻，是变异探针**：`Temp/l89q/mutation_probe.py`
把产品函数里的均值真写回生成器（`analytics.py` 原始字节先进内存、跑完逐字节还原），
再跑本文件。

| 形状 | 口径 | 翻倍比读数 |
| --- | --- | --- |
| 线性（现实现，安静） | n=5000/10000/20000 = 15.2 / 32.4 / 73.3 ms | **1.76 – 2.54** |
| 线性（4 个自旋子进程争用） | 绝对值涨到 27.3 / 54.5 / 119.6 ms | **1.99 / 2.20** |
| 线性（带 coverage 跟踪，3.13 与 3.14） | 绝对值 59–60 / 122 ms（3.13）、70 ms（3.14） | **2.02 – 2.16** |
| **产品函数被变异成 O(n²)** | 10000→20000，不 tracing | **3.78** ⇒ 用例红 |
| 同上 | 带 tracing（3.14 / 3.13） | **4.76 / 7.53** ⇒ 用例红 |
| 二次复刻（本文件底部的 `_quadratic_diversity`） | 5000→10000 | **3.95 – 3.99** |

⇒ 绝对值在争用下被推到 **1.6–1.8 倍**，而比值几乎不动（2.12/2.27 → 1.99/2.20）；
tracing 把二次侧推得**更远**（3.78 → 7.53），所以跟踪只会让判据更灵、不会更钝。
线性侧最大读数 2.54、退化侧最小读数 3.78 ⇒ 阈值取 **3.0**，两边各留 ≥18 % 与 ≥26 %
的余量。`test_ratio_ceiling_separates_linear_from_quadratic` 把「这条阈值真的落在两种
形状中间」冻成断言，所以它不再只是一张表。


## 为什么还留一条极松的绝对兜底

纯比值对「复杂度没变、但绝对值炸了」变迟钝（A157 代价那一格自己写过）。所以另加
一条 `ABSOLUTE_BACKSTOP_MS`：它松到不参与机器抖动（现实现安静跑约 70 ms，兜底 20 倍），
只在数量级出事时红。两条各测一轴，别把兜底当预算。

## 旧版注释里的两个数，本轮按实测纠正

1. 「trace 会把纯 Python 循环放大近 70 倍」——**复现不出来**：同一份实现，3.13 上
   不 tracing 66–69 ms / 带 tracing 122 ms（×1.8），3.14 上 66–69 / 69–71 ms（×1.0–1.1）。
   旧注释那两个数是 143 ms（tracing）与 2 ms（修好之后），相除才得到 70 —— 那是
   **拿 tracing 与不 tracing 的两档相减之外，又假设它们是同一份数据**；本轮没去追
   2 ms 那档到底量的什么（原始数据已经追不回来），只把「放大 70 倍」这句不可复现的
   话从判据里撤掉。与 L88 那格「拿 `write_text` 的数去说 `save_config` 的窗口」同族。
2. 「均值提到循环外之后同一份数据 2 ms」——在这份合成数据上，线性实现 20000 条实测
   **66–70 ms**（两解释器、tracing 与否四格都是这个量级）。2 ms 那个数对应的不是这份
   数据分布，留在注释里会让下一轮以为还有一档 30 倍的可优化空间。

⇒ 换判据的同时把这两个数换掉，比「两个数都留着」好：本文件现在只引用**同一口径**
（同一函数、同一合成数据、min-of-3）量出来的读数。

## 本文件前身踩过的一格，写在这里当口径

第一版探针把「线性」量成了平的（三档全 ≈930 ms，比值 1.00），据此差点得出「比值判据
不成立」。真相是 `bench()` 写成 `min(fn()) * 1000` —— `fn` 返回的是**0~1 的分数**，
所以那三档「毫秒」是 0.932 / 0.930 / 0.926 分。三次旁证都在指向同一件事：两次进程
逐位相同、`gc.disable()` 一字不改、n 变 4 倍不动。**耗时不会跨进程逐位相同。**
⇒ 口径：**计时探针必须自带一根随被测规模变化的把手**（本文件的自检用例就是这根把手），
读数不随规模动的计时一律作废，不进任何判据。
"""

import random
import string
import time

from augmentor.analytics import DatasetAnalyzer

#: 两档规模。LARGE 沿用旧版那 20000 条，SMALL 是它的**前缀**（同一颗随机种子按序
#: 生成 ⇒ 前 10000 条与 `_synthetic_items(10000)` 逐条相同），所以比值里只剩规模差，
#: 没有「两档数据分布不同」这一层噪声。
N_SMALL, N_LARGE = 10000, 20000
#: 线性与二次的实测分界（见模块 docstring 的表）。
RATIO_CEILING = 3.0
#: 读数的下限：低于它，计时器分辨的是启动与调度噪声，不是算法。撞到它说明实现
#: 变快了（好事），处理方式是把两档规模一起放大，而不是放宽这条断言。
MIN_MEANINGFUL_MS = 10.0
#: 极松的绝对兜底，见模块 docstring。
ABSOLUTE_BACKSTOP_MS = 2000.0


def _synthetic_items(n: int, seed: int = 7):
    """造 n 条长度不等的合成条目（长度分布影响方差，固定随机种子保证可复现）"""
    random.seed(seed)
    return [
        {
            "instruction": " ".join(
                "".join(random.choices(string.ascii_lowercase, k=5))
                for _ in range(random.randint(1, 24))
            ),
            "output": "a",
        }
        for _ in range(n)
    ]


def _elapsed_ms(fn, reps: int = 3) -> float:
    """纯计算耗时，取 `reps` 次的**最小值**

    为什么是 min 而不是 median/mean：本函数要的是上界判据，一次被调度打断的读数
    会把比值推向假红，而 min 天然只认「这台机器能给的最快一次」。代价是它对小常数
    不敏感 —— 那部分由 `ABSOLUTE_BACKSTOP_MS` 与自检用例分别兜住。
    """
    samples = []
    for _ in range(reps):
        start = time.perf_counter()
        fn()
        samples.append((time.perf_counter() - start) * 1000)
    return min(samples)


def _quadratic_diversity(items):
    """退化实现的原样复刻（均值写在生成器里 ⇒ O(n²)），只给自检用例量另一侧

    复刻的形状必须与被描述的缺陷一致：写成「外层再套一遍 n」量到的是 O(n³)，
    那样得到的 4 倍以上是凭空造的数，不是阈值另一侧的证据。
    """
    instructions = [it["instruction"] for it in items]
    lengths = [len(inst) for inst in instructions]
    variance = sum((x - sum(lengths) / len(lengths)) ** 2 for x in lengths) / len(lengths)
    return variance ** 0.5 / 50


class TestDiversityScoreComplexity:
    """多样性分数必须是线性时间，且语义与重构前一致"""

    def test_score_matches_population_stddev(self):
        """手算对照：词汇多样性 0.6 权重 + 长度标准差/50 截断 0.4 权重"""
        items = [{"instruction": "aa bb", "output": "x"}, {"instruction": "cc", "output": "y"}]
        analyzer = DatasetAnalyzer(items)

        score = analyzer._calculate_diversity_score()

        words = analyzer._get_words("aa bb") + analyzer._get_words("cc")
        unique_ratio = len(set(words)) / len(words)
        lengths = [2 + 1 + 2, 2]
        mean = sum(lengths) / len(lengths)
        variance = sum((x - mean) ** 2 for x in lengths) / len(lengths)
        expected = min(max(unique_ratio * 0.6 + min(variance**0.5 / 50, 1.0) * 0.4, 0.0), 1.0)

        assert score == expected

    def test_doubling_the_dataset_stays_linear(self):
        """数据翻倍 ⇒ 耗时最多翻约两倍；退化成 O(n²) 时这条会到 4 倍并当场红"""
        items = _synthetic_items(N_LARGE)
        small = items[:N_SMALL]
        large_analyzer = DatasetAnalyzer(items)
        small_analyzer = DatasetAnalyzer(small)

        # 顺序：先量小档再量大档，让两档之间只隔一次分配，中间不夹别的负载。
        t_small = _elapsed_ms(small_analyzer._calculate_diversity_score)
        t_large = _elapsed_ms(large_analyzer._calculate_diversity_score)
        ratio = t_large / t_small

        assert t_small > MIN_MEANINGFUL_MS, (
            f"n={N_SMALL} 只量到 {t_small:.1f} ms，计时器分辨的是噪声不是算法；"
            "实现变快了就把两档规模一起放大，别放宽比值阈值"
        )
        assert ratio < RATIO_CEILING, (
            f"{N_SMALL}→{N_LARGE} 慢了 {ratio:.2f} 倍（线性实测 2.0–2.5，"
            f"退化到 O(n²) 实测 3.1–3.9）⇒ 每比较一个长度都在重算全体之和"
        )
        assert t_large < ABSOLUTE_BACKSTOP_MS, (
            f"复杂度没变但绝对值出事：{t_large:.0f} ms（兜底 {ABSOLUTE_BACKSTOP_MS:.0f} ms，"
            "现实现安静跑约 70 ms）"
        )


class TestTheJudgementItself:
    """判据也要能失败 —— 否则它只是一段会跑的代码

    A157 的原案是「换成相对预算」，但相对预算自己可能是一条**永远不会红**的用例：
    如果比值判据在退化实现上也成立，那 500 ms 换成比值只是把假红换成了假绿。
    这两条用例把「计时器有分辨力」和「阈值真能分辨两种形状」冻成断言。
    """

    def test_the_timer_can_see_scale(self):
        """自检：同一份实现，4 倍数据必须明显更慢（否则任何比值判据都是空的）"""
        items = _synthetic_items(8000)
        quarter = items[:2000]
        t_q = _elapsed_ms(DatasetAnalyzer(quarter)._calculate_diversity_score)
        t_f = _elapsed_ms(DatasetAnalyzer(items)._calculate_diversity_score)
        assert t_f / t_q > 1.5, f"2000→8000 只读出 {t_f / t_q:.2f} 倍，计时器没有分辨力"

    def test_ratio_ceiling_separates_linear_from_quadratic(self):
        """`RATIO_CEILING` 必须落在两种形状**中间**，不是凭手感

        复刻的 O(n²) 版本在同一口径下的翻倍比必须 ≥ 阈值；哪天有人把常数调到
        退化侧的读数之上，这条当场红 —— 这正是「阈值有实测间隔」的机械版本。
        规模取 5000→10000 而不是 1250→2500：小档上 O(n) 的那部分常数会把比值往下
        拽（实测 3.14），大档才贴近 4 的渐近值（实测 3.95–3.99），阈值余量才真实。
        """
        items = _synthetic_items(10000)
        half = items[:5000]
        t_half = _elapsed_ms(lambda: _quadratic_diversity(half), reps=3)
        t_full = _elapsed_ms(lambda: _quadratic_diversity(items), reps=3)
        assert t_full / t_half >= RATIO_CEILING, (
            f"O(n²) 复刻只量到 {t_full / t_half:.2f} 倍 ⇒ 阈值 "
            f"{RATIO_CEILING} 已经在退化侧之内，比值判据失去分辨力"
        )
