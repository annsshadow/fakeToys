# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""数据集验证模块

提供数据集格式验证、完整性检查等功能，以及跨模块共用的标量入参判据
（`require_count` 计数旋钮 / `require_seconds` 时长旋钮 /
`require_positive` 无量纲倍率旋钮 / `require_ratio` 0-1 比例旋钮）。
"""

import json
import logging
import math
import re
from typing import List, Dict, Optional, Any, Set
from dataclasses import dataclass, field
from pathlib import Path
from enum import Enum

from augmentor.exceptions import DataValidationError

logger = logging.getLogger(__name__)


def require_count(name: str, value: Any, minimum: int = 0) -> Optional[int]:
    """校验「取前 N 条 / 向后移 N 条」这类计数旋钮，返回原值。

    口径：这类旋钮只有一种合法读法——**要多少条**。它们在实现里几乎都落到
    下标切片，而 Python 的切片对越界值不是报错，是换语义：

    - 负数：`seq[:-1]` 是「丢掉最后一条」，于是「要 1 条」变成「要 N-1 条」；
      `seq[-limit:]` 在 `limit` 为负时又变成「从倒数第 |limit| 条往后全要」。
    - 零：走 `value or default` 回落的调用点会把「要 0 条」当成「没传参数」，
      静默给出默认条数。

    两种都是**静默错答**：调用方拿到一个形状合法、内容却与请求相反的结果，
    没有任何信号。所以在 SDK 的入参处一次判掉，而不是让每个调用点各自小心。

    Args:
        name: 参数名，直接出现在报错里（调用方看到的就是自己写的那个名字）
        value: 传入的值
        minimum: 允许的下界。默认 0——「一条都不要」是合法请求（只要计数、
            只要格式信息），必须与「没传参数」区分开。需要强制正数的调用点
            （如每轮必须选出样本的主动学习）显式传 1。

    Returns:
        校验通过后的原值。`None` 直接放行——「未提供」由各调用点自己决定
        回落哪个默认值，这里不替它决定。

    Raises:
        DataValidationError: 值不是整数（`bool` 也不算），或小于 `minimum`
    """
    if value is None:
        return None
    if isinstance(value, bool) or not isinstance(value, int):
        raise DataValidationError(
            f"{name} 必须是整数，当前是 {value!r}（{type(value).__name__}）"
        )
    if value < minimum:
        raise DataValidationError(f"{name} 必须是不小于 {minimum} 的整数，当前是 {value}")
    return value


def require_seconds(name: str, value: Any, minimum: float = 0.0) -> Optional[float]:
    """校验「等待多少秒」这类时长旋钮，返回原值。

    `require_count` 的浮点对应物。计数旋钮的越界症状是「换语义」，时长旋钮一样，
    而且更隐蔽——`retry.compute_delay` 首尾各有一道夹逼（`min(max_delay, …)` 与
    `max(0.0, …)`），实测下来它**不会**把坏值抛出来，而是悄悄换成另一档等待：

    - `base_delay=-5.0` → `0.0`（尾部 `max()` 夹掉负号）：想要「等 5 秒」拿到「立刻重试」
    - `base_delay=NaN` → `30.0`（`30.0 < nan` 为假，`min()` 交出第一个参数）：等到上限
    - `max_delay=NaN` → `0.0`（尾部 `max()` 交出第一个参数）

    三条都不报错，所以判据只能放在入参处。YAML 侧 `.nan` / `.inf` 是合法浮点字面量
    （实测 `yaml.safe_load("a: .nan")` 给出 `float('nan')`），模板没填值就可能留下 NaN，
    这不是纯理论输入。

    正无穷不在拒绝之列：它被 `min(max_delay, …)` 夹住，行为有界且与「等很久」的意图一致。

    Args:
        name: 参数名，直接出现在报错里
        value: 传入的值。`int` 也接受（YAML 的 `retry_delay: 1` 读进来是整数）
        minimum: 允许的下界。默认 0——「不等，失败就立刻重试」是合法请求

    Returns:
        校验通过后的原值。`None` 直接放行——「未提供」由各调用点决定回落哪个默认值
    """
    if value is None:
        return None
    if isinstance(value, bool) or not isinstance(value, (int, float)):
        raise DataValidationError(
            f"{name} 必须是数值秒数，当前是 {value!r}（{type(value).__name__}）"
        )
    if math.isnan(value):
        raise DataValidationError(f"{name} 不能是 NaN")
    if value < minimum:
        raise DataValidationError(f"{name} 必须是不小于 {minimum} 的秒数，当前是 {value}")
    return value


def require_positive(name: str, value: Any) -> Optional[float]:
    """校验「指数因子」这类**无量纲倍率**旋钮，返回原值。

    与 `require_seconds` 同族，但下界固定为「严格大于 0」、没有 `minimum` 可配：
    这类值只有「每档乘多少」一种合法读法，而实现里的幂运算对 0 与负数都不报错，
    只是换形状。实测（`retry.compute_delay`，`base_delay=1`、`max_delay=30`，
    退避取第 1/2/3 档）：

    - `factor=0` → ``[1.0, 0.0, 0.0]``（``0 ** 0`` 是 1，之后每档都是 0 ⇒ 忙重试）
    - `factor=-2` → ``[1.0, 0.0, 4.0]``（负幂次的小数被尾部 `max(0.0, …)` 与正负
      交替搅在一起，得到一个非单调、隔档完全不等待的序列）
    - `factor=NaN` → ``[1.0, 30.0, 30.0]``（首档照常，之后 `min()` 交出第一个参数）
    - `factor='2'` / `None` → 不报「参数写错」，而是 ``TypeError: unsupported
      operand type(s) for ** or pow()`` —— 且它发生在**第一次真实调用失败之后**
      （见 `with_retries` 里 `raise last_exc` 根本轮不到执行），原始异常整个丢失。

    `+inf` 与 `int` 照常接受：实测 `factor=inf` 给 ``[1.0, 30.0, 30.0]``，行为有界
    且与「一路上限」的意图一致（与 `require_seconds` 同口径）。

    Args:
        name: 参数名，直接出现在报错里
        value: 传入的值

    Returns:
        校验通过后的原值。`None` 视为「没传参数」，与 `require_count` /
        `require_seconds` 一致，由各调用点自己决定回落哪个默认值。

    Raises:
        DataValidationError: 值不是数值（`bool` 也不算）、是 NaN，或不大于 0
    """
    if value is None:
        return None
    if isinstance(value, bool) or not isinstance(value, (int, float)):
        raise DataValidationError(
            f"{name} 必须是数值，当前是 {value!r}（{type(value).__name__}）"
        )
    if math.isnan(value):
        raise DataValidationError(f"{name} 不能是 NaN")
    if value <= 0:
        raise DataValidationError(f"{name} 必须是大于 0 的数值，当前是 {value}")
    return value


def require_ratio(name: str, value: Any,
                  minimum: float = 0.0,
                  maximum: float = 1.0) -> Optional[float]:
    """校验「随机抖动比例」这类**落在闭区间内的无量纲比例**旋钮，返回原值。

    与 `require_positive` 同族，区别只有一句话：比例**有上界**，而实现里没有任何
    一处会替调用方夹住它。以 `retry.compute_delay` 的 `jitter` 为例（实测，
    `max_delay=30`；贴顶档取 `base_delay=20`、`factor=2`、`attempt=3` ⇒ 夹完正好 30.0）：

    - 抖动是**加在夹好之后**的（``delay += rng.uniform(0, jitter * delay)``），所以
      退避一旦贴到 `max_delay`，jitter 就成了那一支真正的上限来源：同一档位实测
      `jitter=0.5` 抽出 30.02 ~ 44.94 s、`1.0` 抽出 30.05 ~ 59.87 s、
      `2.0` 抽出 30.09 ~ 89.74 s、`50.0` 抽出 32.27 ~ **1523.54 s** —— 全部
      100% 越过 `max_delay=30`。把比例判在 1 以内，那一支的最坏等待就可证地封在
      2 × `max_delay`（60 s）；判在外面，等于交出一个上界随用户手滑线性放大的旋钮。
    - **下界 0 不是多余的**：负数与 NaN 都过不了 `if jitter > 0` 这道门，于是
      「传了一个参数」与「什么都没传」逐字同答（实测 `jitter=-5.0` / `NaN` 都交出
      ``[1.0, 2.0, 4.0]``，与 `jitter=0` 完全相同）。这类静默不报错的读法正是
      L33 禁止 `or` 回落时抱怨过的同一形状。
    - `bool` 一并拒：`True` 会被当成 1.0 ⇒ 一个本想写「打开抖动」的参数打开的是
      **最大档**抖动（实测 `jitter=True` 与 `jitter=1.0` 五次抽样逐字相同）。

    Args:
        name: 参数名，直接出现在报错里
        value: 传入的值
        minimum: 下界（含），默认 0.0
        maximum: 上界（含），默认 1.0

    Returns:
        校验通过后的原值。`None` 视为「没传参数」，与 `require_count` /
        `require_seconds` / `require_positive` 一致，由调用点自己决定回落哪个默认值。

    Raises:
        DataValidationError: 值不是数值（`bool` 也不算）、是 NaN，或不在闭区间内
    """
    if value is None:
        return None
    if isinstance(value, bool) or not isinstance(value, (int, float)):
        raise DataValidationError(
            f"{name} 必须是数值，当前是 {value!r}（{type(value).__name__}）"
        )
    if math.isnan(value):
        raise DataValidationError(f"{name} 不能是 NaN")
    if not minimum <= value <= maximum:
        raise DataValidationError(
            f"{name} 必须是 {minimum} 到 {maximum} 之间的比例，当前是 {value}"
        )
    return value


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
            # 浅拷贝：把类级字典直接挂到实例上，任一实例改规则就会污染全部预设
            self.rules = dict(self.PRESET_RULES[preset])
        else:
            self.rules = dict(self.PRESET_RULES["basic"])
        # 禁止模式的编译缓存，元素是 `(源列表, 编译结果)`；见 `_compiled_forbidden()`
        self._forbidden_cache = None

    def _compiled_forbidden(self) -> tuple:
        """把 `forbidden_patterns` 编译一次并复用

        `re.search(字符串模式, ...)` 每次调用都要在 `re` 模块内部重做一遍
        「按 `(pattern, flags)` 查编译缓存」；真实 6902 条数据的 strict 校验里，
        禁止模式那一段占了整档耗时的 53%，预编译实测快 1.69×。

        缓存键用**模式列表对象本身**而不是布尔「编译过了」：调用方换掉
        `rules["forbidden_patterns"]` 时会触发重编译，而不是静默沿用旧结果。
        惰性编译同时保留了报错时机——非法模式仍在第一条数据上抛，而不是构造时。
        """
        patterns = self.rules.get("forbidden_patterns") or ()
        cache = self._forbidden_cache
        if cache is None or cache[0] is not patterns:
            cache = (patterns, tuple(
                re.compile(p, re.IGNORECASE) for p in patterns))
            self._forbidden_cache = cache
        return cache[1]
    
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
        forbidden = self._compiled_forbidden()
        for field_name in ["instruction", "output"]:
            if field_name in item and isinstance(item[field_name], str):
                for pattern in forbidden:
                    if pattern.search(item[field_name]):
                        issues.append(ValidationIssue(
                            field=field_name,
                            message=f"字段 {field_name} 包含禁止的模式: {pattern.pattern}",
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


def generate_validation_report(items: List[Dict], preset: str = "basic") -> Dict:
    """生成批量验证报告（增强功能）
    
    Args:
        items: 数据列表
        preset: 验证预设
    
    Returns:
        包含验证结果、统计信息和建议的完整报告
    """
    validator = DatasetValidator(preset=preset)
    result = validator.validate(items)
    
    issues_by_severity = {}
    for issue in result.issues:
        severity_name = issue.severity.value
        issues_by_severity.setdefault(severity_name, 0)
        issues_by_severity[severity_name] += 1
    
    return {
        "summary": result.to_dict(),
        "issues_by_severity": issues_by_severity,
        "total_items": len(items),
        "passed_items": result.valid_items,
        "failed_items": len(items) - result.valid_items,
        "pass_rate": result.valid_items / len(items) if items else 0.0,
        "suggestions": [
            f"发现 {issues_by_severity.get('error', 0)} 个严重错误",
            f"发现 {issues_by_severity.get('warning', 0)} 个警告",
            "建议根据严重程度优先修复错误"
        ]
    }
