# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""配置管理单元测试

验证配置加载、环境变量解析、默认值合并与校验。
"""

import os
from pathlib import Path

import pytest

from augmentor.config import (
    AppConfig,
    ModelConfig,
    load_config,
    get_model_config,
    MultilingualConfig,
    RAGConfig,
    VectorConfig,
    BenchmarkConfig,
    ActiveLearningConfig,
    FrameworkConfig,
    MultimodalConfig,
    EvaluationConfig,
)

AI_DIR = Path(__file__).resolve().parent.parent.parent


class TestDefaultConfig:
    """默认配置行为"""

    def test_returns_defaults_when_no_path(self):
        """未指定配置文件时必须返回可直接使用的默认配置"""
        config = load_config()
        assert isinstance(config, AppConfig)
        assert config.default_model == "ernie"
        assert config.augmentation.variants_per_seed == 5
        assert config.quality.threshold == 0.6

    def test_returns_defaults_when_file_missing(self, tmp_path):
        """配置文件缺失不应抛异常，而是回退默认值（保证 CLI 可用）"""
        config = load_config(str(tmp_path / "not_exists.yaml"))
        assert config.default_model == "ernie"

    def test_new_sections_have_defaults(self):
        """新增配置段必须有默认值，否则调用方需要到处判空"""
        config = AppConfig()
        assert isinstance(config.multilingual, MultilingualConfig)
        assert isinstance(config.rag, RAGConfig)
        assert isinstance(config.vector, VectorConfig)
        assert isinstance(config.benchmark, BenchmarkConfig)
        assert isinstance(config.active_learning, ActiveLearningConfig)
        assert isinstance(config.frameworks, FrameworkConfig)
        assert isinstance(config.multimodal, MultimodalConfig)
        assert isinstance(config.evaluation, EvaluationConfig)


class TestLoadFromFile:
    """从 YAML 文件加载"""

    def test_loads_project_config(self):
        """项目自带 config.yaml 必须可被完整解析"""
        config = load_config(str(AI_DIR / "config.yaml"))

        assert config.default_model == "ernie"
        # 五种模型后端都应被识别
        assert set(config.models.keys()) == {"ernie", "openai", "ollama", "claude", "gemini"}
        assert config.models["claude"].type == "claude"
        assert config.models["gemini"].type == "gemini"
        assert config.vector.backend == "faiss"
        assert config.rag.chunk_size == 512
        assert config.active_learning.strategy == "uncertainty"

    def test_env_var_resolution(self, tmp_path, monkeypatch):
        """配置中的 ${VAR} 必须被环境变量替换，避免明文密钥入库"""
        monkeypatch.setenv("TEST_MODEL_KEY", "resolved-key")

        config_file = tmp_path / "config.yaml"
        config_file.write_text(
            "models:\n"
            "  default: dummy\n"
            "  dummy:\n"
            "    type: openai\n"
            "    api_key: ${TEST_MODEL_KEY}\n",
            encoding='utf-8'
        )

        config = load_config(str(config_file))
        assert config.models["dummy"].api_key == "resolved-key"

    def test_missing_env_var_becomes_empty(self, tmp_path, monkeypatch):
        """环境变量缺失时应解析为空字符串，由后端在构造期报错"""
        monkeypatch.delenv("ABSENT_KEY", raising=False)

        config_file = tmp_path / "config.yaml"
        config_file.write_text(
            "models:\n"
            "  default: dummy\n"
            "  dummy:\n"
            "    type: openai\n"
            "    api_key: ${ABSENT_KEY}\n",
            encoding='utf-8'
        )

        config = load_config(str(config_file))
        assert config.models["dummy"].api_key == ""

    def test_partial_config_keeps_defaults(self, tmp_path):
        """只配置部分字段时，其余字段应保留默认值（合并而非覆盖）"""
        config_file = tmp_path / "config.yaml"
        config_file.write_text(
            "quality:\n  threshold: 0.85\n",
            encoding='utf-8'
        )

        config = load_config(str(config_file))
        assert config.quality.threshold == 0.85
        assert config.quality.enabled is True
        assert config.dedup.threshold == 0.9

    def test_invalid_yaml_content_raises(self, tmp_path):
        """YAML 顶层不是映射时必须显式报错，不能静默吞掉"""
        config_file = tmp_path / "config.yaml"
        config_file.write_text("models: [1, 2, 3]\n", encoding='utf-8')

        with pytest.raises(Exception):
            load_config(str(config_file))


class TestGetModelConfig:
    """模型配置查询"""

    def test_get_default_model(self):
        """未指定名称时返回默认模型配置"""
        config = load_config(str(AI_DIR / "config.yaml"))
        model_config = get_model_config(config)
        assert model_config.type == "baidu"

    def test_get_named_model(self):
        """指定名称时返回对应模型配置"""
        config = load_config(str(AI_DIR / "config.yaml"))
        model_config = get_model_config(config, "claude")
        assert model_config.type == "claude"

    def test_unknown_model_raises(self):
        """查询未配置模型必须报错，并提示可用模型列表"""
        config = load_config(str(AI_DIR / "config.yaml"))
        with pytest.raises(ValueError) as excinfo:
            get_model_config(config, "not-exists")
        assert "可用模型" in str(excinfo.value)


class TestModelConfig:
    """模型配置数据类"""

    def test_defaults(self):
        """模型配置应有合理的默认采样参数"""
        model_config = ModelConfig(type="openai")
        assert model_config.temperature == 0.99
        assert model_config.top_p == 0.95
        assert model_config.max_output_tokens == 2048


class TestConfigExtended:
    """AppConfig 扩展测试"""

    def test_app_config_defaults(self):
        """AppConfig 默认值"""
        config = AppConfig()
        assert config.default_model == "ernie"
        assert config.augmentation.variants_per_seed == 5

    def test_model_config_custom(self):
        """ModelConfig 自定义参数"""
        config = ModelConfig(type="openai", temperature=0.5, top_p=0.9)
        assert config.temperature == 0.5
        assert config.top_p == 0.9

    def test_quality_config_defaults(self):
        """QualityConfig 默认值"""
        config = AppConfig()
        assert config.quality.threshold == 0.6
        assert config.quality.enabled is True

    def test_dedup_config_defaults(self):
        """DedupConfig 默认值"""
        config = AppConfig()
        assert config.dedup.threshold == 0.9

    def test_multilingual_config_defaults(self):
        """MultilingualConfig 默认值"""
        config = MultilingualConfig()
        assert config.enabled is False

    def test_rag_config_defaults(self):
        """RAGConfig 默认值"""
        config = RAGConfig()
        assert config.chunk_size == 512
        assert config.chunk_overlap == 64

    def test_vector_config_defaults(self):
        """VectorConfig 默认值"""
        config = VectorConfig()
        assert config.backend == "faiss"

    def test_benchmark_config_defaults(self):
        """BenchmarkConfig 默认值"""
        config = BenchmarkConfig()
        assert isinstance(config.metrics, list)

    def test_active_learning_config_defaults(self):
        """ActiveLearningConfig 默认值"""
        config = ActiveLearningConfig()
        assert config.strategy == "uncertainty"

    def test_framework_config_defaults(self):
        """FrameworkConfig 默认值"""
        config = FrameworkConfig()
        assert isinstance(config.frameworks, list)

    def test_save_config_strips_secrets(self, tmp_path):
        """save_config 应剥离模型密钥，避免落盘"""
        import yaml
        from augmentor.config import save_config

        config = AppConfig()
        config.models["openai"] = ModelConfig(type="openai", api_key="sk-actual-secret")
        config.models["ernie"] = ModelConfig(type="baidu", api_key="ak", secret_key="real-secret")
        path = str(tmp_path / "out_config.yaml")
        save_config(config, path)
        with open(path, encoding="utf-8") as f:
            content = f.read()
        assert "sk-actual-secret" not in content
        assert "real-secret" not in content
        with open(path, encoding="utf-8") as f:
            data = yaml.safe_load(f)
        assert "api_key" not in data["models"]["openai"]
        assert "secret_key" not in data["models"]["ernie"]


class TestSaveConfigRoundTrip:
    """`save_config` → `load_config` / `ConfigValidator` 必须闭环

    这一组测的是「保存再加载是否丢东西」。此前两处会静默丢配置：

    * `default_model` 被写成顶层键，而 `load_config` 只读 `models.default`，
      于是保存后默认模型回落到 `ernie`；
    * 非默认模型的 `api_key` 被整个 `pop` 掉，于是 `api_key: ${BAIDU_API_KEY}`
      这行消失，重载后该模型拿不到密钥。
    """

    def test_default_model_survives_round_trip(self, tmp_path):
        """默认模型必须能从保存后的文件读回来"""
        from augmentor.config import save_config

        path = str(tmp_path / "c.yaml")
        config = AppConfig()
        config.default_model = "gemini"
        save_config(config, path)

        assert load_config(path).default_model == "gemini"

    def test_env_placeholder_is_preserved(self, tmp_path):
        """文件里原有的 `${VAR}` 占位符必须原样保留，而不是被删掉"""
        import yaml
        from augmentor.config import save_config

        path = tmp_path / "c.yaml"
        path.write_text(
            "models:\n  default: ernie\n  ernie:\n    type: baidu\n"
            "    api_key: ${BAIDU_API_KEY}\n    secret_key: ${BAIDU_SECRET_KEY}\n",
            encoding="utf-8",
        )
        config = load_config(str(path))
        save_config(config, str(path))

        data = yaml.safe_load(path.read_text(encoding="utf-8"))
        assert data["models"]["ernie"]["api_key"] == "${BAIDU_API_KEY}"
        assert data["models"]["ernie"]["secret_key"] == "${BAIDU_SECRET_KEY}"

    def test_plaintext_secret_is_never_written_back(self, tmp_path):
        """文件里若写着明文密钥，保存时必须丢弃而非回写"""
        from augmentor.config import save_config

        path = tmp_path / "c.yaml"
        path.write_text(
            "models:\n  default: m1\n  m1:\n    type: openai\n"
            "    api_key: sk-literal-secret\n",
            encoding="utf-8",
        )
        save_config(load_config(str(path)), str(path))

        text = path.read_text(encoding="utf-8")
        assert "sk-literal-secret" not in text

    def test_unmodelled_top_level_section_is_preserved(self, tmp_path):
        """`AppConfig` 不建模的顶层段落（如 `app`）不得被覆盖掉"""
        import yaml
        from augmentor.config import save_config

        path = tmp_path / "c.yaml"
        path.write_text(
            "app:\n  name: ai-augmentor\n  version: '1.0'\n"
            "models:\n  default: ernie\n",
            encoding="utf-8",
        )
        save_config(load_config(str(path)), str(path))

        data = yaml.safe_load(path.read_text(encoding="utf-8"))
        assert data["app"] == {"name": "ai-augmentor", "version": "1.0"}

    def test_saved_config_passes_validator(self, tmp_path):
        """保存出来的配置必须能通过项目自己的校验器

        否则「保存配置」再「校验配置」必然失败——这曾是真实状态（4 个必填错误）。
        """
        from augmentor.config import save_config
        from augmentor.config_validator import validate_config_file

        path = str(tmp_path / "c.yaml")
        config = AppConfig()
        config.default_model = "openai"
        save_config(config, path)

        result = validate_config_file(path)
        assert result.is_valid, [e.message for e in result.errors]

    def test_stray_default_entry_is_normalised(self, tmp_path):
        """`models["default"]` 塞了字典（畸形输入）时，落盘必须是模型名"""
        import yaml
        from augmentor.config import save_config

        path = str(tmp_path / "c.yaml")
        config = AppConfig()
        config.models["default"] = {"api_key": "SHOULD_NOT_APPEAR"}
        save_config(config, path)

        text = Path(path).read_text(encoding="utf-8")
        assert "SHOULD_NOT_APPEAR" not in text
        assert yaml.safe_load(text)["models"]["default"] == "ernie"

class TestSectionRegistryMatchesAppConfigFields:
    """`load_config` 的「节 → 类」清单必须与 `AppConfig` 的节字段两集相等（A123 改形）

    本类的前身叫 `TestSectionDefaultsMatchTheMappingTable`，钉的是「手抄的那张默认值表
    与 dataclass 逐字段同值、同覆盖」。**A123 把表删了**（`_load_section` 现在按那个类
    自己的 dataclass 字段集取键，回落值只有字段默认一个权威），于是：

    - **值漂移**结构上不存在：没有第二处可以抄；
    - **覆盖漂移**结构上不存在：字段有而表漏 ⇒ 现在等价于「字段有就一定读」；
      所以 `KNOWN_UNMAPPED_FIELDS` 那张棘轮清单随本条一起退役，留一项就是留一份
      已经不存在的表。

    还剩的手写物只有**节清单**本身（20 个 `(节名, 类)` 二元组），它的两种漏法都还有
    真实症状：清单少一节 ⇒ 该节配置永远不加载（回 200 但读回出厂值）；清单多一节名 ⇒
    `setattr` 往 `AppConfig` 上挂一个字段之外的属性，`dataclasses.asdict` 那一路看不到
    它。两个方向由本类逐节对账，「写了的键必须落地」那一维见
    `tests/unit/test_section_registry_l77.py`。
    """

    @staticmethod
    def _sections():
        import dataclasses

        from augmentor.config import AppConfig

        return [f.name for f in dataclasses.fields(AppConfig)
                if f.default_factory not in (None, dataclasses.MISSING)
                and dataclasses.is_dataclass(f.default_factory())]

    @staticmethod
    def _write(tmp_path, sections):
        import yaml

        path = tmp_path / "empty.yaml"
        with open(path, "w", encoding="utf-8") as handle:
            yaml.safe_dump({name: {} for name in sections}, handle)
        return str(path)

    def test_there_are_twenty_dataclass_sections(self, tmp_path):
        """新增配置节会立刻改变这里的计数，逼作者决定它进不进守护范围"""
        assert len(self._sections()) == 20

    def test_empty_sections_fall_back_to_dataclass_defaults(self, tmp_path):
        """只写节名不写字段时，读出来的必须与 dataclass 默认实例逐节相等"""
        from augmentor.config import AppConfig, load_config

        sections = self._sections()
        loaded = load_config(self._write(tmp_path, sections))
        baseline = AppConfig()
        drifted = []
        for name in sections:
            got, want = getattr(loaded, name), getattr(baseline, name)
            if got != want:
                drifted.append(
                    (name, {k: (getattr(got, k), getattr(want, k))
                            for k in vars(got)
                            if getattr(got, k) != getattr(want, k)}))
        assert drifted == []

    def test_registry_names_and_classes_match_appconfig(self, tmp_path):
        """清单与 `AppConfig` 双向对账：节名集合相等，且每节传的就是那个字段的类型

        拦截 `_load_section` 记录 `(节名, 类)` 两件事。类这一半不是同义反复：校验器那侧
        的白名单是从 `AppConfig` 的字段类型**推导**的（`consumed_section_keys`），而清单
        是手写的，两边可以各指一个不同的类 —— 症状是「运行时按 A 类的字段读、判据按
        B 类的字段报警告」，于是合法键被报成「写了没人读」。
        """
        import dataclasses

        from augmentor import config as config_module
        from augmentor.config import AppConfig

        seen = {}
        real = config_module._load_section
        monkeypatch = pytest.MonkeyPatch()
        monkeypatch.setattr(
            config_module, "_load_section",
            lambda raw, key, cls: (seen.__setitem__(key, cls),
                                   real(raw, key, cls))[1])
        try:
            config_module.load_config(self._write(tmp_path, self._sections()))
        finally:
            monkeypatch.undo()

        assert set(seen) == set(self._sections()), sorted(
            set(seen) ^ set(self._sections()))
        wrong = []
        for key, cls in seen.items():
            declared = type(getattr(AppConfig(), key))
            if cls is not declared:
                wrong.append((key, cls.__name__, declared.__name()))
            missing = {f.name for f in dataclasses.fields(cls)
                       if f.init and f.default is dataclasses.MISSING
                       and f.default_factory is dataclasses.MISSING}
            assert not missing, (key, sorted(missing))
        assert wrong == []

    def test_the_new_wait_knobs_default_to_the_documented_pair(self, tmp_path):
        """出厂默认就是文档承诺的那两个数：300 s / 不抖，且不许重抄数字"""
        from augmentor import MAX_RETRY_AFTER
        from augmentor.config import AugmentationConfig, load_config

        loaded = load_config(self._write(tmp_path, ["augmentation"]))
        assert (loaded.augmentation.max_retry_wait,
                loaded.augmentation.retry_jitter) == (MAX_RETRY_AFTER, 0.0)
        assert (AugmentationConfig().max_retry_wait,
                AugmentationConfig().retry_jitter) == (MAX_RETRY_AFTER, 0.0)


class TestWebCorsKeysReachTheConfig:
    """A78 的结案用例：`web.cors_origins` / `cors_credentials` 写了就必须读到

    L49 之前这两个键不在 `load_config` 的映射表里，`_load_section` 按表取键 ⇒
    用户在 YAML 里收紧跨源，读回来永远是出厂值，而**同一份文件里的 `port` 正常生效**
    （所以症状不是「配置文件没读到」，是「这一节只有这两个键没人读」）。L50 把两键
    接进表里，本类钉住「接上了」这件事本身；A123 又把整张表删了 ⇒ 加载器改按
    `dataclasses.fields()` 取键，「漏一两个键」这种写法在结构上不复存在。默认值口径
    与中间件实际行为另见 `tests/integration/test_api_security.py::TestShippedCorsDefault`。
    """

    @staticmethod
    def _write(tmp_path, web, name="cors.yaml"):
        import yaml

        path = tmp_path / name
        with open(path, "w", encoding="utf-8") as handle:
            yaml.safe_dump({"models": {"default": "ernie"}, "web": web},
                           handle, allow_unicode=True)
        return str(path)

    def test_tightened_values_are_read_back(self, tmp_path):
        from augmentor.config import load_config

        loaded = load_config(self._write(
            tmp_path, {"port": 9999,
                       "cors_origins": ["https://only-me.example"],
                       "cors_credentials": False}))
        assert loaded.web.cors_origins == ["https://only-me.example"]
        assert loaded.web.cors_credentials is False
        # 同节邻居照旧生效（A78 的症状只在被漏掉的那几个键上）
        assert loaded.web.port == 9999

    def test_an_explicit_wildcard_still_has_to_be_written_out(self, tmp_path):
        """`["*"]` 不再是默认值，想要它就得显式写 —— 这条是收紧的可见边界"""
        from augmentor.config import load_config

        loaded = load_config(self._write(
            tmp_path, {"cors_origins": ["*"], "cors_credentials": True}))
        assert (loaded.web.cors_origins, loaded.web.cors_credentials) == (["*"], True)

    def test_one_key_does_not_disturb_the_other(self, tmp_path):
        """只写 origins 时 credentials 取表里的默认，而不是被一起吞掉"""
        from augmentor.config import load_config

        loaded = load_config(self._write(
            tmp_path, {"cors_origins": ["https://only-me.example"]}))
        assert loaded.web.cors_origins == ["https://only-me.example"]
        assert loaded.web.cors_credentials is False

    def test_list_defaults_are_not_shared_between_loads(self, tmp_path):
        """两份配置各拿自己的列表对象

        这条守的是「可变默认值被共享」那一类，而它的来源已经换过一次：改前的映射表里
        `'cors_origins': []` / `'data_roots': ['data']` 是函数体内的字面量（提到模块级
        就会变成跨配置实例共享的列表）；A123 删表之后回落值只剩该节 dataclass 的
        `default_factory` 一个来源。工厂造出共享列表的通路只有一条（实测
        `Temp/l77q/probe_dataclass_mutable_default.py`，**Python 语言面**：
        `field(default=[])` 在类定义期就 `ValueError: mutable default …`，而
        `default_factory=lambda: 模块级列表` 会让两份实例共享同一个对象）⇒ 本条盯的就是
        后一种写法，让它当场变红，而不是留下一个要等用户改坏一份配置才暴露的洞。
        """
        from augmentor.config import load_config

        a = load_config(self._write(tmp_path, {}, name="a.yaml"))
        b = load_config(self._write(tmp_path, {}, name="b.yaml"))
        assert a.web.cors_origins == [] and b.web.cors_origins == []
        assert a.web.cors_origins is not b.web.cors_origins
        assert a.web.data_roots is not b.web.data_roots
        a.web.data_roots.append("/tmp")
        assert b.web.data_roots == ["data"]


class TestRuntimeSectionJudgements:
    """`augmentation` / `web` 两节自带运行时判据（L51 / A77 + A80 + A82）

    立项原因是「同一区间在校验器与运行时各抄一遍、还一边漏」：改前实测
    （Temp `l51q/probe1.py` NONCE-A9C41E77）`AugmentationConfig(max_retries=10**6,
    retry_delay=10**6, retry_jitter=50.0)` 无判据构造成功，`WebConfig` 的四个形状
    缺陷同样全部放行。修法是把区间常量搬进 `config.py`、两节各带一个
    `__post_init__`，校验器只能引用不能重打 —— 于是**不经 `validate-config` 的
    写法也认这套契约**。逐值两侧一致由
    `tests/unit/test_config_validator.py::TestRuntimeValidatorParity` 常驻断言，
    本类只管「加载路径」与「SDK 直构」两件事。
    """

    @staticmethod
    def _write(tmp_path, body, name="runtime.yaml"):
        import yaml

        path = tmp_path / name
        with open(path, "w", encoding="utf-8") as handle:
            yaml.safe_dump(body, handle, allow_unicode=True)
        return str(path)

    def test_defaults_satisfy_their_own_verdict(self):
        """出厂默认必须过自己那道判据，否则 `load_config(None)` 直接起不来

        `_load_section` 对整节不在场的输入是直接 `config_class()`（A123 之后没有第二份
        默认值可绕），所以默认值一旦落在区间外，症状是没有配置文件的部署完全无法启动。
        """
        from augmentor.config import AugmentationConfig, WebConfig, load_config

        assert AugmentationConfig().auto_save_interval == 10
        assert WebConfig().cors_origins == []
        loaded = load_config(None)
        assert loaded.web.data_roots == ["data"]
        assert loaded.augmentation.max_retries == 3

    @pytest.mark.parametrize("body,expect", [
        ({"web": {"data_roots": "data"}}, "web.data_roots"),
        ({"web": {"cors_origins": "https://api.corp.example"}}, "web.cors_origins"),
        ({"web": {"data_roots": [None]}}, "web.data_roots"),
        ({"web": {"rate_limit_window_seconds": float("nan")}},
         "web.rate_limit_window_seconds"),
        ({"web": {"port": 0}}, "web.port"),
        ({"web": {"port": None}}, "web.port"),
        ({"augmentation": {"auto_save_interval": 0}},
         "augmentation.auto_save_interval"),
        ({"augmentation": {"max_retries": 10 ** 6}}, "augmentation.max_retries"),
    ])
    def test_load_config_refuses_a_bad_shape(self, tmp_path, body, expect):
        """加载时就拒，而且报错指名是哪个键

        改前这些写法全部静默通过：`data_roots: data` 拆成 `d/a/t/a` 四个根、
        `cors_origins` 标量在 starlette 里退化成**子串**匹配（放行 `https://api`），
        `data_roots=[None]` 造出一个名叫 `None` 的白名单根。
        """
        from augmentor.config import load_config
        from augmentor.exceptions import DataValidationError

        with pytest.raises(DataValidationError) as exc:
            load_config(self._write(tmp_path, body))
        assert expect in str(exc.value)

    @pytest.mark.parametrize("kwargs", [
        {"cors_origins": "https://api.corp.example"},
        {"cors_origins": [None]},
        {"data_roots": "data"},
        {"port": 70000},
        {"host": ""},
        {"rate_limit_max_requests": -1},
    ])
    def test_sdk_construction_is_covered_too(self, kwargs):
        """这是本轮的破坏性变更：SDK 直构 `WebConfig` 从此也要认区间

        影响面（实测两解释器全量）：仓内构造点全部走默认值或合法值，无一处被新判据
        拦下；对外是行为变更 —— 以前能构造成功的越界配置现在抛 `DataValidationError`
        （`ValueError` 子类）。
        """
        from augmentor.config import WebConfig
        from augmentor.exceptions import DataValidationError

        with pytest.raises(DataValidationError):
            WebConfig(**kwargs)

    def test_mutation_after_construction_is_not_covered(self):
        """诚实记下判据的边界：`__post_init__` 只在构造时跑一次

        本条不是在夸这个行为，是在钉它：如果有人以为改字段也会被拦，就会写出依赖
        这个假设的代码。运行时的第二道防线在消费点（`models/base.py` 与各调用点的
        `require_*`），不在配置对象里。
        """
        from augmentor.config import WebConfig

        cfg = WebConfig()
        cfg.port = 70000
        cfg.data_roots = "data"
        assert cfg.port == 70000 and cfg.data_roots == "data"

    def test_null_valued_key_is_rejected_with_its_own_message(self, tmp_path):
        """`port: `（写了键没给值）走的是 null 判据，不是类型判据

        两条报错的修法不同：前者要补值，后者要改写法。实测 YAML `port: ` 读回来
        就是 `None`，改前被原样放进字段，下游 `Path(str(p))` 会把它变成名叫 `None`
        的目录。
        """
        from augmentor.config import load_config
        from augmentor.exceptions import DataValidationError

        with pytest.raises(DataValidationError, match="不能是 null"):
            load_config(self._write(tmp_path, {"web": {"port": None}}))

