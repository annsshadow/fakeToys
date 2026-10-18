"""L275 守卫：并发增强在共享单例上的「运行身份」归属。

缺陷形（不是慢，是**写错对象**）：`api/deps._pipeline` 是进程级单例，
`POST /api/augment/start` 把 `p.augment_dataset(...)` 放进 `BackgroundTasks`，
于是两个任务同时跑时共用同一个 `CheckpointManager`。改前
`update_progress(index, success)` 不带任务身份 ⇒ 每一笔进度写的都是
「此刻 `_current_checkpoint` 指向的那一份」。实测 A=200 与 B=50 并发：230 笔进度里
**170 笔落在对方断点上**，B 的 `completed_indices` 多出 150 个越界下标
（值域 0..199 而 `total_items=50`），A 在盘上停在 `processed_items=0`，
还留下一枚没人认领的增量文件；报告的 `errors` 同样共享，A 的 139 笔明细**全部**来自 B。

L273 的 `path_lock` + `self._lock` 治的是「同一份文件别写坏」，治不到
「这一笔属于哪个任务」——那是身份维，不是串行维。

本文件钉三件事：
① 接线名单（AST）：产品代码里每一个对身份敏感方法的调用，身份参必须给到；
② 行为对账：两个线程各持一份断点，barrier 对齐后交错写完，逐格核对归属；
   并留**环境态反证臂**（同一写法去掉 `checkpoint=` 就重现污染）——
   那支是尺子的牙齿，用来否决「线程调度恰好没重叠」的假绿；
③ 盘上格式：`_RunState` 挂在实例属性上而不是 dataclass 字段上，所以 `asdict()`
   的键集一字未改，旧断点文件照旧读得回来。
"""
import ast
import dataclasses
import json
import os
import threading
from queue import Queue

from augmentor.checkpoint import CheckpointManager, _state_of
from augmentor.config import AppConfig
from augmentor.pipeline import AugmentorPipeline

REPO = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
# 身份参在调用点的最小实参个数：`update_progress(index, ...)` 的第 4 位才是断点
IDENTITY_ARITY = {
    "update_progress": 4,
    "save_checkpoint": 1,
    "get_remaining_indices": 1,
    "get_progress": 1,
    "_save_delta": 1,
}

# ① 接线名单：(相对路径, 方法名) -> (调用总数, 其中把身份给到的数量)。
# 多一格、少一格、或把带身份的写成不带身份的，当场红。
# `api/routes/augment.py` 那一格是**已立账的待拍面**（进度端点读「最后加载的那份」），
# 它钉在这里而不是被顺手修掉，是因为改它等于改响应面。
# L277 收了 A368 的异步入口（`augment_async` 从前根本不读 `use_checkpoint`）：
# pipeline 那三行的总数因此从 4/1/1 涨到 6/2/2——`update_progress` 四笔在同步入口、
# 两笔在异步入口（成功/失败各一），`save_checkpoint` 与 `get_remaining_indices` 各入口一笔。
# 涨的每一格都当场核过身份参（异步那四处是 `checkpoint=ckpt` / `save_checkpoint(ckpt)` /
# `get_remaining_indices(ckpt)`），行为面另有 `tests/unit/test_async_checkpoint_resume_l277.py`。
EXPECTED_PRODUCT_CALLS = {
    ("augmentor/pipeline.py", "update_progress"): (6, 6),
    ("augmentor/pipeline.py", "save_checkpoint"): (2, 2),
    ("augmentor/pipeline.py", "get_remaining_indices"): (2, 2),
    ("augmentor/pipeline.py", "get_progress"): (1, 1),
    ("augmentor/checkpoint.py", "_save_delta"): (2, 2),
    ("api/routes/augment.py", "get_progress"): (1, 0),
}


def _rel(path):
    return os.path.relpath(path, REPO).replace(os.sep, "/")


def _product_py_files():
    for root in ("augmentor", "api", "cli"):
        base = os.path.join(REPO, root)
        for dirpath, _dirnames, filenames in os.walk(base):
            if "__pycache__" in dirpath:
                continue
            for name in filenames:
                if name.endswith(".py"):
                    yield os.path.join(dirpath, name)


