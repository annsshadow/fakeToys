# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""DataSplitter 单元测试

分层划分必须保证各子集分布与总体一致，比例校验与可复现性都要覆盖。
"""

import random
from fractions import Fraction

import pytest
from augmentor import data_splitter as splitter_module
from augmentor.allocation import largest_remainder
from augmentor.data_splitter import (
    DataSplitter,
    SplitResult,
    split_dataset,
)


def _sizes(result):
    return (len(result.train), len(result.val), len(result.test))


def _corpus(shape, field="intent"):
    """`[(组数, 每组条数)]` → 语料；同组共享 `field` 值，`pos` 记下原位置。

    组按声明顺序出现（先声明的组在前），所以「组序 = 语料序」这类偏置能被读到。
    """
    items = []
    pos = 0
    gid = 0
    for group_count, size in shape:
        for _ in range(group_count):
            gid += 1
            for _k in range(size):
                items.append({"instruction": f"问题{pos}", field: f"G{gid}", "pos": pos})
                pos += 1
    return items


def _group_sizes(items, field="intent"):
    seen = {}
    for item in items:
        seen[item[field]] = seen.get(item[field], 0) + 1
    return list(seen.values())


def _max_cell_deviation(items, ratios, field="intent", seeds=range(12)):
    """`stratify_distribution` 里最偏离 `组大小 × 比例` 的那个单元格（跨 12 个种子取最大）

    跨种子取最大是必须的：随机组序本身有方差，单个种子测不出「大组先」这条纪律。
    """
    worst = 0.0
    for seed in seeds:
        result = DataSplitter(*ratios, seed=seed, stratify_field=field).split(items)
        for dist in result.stratify_distribution.values():
            group_size = sum(dist.values())
            for name, ratio in zip(("train", "val", "test"), ratios):
                worst = max(worst, abs(dist[name] - group_size * ratio))
    return worst


def _fraction_targets(total, ratios):
    """独立 oracle：用分数算术重述「三段各要几条」，不碰被测实现的 float 路径

    只用于二进制可精确表示的比例（0.5/0.25/0.75/0.125），因为 `0.1` 之类的 float
    本身就落在 `0.1000000000000000055511151231257827`，oracle 要重述的是**意图**，
    不是复刻 float 舍入。
    """
    exact = [total * Fraction(str(r)) for r in ratios]
    floors = [int(e) for e in exact]
    left = total - sum(floors)
    order = sorted(range(len(exact)), key=lambda k: (floors[k] - exact[k], k))
    for k in order[:left]:
        floors[k] += 1
    return tuple(floors)


@pytest.fixture
def items():
    """构造带意图特征的数据"""
    data = []
    for i in range(20):
        intent = "how" if i % 4 in (0, 1) else ("what" if i % 4 == 2 else "why")
        data.append({"instruction": f"问题{i}", "intent": intent})
    return data


class TestInit:
    def test_rejects_ratio_sum_not_one(self):
        with pytest.raises(ValueError, match="比例之和"):
            DataSplitter(train_ratio=0.5, val_ratio=0.5, test_ratio=0.5)

    def test_rejects_negative_ratio(self):
        with pytest.raises(ValueError, match="负数"):
            DataSplitter(train_ratio=1.5, val_ratio=-0.5, test_ratio=0.0)

    def test_accepts_zero_split(self):
        splitter = DataSplitter(train_ratio=1.0, val_ratio=0.0, test_ratio=0.0)
        assert splitter.train_ratio == 1.0


class TestRandomSplit:
    def test_sums_to_total(self, items):
        result = split_dataset(items, 0.8, 0.1, 0.1, seed=42)
        assert result.total_items == len(items)

    def test_proportions_approximate(self, items):
        result = split_dataset(items, 0.8, 0.1, 0.1, seed=42)
        assert len(result.train) == int(20 * 0.8)
        assert len(result.val) == int(20 * 0.1)
        assert len(result.test) == 20 - len(result.train) - len(result.val)

    def test_reproducible_with_seed(self, items):
        r1 = split_dataset(items, 0.8, 0.1, 0.1, seed=7)
        r2 = split_dataset(items, 0.8, 0.1, 0.1, seed=7)
        assert [x["instruction"] for x in r1.train] == [x["instruction"] for x in r2.train]

    def test_no_overlap_between_splits(self, items):
        result = split_dataset(items, 0.8, 0.1, 0.1, seed=1)
        seen = set()
        for subset in (result.train, result.val, result.test):
            for item in subset:
                assert item["instruction"] not in seen
                seen.add(item["instruction"])

    def test_empty_dataset(self):
        result = split_dataset([], 0.8, 0.1, 0.1)
        assert result.total_items == 0


class TestStratifiedSplit:
    def test_distribution_recorded(self, items):
        result = split_dataset(items, 0.8, 0.1, 0.1, seed=42, stratify_field="intent")
        assert result.stratify_field == "intent"
        assert "how" in result.stratify_distribution
        for key, dist in result.stratify_distribution.items():
            assert set(dist.keys()) == {"train", "val", "test"}
            assert dist["train"] + dist["val"] + dist["test"] >= 1

    def test_each_split_preserves_groups(self, items):
        """每个子集中各特征值组都应尽量出现"""
        result = split_dataset(items, 0.8, 0.1, 0.1, seed=42, stratify_field="intent")
        train_intents = set(i["intent"] for i in result.train)
        # how 类有 10 条，train 8 条，至少应包含 how
        assert "how" in train_intents

    def test_missing_field_grouped_as_empty(self):
        data = [
            {"instruction": "a"},
            {"instruction": "b"},
            {"instruction": "c"},
            {"instruction": "d"},
        ]
        result = split_dataset(data, 0.8, 0.1, 0.1, seed=0, stratify_field="intent")
        assert "empty" in result.stratify_distribution

    def test_stratified_reproducible(self, items):
        r1 = split_dataset(items, 0.8, 0.1, 0.1, seed=3, stratify_field="intent")
        r2 = split_dataset(items, 0.8, 0.1, 0.1, seed=3, stratify_field="intent")
        assert [x["instruction"] for x in r1.train] == [x["instruction"] for x in r2.train]

    def test_single_item_stratified(self):
        result = split_dataset([{"instruction": "x", "intent": "how"}], 0.8, 0.1, 0.1, seed=0, stratify_field="intent")
        assert result.total_items == 1


class TestStratifiedGlobalReconciliation:
    """分层划分的三段条数必须在**全体**上对账，而不是逐组下取整

    HEAD（`5c9294ed3` 之前）逐组 `int(n * 比例)`：真实 6902 条按 `instruction`
    分层交出 train 371 / val **0** / test 6531（test 名义 10% 实拿 94.6%），
    按 `output` 分层交出 (4925, 142, 1835)（应 5522 / 690 / 690）。
    """

    @pytest.mark.parametrize("shape", [
        [(100, 1)],
        [(6, 2)],
        [(617, 1), (36, 2), (1, 3), (2, 4), (1, 5)],
        [(50, 3), (20, 7)],
        [(1, 1)],
    ])
    @pytest.mark.parametrize("ratios", [
        (0.8, 0.1, 0.1),
        (0.7, 0.2, 0.1),
        (0.5, 0.25, 0.25),
    ])
    def test_segment_sizes_reconcile_globally(self, shape, ratios):
        """三段条数必须 == 同一份数据不分层时的三段大小（余数不再整份堆给 test）"""
        items = _corpus(shape)
        total = len(items)
        expected = tuple(largest_remainder(total, ratios))
        result = DataSplitter(*ratios, seed=11, stratify_field="intent").split(items)
        assert _sizes(result) == expected, f"形状 {shape} 期望 {expected} 实得 {_sizes(result)}"

    def test_val_is_not_empty_when_every_group_is_smaller_than_the_ratio(self):
        """100 个「每组 1 条」的类别也要交出 10 条验证集：名义 10% 不等于逐组 0 条

        HEAD 在这里交出 (0, 0, 100) —— 组内 `int(1 * 0.8)` 与 `int(1 * 0.1)` 都是 0，
        余数整份堆给 test。
        """
        items = _corpus([(100, 1)])
        result = DataSplitter(0.8, 0.1, 0.1, seed=3, stratify_field="intent").split(items)
        assert _sizes(result) == (80, 10, 10)
        assert len(result.val) == 10
        assert len(result.test) == 10

    def test_residual_does_not_all_pile_into_test(self):
        """6 组 × 2 条：HEAD 交出 (6, 0, 6)，把分配器在组内套用则交出 (12, 0, 0)

        两个错向都要钉住 —— 前者把余数整份堆给 test（50%），后者让 val 与 test 双双为空。
        正确答案是全体目标 (10, 1, 1)。
        """
        items = _corpus([(6, 2)])
        result = DataSplitter(0.8, 0.1, 0.1, seed=5, stratify_field="intent").split(items)
        assert _sizes(result) == (10, 1, 1)

    def test_distribution_rows_sum_to_group_size_and_columns_to_targets(self):
        """回显的 `stratify_distribution` 必须与真实三段一致：行和 == 组大小，列和 == 目标"""
        items = _corpus([(40, 1), (12, 2), (3, 5), (1, 9)])
        targets = tuple(largest_remainder(len(items), (0.8, 0.1, 0.1)))
        result = DataSplitter(0.8, 0.1, 0.1, seed=8, stratify_field="intent").split(items)
        dist = result.stratify_distribution
        assert sorted(dist) == sorted({i["intent"] for i in items})
        for key, counts in dist.items():
            group_size = sum(1 for i in items if i["intent"] == key)
            assert counts["train"] + counts["val"] + counts["test"] == group_size
        cols = (sum(d["train"] for d in dist.values()),
                sum(d["val"] for d in dist.values()),
                sum(d["test"] for d in dist.values()))
        assert cols == targets
        assert _sizes(result) == targets

    def test_single_group_matches_the_random_split_sizes(self):
        """只有一个组时，分层与不分层必须同答（分层不该改变三段大小，只该改变成员）"""
        items = _corpus([(1, 10)])
        stratified = DataSplitter(0.8, 0.1, 0.1, seed=2, stratify_field="intent").split(items)
        plain = DataSplitter(0.8, 0.1, 0.1, seed=2).split(items)
        assert _sizes(stratified) == _sizes(plain) == (8, 1, 1)

    def test_membership_moves_with_the_seed(self):
        """组序必须随机化：同尺寸组「谁进验证集」不能对每个种子都给出同一批类别

        只按组出现顺序摊目标时，100 个单条组的验证集恒为组序 71..98 那一批
        （实测 8 个种子给出 1 个互异集合）。
        """
        items = _corpus([(100, 1)])
        sets = []
        for seed in range(8):
            result = DataSplitter(0.8, 0.1, 0.1, seed=seed, stratify_field="intent").split(items)
            sets.append({i["intent"] for i in result.val})
        assert len(sets[0]) == 10
        assert len({frozenset(s) for s in sets}) == 8

    def test_val_does_not_come_only_from_the_corpus_tail(self):
        """验证集不能整体落在语料尾段：20 个种子里每个都至少一半来自前 80% 位置

        按出现顺序摊目标时实测 10 条里有 7 条来自后 20% 区段（前 80% 只有 3 条）。
        """
        items = _corpus([(100, 1)])
        for seed in range(20):
            result = DataSplitter(0.8, 0.1, 0.1, seed=seed, stratify_field="intent").split(items)
            front = sum(1 for i in result.val if i["pos"] < 80)
            assert front >= 5, f"seed={seed} 验证集前 80% 位置只有 {front} 条"

    def test_large_groups_absorb_the_proportional_share_not_the_residual(self):
        """组大小降序处理：零头只由小组吸收，任何单元格的偏差不超过 1 条

        同一份语料（116 组 / 140 条，单条组在前）改成全局随机序时实测偏差上限是 2.2
        （12 个种子）/ 3.0（200 个种子），升序同样恒为 3.0，而现口径在 12/40/200 个
        种子上都是 0.9 —— 随机序会让 2 条的组赶上 train 目标耗尽，整组落进 val/test。
        """
        items = _corpus([(100, 1), (12, 2), (1, 3), (2, 4), (1, 5)])
        dev = _max_cell_deviation(items, (0.8, 0.1, 0.1))
        assert dev <= 1.0, f"最大单元偏差 {dev:.3f}，超过「零头只由单条组吸收」的 1.0 上限"

    @pytest.mark.parametrize("ratios,expected", [
        ((1.0, 0.0, 0.0), (11, 0, 0)),
        ((0.0, 0.5, 0.5), (0, 6, 5)),
    ])
    def test_zero_ratio_segment_stays_empty(self, ratios, expected):
        """比例为 0 的那一段必须恒为空，不能被跨组对账「借」出一条"""
        items = _corpus([(5, 1), (2, 3)])
        result = DataSplitter(*ratios, seed=4, stratify_field="intent").split(items)
        assert _sizes(result) == expected

    def test_fraction_oracle_agrees_on_segment_sizes(self):
        """与独立 oracle（分数算术）对照：混尺寸语料的三段大小逐档相同"""
        for ratios in ((0.5, 0.25, 0.25), (0.75, 0.125, 0.125), (0.5, 0.5, 0.0)):
            for shape in ([(7, 1)], [(13, 3), (4, 1)], [(9, 2), (5, 7), (1, 1)]):
                items = _corpus(shape)
                expected = _fraction_targets(len(items), ratios)
                result = DataSplitter(*ratios, seed=6, stratify_field="intent").split(items)
                assert _sizes(result) == expected, f"{ratios} {shape}: 期望 {expected} 实得 {_sizes(result)}"

    def test_no_item_is_lost_or_duplicated(self):
        """三段切片边界必须既不丢条目也不重复条目（逐组三段是全长切分）"""
        items = _corpus([(30, 1), (9, 2), (2, 6), (1, 11)])
        result = DataSplitter(0.8, 0.1, 0.1, seed=9, stratify_field="intent").split(items)
        positions = [i["pos"] for subset in (result.train, result.val, result.test) for i in subset]
        assert sorted(positions) == list(range(len(items)))

    def test_empty_field_group_takes_its_share_of_the_targets(self):
        """缺字段被并成 `empty` 组后同样参与对账，不是把整组塞进某一段

        判别力上限（实测，不是推测）：这一形状在「组内独立 LR」和「不更新剩余目标」两种
        劣化下**恰好凑出**同一份三段大小（`LR(9) = (7, 1, 1)` 加 `LR(3) = (3, 0, 0)` 等于
        全体目标 `(10, 1, 1)`），所以它只对整体回退红；对那两种劣化的判别由列对账例与 15 档
        `test_segment_sizes_reconcile_globally` 矩阵承担。
        """
        items = [{"instruction": f"问题{i}", "intent": ("" if i < 9 else "x"), "pos": i}
                 for i in range(12)]
        result = DataSplitter(0.8, 0.1, 0.1, seed=5, stratify_field="intent").split(items)
        dist = result.stratify_distribution
        assert dist["empty"]["train"] + dist["empty"]["val"] + dist["empty"]["test"] == 9
        assert _sizes(result) == tuple(largest_remainder(12, (0.8, 0.1, 0.1)))

    def test_distribution_key_order_follows_first_appearance(self):
        """回显的类别顺序 = 首次出现顺序，不是「组大小降序 + 同尺寸随机」的处理顺序

        处理顺序只决定谁吸收零头；`stratify_distribution` 会被 `to_dict()` 原样带出去，
        键序若跟着随机处理序走，同一份输入两次产物就不再字节相同（A34 那一类跨进程漂移）。
        """
        items = _corpus([(3, 1), (2, 9)])
        result = DataSplitter(0.8, 0.1, 0.1, seed=17, stratify_field="intent").split(items)
        expected = ["G1", "G2", "G3", "G4", "G5"]
        assert list(result.stratify_distribution) == expected
        assert list(result.to_dict()["stratify_distribution"]) == expected

    def test_allocator_is_called_once_per_multi_row_group_and_once_globally(self, monkeypatch):
        """结构护栏（L39 改口径）：调用数 == 1 + **多元素**组数，单条组一次都不调用

        L38 的口径是「每组一次」，L39 把尾部单条组换成一次比较 + 一次递减的摊派，所以期望值
        随口径一起改；这条在 L38 提交态必红（它给 `(60,1)` 档多算了 60 次）。
        钉的还是同一件事：每组至多摊派一次，不退化成逐段补名额。
        """
        calls = []
        real = splitter_module.largest_remainder

        def spy(*args, **kwargs):
            calls.append(args)
            return real(*args, **kwargs)

        monkeypatch.setattr(splitter_module, "largest_remainder", spy)
        items = _corpus([(60, 1), (15, 2), (1, 5)])
        sizes = _group_sizes(items)
        multi = sum(1 for s in sizes if s > 1)
        DataSplitter(0.8, 0.1, 0.1, seed=1, stratify_field="intent").split(items)
        assert len(calls) == multi + 1, f"多元素组 {multi} 个，期望 {multi + 1} 次实得 {len(calls)}"
        assert [c[0] for c in calls[1:]] == sorted(sizes, reverse=True)[:multi]

        calls.clear()
        all_single = _corpus([(40, 1)])
        DataSplitter(0.8, 0.1, 0.1, seed=1, stratify_field="intent").split(all_single)
        assert len(calls) == 1, "全是单条组时不该有任何逐组摊派"

        calls.clear()
        DataSplitter(0.8, 0.1, 0.1, seed=1).split(items)
        assert len(calls) == 1


class TestStratifiedSingletonTail:
    """L39 口径：尾部单条组的摊派换成「还欠最多的那一段 +1」，答案必须一字不改

    A53 记的是 L38 的代价：真实语料按 `instruction` 分层时 6,531 组里有 6,168 组只有 1 条，
    逐组调用分配器占掉端到端 15.19 / 21.77 ms（70%）。单条组只能交出 1 条，
    `largest_remainder(1, targets)` 的答案恰是 `argmax(targets)`（平局取序号最小）上的一热，
    于是这一大段不必调用分配器。
    """

    @staticmethod
    def _reference(items, ratios, seed, field="intent"):
        """L38 提交态的算法复刻：逐组调用分配器 + 逐组无条件 shuffle，只作同答 oracle

        刻意与被测实现用同一份 `largest_remainder`，所以这里对的答案是「摊派形状」而不是
        数字；三段数字由 `TestStratifiedGlobalReconciliation` 与分配器自己的用例钉住。
        """
        groups: dict = {}
        for item in items:
            key = str(item.get(field, "")) or "empty"
            groups.setdefault(key, []).append(item)
        rng = random.Random(seed)
        targets = list(largest_remainder(len(items), ratios))
        order = list(groups)
        rng.shuffle(order)
        order.sort(key=lambda k: -len(groups[k]))
        cells = {}
        for key in order:
            got = largest_remainder(len(groups[key]), targets)
            targets = [t - c for t, c in zip(targets, got)]
            cells[key] = got
        train, val, test, dist = [], [], [], {}
        for key, group in groups.items():
            rng.shuffle(group)
            train_n, val_n, test_n = cells[key]
            train.extend(group[:train_n])
            val.extend(group[train_n:train_n + val_n])
            test.extend(group[train_n + val_n:])
            dist[key] = {"train": train_n, "val": val_n, "test": test_n}
        rng.shuffle(train)
        rng.shuffle(val)
        rng.shuffle(test)
        return train, val, test, dist

    @pytest.mark.parametrize("shape", [
        [(100, 1)],
        [(60, 1), (15, 2), (1, 5)],
        [(6, 2)],
        [(1, 1)],
        [(30, 1), (9, 2), (2, 6), (1, 11)],
        [(200, 1), (3, 7)],
        [(2, 3), (200, 1)],
    ])
    @pytest.mark.parametrize("seed", [1, 7, 42])
    def test_singleton_tail_batch_matches_group_by_group_allocation(self, shape, seed):
        """逐元素同答：成员与**输出顺序**都必须与逐组摊派一致

        顺序也要钉，是因为实现末尾用同一个 rng 再洗三段；若尾部少消耗或多消耗一次随机
        （例如给单元素组也调 shuffle），成员不变而顺序会变，只比大小的用例照样绿。
        """
        items = _corpus(shape)
        expected = self._reference(items, (0.8, 0.1, 0.1), seed)
        result = DataSplitter(0.8, 0.1, 0.1, seed=seed, stratify_field="intent").split(items)
        for name, got, want in (
                ("train", result.train, expected[0]),
                ("val", result.val, expected[1]),
                ("test", result.test, expected[2])):
            assert [i["pos"] for i in got] == [i["pos"] for i in want], \
                f"形状 {shape} seed={seed} 的 {name} 不同答"
        assert result.stratify_distribution == expected[3]

    @pytest.mark.parametrize("targets,expected", [
        ((3, 1, 1), (1, 0, 0)),
        ((1, 3, 1), (0, 1, 0)),
        ((1, 1, 3), (0, 0, 1)),
        ((2, 2, 1), (1, 0, 0)),        # 平局取序号最小
        ((0, 2, 2), (0, 1, 0)),
        ((0, 0, 5), (0, 0, 1)),
        ((1, 0, 1), (1, 0, 0)),        # 两端同高 ⇒ 仍是第一段
        ((0, 0, 0), (1, 0, 0)),        # 回落「全零权重按份数均分」⇒ 第一段
    ])
    def test_one_row_group_goes_to_the_most_underserved_segment(self, targets, expected):
        """尾部快路的引理：`largest_remainder(1, targets)` == `argmax`（平局按序号）上的一热

        实现里这条引理是内联的三行比较，所以必须在这里独立钉住 —— 下一轮把 `>` 写成
        `>=` 就会静默换掉「谁进验证集」。
        """
        assert tuple(largest_remainder(1, targets)) == expected

    def test_singleton_tie_goes_to_the_lowest_index_segment(self, monkeypatch):
        """尾部快路的**内联**平局口径：目标并列时交给序号最小的那段（train 优先）

        上一条钉的是分配器，实现里没有调用它 —— 平局规则被内联成两行 `>` 比较，所以这里
        直接用它驱动划分。为了让「谁进哪段」可预言，把 shuffle 挂成恒等（组序退化成出现
        序，三段大小不变）：`>=` 会让第二条单条组抢先填 val，分布逐键不同。
        """
        monkeypatch.setattr(random.Random, "shuffle", lambda self, x: None)
        items = _corpus([(3, 1)])
        result = DataSplitter(0.5, 0.5, 0.0, seed=1, stratify_field="intent").split(items)
        assert result.stratify_distribution == {
            "G1": {"train": 1, "val": 0, "test": 0},
            "G2": {"train": 1, "val": 0, "test": 0},
            "G3": {"train": 0, "val": 1, "test": 0},
        }

    def test_multi_row_groups_still_go_through_the_allocator(self, monkeypatch):
        """反向护栏：快路只接管单条组，多元素组的摊派一次都不能少

        把边界写歪成「第一段之后全走快路」时，尺寸 2 的组会被当成 1 条摊给最大目标，
        列和就不再等于全体目标（`test_segment_sizes_reconcile_globally` 也会红）。
        """
        calls = []
        real = splitter_module.largest_remainder

        def spy(*args, **kwargs):
            calls.append(args[0])
            return real(*args, **kwargs)

        monkeypatch.setattr(splitter_module, "largest_remainder", spy)
        items = _corpus([(5, 2), (4, 3), (40, 1)])
        result = DataSplitter(0.8, 0.1, 0.1, seed=3, stratify_field="intent").split(items)
        assert calls[1:] == [3, 3, 3, 3, 2, 2, 2, 2, 2]
        cols = (sum(d["train"] for d in result.stratify_distribution.values()),
                sum(d["val"] for d in result.stratify_distribution.values()),
                sum(d["test"] for d in result.stratify_distribution.values()))
        assert cols == tuple(largest_remainder(len(items), (0.8, 0.1, 0.1)))

    def test_singleton_groups_are_the_contiguous_tail_of_the_processing_order(self, monkeypatch):
        """快路的前提：组按尺寸降序，所以尺寸 1 的组必然排在尾部

        这条把「前提」钉成断言而不是注释：组序纪律若被改掉（比如改成按出现顺序），
        批处理就会把中间的多元素组当成尾部摊派，段大小立刻对不平。
        """
        seen_sizes = []
        real = splitter_module.largest_remainder

        def spy(*args, **kwargs):
            seen_sizes.append(args[0])
            return real(*args, **kwargs)

        monkeypatch.setattr(splitter_module, "largest_remainder", spy)
        items = _corpus([(1, 9), (25, 1), (4, 4), (6, 2)])
        DataSplitter(0.8, 0.1, 0.1, seed=5, stratify_field="intent").split(items)
        group_sizes = seen_sizes[1:]
        assert group_sizes == sorted(group_sizes, reverse=True)
        assert 1 not in group_sizes

    def test_single_item_groups_are_not_shuffled(self, monkeypatch):
        """代价护栏：单元素组不再进 shuffle（行为同答，但少 6,168 次函数调用）

        `shuffle([x])` 的循环区间 `range(1, 1)` 为空 ⇒ 不消耗随机（实测 `Random(1)` 洗一条
        单元素列表后的 `random()` 与不洗时字节相同：0.13436424411240122），所以省掉它不改
        答案；但它是 L38 在真实语料上的固定开销之一，因此这里把「不该再被调用」钉成断言。
        多元素组仍然必须洗 —— 否则「谁进验证集」退化成语料顺序，`test_membership_moves_with_the_seed`
        与 `test_val_does_not_come_only_from_the_corpus_tail` 会红。
        """
        real = random.Random.shuffle
        sizes = []

        def spy(self, x):
            sizes.append(len(x))
            return real(self, x)

        monkeypatch.setattr(random.Random, "shuffle", spy)
        items = _corpus([(100, 1), (5, 2)])
        DataSplitter(0.8, 0.1, 0.1, seed=3, stratify_field="intent").split(items)
        assert 1 not in sizes, f"单元素组仍被洗牌：{sizes}"
        # 1 次组序 + 5 次多元素组 + 3 次末段洗牌
        assert sorted(sizes) == sorted([105, 2, 2, 2, 2, 2, 88, 11, 11]), sizes


class TestSplitResult:
    def test_total_items_property(self, items):
        result = split_dataset(items, 0.8, 0.1, 0.1, seed=0)
        assert result.total_items == len(items)

    def test_to_dict_fields(self, items):
        result = split_dataset(items, 0.8, 0.1, 0.1, seed=0, stratify_field="intent")
        d = result.to_dict()
        assert "train_count" in d
        assert "val_count" in d
        assert "test_count" in d
        assert "stratify_distribution" in d
