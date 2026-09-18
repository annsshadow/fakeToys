"""第57轮: comparison对比增强"""
import pytest
from augmentor.comparison import DatasetComparator


class TestDatasetComparator:
    def test_create(self):
        c = DatasetComparator()
        assert c is not None

    def test_compare_same(self):
        c = DatasetComparator()
        items = [{"instruction": "test", "output": "out"}]
        result = c.compare(items, items)
        assert result is not None

    def test_compare_empty(self):
        c = DatasetComparator()
        result = c.compare([], [])
        assert result is not None
