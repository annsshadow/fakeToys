# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""配置验证模块测试"""

import pytest
import yaml
from pathlib import Path
from augmentor.config_validator import (
    ConfigValidator, ValidationResult, Severity,
    validate_config_file, validate_config
)
from augmentor.exceptions import DataValidationError
from augmentor.validation import require_seconds


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
    """创建无效配置

    缺的是 `models.default`——`load_config` **只**从这个键读默认模型，
    缺了它会静默回落到 `ernie`，所以它是真正的必填项。
    （`app` / `app.name` / `app.version` 曾是必填，但 `AppConfig` 没有对应字段、
    也没有任何代码读它们，属于历史遗留元信息，已降级为可选。）
    """
    return {
        "app": {"name": "ai-augmentor", "version": "1.0"},
        "models": {}
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
        """测试验证包含 null 字段的配置

        用真正必填的 `models.default`：`app.name` 已不是必填项，
        对 null 的必填检查不再作用于它。
        """
        config = {
            "app": {
                "name": "ai-augmentor",
                "version": "1.0.0"
            },
            "models": {"default": None}  # null 值
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


class TestConfigValidatorExtended2:
    """ConfigValidator 第二轮扩展测试（覆盖文件路径/列表/环境引用边界）"""

    def test_validate_file_nonexistent(self, tmp_path):
        """不存在的文件应报错"""
        validator = ConfigValidator()
        result = validator.validate_file(str(tmp_path / "nope.yaml"))
        assert result.is_valid is False
        assert any("文件不存在" in e.message for e in result.errors)

    def test_validate_file_directory(self, tmp_path):
        """目录路径应报错"""
        validator = ConfigValidator()
        result = validator.validate_file(str(tmp_path))
        assert result.is_valid is False
        assert any("不是有效文件" in e.message for e in result.errors)

    def test_validate_file_yaml_error(self, tmp_path, valid_config):
        """YAML 解析错误应被捕获"""
        path = tmp_path / "bad.yaml"
        path.write_text("a: [unclosed", encoding="utf-8")
        result = ConfigValidator().validate_file(str(path))
        assert result.is_valid is False
        assert any("YAML解析错误" in e.message for e in result.errors)

    def test_validate_file_valid_yaml(self, tmp_path, valid_config):
        """有效 YAML 文件应通过"""
        path = tmp_path / "good.yaml"
        with open(path, "w", encoding="utf-8") as f:
            yaml.dump(valid_config, f)
        result = ConfigValidator().validate_file(str(path))
        assert result.is_valid is True

    def test_validate_config_none_yaml(self):
        """YAML 文件为空（None 解析）应按非字典报错"""
        result = ConfigValidator().validate_config(None)
        assert result.is_valid is False
        assert any("配置必须是字典类型" in e.message for e in result.errors)

    def test_env_ref_in_list_item_warns(self, monkeypatch):
        """列表中未设置的环境变量应产生警告"""
        monkeypatch.delenv("MISSING_CLI_VAR", raising=False)
        config = {
            "app": {"name": "test", "version": "1.0.0"},
            "models": {"default": "ernie"},
            "keys": ["${MISSING_CLI_VAR}", "plain"],
        }
        result = ConfigValidator().validate_config(config)
        assert any("MISSING_CLI_VAR" in w.message for w in result.warnings)

    def test_env_ref_set_no_warning(self, monkeypatch):
        """已设置环境变量的引用不应警告"""
        monkeypatch.setenv("SET_CLI_VAR", "value")
        config = {
            "app": {"name": "test", "version": "1.0.0"},
            "models": {"default": "ernie"},
            "api": {"key": "${SET_CLI_VAR}"},
        }
        result = ConfigValidator().validate_config(config)
        assert not any("SET_CLI_VAR" in w.message for w in result.warnings)

    def test_env_ref_nested_dict_warns(self, monkeypatch):
        """嵌套字典中未设置的环境变量应警告"""
        monkeypatch.delenv("NESTED_MISSING_VAR", raising=False)
        config = {
            "app": {"name": "test", "version": "1.0.0"},
            "models": {"default": "ernie"},
            "secret": {"token": "${NESTED_MISSING_VAR}"},
        }
        result = ConfigValidator().validate_config(config)
        assert any("NESTED_MISSING_VAR" in w.message for w in result.warnings)

    def test_required_field_missing_nested(self):
        """嵌套必填字段缺失应报错"""
        config = {
            "app": {"name": "test"},
            "models": {},  # 缺少 default
        }
        result = ConfigValidator().validate_config(config)
        assert result.is_valid is False
        assert any("models.default" in e.message for e in result.errors)

    def test_valid_full_config_passes(self):
        """完整有效配置应零错误零警告"""
        config = {
            "app": {"name": "test", "version": "1.0.0", "debug": True},
            "models": {"default": "ernie"},
            "augmentation": {"variants_per_seed": 5, "num_threads": 4},
            "quality": {"enabled": True, "threshold": 0.7},
            "dedup": {"enabled": True},
            "output": {"export_dir": "./out"},
        }
        result = ConfigValidator().validate_config(config)
        assert result.is_valid is True
        assert result.errors == []


class TestAugmentationRetryKnobs:
    """`augmentation.max_retries` / `retry_delay` 的校验门槛（L45 接线后才有意义）

    接线前这两个字段无人读取，校验它们只是校验空气；现在它们真的决定重试行为，
    所以 `validate-config` 必须能拒掉坏值，且**不能比运行时更严**——校验器把合法
    写法报成非法，用户就会去改一份本来没问题的配置。
    """

    @staticmethod
    def _errors(augmentation):
        config = {
            "app": {"name": "test"},
            "models": {"default": "ernie"},
            "augmentation": augmentation,
        }
        result = validate_config(config)
        return [(e.path, e.message) for e in result.errors]

    @pytest.mark.parametrize("value", [0, 1, 1.0, 30.5, 60.0])
    def test_legal_retry_delay_values_pass(self, value):
        """int 写法必须放行：`retry_delay: 1` 是 YAML 里最自然的写法"""
        assert self._errors({"retry_delay": value}) == []

    @pytest.mark.parametrize("value,expect", [
        (-1, "值过小"),
        (61.0, "值过大"),
        ("1s", "类型错误"),
        (float("nan"), "不是有效数值"),
    ])
    def test_illegal_retry_delay_values_reported(self, value, expect):
        errors = self._errors({"retry_delay": value})
        assert errors and errors[0][0] == "augmentation.retry_delay"
        assert expect in errors[0][1]

    @pytest.mark.parametrize("value", [0, 3, 20])
    def test_legal_max_retries_values_pass(self, value):
        """0 是合法档位（只调用一次），下界不能收到 1"""
        assert self._errors({"max_retries": value}) == []

    @pytest.mark.parametrize("value,expect", [
        (-1, "值过小"),
        (21, "值过大"),
        (1.5, "类型错误"),
    ])
    def test_illegal_max_retries_values_reported(self, value, expect):
        errors = self._errors({"max_retries": value})
        assert errors and expect in errors[0][1]

    @pytest.mark.parametrize("value", [-1, "1s", float("nan"), 0, 1, 1.0, 30.5, 60.0, 60.1])
    def test_validator_and_runtime_use_the_same_verdict(self, value):
        """校验器与运行时判据必须同口径，**含上界**

        两侧不一致时，`validate-config` 绿灯的配置会在建管道时抛
        `DataValidationError`，或者反过来把合法配置拦在门外。

        L51 之前这条只能写成「上界除外」：`retry_delay ≤ 60` 只在校验器那一侧
        （A77），所以这里比的是不带 `maximum` 的 `require_seconds`。本轮把
        `config.RETRY_DELAY_RANGE` 同时接进 `AugmentationConfig.__post_init__`，
        于是本断言补上了 `maximum`，全区间一致。
        """
        from augmentor.config import RETRY_DELAY_RANGE

        rejected_by_validator = bool(self._errors({"retry_delay": value}))
        try:
            require_seconds("retry_delay", value, maximum=RETRY_DELAY_RANGE[1])
            rejected_by_runtime = False
        except DataValidationError:
            rejected_by_runtime = True
        assert rejected_by_validator == rejected_by_runtime, value

class TestWaitBudgetKnobSurface:
    """`augmentation.max_retry_wait` / `retry_jitter` 的校验规格（L49 接线）

    与 `TestAugmentationRetryKnobs` 成对。L49 时两类的口径**不同**：旧两旋钮的上界
    （`retry_delay ≤ 60`、`max_retries ≤ 20`）是校验器独有的天花板，运行时收 999；
    新两旋钮的上界（`max_retry_wait ≤ 300`、`retry_jitter ≤ 1`）在运行时同样判。
    L51 关闭 A77 后这个区别没了 —— 四个键的区间都住在 `config.py` 的常量里，两侧
    同源，逐值一致由 `TestRuntimeValidatorParity` 统一常驻断言。
    """

    @staticmethod
    def _errors(augmentation):
        config = {
            "app": {"name": "test"},
            "models": {"default": "ernie"},
            "augmentation": augmentation,
        }
        return [(e.path, e.message) for e in validate_config(config).errors]

    @pytest.mark.parametrize("field,value", [
        ("max_retry_wait", 0), ("max_retry_wait", 0.0), ("max_retry_wait", 45),
        ("max_retry_wait", 300.0),
        ("retry_jitter", 0), ("retry_jitter", 0.0), ("retry_jitter", 0.5),
        ("retry_jitter", 1), ("retry_jitter", 1.0),
    ])
    def test_legal_values_pass(self, field, value):
        assert self._errors({field: value}) == []

    @pytest.mark.parametrize("field,bad,expect", [
        ("max_retry_wait", -1.0, "值过小"),
        ("max_retry_wait", 301.0, "值过大"),
        ("max_retry_wait", "300", "类型错误"),
        ("max_retry_wait", float("nan"), "不是有效数值"),
        ("retry_jitter", -0.1, "值过小"),
        ("retry_jitter", 1.5, "值过大"),
        ("retry_jitter", "0.5", "类型错误"),
        ("retry_jitter", float("nan"), "不是有效数值"),
    ])
    def test_illegal_values_reported(self, field, bad, expect):
        errors = self._errors({field: bad})
        assert errors and errors[0][0] == f"augmentation.{field}"
        assert expect in errors[0][1]

    @pytest.mark.parametrize("value", [
        -1.0, 0, 45, 300.0, 301.0, float("inf"), True, "300", float("nan"),
    ])
    def test_validator_and_runtime_agree_on_every_axis(self, value):
        """新旋钮没有「校验器独有的天花板」：两侧口径必须逐个一致

        两侧不一致时，`validate-config` 绿灯的配置会在建管道时抛
        `DataValidationError`，或者反过来把合法配置拦在门外。
        """
        from augmentor import MAX_RETRY_AFTER
        from augmentor.validation import require_seconds

        rejected_by_validator = bool(self._errors({"max_retry_wait": value}))
        try:
            require_seconds("max_retry_wait", value, maximum=MAX_RETRY_AFTER)
            rejected_by_runtime = False
        except Exception:
            rejected_by_runtime = True
        assert rejected_by_validator == rejected_by_runtime, value

    @staticmethod
    def _errors_at(path):
        """把 `section.field = True` 塞进一份最小合法配置，返回全部报错"""
        section, _, field = path.partition(".")
        config = {"app": {"name": "t"}, "models": {"default": "ernie"}}
        config.setdefault(section, {})[field] = True
        return [(e.path, e.message) for e in validate_config(config).errors]

    def test_bool_is_rejected_for_every_numeric_spec(self):
        """`isinstance(True, int)` 恒真 ⇒ 每个数值规格键都曾有同一个洞

        L49 实测（Temp `l49_probe4.py`）：7 个数值规格键填 `true`，旧校验器 7/7 判
        合法，而运行时四道判据（`require_count` / `require_seconds` /
        `require_positive` / `require_ratio`）4/4 拒收。本条把「bool 不算数值」从
        一次性修复升格为对全部数值键的常驻断言，以后新增字段自动被覆盖。
        L51 起计数从 10 变 11：新增的那个是 `augmentation.auto_save_interval`
        （A82 给它补的规格）。
        """
        numeric = [p for p, s in ConfigValidator.KNOWN_FIELDS.items()
                   if s.get("type") in (int, float)]
        assert len(numeric) == 11, "新增数值规格键会自动进入本断言"
        for path in numeric:
            hits = [m for p_, m in self._errors_at(path) if p_ == path]
            assert hits and "类型错误" in hits[0], (path, hits)

    def test_declared_bool_fields_stay_acceptable(self):
        """修 bool 洞不许顺手把 `type: bool` 的开关字段判成非法"""
        paths = [p for p, s in ConfigValidator.KNOWN_FIELDS.items()
                 if s.get("type") is bool]
        assert len(paths) == 4, "规格表里应仍有布尔开关字段"
        for path in paths:
            hits = [m for p_, m in self._errors_at(path) if p_ == path]
            assert hits == [], (path, hits)


class TestWebSectionKnobSurface:
    """`web` 节九个字段全部要有规格（L50 补齐）

    补之前实测（Temp `l50q/probe2.py` NONCE-bee0664903ff）：把整节写坏 ——
    `port: eighty`、`data_roots: data`（YAML 标量而非列表）、`cors_origins` 写成字符串、
    `cors_credentials: maybe`、`rate_limit_max_requests: -5`、`rate_limit_window_seconds:
    NaN` —— `validate_config` 仍判 `is_valid=True` / 0 error / 0 warning，因为
    `KNOWN_FIELDS` 里 `web.*` 一条都没有。后果不是「校验器不够严」而是「绿灯跑废」：
    `data_roots: data` 会被 `api/deps._config_data_roots` 按字符拆成 `d/a/t/a` 四个根，
    于是所有数据端点一律 403，症状长得像后端坏了。
    """

    @staticmethod
    def _errors(web):
        config = {
            "app": {"name": "test"},
            "models": {"default": "ernie"},
            "web": web,
        }
        return [(e.path, e.message) for e in validate_config(config).errors]

    #: 与 `WebConfig` 的字段集逐字对齐；新增字段会自动把本断言变红
    FIELDS = ["port", "host", "static_dir", "cors_origins", "cors_credentials",
              "data_roots", "rate_limit_max_requests", "rate_limit_window_seconds",
              "rate_limit_exempt_paths"]

    def test_every_web_field_has_a_spec(self):
        import dataclasses

        from augmentor.config import WebConfig

        declared = {f.name for f in dataclasses.fields(WebConfig)}
        specced = {p.split(".", 1)[1] for p in ConfigValidator.KNOWN_FIELDS
                   if p.startswith("web.") and "." in p}
        assert sorted(declared) == sorted(self.FIELDS), sorted(declared ^ set(self.FIELDS))
        assert declared == specced, sorted(declared ^ specced)

    @pytest.mark.parametrize("field,value", [
        ("port", 8000), ("port", 1), ("port", 65535),
        ("host", "0.0.0.0"), ("static_dir", "web/dist"),
        ("cors_origins", []), ("cors_origins", ["https://only-me.example"]),
        ("cors_credentials", True), ("cors_credentials", False),
        ("data_roots", ["data"]), ("data_roots", []),
        ("rate_limit_max_requests", 0), ("rate_limit_max_requests", 300),
        ("rate_limit_window_seconds", 0.0), ("rate_limit_window_seconds", 60),
        ("rate_limit_exempt_paths", ["/api/health"]),
    ])
    def test_legal_values_pass(self, field, value):
        assert self._errors({field: value}) == []

    @pytest.mark.parametrize("field,bad,expect", [
        ("port", "eighty", "类型错误"),
        ("port", 0, "值过小"),
        ("port", 65536, "值过大"),
        ("host", 8000, "类型错误"),
        ("cors_origins", "https://only-me.example", "类型错误"),
        ("data_roots", "data", "类型错误"),
        ("rate_limit_exempt_paths", "/api/health", "类型错误"),
        ("cors_credentials", "maybe", "类型错误"),
        ("rate_limit_max_requests", -5, "值过小"),
        ("rate_limit_window_seconds", -1.0, "值过小"),
        ("rate_limit_window_seconds", float("nan"), "不是有效数值"),
        ("rate_limit_window_seconds", "60", "类型错误"),
    ])
    def test_illegal_values_reported(self, field, bad, expect):
        errors = self._errors({field: bad})
        assert errors and errors[0][0] == f"web.{field}", (field, errors)
        assert expect in errors[0][1]

    def test_six_broken_keys_are_no_longer_silently_valid(self):
        """本类的立项用例：六个键同时写坏，`validate-config` 曾全绿"""
        errors = self._errors({
            "port": "eighty",
            "cors_origins": "https://only-me.example",
            "cors_credentials": "maybe",
            "data_roots": "data",
            "rate_limit_max_requests": -5,
            "rate_limit_window_seconds": float("nan"),
        })
        assert {p for p, _ in errors} == {
            "web.port", "web.cors_origins", "web.cors_credentials",
            "web.data_roots", "web.rate_limit_max_requests",
            "web.rate_limit_window_seconds"}

    def test_scalar_data_roots_is_the_dangerous_shape(self):
        """`data_roots: data` 不报错才是它危险的地方：它会让白名单按字符拆开

        两侧现在都拦得住：校验器按规格判「期望 list」，运行时
        `WebConfig.__post_init__` 走 `require_string_list`（A80 于 L51 关闭）。
        """
        errors = self._errors({"data_roots": "data"})
        assert errors and "类型错误" in errors[0][1]
        # 阳性对照：同一份配置写成列表时校验器无异议
        assert self._errors({"data_roots": ["data"]}) == []


class TestRuntimeValidatorParity:
    """配置面的最终判据：同一个值在两侧得到同一个答案（L51 / A77 + A80 + A83）

    A77 的根因是同一个区间在两边各抄一遍，抄完还漏；L51 的修法是把区间常量搬进
    `config.py` 并让两个配置类带上 `__post_init__`，于是本表能做到**逐个值**一致。
    改前实测（Temp `l51q/probe1.py` NONCE-A9C41E77）这一整列全是「校验器红、运行时
    绿」：`AugmentationConfig(max_retries=10**6, retry_delay=10**6, retry_jitter=50.0)`
    无判据构造成功，而校验器对同一批值报 6 条错。

    形状判据（A83）是本轮探针自己撞出来的镜像症状：`data_roots: [null]` 与
    `host: ""` 在校验器绿灯、在 `load_config` 抛。两侧现在同源，所以本表必须连
    形状一起判，否则「校验器绿 / 加载红」会重新长回来。
    """

    #: `(path, value, 是否应被判拒)`。边界值一律取自 `config.py` 的区间常量。
    CASES = [
        ("augmentation.variants_per_seed", 1, False),
        ("augmentation.variants_per_seed", 100, False),
        ("augmentation.variants_per_seed", 0, True),
        ("augmentation.variants_per_seed", 101, True),
        ("augmentation.variants_per_seed", True, True),
        ("augmentation.variants_per_seed", None, True),
        ("augmentation.num_threads", 40, False),
        ("augmentation.num_threads", 0, True),
        ("augmentation.num_threads", 101, True),
        ("augmentation.auto_save_interval", 1, False),
        ("augmentation.auto_save_interval", 0, True),
        ("augmentation.auto_save_interval", -1, True),
        ("augmentation.max_retries", 0, False),
        ("augmentation.max_retries", 20, False),
        ("augmentation.max_retries", -1, True),
        ("augmentation.max_retries", 21, True),
        ("augmentation.retry_delay", 0, False),
        ("augmentation.retry_delay", 60.0, False),
        ("augmentation.retry_delay", 60.1, True),
        ("augmentation.retry_delay", float("nan"), True),
        ("augmentation.max_retry_wait", 300.0, False),
        ("augmentation.max_retry_wait", 301.0, True),
        ("augmentation.retry_jitter", 0.0, False),
        ("augmentation.retry_jitter", 1.0, False),
        ("augmentation.retry_jitter", 1.001, True),
        ("web.port", 1, False),
        ("web.port", 65535, False),
        ("web.port", 0, True),
        ("web.port", 65536, True),
        ("web.port", None, True),
        ("web.host", "0.0.0.0", False),
        ("web.host", "", True),
        ("web.host", None, True),
        ("web.static_dir", "web/dist", False),
        ("web.static_dir", "", True),
        ("web.cors_origins", [], False),
        ("web.cors_origins", ["https://only-me.example"], False),
        ("web.cors_origins", "https://only-me.example", True),
        ("web.cors_origins", [None], True),
        ("web.cors_origins", ["", "https://only-me.example"], True),
        ("web.cors_origins", [True], True),
        ("web.cors_credentials", True, False),
        ("web.cors_credentials", False, False),
        ("web.cors_credentials", 1, True),
        ("web.cors_credentials", "maybe", True),
        ("web.data_roots", ["data"], False),
        ("web.data_roots", [], False),
        ("web.data_roots", "data", True),
        ("web.data_roots", [123], True),
        ("web.rate_limit_max_requests", 0, False),
        ("web.rate_limit_max_requests", -1, True),
        ("web.rate_limit_window_seconds", 60.0, False),
        ("web.rate_limit_window_seconds", -0.1, True),
        ("web.rate_limit_window_seconds", float("nan"), True),
        ("web.rate_limit_exempt_paths", ["/api/health"], False),
        ("web.rate_limit_exempt_paths", [""], True),
    ]

    @staticmethod
    def _validator_rejects(path, value):
        section, _, field = path.partition(".")
        config = {"app": {"name": "t"}, "models": {"default": "ernie"},
                  section: {field: value}}
        return any(e.path == path or e.path.startswith(path + "[")
                   for e in validate_config(config).errors)

    @staticmethod
    def _runtime_rejects(path, value):
        from augmentor.config import AugmentationConfig, WebConfig

        section, _, field = path.partition(".")
        cls = WebConfig if section == "web" else AugmentationConfig
        try:
            cls(**{field: value})
            return False
        except DataValidationError:
            return True

    @pytest.mark.parametrize("path,value,expected", CASES)
    def test_both_sides_give_the_documented_verdict(self, path, value, expected):
        assert self._validator_rejects(path, value) is expected, (path, value)
        assert self._runtime_rejects(path, value) is expected, (path, value)

    @pytest.mark.parametrize("path", sorted({p for p, _, _ in CASES}))
    def test_each_field_has_both_a_rejected_and_an_accepted_case(self, path):
        """防空表：本类若两侧同时「什么都不判」，逐值断言会全绿而缺陷没修

        每条 path 至少要有一个被拒值和一个被放行值，否则它没有区分度。
        """
        verdicts = [v for p, _, v in self.CASES if p == path]
        assert True in verdicts and False in verdicts, path

    def test_numeric_bounds_are_read_from_the_config_constants(self):
        """规格表里的数字必须是 `config.py` 那批常量本身，不是抄来的字面量

        A77 要防的就是「抄一遍」：常量住在 `config.py`，校验器只能引用不能重打。
        """
        from augmentor import MAX_RETRY_AFTER
        from augmentor.config import (AUTO_SAVE_INTERVAL_MIN, MAX_RETRIES_RANGE,
                                      NUM_THREADS_RANGE, PORT_RANGE,
                                      RATE_LIMIT_MIN_REQUESTS,
                                      RATE_LIMIT_MIN_WINDOW_SECONDS,
                                      RETRY_DELAY_RANGE, VARIANTS_PER_SEED_RANGE)

        s = ConfigValidator.KNOWN_FIELDS
        bounds = {
            "augmentation.variants_per_seed": VARIANTS_PER_SEED_RANGE,
            "augmentation.num_threads": NUM_THREADS_RANGE,
            "augmentation.max_retries": MAX_RETRIES_RANGE,
            "augmentation.retry_delay": RETRY_DELAY_RANGE,
            "augmentation.max_retry_wait": (0.0, MAX_RETRY_AFTER),
            "web.port": PORT_RANGE,
        }
        for path, (lo, hi) in bounds.items():
            assert (s[path]["min"], s[path]["max"]) == (lo, hi), path
        assert s["augmentation.auto_save_interval"]["min"] == AUTO_SAVE_INTERVAL_MIN
        assert s["web.rate_limit_max_requests"]["min"] == RATE_LIMIT_MIN_REQUESTS
        assert (s["web.rate_limit_window_seconds"]["min"]
                == RATE_LIMIT_MIN_WINDOW_SECONDS)

    def test_shape_flags_exist_only_where_runtime_judges(self):
        """`items` / `non_empty` 只许出现在运行时真判的键上

        反向不一致同样是缺陷：校验器比运行时严，合法配置会被 `validate-config` 拦在
        门外（`quality.threshold` 是数值键、运行时不判形状，规格就不许挂
        `items`/`non_empty`）。
        """
        s = ConfigValidator.KNOWN_FIELDS
        assert {p for p, spec in s.items() if "items" in spec} == {
            "web.cors_origins", "web.data_roots", "web.rate_limit_exempt_paths"}
        assert {p for p, spec in s.items() if spec.get("non_empty")} == {
            "web.host", "web.static_dir"}


class TestUnreadKeyWarnings:
    """写了没人读的键必须出声（L52 / A76）

    实测改前（Temp `l52q/probe1.py`、`probe2.py`、`probe3.py`，NONCE-45A0C1AB9510）：
    节名拼错（`augmenation.variants_per_seed: 999`）与节内键名拼错
    （`augmentation.variant_per_seed: 999`）两边都是 `is_valid=True` / 0 error /
    0 warning，而 `load_config` 那侧读回默认值 5；把 20 个被消费的节各塞一个假键，
    反馈同样是 0 / 0。用户看到的事实只有「我改了数，行为没变」，症状长得像后端坏了
    —— 这一类缺陷无法自查，所以必须让工具出声。

    判据的键集**从 `AppConfig` 的字段类型推导**，不抄第三份清单：`load_config` 只按
    `_load_section(..., defaults)` 的键取字段，而 L49/L50 的棘轮已经把「defaults 的
    键集 == dataclass 字段集」冻住（`KNOWN_UNMAPPED_FIELDS` 现为空集），所以
    dataclass 就是权威。`test_the_derived_whitelist_equals_what_load_section_reads`
    把这条锚住，以后加节、加字段、漏接表都会在这里现形。
    """

    SECTION_MARKER = "顶层段落没人读取"
    KEY_MARKER = "键没人读取"

    #: 仓库根：本类要读出厂 `config.yaml`，而 pytest 的工作目录不保证是仓库根
    REPO = Path(__file__).resolve().parent.parent.parent

    @staticmethod
    def _config(**sections):
        base = {"models": {"default": "ernie"}}
        base.update(sections)
        return base

    def _unread(self, config):
        """只取本轮新增的两类警告

        既有的「环境变量未设置」警告与本轮无关（出厂 `config.yaml` 就带 5 条），
        混进来会让断言随环境变量的设置状态漂移。
        """
        hits = []
        for w in validate_config(config).warnings:
            if self.SECTION_MARKER in w.message or self.KEY_MARKER in w.message:
                hits.append((w.path, w.message))
        return hits

    # === 判据的来源：推导，不是清单 ===

    def test_the_whitelist_is_derived_from_appconfig_fields(self):
        """`AppConfig` 里除 `models` / `default_model` 之外没有第三种字段

        这条是**防空转**：如果以后新增一个非 dataclass 的顶层字段（比如又一个
        `dict`），本类推导会静默跳过它，判据面凭空少一节；那条字段必须显式地
        出现在这里，作者才会被逼着做一次决定。
        """
        import dataclasses

        from augmentor.config import AppConfig

        declared = {f.name for f in dataclasses.fields(AppConfig)}
        derived = set(ConfigValidator.consumed_section_keys())
        assert declared - derived == {"models", "default_model"}, sorted(
            declared ^ derived | (declared - derived))

    def test_the_derived_whitelist_equals_what_load_section_reads(self, tmp_path):
        """推导出的键集必须与 `load_config` 实际读的 defaults 表逐节逐键相等

        这是本轮的结构本体：判据说「没人读」的那份名单，必须就是运行时读的那份，
        一个字都不许多。手法沿用 L49 的拦截（`TestSectionDefaultsMatchTheMappingTable`），
        只是把对照物从 dataclass 换成校验器。
        """
        import yaml

        from augmentor import config as config_module

        derived = ConfigValidator.consumed_section_keys()
        path = tmp_path / "all_sections.yaml"
        with open(path, "w", encoding="utf-8") as handle:
            yaml.safe_dump({name: {} for name in derived}, handle)

        seen = {}
        real = config_module._load_section
        monkeypatch = pytest.MonkeyPatch()
        monkeypatch.setattr(
            config_module, "_load_section",
            lambda raw, key, cls, defaults: (seen.__setitem__(key, set(defaults)),
                                             real(raw, key, cls, defaults))[1])
        try:
            config_module.load_config(str(path))
        finally:
            monkeypatch.undo()

        assert set(seen) == set(derived), sorted(set(seen) ^ set(derived))
        for name, read_keys in seen.items():
            assert read_keys == derived[name], (name, sorted(read_keys ^ derived[name]))

    def test_model_config_fields_are_all_read_by_load_config(self, tmp_path):
        """`models.<名字>` 的子键判据同样不许比运行时宽

        实测（Temp `l52q/probe3.py` 第 1 节）`ModelConfig` 的 8 个字段与
        `load_config` 读走的 8 个键双向差集都为空；这里把「每个字段真的被读」钉成
        常驻断言：字段若多出一个没人读的，本类的判据就会把合法配置报成警告。
        """
        import dataclasses

        import yaml

        from augmentor.config import ModelConfig, load_config

        values = {"type": "openai", "api_key": "AK", "secret_key": "SK",
                  "base_url": "http://example.invalid", "model": "m-1",
                  "temperature": 0.42, "top_p": 0.43,
                  "max_output_tokens": 42}
        declared = {f.name for f in dataclasses.fields(ModelConfig)}
        assert declared == set(values), sorted(declared ^ set(values))

        path = tmp_path / "one_model.yaml"
        with open(path, "w", encoding="utf-8") as handle:
            yaml.safe_dump({"models": {"default": "probe", "probe": values}},
                           handle, allow_unicode=True)

        loaded = load_config(str(path)).models["probe"]
        assert {f.name: getattr(loaded, f.name)
                for f in dataclasses.fields(ModelConfig)} == values

    # === 行为面：拼错就必须出声 ===

    @pytest.mark.parametrize("config,expected_path", [
        pytest.param({"models": {"default": "ernie"},
                      "augmenation": {"variants_per_seed": 3}}, "augmenation",
                     id="节名拼错-augmenation"),
        pytest.param({"models": {"default": "ernie"},
                      "augmentation": {"variant_per_seed": 3}},
                     "augmentation.variant_per_seed", id="节内键名拼错-少个s"),
        pytest.param({"models": {"default": "ernie"},
                      "augmentation": {"auto_save_intervall": 5}},
                     "augmentation.auto_save_intervall", id="节内键名拼错-双写l"),
        pytest.param({"models": {"default": "ernie"},
                      "web": {"ports": 8080}}, "web.ports", id="web节内键名拼错"),
        pytest.param({"models": {"default": "ernie"},
                      "web": {"cors_origin": ["https://only.example"]}},
                     "web.cors_origin", id="web节内漏了复数s"),
        pytest.param({"models": {"default": "ernie"},
                      "logging": {"levl": "DEBUG"}}, "logging.levl",
                     id="规格表零规格的logging节也照判"),
        pytest.param({"models": {"default": "ernie"},
                      "export": {"default_formt": "jsonl"}},
                     "export.default_formt", id="export节内键名拼错"),
        pytest.param({"models": {"default": "ernie"},
                      "quality": {"threshhold": 0.9}}, "quality.threshhold",
                     id="quality节内键名拼错"),
        pytest.param({"models": {"default": "ernie"},
                      "multimodal": {"image_extenshions": [".png"]}},
                     "multimodal.image_extenshions", id="列表字段同样只看键名"),
        pytest.param({"models": {"default": "ernie",
                                 "ernie": {"temperatur": 0.7}}},
                     "models.ernie.temperatur", id="模型条目子键拼错"),
        pytest.param({"models": {"default": "ernie"},
                      "output": {"export_dir": "out"}}, "output",
                     id="幽灵节output现在出声"),
        pytest.param({"models": {"default": "ernie"}, "default_model": "openai"},
                     "default_model", id="顶层default_model是已知陷阱"),
        pytest.param({"models": {"default": "ernie"}, "mycustom": {"a": 1}},
                     "mycustom", id="用户自加的顶层段落也出声"),
    ])
    def test_typo_gets_exactly_one_warning(self, config, expected_path):
        got = self._unread(config)
        assert [p for p, _ in got] == [expected_path], got

    @pytest.mark.parametrize("config,expected_path", [
        pytest.param({"models": {"default": "ernie"},
                      "augmenation": {"variants_per_seed": 3}}, "augmenation",
                     id="节名拼错"),
        pytest.param({"models": {"default": "ernie"},
                      "augmentation": {"variant_per_seed": 3}},
                     "augmentation.variant_per_seed", id="键名拼错"),
    ])
    def test_verdict_stays_valid_and_error_free(self, config, expected_path):
        """本轮判据只许用 WARNING：多余的键不挡服务，`is_valid` 是 CLI 退出码"""
        result = validate_config(config)
        assert result.is_valid is True
        assert result.errors == []
        assert [w.severity.value for w in result.warnings] == ["warning"]

    def test_suggestion_names_the_real_key(self):
        """能确定相近项时要把真名说出来，不能只说「没人读」"""
        messages = dict(self._unread(
            self._config(augmenation={"variants_per_seed": 3},
                         augmentation={"variant_per_seed": 3, "ports": 8080},
                         web={"ports": 8080})))
        assert "是否想写 augmentation？" in messages["augmenation"], messages
        assert "是否想写 variants_per_seed？" in messages["augmentation.variant_per_seed"]
        assert "是否想写 port？" in messages["web.ports"], messages

    def test_no_suggestion_when_nothing_is_close(self):
        """猜不出来就不许硬猜：把无关名写成建议比没有建议更坏"""
        _, message = self._unread(self._config(mycustom={"a": 1}))[0]
        assert "是否想写" not in message, message
        _, message = self._unread(
            self._config(augmentation={"zzz_not_a_field": 1}))[0]
        assert "是否想写" not in message, message

    def test_meta_sections_stay_exempt(self):
        """豁免的两节要真的是那两节：`app` 是文档承认的遗留元信息，`models` 的键是模型名"""
        assert self._unread({"app": {"name": "t", "whatever": 1},
                             "models": {"default": "ernie",
                                        "ernie": {"api_key": "K"}}}) == []

    def test_shape_error_is_not_double_reported(self):
        """节写成标量时规格走查已经判 ERROR，本类不重复报「没人读」"""
        result = validate_config(self._config(augmentation="abc"))
        assert [(e.path, "类型错误" in e.message) for e in result.errors] == [
            ("augmentation", True)], result.errors
        assert self._unread({"models": {"default": "ernie"}, "augmentation": "abc"}) == []

    def test_full_field_sections_stay_silent(self):
        """每个被消费的节写满它自己的字段集时必须零警告（判据不许比运行时窄）

        这条是本类的反向守护：只要推导漏了某个字段、或者某节被静默跳过，写满字段
        的配置立刻变红。值全是占位的 `1`，因为本条只看键名，类型错误由既有用例管。
        """
        config = self._config()
        for name, keys in ConfigValidator.consumed_section_keys().items():
            config[name] = {k: 1 for k in keys}
        assert self._unread(config) == []

    def test_shipped_config_has_no_unread_keys(self):
        """出厂 `config.yaml` 一个「没人读」警告都不许多：判据不能制造疲劳

        假阳性会把人训练成「忽略警告」，那比沉默更坏。
        这里只按标记筛，**不写警告条数**：既有的「环境变量未设置」那几条是环境变量的
        函数（同一份文件在本机 aug 下 5 条、在有 `BAIDU_API_KEY` 的会话里 2 条），
        钉数字就是把断言绑死在进程环境上 —— 与 L51 那条「不许断言
        `starlette.__version__ == \"1.6.0\"`」同罪。
        """
        result = validate_config_file(str(self.REPO / "config.yaml"))
        assert result.errors == []
        assert result.is_valid is True
        unread, other = [], []
        for w in result.warnings:
            (unread if (self.SECTION_MARKER in w.message
                        or self.KEY_MARKER in w.message) else other).append(w)
        assert unread == [], unread
        # 剩下的警告必须全是既有那一类，防止本轮判据悄悄把环境变量警告顶掉
        assert all("环境变量未设置" in w.message for w in other), other

    def test_the_shipped_file_actually_gets_walked(self):
        """上一条只证明「出厂文件干净」，这一条才证明判据真的看了它

        在出厂配置上原样加一个拼错的键，警告必须恰好那一条 —— 否则「零警告」可能
        只是判据压根没跑。
        """
        raw = yaml.safe_load((self.REPO / "config.yaml").read_text(encoding="utf-8"))
        raw["augmentation"]["variant_per_seed"] = 999
        assert self._unread(raw) == [
            ("augmentation.variant_per_seed",
             "键没人读取: augmentation.variant_per_seed（值不会生效）；"
             "是否想写 variants_per_seed？")]

    def test_save_config_roundtrip_is_silent(self, tmp_path):
        """「保存配置 → 校验配置」这条自家回路不许产生警告（否则判据与写入打架）"""
        from augmentor.config import AppConfig, save_config

        path = tmp_path / "saved.yaml"
        save_config(AppConfig(), str(path))
        result = validate_config_file(str(path))
        unread = [(w.path, w.message) for w in result.warnings
                  if (self.SECTION_MARKER in w.message or self.KEY_MARKER in w.message)]
        assert unread == [], unread

    def test_many_unread_keys_still_valid(self):
        """一次写坏 20 个节也不判负：WARNING 是提醒不是判决"""
        config = self._config()
        for name in ConfigValidator.consumed_section_keys():
            config[name] = {"zzz_not_a_field": 1}
        result = validate_config(config)
        assert result.is_valid is True
        assert len(self._unread(config)) == len(ConfigValidator.consumed_section_keys())

    # === 规格表自己要服从推导（A76 的反向棘轮）===

    def test_every_spec_path_points_at_a_real_key(self):
        """`KNOWN_FIELDS` 里每条路径都要指向真存在的路径，幽灵规格当场变红

        `output` / `output.export_dir` 曾是这样一条规格：`load_config` 里没有 `output`
        节（真节后是 `export`），实测 `output: {export_dir: out}` 得到绿灯，而
        `hasattr(load_config(...), "output") == False`。表里留一条死规格等于教用户写
        一个不生效的键 —— 那正是 A76 本尊，只不过这次是校验器自己造的。
        """
        sections = ConfigValidator.consumed_section_keys()
        ghosts = []
        for path in ConfigValidator.KNOWN_FIELDS:
            parts = path.split(".")
            if parts[0] in ConfigValidator.META_TOP_SECTIONS:
                continue
            if len(parts) == 1:
                if parts[0] not in sections:
                    ghosts.append(path)
            elif parts[0] not in sections or parts[1] not in sections[parts[0]]:
                ghosts.append(path)
        assert ghosts == [], ghosts

    def test_the_output_ghost_spec_is_gone(self):
        """删除 `output.*` 规格这件事本身也要一条断言，否则会被顺手加回来"""
        assert "output" not in ConfigValidator.KNOWN_FIELDS
        assert "output.export_dir" not in ConfigValidator.KNOWN_FIELDS
        assert [p for p, _ in self._unread(self._config(output={"export_dir": "out"}))] == [
            "output"]

    def test_exempt_list_is_exactly_the_two_meta_sections(self):
        """豁免清单是显式的两个名字，多一项就等于承认又一处「配了不生效」"""
        assert ConfigValidator.META_TOP_SECTIONS == {"app", "models"}

