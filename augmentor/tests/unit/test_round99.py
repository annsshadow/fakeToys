"""第99轮: compare_enhanced增强对比"""
import pytest
from augmentor.compare_enhanced import EnhancedComparator


class TestCompareEnhanced:
    def test_create(self):
        c = EnhancedComparator()
        assert c is not None

    def test_has_core_methods(self):
        c = EnhancedComparator()
        assert hasattr(c, 'compare')
