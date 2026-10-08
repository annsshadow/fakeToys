"""A140 余 11 节 29 键的四面判据（L154 / B224）

改前现量（A140 行的现量房 Temp/l82q/ungated_census.json，meta.timestamp 2026-09-26）：
全仓 20 个配置节里**余 11 节**运行时零判据（一个 `__post_init__` 都没有）、
静态规格表里**一条规格行都没有**（`ungated_keys_with_static_spec = 0`）、且 11 节
都不可经 `POST /api/config` 写入 ⇒ 29 键的坏值在 YAML 加载、SDK 直构、
`validate_config` 三面**全部静默放行**，症状要拖到各节少数产品消费方
（`context` / `versioning` / `benchmark` 三节有读者，其余 8 节无）出声那天。

本轮（L154）把 A140 整片一次接齐（A140 原框「可按节分轮，单节约等于 S」，
11 节同式同族，一批做掉省得账本跨轮背「A140 余 N 节」）：

- 运行时面：11 个节各加 `__post_init__`（`_reject_null_fields` + `require_bool` /
  `require_count(minimum=1)` / `require_string` / `require_string_list`，全引
  `validation` 族既有成员，不新造判据；四个 int 键的 1 是调用点字面量，无既有
  常数可共引）。
- 静态面：`KNOWN_FIELDS` 补 11 节 dict 行 + 29 键规格行，形状与运行时同档。
- 加载面：`load_config` 走 `__post_init__` 自动吃到，本文件用最小 YAML 钉住。
- 消费面：`context` / `versioning` / `benchmark` 三节读者与合法值集同判；
  其余 8 节无读者（属性面普查），合法默认值构造不动即护栏。

既有 l82 守卫（test_config_gates_l82.py）只管它自己那四节，本文件是它的 A140 续篇：
坏值 × 四面参数化 + 合法值反向护栏 + 「删规格行运行时仍红」的独立性 +
A140 普查名单翻转。
"""
import dataclasses
import json

import pytest
import yaml

from augmentor.config import (ActiveLearningConfig, AppConfig, BenchmarkConfig,
                              ContextConfig, EvaluationConfig, ExpanderConfig,
                              FrameworkConfig, MultilingualConfig, SamplerConfig,
                              TrackerConfig, VisualizationConfig, VersioningConfig)
from augmentor.config_validator import ConfigValidator, validate_config
from augmentor.validation import DataValidationError, require_count

# 11 节名单由改前现量（A140 行）抄入，与 `AppConfig` 的 20 节字段集互补
# （余 9 节 = 7 个可写节 + `models`，前人已接）。翻转断言见
# `TestTheA140CensusIsInverted`。
SECTIONS = {
    "context": ContextConfig,
    "versioning": VersioningConfig,
    "sampler": SamplerConfig,
    "expander": ExpanderConfig,
    "tracker": TrackerConfig,
    "visualization": VisualizationConfig,
    "multilingual": MultilingualConfig,
    "evaluation": EvaluationConfig,
    "benchmark": BenchmarkConfig,
    "active_learning": ActiveLearningConfig,
    "frameworks": FrameworkConfig,
}

BOOL_KEYS = ["%s.enabled" % s for s in SECTIONS] + ["versioning.auto_snapshot"]
INT_KEYS = ["context.num_turns", "multilingual.translate_batch_size",
            "active_learning.batch_size", "active_learning.max_iterations"]
STR_KEYS = ["versioning.storage_dir", "multilingual.default_target_lang",
            "evaluation.reference_field", "benchmark.baseline_file",
            "active_learning.strategy"]
LIST_KEYS = ["sampler.dimensions", "expander.strategies", "tracker.metrics",
             "visualization.types", "multilingual.supported_langs",
             "evaluation.metrics", "benchmark.metrics", "frameworks.frameworks"]
ALL_KEYS = BOOL_KEYS + INT_KEYS + STR_KEYS + LIST_KEYS


def _section_of(dotted):
    return dotted.split(".", 1)[0]


def _key_of(dotted):
    return dotted.split(".", 1)[1]


def _construct(dotted, value):
    """按点分路径构造所在节，其余字段全给出厂默认值（各节默认全部合法）"""
    cls = SECTIONS[_section_of(dotted)]
    kwargs = {f.name: getattr(cls(), f.name) for f in dataclasses.fields(cls)}
    kwargs[_key_of(dotted)] = value
    return cls(**kwargs)


