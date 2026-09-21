"""第84轮: migration数据迁移"""
import pytest
from augmentor.migration import DatasetMigrator


class TestMigration:
    def test_create(self):
        m = DatasetMigrator()
        assert m is not None

    def test_has_core_methods(self):
        m = DatasetMigrator()
        assert hasattr(m, 'migrate')
