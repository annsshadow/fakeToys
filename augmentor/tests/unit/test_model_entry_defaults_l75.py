# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""A114 + A119 + A120（L75）：模型条目的回落值只有一个权威、必填两侧同判、报错点得出条目名

改前三份实测（`Temp/l75q/before.json`，工作树与 HEAD 逐字节一致时取的数）：

- **A114 回落值漂移** —— `load_config` 手抄了九个回落值，四个与 `ModelConfig` 的
  字段默认不一致：`api_key` / `secret_key` / `base_url` 是 `''` 对 `None`，`model` 是
  `''` 对 `'default'`。前一支**行为等价**（探针 `credential_shape` 十例：五个后端对
  `None` 与 `''` 给出逐字相同的 `ModelNotConfiguredError`），后一支是真缺陷：空串模型名
  会原样直发，`openai` / `claude` / `ollama` 的请求体带 `"model": ""`，`gemini` 的
  模型名在 URL 里 ⇒ 实测得到 `.../v1beta/models/:generateContent`；而 `model` 这个键
  两侧都无判据，`model: ''` / `null` / `123` / `true` 四形状**两个面全绿**。
- **A119 静态面看不见「键不在场」** —— `models.qwen` 整条不写 `type`：`is_valid=True` /
  0 error / 0 warning，只有 `load_config` 抛。
- **A120 消息指不出位置** —— 运行时五族消息一律写死 `models.<名字>.xxx`，两个坏条目
  的抛错消息逐字相同（静态面反而没这个洞，它按点分路径报错）。

