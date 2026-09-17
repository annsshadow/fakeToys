"""数据清洗模块测试"""

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
