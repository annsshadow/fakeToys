"""AI 训练数据增强工具 CLI 入口

本文件只负责「解析参数 → 查分发表 → 调用 handler → 统一异常处理」。
44 个子命令的实现分别在 `augmentor/cli/commands/` 包里，按领域分组：
`profiling` / `pipeline` / `quality` / `export` / `analysis` /
`data_ops` / `version` / `security` / `ops`。

历史：这里曾是 1308 行的单文件（`build_parser()` 329 行 + `main()` 922 行的
if/elif 长链，每分支内部用函数内 import）。拆分后 `main()` 只做分发，
新增子命令只需在 `cli/commands/<组>.py` 加一个 `run_xxx(args, config)` 并登记。
"""

import sys

from augmentor import load_config

from augmentor.cli import COMMANDS
from augmentor.cli.parser import build_parser


def main():
    """CLI 主入口"""
    parser = build_parser()
    args = parser.parse_args()

    if not args.command:
        parser.print_help()
        sys.exit(1)

    handler = COMMANDS.get(args.command)
    if handler is None:
        # 分发表漏登记时给出明确报错，而不是静默什么都不做
        print(f"错误: 未实现的子命令 {args.command!r}", file=sys.stderr)
        sys.exit(1)

    config = load_config(args.config)
    try:
        handler(args, config)
    except Exception as e:
        print(f"错误: {e}", file=sys.stderr)
        sys.exit(1)


if __name__ == "__main__":
    main()