class TestTheA140CensusIsInverted:
    """改前普查：11 节无 `__post_init__`、29 键无规格行；这里全翻转

    两份改前读数都钉死在 docstring 的现量房里（A140 行引
    Temp/l82q/ungated_census.json），本条是「普查名单一个不漏地接上」的
    机械证据 —— 任何一节被拆掉 `__post_init__`、任何一行的规格被删，都会红。
    """

    def test_all_eleven_sections_have_runtime_gates(self):
        ungated = sorted(s for s, cls in SECTIONS.items()
                         if not hasattr(cls, "__post_init__"))
        assert ungated == [], "A140 名单有节被拆掉运行时判据：%s" % ungated

    def test_all_29_keys_have_static_spec_rows(self):
        missing = [k for k in ALL_KEYS if k not in ConfigValidator.KNOWN_FIELDS]
        assert missing == [], "规格表缺行：%s" % missing

    def test_the_key_census_is_still_29(self):
        """逐节键数现量（A140 行同数）：4+4+3×3+2×5 = 29，防「删键凑数」

        改前 `ungated_keys` 现量 29；这里按 `dataclasses.fields` 推导而不是
        手抄，删掉任何一节的字段本条就红。
        """
        counts = {s: len(dataclasses.fields(cls)) for s, cls in SECTIONS.items()}
        assert counts == {"context": 2, "versioning": 3, "sampler": 2,
                          "expander": 2, "tracker": 2, "visualization": 2,
                          "multilingual": 4, "evaluation": 3, "benchmark": 3,
                          "active_learning": 4, "frameworks": 2}, counts
        assert sum(counts.values()) == 29

    def test_every_spec_row_shapes_the_runtime_type(self):
        """规格表与运行时判据**同档**：int 行只声明类型（L157 / B227 起判决维
        撤出规格表，下界 1 由回放 + 运行时调用点字面量承担）、str 行非空、
        list 行逐项、bool 行就是 bool"""
        table = ConfigValidator.KNOWN_FIELDS
        for dotted in BOOL_KEYS:
            assert table[dotted] == {"type": bool}, dotted
        for dotted in INT_KEYS:
            spec = table[dotted]
            assert spec["type"] is int and "min" not in spec, dotted
            # 与运行时调用点字面量同档（`require_count(..., minimum=1)`）：
            # 同一值在两侧各自判负/判正；越界由回放报出。
            with pytest.raises(DataValidationError):
                require_count(dotted, 0, minimum=1)
            assert require_count(dotted, 1, minimum=1) == 1
            section, key = dotted.split(".", 1)
            errs = [e for e in validate_config(
                {"app": {"name": "t"}, "models": {"default": "ernie"},
                 section: {key: 0}}).errors if e.path == dotted]
            assert errs and "不小于" in errs[0].message,                 "%s 越界一档在静态面零反馈（回放没出声？）" % dotted
        for dotted in STR_KEYS:
            spec = table[dotted]
            assert spec["type"] is str and spec.get("non_empty") is True, dotted
        for dotted in LIST_KEYS:
            spec = table[dotted]
            assert spec["type"] is list and spec.get("items") is str, dotted
        # 节级 dict 行也在场（`context: 42` 标量档改前同样零反馈）
        for section in SECTIONS:
            assert table[section] == {"type": dict}, section


class TestBoolKeysRefuseEveryReadingButBool:
    """12 个布尔开关：`'no'` / `'true'` / `0` / `1` / null 全拒，`True` / `False` 放行

    `require_bool` docstring 的两面实测（字典面 `enabled: 'no'` 按真值恒真、
    `enabled: 0` 碰巧假）就是这一族存在的全部理由。
    """

    @pytest.mark.parametrize("dotted", BOOL_KEYS)
    @pytest.mark.parametrize("value", ["no", "true", "0", 0, 1, None])
    def test_bad_readings_refused(self, dotted, value):
        with pytest.raises(DataValidationError):
            _construct(dotted, value)

    @pytest.mark.parametrize("dotted", BOOL_KEYS)
    @pytest.mark.parametrize("value", [True, False])
    def test_legal_bools_land(self, dotted, value):
        assert _construct(dotted, value).__dict__[_key_of(dotted)] is value


class TestIntKeysRefuseFalsyZeroAndGarbage:
    """4 个计数键：`0` / 负数 / 字符串 / bool 全拒，`1` 起放行（假零家族，L144 同式）"""

    @pytest.mark.parametrize("dotted", INT_KEYS)
    @pytest.mark.parametrize("value", [0, -1, "3", True, None])
    def test_bad_values_refused(self, dotted, value):
        with pytest.raises(DataValidationError):
            _construct(dotted, value)

    @pytest.mark.parametrize("dotted", INT_KEYS)
    @pytest.mark.parametrize("value", [1, 5])
    def test_legal_counts_land(self, dotted, value):
        built = _construct(dotted, value)
        assert getattr(built, _key_of(dotted)) == value


