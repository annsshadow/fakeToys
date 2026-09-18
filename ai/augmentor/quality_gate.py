"""Quality gate module"""

import logging
from dataclasses import dataclass, field
from typing import Any, Callable, Dict, List, Optional

logger = logging.getLogger(__name__)


class GateVerdict:
    PASSED = "passed"
    FAILED = "failed"
    WARNED = "warned"


@dataclass
class GateRule:
    name: str
    metric_key: str
    operator: str
    value: Any
    severity: str = "error"
    description: str = ""

    def evaluate(self, metrics: Dict[str, Any]) -> bool:
        current = metrics.get(self.metric_key)
        if current is None:
            return False
        if self.operator == ">=":
            return current >= self.value
        if self.operator == "<=":
            return current <= self.value
        return True


@dataclass
class GateReport:
    verdict: str
    failed_rules: List[str] = field(default_factory=list)
    warned_rules: List[str] = field(default_factory=list)
    metrics: Dict[str, Any] = field(default_factory=dict)

    @property
    def passed(self) -> bool:
        return self.verdict in (GateVerdict.PASSED, GateVerdict.WARNED)

    def to_dict(self) -> Dict[str, Any]:
        return {
            "verdict": self.verdict,
            "passed": self.passed,
            "failed_rules": self.failed_rules,
            "warned_rules": self.warned_rules,
            "metrics": self.metrics,
        }


class QualityGate:
    def __init__(self, rules: Optional[List[GateRule]] = None, block_on_warning: bool = False):
        self.rules = rules or []
        self.block_on_warning = block_on_warning

    def add_rule(self, rule: GateRule) -> "QualityGate":
        self.rules.append(rule)
        return self

    def run(self, metrics: Dict[str, Any]) -> GateReport:
        failed_errors: List[str] = []
        failed_warnings: List[str] = []
        for rule in self.rules:
            try:
                satisfied = rule.evaluate(metrics)
            except Exception as e:
                satisfied = False
                logger.warning(f"Rule {rule.name} evaluation failed: {e}")
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
        return GateReport(
            verdict=verdict,
            failed_rules=failed_errors,
            warned_rules=failed_warnings,
            metrics=metrics,
        )


def build_default_gate(pass_rate_min: float = 0.6,
                        duplicate_rate_max: float = 0.3,
                        completeness_min: float = 0.8) -> QualityGate:
    gate = QualityGate()
    gate.add_rule(GateRule(
        name="pass_rate",
        metric_key="pass_rate",
        operator=">=",
        value=pass_rate_min,
        severity="error",
        description="Quality pass rate not below threshold",
    ))
    gate.add_rule(GateRule(
        name="duplicate_rate",
        metric_key="duplicate_rate",
        operator="<=",
        value=duplicate_rate_max,
        severity="error",
        description="Duplicate rate not above threshold",
    ))
    gate.add_rule(GateRule(
        name="completeness",
        metric_key="completeness",
        operator=">=",
        value=completeness_min,
        severity="warning",
        description="Field completeness recommended not below threshold",
    ))
    return gate


__all__ = [
    "QualityGate",
    "GateRule",
    "GateReport",
    "GateVerdict",
    "build_default_gate",
]
