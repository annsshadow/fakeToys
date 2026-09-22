"""第86轮: cleaner数据清洗"""
import pytest
from augmentor.cleaner import DatasetCleaner


class TestCleaner:
    def test_create(self):
        c = DatasetCleaner()
        assert c is not None

    def test_has_core_methods(self):
        c = DatasetCleaner()
        assert hasattr(c, 'clean')
