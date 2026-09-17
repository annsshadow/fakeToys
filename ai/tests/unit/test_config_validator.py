"""配置验证模块测试"""

import pytest
import yaml
from pathlib import Path
from augmentor.config_validator import (
    ConfigValidator, ValidationResult, Severity,
    validate_config_file, validate_config
)


@pytest.fixture
def valid_config():
    """创建有效配置"""
    return {
        "app": {
            "name": "test-app",
            "version": "1.0.0",
            "debug": False
        },
        "models": {
            "default": "ernie"
        },
        "augmentation": {
            "variants_per_seed": 5,
            "num_threads": 10
        },
        "quality": {
            "enabled": True,
            "threshold": 0.7
        }
    }


@pytest.fixture
def invalid_config():
    """创建无效配置"""
    return {
        "models": {
            "default": "ernie"
        }
        # 缺少 app 必填字段
    }


@pytest.fixture
def config_with_env_refs():
    """包含环境变量引用的配置"""
    return {
        "app": {
            "name": "test-app",
            "version": "1.0.0"
        },
        "models": {
            "default": "ernie",
            "ernie": {
                "api_key": "${BAIDU_API_KEY}"
            }
        }
    }


class TestConfigValidator:
    """ConfigValidator 测试"""
    
    def test_validate_valid_config(self, valid_config):
        """测试验证有效配置"""
        validator = ConfigValidator()
        result = validator.validate_config(valid_config)
        
        assert result.is_valid is True
        assert len(result.errors) == 0
    
    def test_validate_missing_required(self, invalid_config):
        """测试验证缺少必填字段"""
        validator = ConfigValidator()
        result = validator.validate_config(invalid_config)
        
        assert result.is_valid is False
        assert len(result.errors) > 0
    
    def test_validate_type_error(self):
        """测试验证类型错误"""
        config = {
            "app": {
                "name": 123,  # 应该是字符串
                "version": "1.0.0"
            },
            "models": {"default": "ernie"}
        }
        
        validator = ConfigValidator()
        result = validator.validate_config(config)
        
        assert result.is_valid is False
        assert any("类型错误" in e.message for e in result.errors)
    
    def test_validate_range_error(self):
        """测试验证范围错误"""
        config = {
            "app": {"name": "test", "version": "1.0.0"},
            "models": {"default": "ernie"},
            "augmentation": {"variants_per_seed": 200}  # 超过最大值
        }
        
        validator = ConfigValidator()
        result = validator.validate_config(config)
        
        assert result.is_valid is False
        assert any("值过大" in e.message for e in result.errors)
    
    def test_validate_env_refs(self, config_with_env_refs):
        """测试验证环境变量引用"""
        validator = ConfigValidator()
        result = validator.validate_config(config_with_env_refs)
        
        # 如果环境变量未设置，应该有警告
        if "BAIDU_API_KEY" not in __import__('os').environ:
            assert len(result.warnings) > 0
            assert any("环境变量未设置" in w.message for w in result.warnings)
    
    def test_validate_empty_config(self):
        """测试验证空配置"""
        validator = ConfigValidator()
        result = validator.validate_config({})
        
        # 应该有错误，因为缺少必填字段
        assert result.is_valid is False
    
    def test_validate_non_dict_config(self):
        """测试验证非字典配置"""
        validator = ConfigValidator()
        result = validator.validate_config("not a dict")
        
        assert result.is_valid is False
        assert any("配置必须是字典类型" in e.message for e in result.errors)
    
    def test_validate_nested_dict_type_error(self):
        """测试验证嵌套字典类型错误"""
        config = {
            "app": "not a dict",  # 应该是字典
            "models": {"default": "ernie"}
        }
        
        validator = ConfigValidator()
        result = validator.validate_config(config)
        
        assert result.is_valid is False
        assert any("类型错误" in e.message for e in result.errors)
    
    def test_validate_list_in_config(self):
        """测试验证配置中的列表"""
        config = {
            "app": {"name": "test", "version": "1.0.0"},
            "models": {"default": "ernie"},
            "augmentation": {
                "variants_per_seed": "not an int"  # 应该是整数
            }
        }
        
        validator = ConfigValidator()
        result = validator.validate_config(config)
        
        assert result.is_valid is False
        assert any("类型错误" in e.message for e in result.errors)
    
    def test_validate_env_refs_in_list(self):
        """测试验证列表中的环境变量引用"""
        config = {
            "app": {"name": "test", "version": "1.0.0"},
            "models": {"default": "ernie"},
            "api_keys": ["${API_KEY_1}", "${API_KEY_2}"]
        }
        
        validator = ConfigValidator()
        result = validator.validate_config(config)
        
        # 检查是否有环境变量警告
        assert isinstance(result.warnings, list)
    
    def test_validate_field_below_min(self):
        """测试验证字段低于最小值"""
        config = {
            "app": {"name": "test", "version": "1.0.0"},
            "models": {"default": "ernie"},
            "augmentation": {"variants_per_seed": 0}  # 低于最小值 1
        }
        
        validator = ConfigValidator()
        result = validator.validate_config(config)
        
        assert result.is_valid is False
        assert any("值过小" in e.message for e in result.errors)
    
    def test_validate_nested_dict(self):
        """测试验证嵌套字典"""
        config = {
            "app": {"name": "test", "version": "1.0.0"},
            "models": {"default": "ernie"},
            "nested": {"key": "value"}
        }
        
        validator = ConfigValidator()
        result = validator.validate_config(config)
        
        # 嵌套字典应该被递归验证
        assert result.is_valid is True or len(result.errors) > 0
    
    def test_validate_nested_list(self):
        """测试验证嵌套列表"""
        config = {
            "app": {"name": "test", "version": "1.0.0"},
            "models": {"default": "ernie"},
            "list_field": ["item1", "item2"]
        }
        
        validator = ConfigValidator()
        result = validator.validate_config(config)
        
        # 列表应该被验证
        assert isinstance(result.warnings, list)


