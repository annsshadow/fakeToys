"""质量门禁模块

将多个质量指标组合为可配置的门禁规则，作为数据进入下游训练前的
最后一道关卡。支持 AND/OR 组合、严重级别与自动处置动作。
"""

import logging
from dataclasses import dataclass, field
from typing import Any, Callable, Dict, List, Optional

logger = logging.getLogger(__name__)


class GateVerdict:
    """门禁判定结果常量"""
    PASSED = "passed"
    FAILED = "failed"
    WARNED = "warned"


@dataclass
class GateRule:
    """单条门禁规则"""
    name: str
    metric_key: str
    operator: str  # ">=", "<=", "==", "in", "not_in"
    value: Any
    severity: str = "error"  # error / warning
    description: str = ""

    def evaluate(self, metrics: Dict[str, Any]) -> bool:
        """对指标字典求值，返回是否满足规则

        Args:
            metrics: 指标字典

        Returns:
            是否满足
        """
        current = metrics.get(self.metric_key)
        if current is None:
            return False

        if self.operator == ">=":
            return current >= self.value
        if self.operator == "<=":
            return current <= self.value
        if self.operator == ">":
            return current > self.value
        if self.operator == "<":
            return current < self.value
        if self.operator == "==":
            return current == self.value
        if self.operator == "!=":
            return current != self.value
        if self.operator == "in":
            return current in self.value
        if self.operator == "not_in":
            return current not in self.value

        raise ValueError(f"不支持的操作符: {self.operator}")


@dataclass
class GateReport:
    """门禁执行报告"""
    verdict: str
    failed_rules: List[str] = field(default_factory=list)
    warned_rules: List[str] = field(default_factory=list)
    metrics: Dict[str, Any] = field(default_factory=dict)

    @property
    def passed(self) -> bool:
        return self.verdict == GateVerdict.PASSED or self.verdict == GateVerdict.WARNED

    def to_dict(self) -> Dict[str, Any]:
        return {
            "verdict": self.verdict,
            "passed": self.passed,
            "failed_rules": self.failed_rules,
            "warned_rules": self.warned_rules,
            "metrics": self.metrics,
        }


class QualityGate:
    """质量门禁执行器

    规则间默认 AND 组合：所有 error 规则必须通过才放行，
    warning 规则失败只产生告警不阻断。
    """

    def __init__(self,
                 rules: Optional[List[GateRule]] = None,
                 block_on_warning: bool = False):
        """初始化门禁

        Args:
            rules: 规则列表
            block_on_warning: 为 True 时 warning 规则失败也会阻断
        """
        self.rules = rules or []
        self.block_on_warning = block_on_warning

    def add_rule(self, rule: GateRule) -> "QualityGate":
        """追加规则（支持链式）"""
        self.rules.append(rule)
        return self

    def run(self, metrics: Dict[str, Any]) -> GateReport:
        """执行门禁

        Args:
            metrics: 指标字典（如 {"pass_rate": 0.8, "duplicate_rate": 0.05}）

        Returns:
            门禁报告
        """
        failed_errors: List[str] = []
        failed_warnings: List[str] = []

        for rule in self.rules:
            try:
                satisfied = rule.evaluate(metrics)
            except Exception as e:
                logger.warning(f"规则 {rule.name} 求值失败: {e}")
                satisfied = False

            if not satisfied:
                if rule.severity == "warning":
                    failed_warnings.append(rule.name)
                else:
                    failed_errors.append(rule.name)

        if failed_errors:
            verdict = GateVerdict.FAILED
        elif self.block_on_warning and failed_warnings:
            verdict = GateVerdict.FAILED
        elif failed_warnings:
            verdict = GateVerdict.WARNED
        else:
            verdict = GateVerdict.PASSED

        report = GateReport(
            verdict=verdict,
            failed_rules=failed_errors,
            warned_rules=failed_warnings,
            metrics=metrics,
        )
        logger.info(f"质量门禁判定: {verdict} (失败 {len(failed_errors)}, 告警 {len(failed_warnings)})")
        return report


def build_default_gate(pass_rate_min: float = 0.6,
                       duplicate_rate_max: float = 0.3,
                       completeness_min: float = 0.8) -> QualityGate:
    """构造常用默认门禁

    Args:
        pass_rate_min: 最低质量通过率
        duplicate_rate_max: 最高重复率
        completeness_min: 最低字段完整率

    Returns:
        QualityGate 实例
    """
    gate = QualityGate()
    gate.add_rule(GateRule(
        name="pass_rate",
        metric_key="pass_rate",
        operator=">=",
        value=pass_rate_min,
        severity="error",
        description="质量通过率不低于阈值",
    ))
    gate.add_rule(GateRule(
        name="duplicate_rate",
        metric_key="duplicate_rate",
        operator="<=",
        value=duplicate_rate_max,
        severity="error",
        description="重复率不高于阈值",
    ))
    gate.add_rule(GateRule(
        name="completeness",
        metric_key="completeness",
        operator=">=",
        value=completeness_min,
        severity="warning",
        description="字段完整率建议不低于阈值",
    ))
    return gate


__all__ = [
    "QualityGate",
    "GateRule",
    "GateReport",
    "GateVerdict",
    "build_default_gate",
]
