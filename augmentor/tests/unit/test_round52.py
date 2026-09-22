# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""第52轮: dataset_ops批量操作测试"""
import pytest
from augmentor.dataset_ops import batch_merge, shuffle_dataset


class TestBatchMerge:
    def test_merge_two_datasets(self):
        d1 = [{"instruction": "a"}, {"instruction": "b"}]
        d2 = [{"instruction": "c"}]
        result = batch_merge([d1, d2])
        assert len(result) == 3

    def test_merge_deduplication(self):
        d1 = [{"instruction": "same"}]
        d2 = [{"instruction": "same"}]
        result = batch_merge([d1, d2], deduplicate=True)
        assert len(result) == 1

    def test_merge_no_dedup(self):
        d1 = [{"instruction": "same"}]
        d2 = [{"instruction": "same"}]
        result = batch_merge([d1, d2], deduplicate=False)
        assert len(result) == 2

    def test_merge_empty(self):
        result = batch_merge([])
        assert result == []


class TestShuffleDataset:
    def test_shuffle_changes_order(self):
        items = [{"id": i} for i in range(100)]
        shuffled = shuffle_dataset(items, seed=42)
        assert len(shuffled) == 100
        assert [i["id"] for i in shuffled] != [i["id"] for i in items]

    def test_shuffle_deterministic(self):
        items = [{"id": i} for i in range(50)]
        s1 = shuffle_dataset(items, seed=123)
        s2 = shuffle_dataset(items, seed=123)
        assert s1 == s2

    def test_shuffle_preserves_all_items(self):
        items = [{"id": i} for i in range(30)]
        shuffled = shuffle_dataset(items, seed=7)
        assert sorted(i["id"] for i in shuffled) == list(range(30))
