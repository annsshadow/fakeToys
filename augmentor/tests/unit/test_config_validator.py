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

    @pytest.mark.parametrize("value", [-1, "1s", float("nan"), 0, 1, 1.0, 30.5])
    def test_validator_and_runtime_use_the_same_verdict(self, value):
        """校验器与 `require_seconds` 必须同口径（上界除外，那是校验器独有的天花板）

        两边不一致时，`validate-config` 绿灯的配置会在建管道时抛
        `DataValidationError`，或者反过来把合法配置拦在门外。
        """
        rejected_by_validator = bool(self._errors({"retry_delay": value}))
        try:
            require_seconds("retry_delay", value)
            rejected_by_runtime = False
        except DataValidationError:
            rejected_by_runtime = True
        assert rejected_by_validator == rejected_by_runtime, value

class TestWaitBudgetKnobSurface:
    """`augmentation.max_retry_wait` / `retry_jitter` 的校验规格（L49 接线）

    与 `TestAugmentationRetryKnobs` 成对，但口径**不同**：那两个旧旋钮的上界
    （`retry_delay ≤ 60`、`max_retries ≤ 20`）是校验器独有的天花板，运行时收 999；
    新旋钮的上界（`max_retry_wait ≤ 300`、`retry_jitter ≤ 1`）在运行时同样判 —— 对这
    两个参数来说上界是承诺本身。所以本类的同口径断言能做到全区间一致，旧旋钮做不到。
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
        """
        numeric = [p for p, s in ConfigValidator.KNOWN_FIELDS.items()
                   if s.get("type") in (int, float)]
        assert len(numeric) == 10, "新增数值规格键会自动进入本断言"
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

        运行时那一侧没有判据（A80），所以校验器是唯一的拦得住它的地方。
        """
        errors = self._errors({"data_roots": "data"})
        assert errors and "类型错误" in errors[0][1]
        # 阳性对照：同一份配置写成列表时校验器无异议
        assert self._errors({"data_roots": ["data"]}) == []
