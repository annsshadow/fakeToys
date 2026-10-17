# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

r"""L279 / A364 前两格：共享缓存的「判据 → 刷新 → 读取」必须罩在同一把锁里

**缺陷形（改前实测读数，探针与 JSON 在仓外 Temp/l277_probes）**
`QualityScorer._calculate_diversity` 把参照集分次写：向量支写 embeddings、
行范数、参照文本三处，n-gram 支写两处，`reset_cache` 与 `batch_score.cached_encode`
的淘汰-插入裸在锁外。两只后台增强线程交错时：

- n-gram 支**静默污染**：A 的候选配上 B 的参照集，实测 `0.5833…` 变 `1.0`
  ——多样性虚高，本该被过滤的重复样本被放行，没有任何异常出声。
- 向量支**当场崩**：`ValueError: operands could not be broadcast together
  with shapes (3,) (2,)`（臂1，写完矩阵停在范数之前）与 `(2,) (3,)`
  （臂2，读完矩阵停在范数之前）。臂1 还会把缓存**永久毒化**——竞态后
  `_existing_texts` 是 2 条而矩阵是 3 行、范数是 2 个，判据与内容不再同源，
  此后每次调用都崩。
- `cached_encode` 的 FIFO 淘汰：两只线程各自看过 `len()>=2`、各自取到同一枚
  最老键，后落盘那一只 `pop` 当场 `KeyError`。

`ContextAugmentor.convert_single_to_multi_turn` 是「存-改-复原」形状：并发时
**双方都错**（要 2 轮的拿到 5、要 5 轮的拿到 3），实例默认值还会被永久留在
最后一次复原的那一格（3 → 2）。

**三面**
① 形状名册（AST）：`quality.py` 里参照属性（五份）在 `__init__` 之外的读写点
   必须全部落在 `with self._cache_lock:` 块内，锁区恰好 4 处；`_embedding_cache`
   锁外只许有 `cached_encode` 里那两枚 dict 原子读；`context.py` 构造器之外对
   `self.num_turns` 的赋值恰好 0 处。
② 行为对账：A 停在临界区内、B 随后起跑。真锁下 B 必须进不去（这就是互斥的直接
   证据），两臂各得自己真值、零异常、竞态后三格缓存同源。
③ 反证与尺子自证：把 `_cache_lock` 换成 no-op 假锁，必须重现 1.0 污染、两形
   `ValueError`、毒态缓存与 `pop` 的 `KeyError`；把 HEAD 版裸写形状的合成语料
   喂给形状判据，必须点名红——判据同时验「换一把别的锁」不算罩住。
"""

import ast
import os
import threading

import numpy as np

from augmentor.context import ContextAugmentor
from augmentor.quality import QualityScorer, _char_ngrams
import augmentor.quality as quality_module

REPO = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
QUALITY_PY = os.path.join(REPO, "augmentor", "quality.py")
CONTEXT_PY = os.path.join(REPO, "augmentor", "context.py")

REF_ATTRS = {"_existing_embeddings", "_existing_norms", "_existing_texts",
             "_existing_ngrams", "_existing_ngram_texts"}
LOCK_ATTR = "_cache_lock"
EVICT_ATTR = "_embedding_cache"

# n-gram 支的两组参照/候选：A 的候选与 B 的参照集零重叠，交叉值恰好 1.0
REF_NG_A = ["人工智能的定义是什么", "机器学习有哪些分类", "深度学习怎么用神经网络"]
REF_NG_B = ["今天的天气怎么样", "推荐几本科幻小说", "如何做番茄炒蛋"]
CAND_NG_A = "人工智能是什么东西"
CAND_NG_B = "明天会下雨吗"
TRUTH_NG_A = 0.5833333333333333
TRUTH_NG_B = 1.0
CROSS_NG_A_VS_B = 1.0

# 向量支：A 的参照切片 2 条、B 的 3 条，行数不等才量得出广播形状差
REF_V_A = ["aaaaaaaaaa", "bbbbbbbbbb"]
REF_V_B = ["cccccccccc", "dddddddddd", "eeeeeeeeee"]
CAND_V_A = "aaaaaaaaaa"
CAND_V_B = "cccccccccc"
TRUTH_V = 2.220446049250313e-16

# 名册格数：`_calculate_diversity`（n-gram 支 4 + 向量支 8）与 `reset_cache`（5）
# 里被锁罩住的参照属性读写点，按现量钉死；删码或漏罩都会顶到这一格。
REF_SITE_ROSTER_SIZE = 17


