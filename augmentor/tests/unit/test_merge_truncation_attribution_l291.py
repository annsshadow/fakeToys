# -*- coding: utf-8 -*-
"""L291：`merge_files` 的 `truncated_by_max_items` 只许记「真截过」的条数（A400 ①）。

立账原格把修法写定的是 L148 / B218 那句既有决定「去重删除与 max_items 截断分开报」：
改前的实现只在 `config` 非空时做到了，`config=None` 那一档因为上一格刻意记 0，
残差 `total_input - 0 - len(merged)` 全成了去重删除数，于是**去掉的条数被记到截断头上**，
而 `MergeConfig().max_items` 是 `None` ⇒ 截断在那一档根本没有发生过（一个不可能为正的
数报了正数，属于账本里「恒等式即假读数」那一族的反向形状：名字与判决相互矛盾的数）。

本文件的判据口径：
* 行为面按六档 config 现量钉死（含 `max_items=0` 与 `max_items=99` 两个边角），
  并单独钉「只有 `config=None` 这一档的读数被本轮推动，其余五档逐字不变」⇒ 零漂移可证。
* 形状面钉两格：那一行的表达式必须引 `max_items` 的判决（把修复撤成裸残差 ⇒ 当场红），
  以及 `dataset_ops.py:220` 这个在册活引用必须仍然落在这条赋值上（A357 点名的
  「尺子只判落点不判内容」用手工逐格核对替代）。
* 产物面钉「报表口径不动产物」：`config=None` 与显式 `MergeConfig(deduplicate=True)`
  两条写出的文件逐字节相同 ⇒ 本轮改的是读数归因，不是交付字节。

零 parametrize：l97 / l98 两把普查棘轮的读数不因本文件而变。
"""

import io
import json
import os
import sys

import pytest

REPO = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", ".."))
if REPO not in sys.path:
    sys.path.insert(0, REPO)

from augmentor.dataset_ops import DatasetOperations, MergeConfig  # noqa: E402

DATASET_OPS = os.path.join(REPO, "augmentor", "dataset_ops.py")
TRUNCATION_LINE = 220


def _write(path, payload):
    with io.open(str(path), "w", encoding="utf-8") as f:
        f.write(json.dumps(payload, ensure_ascii=False))
    return str(path)


@pytest.fixture
def corpus(tmp_path):
    """两份共享一条 `y`、其中一份内部自重复的五条输入语料。

    形状是量出来的：`total_input=5`、去重后 `3`，因此「去重删除 2」与「截断 2」在
    改前那个残差公式下给出同一个数 2 ⇒ 归因错了也看不出数不对，正是这一格危险的地方。
    """
    a = _write(tmp_path / "a.json",
               [{"instruction": "x"}, {"instruction": "x"}, {"instruction": "y"}])
    b = _write(tmp_path / "b.json",
               [{"instruction": "y"}, {"instruction": "z"}])
    return a, b, str(tmp_path / "out.json")


