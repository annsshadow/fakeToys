"""数据集 schema 校验模块

对训练数据条目做轻量级结构校验（必需键、类型、枚举、长度/数值范围），
不依赖 JSON Schema 库，纯标准库实现，便于在数据入池前拦截脏数据。
"""

import logging
from dataclasses import dataclass, field
from typing import Any, Dict, List, Optional

logger = logging.getLogger(__name__)


@dataclass
class FieldRule:
    """单字段校验规则"""

    type: Any = None            # 期望 Python 类型（或类型元组）
    required: bool = True
    enum: Optional[List[Any]] = None
    min_length: Optional[int] = None   # 字符串/列表最小长度
    max_length: Optional[int] = None
    min_value: Optional[float] = None
    max_value: Optional[float] = None


@dataclass
class SchemaValidationResult:
    """schema 校验结果"""

    total_items: int
    invalid_items: List[Dict[str, Any]] = field(default_factory=list)
    error_count: int = 0

    @property
    def is_valid(self) -> bool:
        return self.error_count == 0

    def to_dict(self) -> Dict[str, Any]:
        return {
            "total_items": self.total_items,
            "is_valid": self.is_valid,
            "error_count": self.error_count,
            "invalid_count": len(self.invalid_items),
            "errors": self.invalid_items,
        }


class DatasetSchema:
    """数据集 schema 校验器"""

    def __init__(self, rules: Dict[str, FieldRule], extra_fields_allowed: bool = True):
        """初始化

        Args:
            rules: 字段名 -> 校验规则
            extra_fields_allowed: 是否允许未知字段
        """
        self.rules = rules
        self.extra_fields_allowed = extra_fields_allowed

    @classmethod
    def from_spec(cls, spec: Dict[str, Dict[str, Any]],
                  extra_fields_allowed: bool = True) -> "DatasetSchema":
        """从字典规格构造 schema

        Args:
            spec: 字段名 -> {type/required/enum/min_length/...}
            extra_fields_allowed: 是否允许未知字段

        Returns:
            DatasetSchema 实例
        """
        rules = {
            name: FieldRule(
                type=cfg.get("type"),
                required=cfg.get("required", True),
                enum=cfg.get("enum"),
                min_length=cfg.get("min_length"),
                max_length=cfg.get("max_length"),
                min_value=cfg.get("min_value"),
                max_value=cfg.get("max_value"),
            )
            for name, cfg in spec.items()
        }
        return cls(rules, extra_fields_allowed)

    def validate_item(self, item: Dict[str, Any]) -> List[str]:
        """校验单条数据

        Args:
            item: 数据条目

        Returns:
            错误信息列表（空为通过）
        """
        errors: List[str] = []
        if not isinstance(item, dict):
            return ["条目必须是 dict"]

        for name, rule in self.rules.items():
            present = name in item
            value = item.get(name)

            if not present:
                if rule.required:
                    errors.append(f"缺少必需字段: {name}")
                continue

            if value is None:
                if rule.required:
                    errors.append(f"字段为空: {name}")
                continue

            if rule.type is not None:
                # bool 是 int 子类，排除 bool 误判数值类型
                if rule.type in (int, float) and isinstance(value, bool):
                    errors.append(f"字段 {name} 类型应为 {rule.type.__name__}")
                elif not isinstance(value, rule.type):
                    errors.append(
                        f"字段 {name} 类型应为 {rule.type.__name__}，"
                        f"实际 {type(value).__name__}"
                    )

            if rule.enum is not None and value not in rule.enum:
                errors.append(f"字段 {name} 取值 {value!r} 不在枚举 {rule.enum}")

            if isinstance(value, (str, list)):
                length = len(value)
                if rule.min_length is not None and length < rule.min_length:
                    errors.append(f"字段 {name} 长度 {length} 小于 {rule.min_length}")
                if rule.max_length is not None and length > rule.max_length:
                    errors.append(f"字段 {name} 长度 {length} 大于 {rule.max_length}")

            if isinstance(value, (int, float)) and not isinstance(value, bool):
                if rule.min_value is not None and value < rule.min_value:
                    errors.append(f"字段 {name} 值 {value} 小于 {rule.min_value}")
                if rule.max_value is not None and value > rule.max_value:
                    errors.append(f"字段 {name} 值 {value} 大于 {rule.max_value}")

        if not self.extra_fields_allowed:
            unknown = set(item.keys()) - set(self.rules.keys())
            for key in sorted(unknown):
                errors.append(f"未知字段: {key}")

        return errors

    def validate(self, items: List[Dict[str, Any]]) -> SchemaValidationResult:
        """校验整个数据集

        Args:
            items: 数据列表

        Returns:
            SchemaValidationResult 实例
        """
        invalid = []
        error_total = 0
        for index, item in enumerate(items):
            errors = self.validate_item(item)
            if errors:
                error_total += len(errors)
                invalid.append({"index": index, "errors": errors})
        result = SchemaValidationResult(
            total_items=len(items),
            invalid_items=invalid,
            error_count=error_total,
        )
        if not result.is_valid:
            logger.warning("schema 校验发现 %d 条不合格数据", len(invalid))
        return result


def validate_schema(items: List[Dict[str, Any]],
                    spec: Dict[str, Dict[str, Any]],
                    extra_fields_allowed: bool = True) -> SchemaValidationResult:
    """按字典规格校验数据集（模块级便捷函数）

    Args:
        items: 数据列表
        spec: 字段名 -> 规则配置
        extra_fields_allowed: 是否允许未知字段

    Returns:
        SchemaValidationResult 实例
    """
    return DatasetSchema.from_spec(spec, extra_fields_allowed).validate(items)
