# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""分析模块热路径的复杂度门禁

`DatasetAnalyzer._calculate_diversity_score()` 曾把均值写在生成器里
（`sum((l - sum(lengths)/len(lengths))**2 ...)`），于是每比较一个长度就重算一次
全体长度的和 —— 整体退化成 O(n²)。

## 为什么判据从「绝对墙钟」换成「同进程翻倍比」（A157 的 ①）（**L239 退役，见文末 A286 那节**）

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
形状中间」冻成断言，所以它不再只是一张表。（**这张表与它所冻的那个 `RATIO_CEILING`
用例都在 L239 退役**，上两段是退役前的读数、留着作历史——3.0 两侧余量量的是安静机器，
门禁负载下比值实测冲到 4.38，比退化档最小读数 3.78 还大。）


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

## A286 收口（L239）：复杂度判据从墙钟比值换成「内层重算次数」

上面那张表把比值判据钉在 3.0，两侧余量各 18 % / 26 % —— **在安静机器上成立，在门禁
负载下不成立**。L227 全量复跑实测到一次假红：`stays_linear` 那条比值量到 **4.38**，
比退化档最小读数 3.78 还大，而同一条用例本轮第一次全量是绿的、隔离复跑四次全绿；
红那一次整轮 257.50 s、绿那次 188.88 s（高 36 %）。这就是 A286 立的案：**退化与噪声
在墙钟上是同一个数**，所以它既会假红、也会把真退化放过去。

本轮按 A286 给的候选 ② 落地（"②才是判据，①只是让它稳"）：

* **判据本体换成计数**——`_calculate_diversity_score()` 整趟里 `sum` 的调用数，实测
  n=1000 / 2000 / 10000 / 20000 **四档同为 2**（一次求均值、一次求方差和），n=0 走空档
  早退 ⇒ 0。计数不受负载、频率调节、别的进程影响，也**不可能**被 O(n²) 的实现满足：
  退化写法是「每个长度重算一次全体之和」⇒ 调用数随 n 增长。
* **反恒真也换成计数**——`_quadratic_diversity()` 复刻同一缺陷，实测 5000 档 **5001** 次、
  10000 档 **10001** 次（= n+1），随规模线性增长。两侧都是确定值，比值那张表从此不需要了。
* **留下计时的那一轴**：`ABSOLUTE_BACKSTOP_MS` 是 20 倍松的兜底（现实现安静跑约 70 ms），
  负载推不到它，它管的是「复杂度没变但绝对值炸了」这一类计数看不见的事故；
  `test_the_timer_can_see_scale` 那根把手跟着它一起留，理由就是上面那一格自己写的口径：
  没有随规模变化的把手的计时一律作废，那根把手不能因为比值退役就一起拆走。
* **退役的是比值**：`RATIO_CEILING` / `MIN_MEANINGFUL_MS` / `test_ratio_ceiling_separates_linear_from_quadratic`
  一并撤下。撤下不是放宽——同一件事从此由「比值 < 3.0」变成「调用数 == 2」，是**统计判据
  换成精确判据**；代价是这一维只盯得住「内层重算」这一族退化（别族的退化由那 20 倍兜底
  与语义用例负责），这个取舍按 A286 的原文方向做，不另立口径。
* 计数用的是 `builtins.sum` 的临时替身，`finally` 复原并**当场断言复原成功**：泄漏一个
  计数替身进后面的用例会静默改掉全仓所有 `sum` 的语义，那是比本文件原案更贵的一次事故。
