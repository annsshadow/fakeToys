"""CLI `data_ops` 组命令（由 cli.py 拆出，逻辑未改）"""

from ..io import _load_items, _print
from pathlib import Path
import json


# ============ 数据集合并 ============
def run_merge(args, config):
    """`merge` 子命令"""
    from augmentor.dataset_ops import DatasetOperations, MergeConfig

    config_merge = MergeConfig(deduplicate=not args.no_dedup)
    ops = DatasetOperations()
    result = ops.merge_files(args.inputs, args.output, config_merge)
    _print(result)


# ============ 数据集采样 ============
def run_sample(args, config):
    """`sample` 子命令"""
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
def run_split(args, config):
    """`split` 子命令"""
    from augmentor.dataset_ops import DatasetOperations, SplitConfig

    config_split = SplitConfig(
        ratios=(args.train_ratio, args.val_ratio, args.test_ratio),
        seed=args.seed
    )
    ops = DatasetOperations()
    result = ops.split_file(args.input, args.output_dir, config_split)
    _print(result)


# ============ 数据集验证 ============
def run_validate(args, config):
    """`validate` 子命令"""
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
def run_convert(args, config):
    """`convert` 子命令"""
    from augmentor.converter import convert_file

    result = convert_file(args.input, args.output, target_format=args.format)
    _print(result)


# ============ 配置验证 ============
def run_validate_config(args, config):
    """`validate-config` 子命令"""
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


# ============ 数据集搜索（合并原 search-enhanced）============
def run_search(args, config):
    """`search` 子命令

    默认走 `indexer.DatasetIndexer`：支持 exact / contains / ngram，默认返回 10 条。
    `--enhanced` 走 `search_enhanced.search_dataset`：额外支持 fuzzy / regex 与
    `--offset`，默认返回 100 条，可 `--output` 落盘完整结果。

    `--limit` 的两套默认值不同（10 / 100），所以解析器里留空，在这里按分支补。
    """
    if args.enhanced:
        from augmentor.search_enhanced import search_dataset

        items = _load_items(args.input)
        result = search_dataset(
            items,
            args.query,
            args.field,
            args.method,
            100 if args.limit is None else args.limit
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
        return

    from augmentor.indexer import DatasetIndexer

    items = _load_items(args.input)
    indexer = DatasetIndexer(items)

    result = indexer.search(
        args.query,
        fields=args.field,
        method=args.method
    )

    # 限制返回数量
    result.items = result.items[:10 if args.limit is None else args.limit]

    print(f"找到 {result.total_matches} 条匹配结果，用时 {result.query_time_ms:.2f}ms")
    print(f"搜索索引: {result.index_used}")
    _print(result.items)
