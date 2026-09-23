# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""CLI `analysis` 组命令

3.0 起这 4 个命令各自只有一套实现，`--enhanced` 已移除。合并的原则是
**并集而非二选一**：落败侧独有的能力必须并进幸存实现，否则「合并」就变成了
静默的能力删除。

* `analyze`：幸存实现是 `analytics.DatasetAnalyzer`（质量 / 多样性 / 完整性
  三个分数 + 洞察建议），旧基础实现（`pipeline.analyze_dataset`）独有的
  `coverage_analysis` / `statistics` / `dedup_report` 三段并进同一份 JSON。
* `stats`：幸存实现 `statistics.DatasetStatisticsCalculator` 本就是旧
  `dataset_ops.get_statistics` 的严格超集（逐字段填充率 / 长度 / 唯一值已覆盖
  后者的 total / avg / min / max / unique），无需并能力。
* `compare`：两种实现是**互补**的两种对比——`comparison.DatasetComparator`
  给质量向 A/B 结论，`compare_enhanced` 给重叠度与字段级差异。两者都保留。
* `visualize`：产物形态不同（图片 vs 报告），并为一个命令的两个模式。

stdout 契约：`analyze` / `stats` 走 `_print`（JSON），`compare` 走 markdown 摘要
——与合并前「基础实现」一致，下游管道不受影响。`--output` 只把同一份内容落盘，
提示语写 stderr，不污染 stdout。
"""

import json
import sys
from pathlib import Path

from ..io import _load_items, _print


def _write_json(payload, output_path: str, label: str):
    """把报告写到指定路径（自动建父目录）

    提示语写 stderr：这些命令的 stdout 是机器可读的报告本体，
    往里面掺一行中文提示会让 `json.loads(stdout)` 直接失败。

    Args:
        payload: 可 JSON 序列化的内容
        output_path: 输出路径
        label: 用于提示语的产物名称
    """
    path = Path(output_path)
    path.parent.mkdir(parents=True, exist_ok=True)
    with open(path, 'w', encoding='utf-8') as f:
        json.dump(payload, f, ensure_ascii=False, indent=2)
    print(f"{label}已保存到 {output_path}", file=sys.stderr)


# ============ 数据集分析 ============
def run_analyze(args, config):
    """`analyze` 子命令

    输出一份 JSON 报告，包含两部分：

    * `analytics.DatasetAnalyzer` 的成果（顶层）：`dataset_size` /
      `field_statistics` / `insights` / `scores`；
    * 旧 `pipeline.analyze_dataset` 独有的三段：`coverage_analysis`
      （覆盖分析）、`statistics`（词频与长度）、`dedup_report`（去重报告）。

    后三段若在合并中丢掉，`analyze` 就会悄悄少掉覆盖分析能力——旧基础实现的
    调用方拿 `report["coverage_analysis"]` 会直接 KeyError。
    """
    from augmentor.analytics import analyze_dataset
    from augmentor.dedup import Deduplicator
    from augmentor.sampler import ActiveSampler
    from augmentor.visualizer import generate_statistics

    items = _load_items(args.input)
    report = analyze_dataset(items, args.fields)

    payload = report.to_dict()
    payload["coverage_analysis"] = ActiveSampler().analyze_coverage(items)
    payload["statistics"] = generate_statistics(items)
    payload["dedup_report"] = Deduplicator(
        threshold=config.dedup.threshold
    ).generate_report(items)

    _print(payload)

    if args.output:
        _write_json(payload, args.output, "分析报告")


# ============ 数据集统计 ============
def run_stats(args, config):
    """`stats` 子命令

    走 `statistics.calculate_statistics`：逐字段给出填充率 / 平均长度 / 唯一值，
    并附整体质量指标。旧基础实现（`dataset_ops.get_statistics`）只给
    total / avg_length / min_length / max_length / unique_instructions 五个标量，
    是这套逐字段统计在 `instruction` 字段上的子集，因此没有能力需要并进来。
    """
    from augmentor.statistics import calculate_statistics

    items = _load_items(args.input)
    dataset_name = Path(args.input).stem
    stats = calculate_statistics(items, dataset_name, args.fields)

    payload = stats.to_dict()
    _print(payload)

    if args.output:
        _write_json(payload, args.output, "统计报告")


# ============ 数据可视化 ============
def run_visualize(args, config):
    """`visualize` 子命令

    两种输出模式由参数决定：

    * 传 `--output`：报告模式，走 `visualize_enhanced.visualize_dataset`，
      按 `--format` 输出文本或 JSON 报告；
    * 不传 `--output`：图表模式（默认），走 `visualizer.DataVisualizer`，
      把词云 / 长度分布 / 主题聚类 / 质量分布 / 时间线落盘到 `--output-dir`。

    两种产物形态不同（图片 vs 报告），所以不是「二选一」，而是并存为一个命令的
    两个模式——这也是原来 `--enhanced` 唯一在语义上还站得住的一对。
    """
    items = _load_items(args.input)

    if args.output:
        from augmentor.visualize_enhanced import visualize_dataset

        print(visualize_dataset(items, args.output, args.format))
        return

    from augmentor.visualizer import DataVisualizer

    visualizer = DataVisualizer(output_dir=args.output_dir or "visualizations")
    _print(visualizer.generate_all_visualizations(items))


# ============ 数据集对比 ============
def run_compare(args, config):
    """`compare` 子命令

    同时给出两种互补的对比结论：

    * `comparison.DatasetComparator`（markdown 摘要）：质量分 / 通过率 / 长度 /
      词汇量对比与获胜方判定；
    * `compare_enhanced.compare_datasets_enhanced`：重叠度（相似度 / 共同条数 /
      各自独有条数）与字段级差异 + 改进建议。

    旧基础实现只打印前者、旧增强实现只打印后者，任一单独保留都会丢一半结论。

    数据集展示名优先取 `--name-a` / `--name-b`，未给时取文件名 stem。
    """
    from augmentor.comparison import DatasetComparator
    from augmentor.compare_enhanced import compare_datasets_enhanced

    items_a = _load_items(args.dataset_a)
    items_b = _load_items(args.dataset_b)

    name_a = args.name_a or Path(args.dataset_a).stem
    name_b = args.name_b or Path(args.dataset_b).stem

    quality = DatasetComparator().compare(items_a, items_b, name_a, name_b)
    overlap = compare_datasets_enhanced(
        items_a, items_b, name_a, name_b, args.key_fields
    )

    # markdown 摘要（沿用基础实现的 stdout 契约）
    print(quality.summary)
    print()
    print("### 重叠度对比")
    print(f"- 相似度: {overlap.metrics.similarity_score:.2%}")
    print(f"- 共同数据: {overlap.metrics.common_items} 条")
    print(f"- 仅在A中: {overlap.metrics.unique_a} 条")
    print(f"- 仅在B中: {overlap.metrics.unique_b} 条")

    if overlap.recommendations:
        print()
        print("### 建议")
        for rec in overlap.recommendations:
            print(f"- {rec}")

    if args.output:
        _write_json(
            {
                "quality_comparison": quality.to_dict(),
                "overlap_comparison": overlap.to_dict(),
            },
            args.output,
            "比较报告",
        )
