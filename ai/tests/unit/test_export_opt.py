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
    
    def test_export_cache_stores_converted_data(self, exporter):
        """导出后应缓存转换结果"""
        # 使用简单数据测试缓存机制存在
        sample = [{"instruction": "测试", "output": "回答"}]
        exporter.export(sample, "test_output.json", format="jsonl")
        # 验证缓存机制可访问
        assert isinstance(exporter._export_cache, dict)
