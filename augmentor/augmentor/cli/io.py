# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""CLI 共用 I/O helper（由 cli.py 拆出）"""

import json
import sys
from pathlib import Path


def harden_stdio():
    """把 stdout / stderr 的编码错误处理从 `strict` 换成 `replace`（A98）

    子进程管道按 locale 编码时（zh-CN Windows 上就是 GBK），用户数据里一个 emoji 会让
    整条命令崩成 `UnicodeEncodeError` ⇒ 被 `cli.py:main()` 翻译成人退出码 1，而 1 在
    `docs/ARCHITECTURE.md` §6 的口径里正是「判决未通过」那一档 ⇒ 崩溃与判负同号。

    只改错误处理器、不改编码：GBK 能编码的字符逐字节不变，所以今天正常的输出一个字都不
    会变；变的只有今天**根本产不出来**（崩掉）的那批字符，它们落成一个 `?`，JSON 输出
    因此仍可被 `json.loads` 解析。
    """
    for stream in (sys.stdout, sys.stderr):
        if getattr(stream, "errors", None) == "strict" and hasattr(stream, "reconfigure"):
            stream.reconfigure(errors="replace")


def _load_items(path: str):
    """加载 JSON 数据文件

    Args:
        path: 文件路径

    Returns:
        数据列表
    """
    with open(path, 'r', encoding='utf-8') as f:
        return json.load(f)


def _save_items(items, path: str):
    """保存数据到 JSON 文件

    Args:
        path: 文件路径
    """
    output_path = Path(path)
    output_path.parent.mkdir(parents=True, exist_ok=True)
    with open(output_path, 'w', encoding='utf-8') as f:
        json.dump(items, f, ensure_ascii=False, indent=2)


def _dump_json(data, path: str):
    """将任意可 JSON 序列化对象写入文件

    Args:
        data: 可 JSON 序列化对象
        path: 输出文件路径
    """
    output_path = Path(path)
    output_path.parent.mkdir(parents=True, exist_ok=True)
    with open(output_path, 'w', encoding='utf-8') as f:
        json.dump(data, f, ensure_ascii=False, indent=2, default=str)


def _print(data):
    """打印 JSON 结果

    Args:
        data: 任意可序列化对象
    """
    print(json.dumps(data, ensure_ascii=False, indent=2, default=str))
