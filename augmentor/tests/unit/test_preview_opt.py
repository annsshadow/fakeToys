# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""预览生成优化测试 - 预览缓存复用验证"""

import pytest
from augmentor.preview import PreviewGenerator, ExportFormat


@pytest.fixture
def generator():
    return PreviewGenerator(preview_size=2)


@pytest.fixture
def sample_items():
    return [
        {"instruction": "如何租房？", "output": "建议查看房源信息"},
        {"instruction": "租房合同注意什么？", "output": "注意条款"},
        {"instruction": "租金如何支付？", "output": "按月支付"},
    ]


class TestPreviewOptimization:
    """预览优化测试"""
    
    def test_preview_cache_exists_after_first_call(self, generator, sample_items):
        """首次预览后应初始化缓存"""
        generator.preview(sample_items, format="jsonl")
        assert hasattr(generator, '_preview_cache')
        assert isinstance(generator._preview_cache, dict)
    
    def test_preview_result_has_format_info(self, generator, sample_items):
        """预览结果应包含格式信息"""
        result = generator.preview(sample_items, format="jsonl")
        assert result.format == "jsonl"
        assert "format_info" in result.to_dict()
        assert result.format_info.get("format") == "jsonl"
    
    def test_preview_warning_for_empty_items(self, generator):
        """空数据预览应包含警告"""
        result = generator.preview([])
        assert any("数据集为空" in w for w in result.warnings)


class TestPreviewUnsupportedFormat:
    def test_non_native_export_format_raises(self):
        """合法 ExportFormat 成员但无原生转换器 → DataValidationError（L103，A204）

        13 个 ExportFormat 成员里只有 6 个有原生转换（NATIVE_FORMATS）；
        传 openai 这类合法但不原生的成员必须得到约定好的 DataValidationError，
        而不是 KeyError。
        """
        from augmentor.exceptions import DataValidationError

        items = [{"instruction": "问", "input": "", "output": "答"}]
        with pytest.raises(DataValidationError, match="不支持的导出格式"):
            PreviewGenerator().preview(items, format="openai")
