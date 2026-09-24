# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

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
    gate_dataset_health,
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


class TestGateDatasetHealth:
    """gate_dataset_health：健康分 × 门禁的组合链

    期望值全部手算，且**不**借用被测模块（不用 `DatasetHealthScore` /
    `ImpactEvaluator` 反推）——否则评分器算错时期望值会跟着一起错。

    样本 `DUP_ITEMS` 的推导：
      - completeness：3 条都有非空 instruction 与 output → 3/3 = 1.0
      - diversity：唯一 instruction 为 2 种 / 3 条 = 2/3
      - quality_balance：output 长度 [3, 3, 14]（`len("two three four")` 是 14，
        不是词数），均值 20/3、总体标准差 5.185449728701348，
        1 - 5.185449728701348/(20/3 + 1e-6) = 0.22218265736739917
      - coverage：全部 instruction 切成 6 个词、唯一 4 个，
        4/(6 + 1e-6)*10 > 1 → 截断为 1.0
      - health_score = 0.4*1.0 + 0.3*(2/3) + 0.2*0.22218265736739917 + 0.1*1.0
      - duplicate_rate：出现多次的文本共占 2 条 / 3 条 = 2/3
    """

    DUP_ITEMS = [
        {"instruction": "alpha beta", "output": "one"},
        {"instruction": "alpha beta", "output": "one"},
        {"instruction": "gamma delta", "output": "two three four"},
    ]

    def test_health_metrics_match_hand_computation(self):
        result = gate_dataset_health(
            self.DUP_ITEMS, weights=[0.4, 0.3, 0.2, 0.1]
        )
        health = result["health"]
        assert health["metrics"]["completeness"] == 1.0
        assert health["metrics"]["diversity"] == pytest.approx(2 / 3)
        assert health["metrics"]["quality_balance"] == pytest.approx(
            0.22218265736739917, rel=1e-9
        )
        assert health["metrics"]["coverage"] == 1.0
        assert health["health_score"] == pytest.approx(0.7444365314734798, rel=1e-9)
        assert health["level"] == "healthy"
        assert health["total_samples"] == 3

    def test_duplicate_rate_uses_requested_text_field(self):
        """重复率按 `text_field` 取，而不是写死 instruction

        样本里 instruction 全唯一（0.0）、output 全相同（1.0），两个字段必须
        给出不同答案；写死字段时只能看见其中一个。
        """
        items = [
            {"instruction": "a", "output": "x"},
            {"instruction": "b", "output": "x"},
        ]
        by_instruction = gate_dataset_health(items, text_field="instruction")
        by_output = gate_dataset_health(items, text_field="output")
        assert by_instruction["gate"]["metrics"]["duplicate_rate"] == 0.0
        assert by_output["gate"]["metrics"]["duplicate_rate"] == 1.0

    def test_missing_pass_rate_skips_rule_instead_of_failing_it(self):
        """不传 pass_rate 时必须**跳过**该规则，而不是让它静默判负

        `GateRule.evaluate` 对缺失的指标键返回 False（见
        TestGateRuleEvaluate::test_missing_metric_returns_false）。所以若这里
        照原样把三条规则都跑下去，一份本身合格的数据会因为「没人算过 pass_rate」
        而被门禁拒绝——把「没测」伪装成「不及格」。
        """
        items = [{"instruction": f"q{i}", "output": f"a{i}"} for i in range(5)]
        result = gate_dataset_health(items)
        assert result["skipped_rules"] == ["pass_rate"]
        assert "pass_rate" not in result["gate"]["metrics"]
        assert "pass_rate" not in result["gate"]["failed_rules"]
        assert result["gate"]["verdict"] == GateVerdict.PASSED

    def test_supplied_pass_rate_decides_the_verdict(self):
        """传了 pass_rate 就真的要参与判定（跳过逻辑不能变成永远跳过）"""
        items = [{"instruction": f"q{i}", "output": f"a{i}"} for i in range(5)]

        low = gate_dataset_health(items, pass_rate=0.5)
        assert low["skipped_rules"] == []
        assert low["gate"]["metrics"]["pass_rate"] == 0.5
        assert low["gate"]["failed_rules"] == ["pass_rate"]
        assert low["gate"]["verdict"] == GateVerdict.FAILED
        # CLI 用 `result["gate"]["passed"]` 决定退出码
        assert low["gate"]["passed"] is False

        high = gate_dataset_health(items, pass_rate=0.7)
        assert high["gate"]["verdict"] == GateVerdict.PASSED

    def test_warning_rule_blocks_only_when_asked(self):
        """completeness 是 warning 级：默认只告警，`block_on_warning` 才阻断

        样本 2 条里 1 条缺 output → completeness 0.5 < 0.8。
        """
        items = [{"instruction": "q1", "output": "a1"}, {"instruction": "q2"}]

        warned = gate_dataset_health(items)
        assert warned["gate"]["warned_rules"] == ["completeness"]
        assert warned["gate"]["failed_rules"] == []
        assert warned["gate"]["verdict"] == GateVerdict.WARNED
        assert warned["gate"]["passed"] is True

        blocked = gate_dataset_health(items, block_on_warning=True)
        assert blocked["gate"]["verdict"] == GateVerdict.FAILED
        assert blocked["gate"]["passed"] is False

    def test_invalid_weights_propagate(self):
        """非法权重必须原样抛出，而不是悄悄退回默认值"""
        from augmentor.exceptions import DataValidationError

        with pytest.raises(DataValidationError):
            gate_dataset_health(self.DUP_ITEMS, weights=[0.5, 0.5])

    def test_empty_dataset_raises_instead_of_passing(self):
        """空数据集不允许「指标全 0 → 唯一的 error 规则靠 0 ≤ 上限通过」而放行

        这是门禁最容易骗人的地方：`duplicate_rate <= 上限` 在 0 条数据上恒成立，
        于是「读到一个空文件」会被表达成「数据合格」。
        """
        from augmentor.exceptions import DataValidationError

        with pytest.raises(DataValidationError, match="为空"):
            gate_dataset_health([])


class TestPackageExport:
    def test_gate_dataset_health_reexported(self):
        """包级导出必须可用，否则新增的组合入口在 `import augmentor` 层面又是死的"""
        import augmentor

        assert callable(augmentor.gate_dataset_health)
        assert "gate_dataset_health" in augmentor.__all__
