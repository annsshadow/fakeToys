# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""CLI 子命令分发表

键是 argparse 的 `dest="command"` 取值，值是 handler。
每个 handler 的签名统一为 `(args, config) -> None`。

这里登记的是 **36 个规范命令**。

3.0 起有两项破坏性变更（详见 `augmentor/cli/parser.py` 的模块 docstring）：

1. 8 个旧命令名（`export-enhanced` / `analyze-data` / `visualize-data` /
   `version-control` / `compare-enhanced` / `search-enhanced` / `clean-enhanced` /
   `stats-enhanced`）连同 argparse alias 一并移除，不再注册；
2. `--enhanced` 开关也已移除——同一能力的两种实现各自合并为一套。
"""

from .profiling import run_profile, run_outliers, run_features, run_auto_config, run_aggregate
from .pipeline import run_augment, run_stream
from .quality import run_quality, run_quality_report, run_clean, run_annotate, run_rag, run_benchmark
from .export import run_preview, run_export
from .analysis import run_analyze, run_stats, run_visualize, run_compare
from .data_ops import run_merge, run_sample, run_split, run_validate, run_convert, run_validate_config, run_search
from .version import run_version, run_backup
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
    "annotate": run_annotate,
    "rag": run_rag,
    "benchmark": run_benchmark,
    "preview": run_preview,
    "export": run_export,
    "analyze": run_analyze,
    "stats": run_stats,
    "visualize": run_visualize,
    "compare": run_compare,
    "merge": run_merge,
    "sample": run_sample,
    "split": run_split,
    "validate": run_validate,
    "convert": run_convert,
    "validate-config": run_validate_config,
    "search": run_search,
    "version": run_version,
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
