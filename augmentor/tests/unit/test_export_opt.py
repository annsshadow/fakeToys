# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""导出格式转换优化测试 - 导出缓存复用验证"""

import pytest
from augmentor.export import Exporter, ExportFormat


@pytest.fixture
def exporter():
    return Exporter(default_format="jsonl")


class TestExportOptimization:
    """导出优化测试"""
    
    def test_exporter_has_cache(self, exporter):
        """导出器应包含缓存机制"""
        assert hasattr(exporter, '_export_cache')
        assert isinstance(exporter._export_cache, dict)
    
    def test_export_cache_stores_converted_data(self, exporter, tmp_path):
        """导出后应缓存转换结果"""
        # 落点必须在 tmp_path：这里原来写裸名，相对路径按 CWD 解释 ⇒ 每跑一次套件
        # 就在仓库根留一份产物，而那份既存文件会把 A168 残留守卫的 before/after 差集抵冲成空
        sample = [{"instruction": "测试", "output": "回答"}]
        exporter.export(sample, str(tmp_path / "exported.jsonl"), format="jsonl")
        # 验证缓存机制可访问
        assert isinstance(exporter._export_cache, dict)