def _call_sites(source, filename):
    """扫出「对断点管理器那几个身份敏感方法的调用」"""
    out = []
    for node in ast.walk(ast.parse(source)):
        if not isinstance(node, ast.Call) or not isinstance(node.func, ast.Attribute):
            continue
        attr = node.func.attr
        if attr not in IDENTITY_ARITY:
            continue
        kwargs = {k.arg for k in node.keywords if k.arg}
        has_identity = "checkpoint" in kwargs or \
            len(node.args) >= IDENTITY_ARITY[attr]
        out.append((filename, attr, has_identity))
    return out


def _scan():
    tally = {}
    for path in _product_py_files():
        with open(path, encoding="utf-8") as f:
            src = f.read()
        for filename, attr, has_identity in _call_sites(src, _rel(path)):
            total, wired = tally.get((filename, attr), (0, 0))
            tally[(filename, attr)] = (total + 1, wired + (1 if has_identity else 0))
    return tally


class TestWiringRoster:
    """① 接线名单由 AST 现量，不靠记忆"""

    def test_product_call_sites_match_roster(self):
        got = _scan()
        assert got == EXPECTED_PRODUCT_CALLS, (
            "断点身份参的调用点变了：本账钉的是 %s，实量 %s" % (
                sorted(EXPECTED_PRODUCT_CALLS.items()), sorted(got.items()))
        )

    def test_roster_scanner_bites_without_identity_arg(self):
        """尺子自证：同一处调用去掉 `checkpoint=` 之后，扫描必须报「没给身份」"""
        wired = _call_sites("m.update_progress(3, True, checkpoint=ck)", "x.py")
        ambient = _call_sites("m.update_progress(3, True)", "x.py")
        assert wired == [("x.py", "update_progress", True)], wired
        assert ambient == [("x.py", "update_progress", False)], ambient

    def test_roster_scanner_counts_positional_identity(self):
        """身份参走位置（`save_checkpoint(ck)`）也算给到——扫描器不许只认 kwarg"""
        assert _call_sites("m.save_checkpoint(ck)", "x.py") == [
            ("x.py", "save_checkpoint", True)]
        assert _call_sites("m.save_checkpoint()", "x.py") == [
            ("x.py", "save_checkpoint", False)]

    def test_pipeline_hands_its_own_list_to_worker_threads(self):
        """`executor.submit` 那一处必须把本批 `errors` 交给工作线程"""
        with open(os.path.join(REPO, "augmentor", "pipeline.py"),
                  encoding="utf-8") as f:
            src = ast.parse(f.read())
        submits = [n for n in ast.walk(src)
                   if isinstance(n, ast.Call)
                   and isinstance(n.func, ast.Attribute)
                   and n.func.attr == "submit"]
        assert len(submits) == 1, "pipeline 里的 submit 应当恰好一处"
        args = submits[0].args
        # (方法, idx, item, 质量档, 队列, 本批清单)
        assert len(args) == 6, "并行支路交给工作线程的实参数变了"
        assert args[0].attr == "_process_single_item"
        assert args[-1].id == "errors", (
            "没把本批清单交给工作线程 ⇒ 明细又会落回共享实例属性"
        )


def _mk(tmp_path, name, n):
    mgr = CheckpointManager(str(tmp_path / name), auto_save_interval=10)
    return mgr, mgr.create_checkpoint("task-" + name, n)


def _write_loop(mgr, ck, start, count, barrier, with_identity):
    """barrier 对齐后各写 `count` 笔；下标从 `start` 起，好让越界数可数"""
    barrier.wait()
    for i in range(count):
        if with_identity:
            mgr.update_progress(start + i, True, checkpoint=ck)
        else:
            mgr.update_progress(start + i, True)


def _run_two_threads(mgr, ck_a, ck_b, with_identity):
    """两线程**同时**各写自己的 100 / 20 笔（barrier 保证起点重合，不靠运气）"""
    barrier = threading.Barrier(2)
    th_a = threading.Thread(target=_write_loop,
                            args=(mgr, ck_a, 0, 100, barrier, with_identity),
                            name="A")
    th_b = threading.Thread(target=_write_loop,
                            args=(mgr, ck_b, 0, 20, barrier, with_identity),
                            name="B")
    th_a.start()
    th_b.start()
    th_a.join()
    th_b.join()


