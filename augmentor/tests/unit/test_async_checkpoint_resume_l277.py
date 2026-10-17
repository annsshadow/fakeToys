# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""A368 那一格：`augment_async(use_checkpoint=…)` 从前是死参数（L277）

`docs/augmentor/API_CORE.md` 把 `use_checkpoint=True` 写在 `augment_async` 的公开签名上，
但改前的函数体一个字没读它：实测同一个 task_id 连跑两次，第二次仍然从零重跑全部条目，
断点目录里连文件都不会生成，`use_checkpoint=False` 与 `True` 的产物一模一样——文档承诺的
「跳过已完成条目 + 记进度」在这一条入口上是空话。同步入口 `augment_dataset` 早就接了，
所以这一格属 L215/L216 那一族老毛病：**只接一个入口的接线在文档里看不出来，只在产物里看得出来**。

本文件按同步入口的同一式子对账四件事：
① 首跑把每一笔进度落进**自己那一份**断点（`completed_indices` 逐格可数）；
② 续跑只提交剩余下标，且把上一轮的产物载回来再整份写回——改前这一支会把已完成的结果
   覆盖为空，是「跳过」与「载入旧产物」必须成对的那一刀；
③ 失败那一笔记进断点（`failed_indices`）而不是只留一行日志；
④ `use_checkpoint=False` 时盘上不落断点。

