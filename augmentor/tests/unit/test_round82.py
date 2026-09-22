"""第82轮: enhanced_searcher增强搜索"""
import pytest
from augmentor.search_enhanced import EnhancedSearcher


class TestEnhancedSearcher:
    def test_create(self):
        s = EnhancedSearcher()
        assert s is not None

    def test_has_core_methods(self):
        s = EnhancedSearcher()
        assert hasattr(s, 'search')
        assert hasattr(s, 'load')
        assert hasattr(s, 'get_statistics')
