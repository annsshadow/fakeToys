# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""数据质量趋势追踪测试 - 验证趋势分析、方向计算和报告生成"""

import pytest
import tempfile
from pathlib import Path
from augmentor.quality_trend import QualityTrendTracker


@pytest.fixture
def tracker():
    with tempfile.TemporaryDirectory() as tmp:
        return QualityTrendTracker(storage_path=str(Path(tmp) / "trend.json"))


@pytest.fixture
def tracker_with_data(tracker):
    tracker.record_quality_metrics("dataset_1", {"completeness": 0.8, "diversity": 0.6}, sample_count=100)
    tracker.record_quality_metrics("dataset_1", {"completeness": 0.85, "diversity": 0.65}, sample_count=200)
    tracker.record_quality_metrics("dataset_1", {"completeness": 0.9, "diversity": 0.7}, sample_count=300)
    return tracker


class TestQualityTrendTrackerInit:
    """初始化测试"""
    
    def test_tracker_init_with_path(self, tracker):
        assert tracker.storage_path is not None
    
    def test_tracker_history_empty(self, tracker):
        assert tracker._trend_history == []


class TestRecordQualityMetrics:
    """记录质量指标测试"""
    
    def test_record_creates_entry(self, tracker):
        result = tracker.record_quality_metrics("test_ds", {"quality": 0.75}, sample_count=50)
        assert result["dataset_name"] == "test_ds"
        assert "timestamp" in result
        assert result["metrics"]["quality"] == 0.75
    
    def test_history_grows_after_records(self, tracker):
        tracker.record_quality_metrics("ds", {"a": 1.0})
        tracker.record_quality_metrics("ds", {"a": 1.1})
        assert len(tracker._trend_history) == 2


class TestTrendAnalysis:
    """趋势分析测试"""
    
    def test_trend_direction_improving(self, tracker_with_data):
        direction = tracker_with_data.calculate_trend_direction("completeness", "dataset_1")
        assert direction == "improving"
    
    def test_trend_direction_insufficient_for_single(self, tracker):
        tracker.record_quality_metrics("ds", {"quality": 0.5})
        direction = tracker.calculate_trend_direction("quality", "ds")
        assert direction == "insufficient_data"
    
    def test_trend_stable_for_flat_values(self, tracker):
        for i in range(5):
            tracker.record_quality_metrics("ds", {"quality": 0.5})
        direction = tracker.calculate_trend_direction("quality", "ds")
        assert direction == "stable"


class TestTrendReport:
    """趋势报告测试"""
    
    def test_generate_trend_report_empty(self, tracker):
        report = tracker.generate_trend_report()
        assert report["status"] == "no_data"
    
    def test_generate_trend_report_with_data(self, tracker_with_data):
        report = tracker_with_data.generate_trend_report("dataset_1")
        assert report["status"] == "ok"
        assert "metrics_trends" in report
        assert "dataset_filter" in report


class TestTrendComparison:
    """趋势比较测试"""
    
    def test_compare_trends(self, tracker):
        tracker.record_quality_metrics("ds_a", {"quality": 0.9})
        tracker.record_quality_metrics("ds_b", {"quality": 0.7})
        comparison = tracker.compare_trends("ds_a", "ds_b", "quality")
        assert "dataset_a" in comparison
        assert "dataset_b" in comparison
        assert comparison["comparison"] == "dataset_a_higher"
