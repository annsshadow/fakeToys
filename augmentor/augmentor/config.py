# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""配置管理模块"""

import os
import yaml
from dataclasses import dataclass, field
from typing import Any, Dict, Optional
from pathlib import Path
from .exceptions import ConfigError


@dataclass
class ModelConfig:
    """模型配置"""
    type: str
    api_key: Optional[str] = None
    secret_key: Optional[str] = None
    base_url: Optional[str] = None
    model: str = "default"
    temperature: float = 0.99
    top_p: float = 0.95
    max_output_tokens: int = 2048


@dataclass
class AugmentationConfig:
    """增强配置"""
    variants_per_seed: int = 5
    num_threads: int = 40
    auto_save_interval: int = 10
    max_retries: int = 3
    retry_delay: float = 1.0


@dataclass
class QualityConfig:
    """质量评分配置"""
    enabled: bool = True
    threshold: float = 0.6
    weights: list = field(default_factory=lambda: [0.3, 0.4, 0.3])


@dataclass
class DedupConfig:
    """去重配置"""
    enabled: bool = True
    threshold: float = 0.9


@dataclass
class ExportConfig:
    """导出配置"""
    default_format: str = "jsonl"
    formats: list = field(default_factory=lambda: ["jsonl", "llama_factory", "alpaca", "sharegpt", "chatml"])


@dataclass
class ContextConfig:
    """对话上下文配置"""
    enabled: bool = False
    num_turns: int = 3


@dataclass
class VersioningConfig:
    """版本管理配置"""
    enabled: bool = True
    storage_dir: str = "data/versions"
    auto_snapshot: bool = True


@dataclass
class SamplerConfig:
    """主动学习配置"""
    enabled: bool = False
    dimensions: list = field(default_factory=lambda: ["topic", "question_type", "length", "complexity"])


@dataclass
class ExpanderConfig:
    """领域扩展配置"""
    enabled: bool = False
    strategies: list = field(default_factory=lambda: ["similar", "related", "scenario"])


@dataclass
class TrackerConfig:
    """效果追踪配置"""
    enabled: bool = False
    metrics: list = field(default_factory=lambda: ["train_loss", "eval_accuracy", "perplexity"])


@dataclass
class VisualizationConfig:
    """可视化配置"""
    enabled: bool = True
    types: list = field(default_factory=lambda: ["wordcloud", "length_distribution", "topic_cluster", "timeline", "quality_distribution"])


@dataclass
class MultilingualConfig:
    """多语言配置"""
    enabled: bool = False
    default_target_lang: str = "en"
    supported_langs: list = field(default_factory=lambda: ["zh", "en"])
    translate_batch_size: int = 10


@dataclass
class RAGConfig:
    """RAG 训练数据配置"""
    enabled: bool = False
    default_format: str = "llamaindex"
    chunk_size: int = 512
    chunk_overlap: int = 64


@dataclass
class EvaluationConfig:
    """模型评估配置"""
    enabled: bool = False
    metrics: list = field(default_factory=lambda: ["bleu", "rouge_l", "similarity"])
    reference_field: str = "output"


@dataclass
class VectorConfig:
    """向量数据库配置"""
    enabled: bool = False
    backend: str = "faiss"
    dimension: int = 384
    storage_dir: str = "data/vectors"
    collection: str = "default"


@dataclass
class MultimodalConfig:
    """多模态数据配置"""
    enabled: bool = False
    image_extensions: list = field(default_factory=lambda: [".jpg", ".jpeg", ".png", ".bmp", ".webp"])
    audio_extensions: list = field(default_factory=lambda: [".wav", ".mp3", ".flac", ".ogg", ".m4a"])


@dataclass
class BenchmarkConfig:
    """数据质量基准配置"""
    enabled: bool = False
    baseline_file: str = "data/benchmark_baseline.json"
    metrics: list = field(default_factory=lambda: ["pass_rate", "avg_total_score", "diversity", "duplication_rate"])


@dataclass
class ActiveLearningConfig:
    """主动学习循环配置"""
    enabled: bool = False
    strategy: str = "uncertainty"
    batch_size: int = 50
    max_iterations: int = 10


