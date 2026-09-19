"""OutlierDetector 单元测试

异常值检测必须可解释：方法选择、边界计算与过滤行为都要有明确断言。
"""

import pytest

from augmentor.outlier import (
    OutlierDetector,
    OutlierReport,
    detect_outliers,
)


class TestInit:
    """初始化校验"""

    def test_rejects_unknown_method(self):
        """未知方法必须报错"""
        with pytest.raises(ValueError, match="不支持"):
            OutlierDetector(method="mad")

    def test_rejects_non_positive_threshold(self):
        """阈值必须为正数"""
        with pytest.raises(ValueError, match="threshold"):
            OutlierDetector(threshold=0)
        with pytest.raises(ValueError):
            OutlierDetector(threshold=-1)

    def test_default_settings(self):
        """默认使用 zscore/3.0/length"""
        detector = OutlierDetector()
        assert detector.method == "zscore"
        assert detector.threshold == 3.0
        assert detector.field == "length"


class TestExtractValues:
    """数值提取"""

    def test_missing_field_returns_none(self):
        detector = OutlierDetector()
        assert detector._extract_values([{"other": 1}]) == [None]

    def test_non_numeric_coerced_or_none(self):
        detector = OutlierDetector()
        assert detector._extract_values([
            {"length": "5.5"},
            {"length": "abc"},
            {"length": 3},
        ]) == [5.5, None, 3.0]

    def test_all_none(self):
        detector = OutlierDetector()
        assert detector._extract_values([{}, {}]) == [None, None]


class TestStatisticsHelpers:
    """统计辅助方法"""

    def test_mean_std_basic(self):
        detector = OutlierDetector()
        mean, std = detector._mean_std([1, 2, 3, 4, 5])
        assert mean == pytest.approx(3.0)
        assert std == pytest.approx(1.4142135, abs=1e-4)

    def test_mean_std_empty(self):
        detector = OutlierDetector()
        assert detector._mean_std([]) == (0.0, 0.0)

    def test_mean_std_constant(self):
        detector = OutlierDetector()
        _, std = detector._mean_std([4, 4, 4])
        assert std == pytest.approx(0.0)

    def test_quantile_midpoints(self):
        detector = OutlierDetector()
        values = [1, 2, 3, 4]
        assert detector._quantile(values, 0.0) == 1
        assert detector._quantile(values, 1.0) == 4
        assert detector._quantile(values, 0.5) == pytest.approx(2.5)
        assert detector._quantile([7], 0.5) == 7
        assert detector._quantile([], 0.5) == 0.0


class TestDetect:
    """异常检测"""

    def test_empty_dataset(self):
        report = OutlierDetector().detect([])
        assert report.total_items == 0
        assert report.outlier_count == 0
        assert report.outlier_rate == 0.0

    def test_single_item_no_outlier(self):
        report = OutlierDetector().detect([{"length": 100}])
        assert report.outlier_count == 0

    def test_zscore_detects_long_outlier(self):
        """zscore 应识别极端过长的样本"""
        items = [{"length": 10 + i} for i in range(20)]
        items.append({"length": 1000})
        report = OutlierDetector(method="zscore", threshold=2.5).detect(items)

        assert report.outlier_count >= 1
        assert report.outliers[0]["value"] == 1000.0
        assert report.outliers[0]["index"] == len(items) - 1

    def test_zscore_one_sided_ignores_short_side(self):
        """单侧 zscore 只检测上侧"""
        items = [{"length": 10}] * 10 + [{"length": 0}]
        report = OutlierDetector(method="zscore_one_sided").detect(items)
        # 0 是下侧极端值，单侧方法不应报出
        assert all(o["value"] != 0.0 for o in report.outliers)

    def test_iqr_detects_outlier(self):
        """IQR 方法应识别超出箱体的样本"""
        items = [{"length": 10}] * 30 + [{"length": 500}]
        report = OutlierDetector(method="iqr", threshold=1.5).detect(items)
        assert report.outlier_count == 1
        assert report.outliers[0]["value"] == 500.0

    def test_constant_data_no_outliers(self):
        """常数序列标准差为 0 时不应除零崩溃"""
        items = [{"length": 5}] * 5
        report = OutlierDetector().detect(items)
        assert report.outlier_count == 0

    def test_report_to_dict_fields(self):
        report = OutlierReport(total_items=10)
        d = report.to_dict()
        for key in ("total_items", "outlier_count", "outlier_rate",
                    "method", "threshold", "field", "outliers"):
            assert key in d


class TestFilter:
    """异常过滤"""

    def test_filter_removes_outliers_only(self):
        items = [{"length": 10}] * 20 + [{"length": 9999}]
        detector = OutlierDetector(threshold=2.0)
        kept = detector.filter(items)
        assert len(kept) == 20
        assert all(item["length"] == 10 for item in kept)

    def test_filter_no_outliers_returns_copy(self):
        items = [{"length": 10}, {"length": 12}]
        kept = OutlierDetector().filter(items)
        assert kept == items
        assert kept is not items

    def test_filter_empty(self):
        assert OutlierDetector().filter([]) == []


class TestAttachLength:
    """长度字段附加"""

    def test_attaches_length_from_source(self):
        items = [{"instruction": "短"}, {"instruction": "这是一段较长的文本内容"}]
        enriched = OutlierDetector().attach_length_field(items)
        assert enriched[0]["length"] == 1
        assert enriched[1]["length"] == 11

    def test_does_not_mutate_input(self):
        items = [{"instruction": "abc"}]
        before = dict(items[0])
        OutlierDetector().attach_length_field(items)
        assert items[0] == before
        assert "length" not in items[0]

    def test_custom_source_field(self):
        items = [{"output": "ab"}]
        enriched = OutlierDetector().attach_length_field(items, source_field="output")
        assert enriched[0]["length"] == 2

    def test_missing_source_field_defaults_to_zero(self):
        enriched = OutlierDetector().attach_length_field([{}])
        assert enriched[0]["length"] == 0


class TestConvenienceFunction:
    """便捷函数"""

    def test_detect_outliers_passthrough(self):
        items = [{"length": 10}] * 10 + [{"length": 5000}]
        report = detect_outliers(items, threshold=2.0)
        assert report.outlier_count >= 1

    def test_detect_outliers_defaults(self):
        report = detect_outliers([])
        assert report.total_items == 0
        assert report.method == "zscore"
