"""性能基准测试 - 测量增强流程各阶段耗时和内存峰值"""

import pytest
import time
from augmentor.performance_benchmark import PerformanceBenchmark


@pytest.fixture
def benchmark():
    return PerformanceBenchmark(label="test_benchmark")


class TestPerformanceBenchmark:
    """性能基准测试"""
    
    def test_stage_timing_records_duration(self, benchmark):
        """阶段计时应记录正确持续时间"""
        benchmark.start_stage("test_stage")
        time.sleep(0.01)  # 短暂延迟
        benchmark.end_stage("test_stage")
        
        assert len(benchmark.stages) == 1
        assert benchmark.stages[0]["duration_ms"] is not None
        assert benchmark.stages[0]["duration_ms"] >= 10  # 至少 10ms
    
    def test_total_duration_calculated(self, benchmark):
        """总时长应正确计算"""
        benchmark.start_stage("stage_1")
        benchmark.end_stage("stage_1")
        benchmark.start_stage("stage_2")
        benchmark.end_stage("stage_2")
        
        report = benchmark.generate_benchmark_report()
        assert report["total_duration_ms"] >= 0
        assert report["stage_count"] == 2
    
    def test_memory_peak_updated(self, benchmark):
        """内存峰值应可测量"""
        benchmark.measure_memory_peak()
        assert benchmark.memory_peak_mb >= 0.0
    
    def test_timed_phase_executes_and_records(self, benchmark):
        """计时阶段应执行操作并记录结果"""
        def sample_op():
            return {"result": "ok"}
        
        result = benchmark.run_timed_phase("timed_op", sample_op)
        assert result == {"result": "ok"}
        assert any(s["stage"] == "timed_op" for s in benchmark.stages)
    
    def test_benchmark_report_contains_stages(self, benchmark):
        """基准报告应包含所有阶段信息"""
        benchmark.start_stage("load")
        benchmark.end_stage("load")
        benchmark.start_stage("process")
        benchmark.end_stage("process")
        
        report = benchmark.generate_benchmark_report()
        assert "stages" in report
        assert len(report["stages"]) == 2
        assert "timestamp" in report
        assert "label" in report
