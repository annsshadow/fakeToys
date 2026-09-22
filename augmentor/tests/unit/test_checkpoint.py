# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""CheckpointManager 单元测试

重点验证 R4「断点续传」的核心语义：
- 增量保存（delta）与全量保存（checkpoint）的协作；
- 恢复后进度必须与保存时一致，不能重复累加；
- 恢复后剩余索引正确。
"""

import json

from augmentor.checkpoint import CheckpointData, CheckpointManager


def make_manager(tmp_path, **kwargs) -> CheckpointManager:
    return CheckpointManager(checkpoint_dir=str(tmp_path), **kwargs)


class TestCreateAndLoad:
    def test_create_checkpoint_initializes_state(self, tmp_path):
        manager = make_manager(tmp_path)
        checkpoint = manager.create_checkpoint("task-1", 10)

        assert isinstance(checkpoint, CheckpointData)
        assert checkpoint.task_id == "task-1"
        assert checkpoint.total_items == 10
        assert checkpoint.processed_items == 0
        assert checkpoint.failed_items == 0
        assert checkpoint.completed_indices == []
        assert checkpoint.failed_indices == []
        assert checkpoint.quality_scores == {}
        assert (tmp_path / "task-1_checkpoint.json").exists()

    def test_load_missing_checkpoint_returns_none(self, tmp_path):
        assert make_manager(tmp_path).load_checkpoint("nope") is None

    def test_load_corrupted_checkpoint_returns_none(self, tmp_path):
        manager = make_manager(tmp_path)
        (tmp_path / "broken_checkpoint.json").write_text("{not json", encoding="utf-8")
        assert manager.load_checkpoint("broken") is None

    def test_create_checkpoint_resets_pending_state(self, tmp_path):
        manager = make_manager(tmp_path)
        manager.create_checkpoint("task-1", 5)
        manager.update_progress(0, True)
        assert manager._pending_completed == {0}

        manager.create_checkpoint("task-2", 5)
        assert manager._pending_completed == set()
        assert manager._pending_failed == set()
        assert manager._last_save_count == 0


class TestUpdateProgress:
    def test_success_updates_counters_and_indices(self, tmp_path):
        manager = make_manager(tmp_path)
        manager.create_checkpoint("task-1", 5)
        manager.update_progress(0, True, quality_score=0.9)

        checkpoint = manager._current_checkpoint
        assert checkpoint.processed_items == 1
        assert checkpoint.completed_indices == [0]
        assert checkpoint.quality_scores == {0: 0.9}

    def test_failure_updates_failed_counters(self, tmp_path):
        manager = make_manager(tmp_path)
        manager.create_checkpoint("task-1", 5)
        manager.update_progress(2, False)

        checkpoint = manager._current_checkpoint
        assert checkpoint.processed_items == 0
        assert checkpoint.failed_items == 1
        assert checkpoint.failed_indices == [2]

    def test_without_checkpoint_is_noop(self, tmp_path):
        manager = make_manager(tmp_path)
        manager.update_progress(0, True)  # 不应抛异常
        assert manager._current_checkpoint is None

    def test_auto_save_writes_delta_file(self, tmp_path):
        manager = make_manager(tmp_path, auto_save_interval=3)
        manager.create_checkpoint("task-1", 10)

        for i in range(2):
            manager.update_progress(i, True)
        assert not (tmp_path / "task-1_delta.json").exists()

        manager.update_progress(2, True)
        delta_path = tmp_path / "task-1_delta.json"
        assert delta_path.exists()

        delta = json.loads(delta_path.read_text(encoding="utf-8"))
        assert delta["completed"] == [0, 1, 2]
        assert manager._pending_completed == set()


class TestResumeSemantics:
    """R4：中断后恢复必须与保存时进度一致"""

    def test_resume_after_manual_save(self, tmp_path):
        manager = make_manager(tmp_path)
        manager.create_checkpoint("task-1", 5)
        manager.update_progress(0, True, quality_score=0.8)
        manager.update_progress(1, True)
        manager.save_checkpoint()

        resumed = make_manager(tmp_path).load_checkpoint("task-1")

        assert resumed.processed_items == 2
        assert resumed.completed_indices == [0, 1]
        assert resumed.quality_scores == {0: 0.8}
        assert resumed.total_items == 5

    def test_resume_after_crash_before_manual_save(self, tmp_path):
        """模拟进程崩溃：只有增量落盘，checkpoint 文件仍是初始状态"""
        manager = make_manager(tmp_path, auto_save_interval=2)
        manager.create_checkpoint("task-1", 5)
        manager.update_progress(0, True, quality_score=0.5)
        manager.update_progress(1, True, quality_score=0.6)
        # 崩溃：不调用 save_checkpoint

        resumed = make_manager(tmp_path).load_checkpoint("task-1")

        assert resumed.processed_items == 2
        assert resumed.completed_indices == [0, 1]
        assert resumed.quality_scores == {0: 0.5, 1: 0.6}

    def test_repeated_load_does_not_accumulate_delta(self, tmp_path):
        manager = make_manager(tmp_path, auto_save_interval=1)
        manager.create_checkpoint("task-1", 5)
        manager.update_progress(0, True)
        manager.update_progress(1, True)

        first = make_manager(tmp_path).load_checkpoint("task-1")
        second = make_manager(tmp_path).load_checkpoint("task-1")

        assert first.processed_items == 2
        assert second.processed_items == 2
        assert second.completed_indices == [0, 1]

    def test_remaining_indices_after_resume(self, tmp_path):
        manager = make_manager(tmp_path)
        manager.create_checkpoint("task-1", 5)
        manager.update_progress(0, True)
        manager.update_progress(3, False)
        manager.save_checkpoint()

        resumed = make_manager(tmp_path)
        resumed.load_checkpoint("task-1")

        assert resumed.get_remaining_indices() == [1, 2, 4]

    def test_remaining_indices_without_checkpoint(self, tmp_path):
        assert make_manager(tmp_path).get_remaining_indices() == []


class TestProgressReporting:
    def test_progress_without_checkpoint(self, tmp_path):
        assert make_manager(tmp_path).get_progress() == {"status": "no_checkpoint"}

    def test_progress_fields(self, tmp_path):
        manager = make_manager(tmp_path)
        manager.create_checkpoint("task-1", 4)
        manager.update_progress(0, True, quality_score=0.8)
        manager.update_progress(1, True, quality_score=0.6)
        manager.update_progress(2, False)

        progress = manager.get_progress()

        assert progress["task_id"] == "task-1"
        assert progress["total_items"] == 4
        assert progress["processed_items"] == 2
        assert progress["failed_items"] == 1
        assert progress["progress"] == 0.5
        assert progress["progress_percent"] == "50.0%"
        assert progress["avg_quality_score"] == 0.7
        assert progress["elapsed_time"].endswith("秒")

    def test_average_quality_score_without_scores(self, tmp_path):
        manager = make_manager(tmp_path)
        manager.create_checkpoint("task-1", 2)
        assert manager._calculate_avg_quality_score() == 0.0

    def test_average_quality_score_without_checkpoint(self, tmp_path):
        assert make_manager(tmp_path)._calculate_avg_quality_score() == 0.0

    def test_zero_total_items_does_not_divide_by_zero(self, tmp_path):
        manager = make_manager(tmp_path)
        manager.create_checkpoint("task-1", 0)
        assert manager.get_progress()["progress"] == 0


class TestFormatTime:
    def test_seconds(self, tmp_path):
        assert make_manager(tmp_path)._format_time(30) == "30.0秒"

    def test_minutes(self, tmp_path):
        assert make_manager(tmp_path)._format_time(120) == "2.0分钟"

    def test_hours(self, tmp_path):
        assert make_manager(tmp_path)._format_time(7200) == "2.0小时"


class TestSaveAndDelete:
    def test_save_checkpoint_without_checkpoint_is_noop(self, tmp_path):
        manager = make_manager(tmp_path)
        manager.save_checkpoint()  # 不应抛异常
        assert list(tmp_path.glob("*_checkpoint.json")) == []

    def test_save_checkpoint_merges_delta_into_main_file(self, tmp_path):
        manager = make_manager(tmp_path, auto_save_interval=1)
        manager.create_checkpoint("task-1", 3)
        manager.update_progress(0, True)
        manager.save_checkpoint()

        data = json.loads((tmp_path / "task-1_checkpoint.json").read_text(encoding="utf-8"))
        assert data["processed_items"] == 1
        assert data["completed_indices"] == [0]

    def test_save_checkpoint_without_pending_does_not_write_delta(self, tmp_path):
        manager = make_manager(tmp_path)
        manager.create_checkpoint("task-1", 3)
        manager.save_checkpoint()
        assert not (tmp_path / "task-1_delta.json").exists()

    def test_delete_checkpoint_removes_both_files(self, tmp_path):
        manager = make_manager(tmp_path, auto_save_interval=1)
        manager.create_checkpoint("task-1", 3)
        manager.update_progress(0, True)
        assert (tmp_path / "task-1_delta.json").exists()

        manager.delete_checkpoint("task-1")

        assert not (tmp_path / "task-1_checkpoint.json").exists()
        assert not (tmp_path / "task-1_delta.json").exists()

    def test_delete_missing_checkpoint_is_noop(self, tmp_path):
        make_manager(tmp_path).delete_checkpoint("nope")

    def test_list_checkpoints(self, tmp_path):
        manager = make_manager(tmp_path)
        manager.create_checkpoint("task-a", 1)
        manager.create_checkpoint("task-b", 1)

        assert sorted(manager.list_checkpoints()) == ["task-a", "task-b"]

    def test_list_checkpoints_ignores_delta_files(self, tmp_path):
        manager = make_manager(tmp_path, auto_save_interval=1)
        manager.create_checkpoint("task-a", 2)
        manager.update_progress(0, True)

        assert manager.list_checkpoints() == ["task-a"]


class TestCheckpointExtended:
    """CheckpointManager 扩展测试"""

    def test_checkpoint_data_fields(self):
        """CheckpointData 应包含所有必要字段"""
        from datetime import datetime
        data = CheckpointData(
            task_id="test",
            total_items=10,
            processed_items=5,
            failed_items=1,
            completed_indices=[0, 1, 2, 3, 4],
            failed_indices=[5],
            quality_scores={0: 0.9, 1: 0.8},
            start_time=datetime.now().isoformat(),
            last_update_time=datetime.now().isoformat()
        )
        assert data.task_id == "test"
        assert data.total_items == 10
        assert len(data.completed_indices) == 5

    def test_update_progress_multiple_success(self, tmp_path):
        """多次成功更新"""
        manager = make_manager(tmp_path)
        manager.create_checkpoint("task-1", 10)
        for i in range(5):
            manager.update_progress(i, True, quality_score=0.8)
        assert manager._current_checkpoint.processed_items == 5

    def test_update_progress_mixed_success_failure(self, tmp_path):
        """混合成功和失败更新"""
        manager = make_manager(tmp_path)
        manager.create_checkpoint("task-1", 10)
        manager.update_progress(0, True)
        manager.update_progress(1, False)
        manager.update_progress(2, True)
        assert manager._current_checkpoint.processed_items == 2
        assert manager._current_checkpoint.failed_items == 1

    def test_get_remaining_indices_all_completed(self, tmp_path):
        """全部完成后剩余索引为空"""
        manager = make_manager(tmp_path)
        manager.create_checkpoint("task-1", 3)
        for i in range(3):
            manager.update_progress(i, True)
        manager.save_checkpoint()
        assert manager.get_remaining_indices() == []

    def test_delete_checkpoint_nonexistent(self, tmp_path):
        """删除不存在的 checkpoint 不应报错"""
        manager = make_manager(tmp_path)
        manager.delete_checkpoint("nonexistent")
        assert manager.list_checkpoints() == []

    def test_format_time_various(self, tmp_path):
        """各种时间格式"""
        manager = make_manager(tmp_path)
        assert manager._format_time(0) == "0.0秒"
        assert manager._format_time(0.5) == "0.5秒"
        assert manager._format_time(60) == "1.0分钟"
        assert manager._format_time(3600) == "1.0小时"

    def test_progress_with_failed_items(self, tmp_path):
        """有失败项时的进度"""
        manager = make_manager(tmp_path)
        manager.create_checkpoint("task-1", 4)
        manager.update_progress(0, True)
        manager.update_progress(1, False)
        manager.update_progress(2, True)
        progress = manager.get_progress()
        assert progress["failed_items"] == 1
        assert progress["processed_items"] == 2
