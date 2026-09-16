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
