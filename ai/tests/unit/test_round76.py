"""第76轮: quality_monitor质量监控"""
import pytest
from augmentor.quality_monitor import QualityMonitor, create_monitor


class TestQualityMonitor:
    def test_create_monitor(self):
        m = create_monitor()
        assert m is not None

    def test_has_core_methods(self):
        m = create_monitor()
        assert hasattr(m, 'check_quality')
        assert hasattr(m, 'add_threshold')
        assert hasattr(m, 'add_alert_callback')
        assert hasattr(m, 'get_history')
        assert hasattr(m, 'get_summary')
        assert hasattr(m, 'get_trend')
