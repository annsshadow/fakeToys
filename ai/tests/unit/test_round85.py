"""第85轮: backup备份管理"""
import pytest
from augmentor.backup import DatasetBackup


class TestBackup:
    def test_create(self):
        b = DatasetBackup()
        assert b is not None

    def test_has_core_methods(self):
        b = DatasetBackup()
        assert hasattr(b, 'backup')
        assert hasattr(b, 'restore')
        assert hasattr(b, 'list_backups')
        assert hasattr(b, 'delete_backup')
        assert hasattr(b, 'get_backup_info')
