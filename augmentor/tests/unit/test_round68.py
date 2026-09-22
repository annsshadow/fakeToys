# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""第68轮: dependency依赖管理"""
import pytest
from augmentor.dependency import DependencyManager


class TestDependencyManager:
    def test_create(self):
        d = DependencyManager()
        assert d is not None

    def test_has_core_methods(self):
        d = DependencyManager()
        assert hasattr(d, 'get_dependency_graph')
