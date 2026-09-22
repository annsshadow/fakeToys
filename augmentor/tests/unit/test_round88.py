"""第88轮: multilingual多语言增强"""
import pytest
from augmentor.multilingual import MultilingualSupport


class TestMultilingual:
    def test_create(self):
        m = MultilingualSupport()
        assert m is not None

    def test_detect_language_chinese(self):
        m = MultilingualSupport()
        assert m.detect_language("你好世界") == "zh"

    def test_detect_language_english(self):
        m = MultilingualSupport()
        assert m.detect_language("Hello World") == "en"

    def test_detect_language_unknown(self):
        m = MultilingualSupport()
        assert m.detect_language("") == "unknown"

    def test_analyze_languages(self):
        m = MultilingualSupport()
        items = [
            {"instruction": "你好"},
            {"instruction": "Hello"},
            {"instruction": "你好"},
        ]
        result = m.analyze_languages(items)
        assert isinstance(result, dict)
