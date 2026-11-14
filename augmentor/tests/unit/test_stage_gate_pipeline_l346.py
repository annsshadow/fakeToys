# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later
"""L346 / A271：HTTP 门之外的关闸语义（管道内阶段与 CLI 直调）。

上一轮只挡了 7 个 HTTP 端点；管道与 CLI 绕过中间件，同名能力照跑。本轮按分档
补上：sampler 阶段关掉时保持回执键形状、rag / benchmark 的 CLI 关掉时非零退出。
零 parametrize（不扰动量面尺子）。
"""

import json
import re
from pathlib import Path
from types import SimpleNamespace

import pytest

from api.stage_gate import STAGE_ENDPOINTS, _DISABLED_TEMPLATE
from augmentor.cli.commands import analysis as analysis_cli
from augmentor.cli.commands import quality as quality_cli
from augmentor.config import AppConfig
from augmentor.pipeline import AugmentorPipeline

MESSAGE = "能力未启用：{section}.enabled 为 false"


def _items(tmp_path):
    data = tmp_path / "d.json"
    data.write_text(json.dumps([{"instruction": "a", "output": "b"}]), encoding="utf-8")
    return data


# ---------------------------------------------------------------- 管道阶段
def test_both_sampler_sites_share_the_same_condition():
    """两处判据逐字对账：管道与 CLI 都只读 config.sampler.enabled。

    刻意不抽 helper：`self.sampler.analyze_coverage` 是 census 的 _NAME_COLLISIONS
    撞车名（两节属性链），抽成模块级函数会让形状退化成一节、把尺子弄瞎。
    """
    pipeline_src = Path("augmentor/pipeline.py").read_bytes().decode("utf-8")
    analysis_src = Path("augmentor/cli/commands/analysis.py").read_bytes().decode("utf-8")
    assert "self.sampler.analyze_coverage(items) if self.config.sampler.enabled else {}" in pipeline_src
    assert "ActiveSampler().analyze_coverage(items) if config.sampler.enabled else {}" in analysis_src


def test_pipeline_analyze_dataset_really_computes_coverage_when_open(tmp_path):
    """零漂移反证臂：出厂全 True，覆盖分析照常算、回执三键齐全。"""
    report = AugmentorPipeline(AppConfig()).analyze_dataset(str(_items(tmp_path)))
    assert set(report) == {"coverage_analysis", "statistics", "dedup_report"}
    assert report["coverage_analysis"], "默认开闸不该产出空覆盖分析"


def test_pipeline_analyze_dataset_keeps_all_three_keys_when_gate_closed(tmp_path):
    """真身臂：关掉时不删键（否则前端 KeyError），另两段照常产出。"""
    cfg = AppConfig()
    cfg.sampler.enabled = False
    report = AugmentorPipeline(cfg).analyze_dataset(str(_items(tmp_path)))
    assert set(report) == {"coverage_analysis", "statistics", "dedup_report"}
    assert report["coverage_analysis"] == {}
    assert report["statistics"], "关 sampler 不该连统计段一起关掉"


# ---------------------------------------------------------------- CLI 直调
def _cli_args():
    return SimpleNamespace(input="x.json", output="out.json", format="llamaindex",
                           baseline=None, save_baseline=False, report=None,
                           fields=None, threshold=None)


def test_cli_rag_and_benchmark_exit_nonzero_when_closed(monkeypatch):
    """关掉时 SystemExit + 固定文案，不静默写产物；开回来后不再拦。"""
    monkeypatch.setattr(quality_cli, "_load_items", lambda p: [{"instruction": "a"}])
    written = []
    monkeypatch.setattr(quality_cli, "_save_items",
                        lambda items, path: written.append(path))
    cfg = AppConfig()
    for section, fn in (("rag", quality_cli.run_rag),
                        ("benchmark", quality_cli.run_benchmark)):
        setattr(getattr(cfg, section), "enabled", False)
        with pytest.raises(SystemExit) as exc:
            fn(_cli_args(), cfg)
        assert str(exc.value) == MESSAGE.format(section=section)
        setattr(getattr(cfg, section), "enabled", True)
    assert written == [], "关闸期间不该落任何产物"


def test_cli_analyze_keeps_coverage_key_when_sampler_closed(monkeypatch):
    """CLI 的 analyze 与管道同一口径，关掉时那一段是空 dict 而不是消失。"""
    captured = {}
    monkeypatch.setattr(analysis_cli, "_print", lambda payload: captured.update(payload))
    monkeypatch.setattr(analysis_cli, "_load_items",
                        lambda p: [{"instruction": "a", "output": "b"}])
    cfg = AppConfig()
    cfg.sampler.enabled = False
    analysis_cli.run_analyze(SimpleNamespace(input="x.json", fields=None, output=None), cfg)
    assert "coverage_analysis" in captured
    assert captured["coverage_analysis"] == {}
    assert "statistics" in captured and "dedup_report" in captured


# ---------------------------------------------------------------- 单一口径
def test_disabled_message_is_identical_across_http_and_cli():
    """HTTP 400 与 CLI SystemExit 用同一句文案，逐字对账。"""
    assert _DISABLED_TEMPLATE == MESSAGE
    src = Path(quality_cli.__file__).read_bytes().decode("utf-8")
    assert 'raise SystemExit(f"能力未启用：{section}.enabled 为 false")' in src


def test_cli_gate_sites_are_exactly_the_two_named_ones():
    """名册：CLI 侧只在这两个能力入口判闸，不蔓延到别处。"""
    src = Path(quality_cli.__file__).read_bytes().decode("utf-8")
    calls = re.findall(r'; _require_stage\(config, "(\w+)"\)', src)
    assert calls == ["rag", "benchmark"], f"CLI 关闸点变了：{calls}"


def test_five_sections_still_have_no_product_consumer():
    """明确登记而非遗漏：这五节今天没有可关的端点或阶段。"""
    assert {s for s, keys in STAGE_ENDPOINTS.items() if not keys} == {
        "context", "expander", "tracker", "active_learning", "frameworks"
    }
