"""第87轮: splitter分割增强"""
import pytest
from augmentor.data_splitter import DataSplitter, SplitResult


class TestSplitter:
    def test_create(self):
        s = DataSplitter()
        assert s is not None

    def test_split_basic(self):
        s = DataSplitter()
        items = [{"instruction": f"test_{i}"} for i in range(100)]
        result = s.split(items)
        assert isinstance(result, SplitResult)
        assert result.total_items == 100
