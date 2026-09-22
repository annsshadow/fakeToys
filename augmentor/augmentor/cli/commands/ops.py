# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""CLI `ops` 组命令（由 cli.py 拆出，逻辑未改）"""

from ..io import _load_items, _print
from pathlib import Path
import json
import sys


# ============ 依赖诊断 ============
def run_doctor(args, config):
    """`doctor` 子命令"""
    from augmentor.diagnostics import check_dependencies

    report = check_dependencies()
    if args.json:
        _print(report.to_dict())
    else:
        print(f"可选依赖: {report.available_count}/{len(report.installed)} 可用")
        if report.missing:
            print(f"缺失: {', '.join(report.missing)}")
        if report.degraded_features:
            print("降级特性:")
            for feature in report.degraded_features:
                print(f"  - {feature}")


# ============ 自动化测试 ============
def run_auto_test(args, config):
    """`auto-test` 子命令"""
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
def run_monitor(args, config):
    """`monitor` 子命令"""
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
def run_dependency(args, config):
    """`dependency` 子命令"""
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
def run_migrate(args, config):
    """`migrate` 子命令"""
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
