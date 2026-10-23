# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""AU-Q1（账本 A152②）：save_config 必须保留配置文件里的注释

缺陷形：save_config 走 yaml.safe_dump 整份重写，注释一律不回写。出厂配置的注释里
住的是判据本身（默认值的定标理由、迁移写法），一次保存等于把说明书删了 —— 而保存
是常态路径（POST /api/config、CLI 改配置都会走它）。

判据（全部走产品同一条路 load_config / save_config，不复制第二份实现）：

1. 注释存活：app 之前的顶层注释、可建模节内部的注释、模型条目里的注释、
   AppConfig 不建模的顶层段里的注释，保存后都必须还在；
2. 占位符不回归：api_key 的 ENV_VAR 占位符照旧回写，且重新加载时仍从环境变量
   解析出真实值（占位符那一半是更早一轮修的，不能被这次改动带回去）；
3. 语义不变：保存后的文件加载结果与保存前深相等，未建模的顶层段原样保留；
4. 反空转臂：把 round-trip 支路摘掉（令 _dump_roundtrip_document 回 None）时
   注释必须消失。没有这一臂，「注释还在」可能只是因为断言写错了地方。

本文件注释里不写反引号路径引用：它自己也在注释普查的扫描面上（A184 口径）。
"""

import copy
import os
import sys
from pathlib import Path

import pytest
import yaml

REPO = Path(__file__).resolve().parents[2]
if str(REPO) not in sys.path:
    sys.path.insert(0, str(REPO))

from augmentor import config as config_module  # noqa: E402
from augmentor.config import load_config, save_config  # noqa: E402

#: 四个位置的判据注释（保存后必须原样还在）。用显式标记而不是数行数：
#: 排版允许随 round-trip 微调，注释文本不允许丢。
TOP_COMMENT = "# AUQ1-TOP 这一行住在 app 之前"
SECTION_COMMENT = "# AUQ1-SECTION 这一节里的判据理由"
MODEL_COMMENT = "# AUQ1-MODEL 温度取 0.7 的理由"
UNMODELED_COMMENT = "# AUQ1-UNMODELED 工具不认识的段"

ALL_MARKERS = (TOP_COMMENT, SECTION_COMMENT, MODEL_COMMENT, UNMODELED_COMMENT)

#: 最小合法配置 + 四类注释。刻意让文件里只有一节的一个键（augmentation 只写
#: variants_per_seed）：其余字段由 save_config 的 asdict 补出来，正好量到
#: 「新键追加」与「旧键就地更新」两条路。
CONFIG_WITH_COMMENTS = """\
app:
  name: auq1
  version: 1.0
{top}
models:
  default: ernie
  ernie:
    type: baidu
    model: ernie-4.0
    api_key: ${{AUQ1_TEST_KEY}}
    temperature: 0.7
{model}
augmentation:
{section}
  variants_per_seed: 5
user_notes:
{unmodeled}
  keeper: keep-me
"""


def _write_config(tmp_path):
    """把带注释的出厂形状配置写盘，返回路径"""
    path = tmp_path / "config.yaml"
    path.write_text(CONFIG_WITH_COMMENTS.format(
        top=TOP_COMMENT,
        model=MODEL_COMMENT,
        section=SECTION_COMMENT,
        unmodeled=UNMODELED_COMMENT,
    ), encoding="utf-8")
    return path


def test_comments_survive_a_save_round(tmp_path):
    """判据①：四类注释保存后都在，且改动真的落盘、重新加载拿到新值"""
    path = _write_config(tmp_path)
    config = load_config(str(path))
    config.augmentation.variants_per_seed = 9
    save_config(config, str(path))

    text = path.read_text(encoding="utf-8")
    for marker in ALL_MARKERS:
        assert marker in text, f"注释丢了: {marker}"
    assert load_config(str(path)).augmentation.variants_per_seed == 9


def test_placeholders_are_still_written_back(tmp_path, monkeypatch):
    """判据②：ENV_VAR 占位符照旧回写，重新加载时仍从环境变量解析"""
    monkeypatch.setenv("AUQ1_TEST_KEY", "sk-auq1-secret")
    path = _write_config(tmp_path)
    config = load_config(str(path))
    assert config.models["ernie"].api_key == "sk-auq1-secret"

    save_config(config, str(path))

    text = path.read_text(encoding="utf-8")
    assert "${AUQ1_TEST_KEY}" in text, "占位符被真实值顶替落盘"
    assert load_config(str(path)).models["ernie"].api_key == "sk-auq1-secret"


def test_semantics_unchanged_and_unmodeled_section_kept(tmp_path):
    """判据③：不改任何值时往返深相等，未建模顶层段连值带注释一起保留"""
    path = _write_config(tmp_path)
    before = load_config(str(path))
    save_config(copy.deepcopy(before), str(path))

    assert load_config(str(path)) == before
    doc = yaml.safe_load(path.read_text(encoding="utf-8"))
    assert doc["user_notes"] == {"keeper": "keep-me"}


def test_two_consecutive_saves_keep_the_comments(tmp_path):
    """连续保存两次：注释不因第二次合并而被抹掉（round-trip 是幂等的）"""
    path = _write_config(tmp_path)
    config = load_config(str(path))
    save_config(config, str(path))
    save_config(load_config(str(path)), str(path))

    text = path.read_text(encoding="utf-8")
    for marker in ALL_MARKERS:
        assert marker in text, f"第二次保存后注释丢了: {marker}"


def test_without_the_roundtrip_branch_comments_are_lost(tmp_path, monkeypatch):
    """反空转臂：摘掉 round-trip 支路 ⇒ 注释必须消失，且降级路仍写出可加载的配置"""
    monkeypatch.setattr(config_module, "_dump_roundtrip_document",
                        lambda data, config_path: None)
    path = _write_config(tmp_path)
    config = load_config(str(path))
    save_config(config, str(path))

    text = path.read_text(encoding="utf-8")
    assert TOP_COMMENT not in text
    assert SECTION_COMMENT not in text
    assert MODEL_COMMENT not in text
    # 降级不等于写坏：文件仍加载得回来，语义不变
    assert load_config(str(path)) == config


def test_roundtrip_helper_degrades_when_ruamel_is_missing(tmp_path, monkeypatch):
    """缺依赖时 helper 回 None 并记 warning（保存不中断，代价是丢注释）"""
    import builtins

    real_import = builtins.__import__

    def no_ruamel(name, *args, **kwargs):
        if name == "ruamel.yaml":
            raise ImportError("simulated: ruamel not installed")
        return real_import(name, *args, **kwargs)

    monkeypatch.setattr(builtins, "__import__", no_ruamel)
    path = _write_config(tmp_path)
    config = load_config(str(path))

    assert config_module._dump_roundtrip_document({}, str(path)) is None
    save_config(config, str(path))
    assert load_config(str(path)) == config