class NoOpLock:
    """假锁：只提供 with 协议，不提供互斥。反证臂用它顶掉真锁。"""

    def __enter__(self):
        return self

    def __exit__(self, *exc):
        return False


class FakeModel:
    """本环境没装 sentence_transformers，向量支靠替身走到（生产装齐依赖时同一条路）"""

    def encode(self, texts, **kwargs):
        return np.array([[1.0, float(len(t) % 3)] for t in texts], dtype=np.float64)


def _ngram_scorer(lock=None):
    scorer = QualityScorer(diversity_sample_size=3)
    scorer._model = "fallback"
    if lock is not None:
        scorer._cache_lock = lock
    return scorer


def _vector_scorer(lock=None):
    scorer = QualityScorer(diversity_sample_size=3)
    scorer._model = FakeModel()
    if lock is not None:
        scorer._cache_lock = lock
    return scorer


# --------------------------------------------------------------------------- ① 形状


def _sites(source, watched):
    """返回 (读写点名单, `with self._cache_lock` 锁区数)。

    每格是 (所属函数名, 属性名, Store/Load, 行号, 是否在锁区内)。函数名取最内层
    的 def，`cached_encode` 这类嵌套闭包因此能单独点名。
    """
    tree = ast.parse(source)
    covered = set()
    regions = 0
    for node in ast.walk(tree):
        if not isinstance(node, ast.With):
            continue
        if any(isinstance(it.context_expr, ast.Attribute)
               and it.context_expr.attr == LOCK_ATTR
               and isinstance(it.context_expr.value, ast.Name)
               and it.context_expr.value.id == "self"
               for it in node.items):
            regions += 1
            for sub in ast.walk(node):
                covered.add(id(sub))

    parents = {}
    for node in ast.walk(tree):
        for child in ast.iter_child_nodes(node):
            parents[id(child)] = node

    def innermost_fn(node):
        cur = parents.get(id(node))
        while cur is not None:
            if isinstance(cur, ast.FunctionDef):
                return cur.name
            cur = parents.get(id(cur))
        return "<module>"

    found = []
    for node in ast.walk(tree):
        if (isinstance(node, ast.Attribute) and isinstance(node.value, ast.Name)
                and node.value.id == "self" and node.attr in watched):
            found.append((innermost_fn(node), node.attr, type(node.ctx).__name__,
                          node.lineno, id(node) in covered))
    return found, regions


def _unprotected_ref_sites(source):
    sites, _ = _sites(source, REF_ATTRS)
    return [s for s in sites if s[0] != "__init__" and not s[4]]


def _num_turns_writes_outside_init(source):
    sites, _ = _sites(source, {"num_turns"})
    return [s for s in sites if s[2] == "Store" and s[0] != "__init__"]


def _read(path):
    with open(path, encoding="utf-8") as fh:
        return fh.read()


# HEAD 版（改前）的形状，压缩成判据要认的最小语料：分次写、没有任何锁
OLD_QUALITY_SHAPE = '''
class QualityScorer:
    def __init__(self):
        self._existing_texts = []

    def _calculate_diversity(self, text, existing_texts):
        sample_texts = existing_texts[:3]
        if sample_texts != self._existing_texts:
            self._existing_embeddings = encode(sample_texts)
            self._existing_norms = norm(self._existing_embeddings)
            self._existing_texts = sample_texts
        if len(self._existing_embeddings) == 0:
            return 1.0
        return float(np.max(np.dot(self._existing_embeddings, v) / self._existing_norms))

    def reset_cache(self):
        self._existing_embeddings, self._existing_norms = [], []
        self._existing_texts = []
'''

# 换一把别的锁不算罩住：判据认的是 `self._cache_lock` 这一把
OTHER_LOCK_SHAPE = '''
class QualityScorer:
    def _calculate_diversity(self, sample):
        with self._other_lock:
            self._existing_embeddings = encode(sample)
        return self._existing_norms
'''

OLD_CONTEXT_SHAPE = '''
class ContextAugmentor:
    def __init__(self):
        self.num_turns = 3

    def convert_single_to_multi_turn(self, items, num_turns=None):
        original = self.num_turns
        if num_turns is not None:
            self.num_turns = num_turns
        try:
            return self.batch_generate(items)
        finally:
            self.num_turns = original
'''


