# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""versioning 模块剩余分支测试

覆盖：同微秒版本 ID 去重后缀、历史写入失败降级、历史空行跳过、
current 为真实目录时的 rmtree、Windows 符号链接不可用时的 .txt 降级。
"""

import datetime as dt
import json
from pathlib import Path

import pytest

from augmentor.versioning import VersionManager

ITEMS = [
    {"instruction": "如何申请？", "input": "", "output": "登录官网"},
    {"instruction": "多少钱？", "input": "", "output": "按月计"},
]


@pytest.fixture
def manager(tmp_path):
    return VersionManager(storage_dir=str(tmp_path / "versions"))


class TestVersionIdDedup:
    def test_same_timestamp_gets_suffix(self, manager, monkeypatch):
        """固定时间戳时连续创建需产生 _1 后缀避免覆盖"""
        fixed = dt.datetime(2026, 9, 18, 12, 0, 0, 123456)

        class _FrozenDateTime(dt.datetime):
            @classmethod
            def now(cls, tz=None):
                return fixed

        import augmentor.versioning as versioning
        monkeypatch.setattr(versioning, "datetime", _FrozenDateTime)

        v1 = manager.create_version(ITEMS)
        v2 = manager.create_version(ITEMS)
        assert v1.version_id != v2.version_id
        assert (manager.storage_dir / v2.version_id).exists()


class TestHistoryRobustness:
    def test_missing_parent_logs_warning_not_raise(self, manager):
        """历史文件父目录缺失时写历史需降级为告警而非抛出"""
        manager._history_path = Path(manager.storage_dir) / "gone" / "history.jsonl"
        # create_version 不应因历史写入失败而崩溃
        version = manager.create_version(ITEMS)
        assert version.version_id

    def test_blank_and_corrupt_lines_skipped(self, manager):
        """空行与损坏行需被安全跳过，合法行保留"""
        lines = [
            json.dumps({"action": "create", "version_id": "v1"}),
            "",
            "not json at all",
            json.dumps({"action": "rollback", "version_id": "v2"}),
        ]
        manager._history_path.write_text("\n".join(lines), encoding="utf-8")
        entries = manager.get_history()
        version_ids = [e["version_id"] for e in entries]
        assert version_ids == ["v2", "v1"]


class TestCurrentDirectoryHandling:
    def test_current_as_real_dir_removed_on_create(self, manager):
        """current 若为真实目录（非链接）需被 rmtree 清理而非 unlink"""
        manager._current_symlink.mkdir()  # 真实目录
        manager.create_version(ITEMS)
        # 创建后 current 不再是之前那个空目录
        assert manager._current_symlink.is_symlink() or (
            manager._current_symlink.with_suffix(".txt").exists()
            or manager.get_current_version() is not None
        )


class TestSymlinkFallback:
    def test_windows_txt_fallback_get_current(self, manager, monkeypatch):
        """符号链接不可用时用 .txt 记录当前版本并可被读回"""
        import augmentor.versioning as versioning

        # 强制所有 symlink_to 失败，模拟 Windows 无权限场景
        def _deny_symlink(self, target, **kwargs):
            raise OSError("符号链接不可用")

        monkeypatch.setattr(Path, "symlink_to", _deny_symlink)
        version = manager.create_version(ITEMS)
        txt = manager._current_symlink.with_suffix(".txt")
        assert txt.exists()
        assert manager.get_current_version() == version.version_id

    def test_rollback_txt_fallback(self, manager, monkeypatch):
        """rollback 在符号链接不可用时同样走 .txt 降级"""
        def _deny_symlink(self, target, **kwargs):
            raise OSError("符号链接不可用")

        monkeypatch.setattr(Path, "symlink_to", _deny_symlink)
        version = manager.create_version(ITEMS)
        ok = manager.rollback(version.version_id)
        assert ok is True
        assert manager.get_current_version() == version.version_id

    def test_rollback_missing_version_returns_false(self, manager):
        assert manager.rollback("v_does_not_exist") is False
