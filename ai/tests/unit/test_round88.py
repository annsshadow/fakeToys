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

    def test_get_language_distribution(self):
        m = MultilingualSupport()
        items = [
            {"instruction": "你好"},
            {"instruction": "Hello"},
            {"instruction": "你好"},
        ]
        dist = m.get_language_distribution(items)
        assert dist.get("zh", 0) == 2
        assert dist.get("en", 0) == 1
