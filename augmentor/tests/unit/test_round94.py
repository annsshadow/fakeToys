# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""第94轮: enhanced_visualizer增强可视化"""
import pytest
from augmentor.visualize_enhanced import EnhancedVisualizer


class TestEnhancedVisualizer:
    def test_create(self):
        v = EnhancedVisualizer()
        assert v is not None

    def test_has_core_methods(self):
        v = EnhancedVisualizer()
        assert hasattr(v, 'generate_json_report')
        assert hasattr(v, 'generate_text_report')
        assert hasattr(v, 'save_json_report')
        assert hasattr(v, 'save_text_report')
