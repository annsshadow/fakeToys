# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""第96轮: impact影响评估"""
import pytest
from augmentor.impact import ImpactEvaluator


class TestImpact:
    def test_create(self):
        i = ImpactEvaluator()
        assert i is not None

    def test_has_core_methods(self):
        i = ImpactEvaluator()
        assert hasattr(i, 'evaluate') or hasattr(i, 'assess_impact')
