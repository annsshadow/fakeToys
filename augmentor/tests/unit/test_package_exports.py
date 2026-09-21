"""augmentor 包导出测试

新增模块必须从包级别导出，保证 import augmentor 可直接使用。
"""

import pytest


class TestVersion:
    def test_version_bumped_to_2_2(self):
        import augmentor
        assert augmentor.__version__ == "2.2.0"


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
