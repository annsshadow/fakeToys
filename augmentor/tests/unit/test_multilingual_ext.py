"""多语言支持扩展测试 - 语言检测和翻译功能验证"""

import pytest
from augmentor.multilingual import MultilingualSupport, LANG_NAMES, CJK_PATTERN


@pytest.fixture
def multilingual():
    return MultilingualSupport()


class TestMultilingualExtended:
    """多语言扩展测试"""
    
    def test_language_names_available(self):
        """语言名称映射应包含常用语言"""
        assert "zh" in LANG_NAMES
        assert "en" in LANG_NAMES
    
    def test_cjk_pattern_matches_chinese(self):
        """CJK正则应匹配中文字符"""
        assert CJK_PATTERN.search("如何租房") is not None
        assert CJK_PATTERN.search("hello") is None
    
    def test_detect_language(self, multilingual):
        """应能区分中英文——原断言是 `hasattr(...) or True`，恒真且什么都没测"""
        assert multilingual.detect_language("如何租房？") == "zh"
        assert multilingual.detect_language("How to rent a house?") == "en"
