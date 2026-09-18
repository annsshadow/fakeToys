"""第58轮: quality_report质量报告"""
import pytest
from augmentor.quality_report import QualityReporter


class TestQualityReporter:
    def test_create(self):
        r = QualityReporter()
        assert r is not None

    def test_generate_report_empty(self):
        r = QualityReporter()
        result = r.generate_report([])
        assert result is not None
