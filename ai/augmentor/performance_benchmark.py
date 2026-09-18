"""性能基准测试模块 - 测量增强流程各阶段耗时和内存峰值"""

import logging
import time
from typing import List, Dict, Optional, Callable
from datetime import datetime

logger = logging.getLogger(__name__)

try:
    from .memory_monitor import MemoryMonitor
    HAS_MEMORY_MONITOR = True
except ImportError:
    HAS_MEMORY_MONITOR = False


class PerformanceBenchmark:
    """性能基准测试器"""
    
    def __init__(self, label: str = "benchmark"):
        self.label = label
        self.stages: List[Dict] = []
        self.start_time: Optional[float] = None
        self.memory_peak_mb: float = 0.0
    
    def start_stage(self, stage_name: str):
        """开始计时阶段"""
        self.stages.append({
            "stage": stage_name,
            "start_time": time.time(),
            "end_time": None,
            "duration_ms": None
        })
        logger.info(f"基准阶段开始: {stage_name}")
    
    def end_stage(self, stage_name: Optional[str] = None):
        """结束计时阶段"""
        if not self.stages:
            return
        if stage_name is None or self.stages[-1].get("stage") == stage_name:
            stage = self.stages[-1]
            stage["end_time"] = time.time()
            stage["duration_ms"] = (stage["end_time"] - stage["start_time"]) * 1000
            logger.info(f"基准阶段完成: {stage['stage']} ({stage['duration_ms']:.2f} ms)")
        else:
            # 查找指定阶段
            for stage in self.stages:
                if stage.get("stage") == stage_name and stage.get("end_time") is None:
                    stage["end_time"] = time.time()
                    stage["duration_ms"] = (stage["end_time"] - stage["start_time"]) * 1000
                    logger.info(f"基准阶段完成: {stage['stage']} ({stage['duration_ms']:.2f} ms)")
                    break
    
    def measure_memory_peak(self) -> float:
        """测量当前内存峰值（简化实现）"""
        if HAS_MEMORY_MONITOR:
            monitor = MemoryMonitor()
            snapshot = monitor.take_snapshot()
            self.memory_peak_mb = max(self.memory_peak_mb, snapshot.get("usage_mb", 0.0))
        return self.memory_peak_mb
    
    def generate_benchmark_report(self) -> Dict:
        """生成基准报告"""
        total_duration = sum(s.get("duration_ms", 0) or 0 for s in self.stages)
        stage_details = [
            {
                "stage": s["stage"],
                "duration_ms": s.get("duration_ms", 0),
                "start_time": datetime.fromtimestamp(s["start_time"]).isoformat() if s.get("start_time") else None
            }
            for s in self.stages
        ]
        return {
            "label": self.label,
            "total_duration_ms": total_duration,
            "stage_count": len(self.stages),
            "memory_peak_mb": self.memory_peak_mb,
            "stages": stage_details,
            "timestamp": datetime.now().isoformat()
        }
    
    def run_timed_phase(self, phase_name: str, operation: Callable, *args, **kwargs) -> Any:
        """运行计时阶段并执行操作"""
        self.start_stage(phase_name)
        result = operation(*args, **kwargs)
        self.end_stage(phase_name)
        return result
