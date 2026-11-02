# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""第 70 轮：删除断点必须同时清掉内存里的当前断点

`delete_checkpoint` 此前只 unlink 盘上文件，`_current_checkpoint` 原样留着，
于是 `GET /api/augment/progress`（读内存态）在删除成功后仍报同一个任务；
若任务还在跑，`update_progress` 还会把已删的断点按批写回盘上。
"""

from augmentor.checkpoint import CheckpointManager


def make_manager(tmp_path, **kwargs) -> CheckpointManager:
    return CheckpointManager(checkpoint_dir=str(tmp_path), **kwargs)


class TestDeleteCheckpoint:
    def test_delete_current_clears_in_memory_progress(self, tmp_path):
        manager = make_manager(tmp_path)
        manager.create_checkpoint("task-1", 5)
        manager.update_progress(0, True)

        assert manager.get_progress()["task_id"] == "task-1"

        manager.delete_checkpoint("task-1")

        # 盘上与内存两处都必须清掉：进度接口是内存态读者。
        assert list(tmp_path.glob("task-1_*")) == []
        assert manager.get_progress() == {"status": "no_checkpoint"}
        assert manager.list_checkpoints() == []

    def test_delete_other_task_keeps_current_progress(self, tmp_path):
        manager = make_manager(tmp_path)
        manager.create_checkpoint("task-1", 5)
        manager.create_checkpoint("task-2", 5)

        manager.delete_checkpoint("task-1")

        assert manager.get_progress()["task_id"] == "task-2"
        assert manager.list_checkpoints() == ["task-2"]

    def test_update_after_delete_does_not_recreate_checkpoint(self, tmp_path):
        """删除后残留的批量进度不能把断点文件写回来（删除形同虚设的那一半）。"""
        manager = make_manager(tmp_path)
        manager.create_checkpoint("task-1", 5)

        manager.delete_checkpoint("task-1")
        manager.update_progress(1, True)

        assert list(tmp_path.glob("task-1_*")) == []
        assert manager.list_checkpoints() == []

    def test_delete_removes_delta_files(self, tmp_path):
        manager = make_manager(tmp_path)
        manager.create_checkpoint("task-1", 5)
        manager.update_progress(0, True)
        manager._save_delta()
        assert (tmp_path / "task-1_delta.jsonl").exists()

        manager.delete_checkpoint("task-1")
        assert (tmp_path / "task-1_delta.jsonl").exists() is False
