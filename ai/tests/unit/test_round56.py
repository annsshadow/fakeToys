"""第56轮: dedup去重增强"""
import pytest
from augmentor.dedup import Deduplicator, DedupResult


class TestDeduplicator:
    def test_create(self):
        d = Deduplicator()
        assert d is not None

    def test_deduplicate_basic(self):
        d = Deduplicator()
        items = [
            {"instruction": "hello", "output": "world"},
            {"instruction": "hello", "output": "world"},
            {"instruction": "different", "output": "content"},
        ]
        result = d.deduplicate(items)
        assert isinstance(result, DedupResult)
        assert result.original_count == 3

    def test_deduplicate_empty(self):
        d = Deduplicator()
        result = d.deduplicate([])
        assert isinstance(result, DedupResult)
        assert result.original_count == 0

    def test_deduplicate_no_duplicates(self):
        d = Deduplicator()
        items = [{"instruction": f"unique_{i}"} for i in range(10)]
        result = d.deduplicate(items)
        assert isinstance(result, DedupResult)
        assert result.original_count == 10
        assert result.removed_count == 0
