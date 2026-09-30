# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""质量门禁模块

将多个质量指标组合为可配置的门禁规则，作为数据进入下游训练前的
最后一道关卡。支持 AND/OR 组合、严重级别与自动处置动作。
"""

import logging
from dataclasses import dataclass, field
from typing import Any, Callable, Dict, List, Optional
from .exceptions import QualityError, DataValidationError

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

        raise QualityError(f"不支持的操作符: {self.operator}")


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


def gate_dataset_health(items: List[Dict[str, Any]],
                        text_field: str = "instruction",
                        weights: Optional[List[float]] = None,
                        pass_rate: Optional[float] = None,
                        pass_rate_min: float = 0.6,
                        duplicate_rate_max: float = 0.3,
                        completeness_min: float = 0.8,
                        block_on_warning: bool = False) -> Dict[str, Any]:
    """对一份数据集打健康分并跑默认门禁，返回 `{health, gate, skipped_rules}`

    这是把 `DatasetHealthScore` + `ImpactEvaluator` + `QualityGate` 三个模块接成
    一条判定链的**唯一**入口，CLI 与 API 共用它——门禁的取值口径一旦有两份实现，
    两边就会给出不同的放行结论。

    `pass_rate` 必须显式传：`GateRule.evaluate()` 对**缺失的指标键返回 False**，
    所以指标字典里没有 `pass_rate` 时那条规则会判为不满足，门禁于是把「没算这一项」
    表达成「算出来不及格」。两者对下游的意义完全相反，因此不传时把规则摘掉并记进
    `skipped_rules`，让调用方看得见少了什么。

    Args:
        items: 数据集条目
        text_field: 计算重复率所用的文本字段
        weights: 健康分权重，None 用 `DatasetHealthScore` 默认值
        pass_rate: 质量通过率（由 `QualityScorer` 那侧算好后传入）
        pass_rate_min: pass_rate 规则的最低阈值
        duplicate_rate_max: duplicate_rate 规则的最高阈值
        completeness_min: completeness 规则的最低阈值
        block_on_warning: warning 规则失败是否也阻断

    Returns:
        `{"health": ..., "gate": ..., "skipped_rules": [...]}`
    """
    from .health_score import DatasetHealthScore
    from .impact import ImpactEvaluator

    if not items:
        # 空数据集上每项指标都是 0.0：duplicate_rate 规则会以「0 ≤ 上限」通过，
        # 于是门禁给出一份本身为空的文件放行——正是门禁最不该犯的错。宁可报错。
        raise DataValidationError("数据集为空，门禁无法给出有意义的判定")

    health = DatasetHealthScore(weights=weights).score(items)

    metrics: Dict[str, Any] = dict(health["metrics"])
    metrics["duplicate_rate"] = ImpactEvaluator(
        text_field=text_field
    ).measure(items).duplicate_rate

    rules = build_default_gate(
        pass_rate_min=pass_rate_min,
        duplicate_rate_max=duplicate_rate_max,
        completeness_min=completeness_min,
    ).rules

    skipped_rules: List[str] = []
    if pass_rate is None:
        skipped_rules = [rule.name for rule in rules if rule.metric_key == "pass_rate"]
        rules = [rule for rule in rules if rule.metric_key != "pass_rate"]
    else:
        metrics["pass_rate"] = pass_rate

    report = QualityGate(
        rules=rules,
        block_on_warning=block_on_warning,
    ).run(metrics)

    return {
        "health": health,
        "gate": report.to_dict(),
        "skipped_rules": skipped_rules,
    }


__all__ = [
    "QualityGate",
    "GateRule",
    "GateReport",
    "GateVerdict",
    "build_default_gate",
    "gate_dataset_health",
]