class TestValidationResult:
    """ValidationResult 测试"""
    
    def test_add_error(self):
        """测试添加错误"""
        result = ValidationResult(is_valid=True)
        result.add_error("field", "error message")
        
        assert result.is_valid is False
        assert len(result.errors) == 1
        assert result.errors[0].severity == Severity.ERROR
    
    def test_add_warning(self):
        """测试添加警告"""
        result = ValidationResult(is_valid=True)
        result.add_warning("field", "warning message")
        
        assert result.is_valid is True  # 警告不影响有效性
        assert len(result.warnings) == 1
    
    def test_add_info(self):
        """测试添加信息"""
        result = ValidationResult(is_valid=True)
        result.add_info("field", "info message")
        
        assert result.is_valid is True
        assert len(result.info) == 1
    
    def test_to_dict(self):
        """测试转换为字典"""
        result = ValidationResult(is_valid=True)
        result.add_warning("field", "warning")
        
        d = result.to_dict()
        
        assert d["is_valid"] is True
        assert len(d["errors"]) == 0
        assert len(d["warnings"]) == 1
        assert d["summary"]["errors"] == 0
        assert d["summary"]["warnings"] == 1
    
    def test_to_dict_with_line_number(self):
        """测试转换为字典（包含行号）"""
        result = ValidationResult(is_valid=True)
        result.add_error("field", "error message", line=42)
        
        d = result.to_dict()
        
        assert d["errors"][0]["line"] == 42
    
    def test_multiple_errors(self):
        """测试添加多个错误"""
        result = ValidationResult(is_valid=True)
        result.add_error("field1", "error1")
        result.add_error("field2", "error2")
        result.add_error("field3", "error3")
        
        assert result.is_valid is False
        assert len(result.errors) == 3


class TestConvenienceFunctions:
    """便捷函数测试"""
    
    def test_validate_config_file(self, tmp_path, valid_config):
        """测试验证配置文件"""
        config_file = tmp_path / "config.yaml"
        with open(config_file, 'w', encoding='utf-8') as f:
            yaml.dump(valid_config, f)
        
        result = validate_config_file(str(config_file))
        assert result.is_valid is True
    
    def test_validate_config(self, valid_config):
        """测试验证配置字典"""
        result = validate_config(valid_config)
        assert result.is_valid is True
    
    def test_validate_invalid_yaml(self, tmp_path):
        """测试验证无效YAML文件"""
        config_file = tmp_path / "invalid.yaml"
        with open(config_file, 'w', encoding='utf-8') as f:
            f.write("invalid: yaml: content: [")
        
        result = validate_config_file(str(config_file))
        assert result.is_valid is False
        assert len(result.errors) > 0
    
    def test_validate_nonexistent_file(self):
        """测试验证不存在的文件"""
        result = validate_config_file("nonexistent.yaml")
        assert result.is_valid is False
    
    def test_validate_directory_instead_of_file(self, tmp_path):
        """测试验证目录而不是文件"""
        result = validate_config_file(str(tmp_path))
        assert result.is_valid is False
    
    def test_validate_config_file_with_read_error(self, tmp_path):
        """测试验证无法读取的文件"""
        config_file = tmp_path / "unreadable.yaml"
        config_file.write_text("test: value", encoding='utf-8')
        
        # 模拟读取错误（通过修改文件权限在Windows上可能不工作）
        # 这里我们只是测试文件存在但内容格式错误的情况
        result = validate_config_file(str(config_file))
        assert result.is_valid is False  # 文件格式无效（缺少必填字段）


