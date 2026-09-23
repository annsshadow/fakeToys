# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""分析模块热路径的复杂度门禁

`DatasetAnalyzer._calculate_diversity_score()` 曾把均值写在生成器里
（`sum((l - sum(lengths)/len(lengths))**2 ...)`），于是每比较一个长度就重算一次
全体长度的和 —— 整体退化成 O(n²)。实测 20000 条时该函数 **1043 ms**，把均值提到
循环外后同一份数据 **2 ms**。

这里用两条用例钉住这件事：一条锁**语义**（重构不能改分数），一条锁**量级**。

> **预算为什么取 500 ms 而不是 2 ms**
>
> 本仓库的 `pytest -q` 恒带覆盖率统计，trace 会把纯 Python 循环放大近 70 倍。
> 同一台机器上实测：线性实现 **143 ms**、退化的 O(n²) 实现 **1130 ms**。
> 预算 500 ms 卡在两者之间——比线性实现留 3.5 倍机器余量，又比退化实现快一倍，
> 所以只有复杂度真的回头才会变红，不会因为机器抖动误报。
"""

import random
import string
import time

from augmentor.analytics import DatasetAnalyzer


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

    def test_twenty_thousand_items_stays_linear(self):
        """20000 条的多样性计算必须远低于 O(n²) 的耗时（预算推导见模块 docstring）"""
        analyzer = DatasetAnalyzer(_synthetic_items(20000))

        start = time.perf_counter()
        score = analyzer._calculate_diversity_score()
        elapsed_ms = (time.perf_counter() - start) * 1000

        assert 0.0 <= score <= 1.0
        assert elapsed_ms < 500, f"多样性计算用了 {elapsed_ms:.1f} ms（退化到 O(n²) 时实测 1130 ms）"