class TestStringKeysRefuseEmptyBlankAndNonStr:
    """5 个字符串键：空串 / 纯空白 / null / 非字符串全拒，合法串放行"""

    @pytest.mark.parametrize("dotted", STR_KEYS)
    @pytest.mark.parametrize("value", ["", "   ", None, 5, True])
    def test_bad_values_refused(self, dotted, value):
        with pytest.raises(DataValidationError):
            _construct(dotted, value)

    @pytest.mark.parametrize("dotted,value", [
        ("versioning.storage_dir", "data/versions"),
        ("multilingual.default_target_lang", "en"),
        ("evaluation.reference_field", "output"),
        ("benchmark.baseline_file", "data/benchmark_baseline.json"),
        ("active_learning.strategy", "uncertainty"),
    ])
    def test_legal_strings_land(self, dotted, value):
        assert _construct(dotted, value).__dict__[_key_of(dotted)] == value


class TestListKeysRefuseScalarBlankAndNonStr:
    """8 个清单键：标量 / 混型 / 空项 / 纯空白项 / null 全拒，默认清单放行"""

    @pytest.mark.parametrize("dotted", LIST_KEYS)
    @pytest.mark.parametrize("value", ["scalar", [None], [""], ["   "], [1], None])
    def test_bad_values_refused(self, dotted, value):
        with pytest.raises(DataValidationError):
            _construct(dotted, value)

    @pytest.mark.parametrize("dotted", LIST_KEYS)
    def test_default_lists_still_construct(self, dotted):
        """出厂默认值是本节自己的合法形状 —— 判据不许把默认判红（L151 同护栏）"""
        built = _construct(dotted, _construct(dotted, None).__class__()) \
            if False else SECTIONS[_section_of(dotted)]()
        assert isinstance(getattr(built, _key_of(dotted)), list)


class TestLegalConfigRoundTripIsUnchanged:
    """全默认 `AppConfig()` 构造不动（20 节全合法），坏值才判负"""

    def test_default_app_config_constructs(self):
        config = AppConfig()
        assert config.context.num_turns == 3
        assert config.benchmark.metrics[0] == "pass_rate"
        assert config.frameworks.frameworks == ["langchain", "llamaindex"]


class TestTheStaticFaceAgrees:
    """静态面（`validate_config`）对同一批坏值报同一档的错

    最小合法配置沿 `test_config_validator._errors_at` 的先例（`app` + 一个
    `models` 条目），坏值只注入被测那一格。
    """

    @staticmethod
    def _errors_at(dotted, value):
        base = {"app": {"name": "t"}, "models": {"default": "ernie"}}
        section, key = _section_of(dotted), _key_of(dotted)
        base.setdefault(section, {})[key] = value
        return [(e.path, e.message) for e in validate_config(base).errors]

    @pytest.mark.parametrize("dotted,value", [
        ("context.num_turns", 0),
        ("multilingual.translate_batch_size", -1),
        ("versioning.storage_dir", ""),
        ("benchmark.baseline_file", "   "),
        ("sampler.dimensions", "topic"),
        ("tracker.metrics", ["   "]),
        ("context.enabled", "no"),
        ("versioning.auto_snapshot", 0),
        ("expander.strategies", None),
    ])
    def test_bad_value_flags_the_exact_path(self, dotted, value):
        hits = [(p, m) for p, m in self._errors_at(dotted, value)
                if p == dotted or p.startswith(dotted + "[")]
        assert hits, "%s = %r 静态面没出声" % (dotted, value)

    @pytest.mark.parametrize("dotted", ALL_KEYS)
    def test_legal_value_flags_nothing_at_that_path(self, dotted):
        """反向档：合法值（各键取出厂默认或一条手写合法值）不报该路径的错"""
        if dotted in BOOL_KEYS:
            value = True
        elif dotted in INT_KEYS:
            value = 1
        elif dotted in STR_KEYS:
            value = "ok"
        else:
            value = ["ok"]
        hits = [p for p, m in self._errors_at(dotted, value)
                if p == dotted or p.startswith(dotted + "[")]
        assert hits == [], "%s 合法值被静态面误判：%s" % (dotted, hits)


