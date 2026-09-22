# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""索引模块优化测试 - 增强索引类型验证"""
from augmentor.indexer import IndexType

class TestIndexerEnhancement:
    def test_enhanced_index_type_exists(self):
        assert IndexType.ENHANCED.value == "enhanced"
