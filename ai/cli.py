"""AI 训练数据增强工具 CLI 入口"""

import argparse
import json
import sys
from pathlib import Path

from augmentor import AugmentorPipeline, load_config


def _load_items(path: str):
    """加载 JSON 数据文件

    Args:
        path: 文件路径

    Returns:
        数据列表
    """
    with open(path, 'r', encoding='utf-8') as f:
        return json.load(f)


def _save_items(items, path: str):
    """保存数据到 JSON 文件

    Args:
        path: 文件路径
    """
    output_path = Path(path)
    output_path.parent.mkdir(parents=True, exist_ok=True)
    with open(output_path, 'w', encoding='utf-8') as f:
        json.dump(items, f, ensure_ascii=False, indent=2)


def _dump_json(data, path: str):
    """将任意可 JSON 序列化对象写入文件

    Args:
        data: 可 JSON 序列化对象
        path: 输出文件路径
    """
    output_path = Path(path)
    output_path.parent.mkdir(parents=True, exist_ok=True)
    with open(output_path, 'w', encoding='utf-8') as f:
        json.dump(data, f, ensure_ascii=False, indent=2, default=str)


def _print(data):
    """打印 JSON 结果

    Args:
        data: 任意可序列化对象
    """
    print(json.dumps(data, ensure_ascii=False, indent=2, default=str))


