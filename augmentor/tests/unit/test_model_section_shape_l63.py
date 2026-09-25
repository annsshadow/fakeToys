# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""L63 / A85：`models` 节的形状守卫（A101 的同构漏网）

A101（L57）给 `_load_section` 补了「节写成标量/空」的判据，但那只护住了走映射表
加载的 20 个常规节。`models` 走的是 `load_config` 里一条特判路径，绕开了那道守卫，
于是 `models: 随便一个标量`、或某个模型条目写成标量/列表/空 时，过去会当场崩在
`model_conf.get` 的 `AttributeError`（而校验面对同份文件报的是 `is_valid=False`）——
同族缺陷两侧不同判。本文件钉住：崩溃改成可行动的 `ConfigError`、口径与
`_load_section` 逐字一致、空值退化为全默认、正常配置一字不动。
"""

import pytest

from augmentor.config import load_config
from augmentor.exceptions import ConfigError

_MAP_PHRASE = "必须是「键: 值」的映射"


def _write(tmp_path, body):
    p = tmp_path / "config.yaml"
    p.write_text(body, encoding="utf-8")
    return str(p)


@pytest.mark.parametrize(
    "body, where",
    [
        ("models: hello\n", "models"),
        ("models:\n- a\n- b\n", "models"),
        ("models:\n  default: ernie\n  qwen: hello\n", "models.qwen"),
        ("models:\n  default: ernie\n  qwen: [1, 2]\n", "models.qwen"),
        ("models:\n  default: ernie\n  ernie: 42\n", "models.ernie"),
    ],
)
def test_malformed_models_shape_raises_actionable_configerror(tmp_path, body, where):
    """写成非映射（标量/列表/数）→ 可行动的 ConfigError，不再是 AttributeError 栈"""
    path = _write(tmp_path, body)
    with pytest.raises(ConfigError) as ei:
        load_config(path)
    msg = str(ei.value)
    assert where in msg, f"报错没点名是哪一处：{msg}"
    assert _MAP_PHRASE in msg, f"报错措辞与 A101 口径不一致：{msg}"


def test_models_none_degrades_to_no_entries(tmp_path):
    """`models:` 只写节名没给内容 = 无模型条目（同 A101「空即全默认」）"""
    config = load_config(_write(tmp_path, "models:\n"))
    assert config.models == {}


def test_model_entry_none_degrades_to_default_modelconfig(tmp_path):
    """某个模型条目写成 `qwen:`（None）= 该模型全默认，不崩、不静默跳过"""
    config = load_config(
        _write(tmp_path, "models:\n  default: ernie\n  qwen:\n")
    )
    assert "qwen" in config.models
    qwen = config.models["qwen"]
    assert qwen.type == ""
    assert qwen.temperature == 0.99 and qwen.top_p == 0.95
    assert qwen.max_output_tokens == 2048


def test_guard_phrasing_matches_load_section(tmp_path):
    """同族同判：models 的报错措辞与 `_load_section`（augmentation 那侧）逐字同式"""
    with pytest.raises(ConfigError) as ei_a:
        load_config(_write(tmp_path, "augmentation: abc\n"))
    with pytest.raises(ConfigError) as ei_m:
        load_config(_write(tmp_path, "models: abc\n"))
    a = str(ei_a.value)
    m = str(ei_m.value)
    # 两侧都应是「<节名> 必须是「键: 值」的映射，当前是 'abc'（str）」这个骨架
    assert _MAP_PHRASE in a and _MAP_PHRASE in m
    assert "当前是 'abc'（str）" in a and "当前是 'abc'（str）" in m


def test_wellformed_models_still_parse(tmp_path, monkeypatch):
    """正常路径一字不动：多模型 + 各字段 + 环境变量解析照常"""
    monkeypatch.setenv("OPENAI_KEY", "sk-live")
    config = load_config(
        _write(
            tmp_path,
            "models:\n"
            "  default: gpt\n"
            "  gpt:\n"
            "    type: openai\n"
            "    api_key: ${OPENAI_KEY}\n"
            "    model: gpt-4o\n"
            "    temperature: 0.3\n",
        )
    )
    assert config.default_model == "gpt"
    gpt = config.models["gpt"]
    assert gpt.type == "openai"
    assert gpt.api_key == "sk-live"
    assert gpt.model == "gpt-4o"
    assert gpt.temperature == 0.3
