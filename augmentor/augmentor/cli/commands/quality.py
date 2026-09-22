"""CLI `quality` 组命令（由 cli.py 拆出，逻辑未改）"""

from ..io import _load_items, _print, _save_items
from pathlib import Path
import json
import sys


# ============ 质量评估 ============
def run_quality(args, config):
    """`quality` 子命令"""
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


# ============ 质量报告 ============
def run_quality_report(args, config):
    """`quality-report` 子命令"""
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


# ============ 数据清洗（合并原 clean-enhanced）============
def run_clean(args, config):
    """`clean` 子命令

    默认走 `data.DataCleaner`：可 `--no-url-removal`，输出语言分布与问题清单。
    `--enhanced` 走 `cleaner.clean_dataset`：按 `--rules` 规则式清洗。

    两者是**不同实现**，不是同一实现的强弱版——旧的 `-enhanced` 后缀并不表示
    「更高级」，这正是 T1.7 要合并它们的原因。
    """
    if args.enhanced:
        from augmentor.cleaner import clean_dataset

        items = _load_items(args.input)
        cleaned, result = clean_dataset(items, rules=args.rules)

        _save_items(cleaned, args.output)

        print(f"原始数据: {result.original_count} 条")
        print(f"清洗后: {result.cleaned_count} 条")
        print(f"移除: {result.removed_count} 条")
        print(f"应用规则: {', '.join(result.rules_applied)}")
        print(f"已保存到 {args.output}")
        return

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
def run_annotate(args, config):
    """`annotate` 子命令"""
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
def run_rag(args, config):
    """`rag` 子命令"""
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
def run_benchmark(args, config):
    """`benchmark` 子命令"""
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
