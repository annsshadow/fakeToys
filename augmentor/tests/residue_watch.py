# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""A168：仓库根产物残留守卫的纯逻辑（被 `tests/conftest.py` 的 autouse 用例守卫调用）

立这条守卫的代价是把「测试有没有把产物落在版本树里」变成机器判据，因此判据范围必须说清。
被盯的只有 `WATCHED` 那三个目录的**顶层子项**，认的只有「用例开始前不存在、结束后还在」：

| 形状 | 判不判 | 为什么 |
| --- | --- | --- |
| 用例在自己作用域里建了又删 | 不判 | 收尾干净，树里没有残留 |
| 用例改动本来就在的文件（内容变、名字没变） | 不判 | 残留量的是「多出来的东西」，内容漂移另有尺子 |
| 用例删掉一件原有产物 | 不判 | 那是另一种破坏，要另一条判据，别混进这个桶 |
| 写进 `data/` 更深一层或 `tests/` 里 | 不判 | 不在被盯的三处顶层 |
| 在用例结束后留下一件顶层新产物 | **判** | 这正是 L93 踩过的那格 |

口径的两端都是量出来的，不是推出来的（L94 实测，形状与守卫一致的探针）：

- 全套 `7063 passed / 3 skipped` 跑下来 `RESIDUE_COUNT 0` ⇒ 这条守卫入库**不需要任何白名单**，
  也不会搬倒既有 7 千条用例。
- 一次注入对照（一条用例往仓库根写 `xxx.json`）量到 `RESIDUE_COUNT 1 CASES 1`，且名单里就是
  那条用例的 node-id ⇒ 它不是恒绿的空尺子。

仓库根取 `conftest` 传进来的目录，不取 `os.getcwd()`：用例里一旦有人 `chdir` 又没还回去，
以工作目录为准的守卫就会去盯别人家的目录，那正是它要防的形状之一。
"""

import os
from typing import Dict, List, Set

#: 相对仓库根被盯的目录；`""` 就是仓库根本身（`data/`、`.backups/` 只看各自顶层）
WATCHED = ("", "data", ".backups")


def list_top(directory: str) -> Set[str]:
    """列出一个目录的顶层子项名

    Args:
        directory: 目录绝对路径

    Returns:
        子项名集合；目录不存在时是空集合（缺失不等于错误，`data/` 可以被谁都没建过）
    """
    if not os.path.isdir(directory):
        return set()
    return set(os.listdir(directory))


def snapshot(root: str) -> Dict[str, Set[str]]:
    """给被盯目录拍一张「顶层有什么」的快照

    Args:
        root: 仓库根

    Returns:
        `{被盯目录: 顶层子项名集合}`，键与 `WATCHED` 同序
    """
    return {name: list_top(os.path.join(root, name)) for name in WATCHED}


def display(root: str, watched: str, name: str) -> str:
    """把一件残留写成相对仓库根的可读路径

    Args:
        root: 仓库根（只为与 `watched` 拼接后取相对形式，不重复参与判定）
        watched: `WATCHED` 里的那一项，`""` 表示仓库根
        name: 顶层子项名

    Returns:
        仓库根下的产物给裸名，子目录下的给 `data/xxx.json` 这种相对路径
    """
    return os.path.relpath(os.path.join(root, watched, name), root).replace("\\", "/")


def leaked(root: str, before: Dict[str, Set[str]]) -> List[str]:
    """数出「开始前不存在、现在还在」的顶层新产物

    Args:
        root: 仓库根
        before: 用例开始前 `snapshot(root)` 的快照

    Returns:
        相对仓库根的残留路径列表，已排序；没有残留时是空列表
    """
    after = snapshot(root)
    found: List[str] = []
    for watched in WATCHED:
        for name in sorted(after[watched] - before.get(watched, set())):
            found.append(display(root, watched, name))
    return found


def find_residue(root: str, before: Dict[str, Set[str]], nodeid: str):
    """把「有没有残留」做成一条可直接判红的消息

    判定与措辞住在这里而不是住在 `conftest` 里，是为了让两个方向都能被用例直接驱动
    （pytest 9 起不允许直接调用 fixture，见 `tests/unit/test_repo_residue_l94.py`）。

    Args:
        root: 仓库根
        before: 用例开始前的快照
        nodeid: 被点名的用例 id

    Returns:
        有残留时返回那条消息；干净时返回 `None`
    """
    found = leaked(root, before)
    if not found:
        return None
    return (f"{nodeid} 在被盯目录顶层留下了 {len(found)} 件新产物："
            f"{', '.join(found)} —— 请改用 tmp_path，或把这件产物在用例内删掉。")
