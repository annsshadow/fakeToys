"""数据清洗模块测试"""

import pytest
from augmentor.data.cleaner import DataCleaner, CleanResult


@pytest.fixture
def sample_dataset():
    """创建测试数据集"""
    return [
        {"instruction": "如何申请租房？", "input": "", "output": "请登录官网申请"},
        {"instruction": "租房需要什么材料？", "input": "", "output": "身份证、工作证明"},
        {"instruction": "租房流程是什么？", "input": "", "output": "选房、签约、付款"},
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


class TestDataCleaner:
    """DataCleaner 测试"""
    
    def test_init(self):
        """测试初始化"""
        cleaner = DataCleaner()
        assert cleaner.text_key == "instruction"
        assert cleaner.remove_urls is True
        assert cleaner.drop_empty is True
        assert cleaner.min_length == 1
    
    def test_init_custom_params(self):
        """测试自定义参数初始化"""
        cleaner = DataCleaner(
            text_key="question",
            remove_urls=False,
            drop_empty=False,
            min_length=5
        )
        assert cleaner.text_key == "question"
        assert cleaner.remove_urls is False
        assert cleaner.drop_empty is False
        assert cleaner.min_length == 5
    
    def test_remove_noise(self):
        """测试去除噪声"""
        cleaner = DataCleaner()
        
        # 测试HTML标签
        assert cleaner.remove_noise("<p>问题</p>") == "问题"
        
        # 测试URL
        assert cleaner.remove_noise("请访问 https://example.com 申请") == "请访问 申请"
        
        # 测试零宽字符
        assert cleaner.remove_noise("问题\u200b") == "问题"
        
        # 测试控制字符
        assert cleaner.remove_noise("问题\x00") == "问题"
        
        # 测试多余空白
        assert cleaner.remove_noise("  问题  ") == "问题"
    
    def test_remove_noise_non_string(self):
        """测试非字符串输入"""
        cleaner = DataCleaner()
        assert cleaner.remove_noise(123) == ""
        assert cleaner.remove_noise(None) == ""
    
    def test_normalize_format(self):
        """测试格式标准化"""
        cleaner = DataCleaner()
        
        # 测试全角标点
        assert cleaner.normalize_format("问题，") == "问题,"
        assert cleaner.normalize_format("问题。") == "问题."
        
        # 测试连续换行
        assert cleaner.normalize_format("问题\n\n\n回答") == "问题\n\n回答"
        
        # 测试连续空白
        assert cleaner.normalize_format("问题   回答") == "问题 回答"
    
    def test_normalize_format_non_string(self):
        """测试非字符串输入"""
        cleaner = DataCleaner()
        assert cleaner.normalize_format(123) == ""
    
    def test_detect_language(self):
        """测试语言检测"""
        cleaner = DataCleaner()
        
        # 测试中文
        assert cleaner.detect_language("如何申请租房") == "zh"
        
        # 测试英文
        assert cleaner.detect_language("How to apply") == "en"
        
        # 测试混合
        assert cleaner.detect_language("如何apply租房") == "mixed"
        
        # 测试空文本
        assert cleaner.detect_language("") == "unknown"
        
        # 测试数字
        assert cleaner.detect_language("12345") == "unknown"
    
    def test_detect_language_non_string(self):
        """测试非字符串输入"""
        cleaner = DataCleaner()
        assert cleaner.detect_language(123) == "unknown"
    
    def test_clean_text(self):
        """测试清洗文本"""
        cleaner = DataCleaner()
        
        text = "  <p>如何申请租房？</p>  "
        cleaned = cleaner.clean_text(text)
        
        # HTML标签被替换为空格，全角？被转半角?，然后strip
        assert "<p>" not in cleaned
        assert "</p>" not in cleaned
        assert "?" in cleaned
    
    def test_clean(self, sample_dataset):
        """测试批量清洗"""
        cleaner = DataCleaner()
        result = cleaner.clean(sample_dataset)
        
        assert isinstance(result, CleanResult)
        assert result.original_count == 3
        assert result.cleaned_count == 3
        assert result.dropped_count == 0
    
    def test_clean_with_empty_data(self, messy_dataset):
        """测试清洗包含空数据的数据集"""
        cleaner = DataCleaner()
        result = cleaner.clean(messy_dataset)
        
        assert result.original_count == 4
        assert result.cleaned_count == 3  # 空数据被丢弃
        assert result.dropped_count == 1
    
    def test_clean_without_drop_empty(self, messy_dataset):
        """测试不丢弃空数据"""
        cleaner = DataCleaner(drop_empty=False)
        result = cleaner.clean(messy_dataset)
        
        assert result.original_count == 4
        assert result.cleaned_count == 4  # 空数据不被丢弃
        assert result.dropped_count == 0
    
    def test_clean_min_length(self):
        """测试最小长度过滤"""
        cleaner = DataCleaner(min_length=5)
        
        items = [
            {"instruction": "短"},
            {"instruction": "这是一个足够长的问题"},
        ]
        result = cleaner.clean(items)
        
        assert result.original_count == 2
        assert result.cleaned_count == 1  # 短文本被丢弃
    
    def test_clean_non_string_instruction(self):
        """测试非字符串instruction"""
        cleaner = DataCleaner()
        
        items = [
            {"instruction": 123},
            {"instruction": None},
        ]
        result = cleaner.clean(items)
        
        assert result.original_count == 2
    
    def test_generate_report(self, sample_dataset):
        """测试生成清洗报告"""
        cleaner = DataCleaner()
        report = cleaner.generate_report(sample_dataset)
        
        assert "original_count" in report
        assert "cleaned_count" in report
        assert "dropped_count" in report
        assert "changed_count" in report
        assert "drop_rate" in report
        assert "language_distribution" in report
        assert "issues" in report
        assert "text_key" in report
    
    def test_generate_report_empty_dataset(self):
        """测试空数据集报告"""
        cleaner = DataCleaner()
        report = cleaner.generate_report([])
        
        assert report["original_count"] == 0
        assert report["drop_rate"] == 0.0


class TestCleanResult:
    """CleanResult 测试"""
    
    def test_default_values(self):
        """测试默认值"""
        result = CleanResult()
        
        assert result.original_count == 0
        assert result.cleaned_count == 0
        assert result.dropped_count == 0
        assert result.changed_count == 0
        assert result.language_distribution == {}
        assert result.issues == {}
        assert result.items == []
