# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""L273：同一份文件上的「读 → 改 → 写」必须串行

**缺陷形状**：本仓的写侧早就收成了原子写（临时件 + `os.replace`），它保证的是**读者**
看不见半份文件。它保证不了**写者**不互相覆盖：两个请求各自读回整份旧内容、各自只改
自己那一格、各自把整份写回 ⇒ 后写的那一份带着旧快照，把前一次改动凭空抹掉，而两边都
拿到 `{"success": true}`。**两次原子不等于一次原子。**

改前实测（仓外沙箱、单进程单 worker，探针 `probe_race.py` / `probe3_versions.py`）：

* 200 条数据集上 20 支并发 `PUT /api/data/update` ⇒ 盘上只落 **1** 处，丢 19 处，全部
  回 200；50 支时另有 3 支 500（读侧撞 Windows 的瞬时拒绝访问）。
* 并发 `DELETE` 20 支 ⇒ 只删掉 **1** 条，丢 19 次删除，全部回 200。
* 备份索引 20 次并发创建 ⇒ 索引 **1** 行、盘上 20 份文件，那 19 份对 `list` / `restore`
  / `delete` / `clean_old_backups` 全部不可见：数据没坏，账本丢了。
* `versioning.VersionManager` 24 支并发 `create_version` ⇒ 重复版本号让后者覆盖前者的
  快照，且 `current.txt` 的「判存在再清」与另一支的写相撞 ⇒ 1/24 次 `PermissionError`
  落 500 并留下孤儿版本目录。
* CLI 版 `version_control.create_version`（每次调用新建实例）20 支并发 ⇒ 索引只剩 1 行，
  盘上 20 个版本目录却都在。

修法：一把**按路径**的可重入锁（`augmentor.atomic_write.path_lock`），罩住整段读-改-写。
选线程锁而不是 `asyncio.Lock`，因为同步写者（备份索引、版本账）与异步路由面碰的是同一批
文件，只有一把共享的按路径线程键才能让他们互斥；HTTP 面因此把整段 RMW 交给一支工作线程，
绝不在 `await` 上持锁。选 `RLock` 而不是 `Lock`，因为外层罩整段与内层辅助方法自保会套在
同一把键上（`version_control._load_index` 被 `create_version` 在锁内调用就是这一形）。

本文件三面：一、锁的身份；二、改后的并发读数（全部落在 `tmp_path`）；三、结构普查尺子
（新写的 RMW 点没进锁就红）加它的四支注入臂；四、落点判据没被写松（安全面）。

L290 给这把尺子补了一格它原本**结构性看不见**的形状：`augmentor/dependency.py` 的登记账
把整份读住在构造里、整份写回住在别的方法里，「同函数体内读写同目标」这一判据配不上对。
那一轮把读面收进 `_load_datasets` / `_load_dependencies` 之后，形状变成同函数体内的先读
后写，尺子第一次能点名这一族 ⇒ 名单从七格长成十格（`WRITE_CALLS` / `READ_CALLS` 各加两只
名字，判据一字未松）。新格的注入臂在 `test_dependency_registry_lock_l290.py`：把
`register_dataset` 的锁摘掉，本文件的 `census_tree` 必须点名它。

