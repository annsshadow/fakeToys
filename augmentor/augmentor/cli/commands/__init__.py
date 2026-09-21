"""CLI 子命令分发表

键是 argparse 的 `dest="command"` 取值，值是 handler。
每个 handler 的签名统一为 `(args, config) -> None`。
"""

from .profiling import run_profile, run_outliers, run_features, run_auto_config, run_aggregate
from .pipeline import run_augment, run_stream
from .quality import run_quality, run_quality_report, run_clean, run_clean_enhanced, run_annotate, run_rag, run_benchmark
from .export import run_preview, run_export, run_export_enhanced
from .analysis import run_analyze, run_analyze_data, run_stats, run_stats_enhanced, run_visualize, run_visualize_data, run_compare, run_compare_enhanced
from .data_ops import run_merge, run_sample, run_split, run_validate, run_convert, run_validate_config, run_search, run_search_enhanced
from .version import run_version, run_version_control, run_backup
from .security import run_sanitize, run_check_leakage, run_audit
from .ops import run_doctor, run_auto_test, run_monitor, run_dependency, run_migrate


COMMANDS = {
    "profile": run_profile,
    "outliers": run_outliers,
    "features": run_features,
    "auto-config": run_auto_config,
    "aggregate": run_aggregate,
    "augment": run_augment,
    "stream": run_stream,
    "quality": run_quality,
    "quality-report": run_quality_report,
    "clean": run_clean,
    "clean-enhanced": run_clean_enhanced,
    "annotate": run_annotate,
    "rag": run_rag,
    "benchmark": run_benchmark,
    "preview": run_preview,
    "export": run_export,
    "export-enhanced": run_export_enhanced,
    "analyze": run_analyze,
    "analyze-data": run_analyze_data,
    "stats": run_stats,
    "stats-enhanced": run_stats_enhanced,
    "visualize": run_visualize,
    "visualize-data": run_visualize_data,
    "compare": run_compare,
    "compare-enhanced": run_compare_enhanced,
    "merge": run_merge,
    "sample": run_sample,
    "split": run_split,
    "validate": run_validate,
    "convert": run_convert,
    "validate-config": run_validate_config,
    "search": run_search,
    "search-enhanced": run_search_enhanced,
    "version": run_version,
    "version-control": run_version_control,
    "backup": run_backup,
    "sanitize": run_sanitize,
    "check-leakage": run_check_leakage,
    "audit": run_audit,
    "doctor": run_doctor,
    "auto-test": run_auto_test,
    "monitor": run_monitor,
    "dependency": run_dependency,
    "migrate": run_migrate,
}


__all__ = ["COMMANDS"]
