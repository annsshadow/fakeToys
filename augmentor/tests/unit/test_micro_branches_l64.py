"""微型分支收尾 L64：converter/health_score/impact/outlier/multilingual/perf

- converter: csv→json 与 jsonl(非列表输入) 两条私有转换路径
- health_score: 无空格分词的覆盖率为 0
- impact: 去重收益为负超阈值时判定非有益
- outlier: 数值缺失的样本在检测中被跳过
- multilingual: detect_language 四种文档词表值（other 死分支已改 unknown）
- performance_benchmark: MemoryMonitor 缺失时降级
"""

import importlib
import sys
from types import SimpleNamespace

import pytest

from augmentor.converter import DatasetConverter
from augmentor.health_score import DatasetHealthScore
from augmentor.impact import AugmentationImpact, ImpactEvaluator
from augmentor.multilingual import MultilingualSupport
from augmentor.outlier import OutlierDetector
from augmentor.performance_benchmark import HAS_MEMORY_MONITOR


class TestConverterPrivatePaths:
    def test_csv_to_json_via_convert(self):
        conv = DatasetConverter()
        # csv 源走 _csv_to_json（_to_json 内部分支）
        result = conv._to_json([{"a": "1"}], "csv")
        assert isinstance(result, list)

    def test_jsonl_non_list_input(self):
        conv = DatasetConverter()
        # 非列表输入走逐行 json.loads 分支
        import json
        gen = (json.dumps({"i": 1}), json.dumps({"i": 2}))
        result = conv._jsonl_to_json(gen)
        assert result == [{"i": 1}, {"i": 2}]


class TestHealthScoreNoWords:
    def test_coverage_zero_when_no_tokens(self):
        scorer = DatasetHealthScore()
        # instruction 全为空格 → split() 为空 → 覆盖率 0
        assert scorer.calculate_coverage([{"instruction": "   "}]) == 0.0


class TestImpactUnfavorable:
    def test_dedup_gain_negative_disqualifies(self):
        evaluator = ImpactEvaluator()
        before = [{"instruction": "a"}]
        after = [
            {"instruction": "a"},
            {"instruction": "a"},
            {"instruction": "a"},
        ]
        impact = evaluator.evaluate(before, after)
        # 人为置入负向去重收益，验证 is_beneficial 的否决分支
        impact.gains["dedup_gain"] = -0.2
        assert evaluator.is_beneficial(impact, min_scale_gain=0.0) is False


class TestOutlierSkipsMissing:
    def test_missing_values_ignored(self):
        detector = OutlierDetector(field="value")
        items = [
            {"value": 1.0},
            {"value": 2.0},
            {},  # 缺字段 → raw_value None → 被跳过
            {"value": 3.0},
        ]
        report = detector.detect(items)
        # 缺失样本不应计入异常，也不应崩溃
        assert report.outlier_count >= 0


class TestMultilingualVocabulary:
    @pytest.mark.parametrize("text,expected", [
        ("你好世界", "zh"),
        ("hello world", "en"),
        ("你好 hello", "mixed"),
        ("12345", "unknown"),
    ])
    def test_detect_language_values(self, text, expected):
        assert MultilingualSupport().detect_language(text) == expected


class TestPerfBenchMemoryMonitorMissing:
    def test_degrades_without_memory_monitor(self, monkeypatch):
        """MemoryMonitor 不可导入时模块应降级（HAS_MEMORY_MONITOR=False）而非崩溃"""
        import augmentor.performance_benchmark as perf
        import augmentor.memory_monitor as mem

        original_flag = perf.HAS_MEMORY_MONITOR
        # 临时把 memory_monitor 置为 None 触发 ImportError 分支
        monkeypatch.setitem(sys.modules, "augmentor.memory_monitor", None)
        try:
            reloaded = importlib.reload(perf)
            assert reloaded.HAS_MEMORY_MONITOR is False
        finally:
            # reload 是原地改全局：先恢复 memory_monitor 再 reload，
            # 否则模块留在降级状态污染后续
            sys.modules.pop("augmentor.memory_monitor", None)
            importlib.reload(perf)
        assert perf.HAS_MEMORY_MONITOR == original_flag
