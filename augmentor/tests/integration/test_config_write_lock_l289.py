# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""L289（A384 / A392）：配置写面的两把锁 —— 「改内存 → 整份落盘」同锁，管道单例双检

**缺陷形（两格，实测串成一条因果）**：

1. `api/deps.py::get_pipeline` 的惰性装配是 **check-then-act**（判 `is None` → 建 → 赋），
   没有任何互斥。并发首用 ⇒ 一个进程里同时站着**多只** pipeline，各持一份 `AppConfig`。
   本轮同尺实测（20 支并发各持一只 `TestClient`，配置与数据根全在仓外副本）：路由实际拿到
   **16 只不同的 pipeline 对象**，20 支请求**全回 200**，而盘上只剩最后一支自己那份快照 ——
   另一群写的键原地回到原值（`dedup.threshold` 停在 0.9）。
2. `POST /api/config` 的「判决 → 改内存 → 整份落盘」整段没有互斥（L273 那族丢更新在配置面的
   孪生格）。只修 ① 不修 ② 仍然会丢：同一只共享对象下，A 先取快照、B 改完并写盘、A 用旧快照
   后写 ⇒ B 那次改动被抹掉，两边都回 200。

**为什么 L283 立账时那句「单事件循环下结构上无法交错」不够**：那段同步区里确实没有 `await`，
同一把循环内的两支请求不会交错；但 L283 的读数出自**每线程各一把事件循环**那一档，而它正落在
`threading.RLock` 治得到的范围内。跨进程（多 worker）仍然治不到，那一格沿用 A156 / A158 的
既有口径，不在本轮自决。

**修法**：① 装配改双检锁（外层一次无锁快读 + `with _PIPELINE_LOCK` 内再判一次并赋值）；
② 端点用 `path_lock(config_file_path())` 罩住「`applied` 判决 + 改内存 + 落盘 + 作废缓存」
整段。锁罩整段而不是只罩 `save_config`：后者治不到上面第 2 条那一形。③ `reset_pipeline` 的
裸写 `_pipeline = None` 也进同一把锁 —— 否则它会撞上「另一支正持锁装配、出来时把 None 覆盖
回一只旧管道」，同一族缺陷在写点上的第三格。

> 「无锁 check-then-act」这一族不止 `get_pipeline`：`logging_setup.apply_logging_config` 的
> A106 快路也是同一形，实测 16 支并发装出 7/14/11 条 handler、每条日志打 6/13/10 行，本轮另
> 立 A393 并单独收口，见 `tests/unit/test_logging_apply_concurrency_l289.py`。

**本文件四面**：一、结构面（AST，逐处点名「锁罩没罩在该罩的那一段上」——路由一把、`get_pipeline`
双检、`reset_pipeline` 同锁，附两支内存注入臂）；二、编排面（把「快照已取、盘未换」那一格窗口撑开，
装锁必不落、摘锁必落，读数确定）；三、并发面（20 支真并发：全 200 且两群各自离开原值）；四、单例面
（并发首用只建一只，摘掉双检必建两只）。

所有注入臂走「内存里的源码」或「导入后的模块对象」，仓库文件一字不改；`_pipeline` 与写盘目标
逐支复位，仓档 `config.yaml` 由 autouse 夹具逐支逐字节对账。