class TestLockRegionRoster:
    """① 形状名册：该罩住的点全在锁内，锁外只剩两枚 dict 原子读"""

    def test_reference_cache_sites_are_all_under_the_lock(self):
        bad = _unprotected_ref_sites(_read(QUALITY_PY))
        assert bad == [], bad

    def test_lock_region_roster_is_exactly_four(self):
        _, regions = _sites(_read(QUALITY_PY), REF_ATTRS | {EVICT_ATTR})
        assert regions == 4, regions

    def test_evict_cache_read_outside_the_lock_is_exactly_the_two_atomic_probes(self):
        """锁外只许 `if key in cache` / `return cache[key]` 两枚——dict 单步读在 GIL 下原子"""
        sites, _ = _sites(_read(QUALITY_PY), {EVICT_ATTR})
        outside = [s for s in sites if not s[4] and s[0] != "__init__"]
        assert len(outside) == 2, outside
        assert {s[0] for s in outside} == {"cached_encode"}, outside
        assert sorted(s[2] for s in outside) == ["Load", "Load"], outside
        inside = [s for s in sites if s[4]]
        assert {s[0] for s in inside} == {"cached_encode"}, inside
        assert len(inside) == 4, inside
        init_stores = [s for s in sites if s[0] == "__init__"]
        assert [(s[2], s[4]) for s in init_stores] == [("Store", False)], init_stores

    def test_eviction_and_insertion_share_one_critical_region(self):
        """`while` 淘汰与 `[key] = embeddings` 插入必须在同一个 `with` 体里——分两处等于没锁"""
        tree = ast.parse(_read(QUALITY_PY))
        shapes = []
        for node in ast.walk(tree):
            if isinstance(node, ast.With) and any(
                    isinstance(it.context_expr, ast.Attribute)
                    and it.context_expr.attr == LOCK_ATTR for it in node.items):
                shapes.append(sorted(type(st).__name__ for st in node.body))
        assert ["Assign", "While"] in shapes, shapes

    def test_no_assignment_to_instance_num_turns_outside_the_constructor(self):
        bad = _num_turns_writes_outside_init(_read(CONTEXT_PY))
        assert bad == [], bad

    def test_turns_travel_down_the_call_chain_as_a_parameter(self):
        tree = ast.parse(_read(CONTEXT_PY))
        args = {f.name: [a.arg for a in f.args.args]
                for f in ast.walk(tree) if isinstance(f, ast.FunctionDef)}
        for fn in ("generate_multi_turn", "batch_generate",
                   "convert_single_to_multi_turn"):
            assert "num_turns" in args[fn], (fn, args[fn])

    def test_ruler_bites_on_the_pre_fix_shape(self):
        """尺子自证：HEAD 版裸写形状必须被点名，且计数与形状一一对应"""
        bad = _unprotected_ref_sites(OLD_QUALITY_SHAPE)
        assert len(bad) == 11, bad
        assert sorted({s[0] for s in bad}) == ["_calculate_diversity", "reset_cache"]
        assert len(_num_turns_writes_outside_init(OLD_CONTEXT_SHAPE)) == 2
        # 只锁一把不相干的锁，同样判红
        assert len(_unprotected_ref_sites(OTHER_LOCK_SHAPE)) == 2

    def test_roster_actually_sees_the_sites_it_claims_to_cover(self):
        """反恒真：判据不是「什么都匹配不到」才报 0——名册格数按现量钉死"""
        sites, _ = _sites(_read(QUALITY_PY), REF_ATTRS)
        outside = [s for s in sites if not s[4]]
        assert len(sites) - len(outside) == REF_SITE_ROSTER_SIZE, sites
        assert {s[0] for s in outside} == {"__init__"}, outside


# --------------------------------------------------------------------------- ② 行为


