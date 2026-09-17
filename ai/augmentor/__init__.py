"""AI 训练数据增强工具包"""

__version__ = "2.2.0"

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
from .export_enhanced import EnhancedExporter, ExportOptions, ExportFormat, export_dataset, get_supported_formats as get_supported_export_formats
from .quality_report import QualityReporter, QualityReport, QualityMetric, generate_quality_report, save_quality_report
from .visualize_enhanced import EnhancedVisualizer, VisualizationConfig, visualize_dataset
from .backup import DatasetBackup, BackupInfo, create_backup, restore_backup, list_backups, delete_backup
from .search_enhanced import EnhancedSearcher, SearchResult, SearchFilter, search_dataset, create_searcher
from .statistics import DatasetStatisticsCalculator, DatasetStatistics, FieldStatistics, calculate_statistics, get_field_summary
from .compare_enhanced import EnhancedComparator, EnhancedComparisonResult, ComparisonMetrics, FieldComparison, compare_datasets_enhanced, diff_datasets
from .version_control import DatasetVersionManager, DatasetVersion, create_version, load_version, list_versions
from .auto_test import DatasetTestRunner, TestSuite, TestCase, TestResult, run_dataset_tests, create_test_suite
from .quality_monitor import QualityMonitor, QualityThreshold, QualityAlert, QualitySnapshot, monitor_quality, create_monitor
from .dependency import DependencyManager, Dependency, DatasetInfo, register_dataset, add_dependency, get_dependency_graph
from .migration import DatasetMigrator, MigrationRule, MigrationResult, migrate_dataset, migrate_file
from .outlier import OutlierDetector, OutlierReport, detect_outliers
from .profiling import DataProfiler, ProfilingConfig, profile_dataset
from .feature_detect import FeatureDetector, FeatureInfo, detect_features
from .auto_config import AutoConfig, AutoConfigRecommendation, auto_recommend
from .data_pipeline import DataPipeline, StageResult
from .aggregator import DataAggregator, AggregationResult, aggregate_datasets
from .impact import ImpactEvaluator, AugmentationImpact, evaluate_augmentation
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
    "EnhancedExporter",
    "ExportOptions",
    "ExportFormat",
    "export_dataset",
    "get_supported_export_formats",
    "QualityReporter",
    "QualityReport",
    "QualityMetric",
    "generate_quality_report",
    "save_quality_report",
    "EnhancedVisualizer",
    "VisualizationConfig",
    "visualize_dataset",
    "DatasetBackup",
    "BackupInfo",
    "create_backup",
    "restore_backup",
    "list_backups",
    "delete_backup",
    "EnhancedSearcher",
    "SearchResult",
    "SearchFilter",
    "search_dataset",
    "create_searcher",
    "DatasetStatisticsCalculator",
    "DatasetStatistics",
    "FieldStatistics",
    "calculate_statistics",
    "get_field_summary",
    "EnhancedComparator",
    "EnhancedComparisonResult",
    "ComparisonMetrics",
    "FieldComparison",
    "compare_datasets_enhanced",
    "diff_datasets",
    "DatasetVersionManager",
    "DatasetVersion",
    "create_version",
    "load_version",
    "list_versions",
    "DatasetTestRunner",
    "TestSuite",
    "TestCase",
    "TestResult",
    "run_dataset_tests",
    "create_test_suite",
    "QualityMonitor",
    "QualityThreshold",
    "QualityAlert",
    "QualitySnapshot",
    "monitor_quality",
    "create_monitor",
    "DependencyManager",
    "Dependency",
    "DatasetInfo",
    "register_dataset",
    "add_dependency",
    "get_dependency_graph",
    "DatasetMigrator",
    "MigrationRule",
    "MigrationResult",
    "migrate_dataset",
    "migrate_file",
    "OutlierDetector",
    "OutlierReport",
    "detect_outliers",
    "DataProfiler",
    "ProfilingConfig",
    "profile_dataset",
    "FeatureDetector",
    "FeatureInfo",
    "detect_features",
    "AutoConfig",
    "AutoConfigRecommendation",
    "auto_recommend",
    "DataPipeline",
    "StageResult",
    "DataAggregator",
    "AggregationResult",
    "aggregate_datasets",
    "ImpactEvaluator",
    "AugmentationImpact",
    "evaluate_augmentation",
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
