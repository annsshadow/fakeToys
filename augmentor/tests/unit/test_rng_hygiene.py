# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""SDK 随机操作的进程级 RNG 卫生门禁

`random.seed()` / `random.shuffle()` 动的是**解释器全局**的 RNG 状态。库代码这么写
会让「先种好种子、再调 augmentor、再继续用随机数」的调用方拿到被污染的序列——
而 augmentor 自己的输出照样逐次复现，所以这个缺陷对使用者是隐形的，只体现在
别人的可复现性上。

3.0 合并 CLI 时已在 `cli/commands/export.py` 改用局部 `random.Random`，SDK 侧的
六处（`dataset_ops` / `data_splitter` / `indexer` / `export_enhanced`）当时漏了。
本文件是防止它重新长回来的门禁。
"""

import random

import pytest

from augmentor.data_splitter import DataSplitter
from augmentor.dataset_ops import (
    DatasetOperations,
    MergeConfig,
    SampleConfig,
    SplitConfig,
    shuffle_dataset,
)

ITEMS = [{"instruction": f"问{i}", "output": f"答{i}"} for i in range(12)]


def assert_no_global_rng_side_effect(action):
    """调用 `action()` 前后，进程级 RNG 的后续序列必须完全一致

    先在固定种子上取一个值把状态推进，再取「参照下一值」；然后同样的种子上执行
    动作并取下一值。动作若 `random.seed(...)`（重置状态）或 `random.random()`
    （消耗状态），两个值就会不同。
    """
    random.seed(20260923)
    random.random()
    reference = random.random()

    random.seed(20260923)
    random.random()
    action()
    after = random.random()

    assert after == reference, "SDK 调用改写了进程级 RNG 状态"


class TestRngHygiene:
    @pytest.mark.parametrize("config", [
        SampleConfig(size=5, seed=7),
        SampleConfig(size=5, seed=7, method="stratified"),
        SampleConfig(size=5, seed=7, method="systematic"),
    ], ids=["random", "stratified", "systematic"])
    def test_sample(self, config):
        ops = DatasetOperations()
        assert_no_global_rng_side_effect(lambda: ops.sample(ITEMS, config))

    @pytest.mark.parametrize("config", [
        SplitConfig(ratios=(0.6, 0.2, 0.2), seed=7),
        SplitConfig(ratios=(0.6, 0.2, 0.2), seed=7, shuffle=False),
    ], ids=["shuffled", "ordered"])
    def test_split(self, config):
        ops = DatasetOperations()
        assert_no_global_rng_side_effect(lambda: ops.split(ITEMS, config))

    def test_shuffle_method_and_helper(self):
        ops = DatasetOperations()
        assert_no_global_rng_side_effect(lambda: ops.shuffle(ITEMS, seed=7))
        assert_no_global_rng_side_effect(lambda: shuffle_dataset(ITEMS, seed=7))

    def test_merge_without_order_preservation(self):
        """`preserve_order=False` 的打乱以前直接消耗调用方的随机序列"""
        ops = DatasetOperations()
        assert_no_global_rng_side_effect(
            lambda: ops.merge([ITEMS, ITEMS], MergeConfig(preserve_order=False))
        )

    @pytest.mark.parametrize("stratify_field", [None, "output"])
    def test_data_splitter(self, stratify_field):
        splitter = DataSplitter(seed=7, stratify_field=stratify_field)
        assert_no_global_rng_side_effect(lambda: splitter.split(ITEMS))

    def test_determinism_survives_the_change(self):
        """换成局部 Random 后，同种子仍必须逐次可复现

        这条是防「修污染时把可复现性一起改没了」：外部先随机消费多少次都不影响
        augmentor 的输出。
        """
        ops = DatasetOperations()
        random.seed(99)
        random.random()
        first = ops.sample(ITEMS, SampleConfig(size=4, seed=7))

        random.seed(1)
        for _ in range(5):
            random.random()
        second = ops.sample(ITEMS, SampleConfig(size=4, seed=7))

        assert first == second