def _run_parked_race(a_target, b_target, a_parked, release, timeout=1.5):
    """A 停在临界区内、B 随后起跑，返回 (两臂的读数, B 是否在 A 停住期间跑完)

    调度钩子必须双向对齐：只挂一只线程的话，并发会静默退化成顺序，
    仪器恒绿（L275 同一类缺陷在本轮第二次实测到）。
    """
    got = {}
    b_done = threading.Event()

    def thread_a():
        threading.current_thread().name = "A"
        try:
            got["A"] = a_target()
        except Exception as exc:  # noqa: BLE001
            got["A_error"] = f"{type(exc).__name__}: {exc}"

    def thread_b():
        threading.current_thread().name = "B"
        try:
            got["B"] = b_target()
        except Exception as exc:  # noqa: BLE001
            got["B_error"] = f"{type(exc).__name__}: {exc}"
        b_done.set()

    ta = threading.Thread(target=thread_a)
    ta.start()
    assert a_parked.wait(timeout), "A 没有停在预期的窗口上：调度钩子失效"
    tb = threading.Thread(target=thread_b)
    tb.start()
    tb.join(timeout)
    b_ran_while_a_parked = b_done.is_set()
    release.set()
    ta.join(20)
    tb.join(20)
    return got, b_ran_while_a_parked


def _ngram_drive(lock):
    """n-gram 支：A 已把 REF_A 刷进两份属性、停在读参照集之前，B 随后整次跑"""
    scorer = _ngram_scorer(lock)
    a_parked, release = threading.Event(), threading.Event()
    orig = quality_module._char_ngrams

    def hook(text, n=quality_module._NGRAM_SIZE):
        if threading.current_thread().name == "A" and text == CAND_NG_A:
            a_parked.set()
            release.wait(20)
        return orig(text, n)

    quality_module._char_ngrams = hook
    try:
        return _run_parked_race(
            lambda: scorer._calculate_diversity(CAND_NG_A, REF_NG_A),
            lambda: scorer._calculate_diversity(CAND_NG_B, REF_NG_B),
            a_parked, release), scorer
    finally:
        quality_module._char_ngrams = orig


def _vector_drive(park_on, lock):
    """向量支两臂：`norm`（写完矩阵停在范数前）或 `dot`（读完范数停在除法前）"""
    scorer = _vector_scorer(lock)
    a_parked, release = threading.Event(), threading.Event()
    real = np.linalg.norm if park_on == "norm" else np.dot

    def hook(first, *args, **kwargs):
        result = real(first, *args, **kwargs)
        name = threading.current_thread().name
        if name == "A" and (park_on != "norm" or kwargs.get("axis") == 1):
            a_parked.set()
            release.wait(20)
        return result

    holder = np.linalg if park_on == "norm" else np
    original = holder.norm if park_on == "norm" else holder.dot
    setattr(holder, "norm" if park_on == "norm" else "dot", hook)
    try:
        return _run_parked_race(
            lambda: scorer._calculate_diversity(CAND_V_A, REF_V_A),
            lambda: scorer._calculate_diversity(CAND_V_B, REF_V_B),
            a_parked, release), scorer
    finally:
        setattr(holder, "norm" if park_on == "norm" else "dot", original)


def _cache_homogeneity(scorer):
    return (len(scorer._existing_texts),
            int(np.asarray(scorer._existing_embeddings).shape[0]),
            len(scorer._existing_norms))


class TestLockExcludesWhileInsideTheRegion:
    """② 行为对账：真锁下 A 停在临界区内时 B 进不去，两臂各得真值"""

    def test_ngram_arm_parks_the_other_thread_out_and_returns_its_own_truth(self):
        (got, b_ran), scorer = _ngram_drive(None)
        assert b_ran is False, "B 在 A 停住期间跑完了——临界区没挡住第二次运行"
        assert got.get("A_error") is None, got
        assert got.get("B_error") is None, got
        assert got["A"] == TRUTH_NG_A, got
        assert got["B"] == TRUTH_NG_B, got
        assert len(scorer._existing_ngrams) == len(scorer._existing_ngram_texts) == 3

    def test_vector_norm_arm_returns_truth_and_keeps_the_cache_self_consistent(self):
        (got, b_ran), scorer = _vector_drive("norm", None)
        assert b_ran is False, got
        assert got.get("A_error") is None, got
        assert got.get("B_error") is None, got
        assert got["A"] == TRUTH_V, got
        assert got["B"] == TRUTH_V, got
        rows = _cache_homogeneity(scorer)
        assert rows[0] == rows[1] == rows[2], rows

    def test_vector_dot_arm_returns_truth_and_keeps_the_cache_self_consistent(self):
        (got, b_ran), scorer = _vector_drive("dot", None)
        assert b_ran is False, got
        assert got.get("A_error") is None, got
        assert got["A"] == TRUTH_V, got
        rows = _cache_homogeneity(scorer)
        assert rows[0] == rows[1] == rows[2], rows


