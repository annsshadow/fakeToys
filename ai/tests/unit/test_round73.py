"""第73轮: exporter导出增强"""
import pytest
from augmentor.export import Exporter, ExportFormat


class TestExporter:
    def test_create(self):
        e = Exporter()
        assert e is not None

    def test_export_formats(self):
        assert hasattr(ExportFormat, 'JSONL')
        assert hasattr(ExportFormat, 'CSV')
        assert hasattr(ExportFormat, 'ALPACA')
        assert hasattr(ExportFormat, 'SHARE_GPT')
