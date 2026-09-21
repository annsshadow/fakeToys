"""CLI `analysis` 组命令（由 cli.py 拆出，逻辑未改）"""

from ..io import _load_items, _print
from pathlib import Path
import json
from augmentor import AugmentorPipeline


# ============ 分析 ============
def run_analyze(args, config):
    """`analyze` 子命令"""
    pipeline = AugmentorPipeline(config)
    result = pipeline.analyze_dataset(args.input)
    _print(result)


# ============ 数据分析 ============
def run_analyze_data(args, config):
    """`analyze-data` 子命令"""
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


# ============ 数据集统计 ============
def run_stats(args, config):
    """`stats` 子命令"""
    from augmentor.dataset_ops import DatasetOperations

    items = _load_items(args.input)
    ops = DatasetOperations()
    stats = ops.get_statistics(items)
    _print(stats)


# ============ 增强统计 ============
def run_stats_enhanced(args, config):
    """`stats-enhanced` 子命令"""
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


# ============ 可视化 ============
def run_visualize(args, config):
    """`visualize` 子命令"""
    pipeline = AugmentorPipeline(config)
    results = pipeline.visualize_dataset(args.input, args.output_dir)
    _print(results)


# ============ 数据可视化 ============
def run_visualize_data(args, config):
    """`visualize-data` 子命令"""
    from augmentor.visualize_enhanced import visualize_dataset

    items = _load_items(args.input)
    result = visualize_dataset(items, args.output, args.format)

    print(result)


# ============ 数据集对比 ============
def run_compare(args, config):
    """`compare` 子命令"""
    from augmentor.comparison import compare_datasets

    result = compare_datasets(
        args.dataset_a,
        args.dataset_b,
        name_a=args.name_a,
        name_b=args.name_b,
        output_path=args.output
    )
    print(result.summary)


# ============ 增强比较 ============
def run_compare_enhanced(args, config):
    """`compare-enhanced` 子命令"""
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