class TestFakeLockReproducesTheDefect:
    """③ 反证：同一套驱动换成 no-op 假锁，必须把污染与两形崩溃量回来"""

    def test_ngram_arm_under_fake_lock_returns_the_cross_reference_value(self):
        (got, b_ran), _ = _ngram_drive(NoOpLock())
        assert b_ran is True
        assert got.get("A_error") is None, got
        assert got["A"] != TRUTH_NG_A, got
        assert got["A"] == CROSS_NG_A_VS_B, got

    def test_norm_arm_under_fake_lock_crashes_and_poisons_the_cache(self):
        (got, b_ran), scorer = _vector_drive("norm", NoOpLock())
        assert b_ran is True
        assert got.get("A_error") == (
            "ValueError: operands could not be broadcast together with shapes (3,) (2,) "
        ), got
        assert _cache_homogeneity(scorer) == (2, 3, 2), _cache_homogeneity(scorer)

    def test_dot_arm_under_fake_lock_crashes_with_the_mirrored_shape_error(self):
        (got, b_ran), _ = _vector_drive("dot", NoOpLock())
        assert b_ran is True
        assert got.get("A_error") == (
            "ValueError: operands could not be broadcast together with shapes (2,) (3,) "
        ), got


class TestEvictionAndInsertionAreAtomic:
    """`cached_encode` 的 FIFO 淘汰：看过长度与取最老键、pop、插入必须在同一临界区"""

    def test_no_keyerror_when_two_threads_evict_through_the_same_oldest_key(self):
        scorer = _evict_scorer(None)
        got = _evict_race(scorer)
        assert got.get("A_error") is None, got
        assert got.get("B_error") is None, got
        assert got["b_blocked_on_lock"] is True, got
        assert len(scorer._embedding_cache) == 2, got

    def test_fake_lock_reproduces_the_double_pop_keyerror(self):
        scorer = _evict_scorer(NoOpLock())
        got = _evict_race(scorer)
        assert got["b_blocked_on_lock"] is False, got
        assert "KeyError" in (got.get("A_error") or got.get("B_error") or ""), got

    def test_reset_cache_clears_every_reference_attribute_under_the_lock(self):
        scorer = _vector_scorer(None)
        scorer._calculate_diversity(CAND_V_A, REF_V_A)
        assert _cache_homogeneity(scorer) == (2, 2, 2)
        scorer.reset_cache()
        assert _cache_homogeneity(scorer) == (0, 0, 0)
        assert scorer._existing_ngrams == [] and scorer._existing_ngram_texts == []


