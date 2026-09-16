"""AI 训练数据增强工具 CLI 入口"""

import argparse
import json
import sys
from pathlib import Path

from augmentor import AugmentorPipeline, load_config


def main():
    parser = argparse.ArgumentParser(description="AI 训练数据增强工具")
    parser.add_argument("--config", type=str, default="config.yaml", help="配置文件路径")
    
    subparsers = parser.add_subparsers(dest="command", help="子命令")
    
    # 增强命令
    augment_parser = subparsers.add_parser("augment", help="增强数据集")
    augment_parser.add_argument("--input", type=str, required=True, help="输入文件路径")
    augment_parser.add_argument("--output", type=str, required=True, help="输出文件路径")
    augment_parser.add_argument("--no-quality", action="store_true", help="禁用质量检查")
    augment_parser.add_argument("--no-dedup", action="store_true", help="禁用去重")
    augment_parser.add_argument("--no-checkpoint", action="store_true", help="禁用断点续传")
    
    # 导出命令
    export_parser = subparsers.add_parser("export", help="导出数据集")
    export_parser.add_argument("--input", type=str, required=True, help="输入文件路径")
    export_parser.add_argument("--output-dir", type=str, required=True, help="输出目录")
    export_parser.add_argument("--formats", nargs="+", help="导出格式")
    
    # 分析命令
    analyze_parser = subparsers.add_parser("analyze", help="分析数据集")
    analyze_parser.add_argument("--input", type=str, required=True, help="输入文件路径")
    
    # 可视化命令
    visualize_parser = subparsers.add_parser("visualize", help="可视化数据集")
    visualize_parser.add_argument("--input", type=str, required=True, help="输入文件路径")
    visualize_parser.add_argument("--output-dir", type=str, default="visualizations", help="输出目录")
    
    # 版本管理命令
    version_parser = subparsers.add_parser("version", help="版本管理")
    version_parser.add_argument("--action", choices=["list", "create", "diff", "rollback"], required=True, help="操作类型")
    version_parser.add_argument("--input", type=str, help="输入文件路径")
    version_parser.add_argument("--version-id", type=str, help="版本 ID")
    version_parser.add_argument("--version-id-2", type=str, help="第二个版本 ID（用于 diff）")
    
    args = parser.parse_args()
    
    if not args.command:
        parser.print_help()
        sys.exit(1)
    
    # 加载配置
    config = load_config(args.config)
    pipeline = AugmentorPipeline(config)
    
    try:
        if args.command == "augment":
            result = pipeline.augment_dataset(
                args.input,
                args.output,
                use_checkpoint=not args.no_checkpoint,
                use_quality_check=not args.no_quality,
                use_dedup=not args.no_dedup
            )
            print(json.dumps(result, ensure_ascii=False, indent=2))
        
        elif args.command == "export":
            results = pipeline.export_dataset(args.input, args.output_dir, args.formats)
            print(json.dumps(results, ensure_ascii=False, indent=2))
        
        elif args.command == "analyze":
            result = pipeline.analyze_dataset(args.input)
            print(json.dumps(result, ensure_ascii=False, indent=2))
        
        elif args.command == "visualize":
            results = pipeline.visualize_dataset(args.input, args.output_dir)
            print(json.dumps(results, ensure_ascii=False, indent=2))
        
        elif args.command == "version":
            if args.action == "list":
                versions = pipeline.version_manager.list_versions()
                for v in versions:
                    print(f"{v.version_id}: {v.label} ({v.item_count} items)")
            
            elif args.action == "create":
                with open(args.input, 'r', encoding='utf-8') as f:
                    items = json.load(f)
                version = pipeline.version_manager.create_version(items)
                print(f"创建版本: {version.version_id}")
            
            elif args.action == "diff":
                diff = pipeline.version_manager.diff(args.version_id, args.version_id_2)
                print(f"新增: {diff.added_count}, 移除: {diff.removed_count}")
            
            elif args.action == "rollback":
                success = pipeline.version_manager.rollback(args.version_id)
                print("回滚成功" if success else "回滚失败")
    
    except Exception as e:
        print(f"错误: {e}", file=sys.stderr)
        sys.exit(1)


if __name__ == "__main__":
    main()
