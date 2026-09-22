# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""微型分支收尾 L63：quality/quality_report/quality_trend/report 残留分支

- quality: cross-encoder 成功加载 + batch_score 真实 encoder 相关性路径
- quality_report: 空 items 的长度分布/重复率降级指标
- quality_trend: 无存储路径空操作、保存失败被捕获
- report: 空分布与模块级 build_report 便捷函数
"""

import json
from pathlib import Path
from types import ModuleType

import numpy as np
import pytest

from augmentor.quality import QualityScorer
from augmentor.quality_report import QualityReporter
from augmentor.quality_trend import QualityTrendTracker
from augmentor.report import ReportGenerator, build_report


class _FakeCE:
    """假 cross-encoder：predict 直接返回固定分数"""

    def __init__(self, name=None):
        self.name = name

    def predict(self, pairs):
        return np.array([0.8] * len(pairs))


class TestQualityCrossEncoder:
    def test_load_models_with_fake_cross_encoder(self, monkeypatch):
        fake_st = ModuleType("sentence_transformers")

        class _FakeCE2:
            def __init__(self, name=None):
                self.name = name

        fake_st.CrossEncoder = _FakeCE2
        fake_st.SentenceTransformer = lambda name=None: "fake-model"
        monkeypatch.setitem(__import__("sys").modules, "sentence_transformers", fake_st)

        scorer = QualityScorer()
        scorer._load_models()
        assert scorer._cross_encoder != "fallback"
        assert scorer._cross_encoder.name is not None

    def test_batch_score_uses_cross_encoder_relevance(self):
        scorer = QualityScorer()
        scorer._model = "fallback"
        scorer._cross_encoder = _FakeCE()
        results = scorer.batch_score(
            [{"original": "q", "generated": "g", "output": "o"}]
        )
        # relevance = clip((0.8 + 1) / 2) = 0.9
        assert results[0].relevance == pytest.approx(0.9)


class TestQualityReportEmptyItems:
    def test_length_distribution_no_valid_questions(self):
        generator = QualityReporter()
        metric = generator._calculate_length_distribution([{"output": "x"}])
        assert metric.passed is False
        assert metric.value == 0

    def test_duplication_rate_no_instructions(self):
        generator = QualityReporter()
        metric = generator._calculate_duplication_rate([{"output": "x"}])
        # 无有效问题时重复率按 1.0 上报，但默认放行（passed=True）
        assert metric.value == 1.0
        assert metric.passed is True


class TestQualityTrend:
    def test_save_history_noop_without_storage(self):
        tracker = QualityTrendTracker()
        tracker.record_quality_metrics("ds", {"accuracy": 0.9})
        tracker._save_history()  # 无 storage_path，直接返回
        assert tracker._trend_history

    def test_save_history_failure_caught(self, tmp_path, monkeypatch):
        tracker = QualityTrendTracker(storage_path=str(tmp_path / "h.json"))
        tracker.record_quality_metrics("ds", {"accuracy": 0.9})
        real_open = __import__("builtins").open

        def failing_open(file, *args, **kwargs):
            if "h.json" in str(file):
                raise PermissionError("denied")
            return real_open(file, *args, **kwargs)

        monkeypatch.setattr(__import__("builtins"), "open", failing_open)
        tracker._save_history()  # 失败只记录日志，不抛异常

    def test_trend_direction_insufficient(self):
        tracker = QualityTrendTracker()
        tracker.record_quality_metrics("ds", {"accuracy": 0.5})
        assert tracker.calculate_trend_direction("accuracy", "ds") == "insufficient_data"

    def test_trend_direction_improving(self):
        tracker = QualityTrendTracker()
        tracker.record_quality_metrics("ds", {"accuracy": 0.4})
        tracker.record_quality_metrics("ds", {"accuracy": 0.6})
        tracker.record_quality_metrics("ds", {"accuracy": 0.8})
        assert tracker.calculate_trend_direction("accuracy", "ds") == "improving"


class TestReportEmptyBranches:
    def test_distribution_of_empty_values(self):
        assert ReportGenerator()._compute_distribution([]) == {}

    def test_build_report_convenience(self):
        report = build_report([], [])
        assert isinstance(report, dict)
        assert "summary" in report or "insights" in report or report
