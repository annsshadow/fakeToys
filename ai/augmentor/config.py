"""配置管理模块"""

import os
import yaml
from dataclasses import dataclass, field
from typing import Optional
from pathlib import Path


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
            if isinstance(value, str) and value.startswith('${') and value.endswith('}'):
                env_var = value[2:-1]
                return os.environ.get(env_var, '')
            return value
        
        # 优化配置加载：缓存解析结果（避免重复解析相同配置）
        config_cache_key = config_path or "default"
        if hasattr(config, '_load_cache') and config_cache_key in config._load_cache:
            logger.debug("使用缓存配置（性能优化）")
            return config._load_cache[config_cache_key]
        
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
        
        # 加载其他配置
        if 'augmentation' in raw_config:
            conf = raw_config['augmentation']
            config.augmentation = AugmentationConfig(
                variants_per_seed=conf.get('variants_per_seed', 5),
                num_threads=conf.get('num_threads', 40),
                auto_save_interval=conf.get('auto_save_interval', 10),
                max_retries=conf.get('max_retries', 3),
                retry_delay=conf.get('retry_delay', 1.0)
            )
        
        if 'quality' in raw_config:
            conf = raw_config['quality']
            config.quality = QualityConfig(
                enabled=conf.get('enabled', True),
                threshold=conf.get('threshold', 0.6),
                weights=conf.get('weights', [0.3, 0.4, 0.3])
            )
        
        if 'dedup' in raw_config:
            conf = raw_config['dedup']
            config.dedup = DedupConfig(
                enabled=conf.get('enabled', True),
                threshold=conf.get('threshold', 0.9)
            )
        
        if 'export' in raw_config:
            conf = raw_config['export']
            config.export = ExportConfig(
                default_format=conf.get('default_format', 'jsonl'),
                formats=conf.get('formats', ['jsonl', 'llama_factory', 'alpaca', 'sharegpt', 'chatml'])
            )
        
        if 'context' in raw_config:
            conf = raw_config['context']
            config.context = ContextConfig(
                enabled=conf.get('enabled', False),
                num_turns=conf.get('num_turns', 3)
            )
        
        if 'versioning' in raw_config:
            conf = raw_config['versioning']
            config.versioning = VersioningConfig(
                enabled=conf.get('enabled', True),
                storage_dir=conf.get('storage_dir', 'data/versions'),
                auto_snapshot=conf.get('auto_snapshot', True)
            )
        
        if 'sampler' in raw_config:
            conf = raw_config['sampler']
            config.sampler = SamplerConfig(
                enabled=conf.get('enabled', False),
                dimensions=conf.get('dimensions', ['topic', 'question_type', 'length', 'complexity'])
            )
        
        if 'expander' in raw_config:
            conf = raw_config['expander']
            config.expander = ExpanderConfig(
                enabled=conf.get('enabled', False),
                strategies=conf.get('strategies', ['similar', 'related', 'scenario'])
            )
        
        if 'tracker' in raw_config:
            conf = raw_config['tracker']
            config.tracker = TrackerConfig(
                enabled=conf.get('enabled', False),
                metrics=conf.get('metrics', ['train_loss', 'eval_accuracy', 'perplexity'])
            )
        
        if 'visualization' in raw_config:
            conf = raw_config['visualization']
            config.visualization = VisualizationConfig(
                enabled=conf.get('enabled', True),
                types=conf.get('types', ['wordcloud', 'length_distribution', 'topic_cluster', 'timeline', 'quality_distribution'])
            )
        
        if 'multilingual' in raw_config:
            conf = raw_config['multilingual']
            config.multilingual = MultilingualConfig(
                enabled=conf.get('enabled', False),
                default_target_lang=conf.get('default_target_lang', 'en'),
                supported_langs=conf.get('supported_langs', ['zh', 'en']),
                translate_batch_size=conf.get('translate_batch_size', 10)
            )
        
        if 'rag' in raw_config:
            conf = raw_config['rag']
            config.rag = RAGConfig(
                enabled=conf.get('enabled', False),
                default_format=conf.get('default_format', 'llamaindex'),
                chunk_size=conf.get('chunk_size', 512),
                chunk_overlap=conf.get('chunk_overlap', 64)
            )
        
        if 'evaluation' in raw_config:
            conf = raw_config['evaluation']
            config.evaluation = EvaluationConfig(
                enabled=conf.get('enabled', False),
                metrics=conf.get('metrics', ['bleu', 'rouge_l', 'similarity']),
                reference_field=conf.get('reference_field', 'output')
            )
        
        if 'vector' in raw_config:
            conf = raw_config['vector']
            config.vector = VectorConfig(
                enabled=conf.get('enabled', False),
                backend=conf.get('backend', 'faiss'),
                dimension=conf.get('dimension', 384),
                storage_dir=conf.get('storage_dir', 'data/vectors'),
                collection=conf.get('collection', 'default')
            )
        
        if 'multimodal' in raw_config:
            conf = raw_config['multimodal']
            config.multimodal = MultimodalConfig(
                enabled=conf.get('enabled', False),
                image_extensions=conf.get('image_extensions', ['.jpg', '.jpeg', '.png', '.bmp', '.webp']),
                audio_extensions=conf.get('audio_extensions', ['.wav', '.mp3', '.flac', '.ogg', '.m4a'])
            )
        
        if 'benchmark' in raw_config:
            conf = raw_config['benchmark']
            config.benchmark = BenchmarkConfig(
                enabled=conf.get('enabled', False),
                baseline_file=conf.get('baseline_file', 'data/benchmark_baseline.json'),
                metrics=conf.get('metrics', ['pass_rate', 'avg_total_score', 'diversity', 'duplication_rate'])
            )
        
        if 'active_learning' in raw_config:
            conf = raw_config['active_learning']
            config.active_learning = ActiveLearningConfig(
                enabled=conf.get('enabled', False),
                strategy=conf.get('strategy', 'uncertainty'),
                batch_size=conf.get('batch_size', 50),
                max_iterations=conf.get('max_iterations', 10)
            )
        
        if 'frameworks' in raw_config:
            conf = raw_config['frameworks']
            config.frameworks = FrameworkConfig(
                enabled=conf.get('enabled', False),
                frameworks=conf.get('frameworks', ['langchain', 'llamaindex'])
            )
        
        if 'web' in raw_config:
            conf = raw_config['web']
            config.web = WebConfig(
                port=conf.get('port', 8000),
                host=conf.get('host', '0.0.0.0'),
                static_dir=conf.get('static_dir', 'web/dist')
            )
        
        if 'logging' in raw_config:
            conf = raw_config['logging']
            config.logging = LoggingConfig(
                level=conf.get('level', 'INFO'),
                file=conf.get('file', 'app.log'),
                format=conf.get('format', '%(asctime)s - %(levelname)s - %(message)s')
            )
    
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
        raise ValueError(f"模型 '{name}' 未配置。可用模型: {list(config.models.keys())}")
    return config.models[name]


def save_config(config: AppConfig, config_path: str = "config.yaml") -> None:
    """将配置保存回 YAML 文件（仅持久化非敏感运行时字段；密钥保留占位符）

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

    # 保留 models 下的 api_key/secret_key 为环境变量占位符，避免密钥落盘
    for name, model_conf in data.get("models", {}).items():
        if name == "default":
            continue
        model_conf.pop("api_key", None)
        model_conf.pop("secret_key", None)

    with open(config_path, "w", encoding="utf-8") as f:
        yaml.safe_dump(data, f, allow_unicode=True, sort_keys=False)
