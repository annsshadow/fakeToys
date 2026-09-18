"""微型分支收尾 L67：config/export/profiling/diagnostics/expander 残留分支

- config: save_config 对非 default 模型去除密钥、load_config 幂等
- export: 全格式导出/批量导出失败被吞并
- profiling: 语言检测 mixed 判定、空数据集画像
- diagnostics: yaml 缺失降级提示
- expander: 并行生成种子失败被吞并
"""

import json

import pytest

from augmentor.config import AppConfig, load_config, save_config
from augmentor.diagnostics import check_dependencies
from augmentor.export import Exporter
from augmentor.expander import DomainExpander
from augmentor.profiling import DataProfiler


class TestConfigSaveSecret:
    def test_non_default_model_secret_stripped(self, tmp_path):
        cfg = AppConfig()
        cfg.models["default"] = {"api_key": "KEEP", "secret_key": "KEEP"}
        cfg.models["baidu"] = {"api_key": "DROP", "secret_key": "DROP"}
        out = tmp_path / "cfg.yaml"
        save_config(cfg, str(out))
        text = out.read_text(encoding="utf-8")
        assert "KEEP" in text  # default 保留
        assert "DROP" not in text  # 非 default 去除

    def test_load_config_idempotent(self, tmp_path):
        path = tmp_path / "c.yaml"
        path.write_text(
            "app:\n  name: x\n  version: '1'\nmodels:\n  default: ernie\n",
            encoding="utf-8",
        )
        c1 = load_config(str(path))
        c2 = load_config(str(path))
        assert c1.default_model == c2.default_model == "ernie"


class TestExportFailureSwallowed:
    def test_export_all_formats_swallows_errors(self, tmp_path, monkeypatch):
        exporter = Exporter()

        def _boom(self, items):
            raise RuntimeError("convert fail")

        monkeypatch.setattr(Exporter, "_convert_to_jsonl", _boom, raising=False)
        # 仅并行路径会在 as_completed 中吞并单格式失败
        results = exporter.export_all_formats(
            [{"instruction": "q", "output": "a"}],
            str(tmp_path),
            use_parallel=True,
        )
        assert "jsonl" not in results
        assert "alpaca" in results

    def test_export_batch_swallows_errors(self, tmp_path, monkeypatch):
        exporter = Exporter()

        def _boom(self, items):
            raise RuntimeError("convert fail")

        monkeypatch.setattr(Exporter, "_convert_to_jsonl", _boom, raising=False)
        results = exporter.export_batch(
            {"ds": [{"instruction": "q", "output": "a"}]},
            str(tmp_path),
            formats=["jsonl", "alpaca"],
            use_parallel=True,
        )
        assert "jsonl" not in results["ds"]
        assert "alpaca" in results["ds"]


class TestProfilingBranches:
    def test_language_mixed_detection(self):
        profiler = DataProfiler()
        # 中英各半 → mixed（cjk_ratio 介于 0.2 与 0.8 之间）
        dist = profiler._language_distribution(
            [{"instruction": "你好 hello world 申请"}]
        )
        assert dist.get("mixed", 0) >= 1

    def test_profile_empty_dataset(self):
        profiler = DataProfiler()
        report = profiler.profile([])
        assert report["total_items"] == 0
        assert report["field_completeness"] == {}


class TestDiagnosticsYamlMissing:
    def test_yaml_missing_degrades(self, monkeypatch):
        import augmentor.diagnostics as diag

        def fake_importable(name):
            return name != "yaml"

        monkeypatch.setattr(diag, "_importable", fake_importable)
        report = check_dependencies()
        assert "yaml" in report.missing or not report.all_required_present
        assert any("yaml" in d for d in report.degraded_features)


class TestExpanderParallelFailure:
    def test_parallel_seed_failure_swallowed(self, monkeypatch):
        expander = DomainExpander(model_backend=None, max_workers=2)

        def _boom(self, topic, n):
            raise RuntimeError("expand fail")

        monkeypatch.setattr(
            DomainExpander, "_generate_seeds_for_topic", _boom, raising=False
        )
        topics = [
            {"topic": "租房", "strategy": "similar"},
            {"topic": "押金", "strategy": "related"},
        ]
        # 全部失败仍应返回空列表而非抛异常
        assert expander.generate_seeds_from_topics(topics, use_parallel=True) == []
