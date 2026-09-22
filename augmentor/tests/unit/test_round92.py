# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""第92轮: config配置管理"""
import pytest
from augmentor.config import AppConfig


class TestConfig:
    def test_import(self):
        from augmentor.config import AppConfig
        assert AppConfig is not None

    def test_has_core_attributes(self):
        from augmentor.config import AppConfig
        assert hasattr(AppConfig, 'default_model')
