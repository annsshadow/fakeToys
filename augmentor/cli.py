# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""AI 训练数据增强工具 CLI 入口

本文件只负责「解析参数 → 归一化旧命令名 → 查分发表 → 调用 handler → 统一异常处理」。
36 个规范命令的实现分别在 `augmentor/cli/commands/` 包里，按领域分组：
`profiling` / `pipeline` / `quality` / `export` / `analysis` /
`data_ops` / `version` / `security` / `ops`。

历史：这里曾是 1308 行的单文件（`build_parser()` 329 行 + `main()` 922 行的
if/elif 长链，每分支内部用函数内 import）。拆分后 `main()` 只做分发，
新增子命令只需在 `cli/commands/<组>.py` 加一个 `run_xxx(args, config)` 并登记。
"""

import sys

from augmentor import load_config

from augmentor.cli import COMMANDS
from augmentor.cli.parser import LEGACY_ALIASES, build_parser


def main():
    """CLI 主入口"""
    parser = build_parser()
    args = parser.parse_args()

    if not args.command:
        parser.print_help()
        sys.exit(1)

    # 旧命令名归一化。`export-enhanced` 等 8 个名字在 T1.7 合并进了主命令，
    # 但保留了 argparse alias，因此 `args.command` 仍会是旧名——在这里映射成
    # 规范名并置 `--enhanced`，让 handler 不必知道旧名的存在。
    canonical = LEGACY_ALIASES.get(args.command)
    if canonical is not None:
        print(
            f"提示: `{args.command}` 已合并为 `{canonical} --enhanced`，"
            "旧名将在下个大版本移除",
            file=sys.stderr,
        )
        args.command = canonical
        args.enhanced = True

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
