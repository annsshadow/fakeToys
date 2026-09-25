# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""A115（L74）：`models.<名字>.type` 第一次有了封闭清单，三个面同判

改前取证（`Temp/l74q/faces_before.json`，五例 × 三面，对 HEAD `ed27cc4cc` 的沙箱跑）：
`type: openaii` / `ernie` / 空 / 缺键 四种写法，**构造面 ok、静态面 0 错、`load_config`
加载成功**，全部一路走到 `AugmentorPipeline` 的降级分支，症状只剩一条 WARNING
「模型后端初始化失败，增强功能将不可用」—— 用户看到的是「后端不可用」，真因是
自己把类型名写错。改后同一批取证（`faces_after.json`）四面全红，且红在**加载那一步**，
所以那个「模型没配好就降级」的 `except Exception` 对本键已不可达。

`ernie` 是本条缺陷存在的原因，两种混淆都要防：
- `ERNIEBackend` 是类名，它的 `type` 取值是 `baidu`；
- `models.default: ernie` 是「默认用哪个条目名」的指针（出厂模板里条目名确实叫
  `ernie`），不是 type。所以本文件既测「条目名叫 ernie 且 type 是 baidu ⇒ 合法」，
  也测「type 写 ernie ⇒ 两面同拒」。

清单只有一处定义（`config.MODEL_TYPES`），三个消费者共引：构造期判据、
`config_validator.MODEL_ENTRY_FIELDS["type"]["choices"]` 规格、工厂的错误文案。
「工厂表的键集 == 清单」由本文件钉住 —— 这是清单与分发表之间唯一的防线，
因为 `factory.py` 反向 import `config` 会成循环导入，两集只能在测试里对账。
"""

import pathlib

import pytest

from augmentor.config import MODEL_TYPES, ModelConfig, load_config
from augmentor.config_validator import ConfigValidator
from augmentor.exceptions import ConfigError, DataValidationError
from augmentor.models.base import ModelBackend
from augmentor.models.factory import MODEL_BACKENDS, create_model_backend

SHIPPED_YAML = pathlib.Path(__file__).resolve().parent.parent.parent / "config.yaml"

# 清单外取值（每例都要在构造面与静态面上同时红）。
BAD_TYPES = [
    ("", "空串：`load_config` 对「这条没写 type」的回落值，语义上就是没配"),
    ("ernie", "类名 / 出厂条目名，不是 type 取值"),
    ("openaii", "多打一个字母"),
    ("BAIDU", "大小写不是这家的宽容项"),
    ("baidu ", "尾随空格：YAML 里手抄最容易漏"),
    (None, "写了键没给值"),
    (0, "整数"),
    (True, "布尔"),
    (["baidu"], "不可哈希的入参：改前的 `==` 链一路判假，改后也不许炸成 TypeError"),
]


def _entry(value):
    """只含一个模型条目的最小配置字典（`validate_config` 吃字典，不吃文件）"""
    return {"models": {"default": "m", "m": {"type": value}}, "augmentation": {}}


def _static_errors(value):
    result = ConfigValidator().validate_config(_entry(value))
    return [e for e in result.errors if e.path == "models.m.type"]


def _yaml(text):
    """一份可被 `load_config` 吃下的最小 YAML"""
    return (
        "models:\n"
        "  default: m\n"
        "  m:\n"
        + text
        + "    model: x\n"
        "augmentation:\n"
        "  variants_per_seed: 3\n"
    )


def _backend_cfg(model_type):
    """够五个后端构造用的凭据（工厂在查表之前不会碰这些键）"""
    return ModelConfig(type=model_type, api_key="k", secret_key="s",
                       base_url="http://localhost:11434", model="m")


class TestClosedListBothFaces:
    """清单内两侧同收、清单外两侧同拒 —— 少一边就是 A77 的复发形状"""

    @pytest.mark.parametrize("model_type", MODEL_TYPES)
    def test_every_listed_type_is_accepted_by_both_faces(self, model_type):
        """合法取值从 `MODEL_TYPES` 推导，不写死第二份清单"""
        assert _backend_cfg(model_type).type == model_type
        assert not _static_errors(model_type), (
            "%r 在清单里却被 validate-config 判红：规格表与清单漂了" % model_type)

    @pytest.mark.parametrize("value,why", BAD_TYPES)
    def test_runtime_rejects_unlisted_type(self, value, why):
        with pytest.raises(DataValidationError, match="type 不支持"):
            ModelConfig(type=value)

    @pytest.mark.parametrize("value,why", BAD_TYPES[:-1])
    def test_static_face_reports_the_same_value(self, value, why):
        """静态面同步判红。最后那一例（列表）不测：它压根进不了 YAML"""
        errors = _static_errors(value)
        assert errors, "清单外的 %r（%s）在 validate-config 上零反馈" % (value, why)
        assert errors[0].severity.value == "error"

    @pytest.mark.parametrize("value,why", BAD_TYPES)
    def test_the_rejection_names_the_whole_list(self, value, why):
        """报错必须把合法取值一次说完：只说「不支持」等于把纠错成本还给用户"""
        with pytest.raises(DataValidationError) as info:
            ModelConfig(type=value)
        message = str(info.value)
        for model_type in MODEL_TYPES:
            assert model_type in message


class TestEntryNameIsNotType:
    """条目名与 type 取值是两回事：出厂模板就是「条目叫 ernie、type 是 baidu」"""

    def test_entry_named_ernie_with_baidu_type_is_legal(self, tmp_path):
        """出厂模板的形状：条目名 `ernie` + `type: baidu` 必须照样合法"""
        path = tmp_path / "c.yaml"
        path.write_text("models:\n"
                        "  default: ernie\n"
                        "  ernie:\n"
                        "    type: baidu\n"
                        "    api_key: k\n"
                        "    secret_key: s\n"
                        "    model: x\n"
                        "augmentation:\n"
                        "  variants_per_seed: 3\n",
                        encoding="utf-8", newline="\n")
        config = load_config(str(path))
        assert config.default_model == "ernie"
        assert config.models["ernie"].type == "baidu"

    def test_shipped_config_still_loads_and_is_all_listed(self):
        """出厂配置的代价必须是零：清单收紧不能把自己的模板判红"""
        result = ConfigValidator().validate_file(str(SHIPPED_YAML))
        assert result.is_valid, [e.message for e in result.errors]
        config = load_config(str(SHIPPED_YAML))
        assert {c.type for c in config.models.values()} <= set(MODEL_TYPES)


class TestDispatchTableMatchesList:
    """工厂那张表与清单必须是同一批名字 —— 两集相等只在测试里可对账"""

    def test_keys_are_exactly_the_closed_list(self):
        assert set(MODEL_BACKENDS) == set(MODEL_TYPES)
        assert len(MODEL_BACKENDS) == len(MODEL_TYPES)

    @pytest.mark.parametrize("model_type", MODEL_TYPES)
    def test_every_listed_type_builds_its_backend(self, model_type):
        backend = create_model_backend(_backend_cfg(model_type))
        assert isinstance(backend, MODEL_BACKENDS[model_type])
        assert isinstance(backend, ModelBackend)

    @pytest.mark.parametrize("model_type", MODEL_TYPES)
    def test_table_values_are_backends_not_strings(self, model_type):
        assert issubclass(MODEL_BACKENDS[model_type], ModelBackend)


class TestFactoryStillGuardsItsOwnLayer:
    """构造期拦不住那一层（`model_type=` 是工厂的入参）必须仍有判据

    `None` 不在坏值之列：它是本仓库文档化的「未提供」哨兵，语义就是回落到
    `config.type`。改前工厂用 `model_type or config.type`，于是**任何假值**都被当成
    「未提供」⇒ 显式传 `''` / `0` 的调用方不报错，而是静默拿到配置里那个类型的后端
    （A115 顺手修的那条，与下方 `request_timeout` 的既有口径同源）。
    """

    # `None` 是哨兵而不是坏值，单独测；其余假值要与构造面同拒。
    FACTORY_BAD_TYPES = [(v, w) for v, w in BAD_TYPES if v is not None]

    @pytest.mark.parametrize("value,why", FACTORY_BAD_TYPES)
    def test_factory_rejects_via_model_type_argument(self, value, why):
        with pytest.raises(ConfigError, match="不支持的模型类型"):
            create_model_backend(_backend_cfg("baidu"), model_type=value)

    @pytest.mark.parametrize("value,why", FACTORY_BAD_TYPES)
    def test_factory_message_lists_the_same_names(self, value, why):
        with pytest.raises(ConfigError) as info:
            create_model_backend(_backend_cfg("baidu"), model_type=value)
        assert ", ".join(MODEL_TYPES) in str(info.value)

    def test_none_is_the_documented_sentinel_not_a_bad_value(self):
        """传 `None` = 没传：应回落到 `config.type`，而不是被判成清单外"""
        backend = create_model_backend(_backend_cfg("ollama"), model_type=None)
        assert isinstance(backend, MODEL_BACKENDS["ollama"])

    def test_config_type_is_the_fallback_when_argument_omitted(self):
        assert isinstance(create_model_backend(_backend_cfg("ollama")),
                          MODEL_BACKENDS["ollama"])


class TestLoadFaceCannotDegradeSilently:
    """加载这一步就要出声，而不是让 pipeline 的降级分支把它读成「后端不可用」"""

    def test_bogus_type_raises_out_of_load_config(self, tmp_path):
        path = tmp_path / "c.yaml"
        path.write_text(_yaml("    type: openaii\n"), encoding="utf-8",
                        newline="\n")
        with pytest.raises(DataValidationError, match="type 不支持"):
            load_config(str(path))

    def test_bogus_type_raises_out_of_pipeline_instead_of_none(self, tmp_path):
        """改前这一例的症状是 `model_backend is None` + 一条 WARNING"""
        from augmentor.pipeline import AugmentorPipeline

        path = tmp_path / "c.yaml"
        path.write_text(_yaml("    type: ernie\n"), encoding="utf-8", newline="\n")
        with pytest.raises(DataValidationError):
            AugmentorPipeline(config_path=str(path))

    def test_missing_type_key_is_caught_by_the_load_face(self, tmp_path):
        """`type` 整键没写：回落值是空串，加载面拒（清单只有运行时那一侧看得到）"""
        path = tmp_path / "c.yaml"
        path.write_text(_yaml(""), encoding="utf-8", newline="\n")
        with pytest.raises(DataValidationError, match="type 不支持"):
            load_config(str(path))

    def test_missing_type_key_is_known_gap_on_the_static_face(self, tmp_path):
        """把残留缝记成断言，而不是让它悄悄躺着（账本 A119）

        `_check_required_fields` 只走 `KNOWN_FIELDS` 的固定路径，折进
        `MODEL_ENTRY_FIELDS` 的条目子键拿不到「必填」这一维 ⇒ 缺 `type` 在
        `validate-config` 上仍是 0 反馈。运行时那一侧已经拦住了，所以本条是
        「诊断面比产品面安静」而不是「三面全绿」。
        """
        result = ConfigValidator().validate_config(
            {"models": {"default": "m", "m": {"model": "x"}}, "augmentation": {}})
        assert [e for e in result.errors if e.path == "models.m.type"] == []
