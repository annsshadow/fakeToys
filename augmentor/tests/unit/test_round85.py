# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""第85轮: backup备份管理"""
import pytest
from augmentor.backup import DatasetBackup


class TestBackup:
    def test_create(self, tmp_path):
        # 默认 `.backups` 会在构造期落进工作目录根（仓库残留守卫会判红）；测试给临时目录
        b = DatasetBackup(backup_dir=str(tmp_path / ".backups"))
        assert b is not None

    def test_has_core_methods(self, tmp_path):
        b = DatasetBackup(backup_dir=str(tmp_path / ".backups"))
        assert hasattr(b, 'backup')
        assert hasattr(b, 'restore')
        assert hasattr(b, 'list_backups')
        assert hasattr(b, 'delete_backup')
        assert hasattr(b, 'get_backup_info')
