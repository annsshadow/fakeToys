# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""A168：仓库根产物残留守卫的纯逻辑（被 `tests/conftest.py` 的 autouse 用例守卫调用）

立这条守卫的代价是把「测试有没有把产物落在版本树里」变成机器判据，因此判据范围必须说清。
被盯的只有 `WATCHED` 那五处的**顶层子项**，认的只有「用例开始前不存在、结束后还在」：

| 形状 | 判不判 | 为什么 |
| --- | --- | --- |
| 用例在自己作用域里建了又删 | 不判 | 收尾干净，树里没有残留 |
| 用例改动本来就在的文件（内容变、名字没变） | 不判 | 残留量的是「多出来的东西」，内容漂移另有尺子 |
| 用例删掉一件原有产物 | 不判 | 那是另一种破坏，要另一条判据，别混进这个桶 |
| 写进 `data/` 更深一层（`data/versions/` 与 `checkpoints/` 除外）或 `tests/` 里 | 不判 | 不在被盯的五处顶层 |
| 在用例结束后留下一件顶层新产物 | **判** | 这正是 L93 踩过的那格 |

口径的两端都是量出来的，不是推出来的（L94 实测，形状与守卫一致的探针）：

- 全套 `7063 passed / 3 skipped` 跑下来 `RESIDUE_COUNT 0` ⇒ 这条守卫入库**不需要任何白名单**，
  也不会搬倒既有 7 千条用例。
- 一次注入对照（一条用例往仓库根写 `xxx.json`）量到 `RESIDUE_COUNT 1 CASES 1`，且名单里就是
  那条用例的 node-id ⇒ 它不是恒绿的空尺子。

**L234 把 `data/versions/` 与 `checkpoints/` 补进被盯面（范围扩大，不是放松）**：原口径那句
「往 `data/` 更深一层写不判」在产品自己的目录结构上是对的，但 `data/versions/` 不是产品的
目录结构，是**用例跑出来的运行期产物**——`AugmentorPipeline` 按 `config.versioning.storage_dir`
（出厂默认是相对路径 `data/versions`）建 `VersionManager`，于是用例的 cwd 只要在仓库根，
`augment_dataset` 就往仓库里堆一个 `v_*` 快照。L234 现场量到 `data/versions/` 里有 **67** 个
`v_*` 目录，其中 `current` 是一份 WSL 在 drvfs 上留下的 LX 链接：Windows 侧 `is_symlink()`
认不出、`os.readlink()` 报 not a symbolic link、只有 `os.stat` 抛 WinError 1920 ⇒ 定标门禁被它
炸掉一支（A314）。扩范围之后现量（47 个「碰管道/版本」的文件、1 499 条用例、同一支插件把
`WATCHED` 换成扩过的元组）：**泄漏 1 条**，就是 `test_partial_branches_l120.py` 的
`TestPipelineVariantFiltering::test_malformed_variants_filtered_out`，已就地改成
`monkeypatch.chdir(tmp_path)` ⇒ 这条扩范围**不需要白名单**，与 L94 那次入库同形。

仓库根取 `conftest` 传进来的目录，不取 `os.getcwd()`：用例里一旦有人 `chdir` 又没还回去，
以工作目录为准的守卫就会去盯别人家的目录，那正是它要防的形状之一。
"""

import os
from typing import Dict, List, Set

#: 相对仓库根被盯的目录；`""` 就是仓库根本身（其余四处只看各自顶层）。
#: `data/versions` 与 `checkpoints` 是 L234 补进来的：它们是运行期产物的落点，不是产品结构。
WATCHED = ("", "data", ".backups", "data/versions", "checkpoints")


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
