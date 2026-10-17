# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""L290（A397）：`delete_version` / `rollback` 也得住进 L273 那把按目录的锁

**缺陷形状**：L273 把 `create_version` 整段罩进 `path_lock(self.storage_dir)`，并在注释里
写明了为什么 —— 「current 标记那一段更直接：一支线程正写着 current.txt，另一支的
`_clear_current_link` 把它 unlink 掉，Windows 当场拒绝访问」。可同一目录上的另外两处写面
`rollback()` 与 `delete_version()` 干的是同一段事（判 current、清 current、写 current、
删版本目录），却留在锁外。于是并发之下「不能删除当前版本」这条承诺破了。

* 概率式对撞量不到 —— 24 轮 × 40 轮实测 **0 异常、0 悬空**（`Temp/l290q/probe_versioning_lock.py`
  两臂读数）。窗口只有几微秒，靠撞是撞不出来的，这也反过来证明「跑过并发没坏」根本不算证据。
* 把窗口撑开就必然命中 —— 仓外探针在 `get_current_version` 与 `shutil.rmtree` 之间插一次会合
  （零写入通道：只在类对象上打补丁）：`rollback(v1)` 先完成并把 current 指向 v1，
  `delete_version(v1)` 用**它进门前读到的那份旧 current** 判「不是当前版本」，随后把 v1 连目录
  删掉。终态是 **current 指向一个已经不存在的版本目录**，而两支线程都回了成功
  （`Temp/l290q/probe_versioning_window.py` 读数 `delete_current: DANGLING-CURRENT`）。
* 修后同一支探针读数 `no-dangling`：删除与回滚被排成先后两趟，排在后面的那次按**新的** current
  与**新的**目录存在性判决 —— `rollback` 回 `False` 并记下「版本不存在」，current 仍指向活着的版本。

修法与 L273 / L289 同一形状：一把按 `storage_dir` 建键的可重入锁，罩住整段 check-then-act。
不改成「先删再判」那种局部巧法：那只是把窗口挪个位置，current 标记那一段照旧会撞。

