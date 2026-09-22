# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""迁移模块测试扩展"""
from augmentor.migration import migrate_dataset

def test_migration_exists():
    assert callable(migrate_dataset)
