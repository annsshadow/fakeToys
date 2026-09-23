# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""数据清洗模块测试"""

import json

import pytest
from augmentor.cleaner import (
    DatasetCleaner, TextNormalizer, CleaningRule, CleaningResult,
    clean_dataset, normalize_text, extract_keywords
)


@pytest.fixture
def sample_dataset():
    """创建测试数据集"""
    return [
        {"instruction": "如何申请租房？", "input": "", "output": "请登录官网申请"},
        {"instruction": "租房需要什么材料？", "input": "", "output": "身份证、工作证明"},
        {"instruction": "租房流程是什么？", "input": "", "output": "选房、签约、付款"},
        {"instruction": "", "input": "", "output": ""},  # 空数据
        {"instruction": "如何申请租房？", "input": "", "output": "请登录官网申请"},  # 重复数据
    ]


@pytest.fixture
def messy_dataset():
    """创建混乱数据集"""
    return [
        {"instruction": "  如何申请租房？  ", "input": "", "output": "请登录官网申请"},
        {"instruction": "如何申请租房？", "input": "", "output": "请登录官网申请"},
        {"instruction": "如何   申请   租房？", "input": "", "output": "请登录官网申请"},
        {"instruction": "", "input": "", "output": ""},  # 空数据
    ]


class TestDatasetCleaner:
    """DatasetCleaner 测试"""
    
    def test_init(self):
        """测试初始化"""
        cleaner = DatasetCleaner()
        assert len(cleaner._rules) == 0
        assert len(cleaner._custom_rules) == 0
    
    def test_clean(self, sample_dataset):
        """测试清洗数据"""
        cleaner = DatasetCleaner()
        cleaned, result = cleaner.clean(sample_dataset)
        
        assert isinstance(cleaned, list)
        assert isinstance(result, CleaningResult)
        assert result.original_count == 5
    
    def test_remove_empty(self, messy_dataset):
        """测试移除空数据"""
        cleaner = DatasetCleaner()
        cleaned, result = cleaner.clean(messy_dataset, rules=["remove_empty"])
        
        assert len(cleaned) < len(messy_dataset)
        assert "remove_empty" in result.rules_applied
    
    def test_remove_duplicates(self, sample_dataset):
        """测试移除重复数据"""
        cleaner = DatasetCleaner()
        cleaned, result = cleaner.clean(sample_dataset, rules=["remove_duplicates"])
        
        assert len(cleaned) < len(sample_dataset)
        assert "remove_duplicates" in result.rules_applied
    
    def test_normalize_whitespace(self, messy_dataset):
        """测试标准化空白字符"""
        cleaner = DatasetCleaner()
        cleaned, result = cleaner.clean(messy_dataset, rules=["normalize_whitespace"])
        
        assert "normalize_whitespace" in result.rules_applied
    
    def test_trim_whitespace(self, messy_dataset):
        """测试去除首尾空白"""
        cleaner = DatasetCleaner()
        cleaned, result = cleaner.clean(messy_dataset, rules=["trim_whitespace"])
        
        assert "trim_whitespace" in result.rules_applied
    
    def test_add_rule(self):
        """测试添加规则"""
        cleaner = DatasetCleaner()
        rule = CleaningRule(
            name="test_rule",
            description="测试规则",
            enabled=True,
            priority=0
        )
        cleaner.add_rule(rule)
        
        assert len(cleaner._rules) == 1
    
    def test_add_custom_rule(self):
        """测试添加自定义规则"""
        cleaner = DatasetCleaner()
        
        def custom_rule(items):
            return items, True
        
        cleaner.add_custom_rule(custom_rule)
        
        assert len(cleaner._custom_rules) == 1


class TestTextNormalizer:
    """TextNormalizer 测试"""
    
    def test_init(self):
        """测试初始化"""
        normalizer = TextNormalizer()
        assert normalizer is not None
    
    def test_normalize(self):
        """测试标准化文本"""
        normalizer = TextNormalizer()
        
        assert normalizer.normalize("  hello  ") == "hello"
        assert normalizer.normalize("hello   world") == "hello world"
        assert normalizer.normalize("") == ""
    
    def test_extract_keywords(self):
        """测试提取关键词"""
        normalizer = TextNormalizer()
        text = "如何申请租房？租房需要什么材料？租房流程是什么？"
        keywords = normalizer.extract_keywords(text, top_k=3)
        
        assert isinstance(keywords, list)
        assert len(keywords) <= 3


