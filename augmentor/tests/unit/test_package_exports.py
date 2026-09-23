# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""augmentor 包导出测试

新增模块必须从包级别导出，保证 import augmentor 可直接使用。
"""

import pytest


class TestVersion:
    def test_version_bumped_to_3_0(self):
        import augmentor
        assert augmentor.__version__ == "3.0.0"


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

    def test_docker_compose_default_image_tag_follows_package(self):
        """`docker-compose.yml` 的默认镜像 tag 也是版本号的一份副本

        它写在 ``${IMAGE_TAG:-<默认值>}`` 的兜底位上：不检查的话，直接
        ``docker compose up`` 会得到一个 tag 与代码版本不符的镜像 ——
        和上面两处是同一类漂移，只是更容易被忘掉。
        """
        import re
        from pathlib import Path

        import augmentor

        compose = (
            Path(__file__).resolve().parent.parent.parent / "docker" / "docker-compose.yml"
        )
        if not compose.is_file():
            pytest.skip("docker 目录不存在")

        match = re.search(r"\$\{IMAGE_TAG:-([^}]+)\}", compose.read_text(encoding="utf-8"))
        assert match is not None, "docker-compose.yml 中找不到 IMAGE_TAG 的默认值"
        assert match.group(1) == augmentor.__version__


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

    def test_legacy_ambiguous_names_are_removed(self):
        """历史歧义名必须在 3.0 彻底移除，而不是继续解析到某个同名类

        这两个名字曾因「两个模块同名、包级导出互相覆盖」而指向与产出方
        不一致的类。保留它们（哪怕带弃用警告）会让 `isinstance` 判定继续
        出错；正确做法是让调用方显式选择语义明确的名字。
        """
        import pytest as _pytest

        import augmentor

        for name in ("QualityReport", "ValidationResult"):
            assert not hasattr(augmentor, name), f"augmentor.{name} 仍可访问"
            with _pytest.raises(AttributeError):
                getattr(augmentor, name)

        # 明确名字必须仍在，否则用户无路可走
        assert augmentor.PipelineQualityReport is not None
        assert augmentor.DatasetQualityReport is not None
        assert augmentor.DataValidationResult is not None
        assert augmentor.ConfigValidationResult is not None

    def test_no_pep562_fallback_hook(self):
        """包级不得再保留 __getattr__ 兜底（它会掩盖拼错的符号名）"""
        import augmentor

        assert "__getattr__" not in vars(augmentor)

    def test_unknown_attribute_raises(self):
        """未定义的属性仍应抛 AttributeError，不能被 __getattr__ 吞掉"""
        import pytest as _pytest

        import augmentor

        with _pytest.raises(AttributeError):
            augmentor.NotARealSymbol
