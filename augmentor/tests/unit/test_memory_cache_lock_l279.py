# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

r"""L279 / A373：`MemoryCache` 的公开入口必须罩在同一把实例锁上

**缺陷形（改前实测读数，探针在仓外 `Temp/l277_probes/_l279_memo_hook.py`）**
`augmentor/cache.py` 里 `_synchronized` 改前把 `MemoryCache` 写成豁免面，理由逐字是
「后者只碰进程内的 dict/计数器，竞态最坏结果是少统计一次」。L279 把这句话证伪：
把 `self._cache` 换成一只在指定那一步停住的 dict 替身，两个缺陷都是**必现异常**，
不是少统计——

- `get`：`if key in self._cache: entry = self._cache[key]` 是 check-then-act。A 的
  `in` 已经返回 True 并停在临界区里，B 删掉同一枚键，A 复跑第二次读当场
  `KeyError: 'a'`。
- `_evict`：`sorted(self._cache.keys(), key=lambda k: self._cache[k].hit_count)` 先取
  键名单再逐个回读。名单里任何一键在这两步之间被别人删掉，`sorted` 里当场
  `KeyError: 'b'`。

概率锤（8 线程 × 4000 轮）在 0.02 秒内一格都打不出来 —— 窗口只有几条字节码宽，
所以取证用确定性钩子，不靠循环撞。

**产品可达**：`augmentor/models/base.py:183` 给每只模型构造**一只**共享的
`_generation_cache`，`_cache_get` / `_cache_put` 每次 `generate()` 都走它；
`augmentor/context.py:175` 的 `ThreadPoolExecutor` 把多笔增强任务同时打进来。
一次 `KeyError` 的后果不是「统计少一格」，而是把一笔**已成功的生成记成失败条目**
（`pipeline.py` 的 `except` 支把它写进本批错误清单）。

**三面**
① 形状名册（AST）：`MemoryCache` 七个公开入口逐个带 `@_synchronized`；`__init__` 与
   `_evict` 不带，且 `_evict` 的 docstring 必须写明「调用方持锁」；`_synchronized` 的
   定义必须落在 `class MemoryCache` 之前（否则 import 期就找不到装饰器）；
   `DiskCache` 同一名册一格不许退化；可达性前提两格（模型侧单例、线程池）现量钉住。
② 行为对账：真锁下 A 停在临界区内时 B 必须**进不去**（等 0.25 秒仍未完成就是互斥的
   直接证据），放行后两臂零异常、终态自洽。
③ 反证与尺子自证：把 `self._lock` 换成 no-op 假锁，必须原样重现两形 `KeyError`；
   把摘掉装饰器的合成语料喂给形状判据必须整族点名红，换成一把不相干的锁同样判红。
"""

import ast
import os
import threading

from augmentor.cache import CacheEntry, MemoryCache

REPO = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
CACHE_PY = os.path.join(REPO, "augmentor", "cache.py")
BASE_PY = os.path.join(REPO, "augmentor", "models", "base.py")
CONTEXT_PY = os.path.join(REPO, "augmentor", "context.py")

#: `MemoryCache` 的公开入口 —— 七格全罩，一格不许漏
MEMO_LOCKED = {"__len__", "__contains__", "get", "set", "delete", "clear", "stats"}
#: 类内其余方法：`__init__` 是建锁的地方，`_evict` 由持锁的 `set` 调用
MEMO_PLAIN = {"__init__", "_evict"}
#: `DiskCache` 的同一族入口（L277 落地），本轮一格未动
DISK_LOCKED = {"get", "set", "delete", "clear", "stats"}


def _read(path):
    with open(path, encoding="utf-8") as fh:
        return fh.read()


def _class_node(source, class_name):
    tree = ast.parse(source)
    for node in tree.body:
        if isinstance(node, ast.ClassDef) and node.name == class_name:
            return node
    raise AssertionError("类不在语料里：%s" % class_name)


def _locked_names(source, class_name):
    """该类里带 `@_synchronized` 的方法名集合"""
    out = set()
    for item in _class_node(source, class_name).body:
        if not isinstance(item, ast.FunctionDef):
            continue
        if any(isinstance(d, ast.Name) and d.id == "_synchronized" for d in item.decorator_list):
            out.add(item.name)
    return out


def _shape_verdict(source, class_name, expected):
    """形状判据：返回点名红的入口（空集 = 判绿）。漏罩与多罩都算红。"""
    return _locked_names(source, class_name) ^ expected


def _spans(source):
    """切成 (头, `MemoryCache` 类体, 尾) —— 类体锚在 class 行与下一只类行之间"""
    i = source.index("class MemoryCache:")
    j = source.index("class DiskCache:")
    return source[:i], source[i:j], source[j:]


