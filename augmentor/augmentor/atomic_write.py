# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""整份 JSON 的**原子**落盘 —— 读者永远看不见半份

存在理由是一条实测，不是一种洁癖（L99，`Temp/l99q/fin_after_drain_l99.py`）：一份
240 000 行的 xlsx 上传在服务端要处理 7.44 s，客户端在这段里放弃之后服务端**照样写完**，
而轮询读者在 t=5.76 s 与 6.05 s 两次读到 `JSONDecodeError`（磁盘上分别是 9 239 624 B 与
39 668 789 B，最终 45 840 001 B）。⇒ 旧写法 `open(path, 'w') + json.dump` 的**整个写窗口
都是一份坏数据集**：读端点、列表刷新、任何按 `*.json` 扫目录的消费者在窗口里都会拿到截断
的 JSON，而它们报出来的是一句「文件不是合法 JSON」——一份本来好好的文件被写得像是坏了。

同一轮量出的另一格更值得记：`CheckpointManager.load_checkpoint()` 的读侧是
`except Exception: logger.error(...); return None`，所以断点文件被截断时它不报错，而是
**返回「没有断点」**——上一轮写下的增量文件明明还在盘上，却再也不会被合并。截断在断点这一
条路上的读数不是「一次失败的读取」，是「整段已完成的工作凭空消失」。

因此本模块只管一件事：**要么旧内容，要么新内容，没有中间态**。`os.replace` 在同一文件系统
内是原子替换（Windows 走 `MoveFileExW` 的 `REPLACE_EXISTING`），先写同目录临时件再换过去
就够了。

判据口径（决定哪些写面该走这里，L99 的普查见账本 A182）：只看**读侧拿到截断内容时的行为**
——读侧把坏 JSON 报成异常 / 按未命中重算的，半份窗口是可恢复的代价，不必动；读侧把坏 JSON
**变成一个错误答案**（`return None` 那种）的，必须走原子替换。