判据的住处没动：合法性仍然只在 `ModelConfig.__post_init__`（A77），本轮加载层补的是
它结构上看不见的那一维（「键在不在场」）与它拿不到的那一个信息（条目名）。
"""

import ast
import dataclasses
import inspect
import pathlib

import pytest

from augmentor.config import (MODEL_ENTRY_KEYS, MODEL_ENTRY_PLACEHOLDER,
                              ModelConfig, _model_entry, load_config)
from augmentor.config_validator import ConfigValidator
from augmentor.exceptions import DataValidationError
from augmentor.models import (ClaudeBackend, ERNIEBackend, GeminiBackend,
                              OllamaBackend, OpenAIBackend)

SHIPPED_YAML = pathlib.Path(__file__).resolve().parent.parent.parent / "config.yaml"

# (条目正文, 断言用的读法)。每个键都从 `ModelConfig` 推，不写死清单 —— A114 的根
# 正是「抄了一份就会漂」，测试这边再抄一份就是第二个权威。
DEFAULTED_KEYS = sorted(f.name for f in dataclasses.fields(ModelConfig)
                        if f.default is not dataclasses.MISSING)

# (键, 坏形状, 一句话原因)。`model` 是本轮第一次有判据的第二个字符串键。
BAD_MODEL_VALUES = [
    ("", "空串模型名会被原样直发后端"),
    (None, "写了键没给值"),
    (123, "YAML 里的裸数字不是模型名"),
    (True, "布尔被读成模型名"),
    (["gpt"], "列表同理，形状合法不等于语义合法"),
]

# 五族消息的探针：(键, 值)。每例都要求「加载面消息里出现真条目名」。
MESSAGE_FAMILIES = [
    ("type", "ernie"),
    ("model", ""),
    ("temperature", 999),
    ("top_p", 2.5),
    ("max_output_tokens", 0),
    ("request_timeout", 601),
]

BACKENDS = [
    ("openai", OpenAIBackend, {"api_key": "k"}, {"choices": [{"message": {"content": "x"}}]}),
    ("claude", ClaudeBackend, {"api_key": "k"}, {"content": [{"text": "x"}]}),
    ("gemini", GeminiBackend, {"api_key": "k"},
     {"candidates": [{"content": {"parts": [{"text": "x"}]}}]}),
    ("ollama", OllamaBackend, {"base_url": "http://127.0.0.1:1"},
     {"message": {"content": "x"}, "done": True}),
    ("baidu", ERNIEBackend, {"api_key": "k", "secret_key": "s"},
     [{"access_token": "t", "expires_in": 1000}, {"result": "x"}]),
]


def _raw(entry_body, name="m"):
    return {"models": {"default": name, name: entry_body}, "augmentation": {}}


def _write(tmp_path, body, name="m"):
    path = tmp_path / "c.yaml"
    path.write_text("models:\n  default: %s\n%s" % (name, body), encoding="utf-8")
    return str(path)


def _errors_for(result, path):
    return [e for e in result.errors if e.path == path]


class TestLoaderHasNoSecondAuthority:
    """A114 本体：回落值由字段默认唯一决定，加载器不再抄第二份"""

    def test_every_defaulted_key_survives_absence(self, tmp_path):
        """八个有默认值的键：YAML 里不写 = 拿到 dataclass 的那一档，逐字相等"""
        assert len(DEFAULTED_KEYS) == 8, "`type` 应是唯一没有默认值的字段"
        path = _write(tmp_path, "  m:\n    type: openai\n")
        loaded = load_config(path).models["m"]
        plain = ModelConfig(type="openai")
        for key in DEFAULTED_KEYS:
            assert getattr(loaded, key) == getattr(plain, key), \
                "%s 的加载器回落与 dataclass 默认漂了" % key

    def test_no_source_reads_an_entry_key_by_name_anymore(self):
        """机械守卫：装配链路上不许再有「点名读某个条目键」的写法

        这条判的是 A114 的**形状**而不是取值：上面那条只验当下不漂，而漂移的机制恰恰
        是「在加载器里 `conf.get('temperature', 手抄默认)`」。用 AST 而不是文本匹配 ——
        改后的注释里就还留着 `conf.get('type', '')` 这句历史描述，文本判据会把自己的
        注释也当成缺陷（本轮第一版就红在这里，红得对：说明那种写法太容易复现）。

        两个函数一起判：`load_config`（改前的九个手抄默认就在这里）与 `_model_entry`
        （改后唯一的装配口）。
        """
        reads = set()
        for func in (load_config, _model_entry):
            for node in ast.walk(ast.parse(inspect.getsource(func))):
                if (isinstance(node, ast.Call)
                        and isinstance(node.func, ast.Attribute)
                        and node.func.attr == "get" and node.args
                        and isinstance(node.args[0], ast.Constant)
                        and isinstance(node.args[0].value, str)):
                    reads.add(node.args[0].value)
        assert not (reads & set(MODEL_ENTRY_KEYS)), \
            "按名读取的条目键：%s —— 那就是第二个回落权威" % sorted(
                reads & set(MODEL_ENTRY_KEYS))

    def test_every_written_key_actually_lands(self, tmp_path):
        """反向一半：YAML 里写了的九个键必须逐个生效

        上面那条判「不写 = 由字段默认回答」，本条判「写了 = 真的落进条目」。两半合
        起来才等于「装配口覆盖全部字段」：只有前一半时，把 `MODEL_ENTRY_KEYS` 手抄成
        七个键（漏两个）照样全绿，而症状是「配了不生效」—— A114 的镜像形状。
        取值清单同样从 `fields()` 推，不写死。
        """
        written = {"type": "openai", "api_key": "ak", "secret_key": "sk",
                   "base_url": "http://127.0.0.1:1", "model": "gpt-4o",
                   "temperature": 0.5, "top_p": 0.5, "max_output_tokens": 128,
                   "request_timeout": 7}
        assert sorted(written) == sorted(MODEL_ENTRY_KEYS), \
            "`ModelConfig` 加了字段但本用例的取值表没跟上"
        body = "".join("    %s: %s\n" % (k, v) for k, v in written.items())
        entry = load_config(_write(tmp_path, "  m:\n" + body)).models["m"]
        for key, value in written.items():
            assert getattr(entry, key) == value, "%s 写了却没落地" % key

    def test_unknown_entry_keys_are_still_dropped_not_crashed(self, tmp_path):
        """未知键由「写了没人读」那一路出声（A76 / A84），装配处不重复判也不崩"""
        path = _write(tmp_path, "  m:\n    type: openai\n    temperture: 0.5\n")
        assert load_config(path).models["m"].temperature == 0.99


class TestModelKeyShapeBothFaces:
    """`model` 的两侧同判：本轮补上的第二个字符串键（A114 的那一支真缺陷）"""

    @pytest.mark.parametrize("value,why", BAD_MODEL_VALUES)
    def test_runtime_rejects(self, value, why):
        with pytest.raises(DataValidationError, match="model"):
            ModelConfig(type="openai", model=value)

    @pytest.mark.parametrize("value,why", BAD_MODEL_VALUES)
    def test_static_face_reports_the_same_path(self, value, why):
        errors = _errors_for(ConfigValidator().validate_config(
            _raw({"type": "openai", "model": value})), "models.m.model")
        assert errors, "model=%r（%s）在 validate-config 上零反馈" % (value, why)
        assert errors[0].severity.value == "error"

    @pytest.mark.parametrize("value", ["gpt-4o", "ernie-3.5-8k", "default"])
    def test_a_real_model_name_is_green_on_both_faces(self, value):
        assert ModelConfig(type="openai", model=value).model == value
        assert _errors_for(ConfigValidator().validate_config(
            _raw({"type": "openai", "model": value})), "models.m.model") == []

    def test_the_empty_model_url_shape_cannot_come_back(self, tmp_path):
        """行为面：省略 `model` 的条目不再拼出 `models/:generateContent`

        这是 A114 那一支的**症状**判据。改前实测（`before.json`）gemini 拿到的是
        `.../v1beta/models/:generateContent`，服务端回一条 400 而真因看不见；
        现在省略档落在 `'default'`，坏形状（空串）在加载期就被拒 ⇒ 这条 URL 无法再出现。
        """
        path = _write(tmp_path, "  m:\n    type: gemini\n    api_key: k\n")
        backend = GeminiBackend(load_config(path).models["m"])
        assert "models/:generate" not in backend.api_url
        assert "models/default:generate" in backend.api_url


class TestCredentialsAreEquivalentFalsyShapes:
    """把「本改对凭证消费方零影响」从推理论证升级成断言

    A114 让省略的 `api_key` / `secret_key` / `base_url` 从 `''` 变成 `None`。改前
    取证（`before.json` 的 `credential_shape` 十例）就说两种假值在五个后端上同判，
    但那是探针读数；这一类「换了形状所以行为没换」的主张最容易在下一根旋钮上失效，
    所以钉成用例。
    """

    @pytest.mark.parametrize("type_name,cls,creds,resp", BACKENDS,
                             ids=[b[0] for b in BACKENDS])
    def test_none_and_empty_string_reach_the_same_verdict(self, type_name, cls,
                                                          creds, resp):
        def verdict(**overrides):
            kwargs = {"type": type_name, "model": "m1"}
            kwargs.update(overrides)
            try:
                cls(ModelConfig(**kwargs))
                return "构造成功"
            except Exception as exc:  # noqa: BLE001 - 判的就是异常本身
                return "%s: %s" % (type(exc).__name__, exc)

        empty = {k: "" for k in creds}
        missing = {k: None for k in creds}
        assert verdict(**empty) == verdict(**missing), \
            "%s 后端对 `''` 与 `None` 开始分判了：A114 的等价前提失效" % type_name


class TestStaticFaceSeesAbsentKeys:
    """A119：`required` 这一维第一次走到模型条目上"""

    def test_missing_type_is_red_with_the_existing_wording(self):
        errors = _errors_for(ConfigValidator().validate_config(
            _raw({"model": "qwen-plus"})), "models.m.type")
        assert [e.message for e in errors] == ["缺少必填字段: models.m.type"]

    def test_null_type_is_red_with_the_existing_wording(self):
        errors = _errors_for(ConfigValidator().validate_config(
            _raw({"type": None, "model": "x"})), "models.m.type")
        assert any("字段不能为null" in e.message for e in errors)

    def test_null_type_is_the_other_branch_on_the_loader_face(self, tmp_path):
        """加载侧的「键在场、值缺席」与「键整条不在场」是两支判据、两种文案

        YAML 写 `type:` 留空 = `None`，它必须落进 `不能是 null` 那一支；如果哪天
        有人把它并回「缺少必填字段」，本条会红 —— 那等于对用户说「你没写」，
        而真话是「你写了但没给值」（A119 的两半）。
        """
        path = _write(tmp_path, "  m:\n    type:\n")
        with pytest.raises(DataValidationError) as ei:
            load_config(path)
        assert str(ei.value).startswith("models.m.type 不能是 null"), \
            "null 那一支的文案或条目名漂了：%s" % ei.value

    def test_every_entry_gets_its_own_path(self):
        """两条都缺 `type` ⇒ 两条判决、两个路径，不是一句概括"""
        result = ConfigValidator().validate_config(
            {"models": {"default": "a", "a": {"model": "x"}, "b": {"model": "y"}},
             "augmentation": {}})
        assert sorted(e.path for e in result.errors
                      if "缺少必填字段" in e.message) == [
            "models.a.type", "models.b.type"]

    def test_default_pointer_is_not_an_entry(self):
        """`models.default` 不是模型条目：不许对它报「缺 type」"""
        errors = _errors_for(ConfigValidator().validate_config(
            {"models": {"default": None}, "augmentation": {}}), "models.default.type")
        assert errors == []

    def test_non_mapping_entry_gets_no_required_noise(self):
        """条目正文写成标量时不叠一条「缺少必填字段」噪声（形状那一层管）"""
        result = ConfigValidator().validate_config(
            {"models": {"default": "m", "m": "abc"}, "augmentation": {}})
        assert [e.path for e in result.errors] == []

    def test_only_type_is_required_in_the_entry_table(self):
        """必填只有 `type` 一根：其余八键「不在场」是合法状态（由字段默认回答）

        反向也要判：哪天有人给 `temperature` 之类补上 `required`，出厂模板与
        本条都会红 —— 那意味着静态面开始比运行时严（A77 的原始症状）。
        """
        required = sorted(k for k, s in ConfigValidator.MODEL_ENTRY_FIELDS.items()
                          if s.get("required"))
        assert required == ["type"]
        for name, body in _shipped_entries().items():
            assert "type" in body, "出厂模板的 %s 条目没有 type，改前判负" % name


class TestMessagesNameTheEntry:
    """A120：消息里的条目名由加载层补上，判据本身不动"""

    @pytest.mark.parametrize("key,value", MESSAGE_FAMILIES,
                             ids=[m[0] for m in MESSAGE_FAMILIES])
    def test_loader_message_names_the_real_entry(self, tmp_path, key, value):
        body = "  beta:\n    type: openai\n" if key != "type" else "  beta:\n"
        path = _write(tmp_path, body + "    %s: %s\n" % (key, repr(value)))
        with pytest.raises(DataValidationError) as ei:
            load_config(path)
        message = str(ei.value)
        assert "models.<名字>" not in message, "占位符漏到了产品面"
        assert message.startswith("models.beta.%s" % key), \
            "%s 族的消息没点名条目：%s" % (key, message)

    @pytest.mark.parametrize("key,value", MESSAGE_FAMILIES,
                             ids=[m[0] for m in MESSAGE_FAMILIES])
    def test_direct_construction_keeps_the_placeholder(self, key, value):
        """判据仍住在 dataclass：SDK 直构拿不到条目名，占位符就是它的诚实读数"""
        with pytest.raises(DataValidationError) as ei:
            ModelConfig(**({"type": value} if key == "type" else
                           {"type": "openai", key: value}))
        assert str(ei.value).startswith(MODEL_ENTRY_PLACEHOLDER)

    def test_placeholder_is_defined_once(self):
        """五族消息共引一个占位常量：源码里 `models.<名字>` 只许出现在常数定义处

        这条防的是「下一根旋钮又手写一遍字面量」—— 那种写法在功能上与常量等价，
        于是 A120 的替换会漏掉它，而且没有任何测试会红。
        """
        source = (pathlib.Path(__file__).resolve().parents[2] /
                  "augmentor" / "config.py").read_text(encoding="utf-8")
        assert source.count('"models.<名字>"') == 1, \
            "占位符字面量出现了第二处：消息拼装回到了硬编码"

    def test_two_bad_entries_now_tell_apart(self, tmp_path):
        """改前那两个坏条目的消息逐字相同（`before.json` 的 `a120_messages` 档）"""
        path = _write(tmp_path,
                      "  alpha:\n    type: openai\n    temperature: 999\n"
                      "  beta:\n    type: openai\n    temperature: -1\n")
        with pytest.raises(DataValidationError) as ei:
            load_config(path)
        assert "models.alpha.temperature" in str(ei.value)


class TestRetiredShapesAreRecorded:
    """改前写出、改后被拒的形状：把破坏面写下来而不是假装没有"""

    def test_a_saved_empty_model_name_is_now_rejected(self, tmp_path):
        """旧版 `save_config` 会把省略的 `model` 落成 `model: ''` —— 改后这种文件判负

        判负是本轮**主动选的**破坏面：症状是「一条能读的报错」而不是「请求发出去
        换回 400」。仓库内跟踪的 YAML 实测零命中（`git ls-files '*.yaml'` 只有
        `config.yaml` 与 compose 文件，五条条目全写了 `model`）。
        """
        path = _write(tmp_path, "  m:\n    type: openai\n    model: ''\n")
        with pytest.raises(DataValidationError, match="不能是空字符串"):
            load_config(path)

    def test_an_entry_omitting_model_still_loads_and_round_trips(self, tmp_path):
        """省略 `model` 不受影响：拿到 `'default'`，存回来再读仍然合法"""
        from augmentor.config import save_config

        path = _write(tmp_path, "  m:\n    type: openai\n    api_key: k\n")
        config = load_config(path)
        assert config.models["m"].model == "default"
        target = pathlib.Path(path).with_name("saved.yaml")
        save_config(config, str(target))
        assert load_config(str(target)).models["m"].model == "default"

    def test_shipped_yaml_is_green_on_both_faces(self):
        config = load_config(str(SHIPPED_YAML))
        result = ConfigValidator().validate_file(str(SHIPPED_YAML))
        assert [e.message for e in result.errors] == []
        assert config.models, "出厂模板一个条目都没有"


def _shipped_entries():
    import yaml

    raw = yaml.safe_load(SHIPPED_YAML.read_text(encoding="utf-8")) or {}
    models = raw.get("models") or {}
    return {k: v for k, v in models.items() if k != "default"}


class TestPinnedGaps:
    """「钉住不修」清单：让下一轮看得见，也让新键无法悄悄加进未判据那一档"""

    def test_credentials_are_the_only_unjudged_entry_keys(self):
        """A122：三条凭证键至今两侧都无判据（`api_key: 123` 实测能过）"""
        unjudged = set(MODEL_ENTRY_KEYS) - set(
            ConfigValidator.MODEL_ENTRY_FIELDS)
        assert unjudged == {"api_key", "secret_key", "base_url"}, \
            "模型条目里「两侧无判据」的键集变了：新旋钮要么补判据，要么记成缺陷"

    def test_model_entry_helper_is_the_single_entry_door(self):
        """`_model_entry` 是唯一的条目装配口：直构 `ModelConfig` 的回落权威在别处不成立"""
        entry = _model_entry("solo", {"type": "openai", "temperature": 0.5})
        assert entry.temperature == 0.5
        with pytest.raises(DataValidationError, match="models.solo.temperature"):
            _model_entry("solo", {"type": "openai", "temperature": 999})
