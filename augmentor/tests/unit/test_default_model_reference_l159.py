# -*- coding: utf-8 -*-
"""L159 / B229：悬空 `models.default` 在校验面从零反馈升为 warning。

`models.default` 是 `default_model` 的唯一来源；它指向不存在的条目时，
消费点 `get_model_config`（pipeline 构造必经）抛可行动的 ConfigError——
但只在「真要建默认后端」时：显式按名取模型的调用方不受影响，所以这一档
**不是必炸**，校验面判 ERROR 会拒绝「合法不用默认模型」的配置（v2 的
ERROR 档被 56 例既有测试面实证为过度收紧，v3 降 warning）。修法手法同
L158：文案提为 `_missing_model_message` 唯一产地（A77），消费点与校验面
共引，运行时判决零变化；校验面按「值不会生效」族（`_warn_unread_model_keys`）
先例报 warning。
"""
import io
import pathlib

from augmentor.config import (
    AppConfig,
    ModelConfig,
    _missing_model_message,
    get_model_config,
)
from augmentor.config_validator import validate_config
from augmentor.exceptions import ConfigError
import pytest

_CONFIG = pathlib.Path(__file__).resolve().parents[2] / "augmentor" / "config.py"
_VALIDATOR = pathlib.Path(__file__).resolve().parents[2] / "augmentor" / "config_validator.py"


def _cfg(default_value):
    return {
        "models": {
            "default": default_value,
            "m1": {"type": "openai", "api_key": "k"},
        }
    }


def test_dangling_default_now_warns_on_the_validation_face():
    """default 指向不存在的条目 / 空串：恰一条 warning、路径点名、文案同源。"""
    for bad in ("ghost", ""):
        result = validate_config(_cfg(bad))
        warns = [w for w in result.warnings if w.path == "models.default"]
        assert len(warns) == 1, f"{bad!r} 应恰一条 warning，实得 {[str(w) for w in result.warnings]}"
        assert warns[0].message == _missing_model_message(bad, ["m1"])
        assert [e.path for e in result.errors if e.path == "models.default"] == [], (
            "悬空 default 不是必炸（显式按名取模型不受影响），不得判 ERROR"
        )


def test_wellformed_default_gains_no_new_noise():
    """default 指向真实条目：无 warning 无 error。"""
    result = validate_config(_cfg("m1"))
    assert [str(e) for e in result.errors] == []
    assert [w.path for w in result.warnings if w.path == "models.default"] == []


def test_type_face_reports_alone_without_the_reference_overlay():
    """default 写成非 str：类型层报 ERROR，引用档不叠（非 str 不进 warning 分支）。"""
    result = validate_config({"models": {"default": 42}})
    assert len(result.errors) == 1 and "期望 str" in result.errors[0].message
    assert [w.path for w in result.warnings if w.path == "models.default"] == []


def test_absent_default_is_judged_by_the_required_face_not_the_reference_overlay():
    """default 键缺席：规格表必填档报「缺少必填字段」（既有），无引用档叠报。"""
    result = validate_config({"models": {"m1": {"type": "openai", "api_key": "k"}}})
    defaults = [e for e in result.errors if e.path == "models.default"]
    assert len(defaults) == 1 and "缺少必填字段" in defaults[0].message
    assert all("未配置" not in e.message for e in result.errors)


def test_the_consumer_verdict_is_unchanged_and_shares_the_producer():
    """消费点 get_model_config 判决零变化，且文案出自同一产地。"""
    config = AppConfig()
    config.models["m1"] = ModelConfig(type="openai")
    with pytest.raises(ConfigError) as ei:
        get_model_config(config, "ghost")
    assert str(ei.value) == _missing_model_message("ghost", config.models.keys())


def test_the_message_has_exactly_one_producer_and_the_validator_projects():
    """A77 机械守卫：文案只住 config.py 一处，校验面不抄、真引产地。"""
    src = io.open(_CONFIG, "r", encoding="utf-8", newline="").read()
    assert src.count("未配置。可用模型") == 1
    assert "def _missing_model_message" in src
    vsrc = io.open(_VALIDATOR, "r", encoding="utf-8", newline="").read()
    assert "未配置。可用模型" not in vsrc, "校验面抄了文案 = 第二份实现"
    assert "_missing_model_message" in vsrc
