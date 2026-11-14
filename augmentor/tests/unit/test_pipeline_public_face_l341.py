# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later
"""L341 / A273：管道公开面缩到只剩真有读者的属性，SDK 能力一个都没删。

零 parametrize（l97 / l98 把全仓 parametrize 三面钉成现量数）。
"""

import inspect
from pathlib import Path

import augmentor
from augmentor import AugmentorPipeline, DomainExpander, ExperimentTracker, load_config
from augmentor.sampler import ActiveSampler
from augmentor.visualizer import DataVisualizer

ROOT = Path(__file__).resolve().parents[2]
PIPELINE_SRC = (ROOT / "augmentor" / "pipeline.py").read_text(encoding="utf-8")


def _pipeline():
    return AugmentorPipeline(load_config(str(ROOT / "config.yaml")))


def test_the_two_unconsumed_attributes_are_gone():
    instance = _pipeline()
    assert not hasattr(instance, "expander")
    assert not hasattr(instance, "tracker")


def test_the_rest_of_the_component_face_is_intact():
    """反证：不是把 _init_components 整段删了，只撤那两只"""
    instance = _pipeline()
    assert isinstance(instance.sampler, ActiveSampler)
    assert isinstance(instance.visualizer, DataVisualizer)
    assert hasattr(instance, "version_manager")


def test_the_sdk_classes_are_still_exported_and_usable():
    """防「删过头」：能力留在 SDK，将来要接消费者不必重写"""
    assert augmentor.DomainExpander is DomainExpander
    assert augmentor.ExperimentTracker is ExperimentTracker
    assert "strategies" in inspect.signature(DomainExpander.batch_expand).parameters
    # A364 的 run 把手仍在：这是 SDK 层的公开签名，不随管道属性一起消失
    assert "experiment_id" in inspect.signature(ExperimentTracker.log_metric).parameters


def test_the_constructors_are_not_coming_back_by_accident():
    """静态面：源码里不许再出现那两行赋值（注释里提到不算）"""
    assert "self.expander =" not in PIPELINE_SRC
    assert "self.tracker =" not in PIPELINE_SRC


def test_sdk_tracker_still_records_a_metric(tmp_path):
    """能力面真跑一遍：run 把手写得出文件，没被这次缩面误伤"""
    tr = ExperimentTracker(storage_dir=str(tmp_path))
    tr.start_experiment("runL341", "v1", "model")
    tr.log_metric("loss", 0.5, experiment_id="runL341")
    import json

    saved = json.loads((tmp_path / "runL341.json").read_text(encoding="utf-8"))
    assert saved["metrics"] == {"loss": [0.5]}
