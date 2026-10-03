# -*- coding: utf-8 -*-
"""L158 / B228：`models.<名字>` 条目写成标量 / 列表 / 数时，校验面不再静默。

A85 / A101 当年补的是运行时两侧（`_load_section` 与 models 条目的
`_as_mapping`），L158 实测校验面（`validate_config` → CLI validate-config）
对这一档**完全静默**：同一份 `models: {m1: [1]}`，校验工具报 is_valid=True，
应用启动时 `load_config` 才抛 ConfigError —— 判决分叉。修法不是在校验面抄
一份文案，而是把「必须是映射」判据提升为 `_require_mapping` 唯一产地
（A77：一条界只住一个地方），运行时两侧与校验面投影三处共引。
"""
import io
import pathlib

import pytest

from augmentor.config import load_config
from augmentor.config_validator import validate_config
from augmentor.exceptions import ConfigError

_MAP_PHRASE = "必须是「键: 值」的映射"

_CONFIG = pathlib.Path(__file__).resolve().parents[2] / "augmentor" / "config.py"
_VALIDATOR = pathlib.Path(__file__).resolve().parents[2] / "augmentor" / "config_validator.py"


def _entry_config(bad_value):
    return {"models": {"default": "m1", "m1": bad_value}}


def test_entry_as_non_mapping_is_now_red_on_the_validation_face():
    """条目写成标量 / 列表 / 数：validate_config 恰报一条、路径点名、文案骨架对。

    改前这一档零报错（`not isinstance(entry, dict): continue` 静默跳过）——
    四档现在各判各的，且判决与运行时同源。
    """
    for bad in ([1, 2], "hello", None, 42):
        result = validate_config(_entry_config(bad))
        errors = [e for e in result.errors if e.path == "models.m1"]
        assert len(errors) == 1, f"{bad!r} 应恰报一条，实得 {[str(e) for e in result.errors]}"
        assert _MAP_PHRASE in errors[0].message
        assert repr(bad) in errors[0].message, "报错应回显原值"
        assert type(bad).__name__ in errors[0].message, "报错应点名类型"


def test_the_validation_face_message_is_the_runtime_message_verbatim():
    """同一档，校验面的 message 与 load_config 抛的 ConfigError 文案逐字相同。

    这是共引的行为面证明：两侧都出自 `_require_mapping`，不是两份实现。
    """
    import tempfile

    with tempfile.NamedTemporaryFile(
        "w", suffix=".yaml", delete=False, encoding="utf-8"
    ) as f:
        f.write("models:\n  default: m1\n  m1: [1, 2]\n")
        path = f.name
    with pytest.raises(ConfigError) as ei:
        load_config(path)
    result = validate_config(_entry_config([1, 2]))
    [err] = [e for e in result.errors if e.path == "models.m1"]
    assert err.message == str(ei.value), (
        "校验面与运行时的文案不是同一产地：\n  校验面: %r\n  运行时: %r"
        % (err.message, str(ei.value))
    )


def test_default_key_is_still_exempt_from_the_entry_channel():
    """`default` 键是「默认模型名」（str 语义），不进条目通道。

    `default: 42` 只由 KNOWN_FIELDS 的 `models.default` 规格报「期望 str」，
    不得被条目通道叠报一份「映射」错。
    """
    result = validate_config({"models": {"default": 42}})
    assert [str(e) for e in result.errors] == [
        "ValidationError(path='models.default', message='类型错误: 期望 str, 实际 int', "
        "severity=<Severity.ERROR: 'error'>, line=None)"
    ] or all("映射" not in e.message for e in result.errors), [
        str(e) for e in result.errors
    ]


def test_wellformed_entries_gain_no_new_noise():
    """合法条目与既有合法配置不因新分支多出任何错误。"""
    result = validate_config(
        {"models": {"default": "m1", "m1": {"type": "openai", "api_key": "k"}}}
    )
    assert [str(e) for e in result.errors] == []


def test_the_map_phrase_has_exactly_one_producer():
    """A77 机械守卫：「必须是映射」文案在 config.py 只住 `_require_mapping` 一处。

    改前它以两份逐字相同的 raise 分住在 `_load_section` 与 `_as_mapping`——
    那正是 A85 族每加一个消费面就要抄一遍的根因。
    """
    src = io.open(_CONFIG, "r", encoding="utf-8", newline="").read()
    assert src.count(_MAP_PHRASE) == 1, src.count(_MAP_PHRASE)
    assert "def _require_mapping" in src


def test_the_validator_projects_the_producer_instead_of_copying():
    """校验面不抄文案、真调 `_require_mapping`（A77 投影，L157 回放纪律的同族）。"""
    src = io.open(_VALIDATOR, "r", encoding="utf-8", newline="").read()
    assert _MAP_PHRASE not in src, "校验面抄了文案 = 第二份实现"
    assert "_require_mapping" in src, "校验面没有引产地"
