# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""纯空白字符串的四面判据测试（L83 / A142）

A142 的原文是「`require_string` 第二刀是 `if not value:`，不判纯空白」，而它撞出来的
范围比那句话大：**「拒空串、放行纯空白」是一族**，`require_string_list` 的元素那一刀
同形，静态校验器的 `non_empty` 与 `items` 两格也同形。L82 的现量（`Temp/l83q/`，
`blank_census_before.json`）是 33 个字符串键里 **10 个洞、0 个「只拒空白」、6 个安全、
17 个两档全放**。本文件锁四件事：

1. **洞全关**：那十键的纯空白写法在 SDK 直构 / `load_config` / `validate_config`
   三面上同时判负（写入面那一路在 `tests/integration/test_config_write_path_l73.py`
   的 `TestApiRejectsBlankStrings`，四面同判才叫收口）；
2. **不越界**：A140 那片（两档全放的 16 键）本轮一格没动 —— 这条与上一条同等重要，
   没有它，「顺手多判几键」会被读成改进；
3. **空串与空白是两种判决**：`logging.file: ''` 是「不落文件」这一档设计，
   `logging.file: '   '` 是一个名叫空格的文件，本轮只拒后者（四分类里新立的第三格）；
4. **判据不裁剪值**：含空格的合法值照原样通过，一个字符也不动。

