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

# 区间常量与运行时判据同一批来源（`config.py` 开头 + `retry.MAX_RETRY_AFTER`）。
# 这里是 A77 的修法本体：此前两边各抄一遍数字，抄漏的一边就成了「校验器独有
# 天花板」—— `max_retries: 10**6` 在这里报红、在 SDK 直构那边畅通无阻。
from .config import (AUTO_SAVE_INTERVAL_MIN, MAX_RETRIES_RANGE,
                     NUM_THREADS_RANGE, PORT_RANGE, RATE_LIMIT_MIN_REQUESTS,
                     RATE_LIMIT_MIN_WINDOW_SECONDS, RETRY_DELAY_RANGE,
                     VARIANTS_PER_SEED_RANGE)
from .retry import MAX_RETRY_AFTER

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
    #
    # `app` 段是历史遗留的元信息：`AppConfig` 没有对应字段，`load_config` 也从不
    # 读它。因此**不能**把 `app.*` 设为必填——`save_config` 按 `AppConfig` 的字段集
    # 落盘，永远写不出这个段，一旦设为必填，「保存配置」再「校验配置」必然失败
    # （实测 4 个必填错误）。这里保留为已知字段，仍做类型检查，只是不强制存在。
    KNOWN_FIELDS = {
        "app": {"type": dict},
        "app.name": {"type": str},
        "app.version": {"type": str},
        "app.debug": {"type": bool, "default": False},
        # `models.default` 是 `default_model` 的**唯一**来源（见 load_config），
        # 必须必填：缺了它默认模型会静默回落到 "ernie"。
        "models": {"type": dict, "required": True},
        "models.default": {"type": str, "required": True},
        "augmentation": {"type": dict},
        "augmentation.variants_per_seed": {
            "type": int, "min": VARIANTS_PER_SEED_RANGE[0],
            "max": VARIANTS_PER_SEED_RANGE[1]},
        "augmentation.num_threads": {
            "type": int, "min": NUM_THREADS_RANGE[0],
            "max": NUM_THREADS_RANGE[1]},
        # 每 N 条自动存一次增量。下界 1 是 L51 补的：实测改前 0 与 −1 都和 1 逐字
        # 同答（`processed - last >= interval` 恒真），20 条样本触发 20 次增量写盘，
        # 默认档 10 只 2 次 —— 不是崩，是 10 倍静默写放大，而且两侧都没有判据。
        "augmentation.auto_save_interval": {
            "type": int, "min": AUTO_SAVE_INTERVAL_MIN},
        # 重试两旋钮自 L45 起真的被消费（经 create_model_backend 透传给模型后端），
        # 所以要在校验器里有对应门槛。上界不是形式主义：`max_retries: 1000000` 在最坏
        # 情况下是 10^6 × 30s 的等待，`retry_delay` 再大也会被退避上限夹住，但 60s 已经
        # 远超任何合理的单次退避基数。**L51 起这两条不再只是校验器独有的天花板**：
        # 同一批常量已经进了 `AugmentationConfig.__post_init__`（A77）。
        "augmentation.max_retries": {
            "type": int, "min": MAX_RETRIES_RANGE[0],
            "max": MAX_RETRIES_RANGE[1]},
        "augmentation.retry_delay": {
            "type": float, "min": RETRY_DELAY_RANGE[0],
            "max": RETRY_DELAY_RANGE[1]},
        # 等待预算两旋钮（L49 / A73 + A75）。上界与运行时判据**同源**，不是校验器
        # 独有的天花板：`max_retry_wait` 本身就是那道封顶，放大它会作废
        # 「服务端指令一支封顶 300 s」的承诺（retry.MAX_RETRY_AFTER）；
        # `retry_jitter` 的 0-1 与 `validation.require_ratio` 的默认闭区间逐字一致。
        "augmentation.max_retry_wait": {"type": float, "min": 0.0,
                                        "max": MAX_RETRY_AFTER},
        "augmentation.retry_jitter": {"type": float, "min": 0.0, "max": 1.0},
        "quality": {"type": dict},
        "quality.enabled": {"type": bool},
        "quality.threshold": {"type": float, "min": 0.0, "max": 1.0},
        "dedup": {"type": dict},
        "dedup.enabled": {"type": bool},
        "output": {"type": dict},
        "output.export_dir": {"type": str},
        # `web` 节的规格自 L50 起补齐（此前整节 9 个字段在 `validate-config` 上零反馈：
        # 实测把 port 写成字符串、data_roots 写成 YAML 标量、限流数写成负数与 NaN，
        # 一份配置里六个键全错仍判 is_valid=True / 0 error / 0 warning）。`data_roots`
        # 是这里最要紧的一条：`data_roots: data` 会被按字符拆成 d/a/t/a 四个根目录，
        # 于是**所有**数据端点一律 403，症状长得像后端坏了。区间与运行时判据同一批
        # 常量（`config.PORT_RANGE` 等）：L51 已给 `WebConfig` 补上 `__post_init__`，
        # SDK 直构也认这套区间，「校验器红 / 运行时绿」的缝隙就此封住（关闭 A80）。
        # `items` / `non_empty` 两个形状键同样是 L51 补的，方向相反（新立 A83）：
        # 改前 `data_roots: [null]` 与 `host: ""` 在校验器绿灯、在 `load_config` 抛。
        "web": {"type": dict},
        "web.port": {"type": int, "min": PORT_RANGE[0], "max": PORT_RANGE[1]},
        "web.host": {"type": str, "non_empty": True},
        "web.static_dir": {"type": str, "non_empty": True},
        "web.cors_origins": {"type": list, "items": str},
        "web.cors_credentials": {"type": bool},
        "web.data_roots": {"type": list, "items": str},
        "web.rate_limit_max_requests": {"type": int,
                                        "min": RATE_LIMIT_MIN_REQUESTS},
        "web.rate_limit_window_seconds": {"type": float,
                                          "min": RATE_LIMIT_MIN_WINDOW_SECONDS},
        "web.rate_limit_exempt_paths": {"type": list, "items": str},
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
                # `float` 字段同时接受 `int`：YAML 里 `retry_delay: 1` / `threshold: 1`
                # 是最自然的写法，运行时的 `require_seconds` 也收 int。校验器若比运行
                # 时更严，就会把合法配置报成非法（实测：加本规则后 `retry_delay: 1`
                # 曾得到「类型错误: 期望 float, 实际 int」）。
                checked_type = (int, float) if expected_type is float else expected_type
                # `bool` 不算数值。`isinstance(True, int)` 恒真，所以 YAML 里写
                # `retry_jitter: true` 从前会被本校验器读成合法，而运行时四道判据
                # （`require_count` / `require_seconds` / `require_positive` /
                # `require_ratio`）**全部**显式拒 bool ⇒ `validate-config` 绿灯的配置
                # 会在建管道时抛（L49 实测：7 个数值规格键 7/7 都有这个洞）。
                # `type: bool` 的开关字段不受影响。
                if checked_type and (
                        (expected_type in (int, float) and isinstance(value, bool))
                        or not isinstance(value, checked_type)
                ):
                    result.add_error(field_path, 
                                   f"类型错误: 期望 {expected_type.__name__}, 实际 {type(value).__name__}")
                    continue
                # NaN 过不了任何比较（`nan < min` 与 `nan > max` 都是 False），于是
                # 会绕过下面的范围检查被读成「合法」，而运行时那一层拒收它——两边口径
                # 必须一致，否则 `validate-config` 绿灯的配置会在建管道时炸。
                if expected_type is float and value != value:
                    result.add_error(field_path, f"值不是有效数值: {value}")
                    continue
                
                # 范围检查
                if "min" in spec and value < spec["min"]:
                    result.add_error(field_path, 
                                   f"值过小: {value} < {spec['min']}")
                if "max" in spec and value > spec["max"]:
                    result.add_error(field_path, 
                                   f"值过大: {value} > {spec['max']}")
                # 形状判据（L51 / A83）：列表元素类型与非空。这两条以前只有运行时那一侧
                # 有（`validation.require_string_list` / `require_string`），于是
                # `data_roots: [null]` 与 `host: ""` 在 `validate-config` 上绿灯、在
                # `load_config` 里抛 —— A77 的镜像症状，同一轮一起封住。运行时拒的这里
                # 才拒（`items`/`non_empty` 只写在有运行时判据的键上），免得反向造出
                # 「校验器红 / 运行时绿」。
                if "items" in spec:
                    for i, item in enumerate(value):
                        if not isinstance(item, spec["items"]):
                            result.add_error(
                                f"{field_path}[{i}]",
                                f"元素类型错误: 期望 {spec['items'].__name__}, "
                                f"实际 {type(item).__name__}")
                        elif not item:
                            result.add_error(f"{field_path}[{i}]", "元素不能为空字符串")
                elif spec.get("non_empty") and not value:
                    result.add_error(field_path, "值不能为空字符串")
            
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