@dataclass
class FrameworkConfig:
    """LLM 框架集成配置"""
    enabled: bool = False
    frameworks: list = field(default_factory=lambda: ["langchain", "llamaindex"])


@dataclass
class WebConfig:
    """Web UI 配置"""
    port: int = 8000
    host: str = "0.0.0.0"
    static_dir: str = "web/dist"
    cors_origins: list = field(default_factory=lambda: ["*"])
    cors_credentials: bool = True
    # REST API 允许访问的目录白名单。客户端传入的文件路径必须落在其中某个根目录内，
    # 否则返回 403。默认 ["."] 即进程工作目录；相对路径按工作目录解析。
    # 环境变量 AUGMENTOR_DATA_ROOTS（os.pathsep 分隔）优先级更高。
    data_roots: list = field(default_factory=lambda: ["."])
    # 滑动窗口限流：窗口内单客户端最大请求数。0 表示关闭限流。
    rate_limit_max_requests: int = 300
    # 限流窗口长度（秒）
    rate_limit_window_seconds: float = 60.0
    # 免限流路径（前缀匹配）
    rate_limit_exempt_paths: list = field(
        default_factory=lambda: ["/api/health", "/docs", "/redoc", "/openapi.json"]
    )


@dataclass
class LoggingConfig:
    """日志配置"""
    level: str = "INFO"
    file: str = "app.log"
    format: str = "%(asctime)s - %(levelname)s - %(message)s"


@dataclass
class AppConfig:
    """应用配置"""
    models: dict = field(default_factory=dict)
    default_model: str = "ernie"
    augmentation: AugmentationConfig = field(default_factory=AugmentationConfig)
    quality: QualityConfig = field(default_factory=QualityConfig)
    dedup: DedupConfig = field(default_factory=DedupConfig)
    export: ExportConfig = field(default_factory=ExportConfig)
    context: ContextConfig = field(default_factory=ContextConfig)
    versioning: VersioningConfig = field(default_factory=VersioningConfig)
    sampler: SamplerConfig = field(default_factory=SamplerConfig)
    expander: ExpanderConfig = field(default_factory=ExpanderConfig)
    tracker: TrackerConfig = field(default_factory=TrackerConfig)
    visualization: VisualizationConfig = field(default_factory=VisualizationConfig)
    multilingual: MultilingualConfig = field(default_factory=MultilingualConfig)
    rag: RAGConfig = field(default_factory=RAGConfig)
    evaluation: EvaluationConfig = field(default_factory=EvaluationConfig)
    vector: VectorConfig = field(default_factory=VectorConfig)
    multimodal: MultimodalConfig = field(default_factory=MultimodalConfig)
    benchmark: BenchmarkConfig = field(default_factory=BenchmarkConfig)
    active_learning: ActiveLearningConfig = field(default_factory=ActiveLearningConfig)
    frameworks: FrameworkConfig = field(default_factory=FrameworkConfig)
    web: WebConfig = field(default_factory=WebConfig)
    logging: LoggingConfig = field(default_factory=LoggingConfig)


def _resolve_env(value: Any) -> Any:
    """解析环境变量占位符
    
    Args:
        value: 配置值，可能是 ${ENV_VAR} 格式的环境变量占位符
    
    Returns:
        解析后的值
    """
    if isinstance(value, str) and value.startswith('${') and value.endswith('}'):
        env_var = value[2:-1]
        return os.environ.get(env_var, '')
    return value


def _load_section(raw_config: Dict, key: str, config_class: type, defaults: Dict) -> Any:
    """加载配置节
    
    Args:
        raw_config: 原始配置字典
        key: 配置节名称
        config_class: 配置类
        defaults: 默认值字典
    
    Returns:
        配置实例
    """
    if key not in raw_config:
        return config_class(**defaults)
    
    conf = raw_config[key]
    kwargs = {}
    for param_name, default_value in defaults.items():
        kwargs[param_name] = conf.get(param_name, default_value)
    
    return config_class(**kwargs)