另两格不在本轮自决的面，已记进账本待拍：HTTP **读**侧撞 `os.replace` 那一瞬的
`PermissionError`/500（要不要给读也上锁是吞吐与响应面的取舍）、以及这把锁只保**同进程**
（多 worker 部署跨不出去）。
"""

import ast
import asyncio
import json
import os
import threading
from concurrent.futures import ThreadPoolExecutor
from pathlib import Path
from typing import Dict, List, Optional, Set, Tuple

import httpx
import pytest

from api import deps
from api.main import app
from augmentor.atomic_write import path_lock
from augmentor.validation import path_lands_within

ROOT = Path(__file__).resolve().parents[2]  # tests/integration/ -> 仓内 augmentor 根

ITEMS: List[Dict] = [{"instruction": f"问句 {i}", "output": f"答句 {i}"} for i in range(40)]

#: Windows 扩展长度前缀：`Path.resolve()` 拿到句柄的那一次会给路径戴上它，没拿到的那次
#: 不戴 —— L273 的落点误判就住在这个形状差上（字面量写成 `"\\\\?\\"` 才是两个反斜杠加问号）
EXT_PREFIX = "\\\\?\\"

#: HTTP 那两支（正判据 + 红能力臂）共用的底料。条数取 200 而不是 40：读-改-写的窗口
#: 由「读整份 + 写整份」的耗时决定，40 条时窗口太窄，替身臂量不出丢失（实测 20 支全落盘），
#: 200 条与探针同尺 ⇒ 两支是只对「锁」这一件事做 A/B。
RACE_ITEMS: List[Dict] = [{"instruction": f"问句 {i}", "output": f"答句 {i}"} for i in range(200)]

#: 并发规模：改前 20 支只落 1 处，5—8 支已能翻出丢失；取 20 与探针同尺，
#: 单支用例只跑一次，全套六支的墙钟在秒级。
N_CONCURRENT = 20

# ============ 三、结构普查尺子：判据与现量名单 ============

#: 「整份写回」的六种写法。只认这几类，是因为丢更新的**代价**来自整份覆盖：
#: 追加式写（`open(..., "a")`、`history.jsonl`）不在这一族。
#: 后两格（`_save_datasets` / `_save_dependencies`）由 L290 接进来，理由见下面名单的归因段。
WRITE_CALLS = {"atomic_write_json", "atomic_write_text", "_save_index", "write_json_file",
               "_save_datasets", "_save_dependencies"}
#: 「读回整份」的写法，与上面**同一目标**成对出现才叫读-改-写
READ_CALLS = {"read_items", "read_json_file", "_load_index", "load_version", "load_config",
              "_load_datasets", "_load_dependencies"}

SCAN_DIRS = ("augmentor", "api", "cli")

#: 现量（L273 建尺，L290 重钉）：全仓「同一目标先读回整份、再整份写回」的函数名单，
#: 逐处点名其写点数。这张名单是**普查的输出**而不是手抄的期望 ——
#: `test_rmw_roster_is_exactly_the_census` 拿它和真扫一遍做集合相等：多一处（新写的 RMW）
#: 或少一处（形状被改造）都红。计数都是 1，因为每处只有那一次整份写回。
#:
#: **L290 的三格（A396）归因**：`augmentor/dependency.py` 的三处写点改前**看不见**，不是
#: 因为漏扫，而是因为它的读住在 `__init__`、写住在别的方法里 —— 判据按「同一函数体内
#: 读写打到同一目标」配对，跨函数的那一形结构性地落在尺子之外。L290 把整份读收进
#: `_load_datasets` / `_load_dependencies` 并让写面在锁内调用它们，形状变成同函数体内
#: 的先读后写，于是这三格第一次进名单。名单从七格长成十格是**覆盖面扩大**，不是判据放宽。
MEASURED_RMW_SITES: Dict[str, int] = {
    "api/deps.py::_read_modify_write": 1,
    "augmentor/backup.py::DatasetBackup.backup": 1,
    "augmentor/backup.py::DatasetBackup.delete_backup": 1,
    "augmentor/dependency.py::DependencyManager.add_dependency": 1,
    "augmentor/dependency.py::DependencyManager.register_dataset": 1,
    "augmentor/dependency.py::DependencyManager.remove_dependency": 1,
    "augmentor/version_control.py::DatasetVersionManager._load_index": 1,
    "augmentor/version_control.py::DatasetVersionManager.create_version": 1,
    "augmentor/version_control.py::DatasetVersionManager.delete_version": 1,
    "augmentor/version_control.py::DatasetVersionManager.set_current_version": 1,
}


def _target_of(call: ast.Call) -> str:
    """取读/写的目标：第一形参的文本；`_save_index` / `_load_index` 这类不带形参的，
    用 `<self>` 这一个隐式目标（它们读写的是 `self._index_file`）"""
    if call.args and isinstance(call.args[0], (ast.Name, ast.Attribute, ast.Subscript)):
        return ast.unparse(call.args[0])
    return "<self>"


def _lock_spans(node: ast.AST) -> List[Tuple[int, int]]:
    """`node` 内每个 `with path_lock(...)` 体的行范围"""
    spans: List[Tuple[int, int]] = []
    for n in ast.walk(node):
        if not isinstance(n, ast.With):
            continue
        if not any("path_lock" in ast.unparse(item.context_expr) for item in n.items):
            continue
        starts = [x.lineno for x in ast.walk(n) if hasattr(x, "lineno")]
        ends = [x.end_lineno for x in ast.walk(n) if getattr(x, "end_lineno", None)]
        if starts and ends:
            spans.append((min(starts), max(ends)))
    return spans


def _covered(line: int, spans: List[Tuple[int, int]]) -> bool:
    return any(lo <= line <= hi for lo, hi in spans)


def _nested_lines(fn: ast.AST) -> Set[int]:
    """嵌套函数自己的行号 —— 外层函数不该替内层背写点，否则一格 RMW 会被数成两格"""
    out: Set[int] = set()
    for n in ast.walk(fn):
        if isinstance(n, (ast.FunctionDef, ast.AsyncFunctionDef)) and n is not fn:
            out.update(range(n.lineno, (n.end_lineno or n.lineno) + 1))
    return out


def census_tree(tree: ast.AST, origin: str) -> Tuple[Dict[str, int], List[str]]:
    """对一棵 AST 跑判据

    Args:
        tree: 已解析的模块
        origin: 名单键里的文件名（真扫描给相对路径，注入臂给假名）

    Returns:
        `(读-改-写名单, 未受锁保护的写点清单)`
    """
    sites: Dict[str, int] = {}
    violations: List[str] = []
    for fn in ast.walk(tree):
        if not isinstance(fn, (ast.FunctionDef, ast.AsyncFunctionDef)):
            continue
        skip = _nested_lines(fn)
        reads: Set[str] = set()
        writes: List[Tuple[int, str, str]] = []
        for call in ast.walk(fn):
            if not isinstance(call, ast.Call) or call.lineno <= fn.lineno or call.lineno in skip:
                continue
            name = call.func.id if isinstance(call.func, ast.Name) else getattr(call.func, "attr", "")
            if name in READ_CALLS:
                reads.add(_target_of(call))
            elif name in WRITE_CALLS:
                writes.append((call.lineno, name, _target_of(call)))
            elif name == "open" and call.args:
                mode = call.args[1].value if len(call.args) > 1 and isinstance(call.args[1], ast.Constant) else "r"
                target = _target_of(call)
                if "w" in str(mode) or "a" in str(mode):
                    writes.append((call.lineno, "open", target))
                else:
                    reads.add(target)
        matched = [w for w in writes if w[2] in reads]
        if not matched:
            continue
        cls = next((c for c in ast.walk(tree)
                    if isinstance(c, ast.ClassDef) and any(f is fn for f in c.body)), None)
        key = f"{origin}::{cls.name}.{fn.name}" if cls else f"{origin}::{fn.name}"
        sites[key] = len(matched)
        spans = _lock_spans(fn)
        for line, name, _target in matched:
            if not _covered(line, spans):
                violations.append(f"{key} 的 `{name}` 写点（行 {line}）不在 `path_lock` 里")
    return sites, violations


def census_repo() -> Tuple[Dict[str, int], List[str]]:
    """扫产品三包 ⇒ `(RMW 名单, 未受锁保护的写点清单)`"""
    sites: Dict[str, int] = {}
    violations: List[str] = []
    for pkg in SCAN_DIRS:
        for path in sorted((ROOT / pkg).rglob("*.py")):
            rel = str(path.relative_to(ROOT)).replace("\\", "/")
            found, bad = census_tree(ast.parse(path.read_text(encoding="utf-8")), rel)
            sites.update(found)
            violations.extend(bad)
    return sites, violations


# ============ 一、锁的身份 ============


def test_same_file_gives_one_lock_whatever_the_spelling(tmp_path):
    """相对形、绝对形、大小写不同形 ⇒ 必须是同一把锁

    锁表按 `normcase(abspath(...))` 建键。若键取原样字符串，同一份文件被写成两种形状时
    拿到两把互不相识的锁 ⇒ 锁只是装饰，丢更新照旧。
    """
    target = tmp_path / "Index.json"
    first = path_lock(target)
    assert first is path_lock(str(target))
    assert first is path_lock(os.path.join(str(tmp_path), "Index.json"))
    assert first is path_lock(target.resolve())
    if os.name == "nt":
        assert first is path_lock(str(target).upper())
    # 可重入是 `RLock` 而非 `Lock`：见 `test_lock_is_reentrant_for_nested_wraps`
    assert isinstance(first, type(threading.RLock()))


def test_different_files_do_not_share_a_lock(tmp_path):
    """不同文件各拿一把：共用一把会把并发读数拖成串行，也把判据洗成恒真"""
    assert path_lock(tmp_path / "a.json") is not path_lock(tmp_path / "b.json")


def test_lock_is_reentrant_for_nested_wraps(tmp_path):
    """外层罩整段、内层自保 ⇒ 同一线程两次进入不许自锁死（`RLock` 存在的唯一理由）

    `version_control._load_index` 被 `create_version` 在锁内调用就是这一形；换成普通
    `Lock` 时这一支会挂在第二次进入上，不是断言红而是整条用例超时。
    """
    lock = path_lock(tmp_path / "index.json")
    with lock:
        with lock:
            assert lock.acquire(blocking=False)
            lock.release()


# ============ 二、行为面：改后并发读数 ============


def _seed(tmp_path: Path, name: str = "race.json", items: Optional[List[Dict]] = None) -> str:
    (tmp_path / name).write_text(json.dumps(items or ITEMS, ensure_ascii=False), encoding="utf-8")
    return name


#: L277 起 `api/main.py` 上多了 `WriteTransportGate`（未配 `AUGMENTOR_API_KEY` 时，
#: `/api/` 的写方法只允许从受信传输进来：回环地址、Unix 套接字、或 ASGI 测试夹具给出的
#: `testserver` 一类固定主机名）。本文件的夹具是 `httpx.ASGITransport(base_url="http://t")`，
#: 它写进 `scope["server"]` 的是 `("t", None)` —— 既不是回环也不在那份名单里 ⇒ 匿名写请求
#: 会在**进路由之前**拿 403，RMW 那三格一个字都量不到（实测读数：20 支全 403）。
#:
#: 处置刻意不走「把 base_url 换成 testserver」那条捷径：那是靠闸的**测试豁免**档过关，
#: 而本文件要的是「写请求真的进到路由、真的在锁里做读-改-写」。这里按生产契约把鉴权开起来
#: （设密钥 + 每支请求带 `X-API-Key`），闸与路由依赖两侧同时成立，判据一档没放松。
WRITE_KEY = "l273-rmw-gate-key"


def _authorize(monkeypatch) -> None:
    """按 L277 的写鉴权契约把本文件的 HTTP 面变成「已授权客户端」"""
    monkeypatch.setenv(deps.API_KEY_ENV, WRITE_KEY)


def _gather(request):
    async def go():
        async with httpx.AsyncClient(transport=httpx.ASGITransport(app=app), base_url="http://t",
                                     headers={"X-API-Key": WRITE_KEY}) as c:
            return await asyncio.gather(*[request(c, i) for i in range(N_CONCURRENT)])
    return asyncio.run(go())


def _single(request):
    async def go():
        async with httpx.AsyncClient(transport=httpx.ASGITransport(app=app), base_url="http://t",
                                     headers={"X-API-Key": WRITE_KEY}) as c:
            return await request(c)
    return asyncio.run(go())


def test_concurrent_updates_all_land(tmp_path, monkeypatch):
    """20 支并发改写各写各的一格 ⇒ 20 处全部落盘（改前：只落 1 处、丢 19 处）

    判据不看状态码 —— 改前 20 支**全**回 200，这正是「成功回执撒谎」那一格；只看盘上
    有多少条真的是本次请求写进去的那一条。
    """
    monkeypatch.setenv("AUGMENTOR_DATA_ROOTS", str(tmp_path))
    _authorize(monkeypatch)
    name = _seed(tmp_path, items=RACE_ITEMS)
    responses = _gather(lambda c, i: c.put(
        f"/api/data/update/{name}", params={"index": i},
        json={"instruction": f"改写 {i}", "output": f"答句 {i}"}))
    assert [r.status_code for r in responses] == [200] * N_CONCURRENT, [r.text for r in responses]

    on_disk = json.loads((tmp_path / name).read_text(encoding="utf-8"))
    landed = sum(1 for i in range(N_CONCURRENT) if on_disk[i].get("instruction") == f"改写 {i}")
    assert landed == N_CONCURRENT, f"丢了 {N_CONCURRENT - landed} 次改写"
    assert len(on_disk) == len(RACE_ITEMS)


def test_concurrent_deletes_land_exactly(tmp_path, monkeypatch):
    """20 支并发 `DELETE index=0` ⇒ 恰好少 20 条，且一支都不许拿到 400（改前：只删掉 1 条）

    全取 0 是刻意的：串行之后「删第一条」二十次就是把前 20 条依次删掉，条数判据与请求
    顺序无关；而改前的丢更新只删掉 1 条，两边在条数上必然分岔。若改成分散下标，串行化
    反而会让高位下标被判越界（那是**正确**的新语义），条数就不再是纯净的判据。
    """
    monkeypatch.setenv("AUGMENTOR_DATA_ROOTS", str(tmp_path))
    _authorize(monkeypatch)
    name = _seed(tmp_path)
    responses = _gather(lambda c, i: c.delete(f"/api/data/delete/{name}", params={"index": 0}))
    assert [r.status_code for r in responses] == [200] * N_CONCURRENT, [r.text for r in responses]
    left = json.loads((tmp_path / name).read_text(encoding="utf-8"))
    expected = len(ITEMS) - N_CONCURRENT
    assert len(left) == expected, f"删除丢失 {len(left) - expected} 次"


def test_concurrent_backup_create_keeps_every_row(tmp_path):
    """20 次并发创建备份 ⇒ 索引行数 == 盘上文件数 == 20（改前：索引 1 行、盘上 20 份）"""
    from augmentor import backup as backup_mod

    source = tmp_path / "src.json"
    source.write_text(json.dumps(ITEMS, ensure_ascii=False), encoding="utf-8")
    bdir = tmp_path / "backups"
    with ThreadPoolExecutor(max_workers=N_CONCURRENT) as pool:
        infos = list(pool.map(
            lambda i: backup_mod.create_backup(str(source), str(bdir), name=f"b{i:03d}"),
            range(N_CONCURRENT)))
    assert len(infos) == N_CONCURRENT
    rows = backup_mod.list_backups(str(bdir))
    on_disk = [p for p in bdir.glob("*.json") if p.name != "index.json"]
    assert len(rows) == N_CONCURRENT, f"索引只剩 {len(rows)} 行"
    assert len(on_disk) == len(rows)


def test_concurrent_versioning_create_keeps_every_version(tmp_path):
    """24 支并发 `create_version` ⇒ 版本号不重、目录不丢、标签一个都不许蒸发

    改前重复版本号让后者覆盖前者的快照，且 `current.txt` 的「判存在再清」撞上另一支的
    写 ⇒ `PermissionError` 落 500 并留下孤儿目录。
    """
    from augmentor.versioning import VersionManager

    vdir = tmp_path / "versions"
    vm = VersionManager(str(vdir))
    with ThreadPoolExecutor(max_workers=24) as pool:
        infos = list(pool.map(lambda i: vm.create_version(ITEMS, label=f"L{i}"), range(24)))
    ids = [x.version_id for x in infos]
    assert len(set(ids)) == 24, f"版本号重复：{sorted(ids)}"
    labels = {json.loads(m.read_text(encoding="utf-8")).get("label") for m in vdir.glob("*/metadata.json")}
    wanted = {f"L{i}" for i in range(24)}
    assert labels == wanted, f"丢掉的标签：{sorted(wanted - labels)}"


def test_concurrent_version_control_create_keeps_every_row(tmp_path):
    """CLI 版每次调用新建实例，20 支并发 ⇒ 索引行数 == 版本目录数 == 20、0 次异常

    这一族里它最难缠：实例是新的，锁必须按**路径**而不是按实例建键才有意义。
    """
    import augmentor.version_control as vc

    vdir = tmp_path / "vc"
    errors: List[str] = []

    def one(i):
        try:
            return vc.create_version(ITEMS, str(vdir), f"desc{i}")
        except Exception as exc:  # 收敛成人能读的读数，不让一支异常吃掉整份对账
            errors.append(f"{type(exc).__name__}: {exc}")
            return None

    with ThreadPoolExecutor(max_workers=N_CONCURRENT) as pool:
        made = [r for r in pool.map(one, range(N_CONCURRENT)) if r is not None]
    assert errors == [], errors
    dirs = [d for d in vdir.iterdir() if d.is_dir()]
    rows = vc.list_versions(str(vdir))
    assert len(made) == N_CONCURRENT
    assert len(rows) == len(dirs) == N_CONCURRENT, f"索引 {len(rows)} 行 / 盘上 {len(dirs)} 个目录"


def test_transform_failure_writes_nothing(tmp_path, monkeypatch):
    """锁内改坏了 ⇒ 不写盘、异常原样上抛，磁盘逐字节等于进入时

    这条钉的是接线时最容易顺手写错的形状：把写盘放进 `finally` 或 `try` 之外，
    「索引越界」那种请求就会把整份旧快照（连同半路的改动）再写一遍。
    """
    from fastapi import HTTPException

    from api.deps import mutate_json_file

    monkeypatch.setenv("AUGMENTOR_DATA_ROOTS", str(tmp_path))
    target = tmp_path / _seed(tmp_path)
    before = target.read_bytes()

    def boom(items):
        items[0] = {"instruction": "已经改掉", "output": "x"}
        raise HTTPException(status_code=400, detail="故意失败")

    with pytest.raises(HTTPException):
        asyncio.run(mutate_json_file(target, boom))
    assert target.read_bytes() == before, "失败的一趟把改动落盘了"


def test_out_of_range_index_is_judged_on_the_locked_snapshot(tmp_path, monkeypatch):
    """越界判决读的是**锁内刚拿到的那一份**：并发删空之后再改同一格，必须 400 而不是 IndexError

    请求进入时文件还有 40 条、下标 39 合法；等它排到锁时文件已被 20 支删除请求改小 ⇒
    判决必须按新那份来。这里直接构造「比当前长度大」的那一格做同值验证（并发编排留给
    上一支，本支要的是「越界文案出自锁内读数」这句承诺）。
    """
    monkeypatch.setenv("AUGMENTOR_DATA_ROOTS", str(tmp_path))
    _authorize(monkeypatch)
    name = _seed(tmp_path)
    resp = _single(lambda c: c.put(f"/api/data/update/{name}", params={"index": 9999},
                                   json={"instruction": "x", "output": "y"}))
    assert resp.status_code == 400, resp.text
    assert "索引越界" in resp.text
    assert json.loads((tmp_path / name).read_text(encoding="utf-8")) == ITEMS


# ============ 三之二、尺子的四支注入臂 ============

UNLOCKED_SRC = '''
import json
from augmentor.atomic_write import atomic_write_json


class Box:
    def _load_index(self):
        return json.loads(open("i.json", encoding="utf-8").read())

    def _save_index(self):
        atomic_write_json("i.json", self._index)

    def add(self, row):
        self._index = self._load_index()
        self._index["rows"].append(row)
        self._save_index()
'''

LOCKED_SRC = '''
import json
from augmentor.atomic_write import atomic_write_json, path_lock


class Box:
    def _load_index(self):
        return json.loads(open("i.json", encoding="utf-8").read())

    def _save_index(self):
        atomic_write_json("i.json", self._index)

    def add(self, row):
        with path_lock(self._index_file):
            self._index = self._load_index()
            self._index["rows"].append(row)
            self._save_index()
'''


def test_injection_s1_unwrapped_copy_is_named():
    """S1：同一份代码去掉 `with path_lock` ⇒ 名单点名它、违规清单也点名它"""
    sites, bad = census_tree(ast.parse(UNLOCKED_SRC), "box.py")
    assert sites == {"box.py::Box.add": 1}, sites
    assert len(bad) == 1 and "Box.add" in bad[0], bad


def test_injection_s2_wrapped_copy_is_silent():
    """S2：加上锁 ⇒ 仍在名单里（形状没被改坏），但违规清单必须空

    两臂合起来才证明判据量的是「写点是否在锁内」，而不是「有没有 `_save_index`」。
    """
    sites, bad = census_tree(ast.parse(LOCKED_SRC), "box.py")
    assert sites == {"box.py::Box.add": 1}, sites
    assert bad == [], bad


def test_injection_s3_plain_write_is_not_a_rmw():
    """S3 反面：只写不读（一次性导出）不算读-改-写，不许进名单

    否则尺子会把全仓 20 多处导出各报成一格，判据退化成「到处加锁」。
    """
    plain = '''
from augmentor.atomic_write import atomic_write_json


def make_new(rows, output_path):
    atomic_write_json(output_path, rows)
'''
    sites, bad = census_tree(ast.parse(plain), "plain.py")
    assert sites == {}, sites
    assert bad == [], bad


def test_injection_s4_different_targets_are_not_a_rmw():
    """S4：读 A 写 B（源文件只读、目标另建）不算同一份文件上的 RMW

    「同目标」这一条如果不成立，`augment_dataset` 这类「读输入 → 写输出」全都会被判成
    需要加锁，尺子就失去了指名道姓的能力。
    """
    cross = '''
from augmentor.atomic_write import atomic_write_json


def transform(input_path, output_path):
    rows = read_items(input_path)
    atomic_write_json(output_path, rows)
'''
    sites, bad = census_tree(ast.parse(cross), "cross.py")
    assert sites == {}, sites
    assert bad == [], bad


def test_rmw_roster_is_exactly_the_census():
    """现量名单 == 真扫结果：新增一处未登记的 RMW 就红，形状改造导致失踪也红"""
    sites, _bad = census_repo()
    assert sites == MEASURED_RMW_SITES, (
        f"新增 {sorted(set(sites) - set(MEASURED_RMW_SITES))} / "
        f"失踪 {sorted(set(MEASURED_RMW_SITES) - set(sites))}")


def test_no_rmw_write_lands_outside_a_path_lock():
    """判据正面：十格读-改-写的写点全部在锁里（L290 起含 `dependency.py` 的三格）"""
    sites, bad = census_repo()
    assert len(sites) == 10, sites
    assert bad == [], bad


# ============ 四、落点判据没被写松（安全面） ============


def test_escape_backup_name_is_still_rejected(tmp_path):
    """`name="../../evil"` 仍然拒 —— 修形状误判时不许顺手把闸放开

    L273 把 `backup.py` 的落点判据从「比 `resolve()` 字符串」换成「比形状」，这一支钉住
    它仍拦得住逃逸（HTTP 面从前是 200 且真落盘）。
    """
    from augmentor.backup import DatasetBackup
    from augmentor.exceptions import DataValidationError

    source = tmp_path / "src.json"
    source.write_text(json.dumps(ITEMS[:2], ensure_ascii=False), encoding="utf-8")
    db = DatasetBackup(str(tmp_path / "backups"))
    with pytest.raises(DataValidationError):
        db.backup(str(source), name="../../evil")
    assert not (tmp_path / "evil.json").exists()
    assert not (tmp_path.parent / "evil.json").exists()


def test_old_shape_comparison_would_misfire_on_the_same_pair(tmp_path):
    """同一对路径，旧写法（拿两次 `resolve()` 比句柄）当场判「之外」，新写法判「在内」

    这一支是给形状修复留的**方向证据**：如果哪天有人把它改回 `base in landing.parents`
    的裸比较，前一支用例只会变红而说不出为什么；这里直接把两套形状摆在一起量。
    """
    base = tmp_path / "c00"
    landing = Path(EXT_PREFIX + str(tmp_path / "c00" / "b002.json"))
    assert path_lands_within(base, landing) is True
    # 旧判据的算术：`landing.parents` 里全是带前缀的形状，与不带前缀的 `base` 永不相等
    assert base not in landing.parents


def test_mixed_resolve_shapes_no_longer_reject_a_legal_name(tmp_path):
    """带 `\\\\?\\` 扩展长度前缀与不带前缀两种形状必须判「在内」

    这是并发首用 20 次实测 4—12 次误判的根因：`Path.resolve()` 的输出形状取决于调用那
    一刻哪一段存在，同一条表达式里两次 `resolve()` 就能拿到两套形状。
    """
    base = tmp_path / "c00"
    landing = tmp_path / "c00" / "b002.json"
    prefixed_base = Path(EXT_PREFIX + str(base))
    prefixed_landing = Path(EXT_PREFIX + str(landing))
    assert path_lands_within(prefixed_base, landing) is True
    assert path_lands_within(base, prefixed_landing) is True
    assert path_lands_within(base, base) is True
    assert path_lands_within(prefixed_base, prefixed_landing) is True


def test_sibling_prefix_is_not_inside(tmp_path):
    """`backups_evil/x.json` 不算落在 `backups` 之内 —— 名字前缀相同不等于在目录里"""
    base = tmp_path / "backups"
    assert path_lands_within(base, tmp_path / "backups_evil" / "x.json") is False
    assert path_lands_within(base, tmp_path / "other" / "x.json") is False


def test_concurrent_first_use_backup_never_misjudges_a_legal_name(tmp_path):
    """20 支并发首用（目录尚未建）⇒ 一次误判都不许有，索引与盘上一致

    探针里这一形还会被 `resolve()` 的形状差异翻成 400（`DataValidationError`），
    与丢更新是同一族的两个面，因此这里两件事一起对账。
    """
    from augmentor import backup as backup_mod

    source = tmp_path / "src.json"
    source.write_text(json.dumps(ITEMS[:3], ensure_ascii=False), encoding="utf-8")
    bdir = tmp_path / "cold_start_backups"  # 刻意不预先 mkdir
    with ThreadPoolExecutor(max_workers=N_CONCURRENT) as pool:
        infos = list(pool.map(
            lambda i: backup_mod.create_backup(str(source), str(bdir), name=f"c{i:03d}"),
            range(N_CONCURRENT)))
    assert len(infos) == N_CONCURRENT
    assert len(backup_mod.list_backups(str(bdir))) == N_CONCURRENT


def test_probe_temp_roots_stay_outside_the_repo():
    """本文件的并发面全部落在 `tmp_path`：仓库里不许留下索引、备份或版本目录

    写盘类探针的历史事故是「服务自己那份 `config.yaml` 被重写成 186 行」，所以这里把
    「产物不落仓」做成断言而不是纪律。
    """
    race_leftovers = [str(q.relative_to(ROOT)) for q in ROOT.rglob("race.json")]
    assert race_leftovers == [], f"探针产物落进了仓库：{race_leftovers}"
    seeded = [str(q.relative_to(ROOT)) for q in (ROOT / ".backups").glob("b0*.json")] if (ROOT / ".backups").is_dir() else []
    assert seeded == [], f"并发备份面把文件写进了仓库：{seeded}"
    vleft = [str(q.name) for q in (ROOT / ".versions").glob("v*")] if (ROOT / ".versions").is_dir() else []
    assert vleft == [], f"并发版本面把目录写进了仓库：{vleft}"


# ============ 五、红能力：把锁换成「每次给一把新锁」时，并发面必须重现丢更新 ============

#: 本轮被接线的四个消费面都按 `from .atomic_write import path_lock` 绑定了名字，
#: 因此打补丁必须逐模块打；只改 `atomic_write.path_lock` 会一支都拦不住（替身失效）。
PATH_LOCK_CONSUMERS = ("api.deps", "augmentor.backup", "augmentor.version_control",
                       "augmentor.versioning")


def test_arms_have_teeth_when_the_lock_is_faked_away(tmp_path, monkeypatch):
    """注入臂 R1：`path_lock` 退化成「每次新建一把」⇒ 20 支并发改写必然至少丢一次

    判据是「丢更新与瞬时 500 至少中一个」，两边都不许为零：去掉锁之后丢多少取决于线程
    交错（实测 1—19 浮动），而读者撞上 `os.replace` 那一瞬给的 `PermissionError`
    也会回来（50 支实测 3 支 500）。没有这一支，`test_concurrent_updates_all_land`
    有可能只是恰好绿。

    **既有红 2026-10-19（判据靠调度，快机上恒绿）**：只摘锁这一格不稳——读-改-写三段全在
    工作线程里跑，快机上 20 支常被调度成近乎串行，实测 5 跑 3 红 2 绿、`lost` 在 0—19 浮动，
    全量跑正好撞上恒绿那几跑（main 上这条红就是这个形状）。所以本支除了摘锁，还给**读边**
    加一格窗：读回真内容后停 50 ms 再返回，快照与「写回整份」之间的窗口撑到毫秒级，
    20 支快照必然互相重叠（丢更新的充分条件）——判据从「会不会坏」改成「必然坏」。
    """
    import threading
    import time

    import api.deps as deps_mod
    import augmentor.backup as backup_mod
    import augmentor.version_control as vc_mod
    import augmentor.versioning as vm_mod

    modules = {"api.deps": deps_mod, "augmentor.backup": backup_mod,
               "augmentor.version_control": vc_mod, "augmentor.versioning": vm_mod}
    for name in PATH_LOCK_CONSUMERS:
        monkeypatch.setattr(modules[name], "path_lock", lambda _p: threading.RLock())

    # 读边窗（见 docstring「既有红 2026-10-19」）：`mutate_json_file` 内部按模块全局取
    # `read_items`，这里读回**真内容**后再停 50 ms —— 快照与写回整份的窗口被撑开，
    # 20 支快照必然重叠，丢更新不再靠调度运气。
    _real_read_items = deps_mod.read_items

    def _read_then_window(path):
        snapshot = _real_read_items(path)
        time.sleep(0.05)
        return snapshot

    monkeypatch.setattr(deps_mod, "read_items", _read_then_window)

    monkeypatch.setenv("AUGMENTOR_DATA_ROOTS", str(tmp_path))
    _authorize(monkeypatch)
    name = _seed(tmp_path, items=RACE_ITEMS)
    responses = _gather(lambda c, i: c.put(
        f"/api/data/update/{name}", params={"index": i},
        json={"instruction": f"改写 {i}", "output": f"答句 {i}"}))
    codes = [r.status_code for r in responses]
    on_disk = json.loads((tmp_path / name).read_text(encoding="utf-8"))
    landed = sum(1 for i in range(N_CONCURRENT) if on_disk[i].get("instruction") == f"改写 {i}")
    lost = N_CONCURRENT - landed
    bad = sum(1 for code in codes if code != 200)
    # L277 起补的一格：非 200 只许是「读者撞上 `os.replace` 那一瞬」的那档 500。
    # 403 / 401 / 429 是**闸、鉴权或限流**把请求拦在路由之外，那种形状下 `bad > 0` 会让本支
    # 靠「压根没量到丢更新」通过 —— 而它声称咬住的是丢更新那一档，不是回执码非 200。
    # （首跑正是这一形：20 支全 403，`lost` 与 `bad` 一起失真，判据却照样绿。）
    # 鉴权接线后复量（连跑三次）：`lost` = 14 / 11 / 11、`bad` = 1 / 0 / 1 且非 200 全在 500 档
    # ⇒ 每一跑 `lost > 0` 都成立，这一支咬的是丢更新本身而不是回执码形状。
    assert set(codes) <= {200, 500}, codes
    assert lost > 0 or bad > 0, "替身没生效：全落盘且全 200，这一支量不出丢更新"
