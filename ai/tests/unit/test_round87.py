"""第87轮: splitter分割增强"""
import pytest
from augmentor.data_splitter import DataSplitter


class TestSplitter:
    def test_create(self):
        s = DataSplitter()
        assert s is not None

    def test_split_basic(self):
        s = DataSplitter()
        items = [{"instruction": f"test_{i}"} for i in range(100)]
        train, test = s.split_dataset(items, split_ratio=0.8)
        assert len(train) == 80
        assert len(test) == 20
