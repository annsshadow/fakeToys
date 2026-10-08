# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""leakage 泄漏检测模块测试

覆盖精确/近似泄漏判定、归一化、倒排候选约束、空数据集、
阈值边界、示例截断与包级导出。
"""

import pytest

from augmentor import LeakageDetector, LeakageReport, detect_leakage
from augmentor.leakage import _char_trigrams, _jaccard, _normalize


class TestNormalizeAndGrams:
    def test_normalize_lowercase_strip(self):
        assert _normalize("Hello World 123!!") == "helloworld123"
        assert _normalize("租房多少钱？") == "租房多少钱"

    def test_normalize_non_str(self):
        assert _normalize(None) == ""
        assert _normalize(123) == ""

    def test_trigrams(self):
        assert _char_trigrams("abcd") == {"abc", "bcd"}
        assert _char_trigrams("ab") == {"ab"}
        assert _char_trigrams("") == set()

    def test_jaccard(self):
        assert _jaccard({"a", "b"}, {"a", "b"}) == 1.0
        assert _jaccard({"a"}, {"b"}) == 0.0
        assert _jaccard(set(), {"a"}) == 0.0


class TestExactLeak:
    def test_exact_match_detected(self):
        train = [{"instruction": "如何申请？"}]
        test = [{"instruction": "如何申请？"}]
        report = detect_leakage(train, test)
        assert report.exact_leaks == 1
        assert report.fuzzy_leaks == 0

    def test_normalization_ignores_punctuation(self):
        """同一问题不同标点/大小写需判为精确泄漏"""
        train = [{"instruction": "How much is rent?"}]
        test = [{"instruction": "how much is rent"}]
        report = detect_leakage(train, test)
        assert report.exact_leaks == 1


class TestFuzzyLeak:
    def test_near_duplicate_detected(self):
        train = [{"instruction": "北京朝阳区两居室每个月租金大概是多少呢"}]
        test = [{"instruction": "北京朝阳区两居室每个月租金大概是多少"}]
        report = detect_leakage(train, test)
        assert report.fuzzy_leaks == 1
        assert report.exact_leaks == 0

    def test_distinct_not_flagged(self):
        train = [{"instruction": "北京朝阳区两居室租金"}]
        test = [{"instruction": "上海浦东一居室押金"}]
        report = detect_leakage(train, test)
        assert report.total_leaks == 0
        assert report.is_clean is True

    def test_threshold_controls_sensitivity(self):
        train = [{"instruction": "租房多少钱"}]
        test = [{"instruction": "租房多少"}]
        # 高阈值不判泄漏，低阈值判泄漏
        strict = detect_leakage(train, test, fuzzy_threshold=0.99)
        loose = detect_leakage(train, test, fuzzy_threshold=0.5)
        assert strict.total_leaks < loose.total_leaks or strict.total_leaks == 0


class TestReport:
    def test_report_properties_and_to_dict(self):
        train = [{"instruction": "a"}, {"instruction": "b"}]
        test = [{"instruction": "a"}]
        report = detect_leakage(train, test)
        assert report.train_size == 2
        assert report.test_size == 1
        assert report.leak_rate == pytest.approx(1.0)
        d = report.to_dict()
        assert d["total_leaks"] == 1
        assert d["is_clean"] is False

    def test_empty_test_returns_zero_rate(self):
        report = detect_leakage([{"instruction": "a"}], [])
        assert report.leak_rate == 0.0
        assert report.is_clean is True
        assert report.total_leaks == 0

    def test_examples_capped(self):
        train = [{"instruction": f"q{i}"} for i in range(5)]
        test = [{"instruction": f"q{i}"} for i in range(5)]
        detector = LeakageDetector(min_examples=2)
        report = detector.detect(train, test)
        assert report.exact_leaks == 5
        assert len(report.leaked_examples) == 2

    def test_custom_fields(self):
        train = [{"output": "回答A"}]
        test = [{"output": "回答A"}]
        report = detect_leakage(train, test, fields=["output"])
        assert report.exact_leaks == 1


class TestPackageExports:
    def test_exports_available(self):
        import augmentor

        assert augmentor.LeakageDetector is LeakageDetector
        assert augmentor.LeakageReport is LeakageReport
        assert augmentor.detect_leakage is detect_leakage


class TestFuzzyThresholdGuardL181:
    """L181（A172）：fuzzy_threshold 构造期判 (0, 1]——假零/越界/坏形状全拒，正区间放行。

    判据权威住 LeakageDetector 构造器（A77）：detect_leakage 与 API/CLI 面经它吃到同一档。
    """

    def test_valid_open_upper_interval_passes(self):
        for ok in (0.1, 0.8, 1.0, 1):
            assert LeakageDetector(fuzzy_threshold=ok).fuzzy_threshold == ok

    @pytest.mark.parametrize("bad", [0.0, -0.5, 1.5, 2.0, None, True, False,
                                     float("nan"), "0.8"])
    def test_bad_shapes_rejected(self, bad):
        from augmentor.exceptions import DataValidationError

        with pytest.raises(DataValidationError):
            LeakageDetector(fuzzy_threshold=bad)

    def test_detect_leakage_delegates_guard(self):
        from augmentor.exceptions import DataValidationError

        train, test = [{"instruction": "a"}], [{"instruction": "b"}]
        with pytest.raises(DataValidationError):
            detect_leakage(train, test, fuzzy_threshold=0.0)
        # 合法值仍正常出报告（拒判据没有修过头）
        assert detect_leakage(train, test, fuzzy_threshold=0.5).total_leaks == 0


class TestMinExamplesGuardL190:
    """L190（B260）：min_examples 计数旋钮收口——非负整数放行、负数/坏形状拒。

    改前 `self.min_examples = min_examples` 零判据：负数让 `[:n]` 切片换语义
    （`[:-2]` = 丢掉最后 2 条，「要 2 条」变成「要 N-2 条」，silent 错答族），
    None/非整数/bool 一律原样进切片下标。0 是合法「报告不带示例」意图，
    与「没传参数」区分开（require_count minimum=0 口径）。
    """

    @pytest.mark.parametrize("ok", [0, 1, 10, 100])
    def test_non_negative_counts_pass(self, ok):
        assert LeakageDetector(min_examples=ok).min_examples == ok

    @pytest.mark.parametrize("bad", [-1, -2, -10, None, True, False, 2.0, "3"])
    def test_bad_shapes_rejected(self, bad):
        from augmentor.exceptions import DataValidationError

        with pytest.raises(DataValidationError):
            LeakageDetector(min_examples=bad)

    def test_zero_meaning_is_no_examples_kept(self):
        """0 档语义钉死：精确泄漏也一条示例都不留（报告面照出统计）。"""
        train = [{"instruction": "exact leak phrase"}]
        test = [{"instruction": "exact leak phrase"}]
        report = LeakageDetector(min_examples=0).detect(train, test)
        assert report.total_leaks >= 1
        assert report.leaked_examples == []
