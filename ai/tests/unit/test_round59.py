"""第59轮: expander域扩展"""
import pytest
from augmentor.expander import DomainExpander


class TestDomainExpander:
    def test_create_with_backend(self):
        class FakeBackend:
            pass
        e = DomainExpander(model_backend=FakeBackend())
        assert e is not None

    def test_has_expand_method(self):
        class FakeBackend:
            pass
        e = DomainExpander(model_backend=FakeBackend())
        assert hasattr(e, 'expand')
