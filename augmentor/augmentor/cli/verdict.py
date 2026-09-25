# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""CLI 判决 → 退出码的唯一出口

退出码在这里只有三种形状，彼此不许互相顶替：

* `0` —— 判决通过，或这条命令压根不判负（报告形命令的默认档）；
* `1` —— 判决未通过，只由本模块 `verdict_exit()` 产生；
* `2` —— argparse 的用法错误（参数不认识 / 缺必填），由解析器产生。

「异常」不从这里走：`cli.py` 的 `main()` 把 handler 抛出的任何异常翻译成
`错误: …` + 退出码 1，所以 **同样是 1，判决与崩溃必须由 stderr 有无「错误:」来分**
—— 这也是为什么判决不能靠崩出来的 1（L55 修掉的 GBK 崩溃就是这一档的现形）。
口径的来龙去脉见 `docs/ARCHITECTURE.md` §3.28（L53 立）与 §3.30（L55 接线收口）。
"""

import sys


def verdict_exit(passed: bool, enforce: bool = True) -> None:
    """判决未通过时以退出码 1 结束进程。

    Args:
        passed: 这条命令的判决，`True` = 通过
        enforce: `False` 时只出报告不判负。校验形命令用默认值（恒判负）；
            报告形命令把 parser 的 `--gate` 传进来（默认不判负，那是 L53 的
            口径「审计报告的本分是有问题时照样把报告出完」，不是遗漏）
    """
    if enforce and not passed:
        sys.exit(1)
