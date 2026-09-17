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
