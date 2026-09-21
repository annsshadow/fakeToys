"""第54轮: converter格式检测"""
import pytest
from pathlib import Path
from augmentor.converter import DatasetConverter


class TestConverterFormatDetection:
    def test_infer_format_json(self):
        conv = DatasetConverter()
        result = conv._infer_format(Path("test.json"))
        assert result == "json"

    def test_infer_format_csv(self):
        conv = DatasetConverter()
        result = conv._infer_format(Path("data.csv"))
        assert result == "csv"

    def test_has_converters(self):
        conv = DatasetConverter()
        assert hasattr(conv, '_converters')
        assert isinstance(conv._converters, dict)
        assert len(conv._converters) > 0
