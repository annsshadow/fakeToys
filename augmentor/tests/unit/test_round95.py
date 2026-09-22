# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""第95轮: visualizer可视化"""
import pytest
from augmentor.visualizer import DataVisualizer


class TestVisualizer:
    def test_create(self):
        v = DataVisualizer()
        assert v is not None

    def test_has_core_methods(self):
        v = DataVisualizer()
        assert hasattr(v, 'plot_length_distribution') or hasattr(v, 'generate_statistics')
