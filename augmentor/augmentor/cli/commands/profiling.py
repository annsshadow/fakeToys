# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""CLI `profiling` 组命令（由 cli.py 拆出，逻辑未改）"""

from ..io import _dump_json, _load_items, _print, _save_items
from pathlib import Path


# 数据画像命令
def run_profile(args, config):
    """`profile` 子命令"""
    from augmentor.profiling import DataProfiler

    items = _load_items(args.input)
    profiler = DataProfiler()
    report = profiler.profile(items)
    if args.output:
        profiler.save_profile(items, args.output)
        _print({"saved_to": args.output})
    else:
        _print(report)
    return


# 异常检测命令
def run_outliers(args, config):
    """`outliers` 子命令"""
    from augmentor.outlier import OutlierDetector

    items = _load_items(args.input)
    detector = OutlierDetector(
        method=args.method, threshold=args.threshold, field=args.field
    )
    if args.field == "length":
        items = detector.attach_length_field(items)
    report = detector.detect(items)
    _print(report.to_dict())
    if args.output:
        _dump_json(report.to_dict(), args.output)
    return


# 特征检测命令
def run_features(args, config):
    """`features` 子命令"""
    from augmentor.feature_detect import FeatureDetector

    items = _load_items(args.input)
    detector = FeatureDetector()
    report = detector.detect(items)
    _print(report)
    if args.output:
        _dump_json(report, args.output)
    return


# 自动配置推荐命令
def run_auto_config(args, config):
    """`auto-config` 子命令"""
    from augmentor.auto_config import AutoConfig
    from augmentor.profiling import DataProfiler

    items = _load_items(args.input)
    profile = DataProfiler().profile(items)
    total = len(items)
    recommendation = AutoConfig().recommend(profile, total)
    _print(recommendation.to_dict())
    if args.output:
        _dump_json(recommendation.to_dict(), args.output)
    return


# 数据聚合命令
def run_aggregate(args, config):
    """`aggregate` 子命令"""
    from augmentor.aggregator import DataAggregator

    datasets = {}
    for index, path in enumerate(args.inputs):
        source_name = Path(path).stem if index == 0 else f"source-{index}"
        datasets[source_name] = _load_items(path)
    aggregator = DataAggregator()
    result = aggregator.aggregate(
        datasets,
        args.strategy,
        target_size=args.target_size,
    )
    _save_items(result.aggregated, args.output)
    _print({
        "aggregated_count": result.total_count,
        "source_counts": result.source_counts,
        "removed_duplicates": result.removed_duplicates,
        "conflicts": result.conflicts,
    })
    return
