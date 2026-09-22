# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

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


class TestValidationExtended:
    """验证扩展测试"""

    def test_validate_file_missing(self, tmp_path):
        """验证不存在的文件"""
        validator = DatasetValidator()
        with pytest.raises(FileNotFoundError):
            validator.validate_file(str(tmp_path / "nonexistent.json"))

    def test_validate_file_invalid_json(self, tmp_path):
        """验证无效 JSON 文件"""
        file_path = tmp_path / "invalid.json"
        file_path.write_text("not json", encoding="utf-8")
        validator = DatasetValidator()
        with pytest.raises(json.JSONDecodeError):
            validator.validate_file(str(file_path))

    def test_sanitize_preserves_valid_data(self):
        """清洗应保留有效数据"""
        data = [{"instruction": "问题", "input": "", "output": "回答"}]
        sanitizer = DataSanitizer()
        result = sanitizer.sanitize(data)
        assert result == data

    def test_remove_duplicates_empty(self):
        """移除空重复列表"""
        sanitizer = DataSanitizer()
        result = sanitizer.remove_duplicates([], keep="first")
        assert result == []

    def test_validation_result_to_dict_complete(self):
        """ValidationResult.to_dict 完整性"""
        from augmentor.validation import ValidationIssue, ValidationSeverity
        result = ValidationResult(
            is_valid=True,
            total_items=10,
            valid_items=8,
            issues=[
                ValidationIssue(field="test", message="msg", severity=ValidationSeverity.ERROR, index=0)
            ]
        )
        d = result.to_dict()
        assert d["is_valid"] is True
        assert d["total_items"] == 10
        assert d["valid_items"] == 8
        assert d["error_count"] == 1

    def test_validate_with_custom_rules(self):
        """自定义规则验证"""
        custom_rules = {"required_fields": ["question", "answer"]}
        validator = DatasetValidator(rules=custom_rules)
        data = [{"question": "q", "answer": "a"}]
        result = validator.validate(data)
        assert result.is_valid is True

    def test_sanitize_control_chars_multiple(self):
        """多种控制字符清洗"""
        data = [{"instruction": "问\x00题\x01", "input": "", "output": "回\x02答"}]
        sanitizer = DataSanitizer()
        result = sanitizer.sanitize(data)
        assert "\x00" not in result[0]["instruction"]
        assert "\x01" not in result[0]["instruction"]
        assert "\x02" not in result[0]["output"]


class TestValidationExtended:
    """DatasetValidator 扩展测试"""

    def test_validate_empty_dataset(self):
        """验证空数据集"""
        validator = DatasetValidator()
        result = validator.validate([])
        assert result.total_items == 0

    def test_validate_single_item(self):
        """验证单条数据"""
        validator = DatasetValidator()
        data = [{"instruction": "q1", "input": "", "output": "a1"}]
        result = validator.validate(data)
        assert result.is_valid is True

    def test_validate_file_missing(self, tmp_path):
        """验证缺失文件"""
        validator = DatasetValidator()
        with pytest.raises(FileNotFoundError):
            validator.validate_file(str(tmp_path / "missing.json"))

    def test_validate_file_invalid_json(self, tmp_path):
        """验证无效 JSON 文件"""
        file_path = tmp_path / "invalid.json"
        file_path.write_text("not json", encoding='utf-8')
        validator = DatasetValidator()
        with pytest.raises(json.JSONDecodeError):
            validator.validate_file(str(file_path))

    def test_sanitize_empty(self):
        """清洗空数据"""
        sanitizer = DataSanitizer()
        result = sanitizer.sanitize([])
        assert result == []

    def test_sanitize_remove_empty_fields(self):
        """清洗移除空字段"""
        data = [{"instruction": "", "input": "", "output": ""}]
        sanitizer = DataSanitizer()
        result = sanitizer.sanitize(data)
        assert len(result) == 0

    def test_validation_result_warning_count(self):
        """ValidationResult 警告计数"""
        from augmentor.validation import ValidationIssue, ValidationSeverity
        result = ValidationResult(
            is_valid=True, total_items=10, valid_items=10,
            issues=[
                ValidationIssue(field="f", message="m", severity=ValidationSeverity.WARNING, index=0)
            ]
        )
        assert result.warning_count == 1

    def test_remove_duplicates_keep_last(self):
        """移除重复保留最后"""
        sanitizer = DataSanitizer()
        data = [{"instruction": "q1"}, {"instruction": "q1"}]
        result = sanitizer.remove_duplicates(data, keep="last")
        assert len(result) == 1


