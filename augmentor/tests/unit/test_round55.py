# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""第55轮: outlier检测增强"""
import pytest
from augmentor.outlier import OutlierDetector, OutlierReport


class TestOutlierDetector:
    def test_create_detector(self):
        d = OutlierDetector()
        assert d is not None

    def test_detect_returns_report(self):
        d = OutlierDetector()
        items = [{"instruction": f"test {i}", "output": f"out {i}"} for i in range(20)]
        result = d.detect(items)
        assert isinstance(result, OutlierReport)
        assert result.total_items == 20

    def test_empty_detect(self):
        d = OutlierDetector()
        result = d.detect([])
        assert isinstance(result, OutlierReport)
        assert result.total_items == 0

    def test_report_fields(self):
        d = OutlierDetector()
        items = [{"instruction": "hello", "output": "world"}]
        result = d.detect(items)
        assert hasattr(result, 'method')
        assert hasattr(result, 'threshold')
        assert hasattr(result, 'outliers')
