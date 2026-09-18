"""第69轮: auto_test自动测试"""
import pytest
from augmentor.auto_test import DatasetTestRunner


class TestAutoTest:
    def test_create_runner(self):
        r = DatasetTestRunner()
        assert r is not None

    def test_has_core_methods(self):
        r = DatasetTestRunner()
        assert hasattr(r, 'run_tests')
        assert hasattr(r, 'create_test_suite')
        assert hasattr(r, 'add_custom_test')
        assert hasattr(r, 'get_test_report')
