# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""第74轮: model_manager模型管理"""
import pytest
from augmentor.model_manager import model_manager, ModelManager


class TestModelManager:
    def test_singleton(self):
        m1 = ModelManager()
        m2 = ModelManager()
        assert m1 is m2

    def test_has_core_methods(self):
        m = model_manager
        assert hasattr(m, 'get_sentence_model')
        assert hasattr(m, 'clear')
