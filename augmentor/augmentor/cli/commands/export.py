# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""CLI `export` 组命令"""

import random
import sys
from pathlib import Path

from ..io import _load_items, _print


# ============ 导出预览 ============
def run_preview(args, config):
    """`preview` 子命令"""
    from augmentor.preview import PreviewGenerator

    items = _load_items(args.input)
    preview = PreviewGenerator(preview_size=args.size).preview(
        items, args.format
    )
    _print(preview.to_dict())


# ============ 数据集导出 ============
def run_export(args, config):
    """`export` 子命令

    单一实现：`Exporter`（`augmentor/export.py`）。它原生实现 6 种格式，其余
    委托给 `EnhancedExporter`，因此覆盖 `EXPORT_FORMATS` 全量条目——这正是
    原来「基础实现 / `--enhanced`」两条路可以合并的前提：两条路用的转换器本来
    就大量重叠，各写一份只会让同一种格式出现两种产物。

    输出模式由参数决定：

    * `--output` + `--format`：导出单个文件
    * `--output-dir` + `--formats`：每个格式导出一个文件

    `--max-items` / `--shuffle` / `--seed` 是格式无关的预处理，两种模式都生效
    （原先只有 `--enhanced` 那条路支持）。

    stdout 形态**随模式不同**，这是刻意保留两侧原契约的结果，不是疏漏：

    * 批量模式 `{格式: 路径}` 的 JSON —— 沿用原基础实现（`_print`）；
    * 单文件模式三行文本确认 —— 沿用原 `--enhanced` 实现。

    批量模式的产物是一张「格式 → 文件」映射表，天然适合机器读；单文件模式的
    产物是「刚才那一次导出」，给人看更合适。
    """
    from augmentor.export import Exporter

    items = _load_items(args.input)

    if args.max_items is not None:
        if args.max_items < 0:
            print("错误: --max-items 不能为负数", file=sys.stderr)
            sys.exit(1)
        items = items[:args.max_items]

    if args.shuffle:
        # 用局部 Random 而非 `random.seed()`：后者会改写全局 RNG 状态，
        # 影响同进程内其它调用方的随机性（既有实现就有这个副作用）。
        random.Random(args.seed).shuffle(items)

    exporter = Exporter()

    if args.output_dir:
        if not args.formats:
            print("错误: --output-dir 需要配合 --formats 指定格式", file=sys.stderr)
            sys.exit(1)
        base_name = Path(args.input).stem
        results = exporter.export_batch(
            {base_name: items}, args.output_dir, args.formats
        )
        _print(results[base_name])
        return

    if not args.output:
        print(
            "错误: 需要 --output（导出单个文件）或 --output-dir（批量导出多种格式）",
            file=sys.stderr,
        )
        sys.exit(1)

    fmt = args.format or "json"
    exporter.export(items, args.output, fmt)
    print(f"导出格式: {fmt}")
    print(f"导出数量: {len(items)} 条")
    print(f"已保存到 {args.output}")
