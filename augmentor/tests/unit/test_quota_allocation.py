# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""「比例 → 条数」的配额分配测试（L36 口径：余数必须有确定归属）

钉住九条主张：

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
7. **「每份上限都是 1」的快路与一般定义同答**（L37 口径：还上一轮的代价账，一行答案
   都不许改）。分层采样用 `instruction` 作组键时每组只有 1 条 ⇒ 份数 == 行数，
   逐段下取整 + 排序在 6902 段上是 6.8 ms；快路换成「按权重降序、平局按输入顺序取
   前 `total` 份」，所以这里钉的是换算法不换答案，而不只是换算法不换总数。
8. **`caps=None` 且无保底的请求走单轮快路，且与多轮通用路径逐段同答**（L39 口径：
   分层摊派的每组一次调用是 A53 的主要代价，快路把三段摊派从 3.632 µs 降到 1.804 µs）。
   准入条件必须同时看两个参数：只看 `caps` 会吞掉 `minimum_each`，只看 `minimum_each`
   会把让渡请求当成无上限。
9. **`minimum_each` 的准入读「保底之和装不装得下」**（L40 口径：A49/A55）。旧判据读
   「名额盖得住份数」，在 399,594 例契约合法穷举里超发 **24,889** 例、误拒 **261** 例。
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
        """20 个单条组只要 5 条：谁进谁出是平局，按输入顺序取前 5 组（确定、可复现）

        L37 起这一形状走的是「上限全 1」快路，一般路径的平局次序由
        `test_general_path_ties_resolve_by_input_order` 另行钉住。
        """
        quotas = largest_remainder(5, [1] * 20, caps=[1] * 20, minimum_each=1)
        assert quotas == [1, 1, 1, 1, 1] + [0] * 15

    def test_general_path_ties_resolve_by_input_order(self):
        """不带 `caps` 的形状不走快路，平局仍按输入顺序补

        钉的是「小数部分相同 ⇒ 先出现的先补」：4 段等权要 7 条，下取整各 1 条后
        剩 3 条名额，四段的小数部分全是 0.75 ⇒ 前 3 段各 +1。次序若改成按权重表
        原地遍历的倒序或按段长，答案会变成别的三元组。
        """
        assert largest_remainder(7, [1, 1, 1, 1]) == [2, 2, 2, 1]
        assert largest_remainder(5, [2, 2, 1]) == [2, 2, 1]
        assert largest_remainder(9, [1] * 6) == [2, 2, 2, 1, 1, 1]

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