def build_parser() -> argparse.ArgumentParser:
    """构建命令行解析器

    Returns:
        ArgumentParser 实例
    """
    parser = argparse.ArgumentParser(description="AI 训练数据增强工具")
    parser.add_argument("--config", type=str, default="config.yaml", help="配置文件路径")

    subparsers = parser.add_subparsers(dest="command", help="子命令")

    # 增强命令
    augment_parser = subparsers.add_parser("augment", help="增强数据集")
    augment_parser.add_argument("--input", type=str, required=True, help="输入文件路径")
    augment_parser.add_argument("--output", type=str, required=True, help="输出文件路径")
    augment_parser.add_argument("--no-quality", action="store_true", help="禁用质量检查")
    augment_parser.add_argument("--no-dedup", action="store_true", help="禁用去重")
    augment_parser.add_argument("--no-checkpoint", action="store_true", help="禁用断点续传")

    # 导出命令
    export_parser = subparsers.add_parser("export", help="导出数据集")
    export_parser.add_argument("--input", type=str, required=True, help="输入文件路径")
    export_parser.add_argument("--output-dir", type=str, required=True, help="输出目录")
    export_parser.add_argument("--formats", nargs="+", help="导出格式")

    # 导出预览命令
    preview_parser = subparsers.add_parser("preview", help="预览导出格式转换结果")
    preview_parser.add_argument("--input", type=str, required=True, help="输入文件路径")
    preview_parser.add_argument("--format", type=str, default="jsonl", help="目标格式")
    preview_parser.add_argument("--size", type=int, default=5, help="预览条数")

    # 质量命令
    quality_parser = subparsers.add_parser("quality", help="质量评估、去重与报告")
    quality_parser.add_argument("--input", type=str, required=True, help="输入文件路径")
    quality_parser.add_argument("--output", type=str, help="过滤后数据的输出路径")
    quality_parser.add_argument("--report", type=str, help="质量报告输出路径（.md 或 .json）")

    # 清洗命令
    clean_parser = subparsers.add_parser("clean", help="数据清洗")
    clean_parser.add_argument("--input", type=str, required=True, help="输入文件路径")
    clean_parser.add_argument("--output", type=str, required=True, help="输出文件路径")
    clean_parser.add_argument("--no-url-removal", action="store_true", help="保留 URL")

    # 标注命令
    annotate_parser = subparsers.add_parser("annotate", help="自动标注")
    annotate_parser.add_argument("--input", type=str, required=True, help="输入文件路径")
    annotate_parser.add_argument("--output", type=str, required=True, help="输出文件路径")

    # RAG 格式命令
    rag_parser = subparsers.add_parser("rag", help="转换为 RAG 训练数据格式")
    rag_parser.add_argument("--input", type=str, required=True, help="输入文件路径")
    rag_parser.add_argument("--output", type=str, required=True, help="输出文件路径")
    rag_parser.add_argument("--format", type=str, default="llamaindex",
                            choices=["llamaindex", "langchain", "custom"], help="目标格式")

    # 基准命令
    benchmark_parser = subparsers.add_parser("benchmark", help="数据质量基准")
    benchmark_parser.add_argument("--input", type=str, required=True, help="输入文件路径")
    benchmark_parser.add_argument("--baseline", type=str, help="基准文件路径")
    benchmark_parser.add_argument("--save-baseline", action="store_true", help="将结果保存为基准")
    benchmark_parser.add_argument("--report", type=str, help="报告输出路径")

    # 分析命令
    analyze_parser = subparsers.add_parser("analyze", help="分析数据集")
    analyze_parser.add_argument("--input", type=str, required=True, help="输入文件路径")
    # 可视化命令
    visualize_parser = subparsers.add_parser("visualize", help="可视化数据集")
    visualize_parser.add_argument("--input", type=str, required=True, help="输入文件路径")
    visualize_parser.add_argument("--output-dir", type=str, default="visualizations", help="输出目录")

    # 版本管理命令
    version_parser = subparsers.add_parser("version", help="版本管理")
    version_parser.add_argument("--action",
                                choices=["list", "create", "diff", "rollback", "history"],
                                required=True, help="操作类型")
    version_parser.add_argument("--input", type=str, help="输入文件路径")
    version_parser.add_argument("--version-id", type=str, help="版本 ID")
    version_parser.add_argument("--version-id-2", type=str, help="第二个版本 ID（用于 diff）")

    # 数据集对比命令
    compare_parser = subparsers.add_parser("compare", help="对比两个数据集")
    compare_parser.add_argument("--dataset-a", type=str, required=True, help="数据集A文件路径")
    compare_parser.add_argument("--dataset-b", type=str, required=True, help="数据集B文件路径")
    compare_parser.add_argument("--name-a", type=str, default="Dataset A", help="数据集A的名称")
    compare_parser.add_argument("--name-b", type=str, default="Dataset B", help="数据集B的名称")
    compare_parser.add_argument("--output", type=str, help="对比结果输出路径")

    # 流式处理命令
    stream_parser = subparsers.add_parser("stream", help="流式处理大数据集")
    stream_parser.add_argument("--input", type=str, required=True, help="输入文件路径")
    stream_parser.add_argument("--output", type=str, required=True, help="输出文件路径")
    stream_parser.add_argument("--chunk-size", type=int, default=1000, help="分块大小")
    stream_parser.add_argument("--operation", type=str, required=True,
                               choices=["quality", "dedup", "clean", "export"],
                               help="处理操作")

    # 数据集合并命令
    merge_parser = subparsers.add_parser("merge", help="合并多个数据集")
    merge_parser.add_argument("--inputs", type=str, nargs="+", required=True, help="输入文件路径列表")
    merge_parser.add_argument("--output", type=str, required=True, help="输出文件路径")
    merge_parser.add_argument("--no-dedup", action="store_true", help="禁用去重")

    # 数据集采样命令
    sample_parser = subparsers.add_parser("sample", help="采样数据集")
    sample_parser.add_argument("--input", type=str, required=True, help="输入文件路径")
    sample_parser.add_argument("--output", type=str, required=True, help="输出文件路径")
    sample_parser.add_argument("--size", type=int, help="采样数量")
    sample_parser.add_argument("--ratio", type=float, help="采样比例 (0-1)")
    sample_parser.add_argument("--method", type=str, default="random",
                               choices=["random", "systematic", "stratified"], help="采样方法")
    sample_parser.add_argument("--seed", type=int, help="随机种子")

    # 数据集分割命令
    split_parser = subparsers.add_parser("split", help="分割数据集")
    split_parser.add_argument("--input", type=str, required=True, help="输入文件路径")
    split_parser.add_argument("--output-dir", type=str, required=True, help="输出目录")
    split_parser.add_argument("--train-ratio", type=float, default=0.8, help="训练集比例")
    split_parser.add_argument("--val-ratio", type=float, default=0.1, help="验证集比例")
    split_parser.add_argument("--test-ratio", type=float, default=0.1, help="测试集比例")
    split_parser.add_argument("--seed", type=int, help="随机种子")

    # 数据集统计命令
    stats_parser = subparsers.add_parser("stats", help="数据集统计信息")
    stats_parser.add_argument("--input", type=str, required=True, help="输入文件路径")

    # 数据集验证命令
    validate_parser = subparsers.add_parser("validate", help="验证数据集格式")
    validate_parser.add_argument("--input", type=str, required=True, help="输入文件路径")
    validate_parser.add_argument("--preset", type=str, default="basic",
                                 choices=["basic", "strict", "chat"], help="验证规则预设")
    validate_parser.add_argument("--output", type=str, help="验证结果输出路径")

    # 数据集转换命令
    convert_parser = subparsers.add_parser("convert", help="转换数据集格式")
    convert_parser.add_argument("--input", type=str, required=True, help="输入文件路径")
    convert_parser.add_argument("--output", type=str, required=True, help="输出文件路径")
    convert_parser.add_argument("--format", type=str, required=True,
                                choices=["json", "jsonl", "csv", "alpaca", "sharegpt", 
                                        "chatml", "llama_factory", "vicuna", "belle"],
                                help="目标格式")

    # 数据集搜索命令
    search_parser = subparsers.add_parser("search", help="搜索数据集")
    search_parser.add_argument("--input", type=str, required=True, help="输入文件路径")
    search_parser.add_argument("--query", type=str, required=True, help="搜索查询")
    search_parser.add_argument("--field", type=str, nargs="+", default=["instruction", "output"],
                               help="搜索字段")
    search_parser.add_argument("--method", type=str, default="contains",
                               choices=["exact", "contains", "ngram"], help="搜索方法")
    search_parser.add_argument("--limit", type=int, default=10, help="返回数量")

    # 配置验证命令
    validate_config_parser = subparsers.add_parser("validate-config", help="验证配置文件")
    validate_config_parser.add_argument("--config", type=str, help="配置文件路径")

    # 数据分析命令（合并到 analyze 子命令）
    analyze_parser = subparsers.add_parser("analyze-data", help="数据分析")
    analyze_parser.add_argument("--input", type=str, required=True, help="输入文件路径")
    analyze_parser.add_argument("--fields", type=str, nargs="+", default=["instruction", "output"],
                               help="分析字段")
    analyze_parser.add_argument("--output", type=str, help="输出报告路径")

    # 数据清洗命令
    clean_parser = subparsers.add_parser("clean-enhanced", help="增强数据清洗")
    clean_parser.add_argument("--input", type=str, required=True, help="输入文件路径")
    clean_parser.add_argument("--output", type=str, required=True, help="输出文件路径")
    clean_parser.add_argument("--rules", type=str, nargs="+", 
                             default=["remove_empty", "remove_duplicates", "normalize_whitespace", "trim_whitespace"],
                             help="清洗规则")

    # 增强导出命令
    export_enhanced_parser = subparsers.add_parser("export-enhanced", help="增强数据导出")
    export_enhanced_parser.add_argument("--input", type=str, required=True, help="输入文件路径")
    export_enhanced_parser.add_argument("--output", type=str, required=True, help="输出文件路径")
    export_enhanced_parser.add_argument("--format", type=str, default="json",
                                       choices=["json", "jsonl", "csv", "tsv", "alpaca", "sharegpt", 
                                               "chatml", "llama_factory", "vicuna", "belle", "openai", "huggingface"],
                                       help="导出格式")
    export_enhanced_parser.add_argument("--max-items", type=int, help="最大导出数量")
    export_enhanced_parser.add_argument("--shuffle", action="store_true", help="随机打乱")
    export_enhanced_parser.add_argument("--seed", type=int, default=42, help="随机种子")

    # 质量报告命令
    quality_report_parser = subparsers.add_parser("quality-report", help="生成质量报告")
    quality_report_parser.add_argument("--input", type=str, required=True, help="输入文件路径")
    quality_report_parser.add_argument("--output", type=str, help="输出报告路径")
    quality_report_parser.add_argument("--format", type=str, default="json", choices=["json", "markdown"], help="报告格式")
    quality_report_parser.add_argument("--threshold", type=float, default=0.7, help="质量阈值")

    # 可视化命令（增强版）
    visualize_parser = subparsers.add_parser("visualize-data", help="数据可视化")
    visualize_parser.add_argument("--input", type=str, required=True, help="输入文件路径")
    visualize_parser.add_argument("--output", type=str, help="输出报告路径")
    visualize_parser.add_argument("--format", type=str, default="text", choices=["text", "json"], help="报告格式")

    # 备份命令
    backup_parser = subparsers.add_parser("backup", help="数据备份")
    backup_parser.add_argument("--action", type=str, required=True, choices=["create", "restore", "list", "delete"], help="操作类型")
    backup_parser.add_argument("--input", type=str, help="输入文件路径")
    backup_parser.add_argument("--output", type=str, help="输出文件路径")
    backup_parser.add_argument("--name", type=str, help="备份名称")
    backup_parser.add_argument("--backup-dir", type=str, default=".backups", help="备份目录")

    # 增强搜索命令
    search_enhanced_parser = subparsers.add_parser("search-enhanced", help="增强搜索")
    search_enhanced_parser.add_argument("--input", type=str, required=True, help="输入文件路径")
    search_enhanced_parser.add_argument("--query", type=str, required=True, help="搜索查询")
    search_enhanced_parser.add_argument("--field", type=str, nargs="+", default=["instruction", "output"], help="搜索字段")
    search_enhanced_parser.add_argument("--method", type=str, default="contains", choices=["exact", "contains", "fuzzy", "regex"], help="搜索方法")
    search_enhanced_parser.add_argument("--limit", type=int, default=100, help="返回数量")
    search_enhanced_parser.add_argument("--offset", type=int, default=0, help="偏移量")

    # 统计命令
    stats_enhanced_parser = subparsers.add_parser("stats-enhanced", help="增强统计")
    stats_enhanced_parser.add_argument("--input", type=str, required=True, help="输入文件路径")
    stats_enhanced_parser.add_argument("--output", type=str, help="输出报告路径")
    stats_enhanced_parser.add_argument("--fields", type=str, nargs="+", help="统计字段")

    # 增强比较命令
    compare_enhanced_parser = subparsers.add_parser("compare-enhanced", help="增强比较")
    compare_enhanced_parser.add_argument("--dataset-a", type=str, required=True, help="数据集A路径")
    compare_enhanced_parser.add_argument("--dataset-b", type=str, required=True, help="数据集B路径")
    compare_enhanced_parser.add_argument("--output", type=str, help="输出报告路径")
    compare_enhanced_parser.add_argument("--key-fields", type=str, nargs="+", default=["instruction"], help="关键字段")

    # 版本控制命令
    version_parser = subparsers.add_parser("version-control", help="版本控制")
    version_parser.add_argument("--action", type=str, required=True, choices=["create", "list", "load", "compare"], help="操作类型")
    version_parser.add_argument("--input", type=str, help="输入文件路径")
    version_parser.add_argument("--output", type=str, help="输出文件路径")
    version_parser.add_argument("--version", type=str, help="版本ID")
    version_parser.add_argument("--description", type=str, default="", help="版本描述")
    version_parser.add_argument("--versions-dir", type=str, default=".versions", help="版本目录")

    # 自动化测试命令
    auto_test_parser = subparsers.add_parser("auto-test", help="自动化测试")
    auto_test_parser.add_argument("--input", type=str, required=True, help="输入文件路径")
    auto_test_parser.add_argument("--output", type=str, help="输出报告路径")
    auto_test_parser.add_argument("--suite", type=str, help="测试套件名称")

    # 质量监控命令
    monitor_parser = subparsers.add_parser("monitor", help="质量监控")
    monitor_parser.add_argument("--input", type=str, required=True, help="输入文件路径")
    monitor_parser.add_argument("--output", type=str, help="输出报告路径")

    # 依赖管理命令
    dependency_parser = subparsers.add_parser("dependency", help="依赖管理")
    dependency_parser.add_argument("--action", type=str, required=True, choices=["register", "list", "graph", "validate"], help="操作类型")
    dependency_parser.add_argument("--input", type=str, help="输入文件路径")
    dependency_parser.add_argument("--name", type=str, help="数据集名称")
    dependency_parser.add_argument("--output", type=str, help="输出报告路径")
    dependency_parser.add_argument("--registry-path", type=str, default=".dependency_registry", help="注册表路径")

    # 迁移命令
    migrate_parser = subparsers.add_parser("migrate", help="数据迁移")
    migrate_parser.add_argument("--input", type=str, required=True, help="输入文件路径")
    migrate_parser.add_argument("--output", type=str, required=True, help="输出文件路径")
    migrate_parser.add_argument("--rules", type=str, nargs="+", help="迁移规则")

    # 数据画像命令
    profiling_parser = subparsers.add_parser("profile", help="生成数据集画像")
    profiling_parser.add_argument("--input", type=str, required=True, help="输入文件路径")
    profiling_parser.add_argument("--output", type=str, help="画像输出路径（.json）")

    # 异常检测命令
    outlier_parser = subparsers.add_parser("outliers", help="检测长度异常样本")
    outlier_parser.add_argument("--input", type=str, required=True, help="输入文件路径")
    outlier_parser.add_argument("--method", type=str, default="zscore",
                                choices=["zscore", "iqr", "zscore_one_sided"], help="检测方法")
    outlier_parser.add_argument("--threshold", type=float, default=3.0, help="判定阈值")
    outlier_parser.add_argument("--field", type=str, default="length", help="检测字段")
    outlier_parser.add_argument("--output", type=str, help="结果输出路径（.json）")

    # 特征检测命令
    feature_parser = subparsers.add_parser("features", help="检测数据集特征维度")
    feature_parser.add_argument("--input", type=str, required=True, help="输入文件路径")
    feature_parser.add_argument("--output", type=str, help="结果输出路径（.json）")

    # 自动配置推荐命令
    auto_config_parser = subparsers.add_parser("auto-config", help="推荐数据管线参数")
    auto_config_parser.add_argument("--input", type=str, required=True, help="输入文件路径")
    auto_config_parser.add_argument("--output", type=str, help="推荐结果输出路径（.json）")

    # 数据聚合命令
    aggregate_parser = subparsers.add_parser("aggregate", help="聚合多个数据集")
    aggregate_parser.add_argument("--inputs", type=str, nargs="+", required=True,
                                   help="源数据集文件路径列表")
    aggregate_parser.add_argument("--output", type=str, required=True, help="输出文件路径")
    aggregate_parser.add_argument("--strategy", type=str, default="union",
                                   choices=["union", "intersection", "weighted", "consistent"],
                                   help="聚合策略")
    aggregate_parser.add_argument("--target-size", type=int, default=100, help="加权采样目标量")

    return parser


