# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""L289 / A393：`logging_setup` 的装配段必须序列化 —— 「不撤就翻倍」那一族从并发面回来了

**缺陷形**：`apply_logging_config` 自 A106 起是**条件性**撤装 —— 先拿一份签名
（写出的键 + 三值 + `tuple(root.handlers)` 拓扑）与 `_APPLIED` 比，命中就原样返回缓存摘要，
不命中才 `_detach_installed` 再装一套。这一格是教科书式的 check-then-act，而「判」与「装」
之间没有任何互斥 ⇒ 两支同时进来，两支都 miss、都看到 `not root.handlers`、都各加一条
`StreamHandler` 与一条文件 handler。本模块 docstring 早就把这一族写死过一遍：

    单发重复装配会「每调一次多一个 handler，日志跟着翻倍」—— A106 消掉了单发那一半，
    并发那一半一直没管。

**实测读数**（仓外探针 `Temp/l289q/probe_applied.py`，同进程同 root、`Barrier` 齐发 N 支
`load_config`，改前三跑取同形）：

| N | root 上本模块装的 handler | 装配后一条 INFO 在日志文件里的行数 |
|---|---|---|
| 1 | 2 | 1 |
| 2 | 3 | 2 |
| 8 | 4 | 3 |
| 16 | **7 / 14 / 11**（三跑） | **6 / 13 / 10** |

也就是说对外可观测的后果不是「多一条 handler」这种洁癖，而是**每条日志按到场人数打 N 遍**：
文件被同一行写 N 次、stderr 同样翻倍，而 `RotatingFileHandler` 的切分尺寸由这些重复行决定 ——
一台机器上「日志忽然涨十倍」就是这么来的。

**为什么 L289 那把装配单例锁管不到它**：`get_pipeline` 的锁只保证「建管道那一次」串行，
而 `load_config` 在 API 上还有别的并发入口 —— `api/deps.py` 的数据白名单与上传上限两处缓存
都是「按 `(路径, mtime_ns)` 命中，未命中就整份 `load_config`」⇒ 一次配置写入之后（mtime 变了）
进来的那一批请求会同时 miss、同时装配。所以这一格必须锁在 `logging_setup` 自己身上，
而不是锁在某个调用方。

**修法**：一把模块级 `threading.Lock()`，罩住「签名判据 + 撤 + 装 + 记缓存」整段。
锁放在**公开入口**那一层，原函数体一字未改地搬进 `_apply_logging_config_locked` ⇒
零漂移判据（第 3 组）量的还是搬走之前那份形状。跨进程那一档与 A156 / A158 同口径，不在本轮面。

**四面**：1 结构（锁在不在、装配体在不在锁里）；2 并发行为（16 支齐发之后 handler 条数与行数）；
3 零漂移（单发两次的幂等面与 A106 的快路一字未变）；4 两支牙齿（S1 在内存里摘掉锁 ⇒ 结构面红，
R1 在模块对象上把锁换成 `nullcontext` ⇒ 并发面当场重现翻倍）。

**参照量而不是常数**：上表那组读数出自 root 上空无一物的裸进程（单发装 2 条：控制台 + 文件）。
pytest 跑起来时 root 上挂着日志捕获 handler，`if keys and not root.handlers` 那一档按设计
不触发 ⇒ 单发只装文件那 1 条。所以第 2、4 两组的判据写成「并发装出的条数 == **同一环境**
单发装出的条数」，而不是把 2 抄进断言 —— 抄死的那个数字会随跑法变，量不到缺陷。

**零落盘承诺**：全部读写落在 pytest 的 `tmp_path`；仓库那份 `config.yaml` 与 root logger 上
原有的 handler 由 autouse 夹具逐条对账并复位（改前形状抄 `test_logging_wiring_a97.py` 的
`isolate_root_logger`）。