class TestConvenienceFunctions:
    """便捷函数测试"""
    
    def test_clean_dataset(self, sample_dataset):
        """测试清洗数据集"""
        cleaned, result = clean_dataset(sample_dataset)
        
        assert isinstance(cleaned, list)
        assert isinstance(result, CleaningResult)
    
    def test_normalize_text(self):
        """测试标准化文本"""
        result = normalize_text("  hello  ")
        assert result == "hello"
    
    def test_extract_keywords(self):
        """测试提取关键词"""
        text = "如何申请租房？租房需要什么材料？"
        keywords = extract_keywords(text, top_k=3)
        assert isinstance(keywords, list)


class TestCleanerExtended:
    """DatasetCleaner 扩展测试"""

    def test_clean_empty_dataset(self):
        """清洗空数据集"""
        cleaner = DatasetCleaner()
        cleaned, result = cleaner.clean([])
        assert cleaned == []
        assert result.original_count == 0

    def test_normalize_mixed_whitespace_and_punct(self):
        """normalize 同时压缩空白与合并重复标点"""
        normalizer = TextNormalizer()
        assert normalizer.normalize("  你好   世界！！！  ") == "你好 世界！"

    def test_normalize_fullwidth_punctuation(self):
        """全角标点应被保留并合并"""
        normalizer = TextNormalizer()
        assert normalizer.normalize("问题？？？") == "问题？"

    def test_extract_keywords_respects_top_k(self):
        """extract_keywords 严格限制返回数量"""
        normalizer = TextNormalizer()
        keywords = normalizer.extract_keywords(
            "租房 押金 合同 维修 投诉", top_k=2
        )
        assert len(keywords) <= 2
        assert all(isinstance(k, str) for k in keywords)

    def test_clean_with_all_rules(self, sample_dataset):
        """使用所有规则清洗"""
        cleaner = DatasetCleaner()
        cleaned, result = cleaner.clean(
            sample_dataset,
            rules=["remove_empty", "remove_duplicates", "normalize_whitespace", "trim_whitespace"]
        )
        assert len(cleaned) <= len(sample_dataset)

    def test_cleaning_result_fields(self, sample_dataset):
        """CleaningResult 应包含所有必要字段"""
        cleaner = DatasetCleaner()
        cleaned, result = cleaner.clean(sample_dataset)
        assert hasattr(result, 'original_count')
        assert hasattr(result, 'cleaned_count')
        assert hasattr(result, 'removed_count')
        assert hasattr(result, 'rules_applied')

    def test_text_normalizer_various(self):
        """TextNormalizer 各种输入"""
        normalizer = TextNormalizer()
        assert normalizer.normalize("hello\tworld") == "hello world"
        assert normalizer.normalize("hello\n\nworld") == "hello world"
        assert normalizer.normalize("") == ""
        assert normalizer.normalize("  ") == ""

    def test_extract_keywords_empty(self):
        """空文本提取关键词"""
        keywords = extract_keywords("", top_k=5)
        assert keywords == []

    def test_extract_keywords_short(self):
        """短文本提取关键词"""
        keywords = extract_keywords("租房", top_k=5)
        assert isinstance(keywords, list)


class TestCleaningResult:
    """CleaningResult 测试"""
    
    def test_to_dict(self):
        """测试转换为字典"""
        result = CleaningResult(
            original_count=10,
            cleaned_count=8,
            removed_count=2,
            modified_count=1,
            rules_applied=["remove_empty", "remove_duplicates"]
        )
        
        d = result.to_dict()
        
        assert d["original_count"] == 10
        assert d["cleaned_count"] == 8
        assert d["removed_count"] == 2
        assert d["modified_count"] == 1
        assert len(d["rules_applied"]) == 2

    def test_to_dict_empty_result(self):
        """空结果字典"""
        result = CleaningResult(
            original_count=0, cleaned_count=0, removed_count=0,
            modified_count=0, rules_applied=[]
        )
        d = result.to_dict()
        assert d["original_count"] == 0
        assert d["cleaned_count"] == 0


class TestCleanerExtended:
    """数据清洗扩展测试"""

    def test_text_normalizer_empty_string(self):
        """空字符串归一化"""
        normalizer = TextNormalizer()
        result = normalizer.normalize("")
        assert result == ""

    def test_text_normalizer_whitespace_only(self):
        """仅空白字符归一化"""
        normalizer = TextNormalizer()
        result = normalizer.normalize("   \t\n  ")
        assert result == ""

    def test_extract_keywords_empty(self):
        """空文本提取关键词"""
        result = extract_keywords("")
        assert result == []

    def test_clean_empty_dataset(self):
        """清洗空数据集"""
        cleaner = DatasetCleaner()
        cleaned, result = cleaner.clean([])
        assert cleaned == []
        assert result.original_count == 0