def load_config(config_path: Optional[str] = None) -> AppConfig:
    """加载配置文件
    
    Args:
        config_path: 配置文件路径，为 None 时使用默认配置
    
    Returns:
        AppConfig 实例
    """
    config = AppConfig()
    
    if config_path and os.path.exists(config_path):
        with open(config_path, 'r', encoding='utf-8') as f:
            raw_config = yaml.safe_load(f)
        
        # 解析环境变量
        def resolve_env(value):
            return _resolve_env(value)
        
        # 加载模型配置
        if 'models' in raw_config:
            config.default_model = raw_config['models'].get('default', 'ernie')
            for name, model_conf in raw_config['models'].items():
                if name == 'default':
                    continue
                config.models[name] = ModelConfig(
                    type=model_conf.get('type', ''),
                    api_key=resolve_env(model_conf.get('api_key', '')),
                    secret_key=resolve_env(model_conf.get('secret_key', '')),
                    base_url=resolve_env(model_conf.get('base_url', '')),
                    model=model_conf.get('model', ''),
                    temperature=model_conf.get('temperature', 0.99),
                    top_p=model_conf.get('top_p', 0.95),
                    max_output_tokens=model_conf.get('max_output_tokens', 2048)
                )
        
        # 使用映射表加载其他配置（减少重复代码）
        config_sections = [
            ('augmentation', AugmentationConfig, {
                'variants_per_seed': 5, 'num_threads': 40, 'auto_save_interval': 10,
                'max_retries': 3, 'retry_delay': 1.0
            }),
            ('quality', QualityConfig, {
                'enabled': True, 'threshold': 0.6, 'weights': [0.3, 0.4, 0.3]
            }),
            ('dedup', DedupConfig, {'enabled': True, 'threshold': 0.9}),
            ('export', ExportConfig, {
                'default_format': 'jsonl',
                'formats': ['jsonl', 'llama_factory', 'alpaca', 'sharegpt', 'chatml']
            }),
            ('context', ContextConfig, {'enabled': False, 'num_turns': 3}),
            ('versioning', VersioningConfig, {
                'enabled': True, 'storage_dir': 'data/versions', 'auto_snapshot': True
            }),
            ('sampler', SamplerConfig, {
                'enabled': False,
                'dimensions': ['topic', 'question_type', 'length', 'complexity']
            }),
            ('expander', ExpanderConfig, {
                'enabled': False, 'strategies': ['similar', 'related', 'scenario']
            }),
            ('tracker', TrackerConfig, {
                'enabled': False,
                'metrics': ['train_loss', 'eval_accuracy', 'perplexity']
            }),
            ('visualization', VisualizationConfig, {
                'enabled': True,
                'types': ['wordcloud', 'length_distribution', 'topic_cluster', 'timeline', 'quality_distribution']
            }),
            ('multilingual', MultilingualConfig, {
                'enabled': False, 'default_target_lang': 'en',
                'supported_langs': ['zh', 'en'], 'translate_batch_size': 10
            }),
            ('rag', RAGConfig, {
                'enabled': False, 'default_format': 'llamaindex',
                'chunk_size': 512, 'chunk_overlap': 64
            }),
            ('evaluation', EvaluationConfig, {
                'enabled': False, 'metrics': ['bleu', 'rouge_l', 'similarity'],
                'reference_field': 'output'
            }),
            ('vector', VectorConfig, {
                'enabled': False, 'backend': 'faiss', 'dimension': 384,
                'storage_dir': 'data/vectors', 'collection': 'default'
            }),
            ('multimodal', MultimodalConfig, {
                'enabled': False,
                'image_extensions': ['.jpg', '.jpeg', '.png', '.bmp', '.webp'],
                'audio_extensions': ['.wav', '.mp3', '.flac', '.ogg', '.m4a']
            }),
            ('benchmark', BenchmarkConfig, {
                'enabled': False, 'baseline_file': 'data/benchmark_baseline.json',
                'metrics': ['pass_rate', 'avg_total_score', 'diversity', 'duplication_rate']
            }),
            ('active_learning', ActiveLearningConfig, {
                'enabled': False, 'strategy': 'uncertainty',
                'batch_size': 50, 'max_iterations': 10
            }),
            ('frameworks', FrameworkConfig, {
                'enabled': False, 'frameworks': ['langchain', 'llamaindex']
            }),
            ('web', WebConfig, {
                'port': 8000, 'host': '0.0.0.0', 'static_dir': 'web/dist',
                'data_roots': ['.'],
                'rate_limit_max_requests': 300,
                'rate_limit_window_seconds': 60.0,
                'rate_limit_exempt_paths': [
                    '/api/health', '/docs', '/redoc', '/openapi.json'
                ],
            }),
            ('logging', LoggingConfig, {
                'level': 'INFO', 'file': 'app.log',
                'format': '%(asctime)s - %(levelname)s - %(message)s'
            }),
        ]
        
        for key, config_class, defaults in config_sections:
            setattr(config, key, _load_section(raw_config, key, config_class, defaults))
    
    return config