class _ParkOnFirstIter(dict):
    """把 `next(iter(cache))` 这一步钉成可对齐的窗口：每只线程第一次取快照时停住"""

    def __init__(self, events=None, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self._events = events or {}

    def __iter__(self):
        keys = list(dict.keys(self))
        evs = self._events.get(threading.current_thread().name)
        if evs is not None and not evs[2].is_set():
            evs[0].set()
            evs[1].wait(20)
            evs[2].set()
        return iter(keys)


def _evict_scorer(lock):
    scorer = QualityScorer(threshold=0.999)
    scorer._cross_encoder = "fallback"
    scorer._model = FakeModel()
    if lock is not None:
        scorer._cache_lock = lock
    return scorer


def _evict_batch(idx, n=6):
    return [{"original": "种子问题 %d_%d" % (idx, i),
             "generated": "生成回答 %d_%d 的内容，这一段要够长才能彼此不同" % (idx, i),
             "output": "参考答案 %d_%d" % (idx, i)} for i in range(n)]


def _evict_race(scorer):
    """先把缓存灌满两枚历史键，再让两只线程各自插入新批次——两次淘汰都要取最老键"""
    scorer.batch_score(_evict_batch(0), include_semantic=True)
    scorer.batch_score(_evict_batch(1), include_semantic=True)
    assert len(scorer._embedding_cache) == 2, list(scorer._embedding_cache)

    a_parked, a_release, a_used = threading.Event(), threading.Event(), threading.Event()
    b_parked, b_release, b_used = threading.Event(), threading.Event(), threading.Event()
    cache = _ParkOnFirstIter({"A": (a_parked, a_release, a_used),
                              "B": (b_parked, b_release, b_used)},
                             scorer._embedding_cache)
    scorer._embedding_cache = cache
    got = {}

    def runner(name, idx):
        try:
            scorer.batch_score(_evict_batch(idx), include_semantic=True)
        except Exception as exc:  # noqa: BLE001
            got[f"{name}_error"] = f"{type(exc).__name__}: {exc}"

    ta = threading.Thread(target=runner, args=("A", 2), name="A")
    ta.start()
    assert a_parked.wait(20), "A 没停在取最老键的窗口上：调度钩子失效"
    tb = threading.Thread(target=runner, args=("B", 3), name="B")
    tb.start()
    b_blocked = not b_parked.wait(1.5)
    a_release.set()
    b_release.set()
    ta.join(20)
    tb.join(20)
    got["b_blocked_on_lock"] = b_blocked
    return got


class TestTurnsTravelByArgument:
    """② + ③（context 格）：轮数沿调用链带下去，存-改-复原的形状当场判错"""

    class StoreRestoreAugmentor(ContextAugmentor):
        """HEAD 版 `convert_single_to_multi_turn` 的形状，只用于证明仪器抓得到"""

        def convert_single_to_multi_turn(self, items, num_turns=None):
            original = self.num_turns
            if num_turns is not None:
                self.num_turns = num_turns
            try:
                return self.batch_generate(items)
            finally:
                self.num_turns = original

    @staticmethod
    def _turns(result):
        return len(result["history"]) // 2 + 1 if result else 0

    def _drive(self, augmentor):
        """三段事件：A 停在「已写下轮数、还没读」，B 停在「已写、还没读」，
        A 跑完并复原之后才放行 B 读——交错两轮才量得出串轮数"""
        a_at_bg, b_at_bg = threading.Event(), threading.Event()
        b_at_read, a_release, a_done = threading.Event(), threading.Event(), threading.Event()
        got = {}
        bg = ContextAugmentor.batch_generate
        gm = ContextAugmentor.generate_multi_turn

        def bg_wrapped(self, items, *args, **kwargs):
            name = threading.current_thread().name
            if name == "A":
                a_at_bg.set()
                b_at_bg.wait(20)
            elif name == "B":
                b_at_bg.set()
            return bg(self, items, *args, **kwargs)

        def gm_wrapped(self, item, *args, **kwargs):
            name = threading.current_thread().name
            if name == "B":
                b_at_read.set()
                a_done.wait(20)
            elif name == "A":
                a_at_bg.wait(20)
                a_release.wait(20)
            return gm(self, item, *args, **kwargs)

        ContextAugmentor.batch_generate = bg_wrapped
        ContextAugmentor.generate_multi_turn = gm_wrapped
        try:
            def thread_a():
                threading.current_thread().name = "A"
                r = augmentor.convert_single_to_multi_turn(
                    [{"instruction": "问题一", "output": "回答一"}], num_turns=2)
                got["A"] = self._turns(r[0])
                a_done.set()

            def thread_b():
                threading.current_thread().name = "B"
                r = augmentor.convert_single_to_multi_turn(
                    [{"instruction": "问题二", "output": "回答二"}], num_turns=5)
                got["B"] = self._turns(r[0])

            ta = threading.Thread(target=thread_a)
            ta.start()
            assert a_at_bg.wait(20), "A 没停在写轮数之后的窗口上"
            tb = threading.Thread(target=thread_b)
            tb.start()
            assert b_at_bg.wait(20), "B 没接上 A 的事件"
            assert b_at_read.wait(20), "B 没停在读轮数之前：驱动退化成顺序了"
            a_release.set()
            ta.join(20)
            tb.join(20)
        finally:
            ContextAugmentor.batch_generate = bg
            ContextAugmentor.generate_multi_turn = gm
        return got

    def _augmentor(self, cls):
        instance = cls(model_backend=None, num_turns=3)
        instance._generate_follow_up_question = (
            lambda history, q, a: "追问%d" % len(history))
        return instance

    def test_store_restore_shape_crosses_two_callers_and_poisons_the_default(self):
        """反证臂：把轮数存进实例属性再复原，同样的驱动下双方都错"""
        instance = self._augmentor(self.StoreRestoreAugmentor)
        got = self._drive(instance)
        assert got == {"A": 5, "B": 3}, got
        assert instance.num_turns == 2, instance.num_turns

    def test_real_augmentor_gives_each_caller_its_own_turns_and_keeps_the_default(self):
        instance = self._augmentor(ContextAugmentor)
        got = self._drive(instance)
        assert got == {"A": 2, "B": 5}, got
        assert instance.num_turns == 3, instance.num_turns