第 5 面是**反证臂**：把 `get_remaining_indices` 钉回「交回全部下标」（= 改前的死参数行为），
上一轮产物立刻从输出文件里消失。这一臂证明②的断言不是恒真，也证明这一对不是可以只接一半的。
"""

import asyncio
import json
import pathlib

import pytest

import augmentor.pipeline as pl
from augmentor import AugmentorPipeline, load_config
from augmentor.checkpoint import CheckpointManager

REPO = pathlib.Path(__file__).resolve().parents[2]


@pytest.fixture
def pipeline(tmp_path, monkeypatch):
    """离线管道，断点目录与 chdir 都重定向到 tmp（与 l68 / 分支测试同一套隔离）"""
    config = load_config(str(REPO / "config.yaml"))
    config.versioning.storage_dir = str(tmp_path / "versions")
    config.versioning.auto_snapshot = False
    instance = AugmentorPipeline(config)
    instance.version_manager.storage_dir = tmp_path / "versions"
    instance.checkpoint_manager.checkpoint_dir = tmp_path / "checkpoints"
    monkeypatch.chdir(tmp_path)
    return instance


def _seed_file(tmp_path, count):
    items = [{"instruction": f"问题{i}？", "input": "", "output": f"回答{i}"}
             for i in range(count)]
    path = tmp_path / "async_in.json"
    path.write_text(json.dumps(items, ensure_ascii=False), encoding="utf-8")
    return str(path)


def _install_spy(monkeypatch, calls, fail_outputs=()):
    """把 `augment_seed` 换成记账的桩：变体 = 一条，失败按 output 点名

    变体把 `input` 写成「变体」——续传用例要靠这一格把「上一轮的旧产物」和
    「本轮重跑生成的新行」区分开，否则两行内容一字不差，覆盖与保留看不出差别。
    """
    def fake_seed(self, item, use_quality_check=False, existing_generated=None):
        calls.append(item["output"])
        if item["output"] in fail_outputs:
            raise RuntimeError("桩失败: " + item["output"])
        return [{**item, "input": "变体"}]

    monkeypatch.setattr(pl.AugmentorPipeline, "augment_seed", fake_seed)


def _run(pipeline, src, out, **kwargs):
    kwargs.setdefault("use_quality_check", False)
    kwargs.setdefault("use_dedup", False)
    return asyncio.run(pipeline.augment_async(src, out, **kwargs))


class TestTheAsyncEntryFillsItsOwnCheckpoint:
    def test_first_run_records_every_index(self, pipeline, tmp_path, monkeypatch):
        calls = []
        _install_spy(monkeypatch, calls)
        src = _seed_file(tmp_path, 3)
        out = str(tmp_path / "async_out.json")

        _run(pipeline, src, out, use_checkpoint=True)

        ck = pipeline.checkpoint_manager.load_checkpoint(pipeline.build_task_id(src))
        assert ck is not None, "异步入口没落断点 ⇒ use_checkpoint 又成了死参数"
        assert ck.total_items == 3
        assert ck.completed_indices == [0, 1, 2]
        assert ck.failed_indices == []
        # 线程池里三笔的完成顺序不保证 ⇒ 判集合不判序列
        assert sorted(calls) == ["回答0", "回答1", "回答2"]

    def test_the_dead_parameter_shape_is_now_visible_on_disk(self, pipeline, tmp_path, monkeypatch):
        """`use_checkpoint=False` 一侧：跑完整批，但断点目录不落文件"""
        calls = []
        _install_spy(monkeypatch, calls)
        src = _seed_file(tmp_path, 2)
        out = str(tmp_path / "async_off.json")

        _run(pipeline, src, out, use_checkpoint=False)

        assert len(calls) == 2
        assert pipeline.checkpoint_manager.list_checkpoints() == []


class TestResumeKeepsPriorProducts:
    def test_resume_submits_only_remaining_indices_and_keeps_earlier_rows(
            self, pipeline, tmp_path, monkeypatch):
        src = _seed_file(tmp_path, 3)
        out = str(tmp_path / "async_out.json")
        task_id = pipeline.build_task_id(src)

        # 上一轮只跑完第 0 条，产物已在盘上
        mgr = pipeline.checkpoint_manager
        ck = mgr.create_checkpoint(task_id, 3)
        mgr.update_progress(0, True, checkpoint=ck)
        mgr.save_checkpoint(ck)
        pathlib.Path(out).write_text(
            json.dumps([{"instruction": "问题0？", "input": "", "output": "回答0"}],
                       ensure_ascii=False),
            encoding="utf-8")

        calls = []
        _install_spy(monkeypatch, calls)
        report = _run(pipeline, src, out, use_checkpoint=True)

        assert sorted(calls) == ["回答1", "回答2"], "续跑又提交了已完成的下标"
        rows = json.loads(pathlib.Path(out).read_text(encoding="utf-8"))
        assert [r["output"] for r in rows] == ["回答0", "回答1", "回答2"], (
            "续传时没载入上一轮产物 ⇒ 整份写回把已完成的结果覆盖掉了"
        )
        assert rows[0]["input"] == "", "第 0 行不是上一轮那份旧产物"
        assert [r["input"] for r in rows[1:]] == ["变体", "变体"]
        assert report["output_count"] == 3

    def test_the_clobber_returns_if_the_skip_is_not_wired(
            self, pipeline, tmp_path, monkeypatch):
        """反证臂：把 `get_remaining_indices` 钉回交回全部下标（= 改前的死参数），上一轮产物就丢

        这一臂是②的牙齿：如果「载入旧产物」那一段独立成立，摘掉跳过之后仍然该留 3 行；
        实测它变成 3 条新行、第 0 条的旧产物消失——证明跳过与载入是同一刀的两半。
        """
        src = _seed_file(tmp_path, 3)
        out = str(tmp_path / "async_out.json")
        mgr = pipeline.checkpoint_manager
        ck = mgr.create_checkpoint(pipeline.build_task_id(src), 3)
        mgr.update_progress(0, True, checkpoint=ck)
        mgr.save_checkpoint(ck)
        pathlib.Path(out).write_text(
            json.dumps([{"instruction": "问题0？", "input": "", "output": "回答0"}],
                       ensure_ascii=False),
            encoding="utf-8")

        monkeypatch.setattr(CheckpointManager, "get_remaining_indices",
                            lambda self, checkpoint=None: [0, 1, 2])

        calls = []
        _install_spy(monkeypatch, calls)
        _run(pipeline, src, out, use_checkpoint=True)

        assert sorted(calls) == ["回答0", "回答1", "回答2"]
        rows = json.loads(pathlib.Path(out).read_text(encoding="utf-8"))
        assert [r["output"] for r in rows] == ["回答0", "回答1", "回答2"]
        assert rows[0]["input"] == "变体", (
            "去掉跳过后旧产物仍在 ⇒ 本臂没咬到「整份写回覆盖」这一刀"
        )


class TestFailuresLandOnTheCheckpoint:
    def test_a_failing_item_is_recorded_as_failed_not_only_logged(
            self, pipeline, tmp_path, monkeypatch):
        calls = []
        _install_spy(monkeypatch, calls, fail_outputs=["回答1"])
        src = _seed_file(tmp_path, 3)
        out = str(tmp_path / "async_out.json")

        report = _run(pipeline, src, out, use_checkpoint=True)

        ck = pipeline.checkpoint_manager.load_checkpoint(pipeline.build_task_id(src))
        assert ck.failed_indices == [1]
        assert ck.completed_indices == [0, 2]
        assert ck.failed_items == 1
        rows = json.loads(pathlib.Path(out).read_text(encoding="utf-8"))
        assert [r["output"] for r in rows] == ["回答0", "回答2"]
        assert report["output_count"] == 2

    def test_a_failed_index_is_not_resubmitted_on_the_next_run(
            self, pipeline, tmp_path, monkeypatch):
        """失败格也是已完成格：`get_remaining_indices` 把 completed+failed 一起扣掉"""
        src = _seed_file(tmp_path, 3)
        out = str(tmp_path / "async_out.json")
        task_id = pipeline.build_task_id(src)

        mgr = pipeline.checkpoint_manager
        ck = mgr.create_checkpoint(task_id, 3)
        mgr.update_progress(1, False, checkpoint=ck)
        mgr.save_checkpoint(ck)

        calls = []
        _install_spy(monkeypatch, calls)
        _run(pipeline, src, out, use_checkpoint=True)

        assert sorted(calls) == ["回答0", "回答2"]
