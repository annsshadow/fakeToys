# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

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
        """非默认模型的密钥不落盘；`models.default` 归一化为模型名

        早前这里断言 `models["default"]` 里那个畸形字典要**原样保留**，
        等于把「密钥藏在 default 键下就能落盘」固化成了期望。而 `models.default`
        在 `load_config` 里必须是模型名字符串（`config.default_model = ...get('default')`），
        字典会让 `default_model` 变成一个 dict。因此这里断言的是修正后的语义。
        """
        cfg = AppConfig()
        cfg.models["default"] = {"api_key": "SHOULD_NOT_APPEAR", "secret_key": "SHOULD_NOT_APPEAR"}
        cfg.models["baidu"] = {"api_key": "DROP", "secret_key": "DROP"}
        out = tmp_path / "cfg.yaml"
        save_config(cfg, str(out))
        text = out.read_text(encoding="utf-8")
        assert "SHOULD_NOT_APPEAR" not in text  # 密钥不落盘
        assert "DROP" not in text  # 非 default 模型密钥去除
        assert "default: ernie" in text  # default 归一化为模型名

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
    """**A340 / L311 已改**：并行支路不再把单格式失败咽掉

    改前：并行路径在 `as_completed` 里吞掉单个格式的异常 ⇒ 回执少一格而调用方**看不出**
    （HTTP 面照样回 `success: true` —— 失败冒充成功）。改后整批 raise：失败的那一条由门口
    那把判据说话（写边抛 `DataValidationError` ⇒ 400；这里是通用 `RuntimeError` ⇒ 500）。
    """

    def test_export_all_formats_raises_the_failure(self, tmp_path, monkeypatch):
        exporter = Exporter()

        def _boom(self, items):
            raise RuntimeError("convert fail")

        monkeypatch.setattr(Exporter, "_convert_to_jsonl", _boom, raising=False)
        # 仅并行路径走 as_completed；改后它不再吞掉失败
        with pytest.raises(RuntimeError):
            exporter.export_all_formats(
                [{"instruction": "q", "output": "a"}],
                str(tmp_path),
                use_parallel=True,
            )

    def test_export_batch_raises_the_failure(self, tmp_path, monkeypatch):
        exporter = Exporter()

        def _boom(self, items):
            raise RuntimeError("convert fail")

        monkeypatch.setattr(Exporter, "_convert_to_jsonl", _boom, raising=False)
        with pytest.raises(RuntimeError):
            exporter.export_batch(
                {"ds": [{"instruction": "q", "output": "a"}]},
                str(tmp_path),
                formats=["jsonl", "alpaca"],
                use_parallel=True,
            )


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
        # check_dependencies 自 2026-10-24 起结果进程内缓存：patch 了探测函数
        # 就必须先清缓存，否则拿到的是先前用例缓存的**真**报告（本机 yaml 在位），
        # 「缺失降级」这条路根本不会被执行。finally 里再清一次：别让这份
        # 「yaml 缺失」的假报告漏给后续用例。
        check_dependencies.cache_clear()
        try:
            report = check_dependencies()
        finally:
            check_dependencies.cache_clear()
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
