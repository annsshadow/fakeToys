# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""「比例 → 条数」的配额分配测试（L36 口径：余数必须有确定归属）

钉住六条主张：

1. **分配之和 == 请求条数**。`int(total * 比例)` 逐份下取整会少交：等权 4 组要 7 条，
   实测 HEAD 逐组 `int(1.75)` → 只交回 **4** 条，报告照写「采样完成」。
2. **余数不得整份偏给某一份**。`test = data[a:]` 让名义 10% 的测试集在 7 条数据上
   实际拿到 **2 条（28.6%）**，而名义 10% 的验证集在 n ≤ 9 时交出 **0 条**。
3. **每段与名义值之差 < 1 条**。这是最大余数法的保证，也是主张 2 的可判定形式。
4. **平局按输入顺序，同一份输入永不换答案**。HEAD 的「先每组至少 1 条（超发）再
   `rng.sample` 随机砍」让 20 个单条组要 5 条时三个种子交出三组互不相干的类别 ——
   「分层」退化成「随机挑组」。
5. **可整除时行为不变**（改分配口径不许顺手改掉正常路径）。
6. **让渡一次算清，代价不随 `total` 放大**。装不下的那份把名额交给还有空间的段，
   轮数只与**份数**同阶：1e9 条的名额与 7 条的名额走同一套算式。
