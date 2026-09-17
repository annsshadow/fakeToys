"""数据集验证模块

提供数据集格式验证、完整性检查等功能。
"""

import json
import logging
from typing import List, Dict, Optional, Any, Set
from dataclasses import dataclass, field
from pathlib import Path
from enum import Enum

logger = logging.getLogger(__name__)


class ValidationSeverity(Enum):
    """验证严重程度"""
    ERROR = "error"
    WARNING = "warning"
    INFO = "info"


@dataclass
class ValidationIssue:
    """验证问题"""
    field: str
    message: str
    severity: ValidationSeverity
    index: Optional[int] = None
    value: Any = None


@dataclass
class ValidationResult:
    """验证结果"""
    is_valid: bool
    total_items: int
    valid_items: int
    issues: List[ValidationIssue] = field(default_factory=list)
    
    @property
    def error_count(self) -> int:
        return sum(1 for i in self.issues if i.severity == ValidationSeverity.ERROR)
    
    @property
    def warning_count(self) -> int:
        return sum(1 for i in self.issues if i.severity == ValidationSeverity.WARNING)
    
    def to_dict(self) -> Dict:
        return {
            "is_valid": self.is_valid,
            "total_items": self.total_items,
            "valid_items": self.valid_items,
            "error_count": self.error_count,
            "warning_count": self.warning_count,
            "issues": [
                {
                    "field": i.field,
                    "message": i.message,
                    "severity": i.severity.value,
                    "index": i.index
                }
                for i in self.issues[:100]  # 限制返回数量
            ]
        }


class DatasetValidator:
    """数据集验证器
    
    支持多种验证规则和自定义验证。
    """
    
    # 预定义的验证规则
    PRESET_RULES = {
        "basic": {
            "required_fields": ["instruction", "output"],
            "optional_fields": ["input"],
            "max_instruction_length": 1000,
            "max_output_length": 5000
        },
        "strict": {
            "required_fields": ["instruction", "output"],
            "optional_fields": ["input"],
            "min_instruction_length": 5,
            "max_instruction_length": 500,
            "min_output_length": 10,
            "max_output_length": 2000,
            "forbidden_patterns": [r"<script>", r"javascript:"]
        },
        "chat": {
            "required_fields": ["instruction", "output"],
            "optional_fields": ["input", "history"],
            "max_instruction_length": 500,
            "max_output_length": 1000,
            "check_history_format": True
        }
    }
    
    def __init__(self, rules: Optional[Dict] = None, preset: str = "basic"):
        """初始化验证器
        
        Args:
            rules: 自定义验证规则
            preset: 预设规则名称
        """
        if rules:
            self.rules = rules
        elif preset in self.PRESET_RULES:
            self.rules = self.PRESET_RULES[preset]
        else:
            self.rules = self.PRESET_RULES["basic"]
    
    def validate(self, items: List[Dict]) -> ValidationResult:
        """验证数据集
        
        Args:
            items: 数据列表
        
        Returns:
            验证结果
        """
        issues = []
        valid_count = 0
        
        for idx, item in enumerate(items):
            item_issues = self._validate_item(item, idx)
            issues.extend(item_issues)
            
            # 如果没有ERROR级别问题，计为有效
            has_error = any(i.severity == ValidationSeverity.ERROR for i in item_issues)
            if not has_error:
                valid_count += 1
        
        return ValidationResult(
            is_valid=valid_count == len(items),
            total_items=len(items),
            valid_items=valid_count,
            issues=issues
        )
    
    def _validate_item(self, item: Dict, index: int) -> List[ValidationIssue]:
        """验证单条数据
        
        Args:
            item: 数据项
            index: 索引
        
        Returns:
            问题列表
        """
        issues = []
        
        # 检查是否为字典
        if not isinstance(item, dict):
            issues.append(ValidationIssue(
                field="root",
                message="数据项必须是字典类型",
                severity=ValidationSeverity.ERROR,
                index=index
            ))
            return issues
        
        # 检查必填字段
        for field_name in self.rules.get("required_fields", []):
            if field_name not in item:
                issues.append(ValidationIssue(
                    field=field_name,
                    message=f"缺少必填字段: {field_name}",
                    severity=ValidationSeverity.ERROR,
                    index=index
                ))
            elif not isinstance(item[field_name], str):
                issues.append(ValidationIssue(
                    field=field_name,
                    message=f"字段 {field_name} 必须是字符串类型",
                    severity=ValidationSeverity.ERROR,
                    index=index
                ))
            elif len(item[field_name].strip()) == 0:
                issues.append(ValidationIssue(
                    field=field_name,
                    message=f"字段 {field_name} 不能为空",
                    severity=ValidationSeverity.WARNING,
                    index=index
                ))
        
        # 检查字段长度
        for field_name in ["instruction", "output", "input"]:
            if field_name in item and isinstance(item[field_name], str):
                value = item[field_name]
                min_key = f"min_{field_name}_length"
                max_key = f"max_{field_name}_length"
                
                if min_key in self.rules and len(value) < self.rules[min_key]:
                    issues.append(ValidationIssue(
                        field=field_name,
                        message=f"字段 {field_name} 长度不能小于 {self.rules[min_key]}",
                        severity=ValidationSeverity.WARNING,
                        index=index,
                        value=len(value)
                    ))
                
                if max_key in self.rules and len(value) > self.rules[max_key]:
                    issues.append(ValidationIssue(
                        field=field_name,
                        message=f"字段 {field_name} 长度不能大于 {self.rules[max_key]}",
                        severity=ValidationSeverity.WARNING,
                        index=index,
                        value=len(value)
                    ))
        
        # 检查禁止的模式
        import re
        forbidden_patterns = self.rules.get("forbidden_patterns", [])
        for field_name in ["instruction", "output"]:
            if field_name in item and isinstance(item[field_name], str):
                for pattern in forbidden_patterns:
                    if re.search(pattern, item[field_name], re.IGNORECASE):
                        issues.append(ValidationIssue(
                            field=field_name,
                            message=f"字段 {field_name} 包含禁止的模式: {pattern}",
                            severity=ValidationSeverity.ERROR,
                            index=index
                        ))
        
        # 检查历史记录格式
        if self.rules.get("check_history_format") and "history" in item:
            history = item["history"]
            if not isinstance(history, list):
                issues.append(ValidationIssue(
                    field="history",
                    message="history 字段必须是列表类型",
                    severity=ValidationSeverity.ERROR,
                    index=index
                ))
            else:
                for turn_idx, turn in enumerate(history):
                    if not isinstance(turn, dict):
                        issues.append(ValidationIssue(
                            field="history",
                            message=f"history[{turn_idx}] 必须是字典类型",
                            severity=ValidationSeverity.ERROR,
                            index=index
                        ))
                    elif "role" not in turn or "content" not in turn:
                        issues.append(ValidationIssue(
                            field="history",
                            message=f"history[{turn_idx}] 缺少 role 或 content 字段",
                            severity=ValidationSeverity.WARNING,
                            index=index
                        ))
        
        return issues
    
    def validate_file(self, file_path: str) -> ValidationResult:
        """验证文件
        
        Args:
            file_path: 文件路径
        
        Returns:
            验证结果
        """
        with open(file_path, 'r', encoding='utf-8') as f:
            items = json.load(f)
        
        return self.validate(items)