"""

import builtins
import random
import string
import time

import pytest

from augmentor.analytics import DatasetAnalyzer

#: 两档规模。LARGE 沿用旧版那 20000 条，SMALL 是它的**前缀**（同一颗随机种子按序
#: 生成 ⇒ 前 10000 条与 `_synthetic_items(10000)` 逐条相同），所以判据里只剩规模差，
#: 没有「两档数据分布不同」这一层噪声。
N_SMALL, N_LARGE = 10000, 20000
#: **现量**（L239，A286 收口后这一维的判据就是它，不再看墙钟比值）：
#: `_calculate_diversity_score()` 整趟里 `sum` 的调用数，n=1000 / 2000 / 10000 / 20000
#: 四档**同为 2**（一次求均值、一次求方差和），n=0 走空档早退 ⇒ 0。
#: 这条不随负载、主频、别的进程变化，而 O(n²) 的写法**不可能**满足它（见下面牙齿那节）。
MEASURED_SUM_CALLS = 2
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


_REAL_SUM = builtins.sum


class _SumCounter:
    """`builtins.sum` 的临时替身：原样转发真函数，只多记一次调用

    转发必须**完全等价**（含 `sum(iterable, start)` 的两参写法），否则计数判据量的
    是替身而不是产品；上面那条「替身不改结果」的断言就是钉这件事的。
    """

    def __init__(self):
        self.calls = 0

    def __call__(self, iterable, *args):
        self.calls += 1
        return _REAL_SUM(iterable, *args)


def counted_sum_calls(fn):
    """在 `builtins.sum` 换成计数替身的窗口里跑 `fn`，返回 `(调用数, 返回值)`

    复原写在 `finally`，并在返回前**当场断言已复原**：泄漏一个替身进后续用例，等于把
    全进程的 `sum` 换成多一层 Python 调用的包装，那是静默的口径污染，比本文件原案贵。
    """
    counter = _SumCounter()
    builtins.sum = counter
    try:
        result = fn()
    finally:
        builtins.sum = _REAL_SUM
        assert builtins.sum is _REAL_SUM, "builtins.sum 没复原成原函数"
    return counter.calls, result


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

    def test_inner_recompute_is_counted_not_timed(self):
        """判据本体：整趟只许求和 2 次，且**不随规模增长**（A286 收口，取代比值计时）

        退化写法是「每比较一个长度就重算一次全体之和」⇒ 求和次数随 n 增长；
        计数把它变成确定值，负载、主频、邻居进程都不在判据里。
        """
        items = _synthetic_items(N_LARGE)
        small = items[:N_SMALL]
        large_analyzer = DatasetAnalyzer(items)
        small_analyzer = DatasetAnalyzer(small)

        large_calls, counted_score = counted_sum_calls(large_analyzer._calculate_diversity_score)
        small_calls, _ = counted_sum_calls(small_analyzer._calculate_diversity_score)

        assert large_calls == MEASURED_SUM_CALLS, (
            f"n={N_LARGE} 的求和次数是 {large_calls}，现量口径是 {MEASURED_SUM_CALLS}"
            "（一次均值 + 一次方差和）⇒ 要么内层又在重算全体之和，要么这一维换了形状、"
            "常数要按现量重钉"
        )
        assert small_calls == MEASURED_SUM_CALLS, (
            f"n={N_SMALL} 量到 {small_calls} 次而 n={N_LARGE} 量到 {large_calls} 次："
            "求和次数随规模变化 ⇒ 每次比较都在重算全体长度之和，正是那个 O(n²) 缺陷"
        )
        # 尺子自证：替身只许记账，不许改结果（改了结果，上面两个数量的就不是产品了）
        assert counted_score == large_analyzer._calculate_diversity_score(), \
            "计数替身改变了返回值——判据量的是替身而不是被测实现"


class TestTheJudgementItself:
    """判据也要能失败 —— 否则它只是一段会跑的代码

    A157 的原案是「换成相对预算」，但相对预算自己可能是一条**永远不会红**的用例；
    L239 换成计数之后同一个坑换了形状：**「求和次数 == 2」也可能只是因为计数器没数到**。
    下面三条把「计数器有分辨力」「空档确实早退」「替身不污染全局」冻成断言。
    """

    def test_the_counter_sees_the_inner_recompute(self):
        """牙齿：`_quadratic_diversity` 复刻的退化写法必须被数成 **n+1** 次，且随 n 增长

        实测 5000 档 5001 次、10000 档 10001 次（外层一次方差和 + 每条目一次全体求和）。
        这条若绿而上一条也绿，判据才不是恒真：计数器看不见了内层重算的话，
        「等于 2」就是没数到而不是没有。
        """
        big = _synthetic_items(10000)
        for n in (5000, 10000):
            rows = big[:n]
            calls, _ = counted_sum_calls(lambda i=rows: _quadratic_diversity(i))
            assert calls == n + 1, (
                f"{n} 条的退化复刻只数到 {calls} 次 sum ⇒ 计数器没有分辨力，"
                "上面那条『等于 2』的判据作废"
            )

    def test_empty_items_short_circuit_before_any_sum(self):
        """空档走早退 ⇒ 一次求和都没有。这条同时钉住「计数不是恒 2」的另一侧"""
        calls, score = counted_sum_calls(DatasetAnalyzer([])._calculate_diversity_score)
        assert calls == 0, f"空数据集数到 {calls} 次求和，早退那一支没被走到"
        assert score == 0.0

    def test_the_real_builtin_is_restored_after_counting(self):
        """`builtins.sum` 在窗口外必须是原函数，且抛异常时也一样复原"""
        original = builtins.sum

        def boom():
            raise RuntimeError("计数窗口里抛错")

        try:
            counted_sum_calls(boom)
        except RuntimeError:
            pass
        else:
            pytest.fail("counted_sum_calls 没有把窗口里的异常放出来")
        assert builtins.sum is original, "替身泄漏进全局——后面的每条用例都在被它改写语义"


class TestTheTimerAxisIsStillGuarded:
    """计时那一轴只留「绝对值出事」这一件事（比值判据已在 A286 收口时退役）

    兜底松到不参与机器抖动（现实现安静跑约 70 ms，兜底 2000 ms = 20 倍以上），
    所以它不会像比值那样在门禁负载下假红；它管的是计数看不见的那一类事故
    ——复杂度没变、绝对值炸了（例如换成了每次调用都重编码的写法）。
    模块 docstring 里那格教训仍然生效：**没有随规模变化的把手的计时一律作废**，
    所以这根把手跟着兜底一起留。
    """

    def test_the_absolute_backstop_holds(self):
        items = _synthetic_items(N_LARGE)
        t_large = _elapsed_ms(DatasetAnalyzer(items)._calculate_diversity_score)
        assert t_large < ABSOLUTE_BACKSTOP_MS, (
            f"数量级出事：{t_large:.0f} ms（兜底 {ABSOLUTE_BACKSTOP_MS:.0f} ms，"
            "现实现安静跑约 70 ms）"
        )

    def test_the_timer_can_see_scale(self):
        """自检：同一份实现，4 倍数据必须明显更慢（否则上一条兜底是段会跑的空代码）"""
        items = _synthetic_items(8000)
        quarter = items[:2000]
        t_q = _elapsed_ms(DatasetAnalyzer(quarter)._calculate_diversity_score)
        t_f = _elapsed_ms(DatasetAnalyzer(items)._calculate_diversity_score)
        assert t_f / t_q > 1.5, f"2000→8000 只读出 {t_f / t_q:.2f} 倍，计时器没有分辨力"
