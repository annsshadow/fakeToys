# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""CLI `quality` 组命令（由 cli.py 拆出，逻辑未改）"""

from ..io import _load_items, _print, _save_items
from ..verdict import verdict_exit
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
        # 不用 ✅/❌：stdout 按 locale 编码，GBK 管道下这个字符会让整条命令崩在
        # rc=1（L55 实测），而 1 在这个 CLI 里是「判决未通过」的形状，不该由装饰符占
        status = "[通过]" if metric.passed else "[未通过]"
        print(f"  {status} {metric.name}: {metric.value:.2f} ({metric.description})")

    if report.recommendations:
        print("\n改进建议:")
        for rec in report.recommendations:
            print(f"  - {rec}")

    if args.output:
        save_quality_report(report, args.output, args.format)
        print(f"\n报告已保存到 {args.output}")

    # 报告形命令默认恒 0（L53 的口径），`--gate` 才把「总体状态: 未通过」翻译成退出码
    verdict_exit(report.overall_passed, enforce=args.gate)


# ============ 数据清洗 ============
def run_clean(args, config):
    """`clean` 子命令

    走 `cleaner.clean_dataset`（规则式）。合并前的另一套是
    `data.DataCleaner`，它的噪声清除能力（去 URL / 去 HTML 标签 / 去控制字符）
    已经作为 `remove_urls` / `remove_html_tags` / `remove_control_chars`
    三条规则并进规则式实现，因此两套的能力面现在是并集而不是二选一：
    规则式还额外能分别控制去空、去重、空白归一化、长短文本过滤、标点归一化。

    旧 `--no-url-removal` 的等价写法是 `--rules` 里不带 `remove_urls`。

    stdout 输出 `CleaningResult` 的 JSON（合并前基础实现的契约），`--output`
    只负责把清洗后的数据落盘。
    """
    from augmentor.cleaner import clean_dataset

    items = _load_items(args.input)
    cleaned, result = clean_dataset(items, fields=args.fields, rules=args.rules)

    _save_items(cleaned, args.output)
    _print(result.to_dict())


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


# ============ 健康度门禁 ============
def run_health_gate(args, config):
    """`health-gate` 子命令：健康度评分 + 质量门禁

    门禁不通过时以退出码 1 结束——这是它作为 CI 关卡的全部意义。判定明细仍然会
    打印出来，用来定位是哪条规则拦住的。`skipped_rules` 非空表示有规则因缺指标
    而未参与判定（最常见是 `--pass-rate` 没传）。
    """
    from augmentor.quality_gate import gate_dataset_health

    items = _load_items(args.input)
    result = gate_dataset_health(
        items,
        text_field=args.text_field,
        weights=args.weights,
        pass_rate=args.pass_rate,
        pass_rate_min=args.pass_rate_min,
        duplicate_rate_max=args.duplicate_rate_max,
        completeness_min=args.completeness_min,
        block_on_warning=args.block_on_warning,
    )

    _print(result)
    verdict_exit(result["gate"]["passed"])