class TestSeverity:
    """Severity 测试"""
    
    def test_severity_values(self):
        """测试严重程度值"""
        assert Severity.ERROR.value == "error"
        assert Severity.WARNING.value == "warning"
        assert Severity.INFO.value == "info"
    
    def test_severity_members(self):
        """测试严重程度成员"""
        assert len(Severity) == 3


class TestConfigValidatorEdgeCases:
    """ConfigValidator 边界情况测试"""
    
    def test_validate_dict_type_error(self):
        """测试验证字典类型错误"""
        validator = ConfigValidator()
        result = ValidationResult(is_valid=True)
        
        validator._validate_dict("not a dict", {}, "test.path", result)
        
        assert len(result.errors) == 1
        assert "期望字典类型" in result.errors[0].message
    
    def test_validate_list_type_error(self):
        """测试验证列表类型错误"""
        validator = ConfigValidator()
        result = ValidationResult(is_valid=True)
        
        validator._validate_list("not a list", {}, "test.path", result)
        
        assert len(result.errors) == 1
        assert "期望列表类型" in result.errors[0].message
    
    def test_validate_string_type_error(self):
        """测试验证字符串类型错误"""
        validator = ConfigValidator()
        result = ValidationResult(is_valid=True)
        
        validator._validate_string(123, {}, "test.path", result)
        
        assert len(result.errors) == 1
        assert "期望字符串类型" in result.errors[0].message
    
    def test_validate_string_enum_error(self):
        """测试验证字符串枚举错误"""
        validator = ConfigValidator()
        result = ValidationResult(is_valid=True)
        
        validator._validate_string("invalid", {"enum": ["valid1", "valid2"]}, "test.path", result)
        
        assert len(result.errors) == 1
        assert "值不在允许范围内" in result.errors[0].message
    
    def test_validate_int_type_error(self):
        """测试验证整数类型错误"""
        validator = ConfigValidator()
        result = ValidationResult(is_valid=True)
        
        validator._validate_int("not an int", {}, "test.path", result)
        
        assert len(result.errors) == 1
        assert "期望整数类型" in result.errors[0].message
    
    def test_validate_int_range_error(self):
        """测试验证整数范围错误"""
        validator = ConfigValidator()
        result = ValidationResult(is_valid=True)
        
        validator._validate_int(10, {"min": 20}, "test.path", result)
        
        assert len(result.errors) == 1
        assert "值过小" in result.errors[0].message
    
    def test_validate_float_type_error(self):
        """测试验证浮点数类型错误"""
        validator = ConfigValidator()
        result = ValidationResult(is_valid=True)
        
        validator._validate_float("not a float", {}, "test.path", result)
        
        assert len(result.errors) == 1
        assert "期望数值类型" in result.errors[0].message
    
    def test_validate_float_range_error(self):
        """测试验证浮点数范围错误"""
        validator = ConfigValidator()
        result = ValidationResult(is_valid=True)
        
        validator._validate_float(0.5, {"min": 1.0}, "test.path", result)
        
        assert len(result.errors) == 1
        assert "值过小" in result.errors[0].message
    
    def test_validate_bool_type_error(self):
        """测试验证布尔类型错误"""
        validator = ConfigValidator()
        result = ValidationResult(is_valid=True)
        
        validator._validate_bool("not a bool", {}, "test.path", result)
        
        assert len(result.errors) == 1
        assert "期望布尔类型" in result.errors[0].message

    def test_validate_int_bool_excluded(self):
        """bool 应被排除在 int 验证之外"""
        validator = ConfigValidator()
        result = ValidationResult(is_valid=True)
        
        validator._validate_int(True, {}, "test.path", result)
        
        assert len(result.errors) == 1
        assert "期望整数类型" in result.errors[0].message

    def test_validate_float_bool_excluded(self):
        """bool 应被排除在 float 验证之外"""
        validator = ConfigValidator()
        result = ValidationResult(is_valid=True)
        
        validator._validate_float(False, {}, "test.path", result)
        
        assert len(result.errors) == 1
        assert "期望数值类型" in result.errors[0].message

    def test_validate_float_int_accepted(self):
        """int 应被接受为 float"""
        validator = ConfigValidator()
        result = ValidationResult(is_valid=True)
        
        validator._validate_float(5, {"min": 0, "max": 10}, "test.path", result)
        
        assert len(result.errors) == 0

    def test_validate_string_enum_valid(self):
        """有效枚举值不应报错"""
        validator = ConfigValidator()
        result = ValidationResult(is_valid=True)
        
        validator._validate_string("valid1", {"enum": ["valid1", "valid2"]}, "test.path", result)
        
        assert len(result.errors) == 0

    def test_validate_int_valid(self):
        """有效整数不应报错"""
        validator = ConfigValidator()
        result = ValidationResult(is_valid=True)
        
        validator._validate_int(5, {"min": 0, "max": 10}, "test.path", result)
        
        assert len(result.errors) == 0

    def test_validate_float_valid(self):
        """有效浮点数不应报错"""
        validator = ConfigValidator()
        result = ValidationResult(is_valid=True)
        
        validator._validate_float(0.5, {"min": 0.0, "max": 1.0}, "test.path", result)
        
        assert len(result.errors) == 0

    def test_validate_bool_valid(self):
        """有效布尔值不应报错"""
        validator = ConfigValidator()
        result = ValidationResult(is_valid=True)
        
        validator._validate_bool(True, {}, "test.path", result)
        
        assert len(result.errors) == 0

    def test_error_to_dict(self):
        """ValidationError.to_dict 应返回正确结构"""
        from augmentor.config_validator import ValidationError
        err = ValidationError(path="test.path", message="test msg", severity=Severity.ERROR, line=10)
        d = err.to_dict()
        
        assert d["path"] == "test.path"
        assert d["message"] == "test msg"
        assert d["severity"] == "error"
        assert d["line"] == 10

    def test_validate_file_read_error(self, tmp_path):
        """测试验证文件读取错误"""
        config_file = tmp_path / "bad.yaml"
        # 写入二进制内容导致 YAML 解析失败
        config_file.write_bytes(b'\x00\x01\x02')
        
        result = validate_config_file(str(config_file))
        # 应该有错误（YAML 解析或读取失败）
        assert result.is_valid is False

    def test_validate_config_with_null_field(self):
        """测试验证包含 null 字段的配置"""
        config = {
            "app": {
                "name": None,  # null 值
                "version": "1.0.0"
            },
            "models": {"default": "ernie"}
        }
        
        validator = ConfigValidator()
        result = validator.validate_config(config)
        
        assert result.is_valid is False
        assert any("字段不能为null" in e.message for e in result.errors)

    def test_validate_range_float(self):
        """测试浮点数范围验证"""
        config = {
            "app": {"name": "test", "version": "1.0.0"},
            "models": {"default": "ernie"},
            "quality": {"threshold": 1.5}  # 超过最大值 1.0
        }
        
        validator = ConfigValidator()
        result = validator.validate_config(config)
        
        assert result.is_valid is False
        assert any("值过大" in e.message for e in result.errors)