class TestUnitCapFastPath:
    """主张 7：`caps` 全为 1 时的快路必须与一般定义逐例同答

    一般路径在这一形状上的代价按份数收：6902 段（真实数据集上组键 = `instruction`，
    每组恰好 1 条）实测 6.8 ms/次，比它服务的分组循环本身还贵。快路只做一次排序取
    阈值（等权时连排序都省掉），所以这里钉的是「选中集 = 按 `exact`（权重乘完比例的
    浮点值）降序、平局按输入顺序的前 `total` 份」。
    """

    @pytest.mark.parametrize("total,weights,expected", [
        (2, [1, 5, 3, 2], [0, 1, 1, 0]),
        (3, [1, 2, 2, 1], [1, 1, 1, 0]),
        (3, [-1, 0, 5, -2], [1, 1, 1, 0]),
        (1, [2, 2, 2], [1, 0, 0]),
        (4, [1] * 4, [1, 1, 1, 1]),
        (0, [3, 1, 2], [0, 0, 0]),
        (1, [0.5, 0.5, 5], [0, 0, 1]),
    ])
    def test_picks_the_heaviest_segments(self, total, weights, expected):
        assert largest_remainder(total, weights, caps=[1] * len(weights)) == expected

    def test_quota_at_or_above_the_pool_takes_every_segment(self):
        """名额盖得住份数 ⇒ 每份 1 条，且不得逐条去挪名额（1e9 也一次答完）"""
        assert largest_remainder(9, [1] * 4, caps=[1] * 4, minimum_each=1) == [1] * 4
        assert largest_remainder(10 ** 9, [1] * 3, caps=[1] * 3) == [1, 1, 1]

    def test_minimum_each_cannot_overissue_under_unit_caps(self):
        """上限 1 之下 `minimum_each` 再大也只能要 1 条，快路不得把它读成超发理由"""
        for each in (0, 1, 2, 5):
            quotas = largest_remainder(3, [7, 1, 1, 1, 1],
                                       caps=[1] * 5, minimum_each=each)
            assert quotas == [1, 1, 1, 0, 0], f"minimum_each={each} 改写了答案"

    @pytest.mark.parametrize("seed", [11, 12, 13, 14])
    def test_fast_path_answers_the_general_definition(self, seed):
        """独立 oracle 对答案：按 `(-exact, 段序)` 排序取前 `total` 份

        与实现共用的那条定义是分开写的（这里逐段算出名义值再排序，实现用阈值 +
        平局补位），所以两条路任一写歪都会在这里露出来。权重掺浮点数，因为快路
        比较的必须是舍入之后的 `exact`。
        """
        rng = random.Random(seed)
        for _ in range(60):
            count = rng.randint(1, 25)
            weights = []
            for _ in range(count):
                w = rng.choice([0, 1, 2, 5, 0.5, -3, 7.25])
                weights.append(w * (1 + rng.random() * 1e-12)
                               if rng.random() < 0.5 else w)
            total = rng.randint(0, count + 2)
            clamped = [w if w > 0 else 0.0 for w in weights]
            weight_sum = sum(clamped)
            if weight_sum <= 0:
                clamped = [1.0] * count
                weight_sum = float(count)
            exacts = [total * s / weight_sum for s in clamped]
            order = sorted(range(count), key=lambda i: (-exacts[i], i))
            wanted = [0] * count
            for i in order[:total]:
                wanted[i] = 1
            got = largest_remainder(total, weights, caps=[1] * count)
            assert got == wanted, f"seed={seed} total={total} weights={weights}"

    def test_fast_path_compares_the_same_float_as_the_general_path(self):
        """排序键必须是 `exact`（乘完比例之后的浮点），不是原始权重

        权重取**相邻 double** 时，`total * 权重 / 权重和` 会把它们舍入成同一个
        `exact`；一般路径把这视为平局（先到先得），按权重排序却会挑后出现的那份。
        实测两个形状：10 段要 3 条、20 段要 6 条，正确答案都不是「权重最大的前 N 段」。
        """
        eps = 2.0 ** -52
        ten = [1.0 + i * eps for i in range(10)]
        assert largest_remainder(3, ten, caps=[1] * 10) == \
            [0, 0, 0, 0, 0, 0, 1, 0, 1, 1]
        assert largest_remainder(3, ten, caps=[1] * 10, minimum_each=1) == \
            [0, 0, 0, 0, 0, 0, 1, 0, 1, 1]
        twenty = [1.0 + i * eps for i in range(20)]
        assert largest_remainder(6, twenty, caps=[1] * 20) == \
            [0] * 13 + [1, 0, 1, 1, 1, 1, 1]

    def test_two_thousand_single_row_groups_answer_in_one_pass(self):
        """2000 段要 500 条：和 == 500、前 500 段各 1 条，且不会退化成逐条挪名额"""
        quotas = largest_remainder(500, [1] * 2000, caps=[1] * 2000, minimum_each=1)
        assert sum(quotas) == 500
        assert quotas[:500] == [1] * 500
        assert 1 not in quotas[500:]

    @staticmethod
    def _spy_unit_path(monkeypatch):
        from augmentor import allocation as allocation_module
        seen = []
        real = getattr(allocation_module, "_unit_cap_quota", None)
        if real is None:  # 缺陷态（还没有快路）也要能被计数，红在「一次都没走」上
            def real(*args, **kwargs):  # noqa: ANN001
                raise AssertionError("unit-cap fast path is missing")
        def spy(*args, **kwargs):
            seen.append(args)
            return real(*args, **kwargs)
        monkeypatch.setattr(allocation_module, "_unit_cap_quota", spy)
        return seen

    def test_unit_cap_request_uses_the_fast_path_even_with_a_guarantee(self, monkeypatch):
        """接线护栏：上限全 1 的请求必须走快路，`minimum_each` 不得把它赶出快路

        分层采样的真实调用正是 `caps=组大小, minimum_each=1`（`dataset_ops.py:242`），
        每组只有 1 条时 `caps` 全 1 ⇒ 落在这条快路上。L40 把保底的准入改成「保底之和
        装不装得下」，而 `minimum_each` 在上限 1 之下被夹成 1、保底之和必然等于
        `count` —— 快路里 `total >= count` 那一支已经把它判完了，所以准入**不该**多读
        这个参数。多读一次（注入模式 12）答案不变、代价从一次比较变成一般路径的
        按份数平方，所以这里钉的是接线而不是答案。
        """
        seen = self._spy_unit_path(monkeypatch)
        sizes = [1] * 2000
        assert largest_remainder(500, sizes, caps=sizes, minimum_each=1) == \
            [1] * 500 + [0] * 1500
        assert len(seen) == 1

    @pytest.mark.parametrize("total,weights,expected", [
        (2, [1] * 4, [1, 1, 0, 0]),
        (3, [7, 7, 7], [1, 1, 1]),
        (1, [0, 0, 0, 0], [1, 0, 0, 0]),
        (2, [-0.0, 0.0, 0.0], [1, 1, 0]),
        (5, [2.5] * 8, [1, 1, 1, 1, 1, 0, 0, 0]),
    ])
    def test_equal_weights_take_the_input_order_prefix(self, total, weights, expected):
        """权重全相等 ⇒ 全体平局 ⇒ 前 `total` 份，掺零权重与非负零也一样

        这条是快路里「等权短路」的依据：`exact` 由同一个算式作用在同一个值上，必然全体
        相同，于是「按 `exact` 降序、平局按段序」退化成的就是输入顺序。真实分层采样正落在
        这一形状上（`weights` 与 `caps` 都是组大小，上限全 1 就意味着每组只有 1 条），
        所以它既要答对，也必须便宜。
        """
        assert largest_remainder(total, weights, caps=[1] * len(weights)) == expected

    def test_nan_weight_is_read_as_zero_like_the_general_path(self):
        """`NaN` 既不大于 0、按 `==` 也不等于自己：短路不触发，且按 0 权重读

        钉的是「快路不得因为 `count` 判不出等权就换一个答案」。两个 `NaN` 必须写成
        **两个不同对象**：`list.count` 带身份快路径，同一个对象会被判成等权而走短路，
        那样这条用例量的就不是「全零权重按份数均分」的回落了（两种写法答案相同，但只有
        后者在回落被摘掉时会红）。
        """
        nan = float("nan")
        assert largest_remainder(2, [nan, 1, 1], caps=[1] * 3) == [0, 1, 1]
        assert largest_remainder(1, [float("nan"), float("nan")], caps=[1] * 2) == [1, 0]
        # 同一个 NaN 对象会被 `count` 判成等权 ⇒ 走另一条支路，答案仍必须同
        assert largest_remainder(1, [nan, nan], caps=[1] * 2) == [1, 0]

    def test_mixed_caps_still_go_through_the_general_path(self):
        """反向护栏：只要有一段上限不是 1，快路就必须让位给一般路径

        钉的是快路的准入条件写歪成 `1 in caps` / `caps[0] == 1` 之类：`[1, 1, 1, 5]`
        这种上限参差的形状若被当成全 1，第 4 段会被压到 1 条，6 条名额只交回 4 条，
        而一般路径会把它让渡到 3 条。
        """
        assert largest_remainder(6, [1, 1, 1, 5], caps=[1, 1, 1, 5]) == [1, 1, 1, 3]
        assert largest_remainder(17, [1, 1, 1, 1], caps=[1, 1, 1, 5]) == [1, 1, 1, 5]


