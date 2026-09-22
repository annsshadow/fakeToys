# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""augmentor 包导出测试

新增模块必须从包级别导出，保证 import augmentor 可直接使用。
"""

import pytest


class TestVersion:
    def test_version_bumped_to_2_2(self):
        import augmentor
        assert augmentor.__version__ == "2.2.0"


class TestVersionConsistency:
    """版本号单一来源

    ``augmentor.__version__`` 是唯一权威声明。FastAPI 应用元数据、前端
    ``package.json`` 都必须与它一致——历史上这里漂移成了 2.2.0 / 2.0.0 / 1.0.0
    三个值，且没有任何检查会发现。
    """

    def test_fastapi_app_version_follows_package(self):
        import augmentor
        from api.main import app

        assert app.version == augmentor.__version__

    def test_web_package_json_follows_package(self):
        import json
        from pathlib import Path

        import augmentor

        web_pkg = Path(__file__).resolve().parent.parent.parent / "web" / "package.json"
        if not web_pkg.is_file():
            pytest.skip("前端目录不存在")
        assert json.loads(web_pkg.read_text(encoding="utf-8"))["version"] == augmentor.__version__


class TestOutlierExports:
    def test_outlier_detector_exported(self):
        from augmentor import OutlierDetector, OutlierReport, detect_outliers
        assert OutlierDetector is not None
        assert OutlierReport is not None
        assert callable(detect_outliers)

    def test_detect_outliers_integration(self):
        from augmentor import detect_outliers
        report = detect_outliers([{"length": 10}] * 10 + [{"length": 9999}], threshold=2.0)
        assert report.outlier_count >= 1


class TestProfilingExports:
    def test_profiling_exported(self):
        from augmentor import DataProfiler, ProfilingConfig, profile_dataset
        assert DataProfiler is not None
        assert ProfilingConfig is not None
        assert callable(profile_dataset)

    def test_profile_dataset_smoke(self):
        from augmentor import profile_dataset
        report = profile_dataset([{"instruction": "q", "output": "a"}])
        assert report["total_items"] == 1


class TestFeatureDetectExports:
    def test_feature_detector_exported(self):
        from augmentor import FeatureDetector, FeatureInfo, detect_features
        assert FeatureDetector is not None
        assert FeatureInfo is not None
        assert callable(detect_features)

    def test_detect_features_smoke(self):
        from augmentor import detect_features
        report = detect_features([{"instruction": "如何申请"}])
        assert report["total_items"] == 1
        assert "field_features" in report

    def test_all_new_names_in_all(self):
        import augmentor
        for name in ("OutlierDetector", "OutlierReport", "detect_outliers",
                     "DataProfiler", "ProfilingConfig", "profile_dataset",
                     "FeatureDetector", "FeatureInfo", "detect_features"):
            assert name in augmentor.__all__, f"{name} 未加入 __all__"


class TestSymbolIntegrity:
    """包级导出符号的唯一性

    历史上 `QualityReport` 与 `ValidationResult` 各被从两个模块导入到包级别，
    后一次静默覆盖前一次，导致 `augmentor.QualityReport` 指向的类与
    `ReportGenerator.generate()` 实际返回的类不是同一个
    （见 docs/plans/2026-09-21-001-audit-augmentor-fullstack-optimization-plan.md F2）。
    """

    def test_no_duplicate_names_in_all(self):
        """__all__ 不得有重复项"""
        import augmentor

        duplicates = sorted(
            {n for n in augmentor.__all__ if augmentor.__all__.count(n) > 1}
        )
        assert duplicates == [], f"__all__ 存在重复项: {duplicates}"

    def test_all_entries_are_resolvable(self):
        """__all__ 中的每个名字都必须可解析"""
        import augmentor

        missing = [n for n in augmentor.__all__ if not hasattr(augmentor, n)]
        assert missing == [], f"__all__ 中无法解析的名字: {missing}"

    def test_pipeline_quality_report_points_to_report_module(self):
        """ReportGenerator 产出的类必须就是包级导出的那个"""
        import augmentor
        from augmentor.report import ReportGenerator

        assert augmentor.PipelineQualityReport.__module__ == "augmentor.report"

        from augmentor.quality import QualityScore

        score = QualityScore(
            semantic_similarity=0.9,
            relevance=0.8,
            diversity=0.7,
            total_score=0.8,
            passed=True,
        )
        report = ReportGenerator().generate(
            [{"instruction": "q", "output": "a"}], [score]
        )
        assert isinstance(report, augmentor.PipelineQualityReport), (
            "ReportGenerator.generate() 的返回值必须能通过包级导出的类型做 isinstance 判定"
        )

    def test_dataset_quality_report_points_to_quality_report_module(self):
        """QualityReporter 产出的类必须就是包级导出的那个"""
        import augmentor
        from augmentor.quality_report import QualityReporter

        assert augmentor.DatasetQualityReport.__module__ == "augmentor.quality_report"
        report = QualityReporter().generate_report(
            [{"instruction": "q", "output": "a"}], dataset_name="t"
        )
        assert isinstance(report, augmentor.DatasetQualityReport)

    def test_validation_results_are_distinguishable(self):
        """两个同名 ValidationResult 必须可通过包级名字区分"""
        import augmentor

        assert augmentor.DataValidationResult.__module__ == "augmentor.validation"
        assert (
            augmentor.ConfigValidationResult.__module__
            == "augmentor.config_validator"
        )
        assert augmentor.DataValidationResult is not augmentor.ConfigValidationResult

    def test_legacy_ambiguous_names_warn_but_still_work(self):
        """历史歧义名保留一个版本，但必须发出弃用警告"""
        import warnings

        import augmentor

        with warnings.catch_warnings(record=True) as caught:
            warnings.simplefilter("always")
            legacy_quality = augmentor.QualityReport
            legacy_validation = augmentor.ValidationResult

        assert legacy_quality is augmentor.DatasetQualityReport
        assert legacy_validation is augmentor.ConfigValidationResult
        assert len(caught) == 2
        assert all(issubclass(w.category, DeprecationWarning) for w in caught)

    def test_unknown_attribute_raises(self):
        """未定义的属性仍应抛 AttributeError，不能被 __getattr__ 吞掉"""
        import pytest as _pytest

        import augmentor

        with _pytest.raises(AttributeError):
            augmentor.NotARealSymbol