class TestRunScopedAttribution:
    """② 行为对账：归属由手里那份对象决定，与「谁最后加载」无关"""

    def test_two_threads_do_not_cross_write(self, tmp_path):
        mgr, ck_a = _mk(tmp_path, "a", 100)
        # B 后创建 ⇒ 改前 `_current_checkpoint` 是 B，A 的 100 笔全写进 B
        ck_b = mgr.create_checkpoint("task-b", 20)
        _run_two_threads(mgr, ck_a, ck_b, True)

        assert set(ck_a.completed_indices) == set(range(100)), ck_a.completed_indices
        assert set(ck_b.completed_indices) == set(range(20)), ck_b.completed_indices
        assert len(ck_a.completed_indices) == 100 and len(ck_b.completed_indices) == 20
        assert ck_a.processed_items == 100 and ck_b.processed_items == 20
        # 待保存集合也跟着对象走：各自按自己的 interval 早已落盘
        assert _state_of(ck_a).pending_completed == set()
        assert _state_of(ck_b).pending_completed == set()

    def test_ambient_write_reproduces_the_contamination(self, tmp_path):
        """反证臂（尺子的牙齿）：同一写法不带 `checkpoint=`，污染当场重现

        这一支不是「顺便复现旧行为」，它证明上一条测试真会红：把接线的
        `checkpoint=` 摘掉，产品就回到 L275 量到的那一形——A 在盘上一条没记，
        B 收下 120 笔，其中 80 笔下标越出自己的 `total_items`。
        """
        mgr, ck_a = _mk(tmp_path, "amb_a", 100)
        ck_b = mgr.create_checkpoint("task-amb_b", 20)
        _run_two_threads(mgr, ck_a, ck_b, False)

        assert ck_a.processed_items == 0, (
            "环境态这一支已经没有污染力了 ⇒ 上一条测试的『不串』是假绿"
        )
        assert len(ck_b.completed_indices) == 120
        out_of_range = [i for i in ck_b.completed_indices if i >= ck_b.total_items]
        assert len(out_of_range) == 80, len(out_of_range)

    def test_delta_file_belongs_to_the_watched_checkpoint(self, tmp_path):
        """增量落点也按身份走：A 的 10 笔增量不许写进 B 的 delta 文件"""
        mgr, ck_a = _mk(tmp_path, "d", 100)
        ck_b = mgr.create_checkpoint("task-bd", 500)
        for i in range(10):
            mgr.update_progress(i, True, checkpoint=ck_a)

        with open(mgr._get_delta_path(ck_a.task_id), encoding="utf-8") as f:
            mine = json.loads(f.readline())
        assert sorted(mine["completed"]) == list(range(10))
        assert not os.path.exists(mgr._get_delta_path(ck_b.task_id)), (
            "A 的增量落到了 B 的 delta 文件上 ⇒ 归属仍由『谁最后加载』决定"
        )

    def test_get_progress_reads_the_named_checkpoint(self, tmp_path):
        """报告里的 `progress` 读自己那份，不读「最后加载的那份」"""
        mgr, ck_a = _mk(tmp_path, "g", 10)
        ck_b = mgr.create_checkpoint("task-bg", 40)
        for i in range(4):
            mgr.update_progress(i, True, checkpoint=ck_a)
            mgr.update_progress(i, True, quality_score=0.5, checkpoint=ck_b)

        pa, pb = mgr.get_progress(ck_a), mgr.get_progress(ck_b)
        assert pa["task_id"] == ck_a.task_id and pb["task_id"] == ck_b.task_id
        assert pa["processed_items"] == 4 and pb["processed_items"] == 4
        assert pa["total_items"] == 10 and pb["total_items"] == 40
        # 环境态读法仍然读「最后加载的那份」（B）——那是待拍的进度端点面，本账不改
        assert mgr.get_progress()["task_id"] == ck_b.task_id
        assert pa["avg_quality_score"] == 0.0, "A 没评分，读到的不该是 B 的 0.5"
        assert pb["avg_quality_score"] == 0.5

    def test_remaining_indices_follow_the_named_checkpoint(self, tmp_path):
        mgr, ck_a = _mk(tmp_path, "r", 5)
        ck_b = mgr.create_checkpoint("task-br", 5)
        mgr.update_progress(0, True, checkpoint=ck_a)
        assert mgr.get_remaining_indices(ck_a) == [1, 2, 3, 4]
        assert mgr.get_remaining_indices(ck_b) == [0, 1, 2, 3, 4]

    def test_save_checkpoint_finishes_the_named_one(self, tmp_path):
        """收尾只收自己那一份：既写完 A，也不许顺手删掉 B 的增量"""
        mgr, ck_a = _mk(tmp_path, "s", 30)
        ck_b = mgr.create_checkpoint("task-bs", 30)
        for i in range(5):
            mgr.update_progress(i, True, checkpoint=ck_a)
        # B 写满一个 auto_save_interval（10）⇒ 它自己那份增量已经在盘上，
        # 收尾收 A 时不许把它顺手删掉（B 还没并进主文件，删了就是丢进度）
        for i in range(10):
            mgr.update_progress(i, True, checkpoint=ck_b)
        mgr.save_checkpoint(ck_a)

        with open(mgr._get_checkpoint_path(ck_a.task_id), encoding="utf-8") as f:
            on_disk = json.load(f)
        assert on_disk["processed_items"] == 5
        assert not os.path.exists(mgr._get_delta_path(ck_a.task_id))
        assert os.path.exists(mgr._get_delta_path(ck_b.task_id)), (
            "收尾把 B 的增量也一并删了（B 还没写主文件，那是丢进度）"
        )


