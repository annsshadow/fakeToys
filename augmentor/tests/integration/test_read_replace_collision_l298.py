# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""L298 / A356①：HTTP 读侧撞上写侧 `os.replace` 那一瞬的 `PermissionError` 折成 500

**已知形状**（L273 改前读数里第一次观测到，A356 立账时只记了读数、没写探针）：Windows 上
`os.replace` 走 `MoveFileExW(REPLACE_EXISTING)`，它替换目标的那一瞬与裸 `open()` 互斥；
读者撞上就拿到 `PermissionError [Errno 13]`，路由的兜底分支把它折成 **500**，响应体是
固定文案 `INTERNAL_ERROR_DETAIL`，原文只进服务端日志。POSIX 的 `rename` 不因打开着的句柄
失败，所以这是**平台特定的瞬时失败**，不是逻辑缺陷。

**本轮只补尺子、不改产品**（A356 候选①）：读侧要不要重试（② 改延迟与吞吐）、瞬时 OS
失败算 500 还是 503 / 429（③ 改对外状态码契约，与 A280 同批）都待拍，本轮一格不碰。

**探针怎么做到不 flaky**：真实窗口住在 `MoveFileExW` 内部，拿不到也撑不住，所以分两臂：

1. **机制臂（是真的）**：读者持有 `open()` 句柄时，`os.replace` 到同一目标必被拒
   （`PermissionError` / `EACCES`），句柄关掉后同一份临时件照常换上。这是 Windows 上
   「读句柄挡住替换」的精确形状，确定性复现。
2. **读侧臂（同形替身）**：用 `msvcrt.locking` 锁住目标文件的字节范围，另一支裸 `open()`
   当场拿 `PermissionError(13)` —— 与 L273 观测到的读侧异常**同一个 Python 形状**
   （同为 `EACCES`），把「这一瞬读不到」变成可持有的状态。替身不是 `MoveFileExW` 本身，
   docstring 在这里写明，避免下一轮把它当成机制证明。

**产品面两臂**：`GET /api/data/load/{filename}` 与 `PUT /api/data/update/{filename}` 在
各自的拒绝窗口里都回 **500 + 固定文案**，响应体不含 `Permission` / `Errno 13` / 部署路径，
原文进日志；窗口解除后同一请求转绿，GET 内容不变、失败过的 PUT 盘上仍是旧内容（原子写
承诺的那一半没被破坏）。

**结构臂**：`api/deps.py::read_items` 仍是裸 `open`，不经 `path_lock`、无重试循环；
`_read_items_cached` 也不取锁；而写侧 `mutate_json_file._read_modify_write` 的读-改-写整段
在 `path_lock` 内。这条尺子钉的是 **A356 的窗口本身**：将来谁给读侧加锁 / 重试，这里会红，
提醒把账本 A356 与对外语义一起更新 —— 红是设计，不是回归。

