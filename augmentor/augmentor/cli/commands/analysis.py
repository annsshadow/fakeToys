# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""CLI `analysis` 组命令"""

import json
from pathlib import Path

from ..io import _load_items, _print
from augmentor import AugmentorPipeline


# ============ 分析（合并原 analyze-data）============
def run_analyze(args, config):
    """`analyze` 子命令

    默认走 `pipeline.analyze_dataset`，输出 pipeline 的分析字典。
    `--enhanced` 走 `analytics.analyze_dataset`，输出带洞察与建议的分析报告。
    """
    if args.enhanced:
        from augmentor.analytics import analyze_dataset

        items = _load_items(args.input)
        report = analyze_dataset(items, args.fields)

        print(f"数据集大小: {report.dataset_size}")
        print(f"质量分数: {report.quality_score:.2f}")
        print(f"多样性分数: {report.diversity_score:.2f}")
        print(f"完整性分数: {report.completeness_score:.2f}")

        if report.insights:
            print("\n洞察:")
            for insight in report.insights:
                print(f"  [{insight.severity}] {insight.title}: {insight.description}")
                print(f"    建议: {insight.recommendation}")

        if args.output:
            output_path = Path(args.output)
            output_path.parent.mkdir(parents=True, exist_ok=True)
            with open(output_path, 'w', encoding='utf-8') as f:
                json.dump(report.to_dict(), f, ensure_ascii=False, indent=2)
            print(f"\n分析报告已保存到 {args.output}")
        return

    pipeline = AugmentorPipeline(config)
    result = pipeline.analyze_dataset(args.input)
    _print(result)


# ============ 数据集统计（合并原 stats-enhanced）============
def run_stats(args, config):
    """`stats` 子命令

    默认走 `dataset_ops.DatasetOperations.get_statistics`（概要字典）。
    `--enhanced` 走 `statistics.calculate_statistics`（逐字段填充率/长度/唯一值）。
    """
    if args.enhanced:
        from augmentor.statistics import calculate_statistics

        items = _load_items(args.input)
        dataset_name = Path(args.input).stem
        stats = calculate_statistics(items, dataset_name, args.fields)

        print(f"数据集: {stats.dataset_name}")
        print(f"数据总量: {stats.total_items}")
        print(f"字段数量: {len(stats.field_statistics)}")

        print("\n字段统计:")
        for field_name, field_stats in stats.field_statistics.items():
            fill_rate = field_stats.filled_count / field_stats.total_count if field_stats.total_count > 0 else 0
            print(f"  {field_name}:")
            print(f"    填充率: {fill_rate:.1%}")
            print(f"    平均长度: {field_stats.avg_length:.1f}")
            print(f"    唯一值: {field_stats.unique_count}")

        if stats.quality_metrics:
            print("\n质量指标:")
            for metric_name, metric_value in stats.quality_metrics.items():
                print(f"  {metric_name}: {metric_value:.2f}")

        if args.output:
            output_path = Path(args.output)
            output_path.parent.mkdir(parents=True, exist_ok=True)
            with open(output_path, 'w', encoding='utf-8') as f:
                json.dump(stats.to_dict(), f, ensure_ascii=False, indent=2)
            print(f"\n统计报告已保存到 {args.output}")
        return

    from augmentor.dataset_ops import DatasetOperations

    items = _load_items(args.input)
    ops = DatasetOperations()
    stats = ops.get_statistics(items)
    _print(stats)


# ============ 可视化（合并原 visualize-data）============
def run_visualize(args, config):
    """`visualize` 子命令

    默认走 `pipeline.visualize_dataset`（落盘到 `--output-dir`）。
    `--enhanced` 走 `visualize_enhanced.visualize_dataset`（文本/JSON 报告）。
    """
    if args.enhanced:
        from augmentor.visualize_enhanced import visualize_dataset

        items = _load_items(args.input)
        result = visualize_dataset(items, args.output, args.format)
        print(result)
        return

    pipeline = AugmentorPipeline(config)
    results = pipeline.visualize_dataset(args.input, args.output_dir)
    _print(results)


# ============ 数据集对比（合并原 compare-enhanced）============
def run_compare(args, config):
    """`compare` 子命令

    默认走 `comparison.compare_datasets`：按**文件路径**对比，输出摘要。
    `--enhanced` 走 `compare_enhanced.compare_datasets_enhanced`：先在内存里读入
    两个数据集，输出字段级指标（相似度 / 仅在 A / 仅在 B）与建议。
    """
    if args.enhanced:
        from augmentor.compare_enhanced import compare_datasets_enhanced

        items_a = _load_items(args.dataset_a)
        items_b = _load_items(args.dataset_b)

        name_a = Path(args.dataset_a).stem
        name_b = Path(args.dataset_b).stem

        result = compare_datasets_enhanced(items_a, items_b, name_a, name_b, args.key_fields)

        print(f"数据集比较结果:")
        print(f"  数据集A ({name_a}): {result.metrics.size_a} 条")
        print(f"  数据集B ({name_b}): {result.metrics.size_b} 条")
        print(f"  相似度: {result.metrics.similarity_score:.2%}")
        print(f"  共同数据: {result.metrics.common_items} 条")
        print(f"  仅在A中: {result.metrics.unique_a} 条")
        print(f"  仅在B中: {result.metrics.unique_b} 条")

        if result.recommendations:
            print("\n建议:")
            for rec in result.recommendations:
                print(f"  - {rec}")

        if args.output:
            output_path = Path(args.output)
            output_path.parent.mkdir(parents=True, exist_ok=True)
            with open(output_path, 'w', encoding='utf-8') as f:
                json.dump(result.to_dict(), f, ensure_ascii=False, indent=2)
            print(f"\n比较报告已保存到 {args.output}")
        return

    from augmentor.comparison import compare_datasets

    result = compare_datasets(
        args.dataset_a,
        args.dataset_b,
        name_a=args.name_a,
        name_b=args.name_b,
        output_path=args.output
    )
    print(result.summary)
