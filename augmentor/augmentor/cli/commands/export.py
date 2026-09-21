"""CLI `export` 组命令（由 cli.py 拆出，逻辑未改）"""

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


# ============ 导出 ============
def run_export(args, config):
    """`export` 子命令"""
    pipeline = AugmentorPipeline(config)
    results = pipeline.export_dataset(args.input, args.output_dir, args.formats)
    _print(results)


# ============ 增强数据导出 ============
def run_export_enhanced(args, config):
    """`export-enhanced` 子命令"""
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
