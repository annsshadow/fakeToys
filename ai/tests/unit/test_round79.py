"""第79轮: memory_monitor内存监控"""
import pytest
from augmentor.memory_monitor import MemoryMonitor


class TestMemoryMonitor:
    def test_create(self):
        m = MemoryMonitor()
        assert m is not None

    def test_has_core_methods(self):
        m = MemoryMonitor()
        assert hasattr(m, 'get_memory_usage_mb')
        assert hasattr(m, 'get_peak_usage_mb')
        assert hasattr(m, 'get_trend')
        assert hasattr(m, 'take_snapshot')
        assert hasattr(m, 'reset')

    def test_get_memory_usage_returns_number(self):
        m = MemoryMonitor()
        usage = m.get_memory_usage_mb()
        assert isinstance(usage, (int, float))
        assert usage >= 0
