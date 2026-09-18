"""第83轮: auto_config自动配置"""
import pytest
from augmentor.auto_config import AutoConfig


class TestAutoConfig:
    def test_create(self):
        c = AutoConfig()
        assert c is not None

    def test_has_core_methods(self):
        c = AutoConfig()
        assert hasattr(c, 'recommend') or hasattr(c, 'auto_recommend')
