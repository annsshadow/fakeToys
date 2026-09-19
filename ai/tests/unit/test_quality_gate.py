"""QualityGate 单元测试

门禁规则必须可解释：每条规则的判定、严重级别与组合语义都要有断言。
"""

import pytest

from augmentor.quality_gate import (
    QualityGate,
    GateRule,
    GateReport,
    GateVerdict,
    build_default_gate,
)


class TestGateRuleEvaluate:
    """GateRule.evaluate 各操作符"""

    def test_gte(self):
        rule = GateRule(name="r", metric_key="x", operator=">=", value=0.5)
        assert rule.evaluate({"x": 0.6}) is True
        assert rule.evaluate({"x": 0.4}) is False

    def test_lte(self):
        rule = GateRule(name="r", metric_key="x", operator="<=", value=0.3)
        assert rule.evaluate({"x": 0.1}) is True
        assert rule.evaluate({"x": 0.5}) is False

    def test_gt(self):
        rule = GateRule(name="r", metric_key="x", operator=">", value=0.5)
        assert rule.evaluate({"x": 0.5}) is False
        assert rule.evaluate({"x": 0.51}) is True

    def test_lt(self):
        rule = GateRule(name="r", metric_key="x", operator="<", value=0.5)
        assert rule.evaluate({"x": 0.5}) is False
        assert rule.evaluate({"x": 0.49}) is True

    def test_eq(self):
        rule = GateRule(name="r", metric_key="x", operator="==", value="ok")
        assert rule.evaluate({"x": "ok"}) is True
        assert rule.evaluate({"x": "bad"}) is False

    def test_ne(self):
        rule = GateRule(name="r", metric_key="x", operator="!=", value="bad")
        assert rule.evaluate({"x": "bad"}) is False
        assert rule.evaluate({"x": "ok"}) is True

    def test_in(self):
        rule = GateRule(name="r", metric_key="lang", operator="in", value=["zh", "en"])
        assert rule.evaluate({"lang": "zh"}) is True
        assert rule.evaluate({"lang": "fr"}) is False

    def test_not_in(self):
        rule = GateRule(name="r", metric_key="lang", operator="not_in", value=["fr"])
        assert rule.evaluate({"lang": "fr"}) is False
        assert rule.evaluate({"lang": "zh"}) is True

    def test_missing_metric_returns_false(self):
        rule = GateRule(name="r", metric_key="x", operator=">=", value=0)
        assert rule.evaluate({}) is False

    def test_unknown_operator_raises(self):
        rule = GateRule(name="r", metric_key="x", operator="~", value=0)
        with pytest.raises(ValueError, match="不支持"):
            rule.evaluate({"x": 1})


class TestQualityGateRun:
    """QualityGate.run 判定逻辑"""

    def test_all_pass(self):
        gate = QualityGate([
            GateRule("pr", "pass_rate", ">=", 0.6, "error"),
            GateRule("dr", "duplicate_rate", "<=", 0.3, "error"),
        ])
        report = gate.run({"pass_rate": 0.8, "duplicate_rate": 0.1})
        assert report.verdict == GateVerdict.PASSED
        assert report.passed is True
        assert report.failed_rules == []

    def test_error_rule_failure_blocks(self):
        gate = QualityGate([
            GateRule("pr", "pass_rate", ">=", 0.6, "error"),
        ])
        report = gate.run({"pass_rate": 0.3})
        assert report.verdict == GateVerdict.FAILED
        assert report.passed is False
        assert "pr" in report.failed_rules

    def test_warning_failure_does_not_block_by_default(self):
        gate = QualityGate([
            GateRule("comp", "completeness", ">=", 0.9, "warning"),
        ])
        report = gate.run({"completeness": 0.5})
        assert report.verdict == GateVerdict.WARNED
        assert report.passed is True
        assert "comp" in report.warned_rules

    def test_block_on_warning_turns_warning_into_failure(self):
        gate = QualityGate(
            [GateRule("comp", "completeness", ">=", 0.9, "warning")],
            block_on_warning=True,
        )
        report = gate.run({"completeness": 0.5})
        assert report.verdict == GateVerdict.FAILED
        assert report.passed is False

    def test_add_rule_chaining(self):
        gate = QualityGate()
        gate.add_rule(GateRule("a", "x", ">=", 0, "error"))
        gate.add_rule(GateRule("b", "y", "<=", 1, "error"))
        assert len(gate.rules) == 2

    def test_report_to_dict_fields(self):
        report = GateReport(verdict=GateVerdict.PASSED, metrics={"x": 1})
        d = report.to_dict()
        assert d["verdict"] == "passed"
        assert d["passed"] is True
        assert d["metrics"] == {"x": 1}


class TestBuildDefaultGate:
    """build_default_gate"""

    def test_default_gate_structure(self):
        gate = build_default_gate()
        assert len(gate.rules) == 3
        names = [r.name for r in gate.rules]
        assert "pass_rate" in names
        assert "duplicate_rate" in names
        assert "completeness" in names

    def test_default_gate_passes_good_metrics(self):
        gate = build_default_gate()
        report = gate.run({
            "pass_rate": 0.8,
            "duplicate_rate": 0.1,
            "completeness": 0.9,
        })
        assert report.verdict == GateVerdict.PASSED

    def test_default_gate_custom_thresholds(self):
        gate = build_default_gate(pass_rate_min=0.95)
        report = gate.run({
            "pass_rate": 0.8,
            "duplicate_rate": 0.0,
            "completeness": 1.0,
        })
        assert "pass_rate" in report.failed_rules

    def test_gate_verdict_constants(self):
        assert GateVerdict.PASSED == "passed"
        assert GateVerdict.FAILED == "failed"
        assert GateVerdict.WARNED == "warned"
