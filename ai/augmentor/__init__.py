"""AI 训练数据增强工具包"""

__version__ = "2.1.0"

from .config import AppConfig, load_config
from .models import create_model_backend, ModelBackend
from .pipeline import AugmentorPipeline
from .quality import QualityScorer
from .dedup import Deduplicator
from .export import Exporter
from .preview import PreviewGenerator, ExportPreview
from .report import ReportGenerator, QualityReport
from .context import ContextAugmentor
from .checkpoint import CheckpointManager
from .versioning import VersionManager
from .sampler import ActiveSampler
from .active_learning import ActiveLearningLoop
from .expander import DomainExpander
from .tracker import ExperimentTracker
from .visualizer import DataVisualizer
from .benchmark import QualityBenchmark
from .multilingual import MultilingualSupport
from .rag import RAGFormatter
from .evaluation import ModelEvaluator
from .streaming import StreamReader, StreamWriter, StreamProcessor, StreamAugmentor
from .comparison import DatasetComparator, ComparisonResult, compare_datasets
from .dataset_ops import DatasetOperations, MergeConfig, SampleConfig, SplitConfig
from .validation import DatasetValidator, DataSanitizer, ValidationResult, ValidationSeverity
from .converter import DatasetConverter, DataFormat, convert_dataset, convert_file, get_supported_formats
from .indexer import DatasetIndexer, DatasetView, QueryResult, create_indexer, create_view
from .cache import MemoryCache, DiskCache, CachedProcessor, cached, create_memory_cache, create_disk_cache
from .config_validator import ConfigValidator, ValidationResult, validate_config_file, validate_config
from .analytics import DatasetAnalyzer, AnalysisReport, DataInsight, analyze_dataset, get_dataset_insights
from .cleaner import DatasetCleaner, TextNormalizer, CleaningResult, clean_dataset, normalize_text, extract_keywords
from .exceptions import (
    AugmentorError,
    ConfigError,
    ModelError,
    ModelNotConfiguredError,
    ModelInitError,
    ModelGenerateError,
    DataError,
    DataLoadError,
    DataFormatError,
    DataValidationError,
    QualityError,
    DedupError,
    ExportError,
    UnsupportedFormatError,
    CheckpointError,
    VersionError,
    PipelineError
)

__all__ = [
    "AppConfig",
    "load_config",
    "create_model_backend",
    "ModelBackend",
    "AugmentorPipeline",
    "QualityScorer",
    "Deduplicator",
    "Exporter",
    "PreviewGenerator",
    "ExportPreview",
    "ReportGenerator",
    "QualityReport",
    "ContextAugmentor",
    "CheckpointManager",
    "VersionManager",
    "ActiveSampler",
    "ActiveLearningLoop",
    "DomainExpander",
    "ExperimentTracker",
    "DataVisualizer",
    "QualityBenchmark",
    "MultilingualSupport",
    "RAGFormatter",
    "ModelEvaluator",
    "StreamReader",
    "StreamWriter",
    "StreamProcessor",
    "StreamAugmentor",
    "DatasetComparator",
    "ComparisonResult",
    "compare_datasets",
    "DatasetOperations",
    "MergeConfig",
    "SampleConfig",
    "SplitConfig",
    "DatasetValidator",
    "DataSanitizer",
    "ValidationResult",
    "ValidationSeverity",
    "DatasetConverter",
    "DataFormat",
    "convert_dataset",
    "convert_file",
    "get_supported_formats",
    "DatasetIndexer",
    "DatasetView",
    "QueryResult",
    "create_indexer",
    "create_view",
    "MemoryCache",
    "DiskCache",
    "CachedProcessor",
    "cached",
    "create_memory_cache",
    "create_disk_cache",
    "ConfigValidator",
    "ValidationResult",
    "validate_config_file",
    "validate_config",
    "DatasetAnalyzer",
    "AnalysisReport",
    "DataInsight",
    "analyze_dataset",
    "get_dataset_insights",
    "DatasetCleaner",
    "TextNormalizer",
    "CleaningResult",
    "clean_dataset",
    "normalize_text",
    "extract_keywords",
    "AugmentorError",
    "ConfigError",
    "ModelError",
    "ModelNotConfiguredError",
    "ModelInitError",
    "ModelGenerateError",
    "DataError",
    "DataLoadError",
    "DataFormatError",
    "DataValidationError",
    "QualityError",
    "DedupError",
    "ExportError",
    "UnsupportedFormatError",
    "CheckpointError",
    "VersionError",
    "PipelineError"
]
