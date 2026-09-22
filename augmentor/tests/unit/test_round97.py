# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""第97轮: enhanced_exporter增强导出"""
import pytest
from augmentor.export_enhanced import EnhancedExporter


class TestEnhancedExporter:
    def test_create(self):
        e = EnhancedExporter()
        assert e is not None

    def test_has_core_methods(self):
        e = EnhancedExporter()
        assert hasattr(e, 'export')
