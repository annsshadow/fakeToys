# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""L266：分支维 18 格偏支逐格定性并接线（死支删除 / 链尾栅栏 / 可达档补测）

**靶子**：上一轮终态门禁读到的 `TOTAL 14384 语句 / 0 缺数 / 3968 分支 / 18 偏支`，
名册钉在卷二 L265 日志块第 ⑫ 条要点里（18 格 / 16 只文件）。本轮把它逐格判掉。

**三档处置，18 格一格不落空**：

- 甲「结构性不可达，删除或等价改写」（7 格）——`cleaner.py` 的循环内存在性判断、
  `rag.py` 的「空块」与「循环耗尽」两支、`report.py` 指标名 if/elif 链的链尾、
  `retry.py` 的第二条终止权威、`quality.py` 对恒非空建议表的兜底判断、
  `config_validator.py` 对唯一产地路径的去重。判据是「这一支在任何输入下都走不到」，
  所以留在代码里不是保险而是**假保险**：它会让人以为那条路径被测过。
- 乙「封闭词表的链尾加栅栏」（5 格）——`ops.py` / `version.py`（两条链）/
  `export.py` / `sampler.py`。L124 曾把这一族判成「记档不测」，本轮**改判**，理由写在
  `TestChainTailFences` 的类 docstring 里：词表在 parser，分支表在 commands，两个文件
  之间只有一条端点级尺子量过（L244 量的是 backup 一条），而 `run_*` 全部可以被直接调用。
- 丙「可达档补行为用例」（6 格）——`api/main.py` 的鉴权告警、`cache.py` 的淘汰走到
  最后一条、`model_manager.py` 与 `search_enhanced.py` 的双检锁、`pipeline.py` 的
  并行失败且不带断点、`visualizer.py` 的第二次加载。

**形状判据为什么和行为的判据都要写**：只写行为用例的话，「那一支被删掉」这件事在
覆盖率之外没有任何东西会红——下一轮有人把死支加回来，覆盖率数字一样难看，而本轮
收口写过的「这一格已定性」就成了空话。所以甲/乙两档各配一条 AST 形状断言（判「形状
消失了 / 栅栏在场」），丙档只配行为断言（那 6 格本来就该被行为钉住）。

**注入臂**：见 `TestRosterAndRulersAreNotVacuous`。每条形状判据都喂一份「改前源码」，要求它当场
报出命中数；词表对账判据喂一份「少接一档的分支表」。全部零落盘。

