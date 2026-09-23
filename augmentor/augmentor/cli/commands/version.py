# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""CLI `version` 组命令（由 cli.py 拆出，逻辑未改）"""

from ..io import _load_items, _print, _save_items
import sys


# ============ 版本管理 ============
def run_version(args, config):
    """`version` 子命令

    单一后端：`version_control.DatasetVersionManager`（独立 `--versions-dir`，
    不依赖 pipeline，因此不需要 config.yaml 与模型后端）。

    原先还有一套走 `pipeline.version_manager` 的后端。两套的版本 ID 规则与存储
    目录都不同，用户必须靠 `--enhanced` 才知道自己在跟哪一套打交道——这正是
    双轨最糟的形态，所以按「保留动作更全、依赖更少的那套」合并：

    * `diff` → 并入 `compare`（后者给出 only_in_a / only_in_b / in_both 统计）
    * `rollback` → 由 `set_current_version` 实现
    * `history` → 由 `DatasetVersionManager.get_history` 实现（操作日志，
      见 `version_control` 模块 docstring）。它记录「做过什么操作」，与
      `list` 的「现在有哪些版本」是两件事，所以不能靠 `list` 顶替。

    `compare` 的参数方向：版本A = `--version`，版本B = `--version-b`；
    `--version-b` 缺省时取当前版本（即「从 `--version` 到现在改了什么」）。
    """
    from augmentor.version_control import DatasetVersionManager

    manager = DatasetVersionManager(args.versions_dir)
    action = args.action

    if action == "create":
        if not args.input:
            print("错误: --input 参数是必需的", file=sys.stderr)
            sys.exit(1)
        items = _load_items(args.input)
        version = manager.create_version(items, args.description)
        print(f"版本创建成功")
        print(f"版本ID: {version.version_id}")
        print(f"版本号: {version.version_number}")
        print(f"数据量: {version.item_count} 条")

    elif action == "list":
        versions = manager.list_versions()
        if not versions:
            print("没有找到版本")
        else:
            print(f"找到 {len(versions)} 个版本:")
            for v in versions:
                print(f"  - {v['version_id']}: {v['item_count']} 条 ({v['timestamp']})")

    elif action == "history":
        _print(manager.get_history(limit=20))

    elif action == "load":
        if not args.version or not args.output:
            print("错误: --version 和 --output 参数是必需的", file=sys.stderr)
            sys.exit(1)
        items = manager.load_version(args.version)
        _save_items(items, args.output)
        print(f"版本加载成功")
        print(f"数据量: {len(items)} 条")
        print(f"保存到: {args.output}")

    elif action == "compare":
        # 版本A = --version，版本B = --version-b（缺省时取当前版本）
        if not args.version:
            print("错误: --version 参数是必需的", file=sys.stderr)
            sys.exit(1)
        version_b = args.version_b or manager.get_current_version()
        if not version_b:
            print(
                "错误: --version-b 缺省时需要已存在当前版本",
                file=sys.stderr,
            )
            sys.exit(1)
        result = manager.compare_versions(args.version, version_b)
        print(f"版本比较结果:")
        print(f"  版本A: {result['version_a']}")
        print(f"  版本B: {result['version_b']}")
        print(f"  仅在A中: {result['stats']['only_in_a_count']} 条")
        print(f"  仅在B中: {result['stats']['only_in_b_count']} 条")
        print(f"  共同数据: {result['stats']['in_both_count']} 条")

    elif action == "rollback":
        if not args.version:
            print("错误: --version 参数是必需的", file=sys.stderr)
            sys.exit(1)
        if manager.set_current_version(args.version):
            print(f"已回滚到 {args.version}")
        else:
            print(f"回滚失败: 版本不存在 {args.version}", file=sys.stderr)
            sys.exit(1)

    elif action == "delete":
        if not args.version:
            print("错误: --version 参数是必需的", file=sys.stderr)
            sys.exit(1)
        if manager.delete_version(args.version):
            print(f"删除成功: {args.version}")
        else:
            print(f"删除失败: 版本不存在 {args.version}", file=sys.stderr)
            sys.exit(1)


# ============ 数据备份 ============
def run_backup(args, config):
    """`backup` 子命令"""
    from augmentor.backup import create_backup, restore_backup, list_backups, delete_backup

    if args.action == "create":
        if not args.input:
            print("错误: --input 参数是必需的", file=sys.stderr)
            sys.exit(1)
        info = create_backup(args.input, args.backup_dir, args.name)
        print(f"备份创建成功")
        print(f"备份ID: {info.backup_id}")
        print(f"数据量: {info.item_count} 条")
        print(f"文件大小: {info.file_size} 字节")

    elif args.action == "restore":
        if not args.name or not args.output:
            print("错误: --name 和 --output 参数是必需的", file=sys.stderr)
            sys.exit(1)
        result = restore_backup(args.name, args.output, args.backup_dir)
        print(f"恢复成功")
        print(f"恢复数量: {result['item_count']} 条")
        print(f"保存到: {args.output}")

    elif args.action == "list":
        backups = list_backups(args.backup_dir)
        if not backups:
            print("没有找到备份")
        else:
            print(f"找到 {len(backups)} 个备份:")
            for backup in backups:
                print(f"  - {backup['backup_id']}: {backup['item_count']} 条 ({backup['timestamp']})")

    elif args.action == "delete":
        if not args.name:
            print("错误: --name 参数是必需的", file=sys.stderr)
            sys.exit(1)
        success = delete_backup(args.name, args.backup_dir)
        if success:
            print(f"删除成功: {args.name}")
        else:
            print(f"删除失败: 备份不存在 {args.name}")
