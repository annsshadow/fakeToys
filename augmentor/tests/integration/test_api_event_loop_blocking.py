# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""`async def` 路由不能在事件循环里读文件

两族同源缺陷（回退清单里的 A4 与 A13）：

* `quality` / `audit` / `leakage` / `privacy` / `export` —— 路由把**分析**丢进了
  线程池（`run_in_thread(run)`），却把第一行的 `load_items()` 留在 `async def`
  函数体上；
* `dataset_tools` / `system_ops` —— 同样漏了读取，只不过用的是收「已 resolve
  路径」的同步版 `read_items()`；`/api/dataset/stats` 连分析（`calculate_statistics`）
  都留在循环上，`/api/dataset/aggregate` 更是「多个文件在循环里串行读」。

同步读文件是「stat + 打开 + `json.load`」：

- 实测仓库里 3.6 MB / 6902 条的 `train_data.json` 一次要 **15 ms**（缓存更冷的
  时候更高），这段时间整个服务的**所有**其他请求都停在事件循环上 —— 一个慢
  客户端能拖慢全站，多条这类端点并发时也不会并行，只会排队；
- 同一条路由里「分析」已经离线了，只有读取没离线，属于漏网而非设计。

预言机不靠计时（计时在 CI 上会误报），而是**线程身份**：`httpx.ASGITransport`
直接在当前线程跑事件循环，所以「读文件发生在哪个线程」是确定的 —— 只要它等于
循环线程，就是阻塞。修好后必须落在工作线程上。
"""

import asyncio
import importlib
import json
import threading
from types import SimpleNamespace
from typing import Any, Dict, List, Tuple

import httpx
import pytest

from api.main import app

_ITEMS: List[Dict[str, Any]] = [
    {"instruction": "how do i parse a json file in python?", "output": "use json.loads"},
    {"instruction": "how do i sort a list of tuples by second element?", "output": "sorted()"},
]

# (路由模块, 路径, 请求体, 预期读取次数, 查询参数)
#
# 请求体 / 查询参数可以是字面 dict，也可以是 `env -> dict`：后者用于必须真实落盘
# 的路径（见 `env` 的说明）。
BLOCKING_SITES = [
    (
        "api.routes.quality",
        "/api/quality/evaluate",
        {"input_file": "ds.json"},
        1,
        None,
    ),
    (
        "api.routes.quality",
        "/api/quality/dedup",
        {"input_file": "ds.json"},
        1,
        None,
    ),
    (
        "api.routes.quality",
        "/api/quality/report",
        {"input_file": "ds.json"},
        1,
        None,
    ),
    (
        "api.routes.quality",
        "/api/quality/clean",
        {"input_file": "ds.json"},
        1,
        None,
    ),
    (
        "api.routes.quality",
        "/api/quality/annotate",
        {"input_file": "ds.json"},
        1,
        None,
    ),
    (
        "api.routes.quality",
        "/api/quality/benchmark",
        {"input_file": "ds.json"},
        1,
        None,
    ),
    (
        "api.routes.quality",
        "/api/quality/outliers",
        {"input_file": "ds.json"},
        1,
        None,
    ),
    (
        "api.routes.quality",
        "/api/quality/profiling",
        {"input_file": "ds.json", "save": False},
        1,
        None,
    ),
    (
        "api.routes.quality",
        "/api/quality/health-gate",
        {"input_file": "ds.json"},
        1,
        None,
    ),
    (
        "api.routes.audit",
        "/api/audit",
        {"input_file": "ds.json", "reference_file": "ref.json"},
        2,
        None,
    ),
    (
        "api.routes.leakage",
        "/api/leakage/check",
        {"train_file": "train.json", "test_file": "test.json"},
        2,
        None,
    ),
    (
        "api.routes.privacy",
        "/api/privacy/sanitize",
        {"input_file": "ds.json"},
        1,
        None,
    ),
    (
        "api.routes.export",
        "/api/export/preview",
        {"input_file": "ds.json", "format": "jsonl"},
        1,
        None,
    ),
    # ---- dataset / system 组：读取前先 `resolve_data_path()`，它会确认文件存在
    #      （不存在直接 404），所以这一组的文件名必须是真的落在盘上 ----
    (
        "api.routes.dataset_tools",
        "/api/dataset/stats",
        lambda env: {"input_file": str(env.data)},
        1,
        None,
    ),
    (
        "api.routes.dataset_tools",
        "/api/dataset/search",
        lambda env: {"input_file": str(env.data), "query": "json"},
        1,
        None,
    ),
    (
        "api.routes.dataset_tools",
        "/api/dataset/compare",
        lambda env: {"dataset_a": str(env.data), "dataset_b": str(env.second)},
        2,
        None,
    ),
    (
        "api.routes.dataset_tools",
        "/api/dataset/impact",
        lambda env: {"before_file": str(env.data), "after_file": str(env.second)},
        2,
        None,
    ),
    (
        "api.routes.dataset_tools",
        "/api/dataset/evaluate",
        lambda env: {
            "generated_file": str(env.data),
            "reference_file": str(env.second),
        },
        2,
        None,
    ),
    (
        "api.routes.dataset_tools",
        "/api/dataset/features",
        lambda env: {"input_file": str(env.data)},
        1,
        None,
    ),
    (
        "api.routes.dataset_tools",
        "/api/dataset/auto-config",
        lambda env: {"input_file": str(env.data)},
        1,
        None,
    ),
    (
        "api.routes.dataset_tools",
        "/api/dataset/aggregate",
        lambda env: {
            "datasets": {"a": str(env.data), "b": str(env.second)},
            "output_file": str(env.out),
        },
        2,
        None,
    ),
    (
        "api.routes.dataset_tools",
        "/api/dataset/rag",
        lambda env: {"input_file": str(env.data), "output_file": str(env.out)},
        1,
        None,
    ),
    (
        "api.routes.system_ops",
        "/api/system/monitor",
        lambda env: {"input_file": str(env.data)},
        1,
        None,
    ),
    (
        "api.routes.system_ops",
        "/api/system/auto-test",
        lambda env: {"input_file": str(env.data)},
        1,
        None,
    ),
    (
        # 注册表目录也必须显式给：缺省值落在第一个白名单根目录下，
        # 用 `env.registry` 才不会把登记文件写进版本树。
        "api.routes.system_ops",
        "/api/system/dependency/datasets",
        lambda env: {"name": "ds1", "input_file": str(env.data)},
        1,
        lambda env: {"registry_path": str(env.registry)},
    ),
]


@pytest.fixture
def env(tmp_path):
    """真实落盘的两份数据集 + 一个输出路径 + 一个注册表目录

    这一组路由在读取之前会先 `resolve_data_path()` 校验路径（文件不存在 → 404），
    所以不能像 quality 组那样给个假名字就完事。输出与注册表都指向 `tmp_path`：
    测试绝不往仓库工作目录写产物。
    """
    data = tmp_path / "ds.json"
    data.write_text(json.dumps(_ITEMS, ensure_ascii=False), encoding="utf-8")
    second = tmp_path / "ds2.json"
    second.write_text(json.dumps(_ITEMS[:1], ensure_ascii=False), encoding="utf-8")
    return SimpleNamespace(
        data=data,
        second=second,
        out=tmp_path / "out.json",
        registry=tmp_path / "registry",
    )


def _record_reads(monkeypatch, module_name: str) -> List[Tuple[int, str]]:
    """把「同步读数据集」的入口全部换成记录器，返回 ``（线程 ident, 读的路径）`` 清单

    两处都得打，只打一处就只能看见一种状态：

    - 路由模块自己的 `load_items` / `read_items`：缺陷态下路由直接在函数体里调用
      它们（`from ..deps import read_items` 是各自命名空间的绑定），只有打在模块上
      才看得见「这次读取跑在哪个线程」；
    - `api.deps.read_items`：修好后读取一律走 `read_json_file` / `run_in_thread`，
      收口在 `deps.read_items`，此时路由模块上那个名字已经不存在了，只打模块就
      变成「什么都没测到」。
    """
    module = importlib.import_module(module_name)
    seen: List[Tuple[int, str]] = []

    def fake_read(*args, **kwargs):
        target: Any = args[0] if args else (kwargs.get("file_path") or kwargs.get("filename"))
        seen.append((threading.current_thread().ident, str(target)))
        return [dict(item) for item in _ITEMS]

    monkeypatch.setattr("api.deps.read_items", fake_read)
    for name in ("load_items", "read_items"):
        if hasattr(module, name):
            monkeypatch.setattr(module, name, fake_read)
    return seen


def _hit_off_loop_thread(
    monkeypatch,
    module_name: str,
    path: str,
    payload: Any,
    params: Any = None,
    env: Any = None,
):
    """在真实事件循环里打一次路由，返回读取记录、循环线程 ident 和响应"""
    seen = _record_reads(monkeypatch, module_name)
    body = payload(env) if callable(payload) else payload
    query = params(env) if callable(params) else params

    async def go():
        loop_thread = threading.current_thread().ident
        transport = httpx.ASGITransport(app=app)
        async with httpx.AsyncClient(transport=transport, base_url="http://svc") as client:
            response = await client.post(path, json=body, params=query)
        return loop_thread, response

    loop_thread, response = asyncio.run(go())
    return seen, loop_thread, response


@pytest.mark.parametrize("module_name,path,payload,reads,params", BLOCKING_SITES)
def test_route_reads_dataset_off_the_event_loop(
    module_name, path, payload, reads, params, monkeypatch, env
):
    """数据集读取必须发生在工作线程，而不是事件循环线程"""
    seen, loop_thread, _response = _hit_off_loop_thread(
        monkeypatch, module_name, path, payload, params, env
    )

    assert len(seen) == reads, f"{path} 读取了 {len(seen)} 次，预期 {reads} 次（路由没走到读取？）"
    on_loop = [name for idx, name in seen if idx == loop_thread]
    assert not on_loop, f"{path} 在事件循环线程上同步读取了 {on_loop}，会阻塞整个服务"


@pytest.mark.parametrize("module_name,path,payload,reads,params", BLOCKING_SITES)
def test_route_still_answers_after_offloading(
    module_name, path, payload, reads, params, monkeypatch, env
):
    """把读取挪进线程池不能改变响应：仍然要正常出结果

    只锁「不再是在循环上读文件」这一条很容易被「干脆不读了」蒙过去 —— 上一条用例的
    读取次数断言挡不住「提前 return 404」，这一条用 200 把它堵住。
    """
    _seen, _loop_thread, response = _hit_off_loop_thread(
        monkeypatch, module_name, path, payload, params, env
    )

    assert response.status_code == 200, (
        f"{path} -> {response.status_code} {response.text[:200]}"
    )


def test_stats_analysis_runs_off_the_event_loop(monkeypatch, env):
    """`/api/dataset/stats` 的**分析**也不能留在事件循环上

    这条路由除了读取，原本还把 `calculate_statistics()` 直接写在函数体里：实测
    6902 条 / 3.43 MB 的 `train_data.json` 要 **43.4 ms**，比读同一份文件的
    9.2 ms 还贵 4 倍 —— 阻塞的大头其实是分析。它是这两组路由里唯一「算完才返回、
    没有离线分析」的端点，其余端点都把重活放进了 `run_in_thread`。

    预言机仍然是线程身份。路由在函数体里 `from augmentor.statistics import
    calculate_statistics`，所以打在**库模块**上：缺陷态（直接在循环里调）和修复态
    （线程池里调）都拦得到，不会因为导入时机而漏测。
    """
    import augmentor.statistics as statistics_module

    real = statistics_module.calculate_statistics
    seen: List[int] = []

    def spy(items, *args, **kwargs):
        seen.append(threading.current_thread().ident)
        return real(items, *args, **kwargs)

    monkeypatch.setattr(statistics_module, "calculate_statistics", spy)
    _reads, loop_thread, response = _hit_off_loop_thread(
        monkeypatch,
        "api.routes.dataset_tools",
        "/api/dataset/stats",
        lambda e: {"input_file": str(e.data)},
        None,
        env,
    )

    assert response.status_code == 200, response.text
    assert len(seen) == 1, f"calculate_statistics 跑了 {len(seen)} 次，预期 1 次（没走到统计？）"
    assert seen[0] != loop_thread, "逐字段统计在事件循环线程上跑，全站请求都要为它停 43 ms"
