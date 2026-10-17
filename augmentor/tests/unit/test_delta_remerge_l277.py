# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""A369（L277）：增量并回主文件时必须按「这一格记过没有」判

**这一格的两种来路**（`augmentor/checkpoint.py` 的 `load_checkpoint` 合并段）：

1. **崩溃窗**：`_save_checkpoint(ck)` 写成功之后、`_remove_delta(task_id)` 之前被杀
   ⇒ 盘上同时留着「已含这一批的主文件」与「这一批本身」。下次恢复再并一次，纯
   `extend` 实测把 6 条已完成读成 `processed_items=12`、`progress_percent=200.0%`，
   而且这份错读数会随合并**落盘固化**——下一次读它的人拿到的就是 12。
2. **两个增量文件同标一个下标**：`_delta_files` 把 legacy（`_delta.json`，整份一个
   文档）与 JSONL（`_delta.jsonl`，一行一批）一并读回再 `extend` 成一份 `delta`。
   L86 之前的运行留下的 legacy 与之后的新文件若都记了同一条，一次恢复里就记两笔。

`test_checkpoint_recovery_l100.py` 钉的是**写失败**那一支（主文件没写成 ⇒ 增量必须留着），
所以这两序当时无人守。本文件把两序都钉成读数，并验合并后的落盘与增量的清除。
"""

import json

from augmentor.checkpoint import CheckpointManager

TOTAL = 6
BATCH = [0, 1, 2, 3, 4, 5]


def _manager(tmp_path):
    return CheckpointManager(checkpoint_dir=str(tmp_path / "ck"))


def _finished_manager(tmp_path):
    """跑完 6 条并正常收尾：主文件 `processed_items=6`，此刻盘上没有增量"""
    mgr = _manager(tmp_path)
    ck = mgr.create_checkpoint("t1", total_items=TOTAL)
    for index in BATCH:
        mgr.update_progress(index, True, checkpoint=ck)
    mgr.save_checkpoint(ck)
    assert not mgr._get_delta_path("t1").exists()
    return mgr, ck


def _partial_manager(tmp_path, done):
    """跑完 `done` 这几条并正常收尾（主文件里就这些，盘上不留增量）"""
    mgr = _manager(tmp_path)
    ck = mgr.create_checkpoint("t1", total_items=TOTAL)
    for index in done:
        mgr.update_progress(index, True, checkpoint=ck)
    mgr.save_checkpoint(ck)
    return mgr, ck


def _main_row(tmp_path):
    """直接从盘上读主文件，不经 `load_checkpoint`（否则等于让被测对象自证）"""
    files = list((tmp_path / "ck").glob("*_checkpoint.json"))
    assert len(files) == 1, files
    return json.loads(files[0].read_text(encoding="utf-8"))


def _write_delta(path, record):
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(record, ensure_ascii=False), encoding="utf-8")


def _record(completed=None, failed=None, scores=None):
    return {
        "completed": completed or [],
        "failed": failed or [],
        "quality_scores": scores or {},
    }


class TestCrashWindowRemerge:
    def test_replaying_the_same_batch_adds_nothing(self, tmp_path):
        """主文件已含这一批、增量又活着 ⇒ 恢复之后条数与进度都不该翻倍"""
        mgr, _ck = _finished_manager(tmp_path)
        _write_delta(mgr._get_delta_path("t1"), _record(completed=BATCH))

        back = mgr.load_checkpoint("t1")

        assert back is not None
        assert back.completed_indices == BATCH, "同一批被并了两次"
        assert back.processed_items == TOTAL
        assert back.failed_items == 0

    def test_the_naive_extend_reading_would_have_been_twelve(self, tmp_path):
        """把「翻倍」这一形钉成可核对的算术：主 6 条 + 增量 6 条 ⇒ 纯 extend 得 12"""
        mgr, _ck = _finished_manager(tmp_path)
        _write_delta(mgr._get_delta_path("t1"), _record(completed=BATCH))
        assert len(_main_row(tmp_path)["completed_indices"]) == TOTAL

        merged = mgr.load_checkpoint("t1")
        assert len(merged.completed_indices) == TOTAL != 2 * TOTAL

    def test_progress_stops_at_one_hundred_percent(self, tmp_path):
        """`200.0%` 是这一格对外的读数形状，进度面单独钉一次"""
        mgr, _ck = _finished_manager(tmp_path)
        _write_delta(mgr._get_delta_path("t1"), _record(completed=BATCH))

        back = mgr.load_checkpoint("t1")
        assert mgr.get_progress(back)["progress_percent"] == "100.0%"

    def test_the_idempotent_merge_is_persisted_and_the_delta_removed(self, tmp_path):
        """合并结果要落盘、增量要清掉，否则下一次恢复又问一遍同一件事"""
        mgr, _ck = _finished_manager(tmp_path)
        delta_path = mgr._get_delta_path("t1")
        _write_delta(delta_path, _record(completed=BATCH))

        assert mgr.load_checkpoint("t1") is not None
        assert not delta_path.exists(), "合并成功却没删增量"
        assert _main_row(tmp_path)["processed_items"] == TOTAL

        again = _manager(tmp_path).load_checkpoint("t1")
        assert again.processed_items == TOTAL, "第二趟恢复又累加了一次"

    def test_failed_indices_replay_is_not_double_counted(self, tmp_path):
        """`failed` 那一侧与 `completed` 同判：重放同一批不增加 `failed_items`"""
        mgr = _manager(tmp_path)
        ck = mgr.create_checkpoint("t2", total_items=TOTAL)
        for index in BATCH:
            mgr.update_progress(index, False, checkpoint=ck)
        mgr.save_checkpoint(ck)
        assert _main_row(tmp_path)["failed_items"] == TOTAL

        _write_delta(mgr._get_delta_path("t2"), _record(failed=BATCH))
        back = mgr.load_checkpoint("t2")

        assert back.failed_items == TOTAL
        assert back.failed_indices == BATCH
        assert back.processed_items == 0


class TestTwoDeltaFilesCarryTheSameIndex:
    def test_legacy_and_jsonl_overlapping_merge_once(self, tmp_path):
        """legacy 与 JSONL 各记一次同一下标 ⇒ 并回主文件只算一笔（来路 ②）"""
        mgr, _ck = _partial_manager(tmp_path, [0, 1, 2, 3])
        both = [4, 5]
        _write_delta(mgr._get_legacy_delta_path("t1"), _record(completed=both))
        _write_delta(mgr._get_delta_path("t1"), _record(completed=both))
        assert len(mgr._delta_files("t1")) == 2, "两份增量都在场才有这一格"

        back = CheckpointManager(
            checkpoint_dir=str(tmp_path / "ck")
        ).load_checkpoint("t1")

        assert back.completed_indices == [0, 1, 2, 3, 4, 5]
        assert back.processed_items == 6, "两份增量各并一次会读成 8"


class TestAGenuinelyNewBatchStillMerges:
    def test_new_indices_are_added_and_counted(self, tmp_path):
        """去重不能把「真新的一笔」也吞掉：增量里的新下标照并、进度照涨"""
        mgr, _ck = _partial_manager(tmp_path, [0, 1, 2])
        _write_delta(mgr._get_delta_path("t1"), _record(completed=[2, 3, 4]))

        back = mgr.load_checkpoint("t1")

        assert back.completed_indices == [0, 1, 2, 3, 4]
        assert back.processed_items == 5, "已记过的 2 不该再记，而 3 / 4 两笔必须进来"

    def test_quality_scores_from_the_replayed_batch_survive(self, tmp_path):
        """评分表是字典并回，重放那一批不该把已有分数改成别的值"""
        mgr, _ck = _finished_manager(tmp_path)
        _write_delta(mgr._get_delta_path("t1"),
                     _record(completed=BATCH, scores={"0": 0.25}))

        back = mgr.load_checkpoint("t1")

        assert back.quality_scores[0] == 0.25
        assert len(back.completed_indices) == TOTAL