class TestAttributionIsTruthful:
    """截断那一格的正负必须只由 `max_items` 决定"""

    def test_config_none_reports_no_truncation(self, corpus):
        """改前红读数：`{truncated_by_max_items: 2}` 而 `max_items` 从未设置"""
        a, b, out = corpus
        r = DatasetOperations().merge_files([a, b], out, None)
        assert r["removed_duplicates"] == 0, "L196/B218 刻意口径：没传配置就不统计去重数"
        assert r["total_output"] == 3, "产物侧照默认去重 ⇒ 五条进、三条出"
        assert r["truncated_by_max_items"] == 0, (
            "5 - 0 - 3 的残差是去重删除数，不许记成 max_items 截断"
            "（MergeConfig().max_items 是 None，截断不可能发生）")

    def test_truncation_equals_the_independently_computed_truth(self, corpus):
        """逐档独立复算「真被截了几条」再与读数比：`None` / `max_items=0` / 上限高于条数三形都判"""
        a, b, out = corpus
        five_after_dedup = 3   # 本语料去重删 2 条（`corpus` 的 docstring 里量过）
        arms = [
            (None, 0),
            (MergeConfig(), 0),
            (MergeConfig(deduplicate=True), 0),
            (MergeConfig(deduplicate=False), 0),
            (MergeConfig(deduplicate=True, max_items=2), max(0, five_after_dedup - 2)),
            (MergeConfig(deduplicate=False, max_items=0), max(0, 5 - 0)),
            (MergeConfig(deduplicate=True, max_items=99), max(0, five_after_dedup - 99)),
            (MergeConfig(deduplicate=False, max_items=4), max(0, 5 - 4)),
        ]
        for cfg, truth in arms:
            r = DatasetOperations().merge_files([a, b], out, cfg)
            assert r["truncated_by_max_items"] == truth, (
                "档位 %r：读数 %r、独立复算 %r" % (
                    cfg, r["truncated_by_max_items"], truth))

    def test_the_five_arms_with_a_config_are_unchanged(self, corpus):
        """零漂移：只有 `config=None` 那一档被本轮推动，其余五档逐格与残差公式同值"""
        a, b, out = corpus
        expected = {
            "dedup_only": (2, 0),
            "max_2": (2, 1),
            "dedup_off": (0, 0),
            "max_0": (0, 5),
            "max_99": (2, 0),
        }
        got = {
            "dedup_only": _pair(DatasetOperations().merge_files([a, b], out,
                                                                MergeConfig(deduplicate=True))),
            "max_2": _pair(DatasetOperations().merge_files([a, b], out, MergeConfig(
                deduplicate=True, max_items=2))),
            "dedup_off": _pair(DatasetOperations().merge_files([a, b], out,
                                                               MergeConfig(deduplicate=False))),
            "max_0": _pair(DatasetOperations().merge_files([a, b], out, MergeConfig(
                deduplicate=False, max_items=0))),
            "max_99": _pair(DatasetOperations().merge_files([a, b], out, MergeConfig(
                deduplicate=True, max_items=99))),
        }
        assert got == expected, "带 config 的档位里 `removed` 是真的 ⇒ 残差公式仍成立"

    def test_the_report_shape_does_not_move_the_product(self, corpus, tmp_path):
        """报表归因改了，产物一字未动：两档写出同一份文件"""
        a, b, _ = corpus
        ops = DatasetOperations()
        p1 = str(tmp_path / "p1.json")
        p2 = str(tmp_path / "p2.json")
        ops.merge_files([a, b], p1, None)
        ops.merge_files([a, b], p2, MergeConfig(deduplicate=True))
        assert _bytes(p1) == _bytes(p2)


class TestTheRulerHasTeeth:
    """修复被撤掉时本文件必须响，而不是静默放行"""

    def test_the_formula_still_cites_the_ceiling(self):
        src = _merge_files_source()
        line = [ln for ln in src.splitlines() if "truncated_by_max_items =" in ln]
        assert len(line) == 1, line
        # 只看赋值号**右边**：左边那个名字本来就带 max_items，整行去比会恒真（注入臂 A1
        # 第一轮就是这么空的——撤成裸残差仍判绿，等于尺子没牙齿）。
        right = line[0].split("=", 1)[1]
        assert "max_items is None" in right, (
            "截断那一格一旦退回裸残差 `total_input - removed_duplicates - len(merged)`，"
            "`config=None` 档就把去重数报成截断数（本文件的案面）")

    def test_the_cited_line_number_still_lands_on_the_assignment(self):
        """在册活引用 `augmentor/dataset_ops.py:220` 的落点内容核对（A357 的手工替代）"""
        lines = io.open(DATASET_OPS, encoding="utf-8").read().split("\n")
        assert lines[TRUNCATION_LINE - 1].lstrip().startswith(
            "truncated_by_max_items = "), repr(lines[TRUNCATION_LINE - 1][:60])


def _pair(result):
    return result["removed_duplicates"], result["truncated_by_max_items"]


def _bytes(path):
    return io.open(path, "rb").read()


def _merge_files_source():
    import inspect

    return inspect.getsource(DatasetOperations.merge_files)
