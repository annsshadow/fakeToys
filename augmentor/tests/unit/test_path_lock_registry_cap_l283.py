# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""L283：`atomic_write.path_lock` 的锁表必须有一条**被代码 enforce** 的上限

存在理由（`Temp/probe_l283_locks.py` 实测）：`_PATH_LOCKS` 按路径单调增长、**没有任何东西
回收**，而它的键来自请求方能点名的数据集文件名。改之前那句「一个进程碰过的文件数量是有界
的」只写在 docstring 里，没有 enforce 它的那段代码 —— 实测登记 20 000 个不同路径占
**6.25 MB**（每格约 312 B），且把路径对象全部丢弃、文件本身也不存在之后，**一格都不退**。
这不是「读侧拿到坏答案」那一族，是长跑进程被外部可点名的键慢慢吃掉内存那一族，所以修法与
`api/middleware/rate_limit.py` 的 `DEFAULT_MAX_KEYS` 对齐：封顶，不回收。

为什么不回收而是共用一把 spill 锁：eviction 必然在「取出锁对象」与「进入 `with`」之间开窗
—— A 取到锁还没进，B 把这一格删掉，C 给同一路径建出第二把锁，两边并发写同一份文件而两边都
回 `{"success": true}`，正是 L273 那族丢更新的形状。共用 spill 锁只会**多**串行化，永远不会
放宽，所以它是安全的那一侧。

本守卫断的可判定集合：
1. 登记数永不超过 `MAX_PATH_LOCKS`（用真上限跑一次溢出，钉住「超限即不再登记」）；
2. 上限之内的老口径不破 —— 同一路径 `is` 同一把锁、不同路径不同锁、绝对与相对两种形状
   （含 `..` 段）折叠成同一把，否则锁只是装饰；
3. 超限之后拿到的是同一把 spill 锁，且**行为差**成立：两个不同文件的读-改-写仍然互斥；
4. spill 锁仍是可重入锁 —— `api/deps.py` 的整段读-改-写与工作线程里的同步写者都直接用
   `with` / `acquire`，`tests/integration/test_rmw_serialization_l273.py` 还钉了返回类型。

