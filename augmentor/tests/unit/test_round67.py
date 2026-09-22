# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""第67轮: versioning版本管理"""
import pytest
from augmentor.versioning import VersionManager


class TestVersionManager:
    def test_create(self):
        v = VersionManager()
        assert v is not None

    def test_has_core_methods(self):
        v = VersionManager()
        assert hasattr(v, 'create_version')
        assert hasattr(v, 'load_version')
        assert hasattr(v, 'list_versions')
