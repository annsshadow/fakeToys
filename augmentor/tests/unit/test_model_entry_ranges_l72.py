# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""A113（L72）：模型条目的四个数值键两侧同判 —— 采样三键不再裸奔

改前实测（Temp `l72q/probe_before_orig.json` 十五例 × 三个面）：`temperature: 999`、
`top_p: 2.5`、`max_output_tokens: -5` 以及它们的字符串 / 布尔 / null / NaN 形状，
**运行时构造成功、`validate-config` 零错、`load_config` 加载成功** —— 三个面全绿。
症状不是本地崩：五个后端只把这些值塞进请求体（`claude.py:78`、`ernie.py:120-122`、
`gemini.py:81-83`、`ollama.py:72-74`、`openai_model.py:70-72`），坏值换回一条服务端
400，再被 `classify_error` 读成「后端不可用」⇒ 用户看不出是自己配错。

同一份取证还抓到 **L71 自己留下的缝**：`models.m.request_timeout: 0` 运行时抛
（A74 已接），但静态面 `is_valid=True / 0 错误` —— 正是 A77 的镜像症状「校验器绿灯、
加载时抛」。本轮把它一起钉住：四个键在两个面上同判。

判据族的分工（每条口径决定都写在 `config.py` 的常数注释里）：
- `temperature` 闭区间 0-2、`top_p` 闭区间 0-1 ⇒ `require_ratio`（族里唯一带上界的成员）；
- `max_output_tokens` 下界 1 且必须是整数、**不设上界** ⇒ `require_count`；
- `request_timeout` 1-600 且 `None` 合法（= 不覆盖全局档）⇒ `require_seconds`。

