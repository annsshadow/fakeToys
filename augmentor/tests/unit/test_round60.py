# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""第60轮: context上下文增强"""
import pytest
from augmentor.context import ContextAugmentor


class TestContextAugmentor:
    def test_create_with_backend(self):
        class FakeBackend:
            pass
        c = ContextAugmentor(model_backend=FakeBackend())
        assert c is not None

    def test_has_enhance_method(self):
        class FakeBackend:
            pass
        c = ContextAugmentor(model_backend=FakeBackend())
        assert hasattr(c, 'enhance_with_context_awareness')
        assert hasattr(c, 'add_history_to_single_turn')
        assert hasattr(c, 'convert_single_to_multi_turn')