def get_model_config(config: AppConfig, model_name: Optional[str] = None) -> ModelConfig:
    """获取模型配置
    
    Args:
        config: 应用配置
        model_name: 模型名称，为 None 时使用默认模型
    
    Returns:
        ModelConfig 实例
    """
    name = model_name or config.default_model
    if name not in config.models:
        raise ConfigError(f"模型 '{name}' 未配置。可用模型: {list(config.models.keys())}")
    return config.models[name]


def save_config(config: AppConfig, config_path: str = "config.yaml") -> None:
    """将配置保存回 YAML 文件（仅持久化非敏感运行时字段；密钥保留占位符）

    有两件事必须保证，否则「保存 → 重新加载」会静默丢配置：

    1. **默认模型要写成 `models.default`**。`load_config` 只从这个键读默认模型
       （`config.default_model = raw_config['models'].get('default', 'ernie')`），
       写成顶层 `default_model` 下次加载时会被忽略并回落到 `ernie`。
    2. **文件里已经存在、而 `AppConfig` 不建模的顶层段落必须原样保留**。
       `ConfigValidator` 会把它们当作配置的一部分，直接整体覆盖等于删掉用户
       手写的段落。

    Args:
        config: 应用配置
        config_path: 配置文件路径
    """
    import dataclasses

    def _to_dict(obj):
        if dataclasses.is_dataclass(obj):
            return {k: _to_dict(v) for k, v in dataclasses.asdict(obj).items()}
        return obj

    data = _to_dict(config)

    # 默认模型并入 models.default：这是 load_config 唯一认的键
    models = data.get("models") or {}
    models["default"] = data.pop("default_model", config.default_model)
    data["models"] = models

    # 保留既有文件中 AppConfig 不建模的顶层键
    existing = {}
    if os.path.exists(config_path):
        try:
            with open(config_path, "r", encoding="utf-8") as f:
                existing = yaml.safe_load(f) or {}
        except (OSError, yaml.YAMLError):
            existing = {}
        if not isinstance(existing, dict):
            existing = {}
        for key, value in existing.items():
            if key not in data:
                data[key] = value

    # 密钥不落盘：沿用文件里原有的 `${ENV_VAR}` 占位符。
    # 早前直接 pop 掉整个键，等于把 `api_key: ${BAIDU_API_KEY}` 这行删了——
    # 重新加载时该模型拿不到密钥，增强能力静默失效。而如果文件里写的是明文
    # 密钥，则一律丢弃（只保留环境变量引用，避免把明文写回磁盘）。
    existing_models = existing.get("models") if isinstance(existing.get("models"), dict) else {}
    for name, model_conf in data.get("models", {}).items():
        if name == "default" or not isinstance(model_conf, dict):
            continue
        previous = existing_models.get(name) if isinstance(existing_models.get(name), dict) else {}
        for secret in ("api_key", "secret_key"):
            placeholder = previous.get(secret, "")
            if isinstance(placeholder, str) and placeholder.startswith("${") and placeholder.endswith("}"):
                model_conf[secret] = placeholder
            else:
                model_conf.pop(secret, None)

    with open(config_path, "w", encoding="utf-8") as f:
        yaml.safe_dump(data, f, allow_unicode=True, sort_keys=False)