跑法：`python -m pytest tests/integration/test_read_replace_collision_l298.py -q --no-cov`
"""

from __future__ import annotations

import ast
import asyncio
import errno
import json
import logging
import os
import sys
from contextlib import contextmanager
from pathlib import Path
from typing import Dict, Iterator, List, Optional, Tuple

import httpx
import pytest

REPO = Path(__file__).resolve().parents[2]
if str(REPO) not in sys.path:
    sys.path.insert(0, str(REPO))

from api import deps  # noqa: E402
from api.deps import INTERNAL_ERROR_DETAIL  # noqa: E402
from api.main import app  # noqa: E402

#: Windows 专属面：POSIX 的 `rename` 不因打开着的句柄失败，机制臂与同形替身都不成立。
#: 结构臂与平台无关，单独跑。
WINDOWS_ONLY = pytest.mark.skipif(
    os.name != "nt", reason="Windows 的 MoveFileExW 与句柄共享语义是 A356 的前提"
)

#: 写路由的鉴权契约（与 L273 同一把钥匙）：`WriteTransportGate` 对非受信传输的匿名写
#: 请求在进路由之前就回 403，量不到 `os.replace` 那一格。
WRITE_KEY = "l298-read-replace-key"

ITEMS: List[Dict] = [
    {"instruction": f"问句 {i}", "output": f"答句 {i}"} for i in range(8)
]


def _seed(tmp_path: Path, name: str = "read_replace.json") -> Path:
    target = tmp_path / name
    target.write_text(json.dumps(ITEMS, ensure_ascii=False), encoding="utf-8")
    return target


def _request(method: str, url: str, **kwargs):
    """一次 ASGI 请求；写方法带鉴权头，读方法不带（与生产契约一致）"""
    headers = {"X-API-Key": WRITE_KEY} if method.upper() in {"POST", "PUT", "DELETE"} else {}

    async def go():
        async with httpx.AsyncClient(
            transport=httpx.ASGITransport(app=app),
            base_url="http://t",
            headers=headers,
        ) as client:
            return await client.request(method, url, **kwargs)

    return asyncio.run(go())


@contextmanager
def _read_denied(path: Path) -> Iterator[None]:
    """把「这一瞬读不到」做成可持有的状态：字节范围锁让另一支句柄的 `open()` 拿 EACCES

    `msvcrt.locking` 锁的是文件区域而不是共享模式，但两支句柄在**同一进程内**同样互斥
    （实测：锁住 5 字节后 `open(path).read()` 逐字 `PermissionError(13, 'Permission denied')`）。
    选它而不是 `ctypes` 共享模式，是因为这条探针只需要「同形错误」，不需要模拟 Win32 的
    共享位；锁的长度取整个文件，避免读到大半才撞上而变成「读到半份」的另一种形状。
    """
    import msvcrt

    handle = open(path, "r+b")
    size = max(1, os.fstat(handle.fileno()).st_size)
    try:
        msvcrt.locking(handle.fileno(), msvcrt.LK_NBLCK, size)
        try:
            yield
        finally:
            handle.seek(0)
            msvcrt.locking(handle.fileno(), msvcrt.LK_UNLCK, size)
    finally:
        handle.close()


# ============ 一、机制臂：Windows 上读句柄挡住替换 / 锁住区域挡住裸读 ============


@WINDOWS_ONLY
def test_a_reader_handle_denies_os_replace_until_it_is_closed(tmp_path):
    """读者持有目标句柄时，`os.replace` 必被拒；关掉后同一份临时件照常换上

    这是 A356 机制里**精确可复现**的那一半：Windows 的替换要求目标没有未共享删除的
    打开句柄。产品写侧的 `_replace_with_retry` 会对此做 5 次 × 20 ms 的有界重试，但
    重试只吸收瞬时撞车，不吸收「有人一直开着」——后者最终仍以 `PermissionError` 上抛，
    由路由折成 500（下面 PUT 那一臂现量）。
    """
    target = _seed(tmp_path)
    staged = tmp_path / "staged.json"
    staged.write_text(json.dumps([{"instruction": "新内容"}], ensure_ascii=False), encoding="utf-8")
    original = target.read_text(encoding="utf-8")

    with open(target, encoding="utf-8") as held:
        with pytest.raises(PermissionError) as caught:
            os.replace(staged, target)
        assert caught.value.errno == errno.EACCES
        # 句柄还在时旧内容仍读得到：拒绝的是「换名」，不是「读」
        held.seek(0)
        assert held.read() == original
        # 另一个读者也不受读句柄影响（读-读共享），被挡住的只有替换
        assert target.read_text(encoding="utf-8") == original

    os.replace(staged, target)
    assert json.loads(target.read_text(encoding="utf-8")) == [{"instruction": "新内容"}]


@WINDOWS_ONLY
def test_a_denied_read_raises_the_same_permission_error_shape(tmp_path):
    """同形替身：锁住目标区域后，裸 `open()` 逐字拿 `PermissionError(13)`；解锁即恢复

    这条钉的是**读侧异常的形状**（`PermissionError` / `EACCES`），而不是 `MoveFileExW`
    的实现细节。它保证下面两条产品臂量到的是「读被 OS 拒绝」这一族，而不是我们手工
    构造的别的异常。
    """
    target = _seed(tmp_path)
    with _read_denied(target):
        with pytest.raises(PermissionError) as caught:
            target.read_text(encoding="utf-8")
        assert caught.value.errno == errno.EACCES

    assert json.loads(target.read_text(encoding="utf-8")) == ITEMS


# ============ 二、产品面：GET / PUT 在拒绝窗口里都折成固定 500，窗口过后转绿 ============


@WINDOWS_ONLY
def test_get_load_folds_a_denied_read_into_the_fixed_500(tmp_path, monkeypatch, caplog):
    """`GET /api/data/load` 撞上读拒绝 ⇒ 500 + 固定文案；原文只进日志；解锁后同请求 200

    判据分三格：① 状态码与响应体逐字（不泄漏 `Permission` / `Errno 13`）；② 日志里
    确实有那一支 `PermissionError`（证明固定文案不是靠吞异常换来的）；③ 窗口解除后
    同一请求拿回原数据（证明失败没有被缓存成「这份文件坏了」）。
    """
    monkeypatch.setenv("AUGMENTOR_DATA_ROOTS", str(tmp_path))
    name = _seed(tmp_path).name

    with _read_denied(tmp_path / name):
        with caplog.at_level(logging.ERROR):
            denied = _request(
                "GET", f"/api/data/load/{name}", params={"page": 1, "page_size": 3}
            )

    assert denied.status_code == 500, denied.text
    assert denied.json() == {"detail": INTERNAL_ERROR_DETAIL}
    assert "Permission" not in denied.text
    assert "Errno 13" not in denied.text
    assert str(tmp_path) not in denied.text
    logged = [
        record for record in caplog.records
        if record.exc_info and isinstance(record.exc_info[1], PermissionError)
    ]
    assert logged, [record.getMessage() for record in caplog.records]
    assert logged[0].exc_info[1].errno == errno.EACCES

    ok = _request("GET", f"/api/data/load/{name}", params={"page": 1, "page_size": 3})
    assert ok.status_code == 200, ok.text
    body = ok.json()
    assert body["total"] == len(ITEMS)
    assert [item["instruction"] for item in body["items"]] == [
        f"问句 {i}" for i in range(3)
    ]


@WINDOWS_ONLY
def test_put_update_folds_a_denied_replace_into_the_fixed_500(tmp_path, monkeypatch, caplog):
    """`PUT /api/data/update` 撞上替换拒绝 ⇒ 500 + 固定文案，盘上仍是旧内容；关句柄后成功

    这一臂是**真的**：读者（本用例）持有读句柄，写侧读完、改完、写临时件，最后
    `os.replace` 被拒。它同时钉住原子写的失败语义——临时件被清掉、目标保持进入时的
    内容，不许出现「回了 500 但盘上已经是半份」或「回了 500 却其实写成功了」。
    """
    monkeypatch.setenv("AUGMENTOR_DATA_ROOTS", str(tmp_path))
    monkeypatch.setenv(deps.API_KEY_ENV, WRITE_KEY)
    target = _seed(tmp_path)
    name = target.name
    before = target.read_text(encoding="utf-8")

    with open(target, encoding="utf-8") as held:
        with caplog.at_level(logging.ERROR):
            denied = _request(
                "PUT",
                f"/api/data/update/{name}",
                params={"index": 0},
                json={"instruction": "改写 0", "output": "答句 0"},
            )
        held.seek(0)
        assert held.read() == before

    assert denied.status_code == 500, denied.text
    assert denied.json() == {"detail": INTERNAL_ERROR_DETAIL}
    assert "Permission" not in denied.text
    assert "Errno 13" not in denied.text
    logged = [
        record for record in caplog.records
        if record.exc_info and isinstance(record.exc_info[1], PermissionError)
    ]
    assert logged, [record.getMessage() for record in caplog.records]

    # 回 500 之后盘上仍是进入时的内容：原子写的「失败保持原样」没有被这一次撞车撕开
    assert target.read_text(encoding="utf-8") == before
    leftovers = sorted(p.name for p in tmp_path.glob(".*.tmp-*"))
    assert leftovers == [], leftovers

    retried = _request(
        "PUT",
        f"/api/data/update/{name}",
        params={"index": 0},
        json={"instruction": "改写 0", "output": "答句 0"},
    )
    assert retried.status_code == 200, retried.text
    after = json.loads(target.read_text(encoding="utf-8"))
    assert after[0]["instruction"] == "改写 0"
    assert after[1:] == ITEMS[1:]


# ============ 三、结构臂：读侧没有锁也没有重试，写侧 RMW 有锁 ============


def _defs(tree: ast.AST) -> Dict[str, ast.AST]:
    """按名字收集函数定义（含嵌套函数，`_read_modify_write` 住在这里）"""
    return {
        node.name: node
        for node in ast.walk(tree)
        if isinstance(node, (ast.FunctionDef, ast.AsyncFunctionDef))
    }


def _called_names(node: ast.AST) -> List[str]:
    names: List[str] = []
    for sub in ast.walk(node):
        if isinstance(sub, ast.Call):
            func = sub.func
            if isinstance(func, ast.Name):
                names.append(func.id)
            elif isinstance(func, ast.Attribute):
                names.append(func.attr)
    return names


def _has_loop(node: ast.AST) -> bool:
    return any(isinstance(sub, (ast.For, ast.While)) for sub in ast.walk(node))


def test_the_ruler_itself_recognises_lock_and_retry():
    """尺子的牙齿：合成本里认得出 `path_lock` 与重试循环，避免「查不到是因为不会查」"""
    locked = ast.parse(
        "def f(p):\n"
        "    with path_lock(p):\n"
        "        return open(p)\n"
    )
    retrying = ast.parse(
        "def f(p):\n"
        "    for _ in range(5):\n"
        "        try:\n"
        "            return open(p)\n"
        "        except PermissionError:\n"
        "            pass\n"
    )
    assert "path_lock" in _called_names(_defs(locked)["f"])
    assert _has_loop(_defs(retrying)["f"])


def test_get_read_path_has_no_lock_and_no_retry_while_rmw_has_the_lock():
    """A356 的窗口就在这三格：`read_items` 裸读、缓存层不取锁、GET 直接走它们

    这不是「忘了加锁」的断言，而是**当前产品面**的断言：锁只覆盖写侧读-改-写整段，
    HTTP 读侧与 `os.replace` 之间仍开着窗。谁给读侧加锁或重试，本支会红 —— 那时必须
    同时更新账本 A356 与对外语义（延迟 / 吞吐 / 状态码），红是提醒而不是回归。
    """
    source = (REPO / "api" / "deps.py").read_text(encoding="utf-8")
    functions = _defs(ast.parse(source))

    read_items = functions["read_items"]
    cached = functions["_read_items_cached"]
    rmw = functions["_read_modify_write"]

    assert "path_lock" not in _called_names(read_items), "读侧开始取锁了：A356 待重新拍板"
    assert not _has_loop(read_items), "读侧出现重试循环：A356 候选② 已被实现，账本要更新"
    assert "path_lock" not in _called_names(cached), "缓存层开始取锁了：A356 待重新拍板"

    # 对照：写侧的读-改-写整段确实在同一把按路径锁里（L273 的修法仍在原位）
    assert "path_lock" in _called_names(rmw)
    assert {"read_items", "atomic_write_json"} <= set(_called_names(rmw))
    # GET 的异步入口走线程里的缓存读，不经过 `mutate_json_file`
    assert "to_thread" in _called_names(functions["read_json_file"])
