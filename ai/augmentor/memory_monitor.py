"""内存使用监控模块 - 性能优化"""

import logging
import sys
from typing import Optional, Dict, Callable

logger = logging.getLogger(__name__)

try:
    import psutil
    HAS_PSUTIL = True
except ImportError:
    HAS_PSUTIL = False


class MemoryMonitor:
    """内存使用监控器"""
    
    def __init__(self, interval_mb: int = 10, alert_threshold_percent: float = 80.0):
        self.interval_mb = interval_mb
        self.alert_threshold_percent = alert_threshold_percent
        self._snapshots = []
        self._current_usage_mb = 0.0
    
    def get_memory_usage_mb(self) -> float:
        """获取当前进程内存使用（MB）"""
        if HAS_PSUTIL:
            process = psutil.Process()
            return process.memory_info().rss / (1024 * 1024)
        # 退化方案：无法精确获取时返回估计值
        return self._current_usage_mb
    
    def take_snapshot(self) -> Dict:
        """记录内存快照"""
        usage = self.get_memory_usage_mb()
        snapshot = {
            "usage_mb": usage,
            "threshold_percent": self.alert_threshold_percent,
            "alert": usage > (self.interval_mb * self.alert_threshold_percent / 100) if self.interval_mb > 0 else False
        }
        self._snapshots.append(snapshot)
        self._current_usage_mb = usage
        if snapshot["alert"]:
            logger.warning(f"内存使用警告: {usage:.1f} MB (阈值: {snapshot['threshold_percent']}%)")
        return snapshot
    
    def get_peak_usage_mb(self) -> float:
        """获取峰值内存使用"""
        if not self._snapshots:
            return 0.0
        return max(s["usage_mb"] for s in self._snapshots)
    
    def get_trend(self) -> str:
        """获取内存使用趋势"""
        if len(self._snapshots) < 2:
            return "stable"
        recent = [s["usage_mb"] for s in self._snapshots[-5:]]
        if all(recent[i] <= recent[i + 1] for i in range(len(recent) - 1)):
            return "increasing"
        if all(recent[i] >= recent[i + 1] for i in range(len(recent) - 1)):
            return "decreasing"
        return "fluctuating"
    
    def reset(self):
        """重置监控状态"""
        self._snapshots.clear()
        self._current_usage_mb = 0.0


def monitor_memory_usage(func: Callable) -> Callable:
    """内存监控装饰器（简化版）"""
    monitor = MemoryMonitor()
    
    def wrapper(*args, **kwargs):
        monitor.take_snapshot()
        result = func(*args, **kwargs)
        monitor.take_snapshot()
        logger.info(f"内存监控完成，峰值: {monitor.get_peak_usage_mb():.2f} MB，趋势: {monitor.get_trend()}")
        return result
    
    wrapper.memory_monitor = monitor
    return wrapper


def get_memory_summary() -> Dict:
    """获取简要内存状态"""
    monitor = MemoryMonitor()
    snapshot = monitor.take_snapshot()
    return {
        "current_usage_mb": snapshot["usage_mb"],
        "monitor_available": HAS_PSUTIL,
        "trend": "initial"
    }
