# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""CLI `export` 组命令"""

import sys

from ..io import _load_items, _print
from augmentor import AugmentorPipeline


# ============ 导出预览 ============
def run_preview(args, config):
    """`preview` 子命令"""
    from augmentor.preview import PreviewGenerator

    items = _load_items(args.input)
    preview = PreviewGenerator(preview_size=args.size).preview(
        items, args.format
    )
    _print(preview.to_dict())


# ============ 导出（合并原 export-enhanced）============
def run_export(args, config):
    """`export` 子命令

    默认走 `pipeline.export_dataset`：按 `--output-dir` 批量导出多种格式。
    `--enhanced` 走 `export_enhanced.export_dataset`：按 `--output` 导单个文件，
    支持 `--max-items` / `--shuffle` / `--seed`。

    两条路径的输出参数不通用（一个要目录、一个要文件），因此不能都标
    `required=True`——argparse 无法表达「取决于另一个 flag」的必填。改为在
    这里显式校验，与 `version` / `backup` 的既有写法一致。
    """
    if args.enhanced:
        from augmentor.export_enhanced import export_dataset

        if not args.output:
            print("错误: --enhanced 需要 --output", file=sys.stderr)
            sys.exit(1)
        items = _load_items(args.input)
        result = export_dataset(
            items,
            args.output,
            format=args.format or "json",
            max_items=args.max_items,
            shuffle=args.shuffle,
            seed=args.seed,
        )
        print(f"导出格式: {result['format']}")
        print(f"导出数量: {result['item_count']} 条")
        print(f"已保存到 {args.output}")
        return

    if not args.output_dir:
        print(
            "错误: 需要 --output-dir（批量导出多种格式）；"
            "导出单个文件请加 --enhanced --output",
            file=sys.stderr,
        )
        sys.exit(1)
    pipeline = AugmentorPipeline(config)
    results = pipeline.export_dataset(args.input, args.output_dir, args.formats)
    _print(results)
