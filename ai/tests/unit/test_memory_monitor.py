"""内存监控测试 - 验证内存使用趋势检测和警告功能"""

import pytest
from augmentor.memory_monitor import MemoryMonitor, get_memory_summary, monitor_memory_usage


@pytest.fixture
def monitor():
    return MemoryMonitor(interval_mb=10, alert_threshold_percent=80.0)


class TestMemoryMonitorInit:
    """初始化测试"""
    
    def test_default_interval(self, monitor):
        assert monitor.interval_mb == 10
    
    def test_alert_threshold_set(self, monitor):
        assert monitor.alert_threshold_percent == 80.0


class TestMemoryMonitorSnapshots:
    """快照和趋势测试"""
    
    def test_take_snapshot_returns_dict(self, monitor):
        snapshot = monitor.take_snapshot()
        assert "usage_mb" in snapshot
        assert "threshold_percent" in snapshot
        assert "alert" in snapshot
    
    def test_peak_usage_updates(self, monitor):
        monitor.take_snapshot()
        peak = monitor.get_peak_usage_mb()
        assert peak >= 0.0
    
    def test_trend_stable_for_single_snapshot(self, monitor):
        monitor.take_snapshot()
        assert monitor.get_trend() == "stable"
    
    def test_trend_increasing_for_rising_usage(self, monitor):
        # 模拟上升趋势：手动插入快照
        monitor._snapshots = [
            {"usage_mb": 100},
            {"usage_mb": 150},
            {"usage_mb": 200},
        ]
        trend = monitor.get_trend()
        assert trend == "increasing"


class TestMemoryMonitorDecorator:
    """装饰器功能测试"""
    
    def test_decorator_preserves_result(self):
        @monitor_memory_usage
        def sample_task():
            return {"status": "ok"}
        
        result = sample_task()
        assert result == {"status": "ok"}
        assert hasattr(sample_task, "memory_monitor")
    
    def test_memory_summary_available(self):
        summary = get_memory_summary()
        assert "current_usage_mb" in summary
        assert "monitor_available" in summary


class TestMemoryMonitorReset:
    """重置功能测试"""
    
    def test_reset_clears_snapshots(self, monitor):
        monitor.take_snapshot()
        monitor.reset()
        assert len(monitor._snapshots) == 0
        assert monitor.get_peak_usage_mb() == 0.0
