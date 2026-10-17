# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""L276 守卫：并行增强的消费侧按「这一笔」配对（A365 修法①，用户拍板「条目失败继续」）

立账那一轮（L275 ⑰）量到的形状是三格同族：

1. `result_queue.get()` 不带超时 ⇒ 任一 worker 少投一次，整批零 CPU 永久挂死
   （门禁实测两次停在 62%，CPU 两次采样逐字相同）；
2. `future` 的返回值一次也不读 ⇒ worker 里逃出 `_process_single_item` 的异常无声吞掉；
3. `futures[future] = idx` 建了从不读 ⇒ 配对纯靠顺序，worker 多投一条时
   进度与变体记到别的条目名下。

修法：每条目一只信箱（`mailboxes[idx]`），消费侧从 `futures[future]` 拿到「这一笔」
的编号去读那一只，读不到（有界等待超时）就记**该条目失败并继续**，同时读
`future.exception()` 让逃出的异常出声。

本文件三面：源码形状面（AST 名册，钉住「按笔配对」而不是「按计数配对」）、
行为面（少投 / 逃出异常 / 多投 / 逐只归属 / 有界等待）、尺子自证面（把改前那段
消费侧代码喂给同一只分析器，证明它会拒）。零 parametrize。
"""

import ast
import inspect
import json
import logging
import textwrap
from pathlib import Path
from queue import Queue

import pytest

from augmentor import AugmentorPipeline
from augmentor.config import load_config, ModelConfig
from augmentor.checkpoint import CheckpointManager
from augmentor.models.base import ModelBackend
from augmentor.versioning import VersionManager

import augmentor.pipeline as pipeline_mod

AI_DIR = Path(__file__).resolve().parent.parent.parent


class _FakeBackend(ModelBackend):
    """每输入一条就产 `variants` 只变体的替身后端（不碰网络）"""

    # `type` 自 A115 起是封闭清单（构造期就拒），替身因此要填一个真类型
    def __init__(self, variants=1):
        super().__init__(ModelConfig(type="ollama"))
        self._variants = variants

    def _call_api(self, prompt):
        return json.dumps(
            [{"instruction": f"变体 {i}", "output": "答"} for i in range(self._variants)],
            ensure_ascii=False,
        )


def _pipeline(tmp_path, variants=1, threads=2):
    config = load_config(str(AI_DIR / "config.yaml"))
    config.versioning.storage_dir = str(tmp_path / "versions")
    config.augmentation.num_threads = threads
    p = AugmentorPipeline(config)
    p.version_manager = VersionManager(storage_dir=str(tmp_path / "versions"))
    p.checkpoint_manager = CheckpointManager(checkpoint_dir=str(tmp_path / "checkpoints"))
    backend = _FakeBackend(variants)
    p.model_backend = backend
    p.context_augmentor.model_backend = backend
    return p


def _seed(tmp_path, n):
    path = tmp_path / "seed.json"
    path.write_text(
        json.dumps([{"instruction": f"问 {i}", "output": f"答 {i}"} for i in range(n)],
                   ensure_ascii=False),
        encoding="utf-8",
    )
    return str(path)


def _augment_body():
    """取 `augment_dataset` 的函数体 AST（源码现读，不靠记忆里的行号）"""
    src = textwrap.dedent(inspect.getsource(AugmentorPipeline.augment_dataset))
    return ast.parse(src)


def _consumer_loop(tree):
    """找出 `for future in as_completed(futures)` 那一只循环节点"""
    for node in ast.walk(tree):
        if isinstance(node, ast.For) and _calls_as_completed(node.iter):
            return node
    return None


def _calls_as_completed(node):
    return (isinstance(node, ast.Call)
            and isinstance(node.func, ast.Name)
            and node.func.id == "as_completed")


def _attr_calls(node, attr):
    """循环体内所有 `.attr(...)` 调用节点"""
    out = []
    for sub in ast.walk(node):
        if (isinstance(sub, ast.Call) and isinstance(sub.func, ast.Attribute)
                and sub.func.attr == attr):
            out.append(sub)
    return out


def _subscripts_of(node, name):
    """循环体内所有 `name[...]` 下标读取"""
    return [s for s in ast.walk(node)
            if isinstance(s, ast.Subscript)
            and isinstance(s.value, ast.Name) and s.value.id == name]


def _keyword(call, arg):
    for k in call.keywords:
        if k.arg == arg:
            return k.value
    return None


class TestTheShapeOfTheConsumer:
    """源码形状面：钉「按笔配对」这一族，不钉某一行号"""

    def test_the_loop_takes_the_index_from_the_completed_future(self):
        loop = _consumer_loop(_augment_body())
        assert loop is not None, "并行消费侧的 `for future in as_completed(futures)` 不见了"
        hits = [s for s in _subscripts_of(loop, "futures")
                if isinstance(s.slice, ast.Name) and s.slice.id == "future"]
        assert len(hits) == 1, f"`futures[future]` 应恰好读一次，实测 {len(hits)}"

    def test_the_wait_for_this_items_result_is_bounded(self):
        loop = _consumer_loop(_augment_body())
        gets = [g for g in _attr_calls(loop, "get")
                if isinstance(g.func.value, ast.Subscript)
                and isinstance(g.func.value.value, ast.Name)
                and g.func.value.value.id == "mailboxes"]
        assert len(gets) == 1, f"这一笔的信箱读取应恰好一次，实测 {len(gets)}"
        timeout = _keyword(gets[0], "timeout")
        assert isinstance(timeout, ast.Name) and timeout.id == "RESULT_WAIT_SECONDS", \
            "信箱读取的超时必须是那个具名常数，不是裸数字也不是省略"

    def test_no_untimed_shared_queue_read_survives(self):
        """改前那一形（`result_queue.get()` 不带 timeout）不许回来"""
        tree = _augment_body()
        offenders = []
        for node in ast.walk(tree):
            if isinstance(node, ast.Call) and isinstance(node.func, ast.Attribute) \
                    and node.func.attr == "get":
                if isinstance(node.func.value, ast.Name) \
                        and node.func.value.id == "result_queue":
                    offenders.append(node.lineno)
        assert offenders == [], f"又出现不带信箱的 `result_queue.get()`：行 {offenders}"

    def test_the_futures_exception_is_read_so_nothing_is_swallowed(self):
        loop = _consumer_loop(_augment_body())
        exc = [c for c in _attr_calls(loop, "exception")
               if isinstance(c.func.value, ast.Name) and c.func.value.id == "future"]
        assert len(exc) == 1, f"`future.exception()` 应读一次，实测 {len(exc)}"
        assert not any(_keyword(c, "timeout") is None and c.func.attr == "result"
                       for c in _attr_calls(loop, "result")), "不许用阻塞的 future.result()"

    def test_one_mailbox_per_pending_item(self):
        tree = _augment_body()
        comp = [n for n in ast.walk(tree)
                if isinstance(n, ast.DictComp) and isinstance(n.value, ast.Call)
                and isinstance(n.value.func, ast.Name) and n.value.func.id == "Queue"]
        assert len(comp) == 1, "信箱必须按条目各建一只"
        gen = comp[0].generators[0]
        assert isinstance(gen.target, ast.Name) and gen.target.id == "idx"
        assert isinstance(gen.iter, ast.Name) and gen.iter.id == "remaining_indices"

    def test_worker_is_handed_its_own_mailbox_not_the_shared_one(self):
        tree = _augment_body()
        subs = [c for c in _attr_calls(tree, "submit")]
        assert len(subs) == 1
        args = subs[0].args
        passed = [a for a in args
                  if isinstance(a, ast.Subscript) and isinstance(a.value, ast.Name)
                  and a.value.id == "mailboxes"]
        assert len(passed) == 1, "提交给 worker 的必须是 `mailboxes[idx]` 这一只"

    def test_the_wait_constant_is_positive_and_finite(self):
        value = pipeline_mod.RESULT_WAIT_SECONDS
        assert isinstance(value, (int, float)) and 0 < value < 3600

    def test_the_worker_still_delivers_exactly_once_on_both_paths(self):
        """信箱这一侧的前提：worker 的两条支路各投且只投一次。多投一条现在不会
        记到别人名下（本文件 `test_a_double_delivery...`），少投一条不会挂死，
        但产品语义仍要求它「投」——这一格钉住那个契约。"""
        src = textwrap.dedent(inspect.getsource(AugmentorPipeline._process_single_item))
        tree = ast.parse(src)
        puts = [n for n in ast.walk(tree)
                if isinstance(n, ast.Call) and isinstance(n.func, ast.Attribute)
                and n.func.attr == "put"
                and isinstance(n.func.value, ast.Name) and n.func.value.id == "result_queue"]
        assert len(puts) == 2, f"成功支与失败支各投一次，实测 {len(puts)}"


class TestBehaviourOfTheConsumer:
    """行为面：三格缺陷各自的可观察归宿"""

    def test_a_missing_put_becomes_an_item_failure_not_a_hang(self, tmp_path, monkeypatch):
        monkeypatch.setattr(pipeline_mod, "RESULT_WAIT_SECONDS", 0.3)
        p = _pipeline(tmp_path, variants=2)
        seed = _seed(tmp_path, 4)
        out = tmp_path / "out.json"

        real = AugmentorPipeline._process_single_item

        def dropping(idx, item, use_quality_check, q, errors=None):
            if idx == 1:
                return                      # 少投这一笔：改前整批永挂
            real(p, idx, item, use_quality_check, q, errors)

        monkeypatch.setattr(p, "_process_single_item", dropping)
        report = p.augment_dataset(seed, str(out), use_checkpoint=False,
                                   use_quality_check=False, use_dedup=False,
                                   use_parallel=True)
        bad = [e for e in report["errors"] if e["error_type"] == "ResultNotDelivered"]
        assert [e["index"] for e in bad] == [1], report["errors"]
        assert bad[0]["item_preview"], "失败明细要带上条目预览，和 worker 自己那格同形"
        assert report["output_count"] == 6, "其余 3 条 × 2 只变体不许被牵连"

    def test_an_escaped_worker_exception_is_loud(self, tmp_path, monkeypatch):
        monkeypatch.setattr(pipeline_mod, "RESULT_WAIT_SECONDS", 0.3)
        p = _pipeline(tmp_path, variants=1)
        seed = _seed(tmp_path, 3)
        out = tmp_path / "out.json"

        def exploding(idx, item, use_quality_check, q, errors=None):
            if idx == 0:
                raise RuntimeError("worker 逃出的异常")
            q.put((idx, True, [{"instruction": "ok", "output": "答"}]))

        monkeypatch.setattr(p, "_process_single_item", exploding)
        records = []

        class _Sink(logging.Handler):
            def emit(self, record):
                records.append(record)

        logger = logging.getLogger("augmentor.pipeline")
        handler = _Sink()
        logger.addHandler(handler)
        try:
            report = p.augment_dataset(seed, str(out), use_checkpoint=False,
                                       use_quality_check=False, use_dedup=False,
                                       use_parallel=True)
        finally:
            logger.removeHandler(handler)
        assert any("逃出异常" in r.getMessage() for r in records), \
            "逃出的异常必须出声：改前 `future.result()` 一次也不读，日志里什么都没有"
        assert any("worker 逃出的异常" in r.getMessage() for r in records)
        assert [e["index"] for e in report["errors"]] == [0]

    def test_a_double_delivery_cannot_be_attributed_to_another_item(self, tmp_path,
                                                                    monkeypatch):
        monkeypatch.setattr(pipeline_mod, "RESULT_WAIT_SECONDS", 0.3)
        p = _pipeline(tmp_path, variants=1)
        seed = _seed(tmp_path, 3)
        out = tmp_path / "out.json"

        real = AugmentorPipeline._process_single_item

        def greedy(idx, item, use_quality_check, q, errors=None):
            real(p, idx, item, use_quality_check, q, errors)
            if idx == 0:
                q.put((0, True, [{"instruction": "多投的一条", "output": "答"}]))

        monkeypatch.setattr(p, "_process_single_item", greedy)
        report = p.augment_dataset(seed, str(out), use_checkpoint=False,
                                   use_quality_check=False, use_dedup=False,
                                   use_parallel=True)
        results = json.loads(Path(out).read_text(encoding="utf-8"))
        assert report["output_count"] == 3, "消费次数按条目数封顶，多投的那条不进账"
        assert sum(1 for r in results if r.get("instruction") == "多投的一条") == 0

    def test_each_items_progress_lands_on_its_own_index(self, tmp_path, monkeypatch):
        """逐只归属：成功条目记自己的下标，失败条目也记自己的下标（改前配对靠顺序，
        少投/多投都会把进度记到别的条目名下）"""
        p = _pipeline(tmp_path, variants=1)
        seed = _seed(tmp_path, 5)
        out = tmp_path / "out.json"

        real = AugmentorPipeline._process_single_item

        def flaky(idx, item, use_quality_check, q, errors=None):
            if idx == 2:
                q.put((idx, False, []))
                return
            real(p, idx, item, use_quality_check, q, errors)

        monkeypatch.setattr(p, "_process_single_item", flaky)
        report = p.augment_dataset(seed, str(out), use_checkpoint=True,
                                   use_quality_check=False, use_dedup=False,
                                   use_parallel=True)
        ck = p.checkpoint_manager.load_checkpoint(report["task_id"])
        assert sorted(ck.completed_indices) == [0, 1, 3, 4], ck.completed_indices
        assert ck.failed_indices == [2], ck.failed_indices

    def test_the_wait_is_bounded_even_when_nothing_is_delivered(self, tmp_path,
                                                                 monkeypatch):
        """有界等待这一格的行为上界：全部条目都不投，整批仍必须走完而不是永挂"""
        monkeypatch.setattr(pipeline_mod, "RESULT_WAIT_SECONDS", 0.05)
        p = _pipeline(tmp_path, variants=1)
        seed = _seed(tmp_path, 3)
        out = tmp_path / "out.json"
        monkeypatch.setattr(p, "_process_single_item",
                            lambda idx, item, use_quality_check, q, errors=None: None)
        report = p.augment_dataset(seed, str(out), use_checkpoint=False,
                                   use_quality_check=False, use_dedup=False,
                                   use_parallel=True)
        assert report["output_count"] == 0
        assert len(report["errors"]) == 3

    def test_the_error_entry_keeps_the_workers_four_keys(self, tmp_path, monkeypatch):
        """失败明细的形状不许自创一格：与 worker 自己的 error_info 同四键，
        否则报告消费面（前端与 `retry_failed`）读不到"""
        monkeypatch.setattr(pipeline_mod, "RESULT_WAIT_SECONDS", 0.3)
        p = _pipeline(tmp_path, variants=1)
        seed = _seed(tmp_path, 2)
        out = tmp_path / "out.json"
        monkeypatch.setattr(p, "_process_single_item",
                            lambda idx, item, use_quality_check, q, errors=None: None)
        report = p.augment_dataset(seed, str(out), use_checkpoint=False,
                                   use_quality_check=False, use_dedup=False,
                                   use_parallel=True)
        assert {e["error_type"] for e in report["errors"]} == {"ResultNotDelivered"}
        for e in report["errors"]:
            assert set(e) == {"index", "error_type", "message", "item_preview"}

    def test_the_happy_path_is_unchanged(self, tmp_path):
        """健康跑次零漂移：这是「修法①只在坏档上出声」的判据——
        全部 worker 正常投递时，产物条数、错误清单与串行路径完全一致"""
        p = _pipeline(tmp_path, variants=3)
        seed = _seed(tmp_path, 4)
        par_out = tmp_path / "par.json"
        ser_out = tmp_path / "ser.json"
        par = p.augment_dataset(seed, str(par_out), use_checkpoint=False,
                                use_quality_check=False, use_dedup=False,
                                use_parallel=True)
        ser = p.augment_dataset(seed, str(ser_out), use_checkpoint=False,
                                use_quality_check=False, use_dedup=False,
                                use_parallel=False)
        assert par["output_count"] == ser["output_count"] > 0
        assert par["errors"] == ser["errors"] == []
        # 顺序在两跑之间本来就不保证（并行按完成先后入账），比的是同一批内容
        assert sorted(json.loads(par_out.read_text(encoding="utf-8")),
                      key=lambda r: json.dumps(r, sort_keys=True)) == \
            sorted(json.loads(ser_out.read_text(encoding="utf-8")),
                   key=lambda r: json.dumps(r, sort_keys=True))


class TestTheRulerHasTeeth:
    """尺子自证面：把改前那段消费侧代码喂给同一只分析器，它必须拒——
    否则「形状名册」等于恒真"""

    PRE_FIX = textwrap.dedent("""
        def augment_dataset(self):
            remaining_indices = [0, 1]
            result_queue = Queue()
            futures = {}
            for future in as_completed(futures):
                idx, success, variants = result_queue.get()
                if success:
                    self.all_results.extend(variants)
    """)

    def test_the_pre_fix_consumer_is_rejected_by_the_same_analyser(self):
        tree = ast.parse(self.PRE_FIX)
        loop = _consumer_loop(tree)
        assert loop is not None, "分析器连改前那一形都找不到了"
        assert _subscripts_of(loop, "futures") == [], "改前没读 `futures[future]`"
        assert _attr_calls(loop, "exception") == [], "改前没读 `future.exception()`"
        gets = _attr_calls(loop, "get")
        assert len(gets) == 1 and _keyword(gets[0], "timeout") is None, \
            "改前那一笔不带超时"

    def test_the_mailbox_ruler_finds_nothing_in_the_pre_fix_shape(self):
        tree = ast.parse(self.PRE_FIX)
        assert [n for n in ast.walk(tree) if isinstance(n, ast.DictComp)] == []
        assert _subscripts_of(_consumer_loop(tree), "mailboxes") == []

    def test_the_current_shape_passes_every_teeth_check_it_rejects(self):
        """同一只分析器在现形上给出的是「恰好各一处」，在改前形上给出 0 处或裸 get
        ⇒ 两形的差就是这把尺子的牙齿"""
        tree = _augment_body()
        loop = _consumer_loop(tree)
        assert len(_subscripts_of(loop, "futures")) == 1
        assert len(_attr_calls(loop, "exception")) == 1
        mailbox_gets = [g for g in _attr_calls(loop, "get")
                        if isinstance(g.func.value, ast.Subscript)]
        assert len(mailbox_gets) == 1
        assert _keyword(mailbox_gets[0], "timeout") is not None

    def test_the_pre_fix_shape_reproduces_the_hang_on_the_same_workload(self):
        """行为面的反证臂：把改前那段消费侧写成一个副本，喂**同一份**「少投一笔」的
        工作负载 ⇒ 它停在零 CPU 的等待里；本轮修法在同一负载下走完
        （见 `test_a_missing_put_becomes_an_item_failure_not_a_hang`）。
        这一臂不测产品代码，测的是「上面那批行为断言不是恒真」。"""
        from concurrent.futures import ThreadPoolExecutor, as_completed
        import threading

        shared = Queue()
        n_items = 4

        def worker(idx):
            if idx != 1:
                shared.put((idx, True, [{"instruction": f"变体-{idx}"}]))
            return idx

        with ThreadPoolExecutor(max_workers=2) as pool:
            futures = {pool.submit(worker, i): i for i in range(n_items)}
            done = threading.Event()

            def consume():
                for future in as_completed(futures):
                    shared.get()          # 改前那一形：不带超时、不看 future
                done.set()

            t = threading.Thread(target=consume, daemon=True)
            t.start()
            t.join(timeout=1.0)
            assert t.is_alive(), "改前那一形竟然走完了 ⇒ 反证臂失效"
            assert not done.is_set()
            shared.put((1, True, []))     # 补齐那一笔，放线程出来，不留残留
            t.join(timeout=5.0)
            assert done.is_set() and not t.is_alive()
