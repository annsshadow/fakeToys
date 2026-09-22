"""CLI `version` 组命令（由 cli.py 拆出，逻辑未改）"""

from ..io import _load_items, _print, _save_items
import sys
from augmentor import AugmentorPipeline


# ============ 版本管理（合并原 version-control）============
def run_version(args, config):
    """`version` 子命令

    默认走 `pipeline.version_manager`（`augmentor/versioning.py`）：
    `list` / `create` / `diff` / `rollback` / `history`，版本 ID 用 `--version-id`。
    `--enhanced` 走 `version_control.DatasetVersionManager`（独立版本目录）：
    `create` / `list` / `load` / `compare`，版本 ID 用 `--version`。

    两者是**两套独立后端**而非同一实现的强弱版，所以合并后仍有 `--enhanced`
    开关；`create` / `list` 两个动作两边都有，靠 `--enhanced` 区分。

    动作与后端不匹配时显式报错：原来基础分支的 if/elif 链在没有命中时
    **什么都不做**就正常退出，比报错更难排查。
    """
    if args.enhanced:
        from augmentor.version_control import DatasetVersionManager

        manager = DatasetVersionManager(args.versions_dir)

        if args.action == "create":
            if not args.input:
                print("错误: --input 参数是必需的", file=sys.stderr)
                sys.exit(1)
            items = _load_items(args.input)
            version = manager.create_version(items, args.description)
            print(f"版本创建成功")
            print(f"版本ID: {version.version_id}")
            print(f"版本号: {version.version_number}")
            print(f"数据量: {version.item_count} 条")

        elif args.action == "list":
            versions = manager.list_versions()
            if not versions:
                print("没有找到版本")
            else:
                print(f"找到 {len(versions)} 个版本:")
                for v in versions:
                    print(f"  - {v['version_id']}: {v['item_count']} 条 ({v['timestamp']})")

        elif args.action == "load":
            if not args.version or not args.output:
                print("错误: --version 和 --output 参数是必需的", file=sys.stderr)
                sys.exit(1)
            items = manager.load_version(args.version)
            _save_items(items, args.output)
            print(f"版本加载成功")
            print(f"数据量: {len(items)} 条")
            print(f"保存到: {args.output}")

        elif args.action == "compare":
            if not args.version or not args.output:
                print("错误: --version 和 --output 参数是必需的", file=sys.stderr)
                sys.exit(1)
            # 比较当前版本和指定版本
            current_version = manager.get_current_version()
            if not current_version:
                print("错误: 没有当前版本", file=sys.stderr)
                sys.exit(1)
            result = manager.compare_versions(current_version, args.version)
            print(f"版本比较结果:")
            print(f"  版本A: {result['version_a']}")
            print(f"  版本B: {result['version_b']}")
            print(f"  仅在A中: {result['stats']['only_in_a_count']} 条")
            print(f"  仅在B中: {result['stats']['only_in_b_count']} 条")
            print(f"  共同数据: {result['stats']['in_both_count']} 条")

        else:
            print(
                f"错误: --enhanced 不支持 --action {args.action}，"
                "支持: create / list / load / compare",
                file=sys.stderr,
            )
            sys.exit(1)
        return

    pipeline = AugmentorPipeline(config)

    if args.action == "list":
        versions = pipeline.version_manager.list_versions()
        for v in versions:
            print(f"{v.version_id}: {v.label} ({v.item_count} items)")

    elif args.action == "create":
        items = _load_items(args.input)
        version = pipeline.version_manager.create_version(items)
        print(f"创建版本: {version.version_id}")

    elif args.action == "diff":
        diff = pipeline.version_manager.diff(args.version_id, args.version_id_2)
        print(f"新增: {diff.added_count}, 移除: {diff.removed_count}")

    elif args.action == "rollback":
        success = pipeline.version_manager.rollback(args.version_id)
        print("回滚成功" if success else "回滚失败")

    elif args.action == "history":
        _print(pipeline.version_manager.get_history(limit=20))

    else:
        print(
            f"错误: 不支持 --action {args.action}，"
            "支持: list / create / diff / rollback / history"
            "（load / compare 需要 --enhanced）",
            file=sys.stderr,
        )
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