注入臂把 `MAX_PATH_LOCKS` 顶成 `sys.maxsize` —— 等价于摘掉封顶那两行 —— 证明「登记数不越
界」这条断言真的依赖那段代码，而不是恒真。全程零写入：只在导入后的模块对象上打补丁，产品
文件一字未改。
"""

import os
import sys
import threading

import pytest

from augmentor import atomic_write as aw


@pytest.fixture(autouse=True)
def pristine_registry():
    """进出各一次快照恢复全局锁表 —— 本文件会登记上千个探针键，不能留给别的用例"""
    saved = dict(aw._PATH_LOCKS)
    aw._PATH_LOCKS.clear()
    yield
    aw._PATH_LOCKS.clear()
    aw._PATH_LOCKS.update(saved)


def _probe_key(i: int) -> str:
    """造一个必然不与真实数据集重名、且每次都不一样的锁键（不落盘）"""
    return f"l283_probe_{i}.json"


def _regkey(name: str) -> str:
    return os.path.normcase(os.path.abspath(name))


def test_cap_constant_is_pinned():
    """上限值本身钉住：调它要连带重测内存代价，不许顺手放宽

    `path_lock` 每登记一格约 312 B（实测 20 000 格 = 6.25 MB）⇒ 10 000 格的封顶把这一族的
    最坏驻留钉在约 3 MB 量级。
    """
    assert aw.MAX_PATH_LOCKS == 10000
    assert aw.MAX_PATH_LOCKS > 0


def test_registry_never_exceeds_cap():
    """真上限下跑一次溢出：登记数恰好等于上限，多出来的格一格都不进表"""
    cap = aw.MAX_PATH_LOCKS
    for i in range(cap + 500):
        aw.path_lock(_probe_key(i))

    assert len(aw._PATH_LOCKS) == cap
    for i in range(cap):
        assert _regkey(_probe_key(i)) in aw._PATH_LOCKS
    for i in range(cap, cap + 500):
        assert _regkey(_probe_key(i)) not in aw._PATH_LOCKS


def test_within_cap_identity_still_holds(tmp_path):
    """上限之内，旧口径一字不让：同一路径同一把锁，不同路径不同锁"""
    same = tmp_path / "data.json"
    dotted = os.path.join(str(tmp_path), "..", tmp_path.name, "data.json")
    assert dotted != str(same)
    a = aw.path_lock(str(same))
    b = aw.path_lock(same)
    c = aw.path_lock(dotted)
    assert a is b is c
    assert a is not aw.path_lock(tmp_path / "other.json")


def test_spill_lock_is_shared_and_reentrant(monkeypatch):
    """超限之后拿到的是同一把 spill 锁，且它仍是可重入锁

    用小上限跑这一段：可重入性断的是「同一线程套同一把锁不死锁」，与表有多大无关。
    """
    monkeypatch.setattr(aw, "MAX_PATH_LOCKS", 3)
    locks = [aw.path_lock(_probe_key(i)) for i in range(20)]
    assert len({id(x) for x in locks[:3]}) == 3
    assert all(x is aw._SPILL_LOCK for x in locks[3:])

    with aw._SPILL_LOCK:
        with aw._SPILL_LOCK:
            pass
    assert aw._SPILL_LOCK.acquire(timeout=0)
    assert aw._SPILL_LOCK.acquire(timeout=0)
    aw._SPILL_LOCK.release()
    aw._SPILL_LOCK.release()


def test_overflow_paths_still_serialize_writes(monkeypatch):
    """行为差：共用 spill 锁之后，两个**不同**文件仍然互斥

    这条是「封顶而不是回收」的代价核算 —— 超限只是把并发写排成队，绝不会变成并发写同一格。
    写法取确定性形状：主线程先按住 spill 锁，另一支线程去锁第二个文件，按住期间它必须进不去。
    """
    monkeypatch.setattr(aw, "MAX_PATH_LOCKS", 1)
    aw.path_lock(_probe_key(900))  # 把表填满，之后一律走 spill
    lock_a = aw.path_lock(_probe_key(901))
    lock_b = aw.path_lock(_probe_key(902))
    assert lock_a is lock_b is aw._SPILL_LOCK

    entered = threading.Event()
    holder = threading.Thread(target=lambda: (lock_b.acquire(), entered.set(), lock_b.release()))
    with lock_a:
        holder.start()
        assert not entered.wait(timeout=0.15), "两个不同文件在 spill 锁下竟然并发了"
    holder.join(timeout=10)
    assert entered.is_set()


def test_injection_arm_no_cap_grows_past_the_ceiling(monkeypatch):
    """注入臂：把封顶支摘掉（顶成 `sys.maxsize`）⇒ 登记数越界、溢出格各得一把新锁

    `test_registry_never_exceeds_cap` 断的是「恰好等于上限、溢出格不进表」。这条臂不改任何
    产品文件，只把 `MAX_PATH_LOCKS` 顶到无界，等价于删掉 `if len(_PATH_LOCKS) >=
    MAX_PATH_LOCKS: return _SPILL_LOCK` 那两行：同一批调用之后表长到 `n` 格，且每格都是**新**
    锁而不是 spill 锁 —— 于是那两条断言同时落空。原缺陷（无界增长）因此仍能让守卫变红，尺子
    没有被收窄成恒真。
    """
    monkeypatch.setattr(aw, "MAX_PATH_LOCKS", sys.maxsize)
    n = 60
    locks = [aw.path_lock(_probe_key(i)) for i in range(n)]
    assert len(aw._PATH_LOCKS) == n
    assert all(x is not aw._SPILL_LOCK for x in locks)
    assert len({id(x) for x in locks}) == n