class TestNoiseRemovalRules:
    """噪声清除三条规则（`remove_urls` / `remove_html_tags` / `remove_control_chars`）

    这三条是从 `data.cleaner.DataCleaner` 并进规则式实现的。合并 CLI 的 `clean`
    命令时若漏掉它们，默认清洗行为就会静默变化（URL / HTML 标签留在数据里）。
    """

    ITEMS = [
        {"instruction": "见 https://example.com/a?b=1 与 www.foo.cn", "output": "ok"},
        {"instruction": "正文 <b>加粗</b> 结束", "output": "<p>段落</p>"},
        {"instruction": "零宽\u200b空格与 BOM\ufeff", "output": "ok"},
    ]

    def _clean(self, rules, items=None):
        cleaner = DatasetCleaner()
        return cleaner.clean(items if items is not None else self.ITEMS, rules=rules)

    def test_remove_urls(self):
        cleaned, result = self._clean(["remove_urls"])
        assert "remove_urls" in result.rules_applied
        joined = " ".join(i["instruction"] for i in cleaned)
        assert "example.com" not in joined
        assert "www.foo.cn" not in joined

    def test_remove_html_tags(self):
        cleaned, result = self._clean(["remove_html_tags"])
        assert "remove_html_tags" in result.rules_applied
        joined = " ".join(i["instruction"] + i["output"] for i in cleaned)
        assert "<b>" not in joined and "</b>" not in joined
        assert "<p>" not in joined and "</p>" not in joined
        # 标签内文本应保留，而不是整段删掉
        assert "加粗" in joined and "段落" in joined

    def test_remove_control_chars(self):
        cleaned, result = self._clean(["remove_control_chars"])
        assert "remove_control_chars" in result.rules_applied
        joined = " ".join(i["instruction"] for i in cleaned)
        assert "\u200b" not in joined
        assert "\ufeff" not in joined

    def test_omitting_remove_urls_keeps_url(self):
        """不带 remove_urls 时 URL 原样保留（旧 `--no-url-removal` 的等价写法）"""
        cleaned, result = self._clean(["trim_whitespace"])
        assert "remove_urls" not in result.rules_applied
        assert "https://example.com/a?b=1" in cleaned[0]["instruction"]

    def test_remove_special_chars_is_not_url_removal(self):
        """`remove_special_chars` 不能替代 `remove_urls`

        它删的是「非中文/英文/数字/常用标点」的字符，只会把 URL 削成
        `https:example.com`——`example.com` 仍在。这条测试守住两者的区别，
        避免以后有人用前者去实现后者。
        """
        cleaned, _ = self._clean(["remove_special_chars"])
        instruction = cleaned[0]["instruction"]
        assert "example.com" in instruction
        assert "https://example.com/a?b=1" not in instruction


class TestCleanDoesNotMutateInput:
    """`clean` 不得就地修改调用方传入的数据

    规则是就地改 `item[field]` 的，只做 `items.copy()`（浅拷贝）会把调用方的
    dict 一起改掉：`clean_dataset(items)` 之后 `items` 就不是原数据了。
    这种缺陷很隐蔽——返回值是对的，坏掉的是调用方手上那份。
    """

    def test_clean_dataset_leaves_input_untouched(self):
        items = [{"instruction": "见 http://a.com", "output": "y"}]
        snapshot = json.dumps(items, ensure_ascii=False)

        cleaned, _ = clean_dataset(
            items, rules=["remove_urls", "normalize_whitespace", "trim_whitespace"]
        )

        assert json.dumps(items, ensure_ascii=False) == snapshot, "入参被就地修改"
        assert "a.com" not in cleaned[0]["instruction"], "返回值未被清洗"

    def test_cleaner_clean_leaves_input_untouched(self):
        items = [{"instruction": "  首尾空白  ", "output": "y"}]
        snapshot = json.dumps(items, ensure_ascii=False)

        DatasetCleaner().clean(items, rules=["trim_whitespace"])

        assert json.dumps(items, ensure_ascii=False) == snapshot

    def test_batch_clean_leaves_input_untouched(self):
        from augmentor.cleaner import clean_batch_optimized

        items = [{"instruction": "见 http://a.com", "output": "y"}]
        snapshot = json.dumps(items, ensure_ascii=False)

        clean_batch_optimized(items, rules=["remove_urls"], batch_size=1)

        assert json.dumps(items, ensure_ascii=False) == snapshot