刻意不做的事：不加 `os.fsync`。本轮修的是「读者看见半份」，它只需要原子替换；`fsync` 管的
是断电之后的持久性，是另一条主张，而且要拿真实写盘开销去换，本仓没测过那份开销。
"""

import json
import os
import secrets
import threading
import time
from pathlib import Path
from typing import Any, Dict, List, Optional, Union

# 同名并发替换在 Windows 上会撞到瞬时的 `ERROR_ACCESS_DENIED`：两支线程同时把各自的临时件
# 换到同一个目标时，后到的那支会被拒绝（L99 的并发守卫实测：8 支线程同一时刻写同一文件，
# 不打重试必有 PermissionError）。旧写法不报这个错，因为它让两支线程**同时往同一个文件里
# 交错写**——那是一份坏文件而没有人出声。所以这里换成正面处置：短暂退避重试，换上就成功，
# 上限内始终失败才抛。上限 5 次 × 20 ms ⇒ 最坏多花 0.08 s，且只在真的撞车时才付。
_SWAP_ATTEMPTS = 5
_SWAP_BACKOFF_S = 0.02


def _replace_with_retry(src: Path, dst: Path):
    """`os.replace` 的有界重试档，只对 Windows 的瞬时拒绝访问重试

    最后一次替换刻意放在循环外：它让「上限内始终被拒」不需要一个 `if attempt == 末次: raise`
    的分支，于是循环的自然出口就是「前面每次都被拒、最后一次照常上抛」这条真实路径。
    旧写法那条 `raise` 让循环出口结构上不可达，覆盖率只能挂着一个永远红不了的偏支。
    """
    for _attempt in range(_SWAP_ATTEMPTS - 1):
        try:
            os.replace(src, dst)
            return
        except PermissionError:
            time.sleep(_SWAP_BACKOFF_S)
    os.replace(src, dst)


def atomic_write_json(file_path: Path, data: Union[List, Any],
                       ensure_ascii: bool = False, indent: Optional[int] = None):
    """把 `data` 序列化成紧凑 JSON，原子地替换到 `file_path`

    失败语义：序列化或写盘抛错时目标文件**保持原样**（旧内容还在），临时件被清掉，
    异常原样上抛交给调用方判决 —— 本模块不吞异常，因为吞掉它就把「写失败」变成了
    「写成功但内容没变」。

    Args:
        file_path: 目标路径，父目录不存在时创建
        data: 可 JSON 序列化的对象（数据集是 list，断点与索引是 dict）

    Raises:
        OSError: 建目录失败，或临时件写失败，或替换在重试上限内始终被拒绝
        TypeError: `data` 里有 JSON 不可序列化的值（发生在临时件上，目标不受影响）
    """
    file_path = Path(file_path)
    file_path.parent.mkdir(parents=True, exist_ok=True)
    # 前导点 + `.tmp-` 中段：让它不匹配 `*.json`，否则目录扫描会把半份临时件当成一份
    # 数据文件列出来 —— 那只是把缺陷换了个位置。随机后缀保证同名并发写不互相踩临时件。
    tmp = file_path.with_name(f".{file_path.name}.tmp-{secrets.token_hex(4)}")
    try:
        with open(tmp, "w", encoding="utf-8") as f:
            if indent is None:
                json.dump(data, f, ensure_ascii=ensure_ascii, separators=(",", ":"))
            else:
                json.dump(data, f, ensure_ascii=ensure_ascii, indent=indent)
        _replace_with_retry(tmp, file_path)
    finally:
        if tmp.exists():
            tmp.unlink()


def atomic_write_text(file_path: Path, text: str):
    """把一段文本原子地替换到 `file_path`（L163 为 JSONL 导出边而加）

    与 `atomic_write_json` 同一替换语义：失败时目标保持原样、临时件清掉、
    异常原样上抛。临时件命名同样以「前导点 + `.tmp-` 中段」避开 `*.json`
    与 `*.jsonl` 的目录扫描。
    """
    file_path = Path(file_path)
    file_path.parent.mkdir(parents=True, exist_ok=True)
    tmp = file_path.with_name(f".{file_path.name}.tmp-{secrets.token_hex(4)}")
    try:
        with open(tmp, "w", encoding="utf-8", newline="") as f:
            f.write(text)
        _replace_with_retry(tmp, file_path)
    finally:
        if tmp.exists():
            tmp.unlink()


# ============ 读-改-写的互斥（L273） ============

#: 按「规范化后的绝对路径」登记的进程内互斥锁。第二把尺子管的是本模块 docstring
#: 没覆盖的那一半承诺：`os.replace` 保证**读者**看不见半份，它保证不了**写者**不互相
#: 覆盖 —— 两个写者各自先读整份、各自改自己那一格、各自整份写回，后写的那一份带着
#: 旧快照，把前一次改动凭空抹掉，而两边都回了 `success: true`。
#: 实测（Temp 沙箱，`probe_race.py` / `probe1_guard.py` / `probe3_versions.py`）：
#: 同一份 200 条数据集上 20 支并发 `PUT /api/data/update` ⇒ 落盘 1 处、丢 19 处，
#: 全部 200；50 支时另有 3 支 500（读侧撞 Windows 的瞬时拒绝访问）；并发 `DELETE` 20 支
#: ⇒ 只删掉 1 条，丢 19 次删除；备份索引 20 次创建 ⇒ 索引 1 行、盘上 20 份文件，
#: 那 19 份对 `list` / `restore` / `delete` / `clean_old_backups` 全部不可见。
_PATH_LOCKS: Dict[str, threading.RLock] = {}
_PATH_LOCKS_REGISTRY = threading.Lock()


def path_lock(file_path: Any) -> threading.RLock:
    r"""取（并记住）`file_path` 这一把锁 —— 同一个文件路径的读-改-写共用一把

    为什么是 `RLock` 不是 `Lock`：锁按路径建键之后，「外层罩住整段读-改-写」与「内层
    那个读写同一份文件的辅助方法自保」会**套在同一把锁上**（`version_control._load_index`
    被 `create_version` 在锁内调用就是这一形）。普通锁同线程重入会当场自锁死，可重入锁
    让两处各自成立、合起来仍然是一趟。

    键取 `normcase(abspath(...))`：同一份文件被写成相对与绝对两种形状时必须是同一把锁，
    否则锁就只是装饰。Windows 上 `normcase` 顺带折叠大小写（盘符路径本就不区分大小写）。

    锁表按路径单调增长，不在场就回收：一个进程碰过的文件数量是有界的（数据集名、
    备份索引、版本目录），比「每次调用新建一把锁」要对——那样两把锁互不相识，等于没锁。

    Args:
        file_path: 目标文件路径（`str` 或 `Path`）

    Returns:
        该路径专属的 `threading.RLock`
    """
    key = os.path.normcase(os.path.abspath(str(file_path)))
    with _PATH_LOCKS_REGISTRY:
        lock = _PATH_LOCKS.get(key)
        if lock is None:
            lock = threading.RLock()
            _PATH_LOCKS[key] = lock
        return lock