`None` 的读法**按键分档**是本轮唯一需要解释的不对称：采样三键的 null 两侧都拒，
`request_timeout` 的 null 两侧都放行。所以本文件既有「同一坏值两侧同拒」，也有
「同一合法 null 两侧同收」。
"""

import dataclasses
import pathlib
from typing import get_args

import pytest
import yaml

from augmentor.config import (MAX_OUTPUT_TOKENS_MIN, ModelConfig,
                              REQUEST_TIMEOUT_RANGE, TEMPERATURE_RANGE,
                              TOP_P_RANGE, load_config)
from augmentor.config_validator import ConfigValidator
from augmentor.exceptions import DataValidationError

SHIPPED_YAML = pathlib.Path(__file__).resolve().parent.parent.parent / "config.yaml"

# (键, 坏值, 一句话原因)。每例都要在**两个面**上同时红。
BAD_VALUES = [
    ("temperature", 999, "上界之外"),
    ("temperature", -1, "下界之外"),
    ("temperature", "0.5", "字符串"),
    ("temperature", True, "布尔被算术读成 1"),
    ("temperature", None, "写了键没给值"),
    ("temperature", float("nan"), "NaN 过不了任何比较"),
    ("top_p", 2.5, "上界之外"),
    ("top_p", -0.1, "下界之外"),
    ("top_p", "high", "字符串"),
    ("top_p", None, "写了键没给值"),
    ("max_output_tokens", 0, "0 条输出没有合法读法"),
    ("max_output_tokens", -5, "负数"),
    ("max_output_tokens", 2048.5, "非整数"),
    ("max_output_tokens", "2048", "字符串"),
    ("max_output_tokens", None, "写了键没给值"),
    ("request_timeout", 0, "requests 对 timeout=0 当场抛"),
    ("request_timeout", 601, "上界 600 之外（客户端等得比网关久没有意义）"),
    ("request_timeout", "30", "字符串"),
]

# (键, 合法值, 一句话原因)。每例都要在**两个面**上同时绿。
GOOD_VALUES = [
    ("temperature", 0.0, "闭区间下界"),
    ("temperature", 2.0, "闭区间上界"),
    ("temperature", 1, "YAML 写整数：运行时 `require_ratio` 收 int，静态面同样收"),
    ("temperature", 0.99, "出厂模板那一档"),
    ("top_p", 0.0, "含 0 是刻意的松弛（各家对 0 的说法不一致，见 `TOP_P_RANGE`）"),
    ("top_p", 1.0, "闭区间上界"),
    ("max_output_tokens", 1, "下界"),
    ("max_output_tokens", 2048, "出厂模板那一档"),
    ("max_output_tokens", 10 ** 9, "刻意**不设上界**：编一个本地天花板就是造第二个权威"),
    ("request_timeout", None, "null 是一档合法语义 = 不覆盖全局档"),
    ("request_timeout", 1.0, "闭区间下界"),
    ("request_timeout", 600.0, "闭区间上界"),
]


def _raw(key, value):
    """一份只含一个模型条目的最小配置字典（`validate_config` 吃字典，不吃文件）"""
    entry = {"type": "openai"}
    entry[key] = value
    return {"models": {"default": "m", "m": entry}, "augmentation": {}}


def _static_errors(key, value):
    result = ConfigValidator().validate_config(_raw(key, value))
    return [e for e in result.errors if e.path == "models.m.%s" % key]


def _numeric_field_names():
    """从注解推导 `ModelConfig` 的数值字段，不写死清单（A113 的根就是「漏了一个」）"""
    out = []
    for f in dataclasses.fields(ModelConfig):
        args = get_args(f.type)
        numerics = [a for a in (args or (f.type,)) if a in (int, float)]
        if numerics:
            out.append(f.name)
    return out


class TestBothFacesReject:
    """同一坏值：运行时构造抛、静态面报红 —— 少一边就是 A77 的复发"""

    @pytest.mark.parametrize("key,value,why", BAD_VALUES)
    def test_runtime_rejects(self, key, value, why):
        with pytest.raises(DataValidationError, match=key):
            ModelConfig(type="openai", **{key: value})

    @pytest.mark.parametrize("key,value,why", BAD_VALUES)
    def test_static_face_reports_the_same_path(self, key, value, why):
        errors = _static_errors(key, value)
        assert errors, "%s=%r（%s）在 validate-config 上零反馈" % (key, value, why)
        assert errors[0].severity.value == "error"

    @pytest.mark.parametrize("key,value,why", BAD_VALUES)
    def test_the_runtime_face_does_not_depend_on_the_spec_table(self, key, value, why):
        """判据族在构造期真的动手：规格表若被删空，运行时那一侧仍然红。

        上面两条钉「两边都判」，这条钉「两边各自都成立」—— 于是「把 `__post_init__`
        掏空、只留规格表」那种写法也当场红。规格表在这里被清空再复原（同进程内，
        不跨 worker）。

        参数化到**每一个坏值**而不是只测 `temperature` 一档：注入实测（Temp
        `l72q/injection_out.txt` m4）显示只测一档时，摘掉 `top_p` 或
        `max_output_tokens` 的运行时判据不会被这条抓到。
        """
        saved = dict(ConfigValidator.MODEL_ENTRY_FIELDS)
        try:
            ConfigValidator.MODEL_ENTRY_FIELDS.clear()
            with pytest.raises(DataValidationError):
                ModelConfig(type="openai", **{key: value})
        finally:
            ConfigValidator.MODEL_ENTRY_FIELDS.update(saved)


class TestBothFacesAccept:
    """合法档不许被任何一面误判 —— 收紧判据的代价就记在这条轴上"""

    @pytest.mark.parametrize("key,value,why", GOOD_VALUES)
    def test_runtime_constructs(self, key, value, why):
        cfg = ModelConfig(type="openai", **{key: value})
        assert getattr(cfg, key) == value

    @pytest.mark.parametrize("key,value,why", GOOD_VALUES)
    def test_static_face_is_quiet(self, key, value, why):
        assert _static_errors(key, value) == []


def _spec_kind(spec):
    """条目规格的种类：`range`（有上下界）/ `choices`（封闭清单）/ `string`
    （形状判据：类型 + 非空）/ `other`

    种类从规格自己的维度推，不另立清单 —— 所以 L74 往表里加 `type` 时，本仓的
    对账要么把它认成 `choices`，要么当场红在 `test_every_spec_declares_its_kind`。
    L75 加 `model` 时正是后者先红了一次：那条 `{"type": str, "non_empty": True}`
    既无界也无清单，于是本函数多认一类 `string`（判据住在运行时的
    `require_string`，规格表只表达它的形状，不另立权威）。
    """
    if "choices" in spec:
        return "choices"
    if "min" in spec or "max" in spec:
        return "range"
    if spec.get("type") is str:
        return "string"
    return "other"


def _ranged_spec_names():
    return sorted(k for k, s in ConfigValidator.MODEL_ENTRY_FIELDS.items()
                  if _spec_kind(s) == "range")


class TestTheRangesAreOneCopyOnly:
    """规格表里的界必须**就是** `config.py` 那批常数，不是抄来的第二份数

    做法：拿规格表自己的 min/max 推出「越界一档」的探针值，要求两个面同时拒。
    于是「只改校验器那一半的界」（A77 的原始症状）当场红 —— 运行时的界没变，
    而探针值是按规格表算出来的。
    """

    def _probe_outside(self, spec):
        lo, hi = spec.get("min"), spec.get("max")
        step = 1 if spec["type"] is int else 0.5
        out = []
        if lo is not None:
            out.append(lo - step)
        if hi is not None:
            out.append(hi + step)
        return out

    def test_every_numeric_model_field_has_a_spec(self):
        """`ModelConfig` 的每个数值字段都必须有规格 —— 防 A113 的根复发

        改前正是「九个字段里只有 `request_timeout` 有判据」，而这条对账不存在，
        所以下一轮新加一根旋钮照样会漏。清单从字段注解推导。

        L74 起对账对象是**区间类规格**那一半：条目表里现在还有 `choices` 那一类
        （`type`，A115），它没有上下界，混进这条等式会把「加一类新规格」误报成
        「漏了一个数值字段」。所以本条与 `test_every_spec_declares_its_kind`
        合起来才等于改前那条等式：数值字段 ⇔ 区间规格，且每条规格都必须声明种类。
        """
        numeric = sorted(_numeric_field_names())
        ranged = _ranged_spec_names()
        assert numeric == ranged, \
            "数值字段清单与区间规格清单不吻合：%s vs %s" % (
                numeric, ranged)

    def test_every_spec_declares_its_kind(self):
        """规格表里不许有「既无界也无清单」的第三种条目（上面那条对账的另一半）"""
        other = [k for k, s in ConfigValidator.MODEL_ENTRY_FIELDS.items()
                 if _spec_kind(s) == "other"]
        assert not other, "未声明种类的规格：%s" % sorted(other)

    def test_no_ghost_model_spec(self):
        """规格表里不许躺着 `ModelConfig` 没有的键（A76 方向守护在模型条目那一侧的版本）"""
        names = {f.name for f in dataclasses.fields(ModelConfig)}
        ghost = set(ConfigValidator.MODEL_ENTRY_FIELDS) - names
        assert not ghost, "幽灵规格：%s" % sorted(ghost)

    @pytest.mark.parametrize("key", _ranged_spec_names())
    def test_one_step_outside_the_table_is_red_on_both_faces(self, key):
        """「界外一步」两侧同红 —— 只对区间类规格成立

        `choices` 那一类（`type`）没有「界外一步」可探，它的两侧同拒由
        `tests/unit/test_model_type_choices_l74.py` 负责。
        """
        spec = ConfigValidator.MODEL_ENTRY_FIELDS[key]
        assert spec.get("min") is not None, "%s 的规格没有下界" % key
        for probe in self._probe_outside(spec):
            with pytest.raises(DataValidationError, match=key):
                ModelConfig(type="openai", **{key: probe})
            assert _static_errors(key, probe), \
                "%s=%r 规格表拒了、运行时没拒" % (key, probe)

    def test_the_bounds_match_the_config_constants(self):
        """界与常数的对应关系本身也要钉住：`temperature` 用 `TEMPERATURE_RANGE` ……

        这条看着像同义反复（表就是从常数构造的），它守的是**接线方向**：哪天有人
        把某行的常数换成字面量，表与常数就不再同批，两侧同判立刻漂。
        """
        table = ConfigValidator.MODEL_ENTRY_FIELDS
        assert (table["temperature"]["min"],
                table["temperature"]["max"]) == TEMPERATURE_RANGE
        assert (table["top_p"]["min"], table["top_p"]["max"]) == TOP_P_RANGE
        assert table["max_output_tokens"]["min"] == MAX_OUTPUT_TOKENS_MIN
        assert "max" not in table["max_output_tokens"], \
            "给 `max_output_tokens` 编上界等于凭空造第二家权威"
        assert (table["request_timeout"]["min"],
                table["request_timeout"]["max"]) == REQUEST_TIMEOUT_RANGE

    def test_only_request_timeout_accepts_null(self):
        """`nullable` 那一维只给 `request_timeout`：采样三键的 null 两侧都拒"""
        table = ConfigValidator.MODEL_ENTRY_FIELDS
        assert [k for k, s in table.items() if s.get("nullable")] == ["request_timeout"]
        for key in ("temperature", "top_p", "max_output_tokens"):
            assert _static_errors(key, None), "%s 的 null 在静态面绿灯" % key


class TestShippedConfigAndDefaults:
    """出厂配置与「两处默认值」的对账（L71 的先例：模板漂离代码默认值要当场红）"""

    def test_shipped_yaml_passes_both_faces(self):
        config = load_config(str(SHIPPED_YAML))
        with open(SHIPPED_YAML, encoding="utf-8") as handle:
            raw = yaml.safe_load(handle)
        result = ConfigValidator().validate_config(raw)
        assert result.errors == [], "出厂模板被判红：%s" % result.errors
        assert config.models, "出厂模板里一个模型条目都没有"
        for entry in config.models.values():
            assert TEMPERATURE_RANGE[0] <= entry.temperature <= TEMPERATURE_RANGE[1]
            assert TOP_P_RANGE[0] <= entry.top_p <= TOP_P_RANGE[1]
            assert entry.max_output_tokens >= MAX_OUTPUT_TOKENS_MIN

    def test_loader_fallbacks_equal_the_dataclass_defaults(self, tmp_path):
        """「不写这个键」时加载器给的值必须等于 `ModelConfig` 的字段默认值

        本轮（L72）这条棘轮**当场抓到了一份既有漂移**：同一份只写 `type` 的最小条目里，
        `model` 拿到的是加载器字面量 `''`，而 `ModelConfig.model` 的默认值是
        `'default'`（`api_key` / `secret_key` / `base_url` 同族：那边 `None`、
        这边 `resolve_env('')` = `''`）。当时只钉了四个数值键，漂移本体记在 **A114**。

        L75 关掉 A114：`config.py` 的 `_model_entry` 只把 YAML 里**在场**的键交给
        构造函数，回落值从此由字段默认唯一决定 ⇒ 对账范围从「四个数值键」扩到
        「除 `type` 之外的全部字段」，且清单再次从 `dataclasses.fields` 推导，
        不写死。同轮改判的一句就是原来那句 `assert loaded.model == ''` ——
        它钉的是缺陷取证，A114 修完必须翻成 `== 'default'`，由下面的循环自动覆盖。
        """
        path = tmp_path / "minimal.yaml"
        path.write_text(
            "models:\n  default: m\n  m:\n    type: openai\naugmentation: {}\n",
            encoding="utf-8")
        loaded = load_config(str(path)).models["m"]
        plain = ModelConfig(type="openai")
        defaulted = [f.name for f in dataclasses.fields(ModelConfig)
                     if f.default is not dataclasses.MISSING]
        assert len(defaulted) == 8, "有字段没默认值？`type` 应该是唯一的一个"
        for key in defaulted:
            assert getattr(loaded, key) == getattr(plain, key), \
                "%s 的加载器回落与 dataclass 默认漂了" % key

    def test_optional_identity_keys_are_not_caught_in_the_dragnet(self):
        """`api_key` / `secret_key` / `base_url` 的「没给值」仍然是合法状态

        A113 明确不扩面到这三根（那是 A95 那条 `${ENV}` 线的地盘），所以这里钉的是
        「本轮没有误伤它们」：`_reject_null_fields` 的白名单只点名采样三键。
        """
        cfg = ModelConfig(type="ollama", api_key=None, base_url=None)
        assert cfg.api_key is None and cfg.base_url is None
        assert cfg.temperature == 0.99