class DataSanitizer:
    """数据清洗器
    
    提供数据清洗和修复功能。
    """
    
    def __init__(self):
        """初始化清洗器"""
        pass
    
    def sanitize(self, items: List[Dict], fix: bool = True) -> List[Dict]:
        """清洗数据集
        
        Args:
            items: 数据列表
            fix: 是否尝试修复问题
        
        Returns:
            清洗后的数据列表
        """
        result = []
        
        for item in items:
            sanitized = self._sanitize_item(item, fix) if fix else item
            if sanitized is not None:
                result.append(sanitized)
        
        return result
    
    def _sanitize_item(self, item: Dict, fix: bool) -> Optional[Dict]:
        """清洗单条数据
        
        Args:
            item: 数据项
            fix: 是否尝试修复
        
        Returns:
            清洗后的数据项，或 None（如果应删除）
        """
        if not isinstance(item, dict):
            return None
        
        result = item.copy()
        
        # 清洗字符串字段
        for field_name in ["instruction", "output", "input"]:
            if field_name in result and isinstance(result[field_name], str):
                # 去除首尾空白
                result[field_name] = result[field_name].strip()
                
                # 去除多余空白
                if fix:
                    import re
                    result[field_name] = re.sub(r'\s+', ' ', result[field_name])
                
                # 移除控制字符
                if fix:
                    result[field_name] = ''.join(
                        c for c in result[field_name] 
                        if c.isprintable() or c in '\n\r\t'
                    )
        
        # 移除空的必填字段
        if not result.get("instruction") or not result.get("output"):
            return None
        
        return result
    
    def remove_duplicates(self, 
                         items: List[Dict], 
                         key: str = "instruction",
                         keep: str = "first") -> List[Dict]:
        """移除重复项
        
        Args:
            items: 数据列表
            key: 去重依据的字段
            keep: 保留策略 (first/last)
        
        Returns:
            去重后的数据列表
        """
        seen = {}
        result = []
        
        for idx, item in enumerate(items):
            value = item.get(key, "")
            if value not in seen:
                seen[value] = idx
                result.append(item)
            elif keep == "last":
                # 替换之前的记录
                old_idx = seen[value]
                for i, r in enumerate(result):
                    if r.get(key, "") == value:
                        result[i] = item
                        break
        
        return result


def validate_dataset(file_path: str, preset: str = "basic") -> Dict:
    """验证数据集文件
    
    Args:
        file_path: 文件路径
        preset: 预设规则
    
    Returns:
        验证结果字典
    """
    validator = DatasetValidator(preset=preset)
    result = validator.validate_file(file_path)
    return result.to_dict()


def sanitize_dataset(items: List[Dict], remove_duplicates: bool = True) -> List[Dict]:
    """清洗数据集
    
    Args:
        items: 数据列表
        remove_duplicates: 是否移除重复项
    
    Returns:
        清洗后的数据列表
    """
    sanitizer = DataSanitizer()
    result = sanitizer.sanitize(items)
    
    if remove_duplicates:
        result = sanitizer.remove_duplicates(result)
    
    return result
