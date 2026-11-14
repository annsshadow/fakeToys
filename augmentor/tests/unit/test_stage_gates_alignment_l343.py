# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later
"""L343 / A271：十只阶段闸出厂值与今天行为对齐，且有真实读者。

用户代拍口径：接线时出厂值同步改成与今天行为一致，避免「接上闸 = 一次关掉十个功能」。
本轮三侧（dataclass 默认 / config.yaml / 统一读取函数）全部对齐到 True；
census 的零读者名册移出这十只。零 parametrize。
"""

import pytest

from augmentor import load_config
from api.deps import stage_gates
from augmentor.config import AppConfig

SECTIONS = ("context", "sampler", "expander", "tracker", "rag", "evaluation",
            "multimodal", "benchmark", "active_learning", "frameworks")


@pytest.fixture(scope="module")
def config():
    return load_config("config.yaml")


def test_the_ten_gates_are_all_on_in_both_default_sides(config):
    """dataclass 出厂与 config.yaml 写出值都必须全开（零行为漂移）"""
    bare = AppConfig()
    for section in SECTIONS:
        assert getattr(getattr(bare, section), "enabled") is True, section
        assert getattr(getattr(config, section), "enabled") is True, section


def test_stage_gates_covers_exactly_the_ten_sections(config):
    gates = stage_gates(config)
    assert set(gates) == set(SECTIONS)
    assert all(gates.values())


def test_a_gate_turned_off_is_really_visible(config):
    """反证：闸不是恒定 True 的假读数，改一只就看得见"""
    original = config.benchmark.enabled
    try:
        config.benchmark.enabled = False
        assert stage_gates(config)["benchmark"] is False
        assert stage_gates(config)["context"] is True
    finally:
        config.benchmark.enabled = original


def test_the_census_no_longer_archives_them_as_unread():
    """census 零读者名册与统一读者同批清账"""
    from tests.unit.test_config_reader_census_l217 import _UNREAD, _UNREAD_KNOB

    for section in SECTIONS:
        key = section + ".enabled"
        assert key not in _UNREAD, key
        assert key not in _UNREAD_KNOB, key