class TestValidationHistoryFormat:
    """历史记录格式验证测试"""

    def test_validate_history_not_list(self):
        """history 不是列表"""
        validator = DatasetValidator(rules={"check_history_format": True})
        data = [{"instruction": "q1", "history": "not a list"}]
        result = validator.validate(data)
        assert result.error_count > 0

    def test_validate_history_item_not_dict(self):
        """history 项不是字典"""
        validator = DatasetValidator(rules={"check_history_format": True})
        data = [{"instruction": "q1", "history": ["not a dict"]}]
        result = validator.validate(data)
        assert result.error_count > 0

    def test_validate_history_missing_role(self):
        """history 项缺少 role"""
        validator = DatasetValidator(rules={"check_history_format": True})
        data = [{"instruction": "q1", "history": [{"content": "answer"}]}]
        result = validator.validate(data)
        assert result.warning_count > 0

    def test_validate_history_missing_content(self):
        """history 项缺少 content"""
        validator = DatasetValidator(rules={"check_history_format": True})
        data = [{"instruction": "q1", "history": [{"role": "user"}]}]
        result = validator.validate(data)
        assert result.warning_count > 0

    def test_validate_history_valid(self):
        """有效的 history 格式"""
        validator = DatasetValidator(rules={"check_history_format": True})
        data = [{"instruction": "q1", "history": [{"role": "user", "content": "q"}, {"role": "assistant", "content": "a"}]}]
        result = validator.validate(data)
        assert result.error_count == 0

    def test_validate_forbidden_patterns(self):
        """禁止模式验证"""
        validator = DatasetValidator(rules={"forbidden_patterns": ["test_pattern"]})
        data = [{"instruction": "test_pattern found"}]
        result = validator.validate(data)
        assert result.error_count > 0


class TestValidationExtended:
    """DatasetValidator 第二轮扩展测试（覆盖剩余分支）"""

    def test_unknown_preset_falls_back_to_basic(self):
        """未知 preset 应回退到 basic"""
        validator = DatasetValidator(preset="nonexistent_preset")
        assert validator.rules == DatasetValidator.PRESET_RULES["basic"]

    def test_validate_non_dict_item(self):
        """非字典数据项应产生 root 错误"""
        validator = DatasetValidator()
        result = validator.validate(["not-a-dict"])
        assert result.error_count > 0
        assert any("root" in str(i.field) for i in result.issues if i.field)

    def test_validate_non_string_required_field(self):
        """必填字段非字符串应产生错误"""
        validator = DatasetValidator()
        result = validator.validate([{"instruction": 123, "output": "a"}])
        assert result.error_count > 0

    def test_validate_empty_required_field_warning(self):
        """空必填字段应产生警告而非错误"""
        validator = DatasetValidator()
        result = validator.validate([{"instruction": "   ", "output": "a"}])
        assert result.warning_count > 0
        assert result.error_count == 0

    def test_validate_min_length_violation(self):
        """低于最小长度应产生警告"""
        validator = DatasetValidator(rules={"min_instruction_length": 10})
        result = validator.validate([{"instruction": "短", "output": "a" * 10}])
        assert any("长度" in i.message for i in result.issues)

    def test_validate_max_length_violation(self):
        """超过最大长度应产生警告"""
        validator = DatasetValidator(rules={"max_instruction_length": 3})
        result = validator.validate([{"instruction": "这段文本超过了长度限制", "output": "a"}])
        assert any("长度" in i.message for i in result.issues)

    def test_validate_missing_required_field(self):
        """缺少必填字段应产生错误"""
        validator = DatasetValidator()
        result = validator.validate([{"output": "a"}])
        assert result.error_count > 0

    def test_validate_all_valid_dataset(self):
        """全部有效数据集应通过"""
        validator = DatasetValidator()
        result = validator.validate([
            {"instruction": "如何申请租房？", "output": "登录官网申请"}
        ])
        assert result.is_valid is True
        assert result.error_count == 0

    def test_validate_result_fields(self):
        """ValidationResult 应包含全部统计字段"""
        validator = DatasetValidator()
        result = validator.validate([{"instruction": "q", "output": "a"}])
        assert result.total_items == 1
        assert result.valid_items == 1
        assert "issues" in result.__dict__ or hasattr(result, "issues")

    def test_validate_file_preset_strict(self, valid_dataset, tmp_path):
        """strict 预设验证文件"""
        validator = DatasetValidator(preset="strict")
        from augmentor.validation import ValidationResult
        result = validator.validate(valid_dataset)
        assert isinstance(result, ValidationResult)

    def test_validate_chat_preset(self, valid_dataset):
        """chat 预设验证"""
        validator = DatasetValidator(preset="chat")
        result = validator.validate(valid_dataset)
        assert result.total_items == 3