class HookDict(dict):
    """在指定那一步把 A 线程停住，给 B 留出一个确定的插入窗口

    只咬第一次命中（`_armed`）：A 停住期间 B 走的是同一批代码路径，每次都咬就变成
    自锁等待，量不到原缺陷。`__contains__` 的停车点在**取值之后** —— 忠实复刻「`in`
    已经返回 True，随后键被别人删掉」这一序；`__getitem__` 的停车点在取值之前，
    复刻「键名单已取回，回读时键没了」。
    """

    def __init__(self, *args, watch=None, hook="contains", **kwargs):
        super().__init__(*args, **kwargs)
        self.watch = watch
        self.hook = hook
        self.reached = threading.Event()
        self.resume = threading.Event()
        self._armed = True

    def _park_after(self, result, key):
        if self._armed and key == self.watch:
            self._armed = False
            self.reached.set()
            if not self.resume.wait(2.0):
                raise TimeoutError("钩子等不到放行信号")
        return result

    def __contains__(self, key):
        result = dict.__contains__(self, key)
        if self.hook == "contains":
            return self._park_after(result, key)
        return result

    def __getitem__(self, key):
        if self.hook == "getitem" and self._armed and key == self.watch:
            self._armed = False
            self.reached.set()
            if not self.resume.wait(2.0):
                raise TimeoutError("钩子等不到放行信号")
        return dict.__getitem__(self, key)


class NoOpLock:
    """假锁：只提供 with 协议，不提供互斥。反证臂用它顶掉真锁。"""

    def __enter__(self):
        return self

    def __exit__(self, *exc):
        return False


def _entry(key):
    return CacheEntry(key=key, value="v-" + key, created_at=0.0, ttl=None)


def _run_against_hook(hook, lock=None):
    """A 线程走 `hook` 那一步并停住；B 线程删同一枚键

    hook="contains" ⇒ A 走 `get("a")`，B 删 "a"；
    hook="getitem"  ⇒ A 走满仓后的 `set("c")`（进 `_evict` 的 sorted 支），B 删 "b"。
    返回 (异常读数, B 是否在 A 停住期间完成, cache, backing)。
    """
    watched = "a" if hook == "contains" else "b"
    backing = HookDict(watch=watched, hook=hook)
    cache = MemoryCache(max_size=2)
    cache._cache = backing
    if lock is not None:
        cache._lock = lock
    for key in ("a", "b"):
        backing[key] = _entry(key)

    errors = {}
    b_done = threading.Event()

    def worker_a():
        try:
            if hook == "contains":
                cache.get(watched)
            else:
                cache.set("c", "new", ttl=None)
        except Exception as exc:  # noqa: BLE001 - 读数原样交给断言
            errors["A"] = repr(exc)

    def worker_b():
        try:
            cache.delete(watched)
        except Exception as exc:  # noqa: BLE001
            errors["B"] = repr(exc)
        finally:
            b_done.set()

    ta = threading.Thread(target=worker_a)
    ta.start()
    assert backing.reached.wait(2.0), "钩子没咬住，A 一路跑完了"
    tb = threading.Thread(target=worker_b)
    tb.start()
    b_finished_while_parked = b_done.wait(0.25)
    backing.resume.set()
    ta.join(2.0)
    tb.join(2.0)
    assert not ta.is_alive() and not tb.is_alive(), "线程没退出，读数不可信"
    return errors, b_finished_while_parked, cache, backing


class TestThePublicEntriesAreLocked:
    """① 形状名册：谁该罩、谁不该罩、装饰器定义在哪"""

    def setup_method(self):
        self.source = _read(CACHE_PY)

    def test_memory_cache_roster_is_exact(self):
        assert _shape_verdict(self.source, "MemoryCache", MEMO_LOCKED) == set()
        got = {item.name for item in _class_node(self.source, "MemoryCache").body
               if isinstance(item, ast.FunctionDef)}
        assert got == MEMO_LOCKED | MEMO_PLAIN, sorted(got)

    def test_evict_is_plain_and_declares_the_callers_lock(self):
        names = _locked_names(self.source, "MemoryCache")
        assert "_evict" not in names, "双重取锁要靠可重入才不自锁，名册也就读不出来"
        evict = [item for item in _class_node(self.source, "MemoryCache").body
                 if isinstance(item, ast.FunctionDef) and item.name == "_evict"][0]
        assert "持锁" in (ast.get_docstring(evict) or ""), "「调用方持锁」的口径没写进源码"

    def test_the_decorator_is_defined_before_the_class_that_uses_it(self):
        assert self.source.index("def _synchronized") < self.source.index("class MemoryCache:"), \
            "装饰器定义晚于类 ⇒ import 期 NameError"

    def test_disk_cache_roster_did_not_regress(self):
        assert _shape_verdict(self.source, "DiskCache", DISK_LOCKED) == set()

    def test_the_exemption_claim_is_now_recorded_as_refuted(self):
        assert "装的是 `DiskCache` 而不是 `MemoryCache`" not in self.source, \
            "豁免面那一格还在给 `MemoryCache` 开路"
        assert "少统计一次" in self.source, "那句假话要以「被证伪」的口径留在案上，不许抹掉"

    def test_reachability_premise_is_still_wired(self):
        """共享单例 + 线程池两格前提：它们塌了，本文件的缺陷面就退回理论

        模型侧是「每实例一只、每次调用都读同一格」：`base.py:183` 建，`:332~333` 是
        `_cache_put` 的惰性首用兜底，两处都落在 `self._generation_cache` 同一格上。
        """
        base_src = _read(BASE_PY)
        ctx_src = _read(CONTEXT_PY)
        assert base_src.count('getattr(self, "_generation_cache", None)') == 2, \
            "读写点不再共引同一格 ⇒ 共享面没了"
        assert base_src.count("self._generation_cache = ") == 2
        assert base_src.count("MemoryCache(max_size=self._GENERATION_CACHE_MAX)") == 2
        assert "ThreadPoolExecutor" in ctx_src


