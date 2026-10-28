# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""L290（A396）：依赖登记表的「读整份 → 改 → 整份写回」必须是一趟

**缺陷形状**：`DependencyManager` 在构造时把 `datasets.json` / `dependencies.json` 整份
读进内存，写面（`register_dataset` / `add_dependency` / `remove_dependency`）改完内存
就整份写回。HTTP 面每支请求新建一只管理器（`api/routes/system_ops.py::_dependency_manager`），
于是两支并发请求的「读」都发生在对方的「写」之前 ⇒ 后写的那一份带着前一次改动之前的
快照，把别人的登记凭空抹掉，而两边都回 200。`atomic_write_json` 对此无能为力：它保证
**读者**看不见半份，保证不了**写者**不互相覆盖（`atomic_write.py` 的 L273 段同一条口径）。

改前实测（仓外沙箱 `Temp/l290q/probe_dependency_race.py`，20 支线程同一时刻各登记一条、
逐支一个新实例）：**盘上 1 条、丢 19 条**，另有 1 支 `PermissionError`（两支同时把各自的
临时件换到同一个 `datasets.json`，Windows 瞬时拒绝访问，`atomic_write.py` 的 5 次退避上限
内没换上）⇒ 20 支全回 200 而 19 条登记凭空消失。修后同一支探针读数：**20 条全落盘、
0 次异常**。

**为什么 L273 那把普查尺看不见这一族**：它按「同一函数体内，读回整份的调用与整份写回
的调用打到同一个目标」判定 RMW。这一格的读住在 `__init__`、写住在另一个方法里，跨了
函数边界，判据结构性地看不见它。本轮把读面收进 `_load_datasets` / `_load_dependencies`
两只方法并让写面在锁内调用它们 ⇒ 形状变成「同一函数体内先读后写」，尺子从此看得见
（`test_rmw_serialization_l273.py` 的名单因此从七格长成十格）。

本文件四面：一、结构面（三处写点各进锁一次、且读在写前；**构造的那两份读也必须在同一
把锁里** —— 这一格不是洁癖：写面进锁之后连跑多轮仍会撞上 `PermissionError: [Errno 13]`，
抛点在 `_load_file` 的裸 `open(..., 'r')` 上，Windows 会让「读一个正被 `os.replace` 的
目标」直接失败，锁外构造等于把「丢更新」修成「500」，只修了一半）；二、编排面（单线程就能破的
那一格 —— 拿着旧快照写回，把别人新登记的抹掉）；三、并发面（与探针同尺的 20 支）；
四、红能力（把 `path_lock` 换成「每次给一把新锁」，丢失必须回来）。

