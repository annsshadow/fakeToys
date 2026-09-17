"""DataSplitter 单元测试

分层划分必须保证各子集分布与总体一致，比例校验与可复现性都要覆盖。
"""

import pytest

from augmentor.data_splitter import (
    DataSplitter,
    SplitResult,
    split_dataset,
)


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