"""
import random

import pytest

from augmentor.allocation import largest_remainder
from augmentor.aggregator import DataAggregator
from augmentor.data_splitter import DataSplitter
from augmentor.dataset_ops import DatasetOperations, SampleConfig, SplitConfig

RATIOS = (0.8, 0.1, 0.1)


def cat_rows(counts):
    """按 [(类别, 条数)] 造数据；`category` 整体短于 10 字符，组键不会被截断改写"""
    out = []
    for name, n in counts:
        out.extend({"category": name, "instruction": f"{name}第{i}问", "output": f"答{i}"}
                   for i in range(n))
    return out


def per_category(items):
    counts = {}
    for item in items:
        counts[item["category"]] = counts.get(item["category"], 0) + 1
    return counts


def shares(total, weights):
    """名义应得的浮点份数"""
    s = sum(weights)
    return [total * w / s for w in weights]


class TestLargestRemainderCore:
    """分配器本体的六条性质"""

    @pytest.mark.parametrize("total,weights", [
        (7, [1, 1, 1]), (7, [1, 1, 1, 1]), (1, [1, 1, 1]),
        (13, [0.8, 0.1, 0.1]), (9, [0.8, 0.1, 0.1]), (25, [2, 1, 1]),
        (0, [1, 1]), (3, [0, 0, 0]),
    ])
    def test_sum_is_the_requested_total(self, total, weights):
        assert sum(largest_remainder(total, weights)) == total

    @pytest.mark.parametrize("total,weights", [
        (7, [1, 1, 1, 1]), (13, [0.8, 0.1, 0.1]), (1, [1, 1, 1]), (99, [3, 5, 7]),
    ])
    def test_every_share_stays_within_one_row_of_nominal(self, total, weights):
        nominal = shares(total, weights)
        for got, want in zip(largest_remainder(total, weights), nominal):
            assert abs(got - want) < 1, f"{got} 偏离名义 {want:.2f} 超过 1 条"

    def test_ties_resolve_by_input_order(self):
        """20 个单条组只要 5 条：谁进谁出是平局，按输入顺序取前 5 组（确定、可复现）"""
        quotas = largest_remainder(5, [1] * 20, caps=[1] * 20, minimum_each=1)
        assert quotas == [1, 1, 1, 1, 1] + [0] * 15

    def test_caps_hand_the_leftover_to_the_next_part(self):
        assert largest_remainder(10, [1, 1], caps=[3, 20]) == [3, 7]

    def test_all_caps_full_is_the_only_honest_shortfall(self):
        """分不出 10 条时交回 6 条而不是硬凑 —— 没有更多东西可分"""
        assert largest_remainder(10, [1, 1], caps=[3, 3]) == [3, 3]

    def test_surplus_goes_to_the_part_with_room_even_at_a_huge_total(self):
        """让渡不是逐条挪：1e9 条的名额也一样一次算清

        钉的是复杂度背后的那条性质——结果只跟**份数**有关，不跟 `total` 有关。
        逐条补余数的写法在这里要多跑约 3.3 亿轮（实测 1e6 规模 67 ms，
        而 1e9 规模直接成为调用方挂死），`/api/dataset/aggregate` 的
        `target_size` 只有下界没有上界，所以这条是用户可打到的路径。
        """
        assert largest_remainder(10 ** 6, [1, 1, 1],
                                 caps=[1, 1, 10 ** 6]) == [1, 1, 999998]
        quotas = largest_remainder(10 ** 9, [1, 1, 1], caps=[1, 1, 10 ** 9])
        assert quotas == [1, 1, 999999998]
        assert sum(quotas) == 10 ** 9

    def test_no_items_no_quota(self):
        assert largest_remainder(5, []) == []

    def test_negative_weight_is_read_as_zero(self):
        assert largest_remainder(4, [-1, 1]) == [0, 4]

    def test_minimum_each_is_dropped_when_the_pool_cannot_cover_it(self):
        """名额比份数还少时，`minimum_each` 不能变成「每份都塞一条」的超发理由"""
        assert sum(largest_remainder(3, [1] * 8, minimum_each=1)) == 3

    def test_minimum_each_is_honored_when_the_pool_covers_it(self):
        """名额盖得住份数时，`minimum_each` 要真的把每份都塞到 1 条

        实测钉的是「权重悬殊也能到」：`[100, 1, 1]` 的名义份数是 6.86/0.07/0.07，
        纯下取整 + 补余数会让前一份吃掉全部 7 条，后两份是 0 条 ——
        而 HEAD 在分层采样里正是靠「先每组超发 1 条再随机砍」来避免这种缺组。
        """
        assert largest_remainder(7, [100, 1, 1], minimum_each=1) == [5, 1, 1]
        assert largest_remainder(7, [100, 1, 1]) == [7, 0, 0]

    def test_evenly_divisible_split_is_untouched(self):
        assert largest_remainder(8, [1, 1, 1, 1]) == [2, 2, 2, 2]
        assert largest_remainder(12, [0.5, 0.25, 0.25]) == [6, 3, 3]


class TestAggregateWeightedQuota:
    """`aggregate_weighted` 的逐源配额"""

    @staticmethod
    def _sources(per_source=10, names=("s0", "s1", "s2")):
        # 内容必须逐源不同，否则 `_key_of` 去重会让后两个源一条都进不来
        return {name: [{"instruction": f"{name}第{j}问租房", "output": f"答{j}"}
                       for j in range(per_source)] for name in names}

    @pytest.mark.parametrize("target", [1, 2, 5, 7, 10, 17, 25, 29])
    def test_requested_total_is_delivered(self, target):
        """实测 HEAD：3 源 x10 等权时 30 个 target 里 **20 个** 少交，
        最重的是 `target_size=1` 与 `2` → 三源各 `int(0.33)` 交出 **0 条**"""
        result = DataAggregator().aggregate_weighted(self._sources(), target_size=target)
        assert result.total_count == target

    def test_equal_weights_still_split_the_remainder_towards_the_first_source(self):
        result = DataAggregator().aggregate_weighted(self._sources(), target_size=7)
        assert sum(result.source_counts.values()) == 30
        assert result.total_count == 7

    def test_unequal_weights_keep_their_proportions(self):
        sources = self._sources(per_source=20)
        result = DataAggregator().aggregate_weighted(
            sources, weights={"s0": 2, "s1": 1, "s2": 1}, target_size=14)
        # 名义 7/3.5/3.5 → 最大余数：3.5 与 3.5 打平，按输入顺序先补 s1
        assert result.total_count == 14
        assert self._by_source(result.aggregated) == {"s0": 7, "s1": 4, "s2": 3}

    @staticmethod
    def _by_source(items):
        """按 `instruction` 前缀还原每条来自哪个源（`source_counts` 给的是源大小）"""
        counts = {"s0": 0, "s1": 0, "s2": 0}
        for item in items:
            counts[item["instruction"][:2]] += 1
        return counts

    def test_target_beyond_the_pool_is_capped_by_source_sizes(self):
        """3 源各 10 条要 25 条：可以满配，但任何一源不得超过它自己的条数"""
        result = DataAggregator().aggregate_weighted(self._sources(), target_size=25)
        assert result.total_count == 25

    def test_evenly_divisible_quota_is_unchanged(self):
        """反向护栏：12 条三等分仍是每源 4 条 —— 换分配口径不许改掉能整除的常规路径"""
        result = DataAggregator().aggregate_weighted(self._sources(), target_size=12)
        assert result.total_count == 12


class TestStratifiedSampleQuota:
    """`sample(method="stratified")` 的逐组配额"""

    @staticmethod
    def _sample(counts, size, seed=42):
        return DatasetOperations().sample(
            cat_rows(counts),
            SampleConfig(method="stratified", size=size, seed=seed,
                         stratify_key="category"))

    def test_requested_size_is_delivered(self):
        """实测 HEAD：等权 4 组要 7 条只交回 **4** 条（逐组 `int(1.75)`），少 3 条无提示"""
        for counts, size in [([(f"g{i}", 5) for i in range(4)], 7),
                             ([("a", 10), ("b", 5), ("c", 5)], 7),
                             ([("a", 3), ("b", 3)], 5),
                             ([("a", 7), ("b", 7), ("c", 6)], 10)]:
            assert len(self._sample(counts, size)) == size, f"{counts} 要 {size} 条"

    def test_equal_groups_share_the_remainder_by_input_order(self):
        assert per_category(self._sample([(f"g{i}", 5) for i in range(4)], 7)) == {
            "g0": 2, "g1": 2, "g2": 2, "g3": 1}

    def test_every_group_is_represented_when_the_pool_allows_it(self):
        """保留 HEAD `max(1, ...)` 想要的「每组都到」，但不再靠超发 + 随机砍"""
        got = self._sample([(f"g{i}", 5) for i in range(4)], 8)
        assert per_category(got) == {f"g{i}": 2 for i in range(4)}

    def test_a_dominant_group_does_not_eat_the_small_groups(self):
        """组大小悬殊（10/1/1）要 7 条时，两个小组仍各得 1 条

        缺陷态（本轮的分配器若漏掉 `minimum_each`）：前一组按名义 5.83 条取整 +
        补余数吃掉全部 7 条，后两组 0 条 —— 「分层」在悬殊分布下又退化成一刀切。
        """
        got = self._sample([("big", 10), ("small", 1), ("tiny", 1)], 7)
        assert per_category(got) == {"big": 5, "small": 1, "tiny": 1}

    def test_single_row_groups_are_not_randomly_repicked_per_seed(self):
        """实测 HEAD：20 个单条组要 5 条，三个种子交出三组互不相干的类别"""
        counts = [(f"g{i}", 1) for i in range(20)]
        picked = [sorted(per_category(self._sample(counts, 5, seed=seed)))
                  for seed in (42, 7, 2024)]
        assert picked[0] == picked[1] == picked[2]
        assert picked[0] == [f"g{i}" for i in range(5)]

    def test_size_above_the_dataset_still_returns_everything(self):
        """反向护栏：`size` 超量时截到全量是库层既定语义，分配器不得改成报错"""
        counts = [(f"g{i}", 3) for i in range(5)]
        assert len(self._sample(counts, 999)) == 15


class TestSplitQuota:
    """train / val / test 三段的余数归属"""

    @staticmethod
    def _split(n, ratios=RATIOS):
        items = [{"instruction": f"第{i}问", "output": f"答{i}"} for i in range(n)]
        return DatasetOperations().split(
            items, SplitConfig(ratios=ratios, shuffle=False, seed=7))

    @pytest.mark.parametrize("n", [1, 2, 5, 7, 9, 13, 17, 19, 20])
    def test_sizes_sum_up_and_stay_within_one_row(self, n):
        train, val, test = self._split(n)
        sizes = (len(train), len(val), len(test))
        assert sum(sizes) == n
        for got, want in zip(sizes, shares(n, RATIOS)):
            assert abs(got - want) < 1, f"n={n} 交出 {sizes}，名义 {want:.2f}"

    def test_val_is_not_silently_empty_on_a_seven_row_dataset(self):
        """实测 HEAD：n ≤ 9 时 0.1 的验证集交出 **0** 条，用户没要过空验证集"""
        train, val, test = self._split(7)
        assert (len(train), len(val), len(test)) == (5, 1, 1)
        assert val and test

    def test_test_set_no_longer_absorbs_every_remainder(self):
        """实测 HEAD：n=7 的测试集实际占比 28.6%（名义 10%）"""
        _, _, test = self._split(7)
        assert len(test) / 7 < 0.2

    def test_divisible_case_is_unchanged(self):
        """反向护栏：10 条 0.8/0.1/0.1 仍是 (8, 1, 1)"""
        train, val, test = self._split(10)
        assert (len(train), len(val), len(test)) == (8, 1, 1)

    def test_data_splitter_agrees_with_dataset_ops(self):
        """两个划分器过去各写一遍 `int()`，同一份输入不得再给出两种答案"""
        for n in range(1, 21):
            items = [{"instruction": f"第{i}问", "output": f"答{i}"} for i in range(n)]
            mine = DataSplitter(train_ratio=0.8, val_ratio=0.1, test_ratio=0.1,
                                seed=7).split(items)
            train, val, test = self._split(n)
            assert (len(mine.train), len(mine.val), len(mine.test)) == (
                len(train), len(val), len(test)), f"n={n} 两个划分器答案不同"

    def test_random_splitter_does_not_prefer_a_segment_by_coincidence(self):
        """段内元素必须是同一份 rng 的产物，不得出现重复或丢条"""
        items = [{"instruction": f"第{i}问", "output": f"答{i}"} for i in range(13)]
        result = DataSplitter(seed=7).split(items)
        merged = result.train + result.val + result.test
        assert len({id(x) for x in merged}) == 13
        assert {x["instruction"] for x in merged} == {x["instruction"] for x in items}


class TestNoProcessRngPollution:
    """分配改成确定性的同时，不得把随机性退回进程级 RNG"""

    def test_stratified_sample_leaves_the_global_rng_untouched(self):
        random.seed(7)
        before = random.random()
        random.seed(7)
        DatasetOperations().sample(
            cat_rows([(f"g{i}", 4) for i in range(5)]),
            SampleConfig(method="stratified", size=6, seed=3, stratify_key="category"))
        assert random.random() == before