谓词只有一个产地（`validation.is_blank_string`），静态面共引同一个函数对象，
由 `TestThePredicateIsOneCopyOnly` 用 `is` 钉住（承 A77）。
"""

import dataclasses

import pytest

from augmentor import config as config_module
from augmentor.config import (AppConfig, LoggingConfig, MODEL_TYPES, ModelConfig,
                              WebConfig, load_config)
from augmentor.config_validator import ConfigValidator, validate_config
from augmentor.config_validator import is_blank_string as validator_predicate
from augmentor.exceptions import DataValidationError
from augmentor.validation import (is_blank_string, require_choice, require_string,
                                  require_string_list)

#: 本轮从「拒 `''`、放行纯空白」关进「两档都拒」的十键（L82 现量的洞集合，逐条来自
#: `Temp/l83q/blank_census_before.json` 的 `holes`）。写死名字而不是每次推导，是为了让
#: 「某一刀的删除」在这里当场红 —— 推导只能证明现在的分桶自洽，证明不了这十格是修来的。
HOLES_CLOSED_IN_L83 = [
    "models.<名>.model",
    "multimodal.audio_extensions",
    "multimodal.image_extensions",
    "vector.collection",
    "vector.storage_dir",
    "web.cors_origins",
    "web.data_roots",
    "web.host",
    "web.rate_limit_exempt_paths",
    "web.static_dir",
]

#: `logging.file` 不在上面那十键里：它的空串是**合法值**，本轮把它从「两档全放」挪进
#: 「只拒空白」这一格。
BLANK_ONLY_KEYS = ["logging.file"]

#: 两档都照旧放行的键（A140 那一片的字符串子集）。**精确相等**棘轮：本轮一格没动，
#: 以后谁动了 A140 就得同时改这里并在账本留一句（承 L76 / L77 的只降不升口径）。
STILL_UNGATED = [
    "active_learning.strategy",
    "benchmark.baseline_file",
    "benchmark.metrics",
    "evaluation.metrics",
    "evaluation.reference_field",
    "expander.strategies",
    "frameworks.frameworks",
    "models.<名>.api_key",
    "models.<名>.base_url",
    "multilingual.default_target_lang",
    "multilingual.supported_langs",
    "sampler.dimensions",
    "tracker.metrics",
    "versioning.storage_dir",
    "visualization.types",
]

#: 已有的安全键（`''` 与空白都拒）：四条清单键靠 membership、`logging.format` 靠
#: require_string + renderable、`export.formats` 靠逐项清单。
ALREADY_SAFE = [
    "export.default_format",
    "export.formats",
    "quality.weights",  # L153 / B223 起：require_ratio_list 两面全拒（null 与坏形状都挡）
    "logging.format",
    "logging.level",
    "rag.default_format",
    "vector.backend",
]

BLANKS = [" ", "   ", "\t", "\n", "\r\n", "\u00a0", " \t\n "]

#: 量不到的键（默认值本身构造就抛 ⇒ `raised` 是假安全）。L83 的第一版就栽在这里：
#: `ModelConfig` 的 `type` 是无默认值的位置参，不给它时三档全抛 `TypeError`，
#: 于是 `models.<名>.model` 一度被读成「已安全」。
UNMEASURABLE = []


def _build(dotted, value):
    """按点分路径构造一个键所在的配置节（`models.<名>.x` 走 `ModelConfig`）"""
    parts = dotted.split(".")
    if parts[0] == "models":
        return ModelConfig(type=MODEL_TYPES[0], **{parts[-1]: value})
    section = getattr(AppConfig(), parts[0])
    return type(section)(**{parts[-1]: value})


def _default(dotted):
    """该键的出厂默认值 —— 基线档用它，而不是随手编一个值

    这一格不是形式主义：`export.default_format` 之类清单键对 `"x"` 必然抛「不在清单里」，
    拿它当基线就会把「量得到」的键读成「量不到」。
    """
    parts = dotted.split(".")
    if parts[0] == "models":
        return getattr(ModelConfig(type=MODEL_TYPES[0]), parts[-1])
    return getattr(getattr(AppConfig(), parts[0]), parts[1])


def _arm(dotted, value):
    try:
        _build(dotted, value)
        return "accepted"
    except DataValidationError:
        return "raised"
    except TypeError as exc:  # 构造器形状问题 ≠ 判据判负
        return "raised:%s" % exc


def _probe(dotted):
    """该键在（默认值, 空串, 纯空白）三档上的行为结局"""
    current = _default(dotted)
    kind = "list" if isinstance(current, list) else "str"
    empty = [""] if kind == "list" else ""
    blank = ["   "] if kind == "list" else "   "
    return {"kind": kind,
            "baseline": _arm(dotted, current),
            "empty": _arm(dotted, empty),
            "blank": _arm(dotted, blank)}


def _partition():
    """把清单里的字符串键按（空串, 空白）两档分进四个桶"""
    buckets = {"holes": [], "both_open": [], "safe": [], "blank_only": []}
    for dotted in HOLES_CLOSED_IN_L83 + BLANK_ONLY_KEYS + STILL_UNGATED + ALREADY_SAFE:
        arms = _probe(dotted)
        if arms["baseline"] != "accepted":
            UNMEASURABLE.append(dotted)
        target = {("raised", "accepted"): "holes",
                  ("accepted", "accepted"): "both_open",
                  ("raised", "raised"): "safe",
                  ("accepted", "raised"): "blank_only"}[(arms["empty"], arms["blank"])]
        buckets[target].append(dotted)
    return buckets


class TestThePredicate:
    """谓词本身：空白≠空串、不裁剪、NBSP 也算空白"""

    @pytest.mark.parametrize("value", BLANKS)
    def test_every_whitespace_flavour_is_blank(self, value):
        assert is_blank_string(value), repr(value)

    @pytest.mark.parametrize("value", ["", "x", " x ", "my docs", "0.0.0.0",
                                       "%(message)s", "data"])
    def test_non_blank_values_are_not_blank(self, value):
        assert not is_blank_string(value), repr(value)

    def test_empty_string_is_not_the_same_verdict_as_blank(self):
        """这条是本文件的支点：`''` 走「空字符串」那一刀，`'   '` 走「纯空白」那一刀

        两者若合并成一刀，`logging.file` 的合法默认值就会被拒 —— L83 第一版正是如此，
        当场让 `AppConfig()` 构造失败（谓词的文档字符串里记着这次）。
        """
        assert not is_blank_string("")
        assert is_blank_string("   ")

    def test_predicate_never_mutates(self):
        value = "  padded  "
        require_string("web.host", value)
        assert value == "  padded  "


class TestEveryClosedHoleNowRejectsBlank:
    """十键的纯空白写法在 SDK 直构面上判负，且文案点名键与值"""

    @pytest.mark.parametrize("dotted", HOLES_CLOSED_IN_L83)
    def test_blank_is_rejected_where_empty_was(self, dotted):
        arms = _probe(dotted)
        assert arms["empty"] == "raised", "%s：空串本就一直被拒，本用例的对照失效" % dotted
        assert arms["blank"] == "raised", "%s：洞没关掉" % dotted

    @pytest.mark.parametrize("dotted", HOLES_CLOSED_IN_L83)
    def test_message_names_the_key_and_shows_the_invisible(self, dotted):
        arg = ["   "] if _probe(dotted)["kind"] == "list" else "   "
        with pytest.raises(DataValidationError) as caught:
            _build(dotted, arg)
        text = str(caught.value)
        assert "纯空白" in text, text
        assert "'   '" in text or "'\\xa0'" in text, text

    def test_the_five_blank_flavours_reach_the_same_verdict(self):
        for value in BLANKS:
            with pytest.raises(DataValidationError):
                WebConfig(host=value)

    def test_list_items_get_the_same_cut_as_scalars(self):
        with pytest.raises(DataValidationError) as caught:
            WebConfig(data_roots=["data", "\u00a0"])
        assert "每一项" in str(caught.value)

    def test_legal_values_with_inner_spaces_still_land(self):
        """收紧的边界：含空格的值不许被顺手拒掉，也不许被裁剪"""
        config = WebConfig(host="local host", static_dir="my docs",
                           data_roots=["my data"])
        assert config.host == "local host"
        assert config.static_dir == "my docs"
        assert config.data_roots == ["my data"]


class TestEmptyAndBlankAreDifferentVerdicts:
    """两种错各说各的话；`logging.file` 只挂第二刀"""

    def test_empty_message_unchanged(self):
        with pytest.raises(DataValidationError) as caught:
            require_string("web.host", "")
        assert str(caught.value) == "web.host 不能是空字符串"

    def test_blank_message_shows_repr(self):
        with pytest.raises(DataValidationError) as caught:
            require_string("web.host", "\u00a0\u00a0")
        assert str(caught.value) == "web.host 不能是纯空白字符串，当前是 '\\xa0\\xa0'"

    def test_list_blank_message_shows_repr(self):
        with pytest.raises(DataValidationError) as caught:
            require_string_list("web.data_roots", ["   "])
        assert "当前含 '   '" in str(caught.value)

    def test_logging_file_keeps_its_empty_string_meaning(self):
        assert LoggingConfig(file="").file == ""
        with pytest.raises(DataValidationError) as caught:
            LoggingConfig(file="   ")
        assert "空串 = 不落文件" in str(caught.value)

    def test_no_config_default_is_blank(self):
        """出厂默认必须一个都不落在新判据上（否则 `AppConfig()` 直接构造失败）"""
        sample = AppConfig()
        blanks = []
        for field in dataclasses.fields(AppConfig):
            section = getattr(sample, field.name)
            if not dataclasses.is_dataclass(section):
                continue
            for sub in dataclasses.fields(type(section)):
                current = getattr(section, sub.name)
                if isinstance(current, str) and is_blank_string(current):
                    blanks.append("%s.%s" % (field.name, sub.name))
                elif isinstance(current, list):
                    blanks += ["%s.%s" % (field.name, sub.name)
                               for item in current
                               if isinstance(item, str) and is_blank_string(item)]
        assert blanks == []


class TestShapeErrorsComeBeforeMembership:
    """A142 的立项现场：m8 那档红掉的用例名字写着「先报形状」，本轮让名字成立"""

    @pytest.mark.parametrize("name,choices", [
        ("vector.backend", ("faiss", "chromadb")),
        ("export.default_format", ("json", "csv")),
        ("rag.default_format", ("custom",)),
        ("logging.level", ("INFO", "WARNING")),
    ])
    def test_membership_never_gets_a_blank_to_judge(self, name, choices):
        with pytest.raises(DataValidationError) as caught:
            require_choice(name, "   ", choices)
        text = str(caught.value)
        assert "纯空白" in text, text
        assert "之一" not in text, "形状错被清单错盖掉了 ⇒ 用例名与行为又分家"


class TestThePredicateIsOneCopyOnly:
    """A77 在这一族的落点：一刀只有一个产地，静态面共引同一个对象"""

    def test_static_face_co_references_the_runtime_predicate(self):
        assert validator_predicate is is_blank_string

    def test_runtime_and_loader_share_the_same_object(self):
        assert config_module.is_blank_string is is_blank_string

    def test_the_binding_sites_are_derived_not_hand_copied(self):
        """「注入时要一起抽掉的模块清单」不许靠回忆：从 import 语句推导并逐字对账

        这一条是给**下一轮**立的，不是给本轮的。`TestTheGuardCanFail.BINDINGS` 那三处如
        果是手抄的，失效形状很具体：以后谁第四处 `import` 这个谓词，注入档仍只抽三处 ⇒
        那一处照判，「抽掉判据应当全绿变全红」的守卫就以**假红**收场，而读它的人会以为
        判据族出了第二个产地。所以判据写成集合等式，两边都是推导：左边是 AST 扫出来的
        「从 `validation` 模块 import `is_blank_string` 的全部产品模块」，右边是那份清单
        补上谓词自己住的那一处。
        """
        import ast
        import pathlib

        root = pathlib.Path(__file__).resolve().parents[2]
        importers = set()
        for pkg in ("augmentor", "api"):
            for path in sorted((root / pkg).rglob("*.py")):
                tree = ast.parse(path.read_text(encoding="utf-8"))
                for node in ast.walk(tree):
                    if (isinstance(node, ast.ImportFrom)
                            and (node.module or "").endswith("validation")
                            and any(a.name == "is_blank_string" for a in node.names)):
                        parts = list(path.relative_to(root).parts)
                        importers.add(".".join(parts[:-1] + [parts[-1][:-3]]))
        assert importers == {"augmentor.config", "augmentor.config_validator"}
        assert importers | {"augmentor.validation"} == set(
            TestTheGuardCanFail.BINDINGS), "共引点清单漂了：注入抽的不是全仓真正的绑定点"

    def test_new_spec_dimension_is_used_where_runtime_judges(self):
        """`non_blank` = 「空串合法、纯空白不合法」，目前只该有 `logging.file` 一格"""
        specs = ConfigValidator.KNOWN_FIELDS
        assert {p for p, spec in specs.items() if spec.get("non_blank")} == {
            "logging.file"}
        assert specs["logging.file"]["type"] is str


class TestStaticFaceAgreesWithRuntime:
    """`validate-config` 与 `load_config` 对同一份空白值同判（A83 的镜像症状不再出现）"""

    @pytest.mark.parametrize("body", [
        {"web": {"host": "   "}},
        {"web": {"data_roots": ["   "]}},
        {"vector": {"storage_dir": "\u00a0"}},
        {"multimodal": {"image_extensions": ["  "]}},
        {"logging": {"file": "   "}},
        {"models": {"svc": {"type": MODEL_TYPES[0], "model": "   "}}},
    ])
    def test_blank_value_reports_one_error_naming_the_path(self, body):
        result = validate_config({"app": {"name": "t", "version": "1.0.0"},
                                  "models": {"default": "ernie"}, **body})
        hits = [e for e in result.errors if "纯空白" in e.message]
        assert len(hits) == 1, [e.path for e in result.errors]
        assert hits[0].path.startswith(next(iter(body)))

    def test_a_blank_yaml_never_reaches_a_loaded_config(self, tmp_path):
        path = tmp_path / "blank.yaml"
        path.write_text("web:\n  host: '   '\n", encoding="utf-8")
        with pytest.raises(DataValidationError):
            load_config(str(path))

    def test_a_legal_yaml_still_loads(self, tmp_path):
        path = tmp_path / "ok.yaml"
        path.write_text("web:\n  host: 0.0.0.0\n  static_dir: my docs\n",
                        encoding="utf-8")
        config = load_config(str(path))
        assert config.web.static_dir == "my docs"


class TestTheBoundaryIsThePartition:
    """反空转 + 不越界：四个桶的精确划分就是本轮的边界声明"""

    def test_partition_is_exactly_what_l83_left(self):
        buckets = _partition()
        assert buckets["holes"] == [], "又出现「拒空串、放行空白」的键"
        assert sorted(buckets["blank_only"]) == sorted(BLANK_ONLY_KEYS)
        assert sorted(buckets["both_open"]) == sorted(STILL_UNGATED), \
            "「两档全放」集合变了 ⇒ 要么动了 A140 那一片（越界），要么口径漂移"
        assert sorted(buckets["safe"]) == sorted(ALREADY_SAFE + HOLES_CLOSED_IN_L83)

    def test_nothing_was_dropped_in_the_shuffle(self):
        buckets = _partition()
        total = sum(len(v) for v in buckets.values())
        assert total == len(HOLES_CLOSED_IN_L83) + len(BLANK_ONLY_KEYS) \
            + len(STILL_UNGATED) + len(ALREADY_SAFE), "有键同时不落任何桶或落了两处"

    def test_probe_covers_every_key_it_claims(self):
        assert len(set(HOLES_CLOSED_IN_L83) | set(BLANK_ONLY_KEYS)
                   | set(STILL_UNGATED) | set(ALREADY_SAFE)) == 33, \
            "清单本身有空集/重名，分桶断言就只是自等"

    def test_no_key_is_unmeasurable(self):
        _partition()
        assert UNMEASURABLE == [], "这些键的默认值本身就抛，raised 是假安全"


class TestTheGuardCanFail:
    """把谓词换成永假 ⇒ 洞重新出现，证明上面的红来自这一刀而不是别处"""

    #: 谓词在三个模块里各有一个名字（定义处 + 两处共引），注入要同时抽掉三个绑定点。
    #: 只 patch `validation` 会得出**假红**：`LoggingConfig` 与静态面手里是各自 import
    #: 的那个名字，抽一处仍判负。本条注释就是这个测试第一版的失败原因，留着是因为
    #: 「共引同一个对象」这件事只在替换时才会现形 —— 平时三条 `is` 断言全绿。
    BINDINGS = ("augmentor.validation", "augmentor.config", "augmentor.config_validator")

    def test_disabling_the_predicate_reopens_every_hole(self, monkeypatch):
        import importlib

        for name in self.BINDINGS:
            monkeypatch.setattr(importlib.import_module(name), "is_blank_string",
                                lambda value: False)
        assert _probe("web.host")["blank"] == "accepted", \
            "抽掉判据仍然红 ⇒ 空白不是一刀判的，本守卫的红另有来源"
        assert _probe("logging.file")["blank"] == "accepted"
        assert _probe("models.<名>.model")["blank"] == "accepted"
        with pytest.raises(DataValidationError):
            require_string("web.host", "")  # 空串那一刀不受影响
        assert _probe("vector.backend")["blank"] == "raised", \
            "清单键的 membership 仍在：本轮没有把别的判据卷进来"