def main():
    parser = build_parser()
    args = parser.parse_args()

    if not args.command:
        parser.print_help()
        sys.exit(1)

    # 加载配置
    config = load_config(args.config)

    # 数据画像命令
    if args.command == "profile":
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
    if args.command == "outliers":
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
    if args.command == "features":
        from augmentor.feature_detect import FeatureDetector

        items = _load_items(args.input)
        detector = FeatureDetector()
        report = detector.detect(items)
        _print(report)
        if args.output:
            _dump_json(report, args.output)
        return

    # 自动配置推荐命令
    if args.command == "auto-config":
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
    if args.command == "aggregate":
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

    try:
        # ============ 质量评估 ============
        if args.command == "quality":
            from augmentor.quality import QualityScorer
            from augmentor.dedup import Deduplicator
            from augmentor.report import ReportGenerator

            items = _load_items(args.input)

            scorer = QualityScorer(
                threshold=config.quality.threshold,
                weights=config.quality.weights
            )
            scoring_items = [
                {
                    "original": item.get("instruction", ""),
                    "generated": item.get("instruction", ""),
                    "output": item.get("output", "")
                }
                for item in items
            ]
            scores = scorer.batch_score(scoring_items)

            dedup_summary = Deduplicator(
                threshold=config.dedup.threshold
            ).generate_report(items)

            report = ReportGenerator(
                threshold=config.quality.threshold
            ).generate(items, scores, dedup_summary)

            if args.output:
                filtered = [
                    item for item, score in zip(items, scores) if score.passed
                ]
                _save_items(filtered, args.output)
                print(f"已保存 {len(filtered)} 条通过质量检查的数据到 {args.output}")

            if args.report:
                report_path = Path(args.report)
                report_path.parent.mkdir(parents=True, exist_ok=True)
                if report_path.suffix.lower() == ".md":
                    report_path.write_text(report.to_markdown(), encoding='utf-8')
                else:
                    with open(report_path, 'w', encoding='utf-8') as f:
                        json.dump(report.to_dict(), f, ensure_ascii=False, indent=2)
                print(f"质量报告已保存到 {args.report}")

            if not args.output and not args.report:
                _print(report.to_dict())

        # ============ 导出预览 ============
        elif args.command == "preview":
            from augmentor.preview import PreviewGenerator

            items = _load_items(args.input)
            preview = PreviewGenerator(preview_size=args.size).preview(
                items, args.format
            )
            _print(preview.to_dict())

        # ============ 数据清洗 ============
        elif args.command == "clean":
            from augmentor.data import DataCleaner

            items = _load_items(args.input)
            cleaner = DataCleaner(remove_urls=not args.no_url_removal)
            result = cleaner.clean(items)
            _save_items(result.items, args.output)
            _print({
                "original_count": result.original_count,
                "cleaned_count": result.cleaned_count,
                "dropped_count": result.dropped_count,
                "changed_count": result.changed_count,
                "language_distribution": result.language_distribution,
                "issues": result.issues
            })

        # ============ 自动标注 ============
        elif args.command == "annotate":
            from augmentor.data import AutoAnnotator

            items = _load_items(args.input)
            annotator = AutoAnnotator()
            result = annotator.annotate(items)
            _save_items(result.items, args.output)
            _print({
                "total_items": len(items),
                "entity_count": result.entity_count,
                "intent_distribution": result.intent_distribution,
                "sentiment_distribution": result.sentiment_distribution
            })

        # ============ RAG 格式 ============
        elif args.command == "rag":
            from augmentor.rag import RAGFormatter

            items = _load_items(args.input)
            formatter = RAGFormatter(
                chunk_size=config.rag.chunk_size,
                chunk_overlap=config.rag.chunk_overlap
            )
            records = formatter.format(items, args.format)
            _save_items(records, args.output)
            print(f"已转换 {len(items)} 条数据为 {args.format} 格式，输出 {len(records)} 条记录")

        # ============ 质量基准 ============
        elif args.command == "benchmark":
            from augmentor.benchmark import QualityBenchmark

            items = _load_items(args.input)
            bench = QualityBenchmark(
                metrics=config.benchmark.metrics,
                baseline_file=args.baseline or config.benchmark.baseline_file,
                threshold=config.quality.threshold
            )
            results = bench.run_benchmark(items)

            if args.save_baseline:
                bench.save_baseline(results)
                print(f"基准已保存到 {bench.baseline_file}")

            if bench.baseline_file and bench.baseline_file.exists() and not args.save_baseline:
                try:
                    results["comparisons"] = bench.compare_with_baseline(results)["comparisons"]
                except ValueError as e:
                    print(f"跳过基准对比: {e}", file=sys.stderr)

            if args.report:
                report_path = Path(args.report)
                report_path.parent.mkdir(parents=True, exist_ok=True)
                report_path.write_text(bench.generate_report(results), encoding='utf-8')
                print(f"基准报告已保存到 {args.report}")

            _print(results)

        # ============ 增强 ============
        elif args.command == "augment":
            pipeline = AugmentorPipeline(config)
            result = pipeline.augment_dataset(
                args.input,
                args.output,
                use_checkpoint=not args.no_checkpoint,
                use_quality_check=not args.no_quality,
                use_dedup=not args.no_dedup
            )
            _print(result)

        # ============ 导出 ============
        elif args.command == "export":
            pipeline = AugmentorPipeline(config)
            results = pipeline.export_dataset(args.input, args.output_dir, args.formats)
            _print(results)

        # ============ 分析 ============
        elif args.command == "analyze":
            pipeline = AugmentorPipeline(config)
            result = pipeline.analyze_dataset(args.input)
            _print(result)

        # ============ 可视化 ============
        elif args.command == "visualize":
            pipeline = AugmentorPipeline(config)
            results = pipeline.visualize_dataset(args.input, args.output_dir)
            _print(results)

        # ============ 版本管理 ============
        elif args.command == "version":
            pipeline = AugmentorPipeline(config)

            if args.action == "list":
                versions = pipeline.version_manager.list_versions()
                for v in versions:
                    print(f"{v.version_id}: {v.label} ({v.item_count} items)")

            elif args.action == "create":
                items = _load_items(args.input)
                version = pipeline.version_manager.create_version(items)
                print(f"创建版本: {version.version_id}")

            elif args.action == "diff":
                diff = pipeline.version_manager.diff(args.version_id, args.version_id_2)
                print(f"新增: {diff.added_count}, 移除: {diff.removed_count}")

            elif args.action == "rollback":
                success = pipeline.version_manager.rollback(args.version_id)
                print("回滚成功" if success else "回滚失败")

            elif args.action == "history":
                _print(pipeline.version_manager.get_history(limit=20))

        # ============ 数据集对比 ============
        elif args.command == "compare":
            from augmentor.comparison import compare_datasets

            result = compare_datasets(
                args.dataset_a,
                args.dataset_b,
                name_a=args.name_a,
                name_b=args.name_b,
                output_path=args.output
            )
            print(result.summary)

        # ============ 流式处理 ============
        elif args.command == "stream":
            from augmentor.streaming import StreamAugmentor
            from augmentor.quality import QualityScorer
            from augmentor.dedup import Deduplicator

            def get_processor(operation: str):
                if operation == "quality":
                    scorer = QualityScorer(threshold=config.quality.threshold)
                    def process(items):
                        scoring_items = [
                            {
                                "original": item.get("instruction", ""),
                                "generated": item.get("instruction", ""),
                                "output": item.get("output", "")
                            }
                            for item in items
                        ]
                        scores = scorer.batch_score(scoring_items)
                        return [
                            item for item, score in zip(items, scores) if score.passed
                        ]
                    return process
                elif operation == "dedup":
                    deduplicator = Deduplicator(threshold=config.dedup.threshold)
                    return lambda items: deduplicator.deduplicate_and_filter(items)
                elif operation == "clean":
                    from augmentor.data import DataCleaner
                    cleaner = DataCleaner()
                    def process(items):
                        result = cleaner.clean(items)
                        return result.items
                    return process
                else:
                    return lambda items: items

            processor = get_processor(args.operation)
            augmentor = StreamAugmentor(
                input_file=args.input,
                output_file=args.output,
                processor=processor,
                chunk_size=args.chunk_size
            )
            result = augmentor.augment()
            _print(result)

        # ============ 数据集合并 ============
        elif args.command == "merge":
            from augmentor.dataset_ops import DatasetOperations, MergeConfig

            config_merge = MergeConfig(deduplicate=not args.no_dedup)
            ops = DatasetOperations()
            result = ops.merge_files(args.inputs, args.output, config_merge)
            _print(result)

        # ============ 数据集采样 ============
        elif args.command == "sample":
            from augmentor.dataset_ops import DatasetOperations, SampleConfig

            config_sample = SampleConfig(
                method=args.method,
                size=args.size,
                ratio=args.ratio,
                seed=args.seed
            )
            ops = DatasetOperations()
            result = ops.sample_file(args.input, args.output, config_sample)
            _print(result)

        # ============ 数据集分割 ============
        elif args.command == "split":
            from augmentor.dataset_ops import DatasetOperations, SplitConfig

            config_split = SplitConfig(
                ratios=(args.train_ratio, args.val_ratio, args.test_ratio),
                seed=args.seed
            )
            ops = DatasetOperations()
            result = ops.split_file(args.input, args.output_dir, config_split)
            _print(result)

        # ============ 数据集统计 ============
        elif args.command == "stats":
            from augmentor.dataset_ops import DatasetOperations

            items = _load_items(args.input)
            ops = DatasetOperations()
            stats = ops.get_statistics(items)
            _print(stats)

        # ============ 数据集验证 ============
        elif args.command == "validate":
            from augmentor.validation import DatasetValidator

            validator = DatasetValidator(preset=args.preset)
            result = validator.validate_file(args.input)
            
            if args.output:
                output_path = Path(args.output)
                output_path.parent.mkdir(parents=True, exist_ok=True)
                with open(output_path, 'w', encoding='utf-8') as f:
                    json.dump(result.to_dict(), f, ensure_ascii=False, indent=2)
                print(f"验证结果已保存到 {args.output}")
            
            print(f"验证结果: {'通过' if result.is_valid else '失败'}")
            print(f"总数据: {result.total_items}, 有效: {result.valid_items}")
            print(f"错误: {result.error_count}, 警告: {result.warning_count}")

        # ============ 数据集转换 ============
        elif args.command == "convert":
            from augmentor.converter import convert_file

            result = convert_file(args.input, args.output, target_format=args.format)
            _print(result)

        # ============ 数据集搜索 ============
        elif args.command == "search":
            from augmentor.indexer import DatasetIndexer

            items = _load_items(args.input)
            indexer = DatasetIndexer(items)
            
            result = indexer.search(
                args.query,
                fields=args.field,
                method=args.method
            )
            
            # 限制返回数量
            result.items = result.items[:args.limit]
            
            print(f"找到 {result.total_matches} 条匹配结果，用时 {result.query_time_ms:.2f}ms")
            print(f"搜索索引: {result.index_used}")
            _print(result.items)

        # ============ 配置验证 ============
        elif args.command == "validate-config":
            from augmentor.config_validator import validate_config_file

            config_path = args.config or "config.yaml"
            result = validate_config_file(config_path)
            
            print(f"配置验证结果: {'通过' if result.is_valid else '失败'}")
            print(f"错误: {len(result.errors)}, 警告: {len(result.warnings)}, 信息: {len(result.info)}")
            
            if result.errors:
                print("\n错误:")
                for e in result.errors:
                    print(f"  - {e.path}: {e.message}")
            
            if result.warnings:
                print("\n警告:")
                for w in result.warnings:
                    print(f"  - {w.path}: {w.message}")

        # ============ 数据分析 ============
        elif args.command == "analyze-data":
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

        # ============ 增强数据清洗 ============
        elif args.command == "clean-enhanced":
            from augmentor.cleaner import clean_dataset

            items = _load_items(args.input)
            cleaned, result = clean_dataset(items, rules=args.rules)
            
            _save_items(cleaned, args.output)
            
            print(f"原始数据: {result.original_count} 条")
            print(f"清洗后: {result.cleaned_count} 条")
            print(f"移除: {result.removed_count} 条")
            print(f"应用规则: {', '.join(result.rules_applied)}")
            print(f"已保存到 {args.output}")

        # ============ 增强数据导出 ============
        elif args.command == "export-enhanced":
            from augmentor.export_enhanced import export_dataset

            items = _load_items(args.input)
            result = export_dataset(
                items, 
                args.output, 
                format=args.format,
                max_items=args.max_items,
                shuffle=args.shuffle,
                seed=args.seed
            )
            
            print(f"导出格式: {result['format']}")
            print(f"导出数量: {result['item_count']} 条")
            print(f"已保存到 {args.output}")

        # ============ 质量报告 ============
        elif args.command == "quality-report":
            from augmentor.quality_report import generate_quality_report, save_quality_report

            items = _load_items(args.input)
            dataset_name = Path(args.input).stem
            report = generate_quality_report(items, dataset_name, args.threshold)
            
            print(f"数据集: {report.dataset_name}")
            print(f"数据总量: {report.total_items}")
            print(f"总体评分: {report.overall_score:.2f}")
            print(f"总体状态: {'通过' if report.overall_passed else '未通过'}")
            
            print("\n质量指标:")
            for metric in report.metrics:
                status = "✅" if metric.passed else "❌"
                print(f"  {status} {metric.name}: {metric.value:.2f} ({metric.description})")
            
            if report.recommendations:
                print("\n改进建议:")
                for rec in report.recommendations:
                    print(f"  - {rec}")
            
            if args.output:
                save_quality_report(report, args.output, args.format)
                print(f"\n报告已保存到 {args.output}")

        # ============ 数据可视化 ============
        elif args.command == "visualize-data":
            from augmentor.visualize_enhanced import visualize_dataset

            items = _load_items(args.input)
            result = visualize_dataset(items, args.output, args.format)
            
            print(result)

        # ============ 数据备份 ============
        elif args.command == "backup":
            from augmentor.backup import create_backup, restore_backup, list_backups, delete_backup

            if args.action == "create":
                if not args.input:
                    print("错误: --input 参数是必需的", file=sys.stderr)
                    sys.exit(1)
                info = create_backup(args.input, args.backup_dir, args.name)
                print(f"备份创建成功")
                print(f"备份ID: {info.backup_id}")
                print(f"数据量: {info.item_count} 条")
                print(f"文件大小: {info.file_size} 字节")
            
            elif args.action == "restore":
                if not args.name or not args.output:
                    print("错误: --name 和 --output 参数是必需的", file=sys.stderr)
                    sys.exit(1)
                result = restore_backup(args.name, args.output, args.backup_dir)
                print(f"恢复成功")
                print(f"恢复数量: {result['item_count']} 条")
                print(f"保存到: {args.output}")
            
            elif args.action == "list":
                backups = list_backups(args.backup_dir)
                if not backups:
                    print("没有找到备份")
                else:
                    print(f"找到 {len(backups)} 个备份:")
                    for backup in backups:
                        print(f"  - {backup['backup_id']}: {backup['item_count']} 条 ({backup['timestamp']})")
            
            elif args.action == "delete":
                if not args.name:
                    print("错误: --name 参数是必需的", file=sys.stderr)
                    sys.exit(1)
                success = delete_backup(args.name, args.backup_dir)
                if success:
                    print(f"删除成功: {args.name}")
                else:
                    print(f"删除失败: 备份不存在 {args.name}")

        # ============ 增强搜索 ============
        elif args.command == "search-enhanced":
            from augmentor.search_enhanced import search_dataset

            items = _load_items(args.input)
            result = search_dataset(
                items, 
                args.query, 
                args.field, 
                args.method, 
                args.limit
            )
            
            print(f"找到 {result.total_matches} 条匹配结果，用时 {result.query_time_ms:.2f}ms")
            print(f"搜索方法: {result.method}")
            
            if result.items:
                print("\n搜索结果:")
                for i, item in enumerate(result.items[:10], 1):
                    print(f"  {i}. {item.get('instruction', '')[:50]}...")
            
            if args.output:
                output_path = Path(args.output)
                output_path.parent.mkdir(parents=True, exist_ok=True)
                with open(output_path, 'w', encoding='utf-8') as f:
                    json.dump(result.to_dict(), f, ensure_ascii=False, indent=2)
                print(f"\n结果已保存到 {args.output}")

        # ============ 增强统计 ============
        elif args.command == "stats-enhanced":
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

        # ============ 增强比较 ============
        elif args.command == "compare-enhanced":
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

        # ============ 版本控制 ============
        elif args.command == "version-control":
            from augmentor.version_control import create_version, load_version, list_versions
            from augmentor.version_control import DatasetVersionManager

            manager = DatasetVersionManager(args.versions_dir)
            
            if args.action == "create":
                if not args.input:
                    print("错误: --input 参数是必需的", file=sys.stderr)
                    sys.exit(1)
                items = _load_items(args.input)
                version = manager.create_version(items, args.description)
                print(f"版本创建成功")
                print(f"版本ID: {version.version_id}")
                print(f"版本号: {version.version_number}")
                print(f"数据量: {version.item_count} 条")
            
            elif args.action == "list":
                versions = manager.list_versions()
                if not versions:
                    print("没有找到版本")
                else:
                    print(f"找到 {len(versions)} 个版本:")
                    for v in versions:
                        print(f"  - {v['version_id']}: {v['item_count']} 条 ({v['timestamp']})")
            
            elif args.action == "load":
                if not args.version or not args.output:
                    print("错误: --version 和 --output 参数是必需的", file=sys.stderr)
                    sys.exit(1)
                items = manager.load_version(args.version)
                _save_items(items, args.output)
                print(f"版本加载成功")
                print(f"数据量: {len(items)} 条")
                print(f"保存到: {args.output}")
            
            elif args.action == "compare":
                if not args.version or not args.output:
                    print("错误: --version 和 --output 参数是必需的", file=sys.stderr)
                    sys.exit(1)
                # 比较当前版本和指定版本
                current_version = manager.get_current_version()
                if not current_version:
                    print("错误: 没有当前版本", file=sys.stderr)
                    sys.exit(1)
                result = manager.compare_versions(current_version, args.version)
                print(f"版本比较结果:")
                print(f"  版本A: {result['version_a']}")
                print(f"  版本B: {result['version_b']}")
                print(f"  仅在A中: {result['stats']['only_in_a_count']} 条")
                print(f"  仅在B中: {result['stats']['only_in_b_count']} 条")
                print(f"  共同数据: {result['stats']['in_both_count']} 条")

        # ============ 自动化测试 ============
        elif args.command == "auto-test":
            from augmentor.auto_test import DatasetTestRunner, run_dataset_tests

            items = _load_items(args.input)
            suite = run_dataset_tests(items, args.suite)
            
            runner = DatasetTestRunner()
            report = runner.get_test_report(suite)
            print(report)
            
            if args.output:
                output_path = Path(args.output)
                output_path.parent.mkdir(parents=True, exist_ok=True)
                with open(output_path, 'w', encoding='utf-8') as f:
                    json.dump(suite.to_dict(), f, ensure_ascii=False, indent=2)
                print(f"\n测试报告已保存到 {args.output}")

        # ============ 质量监控 ============
        elif args.command == "monitor":
            from augmentor.quality_monitor import monitor_quality

            items = _load_items(args.input)
            snapshot = monitor_quality(items)
            
            print(f"质量监控结果:")
            print(f"  快照ID: {snapshot.snapshot_id}")
            print(f"  时间: {snapshot.timestamp}")
            print(f"  指标:")
            for metric_name, metric_value in snapshot.metrics.items():
                print(f"    {metric_name}: {metric_value:.2f}")
            
            if snapshot.alerts:
                print(f"  告警:")
                for alert in snapshot.alerts:
                    print(f"    - {alert.message}")
            
            if args.output:
                output_path = Path(args.output)
                output_path.parent.mkdir(parents=True, exist_ok=True)
                with open(output_path, 'w', encoding='utf-8') as f:
                    json.dump(snapshot.to_dict(), f, ensure_ascii=False, indent=2)
                print(f"\n监控报告已保存到 {args.output}")

        # ============ 依赖管理 ============
        elif args.command == "dependency":
            from augmentor.dependency import DependencyManager

            manager = DependencyManager(args.registry_path)
            
            if args.action == "register":
                if not args.input or not args.name:
                    print("错误: --input 和 --name 参数是必需的", file=sys.stderr)
                    sys.exit(1)
                items = _load_items(args.input)
                info = manager.register_dataset(args.name, args.input, len(items))
                print(f"数据集注册成功")
                print(f"  ID: {info.dataset_id}")
                print(f"  名称: {info.name}")
                print(f"  数据量: {info.item_count} 条")
            
            elif args.action == "list":
                datasets = manager.list_datasets()
                if not datasets:
                    print("没有注册的数据集")
                else:
                    print(f"找到 {len(datasets)} 个数据集:")
                    for ds in datasets:
                        print(f"  - {ds.dataset_id}: {ds.name} ({ds.item_count} 条)")
            
            elif args.action == "graph":
                graph = manager.get_dependency_graph()
                print(f"依赖图:")
                print(f"  节点数: {len(graph['nodes'])}")
                print(f"  边数: {len(graph['edges'])}")
                
                if args.output:
                    output_path = Path(args.output)
                    output_path.parent.mkdir(parents=True, exist_ok=True)
                    with open(output_path, 'w', encoding='utf-8') as f:
                        json.dump(graph, f, ensure_ascii=False, indent=2)
                    print(f"  依赖图已保存到 {args.output}")
            
            elif args.action == "validate":
                issues = manager.validate_dependencies()
                if not issues:
                    print("依赖关系验证通过")
                else:
                    print(f"发现 {len(issues)} 个问题:")
                    for issue in issues:
                        print(f"  - {issue['message']}")

        # ============ 数据迁移 ============
        elif args.command == "migrate":
            from augmentor.migration import migrate_file

            result = migrate_file(args.input, args.output, args.rules)
            
            print(f"数据迁移完成")
            print(f"  迁移ID: {result.migration_id}")
            print(f"  总数据: {result.total_items} 条")
            print(f"  成功迁移: {result.migrated_items} 条")
            print(f"  失败: {result.failed_items} 条")
            
            if result.rules_applied:
                print(f"  应用规则: {', '.join(result.rules_applied)}")
            
            if result.errors:
                print(f"  错误详情:")
                for error in result.errors[:5]:
                    print(f"    - 第 {error['index']} 条: {error['error']}")

    except Exception as e:
        print(f"错误: {e}", file=sys.stderr)
        sys.exit(1)


if __name__ == "__main__":
    main()
