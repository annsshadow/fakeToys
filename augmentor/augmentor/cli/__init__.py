# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""CLI 实现包（`cli.py` 是它的薄入口）

结构：
    io.py            共用 I/O helper
    parser.py        build_parser()
    commands/        44 个子命令的 handler 与分发表
"""

from .commands import COMMANDS

__all__ = ["COMMANDS"]