class TestErrorsAreRunLocal:
    """②-b 失败明细的归属：本批清单优先，实例属性只留作直接调用方的默认出口"""

    def _boom_pipeline(self, monkeypatch):
        p = AugmentorPipeline(config=AppConfig())
        monkeypatch.setattr(
            AugmentorPipeline, "_generate_variants",
            lambda self, item: (_ for _ in ()).throw(RuntimeError("boom")))
        return p

    def test_direct_caller_still_uses_the_instance_sink(self, monkeypatch, tmp_path):
        """不传 `errors=` 的旧写法照旧落实例属性（兼容档，不是残留缺陷）"""
        monkeypatch.chdir(tmp_path)
        p = self._boom_pipeline(monkeypatch)
        q = Queue()
        p._process_single_item(0, {"instruction": "x"}, False, q)
        assert len(p._process_errors) == 1
        assert q.get()[1] is False

    def test_run_local_list_wins_when_given(self, monkeypatch, tmp_path):
        monkeypatch.chdir(tmp_path)
        p = self._boom_pipeline(monkeypatch)
        q = Queue()
        mine = []
        p._process_single_item(0, {"instruction": "x"}, False, q, mine)
        assert len(mine) == 1, "本批清单没接到 ⇒ 并行支路的明细又写回共享态"
        assert p._process_errors == [], "共享实例属性被一次运行写过了"


class TestDiskFormatUnchanged:
    """③ 挂实例属性而不是 dataclass 字段：盘上格式一字未改"""

    def test_asdict_key_set_does_not_grow(self, tmp_path):
        _mgr, ck = _mk(tmp_path, "f", 3)
        _state_of(ck).pending_completed.add(1)
        fields = set(dataclasses.asdict(ck))
        assert fields == {
            "task_id", "total_items", "processed_items", "failed_items",
            "completed_indices", "failed_indices",
            "start_time", "last_update_time", "quality_scores",
        }
        assert "_run_state" not in fields

    def test_pre_existing_checkpoint_file_still_loads(self, tmp_path):
        """旧文件（不含任何运行态字段）读回来进度与续传集合都对"""
        mgr = CheckpointManager(str(tmp_path / "old"), auto_save_interval=10)
        path = mgr._get_checkpoint_path("legacy")
        os.makedirs(os.path.dirname(path), exist_ok=True)
        with open(path, "w", encoding="utf-8") as f:
            json.dump({
                "task_id": "legacy", "total_items": 5, "processed_items": 2,
                "failed_items": 0, "completed_indices": [0, 1], "failed_indices": [],
                "start_time": 1.0, "last_update_time": 1.0,
                "quality_scores": {0: 0.7},
            }, f)
        ck = mgr.load_checkpoint("legacy")
        assert ck is not None
        assert mgr.get_remaining_indices(ck) == [2, 3, 4]
        assert mgr.get_progress(ck)["avg_quality_score"] == 0.7