跑法：`python -m pytest tests/unit/test_partial_branch_residue_l266.py -q --no-cov`
"""

import ast
import importlib
import inspect
import json
import logging
import sys
import textwrap
import threading
from io import StringIO
from pathlib import Path
from types import ModuleType, SimpleNamespace
from contextlib import redirect_stderr, redirect_stdout

import pytest

from augmentor.cache import DiskCache
from augmentor.cleaner import DatasetCleaner
from augmentor.cli.commands.ops import run_dependency
from augmentor.cli.commands.quality import run_quality_report
from augmentor.cli.commands.version import run_backup, run_version
from augmentor.cli.parser import build_parser
from augmentor.config_validator import ConfigValidator, validate_config
from augmentor.exceptions import DataValidationError, UnsupportedFormatError
from augmentor.export import NATIVE_FORMATS, Exporter
from augmentor.export_enhanced import ExportFormat
from augmentor.model_manager import ModelManager
from augmentor.pipeline import AugmentorPipeline
from augmentor.rag import RAGFormatter
from augmentor.report import ReportGenerator
from augmentor.retry import with_retries
from augmentor.sampler import ActiveSampler
from augmentor.search_enhanced import EnhancedSearcher
from augmentor.visualizer import DataVisualizer

ROOT = Path(__file__).resolve().parents[2]

#: 本轮名册（卷二 L265 日志块 ⑫ 的 18 格）。三档必须把 18 格分完，谁也不许既不在
#: 甲也不在丙却自称收口；这个分区断言在 `test_the_roster_partitions_eighteen_sites` 里。
DEAD_REWRITES = {
    "cleaner 循环内再判存在性": ("augmentor/cleaner.py", DatasetCleaner.clean),
    "rag 空块支": ("augmentor/rag.py", RAGFormatter.chunk_text),
    "rag 循环耗尽支": ("augmentor/rag.py", RAGFormatter.chunk_text),
    "report 指标名链尾": ("augmentor/report.py", ReportGenerator._generate_suggestions),
    "retry 第二条终止权威": ("augmentor/retry.py", with_retries),
    "quality 恒非空建议表的兜底判断": ("augmentor/cli/commands/quality.py", run_quality_report),
    "config_validator 唯一产地路径的去重": ("augmentor/config_validator.py",
                                        ConfigValidator._validate_replay_sections),
}
CHAIN_FENCES = {
    "dependency 链尾": ("augmentor/cli/commands/ops.py", run_dependency, "dependency"),
    "version 链尾": ("augmentor/cli/commands/version.py", run_version, "version"),
    "backup 链尾": ("augmentor/cli/commands/version.py", run_backup, "backup"),
    "export 原生链尾": ("augmentor/export.py", Exporter.export, None),
    "sampler 类别链尾": ("augmentor/sampler.py", ActiveSampler.recommend_seeds, None),
}
COVERED_ARCS = {
    "api/main 未鉴权告警": ("api/main.py",),
    "cache 淘汰走到最后一条": ("augmentor/cache.py",),
    "model_manager 双检锁": ("augmentor/model_manager.py",),
    "search_enhanced 双检锁": ("augmentor/search_enhanced.py",),
    "pipeline 并行失败无断点": ("augmentor/pipeline.py",),
    "visualizer 第二次加载": ("augmentor/visualizer.py",),
}

#: 网格对拍的读数：`chunk_size` 1..12 与 `chunk_overlap` 0..size-1 与长度 1..40 的全组合。
GRID_CASES = 3120


# ============ 形状判据（全部只吃源码字符串，注入臂可以喂改前文本） ============

def _tree(source):
    return ast.parse(textwrap.dedent(source))


def _snip(fragment):
    """注入样本去首行空行——形状判据报的是行号，留个空行就会全体 +1，把「命中」读成「错位」"""
    return textwrap.dedent(fragment).lstrip("\n")


def _membership_guards_inside_for_loops(source):
    """`for` 的**直接**语句体里「`x in 集合`」形状的 `if`——甲档第一格的靶子

    只看循环体的顶层，不看 `if was_modified:` 这类同层但不同形的判断：判据要能点名
    「把静默跳过藏回闸后」那一形，而不是把所有循环里的 `if` 都算成命中。
    """
    hits = []
    for loop in ast.walk(_tree(source)):
        if not isinstance(loop, ast.For):
            continue
        for stmt in loop.body:
            if isinstance(stmt, ast.If) and isinstance(stmt.test, ast.Compare) \
                    and any(isinstance(op, ast.In) for op in stmt.test.ops):
                hits.append(stmt.lineno)
    return hits


def _old_style_loop_body(source):
    """`for` + `break` + 「真值判断的 if」三件一起出现 ⇒ 旧 rag 写法那一形"""
    tree = _tree(source)
    has_for = any(isinstance(n, ast.For) for n in ast.walk(tree))
    has_break = any(isinstance(n, ast.Break) for n in ast.walk(tree))
    truthy = [n.lineno for n in ast.walk(tree)
              if isinstance(n, ast.If) and isinstance(n.test, ast.Name)]
    return {"for": has_for, "break": has_break, "truthy_ifs": truthy}


def _last_attempt_break(source):
    """`if attempt >= max_retries: break` 那一形（第二条终止权威）"""
    hits = []
    for node in ast.walk(_tree(source)):
        if not isinstance(node, ast.If) or not isinstance(node.test, ast.Compare):
            continue
        if not any(isinstance(op, (ast.GtE, ast.Gt)) for op in node.test.ops):
            continue
        if isinstance(node.test.left, ast.Name) and node.test.left.id == "attempt":
            if [type(s) for s in node.body] == [ast.Break]:
                hits.append(node.lineno)
    return hits


def _metric_name_if_chain(source):
    """`if/elif metric == "…"` 链里的成员点名次数（report 甲档的靶子）"""
    hits = []
    for node in ast.walk(_tree(source)):
        if isinstance(node, ast.If) and isinstance(node.test, ast.Compare):
            cmp_ = node.test
            if isinstance(cmp_.left, ast.Name) and cmp_.left.id == "metric" \
                    and any(isinstance(op, ast.Eq) for op in cmp_.ops) \
                    and isinstance(cmp_.comparators[0], ast.Constant) \
                    and isinstance(cmp_.comparators[0].value, str):
                hits.append(cmp_.comparators[0].value)
    return hits


def _truthy_guard_on(source, dotted):
    """`if <dotted>:` 那一形的命中行号（quality 甲档的靶子）"""
    parts = dotted.split(".")
    hits = []
    for node in ast.walk(_tree(source)):
        if not isinstance(node, ast.If) or not isinstance(node.test, ast.Attribute):
            continue
        walk, names = node.test, []
        while isinstance(walk, ast.Attribute):
            names.append(walk.attr)
            walk = walk.value
        if isinstance(walk, ast.Name):
            names.append(walk.id)
        if ".".join(reversed(names)) == dotted:
            hits.append(node.lineno)
    return hits


def _dedup_by_err_paths(source):
    """`if <某名字> not in err_paths:` 那一**裸形**的命中行号（config_validator 甲档的靶子）

    批量回放里保留的两处去重写成 `if not (path in err_paths or …)`（复合判据），形不同、
    不进这把尺子——它只认改前那一格「单条 `x not in err_paths` 挡一条唯一产地的路径」。
    """
    hits = []
    for node in ast.walk(_tree(source)):
        if not isinstance(node, ast.If) or not isinstance(node.test, ast.Compare):
            continue
        cmp_ = node.test
        if any(isinstance(op, ast.NotIn) for op in cmp_.ops) \
                and any(isinstance(c, ast.Name) and c.id == "err_paths"
                        for c in cmp_.comparators):
            hits.append(node.lineno)
    return hits


def _chain_branch_actions(source):
    """链里点名动作的字符串字面量集合（`action == "list"` / `args.action == "create"`）"""
    found = set()
    for node in ast.walk(_tree(source)):
        if not isinstance(node, ast.If) or not isinstance(node.test, ast.Compare):
            continue
        cmp_ = node.test
        if not any(isinstance(op, ast.Eq) for op in cmp_.ops):
            continue
        left = cmp_.left
        tail = left.attr if isinstance(left, ast.Attribute) else getattr(left, "id", "")
        if tail != "action":
            continue
        value = cmp_.comparators[0]
        if isinstance(value, ast.Constant) and isinstance(value.value, str):
            found.add(value.value)
    return found


def _dispatch_chains(source):
    """「按字面量/枚举成员分派」的 if/elif 链：返回 `(链首行号, 判据名, 点名值, 链尾有无 else)`

    只认链里 **不少于两个成员、全链比同一个左值、右侧是字符串常量或 `Enum.MEMBER`** 的形——
    那是「词表住在别处、分支表住在这里」的签名。单臂 `if`、数值比较、`a and b` 复合判据
    都不算，否则会把 `quality.py` 那种正常分支读成分派链。
    """
    tree = _tree(source)
    elif_members = set()
    for node in ast.walk(tree):
        if isinstance(node, ast.If):
            for nxt in node.orelse:
                if isinstance(nxt, ast.If):
                    elif_members.add(id(nxt))

    def dotted(node):
        names = []
        while isinstance(node, ast.Attribute):
            names.append(node.attr)
            node = node.value
        if not isinstance(node, ast.Name):
            return None
        names.append(node.id)
        return ".".join(reversed(names))

    def arm(test):
        if not isinstance(test, ast.Compare):
            return None
        if not any(isinstance(op, ast.Eq) for op in test.ops):
            return None
        left = dotted(test.left)
        right = test.comparators[0]
        if left is None:
            return None
        if isinstance(right, ast.Constant) and isinstance(right.value, str):
            return left, right.value
        enum_tail = dotted(right)
        if enum_tail:
            return left, enum_tail
        return None

    chains = []
    for node in ast.walk(tree):
        if not isinstance(node, ast.If) or id(node) in elif_members:
            continue
        names, values, tail = set(), set(), node
        while True:
            got = arm(tail.test)
            if got:
                names.add(got[0])
                values.add(got[1])
            if not tail.orelse or not isinstance(tail.orelse[0], ast.If):
                break
            tail = tail.orelse[0]
        if len(names) == 1 and len(values) >= 2:
            chains.append((node.lineno, names.pop(), sorted(values), bool(tail.orelse)))
    return chains


def _parser_action_choices(subcommand):
    """`build_parser()` 里那只子命令的 `--action` choices 现量集合"""
    for action in build_parser()._actions:
        choices = getattr(action, "choices", None)
        if isinstance(choices, dict) and subcommand in choices:
            for arg in choices[subcommand]._actions:
                if arg.dest == "action":
                    return set(arg.choices)
            raise AssertionError(f"{subcommand} 子命令没有 --action ⇒ 对账失效")
    raise AssertionError(f"build_parser() 里没有 {subcommand} 子命令 ⇒ 对账失效")


class _RecordingLock:
    """可观测的锁替身：`__enter__` 一进场就举旗，再把真锁的申请交给内部 Lock

    双检锁那一族的第二支（锁内复查）要**确定性地**走到，就必须让工作线程确实卡在
    `acquire()` 上；`threading.Lock` 本身不给这个观察点，所以换个有旗子的替身。
    """

    def __init__(self):
        self.inner = threading.Lock()
        self.entered = threading.Event()

    def __enter__(self):
        self.entered.set()
        self.inner.acquire()
        return self

    def __exit__(self, *exc):
        self.inner.release()


def _src(obj):
    return inspect.getsource(obj)


# ============ 甲档：死支删除与等价改写的形状判据 ============

class TestDeadBranchesAreGone:
    """甲档 7 格：形状必须消失，行为必须不变"""

    def test_cleaner_loop_no_longer_rechecks_membership(self):
        assert _membership_guards_inside_for_loops(_src(DatasetCleaner.clean)) == []

    def test_cleaner_still_raises_on_unknown_rule_and_applies_every_named_rule(self):
        cleaner = DatasetCleaner()
        items = [{"instruction": "  hello   world  ", "output": "ok"}]
        with pytest.raises(DataValidationError):
            cleaner.clean(items, rules=["normalise_whitespace"])
        _, result = cleaner.clean(items, rules=["normalize_whitespace", "trim_whitespace"])
        assert result.rules_applied == ["normalize_whitespace", "trim_whitespace"]

    def test_rag_chunk_text_has_no_break_and_no_truthiness_guard(self):
        shape = _old_style_loop_body(_src(RAGFormatter.chunk_text))
        assert shape == {"for": False, "break": False, "truthy_ifs": []}, shape

    def test_report_metric_names_are_data_not_an_if_elif_chain(self):
        assert _metric_name_if_chain(_src(ReportGenerator._generate_suggestions)) == []

    def test_quality_report_handler_no_longer_guards_the_recommendations_list(self):
        assert _truthy_guard_on(_src(run_quality_report), "report.recommendations") == []

    def test_retry_keeps_exactly_one_break_and_the_single_terminator(self):
        assert _last_attempt_break(_src(with_retries)) == []
        breaks = [n for n in ast.walk(_tree(_src(with_retries)))
                  if isinstance(n, ast.Break)]
        assert len(breaks) == 1, "「不可重试」那一支是唯一该留的 break"

    def test_config_validator_non_dict_entry_reports_without_a_dedup_guard(self):
        source = _src(ConfigValidator._validate_replay_sections)
        assert _dedup_by_err_paths(source) == [], \
            "「`x not in err_paths` 才报」那一形消失了才算收口；留着就是把不可达的保险写回代码"
        assert source.count("path in err_paths") == 2, \
            "批量回放那两处复合去重保留，本轮只收「唯一产地」那一格"
        assert source.count('result.add_error("models.%s" % name, str(exc))') == 1, \
            "非字典条目那一档该是直报：一声，不加任何前置判断"


# ============ 甲档：等价性由行为证明，不由推理证明 ============

class TestRewritesPreserveBehaviour:
    """改写前后逐格同判决——`chunk_text` 走全网格对拍，其余走点名用例"""

    def test_chunk_text_matches_the_old_break_based_algorithm_on_the_full_grid(self):
        def old(text, size, overlap):
            if not isinstance(text, str) or not text:
                return []
            if len(text) <= size:
                return [text]
            out, step = [], size - overlap
            for start in range(0, len(text), step):
                chunk = text[start:start + size]
                if chunk:
                    out.append(chunk)
                if start + size >= len(text):
                    break
            return out

        cases = 0
        for size in range(1, 13):
            for overlap in range(0, size):
                for length in range(1, 41):
                    text = "".join("abcdefghij"[(i * 3 + size) % 10] for i in range(length))
                    formatter = RAGFormatter(chunk_size=size, chunk_overlap=overlap)
                    assert formatter.chunk_text(text) == old(text, size, overlap), \
                        f"size={size} overlap={overlap} len={length}"
                    cases += 1
        assert cases == GRID_CASES

    def test_chunk_text_still_returns_the_documented_edge_shapes(self):
        formatter = RAGFormatter(chunk_size=4, chunk_overlap=1)
        assert formatter.chunk_text("") == []
        assert formatter.chunk_text(None) == []
        assert formatter.chunk_text("abc") == ["abc"]
        assert formatter.chunk_text("abcde") == ["abcd", "de"], \
            "旧写法也是这个读数：break 在下一跳的起点上收手，尾巴落进最后一块"
        assert formatter.chunk_text("abcdefghij") == ["abcd", "defg", "ghij"]
        exact = RAGFormatter(chunk_size=4, chunk_overlap=0)
        assert exact.chunk_text("abcdefgh") == ["abcd", "efgh"]

    def test_retry_runs_max_retries_plus_one_calls_and_never_sleeps_after_the_last(self):
        for max_retries in range(0, 5):
            calls, sleeps = [], []

            def boom(calls=calls, sleeps=sleeps):
                calls.append(1)
                raise ValueError("always fails")

            with pytest.raises(ValueError):
                with_retries(boom, max_retries=max_retries, base_delay=0.0,
                             retry_on=(ValueError,), sleeper=sleeps.append)
            assert len(calls) == max_retries + 1, max_retries
            assert len(sleeps) == max_retries, \
                f"max_retries={max_retries} 时末次失败不该再等一次：{sleeps}"

    def test_retry_non_retryable_gives_up_immediately_and_success_returns_stats(self):
        seen = []

        def nope():
            seen.append(1)
            raise TypeError("not in retry_on")

        with pytest.raises(TypeError):
            with_retries(nope, max_retries=3, sleeper=lambda d: None,
                         retry_on=(ValueError,))
        assert len(seen) == 1

        stats_seen = []
        value, stats = with_retries(lambda: "ok", max_retries=3,
                                    sleeper=lambda d: stats_seen.append(d))
        assert (value, stats.attempts, stats.succeeded) == ("ok", 1, True)

    def test_report_suggestions_match_the_old_if_elif_chain_on_every_shape(self):
        """改写等价性对拍：把改前那段 if/elif 链原样抄进测试，逐格比整份建议表"""
        def old(passed, total, scores, dedup_summary):
            out = []
            pass_rate = passed / total if total else 0.0
            if pass_rate < 0.5:
                out.append("通过率偏低，建议放宽过滤阈值以保留更多有效样本")
            elif pass_rate > 0.95:
                out.append("通过率过高，建议适当提高阈值以提升数据质量")
            if dedup_summary.get("removal_rate", 0) > 0.5:
                out.append("去重移除比例较高，种子数据同质化严重，建议增加多样性")
            metrics = ["semantic_similarity", "relevance", "diversity"]
            labels = ["语义相似度", "回答相关性", "多样性"]
            for metric, label in zip(metrics, labels):
                values = [getattr(s, metric, 0.0) for s in scores]
                if values and sum(values) / len(values) < 0.5:
                    if metric == "semantic_similarity":
                        out.append(f"{label}偏低，建议收敛生成约束或优化prompt")
                    elif metric == "relevance":
                        out.append(f"{label}偏低，建议检查问答匹配质量")
                    elif metric == "diversity":
                        out.append(f"{label}偏低，建议提高temperature或增加采样多样性")
            if not out:
                out.append("各项指标均在合理区间，数据质量良好")
            return out

        generator = ReportGenerator()
        low = SimpleNamespace(semantic_similarity=0.1, relevance=0.2, diversity=0.3)
        high = SimpleNamespace(semantic_similarity=0.9, relevance=0.9, diversity=0.9)
        mixed = SimpleNamespace(semantic_similarity=0.9, relevance=0.1, diversity=0.9)
        missing = SimpleNamespace(relevance=0.2)          # getattr 默认 0.0 那一档
        shapes = [[], [low], [high], [mixed], [missing], [low, high], [high, mixed, missing]]
        for scores in shapes:
            for passed, total in ((4, 10), (5, 10), (9, 10), (10, 10), (0, 0)):
                for dedup in ({}, {"removal_rate": 0.9}):
                    assert generator._generate_suggestions(passed, total, scores, dedup) \
                        == old(passed, total, scores, dedup), (passed, total, dedup)

        assert generator._generate_suggestions(4, 10, [low], {}) == [
            "通过率偏低，建议放宽过滤阈值以保留更多有效样本",
            "语义相似度偏低，建议收敛生成约束或优化prompt",
            "回答相关性偏低，建议检查问答匹配质量",
            "多样性偏低，建议提高temperature或增加采样多样性",
        ]
        assert generator._generate_suggestions(9, 10, [high], {})[-1] == \
            "各项指标均在合理区间，数据质量良好"
        assert generator._generate_suggestions(9, 10, [], {})[-1] == \
            "各项指标均在合理区间，数据质量良好", "空 scores 那一支靠 `values` 挡除零"

    def test_quality_recommendations_are_never_empty_for_any_dataset_shape(self):
        """甲档删掉兜底判断的依据：建议恒非空——把这句从推理变成读数"""
        from augmentor.quality_report import generate_quality_report

        shapes = [[], [{"instruction": "a", "output": "b"}],
                  [{"instruction": "同一句", "output": "同一句"}] * 6,
                  [{"input": "只有 input 键"}]]
        for items in shapes:
            report = generate_quality_report(items, "probe")
            assert report.recommendations, f"空建议会让删掉的兜底判断变成真缺口：{items[:1]}"

    def test_quality_report_handler_always_prints_the_suggestions_block(self, tmp_path):
        from augmentor.quality_report import generate_quality_report

        src = tmp_path / "in.json"
        src.write_text(json.dumps([{"instruction": "q1", "output": "a1"}]), encoding="utf-8")
        args = SimpleNamespace(input=str(src), threshold=0.6, output=None,
                               format="json", gate=False)
        out, err = StringIO(), StringIO()
        with redirect_stdout(out), redirect_stderr(err):
            run_quality_report(args, SimpleNamespace())
        assert "改进建议:" in out.getvalue()

    def test_config_validator_reports_each_bad_entry_exactly_once(self):
        result = validate_config({"models": {"bad_a": 5, "bad_b": "text", "default": "bad_a"}})
        paths = [e.path for e in result.errors]
        assert paths.count("models.bad_a") == 1, paths
        assert paths.count("models.bad_b") == 1, paths
        assert len(paths) == len(set(paths)), f"同一路径报了两声 ⇒ 删掉去重判据判错了：{paths}"


# ============ 乙档：链尾栅栏 ============

class TestChainTailFences:
    """L124 判过「记档不测」，本轮改判：封闭词表写在 parser.py，分支表写在 commands/*.py

    两点让「记档」不再够用：① 两份名单之间只有一条端点级尺子量过（L244 钉的是 backup
    一条），version / dependency 的 choices 与分支表脱节时没有任何东西会红；② 这些
    `run_*` 都能被直接调用（L244 自己就 import 了 `run_backup`），直接调用绕开 argparse
    的 2 号 usage 错误，落到链尾就是「什么都不做 + 退出 0」——那是本仓最难查的一档。
    栅栏把这一档翻成「点名 + 退出 1」，并配双向往复的词表对账。
    """

    def _expect_loud_exit(self, handler, args):
        out, err = StringIO(), StringIO()
        with redirect_stdout(out), redirect_stderr(err):
            with pytest.raises(SystemExit) as excinfo:
                handler(args, SimpleNamespace())
        assert excinfo.value.code == 1
        return out.getvalue(), err.getvalue()

    def test_unhandled_version_action_exits_loud(self, tmp_path):
        out, err = self._expect_loud_exit(
            run_version, SimpleNamespace(action="frobnicate",
                                        versions_dir=str(tmp_path / "v")))
        assert out == "", "链尾该一声不出，只走 stderr"
        assert "frobnicate" in err and "choices 与分支表脱节" in err, err

    def test_unhandled_backup_action_exits_loud(self, tmp_path):
        out, err = self._expect_loud_exit(
            run_backup, SimpleNamespace(action="frobnicate",
                                       backup_dir=str(tmp_path / "b"), name=None,
                                       input=None, output=None, max_backups=5))
        assert out == ""
        assert "frobnicate" in err and "backup 分支" in err, err

    def test_unhandled_dependency_action_exits_loud(self, tmp_path):
        out, err = self._expect_loud_exit(
            run_dependency, SimpleNamespace(action="frobnicate",
                                           registry_path=str(tmp_path / "r.json")))
        assert out == ""
        assert "frobnicate" in err and "dependency 分支" in err, err

    def test_argparse_still_rejects_the_same_action_with_two(self):
        """两层各判各的：parser 的 2 号 usage 错误没被栅栏替掉"""
        for argv in (["version", "--action", "frobnicate"],
                     ["backup", "--action", "frobnicate"],
                     ["dependency", "--action", "frobnicate"]):
            with pytest.raises(SystemExit) as excinfo:
                build_parser().parse_args(argv)
            assert excinfo.value.code == 2, argv

    def test_every_fence_handler_has_exactly_one_dispatch_chain_and_it_ends_in_else(self):
        """栅栏在场：分派链只有一条（判据没飘到别的 if 上），链尾收在 `else`（栅栏接上了）"""
        for name, handler in (("run_version", run_version), ("run_backup", run_backup),
                              ("run_dependency", run_dependency), ("export", Exporter.export),
                              ("recommend_seeds", ActiveSampler.recommend_seeds)):
            chains = _dispatch_chains(_src(handler))
            assert len(chains) == 1, (name, chains)
            assert chains[0][3] is True, (name, chains)

    def test_export_and_sampler_fence_branch_counts_match_the_closed_lists(self):
        """栅栏前面的分支数：export 六个原生成员、sampler 两类覆盖不足——各就各位"""
        assert len(_dispatch_chains(_src(Exporter.export))[0][2]) == 6
        assert _dispatch_chains(_src(ActiveSampler.recommend_seeds))[0][2] == \
            sorted(["length", "question_type"])

    def test_parser_choices_and_chain_actions_are_the_same_set(self):
        """双向往复：词表长一档而分支表没跟着长 ⇒ 红；分支表多余一档 ⇒ 也红"""
        pairs = [("version", run_version), ("backup", run_backup),
                 ("dependency", run_dependency)]
        for subcommand, handler in pairs:
            assert _parser_action_choices(subcommand) == _chain_branch_actions(
                _src(handler)), subcommand

    def test_export_chain_tail_raises_when_the_native_tuple_grows(self, tmp_path):
        extra = tuple(NATIVE_FORMATS) + (ExportFormat.TSV,)
        import augmentor.export as export_module
        original = export_module.NATIVE_FORMATS
        export_module.NATIVE_FORMATS = extra
        try:
            target = tmp_path / "out.tsv"
            with pytest.raises(UnsupportedFormatError):
                export_module.Exporter().export(
                    [{"instruction": "q", "output": "a"}], str(target), format="tsv")
            assert not target.exists(), "报错之前不该留下半份产物"
        finally:
            export_module.NATIVE_FORMATS = original

    def test_every_real_native_format_still_writes_a_file(self, tmp_path):
        exporter = Exporter()
        items = [{"instruction": "问题", "input": "", "output": "答案"}]
        for member in NATIVE_FORMATS:
            target = tmp_path / f"out-{member.value}.txt"
            used = exporter.export(items, str(target), format=member.value)
            assert used == member.value
            assert target.exists() and target.stat().st_size > 0, member

    def test_sampler_raises_on_a_category_the_producer_cannot_emit(self):
        sampler = ActiveSampler()
        sampler.analyze_coverage = lambda items: {"question_type_distribution": {}}
        sampler.identify_underrepresented = lambda items, analysis=None: ["topic:science"]
        with pytest.raises(DataValidationError):
            sampler.recommend_seeds([{"instruction": "为什么天空是蓝的"}])

    def test_sampler_still_recommends_seeds_for_the_two_real_categories(self):
        sampler = ActiveSampler()
        items = [{"instruction": "如何备份数据库文件"}, {"instruction": "短"},
                 {"instruction": "这是一个足够长的说明句子用来落进长档里面"}]
        result = sampler.recommend_seeds(items, top_k=5)
        assert result.recommended_seeds is not None
        assert isinstance(result.recommended_seeds, list)


# ============ 丙档：可达的 6 格，逐格补行为 ============

class TestReachableArcsAreCovered:
    """这 6 格不是死支，只是此前没有任何用例走过去"""

    def test_disk_cache_eviction_can_run_the_loop_down_to_the_last_entry(self, tmp_path):
        warm = DiskCache(tmp_path, max_bytes=None)
        warm.set("k1", {"payload": "x" * 200})
        warm.set("k2", {"payload": "y" * 200})
        assert set(warm._metadata) == {"k1", "k2"}

        tight = DiskCache(tmp_path, max_bytes=1)
        tight._enforce_size_limit()
        assert tight._metadata == {}, "上限 1 字节 ⇒ 两条都得淘汰，淘汰循环走到最后一条"
        assert tight.get("k1") is None and tight.get("k2") is None

        reread = DiskCache(tmp_path)
        assert reread._metadata == {}, "淘汰完没落盘 ⇒ 新实例会把幽灵元数据读回来"

    def test_disk_cache_breaks_out_while_it_is_still_above_the_cap(self, tmp_path):
        warm = DiskCache(tmp_path, max_bytes=None)
        warm.set("old", {"payload": "x" * 400})
        warm.set("new", {"payload": "y" * 400})
        sizes = {key: warm._get_cache_path(key).stat().st_size for key in ("old", "new")}
        keep = sizes["new"] + 1
        tight = DiskCache(tmp_path, max_bytes=keep)
        tight._enforce_size_limit()
        assert set(tight._metadata) == {"new"}, "达标就该停手，最旧那条之外的数据不该被牵连"

    def test_api_main_auth_warning_takes_both_directions_at_import(self, monkeypatch, caplog):
        """导入期那条告警是「无鉴权」的唯一出口：两个方向都在本进程里 reload 出读数

        走 reload 而不是子进程：`api.main:197` 那条 `if not _api_key_required():` 的**假支**
        （设了 key ⇒ 跳过告警直达下一句）正是覆盖率里最后剩的那格偏支，而覆盖率只量本进程，
        子进程里跑得再对也不会让那一格闭合。`api_key_required()` 每次调用现读环境变量
        （`api.deps`），所以 reload 就足以换档，不必换解释器。
        收尾刻意停在「没设 key」那一档 ⇒ 与门禁进程的常态导入状态同形。
        """
        import api.main as api_main

        for key, expect_warning in (("probe-key", False), (None, True)):
            if key is None:
                monkeypatch.delenv("AUGMENTOR_API_KEY", raising=False)
            else:
                monkeypatch.setenv("AUGMENTOR_API_KEY", key)
            caplog.clear()
            with caplog.at_level(logging.WARNING, logger="api.main"):
                importlib.reload(api_main)
            assert ("无鉴权" in caplog.text) is expect_warning, \
                (key, caplog.text[-500:])

    def test_model_manager_double_check_reuses_the_instance_another_thread_made(self):
        saved_instance, saved_lock = ModelManager._instance, ModelManager._lock
        recording = _RecordingLock()
        try:
            ModelManager._lock = recording
            ModelManager._instance = None
            winner = ModelManager.__new__(ModelManager)
            winner._initialized = True

            ModelManager._instance = None
            recording.inner.acquire()
            handed_back = []
            worker = threading.Thread(target=lambda: handed_back.append(ModelManager()))
            worker.start()
            assert recording.entered.wait(2.0), "工作线程没走到 with 里 ⇒ 双检那一支没量到"
            ModelManager._instance = winner
            recording.inner.release()
            worker.join(2.0)

            assert not worker.is_alive(), "工作线程卡在锁上没回来"
            assert handed_back == [winner], handed_back
            assert ModelManager._instance is winner, "锁内没复查 ⇒ 第二个线程会另造一只单例"
        finally:
            ModelManager._lock = saved_lock
            ModelManager._instance = saved_instance

    def test_searcher_double_check_does_not_rebuild_indexes_another_thread_built(self):
        searcher = EnhancedSearcher([{"instruction": "唯一的一条"}])
        recording = _RecordingLock()
        searcher._index_lock = recording
        built = []
        searcher._build_indexes = lambda: built.append(1)

        recording.inner.acquire()
        worker = threading.Thread(target=searcher._ensure_indexes)
        worker.start()
        assert recording.entered.wait(2.0), "工作线程没走到 with 里 ⇒ 锁内复查那一支没量到"
        searcher._index_ready = True
        recording.inner.release()
        worker.join(2.0)

        assert not worker.is_alive()
        assert built == [], "另一只线程已经建好了，这一只不该再扫一遍全文"

    def test_pipeline_parallel_failure_without_checkpoint_does_not_touch_the_manager(self,
                                                                                     tmp_path,
                                                                                     monkeypatch):
        monkeypatch.chdir(tmp_path)
        src = tmp_path / "in.json"
        src.write_text(json.dumps([{"instruction": "good-1", "input": "", "output": "a"},
                                   {"instruction": "bad-1", "input": "", "output": "b"}],
                                  ensure_ascii=False), encoding="utf-8")
        pipeline = AugmentorPipeline(config=_minimal_app_config())
        calls = []

        def variants(item, *args, **kwargs):
            if str(item.get("instruction", "")).startswith("bad"):
                raise RuntimeError("boom")
            return [dict(item, instruction=item["instruction"] + "-v")]

        pipeline._generate_variants = variants
        pipeline.checkpoint_manager.update_progress = \
            lambda *a, **k: calls.append(a)

        report = pipeline.augment_dataset(str(src), str(tmp_path / "out.json"),
                                         use_checkpoint=False, use_parallel=True,
                                         use_quality_check=False, use_dedup=False)
        assert any(e["error_type"] == "RuntimeError" for e in report["errors"]), report["errors"]
        assert calls == [], "use_checkpoint=False 时失败条目不该写进断点"

    def test_visualizer_second_load_takes_the_already_loaded_path(self, tmp_path, monkeypatch):
        sentinel = type("WordCloud", (), {"marker": "stub"})
        stub = ModuleType("wordcloud")
        stub.WordCloud = sentinel
        monkeypatch.setitem(sys.modules, "wordcloud", stub)

        visualizer = DataVisualizer(str(tmp_path / "viz"))
        visualizer._load_wordcloud()
        assert visualizer._wordcloud is sentinel
        visualizer._load_wordcloud()
        assert visualizer._wordcloud is sentinel, "第二次调用该直接走过，不该重新 import"

    def test_visualizer_first_load_without_the_package_leaves_it_unset(self, tmp_path,
                                                                       monkeypatch):
        monkeypatch.setitem(sys.modules, "wordcloud", None)
        visualizer = DataVisualizer(str(tmp_path / "viz2"))
        visualizer._load_wordcloud()
        assert visualizer._wordcloud is None


def _minimal_app_config():
    from augmentor.config import AppConfig

    config = AppConfig()
    config.augmentation.num_threads = 1
    config.quality.enabled = False
    config.dedup.enabled = False
    return config


# ============ 名册自证与注入臂 ============

class TestRosterAndRulersAreNotVacuous:
    """本轮的 18 格名册要分得干净，每条形状判据都要能被「改前源码」判红"""

    def test_the_roster_partitions_eighteen_sites_across_three_tiers(self):
        assert len(DEAD_REWRITES) == 7
        assert len(CHAIN_FENCES) == 5
        assert len(COVERED_ARCS) == 6
        assert len(DEAD_REWRITES) + len(CHAIN_FENCES) + len(COVERED_ARCS) == 18, \
            "三档加起来不等于 ⑫ 名册的 18 格 ⇒ 有名册外的格子被自称收口，或有格子没落档"
        named_files = {v[0] for v in DEAD_REWRITES.values()} \
            | {v[0] for v in CHAIN_FENCES.values()} \
            | {v[0] for v in COVERED_ARCS.values()}
        assert len(named_files) == 16, named_files
        for name, (path, _) in DEAD_REWRITES.items():
            assert (ROOT / path).exists(), name

    def test_arm_cleaner_the_membership_guard_back_in(self):
        injected = _snip("""
            for rule_name in rules:
                if rule_name in self._default_rules:
                    rule_func = self._default_rules[rule_name]
        """)
        assert _membership_guards_inside_for_loops(injected) == [2]

    def test_arm_rag_the_break_based_loop_back_in(self):
        injected = _snip("""
            for start in range(0, len(text), step):
                chunk = text[start:start + size]
                if chunk:
                    chunks.append(chunk)
                if start + size >= len(text):
                    break
        """)
        assert _old_style_loop_body(injected) == {"for": True, "break": True,
                                                  "truthy_ifs": [3]}

    def test_arm_retry_the_second_terminator_back_in(self):
        injected = _snip("""
            if attempt >= max_retries:
                break
        """)
        assert _last_attempt_break(injected) == [1]

    def test_arm_report_the_metric_name_chain_back_in(self):
        injected = _snip("""
            if metric == "semantic_similarity":
                pass
            elif metric == "relevance":
                pass
            elif metric == "diversity":
                pass
        """)
        assert _metric_name_if_chain(injected) == ["semantic_similarity", "relevance",
                                                   "diversity"]

    def test_arm_quality_the_empty_list_guard_back_in(self):
        injected = _snip("""
            if report.recommendations:
                print(1)
        """)
        assert _truthy_guard_on(injected, "report.recommendations") == [1]

    def test_arm_config_validator_the_dedup_back_in(self):
        injected = _snip("""
            if where not in err_paths:
                add_error(where)
        """)
        assert _dedup_by_err_paths(injected) == [1]

    def test_arm_action_table_missing_one_choice(self):
        injected = _snip("""
            if args.action == "register":
                pass
            elif args.action == "list":
                pass
        """)
        assert _chain_branch_actions(injected) == {"register", "list"}
        assert _parser_action_choices("dependency") - _chain_branch_actions(injected) == \
            {"graph", "validate"}

    def test_arm_action_table_with_a_branch_the_parser_does_not_offer(self):
        injected = _snip("""
            if args.action == "register":
                pass
            elif args.action == "teleport":
                pass
        """)
        assert _chain_branch_actions(injected) - _parser_action_choices("dependency") == \
            {"teleport"}

    def test_arm_chain_without_a_tail_else_is_reported(self):
        without_else = _snip("""
            def handler(args, config):
                if args.action == "register":
                    print(1)
                elif args.action == "list":
                    print(2)
        """)
        chains = _dispatch_chains(without_else)
        assert len(chains) == 1, chains
        assert chains[0][3] is False, "栅栏在场判据认不出「链尾落空」⇒ 乙档五格等于没量"

    def test_arm_enum_shaped_chain_without_a_tail_else_is_reported(self):
        """export 那一格比的是 `Enum.MEMBER` 不是字符串：同一把尺子要认这一形"""
        without_else = _snip("""
            def export(items, path, export_format):
                if export_format == ExportFormat.JSONL:
                    print(1)
                elif export_format == ExportFormat.CSV:
                    print(2)
        """)
        chains = _dispatch_chains(without_else)
        assert len(chains) == 1, chains
        assert chains[0][2] == sorted(["ExportFormat.CSV", "ExportFormat.JSONL"]), chains
        assert chains[0][3] is False, chains

    def test_the_shape_checkers_read_the_real_sources_not_a_fixture(self):
        """防「判据只对自己的注入样本有效」：每条判据都必须在真实源码上取到读数"""
        assert _membership_guards_inside_for_loops(_src(DatasetCleaner.clean)) == []
        assert _last_attempt_break(_src(with_retries)) == []
        assert _metric_name_if_chain(_src(ReportGenerator._generate_suggestions)) == []
        assert _truthy_guard_on(_src(run_quality_report), "report.recommendations") == []
        assert _dedup_by_err_paths(_src(ConfigValidator._validate_replay_sections)) == []

        assert _old_style_loop_body(_src(RAGFormatter.chunk_text))["break"] is False
