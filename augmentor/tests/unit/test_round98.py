# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""第98轮: sampler采样器"""
import pytest
from augmentor.sampler import ActiveSampler


class TestSampler:
    def test_create(self):
        s = ActiveSampler()
        assert s is not None

    def test_has_core_methods(self):
        s = ActiveSampler()
        assert hasattr(s, 'analyze_coverage')
        assert hasattr(s, 'identify_underrepresented')
        assert hasattr(s, 'recommend_seeds')
        assert hasattr(s, 'generate_report')
