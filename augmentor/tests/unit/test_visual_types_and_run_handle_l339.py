# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later
"""L339 行为面：A267 出图清单真的驱动产物 / A364 log_metric 的 run 把手真的换目标。

零 parametrize（l97 / l98 把全仓 parametrize 三面钉成现量数，新增参数化会扰动量面）。
"""

import json
from pathlib import Path

from augmentor.tracker import ExperimentTracker
from augmentor.visualizer import DataVisualizer

ROOT = Path(__file__).resolve().parents[2]


# ---------------------------------------------------------------------------
# A267：types 从「出厂死清单」变成出图清单
# ---------------------------------------------------------------------------


def _stub_charts(viz, monkeypatch, timeline="timeline.png"):
    """四类老图一律返回空串（本机没装 matplotlib / wordcloud 时的真实形状）"""
    monkeypatch.setattr(viz, "generate_wordcloud", lambda items, key: "")
    monkeypatch.setattr(viz, "generate_length_distribution", lambda items, key: "")
    monkeypatch.setattr(viz, "generate_topic_cluster", lambda items, key: "")
    monkeypatch.setattr(viz, "generate_quality_distribution", lambda scores: "")
    monkeypatch.setattr(viz, "generate_timeline", lambda data: timeline)


def test_explicit_types_make_timeline_a_real_product(monkeypatch):
    """出厂清单点名 timeline ⇒ 今天第一次真的出图（A267 的行为差就在这里）"""
    viz = DataVisualizer()
    _stub_charts(viz, monkeypatch)
    out = viz.generate_all_visualizations(
        [{"instruction": "问题一"}, {"instruction": "问题二"}],
        types=["wordcloud", "length_distribution", "topic_cluster", "timeline"],
    )
    assert out == {"timeline": "timeline.png"}


def test_empty_types_produces_no_chart(monkeypatch):
    viz = DataVisualizer()
    _stub_charts(viz, monkeypatch)
    assert viz.generate_all_visualizations([{"instruction": "x"}], types=[]) == {}


def test_unknown_type_is_silently_ignored(monkeypatch):
    viz = DataVisualizer()
    _stub_charts(viz, monkeypatch)
    assert viz.generate_all_visualizations([{"instruction": "x"}], types=["not_a_chart"]) == {}


def test_none_types_keeps_the_legacy_face(monkeypatch):
    """不传 types 是向后兼容档：时间线一次都不许被调（既有守卫按这一档钉着）"""
    viz = DataVisualizer()
    called = []
    _stub_charts(viz, monkeypatch)
    monkeypatch.setattr(viz, "generate_timeline", lambda data: called.append(data) or "timeline.png")
    out = viz.generate_all_visualizations([{"instruction": "x"}], scores=[0.5])
    assert called == [] and "timeline" not in out


def test_quality_distribution_still_needs_scores(monkeypatch):
    """清单点名也要 scores：两个产品调用点今天都不传 ⇒ 该类恒跳过，写进日志块"""
    viz = DataVisualizer()
    _stub_charts(viz, monkeypatch)
    monkeypatch.setattr(viz, "generate_quality_distribution", lambda scores: "quality.png")
    assert "quality_distribution" not in viz.generate_all_visualizations(
        [{"instruction": "x"}], types=["quality_distribution"])
    assert viz.generate_all_visualizations(
        [{"instruction": "x"}], scores=[0.5], types=["quality_distribution"]) == {
        "quality_distribution": "quality.png"}


def test_both_product_callers_pass_the_config_list():
    """静态面：pipeline 与 CLI 两个调用点都显式把 config 的清单交给生成器"""
    pipeline = (ROOT / "augmentor" / "pipeline.py").read_text(encoding="utf-8")
    cli = (ROOT / "augmentor" / "cli" / "commands" / "analysis.py").read_text(encoding="utf-8")
    assert "types=self.config.visualization.types" in pipeline
    assert "types=config.visualization.types" in cli


# ---------------------------------------------------------------------------
# A364：log_metric 的 run 把手
# ---------------------------------------------------------------------------


def _read_run(tmp_path, run_id):
    return json.loads((tmp_path / f"{run_id}.json").read_text(encoding="utf-8"))


def test_named_run_gets_the_metric_without_touching_the_current_one(tmp_path):
    """指定 run 时从磁盘读该 run 写入，实例级 _current_experiment 一字不动"""
    tr = ExperimentTracker(storage_dir=str(tmp_path))
    tr.start_experiment("runA", "v1", "model")
    tr.start_experiment("runB", "v2", "model")  # 当前实验是 runB
    tr.log_metric("loss", 0.5, experiment_id="runA")
    assert _read_run(tmp_path, "runA")["metrics"] == {"loss": [0.5]}
    assert _read_run(tmp_path, "runB")["metrics"] == {}
    assert tr._current_experiment.experiment_id == "runB"


def test_named_run_works_even_without_a_current_experiment(tmp_path):
    """把手的意义就是不依赖实例状态：没有 start_experiment 也能写到指定 run"""
    tr = ExperimentTracker(storage_dir=str(tmp_path))
    tr.start_experiment("runA", "v1", "model")
    tr.end_experiment()  # 清空 _current_experiment
    tr.log_metric("loss", 0.25, experiment_id="runA")
    assert _read_run(tmp_path, "runA")["metrics"] == {"loss": [0.25]}


def test_unknown_run_is_a_noop_and_creates_nothing(tmp_path):
    tr = ExperimentTracker(storage_dir=str(tmp_path))
    tr.log_metric("loss", 0.5, experiment_id="missing")
    assert list(tmp_path.glob("*.json")) == []


def test_legacy_two_arg_call_still_writes_the_current_experiment(tmp_path):
    """旧签名不破坏：两参调用仍写实例当前实验（向后兼容反证臂）"""
    tr = ExperimentTracker(storage_dir=str(tmp_path))
    tr.start_experiment("runA", "v1", "model")
    tr.log_metric("loss", 0.75)
    assert _read_run(tmp_path, "runA")["metrics"] == {"loss": [0.75]}