已知边界（不在本轮自决的面，已记进账本）：这把锁只保**同进程**，多 worker 部署跨不出去，
与 A156 / A158 那一族同一格待拍。
"""

import ast
import json
import threading
import time
from concurrent.futures import ThreadPoolExecutor
from pathlib import Path
from typing import Dict, List, Tuple

import pytest

from augmentor import dependency as dep_mod
from augmentor.dependency import DependencyManager

ROOT = Path(__file__).resolve().parents[2]

#: 并发规模与探针同尺：改前 20 支只落 1 条。
N_CONCURRENT = 20

SRC = (ROOT / "augmentor" / "dependency.py").read_text(encoding="utf-8")
TREE = ast.parse(SRC)


def _method(name: str) -> ast.FunctionDef:
    """取 `DependencyManager` 里那一方法的 AST"""
    cls = next(n for n in ast.walk(TREE)
               if isinstance(n, ast.ClassDef) and n.name == "DependencyManager")
    return next(n for n in cls.body
                if isinstance(n, ast.FunctionDef) and n.name == name)


def _lock_spans(fn: ast.FunctionDef) -> List[Tuple[int, int]]:
    """`fn` 内每个 `with path_lock(...)` 体的行范围"""
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


def _call_lines(fn: ast.FunctionDef, names: set) -> List[Tuple[int, str]]:
    """`fn` 里这些调用的 `(行号, 名字)`，按行号升序"""
    out = []
    for call in ast.walk(fn):
        if not isinstance(call, ast.Call) or call.lineno <= fn.lineno:
            continue
        name = call.func.id if isinstance(call.func, ast.Name) else getattr(call.func, "attr", "")
        if name in names:
            out.append((call.lineno, name))
    return sorted(out)


def _covered(line: int, spans: List[Tuple[int, int]]) -> bool:
    return any(lo <= line <= hi for lo, hi in spans)


#: 三处写点：读谁、写谁。逐格点名，不用「都加了锁」这种糊过去的断言。
WRITE_METHODS = {
    "register_dataset": ("_load_datasets", "_save_datasets"),
    "add_dependency": ("_load_dependencies", "_save_dependencies"),
    "remove_dependency": ("_load_dependencies", "_save_dependencies"),
}


# ============ 一、结构面 ============


@pytest.mark.parametrize("method", sorted(WRITE_METHODS))
def test_each_write_site_enters_the_registry_lock_exactly_once(method):
    """每处写点恰好一处 `with path_lock(self._registry_path)`

    「恰好一处」而不是「至少一处」：两处意味着锁被拆成两段，中间那段仍是丢更新的窗口。
    锁按**注册表目录**建键（与 `versioning.py` 用 `storage_dir` 做键同一形状）—— 按实例
    建键在这族里等于没锁，因为 HTTP 面每支请求一只新实例。
    """
    spans = _lock_spans(_method(method))
    assert len(spans) == 1, f"{method} 的锁段数 = {len(spans)}"
    assert "self._registry_path" in ast.unparse(
        next(n for n in ast.walk(_method(method))
             if isinstance(n, ast.With) and n.items)), method


@pytest.mark.parametrize("method", sorted(WRITE_METHODS))
def test_read_precedes_write_and_both_sit_inside_the_lock(method):
    """读在写前、且读写都在锁段内 —— 只加锁不重读等于把旧快照盖给别人看

    把 `self._datasets = self._load_datasets()` 挪到 `with` 之外（哪怕锁还留着），
    这支就会红：锁里的写用的是锁外的读数，两支并发仍然互相覆盖。
    """
    fn = _method(method)
    read_name, write_name = WRITE_METHODS[method]
    spans = _lock_spans(fn)
    calls = _call_lines(fn, {read_name, write_name})
    pairs = [(line, name) for line, name in calls]
    reads = [line for line, name in pairs if name == read_name]
    writes = [line for line, name in pairs if name == write_name]
    assert reads and writes, f"{method} 找不到 {read_name} / {write_name}：{calls}"
    assert max(reads) < min(writes), f"{method} 的写在读之前：{calls}"
    for line in reads + writes:
        assert _covered(line, spans), f"{method} 行 {line} 不在锁内：{spans}"


def test_reads_are_centralized_so_the_census_can_see_them():
    """登记账的整份读只住两只方法，且构造也用它们

    这一条是给 L273 普查尺铺路：读面散在 `__init__` 里时，「先读后写同目标」的形状跨了
    函数边界，尺子看不见；收进方法之后尺子按函数体就能点名这一族。
    """
    for name in ("_load_datasets", "_load_dependencies"):
        assert _method(name), name
    init = _method("__init__")
    assert {n for _l, n in _call_lines(init, {"_load_datasets", "_load_dependencies"})} == {
        "_load_datasets", "_load_dependencies"}
    # 构造不许自己拼 `open` / `_load_file(self._datasets_file...)`：那样会绕开唯一读者
    assert not [line for line, name in _call_lines(init, {"_load_file"})], \
        "__init__ 里另有整份读的入口"


def test_the_constructors_read_also_sits_inside_the_lock():
    """构造器的那两份整份读也在 `path_lock` 段内 —— 这一条是被一次 flake 逼出来的

    写面收进锁之后连跑多轮仍能撞上一支 `PermissionError: [Errno 13]`（同一对文件复跑
    六次命中一次，七文件合跑当场命中），抛点在 `_load_file` 的裸 `open(..., 'r')` 上：
    Windows 会让「读一个正被 `os.replace` 的目标」直接失败。所以构造与写面必须抢同一把
    目录锁，否则回执从「丢更新」变成「500」，修了一半。摘掉这一格（把两行读挪出 `with`）
    ⇒ 本支红。
    """
    init = _method("__init__")
    spans = _lock_spans(init)
    assert len(spans) == 1, f"构造器的锁段数 = {len(spans)}"
    calls = _call_lines(init, {"_load_datasets", "_load_dependencies"})
    assert len(calls) == 2, calls
    for line, name in calls:
        assert _covered(line, spans), f"构造器的 {name}（行 {line}）在锁外：{spans}"


# ============ 二、编排面（单线程就能破的那一格）============


def test_stale_instance_does_not_erase_a_foreign_registration(tmp_path):
    """拿着旧快照的那一支写回时，必须把别人新登记的一起带上

    形状：`m1` 与 `m2` 指向同一个注册表目录。`m1` 先构造（读到空账），`m2` 登记 `外部`，
    然后 `m1` 登记 `自己`。改前 `m1` 写回的是它那份空快照 ⇒ `外部` 凭空消失，而两次调用
    都返回了 `DatasetInfo`。这一支不需要并发就能红，是那一族最干净的判据。
    """
    reg = tmp_path / "reg"
    m1 = DependencyManager(str(reg))
    m2 = DependencyManager(str(reg))
    m2.register_dataset("外部", "data/other.json", 1)
    m1.register_dataset("自己", "data/mine.json", 2)

    on_disk = json.loads((reg / "datasets.json").read_text(encoding="utf-8"))
    names = {row["name"] for row in on_disk.values()}
    assert names == {"外部", "自己"}, f"旧快照抹掉了别人的登记：{sorted(names)}"


def test_stale_instance_does_not_erase_a_foreign_dependency(tmp_path):
    """同一形状的依赖账版：`add_dependency` 也不能拿构造时那份快照写回"""
    reg = tmp_path / "reg2"
    m1 = DependencyManager(str(reg))
    m2 = DependencyManager(str(reg))
    m2.add_dependency("a", "b", "derived", "外部那条")
    m1.add_dependency("c", "d", "merged", "自己那条")

    rows = json.loads((reg / "dependencies.json").read_text(encoding="utf-8"))
    pairs = {(r["source_dataset"], r["target_dataset"]) for r in rows}
    assert pairs == {("a", "b"), ("c", "d")}, f"丢了一条依赖：{sorted(pairs)}"


def test_remove_does_not_resurrect_a_foreign_dependency(tmp_path):
    """删除那一侧的镜像形状：`remove_dependency` 写回时不许把别人刚加的依赖「删掉」

    改前它把构造时的快照过滤一遍再整份写回 —— 别人在这之后新增的那条压根不在快照里，
    于是「删除」变成了「连别人的一起抹掉」。
    """
    reg = tmp_path / "reg3"
    m1 = DependencyManager(str(reg))
    m1.add_dependency("a", "b", "derived", "将被删")
    m1.add_dependency("x", "y", "filtered", "外部新增")
    m2 = DependencyManager(str(reg))          # 拿着含两条的快照
    m1.add_dependency("p", "q", "transformed", "随后又加一条")
    assert m2.remove_dependency("a", "b") is True
    rows = json.loads((reg / "dependencies.json").read_text(encoding="utf-8"))
    pairs = {(r["source_dataset"], r["target_dataset"]) for r in rows}
    assert pairs == {("x", "y"), ("p", "q")}, f"删除写回了旧快照：{sorted(pairs)}"


# ============ 三、并发面（与探针同尺）============


def _per_request_register(reg: Path, i: int) -> List[str]:
    """逐字复刻 HTTP 面的形状：每支请求一只新管理器（`_dependency_manager`）"""
    errors: List[str] = []
    try:
        DependencyManager(str(reg)).register_dataset(f"ds{i}", f"data/ds{i}.json", i)
    except Exception as exc:  # noqa: BLE001 —— 读数要收敛成人能看的一行，不让一支异常吃掉对账
        errors.append(f"{type(exc).__name__}: {exc}")
    return errors


def test_twenty_concurrent_registrations_all_land(tmp_path):
    """20 支并发登记（逐支一个新实例）⇒ 20 条全落盘、0 次异常

    改前读数：**盘上 1 条、丢 19 条**，另 1 支 `PermissionError`。判据不看返回值 ——
    改前 20 支全回 `DatasetInfo`，这正是「成功回执撒谎」那一格。
    """
    reg = tmp_path / "race_reg"
    with ThreadPoolExecutor(max_workers=N_CONCURRENT) as pool:
        results = list(pool.map(lambda i: _per_request_register(reg, i), range(N_CONCURRENT)))
    errors = [e for row in results for e in row]
    assert errors == [], errors
    on_disk = json.loads((reg / "datasets.json").read_text(encoding="utf-8"))
    assert len(on_disk) == N_CONCURRENT, f"只落盘 {len(on_disk)} 条，丢 {N_CONCURRENT - len(on_disk)} 条"
    assert DependencyManager(str(reg)).list_datasets().__len__() == N_CONCURRENT


def test_twenty_concurrent_dependencies_all_land(tmp_path):
    """依赖账同一档：20 支并发 `add_dependency` ⇒ 20 条全落盘、0 次异常"""
    reg = tmp_path / "race_dep"

    def one(i: int) -> List[str]:
        try:
            DependencyManager(str(reg)).add_dependency(f"s{i}", f"t{i}", "derived", "")
        except Exception as exc:  # noqa: BLE001
            return [f"{type(exc).__name__}: {exc}"]
        return []

    with ThreadPoolExecutor(max_workers=N_CONCURRENT) as pool:
        errors = [e for row in pool.map(one, range(N_CONCURRENT)) for e in row]
    assert errors == [], errors
    rows = json.loads((reg / "dependencies.json").read_text(encoding="utf-8"))
    assert len(rows) == N_CONCURRENT, f"只落盘 {len(rows)} 条"
    assert {(r["source_dataset"], r["target_dataset"]) for r in rows} == {
        (f"s{i}", f"t{i}") for i in range(N_CONCURRENT)}


def _read_text_retrying(path: Path, attempts: int = 20, backoff: float = 0.01) -> str:
    """读一份正在被 `os.replace` 的文件，区分「瞬时共享冲突」与「读到半份」

    Windows 上裸 `open` 撞上 `MoveFileExW(REPLACE_EXISTING)` 会瞬时
    `PermissionError: [Errno 13]`（产品读面走 `path_lock`，不会撞这一格；本用例
    刻意绕开锁量原子替换的可见性）。这里对共享冲突做有界重试，重试耗尽仍失败就
    原样上抛——那是真问题，不能当成「没读到」咽掉。读到截断内容仍由调用方的
    `JSONDecodeError` 分支判红。
    """
    for _ in range(attempts - 1):
        try:
            return path.read_text(encoding="utf-8")
        except PermissionError:
            time.sleep(backoff)
    return path.read_text(encoding="utf-8")


def test_concurrent_mixed_reads_never_see_a_half_written_registry(tmp_path):
    """并发的读者（`GET …/datasets` 那一形）永远拿到合法 JSON，不许读到半份

    `atomic_write_json` 承诺的那一半仍在：锁管写者互斥，原子替换管读者可见性。
    这里让 20 支写与 20 支读同时跑，读数只许是「合法 JSON 或还没有账」。
    """
    reg = tmp_path / "mixed"
    DependencyManager(str(reg)).register_dataset("seed", "data/seed.json", 0)
    bad: List[str] = []

    def reader(i: int) -> List[str]:
        out: List[str] = []
        for _ in range(5):
            path = reg / "datasets.json"
            if not path.exists():
                continue
            try:
                json.loads(_read_text_retrying(path))
            except (json.JSONDecodeError, UnicodeDecodeError) as exc:
                out.append(f"读到半份：{type(exc).__name__}")
        return out

    with ThreadPoolExecutor(max_workers=40) as pool:
        futs = [pool.submit(_per_request_register, reg, i) for i in range(1, 21)]
        futs += [pool.submit(reader, i) for i in range(20)]
        for f in futs:
            bad.extend(f.result())
    assert bad == [], bad[:3]
    assert len(json.loads((reg / "datasets.json").read_text(encoding="utf-8"))) == 21


# ============ 四、红能力 ============


def test_lost_update_returns_when_the_lock_is_faked_away(tmp_path, monkeypatch):
    """注入臂：把 `path_lock` 换成「每次给一把新锁」⇒ 丢失或异常至少中一个

    判据与 L273 那支同口径：坏几次取决于线程交错（探针实测落盘 1—2 条），能稳定复现的是
    **会不会坏**。没有这一支，上一支并发用例有可能只是恰好绿。
    """
    monkeypatch.setattr(dep_mod, "path_lock", lambda _p: threading.RLock())
    reg = tmp_path / "fake_lock"
    with ThreadPoolExecutor(max_workers=N_CONCURRENT) as pool:
        results = list(pool.map(lambda i: _per_request_register(reg, i), range(N_CONCURRENT)))
    errors = [e for row in results for e in row]
    landed = len(json.loads((reg / "datasets.json").read_text(encoding="utf-8")))
    assert landed < N_CONCURRENT or errors, (
        f"替身没生效：{landed} 条全落盘且 0 次异常，这一支量不出丢更新")


def test_stale_write_site_is_named_by_the_l273_census(tmp_path):
    """注入臂：把 `register_dataset` 的锁摘掉，L273 那把普查尺必须点名它

    证明本轮把读面收进方法之后，尺子真的看得见这一族（而不是只在我的新文件里看得见）。
    用 AST 替身而不是改仓库文件：零写入通道，磁盘不动。
    """
    import tests.integration.test_rmw_serialization_l273 as l273

    locked = SRC
    sites, bad = l273.census_tree(ast.parse(locked), "dependency.py")
    assert "dependency.py::DependencyManager.register_dataset" in sites, sites
    assert bad == [], bad

    unlocked = SRC.replace(
        "        with path_lock(self._registry_path):\n"
        "            self._datasets = self._load_datasets()\n"
        "            self._datasets[dataset_id] = info\n"
        "            self._save_datasets()\n",
        "        self._datasets = self._load_datasets()\n"
        "        self._datasets[dataset_id] = info\n"
        "        self._save_datasets()\n",
        1,
    )
    assert unlocked != SRC, "锚点失真：这一臂没有真的摘掉锁"
    sites2, bad2 = l273.census_tree(ast.parse(unlocked), "dependency.py")
    assert "dependency.py::DependencyManager.register_dataset" in sites2, sites2
    assert any("register_dataset" in line for line in bad2), (
        f"摘掉锁之后尺子仍然不点名它：{bad2}")


def test_registry_lock_is_per_directory_not_per_instance(tmp_path):
    """同一目录的两只实例必须拿到同一把锁：按实例建键等于没锁

    这一条钉的是修法的地基 —— HTTP 面每支请求一只新实例，`self._lock` 那种形状
    在改前也「有锁」，却一把都拦不住。
    """
    from augmentor.atomic_write import path_lock

    reg = tmp_path / "identity"
    a = DependencyManager(str(reg))
    b = DependencyManager(str(reg / "."))
    assert path_lock(a._registry_path) is path_lock(b._registry_path)
    assert path_lock(a._registry_path) is not path_lock(tmp_path / "other_reg")


@pytest.fixture(autouse=True)
def _no_repo_leftovers():
    """本文件只写 `tmp_path`：仓库里不许留下注册表目录

    探针的历史事故是把服务自己那份档写坏，所以这里把「产物不落仓」做成断言。
    """
    yield
    leftovers = [str(q.relative_to(ROOT)) for q in ROOT.rglob("datasets.json")
                 if "__pycache__" not in str(q)]
    assert leftovers == [], f"并发登记面把注册表写进了仓库：{leftovers}"
