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

    return parser


def main():
    parser = build_parser()
    args = parser.parse_args()

    if not args.command:
        parser.print_help()
        sys.exit(1)

    # 加载配置
    config = load_config(args.config)

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

    except Exception as e:
        print(f"错误: {e}", file=sys.stderr)
        sys.exit(1)


if __name__ == "__main__":
    main()
