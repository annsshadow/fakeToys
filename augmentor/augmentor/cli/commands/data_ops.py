# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""CLI `data_ops` 组命令（由 cli.py 拆出，逻辑未改）"""

from ..io import _load_items, _print
from pathlib import Path
import json
import sys


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
        seed=args.seed,
        stratify_key=args.stratify_key
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
        seed=args.seed,
        shuffle=not args.no_shuffle,
        stratify=args.stratify,
        stratify_key=args.stratify_key
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

    result = convert_file(args.input, args.output, target_format=args.format,
                          source_format=args.input_format)
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


# ============ 数据集搜索 ============
def _filter_specs(raw):
    """把 `--filter FIELD OP VALUE` 的三元组转成 SDK 认识的字典清单

    命令行只能给出字符串，而 `gt` 要数字、`in` 要列表，所以值先按 JSON 解析、
    解不动才当字符串（`--filter instruction contains 申请` 是绝大多数用法，
    `--filter count gt 5` / `--filter status in '["a","b"]'` 走 JSON 那一支）。
    **算子与值形状对不对不在这里判**——判据在 `search_enhanced.normalize_filters`
    那一处咽喉，经 `main()` 的 `except Exception` 变成 `错误: …` + 退出码 1。
    """
    if not raw:
        return None

    specs = []
    for field, operator, value in raw:
        try:
            parsed = json.loads(value)
        except json.JSONDecodeError:
            parsed = value
        specs.append({"field": field, "operator": operator, "value": parsed})
    return specs


def run_search(args, config):
    """`search` 子命令

    走 `search_enhanced.search_dataset`：支持 exact / contains / ngram /
    fuzzy / regex 五种方法，以及 `--offset` 分页与 `--output` 落盘完整结果。
    `--fuzzy-threshold` / `--ngram-n` 是 fuzzy 与 ngram 的松紧旋钮（`search()` 的
    `fuzzy_threshold` / `ngram_n`），默认 0.6 / 2；`--filter FIELD OP VALUE` 是可重复的
    收窄条件（`search()` 的 `filters`，检索之后按字段值再筛一遍）；`--limit` /
    `--offset` 是分页窗口。这四类的越界值都由 SDK 那**一处**判据拦下，经 `main()`
    的 `except Exception` 变成 `错误: …` + 退出码 1，不在此重复校验。
    分页以前不在拒绝名单里：`--limit -1` 打印「找到 2 条」却把除最后一条以外的
    全部条目吐进 JSON（切片把负数读成反向窗口）。`--limit 0` 不是越界值，它报
    总数不给条目，是合法答案。

    `ngram` 原先只存在于另一套实现（`indexer.DatasetIndexer`），已移植进
    `EnhancedSearcher`——所以这里不是「少了一个方法」，而是五种方法齐全。

    stdout 契约沿用合并前的基础实现：两行摘要 + **匹配条目的 JSON 列表**。
    条目本身就是这个命令的产物，只打印摘要会逼着下游改用 `--output` 落盘再读，
    属于输出能力的静默缩水。`--output` 额外落盘 `SearchResult` 的完整字典
    （含 `total_matches` / `method` / `query_time_ms`）。生效的松紧旋钮印在
    **第二行末尾**（不另起一行），所以「两行摘要 + JSON」这个形状对下游解析不变。
    带 `--filter` 时同一行末尾再追一段「N 个过滤器: 检索 X → 保留 Y」，
    让「被过滤到 0 条」与「检索本来就没命中」在 stdout 上也分得开。
    """
    from augmentor.search_enhanced import search_dataset

    items = _load_items(args.input)
    result = search_dataset(
        items,
        args.query,
        args.field,
        args.method,
        100 if args.limit is None else args.limit,
        args.offset,
        fuzzy_threshold=args.fuzzy_threshold,
        ngram_n=args.ngram_n,
        filters=_filter_specs(args.filter),
    )

    print(f"找到 {result.total_matches} 条匹配结果，用时 {result.query_time_ms:.2f}ms")
    # 生效的旋钮并排印出（不是回显用户输入的那份）：「找到 0 条」有两种成因——
    # 语料里确实没有，和阈值/ gram 长度拧得太紧，只报方法名分不开这两种。
    effective = ""
    if result.fuzzy_threshold is not None:
        effective += f" (生效阈值 {result.fuzzy_threshold})"
    elif result.ngram_n is not None:
        effective += f" (生效 gram 长度 {result.ngram_n})"
    # 第三种成因：过滤器把候选筛掉了。没有这一段时，「找到 0 条」配上
    # `--filter` 读起来和「检索本身就没命中」一模一样，而收窄其实是生效了的。
    if result.applied_filters is not None:
        effective += (f" ({len(result.applied_filters)} 个过滤器: "
                      f"检索 {result.matches_before_filters} → 保留 {result.total_matches})")
    print(f"搜索方法: {result.method}{effective}")
    _print(result.items)

    if args.output:
        output_path = Path(args.output)
        output_path.parent.mkdir(parents=True, exist_ok=True)
        with open(output_path, 'w', encoding='utf-8') as f:
            json.dump(result.to_dict(), f, ensure_ascii=False, indent=2)
        print(f"结果已保存到 {args.output}", file=sys.stderr)
