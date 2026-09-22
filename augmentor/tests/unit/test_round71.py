# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""第71轮: statistics统计计算"""
import pytest
from augmentor.statistics import DatasetStatisticsCalculator


class TestStatistics:
    def test_create(self):
        s = DatasetStatisticsCalculator()
        assert s is not None

    def test_has_core_methods(self):
        s = DatasetStatisticsCalculator()
        assert hasattr(s, 'calculate')
        assert hasattr(s, 'load')