class TestNoCapFastPath:
    """`caps=None` 且无保底时的单轮快路（L39 口径：还 A53 的账，一行答案都不许改）

    依据：一般路径每轮的「取满并冻结」条件是 `floor >= limits[i]`，而 `caps=None` 把上限
    读成 `total`，`exact = remaining * share / weight_sum <= remaining <= total` 永不撞线
    ⇒ 循环必然一轮结束。快路因此只留下「下取整 + 按小数部分降序补 `left` 名」，实测三段
    摊派 3.632 → 1.804 µs/次、2000 份 1166.8 → 593.9 µs/次。
    """

    @pytest.mark.parametrize("total,weights", [
        (7, [1, 1, 1]), (7, [1, 1, 1, 1]), (1, [1, 1, 1]), (0, [3, 1, 2]),
        (13, [0.8, 0.1, 0.1]), (5, [2, 2, 1]), (999, [1, 0, 0]), (3, [0, 0, 0]),
        (4, [-1, 2, 3]), (2, [1e9, 1, 1]), (6, [0.5, 0.25, 0.25]), (11, [3, 3, 5]),
    ])
    def test_matches_the_multi_round_general_path(self, total, weights):
        """oracle = 同一份权重配一份「大得撞不到」的上限，逼实现走多轮通用路径

        `caps` 取 `max(total, 2)` ⇒ 任何一段的配额都不可能撞上它，所以通用路径的让渡与
        冻结逻辑一步都不触发，答案必须与快路逐段相同。上限全为 1 的快路不会误抢这条
        （`caps.count(1) != count`）。
        """
        room = max(total, 2)
        assert largest_remainder(total, weights) == largest_remainder(
            total, weights, caps=[room] * len(weights))

    @pytest.mark.parametrize("seed", [21, 22, 23, 24])
    def test_randomized_matrix_matches_the_general_path(self, seed):
        """随机形状（掺浮点、零、负权重与 1–6 段）逐个与多轮通用路径对答案"""
        rng = random.Random(seed)
        for _ in range(120):
            count = rng.randint(1, 6)
            weights = [rng.choice([0, 1, 2, 5, 0.5, -3, 7.25, 0, 1])
                       for _ in range(count)]
            total = rng.choice([0, 1, 2, 5, 13, rng.randint(0, 400)])
            room = max(total, 2)
            assert largest_remainder(total, weights) == largest_remainder(
                total, weights, caps=[room] * count), f"seed={seed} {total} {weights}"

    @pytest.mark.parametrize("total,weights,expected", [
        (5, [1, 1, 1], [2, 2, 1]),      # 平局 1.666/1.666/1.333 → 先补 0 段再补 1 段
        (3, [1, 1], [2, 1]),            # 小数部分全相等 ⇒ 平局按段序，前一份赢
        (1, [1, 1, 1], [1, 0, 0]),
        (7, [4, 2, 1], [4, 2, 1]),      # 可整除的常规路径不得被顺手改掉
        (7, [2, 1, 1], [3, 2, 2]),      # 小数部分 .5/.75/.75 ⇒ 补的是后两段，不是第一段
        (10, [1, 1, 1], [4, 3, 3]),
        (1, [0, 0, 0], [1, 0, 0]),      # 全零权重回落成「按份数均分」，即第一份
    ])
    def test_answers_are_the_textbook_largest_remainder(self, total, weights, expected):
        """手算答案：快路换的是算法形状，不是数字"""
        assert largest_remainder(total, weights) == expected

    def test_non_positive_total_answers_zeros_like_the_general_path(self):
        """`total` 非正时通用路径的 `while` 一步都不走 ⇒ 交出全 0，而不是负配额

        快路若不显式收住这一形状，`int(负 exact)` 的下取整会交出负数配额（实测
        `total=-5, weights=[1,1]` 落到 [-1,-1]），把一个静默的边角改成静默的错误。
        """
        for total in (0, -1, -50):
            assert largest_remainder(total, [1, 2, 3]) == [0, 0, 0], f"total={total}"

    @staticmethod
    def _spy_fast_path(monkeypatch):
        from augmentor import allocation as allocation_module
        seen = []
        real = getattr(allocation_module, "_no_cap_quota", None)
        if real is None:  # 缺陷态（还没有快路）也要能被计数，红在「一次都没走」上
            def real(*args, **kwargs):  # noqa: ANN001
                raise AssertionError("fast path is missing")
        def spy(*args, **kwargs):
            seen.append(args)
            return real(*args, **kwargs)
        monkeypatch.setattr(allocation_module, "_no_cap_quota", spy)
        return seen

    def test_uncapped_request_uses_the_fast_path(self, monkeypatch):
        """正向接线：`caps=None` 且无保底的请求必须真的走快路（一次）"""
        seen = self._spy_fast_path(monkeypatch)
        assert largest_remainder(7, [1, 1, 1]) == [3, 2, 2]
        assert len(seen) == 1

    @pytest.mark.parametrize("kwargs", [
        {"caps": [1, 5, 5]},          # 有上限 ⇒ 让渡逻辑在场
        {"caps": [5, 5, 5]},
        {"minimum_each": 1},          # 有保底无上限 ⇒ 快路里没有「每份至少几条」的位置
        {"caps": [5, 5, 5], "minimum_each": 1},
        {"minimum_each": 2},          # 保底装不下（3 × 2 > 5）⇒ 走一般路径去判准入
        {"caps": [5, 5, 5], "minimum_each": 2},
        {"caps": [2, 2, 2], "minimum_each": 2},   # 保底之和 6 > 5 ⇒ 整条退回按权重摊派
        {"caps": [1, 2, 2], "minimum_each": 2},   # 保底之和恰等于 5 ⇒ 边界上准入
    ])
    def test_capped_or_minimum_each_requests_do_not_enter_the_fast_path(
            self, monkeypatch, kwargs):
        """反向接线：准入条件必须同时看 `caps` 与 `minimum_each`

        只判 `caps is None` 会把 `minimum_each` 非 0 的请求送进单轮闭式；只判 `minimum_each`
        则会把带上限的让渡请求当成无上限处理（少交条数）。两种写歪都在这里红。

        `minimum_each >= 2` 的几行是 L40 才加进来的：那时一般路径的准入判据还是
        `total >= count`，`total=5, weights=[1,1,1], minimum_each=2` 会交出 `[2,2,2]`
        （和 6，超发一条，即 A55），`sum(quotas) == 5` 这一条断言当场就红。判据改成
        「保底之和装得下」之后同一请求交 `[2,2,1]`，本用例才只测它「不进快路」。
        """
        seen = self._spy_fast_path(monkeypatch)
        quotas = largest_remainder(5, [1, 1, 1], **kwargs)
        assert len(seen) == 0, f"{kwargs} 走了快路"
        assert sum(quotas) == 5

    def test_split_and_random_paths_share_the_one_round_answer(self):
        """两个划分器的三段大小必须仍与快路同答（L36 的跨路径契约不因快路失效）"""
        for n in (7, 13, 91, 6902):
            expected = largest_remainder(n, RATIOS)
            items = [{"instruction": f"第{i}问", "output": f"答{i}"} for i in range(n)]
            result = DataSplitter(seed=7).split(items)
            assert (len(result.train), len(result.val), len(result.test)) == tuple(expected)


