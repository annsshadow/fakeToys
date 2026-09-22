# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""第78轮: health_score健康评分"""
import pytest
from augmentor.health_score import DatasetHealthScore


class TestHealthScore:
    def test_create(self):
        h = DatasetHealthScore()
        assert h is not None

    def test_has_core_methods(self):
        h = DatasetHealthScore()
        assert hasattr(h, 'score')
        assert hasattr(h, 'calculate_completeness')
        assert hasattr(h, 'calculate_diversity')
        assert hasattr(h, 'calculate_coverage')
        assert hasattr(h, 'calculate_quality_balance')
