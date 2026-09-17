"""FeatureDetector 单元测试

特征检测是主动学习的输入，覆盖率与唯一性口径必须稳定。
"""

import pytest

from augmentor.feature_detect import (
    FeatureDetector,
    FeatureInfo,
    detect_features,
    INTENT_KEYWORDS,
)


@pytest.fixture
def sample_items():
    return [
        {"instruction": "如何申请租房？", "output": "登录官网", "level": 1},
        {"instruction": "租房多少钱？", "output": "1000元", "level": 2},
        {"instruction": "什么时候可以入住？", "output": "随时", "level": 1},
        {"instruction": "押金怎么退？", "output": "满一年后"},
    ]


class TestInit:
    def test_rejects_invalid_min_coverage(self):
        with pytest.raises(ValueError):
            FeatureDetector(min_coverage=1.5)

    def test_accepts_boundary_values(self):
        assert FeatureDetector(min_coverage=0).min_coverage == 0
        assert FeatureDetector(min_coverage=1).min_coverage == 1


class TestFieldCoverage:
    def test_full_coverage(self, sample_items):
        detector = FeatureDetector()
        assert detector._field_coverage(sample_items, "instruction") == 1.0

    def test_partial_coverage(self, sample_items):
        detector = FeatureDetector()
        # level 字段只有 3 条有值
        assert detector._field_coverage(sample_items, "level") == pytest.approx(0.75)

    def test_empty_items(self):
        detector = FeatureDetector()
        assert detector._field_coverage([], "instruction") == 0.0


class TestUniqueRatio:
    def test_all_unique(self, sample_items):
        detector = FeatureDetector()
        assert detector._unique_ratio(sample_items, "instruction") == 1.0

    def test_duplicates_lower_ratio(self):
        items = [{"instruction": "a"}, {"instruction": "a"}, {"instruction": "b"}]
        assert FeatureDetector()._unique_ratio(items, "instruction") == pytest.approx(2 / 3)

    def test_no_values(self, sample_items):
        assert FeatureDetector()._unique_ratio(sample_items, "missing") == 0.0


class TestInferType:
    def test_numeric(self, sample_items):
        assert FeatureDetector()._infer_type(sample_items, "level") == "numeric"

    def test_text(self, sample_items):
        assert FeatureDetector()._infer_type(sample_items, "instruction") == "text"

    def test_mixed(self):
        items = [{"x": 1}, {"x": "text"}]
        assert FeatureDetector()._infer_type(items, "x") == "mixed"

    def test_boolean_takes_priority(self):
        items = [{"x": True}, {"x": False}]
        assert FeatureDetector()._infer_type(items, "x") == "boolean"

    def test_unknown_when_no_values(self):
        items = [{"x": ""}, {"x": None}]
        assert FeatureDetector()._infer_type(items, "x") == "unknown"


class TestDetectFieldFeatures:
    def test_empty_items(self):
        assert FeatureDetector().detect_field_features([]) == []

    def test_feature_fields_complete(self, sample_items):
        features = FeatureDetector().detect_field_features(sample_items)
        names = [f.name for f in features]
        for expected in ("instruction", "output", "level"):
            assert expected in names
        assert names == sorted(names)

    def test_sparse_flag_threshold(self, sample_items):
        features = FeatureDetector(min_coverage=0.8).detect_field_features(sample_items)
        by_name = {f.name: f for f in features}
        assert by_name["level"].description == "稀疏特征"
        assert by_name["instruction"].description == "常规特征"

    def test_feature_info_to_dict(self):
        info = FeatureInfo(name="a", feature_type="text", coverage=0.5,
                           unique_ratio=0.8, description="d")
        d = info.to_dict()
        assert d["name"] == "a"
        assert d["coverage"] == 0.5
        assert d["description"] == "d"


class TestIntentDistribution:
    def test_how_intent(self, sample_items):
        distribution = FeatureDetector().detect_intent_distribution(sample_items)
        assert distribution.get("how") >= 2

    def test_cost_intent(self, sample_items):
        distribution = FeatureDetector().detect_intent_distribution(sample_items)
        assert distribution.get("cost") >= 1

    def test_empty_items(self):
        assert FeatureDetector().detect_intent_distribution([]) == {}

    def test_custom_text_field(self):
        items = [{"question": "多少钱？"}]
        distribution = FeatureDetector(text_field="question").detect_intent_distribution(items)
        assert distribution.get("cost") == 1

    def test_all_intents_have_keywords(self):
        for intent, keywords in INTENT_KEYWORDS.items():
            assert intent
            assert len(keywords) > 0


class TestFeatureMatrix:
    def test_default_fields_union(self, sample_items):
        matrix = FeatureDetector().build_feature_matrix(sample_items)
        assert len(matrix) == 4
        assert "instruction" in matrix[0]
        assert "level" in matrix[0]

    def test_explicit_fields(self, sample_items):
        matrix = FeatureDetector().build_feature_matrix(
            sample_items, fields=["instruction"]
        )
        assert all(set(row.keys()) == {"instruction"} for row in matrix)

    def test_empty_items(self):
        assert FeatureDetector().build_feature_matrix([]) == []


class TestDetect:
    def test_full_report_structure(self, sample_items):
        report = detect_features(sample_items)
        assert report["total_items"] == 4
        assert "field_features" in report
        assert "intent_distribution" in report
        assert "sparse_fields" in report

    def test_empty_dataset(self):
        report = detect_features([])
        assert report["total_items"] == 0
        assert report["field_features"] == []
        assert report["sparse_fields"] == []

    def test_sparse_fields_reported(self):
        items = [
            {"instruction": "a", "rare": "x"},
            {"instruction": "b"},
            {"instruction": "c"},
            {"instruction": "d"},
        ]
        report = detect_features(items, min_coverage=0.5)
        assert "rare" in report["sparse_fields"]
