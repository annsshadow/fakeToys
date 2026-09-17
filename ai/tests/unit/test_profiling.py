"""DataProfiler 单元测试

数据画像是质量门禁的前置诊断，统计口径必须稳定可复现。
"""

import json

import pytest

from augmentor.profiling import (
    DataProfiler,
    ProfilingConfig,
    _detect_language,
    profile_dataset,
)


@pytest.fixture
def sample_items():
    return [
        {"instruction": "如何申请租房？", "output": "登录官网申请"},
        {"instruction": "租房需要什么材料？", "output": "身份证、工作证明"},
        {"instruction": "如何申请租房？", "output": "重复的问题啊"},
        {"instruction": "", "output": ""},
        {"instruction": "what is rent", "output": "the rent amount"},
    ]


class TestDetectLanguage:
    """语言检测辅助函数"""

    def test_chinese(self):
        assert _detect_language("如何申请租房") == "zh"

    def test_english(self):
        assert _detect_language("hello world") == "en"

    def test_mixed(self):
        assert _detect_language("abc中文def") == "mixed"

    def test_unknown_empty(self):
        assert _detect_language("") == "unknown"

    def test_unknown_pure_digits(self):
        assert _detect_language("12345") == "unknown"

    def test_boundary_ratios(self):
        # 纯中文 -> zh
        assert _detect_language("中" * 10) == "zh"
        # 纯英文 -> en
        assert _detect_language("a" * 10) == "en"


class TestProfile:
    """画像主流程"""

    def test_empty_dataset(self):
        report = DataProfiler().profile([])
        assert report["total_items"] == 0
        assert report["duplicate_rate"] == 0.0
        assert report["top_keywords"] == []

    def test_total_items(self, sample_items):
        report = DataProfiler().profile(sample_items)
        assert report["total_items"] == 5

    def test_field_completeness(self, sample_items):
        report = DataProfiler().profile(sample_items)
        completeness = report["field_completeness"]
        assert "instruction" in completeness
        assert "output" in completeness
        # instruction 有 1 条为空：4/5 = 0.8
        assert completeness["instruction"] == pytest.approx(0.8)
        assert completeness["output"] == pytest.approx(0.8)

    def test_length_stats(self, sample_items):
        report = DataProfiler().profile(sample_items)
        stats = report["length_stats"]
        assert stats["min"] == 0
        assert stats["max"] >= 9
        assert 0 <= stats["avg"] <= stats["max"]
        assert "median" in stats

    def test_duplicate_rate(self, sample_items):
        report = DataProfiler().profile(sample_items)
        # 1 条重复 -> 1/5
        assert report["duplicate_rate"] == pytest.approx(0.2)

    def test_language_distribution_keys(self, sample_items):
        report = DataProfiler().profile(sample_items)
        dist = report["language_distribution"]
        # 中文样本存在
        assert dist.get("zh", 0) >= 2
        # 英文样本存在
        assert dist.get("en", 0) >= 1

    def test_top_keywords_not_empty(self, sample_items):
        report = DataProfiler().profile(sample_items)
        assert len(report["top_keywords"]) > 0
        assert all("keyword" in k and "count" in k for k in report["top_keywords"])

    def test_keywords_ranked_descending(self, sample_items):
        report = DataProfiler().profile(sample_items)
        counts = [k["count"] for k in report["top_keywords"]]
        assert counts == sorted(counts, reverse=True)


class TestConfig:
    """配置"""

    def test_default_config(self):
        config = ProfilingConfig()
        assert config.length_field == "instruction"
        assert config.top_keywords == 20
        assert config.min_keyword_length == 2

    def test_custom_length_field(self):
        profiler = DataProfiler(ProfilingConfig(length_field="output"))
        report = profiler.profile([
            {"instruction": "", "output": "abcd"}
        ])
        assert report["length_stats"]["max"] == 4

    def test_custom_top_keywords_limit(self):
        profiler = DataProfiler(ProfilingConfig(top_keywords=1))
        report = profiler.profile([
            {"instruction": "如何申请租房"},
            {"instruction": "如何申请买房"},
        ])
        assert len(report["top_keywords"]) <= 1

    def test_language_sample_limit(self):
        profiler = DataProfiler(ProfilingConfig(language_sample_limit=2))
        items = [{"instruction": "中文问题"}] * 10
        report = profiler.profile(items)
        assert sum(report["language_distribution"].values()) == 2


class TestExport:
    """画像导出"""

    def test_profile_to_json(self, sample_items):
        content = DataProfiler().profile_to_json(sample_items)
        data = json.loads(content)
        assert data["total_items"] == 5

    def test_save_profile(self, sample_items, tmp_path):
        path = str(tmp_path / "profile.json")
        written = DataProfiler().save_profile(sample_items, path)
        assert written == path
        with open(path, encoding="utf-8") as f:
            data = json.load(f)
        assert data["total_items"] == 5

    def test_profile_dataset_convenience(self, sample_items):
        report = profile_dataset(sample_items)
        assert report["total_items"] == 5

    def test_profile_dataset_with_output(self, sample_items, tmp_path):
        path = str(tmp_path / "out.json")
        report = profile_dataset(sample_items, output_path=path)
        assert report["total_items"] == 5
        import os
        assert os.path.exists(path)
