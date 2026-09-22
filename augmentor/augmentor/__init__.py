"""AI 训练数据增强工具包"""

import warnings
from typing import Any, Dict

__version__ = "2.2.0"

from .config import AppConfig, load_config
from .models import create_model_backend, ModelBackend
from .pipeline import AugmentorPipeline
from .quality import QualityScorer
from .dedup import Deduplicator
from .export import Exporter
from .preview import PreviewGenerator, ExportPreview
from .report import ReportGenerator, QualityReport as PipelineQualityReport
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
from .validation import DatasetValidator, DataSanitizer, ValidationResult as DataValidationResult, ValidationSeverity
from .converter import DatasetConverter, DataFormat, convert_dataset, convert_file, get_supported_formats
from .indexer import DatasetIndexer, DatasetView, QueryResult, create_indexer, create_view
from .cache import MemoryCache, DiskCache, CachedProcessor, cached, create_memory_cache, create_disk_cache
from .config_validator import ConfigValidator, ValidationResult as ConfigValidationResult, validate_config_file, validate_config
from .analytics import DatasetAnalyzer, AnalysisReport, DataInsight, analyze_dataset, get_dataset_insights
from .cleaner import DatasetCleaner, TextNormalizer, CleaningResult, clean_dataset, normalize_text, extract_keywords
from .export_enhanced import EnhancedExporter, ExportOptions, ExportFormat, export_dataset, get_supported_formats as get_supported_export_formats
from .quality_report import QualityReporter, QualityReport as DatasetQualityReport, QualityMetric, generate_quality_report, save_quality_report
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
from .privacy import PiiSanitizer, SanitizeReport, sanitize_pii
from .leakage import LeakageDetector, LeakageReport, detect_leakage
from .json_extract import ExtractResult, extract_json, extract_json_list
from .retry import RetryStats, with_retries, compute_delay, should_retry
from .audit import DatasetAuditor, AuditReport, audit_dataset
from .prompts import PromptTemplate, PromptRegistry, TemplateError, create_template
from .schema import DatasetSchema, FieldRule, SchemaValidationResult, validate_schema
from .diagnostics import DiagnosticsReport, check_dependencies, require_dependency, summary_line
from .report_md import (
    sanitize_report_to_markdown,
    leakage_report_to_markdown,
    audit_report_to_markdown,
    generic_report_to_markdown,
)
from .profiling import DataProfiler, ProfilingConfig, profile_dataset
from .feature_detect import FeatureDetector, FeatureInfo, detect_features
from .auto_config import AutoConfig, AutoConfigRecommendation, auto_recommend
from .data_pipeline import DataPipeline, StageResult
from .aggregator import DataAggregator, AggregationResult, aggregate_datasets
from .impact import ImpactEvaluator, AugmentationImpact, evaluate_augmentation
from .data_splitter import DataSplitter, split_dataset
from .quality_gate import QualityGate, GateRule, GateReport, build_default_gate
from .health_score import DatasetHealthScore
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
    "PipelineQualityReport",
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
    "DataValidationResult",
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
    "ConfigValidationResult",
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
    "DatasetQualityReport",
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
    "PiiSanitizer",
    "SanitizeReport",
    "sanitize_pii",
    "LeakageDetector",
    "LeakageReport",
    "detect_leakage",
    "ExtractResult",
    "extract_json",
    "extract_json_list",
    "RetryStats",
    "with_retries",
    "compute_delay",
    "should_retry",
    "DatasetAuditor",
    "AuditReport",
    "audit_dataset",
    "PromptTemplate",
    "PromptRegistry",
    "TemplateError",
    "create_template",
    "DatasetSchema",
    "FieldRule",
    "SchemaValidationResult",
    "validate_schema",
    "DiagnosticsReport",
    "check_dependencies",
    "require_dependency",
    "summary_line",
    "sanitize_report_to_markdown",
    "leakage_report_to_markdown",
    "audit_report_to_markdown",
    "generic_report_to_markdown",
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
    "DataSplitter",
    "split_dataset",
    "QualityGate",
    "GateRule",
    "GateReport",
    "build_default_gate",
    "DatasetHealthScore",
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


# ============ 历史歧义名兼容层 ============
#
# 早期版本中 `QualityReport` 与 `ValidationResult` 各被从两个模块导入到包级别，
# 后者静默覆盖前者，导致 `augmentor.QualityReport` 指向的类与
# `ReportGenerator.generate()` 实际返回的类不是同一个（`isinstance` 判定为 False）。
#
# 现已改为导出语义明确的独立名字：
#   PipelineQualityReport   ← augmentor.report.QualityReport（ReportGenerator 产出）
#   DatasetQualityReport    ← augmentor.quality_report.QualityReport（QualityReporter 产出）
#   DataValidationResult    ← augmentor.validation.ValidationResult（数据集校验）
#   ConfigValidationResult  ← augmentor.config_validator.ValidationResult（配置校验）
#
# 下面按「修复前的实际解析结果」保留旧名一个版本，并给出弃用警告。

_LEGACY_ALIASES: Dict[str, str] = {
    "QualityReport": "DatasetQualityReport",
    "ValidationResult": "ConfigValidationResult",
}

_LEGACY_ORIGIN: Dict[str, str] = {
    "QualityReport": "quality_report",
    "ValidationResult": "config_validator",
}


def __getattr__(name: str) -> Any:
    """按需解析历史歧义名并发出弃用警告（PEP 562）"""
    target = _LEGACY_ALIASES.get(name)
    if target is None:
        raise AttributeError(f"module {__name__!r} has no attribute {name!r}")
    warnings.warn(
        f"augmentor.{name} 存在歧义（历史上由两个模块的同名类互相覆盖），"
        f"已弃用并将在下一大版本移除；请改用 augmentor.{target}，"
        f"或直接 from augmentor.{_LEGACY_ORIGIN[name]} import {name}。",
        DeprecationWarning,
        stacklevel=2,
    )
    return globals()[target]