跑法：`python -m pytest tests/integration/test_config_write_lock_l289.py -q --no-cov`
"""

import ast
import contextlib
import threading
import time
from pathlib import Path
from typing import Dict, List, Tuple

import pytest
import yaml

import api.deps as deps
import augmentor.atomic_write as atomic_module
from api.main import app
from augmentor.config import AppConfig

ROOT = Path(__file__).resolve().parents[2]
ROUTE_PY = ROOT / "api" / "routes" / "config.py"
DEPS_PY = ROOT / "api" / "deps.py"
REPO_CONFIG = ROOT / "config.yaml"

#: 写鉴权闸（L277）与 `verify_api_key` 两侧同时成立，判据不靠测试豁免过关
WRITE_KEY = "l289-config-lock-key"

#: 编排面两批写入的值，都避开夹具里的原值 0.6 / 0.9 ⇒「回到原值」就是整批丢
VALUE_A = 0.333
VALUE_B = 0.444

#: 夹具写出的原值，R1 臂拿它当「丢回旧快照」的签名
ORIG_Q, ORIG_D = 0.6, 0.9


@pytest.fixture(autouse=True)
def no_repo_config_damage():
    """本文件任何一支用例都不许碰仓库自己那份 config.yaml（逐字节对账）"""
    before = REPO_CONFIG.read_bytes()
    yield
    assert REPO_CONFIG.read_bytes() == before, "用例把写入落到了仓库自己的配置上"


def _factory_yaml() -> Dict:
    """一份够 `load_config` 装配的顶层形状，两键原值刻意远离判据值

    Returns:
        含 quality / dedup 两节的 dict
    """
    return {"quality": {"threshold": ORIG_Q}, "dedup": {"threshold": ORIG_D}}


@pytest.fixture
def isolated_pipeline(tmp_path, monkeypatch):
    """把服务自身的配置指到仓外副本，并交出那只隔离出来的 live config

    `get_pipeline` 的缓存是进程级的，`_pipeline` 与环境变量必须成对复位，否则本文件建出来的
    假管道或仓外路径会留给后面的用例。
    """
    cfg = tmp_path / "config.yaml"
    cfg.write_text(yaml.safe_dump(_factory_yaml(), sort_keys=False, allow_unicode=True),
                   encoding="utf-8")
    saved = deps._pipeline
    monkeypatch.setenv("AUGMENTOR_CONFIG_PATH", str(cfg))
    monkeypatch.setenv("AUGMENTOR_DATA_ROOTS", str(tmp_path / "data"))
    monkeypatch.setenv(deps.API_KEY_ENV, WRITE_KEY)
    (tmp_path / "data").mkdir(parents=True, exist_ok=True)
    deps.reset_pipeline()
    yield cfg, deps.get_pipeline().config
    deps.reset_pipeline()
    deps._pipeline = saved


def _post(payload: Dict) -> Tuple[int, str]:
    """打一支 `POST /api/config`，返回 `(状态码, 正文前 200 字)`

    不进 `with` 上下文 ⇒ 不跑 lifespan，每次请求各起一支门户线程，与 L283 的并发形状一致。
    不在这里断言状态码：本文件的判据要么跨线程收集后统一判，要么是盘上读数。
    """
    from fastapi.testclient import TestClient

    resp = TestClient(app, headers={"X-API-Key": WRITE_KEY}).post("/api/config", json=payload)
    return resp.status_code, resp.text[:200]


# ============ 一、结构面 ============


def _function_node(text: str, name: str) -> ast.AST:
    """从源码文本里按名字取函数定义节点

    Args:
        text: 源码文本
        name: 函数名

    Returns:
        `ast.FunctionDef` / `ast.AsyncFunctionDef`
    """
    for node in ast.walk(ast.parse(text)):
        if isinstance(node, (ast.FunctionDef, ast.AsyncFunctionDef)) and node.name == name:
            return node
    raise AssertionError(f"没有这个函数：{name}")


def _lock_spans(fn: ast.AST, marker: str) -> List[Tuple[int, int]]:
    """函数体内每个「上下文表达式含 `marker`」的 `with` 跨度

    Args:
        fn: 函数节点
        marker: 必须出现在 `with` 表达式里的子串

    Returns:
        `[(起行, 止行)]`
    """
    spans = []
    for node in ast.walk(fn):
        if not isinstance(node, (ast.With, ast.AsyncWith)):
            continue
        if not any(marker in ast.unparse(item.context_expr) for item in node.items):
            continue
        lines = [x.lineno for x in ast.walk(node) if hasattr(x, "lineno")]
        spans.append((min(lines), max(lines)))
    return spans


def _statement_lines(fn: ast.AST, predicate) -> List[int]:
    """函数体内满足 `predicate` 的节点行号（升序，不含嵌套函数自己的行）

    Args:
        fn: 函数节点
        predicate: 吃节点返回布尔

    Returns:
        行号列表
    """
    nested: set = set()
    for node in ast.walk(fn):
        if isinstance(node, (ast.FunctionDef, ast.AsyncFunctionDef)) and node is not fn:
            nested.update(range(node.lineno, (node.end_lineno or node.lineno) + 1))
    hits = []
    for node in ast.walk(fn):
        line = getattr(node, "lineno", 0)
        if node is fn or line <= fn.lineno or line in nested:
            continue
        if predicate(node):
            hits.append(line)
    return sorted(hits)


def _is_call(node, name: str) -> bool:
    """`node` 是不是名为 `name` 的调用（认裸名与 `x.name(...)` 末段）

    Args:
        node: AST 节点
        name: 名字

    Returns:
        布尔
    """
    if not isinstance(node, ast.Call):
        return False
    return getattr(node.func, "id", None) == name or getattr(node.func, "attr", None) == name


def test_route_wraps_mutation_and_write_in_one_path_lock():
    """端点里恰好一把按路径的锁，锁键是配置文件本身，三个落点全在其跨度内

    三个落点是 `default_model` 的赋值、`apply_section_update` 的调用、`save_config` 的调用 ——
    少罩住任何一个，另一批的旧快照就能把它抹掉。
    """
    fn = _function_node(ROUTE_PY.read_text(encoding="utf-8"), "update_config")
    spans = _lock_spans(fn, "path_lock")
    assert len(spans) == 1, spans
    lo, hi = spans[0]
    context = [node for node in ast.walk(fn) if isinstance(node, (ast.With, ast.AsyncWith))
               and any("path_lock" in ast.unparse(item.context_expr) for item in node.items)]
    assert ast.unparse(context[0].items[0].context_expr) == "path_lock(config_file_path())"

    assigns = _statement_lines(fn, lambda n: isinstance(n, ast.Assign)
                               and any(getattr(t, "attr", None) == "default_model"
                                       for t in n.targets))
    calls_update = _statement_lines(fn, lambda n: _is_call(n, "apply_section_update"))
    calls_save = _statement_lines(fn, lambda n: _is_call(n, "save_config"))
    assert len(assigns) == 1 and len(calls_update) == 1 and len(calls_save) == 1, (
        assigns, calls_update, calls_save)
    for line in assigns + calls_update + calls_save:
        assert lo <= line <= hi, f"落点 {line} 不在锁跨度 {lo}-{hi} 之内"


def test_get_pipeline_is_double_checked_under_one_lock():
    """装配单例必须双检：两次 `is None` 判断、一次赋值，且赋值在锁跨度之内"""
    fn = _function_node(DEPS_PY.read_text(encoding="utf-8"), "get_pipeline")
    spans = _lock_spans(fn, "_PIPELINE_LOCK")
    assert len(spans) == 1, spans
    lo, hi = spans[0]

    checks = _statement_lines(fn, lambda n: isinstance(n, ast.If)
                              and "_pipeline" in ast.unparse(n.test)
                              and "None" in ast.unparse(n.test))
    stores = _statement_lines(fn, lambda n: isinstance(n, ast.Assign)
                              and any(getattr(t, "id", None) == "_pipeline" for t in n.targets))
    assert len(checks) == 2, checks
    assert len(stores) == 1 and lo <= stores[0] <= hi, stores


def test_reset_shares_that_same_build_lock():
    """复位也进同一把锁：装配与复位共用一把 ⇒ 模块级 `with _PIPELINE_LOCK:` 现量 2 处

    裸写 `_pipeline = None` 会撞上「另一支正持锁装配、出来时把 None 覆盖回一只旧管道」，
    复位动作凭空丢失（`get_pipeline` 之后拿到的还是装配前那一份）。
    """
    text = DEPS_PY.read_text(encoding="utf-8")
    assert text.count("with _PIPELINE_LOCK:") == 2, "装配锁的落点数变了，先对齐再谈判据"
    fn = _function_node(text, "reset_pipeline")
    assert len(_lock_spans(fn, "_PIPELINE_LOCK")) == 1, _lock_spans(fn, "_PIPELINE_LOCK")
    stores = _statement_lines(fn, lambda n: isinstance(n, ast.Assign)
                              and any(getattr(t, "id", None) == "_pipeline" for t in n.targets))
    lo, hi = _lock_spans(fn, "_PIPELINE_LOCK")[0]
    assert len(stores) == 1 and lo <= stores[0] <= hi, stores


def test_injection_s1_unlocked_route_body_has_no_span():
    """S1：把端点那把锁换成 `nullcontext` ⇒ 按 `path_lock` 数出零个跨度，三个落点全被替身接住

    没有这一臂，`_lock_spans` 只是「源码里有没有 path_lock 这个词」的另一种写法。
    """
    text = ROUTE_PY.read_text(encoding="utf-8")
    anchor = "with path_lock(config_file_path()):"
    assert text.count(anchor) == 1, "锚点形状变了，先对齐再谈削弱"
    weakened = text.replace(anchor, "with contextlib.nullcontext():", 1)
    fn = _function_node(weakened, "update_config")
    assert _lock_spans(fn, "path_lock") == []
    lo, hi = _lock_spans(fn, "nullcontext")[0]
    assigns = _statement_lines(fn, lambda n: isinstance(n, ast.Assign)
                               and any(getattr(t, "attr", None) == "default_model"
                                       for t in n.targets))
    calls_update = _statement_lines(fn, lambda n: _is_call(n, "apply_section_update"))
    calls_save = _statement_lines(fn, lambda n: _is_call(n, "save_config"))
    assert len(assigns) == 1 and len(calls_update) == 1 and len(calls_save) == 1, (
        assigns, calls_update, calls_save)
    for line in assigns + calls_update + calls_save:
        assert lo <= line <= hi, f"替身没接住落点 {line}，这一臂没牙"


def test_injection_s2_single_check_shape_has_no_span():
    """S2：把装配锁换成 `nullcontext` ⇒ 同一份代码按 `_PIPELINE_LOCK` 数出零个跨度

    与 S1 同形：替身必须照样接住赋值行，否则这一臂只是把源码改坏了，量不到判据。
    定位用的是「这一行之后紧跟双检第二层」而不是整串多行锚点 —— `api/deps.py` 是纯 CRLF，
    多行锚点里的 `\\n` 永远匹配不上；而 `reset_pipeline` 现在也进同一把锁，所以必须逐行找、
    只换装配那一处（整串 `with _PIPELINE_LOCK:` 在产品码里是 2 处）。
    """
    lines = DEPS_PY.read_text(encoding="utf-8").splitlines(keepends=True)
    hits = [i for i in range(len(lines) - 1)
            if lines[i].strip() == "with _PIPELINE_LOCK:"
            and lines[i + 1].strip() == "if _pipeline is None:"]
    assert len(hits) == 1, hits
    swapped = lines[hits[0]].replace("_PIPELINE_LOCK", "contextlib.nullcontext()")
    weakened = "".join(lines[:hits[0]] + [swapped] + lines[hits[0] + 1:])
    fn = _function_node(weakened, "get_pipeline")
    assert _lock_spans(fn, "_PIPELINE_LOCK") == []
    lo, hi = _lock_spans(fn, "nullcontext")[0]
    stores = _statement_lines(fn, lambda n: isinstance(n, ast.Assign)
                              and any(getattr(t, "id", None) == "_pipeline" for t in n.targets))
    assert len(stores) == 1 and lo <= stores[0] <= hi, "替身没接住赋值行：这一臂没牙"


# ============ 二、编排面（确定先后） ============


@pytest.fixture
def staged_writer(monkeypatch):
    """把 `atomic_write_text` 换成「进写窗口就出声、等放行再落盘」的替身

    只延迟**第一次**调用：第二批因此能立刻落盘，而第一批停在「快照已取、盘未换」那一格里 ——
    丢更新就发生在那一格，把它撑开才知道锁到底罩住了什么。
    """
    hold = threading.Event()
    gate = threading.Event()
    state = {"first": True}
    real = atomic_module.atomic_write_text

    def spy(file_path, text):
        if state["first"]:
            state["first"] = False
            hold.set()
            assert gate.wait(timeout=5), "放行没来：编排失效"
        return real(file_path, text)

    monkeypatch.setattr(atomic_module, "atomic_write_text", spy)
    return hold, gate


def _stage_two_batches(hold, gate, allow_second_to_finish):
    """A 批停在写窗口里时放 B 批进去，返回 `(两批状态码, 第二批是否被拦在门外)`

    Args:
        hold: A 批进入写窗口时置位的信号
        gate: 放行 A 批落盘的信号
        allow_second_to_finish: 给第二批的等待上限（秒）—— 装锁时它排不到，摘锁时它跑完

    Returns:
        `([(码, 正文), ...], second_alive)`
    """
    codes: List[Tuple[int, str]] = []
    lock = threading.Lock()

    def run(payload):
        code, text = _post(payload)
        with lock:
            codes.append((code, text))

    a = {"quality": {"threshold": VALUE_A}}
    b = {"dedup": {"threshold": VALUE_B}}
    first = threading.Thread(target=run, args=(a,), daemon=True)
    first.start()
    assert hold.wait(timeout=5), "第一批没进到写窗口里"
    second = threading.Thread(target=run, args=(b,), daemon=True)
    second.start()
    second.join(timeout=allow_second_to_finish)
    alive = second.is_alive()
    gate.set()
    first.join(timeout=10)
    second.join(timeout=10)
    assert not first.is_alive() and not second.is_alive(), "有请求没跑完，读数不可用"
    return codes, alive


def test_the_lock_holds_across_snapshot_and_write(isolated_pipeline, staged_writer):
    """正判据：A 停在写窗口里时 B 改完并落盘 ⇒ 排到锁后的 A 那份快照也必须带上 B 的改动

    锁罩的是「判决 + 改 + 写」整段，所以 B 的整段只能排在 A 之后，盘上两键同时为新值。
    """
    cfg, live = isolated_pipeline
    codes, blocked = _stage_two_batches(*staged_writer, allow_second_to_finish=0.3)
    assert blocked, "第二批没被锁拦住：路由面根本没有互斥"
    assert [c for c, _ in codes] == [200, 200], codes

    on_disk = yaml.safe_load(cfg.read_text(encoding="utf-8"))
    assert on_disk["quality"]["threshold"] == VALUE_A
    assert on_disk["dedup"]["threshold"] == VALUE_B, "锁内整段没罩住改与写，B 那一批被旧快照抹掉"
    assert live.quality.threshold == VALUE_A and live.dedup.threshold == VALUE_B


def test_injection_r1_removing_the_route_lock_loses_a_batch(isolated_pipeline, staged_writer,
                                                            monkeypatch):
    """R1：`path_lock` 退化成「每次给一把新锁」⇒ 同一段编排必然整批丢 B，而且两支都回 200

    这一臂是上一支的牙齿：判据不看状态码，只看盘上 B 那一格还在不在，并要求丢的形状就是
    「回到 A 那份旧快照的原值」。
    """
    cfg, _live = isolated_pipeline
    monkeypatch.setattr(atomic_module, "path_lock", lambda _p: threading.RLock())
    codes, blocked = _stage_two_batches(*staged_writer, allow_second_to_finish=10)
    assert not blocked, "第二批仍被拦住：替身没生效"
    assert [c for c, _ in codes] == [200, 200], codes

    on_disk = yaml.safe_load(cfg.read_text(encoding="utf-8"))
    assert on_disk["quality"]["threshold"] == VALUE_A
    assert on_disk["dedup"]["threshold"] == ORIG_D, (
        f"摘掉路由锁之后 B 没丢（读到 {on_disk['dedup']['threshold']}）：判据恒真")


# ============ 三、并发面（真 20 支） ============


def test_twenty_concurrent_writes_all_land(isolated_pipeline):
    """20 支并发（每支一把事件循环，10 支写 quality、10 支写 dedup）⇒ 全 200、两群都离开原值

    改前的两笔读数都在这里：500 若干（`os.replace` 撞 Windows 瞬时拒绝访问顶穿 5×20 ms 退避），
    以及一整群原地回到原值。本支刻意不问「同键的并发里哪一条赢」——那由排程决定——只问
    「这一群有没有被整批抹掉」。
    """
    cfg, _live = isolated_pipeline
    codes: List[int] = []
    lock = threading.Lock()

    def one(group, i):
        payload = {"quality": {"threshold": 0.3 + i / 1000.0}} if group == "Q" \
            else {"dedup": {"threshold": 0.4 + i / 1000.0}}
        code, text = _post(payload)
        with lock:
            codes.append(code) if code == 200 else codes.append(code)
            if code != 200:
                print(text)

    threads = [threading.Thread(target=one, args=("Q", i)) for i in range(10)] \
        + [threading.Thread(target=one, args=("D", i)) for i in range(10)]
    for t in threads:
        t.start()
    for t in threads:
        t.join(timeout=30)
    assert not any(t.is_alive() for t in threads), "有请求没跑完"
    assert len(codes) == 20 and set(codes) == {200}, codes

    on_disk = yaml.safe_load(cfg.read_text(encoding="utf-8"))
    q_values = {round(0.3 + i / 1000.0, 6) for i in range(10)}
    d_values = {round(0.4 + i / 1000.0, 6) for i in range(10)}
    q, d = on_disk["quality"]["threshold"], on_disk["dedup"]["threshold"]
    assert any(abs(q - x) < 1e-9 for x in q_values), f"quality 整批丢，盘上是 {q}"
    assert any(abs(d - x) < 1e-9 for x in d_values), f"dedup 整批丢，盘上是 {d}"


# ============ 四、单例面 ============


class _CountingPipeline:
    """替身管道：进构造就出声，再停一小会儿 —— 把 check-then-act 的窗口撑成可判的形状"""

    builds = 0
    entered = threading.Event()

    def __init__(self, config):
        type(self).builds += 1
        type(self).entered.set()
        time.sleep(0.05)
        self.config = config

    @classmethod
    def reset(cls):
        cls.builds = 0
        cls.entered = threading.Event()


@pytest.fixture
def pipeline_factory(monkeypatch):
    """把装配面换成计数替身（`load_config` 一并替掉，这一族不读盘）"""
    _CountingPipeline.reset()
    saved = deps._pipeline
    deps.reset_pipeline()
    monkeypatch.setattr(deps, "load_config", lambda _p: AppConfig())
    monkeypatch.setattr(deps, "AugmentorPipeline", _CountingPipeline)
    yield
    deps.reset_pipeline()
    deps._pipeline = saved


def _build_under_first_construction() -> Tuple[int, int, bool]:
    """第一支正在构造时放第二支进去取

    Returns:
        `(窗口内的构造次数, 两跑结束后的总次数, 两次返回是否同一只)`

    窗口内那一次读数才是判据：被锁拦住的第二支还没进到构造函数里，读数停在 1；没有锁的第二支
    此刻已经站在自己的构造函数中，读数变成 2。
    「第二支还活着」不能当判据 —— 没锁时它同样活着，只是活着的原因换成了「自己也在睡」。
    """
    got: List[object] = []
    first = threading.Thread(target=lambda: got.append(deps.get_pipeline()), daemon=True)
    first.start()
    assert _CountingPipeline.entered.wait(timeout=5), "第一支没进到构造函数里"
    second = threading.Thread(target=lambda: got.append(deps.get_pipeline()), daemon=True)
    second.start()
    time.sleep(0.02)
    mid = _CountingPipeline.builds
    first.join(timeout=10)
    second.join(timeout=10)
    assert not first.is_alive() and not second.is_alive(), "有线程没跑完"
    assert len(got) == 2, got
    return mid, _CountingPipeline.builds, got[0] is got[1]


def test_concurrent_first_use_builds_exactly_one_pipeline(pipeline_factory):
    """正判据：构造进行中第二支被 `with _PIPELINE_LOCK` 拦在门外 ⇒ 只建一只、拿到同一只"""
    mid, builds, same = _build_under_first_construction()
    assert mid == 1, f"第一支还在构造时第二支就进了构造函数：双检锁没罩住装配那一步（{mid}）"
    assert builds == 1, builds
    assert same, "两次取用拿到不同对象：进程里同时站着好几个「当前生效配置」"


def test_injection_r2_single_check_builds_two_pipelines(pipeline_factory, monkeypatch):
    """R2：把双检锁换成 `nullcontext` ⇒ 同一编排必建两只、两次拿到不同对象

    这一臂钉的是「`_PIPELINE_LOCK` 那一条真在起作用」，而不是「线程调度恰好没撞上」。
    """
    monkeypatch.setattr(deps, "_PIPELINE_LOCK", contextlib.nullcontext())
    mid, builds, same = _build_under_first_construction()
    assert mid == 2, f"窗口内只建了 {mid} 只：替身没生效，这一臂没牙"
    assert builds == 2, builds
    assert not same, "摘掉装配锁之后两次还拿到同一只：判据恒真"