class TestTheLockReallyIsMutual:
    """② 真锁两臂：A 停在临界区里时 B 进不去，放行后双方都拿到自己的结果"""

    def test_get_critical_section_excludes_the_deleter(self):
        errors, b_finished, cache, backing = _run_against_hook("contains")
        assert b_finished is False, "A 还停在临界区里 B 就删完了 —— 这一格没罩住"
        assert errors == {}, errors
        assert "a" not in dict(backing), "B 的删除应当照常生效"
        assert cache.stats["size"] == 1

    def test_evict_critical_section_excludes_the_deleter(self):
        errors, b_finished, cache, backing = _run_against_hook("getitem")
        assert b_finished is False, "`_evict` 的键名单回读没罩在锁里"
        assert errors == {}, errors
        assert "b" not in dict(backing)
        assert "c" in dict(backing), "A 的写入被 B 挤掉了却没出声"


class TestTheFakeLockReproducesTheCrash:
    """③ 反证：顶掉互斥，两形 `KeyError` 必须原样回来"""

    def test_get_reads_a_key_the_other_thread_deleted(self):
        errors, b_finished, cache, backing = _run_against_hook("contains", lock=NoOpLock())
        assert b_finished is True, "假锁下 B 仍被挡住 ⇒ 反证臂没生效"
        assert errors.get("A") == "KeyError('a')", errors

    def test_evict_reads_a_key_the_other_thread_deleted(self):
        errors, b_finished, cache, backing = _run_against_hook("getitem", lock=NoOpLock())
        assert b_finished is True, "假锁下 B 仍被挡住 ⇒ 反证臂没生效"
        assert errors.get("A") == "KeyError('b')", errors


class TestTheShapeRulerHasTeeth:
    """③ 尺子自证：摘装饰器、换不相干的锁，判据都要整族点名"""

    def setup_method(self):
        self.source = _read(CACHE_PY)

    def test_unlocked_shape_is_named_site_by_site(self):
        stripped = "\n".join(ln for ln in self.source.split("\n")
                             if ln.strip() != "@_synchronized")
        assert stripped != self.source, "改写没落地，这一臂等于恒真"
        assert _shape_verdict(stripped, "MemoryCache", MEMO_LOCKED) == MEMO_LOCKED

    def test_foreign_lock_is_named_site_by_site(self):
        swapped = self.source.replace("@_synchronized", "@_not_the_same_lock")
        assert _shape_verdict(swapped, "MemoryCache", MEMO_LOCKED) == MEMO_LOCKED, \
            "换一把别的锁不算罩住：判据读的是装饰器身份，不是「有没有装饰器」"

    def test_a_partial_strip_is_named_by_name(self):
        """摘掉其中两格的装饰器，判据必须逐格点出 `get` 与 `clear`，不是只报「有缺口」

        摘的动作限定在 `MemoryCache` 的类体内：`DiskCache` 里同样的
        `@_synchronized` + `def get(` 相邻两行是逐字节相同的，全文件替换会一次命中两格。
        """
        head, memo, tail = _spans(self.source)
        partial = memo
        for name in ("get", "clear"):
            anchor = "    @_synchronized\n    def %s(" % name
            assert partial.count(anchor) == 1, anchor
            partial = partial.replace(anchor, "    def %s(" % name)
        assert _shape_verdict(head + partial + tail, "MemoryCache", MEMO_LOCKED) \
            == {"get", "clear"}
        assert _shape_verdict(head + partial + tail, "DiskCache", DISK_LOCKED) == set(), \
            "摘错了面：DiskCache 的格子被这一臂顺带动了"
