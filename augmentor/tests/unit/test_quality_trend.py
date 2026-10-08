# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""数据质量趋势追踪测试 - 验证趋势分析、方向计算和报告生成"""

import pytest
import json
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


    def test_compare_trends_tie_when_equal(self, tracker):
        """两数据集某指标最新值相等 ⇒ comparison = tie（改前误报 dataset_b_higher，L142，B212）"""
        tracker.record_quality_metrics("ds_a", {"quality": 0.5})
        tracker.record_quality_metrics("ds_b", {"quality": 0.5})
        comparison = tracker.compare_trends("ds_a", "ds_b", "quality")
        assert comparison["comparison"] == "tie"

    def test_compare_trends_tie_when_metric_absent_on_both(self, tracker):
        """某指标两数据集都从未记录 ⇒ comparison = tie（改前把缺指标静默当 0 比，误报 b_higher）"""
        tracker.record_quality_metrics("ds_a", {"other": 0.9})
        comparison = tracker.compare_trends("ds_a", "ds_b", "never")
        assert comparison["dataset_a_latest"] is None
        assert comparison["dataset_b_latest"] is None
        assert comparison["comparison"] == "tie"

    def test_compare_trends_a_higher_strict(self, tracker):
        """严格大于才判 a_higher（钉死只有「>」触发，平局/小于都不是）"""
        tracker.record_quality_metrics("ds_a", {"quality": 0.9})
        tracker.record_quality_metrics("ds_b", {"quality": 0.7})
        assert tracker.compare_trends("ds_a", "ds_b", "quality")["comparison"] == "dataset_a_higher"
        assert tracker.compare_trends("ds_b", "ds_a", "quality")["comparison"] == "dataset_b_higher"

class TestCorruptHistoryQuarantine:
    """畸形文件防护：不静默置空历史、新条目不得覆盖原始数据（L146，B216）"""

    def test_corrupt_file_quarantined_and_new_file_clean(self, tmp_path):
        """损坏 JSON ⇒ 备份到 .corrupt-*，原内容可从备份找回；新文件只含新条目"""
        p = tmp_path / "trend.json"
        p.write_text("{this is not json", encoding="utf-8")
        t = QualityTrendTracker(storage_path=str(p))
        assert t._trend_history == []
        t.record_quality_metrics("ds", {"quality": 0.5})
        backups = list(tmp_path.glob("trend.json.corrupt-*"))
        assert len(backups) == 1
        assert "this is not json" in backups[0].read_text(encoding="utf-8")
        assert len(json.loads(p.read_text(encoding="utf-8"))["trends"]) == 1

    def test_non_list_trends_quarantined(self, tmp_path):
        """'trends' 是非列表（字符串）⇒ 同样按畸形文件备份重置（改前：_trend_history 被置成字符串，append 直接崩）"""
        p = tmp_path / "trend.json"
        p.write_text(json.dumps({"trends": "not-a-list"}), encoding="utf-8")
        t = QualityTrendTracker(storage_path=str(p))
        assert t._trend_history == []
        assert t._corrupt_unquarantined is False
        t.record_quality_metrics("ds", {"quality": 0.5})
        assert len(list(tmp_path.glob("trend.json.corrupt-*"))) == 1
        assert len(t._trend_history) == 1

    def test_unquarantineable_corrupt_file_kept_intact(self, tmp_path, monkeypatch):
        """两条备份路径全断（rename 被挡、复制源被拒）⇒ 标记禁写、保存跳过，原文件逐字保留
        （改前：open('w') 截断覆盖致数据永久丢失；monkeypatch 平台无关，不受
        Windows/Linux 只读文件语义差异影响）"""
        p = tmp_path / "trend.json"
        p.write_text("{corrupt-locked", encoding="utf-8")
        original = p.read_text(encoding="utf-8")

        def boom(self, target=None):
            raise OSError("file locked")
        monkeypatch.setattr(Path, "rename", boom)
        monkeypatch.setattr(Path, "read_bytes",
                            lambda self: (_ for _ in ()).throw(PermissionError("复制源被拒")))
        t = QualityTrendTracker(storage_path=str(p))
        assert t._corrupt_unquarantined is True
        t.record_quality_metrics("ds", {"quality": 0.5})
        assert p.read_text(encoding="utf-8") == original
        assert len(t._trend_history) == 1  # 内存历史仍在（功能不丢），仅不落盘
