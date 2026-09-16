"""数据清洗单元测试

清洗会直接改写训练语料，去噪、标准化与语言判定必须精确可控。
"""

import pytest

from augmentor.data import DataCleaner, CleanResult


class TestRemoveNoise:
    """去噪"""

    def test_removes_html_tags(self):
        """HTML 标签属于噪声，会污染模型输入"""
        cleaner = DataCleaner()
        assert cleaner.remove_noise("<p>你好</p>") == "你好"

    def test_removes_urls_by_default(self):
        """URL 默认移除"""
        cleaner = DataCleaner()
        assert "http" not in cleaner.remove_noise("访问 https://example.com 查看")

    def test_keeps_urls_when_disabled(self):
        """显式关闭时应保留 URL，供需要链接的场景使用"""
        cleaner = DataCleaner(remove_urls=False)
        assert "https://example.com" in cleaner.remove_noise("访问 https://example.com")

    def test_removes_zero_width_chars(self):
        """零宽字符不可见却会影响分词，必须清除"""
        cleaner = DataCleaner()
        assert cleaner.remove_noise("你\u200b好\ufeff") == "你好"

    def test_unescapes_html_entities(self):
        """HTML 实体应被还原为真实字符"""
        cleaner = DataCleaner()
        assert cleaner.remove_noise("a &amp; b") == "a & b"

    def test_collapses_whitespace(self):
        """连续空白应折叠为单个空格"""
        cleaner = DataCleaner()
        assert cleaner.remove_noise("你好    世界") == "你好 世界"

    def test_non_string_returns_empty(self):
        """非字符串输入应安全返回空串，避免上层崩溃"""
        cleaner = DataCleaner()
        assert cleaner.remove_noise(None) == ""


class TestNormalizeFormat:
    """格式标准化"""

    def test_fullwidth_punctuation_to_halfwidth(self):
        """全角标点统一为半角，减少词表碎片"""
        cleaner = DataCleaner()
        assert cleaner.normalize_format("你好，世界。") == "你好,世界."

    def test_keeps_fullwidth_parens_converted(self):
        """全角括号同样应被转换"""
        cleaner = DataCleaner()
        assert cleaner.normalize_format("（测试）") == "(测试)"

    def test_collapses_excess_newlines(self):
        """连续空行应被压缩，避免无效 token"""
        cleaner = DataCleaner()
        assert cleaner.normalize_format("第一段\n\n\n\n第二段") == "第一段\n\n第二段"


class TestDetectLanguage:
    """语言检测"""

    @pytest.mark.parametrize("text,expected", [
        ("这是一个中文句子", "zh"),
        ("this is an english sentence", "en"),
        ("你好 hello world 混合", "mixed"),
    ])
    def test_detects_language(self, text, expected):
        """中/英/混合三种情况必须区分，供多语言流程分流"""
        assert DataCleaner().detect_language(text) == expected

    def test_empty_returns_unknown(self):
        """空文本无法判断语言"""
        assert DataCleaner().detect_language("") == "unknown"

    def test_pure_punctuation_returns_unknown(self):
        """纯标点不含语言特征"""
        assert DataCleaner().detect_language("!!!???") == "unknown"


class TestCleanBatch:
    """批量清洗"""

    def test_cleans_and_preserves_other_fields(self):
        """清洗只改目标字段，其他字段必须原样保留"""
        cleaner = DataCleaner()
        items = [{"instruction": "<b>问题</b>", "output": "回答", "id": 1}]
        result = cleaner.clean(items)

        assert result.items[0]["instruction"] == "问题"
        assert result.items[0]["output"] == "回答"
        assert result.items[0]["id"] == 1

    def test_drops_empty_after_cleaning(self):
        """清洗后为空的样本应被丢弃，避免空样本进入训练集"""
        cleaner = DataCleaner(drop_empty=True)
        items = [{"instruction": "<p></p>"}, {"instruction": "有效内容"}]
        result = cleaner.clean(items)

        assert result.dropped_count == 1
        assert len(result.items) == 1

    def test_keeps_empty_when_disabled(self):
        """关闭丢弃开关时空样本应被保留"""
        cleaner = DataCleaner(drop_empty=False)
        result = cleaner.clean([{"instruction": "<p></p>"}])

        assert result.dropped_count == 0
        assert len(result.items) == 1

    def test_reports_changed_count(self):
        """发生变更的样本数需被统计，便于评估清洗强度"""
        cleaner = DataCleaner()
        result = cleaner.clean([
            {"instruction": "<b>需要清洗</b>"},
            {"instruction": "无需清洗"},
        ])

        assert result.changed_count == 1

    def test_empty_dataset(self):
        """空数据集不应抛异常"""
        result = DataCleaner().clean([])
        assert isinstance(result, CleanResult)
        assert result.original_count == 0
        assert result.items == []

    def test_language_distribution(self):
        """语言分布统计需覆盖所有输入样本"""
        cleaner = DataCleaner()
        result = cleaner.clean([
            {"instruction": "中文内容"},
            {"instruction": "english content"},
        ])

        assert sum(result.language_distribution.values()) == 2

    def test_report_fields(self):
        """报告需包含丢弃比例等关键指标"""
        report = DataCleaner().generate_report([{"instruction": "<p></p>"}])

        assert report["original_count"] == 1
        assert report["drop_rate"] == 1.0
        assert "issues" in report