class TestMinimumEachAdmission:
    """主张 9：`minimum_each` 的准入读「保底之和装不装得下」，不是「名额盖不盖得住份数」

    A49/A55（L37 立、L39 复记）：判据 `total >= count` 在两处同时错。
    ①名额盖得住份数却盖不住「保底 × 份数」时**超发**（`sum(配额) > total`）——
    契约合法域穷举 399,594 例（1..5 段 × 权重 1/2/3 × `caps` 取 None 或 0/1/2/5 ×
    `total` 0..12 × `minimum_each` 0..5）里 **24,889 例**；
    ②份数里有吃不下保底的段（上限为 0）时**误拒**真正装得下的请求 —— 同域 **261 例**
    「保底其实装得下，却没给任何一份保底」。
    修口 = 先按各自上限夹一遍保底、再求和判准入。

    在库可达面不动：唯一的调用方 `dataset_ops._stratified_sample` 传 `minimum_each=1`
    且 `caps` 是组大小（必然 >= 1），那时「保底之和」恰为 `count × 1`，新判据与旧判据
    同一条式子。上面那 399,594 例里 `minimum_each <= 1` 的新旧分歧共 253 例，
    **全部含上限为 0 的段**（不含零上限的 0 例）。
    """

    @pytest.mark.parametrize("total, weights, caps, minimum_each, expected", [
        # ①超发族：保底 × 份数 > 名额 ⇒ 整条保底不生效，退回纯按权重摊派
        (5, [1, 1, 1], None, 2, [2, 2, 1]),
        (5, [1, 1, 1], [5, 5, 5], 2, [2, 2, 1]),
        (2, [5, 5], [10, 10], 2, [1, 1]),
        (7, [1] * 4, None, 2, [2, 2, 2, 1]),
        (11, [5, 5, 5], None, 4, [4, 4, 3]),
        (3, [1, 1], None, 5, [2, 1]),
        # ②装得下族：答案与 L39 逐字相同，本轮不许动
        (8, [1, 1, 1], None, 2, [3, 3, 2]),
        (12, [5, 5, 5], None, 4, [4, 4, 4]),
        (6, [100, 1, 1], [2, 2, 2], 2, [2, 2, 2]),
        (7, [100, 1, 1], None, 1, [5, 1, 1]),
        (3, [1] * 8, None, 1, [1, 1, 1, 0, 0, 0, 0, 0]),
        # ③边界：保底之和恰好等于名额 ⇒ 「装得下」，必须逐份给满（`<=` 不是 `<`）
        (4, [1, 2], None, 2, [2, 2]),
        (9, [3, 3, 3], [3, 3, 3], 3, [3, 3, 3]),
    ])
    def test_answers(self, total, weights, caps, minimum_each, expected):
        got = largest_remainder(total, weights, None if caps is None else list(caps),
                                minimum_each)
        assert got == expected

    def test_a_guarantee_that_does_not_fit_is_dropped_whole_not_partially(self):
        """装不下时是「整条退回按权重摊派」，不是「能塞几份塞几份」

        `total=9, weights=[5,1,1], minimum_each=4`：保底之和 12 > 9 ⇒ 不生效 ⇒
        交回 `[7, 1, 1]`（纯最大余数法）。若改成尽力塞，答案会是 `[4, 4, 1]` ——
        那等于让保底反过来吞掉权重，`minimum_each` 就成了第二份权重而不是下界。
        """
        assert largest_remainder(9, [5, 1, 1], None, 4) == [7, 1, 1]

    def test_segments_with_no_room_do_not_count_against_the_guarantee(self):
        """份数里含「上限 0」的段时，旧判据会误拒一个真正装得下的请求

        5 段里三段的上限是 0 ⇒ 吃得下保底的只有 2 段 ⇒ 保底之和是 2 而不是 5，
        `total=4` 装得下。旧判据读份数（`4 >= 5` 为假）而拒掉，交回 `[0,0,0,1,3]`；
        新判据先夹一遍上限再求和，交 `[0,0,0,2,2]`。`minimum_each` 取 1 或 2 同答：
        上限 2 的那段在两种保底下都只吃得下 2 条。
        """
        caps = [0, 0, 0, 2, 5]
        weights = [1, 1, 1, 1, 2]
        for each in (1, 2):
            assert largest_remainder(4, weights, list(caps), each) == [0, 0, 0, 2, 2], each

    def test_the_guarantee_is_clipped_by_each_cap_before_it_is_summed(self):
        """保底要按各自上限夹过再求和：`caps=[2,2,2] + minimum_each=5` 的保底之和是 6"""
        assert largest_remainder(6, [1, 1, 1], [2, 2, 2], 5) == [2, 2, 2]
        # 上限之和不足时短交是诚实答案（`caps` 之和 3 < 名额 5），但不是超发
        assert largest_remainder(5, [1, 1, 1], [1, 1, 1], 2) == [1, 1, 1]
        assert largest_remainder(4, [1, 1], [0, 0], 3) == [0, 0]

    def test_no_legal_request_ever_gets_more_items_than_it_asked_for(self):
        """不变量护栏：`sum(配额) <= total`，且逐段不越上限、装得下时保底必到

        这条是 A49/A55 的直接护栏 —— 上面那些定点答案换任何一种判据都能凑出来，
        而「全域不超发」只有准入判据写对才成立。域取契约合法形状（段数 1..5、
        `caps` 为 None 或含 0 的上限、`total` 0..12、`minimum_each` 0..4），
        展开 1,625 例。
        """
        for count in (1, 2, 3, 4, 5):
            for total in range(0, 13):
                for each in range(0, 5):
                    cap_sets = [None, [1] * count, [total] * count,
                                [0] * count, [i % 3 for i in range(count)]]
                    for caps in cap_sets:
                        limits = [total] * count if caps is None else list(caps)
                        quotas = largest_remainder(total, [1] * count, caps, each)
                        assert sum(quotas) <= total, (count, total, caps, each, quotas)
                        assert all(q <= lim for q, lim in zip(quotas, limits)), \
                            (count, total, caps, each, quotas)
                        floor_total = sum(min(each, lim) for lim in limits)
                        if each and floor_total <= total:
                            assert all(q >= each for q, lim in zip(quotas, limits)
                                       if lim >= each), (count, total, caps, each, quotas)
                        else:
                            assert sum(quotas) == min(total, sum(limits))


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
