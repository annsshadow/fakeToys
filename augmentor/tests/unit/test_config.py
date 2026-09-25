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

class TestSectionDefaultsMatchTheMappingTable:
    """`load_config` 里那份映射表必须与 dataclass 逐字段同值、同覆盖（L49 守护）

    为什么值得单独钉：`load_config` 不直接用 dataclass 的默认值，而是把每个字段的默认
    又抄了一遍（`config_sections`）。两处漂移有两种症状，都不报错：
    值漂移（表里 0.6、类里 0.7 ⇒ 改 dataclass 的用户看不到变化）、覆盖漂移（类里新增
    字段忘了进表 ⇒ 配置文件里写这个字段没人读）。L49 接 `max_retry_wait` /
    `retry_jitter` 时实测值漂移 0（20 节 65 字段，Temp `l49_probe2.py`），但那一版探针
    **只比了默认值、没有比字段覆盖**，补上覆盖方向后当场抓到两个漏字段（见
    `KNOWN_UNMAPPED_FIELDS`）。本类把两个方向都变成常驻断言。
    """

    #: 映射表漏掉的字段 = 配置文件里写了也没人读的死旋钮。L49 实测：`api/main.py:119`
    #: 真的在读 `_config.web.cors_origins` / `.cors_credentials`，而映射表没有这两项 ⇒
    #: 无论 `config.yaml` 写什么，CORS 永远是出厂默认（同一份文件里 `port: 9999` 正常
    #: 生效，故不是整节失效）。那是 A78，L50 已修 ⇒ 本清单**当前为空**，且必须为空：
    #: 留一项就等于承认还有一处「配了不生效」。跨源默认值本身的口径见
    #: `tests/integration/test_api_security.py::TestShippedCorsDefault`。
    KNOWN_UNMAPPED_FIELDS: set = set()

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
        """值漂移守护：只写节名不写字段时，读出来的必须与 dataclass 默认实例相等"""
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

    def test_mapping_table_covers_every_field(self, tmp_path):
        """覆盖漂移守护：表里漏一个字段，配置文件里写它就是「写了没人读」

        本条是**棘轮**而不是豁免清单：`KNOWN_UNMAPPED_FIELDS` 必须与实测缺口逐字
        相等，修好一项就得从清单里删一项，新出现漏字段也立刻变红。
        """
        import dataclasses

        from augmentor import config as config_module
        from augmentor.config import AppConfig

        seen = {}
        real = config_module._load_section
        monkeypatch = pytest.MonkeyPatch()
        monkeypatch.setattr(
            config_module, "_load_section",
            lambda raw, key, cls, defaults: (seen.__setitem__(key, set(defaults)),
                                             real(raw, key, cls, defaults))[1])
        try:
            config_module.load_config(self._write(tmp_path, self._sections()))
        finally:
            monkeypatch.undo()

        assert set(seen) == set(self._sections())
        actual = set()
        for key, names in seen.items():
            fields = {f.name for f in dataclasses.fields(
                type(getattr(AppConfig(), key)))}
            assert names <= fields, (key, sorted(names - fields))
            actual |= {(key, name) for name in fields - names}
        assert actual == self.KNOWN_UNMAPPED_FIELDS, (
            sorted(actual ^ self.KNOWN_UNMAPPED_FIELDS))

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
    接进表里，本类钉住「接上了」这件事本身；默认值口径与中间件实际行为另见
    `tests/integration/test_api_security.py::TestShippedCorsDefault`。
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

        映射表里的 `'cors_origins': []` / `'data_roots': ['data']` 现在是函数体内的
        字面量（每次调用新建）。把 `config_sections` 提到模块级的那一刻，它们就会变成
        跨配置实例共享的可变列表 —— 本条让那次提法当场变红，而不是留下一个要等用户
        改坏一份配置才暴露的洞。
        """
        from augmentor.config import load_config

        a = load_config(self._write(tmp_path, {}, name="a.yaml"))
        b = load_config(self._write(tmp_path, {}, name="b.yaml"))
        assert a.web.cors_origins == [] and b.web.cors_origins == []
        assert a.web.cors_origins is not b.web.cors_origins
        assert a.web.data_roots is not b.web.data_roots
        a.web.data_roots.append("/tmp")
        assert b.web.data_roots == ["data"]