class TestTheGatesDoNotDependOnTheSpecTable:
    """独立性（l82 同名用例的 A140 续篇）：把该键所在节的规格行全删掉，运行时仍红

    「只留规格表、掏空 `__post_init__`」或「只留运行时、规格行被删」两种
    偷懒都当场红 —— 四面是四个独立产地，不是四份抄写。
    """

    @pytest.mark.parametrize("dotted,value", [
        ("context.num_turns", 0),
        ("context.enabled", "no"),
        ("versioning.storage_dir", ""),
        ("versioning.auto_snapshot", 0),
        ("sampler.dimensions", "topic"),
        ("expander.strategies", [None]),
        ("tracker.metrics", [1]),
        ("visualization.types", "x"),
        ("multilingual.default_target_lang", ""),
        ("multilingual.supported_langs", "zh"),
        ("multilingual.translate_batch_size", 0),
        ("evaluation.metrics", [1]),
        ("evaluation.reference_field", "   "),
        ("benchmark.baseline_file", ""),
        ("benchmark.metrics", "pass_rate"),
        ("active_learning.strategy", None),
        ("active_learning.batch_size", 0),
        ("active_learning.max_iterations", -1),
        ("frameworks.frameworks", "langchain"),
    ])
    def test_runtime_still_raises_with_the_spec_rows_deleted(self, dotted, value):
        """删的是**被测键所在节**的全部规格行（节级 dict 行 + 该节各键行）"""
        section = _section_of(dotted)
        table = ConfigValidator.KNOWN_FIELDS
        saved = {k: v for k, v in table.items()
                 if k == section or k.startswith(section + ".")}
        try:
            for name in list(table):
                if name == section or name.startswith(section + "."):
                    del table[name]
            with pytest.raises(DataValidationError):
                _construct(dotted, value)
        finally:
            table.update(saved)


class TestTheLoadConfigFaceAgrees:
    """加载面：最小 YAML 里写坏值，`load_config` 构造即拒（L151 同面同口径）"""

    @pytest.mark.parametrize("section,key,value", [
        ("context", "num_turns", 0),
        ("context", "enabled", "no"),
        ("versioning", "storage_dir", ""),
        ("sampler", "dimensions", "topic"),
        ("multilingual", "translate_batch_size", None),
        ("benchmark", "metrics", ["   "]),
        ("active_learning", "max_iterations", 0),
    ])
    def test_bad_yaml_refused_at_load(self, tmp_path, section, key, value):
        from augmentor import load_config

        path = tmp_path / ("a140_%s.yaml" % section)
        path.write_text(yaml.safe_dump({section: {key: value}}), encoding="utf-8")
        with pytest.raises(DataValidationError):
            load_config(str(path))

    def test_legal_yaml_still_loads(self, tmp_path):
        from augmentor import load_config

        path = tmp_path / "a140_ok.yaml"
        path.write_text(yaml.safe_dump(
            {"context": {"num_turns": 5},
             "benchmark": {"metrics": ["pass_rate"], "baseline_file": "b.json"}}),
            encoding="utf-8")
        config = load_config(str(path))
        assert config.context.num_turns == 5
        assert config.benchmark.metrics == ["pass_rate"]


class TestTheConsumerFaceAgreesOnTheLegalSet:
    """消费面：三节有读者的键，合法值集两侧同判（A140 行「消费方期待清单」那半）

    `benchmark.metrics` 的消费方（`QualityBenchmark`）期待字符串列表并逐项对
    `SUPPORTED_METRICS` 判成员 —— 本轮判的是**形状**不是成员资格（与
    `require_string_list` 同语义，不焊死业务名单），所以合法默认清单两侧同过；
    `context.num_turns` 的读者 `context.py` 的 `range(self.num_turns - 1)` 对
    正整数各档行为不变；`versioning.storage_dir` 的读者 `VersionManager` 拿
    `Path(storage_dir)`，非空串即正常根目录。
    """

    def test_benchmark_metrics_flow_into_the_consumer(self):
        from augmentor.benchmark import QualityBenchmark

        bench = QualityBenchmark(metrics=BenchmarkConfig().metrics)
        assert bench.metrics[0] == "pass_rate"

    def test_context_num_turns_flow_into_the_consumer(self):
        from augmentor.context import ContextAugmentor

        # `model_backend` 是形参第一位（`None` 档合法，`pipeline` 就只在建后端后
        # 才构造它）；这里只钉 `num_turns` 这一档在两侧同答。
        aug = ContextAugmentor(model_backend=None,
                               num_turns=ContextConfig().num_turns)
        assert aug.num_turns == 3

    def test_versioning_storage_dir_is_a_real_dir_name(self):
        from pathlib import Path

        storage_dir = VersioningConfig().storage_dir
        assert storage_dir and Path(storage_dir) not in (Path("."),)
        assert bool(VersioningConfig().auto_snapshot) is True