跑法：`python -m pytest tests/unit/test_logging_apply_concurrency_l289.py -q --no-cov`
"""
import ast
import contextlib
import logging
import threading
import time
from pathlib import Path
from typing import Dict, List

import pytest

from augmentor import logging_setup
from augmentor.config import load_config

AI_DIR = Path(__file__).resolve().parent.parent.parent
LOGGING_SETUP_PY = AI_DIR / "augmentor" / "logging_setup.py"

#: 现量（L289）：一次完整装配往 root 上挂几条 handler，**取决于 root 当时的拓扑** ——
#: 仓外探针的裸进程（root 上没人挂过 handler）装 **2** 条：控制台 + 文件；pytest 环境里
#: root 上挂着日志捕获 handler，`if keys and not root.handlers` 那一档按设计不触发（装第二条
#: 控制台会让每条日志打两遍）⇒ 只装文件那 **1** 条。所以并发判据不许硬写条数，量的是
#: 「并发装配装出的条数 == 同一环境单发装出的条数」，两档形状都在册就不再冒出第三种形状。
MEASURED_INSTALLED_BARE_PROCESS = 2
MEASURED_INSTALLED_UNDER_PYTEST = 1
#: 并发规模：改前 16 支三跑读数 7 / 14 / 11 条，2 支就已翻到 3；取 16 与探针同尺。
N_CONCURRENT = 16


# ============ 夹具 ============


@pytest.fixture(autouse=True)
def isolate_root_logger():
    """把 root logger 的 handler、本模块的 `_INSTALLED` / `_APPLIED` 隔离在单条用例内

    与 `test_logging_wiring_a97.py` 的同名夹具同形：夹具手工清空时绕过了
    `_detach_installed`，所以必须把 `_APPLIED` 一并归 None，否则上一条留下的签名会让
    下一条「该 miss 的调用」假命中。
    """
    root = logging.getLogger()
    saved_handlers, saved_level = root.handlers[:], root.level
    for handler in logging_setup._INSTALLED[:]:
        handler.close()
    logging_setup._INSTALLED.clear()
    logging_setup._APPLIED = None
    root.handlers[:] = []
    root.setLevel(saved_level)
    yield
    for handler in logging_setup._INSTALLED[:]:
        if handler in root.handlers:
            root.removeHandler(handler)
        handler.close()
    logging_setup._INSTALLED.clear()
    logging_setup._APPLIED = None
    root.handlers[:] = saved_handlers
    root.setLevel(saved_level)


def _config_with_logging(tmp_path: Path) -> str:
    """写一份只带 `logging.level` + `logging.file` 两键的仓外配置，返回其路径

    Args:
        tmp_path: pytest 的临时目录

    Returns:
        配置文件的绝对路径字符串
    """
    log_file = (tmp_path / "app.log").as_posix()
    cfg = tmp_path / "config.yaml"
    cfg.write_text(f"logging:\n  level: INFO\n  file: {log_file}\n", encoding="utf-8")
    return str(cfg)


def _fire_concurrently(cfg_path: str, n: int) -> List[str]:
    """`Barrier` 齐发 n 支 `load_config`，返回线程内异常清单

    Args:
        cfg_path: 配置文件路径
        n: 并发规模

    Returns:
        `["类型: 消息", ...]`，全部跑完且无异常时为空表
    """
    barrier = threading.Barrier(n)
    errors: List[str] = []
    guard = threading.Lock()

    def one():
        try:
            barrier.wait(timeout=10)
            load_config(cfg_path)
        except Exception as exc:  # noqa: BLE001  （收敛成人能读的读数，不让一支异常吃掉整份对账）
            with guard:
                errors.append(f"{type(exc).__name__}: {exc}")

    threads = [threading.Thread(target=one, daemon=True) for _ in range(n)]
    for t in threads:
        t.start()
    for t in threads:
        t.join(timeout=30)
    assert not any(t.is_alive() for t in threads), "有线程没跑完，读数不可用"
    return errors


def _write_one_line_and_count(tmp_path: Path) -> int:
    """装配完之后打一条只含 `ONE LINE` 的 INFO，返回日志文件里该行的出现次数

    Args:
        tmp_path: 配置里 `logging.file` 所在目录

    Returns:
        行数（判据看的是「一条日志打几遍」）
    """
    root = logging.getLogger()
    root.setLevel(logging.DEBUG)
    logging.info("ONE LINE")
    for handler in root.handlers:
        handler.flush()
    text = (tmp_path / "app.log").read_text(encoding="utf-8")
    return len([line for line in text.splitlines() if "ONE LINE" in line])


# ============ 1、结构面 ============


def _function_node(text: str, name: str) -> ast.FunctionDef:
    """按名字取顶层函数定义

    Args:
        text: 源码文本
        name: 函数名

    Returns:
        `ast.FunctionDef`
    """
    for node in ast.walk(ast.parse(text)):
        if isinstance(node, ast.FunctionDef) and node.name == name:
            return node
    raise AssertionError(f"no such function: {name}")


def _lock_spans(node: ast.AST, marker: str) -> List[tuple]:
    """`node` 内每个 `with <marker>(...)` / `with <marker>:` 体的行范围

    Args:
        node: 起点 AST 节点
        marker: 锁的名字（`_APPLY_LOCK` / `nullcontext`）

    Returns:
        `[(起始行, 结束行)]`
    """
    spans: List[tuple] = []
    for item in ast.walk(node):
        if not isinstance(item, ast.With):
            continue
        if not any(marker in ast.unparse(ctx.context_expr) for ctx in item.items):
            continue
        starts = [x.lineno for x in ast.walk(item) if hasattr(x, "lineno")]
        ends = [x.end_lineno for x in ast.walk(item) if getattr(x, "end_lineno", None)]
        if starts and ends:
            spans.append((min(starts), max(ends)))
    return spans


def _call_names(node: ast.AST) -> Dict[str, int]:
    """节点内按名字统计调用点

    Args:
        node: 起点 AST 节点

    Returns:
        `{名字: 计数}`
    """
    counts: Dict[str, int] = {}
    for call in ast.walk(node):
        if isinstance(call, ast.Call):
            name = getattr(call.func, "id", None) or getattr(call.func, "attr", None)
            if name:
                counts[name] = counts.get(name, 0) + 1
    return counts


def test_the_apply_entry_is_locked_once():
    """模块级恰好一把 `_APPLY_LOCK`，公开入口体内恰好一个 `with _APPLY_LOCK:`

    两把就变成「锁了但没锁全」：撤与装各自守着一把，中间那段照样能进来第二支。
    """
    text = LOGGING_SETUP_PY.read_text(encoding="utf-8")
    tree = ast.parse(text)
    stores = [node for node in ast.walk(tree)
              if isinstance(node, ast.Assign)
              and any(getattr(t, "id", None) == "_APPLY_LOCK" for t in node.targets)]
    assert len(stores) == 1, stores
    assert "_APPLY_LOCK = threading.Lock()" in text, ast.unparse(stores[0])

    entry = _function_node(text, "apply_logging_config")
    spans = _lock_spans(entry, "_APPLY_LOCK")
    assert len(spans) == 1, spans


def test_the_real_body_lives_inside_the_lock():
    """装配体整段搬进 `_apply_logging_config_locked`，公开入口除「进锁 + 转调」外不留一句

    判据不看「有没有 `_APPLY_LOCK` 这个词」，看的是 `addHandler` 那两处真写点在哪一侧：
    留在锁外就等于罩了个空壳，翻倍照旧。
    """
    text = LOGGING_SETUP_PY.read_text(encoding="utf-8")
    entry = _function_node(text, "apply_logging_config")
    span = _lock_spans(entry, "_APPLY_LOCK")[0]
    inside = [n for n in ast.walk(entry)
              if isinstance(n, (ast.With, ast.Return)) and span[0] <= n.lineno <= span[1]]
    assert len(inside) == 2, [ast.unparse(x)[:60] for x in inside]

    body = _function_node(text, "_apply_logging_config_locked")
    counts = _call_names(body)
    assert counts.get("addHandler") == 2, counts  # 现量：控制台 + 文件各一处
    assert counts.get("_detach_installed") == 1, counts
    assert _lock_spans(body, "_APPLY_LOCK") == [], "装配体里又锁了一次：入口那把成了装饰"


# ============ 2、并发行为面 ============


def _single_load_baseline(cfg_path: str) -> int:
    """同一环境里**单发**一次装配装出的条数 —— 并发面的参照，不硬写成常数

    Args:
        cfg_path: 配置文件路径

    Returns:
        `len(logging_setup._INSTALLED)`
    """
    load_config(cfg_path)
    return len(logging_setup._INSTALLED)


def test_concurrent_loads_install_no_more_than_a_single_load(tmp_path):
    """16 支齐发 `load_config` ⇒ 装出的 handler 条数与单发一模一样，一条日志只打一遍

    判据不看异常、不看返回摘要，看两件事：`_INSTALLED` 的条数（改前 16 支在三跑里是
    7 / 14 / 11 条，而单发参照只有 1～2 条）与日志文件里那一行出现几次（改前 6 / 13 / 10 次）。
    """
    cfg_path = _config_with_logging(tmp_path)
    baseline = _single_load_baseline(cfg_path)
    assert baseline in (MEASURED_INSTALLED_UNDER_PYTEST, MEASURED_INSTALLED_BARE_PROCESS), baseline
    logging_setup._detach_installed(logging.getLogger())
    assert logging_setup._INSTALLED == [], "复位没生效，参照面与并发面量的不是同一段"

    errors = _fire_concurrently(cfg_path, N_CONCURRENT)
    assert errors == [], errors

    root = logging.getLogger()
    assert len(logging_setup._INSTALLED) == baseline, logging_setup._INSTALLED
    assert all(h in root.handlers for h in logging_setup._INSTALLED), "装上的又被谁摘了"
    assert _write_one_line_and_count(tmp_path) == 1


def test_a_second_load_with_the_same_config_is_still_the_fast_path(tmp_path):
    """零漂移面：单发两次的 A106 快路一字未变 —— 第二次不撤不装、`_INSTALLED` 不换对象

    这一支是「加锁没把幂等面改成每请求重装配」的证据；那条快路本来就是为了省掉
    「关掉再重开日志文件」（实测约 103 µs/请求）而立的。
    """
    cfg_path = _config_with_logging(tmp_path)
    load_config(cfg_path)
    installed = list(logging_setup._INSTALLED)
    signature = logging_setup._APPLIED
    assert len(installed) in (MEASURED_INSTALLED_UNDER_PYTEST,
                              MEASURED_INSTALLED_BARE_PROCESS), installed
    load_config(cfg_path)
    assert logging_setup._INSTALLED == installed, "第二次装配换了 handler ⇒ 快路没命中"
    assert logging_setup._APPLIED == signature, "签名被第二次调用重写 ⇒ 每请求都在重装配"


# ============ 3、两支牙齿 ============


def test_injection_s1_unlocking_the_entry_reds_the_structure_ruler():
    """S1：内存里把 `with _APPLY_LOCK:` 换成 `nullcontext` ⇒ 结构面按 `_APPLY_LOCK` 数出零个跨度"""
    text = LOGGING_SETUP_PY.read_text(encoding="utf-8")
    anchor = "    with _APPLY_LOCK:"
    assert text.count(anchor) == 1, "锚点形状变了，先对齐再谈削弱"
    weakened = text.replace(anchor, "    with contextlib.nullcontext():", 1)
    entry = _function_node(weakened, "apply_logging_config")
    assert _lock_spans(entry, "_APPLY_LOCK") == []
    assert len(_lock_spans(entry, "nullcontext")) == 1, "替身没接住转调，这一臂没牙"


def test_injection_r1_null_lock_reproduces_the_duplicate_handlers(tmp_path, monkeypatch):
    """R1：把 `_APPLY_LOCK` 换成 `nullcontext` ⇒ 同一编排必须装出比单发更多的 handler

    读的是 `_INSTALLED` 的条数（装配面自己的账），参照量是**同一环境**下单发一次的条数 ——
    改前 16 支的裸进程读数是 7 / 14 / 11 条对单发的 2 条。不读日志行数是因为替身档下
    多条 `RotatingFileHandler` 指向同一个文件，其中一条被 `_detach_installed` 关过之后
    再写就只剩噪声，判据会被搅浑。
    """
    monkeypatch.setattr(logging_setup, "_APPLY_LOCK", contextlib.nullcontext())
    # 把开文件 handler 这一跳人为加宽：无锁时，两条支能不能同时站在「读 _APPLIED 还
    # 是 None、装完才写 _APPLIED」的窗口里，是这条牙有没有牙的关键。不加宽时本机本地盘
    # 开文件太快 + GIL 串行，16 支齐发常被第一条串行吃完（它先把 _APPLIED 写上），剩下
    # 15 支全是快路、只装一套 ⇒ 牙变假阴性（_INSTALLED == baseline）。CI 负载一波动就
    # 偶发红。这里只在测试里把开文件这一跳睡 20ms，让多条支确定地同时站在临界段里，
    # 翻倍判据才稳定可复现。生产代码一字未动。
    orig_open = logging_setup._open_file_handler
    monkeypatch.setattr(
        logging_setup,
        "_open_file_handler",
        lambda p: (time.sleep(0.02), orig_open(p))[1],
    )
    cfg_path = _config_with_logging(tmp_path)
    baseline = _single_load_baseline(cfg_path)
    logging_setup._detach_installed(logging.getLogger())
    errors = _fire_concurrently(cfg_path, N_CONCURRENT)
    assert errors == [], errors
    assert len(logging_setup._INSTALLED) > baseline, (
        f"摘掉装配锁之后仍只装了一套（{len(logging_setup._INSTALLED)} 对单发的 {baseline}）："
        "这一臂没牙")
