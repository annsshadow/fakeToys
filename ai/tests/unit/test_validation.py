"""数据集验证模块测试"""

import json
import pytest
from pathlib import Path
from augmentor.validation import (
    DatasetValidator, DataSanitizer, ValidationResult,
    ValidationSeverity, validate_dataset, sanitize_dataset
)


@pytest.fixture
def valid_dataset():
    """创建有效数据集"""
    return [
        {"instruction": "如何申请租房？", "input": "", "output": "请登录官网申请"},
        {"instruction": "租房需要什么材料？", "input": "", "output": "身份证、工作证明"},
        {"instruction": "租房流程是什么？", "input": "", "output": "选房、签约、付款"},
    ]


@pytest.fixture
def invalid_dataset():
    """创建无效数据集"""
    return [
        {"instruction": "如何申请租房？", "input": "", "output": "请登录官网申请"},
        {"input": "", "output": "缺少instruction字段"},  # 缺少必填字段
        {"instruction": "", "input": "", "output": ""},  # 空字段
        {"instruction": "测试", "input": "", "output": "回答"},
    ]


@pytest.fixture
def temp_dataset_file(tmp_path, valid_dataset):
    """创建临时数据集文件"""
    file_path = tmp_path / "dataset.json"
    with open(file_path, 'w', encoding='utf-8') as f:
        json.dump(valid_dataset, f, ensure_ascii=False)
    return str(file_path)


class TestDatasetValidator:
    """DatasetValidator 测试"""
    
    def test_init_default(self):
        """测试默认初始化"""
        validator = DatasetValidator()
        assert validator.rules == DatasetValidator.PRESET_RULES["basic"]
    
    def test_init_preset(self):
        """测试预设初始化"""
        validator = DatasetValidator(preset="strict")
        assert validator.rules == DatasetValidator.PRESET_RULES["strict"]
    
    def test_init_custom_rules(self):
        """测试自定义规则初始化"""
        custom_rules = {"required_fields": ["question", "answer"]}
        validator = DatasetValidator(rules=custom_rules)
        assert validator.rules == custom_rules
    
    def test_validate_valid_data(self, valid_dataset):
        """测试验证有效数据"""
        validator = DatasetValidator()
        result = validator.validate(valid_dataset)
        
        assert result.is_valid is True
        assert result.total_items == 3
        assert result.valid_items == 3
        assert result.error_count == 0
    
    def test_validate_invalid_data(self, invalid_dataset):
        """测试验证无效数据"""
        validator = DatasetValidator()
        result = validator.validate(invalid_dataset)
        
        assert result.is_valid is False
        assert result.total_items == 4
        assert result.error_count > 0
    
    def test_validate_missing_required_field(self):
        """测试验证缺少必填字段"""
        data = [{"input": "", "output": "回答"}]  # 缺少 instruction
        validator = DatasetValidator()
        result = validator.validate(data)
        
        assert result.is_valid is False
        assert any(i.field == "instruction" for i in result.issues)
    
    def test_validate_empty_field(self):
        """测试验证空字段"""
        data = [{"instruction": "", "input": "", "output": "回答"}]
        validator = DatasetValidator()
        result = validator.validate(data)
        
        assert result.warning_count > 0
    
    def test_validate_strict_mode(self):
        """测试严格模式验证"""
        data = [{"instruction": "短", "input": "", "output": "回答"}]  # 太短
        validator = DatasetValidator(preset="strict")
        result = validator.validate(data)
        
        assert result.warning_count > 0
    
    def test_validate_file(self, temp_dataset_file):
        """测试验证文件"""
        validator = DatasetValidator()
        result = validator.validate_file(temp_dataset_file)
        
        assert result.is_valid is True
        assert result.total_items == 3


class TestDataSanitizer:
    """DataSanitizer 测试"""
    
    def test_sanitize_basic(self):
        """测试基本清洗"""
        data = [
            {"instruction": "  问题  ", "input": "", "output": "  回答  "},
            {"instruction": "问题", "input": "", "output": "回答"},
        ]
        sanitizer = DataSanitizer()
        result = sanitizer.sanitize(data)
        
        assert result[0]["instruction"] == "问题"
        assert result[0]["output"] == "回答"
    
    def test_sanitize_remove_empty(self):
        """测试移除空数据"""
        data = [
            {"instruction": "问题", "input": "", "output": "回答"},
            {"instruction": "", "input": "", "output": ""},  # 空数据
        ]
        sanitizer = DataSanitizer()
        result = sanitizer.sanitize(data)
        
        assert len(result) == 1
    
    def test_sanitize_remove_control_chars(self):
        """测试移除控制字符"""
        data = [
            {"instruction": "问\x00题", "input": "", "output": "回答"},
        ]
        sanitizer = DataSanitizer()
        result = sanitizer.sanitize(data)
        
        assert "\x00" not in result[0]["instruction"]
    
    def test_remove_duplicates(self):
        """测试移除重复项"""
        data = [
            {"instruction": "问题1", "input": "", "output": "回答1"},
            {"instruction": "问题1", "input": "", "output": "回答1重复"},
            {"instruction": "问题2", "input": "", "output": "回答2"},
        ]
        sanitizer = DataSanitizer()
        result = sanitizer.remove_duplicates(data, keep="first")
        
        assert len(result) == 2
    
    def test_remove_duplicates_keep_last(self):
        """测试移除重复项保留最后"""
        data = [
            {"instruction": "问题1", "input": "", "output": "回答1"},
            {"instruction": "问题1", "input": "", "output": "回答1新"},
            {"instruction": "问题2", "input": "", "output": "回答2"},
        ]
        sanitizer = DataSanitizer()
        result = sanitizer.remove_duplicates(data, keep="last")
        
        assert len(result) == 2
        assert result[0]["output"] == "回答1新"


class TestValidationResult:
    """ValidationResult 测试"""
    
    def test_to_dict(self, valid_dataset):
        """测试转换为字典"""
        validator = DatasetValidator()
        result = validator.validate(valid_dataset)
        result_dict = result.to_dict()
        
        assert "is_valid" in result_dict
        assert "total_items" in result_dict
        assert "valid_items" in result_dict
        assert "error_count" in result_dict
        assert "warning_count" in result_dict
        assert "issues" in result_dict


class TestConvenienceFunctions:
    """便捷函数测试"""
    
    def test_validate_dataset(self, temp_dataset_file):
        """测试验证数据集文件"""
        result = validate_dataset(temp_dataset_file)
        
        assert result["is_valid"] is True
        assert result["total_items"] == 3
    
    def test_sanitize_dataset(self, valid_dataset):
        """测试清洗数据集"""
        result = sanitize_dataset(valid_dataset)
        
        assert len(result) == 3
        for item in result:
            assert "instruction" in item
            assert "output" in item


class TestValidationEdgeCases:
    """边界情况测试"""
    
    def test_empty_dataset(self):
        """测试空数据集"""
        validator = DatasetValidator()
        result = validator.validate([])
        
        assert result.is_valid is True
        assert result.total_items == 0
    
    def test_single_item_dataset(self):
        """测试单条数据集"""
        data = [{"instruction": "问题", "input": "", "output": "回答"}]
        validator = DatasetValidator()
        result = validator.validate(data)
        
        assert result.is_valid is True
        assert result.total_items == 1