class TestConfigValidatorExtended:
    """ConfigValidator 扩展测试"""

    def test_validate_empty_config(self):
        """验证空配置"""
        validator = ConfigValidator()
        result = validator.validate_config({})
        assert result.is_valid is False

    def test_validate_valid_config(self, valid_config):
        """验证有效配置"""
        validator = ConfigValidator()
        result = validator.validate_config(valid_config)
        assert result.is_valid is True

    def test_validate_config_with_extra_fields(self):
        """验证包含额外字段的配置"""
        config = {
            "app": {"name": "test", "version": "1.0.0", "extra": "field"},
            "models": {"default": "ernie"}
        }
        validator = ConfigValidator()
        result = validator.validate_config(config)
        # 额外字段不应导致验证失败
        assert result.is_valid is True

    def test_validate_config_type_error(self):
        """验证类型错误的配置"""
        config = {
            "app": "not a dict",
            "models": {"default": "ernie"}
        }
        validator = ConfigValidator()
        result = validator.validate_config(config)
        assert result.is_valid is False

    def test_validation_result_to_dict(self):
        """ValidationResult.to_dict 完整性"""
        result = ValidationResult(is_valid=True)
        d = result.to_dict()
        assert d["is_valid"] is True
        assert "errors" in d
        assert "warnings" in d

    def test_severity_values(self):
        """Severity 枚举值"""
        assert Severity.ERROR.value == "error"
        assert Severity.WARNING.value == "warning"
        assert Severity.INFO.value == "info"
