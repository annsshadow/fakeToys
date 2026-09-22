# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""第93轮: aggregator聚合器"""
import pytest
from augmentor.aggregator import DataAggregator


class TestAggregator:
    def test_create(self):
        a = DataAggregator()
        assert a is not None

    def test_has_core_methods(self):
        a = DataAggregator()
        assert hasattr(a, 'aggregate')
