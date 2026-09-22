# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""配置验证模块

提供配置文件的验证和校验功能。
"""

import os
import re
import yaml
import logging
from typing import Any, Dict, List, Optional, Set, Union
from dataclasses import dataclass, field
from pathlib import Path
from enum import Enum

logger = logging.getLogger(__name__)


class Severity(Enum):
    """严重程度"""
    ERROR = "error"
    WARNING = "warning"
    INFO = "info"


@dataclass
class ValidationError:
    """验证错误"""
    path: str
    message: str
    severity: Severity
    line: Optional[int] = None
    
    def to_dict(self) -> Dict:
        """转换为字典"""
        return {
            "path": self.path,
            "message": self.message,
            "severity": self.severity.value,
            "line": self.line
        }


@dataclass
class ValidationResult:
    """验证结果"""
    is_valid: bool
    errors: List[ValidationError] = field(default_factory=list)
    warnings: List[ValidationError] = field(default_factory=list)
    info: List[ValidationError] = field(default_factory=list)
    
    def add_error(self, path: str, message: str, line: int = None):
        """添加错误"""
        self.errors.append(ValidationError(path, message, Severity.ERROR, line))
        self.is_valid = False
    
    def add_warning(self, path: str, message: str, line: int = None):
        """添加警告"""
        self.warnings.append(ValidationError(path, message, Severity.WARNING, line))
    
    def add_info(self, path: str, message: str, line: int = None):
        """添加信息"""
        self.info.append(ValidationError(path, message, Severity.INFO, line))
    
    def to_dict(self) -> Dict:
        """转换为字典"""
        return {
            "is_valid": self.is_valid,
            "errors": [e.to_dict() for e in self.errors],
            "warnings": [e.to_dict() for e in self.warnings],
            "info": [e.to_dict() for e in self.info],
            "summary": {
                "errors": len(self.errors),
                "warnings": len(self.warnings),
                "info": len(self.info)
            }
        }


class ConfigValidator:
    """配置验证器
    
    支持多种配置文件格式的验证。
    """
    
    # 已知的配置模式
    KNOWN_FIELDS = {
        "app": {"type": dict, "required": True},
        "app.name": {"type": str, "required": True},
        "app.version": {"type": str, "required": True},
        "app.debug": {"type": bool, "default": False},
        "models": {"type": dict, "required": True},
        "models.default": {"type": str, "required": True},
        "augmentation": {"type": dict},
        "augmentation.variants_per_seed": {"type": int, "min": 1, "max": 100},
        "augmentation.num_threads": {"type": int, "min": 1, "max": 100},
        "quality": {"type": dict},
        "quality.enabled": {"type": bool},
        "quality.threshold": {"type": float, "min": 0.0, "max": 1.0},
        "dedup": {"type": dict},
        "dedup.enabled": {"type": bool},
        "output": {"type": dict},
        "output.export_dir": {"type": str},
    }
    
    # 环境变量模式
    ENV_VAR_PATTERN = re.compile(r'\$\{(\w+)\}')
    
    def __init__(self):
        """初始化验证器"""
        self._validators = {
            dict: self._validate_dict,
            list: self._validate_list,
            str: self._validate_string,
            int: self._validate_int,
            float: self._validate_float,
            bool: self._validate_bool,
        }
    
    def validate_file(self, file_path: str) -> ValidationResult:
        """验证配置文件
        
        Args:
            file_path: 配置文件路径
        
        Returns:
            验证结果
        """
        result = ValidationResult(is_valid=True)
        
        path = Path(file_path)
        if not path.exists():
            result.add_error("file", f"文件不存在: {file_path}")
            return result
        
        if not path.is_file():
            result.add_error("file", f"不是有效文件: {file_path}")
            return result
        
        try:
            with open(path, 'r', encoding='utf-8') as f:
                config = yaml.safe_load(f)
        except yaml.YAMLError as e:
            result.add_error("yaml", f"YAML解析错误: {str(e)}")
            return result
        except Exception as e:
            result.add_error("file", f"读取文件错误: {str(e)}")
            return result
        
        return self.validate_config(config)
    
    def validate_config(self, config: Dict) -> ValidationResult:
        """验证配置字典
        
        Args:
            config: 配置字典
        
        Returns:
            验证结果
        """
        result = ValidationResult(is_valid=True)
        
        if not isinstance(config, dict):
            result.add_error("config", "配置必须是字典类型")
            return result
        
        # 检查必填字段
        self._check_required_fields(config, "", result)
        
        # 验证已知字段
        self._validate_known_fields(config, "", result)
        
        # 验证环境变量引用
        self._validate_env_refs(config, "", result)
        
        return result
    
    def _check_required_fields(self, config: Dict, prefix: str, result: ValidationResult):
        """检查必填字段"""
        for field_path, spec in self.KNOWN_FIELDS.items():
            if not spec.get("required"):
                continue
            
            parts = field_path.split(".")
            current = config
            
            for part in parts:
                if isinstance(current, dict) and part in current:
                    current = current[part]
                else:
                    result.add_error(field_path, f"缺少必填字段: {field_path}")
                    break
            else:
                # 字段存在
                if current is None:
                    result.add_error(field_path, f"字段不能为null: {field_path}")
    
    def _validate_known_fields(self, config: Dict, prefix: str, result: ValidationResult):
        """验证已知字段"""
        for key, value in config.items():
            field_path = f"{prefix}.{key}" if prefix else key
            
            if field_path in self.KNOWN_FIELDS:
                spec = self.KNOWN_FIELDS[field_path]
                
                # 类型检查
                expected_type = spec.get("type")
                if expected_type and not isinstance(value, expected_type):
                    result.add_error(field_path, 
                                   f"类型错误: 期望 {expected_type.__name__}, 实际 {type(value).__name__}")
                    continue
                
                # 范围检查
                if "min" in spec and value < spec["min"]:
                    result.add_error(field_path, 
                                   f"值过小: {value} < {spec['min']}")
                if "max" in spec and value > spec["max"]:
                    result.add_error(field_path, 
                                   f"值过大: {value} > {spec['max']}")
            
            # 递归验证嵌套字典
            if isinstance(value, dict):
                self._validate_known_fields(value, field_path, result)
            elif isinstance(value, list):
                for i, item in enumerate(value):
                    if isinstance(item, dict):
                        self._validate_known_fields(item, f"{field_path}[{i}]", result)
    
    def _validate_env_refs(self, config: Dict, prefix: str, result: ValidationResult):
        """验证环境变量引用"""
        for key, value in config.items():
            field_path = f"{prefix}.{key}" if prefix else key
            
            if isinstance(value, str):
                matches = self.ENV_VAR_PATTERN.findall(value)
                for var_name in matches:
                    if var_name not in os.environ:
                        result.add_warning(field_path, 
                                         f"环境变量未设置: {var_name}")
            elif isinstance(value, dict):
                self._validate_env_refs(value, field_path, result)
            elif isinstance(value, list):
                for i, item in enumerate(value):
                    if isinstance(item, str):
                        matches = self.ENV_VAR_PATTERN.findall(item)
                        for var_name in matches:
                            if var_name not in os.environ:
                                result.add_warning(f"{field_path}[{i}]", 
                                                 f"环境变量未设置: {var_name}")
    
    def _validate_dict(self, value: Any, spec: Dict, path: str, result: ValidationResult):
        """验证字典"""
        if not isinstance(value, dict):
            result.add_error(path, f"期望字典类型, 实际 {type(value).__name__}")
    
    def _validate_list(self, value: Any, spec: Dict, path: str, result: ValidationResult):
        """验证列表"""
        if not isinstance(value, list):
            result.add_error(path, f"期望列表类型, 实际 {type(value).__name__}")
    
    def _validate_string(self, value: Any, spec: Dict, path: str, result: ValidationResult):
        """验证字符串"""
        if not isinstance(value, str):
            result.add_error(path, f"期望字符串类型, 实际 {type(value).__name__}")
        elif "enum" in spec and value not in spec["enum"]:
            result.add_error(path, f"值不在允许范围内: {value}")
    
    def _validate_int(self, value: Any, spec: Dict, path: str, result: ValidationResult):
        """验证整数"""
        if not isinstance(value, int) or isinstance(value, bool):
            result.add_error(path, f"期望整数类型, 实际 {type(value).__name__}")
        else:
            if "min" in spec and value < spec["min"]:
                result.add_error(path, f"值过小: {value} < {spec['min']}")
            if "max" in spec and value > spec["max"]:
                result.add_error(path, f"值过大: {value} > {spec['max']}")
    
    def _validate_float(self, value: Any, spec: Dict, path: str, result: ValidationResult):
        """验证浮点数"""
        if not isinstance(value, (int, float)) or isinstance(value, bool):
            result.add_error(path, f"期望数值类型, 实际 {type(value).__name__}")
        else:
            if "min" in spec and value < spec["min"]:
                result.add_error(path, f"值过小: {value} < {spec['min']}")
            if "max" in spec and value > spec["max"]:
                result.add_error(path, f"值过大: {value} > {spec['max']}")
    
    def _validate_bool(self, value: Any, spec: Dict, path: str, result: ValidationResult):
        """验证布尔值"""
        if not isinstance(value, bool):
            result.add_error(path, f"期望布尔类型, 实际 {type(value).__name__}")


def validate_config_file(file_path: str) -> ValidationResult:
    """验证配置文件
    
    Args:
        file_path: 配置文件路径
    
    Returns:
        验证结果
    """
    validator = ConfigValidator()
    return validator.validate_file(file_path)


def validate_config(config: Dict) -> ValidationResult:
    """验证配置字典
    
    Args:
        config: 配置字典
    
    Returns:
        验证结果
    """
    validator = ConfigValidator()
    return validator.validate_config(config)
