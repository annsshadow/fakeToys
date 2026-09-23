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
