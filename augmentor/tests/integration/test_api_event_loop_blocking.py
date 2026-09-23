# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""`async def` 路由不能在事件循环里读文件

这些路由已经把**分析**丢进线程池（`run_in_thread(run)`），但读取数据集的
`load_items()` 仍留在 `async def` 函数体第一行。`load_items` 是同步的
「stat + 打开 + `json.load`」：

- 实测仓库里 3.6 MB / 6902 条的 `train_data.json` 一次要 **15 ms**（缓存更冷的
  时候更高），这段时间整个服务的**所有**其他请求都停在事件循环上 —— 一个慢
  客户端能拖慢全站，12 条这类端点并发时也不会并行，只会排队；
- 同一条路由里「分析」已经离线了，只有读取没离线，属于漏网而非设计。

预言机不靠计时（计时在 CI 上会误报），而是**线程身份**：`httpx.ASGITransport`
直接在当前线程跑事件循环，所以「读文件发生在哪个线程」是确定的 —— 只要它等于
循环线程，就是阻塞。修好后必须落在工作线程上。
"""

import asyncio
import importlib
import threading
from typing import Any, Dict, List, Tuple

import httpx
import pytest

from api.main import app

_ITEMS: List[Dict[str, Any]] = [
    {"instruction": "how do i parse a json file in python?", "output": "use json.loads"},
    {"instruction": "how do i sort a list of tuples by second element?", "output": "sorted()"},
]

# (路由模块, 路径, 请求体, 预期读取次数)
BLOCKING_SITES = [
    (
        "api.routes.quality",
        "/api/quality/evaluate",
        {"input_file": "ds.json"},
        1,
    ),
    (
        "api.routes.quality",
        "/api/quality/dedup",
        {"input_file": "ds.json"},
        1,
    ),
    (
        "api.routes.quality",
        "/api/quality/report",
        {"input_file": "ds.json"},
        1,
    ),
    (
        "api.routes.quality",
        "/api/quality/clean",
        {"input_file": "ds.json"},
        1,
    ),
    (
        "api.routes.quality",
        "/api/quality/annotate",
        {"input_file": "ds.json"},
        1,
    ),
    (
        "api.routes.quality",
        "/api/quality/benchmark",
        {"input_file": "ds.json"},
        1,
    ),
    (
        "api.routes.quality",
        "/api/quality/outliers",
        {"input_file": "ds.json"},
        1,
    ),
    (
        "api.routes.quality",
        "/api/quality/profiling",
        {"input_file": "ds.json", "save": False},
        1,
    ),
    (
        "api.routes.audit",
        "/api/audit",
        {"input_file": "ds.json", "reference_file": "ref.json"},
        2,
    ),
    (
        "api.routes.leakage",
        "/api/leakage/check",
        {"train_file": "train.json", "test_file": "test.json"},
        2,
    ),
    (
        "api.routes.privacy",
        "/api/privacy/sanitize",
        {"input_file": "ds.json"},
        1,
    ),
    (
        "api.routes.export",
        "/api/export/preview",
        {"input_file": "ds.json", "format": "jsonl"},
        1,
    ),
]


def _hit_off_loop_thread(monkeypatch, module_name: str, path: str, payload: Dict[str, Any]):
    """在真实事件循环里打一次路由，返回 ``load_items`` 的 (线程 ident, 文件名) 记录"""
    module = importlib.import_module(module_name)
    seen: List[Tuple[int, str]] = []

    def fake_load_items(filename, *args, **kwargs):
        seen.append((threading.current_thread().ident, str(filename)))
        return [dict(item) for item in _ITEMS]

    # 路由用的是 `from ..deps import load_items`，所以必须打在路由模块的命名空间上
    monkeypatch.setattr(module, "load_items", fake_load_items)

    async def go():
        loop_thread = threading.current_thread().ident
        transport = httpx.ASGITransport(app=app)
        async with httpx.AsyncClient(transport=transport, base_url="http://svc") as client:
            response = await client.post(path, json=payload)
        return loop_thread, response

    loop_thread, response = asyncio.run(go())
    return seen, loop_thread, response


@pytest.mark.parametrize("module_name,path,payload,reads", BLOCKING_SITES)
def test_route_reads_dataset_off_the_event_loop(module_name, path, payload, reads, monkeypatch):
    """数据集读取必须发生在工作线程，而不是事件循环线程"""
    seen, loop_thread, _response = _hit_off_loop_thread(monkeypatch, module_name, path, payload)

    assert len(seen) == reads, f"{path} 读取了 {len(seen)} 次，预期 {reads} 次（路由没走到读取？）"
    on_loop = [name for idx, name in seen if idx == loop_thread]
    assert not on_loop, f"{path} 在事件循环线程上同步读取了 {on_loop}，会阻塞整个服务"


@pytest.mark.parametrize("module_name,path,payload,reads", BLOCKING_SITES)
def test_route_still_answers_after_offloading(module_name, path, payload, reads, monkeypatch):
    """把读取挪进线程池不能改变响应：仍然要正常出结果

    只锁「不再是在循环上读文件」这一条很容易被「干脆不读了」蒙过去 —— 上一条用例的
    读取次数断言挡不住「提前 return 404」，这一条用 200 把它堵住。
    """
    _seen, _loop_thread, response = _hit_off_loop_thread(monkeypatch, module_name, path, payload)

    assert response.status_code == 200, (
        f"{path} -> {response.status_code} {response.text[:200]}"
    )