本文件四面：一、结构面（两处写点各进锁一次，且判据与动手落在同一段跨度里）；二、行为面
（撑开窗口的确定性对撞，真假两把锁给出两个不同终态）；三、承诺面（`delete_version` 删当前
版本仍报 `VersionError`、`rollback` 回不存在版本仍返回 `False`，加锁之后一档都没松）；
四、红能力（把 `path_lock` 换成「每次给一把新锁」，悬空 current 必须回来）。
"""

import ast
import json
import shutil
import threading
from pathlib import Path
from typing import Dict, List, Tuple

import pytest

from augmentor.exceptions import VersionError
from augmentor.versioning import VersionManager

ROOT = Path(__file__).resolve().parents[2]
SRC = (ROOT / "augmentor" / "versioning.py").read_text(encoding="utf-8")
TREE = ast.parse(SRC)

ITEMS: List[Dict] = [{"instruction": "问句", "output": "答句"}]

#: 等待回滚线程「跑完 / 被挡住」的那一格时限。真锁下回滚拿不到锁，必然超时；替身锁下
#: 回滚立刻跑完，必然在时限内被观察到。两支的判据都只依赖「锁在不在」，不依赖机器速度。
WINDOW_S = 0.35

#: 两处写点各自必须落在锁段里的动作（方法名 → 跨度内必须出现的调用）。挑这几个而不是
#: 随便一行：它们正是 check-then-act 的两端 —— 判据与动手必须同趟。
PROTECTED_ACTIONS = {
    "delete_version": ("get_current_version", "rmtree"),
    "rollback": ("_clear_current_link", "symlink_to"),
}


def _method(name: str) -> ast.FunctionDef:
    """取 `VersionManager` 里那一方法的 AST"""
    cls = next(n for n in ast.walk(TREE)
               if isinstance(n, ast.ClassDef) and n.name == "VersionManager")
    return next(n for n in cls.body
                if isinstance(n, ast.FunctionDef) and n.name == name)


def _lock_spans(fn: ast.FunctionDef) -> List[Tuple[int, int]]:
    """`fn` 内每个 `with path_lock(...)` 体的行范围（与 l273 尺子同一算法）"""
    spans: List[Tuple[int, int]] = []
    for n in ast.walk(fn):
        if not isinstance(n, ast.With):
            continue
        if not any("path_lock" in ast.unparse(item.context_expr) for item in n.items):
            continue
        starts = [x.lineno for x in ast.walk(n) if hasattr(x, "lineno")]
        ends = [x.end_lineno for x in ast.walk(n) if getattr(x, "end_lineno", None)]
        if starts and ends:
            spans.append((min(starts), max(ends)))
    return spans


def _call_lines(fn: ast.FunctionDef, names: Tuple[str, ...]) -> List[Tuple[int, str]]:
    """`fn` 里这些调用的 `(行号, 名字)`，按行号升序"""
    out: List[Tuple[int, str]] = []
    for call in ast.walk(fn):
        if not isinstance(call, ast.Call) or call.lineno <= fn.lineno:
            continue
        name = call.func.id if isinstance(call.func, ast.Name) else getattr(call.func, "attr", "")
        if name in names:
            out.append((call.lineno, name))
    return sorted(out)


def _covered(line: int, spans: List[Tuple[int, int]]) -> bool:
    return any(lo <= line <= hi for lo, hi in spans)


def _store(tmp_path: Path) -> VersionManager:
    return VersionManager(str(tmp_path / "versions"))


# ============ 一、结构面 ============


@pytest.mark.parametrize("method", sorted(PROTECTED_ACTIONS))
def test_write_site_enters_the_storage_dir_lock_exactly_once(method):
    """两处写点各恰好一处 `with path_lock(self.storage_dir)`

    键与 `create_version` 用**同一个目录** ⇒ 三处写点抢同一把锁。这就是本轮的全部主张：
    L273 只罩住了三处里的一处。「恰好一处」而不是「至少一处」：拆成两段，中间仍是窗口。
    """
    fn = _method(method)
    spans = _lock_spans(fn)
    assert len(spans) == 1, f"{method} 的锁段数 = {len(spans)}"
    assert "self.storage_dir" in ast.unparse(
        next(n for n in ast.walk(fn) if isinstance(n, ast.With))), method


@pytest.mark.parametrize("method", sorted(PROTECTED_ACTIONS))
def test_the_check_and_the_act_are_in_the_same_span(method):
    """判据与被保护的动手都在同一段锁里 —— 只锁一半等于没锁

    `delete_version` 的「判 current」与 `rmtree` 分处两端，正是这一对被并发排开了才出悬空；
    `rollback` 的清格与建格同理。少一行进锁，这一支就红。
    """
    fn = _method(method)
    spans = _lock_spans(fn)
    calls = _call_lines(fn, PROTECTED_ACTIONS[method])
    assert len(calls) >= 2, f"{method} 里找不到那两个动作：{calls}"
    for line, name in calls:
        assert _covered(line, spans), f"{method} 的 `{name}`（行 {line}）不在锁内：{spans}"


def test_create_version_lock_shape_is_unchanged():
    """`create_version` 那一格（L273 已收）不许被本轮带跑：三处写点必须共用同一把键

    否则「并发下 current 只有一支赢」这条承诺又回到运气 —— 各锁各的目录，看着都加了锁。
    比的是锁的**键表达式**（`ast.unparse` 后的形状），不是行号跨度：跨度天然各不相同。
    """
    def keys(method: str) -> List[str]:
        fn = _method(method)
        return [ast.unparse(item.context_expr)
                for n in ast.walk(fn) if isinstance(n, ast.With)
                for item in n.items if "path_lock" in ast.unparse(item.context_expr)]

    assert keys("create_version") == ["path_lock(self.storage_dir)"], keys("create_version")
    for method in PROTECTED_ACTIONS:
        assert keys(method) == ["path_lock(self.storage_dir)"], (method, keys(method))


# ============ 二、行为面 ============


def _run_window(vm: VersionManager, victim: str, monkeypatch) -> Dict[str, object]:
    """把「判 current」与「动手」之间的窗口撑开，让并发排次变得确定

    零写入通道：只在类对象上打补丁（`monkeypatch` 负责还原），仓库文件一个字节不动。
    删除那支线程在拿到旧 current 之后停在窗口里，等主测试确认「回滚那一支已经跑完 /
    已经被挡住」才放行 —— 于是真锁与替身锁给出两个**不同**的终态，而不是「也许」。

    Args:
        vm: 已备好两个版本（`victim` 与当前版本）的管理器
        victim: 被并发删除 / 回滚的那个版本 ID
        monkeypatch: pytest 的补丁器，负责在本支结束时还原 `get_current_version`

    Returns:
        读数：两支的返回、回滚是否抢先跑完、终态 current、那个版本目录还在不在
    """
    real_get_current = VersionManager.get_current_version
    in_window = threading.Event()
    proceed = threading.Event()
    rollback_finished = threading.Event()
    outcome: Dict[str, object] = {}

    def patched(self):
        cur = real_get_current(self)
        if getattr(threading.current_thread(), "_is_deleter", False):
            in_window.set()
            proceed.wait(5.0)
        return cur

    monkeypatch.setattr(VersionManager, "get_current_version", patched)

    def deleter():
        threading.current_thread()._is_deleter = True
        try:
            vm.delete_version(victim)
            outcome["delete"] = "ok"
        except VersionError as exc:
            outcome["delete"] = f"VersionError: {exc}"

    def roller():
        outcome["rollback"] = vm.rollback(victim)
        rollback_finished.set()

    t_del = threading.Thread(target=deleter)
    t_del.start()
    assert in_window.wait(5.0), "删除那支没停在窗口里，这一臂量不到东西"
    t_roll = threading.Thread(target=roller)
    t_roll.start()
    # 真锁：回滚进不来，这里必然超时；替身锁：回滚跑完并置位
    rollback_ran_first = rollback_finished.wait(WINDOW_S)
    proceed.set()
    for t in (t_roll, t_del):
        t.join(5.0)
    assert not t_del.is_alive() and not t_roll.is_alive(), "有一支线程卡住了"

    outcome["victim"] = victim
    outcome["rollback_ran_first"] = rollback_ran_first
    outcome["current"] = real_get_current(vm)
    outcome["victim_dir_exists"] = (vm.storage_dir / victim).exists()
    return outcome


def _is_dangling(outcome: Dict[str, object]) -> bool:
    """current 指向一个已经不存在的版本目录 —— 「不能删除当前版本」这条承诺被破掉"""
    return (outcome["current"] == outcome["victim"]
            and outcome["victim_dir_exists"] is False)


def test_forced_window_leaves_no_dangling_current(tmp_path, monkeypatch):
    """撑开窗口的确定性对撞 ⇒ 终态一致，落后那支按新状态出声

    改前（check-then-act 在锁外）读数：`DANGLING-CURRENT` —— `rollback` 回 True，
    `delete_version` 用旧 current 判「不是当前版本」然后把目录删掉。
    改后读数：两支被排成先后，回滚拿到 `False` 并记下「版本不存在」，current 仍是活着的版本。
    """
    vm = _store(tmp_path)
    vm.create_version(ITEMS, label="v1")
    victim = vm.list_versions()[0].version_id
    vm.create_version(ITEMS, label="v2")

    outcome = _run_window(vm, victim, monkeypatch)

    assert not _is_dangling(outcome), f"current 指向已删目录：{outcome}"
    assert outcome["rollback_ran_first"] is False, (
        f"回滚那一支没有排队，锁没生效：{outcome}")
    assert outcome["rollback"] is False, f"回滚对已删版本回了成功：{outcome}"
    assert outcome["delete"] == "ok", outcome
    current = outcome["current"]
    assert current is not None and (vm.storage_dir / current).exists(), f"current 落空：{outcome}"


def test_history_records_the_order_that_actually_happened(tmp_path, monkeypatch):
    """`history.jsonl` 与终态不矛盾：删除记了一条，失败的那次回滚不该留下 rollback 记录

    这一支管的是承诺的另一半 —— 用户翻历史时不能看到「回滚成功了」而 current 指向别处。
    """
    vm = _store(tmp_path)
    vm.create_version(ITEMS, label="v1")
    victim = vm.list_versions()[0].version_id
    vm.create_version(ITEMS, label="v2")
    outcome = _run_window(vm, victim, monkeypatch)
    assert outcome["rollback"] is False, outcome

    lines = [json.loads(x) for x in
             (vm.storage_dir / "history.jsonl").read_text(encoding="utf-8").splitlines()
             if x.strip()]
    actions = [e["action"] for e in lines]
    assert actions.count("delete") == 1, actions
    assert actions.count("rollback") == 0, actions
    created = {e["version_id"] for e in lines if e["action"] == "create"}
    assert outcome["current"] in created, f"current 不在任何一次创建过的版本里：{actions}"


# ============ 三、承诺面没被写松 ============


def test_deleting_the_current_version_still_raises(tmp_path):
    """串行化之后「不能删除当前版本」仍然出声，而不是退化成一句静默跳过"""
    vm = _store(tmp_path)
    vm.create_version(ITEMS, label="only")
    current = vm.get_current_version()
    with pytest.raises(VersionError, match="不能删除当前版本"):
        vm.delete_version(current)
    assert (vm.storage_dir / current).exists()


def test_rollback_to_a_missing_version_still_returns_false(tmp_path):
    """回滚不存在的版本仍是 `False`（不是异常、也不是 True）—— 判据挪进锁里不改语义"""
    vm = _store(tmp_path)
    vm.create_version(ITEMS, label="seed")
    assert vm.rollback("v_19700101_000000_000000") is False


# ============ 四、红能力 ============


def test_dangling_current_returns_when_the_lock_is_faked_away(tmp_path, monkeypatch):
    """注入臂：把 `path_lock` 换成「每次给一把新锁」⇒ 同一场对撞必须重现悬空 current

    没有这一支，行为面那支可能只是「恰好没撞上」。这一臂与它共用同一个撑开窗口的编排，
    唯一的差别就是锁的真假 —— 终态必须分岔。
    """
    import augmentor.versioning as vm_mod

    monkeypatch.setattr(vm_mod, "path_lock", lambda _p: threading.RLock())
    vm = _store(tmp_path)
    vm.create_version(ITEMS, label="v1")
    victim = vm.list_versions()[0].version_id
    vm.create_version(ITEMS, label="v2")

    outcome = _run_window(vm, victim, monkeypatch)

    assert outcome["rollback_ran_first"] is True, (
        f"替身没生效：回滚那一支仍被挡住，锁没被换掉：{outcome}")
    assert _is_dangling(outcome), f"摘掉锁之后仍没重现悬空 current，行为用例没有牙齿：{outcome}"


def test_probe_leftovers_stay_outside_the_repo():
    """本文件的版本面全部落在 `tmp_path`：仓库根不许多出 `versions` 目录"""
    stray = ROOT / "versions"
    assert not stray.exists(), f"版本面把目录写进了仓库根：{stray}"


@pytest.fixture(autouse=True)
def _cleanup_store(tmp_path):
    """Windows 上符号链接 / 被占句柄会让 `tmp_path` 清理失败，这里显式收掉版本目录"""
    yield
    store = tmp_path / "versions"
    if store.exists():
        shutil.rmtree(store, ignore_errors=True)
